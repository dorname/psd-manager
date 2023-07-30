pub mod merhash;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       merhash::mersenne_hash("test");
       let x = 3 as usize;
       println!("{}",x.pow(2));
    }
}
