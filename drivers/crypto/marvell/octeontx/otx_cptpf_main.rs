// SPDX-License-Identifier: GPL-2.0
/* Marvell OcteonTX CPT driver
 *
 * Copyright (C) 2019 Marvell International Ltd.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

// Dependencies supplied by the surrounding kernel-driver translation.

const DRV_NAME: &str = "octeontx-cpt";
const DRV_VERSION: &str = "1.0";

unsafe fn otx_cpt_disable_mbox_interrupts(cpt: *mut otx_cpt_device) {
    /* Disable mbox(0) interrupts for all VFs */
    writeq(!0u64, (*cpt).reg_base.add(OTX_CPT_PF_MBOX_ENA_W1CX(0) as usize));
}

unsafe fn otx_cpt_enable_mbox_interrupts(cpt: *mut otx_cpt_device) {
    /* Enable mbox(0) interrupts for all VFs */
    writeq(!0u64, (*cpt).reg_base.add(OTX_CPT_PF_MBOX_ENA_W1SX(0) as usize));
}

unsafe extern "C" fn otx_cpt_mbx0_intr_handler(_irq: i32, cpt: *mut core::ffi::c_void) -> irqreturn_t {
    otx_cpt_mbox_intr_handler(cpt, 0);
    IRQ_HANDLED
}

unsafe fn otx_cpt_reset(cpt: *mut otx_cpt_device) {
    writeq(1, (*cpt).reg_base.add(OTX_CPT_PF_RESET as usize));
}

unsafe fn otx_cpt_find_max_enabled_cores(cpt: *mut otx_cpt_device) {
    let mut pf_cnsts: otx_cptx_pf_constants = core::mem::zeroed();
    pf_cnsts.u = readq((*cpt).reg_base.add(OTX_CPT_PF_CONSTANTS as usize));
    (*cpt).eng_grps.avail.max_se_cnt = pf_cnsts.s.se;
    (*cpt).eng_grps.avail.max_ae_cnt = pf_cnsts.s.ae;
}

unsafe fn otx_cpt_check_bist_status(cpt: *mut otx_cpt_device) -> u32 {
    let mut bist_sts: otx_cptx_pf_bist_status = core::mem::zeroed();
    bist_sts.u = readq((*cpt).reg_base.add(OTX_CPT_PF_BIST_STATUS as usize)) as u32;
    bist_sts.u
}

unsafe fn otx_cpt_check_exe_bist_status(cpt: *mut otx_cpt_device) -> u64 {
    let mut bist_sts: otx_cptx_pf_exe_bist_status = core::mem::zeroed();
    bist_sts.u = readq((*cpt).reg_base.add(OTX_CPT_PF_EXE_BIST_STATUS as usize));
    bist_sts.u
}

unsafe fn otx_cpt_device_init(cpt: *mut otx_cpt_device) -> i32 {
    let dev = &mut (*(*cpt).pdev).dev;
    let mut sdevid: u16 = 0;
    let mut bist: u64;

    /* Reset the PF when probed first */
    otx_cpt_reset(cpt);
    mdelay(100);

    pci_read_config_word((*cpt).pdev, PCI_SUBSYSTEM_ID, &mut sdevid);

    /* Check BIST status */
    bist = otx_cpt_check_bist_status(cpt) as u64;
    if bist != 0 {
        dev_err(dev, "RAM BIST failed with code 0x{:x}\n", bist);
        return -ENODEV;
    }

    bist = otx_cpt_check_exe_bist_status(cpt);
    if bist != 0 {
        dev_err(dev, "Engine BIST failed with code 0x{:x}\n", bist);
        return -ENODEV;
    }

    /* Get max enabled cores */
    otx_cpt_find_max_enabled_cores(cpt);

    if (sdevid == OTX_CPT_PCI_PF_SUBSYS_ID && (*cpt).eng_grps.avail.max_se_cnt == 0) {
        (*cpt).pf_type = OTX_CPT_AE;
    } else if (sdevid == OTX_CPT_PCI_PF_SUBSYS_ID && (*cpt).eng_grps.avail.max_ae_cnt == 0) {
        (*cpt).pf_type = OTX_CPT_SE;
    }

    /* Get max VQs/VFs supported by the device */
    (*cpt).max_vfs = pci_sriov_get_totalvfs((*cpt).pdev);

    /* Disable all cores */
    otx_cpt_disable_all_cores(cpt);
    0
}

