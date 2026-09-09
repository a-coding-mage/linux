// SPDX-License-Identifier: GPL-2.0-or-later
/*    Signal support for 32-bit kernel builds
 *
 *    Copyright (C) 2001 Matthew Wilcox <willy at parisc-linux.org>
 *    Copyright (C) 2006 Kyle McMartin <kyle at parisc-linux.org>
 *
 *    Code was mostly borrowed from kernel/signal.c.
 *    See kernel/signal.c for additional Copyrights.
 */

// Dependencies are supplied by the surrounding kernel translation unit.

const DEBUG_COMPAT_SIG: bool = false;
const DEBUG_COMPAT_SIG_LEVEL: i32 = 2;

#[inline(always)]
unsafe fn dbg(_level: i32, _message: &str) {
    // The C DBG macro is disabled by DEBUG_COMPAT_SIG.
}

pub unsafe fn restore_sigcontext32(
    sc: *mut compat_sigcontext,
    rf: *mut compat_regfile,
    regs: *mut pt_regs,
) -> i64 {
    let mut err: i64 = 0;
    let mut compat_reg: compat_uint_t;
    let mut compat_regt: compat_uint_t;

    /* When loading 32-bit values into 64-bit registers make
       sure to clear the upper 32-bits */
    dbg(2, "restore_sigcontext32: PER_LINUX32 process\n");
    dbg(2, "restore_sigcontext32: sc = 0x%p, rf = 0x%p, regs = 0x%p\n");
    dbg(2, "restore_sigcontext32: compat_sigcontext is %#lx bytes\n");
    for regn in 0..32 {
        err |= __get_user(&mut compat_reg, &(*sc).sc_gr[regn]);
        (*regs).gr[regn] = compat_reg as u64;
        /* Load upper half */
        err |= __get_user(&mut compat_regt, &(*rf).rf_gr[regn]);
        (*regs).gr[regn] = ((compat_regt as u64) << 32) | compat_reg as u64;
        dbg(3, "restore_sigcontext32: gr%02d = %#lx (%#x / %#x)\n");
    }
    dbg(2, "restore_sigcontext32: sc->sc_fr = 0x%p (%#lx)\n");
    /* XXX: BE WARNED FR's are 64-BIT! */
    err |= __copy_from_user((*regs).fr.as_mut_ptr(), (*sc).sc_fr, core::mem::size_of_val(&(*regs).fr));

    /* Better safe than sorry, pass __get_user two things of
       the same size and let gcc do the upward conversion to
       64-bits */
    err |= __get_user(&mut compat_reg, &(*sc).sc_iaoq[0]);
    /* Load upper half */
    err |= __get_user(&mut compat_regt, &(*rf).rf_iaoq[0]);
    (*regs).iaoq[0] = ((compat_regt as u64) << 32) | compat_reg as u64;
    dbg(2, "restore_sigcontext32: upper half of iaoq[0] = %#lx\n");
    dbg(2, "restore_sigcontext32: sc->sc_iaoq[0] = %p => %#x\n");

    err |= __get_user(&mut compat_reg, &(*sc).sc_iaoq[1]);
    /* Load upper half */
    err |= __get_user(&mut compat_regt, &(*rf).rf_iaoq[1]);
    (*regs).iaoq[1] = ((compat_regt as u64) << 32) | compat_reg as u64;
    dbg(2, "restore_sigcontext32: upper half of iaoq[1] = %#lx\n");
    dbg(2, "restore_sigcontext32: sc->sc_iaoq[1] = %p => %#x\n");
    dbg(2, "restore_sigcontext32: iaoq is %#lx / %#lx\n");

    err |= __get_user(&mut compat_reg, &(*sc).sc_iasq[0]);
    /* Load the upper half for iasq */
    err |= __get_user(&mut compat_regt, &(*rf).rf_iasq[0]);
    (*regs).iasq[0] = ((compat_regt as u64) << 32) | compat_reg as u64;
    dbg(2, "restore_sigcontext32: upper half of iasq[0] = %#lx\n");
    err |= __get_user(&mut compat_reg, &(*sc).sc_iasq[1]);
    /* Load the upper half for iasq */
    err |= __get_user(&mut compat_regt, &(*rf).rf_iasq[1]);
    (*regs).iasq[1] = ((compat_regt as u64) << 32) | compat_reg as u64;
    dbg(2, "restore_sigcontext32: upper half of iasq[1] = %#lx\n");
    dbg(2, "restore_sigcontext32: iasq is %#lx / %#lx\n");

    err |= __get_user(&mut compat_reg, &(*sc).sc_sar);
    /* Load the upper half for sar */
    err |= __get_user(&mut compat_regt, &(*rf).rf_sar);
    (*regs).sar = ((compat_regt as u64) << 32) | compat_reg as u64;
    dbg(2, "restore_sigcontext32: upper_half & sar = %#lx\n");
    dbg(2, "restore_sigcontext32: sar is %#lx\n");
    dbg(2, "restore_sigcontext32: r28 is %ld\n");
    err
}

