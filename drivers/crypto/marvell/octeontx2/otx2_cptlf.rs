// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2020 Marvell. */

// Dependencies are supplied by the surrounding kernel translation.

const CPT_TIMER_HOLD: i32 = 0x03F;
const CPT_COUNT_HOLD: i32 = 32;

unsafe fn cptlf_do_set_done_time_wait(lf: *mut otx2_cptlf_info, time_wait: i32) {
    let mut done_wait: otx2_cptx_lf_done_wait = core::mem::zeroed();
    done_wait.u = otx2_cpt_read64((*(*lf).lfs).reg_base, (*(*lf).lfs).blkaddr,
                                  (*lf).slot, OTX2_CPT_LF_DONE_WAIT);
    done_wait.s.time_wait = time_wait;
    otx2_cpt_write64((*(*lf).lfs).reg_base, (*(*lf).lfs).blkaddr, (*lf).slot,
                     OTX2_CPT_LF_DONE_WAIT, done_wait.u);
}

unsafe fn cptlf_do_set_done_num_wait(lf: *mut otx2_cptlf_info, num_wait: i32) {
    let mut done_wait: otx2_cptx_lf_done_wait = core::mem::zeroed();
    done_wait.u = otx2_cpt_read64((*(*lf).lfs).reg_base, (*(*lf).lfs).blkaddr,
                                  (*lf).slot, OTX2_CPT_LF_DONE_WAIT);
    done_wait.s.num_wait = num_wait;
    otx2_cpt_write64((*(*lf).lfs).reg_base, (*(*lf).lfs).blkaddr, (*lf).slot,
                     OTX2_CPT_LF_DONE_WAIT, done_wait.u);
}

unsafe fn cptlf_set_done_time_wait(lfs: *mut otx2_cptlfs_info, time_wait: i32) {
    for slot in 0..(*lfs).lfs_num {
        cptlf_do_set_done_time_wait((*lfs).lf.add(slot as usize), time_wait);
    }
}

unsafe fn cptlf_set_done_num_wait(lfs: *mut otx2_cptlfs_info, num_wait: i32) {
    for slot in 0..(*lfs).lfs_num {
        cptlf_do_set_done_num_wait((*lfs).lf.add(slot as usize), num_wait);
    }
}

unsafe fn cptlf_set_pri(lf: *mut otx2_cptlf_info, pri: i32) -> i32 {
    let lfs = (*lf).lfs;
    let mut lf_ctrl: otx2_cptx_af_lf_ctrl = core::mem::zeroed();
    let mut ret = otx2_cpt_read_af_reg((*lfs).mbox, (*lfs).pdev,
                                       CPT_AF_LFX_CTL((*lf).slot), &mut lf_ctrl.u,
                                       (*lfs).blkaddr);
    if ret != 0 { return ret; }
    lf_ctrl.s.pri = if pri != 0 { 1 } else { 0 };
    ret = otx2_cpt_write_af_reg((*lfs).mbox, (*lfs).pdev,
                                CPT_AF_LFX_CTL((*lf).slot), lf_ctrl.u,
                                (*lfs).blkaddr);
    ret
}

unsafe fn cptlf_set_eng_grps_mask(lf: *mut otx2_cptlf_info, eng_grps_mask: i32) -> i32 {
    let lfs = (*lf).lfs;
    let mut lf_ctrl: otx2_cptx_af_lf_ctrl = core::mem::zeroed();
    let mut ret = otx2_cpt_read_af_reg((*lfs).mbox, (*lfs).pdev,
                                       CPT_AF_LFX_CTL((*lf).slot), &mut lf_ctrl.u,
                                       (*lfs).blkaddr);
    if ret != 0 { return ret; }
    lf_ctrl.s.grp = eng_grps_mask;
    ret = otx2_cpt_write_af_reg((*lfs).mbox, (*lfs).pdev,
                                CPT_AF_LFX_CTL((*lf).slot), lf_ctrl.u,
                                (*lfs).blkaddr);
    ret
}

