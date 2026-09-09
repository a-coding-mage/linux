/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <keys/trusted-type.h>
// #include <linux/bitops.h>
// #include <linux/printk.h>

extern "C" {
    pub static mut pkwm_trusted_key_ops: trusted_key_ops;
}

#[repr(C)]
pub struct trusted_pkwm_options {
    pub wrap_flags: u16,
}

#[inline]
pub unsafe fn dump_options(o: *mut trusted_key_options) {
    let pkwm: *const trusted_pkwm_options = (*o).private as *const trusted_pkwm_options;
    let sb_audit_or_enforce_bit: bool = ((*pkwm).wrap_flags & (1u16 << 0)) != 0;
    let sb_enforce_bit: bool = ((*pkwm).wrap_flags & (1u16 << 1)) != 0;

    if sb_audit_or_enforce_bit {
        pr_debug!("secure boot mode required: audit or enforce");
    } else if sb_enforce_bit {
        pr_debug!("secure boot mode required: enforce");
    } else {
        pr_debug!("secure boot mode required: disabled");
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
