// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2011-2012 Calxeda, Inc.
 */

// Linux kernel dependencies supplied by the surrounding repository.

const SR_CLR_SB_ECC_INTR: usize = 0x0;
const SR_CLR_DB_ECC_INTR: usize = 0x4;

#[repr(C)]
struct hb_l2_drvdata {
    base: *mut core::ffi::c_void,
    sb_irq: i32,
    db_irq: i32,
}

unsafe fn highbank_l2_err_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let dci = dev_id as *mut edac_device_ctl_info;
    let drvdata = (*dci).pvt_info as *mut hb_l2_drvdata;

    if irq == (*drvdata).sb_irq {
        writel(1, ((*drvdata).base as usize + SR_CLR_SB_ECC_INTR) as *mut u32);
        edac_device_handle_ce(dci, 0, 0, (*dci).ctl_name);
    }
    if irq == (*drvdata).db_irq {
        writel(1, ((*drvdata).base as usize + SR_CLR_DB_ECC_INTR) as *mut u32);
        edac_device_handle_ue(dci, 0, 0, (*dci).ctl_name);
    }

    IRQ_HANDLED
}

static hb_l2_err_of_match: [of_device_id; 2] = [
    of_device_id { compatible: "calxeda,hb-sregs-l2-ecc" },
    of_device_id { compatible: "" },
];

// MODULE_DEVICE_TABLE(of, hb_l2_err_of_match);

unsafe fn highbank_l2_err_probe(pdev: *mut platform_device) -> i32 {
    let mut id: *const of_device_id;
    let mut dci: *mut edac_device_ctl_info;
    let mut drvdata: *mut hb_l2_drvdata;
    let mut r: *mut resource;
    let mut res: i32 = 0;

    dci = edac_device_alloc_ctl_info(
        core::mem::size_of::<hb_l2_drvdata>(), "cpu", 1, "L", 1, 2, 0,
    );
    if dci.is_null() {
        return -ENOMEM;
    }

    drvdata = (*dci).pvt_info as *mut hb_l2_drvdata;
    (*dci).dev = &mut (*pdev).dev;
    platform_set_drvdata(pdev, dci);

    if !devres_open_group(&mut (*pdev).dev, core::ptr::null_mut(), GFP_KERNEL) {
        return -ENOMEM;
    }

    r = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if r.is_null() {
        dev_err(&mut (*pdev).dev, "Unable to get mem resource\n");
        res = -ENODEV;
        devres_release_group(&mut (*pdev).dev, core::ptr::null_mut());
        edac_device_free_ctl_info(dci);
        return res;
    }

    if !devm_request_mem_region(&mut (*pdev).dev, (*r).start, resource_size(r), dev_name(&mut (*pdev).dev)) {
        dev_err(&mut (*pdev).dev, "Error while requesting mem region\n");
        res = -EBUSY;
        devres_release_group(&mut (*pdev).dev, core::ptr::null_mut());
        edac_device_free_ctl_info(dci);
        return res;
    }

    (*drvdata).base = devm_ioremap(&mut (*pdev).dev, (*r).start, resource_size(r));
    if (*drvdata).base.is_null() {
        dev_err(&mut (*pdev).dev, "Unable to map regs\n");
        res = -ENOMEM;
        devres_release_group(&mut (*pdev).dev, core::ptr::null_mut());
        edac_device_free_ctl_info(dci);
        return res;
    }

    id = of_match_device(hb_l2_err_of_match.as_ptr(), &mut (*pdev).dev);
    (*dci).mod_name = (*(*pdev).dev.driver).name;
    (*dci).ctl_name = if !id.is_null() { (*id).compatible } else { "unknown" };
    (*dci).dev_name = dev_name(&mut (*pdev).dev);

    if edac_device_add_device(dci) != 0 {
        devres_release_group(&mut (*pdev).dev, core::ptr::null_mut());
        edac_device_free_ctl_info(dci);
        return res;
    }

    (*drvdata).db_irq = platform_get_irq(pdev, 0);
    res = devm_request_irq(&mut (*pdev).dev, (*drvdata).db_irq, Some(highbank_l2_err_handler), 0, dev_name(&mut (*pdev).dev), dci);
    if res < 0 {
        edac_device_del_device(&mut (*pdev).dev);
        devres_release_group(&mut (*pdev).dev, core::ptr::null_mut());
        edac_device_free_ctl_info(dci);
        return res;
    }

    (*drvdata).sb_irq = platform_get_irq(pdev, 1);
    res = devm_request_irq(&mut (*pdev).dev, (*drvdata).sb_irq, Some(highbank_l2_err_handler), 0, dev_name(&mut (*pdev).dev), dci);
    if res < 0 {
        edac_device_del_device(&mut (*pdev).dev);
        devres_release_group(&mut (*pdev).dev, core::ptr::null_mut());
        edac_device_free_ctl_info(dci);
        return res;
    }

    devres_close_group(&mut (*pdev).dev, core::ptr::null_mut());
    0
}

unsafe fn highbank_l2_err_remove(pdev: *mut platform_device) {
    let dci = platform_get_drvdata(pdev);
    edac_device_del_device(&mut (*pdev).dev);
    edac_device_free_ctl_info(dci);
}

static mut highbank_l2_edac_driver: platform_driver = platform_driver {
    probe: Some(highbank_l2_err_probe),
    remove: Some(highbank_l2_err_remove),
    driver: driver {
        name: "hb_l2_edac",
        of_match_table: hb_l2_err_of_match.as_ptr(),
    },
};

// module_platform_driver(highbank_l2_edac_driver);

// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Calxeda, Inc.");
// MODULE_DESCRIPTION("EDAC Driver for Calxeda Highbank L2 Cache");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
