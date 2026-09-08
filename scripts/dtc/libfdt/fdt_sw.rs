// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
/*
 * libfdt - Flat Device Tree manipulation
 * Copyright (C) 2006 David Gibson, IBM Corporation.
 */

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    fn fdt_magic(fdt: *const c_void) -> u32;
    fn fdt_last_comp_version(fdt: *const c_void) -> u32;
    fn fdt_off_dt_strings(fdt: *const c_void) -> c_int;
    fn fdt_totalsize(fdt: *const c_void) -> c_int;
    fn fdt_off_dt_struct(fdt: *const c_void) -> c_int;
    fn fdt_size_dt_struct(fdt: *const c_void) -> c_int;
    fn fdt_size_dt_strings(fdt: *const c_void) -> c_int;
    fn fdt_set_magic(fdt: *mut c_void, value: u32);
    fn fdt_set_version(fdt: *mut c_void, value: u32);
    fn fdt_set_last_comp_version(fdt: *mut c_void, value: u32);
    fn fdt_set_totalsize(fdt: *mut c_void, value: c_int);
    fn fdt_set_off_mem_rsvmap(fdt: *mut c_void, value: c_int);
    fn fdt_set_off_dt_struct(fdt: *mut c_void, value: c_int);
    fn fdt_set_off_dt_strings(fdt: *mut c_void, value: c_int);
    fn fdt_set_size_dt_struct(fdt: *mut c_void, value: c_int);
    fn fdt_set_size_dt_strings(fdt: *mut c_void, value: c_int);
    fn fdt_offset_ptr_w_(fdt: *mut c_void, offset: c_int) -> *mut c_void;
    fn fdt_next_tag(fdt: *mut c_void, offset: c_int, nextoffset: *mut c_int) -> u32;
    fn fdt_find_string_(strtab: *const c_char, tabsize: c_int, s: *const c_char) -> *const c_char;
    fn can_assume_valid_input() -> bool;
    fn can_assume_valid_dtb() -> bool;
}

const FDT_MAGIC: u32 = 0xd00dfeed;
const FDT_SW_MAGIC: u32 = 0xffffffff;
const FDT_LAST_SUPPORTED_VERSION: u32 = 17;
const FDT_LAST_COMPATIBLE_VERSION: u32 = 16;
const FDT_CREATE_FLAG_NO_NAME_DEDUP: u32 = 1;
const FDT_CREATE_FLAGS_ALL: u32 = FDT_CREATE_FLAG_NO_NAME_DEDUP;
const FDT_BEGIN_NODE: u32 = 1;
const FDT_END_NODE: u32 = 2;
const FDT_PROP: u32 = 3;
const FDT_END: u32 = 9;
const FDT_TAGSIZE: usize = 4;
const FDT_ERR_NOSPACE: c_int = 3;
const FDT_ERR_BADMAGIC: c_int = 9;
const FDT_ERR_BADSTATE: c_int = 5;
const FDT_ERR_BADFLAGS: c_int = 8;
const FDT_ERR_INTERNAL: c_int = 14;

#[repr(C)] struct FdtReserveEntry { address: u64, size: u64 }
#[repr(C)] struct FdtNodeHeader { tag: u32, name: [c_char; 0] }
#[repr(C)] struct FdtProperty { tag: u32, len: u32, nameoff: u32, data: [u8; 0] }

unsafe fn probe(fdt: *mut c_void) -> c_int {
    if !can_assume_valid_input() {
        if fdt_magic(fdt) == FDT_MAGIC { return -FDT_ERR_BADSTATE; }
        if fdt_magic(fdt) != FDT_SW_MAGIC { return -FDT_ERR_BADMAGIC; }
    }
    0
}
unsafe fn probe_memrsv(fdt: *mut c_void) -> c_int {
    let err = probe(fdt); if err != 0 { return err; }
    if !can_assume_valid_input() && fdt_off_dt_strings(fdt) != 0 { return -FDT_ERR_BADSTATE; } 0
}
unsafe fn probe_struct(fdt: *mut c_void) -> c_int {
    let err = probe(fdt); if err != 0 { return err; }
    if !can_assume_valid_input() && fdt_off_dt_strings(fdt) != fdt_totalsize(fdt) { return -FDT_ERR_BADSTATE; } 0
}
unsafe fn sw_flags(fdt: *mut c_void) -> u32 { fdt_last_comp_version(fdt) }
unsafe fn grab_space(fdt: *mut c_void, len: usize) -> *mut c_void {
    let offset = fdt_size_dt_struct(fdt) as usize;
    let spaceleft = fdt_totalsize(fdt) as usize - fdt_off_dt_struct(fdt) as usize - fdt_size_dt_strings(fdt) as usize;
    if offset.checked_add(len).is_none() || offset + len > spaceleft { return core::ptr::null_mut(); }
    fdt_set_size_dt_struct(fdt, (offset + len) as c_int); fdt_offset_ptr_w_(fdt, offset as c_int)
}

