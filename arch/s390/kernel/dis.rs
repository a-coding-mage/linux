// SPDX-License-Identifier: GPL-2.0
/* Disassemble s390 instructions.  C kernel dependencies are supplied externally. */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

use core::{ffi::{c_char, c_int, c_uint, c_ulong, c_void}, ptr};

pub const OPERAND_GPR: u16 = 0x1;
pub const OPERAND_FPR: u16 = 0x2;
pub const OPERAND_AR: u16 = 0x4;
pub const OPERAND_CR: u16 = 0x8;
pub const OPERAND_VR: u16 = 0x10;
pub const OPERAND_DISP: u16 = 0x20;
pub const OPERAND_BASE: u16 = 0x40;
pub const OPERAND_INDEX: u16 = 0x80;
pub const OPERAND_PCREL: u16 = 0x100;
pub const OPERAND_SIGNED: u16 = 0x200;
pub const OPERAND_LENGTH: u16 = 0x400;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct s390_operand { pub bits: u8, pub shift: u8, pub flags: u16 }
#[repr(C)]
pub union s390_insn_name { pub name: [c_char; 5], pub data: s390_insn_data }
#[repr(C, packed)]
pub struct s390_insn_data { pub zero: u8, pub offset: u32 }
#[repr(C)]
pub struct s390_insn { pub name: s390_insn_name, pub opfrag: u8, pub format: u8 }
#[repr(C, packed)]
pub struct s390_opcode_offset { pub opcode: u8, pub mask: u8, pub byte: u8, pub offset: u16, pub count: u16 }

/* Operand indices and instruction-format identifiers are supplied by asm/dis.h. */
extern "C" {
    static mut long_insn_name: [[c_char; 7]; 256];
    static mut opcode: [s390_insn; 256];
    static mut opcode_offset: [s390_opcode_offset; 256];
    fn insn_length(opcode: u8) -> c_int;
    fn user_mode(regs: *mut pt_regs) -> bool;
    fn copy_from_user(dst: *mut c_void, src: *const c_void, len: c_int) -> c_int;
    fn copy_from_kernel_nofault(dst: *mut c_void, src: *const c_void, len: c_int) -> c_int;
    fn sprintf(dst: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn printk(fmt: *const c_char, ...);
    fn pr_cont(fmt: *const c_char, ...);
}
#[repr(C)] pub struct pt_regs { pub psw: psw_t, pub int_code: u32 }
#[repr(C)] pub struct psw_t { pub addr: c_ulong }

/* The indexed format table is initialized by the architecture opcode table. */
extern "C" { static formats: [[u8; 6]; 256]; static operands: [s390_operand; 256]; }

unsafe fn extract_operand(code: *mut u8, operand: *const s390_operand) -> u32 {
    let o = &*operand;
    let mut cp = code.add((o.shift / 8) as usize);
    let mut bits = (o.shift & 7) as i32 + o.bits as i32;
    let mut val = 0u32;
    while bits > 0 { val = (val << 8) | *cp as u32; cp = cp.add(1); bits -= 8; }
    val >>= (-bits) as u32;
    val &= ((1u32 << (o.bits - 1)) << 1) - 1;
    if o.bits == 20 && o.shift == 20 { val = (val & 0xff) << 12 | (val & 0xfff00) >> 8; }
    if o.flags & OPERAND_VR != 0 {
        if o.shift == 8 { val |= ((*code.add(4) & 8) as u32) << 1; }
        else if o.shift == 12 { val |= ((*code.add(4) & 4) as u32) << 2; }
        else if o.shift == 16 { val |= ((*code.add(4) & 2) as u32) << 3; }
        else if o.shift == 32 { val |= ((*code.add(4) & 1) as u32) << 4; }
    }
    if o.flags & (OPERAND_SIGNED | OPERAND_PCREL) != 0 && val & (1u32 << (o.bits - 1)) != 0 { val |= (!0u32 << (o.bits - 1)) << 1; }
    if o.flags & OPERAND_PCREL != 0 { val <<= 1; }
    if o.flags & OPERAND_LENGTH != 0 { val = val.wrapping_add(1); }
    val
}

#[no_mangle]
pub unsafe extern "C" fn find_insn(code: *mut u8) -> *mut s390_insn {
    let mut entry = &opcode_offset[255];
    for i in 0..256 { if opcode_offset[i].opcode == *code { entry = &opcode_offset[i]; break; } }
    let opfrag = *code.add(entry.byte as usize) & entry.mask;
    let mut insn = opcode.as_mut_ptr().add(entry.offset as usize);
    for _ in 0..entry.count { if (*insn).opfrag == opfrag { return insn; } insn = insn.add(1); }
    ptr::null_mut()
}

unsafe fn print_insn(buffer: *mut c_char, code: *mut u8, addr: c_ulong) -> c_int {
    let insn = find_insn(code); if insn.is_null() { return sprintf(buffer, b"unknown\0".as_ptr() as _,); }
    let mut p = buffer;
    let d = &(*insn).name.data;
    let name = if d.zero == 0 { long_insn_name.as_ptr().add(d.offset as usize) as *const c_char } else { (*insn).name.name.as_ptr() };
    p = p.add(sprintf(p, b"%.7s\t\0".as_ptr() as _, name) as usize);
    let mut sep = 0i32;
    for opid in formats[(*insn).format as usize] { if opid == 0 { break; } let op = &operands[opid as usize]; let v = extract_operand(code, op); if op.flags & OPERAND_INDEX != 0 && v == 0 { continue; } if op.flags & OPERAND_BASE != 0 && v == 0 && sep == b'(' as i32 { sep = b',' as i32; continue; } if sep != 0 { p = p.add(sprintf(p, b"%c\0".as_ptr() as _, sep) as usize); } let f = if op.flags & OPERAND_GPR != 0 { b"%r%u\0" } else if op.flags & OPERAND_FPR != 0 { b"%f%u\0" } else { b"%u\0" }; p = p.add(sprintf(p, f.as_ptr() as _, v) as usize); sep = if op.flags & OPERAND_DISP != 0 { b'(' as i32 } else { b',' as i32 }; }
    p.offset_from(buffer) as c_int
}

// Kernel entry points retain their original interfaces; register and formatting helpers are external dependencies.
#[no_mangle] pub unsafe extern "C" fn print_fn_code(mut code: *mut u8, mut len: c_ulong) { let mut buffer = [0i8; 128]; while len != 0 { let n = insn_length(*code); if n as c_ulong > len { break; } let p = buffer.as_mut_ptr(); let used = print_insn(p, code, code as c_ulong); let _ = used; printk(b"%s\0".as_ptr() as _, p); code = code.add(n as usize); len -= n as c_ulong; } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
