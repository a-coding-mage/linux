/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from ioapic.h. External kernel types and symbols are supplied by dependencies. */

#[cfg(CONFIG_KVM_IOAPIC)]
pub const IOAPIC_NUM_PINS: usize = KVM_IOAPIC_NUM_PINS;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const IOAPIC_VERSION_ID: u32 = 0x11;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const IOAPIC_EDGE_TRIG: u32 = 0;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const IOAPIC_LEVEL_TRIG: u32 = 1;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const IOAPIC_DEFAULT_BASE_ADDRESS: u32 = 0xfec00000;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const IOAPIC_MEM_LENGTH: u32 = 0x100;

#[cfg(CONFIG_KVM_IOAPIC)]
pub const IOAPIC_REG_SELECT: u32 = 0x00;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const IOAPIC_REG_WINDOW: u32 = 0x10;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const IOAPIC_REG_APIC_ID: u32 = 0x00;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const IOAPIC_REG_VERSION: u32 = 0x01;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const IOAPIC_REG_ARB_ID: u32 = 0x02;

#[cfg(CONFIG_KVM_IOAPIC)]
pub const IOAPIC_FIXED: u32 = 0x0;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const IOAPIC_LOWEST_PRIORITY: u32 = 0x1;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const IOAPIC_PMI: u32 = 0x2;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const IOAPIC_NMI: u32 = 0x4;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const IOAPIC_INIT: u32 = 0x5;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const IOAPIC_EXTINT: u32 = 0x7;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const RTC_GSI: u32 = 8;

#[cfg(CONFIG_KVM_IOAPIC)]
#[repr(C)]
pub struct rtc_status {
    pub pending_eoi: ::core::ffi::c_int,
    pub map: [::core::ffi::c_ulong; (KVM_MAX_VCPU_IDS + (usize::BITS as usize) - 1) / (usize::BITS as usize)],
    pub vectors: [u8; KVM_MAX_VCPU_IDS],
}

#[cfg(CONFIG_KVM_IOAPIC)]
#[repr(C)]
pub struct kvm_ioapic_redirect_entry_fields {
    pub vector: u8,
    /* C bit-fields delivery_mode..reserve, packed into this byte. */
    pub flags: u8,
    pub reserved: [u8; 4],
    pub dest_id: u8,
}

#[cfg(CONFIG_KVM_IOAPIC)]
#[repr(C)]
pub union kvm_ioapic_redirect_entry {
    pub bits: u64,
    pub fields: kvm_ioapic_redirect_entry_fields,
}

#[cfg(CONFIG_KVM_IOAPIC)]
#[repr(C)]
pub struct kvm_ioapic {
    pub base_address: u64,
    pub ioregsel: u32,
    pub id: u32,
    pub irr: u32,
    pub pad: u32,
    pub redirtbl: [kvm_ioapic_redirect_entry; IOAPIC_NUM_PINS],
    pub irq_states: [::core::ffi::c_ulong; IOAPIC_NUM_PINS],
    pub dev: kvm_io_device,
    pub kvm: *mut kvm,
    pub lock: spinlock_t,
    pub rtc_status: rtc_status,
    pub eoi_inject: delayed_work,
    pub irq_eoi: [u32; IOAPIC_NUM_PINS],
    pub irr_delivered: u32,
    pub mask_notifier_list: hlist_head,
}

#[cfg(CONFIG_KVM_IOAPIC)]
#[repr(C)]
pub struct kvm_irq_mask_notifier {
    pub func: Option<unsafe extern "C" fn(*mut kvm_irq_mask_notifier, bool)>,
    pub irq: ::core::ffi::c_int,
    pub link: hlist_node,
}

extern "C" {
    pub fn kvm_register_irq_mask_notifier(kvm: *mut kvm, irq: ::core::ffi::c_int, kimn: *mut kvm_irq_mask_notifier);
    pub fn kvm_unregister_irq_mask_notifier(kvm: *mut kvm, irq: ::core::ffi::c_int, kimn: *mut kvm_irq_mask_notifier);
    pub fn kvm_fire_mask_notifiers(kvm: *mut kvm, unsigned: u32, pin: u32, mask: bool);
    pub fn kvm_rtc_eoi_tracking_restore_one(vcpu: *mut kvm_vcpu);
    pub fn kvm_ioapic_update_eoi(vcpu: *mut kvm_vcpu, vector: ::core::ffi::c_int, trigger_mode: ::core::ffi::c_int);
    pub fn kvm_ioapic_init(kvm: *mut kvm) -> ::core::ffi::c_int;
    pub fn kvm_ioapic_destroy(kvm: *mut kvm);
    pub fn kvm_ioapic_set_irq(e: *mut kvm_kernel_irq_routing_entry, kvm: *mut kvm, irq_source_id: ::core::ffi::c_int, level: ::core::ffi::c_int, line_status: bool) -> ::core::ffi::c_int;
    pub fn kvm_get_ioapic(kvm: *mut kvm, state: *mut kvm_ioapic_state);
    pub fn kvm_set_ioapic(kvm: *mut kvm, state: *mut kvm_ioapic_state);
    pub fn kvm_ioapic_scan_entry(vcpu: *mut kvm_vcpu, ioapic_handled_vectors: *mut ulong);
    pub fn kvm_scan_ioapic_routes(vcpu: *mut kvm_vcpu, ioapic_handled_vectors: *mut ulong);
    pub fn kvm_scan_ioapic_irq(vcpu: *mut kvm_vcpu, dest_id: u32, dest_mode: u16, vector: u8, ioapic_handled_vectors: *mut ::core::ffi::c_ulong);
}

#[cfg(CONFIG_KVM_IOAPIC)]
pub unsafe fn __kvm_irq_line_state(irq_state: *mut ::core::ffi::c_ulong, irq_source_id: ::core::ffi::c_int, level: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if level != 0 { __set_bit(irq_source_id, irq_state); } else { __clear_bit(irq_source_id, irq_state); }
    (*irq_state != 0) as ::core::ffi::c_int
}

pub unsafe fn ioapic_in_kernel(kvm: *mut kvm) -> bool { irqchip_full(kvm) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
