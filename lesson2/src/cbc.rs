use aes::cipher::generic_array::GenericArray;
use aes::cipher::BlockDecrypt;
use aes::{Aes128, Block, BlockEncrypt, NewBlockCipher};
use rand::Rng;

const BLOCK_SIZE: usize = 16;
type Bytes = Vec<u8>;

pub fn decode(pk: Bytes, cipher_text: Bytes) -> Bytes {
    // fetch IV
    let mut chunks_iter = cipher_text.chunks(BLOCK_SIZE);
    let iv = chunks_iter.next().unwrap();
    let mut msg = Bytes::default();

    // setup AES
    let pk = GenericArray::from_slice(&pk);
    let aes = Aes128::new(&pk);

    // decode first block
    let mut last_cipher = chunks_iter.next().unwrap();
    let mut block = Block::default();
    block.copy_from_slice(last_cipher);
    aes.decrypt_block(&mut block);
    msg.extend(xor(&block, &iv));

    // decode remain block
    for cipher_block in chunks_iter {
        block.copy_from_slice(cipher_block);
        aes.decrypt_block(&mut block);
        msg.extend(xor(&block, &last_cipher));
        last_cipher = cipher_block.clone();
    }
    remove_dummy(msg)
}

pub fn encode(pk: Bytes, msg: Bytes) -> Bytes {
    // random chose IV
    let mut rng = rand::thread_rng();
    let mut iv = [0u8; BLOCK_SIZE];
    rng.fill(&mut iv);
    let mut cipher_text = Bytes::default();

    // setup AES
    let pk = GenericArray::from_slice(&pk);
    let aes = Aes128::new(&pk);
    // encode first block
    let mut chunks_iter = msg.chunks(BLOCK_SIZE);
    let first = chunks_iter.next().unwrap();
    // mix content
    let mut block = Block::default();
    block.copy_from_slice(&xor(&iv, first));
    // encrypt
    aes.encrypt_block(&mut block);
    cipher_text.extend_from_slice(block.as_slice());

    // encode others
    for msg in chunks_iter {
        let msg = xor(block.as_slice(), msg);
        block.copy_from_slice(&msg);
        cipher_text.extend_from_slice(block.as_slice());
    }

    append_dummy(cipher_text)
}

fn xor(b1: &[u8], b2: &[u8]) -> Vec<u8> {
    debug_assert_eq!(b1.len(), b2.len());
    b1.iter().zip(b2).map(|(l, r)| l ^ r).collect()
}

fn append_dummy(mut b: Bytes) -> Bytes {
    let r = b.len() % BLOCK_SIZE;
    if r == 0 {
        return b;
    }
    let padding_len = BLOCK_SIZE - r + BLOCK_SIZE;
    b.resize_with(b.len() + padding_len, || padding_len as u8);
    b
}

fn remove_dummy(mut b: Bytes) -> Bytes {
    let dummy = &b[(b.len() - BLOCK_SIZE)..];
    let is_dummy = dummy.iter().all(|n| n == &dummy[0]);
    if !is_dummy {
        return b;
    }
    b.truncate(b.len() - dummy[0] as usize);
    b
}
