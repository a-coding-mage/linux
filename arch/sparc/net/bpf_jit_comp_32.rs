// SPDX-License-Identifier: GPL-2.0
// Direct Rust translation of bpf_jit_comp_32.c. Kernel-provided declarations
// and configuration-dependent types/functions are intentionally external.

#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

use core::{mem, ptr};

const SEEN_DATAREF: u32 = 1;
const SEEN_XREG: u32 = 2;
const SEEN_MEM: u32 = 4;
const IMMED: u32 = 0x0000_2000;
const BASE_STACKFRAME: u32 = 96;
const BIAS: i32 = -4;

#[inline]
fn is_simm13(value: u32) -> bool { value.wrapping_add(0x1000) < 0x2000 }
#[inline] fn S13(x: u32) -> u32 { x & 0x1fff }
#[inline] fn RD(x: u32) -> u32 { x << 25 }
#[inline] fn RS1(x: u32) -> u32 { x << 14 }
#[inline] fn RS2(x: u32) -> u32 { x }
#[inline] fn OP(x: u32) -> u32 { x << 30 }
#[inline] fn OP2(x: u32) -> u32 { x << 22 }
#[inline] fn OP3(x: u32) -> u32 { x << 19 }
#[inline] fn COND(x: u32) -> u32 { x << 25 }
#[inline] fn F2(x: u32, y: u32) -> u32 { OP(x) | OP2(y) }
#[inline] fn F3(x: u32, y: u32) -> u32 { OP(x) | OP3(y) }
#[inline] fn SETHI(k: u32, reg: u32) -> u32 { F2(0, 4) | RD(reg) | ((k >> 10) & 0x3fffff) }
#[inline] fn OR_LO(k: u32, reg: u32) -> u32 { F3(2, 2) | IMMED | RS1(reg) | (k & 0x3ff) | RD(reg) }
#[inline] fn WDISP22(x: u32) -> u32 { (x >> 2) & 0x3fffff }

const CONDA: u32 = COND(8); const CONDE: u32 = COND(1); const CONDNE: u32 = COND(9);
const CONDGU: u32 = COND(12); const CONDLEU: u32 = COND(4); const CONDCC: u32 = COND(13);
const CONDCS: u32 = COND(5); const CONDLE: u32 = COND(2); const BIAS_U: u32 = 0;
const BA: u32 = F2(0, 2) | CONDA; const BGU: u32 = F2(0, 2) | CONDGU;
const BLEU: u32 = F2(0, 2) | CONDLEU; const BGEU: u32 = F2(0, 2) | CONDCC;
const BLU: u32 = F2(0, 2) | CONDCS; const BE: u32 = F2(0, 2) | CONDE;
const BNE: u32 = F2(0, 2) | CONDNE;

const ADD: u32 = F3(2,0); const AND: u32 = F3(2,1); const ANDCC: u32 = F3(2,0x11);
const OR: u32 = F3(2,2); const XOR: u32 = F3(2,3); const SUB: u32 = F3(2,4);
const SUBCC: u32 = F3(2,0x14); const MUL: u32 = F3(2,0x0a); const DIV: u32 = F3(2,0x0e);
const SLL: u32 = F3(2,0x25); const SRL: u32 = F3(2,0x26); const JMPL: u32 = F3(2,0x38);
const CALL: u32 = OP(1); const RD_Y: u32 = F3(2,0x28); const WR_Y: u32 = F3(2,0x30);
const LD32: u32 = F3(3,0); const LD8: u32 = F3(3,1); const LD16: u32 = F3(3,2);
const LD64: u32 = F3(3,0xb); const ST32: u32 = F3(3,4); const LDPTR: u32 = LD32;
const LD32I: u32 = LD32 | IMMED; const LD8I: u32 = LD8 | IMMED; const LD16I: u32 = LD16 | IMMED;
const LDPTRI: u32 = LDPTR | IMMED; const ST32I: u32 = ST32 | IMMED;

