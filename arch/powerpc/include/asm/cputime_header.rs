/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Definitions for measuring cputime on powerpc machines.
 *
 * Copyright (C) 2006 Paul Mackerras, IBM Corp.
 *
 * If CONFIG_VIRT_CPU_ACCOUNTING_NATIVE is enabled, cpu time is measured in
 * the same units as the timebase. Otherwise it is measured in jiffies using
 * the generic definitions.
 */

/* C header dependencies are supplied by other translated files. */

#[cfg(feature = "CONFIG_VIRT_CPU_ACCOUNTING_NATIVE")]
#[cfg(feature = "__KERNEL__")]
pub fn cputime_to_nsecs(cputime: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    tb_to_ns(cputime)
}

/*
 * PPC64 uses PACA, which is task independent, for storing accounting data,
 * while PPC32 uses struct thread_info. Therefore, at task switch the
 * accounting data has to be populated in the new task.
 *
 * These macros preserve the source-level get_accounting/raw_get_accounting
 * interfaces; the referenced platform symbols are supplied externally.
 */
#[cfg(all(feature = "CONFIG_VIRT_CPU_ACCOUNTING_NATIVE", feature = "__KERNEL__", feature = "CONFIG_PPC64"))]
macro_rules! get_accounting {
    ($tsk:expr) => {
        &mut (*get_paca()).accounting
    };
}

#[cfg(all(feature = "CONFIG_VIRT_CPU_ACCOUNTING_NATIVE", feature = "__KERNEL__", feature = "CONFIG_PPC64"))]
macro_rules! raw_get_accounting {
    ($tsk:expr) => {
        &mut (*local_paca).accounting
    };
}

#[cfg(all(feature = "CONFIG_VIRT_CPU_ACCOUNTING_NATIVE", feature = "__KERNEL__", not(feature = "CONFIG_PPC64")))]
macro_rules! get_accounting {
    ($tsk:expr) => {
        &mut (*task_thread_info($tsk)).accounting
    };
}

#[cfg(all(feature = "CONFIG_VIRT_CPU_ACCOUNTING_NATIVE", feature = "__KERNEL__", not(feature = "CONFIG_PPC64")))]
macro_rules! raw_get_accounting {
    ($tsk:expr) => {
        get_accounting!($tsk)
    };
}

/* account_cpu_user_entry/exit runs unreconciled, so it cannot trace or use
 * get_paca(). */
#[cfg(all(feature = "CONFIG_VIRT_CPU_ACCOUNTING_NATIVE", feature = "__KERNEL__"))]
#[inline]
pub unsafe fn account_cpu_user_entry() {
    let tb: ::core::ffi::c_ulong = mftb();
    let acct = raw_get_accounting!(current);

    (*acct).utime = (*acct).utime.wrapping_add(tb.wrapping_sub((*acct).starttime_user));
    (*acct).starttime = tb;
}

#[cfg(all(feature = "CONFIG_VIRT_CPU_ACCOUNTING_NATIVE", feature = "__KERNEL__"))]
#[inline]
pub unsafe fn account_cpu_user_exit() {
    let tb: ::core::ffi::c_ulong = mftb();
    let acct = raw_get_accounting!(current);

    (*acct).stime = (*acct).stime.wrapping_add(tb.wrapping_sub((*acct).starttime));
    (*acct).starttime_user = tb;
}

#[cfg(all(feature = "CONFIG_VIRT_CPU_ACCOUNTING_NATIVE", feature = "__KERNEL__"))]
#[inline]
pub unsafe fn account_stolen_time() {
    /* CONFIG_PPC_SPLPAR */
    #[cfg(feature = "CONFIG_PPC_SPLPAR")]
    if firmware_has_feature(FW_FEATURE_SPLPAR) {
        let lp = (*local_paca).lppaca_ptr;

        if (*local_paca).dtl_ridx != be64_to_cpu((*lp).dtl_idx) {
            pseries_accumulate_stolen_time();
        }
    }
}

#[cfg(not(feature = "CONFIG_VIRT_CPU_ACCOUNTING_NATIVE"))]
#[inline]
pub fn account_cpu_user_entry() {}

#[cfg(not(feature = "CONFIG_VIRT_CPU_ACCOUNTING_NATIVE"))]
#[inline]
pub fn account_cpu_user_exit() {}

#[cfg(not(feature = "CONFIG_VIRT_CPU_ACCOUNTING_NATIVE"))]
#[inline]
pub unsafe fn account_stolen_time() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
