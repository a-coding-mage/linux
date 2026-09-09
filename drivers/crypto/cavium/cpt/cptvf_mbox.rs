// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Cavium, Inc.
 */

// Dependency declarations and constants are supplied by cptvf.h and other
// translation units in the surrounding kernel code.

unsafe fn cptvf_send_msg_to_pf(cptvf: *mut cpt_vf, mbx: *mut cpt_mbox) {
    /* Writing mbox(1) causes interrupt */
    cpt_write_csr64((*cptvf).reg_base, CPTX_VFX_PF_MBOXX(0, 0, 0), (*mbx).msg);
    cpt_write_csr64((*cptvf).reg_base, CPTX_VFX_PF_MBOXX(0, 0, 1), (*mbx).data);
}

/* Interrupt handler to handle mailbox messages from VFs */
#[no_mangle]
pub unsafe extern "C" fn cptvf_handle_mbox_intr(cptvf: *mut cpt_vf) {
    let mut mbx: cpt_mbox = core::mem::zeroed();

    /*
     * MBOX[0] contains msg
     * MBOX[1] contains data
     */
    mbx.msg = cpt_read_csr64((*cptvf).reg_base, CPTX_VFX_PF_MBOXX(0, 0, 0));
    mbx.data = cpt_read_csr64((*cptvf).reg_base, CPTX_VFX_PF_MBOXX(0, 0, 1));
    dev_dbg(&(*(*cptvf).pdev).dev, "%s: Mailbox msg 0x%llx from PF\n", "cptvf_handle_mbox_intr", mbx.msg);
    match mbx.msg {
        CPT_MSG_READY => {
            (*cptvf).pf_acked = true;
            (*cptvf).vfid = mbx.data;
            dev_dbg(&(*(*cptvf).pdev).dev, "Received VFID %d\n", (*cptvf).vfid);
        }
        CPT_MSG_QBIND_GRP => {
            (*cptvf).pf_acked = true;
            (*cptvf).vftype = mbx.data;
            dev_dbg(&(*(*cptvf).pdev).dev, "VF %d type %s group %d\n", (*cptvf).vfid,
                    if mbx.data == SE_TYPES { "SE" } else { "AE" }, (*cptvf).vfgrp);
        }
        CPT_MBOX_MSG_TYPE_ACK => (*cptvf).pf_acked = true,
        CPT_MBOX_MSG_TYPE_NACK => (*cptvf).pf_nacked = true,
        _ => dev_err(&(*(*cptvf).pdev).dev, "Invalid msg from PF, msg 0x%llx\n", mbx.msg),
    }
}

unsafe fn cptvf_send_msg_to_pf_timeout(cptvf: *mut cpt_vf, mbx: *mut cpt_mbox) -> i32 {
    let mut timeout = CPT_MBOX_MSG_TIMEOUT;
    let sleep = 10;

    (*cptvf).pf_acked = false;
    (*cptvf).pf_nacked = false;
    cptvf_send_msg_to_pf(cptvf, mbx);
    /* Wait for previous message to be acked, timeout 2sec */
    while !(*cptvf).pf_acked {
        if (*cptvf).pf_nacked { return -EINVAL; }
        msleep(sleep);
        if (*cptvf).pf_acked { break; }
        timeout -= sleep;
        if timeout == 0 {
            dev_err(&(*(*cptvf).pdev).dev, "PF didn't ack to mbox msg %llx from VF%u\n",
                    (*mbx).msg & 0xFF, (*cptvf).vfid);
            return -EBUSY;
        }
    }
    0
}

/*
 * Checks if VF is able to comminicate with PF
 * and also gets the CPT number this VF is associated to.
 */
#[no_mangle]
pub unsafe extern "C" fn cptvf_check_pf_ready(cptvf: *mut cpt_vf) -> i32 {
    let pdev = (*cptvf).pdev;
    let mut mbx: cpt_mbox = core::mem::zeroed();
    mbx.msg = CPT_MSG_READY;
    if cptvf_send_msg_to_pf_timeout(cptvf, &mut mbx) != 0 {
        dev_err(&(*pdev).dev, "PF didn't respond to READY msg\n");
        return -EBUSY;
    }
    0
}

/* Communicate VQs size to PF to program CPT(0)_PF_Q(0-15)_CTL of the VF. */
#[no_mangle]
pub unsafe extern "C" fn cptvf_send_vq_size_msg(cptvf: *mut cpt_vf) -> i32 {
    let pdev = (*cptvf).pdev; let mut mbx: cpt_mbox = core::mem::zeroed();
    mbx.msg = CPT_MSG_QLEN; mbx.data = (*cptvf).qsize;
    if cptvf_send_msg_to_pf_timeout(cptvf, &mut mbx) != 0 { dev_err(&(*pdev).dev, "PF didn't respond to vq_size msg\n"); return -EBUSY; } 0
}

/* Communicate VF group required to PF and get the VQ binded to that group */
#[no_mangle]
pub unsafe extern "C" fn cptvf_send_vf_to_grp_msg(cptvf: *mut cpt_vf) -> i32 {
    let pdev = (*cptvf).pdev; let mut mbx: cpt_mbox = core::mem::zeroed();
    mbx.msg = CPT_MSG_QBIND_GRP; mbx.data = (*cptvf).vfgrp;
    if cptvf_send_msg_to_pf_timeout(cptvf, &mut mbx) != 0 { dev_err(&(*pdev).dev, "PF didn't respond to vf_type msg\n"); return -EBUSY; } 0
}

#[no_mangle]
pub unsafe extern "C" fn cptvf_send_vf_priority_msg(cptvf: *mut cpt_vf) -> i32 {
    let pdev = (*cptvf).pdev; let mut mbx: cpt_mbox = core::mem::zeroed();
    mbx.msg = CPT_MSG_VQ_PRIORITY; mbx.data = (*cptvf).priority;
    if cptvf_send_msg_to_pf_timeout(cptvf, &mut mbx) != 0 { dev_err(&(*pdev).dev, "PF didn't respond to vf_type msg\n"); return -EBUSY; } 0
}

unsafe fn cptvf_send_simple(cptvf: *mut cpt_vf, msg: u64, text: *const core::ffi::c_char) -> i32 {
    let pdev = (*cptvf).pdev; let mut mbx: cpt_mbox = core::mem::zeroed(); mbx.msg = msg;
    if cptvf_send_msg_to_pf_timeout(cptvf, &mut mbx) != 0 { dev_err(&(*pdev).dev, text); return -EBUSY; } 0
}

#[no_mangle]
pub unsafe extern "C" fn cptvf_send_vf_up(cptvf: *mut cpt_vf) -> i32 { cptvf_send_simple(cptvf, CPT_MSG_VF_UP, "PF didn't respond to UP msg\0" as *const _ as *const _) }

#[no_mangle]
pub unsafe extern "C" fn cptvf_send_vf_down(cptvf: *mut cpt_vf) -> i32 { cptvf_send_simple(cptvf, CPT_MSG_VF_DOWN, "PF didn't respond to DOWN msg\0" as *const _ as *const _) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
