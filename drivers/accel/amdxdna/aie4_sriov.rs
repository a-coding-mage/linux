// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2026, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding kernel/Rust bindings.

unsafe fn aie4_destroy_vfs(ndev: *mut amdxdna_dev_hdl) -> i32 {
    // DECLARE_AIE_MSG(aie4_msg_destroy_vfs, AIE4_MSG_OP_DESTROY_VFS);
    let mut msg: aie4_msg_destroy_vfs = unsafe { core::mem::zeroed() };
    let ret = unsafe { aie_send_mgmt_msg_wait(&mut (*ndev).aie, &mut msg) };
    if ret != 0 {
        unsafe {
            XDNA_ERR((*ndev).aie.xdna, "destroy vfs op failed: %d", ret);
        }
    }
    ret
}

unsafe fn aie4_create_vfs(ndev: *mut amdxdna_dev_hdl, num_vfs: i32) -> i32 {
    // DECLARE_AIE_MSG(aie4_msg_create_vfs, AIE4_MSG_OP_CREATE_VFS);
    let mut msg: aie4_msg_create_vfs = unsafe { core::mem::zeroed() };
    unsafe {
        msg.req.vf_cnt = num_vfs;
    }
    let ret = unsafe { aie_send_mgmt_msg_wait(&mut (*ndev).aie, &mut msg) };
    if ret != 0 {
        unsafe {
            XDNA_ERR((*ndev).aie.xdna, "create vfs op failed: %d", ret);
        }
    }
    ret
}

pub unsafe fn aie4_sriov_stop(ndev: *mut amdxdna_dev_hdl) -> i32 {
    let xdna = unsafe { (*ndev).aie.xdna };
    let pdev = unsafe { to_pci_dev((*xdna).ddev.dev) };

    if unsafe { pci_num_vf(pdev) } == 0 {
        return 0;
    }

    let ret = unsafe { pci_vfs_assigned(pdev) };
    if ret != 0 {
        unsafe {
            XDNA_ERR(xdna, "VFs are still assigned to VMs");
        }
        return -EPERM;
    }

    unsafe { pci_disable_sriov(pdev) };
    unsafe { aie4_destroy_vfs(ndev) }
}

unsafe fn aie4_sriov_start(ndev: *mut amdxdna_dev_hdl, num_vfs: i32) -> i32 {
    let xdna = unsafe { (*ndev).aie.xdna };
    let pdev = unsafe { to_pci_dev((*xdna).ddev.dev) };

    let ret = unsafe { aie4_create_vfs(ndev, num_vfs) };
    if ret != 0 {
        return ret;
    }

    let ret = unsafe { pci_enable_sriov(pdev, num_vfs) };
    if ret != 0 {
        unsafe {
            XDNA_ERR(xdna, "configure VFs failed, ret: %d", ret);
            aie4_destroy_vfs(ndev);
        }
        return ret;
    }

    num_vfs
}

pub unsafe fn aie4_sriov_configure(xdna: *mut amdxdna_dev, num_vfs: i32) -> i32 {
    let ndev = unsafe { (*xdna).dev_handle };

    unsafe {
        drm_WARN_ON(&mut (*xdna).ddev, !mutex_is_locked(&(*xdna).dev_lock));
    }

    if num_vfs != 0 {
        unsafe { aie4_sriov_start(ndev, num_vfs) }
    } else {
        unsafe { aie4_sriov_stop(ndev) }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
