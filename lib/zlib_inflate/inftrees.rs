/* inftrees.c -- generate Huffman trees for efficient decoding
 * Copyright (C) 1995-2005 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */

// Dependencies supplied by the surrounding translation unit: codetype, code,
// CODES, LENS, DISTS, ENOUGH, and MAXD.

const MAXBITS: usize = 15;

/*
   Build a set of tables to decode the provided canonical Huffman code.
   The code lengths are lens[0..codes-1].  The result starts at *table,
   whose indices are 0..2^bits-1.  work is a writable array of at least
   lens shorts, which is used as a work area.  type is the type of code
   to be generated, CODES, LENS, or DISTS.  On return, zero is success,
   -1 is an invalid code, and +1 means that ENOUGH isn't enough.  table
   on return points to the next available entry's address.  bits is the
   requested root table index bits, and on return it is the actual root
   table index bits.  It will differ if the request is greater than the
   longest code or if it is less than the shortest code.
 */
pub unsafe fn zlib_inflate_table(
    type_: codetype,
    lens: *mut u16,
    codes: u32,
    table: *mut *mut code,
    bits: *mut u32,
    work: *mut u16,
) -> i32 {
    let mut len: u32;
    let mut sym: u32;
    let mut min: u32;
    let mut max: u32;
    let mut root: u32;
    let mut curr: u32;
    let mut drop: u32;
    let mut left: i32;
    let mut used: u32;
    let mut huff: u32;
    let mut incr: u32;
    let mut fill: u32;
    let mut low: u32;
    let mut mask: u32;
    let mut this: code;
    let mut next: *mut code;
    let mut base: *const u16;
    let mut extra: *const u16;
    let mut end: i32;
    let mut count = [0u16; MAXBITS + 1];
    let mut offs = [0u16; MAXBITS + 1];
    static LBASE: [u16; 31] = [3,4,5,6,7,8,9,10,11,13,15,17,19,23,27,31,35,43,51,59,67,83,99,115,131,163,195,227,258,0,0];
    static LEXT: [u16; 31] = [16,16,16,16,16,16,16,16,17,17,17,17,18,18,18,18,19,19,19,19,20,20,20,20,21,21,21,21,16,201,196];
    static DBASE: [u16; 32] = [1,2,3,4,5,7,9,13,17,25,33,49,65,97,129,193,257,385,513,769,1025,1537,2049,3073,4097,6145,8193,12289,16385,24577,0,0];
    static DEXT: [u16; 32] = [16,16,16,16,17,17,18,18,19,19,20,20,21,21,22,22,23,23,24,24,25,25,26,26,27,27,28,28,29,29,64,64];

    for i in 0..=MAXBITS { count[i] = 0; }
    for i in 0..codes { count[*lens.add(i as usize) as usize] += 1; }

    root = *bits;
    max = 0;
    for i in (1..=MAXBITS as u32).rev() { if count[i as usize] != 0 { max = i; break; } }
    if root > max { root = max; }
    if max == 0 {
        this.op = 64; this.bits = 1; this.val = 0;
        let p = *table; *p = this; *table = p.add(1);
        let p = *table; *p = this; *table = p.add(1);
        *bits = 1; return 0;
    }
    min = 1; while min < MAXBITS as u32 && count[min as usize] == 0 { min += 1; }
    if root < min { root = min; }

    left = 1;
    for i in 1..=MAXBITS { left = (left << 1) - count[i] as i32; if left < 0 { return -1; } }
    if left > 0 && (type_ == CODES || max != 1) { return -1; }

    offs[1] = 0;
    for i in 1..MAXBITS { offs[i + 1] = offs[i] + count[i]; }
    for i in 0..codes {
        let l = *lens.add(i as usize) as usize;
        if l != 0 { let at = offs[l] as usize; *work.add(at) = i as u16; offs[l] += 1; }
    }

    match type_ {
        CODES => { base = work; extra = work; end = 19; }
        LENS => { base = LBASE.as_ptr().sub(257); extra = LEXT.as_ptr().sub(257); end = 256; }
        _ => { base = DBASE.as_ptr(); extra = DEXT.as_ptr(); end = -1; }
    }
    huff = 0; sym = 0; len = min; next = *table; curr = root; drop = 0;
    low = u32::MAX; used = 1u32 << root; mask = used - 1;
    if type_ == LENS && used >= ENOUGH - MAXD { return 1; }

    loop {
        this.bits = (len - drop) as u8;
        let symbol = *work.add(sym as usize);
        if symbol as i32 < end { this.op = 0; this.val = symbol; }
        else if symbol as i32 > end { this.op = *extra.add(symbol as usize) as u8; this.val = *base.add(symbol as usize); }
        else { this.op = 96; this.val = 0; }
        incr = 1 << (len - drop); fill = 1 << curr; min = fill;
        loop { fill -= incr; *next.add((huff >> drop) as usize + fill as usize) = this; if fill == 0 { break; } }
        incr = 1 << (len - 1); while huff & incr != 0 { incr >>= 1; }
        if incr != 0 { huff &= incr - 1; huff += incr; } else { huff = 0; }
        sym += 1; count[len as usize] -= 1;
        if count[len as usize] == 0 { if len == max { break; } len = *lens.add(*work.add(sym as usize) as usize) as u32; }
        if len > root && (huff & mask) != low {
            if drop == 0 { drop = root; }
            next = next.add(min as usize); curr = len - drop; left = 1 << curr;
            while curr + drop < max { left -= count[(curr + drop) as usize] as i32; if left <= 0 { break; } curr += 1; left <<= 1; }
            used += 1 << curr; if type_ == LENS && used >= ENOUGH - MAXD { return 1; }
            low = huff & mask; (*table).add(low as usize).write(code { op: curr as u8, bits: root as u8, val: (next.offset_from(*table)) as u16 });
        }
    }
    this.op = 64; this.bits = (len - drop) as u8; this.val = 0;
    while huff != 0 {
        if drop != 0 && (huff & mask) != low { drop = 0; len = root; next = *table; this.bits = len as u8; }
        *next.add((huff >> drop) as usize) = this;
        incr = 1 << (len - 1); while huff & incr != 0 { incr >>= 1; }
        if incr != 0 { huff &= incr - 1; huff += incr; } else { huff = 0; }
    }
    *table = (*table).add(used as usize); *bits = root; 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
