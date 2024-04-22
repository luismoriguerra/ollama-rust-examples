use futures::StreamExt;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use tokio::io::AsyncWriteExt;
use xp_ollama::{
    consts::{DEFAUL_SYSTEM_MOCK, MODEL},
    Result
};

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();

    let model = MODEL.to_string();

    let prompt = "what is the best programming language? ".to_string();

    let gen_req = GenerationRequest::new(model, prompt)
        .system(DEFAUL_SYSTEM_MOCK.to_string());

    //  simple response generation
    // let res = ollama.generate(gen_req).await?;
    // println!("->> res {}", res.response);

    //  stream response generation
    gen_stream_print(&ollama, gen_req).await?;

    Ok(())
}

pub async fn gen_stream_print(
    ollama: &Ollama,
    gen_req: GenerationRequest
) -> Result<()> {
    let mut stream = ollama.generate_stream(gen_req).await?;

    let mut stdout = tokio::io::stdout();
    let mut char_count = 0;

    while let Some(res) = stream.next().await {
        let responses = res.unwrap();

        for resp in responses {
            let bytes = resp.response.as_bytes();

            char_count += bytes.len();
            if char_count > 80 {
                stdout.write_all(b"\n").await?;
                char_count = 0;
            }

            // write output
            stdout.write_all(bytes).await?;
            stdout.flush().await?;
        }
    }

    stdout.write_all(b"\n").await?;

    Ok(())
}
