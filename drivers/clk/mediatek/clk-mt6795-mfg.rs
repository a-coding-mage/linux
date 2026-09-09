// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 Collabora Ltd.
 * Author: AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// Dependencies supplied by the surrounding kernel translation.
use crate::clk_gate::{mtk_clk_gate_ops_setclr, mtk_gate_regs, mtk_gate};
use crate::clk_mtk::{mtk_clk_desc, mtk_clk_simple_probe, mtk_clk_simple_remove};

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct PlatformDriverDriver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const OfDeviceId,
}

#[repr(C)]
pub struct PlatformDriver {
    pub driver: PlatformDriverDriver,
    pub probe: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32,
    pub remove: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32,
}

extern "C" {
    static mut clk_mt6795_mfg_drv: PlatformDriver;
}

static MFG_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

const CLK_MFG_BAXI: u32 = 0;
const CLK_MFG_BMEM: u32 = 1;
const CLK_MFG_BG3D: u32 = 2;
const CLK_MFG_B26M: u32 = 3;

static MFG_CLKS: [mtk_gate; 4] = [
    mtk_gate { id: CLK_MFG_BAXI, name: b"mfg_baxi\0".as_ptr() as *const _, parent_name: b"axi_mfg_in_sel\0".as_ptr() as *const _, regs: &MFG_CG_REGS, shift: 0, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_MFG_BMEM, name: b"mfg_bmem\0".as_ptr() as *const _, parent_name: b"mem_mfg_in_sel\0".as_ptr() as *const _, regs: &MFG_CG_REGS, shift: 1, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_MFG_BG3D, name: b"mfg_bg3d\0".as_ptr() as *const _, parent_name: b"mfg_sel\0".as_ptr() as *const _, regs: &MFG_CG_REGS, shift: 2, ops: &mtk_clk_gate_ops_setclr },
    mtk_gate { id: CLK_MFG_B26M, name: b"mfg_b26m\0".as_ptr() as *const _, parent_name: b"clk26m\0".as_ptr() as *const _, regs: &MFG_CG_REGS, shift: 3, ops: &mtk_clk_gate_ops_setclr },
];

static MFG_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: MFG_CLKS.as_ptr(),
    num_clks: MFG_CLKS.len(),
};

static OF_MATCH_CLK_MT6795_MFG: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"mediatek,mt6795-mfgcfg\0".as_ptr() as *const _, data: &MFG_DESC as *const _ as *const core::ffi::c_void },
    OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() },
];

#[no_mangle]
pub static mut CLK_MT6795_MFG_DRV: PlatformDriver = PlatformDriver {
    driver: PlatformDriverDriver {
        name: b"clk-mt6795-mfg\0".as_ptr() as *const _,
        of_match_table: OF_MATCH_CLK_MT6795_MFG.as_ptr(),
    },
    probe: mtk_clk_simple_probe,
    remove: mtk_clk_simple_remove,
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt6795_mfg);
// module_platform_driver(clk_mt6795_mfg_drv);
// MODULE_DESCRIPTION("MediaTek MT6795 mfg clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
