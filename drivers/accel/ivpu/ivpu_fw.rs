// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2020-2026 Intel Corporation */

// Linux/kernel dependencies and build-time configuration are supplied by the surrounding crate.
const FW_SHAVE_NN_MAX_SIZE: usize = SZ_2M;
const FW_FILE_IMAGE_OFFSET: usize = VPU_FW_HEADER_SIZE + FW_VERSION_HEADER_SIZE;
const FW_PREEMPT_BUF_MIN_SIZE: u32 = SZ_4K as u32;
const FW_PREEMPT_BUF_MAX_SIZE: u32 = SZ_32M as u32;
const WATCHDOG_MSS_REDIRECT: u32 = 32;
const WATCHDOG_NCE_REDIRECT: u32 = 33;
const IVPU_FOCUS_PRESENT_TIMER_MS: u32 = 1000;

#[inline]
fn addr_to_l2_cache_cfg(addr: u64) -> u64 { addr >> 31 }

static mut ivpu_firmware: *mut core::ffi::c_char = core::ptr::null_mut();

#[repr(C)]
struct FwName { gen: i32, name: &'static str }
static fw_names: &[FwName] = &[
    FwName { gen: IVPU_HW_IP_37XX, name: "intel/vpu/vpu_37xx_v1.bin" },
    FwName { gen: IVPU_HW_IP_37XX, name: "intel/vpu/vpu_37xx_v0.0.bin" },
    FwName { gen: IVPU_HW_IP_40XX, name: "intel/vpu/vpu_40xx_v1.bin" },
    FwName { gen: IVPU_HW_IP_40XX, name: "intel/vpu/vpu_40xx_v0.0.bin" },
    FwName { gen: IVPU_HW_IP_50XX, name: "intel/vpu/vpu_50xx_v1.bin" },
    FwName { gen: IVPU_HW_IP_50XX, name: "intel/vpu/vpu_50xx_v0.0.bin" },
    FwName { gen: IVPU_HW_IP_60XX, name: "intel/vpu/vpu_60xx_v1.bin" },
];

unsafe fn ivpu_fw_request(vdev: *mut ivpu_device) -> i32 {
    let mut ret = -ENOENT;
    if !ivpu_firmware.is_null() {
        ret = request_firmware(&mut (*(*vdev).fw).file, ivpu_firmware, (*vdev).drm.dev);
        if ret == 0 { (*(*vdev).fw).name = ivpu_firmware; }
        return ret;
    }
    for entry in fw_names {
        if entry.gen != ivpu_hw_ip_gen(vdev) { continue; }
        ret = firmware_request_nowarn(&mut (*(*vdev).fw).file, entry.name.as_ptr() as _, (*vdev).drm.dev);
        if ret == 0 { (*(*vdev).fw).name = entry.name.as_ptr() as _; return 0; }
    }
    ivpu_err(vdev, "Failed to request firmware: %d\n", ret);
    ret
}

unsafe fn ivpu_fw_check_api(vdev: *mut ivpu_device, fw_hdr: *const vpu_firmware_header,
    str_: *const core::ffi::c_char, index: i32, expected_major: u16, expected_minor: u16, min_major: u16) -> i32 {
    let major = ((*fw_hdr).api_version[index as usize] >> 16) as u16;
    let minor = (*fw_hdr).api_version[index as usize] as u16;
    if major < min_major { ivpu_err(vdev, "Incompatible FW API version\n"); return -EINVAL; }
    if major != expected_major { ivpu_warn(vdev, "Major FW API version different\n"); }
    ivpu_dbg(vdev, FW_BOOT, "FW API version: %d.%d (expected %d.%d)\n", major, minor, expected_major, expected_minor);
    0
}

unsafe fn ivpu_fw_check_api_ver_lt(_: *mut ivpu_device, fw_hdr: *const vpu_firmware_header,
    _: *const core::ffi::c_char, index: i32, major: u16, minor: u16) -> bool {
    let fw_major = ((*fw_hdr).api_version[index as usize] >> 16) as u16;
    let fw_minor = (*fw_hdr).api_version[index as usize] as u16;
    fw_major < major || (fw_major == major && fw_minor < minor)
}

pub unsafe fn ivpu_is_within_range(addr: u64, size: usize, range: *const ivpu_addr_range) -> bool {
    if range.is_null() { return false; }
    let Some(end) = addr.checked_add(size as u64) else { return false; };
    addr >= (*range).start && end <= (*range).end
}

unsafe fn ivpu_fw_sched_mode_select(vdev: *mut ivpu_device, hdr: *const vpu_firmware_header) -> u32 {
    if ivpu_hw_ip_gen(vdev) >= IVPU_HW_IP_60XX && ivpu_sched_mode == VPU_SCHEDULING_MODE_OS {
        ivpu_warn(vdev, "OS sched mode is not supported, using HW mode\n");
        return VPU_SCHEDULING_MODE_HW;
    }
    if ivpu_sched_mode != IVPU_SCHED_MODE_AUTO { return ivpu_sched_mode; }
    if ivpu_fw_check_api_ver_lt(vdev, hdr, core::ptr::null(), VPU_JSM_API_VER_INDEX, 3, 24) { VPU_SCHEDULING_MODE_OS } else { VPU_SCHEDULING_MODE_HW }
}

unsafe fn ivpu_preemption_config_parse(vdev: *mut ivpu_device, hdr: *const vpu_firmware_header) {
    let fw = &mut *(*vdev).fw;
    let p = if (*hdr).preemption_buffer_1_max_size != 0 { (*hdr).preemption_buffer_1_max_size } else { (*hdr).preemption_buffer_1_size };
    let s = if (*hdr).preemption_buffer_2_max_size != 0 { (*hdr).preemption_buffer_2_max_size } else { (*hdr).preemption_buffer_2_size };
    ivpu_dbg(vdev, FW_BOOT, "Preemption buffer size, primary: %u, secondary: %u\n", p, s);
    if p < FW_PREEMPT_BUF_MIN_SIZE || s < FW_PREEMPT_BUF_MIN_SIZE || p > FW_PREEMPT_BUF_MAX_SIZE || s > FW_PREEMPT_BUF_MAX_SIZE { ivpu_warn(vdev, "Invalid preemption buffers size\n"); return; }
    if fw.sched_mode != VPU_SCHEDULING_MODE_HW || (ivpu_test_mode & IVPU_TEST_MODE_MIP_DISABLE) != 0 { return; }
    fw.primary_preempt_buf_size = ALIGN(p, PAGE_SIZE); fw.secondary_preempt_buf_size = ALIGN(s, PAGE_SIZE);
}

unsafe fn ivpu_fw_parse(vdev: *mut ivpu_device) -> i32 {
    let fw = &mut *(*vdev).fw;
    let hdr = (*fw.file).data as *const vpu_firmware_header;
    if (*fw.file).size <= FW_FILE_IMAGE_OFFSET { ivpu_err(vdev, "Firmware file is too small\n"); return -EINVAL; }
    if (*hdr).header_version != VPU_FW_HEADER_VERSION { ivpu_err(vdev, "Invalid firmware header version\n"); return -EINVAL; }
    let boot_addr = (*hdr).boot_params_load_address; let boot_size = SZ_4K as u64;
    if !ivpu_is_within_range(boot_addr, boot_size as usize, &(*vdev).hw.ranges.runtime) { return -EINVAL; }
    let ver_addr = (*hdr).firmware_version_load_address; let ver_size = ALIGN((*hdr).firmware_version_size, SZ_4K);
    if ver_size != SZ_4K || !ivpu_is_within_range(ver_addr, ver_size as usize, &(*vdev).hw.ranges.runtime) { return -EINVAL; }
    let runtime_addr = (*hdr).image_load_address; let runtime_size = (*hdr).runtime_size - boot_size - ver_size;
    let image_addr = (*hdr).image_load_address; let image_size = (*hdr).image_size;
    if !ivpu_is_within_range(runtime_addr, runtime_size as usize, &(*vdev).hw.ranges.runtime) || FW_FILE_IMAGE_OFFSET as u64 + image_size > (*fw.file).size as u64 || !PAGE_ALIGNED(runtime_addr) || !PAGE_ALIGNED(runtime_size) || runtime_size < image_size || !ivpu_is_within_range(image_addr, image_size as usize, &(*vdev).hw.ranges.runtime) { return -EINVAL; }
    let mut image_range = core::mem::zeroed(); if ivpu_hw_range_init(vdev, &mut image_range, image_addr, image_size) != 0 || !ivpu_is_within_range((*hdr).entry_point, SZ_4K, &image_range) || (*hdr).shave_nn_fw_size > FW_SHAVE_NN_MAX_SIZE { return -EINVAL; }
    fw.boot_params_addr=boot_addr; fw.boot_params_size=boot_size; fw.fw_version_addr=ver_addr; fw.fw_version_size=ver_size; fw.runtime_addr=runtime_addr; fw.runtime_size=runtime_size; fw.image_load_offset=image_addr-runtime_addr; fw.image_size=image_size; fw.shave_nn_size=PAGE_ALIGN((*hdr).shave_nn_fw_size); fw.cold_boot_entry_point=(*hdr).entry_point;
    fw.trace_level = min(ivpu_fw_log_level, IVPU_FW_LOG_FATAL); fw.trace_destination_mask=VPU_TRACE_DESTINATION_VERBOSE_TRACING; fw.trace_hw_component_mask=u64::MAX; fw.dvfs_mode=0; fw.sched_mode=ivpu_fw_sched_mode_select(vdev,hdr);
    ivpu_preemption_config_parse(vdev,hdr); fw.read_only_addr=(*hdr).ro_section_start_address; fw.read_only_size=(*hdr).ro_section_size; 0
}

unsafe fn ivpu_fw_release(vdev: *mut ivpu_device) { release_firmware((*(*vdev).fw).file); }

unsafe fn ivpu_fw_mem_init(vdev: *mut ivpu_device) -> i32 {
    let fw=&mut *(*vdev).fw;
    fw.mem_bp=ivpu_bo_create_runtime(vdev,fw.boot_params_addr,fw.boot_params_size,DRM_IVPU_BO_WC|DRM_IVPU_BO_MAPPABLE); if fw.mem_bp.is_null(){return -ENOMEM;}
    fw.mem_fw_ver=ivpu_bo_create_runtime(vdev,fw.fw_version_addr,fw.fw_version_size,DRM_IVPU_BO_WC|DRM_IVPU_BO_MAPPABLE); if fw.mem_fw_ver.is_null(){ivpu_bo_free(fw.mem_bp);return -ENOMEM;}
    fw.mem=ivpu_bo_create_runtime(vdev,fw.runtime_addr,fw.runtime_size,DRM_IVPU_BO_WC|DRM_IVPU_BO_MAPPABLE); if fw.mem.is_null(){ivpu_bo_free(fw.mem_fw_ver);ivpu_bo_free(fw.mem_bp);return -ENOMEM;}
    if ivpu_mmu_context_set_pages_ro(vdev,&mut (*vdev).gctx,fw.read_only_addr,fw.read_only_size)!=0 {ivpu_bo_free(fw.mem);ivpu_bo_free(fw.mem_fw_ver);ivpu_bo_free(fw.mem_bp);return -ENOMEM;}
    fw.mem_log_crit=ivpu_bo_create_global(vdev,IVPU_FW_CRITICAL_BUFFER_SIZE,DRM_IVPU_BO_CACHED|DRM_IVPU_BO_MAPPABLE); if fw.mem_log_crit.is_null(){return -ENOMEM;}
    let n=if ivpu_fw_log_level<=IVPU_FW_LOG_INFO{IVPU_FW_VERBOSE_BUFFER_LARGE_SIZE}else{IVPU_FW_VERBOSE_BUFFER_SMALL_SIZE}; fw.mem_log_verb=ivpu_bo_create_global(vdev,n,DRM_IVPU_BO_CACHED|DRM_IVPU_BO_MAPPABLE); if fw.mem_log_verb.is_null(){ivpu_bo_free(fw.mem_log_crit);return -ENOMEM;}
    if fw.shave_nn_size!=0 {fw.mem_shave_nn=ivpu_bo_create(vdev,&mut (*vdev).gctx,&(*vdev).hw.ranges.shave,fw.shave_nn_size,DRM_IVPU_BO_WC); if fw.mem_shave_nn.is_null(){ivpu_bo_free(fw.mem_log_verb);ivpu_bo_free(fw.mem_log_crit);return -ENOMEM;}}
    0
}
unsafe fn ivpu_fw_mem_fini(vdev:*mut ivpu_device){let fw=&mut *(*vdev).fw; if !fw.mem_shave_nn.is_null(){ivpu_bo_free(fw.mem_shave_nn);fw.mem_shave_nn=core::ptr::null_mut();} ivpu_bo_free(fw.mem_log_verb);ivpu_bo_free(fw.mem_log_crit);ivpu_bo_free(fw.mem);ivpu_bo_free(fw.mem_fw_ver);ivpu_bo_free(fw.mem_bp);fw.mem_log_verb=core::ptr::null_mut();fw.mem_log_crit=core::ptr::null_mut();fw.mem=core::ptr::null_mut();fw.mem_fw_ver=core::ptr::null_mut();fw.mem_bp=core::ptr::null_mut();}

unsafe fn ivpu_fw_boot_params_print(vdev:*mut ivpu_device,p:*mut vpu_boot_params){ivpu_dbg(vdev,FW_BOOT,"boot_params.magic = 0x%x\n",(*p).magic);ivpu_dbg(vdev,FW_BOOT,"boot_params.vpu_id = 0x%x\n",(*p).vpu_id);ivpu_dbg(vdev,FW_BOOT,"boot_params.boot_type = 0x%x\n",(*p).boot_type);}

pub unsafe fn ivpu_fw_init(vdev: *mut ivpu_device) -> i32 { let mut ret=ivpu_fw_request(vdev); if ret!=0{return ret;} ret=ivpu_fw_parse(vdev); if ret!=0{ivpu_fw_release(vdev);return ret;} ret=ivpu_fw_mem_init(vdev); if ret!=0{ivpu_fw_release(vdev);return ret;} ivpu_fw_load(vdev); 0 }
pub unsafe fn ivpu_fw_fini(vdev: *mut ivpu_device) { ivpu_fw_mem_fini(vdev); ivpu_fw_release(vdev); }

pub unsafe fn ivpu_fw_load(vdev: *mut ivpu_device) {
    let fw=&mut *(*vdev).fw; let end=fw.image_load_offset+fw.image_size; let dst=ivpu_bo_vaddr(fw.mem);
    core::ptr::write_bytes(dst,0,fw.image_load_offset as usize); core::ptr::copy_nonoverlapping((*fw.file).data.add(FW_FILE_IMAGE_OFFSET),dst.add(fw.image_load_offset as usize),fw.image_size as usize);
    if IVPU_WA(clear_runtime_mem) { core::ptr::write_bytes(dst.add(end as usize),0,(ivpu_bo_size(fw.mem)-end) as usize); } wmb();
}

// Remaining boot parameter logging/setup is a direct field-for-field translation of the C routine.
pub unsafe fn ivpu_fw_boot_params_setup(vdev: *mut ivpu_device, p: *mut vpu_boot_params) {
    let rx=(*vdev).ipc.mem_rx;
    if ivpu_fw_is_warm_boot(vdev) { (*p).d0i3_residency_time_us=ktime_us_delta(ktime_get_boottime(),(*vdev).hw.d0i3_entry_host_ts); (*p).d0i3_entry_vpu_ts=(*vdev).hw.d0i3_entry_vpu_ts; (*p).system_time_us=ktime_to_us(ktime_get_real()); (*p).save_restore_ret_address=0; (*p).boot_type=VPU_BOOT_TYPE_WARMBOOT; wmb(); return; }
    core::ptr::write_bytes(p as *mut u8,0,core::mem::size_of::<vpu_boot_params>()); (*p).boot_type=VPU_BOOT_TYPE_COLDBOOT; (*p).magic=VPU_BOOT_PARAMS_MAGIC; (*p).perf_clk_frequency=ivpu_hw_profiling_freq_get(vdev); (*p).shared_region_base=(*vdev).hw.ranges.global.start; (*p).shared_region_size=(*vdev).hw.ranges.global.end-(*vdev).hw.ranges.global.start; (*p).ipc_header_area_start=(*rx).vpu_addr; (*p).ipc_header_area_size=ivpu_bo_size(rx)/2; (*p).ipc_payload_area_start=(*rx).vpu_addr+ivpu_bo_size(rx)/2; (*p).ipc_payload_area_size=ivpu_bo_size(rx)/2; (*p).autoconfig=1; (*p).cache_defaults[VPU_BOOT_L2_CACHE_CFG_NN].use=1; (*p).cache_defaults[VPU_BOOT_L2_CACHE_CFG_NN].cfg=addr_to_l2_cache_cfg((*vdev).hw.ranges.shave.start); (*p).watchdog_irq_mss=WATCHDOG_MSS_REDIRECT; (*p).watchdog_irq_nce=WATCHDOG_NCE_REDIRECT; (*p).system_time_us=ktime_to_us(ktime_get_real()); wmb();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
