// SPDX-License-Identifier: GPL-2.0-only
//! Original list-sort KUnit checks against either native provider.

use core::{
    mem::{offset_of, size_of},
    ptr,
};
use kernel::{bindings, ffi};

#[allow(unreachable_pub)]
#[path = "../../include/linux/list_sort_header.rs"]
mod list_sort;

const TEST_LIST_LEN: usize = 512 + 128 + 2;
const TEST_POISON1: u32 = 0xDEADBEEF;
const TEST_POISON2: u32 = 0xA324354C;

#[repr(C)]
struct DebugEl {
    poison1: u32,
    list: bindings::list_head,
    poison2: u32,
    value: i32,
    serial: u32,
}

// These adapters are local because the shared helpers do not yet implement
// pointer assertions, inequalities, or explicit original C operand labels.
// Use the same native diagnostic records and atomic last_seen stores as
// kernel::kunit. Fatal calls have its documented test-only foreign-exit rules:
// no guards, destructors, or stack-pinned Rust state may span an assertion.
macro_rules! location {
    ($test:expr) => {{
        #[repr(transparent)]
        struct Location(bindings::kunit_loc);
        // SAFETY: The immutable location refers only to a static C string.
        unsafe impl Sync for Location {}
        static LOCATION: Location = Location(bindings::kunit_loc {
            file: concat!(file!(), "\0").as_ptr().cast(),
            line: line!() as ffi::c_int,
        });
        kernel::sync::atomic::atomic_store(
            ptr::addr_of_mut!((*$test).last_seen.file),
            LOCATION.0.file,
            kernel::sync::atomic::Relaxed,
        );
        kernel::sync::atomic::atomic_store(
            ptr::addr_of_mut!((*$test).last_seen.line),
            LOCATION.0.line,
            kernel::sync::atomic::Relaxed,
        );
        ptr::addr_of!(LOCATION.0)
    }};
}

macro_rules! compare {
    ($test:expr, $fatal:expr, $record:ident, $formatter:ident,
     $left:expr, $op:tt, $right:expr, $left_text:expr, $right_text:expr, $message:expr) => {{
        let test = $test;
        let left = $left;
        let right = $right;
        #[repr(transparent)]
        struct Text(bindings::kunit_binary_assert_text);
        // SAFETY: Every field points to an immutable static C string.
        unsafe impl Sync for Text {}
        static TEXT: Text = Text(bindings::kunit_binary_assert_text {
            operation: concat!(stringify!($op), "\0").as_ptr().cast(),
            left_text: $left_text.as_ptr().cast(),
            right_text: $right_text.as_ptr().cast(),
        });
        let loc = location!(test);
        if !(left $op right) {
            let assertion = bindings::$record {
                assert: bindings::kunit_assert {},
                text: ptr::addr_of!(TEXT.0),
                left_value: left as _,
                right_value: right as _,
            };
            bindings::__kunit_do_failed_assertion(
                test, loc,
                if $fatal { bindings::kunit_assert_type_KUNIT_ASSERTION }
                else { bindings::kunit_assert_type_KUNIT_EXPECTATION },
                ptr::addr_of!(assertion.assert), Some(bindings::$formatter),
                $message.as_ptr().cast(),
            );
            if $fatal {
                bindings::__kunit_abort(test);
            }
        }
    }};
}

macro_rules! int_check {
    ($test:expr, $fatal:expr, $left:expr, $op:tt, $right:expr,
     $left_text:expr, $right_text:expr, $message:expr) => {
        compare!(
            $test,
            $fatal,
            kunit_binary_assert,
            kunit_binary_assert_format,
            $left,
            $op,
            $right,
            $left_text,
            $right_text,
            $message
        )
    };
}

macro_rules! ptr_check {
    ($test:expr, $fatal:expr, $left:expr, $right:expr,
     $left_text:expr, $right_text:expr, $message:expr) => {
        compare!($test, $fatal, kunit_binary_ptr_assert, kunit_binary_ptr_assert_format,
                 $left, ==, $right, $left_text, $right_text, $message)
    };
}

