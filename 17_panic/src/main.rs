use std::{fs::File, io::{Error, ErrorKind}};
fn main() {
    let fd = File::open("Hello.txt");
    let fd = match fd {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("{:?}",e)
            },
            oe => panic!("{:?}",oe),
        }
    };
}
