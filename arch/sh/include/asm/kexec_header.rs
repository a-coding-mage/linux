/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: asm/ptrace.h, asm/string.h, and linux/kernel.h.

/*
 * KEXEC_SOURCE_MEMORY_LIMIT maximum page get_free_page can return.
 * I.e. Maximum page that is mapped directly into kernel memory,
 * and kmap is not required.
 *
 * Someone correct me if FIXADDR_START - PAGEOFFSET is not the correct
 * calculation for the amount of memory directly mappable into the
 * kernel memory space.
 */

/* Maximum physical address we can use pages from */
pub const KEXEC_SOURCE_MEMORY_LIMIT: usize = (!0usize);
/* Maximum address we can reach in physical address mode */
pub const KEXEC_DESTINATION_MEMORY_LIMIT: usize = (!0usize);
/* Maximum address we can use for the control code buffer */
pub const KEXEC_CONTROL_MEMORY_LIMIT: usize = TASK_SIZE;

pub const KEXEC_CONTROL_PAGE_SIZE: usize = 4096;

/* The native architecture */
pub const KEXEC_ARCH: usize = KEXEC_ARCH_SH;

// CONFIG_KEXEC_CORE conditionally includes the following declarations.
#[cfg(CONFIG_KEXEC_CORE)]
extern "C" {
    pub fn reserve_crashkernel();
}

#[cfg(CONFIG_KEXEC_CORE)]
#[inline(always)]
pub unsafe fn crash_setup_regs(newregs: *mut pt_regs, oldregs: *mut pt_regs) {
    if !oldregs.is_null() {
        core::ptr::copy_nonoverlapping(
            oldregs as *const u8,
            newregs as *mut u8,
            core::mem::size_of::<pt_regs>(),
        );
    } else {
        core::arch::asm!("mov r0, {0}", out(reg) (*newregs).regs[0]);
        core::arch::asm!("mov r1, {0}", out(reg) (*newregs).regs[1]);
        core::arch::asm!("mov r2, {0}", out(reg) (*newregs).regs[2]);
        core::arch::asm!("mov r3, {0}", out(reg) (*newregs).regs[3]);
        core::arch::asm!("mov r4, {0}", out(reg) (*newregs).regs[4]);
        core::arch::asm!("mov r5, {0}", out(reg) (*newregs).regs[5]);
        core::arch::asm!("mov r6, {0}", out(reg) (*newregs).regs[6]);
        core::arch::asm!("mov r7, {0}", out(reg) (*newregs).regs[7]);
        core::arch::asm!("mov r8, {0}", out(reg) (*newregs).regs[8]);
        core::arch::asm!("mov r9, {0}", out(reg) (*newregs).regs[9]);
        core::arch::asm!("mov r10, {0}", out(reg) (*newregs).regs[10]);
        core::arch::asm!("mov r11, {0}", out(reg) (*newregs).regs[11]);
        core::arch::asm!("mov r12, {0}", out(reg) (*newregs).regs[12]);
        core::arch::asm!("mov r13, {0}", out(reg) (*newregs).regs[13]);
        core::arch::asm!("mov r14, {0}", out(reg) (*newregs).regs[14]);
        core::arch::asm!("mov r15, {0}", out(reg) (*newregs).regs[15]);

        core::arch::asm!("sts pr, {0}", out(reg) (*newregs).pr);
        core::arch::asm!("sts macl, {0}", out(reg) (*newregs).macl);
        core::arch::asm!("sts mach, {0}", out(reg) (*newregs).mach);

        core::arch::asm!("stc gbr, {0}", out(reg) (*newregs).gbr);
        core::arch::asm!("stc sr, {0}", out(reg) (*newregs).sr);

        (*newregs).pc = _THIS_IP_;
    }
}

#[cfg(not(CONFIG_KEXEC_CORE))]
#[inline(always)]
pub fn reserve_crashkernel() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
