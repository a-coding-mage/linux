// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the Linux kernel and prime_numbers_private.h are
// intentionally referenced here rather than reimplemented.

use core::ffi::c_void;

type CBool = bool;

#[repr(C)]
pub struct primes {
    pub last: usize,
    pub sz: usize,
    pub primes: [usize; 1],
}

type PrimesFn = unsafe extern "C" fn(*mut c_void, *const primes);

extern "C" {
    fn int_sqrt(x: usize) -> usize;
    fn kmalloc(size: usize, flags: usize) -> *mut primes;
    fn kfree(ptr: *mut primes);
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn rcu_dereference(ptr: *const *const primes) -> *const primes;
    fn rcu_assign_pointer(ptr: *mut *const primes, value: *const primes);
    fn kfree_rcu(ptr: *mut primes, rcu: *mut c_void);
    fn bitmap_size(bits: usize) -> usize;
    fn bitmap_fill(bitmap: *mut usize, bits: usize);
    fn bitmap_copy(dst: *mut usize, src: *const usize, bits: usize);
    fn find_next_bit(bitmap: *const usize, size: usize, offset: usize) -> usize;
    fn __clear_bit(bit: usize, bitmap: *mut usize);
    fn test_bit(bit: usize, bitmap: *const usize) -> bool;
    fn lockdep_is_held(lock: *mut c_void) -> bool;
    fn with_primes(ctx: *mut c_void, fn_: PrimesFn);
}

#[cfg(target_pointer_width = "64")]
static SMALL_PRIMES: primes = primes {
    last: 61,
    sz: 64,
    primes: [
        (1usize << 2) | (1usize << 3) | (1usize << 5) | (1usize << 7) |
        (1usize << 11) | (1usize << 13) | (1usize << 17) | (1usize << 19) |
        (1usize << 23) | (1usize << 29) | (1usize << 31) | (1usize << 37) |
        (1usize << 41) | (1usize << 43) | (1usize << 47) | (1usize << 53) |
        (1usize << 59) | (1usize << 61),
    ],
};

#[cfg(target_pointer_width = "32")]
static SMALL_PRIMES: primes = primes {
    last: 31,
    sz: 32,
    primes: [
        (1usize << 2) | (1usize << 3) | (1usize << 5) | (1usize << 7) |
        (1usize << 11) | (1usize << 13) | (1usize << 17) | (1usize << 19) |
        (1usize << 23) | (1usize << 29) | (1usize << 31),
    ],
};

static mut LOCK: *mut c_void = core::ptr::null_mut();
static mut PRIMES: *const primes = &SMALL_PRIMES;

/* Calls the callback under RCU lock. The callback must not retain
 * the primes pointer.
 */
#[cfg(feature = "CONFIG_PRIME_NUMBERS_KUNIT_TEST")]
pub unsafe fn with_primes_export(ctx: *mut c_void, fn_: PrimesFn) {
    rcu_read_lock();
    fn_(ctx, rcu_dereference(&PRIMES));
    rcu_read_unlock();
}

pub unsafe fn slow_is_prime_number(mut x: usize) -> CBool {
    let mut y = int_sqrt(x);

    while y > 1 {
        if x % y == 0 {
            break;
        }
        y -= 1;
    }

    y == 1
}

unsafe fn slow_next_prime_number(mut x: usize) -> usize {
    while x < usize::MAX && {
        x = x.wrapping_add(1);
        !slow_is_prime_number(x)
    } {}

    x
}

unsafe fn clear_multiples(x: usize, p: *mut usize, start: usize, end: usize) -> usize {
    let mut m = 2usize.wrapping_mul(x);
    if m < start {
        m = ((start + x - 1) / x) * x;
    }

    while m < end {
        __clear_bit(m, p);
        m = m.wrapping_add(x);
    }

    x
}

unsafe fn expand_to_next_prime(x: usize) -> CBool {
    let mut sz = 2usize.wrapping_mul(x);
    if sz < x {
        return false;
    }

    sz = (sz + usize::BITS as usize - 1) & !(usize::BITS as usize - 1);
    let new = kmalloc(core::mem::size_of::<primes>() + bitmap_size(sz), 0);
    if new.is_null() {
        return false;
    }

    mutex_lock(LOCK);
    let p = rcu_dereference(&PRIMES);
    if x < (*p).last {
        kfree(new);
        mutex_unlock(LOCK);
        return true;
    }

    bitmap_fill((*new).primes.as_mut_ptr(), sz);
    bitmap_copy((*new).primes.as_mut_ptr(), (*p).primes.as_ptr(), (*p).sz);
    let mut y = 2usize;
    while y < sz {
        (*new).last = clear_multiples(y, (*new).primes.as_mut_ptr(), (*p).sz, sz);
        y = find_next_bit((*new).primes.as_ptr(), sz, y + 1);
    }
    (*new).sz = sz;

    rcu_assign_pointer(&mut PRIMES, new);
    if p != &SMALL_PRIMES {
        kfree_rcu(p as *mut primes, core::ptr::null_mut());
    }
    mutex_unlock(LOCK);
    true
}

unsafe fn free_primes() {
    mutex_lock(LOCK);
    let p = rcu_dereference(&PRIMES);
    if p != &SMALL_PRIMES {
        rcu_assign_pointer(&mut PRIMES, &SMALL_PRIMES);
        kfree_rcu(p as *mut primes, core::ptr::null_mut());
    }
    mutex_unlock(LOCK);
}

pub unsafe fn next_prime_number(mut x: usize) -> usize {
    rcu_read_lock();
    let mut p = rcu_dereference(&PRIMES);
    while x >= (*p).last {
        rcu_read_unlock();
        if !expand_to_next_prime(x) {
            return slow_next_prime_number(x);
        }
        rcu_read_lock();
        p = rcu_dereference(&PRIMES);
    }
    x = find_next_bit((*p).primes.as_ptr(), (*p).last, x + 1);
    rcu_read_unlock();
    x
}

pub unsafe fn is_prime_number(x: usize) -> CBool {
    let p;
    rcu_read_lock();
    p = rcu_dereference(&PRIMES);
    if x >= (*p).sz {
        rcu_read_unlock();
        if !expand_to_next_prime(x) {
            return slow_is_prime_number(x);
        }
        return is_prime_number(x);
    }
    let result = test_bit(x, (*p).primes.as_ptr());
    rcu_read_unlock();
    result
}

unsafe fn primes_exit() {
    free_primes();
}

// module_exit(primes_exit)
// MODULE_AUTHOR("Intel Corporation")
// MODULE_DESCRIPTION("Prime number library")
// MODULE_LICENSE("GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
