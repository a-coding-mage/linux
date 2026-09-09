// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2020 Marvell. */

// Dependencies supplied by the surrounding kernel/Rust translation.

pub unsafe fn otx2_cpt_mbox_bbuf_init(
    cptvf: *mut otx2_cptvf_dev,
    pdev: *mut pci_dev,
) -> i32 {
    let mdev: *mut otx2_mbox_dev;
    let otx2_mbox: *mut otx2_mbox;

    (*cptvf).bbuf_base = devm_kmalloc(&mut (*pdev).dev, MBOX_SIZE, GFP_KERNEL);
    if (*cptvf).bbuf_base.is_null() {
        return -ENOMEM;
    }
    /*
     * Overwrite mbox mbase to point to bounce buffer, so that PF/VF
     * prepare all mbox messages in bounce buffer instead of directly
     * in hw mbox memory.
     */
    otx2_mbox = &mut (*cptvf).pfvf_mbox;
    mdev = &mut (*otx2_mbox).dev[0];
    (*mdev).mbase = (*cptvf).bbuf_base;

    0
}

unsafe fn otx2_cpt_sync_mbox_bbuf(mbox: *mut otx2_mbox, devid: i32) {
    let msgs_offset: u16 = ALIGN(core::mem::size_of::<mbox_hdr>(), MBOX_MSG_ALIGN);
    let hw_mbase = (*mbox).hwbase.add((devid as usize) * MBOX_SIZE);
    let mdev = &mut (*mbox).dev[devid as usize];
    let hdr: *mut mbox_hdr;
    let mut msg_size: u64;

    if (*mdev).mbase == hw_mbase {
        return;
    }

    hdr = hw_mbase.add((*mbox).rx_start) as *mut mbox_hdr;
    msg_size = (*hdr).msg_size;

    if msg_size > (*mbox).rx_size - msgs_offset as u64 {
        msg_size = (*mbox).rx_size - msgs_offset as u64;
    }

    /* Copy mbox messages from mbox memory to bounce buffer */
    memcpy(
        (*mdev).mbase.add((*mbox).rx_start),
        hw_mbase.add((*mbox).rx_start),
        msg_size as usize + msgs_offset as usize,
    );
}

pub unsafe extern "C" fn otx2_cptvf_pfvf_mbox_intr(
    _irq: i32,
    arg: *mut core::ffi::c_void,
) -> irqreturn_t {
    let cptvf = arg as *mut otx2_cptvf_dev;
    let intr: u64;

    /* Read the interrupt bits */
    intr = otx2_cpt_read64((*cptvf).reg_base, BLKADDR_RVUM, 0, OTX2_RVU_VF_INT);

    if intr & 0x1u64 != 0 {
        /* Schedule work queue function to process the MBOX request */
        queue_work((*cptvf).pfvf_mbox_wq, &mut (*cptvf).pfvf_mbox_work);
        /* Clear and ack the interrupt */
        otx2_cpt_write64((*cptvf).reg_base, BLKADDR_RVUM, 0,
                         OTX2_RVU_VF_INT, 0x1u64);
    }
    IRQ_HANDLED
}

unsafe fn process_pfvf_mbox_mbox_msg(cptvf: *mut otx2_cptvf_dev,
                                     msg: *mut mbox_msghdr) {
    let lfs = &mut (*cptvf).lfs;
    let rsp_limits: *mut otx2_cpt_kvf_limits_rsp;
    let rsp_grp: *mut otx2_cpt_egrp_num_rsp;
    let eng_caps: *mut otx2_cpt_caps_rsp;
    let rsp_reg: *mut cpt_rd_wr_reg_msg;
    let rsp_msix: *mut msix_offset_rsp;
    let mut grp_num: u8;
    let mut i: i32;

    if (*msg).id >= MBOX_MSG_MAX {
        dev_err(&(*cptvf).pdev.as_ref().unwrap().dev,
                "MBOX msg with unknown ID %d\n", (*msg).id);
        return;
    }
    if (*msg).sig != OTX2_MBOX_RSP_SIG {
        dev_err(&(*cptvf).pdev.as_ref().unwrap().dev,
                "MBOX msg with wrong signature %x, ID %d\n", (*msg).sig, (*msg).id);
        return;
    }
    match (*msg).id {
        MBOX_MSG_READY => {
            (*cptvf).vf_id = (((*msg).pcifunc >> RVU_PFVF_FUNC_SHIFT) & RVU_PFVF_FUNC_MASK) - 1;
        }
        MBOX_MSG_ATTACH_RESOURCES => { if (*msg).rc == 0 { (*lfs).are_lfs_attached = 1; } }
        MBOX_MSG_DETACH_RESOURCES => { if (*msg).rc == 0 { (*lfs).are_lfs_attached = 0; } }
        MBOX_MSG_MSIX_OFFSET => {
            rsp_msix = msg as *mut msix_offset_rsp;
            i = 0;
            while i < (*rsp_msix).cptlfs as i32 {
                (*lfs).lf[i as usize].msix_offset = (*rsp_msix).cptlf_msixoff[i as usize];
                i += 1;
            }
        }
        MBOX_MSG_CPT_RD_WR_REGISTER => {
            rsp_reg = msg as *mut cpt_rd_wr_reg_msg;
            if (*msg).rc != 0 {
                dev_err(&(*cptvf).pdev.as_ref().unwrap().dev,
                        "Reg %llx rd/wr(%d) failed %d\n", (*rsp_reg).reg_offset,
                        (*rsp_reg).is_write, (*msg).rc);
                return;
            }
            if (*rsp_reg).is_write == 0 { *(*rsp_reg).ret_val = (*rsp_reg).val; }
        }
        MBOX_MSG_GET_ENG_GRP_NUM => {
            rsp_grp = msg as *mut otx2_cpt_egrp_num_rsp;
            grp_num = (*rsp_grp).eng_grp_num;
            if (*rsp_grp).eng_type == OTX2_CPT_SE_TYPES { (*lfs).kcrypto_se_eng_grp_num = grp_num; }
            else if (*rsp_grp).eng_type == OTX2_CPT_AE_TYPES { (*lfs).kcrypto_ae_eng_grp_num = grp_num; }
        }
        MBOX_MSG_GET_KVF_LIMITS => { rsp_limits = msg as *mut otx2_cpt_kvf_limits_rsp; (*lfs).kvf_limits = (*rsp_limits).kvf_limits; }
        MBOX_MSG_GET_CAPS => { eng_caps = msg as *mut otx2_cpt_caps_rsp; memcpy((*cptvf).eng_caps, (*eng_caps).eng_caps, core::mem::size_of_val(&(*cptvf).eng_caps)); }
        MBOX_MSG_CPT_LF_RESET | MBOX_MSG_LMTST_TBL_SETUP => {}
        _ => { dev_err(&(*cptvf).pdev.as_ref().unwrap().dev, "Unsupported msg %d received.\n", (*msg).id); }
    }
}

