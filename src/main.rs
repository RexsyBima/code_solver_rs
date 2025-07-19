use deepseek_rs::{
    DeepSeekClient,
    client::chat_completions::request::{Message, Model, RequestBody},
};
use screenshots::Screen;
use std::env;
use std::error::Error;

fn takescreenshot() {
    let screens = Screen::all().unwrap();
    for s in screens {
        println!("capturing {s:?}");
        let mut image = s.capture().unwrap();
        image.save(format!("{}.png", s.display_info.id)).unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    takescreenshot();
    for (k, v) in env::vars() {
        println!("{}: {}", k, v);
    }
    let client = DeepSeekClient::default()?;
    let question = String::from("Description:

Given an array of integers nums and an integer k, return the length of the longest contiguous subarray whose sum equals k.
If no such subarray exists, return 0. please answer it in python");
    let request = RequestBody::new_messages(vec![Message::new_user_message(question)])
        .with_model(Model::DeepseekChat);
    println!("Hello world chat gpt app");
    let response = client.chat_completions(request).await?;
    println!("{}", response.choices[0].message.content.as_ref().unwrap());
    Ok(())
}
