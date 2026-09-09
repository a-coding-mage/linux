// SPDX-License-Identifier: GPL-2.0
/* Marvell OcteonTX CPT driver
 *
 * Copyright (C) 2019 Marvell International Ltd.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

// linux/delay.h and otx_cptvf.h provide the declarations used below.

const CPT_MBOX_MSG_TIMEOUT: i32 = 2000;

unsafe fn get_mbox_opcode_str(msg_opcode: i32) -> *const u8 {
    let mut str_: *const u8 = b"Unknown\0".as_ptr();

    match msg_opcode {
        OTX_CPT_MSG_VF_UP => str_ = b"UP\0".as_ptr(),
        OTX_CPT_MSG_VF_DOWN => str_ = b"DOWN\0".as_ptr(),
        OTX_CPT_MSG_READY => str_ = b"READY\0".as_ptr(),
        OTX_CPT_MSG_QLEN => str_ = b"QLEN\0".as_ptr(),
        OTX_CPT_MSG_QBIND_GRP => str_ = b"QBIND_GRP\0".as_ptr(),
        OTX_CPT_MSG_VQ_PRIORITY => str_ = b"VQ_PRIORITY\0".as_ptr(),
        OTX_CPT_MSG_PF_TYPE => str_ = b"PF_TYPE\0".as_ptr(),
        OTX_CPT_MSG_ACK => str_ = b"ACK\0".as_ptr(),
        OTX_CPT_MSG_NACK => str_ = b"NACK\0".as_ptr(),
        _ => {}
    }
    str_
}

unsafe fn dump_mbox_msg(mbox_msg: *mut otx_cpt_mbox, vf_id: i32) {
    let mut raw_data_str = [0u8; OTX_CPT_MAX_MBOX_DATA_STR_SIZE as usize];

    hex_dump_to_buffer(
        mbox_msg as *const _,
        core::mem::size_of::<otx_cpt_mbox>(),
        16,
        8,
        raw_data_str.as_mut_ptr(),
        OTX_CPT_MAX_MBOX_DATA_STR_SIZE as usize,
        false,
    );
    if vf_id >= 0 {
        pr_debug_mbox_vf(get_mbox_opcode_str((*mbox_msg).msg), vf_id, raw_data_str.as_ptr());
    } else {
        pr_debug_mbox_pf(get_mbox_opcode_str((*mbox_msg).msg), raw_data_str.as_ptr());
    }
}

unsafe fn cptvf_send_msg_to_pf(cptvf: *mut otx_cptvf, mbx: *mut otx_cpt_mbox) {
    /* Writing mbox(1) causes interrupt */
    writeq((*mbx).msg, (*cptvf).reg_base + OTX_CPT_VFX_PF_MBOXX(0, 0));
    writeq((*mbx).data, (*cptvf).reg_base + OTX_CPT_VFX_PF_MBOXX(0, 1));
}

/* Interrupt handler to handle mailbox messages from VFs */
pub unsafe fn otx_cptvf_handle_mbox_intr(cptvf: *mut otx_cptvf) {
    let mut mbx: otx_cpt_mbox = core::mem::zeroed();

    /*
     * MBOX[0] contains msg
     * MBOX[1] contains data
     */
    mbx.msg = readq((*cptvf).reg_base + OTX_CPT_VFX_PF_MBOXX(0, 0));
    mbx.data = readq((*cptvf).reg_base + OTX_CPT_VFX_PF_MBOXX(0, 1));

    dump_mbox_msg(&mut mbx, -1);

    match mbx.msg {
        OTX_CPT_MSG_VF_UP => {
            (*cptvf).pf_acked = true;
            (*cptvf).num_vfs = mbx.data;
        }
        OTX_CPT_MSG_READY => {
            (*cptvf).pf_acked = true;
            (*cptvf).vfid = mbx.data;
            dev_dbg_vfid((*cptvf).vfid);
        }
        OTX_CPT_MSG_QBIND_GRP => {
            (*cptvf).pf_acked = true;
            (*cptvf).vftype = mbx.data;
            dev_dbg_vf_type_group(
                (*cptvf).vfid,
                if mbx.data == OTX_CPT_SE_TYPES { b"SE\0".as_ptr() } else { b"AE\0".as_ptr() },
                (*cptvf).vfgrp,
            );
        }
        OTX_CPT_MSG_ACK => (*cptvf).pf_acked = true,
        OTX_CPT_MSG_NACK => (*cptvf).pf_nacked = true,
        _ => dev_err_invalid_msg(mbx.msg),
    }
}

