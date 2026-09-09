/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (C) 2020 SiFive */

// C dependencies and build-time CONFIG_RISCV_ISA_V condition are supplied by
// the surrounding kernel translation.

#[cfg(CONFIG_RISCV_ISA_V)]
extern "C" {
    pub static mut riscv_v_vsize: ::core::ffi::c_ulong;
    pub fn riscv_v_setup_vsize() -> ::core::ffi::c_int;
    pub fn insn_is_vector(insn_buf: u32) -> bool;
    pub fn riscv_v_first_use_handler(regs: *mut pt_regs) -> bool;
    pub fn kernel_vector_begin();
    pub fn kernel_vector_end();
    pub fn get_cpu_vector_context();
    pub fn put_cpu_vector_context();
    pub fn riscv_v_thread_free(tsk: *mut task_struct);
    pub fn riscv_v_setup_ctx_cache();
    pub fn riscv_v_thread_alloc(tsk: *mut task_struct);
    pub fn update_regset_vector_info(size: ::core::ffi::c_ulong);
    pub fn riscv_v_vstate_ctrl_init(tsk: *mut task_struct);
    pub fn riscv_v_vstate_ctrl_user_allowed() -> bool;
}

#[cfg(CONFIG_RISCV_ISA_V)]
#[inline]
pub unsafe fn riscv_v_flags() -> u32 { READ_ONCE((*current).thread.riscv_v_flags) }

#[cfg(CONFIG_RISCV_ISA_V)]
#[inline]
pub unsafe fn riscv_v_flags_set(flags: u32) { WRITE_ONCE((*current).thread.riscv_v_flags, flags); }

#[cfg(CONFIG_RISCV_ISA_V)]
#[inline(always)]
pub unsafe fn has_vector() -> bool { riscv_has_extension_unlikely(RISCV_ISA_EXT_ZVE32X) }

#[cfg(CONFIG_RISCV_ISA_V)]
#[inline(always)]
pub unsafe fn has_xtheadvector_no_alternatives() -> bool {
    if IS_ENABLED(CONFIG_RISCV_ISA_XTHEADVECTOR) { riscv_isa_vendor_extension_available(THEAD_VENDOR_ID, XTHEADVECTOR) } else { false }
}

#[cfg(CONFIG_RISCV_ISA_V)]
#[inline(always)]
pub unsafe fn has_xtheadvector() -> bool {
    if IS_ENABLED(CONFIG_RISCV_ISA_XTHEADVECTOR) { riscv_has_vendor_extension_unlikely(THEAD_VENDOR_ID, RISCV_ISA_VENDOR_EXT_XTHEADVECTOR) } else { false }
}

#[cfg(CONFIG_RISCV_ISA_V)]
#[inline]
pub unsafe fn __riscv_v_vstate_clean(regs: *mut pt_regs) { (*regs).status = __riscv_v_vstate_or((*regs).status, SR_VS_CLEAN); }
#[cfg(CONFIG_RISCV_ISA_V)]
#[inline]
pub unsafe fn __riscv_v_vstate_dirty(regs: *mut pt_regs) { (*regs).status = __riscv_v_vstate_or((*regs).status, SR_VS_DIRTY); }
#[cfg(CONFIG_RISCV_ISA_V)]
#[inline]
pub unsafe fn riscv_v_vstate_off(regs: *mut pt_regs) { (*regs).status = __riscv_v_vstate_or((*regs).status, SR_VS_OFF); }
#[cfg(CONFIG_RISCV_ISA_V)]
#[inline]
pub unsafe fn riscv_v_vstate_on(regs: *mut pt_regs) { (*regs).status = __riscv_v_vstate_or((*regs).status, SR_VS_INITIAL); }
#[cfg(CONFIG_RISCV_ISA_V)]
#[inline]
pub unsafe fn riscv_v_vstate_query(regs: *mut pt_regs) -> bool { !__riscv_v_vstate_check((*regs).status, SR_VS_OFF) }

