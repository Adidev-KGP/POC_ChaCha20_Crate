// use super::block::chacha20_block;

const BLOCK_SIZE: usize = 64;
const ROTATION_CONSTANTS: [u32; 4] = [16, 12, 8, 7];


fn quarter_round(x: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    x[a] = x[a].wrapping_add(x[b]);
    x[d] = (x[d] ^ x[a]).rotate_left(16);
    x[c] = x[c].wrapping_add(x[d]);
    x[b] = (x[b] ^ x[c]).rotate_left(12);
    x[a] = x[a].wrapping_add(x[b]);
    x[d] = (x[d] ^ x[a]).rotate_left(8);
    x[c] = x[c].wrapping_add(x[d]);
    x[b] = (x[b] ^ x[c]).rotate_left(7);
}

pub fn chacha20_block(key: &[u8; 32], nonce: &[u8; 12], counter: u32, output: &mut [u8; 64]) {
    let mut state = [0u32; 16];
    let mut input = [0u8; 64];
    state[0] = 0x61707865; 
    state[1] = 0x3320646e; 
    state[2] = 0x79622d32; 
    state[3] = 0x6b206574; 
    for i in 0..8 {
        state[4 + i] = u32::from_le_bytes(key[4 * i..4 * i + 4].try_into().unwrap());
    }
    state[12] = counter;
    for i in 0..3 {
        state[13 + i] = u32::from_le_bytes(nonce[4 * i..4 * i + 4].try_into().unwrap());
    }
    for i in 0..16 {
        input[4 * i..4 * i + 4].copy_from_slice(&state[i].to_le_bytes());
    }
    let mut working_state = state;
    for i in 0..10 {
        quarter_round(&mut working_state, 0, 4, 8, 12);
        quarter_round(&mut working_state, 1, 5, 9, 13);
        quarter_round(&mut working_state, 2, 6, 10, 14);
        quarter_round(&mut working_state, 3, 7, 11, 15);
        quarter_round(&mut working_state, 0, 5, 10, 15);
        quarter_round(&mut working_state, 1, 6, 11, 12);
        quarter_round(&mut working_state, 2, 7, 8, 13);
        quarter_round(&mut working_state, 3, 4, 9, 14);
    }
   
    for i in 0..16 {
        working_state[i] = working_state[i].wrapping_add(state[i]);
    }
    for i in 0..16 {
        output[4 * i..4 * i + 4].copy_from_slice(&working_state[i].to_le_bytes());
    }
}

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
