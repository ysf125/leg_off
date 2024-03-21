use std::fs;

pub fn add_cpp_to_makefile(name: &str) {
    let mut makefile: String = fs::read_to_string("makefile").unwrap();
    makefile = makefile.replacen("cppFiles =", format!("cppFiles = .\\src\\{}.cpp", name).as_str(), 1);
    let _ = fs::write("makefile", makefile);
}

pub fn remove_cpp_from_makefile(name: &str) {
    let mut makefile: String = fs::read_to_string("makefile").unwrap();
    makefile = makefile.replacen(format!(".\\src\\{}.cpp", name).as_str(), "", 1);
    let _ = fs::write("makefile", makefile);
}