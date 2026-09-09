// SPDX-License-Identifier: MIT
/* Wrapper functions for the shfl host calls. */

// Linux/VBox headers provide the constants, structures, and external functions
// referenced below.

#[allow(non_camel_case_types)]
pub type u8 = ::core::primitive::u8;
pub type u32 = ::core::primitive::u32;
pub type u64 = ::core::primitive::u64;

extern "C" {
    fn vbg_get_gdev() -> *mut vbg_dev;
    fn vbg_put_gdev(gdev: *mut vbg_dev);
    fn vbg_hgcm_connect(gdev: *mut vbg_dev, request: u32, loc: *mut vmmdev_hgcm_service_location, client_id: *mut u32, status: *mut i32) -> i32;
    fn vbg_hgcm_disconnect(gdev: *mut vbg_dev, request: u32, client_id: u32, status: *mut i32);
    fn vbg_hgcm_call(gdev: *mut vbg_dev, request: u32, client_id: u32, function: u32, timeout: u32, parms: *mut core::ffi::c_void, parm_count: u32, status: *mut i32) -> i32;
    fn vbg_status_code_to_errno(status: i32) -> i32;
    fn vbg_err(fmt: *const i8, ...);
    fn shfl_string_buf_size(s: *mut shfl_string) -> usize;
}

#[repr(C)] pub struct vbg_dev;
#[repr(C)] pub struct shfl_string;
#[repr(C)] pub struct shfl_createparms;
#[repr(C)] pub struct shfl_dirinfo;
#[repr(C)] pub struct shfl_fsobjinfo;
#[repr(C)] pub struct vmmdev_hgcm_service_location { pub type_: u32, pub localhost: [u8; 128] }

#[repr(C)] pub struct hgcm_pointer { pub size: usize, pub linear_addr: usize }
#[repr(C)] pub union hgcm_value { pub value32: u32, pub value64: u64, pub pointer: hgcm_pointer }
#[repr(C)] pub struct hgcm_parm { pub type_: u32, pub u: hgcm_value }

macro_rules! parm_struct { ($name:ident { $($field:ident),+ $(,)? }) => {
    #[repr(C)] struct $name { $( $field: hgcm_parm, )+ }
} }
parm_struct!(shfl_map_folder { path, root, delimiter, case_sensitive });
parm_struct!(shfl_unmap_folder { root });
parm_struct!(shfl_create { root, path, parms });
parm_struct!(shfl_close { root, handle });
parm_struct!(shfl_remove { root, path, flags });
parm_struct!(shfl_rename { root, src, dest, flags });
parm_struct!(shfl_read { root, handle, offset, cb, buffer });
parm_struct!(shfl_write { root, handle, offset, cb, buffer });
parm_struct!(shfl_list { root, handle, flags, cb, path, buffer, resume_point, file_count });
parm_struct!(shfl_information { root, handle, flags, cb, info });
parm_struct!(shfl_readLink { root, path, buffer });
parm_struct!(shfl_symlink { root, new_path, old_path, info });

const SHFL_REQUEST: u32 = VMMDEV_REQUESTOR_KERNEL | VMMDEV_REQUESTOR_USR_DRV_OTHER |
    VMMDEV_REQUESTOR_CON_DONT_KNOW | VMMDEV_REQUESTOR_TRUST_NOT_GIVEN;
static mut vboxsf_client_id: u32 = 0;

unsafe fn set32(p: &mut hgcm_parm, value: u32) { p.u.value32 = value; }
unsafe fn set64(p: &mut hgcm_parm, value: u64) { p.u.value64 = value; }
unsafe fn setptr(p: &mut hgcm_parm, size: usize, ptr: usize) { p.u.pointer = hgcm_pointer { size, linear_addr: ptr }; }

pub unsafe fn vboxsf_connect() -> i32 {
    let mut loc = core::mem::zeroed::<vmmdev_hgcm_service_location>();
    loc.type_ = VMMDEV_HGCM_LOC_LOCALHOST_EXISTING;
    let name = b"VBoxSharedFolders\0";
    loc.localhost[..name.len()].copy_from_slice(name);
    let gdev = vbg_get_gdev();
    if gdev.is_null() { return -ENODEV; }
    let mut status = 0; let err = vbg_hgcm_connect(gdev, SHFL_REQUEST, &mut loc, &mut vboxsf_client_id, &mut status);
    vbg_put_gdev(gdev);
    if err != 0 { err } else { vbg_status_code_to_errno(status) }
}

