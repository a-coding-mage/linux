// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

const fn ring_to_vfno(x: u32, y: u32) -> u32 {
    x / y
}

/*
 * mbx_msg_type - Mailbox message types
 */
#[repr(C)]
enum MbxMsgType {
    MbxMsgTypeNop,
    MbxMsgTypeReq,
    MbxMsgTypeAck,
    MbxMsgTypeNack,
}

/*
 * mbx_msg_opcode - Mailbox message opcodes
 */
#[repr(C)]
enum MbxMsgOpcode {
    MsgOpVfMode = 1,
    MsgOpVfUp,
    MsgOpVfDown,
    MsgOpChipidVfid,
    MsgOpMcodeInfo = 11,
}

#[repr(C)]
struct Pf2vfWork {
    vfdev: *mut NitroxVfdev,
    ndev: *mut NitroxDevice,
    pf2vf_resp: WorkStruct,
}

#[inline]
unsafe fn pf2vf_read_mbox(ndev: *mut NitroxDevice, ring: i32) -> u64 {
    let reg_addr: u64 = NPS_PKT_MBOX_VF_PF_PFDATAX(ring);
    nitrox_read_csr(ndev, reg_addr)
}

#[inline]
unsafe fn pf2vf_write_mbox(ndev: *mut NitroxDevice, value: u64, ring: i32) {
    let reg_addr: u64 = NPS_PKT_MBOX_PF_VF_PFDATAX(ring);
    nitrox_write_csr(ndev, reg_addr, value);
}

unsafe fn pf2vf_send_response(ndev: *mut NitroxDevice, vfdev: *mut NitroxVfdev) {
    let mut msg: MboxMsg;

    msg.value = (*vfdev).msg.value;

    match (*vfdev).msg.opcode {
        MSG_OP_VF_MODE => {
            msg.data = (*ndev).mode;
        }
        MSG_OP_VF_UP => {
            (*vfdev).nr_queues = (*vfdev).msg.data;
            atomic_set(&mut (*vfdev).state, NDEV_READY);
        }
        MSG_OP_CHIPID_VFID => {
            msg.id.chipid = (*ndev).idx;
            msg.id.vfid = (*vfdev).vfno;
        }
        MSG_OP_VF_DOWN => {
            (*vfdev).nr_queues = 0;
            atomic_set(&mut (*vfdev).state, NDEV_NOT_READY);
        }
        MSG_OP_MCODE_INFO => {
            msg.data = 0;
            msg.mcode_info.count = 2;
            msg.mcode_info.info = MCODE_TYPE_SE_SSL | (MCODE_TYPE_AE << 5);
            msg.mcode_info.next_se_grp = 1;
            msg.mcode_info.next_ae_grp = 1;
        }
        _ => {
            msg.msg_type = MBX_MSG_TYPE_NOP;
        }
    }

    if msg.msg_type == MBX_MSG_TYPE_NOP {
        return;
    }

    /* send ACK to VF */
    msg.msg_type = MBX_MSG_TYPE_ACK;
    pf2vf_write_mbox(ndev, msg.value, (*vfdev).ring);

    (*vfdev).msg.value = 0;
    atomic64_inc(&mut (*vfdev).mbx_resp);
}

unsafe fn pf2vf_resp_handler(work: *mut WorkStruct) {
    let pf2vf_resp: *mut Pf2vfWork = container_of!(work, Pf2vfWork, pf2vf_resp);
    let vfdev = (*pf2vf_resp).vfdev;
    let ndev = (*pf2vf_resp).ndev;

    match (*vfdev).msg.msg_type {
        MBX_MSG_TYPE_REQ => {
            /* process the request from VF */
            pf2vf_send_response(ndev, vfdev);
        }
        MBX_MSG_TYPE_ACK | MBX_MSG_TYPE_NACK => {}
        _ => {}
    }

    kfree(pf2vf_resp);
}

