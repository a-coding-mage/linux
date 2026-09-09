/* SPDX-License-Identifier: GPL-2.0 */
// Direct Rust translation of x86.h. Kernel-provided types, constants, macros,
// and functions referenced here are supplied by other translation units.

pub const KVM_MAX_MCE_BANKS: u32 = 32;
pub const KVM_DEFAULT_PLE_GAP: u32 = 128;
pub const KVM_VMX_DEFAULT_PLE_WINDOW: u32 = 4096;
pub const KVM_DEFAULT_PLE_WINDOW_GROW: u32 = 2;
pub const KVM_DEFAULT_PLE_WINDOW_SHRINK: u32 = 0;
pub const KVM_VMX_DEFAULT_PLE_WINDOW_MAX: u32 = u32::MAX;
pub const KVM_SVM_DEFAULT_PLE_WINDOW_MAX: u32 = u16::MAX as u32;
pub const KVM_SVM_DEFAULT_PLE_WINDOW: u32 = 3000;

extern "C" {
    pub fn kvm_x86_vendor_init(ops: *mut kvm_x86_init_ops) -> i32;
    pub fn kvm_x86_vendor_exit();
    pub fn kvm_spurious_fault();
    pub fn kvm_service_local_tlb_flush_requests(vcpu: *mut kvm_vcpu);
    pub fn kvm_check_nested_events(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_vcpu_reset(vcpu: *mut kvm_vcpu, init_event: bool);
    pub fn kvm_inject_realmode_interrupt(vcpu: *mut kvm_vcpu, irq: i32, inc_eip: i32);
    pub fn get_kvmclock_ns(kvm: *mut kvm) -> u64;
    pub fn kvm_get_wall_clock_epoch(kvm: *mut kvm) -> u64;
    pub fn kvm_get_monotonic_and_clockread(kernel_ns: *mut i64, tsc_timestamp: *mut u64) -> bool;
    pub fn kvm_guest_time_update(v: *mut kvm_vcpu) -> i32;
    pub fn kvm_synchronize_tsc(vcpu: *mut kvm_vcpu, user_value: *mut u64);
    pub fn kvm_scale_tsc(tsc: u64, ratio: u64) -> u64;
    pub fn kvm_read_l1_tsc(vcpu: *mut kvm_vcpu, host_tsc: u64) -> u64;
    pub fn kvm_calc_nested_tsc_offset(l1_offset: u64, l2_offset: u64, l2_multiplier: u64) -> u64;
    pub fn kvm_calc_nested_tsc_multiplier(l1_multiplier: u64, l2_multiplier: u64) -> u64;
    pub fn kvm_compute_l1_tsc_offset(vcpu: *mut kvm_vcpu, target_tsc: u64) -> u64;
    pub fn kvm_vcpu_write_tsc_offset(vcpu: *mut kvm_vcpu, l1_offset: u64);
    pub fn kvm_read_guest_virt(vcpu: *mut kvm_vcpu, addr: gva_t, val: *mut core::ffi::c_void, bytes: u32, exception: *mut x86_exception) -> i32;
    pub fn kvm_write_guest_virt_system(vcpu: *mut kvm_vcpu, addr: gva_t, val: *mut core::ffi::c_void, bytes: u32, exception: *mut x86_exception) -> i32;
    pub fn handle_ud(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_deliver_exception_payload(vcpu: *mut kvm_vcpu, ex: *mut kvm_queued_exception);
    pub fn kvm_handle_exception_payload_quirk(vcpu: *mut kvm_vcpu);
    pub fn kvm_fixup_and_inject_pf_error(vcpu: *mut kvm_vcpu, gva: gva_t, error_code: u16);
    pub fn x86_decode_emulated_instruction(vcpu: *mut kvm_vcpu, emulation_type: i32, insn: *mut core::ffi::c_void, insn_len: i32) -> i32;
    pub fn x86_emulate_instruction(vcpu: *mut kvm_vcpu, cr2_or_gpa: gpa_t, emulation_type: i32, insn: *mut core::ffi::c_void, insn_len: i32) -> i32;
    pub fn kvm_emulate_instruction(vcpu: *mut kvm_vcpu, emulation_type: i32) -> i32;
    pub fn kvm_emulate_instruction_from_buffer(vcpu: *mut kvm_vcpu, insn: *mut core::ffi::c_void, insn_len: i32) -> i32;
    pub fn __kvm_prepare_emulation_failure_exit(vcpu: *mut kvm_vcpu, data: *mut u64, ndata: u8);
    pub fn kvm_prepare_emulation_failure_exit(vcpu: *mut kvm_vcpu);
    pub fn kvm_prepare_event_vectoring_exit(vcpu: *mut kvm_vcpu, gpa: gpa_t);
    pub fn kvm_prepare_unexpected_reason_exit(vcpu: *mut kvm_vcpu, exit_reason: u64);
    pub fn kvm_emulate_as_nop(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_emulate_invd(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_emulate_mwait(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_handle_invalid_op(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_emulate_monitor(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_fast_pio(vcpu: *mut kvm_vcpu, size: i32, port: u16, input: i32) -> i32;
    pub fn kvm_emulate_cpuid(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_emulate_halt(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_emulate_halt_noskip(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_emulate_ap_reset_hold(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_emulate_wbinvd(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_vcpu_deliver_sipi_vector(vcpu: *mut kvm_vcpu, vector: u8);
    pub fn kvm_task_switch(vcpu: *mut kvm_vcpu, tss_selector: u16, idt_index: i32, reason: i32, has_error_code: bool, error_code: u32) -> i32;
    pub fn __kvm_set_xcr(vcpu: *mut kvm_vcpu, index: u32, xcr: u64) -> i32;
    pub fn kvm_emulate_xsetbv(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_emulate_rdpmc(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_skip_emulated_instruction(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_complete_insn_gp(vcpu: *mut kvm_vcpu, err: i32) -> i32;
    pub fn kvm_queue_exception(vcpu: *mut kvm_vcpu, nr: u32);
    pub fn kvm_queue_exception_e(vcpu: *mut kvm_vcpu, nr: u32, error_code: u32);
    pub fn kvm_queue_exception_p(vcpu: *mut kvm_vcpu, nr: u32, payload: usize);
    pub fn kvm_requeue_exception(vcpu: *mut kvm_vcpu, nr: u32, has_error_code: bool, error_code: u32);
    pub fn kvm_inject_page_fault(vcpu: *mut kvm_vcpu, fault: *mut x86_exception, from_hardware: bool);
    pub fn __kvm_inject_emulated_page_fault(vcpu: *mut kvm_vcpu, fault: *mut x86_exception, from_hardware: bool);
    pub fn kvm_require_dr(vcpu: *mut kvm_vcpu, dr: i32) -> bool;
    pub fn kvm_inject_nmi(vcpu: *mut kvm_vcpu);
    pub fn kvm_get_nr_pending_nmis(vcpu: *mut kvm_vcpu) -> i32;
    pub fn __x86_set_memory_region(kvm: *mut kvm, id: i32, gpa: gpa_t, size: u32) -> *mut core::ffi::c_void;
    pub fn memslot_rmap_alloc(slot: *mut kvm_memory_slot, npages: usize) -> bool;
    pub fn kvm_vcpu_is_reset_bsp(vcpu: *mut kvm_vcpu) -> bool;
    pub fn kvm_vcpu_is_bsp(vcpu: *mut kvm_vcpu) -> bool;
}

pub const EMULTYPE_NO_DECODE: i32 = 1 << 0;
pub const EMULTYPE_TRAP_UD: i32 = 1 << 1;
pub const EMULTYPE_SKIP: i32 = 1 << 2;
pub const EMULTYPE_ALLOW_RETRY_PF: i32 = 1 << 3;
pub const EMULTYPE_TRAP_UD_FORCED: i32 = 1 << 4;
pub const EMULTYPE_VMWARE_GP: i32 = 1 << 5;
pub const EMULTYPE_PF: i32 = 1 << 6;
pub const EMULTYPE_COMPLETE_USER_EXIT: i32 = 1 << 7;
pub const EMULTYPE_WRITE_PF_TO_SP: i32 = 1 << 8;
pub const EMULTYPE_SKIP_SOFT_INT: i32 = 1 << 9;

#[inline] pub const fn emultype_set_soft_int_vector(v: u32) -> u32 { (v & 0xff) << 16 }
#[inline] pub const fn emultype_get_soft_int_vector(e: u32) -> u32 { (e >> 16) & 0xff }

#[inline]
pub fn __grow_ple_window(val: u32, base: u32, modifier: u32, max: u32) -> u32 {
    if modifier < 1 { return base; }
    let ret = if modifier < base { (val as u64).wrapping_mul(modifier as u64) } else { (val as u64).wrapping_add(modifier as u64) };
    core::cmp::min(ret, max as u64) as u32
}
#[inline]
pub fn __shrink_ple_window(mut val: u32, base: u32, modifier: u32, min: u32) -> u32 {
    if modifier < 1 { return base; }
    if modifier < base { val /= modifier; } else { val = val.wrapping_sub(modifier); }
    core::cmp::max(val, min)
}

#[repr(C)] pub struct kvm_x86_init_ops { _private: [u8; 0] }
#[repr(C)] pub struct kvm_vcpu { _private: [u8; 0] }
#[repr(C)] pub struct kvm { _private: [u8; 0] }
#[repr(C)] pub struct kvm_memory_slot { _private: [u8; 0] }
#[repr(C)] pub struct x86_exception { _private: [u8; 0] }
#[repr(C)] pub struct kvm_queued_exception { _private: [u8; 0] }

pub type gva_t = u64;
pub type gpa_t = u64;

#[repr(i32)] pub enum kvm_task_switch_reason { TASK_SWITCH_CALL = 0, TASK_SWITCH_IRET = 1, TASK_SWITCH_JMP = 2, TASK_SWITCH_GATE = 3 }
#[repr(i32)] pub enum kvm_apicv_inhibit {
    APICV_INHIBIT_REASON_DISABLED, APICV_INHIBIT_REASON_HYPERV, APICV_INHIBIT_REASON_ABSENT,
    APICV_INHIBIT_REASON_BLOCKIRQ, APICV_INHIBIT_REASON_PHYSICAL_ID_ALIASED,
    APICV_INHIBIT_REASON_APIC_ID_MODIFIED, APICV_INHIBIT_REASON_APIC_BASE_MODIFIED,
    APICV_INHIBIT_REASON_NESTED, APICV_INHIBIT_REASON_IRQWIN, APICV_INHIBIT_REASON_PIT_REINJ,
    APICV_INHIBIT_REASON_SEV, APICV_INHIBIT_REASON_LOGICAL_ID_ALIASED,
    APICV_INHIBIT_REASON_PHYSICAL_ID_TOO_BIG, NR_APICV_INHIBIT_REASONS,
}

pub const MMIO_GVA_ANY: gva_t = !0;
pub const KVM_HANDLING_NMI: u8 = 2;

extern "C" {
    pub fn kvm_emulate_hypercall(vcpu: *mut kvm_vcpu) -> i32;
    pub fn ____kvm_emulate_hypercall(vcpu: *mut kvm_vcpu, cpl: i32, complete_hypercall: Option<unsafe extern "C" fn(*mut kvm_vcpu) -> i32>) -> i32;
}

extern "C" {
    pub fn kvm_apicv_activated(kvm: *mut kvm) -> bool;
    pub fn kvm_vcpu_apicv_activated(vcpu: *mut kvm_vcpu) -> bool;
    pub fn __kvm_vcpu_update_apicv(vcpu: *mut kvm_vcpu);
    pub fn __kvm_set_or_clear_apicv_inhibit(kvm: *mut kvm, reason: kvm_apicv_inhibit, set: bool);
    pub fn kvm_set_or_clear_apicv_inhibit(kvm: *mut kvm, reason: kvm_apicv_inhibit, set: bool);
    pub fn kvm_inc_or_dec_irq_window_inhibit(kvm: *mut kvm, inc: bool);
    pub fn kvm_make_scan_ioapic_request(kvm: *mut kvm);
    pub fn kvm_make_scan_ioapic_request_mask(kvm: *mut kvm, vcpu_bitmap: *mut usize);
    pub fn kvm_setup_xss_caps();
    pub fn kvm_find_async_pf_gfn(vcpu: *mut kvm_vcpu, gfn: u64) -> bool;
    pub fn kvm_handle_memory_failure(vcpu: *mut kvm_vcpu, r: i32, e: *mut x86_exception) -> i32;
    pub fn kvm_invalidate_pcid(vcpu: *mut kvm_vcpu, pcid: usize);
    pub fn kvm_handle_invpcid(vcpu: *mut kvm_vcpu, kind: usize, gva: gva_t) -> i32;
    pub fn kvm_sev_es_mmio(vcpu: *mut kvm_vcpu, is_write: bool, gpa: gpa_t, bytes: u32, data: *mut core::ffi::c_void) -> i32;
    pub fn kvm_sev_es_string_io(vcpu: *mut kvm_vcpu, size: u32, port: u32, data: *mut core::ffi::c_void, count: u32, input: i32) -> i32;
}

// External globals declared by the header.
extern "C" {
    pub static mut min_timer_period_us: u32;
    pub static mut enable_vmware_backdoor: bool;
    pub static mut pi_inject_timer: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
