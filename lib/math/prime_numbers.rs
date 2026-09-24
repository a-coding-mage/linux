// SPDX-License-Identifier: GPL-2.0-only
//! Native prime-number cache, immutable RCU snapshots, and allocation fallback.
//!
//! This keeps the original sieve and its serialized copy/grow/publication
//! protocol. The public C ABI and the KUnit callback use actual kernel bindings.

use core::{
    alloc::Layout,
    mem,
    ptr::{self, NonNull},
    slice,
};
use kernel::{
    alloc::{
        allocator::Kmalloc,
        flags::{__GFP_NOWARN, GFP_KERNEL},
        Allocator, NumaNode,
    },
    bindings,
    ffi::c_ulong,
    sync::{
        atomic::{Acquire, Atomic, Release},
        rcu,
    },
};

#[path = "../../rust/ffi_export.rs"]
mod ffi_export;
#[path = "prime_numbers_mutex.rs"]
mod static_mutex;

const WORD_BITS: usize = c_ulong::BITS as usize;

// A genuine C header followed by one inline native-long bitmap word. Never
// create a reference to only the header and then derive a larger FAM pointer.
#[repr(C)]
struct SmallPrimes {
    header: bindings::primes,
    bitmap: [c_ulong; 1],
}

// SAFETY: Every byte read from SMALL_PRIMES is immutable for its entire
// lifetime. Its zero-initialized RCU head is never queued or modified.
unsafe impl Sync for SmallPrimes {}

static SMALL_PRIMES: SmallPrimes = {
    // SAFETY: The actual C header contains only a callback_head (nullable
    // pointer fields), unsigned integers and a zero-length FAM marker.
    let mut header: bindings::primes = unsafe { mem::zeroed() };
    header.last = if WORD_BITS == 64 { 61 } else { 31 };
    header.sz = WORD_BITS as c_ulong;
    let low = (1 << 2)
        | (1 << 3)
        | (1 << 5)
        | (1 << 7)
        | (1 << 11)
        | (1 << 13)
        | (1 << 17)
        | (1 << 19)
        | (1 << 23)
        | (1 << 29)
        | (1 << 31);
    let high = (1u64 << 37)
        | (1u64 << 41)
        | (1u64 << 43)
        | (1u64 << 47)
        | (1u64 << 53)
        | (1u64 << 59)
        | (1u64 << 61);
    SmallPrimes {
        header,
        bitmap: [low | high as c_ulong],
    }
};

const _: () = {
    assert!(WORD_BITS == 32 || WORD_BITS == 64);
    assert!(mem::offset_of!(SmallPrimes, bitmap) == mem::offset_of!(bindings::primes, primes));
    assert!(mem::size_of::<bindings::primes>() == mem::offset_of!(bindings::primes, primes));
    assert!(mem::offset_of!(bindings::primes, rcu) < 4096);
};

// SAFETY: LOCK is pinned in permanent static storage at this exact address.
// Its initializer is the separately proved original DEFINE_MUTEX initializer.
static LOCK: static_mutex::StaticMutex =
    unsafe { static_mutex::StaticMutex::new_at(ptr::addr_of!(LOCK)) };
static PRIMES: Atomic<*const bindings::primes> = Atomic::new(small_primes());

const fn small_primes() -> *const bindings::primes {
    ptr::addr_of!(SMALL_PRIMES).cast()
}

/// A read-side lifetime for a single immutable published cache.
struct Snapshot {
    pointer: *const bindings::primes,
    _guard: rcu::Guard,
}

impl Snapshot {
    fn new() -> Self {
        let guard = rcu::read_lock();
        // Paired with publication below. Acquire is stronger than the
        // original dependency-ordered rcu_dereference, not a weaker substitute.
        Self {
            pointer: PRIMES.load(Acquire),
            _guard: guard,
        }
    }

    fn last(&self) -> c_ulong {
        // SAFETY: The read-side guard keeps this initialized header alive.
        unsafe { ptr::addr_of!((*self.pointer).last).read() }
    }

    fn size(&self) -> usize {
        // SAFETY: As above. All published sizes are aligned native bit counts.
        unsafe { ptr::addr_of!((*self.pointer).sz).read() as usize }
    }

    fn bitmap(&self) -> &[c_ulong] {
        // SAFETY: The immutable inline bitmap has size()/WORD_BITS words,
        // remains live under this guard, and its slice cannot outlive self.
        unsafe { bitmap(self.pointer, self.size()) }
    }
}

/// Owns a not-yet-published allocation, including cancellation after recheck.
struct NewCache {
    pointer: NonNull<bindings::primes>,
    layout: Layout,
}

impl NewCache {
    fn allocate(bits: usize) -> Option<Self> {
        let bytes = (bits / WORD_BITS).checked_mul(mem::size_of::<c_ulong>())?;
        let size = mem::size_of::<bindings::primes>().checked_add(bytes)?;
        let layout = Layout::from_size_align(size, mem::align_of::<bindings::primes>()).ok()?;
        // This is deliberately before the mutex, exactly as in C. Kmalloc,
        // unlike KVmalloc, never silently broadens the original allocation.
        let buffer = Kmalloc::alloc(layout, GFP_KERNEL | __GFP_NOWARN, NumaNode::NO_NODE).ok()?;
        Some(Self {
            pointer: buffer.cast(),
            layout,
        })
    }
}

