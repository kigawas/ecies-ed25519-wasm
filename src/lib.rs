use ecies_ed25519::{
    decrypt as _decrypt, encrypt as _encrypt, generate_keypair as _generate_keypair, PublicKey,
    SecretKey,
};
use js_sys::{Array, Uint8Array};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_keypair() -> Array {
    let mut rng = rand::thread_rng();

    let (sk, pk) = _generate_keypair(&mut rng);
    let (sk, pk) = (sk.to_bytes(), pk.to_bytes());
    let (sk, pk) = (Uint8Array::from(&sk[..]), Uint8Array::from(&pk[..]));

    let ret = Array::new();
    ret.push(&sk);
    ret.push(&pk);
    ret
}

#[wasm_bindgen]
pub fn encrypt(receiver_pub: &[u8], msg: &[u8]) -> Option<Uint8Array> {
    let pk = PublicKey::from_bytes(receiver_pub).ok();
    if pk.is_none() {
        return None;
    }
    let mut rng = rand::thread_rng();
    _encrypt(&pk.unwrap(), msg, &mut rng)
        .map(|v| Uint8Array::from(v.as_slice()))
        .ok()
}

#[wasm_bindgen]
pub fn decrypt(receiver_sec: &[u8], msg: &[u8]) -> Option<Uint8Array> {
    let sk = SecretKey::from_bytes(receiver_sec).ok();
    if sk.is_none() {
        return None;
    }
    _decrypt(&sk.unwrap(), msg)
        .map(|v| Uint8Array::from(v.as_slice()))
        .ok()
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use rand::thread_rng;
    use wasm_bindgen_test::*;

    use super::*;

    #[wasm_bindgen_test]
    fn test_rust() {
        let (peer_sk, peer_pk) = _generate_keypair(&mut thread_rng());

        let plaintext = b"ABOLISH ICE";

        let encrypted = _encrypt(&peer_pk, plaintext, &mut thread_rng()).unwrap();
        let decrypted = _decrypt(&peer_sk, &encrypted).unwrap();

        assert_eq!(plaintext, decrypted.as_slice());

        // Test bad ciphertext
        assert!(_decrypt(&peer_sk, &[0u8; 16]).is_err());

        // Test that it fails when using a bad secret key
        let bad_secret = SecretKey::generate(&mut thread_rng());
        assert!(_decrypt(&bad_secret, &encrypted).is_err());
    }

    #[wasm_bindgen_test]
    fn test_wasm() {
        let data = "😀😀😀😀".as_bytes();
        let data_js = Uint8Array::from(&data[..]);

        let pair = generate_keypair();
        let (sk, pk) = (Uint8Array::from(pair.get(0)), Uint8Array::from(pair.get(1)));

        let encrypted = encrypt(&pk.to_vec(), data).unwrap();
        let decrypted = decrypt(&sk.to_vec(), &encrypted.to_vec()).unwrap();
        assert_eq!(data_js.to_vec(), decrypted.to_vec());
    }
}
