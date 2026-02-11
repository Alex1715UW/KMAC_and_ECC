use crate::shake::Shake;

const RIGHT_ENCODE_0:[u8;2] = [0x0,0x1];
pub fn kmacxof256(k:&[u8], x:&[u8], l:usize, s:&[u8]) ->Vec<u8>{
    assert_eq!(l % 8, 0, "Implementation restriction: output length (in bits) must be a multiple of 8");
    let mut shake = Shake::new(&[]);
    
}

fn encode_string(s:&[u8]) ->Vec<u8>{
    let length_encode = left_encode((s.len() * 8) as u64);
    let mut output:Vec<u8> = Vec::with_capacity(length_encode.len()+s.len());
    output.extend_from_slice(&length_encode);
    output.extend_from_slice(s);
    output
}

fn left_encode(x:u64) ->Vec<u8>{
    let mut n = ((64 - x.leading_zeros() + 7) / 8) as usize;
    if n == 0 { 
        n = 1; 
    }
    let mut value =x;
    let mut x_bytes:Vec<u8> = vec![0u8; n];
    for i in 0..n{
        x_bytes[n-(i+1)] = (value & (0xFF)) as u8;
        value = value >> 8;
    }
    let n_byte:u8 =n as u8;
    let mut output:Vec<u8> = Vec::with_capacity((n_byte + 1) as usize);
    output.push(n_byte);
    output.extend_from_slice(&x_bytes);
    output
}
fn bytepad(x:&[u8],w:usize)->Vec<u8>{
    let w_encode = left_encode(w as u64);
    let total_len = ((w_encode.len() + x.len() + w - 1) / w) * w;
    let mut output:Vec<u8> = Vec::with_capacity(total_len);
    output.extend_from_slice(&w_encode);
    output.extend_from_slice(x);
    output.resize(total_len, 0);
    output
}