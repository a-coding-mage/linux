// SPDX-License-Identifier: GPL-2.0-only

// Translated from testing/selftests/perf_events/mmap.c.
// C dependencies: dirent.h, sched.h, stdbool.h, stdio.h, unistd.h,
// sys/ioctl.h, sys/mman.h, sys/syscall.h, sys/types.h,
// linux/perf_event.h, kselftest_harness.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const RB_SIZE: c_ulong = 0x3000;
const AUX_SIZE: c_ulong = 0x10000;
const AUX_OFFS: c_ulong = 0x4000;

const HOLE_SIZE: c_ulong = 0x1000;

/* Reserve space for rb, aux with space for shrink-beyond-vma testing. */
const REGION_SIZE: c_ulong = 2 * RB_SIZE + 2 * AUX_SIZE;
const REGION_AUX_OFFS: c_ulong = 2 * RB_SIZE;

const MAP_BASE: c_uint = 1;
const MAP_AUX: c_uint = 2;

const EVENT_SRC_DIR: &[u8] = b"/sys/bus/event_source/devices\0";

const EACCES: c_int = 13;
const SYS_PERF_EVENT_OPEN: c_long = 298;

const PROT_NONE: c_int = 0x0;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;

const MAP_SHARED: c_int = 0x01;
const MAP_PRIVATE: c_int = 0x02;
const MAP_FIXED: c_int = 0x10;
const MAP_ANON: c_int = 0x20;
const MAP_ANONYMOUS: c_int = MAP_ANON;
const MREMAP_MAYMOVE: c_int = 1;
const MREMAP_FIXED: c_int = 2;

const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    pub d_ino: c_ulong,
    pub d_off: c_long,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period: u64,
    pub sample_type: u64,
    pub read_format: u64,
    pub flags: u64,
}

impl perf_event_attr {
    const DISABLED: u64 = 1 << 0;
    const EXCLUDE_KERNEL: u64 = 1 << 5;
    const EXCLUDE_HV: u64 = 1 << 6;
}

#[repr(C)]
pub struct perf_event_mmap_page {
    pub version: u32,
    pub compat_version: u32,
    pub lock: u32,
    pub index: u32,
    pub offset: i64,
    pub time_enabled: u64,
    pub time_running: u64,
    pub capabilities: u64,
    pub pmc_width: u16,
    pub time_shift: u16,
    pub time_mult: u32,
    pub time_offset: u64,
    pub time_zero: u64,
    pub size: u32,
    pub __reserved_1: u32,
    pub time_cycles: u64,
    pub time_mask: u64,
    pub __reserved: [u8; 928],
    pub data_head: u64,
    pub data_tail: u64,
    pub data_offset: u64,
    pub data_size: u64,
    pub aux_head: u64,
    pub aux_tail: u64,
    pub aux_offset: u64,
    pub aux_size: u64,
}

#[repr(C)]
pub struct perf_mmap {
    pub fd: c_int,
    pub ptr: *mut c_void,
    pub region: *mut c_void,
}

#[repr(C)]
pub struct perf_mmap_variant {
    pub aux: bool,
    pub ptr_size: c_ulong,
}

pub static perf_mmap_rb: perf_mmap_variant = perf_mmap_variant {
    aux: false,
    ptr_size: RB_SIZE,
};

pub static perf_mmap_aux: perf_mmap_variant = perf_mmap_variant {
    aux: true,
    ptr_size: AUX_SIZE,
};

unsafe extern "C" {
    static mut errno: c_int;

    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn close(fd: c_int) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn mremap(
        old_address: *mut c_void,
        old_size: usize,
        new_size: usize,
        flags: c_int,
        ...
    ) -> *mut c_void;
}

macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {
        assert_ne!($left, $right)
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! SKIP {
    (return, $msg:expr) => {
        return
    };
}

unsafe fn read_event_type(dent: *mut dirent, type_: *mut u32) -> bool {
    let mut typefn: [c_char; 512] = [0; 512];
    let fp: *mut FILE;
    let res: c_int;

    snprintf(
        typefn.as_mut_ptr(),
        typefn.len(),
        b"%s/%s/type\0".as_ptr() as *const c_char,
        EVENT_SRC_DIR.as_ptr() as *const c_char,
        (*dent).d_name.as_ptr(),
    );
    fp = fopen(typefn.as_ptr(), b"r\0".as_ptr() as *const c_char);
    if fp.is_null() {
        return false;
    }

    res = fscanf(fp, b"%u\0".as_ptr() as *const c_char, type_);
    fclose(fp);
    res > 0
}

pub unsafe fn perf_mmap_setup(self_: *mut perf_mmap, variant: *const perf_mmap_variant) {
    let mut attr = perf_event_attr {
        type_: 0,
        size: core::mem::size_of::<perf_event_attr>() as u32,
        config: 0,
        sample_period: 0,
        sample_type: 0,
        read_format: 0,
        flags: perf_event_attr::DISABLED
            | perf_event_attr::EXCLUDE_KERNEL
            | perf_event_attr::EXCLUDE_HV,
    };
    let mut attr_ok: perf_event_attr = core::mem::zeroed();
    let mut eacces: c_uint = 0;
    let mut map: c_uint = 0;
    let mut rb: *mut perf_event_mmap_page;
    let mut dent: *mut dirent;
    let mut aux: *mut c_void;
    let region: *mut c_void;
    let dir: *mut DIR;

    (*self_).ptr = core::ptr::null_mut();

    dir = opendir(EVENT_SRC_DIR.as_ptr() as *const c_char);
    if dir.is_null() {
        SKIP!(return, "perf not available.");
    }

    region = mmap(
        core::ptr::null_mut(),
        REGION_SIZE as usize,
        PROT_NONE,
        MAP_ANON | MAP_PRIVATE,
        -1,
        0,
    );
    ASSERT_NE!(region, MAP_FAILED);
    (*self_).region = region;

    // Try to find a suitable event on this system
    loop {
        dent = readdir(dir);
        if dent.is_null() {
            break;
        }
        let fd: c_int;

        if !read_event_type(dent, &mut attr.type_) {
            continue;
        }

        fd = syscall(SYS_PERF_EVENT_OPEN, &mut attr as *mut perf_event_attr, 0, -1, -1, 0) as c_int;
        if fd < 0 {
            if errno == EACCES {
                eacces += 1;
            }
            continue;
        }

        // Check whether the event supports mmap()
        rb = mmap(
            region,
            RB_SIZE as usize,
            PROT_READ | PROT_WRITE,
            MAP_SHARED | MAP_FIXED,
            fd,
            0,
        ) as *mut perf_event_mmap_page;
        if rb as *mut c_void == MAP_FAILED {
            close(fd);
            continue;
        }

        if map == 0 {
            // Save the event in case that no AUX capable event is found
            attr_ok = attr;
            map = MAP_BASE;
        }

        if !(*variant).aux {
            continue;
        }

        (*rb).aux_offset = AUX_OFFS as u64;
        (*rb).aux_size = AUX_SIZE as u64;

        // Check whether it supports a AUX buffer
        aux = mmap(
            (region as *mut u8).add(REGION_AUX_OFFS as usize) as *mut c_void,
            AUX_SIZE as usize,
            PROT_READ | PROT_WRITE,
            MAP_SHARED | MAP_FIXED,
            fd,
            AUX_OFFS as c_long,
        );
        if aux == MAP_FAILED {
            munmap(rb as *mut c_void, RB_SIZE as usize);
            close(fd);
            continue;
        }

        attr_ok = attr;
        map = MAP_AUX;
        munmap(aux, AUX_SIZE as usize);
        munmap(rb as *mut c_void, RB_SIZE as usize);
        close(fd);
        break;
    }
    closedir(dir);

    if map == 0 {
        if eacces == 0 {
            SKIP!(return, "No mappable perf event found.");
        } else {
            SKIP!(return, "No permissions for perf_event_open()");
        }
    }

    (*self_).fd =
        syscall(SYS_PERF_EVENT_OPEN, &mut attr_ok as *mut perf_event_attr, 0, -1, -1, 0) as c_int;
    ASSERT_NE!((*self_).fd, -1);

    rb = mmap(
        region,
        RB_SIZE as usize,
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_FIXED,
        (*self_).fd,
        0,
    ) as *mut perf_event_mmap_page;
    ASSERT_NE!(rb as *mut c_void, MAP_FAILED);

    if !(*variant).aux {
        (*self_).ptr = rb as *mut c_void;
        return;
    }

    if map != MAP_AUX {
        SKIP!(return, "No AUX event found.");
    }

    (*rb).aux_offset = AUX_OFFS as u64;
    (*rb).aux_size = AUX_SIZE as u64;
    aux = mmap(
        (region as *mut u8).add(REGION_AUX_OFFS as usize) as *mut c_void,
        AUX_SIZE as usize,
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_FIXED,
        (*self_).fd,
        AUX_OFFS as c_long,
    );
    ASSERT_NE!(aux, MAP_FAILED);
    (*self_).ptr = aux;
}

pub unsafe fn perf_mmap_teardown(self_: *mut perf_mmap) {
    ASSERT_EQ!(munmap((*self_).region, REGION_SIZE as usize), 0);
    if (*self_).fd != -1 {
        ASSERT_EQ!(close((*self_).fd), 0);
    }
}

pub unsafe fn perf_mmap_remap(self_: *mut perf_mmap, variant: *const perf_mmap_variant) {
    let tmp: *mut c_void;
    let mut ptr: *mut c_void = (*self_).ptr;
    let size: c_ulong = (*variant).ptr_size;

    // Test the invalid remaps
    ASSERT_EQ!(
        mremap(ptr, size as usize, HOLE_SIZE as usize, MREMAP_MAYMOVE),
        MAP_FAILED
    );
    ASSERT_EQ!(
        mremap(
            (ptr as *mut u8).add(HOLE_SIZE as usize) as *mut c_void,
            size as usize,
            HOLE_SIZE as usize,
            MREMAP_MAYMOVE,
        ),
        MAP_FAILED
    );
    ASSERT_EQ!(
        mremap(
            (ptr as *mut u8).add((size - HOLE_SIZE) as usize) as *mut c_void,
            HOLE_SIZE as usize,
            size as usize,
            MREMAP_MAYMOVE,
        ),
        MAP_FAILED
    );
    // Shrink the end of the mapping such that we only unmap past end of the VMA,
    // which should succeed and poke a hole into the PROT_NONE region
    ASSERT_NE!(
        mremap(
            (ptr as *mut u8).add((size - HOLE_SIZE) as usize) as *mut c_void,
            size as usize,
            HOLE_SIZE as usize,
            MREMAP_MAYMOVE,
        ),
        MAP_FAILED
    );

    // Remap the whole buffer to a new address
    tmp = mmap(
        core::ptr::null_mut(),
        size as usize,
        PROT_READ | PROT_WRITE,
        MAP_SHARED | MAP_ANONYMOUS,
        -1,
        0,
    );
    ASSERT_NE!(tmp, MAP_FAILED);

    // Try splitting offset 1 hole size into VMA, this should fail
    ASSERT_EQ!(
        mremap(
            (ptr as *mut u8).add(HOLE_SIZE as usize) as *mut c_void,
            (size - HOLE_SIZE) as usize,
            (size - HOLE_SIZE) as usize,
            MREMAP_MAYMOVE | MREMAP_FIXED,
            tmp,
        ),
        MAP_FAILED
    );
    // Remapping the whole thing should succeed fine
    ptr = mremap(
        ptr,
        size as usize,
        size as usize,
        MREMAP_MAYMOVE | MREMAP_FIXED,
        tmp,
    );
    ASSERT_EQ!(ptr, tmp);
    ASSERT_EQ!(munmap(tmp, size as usize), 0);
}

pub unsafe fn perf_mmap_unmap(self_: *mut perf_mmap, variant: *const perf_mmap_variant) {
    let size: c_ulong = (*variant).ptr_size;

    // Try to poke holes into the mappings
    ASSERT_NE!(munmap((*self_).ptr, HOLE_SIZE as usize), 0);
    ASSERT_NE!(
        munmap(
            ((*self_).ptr as *mut u8).add(HOLE_SIZE as usize) as *mut c_void,
            HOLE_SIZE as usize,
        ),
        0
    );
    ASSERT_NE!(
        munmap(
            ((*self_).ptr as *mut u8).add((size - HOLE_SIZE) as usize) as *mut c_void,
            HOLE_SIZE as usize,
        ),
        0
    );
}

pub unsafe fn perf_mmap_map(self_: *mut perf_mmap, variant: *const perf_mmap_variant) {
    let size: c_ulong = (*variant).ptr_size;

    // Try to poke holes into the mappings by mapping anonymous memory over it
    ASSERT_EQ!(
        mmap(
            (*self_).ptr,
            HOLE_SIZE as usize,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANON | MAP_FIXED,
            -1,
            0,
        ),
        MAP_FAILED
    );
    ASSERT_EQ!(
        mmap(
            ((*self_).ptr as *mut u8).add(HOLE_SIZE as usize) as *mut c_void,
            HOLE_SIZE as usize,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANON | MAP_FIXED,
            -1,
            0,
        ),
        MAP_FAILED
    );
    ASSERT_EQ!(
        mmap(
            ((*self_).ptr as *mut u8).add((size - HOLE_SIZE) as usize) as *mut c_void,
            HOLE_SIZE as usize,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANON | MAP_FIXED,
            -1,
            0,
        ),
        MAP_FAILED
    );
}

// TEST_HARNESS_MAIN
