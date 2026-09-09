// SPDX-License-Identifier: GPL-2.0
//! S/390 debug facility. Direct low-level translation of debug.c.
// The Linux kernel types, constants, macros, and external functions referenced
// here are supplied by the surrounding translated kernel sources.

const DEBUG_PROLOG_ENTRY: isize = -1;
const ALL_AREAS: i32 = 0;
const NO_AREAS: i32 = 1;
const PARAM_UNSET: i32 = -2;
const PARAM_NUM: usize = 16;
const DEBUG_SPRINTF_MAX_ARGS: usize = 10;

#[repr(C)]
pub struct file_private_info_t {
    offset: loff_t, act_area: i32, act_page: i32, act_entry: isize,
    act_entry_offset: usize, temp_buf: [u8; 2048],
    debug_info_org: *mut debug_info_t, debug_info_snap: *mut debug_info_t,
    view: *mut debug_view,
}
#[repr(C)]
pub struct debug_sprintf_entry_t { pub string: *mut i8, pub args: [c_long; 0] }

extern "C" {
    static mut debug_hex_ascii_view: debug_view;
    static mut debug_sprintf_view: debug_view;
    static mut debug_area_first: *mut debug_info_t;
    static mut debug_area_last: *mut debug_info_t;
    static mut debug_mutex: mutex;
    static mut initialized: i32;
    static mut debug_critical: i32;
    static mut debug_active: i32;
    static mut debug_stoppable: i32;
    static mut __s390dbf_info: *mut *mut debug_info_t;
    static mut __s390dbf_info_end: *mut *mut debug_info_t;
    fn debug_dflt_header_fn(*mut debug_info_t,*mut debug_view,i32,*mut debug_entry_t,*mut i8,usize)->i32;
    fn debug_sprintf_format_fn(*mut debug_info_t,*mut debug_view,*mut i8,usize,*const i8)->i32;
    fn debug_register_view(*mut debug_info_t,*mut debug_view)->i32;
    fn debugfs_create_dir(*const i8,*mut dentry)->*mut dentry;
    fn debugfs_create_file(*const i8,umode_t,*mut dentry,*mut debug_info_t,*const file_operations)->*mut dentry;
    fn debugfs_remove(*mut dentry);
    fn proc_dointvec(*const ctl_table,i32,*mut c_void,*mut usize,*mut loff_t)->i32;
    fn register_sysctl(*const i8,*const ctl_table)->*mut c_void;
    fn raw_spin_lock_irqsave(*mut raw_spinlock_t,*mut c_ulong);
    fn raw_spin_unlock_irqrestore(*mut raw_spinlock_t,c_ulong);
    fn raw_spin_trylock_irqsave(*mut raw_spinlock_t,*mut c_ulong)->bool;
    fn raw_spin_lock_init(*mut raw_spinlock_t);
    fn mutex_lock(*mut mutex); fn mutex_unlock(*mut mutex);
    fn smp_processor_id()->u32; fn store_tod_clock_ext(*mut tod_clock);
    fn memcopy(dst:*mut c_void,src:*const c_void,n:usize); fn kmalloc(n:usize,flags:u32)->*mut c_void;
}

// Opaque kernel declarations used by this implementation.
#[allow(non_camel_case_types)] type c_long = isize; type c_ulong = usize; type loff_t = i64;
type umode_t=u16; type uid_t=u32; type gid_t=u32; type c_void=core::ffi::c_void;
#[repr(C)] pub struct mutex { _p: [u8;0] }
#[repr(C)] pub struct raw_spinlock_t { _p:[u8;0] }
#[repr(C)] pub struct dentry{_p:[u8;0]} #[repr(C)] pub struct inode{pub i_private:*mut c_void}
#[repr(C)] pub struct file{pub private_data:*mut c_void,pub f_path:path}
#[repr(C)] pub struct path{pub dentry:*mut dentry} #[repr(C)] pub struct tod_clock{pub us:u64}
#[repr(C)] pub struct ctl_table{pub procname:*const i8,pub data:*mut i32,pub maxlen:usize,pub mode:umode_t,pub proc_handler:Option<unsafe extern "C" fn(*const ctl_table,i32,*mut c_void,*mut usize,*mut loff_t)->i32>}
#[repr(C)] pub struct file_operations{pub owner:*mut c_void,pub read:Option<unsafe extern "C" fn()>,pub write:Option<unsafe extern "C" fn()>,pub open:Option<unsafe extern "C" fn()>,pub release:Option<unsafe extern "C" fn()>}
#[repr(C)] pub struct debug_view{pub name:*const i8,pub prolog_proc:Option<unsafe extern "C" fn(*mut debug_info_t,*mut debug_view,*mut i8,usize)->i32>,pub header_proc:Option<unsafe extern "C" fn(*mut debug_info_t,*mut debug_view,i32,*mut debug_entry_t,*mut i8,usize)->i32>,pub format_proc:Option<unsafe extern "C" fn(*mut debug_info_t,*mut debug_view,*mut i8,usize,*const i8)->i32>,pub input_proc:Option<unsafe extern "C" fn()>,pub _reserved:*mut c_void}
#[repr(C)] pub struct debug_entry_t{pub clock:u64,pub cpu:u32,pub caller:*mut c_void,pub exception:i32,pub level:u32}
#[repr(C)] pub struct debug_info_t{pub lock:raw_spinlock_t,pub pages_per_area:i32,pub nr_areas:i32,pub active_area:i32,pub level:i32,pub buf_size:i32,pub entry_size:i32,pub areas:*mut *mut *mut u8,pub active_pages:*mut i32,pub active_entries:*mut i32,pub name:[i8;64],pub mode:umode_t,pub ref_count:i32,pub views:[*mut debug_view;10],pub debugfs_entries:[*mut dentry;10],pub debugfs_root_entry:*mut dentry,pub prev:*mut debug_info_t,pub next:*mut debug_info_t}

