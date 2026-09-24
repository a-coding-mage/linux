// SPDX-License-Identifier: GPL-2.0-only
//! Original prime-number KUnit checks against the selected native cache provider.

use kernel::{bindings, ffi};

#[allow(unreachable_pub)]
#[path = "../prime_numbers_private_header.rs"]
mod private;

#[cfg(CONFIG_PRINTK_INDEX)]
#[repr(transparent)]
struct PrintkIndexEntry(bindings::pi_entry);

// SAFETY: The immutable record only points to static C strings or NULL.
#[cfg(CONFIG_PRINTK_INDEX)]
unsafe impl Sync for PrintkIndexEntry {}

#[cfg(CONFIG_PRINTK_INDEX)]
#[repr(transparent)]
struct PrintkIndexPointer(*const bindings::pi_entry);

// SAFETY: The pointer refers to an immutable static PrintkIndexEntry.
#[cfg(CONFIG_PRINTK_INDEX)]
unsafe impl Sync for PrintkIndexPointer {}

macro_rules! suite_printk {
    ($format:expr, $($argument:expr),* $(,)?) => {{
        // The C printk_index_wrap emits its record even when CONFIG_PRINTK=n
        // makes _printk a no-op. The complete format contains the log level.
        #[cfg(CONFIG_PRINTK_INDEX)]
        {
            #[used]
            static ENTRY: PrintkIndexEntry = PrintkIndexEntry(bindings::pi_entry {
                fmt: $format.as_ptr().cast(),
                func: c"dump_primes".as_ptr().cast(),
                file: concat!(file!(), "\0").as_ptr().cast(),
                line: line!(),
                level: core::ptr::null(),
                subsys_fmt_prefix: core::ptr::null(),
            });
            #[used]
            #[link_section = ".printk_index"]
            static POINTER: PrintkIndexPointer = PrintkIndexPointer(&ENTRY.0);
        }
        // The invocation supplies the unsafe context and original printf
        // argument types. The native binding preserves unsigned C char.
        #[cfg(CONFIG_PRINTK)]
        bindings::_printk($format.as_ptr().cast(), $($argument),*);
    }};
}

unsafe extern "C" fn dump_primes(ctx: *mut ffi::c_void, primes: *const private::primes) {
    let suite = ctx.cast::<bindings::kunit_suite>();
    // SAFETY: with_primes holds its RCU read lock throughout this callback.
    // The published cache has an immutable, nonempty, word-aligned bitmap.
    // Read only its immutable fields, not the RCU head modified on retirement.
    let (last, size, bitmap, final_word) = unsafe {
        let last = core::ptr::addr_of!((*primes).last).read();
        let size = core::ptr::addr_of!((*primes).sz).read();
        let bitmap = core::ptr::addr_of!((*primes).primes).cast::<ffi::c_ulong>();
        let words = size.wrapping_add(ffi::c_ulong::BITS as ffi::c_ulong - 1)
            / ffi::c_ulong::BITS as ffi::c_ulong;
        (last, size, bitmap, bitmap.add(words as usize - 1).read())
    };
    // SAFETY: KUnit owns this live suite and its name/log for the callback.
    // These are the exact kunit_info format, prefix, argument types and order.
    // In particular, %*pbl receives the original int width and bitmap pointer;
    // neither call retains that RCU-protected pointer. Do not add a newline.
    unsafe {
        suite_printk!(
            c"\x016    # %s: primes.{last=%lu, .sz=%lu, .primes[]=...x%lx} = %*pbl",
            core::ptr::addr_of!((*suite).name).cast::<ffi::c_char>(),
            last,
            size,
            final_word,
            size as ffi::c_int,
            bitmap,
        );
        bindings::kunit_log_append(
            core::ptr::addr_of!((*suite).log).read(),
            c"    # %s: primes.{last=%lu, .sz=%lu, .primes[]=...x%lx} = %*pbl"
                .as_ptr()
                .cast(),
            core::ptr::addr_of!((*suite).name).cast::<ffi::c_char>(),
            last,
            size,
            final_word,
            size as ffi::c_int,
            bitmap,
        );
    }
}

unsafe extern "C" fn prime_numbers_test(test: *mut bindings::kunit) {
    let mut last: ffi::c_ulong = 0;
    let mut x: ffi::c_ulong = 2;
    while x < 65536 {
        let slow = private::slow_is_prime_number(x);
        // SAFETY: This native predicate accepts every unsigned-long input and
        // deliberately exercises the selected provider.
        let fast = unsafe { bindings::is_prime_number(x) };
        // SAFETY: KUnit supplies this live context. No guard, pinned local, or
        // resource requiring destruction spans this fatal native assertion.
        // Preserve both operand labels and the original unsigned-long message.
        unsafe { kernel::kunit_assert_eq_msg!(test, slow, fast, c"is-prime(%lu)", x) };
        if slow {
            // SAFETY: The native next-prime function accepts unsigned long.
            let next = unsafe { bindings::next_prime_number(last) };
            // SAFETY: As above. The message reports last, not the expected x.
            unsafe { kernel::kunit_assert_eq_msg!(test, next, x, c"next-prime(%lu)", last) };
            last = next;
        }
        x += 1;
    }
}

unsafe extern "C" fn kunit_suite_exit(suite: *mut bindings::kunit_suite) {
    // SAFETY: KUnit supplies its live suite. with_primes calls dump_primes
    // synchronously under RCU, with the original context and actual cache FAM.
    unsafe { private::with_primes(suite.cast(), dump_primes) };
}

const fn prime_case() -> bindings::kunit_case {
    let mut case = kernel::kunit::kunit_case(c"prime_numbers_test", prime_numbers_test);
    case.module_name = c"prime_numbers_kunit".as_ptr().cast_mut().cast();
    case.attr.speed = bindings::kunit_speed_KUNIT_SPEED_UNSET;
    case
}

static mut TEST_CASES: [bindings::kunit_case; 2] = [
    prime_case(),
    // SAFETY: An all-zero case is the original array's terminator.
    unsafe { core::mem::zeroed() },
];

// SAFETY: The cases and callbacks remain live for the registered suite.
kernel::kunit_unsafe_test_suite!(
    "math-prime_numbers",
    TEST_CASES,
    suite_exit = kunit_suite_exit
);

#[cfg(MODULE)]
const MODINFO: &str = "author=Intel Corporation\0description=Prime number library\0license=GPL\0";
#[cfg(not(MODULE))]
const MODINFO: &str = concat!(
    "prime_numbers_kunit.author=Intel Corporation\0",
    "prime_numbers_kunit.description=Prime number library\0",
    "prime_numbers_kunit.license=GPL\0",
    "prime_numbers_kunit.file=",
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
