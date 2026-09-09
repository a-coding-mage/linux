/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel headers:
// linux/cpumask.h and uapi/asm/hwprobe.h.

#[cfg(CONFIG_RISCV_ISA_VENDOR_EXT_THEAD)]
unsafe extern "C" {
    pub fn hwprobe_isa_vendor_ext_thead_0(
        pair: *mut riscv_hwprobe,
        cpus: *const cpumask,
    );
}

#[cfg(not(CONFIG_RISCV_ISA_VENDOR_EXT_THEAD))]
#[inline]
pub unsafe fn hwprobe_isa_vendor_ext_thead_0(
    pair: *mut riscv_hwprobe,
    _cpus: *const cpumask,
) {
    (*pair).value = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
