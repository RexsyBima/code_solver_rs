use deepseek_rs::{
    DeepSeekClient,
    client::chat_completions::request::{Message, Model as DsModel, RequestBody},
};
use image::{DynamicImage, RgbaImage};
use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
use std::error::Error;
use std::net::TcpStream;
use std::path::PathBuf;
use win_screenshot::prelude::*;

fn file_path(path: &str) -> PathBuf {
    let mut abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    abs_path.push(path);
    abs_path
}

fn check_internet() {
    let urls = vec![("8.8.8.8", 53), ("google.com", 80)];
    let mut connected = false;
    for (host, port) in urls {
        let stream = TcpStream::connect((host, port));
        if stream.is_ok() {
            connected = true;
            break;
        }
    }
    if !connected {
        println!("No internet connection");
        std::process::exit(1);
    }
}

fn takescreenshot() {
    let buf = capture_display().unwrap();
    let img =
        DynamicImage::ImageRgba8(RgbaImage::from_raw(buf.width, buf.height, buf.pixels).unwrap());
    img.to_rgb8().save("screenshot.png").unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    check_internet();
    dotenvy::dotenv()?;
    // https://docs.rs/openai-api-rs/6.0.7/
    //https://crates.io/crates/ocrs
    let client = DeepSeekClient::default()?;
    let detection_model = Model::load_file(file_path("text-detection.rten"))?;
    let recognition_model = Model::load_file(file_path("text-recognition.rten"))?;

    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    })?;

    takescreenshot();
    let img = image::open("screenshot.png").map(|image| image.into_rgb8())?;
    let image_source = ImageSource::from_bytes(img.as_raw(), img.dimensions())?;
    let ocr_input = engine.prepare_input(image_source)?;

    // Detect and recognize text. If you only need the text and don't need any
    // layout information, you can also use `engine.get_text(&ocr_input)`,
    // which returns all the text in an image as a single string.

    // Get oriented bounding boxes of text words in input image.
    let word_rects = engine.detect_words(&ocr_input)?;

    // Group words into lines. Each line is represented by a list of word
    // bounding boxes.
    let line_rects = engine.find_text_lines(&ocr_input, &word_rects);

    // Recognize the characters in each line.
    let line_texts = engine.recognize_text(&ocr_input, &line_rects)?;
    for line in line_texts
        .iter()
        .flatten()
        // Filter likely spurious detections. With future model improvements
        // this should become unnecessary.
        .filter(|l| l.to_string().len() > 1)
    {
        println!("{}", line);
    }
    println!(
        "-----------------------------------------------------------------------------------------------"
    );
    let question = String::from("Description:

    Given an array of integers nums and an integer k, return the length of the longest contiguous subarray whose sum equals k.
    If no such subarray exists, return 0. please answer it in python");
    let request = RequestBody::new_messages(vec![Message::new_user_message(question)])
        .with_model(DsModel::DeepseekChat);
    let response = client.chat_completions(request).await?;
    println!("{}", response.choices[0].message.content.as_ref().unwrap());
    Ok(())
}

#[cfg(test)]
mod tests {
    // Bring outer scope functions into scope
    use super::*;

    #[test]
    fn test_add() {
        let result = file_path("foo");
        assert_eq!(result.to_str().unwrap(), "foo");
    }
}
