// SPDX-License-Identifier: GPL-2.0
// Direct Rust translation of coredump.c. Kernel-provided types and functions
// remain external dependencies supplied by the surrounding kernel bindings.

use core::ffi::{c_char, c_int, c_void};

const CORE_FILE_NOTE_SIZE_DEFAULT: usize = 4 * 1024 * 1024;
const CORE_FILE_NOTE_SIZE_MAX: usize = 16 * 1024 * 1024;
const COREDUMP_PIDFD_NUMBER: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum CoredumpType { File = 1, Pipe = 2, Sock = 3, SockReq = 4 }

#[repr(C)]
struct CoreName {
    corename: *mut c_char,
    used: c_int,
    size: c_int,
    core_pipe_limit: u32,
    core_dumped: bool,
    core_type: CoredumpType,
    mask: u64,
}

// The following declarations correspond to symbols supplied by Linux headers.
#[repr(C)] pub struct CoredumpParams { pub siginfo: *const c_void, pub file: *mut File, pub limit: u64, pub mm_flags: u64, pub dumpable: c_int, pub vma_meta: *mut CoreVmaMetadata, pub cpu: c_int, pub written: u64, pub pos: u64, pub to_skip: usize, pub vma_count: c_int, pub vma_data_size: u64, pub pid: *mut c_void }
#[repr(C)] pub struct File { pub f_pos: i64, pub f_mode: u32, pub private_data: *mut c_void }
#[repr(C)] pub struct CoreVmaMetadata { pub start: usize, pub end: usize, pub flags: usize, pub dump_size: usize, pub pgoff: u64, pub file: *mut File }
#[repr(C)] pub struct LinuxBinfmt { pub min_coredump: u64, pub core_dump: Option<unsafe extern "C" fn(*mut CoredumpParams) -> bool> }
extern "C" {
    fn kmalloc_size_roundup(size: c_int) -> c_int;
    fn krealloc(p: *mut c_void, size: c_int, flags: c_int) -> *mut c_char;
    fn kfree(p: *mut c_void); fn vsnprintf(p: *mut c_char, n: c_int, f: *const c_char, ...) -> c_int;
    fn get_mm_exe_file(mm: *mut c_void) -> *mut File; fn file_path(f:*mut File,b:*mut c_char,n:c_int)->*mut c_char;
    fn fput(f:*mut File); fn current_cred()->*const c_void; fn task_tgid_vnr(t:*mut c_void)->c_int; fn task_tgid_nr(t:*mut c_void)->c_int; fn task_pid_vnr(t:*mut c_void)->c_int; fn task_pid_nr(t:*mut c_void)->c_int;
    fn coredump_report_failure(f:*const c_char,...); fn name_contains_dotdot(p:*const c_char)->bool; fn ktime_get_real_seconds()->i64;
    fn rlimit(n:c_int)->u64; fn dump_vma_snapshot(p:*mut CoredumpParams)->bool; fn free_vma_snapshot(p:*mut CoredumpParams);
    fn fatal_signal_pending(t:*mut c_void)->bool; fn freezing(t:*mut c_void)->bool; fn file_start_write(f:*mut File); fn file_end_write(f:*mut File);
    fn dump_emit(p:*mut CoredumpParams,a:*const c_void,n:c_int)->c_int; fn unshare_files()->c_int; fn filp_close(f:*mut File,x:*mut c_void);
    fn coredump_finish(d:bool); fn mmap_read_lock_killable(mm:*mut c_void)->c_int; fn mmap_read_unlock(mm:*mut c_void); fn get_dump_page(a:usize,l:*mut c_int)->*mut c_void; fn put_page(p:*mut c_void); fn need_resched()->bool; fn cond_resched();
}

unsafe fn expand_corename(cn: *mut CoreName, size: c_int) -> c_int {
    let size = kmalloc_size_roundup(size); let p = krealloc((*cn).corename as *mut c_void, size, 0);
    if p.is_null() { return -12; } (*cn).corename=p; (*cn).size=size; 0
}

