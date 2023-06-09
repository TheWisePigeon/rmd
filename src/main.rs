use std::env::args;


fn main() {
    let file_path = args().nth(1);
    let output_destination = args().nth(2);
    match file_path {
        Some(path)=>{

        },
        None=>{
            //Display help message
        }
    }
}
