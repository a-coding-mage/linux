/* SPDX-License-Identifier: GPL-2.0 */
/* Guest/host pointer-authentication save/restore declarations. */

#![allow(unused_macros)]

/// The assembler implementation is retained verbatim in this documentation
/// because it is consumed by the AArch64 assembler, not by Rust's parser.
///
/// ```text
/// #define PTRAUTH_REG_OFFSET(x) (x - CPU_APIAKEYLO_EL1)
/// ptrauth_save_state(base, reg1, reg2):
///   mrs_s reg1, SYS_APIAKEYLO_EL1; mrs_s reg2, SYS_APIAKEYHI_EL1
///   stp reg1, reg2, [base, #PTRAUTH_REG_OFFSET(CPU_APIAKEYLO_EL1)]
///   mrs_s reg1, SYS_APIBKEYLO_EL1; mrs_s reg2, SYS_APIBKEYHI_EL1
///   stp reg1, reg2, [base, #PTRAUTH_REG_OFFSET(CPU_APIBKEYLO_EL1)]
///   mrs_s reg1, SYS_APDAKEYLO_EL1; mrs_s reg2, SYS_APDAKEYHI_EL1
///   stp reg1, reg2, [base, #PTRAUTH_REG_OFFSET(CPU_APDAKEYLO_EL1)]
///   mrs_s reg1, SYS_APDBKEYLO_EL1; mrs_s reg2, SYS_APDBKEYHI_EL1
///   stp reg1, reg2, [base, #PTRAUTH_REG_OFFSET(CPU_APDBKEYLO_EL1)]
///   mrs_s reg1, SYS_APGAKEYLO_EL1; mrs_s reg2, SYS_APGAKEYHI_EL1
///   stp reg1, reg2, [base, #PTRAUTH_REG_OFFSET(CPU_APGAKEYLO_EL1)]
/// ptrauth_restore_state(base, reg1, reg2):
///   ldp/msr each of APIA, APIB, APDA, APDB, and APGA low/high key pair
/// ptrauth_switch_to_guest(g_ctxt, reg1, reg2, reg3):
///   if ARM64_HAS_ADDRESS_AUTH and (hcr_el2 & (HCR_API | HCR_APK)) != 0,
///   restore the guest keys at g_ctxt + CPU_APIAKEYLO_EL1.
/// ptrauth_switch_to_hyp(g_ctxt, h_ctxt, reg1, reg2, reg3):
///   under the same condition, save guest keys, restore hyp keys, then isb.
/// ```

/// Save one pointer-authentication key pair into a context.
#[macro_export]
macro_rules! __ptrauth_save_key {
    ($ctxt:expr, $key:ident) => {{
        let mut __val: u64;
        __val = $crate::read_sysreg_s!($crate::concat_sysreg!($key, KEYLO_EL1));
        $crate::ctxt_sys_reg!($ctxt, $crate::concat_sysreg!($key, KEYLO_EL1)) = __val;
        __val = $crate::read_sysreg_s!($crate::concat_sysreg!($key, KEYHI_EL1));
        $crate::ctxt_sys_reg!($ctxt, $crate::concat_sysreg!($key, KEYHI_EL1)) = __val;
    }};
}

/// Save all five pointer-authentication key pairs into a context.
#[macro_export]
macro_rules! ptrauth_save_keys {
    ($ctxt:expr) => {{
        $crate::__ptrauth_save_key!($ctxt, APIA);
        $crate::__ptrauth_save_key!($ctxt, APIB);
        $crate::__ptrauth_save_key!($ctxt, APDA);
        $crate::__ptrauth_save_key!($ctxt, APDB);
        $crate::__ptrauth_save_key!($ctxt, APGA);
    }};
}

/* The following assembler macro names are part of the external interface. */
#[cfg(target_arch = "aarch64")]
#[macro_export]
macro_rules! ptrauth_switch_to_guest { ($($args:tt)*) => { /* assembler macro */ }; }

#[cfg(target_arch = "aarch64")]
#[macro_export]
macro_rules! ptrauth_switch_to_hyp { ($($args:tt)*) => { /* assembler macro */ }; }

#[cfg(not(target_arch = "aarch64"))]
#[macro_export]
macro_rules! ptrauth_switch_to_guest { ($($args:tt)*) => {}; }

#[cfg(not(target_arch = "aarch64"))]
#[macro_export]
macro_rules! ptrauth_switch_to_hyp { ($($args:tt)*) => {}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
