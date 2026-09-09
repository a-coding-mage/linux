// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Garmin Chang <garmin.chang@mediatek.com>
 */

// Dependencies supplied by the kernel clock and platform layers:
// <dt-bindings/clock/mediatek,mt8188-clk.h>
// <linux/clk-provider.h>
// <linux/platform_device.h>
// "clk-gate.h"
// "clk-mtk.h"

#[repr(C)]
struct MtkGateRegs {
    set_ofs: u32,
    clr_ofs: u32,
    sta_ofs: u32,
}

#[repr(C)]
struct MtkGate {
    id: u32,
    name: *const core::ffi::c_char,
    parent_name: *const core::ffi::c_char,
    regs: *const MtkGateRegs,
    shift: u32,
    ops: *const core::ffi::c_void,
}

#[repr(C)]
struct MtkClkDesc {
    clks: *const MtkGate,
    num_clks: usize,
}

#[repr(C)]
struct PlatformDeviceId {
    name: *const core::ffi::c_char,
    driver_data: usize,
}

#[repr(C)]
struct PlatformDriver {
    probe: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    remove: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    driver: Driver,
    id_table: *const PlatformDeviceId,
}

#[repr(C)]
struct Driver {
    name: *const core::ffi::c_char,
}

extern "C" {
    static mtk_clk_gate_ops_setclr: core::ffi::c_void;
    static mtk_clk_pdev_probe: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32;
    static mtk_clk_pdev_remove: unsafe extern "C" fn(*mut core::ffi::c_void) -> i32;
}

static VPP1_0_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x104,
    clr_ofs: 0x108,
    sta_ofs: 0x100,
};

static VPP1_1_CG_REGS: MtkGateRegs = MtkGateRegs {
    set_ofs: 0x114,
    clr_ofs: 0x118,
    sta_ofs: 0x110,
};

macro_rules! gate_mtk {
    ($id:expr, $name:literal, $parent:literal, $regs:expr, $shift:expr) => {
        MtkGate {
            id: $id,
            name: concat!($name, "\0").as_ptr() as *const core::ffi::c_char,
            parent_name: concat!($parent, "\0").as_ptr() as *const core::ffi::c_char,
            regs: $regs,
            shift: $shift,
            ops: unsafe { &mtk_clk_gate_ops_setclr as *const _ as *const core::ffi::c_void },
        }
    };
}

macro_rules! gate_vpp1_0 {
    ($id:expr, $name:literal, $parent:literal, $shift:expr) => {
        gate_mtk!($id, $name, $parent, &VPP1_0_CG_REGS, $shift)
    };
}

macro_rules! gate_vpp1_1 {
    ($id:expr, $name:literal, $parent:literal, $shift:expr) => {
        gate_mtk!($id, $name, $parent, &VPP1_1_CG_REGS, $shift)
    };
}

