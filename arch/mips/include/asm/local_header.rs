/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel headers are intentionally
// left external; this file is the direct Rust translation of the MIPS header.

#[repr(C)]
pub struct local_t {
    pub a: atomic_long_t,
}

pub const fn LOCAL_INIT(i: long) -> local_t {
    local_t { a: ATOMIC_LONG_INIT(i) }
}

pub unsafe fn local_read(l: *const local_t) -> long { atomic_long_read(&(*l).a) }
pub unsafe fn local_set(l: *mut local_t, i: long) { atomic_long_set(&mut (*l).a, i) }
pub unsafe fn local_add(i: long, l: *mut local_t) { atomic_long_add(i, &mut (*l).a) }
pub unsafe fn local_sub(i: long, l: *mut local_t) { atomic_long_sub(i, &mut (*l).a) }
pub unsafe fn local_inc(l: *mut local_t) { atomic_long_inc(&mut (*l).a) }
pub unsafe fn local_dec(l: *mut local_t) { atomic_long_dec(&mut (*l).a) }

/* Same as above, but return the result value. */
pub unsafe fn local_add_return(i: long, l: *mut local_t) -> long {
    let mut result: ulong;
    if kernel_uses_llsc {
        let mut temp: ulong;
        // MIPS LL/SC sequence from the original header.  The architecture
        // assembler is supplied by the target build.
        core::arch::asm!(
            "1:",
            "ll {temp}, 0({counter})",
            "addu {result}, {temp}, {i}",
            "sc {result}, 0({counter})",
            "beqz {result}, 1b",
            "addu {result}, {temp}, {i}",
            result = out(reg) result, temp = out(reg) temp,
            counter = in(reg) &mut (*l).a.counter,
            i = in(reg) i,
            options(nostack)
        );
    } else {
        let mut flags: ulong = 0;
        local_irq_save(&mut flags);
        result = (*l).a.counter as ulong;
        result = result.wrapping_add(i as ulong);
        (*l).a.counter = result as long;
        local_irq_restore(flags);
    }
    result as long
}

pub unsafe fn local_sub_return(i: long, l: *mut local_t) -> long {
    let mut result: ulong;
    if kernel_uses_llsc {
        let mut temp: ulong;
        // MIPS LL/SC sequence from the original header.
        core::arch::asm!(
            "1:",
            "ll {temp}, 0({counter})",
            "subu {result}, {temp}, {i}",
            "sc {result}, 0({counter})",
            "beqz {result}, 1b",
            "subu {result}, {temp}, {i}",
            result = out(reg) result, temp = out(reg) temp,
            counter = in(reg) &mut (*l).a.counter,
            i = in(reg) i,
            options(nostack)
        );
    } else {
        let mut flags: ulong = 0;
        local_irq_save(&mut flags);
        result = (*l).a.counter as ulong;
        result = result.wrapping_sub(i as ulong);
        (*l).a.counter = result as long;
        local_irq_restore(flags);
    }
    result as long
}

pub unsafe fn local_cmpxchg(l: *mut local_t, old: long, new: long) -> long {
    cmpxchg_local(&mut (*l).a.counter, old, new)
}

pub unsafe fn local_try_cmpxchg(l: *mut local_t, old: *mut long, new: long) -> bool {
    try_cmpxchg_local(&mut (*l).a.counter, old, new)
}

pub unsafe fn local_xchg(l: *mut local_t, n: long) -> long {
    atomic_long_xchg(&mut (*l).a, n)
}

/** Atomically add `a` unless `l` is already equal to `u`. */
pub unsafe fn local_add_unless(l: *mut local_t, a: long, u: long) -> bool {
    let mut c = local_read(l);
    loop {
        if c == u { return false; }
        if local_try_cmpxchg(l, &mut c, c.wrapping_add(a)) { return true; }
    }
}

pub unsafe fn local_inc_not_zero(l: *mut local_t) -> bool { local_add_unless(l, 1, 0) }
pub unsafe fn local_dec_return(l: *mut local_t) -> long { local_sub_return(1, l) }
pub unsafe fn local_inc_return(l: *mut local_t) -> long { local_add_return(1, l) }
pub unsafe fn local_sub_and_test(i: long, l: *mut local_t) -> bool { local_sub_return(i, l) == 0 }
pub unsafe fn local_inc_and_test(l: *mut local_t) -> bool { local_inc_return(l) == 0 }
pub unsafe fn local_dec_and_test(l: *mut local_t) -> bool { local_sub_return(1, l) == 0 }
pub unsafe fn local_add_negative(i: long, l: *mut local_t) -> bool { local_add_return(i, l) < 0 }

/* Per-cpu local_t operations; these take a variable, not an address. */
pub unsafe fn __local_inc(l: *mut local_t) { (*l).a.counter = (*l).a.counter.wrapping_add(1) }
pub unsafe fn __local_dec(l: *mut local_t) { (*l).a.counter = (*l).a.counter.wrapping_add(1) }
pub unsafe fn __local_add(i: long, l: *mut local_t) { (*l).a.counter = (*l).a.counter.wrapping_add(i) }
pub unsafe fn __local_sub(i: long, l: *mut local_t) { (*l).a.counter = (*l).a.counter.wrapping_sub(i) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
