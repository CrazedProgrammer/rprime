use std::thread;
use std::io;
use std::sync::{Arc, Mutex};

fn main () {
	let start_number: u64 = 2u64;
	let num_threads = 4;

	let data = Arc::new(Mutex::new(vec![start_number]));
	for i in 0..num_threads {
		let data = data.clone();
		let child = thread::spawn(move || {
			let threadid = i;
			
			loop {
				let number: u64;
				{
					let mut data = data.lock().unwrap();
					number = data[0];
					data[0] += 1;
				}

				let mut isprime = true;
				for j in 2u64..((number as f64).sqrt() as u64 + 2) {
					if number % j == 0 {
						isprime = false;
						break;
					}
				}
				if isprime {
					println!("Thread {} found prime number {}", threadid, number);
				}
			}
		});
	}

	let mut text: String = "".to_string();
	io::stdin().read_line(&mut text);
}