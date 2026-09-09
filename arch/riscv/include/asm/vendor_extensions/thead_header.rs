/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding vendor-extension translation.

/*
 * Extension keys must be strictly less than RISCV_ISA_VENDOR_EXT_MAX.
 */
pub const RISCV_ISA_VENDOR_EXT_XTHEADVECTOR: u32 = 0;

extern "C" {
    pub static mut riscv_isa_vendor_ext_list_thead:
        riscv_isa_vendor_ext_data_list;
}

// CONFIG_RISCV_ISA_VENDOR_EXT_THEAD is a build-time configuration condition.
#[cfg(CONFIG_RISCV_ISA_VENDOR_EXT_THEAD)]
extern "C" {
    pub fn disable_xtheadvector();
}

#[cfg(not(CONFIG_RISCV_ISA_VENDOR_EXT_THEAD))]
#[inline]
pub fn disable_xtheadvector() {}

/* Extension specific helpers */

/*
 * Vector 0.7.1 as used for example on T-Head Xuantie cores, uses an older
 * encoding for vsetvli (ta, ma vs. d1), so provide an instruction for
 * vsetvli t4, x0, e8, m8, d1
 */
pub const THEAD_VSETVLI_T4X0E8M8D1: &str = ".long\t0x00307ed7\n\t";

/*
 * While in theory, the vector-0.7.1 vsb.v and vlb.v result in the same
 * encoding as the standard vse8.v and vle8.v, compilers seem to optimize
 * the call resulting in a different encoding and then using a value for the
 * "mop" field that is not part of vector-0.7.1
 * So encode specific variants for vstate_save and _restore.
 */
pub const THEAD_VSB_V_V0T0: &str = ".long\t0x02028027\n\t";
pub const THEAD_VSB_V_V8T0: &str = ".long\t0x02028427\n\t";
pub const THEAD_VSB_V_V16T0: &str = ".long\t0x02028827\n\t";
pub const THEAD_VSB_V_V24T0: &str = ".long\t0x02028c27\n\t";
pub const THEAD_VLB_V_V0T0: &str = ".long\t0x012028007\n\t";
pub const THEAD_VLB_V_V8T0: &str = ".long\t0x012028407\n\t";
pub const THEAD_VLB_V_V16T0: &str = ".long\t0x012028807\n\t";
pub const THEAD_VLB_V_V24T0: &str = ".long\t0x012028c07\n\t";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