#[no_mangle] pub unsafe extern "C" fn fdt_create_with_flags(buf: *mut c_void, bufsize: c_int, flags: u32) -> c_int {
    let hdrsize = (core::mem::size_of::<[u8; 40]>() + 7) & !7;
    if bufsize < hdrsize as c_int { return -FDT_ERR_NOSPACE; }
    if flags & !FDT_CREATE_FLAGS_ALL != 0 { return -FDT_ERR_BADFLAGS; }
    core::ptr::write_bytes(buf as *mut u8, 0, bufsize as usize);
    fdt_set_magic(buf, FDT_SW_MAGIC); fdt_set_version(buf, FDT_LAST_SUPPORTED_VERSION); fdt_set_last_comp_version(buf, flags);
    fdt_set_totalsize(buf, bufsize); fdt_set_off_mem_rsvmap(buf, hdrsize as c_int); fdt_set_off_dt_struct(buf, hdrsize as c_int); fdt_set_off_dt_strings(buf, 0); 0
}
#[no_mangle] pub unsafe extern "C" fn fdt_create(buf: *mut c_void, bufsize: c_int) -> c_int { fdt_create_with_flags(buf, bufsize, 0) }

#[no_mangle] pub unsafe extern "C" fn fdt_resize(fdt: *mut c_void, buf: *mut c_void, bufsize: c_int) -> c_int {
    let err = probe(fdt); if err != 0 { return err; } if bufsize < 0 { return -FDT_ERR_NOSPACE; }
    let headsize = fdt_off_dt_struct(fdt) as usize + fdt_size_dt_struct(fdt) as usize; let tailsize = fdt_size_dt_strings(fdt) as usize;
    if !can_assume_valid_dtb() && headsize + tailsize > fdt_totalsize(fdt) as usize { return -FDT_ERR_INTERNAL; }
    if headsize + tailsize > bufsize as usize { return -FDT_ERR_NOSPACE; }
    let oldtail = (fdt as *mut u8).add(fdt_totalsize(fdt) as usize - tailsize); let newtail = (buf as *mut u8).add(bufsize as usize - tailsize);
    if (buf as usize) <= (fdt as usize) { core::ptr::copy(fdt, buf, headsize); core::ptr::copy(oldtail, newtail, tailsize); } else { core::ptr::copy(oldtail, newtail, tailsize); core::ptr::copy(fdt, buf, headsize); }
    fdt_set_totalsize(buf, bufsize); if fdt_off_dt_strings(buf) != 0 { fdt_set_off_dt_strings(buf, bufsize); } 0
}

#[no_mangle] pub unsafe extern "C" fn fdt_add_reservemap_entry(fdt: *mut c_void, addr: u64, size: u64) -> c_int {
    let err = probe_memrsv(fdt); if err != 0 { return err; } let offset = fdt_off_dt_struct(fdt) as usize;
    if offset + core::mem::size_of::<FdtReserveEntry>() > fdt_totalsize(fdt) as usize { return -FDT_ERR_NOSPACE; }
    let re = (fdt as *mut u8).add(offset) as *mut FdtReserveEntry; (*re).address = addr.to_be(); (*re).size = size.to_be(); fdt_set_off_dt_struct(fdt, (offset + 16) as c_int); 0
}
#[no_mangle] pub unsafe extern "C" fn fdt_finish_reservemap(fdt: *mut c_void) -> c_int { let err = fdt_add_reservemap_entry(fdt, 0, 0); if err != 0 { return err; } fdt_set_off_dt_strings(fdt, fdt_totalsize(fdt)); 0 }

