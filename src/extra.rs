use std::fs;
use crate::makefile;

pub fn new_project(name: &str) {
    let path_to_project: String = format!(".\\{}", name);
    let _ = fs::create_dir_all(format!("{}\\src", path_to_project));
    let _ = fs::create_dir_all(format!("{}\\include\\include", path_to_project));
    let _ = fs::create_dir_all(format!("{}\\build\\release", path_to_project));
    let _ = fs::create_dir_all(format!("{}\\build\\debug", path_to_project));
    make_maincpp(&path_to_project);
    make_makefile(&path_to_project);
}

fn make_makefile(path: &str) {
    let _ = fs::File::create(format!("{}\\makefile", path));
    let makefile: String = 
r"flags =
include = -Iinclude\include
cppFiles = .\src\main.cpp
        
run :
#g++ $(cppFiles) -o .\build\release\main.exe $(flags) $(include)
#.\build\release\main.exe
debug :
#g++ $(cppFiles) -o .\build\release\main.exe -g $(flags) $(include)
#gdb .\build\release\main.exe 
".replace("#", "\t");
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

pub fn new_include(name: &str) {
    let _ = fs::File::create(format!(".\\include\\include\\{}", name));
} 

pub fn remove_include(name: &str) {
    let _ = fs::remove_file(format!(".\\include\\include\\{}.hpp", name));
}

pub fn new_cpp(name: &str) {
    let _ = fs::File::create(format!(".\\src\\{}.cpp", name));
    makefile::add_cpp_to_makefile(name);
}

pub fn remove_cpp(name: &str) {
    let _ = fs::remove_file(format!(".\\src\\{}.cpp", name));
    makefile::remove_cpp_from_makefile(name);
}