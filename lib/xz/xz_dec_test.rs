// SPDX-License-Identifier: 0BSD

/*
 * XZ decoder tester
 *
 * Author: Lasse Collin <lasse.collin@tukaani.org>
 */

// Linux kernel dependencies supplied by the surrounding translation.

#[repr(C)]
pub struct inode;
#[repr(C)]
pub struct file;
#[repr(C)]
pub struct xz_dec;
#[repr(C)]
pub struct xz_buf {
    pub in_: *mut u8,
    pub in_pos: usize,
    pub in_size: usize,
    pub out: *mut u8,
    pub out_pos: usize,
    pub out_size: usize,
}
pub type loff_t = i64;
pub type ssize_t = isize;
pub type xz_ret = i32;

extern "C" {
    fn xz_dec_reset(state: *mut xz_dec);
    fn xz_dec_run(state: *mut xz_dec, buffers: *mut xz_buf) -> xz_ret;
    fn xz_dec_init(mode: i32, dict_max: usize) -> *mut xz_dec;
    fn xz_dec_end(state: *mut xz_dec);
    fn register_chrdev(major: i32, name: *const u8, fileops: *const file_operations) -> i32;
    fn unregister_chrdev(major: i32, name: *const u8);
    fn copy_from_user(to: *mut u8, from: *const u8, size: usize) -> usize;
    fn crc32(crc: u32, buf: *const u8, size: usize) -> u32;
    fn printk(level: *const u8, message: *const u8, ...);
}

#[repr(C)]
pub struct file_operations {
    pub owner: *const u8,
    pub open: Option<unsafe fn(*mut inode, *mut file) -> i32>,
    pub release: Option<unsafe fn(*mut inode, *mut file) -> i32>,
    pub write: Option<unsafe fn(*mut file, *const u8, usize, *mut loff_t) -> ssize_t>,
}

const KERN_INFO: &[u8] = b"<6>\0";
const EBUSY: i32 = 16;
const EFAULT: i32 = 14;
const ENOSPC: i32 = 28;
const EIO: i32 = 5;
const ENOMEM: i32 = 12;
const XZ_PREALLOC: i32 = 0;
const XZ_OK: xz_ret = 0;
const XZ_STREAM_END: xz_ret = 1;
const XZ_MEMLIMIT_ERROR: xz_ret = 2;
const XZ_FORMAT_ERROR: xz_ret = 3;
const XZ_OPTIONS_ERROR: xz_ret = 4;
const XZ_DATA_ERROR: xz_ret = 5;
const XZ_BUF_ERROR: xz_ret = 6;

/* Maximum supported dictionary size */
const DICT_MAX: usize = 1 << 20;

/* Device name to pass to register_chrdev(). */
const DEVICE_NAME: &[u8] = b"xz_dec_test\0";

/* Dynamically allocated device major number */
static mut DEVICE_MAJOR: i32 = 0;

/*
 * We reuse the same decoder state, and thus can decode only one
 * file at a time.
 */
static mut DEVICE_IS_OPEN: bool = false;

/* XZ decoder state */
static mut STATE: *mut xz_dec = core::ptr::null_mut();

/*
 * Return value of xz_dec_run(). We need to avoid calling xz_dec_run() after
 * it has returned XZ_STREAM_END, so we make this static.
 */
static mut RET: xz_ret = XZ_OK;

/*
 * Input and output buffers. The input buffer is used as a temporary safe
 * place for the data coming from the userspace.
 */
static mut BUFFER_IN: [u8; 1024] = [0; 1024];
static mut BUFFER_OUT: [u8; 1024] = [0; 1024];

/*
 * Structure to pass the input and output buffers to the XZ decoder.
 * A few of the fields are never modified so we initialize them here.
 */
static mut BUFFERS: xz_buf = xz_buf {
    in_: core::ptr::addr_of_mut!(BUFFER_IN) as *mut u8,
    in_pos: 0,
    in_size: 0,
    out: core::ptr::addr_of_mut!(BUFFER_OUT) as *mut u8,
    out_pos: 0,
    out_size: 1024,
};

/*
 * CRC32 of uncompressed data. This is used to give the user a simple way
 * to check that the decoder produces correct output.
 */
static mut CRC: u32 = 0;

unsafe fn xz_dec_test_open(_i: *mut inode, _f: *mut file) -> i32 {
    if DEVICE_IS_OPEN {
        return -EBUSY;
    }

    DEVICE_IS_OPEN = true;

    xz_dec_reset(STATE);
    RET = XZ_OK;
    CRC = 0xFFFF_FFFF;

    BUFFERS.in_pos = 0;
    BUFFERS.in_size = 0;
    BUFFERS.out_pos = 0;

    printk(KERN_INFO, b"xz_dec_test: opened\n\0");
    0
}

