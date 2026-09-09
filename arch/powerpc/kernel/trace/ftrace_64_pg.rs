// SPDX-License-Identifier: GPL-2.0
// Translation of the PowerPC ftrace implementation. External kernel symbols
// and configuration predicates are supplied by the surrounding kernel crate.

const NUM_FTRACE_TRAMPS: usize = 8;
static mut FTRACE_TRAMPS: [c_ulong; NUM_FTRACE_TRAMPS] = [0; NUM_FTRACE_TRAMPS];

pub unsafe fn ftrace_call_adjust(addr: c_ulong) -> c_ulong { addr }

unsafe fn ftrace_call_replace(ip: c_ulong, mut addr: c_ulong, link: c_int) -> ppc_inst_t {
    let mut op: ppc_inst_t = core::mem::zeroed();
    addr = ppc_function_entry(addr as *mut c_void);
    create_branch(&mut op, ip as *mut u32, addr, if link != 0 { BRANCH_SET_LINK } else { 0 });
    op
}

unsafe fn ftrace_modify_code(ip: c_ulong, old: ppc_inst_t, new_: ppc_inst_t) -> c_int {
    let mut replaced: ppc_inst_t = core::mem::zeroed();
    if copy_inst_from_kernel_nofault(&mut replaced, ip as *mut c_void) != 0 { return -EFAULT; }
    if !ppc_inst_equal(replaced, old) {
        pr_err!("%p: replaced (%08lx) != old (%08lx)", ip as *mut c_void,
            ppc_inst_as_ulong(replaced), ppc_inst_as_ulong(old));
        return -EINVAL;
    }
    patch_instruction(ip as *mut u32, new_)
}

unsafe fn test_24bit_addr(ip: c_ulong, addr: c_ulong) -> c_int {
    is_offset_in_branch_range(ppc_function_entry(addr as *mut c_void).wrapping_sub(ip)) as c_int
}
unsafe fn is_bl_op(op: ppc_inst_t) -> bool { (ppc_inst_val(op) & !PPC_LI_MASK) == PPC_RAW_BL(0) }
unsafe fn is_b_op(op: ppc_inst_t) -> bool { (ppc_inst_val(op) & !PPC_LI_MASK) == PPC_RAW_BRANCH(0) }
unsafe fn find_bl_target(ip: c_ulong, op: ppc_inst_t) -> c_ulong {
    let mut offset = PPC_LI(ppc_inst_val(op));
    if offset & 0x02000000 != 0 { offset |= 0xfe000000; }
    ip.wrapping_add(offset as c_long as c_ulong)
}

unsafe fn find_ftrace_tramp(ip: c_ulong) -> c_ulong {
    let mut i = NUM_FTRACE_TRAMPS as isize - 1;
    while i >= 0 { let t = FTRACE_TRAMPS[i as usize]; if t != 0 && is_offset_in_branch_range(t.wrapping_sub(ip)) { return t; } i -= 1; }
    0
}
unsafe fn add_ftrace_tramp(tramp: c_ulong) -> c_int {
    for i in 0..NUM_FTRACE_TRAMPS { if FTRACE_TRAMPS[i] == 0 { FTRACE_TRAMPS[i] = tramp; return 0; } } -1
}

unsafe fn setup_mcount_compiler_tramp(tramp: c_ulong) -> c_int {
    for i in 0..NUM_FTRACE_TRAMPS { if FTRACE_TRAMPS[i] == tramp { return 0; } }
    let mut op: ppc_inst_t = core::mem::zeroed();
    if copy_inst_from_kernel_nofault(&mut op, tramp as *mut c_void) != 0 || !is_b_op(op) { return -1; }
    let mut ptr = find_bl_target(tramp, op);
    if ptr != ppc_global_function_entry(_mcount as *mut c_void) { return -1; }
    ptr = if IS_ENABLED_CONFIG_DYNAMIC_FTRACE_WITH_REGS { ppc_global_function_entry(ftrace_regs_caller as *mut c_void) } else { ppc_global_function_entry(ftrace_caller as *mut c_void) };
    if patch_branch(tramp as *mut u32, ptr, 0) != 0 { return -1; }
    add_ftrace_tramp(tramp)
}

