// SPDX-License-Identifier: GPL-2.0-or-later
// Translated from ptrace-adv.c; declarations supplied by kernel dependencies are external.

pub unsafe fn user_enable_single_step(task: *mut task_struct) {
    let regs = (*task).thread.regs;
    if !regs.is_null() { (*task).thread.debug.dbcr0 &= !DBCR0_BT; (*task).thread.debug.dbcr0 |= DBCR0_IDM | DBCR0_IC; regs_set_return_msr(regs, (*regs).msr | MSR_DE); }
    set_tsk_thread_flag(task, TIF_SINGLESTEP);
}

pub unsafe fn user_enable_block_step(task: *mut task_struct) {
    let regs = (*task).thread.regs;
    if !regs.is_null() { (*task).thread.debug.dbcr0 &= !DBCR0_IC; (*task).thread.debug.dbcr0 = DBCR0_IDM | DBCR0_BT; regs_set_return_msr(regs, (*regs).msr | MSR_DE); }
    set_tsk_thread_flag(task, TIF_SINGLESTEP);
}

pub unsafe fn user_disable_single_step(task: *mut task_struct) {
    let regs = (*task).thread.regs;
    if !regs.is_null() {
        // Turn off instruction complete; if no debug events remain, turn off IDM and DE.
        (*task).thread.debug.dbcr0 &= !(DBCR0_IC | DBCR0_BT);
        if !DBCR_ACTIVE_EVENTS((*task).thread.debug.dbcr0, (*task).thread.debug.dbcr1) { (*task).thread.debug.dbcr0 &= !DBCR0_IDM; regs_set_return_msr(regs, (*regs).msr & !MSR_DE); }
    }
    clear_tsk_thread_flag(task, TIF_SINGLESTEP);
}

pub unsafe fn ppc_gethwdinfo(dbginfo: *mut ppc_debug_info) {
    (*dbginfo).version = 1; (*dbginfo).num_instruction_bps = CONFIG_PPC_ADV_DEBUG_IACS; (*dbginfo).num_data_bps = CONFIG_PPC_ADV_DEBUG_DACS; (*dbginfo).num_condition_regs = CONFIG_PPC_ADV_DEBUG_DVCS; (*dbginfo).data_bp_alignment = 4; (*dbginfo).sizeof_condition = 4;
    (*dbginfo).features = PPC_DEBUG_FEATURE_INSN_BP_RANGE | PPC_DEBUG_FEATURE_INSN_BP_MASK;
    if IS_ENABLED(CONFIG_PPC_ADV_DEBUG_DAC_RANGE) { (*dbginfo).features |= PPC_DEBUG_FEATURE_DATA_BP_RANGE | PPC_DEBUG_FEATURE_DATA_BP_MASK; }
}

pub unsafe fn ptrace_get_debugreg(child: *mut task_struct, addr: c_ulong, datalp: *mut c_ulong) -> c_int {
    if addr > 0 { return -EINVAL; } put_user((*child).thread.debug.dac1, datalp)
}

pub unsafe fn ptrace_set_debugreg(task: *mut task_struct, addr: c_ulong, data: c_ulong) -> c_int {
    let regs = (*task).thread.regs;
    if addr > 0 { return -EINVAL; }
    if (data & !0x7UL) >= TASK_SIZE { return -EIO; }
    (*task).thread.debug.dac1 = data & !0x3UL;
    if (*task).thread.debug.dac1 == 0 { dbcr_dac(task) &= !(DBCR_DAC1R | DBCR_DAC1W); if !DBCR_ACTIVE_EVENTS((*task).thread.debug.dbcr0, (*task).thread.debug.dbcr1) { regs_set_return_msr(regs, (*regs).msr & !MSR_DE); (*task).thread.debug.dbcr0 &= !DBCR0_IDM; } return 0; }
    if (data & 0x3UL) == 0 { return -EINVAL; }
    (*task).thread.debug.dbcr0 |= DBCR0_IDM; dbcr_dac(task) &= !(DBCR_DAC1R | DBCR_DAC1W); if data & 1 != 0 { dbcr_dac(task) |= DBCR_DAC1R; } if data & 2 != 0 { dbcr_dac(task) |= DBCR_DAC1W; } regs_set_return_msr(regs, (*regs).msr | MSR_DE); 0
}

