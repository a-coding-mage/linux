/* SPDX-License-Identifier: GPL-2.0 */

/* I/O Translation Anchor (IOTA) */
#[repr(C)]
pub enum zpci_ioat_dtype {
    ZPCI_IOTA_STO = 0,
    ZPCI_IOTA_RTTO = 1,
    ZPCI_IOTA_RSTO = 2,
    ZPCI_IOTA_RFTO = 3,
    ZPCI_IOTA_PFAA = 4,
    ZPCI_IOTA_IOPFAA = 5,
    ZPCI_IOTA_IOPTO = 7,
}

pub const ZPCI_IOTA_IOT_ENABLED: usize = 0x800;
pub const ZPCI_IOTA_DT_ST: usize = (ZPCI_IOTA_STO as usize) << 2;
pub const ZPCI_IOTA_DT_RT: usize = (ZPCI_IOTA_RTTO as usize) << 2;
pub const ZPCI_IOTA_DT_RS: usize = (ZPCI_IOTA_RSTO as usize) << 2;
pub const ZPCI_IOTA_DT_RF: usize = (ZPCI_IOTA_RFTO as usize) << 2;
pub const ZPCI_IOTA_DT_PF: usize = (ZPCI_IOTA_PFAA as usize) << 2;
pub const ZPCI_IOTA_FS_4K: usize = 0;
pub const ZPCI_IOTA_FS_1M: usize = 1;
pub const ZPCI_IOTA_FS_2G: usize = 2;
pub const ZPCI_KEY: usize = (PAGE_DEFAULT_KEY as usize) << 5;

pub const ZPCI_TABLE_SIZE_RT: usize = 1usize << 42;
pub const ZPCI_TABLE_SIZE_RS: usize = 1usize << 53;

pub const ZPCI_IOTA_STO_FLAG: usize = ZPCI_IOTA_IOT_ENABLED | ZPCI_KEY | ZPCI_IOTA_DT_ST;
pub const ZPCI_IOTA_RTTO_FLAG: usize = ZPCI_IOTA_IOT_ENABLED | ZPCI_KEY | ZPCI_IOTA_DT_RT;
pub const ZPCI_IOTA_RSTO_FLAG: usize = ZPCI_IOTA_IOT_ENABLED | ZPCI_KEY | ZPCI_IOTA_DT_RS;
pub const ZPCI_IOTA_RFTO_FLAG: usize = ZPCI_IOTA_IOT_ENABLED | ZPCI_KEY | ZPCI_IOTA_DT_RF;
pub const ZPCI_IOTA_RFAA_FLAG: usize = ZPCI_IOTA_IOT_ENABLED | ZPCI_KEY | ZPCI_IOTA_DT_PF | ZPCI_IOTA_FS_2G;

/* I/O Region and segment tables */
pub const ZPCI_INDEX_MASK: usize = 0x7ff;
pub const ZPCI_TABLE_TYPE_MASK: usize = 0xc;
pub const ZPCI_TABLE_TYPE_RFX: usize = 0xc;
pub const ZPCI_TABLE_TYPE_RSX: usize = 0x8;
pub const ZPCI_TABLE_TYPE_RTX: usize = 0x4;
pub const ZPCI_TABLE_TYPE_SX: usize = 0x0;
pub const ZPCI_TABLE_LEN_RFX: usize = 0x3;
pub const ZPCI_TABLE_LEN_RSX: usize = 0x3;
pub const ZPCI_TABLE_LEN_RTX: usize = 0x3;
pub const ZPCI_TABLE_OFFSET_MASK: usize = 0xc0;
pub const ZPCI_TABLE_SIZE: usize = 0x4000;
pub const ZPCI_TABLE_ALIGN: usize = ZPCI_TABLE_SIZE;
pub const ZPCI_TABLE_ENTRY_SIZE: usize = core::mem::size_of::<usize>();
pub const ZPCI_TABLE_ENTRIES: usize = ZPCI_TABLE_SIZE / ZPCI_TABLE_ENTRY_SIZE;
pub const ZPCI_TABLE_BITS: usize = 11;
pub const ZPCI_PT_BITS: usize = 8;
pub const ZPCI_ST_SHIFT: usize = ZPCI_PT_BITS + PAGE_SHIFT as usize;
pub const ZPCI_RT_SHIFT: usize = ZPCI_ST_SHIFT + ZPCI_TABLE_BITS;
pub const ZPCI_RS_SHIFT: usize = ZPCI_RT_SHIFT + ZPCI_TABLE_BITS;
pub const ZPCI_RF_SHIFT: usize = ZPCI_RS_SHIFT + ZPCI_TABLE_BITS;
pub const ZPCI_RTE_FLAG_MASK: usize = 0x3fff;
pub const ZPCI_RTE_ADDR_MASK: usize = !ZPCI_RTE_FLAG_MASK;
pub const ZPCI_STE_FLAG_MASK: usize = 0x7ff;
pub const ZPCI_STE_ADDR_MASK: usize = !ZPCI_STE_FLAG_MASK;

/* I/O Page tables */
pub const ZPCI_PTE_VALID_MASK: usize = 0x400;
pub const ZPCI_PTE_INVALID: usize = 0x400;
pub const ZPCI_PTE_VALID: usize = 0x000;
pub const ZPCI_PT_SIZE: usize = 0x800;
pub const ZPCI_PT_ALIGN: usize = ZPCI_PT_SIZE;
pub const ZPCI_PT_ENTRIES: usize = ZPCI_PT_SIZE / ZPCI_TABLE_ENTRY_SIZE;
pub const ZPCI_PT_MASK: usize = ZPCI_PT_ENTRIES - 1;
pub const ZPCI_PTE_FLAG_MASK: usize = 0xfff;
pub const ZPCI_PTE_ADDR_MASK: usize = !ZPCI_PTE_FLAG_MASK;

/* Shared bits */
pub const ZPCI_TABLE_VALID: usize = 0x00;
pub const ZPCI_TABLE_INVALID: usize = 0x20;
pub const ZPCI_TABLE_PROTECTED: usize = 0x200;
pub const ZPCI_TABLE_UNPROTECTED: usize = 0x000;
pub const ZPCI_TABLE_VALID_MASK: usize = 0x20;
pub const ZPCI_TABLE_PROT_MASK: usize = 0x200;

#[repr(C)]
pub struct zpci_iommu_ctrs {
    pub mapped_pages: atomic64_t,
    pub unmapped_pages: atomic64_t,
    pub global_rpcits: atomic64_t,
    pub sync_map_rpcits: atomic64_t,
    pub sync_rpcits: atomic64_t,
}

#[repr(C)]
pub struct zpci_dev {
    _private: [u8; 0],
}

extern "C" {
    pub fn zpci_get_iommu_ctrs(zdev: *mut zpci_dev) -> *mut zpci_iommu_ctrs;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
