/*
 * Copyright (C) 2014 Stefan Kristiansson <stefan.kristiansson@saunalahti.fi>
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2.  This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 */

// Dependency intent: BIT_MASK and BIT_WORD are supplied by the surrounding
// OpenRISC bitops definitions.

#[inline]
pub unsafe fn set_bit(nr: i32, addr: *mut core::ffi::c_ulong) {
    let mask: core::ffi::c_ulong = BIT_MASK(nr);
    let p = (addr as *mut core::ffi::c_ulong).add(BIT_WORD(nr) as usize);
    let mut tmp: core::ffi::c_ulong;

    core::arch::asm!(
        "1: l.lwa {tmp},0({p})",
        "l.or {tmp},{tmp},{mask}",
        "l.swa 0({p}),{tmp}",
        "l.bnf 1b",
        " l.nop",
        tmp = out(reg) tmp,
        p = in(reg) p,
        mask = in(reg) mask,
        options(nostack)
    );
}

#[inline]
pub unsafe fn clear_bit(nr: i32, addr: *mut core::ffi::c_ulong) {
    let mask: core::ffi::c_ulong = BIT_MASK(nr);
    let p = (addr as *mut core::ffi::c_ulong).add(BIT_WORD(nr) as usize);
    let mut tmp: core::ffi::c_ulong;

    core::arch::asm!(
        "1: l.lwa {tmp},0({p})",
        "l.and {tmp},{tmp},{mask}",
        "l.swa 0({p}),{tmp}",
        "l.bnf 1b",
        " l.nop",
        tmp = out(reg) tmp,
        p = in(reg) p,
        mask = in(reg) !mask,
        options(nostack)
    );
}

#[inline]
pub unsafe fn change_bit(nr: i32, addr: *mut core::ffi::c_ulong) {
    let mask: core::ffi::c_ulong = BIT_MASK(nr);
    let p = (addr as *mut core::ffi::c_ulong).add(BIT_WORD(nr) as usize);
    let mut tmp: core::ffi::c_ulong;

    core::arch::asm!(
        "1: l.lwa {tmp},0({p})",
        "l.xor {tmp},{tmp},{mask}",
        "l.swa 0({p}),{tmp}",
        "l.bnf 1b",
        " l.nop",
        tmp = out(reg) tmp,
        p = in(reg) p,
        mask = in(reg) mask,
        options(nostack)
    );
}

#[inline]
pub unsafe fn test_and_set_bit(nr: i32, addr: *mut core::ffi::c_ulong) -> i32 {
    let mask: core::ffi::c_ulong = BIT_MASK(nr);
    let p = (addr as *mut core::ffi::c_ulong).add(BIT_WORD(nr) as usize);
    let mut old: core::ffi::c_ulong;
    let mut tmp: core::ffi::c_ulong;

    core::arch::asm!(
        "1: l.lwa {old},0({p})",
        "l.or {tmp},{old},{mask}",
        "l.swa 0({p}),{tmp}",
        "l.bnf 1b",
        " l.nop",
        old = out(reg) old,
        tmp = out(reg) tmp,
        p = in(reg) p,
        mask = in(reg) mask,
        options(nostack)
    );

    ((old & mask) != 0) as i32
}

#[inline]
pub unsafe fn test_and_clear_bit(nr: i32, addr: *mut core::ffi::c_ulong) -> i32 {
    let mask: core::ffi::c_ulong = BIT_MASK(nr);
    let p = (addr as *mut core::ffi::c_ulong).add(BIT_WORD(nr) as usize);
    let mut old: core::ffi::c_ulong;
    let mut tmp: core::ffi::c_ulong;

    core::arch::asm!(
        "1: l.lwa {old},0({p})",
        "l.and {tmp},{old},{mask}",
        "l.swa 0({p}),{tmp}",
        "l.bnf 1b",
        " l.nop",
        old = out(reg) old,
        tmp = out(reg) tmp,
        p = in(reg) p,
        mask = in(reg) !mask,
        options(nostack)
    );

    ((old & mask) != 0) as i32
}

#[inline]
pub unsafe fn test_and_change_bit(nr: i32, addr: *mut core::ffi::c_ulong) -> i32 {
    let mask: core::ffi::c_ulong = BIT_MASK(nr);
    let p = (addr as *mut core::ffi::c_ulong).add(BIT_WORD(nr) as usize);
    let mut old: core::ffi::c_ulong;
    let mut tmp: core::ffi::c_ulong;

    core::arch::asm!(
        "1: l.lwa {old},0({p})",
        "l.xor {tmp},{old},{mask}",
        "l.swa 0({p}),{tmp}",
        "l.bnf 1b",
        " l.nop",
        old = out(reg) old,
        tmp = out(reg) tmp,
        p = in(reg) p,
        mask = in(reg) mask,
        options(nostack)
    );

    ((old & mask) != 0) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
