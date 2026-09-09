// SPDX-License-Identifier: GPL-2.0-or-later
/* Test cases for struct randomization, i.e. CONFIG_RANDSTRUCT=y. */

// The following kernel/KUnit items are supplied by the surrounding build.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct kunit { _private: [u8; 0] }

#[repr(u32)]
enum RandstructMemberNames {
    MEMBER_NAME_a, MEMBER_NAME_b, MEMBER_NAME_c, MEMBER_NAME_d,
    MEMBER_NAME_e, MEMBER_NAME_f, MEMBER_NAME_g, MEMBER_NAME_h,
    MEMBER_NAME_MAX,
}

#[repr(C)]
pub struct randstruct_untouched {
    pub a: usize, pub b: usize, pub c: usize, pub d: usize,
    pub e: usize, pub f: usize, pub g: usize, pub h: usize,
}

#[repr(C)]
pub struct randstruct_shuffled {
    pub a: usize, pub b: usize, pub c: usize, pub d: usize,
    pub e: usize, pub f: usize, pub g: usize, pub h: usize,
}

pub type RandstructFunc = unsafe extern "C" fn(c_int) -> usize;

#[repr(C)]
pub struct randstruct_funcs_untouched {
    pub a: RandstructFunc, pub b: RandstructFunc, pub c: RandstructFunc, pub d: RandstructFunc,
    pub e: RandstructFunc, pub f: RandstructFunc, pub g: RandstructFunc, pub h: RandstructFunc,
}

#[repr(C)]
pub struct randstruct_funcs_shuffled {
    pub a: RandstructFunc, pub b: RandstructFunc, pub c: RandstructFunc, pub d: RandstructFunc,
    pub e: RandstructFunc, pub f: RandstructFunc, pub g: RandstructFunc, pub h: RandstructFunc,
}

unsafe extern "C" fn func_a(_: c_int) -> usize { core::mem::offset_of!(randstruct_funcs_untouched, a) }
unsafe extern "C" fn func_b(_: c_int) -> usize { core::mem::offset_of!(randstruct_funcs_untouched, b) }
unsafe extern "C" fn func_c(_: c_int) -> usize { core::mem::offset_of!(randstruct_funcs_untouched, c) }
unsafe extern "C" fn func_d(_: c_int) -> usize { core::mem::offset_of!(randstruct_funcs_untouched, d) }
unsafe extern "C" fn func_e(_: c_int) -> usize { core::mem::offset_of!(randstruct_funcs_untouched, e) }
unsafe extern "C" fn func_f(_: c_int) -> usize { core::mem::offset_of!(randstruct_funcs_untouched, f) }
unsafe extern "C" fn func_g(_: c_int) -> usize { core::mem::offset_of!(randstruct_funcs_untouched, g) }
unsafe extern "C" fn func_h(_: c_int) -> usize { core::mem::offset_of!(randstruct_funcs_untouched, h) }

#[repr(C)]
pub struct randstruct_mixed_untouched {
    pub a: bool, pub b: i16, pub c: u32, pub d: usize, pub e: i8, pub f: u64,
    pub shuffled: randstruct_shuffled, pub ptr: *mut c_void,
}
#[repr(C)]
pub struct randstruct_mixed_shuffled {
    pub a: bool, pub b: i16, pub c: u32, pub d: usize, pub e: i8, pub f: u64,
    pub shuffled: randstruct_shuffled, pub ptr: *mut c_void,
}

#[repr(C)]
pub struct contains_randstruct_untouched { pub before: c_int, pub untouched: randstruct_untouched, pub after: c_int }
#[repr(C)]
pub struct contains_randstruct_shuffled { pub before: c_int, pub shuffled: randstruct_shuffled, pub after: c_int }

#[repr(C)]
pub struct contains_func_untouched {
    pub inner: randstruct_funcs_shuffled,
    pub a: RandstructFunc, pub b: RandstructFunc, pub c: RandstructFunc, pub d: RandstructFunc,
    pub e: RandstructFunc, pub f: RandstructFunc, pub g: RandstructFunc, pub h: RandstructFunc,
}
#[repr(C)]
pub struct contains_func_shuffled {
    pub inner: randstruct_funcs_shuffled,
    pub a: RandstructFunc, pub b: RandstructFunc, pub c: RandstructFunc, pub d: RandstructFunc,
    pub e: RandstructFunc, pub f: RandstructFunc, pub g: RandstructFunc, pub h: RandstructFunc,
}

extern "C" {
    fn kunit_skip(test: *mut kunit, reason: *const c_char);
    fn kunit_info(test: *mut kunit, fmt: *const c_char, ...);
}

unsafe fn randstruct_layout_same(_test: *mut kunit) {}
unsafe fn randstruct_layout_mixed(_test: *mut kunit) {}
unsafe fn randstruct_layout_fptr(_test: *mut kunit) {}
unsafe fn randstruct_layout_fptr_deep(_test: *mut kunit) {}

unsafe fn test_check_init(_test: *mut kunit, _name: *const c_char,
                         _untouched: *mut randstruct_untouched,
                         _shuffled: *mut randstruct_shuffled) {}
unsafe fn test_check_mixed_init(_test: *mut kunit, _name: *const c_char,
                                _untouched: *mut randstruct_mixed_untouched,
                                _shuffled: *mut randstruct_mixed_shuffled) {}
unsafe fn test_check_contained_init(_test: *mut kunit, _name: *const c_char,
                                    _untouched: *mut contains_randstruct_untouched,
                                    _shuffled: *mut contains_randstruct_shuffled) {}
unsafe fn test_check_funcs_init(_test: *mut kunit, _name: *const c_char,
                                _untouched: *mut randstruct_funcs_untouched,
                                _shuffled: *mut randstruct_funcs_shuffled) {}

unsafe fn randstruct_initializers(test: *mut kunit) {
    let mut untouched = randstruct_untouched { a: 1, b: 3, c: 5, d: 7, e: 11, f: 13, g: 17, h: 19 };
    let mut shuffled = randstruct_shuffled { a: 1, b: 3, c: 5, d: 7, e: 11, f: 13, g: 17, h: 19 };
    test_check_init(test, c"named".as_ptr(), &mut untouched, &mut shuffled);
    test_check_init(test, c"unnamed".as_ptr(), &mut untouched, &mut shuffled);
}

unsafe fn randstruct_test_init(_test: *mut kunit) -> c_int { 0 }

#[no_mangle]
pub static mut randstruct_test_cases: [*const c_void; 6] = [
    randstruct_layout_same as *const c_void, randstruct_layout_mixed as *const c_void,
    randstruct_layout_fptr as *const c_void, randstruct_layout_fptr_deep as *const c_void,
    randstruct_initializers as *const c_void, core::ptr::null(),
];

#[repr(C)]
pub struct kunit_suite {
    pub name: *const c_char,
    pub init: Option<unsafe fn(*mut kunit) -> c_int>,
    pub test_cases: *mut *const c_void,
}

#[no_mangle]
pub static mut randstruct_test_suite: kunit_suite = kunit_suite {
    name: c"randstruct".as_ptr(), init: Some(randstruct_test_init),
    test_cases: randstruct_test_cases.as_mut_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
