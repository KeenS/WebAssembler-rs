#[inline]
pub fn write_uint8(buf: &mut Vec<u8>, u: u8) -> usize {
    buf.push(u);
    1
}

#[inline]
pub fn write_uint16(buf: &mut Vec<u8>, u: u16) -> usize {
    let mut size = 0;
    size += write_uint8(buf, (u & 0xff) as u8);
    size += write_uint8(buf, ((u >> 8) & 0xff) as u8);
    size
}

#[inline]
pub fn write_uint32(buf: &mut Vec<u8>, u: u32) -> usize {
    let mut size = 0;
    size += write_uint16(buf, (u & 0xffff) as u16);
    size += write_uint16(buf, ((u >> 16) & 0xffff) as u16);
    size
}

#[inline]
pub fn write_uint64(buf: &mut Vec<u8>, u: u64) -> usize {
    let mut size = 0;
    size += write_uint32(buf, (u & 0xffffffff) as u32);
    size += write_uint32(buf, ((u >> 32) & 0xffffffff) as u32);
    size
}

#[inline]
pub fn write_varuint1(buf: &mut Vec<u8>, u: u8) -> usize {
    write_uint8(buf, u)
}

#[allow(dead_code)]
#[inline]
pub fn write_varuint7(buf: &mut Vec<u8>, u: u8) -> usize {
    write_uint8(buf, u)
}

#[inline]
pub fn write_varint7(buf: &mut Vec<u8>, i: i8) -> usize {
    write_uint8(buf, (i as u8) ^ 0x80)
}

macro_rules! gen_write_var {
    ($name: ident, $ty: ty) => {
        #[inline]
        #[allow(unused_comparisons)]
        #[allow(overflowing_literals)]
        pub fn $name(buf: &mut Vec<u8>, mut u: $ty) -> usize {
            let end: i8 = if u < 0 { 0xff } else { 0 };

            let mut size = 0;
            let bit7 = 0b01111111;
            let mut cur: u8 = (u & bit7) as u8;
            // rust generates sar for i32 by >>
            u >>= 7;
            while u != (end as $ty) {
                size += write_uint8(buf, cur | 0x80);
                cur = (u & bit7) as u8;
                u >>= 7;
            }
            size += write_uint8(buf, cur);
            size
        }

    }
}

gen_write_var!(write_varuint32, u32);
gen_write_var!(write_varint32, i32);
gen_write_var!(write_varint64, i64);



#[inline]
pub fn write_slice(buf: &mut Vec<u8>, u: &[u8]) -> usize {
    buf.extend_from_slice(u);
    u.len()
}
