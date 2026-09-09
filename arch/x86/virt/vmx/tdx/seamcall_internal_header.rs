/* SPDX-License-Identifier: GPL-2.0 */
/*
 * SEAMCALL utilities for TDX host-side operations.
 *
 * Provides convenient wrappers around SEAMCALL assembly with retry logic,
 * error reporting and cache coherency tracking.
 *
 * Copyright (C) 2021-2023 Intel Corporation
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external: printk, kernel types, archrandom, processor, and TDX symbols.

extern "C" {
    pub fn __seamcall(fn_: u64, args: *mut tdx_module_args) -> u64;
    pub fn __seamcall_ret(fn_: u64, args: *mut tdx_module_args) -> u64;
    pub fn __seamcall_saved_ret(fn_: u64, args: *mut tdx_module_args) -> u64;
}

pub type sc_func_t = unsafe extern "C" fn(fn_: u64, args: *mut tdx_module_args) -> u64;

#[inline(always)]
pub unsafe fn __seamcall_dirty_cache(
    func: sc_func_t,
    fn_: u64,
    args: *mut tdx_module_args,
) -> u64 {
    lockdep_assert_preemption_disabled!();

    /*
     * SEAMCALLs are made to the TDX module and can generate dirty
     * cachelines of TDX private memory.  Mark cache state incoherent
     * so that the cache can be flushed during kexec.
     *
     * This needs to be done before actually making the SEAMCALL,
     * because kexec-ing CPU could send NMI to stop remote CPUs,
     * in which case even disabling IRQ won't help here.
     */
    this_cpu_write!(cache_state_incoherent, true);

    func(fn_, args)
}

#[inline(always)]
pub unsafe fn sc_retry(func: sc_func_t, fn_: u64, args: *mut tdx_module_args) -> u64 {
    let mut retry: i32 = RDRAND_RETRY_LOOPS;
    let mut ret: u64;

    loop {
        preempt_disable!();
        ret = __seamcall_dirty_cache(func, fn_, args);
        preempt_enable!();
        retry -= 1;
        if ret != TDX_RND_NO_ENTROPY || retry == 0 {
            break;
        }
    }

    ret
}

#[inline(always)]
pub unsafe fn seamcall(fn_: u64, args: *mut tdx_module_args) -> u64 {
    sc_retry(__seamcall, fn_, args)
}

#[inline(always)]
pub unsafe fn seamcall_ret(fn_: u64, args: *mut tdx_module_args) -> u64 {
    sc_retry(__seamcall_ret, fn_, args)
}

#[inline(always)]
pub unsafe fn seamcall_saved_ret(fn_: u64, args: *mut tdx_module_args) -> u64 {
    sc_retry(__seamcall_saved_ret, fn_, args)
}

pub type sc_err_func_t = unsafe extern "C" fn(u64, u64, *mut tdx_module_args);

#[inline]
pub unsafe fn seamcall_err(fn_: u64, err: u64, _args: *mut tdx_module_args) {
    pr_err!("SEAMCALL (0x{:016x}) failed: 0x{:016x}\n", fn_, err);
}

#[inline]
pub unsafe fn seamcall_err_ret(fn_: u64, err: u64, args: *mut tdx_module_args) {
    seamcall_err(fn_, err, args);
    pr_err!(
        "RCX 0x{:016x} RDX 0x{:016x} R08 0x{:016x}\n",
        (*args).rcx,
        (*args).rdx,
        (*args).r8
    );
    pr_err!(
        "R09 0x{:016x} R10 0x{:016x} R11 0x{:016x}\n",
        (*args).r9,
        (*args).r10,
        (*args).r11
    );
}

#[inline(always)]
pub unsafe fn sc_retry_prerr(
    func: sc_func_t,
    err_func: sc_err_func_t,
    fn_: u64,
    args: *mut tdx_module_args,
) -> i32 {
    let sret = sc_retry(func, fn_, args);

    if sret == TDX_SUCCESS {
        return 0;
    }
    if sret == TDX_SEAMCALL_VMFAILINVALID {
        return -ENODEV;
    }
    if sret == TDX_SEAMCALL_GP {
        return -EOPNOTSUPP;
    }
    if sret == TDX_SEAMCALL_UD {
        return -EACCES;
    }

    err_func(fn_, sret, args);
    -EIO
}

#[inline(always)]
pub unsafe fn seamcall_prerr(fn_: u64, args: *mut tdx_module_args) -> i32 {
    sc_retry_prerr(__seamcall, seamcall_err, fn_, args)
}

#[inline(always)]
pub unsafe fn seamcall_prerr_ret(fn_: u64, args: *mut tdx_module_args) -> i32 {
    sc_retry_prerr(__seamcall_ret, seamcall_err_ret, fn_, args)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
