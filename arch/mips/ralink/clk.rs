// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 *  Copyright (C) 2011 Gabor Juhos <juhosg@openwrt.org>
 *  Copyright (C) 2013 John Crispin <john@phrozen.org>
 */

// Dependencies supplied by the surrounding kernel/Ralink implementation.

use core::ffi::{c_char, c_int, c_long, c_void};

#[repr(C)]
pub struct of_phandle_args {
    pub np: *mut c_void,
    pub args_count: c_int,
    pub args: [u32; 16],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

extern "C" {
    static mut ralink_soc: c_int;
    static mut mips_hpt_frequency: c_int;

    fn ralink_of_remap();
    fn panic(message: *const c_char) -> !;
    fn of_clk_init(node: *const c_void);
    fn of_find_compatible_node(
        from: *mut c_void,
        typ: *mut c_void,
        compatible: *const c_char,
    ) -> *mut c_void;
    fn of_clk_get_from_provider(spec: *mut of_phandle_args) -> *mut clk;
    fn clk_get_rate(clock: *mut clk) -> c_long;
    fn clk_put(clock: *mut clk);
    fn timer_probe();
    fn pr_info(format: *const c_char, ...);
}

extern "C" {
    static RT2880_SOC: c_int;
    static RT3883_SOC: c_int;
    static RT305X_SOC_RT3050: c_int;
    static RT305X_SOC_RT3052: c_int;
    static RT305X_SOC_RT3350: c_int;
    static RT305X_SOC_RT3352: c_int;
    static RT305X_SOC_RT5350: c_int;
    static MT762X_SOC_MT7620A: c_int;
    static MT762X_SOC_MT7620N: c_int;
    static MT762X_SOC_MT7628AN: c_int;
    static MT762X_SOC_MT7688: c_int;
}

unsafe fn clk_cpu(idx: *mut c_int) -> *const c_char {
    match ralink_soc {
        x if x == RT2880_SOC => {
            *idx = 1;
            b"ralink,rt2880-sysc\0".as_ptr() as *const c_char
        }
        x if x == RT3883_SOC => {
            *idx = 1;
            b"ralink,rt3883-sysc\0".as_ptr() as *const c_char
        }
        x if x == RT305X_SOC_RT3050 => {
            *idx = 1;
            b"ralink,rt3050-sysc\0".as_ptr() as *const c_char
        }
        x if x == RT305X_SOC_RT3052 => {
            *idx = 1;
            b"ralink,rt3052-sysc\0".as_ptr() as *const c_char
        }
        x if x == RT305X_SOC_RT3350 => {
            *idx = 1;
            b"ralink,rt3350-sysc\0".as_ptr() as *const c_char
        }
        x if x == RT305X_SOC_RT3352 => {
            *idx = 1;
            b"ralink,rt3352-sysc\0".as_ptr() as *const c_char
        }
        x if x == RT305X_SOC_RT5350 => {
            *idx = 1;
            b"ralink,rt5350-sysc\0".as_ptr() as *const c_char
        }
        x if x == MT762X_SOC_MT7620A || x == MT762X_SOC_MT7620N => {
            *idx = 2;
            b"ralink,mt7620-sysc\0".as_ptr() as *const c_char
        }
        x if x == MT762X_SOC_MT7628AN => {
            *idx = 1;
            b"ralink,mt7628-sysc\0".as_ptr() as *const c_char
        }
        x if x == MT762X_SOC_MT7688 => {
            *idx = 1;
            b"ralink,mt7688-sysc\0".as_ptr() as *const c_char
        }
        _ => {
            *idx = -1;
            b"invalid\0".as_ptr() as *const c_char
        }
    }
}

pub unsafe fn plat_time_init() {
    let mut clkspec = of_phandle_args {
        np: core::ptr::null_mut(),
        args_count: 0,
        args: [0; 16],
    };
    let mut cpu_clk_idx: c_int = 0;

    ralink_of_remap();

    let compatible = clk_cpu(&mut cpu_clk_idx);
    if cpu_clk_idx == -1 {
        panic(b"unable to get CPU clock index\0".as_ptr() as *const c_char);
    }

    of_clk_init(core::ptr::null());
    clkspec.np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), compatible);
    clkspec.args_count = 1;
    clkspec.args[0] = cpu_clk_idx as u32;
    let clk = of_clk_get_from_provider(&mut clkspec);
    if clk as isize == -1 {
        panic(b"unable to get CPU clock, err=%ld\0".as_ptr() as *const c_char);
    }
    pr_info(b"CPU Clock: %ldMHz\n\0".as_ptr() as *const c_char, clk_get_rate(clk) / 1_000_000);
    mips_hpt_frequency = clk_get_rate(clk) as c_int / 2;
    clk_put(clk);
    timer_probe();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
