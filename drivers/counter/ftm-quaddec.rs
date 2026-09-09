// SPDX-License-Identifier: GPL-2.0
/*
 * Flex Timer Module Quadrature decoder
 *
 * This module implements a driver for decoding the FTM quadrature
 * of ex. a LS1021A
 */

// External Linux kernel, FTM register, counter, and module symbols are
// supplied by the surrounding kernel translation.

#[repr(C)]
struct FtmQuaddec {
    pdev: *mut platform_device,
    ftm_base: *mut core::ffi::c_void,
    big_endian: bool,
    ftm_quaddec_mutex: mutex,
}

unsafe fn ftm_read(ftm: *mut FtmQuaddec, offset: u32, data: *mut u32) {
    unsafe {
        if (*ftm).big_endian {
            *data = ioread32be((*ftm).ftm_base.add(offset as usize));
        } else {
            *data = ioread32((*ftm).ftm_base.add(offset as usize));
        }
    }
}

unsafe fn ftm_write(ftm: *mut FtmQuaddec, offset: u32, data: u32) {
    unsafe {
        if (*ftm).big_endian {
            iowrite32be(data, (*ftm).ftm_base.add(offset as usize));
        } else {
            iowrite32(data, (*ftm).ftm_base.add(offset as usize));
        }
    }
}

unsafe fn ftm_field_update(ftm: *mut FtmQuaddec, offset: u32, mask: u32, val: u32) {
    let mut flags: u32 = 0;
    unsafe {
        ftm_read(ftm, offset, &mut flags);
        flags &= !mask;
        flags |= field_prep(mask, val);
        ftm_write(ftm, offset, flags);
    }
}

/* Hold mutex before modifying write protection state */
unsafe fn ftm_clear_write_protection(ftm: *mut FtmQuaddec) {
    let mut flag: u32 = 0;

    /* First see if it is enabled */
    unsafe {
        ftm_read(ftm, FTM_FMS, &mut flag);
        if flag & FTM_FMS_WPEN != 0 {
            ftm_field_update(ftm, FTM_MODE, FTM_MODE_WPDIS, 1);
        }
    }
}

unsafe fn ftm_set_write_protection(ftm: *mut FtmQuaddec) {
    unsafe { ftm_field_update(ftm, FTM_FMS, FTM_FMS_WPEN, 1) }
}

unsafe fn ftm_reset_counter(ftm: *mut FtmQuaddec) {
    /* Reset hardware counter to CNTIN */
    unsafe { ftm_write(ftm, FTM_CNT, 0x0) }
}

unsafe fn ftm_quaddec_init(ftm: *mut FtmQuaddec) {
    unsafe {
        ftm_clear_write_protection(ftm);

        /*
         * Do not write in the region from the CNTIN register through the
         * PWMLOAD register when FTMEN = 0.
         * Also reset other fields to zero
         */
        ftm_write(ftm, FTM_MODE, FTM_MODE_FTMEN);
        ftm_write(ftm, FTM_CNTIN, 0x0000);
        ftm_write(ftm, FTM_MOD, 0xffff);
        ftm_write(ftm, FTM_CNT, 0x0);
        /* Set prescaler, reset other fields to zero */
        ftm_write(ftm, FTM_SC, FTM_SC_PS_1);

        /* Select quad mode, reset other fields to zero */
        ftm_write(ftm, FTM_QDCTRL, FTM_QDCTRL_QUADEN);

        /* Unused features and reset to default section */
        ftm_write(ftm, FTM_POL, 0x0);
        ftm_write(ftm, FTM_FLTCTRL, 0x0);
        ftm_write(ftm, FTM_SYNCONF, 0x0);
        ftm_write(ftm, FTM_SYNC, 0xffff);

        /* Lock the FTM */
        ftm_set_write_protection(ftm);
    }
}

unsafe extern "C" fn ftm_quaddec_disable(ftm: *mut core::ffi::c_void) {
    let ftm_qua = ftm as *mut FtmQuaddec;

    unsafe {
        ftm_clear_write_protection(ftm_qua);
        ftm_write(ftm_qua, FTM_MODE, 0);
        ftm_write(ftm_qua, FTM_QDCTRL, 0);
        /*
         * This is enough to disable the counter. No clock has been
         * selected by writing to FTM_SC in init()
         */
        ftm_set_write_protection(ftm_qua);
    }
}

