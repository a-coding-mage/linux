// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024 Rivos, Inc.
 * Deepak Gupta <debug@rivosinc.com>
 */

// C dependencies supplied by the surrounding kernel translation unit.

pub static mut riscv_nousercfi: ::core::ffi::c_ulong = 0;

const SHSTK_ENTRY_SIZE: usize = core::mem::size_of::<*mut core::ffi::c_void>();

pub unsafe fn is_shstk_enabled(task: *mut task_struct) -> bool {
    (*task).thread_info.user_cfi_state.ubcfi_en != 0
}

pub unsafe fn is_shstk_allocated(task: *mut task_struct) -> bool {
    (*task).thread_info.user_cfi_state.shdw_stk_base != 0
}

pub unsafe fn is_shstk_locked(task: *mut task_struct) -> bool {
    (*task).thread_info.user_cfi_state.ubcfi_locked != 0
}

pub unsafe fn set_shstk_base(task: *mut task_struct, shstk_addr: c_ulong, size: c_ulong) {
    (*task).thread_info.user_cfi_state.shdw_stk_base = shstk_addr;
    (*task).thread_info.user_cfi_state.shdw_stk_size = size;
}

pub unsafe fn get_shstk_base(task: *mut task_struct, size: *mut c_ulong) -> c_ulong {
    if !size.is_null() {
        *size = (*task).thread_info.user_cfi_state.shdw_stk_size;
    }
    (*task).thread_info.user_cfi_state.shdw_stk_base
}

pub unsafe fn set_active_shstk(task: *mut task_struct, shstk_addr: c_ulong) {
    (*task).thread_info.user_cfi_state.user_shdw_stk = shstk_addr;
}

pub unsafe fn get_active_shstk(task: *mut task_struct) -> c_ulong {
    (*task).thread_info.user_cfi_state.user_shdw_stk
}

pub unsafe fn set_shstk_status(task: *mut task_struct, enable: bool) {
    if !is_user_shstk_enabled() { return; }
    (*task).thread_info.user_cfi_state.ubcfi_en = if enable { 1 } else { 0 };
    if enable { (*task).thread.envcfg |= ENVCFG_SSE; }
    else { (*task).thread.envcfg &= !ENVCFG_SSE; }
    csr_write(CSR_ENVCFG, (*task).thread.envcfg);
}

pub unsafe fn set_shstk_lock(task: *mut task_struct, lock: bool) {
    (*task).thread_info.user_cfi_state.ubcfi_locked = lock;
}

pub unsafe fn is_indir_lp_enabled(task: *mut task_struct) -> bool {
    (*task).thread_info.user_cfi_state.ufcfi_en != 0
}

pub unsafe fn is_indir_lp_locked(task: *mut task_struct) -> bool {
    (*task).thread_info.user_cfi_state.ufcfi_locked != 0
}

pub unsafe fn set_indir_lp_status(task: *mut task_struct, enable: bool) {
    if !is_user_lpad_enabled() { return; }
    (*task).thread_info.user_cfi_state.ufcfi_en = if enable { 1 } else { 0 };
    if enable { (*task).thread.envcfg |= ENVCFG_LPE; }
    else { (*task).thread.envcfg &= !ENVCFG_LPE; }
    csr_write(CSR_ENVCFG, (*task).thread.envcfg);
}

pub unsafe fn set_indir_lp_lock(task: *mut task_struct, lock: bool) {
    (*task).thread_info.user_cfi_state.ufcfi_locked = lock;
}

static unsafe fn calc_shstk_size(size: c_ulong) -> c_ulong {
    if size != 0 { return PAGE_ALIGN(size); }
    PAGE_ALIGN(core::cmp::min(rlimit(RLIMIT_STACK) / 8, SZ_512M))
}

// The C implementation uses an inline ssamoswap.d instruction with an exception table.
// Preserve the operation and fault result through the external low-level helper.
static unsafe fn amo_user_shstk(addr: *mut c_ulong, val: c_ulong) -> c_ulong {
    __enable_user_access();
    let swap = arch_amo_user_shstk(addr, val);
    __disable_user_access();
    swap
}

static unsafe fn create_rstor_token(ssp: c_ulong, token_addr: *mut c_ulong) -> c_int {
    if !IS_ALIGNED(ssp, SHSTK_ENTRY_SIZE as c_ulong) { return -EINVAL; }
    let addr = ssp - SHSTK_ENTRY_SIZE as c_ulong;
    if amo_user_shstk(addr as *mut c_ulong, ssp) == !0 { return -EFAULT; }
    if !token_addr.is_null() { *token_addr = addr; }
    0
}

