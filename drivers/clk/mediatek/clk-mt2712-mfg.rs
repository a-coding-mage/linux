// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 MediaTek Inc.
 * Author: Weiyi Lu <weiyi.lu@mediatek.com>
 */

// Dependencies supplied by the Linux clock, platform-device, MediaTek clock,
// gate, and MT2712 clock-binding headers are intentionally external.

use core::ffi::c_char;

#[repr(C)]
struct MtkGateRegs {
    set_ofs: u32,
    clr_ofs: u32,
    sta_ofs: u32,
}

#[repr(C)]
struct MtkGate {
    id: u32,
    name: *const c_char,
    parent_name: *const c_char,
    regs: *const MtkGateRegs,
    shift: u8,
    ops: *const core::ffi::c_void,
}

#[repr(C)]
struct MtkClkDesc {
    clks: *const MtkGate,
    num_clks: usize,
}

#[repr(C)]
struct OfDeviceId {
    compatible: *const c_char,
    data: *const core::ffi::c_void,
}

#[repr(C)]
struct PlatformDriver {
    probe: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    remove: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    driver: Driver,
}

#[repr(C)]
struct Driver {
    name: *const c_char,
    of_match_table: *const OfDeviceId,
}

extern "C" {
    static mtk_clk_gate_ops_setclr: core::ffi::c_void;
    fn mtk_clk_simple_probe(dev: *mut core::ffi::c_void) -> i32;
    fn mtk_clk_simple_remove(dev: *mut core::ffi::c_void) -> i32;
}

const CLK_MFG_BG3D: u32 = 0;

static MFG_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

// GATE_MFG(_id, _name, _parent, _shift)
// expands to GATE_MTK(_id, _name, _parent, &mfg_cg_regs, _shift,
//                    &mtk_clk_gate_ops_setclr)
static MFG_CLKS: [MtkGate; 1] = [MtkGate {
    id: CLK_MFG_BG3D,
    name: b"mfg_bg3d\0".as_ptr() as *const c_char,
    parent_name: b"mfg_sel\0".as_ptr() as *const c_char,
    regs: &MFG_CG_REGS,
    shift: 0,
    ops: unsafe { &mtk_clk_gate_ops_setclr },
}];

static MFG_DESC: MtkClkDesc = MtkClkDesc {
    clks: MFG_CLKS.as_ptr(),
    num_clks: MFG_CLKS.len(),
};

static OF_MATCH_CLK_MT2712_MFG: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"mediatek,mt2712-mfgcfg\0".as_ptr() as *const c_char,
        data: &MFG_DESC as *const MtkClkDesc as *const core::ffi::c_void,
    },
    OfDeviceId {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut CLK_MT2712_MFG_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: Driver {
        name: b"clk-mt2712-mfg\0".as_ptr() as *const c_char,
        of_match_table: OF_MATCH_CLK_MT2712_MFG.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt2712_mfg);
// module_platform_driver(clk_mt2712_mfg_drv);
// MODULE_DESCRIPTION("MediaTek MT2712 GPU mfg clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
