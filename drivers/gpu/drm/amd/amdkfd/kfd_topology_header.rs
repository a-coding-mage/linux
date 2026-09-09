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

// Dependencies supplied by the surrounding kernel translation.

pub const KFD_TOPOLOGY_PUBLIC_NAME_SIZE: usize = 32;

pub const HSA_DBG_WATCH_ADDR_MASK_LO_BIT_GFX9: u32 = 6;
pub const HSA_DBG_WATCH_ADDR_MASK_LO_BIT_GFX9_4_3: u32 = 7;
pub const HSA_DBG_WATCH_ADDR_MASK_LO_BIT_GFX10: u32 = 7;
pub const HSA_DBG_WATCH_ADDR_MASK_HI_BIT: u32 = 29 << HSA_DBG_WATCH_ADDR_MASK_HI_BIT_SHIFT;
pub const HSA_DBG_WATCH_ADDR_MASK_HI_BIT_GFX9_4_3: u32 =
    30 << HSA_DBG_WATCH_ADDR_MASK_HI_BIT_SHIFT;

#[repr(C)]
pub struct kfd_node_properties {
    pub hive_id: u64,
    pub cpu_cores_count: u32,
    pub simd_count: u32,
    pub mem_banks_count: u32,
    pub caches_count: u32,
    pub io_links_count: u32,
    pub p2p_links_count: u32,
    pub cpu_core_id_base: u32,
    pub simd_id_base: u32,
    pub capability: u32,
    pub capability2: u32,
    pub debug_prop: u64,
    pub max_waves_per_simd: u32,
    pub lds_size_in_kb: u32,
    pub gds_size_in_kb: u32,
    pub num_gws: u32,
    pub wave_front_size: u32,
    pub array_count: u32,
    pub simd_arrays_per_engine: u32,
    pub cu_per_simd_array: u32,
    pub simd_per_cu: u32,
    pub max_slots_scratch_cu: u32,
    pub engine_id: u32,
    pub gfx_target_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub location_id: u32,
    pub domain: u32,
    pub max_engine_clk_fcompute: u32,
    pub max_engine_clk_ccompute: u32,
    pub drm_render_minor: i32,
    pub num_sdma_engines: u32,
    pub num_sdma_xgmi_engines: u32,
    pub num_sdma_queues_per_engine: u32,
    pub num_cp_queues: u32,
    pub cwsr_size: u32,
    pub ctl_stack_size: u32,
    pub eop_buffer_size: u32,
    pub debug_memory_size: u32,
    pub name: [core::ffi::c_char; KFD_TOPOLOGY_PUBLIC_NAME_SIZE],
}

#[repr(C)]
pub struct kfd_mem_properties {
    pub list: list_head,
    pub heap_type: u32,
    pub size_in_bytes: u64,
    pub flags: u32,
    pub width: u32,
    pub mem_clk_max: u32,
    pub gpu: *mut kfd_node,
    pub kobj: *mut kobject,
    pub attr: attribute,
}

pub const CACHE_SIBLINGMAP_SIZE: usize = 128;

#[repr(C)]
pub struct kfd_cache_properties {
    pub list: list_head,
    pub processor_id_low: u32,
    pub cache_level: u32,
    pub cache_size: u32,
    pub cacheline_size: u32,
    pub cachelines_per_tag: u32,
    pub cache_assoc: u32,
    pub cache_latency: u32,
    pub cache_type: u32,
    pub sibling_map: [u8; CACHE_SIBLINGMAP_SIZE],
    pub gpu: *mut kfd_node,
    pub kobj: *mut kobject,
    pub attr: attribute,
    pub sibling_map_size: u32,
}

#[repr(C)]
pub struct kfd_iolink_properties {
    pub list: list_head,
    pub iolink_type: u32,
    pub ver_maj: u32,
    pub ver_min: u32,
    pub node_from: u32,
    pub node_to: u32,
    pub weight: u32,
    pub min_latency: u32,
    pub max_latency: u32,
    pub min_bandwidth: u32,
    pub max_bandwidth: u32,
    pub rec_transfer_size: u32,
    pub rec_sdma_eng_id_mask: u32,
    pub flags: u32,
    pub gpu: *mut kfd_node,
    pub kobj: *mut kobject,
    pub attr: attribute,
}

#[repr(C)]
pub struct kfd_perf_properties {
    pub list: list_head,
    pub block_name: [core::ffi::c_char; 16],
    pub max_concurrent: u32,
    pub attr_group: *mut attribute_group,
}

#[repr(C)]
pub struct kfd_topology_device {
    pub list: list_head,
    pub gpu_id: u32,
    pub proximity_domain: u32,
    pub node_props: kfd_node_properties,
    pub mem_props: list_head,
    pub cache_props: list_head,
    pub io_link_props: list_head,
    pub p2p_link_props: list_head,
    pub perf_props: list_head,
    pub gpu: *mut kfd_node,
    pub kobj_node: *mut kobject,
    pub kobj_mem: *mut kobject,
    pub kobj_cache: *mut kobject,
    pub kobj_iolink: *mut kobject,
    pub kobj_p2plink: *mut kobject,
    pub kobj_perf: *mut kobject,
    pub attr_gpuid: attribute,
    pub attr_name: attribute,
    pub attr_props: attribute,
    pub oem_id: kfd_topology_device_oem_id,
    pub oem_table_id: [u8; CRAT_OEMTABLEID_LENGTH],
    pub oem_revision: u32,
}

#[repr(C)]
pub union kfd_topology_device_oem_id {
    pub oem_id: [u8; CRAT_OEMID_LENGTH],
    pub oem_id64: u64,
}

#[repr(C)]
pub struct kfd_system_properties {
    pub num_devices: u32, // Number of H-NUMA nodes
    pub generation_count: u32,
    pub platform_oem: u64,
    pub platform_id: u64,
    pub platform_rev: u64,
    pub kobj_topology: *mut kobject,
    pub kobj_nodes: *mut kobject,
    pub attr_genid: attribute,
    pub attr_props: attribute,
}

#[repr(C, packed)]
pub struct dmi_mem_device {
    pub header: dmi_header,
    pub physical_handle: u16,
    pub error_handle: u16,
    pub total_width: u16,
    pub data_width: u16,
    pub size: u16,
    pub form_factor: u8,
    pub device_set: u8,
    pub device_locator: u8,
    pub bank_locator: u8,
    pub memory_type: u8,
    pub type_detail: u16,
    pub speed: u16,
}

extern "C" {
    pub fn kfd_create_topology_device(device_list: *mut list_head) -> *mut kfd_topology_device;
    pub fn kfd_release_topology_device_list(device_list: *mut list_head);

    // Under CONFIG_HSA_AMD this is the external implementation; otherwise the C header
    // provides an empty inline function.
    pub fn kfd_update_svm_support_properties(adev: *mut amdgpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
