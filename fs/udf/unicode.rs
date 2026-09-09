// SPDX-License-Identifier: GPL-2.0-only
/*
 * unicode.c
 *
 * PURPOSE
 *	Routines for converting between UTF-8 and OSTA Compressed Unicode.
 *      Also handles filename mangling
 */

use core::ffi::{c_int, c_uchar, c_uint, c_void};

const PLANE_SIZE: u32 = 0x10000;
const UNICODE_MAX: u32 = 0x10ffff;
const SURROGATE_MASK: u32 = 0xfffff800;
const SURROGATE_PAIR: u32 = 0x0000d800;
const SURROGATE_LOW: u32 = 0x00000400;
const SURROGATE_CHAR_BITS: u32 = 10;
const SURROGATE_CHAR_MASK: u32 = (1 << SURROGATE_CHAR_BITS) - 1;
const ILLEGAL_CHAR_MARK: u8 = b'_';
const EXT_MARK: u8 = b'.';
const CRC_MARK: u8 = b'#';
const EXT_SIZE: usize = 5;
const CRC_LEN: usize = 5;
const NLS_MAX_CHARSET_SIZE: usize = 6;
const MAX_WCHAR_T: u32 = 0x7fffffff;
const ENAMETOOLONG: c_int = 36;
const EINVAL: c_int = 22;
const EIO: c_int = 5;

type unicode_t = u32;
type wchar_t = i32;

#[repr(C)] pub struct super_block { _private: [u8; 0] }

extern "C" {
    fn utf32_to_utf8(c: unicode_t, out: *mut u8, len: c_int) -> c_int;
    fn utf8_to_utf32(input: *const u8, len: c_int, out: *mut unicode_t) -> c_int;
    fn UDF_SB(sb: *mut super_block) -> *mut c_void;
    fn crc_itu_t(crc: u16, buffer: *const u8, len: usize) -> u16;
    fn hex_asc_upper_hi(x: u32) -> u8;
    fn hex_asc_upper_lo(x: u32) -> u8;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn pr_err(fmt: *const u8, ...);
    fn pr_warn(fmt: *const u8, ...);
}

unsafe fn get_utf16_char(str_i: *const u8, str_i_max_len: c_int,
                         str_i_idx: c_int, u_ch: c_int, ret: *mut unicode_t) -> c_int {
    let start_idx = str_i_idx;
    let mut idx = str_i_idx;
    let mut c = *str_i.add(idx as usize) as u32;
    idx += 1;
    if u_ch > 1 { c = (c << 8) | *str_i.add(idx as usize) as u32; idx += 1; }
    if (c & SURROGATE_MASK) == SURROGATE_PAIR {
        if idx >= str_i_max_len || (c & SURROGATE_LOW) != 0 {
            c = UNICODE_MAX + 1;
        } else {
            let mut next = (*str_i.add(idx as usize) as u32) << 8; idx += 1;
            next |= *str_i.add(idx as usize) as u32; idx += 1;
            if (next & SURROGATE_MASK) != SURROGATE_PAIR || (next & SURROGATE_LOW) == 0 {
                c = UNICODE_MAX + 1;
            } else {
                c = PLANE_SIZE + ((c & SURROGATE_CHAR_MASK) << SURROGATE_CHAR_BITS)
                    + (next & SURROGATE_CHAR_MASK);
            }
        }
    }
    *ret = c;
    idx - start_idx
}

unsafe fn udf_name_conv_char(str_o: *mut u8, str_o_max_len: c_int, str_o_idx: *mut c_int,
    str_i: *const u8, str_i_max_len: c_int, str_i_idx: *mut c_int, u_ch: c_int,
    needs_crc: *mut c_int, conv_f: Option<unsafe extern "C" fn(wchar_t, *mut u8, c_int) -> c_int>,
    translate: c_int) -> c_int {
    let mut ill = 0; let mut len; let mut gotch = 0;
    while gotch == 0 && *str_i_idx < str_i_max_len {
        if *str_o_idx >= str_o_max_len { *needs_crc = 1; return gotch; }
        let mut c = 0u32; len = get_utf16_char(str_i, str_i_max_len, *str_i_idx, u_ch, &mut c);
        if c == 0 || c > UNICODE_MAX || (conv_f.is_some() && c > MAX_WCHAR_T) || (translate != 0 && c == b'/' as u32) {
            ill = 1; if translate == 0 { gotch = 1; }
        } else if ill { break; } else { gotch = 1; }
        *str_i_idx += len;
    }
    let mut c = 0u32;
    if ill != 0 { *needs_crc = 1; c = ILLEGAL_CHAR_MARK as u32; gotch = 1; }
    if gotch != 0 {
        len = if let Some(f) = conv_f { f(c as wchar_t, str_o.add(*str_o_idx as usize), str_o_max_len - *str_o_idx) }
            else { let n = utf32_to_utf8(c, str_o.add(*str_o_idx as usize), str_o_max_len - *str_o_idx); if n < 0 { -ENAMETOOLONG } else { n } };
        if len >= 0 { *str_o_idx += len; } else if len == -ENAMETOOLONG { *needs_crc = 1; gotch = 0; }
        else { *str_o.add(*str_o_idx as usize) = ILLEGAL_CHAR_MARK; *str_o_idx += 1; *needs_crc = 1; }
    }
    gotch
}

