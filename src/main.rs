mod algo;

fn main() {
    let key = String::from("This is a key!");
    let str = String::from("Hello world!");
    let encrypted = algo::encrypt(str.as_bytes().to_vec(), key.clone());
    let decrypt = algo::decrypt(encrypted, key);
    println!("{}", String::from_utf8(decrypt).unwrap());
}
