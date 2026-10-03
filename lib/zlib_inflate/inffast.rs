/* inffast.c -- fast decoding
 * Copyright (C) 1995-2004 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 *
 * Rust translation of the frozen source at
 * 8e8505218ff400546323d71109207c781769e54e.
 */

#![cfg(not(ASMINF))]

use crate::bindings::{code, inflate_state, z_streamp, BAD, TYPE};
use core::ffi::{c_char, c_uint, c_ulong};
use core::ptr;

// Equivalent to the native byte-filled union: native-endian, with no alignment
// requirement on the input. Kept for targets without efficient unaligned loads.
#[cfg(not(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS))]
#[inline(always)]
unsafe fn get_unaligned16(p: *const u16) -> u16 {
    unsafe {
        let b = p.cast::<u8>();
        u16::from_ne_bytes([*b, *b.add(1)])
    }
}

#[inline(always)]
unsafe fn pull_byte(input: &mut *const u8, hold: &mut c_ulong, bits: &mut c_uint) {
    unsafe {
        *hold = hold.wrapping_add((**input as c_ulong).wrapping_shl(*bits));
        *input = (*input).add(1);
        *bits = bits.wrapping_add(8);
    }
}

// Preserve each load/store pair and both post-increments. In particular, these
// operations must remain sequential for overlapping matches.
#[inline(always)]
unsafe fn copy_byte(out: &mut *mut u8, from: &mut *mut u8) {
    unsafe {
        **out = **from;
        *out = (*out).add(1);
        *from = (*from).add(1);
    }
}

