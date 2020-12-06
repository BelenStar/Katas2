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
        number if number == 3 => "3".to_string(),
        number if number == 4 => "4".to_string(),
        number if number == 5 => "5".to_string(),
        number if number == 6 => "6".to_string(),
        number if number == 7 => "7".to_string(),
        number if number == 8 => "8".to_string(),
        number if number == 9 => "9".to_string(),
        number if number == 10 => "A".to_string(),
        number if number == 11 => "B".to_string(),
        number if number == 12 => "C".to_string(),
        number if number == 13 => "D".to_string(),
        number if number == 14 => "E".to_string(),
        number if number == 15 => "F".to_string(),
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

    #[test]
    fn dec26_to_hex_1_a() {
        let decimal = 26;
        let hex = hex_converter(decimal);
        assert_eq!(hex, "1A");
    }

    #[test]
    fn dec31_to_hex_1_f() {
        let decimal = 31;
        let hex = hex_converter(decimal);
        assert_eq!(hex, "1F");
    }

    #[test]
    fn dec500_to_hex_1_f4() {
        let decimal = 500;
        let hex = hex_converter(decimal);
        assert_eq!(hex, "1F4");
    }

    #[test]
    fn dec1000_to_hex_3_e8() {
        let decimal = 1000;
        let hex = hex_converter(decimal);
        assert_eq!(hex, "3E8");
    }
}
