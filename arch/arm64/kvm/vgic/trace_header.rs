/* SPDX-License-Identifier: GPL-2.0 */

// TRACE_SYSTEM kvm
//
// The C TRACE_EVENT machinery generates the tracepoint declaration and its
// associated entry layout.  The corresponding Rust layout is preserved here.

#[repr(C)]
pub struct VgicUpdateIrqPendingEntry {
    pub vcpu_id: ::core::ffi::c_ulong,
    pub irq: u32,
    pub level: bool,
}

impl VgicUpdateIrqPendingEntry {
    #[inline]
    pub const unsafe fn new(
        vcpu_id: ::core::ffi::c_ulong,
        irq: u32,
        level: bool,
    ) -> Self {
        Self {
            vcpu_id,
            irq,
            level,
        }
    }
}

// TRACE_EVENT(vgic_update_irq_pending,
//     TP_PROTO(unsigned long vcpu_id, __u32 irq, bool level),
//     TP_ARGS(vcpu_id, irq, level),
//     TP_fast_assign(
//         __entry->vcpu_id = vcpu_id;
//         __entry->irq = irq;
//         __entry->level = level;
//     ),
//     TP_printk("VCPU: %ld, IRQ %d, level: %d",
//         __entry->vcpu_id, __entry->irq, __entry->level)
// );

// TRACE_INCLUDE_PATH ../../arch/arm64/kvm/vgic
// TRACE_INCLUDE_FILE trace
// The C header includes <trace/define_trace.h> outside the include guard.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
