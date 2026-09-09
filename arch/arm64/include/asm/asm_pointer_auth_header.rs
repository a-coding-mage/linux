/* SPDX-License-Identifier: GPL-2.0 */

// The original header is an AArch64 assembler header.  The macros below
// preserve its conditional availability and expand to equivalent inline
// assembly; constants and feature predicates are supplied by other headers.

#[cfg(CONFIG_ARM64_PTR_AUTH_KERNEL)]
macro_rules! __ptrauth_keys_install_kernel_nosync {
    ($tsk:ident, $tmp1:ident, $tmp2:ident, $tmp3:ident) => {
        unsafe {
            core::arch::asm!(
                concat!("mov ", stringify!($tmp1), ", #THREAD_KEYS_KERNEL\n",
                        "add ", stringify!($tmp1), ", ", stringify!($tsk), ", ", stringify!($tmp1), "\n",
                        "ldp ", stringify!($tmp2), ", ", stringify!($tmp3), ", [", stringify!($tmp1), ", #PTRAUTH_KERNEL_KEY_APIA]\n",
                        "msr_s SYS_APIAKEYLO_EL1, ", stringify!($tmp2), "\n",
                        "msr_s SYS_APIAKEYHI_EL1, ", stringify!($tmp3)),
                options(nostack)
            );
        }
    };
}

#[cfg(not(CONFIG_ARM64_PTR_AUTH_KERNEL))]
macro_rules! __ptrauth_keys_install_kernel_nosync { ($($arg:tt)*) => {}; }

#[cfg(CONFIG_ARM64_PTR_AUTH_KERNEL)]
macro_rules! ptrauth_keys_install_kernel_nosync {
    ($tsk:ident, $tmp1:ident, $tmp2:ident, $tmp3:ident) => {
        #[cfg(ARM64_HAS_ADDRESS_AUTH)]
        { __ptrauth_keys_install_kernel_nosync!($tsk, $tmp1, $tmp2, $tmp3); }
    };
}

#[cfg(not(CONFIG_ARM64_PTR_AUTH_KERNEL))]
macro_rules! ptrauth_keys_install_kernel_nosync { ($($arg:tt)*) => {}; }

#[cfg(CONFIG_ARM64_PTR_AUTH_KERNEL)]
macro_rules! ptrauth_keys_install_kernel {
    ($tsk:ident, $tmp1:ident, $tmp2:ident, $tmp3:ident) => {
        #[cfg(ARM64_HAS_ADDRESS_AUTH)]
        {
            __ptrauth_keys_install_kernel_nosync!($tsk, $tmp1, $tmp2, $tmp3);
            unsafe { core::arch::asm!("isb", options(nostack)); }
        }
    };
}

#[cfg(not(CONFIG_ARM64_PTR_AUTH_KERNEL))]
macro_rules! ptrauth_keys_install_kernel { ($($arg:tt)*) => {}; }

#[cfg(CONFIG_ARM64_PTR_AUTH)]
macro_rules! __ptrauth_keys_install_user {
    ($tsk:ident, $tmp1:ident, $tmp2:ident, $tmp3:ident) => {
        unsafe {
            core::arch::asm!(
                concat!("mov ", stringify!($tmp1), ", #THREAD_KEYS_USER\n",
                        "add ", stringify!($tmp1), ", ", stringify!($tsk), ", ", stringify!($tmp1), "\n",
                        "ldp ", stringify!($tmp2), ", ", stringify!($tmp3), ", [", stringify!($tmp1), ", #PTRAUTH_USER_KEY_APIA]\n",
                        "msr_s SYS_APIAKEYLO_EL1, ", stringify!($tmp2), "\n",
                        "msr_s SYS_APIAKEYHI_EL1, ", stringify!($tmp3)),
                options(nostack)
            );
        }
    };
}

#[cfg(CONFIG_ARM64_PTR_AUTH)]
macro_rules! __ptrauth_keys_init_cpu {
    ($tsk:ident, $tmp1:ident, $tmp2:ident, $tmp3:ident) => {
        unsafe {
            core::arch::asm!(
                concat!("mrs ", stringify!($tmp1), ", id_aa64isar1_el1\n",
                        "ubfx ", stringify!($tmp1), ", ", stringify!($tmp1), ", #ID_AA64ISAR1_EL1_APA_SHIFT, #8\n",
                        "mrs_s ", stringify!($tmp2), ", SYS_ID_AA64ISAR2_EL1\n",
                        "ubfx ", stringify!($tmp2), ", ", stringify!($tmp2), ", #ID_AA64ISAR2_EL1_APA3_SHIFT, #4\n",
                        "orr ", stringify!($tmp1), ", ", stringify!($tmp1), ", ", stringify!($tmp2), "\n",
                        "cbz ", stringify!($tmp1), ".Lno_addr_auth\n",
                        "mov ", stringify!($tmp1), ", #(SCTLR_ELx_ENIA | SCTLR_ELx_ENIB | SCTLR_ELx_ENDA | SCTLR_ELx_ENDB)\n",
                        "mrs ", stringify!($tmp2), ", sctlr_el1\n",
                        "orr ", stringify!($tmp2), ", ", stringify!($tmp2), ", ", stringify!($tmp1), "\n",
                        "msr sctlr_el1, ", stringify!($tmp2), "\n",
                        "isb"),
                options(nostack)
            );
        }
        __ptrauth_keys_install_kernel_nosync!($tsk, $tmp1, $tmp2, $tmp3);
    };
}

#[cfg(CONFIG_ARM64_PTR_AUTH)]
macro_rules! ptrauth_keys_init_cpu {
    ($tsk:ident, $tmp1:ident, $tmp2:ident, $tmp3:ident) => {
        #[cfg(ARM64_HAS_ADDRESS_AUTH)]
        { __ptrauth_keys_init_cpu!($tsk, $tmp1, $tmp2, $tmp3); }
    };
}

#[cfg(not(CONFIG_ARM64_PTR_AUTH))]
macro_rules! ptrauth_keys_install_user { ($($arg:tt)*) => {}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
