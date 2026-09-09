// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2020 Marvell. */

// Dependencies are supplied by the translated common and CPT LF headers.

pub unsafe fn otx2_cpt_send_mbox_msg(
    mbox: *mut otx2_mbox,
    pdev: *mut pci_dev,
) -> ::core::ffi::c_int {
    let ret: ::core::ffi::c_int;

    otx2_mbox_msg_send(mbox, 0);
    ret = otx2_mbox_wait_for_rsp(mbox, 0);
    if ret == -EIO {
        dev_err(&mut (*pdev).dev, "RVU MBOX timeout.\n");
        return ret;
    } else if ret != 0 {
        dev_err(&mut (*pdev).dev, "RVU MBOX error: %d.\n", ret);
        return -EFAULT;
    }
    ret
}

pub unsafe fn otx2_cpt_send_ready_msg(
    mbox: *mut otx2_mbox,
    pdev: *mut pci_dev,
) -> ::core::ffi::c_int {
    let req = otx2_mbox_alloc_msg_rsp(
        mbox,
        0,
        core::mem::size_of::<mbox_msghdr>(),
        core::mem::size_of::<ready_msg_rsp>(),
    ) as *mut mbox_msghdr;
    if req.is_null() {
        dev_err(&mut (*pdev).dev, "RVU MBOX failed to get message.\n");
        return -EFAULT;
    }
    (*req).id = MBOX_MSG_READY;
    (*req).sig = OTX2_MBOX_REQ_SIG;
    (*req).pcifunc = 0;

    otx2_cpt_send_mbox_msg(mbox, pdev)
}

pub unsafe fn otx2_cpt_send_af_reg_requests(
    mbox: *mut otx2_mbox,
    pdev: *mut pci_dev,
) -> ::core::ffi::c_int {
    otx2_cpt_send_mbox_msg(mbox, pdev)
}

unsafe fn otx2_cpt_add_read_af_reg(
    mbox: *mut otx2_mbox,
    pdev: *mut pci_dev,
    reg: u64,
    val: *mut u64,
    blkaddr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let reg_msg = otx2_mbox_alloc_msg_rsp(
        mbox, 0, core::mem::size_of::<cpt_rd_wr_reg_msg>(),
        core::mem::size_of::<cpt_rd_wr_reg_msg>(),
    ) as *mut cpt_rd_wr_reg_msg;
    if reg_msg.is_null() {
        dev_err(&mut (*pdev).dev, "RVU MBOX failed to get message.\n");
        return -EFAULT;
    }
    (*reg_msg).hdr.id = MBOX_MSG_CPT_RD_WR_REGISTER;
    (*reg_msg).hdr.sig = OTX2_MBOX_REQ_SIG;
    (*reg_msg).hdr.pcifunc = 0;
    (*reg_msg).is_write = 0;
    (*reg_msg).reg_offset = reg;
    (*reg_msg).ret_val = val;
    (*reg_msg).blkaddr = blkaddr;
    0
}

pub unsafe fn otx2_cpt_add_write_af_reg(
    mbox: *mut otx2_mbox, pdev: *mut pci_dev, reg: u64, val: u64,
    blkaddr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let reg_msg = otx2_mbox_alloc_msg_rsp(
        mbox, 0, core::mem::size_of::<cpt_rd_wr_reg_msg>(),
        core::mem::size_of::<cpt_rd_wr_reg_msg>(),
    ) as *mut cpt_rd_wr_reg_msg;
    if reg_msg.is_null() {
        dev_err(&mut (*pdev).dev, "RVU MBOX failed to get message.\n");
        return -EFAULT;
    }
    (*reg_msg).hdr.id = MBOX_MSG_CPT_RD_WR_REGISTER;
    (*reg_msg).hdr.sig = OTX2_MBOX_REQ_SIG;
    (*reg_msg).hdr.pcifunc = 0;
    (*reg_msg).is_write = 1;
    (*reg_msg).reg_offset = reg;
    (*reg_msg).val = val;
    (*reg_msg).blkaddr = blkaddr;
    0
}

pub unsafe fn otx2_cpt_read_af_reg(
    mbox: *mut otx2_mbox, pdev: *mut pci_dev, reg: u64, val: *mut u64,
    blkaddr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let ret = otx2_cpt_add_read_af_reg(mbox, pdev, reg, val, blkaddr);
    if ret != 0 { return ret; }
    otx2_cpt_send_mbox_msg(mbox, pdev)
}

pub unsafe fn otx2_cpt_write_af_reg(
    mbox: *mut otx2_mbox, pdev: *mut pci_dev, reg: u64, val: u64,
    blkaddr: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let ret = otx2_cpt_add_write_af_reg(mbox, pdev, reg, val, blkaddr);
    if ret != 0 { return ret; }
    otx2_cpt_send_mbox_msg(mbox, pdev)
}

