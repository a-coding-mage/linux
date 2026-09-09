/*
 * Copyright (C) 2014 Stefan Kristiansson <stefan.kristiansson@saunalahti.fi>
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2.  This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 */

// Translated from <linux/types.h>. `atomic_t`, `READ_ONCE`, and `WRITE_ONCE`
// are supplied by the surrounding kernel translation.

#[inline(always)]
pub unsafe fn arch_atomic_add(i: i32, v: *mut atomic_t) {
    let mut tmp: i32;
    core::arch::asm!(
        "1: l.lwa {tmp},0({ptr})",
        "l.add {tmp},{tmp},{i}",
        "l.swa 0({ptr}),{tmp}",
        "l.bnf 1b",
        "l.nop",
        tmp = out(reg) tmp,
        ptr = in(reg) core::ptr::addr_of_mut!((*v).counter),
        i = in(reg) i,
        options(nostack),
    );
}

#[inline(always)]
pub unsafe fn arch_atomic_sub(i: i32, v: *mut atomic_t) {
    let mut tmp: i32;
    core::arch::asm!(
        "1: l.lwa {tmp},0({ptr})",
        "l.sub {tmp},{tmp},{i}",
        "l.swa 0({ptr}),{tmp}",
        "l.bnf 1b",
        "l.nop",
        tmp = out(reg) tmp,
        ptr = in(reg) core::ptr::addr_of_mut!((*v).counter),
        i = in(reg) i,
        options(nostack),
    );
}

macro_rules! atomic_fetch_op {
    ($name:ident, $instruction:literal) => {
        #[inline(always)]
        pub unsafe fn $name(i: i32, v: *mut atomic_t) -> i32 {
            let mut old: i32;
            let mut tmp: i32;
            core::arch::asm!(
                "1: l.lwa {old},0({ptr})",
                concat!("l.", $instruction, " {tmp},{old},{i}"),
                "l.swa 0({ptr}),{tmp}",
                "l.bnf 1b",
                "l.nop",
                old = out(reg) old,
                tmp = out(reg) tmp,
                ptr = in(reg) core::ptr::addr_of_mut!((*v).counter),
                i = in(reg) i,
                options(nostack),
            );
            old
        }
    };
}

macro_rules! atomic_op_return {
    ($name:ident, $instruction:literal) => {
        #[inline(always)]
        pub unsafe fn $name(i: i32, v: *mut atomic_t) -> i32 {
            let mut tmp: i32;
            core::arch::asm!(
                "1: l.lwa {tmp},0({ptr})",
                concat!("l.", $instruction, " {tmp},{tmp},{i}"),
                "l.swa 0({ptr}),{tmp}",
                "l.bnf 1b",
                "l.nop",
                tmp = inout(reg) tmp,
                ptr = in(reg) core::ptr::addr_of_mut!((*v).counter),
                i = in(reg) i,
                options(nostack),
            );
            tmp
        }
    };
}

#[inline(always)]
pub unsafe fn arch_atomic_and(i: i32, v: *mut atomic_t) {
    let mut tmp: i32;
    core::arch::asm!(
        "1: l.lwa {tmp},0({ptr})", "l.and {tmp},{tmp},{i}",
        "l.swa 0({ptr}),{tmp}", "l.bnf 1b", "l.nop",
        tmp = out(reg) tmp, ptr = in(reg) core::ptr::addr_of_mut!((*v).counter),
        i = in(reg) i, options(nostack),
    );
}

#[inline(always)]
pub unsafe fn arch_atomic_or(i: i32, v: *mut atomic_t) {
    let mut tmp: i32;
    core::arch::asm!(
        "1: l.lwa {tmp},0({ptr})", "l.or {tmp},{tmp},{i}",
        "l.swa 0({ptr}),{tmp}", "l.bnf 1b", "l.nop",
        tmp = out(reg) tmp, ptr = in(reg) core::ptr::addr_of_mut!((*v).counter),
        i = in(reg) i, options(nostack),
    );
}

#[inline(always)]
pub unsafe fn arch_atomic_xor(i: i32, v: *mut atomic_t) {
    let mut tmp: i32;
    core::arch::asm!(
        "1: l.lwa {tmp},0({ptr})", "l.xor {tmp},{tmp},{i}",
        "l.swa 0({ptr}),{tmp}", "l.bnf 1b", "l.nop",
        tmp = out(reg) tmp, ptr = in(reg) core::ptr::addr_of_mut!((*v).counter),
        i = in(reg) i, options(nostack),
    );
}

atomic_fetch_op!(arch_atomic_fetch_add, "add");
atomic_fetch_op!(arch_atomic_fetch_sub, "sub");
atomic_fetch_op!(arch_atomic_fetch_and, "and");
atomic_fetch_op!(arch_atomic_fetch_or, "or");
atomic_fetch_op!(arch_atomic_fetch_xor, "xor");

atomic_op_return!(arch_atomic_add_return, "add");
atomic_op_return!(arch_atomic_sub_return, "sub");

#[inline(always)]
pub unsafe fn arch_atomic_fetch_add_unless(v: *mut atomic_t, a: i32, u: i32) -> i32 {
    let mut old: i32;
    let mut tmp: i32;
    core::arch::asm!(
        "1: l.lwa {old},0({ptr})",
        "l.sfeq {old},{u}",
        "l.bf 2f",
        "l.add {tmp},{old},{a}",
        "l.swa 0({ptr}),{tmp}",
        "l.bnf 1b",
        "l.nop",
        "2:",
        old = out(reg) old,
        tmp = out(reg) tmp,
        ptr = in(reg) core::ptr::addr_of_mut!((*v).counter),
        a = in(reg) a,
        u = in(reg) u,
        options(nostack),
    );
    old
}

#[inline(always)]
pub unsafe fn arch_atomic_read(v: *const atomic_t) -> i32 {
    core::ptr::read_volatile(core::ptr::addr_of!((*v).counter))
}

#[inline(always)]
pub unsafe fn arch_atomic_set(v: *mut atomic_t, i: i32) {
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*v).counter), i)
}

// The original header includes <asm/cmpxchg.h>; its declarations are supplied
// by the surrounding translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