unsafe fn cptvf_send_msg_to_pf_timeout(cptvf: *mut otx_cptvf, mbx: *mut otx_cpt_mbox) -> i32 {
    let mut timeout = CPT_MBOX_MSG_TIMEOUT;
    let sleep = 10;

    (*cptvf).pf_acked = false;
    (*cptvf).pf_nacked = false;
    cptvf_send_msg_to_pf(cptvf, mbx);
    /* Wait for previous message to be acked, timeout 2sec */
    while !(*cptvf).pf_acked {
        if (*cptvf).pf_nacked {
            return -EINVAL;
        }
        msleep(sleep as u64);
        if (*cptvf).pf_acked {
            break;
        }
        timeout -= sleep;
        if timeout == 0 {
            dev_err_no_ack((*mbx).msg, (*cptvf).vfid);
            return -EBUSY;
        }
    }
    0
}

/*
 * Checks if VF is able to comminicate with PF
 * and also gets the CPT number this VF is associated to.
 */
pub unsafe fn otx_cptvf_check_pf_ready(cptvf: *mut otx_cptvf) -> i32 {
    let mut mbx: otx_cpt_mbox = core::mem::zeroed();
    mbx.msg = OTX_CPT_MSG_READY;
    cptvf_send_msg_to_pf_timeout(cptvf, &mut mbx)
}

/*
 * Communicate VQs size to PF to program CPT(0)_PF_Q(0-15)_CTL of the VF.
 * Must be ACKed.
 */
pub unsafe fn otx_cptvf_send_vq_size_msg(cptvf: *mut otx_cptvf) -> i32 {
    let mut mbx: otx_cpt_mbox = core::mem::zeroed();
    mbx.msg = OTX_CPT_MSG_QLEN;
    mbx.data = (*cptvf).qsize;
    cptvf_send_msg_to_pf_timeout(cptvf, &mut mbx)
}

/* Communicate VF group required to PF and get the VQ binded to that group */
pub unsafe fn otx_cptvf_send_vf_to_grp_msg(cptvf: *mut otx_cptvf, group: i32) -> i32 {
    let mut mbx: otx_cpt_mbox = core::mem::zeroed();
    mbx.msg = OTX_CPT_MSG_QBIND_GRP;
    /* Convey group of the VF */
    mbx.data = group as _;
    let ret = cptvf_send_msg_to_pf_timeout(cptvf, &mut mbx);
    if ret != 0 { return ret; }
    (*cptvf).vfgrp = group as _;
    0
}

/* Communicate VF group required to PF and get the VQ binded to that group */
pub unsafe fn otx_cptvf_send_vf_priority_msg(cptvf: *mut otx_cptvf) -> i32 {
    let mut mbx: otx_cpt_mbox = core::mem::zeroed();
    mbx.msg = OTX_CPT_MSG_VQ_PRIORITY;
    /* Convey group of the VF */
    mbx.data = (*cptvf).priority;
    cptvf_send_msg_to_pf_timeout(cptvf, &mut mbx)
}

/* Communicate to PF that VF is UP and running */
pub unsafe fn otx_cptvf_send_vf_up(cptvf: *mut otx_cptvf) -> i32 {
    let mut mbx: otx_cpt_mbox = core::mem::zeroed();
    mbx.msg = OTX_CPT_MSG_VF_UP;
    cptvf_send_msg_to_pf_timeout(cptvf, &mut mbx)
}

/* Communicate to PF that VF is DOWN and running */
pub unsafe fn otx_cptvf_send_vf_down(cptvf: *mut otx_cptvf) -> i32 {
    let mut mbx: otx_cpt_mbox = core::mem::zeroed();
    mbx.msg = OTX_CPT_MSG_VF_DOWN;
    cptvf_send_msg_to_pf_timeout(cptvf, &mut mbx)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
