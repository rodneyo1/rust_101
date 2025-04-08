use std::fs::File;

pub fn open_file(s: &str) -> File {
    File::open(s).unwrap()  // Either returns Ok(success_value) or Err(error_value). 
                           // If it is Ok, it returns the success_value. 
                           // If it is Err, it panics and prints the error message.
}

// //altrnatively
// pub fn open_file(s: &str) -> File {
//     match File::open(s) {
//         Ok(file) => file,
//         Err(e) => panic!("{}", e),
//     }
// }