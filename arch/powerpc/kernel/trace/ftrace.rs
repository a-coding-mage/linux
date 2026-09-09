// SPDX-License-Identifier: GPL-2.0
// Code for replacing ftrace calls with jumps.
// Translated from ftrace.c; external kernel declarations are supplied elsewhere.

const NUM_FTRACE_TRAMPS: usize = 2;
static mut FTRACE_TRAMPS: [usize; NUM_FTRACE_TRAMPS] = [0; NUM_FTRACE_TRAMPS];

pub unsafe fn ftrace_call_adjust(mut addr: usize) -> usize {
    if addr >= __exittext_begin as usize && addr < __exittext_end as usize { return 0; }
    if IS_ENABLED_CONFIG_ARCH_USING_PATCHABLE_FUNCTION_ENTRY {
        if !IS_ENABLED_CONFIG_PPC_FTRACE_OUT_OF_LINE {
            addr += MCOUNT_INSN_SIZE as usize;
            if IS_ENABLED_CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS { addr += MCOUNT_INSN_SIZE as usize; }
        } else if IS_ENABLED_CONFIG_CC_IS_CLANG && IS_ENABLED_CONFIG_PPC64 {
            if is_kernel_text(addr) || is_kernel_inittext(addr) { addr = ppc_function_entry(addr as *const _); }
            else if ppc_inst_val(ppc_inst_read(addr as *const u32)) == PPC_RAW_LD(_R2, _R12, -8)
                && ppc_inst_val(ppc_inst_read((addr + 4) as *const u32)) == PPC_RAW_ADD(_R2, _R2, _R12) { addr += 8; }
        }
    }
    addr
}

unsafe fn ftrace_create_branch_inst(ip: usize, addr: usize, link: i32) -> ppc_inst_t {
    WARN_ON(!is_offset_in_branch_range(addr.wrapping_sub(ip)));
    let mut op = core::mem::zeroed();
    create_branch(&mut op, ip as *mut u32, addr, if link != 0 { BRANCH_SET_LINK } else { 0 }); op
}
unsafe fn ftrace_read_inst(ip: usize, op: *mut ppc_inst_t) -> i32 { if copy_inst_from_kernel_nofault(op, ip as *const _) != 0 { pr_err!("0x%lx: fetching instruction failed\n", ip); -EFAULT } else { 0 } }
unsafe fn ftrace_validate_inst(ip: usize, inst: ppc_inst_t) -> i32 { let mut op = core::mem::zeroed(); let mut ret = ftrace_read_inst(ip, &mut op); if ret == 0 && !ppc_inst_equal(op, inst) { pr_err!("0x%lx: expected (%08lx) != found (%08lx)\n", ip, ppc_inst_as_ulong(inst), ppc_inst_as_ulong(op)); ret = -EINVAL; } ret }
unsafe fn ftrace_modify_code(ip: usize, old: ppc_inst_t, new: ppc_inst_t) -> i32 { let mut ret = ftrace_validate_inst(ip, old); if ret == 0 && !ppc_inst_equal(old, new) { ret = patch_instruction(ip as *mut u32, new); } ret }
unsafe fn is_bl_op(op: ppc_inst_t) -> i32 { ((ppc_inst_val(op) & !PPC_LI_MASK) == PPC_RAW_BL(0)) as i32 }
unsafe fn find_ftrace_tramp(ip: usize) -> usize { for i in 0..NUM_FTRACE_TRAMPS { if FTRACE_TRAMPS[i] != 0 && is_offset_in_branch_range(FTRACE_TRAMPS[i].wrapping_sub(ip)) { return FTRACE_TRAMPS[i]; } } 0 }

unsafe fn ftrace_lookup_module_stub(_ip: usize, _addr: usize) -> usize {
    // CONFIG_MODULES variant uses __module_text_address and module arch trampolines.
    0
}
unsafe fn ftrace_get_ool_stub(rec: *mut dyn_ftrace) -> usize {
    if IS_ENABLED_CONFIG_PPC_FTRACE_OUT_OF_LINE { (*rec).arch.ool_stub } else { BUILD_BUG!(); 0 }
}
unsafe fn ftrace_get_call_inst(rec: *mut dyn_ftrace, mut addr: usize, call_inst: *mut ppc_inst_t) -> i32 {
    let ip = if IS_ENABLED_CONFIG_PPC_FTRACE_OUT_OF_LINE { ftrace_get_ool_stub(rec) + MCOUNT_INSN_SIZE as usize } else { (*rec).ip };
    if !is_offset_in_branch_range(addr.wrapping_sub(ip)) && addr != FTRACE_ADDR && addr != FTRACE_REGS_ADDR { if !IS_ENABLED_CONFIG_DYNAMIC_FTRACE_WITH_DIRECT_CALLS { pr_err!("0x%lx (0x%lx): Unexpected target address 0x%lx\n", ip, (*rec).ip, addr); return -EINVAL; } addr = FTRACE_ADDR; }
    let stub = if is_offset_in_branch_range(addr.wrapping_sub(ip)) { addr } else if core_kernel_text(ip) { find_ftrace_tramp(ip) } else { ftrace_lookup_module_stub(ip, addr) };
    if stub == 0 { pr_err!("0x%lx (0x%lx): No ftrace stubs reachable\n", ip, (*rec).ip); return -EINVAL; }
    *call_inst = ftrace_create_branch_inst(ip, stub, 1); 0
}

unsafe fn ftrace_rec_set_nop_ops(_rec: *mut dyn_ftrace) -> i32 { 0 }
unsafe fn ftrace_rec_update_ops(_rec: *mut dyn_ftrace) -> i32 { 0 }

