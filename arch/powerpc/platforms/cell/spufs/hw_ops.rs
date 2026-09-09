// SPDX-License-Identifier: GPL-2.0-or-later
/* hw_ops.c - query/set operations on active SPU context.
 *
 * Copyright (C) IBM 2005
 * Author: Mark Nutter <mnutter@us.ibm.com>
 */

// Linux and architecture headers from the original implementation provide the
// types, constants, register helpers, and external functions referenced below.

unsafe fn spu_hw_mbox_read(ctx: *mut spu_context, data: *mut u32) -> i32 {
    let spu = (*ctx).spu;
    let prob = (*spu).problem;
    let mut ret: i32 = 0;

    spin_lock_irq(&mut (*spu).register_lock);
    let mbox_stat = in_be32(&(*prob).mb_stat_R);
    if mbox_stat & 0x0000ff != 0 {
        *data = in_be32(&(*prob).pu_mb_R);
        ret = 4;
    }
    spin_unlock_irq(&mut (*spu).register_lock);
    ret
}

unsafe fn spu_hw_mbox_stat_read(ctx: *mut spu_context) -> u32 {
    in_be32(&(*(*ctx).spu).problem.mb_stat_R)
}

unsafe fn spu_hw_mbox_stat_poll(ctx: *mut spu_context, events: __poll_t) -> __poll_t {
    let spu = (*ctx).spu;
    let mut ret: __poll_t = 0;

    spin_lock_irq(&mut (*spu).register_lock);
    let stat = in_be32(&(*(*spu).problem).mb_stat_R);

    /* if the requested event is there, return the poll
       mask, otherwise enable the interrupt to get notified,
       but first mark any pending interrupts as done so
       we don't get woken up unnecessarily */
    if events & (EPOLLIN | EPOLLRDNORM) != 0 {
        if stat & 0xff0000 != 0 {
            ret |= EPOLLIN | EPOLLRDNORM;
        } else {
            spu_int_stat_clear(spu, 2, CLASS2_MAILBOX_INTR);
            spu_int_mask_or(spu, 2, CLASS2_ENABLE_MAILBOX_INTR);
        }
    }
    if events & (EPOLLOUT | EPOLLWRNORM) != 0 {
        if stat & 0x00ff00 != 0 {
            ret = EPOLLOUT | EPOLLWRNORM;
        } else {
            spu_int_stat_clear(spu, 2, CLASS2_MAILBOX_THRESHOLD_INTR);
            spu_int_mask_or(spu, 2, CLASS2_ENABLE_MAILBOX_THRESHOLD_INTR);
        }
    }
    spin_unlock_irq(&mut (*spu).register_lock);
    ret
}

unsafe fn spu_hw_ibox_read(ctx: *mut spu_context, data: *mut u32) -> i32 {
    let spu = (*ctx).spu;
    let prob = (*spu).problem;
    let priv2 = (*spu).priv2;
    let ret: i32;

    spin_lock_irq(&mut (*spu).register_lock);
    if in_be32(&(*prob).mb_stat_R) & 0xff0000 != 0 {
        /* read the first available word */
        *data = in_be64(&(*priv2).puint_mb_R) as u32;
        ret = 4;
    } else {
        /* make sure we get woken up by the interrupt */
        spu_int_mask_or(spu, 2, CLASS2_ENABLE_MAILBOX_INTR);
        ret = 0;
    }
    spin_unlock_irq(&mut (*spu).register_lock);
    ret
}

unsafe fn spu_hw_wbox_write(ctx: *mut spu_context, data: u32) -> i32 {
    let spu = (*ctx).spu;
    let prob = (*spu).problem;
    let ret: i32;

    spin_lock_irq(&mut (*spu).register_lock);
    if in_be32(&(*prob).mb_stat_R) & 0x00ff00 != 0 {
        /* we have space to write wbox_data to */
        out_be32(&mut (*prob).spu_mb_W, data);
        ret = 4;
    } else {
        /* make sure we get woken up by the interrupt when space
           becomes available */
        spu_int_mask_or(spu, 2, CLASS2_ENABLE_MAILBOX_THRESHOLD_INTR);
        ret = 0;
    }
    spin_unlock_irq(&mut (*spu).register_lock);
    ret
}

unsafe fn spu_hw_signal1_write(ctx: *mut spu_context, data: u32) {
    out_be32(&mut (*(*ctx).spu).problem.signal_notify1, data);
}

unsafe fn spu_hw_signal2_write(ctx: *mut spu_context, data: u32) {
    out_be32(&mut (*(*ctx).spu).problem.signal_notify2, data);
}

unsafe fn spu_hw_signal1_type_set(ctx: *mut spu_context, val: u64) {
    let spu = (*ctx).spu;
    let priv2 = (*spu).priv2;
    spin_lock_irq(&mut (*spu).register_lock);
    let mut tmp = in_be64(&(*priv2).spu_cfg_RW);
    if val != 0 { tmp |= 1; } else { tmp &= !1; }
    out_be64(&mut (*priv2).spu_cfg_RW, tmp);
    spin_unlock_irq(&mut (*spu).register_lock);
}

