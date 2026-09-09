/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1991, 1992  Linus Torvalds
 * Copyright (C) 1994 - 2000, 2006  Ralf Baechle
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 * Copyright (C) 2016, Imagination Technologies Ltd.
 */

// Dependencies supplied by the surrounding kernel translation.

/* 32-bit compatibility types */
pub type __sighandler32_t = ::core::ffi::c_uint;
pub type vfptr_t = unsafe extern "C" fn();

/*
 * Atomically swap in the new signal mask, and wait for a signal.
 */
pub unsafe fn sys32_sigsuspend(uset: *mut compat_sigset_t) -> ::core::ffi::c_int {
    compat_sys_rt_sigsuspend(uset, core::mem::size_of::<compat_sigset_t>())
}

pub unsafe fn sys_32_sigaction(
    sig: ::core::ffi::c_long,
    act: *const compat_sigaction,
    oact: *mut compat_sigaction,
) -> ::core::ffi::c_int {
    let mut new_ka: k_sigaction = core::mem::zeroed();
    let mut old_ka: k_sigaction = core::mem::zeroed();
    let mut ret: ::core::ffi::c_int;
    let mut err: ::core::ffi::c_int = 0;

    if !act.is_null() {
        let mut mask: old_sigset_t = 0;
        let mut handler: i32 = 0;

        if !access_ok(act as *const _, core::mem::size_of::<compat_sigaction>()) {
            return -EFAULT;
        }
        err |= __get_user(&mut handler, &(*act).sa_handler);
        new_ka.sa.sa_handler = handler as i64 as *mut _;
        err |= __get_user(&mut new_ka.sa.sa_flags, &(*act).sa_flags);
        err |= __get_user(&mut mask, &(*act).sa_mask.sig[0]);
        if err != 0 {
            return -EFAULT;
        }

        siginitset(&mut new_ka.sa.sa_mask, mask);
    }

    ret = do_sigaction(
        sig,
        if !act.is_null() { &mut new_ka } else { core::ptr::null_mut() },
        if !oact.is_null() { &mut old_ka } else { core::ptr::null_mut() },
    );

    if ret == 0 && !oact.is_null() {
        if !access_ok(oact as *const _, core::mem::size_of::<compat_sigaction>()) {
            return -EFAULT;
        }
        err |= __put_user(old_ka.sa.sa_flags, &mut (*oact).sa_flags);
        err |= __put_user(old_ka.sa.sa_handler as u64 as u32, &mut (*oact).sa_handler);
        err |= __put_user(old_ka.sa.sa_mask.sig[0], (*oact).sa_mask.sig.as_mut_ptr());
        err |= __put_user(0, &mut (*oact).sa_mask.sig[1]);
        err |= __put_user(0, &mut (*oact).sa_mask.sig[2]);
        err |= __put_user(0, &mut (*oact).sa_mask.sig[3]);
        if err != 0 {
            return -EFAULT;
        }
    }

    ret
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
