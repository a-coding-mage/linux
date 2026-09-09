// SPDX-License-Identifier: GPL-2.0-only
// Translated from nitrox_main.c. Kernel headers and symbols are supplied by
// the surrounding Rust/kernel integration.

const CNN55XX_DEV_ID: u32 = 0x12;
const UCODE_HLEN: usize = 48;
const DEFAULT_SE_GROUP: u32 = 0;
const DEFAULT_AE_GROUP: u32 = 0;
const DRIVER_VERSION: &str = "1.2";
const CNN55XX_UCD_BLOCK_SIZE: u32 = 32768;
const CNN55XX_MAX_UCODE_SIZE: u32 = CNN55XX_UCD_BLOCK_SIZE * 2;
const FW_DIR: &str = "cavium/";
const SE_FW: &str = "cavium/cnn55xx_se.fw";
const AE_FW: &str = "cavium/cnn55xx_ae.fw";

static NITROX_DRIVER_NAME: &[u8] = b"CNN55XX\0";
static mut NDEVLIST: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut DEVLIST_LOCK: mutex = mutex { _private: [] };
static mut NUM_DEVICES: u32 = 0;

#[repr(C)]
struct ucode {
    id: u8,
    version: [core::ffi::c_char; VERSION_LEN - 1],
    code_size: __be32,
    raz: [u8; 12],
    code: [u64; 0],
}

static mut QLEN: u32 = DEFAULT_CMD_QLEN;

unsafe fn write_to_ucd_unit(
    ndev: *mut nitrox_device,
    ucode_size: u32,
    ucode_data: *mut u64,
    block_num: i32,
) {
    let mut code_size: u32;
    let mut offset: u64;
    let mut data: u64;
    let mut i: i32 = 0;

    offset = UCD_UCODE_LOAD_BLOCK_NUM;
    nitrox_write_csr(ndev, offset, block_num as u64);

    code_size = roundup(ucode_size, 16);
    while code_size != 0 {
        data = *ucode_data.offset(i as isize);
        offset = UCD_UCODE_LOAD_IDX_DATAX(i);
        nitrox_write_csr(ndev, offset, data);
        code_size -= 8;
        i += 1;
    }

    usleep_range(300, 400);
}