unsafe fn spu_hw_signal1_type_get(ctx: *mut spu_context) -> u64 {
    (in_be64(&(*(*ctx).spu).priv2.spu_cfg_RW) & 1 != 0) as u64
}

unsafe fn spu_hw_signal2_type_set(ctx: *mut spu_context, val: u64) {
    let spu = (*ctx).spu;
    let priv2 = (*spu).priv2;
    spin_lock_irq(&mut (*spu).register_lock);
    let mut tmp = in_be64(&(*priv2).spu_cfg_RW);
    if val != 0 { tmp |= 2; } else { tmp &= !2; }
    out_be64(&mut (*priv2).spu_cfg_RW, tmp);
    spin_unlock_irq(&mut (*spu).register_lock);
}

unsafe fn spu_hw_signal2_type_get(ctx: *mut spu_context) -> u64 {
    (in_be64(&(*(*ctx).spu).priv2.spu_cfg_RW) & 2 != 0) as u64
}

unsafe fn spu_hw_npc_read(ctx: *mut spu_context) -> u32 { in_be32(&(*(*ctx).spu).problem.spu_npc_RW) }
unsafe fn spu_hw_npc_write(ctx: *mut spu_context, val: u32) { out_be32(&mut (*(*ctx).spu).problem.spu_npc_RW, val); }
unsafe fn spu_hw_status_read(ctx: *mut spu_context) -> u32 { in_be32(&(*(*ctx).spu).problem.spu_status_R) }
unsafe fn spu_hw_get_ls(ctx: *mut spu_context) -> *mut i8 { (*(*ctx).spu).local_store }
unsafe fn spu_hw_privcntl_write(ctx: *mut spu_context, val: u64) { out_be64(&mut (*(*ctx).spu).priv2.spu_privcntl_RW, val); }
unsafe fn spu_hw_runcntl_read(ctx: *mut spu_context) -> u32 { in_be32(&(*(*ctx).spu).problem.spu_runcntl_RW) }

unsafe fn spu_hw_runcntl_write(ctx: *mut spu_context, val: u32) {
    let spu = (*ctx).spu;
    spin_lock_irq(&mut (*spu).register_lock);
    if val & SPU_RUNCNTL_ISOLATE != 0 { spu_hw_privcntl_write(ctx, SPU_PRIVCNTL_LOAD_REQUEST_ENABLE_MASK); }
    out_be32(&mut (*(*spu).problem).spu_runcntl_RW, val);
    spin_unlock_irq(&mut (*spu).register_lock);
}

unsafe fn spu_hw_runcntl_stop(ctx: *mut spu_context) {
    let spu = (*ctx).spu;
    spin_lock_irq(&mut (*spu).register_lock);
    out_be32(&mut (*(*spu).problem).spu_runcntl_RW, SPU_RUNCNTL_STOP);
    while in_be32(&(*(*spu).problem).spu_status_R) & SPU_STATUS_RUNNING != 0 { cpu_relax(); }
    spin_unlock_irq(&mut (*spu).register_lock);
}

unsafe fn spu_hw_master_start(ctx: *mut spu_context) {
    let spu = (*ctx).spu;
    spin_lock_irq(&mut (*spu).register_lock);
    let sr1 = spu_mfc_sr1_get(spu) | MFC_STATE1_MASTER_RUN_CONTROL_MASK;
    spu_mfc_sr1_set(spu, sr1);
    spin_unlock_irq(&mut (*spu).register_lock);
}

unsafe fn spu_hw_master_stop(ctx: *mut spu_context) {
    let spu = (*ctx).spu;
    spin_lock_irq(&mut (*spu).register_lock);
    let sr1 = spu_mfc_sr1_get(spu) & !MFC_STATE1_MASTER_RUN_CONTROL_MASK;
    spu_mfc_sr1_set(spu, sr1);
    spin_unlock_irq(&mut (*spu).register_lock);
}

unsafe fn spu_hw_set_mfc_query(ctx: *mut spu_context, mask: u32, mode: u32) -> i32 {
    let prob = (*(*ctx).spu).problem;
    spin_lock_irq(&mut (*(*ctx).spu).register_lock);
    let ret;
    if in_be32(&(*prob).dma_querytype_RW) != 0 { ret = -EAGAIN; }
    else { out_be32(&mut (*prob).dma_querymask_RW, mask); out_be32(&mut (*prob).dma_querytype_RW, mode); ret = 0; }
    spin_unlock_irq(&mut (*(*ctx).spu).register_lock);
    ret
}

unsafe fn spu_hw_read_mfc_tagstatus(ctx: *mut spu_context) -> u32 { in_be32(&(*(*ctx).spu).problem.dma_tagstatus_R) }
unsafe fn spu_hw_get_mfc_free_elements(ctx: *mut spu_context) -> u32 { in_be32(&(*(*ctx).spu).problem.dma_qstatus_R) }

