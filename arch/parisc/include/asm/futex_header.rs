/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the Linux futex, uaccess, atomic, and errno
// interfaces are intentionally referenced here rather than reimplemented.

/* The following has to match the LWS code in syscall.S.  We have
 * 256 four-word locks. We use bits 20-27 of the futex virtual
 * address for the hash index.
 */

#[inline]
unsafe fn _futex_hash_index(ua: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    (ua >> 2) & 0x3fc
}

#[inline]
unsafe fn _futex_spin_lock_irqsave(s: *mut arch_spinlock_t, flags: *mut ::core::ffi::c_ulong) {
    local_irq_save(*flags);
    arch_spin_lock(s);
}

#[inline]
unsafe fn _futex_spin_unlock_irqrestore(
    s: *mut arch_spinlock_t,
    flags: *mut ::core::ffi::c_ulong,
) {
    arch_spin_unlock(s);
    local_irq_restore(*flags);
}

#[inline]
unsafe fn arch_futex_atomic_op_inuser(
    op: ::core::ffi::c_int,
    oparg: ::core::ffi::c_int,
    oval: *mut ::core::ffi::c_int,
    uaddr: *mut u32,
) -> ::core::ffi::c_int {
    unsafe extern "C" {
        static mut lws_lock_start: u32;
    }

    let ua = uaddr as ::core::ffi::c_ulong;
    let s: *mut arch_spinlock_t =
        (&raw mut lws_lock_start).add(_futex_hash_index(ua) as usize) as *mut arch_spinlock_t;
    let mut flags: ::core::ffi::c_ulong = 0;
    let mut oldval: ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int;
    let mut tmp: u32;

    _futex_spin_lock_irqsave(s, &mut flags);

    /* Return -EFAULT if we encounter a page fault or COW break */
    if unlikely(get_user(&mut oldval, uaddr) != 0) {
        _futex_spin_unlock_irqrestore(s, &mut flags);
        return -EFAULT;
    }

    ret = 0;
    tmp = oldval as u32;

    match op {
        FUTEX_OP_SET => tmp = oparg as u32,
        FUTEX_OP_ADD => tmp = tmp.wrapping_add(oparg as u32),
        FUTEX_OP_OR => tmp |= oparg as u32,
        FUTEX_OP_ANDN => tmp &= !(oparg as u32),
        FUTEX_OP_XOR => tmp ^= oparg as u32,
        _ => {
            _futex_spin_unlock_irqrestore(s, &mut flags);
            return -ENOSYS;
        }
    }

    if unlikely(put_user(tmp, uaddr) != 0) {
        ret = -EFAULT;
    }

    _futex_spin_unlock_irqrestore(s, &mut flags);

    if ret == 0 {
        *oval = oldval;
    }

    ret
}

#[inline]
unsafe fn futex_atomic_cmpxchg_inatomic(
    uval: *mut u32,
    uaddr: *mut u32,
    oldval: u32,
    newval: u32,
) -> ::core::ffi::c_int {
    unsafe extern "C" {
        static mut lws_lock_start: u32;
    }

    let ua = uaddr as ::core::ffi::c_ulong;
    let s: *mut arch_spinlock_t =
        (&raw mut lws_lock_start).add(_futex_hash_index(ua) as usize) as *mut arch_spinlock_t;
    let mut val: u32;
    let mut flags: ::core::ffi::c_ulong = 0;

    if !access_ok(uaddr, core::mem::size_of::<u32>()) {
        return -EFAULT;
    }

    /* HPPA has no cmpxchg in hardware and therefore the
     * best we can do here is use an array of locks. The
     * lock selected is based on a hash of the virtual
     * address of the futex. This should scale to a couple
     * of CPUs.
     */

    _futex_spin_lock_irqsave(s, &mut flags);
    if unlikely(get_user(&mut val, uaddr) != 0) {
        _futex_spin_unlock_irqrestore(s, &mut flags);
        return -EFAULT;
    }

    if val == oldval && unlikely(put_user(newval, uaddr) != 0) {
        _futex_spin_unlock_irqrestore(s, &mut flags);
        return -EFAULT;
    }

    *uval = val;
    _futex_spin_unlock_irqrestore(s, &mut flags);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
