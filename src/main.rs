use rand::rngs::OsRng;
use secp256k1::{All, Secp256k1};
use sha3::{Digest, Keccak256};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

static TOTAL_PROCESSED: AtomicU64 = AtomicU64::new(0);
static FOUND_COUNT: AtomicU64 = AtomicU64::new(0);

fn main() {
    let num_threads = num_cpus::get();
    println!("Starting wallet grinder with {num_threads} threads");
    println!("Looking for addresses starting with ten consecutive 8s after 0x");

    let start_time = Arc::new(Instant::now());
    let mut handles = vec![];

    for i in 0..num_threads {
        let start_time_clone = start_time.clone();
        let handle = std::thread::spawn(move || {
            thread_logic(i as u8, start_time_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn thread_logic(id: u8, start_time: Arc<Instant>) {
    let secp = Secp256k1::<All>::new();
    let mut rng = OsRng;
    let mut local_count: u64 = 0;
    let mut buffer = [0u8; 65];
    let mut address_bytes = [0u8; 20];

    loop {
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);

        buffer[0] = 0x04;
        buffer[1..65].copy_from_slice(&public_key.serialize_uncompressed()[1..]);

        let mut hasher = Keccak256::new();
        hasher.update(&buffer[1..]);
        let hash = hasher.finalize();
        address_bytes.copy_from_slice(&hash[12..32]);

        if is_good_optimized(&address_bytes) {
            let address_hex = hex::encode(address_bytes);
            let found_count = FOUND_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
            println!("\n🎯 FOUND #{found_count}: 0x{address_hex}");
            println!("   Private key: {}", secret_key.display_secret());

            let filename = format!("0x{address_hex}.key");
            std::fs::write(&filename, secret_key.display_secret().to_string()).unwrap();
            println!("   Saved to: {filename}");
        }

        local_count += 1;

        if local_count.is_multiple_of(100_000) {
            TOTAL_PROCESSED.fetch_add(100_000, Ordering::Relaxed);

            if id == 0 {
                let total = TOTAL_PROCESSED.load(Ordering::Relaxed);
                let elapsed = start_time.elapsed();
                let rate = total as f64 / elapsed.as_secs_f64();
                let found = FOUND_COUNT.load(Ordering::Relaxed);
                println!(
                    "Progress: {:.2}M wallets | Speed: {:.2}K/s | Found: {}",
                    total as f64 / 1_000_000.0,
                    rate / 1_000.0,
                    found
                );
            }
        }
    }
}

#[inline(always)]
fn is_good_optimized(address_bytes: &[u8; 20]) -> bool {
    address_bytes[0] == 0x88
        && address_bytes[1] == 0x88
        && address_bytes[2] == 0x88
        && address_bytes[3] == 0x88
        && address_bytes[4] == 0x88
}
