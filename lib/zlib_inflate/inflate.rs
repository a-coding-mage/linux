/* inflate.c -- zlib decompression, translated to Rust.
 * Copyright (C) 1995-2005 Mark Adler
 * Based on the Linux 1.2.3-derived implementation by Richard Purdie.
 * Conditions of distribution and use: include/linux/zlib.h.
 * This is an altered source version. Native layouts/constants are bindgen's.
 */

use crate::bindings::*;
#[cfg(not(ASMINF))]
use crate::zlib_inffast::inflate_fast;
use crate::zlib_inftrees::zlib_inflate_table;
use crate::zlib_infutil::zlib_adler32;
use core::ffi::{c_char, c_int, c_uint, c_ulong};
use core::{mem, ptr};

#[cfg(ASMINF)]
extern "C" {
    // The native ASMINF row supplies an architecture assembly implementation.
    fn inflate_fast(strm: z_streamp, start: c_uint);
}

#[cfg(CONFIG_ZLIB_DFLTCC)]
compile_error!("Rust inflate requires the DFLTCC reset/typed/window/checksum hooks; this source currently supports the non-DFLTCC row only");

mod fixed {
    use crate::bindings::code;
    include!("inffixed_header.rs");
}

#[inline]
fn reverse(q: c_ulong) -> c_ulong {
    ((q >> 24) & 0xff) + ((q >> 8) & 0xff00) + ((q & 0xff00) << 8) + ((q & 0xff) << 24)
}

#[no_mangle]
pub(crate) unsafe extern "C" fn zlib_inflate_workspacesize() -> c_int {
    mem::size_of::<inflate_workspace>() as c_int
}

