// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022-2023 Loongson Technology Corporation Limited
 */

/* Dependencies supplied by the surrounding kernel translation. */

static mut BP_ON_REG: [*mut perf_event; LOONGARCH_MAX_BRP] = [core::ptr::null_mut(); LOONGARCH_MAX_BRP];
static mut WP_ON_REG: [*mut perf_event; LOONGARCH_MAX_WRP] = [core::ptr::null_mut(); LOONGARCH_MAX_WRP];

pub unsafe fn hw_breakpoint_slots(type_: i32) -> i32 {
    match type_ {
        TYPE_INST => get_num_brps(),
        TYPE_DATA => get_num_wrps(),
        _ => { pr_warn!("unknown slot type: {}\n", type_); 0 }
    }
}

unsafe fn read_wb_reg(reg: i32, n: i32, t: i32) -> u64 {
    let mut val: u64 = 0;
    match reg + n {
        x if x >= CSR_CFG_ADDR && x < CSR_CFG_ADDR + 14 => { loongarch_csr_watch_read(n, ADDR, t, &mut val); }
        x if x >= CSR_CFG_MASK && x < CSR_CFG_MASK + 14 => { loongarch_csr_watch_read(n, MASK, t, &mut val); }
        x if x >= CSR_CFG_CTRL && x < CSR_CFG_CTRL + 14 => { loongarch_csr_watch_read(n, CTRL, t, &mut val); }
        x if x >= CSR_CFG_ASID && x < CSR_CFG_ASID + 14 => { loongarch_csr_watch_read(n, ASID, t, &mut val); }
        _ => { pr_warn!("Attempt to read from unknown breakpoint register {}\n", n); }
    }
    val
}

unsafe fn write_wb_reg(reg: i32, n: i32, t: i32, val: u64) {
    match reg + n {
        x if x >= CSR_CFG_ADDR && x < CSR_CFG_ADDR + 14 => loongarch_csr_watch_write(n, ADDR, t, val),
        x if x >= CSR_CFG_MASK && x < CSR_CFG_MASK + 14 => loongarch_csr_watch_write(n, MASK, t, val),
        x if x >= CSR_CFG_CTRL && x < CSR_CFG_CTRL + 14 => loongarch_csr_watch_write(n, CTRL, t, val),
        x if x >= CSR_CFG_ASID && x < CSR_CFG_ASID + 14 => loongarch_csr_watch_write(n, ASID, t, val),
        _ => pr_warn!("Attempt to write to unknown breakpoint register {}\n", n),
    }
}

#[repr(C)]
pub enum hw_breakpoint_ops { HW_BREAKPOINT_INSTALL, HW_BREAKPOINT_UNINSTALL }

unsafe fn hw_breakpoint_slot_setup(slots: *mut *mut perf_event, max_slots: i32, bp: *mut perf_event, ops: hw_breakpoint_ops) -> i32 {
    for i in 0..max_slots {
        let slot = slots.add(i as usize);
        match ops {
            hw_breakpoint_ops::HW_BREAKPOINT_INSTALL => { if (*slot).is_null() { *slot = bp; return i; } }
            hw_breakpoint_ops::HW_BREAKPOINT_UNINSTALL => { if *slot == bp { *slot = core::ptr::null_mut(); return i; } }
        }
    }
    -ENOSPC
}

pub unsafe fn ptrace_hw_copy_thread(tsk: *mut task_struct) {
    core::ptr::write_bytes((*tsk).thread.hbp_break.as_mut_ptr(), 0, (*tsk).thread.hbp_break.len());
    core::ptr::write_bytes((*tsk).thread.hbp_watch.as_mut_ptr(), 0, (*tsk).thread.hbp_watch.len());
}

