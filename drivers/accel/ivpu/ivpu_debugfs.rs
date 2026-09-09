// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020-2026 Intel Corporation
 */

// Translated from ivpu_debugfs.c. External kernel and driver declarations are
// supplied by the surrounding crate.

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[inline]
unsafe fn seq_to_ivpu(s: *mut seq_file) -> *mut ivpu_device {
    let entry = (*s).private as *mut drm_debugfs_entry;
    to_ivpu_device((*entry).dev)
}

unsafe fn bo_list_show(s: *mut seq_file, _v: *mut c_void) -> c_int {
    let p = drm_seq_file_printer(s);
    let vdev = seq_to_ivpu(s);
    ivpu_bo_list(&mut (*vdev).drm, &p);
    0
}

unsafe fn fw_name_show(s: *mut seq_file, _v: *mut c_void) -> c_int {
    let vdev = seq_to_ivpu(s);
    seq_printf(s, "%s\n", (*(*vdev).fw).name);
    0
}

unsafe fn fw_version_show(s: *mut seq_file, _v: *mut c_void) -> c_int {
    let vdev = seq_to_ivpu(s);
    seq_printf(s, "%s\n", (*(*vdev).fw).version);
    0
}

unsafe fn fw_trace_capability_show(s: *mut seq_file, _v: *mut c_void) -> c_int {
    let vdev = seq_to_ivpu(s);
    let mut trace_hw_component_mask: u64 = 0;
    let mut trace_destination_mask: u32 = 0;
    let ret = ivpu_jsm_trace_get_capability(vdev, &mut trace_destination_mask, &mut trace_hw_component_mask);
    if ret == 0 {
        seq_printf(s, "trace_destination_mask:  %#18x\ntrace_hw_component_mask: %#18llx\n", trace_destination_mask, trace_hw_component_mask);
    }
    0
}

unsafe fn fw_trace_config_show(s: *mut seq_file, _v: *mut c_void) -> c_int {
    let vdev = seq_to_ivpu(s);
    // WA: VPU_JSM_MSG_TRACE_GET_CONFIG is not working yet; use values from vdev->fw.
    let fw = &*(*vdev).fw;
    seq_printf(s, "trace_level:             %#18x\ntrace_destination_mask:  %#18x\ntrace_hw_component_mask: %#18llx\n", fw.trace_level, fw.trace_destination_mask, fw.trace_hw_component_mask);
    0
}

unsafe fn last_bootmode_show(s: *mut seq_file, _v: *mut c_void) -> c_int {
    let vdev = seq_to_ivpu(s);
    let name = if (*(*vdev).fw).last_boot_mode == VPU_BOOT_TYPE_WARMBOOT { "warm boot" } else { "cold boot" };
    seq_printf(s, "%s\n", name);
    0
}

unsafe fn reset_counter_show(s: *mut seq_file, _v: *mut c_void) -> c_int { let v = seq_to_ivpu(s); seq_printf(s, "%d\n", atomic_read(&(*(*v).pm).reset_counter)); 0 }
unsafe fn reset_pending_show(s: *mut seq_file, _v: *mut c_void) -> c_int { let v = seq_to_ivpu(s); seq_printf(s, "%d\n", atomic_read(&(*(*v).pm).reset_pending)); 0 }
unsafe fn firewall_irq_counter_show(s: *mut seq_file, _v: *mut c_void) -> c_int { let v = seq_to_ivpu(s); seq_printf(s, "%d\n", atomic_read(&(*(*v).hw).firewall_irq_counter)); 0 }
unsafe fn engine_reset_counter_show(s: *mut seq_file, _v: *mut c_void) -> c_int { let v = seq_to_ivpu(s); seq_printf(s, "%d\n", atomic_read(&(*(*v).pm).engine_reset_counter)); 0 }

