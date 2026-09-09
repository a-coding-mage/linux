// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright 2024 NXP
 */

// External Linux/SCMI types, constants, functions, and generated show-attribute
// items are supplied by the surrounding kernel translation unit.

static mut IMX_MISC_CTRL_OPS: *const scmi_imx_misc_proto_ops = core::ptr::null();
static mut PH: *mut scmi_protocol_handle = core::ptr::null_mut();
static mut SCMI_IMX_MISC_CTRL_NB: notifier_block = notifier_block { notifier_call: None };

static RST_IMX95: [&'static str; 32] = [
    "cm33_lockup", "cm33_swreq", "cm7_lockup", "cm7_swreq", "fccu",
    "jtag_sw", "ele", "tempsense", "wdog1", "wdog2", "wdog3", "wdog4",
    "wdog5", "jtag", "cm33_exc", "bbm", "sw", "sm_err", "fusa_sreco",
    "pmic", "unused", "unused", "unused", "unused", "unused", "unused",
    "unused", "unused", "unused", "unused", "unused", "por",
];

static RST_IMX94: [&'static str; 32] = [
    "cm33_lockup", "cm33_swreq", "cm70_lockup", "cm70_swreq", "fccu",
    "jtag_sw", "ele", "tempsense", "wdog1", "wdog2", "wdog3", "wdog4",
    "wdog5", "jtag", "wdog6", "wdog7", "wdog8", "wo_netc", "cm33s_lockup",
    "cm33s_swreq", "cm71_lockup", "cm71_swreq", "cm33_exc", "bbm", "sw",
    "sm_err", "fusa_sreco", "pmic", "unused", "unused", "unused", "por",
];

static ALLOWLIST: [of_device_id; 4] = [
    of_device_id { compatible: "fsl,imx952", data: RST_IMX95.as_ptr() as *const core::ffi::c_void },
    of_device_id { compatible: "fsl,imx95", data: RST_IMX95.as_ptr() as *const core::ffi::c_void },
    of_device_id { compatible: "fsl,imx94", data: RST_IMX94.as_ptr() as *const core::ffi::c_void },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() }, // Sentinel
];

#[no_mangle]
pub unsafe extern "C" fn scmi_imx_misc_ctrl_set(id: u32, val: u32) -> i32 {
    if PH.is_null() { return -EPROBE_DEFER; }
    (*IMX_MISC_CTRL_OPS).misc_ctrl_set(PH, id, 1, &val)
}

#[no_mangle]
pub unsafe extern "C" fn scmi_imx_misc_ctrl_get(id: u32, num: *mut u32, val: *mut u32) -> i32 {
    if PH.is_null() { return -EPROBE_DEFER; }
    (*IMX_MISC_CTRL_OPS).misc_ctrl_get(PH, id, num, val)
}

unsafe extern "C" fn scmi_imx_misc_ctrl_notifier(
    _nb: *mut notifier_block, _event: usize, _data: *mut core::ffi::c_void,
) -> i32 {
    /* notifier_chain_register requires a valid notifier_block and valid notifier_call.
     * SCMI_EVENT_IMX_MISC_CONTROL enables control events; this is a dummy hook. */
    0
}

unsafe extern "C" fn syslog_show(file: *mut seq_file, _priv: *mut core::ffi::c_void) -> i32 {
    /* 4KB is large enough for syslog. */
    let syslog = kmalloc(SZ_4K, GFP_KERNEL);
    /* syslog API uses number of words, not number of bytes. */
    let mut size: u16 = (SZ_4K / 4) as u16;
    if syslog.is_null() { return -ENOMEM; }
    if PH.is_null() { kfree(syslog); return -ENODEV; }
    let ret = (*IMX_MISC_CTRL_OPS).misc_syslog(PH, &mut size, syslog);
    if ret != 0 { kfree(syslog); return ret; }
    seq_hex_dump(file, " ", DUMP_PREFIX_NONE, 16, core::mem::size_of::<u32>(), syslog,
                 (size as usize) * 4, false);
    seq_putc(file, b'\n' as i32);
    kfree(syslog);
    0
}

unsafe extern "C" fn scmi_imx_misc_put(p: *mut core::ffi::c_void) {
    debugfs_remove(p as *mut dentry);
}