unsafe fn cptlf_set_grp_and_pri(lfs: *mut otx2_cptlfs_info, eng_grp_mask: i32, pri: i32) -> i32 {
    let mut ret = 0;
    for slot in 0..(*lfs).lfs_num {
        ret = cptlf_set_pri((*lfs).lf.add(slot as usize), pri);
        if ret != 0 { return ret; }
        ret = cptlf_set_eng_grps_mask((*lfs).lf.add(slot as usize), eng_grp_mask);
        if ret != 0 { return ret; }
    }
    ret
}

unsafe fn cptlf_set_ctx_ilen(lfs: *mut otx2_cptlfs_info, ctx_ilen: i32) -> i32 {
    let mut lf_ctrl: otx2_cptx_af_lf_ctrl = core::mem::zeroed();
    let mut ret = 0;
    for slot in 0..(*lfs).lfs_num {
        let lf = (*lfs).lf.add(slot as usize);
        ret = otx2_cpt_read_af_reg((*lfs).mbox, (*lfs).pdev,
                                   CPT_AF_LFX_CTL((*lf).slot), &mut lf_ctrl.u,
                                   (*lfs).blkaddr);
        if ret != 0 { return ret; }
        lf_ctrl.s.ctx_ilen = ctx_ilen;
        ret = otx2_cpt_write_af_reg((*lfs).mbox, (*lfs).pdev,
                                    CPT_AF_LFX_CTL((*lf).slot), lf_ctrl.u,
                                    (*lfs).blkaddr);
        if ret != 0 { return ret; }
    }
    ret
}

unsafe fn cptlf_hw_init(lfs: *mut otx2_cptlfs_info) {
    otx2_cptlf_disable_iqueues(lfs);
    otx2_cptlf_set_iqueues_base_addr(lfs);
    otx2_cptlf_set_iqueues_size(lfs);
    cptlf_set_done_time_wait(lfs, CPT_TIMER_HOLD);
    cptlf_set_done_num_wait(lfs, CPT_COUNT_HOLD);
    otx2_cptlf_enable_iqueues(lfs);
}

unsafe fn cptlf_hw_cleanup(lfs: *mut otx2_cptlfs_info) {
    otx2_cptlf_disable_iqueues(lfs);
}

unsafe fn cptlf_set_misc_intrs(lfs: *mut otx2_cptlfs_info, enable: u8) {
    let mut irq_misc: otx2_cptx_lf_misc_int_ena_w1s = core::mem::zeroed();
    irq_misc.u = 0;
    let reg = if enable != 0 { OTX2_CPT_LF_MISC_INT_ENA_W1S } else { OTX2_CPT_LF_MISC_INT_ENA_W1C };
    irq_misc.s.fault = 1;
    irq_misc.s.hwerr = 1;
    irq_misc.s.irde = 1;
    irq_misc.s.nqerr = 1;
    irq_misc.s.nwrp = 1;
    for slot in 0..(*lfs).lfs_num {
        otx2_cpt_write64((*lfs).reg_base, (*lfs).blkaddr, slot, reg, irq_misc.u);
    }
}

unsafe fn cptlf_set_done_intrs(lfs: *mut otx2_cptlfs_info, enable: u8) {
    let reg = if enable != 0 { OTX2_CPT_LF_DONE_INT_ENA_W1S } else { OTX2_CPT_LF_DONE_INT_ENA_W1C };
    for slot in 0..(*lfs).lfs_num {
        otx2_cpt_write64((*lfs).reg_base, (*lfs).blkaddr, slot, reg, 1);
    }
}

unsafe fn cptlf_read_done_cnt(lf: *mut otx2_cptlf_info) -> i32 {
    let mut irq_cnt: otx2_cptx_lf_done = core::mem::zeroed();
    irq_cnt.u = otx2_cpt_read64((*(*lf).lfs).reg_base, (*(*lf).lfs).blkaddr,
                                (*lf).slot, OTX2_CPT_LF_DONE);
    irq_cnt.s.done
}

