// Keccakf permutation based on tinySHA3 c implementation
const KECCAKF_RNDC: [u64;24] = [0x0000000000000001, 0x0000000000008082, 0x800000000000808a,
0x8000000080008000, 0x000000000000808b, 0x0000000080000001,
0x8000000080008081, 0x8000000000008009, 0x000000000000008a,
0x0000000000000088, 0x0000000080008009, 0x000000008000000a,
0x000000008000808b, 0x800000000000008b, 0x8000000000008089,
0x8000000000008003, 0x8000000000008002, 0x8000000000000080,
0x000000000000800a, 0x800000008000000a, 0x8000000080008081,
0x8000000000008080, 0x0000000080000001, 0x8000000080008008];

const KECCAKF_ROTC: [u32; 24] = [1,  3,  6,  10, 15, 21, 28, 36, 45, 55, 2,  14,
    27, 41, 56, 8,  25, 43, 62, 18, 39, 61, 20, 44];
const KECCAKF_PILN: [usize;24] =[10, 7,  11, 17, 18, 3, 5,  16, 8,  21, 24, 4,
    15, 23, 19, 13, 12, 2, 20, 14, 22, 9,  6,  1];

const KECCAKF_ROUNDS:usize =24;

fn sha3_keccakf(input: &mut[u8]) {
    let mut j: usize;
    let mut t: u64;
    let mut bc: [u64; 5] = [0; 5]; 
    let mut st: [u64; 25] = [0; 25]; // Keccak State

    // Load bytes into 64-bit lanes (little-endian)
    for (k, chunk) in input.chunks_exact(8).take(25).enumerate() {
        st[k] = u64::from_le_bytes(chunk.try_into().unwrap());
    }
    // Actual Iteration
    for round in 0..KECCAKF_ROUNDS {
        // Theta
        for i in 0..5 {
            bc[i] = st[i] ^ st[i + 5] ^ st[i + 10] ^ st[i + 15] ^ st[i + 20];
        }
        for i in 0..5 {
            t = bc[(i + 4) % 5] ^ bc[(i + 1) % 5].rotate_left(1);
            for j in (0..25).step_by(5) {
                st[j + i] ^= t;
            }
        }
        
        // Rho Pi
        t=st[1];
        for i in  0..24 {
            j = KECCAKF_PILN[i];
            bc[0] =st[j];
            st[j] = t.rotate_left(KECCAKF_ROTC[i]);
            t=bc[0];
        }
        
        //Chi
        for j in (0..25).step_by(5) {
            for i in 0..5 {
                bc[i]= st[j + i];
            }
            for i in 0..5 {
                st[j + i] ^= (!bc[(i + 1) % 5]) & bc[(i + 2) % 5];
            }
        }
        
        //Iota
        st[0] ^= KECCAKF_RNDC[round as usize];
        
    }

    for (k, &lane) in st.iter().enumerate() {
        let bytes = lane.to_le_bytes();
        input[k*8..(k+1)*8].copy_from_slice(&bytes);
    }

    // Sponge Construction
    const RATE:usize = 136;
    pub struct Shake{
        b: [u8;200],
        pos: usize
    }
    impl Shake {
        
        pub fn new(input:&[u8]) -> Self {
            let mut shake = Shake {
                b: [0u8; 200],
                pos: 0,        
            };
            for i in 0..200.min(input.len()) {
                shake.b[i] = input[i];
            }
            shake
        }
        pub fn absorb(&mut self, input:&[u8]) {
            let mut x = input;
            let padded:Vec<u8>;
            if x.len()  % RATE !=0 {
                fn pad_ten_one(val:&[u8]) -> Vec<u8> {
                    let q:usize  =RATE - (val.len() % RATE);
                    let new_len =val.len() + q;
                    let mut padded = Vec::with_capacity(new_len);
                    padded.extend_from_slice(val);
                    padded.resize(new_len, 0);
                    padded[new_len - 1] = 0x80;
                    padded
                }
                padded = pad_ten_one(x);
                x = padded.as_slice();
            }
            
            let mut i = self.pos;
                for &value in x {
                    self.b[i] ^= value;
                    i = i + 1;
                    if i>=RATE {
                        sha3_keccakf(&mut self.b);
                        i=0;
                    }
            }
            self.pos = i;
        }
    }
}