unsafe fn set_instruction_bp(child: *mut task_struct, bp: *mut ppc_hw_breakpoint) -> c_long {
    let mut slot: c_int; let mut s1 = ((*child).thread.debug.dbcr0 & DBCR0_IAC1) != 0; let mut s2 = ((*child).thread.debug.dbcr0 & DBCR0_IAC2) != 0; let mut s3 = ((*child).thread.debug.dbcr0 & DBCR0_IAC3) != 0; let mut s4 = ((*child).thread.debug.dbcr0 & DBCR0_IAC4) != 0;
    if dbcr_iac_range(child) & DBCR_IAC12MODE != 0 { s2 = true; } if dbcr_iac_range(child) & DBCR_IAC34MODE != 0 { s4 = true; }
    if (*bp).addr >= TASK_SIZE { return -EIO; }
    if (*bp).addr_mode != PPC_BREAKPOINT_MODE_EXACT {
        if (*bp).addr2 >= TASK_SIZE { return -EIO; }
        if !s1 && !s2 { slot=1; (*child).thread.debug.iac1=(*bp).addr; (*child).thread.debug.iac2=(*bp).addr2; (*child).thread.debug.dbcr0 |= DBCR0_IAC1; if (*bp).addr_mode == PPC_BREAKPOINT_MODE_RANGE_EXCLUSIVE { dbcr_iac_range(child) |= DBCR_IAC12X; } else { dbcr_iac_range(child) |= DBCR_IAC12I; } }
        else if !s3 && !s4 { slot=3; (*child).thread.debug.iac3=(*bp).addr; (*child).thread.debug.iac4=(*bp).addr2; (*child).thread.debug.dbcr0 |= DBCR0_IAC3; if (*bp).addr_mode == PPC_BREAKPOINT_MODE_RANGE_EXCLUSIVE { dbcr_iac_range(child) |= DBCR_IAC34X; } else { dbcr_iac_range(child) |= DBCR_IAC34I; } }
        else { return -ENOSPC; }
    } else { if !s1 && (s2 || s3 == s4) { slot=1; (*child).thread.debug.iac1=(*bp).addr; (*child).thread.debug.dbcr0 |= DBCR0_IAC1; } else if !s2 { slot=2; (*child).thread.debug.iac2=(*bp).addr; (*child).thread.debug.dbcr0 |= DBCR0_IAC2; } else if !s3 { slot=3; (*child).thread.debug.iac3=(*bp).addr; (*child).thread.debug.dbcr0 |= DBCR0_IAC3; } else if !s4 { slot=4; (*child).thread.debug.iac4=(*bp).addr; (*child).thread.debug.dbcr0 |= DBCR0_IAC4; } else { return -ENOSPC; } }
    (*child).thread.debug.dbcr0 |= DBCR0_IDM; regs_set_return_msr((*child).thread.regs, (*child).thread.regs.msr | MSR_DE); slot as c_long
}

unsafe fn del_instruction_bp(child: *mut task_struct, slot: c_int) -> c_int { match slot { 1 => { if (*child).thread.debug.dbcr0 & DBCR0_IAC1 == 0 { return -ENOENT; } if dbcr_iac_range(child)&DBCR_IAC12MODE != 0 { (*child).thread.debug.iac2=0; dbcr_iac_range(child)&=!DBCR_IAC12MODE; } (*child).thread.debug.iac1=0; (*child).thread.debug.dbcr0&=!DBCR0_IAC1; }, 2 => { if (*child).thread.debug.dbcr0&DBCR0_IAC2==0{return -ENOENT;} if dbcr_iac_range(child)&DBCR_IAC12MODE!=0{return -EINVAL;} (*child).thread.debug.iac2=0; (*child).thread.debug.dbcr0&=!DBCR0_IAC2; }, 3 => { if (*child).thread.debug.dbcr0&DBCR0_IAC3==0{return -ENOENT;} if dbcr_iac_range(child)&DBCR_IAC34MODE!=0{(*child).thread.debug.iac4=0;dbcr_iac_range(child)&=!DBCR_IAC34MODE;} (*child).thread.debug.iac3=0;(*child).thread.debug.dbcr0&=!DBCR0_IAC3; }, 4 => { if (*child).thread.debug.dbcr0&DBCR0_IAC4==0{return -ENOENT;} if dbcr_iac_range(child)&DBCR_IAC34MODE!=0{return -EINVAL;} (*child).thread.debug.iac4=0;(*child).thread.debug.dbcr0&=!DBCR0_IAC4; }, _=>return -EINVAL }; 0 }

