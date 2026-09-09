/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_KEXEC_CORE */

/* Maximum physical address we can use pages from */
pub const KEXEC_SOURCE_MEMORY_LIMIT: usize = usize::MAX;
/* Maximum address we can reach in physical address mode */
pub const KEXEC_DESTINATION_MEMORY_LIMIT: usize = usize::MAX;
/* Maximum address we can use for the control code buffer */
pub const KEXEC_CONTROL_MEMORY_LIMIT: usize = usize::MAX;

pub const KEXEC_CONTROL_PAGE_SIZE: usize = 4096;

pub const KEXEC_ARCH: _ = KEXEC_ARCH_68K;

/* __ASSEMBLER__ */

#[inline]
pub fn crash_setup_regs(newregs: *mut pt_regs, oldregs: *mut pt_regs) {
    let _ = newregs;
    let _ = oldregs;
    /* Dummy implementation for now */
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