#[no_mangle]
pub(crate) unsafe extern "C" fn zlib_inflateReset(strm: z_streamp) -> c_int {
    unsafe {
        if strm.is_null() || (*strm).state.is_null() {
            return Z_STREAM_ERROR;
        }
        let state = (*strm).state.cast::<inflate_state>();
        (*strm).total_in = 0;
        (*strm).total_out = 0;
        (*state).total = 0;
        (*strm).msg = ptr::null_mut();
        (*strm).adler = 1;
        (*state).mode = HEAD;
        (*state).last = 0;
        (*state).havedict = 0;
        (*state).dmax = 32768;
        (*state).hold = 0;
        (*state).bits = 0;
        let codes = ptr::addr_of_mut!((*state).codes).cast::<code>();
        (*state).lencode = codes;
        (*state).distcode = codes;
        (*state).next = codes;
        (*state).wsize = 1u32 << (*state).wbits;
        (*state).write = 0;
        (*state).whave = 0;
        Z_OK as c_int
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn zlib_inflateInit2(
    strm: z_streamp,
    mut window_bits: c_int,
) -> c_int {
    unsafe {
        if strm.is_null() {
            return Z_STREAM_ERROR;
        }
        (*strm).msg = ptr::null_mut();
        let workspace = (*strm).workspace.cast::<inflate_workspace>();
        let state = ptr::addr_of_mut!((*workspace).inflate_state);
        (*strm).state = state.cast();
        if window_bits < 0 {
            (*state).wrap = 0;
            window_bits = window_bits.wrapping_neg();
        } else {
            (*state).wrap = (window_bits >> 4) + 1;
        }
        if !(8..=15).contains(&window_bits) {
            return Z_STREAM_ERROR;
        }
        (*state).wbits = window_bits as c_uint;
        // The no-flush wrapper allocates only inflate_state. Computing this
        // address must not create a reference to a full, unallocated workspace.
        (*state).window = ptr::addr_of_mut!((*workspace).working_window).cast::<u8>();
        zlib_inflateReset(strm)
    }
}

unsafe fn zlib_fixedtables(state: *mut inflate_state) {
    unsafe {
        (*state).lencode = fixed::LENFIX.as_ptr();
        (*state).lenbits = 9;
        (*state).distcode = fixed::DISTFIX.as_ptr();
        (*state).distbits = 5;
    }
}

unsafe fn zlib_updatewindow(strm: z_streamp, out: c_uint) {
    unsafe {
        let state = (*strm).state.cast::<inflate_state>();
        let mut copy = (out as c_ulong).wrapping_sub((*strm).avail_out) as c_uint;
        if copy >= (*state).wsize {
            // memcpy of zero bytes in C is also used with window == NULL.
            if (*state).wsize != 0 {
                ptr::copy_nonoverlapping(
                    (*strm).next_out.sub((*state).wsize as usize),
                    (*state).window,
                    (*state).wsize as usize,
                );
            }
            (*state).write = 0;
            (*state).whave = (*state).wsize;
        } else {
            let dist = ((*state).wsize - (*state).write).min(copy);
            if dist != 0 {
                ptr::copy_nonoverlapping(
                    (*strm).next_out.sub(copy as usize),
                    (*state).window.add((*state).write as usize),
                    dist as usize,
                );
            }
            copy -= dist;
            if copy != 0 {
                ptr::copy_nonoverlapping(
                    (*strm).next_out.sub(copy as usize),
                    (*state).window,
                    copy as usize,
                );
                (*state).write = copy;
                (*state).whave = (*state).wsize;
            } else {
                (*state).write += dist;
                if (*state).write == (*state).wsize {
                    (*state).write = 0;
                }
                if (*state).whave < (*state).wsize {
                    (*state).whave += dist;
                }
            }
        }
    }
}

unsafe fn zlib_inflate_sync_packet(strm: z_streamp) -> c_int {
    unsafe {
        if strm.is_null() || (*strm).state.is_null() {
            return Z_STREAM_ERROR;
        }
        let state = (*strm).state.cast::<inflate_state>();
        if (*state).mode == STORED && (*state).bits == 0 {
            (*state).mode = TYPE;
            return Z_OK as c_int;
        }
        Z_DATA_ERROR
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn zlib_inflate(strm: z_streamp, flush: c_int) -> c_int {
    unsafe {
        static ORDER: [usize; 19] = [
            16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
        ];
        if strm.is_null()
            || (*strm).state.is_null()
            || ((*strm).next_in.is_null() && (*strm).avail_in != 0)
        {
            return Z_STREAM_ERROR;
        }
        let state = (*strm).state.cast::<inflate_state>();
        if (*state).mode == TYPE {
            (*state).mode = TYPEDO;
        }
        let mut put = (*strm).next_out;
        let mut left = (*strm).avail_out as c_uint;
        let mut next = (*strm).next_in;
        let mut have = (*strm).avail_in as c_uint;
        let mut hold = (*state).hold;
        let mut bits = (*state).bits;
        let mut input = have;
        let mut out = left;
        let mut ret = Z_OK as c_int;

        macro_rules! restore {
            () => {{
                (*strm).next_out = put;
                (*strm).avail_out = left as c_ulong;
                (*strm).next_in = next;
                (*strm).avail_in = have as c_ulong;
                (*state).hold = hold;
                (*state).bits = bits;
            }};
        }
        macro_rules! load {
            () => {{
                put = (*strm).next_out;
                left = (*strm).avail_out as c_uint;
                next = (*strm).next_in;
                have = (*strm).avail_in as c_uint;
                hold = (*state).hold;
                bits = (*state).bits;
            }};
        }
        macro_rules! value {
            ($n:expr) => {
                (hold as c_uint) & ((1u32 << ($n as c_uint)) - 1)
            };
        }
        macro_rules! drop_bits {
            ($n:expr) => {{
                let n = $n as c_uint;
                hold >>= n;
                bits -= n;
            }};
        }
        macro_rules! init_bits {
            () => {{
                hold = 0;
                bits = 0;
            }};
        }
        macro_rules! byte_bits {
            () => {{
                hold >>= bits & 7;
                bits -= bits & 7;
            }};
        }
        macro_rules! bad {
            ($message:expr) => {{
                (*strm).msg = $message.as_ptr() as *mut c_char;
                (*state).mode = BAD;
            }};
        }

        'inf_leave: loop {
            // These macros live inside the labelled loop so their exit label
            // has the same lexical scope as C's goto inf_leave.
            macro_rules! pull_byte {
                () => {{
                    if have == 0 {
                        break 'inf_leave;
                    }
                    have -= 1;
                    hold = hold.wrapping_add((*next as c_ulong) << bits);
                    next = next.add(1);
                    bits += 8;
                }};
            }
            macro_rules! need_bits {
                ($n:expr) => {{
                    while bits < $n as c_uint {
                        pull_byte!();
                    }
                }};
            }

            match (*state).mode {
                HEAD => {
                    if (*state).wrap == 0 {
                        (*state).mode = TYPEDO;
                        continue;
                    }
                    need_bits!(16);
                    if (((value!(8) << 8) as c_ulong) + (hold >> 8)) % 31 != 0 {
                        bad!(b"incorrect header check\0");
                        continue;
                    }
                    if value!(4) != Z_DEFLATED as c_uint {
                        bad!(b"unknown compression method\0");
                        continue;
                    }
                    drop_bits!(4);
                    let len = value!(4) + 8;
                    if len > (*state).wbits {
                        bad!(b"invalid window size\0");
                        continue;
                    }
                    (*state).dmax = 1u32 << len;
                    (*state).check = zlib_adler32(0, ptr::null(), 0);
                    (*strm).adler = (*state).check;
                    (*state).mode = if hold & 0x200 != 0 { DICTID } else { TYPE };
                    init_bits!();
                }
                DICTID => {
                    need_bits!(32);
                    (*state).check = reverse(hold);
                    (*strm).adler = (*state).check;
                    init_bits!();
                    (*state).mode = DICT;
                }
                DICT => {
                    if (*state).havedict == 0 {
                        restore!();
                        return Z_NEED_DICT as c_int;
                    }
                    (*state).check = zlib_adler32(0, ptr::null(), 0);
                    (*strm).adler = (*state).check;
                    (*state).mode = TYPE;
                }
                TYPE | TYPEDO => {
                    if (*state).mode == TYPE && flush == Z_BLOCK as c_int {
                        break 'inf_leave;
                    }
                    if (*state).last != 0 {
                        byte_bits!();
                        (*state).mode = CHECK;
                        continue;
                    }
                    need_bits!(3);
                    (*state).last = value!(1) as c_int;
                    drop_bits!(1);
                    match value!(2) {
                        0 => (*state).mode = STORED,
                        1 => {
                            zlib_fixedtables(state);
                            (*state).mode = LEN;
                        }
                        2 => (*state).mode = TABLE,
                        _ => bad!(b"invalid block type\0"),
                    }
                    drop_bits!(2);
                }
                STORED => {
                    byte_bits!();
                    need_bits!(32);
                    if (hold & 0xffff) != ((hold >> 16) ^ 0xffff) {
                        bad!(b"invalid stored block lengths\0");
                        continue;
                    }
                    (*state).length = hold as c_uint & 0xffff;
                    init_bits!();
                    (*state).mode = COPY;
                }
                COPY => {
                    let mut copy = (*state).length;
                    if copy != 0 {
                        copy = copy.min(have).min(left);
                        if copy == 0 {
                            break 'inf_leave;
                        }
                        ptr::copy_nonoverlapping(next, put, copy as usize);
                        have -= copy;
                        next = next.add(copy as usize);
                        left -= copy;
                        put = put.add(copy as usize);
                        (*state).length -= copy;
                        continue;
                    }
                    (*state).mode = TYPE;
                }
                TABLE => {
                    need_bits!(14);
                    (*state).nlen = value!(5) + 257;
                    drop_bits!(5);
                    (*state).ndist = value!(5) + 1;
                    drop_bits!(5);
                    (*state).ncode = value!(4) + 4;
                    drop_bits!(4);
                    #[cfg(not(PKZIP_BUG_WORKAROUND))]
                    if (*state).nlen > 286 || (*state).ndist > 30 {
                        bad!(b"too many length or distance symbols\0");
                        continue;
                    }
                    (*state).have = 0;
                    (*state).mode = LENLENS;
                }
                LENLENS => {
                    while (*state).have < (*state).ncode {
                        need_bits!(3);
                        (*state).lens[ORDER[(*state).have as usize]] = value!(3) as u16;
                        (*state).have += 1;
                        drop_bits!(3);
                    }
                    while (*state).have < 19 {
                        (*state).lens[ORDER[(*state).have as usize]] = 0;
                        (*state).have += 1;
                    }
                    (*state).next = ptr::addr_of_mut!((*state).codes).cast();
                    (*state).lencode = (*state).next;
                    (*state).lenbits = 7;
                    ret = zlib_inflate_table(
                        CODES,
                        ptr::addr_of_mut!((*state).lens).cast(),
                        19,
                        ptr::addr_of_mut!((*state).next),
                        ptr::addr_of_mut!((*state).lenbits),
                        ptr::addr_of_mut!((*state).work).cast(),
                    );
                    if ret != 0 {
                        bad!(b"invalid code lengths set\0");
                        continue;
                    }
                    (*state).have = 0;
                    (*state).mode = CODELENS;
                }
                CODELENS => {
                    while (*state).have < (*state).nlen + (*state).ndist {
                        let mut this;
                        loop {
                            this = *(*state).lencode.add(value!((*state).lenbits) as usize);
                            if this.bits as c_uint <= bits {
                                break;
                            }
                            pull_byte!();
                        }
                        if this.val < 16 {
                            need_bits!(this.bits);
                            drop_bits!(this.bits);
                            (*state).lens[(*state).have as usize] = this.val;
                            (*state).have += 1;
                        } else {
                            let len;
                            let mut copy;
                            if this.val == 16 {
                                need_bits!(this.bits as c_uint + 2);
                                drop_bits!(this.bits);
                                if (*state).have == 0 {
                                    bad!(b"invalid bit length repeat\0");
                                    break;
                                }
                                len = (*state).lens[(*state).have as usize - 1];
                                copy = 3 + value!(2);
                                drop_bits!(2);
                            } else if this.val == 17 {
                                need_bits!(this.bits as c_uint + 3);
                                drop_bits!(this.bits);
                                len = 0;
                                copy = 3 + value!(3);
                                drop_bits!(3);
                            } else {
                                need_bits!(this.bits as c_uint + 7);
                                drop_bits!(this.bits);
                                len = 0;
                                copy = 11 + value!(7);
                                drop_bits!(7);
                            }
                            if (*state).have + copy > (*state).nlen + (*state).ndist {
                                bad!(b"invalid bit length repeat\0");
                                break;
                            }
                            while copy != 0 {
                                copy -= 1;
                                (*state).lens[(*state).have as usize] = len;
                                (*state).have += 1;
                            }
                        }
                    }
                    if (*state).mode == BAD {
                        continue;
                    }
                    (*state).next = ptr::addr_of_mut!((*state).codes).cast();
                    (*state).lencode = (*state).next;
                    (*state).lenbits = 9;
                    ret = zlib_inflate_table(
                        LENS,
                        ptr::addr_of_mut!((*state).lens).cast(),
                        (*state).nlen,
                        ptr::addr_of_mut!((*state).next),
                        ptr::addr_of_mut!((*state).lenbits),
                        ptr::addr_of_mut!((*state).work).cast(),
                    );
                    if ret != 0 {
                        bad!(b"invalid literal/lengths set\0");
                        continue;
                    }
                    (*state).distcode = (*state).next;
                    (*state).distbits = 6;
                    ret = zlib_inflate_table(
                        DISTS,
                        ptr::addr_of_mut!((*state).lens)
                            .cast::<u16>()
                            .add((*state).nlen as usize),
                        (*state).ndist,
                        ptr::addr_of_mut!((*state).next),
                        ptr::addr_of_mut!((*state).distbits),
                        ptr::addr_of_mut!((*state).work).cast(),
                    );
                    if ret != 0 {
                        bad!(b"invalid distances set\0");
                        continue;
                    }
                    (*state).mode = LEN;
                }
                LEN => {
                    if have >= 6 && left >= 258 {
                        restore!();
                        inflate_fast(strm, out);
                        load!();
                        continue;
                    }
                    let mut this;
                    loop {
                        this = *(*state).lencode.add(value!((*state).lenbits) as usize);
                        if this.bits as c_uint <= bits {
                            break;
                        }
                        pull_byte!();
                    }
                    if this.op != 0 && this.op & 0xf0 == 0 {
                        let last = this;
                        loop {
                            this = *(*state).lencode.add(
                                (last.val as c_uint
                                    + (value!(last.bits as c_uint + last.op as c_uint)
                                        >> last.bits)) as usize,
                            );
                            if last.bits as c_uint + this.bits as c_uint <= bits {
                                break;
                            }
                            pull_byte!();
                        }
                        drop_bits!(last.bits);
                    }
                    drop_bits!(this.bits);
                    (*state).length = this.val as c_uint;
                    if this.op == 0 {
                        (*state).mode = LIT;
                        continue;
                    }
                    if this.op & 32 != 0 {
                        (*state).mode = TYPE;
                        continue;
                    }
                    if this.op & 64 != 0 {
                        bad!(b"invalid literal/length code\0");
                        continue;
                    }
                    (*state).extra = this.op as c_uint & 15;
                    (*state).mode = LENEXT;
                }
                LENEXT => {
                    if (*state).extra != 0 {
                        need_bits!((*state).extra);
                        (*state).length += value!((*state).extra);
                        drop_bits!((*state).extra);
                    }
                    (*state).mode = DIST;
                }
                DIST => {
                    let mut this;
                    loop {
                        this = *(*state).distcode.add(value!((*state).distbits) as usize);
                        if this.bits as c_uint <= bits {
                            break;
                        }
                        pull_byte!();
                    }
                    if this.op & 0xf0 == 0 {
                        let last = this;
                        loop {
                            this = *(*state).distcode.add(
                                (last.val as c_uint
                                    + (value!(last.bits as c_uint + last.op as c_uint)
                                        >> last.bits)) as usize,
                            );
                            if last.bits as c_uint + this.bits as c_uint <= bits {
                                break;
                            }
                            pull_byte!();
                        }
                        drop_bits!(last.bits);
                    }
                    drop_bits!(this.bits);
                    if this.op & 64 != 0 {
                        bad!(b"invalid distance code\0");
                        continue;
                    }
                    (*state).offset = this.val as c_uint;
                    (*state).extra = this.op as c_uint & 15;
                    (*state).mode = DISTEXT;
                }
                DISTEXT => {
                    if (*state).extra != 0 {
                        need_bits!((*state).extra);
                        (*state).offset += value!((*state).extra);
                        drop_bits!((*state).extra);
                    }
                    #[cfg(INFLATE_STRICT)]
                    if (*state).offset > (*state).dmax {
                        bad!(b"invalid distance too far back\0");
                        continue;
                    }
                    if (*state).offset > (*state).whave.wrapping_add(out).wrapping_sub(left) {
                        bad!(b"invalid distance too far back\0");
                        continue;
                    }
                    (*state).mode = MATCH;
                }
                MATCH => {
                    if left == 0 {
                        break 'inf_leave;
                    }
                    let mut copy = out - left;
                    let mut from;
                    if (*state).offset > copy {
                        copy = (*state).offset - copy;
                        if copy > (*state).write {
                            copy -= (*state).write;
                            from = (*state).window.add(((*state).wsize - copy) as usize);
                        } else {
                            from = (*state).window.add(((*state).write - copy) as usize);
                        }
                        copy = copy.min((*state).length);
                    } else {
                        from = put.sub((*state).offset as usize);
                        copy = (*state).length;
                    }
                    copy = copy.min(left);
                    left -= copy;
                    (*state).length -= copy;
                    // Forward byte copies are essential for overlapping matches.
                    loop {
                        *put = *from;
                        put = put.add(1);
                        from = from.add(1);
                        copy -= 1;
                        if copy == 0 {
                            break;
                        }
                    }
                    if (*state).length == 0 {
                        (*state).mode = LEN;
                    }
                }
                LIT => {
                    if left == 0 {
                        break 'inf_leave;
                    }
                    *put = (*state).length as u8;
                    put = put.add(1);
                    left -= 1;
                    (*state).mode = LEN;
                }
                CHECK => {
                    if (*state).wrap != 0 {
                        need_bits!(32);
                        out -= left;
                        (*strm).total_out = (*strm).total_out.wrapping_add(out as c_ulong);
                        (*state).total = (*state).total.wrapping_add(out as c_ulong);
                        if out != 0 {
                            (*state).check =
                                zlib_adler32((*state).check, put.sub(out as usize), out);
                            (*strm).adler = (*state).check;
                        }
                        out = left;
                        if reverse(hold) != (*state).check {
                            bad!(b"incorrect data check\0");
                            continue;
                        }
                        init_bits!();
                    }
                    (*state).mode = DONE;
                }
                DONE => {
                    ret = Z_STREAM_END as c_int;
                    break 'inf_leave;
                }
                BAD => {
                    ret = Z_DATA_ERROR;
                    break 'inf_leave;
                }
                MEM => return Z_MEM_ERROR,
                // FLAGS..HCRC and LENGTH are declared by inflate.h but have
                // no case in the native implementation; SYNC also errors.
                _ => return Z_STREAM_ERROR,
            }
        }
        restore!();
        if (*state).wsize != 0 || ((*state).mode < CHECK && out as c_ulong != (*strm).avail_out) {
            zlib_updatewindow(strm, out);
        }
        input = (input as c_ulong).wrapping_sub((*strm).avail_in) as c_uint;
        out = (out as c_ulong).wrapping_sub((*strm).avail_out) as c_uint;
        (*strm).total_in = (*strm).total_in.wrapping_add(input as c_ulong);
        (*strm).total_out = (*strm).total_out.wrapping_add(out as c_ulong);
        (*state).total = (*state).total.wrapping_add(out as c_ulong);
        if (*state).wrap != 0 && out != 0 {
            (*state).check = zlib_adler32((*state).check, (*strm).next_out.sub(out as usize), out);
            (*strm).adler = (*state).check;
        }
        (*strm).data_type = ((*state).bits
            + if (*state).last != 0 { 64 } else { 0 }
            + if (*state).mode == TYPE { 128 } else { 0 }) as c_int;
        if flush == Z_PACKET_FLUSH as c_int
            && ret == Z_OK as c_int
            && (*strm).avail_out != 0
            && (*strm).avail_in == 0
        {
            return zlib_inflate_sync_packet(strm);
        }
        if ((input == 0 && out == 0) || flush == Z_FINISH as c_int) && ret == Z_OK as c_int {
            ret = Z_BUF_ERROR;
        }
        ret
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn zlib_inflateEnd(strm: z_streamp) -> c_int {
    unsafe {
        if strm.is_null() || (*strm).state.is_null() {
            Z_STREAM_ERROR
        } else {
            Z_OK as c_int
        }
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn zlib_inflateIncomp(z: *mut z_stream) -> c_int {
    unsafe {
        let state = (*z).state.cast::<inflate_state>();
        let saved_no = (*z).next_out;
        let saved_ao = (*z).avail_out as c_uint;
        if (*state).mode != TYPE && (*state).mode != HEAD {
            return Z_DATA_ERROR;
        }
        (*z).avail_out = 0;
        (*z).next_out = (*z).next_in.cast_mut().add((*z).avail_in as usize);
        zlib_updatewindow(z, (*z).avail_in as c_uint);
        (*z).avail_out = saved_ao as c_ulong;
        (*z).next_out = saved_no;
        (*state).check = zlib_adler32((*state).check, (*z).next_in, (*z).avail_in as c_uint);
        (*z).adler = (*state).check;
        (*z).total_out = (*z).total_out.wrapping_add((*z).avail_in);
        (*z).total_in = (*z).total_in.wrapping_add((*z).avail_in);
        (*z).next_in = (*z).next_in.add((*z).avail_in as usize);
        (*state).total = (*state).total.wrapping_add((*z).avail_in);
        (*z).avail_in = 0;
        Z_OK as c_int
    }
}