static VPP1_CLKS: &[MtkGate] = &[
    // VPP1_0
    gate_vpp1_0!(CLK_VPP1_SVPP1_MDP_OVL, "vpp1_svpp1_mdp_ovl", "top_vpp", 0),
    gate_vpp1_0!(CLK_VPP1_SVPP1_MDP_TCC, "vpp1_svpp1_mdp_tcc", "top_vpp", 1),
    gate_vpp1_0!(CLK_VPP1_SVPP1_MDP_WROT, "vpp1_svpp1_mdp_wrot", "top_vpp", 2),
    gate_vpp1_0!(CLK_VPP1_SVPP1_VPP_PAD, "vpp1_svpp1_vpp_pad", "top_vpp", 3),
    gate_vpp1_0!(CLK_VPP1_SVPP2_MDP_WROT, "vpp1_svpp2_mdp_wrot", "top_vpp", 4),
    gate_vpp1_0!(CLK_VPP1_SVPP2_VPP_PAD, "vpp1_svpp2_vpp_pad", "top_vpp", 5),
    gate_vpp1_0!(CLK_VPP1_SVPP3_MDP_WROT, "vpp1_svpp3_mdp_wrot", "top_vpp", 6),
    gate_vpp1_0!(CLK_VPP1_SVPP3_VPP_PAD, "vpp1_svpp3_vpp_pad", "top_vpp", 7),
    gate_vpp1_0!(CLK_VPP1_SVPP1_MDP_RDMA, "vpp1_svpp1_mdp_rdma", "top_vpp", 8),
    gate_vpp1_0!(CLK_VPP1_SVPP1_MDP_FG, "vpp1_svpp1_mdp_fg", "top_vpp", 9),
    gate_vpp1_0!(CLK_VPP1_SVPP2_MDP_RDMA, "vpp1_svpp2_mdp_rdma", "top_vpp", 10),
    gate_vpp1_0!(CLK_VPP1_SVPP2_MDP_FG, "vpp1_svpp2_mdp_fg", "top_vpp", 11),
    gate_vpp1_0!(CLK_VPP1_SVPP3_MDP_RDMA, "vpp1_svpp3_mdp_rdma", "top_vpp", 12),
    gate_vpp1_0!(CLK_VPP1_SVPP3_MDP_FG, "vpp1_svpp3_mdp_fg", "top_vpp", 13),
    gate_vpp1_0!(CLK_VPP1_VPP_SPLIT, "vpp1_vpp_split", "top_vpp", 14),
    gate_vpp1_0!(CLK_VPP1_SVPP2_VDO0_DL_RELAY, "vpp1_svpp2_vdo0_dl_relay", "top_vpp", 15),
    gate_vpp1_0!(CLK_VPP1_SVPP1_MDP_RSZ, "vpp1_svpp1_mdp_rsz", "top_vpp", 16),
    gate_vpp1_0!(CLK_VPP1_SVPP1_MDP_TDSHP, "vpp1_svpp1_mdp_tdshp", "top_vpp", 17),
    gate_vpp1_0!(CLK_VPP1_SVPP1_MDP_COLOR, "vpp1_svpp1_mdp_color", "top_vpp", 18),
    gate_vpp1_0!(CLK_VPP1_SVPP3_VDO1_DL_RELAY, "vpp1_svpp3_vdo1_dl_relay", "top_vpp", 19),
    gate_vpp1_0!(CLK_VPP1_SVPP2_MDP_RSZ, "vpp1_svpp2_mdp_rsz", "top_vpp", 20),
    gate_vpp1_0!(CLK_VPP1_SVPP2_VPP_MERGE, "vpp1_svpp2_vpp_merge", "top_vpp", 21),
    gate_vpp1_0!(CLK_VPP1_SVPP2_MDP_TDSHP, "vpp1_svpp2_mdp_tdshp", "top_vpp", 22),
    gate_vpp1_0!(CLK_VPP1_SVPP2_MDP_COLOR, "vpp1_svpp2_mdp_color", "top_vpp", 23),
    gate_vpp1_0!(CLK_VPP1_SVPP3_MDP_RSZ, "vpp1_svpp3_mdp_rsz", "top_vpp", 24),
    gate_vpp1_0!(CLK_VPP1_SVPP3_VPP_MERGE, "vpp1_svpp3_vpp_merge", "top_vpp", 25),
    gate_vpp1_0!(CLK_VPP1_SVPP3_MDP_TDSHP, "vpp1_svpp3_mdp_tdshp", "top_vpp", 26),
    gate_vpp1_0!(CLK_VPP1_SVPP3_MDP_COLOR, "vpp1_svpp3_mdp_color", "top_vpp", 27),
    gate_vpp1_0!(CLK_VPP1_GALS5, "vpp1_gals5", "top_vpp", 28),
    gate_vpp1_0!(CLK_VPP1_GALS6, "vpp1_gals6", "top_vpp", 29),
    gate_vpp1_0!(CLK_VPP1_LARB5, "vpp1_larb5", "top_vpp", 30),
    gate_vpp1_0!(CLK_VPP1_LARB6, "vpp1_larb6", "top_vpp", 31),
    // VPP1_1
    gate_vpp1_1!(CLK_VPP1_SVPP1_MDP_HDR, "vpp1_svpp1_mdp_hdr", "top_vpp", 0),
    gate_vpp1_1!(CLK_VPP1_SVPP1_MDP_AAL, "vpp1_svpp1_mdp_aal", "top_vpp", 1),
    gate_vpp1_1!(CLK_VPP1_SVPP2_MDP_HDR, "vpp1_svpp2_mdp_hdr", "top_vpp", 2),
    gate_vpp1_1!(CLK_VPP1_SVPP2_MDP_AAL, "vpp1_svpp2_mdp_aal", "top_vpp", 3),
    gate_vpp1_1!(CLK_VPP1_SVPP3_MDP_HDR, "vpp1_svpp3_mdp_hdr", "top_vpp", 4),
    gate_vpp1_1!(CLK_VPP1_SVPP3_MDP_AAL, "vpp1_svpp3_mdp_aal", "top_vpp", 5),
    gate_vpp1_1!(CLK_VPP1_DISP_MUTEX, "vpp1_disp_mutex", "top_vpp", 7),
    gate_vpp1_1!(CLK_VPP1_SVPP2_VDO1_DL_RELAY, "vpp1_svpp2_vdo1_dl_relay", "top_vpp", 8),
    gate_vpp1_1!(CLK_VPP1_SVPP3_VDO0_DL_RELAY, "vpp1_svpp3_vdo0_dl_relay", "top_vpp", 9),
    gate_vpp1_1!(CLK_VPP1_VPP0_DL_ASYNC, "vpp1_vpp0_dl_async", "top_vpp", 10),
    gate_vpp1_1!(CLK_VPP1_VPP0_DL1_RELAY, "vpp1_vpp0_dl1_relay", "top_vpp", 11),
    gate_vpp1_1!(CLK_VPP1_LARB5_FAKE_ENG, "vpp1_larb5_fake_eng", "top_vpp", 12),
    gate_vpp1_1!(CLK_VPP1_LARB6_FAKE_ENG, "vpp1_larb6_fake_eng", "top_vpp", 13),
    gate_vpp1_1!(CLK_VPP1_HDMI_META, "vpp1_hdmi_meta", "top_vpp", 16),
    gate_vpp1_1!(CLK_VPP1_VPP_SPLIT_HDMI, "vpp1_vpp_split_hdmi", "top_vpp", 17),
    gate_vpp1_1!(CLK_VPP1_DGI_IN, "vpp1_dgi_in", "top_vpp", 18),
    gate_vpp1_1!(CLK_VPP1_DGI_OUT, "vpp1_dgi_out", "top_vpp", 19),
    gate_vpp1_1!(CLK_VPP1_VPP_SPLIT_DGI, "vpp1_vpp_split_dgi", "top_vpp", 20),
    gate_vpp1_1!(CLK_VPP1_DL_CON_OCC, "vpp1_dl_con_occ", "top_vpp", 21),
    gate_vpp1_1!(CLK_VPP1_VPP_SPLIT_26M, "vpp1_vpp_split_26m", "top_vpp", 26),
];

static VPP1_DESC: MtkClkDesc = MtkClkDesc {
    clks: VPP1_CLKS.as_ptr(),
    num_clks: VPP1_CLKS.len(),
};

static CLK_MT8188_VPP1_ID_TABLE: &[PlatformDeviceId] = &[
    PlatformDeviceId {
        name: b"clk-mt8188-vpp1\0".as_ptr() as *const core::ffi::c_char,
        driver_data: &VPP1_DESC as *const _ as usize,
    },
    PlatformDeviceId { name: core::ptr::null(), driver_data: 0 },
];

// MODULE_DEVICE_TABLE(platform, clk_mt8188_vpp1_id_table);

static mut CLK_MT8188_VPP1_DRV: PlatformDriver = PlatformDriver {
    probe: Some(mtk_clk_pdev_probe),
    remove: Some(mtk_clk_pdev_remove),
    driver: Driver { name: b"clk-mt8188-vpp1\0".as_ptr() as *const core::ffi::c_char },
    id_table: CLK_MT8188_VPP1_ID_TABLE.as_ptr(),
};

// module_platform_driver(clk_mt8188_vpp1_drv);
// MODULE_DESCRIPTION("MediaTek MT8188 Video Processing Pipe 1 clocks driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