unsafe fn udf_name_from_CS0(_sb: *mut super_block, out: *mut u8, max: c_int,
    ocu: *const u8, olen: c_int, translate: c_int) -> c_int {
    if max <= 0 { return 0; }
    if olen == 0 { memset(out as *mut c_void, 0, max as usize); return 0; }
    let cmp = *ocu; if cmp != 8 && cmp != 16 { memset(out as *mut c_void, 0, max as usize); return -EINVAL; }
    let uch = (cmp >> 3) as c_int; let data = ocu.add(1); let n = olen - 1;
    if n % uch != 0 { return -EINVAL; }
    let mut oi = 0; let mut ii = 0; let mut crc_needed = 0;
    while ii < n && oi < max {
        let mut c = 0u32; let l = get_utf16_char(data, n, ii, uch, &mut c); ii += l;
        if c == 0 || c > UNICODE_MAX || (translate != 0 && c == b'/' as u32) { crc_needed = 1; c = b'_' as u32; }
        let z = utf32_to_utf8(c, out.add(oi as usize), max - oi);
        if z >= 0 { oi += z; } else { *out.add(oi as usize) = b'_'; oi += 1; crc_needed = 1; }
    }
    if translate != 0 && crc_needed != 0 && oi + 5 <= max {
        let v = crc_itu_t(0, data, n as usize); *out.add(oi as usize)=b'#';
        *out.add((oi+1) as usize)=hex_asc_upper_hi((v>>8) as u32); *out.add((oi+2) as usize)=hex_asc_upper_lo((v>>8) as u32);
        *out.add((oi+3) as usize)=hex_asc_upper_hi(v as u32); *out.add((oi+4) as usize)=hex_asc_upper_lo(v as u32); oi += 5;
    }
    oi
}

unsafe fn udf_name_to_CS0(_sb: *mut super_block, out: *mut u8, max: c_int, input: *const u8, len: c_int) -> c_int {
    if max <= 0 { return 0; } memset(out as *mut c_void, 0, max as usize); *out=8;
    let mut oi=1; let mut i=0; while i<len { let mut c=0u32; let mut n=utf8_to_utf32(input.add(i as usize),len-i,&mut c); if n<=0 || c>UNICODE_MAX {n=1;c=b'?' as u32;}
        if c>255 { *out=16; return 0; } if oi+1>max{return 0;} *out.add(oi as usize)=c as u8; oi+=1;i+=n; } oi
}

pub unsafe extern "C" fn udf_dstrCS0toChar(sb:*mut super_block, out:*mut u8, olen:c_int, input:*const u8, ilen:c_int)->c_int {
    let mut n=0; if ilen>0 { n=*input.add((ilen-1) as usize) as c_int; if n>=ilen {n=ilen-1;} } udf_name_from_CS0(sb,out,olen,input,n,0)
}
pub unsafe extern "C" fn udf_get_filename(sb:*mut super_block, s:*const u8, sl:c_int, d:*mut u8, dl:c_int)->c_int { if sl==0{return -EIO;} if dl<=0{return 0;} let n=udf_name_from_CS0(sb,d,dl,s,sl,1); if n==0{-EINVAL}else{n} }
pub unsafe extern "C" fn udf_put_filename(sb:*mut super_block, s:*const u8, sl:c_int, d:*mut u8, dl:c_int)->c_int { udf_name_to_CS0(sb,d,dl,s,sl) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