unsafe extern "C" fn cptlf_misc_intr_handler(_irq: i32, arg: *mut core::ffi::c_void) -> irqreturn_t {
    let lf = arg as *mut otx2_cptlf_info;
    let mut irq_misc: otx2_cptx_lf_misc_int = core::mem::zeroed();
    let mut irq_misc_ack: otx2_cptx_lf_misc_int = core::mem::zeroed();
    irq_misc.u = otx2_cpt_read64((*(*lf).lfs).reg_base, (*(*lf).lfs).blkaddr,
                                 (*lf).slot, OTX2_CPT_LF_MISC_INT);
    irq_misc_ack.u = 0;
    if irq_misc.s.fault != 0 {
        dev_err(&(*(*lf).lfs).pdev.dev, "Memory error detected while executing CPT_INST_S, LF %d.\n", (*lf).slot);
        irq_misc_ack.s.fault = 1;
    } else if irq_misc.s.hwerr != 0 {
        dev_err(&(*(*lf).lfs).pdev.dev, "HW error from an engine executing CPT_INST_S, LF %d.", (*lf).slot);
        irq_misc_ack.s.hwerr = 1;
    } else if irq_misc.s.nwrp != 0 {
        dev_err(&(*(*lf).lfs).pdev.dev, "SMMU fault while writing CPT_RES_S to CPT_INST_S[RES_ADDR], LF %d.\n", (*lf).slot);
        irq_misc_ack.s.nwrp = 1;
    } else if irq_misc.s.irde != 0 {
        dev_err(&(*(*lf).lfs).pdev.dev, "Memory error when accessing instruction memory queue CPT_LF_Q_BASE[ADDR].\n");
        irq_misc_ack.s.irde = 1;
    } else if irq_misc.s.nqerr != 0 {
        dev_err(&(*(*lf).lfs).pdev.dev, "Error enqueuing an instruction received at CPT_LF_NQ.\n");
        irq_misc_ack.s.nqerr = 1;
    } else {
        dev_err(&(*(*lf).lfs).pdev.dev, "Unhandled interrupt in CPT LF %d\n", (*lf).slot);
        return IRQ_NONE;
    }
    otx2_cpt_write64((*(*lf).lfs).reg_base, (*(*lf).lfs).blkaddr, (*lf).slot,
                     OTX2_CPT_LF_MISC_INT, irq_misc_ack.u);
    IRQ_HANDLED
}

unsafe extern "C" fn cptlf_done_intr_handler(_irq: i32, arg: *mut core::ffi::c_void) -> irqreturn_t {
    let lf = arg as *mut otx2_cptlf_info;
    let irq_cnt = cptlf_read_done_cnt(lf);
    if irq_cnt != 0 {
        let mut done_wait: otx2_cptx_lf_done_wait = core::mem::zeroed();
        done_wait.u = otx2_cpt_read64((*(*lf).lfs).reg_base, (*(*lf).lfs).blkaddr,
                                      (*lf).slot, OTX2_CPT_LF_DONE_WAIT);
        otx2_cpt_write64((*(*lf).lfs).reg_base, (*(*lf).lfs).blkaddr, (*lf).slot,
                         OTX2_CPT_LF_DONE_ACK, irq_cnt as u64);
        otx2_cpt_write64((*(*lf).lfs).reg_base, (*(*lf).lfs).blkaddr, (*lf).slot,
                         OTX2_CPT_LF_DONE_WAIT, done_wait.u);
        if (*lf).wqe.is_null() {
            dev_err(&(*(*lf).lfs).pdev.dev, "No work for LF %d\n", (*lf).slot);
            return IRQ_NONE;
        }
        tasklet_hi_schedule(&mut (*(*lf).wqe).work);
    }
    IRQ_HANDLED
}

