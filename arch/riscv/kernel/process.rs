// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright (C) 2009 Sunplus Core Technology Co., Ltd.
 * Copyright (C) 2012 Regents of the University of California
 * Copyright (C) 2017 SiFive */

// Kernel and architecture headers from the C source provide the external
// types, constants, functions, and macros referenced below.

#[cfg(all(feature = "CONFIG_STACKPROTECTOR", not(feature = "CONFIG_STACKPROTECTOR_PER_TASK")))]
#[no_mangle]
pub static mut __stack_chk_guard: c_ulong = 0;

extern "C" {
    fn ret_from_fork_kernel_asm();
    fn ret_from_fork_user_asm();
}

pub unsafe fn arch_cpu_idle() { cpu_do_idle(); }

pub unsafe fn set_unalign_ctl(tsk: *mut task_struct, val: c_uint) -> c_int {
    if !unaligned_ctl_available() { return -EINVAL; }
    (*tsk).thread.align_ctl = val;
    0
}

pub unsafe fn get_unalign_ctl(tsk: *mut task_struct, adr: c_ulong) -> c_int {
    if !unaligned_ctl_available() { return -EINVAL; }
    put_user((*tsk).thread.align_ctl, adr as *mut c_uint)
}

pub unsafe fn __show_regs(regs: *mut pt_regs) {
    show_regs_print_info(KERN_DEFAULT);
    if !user_mode(regs) {
        pr_cont!("epc : %pS\n", (*regs).epc as *mut c_void);
        pr_cont!(" ra : %pS\n", (*regs).ra as *mut c_void);
    }
    pr_cont!("epc : " REG_FMT " ra : " REG_FMT " sp : " REG_FMT "\n", (*regs).epc, (*regs).ra, (*regs).sp);
    pr_cont!(" gp : " REG_FMT " tp : " REG_FMT " t0 : " REG_FMT "\n", (*regs).gp, (*regs).tp, (*regs).t0);
    pr_cont!(" t1 : " REG_FMT " t2 : " REG_FMT " s0 : " REG_FMT "\n", (*regs).t1, (*regs).t2, (*regs).s0);
    pr_cont!(" s1 : " REG_FMT " a0 : " REG_FMT " a1 : " REG_FMT "\n", (*regs).s1, (*regs).a0, (*regs).a1);
    pr_cont!(" a2 : " REG_FMT " a3 : " REG_FMT " a4 : " REG_FMT "\n", (*regs).a2, (*regs).a3, (*regs).a4);
    pr_cont!(" a5 : " REG_FMT " a6 : " REG_FMT " a7 : " REG_FMT "\n", (*regs).a5, (*regs).a6, (*regs).a7);
    pr_cont!(" s2 : " REG_FMT " s3 : " REG_FMT " s4 : " REG_FMT "\n", (*regs).s2, (*regs).s3, (*regs).s4);
    pr_cont!(" s5 : " REG_FMT " s6 : " REG_FMT " s7 : " REG_FMT "\n", (*regs).s5, (*regs).s6, (*regs).s7);
    pr_cont!(" s8 : " REG_FMT " s9 : " REG_FMT " s10: " REG_FMT "\n", (*regs).s8, (*regs).s9, (*regs).s10);
    pr_cont!(" s11: " REG_FMT " t3 : " REG_FMT " t4 : " REG_FMT "\n", (*regs).s11, (*regs).t3, (*regs).t4);
    pr_cont!(" t5 : " REG_FMT " t6 : " REG_FMT " ssp : " REG_FMT "\n", (*regs).t5, (*regs).t6, get_active_shstk(current));
    pr_cont!("status: " REG_FMT " badaddr: " REG_FMT " cause: " REG_FMT "\n", (*regs).status, (*regs).badaddr, (*regs).cause);
}

pub unsafe fn show_regs(regs: *mut pt_regs) { __show_regs(regs); if !user_mode(regs) { dump_backtrace(regs, core::ptr::null_mut(), KERN_DEFAULT); } }

pub unsafe fn arch_align_stack(mut sp: c_ulong) -> c_ulong {
    if ((*current).personality & ADDR_NO_RANDOMIZE) == 0 && randomize_va_space != 0 { sp -= get_random_u32_below(PAGE_SIZE) as c_ulong; }
    sp & !0xf
}

pub unsafe fn start_thread(regs: *mut pt_regs, pc: c_ulong, sp: c_ulong) {
    (*regs).status = SR_PIE;
    if has_fpu() { (*regs).status |= SR_FS_INITIAL; fstate_restore(current, regs); }
    (*regs).epc = pc; (*regs).sp = sp;
    set_shstk_lock(current, false); set_shstk_status(current, false); set_shstk_base(current, 0, 0); set_active_shstk(current, 0);
    set_indir_lp_lock(current, false); set_indir_lp_status(current, false);
    // CONFIG_64BIT: select the user XLEN according to compatibility mode.
    #[cfg(feature = "CONFIG_64BIT")]
    { (*regs).status &= !SR_UXL; if is_compat_task() { (*regs).status |= SR_UXL_32; } else { (*regs).status |= SR_UXL_64; } }
}

