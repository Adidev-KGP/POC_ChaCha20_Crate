// use super::chacha20::{ChaCha20, chacha20_block};
extern crate chacha20;
extern crate hex;

use chacha20::{{chacha20_algo::ChaCha20}, {block::chacha20_block}};
// use hex::{FromHex, ToHex};
use hex::FromHexError;

#[test]
fn test_chacha20() {
    let key = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    ];
    let nonce = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4a,
        0x00, 0x00, 0x00, 0x00,
    ];
    let plaintext = b"Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.";
    let mut chacha20 = ChaCha20::new(&key, &nonce);
    chacha20.set_counter(1);
    let mut ciphertext = plaintext.to_vec();
    chacha20.encrypt(&mut ciphertext);
    // println!("<<<<<<<<<<>>>>>>>>>>>{:?}",ciphertext);
    assert_eq!(ciphertext, hex::decode("6e2e359a2568f98041ba0728dd0d6981e97e7aec1d4360c20a27afccfd9fae0bf91b65c5524733ab8f593dabcd62b3571639d624e65152ab8f530c359f0861d807ca0dbf500d6a6156a38e088a22b65e52bc514d16ccf806818ce91ab77937365af90bbf74a35be6b40b8eedf2785e42874d").unwrap());
}

fn get_bytes(hex_string: &str) -> Result<Vec<u8>, FromHexError> {
    let result_bytes = hex::decode(hex_string)?;
    Ok(result_bytes)
}