static VDEV_DEBUGFS_LIST: [drm_debugfs_info; 10] = [
    drm_debugfs_info { name: b"bo_list\0".as_ptr() as *const c_char, show: Some(bo_list_show), flags: 0 },
    drm_debugfs_info { name: b"fw_name\0".as_ptr() as *const c_char, show: Some(fw_name_show), flags: 0 },
    drm_debugfs_info { name: b"fw_version\0".as_ptr() as *const c_char, show: Some(fw_version_show), flags: 0 },
    drm_debugfs_info { name: b"fw_trace_capability\0".as_ptr() as *const c_char, show: Some(fw_trace_capability_show), flags: 0 },
    drm_debugfs_info { name: b"fw_trace_config\0".as_ptr() as *const c_char, show: Some(fw_trace_config_show), flags: 0 },
    drm_debugfs_info { name: b"last_bootmode\0".as_ptr() as *const c_char, show: Some(last_bootmode_show), flags: 0 },
    drm_debugfs_info { name: b"reset_counter\0".as_ptr() as *const c_char, show: Some(reset_counter_show), flags: 0 },
    drm_debugfs_info { name: b"reset_pending\0".as_ptr() as *const c_char, show: Some(reset_pending_show), flags: 0 },
    drm_debugfs_info { name: b"firewall_irq_counter\0".as_ptr() as *const c_char, show: Some(firewall_irq_counter_show), flags: 0 },
    drm_debugfs_info { name: b"engine_reset_counter\0".as_ptr() as *const c_char, show: Some(engine_reset_counter_show), flags: 0 },
];

unsafe fn dvfs_mode_get(data: *mut c_void, dvfs_mode: *mut u64) -> c_int { let v = data as *mut ivpu_device; *dvfs_mode = (*(*v).fw).dvfs_mode as u64; 0 }
unsafe fn dvfs_mode_set(data: *mut c_void, dvfs_mode: u64) -> c_int { let v = data as *mut ivpu_device; (*(*v).fw).dvfs_mode = dvfs_mode as u32; pci_try_reset_function(to_pci_dev((*v).drm.dev)) }

unsafe fn fw_dyndbg_fops_write(file: *mut file, user_buf: *const c_char, size: usize, _pos: *mut loff_t) -> isize {
    let vdev = (*file).private_data as *mut ivpu_device;
    let mut buffer = [0u8; VPU_DYNDBG_CMD_MAX_LEN as usize];
    if size >= VPU_DYNDBG_CMD_MAX_LEN as usize { return -EINVAL as isize; }
    let ret = strncpy_from_user(buffer.as_mut_ptr() as *mut c_char, user_buf, size);
    if ret < 0 { return ret as isize; }
    ivpu_jsm_dyndbg_control(vdev, buffer.as_mut_ptr() as *mut c_char, size);
    size as isize
}

unsafe fn fw_log_show(s: *mut seq_file, _v: *mut c_void) -> c_int { let v = (*s).private as *mut ivpu_device; let p = drm_seq_file_printer(s); ivpu_fw_log_print(v, true, &p); 0 }
unsafe fn fw_log_fops_open(inode: *mut inode, file: *mut file) -> c_int { single_open(file, Some(fw_log_show), (*inode).i_private) }
unsafe fn fw_log_fops_write(file: *mut file, _user_buf: *const c_char, size: usize, _pos: *mut loff_t) -> isize { let s = (*file).private_data as *mut seq_file; if size == 0 { return -EINVAL as isize; } ivpu_fw_log_mark_read((*s).private as *mut ivpu_device); size as isize }

unsafe fn fw_profiling_freq_fops_write(file: *mut file, user_buf: *const c_char, size: usize, _pos: *mut loff_t) -> isize { let v = (*file).private_data as *mut ivpu_device; let mut enable = false; let mut ret = kstrtobool_from_user(user_buf, size, &mut enable); if ret < 0 { return ret as isize; } ivpu_hw_profiling_freq_drive(v, enable); ret = pci_try_reset_function(to_pci_dev((*v).drm.dev)); if ret != 0 { return ret as isize; } size as isize }

