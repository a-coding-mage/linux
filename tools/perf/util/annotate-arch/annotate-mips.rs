// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/annotate-arch/annotate-mips.c.
// Dependencies from string.h, linux/compiler.h, linux/zalloc.h, and ../disasm.h
// are represented as external declarations or C-compatible local type shapes.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct ins_ops {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e_machine_and_e_flags {
    _private: [u8; 0],
}

#[repr(C)]
pub struct objdump {
    pub comment_char: c_char,
}

pub type AssociateInstructionOps =
    Option<unsafe extern "C" fn(*mut arch, *const c_char) -> *const ins_ops>;

#[repr(C)]
pub struct arch {
    pub name: *const c_char,
    pub id: e_machine_and_e_flags,
    pub objdump: objdump,
    pub associate_instruction_ops: AssociateInstructionOps,
}

unsafe extern "C" {
    static call_ops: ins_ops;
    static ret_ops: ins_ops;
    static jump_ops: ins_ops;

    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn zalloc(size: usize) -> *mut c_void;
    fn arch__associate_ins_ops(arch: *mut arch, name: *const c_char, ops: *const ins_ops);
}

const MIPS_NAME: &[u8; 5] = b"mips\0";

unsafe extern "C" fn mips__associate_ins_ops(
    arch: *mut arch,
    name: *const c_char,
) -> *const ins_ops {
    let mut ops: *const ins_ops = core::ptr::null();

    unsafe {
        if strncmp(name, c"bal".as_ptr(), 3) == 0
            || strncmp(name, c"bgezal".as_ptr(), 6) == 0
            || strncmp(name, c"bltzal".as_ptr(), 6) == 0
            || strncmp(name, c"bgtzal".as_ptr(), 6) == 0
            || strncmp(name, c"blezal".as_ptr(), 6) == 0
            || strncmp(name, c"beqzal".as_ptr(), 6) == 0
            || strncmp(name, c"bnezal".as_ptr(), 6) == 0
            || strncmp(name, c"bgtzl".as_ptr(), 5) == 0
            || strncmp(name, c"bltzl".as_ptr(), 5) == 0
            || strncmp(name, c"bgezl".as_ptr(), 5) == 0
            || strncmp(name, c"blezl".as_ptr(), 5) == 0
            || strncmp(name, c"jialc".as_ptr(), 5) == 0
            || strncmp(name, c"beql".as_ptr(), 4) == 0
            || strncmp(name, c"bnel".as_ptr(), 4) == 0
            || strncmp(name, c"jal".as_ptr(), 3) == 0
        {
            ops = &raw const call_ops;
        } else if strncmp(name, c"jr".as_ptr(), 2) == 0 {
            ops = &raw const ret_ops;
        } else if *name == b'j' as c_char || *name == b'b' as c_char {
            ops = &raw const jump_ops;
        } else {
            return core::ptr::null();
        }

        arch__associate_ins_ops(arch, name, ops);
    }

    ops
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch__new_mips(
    id: *const e_machine_and_e_flags,
    _cpuid: *const c_char,
) -> *const arch {
    let arch = unsafe { zalloc(core::mem::size_of::<arch>()) as *mut arch };

    if arch.is_null() {
        return core::ptr::null();
    }

    unsafe {
        (*arch).name = MIPS_NAME.as_ptr() as *const c_char;
        (*arch).id = *id;
        (*arch).objdump.comment_char = b'#' as c_char;
        (*arch).associate_instruction_ops = Some(mips__associate_ins_ops);
    }

    arch
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
