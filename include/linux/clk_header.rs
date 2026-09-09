/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of linux/include/linux/clk.h. */

use core::ffi::c_char;

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct of_phandle_args { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
#[repr(C)] pub struct srcu_notifier_head { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }

pub const PRE_RATE_CHANGE: u32 = 1 << 0;
pub const POST_RATE_CHANGE: u32 = 1 << 1;
pub const ABORT_RATE_CHANGE: u32 = 1 << 2;

#[repr(C)]
pub struct clk_notifier {
    pub clk: *mut clk,
    pub notifier_head: srcu_notifier_head,
    pub node: list_head,
}

#[repr(C)]
pub struct clk_notifier_data {
    pub clk: *mut clk,
    pub old_rate: c_ulong,
    pub new_rate: c_ulong,
}

#[repr(C)]
pub struct clk_bulk_data {
    pub id: *const c_char,
    pub clk: *mut clk,
}

pub type c_ulong = usize;
pub type c_uint = u32;
pub type c_int = i32;
pub type c_long = isize;

extern "C" {
    #[cfg(feature = "CONFIG_COMMON_CLK")]
    pub fn clk_notifier_register(clk: *mut clk, nb: *mut notifier_block) -> c_int;
    #[cfg(feature = "CONFIG_COMMON_CLK")]
    pub fn clk_notifier_unregister(clk: *mut clk, nb: *mut notifier_block) -> c_int;
    #[cfg(feature = "CONFIG_COMMON_CLK")]
    pub fn devm_clk_notifier_register(dev: *mut device, clk: *mut clk, nb: *mut notifier_block) -> c_int;
    #[cfg(feature = "CONFIG_COMMON_CLK")]
    pub fn clk_get_accuracy(clk: *mut clk) -> c_long;
    #[cfg(feature = "CONFIG_COMMON_CLK")]
    pub fn clk_set_phase(clk: *mut clk, degrees: c_int) -> c_int;
    #[cfg(feature = "CONFIG_COMMON_CLK")]
    pub fn clk_get_phase(clk: *mut clk) -> c_int;
    #[cfg(feature = "CONFIG_COMMON_CLK")]
    pub fn clk_set_duty_cycle(clk: *mut clk, num: c_uint, den: c_uint) -> c_int;
    #[cfg(feature = "CONFIG_COMMON_CLK")]
    pub fn clk_get_scaled_duty_cycle(clk: *mut clk, scale: c_uint) -> c_int;
    #[cfg(feature = "CONFIG_COMMON_CLK")]
    pub fn clk_is_match(p: *const clk, q: *const clk) -> bool;
    #[cfg(feature = "CONFIG_COMMON_CLK")]
    pub fn clk_rate_exclusive_get(clk: *mut clk) -> c_int;
    #[cfg(feature = "CONFIG_COMMON_CLK")]
    pub fn devm_clk_rate_exclusive_get(dev: *mut device, clk: *mut clk) -> c_int;
    #[cfg(feature = "CONFIG_COMMON_CLK")]
    pub fn clk_rate_exclusive_put(clk: *mut clk);
    #[cfg(feature = "CONFIG_COMMON_CLK")]
    pub fn clk_save_context() -> c_int;
    #[cfg(feature = "CONFIG_COMMON_CLK")]
    pub fn clk_restore_context();

    #[cfg(feature = "CONFIG_HAVE_CLK_PREPARE")]
    pub fn clk_prepare(clk: *mut clk) -> c_int;
    #[cfg(feature = "CONFIG_HAVE_CLK_PREPARE")]
    pub fn clk_unprepare(clk: *mut clk);
    #[cfg(feature = "CONFIG_HAVE_CLK_PREPARE")]
    pub fn clk_bulk_prepare(num_clks: c_int, clks: *const clk_bulk_data) -> c_int;
    #[cfg(feature = "CONFIG_HAVE_CLK_PREPARE")]
    pub fn clk_bulk_unprepare(num_clks: c_int, clks: *const clk_bulk_data);
    #[cfg(feature = "CONFIG_HAVE_CLK_PREPARE")]
    pub fn clk_is_enabled_when_prepared(clk: *mut clk) -> bool;

    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_bulk_get(dev: *mut device, num_clks: c_int, clks: *mut clk_bulk_data) -> c_int;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_bulk_get_all(dev: *mut device, clks: *mut *mut clk_bulk_data) -> c_int;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_bulk_get_optional(dev: *mut device, num_clks: c_int, clks: *mut clk_bulk_data) -> c_int;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn devm_clk_bulk_get(dev: *mut device, num_clks: c_int, clks: *mut clk_bulk_data) -> c_int;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn devm_clk_bulk_get_optional(dev: *mut device, num_clks: c_int, clks: *mut clk_bulk_data) -> c_int;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn devm_clk_bulk_get_enable(dev: *mut device, num_clks: c_int, clks: *mut clk_bulk_data) -> c_int;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn devm_clk_bulk_get_optional_enable(dev: *mut device, num_clks: c_int, clks: *mut clk_bulk_data) -> c_int;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn devm_clk_bulk_get_all(dev: *mut device, clks: *mut *mut clk_bulk_data) -> c_int;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn devm_clk_bulk_get_all_enabled(dev: *mut device, clks: *mut *mut clk_bulk_data) -> c_int;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn devm_clk_get_prepared(dev: *mut device, id: *const c_char) -> *mut clk;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn devm_clk_get_enabled(dev: *mut device, id: *const c_char) -> *mut clk;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn devm_clk_get_optional_prepared(dev: *mut device, id: *const c_char) -> *mut clk;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn devm_clk_get_optional_enabled(dev: *mut device, id: *const c_char) -> *mut clk;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn devm_clk_get_optional_enabled_with_rate(dev: *mut device, id: *const c_char, rate: c_ulong) -> *mut clk;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn devm_get_clk_from_child(dev: *mut device, np: *mut device_node, con_id: *const c_char) -> *mut clk;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_enable(clk: *mut clk) -> c_int;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_bulk_enable(num_clks: c_int, clks: *const clk_bulk_data) -> c_int;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_disable(clk: *mut clk);
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_bulk_disable(num_clks: c_int, clks: *const clk_bulk_data);
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_get_rate(clk: *mut clk) -> c_ulong;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_put(clk: *mut clk);
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_bulk_put(num_clks: c_int, clks: *mut clk_bulk_data);
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_bulk_put_all(num_clks: c_int, clks: *mut clk_bulk_data);
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn devm_clk_put(dev: *mut device, clk: *mut clk);
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_round_rate(clk: *mut clk, rate: c_ulong) -> c_long;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_set_rate_exclusive(clk: *mut clk, rate: c_ulong) -> c_int;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_has_parent(clk: *const clk, parent: *const clk) -> bool;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_set_rate_range(clk: *mut clk, min: c_ulong, max: c_ulong) -> c_int;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_set_min_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_set_max_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_get_parent(clk: *mut clk) -> *mut clk;
    #[cfg(feature = "CONFIG_HAVE_CLK")]
    pub fn clk_get_sys(dev_id: *const c_char, con_id: *const c_char) -> *mut clk;

    #[cfg(all(feature = "CONFIG_OF", feature = "CONFIG_COMMON_CLK"))]
    pub fn of_clk_get(np: *mut device_node, index: c_int) -> *mut clk;
    #[cfg(all(feature = "CONFIG_OF", feature = "CONFIG_COMMON_CLK"))]
    pub fn of_clk_get_by_name(np: *mut device_node, name: *const c_char) -> *mut clk;
    #[cfg(all(feature = "CONFIG_OF", feature = "CONFIG_COMMON_CLK"))]
    pub fn of_clk_get_from_provider(clkspec: *mut of_phandle_args) -> *mut clk;
}

/* Inline helpers retain the C control flow and use null as the C NULL value. */
#[inline] pub unsafe fn clk_prepare_enable(clk: *mut clk) -> c_int {
    let ret = clk_prepare(clk); if ret != 0 { return ret; }
    let ret = clk_enable(clk); if ret != 0 { clk_unprepare(clk); } ret
}
#[inline] pub unsafe fn clk_disable_unprepare(clk: *mut clk) { clk_disable(clk); clk_unprepare(clk); }
#[inline] pub unsafe fn clk_bulk_prepare_enable(num_clks: c_int, clks: *const clk_bulk_data) -> c_int {
    let ret = clk_bulk_prepare(num_clks, clks); if ret != 0 { return ret; }
    let ret = clk_bulk_enable(num_clks, clks); if ret != 0 { clk_bulk_unprepare(num_clks, clks); } ret
}
#[inline] pub unsafe fn clk_bulk_disable_unprepare(num_clks: c_int, clks: *const clk_bulk_data) {
    clk_bulk_disable(num_clks, clks); clk_bulk_unprepare(num_clks, clks);
}
#[inline] pub unsafe fn clk_drop_range(clk: *mut clk) -> c_int { clk_set_rate_range(clk, 0, c_ulong::MAX) }
#[inline] pub unsafe fn clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk {
    /* C compares against ERR_PTR(-ENOENT); the errno encoding is dependency-supplied. */
    clk_get(dev, id)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
