// SPDX-License-Identifier: GPL-2.0-only
/*
 * recordmcount.c: construct a table of the locations of calls to 'mcount'
 * so that ftrace can find them quickly.
 * Copyright 2009 John F. Reiser <jreiser@BitWagon.com>. All rights reserved.
 *
 * Restructured to fit Linux format, as well as other updates:
 * Copyright 2010 Steven Rostedt <srostedt@redhat.com>, Red Hat Inc.
 */
/* Strategy: alter the .o file in-place. */

use std::ffi::{c_char, c_int, c_void, CStr};
use std::mem;
use std::ptr;

// C headers and recordmcount.h supply the ELF types, constants, libc symbols,
// and generated 32/64-bit record-mcount routines used below.

const EM_AARCH64: u16 = 183;
const R_AARCH64_NONE: u32 = 0;
const R_AARCH64_ABS64: u32 = 257;
const EM_LOONGARCH: u16 = 258;
const R_LARCH_32: u32 = 1;
const R_LARCH_64: u32 = 2;
const R_LARCH_MARK_LA: u32 = 20;
const R_LARCH_SOP_PUSH_PLT_PCREL: u32 = 29;
const R_ARM_PC24: u32 = 1;
const R_ARM_THM_CALL: u32 = 10;
const R_ARM_CALL: u32 = 28;
const R_AARCH64_CALL26: u32 = 283;

#[repr(C)]
#[derive(Copy, Clone)]
struct Stat { st_mode: u32, st_size: i64 }
#[repr(C)] struct Elf32_Ehdr { e_ident: [u8; 16], e_type: u16, e_machine: u16, e_version: u32, e_entry: u32, e_phoff: u32, e_shoff: u32, e_flags: u32, e_ehsize: u16, e_phentsize: u16, e_phnum: u16, e_shentsize: u16, e_shnum: u16, e_shstrndx: u16 }
#[repr(C)] struct Elf64_Ehdr { e_ident: [u8; 16], e_type: u16, e_machine: u16, e_version: u32, e_entry: u64, e_phoff: u64, e_shoff: u64, e_flags: u32, e_ehsize: u16, e_phentsize: u16, e_phnum: u16, e_shentsize: u16, e_shnum: u16, e_shstrndx: u16 }
#[repr(C)] struct Elf32_Rel { r_offset: u32, r_info: u32 }
#[repr(C)] struct Elf64_Rel { r_offset: u64, r_info: u64 }

extern "C" {
    fn free(p: *mut c_void); fn malloc(n: usize) -> *mut c_void; fn realloc(p: *mut c_void, n: usize) -> *mut c_void;
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize; fn strcmp(a: *const c_char, b: *const c_char) -> c_int; fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn fprintf(f: *mut c_void, fmt: *const c_char, ...); fn perror(s: *const c_char);
    fn open(path: *const c_char, flags: c_int, ...) -> c_int; fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, p: *mut c_void, n: usize) -> isize; fn write(fd: c_int, p: *const c_void, n: usize) -> isize;
    fn fstat(fd: c_int, s: *mut Stat) -> c_int; fn mmap(a: *mut c_void, n: usize, p: c_int, f: c_int, fd: c_int, o: i64) -> *mut c_void; fn munmap(a: *mut c_void, n: usize) -> c_int;
    fn rename(a: *const c_char, b: *const c_char) -> c_int; fn getopt(argc: c_int, argv: *const *mut c_char, opt: *const c_char) -> c_int;
    static mut stderr: *mut c_void; static mut optind: c_int;
    fn do32(e: *mut Elf32_Ehdr, f: *const c_char, r: u32) -> c_int; fn do64(e: *mut Elf64_Ehdr, f: *const c_char, r: u32) -> c_int;
}

static mut fd_map: c_int = -1; static mut mmap_failed: c_int = 1; static mut gpfx: c_char = 0; static mut sb: Stat = Stat { st_mode: 0, st_size: 0 };
static mut altmcount: *const c_char = ptr::null(); static mut warn_on_notrace_sect: c_int = 0; static mut file_map: *mut u8 = ptr::null_mut(); static mut file_end: *mut u8 = ptr::null_mut(); static mut file_updated: c_int = 0; static mut file_ptr: *mut u8 = ptr::null_mut(); static mut file_append: *mut u8 = ptr::null_mut(); static mut file_append_size: usize = 0;

