// SPDX-License-Identifier: GPL-2.0-or-later
/* backing_ops.c - query/set operations on saved SPU context.
 *
 * Copyright (C) IBM 2005
 * Author: Mark Nutter <mnutter@us.ibm.com>
 *
 * These register operations allow SPUFS to operate on saved
 * SPU contexts rather than hardware.
 */

// Linux and architecture dependencies are supplied by the surrounding translation unit.

/*
 * Reads/writes to various problem and priv2 registers require
 * state changes, i.e. generate SPU events, modify channel
 * counts, etc.
 */

unsafe fn gen_spu_event(ctx: *mut spu_context, event: u32) {
    let ch0_cnt: u64 = (*ctx).csa.spu_chnlcnt_RW[0];
    let ch0_data: u64 = (*ctx).csa.spu_chnldata_RW[0];
    let ch1_data: u64 = (*ctx).csa.spu_chnldata_RW[1];
    (*ctx).csa.spu_chnldata_RW[0] |= event as u64;
    if ch0_cnt == 0 && (ch0_data & event as u64) == 0 && (ch1_data & event as u64) != 0 {
        (*ctx).csa.spu_chnlcnt_RW[0] = 1;
    }
}

unsafe fn spu_backing_mbox_read(ctx: *mut spu_context, data: *mut u32) -> i32 {
    let mut ret = 0;
    spin_lock(&mut (*ctx).csa.register_lock);
    let mbox_stat = (*ctx).csa.prob.mb_stat_R;
    if mbox_stat & 0x0000ff != 0 {
        /* Read the first available word. The depth of pu_mb_R is currently 1. */
        *data = (*ctx).csa.prob.pu_mb_R;
        (*ctx).csa.prob.mb_stat_R &= !0x0000ff;
        (*ctx).csa.spu_chnlcnt_RW[28] = 1;
        gen_spu_event(ctx, MFC_PU_MAILBOX_AVAILABLE_EVENT);
        ret = 4;
    }
    spin_unlock(&mut (*ctx).csa.register_lock);
    ret
}

unsafe fn spu_backing_mbox_stat_read(ctx: *mut spu_context) -> u32 { (*ctx).csa.prob.mb_stat_R }

unsafe fn spu_backing_mbox_stat_poll(ctx: *mut spu_context, events: __poll_t) -> __poll_t {
    let mut ret = 0;
    spin_lock_irq(&mut (*ctx).csa.register_lock);
    let stat = (*ctx).csa.prob.mb_stat_R;
    if events & (EPOLLIN | EPOLLRDNORM) != 0 {
        if stat & 0xff0000 != 0 { ret |= EPOLLIN | EPOLLRDNORM; }
        else {
            (*ctx).csa.priv1.int_stat_class2_RW &= !CLASS2_MAILBOX_INTR;
            (*ctx).csa.priv1.int_mask_class2_RW |= CLASS2_ENABLE_MAILBOX_INTR;
        }
    }
    if events & (EPOLLOUT | EPOLLWRNORM) != 0 {
        if stat & 0x00ff00 != 0 { ret = EPOLLOUT | EPOLLWRNORM; }
        else {
            (*ctx).csa.priv1.int_stat_class2_RW &= !CLASS2_MAILBOX_THRESHOLD_INTR;
            (*ctx).csa.priv1.int_mask_class2_RW |= CLASS2_ENABLE_MAILBOX_THRESHOLD_INTR;
        }
    }
    spin_unlock_irq(&mut (*ctx).csa.register_lock);
    ret
}

unsafe fn spu_backing_ibox_read(ctx: *mut spu_context, data: *mut u32) -> i32 {
    spin_lock(&mut (*ctx).csa.register_lock);
    let ret;
    if (*ctx).csa.prob.mb_stat_R & 0xff0000 != 0 {
        /* Read the first available word. The depth of puint_mb_R is currently 1. */
        *data = (*ctx).csa.priv2.puint_mb_R;
        (*ctx).csa.prob.mb_stat_R &= !0xff0000;
        (*ctx).csa.spu_chnlcnt_RW[30] = 1;
        gen_spu_event(ctx, MFC_PU_INT_MAILBOX_AVAILABLE_EVENT);
        ret = 4;
    } else {
        (*ctx).csa.priv1.int_mask_class2_RW |= CLASS2_ENABLE_MAILBOX_INTR;
        ret = 0;
    }
    spin_unlock(&mut (*ctx).csa.register_lock);
    ret
}

