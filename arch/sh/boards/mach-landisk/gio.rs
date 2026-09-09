// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/boards/landisk/gio.c - driver for landisk
 *
 * This driver will also support the I-O DATA Device, Inc. LANDISK Board.
 * LANDISK and USL-5P Button, LED and GIO driver drive function.
 *
 *   Copylight (C) 2006 kogiidena
 *   Copylight (C) 2002 Atom Create Engineering Co., Ltd. *
 */

// C headers and machine headers are supplied by the surrounding kernel translation.

const DEVCOUNT: u32 = 4;
const GIO_MINOR: u32 = 2; // GIO minor no.

type DevT = u64;

#[repr(C)]
pub struct Inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct File {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CDev {
    pub ops: *const FileOperations,
}

#[repr(C)]
pub struct FileOperations {
    pub owner: *const core::ffi::c_void,
    pub open: Option<unsafe extern "C" fn(*mut Inode, *mut File) -> i32>,
    pub release: Option<unsafe extern "C" fn(*mut Inode, *mut File) -> i32>,
    pub unlocked_ioctl:
        Option<unsafe extern "C" fn(*mut File, u32, usize) -> isize>,
    pub llseek: Option<unsafe extern "C" fn(*mut File, i64, i32) -> i64>,
}

extern "C" {
    static THIS_MODULE: core::ffi::c_void;
    fn iminor(inode: *mut Inode) -> i32;
    fn preempt_disable();
    fn preempt_enable();
    fn copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn __raw_writeb(value: u8, addr: usize);
    fn __raw_writew(value: u16, addr: usize);
    fn __raw_writel(value: u32, addr: usize);
    fn __raw_readb(addr: usize) -> u8;
    fn __raw_readw(addr: usize) -> u16;
    fn __raw_readl(addr: usize) -> u32;
    fn printk(fmt: *const u8, ...);
    fn alloc_chrdev_region(dev: *mut DevT, firstminor: u32, count: u32, name: *const u8) -> i32;
    fn cdev_alloc() -> *mut CDev;
    fn cdev_add(cdev: *mut CDev, dev: DevT, count: u32) -> i32;
    fn cdev_del(cdev: *mut CDev);
    fn unregister_chrdev_region(dev: DevT, count: u32);
    fn noop_llseek(file: *mut File, offset: i64, whence: i32) -> i64;
}

extern "C" {
    static GIODRV_IOCSGIOSETADDR: u32;
    static GIODRV_IOCSGIODATA1: u32;
    static GIODRV_IOCSGIODATA2: u32;
    static GIODRV_IOCSGIODATA4: u32;
    static GIODRV_IOCGGIODATA1: u32;
    static GIODRV_IOCGGIODATA2: u32;
    static GIODRV_IOCGGIODATA4: u32;
}

static mut dev: DevT = 0;
static mut cdev_p: *mut CDev = core::ptr::null_mut();
static mut openCnt: i32 = 0;

unsafe extern "C" fn gio_open(inode: *mut Inode, _filp: *mut File) -> i32 {
    let minor = iminor(inode);
    let mut ret: i32 = -2; // -ENOENT

    preempt_disable();
    if minor < DEVCOUNT as i32 {
        if openCnt > 0 {
            ret = -114; // -EALREADY
        } else {
            openCnt += 1;
            ret = 0;
        }
    }
    preempt_enable();
    ret
}

unsafe extern "C" fn gio_close(inode: *mut Inode, _filp: *mut File) -> i32 {
    let minor = iminor(inode);

    if minor < DEVCOUNT as i32 {
        openCnt -= 1;
    }
    0
}

unsafe extern "C" fn gio_ioctl(filp: *mut File, cmd: u32, arg: usize) -> isize {
    let _ = filp;
    let mut data: u32 = 0;
    static mut addr: u32 = 0;

    if (cmd & 0x01) != 0 {
        if copy_from_user(
            &mut data as *mut u32 as *mut core::ffi::c_void,
            arg as *const core::ffi::c_void,
            core::mem::size_of::<i32>(),
        ) != 0 {
            return -14; // -EFAULT
        }
    }

    match cmd {
        x if x == GIODRV_IOCSGIOSETADDR => addr = data,
        x if x == GIODRV_IOCSGIODATA1 => __raw_writeb((0x0ff & data) as u8, addr as usize),
        x if x == GIODRV_IOCSGIODATA2 => {
            if (addr & 0x01) != 0 { return -14; }
            __raw_writew((0x0ffff & data) as u16, addr as usize);
        }
        x if x == GIODRV_IOCSGIODATA4 => {
            if (addr & 0x03) != 0 { return -14; }
            __raw_writel(data, addr as usize);
        }
        x if x == GIODRV_IOCGGIODATA1 => data = __raw_readb(addr as usize) as u32,
        x if x == GIODRV_IOCGGIODATA2 => {
            if (addr & 0x01) != 0 { return -14; }
            data = __raw_readw(addr as usize) as u32;
        }
        x if x == GIODRV_IOCGGIODATA4 => {
            if (addr & 0x03) != 0 { return -14; }
            data = __raw_readl(addr as usize);
        }
        _ => return -14,
    }

    if (cmd & 0x01) == 0 {
        if copy_to_user(
            arg as *mut core::ffi::c_void,
            &data as *const u32 as *const core::ffi::c_void,
            core::mem::size_of::<i32>(),
        ) != 0 {
            return -14;
        }
    }
    0
}

static gio_fops: FileOperations = FileOperations {
    owner: unsafe { &THIS_MODULE as *const _ as *const core::ffi::c_void },
    open: Some(gio_open),
    release: Some(gio_close),
    unlocked_ioctl: Some(gio_ioctl),
    llseek: Some(noop_llseek),
};

unsafe extern "C" fn gio_init() -> i32 {
    let mut error: i32;

    openCnt = 0;

    error = alloc_chrdev_region(&mut dev, 0, DEVCOUNT, b"gio\0".as_ptr());
    if error < 0 {
        return 1;
    }

    cdev_p = cdev_alloc();
    (*cdev_p).ops = &gio_fops;
    error = cdev_add(cdev_p, dev, DEVCOUNT);
    if error != 0 {
        return 1;
    }

    0
}

unsafe extern "C" fn gio_exit() {
    cdev_del(cdev_p);
    unregister_chrdev_region(dev, DEVCOUNT);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
