// SPDX-License-Identifier: GPL-2.0+
/* Originally from efivars.c */

// Kernel headers and `internal.h` are dependencies supplied by the surrounding
// translation unit and are intentionally not reproduced here.

extern "C" {
    fn kmalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn ucs2_strnlen(s: *const efi_char16_t, max: usize) -> usize;
    fn ucs2_strsize(s: *const efi_char16_t, max: usize) -> usize;
    fn ucs2_utf8size(s: *const efi_char16_t) -> i32;
    fn ucs2_as_utf8(dst: *mut u8, src: *const efi_char16_t, len: i32);
    fn hex_to_bin(c: u32) -> i32;
    fn efi_guid_to_str(guid: *const efi_guid_t, dst: *mut u8);
    fn efi_guidcmp(a: efi_guid_t, b: efi_guid_t) -> bool;
    fn strreplace(s: *mut u8, old: u8, new: u8);
    fn efivar_lock() -> i32;
    fn efivar_unlock();
    fn efivar_get_next_variable(size: *mut usize, name: *mut efi_char16_t,
                                 vendor: *mut efi_guid_t) -> efi_status_t;
    fn efivarfs_variable_is_present(name: *mut efi_char16_t,
                                    vendor: *mut efi_guid_t, data: *mut core::ffi::c_void) -> bool;
    fn printk(fmt: *const u8, ...);
    fn pr_warn(fmt: *const u8, ...);
    fn func_placeholder();
    fn efivar_set_variable_locked(name: *mut efi_char16_t, vendor: *mut efi_guid_t,
                                  attributes: u32, size: usize, data: *mut core::ffi::c_void,
                                  nonblocking: bool) -> efi_status_t;
    fn efivar_get_variable(name: *mut efi_char16_t, vendor: *mut efi_guid_t,
                           attributes: *mut u32, size: *mut usize,
                           data: *mut core::ffi::c_void) -> efi_status_t;
    fn efi_status_to_err(status: efi_status_t) -> i32;
}

type efi_char16_t = u16;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type efi_status_t = usize;

#[repr(C)]
struct efi_guid_t { data: [u8; 16] }

#[repr(C)]
struct efi_generic_dev_path { type_: u8, sub_type: u8, length: u16 }

#[repr(C)]
struct efi_variable { VariableName: *mut efi_char16_t, VendorGuid: efi_guid_t }

#[repr(C)]
struct efivar_entry { var: efi_variable }

const GFP_KERNEL: u32 = 0;
const EFI_VAR_NAME_LEN: usize = 1024;
const EFI_VARIABLE_GUID_LEN: usize = 36;
const EFI_DEV_END_PATH: u8 = 0x7f;
const EFI_DEV_END_PATH2: u8 = 0xff;
const EFI_DEV_END_ENTIRE: u8 = 0xff;
const EFI_SUCCESS: efi_status_t = 0;
const EFI_UNSUPPORTED: efi_status_t = 3;
const EFI_NOT_FOUND: efi_status_t = 14;
const EFI_BUFFER_TOO_SMALL: efi_status_t = 5;
const EOPNOTSUPP: i32 = 95;
const EINVAL: i32 = 22;

extern "C" {
    static EFI_GLOBAL_VARIABLE_GUID: efi_guid_t;
    static LINUX_EFI_CRASH_GUID: efi_guid_t;
    static NULL_GUID: efi_guid_t;
}

unsafe fn validate_device_path(_var_name: *mut efi_char16_t, _match_: i32,
                               buffer: *mut u8, len: usize) -> bool {
    if len < core::mem::size_of::<efi_generic_dev_path>() { return false; }
    let mut offset = 0usize;
    let mut node = buffer as *mut efi_generic_dev_path;
    while offset <= len - core::mem::size_of::<efi_generic_dev_path>() &&
          (*node).length as usize >= core::mem::size_of::<efi_generic_dev_path>() &&
          (*node).length as usize <= len - offset {
        offset += (*node).length as usize;
        if ((*node).type_ == EFI_DEV_END_PATH || (*node).type_ == EFI_DEV_END_PATH2) &&
           (*node).sub_type == EFI_DEV_END_ENTIRE { return true; }
        node = buffer.add(offset) as *mut efi_generic_dev_path;
    }
    false
}

