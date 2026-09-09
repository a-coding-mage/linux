// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2020 MediaTek Inc.
 * Copyright (c) 2020 BayLibre, SAS
 * Author: James Liao <jamesjj.liao@mediatek.com>
 *         Fabien Parent <fparent@baylibre.com>
 */

// Translated from the Linux kernel implementation.  The declarations below
// are supplied by the corresponding kernel clock-provider dependencies.

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
    pub data: *const MtkClkDesc,
}

#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn() -> isize>,
    pub remove: Option<unsafe extern "C" fn() -> isize>,
    pub driver: Driver,
}

#[repr(C)]
pub struct Driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const OfDeviceId,
}

extern "C" {
    static mtk_clk_gate_ops_no_setclr: core::ffi::c_void;
    fn mtk_clk_simple_probe() -> isize;
    fn mtk_clk_simple_remove() -> isize;
}

const fn cstr(bytes: &'static [u8]) -> *const core::ffi::c_char {
    bytes.as_ptr() as *const core::ffi::c_char
}

static AUD_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x0,
    clr_ofs: 0x0,
    sta_ofs: 0x0,
};

// GATE_AUD(_id, _name, _parent, _shift)
const fn gate_aud(id: u32, name: &'static [u8], parent: &'static [u8], shift: u8) -> MtkGate {
    MtkGate {
        id,
        name: cstr(name),
        parent_name: cstr(parent),
        regs: &AUD_CG_REGS,
        shift,
        ops: unsafe { &mtk_clk_gate_ops_no_setclr },
    }
}

// Clock IDs are provided by <dt-bindings/clock/mt8167-clk.h>.
extern "C" {
    static CLK_AUD_AFE: u32;
    static CLK_AUD_I2S: u32;
    static CLK_AUD_22M: u32;
    static CLK_AUD_24M: u32;
    static CLK_AUD_INTDIR: u32;
    static CLK_AUD_APLL2_TUNER: u32;
    static CLK_AUD_APLL_TUNER: u32;
    static CLK_AUD_HDMI: u32;
    static CLK_AUD_SPDF: u32;
    static CLK_AUD_ADC: u32;
    static CLK_AUD_DAC: u32;
    static CLK_AUD_DAC_PREDIS: u32;
    static CLK_AUD_TML: u32;
}

static AUD_CLKS: [MtkGate; 13] = [
    gate_aud(unsafe { CLK_AUD_AFE }, b"aud_afe\0", b"clk26m_ck\0", 2),
    gate_aud(unsafe { CLK_AUD_I2S }, b"aud_i2s\0", b"i2s_infra_bck\0", 6),
    gate_aud(unsafe { CLK_AUD_22M }, b"aud_22m\0", b"rg_aud_engen1\0", 8),
    gate_aud(unsafe { CLK_AUD_24M }, b"aud_24m\0", b"rg_aud_engen2\0", 9),
    gate_aud(unsafe { CLK_AUD_INTDIR }, b"aud_intdir\0", b"rg_aud_spdif_in\0", 15),
    gate_aud(unsafe { CLK_AUD_APLL2_TUNER }, b"aud_apll2_tuner\0", b"rg_aud_engen2\0", 18),
    gate_aud(unsafe { CLK_AUD_APLL_TUNER }, b"aud_apll_tuner\0", b"rg_aud_engen1\0", 19),
    gate_aud(unsafe { CLK_AUD_HDMI }, b"aud_hdmi\0", b"apll12_div4\0", 20),
    gate_aud(unsafe { CLK_AUD_SPDF }, b"aud_spdf\0", b"apll12_div6\0", 21),
    gate_aud(unsafe { CLK_AUD_ADC }, b"aud_adc\0", b"aud_afe\0", 24),
    gate_aud(unsafe { CLK_AUD_DAC }, b"aud_dac\0", b"aud_afe\0", 25),
    gate_aud(unsafe { CLK_AUD_DAC_PREDIS }, b"aud_dac_predis\0", b"aud_afe\0", 26),
    gate_aud(unsafe { CLK_AUD_TML }, b"aud_tml\0", b"aud_afe\0", 27),
];

static AUD_DESC: MtkClkDesc = MtkClkDesc {
    clks: AUD_CLKS.as_ptr(),
    num_clks: AUD_CLKS.len(),
};

static OF_MATCH_CLK_MT8167_AUDSYS: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: cstr(b"mediatek,mt8167-audsys\0"),
        data: &AUD_DESC,
    },
    OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() },
];

static mut CLK_MT8167_AUDSYS_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: Driver {
        name: cstr(b"clk-mt8167-audsys\0"),
        of_match_table: OF_MATCH_CLK_MT8167_AUDSYS.as_ptr(),
    },
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt8167_audsys);
// module_platform_driver(clk_mt8167_audsys_drv);
// MODULE_DESCRIPTION("MediaTek MT8167 audio clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
