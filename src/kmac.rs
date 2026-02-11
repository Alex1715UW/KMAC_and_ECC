use crate::shake::Shake;

const RIGHT_ENCODE_0:[u8;2] = [0x0,0x1];
pub fn kmacxof256(k:&[u8], x:&[u8], l:usize, s:&[u8]) ->Vec<u8>{
    assert_eq!(l % 8, 0, "Implementation restriction: output length (in bits) must be a multiple of 8");
    let mut shake = Shake::new(&[]);
    let encode_n = encode_string(b"KMAC");
    let encode_s = encode_string(s);
    let mut sn = Vec::with_capacity(encode_n.len() + encode_s.len());
    sn.extend_from_slice(&encode_n);
    sn.extend_from_slice(&encode_s);
    sn = bytepad(sn.as_slice(), 136);
    shake.absorb(&sn);
    let key = bytepad(encode_string(k).as_slice(),136);
    shake.absorb(&key);
    let mut x_append0:Vec<u8> = Vec::with_capacity(x.len()+RIGHT_ENCODE_0.len() +1);
    x_append0.extend_from_slice(&x);
    x_append0.extend_from_slice(&RIGHT_ENCODE_0);
    x_append0.push(0x04);
    shake.absorb(&x_append0);
    let mut value: Vec<u8> = vec![0u8; l/8];
    shake.squeeze(&mut value);
    value
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