/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019 FORTH-ICS/CARV
 *  Nick Kossifidis <mick@ics.forth.gr>
 */

/* C header guard: _RISCV_KEXEC_H */

/* Dependency supplied by asm/page.h: PAGE_SIZE. */

/* Maximum physical address we can use pages from. */
pub const KEXEC_SOURCE_MEMORY_LIMIT: usize = usize::MAX;

/* Maximum address we can reach in physical address mode. */
pub const KEXEC_DESTINATION_MEMORY_LIMIT: usize = usize::MAX;

/* Maximum address we can use for the control code buffer. */
pub const KEXEC_CONTROL_MEMORY_LIMIT: usize = usize::MAX;

/* Reserve a page for the control code buffer. */
pub const KEXEC_CONTROL_PAGE_SIZE: usize = PAGE_SIZE;

pub const KEXEC_ARCH: _ = KEXEC_ARCH_RISCV;

extern "C" {
    pub fn riscv_crash_save_regs(newregs: *mut pt_regs);
}

pub unsafe fn crash_setup_regs(newregs: *mut pt_regs, oldregs: *const pt_regs) {
    if !oldregs.is_null() {
        core::ptr::copy_nonoverlapping(
            oldregs as *const u8,
            newregs as *mut u8,
            core::mem::size_of::<pt_regs>(),
        );
    } else {
        riscv_crash_save_regs(newregs);
    }
}

/* ARCH_HAS_KIMAGE_ARCH */

#[repr(C)]
pub struct kimage_arch {
    /* For CONFIG_KEXEC_FILE. */
    pub fdt: *mut core::ffi::c_void,
    pub fdt_addr: usize,
}

extern "C" {
    pub static riscv_kexec_relocate: u8;
    pub static riscv_kexec_relocate_size: u32;
}

pub type riscv_kexec_method = unsafe extern "C" fn(
    first_ind_entry: usize,
    jump_addr: usize,
    fdt_addr: usize,
    hartid: usize,
    va_pa_off: usize,
);

extern "C" {
    pub static mut riscv_kexec_norelocate: riscv_kexec_method;
}

/* The following declarations are present only when CONFIG_KEXEC_FILE is enabled. */
/*
#ifdef CONFIG_KEXEC_FILE
*/
extern "C" {
    pub static elf_kexec_ops: kexec_file_ops;
    pub static image_kexec_ops: kexec_file_ops;
}

pub struct purgatory_info;

extern "C" {
    pub fn arch_kexec_apply_relocations_add(
        pi: *mut purgatory_info,
        section: *mut Elf_Shdr,
        relsec: *const Elf_Shdr,
        symtab: *const Elf_Shdr,
    ) -> i32;
}

/* #define arch_kexec_apply_relocations_add arch_kexec_apply_relocations_add */

pub struct kimage;

extern "C" {
    pub fn arch_kimage_file_post_load_cleanup(image: *mut kimage) -> i32;
}

/* #define arch_kimage_file_post_load_cleanup arch_kimage_file_post_load_cleanup */

extern "C" {
    pub fn load_extra_segments(
        image: *mut kimage,
        kernel_start: usize,
        kernel_len: usize,
        initrd: *mut core::ffi::c_char,
        initrd_len: usize,
        cmdline: *mut core::ffi::c_char,
        cmdline_len: usize,
    ) -> i32;
}
/*
#endif
*/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