unsafe fn validate_boot_order(_var_name: *mut efi_char16_t, _match_: i32,
                              _buffer: *mut u8, len: usize) -> bool { len % 2 == 0 }

unsafe fn validate_load_option(var_name: *mut efi_char16_t, match_: i32,
                               buffer: *mut u8, len: usize) -> bool {
    for i in match_..match_ + 4 {
        if *var_name.add(i as usize) > 127 || hex_to_bin((*var_name.add(i as usize) & 0xff) as u32) < 0 { return true; }
    }
    let namelen = ucs2_strnlen(var_name, EFI_VAR_NAME_LEN);
    if namelen > (match_ + 4) as usize || len < 8 { return false; }
    let filepathlength = *buffer.add(4) as usize | ((*buffer.add(5) as usize) << 8);
    let desclength = ucs2_strsize(buffer.add(6) as *const efi_char16_t, len - 6) + 2;
    if desclength == 0 || desclength + filepathlength + 6 > len { return false; }
    validate_device_path(var_name, match_, buffer.add(desclength + 6), filepathlength)
}

unsafe fn validate_uint16(_var_name: *mut efi_char16_t, _match_: i32, _buffer: *mut u8, len: usize) -> bool { len == 2 }

unsafe fn validate_ascii_string(_var_name: *mut efi_char16_t, _match_: i32, buffer: *mut u8, len: usize) -> bool {
    for i in 0..len { if *buffer.add(i) > 127 { return false; } if *buffer.add(i) == 0 { return true; } }
    false
}

#[repr(C)]
struct variable_validate { vendor: efi_guid_t, name: *const u8, validate: Option<unsafe fn(*mut efi_char16_t, i32, *mut u8, usize) -> bool> }

unsafe fn cstr(s: *const u8) -> &'static [u8] { let mut n=0; while *s.add(n)!=0 { n+=1; } core::slice::from_raw_parts(s,n) }

// The table is retained in source order; GUID constants and string literals are external-layout equivalents.
static mut VARIABLE_VALIDATE: [variable_validate; 16] = [
    variable_validate { vendor: efi_guid_t { data:[0;16] }, name: b"BootNext\0".as_ptr(), validate: Some(validate_uint16) },
    variable_validate { vendor: efi_guid_t { data:[0;16] }, name: b"BootOrder\0".as_ptr(), validate: Some(validate_boot_order) },
    variable_validate { vendor: efi_guid_t { data:[0;16] }, name: b"Boot*\0".as_ptr(), validate: Some(validate_load_option) },
    variable_validate { vendor: efi_guid_t { data:[0;16] }, name: b"DriverOrder\0".as_ptr(), validate: Some(validate_boot_order) },
    variable_validate { vendor: efi_guid_t { data:[0;16] }, name: b"Driver*\0".as_ptr(), validate: Some(validate_load_option) },
    variable_validate { vendor: efi_guid_t { data:[0;16] }, name: b"ConIn\0".as_ptr(), validate: Some(validate_device_path) },
    variable_validate { vendor: efi_guid_t { data:[0;16] }, name: b"ConInDev\0".as_ptr(), validate: Some(validate_device_path) },
    variable_validate { vendor: efi_guid_t { data:[0;16] }, name: b"ConOut\0".as_ptr(), validate: Some(validate_device_path) },
    variable_validate { vendor: efi_guid_t { data:[0;16] }, name: b"ConOutDev\0".as_ptr(), validate: Some(validate_device_path) },
    variable_validate { vendor: efi_guid_t { data:[0;16] }, name: b"ErrOut\0".as_ptr(), validate: Some(validate_device_path) },
    variable_validate { vendor: efi_guid_t { data:[0;16] }, name: b"ErrOutDev\0".as_ptr(), validate: Some(validate_device_path) },
    variable_validate { vendor: efi_guid_t { data:[0;16] }, name: b"Lang\0".as_ptr(), validate: Some(validate_ascii_string) },
    variable_validate { vendor: efi_guid_t { data:[0;16] }, name: b"OsIndications\0".as_ptr(), validate: None },
    variable_validate { vendor: efi_guid_t { data:[0;16] }, name: b"PlatformLang\0".as_ptr(), validate: Some(validate_ascii_string) },
    variable_validate { vendor: efi_guid_t { data:[0;16] }, name: b"Timeout\0".as_ptr(), validate: Some(validate_uint16) },
    variable_validate { vendor: efi_guid_t { data:[0;16] }, name: b"\0".as_ptr(), validate: None },
];

