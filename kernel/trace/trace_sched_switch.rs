// SPDX-License-Identifier: GPL-2.0
// Translation of trace_sched_switch.c. Kernel-provided declarations are external dependencies.

const RECORD_CMDLINE: i32 = 1;
const RECORD_TGID: i32 = 2;
const SAVED_CMDLINES_DEFAULT: u32 = 128;
const NO_CMDLINE_MAP: u32 = u32::MAX;

static mut sched_cmdline_ref: i32 = 0;
static mut sched_tgid_ref: i32 = 0;
static mut sched_register_mutex: Mutex = Mutex::new();

extern "C" {
    static mut current: *mut task_struct;
    static mut tgid_map: *mut i32;
    static mut tgid_map_max: usize;
    static mut trace_cmdline_lock: arch_spinlock_t;
    static mut savedcmd: *mut saved_cmdlines_buffer;
    static mut trace_taskinfo_save: bool;
    static mut init_pid_ns: pid_namespace;

    fn register_trace_sched_wakeup(f: unsafe extern "C" fn(*mut c_void, *mut task_struct), data: *mut c_void) -> i32;
    fn register_trace_sched_wakeup_new(f: unsafe extern "C" fn(*mut c_void, *mut task_struct), data: *mut c_void) -> i32;
    fn register_trace_sched_switch(f: unsafe extern "C" fn(*mut c_void, bool, *mut task_struct, *mut task_struct, u32), data: *mut c_void) -> i32;
    fn unregister_trace_sched_wakeup(f: unsafe extern "C" fn(*mut c_void, *mut task_struct), data: *mut c_void);
    fn unregister_trace_sched_wakeup_new(f: unsafe extern "C" fn(*mut c_void, *mut task_struct), data: *mut c_void);
    fn unregister_trace_sched_switch(f: unsafe extern "C" fn(*mut c_void, bool, *mut task_struct, *mut task_struct, u32), data: *mut c_void);
    fn tracing_record_taskinfo_sched_switch(prev: *mut task_struct, next: *mut task_struct, flags: i32);
    fn mutex_lock(m: *mut Mutex); fn mutex_unlock(m: *mut Mutex);
    fn pr_info(s: *const u8);
    fn get_order(size: usize) -> i32; fn kmemleak_free(p: *mut c_void); fn free_pages(p: usize, order: i32);
    fn alloc_pages(flags: u32, order: i32) -> *mut page; fn page_address(p: *mut page) -> *mut c_void;
    fn kmemleak_alloc(p: *mut c_void, size: usize, min_count: usize, flags: u32);
    fn memset(p: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn strscpy(dst: *mut i8, src: *const i8, n: usize) -> isize;
    fn arch_spin_trylock(l: *mut arch_spinlock_t) -> bool; fn arch_spin_lock(l: *mut arch_spinlock_t); fn arch_spin_unlock(l: *mut arch_spinlock_t);
    fn preempt_disable(); fn preempt_enable(); fn lockdep_assert_preemption_disabled();
    fn smp_load_acquire(p: *mut *mut i32) -> *mut i32; fn smp_store_release(p: *mut *mut i32, v: *mut i32);
    fn kvzalloc_objs<T>(obj: T, n: usize) -> *mut i32;
    fn seq_printf(m: *mut seq_file, fmt: *const u8, ... ) -> i32;
    fn tracing_check_open_get_tr(p: *mut c_void) -> i32; fn seq_open(f: *mut file, ops: *const seq_operations) -> i32;
    fn seq_read(); fn seq_lseek(); fn seq_release(); fn tracing_open_generic();
    fn simple_read_from_buffer(ubuf: *mut u8, cnt: usize, ppos: *mut i64, buf: *const u8, n: i32) -> isize;
    fn scnprintf(buf: *mut i8, size: usize, fmt: *const u8, ...) -> i32;
    fn kstrtoul_from_user(ubuf: *const u8, cnt: usize, base: u32, val: *mut usize) -> i32;
}

#[repr(C)] pub struct Mutex { _private: [u8; 0] }
#[repr(C)] pub struct arch_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { pub pid: i32, pub tgid: i32, pub comm: [i8; TASK_COMM_LEN] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct pid_namespace { pub pid_max: usize }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { pub count: usize }
#[repr(C)] pub struct seq_operations { pub start: Option<unsafe extern "C" fn(*mut seq_file,*mut i64)->*mut c_void>, pub stop: Option<unsafe extern "C" fn(*mut seq_file,*mut c_void)>, pub next: Option<unsafe extern "C" fn(*mut seq_file,*mut c_void,*mut i64)->*mut c_void>, pub show: Option<unsafe extern "C" fn(*mut seq_file,*mut c_void)->i32> }
#[repr(C)] pub struct file_operations { pub open: Option<unsafe extern "C" fn(*mut inode,*mut file)->i32>, pub read: Option<unsafe extern "C" fn()>, pub llseek: Option<unsafe extern "C" fn()>, pub release: Option<unsafe extern "C" fn()> }
pub type c_void = core::ffi::c_void; pub const TASK_COMM_LEN: usize = 16; pub const PID_MAX_DEFAULT: usize = 32768; pub const PAGE_SHIFT: i32 = 12; pub const GFP_KERNEL: u32 = 0;
const TRACE_RECORD_CMDLINE: i32 = RECORD_CMDLINE; const TRACE_RECORD_TGID: i32 = RECORD_TGID; const SEQ_SKIP: i32 = 1; const ENOMEM: i32 = 12; const EINVAL: i32 = 22;

#[repr(C)] pub struct saved_cmdlines_buffer { pub map_pid_to_cmdline: [u32; PID_MAX_DEFAULT+1], pub map_cmdline_to_pid: *mut u32, pub cmdline_num: u32, pub cmdline_idx: i32, pub saved_cmdlines: [i8; 0] }

unsafe extern "C" fn probe_sched_switch(_: *mut c_void, _: bool, prev: *mut task_struct, next: *mut task_struct, _: u32) { let flags = RECORD_TGID * (sched_tgid_ref != 0) as i32 + RECORD_CMDLINE * (sched_cmdline_ref != 0) as i32; if flags != 0 { tracing_record_taskinfo_sched_switch(prev,next,flags); } }
unsafe extern "C" fn probe_sched_wakeup(_: *mut c_void, wakee: *mut task_struct) { let flags = RECORD_TGID * (sched_tgid_ref != 0) as i32 + RECORD_CMDLINE * (sched_cmdline_ref != 0) as i32; if flags != 0 { tracing_record_taskinfo_sched_switch(current,wakee,flags); } }

unsafe fn tracing_sched_register() -> i32 { let mut ret=register_trace_sched_wakeup(probe_sched_wakeup,core::ptr::null_mut()); if ret!=0{return ret} ret=register_trace_sched_wakeup_new(probe_sched_wakeup,core::ptr::null_mut()); if ret!=0 { unregister_trace_sched_wakeup(probe_sched_wakeup,core::ptr::null_mut()); return ret } ret=register_trace_sched_switch(probe_sched_switch,core::ptr::null_mut()); if ret!=0 { unregister_trace_sched_wakeup_new(probe_sched_wakeup,core::ptr::null_mut()); unregister_trace_sched_wakeup(probe_sched_wakeup,core::ptr::null_mut()); } ret }
unsafe fn tracing_sched_unregister(){ unregister_trace_sched_switch(probe_sched_switch,core::ptr::null_mut()); unregister_trace_sched_wakeup_new(probe_sched_wakeup,core::ptr::null_mut()); unregister_trace_sched_wakeup(probe_sched_wakeup,core::ptr::null_mut()); }
unsafe fn tracing_start_sched_switch(ops:i32){ mutex_lock(&mut sched_register_mutex); let register=sched_cmdline_ref==0&&sched_tgid_ref==0; if ops==RECORD_CMDLINE{sched_cmdline_ref+=1}else if ops==RECORD_TGID{sched_tgid_ref+=1}; if register&&(sched_cmdline_ref!=0||sched_tgid_ref!=0){tracing_sched_register();} mutex_unlock(&mut sched_register_mutex); }
unsafe fn tracing_stop_sched_switch(ops:i32){ mutex_lock(&mut sched_register_mutex); if ops==RECORD_CMDLINE{sched_cmdline_ref-=1}else if ops==RECORD_TGID{sched_tgid_ref-=1}; if sched_cmdline_ref==0&&sched_tgid_ref==0{tracing_sched_unregister();} mutex_unlock(&mut sched_register_mutex); }
pub unsafe extern "C" fn tracing_start_cmdline_record(){tracing_start_sched_switch(RECORD_CMDLINE)} pub unsafe extern "C" fn tracing_stop_cmdline_record(){tracing_stop_sched_switch(RECORD_CMDLINE)} pub unsafe extern "C" fn tracing_start_tgid_record(){tracing_start_sched_switch(RECORD_TGID)} pub unsafe extern "C" fn tracing_stop_tgid_record(){tracing_stop_sched_switch(RECORD_TGID)}

unsafe fn get_saved_cmdlines(idx:usize)->*mut i8{(*savedcmd).saved_cmdlines.as_mut_ptr().add(idx*TASK_COMM_LEN)} unsafe fn set_cmdline(idx:usize,cmd:*const i8){strscpy(get_saved_cmdlines(idx),cmd,TASK_COMM_LEN);}
unsafe fn free_saved_cmdlines_buffer(s:*mut saved_cmdlines_buffer){let order=get_order(core::mem::size_of::<saved_cmdlines_buffer>()+(*s).cmdline_num as usize*TASK_COMM_LEN);kmemleak_free(s.cast());free_pages(s as usize,order);}
unsafe fn allocate_cmdlines_buffer(mut val:u32)->*mut saved_cmdlines_buffer{let orig=core::mem::size_of::<saved_cmdlines_buffer>()+val as usize*(TASK_COMM_LEN+4);let order=get_order(orig);let size=1usize<<((order+PAGE_SHIFT) as usize);let page=alloc_pages(GFP_KERNEL,order);if page.is_null(){return core::ptr::null_mut()}let s=page_address(page) as *mut saved_cmdlines_buffer;kmemleak_alloc(s.cast(),size,1,GFP_KERNEL);memset(s.cast(),0,core::mem::size_of::<saved_cmdlines_buffer>());val=((size-core::mem::size_of::<saved_cmdlines_buffer>())/(TASK_COMM_LEN+4)) as u32;(*s).cmdline_num=val;(*s).map_cmdline_to_pid=(*s).saved_cmdlines.as_mut_ptr().add(val as usize*TASK_COMM_LEN) as *mut u32;memset((*s).map_pid_to_cmdline.as_mut_ptr().cast(),NO_CMDLINE_MAP as i32,core::mem::size_of_val(&(*s).map_pid_to_cmdline));memset((*s).map_cmdline_to_pid.cast(),NO_CMDLINE_MAP as i32,val as usize*4);s}
pub unsafe extern "C" fn trace_create_savedcmd()->i32{savedcmd=allocate_cmdlines_buffer(SAVED_CMDLINES_DEFAULT);if savedcmd.is_null(){-ENOMEM}else{0}}

unsafe fn trace_find_tgid_ptr(pid:i32)->*mut i32{let map=smp_load_acquire(&mut tgid_map);if map.is_null()||pid as usize>tgid_map_max{core::ptr::null_mut()}else{map.add(pid as usize)}}
pub unsafe extern "C" fn trace_find_tgid(pid:i32)->i32{let p=trace_find_tgid_ptr(pid);if p.is_null(){0}else{*p}}
unsafe fn trace_save_tgid(t:*mut task_struct)->bool{if (*t).pid==0{return true}let p=trace_find_tgid_ptr((*t).pid);if p.is_null(){false}else{*p=(*t).tgid;true}}
unsafe fn trace_save_cmdline(t:*mut task_struct)->bool{if (*t).pid==0{return true}lockdep_assert_preemption_disabled();if !arch_spin_trylock(&mut trace_cmdline_lock){return false}let tp=((*t).pid as usize)&(PID_MAX_DEFAULT-1);let mut idx=(*savedcmd).map_pid_to_cmdline[tp];if idx==NO_CMDLINE_MAP{idx=((*savedcmd).cmdline_idx+1) as u32%(*savedcmd).cmdline_num;(*savedcmd).map_pid_to_cmdline[tp]=idx;(*savedcmd).cmdline_idx=idx as i32}*(*savedcmd).map_cmdline_to_pid.add(idx as usize)=(*t).pid as u32;set_cmdline(idx as usize,(*t).comm.as_ptr());arch_spin_unlock(&mut trace_cmdline_lock);true}
unsafe fn tracing_record_taskinfo_skip(flags:i32)->bool{flags&(TRACE_RECORD_CMDLINE|TRACE_RECORD_TGID)==0||!trace_taskinfo_save}
pub unsafe extern "C" fn tracing_record_taskinfo(t:*mut task_struct,flags:i32){if tracing_record_taskinfo_skip(flags){return}let mut done=flags&TRACE_RECORD_CMDLINE==0||trace_save_cmdline(t);done&=flags&TRACE_RECORD_TGID==0||trace_save_tgid(t);if done{trace_taskinfo_save=false}}
pub unsafe extern "C" fn tracing_record_taskinfo_sched_switch(prev:*mut task_struct,next:*mut task_struct,flags:i32){if tracing_record_taskinfo_skip(flags){return}let mut done=flags&TRACE_RECORD_CMDLINE==0||trace_save_cmdline(prev);done&=flags&TRACE_RECORD_CMDLINE==0||trace_save_cmdline(next);done&=flags&TRACE_RECORD_TGID==0||trace_save_tgid(prev);done&=flags&TRACE_RECORD_TGID==0||trace_save_tgid(next);if done{trace_taskinfo_save=false}}
pub unsafe extern "C" fn tracing_record_cmdline(t:*mut task_struct){tracing_record_taskinfo(t,TRACE_RECORD_CMDLINE)} pub unsafe extern "C" fn tracing_record_tgid(t:*mut task_struct){tracing_record_taskinfo(t,TRACE_RECORD_TGID)}

pub unsafe extern "C" fn trace_find_cmdline(pid:i32,comm:*mut i8){preempt_disable();arch_spin_lock(&mut trace_cmdline_lock);if pid==0{strscpy(comm,b"<idle>\0".as_ptr() as _,TASK_COMM_LEN)}else{strscpy(comm,b"<...>\0".as_ptr() as _,TASK_COMM_LEN)}arch_spin_unlock(&mut trace_cmdline_lock);preempt_enable()}
pub unsafe extern "C" fn trace_alloc_tgid_map()->i32{if !tgid_map.is_null(){return 0}tgid_map_max=init_pid_ns.pid_max;tgid_map=kvzalloc_objs(0i32,tgid_map_max+1);if tgid_map.is_null(){-ENOMEM}else{0}}
pub unsafe extern "C" fn trace_free_saved_cmdlines_buffer(){free_saved_cmdlines_buffer(savedcmd)}

unsafe extern "C" fn saved_tgids_next(_: *mut seq_file,pos:*mut i64)->*mut c_void{*pos+=1;trace_find_tgid_ptr(*pos as i32).cast()}
unsafe extern "C" fn saved_tgids_start(_: *mut seq_file,pos:*mut i64)->*mut c_void{trace_find_tgid_ptr(*pos as i32).cast()}
unsafe extern "C" fn saved_tgids_stop(_: *mut seq_file,_:*mut c_void){}
unsafe extern "C" fn saved_tgids_show(m:*mut seq_file,v:*mut c_void)->i32{let e=v as *mut i32;let pid=e.offset_from(tgid_map) as i32;let tgid=*e;if tgid==0{SEQ_SKIP}else{seq_printf(m,b"%d %d\n\0".as_ptr(),pid,tgid);0}}
static tracing_saved_tgids_seq_ops:seq_operations=seq_operations{start:Some(saved_tgids_start),stop:Some(saved_tgids_stop),next:Some(saved_tgids_next),show:Some(saved_tgids_show)};
unsafe extern "C" fn tracing_saved_tgids_open(_: *mut inode,f:*mut file)->i32{let ret=tracing_check_open_get_tr(core::ptr::null_mut());if ret!=0{ret}else{seq_open(f,&tracing_saved_tgids_seq_ops)}}
#[no_mangle] pub static tracing_saved_tgids_fops:file_operations=file_operations{open:Some(tracing_saved_tgids_open),read:Some(seq_read),llseek:Some(seq_lseek),release:Some(seq_release)};

unsafe extern "C" fn saved_cmdlines_next(m:*mut seq_file,v:*mut c_void,pos:*mut i64)->*mut c_void{let mut p=v as *mut u32;if *pos!=0||(*m).count!=0{p=p.add(1)}*pos+=1;while p<(*savedcmd).map_cmdline_to_pid.add((*savedcmd).cmdline_num as usize){if *p!=u32::MAX{return p.cast()}p=p.add(1)}core::ptr::null_mut()}
unsafe extern "C" fn saved_cmdlines_start(m:*mut seq_file,pos:*mut i64)->*mut c_void{preempt_disable();arch_spin_lock(&mut trace_cmdline_lock);let mut l=0i64;let mut v=(*savedcmd).map_cmdline_to_pid.cast();while l<=*pos{v=saved_cmdlines_next(m,v,&mut l);if v.is_null(){return v}}v}
unsafe extern "C" fn saved_cmdlines_stop(_: *mut seq_file,_:*mut c_void){arch_spin_unlock(&mut trace_cmdline_lock);preempt_enable()}
unsafe extern "C" fn saved_cmdlines_show(m:*mut seq_file,v:*mut c_void)->i32{let mut buf=[0i8;TASK_COMM_LEN];let p=v as *mut u32;trace_find_cmdline(*p as i32,buf.as_mut_ptr());seq_printf(m,b"%d %s\n\0".as_ptr(),*p,buf.as_ptr());0}
static tracing_saved_cmdlines_seq_ops:seq_operations=seq_operations{start:Some(saved_cmdlines_start),stop:Some(saved_cmdlines_stop),next:Some(saved_cmdlines_next),show:Some(saved_cmdlines_show)};
unsafe extern "C" fn tracing_saved_cmdlines_open(_: *mut inode,f:*mut file)->i32{let ret=tracing_check_open_get_tr(core::ptr::null_mut());if ret!=0{ret}else{seq_open(f,&tracing_saved_cmdlines_seq_ops)}}
#[no_mangle] pub static tracing_saved_cmdlines_fops:file_operations=file_operations{open:Some(tracing_saved_cmdlines_open),read:Some(seq_read),llseek:Some(seq_lseek),release:Some(seq_release)};
unsafe extern "C" fn tracing_saved_cmdlines_size_read(f:*mut file,ubuf:*mut u8,cnt:usize,ppos:*mut i64)->isize{let mut buf=[0i8;64];preempt_disable();arch_spin_lock(&mut trace_cmdline_lock);let r=scnprintf(buf.as_mut_ptr(),64,b"%u\n\0".as_ptr(),(*savedcmd).cmdline_num);arch_spin_unlock(&mut trace_cmdline_lock);preempt_enable();simple_read_from_buffer(ubuf,cnt,ppos,buf.as_ptr() as _,r)}
unsafe fn tracing_resize_saved_cmdlines(val:u32)->i32{let s=allocate_cmdlines_buffer(val);if s.is_null(){return -ENOMEM}preempt_disable();arch_spin_lock(&mut trace_cmdline_lock);let old=savedcmd;savedcmd=s;arch_spin_unlock(&mut trace_cmdline_lock);preempt_enable();free_saved_cmdlines_buffer(old);0}
unsafe extern "C" fn tracing_saved_cmdlines_size_write(_: *mut file,ubuf:*const u8,cnt:usize,ppos:*mut i64)->isize{let mut val=0usize;let ret=kstrtoul_from_user(ubuf,cnt,10,&mut val);if ret!=0{return ret as isize}if val==0||val>PID_MAX_DEFAULT{return -EINVAL as isize}let ret=tracing_resize_saved_cmdlines(val as u32);if ret<0{return ret as isize}*ppos+=cnt as i64;cnt as isize}
#[no_mangle] pub static tracing_saved_cmdlines_size_fops:file_operations=file_operations{open:Some(tracing_open_generic),read:Some(tracing_saved_cmdlines_size_read),llseek:None,release:None};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