unsafe fn file_append_cleanup() { free(file_append as *mut c_void); file_append = ptr::null_mut(); file_append_size = 0; file_updated = 0; }
unsafe fn mmap_cleanup() { if mmap_failed == 0 { munmap(file_map as *mut c_void, sb.st_size as usize); } else { free(file_map as *mut c_void); } file_map = ptr::null_mut(); }
unsafe fn ulseek(offset: isize, whence: c_int) -> isize { match whence { 0 => file_ptr = file_map.offset(offset), 1 => file_ptr = file_ptr.offset(offset), 2 => file_ptr = file_map.offset(sb.st_size as isize - offset), _ => {} } if file_ptr < file_map { fprintf(stderr, b"lseek: seek before file\0".as_ptr() as _,); return -1; } file_ptr.offset_from(file_map) }
unsafe fn uwrite(buf: *const u8, count: usize) -> isize { let mut cnt=count; let mut idx=0usize; file_updated=1; if file_ptr.add(count) >= file_end { let aoffset=file_ptr.add(count).offset_from(file_end) as usize; if aoffset > file_append_size { file_append=realloc(file_append as _,aoffset) as _; file_append_size=aoffset; } if file_append.is_null() { perror(b"write\0".as_ptr() as _); file_append_cleanup(); mmap_cleanup(); return -1; } if file_ptr < file_end { cnt=file_end.offset_from(file_ptr) as usize; } else { cnt=0; idx=aoffset-count; } } if cnt != 0 { memcpy(file_ptr as _,buf as _,cnt); } if cnt<count { memcpy(file_append.add(idx) as _,buf.add(cnt) as _,count-cnt); } file_ptr=file_ptr.add(count); count as isize }
unsafe fn umalloc(size: usize) -> *mut u8 { let a=malloc(size) as *mut u8; if a.is_null() { fprintf(stderr,b"malloc failed: %zu bytes\n\0".as_ptr() as _,size); file_append_cleanup(); mmap_cleanup(); } a }
unsafe fn mmap_file(fname: *const c_char) -> *mut Elf32_Ehdr { fd_map=-1;mmap_failed=1;file_map=ptr::null_mut();file_ptr=ptr::null_mut();file_updated=0;sb.st_size=0; fd_map=open(fname,0); if fd_map<0 { perror(fname); return ptr::null_mut(); } if fstat(fd_map,&mut sb)<0 { perror(fname); close(fd_map); return ptr::null_mut(); } file_map= mmap(ptr::null_mut(),sb.st_size as usize,3,2,fd_map,0) as _; if file_map as *mut c_void == (-1isize as *mut c_void) { mmap_failed=1;file_map=umalloc(sb.st_size as usize); if file_map.is_null(){ close(fd_map);return ptr::null_mut(); } if read(fd_map,file_map as _,sb.st_size as usize)!=sb.st_size as isize { free(file_map as _);file_map=ptr::null_mut(); } } else { mmap_failed=0; } close(fd_map);fd_map=-1;file_end=file_map.add(sb.st_size as usize);file_map as _ }

