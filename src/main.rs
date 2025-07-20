use deepseek_rs::{
    DeepSeekClient,
    client::chat_completions::request::{Message, Model, RequestBody},
};
use image::{DynamicImage, RgbaImage};
use std::env;
use std::error::Error;
use std::os;
use win_screenshot::prelude::*;

fn takescreenshot() {
    let buf = capture_display().unwrap();
    let img =
        DynamicImage::ImageRgba8(RgbaImage::from_raw(buf.width, buf.height, buf.pixels).unwrap());
    img.to_rgb8().save("screenshot.png").unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let client = DeepSeekClient::default()?;
    takescreenshot();
    let question = String::from("Description:

Given an array of integers nums and an integer k, return the length of the longest contiguous subarray whose sum equals k.
If no such subarray exists, return 0. please answer it in python");
    let request = RequestBody::new_messages(vec![Message::new_user_message(question)])
        .with_model(Model::DeepseekChat);
    let response = client.chat_completions(request).await?;
    println!("{}", response.choices[0].message.content.as_ref().unwrap());
    Ok(())
}
