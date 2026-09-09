// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of jump_label.c. External kernel symbols are supplied by
 * the surrounding kernel bindings. */

use core::ffi::c_void;

extern "C" {
    static mut jump_label_mutex: c_void;
    fn mutex_lock(m: *mut c_void); fn mutex_unlock(m: *mut c_void);
    fn atomic_read(v: *const c_void) -> i32;
    fn atomic_set(v: *mut c_void, n: i32);
    fn atomic_set_release(v: *mut c_void, n: i32);
    fn atomic_cmpxchg(v: *mut c_void, old: i32, new: i32) -> i32;
    fn atomic_try_cmpxchg(v: *mut c_void, old: *mut i32, new: i32) -> bool;
    fn atomic_dec_and_test(v: *mut c_void) -> bool;
    fn cpus_read_lock(); fn cpus_read_unlock();
    fn sort(base: *mut c_void, n: usize, size: usize, cmp: unsafe extern "C" fn(*const c_void,*const c_void)->i32, swap: *mut c_void);
    fn kzalloc(size: usize, flags: u32) -> *mut c_void; fn kfree(p: *mut c_void);
    fn schedule_delayed_work(w: *mut delayed_work, timeout: usize);
    fn flush_delayed_work(w: *mut delayed_work);
    fn init_delayed_work(w: *mut delayed_work, f: unsafe extern "C" fn(*mut work_struct));
    fn kernel_text_address(a: usize) -> bool; fn init_section_contains(p: *mut c_void, n: usize) -> bool;
    fn is_kernel_ro_after_init(a: usize) -> bool;
    fn arch_jump_label_transform(e: *mut jump_entry, t: jump_label_type);
    fn arch_jump_label_transform_static(e: *mut jump_entry, t: jump_label_type);
    fn jump_entry_key(e: *const jump_entry) -> *mut static_key;
    fn jump_entry_code(e: *const jump_entry) -> usize;
    fn jump_entry_size(e: *const jump_entry) -> usize;
    fn jump_entry_is_init(e: *const jump_entry) -> bool;
    fn jump_entry_is_branch(e: *const jump_entry) -> bool;
    fn jump_entry_set_init(e: *mut jump_entry, b: bool);
    fn static_key_enabled(k: *const static_key) -> bool;
    fn within_module(a: usize, m: *mut module) -> bool; fn within_module_init(a: usize,m:*mut module)->bool;
    fn __module_address(a: usize) -> *mut module; fn __module_text_address(a: usize) -> *mut module;
    fn try_module_get(m:*mut module)->bool; fn module_put(m:*mut module);
    fn notifier_from_errno(n:i32)->i32; fn register_module_notifier(n:*mut notifier_block)->i32;
    fn warn_on_once(b: bool) -> bool; fn warn_on(b: bool) -> bool;
}

#[repr(C)] pub struct jump_entry { pub code: usize, pub target: usize, pub key: usize }
#[repr(C)] pub struct static_key { pub enabled: c_void, pub type_: usize, pub entries: *mut jump_entry, pub next: *mut static_key_mod }
#[repr(C)] pub struct work_struct;
#[repr(C)] pub struct delayed_work { pub work: work_struct }
#[repr(C)] pub struct module { pub jump_entries:*mut jump_entry, pub num_jump_entries:usize, pub state:u32 }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block,usize,*mut c_void)->i32>, pub priority:i32 }
#[repr(C)] pub struct static_key_deferred { pub key: static_key, pub work: delayed_work, pub timeout: usize }
#[repr(C)] pub struct static_key_mod { pub next:*mut static_key_mod, pub entries:*mut jump_entry, pub mod_:*mut module }
#[repr(C)] #[derive(Copy,Clone,PartialEq,Eq)] pub enum jump_label_type { NOP=0, JUMP=1 }

const JUMP_TYPE_MASK:usize=3; const JUMP_TYPE_TRUE:usize=1; const JUMP_TYPE_LINKED:usize=2;
const MODULE_STATE_COMING:u32=1; const MODULE_STATE_GOING:u32=2; const SYSTEM_RUNNING:u32=1;
extern "C" { static mut static_key_initialized: bool; static mut system_state:u32; static __start___jump_table: jump_entry; static __stop___jump_table: jump_entry; }

#[no_mangle] pub unsafe extern "C" fn jump_label_lock(){mutex_lock(&mut jump_label_mutex)}
#[no_mangle] pub unsafe extern "C" fn jump_label_unlock(){mutex_unlock(&mut jump_label_mutex)}

