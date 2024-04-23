use std::{env, fs, io::Write, path::Path};

use ollama_rs::{
    generation::completion::{request::GenerationRequest, GenerationContext},
    Ollama
};

#[tokio::main]
async fn main() {
    let args = env::args().collect::<Vec<String>>();
    let model = &args[1];
    let mut prompt = &args[2]; // args.get(2).expect("prompt is required");
    let mut reply = "";

    if args.len() == 4 {
        // reply = args.get(2).expect("reply is required");
        // prompt = args.get(3).expect("prompt is required");
        reply = &args[2];
        prompt = &args[3];

        let mut request =
            GenerationRequest::new(model.to_string(), prompt.to_string());

        let valid_reply = "-r";
        if reply == valid_reply && Path::new("./tmp/hey-context").exists() {
            let data =
                fs::read("/tmp/hey-context").expect("Unable to read file");
            let decoded: Option<GenerationContext> =
                bincode::deserialize(&data[..]).unwrap();

            if let Some(context) = decoded {
                request = request.context(context);
            }
        }

        let ollama = Ollama::default();
        let res = ollama.generate(request).await;

        if let Ok(res) = res {
            println!("{}", res.response);

            if let Some(final_data) = res.final_data {
                let new_context = Some(final_data.context);
                let encoded = bincode::serialize(&new_context).unwrap();
                let mut f = fs::File::create("./tmp/hey-context")
                    .expect("Unable to create file");
                f.write_all(&encoded).expect("Unable to write data");
            }
        }
    }

    // end
}
