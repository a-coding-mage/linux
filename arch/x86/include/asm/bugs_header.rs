/* SPDX-License-Identifier: GPL-2.0 */

// C dependency: <asm/processor.h>

#[cfg(all(CONFIG_CPU_SUP_INTEL, CONFIG_X86_32))]
extern "C" {
    pub fn ppro_with_ram_bug() -> ::core::ffi::c_int;
}

#[cfg(not(all(CONFIG_CPU_SUP_INTEL, CONFIG_X86_32)))]
#[inline]
pub fn ppro_with_ram_bug() -> ::core::ffi::c_int {
    0
}

extern "C" {
    pub fn cpu_bugs_smt_update();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