pub unsafe fn flush_ptrace_hw_breakpoint(tsk: *mut task_struct) {
    let t = &mut (*tsk).thread;
    for i in 0..LOONGARCH_MAX_BRP { if !t.hbp_break[i].is_null() { unregister_hw_breakpoint(t.hbp_break[i]); t.hbp_break[i] = core::ptr::null_mut(); } }
    for i in 0..LOONGARCH_MAX_WRP { if !t.hbp_watch[i].is_null() { unregister_hw_breakpoint(t.hbp_watch[i]); t.hbp_watch[i] = core::ptr::null_mut(); } }
}

unsafe fn hw_breakpoint_control(bp: *mut perf_event, ops: hw_breakpoint_ops) -> i32 {
    let info = counter_arch_bp(bp);
    let privilege = if arch_check_bp_in_kernelspace(info) { CTRL_PLV0_ENABLE } else { CTRL_PLV3_ENABLE };
    let (slots, max_slots) = if (*info).ctrl.type_ == LOONGARCH_BREAKPOINT_EXECUTE { (BP_ON_REG.as_mut_ptr(), boot_cpu_data.watch_ireg_count) } else { (WP_ON_REG.as_mut_ptr(), boot_cpu_data.watch_dreg_count) };
    let i = hw_breakpoint_slot_setup(slots, max_slots, bp, ops);
    if i < 0 { return i; }
    match ops {
        hw_breakpoint_ops::HW_BREAKPOINT_INSTALL => {
            let t = if (*info).ctrl.type_ == LOONGARCH_BREAKPOINT_EXECUTE { 0 } else { 1 };
            write_wb_reg(CSR_CFG_ADDR, i, t, (*info).address);
            write_wb_reg(CSR_CFG_MASK, i, t, (*info).mask);
            write_wb_reg(CSR_CFG_ASID, i, t, 0);
            let ctrl = if t == 0 { privilege } else { encode_ctrl_reg((*info).ctrl) | privilege };
            write_wb_reg(CSR_CFG_CTRL, i, t, ctrl);
            let enable = csr_read64(LOONGARCH_CSR_CRMD);
            csr_write64(CSR_CRMD_WE | enable, LOONGARCH_CSR_CRMD);
            if !(*bp).hw.target.is_null() && test_tsk_thread_flag((*bp).hw.target, TIF_LOAD_WATCH) { task_pt_regs((*bp).hw.target).csr_prmd |= CSR_PRMD_PWE; }
        }
        hw_breakpoint_ops::HW_BREAKPOINT_UNINSTALL => {
            let t = if (*info).ctrl.type_ == LOONGARCH_BREAKPOINT_EXECUTE { 0 } else { 1 };
            write_wb_reg(CSR_CFG_ADDR, i, t, 0); write_wb_reg(CSR_CFG_MASK, i, t, 0);
            write_wb_reg(CSR_CFG_CTRL, i, t, 0); write_wb_reg(CSR_CFG_ASID, i, t, 0);
            if !(*bp).hw.target.is_null() { task_pt_regs((*bp).hw.target).csr_prmd &= !CSR_PRMD_PWE; }
        }
    }
    0
}

pub unsafe fn arch_install_hw_breakpoint(bp: *mut perf_event) -> i32 { hw_breakpoint_control(bp, hw_breakpoint_ops::HW_BREAKPOINT_INSTALL) }
pub unsafe fn arch_uninstall_hw_breakpoint(bp: *mut perf_event) { hw_breakpoint_control(bp, hw_breakpoint_ops::HW_BREAKPOINT_UNINSTALL); }

unsafe fn get_hbp_len(hbp_len: u8) -> u32 { match hbp_len { LOONGARCH_BREAKPOINT_LEN_1 => 1, LOONGARCH_BREAKPOINT_LEN_2 => 2, LOONGARCH_BREAKPOINT_LEN_4 => 4, LOONGARCH_BREAKPOINT_LEN_8 => 8, _ => 0 } }

