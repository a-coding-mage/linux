// Translation of linux/spinlock_api_up.h.
// The original include guard and direct-inclusion diagnostic are C-only.

// Build-time condition preserved from the source: this header is intended for
// inclusion from the spinlock API implementation.

#[inline(always)]
pub const fn in_lock_functions<T>(_addr: T) -> i32 {
    0
}

macro_rules! assert_raw_spin_locked {
    ($lock:expr) => {{
        let _ = &$lock;
    }};
}

macro_rules! ___LOCK_ {
    ($lock:expr) => {{
        __acquire($lock);
        let _ = &$lock;
    }};
}

macro_rules! ___LOCK_shared {
    ($lock:expr) => {{
        __acquire_shared($lock);
        let _ = &$lock;
    }};
}

macro_rules! __LOCK {
    ($lock:expr) => {{ preempt_disable(); ___LOCK_!($lock); }};
    ($lock:expr, shared) => {{ preempt_disable(); ___LOCK_shared!($lock); }};
}

macro_rules! __LOCK_BH {
    ($lock:expr) => {{ __local_bh_disable_ip(_THIS_IP_(), SOFTIRQ_LOCK_OFFSET); ___LOCK_!($lock); }};
    ($lock:expr, shared) => {{ __local_bh_disable_ip(_THIS_IP_(), SOFTIRQ_LOCK_OFFSET); ___LOCK_shared!($lock); }};
}

macro_rules! __LOCK_IRQ {
    ($lock:expr) => {{ local_irq_disable(); __LOCK!($lock); }};
    ($lock:expr, shared) => {{ local_irq_disable(); __LOCK!($lock, shared); }};
}

macro_rules! __LOCK_IRQSAVE {
    ($lock:expr, $flags:expr) => {{ local_irq_save($flags); __LOCK!($lock); }};
    ($lock:expr, $flags:expr, shared) => {{ local_irq_save($flags); __LOCK!($lock, shared); }};
}

macro_rules! __LOCK_IRQ_DISABLE {
    ($lock:expr) => {{ local_interrupt_disable(); __LOCK!($lock); }};
    ($lock:expr, shared) => {{ local_interrupt_disable(); __LOCK!($lock, shared); }};
}

macro_rules! ___UNLOCK_ {
    ($lock:expr) => {{ __release($lock); let _ = &$lock; }};
}

macro_rules! ___UNLOCK_shared {
    ($lock:expr) => {{ __release_shared($lock); let _ = &$lock; }};
}

macro_rules! __UNLOCK {
    ($lock:expr) => {{ preempt_enable(); ___UNLOCK_!($lock); }};
    ($lock:expr, shared) => {{ preempt_enable(); ___UNLOCK_shared!($lock); }};
}

macro_rules! __UNLOCK_BH {
    ($lock:expr) => {{ __local_bh_enable_ip(_THIS_IP_(), SOFTIRQ_LOCK_OFFSET); ___UNLOCK_!($lock); }};
    ($lock:expr, shared) => {{ __local_bh_enable_ip(_THIS_IP_(), SOFTIRQ_LOCK_OFFSET); ___UNLOCK_shared!($lock); }};
}

macro_rules! __UNLOCK_IRQ {
    ($lock:expr) => {{ local_irq_enable(); __UNLOCK!($lock); }};
    ($lock:expr, shared) => {{ local_irq_enable(); __UNLOCK!($lock, shared); }};
}

macro_rules! __UNLOCK_IRQRESTORE {
    ($lock:expr, $flags:expr) => {{ local_irq_restore($flags); __UNLOCK!($lock); }};
    ($lock:expr, $flags:expr, shared) => {{ local_irq_restore($flags); __UNLOCK!($lock, shared); }};
}

macro_rules! __UNLOCK_IRQ_ENABLE {
    ($lock:expr) => {{ __UNLOCK!($lock); local_interrupt_enable(); }};
    ($lock:expr, shared) => {{ __UNLOCK!($lock, shared); local_interrupt_enable(); }};
}

