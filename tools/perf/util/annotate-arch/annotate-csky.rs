// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2019 Hangzhou C-SKY Microsystems co.,ltd.
// C dependencies: string.h, linux/compiler.h, linux/zalloc.h, ../disasm.h

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct e_machine_and_e_flags {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ins_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct objdump {
    pub comment_char: c_char,
}

#[repr(C)]
pub struct arch {
    pub name: *const c_char,
    pub id: e_machine_and_e_flags,
    pub objdump: objdump,
    pub associate_instruction_ops:
        Option<unsafe extern "C" fn(*mut arch, *const c_char) -> *const ins_ops>,
}

extern "C" {
    static jump_ops: ins_ops;
    static call_ops: ins_ops;
    static ret_ops: ins_ops;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn zalloc(size: usize) -> *mut c_void;
    fn arch__associate_ins_ops(arch: *mut arch, name: *const c_char, ops: *const ins_ops);
}

unsafe extern "C" fn csky__associate_ins_ops(
    arch: *mut arch,
    name: *const c_char,
) -> *const ins_ops {
    let mut ops: *const ins_ops = core::ptr::null();

    /* catch all kind of jumps */
    if strcmp(name, b"bt\0".as_ptr() as *const c_char) == 0
        || strcmp(name, b"bf\0".as_ptr() as *const c_char) == 0
        || strcmp(name, b"bez\0".as_ptr() as *const c_char) == 0
        || strcmp(name, b"bnez\0".as_ptr() as *const c_char) == 0
        || strcmp(name, b"bnezad\0".as_ptr() as *const c_char) == 0
        || strcmp(name, b"bhsz\0".as_ptr() as *const c_char) == 0
        || strcmp(name, b"bhz\0".as_ptr() as *const c_char) == 0
        || strcmp(name, b"blsz\0".as_ptr() as *const c_char) == 0
        || strcmp(name, b"blz\0".as_ptr() as *const c_char) == 0
        || strcmp(name, b"br\0".as_ptr() as *const c_char) == 0
        || strcmp(name, b"jmpi\0".as_ptr() as *const c_char) == 0
        || strcmp(name, b"jmp\0".as_ptr() as *const c_char) == 0
    {
        ops = &jump_ops;
    }

    /* catch function call */
    if strcmp(name, b"bsr\0".as_ptr() as *const c_char) == 0
        || strcmp(name, b"jsri\0".as_ptr() as *const c_char) == 0
        || strcmp(name, b"jsr\0".as_ptr() as *const c_char) == 0
    {
        ops = &call_ops;
    }

    /* catch function return */
    if strcmp(name, b"rts\0".as_ptr() as *const c_char) == 0 {
        ops = &ret_ops;
    }

    if !ops.is_null() {
        arch__associate_ins_ops(arch, name, ops);
    }
    ops
}

#[no_mangle]
pub unsafe extern "C" fn arch__new_csky(
    id: *const e_machine_and_e_flags,
    _cpuid: *const c_char,
) -> *const arch {
    let arch = zalloc(core::mem::size_of::<arch>()) as *mut arch;

    if arch.is_null() {
        return core::ptr::null();
    }

    (*arch).name = b"csky\0".as_ptr() as *const c_char;
    core::ptr::copy_nonoverlapping(id, &mut (*arch).id, 1);
    (*arch).objdump.comment_char = b'/' as c_char;
    (*arch).associate_instruction_ops = Some(csky__associate_ins_ops);
    arch
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
