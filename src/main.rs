use actix_web::{http::header::ContentType, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use rust_bert::pipelines::question_answering::{QuestionAnsweringModel, QuestionAnsweringConfig, QaInput};
use rust_bert::pipelines::common::ModelResource;
use rust_bert::resources::LocalResource;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub context: String,
    pub query: String,
}

#[post("/answer_question")]
async fn answer_question(model_ctx: web::Data<Mutex<QuestionAnsweringModel>>, data: web::Json<Request>) -> impl Responder {
                                                        
    let question = data.query.clone();
    let context = data.context.clone();
    let model = model_ctx.lock().unwrap();

    let result = model.predict(&[QaInput { question, context }], 1, 1);

    // Since we set topk to one, the inner vec should only have one item. Since the batch size is one, the outer vec should only have one item.
    let answer = &result[0][0];

    HttpResponse::Ok().content_type(ContentType::plaintext()).body(answer.answer.clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let model_path = Box::new(LocalResource {
        local_path: PathBuf::from("/app/model/rust_model.ot"),
    });

    let config_path = Box::new(LocalResource {
        local_path: PathBuf::from("/app/model/config.json"),
    });

    let vocab_path = Box::new(LocalResource {
        local_path: PathBuf::from("/app/model/vocab.txt"),
    });

    let qa_model = QuestionAnsweringModel::new(QuestionAnsweringConfig {
        model_resource: ModelResource::Torch(model_path),
        config_resource: config_path,
        vocab_resource: vocab_path,
        ..Default::default()
    });
    
    let model: QuestionAnsweringModel;

    match qa_model {
        Ok(m) => {
            model = m;
            println!("Successfully loaded model");
        },
        Err(e) => {
            let err_str = e.to_string();
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Model initialization failed: {err_str}")));
        }
    }

    let model_wrapper = web::Data::new(Mutex::new(model));

    HttpServer::new(move || App::new().service(answer_question).app_data(model_wrapper.clone()))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
