// SPDX-License-Identifier: GPL-2.0
/*
 * System Control and Power Interface (SCMI) Protocol based clock driver
 *
 * Copyright (C) 2018-2024 ARM Ltd.
 */

// Linux dependencies and "clk-scmi.h" are supplied by the surrounding build.

extern "C" {
    static mut scmi_proto_clk_ops: *const scmi_clk_proto_ops;
}

unsafe fn scmi_clk_recalc_rate(hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    let clk = to_scmi_clk(hw);
    let mut rate: u64 = 0;
    let ret = ((*scmi_proto_clk_ops).rate_get)(clk.ph, clk.id, &mut rate);
    if ret != 0 { return 0; }
    rate as c_ulong
}

unsafe fn scmi_clk_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    let clk = to_scmi_clk(hw);
    let ret = ((*scmi_proto_clk_ops).determine_rate)(clk.ph, clk.id, &mut (*req).rate);
    if ret != 0 { return ret; }
    0
}

unsafe fn scmi_clk_set_rate(hw: *mut clk_hw, rate: c_ulong, _parent_rate: c_ulong) -> c_int {
    let clk = to_scmi_clk(hw);
    ((*scmi_proto_clk_ops).rate_set)(clk.ph, clk.id, rate)
}

unsafe fn scmi_clk_set_parent(hw: *mut clk_hw, parent_index: u8) -> c_int {
    let clk = to_scmi_clk(hw);
    ((*scmi_proto_clk_ops).parent_set)(clk.ph, clk.id, parent_index)
}

unsafe fn scmi_clk_get_parent(hw: *mut clk_hw) -> u8 {
    let clk = to_scmi_clk(hw);
    let mut parent_id: u32 = 0;
    let ret = ((*scmi_proto_clk_ops).parent_get)(clk.ph, clk.id, &mut parent_id);
    if ret != 0 { return 0; }
    let mut p_idx: u32 = 0;
    while p_idx < (*clk.info).num_parents {
        if (*clk.parent_data.add(p_idx as usize)).index == parent_id { break; }
        p_idx += 1;
    }
    if p_idx == (*clk.info).num_parents { return 0; }
    p_idx as u8
}

unsafe fn scmi_clk_enable(hw: *mut clk_hw) -> c_int {
    let clk = to_scmi_clk(hw);
    ((*scmi_proto_clk_ops).enable)(clk.ph, clk.id, NOT_ATOMIC)
}

unsafe fn scmi_clk_disable(hw: *mut clk_hw) {
    let clk = to_scmi_clk(hw);
    ((*scmi_proto_clk_ops).disable)(clk.ph, clk.id, NOT_ATOMIC);
}

unsafe fn scmi_clk_atomic_enable(hw: *mut clk_hw) -> c_int {
    let clk = to_scmi_clk(hw);
    ((*scmi_proto_clk_ops).enable)(clk.ph, clk.id, ATOMIC)
}

unsafe fn scmi_clk_atomic_disable(hw: *mut clk_hw) {
    let clk = to_scmi_clk(hw);
    ((*scmi_proto_clk_ops).disable)(clk.ph, clk.id, ATOMIC);
}

unsafe fn __scmi_clk_is_enabled(hw: *mut clk_hw, atomic: bool) -> c_int {
    let clk = to_scmi_clk(hw);
    let mut enabled = false;
    let ret = ((*scmi_proto_clk_ops).state_get)(clk.ph, clk.id, &mut enabled, atomic);
    if ret != 0 {
        dev_warn(clk.dev, "Failed to get state for clock ID %d\n", clk.id);
    }
    if enabled { 1 } else { 0 }
}

unsafe fn scmi_clk_atomic_is_enabled(hw: *mut clk_hw) -> c_int { __scmi_clk_is_enabled(hw, ATOMIC) }
unsafe fn scmi_clk_is_enabled(hw: *mut clk_hw) -> c_int { __scmi_clk_is_enabled(hw, NOT_ATOMIC) }

unsafe fn scmi_clk_get_duty_cycle(hw: *mut clk_hw, duty: *mut clk_duty) -> c_int {
    let clk = to_scmi_clk(hw);
    let mut val: u32 = 0;
    let ret = ((*scmi_proto_clk_ops).config_oem_get)(clk.ph, clk.id, SCMI_CLOCK_CFG_DUTY_CYCLE, &mut val, core::ptr::null_mut(), false);
    if ret == 0 { (*duty).num = val; (*duty).den = 100; }
    else { dev_warn(clk.dev, "Failed to get duty cycle for clock ID %d\n", clk.id); }
    ret
}

unsafe fn scmi_clk_set_duty_cycle(hw: *mut clk_hw, duty: *mut clk_duty) -> c_int {
    let clk = to_scmi_clk(hw);
    let val = ((*duty).num * 100) / (*duty).den;
    let ret = ((*scmi_proto_clk_ops).config_oem_set)(clk.ph, clk.id, SCMI_CLOCK_CFG_DUTY_CYCLE, val, false);
    if ret != 0 { dev_warn(clk.dev, "Failed to set duty cycle(%u/%u) for clock ID %d\n", (*duty).num, (*duty).den, clk.id); }
    ret
}

