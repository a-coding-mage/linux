// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Chun-Jie Chen <chun-jie.chen@mediatek.com>

// Dependencies corresponding to the C includes:
// linux/clk-provider.h, linux/platform_device.h,
// dt-bindings/clock/mt8186-clk.h, clk-gate.h, and clk-mtk.h.

#[repr(C)]
pub struct mtk_gate_regs {
    pub set_ofs: u32,
    pub clr_ofs: u32,
    pub sta_ofs: u32,
}

#[repr(C)]
pub struct mtk_gate {
    pub id: u32,
    pub name: *const core::ffi::c_char,
    pub parent_name: *const core::ffi::c_char,
    pub regs: *const mtk_gate_regs,
    pub shift: u8,
    pub ops: *const core::ffi::c_void,
}

#[repr(C)]
pub struct mtk_clk_desc {
    pub clks: *const mtk_gate,
    pub num_clks: usize,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub driver: driver,
}

#[repr(C)]
pub struct driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

extern "C" {
    pub static mtk_clk_gate_ops_setclr: core::ffi::c_void;
    pub fn mtk_clk_simple_probe(dev: *mut core::ffi::c_void) -> i32;
    pub fn mtk_clk_simple_remove(dev: *mut core::ffi::c_void) -> i32;
}

const fn gate_mtk(
    id: u32,
    name: &'static [u8],
    parent: &'static [u8],
    regs: &'static mtk_gate_regs,
    shift: u8,
    ops: *const core::ffi::c_void,
) -> mtk_gate {
    mtk_gate {
        id,
        name: name.as_ptr() as *const core::ffi::c_char,
        parent_name: parent.as_ptr() as *const core::ffi::c_char,
        regs: regs as *const mtk_gate_regs,
        shift,
        ops,
    }
}

static CAM_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

// GATE_CAM(_id, _name, _parent, _shift)
// expands to GATE_MTK(_id, _name, _parent, &cam_cg_regs, _shift,
//                     &mtk_clk_gate_ops_setclr).

static CAM_CLKS: [mtk_gate; 15] = [
    gate_mtk(CLK_CAM_LARB13, b"cam_larb13\0", b"top_cam\0", &CAM_CG_REGS, 0, unsafe { &mtk_clk_gate_ops_setclr }),
    gate_mtk(CLK_CAM_DFP_VAD, b"cam_dfp_vad\0", b"top_cam\0", &CAM_CG_REGS, 1, unsafe { &mtk_clk_gate_ops_setclr }),
    gate_mtk(CLK_CAM_LARB14, b"cam_larb14\0", b"top_cam\0", &CAM_CG_REGS, 2, unsafe { &mtk_clk_gate_ops_setclr }),
    gate_mtk(CLK_CAM, b"cam\0", b"top_cam\0", &CAM_CG_REGS, 6, unsafe { &mtk_clk_gate_ops_setclr }),
    gate_mtk(CLK_CAMTG, b"camtg\0", b"top_cam\0", &CAM_CG_REGS, 7, unsafe { &mtk_clk_gate_ops_setclr }),
    gate_mtk(CLK_CAM_SENINF, b"cam_seninf\0", b"top_cam\0", &CAM_CG_REGS, 8, unsafe { &mtk_clk_gate_ops_setclr }),
    gate_mtk(CLK_CAMSV1, b"camsv1\0", b"top_cam\0", &CAM_CG_REGS, 10, unsafe { &mtk_clk_gate_ops_setclr }),
    gate_mtk(CLK_CAMSV2, b"camsv2\0", b"top_cam\0", &CAM_CG_REGS, 11, unsafe { &mtk_clk_gate_ops_setclr }),
    gate_mtk(CLK_CAMSV3, b"camsv3\0", b"top_cam\0", &CAM_CG_REGS, 12, unsafe { &mtk_clk_gate_ops_setclr }),
    gate_mtk(CLK_CAM_CCU0, b"cam_ccu0\0", b"top_cam\0", &CAM_CG_REGS, 13, unsafe { &mtk_clk_gate_ops_setclr }),
    gate_mtk(CLK_CAM_CCU1, b"cam_ccu1\0", b"top_cam\0", &CAM_CG_REGS, 14, unsafe { &mtk_clk_gate_ops_setclr }),
    gate_mtk(CLK_CAM_MRAW0, b"cam_mraw0\0", b"top_cam\0", &CAM_CG_REGS, 15, unsafe { &mtk_clk_gate_ops_setclr }),
    gate_mtk(CLK_CAM_FAKE_ENG, b"cam_fake_eng\0", b"top_cam\0", &CAM_CG_REGS, 17, unsafe { &mtk_clk_gate_ops_setclr }),
    gate_mtk(CLK_CAM_CCU_GALS, b"cam_ccu_gals\0", b"top_cam\0", &CAM_CG_REGS, 18, unsafe { &mtk_clk_gate_ops_setclr }),
    gate_mtk(CLK_CAM2MM_GALS, b"cam2mm_gals\0", b"top_cam\0", &CAM_CG_REGS, 19, unsafe { &mtk_clk_gate_ops_setclr }),
];

