use std::collections::HashMap;
use handlebars::Handlebars;
mod open_api;
use serde::{Serialize, Deserialize};

struct Spec {
    file_name: String
}

impl Spec {
    pub fn parse(&self) -> () {
        println!("{:?}", self.file_name);
        ()
    }
}

fn open_file(file_name: &str) -> String {
    let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push(file_name);

    let file_path = dir.to_str().unwrap();

    std::fs::read_to_string(file_path).unwrap()
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Route {
    path: String,
    method: String,
    path_params: Vec<(String, String)>,
    query_params: Vec<(String, String)>,
    body_params: Vec<(String, String)>,
    // headers: Vec<(String, String)>
}

fn main() {
    let data = open_file("resources/petstore.yaml");
    let schema: open_api::OpenApi = serde_yaml::from_str(&data).unwrap();

    let template = open_file("resources/template.hbs");
    println!("{:?}", &template);
    let mut handlebars = Handlebars::new();
    let mut data = HashMap::new();
    let mut paths: Vec<Route> = vec![];
    for (path, path_item) in schema.paths {
        if let Some(get_operation) = path_item.get {
             let query_params = get_operation.parameters.iter().filter(|&param| match param {
                open_api::SchemaType::Definition(p) => p.placement == Some(String::from("query")),
                _ => false
            }).map(|param| {
                if let open_api::SchemaType::Definition(p) = param {
                    let schema = p.schema.as_ref().unwrap();
                    if let open_api::JsonSchema::Obj(data) = schema {
                        return (p.name.clone(), data.data_type.clone().unwrap());
                    }
                }
                (String::from("NA"), String::from("NA"))
            }).collect::<Vec<(String, String)>>();
            println!("{:?}", path);
            paths.push(Route{
                path,
                method: String::from("GET"),
                query_params
            });
        }
    }
    data.insert("routes", &paths);
    handlebars
        .register_template_string("api", template)
        .unwrap();

    println!("res {:?}", handlebars.render("api", &data).unwrap());
}