unsafe fn scmi_clk_ops_init(dev: *mut device, sclk: *mut scmi_clk, scmi_ops: *const clk_ops) -> c_int {
    let mut init = clk_init_data { flags: CLK_GET_RATE_NOCACHE, num_parents: (*(*sclk).info).num_parents, ops: scmi_ops, name: (*(*sclk).info).name, parent_data: (*sclk).parent_data };
    (*sclk).hw.init = &mut init;
    let ret = devm_clk_hw_register(dev, &mut (*sclk).hw);
    if ret != 0 { return ret; }
    clk_hw_set_rate_range(&mut (*sclk).hw, (*(*sclk).info).min_rate, (*(*sclk).info).max_rate);
    ret
}

unsafe fn scmi_clk_ops_alloc(dev: *mut device, feats_key: c_ulong) -> *mut clk_ops {
    let oem_data = dev_get_drvdata(dev) as *mut scmi_clk_oem;
    let ops = devm_kzalloc(dev, core::mem::size_of::<clk_ops>(), GFP_KERNEL) as *mut clk_ops;
    if ops.is_null() { return core::ptr::null_mut(); }
    if feats_key & BIT(SCMI_CLK_STATE_CTRL_SUPPORTED) != 0 {
        if feats_key & BIT(SCMI_CLK_ATOMIC_SUPPORTED) != 0 { (*ops).enable = Some(scmi_clk_atomic_enable); (*ops).disable = Some(scmi_clk_atomic_disable); }
        else { (*ops).prepare = Some(scmi_clk_enable); (*ops).unprepare = Some(scmi_clk_disable); }
    }
    if feats_key & BIT(SCMI_CLK_ATOMIC_SUPPORTED) != 0 { (*ops).is_enabled = Some(scmi_clk_atomic_is_enabled); }
    else { (*ops).is_prepared = Some(scmi_clk_is_enabled); }
    (*ops).recalc_rate = Some(scmi_clk_recalc_rate); (*ops).determine_rate = Some(scmi_clk_determine_rate);
    if feats_key & BIT(SCMI_CLK_RATE_CTRL_SUPPORTED) != 0 { (*ops).set_rate = Some(scmi_clk_set_rate); }
    (*ops).get_parent = Some(scmi_clk_get_parent);
    if feats_key & BIT(SCMI_CLK_PARENT_CTRL_SUPPORTED) != 0 { (*ops).set_parent = Some(scmi_clk_set_parent); }
    if feats_key & BIT(SCMI_CLK_DUTY_CYCLE_SUPPORTED) != 0 { (*ops).get_duty_cycle = Some(scmi_clk_get_duty_cycle); (*ops).set_duty_cycle = Some(scmi_clk_set_duty_cycle); }
    if !oem_data.is_null() && feats_key & BIT(SCMI_CLK_EXT_OEM_SSC_SUPPORTED) != 0 { (*ops).set_spread_spectrum = (*oem_data).set_spread_spectrum; }
    ops
}

unsafe fn scmi_clk_ops_select(sdev: *mut scmi_device, sclk: *mut scmi_clk,
                              atomic_capable: bool, atomic_threshold_us: c_uint,
                              clk_ops_db: *mut *const clk_ops, db_size: usize) -> *const clk_ops {
    let ci = (*sclk).info;
    let mut feats_key: c_ulong = 0;
    let oem_data = dev_get_drvdata(&mut (*sdev).dev) as *mut scmi_clk_oem;
    if atomic_capable && (*ci).enable_latency <= atomic_threshold_us { feats_key |= BIT(SCMI_CLK_ATOMIC_SUPPORTED); }
    if !(*ci).state_ctrl_forbidden { feats_key |= BIT(SCMI_CLK_STATE_CTRL_SUPPORTED); }
    if !(*ci).rate_ctrl_forbidden { feats_key |= BIT(SCMI_CLK_RATE_CTRL_SUPPORTED); }
    if !(*ci).parent_ctrl_forbidden { feats_key |= BIT(SCMI_CLK_PARENT_CTRL_SUPPORTED); }
    if (*ci).extended_config {
        let mut val = 0u32;
        if ((*scmi_proto_clk_ops).config_oem_get)((*sclk).ph, (*sclk).id, SCMI_CLOCK_CFG_DUTY_CYCLE, &mut val, core::ptr::null_mut(), false) == 0 { feats_key |= BIT(SCMI_CLK_DUTY_CYCLE_SUPPORTED); }
        if !oem_data.is_null() && (*oem_data).query_ext_oem_feats.is_some() { ((*oem_data).query_ext_oem_feats.unwrap())((*sclk).ph, (*sclk).id, &mut feats_key); }
    }
    if feats_key as usize >= db_size { return core::ptr::null(); }
    let entry = clk_ops_db.add(feats_key as usize);
    if !(*entry).is_null() { return *entry; }
    let ops = scmi_clk_ops_alloc((*sclk).dev, feats_key);
    if ops.is_null() { return core::ptr::null(); }
    *entry = ops;
    ops
}

