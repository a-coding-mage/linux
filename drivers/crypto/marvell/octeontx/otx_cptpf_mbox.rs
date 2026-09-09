// SPDX-License-Identifier: GPL-2.0
/* Marvell OcteonTX CPT driver
 *
 * Copyright (C) 2019 Marvell International Ltd.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */

use core::ffi::{c_char, c_int};

unsafe fn get_mbox_opcode_str(msg_opcode: c_int) -> *mut c_char {
    let mut str_: *mut c_char = "Unknown\0".as_ptr() as *mut c_char;

    match msg_opcode {
        OTX_CPT_MSG_VF_UP => str_ = "UP\0".as_ptr() as *mut c_char,
        OTX_CPT_MSG_VF_DOWN => str_ = "DOWN\0".as_ptr() as *mut c_char,
        OTX_CPT_MSG_READY => str_ = "READY\0".as_ptr() as *mut c_char,
        OTX_CPT_MSG_QLEN => str_ = "QLEN\0".as_ptr() as *mut c_char,
        OTX_CPT_MSG_QBIND_GRP => str_ = "QBIND_GRP\0".as_ptr() as *mut c_char,
        OTX_CPT_MSG_VQ_PRIORITY => str_ = "VQ_PRIORITY\0".as_ptr() as *mut c_char,
        OTX_CPT_MSG_PF_TYPE => str_ = "PF_TYPE\0".as_ptr() as *mut c_char,
        OTX_CPT_MSG_ACK => str_ = "ACK\0".as_ptr() as *mut c_char,
        OTX_CPT_MSG_NACK => str_ = "NACK\0".as_ptr() as *mut c_char,
        _ => {}
    }
    str_
}

unsafe fn dump_mbox_msg(mbox_msg: *mut otx_cpt_mbox, vf_id: c_int) {
    let mut raw_data_str = [0 as c_char; OTX_CPT_MAX_MBOX_DATA_STR_SIZE as usize];
    hex_dump_to_buffer(
        mbox_msg as *const _,
        core::mem::size_of::<otx_cpt_mbox>(),
        16,
        8,
        raw_data_str.as_mut_ptr(),
        OTX_CPT_MAX_MBOX_DATA_STR_SIZE,
        false,
    );
    if vf_id >= 0 {
        pr_debug!("MBOX opcode %s received from VF%d raw_data %s\n", get_mbox_opcode_str((*mbox_msg).msg), vf_id, raw_data_str.as_ptr());
    } else {
        pr_debug!("MBOX opcode %s received from PF raw_data %s\n", get_mbox_opcode_str((*mbox_msg).msg), raw_data_str.as_ptr());
    }
}

unsafe fn otx_cpt_send_msg_to_vf(cpt: *mut otx_cpt_device, vf: c_int, mbx: *mut otx_cpt_mbox) {
    writeq((*mbx).data, (*cpt).reg_base + OTX_CPT_PF_VFX_MBOXX(vf, 1));
    writeq((*mbx).msg, (*cpt).reg_base + OTX_CPT_PF_VFX_MBOXX(vf, 0));
}

unsafe fn otx_cpt_mbox_send_ack(cpt: *mut otx_cpt_device, vf: c_int, mbx: *mut otx_cpt_mbox) {
    (*mbx).data = 0;
    (*mbx).msg = OTX_CPT_MSG_ACK;
    otx_cpt_send_msg_to_vf(cpt, vf, mbx);
}

unsafe fn otx_cptpf_mbox_send_nack(cpt: *mut otx_cpt_device, vf: c_int, mbx: *mut otx_cpt_mbox) {
    (*mbx).data = 0;
    (*mbx).msg = OTX_CPT_MSG_NACK;
    otx_cpt_send_msg_to_vf(cpt, vf, mbx);
}

unsafe fn otx_cpt_clear_mbox_intr(cpt: *mut otx_cpt_device, vf: u32) {
    writeq(1u64 << vf, (*cpt).reg_base + OTX_CPT_PF_MBOX_INTX(0));
}

unsafe fn otx_cpt_cfg_qlen_for_vf(cpt: *mut otx_cpt_device, vf: c_int, size: u32) {
    let mut pf_qx_ctl: otx_cptx_pf_qx_ctl = core::mem::zeroed();
    pf_qx_ctl.u = readq((*cpt).reg_base + OTX_CPT_PF_QX_CTL(vf));
    pf_qx_ctl.s.size = size;
    pf_qx_ctl.s.cont_err = true;
    writeq(pf_qx_ctl.u, (*cpt).reg_base + OTX_CPT_PF_QX_CTL(vf));
}

unsafe fn otx_cpt_cfg_vq_priority(cpt: *mut otx_cpt_device, vf: c_int, pri: u32) {
    let mut pf_qx_ctl: otx_cptx_pf_qx_ctl = core::mem::zeroed();
    pf_qx_ctl.u = readq((*cpt).reg_base + OTX_CPT_PF_QX_CTL(vf));
    pf_qx_ctl.s.pri = pri;
    writeq(pf_qx_ctl.u, (*cpt).reg_base + OTX_CPT_PF_QX_CTL(vf));
}

