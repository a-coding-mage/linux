/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from the C header:
// #include <linux/compiler_types.h>
// #include "x86.h"

extern "C" {
    fn kvm_spurious_fault();
}

/*
 * The C macros use asm goto with an exception-table entry. Rust inline
 * assembly has no direct equivalent for asm goto or _ASM_EXTABLE; the fault
 * target is retained as the explicit spurious-fault path below.
 */
macro_rules! svm_asm {
    ($insn:tt $(, $clobber:tt)*) => {{
        unsafe {
            core::arch::asm!(stringify!($insn));
        }
        return;
    }};
}

macro_rules! svm_asm1 {
    ($insn:tt, $op1:expr $(, $clobber:tt)*) => {{
        unsafe {
            core::arch::asm!(concat!(stringify!($insn), " {0}"), in(reg) $op1);
        }
        return;
    }};
}

macro_rules! svm_asm2 {
    ($insn:tt, $op1:expr, $op2:expr $(, $clobber:tt)*) => {{
        unsafe {
            core::arch::asm!(
                concat!(stringify!($insn), " {0}, {1}"),
                in(reg) $op1,
                in(reg) $op2,
            );
        }
        return;
    }};
}

#[inline]
pub unsafe fn clgi() {
    core::arch::asm!("clgi");
}

#[inline]
pub unsafe fn stgi() {
    core::arch::asm!("stgi");
}

#[inline]
pub unsafe fn invlpga(addr: usize, asid: u32) {
    core::arch::asm!(
        "invlpga {addr}, {asid}",
        addr = in("rax") addr,
        asid = in("rcx") asid,
    );
}

/*
 * Despite being a physical address, the portion of rAX that is consumed by
 * VMSAVE, VMLOAD, etc... is still controlled by the effective address size,
 * hence 'unsigned long' instead of 'hpa_t'.
 */
#[inline(always)]
pub unsafe fn vmsave(pa: usize) {
    core::arch::asm!(
        "vmsave {pa}",
        pa = in("rax") pa,
        options(nostack),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
