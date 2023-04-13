// use super::chacha20::{ChaCha20, chacha20_block};
use std::convert::TryInto;
extern crate chacha20;
extern crate hex;

use chacha20::{{chacha20_algo::ChaCha20}, {block::chacha20_block}};
// use hex::{FromHex, ToHex};
use hex::FromHexError;

#[test]
fn test_chacha20_block() {
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
    let mut block = [0u8; 64];
    chacha20.next_block(&mut block);
    println!("<<<<<<<<<<>>>>>>>>>>>{:?}",block);
    let vec : Vec<u8> = get_bytes("401768d9a15a51c8b1f2965fd5fd512b5b098c2f68e5c6fcd7d35b24a78c8dbf03d02b1187d30e71204c09ee8e1ebd9a7bd1d54dc257b8a3fb017d15b7a398e1").unwrap();
    assert_ne!(block, vec[..]);

}

fn get_bytes(hex_string: &str) -> Result<Vec<u8>, FromHexError> {
    let result_bytes = hex::decode(hex_string)?;
    Ok(result_bytes)
}