unsafe extern "C" fn scmi_imx_misc_get_reason(sdev: *mut scmi_device) -> i32 {
    let mut boot = scmi_imx_misc_reset_reason::default();
    let mut shutdown = scmi_imx_misc_reset_reason::default();
    let mut system = true;
    if of_machine_device_match(ALLOWLIST.as_ptr()) == 0 { return 0; }
    let rst = of_machine_get_match_data(ALLOWLIST.as_ptr()) as *const *const u8;
    let mut ret = (*IMX_MISC_CTRL_OPS).misc_reset_reason(PH, system, &mut boot, &mut shutdown, core::ptr::null_mut());
    if ret == 0 {
        if boot.valid { dev_info((*sdev).dev, "%s Boot reason: %s, origin: %d, errid: %d\n", if system { "SYS" } else { "LM" }, *rst.add(boot.reason as usize), if boot.orig_valid { boot.origin as i32 } else { -1 }, if boot.err_valid { boot.errid as i32 } else { -1 }); }
        if shutdown.valid { dev_info((*sdev).dev, "%s shutdown reason: %s, origin: %d, errid: %d\n", if system { "SYS" } else { "LM" }, *rst.add(shutdown.reason as usize), if shutdown.orig_valid { shutdown.origin as i32 } else { -1 }, if shutdown.err_valid { shutdown.errid as i32 } else { -1 }); }
    } else { dev_err((*sdev).dev, "Failed to get system reset reason: %d\n", ret); }
    system = false;
    ret = (*IMX_MISC_CTRL_OPS).misc_reset_reason(PH, system, &mut boot, &mut shutdown, core::ptr::null_mut());
    if ret == 0 {
        if boot.valid { dev_info((*sdev).dev, "%s Boot reason: %s, origin: %d, errid: %d\n", if system { "SYS" } else { "LM" }, *rst.add(boot.reason as usize), if boot.orig_valid { boot.origin as i32 } else { -1 }, if boot.err_valid { boot.errid as i32 } else { -1 }); }
        if shutdown.valid { dev_info((*sdev).dev, "%s shutdown reason: %s, origin: %d, errid: %d\n", if system { "SYS" } else { "LM" }, *rst.add(shutdown.reason as usize), if shutdown.orig_valid { shutdown.origin as i32 } else { -1 }, if shutdown.err_valid { shutdown.errid as i32 } else { -1 }); }
    } else { dev_err((*sdev).dev, "Failed to get lm reset reason: %d\n", ret); }
    0
}

unsafe extern "C" fn scmi_imx_misc_ctrl_probe(sdev: *mut scmi_device) -> i32 {
    let handle = (*sdev).handle;
    let np = (*sdev).dev.of_node;
    let mut scmi_imx_dentry: *mut dentry;
    let (mut src_id, mut flags): (u32, u32) = (0, 0);
    let (mut ret, mut i, mut num): (i32, i32, i32) = (0, 0, 0);
    if handle.is_null() { return -ENODEV; }
    if !IMX_MISC_CTRL_OPS.is_null() { dev_err((*sdev).dev, "misc ctrl already initialized\n"); return -EEXIST; }
    IMX_MISC_CTRL_OPS = (*handle).devm_protocol_get(sdev, SCMI_PROTOCOL_IMX_MISC, &mut PH);
    if IS_ERR(IMX_MISC_CTRL_OPS as *mut core::ffi::c_void) { return PTR_ERR(IMX_MISC_CTRL_OPS as *mut core::ffi::c_void); }
    num = of_property_count_u32_elems(np, "nxp,ctrl-ids");
    if num % 2 != 0 { dev_err((*sdev).dev, "Invalid wakeup-sources\n"); return -EINVAL; }
    SCMI_IMX_MISC_CTRL_NB.notifier_call = Some(scmi_imx_misc_ctrl_notifier);
    while i < num {
        ret = of_property_read_u32_index(np, "nxp,ctrl-ids", i, &mut src_id);
        if ret != 0 { dev_err((*sdev).dev, "Failed to read ctrl-id: %i\n", i); i += 2; continue; }
        ret = of_property_read_u32_index(np, "nxp,ctrl-ids", i + 1, &mut flags);
        if ret != 0 { dev_err((*sdev).dev, "Failed to read ctrl-id value: %d\n", i + 1); i += 2; continue; }
        ret = (*handle).notify_ops.devm_event_notifier_register(sdev, SCMI_PROTOCOL_IMX_MISC, SCMI_EVENT_IMX_MISC_CONTROL, &src_id, &mut SCMI_IMX_MISC_CTRL_NB);
        if ret != 0 { dev_err((*sdev).dev, "Failed to register scmi misc event: %d\n", src_id); }
        else { ret = (*IMX_MISC_CTRL_OPS).misc_ctrl_req_notify(PH, src_id, SCMI_EVENT_IMX_MISC_CONTROL, flags); if ret != 0 { dev_err((*sdev).dev, "Failed to req notify: %d\n", src_id); } }
        i += 2;
    }
    scmi_imx_dentry = debugfs_create_dir("scmi_imx", core::ptr::null_mut());
    debugfs_create_file("syslog", 0o444, scmi_imx_dentry, &mut (*sdev).dev as *mut _, &syslog_fops);
    scmi_imx_misc_get_reason(sdev);
    devm_add_action_or_reset(&mut (*sdev).dev, scmi_imx_misc_put, scmi_imx_dentry as *mut _)
}

static SCMI_ID_TABLE: [scmi_device_id; 2] = [
    scmi_device_id { protocol_id: SCMI_PROTOCOL_IMX_MISC, name: "imx-misc-ctrl" },
    scmi_device_id { protocol_id: 0, name: core::ptr::null() },
];

static SCMI_IMX_MISC_CTRL_DRIVER: scmi_driver = scmi_driver {
    name: "scmi-imx-misc-ctrl",
    probe: Some(scmi_imx_misc_ctrl_probe),
    id_table: SCMI_ID_TABLE.as_ptr(),
};

// Equivalent to module_scmi_driver(scmi_imx_misc_ctrl_driver).
// MODULE_AUTHOR("Peng Fan <peng.fan@nxp.com>");
// MODULE_DESCRIPTION("IMX SM MISC driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