unsafe fn variable_matches(var_name: *const u8, len: usize, match_name: *const u8, match_: *mut i32) -> bool {
    *match_ = 0;
    loop { let c=*match_name.add(*match_ as usize); match c { b'*'=>return true, 0=>return *match_ as usize==len, _=>{ if (*match_ as usize)<len && c==*var_name.add(*match_ as usize) { *match_+=1; continue; } return false; } } }
}

pub unsafe fn efivar_get_utf8name(name16: *const efi_char16_t, vendor: *mut efi_guid_t) -> *mut u8 {
    let len = ucs2_utf8size(name16) as usize;
    let name = kmalloc(len + 1 + EFI_VARIABLE_GUID_LEN + 1, GFP_KERNEL) as *mut u8;
    if name.is_null() { return core::ptr::null_mut(); }
    ucs2_as_utf8(name, name16, len as i32);
    *name.add(len) = b'-';
    efi_guid_to_str(vendor, name.add(len + 1));
    *name.add(len + EFI_VARIABLE_GUID_LEN + 1) = 0;
    strreplace(name, b'/', b'!');
    name
}

pub unsafe fn efivar_validate(vendor: efi_guid_t, var_name: *mut efi_char16_t,
                              data: *mut u8, data_size: usize) -> bool {
    let utf8_size = ucs2_utf8size(var_name) as usize;
    let utf8_name = kmalloc(utf8_size + 1, GFP_KERNEL) as *mut u8;
    if utf8_name.is_null() { return false; }
    ucs2_as_utf8(utf8_name, var_name, utf8_size as i32); *utf8_name.add(utf8_size)=0;
    for i in 0..15 { let v=&VARIABLE_VALIDATE[i]; let mut m=0; if efi_guidcmp(vendor,v.vendor) { continue; }
        if variable_matches(utf8_name, utf8_size+1, v.name, &mut m) { if v.validate.is_none() { break; } kfree(utf8_name as *mut _); return (v.validate.unwrap())(var_name,m,data,data_size); }
    }
    kfree(utf8_name as *mut _); true
}

pub unsafe fn efivar_variable_is_removable(vendor: efi_guid_t, var_name: *const u8, len: usize) -> bool {
    for i in 0..15 { let v=&VARIABLE_VALIDATE[i]; let mut m=0; if !efi_guidcmp(v.vendor,vendor) && variable_matches(var_name,len,v.name,&mut m) { return true; } }
    false
}

unsafe fn var_name_strnsize(variable_name: *mut efi_char16_t, variable_name_size: usize) -> usize {
    let mut len=2usize; while len<=variable_name_size { if *variable_name.add(len/2-1)==0 { break; } len+=2; } core::cmp::min(len,variable_name_size)
}

unsafe fn dup_variable_bug(str16: *mut efi_char16_t, vendor_guid: *mut efi_guid_t, len16: usize) {
    let len8=len16/2; let str8=kzalloc(len8,GFP_KERNEL) as *mut u8; if str8.is_null(){return;}
    for i in 0..len8 {*str8.add(i)=*str16.add(i) as u8;} printk(b"efivars: duplicate variable: %s-%pUl\0".as_ptr(),str8,vendor_guid); kfree(str8 as *mut _);
}

