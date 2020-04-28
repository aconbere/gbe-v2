/* Combines two u8s into a little endian u16
 */
pub fn combine_ms_ls(ms: u8, ls: u8) -> u16 {
    ((ms as u16) << 8) | (ls as u16)
}

/* Splits a u16 into little endian u8s
 */
pub fn split_ms_ls(a: u16) -> (u8, u8) {
    let ms = ((a & 0xFF00) >> 8) as u8;
    let ls = (a & 0x00FF) as u8;
    (ms, ls)
}

pub fn check_bit(input: u8, n: u8) -> bool {
    (input & (1 << n)) != 0
}

// pub fn get_bit(input: u8, n: u8) -> u8 {
//     (input & (1 << n)) >> n
// }

pub fn set_bit(input: u8, n: u8, b: bool) -> u8 {
    if b {
        input | (1 << n)
    } else {
        input & !(1 << n)
    }
}

pub fn add_unsigned_signed(a: u16, b: u8) -> (u16, bool, bool) {
    let al = (a & 0x00FF) as u8;

    let bi: i8 = b as i8;

    if bi >= 0 {
        let (v, overflow) = a.overflowing_add(bi as u16);
        let hc = check_half_carry8(al, b);
        (v, overflow, hc)
    } else {
        let (v, overflow) = a.overflowing_sub((-bi) as u16);
        let hc = check_half_carry_sub8(al, b);
        (v, overflow, hc)
    }
}

pub fn check_half_carry16(a:u16, b:u16) -> bool {
    (a & 0x0FFF) + (b & 0x0FFF) > 0x0FFF
}

pub fn check_half_carry8(a:u8, b:u8) -> bool {
    ((a & 0x0F) + (b & 0x0F)) > 0x0F
}

pub fn check_half_carry_sub8(a:u8, b:u8) -> bool {
    (a & 0x0F) < (b & 0x0F)
}

// pub fn check_half_carry_sub16(a:u16, b:u16) -> bool {
//     (a & 0x0FFF) < (b & 0x0FFF)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_bit() {
        assert_eq!(check_bit(0b0000_0001, 0), true);
        assert_eq!(check_bit(0b0000_0010, 1), true);
        assert_eq!(check_bit(0b0000_0100, 2), true);
        assert_eq!(check_bit(0b0000_1000, 3), true);
        assert_eq!(check_bit(0b0001_0000, 4), true);
        assert_eq!(check_bit(0b0010_0000, 5), true);
        assert_eq!(check_bit(0b0100_0000, 6), true);
        assert_eq!(check_bit(0b1000_0000, 7), true);

        assert_eq!(check_bit(0b0000_0001, 1), false);
        assert_eq!(check_bit(0b0000_0010, 2), false);
        assert_eq!(check_bit(0b0000_0100, 3), false);
        assert_eq!(check_bit(0b0000_1000, 4), false);
        assert_eq!(check_bit(0b0001_0000, 5), false);
        assert_eq!(check_bit(0b0010_0000, 6), false);
        assert_eq!(check_bit(0b0100_0000, 7), false);
        assert_eq!(check_bit(0b1000_0000, 0), false);
    }

    #[test]
    fn test_set_bit() {
        assert_eq!(set_bit(0b0000_0000, 3, true), 0b0000_1000);
        assert_eq!(set_bit(0b1111_1111, 3, false), 0b1111_0111);
    }

    #[test]
    fn test_add_unsigned_signed() {
        // positive addition
        assert_eq!(add_unsigned_signed(0x0032, 0x0D), (0x003F, false, false));
        assert_eq!(add_unsigned_signed(0xFFF8, 0x13), (0x000B, true, false));
        assert_eq!(add_unsigned_signed(0x01FF, 0x13), (0x0212, false, true));

        // negative addition
        // assert_eq!(add_unsigned_signed(0x0032, 0xFD), (0x0023, false, true));
        // assert_eq!(add_unsigned_signed(0x0002, 0xFD), (0xFFF3, true, true));

        assert_eq!(add_unsigned_signed(0x000C, 0xFB), (0x0007, false, false));
    }

    #[test]
    fn unsigned_byte_to_signed() {
    }

    #[test]
    fn test_combine_and_split() {
        let ms = 0x0B;
        let ls = 0x0A;

        let c = combine_ms_ls(ms, ls);
        let (ms2,ls2) = split_ms_ls(c);
        assert_eq!(ms2, ms);
        assert_eq!(ls2, ls);
    }

    #[test]
    fn test_check_half_carry() {
        assert_eq!(check_half_carry8(0b0000_1111, 0b0000_0001), true);
        assert_eq!(check_half_carry8(0b0000_1110, 0b0000_0001), false);

        assert_eq!(check_half_carry16(0b0000_1111_0000_0000, 0b0000_0001_0000_0000), true);
        assert_eq!(check_half_carry16(0b0000_1110_0000_0000, 0b0000_0001_0000_0000), false);
    }
}
