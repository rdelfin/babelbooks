use crate::database;
use anyhow::{anyhow, Result};
use diesel::sqlite::SqliteConnection;
use rand::{distributions::Alphanumeric, Rng};

pub fn create_and_login(
    dbconn: &SqliteConnection,
    username: &str,
    plain_password: &str,
) -> Result<(i32, String)> {
    let id = database::add_user(
        &dbconn,
        &username,
        &libpasta::hash_password(&plain_password),
    )?;
    Ok((id, login_user(dbconn, username, plain_password)?))
}

pub fn login_user(
    dbconn: &SqliteConnection,
    username: &str,
    plain_password: &str,
) -> Result<String> {
    if libpasta::verify_password(
        &plain_password,
        &database::get_hashed_password(dbconn, username)?,
    ) {
        return Err(anyhow!("Username does not match"));
    }
    let session_id = gen_session();
    database::add_session(
        dbconn,
        &session_id,
        database::get_id_for_username(dbconn, username)?,
    )?;
    Ok(session_id.into())
}

fn gen_session() -> String {
    // Thread rng is cryptographically secure as per documentation: https://rust-random.github.io/book/guide-rngs.html
    let rng = rand::thread_rng();

    rng.sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect()
}

pub fn verify(dbconn: &SqliteConnection, session_id: &str) -> Result<()> {
    database::get_user_for_session(dbconn, session_id)?
        .ok_or(anyhow!("User session is invalid"))?;
    Ok(())
}