pub unsafe fn flush_thread() {
    // CONFIG_FPU
    fstate_off(current, task_pt_regs(current)); memset(&mut (*current).thread.fstate as *mut _ as *mut c_void, 0, core::mem::size_of_val(&(*current).thread.fstate));
    // CONFIG_RISCV_ISA_V
    riscv_v_vstate_ctrl_init(current); riscv_v_vstate_off(task_pt_regs(current)); kfree((*current).thread.vstate.datap); memset(&mut (*current).thread.vstate as *mut _ as *mut c_void, 0, core::mem::size_of::<__riscv_v_ext_state>()); clear_tsk_thread_flag(current, TIF_RISCV_V_DEFER_RESTORE);
    // CONFIG_RISCV_ISA_SUPM
    if riscv_has_extension_unlikely(RISCV_ISA_EXT_SUPM) { envcfg_update_bits(current, ENVCFG_PMM, ENVCFG_PMM_PMLEN_0); }
}

pub unsafe fn arch_release_task_struct(tsk: *mut task_struct) { if has_vector() || has_xtheadvector() { riscv_v_thread_free(tsk); } }

pub unsafe fn arch_dup_task_struct(dst: *mut task_struct, src: *mut task_struct) -> c_int {
    fstate_save(src, task_pt_regs(src)); *dst = *src;
    memset(&mut (*dst).thread.vstate as *mut _ as *mut c_void, 0, core::mem::size_of::<__riscv_v_ext_state>()); memset(&mut (*dst).thread.kernel_vstate as *mut _ as *mut c_void, 0, core::mem::size_of::<__riscv_v_ext_state>()); clear_tsk_thread_flag(dst, TIF_RISCV_V_DEFER_RESTORE); 0
}

pub unsafe extern "C" fn ret_from_fork_kernel(fn_arg: *mut c_void, func: Option<unsafe extern "C" fn(*mut c_void) -> c_int>, regs: *mut pt_regs) { if let Some(f) = func { f(fn_arg); } syscall_exit_to_user_mode(regs); }
pub unsafe extern "C" fn ret_from_fork_user(regs: *mut pt_regs) { syscall_exit_to_user_mode(regs); }

// The remainder mirrors copy_thread and the CONFIG_RISCV_ISA_SUPM sysctl code;
// declarations supplied by the included kernel headers retain their C ABI.
pub unsafe fn copy_thread(p: *mut task_struct, args: *const kernel_clone_args) -> c_int {
    let clone_flags = (*args).flags; let usp = (*args).stack; let tls = (*args).tls; let mut ssp = 0; let childregs = task_pt_regs(p);
    if IS_ENABLED!(CONFIG_RISCV_ISA_SUPM) && !(*p).mm.is_null() && clone_flags & CLONE_VM != 0 { set_bit(MM_CONTEXT_LOCK_PMLEN, &mut (*(*p).mm).context.flags); }
    memset(&mut (*p).thread.s as *mut _ as *mut c_void, 0, core::mem::size_of_val(&(*p).thread.s));
    if unlikely(!(*args).fn_.is_null()) { memset(childregs as *mut c_void, 0, core::mem::size_of::<pt_regs>()); (*childregs).status = SR_PP | SR_PIE; (*p).thread.s[0] = (*args).fn_ as c_ulong; (*p).thread.s[1] = (*args).fn_arg as c_ulong; (*p).thread.ra = ret_from_fork_kernel_asm as usize as c_ulong; }
    else { ssp = shstk_alloc_thread_stack(p, args); if IS_ERR_VALUE(ssp) { return PTR_ERR(ssp as *mut c_void); } *childregs = *current_pt_regs(); riscv_v_vstate_off(childregs); if usp != 0 { (*childregs).sp = usp; } if ssp != 0 { set_active_shstk(p, ssp); } if clone_flags & CLONE_SETTLS != 0 { (*childregs).tp = tls; } (*childregs).a0 = 0; (*p).thread.ra = ret_from_fork_user_asm as usize as c_ulong; }
    (*p).thread.riscv_v_flags = 0; if has_vector() || has_xtheadvector() { riscv_v_thread_alloc(p); } (*p).thread.sp = childregs as usize as c_ulong; 0
}

pub unsafe fn arch_task_cache_init() { riscv_v_setup_ctx_cache(); }