impl Drop for NewCache {
    fn drop(&mut self) {
        // SAFETY: This still-unpublished allocation is uniquely owned and
        // was obtained from Kmalloc using this layout.
        unsafe { Kmalloc::free(self.pointer.cast(), self.layout) };
    }
}

// SAFETY contract: p is a live C header plus size/WORD_BITS initialized words,
// immutable and protected either by the writer mutex or by a read-side guard.
unsafe fn bitmap<'a>(p: *const bindings::primes, size: usize) -> &'a [c_ulong] {
    // SAFETY: The caller provides the whole live allocation, not a narrow
    // reference to the flexible-array header.
    let words = unsafe { ptr::addr_of!((*p).primes).cast::<c_ulong>() };
    unsafe { slice::from_raw_parts(words, size / WORD_BITS) }
}

fn find_next_bit(words: &[c_ulong], limit: usize, start: usize) -> usize {
    if start >= limit {
        return limit;
    }
    let mut index = start / WORD_BITS;
    let mut word = words[index] & (c_ulong::MAX << (start % WORD_BITS));
    loop {
        if word != 0 {
            return (index * WORD_BITS + word.trailing_zeros() as usize).min(limit);
        }
        index += 1;
        if index * WORD_BITS >= limit {
            return limit;
        }
        word = words[index];
    }
}

fn clear_multiples(x: usize, words: &mut [c_ulong], start: usize, end: usize) {
    let mut multiple = x.wrapping_mul(2);
    if multiple < start {
        multiple = start.wrapping_add(x - 1) / x * x;
    }
    while multiple < end {
        words[multiple / WORD_BITS] &= !(1 << (multiple % WORD_BITS));
        multiple = multiple.wrapping_add(x);
    }
}

// SAFETY contract: p is an owned, formerly published Kmalloc allocation, no
// longer the current cache. Readers are still allowed to hold its bitmap.
unsafe fn retire(p: *const bindings::primes) {
    if p != small_primes() {
        let p = p.cast_mut();
        // Same header cast as kvfree_rcu_arg_2 in linux/rcupdate.h. The global
        // deferred-free machinery has no callback into this unloadable module.
        unsafe {
            bindings::kvfree_call_rcu(ptr::addr_of_mut!((*p).rcu).cast(), p.cast());
        }
    }
}

fn expand_to_next_prime(x: c_ulong) -> bool {
    // C first rejects multiplication overflow. Reject rounding/layout overflow
    // as well: the C wrap-to-zero path would copy into an undersized buffer.
    let Some(size) = (x as usize)
        .checked_mul(2)
        .and_then(|size| size.checked_add(WORD_BITS - 1))
        .map(|size| size & !(WORD_BITS - 1))
    else {
        return false;
    };
    let Some(new) = NewCache::allocate(size) else {
        return false;
    };
    let _guard = LOCK.lock();
    let previous = PRIMES.load(Acquire);
    // SAFETY: The writer mutex prevents replacement or retirement by another
    // writer. Published header fields and bitmaps are never mutated.
    let (previous_last, previous_size) = unsafe {
        (
            ptr::addr_of!((*previous).last).read(),
            ptr::addr_of!((*previous).sz).read() as usize,
        )
    };
    if x < previous_last {
        drop(new); // Match the original cancellation/free before unlocking.
        return true;
    }
    let pointer = new.pointer.as_ptr();
    // SAFETY: This allocation contains exactly this many native bitmap words;
    // it is uniquely owned and no reader can yet see it. Fill before copying.
    let words = unsafe {
        slice::from_raw_parts_mut(
            ptr::addr_of_mut!((*pointer).primes).cast::<c_ulong>(),
            size / WORD_BITS,
        )
    };
    words.fill(c_ulong::MAX);
    // SAFETY: The writer guard keeps the old, immutable bitmap live.
    words[..previous_size / WORD_BITS].copy_from_slice(unsafe { bitmap(previous, previous_size) });
    let mut y = 2usize;
    let mut last = 0;
    while y < size {
        clear_multiples(y, words, previous_size, size);
        last = y;
        y = find_next_bit(words, size, y + 1);
    }
    // SAFETY: Only this writer can access the unpublished header. The RCU
    // head is intentionally left for kvfree_call_rcu to initialize, as in C.
    unsafe {
        ptr::addr_of_mut!((*pointer).last).write(last as c_ulong);
        ptr::addr_of_mut!((*pointer).sz).write(size as c_ulong);
    }
    if last as c_ulong <= x {
        // SAFETY: This is the original impossible BUG_ON after the sieve.
        unsafe { bindings::BUG() };
    }
    PRIMES.store(pointer.cast_const(), Release);
    // Ownership transfers to the RCU cache.
    mem::forget(new);
    // SAFETY: The replaced allocation is no longer reachable by new readers.
    unsafe { retire(previous) };
    true
}