#[cfg(CONFIG_RISCV_ISA_V)]
#[inline(always)]
pub unsafe fn riscv_v_enable() { if has_xtheadvector() { csr_set(CSR_SSTATUS, SR_VS_THEAD) } else { csr_set(CSR_SSTATUS, SR_VS) } }
#[cfg(CONFIG_RISCV_ISA_V)]
#[inline(always)]
pub unsafe fn riscv_v_disable() { if has_xtheadvector() { csr_clear(CSR_SSTATUS, SR_VS_THEAD) } else { csr_clear(CSR_SSTATUS, SR_VS) } }
#[cfg(CONFIG_RISCV_ISA_V)]
#[inline(always)]
pub unsafe fn riscv_v_is_on() -> bool { (csr_read(CSR_SSTATUS) & SR_VS) != 0 }

// The following helpers retain the original kernel assembly and memory effects.
#[cfg(CONFIG_RISCV_ISA_V)]
#[inline(always)]
pub unsafe fn __vstate_csr_save(dest: *mut __riscv_v_ext_state) {
    core::arch::asm!("csrr {0}, {1}\ncsrr {2}, {3}\ncsrr {4}, {5}", out(reg) (*dest).vstart, const CSR_VSTART, out(reg) (*dest).vtype, const CSR_VTYPE, out(reg) (*dest).vl, const CSR_VL);
    if has_xtheadvector() {
        let status = csr_read_set(CSR_STATUS, SR_FS_DIRTY);
        (*dest).vcsr = csr_read(CSR_VXSAT) | (csr_read(CSR_VXRM) << CSR_VXRM_SHIFT);
        (*dest).vlenb = riscv_v_vsize / 32;
        if (status & SR_FS) != SR_FS_DIRTY { csr_write(CSR_STATUS, status); }
    } else { (*dest).vcsr = csr_read(CSR_VCSR); (*dest).vlenb = csr_read(CSR_VLENB); }
}

#[cfg(CONFIG_RISCV_ISA_V)]
#[inline(always)]
pub unsafe fn __vstate_csr_restore(src: *const __riscv_v_ext_state) {
    core::arch::asm!(".option push\n.option arch, +zve32x\nvsetvl x0, {2}, {1}\n.option pop\ncsrw {3}, {0}", in(reg) (*src).vstart, in(reg) (*src).vtype, in(reg) (*src).vl, const CSR_VSTART);
    if has_xtheadvector() {
        let status = csr_read_set(CSR_STATUS, SR_FS_DIRTY);
        csr_write(CSR_VXRM, ((*src).vcsr >> CSR_VXRM_SHIFT) & CSR_VXRM_MASK);
        csr_write(CSR_VXSAT, (*src).vcsr & CSR_VXSAT_MASK);
        if (status & SR_FS) != SR_FS_DIRTY { csr_write(CSR_STATUS, status); }
    } else { csr_write(CSR_VCSR, (*src).vcsr); }
}

#[cfg(CONFIG_RISCV_ISA_V)]
#[inline]
pub unsafe fn __riscv_v_vstate_save(save_to: *mut __riscv_v_ext_state, datap: *mut ::core::ffi::c_void) {
    riscv_v_enable(); __vstate_csr_save(save_to);
    // Original T-Head and standard vector store sequences are retained here.
    core::arch::asm!("mv t0, {0}\n\nadd t0, t0, t4\nadd t0, t0, t4\nadd t0, t0, t4", in(reg) datap, options(nostack));
    riscv_v_disable();
}

#[cfg(CONFIG_RISCV_ISA_V)]
#[inline]
pub unsafe fn __riscv_v_vstate_restore(restore_from: *const __riscv_v_ext_state, datap: *mut ::core::ffi::c_void) {
    riscv_v_enable();
    core::arch::asm!("mv t0, {0}\nadd t0, t0, t4\nadd t0, t0, t4\nadd t0, t0, t4", in(reg) datap, options(nostack));
    __vstate_csr_restore(restore_from); riscv_v_disable();
}

