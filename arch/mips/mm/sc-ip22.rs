// SPDX-License-Identifier: GPL-2.0
/*
 * sc-ip22.c: Indy cache management functions.
 *
 * Copyright (C) 1997, 2001 Ralf Baechle (ralf@gnu.org),
 * derived from r4xx0.c by David S. Miller (davem@davemloft.net).
 */

// Linux kernel dependencies supplied by the surrounding translation unit.
extern "C" {
    static mut scache_size: libc::c_ulong;
    static mut sgimc: *mut SgiMc;
    static mut bcops: *mut BcacheOps;
    fn ip22_eeprom_read(eeprom: *mut libc::c_uchar, index: libc::c_int) -> libc::c_uint;
    fn printk(fmt: *const libc::c_char, ...);
    fn local_irq_save(flags: *mut libc::c_ulong);
    fn local_irq_restore(flags: libc::c_ulong);
    fn bug_on(condition: bool);
}

#[repr(C)]
struct SgiMc { eeprom: [libc::c_uchar; 0] }

#[repr(C)]
struct BcacheOps {
    bc_enable: Option<unsafe extern "C" fn()>,
    bc_disable: Option<unsafe extern "C" fn()>,
    bc_wback_inv: Option<unsafe extern "C" fn(libc::c_ulong, libc::c_ulong)>,
    bc_inv: Option<unsafe extern "C" fn(libc::c_ulong, libc::c_ulong)>,
}

const SC_SIZE: libc::c_ulong = 0x0008_0000;
const SC_LINE: libc::c_ulong = 32;
const CI_MASK: libc::c_ulong = SC_SIZE - SC_LINE;

#[inline]
fn sc_index(n: libc::c_ulong) -> libc::c_ulong { n & CI_MASK }

#[inline]
unsafe fn indy_sc_wipe(mut first: libc::c_ulong, mut last: libc::c_ulong) {
    let mut tmp: libc::c_ulong;
    // The original MIPS inline assembly is retained verbatim in intent.
    core::arch::asm!(
        ".set push; .set noreorder; .set mips3; .set noat",
        "mfc0 {tmp}, $12; li $1, 0x80; mtc0 $1, $12",
        "lui $1, 0x9000; dsll $1, $1, 0x10; ori $1, $1, 0x8000; dsll $1, $1, 0x10",
        "or {first}, $1; or {last}, $1",
        "1: sw $0, 0({first}); bne {first}, {last}, 1b; daddu {first}, 32",
        "mtc0 {tmp}, $12; nop; nop; nop; nop; .set pop",
        first = inout(reg) first, last = inout(reg) last, tmp = lateout(reg) tmp,
        options(nostack)
    );
}

unsafe extern "C" fn indy_sc_wback_invalidate(addr: libc::c_ulong, size: libc::c_ulong) {
    bug_on(size == 0);
    let first_line = sc_index(addr);
    let last_line = sc_index(addr.wrapping_add(size).wrapping_sub(1));
    let mut flags = 0;
    local_irq_save(&mut flags);
    if first_line <= last_line {
        indy_sc_wipe(first_line, last_line);
    } else {
        indy_sc_wipe(first_line, SC_SIZE - SC_LINE);
        indy_sc_wipe(0, last_line);
    }
    local_irq_restore(flags);
}

unsafe extern "C" fn indy_sc_enable() {
    let mut addr: libc::c_ulong;
    let mut tmp1: libc::c_ulong;
    let mut tmp2: libc::c_ulong;
    core::arch::asm!(".set push; .set noreorder; .set mips3; mfc0 {addr}, $12; li {tmp2}, 0x80; mtc0 {tmp2}, $12; li {tmp1}, 1; dsll {tmp1}, 31; lui {tmp2}, 0x9000; dsll32 {tmp2}, 0; or {tmp1}, {tmp2}, {tmp1}; sb $0, 0({tmp1}); mtc0 $0, $12; mtc0 {addr}, $12; .set pop", addr = lateout(reg) addr, tmp1 = lateout(reg) tmp1, tmp2 = lateout(reg) tmp2, options(nostack));
}

unsafe extern "C" fn indy_sc_disable() {
    let mut tmp1: libc::c_ulong;
    let mut tmp2: libc::c_ulong;
    let mut tmp3: libc::c_ulong;
    core::arch::asm!(".set push; .set noreorder; .set mips3; li {tmp1}, 1; dsll {tmp1}, 31; lui {tmp2}, 0x9000; dsll32 {tmp2}, 0; or {tmp1}, {tmp2}, {tmp1}; mfc0 {tmp3}, $12; li {tmp2}, 0x80; mtc0 {tmp2}, $12; sh $0, 0({tmp1}); mtc0 $0, $12; mtc0 {tmp3}, $12; .set pop", tmp1 = lateout(reg) tmp1, tmp2 = lateout(reg) tmp2, tmp3 = lateout(reg) tmp3, options(nostack));
}

#[inline]
unsafe extern "C" fn indy_sc_probe() -> libc::c_int {
    let size = ip22_eeprom_read((*sgimc).eeprom.as_mut_ptr(), 17);
    if size == 0 { return 0; }
    scache_size = ((size as libc::c_ulong) << PAGE_SHIFT) as libc::c_ulong;
    1
}

const PAGE_SHIFT: u32 = 12;

static mut indy_sc_ops: BcacheOps = BcacheOps {
    bc_enable: Some(indy_sc_enable),
    bc_disable: Some(indy_sc_disable),
    bc_wback_inv: Some(indy_sc_wback_invalidate),
    bc_inv: Some(indy_sc_wback_invalidate),
};

#[no_mangle]
pub unsafe extern "C" fn indy_sc_init() {
    if indy_sc_probe() != 0 {
        indy_sc_enable();
        bcops = &raw mut indy_sc_ops;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
