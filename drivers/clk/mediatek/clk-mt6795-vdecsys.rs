// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 Collabora Ltd.
 * Author: AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

// Dependencies supplied by the surrounding kernel clock framework:
// dt-bindings/clock/mediatek,mt6795-clk.h, linux/module.h,
// linux/platform_device.h, clk-gate.h, and clk-mtk.h.

extern "C" {
    static mtk_clk_gate_ops_setclr_inv: mtk_clk_gate_ops;
    fn mtk_clk_simple_probe(device: *mut platform_device) -> c_int;
    fn mtk_clk_simple_remove(device: *mut platform_device) -> c_int;
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
    name: *const c_char,
    parent_name: *const c_char,
    regs: *const mtk_gate_regs,
    shift: u32,
    ops: *const mtk_clk_gate_ops,
}

#[repr(C)]
struct mtk_clk_desc {
    clks: *const mtk_gate,
    num_clks: usize,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
    data: *const core::ffi::c_void,
}

#[repr(C)]
struct platform_driver_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: platform_driver_driver,
}

type c_int = core::ffi::c_int;
type c_char = core::ffi::c_char;
struct platform_device;
struct mtk_clk_gate_ops;

const CLK_VDEC_CKEN: u32 = 0;
const CLK_VDEC_LARB_CKEN: u32 = 1;

static VDEC0_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0000,
    clr_ofs: 0x0004,
    sta_ofs: 0x0000,
};

static VDEC1_CG_REGS: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0008,
    clr_ofs: 0x000c,
    sta_ofs: 0x0008,
};

static VDEC_CLKS: [mtk_gate; 2] = [
    mtk_gate {
        id: CLK_VDEC_CKEN,
        name: b"vdec_cken\0".as_ptr() as *const c_char,
        parent_name: b"vdec_sel\0".as_ptr() as *const c_char,
        regs: &VDEC0_CG_REGS,
        shift: 0,
        ops: unsafe { &mtk_clk_gate_ops_setclr_inv },
    },
    mtk_gate {
        id: CLK_VDEC_LARB_CKEN,
        name: b"vdec_larb_cken\0".as_ptr() as *const c_char,
        parent_name: b"mm_sel\0".as_ptr() as *const c_char,
        regs: &VDEC1_CG_REGS,
        shift: 0,
        ops: unsafe { &mtk_clk_gate_ops_setclr_inv },
    },
];

static VDEC_DESC: mtk_clk_desc = mtk_clk_desc {
    clks: VDEC_CLKS.as_ptr(),
    num_clks: VDEC_CLKS.len(),
};

static OF_MATCH_CLK_MT6795_VDECSYS: [of_device_id; 2] = [
    of_device_id {
        compatible: b"mediatek,mt6795-vdecsys\0".as_ptr() as *const c_char,
        data: &VDEC_DESC as *const mtk_clk_desc as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

static mut CLK_MT6795_VDECSYS_DRV: platform_driver = platform_driver {
    probe: Some(mtk_clk_simple_probe),
    remove: Some(mtk_clk_simple_remove),
    driver: platform_driver_driver {
        name: b"clk-mt6795-vdecsys\0".as_ptr() as *const c_char,
        of_match_table: OF_MATCH_CLK_MT6795_VDECSYS.as_ptr(),
    },
};

// Equivalent of module_platform_driver(clk_mt6795_vdecsys_drv).
// MODULE_DEVICE_TABLE(of, of_match_clk_mt6795_vdecsys);
// MODULE_DESCRIPTION("MediaTek MT6795 vdecsys clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