unsafe fn otx_cpt_register_interrupts(cpt: *mut otx_cpt_device) -> i32 {
    let dev = &mut (*(*cpt).pdev).dev;
    let mbox_int_idx: u32 = OTX_CPT_PF_MBOX_INT;
    let num_vec: u32 = OTX_CPT_PF_MSIX_VECTORS;
    let mut ret: i32;

    /* Enable MSI-X */
    ret = pci_alloc_irq_vectors((*cpt).pdev, num_vec, num_vec, PCI_IRQ_MSIX);
    if ret < 0 {
        dev_err(&(*cpt).pdev).dev, "Request for #{} msix vectors failed\n", num_vec);
        return ret;
    }

    /* Register mailbox interrupt handlers */
    ret = request_irq(
        pci_irq_vector((*cpt).pdev, OTX_CPT_PF_INT_VEC_E_MBOXX(mbox_int_idx, 0)),
        Some(otx_cpt_mbx0_intr_handler), 0, "CPT Mbox0", cpt,
    );
    if ret != 0 {
        dev_err(dev, "Request irq failed\n");
        pci_free_irq_vectors((*cpt).pdev);
        return ret;
    }
    /* Enable mailbox interrupt */
    otx_cpt_enable_mbox_interrupts(cpt);
    0
}

unsafe fn otx_cpt_unregister_interrupts(cpt: *mut otx_cpt_device) {
    let mbox_int_idx: u32 = OTX_CPT_PF_MBOX_INT;
    otx_cpt_disable_mbox_interrupts(cpt);
    free_irq(pci_irq_vector((*cpt).pdev, OTX_CPT_PF_INT_VEC_E_MBOXX(mbox_int_idx, 0)), cpt);
    pci_free_irq_vectors((*cpt).pdev);
}

unsafe fn otx_cpt_sriov_configure(pdev: *mut pci_dev, mut numvfs: i32) -> i32 {
    let cpt = pci_get_drvdata(pdev) as *mut otx_cpt_device;
    let mut ret: i32 = 0;
    if numvfs > (*cpt).max_vfs { numvfs = (*cpt).max_vfs; }
    if numvfs > 0 {
        ret = otx_cpt_try_create_default_eng_grps((*cpt).pdev, &mut (*cpt).eng_grps, (*cpt).pf_type);
        if ret != 0 { return ret; }
        (*cpt).vfs_enabled = numvfs;
        ret = pci_enable_sriov(pdev, numvfs);
        if ret != 0 { (*cpt).vfs_enabled = 0; return ret; }
        otx_cpt_set_eng_grps_is_rdonly(&mut (*cpt).eng_grps, true);
        try_module_get(THIS_MODULE);
        ret = numvfs;
    } else {
        pci_disable_sriov(pdev);
        otx_cpt_set_eng_grps_is_rdonly(&mut (*cpt).eng_grps, false);
        module_put(THIS_MODULE);
        (*cpt).vfs_enabled = 0;
    }
    dev_notice(&(*cpt).pdev).dev, "VFs enabled: {}\n", ret);
    ret
}

