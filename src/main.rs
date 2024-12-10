use regex::Regex;

use std::{fs, path::Path, process::Command};

use axum::{
    extract::{DefaultBodyLimit, Multipart},
    response::Html,
    routing::post,
    Router, Json,
};
use serde::{Deserialize, Serialize};
use tower_http::limit::RequestBodyLimitLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::info;

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
        .route("/api/transcode/from_json_to", post(from_json_to_rust))
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


#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct CompileCommandItem {
    pub arguments: Vec<String>, // vec!["cc", "-o", "a.c"]
    pub directory: String, // "src"
    pub file: String, // "a.c"
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct CompileCommand {
    pub items: Vec<CompileCommandItem>,
}

impl CompileCommand {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn push(&mut self, item: CompileCommandItem) {
        self.items.push(item);
    }

}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct TranscodePathParams {
    pub path: String,
    pub code: String,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct TranscodeParams {
    #[serde(rename = "·")]
    pub project_name: String,
    pub content: Vec<TranscodePathParams>,
}


fn check_file_name(file_name: &str, main_file_name_: &str) -> bool {
    file_name == main_file_name_
}


async fn from_json_to_rust(Json(params): Json<TranscodeParams>) -> Json<TranscodeParams> {
    info!("Received params: {:?}", params);
    let contents = params.content;
    let mut compile_command = CompileCommand::new();
    // 目录的base路径
    let base_path = Path::new("uploads").join(&params.project_name);
    info!("Base Path: {:?}", base_path);
    let base_dir = base_path.to_str().unwrap();
    let mut main_file_name = String::new();
    for content in contents {
        let path = content.path;
        let code = content.code;
        info!("Path: {:?}, Code: {:?}", path, code);
        // 获取path的目录部分
        let dir = Path::new(&path).parent().unwrap().to_str().unwrap();
        info!("Dir: {:?}", dir);
        // 获取path的文件名部分
        let file_name = Path::new(&path).file_name().unwrap().to_str().unwrap();
        info!("File Name: {:?}", file_name);
        // 判断dir是否存在
        let dir_path = base_path.join(dir);
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path).unwrap();
        }
        // 将code写入到file_name中
        let file_path = dir_path.join(file_name);
        fs::write(&file_path, &code).unwrap();

        // 获取文件扩展名
        let file_extension = {
            let ext = Path::new(&file_name).extension().unwrap();
            ext.to_str().unwrap()
        };
        info!("File Extension: {:?}", file_extension);
        let code_content = code.as_str();
        if file_extension == "c" || file_extension == "cpp" {
            // 判断是否是main函数所在的文件
            if code_content.contains("main(") || code_content.contains("int main(") || code_content.contains("void main(") || code_content.contains("main (") {
                info!("Main Function: {:?}", code_content);
                let re = Regex::new(r"-").unwrap();
                main_file_name = re.replace_all(&file_name, "_").to_string().replace(".c", "");
            }
            // 根据 dir_path 获取文件的绝对路径
            compile_command.push(CompileCommandItem {
                arguments: vec!["cc".to_string(), "-c".to_string(), file_name.to_string()],
                directory: dir_path.to_str().unwrap().to_string(),
                file: file_name.to_string(),
            });
        }
    }

    info!("Compile Command: {:?}", compile_command);
    // 将compile_command转换为json
    let compile_command_json = serde_json::to_string(&compile_command).unwrap();
    info!("Compile Command JSON: {:?}", compile_command_json);

    // 写入到compile_command.json中
    let compile_command_json_path = base_path.join("compile_command.json");
    fs::write(compile_command_json_path, compile_command_json).unwrap();

    let pn = params.project_name.replace("-", "_");

    info!("Main File Name: {:?}", main_file_name);

    // 使用Commond执行 interp
    let command = Command::new("c2rust")
        .current_dir(base_path.clone())
        .arg("transpile")
        .arg(format!("--binary {}", &main_file_name))
        .arg("compile_command.json")
        .arg("-o")
        .arg(format!("{}", pn))
        .output().expect("Failed to execute command");

    info!("Command: {:?}", command);
    let output = String::from_utf8(command.stdout).unwrap();
    info!("Output: {:?}", output);
    let error = String::from_utf8(command.stderr).unwrap();
    info!("Error: {:?}", error);

    if output.contains("Failed to execute command") {
        info!("Failed to execute command");
        return Json(TranscodeParams {
            project_name: params.project_name,
            content: vec![],
        });
    }

    let mut result = TranscodeParams {
        project_name: params.project_name,
        content: vec![],
    };

    result.project_name = pn.clone();

    let output_base_dir_path = Path::new(&base_dir).join(&pn);
    // 递归遍历output_base_dir_path下的所有文件并添加到result中
    fn recursive_read_dir(path: &Path, result: &mut TranscodeParams, main_file_name: &str) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path().to_path_buf();
            if path.is_dir() {
                recursive_read_dir(&path, result, main_file_name);
            } else {
                let is_main_file = check_file_name(path.file_name().unwrap().to_str().unwrap(), main_file_name);
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let mut file_content = fs::read_to_string(&path).unwrap();
                info!("File Name: {:?}, File Content: {:?}", file_name, file_content);
                if file_name == "lib.rs" {
                    let mut c = String::new();
                    let lib_content = file_content.split("\n").collect::<Vec<&str>>();
                    let mut index = 0;
                    for line in lib_content {
                        match index {
                            0..=9 => {}
                            _ => {
                                c.push_str(&format!("{}\n", line));
                            }
                        } 
                        index += 1;
                    }

                    let mut append_code = format!("\n{}", c);
                    let clins = c.split("\n").collect::<Vec<&str>>();
                    for line in clins {
                        if line.contains("pub mod src") {
                            append_code = append_code.replace("pub mod src ", "pub use src::");
                            // 替换pub mod 到; 之间的字符串
                            let re = Regex::new(r"pub mod (\s\S+);").unwrap();
                            append_code = re.replace(&append_code, "pub use $1::*,").to_string();
                            info!("Append Code: {:?}", append_code);
                        }
                    }
                    append_code.push_str(";");
                    info!("Append Code: {:?}", append_code);
                    file_content.push_str(&append_code);
                } else if is_main_file == true {
                    let mut code_c = String::new();
                    let code_content = file_content.split('\n').collect::<Vec<&str>>();
                    for (index, line) in code_content.iter().enumerate() {
                        if index == 1 {
                            continue;
                        }
                        if line.contains("type") && line.contains("_") {
                            // let re = Regex::new(r"type (\s\S+)_").unwrap();
                            // let replace_line = re.replace(line, "type $1").to_string();
                            // code_c.push_str(&format!("{}\n", replace_line));
                            // println!("Line: {:?}", line);
                        } else {
                            code_c.push_str(&format!("{}\n", line));
                        }
                    }
                    file_content = code_c;
                }
                result.content.push(TranscodePathParams {
                    path: file_name.to_string(),
                    code: file_content,
                });
            }
        }
    }
    recursive_read_dir(&output_base_dir_path, &mut result, &main_file_name);

    info!("Response Result: {:?}", result);

    Json(result)
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