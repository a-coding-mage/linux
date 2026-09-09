/* inffast.c -- fast decoding
 * Copyright (C) 1995-2004 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */

/* Dependencies are supplied by the surrounding zlib translation unit. */

#[repr(C)]
union Uu {
    us: u16,
    b: [u8; 2],
}

#[inline]
unsafe fn get_unaligned16(p: *const u16) -> u16 {
    let b = p as *const u8;
    let mut mm = Uu { us: 0 };
    mm.b[0] = *b;
    mm.b[1] = *b.add(1);
    mm.us
}

/* The following names and structures are declarations supplied by zlib. */
#[allow(non_camel_case_types)]
pub type z_streamp = *mut z_stream;
#[allow(non_camel_case_types)]
pub type code = Code;

#[repr(C)]
pub struct Code { pub op: u8, pub bits: u8, pub val: u16 }

#[repr(C)]
pub struct z_stream {
    pub next_in: *const u8, pub avail_in: u32,
    pub next_out: *mut u8, pub avail_out: u32,
    pub msg: *mut i8, pub state: *mut inflate_state,
}

#[repr(C)]
pub struct inflate_state {
    pub mode: u32, pub dmax: u32, pub wsize: u32, pub whave: u32, pub write: u32,
    pub window: *mut u8, pub hold: u64, pub bits: u32,
    pub lencode: *const Code, pub distcode: *const Code,
    pub lenbits: u32, pub distbits: u32,
}

pub const BAD: u32 = 30;
pub const TYPE: u32 = 161;

#[allow(clippy::missing_safety_doc)]
pub unsafe fn inflate_fast(strm: z_streamp, start: u32) {
    let state = (*strm).state;
    let mut input = (*strm).next_in;
    let last = input.add(((*strm).avail_in - 5) as usize);
    let mut out = (*strm).next_out;
    let beg = out.sub((start - (*strm).avail_out) as usize);
    let end = out.add(((*strm).avail_out - 257) as usize);
    let wsize = (*state).wsize as usize;
    let whave = (*state).whave as usize;
    let write = (*state).write as usize;
    let window = (*state).window;
    let mut hold = (*state).hold;
    let mut bits = (*state).bits;
    let lcode = (*state).lencode;
    let dcode = (*state).distcode;
    let lmask = (1u32 << (*state).lenbits) - 1;
    let dmask = (1u32 << (*state).distbits) - 1;
    let mut len: usize;

    'decode: loop {
        if bits < 15 {
            hold += (*input as u64) << bits; input = input.add(1); bits += 8;
            hold += (*input as u64) << bits; input = input.add(1); bits += 8;
        }
        let mut this = *lcode.add((hold & lmask as u64) as usize);
        'dolen: loop {
            let mut op = this.bits as u32;
            hold >>= op; bits -= op;
            op = this.op as u32;
            if op == 0 {
                *out = this.val as u8; out = out.add(1);
            } else if op & 16 != 0 {
                len = this.val as usize; op &= 15;
                if op != 0 {
                    if bits < op { hold += (*input as u64) << bits; input = input.add(1); bits += 8; }
                    len += (hold as usize) & ((1usize << op) - 1); hold >>= op; bits -= op;
                }
                if bits < 15 {
                    hold += (*input as u64) << bits; input = input.add(1); bits += 8;
                    hold += (*input as u64) << bits; input = input.add(1); bits += 8;
                }
                this = *dcode.add((hold & dmask as u64) as usize);
                'dodist: loop {
                    op = this.bits as u32; hold >>= op; bits -= op; op = this.op as u32;
                    if op & 16 != 0 {
                        let mut dist = this.val as usize; op &= 15;
                        if bits < op {
                            hold += (*input as u64) << bits; input = input.add(1); bits += 8;
                            if bits < op { hold += (*input as u64) << bits; input = input.add(1); bits += 8; }
                        }
                        dist += (hold as usize) & ((1usize << op) - 1); hold >>= op; bits -= op;
                        if dist > out.offset_from(beg) as usize {
                            let mut back = dist - out.offset_from(beg) as usize;
                            if back > whave { (*strm).msg = b"invalid distance too far back\0".as_ptr() as *mut i8; (*state).mode = BAD; break 'decode; }
                            let mut from = window;
                            if write == 0 {
                                from = from.add(wsize - back);
                                if back < len { len -= back; while back != 0 { *out = *from; out=out.add(1); from=from.add(1); back-=1; } from=out.sub(dist); }
                            } else if write < back {
                                from = from.add(wsize + write - back); back -= write;
                                if back < len { len -= back; while back != 0 { *out=*from; out=out.add(1); from=from.add(1); back-=1; } from=window; if write < len { back=write; len-=back; while back!=0 { *out=*from; out=out.add(1); from=from.add(1); back-=1; } from=out.sub(dist); } }
                            } else { from=from.add(write-back); if back < len { len-=back; while back!=0 { *out=*from; out=out.add(1); from=from.add(1); back-=1; } from=out.sub(dist); } }
                            while len > 2 { *out=*from; *out.add(1)=*from.add(1); *out.add(2)=*from.add(2); out=out.add(3); from=from.add(3); len-=3; }
                            if len != 0 { *out=*from; out=out.add(1); if len>1 {*out=*from.add(1); out=out.add(1);} }
                        } else {
                            let mut from=out.sub(dist); while len != 0 { *out=*from; out=out.add(1); from=from.add(1); len-=1; }
                        }
                    } else if op & 64 == 0 { this=*dcode.add(this.val as usize + ((hold & ((1u64<<op)-1)) as usize)); continue 'dodist; }
                    else { (*strm).msg=b"invalid distance code\0".as_ptr() as *mut i8; (*state).mode=BAD; break 'decode; }
                    break;
                }
            } else if op & 64 == 0 { this=*lcode.add(this.val as usize + ((hold & ((1u64<<op)-1)) as usize)); continue 'dolen; }
            else if op & 32 != 0 { (*state).mode=TYPE; break 'decode; }
            else { (*strm).msg=b"invalid literal/length code\0".as_ptr() as *mut i8; (*state).mode=BAD; break 'decode; }
        }
        if !(input < last && out < end) { break; }
    }
    len = (bits >> 3) as usize; input=input.sub(len); bits-= (len as u32)<<3; hold &= (1u64<<bits)-1;
    (*strm).next_in=input; (*strm).next_out=out;
    (*strm).avail_in=if input<last {5+(last.offset_from(input) as u32)} else {5-(input.offset_from(last) as u32)};
    (*strm).avail_out=if out<end {257+(end.offset_from(out) as u32)} else {257-(out.offset_from(end) as u32)};
    (*state).hold=hold; (*state).bits=bits;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
