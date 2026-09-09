// Translation of asm/trace/hyperv.h.
//
// The C header uses the Linux TRACE_EVENT declarative macro.  The event
// payloads below preserve the fields and their C layout; registration,
// invocation, and formatted printing remain provided by the tracepoint
// implementation.

// CONFIG_HYPERV controls whether these trace events are emitted in the C
// build.  Rust build configuration for that external dependency is preserved
// by this conditional module.
#[cfg(feature = "CONFIG_HYPERV")]
pub mod hyperv_trace_events {
    use core::ffi::c_int;

    #[repr(C)]
    pub struct HypervMmuFlushTlbMultiEntry {
        pub ncpus: core::ffi::c_uint,
        pub mm: *mut core::ffi::c_void,
        pub addr: usize,
        pub end: usize,
    }

    #[repr(C)]
    pub struct HypervNestedFlushGuestMappingEntry {
        pub as_: u64,
        pub ret: c_int,
    }

    #[repr(C)]
    pub struct HypervNestedFlushGuestMappingRangeEntry {
        pub as_: u64,
        pub ret: c_int,
    }

    #[repr(C)]
    pub struct HypervSendIpiMaskEntry {
        pub ncpus: core::ffi::c_uint,
        pub vector: c_int,
    }

    #[repr(C)]
    pub struct HypervSendIpiOneEntry {
        pub cpu: c_int,
        pub vector: c_int,
    }

    // TRACE_EVENT(hyperv_mmu_flush_tlb_multi,
    //     TP_PROTO(const struct cpumask *cpus,
    //              const struct flush_tlb_info *info),
    //     TP_ARGS(cpus, info),
    //     TP_fast_assign(__entry->ncpus = cpumask_weight(cpus);
    //                    __entry->mm = info->mm;
    //                    __entry->addr = info->start;
    //                    __entry->end = info->end;),
    //     TP_printk("ncpus %d mm %p addr %lx, end %lx", ...))

    // TRACE_EVENT(hyperv_nested_flush_guest_mapping,
    //     TP_PROTO(u64 as, int ret), TP_ARGS(as, ret),
    //     TP_fast_assign(__entry->as = as; __entry->ret = ret;),
    //     TP_printk("address space %llx ret %d", ...))

    // TRACE_EVENT(hyperv_nested_flush_guest_mapping_range,
    //     TP_PROTO(u64 as, int ret), TP_ARGS(as, ret),
    //     TP_fast_assign(__entry->as = as; __entry->ret = ret;),
    //     TP_printk("address space %llx ret %d", ...))

    // TRACE_EVENT(hyperv_send_ipi_mask,
    //     TP_PROTO(const struct cpumask *cpus, int vector),
    //     TP_ARGS(cpus, vector),
    //     TP_fast_assign(__entry->ncpus = cpumask_weight(cpus);
    //                    __entry->vector = vector;),
    //     TP_printk("ncpus %d vector %x", ...))

    // TRACE_EVENT(hyperv_send_ipi_one,
    //     TP_PROTO(int cpu, int vector), TP_ARGS(cpu, vector),
    //     TP_fast_assign(__entry->cpu = cpu; __entry->vector = vector;),
    //     TP_printk("cpu %d vector %x", ...))
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
