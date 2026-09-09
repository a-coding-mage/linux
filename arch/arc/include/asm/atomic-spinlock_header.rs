/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * Non hardware assisted Atomic-R-M-W
 * Locking would change to irq-disabling only (UP) and spinlocks (SMP)
 *
 * The declarations `atomic_t`, `atomic_ops_lock!`, and
 * `atomic_ops_unlock!` are supplied by the surrounding translation.
 */

pub unsafe fn arch_atomic_set(v: *mut atomic_t, i: i32) {
    /*
     * Independent of hardware support, all of the atomic_xxx() APIs need
     * to follow the same locking rules to make sure that a "hardware"
     * atomic insn (e.g. LD) doesn't clobber an "emulated" atomic insn
     * sequence.
     *
     * Thus atomic_set() despite being 1 insn (and seemingly atomic)
     * requires the locking.
     */
    let mut flags: usize = 0;

    atomic_ops_lock!(flags);
    core::ptr::write_volatile(&mut (*v).counter, i);
    atomic_ops_unlock!(flags);
}

pub unsafe fn arch_atomic_set_release(v: *mut atomic_t, i: i32) {
    arch_atomic_set(v, i);
}

pub unsafe fn arch_atomic_add(i: i32, v: *mut atomic_t) {
    let mut flags: usize = 0;
    atomic_ops_lock!(flags);
    (*v).counter = (*v).counter.wrapping_add(i);
    atomic_ops_unlock!(flags);
}

pub unsafe fn arch_atomic_add_return(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags: usize = 0;
    let mut temp: i32;
    atomic_ops_lock!(flags);
    temp = (*v).counter;
    temp = temp.wrapping_add(i);
    (*v).counter = temp;
    atomic_ops_unlock!(flags);
    temp
}

pub unsafe fn arch_atomic_fetch_add(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags: usize = 0;
    let orig: i32;
    atomic_ops_lock!(flags);
    orig = (*v).counter;
    (*v).counter = (*v).counter.wrapping_add(i);
    atomic_ops_unlock!(flags);
    orig
}

pub unsafe fn arch_atomic_sub(i: i32, v: *mut atomic_t) {
    let mut flags: usize = 0;
    atomic_ops_lock!(flags);
    (*v).counter = (*v).counter.wrapping_sub(i);
    atomic_ops_unlock!(flags);
}

pub unsafe fn arch_atomic_sub_return(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags: usize = 0;
    let mut temp: i32;
    atomic_ops_lock!(flags);
    temp = (*v).counter;
    temp = temp.wrapping_sub(i);
    (*v).counter = temp;
    atomic_ops_unlock!(flags);
    temp
}

pub unsafe fn arch_atomic_fetch_sub(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags: usize = 0;
    let orig: i32;
    atomic_ops_lock!(flags);
    orig = (*v).counter;
    (*v).counter = (*v).counter.wrapping_sub(i);
    atomic_ops_unlock!(flags);
    orig
}

pub unsafe fn arch_atomic_and(i: i32, v: *mut atomic_t) {
    let mut flags: usize = 0;
    atomic_ops_lock!(flags);
    (*v).counter &= i;
    atomic_ops_unlock!(flags);
}

pub unsafe fn arch_atomic_fetch_and(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags: usize = 0;
    let orig: i32;
    atomic_ops_lock!(flags);
    orig = (*v).counter;
    (*v).counter &= i;
    atomic_ops_unlock!(flags);
    orig
}

pub unsafe fn arch_atomic_andnot(i: i32, v: *mut atomic_t) {
    let mut flags: usize = 0;
    atomic_ops_lock!(flags);
    (*v).counter &= !i;
    atomic_ops_unlock!(flags);
}

pub unsafe fn arch_atomic_fetch_andnot(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags: usize = 0;
    let orig: i32;
    atomic_ops_lock!(flags);
    orig = (*v).counter;
    (*v).counter &= !i;
    atomic_ops_unlock!(flags);
    orig
}

pub unsafe fn arch_atomic_or(i: i32, v: *mut atomic_t) {
    let mut flags: usize = 0;
    atomic_ops_lock!(flags);
    (*v).counter |= i;
    atomic_ops_unlock!(flags);
}

pub unsafe fn arch_atomic_fetch_or(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags: usize = 0;
    let orig: i32;
    atomic_ops_lock!(flags);
    orig = (*v).counter;
    (*v).counter |= i;
    atomic_ops_unlock!(flags);
    orig
}

pub unsafe fn arch_atomic_xor(i: i32, v: *mut atomic_t) {
    let mut flags: usize = 0;
    atomic_ops_lock!(flags);
    (*v).counter ^= i;
    atomic_ops_unlock!(flags);
}

pub unsafe fn arch_atomic_fetch_xor(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags: usize = 0;
    let orig: i32;
    atomic_ops_lock!(flags);
    orig = (*v).counter;
    (*v).counter ^= i;
    atomic_ops_unlock!(flags);
    orig
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
