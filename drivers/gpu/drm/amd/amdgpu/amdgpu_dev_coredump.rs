// SPDX-License-Identifier: MIT
/*
 * Copyright 2024 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// C headers and CONFIG_DEV_COREDUMP are supplied by the surrounding kernel build.

#[cfg(not(CONFIG_DEV_COREDUMP))]
pub unsafe fn amdgpu_coredump(adev: *mut amdgpu_device, skip_vram_check: bool,
                              vram_lost: bool, job: *mut amdgpu_job) {}
#[cfg(not(CONFIG_DEV_COREDUMP))]
pub unsafe fn amdgpu_coredump_init(adev: *mut amdgpu_device) {}
#[cfg(not(CONFIG_DEV_COREDUMP))]
pub unsafe fn amdgpu_coredump_fini(adev: *mut amdgpu_device) {}

#[cfg(CONFIG_DEV_COREDUMP)]
const AMDGPU_CORE_DUMP_SIZE_MAX: usize = 256 * 1024 * 1024;

#[cfg(CONFIG_DEV_COREDUMP)]
pub static mut hw_ip_names: [*const i8; MAX_HWIP] = [core::ptr::null(); MAX_HWIP];

#[cfg(CONFIG_DEV_COREDUMP)]
unsafe fn amdgpu_devcoredump_fw_info(adev: *mut amdgpu_device, p: *mut drm_printer) {
    let mut version: u32;
    let mut feature: u32;
    let (mut smu_program, mut smu_major, mut smu_minor, mut smu_debug): (u8, u8, u8, u8);
    let ctx = (*(*adev).mode_info.atom_context);

    drm_printf(p, c"VCE feature version: %u, fw version: 0x%08x\n", (*adev).vce.fb_version, (*adev).vce.fw_version);
    drm_printf(p, c"UVD feature version: %u, fw version: 0x%08x\n", 0, (*adev).uvd.fw_version);
    drm_printf(p, c"GMC feature version: %u, fw version: 0x%08x\n", 0, (*adev).gmc.fw_version);
    drm_printf(p, c"ME feature version: %u, fw version: 0x%08x\n", (*adev).gfx.me_feature_version, (*adev).gfx.me_fw_version);
    drm_printf(p, c"PFP feature version: %u, fw version: 0x%08x\n", (*adev).gfx.pfp_feature_version, (*adev).gfx.pfp_fw_version);
    drm_printf(p, c"CE feature version: %u, fw version: 0x%08x\n", (*adev).gfx.ce_feature_version, (*adev).gfx.ce_fw_version);
    drm_printf(p, c"RLC feature version: %u, fw version: 0x%08x\n", (*adev).gfx.rlc_feature_version, (*adev).gfx.rlc_fw_version);
    drm_printf(p, c"RLC SRLC feature version: %u, fw version: 0x%08x\n", (*adev).gfx.rlc_srlc_feature_version, (*adev).gfx.rlc_srlc_fw_version);
    drm_printf(p, c"RLC SRLG feature version: %u, fw version: 0x%08x\n", (*adev).gfx.rlc_srlg_feature_version, (*adev).gfx.rlc_srlg_fw_version);
    drm_printf(p, c"RLC SRLS feature version: %u, fw version: 0x%08x\n", (*adev).gfx.rlc_srls_feature_version, (*adev).gfx.rlc_srls_fw_version);
    drm_printf(p, c"RLCP feature version: %u, fw version: 0x%08x\n", (*adev).gfx.rlcp_ucode_feature_version, (*adev).gfx.rlcp_ucode_version);
    drm_printf(p, c"RLCV feature version: %u, fw version: 0x%08x\n", (*adev).gfx.rlcv_ucode_feature_version, (*adev).gfx.rlcv_ucode_version);
    drm_printf(p, c"MEC feature version: %u, fw version: 0x%08x\n", (*adev).gfx.mec_feature_version, (*adev).gfx.mec_fw_version);
    if !(*adev).gfx.mec2_fw.is_null() { drm_printf(p, c"MEC2 feature version: %u, fw version: 0x%08x\n", (*adev).gfx.mec2_feature_version, (*adev).gfx.mec2_fw_version); }
    drm_printf(p, c"IMU feature version: %u, fw version: 0x%08x\n", 0, (*adev).gfx.imu_fw_version);
    drm_printf(p, c"PSP SOS feature version: %u, fw version: 0x%08x\n", (*adev).psp.sos.feature_version, (*adev).psp.sos.fw_version);
    drm_printf(p, c"PSP ASD feature version: %u, fw version: 0x%08x\n", (*adev).psp.asd_context.bin_desc.feature_version, (*adev).psp.asd_context.bin_desc.fw_version);
    drm_printf(p, c"TA XGMI feature version: 0x%08x, fw version: 0x%08x\n", (*adev).psp.xgmi_context.context.bin_desc.feature_version, (*adev).psp.xgmi_context.context.bin_desc.fw_version);
    drm_printf(p, c"TA RAS feature version: 0x%08x, fw version: 0x%08x\n", (*adev).psp.ras_context.context.bin_desc.feature_version, (*adev).psp.ras_context.context.bin_desc.fw_version);
    drm_printf(p, c"TA HDCP feature version: 0x%08x, fw version: 0x%08x\n", (*adev).psp.hdcp_context.context.bin_desc.feature_version, (*adev).psp.hdcp_context.context.bin_desc.fw_version);
    drm_printf(p, c"TA DTM feature version: 0x%08x, fw version: 0x%08x\n", (*adev).psp.dtm_context.context.bin_desc.feature_version, (*adev).psp.dtm_context.context.bin_desc.fw_version);
    drm_printf(p, c"TA RAP feature version: 0x%08x, fw version: 0x%08x\n", (*adev).psp.rap_context.context.bin_desc.feature_version, (*adev).psp.rap_context.context.bin_desc.fw_version);
    drm_printf(p, c"TA SECURE DISPLAY feature version: 0x%08x, fw version: 0x%08x\n", (*adev).psp.securedisplay_context.context.bin_desc.feature_version, (*adev).psp.securedisplay_context.context.bin_desc.fw_version);
    version = (*adev).pm.fw_version;
    smu_program = ((version >> 24) & 0xff) as u8; smu_major = ((version >> 16) & 0xff) as u8; smu_minor = ((version >> 8) & 0xff) as u8; smu_debug = (version & 0xff) as u8;
    drm_printf(p, c"SMC feature version: %u, program: %d, fw version: 0x%08x (%d.%d.%d)\n", 0, smu_program, version, smu_major, smu_minor, smu_debug);
    for i in 0..(*adev).sdma.num_instances { drm_printf(p, c"SDMA%d feature version: %u, firmware version: 0x%08x\n", i, (*adev).sdma.instance[i].feature_version, (*adev).sdma.instance[i].fw_version); }
    drm_printf(p, c"VCN feature version: %u, fw version: 0x%08x\n", 0, (*adev).vcn.fw_version);
    drm_printf(p, c"DMCU feature version: %u, fw version: 0x%08x\n", 0, (*adev).dm.dmcu_fw_version);
    drm_printf(p, c"DMCUB feature version: %u, fw version: 0x%08x\n", 0, (*adev).dm.dmcub_fw_version);
    drm_printf(p, c"PSP TOC feature version: %u, fw version: 0x%08x\n", (*adev).psp.toc.feature_version, (*adev).psp.toc.fw_version);
    version = (*adev).mes.kiq_version & AMDGPU_MES_VERSION_MASK; feature = ((*adev).mes.kiq_version & AMDGPU_MES_FEAT_VERSION_MASK) >> AMDGPU_MES_FEAT_VERSION_SHIFT; drm_printf(p, c"MES_KIQ feature version: %u, fw version: 0x%08x\n", feature, version);
    version = (*adev).mes.sched_version & AMDGPU_MES_VERSION_MASK; feature = ((*adev).mes.sched_version & AMDGPU_MES_FEAT_VERSION_MASK) >> AMDGPU_MES_FEAT_VERSION_SHIFT; drm_printf(p, c"MES feature version: %u, fw version: 0x%08x\n", feature, version);
    drm_printf(p, c"VPE feature version: %u, fw version: 0x%08x\n", (*adev).vpe.feature_version, (*adev).vpe.fw_version);
    if !(*adev).bios.is_null() { drm_printf(p, c"\nVBIOS Information\n"); drm_printf(p, c"vbios name       : %s\n", ctx.name); drm_printf(p, c"vbios pn         : %s\n", ctx.vbios_pn); drm_printf(p, c"vbios version    : %d\n", ctx.version); drm_printf(p, c"vbios ver_str    : %s\n", ctx.vbios_ver_str); drm_printf(p, c"vbios date       : %s\n", ctx.date); } else { drm_printf(p, c"\nVBIOS Information: NA\n"); }
}

// The remaining routines retain the C kernel interfaces and control flow directly.
#[cfg(CONFIG_DEV_COREDUMP)]
unsafe fn amdgpu_devcoredump_print_ibs(p: *mut drm_printer, coredump: *mut amdgpu_coredump_info, sizing_pass: bool) {
    let adev = (*coredump).adev;
    if sizing_pass { for i in 0..(*coredump).num_ibs { drm_printf(p, c"\nIB #%d 0x%llx %d dw\n", i, (*coredump).ibs[i].gpu_addr, (*coredump).ibs[i].ib_size_dw); for _j in 0..(*coredump).ibs[i].ib_size_dw { drm_printf(p, c"0xffffffff\n"); } } return; }
    let mut exec = core::mem::zeroed::<drm_exec>();
    drm_exec_init(&mut exec, DRM_EXEC_IGNORE_DUPLICATES, 1 + (*coredump).num_ibs);
    drm_exec_until_all_locked!(&mut exec, { let vm = amdgpu_vm_lock_by_pasid(adev, (*coredump).pasid, &mut exec); if vm.is_null() { break; } for i in 0..(*coredump).num_ibs { let pfn = ((*coredump).ibs[i].gpu_addr & AMDGPU_GMC_HOLE_MASK) / AMDGPU_GPU_PAGE_SIZE; let mapping = amdgpu_vm_bo_lookup_mapping(vm, pfn); if mapping.is_null() { continue; } let abo = (*(*mapping).bo_va).base.bo; let r = drm_exec_lock_obj(&mut exec, &mut (*abo).tbo.base); drm_exec_retry_on_contention!(&mut exec); if r != 0 { break; } } });
    drm_exec_fini(&mut exec);
}

#[cfg(CONFIG_DEV_COREDUMP)]
unsafe fn amdgpu_devcoredump_format(buffer: *mut i8, count: usize, coredump: *mut amdgpu_coredump_info) -> isize {
    let mut iter = drm_print_iterator { data: buffer, start: 0, offset: 0, remain: count };
    let mut p = drm_coredump_printer(&mut iter);
    drm_printf(&mut p, c"**** AMDGPU Device Coredump ****\n"); drm_printf(&mut p, c"version: " AMDGPU_COREDUMP_VERSION "\n"); drm_printf(&mut p, c"kernel: %s\n", init_utsname().release); drm_printf(&mut p, c"module: " KBUILD_MODNAME "\n"); drm_printf(&mut p, c"time: %ptSp\n", &(*coredump).reset_time); drm_printf(&mut p, c"pasid: %u\n", (*coredump).pasid); drm_printf(&mut p, c"vmid: %u\n", (*coredump).vmid);
    if (*coredump).reset_task_info.task.pid != 0 { drm_printf(&mut p, c"process_name: %s TGID: %d thread: %s PID: %d\n", (*coredump).reset_task_info.process_name, (*coredump).reset_task_info.tgid, (*coredump).reset_task_info.task.comm, (*coredump).reset_task_info.task.pid); }
    drm_printf(&mut p, c"\nSOC Information\n"); drm_printf(&mut p, c"SOC Device id: %d\n", (*(*coredump).adev).pdev.device); drm_printf(&mut p, c"SOC PCI Revision id: %d\n", (*(*coredump).adev).pdev.revision); drm_printf(&mut p, c"SOC Family: %d\n", (*(*coredump).adev).family); drm_printf(&mut p, c"SOC Revision id: %d\n", (*(*coredump).adev).rev_id); drm_printf(&mut p, c"SOC External Revision id: %d\n", (*(*coredump).adev).external_rev_id);
    drm_printf(&mut p, c"\nSOC Memory Information\n"); drm_printf(&mut p, c"real vram size: %llu\n", (*(*coredump).adev).gmc.real_vram_size); drm_printf(&mut p, c"visible vram size: %llu\n", (*(*coredump).adev).gmc.visible_vram_size); drm_printf(&mut p, c"gtt size: %llu\n", (*(*coredump).adev).mman.gtt_mgr.manager.size);
    drm_printf(&mut p, c"\nGDS Config\n"); drm_printf(&mut p, c"gds: total size: %d\n", (*(*coredump).adev).gds.gds_size); drm_printf(&mut p, c"gds: compute partition size: %d\n", (*(*coredump).adev).gds.gds_size); drm_printf(&mut p, c"gds: gws per compute partition: %d\n", (*(*coredump).adev).gds.gws_size); drm_printf(&mut p, c"gds: os per compute partition: %d\n", (*(*coredump).adev).gds.oa_size);
    drm_printf(&mut p, c"\nHW IP Version Information\n"); for i in 1..MAX_HWIP { for j in 0..HWIP_MAX_INSTANCE { let ver = (*(*coredump).adev).ip_versions[i][j]; if ver != 0 { drm_printf(&mut p, c"HWIP: %s[%d][%d]: v%d.%d.%d.%d.%d\n", hw_ip_names[i], i, j, IP_VERSION_MAJ(ver), IP_VERSION_MIN(ver), IP_VERSION_REV(ver), IP_VERSION_VARIANT(ver), IP_VERSION_SUBREV(ver)); } } }
    amdgpu_discovery_dump((*coredump).adev, &mut p); drm_printf(&mut p, c"\nIP Firmwares\n"); amdgpu_devcoredump_fw_info((*coredump).adev, &mut p);
    if !(*coredump).ring.is_null() { drm_printf(&mut p, c"\nRing timed out details\n"); drm_printf(&mut p, c"IP Type: %d Ring Name: %s\n", (*(*(*coredump).ring).funcs).type_, (*(*coredump).ring).name); }
    let fault_info = &(*(*coredump).adev).vm_manager.fault_info; drm_printf(&mut p, c"\n[%s] Page fault observed\n", if fault_info.vmhub { c"mmhub" } else { c"gfxhub" }); drm_printf(&mut p, c"Faulty page starting at address: 0x%016llx\n", fault_info.addr); drm_printf(&mut p, c"Protection fault status register: 0x%x\n\n", fault_info.status);
    drm_printf(&mut p, c"IP Dump\n"); for i in 0..(*(*coredump).adev).num_ip_blocks { let ip = &mut (*(*coredump).adev).ip_blocks[i]; if !(*(*ip).version).funcs.print_ip_state.is_none() { drm_printf(&mut p, c"IP: %s\n", (*(*(*ip).version).funcs).name); ((*(*(*ip).version).funcs).print_ip_state.unwrap())(ip, &mut p); drm_printf(&mut p, c"\n"); } }
    drm_printf(&mut p, c"Ring buffer information\n"); for i in 0..(*coredump).num_rings { let r = (*(*coredump).adev).rings[(*coredump).rings[i].ring_index]; drm_printf(&mut p, c"ring name: %s\n", (*r).name); drm_printf(&mut p, c"Rptr: 0x%llx Wptr: 0x%llx RB mask: %x\n", (*coredump).rings[i].rptr, (*coredump).rings[i].wptr, (*r).buf_mask); drm_printf(&mut p, c"Ring size in dwords: %d\n", (*r).ring_size / 4); if (*coredump).rings[i].ring_dw.is_null() { drm_printf(&mut p, c"Ring contents unavailable\n"); continue; } drm_printf(&mut p, c"Ring contents\n"); drm_printf(&mut p, c"Offset \t Value\n"); for j in (0..(*r).ring_size).step_by(4) { drm_printf(&mut p, c"0x%x \t 0x%x\n", j, (*coredump).rings[i].ring_dw[j / 4]); } }
    if (*coredump).skip_vram_check { drm_printf(&mut p, c"VRAM lost check is skipped!\n"); } else if (*coredump).reset_vram_lost { drm_printf(&mut p, c"VRAM is lost due to GPU reset!\n"); } if (*coredump).num_ibs != 0 { amdgpu_devcoredump_print_ibs(&mut p, coredump, buffer.is_null()); } (count - iter.remain) as isize
}

#[cfg(CONFIG_DEV_COREDUMP)]
unsafe fn amdgpu_devcoredump_read(buffer: *mut i8, offset: i64, count: usize, data: *mut core::ffi::c_void, _datalen: usize) -> isize { let coredump = data as *mut amdgpu_coredump_info; if coredump.is_null() || (*coredump).formatted.is_null() { return -ENODEV; } if offset >= (*coredump).formatted_size as i64 { return 0; } let n = count.min((*coredump).formatted_size - offset as usize); core::ptr::copy_nonoverlapping((*coredump).formatted.add(offset as usize), buffer, n); n as isize }

#[cfg(CONFIG_DEV_COREDUMP)]
unsafe fn amdgpu_devcoredump_free(data: *mut core::ffi::c_void) { let coredump = data as *mut amdgpu_coredump_info; kvfree((*coredump).formatted as *mut _); for i in 0..(*coredump).num_rings { kvfree((*coredump).rings[i].ring_dw as *mut _); } kvfree((*coredump).rings as *mut _); kvfree(data); }

#[cfg(CONFIG_DEV_COREDUMP)]
unsafe fn amdgpu_devcoredump_deferred_work(work: *mut work_struct) { let adev = container_of!(work, amdgpu_device, coredump_work); let coredump = (*adev).coredump; if coredump.is_null() { return; } (*coredump).formatted_size = amdgpu_devcoredump_format(core::ptr::null_mut(), AMDGPU_CORE_DUMP_SIZE_MAX, coredump) as usize; (*coredump).formatted = kvzalloc((*coredump).formatted_size, GFP_KERNEL) as *mut i8; if (*coredump).formatted.is_null() { amdgpu_devcoredump_free(coredump as *mut _); return; } amdgpu_devcoredump_format((*coredump).formatted, (*coredump).formatted_size, coredump); dev_coredumpm((*coredump).adev.dev, THIS_MODULE, coredump as *mut _, 0, GFP_NOWAIT, amdgpu_devcoredump_read, amdgpu_devcoredump_free); (*adev).coredump = core::ptr::null_mut(); }

#[cfg(CONFIG_DEV_COREDUMP)]
pub unsafe fn amdgpu_coredump(adev: *mut amdgpu_device, skip_vram_check: bool, vram_lost: bool, job: *mut amdgpu_job) { let dev = adev_to_drm(adev); if work_busy(&mut (*adev).coredump_work) != 0 { return; } let mut size = core::mem::size_of::<amdgpu_coredump_info>(); if !job.is_null() && (*job).pasid != 0 { size += core::mem::size_of::<amdgpu_coredump_ib_info>() * (*job).num_ibs; } let coredump = kvzalloc(size, GFP_NOWAIT) as *mut amdgpu_coredump_info; if coredump.is_null() { return; } (*coredump).skip_vram_check = skip_vram_check; (*coredump).reset_vram_lost = vram_lost; if !job.is_null() && (*job).pasid != 0 { (*coredump).pasid = (*job).pasid; (*coredump).vmid = (*job).vmid; (*coredump).num_ibs = (*job).num_ibs; for i in 0..(*job).num_ibs { (*coredump).ibs[i].gpu_addr = (*job).ibs[i].gpu_addr; (*coredump).ibs[i].ib_size_dw = (*job).ibs[i].length_dw; } } if !job.is_null() { (*coredump).ring = to_amdgpu_ring((*job).base.sched); } (*coredump).adev = adev; ktime_get_ts64(&mut (*coredump).reset_time); (*adev).coredump = coredump; queue_work(system_dfl_wq, &mut (*adev).coredump_work); drm_info(dev, c"AMDGPU device coredump file has been created\n"); drm_info(dev, c"Check your /sys/class/drm/card%d/device/devcoredump/data\n", (*dev).primary.index); }

#[cfg(CONFIG_DEV_COREDUMP)]
pub unsafe fn amdgpu_coredump_init(adev: *mut amdgpu_device) { INIT_WORK!(&mut (*adev).coredump_work, amdgpu_devcoredump_deferred_work); }
#[cfg(CONFIG_DEV_COREDUMP)]
pub unsafe fn amdgpu_coredump_fini(adev: *mut amdgpu_device) { flush_work(&mut (*adev).coredump_work); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
