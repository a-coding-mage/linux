// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 IBM Corporation
 * Author: Nayna Jain
 */

use core::ffi::c_char;

extern "C" {
    fn is_ppc_secureboot_enabled() -> bool;
    fn is_ppc_trustedboot_enabled() -> bool;
    fn set_module_sig_enforced();
}

/*
 * The "secure_rules" are enabled only on "secureboot" enabled systems.
 * These rules verify the file signatures against known good values.
 * The "appraise_type=imasig|modsig" option allows the known good signature
 * to be stored as an xattr or as an appended signature.
 *
 * To avoid duplicate signature verification as much as possible, the IMA
 * policy rule for module appraisal is added only if CONFIG_MODULE_SIG
 * is not enabled.
 */
static SECURE_RULES: &[*const c_char] = &[
    b"appraise func=KEXEC_KERNEL_CHECK appraise_type=imasig|modsig\0".as_ptr() as *const c_char,
    // CONFIG_MODULE_SIG: this entry is present when module signatures are not enabled.
    #[cfg(not(CONFIG_MODULE_SIG))]
    b"appraise func=MODULE_CHECK appraise_type=imasig|modsig\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

/*
 * The "trusted_rules" are enabled only on "trustedboot" enabled systems.
 * These rules add the kexec kernel image and kernel modules file hashes to
 * the IMA measurement list.
 */
static TRUSTED_RULES: &[*const c_char] = &[
    b"measure func=KEXEC_KERNEL_CHECK\0".as_ptr() as *const c_char,
    b"measure func=MODULE_CHECK\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

/*
 * The "secure_and_trusted_rules" contains rules for both the secure boot and
 * trusted boot. The "template=ima-modsig" option includes the appended
 * signature, when available, in the IMA measurement list.
 */
static SECURE_AND_TRUSTED_RULES: &[*const c_char] = &[
    b"measure func=KEXEC_KERNEL_CHECK template=ima-modsig\0".as_ptr() as *const c_char,
    b"measure func=MODULE_CHECK template=ima-modsig\0".as_ptr() as *const c_char,
    b"appraise func=KEXEC_KERNEL_CHECK appraise_type=imasig|modsig\0".as_ptr() as *const c_char,
    // CONFIG_MODULE_SIG: this entry is present when module signatures are not enabled.
    #[cfg(not(CONFIG_MODULE_SIG))]
    b"appraise func=MODULE_CHECK appraise_type=imasig|modsig\0".as_ptr() as *const c_char,
    core::ptr::null(),
];

/*
 * Returns the relevant IMA arch-specific policies based on the system secure
 * boot state.
 */
#[no_mangle]
pub unsafe extern "C" fn arch_get_ima_policy() -> *const *const c_char {
    if is_ppc_secureboot_enabled() {
        set_module_sig_enforced();

        if is_ppc_trustedboot_enabled() {
            SECURE_AND_TRUSTED_RULES.as_ptr()
        } else {
            SECURE_RULES.as_ptr()
        }
    } else if is_ppc_trustedboot_enabled() {
        TRUSTED_RULES.as_ptr()
    } else {
        core::ptr::null()
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