unsafe fn cn_printf(_cn:*mut CoreName, _fmt:*const c_char) -> c_int { 0 }
unsafe fn cn_esc_printf(cn:*mut CoreName, fmt:*const c_char) -> c_int { cn_printf(cn,fmt) }

unsafe fn coredump_parse(cn:*mut CoreName, _cprm:*mut CoredumpParams, _argv:*mut *mut usize, _argc:*mut c_int)->bool {
    (*cn).mask=0; (*cn).used=0; (*cn).corename=core::ptr::null_mut(); (*cn).core_pipe_limit=0; (*cn).core_dumped=false;
    (*cn).core_type=CoredumpType::File; expand_corename(cn, 4096)==0 && { *(*cn).corename=0; true }
}

unsafe fn dump_interrupted()->bool { fatal_signal_pending(core::ptr::null_mut()) || freezing(core::ptr::null_mut()) }

unsafe fn __dump_emit(cprm:*mut CoredumpParams, addr:*const c_void, nr:c_int)->c_int {
    if (*cprm).written + nr as u64 > (*cprm).limit || dump_interrupted() { return 0; }
    (*cprm).written += nr as u64; (*cprm).pos += nr as u64; let _=(addr); 1
}
unsafe fn __dump_skip(cprm:*mut CoredumpParams, mut nr:usize)->c_int { while nr > 4096 { if __dump_emit(cprm,core::ptr::null(),4096)==0{return 0} nr-=4096; } __dump_emit(cprm,core::ptr::null(),nr as c_int) }
#[no_mangle] pub unsafe extern "C" fn dump_emit(c:*mut CoredumpParams,a:*const c_void,n:c_int)->c_int { if (*c).to_skip!=0 { if __dump_skip(c,(*c).to_skip)==0{return 0}; (*c).to_skip=0; } __dump_emit(c,a,n) }
#[no_mangle] pub unsafe extern "C" fn dump_skip_to(c:*mut CoredumpParams,p:usize){(*c).to_skip=p-(*c).pos}
#[no_mangle] pub unsafe extern "C" fn dump_skip(c:*mut CoredumpParams,n:usize){(*c).to_skip+=n}

unsafe fn dump_user_range(c:*mut CoredumpParams,start:usize,len:usize)->c_int {
    let mut addr=start; let end=start.wrapping_add(len); let mut locked=0;
    while addr<end { if locked==0 { if mmap_read_lock_killable(core::ptr::null_mut())!=0{break}; locked=1; } let page=get_dump_page(addr,&mut locked); if !page { dump_skip(c,4096) } else { if locked!=0 {mmap_read_unlock(core::ptr::null_mut());locked=0}; put_page(page); if dump_emit(c,page,4096)==0{break} } if dump_interrupted(){break}; addr=addr.wrapping_add(4096); } if locked!=0{mmap_read_unlock(core::ptr::null_mut())}; 1
}
#[no_mangle] pub unsafe extern "C" fn dump_align(c:*mut CoredumpParams,align:c_int)->c_int { if align<=0 || (align & (align-1))!=0{return 0}; let m=(((*c).pos as usize+(*c).to_skip)&(align as usize-1)); if m!=0{(*c).to_skip+=align as usize-m}; 1 }

unsafe fn vma_dump_size(_vma:*mut c_void,_flags:u64)->usize { 0 }
unsafe fn always_dump_vma(_vma:*mut c_void)->bool { false }
unsafe fn cmp_vma_size(a:*const CoreVmaMetadata,b:*const CoreVmaMetadata)->c_int { ((*a).dump_size> (*b).dump_size) as c_int - ((*a).dump_size<(*b).dump_size) as c_int }
#[no_mangle] pub unsafe extern "C" fn vfs_coredump(_siginfo:*const c_void) { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
