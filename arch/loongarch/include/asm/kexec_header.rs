/* SPDX-License-Identifier: GPL-2.0 */
/*
 * kexec.h for kexec
 *
 * Copyright (C) 2022 Loongson Technology Corporation Limited
 */

// C header guard: _ASM_KEXEC_H

// Dependencies supplied by the surrounding kernel translation:
// asm/stacktrace.h, asm/page.h

/* Maximum physical address we can use pages from */
pub const KEXEC_SOURCE_MEMORY_LIMIT: c_ulong = !0 as c_ulong;
/* Maximum address we can reach in physical address mode */
pub const KEXEC_DESTINATION_MEMORY_LIMIT: c_ulong = !0 as c_ulong;
/* Maximum address we can use for the control code buffer */
pub const KEXEC_CONTROL_MEMORY_LIMIT: c_ulong = !0 as c_ulong;

/* Reserve a page for the control code buffer */
pub const KEXEC_CONTROL_PAGE_SIZE: usize = PAGE_SIZE;

/* The native architecture */
pub const KEXEC_ARCH: _ = KEXEC_ARCH_LOONGARCH;

pub unsafe fn crash_setup_regs(newregs: *mut pt_regs, oldregs: *mut pt_regs) {
    if !oldregs.is_null() {
        memcpy(
            newregs as *mut c_void,
            oldregs as *const c_void,
            core::mem::size_of::<pt_regs>(),
        );
    } else {
        prepare_frametrace(newregs);
    }
}

// ARCH_HAS_KIMAGE_ARCH

#[repr(C)]
pub struct kimage_arch {
    pub efi_boot: c_ulong,
    pub cmdline_ptr: c_ulong,
    pub systable_ptr: c_ulong,
}

pub struct kimage;

// CONFIG_KEXEC_FILE conditional declarations
#[cfg(CONFIG_KEXEC_FILE)]
extern "C" {
    pub static kexec_efi_ops: kexec_file_ops;
    pub static kexec_elf_ops: kexec_file_ops;

    pub fn arch_kimage_file_post_load_cleanup(image: *mut kimage) -> c_int;

    pub fn load_other_segments(
        image: *mut kimage,
        kernel_load_addr: c_ulong,
        kernel_size: c_ulong,
        initrd: *mut c_char,
        initrd_len: c_ulong,
        cmdline: *mut c_char,
        cmdline_len: c_ulong,
    ) -> c_int;
}

// The C self-referential macro alias is represented by the function declaration above.

pub type do_kexec_t = unsafe extern "C" fn(
    efi_boot: c_ulong,
    cmdline_ptr: c_ulong,
    systable_ptr: c_ulong,
    start_addr: c_ulong,
    first_ind_entry: c_ulong,
);

extern "C" {
    pub static relocate_new_kernel: [c_uchar; 0];
    pub static relocate_new_kernel_size: size_t;
    pub fn kexec_reboot();
}

// CONFIG_SMP conditional declarations
#[cfg(CONFIG_SMP)]
extern "C" {
    pub static kexec_ready_to_reboot: atomic_t;
    pub static kexec_smp_wait: [c_uchar; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
