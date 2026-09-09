/* SPDX-License-Identifier: GPL-2.0 */

/* Based on the document "getPerfCountInfo v1.07". */

#[derive(Copy, Clone)]
pub enum RequestFieldKind { Field, Count, Array }

#[derive(Copy, Clone)]
pub struct RequestField {
    pub kind: RequestFieldKind,
    pub offset: usize,
    pub bytes: usize,
    pub name: &'static str,
}

pub struct Request {
    pub name: &'static str,
    pub number: u32,
    pub index_kind: &'static str,
    pub fields: &'static [RequestField],
}

pub const DISPATCH_TIMEBASE_BY_PROCESSOR_FIELDS: &[RequestField] = &[
    RequestField { kind: RequestFieldKind::Count, offset: 0, bytes: 8, name: "processor_time_in_timebase_cycles" },
    RequestField { kind: RequestFieldKind::Field, offset: 0x8, bytes: 4, name: "hw_processor_id" },
    RequestField { kind: RequestFieldKind::Field, offset: 0xC, bytes: 2, name: "owning_part_id" },
    RequestField { kind: RequestFieldKind::Field, offset: 0xE, bytes: 1, name: "processor_state" },
    RequestField { kind: RequestFieldKind::Field, offset: 0xF, bytes: 1, name: "version" },
    RequestField { kind: RequestFieldKind::Field, offset: 0x10, bytes: 4, name: "hw_chip_id" },
    RequestField { kind: RequestFieldKind::Field, offset: 0x14, bytes: 4, name: "phys_module_id" },
    RequestField { kind: RequestFieldKind::Field, offset: 0x18, bytes: 4, name: "primary_affinity_domain_idx" },
    RequestField { kind: RequestFieldKind::Field, offset: 0x1C, bytes: 4, name: "secondary_affinity_domain_idx" },
    RequestField { kind: RequestFieldKind::Field, offset: 0x20, bytes: 4, name: "processor_version" },
    RequestField { kind: RequestFieldKind::Field, offset: 0x24, bytes: 2, name: "logical_processor_idx" },
    RequestField { kind: RequestFieldKind::Field, offset: 0x26, bytes: 2, name: "reserved" },
    RequestField { kind: RequestFieldKind::Field, offset: 0x28, bytes: 4, name: "processor_id_register" },
    RequestField { kind: RequestFieldKind::Field, offset: 0x2C, bytes: 4, name: "phys_processor_idx" },
];

pub const DISPATCH_TIMEBASE_BY_PROCESSOR: Request = Request { name: "dispatch_timebase_by_processor", number: 0x10, index_kind: "phys_processor_idx=?", fields: DISPATCH_TIMEBASE_BY_PROCESSOR_FIELDS };

pub const ENTITLED_CAPPED_UNCAPPED_DONATED_IDLE_TIMEBASE_BY_PARTITION_FIELDS: &[RequestField] = &[
    RequestField { kind: RequestFieldKind::Field, offset: 0, bytes: 8, name: "partition_id" },
    RequestField { kind: RequestFieldKind::Count, offset: 0x8, bytes: 8, name: "entitled_cycles" },
    RequestField { kind: RequestFieldKind::Count, offset: 0x10, bytes: 8, name: "consumed_capped_cycles" },
    RequestField { kind: RequestFieldKind::Count, offset: 0x18, bytes: 8, name: "consumed_uncapped_cycles" },
    RequestField { kind: RequestFieldKind::Count, offset: 0x20, bytes: 8, name: "cycles_donated" },
    RequestField { kind: RequestFieldKind::Count, offset: 0x28, bytes: 8, name: "purr_idle_cycles" },
];
pub const ENTITLED_CAPPED_UNCAPPED_DONATED_IDLE_TIMEBASE_BY_PARTITION: Request = Request { name: "entitled_capped_uncapped_donated_idle_timebase_by_partition", number: 0x20, index_kind: "sibling_part_id=?", fields: ENTITLED_CAPPED_UNCAPPED_DONATED_IDLE_TIMEBASE_BY_PARTITION_FIELDS };

