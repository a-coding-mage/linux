// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2003, 2004, 2007  Maciej W. Rozycki
 */

use core::arch::asm;

// Dependencies supplied by the surrounding kernel translation unit.
extern "C" {
    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
    fn printk(fmt: *const c_char, ...);
    fn pr_cont(fmt: *const c_char, ...);
    fn panic(fmt: *const c_char, ... ) -> !;
    fn exception_enter() -> c_int;
    fn exception_exit(prev_state: c_int);
    fn set_except_vector(code: c_int, handler: *mut c_void) -> *mut c_void;
    fn handle_daddi_ov();
}

type c_char = i8;
type c_int = i32;
type c_ulong = usize;
type c_void = core::ffi::c_void;

#[repr(C)]
pub struct pt_regs {
    pub cp0_epc: usize,
}

static mut BUG64HIT: [u8; 34] = *b"reliable operation impossible!\n%s\0";
static mut NOWAR: [u8; 53] = *b"Please report to <linux-mips@vger.kernel.org>.\0";
static mut R4KWAR: [u8; 48] = *b"Enable CPU_R4000_WORKAROUNDS to rectify.\0";
static mut DADDIWAR: [u8; 48] = *b"Enable CPU_DADDI_WORKAROUNDS to rectify.\0";

#[inline(always)]
unsafe fn align_mod(align: c_int, modulo: c_int) {
    asm!(
        ".set push",
        ".set noreorder",
        ".balign {align}",
        ".rept {modulo}",
        "nop",
        ".endr",
        ".set pop",
        align = const align,
        modulo = const modulo,
        options(nostack, preserves_flags)
    );
}

#[inline(always)]
unsafe fn mult_sh_align_mod(v1: *mut c_long, v2: *mut c_long, w: *mut c_long,
                            align: c_int, modulo: c_int) {
    let mut flags: c_ulong = 0;
    let (mut m1, mut m2, mut s): (c_int, c_int, c_int);
    let (mut p, mut lv1, mut lv2, mut lw): (c_long, c_long, c_long, c_long);

    local_irq_save(&mut flags);
    asm!("", out(reg) m1, out(reg) m2, out(reg) s,
         in("{m1}") 5, in("{m2}") 8, in("{s}") 5);
    align_mod(align, modulo);
    asm!(
        "mult {m1}, {m2}",
        "dsll32 {lv1}, {s}, 0",
        "mflo $0",
        "dsll32 {lw}, {s}, 0",
        "nop",
        m1 = in(reg) m1, m2 = in(reg) m2, s = in(reg) s,
        lv1 = out(reg) lv1, lw = out(reg) lw,
        options(nostack)
    );
    asm!("", out(reg) m1, out(reg) m2, out(reg) s,
         in("{m1}") m1, in("{m2}") m2, in("{s}") s);
    align_mod(align, modulo);
    p = m1.wrapping_mul(m2) as c_long;
    lv2 = s.wrapping_shl(32) as c_long;
    asm!("", out(reg) lv2, in(reg) lv2, in(reg) p);
    local_irq_restore(flags);

    *v1 = lv1;
    *v2 = lv2;
    *w = lw;
}

#[inline(always)]
unsafe fn check_mult_sh() {
    let mut v1 = [0 as c_long; 8];
    let mut v2 = [0 as c_long; 8];
    let mut w = [0 as c_long; 8];
    let mut bug: c_int;
    let mut fix: c_int;

    printk(b"Checking for the multiply/shift bug... \0".as_ptr() as *const c_char);
    mult_sh_align_mod(v1.as_mut_ptr().add(0), v2.as_mut_ptr().add(0), w.as_mut_ptr().add(0), 32, 0);
    mult_sh_align_mod(v1.as_mut_ptr().add(1), v2.as_mut_ptr().add(1), w.as_mut_ptr().add(1), 32, 1);
    mult_sh_align_mod(v1.as_mut_ptr().add(2), v2.as_mut_ptr().add(2), w.as_mut_ptr().add(2), 32, 2);
    mult_sh_align_mod(v1.as_mut_ptr().add(3), v2.as_mut_ptr().add(3), w.as_mut_ptr().add(3), 32, 3);
    mult_sh_align_mod(v1.as_mut_ptr().add(4), v2.as_mut_ptr().add(4), w.as_mut_ptr().add(4), 32, 4);
    mult_sh_align_mod(v1.as_mut_ptr().add(5), v2.as_mut_ptr().add(5), w.as_mut_ptr().add(5), 32, 5);
    mult_sh_align_mod(v1.as_mut_ptr().add(6), v2.as_mut_ptr().add(6), w.as_mut_ptr().add(6), 32, 6);
    mult_sh_align_mod(v1.as_mut_ptr().add(7), v2.as_mut_ptr().add(7), w.as_mut_ptr().add(7), 32, 7);

    bug = 0;
    for i in 0..8 { if v1[i] != w[i] { bug = 1; } }
    if bug == 0 { pr_cont(b"no.\n\0".as_ptr() as *const c_char); return; }
    pr_cont(b"yes, workaround... \0".as_ptr() as *const c_char);
    fix = 1;
    for i in 0..8 { if v2[i] != w[i] { fix = 0; } }
    if fix == 1 { pr_cont(b"yes.\n\0".as_ptr() as *const c_char); return; }
    pr_cont(b"no.\n\0".as_ptr() as *const c_char);
    panic(BUG64HIT.as_ptr() as *const c_char, R4KWAR.as_ptr());
}