unsafe fn nitrox_pf2vf_mbox_handler(ndev: *mut NitroxDevice) {
    let mut csr: u64;
    let mut vfdev: *mut NitroxVfdev;
    let mut pfwork: *mut Pf2vfWork;
    let mut value: u64;
    let mut reg_addr: u64;
    let mut i: u32;
    let mut vfno: i32;

    /* loop for VF(0..63) */
    reg_addr = NPS_PKT_MBOX_INT_LO;
    value = nitrox_read_csr(ndev, reg_addr);
    csr = value;
    i = 0;
    while i < 64 {
        if (csr & (1u64 << i)) != 0 {
            /* get the vfno from ring */
            vfno = ring_to_vfno(i, (*ndev).iov.max_vf_queues) as i32;
            vfdev = (*ndev).iov.vfdev.add(vfno as usize);
            (*vfdev).ring = i as i32;
            /* fill the vf mailbox data */
            (*vfdev).msg.value = pf2vf_read_mbox(ndev, (*vfdev).ring);
            pfwork = kzalloc_obj::<Pf2vfWork>(GFP_ATOMIC);
            if !pfwork.is_null() {
                (*pfwork).vfdev = vfdev;
                (*pfwork).ndev = ndev;
                INIT_WORK!(&mut (*pfwork).pf2vf_resp, pf2vf_resp_handler);
                queue_work((*ndev).iov.pf2vf_wq, &mut (*pfwork).pf2vf_resp);
                /* clear the corresponding vf bit */
                nitrox_write_csr(ndev, reg_addr, 1u64 << i);
            }
        }
        i += 1;
    }

    /* loop for VF(64..127) */
    reg_addr = NPS_PKT_MBOX_INT_HI;
    value = nitrox_read_csr(ndev, reg_addr);
    csr = value;
    i = 0;
    while i < 64 {
        if (csr & (1u64 << i)) != 0 {
            /* get the vfno from ring */
            vfno = ring_to_vfno(i + 64, (*ndev).iov.max_vf_queues) as i32;
            vfdev = (*ndev).iov.vfdev.add(vfno as usize);
            (*vfdev).ring = (i + 64) as i32;
            /* fill the vf mailbox data */
            (*vfdev).msg.value = pf2vf_read_mbox(ndev, (*vfdev).ring);
            pfwork = kzalloc_obj::<Pf2vfWork>(GFP_ATOMIC);
            if !pfwork.is_null() {
                (*pfwork).vfdev = vfdev;
                (*pfwork).ndev = ndev;
                INIT_WORK!(&mut (*pfwork).pf2vf_resp, pf2vf_resp_handler);
                queue_work((*ndev).iov.pf2vf_wq, &mut (*pfwork).pf2vf_resp);
                /* clear the corresponding vf bit */
                nitrox_write_csr(ndev, reg_addr, 1u64 << i);
            }
        }
        i += 1;
    }
}

unsafe fn nitrox_mbox_init(ndev: *mut NitroxDevice) -> i32 {
    let mut vfdev: *mut NitroxVfdev;
    let mut i: i32;

    (*ndev).iov.vfdev = kzalloc_objs::<NitroxVfdev>((*ndev).iov.num_vfs);
    if (*ndev).iov.vfdev.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < (*ndev).iov.num_vfs {
        vfdev = (*ndev).iov.vfdev.add(i as usize);
        (*vfdev).vfno = i;
        i += 1;
    }

    /* allocate pf2vf response workqueue */
    (*ndev).iov.pf2vf_wq = alloc_workqueue!("nitrox_pf2vf", WQ_PERCPU, 0);
    if (*ndev).iov.pf2vf_wq.is_null() {
        kfree((*ndev).iov.vfdev);
        (*ndev).iov.vfdev = core::ptr::null_mut();
        return -ENOMEM;
    }
    /* enable pf2vf mailbox interrupts */
    enable_pf2vf_mbox_interrupts(ndev);

    0
}

unsafe fn nitrox_mbox_cleanup(ndev: *mut NitroxDevice) {
    /* disable pf2vf mailbox interrupts */
    disable_pf2vf_mbox_interrupts(ndev);
    /* destroy workqueue */
    if !(*ndev).iov.pf2vf_wq.is_null() {
        destroy_workqueue((*ndev).iov.pf2vf_wq);
    }

    kfree((*ndev).iov.vfdev);
    (*ndev).iov.pf2vf_wq = core::ptr::null_mut();
    (*ndev).iov.vfdev = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
