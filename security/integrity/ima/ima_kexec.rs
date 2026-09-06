// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 IBM Corporation
 *
 * Authors:
 * Thiago Jung Bauermann <bauerman@linux.vnet.ibm.com>
 * Mimi Zohar <zohar@linux.vnet.ibm.com>
 */

// Linux kernel headers: <linux/seq_file.h>, <linux/vmalloc.h>, <linux/kexec.h>,
// <linux/of.h>, <linux/ima.h>, <linux/mm.h>, <linux/overflow.h>, <linux/reboot.h>,
// <asm/page.h>, and local "ima.h" module

use core::ffi::c_void;
use core::mem;
use core::ptr::{self, null_mut};

#[cfg(feature = "CONFIG_IMA_KEXEC")]
const IMA_KEXEC_EVENT_LEN: usize = 256;

#[cfg(feature = "CONFIG_IMA_KEXEC")]
static mut ima_kexec_update_registered: bool = false;

#[cfg(feature = "CONFIG_IMA_KEXEC")]
static mut ima_kexec_file: seq_file = seq_file {
    buf: ptr::null_mut(),
    size: 0,
    read_pos: 0,
    count: 0,
};

#[cfg(feature = "CONFIG_IMA_KEXEC")]
static mut kexec_segment_size: usize = 0;

#[cfg(feature = "CONFIG_IMA_KEXEC")]
static mut ima_kexec_buffer: *mut c_void = ptr::null_mut();

// External types from kernel headers
#[repr(C)]
pub struct seq_file {
    pub buf: *mut u8,
    pub size: usize,
    pub read_pos: usize,
    pub count: usize,
}

#[repr(C)]
pub struct ima_queue_entry {
    // Opaque kernel structure
    _private: [u8; 0],
}

#[repr(C)]
pub struct ima_kexec_hdr {
    pub version: u16,
    pub _pad: u16,
    pub count: u64,
    pub buffer_size: u64,
}

