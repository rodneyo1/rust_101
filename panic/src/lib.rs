use std::fs::File;

pub fn open_file(s: &str) -> File {
    File::open(s).unwrap()
}

// //altrnatively
// pub fn open_file(s: &str) -> File {
//     match File::open(s) {
//         Ok(file) => file,
//         Err(e) => panic!("{}", e),
//     }
// }