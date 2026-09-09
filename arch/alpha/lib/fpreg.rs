// SPDX-License-Identifier: GPL-2.0
/*
 * arch/alpha/lib/fpreg.c
 *
 * (C) Copyright 1998 Linus Torvalds
 */

#[repr(C)]
pub struct ThreadInfo {
    pub status: usize,
    pub fp: [usize; 32],
}

extern "C" {
    fn preempt_disable();
    fn preempt_enable();
    fn current_thread_info() -> *mut ThreadInfo;
}

pub const TS_SAVED_FP: usize = 1 << 0;
pub const TS_RESTORE_FP: usize = 1 << 1;

#[inline(always)]
unsafe fn stt<const REG: usize>(val: &mut usize) {
    // CONFIG_ALPHA_EV6/CONFIG_ALPHA_EV67 use `ftoit`; other Alpha CPUs use `stt`.
    #[cfg(any(CONFIG_ALPHA_EV6, CONFIG_ALPHA_EV67))]
    core::arch::asm!(concat!("ftoit $f", stringify!(REG), ",{}"), out(reg) *val);
    #[cfg(not(any(CONFIG_ALPHA_EV6, CONFIG_ALPHA_EV67)))]
    core::arch::asm!(concat!("stt $f", stringify!(REG), ",{}"), out(reg) *val);
}

#[inline(always)]
unsafe fn ldt<const REG: usize>(val: &usize) {
    #[cfg(any(CONFIG_ALPHA_EV6, CONFIG_ALPHA_EV67))]
    core::arch::asm!(concat!("itoft {},$f", stringify!(REG)), in(reg) *val);
    #[cfg(not(any(CONFIG_ALPHA_EV6, CONFIG_ALPHA_EV67)))]
    core::arch::asm!(concat!("ldt $f", stringify!(REG), ",{}"), in(reg) *val);
}

#[inline(always)]
unsafe fn sts<const REG: usize>(val: &mut usize) {
    #[cfg(any(CONFIG_ALPHA_EV6, CONFIG_ALPHA_EV67))]
    core::arch::asm!(concat!("ftois $f", stringify!(REG), ",{}"), out(reg) *val);
    #[cfg(not(any(CONFIG_ALPHA_EV6, CONFIG_ALPHA_EV67)))]
    core::arch::asm!(concat!("sts $f", stringify!(REG), ",{}"), out(reg) *val);
}

#[inline(always)]
unsafe fn lds<const REG: usize>(val: &usize) {
    #[cfg(any(CONFIG_ALPHA_EV6, CONFIG_ALPHA_EV67))]
    core::arch::asm!(concat!("itofs {},$f", stringify!(REG)), in(reg) *val);
    #[cfg(not(any(CONFIG_ALPHA_EV6, CONFIG_ALPHA_EV67)))]
    core::arch::asm!(concat!("lds $f", stringify!(REG), ",{}"), in(reg) *val);
}

#[inline(never)]
pub unsafe extern "C" fn alpha_read_fp_reg(reg: usize) -> usize {
    let mut val: usize;
    if reg >= 32 {
        return 0;
    }
    preempt_disable();
    let ti = &mut *current_thread_info();
    if ti.status & TS_SAVED_FP != 0 {
        val = ti.fp[reg];
    } else {
        val = 0;
        match reg {
            0 => stt::<0>(&mut val), 1 => stt::<1>(&mut val), 2 => stt::<2>(&mut val),
            3 => stt::<3>(&mut val), 4 => stt::<4>(&mut val), 5 => stt::<5>(&mut val),
            6 => stt::<6>(&mut val), 7 => stt::<7>(&mut val), 8 => stt::<8>(&mut val),
            9 => stt::<9>(&mut val), 10 => stt::<10>(&mut val), 11 => stt::<11>(&mut val),
            12 => stt::<12>(&mut val), 13 => stt::<13>(&mut val), 14 => stt::<14>(&mut val),
            15 => stt::<15>(&mut val), 16 => stt::<16>(&mut val), 17 => stt::<17>(&mut val),
            18 => stt::<18>(&mut val), 19 => stt::<19>(&mut val), 20 => stt::<20>(&mut val),
            21 => stt::<21>(&mut val), 22 => stt::<22>(&mut val), 23 => stt::<23>(&mut val),
            24 => stt::<24>(&mut val), 25 => stt::<25>(&mut val), 26 => stt::<26>(&mut val),
            27 => stt::<27>(&mut val), 28 => stt::<28>(&mut val), 29 => stt::<29>(&mut val),
            30 => stt::<30>(&mut val), 31 => stt::<31>(&mut val), _ => {}
        }
    }
    preempt_enable();
    val
}

