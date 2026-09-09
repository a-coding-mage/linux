/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/kthread.h, kvm/iodev.h, uapi/asm/kvm.h, and ioapic.h.
// The following declarations are conditional on CONFIG_KVM_IOAPIC.

#[cfg(CONFIG_KVM_IOAPIC)]
#[repr(C)]
pub struct kvm_kpit_channel_state {
    pub count: u32, // can be 65536
    pub latched_count: u16,
    pub count_latched: u8,
    pub status_latched: u8,
    pub status: u8,
    pub read_state: u8,
    pub write_state: u8,
    pub write_latch: u8,
    pub rw_mode: u8,
    pub mode: u8,
    pub bcd: u8, // not supported
    pub gate: u8, // timer start
    pub count_load_time: ktime_t,
}

#[cfg(CONFIG_KVM_IOAPIC)]
#[repr(C)]
pub struct kvm_kpit_state {
    // All members before "struct mutex lock" are protected by the lock.
    pub channels: [kvm_kpit_channel_state; 3],
    pub flags: u32,
    pub is_periodic: bool,
    pub period: i64, // unit: ns
    pub timer: hrtimer,

    pub lock: mutex,
    pub reinject: atomic_t,
    pub pending: atomic_t, // accumulated triggered timers
    pub irq_ack: atomic_t,
    pub irq_ack_notifier: kvm_irq_ack_notifier,
}

#[cfg(CONFIG_KVM_IOAPIC)]
#[repr(C)]
pub struct kvm_pit {
    pub dev: kvm_io_device,
    pub speaker_dev: kvm_io_device,
    pub kvm: *mut kvm,
    pub pit_state: kvm_kpit_state,
    pub mask_notifier: kvm_irq_mask_notifier,
    pub worker: *mut kthread_worker,
    pub expired: kthread_work,
}

#[cfg(CONFIG_KVM_IOAPIC)]
pub const KVM_PIT_BASE_ADDRESS: u32 = 0x40;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const KVM_SPEAKER_BASE_ADDRESS: u32 = 0x61;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const KVM_PIT_MEM_LENGTH: u32 = 4;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const KVM_PIT_FREQ: u32 = 1193181;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const KVM_MAX_PIT_INTR_INTERVAL: u32 = HZ / 100;
#[cfg(CONFIG_KVM_IOAPIC)]
pub const KVM_PIT_CHANNEL_MASK: u32 = 0x3;

#[cfg(CONFIG_KVM_IOAPIC)]
extern "C" {
    pub fn kvm_vm_ioctl_get_pit(kvm: *mut kvm, ps: *mut kvm_pit_state) -> i32;
    pub fn kvm_vm_ioctl_set_pit(kvm: *mut kvm, ps: *mut kvm_pit_state) -> i32;
    pub fn kvm_vm_ioctl_get_pit2(kvm: *mut kvm, ps: *mut kvm_pit_state2) -> i32;
    pub fn kvm_vm_ioctl_set_pit2(kvm: *mut kvm, ps: *mut kvm_pit_state2) -> i32;
    pub fn kvm_vm_ioctl_reinject(
        kvm: *mut kvm,
        control: *mut kvm_reinject_control,
    ) -> i32;

    pub fn kvm_create_pit(kvm: *mut kvm, flags: u32) -> *mut kvm_pit;
    pub fn kvm_free_pit(kvm: *mut kvm);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
