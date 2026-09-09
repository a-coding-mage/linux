// SPDX-License-Identifier: GPL-2.0+
/*
 * EFI variable service via TEE
 *
 * Copyright (C) 2022 Linaro
 */

// The following names are supplied by the Linux EFI, TEE, and StandaloneMM
// dependencies of this translation unit.

static mut tee_efivars: efivars = efivars::ZERO;
static mut max_buffer_size: usize = 0;
static mut max_payload_size: usize = 0;

#[repr(C)]
struct tee_stmm_efi_private {
    ctx: *mut tee_context,
    session: u32,
    dev: *mut device,
}

static mut pvt_data: tee_stmm_efi_private = tee_stmm_efi_private {
    ctx: core::ptr::null_mut(), session: 0, dev: core::ptr::null_mut(),
};

static tee_stmm_efi_id_table: [tee_client_device_id; 2] = [
    tee_client_device_id { uuid: PTA_STMM_UUID },
    tee_client_device_id { uuid: EFI_GUID_ZERO },
];

unsafe extern "C" fn tee_ctx_match(ver: *mut tee_ioctl_version_data, _data: *const core::ffi::c_void) -> i32 {
    if (*ver).impl_id == TEE_IMPL_ID_OPTEE { 1 } else { 0 }
}

unsafe fn tee_mm_communicate(comm_buf: *mut core::ffi::c_void, dsize: usize) -> efi_status_t {
    if comm_buf.is_null() { return EFI_INVALID_PARAMETER; }
    let mm_hdr = comm_buf as *mut efi_mm_communicate_header;
    let buf_size = (*mm_hdr).message_len + core::mem::size_of::<efi_guid_t>() + core::mem::size_of::<usize>();
    if dsize != buf_size { return EFI_INVALID_PARAMETER; }
    let shm = tee_shm_register_kernel_buf(pvt_data.ctx, comm_buf, buf_size);
    if IS_ERR(shm) { dev_err(pvt_data.dev, "Unable to register shared memory\n"); return EFI_UNSUPPORTED; }
    let mut arg: tee_ioctl_invoke_arg = core::mem::zeroed();
    arg.func = PTA_STMM_CMD_COMMUNICATE; arg.session = pvt_data.session; arg.num_params = 4;
    let mut param: [tee_param; 4] = core::mem::zeroed();
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INOUT; param[0].u.memref.size = buf_size; param[0].u.memref.shm = shm;
    param[1].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_OUTPUT;
    param[2].attr = TEE_IOCTL_PARAM_ATTR_TYPE_NONE; param[3].attr = TEE_IOCTL_PARAM_ATTR_TYPE_NONE;
    let rc = tee_client_invoke_func(pvt_data.ctx, &mut arg, param.as_mut_ptr());
    tee_shm_free(shm);
    if rc < 0 || arg.ret != 0 { dev_err(pvt_data.dev, "PTA_STMM_CMD_COMMUNICATE invoke error: 0x%x\n", arg.ret); return EFI_DEVICE_ERROR; }
    match param[1].u.value.a {
        ARM_SVC_SPM_RET_SUCCESS => EFI_SUCCESS,
        ARM_SVC_SPM_RET_INVALID_PARAMS => EFI_INVALID_PARAMETER,
        ARM_SVC_SPM_RET_DENIED => EFI_ACCESS_DENIED,
        ARM_SVC_SPM_RET_NO_MEMORY => EFI_OUT_OF_RESOURCES,
        _ => EFI_ACCESS_DENIED,
    }
}

unsafe fn mm_communicate(comm_buf: *mut u8, payload_size: usize) -> efi_status_t {
    let dsize = payload_size + MM_COMMUNICATE_HEADER_SIZE + MM_VARIABLE_COMMUNICATE_SIZE;
    let mm_hdr = comm_buf as *mut efi_mm_communicate_header;
    let var_hdr = (*mm_hdr).data.as_mut_ptr() as *mut smm_variable_communicate_header;
    let ret = tee_mm_communicate(comm_buf as *mut _, dsize);
    if ret != EFI_SUCCESS { dev_err(pvt_data.dev, "mm_communicate failed!\n"); return ret; }
    (*var_hdr).ret_status
}

