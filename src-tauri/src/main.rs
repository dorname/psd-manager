#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use anyhow::{bail,Result};
use encryptor::password::generate_password;
#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}

#[tauri::command]
fn genpsd(seed: String,length:usize) -> String {
   if seed.len()<4 {
     format!("seed {} length must >= 4",seed);
   }
   let (se,len) = (&seed,length);
   println!("{},{}",se,len);
   let psd = generate_password(&se[..],len);
   let re = match psd {
      Ok(val) => val,
      Err(err) => err.to_string()
   };
   re
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
      println!("{}",genpsd(String::from("liguangqiao"),16));
    }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet,genpsd])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
