// SPDX-License-Identifier: GPL-2.0
// C dependency intent: <stdint.h>, <pthread.h>

use std::mem::MaybeUninit;
use std::ptr;

fn main() {
    let mut barrier = MaybeUninit::<libc::pthread_barrier_t>::uninit();

    unsafe {
        libc::pthread_barrier_init(barrier.as_mut_ptr(), ptr::null(), 1);
        libc::pthread_barrier_wait(barrier.as_mut_ptr());
        std::process::exit(libc::pthread_barrier_destroy(barrier.as_mut_ptr()));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
