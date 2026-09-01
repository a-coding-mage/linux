// SPDX-License-Identifier: GPL-2.0
/*
 * Ring-buffer memory mapping tests
 *
 * Copyright (c) 2024 Vincent Donnefort <vdonnefort@google.com>
 */

use libc::{
    c_char, c_int, c_ulong, c_void, close, free, getpagesize, getuid, ioctl, mmap, munmap, open,
    sched_getcpu, sched_setaffinity, stat, write, MAP_FAILED, MAP_SHARED, O_NONBLOCK, O_RDONLY,
    O_TRUNC, O_WRONLY, PROT_READ,
};
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;

// C includes translated as external dependencies:
// <linux/trace_mmap.h>
// ../user_events/user_events_selftests.h
// kselftest_harness.h

const TRACEFS_ROOT: &str = "/sys/kernel/tracing";

const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EBUSY: c_int = 16;

extern "C" {
    static mut errno: c_int;

    fn tracefs_enabled(message: *mut *mut c_char, fail: *mut bool, umount: *mut bool) -> bool;
    fn tracefs_unmount();
}

extern "C" {
    static TRACE_MMAP_IOCTL_GET_READER: c_ulong;
}

#[repr(C)]
pub struct trace_buffer_meta_reader {
    pub id: c_int,
    pub read: c_ulong,
}

#[repr(C)]
pub struct trace_buffer_meta {
    pub meta_struct_len: c_int,
    pub meta_page_size: c_int,
    pub subbuf_size: c_ulong,
    pub nr_subbufs: c_ulong,
    pub entries: c_ulong,
    pub overrun: c_ulong,
    pub read: c_ulong,
    pub reader: trace_buffer_meta_reader,
}

unsafe fn c_path(path: &str) -> CString {
    CString::new(path).unwrap()
}

unsafe fn __tracefs_write(path: *const c_char, value: *const c_char) -> c_int {
    let fd: c_int;
    let ret: isize;

    fd = open(path, O_WRONLY | O_TRUNC);
    if fd < 0 {
        return fd;
    }

    ret = write(fd, value as *const c_void, CStr::from_ptr(value).to_bytes().len());

    close(fd);

    if ret == -1 {
        -errno
    } else {
        0
    }
}

unsafe fn __tracefs_write_int(path: *const c_char, value: c_int) -> c_int {
    let str = CString::new(format!("{}", value)).unwrap();
    let ret: c_int;

    ret = __tracefs_write(path, str.as_ptr());

    ret
}

macro_rules! tracefs_write_int {
    ($path:expr, $value:expr) => {
        ASSERT_EQ!(unsafe { __tracefs_write_int(c_path($path).as_ptr(), $value) }, 0)
    };
}

macro_rules! tracefs_write {
    ($path:expr, $value:expr) => {
        ASSERT_EQ!(
            unsafe { __tracefs_write(c_path($path).as_ptr(), c_path($value).as_ptr()) },
            0
        )
    };
}

unsafe fn tracefs_reset() -> c_int {
    if __tracefs_write_int(c_path(&format!("{}/tracing_on", TRACEFS_ROOT)).as_ptr(), 0) != 0 {
        return -1;
    }
    if __tracefs_write(c_path(&format!("{}/trace", TRACEFS_ROOT)).as_ptr(), c_path("").as_ptr()) != 0
    {
        return -1;
    }
    if __tracefs_write(c_path(&format!("{}/set_event", TRACEFS_ROOT)).as_ptr(), c_path("").as_ptr())
        != 0
    {
        return -1;
    }
    if __tracefs_write(
        c_path(&format!("{}/current_tracer", TRACEFS_ROOT)).as_ptr(),
        c_path("nop").as_ptr(),
    ) != 0
    {
        return -1;
    }

    0
}

#[repr(C)]
pub struct tracefs_cpu_map_desc {
    pub meta: *mut trace_buffer_meta,
    pub cpu_fd: c_int,
}

pub unsafe fn tracefs_cpu_map(desc: *mut tracefs_cpu_map_desc, cpu: c_int) -> c_int {
    let mut page_size: c_int = getpagesize();
    let cpu_path: CString;
    let mut map: *mut c_void;

    cpu_path = match CString::new(format!(
        "{}/per_cpu/cpu{}/trace_pipe_raw",
        TRACEFS_ROOT, cpu
    )) {
        Ok(path) => path,
        Err(_) => return -ENOMEM,
    };

    (*desc).cpu_fd = open(cpu_path.as_ptr(), O_RDONLY | O_NONBLOCK);
    if (*desc).cpu_fd < 0 {
        return -ENODEV;
    }

    loop {
        map = mmap(
            ptr::null_mut(),
            page_size as usize,
            PROT_READ,
            MAP_SHARED,
            (*desc).cpu_fd,
            0,
        );
        if map == MAP_FAILED {
            return -errno;
        }

        (*desc).meta = map as *mut trace_buffer_meta;

        /* the meta-page is bigger than the original mapping */
        if page_size < (*(*desc).meta).meta_struct_len {
            let meta_page_size: c_int = (*(*desc).meta).meta_page_size;

            munmap((*desc).meta as *mut c_void, page_size as usize);
            page_size = meta_page_size;
            continue;
        }

        break;
    }

    0
}

