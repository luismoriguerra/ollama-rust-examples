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
        generation::completion::{
            request::GenerationRequest, GenerationFinalResponseData
        },
        Ollama
    };
    use tokio::io::AsyncWriteExt;

    use super::*;

    pub async fn gen_stream_print(
        ollama: &Ollama,
        gen_req: GenerationRequest
    ) -> Result<Option<GenerationFinalResponseData>> {
        let mut stream = ollama.generate_stream(gen_req).await?;

        let mut stdout = tokio::io::stdout();
        let mut line_words_len_count = 0;

        while let Some(res) = stream.next().await {
            let responses = res.unwrap();

            for prompt_response in responses {
                // early return if response is done
                if let Some(final_data) = prompt_response.final_data {
                    stdout.write_all(b"\n").await?;
                    stdout.flush().await?;
                    return Ok(Some(final_data));
                }

                // bytes will have 1 word step by step
                let word_in_bytes = prompt_response.response.as_bytes();

                // If line is already long, start a new line
                line_words_len_count += word_in_bytes.len();
                if line_words_len_count > 80 {
                    stdout.write_all(b"\n").await?;
                    line_words_len_count = 0;
                }

                // write line
                stdout.write_all(word_in_bytes).await?;
                stdout.flush().await?;
            }
        }

        stdout.write_all(b"\n").await?;

        Ok(None)
    }
}