unsafe fn libc_strlen(s: *const c_char) -> usize { let mut n = 0; while *s.add(n) != 0 { n += 1; } n }
#[no_mangle] pub unsafe extern "C" fn fdt_begin_node(fdt: *mut c_void, name: *const c_char) -> c_int { let e=probe_struct(fdt); if e!=0{return e;} let n=libc_strlen(name)+1; let p=grab_space(fdt,8+((n+3)&!3)); if p.is_null(){return -FDT_ERR_NOSPACE;} *(p as *mut u32)=FDT_BEGIN_NODE.to_be(); core::ptr::copy_nonoverlapping(name as *const u8,(p as *mut u8).add(4),n);0 }
#[no_mangle] pub unsafe extern "C" fn fdt_end_node(fdt:*mut c_void)->c_int {let e=probe_struct(fdt);if e!=0{return e;}let p=grab_space(fdt,4);if p.is_null(){return -FDT_ERR_NOSPACE;}*(p as *mut u32)=FDT_END_NODE.to_be();0}
unsafe fn add_string(fdt:*mut c_void,s:*const c_char)->c_int{let base=(fdt as *mut u8).add(fdt_totalsize(fdt)as usize);let sz=fdt_size_dt_strings(fdt)as usize;let n=libc_strlen(s)+1;let off=sz+n;let top=fdt_off_dt_struct(fdt)as usize+fdt_size_dt_struct(fdt)as usize;if fdt_totalsize(fdt)as usize-off<top{return 0;}core::ptr::copy_nonoverlapping(s as *const u8,base.sub(off),n);fdt_set_size_dt_strings(fdt,(sz+n)as c_int);-(off as c_int)}
unsafe fn del_string(fdt:*mut c_void,s:*const c_char){fdt_set_size_dt_strings(fdt,fdt_size_dt_strings(fdt)-(libc_strlen(s)+1)as c_int)}
#[no_mangle] pub unsafe extern "C" fn fdt_property_placeholder(fdt:*mut c_void,name:*const c_char,len:c_int,valp:*mut *mut c_void)->c_int{let e=probe_struct(fdt);if e!=0{return e;}let no=add_string(fdt,name);if no==0{return -FDT_ERR_NOSPACE;}let p=grab_space(fdt,12+((len as usize+3)&!3));if p.is_null(){del_string(fdt,name);return -FDT_ERR_NOSPACE;}let q=p as *mut FdtProperty;(*q).tag=FDT_PROP.to_be();(*q).nameoff=(no as u32).to_be();(*q).len=(len as u32).to_be();*valp=(p as *mut u8).add(12)as *mut c_void;0}
#[no_mangle] pub unsafe extern "C" fn fdt_property(fdt:*mut c_void,name:*const c_char,val:*const c_void,len:c_int)->c_int{let mut p=core::ptr::null_mut();let e=fdt_property_placeholder(fdt,name,len,&mut p);if e!=0{return e;}core::ptr::copy_nonoverlapping(val as *const u8,p as *mut u8,len as usize);0}
#[no_mangle] pub unsafe extern "C" fn fdt_finish(fdt:*mut c_void)->c_int{let e=probe_struct(fdt);if e!=0{return e;}let x=grab_space(fdt,4);if x.is_null(){return -FDT_ERR_NOSPACE;}*(x as *mut u32)=FDT_END.to_be();let old=fdt_totalsize(fdt)as usize-fdt_size_dt_strings(fdt)as usize;let new=fdt_off_dt_struct(fdt)as usize+fdt_size_dt_struct(fdt)as usize;let sz=fdt_size_dt_strings(fdt)as usize;core::ptr::copy((fdt as *mut u8).add(old),(fdt as *mut u8).add(new),sz);fdt_set_off_dt_strings(fdt,new as c_int);fdt_set_totalsize(fdt,(new+sz)as c_int);fdt_set_last_comp_version(fdt,FDT_LAST_COMPATIBLE_VERSION);fdt_set_magic(fdt,FDT_MAGIC);0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