unsafe fn nitrox_load_fw(ndev: *mut nitrox_device) -> i32 {
    let mut fw: *const firmware;
    let mut fw_name: *const core::ffi::c_char;
    let mut ucode: *mut ucode;
    let mut ucode_data: *mut u64;
    let mut offset: u64;
    let mut core_2_eid_val: ucd_core_eid_ucode_block_num;
    let mut aqm_grp_execmask_lo: aqm_grp_execmsk_lo;
    let mut aqm_grp_execmask_hi: aqm_grp_execmsk_hi;
    let mut ucode_size: u32;
    let mut ret: i32;
    let mut i: i32 = 0;

    fw_name = SE_FW.as_ptr() as *const core::ffi::c_char;
    dev_info(DEV(ndev), c_str!("Loading firmware \"%s\"\n"), fw_name);
    ret = request_firmware(&mut fw, fw_name, DEV(ndev));
    if ret < 0 {
        dev_err(DEV(ndev), c_str!("failed to get firmware %s\n"), fw_name);
        return ret;
    }
    ucode = (*fw).data as *mut ucode;
    ucode_size = be32_to_cpu((*ucode).code_size) * 2;
    if ucode_size == 0 || ucode_size > CNN55XX_MAX_UCODE_SIZE {
        dev_err(DEV(ndev), c_str!("Invalid ucode size: %u for firmware %s\n"), ucode_size, fw_name);
        release_firmware(fw);
        return -EINVAL;
    }
    ucode_data = (*ucode).code.as_mut_ptr();
    memcpy((*ndev).hw.fw_name[0].as_mut_ptr() as *mut core::ffi::c_void,
           (*ucode).version.as_ptr() as *const core::ffi::c_void, VERSION_LEN - 2);
    (*ndev).hw.fw_name[0][VERSION_LEN - 1] = 0;
    write_to_ucd_unit(ndev, ucode_size, ucode_data, 0);
    release_firmware(fw);

    offset = POM_GRP_EXECMASKX(DEFAULT_SE_GROUP);
    nitrox_write_csr(ndev, offset, !0u64);
    core_2_eid_val.value = 0;
    core_2_eid_val.ucode_blk = 0;
    core_2_eid_val.ucode_len = if ucode_size <= CNN55XX_UCD_BLOCK_SIZE { 1 } else { 0 };
    i = 0;
    while i < (*ndev).hw.se_cores {
        nitrox_write_csr(ndev, UCD_SE_EID_UCODE_BLOCK_NUMX(i), core_2_eid_val.value);
        i += 1;
    }

    fw_name = AE_FW.as_ptr() as *const core::ffi::c_char;
    dev_info(DEV(ndev), c_str!("Loading firmware \"%s\"\n"), fw_name);
    ret = request_firmware(&mut fw, fw_name, DEV(ndev));
    if ret < 0 {
        dev_err(DEV(ndev), c_str!("failed to get firmware %s\n"), fw_name);
        return ret;
    }
    ucode = (*fw).data as *mut ucode;
    ucode_size = be32_to_cpu((*ucode).code_size) * 2;
    if ucode_size == 0 || ucode_size > CNN55XX_MAX_UCODE_SIZE {
        dev_err(DEV(ndev), c_str!("Invalid ucode size: %u for firmware %s\n"), ucode_size, fw_name);
        release_firmware(fw);
        return -EINVAL;
    }
    ucode_data = (*ucode).code.as_mut_ptr();
    memcpy((*ndev).hw.fw_name[1].as_mut_ptr() as *mut core::ffi::c_void,
           (*ucode).version.as_ptr() as *const core::ffi::c_void, VERSION_LEN - 2);
    (*ndev).hw.fw_name[1][VERSION_LEN - 1] = 0;
    write_to_ucd_unit(ndev, ucode_size, ucode_data, 2);
    release_firmware(fw);

    offset = AQM_GRP_EXECMSK_LOX(DEFAULT_AE_GROUP);
    aqm_grp_execmask_lo.exec_0_to_39 = 0xFFFFFFFFFF;
    nitrox_write_csr(ndev, offset, aqm_grp_execmask_lo.value);
    offset = AQM_GRP_EXECMSK_HIX(DEFAULT_AE_GROUP);
    aqm_grp_execmask_hi.exec_40_to_79 = 0xFFFFFFFFFF;
    nitrox_write_csr(ndev, offset, aqm_grp_execmask_hi.value);
    core_2_eid_val.value = 0;
    core_2_eid_val.ucode_blk = 2;
    core_2_eid_val.ucode_len = if ucode_size <= CNN55XX_UCD_BLOCK_SIZE { 1 } else { 0 };
    i = 0;
    while i < (*ndev).hw.ae_cores {
        nitrox_write_csr(ndev, UCD_AE_EID_UCODE_BLOCK_NUMX(i), core_2_eid_val.value);
        i += 1;
    }
    0
}

unsafe fn nitrox_add_to_devlist(ndev: *mut nitrox_device) -> i32 {
    let mut dev: *mut nitrox_device;
    let mut ret = 0;
    INIT_LIST_HEAD(&mut (*ndev).list);
    refcount_set(&mut (*ndev).refcnt, 1);
    mutex_lock(&mut DEVLIST_LOCK);
    list_for_each_entry!(dev, &mut NDEVLIST, list, {
        if dev == ndev { ret = -EEXIST; break; }
    });
    if ret == 0 {
        (*ndev).idx = NUM_DEVICES;
        NUM_DEVICES += 1;
        list_add_tail(&mut (*ndev).list, &mut NDEVLIST);
    }
    mutex_unlock(&mut DEVLIST_LOCK);
    ret
}

unsafe fn nitrox_remove_from_devlist(ndev: *mut nitrox_device) {
    mutex_lock(&mut DEVLIST_LOCK);
    list_del(&mut (*ndev).list);
    NUM_DEVICES -= 1;
    mutex_unlock(&mut DEVLIST_LOCK);
}

unsafe fn nitrox_get_first_device() -> *mut nitrox_device {
    let mut ndev = core::ptr::null_mut();
    let mut iter: *mut nitrox_device;
    mutex_lock(&mut DEVLIST_LOCK);
    list_for_each_entry!(iter, &mut NDEVLIST, list, {
        if nitrox_ready(iter) { ndev = iter; break; }
    });
    mutex_unlock(&mut DEVLIST_LOCK);
    if ndev.is_null() { return core::ptr::null_mut(); }
    refcount_inc(&mut (*ndev).refcnt);
    smp_mb__after_atomic();
    ndev
}

unsafe fn nitrox_put_device(ndev: *mut nitrox_device) {
    if !ndev.is_null() { refcount_dec(&mut (*ndev).refcnt); smp_mb__after_atomic(); }
}

