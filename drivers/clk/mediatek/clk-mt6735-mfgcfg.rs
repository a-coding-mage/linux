// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2022 Yassine Oudjana <y.oudjana@protonmail.com>
 */

// Dependencies supplied by the Linux clock, platform-device, gate, Mediatek
// clock, and MT6735 clock-binding headers are intentionally external.

const MFG_CG_CON: u32 = 0x00;
const MFG_CG_SET: u32 = 0x04;
const MFG_CG_CLR: u32 = 0x08;
const MFG_RESET: u32 = 0x0c;

extern "C" {
    static mtk_clk_gate_ops_setclr: mtk_clk_ops;
    fn mtk_clk_simple_probe(device: *mut platform_device) -> i32;
    fn mtk_clk_simple_remove(device: *mut platform_device) -> i32;
}

#[repr(C)]
pub struct mtk_gate_regs {
    pub set_ofs: u32,
    pub clr_ofs: u32,
    pub sta_ofs: u32,
}

#[repr(C)]
pub struct mtk_clk_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_gate {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_clk_rst_desc {
    pub version: u32,
    pub rst_bank_ofs: *mut u16,
    pub rst_bank_nr: usize,
}

#[repr(C)]
pub struct mtk_clk_desc {
    pub clks: *const mtk_gate,
    pub num_clks: usize,
    pub rst_desc: *const mtk_clk_rst_desc,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const u8,
    pub data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const u8,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub driver: platform_driver_inner,
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

const MTK_RST_SIMPLE: u32 = 0;

static mut mfgcfg_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: MFG_CG_SET,
    clr_ofs: MFG_CG_CLR,
    sta_ofs: MFG_CG_CON,
};

// C: GATE_MTK(CLK_MFG_BG3D, "bg3d", "mfg_sel", &mfgcfg_cg_regs, 0,
//             &mtk_clk_gate_ops_setclr)
static mfgcfg_gates: [mtk_gate; 1] = [mtk_gate { _private: [] }];

static mut mfgcfg_rst_ofs: [u16; 1] = [MFG_RESET as u16];

static mfgcfg_resets: mtk_clk_rst_desc = mtk_clk_rst_desc {
    version: MTK_RST_SIMPLE,
    rst_bank_ofs: mfgcfg_rst_ofs.as_ptr() as *mut u16,
    rst_bank_nr: mfgcfg_rst_ofs.len(),
};

static mfgcfg_clks: mtk_clk_desc = mtk_clk_desc {
    clks: mfgcfg_gates.as_ptr(),
    num_clks: mfgcfg_gates.len(),
    rst_desc: &mfgcfg_resets,
};

static of_match_mt6735_mfgcfg: [of_device_id; 2] = [
    of_device_id {
        compatible: b"mediatek,mt6735-mfgcfg\0".as_ptr(),
        data: &mfgcfg_clks as *const mtk_clk_desc as *const core::ffi::c_void,
    },
    of_device_id {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut clk_mt6735_mfgcfg: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: platform_driver_inner {
        name: b"clk-mt6735-mfgcfg\0".as_ptr(),
        of_match_table: of_match_mt6735_mfgcfg.as_ptr(),
    },
};

// module_platform_driver(clk_mt6735_mfgcfg);

// MODULE_AUTHOR("Yassine Oudjana <y.oudjana@protonmail.com>");
// MODULE_DESCRIPTION("Mediatek MT6735 mfgcfg clock and reset driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
