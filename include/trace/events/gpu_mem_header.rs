/* SPDX-License-Identifier: GPL-2.0 */
/*
 * GPU memory trace points
 *
 * Copyright (C) 2020 Google, Inc.
 */

// TRACE_SYSTEM gpu_mem
// The Linux tracepoint definitions are supplied by the surrounding tracepoint
// infrastructure.  The event payload is represented here with its C layout.

/*
 * The gpu_memory_total event indicates that there's an update to either the
 * global or process total gpu memory counters.
 *
 * This event should be emitted whenever the kernel device driver allocates,
 * frees, imports, unimports memory in the GPU addressable space.
 *
 * @gpu_id: This is the gpu id.
 *
 * @pid: Put 0 for global total, while positive pid for process total.
 *
 * @size: Size of the allocation in bytes.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpu_mem_total_entry {
    pub gpu_id: u32,
    pub pid: u32,
    pub size: u64,
}

impl gpu_mem_total_entry {
    #[inline]
    pub const unsafe fn new(gpu_id: u32, pid: u32, size: u64) -> Self {
        Self { gpu_id, pid, size }
    }
}

// TP_printk("gpu_id=%u pid=%u size=%llu", __entry->gpu_id, __entry->pid,
//           __entry->size)
pub const GPU_MEM_TOTAL_PRINT_FORMAT: &str = "gpu_id=%u pid=%u size=%llu";


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
