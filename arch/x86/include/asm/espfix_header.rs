/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_X86_ESPFIX64 build-time condition. */
#[cfg(feature = "CONFIG_X86_ESPFIX64")]
extern "C" {
    /* DECLARE_PER_CPU_READ_MOSTLY(unsigned long, espfix_stack); */
    pub static mut espfix_stack: usize;

    /* DECLARE_PER_CPU_READ_MOSTLY(unsigned long, espfix_waddr); */
    pub static mut espfix_waddr: usize;

    pub fn init_espfix_bsp();
    pub fn init_espfix_ap(cpu: core::ffi::c_int);
}

#[cfg(not(feature = "CONFIG_X86_ESPFIX64"))]
#[inline]
pub fn init_espfix_ap(_cpu: core::ffi::c_int) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
