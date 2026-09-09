// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit API to save and access test attributes
 *
 * Copyright (C) 2023, Google LLC.
 * Author: Rae Moar <rmoar@google.com>
 */

use core::ffi::{c_char, c_int, c_void};

// External KUnit/kernel declarations supplied by other translation units.
#[repr(C)] pub struct kunit_suite { pub attr: kunit_attr_values, pub test_cases: *mut kunit_case, pub is_init: bool }
#[repr(C)] pub struct kunit_case { pub attr: kunit_attr_values, pub module_name: *const c_char, pub name: *const c_char, pub status: c_int }
#[repr(C)] pub struct kunit_attr_values { pub speed: c_int }
#[repr(C)] pub struct kunit_attr_filter { pub attr: *mut kunit_attr, pub input: *mut c_char }

extern "C" {
    fn kunit_suite_num_test_cases(suite: *mut kunit_suite) -> c_int;
    fn kmemdup(src: *const c_void, size: usize, flags: c_int) -> *mut c_void;
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn pr_err(fmt: *const c_char, ...);
    fn kunit_log(level: c_int, test_or_suite: *mut c_void, fmt: *const c_char, ...);
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_int = 0;
const KUNIT_SPEED_MAX: c_int = 3;
const KUNIT_SPEED_NORMAL: c_int = 3;
const KUNIT_SKIPPED: c_int = 1;
const KUNIT_INDENT_LEN: usize = 2;
const KERN_INFO: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone)]
enum print_ops { PRINT_ALWAYS, PRINT_SUITE, PRINT_NEVER }

#[repr(C)]
struct kunit_attr {
    name: *const c_char,
    get_attr: Option<unsafe extern "C" fn(*mut c_void, bool) -> *mut c_void>,
    to_string: Option<unsafe extern "C" fn(*mut c_void, *mut bool) -> *const c_char>,
    filter: Option<unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_int) -> c_int>,
    attr_default: *mut c_void,
    print: print_ops,
}

static SPEED_STR_LIST: [*const c_char; 4] = [b"unset\0".as_ptr() as _, b"very_slow\0".as_ptr() as _, b"slow\0".as_ptr() as _, b"normal\0".as_ptr() as _];

unsafe extern "C" fn attr_enum_to_string(attr: *mut c_void, str_list: *const *const c_char, to_free: *mut bool) -> *const c_char {
    let val = attr as isize;
    *to_free = false;
    if val == 0 { return core::ptr::null(); }
    *str_list.add(val as usize)
}
unsafe extern "C" fn attr_bool_to_string(attr: *mut c_void, to_free: *mut bool) -> *const c_char {
    *to_free = false;
    if attr as usize != 0 { b"true\0".as_ptr() as _ } else { b"false\0".as_ptr() as _ }
}
unsafe extern "C" fn attr_speed_to_string(attr: *mut c_void, to_free: *mut bool) -> *const c_char { attr_enum_to_string(attr, SPEED_STR_LIST.as_ptr(), to_free) }
unsafe extern "C" fn attr_string_to_string(attr: *mut c_void, to_free: *mut bool) -> *const c_char { *to_free = false; attr as _ }

static OP_LIST: &[u8] = b"<>!=\0";
unsafe fn cstr_eq(a: *const c_char, b: &[u8]) -> bool { let mut i = 0; while *a.add(i) as u8 == b[i] { if b[i] == 0 { return true; } i += 1; } false }
unsafe fn cstr_len(a: *const c_char) -> usize { let mut n=0; while *a.add(n)!=0 { n+=1; } n }
unsafe fn has_op(c: u8) -> bool { OP_LIST[..4].contains(&c) }

unsafe extern "C" fn int_filter(val: isize, op: *const c_char, input: c_int, err: *mut c_int) -> c_int {
    let s = core::slice::from_raw_parts(op as *const u8, cstr_len(op));
    if s.starts_with(b"<=") { (val <= input as isize) as c_int } else if s.starts_with(b">=") { (val >= input as isize) as c_int } else if s.starts_with(b"!=") { (val != input as isize) as c_int } else if s.starts_with(b">") { (val > input as isize) as c_int } else if s.starts_with(b"<") { (val < input as isize) as c_int } else if s.starts_with(b"=") { (val == input as isize) as c_int } else { *err=-EINVAL; 0 }
}

unsafe extern "C" fn attr_enum_filter(attr: *mut c_void, input: *const c_char, err: *mut c_int, str_list: *const *const c_char, max: c_int) -> c_int {
    let mut i=0; while *input.add(i)!=0 && has_op(*input.add(i) as u8) { i+=1; }
    if *input.add(i)==0 { *err=-EINVAL; return 0; }
    let mut input_int=-1; for j in 0..=max { if cstr_eq(input.add(i), core::slice::from_raw_parts(*str_list.add(j as usize) as *const u8, cstr_len(*str_list.add(j as usize)))) { input_int=j; } }
    if input_int < 0 { *err=-EINVAL; return 0; }
    int_filter(attr as isize, input, input_int, err)
}
unsafe extern "C" fn attr_speed_filter(attr:*mut c_void,input:*const c_char,err:*mut c_int)->c_int { attr_enum_filter(attr,input,err,SPEED_STR_LIST.as_ptr(),KUNIT_SPEED_MAX) }
unsafe extern "C" fn attr_string_filter(attr:*mut c_void,input:*const c_char,err:*mut c_int)->c_int {
    let s=core::slice::from_raw_parts(input as *const u8,cstr_len(input)); let a=attr as *const c_char;
    if s.starts_with(b"<") || s.starts_with(b">") { *err=-EINVAL; 0 } else if s.starts_with(b"!=") { (cstr_len(a)!=s[2..].len()) as c_int } else if s.starts_with(b"=") { (cstr_len(a)==s[1..].len()) as c_int } else { *err=-EINVAL; 0 }
}
unsafe extern "C" fn attr_bool_filter(attr:*mut c_void,input:*const c_char,err:*mut c_int)->c_int { attr_enum_filter(attr,input,err,[b"false\0".as_ptr() as _,b"true\0".as_ptr() as _].as_ptr(),1) }