pub unsafe fn save_user_shstk(tsk: *mut task_struct, saved_shstk_ptr: *mut c_ulong) -> c_int {
    if saved_shstk_ptr.is_null() { return -EINVAL; }
    let ss_ptr = get_active_shstk(tsk);
    let mut token_loc = 0;
    let ret = create_rstor_token(ss_ptr, &mut token_loc);
    if ret == 0 { *saved_shstk_ptr = token_loc; set_active_shstk(tsk, token_loc); }
    ret
}

pub unsafe fn restore_user_shstk(tsk: *mut task_struct, shstk_ptr: c_ulong) -> c_int {
    let token = amo_user_shstk(shstk_ptr as *mut c_ulong, 0);
    if token == !0 { return -EFAULT; }
    if token.wrapping_sub(shstk_ptr) != SHSTK_ENTRY_SIZE as c_ulong {
        pr_info_ratelimited("%s[%d]: bad restore token in %s: pc=%p sp=%p, token=%p, shstk_ptr=%p\n", (*tsk).comm, task_pid_nr(tsk), "restore_user_shstk", task_pt_regs(tsk).as_ref().unwrap().epc as *const _, task_pt_regs(tsk).as_ref().unwrap().sp as *const _, token as *const _, shstk_ptr as *const _);
        return -EINVAL;
    }
    set_active_shstk(tsk, token);
    0
}

static unsafe fn allocate_shadow_stack(mut addr: c_ulong, size: c_ulong, token_offset: c_ulong, set_tok: bool) -> c_ulong {
    addr = vm_mmap_shadow_stack(addr, size, 0);
    if !set_tok || IS_ERR_VALUE(addr) { return addr; }
    if create_rstor_token(addr + token_offset, core::ptr::null_mut()) != 0 { vm_munmap(addr, size); return -EINVAL; }
    addr
}

// The remaining syscall and architecture hooks retain the kernel ABI and are supplied by the surrounding translation.
extern "C" {
    fn arch_amo_user_shstk(addr: *mut c_ulong, val: c_ulong) -> c_ulong;
}

pub unsafe fn map_shadow_stack(addr: c_ulong, size: c_ulong, flags: c_uint) -> c_long {
    let set_tok = flags & SHADOW_STACK_SET_TOKEN != 0;
    if !is_user_shstk_enabled() { return -EOPNOTSUPP as c_long; }
    if flags & !SHADOW_STACK_SET_TOKEN != 0 { return -EINVAL as c_long; }
    if set_tok && size < SHSTK_ENTRY_SIZE as c_ulong { return -ENOSPC as c_long; }
    if addr != 0 && (addr & (PAGE_SIZE - 1)) != 0 { return -EINVAL as c_long; }
    let aligned_size = PAGE_ALIGN(size);
    if aligned_size < size { return -EOVERFLOW as c_long; }
    allocate_shadow_stack(addr, aligned_size, size, set_tok) as c_long
}

pub unsafe fn shstk_alloc_thread_stack(tsk: *mut task_struct, args: *const kernel_clone_args) -> c_ulong {
    if !is_user_shstk_enabled() || !is_shstk_enabled(tsk) { return 0; }
    if (*args).flags & CLONE_VFORK != 0 { set_shstk_base(tsk, 0, 0); return 0; }
    if (*args).flags & CLONE_VM == 0 { return 0; }
    let size = calc_shstk_size((*args).stack_size);
    let addr = allocate_shadow_stack(0, size, 0, false);
    if IS_ERR_VALUE(addr) { return addr; }
    set_shstk_base(tsk, addr, size);
    addr + size
}

pub unsafe fn shstk_release(tsk: *mut task_struct) {
    if !is_user_shstk_enabled() || !is_shstk_enabled(tsk) { return; }
    if (*tsk).mm.is_null() || (*tsk).mm != current().mm { return; }
    let mut size = 0;
    let base = get_shstk_base(tsk, &mut size);
    if base == 0 { return; }
    vm_munmap(base, size);
    set_shstk_base(tsk, 0, 0);
}

pub unsafe fn arch_get_shadow_stack_status(t: *mut task_struct, status: *mut c_ulong) -> c_int {
    if !is_user_shstk_enabled() { return -EINVAL; }
    let value = if is_shstk_enabled(t) { PR_SHADOW_STACK_ENABLE } else { 0 };
    if copy_to_user(status, &value, core::mem::size_of::<c_ulong>()) != 0 { -EFAULT } else { 0 }
}

