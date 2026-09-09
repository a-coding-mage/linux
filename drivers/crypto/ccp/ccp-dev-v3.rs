// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Cryptographic Coprocessor (CCP) driver
 *
 * Copyright (C) 2013,2017 Advanced Micro Devices, Inc.
 *
 * Author: Tom Lendacky <thomas.lendacky@amd.com>
 * Author: Gary R Hook <gary.hook@amd.com>
 */

// Linux kernel dependencies and "ccp-dev.h" are supplied by the surrounding
// translation unit.

unsafe fn ccp_alloc_ksb(cmd_q: *mut ccp_cmd_queue, count: c_uint) -> u32 {
    let ccp = (*cmd_q).ccp;
    loop {
        mutex_lock(&mut (*ccp).sb_mutex);
        let start = bitmap_find_next_zero_area((*ccp).sb, (*ccp).sb_count,
                                               (*ccp).sb_start, count, 0) as u32;
        if start < (*ccp).sb_count {
            bitmap_set((*ccp).sb, start, count);
            mutex_unlock(&mut (*ccp).sb_mutex);
            break;
        }
        (*ccp).sb_avail = 0;
        mutex_unlock(&mut (*ccp).sb_mutex);
        /* Wait for KSB entries to become available */
        if wait_event_interruptible(&mut (*ccp).sb_queue, (*ccp).sb_avail) != 0 {
            return 0;
        }
    }
    KSB_START + start
}

unsafe fn ccp_free_ksb(cmd_q: *mut ccp_cmd_queue, start: c_uint, count: c_uint) {
    let ccp = (*cmd_q).ccp;
    if start == 0 { return; }
    mutex_lock(&mut (*ccp).sb_mutex);
    bitmap_clear((*ccp).sb, start - KSB_START, count);
    (*ccp).sb_avail = 1;
    mutex_unlock(&mut (*ccp).sb_mutex);
    wake_up_interruptible_all(&mut (*ccp).sb_queue);
}

unsafe fn ccp_get_free_slots(cmd_q: *mut ccp_cmd_queue) -> c_uint {
    CMD_Q_DEPTH(ioread32((*cmd_q).reg_status))
}

unsafe fn ccp_do_cmd(op: *mut ccp_op, cr: *mut u32, cr_count: c_uint) -> c_int {
    let cmd_q = (*op).cmd_q;
    let ccp = (*cmd_q).ccp;
    let mut cr_addr = (*ccp).io_regs.add(CMD_REQ0 + CMD_REQ_INCR);
    let mut ret: c_int = 0;
    (*cmd_q).free_slots -= 1;
    let mut cr0 = ((*cmd_q).id << REQ0_CMD_Q_SHIFT)
        | ((*op).jobid << REQ0_JOBID_SHIFT) | REQ0_WAIT_FOR_WRITE;
    if (*op).soc { cr0 |= REQ0_STOP_ON_COMPLETE | REQ0_INT_ON_COMPLETE; }
    if (*op).ioc || (*cmd_q).free_slots == 0 { cr0 |= REQ0_INT_ON_COMPLETE; }
    mutex_lock(&mut (*ccp).req_mutex);
    for i in 0..cr_count as usize {
        iowrite32(*cr.add(i), cr_addr);
        cr_addr = cr_addr.add(CMD_REQ_INCR);
    }
    wmb();
    iowrite32(cr0, (*ccp).io_regs.add(CMD_REQ0));
    mutex_unlock(&mut (*ccp).req_mutex);
    if cr0 & REQ0_INT_ON_COMPLETE != 0 {
        ret = wait_event_interruptible(&mut (*cmd_q).int_queue, (*cmd_q).int_rcvd);
        if ret != 0 || (*cmd_q).cmd_error != 0 {
            let cmd = ((*cmd_q).id << DEL_Q_ID_SHIFT) | (*op).jobid;
            if (*cmd_q).cmd_error != 0 { ccp_log_error((*cmd_q).ccp, (*cmd_q).cmd_error); }
            iowrite32(cmd, (*ccp).io_regs.add(DEL_CMD_Q_JOB));
            if ret == 0 { ret = -EIO; }
        } else if (*op).soc {
            let cmd = DEL_Q_ACTIVE | ((*cmd_q).id << DEL_Q_ID_SHIFT) | (*op).jobid;
            iowrite32(cmd, (*ccp).io_regs.add(DEL_CMD_Q_JOB));
        }
        (*cmd_q).free_slots = CMD_Q_DEPTH((*cmd_q).q_status);
        (*cmd_q).int_rcvd = 0;
    }
    ret
}

