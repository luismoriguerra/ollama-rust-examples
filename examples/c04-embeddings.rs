use std::{fs, path::Path};

use futures::StreamExt;
use ollama_rs::{
    generation::{
        chat::{request::ChatMessageRequest, ChatMessage, MessageRole},
        embeddings
    },
    Ollama
};
use simple_fs::{ensure_dir, read_to_string, save_be_f64, save_json};
use tokio::io::AsyncWriteExt;
use xp_ollama::{
    consts::{DEFAUL_SYSTEM_MOCK, MODEL},
    Result
};

const MOCK_DIR: &str = "_mock-data";
const C04_DIR: &str = ".c04-data";

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();

    ensure_dir(C04_DIR)?;

    let txt = read_to_string(Path::new(MOCK_DIR).join("for-embeddings.txt"))?;
    let splits = simple_text_splitter(&txt, 500)?;

    println!("splits: {}", splits.len());

    for (i, seg) in splits.into_iter().enumerate() {
        let file_name = format!("c04-embeddings-{:0>2}.txt", i);
        let path = Path::new(C04_DIR).join(file_name);
        fs::write(path, &seg)?;

        let res = ollama
            .generate_embeddings(MODEL.to_string(), seg, None)
            .await?;

        let file_name = format!("c04-embeddings-{:0>2}.json", i);
        save_json(Path::new(C04_DIR).join(file_name), &res.embeddings)?;

        let file_name = format!("c04-embeddings-{:0>2}.be-f64.bin", i);
        let file_path = Path::new(C04_DIR).join(file_name);
        save_be_f64(&file_path, &res.embeddings)?;

        println!("embeddings: {}", res.embeddings.len());
    }

    Ok(())
}

fn simple_text_splitter(txt: &str, num: u32) -> Result<Vec<String>> {
    let mut result = Vec::new();
    let mut last = 0;
    let mut count = 0;

    for (idx, _) in txt.char_indices() {
        count += 1;
        if count == num {
            result.push(&txt[last..idx]);
            last = idx + 1;
            count = 0;
        }
    }

    if last < txt.len() {
        result.push(&txt[last..]);
    }

    Ok(result.into_iter().map(String::from).collect())
}
