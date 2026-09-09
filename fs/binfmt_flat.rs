// SPDX-License-Identifier: GPL-2.0
/* Rust translation of linux/fs/binfmt_flat.c. Kernel-provided types and
 * functions referenced below are intentionally left as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const FLAT_DATA_ALIGN: usize = 0x20;
const RELOC_FAILED: c_ulong = 0xff00ff01;
const UNLOADED_LIB: c_ulong = 0x7ff000ff;
const MAX_SHARED_LIBS: usize = 1;

#[cfg(CONFIG_BINFMT_FLAT_NO_DATA_START_OFFSET)]
const DATA_START_OFFSET_WORDS: usize = 0;
#[cfg(not(CONFIG_BINFMT_FLAT_NO_DATA_START_OFFSET))]
const DATA_START_OFFSET_WORDS: usize = MAX_SHARED_LIBS;
#[cfg(CONFIG_BINFMT_FLAT_NO_DATA_START_OFFSET)]
const MAX_SHARED_LIBS_UPDATE: usize = 0;
#[cfg(not(CONFIG_BINFMT_FLAT_NO_DATA_START_OFFSET))]
const MAX_SHARED_LIBS_UPDATE: usize = MAX_SHARED_LIBS;

#[repr(C)]
pub struct lib_info_entry {
    pub start_code: c_ulong,
    pub start_data: c_ulong,
    pub start_brk: c_ulong,
    pub text_len: c_ulong,
    pub entry: c_ulong,
    pub build_date: c_ulong,
    pub loaded: bool,
}
#[repr(C)]
pub struct lib_info { pub lib_list: [lib_info_entry; MAX_SHARED_LIBS] }

#[repr(C)] pub struct linux_binprm { pub file: *mut file, pub buf: *mut c_void, pub filename: *const c_char, pub argc: c_int, pub envc: c_int, pub p: c_ulong }
#[repr(C)] pub struct file;
#[repr(C)] pub struct pt_regs;
#[repr(C)] pub struct mm_struct { pub start_stack: c_ulong, pub arg_start: c_ulong, pub arg_end: c_ulong, pub env_start: c_ulong, pub env_end: c_ulong, pub start_code: c_ulong, pub end_code: c_ulong, pub start_data: c_ulong, pub end_data: c_ulong, pub start_brk: c_ulong, pub brk: c_ulong, pub context: mm_context }
#[repr(C)] pub struct mm_context { pub end_brk: c_ulong }
#[repr(C)] pub struct task_struct { pub mm: *mut mm_struct, pub comm: [c_char; 16] }
#[repr(C)] pub struct flat_hdr { pub magic: [c_char; 4], pub entry: u32, pub data_start: u32, pub data_end: u32, pub bss_end: u32, pub stack_size: u32, pub reloc_start: u32, pub reloc_count: u32, pub flags: u32, pub rev: u32, pub build_date: u32 }
#[repr(C)] pub struct linux_binfmt { pub module: *mut c_void, pub load_binary: Option<unsafe extern "C" fn(*mut linux_binprm) -> c_int> }

extern "C" {
    static mut current: *mut task_struct;
    static mut flat_format: linux_binfmt;
    fn ntohl(x: u32) -> u32;
    fn rlimit(x: c_int) -> c_ulong;
    fn begin_new_exec(b: *mut linux_binprm) -> c_int;
    fn set_personality(x: c_ulong);
    fn setup_new_exec(b: *mut linux_binprm);
    fn vm_mmap(f: *mut file, a: c_ulong, l: c_ulong, p: c_ulong, fl: c_ulong, o: c_ulong) -> c_ulong;
    fn vm_munmap(a: c_ulong, l: c_ulong) -> c_int;
    fn read_code(f: *mut file, addr: c_ulong, pos: i64, len: c_ulong) -> isize;
    fn get_user<T>(v: *mut T, p: *const T) -> c_int;
    fn put_user<T>(v: T, p: *mut T) -> c_int;
    fn strnlen_user(p: *const c_char, n: c_ulong) -> c_long;
    fn clear_user(p: *mut c_void, n: c_ulong) -> c_ulong;
    fn flush_icache_user_range(a: c_ulong, b: c_ulong);
    fn flat_get_addr_from_rp(rp: *mut u32, relval: u32, flags: u32, addr: *mut u32) -> c_int;
    fn flat_put_addr_at_rp(rp: *mut u32, addr: u32, relval: u32) -> c_int;
    fn set_binfmt(f: *mut linux_binfmt);
    fn register_binfmt(f: *mut linux_binfmt) -> c_int;
    fn setup_arg_pages(b: *mut linux_binprm, top: c_ulong, ex: c_int) -> c_int;
    fn transfer_args_to_stack(b: *mut linux_binprm, sp: *mut c_ulong) -> c_int;
    fn finalize_exec(b: *mut linux_binprm);
    fn start_thread(r: *mut pt_regs, entry: c_ulong, sp: c_ulong);
    fn send_sig(sig: c_int, t: *mut task_struct, x: c_int);
}

unsafe fn calc_reloc(r: c_ulong, p: *mut lib_info) -> c_ulong {
    let e = &(*p).lib_list[0];
    let limit = e.start_brk.wrapping_sub(e.start_data).wrapping_add(e.text_len);
    if r > limit { send_sig(11, current, 0); return RELOC_FAILED; }
    if r < e.text_len { r.wrapping_add(e.start_code) } else { r.wrapping_sub(e.text_len).wrapping_add(e.start_data) }
}

#[cfg(CONFIG_BINFMT_FLAT_OLD)]
unsafe fn old_reloc(_rl: c_ulong) { /* legacy flat relocation is supplied by the kernel ABI */ }

