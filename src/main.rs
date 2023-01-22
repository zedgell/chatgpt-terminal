use async_openai::types::CreateCompletionRequestArgs;
use async_openai::Client;
use std::str::FromStr;
use std::u16;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().len() != 2 {
        panic!("you must pass in tokes. Ex: chatgpt-terminal 1000")
    }
    let tokens = u16::from_str(
        std::env::args()
            .collect::<Vec<String>>()
            .get(1)
            .unwrap()
            .as_str(),
    )
    .unwrap();
    let mut prompt = String::new();
    loop {
        let mut input = String::new();
        println!("input: ");
        let _ = std::io::stdin().read_line(&mut input);
        prompt += input.as_str();

        let request = CreateCompletionRequestArgs::default()
            .prompt(&prompt)
            .n(1)
            .frequency_penalty(2.0)
            .temperature(0.7)
            .top_p(1.0)
            .stop(vec!["(END OF METHOD)"])
            .max_tokens(tokens)
            .model("text-davinci-003")
            .build()?;

        let response = Client::new().completions().create(request).await;

        match response {
            Ok(completion) => {
                let text = &completion.choices[0].text;
                prompt = text.to_string();
                println!("{}\n", &text);
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
        }
    }
    Ok(())
}
