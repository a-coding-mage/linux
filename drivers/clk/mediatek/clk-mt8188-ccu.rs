// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Garmin Chang <garmin.chang@mediatek.com>
 */

use core::ffi::{c_char, c_void};

// Dependencies supplied by the Linux clock and platform-driver bindings.
#[repr(C)]
pub struct mtk_gate_regs {
    pub set_ofs: u32,
    pub clr_ofs: u32,
    pub sta_ofs: u32,
}

#[repr(C)]
pub struct mtk_clk_gate_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_gate {
    pub id: u32,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub regs: *const mtk_gate_regs,
    pub shift: u32,
    pub ops: *const mtk_clk_gate_ops,
}

#[repr(C)]
pub struct mtk_clk_desc {
    pub clks: *const mtk_gate,
    pub num_clks: usize,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub driver: device_driver,
}

extern "C" {
    pub static mtk_clk_gate_ops_setclr: mtk_clk_gate_ops;
    pub static mtk_clk_simple_probe: unsafe extern "C" fn(*mut c_void) -> i32;
    pub static mtk_clk_simple_remove: unsafe extern "C" fn(*mut c_void) -> i32;
}

// Clock identifiers supplied by <dt-bindings/clock/mediatek,mt8188-clk.h>.
const CLK_CCU_LARB27: u32 = 0;
const CLK_CCU_AHB: u32 = 1;
const CLK_CCU_CCU0: u32 = 2;

static ccu_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

static ccu_clks: [mtk_gate; 3] = [
    mtk_gate {
        id: CLK_CCU_LARB27,
        name: b"ccu_larb27\0".as_ptr() as *const c_char,
        parent_name: b"top_ccu\0".as_ptr() as *const c_char,
        regs: &ccu_cg_regs,
        shift: 0,
        ops: unsafe { &mtk_clk_gate_ops_setclr },
    },
    mtk_gate {
        id: CLK_CCU_AHB,
        name: b"ccu_ahb\0".as_ptr() as *const c_char,
        parent_name: b"top_ccu\0".as_ptr() as *const c_char,
        regs: &ccu_cg_regs,
        shift: 1,
        ops: unsafe { &mtk_clk_gate_ops_setclr },
    },
    mtk_gate {
        id: CLK_CCU_CCU0,
        name: b"ccu_ccu0\0".as_ptr() as *const c_char,
        parent_name: b"top_ccu\0".as_ptr() as *const c_char,
        regs: &ccu_cg_regs,
        shift: 2,
        ops: unsafe { &mtk_clk_gate_ops_setclr },
    },
];

static ccu_desc: mtk_clk_desc = mtk_clk_desc {
    clks: ccu_clks.as_ptr(),
    num_clks: ccu_clks.len(),
};

static of_match_clk_mt8188_ccu: [of_device_id; 2] = [
    of_device_id {
        compatible: b"mediatek,mt8188-ccusys\0".as_ptr() as *const c_char,
        data: &ccu_desc as *const mtk_clk_desc as *const c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut clk_mt8188_ccu_drv: platform_driver = platform_driver {
    probe: Some(unsafe { core::mem::transmute(mtk_clk_simple_probe) }),
    remove: Some(unsafe { core::mem::transmute(mtk_clk_simple_remove) }),
    driver: device_driver {
        name: b"clk-mt8188-ccu\0".as_ptr() as *const c_char,
        of_match_table: of_match_clk_mt8188_ccu.as_ptr(),
    },
};

// module_platform_driver(clk_mt8188_ccu_drv);
// MODULE_DEVICE_TABLE(of, of_match_clk_mt8188_ccu);
// MODULE_DESCRIPTION("MediaTek MT8188 Camera Control Unit clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