unsafe extern "C" fn ftm_quaddec_get_prescaler(
    counter: *mut counter_device,
    _count: *mut counter_count,
    cnt_mode: *mut u32,
) -> i32 {
    let ftm = counter_priv(counter) as *mut FtmQuaddec;
    let mut scflags: u32 = 0;
    unsafe {
        ftm_read(ftm, FTM_SC, &mut scflags);
        *cnt_mode = field_get(FTM_SC_PS_MASK, scflags);
    }
    0
}

unsafe extern "C" fn ftm_quaddec_set_prescaler(
    counter: *mut counter_device,
    _count: *mut counter_count,
    cnt_mode: u32,
) -> i32 {
    let ftm = counter_priv(counter) as *mut FtmQuaddec;
    unsafe {
        mutex_lock(&mut (*ftm).ftm_quaddec_mutex);
        ftm_clear_write_protection(ftm);
        ftm_field_update(ftm, FTM_SC, FTM_SC_PS_MASK, cnt_mode);
        ftm_set_write_protection(ftm);
        /* Also resets the counter as it is undefined anyway now */
        ftm_reset_counter(ftm);
        mutex_unlock(&mut (*ftm).ftm_quaddec_mutex);
    }
    0
}

static FTM_QUADDEC_PRESCALER: [&[u8]; 8] = [b"1", b"2", b"4", b"8", b"16", b"32", b"64", b"128"];
static FTM_QUADDEC_PRESCALER_ENUM: counter_enum = counter_enum { items: &FTM_QUADDEC_PRESCALER };

static FTM_QUADDEC_SYNAPSE_ACTIONS: [counter_synapse_action; 1] =
    [COUNTER_SYNAPSE_ACTION_BOTH_EDGES];
static FTM_QUADDEC_COUNT_FUNCTIONS: [counter_function; 1] =
    [COUNTER_FUNCTION_QUADRATURE_X4];

unsafe extern "C" fn ftm_quaddec_count_read(
    counter: *mut counter_device, _count: *mut counter_count, val: *mut u64,
) -> i32 {
    let ftm = counter_priv(counter) as *mut FtmQuaddec;
    let mut cntval: u32 = 0;
    unsafe { ftm_read(ftm, FTM_CNT, &mut cntval); *val = cntval as u64; }
    0
}

unsafe extern "C" fn ftm_quaddec_count_write(
    counter: *mut counter_device, _count: *mut counter_count, val: u64,
) -> i32 {
    let ftm = counter_priv(counter) as *mut FtmQuaddec;
    if val != 0 {
        unsafe { dev_warn((*ftm).pdev, b"Can only accept '0' as new counter value\n\0".as_ptr()) };
        return -EINVAL;
    }
    unsafe { ftm_reset_counter(ftm) }
    0
}

unsafe extern "C" fn ftm_quaddec_count_function_read(
    _counter: *mut counter_device, _count: *mut counter_count, function: *mut counter_function,
) -> i32 { unsafe { *function = COUNTER_FUNCTION_QUADRATURE_X4 }; 0 }

unsafe extern "C" fn ftm_quaddec_action_read(
    _counter: *mut counter_device, _count: *mut counter_count, _synapse: *mut counter_synapse,
    action: *mut counter_synapse_action,
) -> i32 { unsafe { *action = COUNTER_SYNAPSE_ACTION_BOTH_EDGES }; 0 }

static FTM_QUADDEC_CNT_OPS: counter_ops = counter_ops {
    count_read: Some(ftm_quaddec_count_read),
    count_write: Some(ftm_quaddec_count_write),
    function_read: Some(ftm_quaddec_count_function_read),
    action_read: Some(ftm_quaddec_action_read),
};

static mut FTM_QUADDEC_SIGNALS: [counter_signal; 2] = [
    counter_signal { id: 0, name: b"Channel 1 Phase A\0".as_ptr() },
    counter_signal { id: 1, name: b"Channel 1 Phase B\0".as_ptr() },
];

