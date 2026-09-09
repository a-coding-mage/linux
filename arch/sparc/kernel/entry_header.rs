/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from entry.h.  Types supplied by included kernel headers remain
 * external dependencies of this translation. */

extern "C" {
    pub fn handler_irq(irq: i32, regs: *mut pt_regs);
}

#[cfg(CONFIG_SPARC32)]
extern "C" {
    pub fn do_hw_interrupt(regs: *mut pt_regs, type_: usize);
    pub fn do_illegal_instruction(regs: *mut pt_regs, pc: usize, npc: usize, psr: usize);
    pub fn do_priv_instruction(regs: *mut pt_regs, pc: usize, npc: usize, psr: usize);
    pub fn do_memaccess_unaligned(regs: *mut pt_regs, pc: usize, npc: usize, psr: usize);
    pub fn do_fpd_trap(regs: *mut pt_regs, pc: usize, npc: usize, psr: usize);
    pub fn do_fpe_trap(regs: *mut pt_regs, pc: usize, npc: usize, psr: usize);
    pub fn handle_tag_overflow(regs: *mut pt_regs, pc: usize, npc: usize, psr: usize);
    pub fn handle_watchpoint(regs: *mut pt_regs, pc: usize, npc: usize, psr: usize);
    pub fn handle_reg_access(regs: *mut pt_regs, pc: usize, npc: usize, psr: usize);
    pub fn handle_cp_disabled(regs: *mut pt_regs, pc: usize, npc: usize, psr: usize);
    pub fn handle_cp_exception(regs: *mut pt_regs, pc: usize, npc: usize, psr: usize);
    pub fn fpsave(fpregs: *mut usize, fsr: *mut usize, fpqueue: *mut core::ffi::c_void, fpqdepth: *mut usize);
    pub fn fpload(fpregs: *mut usize, fsr: *mut usize);
}

#[cfg(not(CONFIG_SPARC32))]
extern "C" {
    pub fn sun4v_patch_1insn_range(a: *mut sun4v_1insn_patch_entry, b: *mut sun4v_1insn_patch_entry);
    pub fn sun4v_patch_2insn_range(a: *mut sun4v_2insn_patch_entry, b: *mut sun4v_2insn_patch_entry);
    pub fn sun_m7_patch_2insn_range(a: *mut sun4v_2insn_patch_entry, b: *mut sun4v_2insn_patch_entry);
    pub fn sparc_breakpoint(regs: *mut pt_regs);
    pub fn timer_interrupt(irq: i32, regs: *mut pt_regs);
    pub fn do_notify_resume(regs: *mut pt_regs, orig_i0: usize, thread_info_flags: usize);
    pub fn syscall_trace_enter(regs: *mut pt_regs) -> i32;
    pub fn syscall_trace_leave(regs: *mut pt_regs);
    pub fn bad_trap_tl1(regs: *mut pt_regs, lvl: isize);
    pub fn do_fpieee(regs: *mut pt_regs); pub fn do_fpother(regs: *mut pt_regs);
    pub fn do_tof(regs: *mut pt_regs); pub fn do_div0(regs: *mut pt_regs);
    pub fn do_illegal_instruction(regs: *mut pt_regs);
    pub fn mem_address_unaligned(regs: *mut pt_regs, sfar: usize, sfsr: usize);
    pub fn sun4v_do_mna(regs: *mut pt_regs, addr: usize, type_ctx: usize);
    pub fn do_privop(regs: *mut pt_regs); pub fn do_privact(regs: *mut pt_regs);
    pub fn do_cee(regs: *mut pt_regs); pub fn do_div0_tl1(regs: *mut pt_regs);
    pub fn do_fpieee_tl1(regs: *mut pt_regs); pub fn do_fpother_tl1(regs: *mut pt_regs);
    pub fn do_ill_tl1(regs: *mut pt_regs); pub fn do_irq_tl1(regs: *mut pt_regs);
    pub fn do_lddfmna_tl1(regs: *mut pt_regs); pub fn do_stdfmna_tl1(regs: *mut pt_regs);
    pub fn do_paw(regs: *mut pt_regs); pub fn do_paw_tl1(regs: *mut pt_regs);
    pub fn do_vaw(regs: *mut pt_regs); pub fn do_vaw_tl1(regs: *mut pt_regs);
    pub fn do_tof_tl1(regs: *mut pt_regs); pub fn do_getpsr(regs: *mut pt_regs);
    pub fn spitfire_insn_access_exception(regs: *mut pt_regs, sfsr: usize, sfar: usize);
    pub fn spitfire_insn_access_exception_tl1(regs: *mut pt_regs, sfsr: usize, sfar: usize);
    pub fn spitfire_data_access_exception(regs: *mut pt_regs, sfsr: usize, sfar: usize);
    pub fn spitfire_data_access_exception_tl1(regs: *mut pt_regs, sfsr: usize, sfar: usize);
    pub fn spitfire_access_error(regs: *mut pt_regs, status_encoded: usize, afar: usize);
    pub fn cheetah_fecc_handler(regs: *mut pt_regs, afsr: usize, afar: usize);
    pub fn cheetah_cee_handler(regs: *mut pt_regs, afsr: usize, afar: usize);
    pub fn cheetah_deferred_handler(regs: *mut pt_regs, afsr: usize, afar: usize);
    pub fn cheetah_plus_parity_error(type_: i32, regs: *mut pt_regs);
    pub fn sun4v_insn_access_exception(regs: *mut pt_regs, addr: usize, type_ctx: usize);
    pub fn sun4v_insn_access_exception_tl1(regs: *mut pt_regs, addr: usize, type_ctx: usize);
    pub fn sun4v_data_access_exception(regs: *mut pt_regs, addr: usize, type_ctx: usize);
    pub fn sun4v_data_access_exception_tl1(regs: *mut pt_regs, addr: usize, type_ctx: usize);
    pub fn sun4v_resum_error(regs: *mut pt_regs, offset: usize);
    pub fn sun4v_resum_overflow(regs: *mut pt_regs);
    pub fn sun4v_nonresum_error(regs: *mut pt_regs, offset: usize);
    pub fn sun4v_nonresum_overflow(regs: *mut pt_regs);
    pub fn sun4v_mem_corrupt_detect_precise(regs: *mut pt_regs, addr: usize, context: usize);
    pub fn sun4v_itlb_error_report(regs: *mut pt_regs, tl: i32);
    pub fn sun4v_dtlb_error_report(regs: *mut pt_regs, tl: i32);
    pub fn hypervisor_tlbop_error(err: usize, op: usize);
    pub fn hypervisor_tlbop_error_xcall(err: usize, op: usize);
    pub fn init_irqwork_curcpu();
    pub fn sun4v_register_mondo_queues(this_cpu: i32);
}