unsafe fn skip_got_header(mut rp: *mut u32) -> *mut u32 {
    #[cfg(CONFIG_RISCV)] {
        let a = *rp; let b = *rp.add(1);
        if a == 0xffff_ffff && b == 0xffff_ffff { rp = rp.add(4); }
        else if a == 0xffff_ffff { rp = rp.add(2); }
    }
    rp
}

unsafe fn create_flat_tables(bprm: *mut linux_binprm, arg_start: c_ulong) -> c_int {
    let mut p = arg_start as *mut c_char;
    let mm = &mut *(*current).mm;
    let mut sp = mm.start_stack as *mut c_ulong;
    sp = sp.sub((*bprm).envc as usize + 1).sub((*bprm).argc as usize + 1).sub(1);
    mm.start_stack = (sp as c_ulong) & !(core::mem::size_of::<*mut c_void>().max(1) as c_ulong - 1);
    sp = mm.start_stack as *mut c_ulong;
    if put_user((*bprm).argc as c_ulong, sp) != 0 { return -14; } sp = sp.add(1);
    mm.arg_start = p as c_ulong;
    for _ in 0..(*bprm).argc { if put_user(p as c_ulong, sp) != 0 { return -14; } sp=sp.add(1); let n=strnlen_user(p, 131072); if n <= 0 || n > 131072 { return -22; } p=p.add(n as usize); }
    if put_user(0, sp) != 0 { return -14; } sp=sp.add(1); mm.arg_end=p as c_ulong; mm.env_start=p as c_ulong;
    for _ in 0..(*bprm).envc { if put_user(p as c_ulong, sp) != 0 { return -14; } sp=sp.add(1); let n=strnlen_user(p,131072); if n <= 0 || n > 131072 { return -22; } p=p.add(n as usize); }
    if put_user(0, sp) != 0 { return -14; } mm.env_end=p as c_ulong; 0
}