pub unsafe fn arch_check_bp_in_kernelspace(hw: *mut arch_hw_breakpoint) -> bool {
    let va = (*hw).address; let len = get_hbp_len((*hw).ctrl.len);
    (va >= TASK_SIZE) && ((va + len as usize - 1) >= TASK_SIZE)
}

pub unsafe fn arch_bp_generic_fields(ctrl: arch_hw_breakpoint_ctrl, gen_len: *mut i32, gen_type: *mut i32) -> i32 {
    *gen_type = match ctrl.type_ { LOONGARCH_BREAKPOINT_EXECUTE => HW_BREAKPOINT_X, LOONGARCH_BREAKPOINT_LOAD => HW_BREAKPOINT_R, LOONGARCH_BREAKPOINT_STORE => HW_BREAKPOINT_W, x if x == (LOONGARCH_BREAKPOINT_LOAD | LOONGARCH_BREAKPOINT_STORE) => HW_BREAKPOINT_RW, _ => return -EINVAL };
    *gen_len = match ctrl.len { LOONGARCH_BREAKPOINT_LEN_1 => HW_BREAKPOINT_LEN_1, LOONGARCH_BREAKPOINT_LEN_2 => HW_BREAKPOINT_LEN_2, LOONGARCH_BREAKPOINT_LEN_4 => HW_BREAKPOINT_LEN_4, LOONGARCH_BREAKPOINT_LEN_8 => HW_BREAKPOINT_LEN_8, _ => return -EINVAL };
    0
}

unsafe fn arch_build_bp_info(_bp: *mut perf_event, attr: *const perf_event_attr, hw: *mut arch_hw_breakpoint) -> i32 {
    (*hw).ctrl.type_ = match (*attr).bp_type { HW_BREAKPOINT_X => LOONGARCH_BREAKPOINT_EXECUTE, HW_BREAKPOINT_R => LOONGARCH_BREAKPOINT_LOAD, HW_BREAKPOINT_W => LOONGARCH_BREAKPOINT_STORE, HW_BREAKPOINT_RW => LOONGARCH_BREAKPOINT_LOAD | LOONGARCH_BREAKPOINT_STORE, _ => return -EINVAL };
    (*hw).ctrl.len = match (*attr).bp_len { HW_BREAKPOINT_LEN_1 => LOONGARCH_BREAKPOINT_LEN_1, HW_BREAKPOINT_LEN_2 => LOONGARCH_BREAKPOINT_LEN_2, HW_BREAKPOINT_LEN_4 => LOONGARCH_BREAKPOINT_LEN_4, HW_BREAKPOINT_LEN_8 => LOONGARCH_BREAKPOINT_LEN_8, _ => return -EINVAL };
    (*hw).address = (*attr).bp_addr; 0
}

pub unsafe fn hw_breakpoint_arch_parse(bp: *mut perf_event, attr: *const perf_event_attr, hw: *mut arch_hw_breakpoint) -> i32 {
    let ret = arch_build_bp_info(bp, attr, hw); if ret != 0 { return ret; }
    if (*hw).ctrl.type_ == LOONGARCH_BREAKPOINT_EXECUTE { (*hw).address &= !0x3; } 0
}

