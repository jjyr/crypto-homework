pub fn run() {
    let s1_1 = "290b6e3a 39155d6f";
    let s1_2 = "d6f491c5 b645c008";

    println!("case1");
    detect(s1_1, s1_2);

    let s2_1 = "5f67abaf 5210722b";
    let s2_2 = "bbe033c0 0bc9330e";

    println!("case2");
    detect(s2_1, s2_2);

    let s3_1 = "9d1a4f78 cb28d863";
    let s3_2 = "75e5e3ea 773ec3e6";

    println!("case3");
    detect(s3_1, s3_2);

    let s4_1 = "7b50baab 07640c3d";
    let s4_2 = "ac343a22 cea46d60";

    println!("case4");
    detect(s4_1, s4_2);
}

fn detect(s1: &str, s2: &str) {
    let p1: Vec<_> = s1.split(" ").collect();
    let p2: Vec<_> = s2.split(" ").collect();

    let l1 = hex::decode(p1[0]).unwrap();
    let r1 = hex::decode(p1[1]).unwrap();

    let l2 = hex::decode(p2[0]).unwrap();
    let r2 = hex::decode(p2[1]).unwrap();

    let r_result = xor(&r1, &r2);
    let l_result = xor(&l1, &l2);

    println!("L: {}", hex::encode(r_result));
    println!("R: {}", hex::encode(l_result));
}

fn xor(b1: &[u8], b2: &[u8]) -> Vec<u8> {
    b1.iter().zip(b2).map(|(l, r)| l ^ r).collect()
}