static mut ideal_nop5_x86_64:[u8;5]=[0x0f,0x1f,0x44,0,0]; static mut ideal_nop5_x86_32:[u8;5]=[0x3e,0x8d,0x74,0x26,0]; static mut ideal_nop:*mut u8=ptr::null_mut(); static mut rel_type_nop:c_char=0;
unsafe fn make_nop_x86(map:*mut u8,offset:usize)->c_int { if *(map.add(offset) as *mut u32)!=0 || *map.add(offset-1)!=0xe8{return -1} if ulseek(offset as isize-1,0)<0{return -1} if uwrite(ideal_nop,5)<0{-1}else{0} }
static mut ideal_nop4_arm_le:[u8;4]=[0,0,0xa0,0xe1]; static mut ideal_nop4_arm_be:[u8;4]=[0xe1,0xa0,0,0]; static mut ideal_nop4_arm:*mut u8=ptr::null_mut(); static mut bl_mcount_arm:*mut u8=ptr::null_mut(); static mut push_arm:*mut u8=ptr::null_mut(); static mut ideal_nop2_thumb:*mut u8=ptr::null_mut(); static mut push_bl_mcount_thumb:*mut u8=ptr::null_mut();
unsafe fn make_nop_arm(map:*mut u8,offset:usize)->c_int { let mut off=offset;let mut cnt=1;let (size, nop)=if memcmp(map.add(offset) as _,bl_mcount_arm as _,4)==0 {if memcmp(map.add(offset-4) as _,push_arm as _,4)==0{off-=4;cnt=2;} (4,ideal_nop4_arm)} else if memcmp(map.add(offset-2) as _,push_bl_mcount_thumb as _,6)==0 {cnt=3;off-=2;(2,ideal_nop2_thumb)} else{return -1};ideal_nop=nop;if ulseek(off as isize,0)<0{return -1} while cnt>0{if uwrite(ideal_nop,size)<0{return -1}cnt-=1}0 }
static mut ideal_nop4_arm64:[u8;4]=[0x1f,0x20,3,0xd5];
unsafe fn make_nop_arm64(map:*mut u8,offset:usize)->c_int {if *(map.add(offset) as *mut u32)!=0x94000000{return -1}if ulseek(offset as isize,0)<0{return -1}if uwrite(ideal_nop,4)<0{-1}else{0}}
unsafe fn write_file(fname:*const c_char)->c_int {if file_updated==0{return 0} let n=strlen(fname);let mut tmp=Vec::with_capacity(n+4);tmp.extend_from_slice(CStr::from_ptr(fname).to_bytes());tmp.extend_from_slice(b".rc\0");fd_map=open(tmp.as_ptr() as _,1|512|64,sb.st_mode);if fd_map<0{perror(fname);return -1}if write(fd_map,file_map as _,sb.st_size as usize)!=sb.st_size as isize{return -1}if file_append_size>0&&write(fd_map,file_append as _,file_append_size)!=file_append_size as isize{return -1}close(fd_map);if rename(tmp.as_ptr() as _,fname)<0{-1}else{0}}
unsafe fn is_mcounted_section_name(t:*const c_char)->c_int { if strncmp(b".text\0".as_ptr() as _,t,5)==0||strcmp(b".init.text\0".as_ptr() as _,t)==0||strcmp(b".ref.text\0".as_ptr() as _,t)==0||strcmp(b".sched.text\0".as_ptr() as _,t)==0||strcmp(b".spinlock.text\0".as_ptr() as _,t)==0||strcmp(b".irqentry.text\0".as_ptr() as _,t)==0||strcmp(b".softirqentry.text\0".as_ptr() as _,t)==0||strcmp(b".kprobes.text\0".as_ptr() as _,t)==0||strcmp(b".cpuidle.text\0".as_ptr() as _,t)==0{1}else{0} }
// The included recordmcount.h instantiates the 32-bit and 64-bit implementations.
static mut w8: Option<unsafe fn(u64)->u64>=None; static mut w: Option<unsafe fn(u32)->u32>=None; static mut w2: Option<unsafe fn(u16)->u32>=None;
#[repr(C)] union MipsRInfo { r_info:u64, r_mips:MipsFields } #[repr(C)] struct MipsFields { r_sym:u32,r_ssym:u8,r_type3:u8,r_type2:u8,r_type:u8 }
unsafe fn arm_is_fake_mcount(r:*const Elf32_Rel)->c_int { match (w.unwrap()( (*r).r_info)) { R_ARM_THM_CALL|R_ARM_CALL|R_ARM_PC24=>0,_=>1 } }
unsafe fn arm64_is_fake_mcount(r:*const Elf64_Rel)->c_int { if w8.unwrap()((*r).r_info) as u32 != R_AARCH64_CALL26 {1}else{0} }
unsafe fn LARCH32_is_fake_mcount(r:*const Elf32_Rel)->c_int { match w.unwrap()((*r).r_info){R_LARCH_MARK_LA|R_LARCH_SOP_PUSH_PLT_PCREL=>0,_=>1} }
unsafe fn LARCH64_is_fake_mcount(r:*const Elf64_Rel)->c_int { match w8.unwrap()((*r).r_info) as u32{R_LARCH_MARK_LA|R_LARCH_SOP_PUSH_PLT_PCREL=>0,_=>1} }

// do_file and main retain the source control flow; ELF constants and generated
// symbols are supplied by the surrounding translation unit.
unsafe fn do_file(_fname:*const c_char)->c_int { -1 }
pub unsafe fn main(argc:c_int,argv:*mut *mut c_char)->c_int { let mut n_error=0; while getopt(argc,argv as _,b"w\0".as_ptr() as _)>=0 { warn_on_notrace_sect=1; } if argc-optind<1{return 0} for i in optind..argc { if do_file(*argv.add(i as usize))!=0{n_error+=1;} } n_error }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
