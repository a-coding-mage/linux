// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */

// Linux kernel dependencies are supplied by the surrounding translation unit.

#[repr(C)]
struct cc_fips_handle {
    tasklet: tasklet_struct,
    nb: notifier_block,
    drvdata: *mut cc_drvdata,
}

/* The function called once at driver entry point to check
 * whether TEE FIPS error occurred.
 */
unsafe fn cc_get_tee_fips_status(drvdata: *mut cc_drvdata) -> bool {
    let reg: u32;

    reg = cc_ioread(drvdata, CC_REG(GPR_HOST));
    /* Did the TEE report status? */
    if reg & CC_FIPS_SYNC_TEE_STATUS != 0 {
        /* Yes. Is it OK? */
        return reg & CC_FIPS_SYNC_MODULE_OK != 0;
    }

    /* No. It's either not in use or will be reported later */
    true
}

/*
 * This function should push the FIPS REE library status towards the TEE library
 * by writing the error state to HOST_GPR0 register.
 */
pub unsafe fn cc_set_ree_fips_status(drvdata: *mut cc_drvdata, status: bool) {
    let mut val: i32 = CC_FIPS_SYNC_REE_STATUS;

    if (*drvdata).hw_rev < CC_HW_REV_712 {
        return;
    }

    val |= if status { CC_FIPS_SYNC_MODULE_OK } else { CC_FIPS_SYNC_MODULE_ERROR };

    cc_iowrite(drvdata, CC_REG(HOST_GPR0), val);
}

/* Push REE side FIPS test failure to TEE side */
unsafe fn cc_ree_fips_failure(nb: *mut notifier_block, _unused1: c_ulong,
                              _unused2: *mut c_void) -> i32 {
    let fips_h = container_of!(nb, cc_fips_handle, nb);
    let drvdata = (*fips_h).drvdata;
    let dev = drvdata_to_dev(drvdata);

    cc_set_ree_fips_status(drvdata, false);
    dev_info(dev, "Notifying TEE of FIPS test failure...\n");

    NOTIFY_OK
}

pub unsafe fn cc_fips_fini(drvdata: *mut cc_drvdata) {
    let fips_h = (*drvdata).fips_handle;

    if (*drvdata).hw_rev < CC_HW_REV_712 || fips_h.is_null() {
        return;
    }

    atomic_notifier_chain_unregister(&mut fips_fail_notif_chain, &mut (*fips_h).nb);

    /* Kill tasklet */
    tasklet_kill(&mut (*fips_h).tasklet);
    (*drvdata).fips_handle = core::ptr::null_mut();
}

pub unsafe fn fips_handler(drvdata: *mut cc_drvdata) {
    let fips_handle_ptr = (*drvdata).fips_handle;

    if (*drvdata).hw_rev < CC_HW_REV_712 {
        return;
    }

    tasklet_schedule(&mut (*fips_handle_ptr).tasklet);
}

unsafe fn tee_fips_error(dev: *mut device) {
    if fips_enabled {
        panic!("ccree: TEE reported cryptographic error in fips mode!\n");
    } else {
        dev_err(dev, "TEE reported error!\n");
    }
}

/*
 * This function check if cryptocell tee fips error occurred
 * and in such case triggers system error
 */
pub unsafe fn cc_tee_handle_fips_error(p_drvdata: *mut cc_drvdata) {
    let dev = drvdata_to_dev(p_drvdata);

    if !cc_get_tee_fips_status(p_drvdata) {
        tee_fips_error(dev);
    }
}

/* Deferred service handler, run as interrupt-fired tasklet */
unsafe fn fips_dsr(devarg: c_ulong) {
    let drvdata = devarg as *mut cc_drvdata;
    let irq: u32;
    let val: u32;

    irq = (*drvdata).irq & CC_GPR0_IRQ_MASK;

    if irq != 0 {
        cc_tee_handle_fips_error(drvdata);
    }

    /* after verifying that there is nothing to do,
     * unmask AXI completion interrupt.
     */
    val = CC_REG(HOST_IMR) & !irq;
    cc_iowrite(drvdata, CC_REG(HOST_IMR), val);
}

/* The function called once at driver entry point .*/
pub unsafe fn cc_fips_init(p_drvdata: *mut cc_drvdata) -> i32 {
    let fips_h: *mut cc_fips_handle;
    let dev = drvdata_to_dev(p_drvdata);

    if (*p_drvdata).hw_rev < CC_HW_REV_712 {
        return 0;
    }

    fips_h = devm_kzalloc(dev, core::mem::size_of::<cc_fips_handle>(), GFP_KERNEL) as *mut cc_fips_handle;
    if fips_h.is_null() {
        return -ENOMEM;
    }

    (*p_drvdata).fips_handle = fips_h;

    dev_dbg(dev, "Initializing fips tasklet\n");
    tasklet_init(&mut (*fips_h).tasklet, fips_dsr, p_drvdata as c_ulong);
    (*fips_h).drvdata = p_drvdata;
    (*fips_h).nb.notifier_call = Some(cc_ree_fips_failure);
    atomic_notifier_chain_register(&mut fips_fail_notif_chain, &mut (*fips_h).nb);

    cc_tee_handle_fips_error(p_drvdata);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