pub unsafe fn otx2_cptlf_unregister_misc_interrupts(lfs: *mut otx2_cptlfs_info) {
    let irq_offs = OTX2_CPT_LF_INT_VEC_E_MISC;
    for i in 0..(*lfs).lfs_num {
        let lf = (*lfs).lf.add(i as usize);
        if !(*lf).is_irq_reg[irq_offs as usize] { continue; }
        let vector = pci_irq_vector((*lfs).pdev, (*lf).msix_offset + irq_offs);
        free_irq(vector, lf as *mut core::ffi::c_void);
        (*lf).is_irq_reg[irq_offs as usize] = false;
    }
    cptlf_set_misc_intrs(lfs, 0);
}

pub unsafe fn otx2_cptlf_unregister_done_interrupts(lfs: *mut otx2_cptlfs_info) {
    let irq_offs = OTX2_CPT_LF_INT_VEC_E_DONE;
    for i in 0..(*lfs).lfs_num {
        let lf = (*lfs).lf.add(i as usize);
        if !(*lf).is_irq_reg[irq_offs as usize] { continue; }
        let vector = pci_irq_vector((*lfs).pdev, (*lf).msix_offset + irq_offs);
        free_irq(vector, lf as *mut core::ffi::c_void);
        (*lf).is_irq_reg[irq_offs as usize] = false;
    }
    cptlf_set_done_intrs(lfs, 0);
}

unsafe fn cptlf_do_register_interrrupts(lfs: *mut otx2_cptlfs_info, lf_num: i32,
                                        irq_offset: i32, handler: irq_handler_t) -> i32 {
    let lf = (*lfs).lf.add(lf_num as usize);
    let vector = pci_irq_vector((*lfs).pdev, (*lf).msix_offset + irq_offset);
    let ret = request_irq(vector, handler, 0, (*lf).irq_name[irq_offset as usize].as_mut_ptr(), lf as *mut core::ffi::c_void);
    if ret != 0 { return ret; }
    (*lf).is_irq_reg[irq_offset as usize] = true;
    ret
}

pub unsafe fn otx2_cptlf_register_misc_interrupts(lfs: *mut otx2_cptlfs_info) -> i32 {
    let is_cpt1 = ((*lfs).blkaddr == BLKADDR_CPT1) as i32;
    let irq_offs = OTX2_CPT_LF_INT_VEC_E_MISC;
    let mut ret = 0;
    for i in 0..(*lfs).lfs_num {
        snprintf((*lfs).lf.add(i as usize).irq_name[irq_offs as usize].as_mut_ptr(), 32,
                 b"CPT%dLF Misc%d\0".as_ptr() as *const i8, is_cpt1, i);
        ret = cptlf_do_register_interrrupts(lfs, i, irq_offs, cptlf_misc_intr_handler);
        if ret != 0 { otx2_cptlf_unregister_misc_interrupts(lfs); return ret; }
    }
    cptlf_set_misc_intrs(lfs, 1);
    0
}

pub unsafe fn otx2_cptlf_register_done_interrupts(lfs: *mut otx2_cptlfs_info) -> i32 {
    let is_cpt1 = ((*lfs).blkaddr == BLKADDR_CPT1) as i32;
    let irq_offs = OTX2_CPT_LF_INT_VEC_E_DONE;
    let mut ret = 0;
    for i in 0..(*lfs).lfs_num {
        snprintf((*lfs).lf.add(i as usize).irq_name[irq_offs as usize].as_mut_ptr(), 32,
                 b"OTX2_CPT%dLF Done%d\0".as_ptr() as *const i8, is_cpt1, i);
        ret = cptlf_do_register_interrrupts(lfs, i, irq_offs, cptlf_done_intr_handler);
        if ret != 0 { otx2_cptlf_unregister_done_interrupts(lfs); return ret; }
    }
    cptlf_set_done_intrs(lfs, 1);
    0
}

