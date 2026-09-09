/* SPDX-License-Identifier: GPL-2.0 */
// Translation of the Linux KVM tracepoint header.
// The tracepoint framework declarations supplied by <linux/tracepoint.h> and
// <trace/define_trace.h> are external dependencies of this file.

// TRACE_SYSTEM kvm

/// Tracepoint for guest mode entry.
#[repr(C)]
pub struct KvmPpcInstrEntry {
    pub inst: ::core::ffi::c_uint,
    pub pc: ::core::ffi::c_ulong,
    pub emulate: ::core::ffi::c_uint,
}

#[inline]
pub fn kvm_ppc_instr(
    inst: ::core::ffi::c_uint,
    _pc: ::core::ffi::c_ulong,
    emulate: ::core::ffi::c_uint,
) -> KvmPpcInstrEntry {
    KvmPpcInstrEntry { inst, pc: _pc, emulate }
}

pub const KVM_PPC_INSTR_PRINTK: &str = "inst %u pc 0x%lx emulate %u\n";

#[repr(C)]
pub struct KvmStlbInvalEntry {
    pub stlb_index: ::core::ffi::c_uint,
}

#[inline]
pub fn kvm_stlb_inval(stlb_index: ::core::ffi::c_uint) -> KvmStlbInvalEntry {
    KvmStlbInvalEntry { stlb_index }
}

pub const KVM_STLB_INVAL_PRINTK: &str = "stlb_index %u";

#[repr(C)]
pub struct KvmStlbWriteEntry {
    pub victim: ::core::ffi::c_uint,
    pub tid: ::core::ffi::c_uint,
    pub word0: ::core::ffi::c_uint,
    pub word1: ::core::ffi::c_uint,
    pub word2: ::core::ffi::c_uint,
}

#[inline]
pub fn kvm_stlb_write(
    victim: ::core::ffi::c_uint,
    tid: ::core::ffi::c_uint,
    word0: ::core::ffi::c_uint,
    word1: ::core::ffi::c_uint,
    word2: ::core::ffi::c_uint,
) -> KvmStlbWriteEntry {
    KvmStlbWriteEntry { victim, tid, word0, word1, word2 }
}

pub const KVM_STLB_WRITE_PRINTK: &str = "victim %u tid %u w0 %u w1 %u w2 %u";

#[repr(C)]
pub struct KvmGtlbWriteEntry {
    pub gtlb_index: ::core::ffi::c_uint,
    pub tid: ::core::ffi::c_uint,
    pub word0: ::core::ffi::c_uint,
    pub word1: ::core::ffi::c_uint,
    pub word2: ::core::ffi::c_uint,
}

#[inline]
pub fn kvm_gtlb_write(
    gtlb_index: ::core::ffi::c_uint,
    tid: ::core::ffi::c_uint,
    word0: ::core::ffi::c_uint,
    word1: ::core::ffi::c_uint,
    word2: ::core::ffi::c_uint,
) -> KvmGtlbWriteEntry {
    KvmGtlbWriteEntry { gtlb_index, tid, word0, word1, word2 }
}

pub const KVM_GTLB_WRITE_PRINTK: &str = "gtlb_index %u tid %u w0 %u w1 %u w2 %u";

// Declaration supplied by the KVM subsystem; fields accessed by the C
// tracepoint are represented here only as an external opaque dependency.
#[repr(C)]
pub struct kvm_vcpu {
    pub vcpu_id: u32,
    pub requests: u32,
}

#[repr(C)]
pub struct KvmCheckRequestsEntry {
    pub cpu_nr: u32,
    pub requests: u32,
}

#[inline]
pub unsafe fn kvm_check_requests(vcpu: *const kvm_vcpu) -> KvmCheckRequestsEntry {
    KvmCheckRequestsEntry {
        cpu_nr: (*vcpu).vcpu_id,
        requests: (*vcpu).requests,
    }
}

pub const KVM_CHECK_REQUESTS_PRINTK: &str = "vcpu=%x requests=%x";

// TRACE_INCLUDE_PATH .
// TRACE_INCLUDE_FILE trace

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
