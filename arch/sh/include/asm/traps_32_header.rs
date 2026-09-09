/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies:
// #include <linux/types.h>
// #include <asm/mmu.h>

// CONFIG_CPU_HAS_SR_RB selects the corresponding assembly sequence in the
// original header.
#[cfg(CONFIG_CPU_HAS_SR_RB)]
#[inline]
pub unsafe fn lookup_exception_vector() -> ::core::ffi::c_ulong {
    let mut vec: ::core::ffi::c_ulong;
    ::core::arch::asm!(
        "stc r2_bank, {0}",
        out(reg) vec,
        options(nostack, preserves_flags)
    );
    vec
}

#[cfg(not(CONFIG_CPU_HAS_SR_RB))]
#[inline]
pub unsafe fn lookup_exception_vector() -> ::core::ffi::c_ulong {
    let mut vec: ::core::ffi::c_ulong;
    ::core::arch::asm!(
        "mov r4, {0}",
        out(reg) vec,
        options(nostack, preserves_flags)
    );
    vec
}

#[inline]
pub unsafe fn trigger_address_error() {
    ::core::arch::asm!(
        "ldc {0}, sr",
        "mov.l @{1}, {0}",
        in(reg) 0x10000000_u32,
        in(reg) 0x80000001_u32,
        options(nostack)
    );
}

// asmlinkage void do_address_error(struct pt_regs *regs,
//                                   unsigned long writeaccess,
//                                   unsigned long address);
extern "C" {
    pub fn do_address_error(
        regs: *mut pt_regs,
        writeaccess: ::core::ffi::c_ulong,
        address: ::core::ffi::c_ulong,
    );
    pub fn do_page_fault(
        regs: *mut pt_regs,
        error_code: ::core::ffi::c_ulong,
        address: ::core::ffi::c_ulong,
    );
    pub fn do_divide_error(r4: ::core::ffi::c_ulong);
    pub fn do_reserved_inst();
    pub fn do_illegal_slot_inst();
    pub fn do_exception_error();
}

// struct pt_regs is supplied by the translated kernel dependencies.
#[allow(non_camel_case_types)]
pub type pt_regs = crate::pt_regs;

#[macro_export]
macro_rules! BUILD_TRAP_HANDLER {
    ($handler:ident) => {
        extern "C" {
            pub fn $handler(
                r4: ::core::ffi::c_ulong,
                r5: ::core::ffi::c_ulong,
                r6: ::core::ffi::c_ulong,
                r7: ::core::ffi::c_ulong,
                __regs: $crate::pt_regs,
            );
        }
    };
}

// C macro TRAP_HANDLER_DECL:
// struct pt_regs *regs = RELOC_HIDE(&__regs, 0);
// unsigned int vec = regs->tra;
// (void)vec;
#[macro_export]
macro_rules! TRAP_HANDLER_DECL {
    ($regs:ident, $vec:ident, $regs_storage:ident) => {
        let $regs: *mut $crate::pt_regs = &mut $regs_storage;
        let $vec: u32 = unsafe { (*$regs).tra };
        let _ = $vec;
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
