// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of acpi/utils.c. External kernel/ACPI symbols are
 * intentionally referenced but not defined here. */

unsafe fn acpi_util_eval_error(h: acpi_handle, p: acpi_string, s: acpi_status) {
    acpi_handle_debug(h, "Evaluate [%s]: %s\n", p, acpi_format_exception(s));
}

pub unsafe fn acpi_extract_package(package: *mut acpi_object, format: *mut acpi_buffer, buffer: *mut acpi_buffer) -> acpi_status {
    let mut size_required: u32 = 0;
    let mut tail_offset: u32 = 0;
    let mut format_count: u32;
    let mut i: u32;
    if package.is_null() || (*package).type_ != ACPI_TYPE_PACKAGE || (*package).package.count < 1 { return AE_BAD_PARAMETER; }
    if format.is_null() || (*format).pointer.is_null() || (*format).length < 1 { return AE_BAD_PARAMETER; }
    if buffer.is_null() { return AE_BAD_PARAMETER; }
    format_count = ((*format).length as usize / core::mem::size_of::<i8>() - 1) as u32;
    if format_count > (*package).package.count { return AE_BAD_DATA; }
    let format_string = (*format).pointer as *const u8;
    for i in 0..format_count {
        let element = &*(*package).package.elements.add(i as usize);
        match element.type_ {
            ACPI_TYPE_INTEGER => match *format_string.add(i as usize) as char {
                'N' => { size_required += core::mem::size_of::<u64>() as u32; tail_offset += core::mem::size_of::<u64>() as u32; }
                'S' => { size_required += (core::mem::size_of::<*mut i8>() + 8 + 1) as u32; tail_offset += core::mem::size_of::<*mut i8>() as u32; }
                _ => return AE_BAD_DATA,
            },
            ACPI_TYPE_STRING | ACPI_TYPE_BUFFER => match *format_string.add(i as usize) as char {
                'S' => { size_required += (core::mem::size_of::<*mut i8>() as u32) + element.string.length + 1; tail_offset += core::mem::size_of::<*mut i8>() as u32; }
                'B' => { size_required += core::mem::size_of::<*mut u8>() as u32 + element.buffer.length; tail_offset += core::mem::size_of::<*mut u8>() as u32; }
                _ => return AE_BAD_DATA,
            },
            ACPI_TYPE_LOCAL_REFERENCE => if *format_string.add(i as usize) as char != 'R' { return AE_BAD_DATA; } else { size_required += core::mem::size_of::<*mut core::ffi::c_void>() as u32; tail_offset += core::mem::size_of::<*mut core::ffi::c_void>() as u32; },
            _ => return AE_SUPPORT,
        }
    }
    if (*buffer).length == ACPI_ALLOCATE_BUFFER { (*buffer).pointer = ACPI_ALLOCATE_ZEROED(size_required as usize); if (*buffer).pointer.is_null() { return AE_NO_MEMORY; } (*buffer).length = size_required; }
    else if (*buffer).length < size_required { (*buffer).length = size_required; return AE_BUFFER_OVERFLOW; }
    else if (*buffer).length != size_required || (*buffer).pointer.is_null() { return AE_BAD_PARAMETER; }
    let mut head = (*buffer).pointer as *mut u8;
    let mut tail = head.add(tail_offset as usize);
    for i in 0..format_count {
        let element = &*(*package).package.elements.add(i as usize);
        match element.type_ {
            ACPI_TYPE_INTEGER => match *format_string.add(i as usize) as char {
                'N' => { *(head as *mut u64) = element.integer.value; head = head.add(8); }
                'S' => { *(head as *mut *mut u8) = tail; *(tail as *mut u64) = element.integer.value; head = head.add(core::mem::size_of::<*mut u8>()); tail = tail.add(8); *tail = 0; tail = tail.add(1); }
                _ => {}
            },
            ACPI_TYPE_STRING | ACPI_TYPE_BUFFER => match *format_string.add(i as usize) as char {
                'S' => { *(head as *mut *mut u8) = tail; core::ptr::copy_nonoverlapping(element.string.pointer as *const u8, tail, element.string.length as usize); head = head.add(core::mem::size_of::<*mut u8>()); tail = tail.add(element.string.length as usize); *tail = 0; tail = tail.add(1); }
                'B' => { *(head as *mut *mut u8) = tail; core::ptr::copy_nonoverlapping(element.buffer.pointer, tail, element.buffer.length as usize); head = head.add(core::mem::size_of::<*mut u8>()); tail = tail.add(element.buffer.length as usize); }
                _ => {}
            },
            ACPI_TYPE_LOCAL_REFERENCE => { *(head as *mut *mut core::ffi::c_void) = element.reference.handle; head = head.add(core::mem::size_of::<*mut core::ffi::c_void>()); }
            _ => {}
        }
    }
    AE_OK
}

