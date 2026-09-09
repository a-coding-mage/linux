// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Cavium, Inc.
 */
// Dependency declarations and build-time definitions are supplied by cptpf.h.

unsafe fn cpt_send_msg_to_vf(cpt: *mut cpt_device, vf: i32, mbx: *mut cpt_mbox) {
    /* Writing mbox(0) causes interrupt */
    cpt_write_csr64((*cpt).reg_base, CPTX_PF_VFX_MBOXX(0, vf, 1), (*mbx).data);
    cpt_write_csr64((*cpt).reg_base, CPTX_PF_VFX_MBOXX(0, vf, 0), (*mbx).msg);
}

/* ACKs VF's mailbox message
 * @vf: VF to which ACK to be sent
 */
unsafe fn cpt_mbox_send_ack(cpt: *mut cpt_device, vf: i32, mbx: *mut cpt_mbox) {
    (*mbx).data = 0u64;
    (*mbx).msg = CPT_MBOX_MSG_TYPE_ACK;
    cpt_send_msg_to_vf(cpt, vf, mbx);
}

unsafe fn cpt_clear_mbox_intr(cpt: *mut cpt_device, vf: u32) {
    /* W1C for the VF */
    cpt_write_csr64((*cpt).reg_base, CPTX_PF_MBOX_INTX(0, 0), 1u64 << vf);
}

/*
 *  Configure QLEN/Chunk sizes for VF
 */
unsafe fn cpt_cfg_qlen_for_vf(cpt: *mut cpt_device, vf: i32, size: u32) {
    let mut pf_qx_ctl: cptx_pf_qx_ctl = core::mem::zeroed();

    pf_qx_ctl.u = cpt_read_csr64((*cpt).reg_base, CPTX_PF_QX_CTL(0, vf));
    pf_qx_ctl.s.size = size;
    pf_qx_ctl.s.cont_err = true;
    cpt_write_csr64((*cpt).reg_base, CPTX_PF_QX_CTL(0, vf), pf_qx_ctl.u);
}

/*
 * Configure VQ priority
 */
unsafe fn cpt_cfg_vq_priority(cpt: *mut cpt_device, vf: i32, pri: u32) {
    let mut pf_qx_ctl: cptx_pf_qx_ctl = core::mem::zeroed();

    pf_qx_ctl.u = cpt_read_csr64((*cpt).reg_base, CPTX_PF_QX_CTL(0, vf));
    pf_qx_ctl.s.pri = pri;
    cpt_write_csr64((*cpt).reg_base, CPTX_PF_QX_CTL(0, vf), pf_qx_ctl.u);
}

unsafe fn cpt_bind_vq_to_grp(cpt: *mut cpt_device, q: u8, grp: u8) -> i32 {
    let mcode = (*cpt).mcode;
    let mut pf_qx_ctl: cptx_pf_qx_ctl = core::mem::zeroed();
    let dev = &(*cpt).pdev.dev as *const device;

    if q >= CPT_MAX_VF_NUM {
        dev_err(dev, "Queues are more than cores in the group");
        return -EINVAL;
    }
    if grp >= CPT_MAX_CORE_GROUPS {
        dev_err(dev, "Request group is more than possible groups");
        return -EINVAL;
    }
    if grp >= (*cpt).next_mc_idx {
        dev_err(dev, "Request group is higher than available functional groups");
        return -EINVAL;
    }
    pf_qx_ctl.u = cpt_read_csr64((*cpt).reg_base, CPTX_PF_QX_CTL(0, q));
    pf_qx_ctl.s.grp = (*mcode.add(grp as usize)).group;
    cpt_write_csr64((*cpt).reg_base, CPTX_PF_QX_CTL(0, q), pf_qx_ctl.u);
    dev_dbg(dev, "VF %d TYPE %s", q, if (*mcode.add(grp as usize)).is_ae { "AE" } else { "SE" });

    if (*mcode.add(grp as usize)).is_ae { AE_TYPES } else { SE_TYPES }
}

/* Interrupt handler to handle mailbox messages from VFs */
unsafe fn cpt_handle_mbox_intr(cpt: *mut cpt_device, vf: i32) {
    let vfx = &mut (*cpt).vfinfo[vf as usize];
    let mut mbx: cpt_mbox = core::mem::zeroed();
    let mut vftype: i32;
    let dev = &(*cpt).pdev.dev as *const device;
    /*
     * MBOX[0] contains msg
     * MBOX[1] contains data
     */
    mbx.msg = cpt_read_csr64((*cpt).reg_base, CPTX_PF_VFX_MBOXX(0, vf, 0));
    mbx.data = cpt_read_csr64((*cpt).reg_base, CPTX_PF_VFX_MBOXX(0, vf, 1));
    dev_dbg(dev, "%s: Mailbox msg 0x%llx from VF%d", "cpt_handle_mbox_intr", mbx.msg, vf);
    match mbx.msg {
        CPT_MSG_VF_UP => {
            vfx.state = VF_STATE_UP;
            try_module_get(THIS_MODULE);
            cpt_mbox_send_ack(cpt, vf, &mut mbx);
        }
        CPT_MSG_READY => {
            mbx.msg = CPT_MSG_READY;
            mbx.data = vf as u64;
            cpt_send_msg_to_vf(cpt, vf, &mut mbx);
        }
        CPT_MSG_VF_DOWN => {
            /* First msg in VF teardown sequence */
            vfx.state = VF_STATE_DOWN;
            module_put(THIS_MODULE);
            cpt_mbox_send_ack(cpt, vf, &mut mbx);
        }
        CPT_MSG_QLEN => {
            vfx.qlen = mbx.data;
            cpt_cfg_qlen_for_vf(cpt, vf, vfx.qlen);
            cpt_mbox_send_ack(cpt, vf, &mut mbx);
        }
        CPT_MSG_QBIND_GRP => {
            vftype = cpt_bind_vq_to_grp(cpt, vf as u8, mbx.data as u8);
            if vftype != AE_TYPES && vftype != SE_TYPES {
                dev_err(dev, "Queue %d binding to group %llu failed", vf, mbx.data);
            } else {
                dev_dbg(dev, "Queue %d binding to group %llu successful", vf, mbx.data);
                mbx.msg = CPT_MSG_QBIND_GRP;
                mbx.data = vftype as u64;
                cpt_send_msg_to_vf(cpt, vf, &mut mbx);
            }
        }
        CPT_MSG_VQ_PRIORITY => {
            vfx.priority = mbx.data;
            cpt_cfg_vq_priority(cpt, vf, vfx.priority);
            cpt_mbox_send_ack(cpt, vf, &mut mbx);
        }
        _ => {
            dev_err(&(*cpt).pdev.dev, "Invalid msg from VF%d, msg 0x%llx\n", vf, mbx.msg);
        }
    }
}

pub unsafe fn cpt_mbox_intr_handler(cpt: *mut cpt_device, mbx: i32) {
    let intr: u64;
    let mut vf: u8;

    intr = cpt_read_csr64((*cpt).reg_base, CPTX_PF_MBOX_INTX(0, 0));
    dev_dbg(&(*cpt).pdev.dev, "PF interrupt Mbox%d 0x%llx\n", mbx, intr);
    vf = 0;
    while vf < CPT_MAX_VF_NUM {
        if intr & (1u64 << vf) != 0 {
            dev_dbg(&(*cpt).pdev.dev, "Intr from VF %d\n", vf);
            cpt_handle_mbox_intr(cpt, vf as i32);
            cpt_clear_mbox_intr(cpt, vf as u32);
        }
        vf = vf.wrapping_add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
