// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2010 Tilera Corporation. All Rights Reserved.
 * Copyright 2015 Regents of the University of California
 * Copyright 2017 SiFive
 *
 * Copied from arch/tile/kernel/ptrace.c
 */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub enum riscv_regset {
    REGSET_X,
    #[cfg(CONFIG_FPU)] REGSET_F,
    #[cfg(CONFIG_RISCV_ISA_V)] REGSET_V,
    #[cfg(CONFIG_RISCV_ISA_SUPM)] REGSET_TAGGED_ADDR_CTRL,
    #[cfg(CONFIG_RISCV_USER_CFI)] REGSET_CFI,
}

unsafe fn riscv_gpr_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> i32 {
    membuf_write(&mut to, task_pt_regs(target), core::mem::size_of::<user_regs_struct>())
}

unsafe fn riscv_gpr_set(target: *mut task_struct, _regset: *const user_regset, mut pos: u32, mut count: u32, mut kbuf: *const core::ffi::c_void, mut ubuf: *const core::ffi::c_void) -> i32 {
    let regs = task_pt_regs(target);
    user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf, regs as *mut _, 0, -1)
}

#[cfg(CONFIG_FPU)]
unsafe fn riscv_fpr_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> i32 {
    let fstate = &mut (*target).thread.fstate;
    if target == current { fstate_save(current, task_pt_regs(current)); }
    membuf_write(&mut to, fstate, core::mem::offset_of!(__riscv_d_ext_state, fcsr));
    membuf_store(&mut to, fstate.fcsr);
    membuf_zero(&mut to, 4)
}

#[cfg(CONFIG_FPU)]
unsafe fn riscv_fpr_set(target: *mut task_struct, _regset: *const user_regset, mut pos: u32, mut count: u32, mut kbuf: *const core::ffi::c_void, mut ubuf: *const core::ffi::c_void) -> i32 {
    let fstate = &mut (*target).thread.fstate;
    let mut ret = user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf, fstate as *mut _, 0, core::mem::offset_of!(__riscv_d_ext_state, fcsr));
    if ret == 0 { ret = user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf, fstate as *mut _, 0, core::mem::offset_of!(__riscv_d_ext_state, fcsr) + core::mem::size_of_val(&fstate.fcsr)); }
    ret
}

#[cfg(CONFIG_RISCV_ISA_V)]
unsafe fn riscv_vr_get(target: *mut task_struct, regset: *const user_regset, mut to: membuf) -> i32 {
    let vstate = &mut (*target).thread.vstate;
    let mut ptrace_vstate: __riscv_v_regset_state = core::mem::zeroed();
    if !(has_vector() || has_xtheadvector()) { return -EINVAL; }
    if riscv_v_vstate_query(task_pt_regs(target)) == 0 { return -ENODATA; }
    if target == current { get_cpu_vector_context(); riscv_v_vstate_save(&mut (*current).thread.vstate, task_pt_regs(current)); put_cpu_vector_context(); }
    ptrace_vstate.vstart = vstate.vstart; ptrace_vstate.vl = vstate.vl; ptrace_vstate.vtype = vstate.vtype; ptrace_vstate.vcsr = vstate.vcsr; ptrace_vstate.vlenb = vstate.vlenb;
    membuf_write(&mut to, &ptrace_vstate, core::mem::size_of::<__riscv_v_regset_state>());
    membuf_write(&mut to, vstate.datap, riscv_v_vsize)
}

