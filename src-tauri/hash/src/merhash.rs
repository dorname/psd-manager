
pub fn mersenne_hash(seed:&str) -> usize{
    let mut hash:usize = 0;
    for(i,c) in seed.chars().enumerate() {
        // let c1 = c.clone() as usize;
        // println!("i:{i},c:{c},c1:{c1}");
        hash += (i+1)*(c as usize);
        println!("hash:{hash}");
    }
    (hash % 127).pow(3)-1
}
