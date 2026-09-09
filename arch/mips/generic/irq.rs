// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// External declarations supplied by the corresponding kernel headers.
unsafe extern "C" {
    fn mips_gic_present() -> bool;
    fn gic_get_c0_fdc_int() -> ::core::ffi::c_int;
    fn gic_get_c0_perfcount_int() -> ::core::ffi::c_int;
    fn gic_get_c0_compare_int() -> ::core::ffi::c_int;
    static cpu_has_veic: bool;
    static cp0_fdc_irq: ::core::ffi::c_int;
    static cp0_perfcount_irq: ::core::ffi::c_int;
    static cp0_compare_irq: ::core::ffi::c_int;
    static MIPS_CPU_IRQ_BASE: ::core::ffi::c_int;
}

pub unsafe fn get_c0_fdc_int() -> ::core::ffi::c_int {
    let mips_cpu_fdc_irq: ::core::ffi::c_int;

    if mips_gic_present() {
        mips_cpu_fdc_irq = gic_get_c0_fdc_int();
    } else if cpu_has_veic {
        panic!("Unimplemented!");
    } else if cp0_fdc_irq >= 0 {
        mips_cpu_fdc_irq = MIPS_CPU_IRQ_BASE + cp0_fdc_irq;
    } else {
        mips_cpu_fdc_irq = -1;
    }

    mips_cpu_fdc_irq
}

pub unsafe fn get_c0_perfcount_int() -> ::core::ffi::c_int {
    let mips_cpu_perf_irq: ::core::ffi::c_int;

    if mips_gic_present() {
        mips_cpu_perf_irq = gic_get_c0_perfcount_int();
    } else if cpu_has_veic {
        panic!("Unimplemented!");
    } else if cp0_perfcount_irq >= 0 {
        mips_cpu_perf_irq = MIPS_CPU_IRQ_BASE + cp0_perfcount_irq;
    } else {
        mips_cpu_perf_irq = -1;
    }

    mips_cpu_perf_irq
}

pub unsafe fn get_c0_compare_int() -> u32 {
    let mips_cpu_timer_irq: ::core::ffi::c_int;

    if mips_gic_present() {
        mips_cpu_timer_irq = gic_get_c0_compare_int();
    } else if cpu_has_veic {
        panic!("Unimplemented!");
    } else {
        mips_cpu_timer_irq = MIPS_CPU_IRQ_BASE + cp0_compare_irq;
    }

    mips_cpu_timer_irq as u32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
