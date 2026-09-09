// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel environment:
// linux/efi.h, linux/init.h, linux/io.h, linux/kernel.h, linux/kobject.h,
// linux/module.h, linux/platform_device.h, and linux/sysfs.h.

const OVMF_DEBUG_LOG_MAGIC1: u64 = 0x3167_646d_666d_766f; // "ovmfmdg1"
const OVMF_DEBUG_LOG_MAGIC2: u64 = 0x3267_646d_666d_766f; // "ovmfmdg2"

#[repr(C)]
struct ovmf_debug_log_header {
    magic1: u64,
    magic2: u64,
    hdr_size: u64,
    log_size: u64,
    lock: u64, // edk2 spinlock
    head_off: u64,
    tail_off: u64,
    truncated: u64,
    fw_version: [u8; 128],
}

#[repr(C)]
struct file {
    _private: [u8; 0],
}

#[repr(C)]
struct kobject {
    _private: [u8; 0],
}

#[repr(C)]
struct attribute {
    name: *const u8,
    mode: u16,
}

#[repr(C)]
struct bin_attribute {
    attr: attribute,
    size: u64,
    read: Option<unsafe extern "C" fn(
        filp: *mut file,
        kobj: *mut kobject,
        attr: *const bin_attribute,
        buf: *mut i8,
        offset: i64,
        count: usize,
    ) -> isize>,
}

extern "C" {
    static mut efi_kobj: *mut kobject;

    fn memremap(addr: usize, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn memunmap(addr: *mut core::ffi::c_void);
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, count: usize)
        -> *mut core::ffi::c_void;
    fn sysfs_create_bin_file(kobj: *mut kobject, attr: *mut bin_attribute) -> i32;
}

const MEMREMAP_WB: u32 = 0;

extern "C" {
    fn pr_err(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
    fn printk(fmt: *const u8, ...);
}

const KERN_ERR: *const u8 = b"<3>\0".as_ptr();
const EINVAL: i32 = 22;

static mut hdr: *mut ovmf_debug_log_header = core::ptr::null_mut();
static mut logbuf: *mut u8 = core::ptr::null_mut();
static mut logbufsize: u64 = 0;

unsafe extern "C" fn ovmf_log_read(
    _filp: *mut file,
    _kobj: *mut kobject,
    _attr: *const bin_attribute,
    buf: *mut i8,
    offset: i64,
    count: usize,
) -> isize {
    let mut start: u64;
    let mut end: u64;

    start = (*hdr).head_off.wrapping_add(offset as u64);
    if (*hdr).head_off > (*hdr).tail_off && start >= (*hdr).log_size {
        start = start.wrapping_sub((*hdr).log_size);
    }

    end = start.wrapping_add(count as u64);
    if start > (*hdr).tail_off {
        if end > (*hdr).log_size {
            end = (*hdr).log_size;
        }
    } else if end > (*hdr).tail_off {
        end = (*hdr).tail_off;
    }

    if start > logbufsize || end > logbufsize {
        return 0;
    }
    if start >= end {
        return 0;
    }

    memcpy(
        buf as *mut core::ffi::c_void,
        logbuf.add(start as usize) as *const core::ffi::c_void,
        (end - start) as usize,
    );
    (end - start) as isize
}

static mut ovmf_log_bin_attr: bin_attribute = bin_attribute {
    attr: attribute {
        name: b"ovmf_debug_log\0".as_ptr(),
        mode: 0o444,
    },
    size: 0,
    read: Some(ovmf_log_read),
};

// __init
#[no_mangle]
pub unsafe extern "C" fn ovmf_log_probe(ovmf_debug_log_table: usize) -> i32 {
    let mut ret: i32 = -EINVAL;
    let size: u64;

    // map + verify header
    hdr = memremap(
        ovmf_debug_log_table,
        core::mem::size_of::<ovmf_debug_log_header>(),
        MEMREMAP_WB,
    ) as *mut ovmf_debug_log_header;
    if hdr.is_null() {
        pr_err(b"OVMF debug log: header map failed\n\0".as_ptr());
        return -EINVAL;
    }

    if (*hdr).magic1 != OVMF_DEBUG_LOG_MAGIC1 || (*hdr).magic2 != OVMF_DEBUG_LOG_MAGIC2 {
        printk(KERN_ERR);
        goto_err_unmap();
        return ret;
    }

    size = (*hdr).hdr_size.wrapping_add((*hdr).log_size);
    pr_info(b"OVMF debug log: firmware version: \"%s\"\n\0".as_ptr(), (*hdr).fw_version.as_ptr());
    pr_info(b"OVMF debug log: buffer size: %lluk\n\0".as_ptr(), size / 1024);

    // map complete log buffer
    memunmap(hdr as *mut core::ffi::c_void);
    hdr = memremap(ovmf_debug_log_table, size as usize, MEMREMAP_WB) as *mut ovmf_debug_log_header;
    if hdr.is_null() {
        pr_err(b"OVMF debug log: buffer map failed\n\0".as_ptr());
        return -EINVAL;
    }
    logbuf = (hdr as *mut u8).add((*hdr).hdr_size as usize);
    logbufsize = (*hdr).log_size;

    ovmf_log_bin_attr.size = size;
    ret = sysfs_create_bin_file(efi_kobj, &mut ovmf_log_bin_attr);
    if ret != 0 {
        pr_err(b"OVMF debug log: sysfs register failed\n\0".as_ptr());
        goto_err_unmap();
    }

    return ret;
}

#[inline(always)]
unsafe fn goto_err_unmap() {
    memunmap(hdr as *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