unsafe fn spu_backing_wbox_write(ctx: *mut spu_context, data: u32) -> i32 {
    spin_lock(&mut (*ctx).csa.register_lock);
    let ret;
    if (*ctx).csa.prob.mb_stat_R & 0x00ff00 != 0 {
        let slot = (*ctx).csa.spu_chnlcnt_RW[29] as usize;
        let avail = ((*ctx).csa.prob.mb_stat_R & 0x00ff00) >> 8;
        BUG_ON(avail != 4 - slot as u64);
        (*ctx).csa.spu_mailbox_data[slot] = data;
        let next_slot = slot + 1;
        (*ctx).csa.spu_chnlcnt_RW[29] = next_slot as u64;
        (*ctx).csa.prob.mb_stat_R &= !0x00ff00;
        (*ctx).csa.prob.mb_stat_R |= (((4 - next_slot) & 0xff) << 8) as u64;
        gen_spu_event(ctx, MFC_SPU_MAILBOX_WRITTEN_EVENT);
        ret = 4;
    } else {
        (*ctx).csa.priv1.int_mask_class2_RW |= CLASS2_ENABLE_MAILBOX_THRESHOLD_INTR;
        ret = 0;
    }
    spin_unlock(&mut (*ctx).csa.register_lock);
    ret
}

unsafe fn spu_backing_signal1_read(ctx: *mut spu_context) -> u32 { (*ctx).csa.spu_chnldata_RW[3] as u32 }
unsafe fn spu_backing_signal2_read(ctx: *mut spu_context) -> u32 { (*ctx).csa.spu_chnldata_RW[4] as u32 }

unsafe fn spu_backing_signal1_write(ctx: *mut spu_context, data: u32) {
    spin_lock(&mut (*ctx).csa.register_lock);
    if (*ctx).csa.priv2.spu_cfg_RW & 1 != 0 { (*ctx).csa.spu_chnldata_RW[3] |= data as u64; } else { (*ctx).csa.spu_chnldata_RW[3] = data as u64; }
    (*ctx).csa.spu_chnlcnt_RW[3] = 1; gen_spu_event(ctx, MFC_SIGNAL_1_EVENT); spin_unlock(&mut (*ctx).csa.register_lock);
}
unsafe fn spu_backing_signal2_write(ctx: *mut spu_context, data: u32) {
    spin_lock(&mut (*ctx).csa.register_lock);
    if (*ctx).csa.priv2.spu_cfg_RW & 2 != 0 { (*ctx).csa.spu_chnldata_RW[4] |= data as u64; } else { (*ctx).csa.spu_chnldata_RW[4] = data as u64; }
    (*ctx).csa.spu_chnlcnt_RW[4] = 1; gen_spu_event(ctx, MFC_SIGNAL_2_EVENT); spin_unlock(&mut (*ctx).csa.register_lock);
}

unsafe fn spu_backing_signal1_type_set(ctx: *mut spu_context, val: u64) { spin_lock(&mut (*ctx).csa.register_lock); if val != 0 { (*ctx).csa.priv2.spu_cfg_RW |= 1; } else { (*ctx).csa.priv2.spu_cfg_RW &= !1; } spin_unlock(&mut (*ctx).csa.register_lock); }
unsafe fn spu_backing_signal1_type_get(ctx: *mut spu_context) -> u64 { ((*ctx).csa.priv2.spu_cfg_RW & 1 != 0) as u64 }
unsafe fn spu_backing_signal2_type_set(ctx: *mut spu_context, val: u64) { spin_lock(&mut (*ctx).csa.register_lock); if val != 0 { (*ctx).csa.priv2.spu_cfg_RW |= 2; } else { (*ctx).csa.priv2.spu_cfg_RW &= !2; } spin_unlock(&mut (*ctx).csa.register_lock); }
unsafe fn spu_backing_signal2_type_get(ctx: *mut spu_context) -> u64 { ((*ctx).csa.priv2.spu_cfg_RW & 2 != 0) as u64 }

unsafe fn spu_backing_npc_read(ctx: *mut spu_context) -> u32 { (*ctx).csa.prob.spu_npc_RW }
unsafe fn spu_backing_npc_write(ctx: *mut spu_context, val: u32) { (*ctx).csa.prob.spu_npc_RW = val; }
unsafe fn spu_backing_status_read(ctx: *mut spu_context) -> u32 { (*ctx).csa.prob.spu_status_R }
unsafe fn spu_backing_get_ls(ctx: *mut spu_context) -> *mut i8 { (*ctx).csa.lscsa.ls }
unsafe fn spu_backing_privcntl_write(ctx: *mut spu_context, val: u64) { (*ctx).csa.priv2.spu_privcntl_RW = val; }
unsafe fn spu_backing_runcntl_read(ctx: *mut spu_context) -> u32 { (*ctx).csa.prob.spu_runcntl_RW }

