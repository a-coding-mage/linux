// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2018 MediaTek Inc.
// Author: Weiyi Lu <weiyi.lu@mediatek.com>

use core::ffi::{c_char, c_void};

// Types, constants, and operations below are supplied by the corresponding
// Linux MediaTek clock-provider dependencies.
use crate::clk_mtk::{mtk_clk_desc, mtk_gate, mtk_gate_regs};
use crate::clk_gate::{clk_gate_ops_setclr, mtk_clk_gate_ops_setclr};
use crate::platform::{mtk_clk_simple_probe, mtk_clk_simple_remove, platform_driver};
use crate::clock_bindings::{CLK_IPU_CORE0_AXI, CLK_IPU_CORE0_IPU, CLK_IPU_CORE0_JTAG};

static IPU_CORE0_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

macro_rules! gate_ipu_core0 {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        mtk_gate {
            id: $id,
            name: $name.as_ptr() as *const c_char,
            parent_name: $parent.as_ptr() as *const c_char,
            regs: &IPU_CORE0_CG_REGS,
            shift: $shift,
            ops: &mtk_clk_gate_ops_setclr,
        }
    };
}

static IPU_CORE0_CLKS: [mtk_gate; 3] = [
    gate_ipu_core0!(CLK_IPU_CORE0_JTAG, "ipu_core0_jtag", "dsp_sel", 0),
    gate_ipu_core0!(CLK_IPU_CORE0_AXI, "ipu_core0_axi", "dsp_sel", 1),
    gate_ipu_core0!(CLK_IPU_CORE0_IPU, "ipu_core0_ipu", "dsp_sel", 2),
];

static IPU_CORE0_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: IPU_CORE0_CLKS.as_ptr(),
    num_clks: IPU_CORE0_CLKS.len(),
};

#[repr(C)]
struct OfDeviceId {
    compatible: *const c_char,
    data: *const c_void,
}

static OF_MATCH_CLK_MT8183_IPU_CORE0: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"mediatek,mt8183-ipu_core0\0".as_ptr() as *const c_char,
        data: &IPU_CORE0_DESC as *const _ as *const c_void,
    },
    OfDeviceId {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut CLK_MT8183_IPU_CORE0_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: crate::platform::device_driver {
        name: b"clk-mt8183-ipu_core0\0".as_ptr() as *const c_char,
        of_match_table: OF_MATCH_CLK_MT8183_IPU_CORE0.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8183_ipu_core0)
// module_platform_driver(clk_mt8183_ipu_core0_drv)
// MODULE_DESCRIPTION("MediaTek MT8183 Pri. Image Processing Unit clocks driver")
// MODULE_LICENSE("GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
