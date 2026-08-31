// SPDX-License-Identifier: GPL-2.0
// Translated from C. External definitions are provided by the surrounding perf code.

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
    pub associate_instruction_ops:
        Option<unsafe extern "C" fn(*mut arch, *const c_char) -> *const ins_ops>,
    pub objdump: objdump,
}

unsafe extern "C" {
    static call_ops: ins_ops;
    static ret_ops: ins_ops;
    static mov_ops: ins_ops;
    static jump_ops: ins_ops;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn zalloc(size: usize) -> *mut c_void;
    fn arch__associate_ins_ops(arch: *mut arch, name: *const c_char, ops: *const ins_ops);
}

unsafe fn cch(s: *const c_char, idx: usize) -> c_char {
    unsafe { *s.add(idx) }
}

unsafe fn is_branch_cond(cond: *const c_char) -> c_int {
    if unsafe { cch(cond, 0) } == b'\0' as c_char {
        return 1;
    }

    if unsafe { cch(cond, 0) } == b'a' as c_char && unsafe { cch(cond, 1) } == b'\0' as c_char {
        return 1;
    }

    if unsafe { cch(cond, 0) } == b'c' as c_char
        && (unsafe { cch(cond, 1) } == b'c' as c_char
            || unsafe { cch(cond, 1) } == b's' as c_char)
        && unsafe { cch(cond, 2) } == b'\0' as c_char
    {
        return 1;
    }

    if unsafe { cch(cond, 0) } == b'e' as c_char
        && (unsafe { cch(cond, 1) } == b'\0' as c_char
            || (unsafe { cch(cond, 1) } == b'q' as c_char
                && unsafe { cch(cond, 2) } == b'\0' as c_char))
    {
        return 1;
    }

    if unsafe { cch(cond, 0) } == b'g' as c_char
        && (unsafe { cch(cond, 1) } == b'\0' as c_char
            || (unsafe { cch(cond, 1) } == b't' as c_char
                && unsafe { cch(cond, 2) } == b'\0' as c_char)
            || (unsafe { cch(cond, 1) } == b'e' as c_char
                && unsafe { cch(cond, 2) } == b'\0' as c_char)
            || (unsafe { cch(cond, 1) } == b'e' as c_char
                && unsafe { cch(cond, 2) } == b'u' as c_char
                && unsafe { cch(cond, 3) } == b'\0' as c_char))
    {
        return 1;
    }

    if unsafe { cch(cond, 0) } == b'l' as c_char
        && (unsafe { cch(cond, 1) } == b'\0' as c_char
            || (unsafe { cch(cond, 1) } == b't' as c_char
                && unsafe { cch(cond, 2) } == b'\0' as c_char)
            || (unsafe { cch(cond, 1) } == b'u' as c_char
                && unsafe { cch(cond, 2) } == b'\0' as c_char)
            || (unsafe { cch(cond, 1) } == b'e' as c_char
                && unsafe { cch(cond, 2) } == b'\0' as c_char)
            || (unsafe { cch(cond, 1) } == b'e' as c_char
                && unsafe { cch(cond, 2) } == b'u' as c_char
                && unsafe { cch(cond, 3) } == b'\0' as c_char))
    {
        return 1;
    }

    if unsafe { cch(cond, 0) } == b'n' as c_char
        && (unsafe { cch(cond, 1) } == b'\0' as c_char
            || (unsafe { cch(cond, 1) } == b'e' as c_char
                && unsafe { cch(cond, 2) } == b'\0' as c_char)
            || (unsafe { cch(cond, 1) } == b'z' as c_char
                && unsafe { cch(cond, 2) } == b'\0' as c_char)
            || (unsafe { cch(cond, 1) } == b'e' as c_char
                && unsafe { cch(cond, 2) } == b'g' as c_char
                && unsafe { cch(cond, 3) } == b'\0' as c_char))
    {
        return 1;
    }

    if unsafe { cch(cond, 0) } == b'b' as c_char
        && unsafe { cch(cond, 1) } == b'p' as c_char
        && unsafe { cch(cond, 2) } == b'o' as c_char
        && unsafe { cch(cond, 3) } == b's' as c_char
        && unsafe { cch(cond, 4) } == b'\0' as c_char
    {
        return 1;
    }

    if unsafe { cch(cond, 0) } == b'v' as c_char
        && (unsafe { cch(cond, 1) } == b'c' as c_char
            || unsafe { cch(cond, 1) } == b's' as c_char)
        && unsafe { cch(cond, 2) } == b'\0' as c_char
    {
        return 1;
    }

    if unsafe { cch(cond, 0) } == b'b' as c_char
        && unsafe { cch(cond, 1) } == b'z' as c_char
        && unsafe { cch(cond, 2) } == b'\0' as c_char
    {
        return 1;
    }

    0
}