unsafe fn otx_cpt_bind_vq_to_grp(cpt: *mut otx_cpt_device, q: u8, grp: u8) -> c_int {
    let dev = &(*(*cpt).pdev).dev;
    if q >= (*cpt).max_vfs {
        dev_err!(dev, "Requested queue %d is > than maximum avail %d\n", q, (*cpt).max_vfs);
        return -EINVAL;
    }
    if grp >= OTX_CPT_MAX_ENGINE_GROUPS {
        dev_err!(dev, "Requested group %d is > than maximum avail %d\n", grp, OTX_CPT_MAX_ENGINE_GROUPS);
        return -EINVAL;
    }
    let eng_grp = &mut (*cpt).eng_grps.grp[grp as usize];
    if !eng_grp.is_enabled {
        dev_err!(dev, "Requested engine group %d is disabled\n", grp);
        return -EINVAL;
    }
    let mut pf_qx_ctl: otx_cptx_pf_qx_ctl = core::mem::zeroed();
    pf_qx_ctl.u = readq((*cpt).reg_base + OTX_CPT_PF_QX_CTL(q as c_int));
    pf_qx_ctl.s.grp = grp;
    writeq(pf_qx_ctl.u, (*cpt).reg_base + OTX_CPT_PF_QX_CTL(q as c_int));
    let ucode = if eng_grp.mirror.is_ena {
        &eng_grp.g.grp[eng_grp.mirror.idx as usize].ucode[0]
    } else {
        &eng_grp.ucode[0]
    };
    if otx_cpt_uc_supports_eng_type(ucode, OTX_CPT_SE_TYPES) { OTX_CPT_SE_TYPES }
    else if otx_cpt_uc_supports_eng_type(ucode, OTX_CPT_AE_TYPES) { OTX_CPT_AE_TYPES }
    else { BAD_OTX_CPTVF_TYPE }
}

unsafe fn otx_cpt_handle_mbox_intr(cpt: *mut otx_cpt_device, vf: c_int) {
    let mut vftype = 0;
    let mut mbx: otx_cpt_mbox = core::mem::zeroed();
    let dev = &(*(*cpt).pdev).dev;
    mbx.msg = readq((*cpt).reg_base + OTX_CPT_PF_VFX_MBOXX(vf, 0));
    mbx.data = readq((*cpt).reg_base + OTX_CPT_PF_VFX_MBOXX(vf, 1));
    dump_mbox_msg(&mut mbx, vf);
    match mbx.msg {
        OTX_CPT_MSG_VF_UP => { mbx.msg = OTX_CPT_MSG_VF_UP; mbx.data = (*cpt).vfs_enabled; otx_cpt_send_msg_to_vf(cpt, vf, &mut mbx); }
        OTX_CPT_MSG_READY => { mbx.msg = OTX_CPT_MSG_READY; mbx.data = vf as u64; otx_cpt_send_msg_to_vf(cpt, vf, &mut mbx); }
        OTX_CPT_MSG_VF_DOWN => otx_cpt_mbox_send_ack(cpt, vf, &mut mbx),
        OTX_CPT_MSG_QLEN => { otx_cpt_cfg_qlen_for_vf(cpt, vf, mbx.data as u32); otx_cpt_mbox_send_ack(cpt, vf, &mut mbx); }
        OTX_CPT_MSG_QBIND_GRP => { vftype = otx_cpt_bind_vq_to_grp(cpt, vf as u8, mbx.data as u8); if vftype != OTX_CPT_AE_TYPES && vftype != OTX_CPT_SE_TYPES { dev_err!(dev, "VF%d binding to eng group %llu failed\n", vf, mbx.data); otx_cptpf_mbox_send_nack(cpt, vf, &mut mbx); } else { mbx.msg = OTX_CPT_MSG_QBIND_GRP; mbx.data = vftype as u64; otx_cpt_send_msg_to_vf(cpt, vf, &mut mbx); } }
        OTX_CPT_MSG_PF_TYPE => { mbx.msg = OTX_CPT_MSG_PF_TYPE; mbx.data = (*cpt).pf_type; otx_cpt_send_msg_to_vf(cpt, vf, &mut mbx); }
        OTX_CPT_MSG_VQ_PRIORITY => { otx_cpt_cfg_vq_priority(cpt, vf, mbx.data as u32); otx_cpt_mbox_send_ack(cpt, vf, &mut mbx); }
        _ => dev_err!(&(*(*cpt).pdev).dev, "Invalid msg from VF%d, msg 0x%llx\n", vf, mbx.msg),
    }
}

pub unsafe fn otx_cpt_mbox_intr_handler(cpt: *mut otx_cpt_device, mbx: c_int) {
    let intr = readq((*cpt).reg_base + OTX_CPT_PF_MBOX_INTX(0));
    pr_debug!("PF interrupt mbox%d mask 0x%llx\n", mbx, intr);
    let mut vf: u8 = 0;
    while vf < (*cpt).max_vfs {
        if intr & (1u64 << vf) != 0 { otx_cpt_handle_mbox_intr(cpt, vf as c_int); otx_cpt_clear_mbox_intr(cpt, vf as u32); }
        vf += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