unsafe fn spu_hw_send_mfc_command(ctx: *mut spu_context, cmd: *mut mfc_dma_command) -> i32 {
    let prob = (*(*ctx).spu).problem;
    spin_lock_irq(&mut (*(*ctx).spu).register_lock);
    out_be32(&mut (*prob).mfc_lsa_W, (*cmd).lsa);
    out_be64(&mut (*prob).mfc_ea_W, (*cmd).ea);
    out_be32(&mut (*prob).mfc_union_W.by32.mfc_size_tag32, (*cmd).size << 16 | (*cmd).tag);
    out_be32(&mut (*prob).mfc_union_W.by32.mfc_class_cmd32, (*cmd).class_ << 16 | (*cmd).cmd);
    let status = in_be32(&(*prob).mfc_union_W.by32.mfc_class_cmd32);
    spin_unlock_irq(&mut (*(*ctx).spu).register_lock);
    match status & 0xffff { 0 => 0, 2 => -EAGAIN, _ => -EINVAL }
}

unsafe fn spu_hw_restart_dma(ctx: *mut spu_context) {
    let priv2 = (*(*ctx).spu).priv2;
    if !test_bit(SPU_CONTEXT_SWITCH_PENDING, &(*(*ctx).spu).flags) { out_be64(&mut (*priv2).mfc_control_RW, MFC_CNTL_RESTART_DMA_COMMAND); }
}

#[repr(C)]
pub struct spu_context_ops {
    pub mbox_read: unsafe fn(*mut spu_context, *mut u32) -> i32,
    pub mbox_stat_read: unsafe fn(*mut spu_context) -> u32,
    pub mbox_stat_poll: unsafe fn(*mut spu_context, __poll_t) -> __poll_t,
    pub ibox_read: unsafe fn(*mut spu_context, *mut u32) -> i32,
    pub wbox_write: unsafe fn(*mut spu_context, u32) -> i32,
    pub signal1_write: unsafe fn(*mut spu_context, u32),
    pub signal2_write: unsafe fn(*mut spu_context, u32),
    pub signal1_type_set: unsafe fn(*mut spu_context, u64),
    pub signal1_type_get: unsafe fn(*mut spu_context) -> u64,
    pub signal2_type_set: unsafe fn(*mut spu_context, u64),
    pub signal2_type_get: unsafe fn(*mut spu_context) -> u64,
    pub npc_read: unsafe fn(*mut spu_context) -> u32,
    pub npc_write: unsafe fn(*mut spu_context, u32),
    pub status_read: unsafe fn(*mut spu_context) -> u32,
    pub get_ls: unsafe fn(*mut spu_context) -> *mut i8,
    pub privcntl_write: unsafe fn(*mut spu_context, u64),
    pub runcntl_read: unsafe fn(*mut spu_context) -> u32,
    pub runcntl_write: unsafe fn(*mut spu_context, u32),
    pub runcntl_stop: unsafe fn(*mut spu_context),
    pub master_start: unsafe fn(*mut spu_context),
    pub master_stop: unsafe fn(*mut spu_context),
    pub set_mfc_query: unsafe fn(*mut spu_context, u32, u32) -> i32,
    pub read_mfc_tagstatus: unsafe fn(*mut spu_context) -> u32,
    pub get_mfc_free_elements: unsafe fn(*mut spu_context) -> u32,
    pub send_mfc_command: unsafe fn(*mut spu_context, *mut mfc_dma_command) -> i32,
    pub restart_dma: unsafe fn(*mut spu_context),
}

pub static mut spu_hw_ops: spu_context_ops = spu_context_ops {
    mbox_read: spu_hw_mbox_read,
    mbox_stat_read: spu_hw_mbox_stat_read,
    mbox_stat_poll: spu_hw_mbox_stat_poll,
    ibox_read: spu_hw_ibox_read,
    wbox_write: spu_hw_wbox_write,
    signal1_write: spu_hw_signal1_write,
    signal2_write: spu_hw_signal2_write,
    signal1_type_set: spu_hw_signal1_type_set,
    signal1_type_get: spu_hw_signal1_type_get,
    signal2_type_set: spu_hw_signal2_type_set,
    signal2_type_get: spu_hw_signal2_type_get,
    npc_read: spu_hw_npc_read,
    npc_write: spu_hw_npc_write,
    status_read: spu_hw_status_read,
    get_ls: spu_hw_get_ls,
    privcntl_write: spu_hw_privcntl_write,
    runcntl_read: spu_hw_runcntl_read,
    runcntl_write: spu_hw_runcntl_write,
    runcntl_stop: spu_hw_runcntl_stop,
    master_start: spu_hw_master_start,
    master_stop: spu_hw_master_stop,
    set_mfc_query: spu_hw_set_mfc_query,
    read_mfc_tagstatus: spu_hw_read_mfc_tagstatus,
    get_mfc_free_elements: spu_hw_get_mfc_free_elements,
    send_mfc_command: spu_hw_send_mfc_command,
    restart_dma: spu_hw_restart_dma,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