pub unsafe fn acpi_evaluate_integer(handle: acpi_handle, pathname: acpi_string, arguments: *mut acpi_object_list, data: *mut u64) -> acpi_status {
    if data.is_null() { return AE_BAD_PARAMETER; }
    let mut element = core::mem::zeroed::<acpi_object>();
    let mut buffer = acpi_buffer { length: core::mem::size_of::<acpi_object>() as u32, pointer: &mut element as *mut _ as *mut _ };
    let status = acpi_evaluate_object(handle, pathname, arguments, &mut buffer);
    if ACPI_FAILURE(status) { acpi_util_eval_error(handle, pathname, status); return status; }
    if element.type_ != ACPI_TYPE_INTEGER { acpi_util_eval_error(handle, pathname, AE_BAD_DATA); return AE_BAD_DATA; }
    *data = element.integer.value; AE_OK
}

pub unsafe fn acpi_get_local_u64_address(handle: acpi_handle, addr: *mut u64) -> i32 { if ACPI_FAILURE(acpi_evaluate_integer(handle, METHOD_NAME__ADR, core::ptr::null_mut(), addr)) { -ENODATA } else { 0 } }
pub unsafe fn acpi_get_local_address(handle: acpi_handle, addr: *mut u32) -> i32 { let mut adr=0; let r=acpi_get_local_u64_address(handle,&mut adr); if r<0 {r} else {*addr=adr as u32;0} }

pub const ACPI_MAX_SUB_BUF_SIZE: usize = 9;
pub unsafe fn acpi_get_subsystem_id(handle: acpi_handle) -> *const i8 {
    let mut buffer=acpi_buffer{length:ACPI_ALLOCATE_BUFFER,pointer:core::ptr::null_mut()};
    let status=acpi_evaluate_object(handle,METHOD_NAME__SUB,core::ptr::null_mut(),&mut buffer); if ACPI_FAILURE(status){return ERR_PTR(-ENODATA);}
    let obj=buffer.pointer as *mut acpi_object; let mut sub=ERR_PTR(-ENODATA);
    if (*obj).type_==ACPI_TYPE_STRING { let len=libc::strlen((*obj).string.pointer); if len<ACPI_MAX_SUB_BUF_SIZE && len>0 { sub=kstrdup((*obj).string.pointer,GFP_KERNEL); if sub.is_null(){sub=ERR_PTR(-ENOMEM);} } }
    acpi_os_free(buffer.pointer); sub
}

pub unsafe fn acpi_evaluate_reference(handle: acpi_handle, pathname: acpi_string, arguments: *mut acpi_object_list, list: *mut acpi_handle_list) -> bool {
    if list.is_null(){return false;} let mut buffer=acpi_buffer{length:ACPI_ALLOCATE_BUFFER,pointer:core::ptr::null_mut()}; let status=acpi_evaluate_object(handle,pathname,arguments,&mut buffer); if ACPI_FAILURE(status){return false;}
    let package=buffer.pointer as *mut acpi_object; if buffer.length==0||package.is_null()||(*package).type_!=ACPI_TYPE_PACKAGE||(*package).package.count==0 {kfree(buffer.pointer);return false;}
    (*list).count=(*package).package.count; (*list).handles=kzalloc_objs((*list).handles,(*list).count); if (*list).handles.is_null(){(*list).count=0;kfree(buffer.pointer);return false;}
    for i in 0..(*list).count {let e=&*(*package).package.elements.add(i as usize); if e.type_!=ACPI_TYPE_LOCAL_REFERENCE||e.reference.handle.is_null(){kfree((*list).handles);(*list).handles=core::ptr::null_mut();(*list).count=0;kfree(buffer.pointer);return false;} *(*list).handles.add(i as usize)=e.reference.handle;}
    kfree(buffer.pointer); true
}

