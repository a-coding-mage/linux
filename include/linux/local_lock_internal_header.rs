/* SPDX-License-Identifier: GPL-2.0 */
// Do not include directly; include linux/local_lock.h.
// External Linux declarations and configuration symbols are supplied by other files.

#[cfg(not(CONFIG_PREEMPT_RT))]
#[repr(C)]
pub struct local_lock {
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
    pub dep_map: lockdep_map,
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
    pub owner: *mut task_struct,
}
#[cfg(not(CONFIG_PREEMPT_RT))]
pub type local_lock_t = local_lock;

#[cfg(not(CONFIG_PREEMPT_RT))]
#[repr(C)]
pub struct local_trylock {
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
    pub dep_map: lockdep_map,
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
    pub owner: *mut task_struct,
    pub acquired: u8,
}
#[cfg(not(CONFIG_PREEMPT_RT))]
pub type local_trylock_t = local_trylock;

#[cfg(all(not(CONFIG_PREEMPT_RT), CONFIG_DEBUG_LOCK_ALLOC))]
pub unsafe fn local_lock_acquire(l: *mut local_lock_t) {
    lock_map_acquire(&mut (*l).dep_map);
    DEBUG_LOCKS_WARN_ON(!(*l).owner.is_null());
    (*l).owner = current;
}
#[cfg(all(not(CONFIG_PREEMPT_RT), CONFIG_DEBUG_LOCK_ALLOC))]
pub unsafe fn local_trylock_acquire(l: *mut local_lock_t) {
    lock_map_acquire_try(&mut (*l).dep_map);
    DEBUG_LOCKS_WARN_ON(!(*l).owner.is_null());
    (*l).owner = current;
}
#[cfg(all(not(CONFIG_PREEMPT_RT), CONFIG_DEBUG_LOCK_ALLOC))]
pub unsafe fn local_lock_release(l: *mut local_lock_t) {
    DEBUG_LOCKS_WARN_ON((*l).owner != current);
    (*l).owner = core::ptr::null_mut();
    lock_map_release(&mut (*l).dep_map);
}
#[cfg(all(not(CONFIG_PREEMPT_RT), CONFIG_DEBUG_LOCK_ALLOC))]
pub unsafe fn local_lock_debug_init(l: *mut local_lock_t) {
    (*l).owner = core::ptr::null_mut();
}

#[cfg(all(not(CONFIG_PREEMPT_RT), not(CONFIG_DEBUG_LOCK_ALLOC)))]
pub unsafe fn local_lock_acquire(_l: *mut local_lock_t) {}
#[cfg(all(not(CONFIG_PREEMPT_RT), not(CONFIG_DEBUG_LOCK_ALLOC)))]
pub unsafe fn local_trylock_acquire(_l: *mut local_lock_t) {}
#[cfg(all(not(CONFIG_PREEMPT_RT), not(CONFIG_DEBUG_LOCK_ALLOC)))]
pub unsafe fn local_lock_release(_l: *mut local_lock_t) {}
#[cfg(all(not(CONFIG_PREEMPT_RT), not(CONFIG_DEBUG_LOCK_ALLOC)))]
pub unsafe fn local_lock_debug_init(_l: *mut local_lock_t) {}

#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! INIT_LOCAL_LOCK { ($lockname:ident) => { { /* LOCAL_LOCK_DEBUG_INIT($lockname) */ } }; }
#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! INIT_LOCAL_TRYLOCK { ($lockname:ident) => { INIT_LOCAL_LOCK!($lockname) }; }

/* The following macros preserve the C operations and sequencing. */
#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! __local_lock_acquire { ($lock:expr) => {{
    let __l = $lock as *mut local_lock_t;
    unsafe { local_lock_acquire(__l); }
}}; }
#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! __local_lock { ($lock:expr) => {{ preempt_disable(); __local_lock_acquire!($lock); __acquire!($lock); }}; }
#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! __local_lock_irq { ($lock:expr) => {{ local_irq_disable(); __local_lock_acquire!($lock); __acquire!($lock); }}; }
#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! __local_lock_irqsave { ($lock:expr, $flags:expr) => {{ local_irq_save!($flags); __local_lock_acquire!($lock); __acquire!($lock); }}; }
#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! __local_lock_release { ($lock:expr) => {{ unsafe { local_lock_release($lock as *mut local_lock_t); } }}; }
#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! __local_unlock { ($lock:expr) => {{ __release!($lock); __local_lock_release!($lock); preempt_enable(); }}; }
#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! __local_unlock_irq { ($lock:expr) => {{ __release!($lock); __local_lock_release!($lock); local_irq_enable(); }}; }
#[cfg(not(CONFIG_PREEMPT_RT))]
#[macro_export]
macro_rules! __local_unlock_irqrestore { ($lock:expr, $flags:expr) => {{ __release!($lock); __local_lock_release!($lock); local_irq_restore!($flags); }}; }

#[cfg(CONFIG_PREEMPT_RT)]
pub type local_lock_t = spinlock_t;
#[cfg(CONFIG_PREEMPT_RT)]
pub type local_trylock_t = spinlock_t;

#[cfg(CONFIG_PREEMPT_RT)]
#[macro_export]
macro_rules! INIT_LOCAL_LOCK { ($lockname:ident) => { __LOCAL_SPIN_LOCK_UNLOCKED!($lockname) }; }
#[cfg(CONFIG_PREEMPT_RT)]
#[macro_export]
macro_rules! INIT_LOCAL_TRYLOCK { ($lockname:ident) => { __LOCAL_SPIN_LOCK_UNLOCKED!($lockname) }; }
#[cfg(CONFIG_PREEMPT_RT)]
#[macro_export]
macro_rules! __local_lock_init { ($l:expr) => {{ local_spin_lock_init!($l); }}; }
#[cfg(CONFIG_PREEMPT_RT)]
#[macro_export]
macro_rules! __local_trylock_init { ($l:expr) => { __local_lock_init!($l) }; }
#[cfg(CONFIG_PREEMPT_RT)]
#[macro_export]
macro_rules! __local_lock { ($lock:expr) => {{ migrate_disable(); spin_lock!($lock); }}; }
#[cfg(CONFIG_PREEMPT_RT)]
#[macro_export]
macro_rules! __local_lock_irq { ($lock:expr) => { __local_lock!($lock) }; }
#[cfg(CONFIG_PREEMPT_RT)]
#[macro_export]
macro_rules! __local_unlock { ($lock:expr) => {{ spin_unlock!($lock); migrate_enable(); }}; }
#[cfg(CONFIG_PREEMPT_RT)]
#[macro_export]
macro_rules! __local_unlock_irq { ($lock:expr) => { __local_unlock!($lock) }; }

// On PREEMPT_RT local_lock maps to a per-CPU spinlock, remaining preemptible.
// migration must be disabled before calling __local_lock_is_locked.

#[cfg(WARN_CONTEXT_ANALYSIS)]
pub unsafe fn __this_cpu_local_lock<T>(base: *mut T) -> *mut T { this_cpu_ptr(base) }
#[cfg(not(WARN_CONTEXT_ANALYSIS))]
#[macro_export]
macro_rules! __this_cpu_local_lock { ($base:expr) => { this_cpu_ptr!($base) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
