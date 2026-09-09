// SPDX-License-Identifier: GPL-2.0-only

// Interface to CCP/SEV-TIO for generic PCIe TDISP module

// Kernel dependencies supplied externally:
// linux/pci.h, linux/device.h, linux/tsm.h, linux/iommu.h, linux/pci-doe.h,
// linux/bitfield.h, linux/module.h, asm/sev-common.h, asm/sev.h,
// psp-dev.h, sev-dev.h, and sev-dev-tio.h.

// MODULE_IMPORT_NS("PCI_IDE");

const PCI_IDE_EP: usize = 0;
const PCI_IDE_RP: usize = 1;

unsafe fn dev_to_sp(dev: *mut device) -> *mut sp_device {
    dev_get_drvdata(dev) as *mut sp_device
}

unsafe fn dev_to_psp(dev: *mut device) -> *mut psp_device {
    (*dev_to_sp(dev)).psp_data as *mut psp_device
}

unsafe fn dev_to_sev(dev: *mut device) -> *mut sev_device {
    (*dev_to_psp(dev)).sev_data as *mut sev_device
}

unsafe fn tsm_dev_to_sev(tsmdev: *mut tsm_dev) -> *mut sev_device {
    dev_to_sev((*tsmdev).dev.parent)
}

unsafe fn pdev_to_tio_dsm(pdev: *mut pci_dev) -> *mut tio_dsm {
    // container_of((pdev)->tsm, struct tio_dsm, tsm.base_tsm)
    ((*pdev).tsm as *mut tio_dsm).cast::<u8>().sub(0) as *mut tio_dsm
}

unsafe fn sev_tio_spdm_cmd(dsm: *mut tio_dsm, mut ret: i32) -> i32 {
    let dev_data = &mut (*dsm).data;
    let spdm = &mut dev_data.spdm;

    /* Check the main command handler response before entering the loop */
    if ret == 0 && dev_data.psp_ret != SEV_RET_SUCCESS {
        return -EINVAL;
    }
    if ret <= 0 {
        return ret;
    }

    /* ret > 0 means "SPDM requested" */
    while ret == PCI_DOE_FEATURE_CMA || ret == PCI_DOE_FEATURE_SSESSION {
        ret = pci_doe((*dsm).tsm.doe_mb, PCI_VENDOR_ID_PCI_SIG, ret,
                      spdm.req, spdm.req_len, spdm.rsp, spdm.rsp_len);
        if ret < 0 {
            break;
        }
        // WARN_ON_ONCE(ret == 0); /* The response should never be empty */
        spdm.rsp_len = ret;
        ret = sev_tio_continue(dev_data);
    }
    ret
}

unsafe fn stream_enable(ide: *mut pci_ide) -> i32 {
    let rp = pcie_find_root_port((*ide).pdev);
    let mut ret = pci_ide_stream_enable((*ide).pdev, ide);
    if ret != 0 && ret != -ENXIO { return ret; }
    ret = pci_ide_stream_enable(rp, ide);
    if ret != 0 && ret != -ENXIO { pci_ide_stream_disable((*ide).pdev, ide); }
    ret
}

unsafe fn streams_enable(ide: *mut *mut pci_ide) -> i32 {
    let mut ret = 0;
    for i in 0..TIO_IDE_MAX_TC {
        if !(*ide.add(i)).is_null() {
            ret = stream_enable(*ide.add(i));
            if ret != 0 { break; }
        }
    }
    ret
}

unsafe fn stream_disable(ide: *mut pci_ide) {
    pci_ide_stream_disable((*ide).pdev, ide);
    pci_ide_stream_disable(pcie_find_root_port((*ide).pdev), ide);
}

unsafe fn streams_disable(ide: *mut *mut pci_ide) {
    for i in 0..TIO_IDE_MAX_TC { if !(*ide.add(i)).is_null() { stream_disable(*ide.add(i)); } }
}

unsafe fn stream_setup(ide: *mut pci_ide) {
    let rp = pcie_find_root_port((*ide).pdev);
    (*ide).partner[PCI_IDE_EP].rid_start = 0;
    (*ide).partner[PCI_IDE_EP].rid_end = 0xffff;
    (*ide).partner[PCI_IDE_RP].rid_start = 0;
    (*ide).partner[PCI_IDE_RP].rid_end = 0xffff;
    (*ide).pdev.ide_cfg = 0;
    (*ide).pdev.ide_tee_limit = 1;
    (*rp).ide_cfg = 1;
    (*rp).ide_tee_limit = 0;
    pci_warn((*ide).pdev, "Forcing CFG/TEE for %s", pci_name(rp));
    pci_ide_stream_setup((*ide).pdev, ide);
    pci_ide_stream_setup(rp, ide);
}

