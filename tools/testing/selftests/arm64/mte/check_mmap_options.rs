// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2020 ARM Limited

// C dependencies: assert.h, errno.h, fcntl.h, signal.h, stdio.h, stdlib.h,
// string.h, ucontext.h, sys/mman.h, sys/stat.h, sys/types.h,
// kselftest.h, mte_common_util.h, mte_def.h.

use std::ffi::{c_char, c_int, c_void};

const RUNS: c_int = MT_TAG_COUNT;
const UNDERFLOW: c_int = MT_GRANULE_SIZE;
const OVERFLOW: c_int = MT_GRANULE_SIZE;
const TAG_CHECK_ON: c_int = 0;
const TAG_CHECK_OFF: c_int = 1;
const ATAG_CHECK_ON: c_int = 1;
const ATAG_CHECK_OFF: c_int = 0;

const TEST_NAME_MAX: usize = 256;

const CHECK_ANON_MEM: c_int = 0;
const CHECK_FILE_MEM: c_int = 1;
const CHECK_CLEAR_PROT_MTE: c_int = 2;

const TAG_OP_ALL: c_int = 0;
const TAG_OP_STONLY: c_int = 1;

#[repr(C)]
struct check_mmap_testcase {
    check_type: c_int,
    mem_type: c_int,
    mte_sync: c_int,
    mapping: c_int,
    tag_check: c_int,
    atag_check: c_int,
    tag_op: c_int,
    enable_tco: bool,
}

// External constants supplied by the translated test harness headers.
const MT_TAG_COUNT: c_int = 16;
const MT_GRANULE_SIZE: c_int = 16;
const KSFT_SKIP: c_int = 4;
const KSFT_FAIL: c_int = 1;
const KSFT_PASS: c_int = 0;
const MTE_ALLOW_NON_ZERO_TAG: c_int = 0;
const USE_MMAP: c_int = 0;
const USE_MPROTECT: c_int = 1;
const MTE_NONE_ERR: c_int = 0;
const MTE_SYNC_ERR: c_int = 1;
const MTE_ASYNC_ERR: c_int = 2;
const MAP_SHARED: c_int = 0x01;
const MAP_PRIVATE: c_int = 0x02;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const SIGBUS: c_int = 7;
const SIGSEGV: c_int = 11;

#[repr(C)]
struct mte_context {
    fault_valid: bool,
}