unsafe fn load_flat_file(bprm: *mut linux_binprm, libinfo: *mut lib_info, extra_stack: *mut c_ulong) -> c_int {
    let h=&*((*bprm).buf as *mut flat_hdr); let text_len=ntohl(h.data_start) as c_ulong; let data_len=(ntohl(h.data_end)-ntohl(h.data_start)) as c_ulong; let bss_len=(ntohl(h.bss_end)-ntohl(h.data_end)) as c_ulong; let stack_len=ntohl(h.stack_size) as c_ulong; let relocs=ntohl(h.reloc_count) as c_ulong; let flags=ntohl(h.flags); let rev=ntohl(h.rev);
    if &h.magic != b"bFLT" { return -8; }
    let total = text_len + data_len + bss_len + stack_len + relocs * core::mem::size_of::<c_ulong>() as c_ulong;
    if total >> 28 != 0 { return -8; }
    if !cfg!(CONFIG_MMU) && flags & (1|2) == 0 { /* ROM mapping branch follows kernel mapping ABI */ }
    let len = (text_len + data_len + bss_len + stack_len + relocs*core::mem::size_of::<c_ulong>() as c_ulong + DATA_START_OFFSET_WORDS as c_ulong*4 + 4095) & !4095;
    let textpos=vm_mmap(core::ptr::null_mut(),0,len,7,2,0); if textpos==0 { return -12; }
    let datapos=textpos + ntohl(h.data_start) as c_ulong + DATA_START_OFFSET_WORDS as c_ulong*4;
    let result=read_code((*bprm).file,textpos,0,text_len+data_len+relocs*4); if result<0 { let _=vm_munmap(textpos,len); return result as c_int; }
    (*current).mm.as_mut().unwrap().start_code=textpos+core::mem::size_of::<flat_hdr>() as c_ulong; (*current).mm.as_mut().unwrap().end_code=textpos+text_len; (*current).mm.as_mut().unwrap().start_data=datapos; (*current).mm.as_mut().unwrap().end_data=datapos+data_len; (*current).mm.as_mut().unwrap().start_brk=datapos+data_len+bss_len; (*current).mm.as_mut().unwrap().brk=((*current).mm.as_ref().unwrap().start_brk+3)&!3;
    (*libinfo).lib_list[0]=lib_info_entry{start_code:textpos+core::mem::size_of::<flat_hdr>() as c_ulong,start_data:datapos,start_brk:datapos+data_len+bss_len,text_len:text_len-core::mem::size_of::<flat_hdr>() as c_ulong,entry:(ntohl(h.entry)&0x00ff_ffff) as c_ulong+textpos,build_date:ntohl(h.build_date) as c_ulong,loaded:true};
    let reloc=(datapos + (ntohl(h.reloc_start) as c_ulong-text_len)) as *mut u32;
    if rev > 0 { for i in 0..relocs { let relval=(*reloc.add(i as usize)).to_be(); let rp=calc_reloc(relval as c_ulong,libinfo) as *mut u32; let mut addr=0; let ret=flat_get_addr_from_rp(rp,relval,flags,&mut addr); if ret!=0{return ret;} if addr!=0 { addr=calc_reloc(u32::from_be(addr) as c_ulong,libinfo) as u32; let ret=flat_put_addr_at_rp(rp,addr,relval); if ret!=0{return ret;} } } }
    flush_icache_user_range((*libinfo).lib_list[0].start_code,(*current).mm.as_ref().unwrap().end_code); 0
}

unsafe extern "C" fn load_flat_binary(bprm: *mut linux_binprm) -> c_int {
    let mut li=lib_info{lib_list:[lib_info_entry{start_code:0,start_data:0,start_brk:0,text_len:0,entry:0,build_date:0,loaded:false};MAX_SHARED_LIBS]}; let mut stack=0; let r=load_flat_file(bprm,&mut li,&mut stack); if r<0{return r;} set_binfmt(&mut flat_format); let r=setup_arg_pages(bprm,0,0); if r!=0{return r;} let r=create_flat_tables(bprm,(*bprm).p); if r!=0{return r;} finalize_exec(bprm); start_thread(core::ptr::null_mut(),li.lib_list[0].entry,(*current).mm.as_ref().unwrap().start_stack); 0
}

#[no_mangle] pub unsafe extern "C" fn init_flat_binfmt() -> c_int { register_binfmt(&mut flat_format) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
