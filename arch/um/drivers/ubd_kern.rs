// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of ubd_kern.c. */

const UBD_SHIFT: usize = 4;
const UBD_MAX_REQUEST: usize = 8 * core::mem::size_of::<c_long>();
const DRIVER_NAME: &[u8] = b"uml-blkdev\0";
const MAX_DEV: usize = 16;
const MAX_SG: usize = 64;

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)]
pub struct io_desc {
    pub buffer: *mut c_char,
    pub length: c_ulong,
    pub sector_mask: c_ulong,
    pub cow_offset: u64,
    pub bitmap_words: [c_ulong; 2],
}

// Remaining kernel-facing declarations from the implementation source.
extern "C" {
    fn ubd_ioctl(bdev: *mut block_device, mode: c_int, cmd: c_uint, arg: c_ulong) -> c_int;
    fn ubd_getgeo(disk: *mut gendisk, geo: *mut hd_geometry) -> c_int;
    fn fake_ide_setup(s: *mut c_char) -> c_int;
    fn parse_unit(s: *mut *mut c_char) -> c_int;
    fn ubd_setup_common(s: *mut c_char, index: *mut c_int, error: *mut *mut c_char) -> c_int;
    fn ubd_setup(s: *mut c_char) -> c_int;
    fn udb_setup(s: *mut c_char) -> c_int;
    fn ubd_end_request(req: *mut io_thread_req);
    fn ubd_intr(irq: c_int, dev: *mut c_void) -> irqreturn_t;
    fn kill_io_thread();
    fn ubd_file_size(dev: *mut ubd, size: *mut u64) -> c_int;
    fn read_cow_bitmap(fd: c_int, buf: *mut c_void, offset: c_int, len: c_int) -> c_int;
    fn backing_file_mismatch(file: *mut c_char, size: u64, mtime: time64_t) -> c_int;
    fn path_requires_switch(from_cmdline: *mut c_char, from_cow: *mut c_char, cow: *mut c_char) -> c_int;
    fn open_ubd_file(file: *mut c_char, flags: *mut openflags, shared: c_int, backing: *mut *mut c_char, bitmap_offset: *mut c_int, bitmap_len: *mut c_ulong, data_offset: *mut c_int, create_cow: *mut c_int) -> c_int;
    fn create_cow_file(cow: *mut c_char, backing: *mut c_char, flags: openflags, sector: c_int, alignment: c_int, bitmap_offset: *mut c_int, bitmap_len: *mut c_ulong, data_offset: *mut c_int) -> c_int;
    fn ubd_close_dev(dev: *mut ubd);
    fn ubd_open_dev(dev: *mut ubd) -> c_int;
    fn ubd_device_release(dev: *mut device);
    fn serial_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize;
    fn ubd_attrs_are_visible(kobj: *mut c_void, attr: *mut attribute, n: c_int) -> u16;
    fn ubd_add(n: c_int, error: *mut *mut c_char) -> c_int;
    fn ubd_config(s: *mut c_char, error: *mut *mut c_char) -> c_int;
    fn ubd_get_config(name: *mut c_char, s: *mut c_char, size: c_int, error: *mut *mut c_char) -> c_int;
    fn ubd_id(s: *mut *mut c_char, start: *mut c_int, end: *mut c_int) -> c_int;
    fn ubd_remove(n: c_int, error: *mut *mut c_char) -> c_int;
    fn ubd_mc_init() -> c_int;
    fn ubd0_init() -> c_int;
    fn ubd_init() -> c_int;
    fn ubd_driver_init() -> c_int;
    fn cowify_bitmap(io_offset: u64, length: c_int, cow_mask: *mut c_ulong, cow_offset: *mut u64, bitmap: *mut c_ulong, bitmap_offset: u64, bitmap_words: *mut c_ulong, bitmap_len: u64);
    fn cowify_req(req: *mut io_thread_req, segment: *mut io_desc, offset: c_ulong, bitmap: *mut c_ulong, bitmap_offset: u64, bitmap_len: u64);
    fn ubd_map_req(dev: *mut ubd, req: *mut io_thread_req, request: *mut request);
    fn ubd_alloc_req(dev: *mut ubd, request: *mut request, desc_cnt: c_int) -> *mut io_thread_req;
    fn ubd_submit_request(dev: *mut ubd, request: *mut request) -> c_int;
    fn ubd_queue_rq(hctx: *mut blk_mq_hw_ctx, bd: *const blk_mq_queue_data) -> c_int;
    fn map_error(error_code: c_int) -> c_int;
}