macro_rules! allocated {
    ($test:expr, $ptr:ident) => {{
        let value = $ptr;
        let loc = location!($test);
        if value.is_null() || value as usize >= (-(bindings::MAX_ERRNO as isize)) as usize {
            let assertion = bindings::kunit_ptr_not_err_assert {
                assert: bindings::kunit_assert {},
                text: concat!(stringify!($ptr), "\0").as_ptr().cast(),
                value: value.cast(),
            };
            bindings::__kunit_do_failed_assertion(
                $test,
                loc,
                bindings::kunit_assert_type_KUNIT_ASSERTION,
                ptr::addr_of!(assertion.assert),
                Some(bindings::kunit_ptr_not_err_assert_format),
                ptr::null(),
            );
            bindings::__kunit_abort($test);
        }
    }};
}

unsafe fn check(test: *mut bindings::kunit, ela: *const DebugEl, elb: *const DebugEl) {
    // SAFETY: The comparator receives live elements and the KUnit-owned pointer
    // table. As in C, serials must address that table even after an expectation
    // fails; tests injecting invalid serials must provide backing storage.
    unsafe {
        let elts = (*test).priv_.cast::<*mut DebugEl>();
        int_check!(test, false, (*ela).serial, <, TEST_LIST_LEN as u32,
                   c"ela->serial", c"(unsigned int)(512+128+2)", c"incorrect serial");
        int_check!(test, false, (*elb).serial, <, TEST_LIST_LEN as u32,
                   c"elb->serial", c"(unsigned int)(512+128+2)", c"incorrect serial");
        ptr_check!(
            test,
            false,
            *elts.add((*ela).serial as usize),
            ela.cast_mut(),
            c"elts[ela->serial]",
            c"ela",
            c"phantom element"
        );
        ptr_check!(
            test,
            false,
            *elts.add((*elb).serial as usize),
            elb.cast_mut(),
            c"elts[elb->serial]",
            c"elb",
            c"phantom element"
        );
        int_check!(test, false, (*ela).poison1, ==, TEST_POISON1,
                   c"ela->poison1", c"0xDEADBEEF", c"bad poison");
        int_check!(test, false, (*ela).poison2, ==, TEST_POISON2,
                   c"ela->poison2", c"0xA324354C", c"bad poison");
        int_check!(test, false, (*elb).poison1, ==, TEST_POISON1,
                   c"elb->poison1", c"0xDEADBEEF", c"bad poison");
        int_check!(test, false, (*elb).poison2, ==, TEST_POISON2,
                   c"elb->poison2", c"0xA324354C", c"bad poison");
    }
}

unsafe extern "C" fn cmp(
    priv_: *mut ffi::c_void,
    a: *const bindings::list_head,
    b: *const bindings::list_head,
) -> i32 {
    // SAFETY: list_sort passes embedded nodes, including valid self comparisons.
    unsafe {
        let ela = a.byte_sub(offset_of!(DebugEl, list)).cast::<DebugEl>();
        let elb = b.byte_sub(offset_of!(DebugEl, list)).cast::<DebugEl>();
        check(priv_.cast(), ela, elb);
        (*ela).value - (*elb).value
    }
}

unsafe fn random_value() -> i32 {
    // The original constant get_random_u32_below(214) takes the u8 multiply/
    // rejection branch, not the out-of-line variable-bound u32 implementation.
    const CEIL: u32 = (TEST_LIST_LEN / 3) as u32;
    loop {
        // SAFETY: The random byte API has no caller preconditions.
        let mult = CEIL * u32::from(unsafe { bindings::get_random_u8() });
        if u32::from(mult as u8) >= 256 % CEIL {
            return (mult >> 8) as i32;
        }
    }
}