pub unsafe fn tracefs_cpu_unmap(desc: *mut tracefs_cpu_map_desc) {
    munmap(
        (*desc).meta as *mut c_void,
        (*(*desc).meta).meta_page_size as usize,
    );
    close((*desc).cpu_fd);
}

FIXTURE!(map {
    map_desc: tracefs_cpu_map_desc,
    umount: bool,
});

FIXTURE_VARIANT!(map {
    subbuf_size: c_int,
});

FIXTURE_VARIANT_ADD!(map, subbuf_size_4k {
    subbuf_size: 4,
});

FIXTURE_VARIANT_ADD!(map, subbuf_size_8k {
    subbuf_size: 8,
});

FIXTURE_SETUP!(map {
    let cpu: c_int = unsafe { sched_getcpu() };
    let mut cpu_mask: libc::cpu_set_t = unsafe { mem::zeroed() };
    let mut fail: bool = false;
    let mut umount: bool = false;
    let mut message: *mut c_char = ptr::null_mut();

    if unsafe { getuid() } != 0 {
        SKIP!(return, "Skipping: %s", "Please run the test as root");
    }

    if unsafe { !tracefs_enabled(&mut message, &mut fail, &mut umount) } {
        if fail {
            TH_LOG!("Tracefs setup failed: %s", message);
            ASSERT_FALSE!(fail);
        }
        SKIP!(return, "Skipping: %s", message);
    }

    self.umount = umount;

    ASSERT_GE!(cpu, 0);

    ASSERT_EQ!(unsafe { tracefs_reset() }, 0);

    tracefs_write_int!(&format!("{}/buffer_subbuf_size_kb", TRACEFS_ROOT), variant.subbuf_size);

    ASSERT_EQ!(unsafe { tracefs_cpu_map(&mut self.map_desc, cpu) }, 0);

    /*
     * Ensure generated events will be found on this very same ring-buffer.
     */
    unsafe {
        libc::CPU_ZERO(&mut cpu_mask);
        libc::CPU_SET(cpu as usize, &mut cpu_mask);
    }
    ASSERT_EQ!(
        unsafe {
            sched_setaffinity(
                0,
                mem::size_of_val(&cpu_mask),
                &cpu_mask as *const libc::cpu_set_t,
            )
        },
        0
    );
});

FIXTURE_TEARDOWN!(map {
    unsafe {
        tracefs_reset();
    }

    if self.umount {
        unsafe {
            tracefs_unmount();
        }
    }

    unsafe {
        tracefs_cpu_unmap(&mut self.map_desc);
    }
});

TEST_F!(map, meta_page_check {
    let desc: *mut tracefs_cpu_map_desc = &mut self.map_desc;
    let mut cnt: c_int = 0;

    unsafe {
        ASSERT_EQ!((*(*desc).meta).entries, 0);
        ASSERT_EQ!((*(*desc).meta).overrun, 0);
        ASSERT_EQ!((*(*desc).meta).read, 0);

        ASSERT_EQ!((*(*desc).meta).reader.id, 0);
        ASSERT_EQ!((*(*desc).meta).reader.read, 0);

        ASSERT_EQ!(ioctl((*desc).cpu_fd, TRACE_MMAP_IOCTL_GET_READER), 0);
        ASSERT_EQ!((*(*desc).meta).reader.id, 0);

        tracefs_write_int!(&format!("{}/tracing_on", TRACEFS_ROOT), 1);
        for i in 0..16 {
            tracefs_write_int!(&format!("{}/trace_marker", TRACEFS_ROOT), i);
        }

        loop {
            ASSERT_EQ!(ioctl((*desc).cpu_fd, TRACE_MMAP_IOCTL_GET_READER), 0);

            ASSERT_EQ!((*(*desc).meta).entries, 16);
            ASSERT_EQ!((*(*desc).meta).overrun, 0);
            ASSERT_EQ!((*(*desc).meta).read, 16);

            ASSERT_EQ!((*(*desc).meta).reader.id, 1);

            if cnt == 0 {
                cnt += 1;
                continue;
            }
            cnt += 1;
            break;
        }
    }
});

