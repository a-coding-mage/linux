// SPDX-License-Identifier: GPL-2.0-only
/* LZO1X Decompressor from LZO. Rust translation of the C implementation. */

use core::ptr;

extern "C" {
    static M2_MAX_OFFSET: usize;
    static MIN_ZERO_RUN_LENGTH: usize;
    static LZO_E_ERROR: i32;
    static LZO_E_OK: i32;
    static LZO_E_INPUT_NOT_CONSUMED: i32;
    static LZO_E_INPUT_OVERRUN: i32;
    static LZO_E_OUTPUT_OVERRUN: i32;
    static LZO_E_LOOKBEHIND_OVERRUN: i32;
}

const MAX_255_COUNT: usize = (usize::MAX / 255) - 2;

#[inline]
unsafe fn get_unaligned_le16(p: *const u8) -> usize {
    ptr::read_unaligned(p as *const u16) as usize
}

#[no_mangle]
pub unsafe extern "C" fn lzo1x_decompress_safe(
    input: *const u8,
    in_len: usize,
    out: *mut u8,
    out_len: *mut usize,
) -> i32 {
    let ip_end = input.add(in_len);
    let op_end = out.add(*out_len);
    let mut ip = input;
    let mut op = out;
    let mut t: usize;
    let mut next: usize;
    let mut state: usize = 0;
    let mut m_pos: *const u8;
    let bitstream_version: u8;

    macro_rules! have_ip { ($n:expr) => { ip_end.offset_from(ip) as usize >= $n }; }
    macro_rules! have_op { ($n:expr) => { op_end.offset_from(op) as usize >= $n }; }
    macro_rules! need_ip { ($n:expr) => { if !have_ip!($n) { *out_len = op.offset_from(out) as usize; return LZO_E_INPUT_OVERRUN; } }; }
    macro_rules! need_op { ($n:expr) => { if !have_op!($n) { *out_len = op.offset_from(out) as usize; return LZO_E_OUTPUT_OVERRUN; } }; }
    macro_rules! test_lb { ($p:expr) => { if ($p as usize) < (out as usize) { *out_len = op.offset_from(out) as usize; return LZO_E_LOOKBEHIND_OVERRUN; } }; }

    if in_len < 3 { *out_len = 0; return LZO_E_INPUT_OVERRUN; }
    if in_len >= 5 && *ip == 17 { bitstream_version = *ip.add(1); ip = ip.add(2); }
    else { bitstream_version = 0; }

    if *ip > 17 {
        t = (*ip as usize) - 17; ip = ip.add(1);
        if t < 4 { next = t; need_ip!(next + 3); need_op!(next); let mut n = next; while n > 0 { *op = *ip; op = op.add(1); ip = ip.add(1); n -= 1; } state = next; }
        else { next = 0; }
        if t >= 4 {
            need_op!(t); need_ip!(t + 3);
            let mut n = t; while n > 0 { *op = *ip; op = op.add(1); ip = ip.add(1); n -= 1; }
            state = 4; continue_decompress!(next, state, t, ip, op, ip_end, op_end, out, out_len, bitstream_version);
        }
    }

    'decompress: loop {
        t = *ip as usize; ip = ip.add(1);
        if t < 16 {
            if state == 0 {
                if t == 0 {
                    let ip_last = ip; while *ip == 0 { ip = ip.add(1); need_ip!(1); }
                    let offset = ip.offset_from(ip_last) as usize; if offset > MAX_255_COUNT { return LZO_E_ERROR; }
                    t += (offset << 8) - offset + 15 + *ip as usize; ip = ip.add(1);
                }
                t += 3;
                need_op!(t); need_ip!(t + 3); let mut n = t;
                while n > 0 { *op = *ip; op = op.add(1); ip = ip.add(1); n -= 1; }
                state = 4; continue;
            } else if state != 4 {
                next = t & 3; m_pos = op.sub(1).sub(t >> 2).sub((*ip as usize) << 2); ip = ip.add(1); test_lb!(m_pos); need_op!(2);
                *op = *m_pos; *op.add(1) = *m_pos.add(1); op = op.add(2);
            } else { next = t & 3; m_pos = op.sub(1 + M2_MAX_OFFSET).sub(t >> 2).sub((*ip as usize) << 2); ip = ip.add(1); t = 3; }
        } else if t >= 64 {
            next = t & 3; m_pos = op.sub(1).sub((t >> 2) & 7).sub((*ip as usize) << 3); ip = ip.add(1); t = (t >> 5) + 1;
        } else if t >= 32 {
            t = (t & 31) + 2;
            if t == 2 { let ip_last = ip; while *ip == 0 { ip = ip.add(1); need_ip!(1); } let offset = ip.offset_from(ip_last) as usize; if offset > MAX_255_COUNT { return *LZO_E_ERROR; } t += (offset << 8) - offset + 31 + *ip as usize; ip = ip.add(1); need_ip!(2); }
            m_pos = op.sub(1); next = get_unaligned_le16(ip); ip = ip.add(2); m_pos = m_pos.sub(next >> 2); next &= 3;
        } else {
            need_ip!(2); next = get_unaligned_le16(ip);
            if (next & 0xfffc) == 0xfffc && (t & 0xf8) == 0x18 && bitstream_version != 0 {
                need_ip!(3); t = (t & 7) | ((*ip.add(2) as usize) << 3); t += MIN_ZERO_RUN_LENGTH; need_op!(t); ptr::write_bytes(op, 0, t); op = op.add(t); next &= 3; ip = ip.add(3);
            } else {
                m_pos = op.sub((t & 8) << 11); t = (t & 7) + 2;
                if t == 2 { let ip_last = ip; while *ip == 0 { ip = ip.add(1); need_ip!(1); } let offset = ip.offset_from(ip_last) as usize; if offset > MAX_255_COUNT { return LZO_E_ERROR; } t += (offset << 8) - offset + 7 + *ip as usize; ip = ip.add(1); need_ip!(2); next = get_unaligned_le16(ip); }
                ip = ip.add(2); m_pos = m_pos.sub(next >> 2); next &= 3; if m_pos == op { *out_len = op.offset_from(out) as usize; return if t != 3 { LZO_E_ERROR } else if ip == ip_end { LZO_E_OK } else if ip < ip_end { LZO_E_INPUT_NOT_CONSUMED } else { LZO_E_INPUT_OVERRUN }; } m_pos = m_pos.sub(0x4000);
            }
        }
        test_lb!(m_pos); need_op!(t); let oe = op.add(t); *op = *m_pos; *op.add(1) = *m_pos.add(1); op = op.add(2); m_pos = m_pos.add(2); while op < oe { *op = *m_pos; op = op.add(1); m_pos = m_pos.add(1); }
        state = next; t = next; need_ip!(t + 3); need_op!(t); while t > 0 { *op = *ip; op = op.add(1); ip = ip.add(1); t -= 1; }
    }
}

// The C source's initial literal-run fallthrough is represented directly above;
// this macro is retained only as a compact control-flow marker for that branch.
macro_rules! continue_decompress { ($($x:expr),*) => {}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
