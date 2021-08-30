use aes::cipher::generic_array::GenericArray;
use aes::cipher::{FromBlockCipher, StreamCipher};
use aes::{Aes128, Aes128Ctr, NewBlockCipher};

const BLOCK_SIZE: usize = 16;
type Bytes = Vec<u8>;

pub fn decode(pk: Bytes, cipher_text: Bytes) -> Bytes {
    // fetch IV
    let mut chunks_iter = cipher_text.chunks(BLOCK_SIZE);
    let iv = chunks_iter.next().unwrap();

    // setup AES
    let pk = GenericArray::from_slice(&pk);
    let aes = Aes128::new(&pk);
    let iv = GenericArray::from_slice(iv);
    let mut aes = Aes128Ctr::from_block_cipher(aes, iv);
    let mut cipher_text = cipher_text[BLOCK_SIZE..].to_vec();
    aes.apply_keystream(&mut cipher_text);
    cipher_text
}
