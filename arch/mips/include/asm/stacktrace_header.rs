/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding architecture and kernel headers:
// asm/ptrace.h, asm/asm.h, and linux/stringify.h.

#[cfg(feature = "CONFIG_KALLSYMS")]
extern "C" {
    pub static mut raw_show_trace: ::core::ffi::c_int;
    pub fn unwind_stack(
        task: *mut task_struct,
        sp: *mut ::core::ffi::c_ulong,
        pc: ::core::ffi::c_ulong,
        ra: *mut ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong;
    pub fn unwind_stack_by_address(
        stack_page: ::core::ffi::c_ulong,
        sp: *mut ::core::ffi::c_ulong,
        pc: ::core::ffi::c_ulong,
        ra: *mut ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong;
}

#[cfg(not(feature = "CONFIG_KALLSYMS"))]
pub const raw_show_trace: ::core::ffi::c_int = 1;

#[cfg(not(feature = "CONFIG_KALLSYMS"))]
#[inline(always)]
pub unsafe fn unwind_stack(
    _task: *mut task_struct,
    _sp: *mut ::core::ffi::c_ulong,
    _pc: ::core::ffi::c_ulong,
    _ra: *mut ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    0
}

// The following stringify-based constants preserve the architecture macros.
#[allow(unused_macros)]
macro_rules! STR_PTR_LA { () => { stringify!(PTR_LA) }; }
#[allow(unused_macros)]
macro_rules! STR_LONG_S { () => { stringify!(LONG_S) }; }
#[allow(unused_macros)]
macro_rules! STR_LONG_L { () => { stringify!(LONG_L) }; }
#[allow(unused_macros)]
macro_rules! STR_LONGSIZE { () => { stringify!(LONGSIZE) }; }

#[allow(unused_macros)]
macro_rules! STORE_ONE_REG {
    ($r:literal) => {
        concat!(STR_LONG_S!(), " $", stringify!($r), ",(", STR_LONGSIZE!(), "*", stringify!($r), ")(%1)\\n\\t")
    };
}

#[inline(always)]
pub unsafe fn prepare_frametrace(regs: *mut pt_regs) {
    #[cfg(not(feature = "CONFIG_KALLSYMS"))]
    {
        /* Remove garbage in regs, especially function addresses, before the raw backtrace. */
        ::core::ptr::write_bytes(regs, 0, 1);
    }

    ::core::arch::asm!(
        ".set push\\n\\t",
        ".set noat\\n\\t",
        /* Store $1 so we can use it. */
        concat!(STR_LONG_S!(), " $1,", STR_LONGSIZE!(), "(%1)\\n\\t"),
        /* Store the PC. */
        concat!("1: ", STR_PTR_LA!(), " $1, 1b\\n\\t"),
        concat!(STR_LONG_S!(), " $1,%0\\n\\t"),
        STORE_ONE_REG!(2), STORE_ONE_REG!(3), STORE_ONE_REG!(4), STORE_ONE_REG!(5),
        STORE_ONE_REG!(6), STORE_ONE_REG!(7), STORE_ONE_REG!(8), STORE_ONE_REG!(9),
        STORE_ONE_REG!(10), STORE_ONE_REG!(11), STORE_ONE_REG!(12), STORE_ONE_REG!(13),
        STORE_ONE_REG!(14), STORE_ONE_REG!(15), STORE_ONE_REG!(16), STORE_ONE_REG!(17),
        STORE_ONE_REG!(18), STORE_ONE_REG!(19), STORE_ONE_REG!(20), STORE_ONE_REG!(21),
        STORE_ONE_REG!(22), STORE_ONE_REG!(23), STORE_ONE_REG!(24), STORE_ONE_REG!(25),
        STORE_ONE_REG!(26), STORE_ONE_REG!(27), STORE_ONE_REG!(28), STORE_ONE_REG!(29),
        STORE_ONE_REG!(30), STORE_ONE_REG!(31),
        /* Restore $1. */
        concat!(STR_LONG_L!(), " $1,", STR_LONGSIZE!(), "(%1)\\n\\t"),
        ".set pop\\n\\t",
        inout(reg) (*regs).cp0_epc => _,
        in(reg) (*regs).regs,
        options(nostack)
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
