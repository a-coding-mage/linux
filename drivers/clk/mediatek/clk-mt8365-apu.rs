// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022 MediaTek Inc.
 */

// Dependencies supplied by the corresponding Linux clock-provider headers:
// <dt-bindings/clock/mediatek,mt8365-clk.h>
// <linux/clk-provider.h>
// <linux/platform_device.h>
// "clk-gate.h"
// "clk-mtk.h"

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

extern "C" {
    pub static mtk_clk_gate_ops_setclr: core::ffi::c_void;
    pub fn mtk_clk_simple_probe() -> core::ffi::c_int;
    pub fn mtk_clk_simple_remove() -> core::ffi::c_int;
}

static APU_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

// GATE_APU(_id, _name, _parent, _shift) expands to GATE_MTK with these
// register and operation arguments.
const fn gate_apu(
    id: u32,
    name: &'static [u8],
    parent: &'static [u8],
    shift: u8,
) -> MtkGate {
    MtkGate {
        id,
        name: name.as_ptr() as *const core::ffi::c_char,
        parent_name: parent.as_ptr() as *const core::ffi::c_char,
        regs: &APU_CG_REGS,
        shift,
        ops: core::ptr::addr_of!(mtk_clk_gate_ops_setclr),
    }
}

extern "C" {
    pub static CLK_APU_AHB: u32;
    pub static CLK_APU_EDMA: u32;
    pub static CLK_APU_IF_CK: u32;
    pub static CLK_APU_JTAG: u32;
    pub static CLK_APU_AXI: u32;
    pub static CLK_APU_IPU_CK: u32;
}

static APU_CLKS: [MtkGate; 6] = [
    gate_apu(CLK_APU_AHB, b"apu_ahb\0", b"ifr_apu_axi\0", 5),
    gate_apu(CLK_APU_EDMA, b"apu_edma\0", b"apu_sel\0", 4),
    gate_apu(CLK_APU_IF_CK, b"apu_if_ck\0", b"apu_if_sel\0", 3),
    gate_apu(CLK_APU_JTAG, b"apu_jtag\0", b"clk26m\0", 2),
    gate_apu(CLK_APU_AXI, b"apu_axi\0", b"apu_sel\0", 1),
    gate_apu(CLK_APU_IPU_CK, b"apu_ck\0", b"apu_sel\0", 0),
];

static APU_DESC: MtkClkDesc = MtkClkDesc {
    clks: APU_CLKS.as_ptr(),
    num_clks: APU_CLKS.len(),
};

static mut OF_MATCH_CLK_MT8365_APU: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"mediatek,mt8365-apu\0".as_ptr() as *const core::ffi::c_char,
        data: &APU_DESC as *const MtkClkDesc as *const core::ffi::c_void,
    },
    OfDeviceId {
        // sentinel
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8365_apu);

#[repr(C)]
struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn() -> core::ffi::c_int>,
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const OfDeviceId,
}

static mut CLK_MT8365_APU_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    name: b"clk-mt8365-apu\0".as_ptr() as *const core::ffi::c_char,
    of_match_table: unsafe { OF_MATCH_CLK_MT8365_APU.as_ptr() },
};

// module_platform_driver(clk_mt8365_apu_drv);
// MODULE_DESCRIPTION("MediaTek MT8365 AI Processing Unit clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