unsafe fn nitrox_device_flr(pdev: *mut pci_dev) -> i32 {
    let pos = pci_save_state(pdev);
    if pos != 0 { dev_err(&mut (*pdev).dev, c_str!("Failed to save pci state\n")); return -ENOMEM; }
    pcie_reset_flr(pdev, PCI_RESET_DO_RESET);
    pci_restore_state(pdev);
    0
}

unsafe fn nitrox_pf_sw_init(ndev: *mut nitrox_device) -> i32 {
    let err = nitrox_common_sw_init(ndev);
    if err != 0 { return err; }
    let err = nitrox_register_interrupts(ndev);
    if err != 0 { nitrox_common_sw_cleanup(ndev); }
    err
}

unsafe fn nitrox_pf_sw_cleanup(ndev: *mut nitrox_device) { nitrox_unregister_interrupts(ndev); nitrox_common_sw_cleanup(ndev); }

unsafe fn nitrox_bist_check(ndev: *mut nitrox_device) -> i32 {
    let mut value = 0u64;
    for i in 0..NR_CLUSTERS { value += nitrox_read_csr(ndev, EMU_BIST_STATUSX(i)); value += nitrox_read_csr(ndev, EFL_CORE_BIST_REGX(i)); }
    value += nitrox_read_csr(ndev, UCD_BIST_STATUS); value += nitrox_read_csr(ndev, NPS_CORE_BIST_REG); value += nitrox_read_csr(ndev, NPS_CORE_NPC_BIST_REG);
    value += nitrox_read_csr(ndev, NPS_PKT_SLC_BIST_REG); value += nitrox_read_csr(ndev, NPS_PKT_IN_BIST_REG); value += nitrox_read_csr(ndev, POM_BIST_REG);
    value += nitrox_read_csr(ndev, BMI_BIST_REG); value += nitrox_read_csr(ndev, EFL_TOP_BIST_STAT); value += nitrox_read_csr(ndev, BMO_BIST_REG);
    value += nitrox_read_csr(ndev, LBC_BIST_STATUS); value += nitrox_read_csr(ndev, PEM_BIST_STATUSX(0));
    if value != 0 { -EIO } else { 0 }
}

unsafe fn nitrox_pf_hw_init(ndev: *mut nitrox_device) -> i32 {
    let err = nitrox_bist_check(ndev); if err != 0 { dev_err(&mut (*(*ndev).pdev).dev, c_str!("BIST check failed\n")); return err; }
    nitrox_get_hwinfo(ndev); nitrox_config_nps_core_unit(ndev); nitrox_config_aqm_unit(ndev); nitrox_config_nps_pkt_unit(ndev); nitrox_config_pom_unit(ndev);
    nitrox_config_efl_unit(ndev); nitrox_config_bmi_unit(ndev); nitrox_config_bmo_unit(ndev); nitrox_config_lbc_unit(ndev); nitrox_config_rand_unit(ndev);
    let err = nitrox_load_fw(ndev); if err != 0 { return err; } nitrox_config_emu_unit(ndev); 0
}