unsafe extern "C" fn list_sort_test(test: *mut bindings::kunit) {
    // SAFETY: KUnit supplies its live context. All allocations belong to KUnit,
    // including on fatal exit. Nodes and the sentinel remain live throughout
    // this synchronous call; no Rust reference or pinned local spans it.
    unsafe {
        let mut count: i32 = 1;
        let mut head = bindings::list_head {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        let head_ptr = ptr::addr_of_mut!(head);
        head.next = head_ptr;
        head.prev = head_ptr;
        // Exact inline kunit_kcalloc/kunit_kmalloc expansion: only the pointer
        // table is zeroed; the element allocations retain GFP_KERNEL alone.
        let elts = bindings::kunit_kmalloc_array(
            test,
            TEST_LIST_LEN,
            size_of::<*mut DebugEl>(),
            bindings::GFP_KERNEL | bindings::__GFP_ZERO,
        )
        .cast::<*mut DebugEl>();
        allocated!(test, elts);
        (*test).priv_ = elts.cast();
        for i in 0..TEST_LIST_LEN {
            let el =
                bindings::kunit_kmalloc_array(test, 1, size_of::<DebugEl>(), bindings::GFP_KERNEL)
                    .cast::<DebugEl>();
            allocated!(test, el);
            (*el).value = random_value();
            (*el).serial = i as u32;
            (*el).poison1 = TEST_POISON1;
            (*el).poison2 = TEST_POISON2;
            *elts.add(i) = el;
            let node = ptr::addr_of_mut!((*el).list);
            // list_add_tail on this valid, exclusively owned list.
            (*node).next = head_ptr;
            (*node).prev = head.prev;
            (*head.prev).next = node;
            head.prev = node;
        }
        list_sort::list_sort(test.cast(), head_ptr, cmp);
        let mut cur = head.next;
        while (*cur).next != head_ptr {
            ptr_check!(
                test,
                true,
                (*(*cur).next).prev,
                cur,
                c"cur->next->prev",
                c"cur",
                c"list is corrupted"
            );
            let cmp_result = cmp(test.cast(), cur, (*cur).next);
            int_check!(test, true, cmp_result, <=, 0,
                       c"cmp_result", c"0", c"list is not sorted");
            let el = cur.byte_sub(offset_of!(DebugEl, list)).cast::<DebugEl>();
            let el1 = (*cur)
                .next
                .byte_sub(offset_of!(DebugEl, list))
                .cast::<DebugEl>();
            if cmp_result == 0 {
                int_check!(test, true, (*el).serial, <=, (*el1).serial,
                           c"el->serial", c"el1->serial", c"order of equivalent elements not preserved");
            }
            check(test, el, el1);
            count += 1;
            cur = (*cur).next;
        }
        ptr_check!(
            test,
            false,
            head.prev,
            cur,
            c"head.prev",
            c"cur",
            c"list is corrupted"
        );
        int_check!(test, false, count, ==, TEST_LIST_LEN as i32,
                   c"count", c"(512+128+2)", c"list length changed after sorting!");
    }
}

const fn sort_case() -> bindings::kunit_case {
    let mut case = kernel::kunit::kunit_case(c"list_sort_test", list_sort_test);
    case.module_name = c"test_list_sort".as_ptr().cast_mut().cast();
    case.attr.speed = bindings::kunit_speed_KUNIT_SPEED_UNSET;
    case
}

static mut TEST_CASES: [bindings::kunit_case; 2] = [
    sort_case(),
    // SAFETY: An all-zero case terminates the original C array.
    unsafe { core::mem::zeroed() },
];

// SAFETY: The static, terminated case array and its callbacks remain live.
kernel::kunit_unsafe_test_suite!("list_sort", TEST_CASES);

#[cfg(MODULE)]
const MODINFO: &str = "description=list_sort() KUnit test suite\0license=GPL\0";
#[cfg(not(MODULE))]
const MODINFO: &str = concat!(
    "test_list_sort.description=list_sort() KUnit test suite\0",
    "test_list_sort.license=GPL\0",
    "test_list_sort.file=",
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