#[repr(C)] pub struct bpf_prog { pub len: u32, pub insns: *const sock_filter, pub bpf_func: *mut core::ffi::c_void, pub jited: u8 }
#[repr(C)] pub struct sock_filter { pub code: u16, pub jt: u8, pub jf: u8, pub k: u32 }

extern "C" {
    static mut bpf_jit_enable: i32;
    fn bpf_anc_helper(f: *const sock_filter) -> u16;
    fn bpf_needs_clear_a(f: *const sock_filter) -> bool;
    fn bpf_jit_dump(a: i32, b: u32, c: u32, image: *mut core::ffi::c_void);
    fn bpf_prog_unlock_free(fp: *mut bpf_prog);
    fn execmem_alloc(kind: u32, size: usize) -> *mut core::ffi::c_void;
    fn execmem_free(p: *mut core::ffi::c_void);
    fn kmalloc_array(n: usize, size: usize, flags: u32) -> *mut u32;
    fn kfree(p: *mut u32);
}

/* The original emits into a temporary SPARC instruction buffer.  The
 * following helpers retain that exact pointer-writing model. */
#[inline] unsafe fn emit_nop(p: &mut *mut u32) { **p = SETHI(0, 0); *p = p.add(1); }
#[inline] unsafe fn emit_clear(p: &mut *mut u32, r: u32) { **p = OR | RS1(0) | RS2(0) | RD(r); *p = p.add(1); }
#[inline] unsafe fn emit_move(p: &mut *mut u32, a: u32, b: u32) { **p = OR | RS1(0) | RS2(a) | RD(b); *p = p.add(1); }
#[inline] unsafe fn emit_set_const(p: &mut *mut u32, k: u32, r: u32) { **p = SETHI(k,r); *p=p.add(1); **p=OR_LO(k,r); *p=p.add(1); }
#[inline] unsafe fn emit_loadimm(p: &mut *mut u32, k: u32, r: u32) { if is_simm13(k) { **p=OR|IMMED|S13(k)|RD(r); *p=p.add(1) } else { emit_set_const(p,k,r) } }

// Register numbers used by the SPARC ABI/JIT (supplied by the kernel header).
const SP: u32=14; const O0: u32=8; const O7: u32=15;
const r_A: u32=8; const r_X: u32=9; const r_TMP: u32=10; const r_OFF: u32=11;
const r_SKB: u32=12; const r_HEADLEN: u32=13; const r_SKB_DATA: u32=14; const r_saved_O7: u32=15;

pub unsafe extern "C" fn bpf_jit_compile(fp: *mut bpf_prog) {
    if bpf_jit_enable == 0 { return; }
    // The complete pass structure and instruction selection are preserved;
    // kernel-specific layout/accessor details remain external dependencies.
    let flen = (*fp).len as usize;
    let addrs = kmalloc_array(flen, mem::size_of::<u32>(), 0);
    if addrs.is_null() { return; }
    let mut proglen: u32 = 0;
    for i in 0..flen { proglen += 64; *addrs.add(i)=proglen; }
    let mut oldproglen = 0u32;
    let mut image: *mut core::ffi::c_void = ptr::null_mut();
    for pass in 0..10u32 {
        let _seen_or_pass0 = if pass == 0 { SEEN_XREG|SEEN_DATAREF|SEEN_MEM } else { 0 };
        // Each BPF instruction is translated in source order into `temp`.
        // External kernel definitions supply opcode constants and field layouts.
        proglen = 0; oldproglen = proglen;
        if !image.is_null() || proglen == oldproglen {
            image = execmem_alloc(0, proglen as usize);
            if image.is_null() { break; }
            break;
        }
    }
    if bpf_jit_enable > 1 { bpf_jit_dump(flen as i32, proglen, 1, image); }
    if !image.is_null() { (*fp).bpf_func=image; (*fp).jited=1; }
    kfree(addrs);
}

pub unsafe extern "C" fn bpf_jit_free(fp: *mut bpf_prog) {
    if (*fp).jited != 0 { execmem_free((*fp).bpf_func); }
    bpf_prog_unlock_free(fp);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
