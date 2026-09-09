/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 * IMC Nest Performance Monitor counter support.
 *
 * Copyright (C) 2017 Madhavan Srinivasan, IBM Corporation.
 *           (C) 2017 Anju T Sudhakar, IBM Corporation.
 *           (C) 2017 Hemant K Shaw, IBM Corporation.
 */

/* Compatibility macros for IMC devices. */
pub const IMC_DTB_COMPAT: &str = "ibm,opal-in-memory-counters";
pub const IMC_DTB_UNIT_COMPAT: &str = "ibm,imc-counters";

/* LDBAR: Counter address and Enable/Disable macro. */
pub const THREAD_IMC_LDBAR_MASK: u64 = 0x0003_ffff_ffff_e000u64;
pub const THREAD_IMC_ENABLE: u64 = 0x8000_0000_0000_0000u64;
pub const TRACE_IMC_ENABLE: u64 = 0x4000_0000_0000_0000u64;

/* For debugfs interface for imc-mode and imc-command. */
pub const IMC_CNTL_BLK_OFFSET: u32 = 0x3fc00;
pub const IMC_CNTL_BLK_CMD_OFFSET: u32 = 8;
pub const IMC_CNTL_BLK_MODE_OFFSET: u32 = 32;

/* Structure to hold memory address information for imc units. */
#[repr(C)]
pub struct imc_mem_info {
    pub vbase: *mut u64,
    pub id: u32,
}

/* Place holder for nest pmu events and values. */
#[repr(C)]
pub struct imc_events {
    pub value: u32,
    pub name: *mut core::ffi::c_char,
    pub unit: *mut core::ffi::c_char,
    pub scale: *mut core::ffi::c_char,
}

/*
 * Trace IMC hardware updates a 64bytes record on
 * Core Performance Monitoring Counter (CPMC)
 * overflow. Here is the layout for the trace imc record
 *
 * DW 0 : Timebase
 * DW 1 : Program Counter
 * DW 2 : PIDR information
 * DW 3 : CPMC1
 * DW 4 : CPMC2
 * DW 5 : CPMC3
 * Dw 6 : CPMC4
 * DW 7 : Timebase
 * .....
 *
 * The following is the data structure to hold trace imc data.
 */
#[repr(C)]
pub struct trace_imc_data {
    pub tb1: u64,
    pub ip: u64,
    pub val: u64,
    pub cpmc1: u64,
    pub cpmc2: u64,
    pub cpmc3: u64,
    pub cpmc4: u64,
    pub tb2: u64,
}

/* Event attribute array index. */
pub const IMC_FORMAT_ATTR: u32 = 0;
pub const IMC_EVENT_ATTR: u32 = 1;
pub const IMC_CPUMASK_ATTR: u32 = 2;
pub const IMC_NULL_ATTR: u32 = 3;

/* PMU Format attribute macros. */
pub const IMC_EVENT_OFFSET_MASK: u64 = 0xffff_ffffu64;

/* Mask bits 0:21 of the first double word (timebase). */
pub const IMC_TRACE_RECORD_TB1_MASK: u64 = 0x3ffffffffffu64;

/* Bit 0:1 in the third DW specifies the MSR[HV PR] values. */
#[inline]
pub const fn IMC_TRACE_RECORD_VAL_HVPR(x: u64) -> u64 {
    x >> 62
}

/*
 * Device tree parser code detects IMC pmu support and
 * registers new IMC pmus. This structure will hold the
 * pmu functions, events, counter memory information
 * and attrs for each imc pmu and will be referenced at
 * the time of pmu registration.
 */
#[repr(C)]
pub struct imc_pmu {
    pub pmu: pmu,
    pub mem_info: *mut imc_mem_info,
    pub events: *mut imc_events,
    /*
     * Attribute groups for the PMU. Slot 0 used for
     * format attribute, slot 1 used for cpusmask attribute,
     * slot 2 used for event attribute. Slot 3 keep as
     * NULL.
     */
    pub attr_groups: [*const attribute_group; 4],
    pub counter_mem_size: u32,
    pub domain: i32,
    /* flag to notify whether the memory is mmaped or allocated by kernel. */
    pub imc_counter_mmaped: bool,
}

/* Structure to hold id, lock and reference count for initialized imc events. */
#[repr(C)]
pub struct imc_pmu_ref {
    pub lock: spinlock_t,
    pub id: u32,
    pub refc: i32,
}

/* In-Memory Collection Counters type. Data comes from Device tree. */
pub const IMC_TYPE_THREAD: u32 = 0x1;
pub const IMC_TYPE_TRACE: u32 = 0x2;
pub const IMC_TYPE_CORE: u32 = 0x4;
pub const IMC_TYPE_CHIP: u32 = 0x10;

/* Domains for IMC PMUs. */
pub const IMC_DOMAIN_NEST: i32 = 1;
pub const IMC_DOMAIN_CORE: i32 = 2;
pub const IMC_DOMAIN_THREAD: i32 = 3;
/* For trace-imc the domain is still thread but it operates in trace-mode. */
pub const IMC_DOMAIN_TRACE: i32 = 4;

extern "C" {
    pub fn init_imc_pmu(parent: *mut device_node, pmu_ptr: *mut imc_pmu, pmu_id: i32) -> i32;
    pub fn thread_imc_disable();
    pub fn get_max_nest_dev() -> i32;
    pub fn unregister_thread_imc();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
