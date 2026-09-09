/* SPDX-License-Identifier: GPL-2.0 */
// Translated from the SH atomic IRQ header.
// The original include supplies raw_local_irq_save/raw_local_irq_restore.

use core::ffi::c_ulong;

unsafe extern "C" {
    fn raw_local_irq_save(flags: *mut c_ulong);
    fn raw_local_irq_restore(flags: c_ulong);
}

// `atomic_t` is supplied by the surrounding kernel translation.

pub unsafe fn arch_atomic_add(i: i32, v: *mut atomic_t) {
    let mut flags: c_ulong = 0;
    raw_local_irq_save(&mut flags);
    (*v).counter += i;
    raw_local_irq_restore(flags);
}

pub unsafe fn arch_atomic_add_return(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags: c_ulong = 0;
    raw_local_irq_save(&mut flags);
    let mut temp = (*v).counter as c_ulong;
    temp = temp.wrapping_add(i as c_ulong);
    (*v).counter = temp as i32;
    raw_local_irq_restore(flags);
    temp as i32
}

pub unsafe fn arch_atomic_fetch_add(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags: c_ulong = 0;
    raw_local_irq_save(&mut flags);
    let temp = (*v).counter as c_ulong;
    (*v).counter = temp.wrapping_add(i as c_ulong) as i32;
    raw_local_irq_restore(flags);
    temp as i32
}

pub unsafe fn arch_atomic_sub(i: i32, v: *mut atomic_t) {
    let mut flags: c_ulong = 0;
    raw_local_irq_save(&mut flags);
    (*v).counter -= i;
    raw_local_irq_restore(flags);
}

pub unsafe fn arch_atomic_sub_return(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags: c_ulong = 0;
    raw_local_irq_save(&mut flags);
    let mut temp = (*v).counter as c_ulong;
    temp = temp.wrapping_sub(i as c_ulong);
    (*v).counter = temp as i32;
    raw_local_irq_restore(flags);
    temp as i32
}

pub unsafe fn arch_atomic_fetch_sub(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags: c_ulong = 0;
    raw_local_irq_save(&mut flags);
    let temp = (*v).counter as c_ulong;
    (*v).counter = temp.wrapping_sub(i as c_ulong) as i32;
    raw_local_irq_restore(flags);
    temp as i32
}

pub unsafe fn arch_atomic_and(i: i32, v: *mut atomic_t) {
    let mut flags: c_ulong = 0;
    raw_local_irq_save(&mut flags);
    (*v).counter &= i;
    raw_local_irq_restore(flags);
}

pub unsafe fn arch_atomic_fetch_and(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags: c_ulong = 0;
    raw_local_irq_save(&mut flags);
    let temp = (*v).counter;
    (*v).counter &= i;
    raw_local_irq_restore(flags);
    temp
}

pub unsafe fn arch_atomic_or(i: i32, v: *mut atomic_t) {
    let mut flags: c_ulong = 0;
    raw_local_irq_save(&mut flags);
    (*v).counter |= i;
    raw_local_irq_restore(flags);
}

pub unsafe fn arch_atomic_fetch_or(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags: c_ulong = 0;
    raw_local_irq_save(&mut flags);
    let temp = (*v).counter;
    (*v).counter |= i;
    raw_local_irq_restore(flags);
    temp
}

pub unsafe fn arch_atomic_xor(i: i32, v: *mut atomic_t) {
    let mut flags: c_ulong = 0;
    raw_local_irq_save(&mut flags);
    (*v).counter ^= i;
    raw_local_irq_restore(flags);
}

pub unsafe fn arch_atomic_fetch_xor(i: i32, v: *mut atomic_t) -> i32 {
    let mut flags: c_ulong = 0;
    raw_local_irq_save(&mut flags);
    let temp = (*v).counter;
    (*v).counter ^= i;
    raw_local_irq_restore(flags);
    temp
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