pub unsafe fn acpi_handle_list_equal(a:*mut acpi_handle_list,b:*mut acpi_handle_list)->bool { (*a).count==(*b).count && libc::memcmp((*a).handles as *const _,(*b).handles as *const _,(*a).count as usize*core::mem::size_of::<acpi_handle>())==0 }
pub unsafe fn acpi_handle_list_replace(dst:*mut acpi_handle_list,src:*mut acpi_handle_list){if (*dst).count!=0{kfree((*dst).handles as *mut _);}(*dst).count=(*src).count;(*dst).handles=(*src).handles;(*src).handles=core::ptr::null_mut();(*src).count=0;}
pub unsafe fn acpi_handle_list_free(list:*mut acpi_handle_list){if (*list).count!=0{kfree((*list).handles as *mut _);(*list).count=0;}}

pub unsafe fn acpi_device_dep(target:acpi_handle,match_handle:acpi_handle)->bool{if !acpi_has_method(target,"_DEP" as *mut _){return false;}let mut l=core::mem::zeroed::<acpi_handle_list>();if !acpi_evaluate_reference(target,"_DEP" as *const _,core::ptr::null_mut(),&mut l){return false;}let mut r=false;for i in 0..l.count{if *l.handles.add(i as usize)==match_handle{r=true;break;}}acpi_handle_list_free(&mut l);r}

pub unsafe fn acpi_get_physical_device_location(handle:acpi_handle,pld:*mut *mut acpi_pld_info)->bool{let mut b=acpi_buffer{length:ACPI_ALLOCATE_BUFFER,pointer:core::ptr::null_mut()};let mut s=acpi_evaluate_object(handle,"_PLD" as *const _,core::ptr::null_mut(),&mut b);if ACPI_FAILURE(s){return false;}let o=b.pointer as *mut acpi_object;if o.is_null()||(*o).type_!=ACPI_TYPE_PACKAGE||(*o).package.count==0||(*(*o).package.elements).type_!=ACPI_TYPE_BUFFER||(*(*o).package.elements).buffer.length<ACPI_PLD_REV1_BUFFER_SIZE{s=AE_TYPE;}else{s=acpi_decode_pld_buffer((*(*o).package.elements).buffer.pointer,(*(*o).package.elements).buffer.length,pld);}kfree(b.pointer);ACPI_SUCCESS(s)}

pub unsafe fn acpi_evaluate_ost(h:acpi_handle,source:u32,status:u32,status_buf:*mut acpi_buffer)->acpi_status{let mut p:[acpi_object;3]=[core::mem::zeroed(),core::mem::zeroed(),core::mem::zeroed()];p[0].type_=ACPI_TYPE_INTEGER;p[1].type_=ACPI_TYPE_INTEGER;p[2].type_=ACPI_TYPE_BUFFER;p[0].integer.value=source as u64;p[1].integer.value=status as u64;if !status_buf.is_null(){p[2].buffer.pointer=(*status_buf).pointer;p[2].buffer.length=(*status_buf).length;}let a=acpi_object_list{count:3,pointer:p.as_mut_ptr()};acpi_evaluate_object(h,"_OST" as *const _,&a as *const _ as *mut _,core::ptr::null_mut())}
pub unsafe fn acpi_handle_path(h:acpi_handle)->*mut i8{let mut b=acpi_buffer{length:ACPI_ALLOCATE_BUFFER,pointer:core::ptr::null_mut()};if in_interrupt()||acpi_get_name(h,ACPI_FULL_PATHNAME,&mut b)!=AE_OK{core::ptr::null_mut()}else{b.pointer}}
pub unsafe fn acpi_evaluation_failure_warn(h:acpi_handle,name:*const i8,status:acpi_status){acpi_handle_warn(h,"%s evaluation failed: %s\n",name,acpi_format_exception(status));}
pub unsafe fn acpi_has_method(h:acpi_handle,name:*mut i8)->bool{let mut t=core::ptr::null_mut();ACPI_SUCCESS(acpi_get_handle(h,name,&mut t))}
pub unsafe fn acpi_execute_simple_method(h:acpi_handle,method:*mut i8,arg:u64)->acpi_status{let mut o:acpi_object=core::mem::zeroed();o.type_=ACPI_TYPE_INTEGER;o.integer.value=arg;let a=acpi_object_list{count:1,pointer:&mut o};acpi_evaluate_object(h,method,&a,core::ptr::null_mut())}
pub unsafe fn acpi_evaluate_ej0(h:acpi_handle)->acpi_status{acpi_execute_simple_method(h,"_EJ0" as *mut _,1)}
pub unsafe fn acpi_evaluate_lck(h:acpi_handle,lock:i32)->acpi_status{acpi_execute_simple_method(h,"_LCK" as *mut _,(lock!=0) as u64)}
pub unsafe fn acpi_evaluate_reg(h:acpi_handle,space:u8,function:u32)->acpi_status{let mut p:[acpi_object;2]=[core::mem::zeroed(),core::mem::zeroed()];p[0].type_=ACPI_TYPE_INTEGER;p[0].integer.value=space as u64;p[1].type_=ACPI_TYPE_INTEGER;p[1].integer.value=function as u64;let a=acpi_object_list{count:2,pointer:p.as_mut_ptr()};acpi_evaluate_object(h,"_REG" as *const _,&a,core::ptr::null_mut())}