#[repr(C)]
pub struct io_thread_req {
    pub req: *mut request,
    pub fds: [c_int; 2],
    pub offsets: [c_ulong; 2],
    pub offset: u64,
    pub sectorsize: c_int,
    pub error: c_int,
    pub desc_cnt: c_int,
    // io_desc has to be the last element of the struct
    pub io_desc: [io_desc; 0],
}

#[repr(C)] pub struct request { _p: [u8; 0] }
#[repr(C)] pub struct block_device { pub bd_disk: *mut gendisk }
#[repr(C)] pub struct gendisk { pub private_data: *mut c_void, pub major: c_int, pub first_minor: c_int, pub minors: c_int, pub fops: *const block_device_operations, pub disk_name: [c_char; 32] }
#[repr(C)] pub struct hd_geometry { pub heads: u8, pub sectors: u8, pub cylinders: u16, pub start: u64 }
#[repr(C)] pub struct platform_device { pub id: c_int, pub name: *const c_char, pub dev: device }
#[repr(C)] pub struct device { pub release: Option<unsafe extern "C" fn(*mut device)> }
#[repr(C)] pub struct blk_mq_tag_set { pub ops: *const blk_mq_ops, pub queue_depth: c_uint, pub numa_node: c_int, pub driver_data: *mut c_void, pub nr_hw_queues: c_uint }
#[repr(C)] pub struct spinlock_t { _p: [u8; 0] }
#[repr(C)] pub struct openflags { pub r: c_int, pub w: c_int, pub s: c_int, pub c: c_int, pub cl: c_int }
#[repr(C)] pub struct uml_stat { pub ust_dev: u64, pub ust_ino: u64 }
#[repr(C)] pub struct blk_mq_hw_ctx { pub queue: *mut request_queue }
#[repr(C)] pub struct request_queue { pub queuedata: *mut c_void }
#[repr(C)] pub struct blk_mq_queue_data { pub rq: *mut request }
#[repr(C)] pub struct blk_mq_ops { pub queue_rq: Option<unsafe extern "C" fn(*mut blk_mq_hw_ctx, *const blk_mq_queue_data) -> c_int> }
#[repr(C)] pub struct block_device_operations { pub owner: *mut c_void, pub ioctl: Option<unsafe extern "C" fn(*mut block_device, c_int, c_uint, c_ulong) -> c_int>, pub compat_ioctl: *mut c_void, pub getgeo: Option<unsafe extern "C" fn(*mut gendisk, *mut hd_geometry) -> c_int> }
#[repr(C)] pub struct queue_limits { pub max_segments: c_uint, pub seg_boundary_mask: c_ulong, pub features: c_ulong, pub max_hw_sectors: c_uint, pub max_hw_discard_sectors: c_uint, pub max_write_zeroes_sectors: c_uint }
#[repr(C)] pub struct device_attribute { pub mode: u16 }
#[repr(C)] pub struct attribute { pub mode: u16 }
#[repr(C)] pub struct attribute_group { pub attrs: *mut *mut attribute, pub is_visible: Option<unsafe extern "C" fn(*mut c_void, *mut attribute, c_int) -> u16> }
#[repr(C)] pub struct bio_vec { pub bv_len: c_uint }
#[repr(C)] pub struct req_iterator { _p: [u8; 0] }
#[repr(C)] pub struct cdrom_volctrl { pub channel0: u8, pub channel1: u8, pub channel2: u8, pub channel3: u8 }
#[repr(C)] pub struct os_helper_thread { _p: [u8; 0] }

type c_uint = u32;
type irqreturn_t = c_int;
type time64_t = i64;