unsafe fn otx_cpt_probe(pdev: *mut pci_dev, _ent: *const pci_device_id) -> i32 {
    let dev = &mut (*pdev).dev;
    let cpt = devm_kzalloc(dev, core::mem::size_of::<otx_cpt_device>(), GFP_KERNEL) as *mut otx_cpt_device;
    let mut err: i32;
    if cpt.is_null() { return -ENOMEM; }
    pci_set_drvdata(pdev, cpt as *mut core::ffi::c_void);
    (*cpt).pdev = pdev;
    err = pci_enable_device(pdev);
    if err != 0 { dev_err(dev, "Failed to enable PCI device\n"); goto_err_clear_drvdata(pdev); return err; }
    err = pci_request_regions(pdev, DRV_NAME);
    if err != 0 { dev_err(dev, "PCI request regions failed 0x{:x}\n", err); pci_disable_device(pdev); goto_err_clear_drvdata(pdev); return err; }
    err = dma_set_mask_and_coherent(&mut (*pdev).dev, DMA_BIT_MASK(48));
    if err != 0 { dev_err(dev, "Unable to get usable 48-bit DMA configuration\n"); pci_release_regions(pdev); pci_disable_device(pdev); goto_err_clear_drvdata(pdev); return err; }
    /* MAP PF's configuration registers */
    (*cpt).reg_base = pci_iomap(pdev, OTX_CPT_PF_PCI_CFG_BAR, 0);
    if (*cpt).reg_base.is_null() { dev_err(dev, "Cannot map config register space, aborting\n"); pci_release_regions(pdev); pci_disable_device(pdev); goto_err_clear_drvdata(pdev); return -ENOMEM; }
    /* CPT device HW initialization */
    err = otx_cpt_device_init(cpt);
    if err != 0 { pci_iounmap(pdev, (*cpt).reg_base); pci_release_regions(pdev); pci_disable_device(pdev); goto_err_clear_drvdata(pdev); return err; }
    /* Register interrupts */
    err = otx_cpt_register_interrupts(cpt);
    if err != 0 { pci_iounmap(pdev, (*cpt).reg_base); pci_release_regions(pdev); pci_disable_device(pdev); goto_err_clear_drvdata(pdev); return err; }
    /* Initialize engine groups */
    err = otx_cpt_init_eng_grps(pdev, &mut (*cpt).eng_grps, (*cpt).pf_type);
    if err != 0 { otx_cpt_unregister_interrupts(cpt); pci_iounmap(pdev, (*cpt).reg_base); pci_release_regions(pdev); pci_disable_device(pdev); goto_err_clear_drvdata(pdev); return err; }
    0
}

unsafe fn goto_err_clear_drvdata(pdev: *mut pci_dev) { pci_set_drvdata(pdev, core::ptr::null_mut()); }

unsafe fn otx_cpt_remove(pdev: *mut pci_dev) {
    let cpt = pci_get_drvdata(pdev) as *mut otx_cpt_device;
    if cpt.is_null() { return; }
    /* Disable VFs */
    pci_disable_sriov(pdev);
    /* Cleanup engine groups */
    otx_cpt_cleanup_eng_grps(pdev, &mut (*cpt).eng_grps);
    /* Disable CPT PF interrupts */
    otx_cpt_unregister_interrupts(cpt);
    /* Disengage SE and AE cores from all groups */
    otx_cpt_disable_all_cores(cpt);
    pci_iounmap(pdev, (*cpt).reg_base);
    pci_release_regions(pdev);
    pci_disable_device(pdev);
    pci_set_drvdata(pdev, core::ptr::null_mut());
}

/* Supported devices */
static OTX_CPT_ID_TABLE: [pci_device_id; 2] = [
    PCI_DEVICE(PCI_VENDOR_ID_CAVIUM, OTX_CPT_PCI_PF_DEVICE_ID),
    PCI_DEVICE(0, 0), /* end of table */
];

static mut OTX_CPT_PCI_DRIVER: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: OTX_CPT_ID_TABLE.as_ptr(),
    probe: Some(otx_cpt_probe),
    remove: Some(otx_cpt_remove),
    sriov_configure: Some(otx_cpt_sriov_configure),
};

// module_pci_driver(otx_cpt_pci_driver);
// MODULE_DEVICE_TABLE(pci, otx_cpt_id_table);
// MODULE_AUTHOR("Marvell International Ltd.");
// MODULE_DESCRIPTION("Marvell OcteonTX CPT Physical Function Driver");
// MODULE_LICENSE("GPL v2");
// MODULE_VERSION(DRV_VERSION);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
