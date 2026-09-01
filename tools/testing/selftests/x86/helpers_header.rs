// SPDX-License-Identifier: GPL-2.0-only
//
// Rust translation of testing/selftests/x86/helpers.h.
// C include dependencies preserved conceptually:
// - <signal.h>
// - <string.h>
// - <asm/processor-flags.h>
// - "kselftest.h"

use core::arch::asm;

pub unsafe fn get_eflags() -> libc::c_ulong {
    let eflags: libc::c_ulong;

    #[cfg(target_arch = "x86_64")]
    {
        unsafe {
            asm!("pushfq; pop {}", out(reg) eflags, options(nomem, preserves_flags));
        }
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        unsafe {
            asm!("pushfd; pop {}", out(reg) eflags, options(nomem, preserves_flags));
        }
    }

    eflags
}

pub unsafe fn set_eflags(eflags: libc::c_ulong) {
    #[cfg(target_arch = "x86_64")]
    {
        unsafe {
            asm!("push {}; popfq", in(reg) eflags, options(nomem, preserves_flags));
        }
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        unsafe {
            asm!("push {}; popfd", in(reg) eflags, options(nomem, preserves_flags));
        }
    }
}

unsafe extern "C" {
    fn ksft_exit_fail_msg(msg: *const libc::c_char) -> !;
}

pub unsafe fn sethandler(
    sig: libc::c_int,
    handler: Option<unsafe extern "C" fn(libc::c_int, *mut libc::siginfo_t, *mut libc::c_void)>,
    flags: libc::c_int,
) {
    let mut sa: libc::sigaction = unsafe { core::mem::zeroed() };

    sa.sa_sigaction = handler.map_or(0, |f| f as usize);
    sa.sa_flags = libc::SA_SIGINFO | flags;
    unsafe {
        libc::sigemptyset(&mut sa.sa_mask);
        if libc::sigaction(sig, &sa, core::ptr::null_mut()) != 0 {
            ksft_exit_fail_msg(c"sigaction failed".as_ptr());
        }
    }
}

pub unsafe fn clearhandler(sig: libc::c_int) {
    let mut sa: libc::sigaction = unsafe { core::mem::zeroed() };

    sa.sa_sigaction = libc::SIG_DFL;
    unsafe {
        libc::sigemptyset(&mut sa.sa_mask);
        if libc::sigaction(sig, &sa, core::ptr::null_mut()) != 0 {
            ksft_exit_fail_msg(c"sigaction failed".as_ptr());
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
