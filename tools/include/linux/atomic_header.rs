/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency intent from C header: #include <asm/atomic.h> */

extern "C" {
    pub fn atomic_long_set(v: *mut atomic_long_t, i: libc::c_long);
}

/* atomic_cmpxchg_relaxed
 *
 * C macro aliases when atomic_cmpxchg_relaxed is not already defined:
 *   atomic_cmpxchg_relaxed -> atomic_cmpxchg
 *   atomic_cmpxchg_release -> atomic_cmpxchg
 */
pub unsafe fn atomic_cmpxchg_relaxed(ptr: *mut atomic_t, old: libc::c_int, new: libc::c_int) -> libc::c_int {
    unsafe { atomic_cmpxchg(ptr, old, new) }
}

pub unsafe fn atomic_cmpxchg_release(ptr: *mut atomic_t, old: libc::c_int, new: libc::c_int) -> libc::c_int {
    unsafe { atomic_cmpxchg(ptr, old, new) }
}

pub unsafe fn atomic_try_cmpxchg(ptr: *mut atomic_t, oldp: *mut libc::c_int, new: libc::c_int) -> bool {
    let mut ret: libc::c_int;
    let mut old: libc::c_int = unsafe { *oldp };

    ret = unsafe { atomic_cmpxchg(ptr, old, new) };
    if ret != old {
        unsafe {
            *oldp = ret;
        }
    }
    ret == old
}

pub unsafe fn atomic_inc_unless_negative(v: *mut atomic_t) -> bool {
    let mut c: libc::c_int = unsafe { atomic_read(v) };

    loop {
        if unlikely(c < 0) {
            return false;
        }
        if unsafe { atomic_try_cmpxchg(v, &mut c, c.wrapping_add(1)) } {
            break;
        }
    }

    true
}
