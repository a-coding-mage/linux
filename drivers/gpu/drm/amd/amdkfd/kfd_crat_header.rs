/* SPDX-License-Identifier: GPL-2.0 OR MIT */
/*
 * Copyright 2014-2022 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// C header dependency: <linux/types.h>

/* 4CC signature value for the CRAT ACPI table */
pub const CRAT_SIGNATURE: &[u8] = b"CRAT";

/* Component Resource Association Table (CRAT) */
pub const CRAT_OEMID_LENGTH: usize = 6;
pub const CRAT_OEMTABLEID_LENGTH: usize = 8;
pub const CRAT_RESERVED_LENGTH: usize = 6;

/* Compute Unit flags */
pub const COMPUTE_UNIT_CPU: u32 = 1 << 0; /* Create Virtual CRAT for CPU */
pub const COMPUTE_UNIT_GPU: u32 = 1 << 1; /* Create Virtual CRAT for GPU */

#[repr(C, packed)]
pub struct crat_header {
    pub signature: u32, pub length: u32, pub revision: u8, pub checksum: u8,
    pub oem_id: [u8; CRAT_OEMID_LENGTH], pub oem_table_id: [u8; CRAT_OEMTABLEID_LENGTH],
    pub oem_revision: u32, pub creator_id: u32, pub creator_revision: u32,
    pub total_entries: u32, pub num_domains: u16, pub reserved: [u8; CRAT_RESERVED_LENGTH],
}

/* The header structure is immediately followed by total_entries of the data definitions */
/* The currently defined subtype entries in the CRAT */
pub const CRAT_SUBTYPE_COMPUTEUNIT_AFFINITY: u32 = 0;
pub const CRAT_SUBTYPE_MEMORY_AFFINITY: u32 = 1;
pub const CRAT_SUBTYPE_CACHE_AFFINITY: u32 = 2;
pub const CRAT_SUBTYPE_TLB_AFFINITY: u32 = 3;
pub const CRAT_SUBTYPE_CCOMPUTE_AFFINITY: u32 = 4;
pub const CRAT_SUBTYPE_IOLINK_AFFINITY: u32 = 5;
pub const CRAT_SUBTYPE_MAX: u32 = 6;

/* Do not change the value of CRAT_SIBLINGMAP_SIZE from 32 as it breaks the ABI. */
pub const CRAT_SIBLINGMAP_SIZE: usize = 32;

pub const CRAT_CU_FLAGS_ENABLED: u32 = 0x00000001;
pub const CRAT_CU_FLAGS_HOT_PLUGGABLE: u32 = 0x00000002;
pub const CRAT_CU_FLAGS_CPU_PRESENT: u32 = 0x00000004;
pub const CRAT_CU_FLAGS_GPU_PRESENT: u32 = 0x00000008;
pub const CRAT_CU_FLAGS_IOMMU_PRESENT: u32 = 0x00000010;
pub const CRAT_CU_FLAGS_RESERVED: u32 = 0xffffffe0;
pub const CRAT_COMPUTEUNIT_RESERVED_LENGTH: usize = 4;

#[repr(C, packed)]
pub struct crat_subtype_computeunit {
    pub type_: u8, pub length: u8, pub reserved: u16, pub flags: u32, pub proximity_domain: u32,
    pub processor_id_low: u32, pub num_cpu_cores: u16, pub num_simd_cores: u16,
    pub max_waves_simd: u16, pub io_count: u16, pub hsa_capability: u16, pub lds_size_in_kb: u16,
    pub wave_front_size: u8, pub num_banks: u8, pub micro_engine_id: u16, pub array_count: u8,
    pub num_cu_per_array: u8, pub num_simd_per_cu: u8, pub max_slots_scatch_cu: u8,
    pub reserved2: [u8; CRAT_COMPUTEUNIT_RESERVED_LENGTH],
}

