extern crate encoding;
extern crate base64;
extern crate crypto;

use threadpool::ThreadPool;

use std::ops::Add;
use encoding::Encoding;
use encoding::EncoderTrap;
use encoding::all::UTF_16LE;
use crypto::digest::Digest;
use crypto::sha2::Sha512;

fn main() {

    println!("1. ");
    let expected1 = "lw4vwffwk1PyCatLr/dRSvQnsYWbovd81V05EPQkPZGNnJfSHL6jHc+izkBKzCQdu3ydCOGUThKaiECC8X2P4w==";
    let salt1 = "px+fnZAl1LuN/O5A9Frbyw==";
    let pass1 = "kico";
    check(expected1, salt1, pass1);
    multi_check(expected1, salt1);

    // println!("2. ");

    // let expected2 = "ZudeQE349BZr76+cC6ahrW3aE3qRuGXshx5fQGcsJZwwnT/UmJ/dD8KAgHNWNaervWHUgxUuqnpGfarPmZLbjw==";
    // let salt2 =  "DgV8Sq/t9Enui/mvABSBQQ==";
    // let pass2 = "kico";
    // check(expected2, salt2, pass2);

    // println!("3. ");

    // let expected3 = "x4q/XWKVegiNVCwgA1oepOzTMICCgRj/q6Akik873zP6ljPdvlRyNPbHv0jxxLHrhTBv1Nl4+I8PQ01CwapgJA==";
    // let salt3 =  "S5DcDfj4deWv1ABTpY1aGQ==";
    // let pass3 = "kico";
    // check(expected3, salt3, pass3);

}

fn vectorize(possibilities: &str) -> Vec<&str> {
    let mut letters: Vec<_> = possibilities.split("").collect();
    letters.pop();
    letters.remove(0);
    letters
}

fn multi_check(expected: &'static str, salt: &'static str) {

    let pool = ThreadPool::new(12);

    println!("mutli check. ");

    let first = vectorize("kuiojlm,.");
    let sec = vectorize("i789uojkl");
    let third = vectorize("csdfxv");
    let fourth = vectorize("o890ipklé");

    for a in &first {
    for b in &sec {
    for c in &third {
    for d in &fourth {

        let mut pass = String::new();
        pass.push_str(a);
        pass.push_str(b);
        pass.push_str(c);
        pass.push_str(d);

        pool.execute(move|| {
            let succeed = check(expected, salt, pass.as_ref());
            if succeed {
                println!("Pass is : {}", pass);
                std::process::exit(0x0100);
            }
        });

        // let succeed = check(expected, salt, pass.as_ref());

        // if succeed {
        //     return;
        // }
    }}}}

    pool.join();
}


fn check(expected: &str, salt: &str, pass: &str) -> bool {

    let salt =  base64::decode(salt).unwrap();
    let pass_utf16 = UTF_16LE.encode(pass, EncoderTrap::Strict).unwrap();
    let mut algo = Sha512::new();
    let spincount = 100000;

    let n0 = hash(&mut algo, salt, pass_utf16);
    let n100000 = hash_spin(&mut algo, n0, spincount);

    let encoded = base64::encode(&n100000);
    println!("pass : {}", pass);
    println!("result : {}", encoded);
    println!("expect : {}", expected);
    
    encoded == expected
}

fn hash(algo: &mut Sha512, salt:  Vec<u8>, pass: Vec<u8>) -> Vec<u8> {

    let mut data: Vec<u8> = Vec::new();
    data.extend(salt.iter());
    data.extend(pass.iter());

    let mut result: Vec<u8> = vec![0u8; algo.output_bytes()];
    algo.input(data.as_ref());
    algo.result(&mut result);
    algo.reset();
    let encoded = base64::encode(&result);

    result
}

fn hash_spin(algo: &mut Sha512, n0: Vec<u8>, spin_count: u32) -> Vec<u8>{

    let mut i = 0;
    let mut n = n0.clone();
    while i < spin_count {

        let mut result: Vec<u8> = vec![0u8; algo.output_bytes()];

        n.extend(i.to_le_bytes().iter());
        algo.input(n.as_ref());
        algo.result(&mut result);
        algo.reset();

        n = result;
        i = i+1;
    }
    n
}