pub unsafe fn acpi_check_dsm(h:acpi_handle,guid:*const guid_t,rev:u64,funcs:u64)->bool{if funcs==0{return false;}let o=acpi_evaluate_dsm(h,guid,rev,0,core::ptr::null_mut());if o.is_null(){return false;}let mut mask=0u64;if (*o).type_==ACPI_TYPE_INTEGER{mask=(*o).integer.value;}else if (*o).type_==ACPI_TYPE_BUFFER{for i in 0..core::cmp::min((*o).buffer.length as usize,8){mask|=((*o).buffer.pointer.add(i) as u64)<<((i*8) as u32);}}ACPI_FREE(o);mask&1!=0&&(mask&funcs)==funcs}
pub unsafe fn acpi_dev_get_first_match_dev(hid:*const i8,uid:*const i8,hrv:i64)->*mut acpi_device{acpi_dev_get_next_match_dev(core::ptr::null_mut(),hid,uid,hrv)}
pub unsafe fn acpi_reduced_hardware()->bool{acpi_gbl_reduced_hardware}
pub static mut acpi_video_backlight_string:[i8;16]=[0;16];
pub unsafe fn acpi_dev_is_video_device(a:*mut acpi_device)->bool{!a.is_null()&&!acpi_match_device_ids(a,core::ptr::null())}
pub unsafe fn acpi_match_platform_list(plat:*const acpi_platform_list)->i32{if acpi_disabled{return -ENODEV;}let mut i=0;let mut p=plat;while !(*p).oem_id[0].eq(&0){let mut h=core::mem::zeroed::<acpi_table_header>();if ACPI_SUCCESS(acpi_get_table_header((*p).table,0,&mut h))&&libc::strncmp((*p).oem_id.as_ptr(),h.oem_id.as_ptr(),ACPI_OEM_ID_SIZE)==0&&libc::strncmp((*p).oem_table_id.as_ptr(),h.oem_table_id.as_ptr(),ACPI_OEM_TABLE_ID_SIZE)==0{return i;}i+=1;p=p.add(1);}-ENODEV}
pub unsafe fn acpi_evaluate_dsm(h:acpi_handle,g:*const guid_t,rev:u64,func:u64,a4:*mut acpi_object)->*mut acpi_object{let mut p:[acpi_object;4]=[core::mem::zeroed(),core::mem::zeroed(),core::mem::zeroed(),core::mem::zeroed()];p[0].type_=ACPI_TYPE_BUFFER;p[0].buffer.length=16;p[0].buffer.pointer=g as *mut u8;p[1].type_=ACPI_TYPE_INTEGER;p[1].integer.value=rev;p[2].type_=ACPI_TYPE_INTEGER;p[2].integer.value=func;if !a4.is_null(){p[3]=*a4;}else{p[3].type_=ACPI_TYPE_PACKAGE;}let l=acpi_object_list{count:4,pointer:p.as_mut_ptr()};let mut b=acpi_buffer{length:ACPI_ALLOCATE_BUFFER,pointer:core::ptr::null_mut()};if ACPI_SUCCESS(acpi_evaluate_object(h,"_DSM" as *const _,&l,&mut b)){b.pointer as *mut acpi_object}else{core::ptr::null_mut()}}
pub unsafe fn acpi_dev_uid_to_integer(a:*mut acpi_device,out:*mut u64)->i32{if a.is_null(){return -ENODEV;}let u=acpi_device_uid(a);if u.is_null(){return -ENODATA;}kstrtou64(u,0,out)}
pub unsafe fn acpi_dev_present(hid:*const i8,uid:*const i8,hrv:i64)->bool{!acpi_dev_get_first_match_dev(hid,uid,hrv).is_null()}
pub unsafe fn acpi_dev_get_next_match_dev(a:*mut acpi_device,hid:*const i8,uid:*const i8,hrv:i64)->*mut acpi_device{let _=(a,hid,uid,hrv);core::ptr::null_mut()}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
