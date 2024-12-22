use std::{env::{self, current_dir}, fs::{read_to_string, File}, io::Write};

fn main() {
    let alphabet_file_contents = read_to_string("alphabet.txt").unwrap();
    let split = alphabet_file_contents.split("\n").collect::<Vec<&str>>();
    let mut alphabet = vec![];
    for i in 0..split.len() {
        alphabet.push(split[i].chars().last().unwrap());
    }

    const COMMENT: &str = "/// This alphabet is made of characters that come from different alphabets. Uppercase and lowercase letters are
/// different from each other there are also numbers and special characters. This is not supposed to be arranged in any
/// specific way";

    let mut alphabet_rs_file = format!("{}\n", COMMENT);
    alphabet_rs_file.push_str("pub(crate) const ALPHABET: [(char, u32); ALPHABET_LEN as usize] = [\n");

    for i in 0..alphabet.len() {
        alphabet_rs_file.push_str(&format!("\t(\'{}\', {}),\n", alphabet[i], i));
    }

    alphabet_rs_file.push_str("];\n");
    alphabet_rs_file.push_str("/// (character, value, has been set)\n
pub type Alphabet = (char, u32, bool);\n");
    alphabet_rs_file.push_str(&format!("
pub const ALPHABET_LEN: i32 = {};\n", alphabet.len()));

    let project_dir = current_dir().unwrap();
    let mut alphabet_rs = File::options().create(true).write(true).open(
        project_dir.join("src/alphabet.rs")).unwrap();
    alphabet_rs.write_fmt(format_args!("{}", alphabet_rs_file)).unwrap();
}
