// SPDX-License-Identifier: GPL-2.0
/*
 * Author: Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the corresponding kernel headers:
// linux/binfmts.h, linux/elf.h, linux/sched.h,
// asm/cpu-features.h, and asm/cpu-info.h.

pub unsafe fn arch_elf_pt_proc(
    _ehdr: *mut core::ffi::c_void,
    _phdr: *mut core::ffi::c_void,
    _elf: *mut file,
    _is_interp: bool,
    _state: *mut arch_elf_state,
) -> i32 {
    0
}

pub unsafe fn arch_check_elf(
    _ehdr: *mut core::ffi::c_void,
    _has_interpreter: bool,
    _interp_ehdr: *mut core::ffi::c_void,
    _state: *mut arch_elf_state,
) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
