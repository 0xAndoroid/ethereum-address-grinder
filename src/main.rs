use secp256k1::{generate_keypair};
use rand::thread_rng;
use sha3::{Digest, Keccak256};

fn main() {
    for i in 0..10 {
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
        if     address.starts_with("0x00000000")
            || address.starts_with("0x11111111")
            || address.starts_with("0x22222222")
            || address.starts_with("0x33333333")
            || address.starts_with("0x44444444")
            || address.starts_with("0x55555555")
            || address.starts_with("0x66666666")
            || address.starts_with("0x77777777")
            || address.starts_with("0x88888888")
            || address.starts_with("0x99999999")
            || address.starts_with("0xaaaaaaaa")
            || address.starts_with("0xbbbbbbbb")
            || address.starts_with("0xcccccccc")
            || address.starts_with("0xdddddddd")
            || address.starts_with("0xeeeeeeee")
            || address.starts_with("0xffffffff")
            || address.starts_with("0x314159")
            || address.starts_with("0x1234567") {
            println!("Found address {}. Saved private key to file", address);
            std::fs::write(address, secret_key.display_secret().to_string());
        }
        count+=1;
        if count%250000 == 0 {
            println!("[{}] Processed {} wallets.", id, count);
        }
    }
}
