/* SPDX-License-Identifier: GPL-2.0 */
#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

// External types and constants are provided by the included kernel headers.
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct hrtimer { _private: [u8; 0] }
#[repr(C)] pub struct kvm_io_device { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct gfn_to_hva_cache { _private: [u8; 0] }
#[repr(C)] pub struct kvm { _private: [u8; 0] }
#[repr(C)] pub struct kvm_vcpu { _private: [u8; 0] }
#[repr(C)] pub struct kvm_lapic_irq { _private: [u8; 0] }
#[repr(C)] pub struct kvm_lapic_state { _private: [u8; 0] }
pub type gpa_t = u64;
pub type ktime_t = i64;

extern "C" {
    pub fn kvm_lapic_set_irr(vec: i32, apic: *mut kvm_lapic);
    pub fn kvm_lapic_get_reg(apic: *mut kvm_lapic, reg_off: i32) -> u32;
    pub fn lapic_in_kernel(vcpu: *mut kvm_vcpu) -> bool;
    pub fn kvm_apic_hw_enabled(apic: *mut kvm_lapic) -> bool;
    pub fn kvm_apic_sw_enabled(apic: *mut kvm_lapic) -> bool;
    pub fn kvm_apic_present(vcpu: *mut kvm_vcpu) -> bool;
    pub fn kvm_lapic_enabled(vcpu: *mut kvm_vcpu) -> i32;
    pub fn apic_x2apic_mode(apic: *mut kvm_lapic) -> i32;
    pub fn kvm_vcpu_apicv_active(vcpu: *mut kvm_vcpu) -> bool;
    pub fn kvm_apic_has_pending_init_or_sipi(vcpu: *mut kvm_vcpu) -> bool;
    pub fn kvm_apic_init_sipi_allowed(vcpu: *mut kvm_vcpu) -> bool;
    pub fn kvm_lapic_latched_init(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_lapic_irq_dest_mode(dest_mode_logical: bool) -> u16;
    pub fn kvm_apic_mode(apic_base: u64) -> lapic_mode;
    pub fn kvm_get_apic_mode(vcpu: *mut kvm_vcpu) -> lapic_mode;
    pub fn kvm_xapic_id(apic: *mut kvm_lapic) -> u8;
    pub fn kvm_apic_pending_eoi(vcpu: *mut kvm_vcpu, vector: i32) -> bool;
    pub fn kvm_lapic_suppress_eoi_broadcast(apic: *mut kvm_lapic) -> bool;
    pub fn kvm_wait_lapic_expire(vcpu: *mut kvm_vcpu);
    pub fn kvm_bitmap_or_dest_vcpus(kvm: *mut kvm, irq: *mut kvm_lapic_irq, vcpu_bitmap: *mut usize);
    pub fn kvm_intr_is_single_vcpu(kvm: *mut kvm, irq: *mut kvm_lapic_irq, dest_vcpu: *mut *mut kvm_vcpu) -> bool;
    pub fn kvm_lapic_switch_to_sw_timer(vcpu: *mut kvm_vcpu);
    pub fn kvm_lapic_switch_to_hv_timer(vcpu: *mut kvm_vcpu);
    pub fn kvm_lapic_expired_hv_timer(vcpu: *mut kvm_vcpu);
    pub fn kvm_lapic_hv_timer_in_use(vcpu: *mut kvm_vcpu) -> bool;
    pub fn kvm_lapic_restart_hv_timer(vcpu: *mut kvm_vcpu);
}

pub const KVM_APIC_INIT: u32 = 0;
pub const KVM_APIC_SIPI: u32 = 1;
pub const APIC_SHORT_MASK: u32 = 0xc0000;
pub const APIC_DEST_NOSHORT: u32 = 0x0;
pub const APIC_DEST_MASK: u32 = 0x800;
pub const APIC_BUS_CYCLE_NS_DEFAULT: u32 = 1;
pub const APIC_BROADCAST: u32 = 0xFF;
pub const X2APIC_BROADCAST: u32 = 0xFFFFFFFF;

pub const fn X2APIC_MSR(r: u32) -> u32 { APIC_BASE_MSR + (r >> 4) }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum lapic_mode { LAPIC_MODE_DISABLED = 0, LAPIC_MODE_INVALID = X2APIC_ENABLE, LAPIC_MODE_XAPIC = MSR_IA32_APICBASE_ENABLE, LAPIC_MODE_X2APIC = MSR_IA32_APICBASE_ENABLE | X2APIC_ENABLE }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum kvm_apic_logical_mode { KVM_APIC_MODE_SW_DISABLED, KVM_APIC_MODE_XAPIC_CLUSTER, KVM_APIC_MODE_XAPIC_FLAT, KVM_APIC_MODE_X2APIC, KVM_APIC_MODE_MAP_DISABLED }

#[repr(C)]
pub union kvm_apic_map_union {
    pub xapic_flat_map: [*mut kvm_lapic; 8],
    pub xapic_cluster_map: [[*mut kvm_lapic; 4]; 16],
}

#[repr(C)]
pub struct kvm_apic_map {
    pub rcu: rcu_head,
    pub logical_mode: kvm_apic_logical_mode,
    pub max_apic_id: u32,
    pub maps: kvm_apic_map_union,
    pub phys_map: [*mut kvm_lapic; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum lapic_lvt_entry { LVT_TIMER, LVT_THERMAL_MONITOR, LVT_PERFORMANCE_COUNTER, LVT_LINT0, LVT_LINT1, LVT_ERROR, LVT_CMCI, KVM_APIC_MAX_NR_LVT_ENTRIES }

pub const fn APIC_LVTx(x: u32) -> u32 { if x == LVT_CMCI as u32 { APIC_LVTCMCI } else { APIC_LVTT + 0x10 * x } }

#[repr(C)]
pub struct kvm_timer {
    pub timer: hrtimer,
    pub period: i64,
    pub target_expiration: ktime_t,
    pub timer_mode: u32,
    pub timer_mode_mask: u32,
    pub tscdeadline: u64,
    pub expired_tscdeadline: u64,
    pub timer_advance_ns: u32,
    pub pending: atomic_t,
    pub hv_timer_in_use: bool,
}

#[repr(C)]
pub struct kvm_lapic {
    pub base_address: usize,
    pub dev: kvm_io_device,
    pub lapic_timer: kvm_timer,
    pub divide_count: u32,
    pub vcpu: *mut kvm_vcpu,
    pub apicv_active: bool,
    pub sw_enabled: bool,
    pub irr_pending: bool,
    pub lvt0_in_nmi_mode: bool,
    pub guest_apic_protected: bool,
    pub isr_count: i16,
    pub highest_isr_cache: i32,
    pub regs: *mut c_void,
    pub vapic_addr: gpa_t,
    pub vapic_cache: gfn_to_hva_cache,
    pub pending_events: usize,
    pub sipi_vector: u32,
    pub nr_lvt_entries: i32,
}

#[repr(C)] pub struct rtc_status { _private: [u8; 0] }

extern "C" {
    pub fn kvm_create_lapic(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_free_lapic(vcpu: *mut kvm_vcpu);
    pub fn kvm_apic_has_interrupt(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_apic_ack_interrupt(vcpu: *mut kvm_vcpu, vector: i32);
    pub fn kvm_apic_accept_pic_intr(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_apic_accept_events(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_lapic_reset(vcpu: *mut kvm_vcpu, init_event: bool);
    pub fn kvm_lapic_get_cr8(vcpu: *mut kvm_vcpu) -> u64;
    pub fn kvm_lapic_set_tpr(vcpu: *mut kvm_vcpu, cr8: usize);
    pub fn kvm_lapic_update_cr8_intercept(vcpu: *mut kvm_vcpu);
    pub fn kvm_lapic_set_eoi(vcpu: *mut kvm_vcpu);
    pub fn kvm_apic_set_version(vcpu: *mut kvm_vcpu);
    pub fn kvm_apic_after_set_mcg_cap(vcpu: *mut kvm_vcpu);
    pub fn kvm_apic_match_dest(vcpu: *mut kvm_vcpu, source: *mut kvm_lapic, shorthand: i32, dest: u32, dest_mode: i32) -> bool;
    pub fn kvm_apic_clear_irr(vcpu: *mut kvm_vcpu, vec: i32);
    pub fn __kvm_apic_update_irr(pir: *mut usize, regs: *mut c_void, max_irr: *mut i32) -> bool;
    pub fn kvm_apic_update_irr(vcpu: *mut kvm_vcpu, pir: *mut usize, max_irr: *mut i32) -> bool;
    pub fn kvm_apic_update_ppr(vcpu: *mut kvm_vcpu);
    pub fn kvm_apic_set_irq(vcpu: *mut kvm_vcpu, irq: *mut kvm_lapic_irq, rtc_status: *mut rtc_status) -> i32;
    pub fn kvm_apic_local_deliver(apic: *mut kvm_lapic, lvt_type: i32) -> i32;
    pub fn kvm_apic_update_apicv(vcpu: *mut kvm_vcpu);
    pub fn kvm_alloc_apic_access_page(kvm: *mut kvm) -> i32;
    pub fn kvm_inhibit_apic_access_page(vcpu: *mut kvm_vcpu);
    pub fn kvm_irq_delivery_to_apic_fast(kvm: *mut kvm, src: *mut kvm_lapic, irq: *mut kvm_lapic_irq, r: *mut i32) -> bool;
    pub fn __kvm_irq_delivery_to_apic(kvm: *mut kvm, src: *mut kvm_lapic, irq: *mut kvm_lapic_irq, rtc_status: *mut rtc_status) -> i32;
    pub fn kvm_apic_send_ipi(apic: *mut kvm_lapic, icr_low: u32, icr_high: u32);
    pub fn kvm_pv_send_ipi(kvm: *mut kvm, ipi_bitmap_low: usize, ipi_bitmap_high: usize, min: u32, icr: usize, op_64_bit: i32) -> i32;
    pub fn kvm_apic_set_base(vcpu: *mut kvm_vcpu, value: u64, host_initiated: bool) -> i32;
    pub fn kvm_apic_get_state(vcpu: *mut kvm_vcpu, s: *mut kvm_lapic_state) -> i32;
    pub fn kvm_apic_set_state(vcpu: *mut kvm_vcpu, s: *mut kvm_lapic_state) -> i32;
    pub fn kvm_lapic_find_highest_irr(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_get_lapic_tscdeadline_msr(vcpu: *mut kvm_vcpu) -> u64;
    pub fn kvm_set_lapic_tscdeadline_msr(vcpu: *mut kvm_vcpu, data: u64);
    pub fn kvm_apic_write_nodecode(vcpu: *mut kvm_vcpu, offset: u32);
    pub fn kvm_apic_set_eoi_accelerated(vcpu: *mut kvm_vcpu, vector: i32);
    pub fn kvm_lapic_set_vapic_addr(vcpu: *mut kvm_vcpu, vapic_addr: gpa_t) -> i32;
    pub fn kvm_lapic_sync_from_vapic(vcpu: *mut kvm_vcpu);
    pub fn kvm_lapic_sync_to_vapic(vcpu: *mut kvm_vcpu);
    pub fn kvm_x2apic_icr_write_fast(apic: *mut kvm_lapic, data: u64) -> i32;
    pub fn kvm_x2apic_msr_write(vcpu: *mut kvm_vcpu, msr: u32, data: u64) -> i32;
    pub fn kvm_x2apic_msr_read(vcpu: *mut kvm_vcpu, msr: u32, data: *mut u64) -> i32;
    pub fn kvm_hv_vapic_msr_write(vcpu: *mut kvm_vcpu, msr: u32, data: u64) -> i32;
    pub fn kvm_hv_vapic_msr_read(vcpu: *mut kvm_vcpu, msr: u32, data: *mut u64) -> i32;
    pub fn kvm_lapic_set_pv_eoi(vcpu: *mut kvm_vcpu, data: u64, len: usize) -> i32;
    pub fn kvm_lapic_exit();
    pub fn kvm_x2apic_disable_read_intercept_reg_mask(vcpu: *mut kvm_vcpu) -> u64;
}

pub unsafe fn kvm_irq_delivery_to_apic(kvm: *mut kvm, src: *mut kvm_lapic, irq: *mut kvm_lapic_irq) -> i32 { __kvm_irq_delivery_to_apic(kvm, src, irq, core::ptr::null_mut()) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
