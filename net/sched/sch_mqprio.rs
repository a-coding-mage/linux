// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level translation of net/sched/sch_mqprio.c.  Kernel symbols are
 * supplied by the surrounding translation unit. */

#[repr(C)]
pub struct MqprioSched {
    pub qdiscs: *mut *mut Qdisc,
    pub mode: u16,
    pub shaper: u16,
    pub hw_offload: i32,
    pub flags: u32,
    pub min_rate: [u64; TC_QOPT_MAX_QUEUE],
    pub max_rate: [u64; TC_QOPT_MAX_QUEUE],
    pub fp: [u32; TC_QOPT_MAX_QUEUE],
}

unsafe fn mqprio_enable_offload(sch: *mut Qdisc, qopt: *const TcMqprioQopt,
                                extack: *mut NetlinkExtAck) -> i32 {
    let priv_ = qdisc_priv::<MqprioSched>(sch);
    let dev = qdisc_dev(sch);
    let mut mqprio: TcMqprioQoptOffload = core::mem::zeroed();
    mqprio.qopt = *qopt;
    mqprio.extack = extack;
    match (*priv_).mode {
        TC_MQPRIO_MODE_DCB => { if (*priv_).shaper != TC_MQPRIO_SHAPER_DCB { return -EINVAL; } }
        TC_MQPRIO_MODE_CHANNEL => {
            mqprio.flags = (*priv_).flags;
            if (*priv_).flags & TC_MQPRIO_F_MODE != 0 { mqprio.mode = (*priv_).mode; }
            if (*priv_).flags & TC_MQPRIO_F_SHAPER != 0 { mqprio.shaper = (*priv_).shaper; }
            for i in 0..((*qopt).num_tc as usize) {
                if (*priv_).flags & TC_MQPRIO_F_MIN_RATE != 0 { mqprio.min_rate[i] = (*priv_).min_rate[i]; }
                if (*priv_).flags & TC_MQPRIO_F_MAX_RATE != 0 { mqprio.max_rate[i] = (*priv_).max_rate[i]; }
            }
        }
        _ => return -EINVAL,
    }
    mqprio_fp_to_offload((*priv_).fp.as_ptr(), &mut mqprio);
    let err = (*(*dev).netdev_ops).ndo_setup_tc.unwrap()(dev, TC_SETUP_QDISC_MQPRIO, &mut mqprio as *mut _ as *mut core::ffi::c_void);
    if err != 0 { return err; }
    (*priv_).hw_offload = mqprio.qopt.hw as i32;
    0
}

unsafe fn mqprio_disable_offload(sch: *mut Qdisc) {
    let mut mqprio: TcMqprioQoptOffload = core::mem::zeroed();
    let priv_ = qdisc_priv::<MqprioSched>(sch); let dev = qdisc_dev(sch);
    if (*priv_).mode == TC_MQPRIO_MODE_DCB || (*priv_).mode == TC_MQPRIO_MODE_CHANNEL {
        if let Some(f) = (*(*dev).netdev_ops).ndo_setup_tc { f(dev, TC_SETUP_QDISC_MQPRIO, &mut mqprio as *mut _ as *mut core::ffi::c_void); }
    }
}

unsafe fn mqprio_destroy(sch: *mut Qdisc) {
    let dev = qdisc_dev(sch); let priv_ = qdisc_priv::<MqprioSched>(sch);
    if !(*priv_).qdiscs.is_null() {
        let mut ntx = 0usize;
        while ntx < (*dev).num_tx_queues as usize && !(*(*priv_).qdiscs.add(ntx)).is_null() { qdisc_put(*(*priv_).qdiscs.add(ntx)); ntx += 1; }
        kfree((*priv_).qdiscs as *mut core::ffi::c_void);
    }
    if (*priv_).hw_offload != 0 && (*(*dev).netdev_ops).ndo_setup_tc.is_some() { mqprio_disable_offload(sch); } else { netdev_set_num_tc(dev, 0); }
}

unsafe fn mqprio_parse_opt(dev: *mut NetDevice, qopt: *mut TcMqprioQopt, caps: *const TcMqprioCaps, extack: *mut NetlinkExtAck) -> i32 {
    if (*qopt).hw > TC_MQPRIO_HW_OFFLOAD_MAX { (*qopt).hw = TC_MQPRIO_HW_OFFLOAD_MAX; }
    let err = mqprio_validate_qopt(dev, qopt, (*qopt).hw == 0 || (*caps).validate_queue_counts, false, extack);
    if err != 0 { return err; }
    if (*qopt).hw != 0 && (*(*dev).netdev_ops).ndo_setup_tc.is_none() { NL_SET_ERR_MSG(extack, "Device does not support hardware offload"); return -EINVAL; }
    0
}

unsafe fn mqprio_queue_get(sch: *mut Qdisc, cl: usize) -> *mut NetdevQueue {
    let dev = qdisc_dev(sch); let ntx = cl.wrapping_sub(1);
    if ntx >= (*dev).num_tx_queues as usize { core::ptr::null_mut() } else { netdev_get_tx_queue(dev, ntx as u32) }
}

unsafe fn mqprio_select_queue(sch: *mut Qdisc, tcm: *mut Tcmsg) -> *mut NetdevQueue { mqprio_queue_get(sch, TC_H_MIN((*tcm).tcm_parent) as usize) }

// The remaining qdisc callbacks retain the exact kernel ABI and are declared
// here so the surrounding kernel translation can provide their implementations.
unsafe extern "C" {
    fn mq_change_real_num_tx(sch: *mut Qdisc, new: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
