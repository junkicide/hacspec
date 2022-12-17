use hacspec_lib::*;

const BLOCKSIZE: usize = 64;
const ROUNDS: usize = 16;
const P_ARRAY_SIZE: usize = 18;

const KEY_LENGTH: usize = 256;
pub const ITERATIONS: usize = 521;
const INPUT_SIZE: usize = 

array!(State, 16, u64)
bytes!(Block, BLOCKSIZE);
bytes!(Word, KEY_LENGTH);
array!(PArray, P_ARRAY_SIZE, u64);

array!(SBox1, 256, u32)
array!(SBox2, 256, u32)
array!(SBox3, 256, u32)
array!(SBox4, 256, u32)

// subkeys are the digits of pi in hex
#[rustfmt::skip]
const INITKEYS: PArray = PArray([
     0x243f6a88u64, 0x85a308d3u64, 0x13198a2eu64, 0x03707344u64, 0xa4093822u64, 0x299f31d0u64, 0x082efa98u64, 0xec4e6c89u64, 452821e6u64, 0x38d01377u64, 0xbe5466cfu64, 0x34e90c6cu64, 0xc0ac29b7u64, 0xc97c50ddu64, 0x3f84d5b5u64, 0xb5470917u64, 9216d5d9u64, 0x8979fb1bu64 ]);

fn generate_subkeys(input: State) -> PArray {
    let mut st = INITKEYS;
    for i in 0..P_ARRAY_SIZE {
	if i>State {
	    let j = i % 16;
	}
	st[i] = INITKEYS[i] & input[j];
    }
    st
}

fn 
    














