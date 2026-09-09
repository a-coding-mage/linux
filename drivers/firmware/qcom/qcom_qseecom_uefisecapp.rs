// SPDX-License-Identifier: GPL-2.0-or-later
/* Client driver for Qualcomm UEFI Secure Application (qcom.tz.uefisecapp). */

// Kernel dependencies supplied by the surrounding tree are intentionally left external.

const QSEE_MAX_NAME_LEN: usize = 1024;
const fn qsee_cmd_uefi(x: u32) -> u32 { 0x8000 | x }
const QSEE_CMD_UEFI_GET_VARIABLE: u32 = qsee_cmd_uefi(0);
const QSEE_CMD_UEFI_SET_VARIABLE: u32 = qsee_cmd_uefi(1);
const QSEE_CMD_UEFI_GET_NEXT_VARIABLE: u32 = qsee_cmd_uefi(2);
const QSEE_CMD_UEFI_QUERY_VARIABLE_INFO: u32 = qsee_cmd_uefi(3);

#[repr(C, packed)]
struct QseeReqUefiGetVariable { command_id: u32, length: u32, name_offset: u32, name_size: u32, guid_offset: u32, guid_size: u32, data_size: u32 }
#[repr(C, packed)]
struct QseeRspUefiGetVariable { command_id: u32, length: u32, status: u32, attributes: u32, data_offset: u32, data_size: u32 }
#[repr(C, packed)]
struct QseeReqUefiSetVariable { command_id: u32, length: u32, name_offset: u32, name_size: u32, guid_offset: u32, guid_size: u32, attributes: u32, data_offset: u32, data_size: u32 }
#[repr(C, packed)]
struct QseeRspUefiSetVariable { command_id: u32, length: u32, status: u32, _unknown1: u32, _unknown2: u32 }
#[repr(C, packed)]
struct QseeReqUefiGetNextVariable { command_id: u32, length: u32, guid_offset: u32, guid_size: u32, name_offset: u32, name_size: u32 }
#[repr(C, packed)]
struct QseeRspUefiGetNextVariable { command_id: u32, length: u32, status: u32, guid_offset: u32, guid_size: u32, name_offset: u32, name_size: u32 }
#[repr(C, packed)]
struct QseeReqUefiQueryVariableInfo { command_id: u32, length: u32, attributes: u32 }
#[repr(C, packed)]
struct QseeRspUefiQueryVariableInfo { command_id: u32, length: u32, status: u32, _pad: u32, storage_space: u64, remaining_space: u64, max_variable_size: u64 }

#[repr(C)]
struct QcuefiClient { client: *mut QseecomClient, efivars: Efivars, mempool: *mut QcomTzmemPool }