pub unsafe fn otx2_cpt_attach_rscrs_msg(lfs: *mut otx2_cptlfs_info) -> ::core::ffi::c_int {
    let mbox = (*lfs).mbox;
    let req = otx2_mbox_alloc_msg_rsp(
        mbox, 0, core::mem::size_of::<rsrc_attach>(),
        core::mem::size_of::<msg_rsp>(),
    ) as *mut rsrc_attach;
    if req.is_null() {
        dev_err(&mut (*(*lfs).pdev).dev, "RVU MBOX failed to get message.\n");
        return -EFAULT;
    }
    (*req).hdr.id = MBOX_MSG_ATTACH_RESOURCES;
    (*req).hdr.sig = OTX2_MBOX_REQ_SIG;
    (*req).hdr.pcifunc = 0;
    (*req).cptlfs = (*lfs).lfs_num;
    (*req).cpt_blkaddr = (*lfs).blkaddr;
    (*req).modify = 1;
    let mut ret = otx2_cpt_send_mbox_msg(mbox, (*lfs).pdev);
    if ret != 0 { return ret; }
    if !(*lfs).are_lfs_attached { ret = -EINVAL; }
    ret
}

pub unsafe fn otx2_cpt_detach_rsrcs_msg(lfs: *mut otx2_cptlfs_info) -> ::core::ffi::c_int {
    let mbox = (*lfs).mbox;
    let req = otx2_mbox_alloc_msg_rsp(
        mbox, 0, core::mem::size_of::<rsrc_detach>(),
        core::mem::size_of::<msg_rsp>(),
    ) as *mut rsrc_detach;
    if req.is_null() {
        dev_err(&mut (*(*lfs).pdev).dev, "RVU MBOX failed to get message.\n");
        return -EFAULT;
    }
    (*req).hdr.id = MBOX_MSG_DETACH_RESOURCES;
    (*req).hdr.sig = OTX2_MBOX_REQ_SIG;
    (*req).hdr.pcifunc = 0;
    (*req).cptlfs = 1;
    let mut ret = otx2_cpt_send_mbox_msg(mbox, (*lfs).pdev);
    if ret != 0 { return ret; }
    if (*lfs).are_lfs_attached { ret = -EINVAL; }
    ret
}

pub unsafe fn otx2_cpt_msix_offset_msg(lfs: *mut otx2_cptlfs_info) -> ::core::ffi::c_int {
    let mbox = (*lfs).mbox;
    let pdev = (*lfs).pdev;
    let req = otx2_mbox_alloc_msg_rsp(
        mbox, 0, core::mem::size_of::<mbox_msghdr>(),
        core::mem::size_of::<msix_offset_rsp>(),
    ) as *mut mbox_msghdr;
    if req.is_null() {
        dev_err(&mut (*pdev).dev, "RVU MBOX failed to get message.\n");
        return -EFAULT;
    }
    (*req).id = MBOX_MSG_MSIX_OFFSET;
    (*req).sig = OTX2_MBOX_REQ_SIG;
    (*req).pcifunc = 0;
    let ret = otx2_cpt_send_mbox_msg(mbox, pdev);
    if ret != 0 { return ret; }
    let mut i = 0;
    while i < (*lfs).lfs_num {
        if (*lfs).lf[i as usize].msix_offset == MSIX_VECTOR_INVALID {
            dev_err(&mut (*pdev).dev, "Invalid msix offset %d for LF %d\n",
                    (*lfs).lf[i as usize].msix_offset, i);
            return -EINVAL;
        }
        i += 1;
    }
    ret
}

pub unsafe fn otx2_cpt_sync_mbox_msg(mbox: *mut otx2_mbox) -> ::core::ffi::c_int {
    if !otx2_mbox_nonempty(mbox, 0) { return 0; }
    otx2_mbox_msg_send(mbox, 0);
    let err = otx2_mbox_wait_for_rsp(mbox, 0);
    if err != 0 { return err; }
    otx2_mbox_check_rsp_msgs(mbox, 0)
}

pub unsafe fn otx2_cpt_lf_reset_msg(lfs: *mut otx2_cptlfs_info, slot: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mbox = (*lfs).mbox;
    let pdev = (*lfs).pdev;
    let req = otx2_mbox_alloc_msg_rsp(
        mbox, 0, core::mem::size_of::<cpt_lf_rst_req>(),
        core::mem::size_of::<msg_rsp>(),
    ) as *mut cpt_lf_rst_req;
    if req.is_null() {
        dev_err(&mut (*pdev).dev, "RVU MBOX failed to get message.\n");
        return -EFAULT;
    }
    (*req).hdr.id = MBOX_MSG_CPT_LF_RESET;
    (*req).hdr.sig = OTX2_MBOX_REQ_SIG;
    (*req).hdr.pcifunc = 0;
    (*req).slot = slot;
    otx2_cpt_send_mbox_msg(mbox, pdev)
}

pub unsafe fn otx2_cpt_lmtst_tbl_setup_msg(lfs: *mut otx2_cptlfs_info) -> ::core::ffi::c_int {
    let mbox = (*lfs).mbox;
    let pdev = (*lfs).pdev;
    let req = otx2_mbox_alloc_msg_rsp(
        mbox, 0, core::mem::size_of::<lmtst_tbl_setup_req>(),
        core::mem::size_of::<msg_rsp>(),
    ) as *mut lmtst_tbl_setup_req;
    if req.is_null() {
        dev_err(&mut (*pdev).dev, "RVU MBOX failed to alloc message.\n");
        return -EFAULT;
    }
    (*req).hdr.id = MBOX_MSG_LMTST_TBL_SETUP;
    (*req).hdr.sig = OTX2_MBOX_REQ_SIG;
    (*req).hdr.pcifunc = 0;
    (*req).use_local_lmt_region = true;
    (*req).lmt_iova = (*lfs).lmt_info.iova;
    otx2_cpt_send_mbox_msg(mbox, pdev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
