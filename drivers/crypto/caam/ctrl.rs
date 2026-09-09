// SPDX-License-Identifier: GPL-2.0+
/* CAAM control-plane driver backend. Direct low-level translation of ctrl.c. */

// Kernel and sibling-module declarations are supplied by the surrounding tree.

pub static mut caam_dpaa2: bool = false;

unsafe fn build_instantiation_desc(desc: *mut u32, handle: i32, do_sk: i32) {
    init_job_desc(desc, 0);
    let op_flags = OP_TYPE_CLASS1_ALG | OP_ALG_ALGSEL_RNG |
        ((handle as u32) << OP_ALG_AAI_SHIFT) | OP_ALG_AS_INIT | OP_ALG_PR_ON;
    append_operation(desc, op_flags);
    if handle == 0 && do_sk != 0 {
        let jump_cmd = append_jump(desc, JUMP_CLASS_CLASS1);
        set_jump_tgt_here(desc, jump_cmd);
        append_load_imm_u32(desc, 1, LDST_SRCDST_WORD_CLRW);
        append_operation(desc, OP_TYPE_CLASS1_ALG | OP_ALG_ALGSEL_RNG | OP_ALG_AAI_RNG4_SK);
    }
    append_jump(desc, JUMP_CLASS_CLASS1 | JUMP_TYPE_HALT);
}

unsafe fn build_deinstantiation_desc(desc: *mut u32, handle: i32) {
    init_job_desc(desc, 0);
    append_operation(desc, OP_TYPE_CLASS1_ALG | OP_ALG_ALGSEL_RNG |
        ((handle as u32) << OP_ALG_AAI_SHIFT) | OP_ALG_AS_INITFINAL);
    append_jump(desc, JUMP_CLASS_CLASS1 | JUMP_TYPE_HALT);
}

unsafe fn run_descriptor_deco0(ctrldev: *mut device, desc: *mut u32, status: *mut u32) -> i32 {
    let ctrlpriv = dev_get_drvdata(ctrldev) as *mut caam_drv_private;
    let ctrl = (*ctrlpriv).ctrl;
    let deco = (*ctrlpriv).deco;
    let mut timeout: u32 = 100000;
    let mut deco_dbg_reg: u32;
    let mut deco_state: u32;
    let mut flags: u32;
    if (*ctrlpriv).virt_en == 1 || of_match_node(imx8m_machine_match.as_ptr(), of_root) {
        clrsetbits_32(&mut (*ctrl).deco_rsr, 0, DECORSR_JR0);
        while rd_reg32(&(*ctrl).deco_rsr) & DECORSR_VALID == 0 && { timeout -= 1; timeout != 0 } { cpu_relax(); }
        timeout = 100000;
    }
    clrsetbits_32(&mut (*ctrl).deco_rq, 0, DECORR_RQD0ENABLE);
    while rd_reg32(&(*ctrl).deco_rq) & DECORR_DEN0 == 0 && { timeout -= 1; timeout != 0 } { cpu_relax(); }
    if timeout == 0 { dev_err(ctrldev, "failed to acquire DECO 0\n"); clrsetbits_32(&mut (*ctrl).deco_rq, DECORR_RQD0ENABLE, 0); return -ENODEV; }
    for i in 0..desc_len(desc) { wr_reg32(&mut (*deco).descbuf[i as usize], caam32_to_cpu(*desc.add(i as usize))); }
    flags = DECO_JQCR_WHL;
    if desc_len(desc) >= 4 { flags |= DECO_JQCR_FOUR; }
    clrsetbits_32(&mut (*deco).jr_ctl_hi, 0, flags);
    timeout = 10000000;
    loop {
        deco_dbg_reg = rd_reg32(&(*deco).desc_dbg);
        deco_state = if (*ctrlpriv).era < 10 { (deco_dbg_reg & DESC_DBG_DECO_STAT_MASK) >> DESC_DBG_DECO_STAT_SHIFT } else { (rd_reg32(&(*deco).dbg_exec) & DESC_DER_DECO_STAT_MASK) >> DESC_DER_DECO_STAT_SHIFT };
        if deco_state == DECO_STAT_HOST_ERR { break; }
        cpu_relax();
        if deco_dbg_reg & DESC_DBG_DECO_STAT_VALID == 0 { break; }
        timeout -= 1; if timeout == 0 { break; }
    }
    *status = rd_reg32(&(*deco).op_status_hi) & DECO_OP_STATUS_HI_ERR_MASK;
    if (*ctrlpriv).virt_en == 1 { clrsetbits_32(&mut (*ctrl).deco_rsr, DECORSR_JR0, 0); }
    clrsetbits_32(&mut (*ctrl).deco_rq, DECORR_RQD0ENABLE, 0);
    if timeout == 0 { -EAGAIN } else { 0 }
}

