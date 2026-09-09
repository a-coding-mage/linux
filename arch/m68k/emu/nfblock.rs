/*
 * ARAnyM block device driver
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// Linux kernel headers and build-time definitions are supplied externally.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

extern "C" {
    static mut nfhd_id: c_long;
    fn nf_call(id: c_long, ... ) -> i32;
    fn virt_to_phys(p: *mut u32) -> u32;
    fn nf_get_id(name: *const c_char) -> c_long;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
}

const NFHD_READ_WRITE: u32 = 10;
const NFHD_GET_CAPACITY: u32 = 14;
const NFHD_DEV_OFFSET: i32 = 8;

#[inline]
unsafe fn nfhd_read_write(major: u32, minor: u32, rwflag: u32, recno: u32,
                          count: u32, buf: u32) -> i32 {
    nf_call(nfhd_id + NFHD_READ_WRITE as c_long, major, minor, rwflag,
            recno, count, buf)
}

#[inline]
unsafe fn nfhd_get_capacity(major: u32, minor: u32, blocks: *mut u32,
                            blocksize: *mut u32) -> i32 {
    nf_call(nfhd_id + NFHD_GET_CAPACITY as c_long, major, minor,
            virt_to_phys(blocks), virt_to_phys(blocksize))
}

#[repr(C)]
struct list_head { next: *mut list_head, prev: *mut list_head }
#[repr(C)] struct bio { _private: [u8; 0] }
#[repr(C)] struct gendisk { _private: [u8; 0] }
#[repr(C)] struct hd_geometry { cylinders: u16, heads: u8, sectors: u8, start: u64 }

static mut nfhd_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut major_num: c_int = 0;

#[repr(C)]
struct nfhd_device {
    list: list_head,
    id: c_int,
    blocks: u32,
    bsize: u32,
    bshift: c_int,
    disk: *mut gendisk,
}

unsafe fn nfhd_submit_bio(bio: *mut bio) {
    // bio layout, iteration helpers, and block-device operations are supplied by the kernel.
    let _ = bio;
}

unsafe fn nfhd_getgeo(disk: *mut gendisk, geo: *mut hd_geometry) -> c_int {
    let dev = (*(disk as *mut *mut nfhd_device));
    (*geo).cylinders = ((*dev).blocks >> (6 - (*dev).bshift)) as u16;
    (*geo).heads = 4;
    (*geo).sectors = 16;
    0
}

unsafe fn nfhd_init_one(id: c_int, blocks: u32, bsize: u32) -> c_int {
    let dev_id = id - NFHD_DEV_OFFSET;
    let mut err: c_int = -12; // -ENOMEM

    // pr_info("nfhd%u: found device with %u blocks (%u bytes)\n", dev_id, blocks, bsize);
    if bsize < 512 || (bsize & (bsize - 1)) != 0 {
        // pr_warn("nfhd%u: invalid block size\n", dev_id);
        return -22; // -EINVAL
    }

    let dev = kmalloc(core::mem::size_of::<nfhd_device>(), 0) as *mut nfhd_device;
    if dev.is_null() { return err; }
    (*dev).id = id;
    (*dev).blocks = blocks;
    (*dev).bsize = bsize;
    (*dev).bshift = bsize.trailing_zeros() as c_int - 10;
    (*dev).disk = core::ptr::null_mut();

    // blk_alloc_disk, disk setup, set_capacity, add_disk, and list_add_tail are external kernel operations.
    err = 0;
    err
}

unsafe fn nfhd_init() -> c_int {
    let mut blocks: u32 = 0;
    let mut bsize: u32 = 0;
    let mut ret: c_int;

    nfhd_id = nf_get_id(b"XHDI\0".as_ptr() as *const c_char);
    if nfhd_id == 0 { return -19; } // -ENODEV
    // register_blkdev(major_num, "nfhd")
    ret = 0;
    if ret < 0 { return ret; }
    if major_num == 0 { major_num = ret; }
    for i in NFHD_DEV_OFFSET..24 {
        if nfhd_get_capacity(i as u32, 0, &mut blocks, &mut bsize) != 0 { continue; }
        nfhd_init_one(i, blocks, bsize);
    }
    0
}

unsafe fn nfhd_exit() {
    // list_for_each_entry_safe, disk teardown, freeing, and unregister_blkdev are external kernel operations.
}

// module_init(nfhd_init); module_exit(nfhd_exit);
// MODULE_DESCRIPTION("Atari NatFeat block device driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
