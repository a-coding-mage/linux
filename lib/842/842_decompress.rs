// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * 842 Software Decompression
 *
 * Copyright (C) 2015 Dan Streetman, IBM Corp
 *
 * See 842.h for details of the 842 compressed format.
 */

// Dependencies supplied by 842.h and 842_debugfs.h are intentionally external.

const I2_FIFO_SIZE: usize = 2 * (1usize << I2_BITS);
const I4_FIFO_SIZE: usize = 4 * (1usize << I4_BITS);
const I8_FIFO_SIZE: usize = 8 * (1usize << I8_BITS);

static mut DECOMP_OPS: [[u8; 4]; OPS_MAX] = [
    [D8, N0, N0, N0], [D4, D2, I2, N0], [D4, I2, D2, N0],
    [D4, I2, I2, N0], [D4, I4, N0, N0], [D2, I2, D4, N0],
    [D2, I2, D2, I2], [D2, I2, I2, D2], [D2, I2, I2, I2],
    [D2, I2, I4, N0], [I2, D2, D4, N0], [I2, D4, I2, N0],
    [I2, D2, I2, D2], [I2, D2, I2, I2], [I2, D2, I4, N0],
    [I2, I2, D4, N0], [I2, I2, D2, I2], [I2, I2, I2, D2],
    [I2, I2, I2, I2], [I2, I2, I4, N0], [I4, D4, N0, N0],
    [I4, D2, I2, N0], [I4, I2, D2, N0], [I4, I2, I2, N0],
    [I4, I4, N0, N0], [I8, N0, N0, N0],
];

#[repr(C)]
struct Sw842Param {
    in_: *mut u8,
    bit: u8,
    ilen: u64,
    out: *mut u8,
    ostart: *mut u8,
    olen: u64,
}

unsafe fn read_be(p: *const u8, n: u8) -> u64 {
    let mut v = 0u64;
    for i in 0..n { v = (v << 8) | *p.add(i as usize) as u64; }
    v
}

unsafe fn next_bits(p: &mut Sw842Param, d: *mut u64, n: u8) -> i32 {
    let bits = p.bit + n;
    if n > 64 { return -EINVAL; }
    if bits > 64 { return split_next_bits(p, d, n, 32); }
    if p.ilen < 8 && bits > 32 && bits <= 56 { return split_next_bits(p, d, n, 16); }
    if p.ilen < 4 && bits > 16 && bits <= 24 { return split_next_bits(p, d, n, 8); }
    if ((bits as u64 + 7) / 8) > p.ilen { return -EOVERFLOW; }
    let v = read_be(p.in_, ((bits as usize + 7) / 8) as u8);
    *d = (v >> ((((bits as usize + 7) / 8) * 8) - bits as usize)) & ((1u64 << n) - 1);
    p.bit += n;
    if p.bit > 7 { p.in_ = p.in_.add((p.bit / 8) as usize); p.ilen -= (p.bit / 8) as u64; p.bit %= 8; }
    0
}

unsafe fn split_next_bits(p: &mut Sw842Param, d: *mut u64, n: u8, s: u8) -> i32 {
    if n <= s { return -EINVAL; }
    let mut tmp = 0u64;
    let mut ret = next_bits(p, &mut tmp, n - s); if ret != 0 { return ret; }
    ret = next_bits(p, d, s); if ret != 0 { return ret; }
    *d |= tmp << s; 0
}

unsafe fn do_data(p: &mut Sw842Param, n: u8) -> i32 {
    if n as u64 > p.olen { return -ENOSPC; }
    let mut v = 0u64; let ret = next_bits(p, &mut v, n * 8); if ret != 0 { return ret; }
    for i in 0..n as usize { *p.out.add(i) = (v >> ((n as usize - 1 - i) * 8)) as u8; }
    p.out = p.out.add(n as usize); p.olen -= n as u64; 0
}

unsafe fn do_index(p: &mut Sw842Param, size: u8, bits: u8, fsize: u64) -> i32 {
    let mut index = 0u64; let ret = next_bits(p, &mut index, bits); if ret != 0 { return ret; }
    let total = p.out.offset_from(p.ostart) as u64 & !7; let mut offset = index * size as u64;
    if total > fsize { let section = total / fsize * fsize; let pos = total - section; let section = if offset >= pos { section - fsize } else { section }; offset += section; }
    if offset + size as u64 > total { return -EINVAL; }
    std::ptr::copy_nonoverlapping(p.ostart.add(offset as usize), p.out, size as usize);
    p.out = p.out.add(size as usize); p.olen -= size as u64; 0
}

unsafe fn dispatch_index(p: &mut Sw842Param, n: u8) -> i32 {
    match n { 2 => do_index(p, 2, I2_BITS, I2_FIFO_SIZE as u64), 4 => do_index(p, 4, I4_BITS, I4_FIFO_SIZE as u64), 8 => do_index(p, 8, I8_BITS, I8_FIFO_SIZE as u64), _ => -EINVAL }
}

unsafe fn do_op(p: &mut Sw842Param, o: u64) -> i32 {
    if o >= OPS_MAX as u64 { return -EINVAL; }
    for i in 0..4 { let op = DECOMP_OPS[o as usize][i]; let ret = match op & OP_ACTION { OP_ACTION_DATA => do_data(p, op & OP_AMOUNT), OP_ACTION_INDEX => dispatch_index(p, op & OP_AMOUNT), OP_ACTION_NOOP => 0, _ => -EINVAL }; if ret != 0 { return ret; } }
    0
}

pub unsafe fn sw842_decompress(input: *const u8, ilen: u32, output: *mut u8, olen: *mut u32) -> i32 {
    let mut p = Sw842Param { in_: input as *mut u8, bit: 0, ilen: ilen as u64, out: output, ostart: output, olen: *olen as u64 };
    let total = p.olen; *olen = 0;
    loop {
        let mut op = 0u64; let mut ret = next_bits(&mut p, &mut op, OP_BITS); if ret != 0 { return ret; }
        match op {
            OP_REPEAT => { let mut rep=0u64; ret=next_bits(&mut p,&mut rep,REPEAT_BITS); if ret!=0{return ret;} if p.out==output{return -EINVAL;} rep+=1; if rep*8>p.olen{return -ENOSPC;} while rep>0 { std::ptr::copy_nonoverlapping(p.out.sub(8),p.out,8); p.out=p.out.add(8);p.olen-=8;rep-=1; } }
            OP_ZEROS => { if p.olen<8{return -ENOSPC;} std::ptr::write_bytes(p.out,0,8);p.out=p.out.add(8);p.olen-=8; }
            OP_SHORT_DATA => { let mut bytes=0u64;ret=next_bits(&mut p,&mut bytes,SHORT_DATA_BITS);if ret!=0{return ret;}if bytes==0||bytes>SHORT_DATA_BITS_MAX{return -EINVAL;}while bytes>0{let mut tmp=0;ret=next_bits(&mut p,&mut tmp,8);if ret!=0{return ret;}*p.out=tmp as u8;p.out=p.out.add(1);p.olen-=1;bytes-=1;} }
            OP_END => {}, _ => { ret=do_op(&mut p,op);if ret!=0{return ret;} }
        }
        if op == OP_END { break; }
    }
    let mut crc=0; let ret=next_bits(&mut p,&mut crc,CRC_BITS);if ret!=0{return ret;}
    if crc != crc32_be(0, output, (total-p.olen) as usize) as u64 { return -EINVAL; }
    if total-p.olen > u32::MAX as u64 { return -ENOSPC; } *olen=(total-p.olen) as u32; 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
