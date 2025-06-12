use rand::seq::{IteratorRandom, SliceRandom};
use zxcvbn::zxcvbn;

pub fn process_genpass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    digits: bool,
    symbols: bool,
) -> anyhow::Result<()> {
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if uppercase {
        chars.extend(('A'..='Z').collect::<Vec<char>>());
        password.push(
            ('A'..='Z')
                .choose(&mut rng)
                .expect("Failed to choose a character"),
        );
    }

    if lowercase {
        chars.extend(('a'..='z').collect::<Vec<char>>());
        password.push(
            ('a'..='z')
                .choose(&mut rng)
                .expect("Failed to choose a character"),
        );
    }

    if digits {
        chars.extend(('0'..='9').collect::<Vec<char>>());
        password.push(
            ('0'..='9')
                .choose(&mut rng)
                .expect("Failed to choose a character"),
        );
    }

    if symbols {
        chars.extend("!@#$%^&*_".chars().collect::<Vec<char>>());
        password.push(
            "!@#$%^&*_"
                .chars()
                .choose(&mut rng)
                .expect("Failed to choose a character"),
        );
    }

    for _ in 0..(length - password.len() as u8) {
        password.push(chars[rand::random_range(0..chars.len())]);
    }

    password.shuffle(&mut rng);

    let password_str = String::from_iter(password);
    let estimate = zxcvbn(&password_str, &[]);
    println!("Password: {} Score: {}", password_str, estimate.score());

    Ok(())
}
