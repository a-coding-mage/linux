// SPDX-License-Identifier: GPL-2.0
/*
 * KCSAN debugfs interface.
 *
 * Copyright (C) 2019, Google LLC.
 */

// Linux dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct AtomicLongT {
    _private: [u8; 0],
}

#[repr(C)]
pub struct KcsanCtx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SeqFile {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct File {
    _private: [u8; 0],
}

pub type SsizeT = isize;
pub type LoffT = i64;

extern "C" {
    static mut kcsan_counters: [AtomicLongT; KCSAN_COUNTER_COUNT];
    static mut kcsan_enabled: bool;
    static mut current: *mut TaskStruct;
    static mut report_filterlist_lock: RawSpinlock;

    fn get_cycles() -> u64;
    fn __kcsan_check_access(ptr: *mut core::ffi::c_void, size: usize, access_type: i32);
    fn kallsyms_lookup_size_offset(
        addr: usize,
        symbolsize: *mut usize,
        offset: *mut usize,
    ) -> bool;
    fn kallsyms_lookup_name(name: *const i8) -> usize;
    fn raw_spin_lock_irqsave(lock: *mut RawSpinlock, flags: *mut usize);
    fn raw_spin_unlock_irqrestore(lock: *mut RawSpinlock, flags: usize);
    fn kmalloc_array(n: usize, size: usize, flags: u32) -> *mut usize;
    fn kfree(ptr: *mut usize);
    fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
    fn seq_printf(file: *mut SeqFile, fmt: *const i8, ...);
    fn seq_read(file: *mut File, buf: *mut u8, count: usize, off: *mut LoffT) -> SsizeT;
    fn single_open(file: *mut File, show: unsafe extern "C" fn(*mut SeqFile, *mut core::ffi::c_void) -> i32, data: *mut core::ffi::c_void) -> i32;
    fn single_release(inode: *mut Inode, file: *mut File) -> i32;
    fn copy_from_user(to: *mut u8, from: *const u8, n: usize) -> usize;
    fn strstrip(s: *mut i8) -> *mut i8;
    fn strcmp(a: *const i8, b: *const i8) -> i32;
    fn strlen(s: *const i8) -> usize;
    fn kstrtoul(s: *const i8, base: u32, result: *mut usize) -> i32;
    fn debugfs_create_file(name: *const i8, mode: u32, parent: *mut core::ffi::c_void, data: *mut core::ffi::c_void, ops: *const FileOperations) -> *mut core::ffi::c_void;
}

const KCSAN_COUNTER_COUNT: usize = 9;
const KCSAN_COUNTER_USED_WATCHPOINTS: usize = 0;
const KCSAN_COUNTER_SETUP_WATCHPOINTS: usize = 1;
const KCSAN_COUNTER_DATA_RACES: usize = 2;
const KCSAN_COUNTER_ASSERT_FAILURES: usize = 3;
const KCSAN_COUNTER_NO_CAPACITY: usize = 4;
const KCSAN_COUNTER_REPORT_RACES: usize = 5;
const KCSAN_COUNTER_RACES_UNKNOWN_ORIGIN: usize = 6;
const KCSAN_COUNTER_UNENCODABLE_ACCESSES: usize = 7;
const KCSAN_COUNTER_ENCODING_FALSE_POSITIVES: usize = 8;
const KCSAN_ACCESS_ATOMIC: i32 = 1;
const KCSAN_ACCESS_WRITE: i32 = 2;
const PAGE_SIZE: usize = 4096;
const KSYM_NAME_LEN: usize = 128;

#[repr(C)] pub struct RawSpinlock { _private: [u8; 0] }
#[repr(C)] pub struct TaskStruct { pub kcsan_ctx: KcsanCtx }
#[repr(C)] pub struct FileOperations {
    pub read: Option<unsafe extern "C" fn(*mut File, *mut u8, usize, *mut LoffT) -> SsizeT>,
    pub open: Option<unsafe extern "C" fn(*mut Inode, *mut File) -> i32>,
    pub write: Option<unsafe extern "C" fn(*mut File, *const i8, usize, *mut LoffT) -> SsizeT>,
    pub release: Option<unsafe extern "C" fn(*mut Inode, *mut File) -> i32>,
}

static mut counter_names: [&[u8]; KCSAN_COUNTER_COUNT] = [
    b"used_watchpoints\0", b"setup_watchpoints\0", b"data_races\0",
    b"assert_failures\0", b"no_capacity\0", b"report_races\0",
    b"races_unknown_origin\0", b"unencodable_accesses\0", b"encoding_false_positives\0",
];

#[repr(C)]
struct ReportFilterList { addrs: *mut usize, size: usize, used: i32, sorted: bool, whitelist: bool }
static mut report_filterlist: ReportFilterList = ReportFilterList { addrs: core::ptr::null_mut(), size: 0, used: 0, sorted: false, whitelist: false };

