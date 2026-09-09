// SPDX-License-Identifier: GPL-2.0
/*
 * Tracepoints for RISC-V KVM
 *
 * Copyright 2024 Beijing ESWIN Computing Technology Co., Ltd.
 *
 */

// C header guard: !defined(_TRACE_KVM_H) || defined(TRACE_HEADER_MULTI_READ)
// C dependency: <linux/tracepoint.h>
// TRACE_SYSTEM kvm

// Opaque types supplied by the corresponding KVM implementation.
#[repr(C)]
pub struct kvm_vcpu {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm_cpu_trap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct KvmEntry {
    pub pc: usize,
}

/// TRACE_EVENT(kvm_entry): TP_PROTO(struct kvm_vcpu *vcpu)
/// TP_PRINTK("PC: 0x%016lx", __entry->pc)
#[inline]
pub unsafe fn kvm_entry(vcpu: *const kvm_vcpu, pc: usize) -> KvmEntry {
    let _ = vcpu;
    KvmEntry { pc }
}

#[repr(C)]
pub struct KvmExit {
    pub sepc: usize,
    pub scause: usize,
    pub stval: usize,
    pub htval: usize,
    pub htinst: usize,
}

/// TRACE_EVENT(kvm_exit): TP_PROTO(struct kvm_cpu_trap *trap)
/// TP_PRINTK("SEPC:0x%lx, SCAUSE:0x%lx, STVAL:0x%lx, HTVAL:0x%lx, HTINST:0x%lx", ...)
#[inline]
pub unsafe fn kvm_exit(
    trap: *const kvm_cpu_trap,
    sepc: usize,
    scause: usize,
    stval: usize,
    htval: usize,
    htinst: usize,
) -> KvmExit {
    let _ = trap;
    KvmExit { sepc, scause, stval, htval, htinst }
}

#[repr(C)]
pub struct KvmMmioEmulate {
    pub vcpu_id: usize,
    pub sepc: usize,
    pub insn: usize,
    pub fault_addr: usize,
    pub write: bool,
    pub len: i32,
}

/// TRACE_EVENT(kvm_mmio_emulate): TP_PROTO(unsigned long vcpu_id, unsigned long sepc,
/// unsigned long insn, unsigned long fault_addr, bool write, int len)
/// TP_PRINTK("VCPU: %lu, %s MMIO at 0x%lx, len %d, insn 0x%lx, sepc 0x%lx", ...)
#[inline]
pub fn kvm_mmio_emulate(
    vcpu_id: usize,
    sepc: usize,
    insn: usize,
    fault_addr: usize,
    write: bool,
    len: i32,
) -> KvmMmioEmulate {
    KvmMmioEmulate { vcpu_id, sepc, insn, fault_addr, write, len }
}

#[repr(C)]
pub struct KvmVcpuExit {
    pub vcpu_id: usize,
    pub sepc: usize,
    pub scause: usize,
    pub stval: usize,
    pub htval: usize,
    pub htinst: usize,
}

/// TRACE_EVENT(kvm_vcpu_exit): TP_PROTO(unsigned long vcpu_id, unsigned long sepc,
/// unsigned long scause, unsigned long stval, unsigned long htval, unsigned long htinst)
/// TP_PRINTK("VCPU: %lu, SEPC: 0x%lx, SCAUSE: 0x%lx, STVAL: 0x%lx, HTVAL: 0x%lx, HTINST: 0x%lx", ...)
#[inline]
pub fn kvm_vcpu_exit(
    vcpu_id: usize,
    sepc: usize,
    scause: usize,
    stval: usize,
    htval: usize,
    htinst: usize,
) -> KvmVcpuExit {
    KvmVcpuExit { vcpu_id, sepc, scause, stval, htval, htinst }
}

#[repr(C)]
pub struct KvmVcpuIrq {
    pub vcpu_id: usize,
    pub irq: u32,
    pub level: i32,
}

/// TRACE_EVENT(kvm_vcpu_irq): TP_PROTO(unsigned long vcpu_id, unsigned int irq, int level)
/// TP_PRINTK("VCPU: %lu, IRQ: %u, level: %d", __entry->vcpu_id, __entry->irq, __entry->level)
#[inline]
pub fn kvm_vcpu_irq(vcpu_id: usize, irq: u32, level: i32) -> KvmVcpuIrq {
    KvmVcpuIrq { vcpu_id, irq, level }
}

// TRACE_INCLUDE_PATH .
// TRACE_INCLUDE_FILE trace
// This part must be outside protection: <trace/define_trace.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