unsafe fn xz_dec_test_release(_i: *mut inode, _f: *mut file) -> i32 {
    DEVICE_IS_OPEN = false;

    if RET == XZ_OK {
        printk(KERN_INFO, b"xz_dec_test: input was truncated\n\0");
    }

    printk(KERN_INFO, b"xz_dec_test: closed\n\0");
    0
}

/*
 * Decode the data given to us from the userspace. CRC32 of the uncompressed
 * data is calculated and is printed at the end of successful decoding. The
 * uncompressed data isn't stored anywhere for further use.
 *
 * The .xz file must have exactly one Stream and no Stream Padding. The data
 * after the first Stream is considered to be garbage.
 */
unsafe fn xz_dec_test_write(
    _file: *mut file,
    mut buf: *const u8,
    size: usize,
    _pos: *mut loff_t,
) -> ssize_t {
    let mut remaining: usize;

    if RET != XZ_OK {
        if size > 0 {
            printk(KERN_INFO, b"xz_dec_test: garbage at the end of the file\n\0");
        }
        return -ENOSPC as ssize_t;
    }

    printk(KERN_INFO, b"xz_dec_test: decoding input\n\0");

    remaining = size;
    while ((remaining > 0 || BUFFERS.out_pos == BUFFERS.out_size) && RET == XZ_OK) {
        if BUFFERS.in_pos == BUFFERS.in_size {
            BUFFERS.in_pos = 0;
            BUFFERS.in_size = core::cmp::min(remaining, BUFFER_IN.len());
            if copy_from_user(BUFFER_IN.as_mut_ptr(), buf, BUFFERS.in_size) != 0 {
                return -EFAULT as ssize_t;
            }

            buf = buf.add(BUFFERS.in_size);
            remaining -= BUFFERS.in_size;
        }

        BUFFERS.out_pos = 0;
        RET = xz_dec_run(STATE, &mut BUFFERS);
        CRC = crc32(CRC, BUFFER_OUT.as_ptr(), BUFFERS.out_pos);
    }

    match RET {
        XZ_OK => {
            printk(KERN_INFO, b"xz_dec_test: XZ_OK\n\0");
            size as ssize_t
        }
        XZ_STREAM_END => {
            printk(KERN_INFO, b"xz_dec_test: XZ_STREAM_END\n\0");
            (size - remaining - (BUFFERS.in_size - BUFFERS.in_pos)) as ssize_t
        }
        XZ_MEMLIMIT_ERROR => { printk(KERN_INFO, b"xz_dec_test: XZ_MEMLIMIT_ERROR\n\0"); -EIO as ssize_t }
        XZ_FORMAT_ERROR => { printk(KERN_INFO, b"xz_dec_test: XZ_FORMAT_ERROR\n\0"); -EIO as ssize_t }
        XZ_OPTIONS_ERROR => { printk(KERN_INFO, b"xz_dec_test: XZ_OPTIONS_ERROR\n\0"); -EIO as ssize_t }
        XZ_DATA_ERROR => { printk(KERN_INFO, b"xz_dec_test: XZ_DATA_ERROR\n\0"); -EIO as ssize_t }
        XZ_BUF_ERROR => { printk(KERN_INFO, b"xz_dec_test: XZ_BUF_ERROR\n\0"); -EIO as ssize_t }
        _ => { printk(KERN_INFO, b"xz_dec_test: Bug detected!\n\0"); -EIO as ssize_t }
    }
}

/* Allocate the XZ decoder state and register the character device. */
unsafe fn xz_dec_test_init() -> i32 {
    STATE = xz_dec_init(XZ_PREALLOC, DICT_MAX);
    if STATE.is_null() {
        return -ENOMEM;
    }

    let fileops = file_operations {
        owner: core::ptr::null(),
        open: Some(xz_dec_test_open),
        release: Some(xz_dec_test_release),
        write: Some(xz_dec_test_write),
    };
    DEVICE_MAJOR = register_chrdev(0, DEVICE_NAME.as_ptr(), &fileops);
    if DEVICE_MAJOR < 0 {
        xz_dec_end(STATE);
        return DEVICE_MAJOR;
    }

    printk(KERN_INFO, b"xz_dec_test: module loaded\n\0");
    0
}

unsafe fn xz_dec_test_exit() {
    unregister_chrdev(DEVICE_MAJOR, DEVICE_NAME.as_ptr());
    xz_dec_end(STATE);
    printk(KERN_INFO, b"xz_dec_test: module unloaded\n\0");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
