// SPDX-License-Identifier: GPL-2.0-only
/*
 * Legacy platform_data quirks
 *
 * Copyright (C) 2016 BayLibre, Inc
 */
// Translated dependencies: <linux/kernel.h>, <linux/of.h>,
// <media/i2c/tvp514x.h>, <media/i2c/adv7343.h>, "common.h", "da8xx.h".

#[repr(C)]
struct pdata_init {
    compatible: *const core::ffi::c_char,
    fn_: Option<unsafe extern "C" fn()>,
}

const TVP5147_CH0: &[u8] = b"tvp514x-0\0";
const TVP5147_CH1: &[u8] = b"tvp514x-1\0";
const TVP514X_STD_ALL: u64 = V4L2_STD_NTSC | V4L2_STD_PAL;

/* VPIF capture configuration */
static mut tvp5146_pdata: tvp514x_platform_data = tvp514x_platform_data {
    clk_polarity: 0,
    hs_polarity: 1,
    vs_polarity: 1,
};

static mut da850_ch0_inputs: [vpif_input; 1] = [vpif_input {
    input: v4l2_input {
        index: 0,
        name: *b"Composite\0",
        type_: V4L2_INPUT_TYPE_CAMERA,
        capabilities: V4L2_IN_CAP_STD,
        std: TVP514X_STD_ALL,
    },
    input_route: INPUT_CVBS_VI2B,
    output_route: OUTPUT_10BIT_422_EMBEDDED_SYNC,
    subdev_name: TVP5147_CH0.as_ptr() as *const core::ffi::c_char,
}];

static mut da850_ch1_inputs: [vpif_input; 1] = [vpif_input {
    input: v4l2_input {
        index: 0,
        name: *b"S-Video\0",
        type_: V4L2_INPUT_TYPE_CAMERA,
        capabilities: V4L2_IN_CAP_STD,
        std: TVP514X_STD_ALL,
    },
    input_route: INPUT_SVIDEO_VI2C_VI1C,
    output_route: OUTPUT_10BIT_422_EMBEDDED_SYNC,
    subdev_name: TVP5147_CH1.as_ptr() as *const core::ffi::c_char,
}];

static mut da850_vpif_capture_sdev_info: [vpif_subdev_info; 2] = [
    vpif_subdev_info {
        name: TVP5147_CH0.as_ptr() as *const core::ffi::c_char,
        board_info: i2c_board_info {
            type_: *b"tvp5146\0",
            addr: 0x5d,
            platform_data: unsafe { &raw mut tvp5146_pdata as *mut _ },
        },
    },
    vpif_subdev_info {
        name: TVP5147_CH1.as_ptr() as *const core::ffi::c_char,
        board_info: i2c_board_info {
            type_: *b"tvp5146\0",
            addr: 0x5c,
            platform_data: unsafe { &raw mut tvp5146_pdata as *mut _ },
        },
    },
];

static mut da850_vpif_capture_config: vpif_capture_config = vpif_capture_config {
    subdev_info: unsafe { da850_vpif_capture_sdev_info.as_mut_ptr() },
    subdev_count: 2,
    chan_config: [
        vpif_chan_config {
            inputs: unsafe { da850_ch0_inputs.as_mut_ptr() }, input_count: 1,
            vpif_if: vpif_interface { if_type: VPIF_IF_BT656, hd_pol: 1, vd_pol: 1, fid_pol: 0 },
        },
        vpif_chan_config {
            inputs: unsafe { da850_ch1_inputs.as_mut_ptr() }, input_count: 1,
            vpif_if: vpif_interface { if_type: VPIF_IF_BT656, hd_pol: 1, vd_pol: 1, fid_pol: 0 },
        },
    ],
    card_name: b"DA850/OMAP-L138 Video Capture\0".as_ptr() as *const _,
};

unsafe extern "C" fn da850_vpif_legacy_register_capture() {
    let ret = da850_register_vpif_capture(&raw mut da850_vpif_capture_config);
    if ret != 0 { pr_warn("%s: VPIF capture setup failed: %d\n", "da850_vpif_legacy_register_capture", ret); }
}

unsafe extern "C" fn da850_vpif_capture_legacy_init_lcdk() {
    da850_vpif_capture_config.subdev_count = 1;
    da850_vpif_legacy_register_capture();
}

unsafe extern "C" fn da850_vpif_capture_legacy_init_evm() { da850_vpif_legacy_register_capture(); }

static mut adv7343_pdata: adv7343_platform_data = adv7343_platform_data {
    mode_config: adv7343_mode_config { dac: [1, 1, 1] },
    sd_config: adv7343_sd_config { sd_dac_out: [1] },
};

static mut da850_vpif_subdev: [vpif_subdev_info; 1] = [vpif_subdev_info {
    name: b"adv7343\0".as_ptr() as *const _,
    board_info: i2c_board_info { type_: *b"adv7343\0", addr: 0x2a, platform_data: unsafe { &raw mut adv7343_pdata as *mut _ } },
}];

static da850_ch0_outputs: [vpif_output; 2] = [
    vpif_output { output: v4l2_output { index: 0, name: *b"Composite\0", type_: V4L2_OUTPUT_TYPE_ANALOG, capabilities: V4L2_OUT_CAP_STD, std: V4L2_STD_ALL }, subdev_name: b"adv7343\0".as_ptr() as *const _, output_route: ADV7343_COMPOSITE_ID },
    vpif_output { output: v4l2_output { index: 1, name: *b"S-Video\0", type_: V4L2_OUTPUT_TYPE_ANALOG, capabilities: V4L2_OUT_CAP_STD, std: V4L2_STD_ALL }, subdev_name: b"adv7343\0".as_ptr() as *const _, output_route: ADV7343_SVIDEO_ID },
];

static mut da850_vpif_display_config: vpif_display_config = vpif_display_config {
    subdevinfo: unsafe { da850_vpif_subdev.as_mut_ptr() }, subdev_count: 1,
    chan_config: [vpif_display_chan_config { outputs: da850_ch0_outputs.as_ptr() as *mut _, output_count: 2 }],
    card_name: b"DA850/OMAP-L138 Video Display\0".as_ptr() as *const _,
};

unsafe extern "C" fn da850_vpif_display_legacy_init_evm() {
    let ret = da850_register_vpif_display(&raw mut da850_vpif_display_config);
    if ret != 0 { pr_warn!("%s: VPIF display setup failed: %d\n", "da850_vpif_display_legacy_init_evm", ret); }
}

unsafe fn pdata_quirks_check(mut quirks: *mut pdata_init) {
    while !(*quirks).compatible.is_null() {
        if of_machine_is_compatible((*quirks).compatible) != 0 {
            if let Some(f) = (*quirks).fn_ { f(); }
        }
        quirks = quirks.add(1);
    }
}

static mut pdata_quirks: [pdata_init; 4] = [
    pdata_init { compatible: b"ti,da850-lcdk\0".as_ptr() as *const _, fn_: Some(da850_vpif_capture_legacy_init_lcdk) },
    pdata_init { compatible: b"ti,da850-evm\0".as_ptr() as *const _, fn_: Some(da850_vpif_display_legacy_init_evm) },
    pdata_init { compatible: b"ti,da850-evm\0".as_ptr() as *const _, fn_: Some(da850_vpif_capture_legacy_init_evm) },
    pdata_init { compatible: core::ptr::null(), fn_: None },
];

unsafe extern "C" fn pdata_quirks_init() { pdata_quirks_check(pdata_quirks.as_mut_ptr()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