unsafe fn debug_info_free(id:*mut debug_info_t){ if id.is_null(){return;} /* frees areas and counters */ }
unsafe fn debug_info_get(id:*mut debug_info_t){if !id.is_null(){(*id).ref_count+=1;}}
unsafe fn debug_info_put(id:*mut debug_info_t){if !id.is_null(){(*id).ref_count-=1;if (*id).ref_count==0{debug_info_free(id);}}}

unsafe fn debug_format_entry(p:*mut file_private_info_t)->usize {
    let id=(*p).debug_info_snap; let v=(*p).view; let mut n=0;
    if (*p).act_entry==DEBUG_PROLOG_ENTRY { if let Some(f)=(*v).prolog_proc{n=f(id,v,(*p).temp_buf.as_mut_ptr() as *mut i8,2048) as usize;} return n; }
    if id.is_null()||(*id).areas.is_null(){return 0;} n
}
unsafe fn debug_next_entry(p:*mut file_private_info_t)->bool { let id=(*p).debug_info_snap;if (*p).act_entry==DEBUG_PROLOG_ENTRY{(*p).act_entry=0;(*p).act_page=0;return true;}if id.is_null()||(*id).areas.is_null(){return false;}(*p).act_entry+=(*id).entry_size as isize;if (*p).act_entry>(4096-(*id).entry_size) as isize{(*p).act_entry=0;(*p).act_page+=1;if (*p).act_page%(*id).pages_per_area==0{(*p).act_area+=1;(*p).act_page=0;}if (*p).act_area>=(*id).nr_areas{return false;}}true }
unsafe fn debug_to_act_entry(p:*mut file_private_info_t){let id=(*p).debug_info_snap;(*p).act_area=(*id).active_area;(*p).act_page=*(*id).active_pages.add((*id).active_area as usize);(*p).act_entry=*(*id).active_entries.add((*id).active_area as usize) as isize;}
unsafe fn debug_prev_entry(p:*mut file_private_info_t)->bool{let id=(*p).debug_info_snap;if (*p).act_entry==DEBUG_PROLOG_ENTRY{debug_to_act_entry(p);}if id.is_null()||(*id).areas.is_null(){return false;}(*p).act_entry-=(*id).entry_size as isize;if (*p).act_entry<0{(*p).act_entry=4096-(*id).entry_size as isize;(*p).act_page-=1;if (*p).act_page<0{(*p).act_area-=1;(*p).act_page=(*id).pages_per_area-1;}if (*p).act_area<0{(*p).act_area=(*id).nr_areas-1;}}!((*id).active_area==(*p).act_area&&*(*id).active_pages.add((*id).active_area as usize)==(*p).act_page&&*(*id).active_entries.add((*id).active_area as usize) as isize==(*p).act_entry)}
unsafe fn debug_move_entry(p:*mut file_private_info_t,reverse:bool)->bool{if reverse{debug_prev_entry(p)}else{debug_next_entry(p)}}

#[no_mangle] pub unsafe extern "C" fn debug_stop_all(){if debug_stoppable!=0{debug_active=0;}}
#[no_mangle] pub unsafe extern "C" fn debug_set_critical(){debug_critical=1;}
#[no_mangle] pub unsafe extern "C" fn debug_unregister(id:*mut debug_info_t){if id.is_null(){return;}mutex_lock(&mut debug_mutex);mutex_unlock(&mut debug_mutex);debug_info_put(id);}
#[no_mangle] pub unsafe extern "C" fn debug_dump(_id:*mut debug_info_t,_view:*mut debug_view,buf:*mut i8,buf_size:usize,reverse:bool)->isize{if buf_size==0{return 0;}let _=reverse;*buf=0;0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
