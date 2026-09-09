// SPDX-License-Identifier: GPL-2.0
/* Test module for in-kernel synthetic event creation and generation. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)]
pub struct trace_array;
#[repr(C)]
pub struct dynevent_cmd;
#[repr(C)]
pub struct synth_event_trace_state;
#[repr(C)]
pub struct trace_event_file {
    pub tr: *mut trace_array,
}

#[repr(C)]
pub struct synth_field_desc {
    pub type_: *const c_char,
    pub name: *const c_char,
}

extern "C" {
    static THIS_MODULE: *mut c_void;
    fn kzalloc(size: usize, flags: c_ulong) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn synth_event_cmd_init(cmd: *mut dynevent_cmd, buf: *mut c_char, len: usize);
    fn synth_event_gen_cmd_start(cmd: *mut dynevent_cmd, name: *const c_char, module: *mut c_void, ...)
        -> c_int;
    fn synth_event_add_field(cmd: *mut dynevent_cmd, type_: *const c_char, name: *const c_char) -> c_int;
    fn synth_event_gen_cmd_end(cmd: *mut dynevent_cmd) -> c_int;
    fn trace_get_event_file(instance: *mut c_void, system: *const c_char, event: *const c_char)
        -> *mut trace_event_file;
    fn trace_put_event_file(file: *mut trace_event_file);
    fn trace_array_set_clr_event(tr: *mut trace_array, system: *const c_char, event: *const c_char,
                                 set: bool) -> c_int;
    fn synth_event_delete(name: *const c_char) -> c_int;
    fn synth_event_trace_array(file: *mut trace_event_file, vals: *const u64, count: usize) -> c_int;
    fn synth_event_create(name: *const c_char, fields: *const synth_field_desc, count: usize,
                          module: *mut c_void) -> c_int;
    fn synth_event_trace_start(file: *mut trace_event_file, state: *mut synth_event_trace_state) -> c_int;
    fn synth_event_add_next_val(val: u64, state: *mut synth_event_trace_state) -> c_int;
    fn synth_event_add_val(name: *const c_char, val: u64, state: *mut synth_event_trace_state) -> c_int;
    fn synth_event_trace_end(state: *mut synth_event_trace_state) -> c_int;
    fn synth_event_trace(file: *mut trace_event_file, count: usize, ...) -> c_int;
    fn raw_smp_processor_id() -> c_uint;
    fn warn_on(condition: bool) -> bool;
}

const MAX_DYNEVENT_CMD_LEN: usize = 4096;
const GFP_KERNEL: c_ulong = 0;

static mut create_synth_test: *mut trace_event_file = core::ptr::null_mut();
static mut empty_synth_test: *mut trace_event_file = core::ptr::null_mut();
static mut gen_synth_test: *mut trace_event_file = core::ptr::null_mut();

unsafe fn c(s: &'static [u8]) -> *const c_char { s.as_ptr() as *const c_char }
unsafe fn ptrval(s: &'static [u8]) -> u64 { s.as_ptr() as usize as u64 }

unsafe fn test_gen_synth_cmd() -> c_int {
    let mut cmd: dynevent_cmd = core::mem::zeroed();
    let mut vals = [0u64; 7];
    let buf = kzalloc(MAX_DYNEVENT_CMD_LEN, GFP_KERNEL) as *mut c_char;
    if buf.is_null() { return -12; }
    synth_event_cmd_init(&mut cmd, buf, MAX_DYNEVENT_CMD_LEN);
    let mut ret = synth_event_gen_cmd_start(&mut cmd, c(b"gen_synth_test\0"), THIS_MODULE,
        c(b"pid_t\0"), c(b"next_pid_field\0"), c(b"char[16]\0"), c(b"next_comm_field\0"),
        c(b"u64\0"), c(b"ts_ns\0"), c(b"u64\0"), c(b"ts_ms\0"));
    if ret != 0 { kfree(buf as *mut c_void); return ret; }
    for (t, n) in [(b"unsigned int\0", b"cpu\0"), (b"char[64]\0", b"my_string_field\0"),
                   (b"int\0", b"my_int_field\0")] {
        ret = synth_event_add_field(&mut cmd, c(t), c(n));
        if ret != 0 { kfree(buf as *mut c_void); return ret; }
    }
    ret = synth_event_gen_cmd_end(&mut cmd);
    if ret != 0 { kfree(buf as *mut c_void); return ret; }
    gen_synth_test = trace_get_event_file(core::ptr::null_mut(), c(b"synthetic\0"), c(b"gen_synth_test\0"));
    if gen_synth_test as isize == -1 { ret = -1; synth_event_delete(c(b"gen_synth_test\0")); kfree(buf as *mut c_void); return ret; }
    ret = trace_array_set_clr_event((*gen_synth_test).tr, c(b"synthetic\0"), c(b"gen_synth_test\0"), true);
    if ret != 0 { trace_put_event_file(gen_synth_test); synth_event_delete(c(b"gen_synth_test\0")); kfree(buf as *mut c_void); return ret; }
    vals = [777, ptrval(b"hula hoops\0"), 1000000, 1000, raw_smp_processor_id() as u64, ptrval(b"thneed\0"), 598];
    ret = synth_event_trace_array(gen_synth_test, vals.as_ptr(), vals.len());
    kfree(buf as *mut c_void);
    ret
}

unsafe fn test_empty_synth_event() -> c_int {
    let mut cmd: dynevent_cmd = core::mem::zeroed();
    let buf = kzalloc(MAX_DYNEVENT_CMD_LEN, GFP_KERNEL) as *mut c_char;
    if buf.is_null() { return -12; }
    synth_event_cmd_init(&mut cmd, buf, MAX_DYNEVENT_CMD_LEN);
    let mut ret = synth_event_gen_cmd_start(&mut cmd, c(b"empty_synth_test\0"), THIS_MODULE);
    let fields = [(b"pid_t\0",b"next_pid_field\0"),(b"char[16]\0",b"next_comm_field\0"),(b"u64\0",b"ts_ns\0"),(b"u64\0",b"ts_ms\0"),(b"unsigned int\0",b"cpu\0"),(b"char[64]\0",b"my_string_field\0"),(b"int\0",b"my_int_field\0")];
    if ret == 0 { for (t,n) in fields { ret=synth_event_add_field(&mut cmd,c(t),c(n)); if ret!=0 { break; } } }
    if ret == 0 { ret=synth_event_gen_cmd_end(&mut cmd); }
    if ret == 0 { empty_synth_test=trace_get_event_file(core::ptr::null_mut(),c(b"synthetic\0"),c(b"empty_synth_test\0")); ret=trace_array_set_clr_event((*empty_synth_test).tr,c(b"synthetic\0"),c(b"empty_synth_test\0"),true); }
    if ret == 0 { let vals=[777,ptrval(b"tiddlywinks\0"),1000000,1000,raw_smp_processor_id() as u64,ptrval(b"thneed_2.0\0"),399]; ret=synth_event_trace_array(empty_synth_test,vals.as_ptr(),vals.len()); }
    kfree(buf as *mut c_void); if ret!=0 { synth_event_delete(c(b"empty_synth_test\0")); } ret
}

static mut create_synth_test_fields: [synth_field_desc; 9] = [
    synth_field_desc{type_:c(b"pid_t\0"),name:c(b"next_pid_field\0")}, synth_field_desc{type_:c(b"char[16]\0"),name:c(b"next_comm_field\0")}, synth_field_desc{type_:c(b"u64\0"),name:c(b"ts_ns\0")}, synth_field_desc{type_:c(b"char[]\0"),name:c(b"dynstring_field_1\0")}, synth_field_desc{type_:c(b"u64\0"),name:c(b"ts_ms\0")}, synth_field_desc{type_:c(b"unsigned int\0"),name:c(b"cpu\0")}, synth_field_desc{type_:c(b"char[64]\0"),name:c(b"my_string_field\0")}, synth_field_desc{type_:c(b"char[]\0"),name:c(b"dynstring_field_2\0")}, synth_field_desc{type_:c(b"int\0"),name:c(b"my_int_field\0")},
];

unsafe fn test_create_synth_event() -> c_int {
    let mut ret=synth_event_create(c(b"create_synth_test\0"),create_synth_test_fields.as_ptr(),9,THIS_MODULE); if ret!=0{return ret;}
    create_synth_test=trace_get_event_file(core::ptr::null_mut(),c(b"synthetic\0"),c(b"create_synth_test\0")); ret=trace_array_set_clr_event((*create_synth_test).tr,c(b"synthetic\0"),c(b"create_synth_test\0"),true); if ret!=0 {synth_event_delete(c(b"create_synth_test\0"));return ret;}
    let vals=[777,ptrval(b"tiddlywinks\0"),1000000,ptrval(b"xrayspecs\0"),1000,raw_smp_processor_id() as u64,ptrval(b"thneed\0"),ptrval(b"kerplunk\0"),398]; synth_event_trace_array(create_synth_test,vals.as_ptr(),9)
}

unsafe fn test_add_next_synth_val() -> c_int { let mut s: synth_event_trace_state=core::mem::zeroed(); let mut r=synth_event_trace_start(gen_synth_test,&mut s); if r!=0{return r;} for v in [777,ptrval(b"slinky\0"),1000000,1000,raw_smp_processor_id() as u64,ptrval(b"thneed_2.01\0"),395] { r=synth_event_add_next_val(v,&mut s); if r!=0{break;} } synth_event_trace_end(&mut s) }
unsafe fn test_add_synth_val() -> c_int { let mut s: synth_event_trace_state=core::mem::zeroed(); let mut r=synth_event_trace_start(gen_synth_test,&mut s); if r!=0{return r;} for (n,v) in [(b"ts_ns\0",1000000),(b"ts_ms\0",1000),(b"cpu\0",raw_smp_processor_id() as u64),(b"next_pid_field\0",777),(b"next_comm_field\0",ptrval(b"silly putty\0")),(b"my_string_field\0",ptrval(b"thneed_9\0")),(b"my_int_field\0",3999)] { r=synth_event_add_val(c(n),v,&mut s); if r!=0{break;} } synth_event_trace_end(&mut s) }
unsafe fn test_trace_synth_event() -> c_int { synth_event_trace(create_synth_test,9,444u64,ptrval(b"clackers\0"),1000000u64,ptrval(b"viewmaster\0"),1000u64,raw_smp_processor_id() as u64,ptrval(b"Thneed\0"),ptrval(b"yoyos\0"),999u64) }

#[no_mangle] pub unsafe extern "C" fn synth_event_gen_test_init() -> c_int { let r=test_gen_synth_cmd(); if r!=0{return r;} let r=test_empty_synth_event(); if r!=0{return r;} let r=test_create_synth_event(); if r!=0{return r;} warn_on(test_add_next_synth_val()!=0); warn_on(test_add_synth_val()!=0); warn_on(test_trace_synth_event()!=0); 0 }
#[no_mangle] pub unsafe extern "C" fn synth_event_gen_test_exit() { for (f,n) in [(gen_synth_test,b"gen_synth_test\0"),(empty_synth_test,b"empty_synth_test\0"),(create_synth_test,b"create_synth_test\0")] { warn_on(trace_array_set_clr_event((*f).tr,c(b"synthetic\0"),c(n),false)!=0); trace_put_event_file(f); warn_on(synth_event_delete(c(n))!=0); } }

// module_init(synth_event_gen_test_init)
// module_exit(synth_event_gen_test_exit)
// MODULE_AUTHOR("Tom Zanussi"); MODULE_DESCRIPTION("synthetic event generation test"); MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
