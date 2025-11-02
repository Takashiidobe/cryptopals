pub mod set_1;

#[cfg(test)]
mod tests {
    use crate::set_1::{fixed_xor, hex_to_base64};
    use anyhow::Result;

    #[test]
    fn ex_1() -> Result<()> {
        let input = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let want = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(hex_to_base64(input)?, want);
        Ok(())
    }

    #[test]
    fn second() -> Result<()> {
        let left = "1c0111001f010100061a024b53535009181c";
        let right = "686974207468652062756c6c277320657965";
        let want = "746865206b696420646f6e277420706c6179";

        let got = fixed_xor(left, right)?;
        assert_eq!(got, want);
        Ok(())
    }
}
