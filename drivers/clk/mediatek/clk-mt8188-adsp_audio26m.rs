// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Garmin Chang <garmin.chang@mediatek.com>
 */

// Dependencies supplied by the surrounding MediaTek clock framework.

unsafe extern "C" {
    static mtk_clk_gate_ops_no_setclr: mtk_clk_gate_ops;
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
pub struct mtk_gate {
    pub id: u32,
    pub name: *const core::ffi::c_char,
    pub parent_name: *const core::ffi::c_char,
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
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub driver: platform_driver_driver,
}

#[repr(C)]
pub struct platform_device;

#[repr(C)]
pub struct mtk_clk_gate_ops;

const adsp_audio26m_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x80,
    clr_ofs: 0x80,
    sta_ofs: 0x80,
};

// GATE_ADSP_FLAGS(_id, _name, _parent, _shift)
const fn gate_adsp_flags(
    id: u32,
    name: *const core::ffi::c_char,
    parent: *const core::ffi::c_char,
    shift: u32,
) -> mtk_gate {
    mtk_gate {
        id,
        name,
        parent_name: parent,
        regs: &adsp_audio26m_cg_regs,
        shift,
        ops: unsafe { &mtk_clk_gate_ops_no_setclr },
    }
}

unsafe extern "C" {
    static CLK_AUDIODSP_AUDIO26M: u32;
}

static adsp_audio26m_clks: [mtk_gate; 1] = [gate_adsp_flags(
    unsafe { CLK_AUDIODSP_AUDIO26M },
    b"audiodsp_audio26m\0".as_ptr() as *const core::ffi::c_char,
    b"clk26m\0".as_ptr() as *const core::ffi::c_char,
    3,
)];

static adsp_audio26m_desc: mtk_clk_desc = mtk_clk_desc {
    clks: adsp_audio26m_clks.as_ptr(),
    num_clks: adsp_audio26m_clks.len(),
};

static of_match_clk_mt8188_adsp_audio26m: [of_device_id; 2] = [
    of_device_id {
        compatible: b"mediatek,mt8188-adsp-audio26m\0".as_ptr() as *const core::ffi::c_char,
        data: &adsp_audio26m_desc as *const _ as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut clk_mt8188_adsp_audio26m_drv: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: platform_driver_driver {
        name: b"clk-mt8188-adsp_audio26m\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: of_match_clk_mt8188_adsp_audio26m.as_ptr(),
    },
};

// module_platform_driver(clk_mt8188_adsp_audio26m_drv);
// MODULE_DEVICE_TABLE(of, of_match_clk_mt8188_adsp_audio26m);
// MODULE_DESCRIPTION("MediaTek MT8188 AudioDSP clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
