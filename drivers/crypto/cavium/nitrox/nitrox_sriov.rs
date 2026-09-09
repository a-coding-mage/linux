// SPDX-License-Identifier: GPL-2.0
// Translated from nitrox_sriov.c. Kernel and driver dependencies are supplied externally.

use core::ffi::c_int;

// Linux/kernel headers and driver headers omitted; their declarations are external dependencies.

#[inline]
fn num_vfs_valid(num_vfs: c_int) -> bool {
    matches!(num_vfs, 16 | 32 | 64 | 128)
}

#[inline]
fn num_vfs_to_mode(num_vfs: c_int) -> vf_mode {
    match num_vfs {
        0 => __NDEV_MODE_PF,
        16 => __NDEV_MODE_VF16,
        32 => __NDEV_MODE_VF32,
        64 => __NDEV_MODE_VF64,
        128 => __NDEV_MODE_VF128,
        _ => 0,
    }
}

#[inline]
fn vf_mode_to_nr_queues(mode: vf_mode) -> c_int {
    match mode {
        __NDEV_MODE_PF => MAX_PF_QUEUES,
        __NDEV_MODE_VF16 => 8,
        __NDEV_MODE_VF32 => 4,
        __NDEV_MODE_VF64 => 2,
        __NDEV_MODE_VF128 => 1,
        _ => 0,
    }
}

unsafe fn nitrox_pf_cleanup(ndev: *mut nitrox_device) {
    // PF has no queues in SR-IOV mode
    atomic_set(&mut (*ndev).state, __NDEV_NOT_READY);
    // unregister crypto algorithms
    nitrox_crypto_unregister();

    // cleanup PF resources
    nitrox_unregister_interrupts(ndev);
    nitrox_common_sw_cleanup(ndev);
}

unsafe fn nitrox_pf_reinit(ndev: *mut nitrox_device) -> c_int {
    // allocate resources for PF
    let mut err = nitrox_common_sw_init(ndev);
    if err != 0 {
        return err;
    }

    err = nitrox_register_interrupts(ndev);
    if err != 0 {
        nitrox_common_sw_cleanup(ndev);
        return err;
    }

    // configure the AQM queues
    nitrox_config_aqm_rings(ndev);

    // configure the packet queues
    nitrox_config_pkt_input_rings(ndev);
    nitrox_config_pkt_solicit_ports(ndev);

    // set device to ready state
    atomic_set(&mut (*ndev).state, __NDEV_READY);

    // register crypto algorithms
    nitrox_crypto_register()
}

unsafe fn nitrox_sriov_cleanup(ndev: *mut nitrox_device) {
    // unregister interrupts for PF in SR-IOV
    nitrox_sriov_unregister_interrupts(ndev);
    nitrox_mbox_cleanup(ndev);
}

unsafe fn nitrox_sriov_init(ndev: *mut nitrox_device) -> c_int {
    // register interrupts for PF in SR-IOV
    let mut ret = nitrox_sriov_register_interupts(ndev);
    if ret != 0 {
        return ret;
    }

    ret = nitrox_mbox_init(ndev);
    if ret != 0 {
        nitrox_sriov_cleanup(ndev);
        return ret;
    }

    0
}

unsafe fn nitrox_sriov_enable(pdev: *mut pci_dev, num_vfs: c_int) -> c_int {
    let ndev = pci_get_drvdata(pdev);

    if !num_vfs_valid(num_vfs) {
        dev_err(DEV(ndev), "Invalid num_vfs %d\n", num_vfs);
        return -EINVAL;
    }

    if pci_num_vf(pdev) == num_vfs {
        return num_vfs;
    }

    let mut err = pci_enable_sriov(pdev, num_vfs);
    if err != 0 {
        dev_err(DEV(ndev), "failed to enable PCI sriov %d\n", err);
        return err;
    }
    dev_info(DEV(ndev), "Enabled VF(s) %d\n", num_vfs);

    (*ndev).mode = num_vfs_to_mode(num_vfs);
    (*ndev).iov.num_vfs = num_vfs;
    (*ndev).iov.max_vf_queues = vf_mode_to_nr_queues((*ndev).mode);
    // set bit in flags
    set_bit(__NDEV_SRIOV_BIT, &mut (*ndev).flags);

    // cleanup PF resources
    nitrox_pf_cleanup(ndev);

    // PF SR-IOV mode initialization
    err = nitrox_sriov_init(ndev);
    if err != 0 {
        pci_disable_sriov(pdev);
        // clear bit in flags
        clear_bit(__NDEV_SRIOV_BIT, &mut (*ndev).flags);
        (*ndev).iov.num_vfs = 0;
        (*ndev).mode = __NDEV_MODE_PF;
        // reset back to working mode in PF
        nitrox_pf_reinit(ndev);
        return err;
    }

    config_nps_core_vfcfg_mode(ndev, (*ndev).mode);
    num_vfs
}

unsafe fn nitrox_sriov_disable(pdev: *mut pci_dev) -> c_int {
    let ndev = pci_get_drvdata(pdev);

    if !test_bit(__NDEV_SRIOV_BIT, &(*ndev).flags) {
        return 0;
    }

    if pci_vfs_assigned(pdev) {
        dev_warn(DEV(ndev), "VFs are attached to VM. Can't disable SR-IOV\n");
        return -EPERM;
    }
    pci_disable_sriov(pdev);
    // clear bit in flags
    clear_bit(__NDEV_SRIOV_BIT, &mut (*ndev).flags);

    (*ndev).iov.num_vfs = 0;
    (*ndev).iov.max_vf_queues = 0;
    (*ndev).mode = __NDEV_MODE_PF;

    // cleanup PF SR-IOV resources
    nitrox_sriov_cleanup(ndev);

    config_nps_core_vfcfg_mode(ndev, (*ndev).mode);

    nitrox_pf_reinit(ndev)
}

pub unsafe fn nitrox_sriov_configure(pdev: *mut pci_dev, num_vfs: c_int) -> c_int {
    if num_vfs == 0 {
        nitrox_sriov_disable(pdev)
    } else {
        nitrox_sriov_enable(pdev, num_vfs)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
