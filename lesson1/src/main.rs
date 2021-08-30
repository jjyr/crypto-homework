fn main() {
    let m = b"attack at dawn";
    let c = "6c73d5240a948c86981bc294814d";
    let c = hex::decode(&c).unwrap();
    assert_eq!(c.len(), 14);

    // decode k
    let mut k = [0u8; 14];
    for i in 0..k.len() {
        k[i] = m[i] ^ c[i];
    }

    // encode m2
    let m2 = b"attack at dusk";
    let mut c2 = [0u8; 14];
    for i in 0..m2.len() {
        c2[i] = k[i] ^ m2[i];
    }
    let cs = hex::encode(&c2);
    println!("{}", cs);
}
