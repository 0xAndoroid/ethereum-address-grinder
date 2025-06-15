use secp256k1::generate_keypair;
use sha3::{Digest, Keccak256};

fn main() {
    for i in 0..10 {
        //The amount of threads to use
        let handle = std::thread::spawn(move || thread_logic(i));
        if i == 7 {
            handle.join().unwrap();
        }
    }
}

fn thread_logic(id: u8) {
    let mut count: u64 = 0;
    let time = std::time::Instant::now();
    loop {
        let (secret_key, public_key) = generate_keypair(&mut rand::thread_rng());
        let mut pubkey_vec = public_key.serialize_uncompressed().to_vec();
        pubkey_vec.remove(0);
        let mut hasher = <Keccak256 as Digest>::new();
        hasher.update(pubkey_vec);
        let address = hex::encode(&hasher.finalize().to_vec()[12..32]);
        //Address matching
        if is_good(&address) {
            println!("Found address {address}. Saved private key to file");
            std::fs::write(address, secret_key.display_secret().to_string()).unwrap();
        }
        count += 1;
        if count % 1250000 == 0 && id == 0 {
            let elapsed = time.elapsed();
            println!(
                "Processed {} M wallets, doing {:.2}K wallets per second",
                count * 8 / 1000000,
                count as f64 * 8f64 / 1000f64 / (elapsed.as_secs() as f64)
            );
        }
    }
}

fn is_good(address: &str) -> bool {
    address
        .chars()
        .take(4)
        .chain(address.chars().skip(40 - 4))
        .collect::<Vec<char>>()
        .windows(2)
        .all(|a| a[0] == a[1] && a[0].is_numeric())
}
