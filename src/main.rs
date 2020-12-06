fn main() {}

pub fn hex_converter(_decimal: i32) -> String {
    let mut j: u32 = 1;
    let d: i32 = 16;
    let mut i = 1;
    let mut copy_decimal: i32 = _decimal;
    let mut hex = String::new();

    while 1 > 0 {
        if _decimal >= d.pow(i) {
            j = j + 1;
        } else if _decimal < d.pow(i) {
            break;
        }

        i += 1
    }

    for n in 0..j {
        hex.push_str(&hex_compare(copy_decimal / d.pow(j - 1 - n)));
        copy_decimal = copy_decimal % d.pow(j - 1 - n);
    }

    return hex;
}

fn hex_compare(number: i32) -> String {
    match number {
        number if number == 0 => "0".to_string(),
        number if number == 1 => "1".to_string(),
        number if number == 2 => "2".to_string(),
        _ => "".to_string(),
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn dec1_to_hex_1() {
        let decimal = 1;
        let hex = hex_converter(decimal);
        assert_eq!(hex, "1");
    }

    #[test]
    fn dec16_to_hex_10() {
        let decimal = 16;
        let hex = hex_converter(decimal);
        assert_eq!(hex, "10");
    }
}
