extern crate crypto;
extern crate time;

use crypto::digest::Digest;
use time::PreciseTime;

pub fn print(raw : &[u8]){
    for idx in 0..raw.len() {
        print!("{0:02x}", raw[idx]);
    }
    println!("");
}

pub fn cycle(dig : &mut Digest, mut buffer : &mut [u8]) {
    dig.reset();
    dig.input(&buffer);
    dig.result(&mut buffer);
}

pub fn do_1_kk_cycles(dig : &mut Digest, mut buffer : &mut [u8]) {
    for _ in 0..1000000 {
        cycle(dig, buffer);
    }
}


fn main() {

//    let mut dig = crypto::md5::Md5::new();        // 8.726555294S seconds.
    let mut dig = crypto::sha1::Sha1::new();      // 9.015671765S seconds.
//    let mut dig = crypto::sha2::Sha224::new();      // 14.591875648S seconds.
//    let mut dig = crypto::sha2::Sha256::new();      // 14.649101622S seconds.
//    let mut dig = crypto::sha2::Sha384::new();      // 15.633920375S seconds.
//    let mut dig = crypto::sha2::Sha512::new();      // 15.812791771S seconds.

    dig.input(b"Hello, world!");
    let len: usize = dig.output_bits() / 8;
    let mut data = vec![0;len];
    let mut raw = data.as_mut_slice();
    dig.result(&mut raw);
    print(&raw);

    let start = PreciseTime::now();
    do_1_kk_cycles(&mut dig, raw);
    let end = PreciseTime::now();
    print(&raw);
    println!("Spent {} seconds.", start.to(end));

}