pub unsafe fn otx2_cptlf_free_irqs_affinity(lfs: *mut otx2_cptlfs_info) {
    for slot in 0..(*lfs).lfs_num {
        let lf = (*lfs).lf.add(slot as usize);
        for offs in 0..OTX2_CPT_LF_MSIX_VECTORS {
            irq_set_affinity_hint(pci_irq_vector((*lfs).pdev, (*lf).msix_offset + offs), core::ptr::null_mut());
        }
        free_cpumask_var((*lf).affinity_mask);
    }
}

pub unsafe fn otx2_cptlf_set_irqs_affinity(lfs: *mut otx2_cptlfs_info) -> i32 {
    let mut ret = 0;
    for slot in 0..(*lfs).lfs_num {
        let lf = (*lfs).lf.add(slot as usize);
        if !zalloc_cpumask_var(&mut (*lf).affinity_mask, GFP_KERNEL) {
            dev_err(&(*lfs).pdev.dev, "cpumask allocation failed for LF %d", slot);
            ret = -ENOMEM;
            otx2_cptlf_free_irqs_affinity(lfs);
            return ret;
        }
        cpumask_set_cpu(cpumask_local_spread(slot, dev_to_node(&(*lfs).pdev.dev)), (*lf).affinity_mask);
        for offs in 0..OTX2_CPT_LF_MSIX_VECTORS {
            ret = irq_set_affinity_hint(pci_irq_vector((*lfs).pdev, (*lf).msix_offset + offs), (*lf).affinity_mask);
            if ret != 0 { otx2_cptlf_free_irqs_affinity(lfs); return ret; }
        }
    }
    0
}

pub unsafe fn otx2_cptlf_init(lfs: *mut otx2_cptlfs_info, eng_grp_mask: u8, pri: i32, lfs_num: i32) -> i32 {
    if (*lfs).pdev.is_null() || (*lfs).reg_base.is_null() { return -EINVAL; }
    (*lfs).lfs_num = lfs_num;
    for slot in 0..(*lfs).lfs_num {
        let lf = (*lfs).lf.add(slot as usize);
        (*lf).lfs = lfs;
        (*lf).slot = slot;
        if (*lfs).lmt_info.base.is_null() {
            (*lf).lmtline = (*lfs).reg_base.add(OTX2_CPT_RVU_FUNC_ADDR_S(BLKADDR_LMT, slot, OTX2_CPT_LMT_LF_LMTLINEX(0)) as usize);
        }
        (*lf).ioreg = (*lfs).reg_base.add(OTX2_CPT_RVU_FUNC_ADDR_S((*lfs).blkaddr, slot, OTX2_CPT_LF_NQX(0)) as usize);
    }
    let mut ret = otx2_cpt_attach_rscrs_msg(lfs);
    if ret != 0 { (*lfs).lfs_num = 0; return ret; }
    ret = otx2_cpt_alloc_instruction_queues(lfs);
    if ret != 0 { dev_err(&(*lfs).pdev.dev, "Allocating instruction queues failed\n"); otx2_cpt_detach_rsrcs_msg(lfs); (*lfs).lfs_num = 0; return ret; }
    cptlf_hw_init(lfs);
    ret = cptlf_set_grp_and_pri(lfs, eng_grp_mask as i32, pri);
    if ret != 0 { cptlf_hw_cleanup(lfs); otx2_cpt_free_instruction_queues(lfs); otx2_cpt_detach_rsrcs_msg(lfs); (*lfs).lfs_num = 0; return ret; }
    if (*lfs).ctx_ilen_ovrd {
        ret = cptlf_set_ctx_ilen(lfs, (*lfs).ctx_ilen);
        if ret != 0 { cptlf_hw_cleanup(lfs); otx2_cpt_free_instruction_queues(lfs); otx2_cpt_detach_rsrcs_msg(lfs); (*lfs).lfs_num = 0; return ret; }
    }
    0
}

pub unsafe fn otx2_cptlf_shutdown(lfs: *mut otx2_cptlfs_info) {
    cptlf_hw_cleanup(lfs);
    otx2_cpt_free_instruction_queues(lfs);
    otx2_cpt_detach_rsrcs_msg(lfs);
    (*lfs).lfs_num = 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
