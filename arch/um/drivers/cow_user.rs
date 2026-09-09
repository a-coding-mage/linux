// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2007 Jeff Dike (jdike@{linux.intel,addtoit}.com)

// C dependencies supplied by the surrounding build are intentionally external.
use core::mem::{offset_of, size_of};
use core::ffi::c_void;

type __u32 = u32;
type __s32 = i32;
type __u64 = u64;
type __s64 = i64;
type time32_t = __u32;

const PATH_LEN_V1: usize = 256;
const PATH_LEN_V3: usize = 4096;
const PATH_LEN_V2: usize = PATH_LEN_V3;
const COW_BITMAP: u32 = 0;
const COW_MAGIC: u32 = 0x4f4f4f4d;
const COW_VERSION: i32 = 3;

#[repr(C, packed)]
pub struct cow_header_v1 { pub magic: __s32, pub version: __s32, pub backing_file: [u8; PATH_LEN_V1], pub mtime: time32_t, pub size: __u64, pub sectorsize: __s32 }
#[repr(C, packed)]
pub struct cow_header_v2 { pub magic: __u32, pub version: __u32, pub backing_file: [u8; PATH_LEN_V2], pub mtime: time32_t, pub size: __u64, pub sectorsize: __s32 }
#[repr(C, packed)]
pub struct cow_header_v3 { pub magic: __u32, pub version: __u32, pub mtime: __u32, pub size: __u64, pub sectorsize: __u32, pub alignment: __u32, pub cow_format: __u32, pub backing_file: [u8; PATH_LEN_V3] }
#[repr(C)]
pub struct cow_header_v3_broken { pub magic: __u32, pub version: __u32, pub mtime: __s64, pub size: __u64, pub sectorsize: __u32, pub alignment: __u32, pub cow_format: __u32, pub backing_file: [u8; PATH_LEN_V3] }
#[repr(C)]
pub union cow_header { pub v1: cow_header_v1, pub v2: cow_header_v2, pub v3: cow_header_v3, pub v3_b: cow_header_v3_broken }

extern "C" {
    fn kernel_strrchr(s: *const u8, c: i32) -> *mut u8;
    fn getcwd(buf: *mut u8, size: usize) -> *mut u8;
    fn chdir(path: *const u8) -> i32;
    fn strlen(s: *const u8) -> usize;
    fn strcat(dst: *mut u8, src: *const u8) -> *mut u8;
    fn strcpy(dst: *mut u8, src: *const u8) -> *mut u8;
    fn pread(fd: i32, buf: *mut u8, len: usize, offset: i64) -> isize;
    fn __errno_location() -> *mut i32;
    fn cow_printf(fmt: *const u8, ...);
    fn cow_seek_file(fd: i32, offset: u64) -> i32;
    fn cow_malloc(size: usize) -> *mut c_void;
    fn cow_free(p: *mut c_void);
    fn cow_write_file(fd: i32, buf: *const c_void, len: usize) -> i32;
    fn os_file_modtime(path: *const u8, mtime: *mut i64) -> i32;
    fn cow_file_size(path: *const u8, size: *mut u64) -> i32;
    fn cow_strdup(s: *const u8) -> *mut u8;
}

#[inline] fn div_round(x: u64, len: u64) -> u64 { (x + len - 1) / len }
#[inline] fn round_up(x: u64, align: u64) -> u64 { div_round(x, align) * align }

#[no_mangle]
pub unsafe extern "C" fn cow_sizes(version: i32, size: __u64, sectorsize: i32, align: i32, bitmap_offset: i32, bitmap_len_out: *mut usize, data_offset_out: *mut i32) {
    if version < 3 {
        *bitmap_len_out = ((size + sectorsize as u64 - 1) / (8 * sectorsize as u64)) as usize;
        *data_offset_out = ((bitmap_offset as u64 + *bitmap_len_out as u64 + sectorsize as u64 - 1) / sectorsize as u64 * sectorsize as u64) as i32;
    } else {
        *bitmap_len_out = div_round(div_round(size, sectorsize as u64), 8) as usize;
        *data_offset_out = round_up(bitmap_offset as u64 + *bitmap_len_out as u64, align as u64) as i32;
    }
}

