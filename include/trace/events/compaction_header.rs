/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of trace/events/compaction.h.
// Linux tracepoint and mmflags dependencies are supplied externally.

#[repr(C)]
pub struct MmCompactionIsolateTemplate {
    pub start_pfn: usize,
    pub end_pfn: usize,
    pub nr_scanned: usize,
    pub nr_taken: usize,
}

pub type MmCompactionIsolateMigratepages = MmCompactionIsolateTemplate;
pub type MmCompactionIsolateFreepages = MmCompactionIsolateTemplate;
pub type MmCompactionFastIsolateFreepages = MmCompactionIsolateTemplate;

#[cfg(feature = "CONFIG_COMPACTION")]
#[repr(C)]
pub struct MmCompactionMigratepages {
    pub nr_migrated: usize,
    pub nr_failed: usize,
}

#[cfg(feature = "CONFIG_COMPACTION")]
#[repr(C)]
pub struct MmCompactionBegin {
    pub zone_start: usize,
    pub migrate_pfn: usize,
    pub free_pfn: usize,
    pub zone_end: usize,
    pub sync: bool,
}

#[cfg(feature = "CONFIG_COMPACTION")]
#[repr(C)]
pub struct MmCompactionEnd {
    pub zone_start: usize,
    pub migrate_pfn: usize,
    pub free_pfn: usize,
    pub zone_end: usize,
    pub sync: bool,
    pub status: i32,
}

#[cfg(feature = "CONFIG_COMPACTION")]
#[repr(C)]
pub struct MmCompactionTryToCompactPages {
    pub order: i32,
    pub gfp_mask: usize,
    pub prio: i32,
}

#[cfg(feature = "CONFIG_COMPACTION")]
#[repr(C)]
pub struct MmCompactionSuitableTemplate {
    pub nid: i32,
    // enum zone_type
    pub idx: i32,
    pub order: i32,
    pub ret: i32,
}

pub type MmCompactionFinished = MmCompactionSuitableTemplate;
pub type MmCompactionSuitable = MmCompactionSuitableTemplate;

#[cfg(feature = "CONFIG_COMPACTION")]
#[repr(C)]
pub struct MmCompactionDeferTemplate {
    pub nid: i32,
    // enum zone_type
    pub idx: i32,
    pub order: i32,
    pub considered: u32,
    pub defer_shift: u32,
    pub order_failed: i32,
}

pub type MmCompactionDeferred = MmCompactionDeferTemplate;
pub type MmCompactionDeferCompaction = MmCompactionDeferTemplate;
pub type MmCompactionDeferReset = MmCompactionDeferTemplate;

#[cfg(feature = "CONFIG_COMPACTION")]
#[repr(C)]
pub struct MmCompactionKcompactdSleep {
    pub nid: i32,
}

#[cfg(feature = "CONFIG_COMPACTION")]
#[repr(C)]
pub struct KcompactdWakeTemplate {
    pub nid: i32,
    pub order: i32,
    // enum zone_type; printed with the historical name classzone_idx.
    pub highest_zoneidx: i32,
}

pub type MmCompactionWakeupKcompactd = KcompactdWakeTemplate;
pub type MmCompactionKcompactdWake = KcompactdWakeTemplate;

#[inline]
pub fn mm_compaction_migratepages_assign(
    entry: &mut MmCompactionMigratepages,
    nr_migratepages: u32,
    nr_succeeded: u32,
) {
    entry.nr_migrated = nr_succeeded as usize;
    entry.nr_failed = nr_migratepages.wrapping_sub(nr_succeeded) as usize;
}

#[inline]
pub fn mm_compaction_defer_limit(defer_shift: u32) -> usize {
    1usize.wrapping_shl(defer_shift)
}

// TP_printk formats retained from the source:
// isolate: "range=(0x%lx ~ 0x%lx) nr_scanned=%lu nr_taken=%lu"
// migratepages: "nr_migrated=%lu nr_failed=%lu"
// begin: "zone_start=0x%lx migrate_pfn=0x%lx free_pfn=0x%lx zone_end=0x%lx, mode=%s"
// end: "zone_start=0x%lx migrate_pfn=0x%lx free_pfn=0x%lx zone_end=0x%lx, mode=%s status=%s"
// try_to_compact_pages: "order=%d gfp_mask=%s priority=%d"
// suitable: "node=%d zone=%-8s order=%d ret=%s"
// defer: "node=%d zone=%-8s order=%d order_failed=%d consider=%u limit=%lu"
// kcompactd_sleep: "nid=%d"
// kcompactd_wake: "nid=%d order=%d classzone_idx=%-8s"

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