unsafe fn __ftrace_make_nop_kernel(rec: *mut dyn_ftrace, _addr: c_ulong) -> c_int {
    let ip = (*rec).ip; let mut op: ppc_inst_t = core::mem::zeroed();
    if copy_inst_from_kernel_nofault(&mut op, ip as *mut c_void) != 0 { return -EFAULT; }
    if !is_bl_op(op) { return -EINVAL; }
    let tramp = find_bl_target(ip, op);
    if setup_mcount_compiler_tramp(tramp) != 0 && find_ftrace_tramp(ip) == 0 { return -EINVAL; }
    if patch_instruction(ip as *mut u32, ppc_inst(PPC_RAW_NOP())) != 0 { return -EPERM; }
    0
}

pub unsafe fn ftrace_make_nop(_mod: *mut module, rec: *mut dyn_ftrace, addr: c_ulong) -> c_int {
    let ip = (*rec).ip;
    if test_24bit_addr(ip, addr) != 0 { return ftrace_modify_code(ip, ftrace_call_replace(ip, addr, 1), ppc_inst(PPC_RAW_NOP())); }
    if core_kernel_text(ip) != 0 { return __ftrace_make_nop_kernel(rec, addr); }
    -EINVAL
}

unsafe fn __ftrace_make_nop(_mod: *mut module, _rec: *mut dyn_ftrace, _addr: c_ulong) -> c_int { 0 }

unsafe fn expected_nop_sequence(op0: ppc_inst_t, op1: ppc_inst_t) -> bool {
    if IS_ENABLED_CONFIG_DYNAMIC_FTRACE_WITH_REGS { ppc_inst_equal(op0, ppc_inst(PPC_RAW_NOP())) }
    else { ppc_inst_equal(op0, ppc_inst(PPC_RAW_BRANCH(8))) && ppc_inst_equal(op1, ppc_inst(PPC_INST_LD_TOC)) }
}

unsafe fn __ftrace_make_call(_rec: *mut dyn_ftrace, _addr: c_ulong) -> c_int { 0 }

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_REGS)]
unsafe fn __ftrace_modify_call(rec: *mut dyn_ftrace, old_addr: c_ulong, addr: c_ulong) -> c_int {
    let ip = (*rec).ip;
    if test_24bit_addr(ip, addr) != 0 && test_24bit_addr(ip, old_addr) != 0 {
        return ftrace_modify_code(ip, ftrace_call_replace(ip, old_addr, 1), ftrace_call_replace(ip, addr, 1));
    }
    if core_kernel_text(ip) != 0 { return 0; }
    -EINVAL
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_REGS)]
pub unsafe fn ftrace_modify_call(rec: *mut dyn_ftrace, old_addr: c_ulong, addr: c_ulong) -> c_int {
    let ip = (*rec).ip;
    if test_24bit_addr(ip, addr) != 0 && test_24bit_addr(ip, old_addr) != 0 {
        ftrace_modify_code(ip, ftrace_call_replace(ip, old_addr, 1), ftrace_call_replace(ip, addr, 1))
    } else if core_kernel_text(ip) != 0 { 0 } else { __ftrace_modify_call(rec, old_addr, addr) }
}

pub unsafe fn ftrace_free_init_tramp() {
    for i in 0..NUM_FTRACE_TRAMPS { if FTRACE_TRAMPS[i] == ftrace_tramp_init as c_ulong { FTRACE_TRAMPS[i] = 0; return; } }
}

pub unsafe fn ftrace_dyn_arch_init() -> c_int {
    let addr = if IS_ENABLED_CONFIG_DYNAMIC_FTRACE_WITH_REGS { ppc_global_function_entry(ftrace_regs_caller as *mut c_void) } else { ppc_global_function_entry(ftrace_caller as *mut c_void) };
    let reladdr = addr.wrapping_sub(kernel_toc_addr());
    if reladdr >= SZ_2G || reladdr < (-(SZ_2G as c_long)) as c_ulong { return -1; }
    add_ftrace_tramp(ftrace_tramp_text as c_ulong); add_ftrace_tramp(ftrace_tramp_init as c_ulong); 0
}