TEST_F!(map, data_mmap {
    let desc: *mut tracefs_cpu_map_desc = &mut self.map_desc;
    let mut meta_len: c_ulong;
    let mut data_len: c_ulong;
    let mut data: *mut c_void;

    unsafe {
        meta_len = (*(*desc).meta).meta_page_size as c_ulong;
        data_len = (*(*desc).meta).subbuf_size * (*(*desc).meta).nr_subbufs;

        /* Map all the available subbufs */
        data = mmap(
            ptr::null_mut(),
            data_len as usize,
            PROT_READ,
            MAP_SHARED,
            (*desc).cpu_fd,
            meta_len as libc::off_t,
        );
        ASSERT_NE!(data, MAP_FAILED);
        munmap(data, data_len as usize);

        /* Map all the available subbufs - 1 */
        data_len -= (*(*desc).meta).subbuf_size;
        data = mmap(
            ptr::null_mut(),
            data_len as usize,
            PROT_READ,
            MAP_SHARED,
            (*desc).cpu_fd,
            meta_len as libc::off_t,
        );
        ASSERT_NE!(data, MAP_FAILED);
        munmap(data, data_len as usize);

        /* Offset within ring-buffer bounds, mapping size overflow */
        meta_len += (*(*desc).meta).subbuf_size * 2;
        data = mmap(
            ptr::null_mut(),
            data_len as usize,
            PROT_READ,
            MAP_SHARED,
            (*desc).cpu_fd,
            meta_len as libc::off_t,
        );
        ASSERT_EQ!(data, MAP_FAILED);

        /* Offset outside ring-buffer bounds */
        data_len = (*(*desc).meta).subbuf_size * (*(*desc).meta).nr_subbufs;
        data = mmap(
            ptr::null_mut(),
            data_len as usize,
            PROT_READ,
            MAP_SHARED,
            (*desc).cpu_fd,
            (data_len + ((*(*desc).meta).subbuf_size * 2)) as libc::off_t,
        );
        ASSERT_EQ!(data, MAP_FAILED);

        /* Verify meta-page padding */
        if (*(*desc).meta).meta_page_size > getpagesize() {
            data_len = (*(*desc).meta).meta_page_size as c_ulong;
            data = mmap(
                ptr::null_mut(),
                data_len as usize,
                PROT_READ,
                MAP_SHARED,
                (*desc).cpu_fd,
                0,
            );
            ASSERT_NE!(data, MAP_FAILED);

            let mut i = (*(*desc).meta).meta_struct_len;
            while i < (*(*desc).meta).meta_page_size {
                ASSERT_EQ!(*(data.add(i as usize) as *const c_int), 0);
                i += mem::size_of::<c_int>() as c_int;
            }

            munmap(data, data_len as usize);
        }
    }
});

FIXTURE!(snapshot {
    umount: bool,
});

FIXTURE_SETUP!(snapshot {
    let mut fail: bool = false;
    let mut umount: bool = false;
    let mut sb: stat = unsafe { mem::zeroed() };
    let mut message: *mut c_char = ptr::null_mut();

    if unsafe { getuid() } != 0 {
        SKIP!(return, "Skipping: %s", "Please run the test as root");
    }

    if unsafe { stat(c_path(&format!("{}/snapshot", TRACEFS_ROOT)).as_ptr(), &mut sb) } != 0 {
        SKIP!(return, "Skipping: %s", "snapshot not available");
    }

    if unsafe { !tracefs_enabled(&mut message, &mut fail, &mut umount) } {
        if fail {
            TH_LOG!("Tracefs setup failed: %s", message);
            ASSERT_FALSE!(fail);
        }
        SKIP!(return, "Skipping: %s", message);
    }

    self.umount = umount;
});

FIXTURE_TEARDOWN!(snapshot {
    unsafe {
        __tracefs_write(
            c_path(&format!("{}/events/sched/sched_switch/trigger", TRACEFS_ROOT)).as_ptr(),
            c_path("!snapshot").as_ptr(),
        );
        tracefs_reset();
    }

    if self.umount {
        unsafe {
            tracefs_unmount();
        }
    }
});

TEST_F!(snapshot, excludes_map {
    let mut map_desc: tracefs_cpu_map_desc = unsafe { mem::zeroed() };
    let cpu: c_int = unsafe { sched_getcpu() };

    ASSERT_GE!(cpu, 0);
    tracefs_write!(
        &format!("{}/events/sched/sched_switch/trigger", TRACEFS_ROOT),
        "snapshot"
    );
    ASSERT_EQ!(unsafe { tracefs_cpu_map(&mut map_desc, cpu) }, -EBUSY);
});

TEST_F!(snapshot, excluded_by_map {
    let mut map_desc: tracefs_cpu_map_desc = unsafe { mem::zeroed() };
    let cpu: c_int = unsafe { sched_getcpu() };

    ASSERT_EQ!(unsafe { tracefs_cpu_map(&mut map_desc, cpu) }, 0);

    ASSERT_EQ!(
        unsafe {
            __tracefs_write(
                c_path(&format!("{}/events/sched/sched_switch/trigger", TRACEFS_ROOT)).as_ptr(),
                c_path("snapshot").as_ptr(),
            )
        },
        -EBUSY
    );
    ASSERT_EQ!(
        unsafe {
            __tracefs_write(
                c_path(&format!("{}/snapshot", TRACEFS_ROOT)).as_ptr(),
                c_path("1").as_ptr(),
            )
        },
        -EBUSY
    );
});

TEST_HARNESS_MAIN!();

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
