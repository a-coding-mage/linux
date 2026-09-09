// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 Collabora Ltd.
 * Author: AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// Translated dependencies:
// <dt-bindings/clock/mediatek,mt6795-clk.h>
// <linux/module.h>
// <linux/platform_device.h>
// "clk-gate.h"
// "clk-mtk.h"

extern "C" {
    static mtk_clk_gate_ops mtk_clk_gate_ops_setclr_inv;
    fn mtk_clk_simple_probe();
    fn mtk_clk_simple_remove();
}

#[repr(C)]
struct mtk_gate_regs {
    set_ofs: u32,
    clr_ofs: u32,
    sta_ofs: u32,
}

#[repr(C)]
struct mtk_gate {
    id: u32,
    name: *const core::ffi::c_char,
    parent_name: *const core::ffi::c_char,
    regs: *const mtk_gate_regs,
    shift: u8,
    ops: *const mtk_clk_gate_ops,
}

#[repr(C)]
struct mtk_clk_gate_ops;

#[repr(C)]
struct mtk_clk_desc {
    clks: *const mtk_gate,
    num_clks: usize,
}

#[repr(C)]
struct of_device_id {
    compatible: *const core::ffi::c_char,
    data: *const core::ffi::c_void,
}

#[repr(C)]
struct platform_driver_driver {
    name: *const core::ffi::c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    driver: platform_driver_driver,
    probe: Option<unsafe extern "C" fn()>,
    remove: Option<unsafe extern "C" fn()>,
}

static venc_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x4,
    clr_ofs: 0x8,
    sta_ofs: 0x0,
};

macro_rules! gate_venc {
    ($id:expr, $name:expr, $parent:expr, $shift:expr) => {
        mtk_gate {
            id: $id,
            name: concat!($name, "\0").as_ptr() as *const core::ffi::c_char,
            parent_name: concat!($parent, "\0").as_ptr() as *const core::ffi::c_char,
            regs: &venc_cg_regs,
            shift: $shift,
            ops: unsafe { &mtk_clk_gate_ops_setclr_inv },
        }
    };
}

static venc_clks: [mtk_gate; 4] = [
    gate_venc!(CLK_VENC_LARB, "venc_larb", "venc_sel", 0),
    gate_venc!(CLK_VENC_VENC, "venc_venc", "venc_sel", 4),
    gate_venc!(CLK_VENC_JPGENC, "venc_jpgenc", "venc_sel", 8),
    gate_venc!(CLK_VENC_JPGDEC, "venc_jpgdec", "venc_sel", 12),
];

static venc_desc: mtk_clk_desc = mtk_clk_desc {
    clks: venc_clks.as_ptr(),
    num_clks: venc_clks.len(),
};

static of_match_clk_mt6795_vencsys: [of_device_id; 2] = [
    of_device_id {
        compatible: b"mediatek,mt6795-vencsys\0".as_ptr() as *const core::ffi::c_char,
        data: &venc_desc as *const _ as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut clk_mt6795_vencsys_drv: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: b"clk-mt6795-vencsys\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: of_match_clk_mt6795_vencsys.as_ptr(),
    },
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
};

// MODULE_DEVICE_TABLE(of, of_match_clk_mt6795_vencsys);
// module_platform_driver(clk_mt6795_vencsys_drv);
// MODULE_DESCRIPTION("MediaTek MT6795 vdecsys clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
