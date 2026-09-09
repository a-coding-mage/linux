/* SPDX-License-Identifier: GPL-2.0-or-later */
// Translated from proto_memory.h.
// Dependencies supplied by the surrounding kernel translation:
// net/sock.h, net/hotdata.h

/* 1 MB per cpu, in page units */
pub const SK_MEMORY_PCPU_RESERVE: usize = 1usize << (20 - PAGE_SHIFT);

#[inline]
pub unsafe fn sk_has_memory_pressure(sk: *const sock) -> bool {
    !(*(*sk).sk_prot).memory_pressure.is_null()
}

#[inline]
pub unsafe fn proto_memory_pressure(prot: *const proto) -> bool {
    if (*prot).memory_pressure.is_null() {
        return false;
    }
    READ_ONCE(*(*prot).memory_pressure) != 0
}

#[inline]
pub unsafe fn sk_under_global_memory_pressure(sk: *const sock) -> bool {
    proto_memory_pressure((*sk).sk_prot)
}

#[inline]
pub unsafe fn sk_under_memory_pressure(sk: *const sock) -> bool {
    if (*(*sk).sk_prot).memory_pressure.is_null() {
        return false;
    }

    if mem_cgroup_sk_enabled(sk) && mem_cgroup_sk_under_memory_pressure(sk) {
        return true;
    }

    if (*sk).sk_bypass_prot_mem {
        return false;
    }

    READ_ONCE(*(*(*sk).sk_prot).memory_pressure) != 0
}

#[inline]
pub unsafe fn proto_memory_allocated(prot: *const proto) -> c_long {
    core::cmp::max(0 as c_long, atomic_long_read((*prot).memory_allocated))
}

#[inline]
pub unsafe fn sk_memory_allocated(sk: *const sock) -> c_long {
    proto_memory_allocated((*sk).sk_prot)
}

#[inline]
pub unsafe fn proto_memory_pcpu_drain(proto: *mut proto) {
    let val: c_int = this_cpu_xchg((*proto).per_cpu_fw_alloc, 0);

    if val != 0 {
        atomic_long_add(val as c_long, (*proto).memory_allocated);
    }
}

#[inline]
pub unsafe fn sk_memory_allocated_add(sk: *const sock, mut val: c_int) {
    let proto: *mut proto = (*sk).sk_prot;

    val = this_cpu_add_return((*proto).per_cpu_fw_alloc, val);

    if val >= READ_ONCE(net_hotdata.sysctl_mem_pcpu_rsv) {
        proto_memory_pcpu_drain(proto);
    }
}

#[inline]
pub unsafe fn sk_memory_allocated_sub(sk: *const sock, mut val: c_int) {
    let proto: *mut proto = (*sk).sk_prot;

    val = this_cpu_sub_return((*proto).per_cpu_fw_alloc, val);

    if val <= -READ_ONCE(net_hotdata.sysctl_mem_pcpu_rsv) {
        proto_memory_pcpu_drain(proto);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
