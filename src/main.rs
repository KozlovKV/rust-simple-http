use std::{collections::HashMap, env::args, fs::File, io::Read};

use axum::{extract, response::Html, routing, Router};
use fs_tree::FsTree;
use tower_http::services::ServeDir;

fn get_root_name() -> String {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        panic!("Specify the second argument as path to directory with index.html file")
    }
    args[1].clone()
}

fn read_file(path: &str) -> String {
    let file_str = File::open(path)
        .and_then(|mut file| {
            let mut internal_str = String::new();
            file.read_to_string(&mut internal_str)?;
            Ok(internal_str)
        })
        .or_else(|err| {
            Err(err.to_string())
        });
    match file_str {
        Ok(result_str) => result_str,
        Err(error) => error
    }
}

fn replace_templates(mut raw_string: String, params: &HashMap<String, String>) -> String {
    params.iter().for_each(|pair| {
        raw_string = raw_string.replace(&format!("{{{{ {} }}}}", pair.0), pair.1);
        raw_string = raw_string.replace(&format!("{{{{ {}}}}}", pair.0), pair.1);
        raw_string = raw_string.replace(&format!("{{{{{} }}}}", pair.0), pair.1);
        raw_string = raw_string.replace(&format!("{{{{{}}}}}", pair.0), pair.1);
    });
    raw_string
}

fn get_dir_tree(root: &str, indent: usize) -> String {
    let indent_str = "&nbsp".repeat(indent);
    let parsing_result = FsTree::symlink_read_at(root)
        .and_then(|tree_internal| {
            let mut str_tree_layer = String::new();
            tree_internal.children().iter().for_each(|child| {
                child.iter().for_each(|pair| {
                    str_tree_layer = format!("{}<br><code>{}{}</code>", str_tree_layer, indent_str, pair.0.display());
                    if pair.1.is_dir() {
                        let nested_dir_name = format!("{}/{}", root, pair.0.display());
                        str_tree_layer = format!(
                            "{}{}{}", str_tree_layer, indent_str, 
                            get_dir_tree(&nested_dir_name, indent + 2)
                        );
                    }
                })
            });
            Ok(str_tree_layer)
        })
        .or_else(|_| {
            Err("Failed to load dir")
        });
    match parsing_result {
        Ok(tree) => tree,
        Err(error_message) => format!("Error! {error_message}")
    }
}

async fn get_index_page() -> Html<String> {
    let root = get_root_name();
    let mut map = HashMap::new();
    map.insert("list".to_string(), get_dir_tree(
        &format!("{root}"), 0
    ));
    Html(replace_templates(read_file(
        &format!("{root}/index.html")
    ), &map))
}

async fn get_by_wildcard(
    extract::Path(file_path): extract::Path<String>,
    extract::Query(params): extract::Query<HashMap<String, String>>
) -> Html<String> {
    let root = get_root_name();
    let full_path = format!("{}/{}", root, file_path);
    let rendered_response = replace_templates(read_file(&full_path), &params);
    Html(rendered_response)
}

fn set_router() -> Router {
    let root = get_root_name();
    Router::new()
        .route("/", routing::get(get_index_page))
        .route("/render/*file_path", routing::get(get_by_wildcard))
        .nest_service("/static/", ServeDir::new(root))
}

#[tokio::main]
async fn main() {
    let router = set_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
