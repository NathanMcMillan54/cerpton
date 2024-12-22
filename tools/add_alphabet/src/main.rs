use std::{fs::read_to_string, thread::sleep, time::Duration};

use dlcns::{get::CNSGet, name::Owner};
use dlwp::{message::contents_to_string, stream::{Stream, StreamType}};

fn main() {
    let alphabet_file_contents = read_to_string("alphabet.txt").unwrap();
    let split = alphabet_file_contents.split("\n").collect::<Vec<&str>>();
    let mut alphabet = vec![];
    let mut added_chars = vec![];
    for i in 0..split.len() {
        alphabet.push(split[i].chars().last().unwrap());
    }

    println!("Conencting to DLCNS...");
    let mut cns_get = CNSGet::new();

    println!("Getting all CNS names...");
    let try_all = cns_get.get_all_names();

    if try_all.is_none() {
        panic!("Failed to obtain CNS names list");
    }

    cns_get.disconnect();

    let all = try_all.unwrap();

    for i in 0..all.len() {
        let mut stream = Stream::new(StreamType::Client { rid: all[i].id, rdid: all[i].did, port: all[i].port }, false);
        stream.start();
        sleep(Duration::from_millis(100));

        let mut read = stream.read();
        while read.is_empty() {
            read = stream.read();
        }

        for msg in &read {
            let contents = contents_to_string(msg.contents);
            for c in contents.chars() {
                if !alphabet.contains(&c) {
                    added_chars.push(c);
                }
            }
        }
    }

    if !added_chars.is_empty() {
        println!("Added new characters: {:?}", added_chars);
        let _ = added_chars.iter().map(|c| alphabet.push(*c));
    } else {
        println!("No new added characters");
    }
}