// PCI probe/remove/shutdown and module registration retain their C driver
// interfaces; dependent kernel bindings provide the referenced types/macros.
unsafe fn nitrox_probe(pdev: *mut pci_dev, _id: *const pci_device_id) -> i32 {
    dev_info_once(&mut (*pdev).dev, c_str!("%s driver version %s\n"), NITROX_DRIVER_NAME.as_ptr(), DRIVER_VERSION.as_ptr());
    let mut err = pci_enable_device_mem(pdev); if err != 0 { return err; }
    err = nitrox_device_flr(pdev); if err != 0 { pci_disable_device(pdev); return err; }
    if dma_set_mask_and_coherent(&mut (*pdev).dev, DMA_BIT_MASK(64)) != 0 {
        err = dma_set_mask_and_coherent(&mut (*pdev).dev, DMA_BIT_MASK(32)); if err != 0 { pci_disable_device(pdev); return err; }
    }
    err = pci_request_mem_regions(pdev, NITROX_DRIVER_NAME.as_ptr()); if err != 0 { pci_disable_device(pdev); return err; }
    pci_set_master(pdev);
    let ndev = kzalloc_obj::<nitrox_device>(); if ndev.is_null() { pci_release_mem_regions(pdev); pci_disable_device(pdev); return -ENOMEM; }
    pci_set_drvdata(pdev, ndev); (*ndev).pdev = pdev; nitrox_add_to_devlist(ndev);
    (*ndev).hw.vendor_id = (*pdev).vendor; (*ndev).hw.device_id = (*pdev).device; (*ndev).hw.revision_id = (*pdev).revision;
    (*ndev).timeout = msecs_to_jiffies(CMD_TIMEOUT); (*ndev).node = dev_to_node(&mut (*pdev).dev); if (*ndev).node == NUMA_NO_NODE { (*ndev).node = 0; }
    (*ndev).bar_addr = ioremap(pci_resource_start(pdev, 0), pci_resource_len(pdev, 0));
    if (*ndev).bar_addr.is_null() { nitrox_remove_from_devlist(ndev); kfree(ndev); pci_set_drvdata(pdev, core::ptr::null_mut()); pci_release_mem_regions(pdev); pci_disable_device(pdev); return -EIO; }
    (*ndev).nr_queues = min_t::<u32>(MAX_PF_QUEUES, num_online_cpus()); (*ndev).qlen = QLEN;
    err = nitrox_pf_sw_init(ndev); if err != 0 { iounmap((*ndev).bar_addr); nitrox_remove_from_devlist(ndev); kfree(ndev); pci_set_drvdata(pdev, core::ptr::null_mut()); pci_release_mem_regions(pdev); pci_disable_device(pdev); return err; }
    err = nitrox_pf_hw_init(ndev); if err != 0 { nitrox_pf_sw_cleanup(ndev); iounmap((*ndev).bar_addr); nitrox_remove_from_devlist(ndev); kfree(ndev); pci_set_drvdata(pdev, core::ptr::null_mut()); pci_release_mem_regions(pdev); pci_disable_device(pdev); return err; }
    nitrox_debugfs_init(ndev); atomic64_set(&mut (*ndev).stats.posted, 0); atomic64_set(&mut (*ndev).stats.completed, 0); atomic64_set(&mut (*ndev).stats.dropped, 0); atomic_set(&mut (*ndev).state, __NDEV_READY); smp_mb__after_atomic();
    err = nitrox_crypto_register(); if err != 0 { nitrox_debugfs_exit(ndev); atomic_set(&mut (*ndev).state, __NDEV_NOT_READY); smp_mb__after_atomic(); nitrox_pf_sw_cleanup(ndev); iounmap((*ndev).bar_addr); nitrox_remove_from_devlist(ndev); kfree(ndev); pci_set_drvdata(pdev, core::ptr::null_mut()); pci_release_mem_regions(pdev); pci_disable_device(pdev); return err; }
    0
}
unsafe fn nitrox_remove(pdev: *mut pci_dev) { let ndev = pci_get_drvdata(pdev); if ndev.is_null() { return; } if !refcount_dec_and_test(&mut (*ndev).refcnt) { return; } atomic_set(&mut (*ndev).state, __NDEV_NOT_READY); smp_mb__after_atomic(); nitrox_remove_from_devlist(ndev); nitrox_sriov_configure(pdev, 0); nitrox_crypto_unregister(); nitrox_debugfs_exit(ndev); nitrox_pf_sw_cleanup(ndev); iounmap((*ndev).bar_addr); kfree(ndev); pci_set_drvdata(pdev, core::ptr::null_mut()); pci_release_mem_regions(pdev); pci_disable_device(pdev); }
unsafe fn nitrox_shutdown(pdev: *mut pci_dev) { pci_set_drvdata(pdev, core::ptr::null_mut()); pci_release_mem_regions(pdev); pci_disable_device(pdev); }

// PCI ID table: { PCI_VDEVICE(CAVIUM, CNN55XX_DEV_ID) }, followed by the
// required terminating empty entry. Kernel-specific registration metadata is
// emitted by the surrounding module bindings.
static NITROX_PCI_TBL: [pci_device_id; 2] = [
    pci_device_id::vendor_device(PCI_VENDOR_ID_CAVIUM, CNN55XX_DEV_ID),
    pci_device_id::default(),
];

#[allow(dead_code)]
static NITROX_DRIVER: pci_driver = pci_driver {
    name: NITROX_DRIVER_NAME.as_ptr(),
    id_table: NITROX_PCI_TBL.as_ptr(),
    probe: Some(nitrox_probe),
    remove: Some(nitrox_remove),
    shutdown: Some(nitrox_shutdown),
    sriov_configure: Some(nitrox_sriov_configure),
};

// MODULE_DEVICE_TABLE(pci, nitrox_pci_tbl);
// module_pci_driver(nitrox_driver);
// MODULE_AUTHOR("Srikanth Jampala <Jampala.Srikanth@cavium.com>");
// MODULE_DESCRIPTION("Cavium CNN55XX PF Driver" DRIVER_VERSION " ");
// MODULE_LICENSE("GPL");
// MODULE_VERSION(DRIVER_VERSION);
// MODULE_FIRMWARE(SE_FW);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