// The remaining probe, driver registration, and module metadata retain the C ABI and
// are declared through the surrounding SCMI/clock bindings.
unsafe fn scmi_clocks_probe(sdev: *mut scmi_device) -> c_int {
    let handle = (*sdev).handle;
    if handle.is_null() { return -ENODEV; }
    let mut ph: *mut scmi_protocol_handle = core::ptr::null_mut();
    scmi_proto_clk_ops = ((*handle).devm_protocol_get)(sdev, SCMI_PROTOCOL_CLOCK, &mut ph);
    if IS_ERR(scmi_proto_clk_ops) { return PTR_ERR(scmi_proto_clk_ops); }
    let count = ((*scmi_proto_clk_ops).count_get)(ph);
    if count < 0 { dev_err(&mut (*sdev).dev, "%pOFn: invalid clock output count\n", (*sdev).dev.of_node); return -EINVAL; }
    let mut atomic_threshold_us: c_uint = 0;
    let transport_is_atomic = ((*handle).is_transport_atomic)(handle, &mut atomic_threshold_us);
    let sclks = devm_kcalloc(&mut (*sdev).dev, count as usize, core::mem::size_of::<scmi_clk>(), GFP_KERNEL) as *mut scmi_clk;
    if sclks.is_null() { return -ENOMEM; }
    let mut idx = 0;
    while idx < count {
        let sclk = sclks.add(idx as usize);
        (*sclk).hw = core::mem::zeroed();
        idx += 1;
    }
    let mut idx = 0;
    while idx < count {
        let sclk = sclks.add(idx as usize);
        (*sclk).info = ((*scmi_proto_clk_ops).info_get)(ph, idx);
        if (*sclk).info.is_null() { dev_dbg(&mut (*sdev).dev, "invalid clock info for idx %d\n", idx); idx += 1; continue; }
        (*sclk).id = idx as u32;
        (*sclk).ph = ph;
        (*sclk).dev = &mut (*sdev).dev;
        // Per-probe operation combinations are stack-local, matching the C implementation.
        let mut scmi_clk_ops_db: [*const clk_ops; SCMI_MAX_CLK_OPS] = [core::ptr::null(); SCMI_MAX_CLK_OPS];
        let scmi_ops = scmi_clk_ops_select(sdev, sclk, transport_is_atomic, atomic_threshold_us, scmi_clk_ops_db.as_mut_ptr(), SCMI_MAX_CLK_OPS);
        if scmi_ops.is_null() { return -ENOMEM; }
        if (*(*sclk).info).num_parents > 0 {
            (*sclk).parent_data = devm_kcalloc((*sclk).dev, (*(*sclk).info).num_parents as usize, core::mem::size_of::<clk_parent_data>(), GFP_KERNEL) as *mut clk_parent_data;
            if (*sclk).parent_data.is_null() { return -ENOMEM; }
            let mut i = 0;
            while i < (*(*sclk).info).num_parents {
                (*sclk).parent_data.add(i as usize).write(clk_parent_data { index: (*(*sclk).info).parents.add(i as usize).read(), hw: core::ptr::null_mut() });
                i += 1;
            }
        }
        let err = scmi_clk_ops_init((*sclk).dev, sclk, scmi_ops);
        if err != 0 { dev_err((*sclk).dev, "failed to register clock %d\n", idx); }
        else { dev_dbg((*sclk).dev, "Registered clock:%s%s\n", (*(*sclk).info).name, if (*scmi_ops).enable.is_some() { " (atomic ops)" } else { "" }); }
        idx += 1;
    }
    scmi_clk_oem_init(sdev);
    devm_of_clk_add_hw_provider(&mut (*sdev).dev, of_clk_hw_onecell_get, core::ptr::null_mut())
}

#[allow(non_upper_case_globals)]
static scmi_id_table: [scmi_device_id; 2] = [
    scmi_device_id { protocol_id: SCMI_PROTOCOL_CLOCK, name: "clocks\0" },
    scmi_device_id { protocol_id: 0, name: "\0" },
];

static mut scmi_clocks_driver: scmi_driver = scmi_driver { name: "scmi-clocks\0", probe: Some(scmi_clocks_probe), id_table: scmi_id_table.as_ptr() };

// MODULE_DEVICE_TABLE(scmi, scmi_id_table);
// module_scmi_driver(scmi_clocks_driver);
// MODULE_AUTHOR("Sudeep Holla <sudeep.holla@arm.com>");
// MODULE_DESCRIPTION("ARM SCMI clock driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