pub unsafe fn otx2_cptvf_pfvf_mbox_handler(work: *mut work_struct) {
    let cptvf: *mut otx2_cptvf_dev;
    let pfvf_mbox: *mut otx2_mbox;
    let mdev: *mut otx2_mbox_dev;
    let rsp_hdr: *mut mbox_hdr;
    let msg: *mut mbox_msghdr;
    let mut offset: usize;
    let mut i: i32;

    /* sync with mbox memory region */
    smp_rmb();
    cptvf = container_of(work, otx2_cptvf_dev, pfvf_mbox_work);
    pfvf_mbox = &mut (*cptvf).pfvf_mbox;
    otx2_cpt_sync_mbox_bbuf(pfvf_mbox, 0);
    mdev = &mut (*pfvf_mbox).dev[0];
    rsp_hdr = (*mdev).mbase.add((*pfvf_mbox).rx_start) as *mut mbox_hdr;
    if (*rsp_hdr).num_msgs == 0 { return; }
    offset = ALIGN(core::mem::size_of::<mbox_hdr>(), MBOX_MSG_ALIGN);
    i = 0;
    while i < (*rsp_hdr).num_msgs as i32 {
        msg = (*mdev).mbase.add((*pfvf_mbox).rx_start + offset) as *mut mbox_msghdr;
        process_pfvf_mbox_mbox_msg(cptvf, msg);
        offset = (*msg).next_msgoff as usize;
        (*mdev).msgs_acked += 1;
        i += 1;
    }
    otx2_mbox_reset(pfvf_mbox, 0);
}

pub unsafe fn otx2_cptvf_send_eng_grp_num_msg(cptvf: *mut otx2_cptvf_dev, eng_type: i32) -> i32 {
    let mbox = &mut (*cptvf).pfvf_mbox;
    let pdev = (*cptvf).pdev;
    let req = otx2_mbox_alloc_msg_rsp(mbox, 0, core::mem::size_of::<otx2_cpt_egrp_num_msg>(), core::mem::size_of::<otx2_cpt_egrp_num_rsp>()) as *mut otx2_cpt_egrp_num_msg;
    if req.is_null() { dev_err(&(*pdev).dev, "RVU MBOX failed to get message.\n"); return -EFAULT; }
    (*req).hdr.id = MBOX_MSG_GET_ENG_GRP_NUM; (*req).hdr.sig = OTX2_MBOX_REQ_SIG;
    (*req).hdr.pcifunc = OTX2_CPT_RVU_PFFUNC(pdev, (*cptvf).vf_id, 0); (*req).eng_type = eng_type;
    otx2_cpt_send_mbox_msg(mbox, pdev)
}

pub unsafe fn otx2_cptvf_send_kvf_limits_msg(cptvf: *mut otx2_cptvf_dev) -> i32 {
    let mbox = &mut (*cptvf).pfvf_mbox; let pdev = (*cptvf).pdev;
    let req = otx2_mbox_alloc_msg_rsp(mbox, 0, core::mem::size_of::<mbox_msghdr>(), core::mem::size_of::<otx2_cpt_kvf_limits_rsp>()) as *mut mbox_msghdr;
    if req.is_null() { dev_err(&(*pdev).dev, "RVU MBOX failed to get message.\n"); return -EFAULT; }
    (*req).id = MBOX_MSG_GET_KVF_LIMITS; (*req).sig = OTX2_MBOX_REQ_SIG; (*req).pcifunc = OTX2_CPT_RVU_PFFUNC(pdev, (*cptvf).vf_id, 0);
    otx2_cpt_send_mbox_msg(mbox, pdev)
}

pub unsafe fn otx2_cptvf_send_caps_msg(cptvf: *mut otx2_cptvf_dev) -> i32 {
    let mbox = &mut (*cptvf).pfvf_mbox; let pdev = (*cptvf).pdev;
    let req = otx2_mbox_alloc_msg_rsp(mbox, 0, core::mem::size_of::<mbox_msghdr>(), core::mem::size_of::<otx2_cpt_caps_rsp>()) as *mut mbox_msghdr;
    if req.is_null() { dev_err(&(*pdev).dev, "RVU MBOX failed to get message.\n"); return -EFAULT; }
    (*req).id = MBOX_MSG_GET_CAPS; (*req).sig = OTX2_MBOX_REQ_SIG; (*req).pcifunc = OTX2_CPT_RVU_PFFUNC(pdev, (*cptvf).vf_id, 0);
    otx2_cpt_send_mbox_msg(mbox, pdev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
