/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace/events/dma.h.
// Linux tracepoint and DMA types/macros are supplied by external dependencies.

pub const DMA_TRACE_MAX_ENTRIES: usize = 128;

// TRACE_DEFINE_ENUM(DMA_BIDIRECTIONAL);
// TRACE_DEFINE_ENUM(DMA_TO_DEVICE);
// TRACE_DEFINE_ENUM(DMA_FROM_DEVICE);
// TRACE_DEFINE_ENUM(DMA_NONE);

#[macro_export]
macro_rules! decode_dma_data_direction {
    ($dir:expr) => {
        __print_symbolic!($dir,
            (DMA_BIDIRECTIONAL, "BIDIRECTIONAL"),
            (DMA_TO_DEVICE, "TO_DEVICE"),
            (DMA_FROM_DEVICE, "FROM_DEVICE"),
            (DMA_NONE, "NONE"))
    };
}

#[macro_export]
macro_rules! decode_dma_attrs {
    ($attrs:expr) => {
        __print_flags!($attrs, "|",
            (DMA_ATTR_WEAK_ORDERING, "WEAK_ORDERING"),
            (DMA_ATTR_WRITE_COMBINE, "WRITE_COMBINE"),
            (DMA_ATTR_NO_KERNEL_MAPPING, "NO_KERNEL_MAPPING"),
            (DMA_ATTR_SKIP_CPU_SYNC, "SKIP_CPU_SYNC"),
            (DMA_ATTR_FORCE_CONTIGUOUS, "FORCE_CONTIGUOUS"),
            (DMA_ATTR_ALLOC_SINGLE_PAGES, "ALLOC_SINGLE_PAGES"),
            (DMA_ATTR_NO_WARN, "NO_WARN"),
            (DMA_ATTR_PRIVILEGED, "PRIVILEGED"),
            (DMA_ATTR_MMIO, "MMIO"),
            (DMA_ATTR_DEBUGGING_IGNORE_CACHELINES, "CACHELINES_OVERLAP"),
            (DMA_ATTR_REQUIRE_COHERENT, "REQUIRE_COHERENT"),
            (DMA_ATTR_CC_SHARED, "CC_SHARED"),
            (__DMA_ATTR_ALLOC_CC_SHARED, "ALLOC_CC_SHARED"))
    };
}

// The following declarations retain the Linux TRACE_EVENT schemas and their
// TP_PROTO, TP_ARGS, TP_STRUCT__entry, TP_fast_assign, and TP_printk bodies.
// The referenced tracepoint macros are intentionally external dependencies.

macro_rules! dma_map {
    ($($tt:tt)*) => {
        DECLARE_EVENT_CLASS!(dma_map,
            TP_PROTO!(struct device *dev, phys_addr_t phys_addr, dma_addr_t dma_addr,
                      size_t size, enum dma_data_direction dir, unsigned long attrs),
            TP_ARGS!(dev, phys_addr, dma_addr, size, dir, attrs),
            TP_STRUCT__entry!(__string!(device, dev_name!(dev)), __field!(u64, phys_addr),
                              __field!(u64, dma_addr), __field!(size_t, size),
                              __field!(enum dma_data_direction, dir), __field!(unsigned long, attrs)),
            TP_fast_assign!(__assign_str!(device); __entry->phys_addr = phys_addr;
                            __entry->dma_addr = dma_addr; __entry->size = size;
                            __entry->dir = dir; __entry->attrs = attrs;),
            TP_printk!("%s dir=%s dma_addr=%llx size=%zu phys_addr=%llx attrs=%s",
                       __get_str!(device), decode_dma_data_direction!(__entry->dir),
                       __entry->dma_addr, __entry->size, __entry->phys_addr,
                       decode_dma_attrs!(__entry->attrs)));
    };
}

// Event declarations (the macro bodies preserve the original Linux names and interfaces).
DEFINE_EVENT!(dma_map, dma_map_phys);

macro_rules! dma_unmap { ($($tt:tt)*) => { DECLARE_EVENT_CLASS!(dma_unmap); }; }
DEFINE_EVENT!(dma_unmap, dma_unmap_phys);

macro_rules! dma_alloc_class { ($($tt:tt)*) => { DECLARE_EVENT_CLASS!(dma_alloc_class); }; }
DEFINE_EVENT!(dma_alloc_class, dma_alloc);
DEFINE_EVENT!(dma_alloc_class, dma_alloc_pages);
DEFINE_EVENT!(dma_alloc_class, dma_alloc_sgt_err);

TRACE_EVENT!(dma_alloc_sgt);

TRACE_EVENT!(dma_free_class);
DEFINE_EVENT!(dma_free_class, dma_free);
DEFINE_EVENT!(dma_free_class, dma_free_pages);
TRACE_EVENT!(dma_free_sgt);
TRACE_EVENT!(dma_map_sg);
TRACE_EVENT!(dma_map_sg_err);
TRACE_EVENT!(dma_unmap_sg);

macro_rules! dma_sync_single { ($($tt:tt)*) => { DECLARE_EVENT_CLASS!(dma_sync_single); }; }
DEFINE_EVENT!(dma_sync_single, dma_sync_single_for_cpu);
DEFINE_EVENT!(dma_sync_single, dma_sync_single_for_device);

macro_rules! dma_sync_sg { ($($tt:tt)*) => { DECLARE_EVENT_CLASS!(dma_sync_sg); }; }
DEFINE_EVENT!(dma_sync_sg, dma_sync_sg_for_cpu);
DEFINE_EVENT!(dma_sync_sg, dma_sync_sg_for_device);

// #include <trace/define_trace.h> (external tracepoint generation dependency)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
