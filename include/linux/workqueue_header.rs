/* SPDX-License-Identifier: GPL-2.0 */
/* workqueue.h --- work queue handling for Linux. */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub enum work_bits {
    WORK_STRUCT_PENDING_BIT = 0,
    WORK_STRUCT_INACTIVE_BIT,
    WORK_STRUCT_PWQ_BIT,
    WORK_STRUCT_LINKED_BIT,
    #[cfg(CONFIG_DEBUG_OBJECTS_WORK)]
    WORK_STRUCT_STATIC_BIT,
    WORK_STRUCT_FLAG_BITS,
    WORK_STRUCT_COLOR_SHIFT = WORK_STRUCT_FLAG_BITS as isize,
    WORK_STRUCT_COLOR_BITS = 4,
    WORK_STRUCT_PWQ_SHIFT = WORK_STRUCT_COLOR_SHIFT as isize + WORK_STRUCT_COLOR_BITS as isize,
    WORK_OFFQ_FLAG_SHIFT = WORK_STRUCT_FLAG_BITS as isize,
    WORK_OFFQ_BH_BIT = WORK_OFFQ_FLAG_SHIFT as isize,
    WORK_OFFQ_FLAG_END,
    WORK_OFFQ_FLAG_BITS = WORK_OFFQ_FLAG_END as isize - WORK_OFFQ_FLAG_SHIFT as isize,
    WORK_OFFQ_DISABLE_SHIFT = WORK_OFFQ_FLAG_SHIFT as isize + WORK_OFFQ_FLAG_BITS as isize,
    WORK_OFFQ_DISABLE_BITS = 16,
    WORK_OFFQ_POOL_SHIFT = WORK_OFFQ_DISABLE_SHIFT as isize + WORK_OFFQ_DISABLE_BITS as isize,
    WORK_OFFQ_LEFT = BITS_PER_LONG as isize - WORK_OFFQ_POOL_SHIFT as isize,
    WORK_OFFQ_POOL_BITS = if WORK_OFFQ_LEFT <= 31 { WORK_OFFQ_LEFT } else { 31 },
}

pub const WORK_STRUCT_PENDING: usize = 1 << (work_bits::WORK_STRUCT_PENDING_BIT as usize);
pub const WORK_STRUCT_INACTIVE: usize = 1 << (work_bits::WORK_STRUCT_INACTIVE_BIT as usize);
pub const WORK_STRUCT_PWQ: usize = 1 << (work_bits::WORK_STRUCT_PWQ_BIT as usize);
pub const WORK_STRUCT_LINKED: usize = 1 << (work_bits::WORK_STRUCT_LINKED_BIT as usize);
#[cfg(CONFIG_DEBUG_OBJECTS_WORK)]
pub const WORK_STRUCT_STATIC: usize = 1 << (work_bits::WORK_STRUCT_STATIC_BIT as usize);
#[cfg(not(CONFIG_DEBUG_OBJECTS_WORK))]
pub const WORK_STRUCT_STATIC: usize = 0;

pub const WORK_NR_COLORS: usize = 1 << (work_bits::WORK_STRUCT_COLOR_BITS as usize);
pub const WORK_CPU_UNBOUND: u32 = NR_CPUS;
pub const WORK_BUSY_PENDING: u32 = 1 << 0;
pub const WORK_BUSY_RUNNING: u32 = 1 << 1;
pub const WORKER_DESC_LEN: usize = 32;
pub const WORK_OFFQ_BH: usize = 1usize << (work_bits::WORK_OFFQ_BH_BIT as usize);
pub const WORK_OFFQ_FLAG_MASK: usize = ((1usize << (work_bits::WORK_OFFQ_FLAG_BITS as usize)) - 1) << (work_bits::WORK_OFFQ_FLAG_SHIFT as usize);
pub const WORK_OFFQ_DISABLE_MASK: usize = ((1usize << 16) - 1) << (work_bits::WORK_OFFQ_DISABLE_SHIFT as usize);
pub const WORK_OFFQ_POOL_NONE: usize = (1usize << (work_bits::WORK_OFFQ_POOL_BITS as usize)) - 1;
pub const WORK_STRUCT_NO_POOL: usize = WORK_OFFQ_POOL_NONE << (work_bits::WORK_OFFQ_POOL_SHIFT as usize);
pub const WORK_STRUCT_PWQ_MASK: usize = !((1usize << (work_bits::WORK_STRUCT_PWQ_SHIFT as usize)) - 1);

#[repr(C)]
pub struct delayed_work { pub work: work_struct, pub timer: timer_list, pub wq: *mut workqueue_struct, pub cpu: i32 }
#[repr(C)]
pub struct rcu_work { pub work: work_struct, pub rcu: rcu_head, pub wq: *mut workqueue_struct }

