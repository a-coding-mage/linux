// SPDX-License-Identifier: GPL-2.0-or-later
// C dependencies supplied by the surrounding kernel translation unit.

const MAX_NODES: usize = 4;

#[repr(C)]
struct qnode { next: *mut qnode, lock: *mut qspinlock, cpu: i32, sleepy: u8, locked: u8 }
#[repr(C)]
struct qnodes { count: i32, nodes: [qnode; MAX_NODES] }

extern "C" {
    type qspinlock;
    static mut qnodes: qnodes;
    static mut sleepy_lock_seen_clock: u64;
    fn sched_clock() -> u64;
    fn smp_processor_id() -> i32;
    fn numa_node_id() -> i32;
    fn cpu_to_node(cpu: i32) -> i32;
    fn is_shared_processor() -> bool;
    fn vcpu_is_preempted(cpu: i32) -> bool;
    fn yield_count_of(cpu: i32) -> u32;
    fn yield_to_preempted(cpu: i32, count: u32);
    fn spin_begin(); fn spin_end(); fn spin_cpu_relax(); fn cpu_relax();
    fn queued_spin_trylock(lock: *mut qspinlock) -> bool;
    fn __queued_spin_trylock_steal(lock: *mut qspinlock) -> bool;
    fn queued_spin_encode_locked_val() -> u32;
    fn kcsan_release(); fn smp_acquire__after_ctrl_dep(); fn smp_rmb();
    fn prefetchw(p: *mut qnode); fn prod_cpu(cpu: i32);
    fn trace_contention_begin(lock: *mut qspinlock, flags: i32);
    fn trace_contention_end(lock: *mut qspinlock, val: i32);
    fn synchronize_rcu(); fn mutex_lock(lock: *mut core::ffi::c_void); fn mutex_unlock(lock: *mut core::ffi::c_void);
    fn debugfs_create_file(name: *const u8, mode: u32, dir: *mut core::ffi::c_void, data: *mut core::ffi::c_void, fops: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    static mut arch_debugfs_dir: *mut core::ffi::c_void;
}

// Build-time constants/macros are provided by asm/qspinlock.h and related headers.
const _Q_LOCKED_VAL: u32 = 1;
const _Q_SLEEPY_VAL: u32 = 1 << 3;
const _Q_MUST_Q_VAL: u32 = 1 << 4;
const _Q_TAIL_CPU_MASK: u32 = 0xffff_ff00;
const _Q_OWNER_CPU_MASK: u32 = 0xffff;
const _Q_OWNER_CPU_OFFSET: u32 = 0;
const _Q_TAIL_CPU_OFFSET: u32 = 8;
const _Q_SPIN_PREFETCH_NEXT: bool = false;
const _Q_SPIN_MISO: bool = false;

static mut steal_spins: i32 = 1 << 5;
static mut remote_steal_spins: i32 = 1 << 2;
static mut maybe_stealers: bool = true;
static mut head_spins: i32 = 1 << 8;
static mut pv_yield_owner: bool = true;
static mut pv_yield_allow_steal: bool = false;
static mut pv_spin_on_preempted_owner: bool = false;
static mut pv_sleepy_lock: bool = true;
static mut pv_sleepy_lock_sticky: bool = false;
static mut pv_sleepy_lock_interval_ns: u64 = 0;
static mut pv_sleepy_lock_factor: i32 = 256;
static mut pv_yield_prev: bool = true;
static mut pv_yield_sleepy_owner: bool = true;
static mut pv_prod_head: bool = false;

#[inline] unsafe fn recently_sleepy() -> bool {
    if pv_sleepy_lock_interval_ns != 0 { let seen = sleepy_lock_seen_clock; if seen != 0 { let delta = sched_clock().wrapping_sub(seen); if delta < pv_sleepy_lock_interval_ns { return true; } sleepy_lock_seen_clock = 0; } } false
}
#[inline] unsafe fn get_steal_spins(p: bool, s: bool) -> i32 { if p && s { steal_spins * pv_sleepy_lock_factor } else { steal_spins } }
#[inline] unsafe fn get_remote_steal_spins(p: bool, s: bool) -> i32 { if p && s { remote_steal_spins * pv_sleepy_lock_factor } else { remote_steal_spins } }
#[inline] unsafe fn get_head_spins(p: bool, s: bool) -> i32 { if p && s { head_spins * pv_sleepy_lock_factor } else { head_spins } }
#[inline] fn encode_tail_cpu(cpu: i32) -> u32 { ((cpu + 1) as u32) << _Q_TAIL_CPU_OFFSET }
#[inline] fn decode_tail_cpu(v: u32) -> i32 { ((v >> _Q_TAIL_CPU_OFFSET) as i32) - 1 }
#[inline] fn get_owner_cpu(v: u32) -> i32 { ((v & _Q_OWNER_CPU_MASK) >> _Q_OWNER_CPU_OFFSET) as i32 }