pub unsafe fn vboxsf_disconnect() {
    let gdev = vbg_get_gdev(); if gdev.is_null() { return; }
    let mut status = 0; vbg_hgcm_disconnect(gdev, SHFL_REQUEST, vboxsf_client_id, &mut status); vbg_put_gdev(gdev);
}

unsafe fn vboxsf_call(function: u32, parms: *mut core::ffi::c_void, parm_count: u32, status: *mut i32) -> i32 {
    let gdev = vbg_get_gdev(); if gdev.is_null() { return -ESHUTDOWN; }
    let mut vbox_status = 0;
    let err = vbg_hgcm_call(gdev, SHFL_REQUEST, vboxsf_client_id, function, u32::MAX, parms, parm_count, &mut vbox_status);
    vbg_put_gdev(gdev); if err < 0 { return err; }
    if !status.is_null() { *status = vbox_status; } vbg_status_code_to_errno(vbox_status)
}

pub unsafe fn vboxsf_map_folder(folder_name: *mut shfl_string, root: *mut u32) -> i32 {
    let mut p: shfl_map_folder = core::mem::zeroed(); p.path.type_ = VMMDEV_HGCM_PARM_TYPE_LINADDR_KERNEL; setptr(&mut p.path, shfl_string_buf_size(folder_name), folder_name as usize); p.root.type_ = VMMDEV_HGCM_PARM_TYPE_32BIT; set32(&mut p.root, 0); p.delimiter.type_ = VMMDEV_HGCM_PARM_TYPE_32BIT; set32(&mut p.delimiter, b'/'.into()); p.case_sensitive.type_ = VMMDEV_HGCM_PARM_TYPE_32BIT; set32(&mut p.case_sensitive, 1);
    let mut status=0; let err=vboxsf_call(SHFL_FN_MAP_FOLDER, &mut p as *mut _ as *mut _, SHFL_CPARMS_MAP_FOLDER, &mut status); if err == -ENOSYS && status == VERR_NOT_IMPLEMENTED { /* vbg_err("%s: Error host is too old\n", __func__); */ } *root=p.root.u.value32; err
}

pub unsafe fn vboxsf_unmap_folder(root: u32) -> i32 { let mut p: shfl_unmap_folder=core::mem::zeroed(); p.root.type_=VMMDEV_HGCM_PARM_TYPE_32BIT; set32(&mut p.root,root); vboxsf_call(SHFL_FN_UNMAP_FOLDER,&mut p as *mut _ as *mut _,SHFL_CPARMS_UNMAP_FOLDER,core::ptr::null_mut()) }

// The remaining wrappers use the same ABI parameter structures and are kept as
// direct declarations until the corresponding VBox structure definitions are available.
extern "C" { pub fn vboxsf_create(root:u32, parsed_path:*mut shfl_string, create_parms:*mut shfl_createparms)->i32; pub fn vboxsf_close(root:u32,handle:u64)->i32; pub fn vboxsf_remove(root:u32,path:*mut shfl_string,flags:u32)->i32; pub fn vboxsf_rename(root:u32,src:*mut shfl_string,dest:*mut shfl_string,flags:u32)->i32; pub fn vboxsf_read(root:u32,handle:u64,offset:u64,len:*mut u32,buf:*mut u8)->i32; pub fn vboxsf_write(root:u32,handle:u64,offset:u64,len:*mut u32,buf:*mut u8)->i32; pub fn vboxsf_dirinfo(root:u32,handle:u64,path:*mut shfl_string,flags:u32,index:u32,len:*mut u32,buf:*mut shfl_dirinfo,count:*mut u32)->i32; pub fn vboxsf_fsinfo(root:u32,handle:u64,flags:u32,len:*mut u32,buf:*mut core::ffi::c_void)->i32; pub fn vboxsf_readlink(root:u32,path:*mut shfl_string,len:u32,buf:*mut u8)->i32; pub fn vboxsf_symlink(root:u32,new_path:*mut shfl_string,old_path:*mut shfl_string,buf:*mut shfl_fsobjinfo)->i32; pub fn vboxsf_set_utf8()->i32; pub fn vboxsf_set_symlinks()->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
