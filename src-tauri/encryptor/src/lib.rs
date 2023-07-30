pub mod password;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let seed = "jdwnp";
        let length = 16;
        let password = password::generate_password(seed, length);
        match password {
            Ok(val) => println!("{:#?}", val),
            Err(err) => println!("{:#?}", err),
        }
    }
}