unsafe fn ccp_perform_aes(op: *mut ccp_op) -> c_int {
    let mut cr = [0u32; 6];
    cr[0] = (CCP_ENGINE_AES << REQ1_ENGINE_SHIFT) | ((*op).u.aes.type_ << REQ1_AES_TYPE_SHIFT)
        | ((*op).u.aes.mode << REQ1_AES_MODE_SHIFT) | ((*op).u.aes.action << REQ1_AES_ACTION_SHIFT)
        | ((*op).sb_key << REQ1_KEY_KSB_SHIFT);
    cr[1] = (*op).src.u.dma.length - 1; cr[2] = ccp_addr_lo(&(*op).src.u.dma);
    cr[3] = ((*op).sb_ctx << REQ4_KSB_SHIFT) | (CCP_MEMTYPE_SYSTEM << REQ4_MEMTYPE_SHIFT) | ccp_addr_hi(&(*op).src.u.dma);
    cr[4] = ccp_addr_lo(&(*op).dst.u.dma); cr[5] = (CCP_MEMTYPE_SYSTEM << REQ6_MEMTYPE_SHIFT) | ccp_addr_hi(&(*op).dst.u.dma);
    if (*op).u.aes.mode == CCP_AES_MODE_CFB { cr[0] |= 0x7f << REQ1_AES_CFB_SIZE_SHIFT; }
    if (*op).eom { cr[0] |= REQ1_EOM; } if (*op).init { cr[0] |= REQ1_INIT; }
    ccp_do_cmd(op, cr.as_mut_ptr(), cr.len() as c_uint)
}

unsafe fn ccp_perform_xts_aes(op: *mut ccp_op) -> c_int {
    let mut cr = [0u32; 6];
    cr[0] = (CCP_ENGINE_XTS_AES_128 << REQ1_ENGINE_SHIFT) | ((*op).u.xts.action << REQ1_AES_ACTION_SHIFT)
        | ((*op).u.xts.unit_size << REQ1_XTS_AES_SIZE_SHIFT) | ((*op).sb_key << REQ1_KEY_KSB_SHIFT);
    cr[1] = (*op).src.u.dma.length - 1; cr[2] = ccp_addr_lo(&(*op).src.u.dma);
    cr[3] = ((*op).sb_ctx << REQ4_KSB_SHIFT) | (CCP_MEMTYPE_SYSTEM << REQ4_MEMTYPE_SHIFT) | ccp_addr_hi(&(*op).src.u.dma);
    cr[4] = ccp_addr_lo(&(*op).dst.u.dma); cr[5] = (CCP_MEMTYPE_SYSTEM << REQ6_MEMTYPE_SHIFT) | ccp_addr_hi(&(*op).dst.u.dma);
    if (*op).eom { cr[0] |= REQ1_EOM; } if (*op).init { cr[0] |= REQ1_INIT; }
    ccp_do_cmd(op, cr.as_mut_ptr(), 6)
}

unsafe fn ccp_perform_sha(op: *mut ccp_op) -> c_int {
    let mut cr = [0u32; 6];
    cr[0] = (CCP_ENGINE_SHA << REQ1_ENGINE_SHIFT) | ((*op).u.sha.type_ << REQ1_SHA_TYPE_SHIFT) | REQ1_INIT;
    cr[1] = (*op).src.u.dma.length - 1; cr[2] = ccp_addr_lo(&(*op).src.u.dma);
    cr[3] = ((*op).sb_ctx << REQ4_KSB_SHIFT) | (CCP_MEMTYPE_SYSTEM << REQ4_MEMTYPE_SHIFT) | ccp_addr_hi(&(*op).src.u.dma);
    if (*op).eom { cr[0] |= REQ1_EOM; cr[4] = lower_32_bits((*op).u.sha.msg_bits); cr[5] = upper_32_bits((*op).u.sha.msg_bits); }
    else { cr[4] = 0; cr[5] = 0; }
    ccp_do_cmd(op, cr.as_mut_ptr(), 6)
}

