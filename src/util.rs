/// Argument:
///   hex: 0x0123ABCD
pub fn hex_to_usize(hex: &str) -> usize {
    let mut chars = hex.chars();
    chars.next(); // remove 0
    chars.next(); // remove x

    let chars = chars.rev();
    let mut res: usize = 0;
    for (i, c) in chars.enumerate() {
        let v: usize;
        match c.to_ascii_lowercase() {
            '0' => v = 0x0,
            '1' => v = 0x1,
            '2' => v = 0x2,
            '3' => v = 0x3,
            '4' => v = 0x4,
            '5' => v = 0x5,
            '6' => v = 0x6,
            '7' => v = 0x7,
            '8' => v = 0x8,
            '9' => v = 0x9,
            'a' => v = 0xa,
            'b' => v = 0xb,
            'c' => v = 0xc,
            'd' => v = 0xd,
            'e' => v = 0xe,
            'f' => v = 0xf,
            _ => panic!("Not hex: {}", c),
        }
        res += v * (16_usize.pow(i as u32));
    }
    res
}
