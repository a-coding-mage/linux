// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/util/vdso.c. C include dependencies are intentionally
// left as external declarations or comments because they are supplied elsewhere.

use core::ffi::{c_char, c_int, c_void};

const VDSO__TEMP_FILE_NAME: &[u8] = b"/tmp/perf-vdso.so-XXXXXX\0";
const VDSO__TEMP_FILE_NAME_LEN: usize = VDSO__TEMP_FILE_NAME.len();

#[repr(C)]
pub struct vdso_file {
    found: bool,
    error: bool,
    temp_file_name: [c_char; VDSO__TEMP_FILE_NAME_LEN],
    dso_name: *const c_char,
    read_prog: *const c_char,
}

#[repr(C)]
pub struct vdso_info {
    vdso: vdso_file,
    // Present in the C source only when BITS_PER_LONG == 64.
    #[cfg(target_pointer_width = "64")]
    vdso32: vdso_file,
    #[cfg(target_pointer_width = "64")]
    vdsox32: vdso_file,
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dso_type {
    DSO__TYPE_UNKNOWN,
    DSO__TYPE_64BIT,
    DSO__TYPE_32BIT,
    DSO__TYPE_X32BIT,
}

#[repr(C)]
struct machine__thread_dso_type_maps_cb_args {
    machine: *mut machine,
    dso_type: dso_type,
}

unsafe extern "C" {
    static DSO__NAME_VDSO: *const c_char;
    static DSO__NAME_VDSO32: *const c_char;
    static DSO__NAME_VDSOX32: *const c_char;
    static VDSO__MAP_NAME: *const c_char;

    fn memdup(src: *const c_void, len: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn mkstemp(template: *mut c_char) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn __errno_location() -> *mut c_int;

    fn find_map(start: *mut *mut c_void, end: *mut *mut c_void, name: *const c_char) -> c_int;
    fn zfree(ptr: *mut *mut c_void);
    fn dso__new(name: *const c_char) -> *mut dso;
    fn __dsos__add(dsos: *mut c_void, dso: *mut dso);
    fn dso__set_long_name(dso: *mut dso, name: *const c_char, name_allocated: bool);
    fn map__dso(map: *mut map) -> *mut dso;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn dso__type(dso: *mut dso, machine: *mut machine) -> dso_type;
    fn thread__maps(thread: *mut thread) -> *mut c_void;
    fn maps__for_each_map(
        maps: *mut c_void,
        cb: Option<unsafe extern "C" fn(*mut map, *mut c_void) -> c_int>,
        data: *mut c_void,
    );
    fn dsos__find(dsos: *mut c_void, name: *const c_char, cmp_short: bool) -> *mut dso;
    fn dso__put(dso: *mut dso);
    fn dso__short_name(dso: *mut dso) -> *const c_char;

    #[cfg(target_pointer_width = "64")]
    fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
    #[cfg(target_pointer_width = "64")]
    fn ferror(stream: *mut FILE) -> c_int;
    #[cfg(target_pointer_width = "64")]
    fn feof(stream: *mut FILE) -> c_int;
    #[cfg(target_pointer_width = "64")]
    fn writen(fd: c_int, buf: *const c_void, n: usize) -> isize;
    #[cfg(target_pointer_width = "64")]
    fn popen(command: *const c_char, type_: *const c_char) -> *mut FILE;
    #[cfg(target_pointer_width = "64")]
    fn pclose(stream: *mut FILE) -> c_int;
    #[cfg(target_pointer_width = "64")]
    fn pr_err(fmt: *const c_char, ...) -> c_int;
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe fn machine_vdso_info(machine: *mut machine) -> *mut *mut vdso_info {
    machine.cast::<*mut vdso_info>()
}

unsafe fn machine_dsos(machine: *mut machine) -> *mut c_void {
    machine.cast::<c_void>()
}

fn vdso_temp_file_name_array() -> [c_char; VDSO__TEMP_FILE_NAME_LEN] {
    let mut out = [0 as c_char; VDSO__TEMP_FILE_NAME_LEN];
    let mut i = 0;

    while i < VDSO__TEMP_FILE_NAME_LEN {
        out[i] = VDSO__TEMP_FILE_NAME[i] as c_char;
        i += 1;
    }

    out
}

unsafe fn vdso_info__new() -> *mut vdso_info {
    let vdso_info_init = vdso_info {
        vdso: vdso_file {
            found: false,
            error: false,
            temp_file_name: vdso_temp_file_name_array(),
            dso_name: DSO__NAME_VDSO,
            read_prog: core::ptr::null(),
        },
        #[cfg(target_pointer_width = "64")]
        vdso32: vdso_file {
            found: false,
            error: false,
            temp_file_name: vdso_temp_file_name_array(),
            dso_name: DSO__NAME_VDSO32,
            read_prog: c"perf-read-vdso32".as_ptr(),
        },
        #[cfg(target_pointer_width = "64")]
        vdsox32: vdso_file {
            found: false,
            error: false,
            temp_file_name: vdso_temp_file_name_array(),
            dso_name: DSO__NAME_VDSOX32,
            read_prog: c"perf-read-vdsox32".as_ptr(),
        },
    };

    memdup(
        (&vdso_info_init as *const vdso_info).cast::<c_void>(),
        core::mem::size_of_val(&vdso_info_init),
    )
    .cast::<vdso_info>()
}

unsafe fn get_file(vdso_file: *mut vdso_file) -> *mut c_char {
    let mut vdso: *mut c_char = core::ptr::null_mut();
    let mut buf: *mut c_char;
    let mut start: *mut c_void = core::ptr::null_mut();
    let mut end: *mut c_void = core::ptr::null_mut();
    let size: usize;
    let fd: c_int;

    if (*vdso_file).found {
        return (*vdso_file).temp_file_name.as_mut_ptr();
    }

    if (*vdso_file).error || find_map(&mut start, &mut end, VDSO__MAP_NAME) != 0 {
        return core::ptr::null_mut();
    }

    size = (end as usize).wrapping_sub(start as usize);

    buf = memdup(start, size).cast::<c_char>();
    if buf.is_null() {
        return core::ptr::null_mut();
    }

    fd = mkstemp((*vdso_file).temp_file_name.as_mut_ptr());
    if fd < 0 {
        free(buf.cast::<c_void>());
        (*vdso_file).found = !vdso.is_null();
        (*vdso_file).error = !(*vdso_file).found;
        return vdso;
    }

    if size == write(fd, buf.cast::<c_void>(), size) as usize {
        vdso = (*vdso_file).temp_file_name.as_mut_ptr();
    }

    close(fd);

    free(buf.cast::<c_void>());

    (*vdso_file).found = !vdso.is_null();
    (*vdso_file).error = !(*vdso_file).found;
    vdso
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn machine__exit_vdso(machine: *mut machine) {
    let vdso_info = *machine_vdso_info(machine);

    if vdso_info.is_null() {
        return;
    }

    if (*vdso_info).vdso.found {
        unlink((*vdso_info).vdso.temp_file_name.as_ptr());
    }
    #[cfg(target_pointer_width = "64")]
    {
        if (*vdso_info).vdso32.found {
            unlink((*vdso_info).vdso32.temp_file_name.as_ptr());
        }
        if (*vdso_info).vdsox32.found {
            unlink((*vdso_info).vdsox32.temp_file_name.as_ptr());
        }
    }

    zfree(machine_vdso_info(machine).cast::<*mut c_void>());
}

unsafe fn __machine__addnew_vdso(
    machine: *mut machine,
    short_name: *const c_char,
    long_name: *const c_char,
) -> *mut dso {
    let dso = dso__new(short_name);

    if !dso.is_null() {
        __dsos__add(machine_dsos(machine), dso);
        dso__set_long_name(dso, long_name, false);
    }

    dso
}

unsafe extern "C" fn machine__thread_dso_type_maps_cb(map: *mut map, data: *mut c_void) -> c_int {
    let args = data.cast::<machine__thread_dso_type_maps_cb_args>();
    let dso = map__dso(map);

    if dso.is_null() || *dso__long_name(dso) != b'/' as c_char {
        return 0;
    }

    (*args).dso_type = dso__type(dso, (*args).machine);
    if (*args).dso_type != dso_type::DSO__TYPE_UNKNOWN {
        1
    } else {
        0
    }
}

unsafe fn machine__thread_dso_type(machine: *mut machine, thread: *mut thread) -> dso_type {
    let mut args = machine__thread_dso_type_maps_cb_args {
        machine,
        dso_type: dso_type::DSO__TYPE_UNKNOWN,
    };

    maps__for_each_map(
        thread__maps(thread),
        Some(machine__thread_dso_type_maps_cb),
        (&mut args as *mut machine__thread_dso_type_maps_cb_args).cast::<c_void>(),
    );

    args.dso_type
}

#[cfg(target_pointer_width = "64")]
unsafe fn vdso__do_copy_compat(f: *mut FILE, fd: c_int) -> c_int {
    let mut buf = [0 as c_char; 4096];
    let mut count: usize;

    loop {
        count = fread(
            buf.as_mut_ptr().cast::<c_void>(),
            1,
            core::mem::size_of_val(&buf),
            f,
        );
        if ferror(f) != 0 {
            return -*__errno_location();
        }
        if feof(f) != 0 {
            break;
        }
        if count != 0 && writen(fd, buf.as_ptr().cast::<c_void>(), count) != count as isize {
            return -*__errno_location();
        }
    }

    0
}

#[cfg(target_pointer_width = "64")]
unsafe fn vdso__copy_compat(prog: *const c_char, fd: c_int) -> c_int {
    let f: *mut FILE;
    let err: c_int;

    f = popen(prog, c"r".as_ptr());
    if f.is_null() {
        return -*__errno_location();
    }

    err = vdso__do_copy_compat(f, fd);

    if pclose(f) == -1 {
        return -*__errno_location();
    }

    err
}

#[cfg(target_pointer_width = "64")]
unsafe fn vdso__create_compat_file(prog: *const c_char, temp_name: *mut c_char) -> c_int {
    let fd: c_int;
    let err: c_int;

    fd = mkstemp(temp_name);
    if fd < 0 {
        return -*__errno_location();
    }

    err = vdso__copy_compat(prog, fd);

    if close(fd) == -1 {
        return -*__errno_location();
    }

    err
}

#[cfg(target_pointer_width = "64")]
unsafe fn vdso__get_compat_file(vdso_file: *mut vdso_file) -> *const c_char {
    let err: c_int;

    if (*vdso_file).found {
        return (*vdso_file).temp_file_name.as_ptr();
    }

    if (*vdso_file).error {
        return core::ptr::null();
    }

    err = vdso__create_compat_file(
        (*vdso_file).read_prog,
        (*vdso_file).temp_file_name.as_mut_ptr(),
    );
    if err != 0 {
        pr_err(
            c"%s failed, error %d\n".as_ptr(),
            (*vdso_file).read_prog,
            err,
        );
        (*vdso_file).error = true;
        return core::ptr::null();
    }

    (*vdso_file).found = true;

    (*vdso_file).temp_file_name.as_ptr()
}

#[cfg(target_pointer_width = "64")]
unsafe fn __machine__findnew_compat(
    machine: *mut machine,
    vdso_file: *mut vdso_file,
) -> *mut dso {
    let file_name: *const c_char;
    let dso: *mut dso;

    dso = dsos__find(machine_dsos(machine), (*vdso_file).dso_name, true);
    if !dso.is_null() {
        return dso;
    }

    file_name = vdso__get_compat_file(vdso_file);
    if file_name.is_null() {
        return core::ptr::null_mut();
    }

    __machine__addnew_vdso(machine, (*vdso_file).dso_name, file_name)
}

#[cfg(target_pointer_width = "64")]
unsafe fn __machine__findnew_vdso_compat(
    machine: *mut machine,
    thread: *mut thread,
    vdso_info: *mut vdso_info,
    dso: *mut *mut dso,
) -> c_int {
    let dso_type: dso_type;

    dso_type = machine__thread_dso_type(machine, thread);

    // C source has:
    // #ifndef HAVE_PERF_READ_VDSO32
    // if (dso_type == DSO__TYPE_32BIT) return 0;
    // #endif
    // #ifndef HAVE_PERF_READ_VDSOX32
    // if (dso_type == DSO__TYPE_X32BIT) return 0;
    // #endif

    match dso_type {
        dso_type::DSO__TYPE_32BIT => {
            *dso = __machine__findnew_compat(machine, &mut (*vdso_info).vdso32);
            1
        }
        dso_type::DSO__TYPE_X32BIT => {
            *dso = __machine__findnew_compat(machine, &mut (*vdso_info).vdsox32);
            1
        }
        dso_type::DSO__TYPE_UNKNOWN | dso_type::DSO__TYPE_64BIT => 0,
    }
}

unsafe fn machine__find_vdso(machine: *mut machine, thread: *mut thread) -> *mut dso {
    let mut dso: *mut dso = core::ptr::null_mut();
    let dso_type: dso_type;

    dso_type = machine__thread_dso_type(machine, thread);
    match dso_type {
        dso_type::DSO__TYPE_32BIT => {
            dso = dsos__find(machine_dsos(machine), DSO__NAME_VDSO32, true);
            if dso.is_null() {
                dso = dsos__find(machine_dsos(machine), DSO__NAME_VDSO, true);
                if !dso.is_null() && dso_type != dso__type(dso, machine) {
                    dso__put(dso);
                    dso = core::ptr::null_mut();
                }
            }
        }
        dso_type::DSO__TYPE_X32BIT => {
            dso = dsos__find(machine_dsos(machine), DSO__NAME_VDSOX32, true);
        }
        dso_type::DSO__TYPE_64BIT | dso_type::DSO__TYPE_UNKNOWN => {
            dso = dsos__find(machine_dsos(machine), DSO__NAME_VDSO, true);
        }
    }

    dso
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn machine__findnew_vdso(
    machine: *mut machine,
    thread: *mut thread,
) -> *mut dso {
    let vdso_info: *mut vdso_info;
    let mut dso: *mut dso = core::ptr::null_mut();
    let file: *mut c_char;

    if (*machine_vdso_info(machine)).is_null() {
        *machine_vdso_info(machine) = vdso_info__new();
    }

    vdso_info = *machine_vdso_info(machine);
    if vdso_info.is_null() {
        return core::ptr::null_mut();
    }

    dso = machine__find_vdso(machine, thread);
    if !dso.is_null() {
        return dso;
    }

    #[cfg(target_pointer_width = "64")]
    {
        if __machine__findnew_vdso_compat(machine, thread, vdso_info, &mut dso) != 0 {
            return dso;
        }
    }

    dso = dsos__find(machine_dsos(machine), DSO__NAME_VDSO, true);
    if !dso.is_null() {
        return dso;
    }

    file = get_file(&mut (*vdso_info).vdso);
    if file.is_null() {
        return core::ptr::null_mut();
    }

    __machine__addnew_vdso(machine, DSO__NAME_VDSO, file)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__is_vdso(dso: *mut dso) -> bool {
    strcmp(dso__short_name(dso), DSO__NAME_VDSO) == 0
        || strcmp(dso__short_name(dso), DSO__NAME_VDSO32) == 0
        || strcmp(dso__short_name(dso), DSO__NAME_VDSOX32) == 0
}
