/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of the Linux KVM tracepoint header.
// The C TRACE_EVENT/DECLARE_EVENT_CLASS/DEFINE_EVENT macros are supplied by
// the tracepoint infrastructure; their generated registration machinery is
// represented here by payload layouts and external event declarations.

pub const KVM_TRACE_MMIO_READ_UNSATISFIED: u32 = 0;
pub const KVM_TRACE_MMIO_READ: u32 = 1;
pub const KVM_TRACE_MMIO_WRITE: u32 = 2;

#[repr(C)]
pub struct KvmUserspaceExitEntry {
    pub reason: u32,
    pub errno: i32,
}

#[repr(C)]
pub struct KvmVcpuWakeupEntry {
    pub ns: u64,
    pub waited: bool,
    pub valid: bool,
}

#[cfg(feature = "have_kvm_irqchip")]
#[repr(C)]
pub struct KvmSetIrqEntry {
    pub gsi: u32,
    pub level: i32,
    pub irq_source_id: i32,
}

#[cfg(feature = "have_kvm_irqchip")]
#[repr(C)]
pub struct KvmAckIrqEntry {
    pub irqchip: u32,
    pub pin: u32,
}

#[repr(C)]
pub struct KvmMmioEntry {
    pub type_: u32,
    pub len: u32,
    pub gpa: u64,
    pub val: u64,
}

#[repr(C)]
pub struct KvmFpuEntry {
    pub load: u32,
}

#[cfg(feature = "kvm_async_pf")]
#[repr(C)]
pub struct KvmAsyncGetPageEntry {
    pub gva: u64,
    pub gfn: u64,
}

#[cfg(feature = "kvm_async_pf")]
#[repr(C)]
pub struct KvmAsyncPfNopresentReadyEntry {
    pub token: u64,
    pub gva: u64,
}

#[cfg(feature = "kvm_async_pf")]
#[repr(C)]
pub struct KvmAsyncPfCompletedEntry {
    pub address: usize,
    pub gva: u64,
}

#[repr(C)]
pub struct KvmHaltPollNsEntry {
    pub grow: bool,
    pub vcpu_id: u32,
    pub new: u32,
    pub old: u32,
}

#[repr(C)]
pub struct KvmDirtyRingPushEntry {
    pub index: i32,
    pub dirty_index: u32,
    pub reset_index: u32,
    pub slot: u32,
    pub offset: u64,
}

#[repr(C)]
pub struct KvmDirtyRingResetEntry {
    pub index: i32,
    pub dirty_index: u32,
    pub reset_index: u32,
}

#[repr(C)]
pub struct KvmDirtyRingExitEntry {
    pub vcpu_id: i32,
}

#[cfg(feature = "kvm_generic_memory_attributes")]
#[repr(C)]
pub struct KvmVmSetMemAttributesEntry {
    pub start: u64,
    pub end: u64,
    pub attr: usize,
}

#[repr(C)]
pub struct KvmUnmapHvaRangeEntry {
    pub start: usize,
    pub end: usize,
}

#[repr(C)]
pub struct KvmAgeHvaEntry {
    pub start: usize,
    pub end: usize,
}

#[repr(C)]
pub struct KvmTestAgeHvaEntry {
    pub hva: usize,
}

extern "C" {
    pub fn kvm_userspace_exit(reason: u32, errno: i32);
    pub fn kvm_vcpu_wakeup(ns: u64, waited: bool, valid: bool);

    #[cfg(feature = "have_kvm_irqchip")]
    pub fn kvm_set_irq(gsi: u32, level: i32, irq_source_id: i32);

    #[cfg(feature = "have_kvm_irqchip")]
    pub fn kvm_ack_irq(irqchip: u32, pin: u32);

    pub fn kvm_mmio(type_: i32, len: i32, gpa: u64, val: *mut core::ffi::c_void);
    pub fn kvm_fpu(load: i32);

    #[cfg(feature = "kvm_async_pf")]
    pub fn kvm_try_async_get_page(gva: u64, gfn: u64);
    #[cfg(feature = "kvm_async_pf")]
    pub fn kvm_async_pf_repeated_fault(gva: u64, gfn: u64);
    #[cfg(feature = "kvm_async_pf")]
    pub fn kvm_async_pf_not_present(token: u64, gva: u64);
    #[cfg(feature = "kvm_async_pf")]
    pub fn kvm_async_pf_ready(token: u64, gva: u64);
    #[cfg(feature = "kvm_async_pf")]
    pub fn kvm_async_pf_completed(address: usize, gva: u64);

    pub fn kvm_halt_poll_ns(grow: bool, vcpu_id: u32, new: u32, old: u32);
    pub fn kvm_dirty_ring_push(ring: *mut KvmDirtyRing, slot: u32, offset: u64);
    pub fn kvm_dirty_ring_reset(ring: *mut KvmDirtyRing);
    pub fn kvm_dirty_ring_exit(vcpu: *mut KvmVcpu);

    #[cfg(feature = "kvm_generic_memory_attributes")]
    pub fn kvm_vm_set_mem_attributes(start: u64, end: u64, attr: usize);

    pub fn kvm_unmap_hva_range(start: usize, end: usize);
    pub fn kvm_age_hva(start: usize, end: usize);
    pub fn kvm_test_age_hva(hva: usize);
}

#[repr(C)]
pub struct KvmDirtyRing {
    pub index: i32,
    pub dirty_index: u32,
    pub reset_index: u32,
}

#[repr(C)]
pub struct KvmVcpu {
    pub vcpu_id: i32,
}

#[inline]
pub unsafe fn trace_kvm_halt_poll_ns_grow(vcpu_id: u32, new: u32, old: u32) {
    kvm_halt_poll_ns(true, vcpu_id, new, old);
}

#[inline]
pub unsafe fn trace_kvm_halt_poll_ns_shrink(vcpu_id: u32, new: u32, old: u32) {
    kvm_halt_poll_ns(false, vcpu_id, new, old);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
