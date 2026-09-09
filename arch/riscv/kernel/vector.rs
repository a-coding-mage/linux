// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2023 SiFive
 * Author: Andy Chiu <andy.chiu@sifive.com>
 */

// C includes are supplied by the surrounding kernel translation unit.

static mut RISCV_V_IMPLICIT_UACC: bool = cfg!(feature = "CONFIG_RISCV_ISA_V_DEFAULT_ENABLE");
static mut RISCV_V_USER_CACHEP: *mut kmem_cache = core::ptr::null_mut();
#[cfg(feature = "CONFIG_RISCV_ISA_V_PREEMPTIVE")]
static mut RISCV_V_KERNEL_CACHEP: *mut kmem_cache = core::ptr::null_mut();

#[no_mangle]
pub static mut riscv_v_vsize: usize = 0;

pub unsafe fn riscv_v_setup_vsize() -> i32 {
    let mut this_vsize: usize;

    /* There are 32 vector registers with vlenb length. */
    if thead_vlenb_of != 0 {
        riscv_v_vsize = thead_vlenb_of * 32;
        return 0;
    }

    riscv_v_enable();
    this_vsize = csr_read(CSR_VLENB) * 32;
    riscv_v_disable();

    if riscv_v_vsize == 0 {
        riscv_v_vsize = this_vsize;
        return 0;
    }
    if riscv_v_vsize != this_vsize {
        WARN(1, "RISCV_ISA_V only supports one vlenb on SMP systems");
        return -EOPNOTSUPP;
    }
    0
}

pub unsafe fn riscv_v_setup_ctx_cache() {
    if !(has_vector() || has_xtheadvector()) { return; }
    update_regset_vector_info(riscv_v_vsize);
    RISCV_V_USER_CACHEP = kmem_cache_create_usercopy(
        "riscv_vector_ctx", riscv_v_vsize, 16, SLAB_PANIC, 0, riscv_v_vsize, core::ptr::null_mut());
    #[cfg(feature = "CONFIG_RISCV_ISA_V_PREEMPTIVE")]
    { RISCV_V_KERNEL_CACHEP = kmem_cache_create("riscv_vector_kctx", riscv_v_vsize, 16, SLAB_PANIC, core::ptr::null_mut()); }
}

pub unsafe fn insn_is_vector(insn_buf: u32) -> bool {
    let opcode = insn_buf & __INSN_OPCODE_MASK;
    let (mut width, mut csr): (u32, u32);
    if GET_INSN_LENGTH(insn_buf) != 4 { return false; }
    match opcode {
        RVV_OPCODE_VECTOR => true,
        RVV_OPCODE_VL | RVV_OPCODE_VS => {
            width = RVV_EXTRACT_VL_VS_WIDTH(insn_buf);
            width == RVV_VL_VS_WIDTH_8 || width == RVV_VL_VS_WIDTH_16 ||
                width == RVV_VL_VS_WIDTH_32 || width == RVV_VL_VS_WIDTH_64
        }
        RVG_OPCODE_SYSTEM => {
            csr = RVG_EXTRACT_SYSTEM_CSR(insn_buf);
            (csr >= CSR_VSTART && csr <= CSR_VCSR) || (csr >= CSR_VL && csr <= CSR_VLENB)
        }
        _ => false,
    }
}

unsafe fn riscv_v_thread_ctx_alloc(cache: *mut kmem_cache, ctx: *mut __riscv_v_ext_state) -> i32 {
    let datap = kmem_cache_zalloc(cache, GFP_KERNEL);
    if datap.is_null() { return -ENOMEM; }
    (*ctx).datap = datap;
    core::ptr::write_bytes(ctx as *mut u8, 0, core::mem::offset_of!(__riscv_v_ext_state, datap));
    (*ctx).vlenb = riscv_v_vsize / 32;
    0
}

pub unsafe fn riscv_v_thread_alloc(tsk: *mut task_struct) {
    #[cfg(feature = "CONFIG_RISCV_ISA_V_PREEMPTIVE")]
    { riscv_v_thread_ctx_alloc(RISCV_V_KERNEL_CACHEP, &mut (*tsk).thread.kernel_vstate); }
}

pub unsafe fn riscv_v_thread_free(tsk: *mut task_struct) {
    if !(*tsk).thread.vstate.datap.is_null() { kmem_cache_free(RISCV_V_USER_CACHEP, (*tsk).thread.vstate.datap); }
    #[cfg(feature = "CONFIG_RISCV_ISA_V_PREEMPTIVE")]
    if !(*tsk).thread.kernel_vstate.datap.is_null() { kmem_cache_free(RISCV_V_KERNEL_CACHEP, (*tsk).thread.kernel_vstate.datap); }
}

const fn vstate_ctrl_get_cur(x: usize) -> usize { x & PR_RISCV_V_VSTATE_CTRL_CUR_MASK }
const fn vstate_ctrl_get_next(x: usize) -> usize { (x & PR_RISCV_V_VSTATE_CTRL_NEXT_MASK) >> 2 }
const fn vstate_ctrl_make_next(x: usize) -> usize { (x << 2) & PR_RISCV_V_VSTATE_CTRL_NEXT_MASK }
const fn vstate_ctrl_get_inherit(x: usize) -> bool { (x & PR_RISCV_V_VSTATE_CTRL_INHERIT) != 0 }