// These declarations correspond to symbols and types supplied by Linux headers.
extern "C" {
    static mut __qcuefi_lock: Mutex;
    fn qcom_tzmem_alloc(pool: *mut QcomTzmemPool, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn qcom_qseecom_app_send(client: *mut QseecomClient, req: *mut core::ffi::c_void, req_size: usize, rsp: *mut core::ffi::c_void, rsp_size: usize) -> isize;
    fn ucs2_strnlen(s: *const EfiChar16, max: usize) -> usize;
    fn ucs2_strscpy(dst: *mut core::ffi::c_void, src: *const EfiChar16, count: usize) -> isize;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_qcom_tzmem_pool_new(dev: *mut Device, config: *const QcomTzmemPoolConfig) -> *mut QcomTzmemPool;
    fn efivars_register(vars: *mut Efivars, ops: *const EfivarOperations) -> i32;
    fn efivars_unregister(vars: *mut Efivars);
}

type EfiChar16 = u16; type EfiStatus = usize; type U32 = u32;
#[repr(C)] struct EfiGuid { _data: [u8; 16] }
#[repr(C)] struct Efivars { _opaque: [u8; 0] }
#[repr(C)] struct EfivarOperations { get_variable: Option<unsafe extern "C" fn(*mut EfiChar16,*mut EfiGuid,*mut u32,*mut usize,*mut core::ffi::c_void)->EfiStatus>, set_variable: Option<unsafe extern "C" fn(*mut EfiChar16,*mut EfiGuid,u32,usize,*mut core::ffi::c_void)->EfiStatus>, get_next_variable: Option<unsafe extern "C" fn(*mut usize,*mut EfiChar16,*mut EfiGuid)->EfiStatus>, query_variable_info: Option<unsafe extern "C" fn(u32,*mut u64,*mut u64,*mut u64)->EfiStatus> }
#[repr(C)] struct QseecomClient { aux_dev: AuxiliaryDevice }
#[repr(C)] struct AuxiliaryDevice { dev: Device }
#[repr(C)] struct Device { _opaque: [u8; 0] }
#[repr(C)] struct QcomTzmemPool { _opaque: [u8; 0] }
#[repr(C)] struct QcomTzmemPoolConfig { initial_size: usize, policy: u32, increment: u32, max_size: usize }
#[repr(C)] struct Mutex { _opaque: [u8; 0] }

const EFI_SUCCESS: EfiStatus = 0; const EFI_INVALID_PARAMETER: EfiStatus = 2;
const EFI_BUFFER_TOO_SMALL: EfiStatus = 5; const EFI_OUT_OF_RESOURCES: EfiStatus = 9;
const EFI_NOT_READY: EfiStatus = 6; const EFI_DEVICE_ERROR: EfiStatus = 7;
const EEXIST: i32 = 17; const ENOMEM: i32 = 12; const GFP_KERNEL: u32 = 0;
const QCOM_TZMEM_POLICY_MULTIPLIER: u32 = 0;

#[inline] unsafe fn align(x: usize, a: usize) -> usize { (x + a - 1) & !(a - 1) }
#[inline] unsafe fn qcuefi_buf_align_fields(fields: &[usize]) -> usize { let mut n=0; for &(size, a) in fields { n=align(n,a)+size; } n }
unsafe fn qsee_uefi_status_to_efi(status: u32) -> EfiStatus { (((status & 0xf0000000) as u64) << (usize::BITS - 32) | (status & 0x0fffffff) as u64) as usize }

unsafe fn qsee_uefi_get_variable(q: *mut QcuefiClient, name: *const EfiChar16, guid: *const EfiGuid, attributes: *mut u32, data_size: *mut usize, data: *mut core::ffi::c_void) -> EfiStatus {
    if name.is_null() || guid.is_null() { return EFI_INVALID_PARAMETER; }
    let buffer_size=*data_size; let name_length=ucs2_strnlen(name,QSEE_MAX_NAME_LEN)+1;
    if name_length>QSEE_MAX_NAME_LEN || (buffer_size!=0 && data.is_null()) { return EFI_INVALID_PARAMETER; }
    let req_size=qcuefi_buf_align_fields(&[(core::mem::size_of::<QseeReqUefiGetVariable>(),1),(name_length*2,2),(16,4)]);
    let rsp_size=qcuefi_buf_align_fields(&[(core::mem::size_of::<QseeRspUefiGetVariable>(),1),(buffer_size,1)]);
    let req_offs=align(0,8); let rsp_offs=align(req_offs+req_size,8); let total=rsp_offs+rsp_size;
    let buf=qcom_tzmem_alloc((*q).mempool,total,GFP_KERNEL); if buf.is_null(){return EFI_OUT_OF_RESOURCES;}
    let req=buf.add(req_offs) as *mut QseeReqUefiGetVariable; let rsp=buf.add(rsp_offs) as *mut QseeRspUefiGetVariable;
    (*req).command_id=QSEE_CMD_UEFI_GET_VARIABLE; (*req).data_size=buffer_size as u32; (*req).name_offset=core::mem::size_of::<QseeReqUefiGetVariable>() as u32; (*req).name_size=(name_length*2) as u32; (*req).guid_offset=((*req).name_offset as usize+name_length*2) as u32; (*req).guid_size=16; (*req).length=req_size as u32;
    if ucs2_strscpy((req as *mut u8).add((*req).name_offset as usize) as _,name,name_length)<0{return EFI_INVALID_PARAMETER;}
    memcpy((req as *mut u8).add((*req).guid_offset as usize) as _,guid as _,16);
    if qcom_qseecom_app_send((*q).client,buf.add(req_offs),req_size,buf.add(rsp_offs),rsp_size)!=0{return EFI_DEVICE_ERROR;}
    if (*rsp).command_id!=QSEE_CMD_UEFI_GET_VARIABLE||(*rsp).length as usize<core::mem::size_of::<QseeRspUefiGetVariable>() {return EFI_DEVICE_ERROR;}
    if (*rsp).status!=0 { let e=qsee_uefi_status_to_efi((*rsp).status); if e==EFI_BUFFER_TOO_SMALL {*data_size=(*rsp).data_size as usize;if !attributes.is_null(){*attributes=(*rsp).attributes;}} return e; }
    if (*rsp).length as usize>rsp_size || ((*rsp).data_offset+(*rsp).data_size) as usize>(*rsp).length as usize{return EFI_DEVICE_ERROR;}
    *data_size=(*rsp).data_size as usize;if !attributes.is_null(){*attributes=(*rsp).attributes;}
    if buffer_size==0&&data.is_null(){return EFI_SUCCESS;} if buffer_size<(*rsp).data_size as usize{return EFI_BUFFER_TOO_SMALL;}
    memcpy(data,(rsp as *mut u8).add((*rsp).data_offset as usize) as _,(*rsp).data_size as usize); EFI_SUCCESS
}

unsafe fn qsee_uefi_set_variable(q:*mut QcuefiClient,name:*const EfiChar16,guid:*const EfiGuid,attributes:u32,data_size:usize,data:*const core::ffi::c_void)->EfiStatus {
    if name.is_null()||guid.is_null()||(data_size!=0&&data.is_null()){return EFI_INVALID_PARAMETER;}
    let nl=ucs2_strnlen(name,QSEE_MAX_NAME_LEN)+1;if nl>QSEE_MAX_NAME_LEN{return EFI_INVALID_PARAMETER;}
    let rs=qcuefi_buf_align_fields(&[(core::mem::size_of::<QseeReqUefiSetVariable>(),1),(nl*2,2),(16,4),(data_size,1)]);
    let ro=align(0,8);let so=align(ro+rs,8);let b=qcom_tzmem_alloc((*q).mempool,so+core::mem::size_of::<QseeRspUefiSetVariable>(),GFP_KERNEL);if b.is_null(){return EFI_OUT_OF_RESOURCES;}
    let r=b.add(ro)as*mut QseeReqUefiSetVariable;let s=b.add(so)as*mut QseeRspUefiSetVariable;(*r).command_id=QSEE_CMD_UEFI_SET_VARIABLE;(*r).attributes=attributes;(*r).name_offset=core::mem::size_of::<QseeReqUefiSetVariable>()as u32;(*r).name_size=(nl*2)as u32;(*r).guid_offset=((*r).name_offset as usize+nl*2)as u32;(*r).guid_size=16;(*r).data_offset=(*r).guid_offset+16;(*r).data_size=data_size as u32;(*r).length=rs as u32;
    if ucs2_strscpy((r as*mut u8).add((*r).name_offset as usize)as _,name,nl)<0{return EFI_INVALID_PARAMETER;}memcpy((r as*mut u8).add((*r).guid_offset as usize)as _,guid as _,16);if data_size!=0{memcpy((r as*mut u8).add((*r).data_offset as usize)as _,data,data_size);}
    if qcom_qseecom_app_send((*q).client,b.add(ro),rs,b.add(so),core::mem::size_of::<QseeRspUefiSetVariable>())!=0||(*s).command_id!=QSEE_CMD_UEFI_SET_VARIABLE||(*s).length as usize!=core::mem::size_of::<QseeRspUefiSetVariable>(){return EFI_DEVICE_ERROR;}if (*s).status!=0{return qsee_uefi_status_to_efi((*s).status)}EFI_SUCCESS
}
unsafe fn qsee_uefi_get_next_variable(q:*mut QcuefiClient,name_size:*mut usize,name:*mut EfiChar16,guid:*mut EfiGuid)->EfiStatus {
    if name_size.is_null()||name.is_null()||guid.is_null()||*name_size==0{return EFI_INVALID_PARAMETER;}let ns=*name_size;
    let rs=qcuefi_buf_align_fields(&[(core::mem::size_of::<QseeReqUefiGetNextVariable>(),1),(16,4),(ns,2)]);let zs=qcuefi_buf_align_fields(&[(core::mem::size_of::<QseeRspUefiGetNextVariable>(),1),(16,4),(ns,2)]);let ro=align(0,8);let so=align(ro+rs,8);let b=qcom_tzmem_alloc((*q).mempool,so+zs,GFP_KERNEL);if b.is_null(){return EFI_OUT_OF_RESOURCES;}let r=b.add(ro)as*mut QseeReqUefiGetNextVariable;let s=b.add(so)as*mut QseeRspUefiGetNextVariable;(*r).command_id=QSEE_CMD_UEFI_GET_NEXT_VARIABLE;(*r).guid_offset=core::mem::size_of::<QseeReqUefiGetNextVariable>()as u32;(*r).guid_size=16;(*r).name_offset=(*r).guid_offset+16;(*r).name_size=ns as u32;(*r).length=rs as u32;memcpy((r as*mut u8).add((*r).guid_offset as usize)as _,guid as _,16);if ucs2_strscpy((r as*mut u8).add((*r).name_offset as usize)as _,name,ns/2)<0{return EFI_INVALID_PARAMETER;}if qcom_qseecom_app_send((*q).client,b.add(ro),rs,b.add(so),zs)!=0{return EFI_DEVICE_ERROR;}if (*s).command_id!=QSEE_CMD_UEFI_GET_NEXT_VARIABLE||(*s).length as usize<core::mem::size_of::<QseeRspUefiGetNextVariable>(){return EFI_DEVICE_ERROR;}if (*s).status!=0{let e=qsee_uefi_status_to_efi((*s).status);if e==EFI_BUFFER_TOO_SMALL{*name_size=(*s).name_size as usize;}return e;}if (*s).name_size as usize>ns{*name_size=(*s).name_size as usize;return EFI_BUFFER_TOO_SMALL;}if (*s).guid_size!=16{return EFI_DEVICE_ERROR;}memcpy(guid as _,(s as*mut u8).add((*s).guid_offset as usize)as _,16);if ucs2_strscpy(name,(s as*mut u8).add((*s).name_offset as usize)as _,(*s).name_size as usize/2)<0{return EFI_DEVICE_ERROR;}*name_size=(*s).name_size as usize;EFI_SUCCESS
}
unsafe fn qsee_uefi_query_variable_info(q:*mut QcuefiClient,attr:u32,storage:*mut u64,remaining:*mut u64,max:*mut u64)->EfiStatus{let ro=align(0,8);let so=align(ro+core::mem::size_of::<QseeReqUefiQueryVariableInfo>(),8);let b=qcom_tzmem_alloc((*q).mempool,so+core::mem::size_of::<QseeRspUefiQueryVariableInfo>(),GFP_KERNEL);if b.is_null(){return EFI_OUT_OF_RESOURCES;}let r=b.add(ro)as*mut QseeReqUefiQueryVariableInfo;let s=b.add(so)as*mut QseeRspUefiQueryVariableInfo;(*r).command_id=QSEE_CMD_UEFI_QUERY_VARIABLE_INFO;(*r).length=core::mem::size_of::<QseeReqUefiQueryVariableInfo>()as u32;(*r).attributes=attr;if qcom_qseecom_app_send((*q).client,b.add(ro),core::mem::size_of::<QseeReqUefiQueryVariableInfo>(),b.add(so),core::mem::size_of::<QseeRspUefiQueryVariableInfo>())!=0||(*s).command_id!=QSEE_CMD_UEFI_QUERY_VARIABLE_INFO||(*s).length as usize!=core::mem::size_of::<QseeRspUefiQueryVariableInfo>(){return EFI_DEVICE_ERROR;}if (*s).status!=0{return qsee_uefi_status_to_efi((*s).status)}if !storage.is_null(){*storage=(*s).storage_space}if !remaining.is_null(){*remaining=(*s).remaining_space}if !max.is_null(){*max=(*s).max_variable_size}EFI_SUCCESS}

static mut __Qcuefi: *mut QcuefiClient = core::ptr::null_mut();
unsafe fn qcuefi_set_reference(q:*mut QcuefiClient)->i32 { if !q.is_null()&&!__Qcuefi.is_null(){return -EEXIST;} __Qcuefi=q;0 }

unsafe extern "C" fn qcuefi_get_variable(n:*mut EfiChar16,g:*mut EfiGuid,a:*mut u32,z:*mut usize,d:*mut core::ffi::c_void)->EfiStatus{if __Qcuefi.is_null(){return EFI_NOT_READY}qsee_uefi_get_variable(__Qcuefi,n,g,a,z,d)}
unsafe extern "C" fn qcuefi_set_variable(n:*mut EfiChar16,g:*mut EfiGuid,a:u32,z:usize,d:*mut core::ffi::c_void)->EfiStatus{if __Qcuefi.is_null(){return EFI_NOT_READY}qsee_uefi_set_variable(__Qcuefi,n,g,a,z,d)}
unsafe extern "C" fn qcuefi_get_next_variable(z:*mut usize,n:*mut EfiChar16,g:*mut EfiGuid)->EfiStatus{if __Qcuefi.is_null(){return EFI_NOT_READY}qsee_uefi_get_next_variable(__Qcuefi,z,n,g)}
unsafe extern "C" fn qcuefi_query_variable_info(a:u32,s:*mut u64,r:*mut u64,m:*mut u64)->EfiStatus{if __Qcuefi.is_null(){return EFI_NOT_READY}qsee_uefi_query_variable_info(__Qcuefi,a,s,r,m)}
static QCOM_EFIVAR_OPS: EfivarOperations=EfivarOperations{get_variable:Some(qcuefi_get_variable),set_variable:Some(qcuefi_set_variable),get_next_variable:Some(qcuefi_get_next_variable),query_variable_info:Some(qcuefi_query_variable_info)};

// Driver registration metadata is supplied by the kernel module framework.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
