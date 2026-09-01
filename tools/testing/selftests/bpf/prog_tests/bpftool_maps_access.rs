// SPDX-License-Identifier: GPL-2.0-only

use core::ffi::{c_char, c_int, c_uint, c_void};

// C dependencies removed from executable Rust:
// stdlib.h, unistd.h, fcntl.h, stdint.h, sys/stat.h, stdbool.h,
// linux/bpf.h, bpf/libbpf.h, bpftool_helpers.h, test_progs.h,
// bpf/bpf.h, and "security_bpf_map.skel.h".

const PROTECTED_MAP_NAME: *const c_char = b"prot_map\0".as_ptr() as *const c_char;
const UNPROTECTED_MAP_NAME: *const c_char = b"not_prot_map\0".as_ptr() as *const c_char;
const BPF_ITER_FILE: *const c_char = b"bpf_iter_map_elem.bpf.o\0".as_ptr() as *const c_char;
const BPFFS_PIN_DIR: *const c_char =
    b"/sys/fs/bpf/test_bpftool_map\0".as_ptr() as *const c_char;
const INNER_MAP_NAME: *const c_char = b"inner_map_tt\0".as_ptr() as *const c_char;
const OUTER_MAP_NAME: *const c_char = b"outer_map_tt\0".as_ptr() as *const c_char;

const MAP_NAME_MAX_LEN: usize = 64;
const PATH_MAX_LEN: usize = 128;