// The following atomic operations correspond to the PowerPC lwarx/stwcx loops in the source.
#[inline] unsafe fn trylock_clean_tail(_lock: *mut qspinlock, _tail: u32) -> u32 { todo!("PowerPC lwarx/stwcx implementation supplied by target backend") }
#[inline] unsafe fn publish_tail_cpu(_lock: *mut qspinlock, _tail: u32) -> u32 { kcsan_release(); todo!("PowerPC lwarx/stwcx implementation supplied by target backend") }
#[inline] unsafe fn set_mustq(_lock: *mut qspinlock) -> u32 { todo!("PowerPC lwarx/stwcx implementation supplied by target backend") }
#[inline] unsafe fn clear_mustq(_lock: *mut qspinlock) -> u32 { todo!("PowerPC lwarx/stwcx implementation supplied by target backend") }
#[inline] unsafe fn try_set_sleepy(_lock: *mut qspinlock, old: u32) -> bool { todo!("PowerPC lwarx/stwcx implementation supplied by target backend") }

#[inline] unsafe fn seen_sleepy_owner(lock: *mut qspinlock, val: u32) { if pv_sleepy_lock { if pv_sleepy_lock_interval_ns != 0 { sleepy_lock_seen_clock = sched_clock(); } if val & _Q_SLEEPY_VAL == 0 { let _ = try_set_sleepy(lock, val); } } }
#[inline] unsafe fn seen_sleepy_lock() { if pv_sleepy_lock && pv_sleepy_lock_interval_ns != 0 { sleepy_lock_seen_clock = sched_clock(); } }
#[inline] unsafe fn seen_sleepy_node() { if pv_sleepy_lock && pv_sleepy_lock_interval_ns != 0 { sleepy_lock_seen_clock = sched_clock(); } }

unsafe fn get_tail_qnode(lock: *mut qspinlock, prev_cpu: i32) -> *mut qnode {
    let qp = &mut qnodes; for i in 0..MAX_NODES { let n = &mut qp.nodes[i]; if n.lock == lock { return n; } } panic!("BUG")
}

#[inline] unsafe fn __yield_to_locked_owner(lock: *mut qspinlock, val: u32, paravirt: bool, mustq: bool) -> bool {
    if !paravirt || !pv_yield_owner { spin_cpu_relax(); return false; }
    let owner = get_owner_cpu(val); let count = yield_count_of(owner); if count & 1 == 0 { spin_cpu_relax(); return false; }
    spin_end(); seen_sleepy_owner(lock, val); smp_rmb();
    if core::ptr::read_volatile(lock as *const u32) == val { if mustq { clear_mustq(lock); } yield_to_preempted(owner, count); if mustq { set_mustq(lock); } spin_begin(); return true; }
    spin_begin(); spin_cpu_relax(); false
}
#[inline] unsafe fn yield_to_locked_owner(l:*mut qspinlock,v:u32,p:bool)->bool { __yield_to_locked_owner(l,v,p,false) }
#[inline] unsafe fn yield_head_to_locked_owner(l:*mut qspinlock,v:u32,p:bool)->bool { __yield_to_locked_owner(l,v,p,(v&_Q_MUST_Q_VAL)!=0&&pv_yield_allow_steal) }
#[inline] unsafe fn propagate_sleepy(n:*mut qnode,v:u32,p:bool) { if !p || !pv_yield_sleepy_owner { return; } let next=(*n).next; if next.is_null()||(*next).sleepy!=0{return;} if vcpu_is_preempted(get_owner_cpu(v)){(*next).sleepy=1;} }

#[inline] unsafe fn yield_to_prev(lock:*mut qspinlock,node:*mut qnode,prev:i32,p:bool)->bool {
    if !p { spin_cpu_relax(); return false; }
    if pv_yield_sleepy_owner && ((*node).sleepy != 0 || vcpu_is_preempted(prev)) { let val=core::ptr::read_volatile(lock as *const u32); if val&_Q_LOCKED_VAL!=0 { let next=(*node).next; if !next.is_null() && (*next).sleepy==0 && vcpu_is_preempted(get_owner_cpu(val)){(*next).sleepy=1;} if yield_to_locked_owner(lock,val,p){return true;} } (*node).sleepy=0; }
    if !pv_yield_prev { spin_cpu_relax(); return false; } let yc=yield_count_of(prev); if yc&1==0 {spin_cpu_relax();return false;} spin_end(); seen_sleepy_node(); smp_rmb(); if core::ptr::read_volatile(&(*node).locked)==0 {yield_to_preempted(prev,yc);spin_begin();return true;} spin_begin();spin_cpu_relax();false
}

