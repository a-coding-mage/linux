// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2025 NXP
 */

// Dependencies supplied by the surrounding kernel/Rust environment:
// linux/firmware/imx/sm.h, linux/module.h, linux/of.h,
// linux/platform_device.h, linux/scmi_protocol.h, and
// linux/scmi_imx_protocol.h.

static mut imx_cpu_ops: *const scmi_imx_cpu_proto_ops = core::ptr::null();
static mut ph: *mut scmi_protocol_handle = core::ptr::null_mut();

pub unsafe fn scmi_imx_cpu_reset_vector_set(
    cpuid: u32,
    vector: u64,
    start: bool,
    boot: bool,
    resume: bool,
) -> i32 {
    if ph.is_null() {
        return -EPROBE_DEFER;
    }

    ((*imx_cpu_ops).cpu_reset_vector_set)(ph, cpuid, vector, start, boot, resume)
}

// EXPORT_SYMBOL(scmi_imx_cpu_reset_vector_set);

pub unsafe fn scmi_imx_cpu_start(cpuid: u32, start: bool) -> i32 {
    if ph.is_null() {
        return -EPROBE_DEFER;
    }

    if start {
        return ((*imx_cpu_ops).cpu_start)(ph, cpuid, true);
    }

    ((*imx_cpu_ops).cpu_start)(ph, cpuid, false)
}

// EXPORT_SYMBOL(scmi_imx_cpu_start);

pub unsafe fn scmi_imx_cpu_started(cpuid: u32, started: *mut bool) -> i32 {
    if ph.is_null() {
        return -EPROBE_DEFER;
    }

    if started.is_null() {
        return -EINVAL;
    }

    ((*imx_cpu_ops).cpu_started)(ph, cpuid, started)
}

// EXPORT_SYMBOL(scmi_imx_cpu_started);

unsafe fn scmi_imx_cpu_probe(sdev: *mut scmi_device) -> i32 {
    let handle = (*sdev).handle;

    if handle.is_null() {
        return -ENODEV;
    }

    if !imx_cpu_ops.is_null() {
        dev_err(&(*sdev).dev, "sm cpu already initialized\n");
        return -EEXIST;
    }

    imx_cpu_ops = (*handle).devm_protocol_get(
        sdev,
        SCMI_PROTOCOL_IMX_CPU,
        &mut ph,
    );
    if IS_ERR(imx_cpu_ops) {
        return PTR_ERR(imx_cpu_ops);
    }

    0
}

static scmi_id_table: [scmi_device_id; 2] = [
    scmi_device_id {
        protocol_id: SCMI_PROTOCOL_IMX_CPU,
        name: "imx-cpu",
    },
    scmi_device_id {
        ..Default::default()
    },
];

// MODULE_DEVICE_TABLE(scmi, scmi_id_table);

static mut scmi_imx_cpu_driver: scmi_driver = scmi_driver {
    name: "scmi-imx-cpu",
    probe: Some(scmi_imx_cpu_probe),
    id_table: scmi_id_table.as_ptr(),
};

// module_scmi_driver(scmi_imx_cpu_driver);

// MODULE_AUTHOR("Peng Fan <peng.fan@nxp.com>");
// MODULE_DESCRIPTION("IMX SM CPU driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
