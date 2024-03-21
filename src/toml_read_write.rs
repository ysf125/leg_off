use std::fs;

struct Toml {
    name: String,
    cpp_files: Vec<String>,
    dependencies: Dependencies
}

struct Dependencies {}

fn get_data() -> toml::Value {
    let toml_contents: String = fs::read_to_string(".\\projectInfo.toml").unwrap();
    toml::from_str(&toml_contents).unwrap()
}