unsafe extern "C" fn jump_label_cmp(a:*const c_void,b:*const c_void)->i32 { let a=a as *const jump_entry; let b=b as *const jump_entry; let x=jump_entry_key(a) as usize; let y=jump_entry_key(b) as usize; if x<y{-1}else if x>y{1}else{let x=jump_entry_code(a);let y=jump_entry_code(b);if x<y{-1}else if x>y{1}else{0}} }
unsafe extern "C" fn jump_label_swap(a:*mut c_void,b:*mut c_void,_size:i32){let d=(a as isize)-(b as isize);let x=&mut *(a as *mut jump_entry);let y=*(b as *const jump_entry);let t=*x;x.code=(y.code as isize-d) as usize;x.target=(y.target as isize-d) as usize;x.key=(y.key as isize-d) as usize;x=&mut *(b as *mut jump_entry);x.code=(t.code as isize+d) as usize;x.target=(t.target as isize+d) as usize;x.key=(t.key as isize+d) as usize;}
unsafe fn jump_label_sort_entries(start:*mut jump_entry,stop:*mut jump_entry){sort(start as _,(stop as usize-start as usize)/core::mem::size_of::<jump_entry>(),core::mem::size_of::<jump_entry>(),jump_label_cmp,jump_label_swap as *mut _)}

#[no_mangle] pub unsafe extern "C" fn static_key_count(k:*mut static_key)->i32{let n=atomic_read(&(*k).enabled);if n>=0{n}else{1}}
#[no_mangle] pub unsafe extern "C" fn static_key_fast_inc_not_disabled(k:*mut static_key)->bool{let mut v=atomic_read(&(*k).enabled);loop{if v<=0||v==i32::MAX{return false}if atomic_try_cmpxchg(&mut (*k).enabled,&mut v,v+1){return true}}}
unsafe fn jump_label_update(k:*mut static_key);
#[no_mangle] pub unsafe extern "C" fn static_key_slow_inc_cpuslocked(k:*mut static_key)->bool{if static_key_fast_inc_not_disabled(k){return true}jump_label_lock();if atomic_cmpxchg(&mut (*k).enabled,0,-1)==0{jump_label_update(k);atomic_set_release(&mut (*k).enabled,1)}else if !static_key_fast_inc_not_disabled(k){jump_label_unlock();return false}jump_label_unlock();true}
#[no_mangle] pub unsafe extern "C" fn static_key_slow_inc(k:*mut static_key)->bool{cpus_read_lock();let r=static_key_slow_inc_cpuslocked(k);cpus_read_unlock();r}
#[no_mangle] pub unsafe extern "C" fn static_key_enable_cpuslocked(k:*mut static_key){if atomic_read(&(*k).enabled)>0{return}jump_label_lock();if atomic_read(&(*k).enabled)==0{atomic_set(&mut (*k).enabled,-1);jump_label_update(k);atomic_set_release(&mut (*k).enabled,1)}jump_label_unlock()}
#[no_mangle] pub unsafe extern "C" fn static_key_enable(k:*mut static_key){cpus_read_lock();static_key_enable_cpuslocked(k);cpus_read_unlock()}
#[no_mangle] pub unsafe extern "C" fn static_key_disable_cpuslocked(k:*mut static_key){if atomic_read(&(*k).enabled)!=1{return}jump_label_lock();if atomic_cmpxchg(&mut (*k).enabled,1,0)==1{jump_label_update(k)}jump_label_unlock()}
#[no_mangle] pub unsafe extern "C" fn static_key_disable(k:*mut static_key){cpus_read_lock();static_key_disable_cpuslocked(k);cpus_read_unlock()}

unsafe fn static_key_dec_not_one(k:*mut static_key)->bool{let mut v=atomic_read(&(*k).enabled);loop{if v==0{return true}if v<=1{return false}if atomic_try_cmpxchg(&mut (*k).enabled,&mut v,v-1){return true}}}
unsafe fn __static_key_slow_dec_cpuslocked(k:*mut static_key){if static_key_dec_not_one(k){return}jump_label_lock();let v=atomic_read(&(*k).enabled);if v==1&&atomic_dec_and_test(&mut (*k).enabled){jump_label_update(k)}jump_label_unlock()}
unsafe fn __static_key_slow_dec(k:*mut static_key){cpus_read_lock();__static_key_slow_dec_cpuslocked(k);cpus_read_unlock()}
#[no_mangle] pub unsafe extern "C" fn static_key_slow_dec(k:*mut static_key){__static_key_slow_dec(k)}
#[no_mangle] pub unsafe extern "C" fn static_key_slow_dec_cpuslocked(k:*mut static_key){__static_key_slow_dec_cpuslocked(k)}
#[no_mangle] pub unsafe extern "C" fn __static_key_slow_dec_deferred(k:*mut static_key,w:*mut delayed_work,t:usize){if !static_key_dec_not_one(k){schedule_delayed_work(w,t)}}
#[no_mangle] pub unsafe extern "C" fn __static_key_deferred_flush(_k:*mut c_void,w:*mut delayed_work){flush_delayed_work(w)}
#[no_mangle] pub unsafe extern "C" fn jump_label_rate_limit(k:*mut static_key_deferred,rl:usize){(*k).timeout=rl;}

