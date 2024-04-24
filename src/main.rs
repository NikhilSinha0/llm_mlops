use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, http::header::ContentType};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::convert::Infallible;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub input: String,
}

#[post("/message")]
async fn message(data: web::Json<Request>) -> impl Responder {
    // Extract some useful information from the request
    let body = event.body();
    let s = std::str::from_utf8(body).expect("invalid utf-8 sequence");

    //Serialze JSON into struct.
    //If JSON is incorrect, send back 400 with error.
    let item = match from_str::<RequestBody>(s) {
        Ok(item) => item,
        Err(err) => {
            let resp = Response::builder()
                .status(400)
                .header("content-type", "text/html")
                .body(("Body not provided correctly: ".to_string() + &err.to_string() + "\n").into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };

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

    let prompt = item.input;

    let answer = session.infer::<Infallible>(
        model.as_ref(),
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: (&prompt).into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: Some(30),
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
        Ok(_) => {
            return HttpResponse::Ok().content_type(ContentType::plaintext()).body(out);
        },
        Err(err) => {
            let res = &err.to_string();
            return HttpResponse::Ok().content_type(ContentType::plaintext()).body(format!("Error during inference: {res}\n"));
        },
    }
    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(message))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
