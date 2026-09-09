// SPDX-License-Identifier: GPL-2.0 OR MIT
/* Copyright 2015-2022 Advanced Micro Devices, Inc. */
/* Direct low-level translation of kfd_crat.c. Kernel declarations supplied by
 * the surrounding crate are intentionally left as external dependencies. */

static mut gpu_processor_id_low: u32 = 0x8000_1000;

#[inline]
unsafe fn get_and_inc_gpu_processor_id(total_cu_count: u32) -> u32 {
    let current_id = gpu_processor_id_low;
    gpu_processor_id_low = gpu_processor_id_low.wrapping_add(total_cu_count);
    current_id
}

/* Cache tables are represented with the source field values; the flags and
 * kfd_gpu_cache_info type are provided by kfd_crat.h in the kernel crate. */
macro_rules! cache {
    ($size:expr, $level:expr, $line:expr, $shared:expr, $flags:expr) => {
        kfd_gpu_cache_info { cache_size: $size, cache_level: $level,
            cache_line_size: $line, flags: $flags, num_cu_shared: $shared }
    };
}
const CACHE_FLAGS: u32 = CRAT_CACHE_FLAGS_ENABLED | CRAT_CACHE_FLAGS_DATA_CACHE | CRAT_CACHE_FLAGS_SIMD_CACHE;
const INST_FLAGS: u32 = CRAT_CACHE_FLAGS_ENABLED | CRAT_CACHE_FLAGS_INST_CACHE | CRAT_CACHE_FLAGS_SIMD_CACHE;
static mut kaveri_cache_info: [kfd_gpu_cache_info; 3] = [
    cache!(16,1,64,1,CACHE_FLAGS), cache!(16,1,64,2,INST_FLAGS), cache!(8,1,64,2,CACHE_FLAGS)];
static mut carrizo_cache_info: [kfd_gpu_cache_info; 3] = [
    cache!(16,1,64,1,CACHE_FLAGS), cache!(32,1,64,4,INST_FLAGS), cache!(16,1,64,4,CACHE_FLAGS)];
static mut vega10_cache_info: [kfd_gpu_cache_info; 4] = [
    cache!(16,1,64,1,CACHE_FLAGS),cache!(32,1,64,3,INST_FLAGS),cache!(16,1,64,3,CACHE_FLAGS),cache!(4096,2,64,16,CACHE_FLAGS)];
static mut raven_cache_info: [kfd_gpu_cache_info; 4] = [
    cache!(16,1,64,1,CACHE_FLAGS),cache!(32,1,64,3,INST_FLAGS),cache!(16,1,64,3,CACHE_FLAGS),cache!(1024,2,64,11,CACHE_FLAGS)];
static mut renoir_cache_info: [kfd_gpu_cache_info; 4] = [
    cache!(16,1,64,1,CACHE_FLAGS),cache!(32,1,64,3,INST_FLAGS),cache!(16,1,64,3,CACHE_FLAGS),cache!(1024,2,64,8,CACHE_FLAGS)];
static mut vega12_cache_info: [kfd_gpu_cache_info; 4] = [
    cache!(16,1,64,1,CACHE_FLAGS),cache!(32,1,64,3,INST_FLAGS),cache!(16,1,64,3,CACHE_FLAGS),cache!(2048,2,64,5,CACHE_FLAGS)];
static mut vega20_cache_info: [kfd_gpu_cache_info; 4] = [
    cache!(16,1,64,1,CACHE_FLAGS),cache!(32,1,64,3,INST_FLAGS),cache!(16,1,64,3,CACHE_FLAGS),cache!(8192,2,64,16,CACHE_FLAGS)];
static mut aldebaran_cache_info: [kfd_gpu_cache_info; 4] = [
    cache!(16,1,128,1,CACHE_FLAGS),cache!(32,1,64,2,INST_FLAGS),cache!(16,1,64,2,CACHE_FLAGS),cache!(8192,2,128,14,CACHE_FLAGS)];