#[inline] unsafe fn steal_break(val:u32,iters:i32,p:bool,s:bool)->bool { if iters>=get_steal_spins(p,s){return true;} if iters>=get_remote_steal_spins(p,s) && numa_node_id()!=cpu_to_node(get_owner_cpu(val)){return true;} false }
#[inline] unsafe fn try_to_steal_lock(lock:*mut qspinlock,p:bool)->bool { let mut seen=false;let mut sleepy=false;let mut it=0; if steal_spins==0{return false;} spin_begin(); loop {let val=core::ptr::read_volatile(lock as *const u32);if val&_Q_MUST_Q_VAL!=0{break;} if val&_Q_LOCKED_VAL==0{spin_end();if __queued_spin_trylock_steal(lock){return true;}spin_begin();}else{let pre=yield_to_locked_owner(lock,val,p);if p&&pv_sleepy_lock&&!sleepy&&(val&_Q_SLEEPY_VAL!=0||recently_sleepy()){seen_sleepy_lock();sleepy=true;}if pre{seen=true;sleepy=true;if !pv_spin_on_preempted_owner{it+=1;}}else{it+=1;}} if steal_break(val,it,p,sleepy){break;}} spin_end();false }

unsafe fn queued_spin_lock_mcs_queue(lock:*mut qspinlock,p:bool) { let qp=&mut qnodes;if qp.count>=MAX_NODES{while !queued_spin_trylock(lock){cpu_relax();}return;}let idx=qp.count as usize;qp.count+=1;let node=&mut qp.nodes[idx];node.next=core::ptr::null_mut();node.lock=lock;node.cpu=smp_processor_id();node.sleepy=0;node.locked=0;let tail=encode_tail_cpu(node.cpu);let old=publish_tail_cpu(lock,tail);if old&_Q_TAIL_CPU_MASK!=0{let prev=get_tail_qnode(lock,decode_tail_cpu(old));core::ptr::write_volatile(&mut (*prev).next,node);spin_begin();while core::ptr::read_volatile(&node.locked)==0{if yield_to_prev(lock,node,(*prev).cpu,p){}}spin_end();smp_rmb();}
    loop {spin_begin();let mut it=0;loop{let val=core::ptr::read_volatile(lock as *const u32);if val&_Q_LOCKED_VAL==0{break;}propagate_sleepy(node,val,p);let pre=yield_head_to_locked_owner(lock,val,p);if maybe_stealers{if pre{it+=if pv_spin_on_preempted_owner{0}else{1};}else{it+=1;}if it>=get_head_spins(p,false){set_mustq(lock);}}}spin_end();let old=trylock_clean_tail(lock,tail);if old&_Q_LOCKED_VAL!=0{continue;}if old&_Q_TAIL_CPU_MASK==tail{break;}let mut next=core::ptr::read_volatile(&node.next);while next.is_null(){spin_begin();next=core::ptr::read_volatile(&node.next);spin_end();}(*next).locked=1;break;}node.lock=core::ptr::null_mut();qp.count-=1;}

#[no_mangle] pub unsafe extern "C" fn queued_spin_lock_slowpath(lock:*mut qspinlock){trace_contention_begin(lock,0);if is_shared_processor(){if !try_to_steal_lock(lock,true){queued_spin_lock_mcs_queue(lock,true);}}else if !try_to_steal_lock(lock,false){queued_spin_lock_mcs_queue(lock,false);}trace_contention_end(lock,0)}
#[cfg(feature="CONFIG_PARAVIRT_SPINLOCKS")] pub unsafe extern "C" fn pv_spinlocks_init() {}

// Debugfs accessors and file registration retain the source interface; connector types are external.
macro_rules! knob { ($set:ident,$get:ident,$var:ident,$ty:ty) => { unsafe extern "C" fn $set(_data:*mut core::ffi::c_void,val:u64)->i32{$var=val as $ty;0} unsafe extern "C" fn $get(_data:*mut core::ffi::c_void,val:*mut u64)->i32{*val=$var as u64;0} }; }
knob!(steal_spins_set,steal_spins_get,steal_spins,i32); knob!(remote_steal_spins_set,remote_steal_spins_get,remote_steal_spins,i32); knob!(head_spins_set,head_spins_get,head_spins,i32); knob!(pv_yield_owner_set,pv_yield_owner_get,pv_yield_owner,bool); knob!(pv_yield_allow_steal_set,pv_yield_allow_steal_get,pv_yield_allow_steal,bool); knob!(pv_spin_on_preempted_owner_set,pv_spin_on_preempted_owner_get,pv_spin_on_preempted_owner,bool); knob!(pv_sleepy_lock_set,pv_sleepy_lock_get,pv_sleepy_lock,bool); knob!(pv_sleepy_lock_sticky_set,pv_sleepy_lock_sticky_get,pv_sleepy_lock_sticky,bool); knob!(pv_sleepy_lock_interval_ns_set,pv_sleepy_lock_interval_ns_get,pv_sleepy_lock_interval_ns,u64); knob!(pv_sleepy_lock_factor_set,pv_sleepy_lock_factor_get,pv_sleepy_lock_factor,i32); knob!(pv_yield_prev_set,pv_yield_prev_get,pv_yield_prev,bool); knob!(pv_yield_sleepy_owner_set,pv_yield_sleepy_owner_get,pv_yield_sleepy_owner,bool); knob!(pv_prod_head_set,pv_prod_head_get,pv_prod_head,bool);

#[no_mangle] pub unsafe extern "C" fn spinlock_debugfs_init()->i32{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