pub unsafe fn setup_sigcontext32(
    sc: *mut compat_sigcontext,
    rf: *mut compat_regfile,
    regs: *mut pt_regs,
    in_syscall: i32,
) -> i64 {
    let mut flags: compat_int_t = 0;
    let mut err: i64 = 0;
    let mut compat_reg: compat_uint_t;
    let mut compat_regb: compat_uint_t;

    if on_sig_stack(sc as usize) != 0 { flags |= PARISC_SC_FLAG_ONSTACK; }
    if in_syscall != 0 {
        dbg(1, "setup_sigcontext32: in_syscall\n");
        flags |= PARISC_SC_FLAG_IN_SYSCALL;
        compat_reg = (*regs).gr[31] as compat_uint_t;
        err |= __put_user(compat_reg, &mut (*sc).sc_iaoq[0]);
        dbg(2, "setup_sigcontext32: sc->sc_iaoq[0] = %p <= %#x\n");
        compat_reg = ((*regs).gr[31] >> 32) as compat_uint_t;
        err |= __put_user(compat_reg, &mut (*rf).rf_iaoq[0]);
        dbg(2, "setup_sigcontext32: upper half iaoq[0] = %#x\n");
        compat_reg = ((*regs).gr[31].wrapping_add(4)) as compat_uint_t;
        err |= __put_user(compat_reg, &mut (*sc).sc_iaoq[1]);
        dbg(2, "setup_sigcontext32: sc->sc_iaoq[1] = %p <= %#x\n");
        compat_reg = (((*regs).gr[31].wrapping_add(4)) >> 32) as compat_uint_t;
        err |= __put_user(compat_reg, &mut (*rf).rf_iaoq[1]);
        dbg(2, "setup_sigcontext32: upper half iaoq[1] = %#x\n");
        compat_reg = (*regs).sr[3] as compat_uint_t;
        err |= __put_user(compat_reg, &mut (*sc).sc_iasq[0]);
        err |= __put_user(compat_reg, &mut (*sc).sc_iasq[1]);
        compat_reg = ((*regs).sr[3] >> 32) as compat_uint_t;
        err |= __put_user(compat_reg, &mut (*rf).rf_iasq[0]);
        err |= __put_user(compat_reg, &mut (*rf).rf_iasq[1]);
        dbg(2, "setup_sigcontext32: upper half iasq[0] = %#x\n");
        dbg(2, "setup_sigcontext32: upper half iasq[1] = %#x\n");
        dbg(1, "setup_sigcontext32: iaoq %#lx / %#lx\n");
    } else {
        for i in 0..2 { compat_reg = (*regs).iaoq[i] as compat_uint_t; err |= __put_user(compat_reg, &mut (*sc).sc_iaoq[i]); compat_reg = ((*regs).iaoq[i] >> 32) as compat_uint_t; err |= __put_user(compat_reg, &mut (*rf).rf_iaoq[i]); }
        for i in 0..2 { compat_reg = (*regs).iasq[i] as compat_uint_t; err |= __put_user(compat_reg, &mut (*sc).sc_iasq[i]); compat_reg = ((*regs).iasq[i] >> 32) as compat_uint_t; err |= __put_user(compat_reg, &mut (*rf).rf_iasq[i]); }
        dbg(1, "setup_sigcontext32: ia0q %#lx / %#lx\n");
    }
    err |= __put_user(flags, &mut (*sc).sc_flags);
    dbg(1, "setup_sigcontext32: Truncating general registers.\n");
    for regn in 0..32 {
        compat_reg = (*regs).gr[regn] as compat_uint_t;
        err |= __put_user(compat_reg, &mut (*sc).sc_gr[regn]);
        compat_regb = ((*regs).gr[regn] >> 32) as compat_uint_t;
        err |= __put_user(compat_regb, &mut (*rf).rf_gr[regn]);
        dbg(2, "setup_sigcontext32: gr%02d = %#x / %#x\n");
    }
    dbg(1, "setup_sigcontext32: Copying from regs to sc, sc->sc_fr size = %#lx, regs->fr size = %#lx\n");
    err |= __copy_to_user((*sc).sc_fr, (*regs).fr.as_ptr(), core::mem::size_of_val(&(*regs).fr));
    compat_reg = (*regs).sar as compat_uint_t;
    err |= __put_user(compat_reg, &mut (*sc).sc_sar);
    dbg(2, "setup_sigcontext32: sar is %#x\n");
    compat_reg = ((*regs).sar >> 32) as compat_uint_t;
    err |= __put_user(compat_reg, &mut (*rf).rf_sar);
    dbg(2, "setup_sigcontext32: upper half sar = %#x\n");
    dbg(1, "setup_sigcontext32: r28 is %ld\n");
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