unsafe fn fw_trace_destination_mask_fops_write(file: *mut file, user_buf: *const c_char, size: usize, _pos: *mut loff_t) -> isize { let v = (*file).private_data as *mut ivpu_device; let fw = &mut *(*v).fw; let mut x=0u32; let ret=kstrtou32_from_user(user_buf,size,0,&mut x); if ret<0{return ret as isize;} fw.trace_destination_mask=x; ivpu_jsm_trace_set_config(v,fw.trace_level,x,fw.trace_hw_component_mask); size as isize }
unsafe fn fw_trace_hw_comp_mask_fops_write(file: *mut file, user_buf: *const c_char, size: usize, _pos: *mut loff_t) -> isize { let v = (*file).private_data as *mut ivpu_device; let fw = &mut *(*v).fw; let mut x=0u64; let ret=kstrtou64_from_user(user_buf,size,0,&mut x); if ret<0{return ret as isize;} fw.trace_hw_component_mask=x; ivpu_jsm_trace_set_config(v,fw.trace_level,fw.trace_destination_mask,x); size as isize }
unsafe fn fw_trace_level_fops_write(file: *mut file, user_buf: *const c_char, size: usize, _pos: *mut loff_t) -> isize { let v = (*file).private_data as *mut ivpu_device; let fw = &mut *(*v).fw; let mut x=0u32; let ret=kstrtou32_from_user(user_buf,size,0,&mut x); if ret<0{return ret as isize;} fw.trace_level=x; ivpu_jsm_trace_set_config(v,x,fw.trace_destination_mask,fw.trace_hw_component_mask); size as isize }

unsafe fn ivpu_force_recovery_fn(file: *mut file, _user_buf: *const c_char, size: usize, _pos: *mut loff_t) -> isize { let v=(*file).private_data as *mut ivpu_device; if size==0{return -EINVAL as isize;} let ret=ivpu_rpm_get(v); if ret<0{return ret as isize;} ivpu_pm_trigger_recovery(v,b"debugfs\0".as_ptr() as *const c_char); flush_work(&mut (*(*v).pm).recovery_work); ivpu_rpm_put(v); size as isize }
unsafe fn ivpu_reset_engine_fn(data: *mut c_void, val: u64) -> c_int { let mut resp = core::mem::MaybeUninit::<vpu_jsm_msg>::uninit(); ivpu_jsm_reset_engine(data as *mut ivpu_device,val as u32,resp.as_mut_ptr()) }
unsafe fn ivpu_resume_engine_fn(data: *mut c_void, val: u64) -> c_int { ivpu_jsm_hws_resume_engine(data as *mut ivpu_device,val as u32) }
unsafe fn dct_active_get(data: *mut c_void, out: *mut u64) -> c_int { *out=(*(*data.cast::<ivpu_device>()).pm).dct_active_percent as u64; 0 }
unsafe fn dct_active_set(data: *mut c_void, active: u64) -> c_int { let v=data.cast::<ivpu_device>(); if active>100{return -EINVAL;} let mut ret=ivpu_rpm_get(v); if ret<0{return ret;} ret=if active!=0{ivpu_pm_dct_enable(v,active as u32)}else{ivpu_pm_dct_disable(v)}; ivpu_rpm_put(v); ret }

