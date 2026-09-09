/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * SVM helper functions
 *
 * Copyright 2018 Anshuman Khandual, IBM Corporation.
 */

/* _ASM_POWERPC_SVM_H */

/* CONFIG_PPC_SVM */
#[cfg(CONFIG_PPC_SVM)]
mod config_ppc_svm {
    use super::*;

    pub(crate) fn is_secure_guest() -> bool {
        unsafe { mfmsr() & MSR_S != 0 }
    }

    unsafe extern "C" {
        pub(crate) fn dtl_cache_ctor(addr: *mut core::ffi::c_void);
    }

    #[macro_export]
    macro_rules! get_dtl_cache_ctor {
        () => {
            if $crate::config_ppc_svm::is_secure_guest() {
                Some($crate::config_ppc_svm::dtl_cache_ctor)
            } else {
                None
            }
        };
    }
}

/* !CONFIG_PPC_SVM */
#[cfg(not(CONFIG_PPC_SVM))]
pub(crate) fn is_secure_guest() -> bool {
    false
}

#[cfg(not(CONFIG_PPC_SVM))]
#[macro_export]
macro_rules! get_dtl_cache_ctor {
    () => {
        None
    };
}

/* External dependencies supplied by other headers. */
unsafe extern "C" {
    fn mfmsr() -> u64;
}

/* MSR_S is supplied by asm/reg.h. */
unsafe extern "C" {
    static MSR_S: u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