#[repr(C)]
pub enum wq_affn_scope { WQ_AFFN_DFL, WQ_AFFN_CPU, WQ_AFFN_SMT, WQ_AFFN_CACHE, WQ_AFFN_CACHE_SHARD, WQ_AFFN_NUMA, WQ_AFFN_SYSTEM, WQ_AFFN_NR_TYPES }
#[repr(C)]
pub struct workqueue_attrs {
    pub nice: i32,
    pub cpumask: cpumask_var_t,
    pub __pod_cpumask: cpumask_var_t,
    pub affn_strict: bool,
    pub affn_scope: wq_affn_scope,
    pub ordered: bool,
}
#[repr(C)] pub struct execute_work { pub work: work_struct }

#[repr(C)]
pub enum wq_flags {
    WQ_BH=1<<0, WQ_UNBOUND=1<<1, WQ_FREEZABLE=1<<2, WQ_MEM_RECLAIM=1<<3,
    WQ_HIGHPRI=1<<4, WQ_CPU_INTENSIVE=1<<5, WQ_SYSFS=1<<6, WQ_POWER_EFFICIENT=1<<7,
    WQ_PERCPU=1<<8, __WQ_DESTROYING=1<<15, __WQ_DRAINING=1<<16, __WQ_ORDERED=1<<17,
    __WQ_LEGACY=1<<18, __WQ_DEPRECATED=1<<19, __WQ_BH_ALLOWS=(1<<0)|(1<<4)|(1<<8),
}
pub const WQ_MAX_ACTIVE: i32=2048; pub const WQ_UNBOUND_MAX_ACTIVE:i32=WQ_MAX_ACTIVE;
pub const WQ_DFL_ACTIVE:i32=WQ_MAX_ACTIVE/2; pub const WQ_DFL_MIN_ACTIVE:i32=8;

extern "C" {
    pub static mut system_wq: *mut workqueue_struct; pub static mut system_percpu_wq: *mut workqueue_struct;
    pub static mut system_highpri_wq: *mut workqueue_struct; pub static mut system_long_wq: *mut workqueue_struct;
    pub static mut system_unbound_wq: *mut workqueue_struct; pub static mut system_dfl_wq: *mut workqueue_struct;
    pub static mut system_freezable_wq: *mut workqueue_struct; pub static mut system_power_efficient_wq: *mut workqueue_struct;
    pub static mut system_freezable_power_efficient_wq: *mut workqueue_struct; pub static mut system_bh_wq: *mut workqueue_struct;
    pub static mut system_bh_highpri_wq: *mut workqueue_struct; pub static mut system_dfl_long_wq: *mut workqueue_struct;
    pub fn workqueue_softirq_action(highpri: bool); pub fn workqueue_softirq_dead(cpu: u32);
    pub fn alloc_workqueue_noprof(fmt:*const c_char, flags:u32, max_active:i32, ...) -> *mut workqueue_struct;
    pub fn devm_alloc_workqueue_noprof(dev:*mut device, fmt:*const c_char, flags:u32, max_active:i32, ...) -> *mut workqueue_struct;
    pub fn destroy_workqueue(wq:*mut workqueue_struct); pub fn alloc_workqueue_attrs_noprof() -> *mut workqueue_attrs;
    pub fn free_workqueue_attrs(attrs:*mut workqueue_attrs); pub fn apply_workqueue_attrs(wq:*mut workqueue_struct, attrs:*const workqueue_attrs)->i32;
    pub fn queue_work_on(cpu:i32,wq:*mut workqueue_struct,work:*mut work_struct)->bool;
    pub fn queue_work_node(node:i32,wq:*mut workqueue_struct,work:*mut work_struct)->bool;
    pub fn queue_delayed_work_on(cpu:i32,wq:*mut workqueue_struct,work:*mut delayed_work,delay:usize)->bool;
    pub fn mod_delayed_work_on(cpu:i32,wq:*mut workqueue_struct,work:*mut delayed_work,delay:usize)->bool;
    pub fn queue_rcu_work(wq:*mut workqueue_struct,work:*mut rcu_work)->bool;
    pub fn __flush_workqueue(wq:*mut workqueue_struct); pub fn drain_workqueue(wq:*mut workqueue_struct);
    pub fn flush_work(work:*mut work_struct)->bool; pub fn cancel_work(work:*mut work_struct)->bool; pub fn cancel_work_sync(work:*mut work_struct)->bool;
    pub fn flush_delayed_work(w:*mut delayed_work)->bool; pub fn cancel_delayed_work(w:*mut delayed_work)->bool; pub fn cancel_delayed_work_sync(w:*mut delayed_work)->bool;
    pub fn disable_work(w:*mut work_struct)->bool; pub fn disable_work_sync(w:*mut work_struct)->bool; pub fn enable_work(w:*mut work_struct)->bool;
    pub fn disable_delayed_work(w:*mut delayed_work)->bool; pub fn disable_delayed_work_sync(w:*mut delayed_work)->bool; pub fn enable_delayed_work(w:*mut delayed_work)->bool;
    pub fn flush_rcu_work(w:*mut rcu_work)->bool; pub fn work_busy(w:*mut work_struct)->u32; pub fn current_work()->*mut work_struct;
    pub fn current_is_workqueue_rescuer()->bool; pub fn current_is_workqueue_mem_reclaim()->bool;
    pub fn schedule_on_each_cpu(func: work_func_t)->i32; pub fn execute_in_process_context(fn_:work_func_t, ew:*mut execute_work)->i32;
    pub fn workqueue_set_max_active(wq:*mut workqueue_struct, max:i32); pub fn workqueue_set_min_active(wq:*mut workqueue_struct, min:i32);
    pub fn workqueue_congested(cpu:i32,wq:*mut workqueue_struct)->bool;
    pub fn set_worker_desc(fmt:*const c_char,...); pub fn print_worker_info(log_lvl:*const c_char,task:*mut task_struct);
    pub fn show_all_workqueues(); pub fn show_freezable_workqueues(); pub fn show_one_workqueue(wq:*mut workqueue_struct);
    pub fn wq_worker_comm(buf:*mut c_char,size:usize,task:*mut task_struct);
    pub fn __warn_flushing_systemwide_wq();
    #[cfg(CONFIG_FREEZER)] pub fn freeze_workqueues_begin();
    #[cfg(CONFIG_FREEZER)] pub fn freeze_workqueues_busy()->bool;
    #[cfg(CONFIG_FREEZER)] pub fn thaw_workqueues();
    #[cfg(CONFIG_SYSFS)] pub fn workqueue_sysfs_register(wq:*mut workqueue_struct)->i32;
    #[cfg(CONFIG_WQ_WATCHDOG)] pub fn wq_watchdog_touch(cpu:i32);
    #[cfg(CONFIG_SMP)] pub fn workqueue_prepare_cpu(cpu:u32)->i32;
    #[cfg(CONFIG_SMP)] pub fn workqueue_online_cpu(cpu:u32)->i32;
    #[cfg(CONFIG_SMP)] pub fn workqueue_offline_cpu(cpu:u32)->i32;
}

