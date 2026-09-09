/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2005 Mips Technologies
 * Author: Chris Dearman, chris@mips.com derived from fpu.h
 */

// Dependencies supplied by the surrounding MIPS environment:
// asm/cpu.h, asm/cpu-features.h, asm/hazards.h, and asm/mipsregs.h.

pub const DSP_DEFAULT: u32 = 0x0000_0000;
pub const DSP_MASK: u32 = 0x3f;

#[inline(always)]
pub unsafe fn __enable_dsp_hazard() {
    core::arch::asm!("_ehb");
}

#[inline]
pub unsafe fn __init_dsp() {
    mthi1(0);
    mtlo1(0);
    mthi2(0);
    mtlo2(0);
    mthi3(0);
    mtlo3(0);
    wrdsp(DSP_DEFAULT, DSP_MASK);
}

#[inline]
pub unsafe fn init_dsp() {
    if cpu_has_dsp {
        __init_dsp();
    }
}

#[macro_export]
macro_rules! __save_dsp {
    ($tsk:expr) => {{
        $tsk.thread.dsp.dspr[0] = mfhi1();
        $tsk.thread.dsp.dspr[1] = mflo1();
        $tsk.thread.dsp.dspr[2] = mfhi2();
        $tsk.thread.dsp.dspr[3] = mflo2();
        $tsk.thread.dsp.dspr[4] = mfhi3();
        $tsk.thread.dsp.dspr[5] = mflo3();
        $tsk.thread.dsp.dspcontrol = rddsp(DSP_MASK);
    }};
}

#[macro_export]
macro_rules! save_dsp {
    ($tsk:expr) => {{
        if cpu_has_dsp {
            $crate::__save_dsp!($tsk);
        }
    }};
}

#[macro_export]
macro_rules! __restore_dsp {
    ($tsk:expr) => {{
        mthi1($tsk.thread.dsp.dspr[0]);
        mtlo1($tsk.thread.dsp.dspr[1]);
        mthi2($tsk.thread.dsp.dspr[2]);
        mtlo2($tsk.thread.dsp.dspr[3]);
        mthi3($tsk.thread.dsp.dspr[4]);
        mtlo3($tsk.thread.dsp.dspr[5]);
        wrdsp($tsk.thread.dsp.dspcontrol, DSP_MASK);
    }};
}

#[macro_export]
macro_rules! restore_dsp {
    ($tsk:expr) => {{
        if cpu_has_dsp {
            $crate::__restore_dsp!($tsk);
        }
    }};
}

#[macro_export]
macro_rules! __get_dsp_regs {
    ($tsk:expr, $current:expr) => {{
        if $tsk == $current {
            $crate::__save_dsp!($current);
        }
        $tsk.thread.dsp.dspr
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
