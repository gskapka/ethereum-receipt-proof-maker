fn left_pad_with_zero(string: &str) -> String {
    format!("0{}", string)
}

pub fn strip_hex_prefix(prefixed_hex : &str) -> String {
    let res = str::replace(prefixed_hex, "0x", "");
    match res.len() % 2 {
        0 => res,
        _ => left_pad_with_zero(&res),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_left_pad_string_with_zero_correctly() {
        let dummyHex = "0xc0ffee";
        let expected_result = "00xc0ffee".to_string();
        let result = left_pad_with_zero(dummyHex);
        assert!(result == expected_result);
    }

    #[test]
    fn should_strip_hex_prefix_correctly() {
        let dummyHex = "0xc0ffee";
        let expected_result = "c0ffee".to_string();
        let result = strip_hex_prefix(dummyHex);
        assert!(result == expected_result)
    }

    #[test]
    fn should_not_strip_missing_hex_prefix_correctly() {
        let dummyHex = "c0ffee";
        let expected_result = "c0ffee".to_string();
        let result = strip_hex_prefix(dummyHex);
        assert!(result == expected_result)
    }
}