unsafe fn streams_setup(ide: *mut *mut pci_ide, ids: *mut u8) -> u8 {
    let mut def = false;
    let mut tc_mask = 0u8;
    for i in 0..TIO_IDE_MAX_TC {
        if (*ide.add(i)).is_null() { *ids.add(i) = 0xff; continue; }
        tc_mask |= 1u8 << i;
        *ids.add(i) = (*(*ide.add(i))).stream_id;
        if !def {
            let settings = pci_ide_to_settings((*(*ide.add(i))).pdev, *ide.add(i));
            (*settings).default_stream = 1;
            def = true;
        }
        stream_setup(*ide.add(i));
    }
    tc_mask
}

unsafe fn streams_register(ide: *mut *mut pci_ide) -> i32 {
    let mut ret = 0;
    for i in 0..TIO_IDE_MAX_TC {
        if !(*ide.add(i)).is_null() { ret = pci_ide_stream_register(*ide.add(i)); if ret != 0 { break; } }
    }
    ret
}

unsafe fn streams_unregister(ide: *mut *mut pci_ide) {
    for i in 0..TIO_IDE_MAX_TC { if !(*ide.add(i)).is_null() { pci_ide_stream_unregister(*ide.add(i)); } }
}

unsafe fn stream_teardown(ide: *mut pci_ide) {
    pci_ide_stream_teardown((*ide).pdev, ide);
    pci_ide_stream_teardown(pcie_find_root_port((*ide).pdev), ide);
}

unsafe fn streams_teardown(ide: *mut *mut pci_ide) {
    for i in 0..TIO_IDE_MAX_TC {
        if !(*ide.add(i)).is_null() { stream_teardown(*ide.add(i)); pci_ide_stream_free(*ide.add(i)); *ide.add(i) = core::ptr::null_mut(); }
    }
}

unsafe fn stream_alloc(pdev: *mut pci_dev, ide: *mut *mut pci_ide, tc: usize) -> i32 {
    if !(*ide.add(tc)).is_null() { pci_err(pdev, "Stream for class=%d already registered", tc); return -EBUSY; }
    let ide1 = pci_ide_stream_alloc(pdev);
    if ide1.is_null() { return -EFAULT; }
    (*ide1).stream_id = (*ide1).host_bridge_stream;
    *ide.add(tc) = ide1;
    0
}

unsafe fn tio_pf0_probe(pdev: *mut pci_dev, sev: *mut sev_device) -> *mut pci_tsm {
    let dsm = kzalloc_tio_dsm();
    if dsm.is_null() { return core::ptr::null_mut(); }
    let rc = pci_tsm_pf0_constructor(pdev, &mut (*dsm).tsm, (*sev).tsmdev);
    if rc != 0 { return core::ptr::null_mut(); }
    pci_dbg(pdev, "TSM enabled\n");
    (*dsm).sev = sev;
    &mut (*dsm).tsm.base_tsm
}

unsafe fn dsm_probe(tsmdev: *mut tsm_dev, pdev: *mut pci_dev) -> *mut pci_tsm {
    let sev = tsm_dev_to_sev(tsmdev);
    if is_pci_tsm_pf0(pdev) { return tio_pf0_probe(pdev, sev); }
    core::ptr::null_mut()
}

unsafe fn dsm_remove(tsm: *mut pci_tsm) {
    let pdev = (*tsm).pdev;
    pci_dbg(pdev, "TSM disabled\n");
    if is_pci_tsm_pf0(pdev) {
        let dsm = tsm as *mut tio_dsm;
        pci_tsm_pf0_destructor(&mut (*dsm).tsm);
        kfree(dsm);
    }
}

unsafe fn dsm_create(dsm: *mut tio_dsm) -> i32 {
    let pdev = (*dsm).tsm.base_tsm.pdev;
    if (*pdev).bus.is_null() { return -ENODEV; }
    let segment_id = pci_domain_nr((*pdev).bus);
    let rootport = pcie_find_root_port(pdev);
    let device_id = pci_dev_id(pdev);
    let mut lnkcap = 0u32;
    if pci_read_config_dword(rootport, pci_pcie_cap(rootport) + PCI_EXP_LNKCAP, &mut lnkcap) != 0 { return -ENODEV; }
    let root_port_id = (lnkcap & PCI_EXP_LNKCAP_PN) >> PCI_EXP_LNKCAP_PN_SHIFT;
    sev_tio_dev_create(&mut (*dsm).data, device_id, root_port_id as u16, segment_id)
}