static CAM_RAWA_CLKS: [mtk_gate; 3] = [
    gate_mtk(CLK_CAM_RAWA_LARBX_RAWA, b"cam_rawa_larbx_rawa\0", b"top_cam\0", &CAM_CG_REGS, 0, unsafe { &mtk_clk_gate_ops_setclr }),
    gate_mtk(CLK_CAM_RAWA, b"cam_rawa\0", b"top_cam\0", &CAM_CG_REGS, 1, unsafe { &mtk_clk_gate_ops_setclr }),
    gate_mtk(CLK_CAM_RAWA_CAMTG_RAWA, b"cam_rawa_camtg_rawa\0", b"top_cam\0", &CAM_CG_REGS, 2, unsafe { &mtk_clk_gate_ops_setclr }),
];

static CAM_RAWB_CLKS: [mtk_gate; 3] = [
    gate_mtk(CLK_CAM_RAWB_LARBX_RAWB, b"cam_rawb_larbx_rawb\0", b"top_cam\0", &CAM_CG_REGS, 0, unsafe { &mtk_clk_gate_ops_setclr }),
    gate_mtk(CLK_CAM_RAWB, b"cam_rawb\0", b"top_cam\0", &CAM_CG_REGS, 1, unsafe { &mtk_clk_gate_ops_setclr }),
    gate_mtk(CLK_CAM_RAWB_CAMTG_RAWB, b"cam_rawb_camtg_rawb\0", b"top_cam\0", &CAM_CG_REGS, 2, unsafe { &mtk_clk_gate_ops_setclr }),
];

static CAM_DESC: mtk_clk_desc = mtk_clk_desc { clks: CAM_CLKS.as_ptr(), num_clks: CAM_CLKS.len() };
static CAM_RAWA_DESC: mtk_clk_desc = mtk_clk_desc { clks: CAM_RAWA_CLKS.as_ptr(), num_clks: CAM_RAWA_CLKS.len() };
static CAM_RAWB_DESC: mtk_clk_desc = mtk_clk_desc { clks: CAM_RAWB_CLKS.as_ptr(), num_clks: CAM_RAWB_CLKS.len() };

static OF_MATCH_CLK_MT8186_CAM: [of_device_id; 4] = [
    of_device_id { compatible: b"mediatek,mt8186-camsys\0".as_ptr() as _, data: &CAM_DESC as *const _ as _ },
    of_device_id { compatible: b"mediatek,mt8186-camsys_rawa\0".as_ptr() as _, data: &CAM_RAWA_DESC as *const _ as _ },
    of_device_id { compatible: b"mediatek,mt8186-camsys_rawb\0".as_ptr() as _, data: &CAM_RAWB_DESC as *const _ as _ },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() }, // sentinel
];

static mut CLK_MT8186_CAM_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: driver {
        name: b"clk-mt8186-cam\0".as_ptr() as _,
        of_match_table: OF_MATCH_CLK_MT8186_CAM.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8186_cam);
// module_platform_driver(clk_mt8186_cam_drv);
// MODULE_DESCRIPTION("MediaTek MT8186 Camera clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