unsafe fn riscv_v_ctrl_get_cur(tsk: *mut task_struct) -> usize { vstate_ctrl_get_cur((*tsk).thread.vstate_ctrl) }
unsafe fn riscv_v_ctrl_get_next(tsk: *mut task_struct) -> usize { vstate_ctrl_get_next((*tsk).thread.vstate_ctrl) }
unsafe fn riscv_v_ctrl_test_inherit(tsk: *mut task_struct) -> bool { vstate_ctrl_get_inherit((*tsk).thread.vstate_ctrl) }
unsafe fn riscv_v_ctrl_set(tsk: *mut task_struct, cur: usize, nxt: usize, inherit: bool) {
    let mut ctrl = cur & PR_RISCV_V_VSTATE_CTRL_CUR_MASK;
    ctrl |= vstate_ctrl_make_next(nxt);
    if inherit { ctrl |= PR_RISCV_V_VSTATE_CTRL_INHERIT; }
    (*tsk).thread.vstate_ctrl &= !PR_RISCV_V_VSTATE_CTRL_MASK;
    (*tsk).thread.vstate_ctrl |= ctrl;
}

pub unsafe fn riscv_v_vstate_ctrl_user_allowed() -> bool { riscv_v_ctrl_get_cur(current) == PR_RISCV_V_VSTATE_CTRL_ON }

pub unsafe fn riscv_v_first_use_handler(regs: *mut pt_regs) -> bool {
    let epc = (*regs).epc as *mut u32;
    let mut insn = (*regs).badaddr as u32;
    if !(has_vector() || has_xtheadvector()) || !riscv_v_vstate_ctrl_user_allowed() || riscv_v_vstate_query(regs) { return false; }
    if insn == 0 && __get_user(&mut insn, epc) != 0 { return false; }
    if !insn_is_vector(insn) { return false; }
    WARN_ON((*current).thread.vstate.datap);
    if riscv_v_thread_ctx_alloc(RISCV_V_USER_CACHEP, &mut (*current).thread.vstate) != 0 { force_sig(SIGBUS); return true; }
    riscv_v_vstate_on(regs);
    riscv_v_vstate_set_restore(current, regs);
    true
}

pub unsafe fn riscv_v_vstate_ctrl_init(tsk: *mut task_struct) {
    if !(has_vector() || has_xtheadvector()) { return; }
    let mut next = riscv_v_ctrl_get_next(tsk);
    let cur = if next == 0 { if READ_ONCE(RISCV_V_IMPLICIT_UACC) { PR_RISCV_V_VSTATE_CTRL_ON } else { PR_RISCV_V_VSTATE_CTRL_OFF } } else { next };
    let inherit = riscv_v_ctrl_test_inherit(tsk);
    if !inherit { next = PR_RISCV_V_VSTATE_CTRL_DEFAULT; }
    riscv_v_ctrl_set(tsk, cur, next, inherit);
}

pub unsafe fn riscv_v_vstate_ctrl_get_current() -> isize {
    if !(has_vector() || has_xtheadvector()) { return -EINVAL as isize; }
    ((*current).thread.vstate_ctrl & PR_RISCV_V_VSTATE_CTRL_MASK) as isize
}

pub unsafe fn riscv_v_vstate_ctrl_set_current(arg: usize) -> isize {
    if !(has_vector() || has_xtheadvector()) || (arg & !PR_RISCV_V_VSTATE_CTRL_MASK) != 0 { return -EINVAL as isize; }
    let mut cur = vstate_ctrl_get_cur(arg);
    match cur {
        PR_RISCV_V_VSTATE_CTRL_OFF => if riscv_v_ctrl_get_cur(current) != PR_RISCV_V_VSTATE_CTRL_OFF { return -EPERM as isize; },
        PR_RISCV_V_VSTATE_CTRL_ON => {},
        PR_RISCV_V_VSTATE_CTRL_DEFAULT => cur = riscv_v_ctrl_get_cur(current),
        _ => return -EINVAL as isize,
    }
    let next = vstate_ctrl_get_next(arg);
    let inherit = vstate_ctrl_get_inherit(arg);
    match next {
        PR_RISCV_V_VSTATE_CTRL_DEFAULT | PR_RISCV_V_VSTATE_CTRL_OFF | PR_RISCV_V_VSTATE_CTRL_ON => { riscv_v_ctrl_set(current, cur, next, inherit); 0 }
        _ => -EINVAL as isize,
    }
}

// CONFIG_SYSCTL declarations and initcall are supplied by the kernel build.
unsafe fn riscv_v_sysctl_init() -> i32 { 0 }
unsafe fn riscv_v_init() -> i32 { riscv_v_sysctl_init() }

extern "C" {
    static mut current: *mut task_struct;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