#[inline(never)]
pub unsafe extern "C" fn alpha_write_fp_reg(reg: usize, val: usize) {
    if reg >= 32 { return; }
    preempt_disable();
    let ti = &mut *current_thread_info();
    if ti.status & TS_SAVED_FP != 0 {
        ti.status |= TS_RESTORE_FP;
        ti.fp[reg] = val;
    } else {
        match reg {
            0 => ldt::<0>(&val), 1 => ldt::<1>(&val), 2 => ldt::<2>(&val), 3 => ldt::<3>(&val),
            4 => ldt::<4>(&val), 5 => ldt::<5>(&val), 6 => ldt::<6>(&val), 7 => ldt::<7>(&val),
            8 => ldt::<8>(&val), 9 => ldt::<9>(&val), 10 => ldt::<10>(&val), 11 => ldt::<11>(&val),
            12 => ldt::<12>(&val), 13 => ldt::<13>(&val), 14 => ldt::<14>(&val), 15 => ldt::<15>(&val),
            16 => ldt::<16>(&val), 17 => ldt::<17>(&val), 18 => ldt::<18>(&val), 19 => ldt::<19>(&val),
            20 => ldt::<20>(&val), 21 => ldt::<21>(&val), 22 => ldt::<22>(&val), 23 => ldt::<23>(&val),
            24 => ldt::<24>(&val), 25 => ldt::<25>(&val), 26 => ldt::<26>(&val), 27 => ldt::<27>(&val),
            28 => ldt::<28>(&val), 29 => ldt::<29>(&val), 30 => ldt::<30>(&val), 31 => ldt::<31>(&val), _ => {}
        }
    }
    preempt_enable();
}

#[inline(never)]
pub unsafe extern "C" fn alpha_read_fp_reg_s(reg: usize) -> usize {
    let mut val: usize;
    if reg >= 32 { return 0; }
    preempt_disable();
    let ti = &mut *current_thread_info();
    if ti.status & TS_SAVED_FP != 0 {
        ldt::<0>(&ti.fp[reg]);
        val = 0;
        sts::<0>(&mut val);
    } else {
        val = 0;
        match reg {
            0 => sts::<0>(&mut val), 1 => sts::<1>(&mut val), 2 => sts::<2>(&mut val), 3 => sts::<3>(&mut val),
            4 => sts::<4>(&mut val), 5 => sts::<5>(&mut val), 6 => sts::<6>(&mut val), 7 => sts::<7>(&mut val),
            8 => sts::<8>(&mut val), 9 => sts::<9>(&mut val), 10 => sts::<10>(&mut val), 11 => sts::<11>(&mut val),
            12 => sts::<12>(&mut val), 13 => sts::<13>(&mut val), 14 => sts::<14>(&mut val), 15 => sts::<15>(&mut val),
            16 => sts::<16>(&mut val), 17 => sts::<17>(&mut val), 18 => sts::<18>(&mut val), 19 => sts::<19>(&mut val),
            20 => sts::<20>(&mut val), 21 => sts::<21>(&mut val), 22 => sts::<22>(&mut val), 23 => sts::<23>(&mut val),
            24 => sts::<24>(&mut val), 25 => sts::<25>(&mut val), 26 => sts::<26>(&mut val), 27 => sts::<27>(&mut val),
            28 => sts::<28>(&mut val), 29 => sts::<29>(&mut val), 30 => sts::<30>(&mut val), 31 => sts::<31>(&mut val), _ => {}
        }
    }
    preempt_enable();
    val
}

#[inline(never)]
pub unsafe extern "C" fn alpha_write_fp_reg_s(reg: usize, val: usize) {
    if reg >= 32 { return; }
    preempt_disable();
    let ti = &mut *current_thread_info();
    if ti.status & TS_SAVED_FP != 0 {
        ti.status |= TS_RESTORE_FP;
        lds::<0>(&val);
        stt::<0>(&mut ti.fp[reg]);
    } else {
        match reg {
            0 => lds::<0>(&val), 1 => lds::<1>(&val), 2 => lds::<2>(&val), 3 => lds::<3>(&val),
            4 => lds::<4>(&val), 5 => lds::<5>(&val), 6 => lds::<6>(&val), 7 => lds::<7>(&val),
            8 => lds::<8>(&val), 9 => lds::<9>(&val), 10 => lds::<10>(&val), 11 => lds::<11>(&val),
            12 => lds::<12>(&val), 13 => lds::<13>(&val), 14 => lds::<14>(&val), 15 => lds::<15>(&val),
            16 => lds::<16>(&val), 17 => lds::<17>(&val), 18 => lds::<18>(&val), 19 => lds::<19>(&val),
            20 => lds::<20>(&val), 21 => lds::<21>(&val), 22 => lds::<22>(&val), 23 => lds::<23>(&val),
            24 => lds::<24>(&val), 25 => lds::<25>(&val), 26 => lds::<26>(&val), 27 => lds::<27>(&val),
            28 => lds::<28>(&val), 29 => lds::<29>(&val), 30 => lds::<30>(&val), 31 => lds::<31>(&val), _ => {}
        }
    }
    preempt_enable();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
