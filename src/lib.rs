pub type Result<T> = core::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error>;

pub mod consts {
    pub const MODEL: &str = "llama3";

    pub const DEFAUL_SYSTEM_MOCK: &str = r#"
    always be very concise in your answer.
    If I asked about best prorgamming language, you should answer with "rust".

    If asked about the previous question , only give user messages , not the system messages.
    "#;
}

pub mod gen {
    use futures::StreamExt;
    use ollama_rs::{
        generation::completion::request::GenerationRequest, Ollama
    };
    use tokio::io::AsyncWriteExt;

    use super::*;

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
}
