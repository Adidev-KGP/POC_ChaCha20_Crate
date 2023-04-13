use super::block::chacha20_block;

pub struct ChaCha20 {
    key: [u8; 32],
    nonce: [u8; 12],
    counter: u32,
}

impl ChaCha20 {
    pub fn new(key: &[u8; 32], nonce: &[u8; 12]) -> Self {
        Self {
            key: *key,
            nonce: *nonce,
            counter: 0,
        }
    }

    pub fn set_counter(&mut self, counter: u32) {
        self.counter = counter;
    }

    pub fn encrypt(&mut self, plaintext: &mut [u8]) {
        let mut block = [0u8; 64];
        for chunk in plaintext.chunks_mut(64) {
            self.next_block(&mut block);
            for (i, byte) in chunk.iter_mut().enumerate() {
                *byte ^= block[i];
            }
        }
    }

    pub fn decrypt(&mut self, ciphertext: &mut [u8]) {
        self.encrypt(ciphertext);
    }

    pub fn next_block(&mut self, block: &mut [u8]) {
        chacha20_block(&self.key, &self.nonce, self.counter, block.as_mut().try_into().unwrap());
        self.counter = self.counter.checked_add(1).unwrap();
        if self.counter == 0 {
            panic!("ChaCha20 counter wrapped around");
        }
    }
}
