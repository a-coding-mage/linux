// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2021 Intel Corporation */

// Translated from the Linux kernel implementation. External types, constants,
// macros, and functions are supplied by the surrounding repository.

const ADF_4XXX_VM2PF_SOU: u32 = 0x41A180;
const ADF_4XXX_VM2PF_MSK: u32 = 0x41A1C0;
const ADF_GEN4_VF_MSK: u32 = 0xFFFF;

const ADF_PFVF_GEN4_MSGTYPE_SHIFT: u32 = 2;
const ADF_PFVF_GEN4_MSGTYPE_MASK: u32 = 0x3F;
const ADF_PFVF_GEN4_MSGDATA_SHIFT: u32 = 8;
const ADF_PFVF_GEN4_MSGDATA_MASK: u32 = 0xFFFFFF;

static csr_gen4_fmt: pfvf_csr_format = pfvf_csr_format {
    msg_type: pfvf_csr_field {
        shift: ADF_PFVF_GEN4_MSGTYPE_SHIFT,
        mask: ADF_PFVF_GEN4_MSGTYPE_MASK,
    },
    msg_data: pfvf_csr_field {
        shift: ADF_PFVF_GEN4_MSGDATA_SHIFT,
        mask: ADF_PFVF_GEN4_MSGDATA_MASK,
    },
};

unsafe fn adf_gen4_pf_get_pf2vf_offset(i: u32) -> u32 {
    ADF_GEN4_PF2VM_OFFSET(i)
}

unsafe fn adf_gen4_pf_get_vf2pf_offset(i: u32) -> u32 {
    ADF_GEN4_VM2PF_OFFSET(i)
}

unsafe fn adf_gen4_enable_vf2pf_interrupts(pmisc_addr: *mut core::ffi::c_void, vf_mask: u32) {
    let val = ADF_CSR_RD(pmisc_addr, ADF_4XXX_VM2PF_MSK) & !vf_mask;
    ADF_CSR_WR(pmisc_addr, ADF_4XXX_VM2PF_MSK, val);
}

unsafe fn adf_gen4_disable_all_vf2pf_interrupts(pmisc_addr: *mut core::ffi::c_void) {
    ADF_CSR_WR(pmisc_addr, ADF_4XXX_VM2PF_MSK, ADF_GEN4_VF_MSK);
}

unsafe fn adf_gen4_disable_pending_vf2pf_interrupts(
    pmisc_addr: *mut core::ffi::c_void,
) -> u32 {
    let sources = ADF_CSR_RD(pmisc_addr, ADF_4XXX_VM2PF_SOU);
    if sources == 0 {
        return 0;
    }

    let disabled = ADF_CSR_RD(pmisc_addr, ADF_4XXX_VM2PF_MSK);
    let pending = sources & !disabled;
    if pending == 0 {
        return 0;
    }

    /*
     * Due to HW limitations, when disabling the interrupts, we can't just
     * disable the requested sources, as this would lead to missed interrupts
     * if VM2PF_SOU changes just before writing to VM2PF_MSK. To work around
     * it, disable all and re-enable only the sources that are not in vf_mask
     * and were not already disabled. Re-enabling will trigger a new interrupt
     * for the sources that have changed in the meantime, if any.
     */
    ADF_CSR_WR(pmisc_addr, ADF_4XXX_VM2PF_MSK, ADF_GEN4_VF_MSK);
    ADF_CSR_WR(pmisc_addr, ADF_4XXX_VM2PF_MSK, disabled | sources);

    // Return the sources of the (new) interrupt(s)
    pending
}

unsafe fn adf_gen4_pfvf_send(
    accel_dev: *mut adf_accel_dev,
    msg: pfvf_message,
    pfvf_offset: u32,
    csr_lock: *mut mutex,
) -> i32 {
    let pmisc_addr = adf_get_pmisc_base(accel_dev);
    let mut csr_val: u32;

    csr_val = adf_pfvf_csr_msg_of(accel_dev, msg, &csr_gen4_fmt);
    if csr_val == 0 {
        return -EINVAL;
    }

    mutex_lock(csr_lock);
    ADF_CSR_WR(pmisc_addr, pfvf_offset, csr_val | ADF_PFVF_INT);

    // Wait for confirmation from remote that it received the message
    let ret = read_poll_timeout(
        ADF_CSR_RD,
        &mut csr_val,
        (csr_val & ADF_PFVF_INT) == 0,
        ADF_PFVF_MSG_ACK_DELAY_US,
        ADF_PFVF_MSG_ACK_MAX_DELAY_US,
        true,
        pmisc_addr,
        pfvf_offset,
    );
    if ret < 0 {
        dev_dbg(&GET_DEV(accel_dev), "ACK not received from remote\n");
    }

    mutex_unlock(csr_lock);
    ret
}

unsafe fn adf_gen4_pfvf_recv(
    accel_dev: *mut adf_accel_dev,
    pfvf_offset: u32,
    compat_ver: u8,
) -> pfvf_message {
    let pmisc_addr = adf_get_pmisc_base(accel_dev);
    let mut msg: pfvf_message = core::mem::zeroed();
    let csr_val = ADF_CSR_RD(pmisc_addr, pfvf_offset);

    if (csr_val & ADF_PFVF_INT) == 0 {
        dev_info(
            &GET_DEV(accel_dev),
            "Spurious PFVF interrupt, msg 0x%.8x. Ignored\n",
            csr_val,
        );
        return msg;
    }

    // We can now acknowledge the message reception by clearing the interrupt bit
    ADF_CSR_WR(pmisc_addr, pfvf_offset, csr_val & !ADF_PFVF_INT);

    // Return the pfvf_message format
    msg = adf_pfvf_message_of(accel_dev, csr_val, &csr_gen4_fmt);
    msg
}

unsafe fn adf_gen4_init_pf_pfvf_ops(pfvf_ops: *mut adf_pfvf_ops) {
    (*pfvf_ops).enable_comms = adf_enable_pf2vf_comms;
    (*pfvf_ops).get_pf2vf_offset = adf_gen4_pf_get_pf2vf_offset;
    (*pfvf_ops).get_vf2pf_offset = adf_gen4_pf_get_vf2pf_offset;
    (*pfvf_ops).enable_vf2pf_interrupts = adf_gen4_enable_vf2pf_interrupts;
    (*pfvf_ops).disable_all_vf2pf_interrupts = adf_gen4_disable_all_vf2pf_interrupts;
    (*pfvf_ops).disable_pending_vf2pf_interrupts = adf_gen4_disable_pending_vf2pf_interrupts;
    (*pfvf_ops).send_msg = adf_gen4_pfvf_send;
    (*pfvf_ops).recv_msg = adf_gen4_pfvf_recv;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