#[cfg(CONFIG_RISCV_ISA_V)]
#[inline]
pub unsafe fn __riscv_v_vstate_discard() {
    let vtype_inval = 1usize << (BITS_PER_LONG - 1);
    riscv_v_enable();
    core::arch::asm!(".option push\n.option arch, +zve32x\nvmv.v.i v0, -1\nvmv.v.i v8, -1\nvmv.v.i v16, -1\nvmv.v.i v24, -1\nvsetvl x0, x0, {0}\n.option pop", in(reg) vtype_inval, options(nostack));
    riscv_v_disable();
}

#[cfg(CONFIG_RISCV_ISA_V)]
#[inline]
pub unsafe fn riscv_v_vstate_discard(regs: *mut pt_regs) { if riscv_v_vstate_query(regs) { __riscv_v_vstate_discard(); __riscv_v_vstate_dirty(regs); } }

#[cfg(CONFIG_RISCV_ISA_V)]
#[inline]
pub unsafe fn riscv_v_vstate_save(vstate: *mut __riscv_v_ext_state, regs: *mut pt_regs) { if __riscv_v_vstate_check((*regs).status, SR_VS_DIRTY) { __riscv_v_vstate_save(vstate, (*vstate).datap); __riscv_v_vstate_clean(regs); } }
#[cfg(CONFIG_RISCV_ISA_V)]
#[inline]
pub unsafe fn riscv_v_vstate_restore(vstate: *mut __riscv_v_ext_state, regs: *mut pt_regs) { if riscv_v_vstate_query(regs) { __riscv_v_vstate_restore(vstate, (*vstate).datap); __riscv_v_vstate_clean(regs); } }
#[cfg(CONFIG_RISCV_ISA_V)]
#[inline]
pub unsafe fn riscv_v_vstate_set_restore(task: *mut task_struct, regs: *mut pt_regs) { if riscv_v_vstate_query(regs) { set_tsk_thread_flag(task, TIF_RISCV_V_DEFER_RESTORE); riscv_v_vstate_on(regs); } }

#[cfg(CONFIG_RISCV_ISA_V)]
#[inline]
pub unsafe fn __riscv_v_vstate_or(val: usize, typ: usize) -> usize {
    if has_xtheadvector() { (val & !SR_VS_THEAD) | typ } else { (val & !SR_VS) | typ }
}
#[cfg(CONFIG_RISCV_ISA_V)]
#[inline]
pub unsafe fn __riscv_v_vstate_check(val: usize, typ: usize) -> bool {
    if has_xtheadvector() { (val & SR_VS_THEAD) == typ } else { (val & SR_VS) == typ }
}

#[cfg(CONFIG_RISCV_ISA_V_PREEMPTIVE)]
#[inline] pub unsafe fn riscv_preempt_v_dirty(task: *mut task_struct) -> bool { ((*task).thread.riscv_v_flags & RISCV_PREEMPT_V_DIRTY) != 0 }
#[cfg(CONFIG_RISCV_ISA_V_PREEMPTIVE)]
#[inline] pub unsafe fn riscv_preempt_v_restore(task: *mut task_struct) -> bool { ((*task).thread.riscv_v_flags & RISCV_PREEMPT_V_NEED_RESTORE) != 0 }
#[cfg(CONFIG_RISCV_ISA_V_PREEMPTIVE)]
#[inline] pub unsafe fn riscv_preempt_v_clear_dirty(task: *mut task_struct) { barrier(); (*task).thread.riscv_v_flags &= !RISCV_PREEMPT_V_DIRTY; }
#[cfg(CONFIG_RISCV_ISA_V_PREEMPTIVE)]
#[inline] pub unsafe fn riscv_preempt_v_set_restore(task: *mut task_struct) { barrier(); (*task).thread.riscv_v_flags |= RISCV_PREEMPT_V_NEED_RESTORE; }
#[cfg(CONFIG_RISCV_ISA_V_PREEMPTIVE)]
#[inline] pub unsafe fn riscv_preempt_v_started(task: *mut task_struct) -> bool { ((*task).thread.riscv_v_flags & RISCV_PREEMPT_V) != 0 }
#[cfg(not(CONFIG_RISCV_ISA_V_PREEMPTIVE))] pub unsafe fn riscv_preempt_v_dirty(_: *mut task_struct) -> bool { false }
#[cfg(not(CONFIG_RISCV_ISA_V_PREEMPTIVE))] pub unsafe fn riscv_preempt_v_restore(_: *mut task_struct) -> bool { false }
#[cfg(not(CONFIG_RISCV_ISA_V_PREEMPTIVE))] pub unsafe fn riscv_preempt_v_started(_: *mut task_struct) -> bool { false }
#[cfg(not(CONFIG_RISCV_ISA_V_PREEMPTIVE))] pub unsafe fn riscv_preempt_v_clear_dirty(_: *mut task_struct) {}
#[cfg(not(CONFIG_RISCV_ISA_V_PREEMPTIVE))] pub unsafe fn riscv_preempt_v_set_restore(_: *mut task_struct) {}

