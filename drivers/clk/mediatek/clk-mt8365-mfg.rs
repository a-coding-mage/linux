// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022 MediaTek Inc.
 */

// Dependencies supplied by the surrounding kernel clock framework.

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
    pub shift: u32,
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
pub struct platform_driver_driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub driver: platform_driver_driver,
}

extern "C" {
    static mtk_clk_gate_ops_setclr: core::ffi::c_void;
    static mtk_clk_gate_ops_no_setclr: core::ffi::c_void;
    fn mtk_clk_simple_probe(dev: *mut core::ffi::c_void) -> i32;
    fn mtk_clk_simple_remove(dev: *mut core::ffi::c_void) -> i32;
    fn module_platform_driver(driver: *mut platform_driver);
}

const CLK_MFG_BG3D: u32 = 0;
const CLK_MFG_MBIST_DIAG: u32 = 1;

static mfg0_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

static mfg1_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x280,
    clr_ofs: 0x280,
    sta_ofs: 0x280,
};

static mfg_clks: [mtk_gate; 2] = [
    /* MFG0 */
    mtk_gate {
        id: CLK_MFG_BG3D,
        name: b"mfg_bg3d\0".as_ptr() as *const core::ffi::c_char,
        parent_name: b"mfg_sel\0".as_ptr() as *const core::ffi::c_char,
        regs: &mfg0_cg_regs,
        shift: 0,
        ops: unsafe { &mtk_clk_gate_ops_setclr },
    },
    /* MFG1 */
    mtk_gate {
        id: CLK_MFG_MBIST_DIAG,
        name: b"mfg_mbist_diag\0".as_ptr() as *const core::ffi::c_char,
        parent_name: b"mbist_diag_sel\0".as_ptr() as *const core::ffi::c_char,
        regs: &mfg1_cg_regs,
        shift: 24,
        ops: unsafe { &mtk_clk_gate_ops_no_setclr },
    },
];

static mfg_desc: mtk_clk_desc = mtk_clk_desc {
    clks: mfg_clks.as_ptr(),
    num_clks: mfg_clks.len(),
};

static of_match_clk_mt8365_mfg: [of_device_id; 2] = [
    of_device_id {
        compatible: b"mediatek,mt8365-mfgcfg\0".as_ptr() as *const core::ffi::c_char,
        data: &mfg_desc as *const _ as *const core::ffi::c_void,
    },
    of_device_id {
        /* sentinel */
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut clk_mt8365_mfg_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: platform_driver_driver {
        name: b"clk-mt8365-mfg\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: of_match_clk_mt8365_mfg.as_ptr(),
    },
};

#[used]
static MODULE_PLATFORM_DRIVER: unsafe extern "C" fn(*mut platform_driver) = module_platform_driver;

// MODULE_DESCRIPTION("MediaTek MT8365 GPU mfg clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
