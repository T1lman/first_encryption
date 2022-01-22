pub fn encrypt(string: Vec<u8>, key: String) -> Vec<u8> {
    let len = string.len();
    let key = correctkey(key, len);
    let keybytes = key.as_bytes();
    let mut returnvec: Vec<u8> = Vec::new();
    for i in 0..len {
        let num: u8 = (((string[i] as u16) + (keybytes[i] as u16)) % u8::MAX as u16) as u8;
        returnvec.push(num);
    }
    returnvec
}
pub fn decrypt(string: Vec<u8>, key: String) -> Vec<u8> {
    let len = string.len();
    let key = correctkey(key, len);
    let keybytes = key.as_bytes();
    let mut returnvec: Vec<u8> = Vec::new();
    for i in 0..len {
        let num: u8 = (((string[i] as u16) - (keybytes[i] as u16)) % u8::MAX as u16) as u8;
        returnvec.push(num);
    }
    returnvec
}

fn correctkey(key: String, len: usize) -> String {
    let len = len as isize;
    let keylen = key.len();
    let num: isize = keylen as isize - len as isize;
    match num.cmp(&len) {
        std::cmp::Ordering::Equal => key,
        _ => repeat(key, len as usize),
    }
}

fn repeat(string: String, repeat: usize) -> String {
    let num = repeat / string.len() + 1;
    string.repeat(num)[..repeat].to_string()
}