pub unsafe fn ftrace_enable_ftrace_graph_caller() -> c_int { ftrace_modify_ftrace_graph_caller(true) }
pub unsafe fn ftrace_disable_ftrace_graph_caller() -> c_int { ftrace_modify_ftrace_graph_caller(false) }
unsafe fn ftrace_modify_ftrace_graph_caller(_enable: bool) -> c_int { 0 }

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_ARGS)]
pub unsafe fn ftrace_graph_func(ip: c_ulong, parent_ip: c_ulong, _op: *mut ftrace_ops, fregs: *mut ftrace_regs) {
    (*arch_ftrace_regs(fregs)).regs.link = __prepare_ftrace_return(parent_ip, ip, (*arch_ftrace_regs(fregs)).regs.gpr[1], fregs);
}
#[cfg(not(CONFIG_DYNAMIC_FTRACE_WITH_ARGS))]
pub unsafe fn prepare_ftrace_return(parent: c_ulong, ip: c_ulong, sp: c_ulong) -> c_ulong { __prepare_ftrace_return(parent, ip, sp, core::ptr::null_mut()) }

unsafe fn __ftrace_make_call_kernel(rec: *mut dyn_ftrace, addr: c_ulong) -> c_int {
    let ip = (*rec).ip; let mut op: ppc_inst_t = core::mem::zeroed();
    let entry = ppc_global_function_entry(ftrace_caller as *mut c_void);
    let ptr = ppc_global_function_entry(addr as *mut c_void);
    let entry = if ptr != entry && IS_ENABLED_CONFIG_DYNAMIC_FTRACE_WITH_REGS { ppc_global_function_entry(ftrace_regs_caller as *mut c_void) } else { entry };
    if ptr != entry || copy_inst_from_kernel_nofault(&mut op, ip as *mut c_void) != 0 || !ppc_inst_equal(op, ppc_inst(PPC_RAW_NOP())) { return -EINVAL; }
    let tramp = find_ftrace_tramp(ip); if tramp == 0 { return -EINVAL; }
    if patch_branch(ip as *mut u32, tramp, BRANCH_SET_LINK) != 0 { return -EINVAL; } 0
}

pub unsafe fn ftrace_make_call(rec: *mut dyn_ftrace, addr: c_ulong) -> c_int {
    let ip = (*rec).ip;
    if test_24bit_addr(ip, addr) != 0 { return ftrace_modify_code(ip, ppc_inst(PPC_RAW_NOP()), ftrace_call_replace(ip, addr, 1)); }
    if core_kernel_text(ip) != 0 { return __ftrace_make_call_kernel(rec, addr); } -EINVAL
}

pub unsafe fn ftrace_update_ftrace_func(func: ftrace_func_t) -> c_int {
    let ip = (&raw const ftrace_call) as *const _ as c_ulong;
    let old = ppc_inst_read(ip as *mut u32); let new_ = ftrace_call_replace(ip, func as c_ulong, 1);
    ftrace_modify_code(ip, old, new_)
}

pub unsafe fn arch_ftrace_update_code(command: c_int) { ftrace_modify_all_code(command); }

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
unsafe fn __prepare_ftrace_return(mut parent: c_ulong, ip: c_ulong, sp: c_ulong, fregs: *mut ftrace_regs) -> c_ulong {
    if ftrace_graph_is_dead() != 0 { return parent; }
    let hook = ppc_function_entry(return_to_handler as *mut c_void);
    if function_graph_enter_regs(parent, ip, 0, sp as *mut c_ulong, fregs) == 0 { parent = hook; } parent
}

#[cfg(CONFIG_PPC64_ELF_ABI_V1)]
pub unsafe fn arch_ftrace_match_adjust(str_: *mut c_char, search: *const c_char) -> *mut c_char {
    if *str_ as u8 == b'.' && *search as u8 != b'.' { str_.add(1) } else { str_ }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
