use anyhow::{Result, bail};

pub fn hex_to_base64(input: &[u8]) -> Result<String> {
    let bytes = hex_decode(input)?;
    Ok(base64_encode(&bytes))
}

pub fn base64_encode(input: &[u8]) -> String {
    const LOOKUP: [u8; 64] = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
        b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd',
        b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
        b't', b'u', b'v', b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
        b'8', b'9', b'+', b'/',
    ];
    const PAD: u8 = b'=';

    let mut out = Vec::with_capacity(input.len().div_ceil(3) * 4);

    for chunk in input.chunks(3) {
        match chunk.len() {
            3 => {
                let acc = ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32);

                let b3 = ((acc >> 18) & 0b0011_1111) as u8 as usize;
                let b2 = ((acc >> 12) & 0b0011_1111) as u8 as usize;
                let b1 = ((acc >> 6) & 0b0011_1111) as u8 as usize;
                let b0 = (acc & 0b0011_1111) as u8 as usize;

                out.push(LOOKUP[b3]);
                out.push(LOOKUP[b2]);
                out.push(LOOKUP[b1]);
                out.push(LOOKUP[b0]);
            }
            2 => {
                let acc = ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8);

                let b3 = ((acc >> 18) & 0b0011_1111) as u8 as usize;
                let b2 = ((acc >> 12) & 0b0011_1111) as u8 as usize;
                let b1 = ((acc >> 6) & 0b0011_1111) as u8 as usize;

                out.push(LOOKUP[b3]);
                out.push(LOOKUP[b2]);
                out.push(LOOKUP[b1]);
                out.push(PAD);
            }
            1 => {
                let acc = (chunk[0] as u32) << 16;

                let b3 = ((acc >> 18) & 0b0011_1111) as u8 as usize;
                let b2 = ((acc >> 12) & 0b0011_1111) as u8 as usize;

                out.push(LOOKUP[b3]);
                out.push(LOOKUP[b2]);
                out.push(PAD);
                out.push(PAD);
            }
            _ => unreachable!(),
        }
    }

    String::from_utf8(out).unwrap()
}

pub fn hex_decode(input: &[u8]) -> Result<Vec<u8>> {
    fn hex_val(b: u8) -> Result<u8> {
        match b {
            b'0'..=b'9' => Ok(b - b'0'),
            b'a'..=b'f' => Ok(10 + (b - b'a')),
            b'A'..=b'F' => Ok(10 + (b - b'A')),
            _ => bail!("invalid byte: {b}"),
        }
    }

    let mut out = Vec::with_capacity(input.len() / 2);
    let mut hi: Option<u8> = None;

    for &b in input {
        if !b.is_ascii_hexdigit() {
            bail!("invalid hex character: {}", b as char);
        }

        let v = hex_val(b)?;
        if let Some(h) = hi.take() {
            out.push((h << 4) | v);
        } else {
            hi = Some(v);
        }
    }

    if hi.is_some() {
        bail!("odd number of hex digits");
    }

    Ok(out)
}

pub fn fixed_xor(left: &str, right: &str) -> Result<String> {
    if left.len() != right.len() {
        bail!(
            "This function requires that left and right have the same length: l len: {}, r len: {}",
            left.len(),
            right.len()
        )
    }
    let l_bytes = hex_decode(left.as_bytes())?;
    let r_bytes = hex_decode(right.as_bytes())?;

    let xored: Vec<u8> = l_bytes
        .iter()
        .zip(r_bytes.iter())
        .map(|(a, b)| a ^ b)
        .collect();

    Ok(hex_encode_lower(&xored))
}

fn hex_encode_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        s.push(HEX[(b >> 4) as usize] as char);
        s.push(HEX[(b & 0x0f) as usize] as char);
    }
    s
}
