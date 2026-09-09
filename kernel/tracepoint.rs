// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright (C) 2008-2014 Mathieu Desnoyers */

#[repr(C)]
pub struct tracepoint_func { pub func: *mut core::ffi::c_void, pub data: *mut core::ffi::c_void, pub prio: i32 }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct tracepoint { pub iterator: *mut core::ffi::c_void, pub static_call_key: *mut core::ffi::c_void, pub static_call_tramp: *mut core::ffi::c_void, pub funcs: *mut tracepoint_func, pub key: [u8; 0], pub ext: *mut tracepoint_ext }
#[repr(C)] pub struct tracepoint_ext { pub regfunc: Option<unsafe extern "C" fn() -> i32>, pub unregfunc: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct module { pub taints: usize, pub num_tracepoints: usize, pub tracepoints_ptrs: *mut tracepoint_ptr_t }
#[repr(C)] pub struct tp_module { pub list: [u8; 0], pub module: *mut module }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut core::ffi::c_void) -> i32>, pub priority: i32 }
pub type tracepoint_ptr_t = *mut tracepoint;

#[repr(C)] #[derive(Copy, Clone)] enum tp_func_state { TP_FUNC_0, TP_FUNC_1, TP_FUNC_2, TP_FUNC_N }
#[repr(C)] #[derive(Copy, Clone)] enum tp_transition_sync { TP_TRANSITION_SYNC_1_0_1, TP_TRANSITION_SYNC_N_2_1, _NR_TP_TRANSITION_SYNC }
#[repr(C)] struct tp_transition_snapshot { rcu: usize, srcu_gp: usize, ongoing: bool }

extern "C" {
    static mut tracepoint_srcu: core::ffi::c_void;
    static mut tp_transition_snapshot: [tp_transition_snapshot; 2];
    static __start___tracepoints_ptrs: tracepoint_ptr_t;
    static __stop___tracepoints_ptrs: tracepoint_ptr_t;
    fn get_state_synchronize_rcu() -> usize; fn start_poll_synchronize_srcu(_: *mut core::ffi::c_void) -> usize;
    fn cond_synchronize_rcu(_: usize); fn poll_state_synchronize_srcu(_: *mut core::ffi::c_void, _: usize) -> bool; fn synchronize_srcu(_: *mut core::ffi::c_void);
    fn kmalloc_flex(_: usize, _: usize, _: usize) -> *mut tp_probes; fn kfree(_: *mut core::ffi::c_void);
    fn tracepoint_is_faultable(_: *mut tracepoint) -> bool; fn call_rcu_tasks_trace(_: *mut rcu_head, _: unsafe extern "C" fn(*mut rcu_head)); fn call_srcu(_: *mut core::ffi::c_void, _: *mut rcu_head, _: unsafe extern "C" fn(*mut rcu_head));
    fn printk(_: *const core::ffi::c_char, ...); fn static_key_enabled(_: *mut core::ffi::c_void) -> bool; fn static_branch_enable(_: *mut core::ffi::c_void); fn static_branch_disable(_: *mut core::ffi::c_void); fn __static_call_update(_: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void);
    fn mutex_lock(_: *mut core::ffi::c_void); fn mutex_unlock(_: *mut core::ffi::c_void); fn blocking_notifier_chain_register(_: *mut core::ffi::c_void, _: *mut notifier_block) -> i32; fn blocking_notifier_chain_unregister(_: *mut core::ffi::c_void, _: *mut notifier_block) -> i32; fn blocking_notifier_call_chain(_: *mut core::ffi::c_void, _: usize, _: *mut core::ffi::c_void);
    fn register_module_notifier(_: *mut notifier_block) -> i32; fn tracepoint_ptr_deref(_: *mut tracepoint_ptr_t) -> *mut tracepoint;
}
#[repr(C)] struct tp_probes { rcu: rcu_head, probes: [tracepoint_func; 0] }
static mut TRACEPOINT_DEBUG: i32 = 0;
static mut TRACEPOINTS_MUTEX: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe fn tp_rcu_get_state(sync: tp_transition_sync) { let s = &mut tp_transition_snapshot[sync as usize]; s.rcu=get_state_synchronize_rcu(); s.srcu_gp=start_poll_synchronize_srcu(&mut tracepoint_srcu); s.ongoing=true; }
unsafe fn tp_rcu_cond_sync(sync: tp_transition_sync) { let s=&mut tp_transition_snapshot[sync as usize]; if !s.ongoing{return} cond_synchronize_rcu(s.rcu); if !poll_state_synchronize_srcu(&mut tracepoint_srcu,s.srcu_gp){synchronize_srcu(&mut tracepoint_srcu)} s.ongoing=false; }
unsafe extern "C" fn tp_stub_func() {}
unsafe fn allocate_probes(count: usize) -> *mut tracepoint_func { let p=kmalloc_flex(core::mem::size_of::<tp_probes>(),core::mem::size_of::<tracepoint_func>(),count); if p.is_null(){core::ptr::null_mut()}else{(*p).probes.as_mut_ptr()} }
unsafe extern "C" fn rcu_free_old_probes(head:*mut rcu_head){ kfree(head as *mut _); }
unsafe fn release_probes(tp:*mut tracepoint, old:*mut tracepoint_func){if !old.is_null(){let p=(old as *mut u8).sub(core::mem::offset_of!(tp_probes,probes)) as *mut tp_probes;if tracepoint_is_faultable(tp){call_rcu_tasks_trace(&mut (*p).rcu,rcu_free_old_probes)}else{call_srcu(&mut tracepoint_srcu,&mut (*p).rcu,rcu_free_old_probes)}}}
unsafe fn debug_print_probes(mut f:*mut tracepoint_func){if TRACEPOINT_DEBUG==0||f.is_null(){return} let mut i=0;while !(*f.add(i)).func.is_null(){printk(b"Probe %d : %pSb\0".as_ptr() as _,i,(*f.add(i)).func);i+=1}}