#[repr(C)] pub struct cow { pub file: *mut c_char, pub fd: c_int, pub bitmap: *mut c_ulong, pub bitmap_len: c_ulong, pub bitmap_offset: c_int, pub data_offset: c_int }
#[repr(C)] pub struct ubd {
    pub file: *mut c_char, pub serial: *mut c_char, pub fd: c_int, pub size: u64,
    pub boot_openflags: openflags, pub openflags: openflags,
    pub shared: u32, pub no_cow: u32, pub no_trim: u32, pub cow: cow,
    pub pdev: platform_device, pub disk: *mut gendisk, pub tag_set: blk_mq_tag_set, pub lock: *mut spinlock_t,
}

static mut irq_req_buffer: *mut *mut io_thread_req = core::ptr::null_mut();
static mut irq_remainder: *mut io_thread_req = core::ptr::null_mut();
static mut irq_remainder_size: c_int = 0;
static mut io_req_buffer: *mut *mut io_thread_req = core::ptr::null_mut();
static mut io_remainder: *mut io_thread_req = core::ptr::null_mut();
static mut io_remainder_size: c_int = 0;
static mut thread_fd: c_int = -1;
static mut io_td: *mut os_helper_thread = core::ptr::null_mut();
pub static mut kernel_fd: c_int = -1;
static mut io_count: c_int = 0;

extern "C" {
    fn os_read_file(fd: c_int, buf: *mut c_void, len: usize) -> c_int;
    fn os_write_file(fd: c_int, buf: *const c_void, len: usize) -> c_int;
    fn os_pread_file(fd: c_int, buf: *mut c_void, len: usize, off: i64) -> c_int;
    fn os_pwrite_file(fd: c_int, buf: *const c_void, len: usize, off: u64) -> c_int;
    fn os_open_file(file: *mut c_char, flags: openflags, mode: c_int) -> c_int;
    fn os_close_file(fd: c_int); fn os_file_size(file: *mut c_char, size: *mut u64) -> c_int;
    fn os_file_modtime(file: *mut c_char, t: *mut time64_t) -> c_int; fn os_stat_file(file: *mut c_char, st: *mut uml_stat) -> c_int;
    fn os_lock_file(fd: c_int, write: c_int) -> c_int; fn os_sync_file(fd: c_int) -> c_int;
    fn os_falloc_punch(fd: c_int, off: u64, len: usize) -> c_int; fn os_falloc_zeroes(fd: c_int, off: u64, len: usize) -> c_int;
    fn os_kill_helper_thread(t: *mut os_helper_thread); fn os_fix_helper_thread_signals(); fn ubd_read_poll(fd: c_int); fn ubd_write_poll(fd: c_int);
    fn read_cow_header(reader: *mut c_void, fd: *mut c_int, version: *mut u32, backing: *mut *mut c_char, mtime: *mut time64_t, size: *mut u64, sector: *mut c_int, align: *mut u32, bitmap: *mut c_int) -> c_int;
    fn write_cow_header(file: *mut c_char, fd: c_int, backing: *mut c_char, sector: c_int, align: u32, size: *mut u64) -> c_int;
    fn cow_sizes(version: u32, size: u64, sector: c_int, align: u32, bitmap: c_int, len: *mut c_ulong, data: *mut c_int);
    fn init_cow_file(fd: c_int, cow: *mut c_char, backing: *mut c_char, sector: c_int, align: c_int, bitmap: *mut c_int, len: *mut c_ulong, data: *mut c_int) -> c_int;
    fn file_reader() -> c_int;
    fn req_op(req: *mut request) -> c_int; fn blk_rq_bytes(req: *mut request) -> usize; fn blk_rq_pos(req: *mut request) -> u64; fn blk_rq_nr_phys_segments(req: *mut request) -> c_int;
}

#[inline] unsafe fn ubd_test_bit(bit: u64, data: *mut u8) -> bool { let n = bit / 8; let off = bit % 8; (*data.add(n as usize) & (1u8 << off)) != 0 }
#[inline] unsafe fn ubd_set_bit(bit: u64, data: *mut u8) { let n = bit / 8; let off = bit % 8; *data.add(n as usize) |= 1u8 << off; }

