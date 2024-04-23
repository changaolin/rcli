use anyhow::{Ok, Result};
use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;
const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
) -> Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = vec![];
    let mut pool = vec![];
    if uppercase {
        pool.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).unwrap());
    }
    if lowercase {
        pool.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).unwrap());
    }
    if number {
        pool.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).unwrap());
    }
    if symbol {
        pool.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).unwrap());
    }
    for _ in 0..(length - password.len() as u8) {
        let c = pool.choose(&mut rng).unwrap();
        password.push(*c);
    }
    password.shuffle(&mut rng);
    let pwd = String::from_utf8(password);
    let estimate = zxcvbn(pwd.as_ref().unwrap(), &[])?;
    eprintln!("Password {:?} strength: {}", pwd.unwrap(), estimate.score());
    Ok(())
}
