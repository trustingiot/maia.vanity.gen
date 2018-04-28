#![feature(alloc)]
#![feature(const_fn)]
#![feature(conservative_impl_trait)]

extern crate rand;

extern crate iota_trytes as trytes;
extern crate iota_merkle as merkle;
extern crate iota_curl_cpu as curl_cpu;

use rand::Rng;
use trytes::*;
use curl_cpu::*;
use merkle::*;
use std::time::Instant;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	let mut rng = rand::thread_rng();
	let chars = vec!['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z','9'];

	let mut f = File::open("prefixes.txt").expect("file not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents).expect("wrong file");
	let prefixes : Vec<&str> = contents.split("\n").filter(|&x| !x.is_empty()).collect();

	let mut seed: String;
	let mut address;
	let mut root_node;
	let mut hash: [Trit; HASH_LENGTH] = [0; HASH_LENGTH];
	let mut c = 0;

	let mut c1 = CpuCurl::<Trit>::default();
	let mut c2 = CpuCurl::<Trit>::default();
	let mut c3 = CpuCurl::<Trit>::default();
	let mut seedTrit: Vec<Trit>;

	let start: isize = 0;
	let index: usize = 1;
	let security: usize = 2;
	let init = Instant::now();

	loop {
		seed = (0..81).map(|_| rng.gen_range(0, 27)).map(|x| &chars[x]).collect();
		seedTrit = seed.chars().flat_map(char_to_trits).cloned().collect();
		root_node = create(&seedTrit, start, index, security, &mut c1, &mut c2, &mut c3);
		slice(&root_node, &mut hash);
		address = trits_to_string(&hash).unwrap();
		for prefix in &prefixes {
			if address.starts_with(prefix) {
				println!("Prefix: {}\nSeed: {}\nAddress: {}", prefix, seed, address);
			}
		}
		c += 1;
		if c % 5000 == 0 {
			println!("c = {}, seconds = {}", c, init.elapsed().as_secs());
		}
	}
}
