// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// C dependencies: clk-gate.h, clk-mtk.h, dt-bindings/clock/mt8195-clk.h,
// linux/clk-provider.h, and linux/platform_device.h.

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct mtk_gate_regs {
    pub set_ofs: u32,
    pub clr_ofs: u32,
    pub sta_ofs: u32,
}

#[repr(C)]
pub struct mtk_gate {
    pub id: u32,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub regs: *const mtk_gate_regs,
    pub shift: u8,
    pub ops: *const c_void,
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
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut c_void) -> i32>,
    pub driver: driver,
}

#[repr(C)]
pub struct driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

extern "C" {
    static mtk_clk_gate_ops_setclr: c_void;
    fn mtk_clk_simple_probe(pdev: *mut c_void) -> i32;
    fn mtk_clk_simple_remove(pdev: *mut c_void) -> i32;
}

static img_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

macro_rules! gate_img {
    ($id:expr, $name:literal, $parent:literal, $shift:expr) => {
        mtk_gate {
            id: $id,
            name: concat!($name, "\0").as_ptr() as *const c_char,
            parent_name: concat!($parent, "\0").as_ptr() as *const c_char,
            regs: &img_cg_regs,
            shift: $shift,
            ops: unsafe { &mtk_clk_gate_ops_setclr },
        }
    };
}

// Clock IDs are supplied by dt-bindings/clock/mt8195-clk.h.
extern "C" {
    static CLK_IMG_LARB9: u32;
    static CLK_IMG_TRAW0: u32;
    static CLK_IMG_TRAW1: u32;
    static CLK_IMG_TRAW2: u32;
    static CLK_IMG_TRAW3: u32;
    static CLK_IMG_DIP0: u32;
    static CLK_IMG_WPE0: u32;
    static CLK_IMG_IPE: u32;
    static CLK_IMG_DIP1: u32;
    static CLK_IMG_WPE1: u32;
    static CLK_IMG_GALS: u32;
    static CLK_IMG1_DIP_TOP_LARB10: u32;
    static CLK_IMG1_DIP_TOP_DIP_TOP: u32;
    static CLK_IMG1_DIP_NR_RESERVE: u32;
    static CLK_IMG1_DIP_NR_DIP_NR: u32;
    static CLK_IMG1_WPE_LARB11: u32;
    static CLK_IMG1_WPE_WPE: u32;
}

static img_clks: [mtk_gate; 11] = [
    gate_img!(unsafe { CLK_IMG_LARB9 }, "img_larb9", "top_img", 0),
    gate_img!(unsafe { CLK_IMG_TRAW0 }, "img_traw0", "top_img", 1),
    gate_img!(unsafe { CLK_IMG_TRAW1 }, "img_traw1", "top_img", 2),
    gate_img!(unsafe { CLK_IMG_TRAW2 }, "img_traw2", "top_img", 3),
    gate_img!(unsafe { CLK_IMG_TRAW3 }, "img_traw3", "top_img", 4),
    gate_img!(unsafe { CLK_IMG_DIP0 }, "img_dip0", "top_img", 8),
    gate_img!(unsafe { CLK_IMG_WPE0 }, "img_wpe0", "top_img", 9),
    gate_img!(unsafe { CLK_IMG_IPE }, "img_ipe", "top_img", 10),
    gate_img!(unsafe { CLK_IMG_DIP1 }, "img_dip1", "top_img", 11),
    gate_img!(unsafe { CLK_IMG_WPE1 }, "img_wpe1", "top_img", 12),
    gate_img!(unsafe { CLK_IMG_GALS }, "img_gals", "top_img", 31),
];

static img1_dip_top_clks: [mtk_gate; 2] = [
    gate_img!(unsafe { CLK_IMG1_DIP_TOP_LARB10 }, "img1_dip_top_larb10", "top_img", 0),
    gate_img!(unsafe { CLK_IMG1_DIP_TOP_DIP_TOP }, "img1_dip_top_dip_top", "top_img", 1),
];

static img1_dip_nr_clks: [mtk_gate; 2] = [
    gate_img!(unsafe { CLK_IMG1_DIP_NR_RESERVE }, "img1_dip_nr_reserve", "top_img", 0),
    gate_img!(unsafe { CLK_IMG1_DIP_NR_DIP_NR }, "img1_dip_nr_dip_nr", "top_img", 1),
];

static img1_wpe_clks: [mtk_gate; 2] = [
    gate_img!(unsafe { CLK_IMG1_WPE_LARB11 }, "img1_wpe_larb11", "top_img", 0),
    gate_img!(unsafe { CLK_IMG1_WPE_WPE }, "img1_wpe_wpe", "top_img", 1),
];

static img_desc: mtk_clk_desc = mtk_clk_desc { clks: img_clks.as_ptr(), num_clks: img_clks.len() };
static img1_dip_top_desc: mtk_clk_desc = mtk_clk_desc { clks: img1_dip_top_clks.as_ptr(), num_clks: img1_dip_top_clks.len() };
static img1_dip_nr_desc: mtk_clk_desc = mtk_clk_desc { clks: img1_dip_nr_clks.as_ptr(), num_clks: img1_dip_nr_clks.len() };
static img1_wpe_desc: mtk_clk_desc = mtk_clk_desc { clks: img1_wpe_clks.as_ptr(), num_clks: img1_wpe_clks.len() };

static of_match_clk_mt8195_img: [of_device_id; 5] = [
    of_device_id { compatible: b"mediatek,mt8195-imgsys\0".as_ptr() as *const c_char, data: &img_desc as *const _ as *const c_void },
    of_device_id { compatible: b"mediatek,mt8195-imgsys1_dip_top\0".as_ptr() as *const c_char, data: &img1_dip_top_desc as *const _ as *const c_void },
    of_device_id { compatible: b"mediatek,mt8195-imgsys1_dip_nr\0".as_ptr() as *const c_char, data: &img1_dip_nr_desc as *const _ as *const c_void },
    of_device_id { compatible: b"mediatek,mt8195-imgsys1_wpe\0".as_ptr() as *const c_char, data: &img1_wpe_desc as *const _ as *const c_void },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() }, // sentinel
];

static mut clk_mt8195_img_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver {
        name: b"clk-mt8195-img\0".as_ptr() as *const c_char,
        of_match_table: of_match_clk_mt8195_img.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8195_img);
// module_platform_driver(clk_mt8195_img_drv);
// MODULE_DESCRIPTION("MediaTek MT8195 imgsys clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
