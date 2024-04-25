use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, http::header::ContentType};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub input: String,
}

async fn infer(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    // Using https://github.com/AIAnytime/LLM-Inference-API-in-Rust/blob/main/language_model_server/src/main.rs as a source example for how to set up
    let tokenizer_source = llm::TokenizerSource::Embedded;
    let model_architecture = llm::ModelArchitecture::GptNeoX;
    let model_path = PathBuf::from("/app/model/pythia.bin");
    let model = llm::load_dynamic(
        Some(model_architecture),
        &model_path,
        tokenizer_source,
        Default::default(),
        llm::load_progress_callback_stdout,
    )?;

    let mut session = model.start_session(Default::default());
    let mut out = String::new();

    let answer = session.infer::<Infallible>(
        model.as_ref(),
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: (&prompt).into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: Some(64),
        },
        // OutputRequest
        &mut Default::default(),
        |r| match r {
            llm::InferenceResponse::PromptToken(t) | llm::InferenceResponse::InferredToken(t) => {
                print!("{t}");
                std::io::stdout().flush().unwrap();
                out.push_str(&t);
                Ok(llm::InferenceFeedback::Continue)
            }
            _ => Ok(llm::InferenceFeedback::Continue),
        },
    );

    match answer {
        Ok(_) => Ok(out),
        Err(err) => Err(Box::new(err)),
    }
}

#[post("/message")]
async fn message(data: web::Json<Request>) -> impl Responder {

    let prompt = data.input.clone();
    match infer(prompt).await {
        Ok(result) => {
            return HttpResponse::Ok().content_type(ContentType::plaintext()).body(result);
        }
        Err(err) => {
            let res = &err.to_string();
            return HttpResponse::Ok().content_type(ContentType::plaintext()).body(format!("Error during inference: {res}\n"));
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(message))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
