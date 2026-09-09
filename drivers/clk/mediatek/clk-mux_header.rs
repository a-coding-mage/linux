/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2018 MediaTek Inc.
 * Author: Owen Chen <owen.chen@mediatek.com>
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

#[repr(C)]
pub struct mtk_mux {
    pub id: core::ffi::c_int,
    pub name: *const core::ffi::c_char,
    pub parent_names: *const *const core::ffi::c_char,
    pub parent_index: *const u8,
    pub flags: u32,

    pub mux_ofs: u32,
    pub set_ofs: u32,
    pub clr_ofs: u32,
    pub upd_ofs: u32,

    pub hwv_set_ofs: u32,
    pub hwv_clr_ofs: u32,
    pub hwv_sta_ofs: u32,
    pub fenc_sta_mon_ofs: u32,

    pub mux_shift: u8,
    pub mux_width: u8,
    pub gate_shift: u8,
    pub upd_shift: i8,
    pub fenc_shift: u8,

    pub ops: *const clk_ops,
    pub num_parents: i8,
}

#[macro_export]
macro_rules! __GATE_CLR_SET_UPD_FLAGS {
    ($id:expr, $name:expr, $parents:expr, $paridx:expr, $num_parents:expr,
     $mux_ofs:expr, $mux_set_ofs:expr, $mux_clr_ofs:expr, $shift:expr,
     $width:expr, $gate:expr, $upd_ofs:expr, $upd:expr, $flags:expr, $ops:expr) => {
        mtk_mux {
            id: $id, name: $name, mux_ofs: $mux_ofs, set_ofs: $mux_set_ofs,
            clr_ofs: $mux_clr_ofs, upd_ofs: $upd_ofs, mux_shift: $shift,
            mux_width: $width, gate_shift: $gate, upd_shift: $upd,
            parent_names: $parents, parent_index: $paridx,
            num_parents: $num_parents, flags: $flags, ops: &$ops,
            hwv_set_ofs: 0, hwv_clr_ofs: 0, hwv_sta_ofs: 0,
            fenc_sta_mon_ofs: 0, fenc_shift: 0,
        }
    };
}

#[macro_export]
macro_rules! GATE_CLR_SET_UPD_FLAGS {
    ($($args:tt)*) => { __GATE_CLR_SET_UPD_FLAGS!($($args)*) };
}
#[macro_export]
macro_rules! GATE_CLR_SET_UPD_FLAGS_INDEXED {
    ($($args:tt)*) => { __GATE_CLR_SET_UPD_FLAGS!($($args)*) };
}
#[macro_export]
macro_rules! MUX_GATE_CLR_SET_UPD_FLAGS { ($($args:tt)*) => { GATE_CLR_SET_UPD_FLAGS!($($args)*) }; }
#[macro_export]
macro_rules! MUX_GATE_CLR_SET_UPD_FLAGS_INDEXED { ($($args:tt)*) => { GATE_CLR_SET_UPD_FLAGS_INDEXED!($($args)*) }; }
#[macro_export]
macro_rules! MUX_GATE_CLR_SET_UPD { ($($args:tt)*) => { MUX_GATE_CLR_SET_UPD_FLAGS!($($args)*) }; }
#[macro_export]
macro_rules! MUX_GATE_CLR_SET_UPD_INDEXED { ($($args:tt)*) => { MUX_GATE_CLR_SET_UPD_FLAGS_INDEXED!($($args)*) }; }
#[macro_export]
macro_rules! MUX_CLR_SET_UPD { ($($args:tt)*) => { GATE_CLR_SET_UPD_FLAGS!($($args)*) }; }
#[macro_export]
macro_rules! MUX_CLR_SET { ($($args:tt)*) => { MUX_CLR_SET_UPD!($($args)*) }; }

#[macro_export]
macro_rules! MUX_GATE_HWV_FENC_CLR_SET_UPD_FLAGS {
    ($id:expr, $name:expr, $parents:expr, $mux_ofs:expr, $mux_set_ofs:expr,
     $mux_clr_ofs:expr, $hwv_sta_ofs:expr, $hwv_set_ofs:expr, $hwv_clr_ofs:expr,
     $shift:expr, $width:expr, $gate:expr, $upd_ofs:expr, $upd:expr,
     $fenc_sta_mon_ofs:expr, $fenc:expr, $flags:expr) => { mtk_mux {
        id: $id, name: $name, mux_ofs: $mux_ofs, set_ofs: $mux_set_ofs,
        clr_ofs: $mux_clr_ofs, hwv_sta_ofs: $hwv_sta_ofs,
        hwv_set_ofs: $hwv_set_ofs, hwv_clr_ofs: $hwv_clr_ofs, upd_ofs: $upd_ofs,
        fenc_sta_mon_ofs: $fenc_sta_mon_ofs, mux_shift: $shift,
        mux_width: $width, gate_shift: $gate, upd_shift: $upd,
        fenc_shift: $fenc, parent_names: $parents, parent_index: core::ptr::null(),
        num_parents: $parents.len() as i8, flags: $flags,
        ops: &mtk_mux_gate_hwv_fenc_clr_set_upd_ops,
    } };
}
#[macro_export]
macro_rules! MUX_GATE_HWV_FENC_CLR_SET_UPD { ($($args:tt)*) => { MUX_GATE_HWV_FENC_CLR_SET_UPD_FLAGS!($($args)*) }; }
#[macro_export]
macro_rules! MUX_GATE_FENC_CLR_SET_UPD_FLAGS { ($($args:tt)*) => { MUX_GATE_HWV_FENC_CLR_SET_UPD_FLAGS!($($args)*) }; }
#[macro_export]
macro_rules! MUX_GATE_FENC_CLR_SET_UPD { ($($args:tt)*) => { MUX_GATE_FENC_CLR_SET_UPD_FLAGS!($($args)*) }; }
#[macro_export]
macro_rules! MUX_GATE_FENC_CLR_SET_UPD_INDEXED { ($($args:tt)*) => { MUX_GATE_FENC_CLR_SET_UPD_FLAGS!($($args)*) }; }

extern "C" {
    pub static mtk_mux_clr_set_upd_ops: clk_ops;
    pub static mtk_mux_gate_clr_set_upd_ops: clk_ops;
    pub static mtk_mux_gate_fenc_clr_set_upd_ops: clk_ops;
    pub static mtk_mux_gate_hwv_fenc_clr_set_upd_ops: clk_ops;

    pub fn mtk_clk_register_muxes(dev: *mut device, muxes: *const mtk_mux,
        num: core::ffi::c_int, node: *mut device_node, lock: *mut spinlock_t,
        clk_data: *mut clk_hw_onecell_data) -> core::ffi::c_int;
    pub fn mtk_clk_unregister_muxes(muxes: *const mtk_mux, num: core::ffi::c_int,
        clk_data: *mut clk_hw_onecell_data);
    pub fn devm_mtk_clk_mux_notifier_register(dev: *mut device, clk: *mut clk,
        mux_nb: *mut mtk_mux_nb) -> core::ffi::c_int;
}

#[repr(C)]
pub struct mtk_mux_nb {
    pub nb: notifier_block,
    pub ops: *const clk_ops,
    pub bypass_index: u8,
    pub original_index: u8,
}

#[macro_export]
macro_rules! to_mtk_mux_nb { ($nb:expr) => { container_of!($nb, mtk_mux_nb, nb) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
