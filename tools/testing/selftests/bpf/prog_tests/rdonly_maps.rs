// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/bpf/prog_tests/rdonly_maps.c
// Depends on declarations normally provided by test_progs.h/libbpf.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
struct bss {
    did_run: c_uint,
    iters: c_uint,
    sum: c_uint,
}

#[repr(C)]
struct rdonly_map_subtest {
    subtest_name: *const c_char,
    prog_name: *const c_char,
    exp_iters: c_uint,
    exp_sum: c_uint,
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn bpf_object__open_file(
        path: *const c_char,
        opts: *const c_void,
    ) -> *mut bpf_object;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__find_map_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_map;
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_object__close(obj: *mut bpf_object);

    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: c_ulong,
    ) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;

    fn bpf_program__attach_raw_tracepoint(
        prog: *mut bpf_program,
        tp_name: *const c_char,
    ) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);

    fn test__start_subtest(name: *const c_char) -> bool;
    fn usleep(usec: c_uint) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn CHECK(condition: c_int, name: *const c_char, fmt: *const c_char, ...) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_rdonly_maps() {
    let file: *const c_char = c"test_rdonly_maps.bpf.o".as_ptr();
    let subtests = [
        rdonly_map_subtest {
            subtest_name: c"skip loop".as_ptr(),
            prog_name: c"skip_loop".as_ptr(),
            exp_iters: 0,
            exp_sum: 0,
        },
        rdonly_map_subtest {
            subtest_name: c"part loop".as_ptr(),
            prog_name: c"part_loop".as_ptr(),
            exp_iters: 3,
            exp_sum: 2 + 3 + 4,
        },
        rdonly_map_subtest {
            subtest_name: c"full loop".as_ptr(),
            prog_name: c"full_loop".as_ptr(),
            exp_iters: 4,
            exp_sum: 2 + 3 + 4 + 5,
        },
    ];
    let mut i: c_int;
    let mut err: c_int;
    let zero: c_int = 0;
    let mut duration: c_int = 0;
    let mut link: *mut bpf_link = ptr::null_mut();
    let mut prog: *mut bpf_program;
    let mut bss_map: *mut bpf_map;
    let mut obj: *mut bpf_object;
    let mut bss: bss = bss {
        did_run: 0,
        iters: 0,
        sum: 0,
    };

    obj = bpf_object__open_file(file, ptr::null());
    if !ASSERT_OK_PTR(obj as *const c_void, c"obj_open".as_ptr()) {
        return;
    }

    err = bpf_object__load(obj);
    if CHECK(
        err,
        c"obj_load".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno,
    ) != 0
    {
        goto_cleanup(link, obj);
        return;
    }

    bss_map = bpf_object__find_map_by_name(obj, c".bss".as_ptr());
    if CHECK(
        (bss_map.is_null()) as c_int,
        c"find_bss_map".as_ptr(),
        c"failed\n".as_ptr(),
    ) != 0
    {
        goto_cleanup(link, obj);
        return;
    }

    i = 0;
    while (i as usize) < subtests.len() {
        let t: *const rdonly_map_subtest = &subtests[i as usize];

        if !test__start_subtest((*t).subtest_name) {
            i += 1;
            continue;
        }

        prog = bpf_object__find_program_by_name(obj, (*t).prog_name);
        if CHECK(
            (prog.is_null()) as c_int,
            c"find_prog".as_ptr(),
            c"prog '%s' not found\n".as_ptr(),
            (*t).prog_name,
        ) != 0
        {
            goto_cleanup(link, obj);
            return;
        }

        ptr::write_bytes(&mut bss as *mut bss, 0, 1);
        err = bpf_map_update_elem(
            bpf_map__fd(bss_map),
            &zero as *const c_int as *const c_void,
            &bss as *const bss as *const c_void,
            0,
        );
        if CHECK(
            err,
            c"set_bss".as_ptr(),
            c"failed to set bss data: %d\n".as_ptr(),
            err,
        ) != 0
        {
            goto_cleanup(link, obj);
            return;
        }

        link = bpf_program__attach_raw_tracepoint(prog, c"sys_enter".as_ptr());
        if !ASSERT_OK_PTR(link as *const c_void, c"attach_prog".as_ptr()) {
            goto_cleanup(link, obj);
            return;
        }

        /* trigger probe */
        usleep(1);

        bpf_link__destroy(link);
        link = ptr::null_mut();

        err = bpf_map_lookup_elem(
            bpf_map__fd(bss_map),
            &zero as *const c_int as *const c_void,
            &mut bss as *mut bss as *mut c_void,
        );
        if CHECK(
            err,
            c"get_bss".as_ptr(),
            c"failed to get bss data: %d\n".as_ptr(),
            err,
        ) != 0
        {
            goto_cleanup(link, obj);
            return;
        }
        if CHECK(
            (bss.did_run == 0) as c_int,
            c"check_run".as_ptr(),
            c"prog '%s' didn't run?\n".as_ptr(),
            (*t).prog_name,
        ) != 0
        {
            goto_cleanup(link, obj);
            return;
        }
        if CHECK(
            (bss.iters != (*t).exp_iters) as c_int,
            c"check_iters".as_ptr(),
            c"prog '%s' iters: %d, expected: %d\n".as_ptr(),
            (*t).prog_name,
            bss.iters,
            (*t).exp_iters,
        ) != 0
        {
            goto_cleanup(link, obj);
            return;
        }
        if CHECK(
            (bss.sum != (*t).exp_sum) as c_int,
            c"check_sum".as_ptr(),
            c"prog '%s' sum: %d, expected: %d\n".as_ptr(),
            (*t).prog_name,
            bss.sum,
            (*t).exp_sum,
        ) != 0
        {
            goto_cleanup(link, obj);
            return;
        }

        i += 1;
    }

    goto_cleanup(link, obj);

    let _ = size_of::<bss>();
    let _ = &mut duration;
}

unsafe fn goto_cleanup(link: *mut bpf_link, obj: *mut bpf_object) {
    bpf_link__destroy(link);
    bpf_object__close(obj);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