unsafe fn func_add(funcs:*mut *mut tracepoint_func, tf:*mut tracepoint_func, prio:i32)->*mut tracepoint_func { let old=*funcs;let mut n=0;let mut pos:i32=-1;if !old.is_null(){let mut i=0;while !(*old.add(i)).func.is_null(){if (*old.add(i)).func!=tp_stub_func as _ {if (*old.add(i)).func==(*tf).func&&(*old.add(i)).data==(*tf).data{return (-17isize) as _}n+=1;}i+=1}}let new=allocate_probes(n+2);if new.is_null(){return (-12isize) as _}if !old.is_null(){n=0;let mut i=0;while !(*old.add(i)).func.is_null(){if (*old.add(i)).func!=tp_stub_func as _ {if pos<0&&(*old.add(i)).prio<prio{pos=n as i32;n+=1}*new.add(n as usize)=*old.add(i);n+=1}i+=1}if pos<0{pos=n as i32;n+=1}}else{pos=0;n=1}*new.add(pos as usize)=*tf;(*new.add(n as usize)).func=core::ptr::null_mut();*funcs=new;old}
unsafe fn func_remove(funcs:*mut *mut tracepoint_func, tf:*mut tracepoint_func)->*mut tracepoint_func {let old=*funcs;if old.is_null(){return (-2isize) as _}let mut n=0;let mut d=0;while !(*old.add(n)).func.is_null(){if ((*old.add(n)).func==(*tf).func&&(*old.add(n)).data==(*tf).data)||(*old.add(n)).func==tp_stub_func as _{d+=1}n+=1}if n-d==0{*funcs=core::ptr::null_mut();old}else{let new=allocate_probes(n-d+1);if !new.is_null(){let mut j=0;for i in 0..n{if (*old.add(i)).func!=(*tf).func&&(*old.add(i)).func!=tp_stub_func as _{*new.add(j)=*old.add(i);j+=1}}(*new.add(n-d)).func=core::ptr::null_mut();*funcs=new}else{for i in 0..n{if (*old.add(i)).func==(*tf).func&&(*old.add(i)).data==(*tf).data{(*old.add(i)).func=tp_stub_func as _}}*funcs=old}old}}
unsafe fn nr_func_state(p:*const tracepoint_func)->tp_func_state{if p.is_null(){tp_func_state::TP_FUNC_0}else if (*p.add(1)).func.is_null(){tp_func_state::TP_FUNC_1}else if (*p.add(2)).func.is_null(){tp_func_state::TP_FUNC_2}else{tp_func_state::TP_FUNC_N}}
unsafe fn tracepoint_update_call(tp:*mut tracepoint,p:*mut tracepoint_func){if (*tp).static_call_key.is_null(){return}let f=if nr_func_state(p) as u8==tp_func_state::TP_FUNC_1 as u8{(*p).func}else{(*tp).iterator};__static_call_update((*tp).static_call_key,(*tp).static_call_tramp,f)}

