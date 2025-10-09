mod ddg;

use ddg::{DuckDuckGoSearch};
use serde::Deserialize;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Deserialize, utoipa::ToSchema)]
struct SearchRequest {
    query: String,
    pages: Option<usize>,
}

#[derive(utoipa::ToSchema, serde::Serialize)]
struct ResultWrapper {
    title: String,
    link: String,
    snippet: String,
    favicon: Option<String>,
}

#[derive(OpenApi)]
#[openapi(
    paths(search),
    components(schemas(SearchRequest, ResultWrapper))
)]
struct ApiDoc;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[utoipa::path(
    post,
    path = "/search",
    request_body = SearchRequest,
    responses(
        (status = 200, description = "Search results", body = [ResultWrapper]),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/search")]
async fn search(req: web::Json<SearchRequest>) -> impl Responder {
    let ddg = DuckDuckGoSearch::new();
    let pages = req.pages.unwrap_or(1);

    match ddg.get_results(&req.query, pages).await {
        Ok(results) => {
            // Convert DuckDuckGoResult -> ResultWrapper
            let wrapped: Vec<ResultWrapper> = results
                .into_iter()
                .map(|r| ResultWrapper {
                    title: r.title,
                    link: r.link,
                    snippet: r.snippet,
                    favicon: r.favicon
                })
                .collect();
            HttpResponse::Ok().json(wrapped)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let openapi = ApiDoc::openapi();

    println!("Starting server at http://localhost:8080/");

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(search)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone())
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}