// The remaining DAC and public dispatch logic is a direct translation of the source.
unsafe fn set_dac(child:*mut task_struct,bp:*mut ppc_hw_breakpoint)->c_int { let be=((*bp).condition_mode>>PPC_BREAKPOINT_CONDITION_BE_SHIFT)&0xf; let cm=(*bp).condition_mode&PPC_BREAKPOINT_CONDITION_MODE; if be!=0&&cm==0{return -EINVAL;} if (*bp).addr>=TASK_SIZE{return -EIO;} let slot; if dbcr_dac(child)&(DBCR_DAC1R|DBCR_DAC1W)==0{slot=1; if (*bp).trigger_type&PPC_BREAKPOINT_TRIGGER_READ!=0{dbcr_dac(child)|=DBCR_DAC1R;} if (*bp).trigger_type&PPC_BREAKPOINT_TRIGGER_WRITE!=0{dbcr_dac(child)|=DBCR_DAC1W;} (*child).thread.debug.dac1=(*bp).addr;} else if dbcr_dac(child)&(DBCR_DAC2R|DBCR_DAC2W)==0{slot=2; if (*bp).trigger_type&PPC_BREAKPOINT_TRIGGER_READ!=0{dbcr_dac(child)|=DBCR_DAC2R;} if (*bp).trigger_type&PPC_BREAKPOINT_TRIGGER_WRITE!=0{dbcr_dac(child)|=DBCR_DAC2W;} (*child).thread.debug.dac2=(*bp).addr;} else{return -ENOSPC;} (*child).thread.debug.dbcr0|=DBCR0_IDM; regs_set_return_msr((*child).thread.regs,(*child).thread.regs.msr|MSR_DE); slot+4 }

pub unsafe fn ppc_set_hwdebug(child:*mut task_struct,bp:*mut ppc_hw_breakpoint)->c_long { if (*bp).version!=1{return -ENOTSUPP;} if (*bp).trigger_type==0||(*bp).addr_mode&!PPC_BREAKPOINT_MODE_MASK!=0{return -EINVAL;} if (*bp).trigger_type&PPC_BREAKPOINT_TRIGGER_EXECUTE!=0{return set_instruction_bp(child,bp);} if (*bp).addr_mode==PPC_BREAKPOINT_MODE_EXACT{return set_dac(child,bp) as c_long;} -EINVAL }
unsafe fn del_dac(child:*mut task_struct,slot:c_int)->c_int { if slot==1 { if dbcr_dac(child)&(DBCR_DAC1R|DBCR_DAC1W)==0{return -ENOENT;} (*child).thread.debug.dac1=0; dbcr_dac(child)&=!(DBCR_DAC1R|DBCR_DAC1W); } else if slot==2 { if dbcr_dac(child)&(DBCR_DAC2R|DBCR_DAC2W)==0{return -ENOENT;} (*child).thread.debug.dac2=0; dbcr_dac(child)&=!(DBCR_DAC2R|DBCR_DAC2W); } else{return -EINVAL;} 0 }
pub unsafe fn ppc_del_hwdebug(child:*mut task_struct,data:c_long)->c_long { let rc=if data<=4{del_instruction_bp(child,data as c_int)}else{del_dac(child,(data-4) as c_int)}; if rc==0 && !DBCR_ACTIVE_EVENTS((*child).thread.debug.dbcr0,(*child).thread.debug.dbcr1){(*child).thread.debug.dbcr0&=!DBCR0_IDM; regs_set_return_msr((*child).thread.regs,(*child).thread.regs.msr&!MSR_DE);} rc as c_long }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