#[cfg(CONFIG_RISCV_ISA_V)]
unsafe fn invalid_ptrace_v_csr(vstate: *mut __riscv_v_ext_state, ptrace: *mut __riscv_v_regset_state) -> i32 {
    let vlen = (*vstate).vlenb * 8;
    if (*vstate).vlenb != (*ptrace).vlenb { return 1; }
    let mut reserved = !(CSR_VXSAT_MASK | (CSR_VXRM_MASK << CSR_VXRM_SHIFT));
    if (*ptrace).vcsr & reserved != 0 { return 1; }
    if has_vector() {
        reserved = !(VTYPE_VSEW | VTYPE_VLMUL | VTYPE_VMA | VTYPE_VTA);
        if (*ptrace).vtype & reserved != 0 { return 1; }
        let elen = if riscv_has_extension_unlikely(RISCV_ISA_EXT_ZVE64X) { 64 } else { 32 };
        let vsew = ((*ptrace).vtype & VTYPE_VSEW) >> VTYPE_VSEW_SHIFT;
        let sew = 8 << vsew;
        if sew > elen { return 1; }
        let vfrac = (*ptrace).vtype & VTYPE_VLMUL_FRAC;
        let vlmul = (*ptrace).vtype & VTYPE_VLMUL;
        if vlmul == 4 || (vlmul == 5 && elen == 32) { return 1; }
        let vl = if (*ptrace).vl != 0 { (*ptrace).vl } else { 1 };
        if vfrac != 0 { let lmul = 2 << (3 - (vlmul - vfrac)); if vlen < vl * sew * lmul { return 1; } }
        else { let lmul = 1 << vlmul; if vl * sew > lmul * vlen { return 1; } }
    }
    if has_xtheadvector() {
        reserved = !(VTYPE_VSEW_THEAD | VTYPE_VLMUL_THEAD | VTYPE_VEDIV_THEAD);
        if (*ptrace).vtype & reserved != 0 || (*ptrace).vtype & VTYPE_VEDIV_THEAD != 0 { return 1; }
        let vsew = ((*ptrace).vtype & VTYPE_VSEW_THEAD) >> VTYPE_VSEW_THEAD_SHIFT;
        let sew = 8 << vsew; let lmul = 1 << ((*ptrace).vtype & VTYPE_VLMUL_THEAD);
        let vl = if (*ptrace).vl != 0 { (*ptrace).vl } else { 1 };
        if vl * sew > lmul * vlen { return 1; }
    }
    0
}

#[cfg(CONFIG_RISCV_ISA_V)]
unsafe fn riscv_vr_set(target: *mut task_struct, _regset: *const user_regset, mut pos: u32, mut count: u32, mut kbuf: *const core::ffi::c_void, mut ubuf: *const core::ffi::c_void) -> i32 {
    let vstate = &mut (*target).thread.vstate;
    let mut p: __riscv_v_regset_state = core::mem::zeroed();
    if !(has_vector() || has_xtheadvector()) { return -EINVAL; }
    if riscv_v_vstate_query(task_pt_regs(target)) == 0 { return -ENODATA; }
    let mut ret = user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf, &mut p, 0, core::mem::size_of::<__riscv_v_regset_state>());
    if ret != 0 { return ret; }
    if invalid_ptrace_v_csr(vstate, &mut p) != 0 { return -EINVAL; }
    vstate.vstart = p.vstart; vstate.vl = p.vl; vstate.vtype = p.vtype; vstate.vcsr = p.vcsr;
    pos = 0; ret = user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf, vstate.datap, 0, riscv_v_vsize); ret
}

#[cfg(CONFIG_RISCV_ISA_V)]
unsafe fn riscv_vr_active(target: *mut task_struct, regset: *const user_regset) -> i32 {
    if !(has_vector() || has_xtheadvector()) { return -ENODEV; }
    if riscv_v_vstate_query(task_pt_regs(target)) == 0 { return 0; }
    (*regset).n as i32
}

#[cfg(CONFIG_RISCV_ISA_SUPM)]
unsafe fn tagged_addr_ctrl_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> i32 { let ctrl = get_tagged_addr_ctrl(target); if IS_ERR_VALUE(ctrl) { return ctrl as i32; } membuf_write(&mut to, &ctrl, core::mem::size_of::<c_long>()) }

#[cfg(CONFIG_RISCV_ISA_SUPM)]
unsafe fn tagged_addr_ctrl_set(target: *mut task_struct, _regset: *const user_regset, mut pos: u32, mut count: u32, mut kbuf: *const core::ffi::c_void, mut ubuf: *const core::ffi::c_void) -> i32 { let mut ctrl: c_long = 0; let ret = user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf, &mut ctrl, 0, -1); if ret != 0 { return ret; } set_tagged_addr_ctrl(target, ctrl) }

