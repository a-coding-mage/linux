// SPDX-License-Identifier: GPL-2.0

// C dependencies from:
// <sys/types.h>, <sys/stat.h>, <unistd.h>, <test_progs.h>

use core::ffi::{c_char, c_int, c_void};

pub type __u32 = u32;
pub type __u64 = u64;

pub const EINVAL: c_int = 22;
pub const ENOENT: c_int = 2;
pub const BPF_MAP_TYPE_ARRAY: c_int = 2;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_info {
    pub id: __u32,
    _rest: [u8; 0],
}

#[repr(C)]
pub struct stat {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object_open_opts {
    pub sz: usize,
    pub pin_root_path: *const c_char,
}

extern "C" {
    static mut errno: c_int;

    fn bpf_object__find_map_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_map;
    fn bpf_map_get_info_by_fd(
        fd: c_int,
        info: *mut bpf_map_info,
        info_len: *mut __u32,
    ) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_object__open_file(
        path: *const c_char,
        opts: *const bpf_object_open_opts,
    ) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const c_void) -> c_int;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);
    fn stat(path: *const c_char, buf: *mut stat) -> c_int;
    fn bpf_map__pin(map: *mut bpf_map, path: *const c_char) -> c_int;
    fn bpf_object__unpin_maps(obj: *mut bpf_object, path: *const c_char) -> c_int;
    fn bpf_object__pin_maps(obj: *mut bpf_object, path: *const c_char) -> c_int;
    fn bpf_map__pin_path(map: *mut bpf_map) -> *const c_char;
    fn bpf_map__set_pin_path(map: *mut bpf_map, path: *const c_char) -> c_int;
    fn bpf_map__name(map: *mut bpf_map) -> *const c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn unlink(path: *const c_char) -> c_int;
    fn rmdir(path: *const c_char) -> c_int;
    fn bpf_map_create(
        map_type: c_int,
        map_name: *const c_char,
        key_size: __u32,
        value_size: __u32,
        max_entries: __u32,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_map__reuse_fd(map: *mut bpf_map, fd: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
}

unsafe fn get_map_id(obj: *mut bpf_object, name: *const c_char) -> __u32 {
    let mut map_info: bpf_map_info = core::mem::zeroed();
    let mut map_info_len: __u32;
    let mut duration: __u32 = 0;
    let mut map: *mut bpf_map;
    let mut err: c_int;

    map_info_len = core::mem::size_of_val(&map_info) as __u32;

    map = bpf_object__find_map_by_name(obj, name);
    if CHECK!(
        map.is_null(),
        c"find map".as_ptr(),
        c"NULL map".as_ptr()
    ) {
        return 0;
    }

    err = bpf_map_get_info_by_fd(bpf_map__fd(map), &mut map_info, &mut map_info_len);
    CHECK!(
        err != 0,
        c"get map info".as_ptr(),
        c"err %d errno %d".as_ptr(),
        err,
        errno
    );
    map_info.id
}

unsafe fn test_pinning() {
    let file_invalid: *const c_char = c"./test_pinning_invalid.bpf.o".as_ptr();
    let custpinpath: *const c_char = c"/sys/fs/bpf/custom/pinmap".as_ptr();
    let nopinpath: *const c_char = c"/sys/fs/bpf/nopinmap".as_ptr();
    let nopinpath2: *const c_char = c"/sys/fs/bpf/nopinmap2".as_ptr();
    let custpath: *const c_char = c"/sys/fs/bpf/custom".as_ptr();
    let pinpath: *const c_char = c"/sys/fs/bpf/pinmap".as_ptr();
    let file: *const c_char = c"./test_pinning.bpf.o".as_ptr();
    let mut map_id: __u32;
    let mut map_id2: __u32;
    let mut duration: __u32 = 0;
    let mut statbuf: stat = core::mem::zeroed();
    let mut obj: *mut bpf_object;
    let mut map: *mut bpf_map;
    let mut err: c_int;
    let mut map_fd: c_int = 0;
    let opts = bpf_object_open_opts {
        sz: core::mem::size_of::<bpf_object_open_opts>(),
        pin_root_path: custpath,
    };

    /* check that opening fails with invalid pinning value in map def */
    obj = bpf_object__open_file(file_invalid, core::ptr::null());
    err = libbpf_get_error(obj as *const c_void);
    if CHECK!(
        err != -EINVAL,
        c"invalid open".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        obj = core::ptr::null_mut();
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* open the valid object file  */
    obj = bpf_object__open_file(file, core::ptr::null());
    err = libbpf_get_error(obj as *const c_void);
    if CHECK!(
        err != 0,
        c"default open".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        obj = core::ptr::null_mut();
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    err = bpf_object__load(obj);
    if CHECK!(
        err != 0,
        c"default load".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* check that pinmap was pinned */
    err = stat(pinpath, &mut statbuf);
    if CHECK!(
        err != 0,
        c"stat pinpath".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* check that nopinmap was *not* pinned */
    err = stat(nopinpath, &mut statbuf);
    if CHECK!(
        err == 0 || errno != ENOENT,
        c"stat nopinpath".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* check that nopinmap2 was *not* pinned */
    err = stat(nopinpath2, &mut statbuf);
    if CHECK!(
        err == 0 || errno != ENOENT,
        c"stat nopinpath2".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    map_id = get_map_id(obj, c"pinmap".as_ptr());
    if map_id == 0 {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    bpf_object__close(obj);

    obj = bpf_object__open_file(file, core::ptr::null());
    if CHECK_FAIL!(libbpf_get_error(obj as *const c_void)) {
        obj = core::ptr::null_mut();
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    err = bpf_object__load(obj);
    if CHECK!(
        err != 0,
        c"default load".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* check that same map ID was reused for second load */
    map_id2 = get_map_id(obj, c"pinmap".as_ptr());
    if CHECK!(
        map_id != map_id2,
        c"check reuse".as_ptr(),
        c"err %d errno %d id %d id2 %d\n".as_ptr(),
        err,
        errno,
        map_id,
        map_id2
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* should be no-op to re-pin same map */
    map = bpf_object__find_map_by_name(obj, c"pinmap".as_ptr());
    if CHECK!(map.is_null(), c"find map".as_ptr(), c"NULL map".as_ptr()) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    err = bpf_map__pin(map, core::ptr::null());
    if CHECK!(
        err != 0,
        c"re-pin map".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* but error to pin at different location */
    err = bpf_map__pin(map, c"/sys/fs/bpf/other".as_ptr());
    if CHECK!(
        err == 0,
        c"pin map different".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* unpin maps with a pin_path set */
    err = bpf_object__unpin_maps(obj, core::ptr::null());
    if CHECK!(
        err != 0,
        c"unpin maps".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* and re-pin them... */
    err = bpf_object__pin_maps(obj, core::ptr::null());
    if CHECK!(
        err != 0,
        c"pin maps".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* get pinning path */
    if !ASSERT_STREQ!(bpf_map__pin_path(map), pinpath, c"get pin path".as_ptr()) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* set pinning path of other map and re-pin all */
    map = bpf_object__find_map_by_name(obj, c"nopinmap".as_ptr());
    if CHECK!(map.is_null(), c"find map".as_ptr(), c"NULL map".as_ptr()) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    err = bpf_map__set_pin_path(map, custpinpath);
    if CHECK!(
        err != 0,
        c"set pin path".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* get pinning path after set */
    if !ASSERT_STREQ!(
        bpf_map__pin_path(map),
        custpinpath,
        c"get pin path after set".as_ptr()
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* should only pin the one unpinned map */
    err = bpf_object__pin_maps(obj, core::ptr::null());
    if CHECK!(
        err != 0,
        c"pin maps".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* check that nopinmap was pinned at the custom path */
    err = stat(custpinpath, &mut statbuf);
    if CHECK!(
        err != 0,
        c"stat custpinpath".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* remove the custom pin path to re-test it with auto-pinning below */
    err = unlink(custpinpath);
    if CHECK!(
        err != 0,
        c"unlink custpinpath".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    err = rmdir(custpath);
    if CHECK!(
        err != 0,
        c"rmdir custpindir".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    bpf_object__close(obj);

    /* open the valid object file again */
    obj = bpf_object__open_file(file, core::ptr::null());
    err = libbpf_get_error(obj as *const c_void);
    if CHECK!(
        err != 0,
        c"default open".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        obj = core::ptr::null_mut();
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* set pin paths so that nopinmap2 will attempt to reuse the map at
     * pinpath (which will fail), but not before pinmap has already been
     * reused
     */
    bpf_object__for_each_map!(map, obj, {
        if strcmp(bpf_map__name(map), c"nopinmap".as_ptr()) == 0 {
            err = bpf_map__set_pin_path(map, nopinpath2);
        } else if strcmp(bpf_map__name(map), c"nopinmap2".as_ptr()) == 0 {
            err = bpf_map__set_pin_path(map, pinpath);
        } else {
            continue;
        }

        if CHECK!(
            err != 0,
            c"set pin path".as_ptr(),
            c"err %d errno %d\n".as_ptr(),
            err,
            errno
        ) {
            goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
            return;
        }
    });

    /* should fail because of map parameter mismatch */
    err = bpf_object__load(obj);
    if CHECK!(
        err != -EINVAL,
        c"param mismatch load".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* nopinmap2 should have been pinned and cleaned up again */
    err = stat(nopinpath2, &mut statbuf);
    if CHECK!(
        err == 0 || errno != ENOENT,
        c"stat nopinpath2".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* pinmap should still be there */
    err = stat(pinpath, &mut statbuf);
    if CHECK!(
        err != 0,
        c"stat pinpath".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    bpf_object__close(obj);

    /* test auto-pinning at custom path with open opt */
    obj = bpf_object__open_file(file, &opts);
    if CHECK_FAIL!(libbpf_get_error(obj as *const c_void)) {
        obj = core::ptr::null_mut();
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    err = bpf_object__load(obj);
    if CHECK!(
        err != 0,
        c"custom load".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* check that pinmap was pinned at the custom path */
    err = stat(custpinpath, &mut statbuf);
    if CHECK!(
        err != 0,
        c"stat custpinpath".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* remove the custom pin path to re-test it with reuse fd below */
    err = unlink(custpinpath);
    if CHECK!(
        err != 0,
        c"unlink custpinpath".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    err = rmdir(custpath);
    if CHECK!(
        err != 0,
        c"rmdir custpindir".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    bpf_object__close(obj);

    /* test pinning at custom path with reuse fd */
    obj = bpf_object__open_file(file, core::ptr::null());
    err = libbpf_get_error(obj as *const c_void);
    if CHECK!(
        err != 0,
        c"default open".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        obj = core::ptr::null_mut();
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    map_fd = bpf_map_create(
        BPF_MAP_TYPE_ARRAY,
        core::ptr::null(),
        core::mem::size_of::<__u32>() as __u32,
        core::mem::size_of::<__u64>() as __u32,
        1,
        core::ptr::null(),
    );
    if CHECK!(
        map_fd < 0,
        c"create pinmap manually".as_ptr(),
        c"fd %d\n".as_ptr(),
        map_fd
    ) {
        goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    map = bpf_object__find_map_by_name(obj, c"pinmap".as_ptr());
    if CHECK!(map.is_null(), c"find map".as_ptr(), c"NULL map".as_ptr()) {
        goto_close_map_fd(map_fd, obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    err = bpf_map__reuse_fd(map, map_fd);
    if CHECK!(
        err != 0,
        c"reuse pinmap fd".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_close_map_fd(map_fd, obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    err = bpf_map__set_pin_path(map, custpinpath);
    if CHECK!(
        err != 0,
        c"set pin path".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_close_map_fd(map_fd, obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    err = bpf_object__load(obj);
    if CHECK!(
        err != 0,
        c"custom load".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_close_map_fd(map_fd, obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    /* check that pinmap was pinned at the custom path */
    err = stat(custpinpath, &mut statbuf);
    if CHECK!(
        err != 0,
        c"stat custpinpath".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno
    ) {
        goto_close_map_fd(map_fd, obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
        return;
    }

    close(map_fd);
    goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
}

unsafe fn goto_close_map_fd(
    map_fd: c_int,
    obj: *mut bpf_object,
    pinpath: *const c_char,
    nopinpath: *const c_char,
    nopinpath2: *const c_char,
    custpinpath: *const c_char,
    custpath: *const c_char,
) {
    close(map_fd);
    goto_out(obj, pinpath, nopinpath, nopinpath2, custpinpath, custpath);
}

unsafe fn goto_out(
    obj: *mut bpf_object,
    pinpath: *const c_char,
    nopinpath: *const c_char,
    nopinpath2: *const c_char,
    custpinpath: *const c_char,
    custpath: *const c_char,
) {
    unlink(pinpath);
    unlink(nopinpath);
    unlink(nopinpath2);
    unlink(custpinpath);
    rmdir(custpath);
    if !obj.is_null() {
        bpf_object__close(obj);
    }
}
