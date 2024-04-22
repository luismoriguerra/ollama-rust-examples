use futures::StreamExt;
use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage, MessageRole},
    Ollama
};
use tokio::io::AsyncWriteExt;
use xp_ollama::{
    consts::{DEFAUL_SYSTEM_MOCK, MODEL},
    Result
};

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();

    let prompts = &[
        "why the sky is red  ? (be concise) ",
        "what is the second best language ?",
        "what was my last question ?"
    ];

    let system_msg =
        ChatMessage::new(MessageRole::System, DEFAUL_SYSTEM_MOCK.to_string());

    let mut thread_msgs = vec![system_msg];

    for prompt in prompts {
        println!("\n ->> prompt: {}", prompt);

        let prompt_msg =
            ChatMessage::new(MessageRole::User, prompt.to_string());

        thread_msgs.push(prompt_msg);

        let chat_req =
            ChatMessageRequest::new(MODEL.to_string(), thread_msgs.clone());

        let msg_content = run_chat_req(&ollama, chat_req).await?;

        if let Some(content) = msg_content {
            let asst_msg = ChatMessage::new(MessageRole::Assistant, content);
            thread_msgs.push(asst_msg);
        }
    }

    Ok(())
}

pub async fn run_chat_req(
    ollama: &Ollama,
    chat_req: ChatMessageRequest
) -> Result<Option<String>> {
    let mut stream = ollama.send_chat_messages_stream(chat_req).await?;

    let mut stdout = tokio::io::stdout();
    let mut char_count = 0;
    let mut current_asst_msg_elems: Vec<String> = Vec::new();

    while let Some(res) = stream.next().await {
        let res = res.map_err(|_| "stream error")?;

        if let Some(msg) = res.message {
            let msg_content = msg.content;

            char_count += msg_content.len();
            if char_count > 80 {
                char_count = 0;
                stdout.write_all(b"\n").await?;
            }

            stdout.write_all(msg_content.as_bytes()).await?;
            stdout.flush().await?;

            current_asst_msg_elems.push(msg_content);
        }

        if let Some(_final_res) = res.final_data {
            stdout.write_all(b"\n").await?;
            stdout.flush().await?;

            let asst_content = current_asst_msg_elems.join("");

            return Ok(Some(asst_content));
        }
    }

    // new line
    stdout.write_all(b"\n").await?;
    stdout.flush().await?;

    Ok(None)
}