macro_rules! _raw_spin_lock { ($lock:expr) => { __LOCK!($lock) }; }
macro_rules! _raw_spin_lock_nested { ($lock:expr, $subclass:expr) => { __LOCK!($lock) }; }
macro_rules! _raw_read_lock { ($lock:expr) => { __LOCK!($lock, shared) }; }
macro_rules! _raw_write_lock { ($lock:expr) => { __LOCK!($lock) }; }
macro_rules! _raw_write_lock_nested { ($lock:expr, $subclass:expr) => { __LOCK!($lock) }; }
macro_rules! _raw_spin_lock_bh { ($lock:expr) => { __LOCK_BH!($lock) }; }
macro_rules! _raw_read_lock_bh { ($lock:expr) => { __LOCK_BH!($lock, shared) }; }
macro_rules! _raw_write_lock_bh { ($lock:expr) => { __LOCK_BH!($lock) }; }
macro_rules! _raw_spin_lock_irq { ($lock:expr) => { __LOCK_IRQ!($lock) }; }
macro_rules! _raw_spin_lock_irq_disable { ($lock:expr) => { __LOCK_IRQ_DISABLE!($lock) }; }
macro_rules! _raw_read_lock_irq { ($lock:expr) => { __LOCK_IRQ!($lock, shared) }; }
macro_rules! _raw_write_lock_irq { ($lock:expr) => { __LOCK_IRQ!($lock) }; }
macro_rules! _raw_spin_lock_irqsave { ($lock:expr, $flags:expr) => { __LOCK_IRQSAVE!($lock, $flags) }; }
macro_rules! _raw_read_lock_irqsave { ($lock:expr, $flags:expr) => { __LOCK_IRQSAVE!($lock, $flags, shared) }; }
macro_rules! _raw_write_lock_irqsave { ($lock:expr, $flags:expr) => { __LOCK_IRQSAVE!($lock, $flags) }; }

#[inline(always)]
pub unsafe fn _raw_spin_trylock(lock: *mut raw_spinlock_t) -> i32 { __LOCK!(lock); 1 }
#[inline(always)]
pub unsafe fn _raw_spin_trylock_bh(lock: *mut raw_spinlock_t) -> i32 { __LOCK_BH!(lock); 1 }
#[inline(always)]
pub unsafe fn _raw_spin_trylock_irq(lock: *mut raw_spinlock_t) -> i32 { __LOCK_IRQ!(lock); 1 }
#[inline(always)]
pub unsafe fn _raw_spin_trylock_irq_disable(lock: *mut raw_spinlock_t) -> i32 { __LOCK_IRQ_DISABLE!(lock); 1 }
#[inline(always)]
pub unsafe fn _raw_spin_trylock_irqsave(lock: *mut raw_spinlock_t, flags: *mut ::core::ffi::c_ulong) -> i32 { __LOCK_IRQSAVE!(lock, *flags); 1 }
#[inline(always)]
pub unsafe fn _raw_read_trylock(lock: *mut rwlock_t) -> i32 { __LOCK!(lock, shared); 1 }
#[inline(always)]
pub unsafe fn _raw_write_trylock(lock: *mut rwlock_t) -> i32 { __LOCK!(lock); 1 }
#[inline(always)]
pub unsafe fn _raw_write_trylock_irqsave(lock: *mut rwlock_t, flags: *mut ::core::ffi::c_ulong) -> i32 { __LOCK_IRQSAVE!(lock, *flags); 1 }

macro_rules! _raw_spin_unlock { ($lock:expr) => { __UNLOCK!($lock) }; }
macro_rules! _raw_read_unlock { ($lock:expr) => { __UNLOCK!($lock, shared) }; }
macro_rules! _raw_write_unlock { ($lock:expr) => { __UNLOCK!($lock) }; }
macro_rules! _raw_spin_unlock_bh { ($lock:expr) => { __UNLOCK_BH!($lock) }; }
macro_rules! _raw_write_unlock_bh { ($lock:expr) => { __UNLOCK_BH!($lock) }; }
macro_rules! _raw_read_unlock_bh { ($lock:expr) => { __UNLOCK_BH!($lock, shared) }; }
macro_rules! _raw_spin_unlock_irq { ($lock:expr) => { __UNLOCK_IRQ!($lock) }; }
macro_rules! _raw_spin_unlock_irq_enable { ($lock:expr) => { __UNLOCK_IRQ_ENABLE!($lock) }; }
macro_rules! _raw_read_unlock_irq { ($lock:expr) => { __UNLOCK_IRQ!($lock, shared) }; }
macro_rules! _raw_write_unlock_irq { ($lock:expr) => { __UNLOCK_IRQ!($lock) }; }
macro_rules! _raw_spin_unlock_irqrestore { ($lock:expr, $flags:expr) => { __UNLOCK_IRQRESTORE!($lock, $flags) }; }
macro_rules! _raw_read_unlock_irqrestore { ($lock:expr, $flags:expr) => { __UNLOCK_IRQRESTORE!($lock, $flags, shared) }; }
macro_rules! _raw_write_unlock_irqrestore { ($lock:expr, $flags:expr) => { __UNLOCK_IRQRESTORE!($lock, $flags) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