fn slow_is_prime(x: c_ulong) -> bool {
    // SAFETY: int_sqrt has no pointer or value preconditions.
    let mut y = unsafe { bindings::int_sqrt(x) };
    while y > 1 {
        if x % y == 0 {
            break;
        }
        y -= 1;
    }
    y == 1
}

fn slow_next_prime(mut x: c_ulong) -> c_ulong {
    while x < c_ulong::MAX {
        x += 1;
        if slow_is_prime(x) {
            break;
        }
    }
    x
}

/// Return the next prime greater than `x`, or native ULONG_MAX as a sentinel.
///
/// Cache growth may sleep. Exhaustion falls back to the original trial division.
#[no_mangle]
pub extern "C" fn next_prime_number(x: c_ulong) -> c_ulong {
    loop {
        let snapshot = Snapshot::new();
        if x < snapshot.last() {
            return find_next_bit(snapshot.bitmap(), snapshot.last() as usize, x as usize + 1)
                as c_ulong;
        }
        drop(snapshot); // Never allocate or take a sleeping lock under RCU.
        if !expand_to_next_prime(x) {
            return slow_next_prime(x);
        }
    }
}

/// Test primality using the original growing cache and allocation fallback.
///
/// Cache growth may sleep. Zero and one are not prime.
#[no_mangle]
pub extern "C" fn is_prime_number(x: c_ulong) -> bool {
    loop {
        let snapshot = Snapshot::new();
        if x < snapshot.size() as c_ulong {
            return snapshot.bitmap()[x as usize / WORD_BITS] & (1 << (x as usize % WORD_BITS))
                != 0;
        }
        drop(snapshot);
        if !expand_to_next_prime(x) {
            return slow_is_prime(x);
        }
    }
}

/// Invoke the original KUnit callback under the RCU read-side lock.
///
/// # Safety
/// The callback must be non-null, accept `ctx`, neither retain the cache pointer
/// nor mutate it, and obey RCU read-side restrictions (in particular, no sleep).
#[cfg(CONFIG_PRIME_NUMBERS_KUNIT_TEST)]
#[no_mangle]
pub unsafe extern "C" fn with_primes(ctx: *mut kernel::ffi::c_void, r#fn: bindings::primes_fn) {
    let snapshot = Snapshot::new();
    // SAFETY: The caller supplies a valid callback, and the snapshot guard
    // protects the complete original binding/FAM throughout its invocation.
    unsafe { r#fn(ctx, snapshot.pointer) };
}

/// Original trial-division reference, exported only for y/m KUnit tests.
#[cfg(CONFIG_PRIME_NUMBERS_KUNIT_TEST)]
#[no_mangle]
pub extern "C" fn slow_is_prime_number(x: c_ulong) -> bool {
    slow_is_prime(x)
}

#[cfg(CONFIG_PRIME_NUMBERS_KUNIT_TEST)]
ffi_export::export_symbol!(with_primes, with_primes, "", "");
#[cfg(CONFIG_PRIME_NUMBERS_KUNIT_TEST)]
ffi_export::export_symbol!(slow_is_prime_number, slow_is_prime_number, "", "");
ffi_export::export_symbol!(next_prime_number, next_prime_number, "", "");
ffi_export::export_symbol!(is_prime_number, is_prime_number, "", "");

fn free_primes() {
    let _guard = LOCK.lock();
    let previous = PRIMES.load(Acquire);
    if previous != small_primes() {
        PRIMES.store(small_primes(), Release);
        // SAFETY: This exclusively removes the cache before deferred freeing.
        unsafe { retire(previous) };
    }
}

#[cfg(not(MODULE))]
#[link_section = ".exit.text"]
extern "C" fn primes_exit() {
    free_primes();
}

#[cfg(not(MODULE))]
#[used]
#[link_section = ".exitcall.exit"]
static EXITCALL: extern "C" fn() = primes_exit;

/// Original exit-only module entry point; there is deliberately no initializer.
#[cfg(MODULE)]
#[no_mangle]
#[link_section = ".exit.text"]
pub extern "C" fn cleanup_module() {
    free_primes();
}

#[cfg(MODULE)]
#[used]
#[link_section = ".exit.data"]
static CLEANUP_ADDRESSABLE: extern "C" fn() = cleanup_module;

#[cfg(MODULE)]
const MODINFO: &str = "author=Intel Corporation\0description=Prime number library\0license=GPL\0";
#[cfg(not(MODULE))]
const MODINFO: &str = concat!(
    "prime_numbers.author=Intel Corporation\0prime_numbers.description=Prime number library\0",
    "prime_numbers.license=GPL\0prime_numbers.file=",
    env!("RUST_MODFILE"),
    "\0",
);
#[used]
#[link_section = ".modinfo"]
static MODULE_INFO: [u8; MODINFO.len()] = {
    let mut bytes = [0; MODINFO.len()];
    let mut index = 0;
    while index < bytes.len() {
        bytes[index] = MODINFO.as_bytes()[index];
        index += 1;
    }
    bytes
};
#[cfg(MODULE)]
#[used]
static __IS_RUST_MODULE: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