pub unsafe fn ftrace_make_call(rec: *mut dyn_ftrace, addr: usize) -> i32 { let mut old = ppc_inst(PPC_RAW_NOP()); let mut new = core::mem::zeroed(); let mut ip = (*rec).ip; let mut ret = 0; if WARN_ON(!IS_ENABLED_CONFIG_MODULES || core_kernel_text(ip)) { return -EINVAL; } if IS_ENABLED_CONFIG_PPC_FTRACE_OUT_OF_LINE { ip = ftrace_get_ool_stub(rec) + MCOUNT_INSN_SIZE as usize; ret = ftrace_get_call_inst(rec, ftrace_caller as usize, &mut old); } ret |= ftrace_get_call_inst(rec, addr, &mut new); if ret == 0 { ret = ftrace_modify_code(ip, old, new); } ret = ftrace_rec_update_ops(rec); if ret == 0 && IS_ENABLED_CONFIG_PPC_FTRACE_OUT_OF_LINE { ret = ftrace_modify_code((*rec).ip, ppc_inst(PPC_RAW_NOP()), ppc_inst(PPC_RAW_BRANCH((ftrace_get_ool_stub(rec) as isize - (*rec).ip as isize) as i64))); } ret }
pub unsafe fn ftrace_make_nop(_mod: *mut module, _rec: *mut dyn_ftrace, _addr: usize) -> i32 { WARN_ON(1); -EINVAL }

pub unsafe fn ftrace_replace_code(enable: i32) {
    let mut ret = 0; let mut rec: *mut dyn_ftrace = core::ptr::null_mut();
    for_ftrace_rec_iter!(iter) { rec = ftrace_rec_iter_record(iter); if (*rec).flags & FTRACE_FL_DISABLED != 0 && (*rec).flags & FTRACE_FL_ENABLED == 0 { continue; } let update = ftrace_update_record(rec, enable); if update == FTRACE_UPDATE_IGNORE { continue; } let ip = (*rec).ip; let old = ppc_inst(PPC_RAW_NOP()); let mut new = old; if update == FTRACE_UPDATE_MAKE_CALL { ret = ftrace_get_call_inst(rec, ftrace_get_addr_new(rec), &mut new); } else if update == FTRACE_UPDATE_MODIFY_CALL { ret = ftrace_get_call_inst(rec, ftrace_get_addr_new(rec), &mut new); } if ret == 0 { ret = ftrace_modify_code(ip, old, new); } if ret != 0 { break; } }
    if ret != 0 { ftrace_bug(ret, rec); }
}

pub unsafe fn ftrace_init_nop(_mod: *mut module, rec: *mut dyn_ftrace) -> i32 { let ip = (*rec).ip; let mut ret = 0; if IS_ENABLED_CONFIG_ARCH_USING_PATCHABLE_FUNCTION_ENTRY { if !IS_ENABLED_CONFIG_PPC_FTRACE_OUT_OF_LINE { ret = ftrace_validate_inst(ip - 4, ppc_inst(PPC_RAW_NOP())); } if ret == 0 { ret = ftrace_validate_inst(ip, ppc_inst(PPC_RAW_NOP())); } } else { ret = -EINVAL; } if ret == 0 && !IS_ENABLED_CONFIG_PPC_FTRACE_OUT_OF_LINE { ret = ftrace_modify_code(ip, ppc_inst(PPC_RAW_NOP()), ppc_inst(PPC_RAW_NOP())); } if ret == 0 { ret = ftrace_rec_set_nop_ops(rec); } ret }
pub unsafe fn ftrace_update_ftrace_func(func: ftrace_func_t) -> i32 { if IS_ENABLED_CONFIG_DYNAMIC_FTRACE_WITH_CALL_OPS { return 0; } let ip = &ftrace_call as *const _ as usize; ftrace_modify_code(ip, ppc_inst_read(ip as *const u32), ftrace_create_branch_inst(ip, ppc_function_entry(func), 1)) }
pub unsafe fn arch_ftrace_update_code(command: i32) { ftrace_modify_all_code(command); }
pub unsafe fn ftrace_free_init_tramp() { for i in 0..NUM_FTRACE_TRAMPS { if FTRACE_TRAMPS[i] == ftrace_tramp_init as usize { FTRACE_TRAMPS[i] = 0; return; } } }
unsafe fn add_ftrace_tramp(tramp: usize) { for i in 0..NUM_FTRACE_TRAMPS { if FTRACE_TRAMPS[i] == 0 { FTRACE_TRAMPS[i] = tramp; return; } } }
pub unsafe fn ftrace_dyn_arch_init() -> i32 { let tramp = [ftrace_tramp_text as *mut u32, ftrace_tramp_init as *mut u32]; let addr = FTRACE_REGS_ADDR; for i in 0..2 { *tramp[i] = PPC_RAW_BCTR(); add_ftrace_tramp(tramp[i] as usize); } let _ = addr; 0 }

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
pub unsafe fn ftrace_graph_func(ip: usize, mut parent_ip: usize, op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    let sp = arch_ftrace_regs(fregs).regs.gpr[1];
    if unlikely(ftrace_graph_is_dead()) || unlikely(atomic_read(&mut (*current).tracing_graph_pause) != 0) { (*arch_ftrace_regs(fregs)).regs.link = parent_ip; return; }
    if function_graph_enter_regs(parent_ip, ip, 0, sp as *mut usize, fregs) == 0 { parent_ip = ppc_function_entry(return_to_handler); }
    (*arch_ftrace_regs(fregs)).regs.link = parent_ip;
    let _ = op;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
