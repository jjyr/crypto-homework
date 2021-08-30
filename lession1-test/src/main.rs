use std::{cmp::min, collections::HashMap, io::Read};

use itertools::Itertools;

use crate::cipher_text::{CIPHER_TEXTS, TARGET_CIPHER_TEXT};

mod cipher_text;

const SPACE: u8 = 032;
const ASC_0: u8 = 048;
const ASC_A: u8 = 097;
const ASC_UA: u8 = 065;

fn build_dict() -> HashMap<u8, u8> {
    let mut precomputed: HashMap<u8, u8> = HashMap::default();

    precomputed.insert(SPACE ^ SPACE, SPACE);
    for c in ASC_0..(ASC_0 + 10) {
        precomputed.insert(c ^ SPACE, c);
    }
    for c in ASC_A..(ASC_A + 26) {
        precomputed.insert(c ^ SPACE, c);
    }
    for c in ASC_UA..(ASC_UA + 26) {
        precomputed.insert(c ^ SPACE, c);
    }
    precomputed
}

fn decode_cipher(target_cipher: &[u8], pk: &[u8]) -> String {
    // decode target
    let mut clear_text = Vec::default();
    clear_text.resize_with(target_cipher.len(), || 0u8);
    for (i, (c, p)) in target_cipher
        .iter()
        .zip(pk.iter())
        .enumerate()
        .take(pk.len())
    {
        clear_text[i] = c ^ p;
    }

    String::from_utf8(clear_text).unwrap()
}

fn main() {
    let msg =
        b"The secret message is: when using a stream cipher, never use the key more than once";
    let target_cipher = hex::decode(&TARGET_CIPHER_TEXT).unwrap();
    let mut pk: Vec<u8> = Vec::default();
    pk.resize_with(target_cipher.len(), || 0u8);
    for (i, (a, b)) in target_cipher.iter().zip(msg).enumerate() {
        pk[i] = a ^ b;
    }

    // break pk
    let ciphers: Vec<Vec<u8>> = CIPHER_TEXTS
        .iter()
        .map(|c| hex::decode(c).unwrap())
        .collect();
    for cipher in ciphers.iter() {
        for i in 0..min(cipher.len(), pk.len()) {
            let c = cipher[i] ^ pk[i];
            if c.is_ascii() {
                print!("{}", c as char);
            } else {
                print!("*");
            }
        }
        println!();
    }
    // **EC**C***T**S***EN**XE*HT******GQ*A****A*O********N2*E*A*S*L**EF***O*O**ET***B**CT
    // ***E*E****DM******L*S*N***NT***N*O*****E**E****E*IC****RAU**R*******N*O*******T*NNE
    // ********E*H***S***U*SqE2***QU**N*O****R***P******DE**V**NU**I**EAK**TM**EF*********
    // **********T***S***D***Dw**NAU******N******O*I*****I***E*O******EG******R*I****T***E
    // *******U*TW***S**EB***Aw******I**RAK***E**O*I*H**U****E*P***A***E*E*NM***A*********
    // ***R*E***TT**S****SI***4***T*****H***T**********R*I**V**E*S*E***T*E*A**R*R**A*O**C*
    // ***R*E***TT**S****SI***4***O*****Y*****E**A*I*****SN***Rg***R***N*E*OM********EO***
    // **EC**C*******S***N*SMH2**NT**I**I****R***A***H***AN****GU**TT******TM********U***E
    // *HMP**********ZAG*N**CP**********EAS*****M*C*****ET***IRN***L*H*****C****ET********
    // t**ES*****S*E*****D**YT****R*SA*W*W*S*****N**P****T*E**RT**EA**EO*EYW****N*H*NRO**
    // Thersecuettmessagenisme*htntusingiasstreamacipheretnevernuseathe*key*more*than*once
    // The secure message is  when using a stream cipher  never use the key more than once
}