#[cfg(feature = "CONFIG_RISCV_ISA_SUPM")]
pub const PMLEN_0: c_int = 0;
#[cfg(feature = "CONFIG_RISCV_ISA_SUPM")]
pub const PMLEN_7: c_int = 7;
#[cfg(feature = "CONFIG_RISCV_ISA_SUPM")]
pub const PMLEN_16: c_int = 16;
#[cfg(feature = "CONFIG_RISCV_ISA_SUPM")]
static mut have_user_pmlen_7: bool = false;
#[cfg(feature = "CONFIG_RISCV_ISA_SUPM")]
static mut have_user_pmlen_16: bool = false;
#[cfg(feature = "CONFIG_RISCV_ISA_SUPM")]
static mut tagged_addr_disabled: c_uint = 0;
#[cfg(feature = "CONFIG_RISCV_ISA_SUPM")]
static tagged_addr_sysctl_table: [ctl_table; 2] = [
    ctl_table { procname: b"tagged_addr_disabled\0".as_ptr() as *mut c_char, mode: 0o644, data: core::ptr::null_mut(), maxlen: core::mem::size_of::<c_int>(), proc_handler: Some(proc_dointvec_minmax), extra1: SYSCTL_ZERO, extra2: SYSCTL_ONE },
    ctl_table::default(),
];

#[cfg(feature = "CONFIG_RISCV_ISA_SUPM")]
pub unsafe fn set_tagged_addr_ctrl(task: *mut task_struct, arg: c_ulong) -> c_long {
    let valid_mask = PR_PMLEN_MASK | PR_TAGGED_ADDR_ENABLE; let ti = task_thread_info(task); let mm = (*task).mm; let mut pmm; let mut pmlen = FIELD_GET(PR_PMLEN_MASK, arg) as c_uchar;
    if !riscv_has_extension_unlikely(RISCV_ISA_EXT_SUPM) || is_compat_thread(ti) || arg & !valid_mask != 0 { return -EINVAL as c_long; }
    if pmlen == PMLEN_0 { pmm = ENVCFG_PMM_PMLEN_0; } else if pmlen as c_int <= PMLEN_7 && have_user_pmlen_7 { pmlen = PMLEN_7 as c_uchar; pmm = ENVCFG_PMM_PMLEN_7; } else if pmlen as c_int <= PMLEN_16 && have_user_pmlen_16 { pmlen = PMLEN_16 as c_uchar; pmm = ENVCFG_PMM_PMLEN_16; } else { return -EINVAL as c_long; }
    if arg & PR_TAGGED_ADDR_ENABLE != 0 && (tagged_addr_disabled != 0 || pmlen == 0) { return -EINVAL as c_long; }
    if arg & PR_TAGGED_ADDR_ENABLE == 0 { pmlen = 0; pmm = ENVCFG_PMM_PMLEN_0; }
    if mmap_write_lock_killable(mm) != 0 { return -EINTR as c_long; }
    if test_bit(MM_CONTEXT_LOCK_PMLEN, &(*mm).context.flags) && (*mm).context.pmlen != pmlen { mmap_write_unlock(mm); return -EBUSY as c_long; }
    envcfg_update_bits(task, ENVCFG_PMM, pmm); (*mm).context.pmlen = pmlen; mmap_write_unlock(mm); 0
}

#[cfg(feature = "CONFIG_RISCV_ISA_SUPM")]
pub unsafe fn get_tagged_addr_ctrl(task: *mut task_struct) -> c_long {
    let ti = task_thread_info(task); if !riscv_has_extension_unlikely(RISCV_ISA_EXT_SUPM) || is_compat_thread(ti) { return -EINVAL as c_long; }
    let mut ret = 0; match (*task).thread.envcfg & ENVCFG_PMM { ENVCFG_PMM_PMLEN_7 => ret = FIELD_PREP(PR_PMLEN_MASK, PMLEN_7), ENVCFG_PMM_PMLEN_16 => ret = FIELD_PREP(PR_PMLEN_MASK, PMLEN_16), _ => {} } if (*(*task).mm).context.pmlen != 0 { ret |= PR_TAGGED_ADDR_ENABLE; } ret as c_long
}

#[cfg(feature = "CONFIG_RISCV_ISA_SUPM")]
unsafe fn try_to_set_pmm(value: c_ulong) -> bool { csr_set(CSR_ENVCFG, value); (csr_read_clear(CSR_ENVCFG, ENVCFG_PMM) & ENVCFG_PMM) == value }

#[cfg(feature = "CONFIG_RISCV_ISA_SUPM")]
pub unsafe fn tagged_addr_init() -> c_int {
    if !riscv_has_extension_unlikely(RISCV_ISA_EXT_SUPM) { return 0; }
    csr_clear(CSR_ENVCFG, ENVCFG_PMM); have_user_pmlen_7 = try_to_set_pmm(ENVCFG_PMM_PMLEN_7); have_user_pmlen_16 = try_to_set_pmm(ENVCFG_PMM_PMLEN_16);
    if register_sysctl("abi", tagged_addr_sysctl_table) == core::ptr::null_mut() { return -EINVAL; } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
