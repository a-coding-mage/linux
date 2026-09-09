/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

#[cfg(feature = "CONFIG_DEBUG_FS")]
static mut TA_IF_LOAD_DEBUGFS_WRITE: Option<unsafe extern "C" fn(*mut file, *const core::ffi::c_char, usize, *mut loff_t) -> ssize_t> = None;
#[cfg(feature = "CONFIG_DEBUG_FS")]
static mut TA_IF_UNLOAD_DEBUGFS_WRITE: Option<unsafe extern "C" fn(*mut file, *const core::ffi::c_char, usize, *mut loff_t) -> ssize_t> = None;
#[cfg(feature = "CONFIG_DEBUG_FS")]
static mut TA_IF_INVOKE_DEBUGFS_WRITE: Option<unsafe extern "C" fn(*mut file, *const core::ffi::c_char, usize, *mut loff_t) -> ssize_t> = None;

#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe fn get_bin_version(bin: *const u8) -> u32 {
    let hdr = bin as *const common_firmware_header;
    (*hdr).ucode_version
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe fn prep_ta_mem_context(mem_context: *mut ta_mem_context, shared_buf: *mut u8, shared_buf_len: u32) -> i32 {
    if (*mem_context).shared_mem_size < shared_buf_len { return -EINVAL; }
    memset((*mem_context).shared_buf as *mut core::ffi::c_void, 0, (*mem_context).shared_mem_size as usize);
    memcpy((*mem_context).shared_buf as *mut core::ffi::c_void, shared_buf as *const core::ffi::c_void, shared_buf_len as usize);
    0
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe fn is_ta_type_valid(ta_type: ta_type_id) -> bool {
    match ta_type { TA_TYPE_RAS => true, _ => false }
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
static ras_ta_funcs: ta_funcs = ta_funcs {
    fn_ta_initialize: Some(psp_ras_initialize),
    fn_ta_invoke: Some(psp_ras_invoke),
    fn_ta_terminate: Some(psp_ras_terminate),
};

#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe fn set_ta_context_funcs(psp: *mut psp_context, ta_type: ta_type_id, pcontext: *mut *mut ta_context) {
    match ta_type {
        TA_TYPE_RAS => {
            *pcontext = &mut (*psp).ras_context.context;
            (*psp).ta_funcs = &ras_ta_funcs;
        }
        _ => {}
    }
}

/*
 * DOC: AMDGPU TA debugfs interfaces
 *
 * Three debugfs interfaces can be opened by a program to load/invoke/unload TA.
 * The transmit and receive buffer layouts are defined by the corresponding
 * kernel interface and are preserved by the routines below.
 */

#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe extern "C" fn ta_if_load_debugfs_write(fp: *mut file, buf: *const core::ffi::c_char, _len: usize, _off: *mut loff_t) -> ssize_t {
    let mut ta_type = 0u32; let mut ta_bin_len = 0u32; let mut ta_bin: *mut u8 = core::ptr::null_mut(); let mut copy_pos = 0u32; let mut ret = 0i32;
    let adev = file_inode(fp).as_ref().unwrap().i_private as *mut amdgpu_device;
    let psp = &mut (*adev).psp as *mut psp_context; let mut context: *mut ta_context = core::ptr::null_mut();
    if buf.is_null() { return -EINVAL as ssize_t; }
    ret = copy_from_user(&mut ta_type as *mut _ as *mut core::ffi::c_void, buf.add(copy_pos as usize) as *const _, 4);
    if ret != 0 || !is_ta_type_valid(ta_type as ta_type_id) { return -EFAULT as ssize_t; }
    copy_pos += 4;
    ret = copy_from_user(&mut ta_bin_len as *mut _ as *mut core::ffi::c_void, buf.add(copy_pos as usize) as *const _, 4);
    if ret != 0 { return -EFAULT as ssize_t; }
    if ta_bin_len < core::mem::size_of::<common_firmware_header>() as u32 || ta_bin_len > PSP_1_MEG { return -EINVAL as ssize_t; }
    copy_pos += 4;
    ta_bin = memdup_user(buf.add(copy_pos as usize), ta_bin_len as usize);
    if IS_ERR(ta_bin) { return PTR_ERR(ta_bin) as ssize_t; }
    set_ta_context_funcs(psp, ta_type as ta_type_id, &mut context);
    if (*psp).ta_funcs.is_null() || (*(*psp).ta_funcs).fn_ta_terminate.is_none() { ret = -EOPNOTSUPP; goto err_free_bin; }
    if (*context).mem_context.shared_buf.is_null() { ret = psp_ta_init_shared_buf(psp, &mut (*context).mem_context); if ret != 0 { ret = -ENOMEM; goto err_free_bin; } }
    ret = psp_fn_ta_terminate(psp);
    if ret != 0 || (*context).resp_status != 0 { if ret == 0 { ret = -EINVAL; } goto err_free_ta_shared_buf; }
    (*context).ta_type = ta_type as ta_type_id; (*context).bin_desc.fw_version = get_bin_version(ta_bin); (*context).bin_desc.size_bytes = ta_bin_len; (*context).bin_desc.start_addr = ta_bin;
    if (*(*psp).ta_funcs).fn_ta_initialize.is_none() { ret = -EOPNOTSUPP; goto err_free_ta_shared_buf; }
    ret = psp_fn_ta_initialize(psp);
    if ret != 0 || (*context).resp_status != 0 { if ret == 0 { ret = -EINVAL; } goto err_free_ta_shared_buf; }
    if copy_to_user(buf as *mut _, &(*context).session_id as *const _ as *const _, 4) != 0 { ret = -EFAULT; }
err_free_ta_shared_buf:
    if ret != 0 && !(*context).mem_context.shared_buf.is_null() { psp_ta_free_shared_buf(&mut (*context).mem_context); }
err_free_bin:
    kfree(ta_bin as *mut core::ffi::c_void); ret as ssize_t
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe extern "C" fn ta_if_unload_debugfs_write(fp: *mut file, buf: *const core::ffi::c_char, _len: usize, _off: *mut loff_t) -> ssize_t {
    let mut ta_type=0u32; let mut ta_id=0u32; let adev=file_inode(fp).as_ref().unwrap().i_private as *mut amdgpu_device; let psp=&mut (*adev).psp as *mut psp_context; let mut context=core::ptr::null_mut();
    if buf.is_null() { return -EINVAL as ssize_t; }
    if copy_from_user(&mut ta_type as *mut _ as *mut _, buf, 4)!=0 || !is_ta_type_valid(ta_type as ta_type_id) { return -EFAULT as ssize_t; }
    if copy_from_user(&mut ta_id as *mut _ as *mut _, buf.add(4), 4)!=0 { return -EFAULT as ssize_t; }
    set_ta_context_funcs(psp, ta_type as ta_type_id, &mut context); (*context).session_id=ta_id;
    if (*psp).ta_funcs.is_null() || (*(*psp).ta_funcs).fn_ta_terminate.is_none() { return -EOPNOTSUPP as ssize_t; }
    let mut ret=psp_fn_ta_terminate(psp); if ret!=0 || (*context).resp_status!=0 { if ret==0 { ret=-EINVAL; } }
    if !(*context).mem_context.shared_buf.is_null() { psp_ta_free_shared_buf(&mut (*context).mem_context); } ret as ssize_t
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe extern "C" fn ta_if_invoke_debugfs_write(fp: *mut file, buf: *const core::ffi::c_char, _len: usize, _off: *mut loff_t) -> ssize_t {
    let mut vals=[0u32;4]; let mut shared_buf=core::ptr::null_mut(); let adev=file_inode(fp).as_ref().unwrap().i_private as *mut amdgpu_device; let psp=&mut (*adev).psp as *mut psp_context; let mut context=core::ptr::null_mut();
    if buf.is_null() { return -EINVAL as ssize_t; } for i in 0..4 { if copy_from_user(&mut vals[i] as *mut _ as *mut _, buf.add(i*4), 4)!=0 { return -EFAULT as ssize_t; } }
    if vals[3]==0 || vals[3]>PSP_1_MEG { return -EINVAL as ssize_t; } shared_buf=memdup_user(buf.add(16), vals[3] as usize); if IS_ERR(shared_buf) { return PTR_ERR(shared_buf) as ssize_t; }
    set_ta_context_funcs(psp, vals[0] as ta_type_id, &mut context); if context.is_null() || !(*context).initialized { kfree(shared_buf as *mut _); return -EINVAL as ssize_t; }
    if (*psp).ta_funcs.is_null() || (*(*psp).ta_funcs).fn_ta_invoke.is_none() { kfree(shared_buf as *mut _); return -EOPNOTSUPP as ssize_t; }
    (*context).session_id=vals[1]; mutex_lock(&mut (*psp).ras_context.mutex); let mut ret=prep_ta_mem_context(&mut (*context).mem_context, shared_buf, vals[3]); if ret==0 { ret=psp_fn_ta_invoke(psp, vals[2]); if ret==0 && (*context).resp_status!=0 { ret=-EINVAL; } if ret==0 && copy_to_user(buf.add(16) as *mut _, (*context).mem_context.shared_buf, vals[3] as usize)!=0 { ret=-EFAULT; } } mutex_unlock(&mut (*psp).ras_context.mutex); kfree(shared_buf as *mut _); ret as ssize_t
}

pub unsafe extern "C" fn amdgpu_ta_if_debugfs_init(adev: *mut amdgpu_device) {
    #[cfg(feature = "CONFIG_DEBUG_FS")]
    { let minor=adev_to_drm(adev).as_ref().unwrap().primary; let dir=debugfs_create_dir(b"ta_if\0".as_ptr() as *const _, minor.as_ref().unwrap().debugfs_root); debugfs_create_file(b"ta_load\0".as_ptr() as *const _, 0o200, dir, adev as *mut _, &ta_load_debugfs_fops); debugfs_create_file(b"ta_unload\0".as_ptr() as *const _, 0o200, dir, adev as *mut _, &ta_unload_debugfs_fops); debugfs_create_file(b"ta_invoke\0".as_ptr() as *const _, 0o200, dir, adev as *mut _, &ta_invoke_debugfs_fops); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