pub const CRAT_MEM_FLAGS_ENABLED: u32 = 0x00000001;
pub const CRAT_MEM_FLAGS_HOT_PLUGGABLE: u32 = 0x00000002;
pub const CRAT_MEM_FLAGS_NON_VOLATILE: u32 = 0x00000004;
pub const CRAT_MEM_FLAGS_RESERVED: u32 = 0xfffffff8;
pub const CRAT_MEMORY_RESERVED_LENGTH: usize = 8;
#[repr(C, packed)]
pub struct crat_subtype_memory {
    pub type_: u8, pub length: u8, pub reserved: u16, pub flags: u32, pub proximity_domain: u32,
    pub base_addr_low: u32, pub base_addr_high: u32, pub length_low: u32, pub length_high: u32,
    pub width: u32, pub visibility_type: u8, /* for virtual (dGPU) CRAT */
    pub reserved2: [u8; CRAT_MEMORY_RESERVED_LENGTH - 1],
}

pub const CRAT_CACHE_FLAGS_ENABLED: u32 = 0x00000001;
pub const CRAT_CACHE_FLAGS_DATA_CACHE: u32 = 0x00000002;
pub const CRAT_CACHE_FLAGS_INST_CACHE: u32 = 0x00000004;
pub const CRAT_CACHE_FLAGS_CPU_CACHE: u32 = 0x00000008;
pub const CRAT_CACHE_FLAGS_SIMD_CACHE: u32 = 0x00000010;
pub const CRAT_CACHE_FLAGS_RESERVED: u32 = 0xffffffe0;
pub const CRAT_CACHE_RESERVED_LENGTH: usize = 8;
#[repr(C, packed)]
pub struct crat_subtype_cache {
    pub type_: u8, pub length: u8, pub reserved: u16, pub flags: u32, pub processor_id_low: u32,
    pub sibling_map: [u8; CRAT_SIBLINGMAP_SIZE], pub cache_size: u32, pub cache_level: u8,
    pub lines_per_tag: u8, pub cache_line_size: u16, pub associativity: u8, pub cache_properties: u8,
    pub cache_latency: u16, pub reserved2: [u8; CRAT_CACHE_RESERVED_LENGTH],
}

pub const CRAT_TLB_FLAGS_ENABLED: u32 = 0x00000001;
pub const CRAT_TLB_FLAGS_DATA_TLB: u32 = 0x00000002;
pub const CRAT_TLB_FLAGS_INST_TLB: u32 = 0x00000004;
pub const CRAT_TLB_FLAGS_CPU_TLB: u32 = 0x00000008;
pub const CRAT_TLB_FLAGS_SIMD_TLB: u32 = 0x00000010;
pub const CRAT_TLB_FLAGS_RESERVED: u32 = 0xffffffe0;
pub const CRAT_TLB_RESERVED_LENGTH: usize = 4;
#[repr(C, packed)]
pub struct crat_subtype_tlb {
    pub type_: u8, pub length: u8, pub reserved: u16, pub flags: u32, pub processor_id_low: u32,
    pub sibling_map: [u8; CRAT_SIBLINGMAP_SIZE], pub tlb_level: u32,
    pub data_tlb_associativity_2mb: u8, pub data_tlb_size_2mb: u8,
    pub instruction_tlb_associativity_2mb: u8, pub instruction_tlb_size_2mb: u8,
    pub data_tlb_associativity_4k: u8, pub data_tlb_size_4k: u8,
    pub instruction_tlb_associativity_4k: u8, pub instruction_tlb_size_4k: u8,
    pub data_tlb_associativity_1gb: u8, pub data_tlb_size_1gb: u8,
    pub instruction_tlb_associativity_1gb: u8, pub instruction_tlb_size_1gb: u8,
    pub reserved2: [u8; CRAT_TLB_RESERVED_LENGTH],
}

pub const CRAT_CCOMPUTE_FLAGS_ENABLED: u32 = 0x00000001;
pub const CRAT_CCOMPUTE_FLAGS_RESERVED: u32 = 0xfffffffe;
pub const CRAT_CCOMPUTE_RESERVED_LENGTH: usize = 16;
#[repr(C, packed)]
pub struct crat_subtype_ccompute {
    pub type_: u8, pub length: u8, pub reserved: u16, pub flags: u32, pub processor_id_low: u32,
    pub sibling_map: [u8; CRAT_SIBLINGMAP_SIZE], pub apu_size: u32,
    pub reserved2: [u8; CRAT_CCOMPUTE_RESERVED_LENGTH],
}