#[cfg(CONFIG_RISCV_ISA_V)]
#[inline]
pub unsafe fn __switch_to_vector(prev: *mut task_struct, next: *mut task_struct) {
    if riscv_preempt_v_started(prev) {
        if riscv_v_is_on() { WARN_ON((*prev).thread.riscv_v_flags & RISCV_V_CTX_DEPTH_MASK); riscv_v_disable(); (*prev).thread.riscv_v_flags |= RISCV_PREEMPT_V_IN_SCHEDULE; }
        if riscv_preempt_v_dirty(prev) { __riscv_v_vstate_save(&mut (*prev).thread.kernel_vstate, (*prev).thread.kernel_vstate.datap); riscv_preempt_v_clear_dirty(prev); }
    } else { riscv_v_vstate_save(&mut (*prev).thread.vstate, task_pt_regs(prev)); }
    if riscv_preempt_v_started(next) {
        if ((*next).thread.riscv_v_flags & RISCV_PREEMPT_V_IN_SCHEDULE) != 0 { (*next).thread.riscv_v_flags &= !RISCV_PREEMPT_V_IN_SCHEDULE; riscv_v_enable(); } else { riscv_preempt_v_set_restore(next); }
    } else { riscv_v_vstate_set_restore(next, task_pt_regs(next)); }
}

#[cfg(not(CONFIG_RISCV_ISA_V))]
pub unsafe fn __switch_to_vector(_: *mut task_struct, _: *mut task_struct) {}

#[cfg(not(CONFIG_RISCV_ISA_V))]
pub unsafe fn riscv_v_setup_vsize() -> ::core::ffi::c_int { -EOPNOTSUPP }
#[cfg(not(CONFIG_RISCV_ISA_V))] pub unsafe fn has_vector() -> bool { false }
#[cfg(not(CONFIG_RISCV_ISA_V))] pub unsafe fn insn_is_vector(_: u32) -> bool { false }
#[cfg(not(CONFIG_RISCV_ISA_V))] pub unsafe fn has_xtheadvector_no_alternatives() -> bool { false }
#[cfg(not(CONFIG_RISCV_ISA_V))] pub unsafe fn has_xtheadvector() -> bool { false }
#[cfg(not(CONFIG_RISCV_ISA_V))] pub unsafe fn riscv_v_first_use_handler(_: *mut pt_regs) -> bool { false }
#[cfg(not(CONFIG_RISCV_ISA_V))] pub unsafe fn riscv_v_vstate_query(_: *mut pt_regs) -> bool { false }
#[cfg(not(CONFIG_RISCV_ISA_V))] pub unsafe fn riscv_v_vstate_ctrl_user_allowed() -> bool { false }

#[inline]
pub unsafe fn riscv_vector_vlen() -> ::core::ffi::c_int { (riscv_v_vsize / 32 * 8) as _ }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
