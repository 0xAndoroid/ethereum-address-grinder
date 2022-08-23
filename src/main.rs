use secp256k1::{generate_keypair};
use rand::thread_rng;
use sha3::{Digest, Keccak256};

fn main() {
    for i in 0..10 {  //The amount of threads to use
        let handle = std::thread::spawn(move || thread_logic(i));
        if i==9 {
            handle.join().unwrap();
        }
    }
}

fn thread_logic(id: u8) {
    let mut count: u64 = 0;
    loop {
        let (secret_key, public_key) = generate_keypair(&mut thread_rng());
        let mut pubkey_vec = public_key.serialize_uncompressed().to_vec();
        pubkey_vec.remove(0);
        let mut hasher = Keccak256::new();
        hasher.update(pubkey_vec);
        let mut address = String::from("0x");
        address.push_str(&hex::encode(&hasher.finalize().to_vec()[12..32]));
        //Address matching
        if     address.starts_with("0x000000")
            || address.starts_with("0x111111")
            || address.starts_with("0x222222")
            || address.starts_with("0x333333")
            || address.starts_with("0x444444")
            || address.starts_with("0x555555")
            || address.starts_with("0x666666")
            || address.starts_with("0x777777")
            || address.starts_with("0x888888")
            || address.starts_with("0x999999")
            || address.starts_with("0xaaaaaa")
            || address.starts_with("0xbbbbbb")
            || address.starts_with("0xcccccc")
            || address.starts_with("0xdddddd")
            || address.starts_with("0xeeeeee")
            || address.starts_with("0xffffff") {
            println!("Found address {}. Saved private key to file", address);
            std::fs::write(address, secret_key.display_secret().to_string());
        }
        count+=1;
        if count%250000 == 0 {
            println!("[{}] Processed {} wallets.", id, count);
        }
    }
}