unsafe fn spu_backing_runcntl_write(ctx: *mut spu_context, val: u32) {
    spin_lock(&mut (*ctx).csa.register_lock);
    (*ctx).csa.prob.spu_runcntl_RW = val;
    if val & SPU_RUNCNTL_RUNNABLE != 0 {
        (*ctx).csa.prob.spu_status_R &= !SPU_STATUS_STOPPED_BY_STOP & !SPU_STATUS_STOPPED_BY_HALT & !SPU_STATUS_SINGLE_STEP & !SPU_STATUS_INVALID_INSTR & !SPU_STATUS_INVALID_CH;
        (*ctx).csa.prob.spu_status_R |= SPU_STATUS_RUNNING;
    } else { (*ctx).csa.prob.spu_status_R &= !SPU_STATUS_RUNNING; }
    spin_unlock(&mut (*ctx).csa.register_lock);
}
unsafe fn spu_backing_runcntl_stop(ctx: *mut spu_context) { spu_backing_runcntl_write(ctx, SPU_RUNCNTL_STOP); }

unsafe fn spu_backing_master_start(ctx: *mut spu_context) { spin_lock(&mut (*ctx).csa.register_lock); (*ctx).csa.priv1.mfc_sr1_RW |= MFC_STATE1_MASTER_RUN_CONTROL_MASK; spin_unlock(&mut (*ctx).csa.register_lock); }
unsafe fn spu_backing_master_stop(ctx: *mut spu_context) { spin_lock(&mut (*ctx).csa.register_lock); (*ctx).csa.priv1.mfc_sr1_RW &= !MFC_STATE1_MASTER_RUN_CONTROL_MASK; spin_unlock(&mut (*ctx).csa.register_lock); }

unsafe fn spu_backing_set_mfc_query(ctx: *mut spu_context, mask: u32, mode: u32) -> i32 {
    spin_lock(&mut (*ctx).csa.register_lock);
    let ret;
    if (*ctx).csa.prob.dma_querytype_RW != 0 { ret = -EAGAIN; }
    else { ret = 0; (*ctx).csa.prob.dma_querymask_RW = mask; (*ctx).csa.prob.dma_querytype_RW = mode; (*ctx).csa.prob.dma_tagstatus_R &= mask; }
    spin_unlock(&mut (*ctx).csa.register_lock); ret
}
unsafe fn spu_backing_read_mfc_tagstatus(ctx: *mut spu_context) -> u32 { (*ctx).csa.prob.dma_tagstatus_R }
unsafe fn spu_backing_get_mfc_free_elements(ctx: *mut spu_context) -> u32 { (*ctx).csa.prob.dma_qstatus_R }
unsafe fn spu_backing_send_mfc_command(ctx: *mut spu_context, _cmd: *mut mfc_dma_command) -> i32 { spin_lock(&mut (*ctx).csa.register_lock); let ret = -EAGAIN; spin_unlock(&mut (*ctx).csa.register_lock); ret }
unsafe fn spu_backing_restart_dma(ctx: *mut spu_context) { (*ctx).csa.priv2.mfc_control_RW |= MFC_CNTL_RESTART_DMA_COMMAND; }

pub static mut spu_backing_ops: spu_context_ops = spu_context_ops {
    mbox_read: Some(spu_backing_mbox_read), mbox_stat_read: Some(spu_backing_mbox_stat_read), mbox_stat_poll: Some(spu_backing_mbox_stat_poll), ibox_read: Some(spu_backing_ibox_read), wbox_write: Some(spu_backing_wbox_write), signal1_read: Some(spu_backing_signal1_read), signal1_write: Some(spu_backing_signal1_write), signal2_read: Some(spu_backing_signal2_read), signal2_write: Some(spu_backing_signal2_write), signal1_type_set: Some(spu_backing_signal1_type_set), signal1_type_get: Some(spu_backing_signal1_type_get), signal2_type_set: Some(spu_backing_signal2_type_set), signal2_type_get: Some(spu_backing_signal2_type_get), npc_read: Some(spu_backing_npc_read), npc_write: Some(spu_backing_npc_write), status_read: Some(spu_backing_status_read), get_ls: Some(spu_backing_get_ls), privcntl_write: Some(spu_backing_privcntl_write), runcntl_read: Some(spu_backing_runcntl_read), runcntl_write: Some(spu_backing_runcntl_write), runcntl_stop: Some(spu_backing_runcntl_stop), master_start: Some(spu_backing_master_start), master_stop: Some(spu_backing_master_stop), set_mfc_query: Some(spu_backing_set_mfc_query), read_mfc_tagstatus: Some(spu_backing_read_mfc_tagstatus), get_mfc_free_elements: Some(spu_backing_get_mfc_free_elements), send_mfc_command: Some(spu_backing_send_mfc_command), restart_dma: Some(spu_backing_restart_dma),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