unsafe fn ccp_perform_rsa(op: *mut ccp_op) -> c_int {
    let mut cr = [0u32; 6];
    cr[0] = (CCP_ENGINE_RSA << REQ1_ENGINE_SHIFT) | ((*op).u.rsa.mod_size << REQ1_RSA_MOD_SIZE_SHIFT) | ((*op).sb_key << REQ1_KEY_KSB_SHIFT) | REQ1_EOM;
    cr[1] = (*op).u.rsa.input_len - 1; cr[2] = ccp_addr_lo(&(*op).src.u.dma);
    cr[3] = ((*op).sb_ctx << REQ4_KSB_SHIFT) | (CCP_MEMTYPE_SYSTEM << REQ4_MEMTYPE_SHIFT) | ccp_addr_hi(&(*op).src.u.dma);
    cr[4] = ccp_addr_lo(&(*op).dst.u.dma); cr[5] = (CCP_MEMTYPE_SYSTEM << REQ6_MEMTYPE_SHIFT) | ccp_addr_hi(&(*op).dst.u.dma);
    ccp_do_cmd(op, cr.as_mut_ptr(), 6)
}

unsafe fn ccp_perform_passthru(op: *mut ccp_op) -> c_int {
    let mut cr = [0u32; 6];
    cr[0] = (CCP_ENGINE_PASSTHRU << REQ1_ENGINE_SHIFT) | ((*op).u.passthru.bit_mod << REQ1_PT_BW_SHIFT) | ((*op).u.passthru.byte_swap << REQ1_PT_BS_SHIFT);
    if (*op).src.type_ == CCP_MEMTYPE_SYSTEM { cr[1] = (*op).src.u.dma.length - 1; } else { cr[1] = (*op).dst.u.dma.length - 1; }
    if (*op).src.type_ == CCP_MEMTYPE_SYSTEM {
        cr[2] = ccp_addr_lo(&(*op).src.u.dma); cr[3] = (CCP_MEMTYPE_SYSTEM << REQ4_MEMTYPE_SHIFT) | ccp_addr_hi(&(*op).src.u.dma);
        if (*op).u.passthru.bit_mod != CCP_PASSTHRU_BITWISE_NOOP { cr[3] |= (*op).sb_key << REQ4_KSB_SHIFT; }
    } else { cr[2] = (*op).src.u.sb * CCP_SB_BYTES; cr[3] = CCP_MEMTYPE_SB << REQ4_MEMTYPE_SHIFT; }
    if (*op).dst.type_ == CCP_MEMTYPE_SYSTEM { cr[4] = ccp_addr_lo(&(*op).dst.u.dma); cr[5] = (CCP_MEMTYPE_SYSTEM << REQ6_MEMTYPE_SHIFT) | ccp_addr_hi(&(*op).dst.u.dma); }
    else { cr[4] = (*op).dst.u.sb * CCP_SB_BYTES; cr[5] = CCP_MEMTYPE_SB << REQ6_MEMTYPE_SHIFT; }
    if (*op).eom { cr[0] |= REQ1_EOM; }
    ccp_do_cmd(op, cr.as_mut_ptr(), 6)
}