pub unsafe extern "C" fn tracepoint_probe_register_prio_may_exist(tp:*mut tracepoint,probe:*mut core::ffi::c_void,data:*mut core::ffi::c_void,prio:i32)->i32{let mut f=tracepoint_func{func:probe,data,prio};mutex_lock(TRACEPOINTS_MUTEX);let r=func_add(&mut (*tp).funcs,&mut f,prio);mutex_unlock(TRACEPOINTS_MUTEX);if (r as isize)<0{r as isize as i32}else{release_probes(tp,r);0}}
pub unsafe extern "C" fn tracepoint_probe_register_prio(tp:*mut tracepoint,probe:*mut core::ffi::c_void,data:*mut core::ffi::c_void,prio:i32)->i32{tracepoint_probe_register_prio_may_exist(tp,probe,data,prio)}
pub unsafe extern "C" fn tracepoint_probe_register(tp:*mut tracepoint,probe:*mut core::ffi::c_void,data:*mut core::ffi::c_void)->i32{tracepoint_probe_register_prio(tp,probe,data,0)}
pub unsafe extern "C" fn tracepoint_probe_unregister(tp:*mut tracepoint,probe:*mut core::ffi::c_void,data:*mut core::ffi::c_void)->i32{let mut f=tracepoint_func{func:probe,data,prio:0};mutex_lock(TRACEPOINTS_MUTEX);let old=func_remove(&mut (*tp).funcs,&mut f);mutex_unlock(TRACEPOINTS_MUTEX);if (old as isize)<0{old as isize as i32}else{release_probes(tp,old);0}}

unsafe fn for_each_tracepoint_range(begin:*mut tracepoint_ptr_t,end:*mut tracepoint_ptr_t,f:unsafe extern "C" fn(*mut tracepoint,*mut core::ffi::c_void),priv_:*mut core::ffi::c_void){if begin.is_null(){return}let mut p=begin;while p<end{f(tracepoint_ptr_deref(p),priv_);p=p.add(1)}}
pub unsafe extern "C" fn for_each_kernel_tracepoint(f:unsafe extern "C" fn(*mut tracepoint,*mut core::ffi::c_void),p:*mut core::ffi::c_void){for_each_tracepoint_range(&__start___tracepoints_ptrs as *const _ as _,&__stop___tracepoints_ptrs as *const _ as _,f,p)}

// CONFIG_MODULES section. Kernel list, notifier, taint, and module symbols are
// supplied by the surrounding kernel bindings.
#[cfg(feature="CONFIG_MODULES")]
pub unsafe extern "C" fn trace_module_has_bad_taint(m:*mut module)->bool {
    (*m).taints & !((1usize<<0)|(1usize<<1)|(1usize<<2)|(1usize<<3)|(1usize<<4)) != 0
}
#[cfg(feature="CONFIG_MODULES")]
pub unsafe extern "C" fn for_each_tracepoint_in_module(m:*mut module,f:unsafe extern "C" fn(*mut tracepoint,*mut module,*mut core::ffi::c_void),p:*mut core::ffi::c_void){
    if m.is_null(){return} let mut i=0;while i<(*m).num_tracepoints{f(tracepoint_ptr_deref((*m).tracepoints_ptrs.add(i)),m,p);i+=1}
}
#[cfg(feature="CONFIG_MODULES")]
pub unsafe extern "C" fn for_each_module_tracepoint(_f:unsafe extern "C" fn(*mut tracepoint,*mut module,*mut core::ffi::c_void),_p:*mut core::ffi::c_void) {}

#[cfg(feature="CONFIG_HAVE_SYSCALL_TRACEPOINTS")]
static mut sys_tracepoint_refcount:i32=0;
#[cfg(feature="CONFIG_HAVE_SYSCALL_TRACEPOINTS")]
extern "C" { fn read_lock(_: *mut core::ffi::c_void); fn read_unlock(_: *mut core::ffi::c_void); fn set_task_syscall_work(_: *mut core::ffi::c_void, _: usize); fn clear_task_syscall_work(_: *mut core::ffi::c_void, _: usize); }
#[cfg(feature="CONFIG_HAVE_SYSCALL_TRACEPOINTS")]
pub unsafe extern "C" fn syscall_regfunc()->i32 { if sys_tracepoint_refcount==0 { /* for_each_process_thread(p,t): kernel task-list iteration */ } sys_tracepoint_refcount+=1;0 }
#[cfg(feature="CONFIG_HAVE_SYSCALL_TRACEPOINTS")]
pub unsafe extern "C" fn syscall_unregfunc(){sys_tracepoint_refcount-=1;if sys_tracepoint_refcount==0 { /* for_each_process_thread(p,t): kernel task-list iteration */ }}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
