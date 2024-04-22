use ollama_rs::{
    generation::completion::{request::GenerationRequest, GenerationContext},
    Ollama
};
use simple_fs::{ensure_file_dir, save_json};
use xp_ollama::{consts::MODEL, gen::gen_stream_print, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();

    let prompts = &[
        "why the sky is red  ? (be concise) ",
        "what was my first question ?"
    ];

    let mut prev_prompt_ctx: Option<GenerationContext> = None;

    for prompt in prompts {
        println!("Prompt: {}", prompt);

        let mut gen_req =
            GenerationRequest::new(MODEL.to_string(), prompt.to_string());

        // in the 2nd step , we need the context of the previous prompt
        if let Some(prev_prompt_ctx) = prev_prompt_ctx.take() {
            let ctx_file_path = "examples/c02_ctx.json";
            ensure_file_dir(ctx_file_path)?;
            save_json(ctx_file_path, &prev_prompt_ctx)?;

            gen_req = gen_req.context(prev_prompt_ctx);
        }

        let final_data = gen_stream_print(&ollama, gen_req).await?;

        if let Some(final_data) = final_data {
            prev_prompt_ctx = Some(final_data.context);
        }
    }

    Ok(())
}
