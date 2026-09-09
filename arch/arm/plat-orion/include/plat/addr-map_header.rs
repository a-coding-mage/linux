/*
 * arch/arm/plat-orion/include/plat/addr-map.h
 *
 * Marvell Orion SoC address map handling.
 *
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct mbus_dram_target_info {
    _private: [u8; 0],
}

extern "C" {
    pub static mut orion_mbus_dram_info: mbus_dram_target_info;
}

#[repr(C)]
pub struct orion_addr_map_cfg {
    pub num_wins: c_int, /* Total number of windows */
    pub remappable_wins: c_int,
    pub bridge_virt_base: *mut c_void,
    pub hw_io_coherency: c_int,

    /* If NULL, the default cpu_win_can_remap will be used, using
       the value in remappable_wins */
    pub cpu_win_can_remap: Option<
        unsafe extern "C" fn(cfg: *const orion_addr_map_cfg, win: c_int) -> c_int,
    >,
    /* If NULL, the default win_cfg_base will be used, using
       the value in bridge_virt_base */
    pub win_cfg_base: Option<
        unsafe extern "C" fn(
            cfg: *const orion_addr_map_cfg,
            win: c_int,
        ) -> *mut c_void,
    >,
}

/*
 * Information needed to setup one address mapping.
 */
#[repr(C)]
pub struct orion_addr_map_info {
    pub win: c_int,
    pub base: u32,
    pub size: u32,
    pub target: u8,
    pub attr: u8,
    pub remap: c_int,
}

extern "C" {
    pub fn orion_config_wins(
        cfg: *mut orion_addr_map_cfg,
        info: *const orion_addr_map_info,
    );

    pub fn orion_setup_cpu_win(
        cfg: *const orion_addr_map_cfg,
        win: c_int,
        base: u32,
        size: u32,
        target: u8,
        attr: u8,
        remap: c_int,
    );

    pub fn orion_setup_cpu_mbus_target(
        cfg: *const orion_addr_map_cfg,
        ddr_window_cpu_base: *const c_void,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