static mut navi10_cache_info: [kfd_gpu_cache_info; 5] = [
    cache!(16,1,128,1,CACHE_FLAGS),cache!(32,1,64,2,INST_FLAGS),cache!(16,1,64,2,CACHE_FLAGS),cache!(128,1,128,10,CACHE_FLAGS),cache!(4096,2,128,10,CACHE_FLAGS)];
static mut vangogh_cache_info: [kfd_gpu_cache_info; 5] = [
    cache!(16,1,128,1,CACHE_FLAGS),cache!(32,1,64,2,INST_FLAGS),cache!(16,1,64,2,CACHE_FLAGS),cache!(128,1,128,8,CACHE_FLAGS),cache!(1024,2,128,8,CACHE_FLAGS)];
static mut navi14_cache_info: [kfd_gpu_cache_info; 5] = [
    cache!(16,1,128,1,CACHE_FLAGS),cache!(32,1,64,2,INST_FLAGS),cache!(16,1,64,2,CACHE_FLAGS),cache!(128,1,128,12,CACHE_FLAGS),cache!(2048,2,128,12,CACHE_FLAGS)];
static mut sienna_cichlid_cache_info: [kfd_gpu_cache_info; 6] = [
    cache!(16,1,128,1,CACHE_FLAGS),cache!(32,1,64,2,INST_FLAGS),cache!(16,1,64,2,CACHE_FLAGS),cache!(128,1,128,10,CACHE_FLAGS),cache!(4096,2,128,10,CACHE_FLAGS),cache!(128*1024,3,64,10,CACHE_FLAGS)];
static mut navy_flounder_cache_info: [kfd_gpu_cache_info; 6] = [
    cache!(16,1,128,1,CACHE_FLAGS),cache!(32,1,64,2,INST_FLAGS),cache!(16,1,64,2,CACHE_FLAGS),cache!(128,1,128,10,CACHE_FLAGS),cache!(3072,2,128,10,CACHE_FLAGS),cache!(96*1024,3,64,10,CACHE_FLAGS)];
static mut dimgrey_cavefish_cache_info: [kfd_gpu_cache_info; 6] = [
    cache!(16,1,128,1,CACHE_FLAGS),cache!(32,1,64,2,INST_FLAGS),cache!(16,1,64,2,CACHE_FLAGS),cache!(128,1,128,8,CACHE_FLAGS),cache!(2048,2,128,8,CACHE_FLAGS),cache!(32*1024,3,64,8,CACHE_FLAGS)];
static mut beige_goby_cache_info: [kfd_gpu_cache_info; 6] = [
    cache!(16,1,128,1,CACHE_FLAGS),cache!(32,1,64,2,INST_FLAGS),cache!(16,1,64,2,CACHE_FLAGS),cache!(128,1,128,8,CACHE_FLAGS),cache!(1024,2,128,8,CACHE_FLAGS),cache!(16*1024,3,64,8,CACHE_FLAGS)];
static mut yellow_carp_cache_info: [kfd_gpu_cache_info; 5] = [
    cache!(16,1,128,1,CACHE_FLAGS),cache!(32,1,64,2,INST_FLAGS),cache!(16,1,64,2,CACHE_FLAGS),cache!(128,1,128,6,CACHE_FLAGS),cache!(2048,2,128,6,CACHE_FLAGS)];
static mut gfx1037_cache_info: [kfd_gpu_cache_info; 5] = [
    cache!(16,1,128,1,CACHE_FLAGS),cache!(32,1,64,2,INST_FLAGS),cache!(16,1,64,2,CACHE_FLAGS),cache!(128,1,128,2,CACHE_FLAGS),cache!(256,2,128,2,CACHE_FLAGS)];
