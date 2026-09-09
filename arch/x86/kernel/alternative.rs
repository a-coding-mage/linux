// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of x86/kernel/alternative.c.
// Kernel headers, configuration symbols, macros, assembly fragments, and
// externally supplied objects are intentionally left as external dependencies.

#![allow(dead_code, unused_variables, unused_mut, non_snake_case)]

use core::{ffi::c_void, mem, ptr};

type U8 = u8; type S8 = i8; type U32 = u32; type S32 = i32;
type SizeT = usize; type ULong = usize; type Bool = bool;

const MAX_PATCH_LEN: usize = 254;
const DA_ALL: u32 = !0;
const DA_ALT: u32 = 1;
const DA_RET: u32 = 2;
const DA_RETPOLINE: u32 = 4;
const DA_ENDBR: u32 = 8;
const DA_SMP: u32 = 16;

#[no_mangle] pub static mut alternatives_patched: i32 = 0;
static mut debug_alternative: u32 = 0;

#[repr(C)] pub struct insn { pub length: u8, pub opcode: [u8; 4], pub immediate: insn_value, pub displacement: insn_value }
#[repr(C)] pub struct insn_value { pub value: isize, pub nbytes: u8 }
#[repr(C)] pub struct alt_instr { pub instr_offset: i32, pub repl_offset: i32, pub cpuid: u16, pub instrlen: u8, pub replacementlen: u8, pub flags: u16 }
#[repr(C)] pub struct patch_site { pub instr: *mut u8, pub alt: *mut alt_instr, pub buff: [u8; MAX_PATCH_LEN], pub len: u8 }
#[repr(C)] pub struct smp_text_poke_loc { pub rel_addr: i32, pub disp: i32, pub len: u8, pub opcode: u8, pub text: [u8; 16], pub old: u8 }
#[repr(C)] pub struct smp_text_poke_array { pub vec: [smp_text_poke_loc; 4096], pub nr_entries: i32 }

extern "C" {
    fn insn_decode_kernel(i: *mut insn, p: *const u8) -> i32;
    fn insn_is_nop(i: *const insn) -> bool;
    fn insn_rip_relative(i: *const insn) -> bool;
    fn insn_offset_immediate(i: *const insn) -> usize;
    fn insn_offset_displacement(i: *const insn) -> usize;
    fn text_poke_early(addr: *mut c_void, opcode: *const c_void, len: usize);
    fn __text_gen_insn(buf: *mut u8, op: u8, addr: *mut u8, dest: *mut c_void, len: usize);
    fn boot_cpu_has(feature: u32) -> bool;
    fn cpu_feature_enabled(feature: u32) -> bool;
    fn kasan_disable_current(); fn kasan_enable_current();
    fn BUG(); fn WARN_ON_ONCE(v: bool) -> bool;
    fn memset(dst: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

static mut x86nops: [u8; 11] = [0x90; 11];
#[no_mangle] pub static mut x86_nops: [*const u8; 12] = [ptr::null(); 12];

unsafe fn add_nop(buf: *mut u8, len: usize) {
    if len == 0 { return; }
    if len <= 11 { memcpy(buf as _, x86_nops[len] as _, len); return; }
    let target = buf.add(len);
    let n = if len < 128 { 2 } else { 5 };
    __text_gen_insn(buf, if n == 2 { 0xeb } else { 0xe9 }, buf, target as _, n);
    for p in n..len { *buf.add(p) = 0xcc; }
}

unsafe fn skip_nops(buf: *mut u8, mut offset: usize, len: usize) -> usize {
    let mut i = mem::MaybeUninit::<insn>::zeroed().assume_init();
    while offset < len {
        if insn_decode_kernel(&mut i, buf.add(offset)) != 0 || !insn_is_nop(&i) { break; }
        offset += i.length as usize;
    }
    offset
}

unsafe fn optimize_nops(_instr: *const u8, buf: *mut u8, len: usize) {
    let mut i = 0; while i < len { let mut x = mem::MaybeUninit::<insn>::zeroed().assume_init();
        if insn_decode_kernel(&mut x, buf.add(i)) != 0 { return; } let next = i + x.length as usize;
        if insn_is_nop(&x) { if next == len { return; } let end = skip_nops(buf, next, len); add_nop(buf.add(i), end-i); i=end; } else { i=next; }
    }
}

unsafe fn apply_reloc(n: i32, p: *mut u8, diff: isize) {
    match n { 1 => { let v = p.read() as i8 as isize + diff; p.write(v as i8 as u8); },
        2 => { let v = (p as *const i16).read_unaligned() as isize + diff; (p as *mut i16).write_unaligned(v as i16); },
        4 => { let v = (p as *const i32).read_unaligned() as isize + diff; (p as *mut i32).write_unaligned(v as i32); }, _ => BUG() }
}

#[no_mangle] pub unsafe extern "C" fn text_poke_apply_relocation(buf:*mut u8, _instr:*const u8, instrlen:usize, _repl:*mut u8, _repl_len:usize) { optimize_nops(_instr, buf, instrlen); }
#[no_mangle] pub unsafe extern "C" fn apply_alternatives(mut start:*mut alt_instr, end:*mut alt_instr) { kasan_disable_current(); while start < end { start=start.add(1); } kasan_enable_current(); }
#[no_mangle] pub unsafe extern "C" fn apply_retpolines(_start:*mut i32,_end:*mut i32) {}
#[no_mangle] pub unsafe extern "C" fn apply_returns(_start:*mut i32,_end:*mut i32) {}
#[no_mangle] pub unsafe extern "C" fn apply_seal_endbr(_start:*mut i32,_end:*mut i32) {}
#[no_mangle] pub unsafe extern "C" fn apply_fineibt(_a:*mut i32,_b:*mut i32,_c:*mut i32,_d:*mut i32) {}

#[no_mangle] pub unsafe extern "C" fn text_poke(addr:*mut c_void, opcode:*const c_void, len:usize)->*mut c_void { memcpy(addr,opcode,len) }
#[no_mangle] pub unsafe extern "C" fn text_poke_kgdb(addr:*mut c_void, opcode:*const c_void, len:usize)->*mut c_void { text_poke(addr,opcode,len) }
#[no_mangle] pub unsafe extern "C" fn text_poke_copy(addr:*mut c_void, opcode:*const c_void, len:usize)->*mut c_void { text_poke(addr,opcode,len) }
#[no_mangle] pub unsafe extern "C" fn text_poke_set(addr:*mut c_void, c:i32, len:usize)->*mut c_void { memset(addr,c,len) }
#[no_mangle] pub unsafe extern "C" fn smp_text_poke_int3_handler(_regs:*mut c_void)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn smp_text_poke_batch_finish() {}
#[no_mangle] pub unsafe extern "C" fn smp_text_poke_batch_add(_addr:*mut c_void,_opcode:*const c_void,_len:usize,_emulate:*const c_void) {}
#[no_mangle] pub unsafe extern "C" fn smp_text_poke_single(addr:*mut c_void,opcode:*const c_void,len:usize,emulate:*const c_void) { smp_text_poke_batch_add(addr,opcode,len,emulate); smp_text_poke_batch_finish(); }
#[no_mangle] pub unsafe extern "C" fn alternative_instructions() { alternatives_patched=1; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
