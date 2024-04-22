use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use xp_ollama::{
    consts::{DEFAUL_SYSTEM_MOCK, MODEL},
    Result,
};

#[tokio::main]
async fn main() -> Result<(),> {
    let ollama = Ollama::default();

    let model = MODEL.to_string();

    let prompt =
        "what is the best programming language? (be concise)".to_string();

    let gen_req = GenerationRequest::new(model, prompt,)
        .system(DEFAUL_SYSTEM_MOCK.to_string(),);

    let res = ollama.generate(gen_req,).await?;
    println!("->> res {}", res.response);

    Ok((),)
}
