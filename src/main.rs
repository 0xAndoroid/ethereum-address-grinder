use rand::thread_rng;
use secp256k1::generate_keypair;
use sha3::{Digest, Keccak256};

fn main() {
    for i in 0..8 {
        //The amount of threads to use
        let handle = std::thread::spawn(move || thread_logic(i));
        if i == 7 {
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
        let mut hasher = <Keccak256 as Digest>::new();
        hasher.update(pubkey_vec);
        let mut address = String::from("0x");
        address.push_str(&hex::encode(&hasher.finalize().to_vec()[12..32]));
        //Address matching
        if address.starts_with("0x00000000")
            | address.starts_with("0xaaaaaaaa") {
            println!("Found address {}. Saved private key to file", address);
            std::fs::write(address, secret_key.display_secret().to_string()).unwrap();
        }
        count += 1;
        if count % 1250000 == 0 && id == 0 {
            println!("Processed {} M wallets.", count * 8 / 1000000);
        }
    }
}
