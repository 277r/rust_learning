use std::env;



// log2(u128) is always smaller than 8 bits, because the largest log2 is 128, which would fit in 7 bytes
fn log2 (mut input : u128) -> u8 {
	let mut x:u8 = 0;

	while input > 1 {
		input >>= 1;
		x += 1;
	}



	return x;
}

fn main(){
	let args: Vec<String> = env::args().collect();
	let mut numArg:u128 ;
	numArg = 0;
	
	if (args.len()) > 1 {
		numArg = args[1].parse::<u128>().unwrap();
	}

	print!("{0}", log2(numArg));

}