unsafe fn print_priority_band(s:*mut seq_file,hw:*mut ivpu_hw_info,band:c_int,name:*const c_char){seq_printf(s,"%-9s: grace_period %9u process_grace_period %9u process_quantum %9u\n",name,(*hw).hws.grace_period[band as usize],(*hw).hws.process_grace_period[band as usize],(*hw).hws.process_quantum[band as usize]);}
unsafe fn priority_bands_show(s:*mut seq_file,_v:*mut c_void)->c_int{let v=(*s).private as *mut ivpu_device;let h=(*v).hw;print_priority_band(s,h,VPU_JOB_SCHEDULING_PRIORITY_BAND_IDLE,b"Idle\0".as_ptr() as *const c_char);print_priority_band(s,h,VPU_JOB_SCHEDULING_PRIORITY_BAND_NORMAL,b"Normal\0".as_ptr() as *const c_char);print_priority_band(s,h,VPU_JOB_SCHEDULING_PRIORITY_BAND_FOCUS,b"Focus\0".as_ptr() as *const c_char);print_priority_band(s,h,VPU_JOB_SCHEDULING_PRIORITY_BAND_REALTIME,b"Realtime\0".as_ptr() as *const c_char);0}
unsafe fn priority_bands_fops_open(i:*mut inode,f:*mut file)->c_int{single_open(f,Some(priority_bands_show),(*i).i_private)}
unsafe fn priority_bands_fops_write(file:*mut file,user_buf:*const c_char,size:usize,pos:*mut loff_t)->isize{let s=(*file).private_data as *mut seq_file;let v=(*s).private as *mut ivpu_device;let mut buf=[0u8;64];if *pos!=0||size>=buf.len(){return -EINVAL as isize;}let ret=simple_write_to_buffer(buf.as_mut_ptr() as *mut c_void,buf.len()-1,pos,user_buf,size);if ret<0{return ret as isize;}buf[ret as usize]=0;let text=core::str::from_utf8_unchecked(&buf[..ret as usize]);let mut it=text.split_whitespace();let band=it.next().and_then(|x|x.parse::<usize>().ok());let gp=it.next().and_then(|x|x.parse::<u32>().ok());let pgp=it.next().and_then(|x|x.parse::<u32>().ok());let pq=it.next().and_then(|x|x.parse::<u32>().ok());if band.is_none()||gp.is_none()||pgp.is_none()||pq.is_none()||it.next().is_some(){return -EINVAL as isize;}let b=band.unwrap();if b>=VPU_JOB_SCHEDULING_PRIORITY_BAND_COUNT as usize{return -EINVAL as isize;}(*(*v).hw).hws.grace_period[b]=gp.unwrap();(*(*v).hw).hws.process_grace_period[b]=pgp.unwrap();(*(*v).hw).hws.process_quantum[b]=pq.unwrap();size as isize}

pub unsafe fn ivpu_debugfs_init(vdev:*mut ivpu_device){let root=(*vdev).drm.debugfs_root;drm_debugfs_add_files(&mut (*vdev).drm,VDEV_DEBUGFS_LIST.as_ptr(),VDEV_DEBUGFS_LIST.len());debugfs_create_file(b"force_recovery\0".as_ptr() as *const c_char,0o200,root,vdev,&ivpu_force_recovery_fops);debugfs_create_file(b"dvfs_mode\0".as_ptr() as *const c_char,0o644,root,vdev,&dvfs_mode_fops);debugfs_create_file(b"fw_dyndbg\0".as_ptr() as *const c_char,0o200,root,vdev,&fw_dyndbg_fops);debugfs_create_file(b"fw_log\0".as_ptr() as *const c_char,0o644,root,vdev,&fw_log_fops);debugfs_create_file(b"fw_trace_destination_mask\0".as_ptr() as *const c_char,0o200,root,vdev,&fw_trace_destination_mask_fops);debugfs_create_file(b"fw_trace_hw_comp_mask\0".as_ptr() as *const c_char,0o200,root,vdev,&fw_trace_hw_comp_mask_fops);debugfs_create_file(b"fw_trace_level\0".as_ptr() as *const c_char,0o200,root,vdev,&fw_trace_level_fops);debugfs_create_file(b"hws_priority_bands\0".as_ptr() as *const c_char,0o200,root,vdev,&ivpu_hws_priority_bands_fops);debugfs_create_file(b"reset_engine\0".as_ptr() as *const c_char,0o200,root,vdev,&ivpu_reset_engine_fops);debugfs_create_file(b"resume_engine\0".as_ptr() as *const c_char,0o200,root,vdev,&ivpu_resume_engine_fops);if ivpu_hw_ip_gen(vdev)>=IVPU_HW_IP_40XX{debugfs_create_file(b"fw_profiling_freq_drive\0".as_ptr() as *const c_char,0o200,root,vdev,&fw_profiling_freq_fops);debugfs_create_file(b"dct\0".as_ptr() as *const c_char,0o644,root,vdev,&ivpu_dct_fops);}
    // #ifdef CONFIG_FAULT_INJECTION
    #[cfg(feature="CONFIG_FAULT_INJECTION")] fault_create_debugfs_attr(b"fail_hw\0".as_ptr() as *const c_char,root,&ivpu_hw_failure);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