pub const CRAT_IOLINK_FLAGS_ENABLED: u32 = 1 << 0;
pub const CRAT_IOLINK_FLAGS_NON_COHERENT: u32 = 1 << 1;
pub const CRAT_IOLINK_FLAGS_NO_ATOMICS_32_BIT: u32 = 1 << 2;
pub const CRAT_IOLINK_FLAGS_NO_ATOMICS_64_BIT: u32 = 1 << 3;
pub const CRAT_IOLINK_FLAGS_NO_PEER_TO_PEER_DMA: u32 = 1 << 4;
pub const CRAT_IOLINK_FLAGS_BI_DIRECTIONAL: u32 = 1 << 31;
pub const CRAT_IOLINK_FLAGS_RESERVED_MASK: u32 = 0x7fffffe0;

pub const CRAT_IOLINK_TYPE_UNDEFINED: u8 = 0;
pub const CRAT_IOLINK_TYPE_HYPERTRANSPORT: u8 = 1;
pub const CRAT_IOLINK_TYPE_PCIEXPRESS: u8 = 2;
pub const CRAT_IOLINK_TYPE_AMBA: u8 = 3;
pub const CRAT_IOLINK_TYPE_MIPI: u8 = 4;
pub const CRAT_IOLINK_TYPE_QPI_1_1: u8 = 5;
pub const CRAT_IOLINK_TYPE_RESERVED1: u8 = 6;
pub const CRAT_IOLINK_TYPE_RESERVED2: u8 = 7;
pub const CRAT_IOLINK_TYPE_RAPID_IO: u8 = 8;
pub const CRAT_IOLINK_TYPE_INFINIBAND: u8 = 9;
pub const CRAT_IOLINK_TYPE_RESERVED3: u8 = 10;
pub const CRAT_IOLINK_TYPE_XGMI: u8 = 11;
pub const CRAT_IOLINK_TYPE_XGOP: u8 = 12;
pub const CRAT_IOLINK_TYPE_GZ: u8 = 13;
pub const CRAT_IOLINK_TYPE_ETHERNET_RDMA: u8 = 14;
pub const CRAT_IOLINK_TYPE_RDMA_OTHER: u8 = 15;
pub const CRAT_IOLINK_TYPE_OTHER: u8 = 16;
pub const CRAT_IOLINK_TYPE_MAX: u8 = 255;
pub const CRAT_IOLINK_RESERVED_LENGTH: usize = 24;
#[repr(C, packed)]
pub struct crat_subtype_iolink {
    pub type_: u8, pub length: u8, pub reserved: u16, pub flags: u32,
    pub proximity_domain_from: u32, pub proximity_domain_to: u32, pub io_interface_type: u8,
    pub version_major: u8, pub version_minor: u16, pub minimum_latency: u32,
    pub maximum_latency: u32, pub minimum_bandwidth_mbs: u32, pub maximum_bandwidth_mbs: u32,
    pub recommended_transfer_size: u32, pub reserved2: [u8; CRAT_IOLINK_RESERVED_LENGTH - 1],
    pub weight_xgmi: u8,
}

/* HSA generic sub-type header */
pub const CRAT_SUBTYPE_FLAGS_ENABLED: u32 = 0x00000001;
#[repr(C, packed)]
pub struct crat_subtype_generic { pub type_: u8, pub length: u8, pub reserved: u16, pub flags: u32 }

#[repr(C)]
pub struct kfd_node { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { _private: [u8; 0] }

/* Static table to describe GPU Cache information */
#[repr(C)]
pub struct kfd_gpu_cache_info {
    pub cache_size: u32, pub cache_level: u32, pub cache_line_size: u32, pub flags: u32,
    /* Indicates how many Compute Units share this cache within a SA. Value = 1 indicates the cache is not shared */
    pub num_cu_shared: u32,
}

unsafe extern "C" {
    pub fn kfd_get_gpu_cache_info(kdev: *mut kfd_node, pcache_info: *mut *mut kfd_gpu_cache_info) -> i32;
    pub fn kfd_destroy_crat_image(crat_image: *mut core::ffi::c_void);
    pub fn kfd_parse_crat_table(crat_image: *mut core::ffi::c_void, device_list: *mut list_head, proximity_domain: u32) -> i32;
    pub fn kfd_create_crat_image_virtual(crat_image: *mut *mut core::ffi::c_void, size: *mut usize, flags: i32, kdev: *mut kfd_node, proximity_domain: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