static mut FTM_QUADDEC_COUNT_SYNAPSES: [counter_synapse; 2] = [
    counter_synapse { actions_list: FTM_QUADDEC_SYNAPSE_ACTIONS.as_ptr(), num_actions: 1, signal: unsafe { &FTM_QUADDEC_SIGNALS[0] } },
    counter_synapse { actions_list: FTM_QUADDEC_SYNAPSE_ACTIONS.as_ptr(), num_actions: 1, signal: unsafe { &FTM_QUADDEC_SIGNALS[1] } },
];

static mut FTM_QUADDEC_COUNT_EXT: [counter_comp; 1] = [
    counter_comp { name: b"prescaler\0".as_ptr(), get: Some(ftm_quaddec_get_prescaler), set: Some(ftm_quaddec_set_prescaler), enum_data: &FTM_QUADDEC_PRESCALER_ENUM },
];

static mut FTM_QUADDEC_COUNTS: [counter_count; 1] = [counter_count {
    id: 0, name: b"Channel 1 Count\0".as_ptr(), functions_list: FTM_QUADDEC_COUNT_FUNCTIONS.as_ptr(), num_functions: 1,
    synapses: FTM_QUADDEC_COUNT_SYNAPSES.as_ptr(), num_synapses: 2, ext: FTM_QUADDEC_COUNT_EXT.as_ptr(), num_ext: 1,
}];

unsafe extern "C" fn ftm_quaddec_probe(pdev: *mut platform_device) -> i32 {
    let mut counter = devm_counter_alloc(pdev, core::mem::size_of::<FtmQuaddec>());
    if counter.is_null() { return -ENOMEM; }
    let ftm = counter_priv(counter) as *mut FtmQuaddec;
    let node = unsafe { (*pdev).dev.of_node };
    let io = unsafe { platform_get_resource(pdev, IORESOURCE_MEM, 0) };
    if io.is_null() { unsafe { dev_err(pdev, b"Failed to get memory region\n\0".as_ptr()) }; return -ENODEV; }
    unsafe {
        (*ftm).pdev = pdev;
        (*ftm).big_endian = of_property_read_bool(node, b"big-endian\0".as_ptr());
        (*ftm).ftm_base = devm_ioremap(pdev, (*io).start, resource_size(io));
        if (*ftm).ftm_base.is_null() { dev_err(pdev, b"Failed to map memory region\n\0".as_ptr()); return -EINVAL; }
        (*counter).name = dev_name(pdev);
        (*counter).parent = pdev;
        (*counter).ops = &FTM_QUADDEC_CNT_OPS;
        (*counter).counts = FTM_QUADDEC_COUNTS.as_mut_ptr();
        (*counter).num_counts = 1;
        (*counter).signals = FTM_QUADDEC_SIGNALS.as_mut_ptr();
        (*counter).num_signals = 2;
        let ret = devm_mutex_init(pdev, &mut (*ftm).ftm_quaddec_mutex);
        if ret != 0 { return ret; }
        ftm_quaddec_init(ftm);
        let ret = devm_add_action_or_reset(pdev, Some(ftm_quaddec_disable), ftm as *mut core::ffi::c_void);
        if ret != 0 { return ret; }
        let ret = devm_counter_add(pdev, counter);
        if ret != 0 { return dev_err_probe(pdev, ret, b"Failed to add counter\n\0".as_ptr()); }
    }
    0
}

static FTM_QUADDEC_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: b"fsl,ftm-quaddec\0".as_ptr() }, of_device_id { compatible: core::ptr::null() },
];

static mut FTM_QUADDEC_DRIVER: platform_driver = platform_driver {
    driver: device_driver { name: b"ftm-quaddec\0".as_ptr(), of_match_table: FTM_QUADDEC_MATCH.as_ptr() },
    probe: Some(ftm_quaddec_probe),
};

// module_platform_driver!(FTM_QUADDEC_DRIVER);
// MODULE_DEVICE_TABLE!(of, FTM_QUADDEC_MATCH);
// MODULE_DESCRIPTION!("Flex Timer Module Quadrature decoder");
// MODULE_LICENSE!("GPL");
// MODULE_AUTHOR!("Kjeld Flarup <kfa@deif.com>");
// MODULE_AUTHOR!("Patrick Havelange <patrick.havelange@essensium.com>");
// MODULE_IMPORT_NS!("COUNTER");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