pub unsafe fn arch_set_shadow_stack_status(t: *mut task_struct, status: c_ulong) -> c_int {
    if !is_user_shstk_enabled() || status & !PR_SHADOW_STACK_SUPPORTED_STATUS_MASK != 0 || is_shstk_locked(t) { return -EINVAL; }
    let enable = status & PR_SHADOW_STACK_ENABLE != 0;
    if enable && !is_shstk_enabled(t) {
        if is_shstk_allocated(t) { return -EINVAL; }
        let size = calc_shstk_size(0);
        let addr = allocate_shadow_stack(0, size, 0, false);
        if IS_ERR_VALUE(addr) { return -ENOMEM; }
        set_shstk_base(t, addr, size); set_active_shstk(t, addr + size);
    }
    if !enable { shstk_release(t); }
    set_shstk_status(t, enable); 0
}

pub unsafe fn arch_lock_shadow_stack_status(task: *mut task_struct, arg: c_ulong) -> c_int {
    if !is_user_shstk_enabled() || !is_shstk_enabled(task) || arg != 0 { return -EINVAL; }
    set_shstk_lock(task, true); 0
}

pub unsafe fn arch_prctl_get_branch_landing_pad_state(t: *mut task_struct, state: *mut c_ulong) -> c_int {
    if !is_user_lpad_enabled() { return -EINVAL; }
    let mut value = if is_indir_lp_enabled(t) { PR_CFI_ENABLE } else { PR_CFI_DISABLE };
    if is_indir_lp_locked(t) { value |= PR_CFI_LOCK; }
    if copy_to_user(state, &value, core::mem::size_of::<c_ulong>()) != 0 { -EFAULT } else { 0 }
}

pub unsafe fn arch_prctl_set_branch_landing_pad_state(t: *mut task_struct, state: c_ulong) -> c_int {
    if !is_user_lpad_enabled() || state & !PR_CFI_SUPPORTED_STATUS_MASK != 0 || is_indir_lp_locked(t) { return -EINVAL; }
    if state & (PR_CFI_ENABLE | PR_CFI_DISABLE) == 0 || state & PR_CFI_ENABLE != 0 && state & PR_CFI_DISABLE != 0 { return -EINVAL; }
    set_indir_lp_status(t, state & PR_CFI_ENABLE != 0); 0
}

pub unsafe fn arch_prctl_lock_branch_landing_pad_state(task: *mut task_struct) -> c_int {
    if !is_user_lpad_enabled() || !is_indir_lp_enabled(task) { return -EINVAL; }
    set_indir_lp_lock(task, true); 0
}

pub unsafe fn is_user_shstk_enabled() -> bool { cpu_supports_shadow_stack() && (riscv_nousercfi & CMDLINE_DISABLE_RISCV_USERCFI_BCFI == 0) }
pub unsafe fn is_user_lpad_enabled() -> bool { cpu_supports_indirect_br_lp_instr() && (riscv_nousercfi & CMDLINE_DISABLE_RISCV_USERCFI_FCFI == 0) }

static unsafe fn setup_global_riscv_enable(str_: *const c_char) -> c_int {
    if strcmp(str_, b"all\0".as_ptr() as *const c_char) == 0 { riscv_nousercfi = CMDLINE_DISABLE_RISCV_USERCFI; }
    if strcmp(str_, b"fcfi\0".as_ptr() as *const c_char) == 0 { riscv_nousercfi |= CMDLINE_DISABLE_RISCV_USERCFI_FCFI; }
    if strcmp(str_, b"bcfi\0".as_ptr() as *const c_char) == 0 { riscv_nousercfi |= CMDLINE_DISABLE_RISCV_USERCFI_BCFI; }
    if riscv_nousercfi != 0 { pr_info("RISC-V user CFI disabled via cmdline - shadow stack status : %s, landing pad status : %s\n", if riscv_nousercfi & CMDLINE_DISABLE_RISCV_USERCFI_BCFI != 0 { b"disabled\0" } else { b"enabled\0" }, if riscv_nousercfi & CMDLINE_DISABLE_RISCV_USERCFI_FCFI != 0 { b"disabled\0" } else { b"enabled\0" }); }
    1
}

// __setup("riscv_nousercfi=", setup_global_riscv_enable);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