unsafe fn is_branch_reg_cond(cond: *const c_char) -> c_int {
    if (unsafe { cch(cond, 0) } == b'n' as c_char || unsafe { cch(cond, 0) } == b'l' as c_char)
        && unsafe { cch(cond, 1) } == b'z' as c_char
        && unsafe { cch(cond, 2) } == b'\0' as c_char
    {
        return 1;
    }

    if unsafe { cch(cond, 0) } == b'z' as c_char && unsafe { cch(cond, 1) } == b'\0' as c_char {
        return 1;
    }

    if (unsafe { cch(cond, 0) } == b'g' as c_char || unsafe { cch(cond, 0) } == b'l' as c_char)
        && unsafe { cch(cond, 1) } == b'e' as c_char
        && unsafe { cch(cond, 2) } == b'z' as c_char
        && unsafe { cch(cond, 3) } == b'\0' as c_char
    {
        return 1;
    }

    if unsafe { cch(cond, 0) } == b'g' as c_char
        && unsafe { cch(cond, 1) } == b'z' as c_char
        && unsafe { cch(cond, 2) } == b'\0' as c_char
    {
        return 1;
    }

    0
}

unsafe fn is_branch_float_cond(cond: *const c_char) -> c_int {
    if unsafe { cch(cond, 0) } == b'\0' as c_char {
        return 1;
    }

    if (unsafe { cch(cond, 0) } == b'a' as c_char
        || unsafe { cch(cond, 0) } == b'e' as c_char
        || unsafe { cch(cond, 0) } == b'z' as c_char
        || unsafe { cch(cond, 0) } == b'g' as c_char
        || unsafe { cch(cond, 0) } == b'l' as c_char
        || unsafe { cch(cond, 0) } == b'n' as c_char
        || unsafe { cch(cond, 0) } == b'o' as c_char
        || unsafe { cch(cond, 0) } == b'u' as c_char)
        && unsafe { cch(cond, 1) } == b'\0' as c_char
    {
        return 1;
    }

    if (((unsafe { cch(cond, 0) } == b'g' as c_char
        && unsafe { cch(cond, 1) } == b'e' as c_char)
        || (unsafe { cch(cond, 0) } == b'l' as c_char
            && (unsafe { cch(cond, 1) } == b'e' as c_char
                || unsafe { cch(cond, 1) } == b'g' as c_char))
        || (unsafe { cch(cond, 0) } == b'n' as c_char
            && (unsafe { cch(cond, 1) } == b'e' as c_char
                || unsafe { cch(cond, 1) } == b'z' as c_char))
        || (unsafe { cch(cond, 0) } == b'u' as c_char
            && (unsafe { cch(cond, 1) } == b'e' as c_char
                || unsafe { cch(cond, 1) } == b'g' as c_char
                || unsafe { cch(cond, 1) } == b'l' as c_char)))
        && unsafe { cch(cond, 2) } == b'\0' as c_char
    {
        return 1;
    }

    if unsafe { cch(cond, 0) } == b'u' as c_char
        && (unsafe { cch(cond, 1) } == b'g' as c_char
            || unsafe { cch(cond, 1) } == b'l' as c_char)
        && unsafe { cch(cond, 2) } == b'e' as c_char
        && unsafe { cch(cond, 3) } == b'\0' as c_char
    {
        return 1;
    }

    0
}

unsafe extern "C" fn sparc__associate_instruction_ops(
    arch: *mut arch,
    mut name: *const c_char,
) -> *const ins_ops {
    let mut ops: *const ins_ops = core::ptr::null();

    if unsafe { strcmp(name, c"call".as_ptr()) } == 0
        || unsafe { strcmp(name, c"jmp".as_ptr()) } == 0
        || unsafe { strcmp(name, c"jmpl".as_ptr()) } == 0
    {
        ops = &raw const call_ops;
    } else if unsafe { strcmp(name, c"ret".as_ptr()) } == 0
        || unsafe { strcmp(name, c"retl".as_ptr()) } == 0
        || unsafe { strcmp(name, c"return".as_ptr()) } == 0
    {
        ops = &raw const ret_ops;
    } else if unsafe { strcmp(name, c"mov".as_ptr()) } == 0 {
        ops = &raw const mov_ops;
    } else {
        if unsafe { cch(name, 0) } == b'c' as c_char
            && (unsafe { cch(name, 1) } == b'w' as c_char
                || unsafe { cch(name, 1) } == b'x' as c_char)
        {
            name = unsafe { name.add(2) };
        }

        if unsafe { cch(name, 0) } == b'b' as c_char {
            let cond = unsafe { name.add(1) };

            if unsafe { cch(cond, 0) } == b'r' as c_char {
                if unsafe { is_branch_reg_cond(cond.add(1)) } != 0 {
                    ops = &raw const jump_ops;
                }
            } else if unsafe { is_branch_cond(cond) } != 0 {
                ops = &raw const jump_ops;
            }
        } else if unsafe { cch(name, 0) } == b'f' as c_char
            && unsafe { cch(name, 1) } == b'b' as c_char
        {
            if unsafe { is_branch_float_cond(name.add(2)) } != 0 {
                ops = &raw const jump_ops;
            }
        }
    }

    if !ops.is_null() {
        unsafe { arch__associate_ins_ops(arch, name, ops) };
    }

    ops
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch__new_sparc(
    id: *const e_machine_and_e_flags,
    _cpuid: *const c_char,
) -> *const arch {
    let arch = unsafe { zalloc(core::mem::size_of::<arch>()) as *mut arch };

    if arch.is_null() {
        return core::ptr::null();
    }

    unsafe {
        (*arch).name = c"sparc".as_ptr();
        core::ptr::copy_nonoverlapping(id, &mut (*arch).id, 1);
        (*arch).associate_instruction_ops = Some(sparc__associate_instruction_ops);
        (*arch).objdump.comment_char = b'#' as c_char;
    }
    arch
}
