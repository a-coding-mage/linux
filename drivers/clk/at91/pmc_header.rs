/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Rust translation of drivers/clk/at91/pmc.h
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    pub static mut pmc_pcr_lock: spinlock_t;

    pub static at91rm9200_master_layout: clk_master_layout;
    pub static at91sam9x5_master_layout: clk_master_layout;
    pub static at91rm9200_pll_layout: clk_pll_layout;
    pub static at91sam9g45_pll_layout: clk_pll_layout;
    pub static at91sam9g20_pllb_layout: clk_pll_layout;
    pub static sama5d3_pll_layout: clk_pll_layout;
    pub static at91rm9200_programmable_layout: clk_programmable_layout;
    pub static at91sam9g45_programmable_layout: clk_programmable_layout;
    pub static at91sam9x5_programmable_layout: clk_programmable_layout;
}

#[repr(C)]
pub struct pmc_data {
    pub ncore: c_uint,
    pub chws: *mut *mut clk_hw,
    pub nsystem: c_uint,
    pub shws: *mut *mut clk_hw,
    pub nperiph: c_uint,
    pub phws: *mut *mut clk_hw,
    pub ngck: c_uint,
    pub ghws: *mut *mut clk_hw,
    pub npck: c_uint,
    pub pchws: *mut *mut clk_hw,
    pub hwtable: [*mut clk_hw; 0],
}

#[repr(C)]
pub struct clk_range {
    pub min: c_ulong,
    pub max: c_ulong,
}

#[inline]
pub const fn clk_range(min: c_ulong, max: c_ulong) -> clk_range {
    clk_range { min, max }
}

#[repr(C)]
pub struct clk_master_layout {
    pub offset: u32,
    pub mask: u32,
    pub pres_shift: u8,
}

#[repr(C)]
pub struct clk_master_characteristics {
    pub output: clk_range,
    pub divisors: [u32; 5],
    pub have_div3_pres: u8,
}

#[repr(C)]
pub struct clk_pll_layout {
    pub pllr_mask: u32,
    pub mul_mask: u32,
    pub frac_mask: u32,
    pub div_mask: u32,
    pub endiv_mask: u32,
    pub mul_shift: u8,
    pub frac_shift: u8,
    pub div_shift: u8,
    pub endiv_shift: u8,
    pub div2: u8,
}

#[repr(C)]
pub struct clk_pll_characteristics {
    pub input: clk_range,
    pub num_output: c_int,
    pub output: *const clk_range,
    pub core_output: *const clk_range,
    pub icpll: *mut u16,
    pub out: *mut u8,
    pub upll: u8,
    pub acr: u32,
}

#[repr(C)]
pub struct clk_programmable_layout {
    pub pres_mask: u8,
    pub pres_shift: u8,
    pub css_mask: u8,
    pub have_slck_mck: u8,
    pub is_pres_direct: u8,
}

#[repr(C)]
pub struct clk_pcr_layout {
    pub offset: u32,
    pub cmd: u32,
    pub div_mask: u32,
    pub gckcss_mask: u32,
    pub pid_mask: u32,
}

/**
 * struct at91_clk_pms - Power management state for AT91 clock
 * @rate: clock rate
 * @parent_rate: clock parent rate
 * @status: clock status (enabled or disabled)
 * @parent: clock parent index
 */
#[repr(C)]
pub struct at91_clk_pms {
    pub rate: c_ulong,
    pub parent_rate: c_ulong,
    pub status: c_uint,
    pub parent: c_uint,
}

#[inline]
pub unsafe fn ndck<T>(a: *const T, s: usize) -> u32
where
    T: NdckId,
{
    (*a.add(s - 1)).id() + 1
}

#[inline]
pub unsafe fn nck<T>(a: *const T, len: usize) -> u32
where
    T: NdckId,
{
    (*a.add(len - 1)).id() + 1
}

pub trait NdckId { fn id(&self) -> u32; }

#[inline]
pub unsafe fn pmc_init_table(table: *mut u8, count: u8) {
    let mut i = 0u8;
    while i < count {
        *table.add(i as usize) = i;
        i = i.wrapping_add(1);
    }
}

#[inline]
pub unsafe fn pmc_fill_table(to: *mut u8, from: *const u8, count: u8) {
    let mut i = 0u8;
    while i < count {
        *to.add(i as usize) = *from.add(i as usize);
        i = i.wrapping_add(1);
    }
}

