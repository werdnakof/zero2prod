use actix_web::web::Query;
use actix_web::{HttpRequest, HttpResponse};
use std::fs::File;
use std::io::{copy, Cursor};

#[derive(Debug, serde_derive::Deserialize)]
pub struct Params {
    url: String,
}

// Let's start simple: we always return a 200 OK
pub async fn download(req: HttpRequest) -> HttpResponse {
    let query = Query::<Params>::from_query(req.query_string()).unwrap();
    let url = query.url.as_str();

    async move {
        match reqwest::get(url).await {
            Ok(response) => {
                let mut file = {
                    let fname = response
                        .url()
                        .path_segments()
                        .and_then(|segments| segments.last())
                        .and_then(|name| if name.is_empty() { None } else { Some(name) })
                        .unwrap_or("tmp.bin");
                    File::create(fname).unwrap()
                };
                let mut content = Cursor::new(response.bytes().await.unwrap());
                copy(&mut content, &mut file).unwrap();
            }
            Err(_) => {}
        }
    }
    .await;

    HttpResponse::Ok().finish()
}
