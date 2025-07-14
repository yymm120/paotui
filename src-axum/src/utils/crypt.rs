#![allow(unused)]
use anyhow::{Context, anyhow};
use tokio::task;

use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash};

pub async fn hash(password: String) -> anyhow::Result<String> {
  task::spawn_blocking(move || {
    use std::time::Instant;
    let start = Instant::now();
    let salt = SaltString::generate(&mut OsRng);
    let res = Argon2::default()
      .hash_password(password.as_bytes(), &salt)
      .map_err(|e| anyhow!(e).context("failed to hash password"))?
      .to_string();
    let duration = start.elapsed();
    println!("函数执行时间: {:?}", duration);
    Ok(res)
  })
  .await
  .context("panic in hash()")?
}

pub async fn verify(password: String, hash: String) -> anyhow::Result<bool> {
  task::spawn_blocking(move || {
    use std::time::Instant;
    let start = Instant::now();
    let hash =
      PasswordHash::new(&hash).map_err(|e| anyhow!(e).context("BUG: password hash invalid"))?;

    let res = Argon2::default().verify_password(password.as_bytes(), &hash);
    let duration = start.elapsed();
    println!("函数执行时间: {:?}", duration);
    match res {
      Ok(()) => Ok(true),
      Err(password_hash::Error::Password) => Ok(false),
      Err(e) => Err(anyhow!(e).context("failed to verify password")),
    }
  })
  .await
  .context("panic in verify()")?
}

#[cfg(test)]
mod crypt_tests {
  use crate::utils::crypt::hash;

  #[tokio::test]
  async fn test_crypt_hash() -> anyhow::Result<()> {
    // use std::time::Instant;
    // let start = Instant::now();
    let hashed = hash("hello crypt".to_string()).await?;
    // let duration = start.elapsed();
    println!("{}", hashed);
    // println!("函数执行时间: {:?}", duration);
    Ok(())
  }
}