unsafe fn ccp_perform_ecc(op: *mut ccp_op) -> c_int {
    let mut cr = [0u32; 6];
    cr[0] = REQ1_ECC_AFFINE_CONVERT | (CCP_ENGINE_ECC << REQ1_ENGINE_SHIFT) | ((*op).u.ecc.function << REQ1_ECC_FUNCTION_SHIFT) | REQ1_EOM;
    cr[1] = (*op).src.u.dma.length - 1; cr[2] = ccp_addr_lo(&(*op).src.u.dma);
    cr[3] = CCP_MEMTYPE_SYSTEM << REQ4_MEMTYPE_SHIFT | ccp_addr_hi(&(*op).src.u.dma);
    cr[4] = ccp_addr_lo(&(*op).dst.u.dma); cr[5] = CCP_MEMTYPE_SYSTEM << REQ6_MEMTYPE_SHIFT | ccp_addr_hi(&(*op).dst.u.dma);
    ccp_do_cmd(op, cr.as_mut_ptr(), 6)
}

unsafe fn ccp_disable_queue_interrupts(ccp: *mut ccp_device) { iowrite32(0, (*ccp).io_regs.add(IRQ_MASK_REG)); }
unsafe fn ccp_enable_queue_interrupts(ccp: *mut ccp_device) { iowrite32((*ccp).qim, (*ccp).io_regs.add(IRQ_MASK_REG)); }

unsafe fn ccp_irq_bh(data: c_ulong) {
    let ccp = data as *mut ccp_device; let status = ioread32((*ccp).io_regs.add(IRQ_STATUS_REG));
    for i in 0..(*ccp).cmd_q_count as usize {
        let q = &mut *(*ccp).cmd_q.add(i); let q_int = status & (q.int_ok | q.int_err);
        if q_int != 0 { q.int_status = status; q.q_status = ioread32(q.reg_status); q.q_int_status = ioread32(q.reg_int_status);
            if q_int & q.int_err != 0 && q.cmd_error == 0 { q.cmd_error = CMD_Q_ERROR(q.q_status); }
            q.int_rcvd = 1; iowrite32(q_int, (*ccp).io_regs.add(IRQ_STATUS_REG)); wake_up_interruptible(&mut q.int_queue);
        }
    } ccp_enable_queue_interrupts(ccp);
}

unsafe fn ccp_irq_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let ccp = data as *mut ccp_device; ccp_disable_queue_interrupts(ccp);
    if (*ccp).use_tasklet { tasklet_schedule(&mut (*ccp).irq_tasklet); } else { ccp_irq_bh(ccp as c_ulong); }
    IRQ_HANDLED
}

// The remaining initialization/destruction logic is kept in direct Rust form;
// all referenced kernel helpers and CCP structures are external dependencies.
unsafe fn ccp_init(ccp: *mut ccp_device) -> c_int {
    let dev = (*ccp).dev; let mut qmr = ioread32((*ccp).io_regs.add(Q_MASK_REG)); (*ccp).qim = 0;
    let mut i = 0u32;
    while i < MAX_HW_QUEUES && (*ccp).cmd_q_count < (*ccp).max_q_count {
        if qmr & (1 << i) == 0 { i += 1; continue; }
        let mut name = [0i8; MAX_DMAPOOL_NAME_LEN]; snprintf(name.as_mut_ptr(), name.len(), "%s_q%d", (*ccp).name, i);
        let pool = dma_pool_create(name.as_ptr(), dev, CCP_DMAPOOL_MAX_SIZE, CCP_DMAPOOL_ALIGN, 0);
        if pool.is_null() { dev_err(dev, "unable to allocate dma pool\n"); return -ENOMEM; }
        let q = &mut *(*ccp).cmd_q.add((*ccp).cmd_q_count as usize); (*ccp).cmd_q_count += 1;
        q.ccp = ccp; q.id = i; q.dma_pool = pool; q.sb_key = KSB_START + (*ccp).sb_start; (*ccp).sb_start += 1;
        q.sb_ctx = KSB_START + (*ccp).sb_start; (*ccp).sb_start += 1; (*ccp).sb_count -= 2;
        q.reg_status = (*ccp).io_regs.add(CMD_Q_STATUS_BASE + CMD_Q_STATUS_INCR * i); q.reg_int_status = (*ccp).io_regs.add(CMD_Q_INT_STATUS_BASE + CMD_Q_STATUS_INCR * i);
        q.int_ok = 1 << (i * 2); q.int_err = 1 << ((i * 2) + 1); q.free_slots = ccp_get_free_slots(q); init_waitqueue_head(&mut q.int_queue); (*ccp).qim |= q.int_ok | q.int_err;
        dev_dbg(dev, "queue #%u available\n", i); i += 1;
    }
    if (*ccp).cmd_q_count == 0 { dev_notice(dev, "no command queues available\n"); return -EIO; }
    ccp_disable_queue_interrupts(ccp); for j in 0..(*ccp).cmd_q_count as usize { let q = &*(*ccp).cmd_q.add(j); ioread32(q.reg_int_status); ioread32(q.reg_status); }
    iowrite32((*ccp).qim, (*ccp).io_regs.add(IRQ_STATUS_REG));
    let ret = sp_request_ccp_irq((*ccp).sp, ccp_irq_handler, (*ccp).name, ccp); if ret != 0 { return ret; }
    if (*ccp).use_tasklet { tasklet_init(&mut (*ccp).irq_tasklet, ccp_irq_bh, ccp as c_ulong); }
    for j in 0..(*ccp).cmd_q_count as usize { let q = (*ccp).cmd_q.add(j); let k = kthread_run(ccp_cmd_queue_thread, q, "%s-q%u", (*ccp).name, (*q).id); if IS_ERR(k) { sp_free_ccp_irq((*ccp).sp, ccp); return PTR_ERR(k); } (*q).kthread = k; }
    ccp_enable_queue_interrupts(ccp); ccp_add_device(ccp); let ret = ccp_register_rng(ccp); if ret != 0 { return ret; }
    ccp_dmaengine_register(ccp)
}