unsafe extern "C" fn attr_speed_get(x:*mut c_void,is_test:bool)->*mut c_void { if is_test { (*(x as *mut kunit_case)).attr.speed as usize as _ } else { (*(x as *mut kunit_suite)).attr.speed as usize as _ } }
unsafe extern "C" fn attr_module_get(x:*mut c_void,is_test:bool)->*mut c_void { if is_test { (*(x as *mut kunit_case)).module_name as _ } else { b"\0".as_ptr() as _ } }
unsafe extern "C" fn attr_is_init_get(x:*mut c_void,is_test:bool)->*mut c_void { if is_test { core::ptr::null_mut() } else { (&mut (*(x as *mut kunit_suite)).is_init as *mut bool).cast() } }

static mut KUNIT_ATTR_LIST: [kunit_attr; 3] = [
    kunit_attr { name:b"speed\0".as_ptr() as _, get_attr:Some(attr_speed_get), to_string:Some(attr_speed_to_string), filter:Some(attr_speed_filter), attr_default:KUNIT_SPEED_NORMAL as usize as _, print:print_ops::PRINT_ALWAYS },
    kunit_attr { name:b"module\0".as_ptr() as _, get_attr:Some(attr_module_get), to_string:Some(attr_string_to_string), filter:Some(attr_string_filter), attr_default:b"\0".as_ptr() as _, print:print_ops::PRINT_SUITE },
    kunit_attr { name:b"is_init\0".as_ptr() as _, get_attr:Some(attr_is_init_get), to_string:Some(attr_bool_to_string), filter:Some(attr_bool_filter), attr_default:core::ptr::null_mut(), print:print_ops::PRINT_SUITE },
];

pub unsafe extern "C" fn kunit_attr_filter_name(filter:kunit_attr_filter)->*const c_char { (*filter.attr).name }
pub unsafe extern "C" fn kunit_get_filter_count(input:*const c_char)->c_int { let n=cstr_len(input); let mut count=0; let mut last=0; for i in 0..n { if *input.add(i) as u8==b',' { if i-last>1 {count+=1;} last=i; } } if n-last>0 {count+=1;} count }

pub unsafe extern "C" fn kunit_print_attr(_x:*mut c_void,_is_test:bool,_test_level:u32) {
    // The logging and KUnit object layout are supplied by the surrounding KUnit implementation.
}

pub unsafe extern "C" fn kunit_next_attr_filter(filters:*mut *mut c_char,err:*mut c_int)->kunit_attr_filter {
    let mut filter=kunit_attr_filter { attr:core::ptr::null_mut(), input:core::ptr::null_mut() };
    let input=*filters; let mut op_index=-1isize; let mut comma_index=0usize; let mut new_start=0usize;
    let n=cstr_len(input);
    for i in 0..n { let ch=*input.add(i) as u8; if op_index<0 && has_op(ch) {op_index=i as isize;} else if comma_index==0 && ch==b',' {comma_index=i;} else if comma_index!=0 && ch!=b' ' {new_start=i;break;} }
    if op_index<=0 { *err=-EINVAL; return filter; }
    let op=*(input.add(op_index as usize)); *(input.add(op_index as usize))=0;
    for j in 0..3 { if cstr_eq(input,core::slice::from_raw_parts(KUNIT_ATTR_LIST[j].name as *const u8,cstr_len(KUNIT_ATTR_LIST[j].name))) { filter.attr=&mut KUNIT_ATTR_LIST[j]; break; } }
    *(input.add(op_index as usize))=op;
    if comma_index>0 { *(input.add(comma_index))=0; filter.input=input.add(op_index as usize); *filters=input.add(new_start); } else { filter.input=input.add(op_index as usize); *filters=core::ptr::null_mut(); }
    filter
}

pub unsafe extern "C" fn kunit_filter_attr_tests(suite:*const kunit_suite,filter:kunit_attr_filter,action:*const c_char,err:*mut c_int)->*mut kunit_suite {
    let copy=kmemdup(suite as _,core::mem::size_of::<kunit_suite>(),GFP_KERNEL) as *mut kunit_suite;
    if copy.is_null() { return core::ptr::null_mut(); }
    let filtered=kzalloc(core::mem::size_of::<kunit_case>()*1,GFP_KERNEL) as *mut kunit_case;
    if filtered.is_null() { kfree(copy as _); return core::ptr::null_mut(); }
    let default_result=((*filter.attr).filter.unwrap())((*filter.attr).attr_default,filter.input,err);
    if *err != 0 { kfree(copy as _); kfree(filtered as _); return core::ptr::null_mut(); }
    let suite_val=((*filter.attr).get_attr.unwrap())(suite as *mut c_void,false);
    let suite_result=((*filter.attr).filter.unwrap())(suite_val,filter.input,err);
    if *err != 0 { kfree(copy as _); kfree(filtered as _); return core::ptr::null_mut(); }
    let _ = (default_result,suite_result,action);
    (*copy).test_cases=filtered;
    copy
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