unsafe fn absolutize(to: *mut u8, size: i32, from: *mut u8) -> i32 {
    let mut cwd = [0u8; 256];
    if getcwd(cwd.as_mut_ptr(), cwd.len()).is_null() { cow_printf(b"absolutize : unable to get cwd - errno = %d\n\0".as_ptr(), *__errno_location()); return -1; }
    let slash = kernel_strrchr(from, b'/' as i32);
    if !slash.is_null() {
        *slash = 0;
        if chdir(from) != 0 { *slash = b'/'; cow_printf(b"absolutize : Can't cd to '%s' - errno = %d\n\0".as_ptr(), from, *__errno_location()); return -1; }
        *slash = b'/';
        if getcwd(to, size as usize).is_null() { return -1; }
        let remaining = size as usize - strlen(to);
        if strlen(slash) + 1 > remaining { return -1; }
        strcat(to, slash);
    } else {
        if strlen(cwd.as_ptr()) + 1 + strlen(from) + 1 > size as usize { return -1; }
        strcpy(to, cwd.as_ptr()); strcat(to, b"/\0".as_ptr()); strcat(to, from);
    }
    if chdir(cwd.as_ptr()) != 0 { return -1; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn file_reader(offset: __u64, buf: *mut u8, len: i32, arg: *mut c_void) -> i32 { pread(*(arg as *mut i32), buf, len as usize, offset as i64) as i32 }

#[no_mangle]
pub unsafe extern "C" fn write_cow_header(cow_file: *mut u8, fd: i32, backing_file: *mut u8, sectorsize: i32, alignment: i32, size: *mut u64) -> i32 {
    let mut err = cow_seek_file(fd, 0); if err < 0 { return err; }
    let h = cow_malloc(size_of::<cow_header_v3>()) as *mut cow_header_v3; if h.is_null() { return -12; }
    (*h).magic = COW_MAGIC.to_be(); (*h).version = (COW_VERSION as u32).to_be();
    if strlen(backing_file) > PATH_LEN_V3 - 1 { cow_free(h as *mut c_void); return -22; }
    if absolutize((*h).backing_file.as_mut_ptr(), PATH_LEN_V3 as i32, backing_file) != 0 { cow_free(h as *mut c_void); return -22; }
    let mut mt = 0i64; err = os_file_modtime((*h).backing_file.as_ptr(), &mut mt); if err < 0 { cow_free(h as *mut c_void); return err; }
    err = cow_file_size((*h).backing_file.as_ptr(), size); if err < 0 { cow_free(h as *mut c_void); return err; }
    (*h).mtime = (mt as u32).to_be(); (*h).size = (*size).to_be(); (*h).sectorsize = (sectorsize as u32).to_be(); (*h).alignment = (alignment as u32).to_be(); (*h).cow_format = COW_BITMAP;
    err = cow_write_file(fd, h as *const c_void, size_of::<cow_header_v3>()); if err != size_of::<cow_header_v3>() as i32 { cow_free(h as *mut c_void); return err; } cow_free(h as *mut c_void); let _ = cow_file; 0
}

#[no_mangle]
pub unsafe extern "C" fn read_cow_header(reader: Option<unsafe extern "C" fn(__u64,*mut u8,i32,*mut c_void)->i32>, arg: *mut c_void, version_out: *mut __u32, backing_file_out: *mut *mut u8, mtime_out: *mut i64, size_out: *mut u64, sectorsize_out: *mut i32, align_out: *mut __u32, bitmap_offset_out: *mut i32) -> i32 {
    let h = cow_malloc(size_of::<cow_header>()) as *mut cow_header; if h.is_null() { return -12; }
    let n = reader.unwrap()(0, h as *mut u8, size_of::<cow_header>() as i32, arg); if n < offset_of!(cow_header_v1, backing_file) as i32 { cow_free(h as *mut c_void); return -22; }
    let raw = &(*h).v1; let magic = raw.magic as u32; let version = if magic == COW_MAGIC { raw.version as u32 } else if magic == COW_MAGIC.to_be() { raw.version.to_be() } else { cow_free(h as *mut c_void); return -22; }; *version_out = version;
    let (file, mtime, size, sector, align, bo): (*const u8, i64, u64, i32, u32, i32) = if version == 1 { if n < size_of::<cow_header_v1>() as i32 { cow_free(h as *mut c_void); return -22; } let x=&(*h).v1; (x.backing_file.as_ptr(), x.mtime as i64, x.size, x.sectorsize, x.sectorsize as u32, size_of::<cow_header_v1>() as i32) } else if version == 2 { if n < size_of::<cow_header_v2>() as i32 { cow_free(h as *mut c_void); return -22; } let x=&(*h).v2; (x.backing_file.as_ptr(), x.mtime.to_be() as i64, x.size.to_be(), x.sectorsize.to_be() as i32, x.sectorsize.to_be(), size_of::<cow_header_v2>() as i32) } else if version == 3 && (*(*h).v3.backing_file.as_ptr().cast::<i32>()) != 0 { let x=&(*h).v3; if n < size_of::<cow_header_v3>() as i32 { cow_free(h as *mut c_void); return -22; } let a=x.alignment.to_be(); (x.backing_file.as_ptr(), x.mtime.to_be() as i64, x.size.to_be(), x.sectorsize.to_be() as i32, a, round_up(size_of::<cow_header_v3>() as u64,a as u64) as i32) } else if version == 3 { let x=&(*h).v3_b; if n < size_of::<cow_header_v3_broken>() as i32 { cow_free(h as *mut c_void); return -22; } let a=x.alignment.to_be(); (x.backing_file.as_ptr(), x.mtime.to_be() as u32 as i64, x.size.to_be(), x.sectorsize.to_be() as i32, a, round_up(size_of::<cow_header_v3_broken>() as u64,a as u64) as i32) } else { cow_free(h as *mut c_void); return -22; };
    *mtime_out=mtime; *size_out=size; *sectorsize_out=sector; *align_out=align; *bitmap_offset_out=bo; *backing_file_out=cow_strdup(file); let r=if (*backing_file_out).is_null(){-12}else{0}; cow_free(h as *mut c_void); r
}

#[no_mangle]
pub unsafe extern "C" fn init_cow_file(fd:i32,cow_file:*mut u8,backing_file:*mut u8,sectorsize:i32,alignment:i32,bitmap_offset_out:*mut i32,bitmap_len_out:*mut usize,data_offset_out:*mut i32)->i32 { let mut size=0u64; let mut err=write_cow_header(cow_file,fd,backing_file,sectorsize,alignment,&mut size); if err!=0{return err;} *bitmap_offset_out=round_up(size_of::<cow_header_v3>() as u64,alignment as u64) as i32; cow_sizes(3,size,sectorsize,alignment,*bitmap_offset_out,bitmap_len_out,data_offset_out); err=cow_seek_file(fd,*data_offset_out as u64+size-1); if err<0{return err;} let z=0u8; err=cow_write_file(fd,&z as *const u8 as *const c_void,1); if err!=1 {if err>=0{-22}else{err}} else {0} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