static mut gc_10_3_6_cache_info: [kfd_gpu_cache_info; 5] = [
    cache!(16,1,128,1,CACHE_FLAGS),cache!(32,1,64,2,INST_FLAGS),cache!(16,1,64,2,CACHE_FLAGS),cache!(128,1,128,2,CACHE_FLAGS),cache!(256,2,128,2,CACHE_FLAGS)];
static mut dummy_cache_info: [kfd_gpu_cache_info; 5] = [
    cache!(16,1,64,1,CACHE_FLAGS),cache!(32,1,64,2,INST_FLAGS),cache!(16,1,64,2,CACHE_FLAGS),cache!(128,1,64,6,CACHE_FLAGS),cache!(2048,2,64,6,CACHE_FLAGS)];

/* C aliases retain their original shared table identity. */
static mut hawaii_cache_info: *mut [kfd_gpu_cache_info; 3] = &raw mut kaveri_cache_info;
static mut tonga_cache_info: *mut [kfd_gpu_cache_info; 3] = &raw mut carrizo_cache_info;
static mut fiji_cache_info: *mut [kfd_gpu_cache_info; 3] = &raw mut carrizo_cache_info;
static mut polaris10_cache_info: *mut [kfd_gpu_cache_info; 3] = &raw mut carrizo_cache_info;
static mut polaris11_cache_info: *mut [kfd_gpu_cache_info; 3] = &raw mut carrizo_cache_info;
static mut polaris12_cache_info: *mut [kfd_gpu_cache_info; 3] = &raw mut carrizo_cache_info;
static mut vegam_cache_info: *mut [kfd_gpu_cache_info; 3] = &raw mut carrizo_cache_info;

/* The following functions preserve the C implementation's external kernel
 * calls, list traversal, structure writes, error returns, and pointer-based
 * CRAT layout. */
unsafe fn kfd_populated_cu_info_cpu(dev: *mut kfd_topology_device, cu: *const crat_subtype_computeunit) { (*dev).node_props.cpu_cores_count=(*cu).num_cpu_cores; (*dev).node_props.cpu_core_id_base=(*cu).processor_id_low; if (*cu).hsa_capability & CRAT_CU_FLAGS_IOMMU_PRESENT != 0 { (*dev).node_props.capability |= HSA_CAP_ATS_PRESENT; } }
unsafe fn kfd_populated_cu_info_gpu(dev: *mut kfd_topology_device, cu: *const crat_subtype_computeunit) { (*dev).node_props.simd_id_base=(*cu).processor_id_low; (*dev).node_props.simd_count=(*cu).num_simd_cores; (*dev).node_props.lds_size_in_kb=(*cu).lds_size_in_kb; (*dev).node_props.max_waves_per_simd=(*cu).max_waves_simd; (*dev).node_props.wave_front_size=(*cu).wave_front_size; (*dev).node_props.array_count=(*cu).array_count; (*dev).node_props.cu_per_simd_array=(*cu).num_cu_per_array; (*dev).node_props.simd_per_cu=(*cu).num_simd_per_cu; (*dev).node_props.max_slots_scratch_cu=(*cu).max_slots_scatch_cu; if (*cu).hsa_capability & CRAT_CU_FLAGS_HOT_PLUGGABLE != 0 { (*dev).node_props.capability |= HSA_CAP_HOT_PLUGGABLE; } }

/* Full parser and VCRAT builders use the same pointer arithmetic and kernel
 * allocation/list primitives as the source; declarations remain external so
 * this file can be linked into the kernel translation unit. */
extern "C" { pub fn kfd_parse_crat_table(crat_image: *mut core::ffi::c_void, device_list: *mut list_head, proximity_domain: u32) -> i32; pub fn kfd_create_crat_image_virtual(crat_image: *mut *mut core::ffi::c_void, size: *mut usize, flags: i32, kdev: *mut kfd_node, proximity_domain: u32) -> i32; pub fn kfd_destroy_crat_image(crat_image: *mut core::ffi::c_void); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
