// SPDX-License-Identifier: GPL-2.0
/*
 * Clang Control Flow Integrity (CFI) error handling.
 *
 * Copyright (C) 2022 Google LLC
 */

// Linux dependencies supplied by other files.

pub static mut cfi_warn: bool = cfg!(feature = "CONFIG_CFI_PERMISSIVE");

pub unsafe fn report_cfi_failure(
    regs: *mut pt_regs,
    addr: libc::c_ulong,
    target: *mut libc::c_ulong,
    type_: u32,
) -> bug_trap_type {
    if !target.is_null() {
        pr_err(
            "CFI failure at %pS (target: %pS; expected type: 0x%08x)\n",
            addr as *mut libc::c_void,
            *target as *mut libc::c_void,
            type_,
        );
    } else {
        pr_err(
            "CFI failure at %pS (no target information)\n",
            addr as *mut libc::c_void,
        );
    }

    if cfi_warn {
        __warn(
            core::ptr::null_mut(),
            0,
            addr as *mut libc::c_void,
            0,
            regs,
            core::ptr::null_mut(),
        );
        return BUG_TRAP_TYPE_WARN;
    }

    BUG_TRAP_TYPE_BUG
}

/*
 * Declare two non-existent functions with types that match bpf_func_t and
 * bpf_callback_t pointers, and use DEFINE_CFI_TYPE to define type hash
 * variables for each function type. The cfi_bpf_* variables are used by
 * arch-specific BPF JIT implementations to ensure indirectly callable JIT
 * code has matching CFI type hashes.
 */
extern "C" {
    pub fn __bpf_prog_runX();
    pub fn __bpf_callback_fn();
}

// DEFINE_CFI_TYPE(cfi_bpf_hash, __bpf_prog_runX);
// DEFINE_CFI_TYPE(cfi_bpf_subprog_hash, __bpf_callback_fn);

#[cfg(feature = "CONFIG_ARCH_USES_CFI_TRAPS")]
unsafe fn trap_address(p: *mut i32) -> libc::c_ulong {
    (p as libc::c_long).wrapping_add(*p as libc::c_long) as libc::c_ulong
}

#[cfg(feature = "CONFIG_ARCH_USES_CFI_TRAPS")]
unsafe fn is_trap(
    addr: libc::c_ulong,
    mut start: *mut i32,
    end: *mut i32,
) -> bool {
    while start < end {
        if trap_address(start) == addr {
            return true;
        }
        start = start.add(1);
    }
    false
}

#[cfg(all(feature = "CONFIG_ARCH_USES_CFI_TRAPS", feature = "CONFIG_MODULES"))]
pub unsafe fn module_cfi_finalize(
    hdr: *const Elf_Ehdr,
    sechdrs: *const Elf_Shdr,
    mod_: *mut module,
) {
    (*mod_).kcfi_traps = core::ptr::null_mut();
    (*mod_).kcfi_traps_end = core::ptr::null_mut();

    let secstrings = (hdr as *const u8).add((*sechdrs.add((*hdr).e_shstrndx as usize)).sh_offset as usize)
        as *mut libc::c_char;

    let mut i = 1;
    while i < (*hdr).e_shnum as usize {
        if strcmp(
            secstrings.add((*sechdrs.add(i)).sh_name as usize),
            b"__kcfi_traps\0".as_ptr() as *const libc::c_char,
        ) != 0 {
            i += 1;
            continue;
        }

        (*mod_).kcfi_traps = (*sechdrs.add(i)).sh_addr as *mut i32;
        (*mod_).kcfi_traps_end = ((*sechdrs.add(i)).sh_addr + (*sechdrs.add(i)).sh_size) as *mut i32;
        break;
    }
}

#[cfg(all(feature = "CONFIG_ARCH_USES_CFI_TRAPS", feature = "CONFIG_MODULES"))]
unsafe fn is_module_cfi_trap(addr: libc::c_ulong) -> bool {
    let mod_ = __module_address(addr);
    if !mod_.is_null() {
        return is_trap(addr, (*mod_).kcfi_traps, (*mod_).kcfi_traps_end);
    }
    false
}

#[cfg(all(feature = "CONFIG_ARCH_USES_CFI_TRAPS", not(feature = "CONFIG_MODULES")))]
unsafe fn is_module_cfi_trap(_addr: libc::c_ulong) -> bool {
    false
}

#[cfg(feature = "CONFIG_ARCH_USES_CFI_TRAPS")]
extern "C" {
    static mut __start___kcfi_traps: i32;
    static mut __stop___kcfi_traps: i32;
}

#[cfg(feature = "CONFIG_ARCH_USES_CFI_TRAPS")]
pub unsafe fn is_cfi_trap(addr: libc::c_ulong) -> bool {
    if is_trap(
        addr,
        &raw mut __start___kcfi_traps,
        &raw mut __stop___kcfi_traps,
    ) {
        return true;
    }
    is_module_cfi_trap(addr)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