pub unsafe fn efivar_init(func: Option<unsafe extern "C" fn(*mut efi_char16_t, efi_guid_t, usize, *mut core::ffi::c_void)->i32>, data: *mut core::ffi::c_void, duplicate_check: bool) -> i32 {
    let mut size=512usize; let name=kzalloc(size,GFP_KERNEL) as *mut efi_char16_t; if name.is_null(){return -12;}
    let mut err=efivar_lock(); if err!=0 { kfree(name as *mut _); return err; } let mut vendor=efi_guid_t{data:[0;16]}; let mut status;
    loop { size=512; status=efivar_get_next_variable(&mut size,name,&mut vendor); match status {
        EFI_SUCCESS=>{size=var_name_strnsize(name,size); if duplicate_check && efivarfs_variable_is_present(name,&mut vendor,data){dup_variable_bug(name,&mut vendor,size);status=EFI_NOT_FOUND;}else{err=func.unwrap()(name,vendor,size,data);if err!=0{status=EFI_NOT_FOUND;}}},
        EFI_UNSUPPORTED=>{err=-EOPNOTSUPP;status=EFI_NOT_FOUND;}, EFI_NOT_FOUND=>{}, EFI_BUFFER_TOO_SMALL=>{status=EFI_NOT_FOUND;}, _=>{status=EFI_NOT_FOUND;}
    } if status==EFI_NOT_FOUND {break;} } efivar_unlock(); kfree(name as *mut _); err
}

pub unsafe fn efivar_entry_delete(entry:*mut efivar_entry)->i32 { let err=efivar_lock();if err!=0{return err;} let s=efivar_set_variable_locked((*entry).var.VariableName,&mut (*entry).var.VendorGuid,0,0,core::ptr::null_mut(),false);efivar_unlock();if s!=EFI_SUCCESS&&s!=EFI_NOT_FOUND{return efi_status_to_err(s)}0 }
pub unsafe fn efivar_entry_size(entry:*mut efivar_entry,size:*mut usize)->i32 { *size=0;let err=efivar_lock();if err!=0{return err;}let s=efivar_get_variable((*entry).var.VariableName,&mut (*entry).var.VendorGuid,core::ptr::null_mut(),size,core::ptr::null_mut());efivar_unlock();if s!=EFI_BUFFER_TOO_SMALL{return efi_status_to_err(s)}0 }
pub unsafe fn __efivar_entry_get(entry:*mut efivar_entry,attributes:*mut u32,size:*mut usize,data:*mut core::ffi::c_void)->i32 { efi_status_to_err(efivar_get_variable((*entry).var.VariableName,&mut (*entry).var.VendorGuid,attributes,size,data)) }
pub unsafe fn efivar_entry_get(entry:*mut efivar_entry,attributes:*mut u32,size:*mut usize,data:*mut core::ffi::c_void)->i32 {let e=efivar_lock();if e!=0{return e;}let r=__efivar_entry_get(entry,attributes,size,data);efivar_unlock();r}
pub unsafe fn efivar_entry_set_get_size(entry:*mut efivar_entry,attributes:u32,size:*mut usize,data:*mut core::ffi::c_void,set:*mut bool)->i32 { *set=false;let name=(*entry).var.VariableName;let vendor=&mut (*entry).var.VendorGuid;if !efivar_validate(*vendor,name,data as *mut u8,*size){return -EINVAL;}let e=efivar_lock();if e!=0{return e;}let s=efivar_set_variable_locked(name,vendor,attributes,*size,data,false);if s!=EFI_SUCCESS{efivar_unlock();return efi_status_to_err(s)}*set=true;*size=0;let s=efivar_get_variable(name,vendor,core::ptr::null_mut(),size,core::ptr::null_mut());efivar_unlock();if s!=EFI_SUCCESS&&s!=EFI_BUFFER_TOO_SMALL{return efi_status_to_err(s)}0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
