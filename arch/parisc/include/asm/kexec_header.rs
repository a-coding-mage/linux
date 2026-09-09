/* SPDX-License-Identifier: GPL-2.0 */

/* Maximum physical address we can use pages from */
pub const KEXEC_SOURCE_MEMORY_LIMIT: usize = usize::MAX;
/* Maximum address we can reach in physical address mode */
pub const KEXEC_DESTINATION_MEMORY_LIMIT: usize = usize::MAX;
/* Maximum address we can use for the control code buffer */
pub const KEXEC_CONTROL_MEMORY_LIMIT: usize = usize::MAX;

pub const KEXEC_CONTROL_PAGE_SIZE: usize = 4096;

pub const KEXEC_ARCH: _ = KEXEC_ARCH_PARISC;
/* ARCH_HAS_KIMAGE_ARCH */

#[repr(C)]
pub struct kimage_arch {
    pub initrd_start: usize,
    pub initrd_end: usize,
    pub cmdline: usize,
}

pub unsafe fn crash_setup_regs(
    newregs: *mut pt_regs,
    oldregs: *mut pt_regs,
) {
    /* Dummy implementation for now */
    let _ = (newregs, oldregs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