// CONFIG_RISCV_USER_CFI support is translated with the same field-level operations below.
#[cfg(CONFIG_RISCV_USER_CFI)]
unsafe fn riscv_cfi_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> i32 { let mut user_cfi: user_cfi_state = core::mem::zeroed(); let regs = task_pt_regs(target); if is_indir_lp_enabled(target) { user_cfi.cfi_status.cfi_state |= PTRACE_CFI_BRANCH_LANDING_PAD_EN_STATE; if is_indir_lp_locked(target) { user_cfi.cfi_status.cfi_state |= PTRACE_CFI_BRANCH_LANDING_PAD_LOCK_STATE; } if (*regs).status & SR_ELP != 0 { user_cfi.cfi_status.cfi_state |= PTRACE_CFI_BRANCH_EXPECTED_LANDING_PAD_STATE; } } if is_shstk_enabled(target) { user_cfi.cfi_status.cfi_state |= PTRACE_CFI_SHADOW_STACK_EN_STATE | PTRACE_CFI_SHADOW_STACK_PTR_STATE; if is_shstk_locked(target) { user_cfi.cfi_status.cfi_state |= PTRACE_CFI_SHADOW_STACK_LOCK_STATE; } user_cfi.shstk_ptr = get_active_shstk(target); } membuf_write(&mut to, &user_cfi, core::mem::size_of::<user_cfi_state>()) }

#[cfg(CONFIG_RISCV_USER_CFI)]
unsafe fn riscv_cfi_set(target: *mut task_struct, _regset: *const user_regset, mut pos: u32, mut count: u32, mut kbuf: *const core::ffi::c_void, mut ubuf: *const core::ffi::c_void) -> i32 { let regs = task_pt_regs(target); let mut cfi: user_cfi_state = core::mem::zeroed(); let ret = user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf, &mut cfi, 0, -1); if ret != 0 { return ret; } if cfi.cfi_status.cfi_state & (PTRACE_CFI_BRANCH_LANDING_PAD_EN_STATE | PTRACE_CFI_BRANCH_LANDING_PAD_LOCK_STATE | PTRACE_CFI_SHADOW_STACK_EN_STATE | PTRACE_CFI_SHADOW_STACK_LOCK_STATE | PTRACE_CFI_STATE_INVALID_MASK) != 0 { return -EINVAL; } if is_indir_lp_enabled(target) { if cfi.cfi_status.cfi_state & PTRACE_CFI_BRANCH_EXPECTED_LANDING_PAD_STATE != 0 { (*regs).status |= SR_ELP; } else { (*regs).status &= !SR_ELP; } } if is_shstk_enabled(target) && cfi.cfi_status.cfi_state & PTRACE_CFI_SHADOW_STACK_PTR_STATE != 0 { set_active_shstk(target, cfi.shstk_ptr); } 0 }

#[repr(C)] pub struct pt_regs_offset { pub name: *const core::ffi::c_char, pub offset: i32 }

