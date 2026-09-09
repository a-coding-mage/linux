/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Hisilicon Hi3620 clock gate driver declarations. */

use core::ffi::{c_char, c_int, c_void};

// Dependencies supplied by the surrounding kernel translation.
pub use crate::{clk, clk_div_table, clk_onecell_data, device, device_node, platform_device};
pub use crate::spinlock_t;

#[repr(C)]
pub struct hisi_clock_data {
    pub clk_data: clk_onecell_data,
    pub base: *mut c_void,
}

#[repr(C)]
pub struct hisi_fixed_rate_clock {
    pub id: u32,
    pub name: *mut c_char,
    pub parent_name: *const c_char,
    pub flags: usize,
    pub fixed_rate: usize,
}

#[repr(C)]
pub struct hisi_fixed_factor_clock {
    pub id: u32,
    pub name: *mut c_char,
    pub parent_name: *const c_char,
    pub flags: usize,
    pub mult: usize,
    pub div: usize,
}

#[repr(C)]
pub struct hisi_mux_clock {
    pub id: u32,
    pub name: *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
    pub flags: usize,
    pub offset: usize,
    pub shift: u8,
    pub width: u8,
    pub mux_flags: u8,
    pub table: *const u32,
    pub alias: *const c_char,
}

#[repr(C)]
pub struct hisi_phase_clock {
    pub id: u32,
    pub name: *const c_char,
    pub parent_names: *const c_char,
    pub flags: usize,
    pub offset: usize,
    pub shift: u8,
    pub width: u8,
    pub phase_degrees: *mut u32,
    pub phase_regvals: *mut u32,
    pub phase_num: u8,
}

#[repr(C)]
pub struct hisi_divider_clock {
    pub id: u32,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub flags: usize,
    pub offset: usize,
    pub shift: u8,
    pub width: u8,
    pub div_flags: u8,
    pub table: *mut clk_div_table,
    pub alias: *const c_char,
}

#[repr(C)]
pub struct hi6220_divider_clock {
    pub id: u32,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub flags: usize,
    pub offset: usize,
    pub shift: u8,
    pub width: u8,
    pub mask_bit: u32,
    pub alias: *const c_char,
}

#[repr(C)]
pub struct hisi_gate_clock {
    pub id: u32,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub flags: usize,
    pub offset: usize,
    pub bit_idx: u8,
    pub gate_flags: u8,
    pub alias: *const c_char,
}

extern "C" {
    pub fn hisi_register_clkgate_sep(
        dev: *mut device, name: *const c_char, parent_name: *const c_char,
        flags: usize, reg: *mut c_void, bit_idx: u8, gate_flags: u8,
        lock: *mut spinlock_t,
    ) -> *mut clk;
    pub fn hi6220_register_clkdiv(
        dev: *mut device, name: *const c_char, parent_name: *const c_char,
        flags: usize, reg: *mut c_void, shift: u8, width: u8, mask_bit: u32,
        lock: *mut spinlock_t,
    ) -> *mut clk;

    pub fn hisi_clk_alloc(dev: *mut platform_device, nums: c_int) -> *mut hisi_clock_data;
    pub fn hisi_clk_init(np: *mut device_node, nums: c_int) -> *mut hisi_clock_data;
    pub fn hisi_clk_register_fixed_rate(clks: *const hisi_fixed_rate_clock, nums: c_int, data: *mut hisi_clock_data) -> c_int;
    pub fn hisi_clk_register_fixed_factor(clks: *const hisi_fixed_factor_clock, nums: c_int, data: *mut hisi_clock_data) -> c_int;
    pub fn hisi_clk_register_mux(clks: *const hisi_mux_clock, nums: c_int, data: *mut hisi_clock_data) -> c_int;
    pub fn clk_register_hisi_phase(dev: *mut device, clks: *const hisi_phase_clock, base: *mut c_void, lock: *mut spinlock_t) -> *mut clk;
    pub fn hisi_clk_register_phase(dev: *mut device, clks: *const hisi_phase_clock, nums: c_int, data: *mut hisi_clock_data) -> c_int;
    pub fn hisi_clk_register_divider(clks: *const hisi_divider_clock, nums: c_int, data: *mut hisi_clock_data) -> c_int;
    pub fn hisi_clk_register_gate(clks: *const hisi_gate_clock, nums: c_int, data: *mut hisi_clock_data) -> c_int;
    pub fn hisi_clk_register_gate_sep(clks: *const hisi_gate_clock, nums: c_int, data: *mut hisi_clock_data);
    pub fn hi6220_clk_register_divider(clks: *const hi6220_divider_clock, nums: c_int, data: *mut hisi_clock_data);
}

#[macro_export]
macro_rules! hisi_clk_unregister {
    ($unregister:ident, $clock:ty, $clk_unregister:ident) => {
        pub unsafe fn $unregister(clks: *const $clock, nums: c_int, data: *mut hisi_clock_data) {
            let clocks = (*data).clk_data.clks;
            let mut i: c_int = 0;
            while i < nums {
                let id = (*clks.add(i as usize)).id as usize;
                if !(*clocks.add(id)).is_null() {
                    $clk_unregister(*clocks.add(id));
                }
                i += 1;
            }
        }
    };
}

extern "C" {
    fn clk_unregister_fixed_rate(clock: *mut clk);
    fn clk_unregister_fixed_factor(clock: *mut clk);
    fn clk_unregister_mux(clock: *mut clk);
    fn clk_unregister_divider(clock: *mut clk);
    fn clk_unregister_gate(clock: *mut clk);
}

hisi_clk_unregister!(hisi_clk_unregister_fixed_rate, hisi_fixed_rate_clock, clk_unregister_fixed_rate);
hisi_clk_unregister!(hisi_clk_unregister_fixed_factor, hisi_fixed_factor_clock, clk_unregister_fixed_factor);
hisi_clk_unregister!(hisi_clk_unregister_mux, hisi_mux_clock, clk_unregister_mux);
hisi_clk_unregister!(hisi_clk_unregister_divider, hisi_divider_clock, clk_unregister_divider);
hisi_clk_unregister!(hisi_clk_unregister_gate, hisi_gate_clock, clk_unregister_gate);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