/// Decode literals, lengths and distances until input/output is insufficient,
/// an end-of-block symbol is read, or a data error is found.
///
/// # Safety
/// The caller must satisfy the native routine's entry conditions: a valid zlib
/// stream in LEN mode, at least six input bytes, at least 258 writable output
/// bytes, `start >= avail_out`, and fewer than eight buffered bits. The stream,
/// tables, window and output history must be valid for their recorded sizes.
#[no_mangle]
pub(crate) unsafe extern "C" fn inflate_fast(strm: z_streamp, start: c_uint) {
    unsafe {
        let state = (*strm).state.cast::<inflate_state>();
        let mut input = (*strm).next_in;
        let last = input.add((*strm).avail_in.wrapping_sub(5) as usize);
        let mut out = (*strm).next_out;
        // avail_in/out are native unsigned long, unlike the unsigned start
        // parameter and decoder locals. Preserve C's promotion before subtract.
        let beg = out.sub((start as c_ulong).wrapping_sub((*strm).avail_out) as usize);
        let end = out.add((*strm).avail_out.wrapping_sub(257) as usize);
        #[cfg(INFLATE_STRICT)]
        let dmax = (*state).dmax;
        let wsize = (*state).wsize;
        let whave = (*state).whave;
        let write = (*state).write;
        let window = (*state).window;
        let mut hold: c_ulong = (*state).hold;
        let mut bits: c_uint = (*state).bits;
        let lcode = (*state).lencode;
        let dcode = (*state).distcode;
        let lmask = (1 as c_uint).wrapping_shl((*state).lenbits).wrapping_sub(1);
        let dmask = (1 as c_uint)
            .wrapping_shl((*state).distbits)
            .wrapping_sub(1);

        // The outer loop corresponds to the native do/while. The inner loops
        // replace only the dolen and dodist table-link gotos.
        'decode: loop {
            if bits < 15 {
                pull_byte(&mut input, &mut hold, &mut bits);
                pull_byte(&mut input, &mut hold, &mut bits);
            }
            let mut this: code = ptr::read(lcode.add((hold & lmask as c_ulong) as usize));
            'dolen: loop {
                let mut op = this.bits as c_uint;
                hold = hold.wrapping_shr(op);
                bits = bits.wrapping_sub(op);
                op = this.op as c_uint;
                if op == 0 {
                    // Literal.
                    *out = this.val as u8;
                    out = out.add(1);
                } else if op & 16 != 0 {
                    // Length base and any extra length bits.
                    let mut len = this.val as c_uint;
                    op &= 15;
                    if op != 0 {
                        if bits < op {
                            pull_byte(&mut input, &mut hold, &mut bits);
                        }
                        len = len.wrapping_add(
                            (hold as c_uint) & (1 as c_uint).wrapping_shl(op).wrapping_sub(1),
                        );
                        hold = hold.wrapping_shr(op);
                        bits = bits.wrapping_sub(op);
                    }
                    if bits < 15 {
                        pull_byte(&mut input, &mut hold, &mut bits);
                        pull_byte(&mut input, &mut hold, &mut bits);
                    }
                    this = ptr::read(dcode.add((hold & dmask as c_ulong) as usize));
                    'dodist: loop {
                        op = this.bits as c_uint;
                        hold = hold.wrapping_shr(op);
                        bits = bits.wrapping_sub(op);
                        op = this.op as c_uint;
                        if op & 16 != 0 {
                            // Distance base and any extra distance bits.
                            let mut dist = this.val as c_uint;
                            op &= 15;
                            if bits < op {
                                pull_byte(&mut input, &mut hold, &mut bits);
                                if bits < op {
                                    pull_byte(&mut input, &mut hold, &mut bits);
                                }
                            }
                            dist = dist.wrapping_add(
                                (hold as c_uint) & (1 as c_uint).wrapping_shl(op).wrapping_sub(1),
                            );
                            #[cfg(INFLATE_STRICT)]
                            if dist > dmax {
                                (*strm).msg =
                                    b"invalid distance too far back\0".as_ptr() as *mut c_char;
                                (*state).mode = BAD;
                                break 'decode;
                            }
                            hold = hold.wrapping_shr(op);
                            bits = bits.wrapping_sub(op);
                            op = out.offset_from(beg) as c_uint;
                            if dist > op {
                                // Copy first from history, possibly wrapping
                                // around the window, then from current output.
                                op = dist.wrapping_sub(op);
                                if op > whave {
                                    (*strm).msg =
                                        b"invalid distance too far back\0".as_ptr() as *mut c_char;
                                    (*state).mode = BAD;
                                    break 'decode;
                                }
                                let mut from = window;
                                if write == 0 {
                                    from = from.add(wsize.wrapping_sub(op) as usize);
                                    if op < len {
                                        len = len.wrapping_sub(op);
                                        loop {
                                            copy_byte(&mut out, &mut from);
                                            op = op.wrapping_sub(1);
                                            if op == 0 {
                                                break;
                                            }
                                        }
                                        from = out.sub(dist as usize);
                                    }
                                } else if write < op {
                                    from = from
                                        .add(wsize.wrapping_add(write).wrapping_sub(op) as usize);
                                    op = op.wrapping_sub(write);
                                    if op < len {
                                        len = len.wrapping_sub(op);
                                        loop {
                                            copy_byte(&mut out, &mut from);
                                            op = op.wrapping_sub(1);
                                            if op == 0 {
                                                break;
                                            }
                                        }
                                        from = window;
                                        if write < len {
                                            op = write;
                                            len = len.wrapping_sub(op);
                                            loop {
                                                copy_byte(&mut out, &mut from);
                                                op = op.wrapping_sub(1);
                                                if op == 0 {
                                                    break;
                                                }
                                            }
                                            from = out.sub(dist as usize);
                                        }
                                    }
                                } else {
                                    from = from.add(write.wrapping_sub(op) as usize);
                                    if op < len {
                                        len = len.wrapping_sub(op);
                                        loop {
                                            copy_byte(&mut out, &mut from);
                                            op = op.wrapping_sub(1);
                                            if op == 0 {
                                                break;
                                            }
                                        }
                                        from = out.sub(dist as usize);
                                    }
                                }
                                while len > 2 {
                                    copy_byte(&mut out, &mut from);
                                    copy_byte(&mut out, &mut from);
                                    copy_byte(&mut out, &mut from);
                                    len = len.wrapping_sub(3);
                                }
                                if len != 0 {
                                    copy_byte(&mut out, &mut from);
                                    if len > 1 {
                                        copy_byte(&mut out, &mut from);
                                    }
                                }
                            } else {
                                // Direct output match. Preserve the native
                                // aligned 16-bit copy and its short patterns.
                                let mut from = out.sub(dist as usize);
                                if (out as usize).wrapping_sub(1) & 1 == 0 {
                                    copy_byte(&mut out, &mut from);
                                    len = len.wrapping_sub(1);
                                }
                                let mut sout = out.cast::<u16>();
                                let mut loops: c_ulong;
                                if dist > 2 {
                                    let mut sfrom = from.cast::<u16>();
                                    loops = (len >> 1) as c_ulong;
                                    loop {
                                        #[cfg(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS)]
                                        let value = ptr::read_unaligned(sfrom);
                                        #[cfg(not(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS))]
                                        let value = get_unaligned16(sfrom);
                                        ptr::write(sout, value);
                                        sout = sout.add(1);
                                        sfrom = sfrom.add(1);
                                        loops = loops.wrapping_sub(1);
                                        if loops == 0 {
                                            break;
                                        }
                                    }
                                    out = sout.cast::<u8>();
                                    from = sfrom.cast::<u8>();
                                } else {
                                    // dist == 1 or dist == 2. The source union
                                    // duplicates the second address-order byte
                                    // for dist == 1, independent of endianness.
                                    let mut pat16 = ptr::read(sout.sub(1));
                                    if dist == 1 {
                                        let mut bytes = pat16.to_ne_bytes();
                                        bytes[0] = bytes[1];
                                        pat16 = u16::from_ne_bytes(bytes);
                                    }
                                    loops = (len >> 1) as c_ulong;
                                    loop {
                                        ptr::write(sout, pat16);
                                        sout = sout.add(1);
                                        loops = loops.wrapping_sub(1);
                                        if loops == 0 {
                                            break;
                                        }
                                    }
                                    out = sout.cast::<u8>();
                                }
                                if len & 1 != 0 {
                                    copy_byte(&mut out, &mut from);
                                }
                            }
                        } else if op & 64 == 0 {
                            // Second-level distance code.
                            let index = (this.val as c_ulong).wrapping_add(
                                hold & ((1 as c_uint).wrapping_shl(op).wrapping_sub(1) as c_ulong),
                            );
                            this = ptr::read(dcode.add(index as usize));
                            continue 'dodist;
                        } else {
                            (*strm).msg = b"invalid distance code\0".as_ptr() as *mut c_char;
                            (*state).mode = BAD;
                            break 'decode;
                        }
                        break 'dodist;
                    }
                } else if op & 64 == 0 {
                    // Second-level literal/length code.
                    let index = (this.val as c_ulong).wrapping_add(
                        hold & ((1 as c_uint).wrapping_shl(op).wrapping_sub(1) as c_ulong),
                    );
                    this = ptr::read(lcode.add(index as usize));
                    continue 'dolen;
                } else if op & 32 != 0 {
                    (*state).mode = TYPE;
                    break 'decode;
                } else {
                    (*strm).msg = b"invalid literal/length code\0".as_ptr() as *mut c_char;
                    (*state).mode = BAD;
                    break 'decode;
                }
                break 'dolen;
            }
            if !(input < last && out < end) {
                break 'decode;
            }
        }

        // Return unused whole bytes. Entry bits < 8 ensures this cannot move
        // input before its starting position, including every error exit.
        let len = bits >> 3;
        input = input.sub(len as usize);
        bits = bits.wrapping_sub(len.wrapping_shl(3));
        hold &= (1 as c_uint).wrapping_shl(bits).wrapping_sub(1) as c_ulong;

        (*strm).next_in = input;
        (*strm).next_out = out;
        // Native C explicitly narrows these pointer differences to unsigned
        // before storing them in the unsigned-long stream fields.
        (*strm).avail_in = (if input < last {
            5isize.wrapping_add(last.offset_from(input))
        } else {
            5isize.wrapping_sub(input.offset_from(last))
        } as c_uint) as c_ulong;
        (*strm).avail_out = (if out < end {
            257isize.wrapping_add(end.offset_from(out))
        } else {
            257isize.wrapping_sub(out.offset_from(end))
        } as c_uint) as c_ulong;
        (*state).hold = hold;
        (*state).bits = bits;
    }
}