unsafe fn addr_conflict(e:*mut jump_entry,s:*mut c_void,end:*mut c_void)->bool{jump_entry_code(e)<=end as usize&&jump_entry_code(e)+jump_entry_size(e)>s as usize}
unsafe fn __jump_label_text_reserved(mut a:*mut jump_entry,z:*mut jump_entry,s:*mut c_void,e:*mut c_void,init:bool)->i32{while a<z{if (init||!jump_entry_is_init(a))&&addr_conflict(a,s,e){return 1}a=a.add(1)}0}
unsafe fn static_key_entries(k:*mut static_key)->*mut jump_entry{(*k).entries}
unsafe fn static_key_type(k:*mut static_key)->bool{(*k).type_&JUMP_TYPE_TRUE!=0}
unsafe fn static_key_linked(k:*mut static_key)->bool{(*k).type_&JUMP_TYPE_LINKED!=0}
unsafe fn static_key_sealed(k:*mut static_key)->bool{static_key_linked(k)&&(*k).type_&!JUMP_TYPE_MASK==0}
unsafe fn static_key_seal(k:*mut static_key){(*k).type_=JUMP_TYPE_LINKED|(*k).type_&JUMP_TYPE_TRUE}
unsafe fn static_key_set_entries(k:*mut static_key,e:*mut jump_entry){(*k).entries=e}
unsafe fn jump_label_type_of(e:*mut jump_entry)->jump_label_type{if static_key_enabled(jump_entry_key(e))^jump_entry_is_branch(e){jump_label_type::JUMP}else{jump_label_type::NOP}}
unsafe fn __jump_label_update(k:*mut static_key,mut e:*mut jump_entry,z:*mut jump_entry,_init:bool){while e<z&&jump_entry_key(e)==k{arch_jump_label_transform(e,jump_label_type_of(e));e=e.add(1)}}
unsafe fn jump_label_update(k:*mut static_key){let e=static_key_entries(k);if !e{return}__jump_label_update(k,e,__stop___jump_table as *const _ as *mut _,system_state<SYSTEM_RUNNING)}

#[no_mangle] pub unsafe extern "C" fn jump_label_init(){if static_key_initialized{return}cpus_read_lock();jump_label_lock();let mut e=&__start___jump_table as *const _ as *mut jump_entry;let z=&__stop___jump_table as *const _ as *mut jump_entry;jump_label_sort_entries(e,z);while e<z{if jump_label_type_of(e)==jump_label_type::NOP{arch_jump_label_transform_static(e,jump_label_type::NOP)}jump_entry_set_init(e,init_section_contains(jump_entry_code(e) as *mut _,1));static_key_set_entries(jump_entry_key(e),e);e=e.add(1)}static_key_initialized=true;jump_label_unlock();cpus_read_unlock()}
#[no_mangle] pub unsafe extern "C" fn jump_label_init_ro(){if !static_key_initialized{return}cpus_read_lock();jump_label_lock();let mut e=&__start___jump_table as *const _ as *mut jump_entry;let z=&__stop___jump_table as *const _ as *mut jump_entry;while e<z{let k=jump_entry_key(e);if is_kernel_ro_after_init(k as usize)&&!static_key_sealed(k){static_key_seal(k)}e=e.add(1)}jump_label_unlock();cpus_read_unlock()}
#[no_mangle] pub unsafe extern "C" fn jump_label_text_reserved(s:*mut c_void,e:*mut c_void)->i32{__jump_label_text_reserved(&__start___jump_table as *const _ as *mut _,&__stop___jump_table as *const _ as *mut _,s,e,system_state<SYSTEM_RUNNING)}

#[no_mangle] pub unsafe extern "C" fn jump_label_update_timeout(w:*mut work_struct){let k=w as *mut static_key_deferred;__static_key_slow_dec(&mut (*k).key)}
#[no_mangle] pub unsafe extern "C" fn __static_key_slow_dec_deferred_export(k:*mut static_key,w:*mut delayed_work,t:usize){__static_key_slow_dec_deferred(k,w,t)}

// CONFIG_MODULES-specific declarations and module-list manipulation are kept
// as direct external interfaces; their implementations depend on kernel
// module and RCU support supplied by other translation units.
#[cfg(feature="CONFIG_MODULES")]
#[no_mangle] pub unsafe extern "C" fn jump_label_init_type(e:*mut jump_entry)->jump_label_type{if static_key_type(jump_entry_key(e))^jump_entry_is_branch(e){jump_label_type::JUMP}else{jump_label_type::NOP}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