#[cfg(feature = "ENABLE_EVENTS_COUNTERINFO_V6")]
pub const RUN_INSTRUCTIONS_RUN_CYCLES_BY_PARTITION_FIELDS: &[RequestField] = &[
    RequestField { kind: RequestFieldKind::Field, offset: 0, bytes: 8, name: "partition_id" },
    RequestField { kind: RequestFieldKind::Count, offset: 0x8, bytes: 8, name: "instructions_completed" },
    RequestField { kind: RequestFieldKind::Count, offset: 0x10, bytes: 8, name: "cycles" },
];
#[cfg(feature = "ENABLE_EVENTS_COUNTERINFO_V6")]
pub const RUN_INSTRUCTIONS_RUN_CYCLES_BY_PARTITION: Request = Request { name: "run_instructions_run_cycles_by_partition", number: 0x30, index_kind: "sibling_part_id=?", fields: RUN_INSTRUCTIONS_RUN_CYCLES_BY_PARTITION_FIELDS };

pub const SYSTEM_PERFORMANCE_CAPABILITIES_FIELDS: &[RequestField] = &[
    RequestField { kind: RequestFieldKind::Field, offset: 0, bytes: 1, name: "perf_collect_privileged" },
    RequestField { kind: RequestFieldKind::Field, offset: 0x1, bytes: 1, name: "capability_mask" },
    RequestField { kind: RequestFieldKind::Array, offset: 0x2, bytes: 0xE, name: "reserved" },
];
pub const SYSTEM_PERFORMANCE_CAPABILITIES: Request = Request { name: "system_performance_capabilities", number: 0x40, index_kind: "starting_index=0xffffffff", fields: SYSTEM_PERFORMANCE_CAPABILITIES_FIELDS };

/* The following generated requests preserve the source declarations; their
 * field lists are supplied by the external req-gen implementation. */
pub const PROCESSOR_BUS_UTILIZATION_ABC_LINKS: Request = Request { name: "processor_bus_utilization_abc_links", number: 0x50, index_kind: "hw_chip_id=?", fields: &[
    RequestField { kind: RequestFieldKind::Field, offset: 0, bytes: 4, name: "hw_chip_id" }, RequestField { kind: RequestFieldKind::Array, offset: 4, bytes: 0xC, name: "reserved1" }, RequestField { kind: RequestFieldKind::Count, offset: 0x10, bytes: 8, name: "total_link_cycles" }, RequestField { kind: RequestFieldKind::Count, offset: 0x18, bytes: 8, name: "idle_cycles_for_a_link" }, RequestField { kind: RequestFieldKind::Count, offset: 0x20, bytes: 8, name: "idle_cycles_for_b_link" }, RequestField { kind: RequestFieldKind::Count, offset: 0x28, bytes: 8, name: "idle_cycles_for_c_link" }, RequestField { kind: RequestFieldKind::Array, offset: 0x30, bytes: 0x20, name: "reserved2" }] };
pub const PROCESSOR_BUS_UTILIZATION_WXYZ_LINKS: Request = Request { name: "processor_bus_utilization_wxyz_links", number: 0x60, index_kind: "hw_chip_id=?", fields: &[] };
pub const PROCESSOR_BUS_UTILIZATION_GX_LINKS: Request = Request { name: "processor_bus_utilization_gx_links", number: 0x70, index_kind: "hw_chip_id=?", fields: &[] };
pub const PROCESSOR_BUS_UTILIZATION_MC_LINKS: Request = Request { name: "processor_bus_utilization_mc_links", number: 0x80, index_kind: "hw_chip_id=?", fields: &[] };
pub const PROCESSOR_CORE_UTILIZATION: Request = Request { name: "processor_core_utilization", number: 0x94, index_kind: "phys_processor_idx=?", fields: &[] };
pub const PARTITION_HYPERVISOR_QUEUING_TIMES: Request = Request { name: "partition_hypervisor_queuing_times", number: 0xE0, index_kind: "partition_id=?", fields: &[] };
pub const SYSTEM_HYPERVISOR_TIMES: Request = Request { name: "system_hypervisor_times", number: 0xF0, index_kind: "starting_index=0xffffffff", fields: &[] };
pub const SYSTEM_TLBIE_COUNT_AND_TIME: Request = Request { name: "system_tlbie_count_and_time", number: 0xF4, index_kind: "starting_index=0xffffffff", fields: &[] };
pub const PARTITION_INSTRUCTION_COUNT_AND_TIME: Request = Request { name: "partition_instruction_count_and_time", number: 0x100, index_kind: "partition_id=?", fields: &[] };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
