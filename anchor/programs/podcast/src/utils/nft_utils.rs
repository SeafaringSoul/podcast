use ring::hmac::Hmac;
use ring::rand::SystemRandom;
use ring::sha::Sha256;

fn generate_nft_identifier() -> String {
    let mut rng = SystemRandom::new().unwrap();
    let mut key = [0; 32];
    rng.fill(&mut key).unwrap();
    let hmac = Hmac::new_with_key(ring::hmac::SHA256, &key).unwrap();

    let mut data = [0; 32];
    rng.fill(&mut data).unwrap();
    hmac.update(&data);
    let result = hmac.finish();

    let hash = result.as_ref();
    let hex_str: String = hash.iter().map(|b| format!("{:02x}", b)).collect();

    hex_str
}
