use std::fs;
use crate::toml_read_write;

pub fn new_project(name: &str) {
    let path_to_project: String = format!(".\\{}", name);
    let _ = fs::create_dir_all(format!("{}\\src", path_to_project));
    let _ = fs::create_dir_all(format!("{}\\include\\include", path_to_project));
    let _ = fs::create_dir_all(format!("{}\\build\\release", path_to_project));
    let _ = fs::create_dir_all(format!("{}\\build\\debug", path_to_project));
    make_maincpp(&path_to_project);
    make_makefile(&path_to_project);
    make_toml_file(&path_to_project);
}

fn make_makefile(path: &str) {
    let _ = fs::File::create(format!("{}\\makefile", path));
    let makefile: String = 
r#"flags =
include = -Iinclude\include
cppFiles = .\src\main.cpp
        
run :
#g++ $(cppFiles) -o .\build\release\main.exe $(flags) $(include)
#.\build\release\main.exe
debug :
#g++ $(cppFiles) -o .\build\release\main.exe -g $(flags) $(include)
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
    std::cout << "Hello world!\n";
    return 0;
}
"#;
    let _ = fs::write(format!("{}\\src\\main.cpp", path), maincpp);
}

fn make_toml_file(path: &str) {
    let _ = fs::File::create(format!("{}\\projectInfo.toml", path));
    let toml:String  = 
r#"name = "$"
cppFiles = ["main.cpp"]

[dependencies]
"#.replace("$", &path.replace(".\\", ""));
    let _ = fs::write(format!("{}\\projectInfo.toml", path), toml);
}

pub fn new_include_file(name: &str) {
    let _ = fs::File::create(format!(".\\include\\include\\{}", name));
} 

pub fn new_cpp_file(name: &str) {
    let _ = fs::File::create(format!(".\\src\\{}.cpp", name));
    // Edit make file
}