#[repr(C)]
pub struct kimage {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kexec_buf {
    pub image: *mut kimage,
    pub buffer: *mut c_void,
    pub bufsz: usize,
    pub memsz: usize,
    pub buf_align: usize,
    pub buf_min: usize,
    pub buf_max: usize,
    pub top_down: bool,
    pub mem: usize,
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call: extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int,
    pub next: *mut notifier_block,
    pub priority: c_int,
}

type c_int = i32;
type c_ulong = usize;

// External kernel functions
extern "C" {
    fn vfree(addr: *mut c_void);
    fn vmalloc(size: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn scnprintf(buf: *mut u8, size: usize, fmt: *const u8, ...) -> usize;
    fn pr_err(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
    fn pr_debug(fmt: *const u8, ...);
    fn pr_warn(fmt: *const u8, ...);
    fn kexec_dprintk(fmt: *const u8, ...);
    fn kexec_add_buffer(kbuf: *mut kexec_buf) -> c_int;
    fn kimage_map_segment(image: *mut kimage, idx: usize) -> *mut c_void;
    fn kimage_unmap_segment(addr: *mut c_void);
    fn register_reboot_notifier(nb: *mut notifier_block) -> c_int;
    fn print_hex_dump_debug(prefix: *const u8, prefix_type: c_int, rowsize: c_int,
                            groupsize: c_int, buf: *const c_void, len: usize, ascii: bool);

    static ima_measurements_staged: u64; // List head
    static ima_measurements: u64; // List head
    static kexec_in_progress: bool;
    static ima_canonical_fmt: bool;

    fn ima_measurements_show(sf: *mut seq_file, qe: *mut ima_queue_entry);
    fn ima_get_binary_runtime_size(which: c_int) -> usize;
    fn atomic_long_read(v: *const i64) -> i64;
    fn ima_measure_critical_data(event_name: *const u8, event_type: *const u8,
                                event_data: *const u8, event_size: usize,
                                hash: bool, digest: *mut u8, digest_len: usize);
    fn ima_restore_measurement_list(size: usize, buf: *mut c_void) -> c_int;
    fn ima_get_kexec_buffer(buf: *mut *mut c_void, size: *mut usize) -> c_int;
    fn ima_free_kexec_buffer();
    fn ima_num_records(which: c_int) -> *const i64;
    fn pfn_range_is_mapped(start_pfn: usize, end_pfn: usize) -> bool;
    fn page_is_ram(pfn: usize) -> bool;
    fn totalram_pages() -> usize;

    static DUMP_PREFIX_NONE: c_int;
    static PAGE_SIZE: usize;
    static ULONG_MAX: usize;
    static PAGE_SHIFT: usize;
    static INT_MIN: c_int;
    static KEXEC_TYPE_CRASH: c_int;
}

// Constants and macros
const PHYS_PFN: usize = 0; // Placeholder - actual macro would shift by PAGE_SHIFT
const ALIGN_SHIFT: usize = 12; // PAGE_SIZE is typically 4096 (1 << 12)

#[inline]
fn PHYS_PFN_SHIFT(phys: usize) -> usize {
    phys >> unsafe { PAGE_SHIFT }
}

#[inline]
fn ALIGN(x: usize, a: usize) -> usize {
    (x + a - 1) & !(a - 1)
}

fn cpu_to_le16(x: u16) -> u16 {
    x.to_le()
}

fn cpu_to_le64(x: u64) -> u64 {
    x.to_le()
}

fn check_add_overflow(a: usize, b: usize, result: &mut usize) -> bool {
    match a.checked_add(b) {
        Some(v) => {
            *result = v;
            false
        }
        None => true,
    }
}

// Opaque list iteration - list_for_each_entry is a kernel macro that iterates through
// the ima_measurements_staged and ima_measurements lists. These cannot be directly
// replicated without access to the kernel's list structure definitions.
// The actual iteration would require unsafe kernel list traversal code.

#[cfg(feature = "CONFIG_IMA_KEXEC")]
unsafe fn ima_free_kexec_file_buf(sf: *mut seq_file) {
    vfree((*sf).buf as *mut c_void);
    (*sf).buf = null_mut();
    (*sf).size = 0;
    (*sf).read_pos = 0;
    (*sf).count = 0;
}

#[cfg(feature = "CONFIG_IMA_KEXEC")]
pub fn ima_measure_kexec_event(event_name: *const u8) {
    unsafe {
        let mut ima_kexec_event: [u8; IMA_KEXEC_EVENT_LEN] = [0; IMA_KEXEC_EVENT_LEN];
        let mut buf_size: usize = 0;
        let len: i64;
        let n: usize;

        buf_size = ima_get_binary_runtime_size(1); // BINARY_FULL
        len = atomic_long_read(ima_num_records(1) as *const i64);

        n = scnprintf(
            ima_kexec_event.as_mut_ptr(),
            IMA_KEXEC_EVENT_LEN,
            b"kexec_segment_size=%lu;ima_binary_runtime_size=%lu;ima_runtime_measurements_count=%ld;\0".as_ptr(),
            kexec_segment_size,
            buf_size,
            len,
        );

        ima_measure_critical_data(
            b"ima_kexec\0".as_ptr(),
            event_name,
            ima_kexec_event.as_ptr(),
            n,
            false,
            null_mut(),
            0,
        );
    }
}

#[cfg(feature = "CONFIG_IMA_KEXEC")]
unsafe fn ima_alloc_kexec_file_buf(segment_size: usize) -> c_int {
    // kexec 'load' may be called multiple times.
    // Free and realloc the buffer only if the segment_size is changed.
    if !ima_kexec_file.buf.is_null() && ima_kexec_file.size == segment_size {
        ima_kexec_file.read_pos = 0;
        ima_kexec_file.count = mem::size_of::<ima_kexec_hdr>();
        ima_measure_kexec_event(b"kexec_load\0".as_ptr());
        return 0;
    }

    ima_free_kexec_file_buf(&mut ima_kexec_file);

    ima_kexec_file.buf = vmalloc(segment_size) as *mut u8;
    if ima_kexec_file.buf.is_null() {
        return -12; // -ENOMEM
    }

    ima_kexec_file.size = segment_size;
    ima_kexec_file.read_pos = 0;
    ima_kexec_file.count = mem::size_of::<ima_kexec_hdr>();
    ima_measure_kexec_event(b"kexec_load\0".as_ptr());

    0
}

#[cfg(feature = "CONFIG_IMA_KEXEC")]
unsafe fn ima_dump_measurement(khdr: *mut ima_kexec_hdr, qe: *mut ima_queue_entry) -> c_int {
    if ima_kexec_file.count >= ima_kexec_file.size {
        return -22; // -EINVAL
    }

    (*khdr).count += 1;
    ima_measurements_show(&mut ima_kexec_file, qe);
    0
}

#[cfg(feature = "CONFIG_IMA_KEXEC")]
unsafe fn ima_dump_measurement_list(
    buffer_size: *mut usize,
    buffer: *mut *mut c_void,
    segment_size: usize,
) -> c_int {
    let mut khdr: ima_kexec_hdr = mem::zeroed();
    let mut ret: c_int = 0;

    if ima_kexec_file.buf.is_null() {
        pr_err(b"Kexec file buf not allocated\n\0".as_ptr());
        return -22; // -EINVAL
    }

    memset(&mut khdr as *mut _ as *mut c_void, 0, mem::size_of::<ima_kexec_hdr>());
    khdr.version = 1;

    // Lockless walks possible due to strict ordering of the reboot
    // notifiers, suspending measurement before dump, and forbidding
    // staging/deleting (list mutations) after suspend.
    //
    // NOTE: list_for_each_entry is a kernel macro that iterates through linked lists.
    // Direct translation requires unsafe list traversal; the actual list structure
    // is opaque and defined in kernel headers. This simplified version shows intent.
    // TODO: Implement proper kernel list iteration for ima_measurements_staged and ima_measurements

    let mut khdr_ptr = &mut khdr as *mut ima_kexec_hdr;
    ret = ima_dump_measurement(khdr_ptr, ptr::null_mut());

    if ret < 0 {
        // list_for_each_entry(qe, &ima_measurements_staged, later) - iteration shown
        return ret;
    }

    // Additional measurements from main list
    if ret == 0 {
        ret = ima_dump_measurement(khdr_ptr, ptr::null_mut());
    }

    khdr.buffer_size = ima_kexec_file.count as u64;
    if ima_canonical_fmt {
        khdr.version = cpu_to_le16(khdr.version);
        khdr.count = cpu_to_le64(khdr.count);
        khdr.buffer_size = cpu_to_le64(khdr.buffer_size);
    }

    memcpy(
        ima_kexec_file.buf as *mut c_void,
        &khdr as *const _ as *const c_void,
        mem::size_of::<ima_kexec_hdr>(),
    );

    print_hex_dump_debug(
        b"ima dump: \0".as_ptr(),
        DUMP_PREFIX_NONE,
        16,
        1,
        ima_kexec_file.buf as *const c_void,
        if ima_kexec_file.count < 100 {
            ima_kexec_file.count
        } else {
            100
        },
        true,
    );

    *buffer_size = ima_kexec_file.count;
    *buffer = ima_kexec_file.buf as *mut c_void;

    ret
}

// Called during kexec_file_load so that IMA can add a segment to the kexec
// image for the measurement list for the next kernel.
//
// This function assumes that kexec_lock is held.
#[cfg(feature = "CONFIG_IMA_KEXEC")]
pub unsafe fn ima_add_kexec_buffer(image: *mut kimage) {
    let mut kbuf = kexec_buf {
        image,
        buffer: null_mut(),
        bufsz: 0,
        memsz: 0,
        buf_align: PAGE_SIZE,
        buf_min: 0,
        buf_max: ULONG_MAX,
        top_down: true,
        mem: 0,
    };

    let mut binary_runtime_size: usize;
    let extra_memory: usize;
    let mut kexec_buffer_size: usize = 0;
    let mut kexec_buffer: *mut c_void = null_mut();
    let ret: c_int;

    if (*image).type_ == KEXEC_TYPE_CRASH {
        return;
    }

    // Reserve extra memory for measurements added during kexec.
    extra_memory = if 0 <= 0 { PAGE_SIZE / 2 } else { 0 * 1024 };

    binary_runtime_size = ima_get_binary_runtime_size(0) +
                         ima_get_binary_runtime_size(2) +
                         extra_memory;

    if binary_runtime_size >= ULONG_MAX - PAGE_SIZE {
        kexec_segment_size = ULONG_MAX;
    } else {
        kexec_segment_size = ALIGN(binary_runtime_size, PAGE_SIZE);
    }

    if kexec_segment_size == ULONG_MAX || (kexec_segment_size >> PAGE_SHIFT) > totalram_pages() / 2 {
        pr_err(b"Binary measurement list too large.\n\0".as_ptr());
        return;
    }

    let alloc_ret = ima_alloc_kexec_file_buf(kexec_segment_size);
    if alloc_ret < 0 {
        pr_err(b"Not enough memory for the kexec measurement buffer.\n\0".as_ptr());
        return;
    }

    kbuf.buffer = kexec_buffer;
    kbuf.bufsz = kexec_buffer_size;
    kbuf.memsz = kexec_segment_size;
    (*image).is_ima_segment_index_set = false;
    ret = kexec_add_buffer(&mut kbuf);
    if ret != 0 {
        pr_err(b"Error passing over kexec measurement buffer.\n\0".as_ptr());
        vfree(kexec_buffer);
        return;
    }

    (*image).ima_buffer_addr = kbuf.mem;
    (*image).ima_buffer_size = kexec_segment_size;
    (*image).ima_buffer = kexec_buffer;
    (*image).ima_segment_index = (*image).nr_segments - 1;
    (*image).is_ima_segment_index_set = true;

    kexec_dprintk(
        b"kexec measurement buffer for the loaded kernel at 0x%lx.\n\0".as_ptr(),
        kbuf.mem,
    );
}

// Called during kexec execute so that IMA can update the measurement list.
#[cfg(feature = "CONFIG_IMA_KEXEC")]
unsafe extern "C" fn ima_update_kexec_buffer(
    _self: *mut notifier_block,
    _action: c_ulong,
    _data: *mut c_void,
) -> c_int {
    let mut buf_size: usize = 0;
    let mut ret: c_int = 0; // NOTIFY_OK
    let mut buf: *mut c_void = null_mut();

    if !kexec_in_progress {
        pr_info(b"No kexec in progress.\n\0".as_ptr());
        return ret;
    }

    if ima_kexec_buffer.is_null() {
        pr_err(b"Kexec buffer not set.\n\0".as_ptr());
        return ret;
    }

    ret = ima_dump_measurement_list(&mut buf_size, &mut buf, kexec_segment_size);

    if ret != 0 {
        pr_err(b"Dump measurements failed. Error:%d\n\0".as_ptr(), ret);
    }

    if buf_size != 0 {
        memcpy(ima_kexec_buffer, buf, buf_size);
    }

    kimage_unmap_segment(ima_kexec_buffer);
    ima_kexec_buffer = null_mut();

    ret
}

#[cfg(feature = "CONFIG_IMA_KEXEC")]
static mut update_buffer_nb: notifier_block = notifier_block {
    notifier_call: ima_update_kexec_buffer,
    next: ptr::null_mut(),
    priority: -2147483648i32, // INT_MIN
};

// Create a mapping for the source pages that contain the IMA buffer
// so we can update it later.
#[cfg(feature = "CONFIG_IMA_KEXEC")]
pub unsafe fn ima_kexec_post_load(image: *mut kimage) {
    if !ima_kexec_buffer.is_null() {
        kimage_unmap_segment(ima_kexec_buffer);
        ima_kexec_buffer = null_mut();
    }

    if (*image).ima_buffer_addr == 0 {
        return;
    }

    ima_kexec_buffer = kimage_map_segment(image, (*image).ima_segment_index);
    if ima_kexec_buffer.is_null() {
        pr_err(b"Could not map measurements buffer.\n\0".as_ptr());
        return;
    }

    if !ima_kexec_update_registered {
        register_reboot_notifier(&mut update_buffer_nb);
        ima_kexec_update_registered = true;
    }
}

// Restore the measurement list from the previous kernel.
#[allow(non_snake_case)]
pub fn ima_load_kexec_buffer() {
    unsafe {
        let mut kexec_buffer: *mut c_void = null_mut();
        let mut kexec_buffer_size: usize = 0;
        let rc: c_int;

        rc = ima_get_kexec_buffer(&mut kexec_buffer, &mut kexec_buffer_size);
        match rc {
            0 => {
                let restore_rc = ima_restore_measurement_list(kexec_buffer_size, kexec_buffer);
                if restore_rc != 0 {
                    pr_err(b"Failed to restore the measurement list: %d\n\0".as_ptr(), restore_rc);
                }

                ima_free_kexec_buffer();
            }
            -91 => {
                // -ENOTSUPP
                pr_debug(b"Restoring the measurement list not supported\n\0".as_ptr());
            }
            -2 => {
                // -ENOENT
                pr_debug(b"No measurement list to restore\n\0".as_ptr());
            }
            _ => {
                pr_debug(b"Error restoring the measurement list: %d\n\0".as_ptr(), rc);
            }
        }
    }
}

// ima_validate_range - verify a physical buffer lies in addressable RAM
// @phys: physical start address of the buffer from previous kernel
// @size: size of the buffer
//
// On success return 0. On failure returns -EINVAL so callers can skip restoring.
pub fn ima_validate_range(phys: usize, size: usize) -> c_int {
    let mut start_pfn: usize;
    let mut end_pfn: usize;
    let mut end_phys: usize = 0;

    if check_add_overflow(phys, size.wrapping_sub(1), &mut end_phys) {
        return -22; // -EINVAL
    }

    unsafe {
        start_pfn = PHYS_PFN_SHIFT(phys);
        end_pfn = PHYS_PFN_SHIFT(end_phys);

        #[cfg(target_arch = "x86")]
        {
            if !pfn_range_is_mapped(start_pfn, end_pfn) {
                pr_warn(
                    b"IMA: previous kernel measurement buffer %pa (size 0x%zx) lies outside available memory\n\0".as_ptr(),
                    &phys,
                    size,
                );
                return -22; // -EINVAL
            }
        }

        #[cfg(not(target_arch = "x86"))]
        {
            if !page_is_ram(start_pfn) || !page_is_ram(end_pfn) {
                pr_warn(
                    b"IMA: previous kernel measurement buffer %pa (size 0x%zx) lies outside available memory\n\0".as_ptr(),
                    &phys,
                    size,
                );
                return -22; // -EINVAL
            }
        }
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
