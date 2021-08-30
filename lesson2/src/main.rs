fn main() {
    let hex1 = "4ca00ff4c898d61e1edbf1800618fb2828a226d160dad07883d04e008a7897ee2e4b7465d5290d0c0e6c6822236e1daafb94ffe0c5da05d9476be028ad7c1d81";
    let hex2 = "5b68629feb8606f9a6667670b75b38a5b4832d0f26e1ab7da33249de7d4afc48e713ac646ace36e872ad5fb8a512428a6e21364b0c374df45503473c5242a253";
    let cbc_key = "140b41b22a29beb4061bda66b6747e14";
    let hex3 = "69dda8455c7dd4254bf353b773304eec0ec7702330098ce7f7520d1cbbb20fc388d1b0adb5054dbd7370849dbf0b88d393f252e764f1f5f7ad97ef79d59ce29f5f51eeca32eabedd9afa9329";
    let hex4 = "770b80259ec33beb2561358a9f2dc617e46218c0a53cbeca695ae45faa8952aa0e311bde9d4e01726d3184c34451";
    let ctr_key = "36f18357be4dbd77f050515c73fcf9f2";

    let cbc_pk = hex::decode(cbc_key).unwrap();
    let c1 = hex::decode(hex1).unwrap();
    let c2 = hex::decode(hex2).unwrap();
    let msg = lesson2::cbc::decode(cbc_pk.clone(), c1);
    println!("case1: {}", String::from_utf8(msg).unwrap());
    let msg = lesson2::cbc::decode(cbc_pk, c2);
    println!("case2: {}", String::from_utf8(msg).unwrap());

    let ctr_pk = hex::decode(ctr_key).unwrap();
    let c3 = hex::decode(hex3).unwrap();
    let msg = lesson2::ctr::decode(ctr_pk.clone(), c3);
    println!("case3: {}", String::from_utf8(msg).unwrap());
    let c4 = hex::decode(hex4).unwrap();
    let msg = lesson2::ctr::decode(ctr_pk.clone(), c4);
    println!("case4: {}", String::from_utf8(msg).unwrap());
}
