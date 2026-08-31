// SPDX-License-Identifier: GPL-2.0
// C dependencies: <string.h>, <linux/compiler.h>, <linux/zalloc.h>, "../disasm.h"

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

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

pub type associate_instruction_ops_t =
    unsafe extern "C" fn(*mut arch, *const c_char) -> *const ins_ops;

#[repr(C)]
pub struct arch {
    pub name: *const c_char,
    pub id: e_machine_and_e_flags,
    pub objdump: objdump,
    pub associate_instruction_ops: Option<associate_instruction_ops_t>,
}

unsafe extern "C" {
    static call_ops: ins_ops;
    static ret_ops: ins_ops;
    static jump_ops: ins_ops;

    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn zalloc(size: usize) -> *mut c_void;
    fn arch__associate_ins_ops(arch: *mut arch, name: *const c_char, ops: *const ins_ops);
}

static JAL: &[u8; 4] = b"jal\0";
static JR: &[u8; 3] = b"jr\0";
static CALL: &[u8; 5] = b"call\0";
static RET: &[u8; 4] = b"ret\0";
static RISCV: &[u8; 6] = b"riscv\0";

unsafe extern "C" fn riscv64__associate_ins_ops(
    arch: *mut arch,
    name: *const c_char,
) -> *const ins_ops {
    let mut ops: *const ins_ops = ptr::null();

    if strncmp(name, JAL.as_ptr() as *const c_char, 3) == 0
        || strncmp(name, JR.as_ptr() as *const c_char, 2) == 0
        || strncmp(name, CALL.as_ptr() as *const c_char, 4) == 0
    {
        ops = &call_ops;
    } else if strncmp(name, RET.as_ptr() as *const c_char, 3) == 0 {
        ops = &ret_ops;
    } else if *name == b'j' as c_char || *name == b'b' as c_char {
        ops = &jump_ops;
    } else {
        return ptr::null();
    }

    arch__associate_ins_ops(arch, name, ops);

    ops
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch__new_riscv64(
    id: *const e_machine_and_e_flags,
    _cpuid: *const c_char,
) -> *const arch {
    let arch = zalloc(mem::size_of::<arch>()) as *mut arch;

    if arch.is_null() {
        return ptr::null();
    }

    (*arch).name = RISCV.as_ptr() as *const c_char;
    (*arch).id = *id;
    (*arch).objdump.comment_char = b'#' as c_char;
    (*arch).associate_instruction_ops = Some(riscv64__associate_ins_ops);
    arch
}
