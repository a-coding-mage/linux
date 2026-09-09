/* SPDX-License-Identifier: GPL-2.0 */
/* Common header for intel-gtt.ko and i915.ko */

// C dependency: linux/types.h

pub type PhysAddr = u64;
pub type ResourceSize = u64;
pub type DmaAddr = u64;

#[repr(C)]
pub struct AgpBridgeData {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PciDev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SgTable {
    _private: [u8; 0],
}

extern "C" {
    pub fn intel_gmch_gtt_get(
        gtt_total: *mut u64,
        mappable_base: *mut PhysAddr,
        mappable_end: *mut ResourceSize,
    );

    pub fn intel_gmch_probe(
        bridge_pdev: *mut PciDev,
        gpu_pdev: *mut PciDev,
        bridge: *mut AgpBridgeData,
    ) -> i32;
    pub fn intel_gmch_remove();

    pub fn intel_gmch_enable_gtt() -> bool;

    pub fn intel_gmch_gtt_flush();
    pub fn intel_gmch_gtt_insert_page(addr: DmaAddr, pg: u32, flags: u32);
    pub fn intel_gmch_gtt_insert_sg_entries(st: *mut SgTable, pg_start: u32, flags: u32);
    pub fn intel_gmch_gtt_clear_range(first_entry: u32, num_entries: u32);
    pub fn intel_gmch_gtt_read_entry(
        pg: u32,
        is_present: *mut bool,
        is_local: *mut bool,
    ) -> DmaAddr;
}

/* Special gtt memory types */
pub const AGP_DCACHE_MEMORY: u32 = 1;
pub const AGP_PHYS_MEMORY: u32 = 2;

/* flag for GFDT type */
pub const AGP_USER_CACHED_MEMORY_GFDT: u32 = 1 << 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