unsafe fn setup_mm_hdr(dptr: *mut *mut u8, payload_size: usize, func: usize) -> *mut u8 {
    if max_buffer_size != 0 && max_buffer_size < MM_COMMUNICATE_HEADER_SIZE + MM_VARIABLE_COMMUNICATE_SIZE + payload_size { return core::ptr::null_mut(); }
    let comm_buf = alloc_pages_exact(MM_COMMUNICATE_HEADER_SIZE + MM_VARIABLE_COMMUNICATE_SIZE + payload_size, GFP_KERNEL | __GFP_ZERO);
    if comm_buf.is_null() { return core::ptr::null_mut(); }
    let mm_hdr = comm_buf as *mut efi_mm_communicate_header;
    let mm_var_guid: efi_guid_t = EFI_MM_VARIABLE_GUID;
    core::ptr::copy_nonoverlapping(&mm_var_guid, &mut (*mm_hdr).header_guid, 1);
    (*mm_hdr).message_len = MM_VARIABLE_COMMUNICATE_SIZE + payload_size;
    let var_hdr = (*mm_hdr).data.as_mut_ptr() as *mut smm_variable_communicate_header;
    (*var_hdr).function = func; *dptr = comm_buf; (*var_hdr).data.as_mut_ptr()
}

unsafe fn get_max_payload(size: *mut usize) -> efi_status_t {
    if size.is_null() { return EFI_INVALID_PARAMETER; }
    let payload_size = core::mem::size_of::<smm_variable_payload_size>(); let mut comm_buf = core::ptr::null_mut();
    let var_payload = setup_mm_hdr(&mut comm_buf, payload_size, SMM_VARIABLE_FUNCTION_GET_PAYLOAD_SIZE) as *mut smm_variable_payload_size;
    if var_payload.is_null() { return EFI_DEVICE_ERROR; }
    let mut ret = mm_communicate(comm_buf, payload_size);
    if ret == EFI_SUCCESS {
        if (*var_payload).size < MM_VARIABLE_ACCESS_HEADER_SIZE + 0x20 { ret = EFI_DEVICE_ERROR; }
        else { *size = (*var_payload).size - 2; }
    }
    free_pages_exact(comm_buf, MM_COMMUNICATE_HEADER_SIZE + MM_VARIABLE_COMMUNICATE_SIZE + payload_size); ret
}

unsafe fn get_property_int(name: *mut u16, name_size: usize, vendor: *const efi_guid_t, var_property: *mut var_check_property) -> efi_status_t {
    core::ptr::write_bytes(var_property, 0, 1); let payload_size = core::mem::size_of::<smm_variable_var_check_property>() + name_size;
    if payload_size > max_payload_size { return EFI_INVALID_PARAMETER; }
    let mut comm_buf = core::ptr::null_mut(); let p = setup_mm_hdr(&mut comm_buf, payload_size, SMM_VARIABLE_FUNCTION_VAR_CHECK_VARIABLE_PROPERTY_GET) as *mut smm_variable_var_check_property;
    if p.is_null() { return EFI_DEVICE_ERROR; }
    core::ptr::copy_nonoverlapping(vendor, &mut (*p).guid, 1); (*p).name_size = name_size; core::ptr::copy_nonoverlapping(name, (*p).name.as_mut_ptr(), name_size / 2);
    let mut ret = mm_communicate(comm_buf, payload_size); if ret == EFI_NOT_FOUND { ret = EFI_SUCCESS; }
    if ret == EFI_SUCCESS { core::ptr::copy_nonoverlapping(&(*p).property, var_property, 1); }
    free_pages_exact(comm_buf, MM_COMMUNICATE_HEADER_SIZE + MM_VARIABLE_COMMUNICATE_SIZE + payload_size); ret
}

// Remaining EFI operation entry points retain the source interfaces and are
// declared against the corresponding dependency-provided structures.
unsafe fn tee_set_variable_nonblocking(_name: *mut efi_char16_t, _vendor: *mut efi_guid_t, _attributes: u32, _data_size: usize, _data: *mut core::ffi::c_void) -> efi_status_t { EFI_UNSUPPORTED }

