/* SPDX-License-Identifier: GPL-2.0 */

// Translated from trace_pr.h.  The Linux TRACE_EVENT declarations below are
// represented as C-layout event payloads; tracepoint registration and printk
// formatting are supplied by the surrounding tracing implementation.

#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[repr(C)]
pub struct KvmBook3s64MmuMap {
    pub flag_w: u8,
    pub flag_x: u8,
    pub eaddr: libc::c_ulong,
    pub hpteg: libc::c_ulong,
    pub va: libc::c_ulong,
    pub vpage: u64,
    pub hpaddr: libc::c_ulong,
}

#[repr(C)]
pub struct KvmBook3sReenter {
    pub r: libc::c_uint,
    pub pc: libc::c_ulong,
}

#[repr(C)]
pub struct KvmBook3sMmuMap {
    pub host_vpn: u64,
    pub pfn: u64,
    pub eaddr: libc::c_ulong,
    pub vpage: u64,
    pub raddr: libc::c_ulong,
    pub flags: libc::c_int,
}

#[repr(C)]
pub struct KvmBook3sMmuInvalidate {
    pub host_vpn: u64,
    pub pfn: u64,
    pub eaddr: libc::c_ulong,
    pub vpage: u64,
    pub raddr: libc::c_ulong,
    pub flags: libc::c_int,
}

#[repr(C)]
pub struct KvmBook3sMmuFlush {
    pub count: libc::c_int,
    pub p1: u64,
    pub p2: u64,
    pub type_: *const libc::c_char,
}

#[repr(C)]
pub struct KvmBook3sSlbFound {
    pub gvsid: u64,
    pub hvsid: u64,
}

#[repr(C)]
pub struct KvmBook3sSlbFail {
    pub sid_map_mask: u16,
    pub gvsid: u64,
}

#[repr(C)]
pub struct KvmBook3sSlbMap {
    pub sid_map_mask: u16,
    pub guest_vsid: u64,
    pub host_vsid: u64,
}

#[repr(C)]
pub struct KvmBook3sSlbmte {
    pub slb_vsid: u64,
    pub slb_esid: u64,
}

#[repr(C)]
pub struct KvmExit {
    pub exit_nr: libc::c_uint,
    pub pc: libc::c_ulong,
    pub msr: libc::c_ulong,
    pub dar: libc::c_ulong,
    pub srr1: libc::c_ulong,
    pub last_inst: libc::c_ulong,
}

// TRACE_EVENT(kvm_book3s_reenter):
// TP_PROTO(int r, struct kvm_vcpu *vcpu)
// TP_fast_assign: r = r; pc = kvmppc_get_pc(vcpu)
// TP_printk: "reentry r=%d | pc=0x%lx"

// CONFIG_PPC_BOOK3S_64 conditionally declares kvm_book3s_64_mmu_map.
// Its assignment is:
// flag_w = ((rflags & HPTE_R_PP) == 3) ? '-' : 'w';
// flag_x = (rflags & HPTE_R_N) ? '-' : 'x';
// eaddr = orig_pte->eaddr; hpteg = hpteg; va = va;
// vpage = orig_pte->vpage; hpaddr = hpaddr.

// TRACE_EVENT(kvm_book3s_mmu_map) and TRACE_EVENT(kvm_book3s_mmu_invalidate)
// assign host_vpn, pfn, eaddr, vpage, raddr from pte and compute:
// (pte->pte.may_read ? 0x4 : 0) |
// (pte->pte.may_write ? 0x2 : 0) |
// (pte->pte.may_execute ? 0x1 : 0).

// TRACE_EVENT(kvm_book3s_mmu_flush): count = to_book3s(vcpu)->hpte_cache_count;
// p1 = p1; p2 = p2; type = type;
// TRACE_EVENT(kvm_book3s_slb_found): gvsid = gvsid; hvsid = hvsid;
// TRACE_EVENT(kvm_book3s_slb_fail): sid_map_mask = sid_map_mask; gvsid = gvsid;
// TP_printk("%x/%x: %llx", sid_map_mask, SID_MAP_MASK - sid_map_mask, gvsid)
// TRACE_EVENT(kvm_book3s_slb_map): sid_map_mask = sid_map_mask;
// guest_vsid = gvsid; host_vsid = hvsid;
// TRACE_EVENT(kvm_book3s_slbmte): slb_vsid = slb_vsid; slb_esid = slb_esid;

// TRACE_EVENT(kvm_exit):
// exit_nr = exit_nr;
// pc = kvmppc_get_pc(vcpu);
// dar = kvmppc_get_fault_dar(vcpu);
// msr = kvmppc_get_msr(vcpu);
// srr1 = vcpu->arch.shadow_srr1;
// last_inst = vcpu->arch.last_inst;
// TP_printk("exit=%s | pc=0x%lx | msr=0x%lx | dar=0x%lx | srr1=0x%lx | last_inst=0x%lx",
//           __print_symbolic(exit_nr, kvm_trace_symbol_exit), pc, msr, dar,
//           srr1, last_inst)

// TRACE_INCLUDE_PATH .
// TRACE_INCLUDE_FILE trace_pr
// <trace/define_trace.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
