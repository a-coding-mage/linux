/* SPDX-License-Identifier: GPL-2.0 */

// Translation of trace/events/thp.h.
// The Linux tracepoint and trace-definition headers are external dependencies.

#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HugepageSetEntry {
    pub addr: ::core::ffi::c_ulong,
    pub pte: ::core::ffi::c_ulong,
}

#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[inline]
pub unsafe fn hugepage_set_fast_assign(
    entry: *mut HugepageSetEntry,
    addr: ::core::ffi::c_ulong,
    pte: ::core::ffi::c_ulong,
) {
    (*entry).addr = addr;
    (*entry).pte = pte;
}

#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
pub const HUGEPage_SET_PRINTK: &str =
    "Set page table entry with 0x%lx with 0x%lx";

#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[inline]
pub unsafe fn hugepage_set_pmd(
    entry: *mut HugepageSetEntry,
    addr: ::core::ffi::c_ulong,
    pmd: ::core::ffi::c_ulong,
) {
    hugepage_set_fast_assign(entry, addr, pmd);
}

#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[inline]
pub unsafe fn hugepage_set_pud(
    entry: *mut HugepageSetEntry,
    addr: ::core::ffi::c_ulong,
    pud: ::core::ffi::c_ulong,
) {
    hugepage_set_fast_assign(entry, addr, pud);
}

#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HugepageUpdateEntry {
    pub addr: ::core::ffi::c_ulong,
    pub pte: ::core::ffi::c_ulong,
    pub clr: ::core::ffi::c_ulong,
    pub set: ::core::ffi::c_ulong,
}

#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[inline]
pub unsafe fn hugepage_update_fast_assign(
    entry: *mut HugepageUpdateEntry,
    addr: ::core::ffi::c_ulong,
    pte: ::core::ffi::c_ulong,
    clr: ::core::ffi::c_ulong,
    set: ::core::ffi::c_ulong,
) {
    (*entry).addr = addr;
    (*entry).pte = pte;
    (*entry).clr = clr;
    (*entry).set = set;
}

#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
pub const HUGEPage_UPDATE_PRINTK: &str =
    "hugepage update at addr 0x%lx and pte = 0x%lx clr = 0x%lx, set = 0x%lx";

#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[inline]
pub unsafe fn hugepage_update_pmd(
    entry: *mut HugepageUpdateEntry,
    addr: ::core::ffi::c_ulong,
    pmd: ::core::ffi::c_ulong,
    clr: ::core::ffi::c_ulong,
    set: ::core::ffi::c_ulong,
) {
    hugepage_update_fast_assign(entry, addr, pmd, clr, set);
}

#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[inline]
pub unsafe fn hugepage_update_pud(
    entry: *mut HugepageUpdateEntry,
    addr: ::core::ffi::c_ulong,
    pud: ::core::ffi::c_ulong,
    clr: ::core::ffi::c_ulong,
    set: ::core::ffi::c_ulong,
) {
    hugepage_update_fast_assign(entry, addr, pud, clr, set);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MigrationPmdEntry {
    pub addr: ::core::ffi::c_ulong,
    pub pmd: ::core::ffi::c_ulong,
}

#[inline]
pub unsafe fn migration_pmd_fast_assign(
    entry: *mut MigrationPmdEntry,
    addr: ::core::ffi::c_ulong,
    pmd: ::core::ffi::c_ulong,
) {
    (*entry).addr = addr;
    (*entry).pmd = pmd;
}

pub const MIGRATION_PMD_PRINTK: &str = "addr=%lx, pmd=%lx";

#[inline]
pub unsafe fn set_migration_pmd(
    entry: *mut MigrationPmdEntry,
    addr: ::core::ffi::c_ulong,
    pmd: ::core::ffi::c_ulong,
) {
    migration_pmd_fast_assign(entry, addr, pmd);
}

#[inline]
pub unsafe fn remove_migration_pmd(
    entry: *mut MigrationPmdEntry,
    addr: ::core::ffi::c_ulong,
    pmd: ::core::ffi::c_ulong,
) {
    migration_pmd_fast_assign(entry, addr, pmd);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