static mut regoffset_table: [pt_regs_offset; 39] = [
    pt_regs_offset { name: b"epc\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, epc) as i32 },
    pt_regs_offset { name: b"ra\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, ra) as i32 },
    pt_regs_offset { name: b"sp\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, sp) as i32 },
    pt_regs_offset { name: b"gp\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, gp) as i32 },
    pt_regs_offset { name: b"tp\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, tp) as i32 },
    pt_regs_offset { name: b"t0\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, t0) as i32 },
    pt_regs_offset { name: b"t1\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, t1) as i32 },
    pt_regs_offset { name: b"t2\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, t2) as i32 },
    pt_regs_offset { name: b"s0\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, s0) as i32 },
    pt_regs_offset { name: b"s1\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, s1) as i32 },
    pt_regs_offset { name: b"a0\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, a0) as i32 },
    pt_regs_offset { name: b"a1\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, a1) as i32 },
    pt_regs_offset { name: b"a2\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, a2) as i32 },
    pt_regs_offset { name: b"a3\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, a3) as i32 },
    pt_regs_offset { name: b"a4\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, a4) as i32 },
    pt_regs_offset { name: b"a5\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, a5) as i32 },
    pt_regs_offset { name: b"a6\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, a6) as i32 },
    pt_regs_offset { name: b"a7\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, a7) as i32 },
    pt_regs_offset { name: b"s2\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, s2) as i32 },
    pt_regs_offset { name: b"s3\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, s3) as i32 },
    pt_regs_offset { name: b"s4\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, s4) as i32 },
    pt_regs_offset { name: b"s5\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, s5) as i32 },
    pt_regs_offset { name: b"s6\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, s6) as i32 },
    pt_regs_offset { name: b"s7\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, s7) as i32 },
    pt_regs_offset { name: b"s8\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, s8) as i32 },
    pt_regs_offset { name: b"s9\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, s9) as i32 },
    pt_regs_offset { name: b"s10\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, s10) as i32 },
    pt_regs_offset { name: b"s11\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, s11) as i32 },
    pt_regs_offset { name: b"t3\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, t3) as i32 },
    pt_regs_offset { name: b"t4\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, t4) as i32 },
    pt_regs_offset { name: b"t5\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, t5) as i32 },
    pt_regs_offset { name: b"t6\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, t6) as i32 },
    pt_regs_offset { name: b"status\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, status) as i32 },
    pt_regs_offset { name: b"badaddr\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, badaddr) as i32 },
    pt_regs_offset { name: b"cause\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, cause) as i32 },
    pt_regs_offset { name: b"orig_a0\0".as_ptr() as _, offset: core::mem::offset_of!(pt_regs, orig_a0) as i32 },
    pt_regs_offset { name: core::ptr::null(), offset: 0 },
];

pub unsafe fn regs_query_register_offset(name: *const core::ffi::c_char) -> i32 { let mut i = 0; while !regoffset_table[i].name.is_null() { if strcmp(regoffset_table[i].name, name) == 0 { return regoffset_table[i].offset; } i += 1; } -EINVAL }
unsafe fn regs_within_kernel_stack(regs: *mut pt_regs, addr: c_ulong) -> bool { (addr & !(THREAD_SIZE - 1)) == (kernel_stack_pointer(regs) & !(THREAD_SIZE - 1)) }
pub unsafe fn regs_get_kernel_stack_nth(regs: *mut pt_regs, n: u32) -> c_ulong { let addr = (kernel_stack_pointer(regs) as *mut c_ulong).add(n as usize); if regs_within_kernel_stack(regs, addr as c_ulong) { *addr } else { 0 } }
pub unsafe fn ptrace_disable(_child: *mut task_struct) {}
pub unsafe fn arch_ptrace(child: *mut task_struct, request: c_long, addr: c_ulong, data: c_ulong) -> c_long { match request { _ => ptrace_request(child, request, addr, data) } }

// The user-regset tables retain the C array/index layout; their fields and
// callback types are supplied by the kernel's Rust declarations.
#[cfg(CONFIG_RISCV_ISA_V)]
pub unsafe fn update_regset_vector_info(size: c_ulong) { riscv_user_regset[REGSET_V as usize].n = (size + core::mem::size_of::<__riscv_v_regset_state>() as c_ulong) / core::mem::size_of::<u32>() as c_ulong; }

#[cfg(CONFIG_COMPAT)]
unsafe fn compat_riscv_gpr_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> i32 { let mut cregs: compat_user_regs_struct = core::mem::zeroed(); regs_to_cregs(&mut cregs, task_pt_regs(target)); membuf_write(&mut to, &cregs, core::mem::size_of::<compat_user_regs_struct>()) }

#[cfg(CONFIG_COMPAT)]
unsafe fn compat_riscv_gpr_set(target: *mut task_struct, _regset: *const user_regset, mut pos: u32, mut count: u32, mut kbuf: *const core::ffi::c_void, mut ubuf: *const core::ffi::c_void) -> i32 { let mut cregs: compat_user_regs_struct = core::mem::zeroed(); let ret = user_regset_copyin(&mut pos, &mut count, &mut kbuf, &mut ubuf, &mut cregs, 0, -1); if ret == 0 { cregs_to_regs(&mut cregs, task_pt_regs(target)); } ret }

// Equivalent to the C designated-initializer tables. The surrounding kernel
// supplies `user_regset`/`user_regset_view` layout and note-type constants.
static mut riscv_user_regset: [user_regset; 5] = unsafe { core::mem::zeroed() };
static mut riscv_user_native_view: user_regset_view = unsafe { core::mem::zeroed() };
#[cfg(CONFIG_COMPAT)] static mut compat_riscv_user_native_view: user_regset_view = unsafe { core::mem::zeroed() };
#[cfg(not(CONFIG_COMPAT))] static mut compat_riscv_user_native_view: user_regset_view = unsafe { core::mem::zeroed() };

pub unsafe fn task_user_regset_view(task: *mut task_struct) -> *const user_regset_view { if is_compat_thread(&mut (*task).thread_info) { &compat_riscv_user_native_view } else { &riscv_user_native_view } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