unsafe fn update_bp_registers(regs: *mut pt_regs, enable: i32, type_: i32) {
    let (slots, max_slots) = match type_ { 0 => (BP_ON_REG.as_mut_ptr(), boot_cpu_data.watch_ireg_count), 1 => (WP_ON_REG.as_mut_ptr(), boot_cpu_data.watch_dreg_count), _ => return };
    for i in 0..max_slots { let bp = *slots.add(i as usize); if bp.is_null() { continue; } let info = counter_arch_bp(bp);
        if enable != 0 { if (*info).ctrl.type_ == LOONGARCH_BREAKPOINT_EXECUTE && type_ == 0 { write_wb_reg(CSR_CFG_CTRL, i, 0, CTRL_PLV_ENABLE); write_wb_reg(CSR_CFG_CTRL, i, 0, CTRL_PLV_ENABLE); } else { let mut ctrl = read_wb_reg(CSR_CFG_CTRL, i, 1); if (*info).ctrl.type_ == LOONGARCH_BREAKPOINT_LOAD { ctrl |= 0x1 << MWPnCFG3_LoadEn; } if (*info).ctrl.type_ == LOONGARCH_BREAKPOINT_STORE { ctrl |= 0x1 << MWPnCFG3_StoreEn; } write_wb_reg(CSR_CFG_CTRL, i, 1, ctrl); } (*regs).csr_prmd |= CSR_PRMD_PWE;
        } else { if (*info).ctrl.type_ == LOONGARCH_BREAKPOINT_EXECUTE && type_ == 0 { write_wb_reg(CSR_CFG_CTRL, i, 0, 0); } else { let mut ctrl = read_wb_reg(CSR_CFG_CTRL, i, 1); if (*info).ctrl.type_ == LOONGARCH_BREAKPOINT_LOAD { ctrl &= !(0x1 << MWPnCFG3_LoadEn); } if (*info).ctrl.type_ == LOONGARCH_BREAKPOINT_STORE { ctrl &= !(0x1 << MWPnCFG3_StoreEn); } write_wb_reg(CSR_CFG_CTRL, i, 1, ctrl); } (*regs).csr_prmd &= !CSR_PRMD_PWE; }
    }
}

pub unsafe fn breakpoint_handler(regs: *mut pt_regs) { for i in 0..boot_cpu_data.watch_ireg_count { if csr_read32(LOONGARCH_CSR_FWPS) & (0x1 << i) != 0 { let bp = *BP_ON_REG.as_ptr().add(i as usize); if bp.is_null() { continue; } perf_bp_event(bp, regs); csr_write32(0x1 << i, LOONGARCH_CSR_FWPS); update_bp_registers(regs, 0, 0); } } }
pub unsafe fn watchpoint_handler(regs: *mut pt_regs) { for i in 0..boot_cpu_data.watch_dreg_count { if csr_read32(LOONGARCH_CSR_MWPS) & (0x1 << i) != 0 { let wp = *WP_ON_REG.as_ptr().add(i as usize); if wp.is_null() { continue; } perf_bp_event(wp, regs); csr_write32(0x1 << i, LOONGARCH_CSR_MWPS); update_bp_registers(regs, 0, 1); } } }

pub unsafe fn arch_hw_breakpoint_init() -> i32 { boot_cpu_data.watch_ireg_count = get_num_brps(); boot_cpu_data.watch_dreg_count = get_num_wrps(); pr_info!("Found {} breakpoint and {} watchpoint registers.\n", boot_cpu_data.watch_ireg_count, boot_cpu_data.watch_dreg_count); for cpu in 1..NR_CPUS { cpu_data[cpu].watch_ireg_count = boot_cpu_data.watch_ireg_count; cpu_data[cpu].watch_dreg_count = boot_cpu_data.watch_dreg_count; } 0 }

pub unsafe fn hw_breakpoint_thread_switch(next: *mut task_struct) { let regs = task_pt_regs(next); if test_tsk_thread_flag(next, TIF_SINGLESTEP) { let addr = read_wb_reg(CSR_CFG_ADDR, 0, 0); let mask = read_wb_reg(CSR_CFG_MASK, 0, 0); if ((*regs).csr_era ^ addr) & !mask == 0 { csr_write32(CSR_FWPC_SKIP, LOONGARCH_CSR_FWPS); } (*regs).csr_prmd |= CSR_PRMD_PWE; } else { update_bp_registers(regs, 1, 0); update_bp_registers(regs, 1, 1); } }
pub unsafe fn hw_breakpoint_pmu_read(_bp: *mut perf_event) {}
pub unsafe fn hw_breakpoint_exceptions_notify(_unused: *mut notifier_block, _val: usize, _data: *mut core::ffi::c_void) -> i32 { NOTIFY_DONE }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
