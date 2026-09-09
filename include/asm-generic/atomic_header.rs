/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Generic C implementation of atomic counter operations. Do not include in
 * machine independent code.
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 *
 * C dependencies supplied by other headers: atomic_t, arch_cmpxchg,
 * raw_local_irq_save, raw_local_irq_restore, READ_ONCE, and WRITE_ONCE.
 * The CONFIG_SMP conditional is represented by the `CONFIG_SMP` cfg feature.
 */

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn generic_atomic_add_return(i: i32, v: *mut atomic_t) -> i32 {
    let mut c = (*v).counter;
    let mut old;
    loop {
        old = arch_cmpxchg(&mut (*v).counter, c, c.wrapping_add(i));
        if old == c { break; }
        c = old;
    }
    c.wrapping_add(i)
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn generic_atomic_sub_return(i: i32, v: *mut atomic_t) -> i32 {
    let mut c = (*v).counter;
    let mut old;
    loop {
        old = arch_cmpxchg(&mut (*v).counter, c, c.wrapping_sub(i));
        if old == c { break; }
        c = old;
    }
    c.wrapping_sub(i)
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn generic_atomic_fetch_add(i: i32, v: *mut atomic_t) -> i32 {
    let mut c = (*v).counter;
    let mut old;
    loop {
        old = arch_cmpxchg(&mut (*v).counter, c, c.wrapping_add(i));
        if old == c { break; }
        c = old;
    }
    c
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn generic_atomic_fetch_sub(i: i32, v: *mut atomic_t) -> i32 {
    let mut c = (*v).counter;
    let mut old;
    loop {
        old = arch_cmpxchg(&mut (*v).counter, c, c.wrapping_sub(i));
        if old == c { break; }
        c = old;
    }
    c
}

macro_rules! generic_atomic_fetch_bit_op {
    ($name:ident, $op:tt) => {
        #[cfg(feature = "CONFIG_SMP")]
        #[inline]
        pub unsafe fn $name(i: i32, v: *mut atomic_t) -> i32 {
            let mut c = (*v).counter;
            let mut old;
            loop {
                old = arch_cmpxchg(&mut (*v).counter, c, c $op i);
                if old == c { break; }
                c = old;
            }
            c
        }
    };
}
generic_atomic_fetch_bit_op!(generic_atomic_fetch_and, &);
generic_atomic_fetch_bit_op!(generic_atomic_fetch_or, |);
generic_atomic_fetch_bit_op!(generic_atomic_fetch_xor, ^);

macro_rules! generic_atomic_op {
    ($name:ident, $op:tt) => {
        #[cfg(feature = "CONFIG_SMP")]
        #[inline]
        pub unsafe fn $name(i: i32, v: *mut atomic_t) {
            let mut c = (*v).counter;
            let mut old;
            loop {
                old = arch_cmpxchg(&mut (*v).counter, c, c $op i);
                if old == c { break; }
                c = old;
            }
        }
    };
}
generic_atomic_op!(generic_atomic_add, +);
generic_atomic_op!(generic_atomic_sub, -);
generic_atomic_op!(generic_atomic_and, &);
generic_atomic_op!(generic_atomic_or, |);
generic_atomic_op!(generic_atomic_xor, ^);

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn generic_atomic_add_return(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags = 0;
    raw_local_irq_save(&mut flags);
    let ret = (*v).counter.wrapping_add(i);
    (*v).counter = ret;
    raw_local_irq_restore(flags);
    ret
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn generic_atomic_sub_return(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags = 0;
    raw_local_irq_save(&mut flags);
    let ret = (*v).counter.wrapping_sub(i);
    (*v).counter = ret;
    raw_local_irq_restore(flags);
    ret
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn generic_atomic_fetch_add(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags = 0;
    raw_local_irq_save(&mut flags);
    let ret = (*v).counter;
    (*v).counter = ret.wrapping_add(i);
    raw_local_irq_restore(flags);
    ret
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn generic_atomic_fetch_sub(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags = 0;
    raw_local_irq_save(&mut flags);
    let ret = (*v).counter;
    (*v).counter = ret.wrapping_sub(i);
    raw_local_irq_restore(flags);
    ret
}

macro_rules! generic_atomic_fetch_irq_op {
    ($name:ident, $op:tt) => {
        #[cfg(not(feature = "CONFIG_SMP"))]
        #[inline]
        pub unsafe fn $name(i: i32, v: *mut atomic_t) -> i32 {
            let mut flags = 0;
            raw_local_irq_save(&mut flags);
            let ret = (*v).counter;
            (*v).counter = ret $op i;
            raw_local_irq_restore(flags);
            ret
        }
    };
}
generic_atomic_fetch_irq_op!(generic_atomic_fetch_and, &);
generic_atomic_fetch_irq_op!(generic_atomic_fetch_or, |);
generic_atomic_fetch_irq_op!(generic_atomic_fetch_xor, ^);

macro_rules! generic_atomic_irq_op {
    ($name:ident, $op:tt) => {
        #[cfg(not(feature = "CONFIG_SMP"))]
        #[inline]
        pub unsafe fn $name(i: i32, v: *mut atomic_t) {
            let mut flags = 0;
            raw_local_irq_save(&mut flags);
            (*v).counter = (*v).counter $op i;
            raw_local_irq_restore(flags);
        }
    };
}
generic_atomic_irq_op!(generic_atomic_add, +);
generic_atomic_irq_op!(generic_atomic_sub, -);
generic_atomic_irq_op!(generic_atomic_and, &);
generic_atomic_irq_op!(generic_atomic_or, |);
generic_atomic_irq_op!(generic_atomic_xor, ^);

// arch_atomic_* names are C preprocessor aliases to the generic operations.
pub use generic_atomic_add_return as arch_atomic_add_return;
pub use generic_atomic_sub_return as arch_atomic_sub_return;
pub use generic_atomic_fetch_add as arch_atomic_fetch_add;
pub use generic_atomic_fetch_sub as arch_atomic_fetch_sub;
pub use generic_atomic_fetch_and as arch_atomic_fetch_and;
pub use generic_atomic_fetch_or as arch_atomic_fetch_or;
pub use generic_atomic_fetch_xor as arch_atomic_fetch_xor;
pub use generic_atomic_add as arch_atomic_add;
pub use generic_atomic_sub as arch_atomic_sub;
pub use generic_atomic_and as arch_atomic_and;
pub use generic_atomic_or as arch_atomic_or;
pub use generic_atomic_xor as arch_atomic_xor;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