unsafe fn dsm_connect(pdev: *mut pci_dev) -> i32 {
    let dsm = pdev_to_tio_dsm(pdev);
    let dev_data = &mut (*dsm).data;
    let mut ids = [0u8; TIO_IDE_MAX_TC];
    if pci_find_doe_mailbox(pdev, PCI_VENDOR_ID_PCI_SIG, PCI_DOE_FEATURE_SSESSION) != (*dsm).tsm.doe_mb {
        pci_err(pdev, "CMA DOE MB must support SSESSION\n");
        return -EFAULT;
    }
    let mut ret = stream_alloc(pdev, dev_data.ide.as_mut_ptr(), 0);
    if ret != 0 { return ret; }
    ret = dsm_create(dsm);
    if ret != 0 { streams_teardown(dev_data.ide.as_mut_ptr()); return ret; }
    let tc_mask = streams_setup(dev_data.ide.as_mut_ptr(), ids.as_mut_ptr());
    ret = sev_tio_dev_connect(dev_data, tc_mask, ids.as_mut_ptr(), dev_data.cert_slot);
    ret = sev_tio_spdm_cmd(dsm, ret);
    if ret != 0 { sev_tio_dev_reclaim(dev_data); streams_disable(dev_data.ide.as_mut_ptr()); streams_teardown(dev_data.ide.as_mut_ptr()); return ret; }
    streams_enable(dev_data.ide.as_mut_ptr());
    ret = streams_register(dev_data.ide.as_mut_ptr());
    if ret != 0 { sev_tio_dev_reclaim(dev_data); streams_disable(dev_data.ide.as_mut_ptr()); streams_teardown(dev_data.ide.as_mut_ptr()); }
    ret
}

unsafe fn dsm_disconnect(pdev: *mut pci_dev) {
    let force = SYSTEM_HALT <= system_state && system_state <= SYSTEM_RESTART;
    let dsm = pdev_to_tio_dsm(pdev);
    let dev_data = &mut (*dsm).data;
    let mut ret = sev_tio_dev_disconnect(dev_data, force);
    ret = sev_tio_spdm_cmd(dsm, ret);
    if ret != 0 && !force { ret = sev_tio_dev_disconnect(dev_data, true); sev_tio_spdm_cmd(dsm, ret); }
    sev_tio_dev_reclaim(dev_data);
    streams_disable(dev_data.ide.as_mut_ptr());
    streams_unregister(dev_data.ide.as_mut_ptr());
    streams_teardown(dev_data.ide.as_mut_ptr());
}

#[repr(C)]
static mut sev_tsm_ops: pci_tsm_ops = pci_tsm_ops { probe: dsm_probe, remove: dsm_remove, connect: dsm_connect, disconnect: dsm_disconnect };

unsafe fn sev_tsm_init_locked(sev: *mut sev_device, tio_status_page: *mut core::ffi::c_void) {
    let t = kzalloc_sev_tio_status();
    if (*sev).tio_status != core::ptr::null_mut() { warn_on(true); }
    if t.is_null() { return; }
    let mut ret = sev_tio_init_locked(tio_status_page);
    if ret != 0 { pr_warn("SEV-TIO STATUS failed with %d\n", ret); kfree(t.cast()); return; }
    let tsmdev = tsm_register((*sev).dev, &sev_tsm_ops);
    if is_err(tsmdev) { kfree(t.cast()); return; }
    memcpy(t.cast(), tio_status_page, core::mem::size_of::<sev_tio_status>());
    pr_notice("SEV-TIO status: EN=%d INIT_DONE=%d rq=%d..%d rs=%d..%d scr=%d..%d out=%d..%d dev=%d tdi=%d algos=%x\n", (*t).tio_en, (*t).tio_init_done, (*t).spdm_req_size_min, (*t).spdm_req_size_max, (*t).spdm_rsp_size_min, (*t).spdm_rsp_size_max, (*t).spdm_scratch_size_min, (*t).spdm_scratch_size_max, (*t).spdm_out_size_min, (*t).spdm_out_size_max, (*t).devctx_size, (*t).tdictx_size, (*t).tio_crypto_alg);
    (*sev).tsmdev = tsmdev;
    (*sev).tio_status = t;
}

unsafe fn sev_tsm_uninit(sev: *mut sev_device) {
    if !(*sev).tsmdev.is_null() { tsm_unregister((*sev).tsmdev); }
    (*sev).tsmdev = core::ptr::null_mut();
}

// The remaining callbacks retain the C implementation's sequencing and are
// declared against the externally supplied kernel structures and helpers.
extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn kfree(p: *mut core::ffi::c_void);
    fn kzalloc_tio_dsm() -> *mut tio_dsm;
    fn kzalloc_sev_tio_status() -> *mut sev_tio_status;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
