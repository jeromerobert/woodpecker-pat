use clap::Parser;
use hmac::{Hmac, Mac};
use jwt::{header::HeaderType, AlgorithmType, Header, SignWithKey};
use rusqlite::Connection;
use sha2::Sha256;
use std::collections::BTreeMap;

#[derive(Parser, Debug)]
#[clap(
    name = "woodpecker-pat",
    version = "1.0",
    author = "Jérôme Robert",
    about = "Extract a personal access token from a Woodpecker CI sqlite database"
)]
struct CliOptions {
    #[arg(short, help = "Sets the username (default: the first created user)")]
    user_login: Option<String>,

    #[arg(default_value = "woodpecker.sqlite", help = "Sets the SQLite file")]
    sqlite_file: String,
}

fn main() {
    let options = CliOptions::parse();
    let conn = Connection::open(options.sqlite_file).expect("Failed to open database");
    let user_login = options.user_login.map_or_else(
        || {
            conn.query_row(
                "SELECT user_login FROM users ORDER BY ROWID ASC LIMIT 1",
                [],
                |row| row.get::<_, String>(0),
            )
            .map_or_else(
                |_| {
                    panic!("User not found or an error occurred.");
                },
                |user_login| user_login,
            )
        },
        |n| n,
    );
    let Ok(user_hash) = conn.query_row(
        "SELECT user_hash FROM users WHERE user_login = ?1",
        [&user_login],
        |row| row.get::<_, String>(0),
    ) else {
        panic!("User not found or an error occurred.");
    };
    // The default header type in Rust jwt is empty while it's "JWT" in the Go jwt.
    let header = Header {
        algorithm: AlgorithmType::Hs256,
        type_: Some(HeaderType::JsonWebToken),
        ..Default::default()
    };
    let mut claims = BTreeMap::new();
    claims.insert("type", "user");
    claims.insert("text", &user_login);
    let hs256_key: Hmac<Sha256> = Hmac::new_from_slice(user_hash.as_bytes()).unwrap();
    let s = jwt::Token::new(header, claims).sign_with_key(&hs256_key);
    println!("{}", s.unwrap().as_str());
}