unsafe extern "C" {
    static mut mtefar_support: bool;
    static mut mtestonly_support: bool;
    static mut cur_mte_cxt: mte_context;

    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn mprotect(addr: *mut c_void, len: usize, prot: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn getpagesize() -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;

    fn mte_insert_atag(ptr: *mut c_char) -> *mut c_char;
    fn mte_initialize_current_context(mode: c_int, ptr: usize, size: c_int);
    fn mte_wait_after_trig();
    fn mte_switch_mode(mode: c_int, mask: c_int, tag_op: c_int);
    fn mte_allocate_memory(map_size: c_int, mem_type: c_int, mapping: c_int, zero: bool) -> *mut c_void;
    fn check_allocated_memory(ptr: *mut c_char, map_size: c_int, mem_type: c_int, zero: bool) -> c_int;
    fn mte_insert_tags(ptr: *mut c_void, size: c_int) -> *mut c_char;
    fn ksft_print_msg(format: *const c_char, ...);
    fn mte_clear_tags(ptr: *mut c_void, size: c_int);
    fn mte_free_memory(ptr: *mut c_void, map_size: c_int, mem_type: c_int, zero: bool);
    fn create_temp_file() -> c_int;
    fn mte_allocate_file_memory(map_size: c_int, mem_type: c_int, mapping: c_int, zero: bool, fd: c_int) -> *mut c_void;
    fn mte_allocate_memory_tag_range(size: c_int, mem_type: c_int, mapping: c_int, underflow: c_int, overflow: c_int) -> *mut c_void;
    fn check_allocated_memory_range(ptr: *mut c_char, size: c_int, mem_type: c_int, underflow: c_int, overflow: c_int) -> c_int;
    fn mte_free_memory_tag_range(ptr: *mut c_void, size: c_int, mem_type: c_int, underflow: c_int, overflow: c_int);
    fn mte_allocate_file_memory_tag_range(size: c_int, mem_type: c_int, mapping: c_int, underflow: c_int, overflow: c_int, fd: c_int) -> *mut c_void;
    fn ksft_print_header();
    fn mte_default_setup() -> c_int;
    fn ksft_set_plan(plan: usize);
    fn mte_register_signal(signal: c_int, handler: unsafe extern "C" fn(c_int), check_atag: bool);
    fn mte_default_handler(signal: c_int);
    fn mte_enable_pstate_tco();
    fn mte_disable_pstate_tco();
    fn evaluate_test(result: c_int, name: *const c_char);
    fn mte_restore_setup();
    fn ksft_print_cnts();
    fn ksft_get_fail_cnt() -> c_int;
}

static mut page_size: usize = 0;
static mut sizes: [c_int; 9] = [
    1, 537, 989, 1269, MT_GRANULE_SIZE - 1, MT_GRANULE_SIZE,
    /* page size - 1*/ 0, /* page_size */ 0, /* page size + 1 */ 0,
];

unsafe fn check_mte_memory(mut ptr: *mut c_char, size: c_int, mode: c_int,
                           tag_check: c_int, atag_check: c_int, tag_op: c_int) -> c_int {
    let mut buf: [c_char; MT_GRANULE_SIZE as usize] = [0; MT_GRANULE_SIZE as usize];

    if !mtefar_support && atag_check == ATAG_CHECK_ON {
        return KSFT_SKIP;
    }

    if atag_check == ATAG_CHECK_ON {
        ptr = mte_insert_atag(ptr);
    }

    mte_initialize_current_context(mode, ptr as usize, size);
    memset(ptr as *mut c_void, '1' as c_int, size as usize);
    mte_wait_after_trig();
    if cur_mte_cxt.fault_valid == true {
        return KSFT_FAIL;
    }

    mte_initialize_current_context(mode, ptr as usize, -UNDERFLOW);
    memset(ptr.offset(-(UNDERFLOW as isize)) as *mut c_void, '2' as c_int, UNDERFLOW as usize);
    mte_wait_after_trig();
    if cur_mte_cxt.fault_valid == false && tag_check == TAG_CHECK_ON {
        return KSFT_FAIL;
    }
    if cur_mte_cxt.fault_valid == true && tag_check == TAG_CHECK_OFF {
        return KSFT_FAIL;
    }

    mte_initialize_current_context(mode, ptr as usize, size + OVERFLOW);
    memset(ptr.offset(size as isize) as *mut c_void, '3' as c_int, OVERFLOW as usize);
    mte_wait_after_trig();
    if cur_mte_cxt.fault_valid == false && tag_check == TAG_CHECK_ON {
        return KSFT_FAIL;
    }
    if cur_mte_cxt.fault_valid == true && tag_check == TAG_CHECK_OFF {
        return KSFT_FAIL;
    }

    if tag_op == TAG_OP_STONLY {
        mte_initialize_current_context(mode, ptr as usize, -UNDERFLOW);
        memcpy(buf.as_mut_ptr() as *mut c_void, ptr.offset(-(UNDERFLOW as isize)) as *const c_void, MT_GRANULE_SIZE as usize);
        mte_wait_after_trig();
        if cur_mte_cxt.fault_valid == true {
            return KSFT_FAIL;
        }

        mte_initialize_current_context(mode, ptr as usize, size + OVERFLOW);
        memcpy(buf.as_mut_ptr() as *mut c_void, ptr.offset(size as isize) as *const c_void, MT_GRANULE_SIZE as usize);
        mte_wait_after_trig();
        if cur_mte_cxt.fault_valid == true {
            return KSFT_FAIL;
        }
    }

    KSFT_PASS
}

unsafe fn check_anonymous_memory_mapping(mem_type: c_int, mode: c_int, mapping: c_int,
                                         tag_check: c_int, atag_check: c_int, tag_op: c_int) -> c_int {
    let mut ptr: *mut c_char;
    let mut map_ptr: *mut c_char;
    let mut result: c_int;
    let mut map_size: c_int;
    let item = sizes.len();

    if tag_op == TAG_OP_STONLY && !mtestonly_support {
        return KSFT_SKIP;
    }

    mte_switch_mode(mode, MTE_ALLOW_NON_ZERO_TAG, tag_op);
    for run in 0..item {
        map_size = sizes[run] + OVERFLOW + UNDERFLOW;
        map_ptr = mte_allocate_memory(map_size, mem_type, mapping, false) as *mut c_char;
        if check_allocated_memory(map_ptr, map_size, mem_type, false) != KSFT_PASS {
            return KSFT_FAIL;
        }

        ptr = map_ptr.offset(UNDERFLOW as isize);
        mte_initialize_current_context(mode, ptr as usize, sizes[run]);
        /* Only mte enabled memory will allow tag insertion */
        ptr = mte_insert_tags(ptr as *mut c_void, sizes[run]);
        if ptr.is_null() || cur_mte_cxt.fault_valid == true {
            ksft_print_msg(c"FAIL: Insert tags on anonymous mmap memory\n".as_ptr());
            munmap(map_ptr as *mut c_void, map_size as usize);
            return KSFT_FAIL;
        }
        result = check_mte_memory(ptr, sizes[run], mode, tag_check, atag_check, tag_op);
        mte_clear_tags(ptr as *mut c_void, sizes[run]);
        mte_free_memory(map_ptr as *mut c_void, map_size, mem_type, false);
        if result != KSFT_PASS {
            return result;
        }
    }
    KSFT_PASS
}

unsafe fn check_file_memory_mapping(mem_type: c_int, mode: c_int, mapping: c_int,
                                    tag_check: c_int, atag_check: c_int, tag_op: c_int) -> c_int {
    let mut ptr: *mut c_char;
    let mut map_ptr: *mut c_char;
    let mut fd: c_int;
    let mut map_size: c_int;
    let total = sizes.len();
    let mut result = KSFT_PASS;

    if tag_op == TAG_OP_STONLY && !mtestonly_support {
        return KSFT_SKIP;
    }

    mte_switch_mode(mode, MTE_ALLOW_NON_ZERO_TAG, tag_op);
    for run in 0..total {
        fd = create_temp_file();
        if fd == -1 {
            return KSFT_FAIL;
        }

        map_size = sizes[run] + UNDERFLOW + OVERFLOW;
        map_ptr = mte_allocate_file_memory(map_size, mem_type, mapping, false, fd) as *mut c_char;
        if check_allocated_memory(map_ptr, map_size, mem_type, false) != KSFT_PASS {
            close(fd);
            return KSFT_FAIL;
        }
        ptr = map_ptr.offset(UNDERFLOW as isize);
        mte_initialize_current_context(mode, ptr as usize, sizes[run]);
        /* Only mte enabled memory will allow tag insertion */
        ptr = mte_insert_tags(ptr as *mut c_void, sizes[run]);
        if ptr.is_null() || cur_mte_cxt.fault_valid == true {
            ksft_print_msg(c"FAIL: Insert tags on file based memory\n".as_ptr());
            munmap(map_ptr as *mut c_void, map_size as usize);
            close(fd);
            return KSFT_FAIL;
        }
        result = check_mte_memory(ptr, sizes[run], mode, tag_check, atag_check, tag_op);
        mte_clear_tags(ptr as *mut c_void, sizes[run]);
        munmap(map_ptr as *mut c_void, map_size as usize);
        close(fd);
        if result != KSFT_PASS {
            return result;
        }
    }
    KSFT_PASS
}

unsafe fn check_clear_prot_mte_flag(mem_type: c_int, mode: c_int, mapping: c_int, atag_check: c_int) -> c_int {
    let mut ptr: *mut c_char;
    let mut map_ptr: *mut c_char;
    let mut result: c_int;
    let mut fd: c_int;
    let mut map_size: c_int;
    let total = sizes.len();

    let prot_flag = PROT_READ | PROT_WRITE;
    mte_switch_mode(mode, MTE_ALLOW_NON_ZERO_TAG, false as c_int);
    for run in 0..total {
        map_size = sizes[run] + OVERFLOW + UNDERFLOW;
        ptr = mte_allocate_memory_tag_range(sizes[run], mem_type, mapping, UNDERFLOW, OVERFLOW) as *mut c_char;
        if check_allocated_memory_range(ptr, sizes[run], mem_type, UNDERFLOW, OVERFLOW) != KSFT_PASS {
            return KSFT_FAIL;
        }
        map_ptr = ptr.offset(-(UNDERFLOW as isize));
        /* Try to clear PROT_MTE property and verify it by tag checking */
        if mprotect(map_ptr as *mut c_void, map_size as usize, prot_flag) != 0 {
            mte_free_memory_tag_range(ptr as *mut c_void, sizes[run], mem_type, UNDERFLOW, OVERFLOW);
            ksft_print_msg(c"FAIL: mprotect not ignoring clear PROT_MTE property\n".as_ptr());
            return KSFT_FAIL;
        }
        result = check_mte_memory(ptr, sizes[run], mode, TAG_CHECK_ON, atag_check, TAG_OP_ALL);
        mte_free_memory_tag_range(ptr as *mut c_void, sizes[run], mem_type, UNDERFLOW, OVERFLOW);
        if result != KSFT_PASS {
            return result;
        }

        fd = create_temp_file();
        if fd == -1 {
            return KSFT_FAIL;
        }
        ptr = mte_allocate_file_memory_tag_range(sizes[run], mem_type, mapping, UNDERFLOW, OVERFLOW, fd) as *mut c_char;
        if check_allocated_memory_range(ptr, sizes[run], mem_type, UNDERFLOW, OVERFLOW) != KSFT_PASS {
            close(fd);
            return KSFT_FAIL;
        }
        map_ptr = ptr.offset(-(UNDERFLOW as isize));
        /* Try to clear PROT_MTE property and verify it by tag checking */
        if mprotect(map_ptr as *mut c_void, map_size as usize, prot_flag) != 0 {
            ksft_print_msg(c"FAIL: mprotect not ignoring clear PROT_MTE property\n".as_ptr());
            mte_free_memory_tag_range(ptr as *mut c_void, sizes[run], mem_type, UNDERFLOW, OVERFLOW);
            close(fd);
            return KSFT_FAIL;
        }
        result = check_mte_memory(ptr, sizes[run], mode, TAG_CHECK_ON, atag_check, TAG_OP_ALL);
        mte_free_memory_tag_range(ptr as *mut c_void, sizes[run], mem_type, UNDERFLOW, OVERFLOW);
        close(fd);
        if result != KSFT_PASS {
            return result;
        }
    }
    KSFT_PASS
}

unsafe fn format_test_name(tc: *mut check_mmap_testcase) -> *const c_char {
    static mut TEST_NAME: [c_char; TEST_NAME_MAX] = [0; TEST_NAME_MAX];
    let check_type_str: *const c_char;
    let mem_type_str: *const c_char;
    let sync_str: *const c_char;
    let mapping_str: *const c_char;
    let tag_check_str: *const c_char;
    let atag_check_str: *const c_char;
    let tag_op_str: *const c_char;

    match (*tc).check_type {
        CHECK_ANON_MEM => check_type_str = c"anonymous memory".as_ptr(),
        CHECK_FILE_MEM => check_type_str = c"file memory".as_ptr(),
        CHECK_CLEAR_PROT_MTE => check_type_str = c"clear PROT_MTE flags".as_ptr(),
        _ => panic!("assert(0)"),
    }

    match (*tc).mem_type {
        USE_MMAP => mem_type_str = c"mmap".as_ptr(),
        USE_MPROTECT => mem_type_str = c"mmap/mprotect".as_ptr(),
        _ => panic!("assert(0)"),
    }

    match (*tc).mte_sync {
        MTE_NONE_ERR => sync_str = c"no error".as_ptr(),
        MTE_SYNC_ERR => sync_str = c"sync error".as_ptr(),
        MTE_ASYNC_ERR => sync_str = c"async error".as_ptr(),
        _ => panic!("assert(0)"),
    }

    match (*tc).mapping {
        MAP_SHARED => mapping_str = c"shared".as_ptr(),
        MAP_PRIVATE => mapping_str = c"private".as_ptr(),
        _ => panic!("assert(0)"),
    }

    match (*tc).tag_check {
        TAG_CHECK_ON => tag_check_str = c"tag check on".as_ptr(),
        TAG_CHECK_OFF => tag_check_str = c"tag check off".as_ptr(),
        _ => panic!("assert(0)"),
    }

    match (*tc).atag_check {
        ATAG_CHECK_ON => atag_check_str = c"with address tag [63:60]".as_ptr(),
        ATAG_CHECK_OFF => atag_check_str = c"without address tag [63:60]".as_ptr(),
        _ => panic!("assert(0)"),
    }

    snprintf(TEST_NAME.as_mut_ptr(), std::mem::size_of_val(&TEST_NAME),
             c"Check %s with %s mapping, %s mode, %s memory and %s (%s)\n".as_ptr(),
             check_type_str, mapping_str, sync_str, mem_type_str, tag_check_str, atag_check_str);

    match (*tc).tag_op {
        TAG_OP_ALL => tag_op_str = c"".as_ptr(),
        TAG_OP_STONLY => tag_op_str = c" / store-only".as_ptr(),
        _ => panic!("assert(0)"),
    }

    snprintf(TEST_NAME.as_mut_ptr(), TEST_NAME_MAX,
             c"Check %s with %s mapping, %s mode, %s memory and %s (%s%s)\n".as_ptr(),
             check_type_str, mapping_str, sync_str, mem_type_str, tag_check_str, atag_check_str, tag_op_str);

    TEST_NAME.as_ptr()
}

fn tc(check_type: c_int, mem_type: c_int, mte_sync: c_int, mapping: c_int,
      tag_check: c_int, atag_check: c_int, tag_op: c_int, enable_tco: bool) -> check_mmap_testcase {
    check_mmap_testcase { check_type, mem_type, mte_sync, mapping, tag_check, atag_check, tag_op, enable_tco }
}

fn main() {
    unsafe {
        let mut err: c_int;
        let item = sizes.len();
        let mut test_cases = [
            tc(CHECK_ANON_MEM, USE_MMAP, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_OFF, ATAG_CHECK_OFF, TAG_OP_ALL, true),
            tc(CHECK_FILE_MEM, USE_MPROTECT, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_OFF, ATAG_CHECK_OFF, TAG_OP_ALL, true),
            tc(CHECK_ANON_MEM, USE_MMAP, MTE_NONE_ERR, MAP_PRIVATE, TAG_CHECK_OFF, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_FILE_MEM, USE_MPROTECT, MTE_NONE_ERR, MAP_PRIVATE, TAG_CHECK_OFF, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_ANON_MEM, USE_MMAP, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_ANON_MEM, USE_MPROTECT, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_ANON_MEM, USE_MMAP, MTE_SYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_ANON_MEM, USE_MPROTECT, MTE_SYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_ANON_MEM, USE_MMAP, MTE_ASYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_ANON_MEM, USE_MPROTECT, MTE_ASYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_ANON_MEM, USE_MMAP, MTE_ASYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_ANON_MEM, USE_MPROTECT, MTE_ASYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_FILE_MEM, USE_MMAP, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_FILE_MEM, USE_MPROTECT, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_FILE_MEM, USE_MMAP, MTE_SYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_FILE_MEM, USE_MPROTECT, MTE_SYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_FILE_MEM, USE_MMAP, MTE_ASYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_FILE_MEM, USE_MPROTECT, MTE_ASYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_FILE_MEM, USE_MMAP, MTE_ASYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_FILE_MEM, USE_MPROTECT, MTE_ASYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_CLEAR_PROT_MTE, USE_MMAP, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_CLEAR_PROT_MTE, USE_MPROTECT, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_ALL, false),
            tc(CHECK_ANON_MEM, USE_MMAP, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_ALL, false),
            tc(CHECK_ANON_MEM, USE_MPROTECT, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_ALL, false),
            tc(CHECK_ANON_MEM, USE_MMAP, MTE_SYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_ALL, false),
            tc(CHECK_ANON_MEM, USE_MPROTECT, MTE_SYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_ALL, false),
            tc(CHECK_FILE_MEM, USE_MMAP, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_ALL, false),
            tc(CHECK_FILE_MEM, USE_MPROTECT, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_ALL, false),
            tc(CHECK_FILE_MEM, USE_MMAP, MTE_SYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_ALL, false),
            tc(CHECK_FILE_MEM, USE_MPROTECT, MTE_SYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_ALL, false),
            tc(CHECK_FILE_MEM, USE_MMAP, MTE_ASYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_ALL, false),
            tc(CHECK_ANON_MEM, USE_MMAP, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_STONLY, false),
            tc(CHECK_ANON_MEM, USE_MPROTECT, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_STONLY, false),
            tc(CHECK_ANON_MEM, USE_MMAP, MTE_SYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_STONLY, false),
            tc(CHECK_ANON_MEM, USE_MPROTECT, MTE_SYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_STONLY, false),
            tc(CHECK_ANON_MEM, USE_MMAP, MTE_ASYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_STONLY, false),
            tc(CHECK_ANON_MEM, USE_MPROTECT, MTE_ASYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_STONLY, false),
            tc(CHECK_ANON_MEM, USE_MMAP, MTE_ASYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_STONLY, false),
            tc(CHECK_ANON_MEM, USE_MPROTECT, MTE_ASYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_STONLY, false),
            tc(CHECK_FILE_MEM, USE_MMAP, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_STONLY, false),
            tc(CHECK_FILE_MEM, USE_MPROTECT, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_STONLY, false),
            tc(CHECK_FILE_MEM, USE_MMAP, MTE_SYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_STONLY, false),
            tc(CHECK_FILE_MEM, USE_MPROTECT, MTE_SYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_STONLY, false),
            tc(CHECK_FILE_MEM, USE_MMAP, MTE_ASYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_STONLY, false),
            tc(CHECK_FILE_MEM, USE_MPROTECT, MTE_ASYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_STONLY, false),
            tc(CHECK_FILE_MEM, USE_MMAP, MTE_ASYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_STONLY, false),
            tc(CHECK_FILE_MEM, USE_MPROTECT, MTE_ASYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_OFF, TAG_OP_STONLY, false),
            tc(CHECK_ANON_MEM, USE_MMAP, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_STONLY, false),
            tc(CHECK_ANON_MEM, USE_MPROTECT, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_STONLY, false),
            tc(CHECK_ANON_MEM, USE_MMAP, MTE_SYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_STONLY, false),
            tc(CHECK_ANON_MEM, USE_MPROTECT, MTE_SYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_STONLY, false),
            tc(CHECK_FILE_MEM, USE_MMAP, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_STONLY, false),
            tc(CHECK_FILE_MEM, USE_MPROTECT, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_STONLY, false),
            tc(CHECK_FILE_MEM, USE_MMAP, MTE_SYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_STONLY, false),
            tc(CHECK_FILE_MEM, USE_MPROTECT, MTE_SYNC_ERR, MAP_SHARED, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_STONLY, false),
            tc(CHECK_FILE_MEM, USE_MMAP, MTE_ASYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_STONLY, false),
            tc(CHECK_CLEAR_PROT_MTE, USE_MMAP, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_ALL, false),
            tc(CHECK_CLEAR_PROT_MTE, USE_MPROTECT, MTE_SYNC_ERR, MAP_PRIVATE, TAG_CHECK_ON, ATAG_CHECK_ON, TAG_OP_ALL, false),
        ];

        ksft_print_header();

        err = mte_default_setup();
        if err != 0 {
            std::process::exit(err);
        }
        page_size = getpagesize() as usize;
        if page_size == 0 {
            ksft_print_msg(c"ERR: Unable to get page size\n".as_ptr());
            std::process::exit(KSFT_FAIL);
        }
        sizes[item - 3] = page_size as c_int - 1;
        sizes[item - 2] = page_size as c_int;
        sizes[item - 1] = page_size as c_int + 1;

        /* Set test plan */
        ksft_set_plan(test_cases.len());

        for i in 0..test_cases.len() {
            /* Register signal handlers */
            mte_register_signal(SIGBUS, mte_default_handler, test_cases[i].atag_check == ATAG_CHECK_ON);
            mte_register_signal(SIGSEGV, mte_default_handler, test_cases[i].atag_check == ATAG_CHECK_ON);

            if test_cases[i].enable_tco {
                mte_enable_pstate_tco();
            } else {
                mte_disable_pstate_tco();
            }

            match test_cases[i].check_type {
                CHECK_ANON_MEM => evaluate_test(
                    check_anonymous_memory_mapping(test_cases[i].mem_type,
                                                   test_cases[i].mte_sync,
                                                   test_cases[i].mapping,
                                                   test_cases[i].tag_check,
                                                   test_cases[i].atag_check,
                                                   test_cases[i].tag_op),
                    format_test_name(&mut test_cases[i]),
                ),
                CHECK_FILE_MEM => evaluate_test(
                    check_file_memory_mapping(test_cases[i].mem_type,
                                              test_cases[i].mte_sync,
                                              test_cases[i].mapping,
                                              test_cases[i].tag_check,
                                              test_cases[i].atag_check,
                                              test_cases[i].tag_op),
                    format_test_name(&mut test_cases[i]),
                ),
                CHECK_CLEAR_PROT_MTE => evaluate_test(
                    check_clear_prot_mte_flag(test_cases[i].mem_type,
                                              test_cases[i].mte_sync,
                                              test_cases[i].mapping,
                                              test_cases[i].atag_check),
                    format_test_name(&mut test_cases[i]),
                ),
                _ => exit(KSFT_FAIL),
            }
        }

        mte_restore_setup();
        ksft_print_cnts();
        std::process::exit(if ksft_get_fail_cnt() == 0 { KSFT_PASS } else { KSFT_FAIL });
    }
}
