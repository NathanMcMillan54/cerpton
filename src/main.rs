use cerpton::{libcerpton_decode, libcerpton_encode};
use std::env;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

fn main() {
    println!("Cerpton v0.1.2");
    let args: Vec<String> = env::args().collect();

    let encode = &args[1];
    let s1 = args[2].parse::<i32>().unwrap();
    let s2 = args[3].parse::<i32>().unwrap();
    let s3 = args[4].parse::<i32>().unwrap();
    let file = &args[5];
    let output = &args[6];

    if encode == &String::from("encode") {
        let mut output_file = File::create(output).unwrap();
        output_file
            .write_fmt(format_args!(
                "{}",
                libcerpton_encode(
                    [s1, s2, s3, 0, 0, 0],
                    read_to_string(file).unwrap()
                )
            ))
            .unwrap();
        println!("Encoded {} and saved to {}", file, output);
    } else if encode == &String::from("decode") {
        let mut output_file = File::create(output).unwrap();
        output_file.write_fmt(format_args!(
            "{}",
            libcerpton_decode(
                [s1, s2, s3, 0, 0, 0],
                read_to_string(file).unwrap()
            )
        )).unwrap();
        println!("Decoded {} and saved to {}", file, output);
    }
}