unsafe fn bulk_req_safe_read(fd: c_int, request_buffer: *mut *mut io_thread_req, remainder: *mut *mut io_thread_req, remainder_size: *mut c_int, max_recs: usize) -> c_int {
    let mut n = 0; if *remainder_size > 0 { core::ptr::copy_nonoverlapping(remainder as *const u8, request_buffer as *mut u8, *remainder_size as usize); n = *remainder_size; }
    let res = os_read_file(fd, (request_buffer as *mut u8).add(*remainder_size as usize) as *mut c_void, core::mem::size_of::<*mut io_thread_req>() * max_recs - *remainder_size as usize);
    if res > 0 { n += res; let sz = core::mem::size_of::<*mut io_thread_req>() as c_int; if n % sz > 0 { *remainder_size = n % sz; core::ptr::copy_nonoverlapping((request_buffer as *mut u8).add((n / sz * sz) as usize), remainder as *mut u8, *remainder_size as usize); n -= *remainder_size; } } else { n = res; } n
}

unsafe fn update_bitmap(req: *mut io_thread_req, segment: *mut io_desc) -> c_int { if (*segment).cow_offset == u64::MAX { return 0; } let n = os_pwrite_file((*req).fds[1], &(*segment).bitmap_words as *const _ as *const c_void, core::mem::size_of_val(&(*segment).bitmap_words), (*segment).cow_offset); if n != core::mem::size_of_val(&(*segment).bitmap_words) as c_int { return -5; } 0 }

unsafe fn do_io(req: *mut io_thread_req, desc: *mut io_desc) {
    if req_op((*req).req) == 0 { (*req).error = -os_sync_file((*req).fds[0]); return; }
    let nsectors = (*desc).length / (*req).sectorsize as usize; let mut start = 0usize;
    while start < nsectors { let bit = ubd_test_bit(start as u64, &mut (*desc).sector_mask as *mut _ as *mut u8); let mut end = start; while end < nsectors && ubd_test_bit(end as u64, &mut (*desc).sector_mask as *mut _ as *mut u8) == bit { end += 1; }
        let off = (*req).offset + (*req).offsets[bit as usize] as u64 + (start * (*req).sectorsize as usize) as u64; let len = (end-start) * (*req).sectorsize as usize; let buf = if (*desc).buffer.is_null() { core::ptr::null_mut() } else { (*desc).buffer.add(start * (*req).sectorsize as usize) };
        let n = match req_op((*req).req) { 1 => os_pread_file((*req).fds[bit as usize], buf as *mut c_void, len, off as i64), 2 => os_pwrite_file((*req).fds[bit as usize], buf as *const c_void, len, off), 3 => os_falloc_punch((*req).fds[bit as usize], off, len), 4 => os_falloc_zeroes((*req).fds[bit as usize], off, len), _ => -95 };
        if n < 0 || (req_op((*req).req) == 2 && n as usize != len) { (*req).error = -n; return; } start = end;
    }
    (*req).offset += 0; (*req).error = update_bitmap(req, desc);
}

#[no_mangle] pub unsafe extern "C" fn io_thread(_arg: *mut c_void) -> *mut c_void {
    os_fix_helper_thread_signals(); loop { let n = bulk_req_safe_read(kernel_fd, io_req_buffer, &mut io_remainder, &mut io_remainder_size, 64); if n <= 0 { if n == -11 { ubd_read_poll(-1); } continue; } let count = n as usize / core::mem::size_of::<*mut io_thread_req>(); for i in 0..count { let req = *io_req_buffer.add(i); io_count += 1; for j in 0..(*req).desc_cnt as usize { if (*req).error == 0 { do_io(req, &mut (*req).io_desc[j]); } } } let mut written = 0; while written < n { let res = os_write_file(kernel_fd, (io_req_buffer as *mut u8).add(written as usize) as *const c_void, (n-written) as usize); if res >= 0 { written += res; } if written < n { ubd_write_poll(-1); } } }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
