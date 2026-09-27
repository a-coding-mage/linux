/* SPDX-License-Identifier: GPL-2.0 */
/* timer.h: System timer definitions for sun5.
 *
 * Copyright (C) 1997, 2008 David S. Miller (davem@davemloft.net)
 */

// Dependency intent: <uapi/asm/asi.h>, <linux/types.h>, and <linux/init.h>.

/* The most frequently accessed fields should be first,
 * to fit into the same cacheline.
 */
#[repr(C)]
pub struct sparc64_tick_ops {
    pub ticks_per_nsec_quotient: kernel::ffi::c_ulong,
    pub offset: kernel::ffi::c_ulong,
    pub get_tick: Option<unsafe extern "C" fn() -> kernel::ffi::c_ulonglong>,
    pub add_compare: Option<unsafe extern "C" fn(kernel::ffi::c_ulong) -> kernel::ffi::c_int>,
    pub softint_mask: kernel::ffi::c_ulong,
    pub disable_irq: Option<unsafe extern "C" fn()>,
    pub init_tick: Option<unsafe extern "C" fn()>,
    pub add_tick: Option<unsafe extern "C" fn(kernel::ffi::c_ulong) -> kernel::ffi::c_ulong>,
    pub get_frequency: Option<unsafe extern "C" fn() -> kernel::ffi::c_ulong>,
    pub frequency: kernel::ffi::c_ulong,
    pub name: *mut kernel::ffi::c_char,
}

extern "C" {
    pub static mut tick_ops: *mut sparc64_tick_ops;
    pub fn sparc64_get_clock_tick(cpu: kernel::ffi::c_uint) -> kernel::ffi::c_ulong;
    pub fn setup_sparc64_timer();
}

pub const TICK_PRIV_BIT: kernel::ffi::c_ulong = 1u64 << 63;
pub const TICKCMP_IRQ_BIT: kernel::ffi::c_ulong = 1u64 << 63;

pub const HBIRD_STICKCMP_ADDR: kernel::ffi::c_ulong = 0x1fe0000f060;
pub const HBIRD_STICK_ADDR: kernel::ffi::c_ulong = 0x1fe0000f070;

pub const GET_TICK_NINSTR: usize = 13;

#[repr(C)]
pub struct get_tick_patch {
    pub addr: kernel::ffi::c_uint,
    pub tick: [kernel::ffi::c_uint; GET_TICK_NINSTR],
    pub stick: [kernel::ffi::c_uint; GET_TICK_NINSTR],
}

extern "C" {
    pub static mut __get_tick_patch: get_tick_patch;
    pub static mut __get_tick_patch_end: get_tick_patch;
}

#[inline]
pub unsafe fn get_tick() -> kernel::ffi::c_ulong {
    let mut tick: kernel::ffi::c_ulong;
    let mut tmp1: kernel::ffi::c_ulong;
    let mut tmp2: kernel::ffi::c_ulong;

    // The original SPARC inline assembly is preserved verbatim as the
    // required target-specific implementation.  The patch section selects
    // rd %%tick or rd %%asr24 at link/patch time.
    core::arch::asm!(
        "661:\n\tmov 0x1fe, %1\n\tsllx %1, 0x20, %1\n\tsethi %%hi(0xf000), %2\n\tor %2, 0x70, %2\n\tor %1, %2, %1\n\tadd %1, 8, %2\n\tldxa [%2]%3, %0\n\tldxa [%1]%3, %1\n\tldxa [%2]%3, %2\n\tsub %2, %0, %0\n\tbrnz,pn %0, 661b\n\t sllx %2, 32, %2\n\tor %2, %1, %0\n\tsllx %0, 1, %0\n\tsrlx %0, 1, %0\n\t.section .get_tick_patch, \"ax\"\n\t.word 661b\n\tba 1f\n\t rd %%tick, %0\n\t.skip 4 * (%4 - 2)\n\t1:\n\tba 1f\n\t rd %%asr24, %0\n\t.skip 4 * (%4 - 2)\n\t1:\n\t.previous",
        lateout(reg) tick,
        lateout(reg) tmp1,
        lateout(reg) tmp2,
        const ASI_PHYS_BYPASS_EC_E,
        const GET_TICK_NINSTR,
    );

    tick
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
