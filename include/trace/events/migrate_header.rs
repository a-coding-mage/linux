/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace/events/migrate.h.
// The C tracepoint registration macros are represented by the declarations
// and layouts below; registration is supplied by the surrounding tracepoint
// implementation.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum migrate_mode {
    MIGRATE_ASYNC = 0,
    MIGRATE_SYNC_LIGHT,
    MIGRATE_SYNC,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum migrate_reason {
    MR_COMPACTION = 0,
    MR_MEMORY_FAILURE,
    MR_MEMORY_HOTPLUG,
    MR_SYSCALL,
    MR_MEMPOLICY_MBIND,
    MR_NUMA_MISPLACED,
    MR_CONTIG_RANGE,
    MR_LONGTERM_PIN,
    MR_DEMOTION,
    MR_DAMON,
    MR_NEVER,
}

// String mappings emitted by MIGRATE_MODE and MIGRATE_REASON.
pub const MIGRATE_MODE: &[(migrate_mode, &str)] = &[
    (migrate_mode::MIGRATE_ASYNC, "MIGRATE_ASYNC"),
    (migrate_mode::MIGRATE_SYNC_LIGHT, "MIGRATE_SYNC_LIGHT"),
    (migrate_mode::MIGRATE_SYNC, "MIGRATE_SYNC"),
];

pub const MIGRATE_REASON: &[(migrate_reason, &str)] = &[
    (migrate_reason::MR_COMPACTION, "compaction"),
    (migrate_reason::MR_MEMORY_FAILURE, "memory_failure"),
    (migrate_reason::MR_MEMORY_HOTPLUG, "memory_hotplug"),
    (migrate_reason::MR_SYSCALL, "syscall_or_cpuset"),
    (migrate_reason::MR_MEMPOLICY_MBIND, "mempolicy_mbind"),
    (migrate_reason::MR_NUMA_MISPLACED, "numa_misplaced"),
    (migrate_reason::MR_CONTIG_RANGE, "contig_range"),
    (migrate_reason::MR_LONGTERM_PIN, "longterm_pin"),
    (migrate_reason::MR_DEMOTION, "demotion"),
    (migrate_reason::MR_DAMON, "damon"),
    (migrate_reason::MR_NEVER, "never_migrated"),
];

#[repr(C)]
pub struct MmMigratePages {
    pub succeeded: ::core::ffi::c_ulong,
    pub failed: ::core::ffi::c_ulong,
    pub thp_succeeded: ::core::ffi::c_ulong,
    pub thp_failed: ::core::ffi::c_ulong,
    pub thp_split: ::core::ffi::c_ulong,
    pub large_folio_split: ::core::ffi::c_ulong,
    pub mode: migrate_mode,
    pub reason: migrate_reason,
}

#[repr(C)]
pub struct MmMigratePagesStart {
    pub mode: migrate_mode,
    pub reason: migrate_reason,
}

#[repr(C)]
pub struct MigrationPte {
    pub addr: ::core::ffi::c_ulong,
    pub pte: ::core::ffi::c_ulong,
    pub order: ::core::ffi::c_int,
}

// TRACE_EVENT(mm_migrate_pages):
// TP_PROTO(unsigned long succeeded, unsigned long failed,
//          unsigned long thp_succeeded, unsigned long thp_failed,
//          unsigned long thp_split, unsigned long large_folio_split,
//          enum migrate_mode mode, enum migrate_reason reason)
// TP_PRINTK("nr_succeeded=%lu nr_failed=%lu nr_thp_succeeded=%lu nr_thp_failed=%lu nr_thp_split=%lu nr_split=%lu mode=%s reason=%s", ...)

// TRACE_EVENT(mm_migrate_pages_start):
// TP_PROTO(enum migrate_mode mode, enum migrate_reason reason)
// TP_PRINTK("mode=%s reason=%s", ...)

// DECLARE_EVENT_CLASS(migration_pte):
// TP_PROTO(unsigned long addr, unsigned long pte, int order)
// TP_PRINTK("addr=%lx, pte=%lx order=%d", ...)

// DEFINE_EVENT(migration_pte, set_migration_pte)
// DEFINE_EVENT(migration_pte, remove_migration_pte)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