#[inline] pub unsafe fn queue_work(wq:*mut workqueue_struct, work:*mut work_struct)->bool { queue_work_on(WORK_CPU_UNBOUND as i32,wq,work) }
#[inline] pub unsafe fn queue_delayed_work(wq:*mut workqueue_struct,w:*mut delayed_work,d:usize)->bool { queue_delayed_work_on(WORK_CPU_UNBOUND as i32,wq,w,d) }
#[inline] pub unsafe fn mod_delayed_work(wq:*mut workqueue_struct,w:*mut delayed_work,d:usize)->bool { mod_delayed_work_on(WORK_CPU_UNBOUND as i32,wq,w,d) }
#[inline] pub unsafe fn schedule_work_on(cpu:i32,w:*mut work_struct)->bool { queue_work_on(cpu,system_percpu_wq,w) }
#[inline] pub unsafe fn schedule_work(w:*mut work_struct)->bool { queue_work(system_percpu_wq,w) }
#[inline] pub unsafe fn schedule_delayed_work_on(cpu:i32,w:*mut delayed_work,d:usize)->bool { queue_delayed_work_on(cpu,system_percpu_wq,w,d) }
#[inline] pub unsafe fn schedule_delayed_work(w:*mut delayed_work,d:usize)->bool { queue_delayed_work(system_percpu_wq,w,d) }

extern "C" { pub fn workqueue_unbound_housekeeping_update(hk:*const cpumask)->bool; pub fn workqueue_init_early(); pub fn workqueue_init(); pub fn workqueue_init_topology(); }

#[inline] pub unsafe fn enable_and_queue_work(wq:*mut workqueue_struct,w:*mut work_struct)->bool {
    if enable_work(w) { queue_work(wq,w); true } else { false }
}

#[cfg(not(CONFIG_SYSFS))]
#[inline] pub unsafe fn workqueue_sysfs_register(_wq:*mut workqueue_struct)->i32 { 0 }
#[cfg(not(CONFIG_WQ_WATCHDOG))]
#[inline] pub unsafe fn wq_watchdog_touch(_cpu:i32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
