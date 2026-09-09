/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace-s390.h. Linux tracepoint generation is represented
// by the event payloads and their source-level assignment/print expressions.

use core::ffi::c_void;

// External types and constants are supplied by the surrounding kernel bindings.
#[repr(C)]
pub struct kvm_vcpu {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kvm_s390_sie_block {
    _private: [u8; 0],
}

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;

extern "C" {
    fn __print_symbolic(value: __u32, entries: *const IrqTypeEntry) -> *const u8;
}

#[repr(C)]
pub struct IrqTypeEntry {
    pub value: __u32,
    pub name: *const u8,
}

pub const KVM_S390_INT_TYPES: &[IrqTypeEntry] = &[
    IrqTypeEntry { value: KVM_S390_SIGP_STOP, name: b"sigp stop\0".as_ptr() },
    IrqTypeEntry { value: KVM_S390_PROGRAM_INT, name: b"program interrupt\0".as_ptr() },
    IrqTypeEntry { value: KVM_S390_SIGP_SET_PREFIX, name: b"sigp set prefix\0".as_ptr() },
    IrqTypeEntry { value: KVM_S390_RESTART, name: b"sigp restart\0".as_ptr() },
    IrqTypeEntry { value: KVM_S390_INT_PFAULT_INIT, name: b"pfault init\0".as_ptr() },
    IrqTypeEntry { value: KVM_S390_INT_PFAULT_DONE, name: b"pfault done\0".as_ptr() },
    IrqTypeEntry { value: KVM_S390_MCHK, name: b"machine check\0".as_ptr() },
    IrqTypeEntry { value: KVM_S390_INT_CLOCK_COMP, name: b"clock comparator\0".as_ptr() },
    IrqTypeEntry { value: KVM_S390_INT_CPU_TIMER, name: b"cpu timer\0".as_ptr() },
    IrqTypeEntry { value: KVM_S390_INT_VIRTIO, name: b"virtio interrupt\0".as_ptr() },
    IrqTypeEntry { value: KVM_S390_INT_SERVICE, name: b"sclp interrupt\0".as_ptr() },
    IrqTypeEntry { value: KVM_S390_INT_EMERGENCY, name: b"sigp emergency\0".as_ptr() },
    IrqTypeEntry { value: KVM_S390_INT_EXTERNAL_CALL, name: b"sigp ext call\0".as_ptr() },
];

pub unsafe fn get_irq_name(__type: __u32) -> *const u8 {
    if __type > KVM_S390_INT_IO_MAX {
        __print_symbolic(__type, KVM_S390_INT_TYPES.as_ptr())
    } else if (__type & KVM_S390_INT_IO_AI_MASK) != 0 {
        b"adapter I/O interrupt\0".as_ptr()
    } else {
        b"subchannel I/O interrupt\0".as_ptr()
    }
}

#[repr(C)]
pub struct kvm_s390_create_vm_entry { pub type_: usize }
#[repr(C)]
pub struct kvm_s390_create_vcpu_entry {
    pub id: u32,
    pub vcpu: *mut kvm_vcpu,
    pub sie_block: *mut kvm_s390_sie_block,
}
#[repr(C)]
pub struct kvm_s390_destroy_vcpu_entry { pub id: u32 }
#[repr(C)]
pub struct kvm_s390_vcpu_start_stop_entry { pub id: u32, pub state: i32 }
#[repr(C)]
pub struct kvm_s390_inject_vm_entry { pub inttype: u32, pub parm: u32, pub parm64: u64, pub who: i32 }
#[repr(C)]
pub struct kvm_s390_inject_vcpu_entry { pub id: i32, pub inttype: u32, pub parm: u32, pub parm64: u64 }
#[repr(C)]
pub struct kvm_s390_deliver_interrupt_entry { pub id: i32, pub inttype: u32, pub data0: u64, pub data1: u64 }
#[repr(C)]
pub struct kvm_s390_request_resets_entry { pub resets: u64 }
#[repr(C)]
pub struct kvm_s390_stop_request_entry { pub stop_irq: u8, pub flags: u8 }
#[repr(C)]
pub struct kvm_s390_enable_css_entry { pub kvm: *mut c_void }
#[repr(C)]
pub struct kvm_s390_enable_disable_ibs_entry { pub id: u32, pub state: i32 }
#[repr(C)]
pub struct kvm_s390_modify_ais_mode_entry { pub isc: u8, pub from: u16, pub to: u16 }
#[repr(C)]
pub struct kvm_s390_airq_suppressed_entry { pub id: u32, pub isc: u8 }
#[repr(C)]
pub struct kvm_s390_gmap_notifier_entry { pub start: usize, pub end: usize, pub shadow: u32 }

// Trace-event assignment and TP_printk expressions, retained as inline helpers.
pub unsafe fn kvm_s390_inject_vm_assign(type_: u64, parm: u32, parm64: u64, who: i32) -> kvm_s390_inject_vm_entry {
    kvm_s390_inject_vm_entry { inttype: (type_ & 0x00000000ffffffff) as u32, parm, parm64, who }
}
pub unsafe fn kvm_s390_inject_vcpu_assign(id: u32, type_: u64, parm: u32, parm64: u64) -> kvm_s390_inject_vcpu_entry {
    kvm_s390_inject_vcpu_entry { id: id as i32, inttype: (type_ & 0x00000000ffffffff) as u32, parm, parm64 }
}
pub unsafe fn kvm_s390_deliver_interrupt_assign(id: u32, type_: u64, data0: u64, data1: u64) -> kvm_s390_deliver_interrupt_entry {
    kvm_s390_deliver_interrupt_entry { id: id as i32, inttype: (type_ & 0x00000000ffffffff) as u32, data0, data1 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