extern "C" {
    pub fn pmc_data_allocate(ncore: c_uint, nsystem: c_uint, nperiph: c_uint, ngck: c_uint, npck: c_uint) -> *mut pmc_data;
    pub fn of_at91_get_clk_range(np: *mut device_node, propname: *const c_char, range: *mut clk_range) -> c_int;
    pub fn of_clk_hw_pmc_get(clkspec: *mut of_phandle_args, data: *mut c_void) -> *mut clk_hw;
    pub fn at91_clk_register_audio_pll_frac(regmap: *mut regmap, name: *const c_char, parent_name: *const c_char) -> *mut clk_hw;
    pub fn at91_clk_register_audio_pll_pad(regmap: *mut regmap, name: *const c_char, parent_name: *const c_char) -> *mut clk_hw;
    pub fn at91_clk_register_audio_pll_pmc(regmap: *mut regmap, name: *const c_char, parent_name: *const c_char) -> *mut clk_hw;
    pub fn at91_clk_register_generated(regmap: *mut regmap, lock: *mut spinlock_t, layout: *const clk_pcr_layout, name: *const c_char, parent_names: *const *const c_char, parent_hws: *mut *mut clk_hw, mux_table: *mut u32, num_parents: u8, id: u8, range: *const clk_range, chg_pid: c_int) -> *mut clk_hw;
    pub fn at91_clk_register_h32mx(regmap: *mut regmap, name: *const c_char, parent_name: *const c_char) -> *mut clk_hw;
    pub fn at91_clk_i2s_mux_register(regmap: *mut regmap, name: *const c_char, parent_names: *const *const c_char, num_parents: c_uint, bus_id: u8) -> *mut clk_hw;
    pub fn at91_clk_register_main_rc_osc(regmap: *mut regmap, name: *const c_char, frequency: u32, accuracy: u32) -> *mut clk_hw;
    pub fn at91_clk_register_main_osc(regmap: *mut regmap, name: *const c_char, parent_name: *const c_char, parent_data: *mut clk_parent_data, bypass: bool) -> *mut clk_hw;
    pub fn at91_clk_register_rm9200_main(regmap: *mut regmap, name: *const c_char, parent_name: *const c_char, parent_hw: *mut clk_hw) -> *mut clk_hw;
    pub fn at91_clk_register_sam9x5_main(regmap: *mut regmap, name: *const c_char, parent_names: *const *const c_char, parent_hws: *mut *mut clk_hw, num_parents: c_int) -> *mut clk_hw;
    pub fn at91_clk_register_master_pres(regmap: *mut regmap, name: *const c_char, num_parents: c_int, parent_names: *const *const c_char, parent_hws: *mut *mut clk_hw, layout: *const clk_master_layout, characteristics: *const clk_master_characteristics, lock: *mut spinlock_t) -> *mut clk_hw;
    pub fn at91_clk_register_master_div(regmap: *mut regmap, name: *const c_char, parent_names: *const c_char, parent_hw: *mut clk_hw, layout: *const clk_master_layout, characteristics: *const clk_master_characteristics, lock: *mut spinlock_t, flags: u32, safe_div: u32) -> *mut clk_hw;
    pub fn at91_clk_sama7g5_register_master(regmap: *mut regmap, name: *const c_char, num_parents: c_int, parent_names: *const *const c_char, parent_hws: *mut *mut clk_hw, mux_table: *mut u32, lock: *mut spinlock_t, id: u8, critical: bool, chg_pid: c_int) -> *mut clk_hw;
    pub fn at91_clk_register_peripheral(regmap: *mut regmap, name: *const c_char, parent_name: *const c_char, parent_hw: *mut clk_hw, id: u32) -> *mut clk_hw;
    pub fn at91_clk_register_sam9x5_peripheral(regmap: *mut regmap, lock: *mut spinlock_t, layout: *const clk_pcr_layout, name: *const c_char, parent_name: *const c_char, parent_hw: *mut clk_hw, id: u32, range: *const clk_range, chg_pid: c_int, flags: c_ulong) -> *mut clk_hw;
    pub fn at91_clk_register_pll(regmap: *mut regmap, name: *const c_char, parent_name: *const c_char, id: u8, layout: *const clk_pll_layout, characteristics: *const clk_pll_characteristics) -> *mut clk_hw;
    pub fn at91_clk_register_plldiv(regmap: *mut regmap, name: *const c_char, parent_name: *const c_char) -> *mut clk_hw;
    pub fn sam9x60_clk_register_div_pll(regmap: *mut regmap, lock: *mut spinlock_t, name: *const c_char, parent_name: *const c_char, parent_hw: *mut clk_hw, id: u8, characteristics: *const clk_pll_characteristics, layout: *const clk_pll_layout, flags: u32, safe_div: u32) -> *mut clk_hw;
    pub fn sam9x60_clk_register_frac_pll(regmap: *mut regmap, lock: *mut spinlock_t, name: *const c_char, parent_name: *const c_char, parent_hw: *mut clk_hw, id: u8, characteristics: *const clk_pll_characteristics, layout: *const clk_pll_layout, flags: u32) -> *mut clk_hw;
    pub fn at91_clk_register_programmable(regmap: *mut regmap, name: *const c_char, parent_names: *const *const c_char, parent_hws: *mut *mut clk_hw, num_parents: u8, id: u8, layout: *const clk_programmable_layout, mux_table: *mut u32) -> *mut clk_hw;
    pub fn at91_clk_register_sam9260_slow(regmap: *mut regmap, name: *const c_char, parent_names: *const *const c_char, num_parents: c_int) -> *mut clk_hw;
    pub fn at91sam9x5_clk_register_smd(regmap: *mut regmap, name: *const c_char, parent_names: *const *const c_char, num_parents: u8) -> *mut clk_hw;
    pub fn at91_clk_register_system(regmap: *mut regmap, name: *const c_char, parent_name: *const c_char, parent_hw: *mut clk_hw, id: u8, flags: c_ulong) -> *mut clk_hw;
    pub fn at91sam9x5_clk_register_usb(regmap: *mut regmap, name: *const c_char, parent_names: *const *const c_char, num_parents: u8) -> *mut clk_hw;
    pub fn at91sam9n12_clk_register_usb(regmap: *mut regmap, name: *const c_char, parent_name: *const c_char) -> *mut clk_hw;
    pub fn sam9x60_clk_register_usb(regmap: *mut regmap, name: *const c_char, parent_names: *const *const c_char, num_parents: u8) -> *mut clk_hw;
    pub fn at91rm9200_clk_register_usb(regmap: *mut regmap, name: *const c_char, parent_name: *const c_char, divisors: *const u32) -> *mut clk_hw;
    pub fn at91_clk_register_utmi(regmap_pmc: *mut regmap, regmap_sfr: *mut regmap, name: *const c_char, parent_name: *const c_char, parent_hw: *mut clk_hw) -> *mut clk_hw;
    pub fn at91_clk_sama7g5_register_utmi(regmap: *mut regmap, name: *const c_char, parent_name: *const c_char, parent_hw: *mut clk_hw) -> *mut clk_hw;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
