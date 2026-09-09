/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: linux/cpumask.h
// Dependency: uapi/asm/hwprobe.h

#[cfg(feature = "CONFIG_RISCV_ISA_VENDOR_EXT_SIFIVE")]
unsafe extern "C" {
    pub fn hwprobe_isa_vendor_ext_sifive_0(
        pair: *mut riscv_hwprobe,
        cpus: *const cpumask,
    );
}

#[cfg(not(feature = "CONFIG_RISCV_ISA_VENDOR_EXT_SIFIVE"))]
#[inline]
pub unsafe fn hwprobe_isa_vendor_ext_sifive_0(
    pair: *mut riscv_hwprobe,
    _cpus: *const cpumask,
) {
    (*pair).value = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
