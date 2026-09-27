// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Do sleep inside a spin-lock
 *  Copyright (c) 1999 by Takashi Iwai <tiwai@suse.de>
 */

// C dependencies:
// #include <linux/export.h>
// #include <sound/core.h>
// #include "seq_lock.h"

extern "C" {
    fn atomic_read(v: *const snd_use_lock_t) -> kernel::ffi::c_int;
    fn pr_warn(fmt: *const kernel::ffi::c_char, ...);
    fn schedule_timeout_uninterruptible(timeout: kernel::ffi::c_long) -> kernel::ffi::c_long;
}

pub type snd_use_lock_t = kernel::ffi::c_void;

extern "C" {
    static HZ: kernel::ffi::c_int;
}

/* wait until all locks are released */
#[no_mangle]
pub unsafe extern "C" fn snd_use_lock_sync_helper(
    lockp: *mut snd_use_lock_t,
    file: *const kernel::ffi::c_char,
    line: kernel::ffi::c_int,
) {
    let mut warn_count: kernel::ffi::c_int = 5 * HZ;

    if atomic_read(lockp as *const snd_use_lock_t) < 0 {
        pr_warn(
            b"ALSA: seq_lock: lock trouble [counter = %d] in %s:%d\n\0".as_ptr()
                as *const kernel::ffi::c_char,
            atomic_read(lockp as *const snd_use_lock_t),
            file,
            line,
        );
        return;
    }
    while atomic_read(lockp as *const snd_use_lock_t) > 0 {
        let old_warn_count = warn_count;
        warn_count -= 1;
        if old_warn_count == 0 {
            pr_warn(
                b"ALSA: seq_lock: waiting [%d left] in %s:%d\n\0".as_ptr()
                    as *const kernel::ffi::c_char,
                atomic_read(lockp as *const snd_use_lock_t),
                file,
                line,
            );
        }
        schedule_timeout_uninterruptible(1);
    }
}

// EXPORT_SYMBOL(snd_use_lock_sync_helper);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
