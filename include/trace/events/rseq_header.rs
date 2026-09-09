/* SPDX-License-Identifier: GPL-2.0+ */
// TRACE_SYSTEM: rseq
//
// The C header guard and tracepoint includes are intentionally represented as
// comments; their declarations are supplied by the surrounding tracepoint
// implementation.

#[repr(C)]
pub struct rseq_update_entry {
    pub cpu_id: i32,
    pub node_id: i32,
    pub mm_cid: i32,
}

#[repr(C)]
pub struct rseq_ip_fixup_entry {
    pub regs_ip: usize,
    pub start_ip: usize,
    pub post_commit_offset: usize,
    pub abort_ip: usize,
}

extern "C" {
    pub fn cpu_to_node(cpu: i32) -> i32;
}

/// TRACE_EVENT(rseq_update)
///
/// TP_PROTO(struct task_struct *t)
/// TP_ARGS(t)
/// TP_fast_assign:
///   __entry->cpu_id = t->rseq.ids.cpu_id;
///   __entry->node_id = cpu_to_node(__entry->cpu_id);
///   __entry->mm_cid = t->rseq.ids.mm_cid;
/// TP_printk("cpu_id=%d node_id=%d mm_cid=%d", ...)
///
/// The task_struct/rseq layout is provided by the kernel dependency that owns
/// the tracepoint; this declaration preserves the trace event's assignment
/// operation without redefining that external type.
#[inline(always)]
pub unsafe fn rseq_update_fast_assign(
    entry: *mut rseq_update_entry,
    cpu_id: i32,
    mm_cid: i32,
) {
    (*entry).cpu_id = cpu_id;
    (*entry).node_id = cpu_to_node((*entry).cpu_id);
    (*entry).mm_cid = mm_cid;
}

/// TRACE_EVENT(rseq_ip_fixup)
///
/// TP_PROTO(unsigned long regs_ip, unsigned long start_ip,
///          unsigned long post_commit_offset, unsigned long abort_ip)
/// TP_ARGS(regs_ip, start_ip, post_commit_offset, abort_ip)
/// TP_fast_assign:
///   __entry->regs_ip = regs_ip;
///   __entry->start_ip = start_ip;
///   __entry->post_commit_offset = post_commit_offset;
///   __entry->abort_ip = abort_ip;
/// TP_printk("regs_ip=0x%lx start_ip=0x%lx post_commit_offset=%lu abort_ip=0x%lx", ...)
#[inline(always)]
pub unsafe fn rseq_ip_fixup_fast_assign(
    entry: *mut rseq_ip_fixup_entry,
    regs_ip: usize,
    start_ip: usize,
    post_commit_offset: usize,
    abort_ip: usize,
) {
    (*entry).regs_ip = regs_ip;
    (*entry).start_ip = start_ip;
    (*entry).post_commit_offset = post_commit_offset;
    (*entry).abort_ip = abort_ip;
}

// The define_trace include is intentionally omitted from executable Rust;
// tracepoint generation remains an external build-time concern.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
