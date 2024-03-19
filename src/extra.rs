use std::fs;

pub fn new_project(project_name: &str) {
    let path_to_project: String = format!(".\\{}", project_name);
    let _ = fs::create_dir_all(format!("{}\\src", path_to_project));
    let _ = fs::create_dir_all(format!("{}\\build\\release", path_to_project));
    let _ = fs::create_dir_all(format!("{}\\build\\debug", path_to_project));
    make_maincpp(&path_to_project);
    make_json_file(&path_to_project);
    make_makefile(&path_to_project);
}

fn make_json_file(path: &str) {
    let _ = fs::File::create(format!("{}\\projectInfo.json", path));
    let json: String = format!(r#"{{ "name" : "{}", "dependencies" : [] }}"#, path.replace(r".\", ""));
    let _ = fs::write(format!("{}\\projectInfo.json", path), json);
}

fn make_makefile(path: &str) { 
    let _ = fs::File::create(format!("{}\\makefile", path));
    let makefile: String = 
r#"flags =
include = 
staticLibraries = 
dynamicLibraries =
        
run :
#g++ .\src\main.cpp -o .\build\release\main.exe $(flags)
#.\build\release\main.exe
debug :
#g++ .\src\main.cpp -o .\build\release\main.exe -g $(flags)
#gdb .\build\release\main.exe 
"#.replace("#", "\t");
    let _ = fs::write(format!("{}\\makefile", path), makefile);
}

fn make_maincpp(path: &str) {
    let _ = fs::File::create(format!("{}\\src\\main.cpp", path));
    let maincpp: &str = 
r#"#include <iostream>

int main(int argc, char** argv)
{    
    std::cout << "Hello world!" << "\n";
    return 0;
}
"#;
    let _ = fs::write(format!("{}\\src\\main.cpp", path), maincpp);
}