static mut DADDI_OV: c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn do_daddi_ov(regs: *mut pt_regs) {
    let prev_state = exception_enter();
    DADDI_OV = 1;
    (*regs).cp0_epc = (*regs).cp0_epc.wrapping_add(4);
    exception_exit(prev_state);
}

#[inline(never)]
unsafe fn check_daddi() {
    let mut flags: c_ulong = 0;
    let mut handler: *mut c_void;
    let (mut v, mut tmp): (c_long, c_long);
    printk(b"Checking for the daddi bug... \0".as_ptr() as *const c_char);
    local_irq_save(&mut flags);
    handler = set_except_vector(12, handle_daddi_ov as *mut c_void);
    asm!("addiu {tmp}, $0, 0xffffffffffffdb9a", "dsrl {tmp}, {tmp}, 1", "daddi {v}, {tmp}, 0x1234", v = out(reg) v, tmp = out(reg) tmp);
    set_except_vector(12, handler);
    local_irq_restore(flags);
    if DADDI_OV != 0 { pr_cont(b"no.\n\0".as_ptr() as *const c_char); return; }
    pr_cont(b"yes, workaround... \0".as_ptr() as *const c_char);
    local_irq_save(&mut flags);
    handler = set_except_vector(12, handle_daddi_ov as *mut c_void);
    asm!("addiu {tmp}, $0, 0xffffffffffffdb9a", "dsrl {tmp}, {tmp}, 1", "daddi {v}, {tmp}, 0x1234", v = out(reg) v, tmp = out(reg) tmp);
    set_except_vector(12, handler);
    local_irq_restore(flags);
    if DADDI_OV != 0 { pr_cont(b"yes.\n\0".as_ptr() as *const c_char); return; }
    pr_cont(b"no.\n\0".as_ptr() as *const c_char);
    panic(BUG64HIT.as_ptr() as *const c_char, DADDIWAR.as_ptr());
}

type c_long = isize;
pub static mut daddiu_bug: c_int = -1;

#[inline(never)]
unsafe fn check_daddiu() {
    let (mut v, mut w, mut tmp): (c_long, c_long, c_long);
    printk(b"Checking for the daddiu bug... \0".as_ptr() as *const c_char);
    asm!("addiu {tmp}, $0, 0xffffffffffffdb9a", "dsrl {tmp}, {tmp}, 1", "daddiu {v}, {tmp}, 0x1234", "addiu {w}, $0, 0x1234", "daddu {w}, {tmp}", v = out(reg) v, w = out(reg) w, tmp = out(reg) tmp);
    daddiu_bug = (v != w) as c_int;
    if daddiu_bug == 0 { pr_cont(b"no.\n\0".as_ptr() as *const c_char); return; }
    pr_cont(b"yes, workaround... \0".as_ptr() as *const c_char);
    asm!("addiu {tmp}, $0, 0xffffffffffffdb9a", "dsrl {tmp}, {tmp}, 1", "daddiu {v}, {tmp}, 0x1234", "addiu {w}, $0, 0x1234", "daddu {w}, {tmp}", v = out(reg) v, w = out(reg) w, tmp = out(reg) tmp);
    if v == w { pr_cont(b"yes.\n\0".as_ptr() as *const c_char); return; }
    pr_cont(b"no.\n\0".as_ptr() as *const c_char);
    panic(BUG64HIT.as_ptr() as *const c_char, DADDIWAR.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn check_bugs64_early() { check_mult_sh(); check_daddiu(); }

#[no_mangle]
pub unsafe extern "C" fn check_bugs64() { check_daddi(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
