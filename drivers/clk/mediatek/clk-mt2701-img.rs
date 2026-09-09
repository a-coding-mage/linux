// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Author: Shunli Wang <shunli.wang@mediatek.com>
 */

// Dependency headers from the original implementation:
// <linux/clk-provider.h>, <linux/platform_device.h>, "clk-mtk.h",
// "clk-gate.h", and <dt-bindings/clock/mt2701-clk.h>.

#[repr(C)]
pub struct MtkGateRegs {
    pub set_ofs: u32,
    pub clr_ofs: u32,
    pub sta_ofs: u32,
}

#[repr(C)]
pub struct MtkGate {
    pub id: u32,
    pub name: *const core::ffi::c_char,
    pub parent_name: *const core::ffi::c_char,
    pub regs: *const MtkGateRegs,
    pub shift: u8,
    pub ops: *const core::ffi::c_void,
}

#[repr(C)]
pub struct MtkClkDesc {
    pub clks: *const MtkGate,
    pub num_clks: usize,
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub driver: Driver,
}

#[repr(C)]
pub struct Driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const OfDeviceId,
}

// The following clock identifiers and gate-operation symbols are supplied by
// the corresponding kernel headers and other translation units.
extern "C" {
    pub static mtk_clk_gate_ops_setclr: core::ffi::c_void;
    pub fn mtk_clk_simple_probe(device: *mut core::ffi::c_void) -> i32;
    pub fn mtk_clk_simple_remove(device: *mut core::ffi::c_void) -> i32;
}

static IMG_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x0004,
    clr_ofs: 0x0008,
    sta_ofs: 0x0000,
};

// GATE_MTK(_id, _name, _parent, _regs, _shift, _ops)
const fn gate_mtk(
    id: u32,
    name: &'static [u8],
    parent: &'static [u8],
    regs: &'static MtkGateRegs,
    shift: u8,
    ops: *const core::ffi::c_void,
) -> MtkGate {
    MtkGate {
        id,
        name: name.as_ptr() as *const core::ffi::c_char,
        parent_name: parent.as_ptr() as *const core::ffi::c_char,
        regs,
        shift,
        ops,
    }
}

const fn gate_dummy(id: u32, name: &'static [u8]) -> MtkGate {
    MtkGate {
        id,
        name: name.as_ptr() as *const core::ffi::c_char,
        parent_name: core::ptr::null(),
        regs: core::ptr::null(),
        shift: 0,
        ops: core::ptr::null(),
    }
}

const fn gate_img(id: u32, name: &'static [u8], shift: u8) -> MtkGate {
    gate_mtk(
        id,
        name,
        b"mm_sel\0",
        &IMG_CG_REGS,
        shift,
        core::ptr::addr_of!(mtk_clk_gate_ops_setclr),
    )
}

static IMG_CLKS: [MtkGate; 7] = [
    gate_dummy(CLK_DUMMY, b"img_dummy\0"),
    gate_img(CLK_IMG_SMI_COMM, b"img_smi_comm\0", 0),
    gate_img(CLK_IMG_RESZ, b"img_resz\0", 1),
    gate_img(CLK_IMG_JPGDEC_SMI, b"img_jpgdec_smi\0", 5),
    gate_img(CLK_IMG_JPGDEC, b"img_jpgdec\0", 6),
    gate_img(CLK_IMG_VENC_LT, b"img_venc_lt\0", 8),
    gate_img(CLK_IMG_VENC, b"img_venc\0", 9),
];

static IMG_DESC: MtkClkDesc = MtkClkDesc {
    clks: IMG_CLKS.as_ptr(),
    num_clks: IMG_CLKS.len(),
};

static OF_MATCH_CLK_MT2701_IMG: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"mediatek,mt2701-imgsys\0".as_ptr() as *const core::ffi::c_char,
        data: core::ptr::addr_of!(IMG_DESC) as *const core::ffi::c_void,
    },
    OfDeviceId {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut CLK_MT2701_IMG_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: Driver {
        name: b"clk-mt2701-img\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: OF_MATCH_CLK_MT2701_IMG.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt2701_img);
// module_platform_driver(clk_mt2701_img_drv);
// MODULE_DESCRIPTION("MediaTek MT2701 imgsys clocks driver");
// MODULE_LICENSE("GPL");

// Clock identifiers supplied by dt-bindings/clock/mt2701-clk.h.
extern "C" {
    static CLK_DUMMY: u32;
    static CLK_IMG_SMI_COMM: u32;
    static CLK_IMG_RESZ: u32;
    static CLK_IMG_JPGDEC_SMI: u32;
    static CLK_IMG_JPGDEC: u32;
    static CLK_IMG_VENC_LT: u32;
    static CLK_IMG_VENC: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
