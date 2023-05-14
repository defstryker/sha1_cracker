use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const SHA1_HASH_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker <wordlist.txt> <sha1_hash>");
        return Ok(()); 
    }

    // checking the validity of the hash
    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HASH_STRING_LENGTH {
        return Err("SHA1 hash is invalid".into());
    }

    // opening the wordlist
    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    // read lines
    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();
        // println!("{}", line);

        if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password found: {}", &common_password);
            return Ok(());
        }
    }

    println!("Password not found in wordlist ...");

    Ok(())
}
