mod queue;

use axum::{
    extract::{DefaultBodyLimit, Multipart},
    response::Html,
    routing::post,
    Router, Json,
};
use serde::{Deserialize, Serialize};
use tower_http::limit::RequestBodyLimitLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with some routes
    let app = Router::new()
        .route("/file", post(accept_form))
        .route("/c2rust", post(c2rust_by_project))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(
            25 * 1024 * 1024, /* 25mb */
        ))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // run it with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9099")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/" method="post" enctype="multipart/form-data">
                    <label>
                        Upload file:
                        <input type="file" name="file" multiple>
                    </label>

                    <input type="submit" value="Upload files">
                </form>
            </body>
        </html>
        "#,
    )
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct File {
    pub name: String,
    pub content: String,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Directory {
    pub name: String,
    pub directories: Option<Vec<Directory>>,
    pub files: Option<Vec<File>>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ProjectParams {
    pub name: String,
    pub directories: Option<Vec<Directory>>,
}

async fn c2rust_by_project(Json(params): Json<ProjectParams>) {
    println!("Received params: {:?}", params);
    use std::fs;
    use std::path::Path;

    fn create_directory_structure(base_path: &Path, directories: &Option<Vec<Directory>>) {
        if let Some(dirs) = directories {
            for dir in dirs {
                let dir_path = base_path.join(&dir.name);
                fs::create_dir_all(&dir_path).unwrap();
                create_directory_structure(&dir_path, &dir.directories);
                if let Some(files) = &dir.files {
                    for file in files {
                        let file_path = dir_path.join(&file.name);
                        fs::write(file_path, &file.content).unwrap();
                    }
                }
            }
        }
    }

    let base_path = Path::new("uploads").join(&params.name);
    fs::create_dir_all(&base_path).unwrap();
    create_directory_structure(&base_path, &params.directories);
}

async fn accept_form(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!(
            "Length of `{name}` (`{file_name}`: `{content_type}`) is {} bytes",
            data.len()
        );
    }
}