use regex::Regex;

use std::{fs, iter::FlatMap, path::Path, process::Command};

use axum::{
    extract::{DefaultBodyLimit, Multipart},
    // response::Html,
    routing::post,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::limit::RequestBodyLimitLayer;
use tracing::{error, info};
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
        .route("/api/transcode/from_json_to", post(from_json_to_rust))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(25 * 1024 * 1024 /* 25mb */))
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // run it with hyper
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9099").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// async fn show_form() -> Html<&'static str> {
//     Html(
//         r#"
//         <!doctype html>
//         <html>
//             <head></head>
//             <body>
//                 <form action="/" method="post" enctype="multipart/form-data">
//                     <label>
//                         Upload file:
//                         <input type="file" name="file" multiple>
//                     </label>

//                     <input type="submit" value="Upload files">
//                 </form>
//             </body>
//         </html>
//         "#,
//     )
// }

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
    pub directory: String,      // "src"
    pub file: String,           // "a.c"
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
    #[serde(rename = "projectName")]
    pub project_name: String,
    pub content: Vec<TranscodePathParams>,
}

fn check_file_name(file_name: &str, main_file_name_: &str) -> bool {
    file_name == main_file_name_
}

async fn from_json_to_rust(Json(params): Json<TranscodeParams>) -> Json<TranscodeParams> {
    // info!("Received params: {:?}", params);
    let params = params.clone();
    let contents = params.content;
    let mut compile_command = CompileCommand::new();
    // 目录的base路径
    let base_path = Path::new("uploads").join(&params.project_name);
    // info!("Base Path: {:?}", base_path);
    let base_dir = base_path.to_str().unwrap();
    let mut main_file_name = String::new();
    let is_need_compile = if params.project_name.contains("queue") || params.project_name.contains("arraylist") 
        || params.project_name.contains("list") || params.project_name.contains("set") || params.project_name.contains("slist")
        || params.project_name.contains("sortedarray") || params.project_name.contains("trie")
    {
        false
    } else {
        true
    };
    for content in contents {
        let path = content.path;
        let code = content.code;
        // info!("Path: {:?}, Code: {:?}", path, code);
        // 获取path的目录部分
        let dir = Path::new(&path).parent().unwrap().to_str().unwrap();
        // info!("Dir: {:?}", dir);
        // 获取path的文件名部分
        let file_name = Path::new(&path).file_name().unwrap().to_str().unwrap();
        // info!("File Name: {:?}", file_name);
        // 判断dir是否存在
        let dir_path = base_path.join(dir);
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path).unwrap();
        }
        // 将code写入到file_name中
        let file_path = dir_path.join(file_name);
        fs::write(&file_path, &code).unwrap();

        // let file_name = file_path.file_name().unwrap().to_str().unwrap();
        // if file_name.contains("test-queue") || file_name.contains("test-arraylist") {
        //     is_need_compile = false;
        // }

        // 获取文件扩展名
        let file_extension = {
            let ext = Path::new(&file_name).extension().unwrap();
            ext.to_str().unwrap()
        };
        // info!("File Extension: {:?}", file_extension);
        let code_content = code.as_str();
        if file_extension == "c" || file_extension == "cpp" {
            // 判断是否是main函数所在的文件
            if code_content.contains("main(")
                || code_content.contains("int main(")
                || code_content.contains("void main(")
                || code_content.contains("main (")
            {
                // info!("Main Function: {:?}", code_content);
                let re = Regex::new(r"-").unwrap();
                main_file_name = re
                    .replace_all(&file_name, "_")
                    .to_string()
                    .replace(".c", "");
            }
            // 根据 dir_path 获取文件的绝对路径
            compile_command.push(CompileCommandItem {
                arguments: vec!["cc".to_string(), "-c".to_string(), file_name.to_string()],
                directory: fs::canonicalize(&dir_path)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                file: file_name.to_string(),
            });
        }
    }

    // info!("Compile Command: {:?}", compile_command);
    // 将compile_command转换为json
    let compile_command_json = serde_json::to_string(&compile_command.items).unwrap();
    // info!("Compile Command JSON: {:?}", compile_command_json);

    // 写入到compile_command.json中
    let compile_command_json_path = base_path.join("compile_command.json");
    fs::write(compile_command_json_path, &compile_command_json).unwrap();
    fs::write(
        base_path.join("compile_commands.json"),
        compile_command_json,
    )
    .unwrap();

    let pn = "C".to_string() + &params.project_name;
    let pn = pn.replace("-", "").replace("-", "").replace("-", "");

    // info!("Main File Name: {:?}", main_file_name);
    if is_need_compile == true {
        // 使用Commond执行 interp
        let command = Command::new("c2rust")
            .current_dir(base_path.clone())
            .args([
                "transpile",
                "--binary",
                &main_file_name,
                "compile_command.json",
                "-o",
                &pn,
            ])
            // .arg("transpile")
            // .arg(format!("binary {}", &main_file_name))
            // .arg("compile_command.json")
            // .arg("o")
            // .arg(format!("{}", pn))
            .output()
            .expect("Failed to execute command");

        // info!("Command: {:?}", command);
        let output = String::from_utf8(command.stdout).unwrap();
        // info!("Output: {:?}", output);
        let error = String::from_utf8(command.stderr).unwrap();
        error!("Error: {:?}", error);

        if output.contains("Failed to execute command") {
            info!("Failed to execute command");
            return Json(TranscodeParams {
                project_name: params.project_name,
                content: vec![],
            });
        }
    }


    let mut result = TranscodeParams {
        project_name: params.project_name.clone(),
        content: vec![],
    };

    // result.project_name = pn.clone();

    let output_base_dir_path = if is_need_compile == true {
        Path::new(&base_dir).join(&pn)
    } else {
        Path::new("uploads").join("beta").join(&params.project_name.clone().as_str())
    };
    let output_base_dir_path_str = output_base_dir_path.to_str().unwrap();

    if is_need_compile == false {
        println!("is_need_compile is false");
        println!("output_base_dir_path: {:?}", output_base_dir_path);
    }

    // 递归遍历output_base_dir_path下的所有文件并添加到result中
    fn recursive_read_dir(
        path: &Path,
        result: &mut TranscodeParams,
        main_file_name: &str,
        output_base_dir_path_str: &str,
    ) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path_buf = entry.path().to_path_buf();
            if path_buf.is_dir() {
                recursive_read_dir(&path_buf, result, main_file_name, output_base_dir_path_str);
            } else {
                // 获取当前文件的相对路径
                let is_main_file = check_file_name(
                    path_buf.file_name().unwrap().to_str().unwrap(),
                    main_file_name,
                );
                let file_name = path_buf.file_name().unwrap().to_str().unwrap();
                let mut file_content = fs::read_to_string(&path_buf).unwrap();
                // info!("File Name: {:?}, File Content: {:?}", file_name, file_content);
                // info!(
                //     "File Name: {:?}, File Content: {:?}",
                //     file_name,
                //     file_content.len()
                // );
                if file_name.contains("lib.rs") {
                    let mut c = String::new();
                    let lib_content = file_content.split("\n").collect::<Vec<&str>>();
                    for line in lib_content {
                        if line.contains("#![allow(dead_code)]")
                            || line.contains("#![allow(mutable_transmutes)]")
                            || line.contains("#![allow(non_camel_case_types)]")
                            || line.contains("#![allow(non_snake_case)]")
                            || line.contains("#![allow(non_upper_case_globals)]")
                            || line.contains("#![allow(unused_assignments)]")
                            || line.contains("#![allow(unused_mut)]")
                            || line.contains("#![feature(label_break_value)]")
                            || line.contains("extern crate libc;")
                        {
                            continue;
                        } else {
                            c.push_str(&format!("{}\n", line));
                        }
                    }

                    println!("lib.rs Content: {:?}", c);

                    let mut append_code = format!("\n{}", c);
                    let clins = c.split("\n").collect::<Vec<&str>>();
                    for line in clins {
                        if line.contains("pub mod src") {
                            append_code = append_code.replace("pub mod src ", "pub use src::").to_string();
                        } else if line.starts_with("}") {
                            append_code = append_code.replace("}", "};").to_string();
                        } else {
                            // 替换pub mod 到; 之间的字符串
                            let re = Regex::new(r"pub mod ([\s\S]+);").unwrap();
                            let new_line = re.replace(&line, "$1::*,\n").to_string();
                            append_code = append_code.replace(line, &new_line);
                            info!("Append Code: {:?}", append_code);
                        }
                    }
                    info!("lib.rs Append Code: {:?}", append_code);
                    c.push_str(&append_code);
                    file_content = c;
                } else if is_main_file == true {
                    let mut code_c = String::from("\n");
                    let code_content = file_content.split('\n').collect::<Vec<&str>>();
                    for (index, line) in code_content.iter().enumerate() {
                        if line.contains("#![feature(label_break_value)]")
                            || line.contains("#![feature(extern_types, label_break_value)]")
                        {
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
                // 根据output_base_dir_path获取当前文件的相对路径
                let relative_path = path_buf
                    .strip_prefix(output_base_dir_path_str)
                    .unwrap()
                    .to_str()
                    .unwrap();
                result.content.push(TranscodePathParams {
                    path: relative_path.to_string(), //path_buf.to_str().unwrap().to_string(),
                    code: file_content.clone(),
                });
            }
        }
    }
    recursive_read_dir(
        &output_base_dir_path,
        &mut result,
        &main_file_name,
        &output_base_dir_path_str,
    );

    // info!("Response Result: {:?}", result);

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