#[repr(C)]
pub struct popc_3insn_patch_entry { pub addr: u32, pub insns: [u32; 3] }
#[repr(C)]
pub struct popc_6insn_patch_entry { pub addr: u32, pub insns: [u32; 6] }
#[repr(C)]
pub struct pause_patch_entry { pub addr: u32, pub insns: [u32; 3] }

#[repr(C)]
pub struct cheetah_err_info {
    pub afsr: u64, pub afar: u64,
    pub dcache_data: [u64; 4], pub dcache_index: u64, pub dcache_tag: u64,
    pub dcache_utag: u64, pub dcache_stag: u64,
    pub icache_data: [u64; 8], pub icache_index: u64, pub icache_tag: u64,
    pub icache_utag: u64, pub icache_stag: u64, pub icache_upper: u64, pub icache_lower: u64,
    pub ecache_data: [u64; 4], pub ecache_index: u64, pub ecache_tag: u64,
    pub __pad: [u64; 2],
}
pub const CHAFSR_INVALID: u64 = u64::MAX;

#[repr(C)]
pub struct ino_bucket { pub __irq_chain_pa: usize, pub __irq: u32, pub __pad: u32 }

extern "C" {
    pub static mut __popc_3insn_patch: popc_3insn_patch_entry;
    pub static mut __popc_3insn_patch_end: popc_3insn_patch_entry;
    pub static mut __popc_6insn_patch: popc_6insn_patch_entry;
    pub static mut __popc_6insn_patch_end: popc_6insn_patch_entry;
    pub static mut __pause_3insn_patch: pause_patch_entry;
    pub static mut __pause_3insn_patch_end: pause_patch_entry;
    pub static mut dcache_parity_tl1_occurred: u32;
    pub static mut icache_parity_tl1_occurred: u32;
    pub static mut sun4v_err_itlb_vaddr: usize; pub static mut sun4v_err_itlb_ctx: usize;
    pub static mut sun4v_err_itlb_pte: usize; pub static mut sun4v_err_itlb_error: usize;
    pub static mut sun4v_err_dtlb_vaddr: usize; pub static mut sun4v_err_dtlb_ctx: usize;
    pub static mut sun4v_err_dtlb_pte: usize; pub static mut sun4v_err_dtlb_error: usize;
    pub static mut cheetah_error_log: *mut cheetah_err_info;
    pub static mut ivector_table: *mut ino_bucket;
    pub static mut ivector_table_pa: usize;
}

/* C layout required by hand-written assembly; the following external types
 * are declared by the included trap headers. */
extern "C" {
    type pt_regs;
    type sun4v_1insn_patch_entry;
    type sun4v_2insn_patch_entry;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
