// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2025 NXP
 */

// Dependencies supplied by the corresponding Linux/SCMI headers are intentionally
// left as external Rust types and operations.

static mut IMX_LMM_OPS: *const scmi_imx_lmm_proto_ops = core::ptr::null();
static mut PH: *mut scmi_protocol_handle = core::ptr::null_mut();

pub unsafe fn scmi_imx_lmm_info(
    lmid: u32,
    info: *mut scmi_imx_lmm_info,
) -> i32 {
    if PH.is_null() {
        return -EPROBE_DEFER;
    }

    if info.is_null() {
        return -EINVAL;
    }

    ((*IMX_LMM_OPS).lmm_info)(PH, lmid, info)
}
// EXPORT_SYMBOL(scmi_imx_lmm_info);

pub unsafe fn scmi_imx_lmm_reset_vector_set(
    lmid: u32,
    cpuid: u32,
    flags: u32,
    vector: u64,
) -> i32 {
    if PH.is_null() {
        return -EPROBE_DEFER;
    }

    ((*IMX_LMM_OPS).lmm_reset_vector_set)(PH, lmid, cpuid, flags, vector)
}
// EXPORT_SYMBOL(scmi_imx_lmm_reset_vector_set);

pub unsafe fn scmi_imx_lmm_operation(
    lmid: u32,
    op: scmi_imx_lmm_op,
    flags: u32,
) -> i32 {
    if PH.is_null() {
        return -EPROBE_DEFER;
    }

    match op {
        SCMI_IMX_LMM_BOOT => ((*IMX_LMM_OPS).lmm_power_boot)(PH, lmid, true),
        SCMI_IMX_LMM_POWER_ON => ((*IMX_LMM_OPS).lmm_power_boot)(PH, lmid, false),
        SCMI_IMX_LMM_SHUTDOWN => ((*IMX_LMM_OPS).lmm_shutdown)(PH, lmid, flags),
        _ => -EINVAL,
    }
}
// EXPORT_SYMBOL(scmi_imx_lmm_operation);

unsafe fn scmi_imx_lmm_probe(sdev: *mut scmi_device) -> i32 {
    let handle = (*sdev).handle;

    if handle.is_null() {
        return -ENODEV;
    }

    if !IMX_LMM_OPS.is_null() {
        dev_err(&(*sdev).dev, "lmm already initialized\n");
        return -EEXIST;
    }

    IMX_LMM_OPS = ((*handle).devm_protocol_get)(
        sdev,
        SCMI_PROTOCOL_IMX_LMM,
        &mut PH,
    );
    if is_err(IMX_LMM_OPS) {
        return ptr_err(IMX_LMM_OPS);
    }

    0
}

static SCMI_ID_TABLE: &[scmi_device_id] = &[
    scmi_device_id {
        protocol_id: SCMI_PROTOCOL_IMX_LMM,
        name: "imx-lmm",
    },
    scmi_device_id::default(),
];
// MODULE_DEVICE_TABLE(scmi, scmi_id_table);

static mut SCMI_IMX_LMM_DRIVER: scmi_driver = scmi_driver {
    name: "scmi-imx-lmm",
    probe: Some(scmi_imx_lmm_probe),
    id_table: SCMI_ID_TABLE,
};
// module_scmi_driver(scmi_imx_lmm_driver);

// MODULE_AUTHOR("Peng Fan <peng.fan@nxp.com>");
// MODULE_DESCRIPTION("IMX SM LMM driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