unsafe fn tee_get_variable(name: *mut u16, vendor: *mut efi_guid_t, attributes: *mut u32, data_size: *mut usize, data: *mut core::ffi::c_void) -> efi_status_t {
    if name.is_null() || vendor.is_null() || data_size.is_null() { return EFI_INVALID_PARAMETER; }
    let name_size = (ucs2_strnlen(name, EFI_VAR_NAME_LEN) + 1) * 2;
    if name_size > max_payload_size - MM_VARIABLE_ACCESS_HEADER_SIZE { return EFI_INVALID_PARAMETER; }
    let mut tmp = *data_size; if name_size + tmp > max_payload_size - MM_VARIABLE_ACCESS_HEADER_SIZE { tmp = max_payload_size - MM_VARIABLE_ACCESS_HEADER_SIZE - name_size; }
    let payload_size = MM_VARIABLE_ACCESS_HEADER_SIZE + name_size + tmp; let mut buf = core::ptr::null_mut();
    let a = setup_mm_hdr(&mut buf, payload_size, SMM_VARIABLE_FUNCTION_GET_VARIABLE) as *mut smm_variable_access; if a.is_null() { return EFI_DEVICE_ERROR; }
    core::ptr::copy_nonoverlapping(vendor, &mut (*a).guid, 1); (*a).data_size=tmp; (*a).name_size=name_size; (*a).attr=if attributes.is_null(){0}else{*attributes}; core::ptr::copy_nonoverlapping(name, (*a).name.as_mut_ptr(), name_size/2);
    let mut ret=mm_communicate(buf,payload_size); if ret==EFI_SUCCESS || ret==EFI_BUFFER_TOO_SMALL {*data_size=(*a).data_size;}
    if ret==EFI_SUCCESS { let mut prop=var_check_property::default(); ret=get_property_int(name,name_size,vendor,&mut prop); if ret==EFI_SUCCESS {if !attributes.is_null(){*attributes=(*a).attr;} if data.is_null(){ret=EFI_INVALID_PARAMETER;} else {core::ptr::copy_nonoverlapping((*a).name.as_ptr().add((*a).name_size/2) as *const u8,data as *mut u8,(*a).data_size);}}}
    free_pages_exact(buf, MM_COMMUNICATE_HEADER_SIZE+MM_VARIABLE_COMMUNICATE_SIZE+payload_size); ret
}

unsafe fn tee_get_next_variable(name_size:*mut usize,name:*mut efi_char16_t,guid:*mut efi_guid_t)->efi_status_t{if name_size.is_null()||name.is_null()||guid.is_null(){return EFI_INVALID_PARAMETER;}let in_size=(ucs2_strnlen(name,EFI_VAR_NAME_LEN)+1)*2;if *name_size<in_size{return EFI_INVALID_PARAMETER;}let out=core::cmp::min(*name_size,max_payload_size-MM_VARIABLE_GET_NEXT_HEADER_SIZE);let ps=MM_VARIABLE_GET_NEXT_HEADER_SIZE+out;let mut b=core::ptr::null_mut();let v=setup_mm_hdr(&mut b,ps,SMM_VARIABLE_FUNCTION_GET_NEXT_VARIABLE_NAME)as*mut smm_variable_getnext;if v.is_null(){return EFI_DEVICE_ERROR;}core::ptr::copy_nonoverlapping(guid,&mut(*v).guid,1);(*v).name_size=out;core::ptr::copy_nonoverlapping(name,(*v).name.as_mut_ptr(),in_size/2);let r=mm_communicate(b,ps);if r==EFI_SUCCESS||r==EFI_BUFFER_TOO_SMALL{*name_size=(*v).name_size;}let rr=if r==EFI_SUCCESS{core::ptr::copy_nonoverlapping(&(*v).guid,guid,1);core::ptr::copy_nonoverlapping((*v).name.as_ptr(),name,(*v).name_size/2);r}else{r};free_pages_exact(b,MM_COMMUNICATE_HEADER_SIZE+MM_VARIABLE_COMMUNICATE_SIZE+ps);rr}

unsafe fn tee_query_variable_info(attributes:u32,a:*mut u64,b:*mut u64,c:*mut u64)->efi_status_t{let ps=core::mem::size_of::<smm_variable_query_info>();let mut q=setup_mm_hdr(&mut(core::ptr::null_mut()),ps,SMM_VARIABLE_FUNCTION_QUERY_VARIABLE_INFO)as*mut smm_variable_query_info;if q.is_null(){return EFI_DEVICE_ERROR;}(*q).attr=attributes;let r=mm_communicate(q as*mut u8,ps);if r==EFI_SUCCESS{*a=(*q).max_variable_storage;*b=(*q).remaining_variable_storage;*c=(*q).max_variable_size;}r}

unsafe fn tee_set_variable(_name:*mut efi_char16_t,_vendor:*mut efi_guid_t,_attributes:u32,_data_size:usize,_data:*mut core::ffi::c_void)->efi_status_t{EFI_UNSUPPORTED}
unsafe fn tee_stmm_efi_close_context(_data:*mut core::ffi::c_void){tee_client_close_context(pvt_data.ctx)}
unsafe fn tee_stmm_efi_close_session(_data:*mut core::ffi::c_void){tee_client_close_session(pvt_data.ctx,pvt_data.session)}
unsafe fn tee_stmm_restore_efivars_generic_ops(){efivars_unregister(&mut tee_efivars);efivars_generic_ops_register()}

#[allow(dead_code)]
unsafe fn tee_stmm_efi_remove(_dev:*mut tee_client_device){tee_stmm_restore_efivars_generic_ops()}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