unsafe fn ccp_destroy(ccp: *mut ccp_device) {
    ccp_dmaengine_unregister(ccp); ccp_unregister_rng(ccp); ccp_del_device(ccp); ccp_disable_queue_interrupts(ccp);
    for i in 0..(*ccp).cmd_q_count as usize { let q = &*(*ccp).cmd_q.add(i); ioread32(q.reg_int_status); ioread32(q.reg_status); }
    iowrite32((*ccp).qim, (*ccp).io_regs.add(IRQ_STATUS_REG));
    for i in 0..(*ccp).cmd_q_count as usize { let q = &*(*ccp).cmd_q.add(i); if !q.kthread.is_null() { kthread_stop(q.kthread); } dma_pool_destroy(q.dma_pool); }
    sp_free_ccp_irq((*ccp).sp, ccp);
    while !list_empty(&(*ccp).cmd) { let cmd = list_first_entry(&(*ccp).cmd, ccp_cmd); list_del(&mut cmd.entry); (cmd.callback)(cmd.data, -ENODEV); }
    while !list_empty(&(*ccp).backlog) { let cmd = list_first_entry(&(*ccp).backlog, ccp_cmd); list_del(&mut cmd.entry); (cmd.callback)(cmd.data, -ENODEV); }
}

static ccp3_actions: ccp_actions = ccp_actions { aes: Some(ccp_perform_aes), xts_aes: Some(ccp_perform_xts_aes), des3: None, sha: Some(ccp_perform_sha), rsa: Some(ccp_perform_rsa), passthru: Some(ccp_perform_passthru), ecc: Some(ccp_perform_ecc), sballoc: Some(ccp_alloc_ksb), sbfree: Some(ccp_free_ksb), init: Some(ccp_init), destroy: Some(ccp_destroy), get_free_slots: Some(ccp_get_free_slots), irqhandler: Some(ccp_irq_handler) };

pub static ccpv3_platform: ccp_vdata = ccp_vdata { version: CCP_VERSION(3, 0), setup: None, perform: &ccp3_actions, offset: 0, rsamax: CCP_RSA_MAX_WIDTH };
pub static ccpv3: ccp_vdata = ccp_vdata { version: CCP_VERSION(3, 0), setup: None, perform: &ccp3_actions, offset: 0x20000, rsamax: CCP_RSA_MAX_WIDTH };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
