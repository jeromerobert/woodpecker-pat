use clap::Parser;
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, header::HeaderType};
use rusqlite::{Connection, OpenFlags};
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

fn map_query(row: &rusqlite::Row) -> rusqlite::Result<(u32, String)> {
    row.get(0).and_then(|v1| row.get(1).map(|v2| (v1, v2)))
}

/// Detects whether the database uses the new schema (id, login) or the old one
/// (`user_id`, `user_login`)
fn detect_columns(conn: &Connection) -> (&str, &str, &str) {
    let mut stmt = conn
        .prepare("PRAGMA table_info(users)")
        .expect("Failed to query table info");

    let columns: Vec<String> = stmt
        .query_map([], |row| row.get::<_, String>(1))
        .expect("Failed to read table info")
        .map(|r| r.unwrap())
        .collect();

    // Check for the presence of the new 'login' column
    if columns.iter().any(|c| c == "login") {
        ("id", "login", "hash")
    } else {
        ("user_id", "user_login", "user_hash")
    }
}

fn main() {
    let options = CliOptions::parse();
    let conn = Connection::open_with_flags(options.sqlite_file, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .expect("Failed to open database");
    let (id_col, login_col, hash_col) = detect_columns(&conn);
    let user_login = options.user_login.unwrap_or_else(|| {
        conn.query_row(
            &format!("SELECT {login_col} FROM users ORDER BY ROWID ASC LIMIT 1"),
            [],
            |row| row.get::<_, String>(0),
        )
        .unwrap_or_else(|e| {
            panic!("User not found or an error occurred: {e}");
        })
    });
    let (user_id, user_hash) = conn
        .query_row(
            &format!("SELECT {id_col}, {hash_col} FROM users WHERE {login_col} = ?1"),
            [&user_login],
            map_query,
        )
        .unwrap_or_else(|e| {
            panic!("User not found or an error occurred: {e}");
        });
    let user_id = user_id.to_string();
    // The default header type in Rust jwt is empty while it's "JWT" in the Go jwt.
    let header = Header {
        algorithm: AlgorithmType::Hs256,
        type_: Some(HeaderType::JsonWebToken),
        ..Default::default()
    };
    let mut claims = BTreeMap::new();
    claims.insert("type", "user");
    claims.insert("user-id", &user_id);
    let hs256_key: Hmac<Sha256> = Hmac::new_from_slice(user_hash.as_bytes()).unwrap();
    let s = jwt::Token::new(header, claims).sign_with_key(&hs256_key);
    println!("{}", s.unwrap().as_str());
}