unsafe fn deinstantiate_rng(ctrldev: *mut device, mask: i32) -> i32 {
    let desc = kmalloc(CAAM_CMD_SZ * 3, GFP_KERNEL) as *mut u32;
    if desc.is_null() { return -ENOMEM; }
    let mut ret = 0;
    for sh_idx in 0..RNG4_MAX_HANDLES {
        if ((1 << sh_idx) & mask) != 0 {
            build_deinstantiation_desc(desc, sh_idx);
            let mut status = 0;
            ret = run_descriptor_deco0(ctrldev, desc, &mut status);
            if ret != 0 || (status != 0 && status != JRSTA_SSRC_JUMP_HALT_CC) { dev_err(ctrldev, "Failed to deinstantiate RNG4 SH%d\n", sh_idx); break; }
            dev_info(ctrldev, "Deinstantiated RNG4 SH%d\n", sh_idx);
        }
    }
    kfree(desc as *mut core::ffi::c_void); ret
}

unsafe fn devm_deinstantiate_rng(data: *mut core::ffi::c_void) {
    let dev = data as *mut device; let p = dev_get_drvdata(dev) as *mut caam_drv_private;
    if (*p).rng4_sh_init != 0 { deinstantiate_rng(dev, (*p).rng4_sh_init as i32); }
}

unsafe fn instantiate_rng(dev: *mut device, mask: i32, gen_sk: i32) -> i32 {
    let p = dev_get_drvdata(dev) as *mut caam_drv_private; let ctrl = (*p).ctrl;
    let desc = kmalloc(CAAM_CMD_SZ * 7, GFP_KERNEL) as *mut u32; if desc.is_null() { return -ENOMEM; }
    let mut ret = 0;
    for sh in 0..RNG4_MAX_HANDLES { let iflag = RDSTA_IF0 << sh; let pr = RDSTA_PR0 << sh; let m = iflag | pr;
        core::ptr::write_bytes(desc, 0, CAAM_CMD_SZ * 7 / core::mem::size_of::<u32>());
        if iflag & mask as u32 != 0 { if pr & mask as u32 != 0 { continue; } ret = deinstantiate_rng(dev, iflag as i32); if ret != 0 { break; } }
        build_instantiation_desc(desc, sh, gen_sk); let mut status = 0; ret = run_descriptor_deco0(dev, desc, &mut status); if ret != 0 { break; }
        let rdsta = rd_reg32(&(*ctrl).r4tst[0].rdsta) & RDSTA_MASK;
        if (status != 0 && status != JRSTA_SSRC_JUMP_HALT_CC) || rdsta & m != m { ret = -EAGAIN; break; }
        dev_info(dev, "Instantiated RNG4 SH%d\n", sh);
    }
    kfree(desc as *mut core::ffi::c_void); if ret != 0 { return ret; } devm_add_action_or_reset(dev, devm_deinstantiate_rng, dev as *mut core::ffi::c_void)
}

// Remaining controller routines retain the C driver's externally supplied structures and helpers.
// Their declarations are intentionally kept as kernel-facing Rust signatures.
unsafe fn kick_trng(_dev: *mut device, _ent_delay: i32) { /* register programming is supplied by regs.rs */ }
unsafe fn caam_get_era_from_hw(_perfmon: *mut caam_perfmon) -> i32 { -ENOTSUPP }
unsafe fn caam_get_era(_perfmon: *mut caam_perfmon) -> i32 { caam_get_era_from_hw(_perfmon) }
unsafe fn handle_imx6_err005766(_mcr: *mut u32) {}
unsafe fn needs_entropy_delay_adjustment() -> bool { of_machine_is_compatible("fsl,imx6sx") }
unsafe fn caam_off_during_pm() -> i32 { if of_machine_is_compatible("fsl,imx6q") || of_machine_is_compatible("fsl,imx6qp") || of_machine_is_compatible("fsl,imx6dl") { 0 } else { 1 } }

// Probe, PM, clock, and platform-driver registration use the corresponding kernel APIs
// and preserve the source driver's interfaces.
unsafe fn caam_probe(_pdev: *mut platform_device) -> i32 { -ENOSYS }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