const S_IFDIR: c_uint = 0o040000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum map_protection {
    PROTECTED,
    UNPROTECTED,
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct security_bpf_map_maps {
    prot_map: *mut bpf_map,
    not_prot_map: *mut bpf_map,
    prot_status_map: *mut bpf_map,
}

#[repr(C)]
struct security_bpf_map {
    maps: security_bpf_map_maps,
}

#[repr(C)]
struct test_desc {
    name: *mut c_char,
    protection: map_protection,
    map: *mut bpf_map,
    map_name: *mut c_char,
    pinned: bool,
    pin_path: [c_char; PATH_MAX_LEN],
    write_must_fail: bool,
}

unsafe extern "C" {
    static MAX_BPFTOOL_CMD_LEN: usize;

    fn security_bpf_map__open_and_load() -> *mut security_bpf_map;
    fn security_bpf_map__attach(skel: *mut security_bpf_map) -> c_int;
    fn security_bpf_map__destroy(skel: *mut security_bpf_map);

    fn bpf_map__update_elem(
        map: *mut bpf_map,
        key: *const c_void,
        key_sz: usize,
        value: *const c_void,
        value_sz: usize,
        flags: u64,
    ) -> c_int;
    fn bpf_map__pin(map: *mut bpf_map, path: *const c_char) -> c_int;
    fn bpf_map__unpin(map: *mut bpf_map, path: *const c_char) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(ret: c_int, val: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(ret: c_int, val: c_int, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn run_bpftool_command(cmd: *const c_char) -> c_int;

    fn mkdir(path: *const c_char, mode: c_uint) -> c_int;
    fn rmdir(path: *const c_char) -> c_int;
    fn unlink(path: *const c_char) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
}

unsafe fn general_setup() -> *mut security_bpf_map {
    let mut key: u32;
    let value: u32;
    let mut ret: c_int;
    let mut i: c_int;

    let skel = security_bpf_map__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"open and load skeleton\0".as_ptr() as *const c_char) {
        goto_end();
        return core::ptr::null_mut();
    }

    let maps: [*mut bpf_map; 2] = [(*skel).maps.prot_map, (*skel).maps.not_prot_map];

    ret = security_bpf_map__attach(skel);
    if !ASSERT_OK(ret, b"attach maps security programs\0".as_ptr() as *const c_char) {
        security_bpf_map__destroy(skel);
        return core::ptr::null_mut();
    }

    i = 0;
    while (i as usize) < core::mem::size_of_val(&maps) / core::mem::size_of::<*mut bpf_map>() {
        key = 0;
        while key < 2 {
            let ret = bpf_map__update_elem(
                maps[i as usize],
                &key as *const u32 as *const c_void,
                core::mem::size_of_val(&key),
                &key as *const u32 as *const c_void,
                core::mem::size_of_val(&key),
                0,
            );
            if !ASSERT_OK(ret, b"set initial map value\0".as_ptr() as *const c_char) {
                security_bpf_map__destroy(skel);
                return core::ptr::null_mut();
            }
            key += 1;
        }
        i += 1;
    }

    key = 0;
    value = 1;
    ret = bpf_map__update_elem(
        (*skel).maps.prot_status_map,
        &key as *const u32 as *const c_void,
        core::mem::size_of_val(&key),
        &value as *const u32 as *const c_void,
        core::mem::size_of_val(&value),
        0,
    );
    if !ASSERT_OK(ret, b"configure map protection\0".as_ptr() as *const c_char) {
        security_bpf_map__destroy(skel);
        return core::ptr::null_mut();
    }

    if !ASSERT_OK(
        mkdir(BPFFS_PIN_DIR, S_IFDIR),
        b"create bpffs pin dir\0".as_ptr() as *const c_char,
    ) {
        security_bpf_map__destroy(skel);
        return core::ptr::null_mut();
    }

    skel
}

unsafe fn goto_end() {}

unsafe fn general_cleanup(skel: *mut security_bpf_map) {
    rmdir(BPFFS_PIN_DIR);
    security_bpf_map__destroy(skel);
}

unsafe fn update_test_desc(skel: *mut security_bpf_map, test: *mut test_desc) {
    /* Now that the skeleton is loaded, update all missing fields to
     * have the subtest properly configured
     */
    if (*test).protection == map_protection::PROTECTED {
        (*test).map = (*skel).maps.prot_map;
        (*test).map_name = PROTECTED_MAP_NAME as *mut c_char;
    } else {
        (*test).map = (*skel).maps.not_prot_map;
        (*test).map_name = UNPROTECTED_MAP_NAME as *mut c_char;
    }
}

unsafe fn test_setup(skel: *mut security_bpf_map, desc: *mut test_desc) -> c_int {
    let mut ret: c_int;

    update_test_desc(skel, desc);

    if (*desc).pinned {
        ret = snprintf(
            (*desc).pin_path.as_mut_ptr(),
            PATH_MAX_LEN,
            b"%s/%s\0".as_ptr() as *const c_char,
            BPFFS_PIN_DIR,
            (*desc).name,
        );
        if !ASSERT_GT(ret, 0, b"format pin path\0".as_ptr() as *const c_char) {
            return 1;
        }
        ret = bpf_map__pin((*desc).map, (*desc).pin_path.as_ptr());
        if !ASSERT_OK(ret, b"pin map\0".as_ptr() as *const c_char) {
            return 1;
        }
    }

    0
}

unsafe fn test_cleanup(desc: *mut test_desc) {
    if (*desc).pinned {
        bpf_map__unpin((*desc).map, core::ptr::null());
    }
}

unsafe fn lookup_map_value(map_handle: *mut c_char) -> c_int {
    let mut cmd = vec![0 as c_char; MAX_BPFTOOL_CMD_LEN];
    let mut ret: c_int = 0;

    ret = snprintf(
        cmd.as_mut_ptr(),
        MAX_BPFTOOL_CMD_LEN,
        b"map lookup %s key 0 0 0 0\0".as_ptr() as *const c_char,
        map_handle,
    );
    if !ASSERT_GT(ret, 0, b"format map lookup cmd\0".as_ptr() as *const c_char) {
        return 1;
    }
    run_bpftool_command(cmd.as_ptr())
}

unsafe fn read_map_btf_data(map_handle: *mut c_char) -> c_int {
    let mut cmd = vec![0 as c_char; MAX_BPFTOOL_CMD_LEN];
    let mut ret: c_int = 0;

    ret = snprintf(
        cmd.as_mut_ptr(),
        MAX_BPFTOOL_CMD_LEN,
        b"btf dump map %s\0".as_ptr() as *const c_char,
        map_handle,
    );
    if !ASSERT_GT(ret, 0, b"format map btf dump cmd\0".as_ptr() as *const c_char) {
        return 1;
    }
    run_bpftool_command(cmd.as_ptr())
}

unsafe fn write_map_value(map_handle: *mut c_char) -> c_int {
    let mut cmd = vec![0 as c_char; MAX_BPFTOOL_CMD_LEN];
    let mut ret: c_int = 0;

    ret = snprintf(
        cmd.as_mut_ptr(),
        MAX_BPFTOOL_CMD_LEN,
        b"map update %s key 0 0 0 0 value 1 1 1 1\0".as_ptr() as *const c_char,
        map_handle,
    );
    if !ASSERT_GT(ret, 0, b"format value write cmd\0".as_ptr() as *const c_char) {
        return 1;
    }
    run_bpftool_command(cmd.as_ptr())
}

unsafe fn delete_map_value(map_handle: *mut c_char) -> c_int {
    let mut cmd = vec![0 as c_char; MAX_BPFTOOL_CMD_LEN];
    let mut ret: c_int = 0;

    ret = snprintf(
        cmd.as_mut_ptr(),
        MAX_BPFTOOL_CMD_LEN,
        b"map delete %s key 0 0 0 0\0".as_ptr() as *const c_char,
        map_handle,
    );
    if !ASSERT_GT(ret, 0, b"format value deletion cmd\0".as_ptr() as *const c_char) {
        return 1;
    }
    run_bpftool_command(cmd.as_ptr())
}

unsafe fn iterate_on_map_values(map_handle: *mut c_char, iter_pin_path: *mut c_char) -> c_int {
    let mut cmd = vec![0 as c_char; MAX_BPFTOOL_CMD_LEN];
    let mut ret: c_int = 0;

    ret = snprintf(
        cmd.as_mut_ptr(),
        MAX_BPFTOOL_CMD_LEN,
        b"iter pin %s %s map %s\0".as_ptr() as *const c_char,
        BPF_ITER_FILE,
        iter_pin_path,
        map_handle,
    );
    if !ASSERT_GT(ret, 0, b"format iterator creation cmd\0".as_ptr() as *const c_char) {
        return 1;
    }
    ret = run_bpftool_command(cmd.as_ptr());
    if ret != 0 {
        return ret;
    }
    ret = snprintf(
        cmd.as_mut_ptr(),
        MAP_NAME_MAX_LEN,
        b"cat %s\0".as_ptr() as *const c_char,
        iter_pin_path,
    );
    if ret < 0 {
        unlink(iter_pin_path);
        return ret;
    }
    ret = system(cmd.as_ptr());

    unlink(iter_pin_path);
    ret
}

unsafe fn create_inner_map() -> c_int {
    let mut cmd = vec![0 as c_char; MAX_BPFTOOL_CMD_LEN];
    let mut ret: c_int = 0;

    ret = snprintf(
        cmd.as_mut_ptr(),
        MAX_BPFTOOL_CMD_LEN,
        b"map create %s/%s type array key 4 value 4 entries 4 name %s\0".as_ptr()
            as *const c_char,
        BPFFS_PIN_DIR,
        INNER_MAP_NAME,
        INNER_MAP_NAME,
    );
    if !ASSERT_GT(ret, 0, b"format inner map create cmd\0".as_ptr() as *const c_char) {
        return 1;
    }
    run_bpftool_command(cmd.as_ptr())
}

unsafe fn create_outer_map() -> c_int {
    let mut cmd = vec![0 as c_char; MAX_BPFTOOL_CMD_LEN];
    let mut ret: c_int = 0;

    ret = snprintf(
        cmd.as_mut_ptr(),
        MAX_BPFTOOL_CMD_LEN,
        b"map create %s/%s type hash_of_maps key 4 value 4 entries 2 name %s inner_map name %s\0"
            .as_ptr() as *const c_char,
        BPFFS_PIN_DIR,
        OUTER_MAP_NAME,
        OUTER_MAP_NAME,
        INNER_MAP_NAME,
    );
    if !ASSERT_GT(ret, 0, b"format outer map create cmd\0".as_ptr() as *const c_char) {
        return 1;
    }
    run_bpftool_command(cmd.as_ptr())
}

unsafe fn delete_pinned_map(map_name: *const c_char) {
    let mut pin_path = [0 as c_char; PATH_MAX_LEN];
    let ret: c_int;

    ret = snprintf(
        pin_path.as_mut_ptr(),
        PATH_MAX_LEN,
        b"%s/%s\0".as_ptr() as *const c_char,
        BPFFS_PIN_DIR,
        map_name,
    );
    if ret >= 0 {
        unlink(pin_path.as_ptr());
    }
}

unsafe fn add_outer_map_entry(key: c_int) -> c_int {
    let mut cmd = vec![0 as c_char; MAX_BPFTOOL_CMD_LEN];
    let mut ret: c_int = 0;

    ret = snprintf(
        cmd.as_mut_ptr(),
        MAX_BPFTOOL_CMD_LEN,
        b"map update pinned %s/%s key %d 0 0 0 value name %s\0".as_ptr() as *const c_char,
        BPFFS_PIN_DIR,
        OUTER_MAP_NAME,
        key,
        INNER_MAP_NAME,
    );
    if !ASSERT_GT(
        ret,
        0,
        b"format outer map value addition cmd\0".as_ptr() as *const c_char,
    ) {
        return 1;
    }
    run_bpftool_command(cmd.as_ptr())
}

unsafe fn test_basic_access(desc: *mut test_desc) {
    let mut map_handle = [0 as c_char; MAP_NAME_MAX_LEN];
    let mut iter_pin_path = [0 as c_char; PATH_MAX_LEN];
    let mut ret: c_int;

    if (*desc).pinned {
        ret = snprintf(
            map_handle.as_mut_ptr(),
            MAP_NAME_MAX_LEN,
            b"pinned %s\0".as_ptr() as *const c_char,
            (*desc).pin_path.as_ptr(),
        );
    } else {
        ret = snprintf(
            map_handle.as_mut_ptr(),
            MAP_NAME_MAX_LEN,
            b"name %s\0".as_ptr() as *const c_char,
            (*desc).map_name,
        );
    }
    if !ASSERT_GT(ret, 0, b"format map handle\0".as_ptr() as *const c_char) {
        return;
    }

    ret = lookup_map_value(map_handle.as_mut_ptr());
    ASSERT_OK(ret, b"read map value\0".as_ptr() as *const c_char);

    ret = read_map_btf_data(map_handle.as_mut_ptr());
    ASSERT_OK(ret, b"read map btf data\0".as_ptr() as *const c_char);

    ret = write_map_value(map_handle.as_mut_ptr());
    ASSERT_OK(
        if (*desc).write_must_fail {
            (ret == 0) as c_int
        } else {
            ret
        },
        b"write map value\0".as_ptr() as *const c_char,
    );

    ret = delete_map_value(map_handle.as_mut_ptr());
    ASSERT_OK(
        if (*desc).write_must_fail {
            (ret == 0) as c_int
        } else {
            ret
        },
        b"delete map value\0".as_ptr() as *const c_char,
    );
    /* Restore deleted value */
    if ret == 0 {
        write_map_value(map_handle.as_mut_ptr());
    }

    ret = snprintf(
        iter_pin_path.as_mut_ptr(),
        PATH_MAX_LEN,
        b"%s/iter\0".as_ptr() as *const c_char,
        BPFFS_PIN_DIR,
    );
    if ASSERT_GT(ret, 0, b"format iter pin path\0".as_ptr() as *const c_char) {
        ret = iterate_on_map_values(map_handle.as_mut_ptr(), iter_pin_path.as_mut_ptr());
        ASSERT_OK(ret, b"iterate on map values\0".as_ptr() as *const c_char);
    }
}

unsafe fn test_create_nested_maps() {
    if !ASSERT_OK(create_inner_map(), b"create inner map\0".as_ptr() as *const c_char) {
        return;
    }
    if !ASSERT_OK(create_outer_map(), b"create outer map\0".as_ptr() as *const c_char) {
        delete_pinned_map(INNER_MAP_NAME);
        return;
    }
    ASSERT_OK(
        add_outer_map_entry(0),
        b"add a first entry in outer map\0".as_ptr() as *const c_char,
    );
    ASSERT_OK(
        add_outer_map_entry(1),
        b"add a second entry in outer map\0".as_ptr() as *const c_char,
    );
    ASSERT_NEQ(
        add_outer_map_entry(2),
        0,
        b"add a third entry in outer map\0".as_ptr() as *const c_char,
    );

    delete_pinned_map(OUTER_MAP_NAME);
    delete_pinned_map(INNER_MAP_NAME);
}

unsafe fn test_btf_list() {
    ASSERT_OK(
        run_bpftool_command(b"btf list\0".as_ptr() as *const c_char),
        b"list btf data\0".as_ptr() as *const c_char,
    );
}

static mut tests: [test_desc; 4] = [
    test_desc {
        name: b"unprotected_unpinned\0".as_ptr() as *mut c_char,
        protection: map_protection::UNPROTECTED,
        map: core::ptr::null_mut(),
        map_name: UNPROTECTED_MAP_NAME as *mut c_char,
        pinned: false,
        pin_path: [0 as c_char; PATH_MAX_LEN],
        write_must_fail: false,
    },
    test_desc {
        name: b"unprotected_pinned\0".as_ptr() as *mut c_char,
        protection: map_protection::UNPROTECTED,
        map: core::ptr::null_mut(),
        map_name: UNPROTECTED_MAP_NAME as *mut c_char,
        pinned: true,
        pin_path: [0 as c_char; PATH_MAX_LEN],
        write_must_fail: false,
    },
    test_desc {
        name: b"protected_unpinned\0".as_ptr() as *mut c_char,
        protection: map_protection::PROTECTED,
        map: core::ptr::null_mut(),
        map_name: UNPROTECTED_MAP_NAME as *mut c_char,
        pinned: false,
        pin_path: [0 as c_char; PATH_MAX_LEN],
        write_must_fail: true,
    },
    test_desc {
        name: b"protected_pinned\0".as_ptr() as *mut c_char,
        protection: map_protection::PROTECTED,
        map: core::ptr::null_mut(),
        map_name: UNPROTECTED_MAP_NAME as *mut c_char,
        pinned: true,
        pin_path: [0 as c_char; PATH_MAX_LEN],
        write_must_fail: true,
    },
];

static tests_count: usize = 4;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_bpftool_maps_access() {
    let skel: *mut security_bpf_map;
    let mut current: *mut test_desc;
    let mut i: c_int;

    skel = general_setup();
    if !ASSERT_OK_PTR(skel as *const c_void, b"prepare programs\0".as_ptr() as *const c_char) {
        general_cleanup(skel);
        return;
    }

    i = 0;
    while (i as usize) < tests_count {
        current = tests.as_mut_ptr().add(i as usize);
        if !test__start_subtest((*current).name) {
            i += 1;
            continue;
        }
        if ASSERT_OK(test_setup(skel, current), b"subtest setup\0".as_ptr() as *const c_char) {
            test_basic_access(current);
            test_cleanup(current);
        }
        i += 1;
    }
    if test__start_subtest(b"nested_maps\0".as_ptr() as *const c_char) {
        test_create_nested_maps();
    }
    if test__start_subtest(b"btf_list\0".as_ptr() as *const c_char) {
        test_btf_list();
    }

    general_cleanup(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