unsafe fn microbenchmark(mut iters: usize) {
    let ctx_save = (*current).kcsan_ctx;
    let was_enabled = core::ptr::read_volatile(&kcsan_enabled);
    (*current).kcsan_ctx = core::mem::zeroed();
    core::ptr::write_volatile(&mut kcsan_enabled, false);
    let mut cycles = get_cycles();
    while iters != 0 {
        iters -= 1;
        let addr = iters & ((PAGE_SIZE << 8) - 1);
        let ty = if (iters & 0x7f) == 0 { KCSAN_ACCESS_ATOMIC } else if (iters & 0xf) == 0 { KCSAN_ACCESS_WRITE } else { 0 };
        __kcsan_check_access(addr as *mut core::ffi::c_void, core::mem::size_of::<isize>(), ty);
    }
    cycles = get_cycles().wrapping_sub(cycles);
    core::ptr::write_volatile(&mut kcsan_enabled, was_enabled);
    (*current).kcsan_ctx = ctx_save;
    let _ = cycles;
}

unsafe fn cmp_filterlist_addrs(rhs: *const usize, lhs: *const usize) -> i32 { (*rhs).cmp(&*lhs) as i32 }

pub unsafe fn kcsan_skip_report_debugfs(mut func_addr: usize) -> bool {
    let (mut symbolsize, mut offset) = (0, 0);
    if !kallsyms_lookup_size_offset(func_addr, &mut symbolsize, &mut offset) { return false; }
    func_addr -= offset;
    let mut flags = 0; raw_spin_lock_irqsave(&mut report_filterlist_lock, &mut flags);
    let mut ret = false;
    if report_filterlist.used != 0 {
        if report_filterlist.whitelist { ret = !ret; }
    }
    raw_spin_unlock_irqrestore(&mut report_filterlist_lock, flags); ret
}

unsafe fn set_report_filterlist_whitelist(whitelist: bool) { let mut flags=0; raw_spin_lock_irqsave(&mut report_filterlist_lock,&mut flags); report_filterlist.whitelist=whitelist; raw_spin_unlock_irqrestore(&mut report_filterlist_lock,flags); }
unsafe fn insert_report_filterlist(func: *const i8) -> SsizeT {
    let addr = kallsyms_lookup_name(func);
    if addr == 0 { return -2; }
    let mut flags=0;
    if report_filterlist.used == report_filterlist.size {
        let new_size = if report_filterlist.size == 0 { 4 } else { report_filterlist.size * 2 };
        let new_addrs = kmalloc_array(new_size, core::mem::size_of::<usize>(), 0);
        if new_addrs.is_null() { return -12; }
        raw_spin_lock_irqsave(&mut report_filterlist_lock,&mut flags);
        if report_filterlist.used == report_filterlist.size {
            if report_filterlist.used != 0 { memcpy(new_addrs as *mut u8, report_filterlist.addrs as *const u8, report_filterlist.used * core::mem::size_of::<usize>()); }
            let old=report_filterlist.addrs; report_filterlist.addrs=new_addrs; report_filterlist.size=new_size;
            raw_spin_unlock_irqrestore(&mut report_filterlist_lock,flags); kfree(old); flags=0;
        } else { raw_spin_unlock_irqrestore(&mut report_filterlist_lock,flags); kfree(new_addrs); }
    }
    raw_spin_lock_irqsave(&mut report_filterlist_lock,&mut flags);
    *report_filterlist.addrs.add(report_filterlist.used as usize)=addr;
    report_filterlist.used += 1; report_filterlist.sorted=false;
    raw_spin_unlock_irqrestore(&mut report_filterlist_lock,flags); 0
}
unsafe extern "C" fn show_info(file:*mut SeqFile,_v:*mut core::ffi::c_void)->i32 {
    let mut flags=0; raw_spin_lock_irqsave(&mut report_filterlist_lock,&mut flags);
    let _ = file; raw_spin_unlock_irqrestore(&mut report_filterlist_lock,flags); 0
}
unsafe extern "C" fn debugfs_open(_inode:*mut Inode,file:*mut File)->i32 { single_open(file,show_info,core::ptr::null_mut()) }
unsafe extern "C" fn debugfs_write(_file:*mut File,buf:*const i8,count:usize,_off:*mut LoffT)->SsizeT {
    let mut kbuf=[0i8; KSYM_NAME_LEN]; let read_len=count.min(KSYM_NAME_LEN-1);
    if copy_from_user(kbuf.as_mut_ptr() as *mut u8,buf as *const u8,read_len)!=0{return -14;}
    kbuf[read_len]=0; let arg=strstrip(kbuf.as_mut_ptr());
    if strcmp(arg,b"on\0".as_ptr() as *const i8)==0 { core::ptr::write_volatile(&mut kcsan_enabled,true); }
    else if strcmp(arg,b"off\0".as_ptr() as *const i8)==0 { core::ptr::write_volatile(&mut kcsan_enabled,false); }
    else if strcmp(arg,b"whitelist\0".as_ptr() as *const i8)==0 { set_report_filterlist_whitelist(true); }
    else if strcmp(arg,b"blacklist\0".as_ptr() as *const i8)==0 { set_report_filterlist_whitelist(false); }
    else if *arg as u8 == b'!' { if insert_report_filterlist(arg.add(1))<0{return -2;} }
    else { return -22; } count as SsizeT
}

static debugfs_ops: FileOperations = FileOperations { read: None, open: Some(debugfs_open), write: Some(debugfs_write), release: None };
pub unsafe extern "C" fn kcsan_debugfs_init() -> i32 { debugfs_create_file(b"kcsan\0".as_ptr() as *const i8, 0o644, core::ptr::null_mut(), core::ptr::null_mut(), &debugfs_ops); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
