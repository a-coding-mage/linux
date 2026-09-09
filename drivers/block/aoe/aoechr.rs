/* Copyright (c) 2012 Coraid, Inc.  See COPYING for GPL terms. */
/*
 * aoechr.c
 * AoE character device driver
 */

// Linux kernel and aoe.h dependencies are supplied externally.

use core::ffi::{c_char, c_int, c_void};

const MINOR_ERR: usize = 2;
const MINOR_DISCOVER: usize = 3;
const MINOR_INTERFACES: usize = 4;
const MINOR_REVALIDATE: usize = 5;
const MINOR_FLUSH: usize = 6;
const MSGSZ: usize = 2048;
const NMSG: usize = 100;
const EMFL_VALID: i16 = 1;

#[repr(C)]
struct aoe_chardev {
    minor: usize,
    name: [c_char; 32],
}

#[repr(C)]
struct ErrMsg {
    flags: i16,
    len: i16,
    msg: *mut c_char,
}

#[repr(C)]
struct device;
#[repr(C)]
struct inode;
#[repr(C)]
struct file;
#[repr(C)]
struct aoedev;
#[repr(C)]
struct sk_buff;
#[repr(C)]
struct sk_buff_head;
#[repr(C)]
struct completion;
#[repr(C)]
struct spinlock_t;
#[repr(C)]
struct mutex;
#[repr(C)]
struct class;
#[repr(C)]
struct file_operations;

type UmodeT = u16;
type LoffT = i64;
type SsizeT = isize;

extern "C" {
    static mut aoechr_mutex: mutex;
    static mut emsgs: [ErrMsg; NMSG];
    static mut emsgs_head_idx: c_int;
    static mut emsgs_tail_idx: c_int;
    static mut emsgs_comp: completion;
    static mut emsgs_lock: spinlock_t;
    static mut nblocked_emsgs_readers: c_int;

    fn kasprintf(flags: c_int, fmt: *const c_char, ...) -> *mut c_char;
    fn dev_name(dev: *const device) -> *const c_char;
    fn aoecmd_cfg(major: c_int, minor: c_int);
    fn set_aoe_iflist(s: *const c_char, size: usize) -> c_int;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn aoedev_by_aoeaddr(major: c_int, minor: c_int, flags: c_int) -> *mut aoedev;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn aoecmd_cleanslate(d: *mut aoedev);
    fn aoecmd_ata_id(d: *mut aoedev) -> *mut sk_buff;
    fn msleep_interruptible(msecs: u32) -> c_int;
    fn aoedev_put(d: *mut aoedev);
    fn __skb_queue_head_init(q: *mut sk_buff_head);
    fn __skb_queue_tail(q: *mut sk_buff_head, skb: *mut sk_buff);
    fn aoenet_xmit(q: *mut sk_buff_head);
    fn strlen(s: *const c_char) -> usize;
    fn kmemdup(src: *const c_void, len: usize, flags: c_int) -> *mut c_char;
    fn complete(c: *mut completion);
    fn kfree(p: *mut c_void);
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn iminor(i: *const inode) -> c_int;
    fn wait_for_completion_interruptible(c: *mut completion) -> c_int;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn register_chrdev(major: c_int, name: *const c_char, fops: *const file_operations) -> c_int;
    fn unregister_chrdev(major: c_int, name: *const c_char);
    fn init_completion(c: *mut completion);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn class_register(c: *const class) -> c_int;
    fn class_unregister(c: *const class);
    fn device_create(c: *const class, parent: *mut device, dev: usize, drv: *mut c_void, name: *const c_char) -> *mut device;
    fn device_destroy(c: *const class, dev: usize);
    fn aoedev_flush(buf: *const c_char, cnt: usize) -> c_int;
}

static mut chardevs: [aoe_chardev; 5] = [
    aoe_chardev { minor: MINOR_ERR, name: *b"err\0" },
    aoe_chardev { minor: MINOR_DISCOVER, name: *b"discover\0" },
    aoe_chardev { minor: MINOR_INTERFACES, name: *b"interfaces\0" },
    aoe_chardev { minor: MINOR_REVALIDATE, name: *b"revalidate\0" },
    aoe_chardev { minor: MINOR_FLUSH, name: *b"flush\0" },
];

unsafe extern "C" fn aoe_devnode(_dev: *const device, _mode: *mut UmodeT) -> *mut c_char {
    kasprintf(0, b"etherd/%s\0".as_ptr() as *const c_char, dev_name(_dev))
}

static mut aoe_class: class = unsafe { core::mem::zeroed() };

unsafe fn discover() -> c_int { aoecmd_cfg(0xffff, 0xff); 0 }

unsafe fn interfaces(s: *const c_char, size: usize) -> c_int {
    if set_aoe_iflist(s, size) != 0 { return -22; }
    0
}

unsafe fn revalidate(s: *const c_char, size: usize) -> c_int {
    let mut buf = [0i8; 16];
    if size >= buf.len() { return -22; }
    if copy_from_user(buf.as_mut_ptr() as *mut c_void, s as *const c_void, size) != 0 { return -14; }
    let d = aoedev_by_aoeaddr(0, 0, 0);
    if d.is_null() { return -22; }
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*d).lock, &mut flags);
    aoecmd_cleanslate(d);
    aoecmd_cfg(0, 0);
    loop {
        let skb = aoecmd_ata_id(d);
        spin_unlock_irqrestore(&mut (*d).lock, flags);
        if skb.is_null() && msleep_interruptible(250) == 0 {
            spin_lock_irqsave(&mut (*d).lock, &mut flags);
            continue;
        }
        aoedev_put(d);
        if !skb.is_null() {
            let mut queue: sk_buff_head = core::mem::zeroed();
            __skb_queue_head_init(&mut queue);
            __skb_queue_tail(&mut queue, skb);
            aoenet_xmit(&mut queue);
        }
        return 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn aoechr_error(msg: *mut c_char) {
    let n = strlen(msg);
    let mut flags = 0usize;
    spin_lock_irqsave(&mut emsgs_lock, &mut flags);
    let em = &mut emsgs[emsgs_tail_idx as usize];
    if em.flags & EMFL_VALID != 0 { spin_unlock_irqrestore(&mut emsgs_lock, flags); return; }
    let mp = kmemdup(msg as *const c_void, n, 0);
    if mp.is_null() { spin_unlock_irqrestore(&mut emsgs_lock, flags); return; }
    em.msg = mp; em.flags |= EMFL_VALID; em.len = n as i16;
    emsgs_tail_idx = (emsgs_tail_idx + 1) % NMSG as c_int;
    spin_unlock_irqrestore(&mut emsgs_lock, flags);
    if nblocked_emsgs_readers != 0 { complete(&mut emsgs_comp); }
}

unsafe fn aoechr_write(f: *mut file, buf: *const c_char, cnt: usize, _off: *mut LoffT) -> SsizeT {
    let minor = (*f).private_data as usize;
    let ret = match minor {
        MINOR_DISCOVER => discover(),
        MINOR_INTERFACES => interfaces(buf, cnt),
        MINOR_REVALIDATE => revalidate(buf, cnt),
        MINOR_FLUSH => aoedev_flush(buf, cnt),
        _ => -22,
    };
    if ret == 0 { cnt as SsizeT } else { ret as SsizeT }
}

unsafe fn aoechr_open(i: *mut inode, f: *mut file) -> c_int {
    mutex_lock(&mut aoechr_mutex);
    let n = iminor(i) as usize;
    (*f).private_data = n as *mut c_void;
    for d in chardevs.iter() {
        if d.minor == n { mutex_unlock(&mut aoechr_mutex); return 0; }
    }
    mutex_unlock(&mut aoechr_mutex); -22
}

unsafe fn aoechr_rel(_inode: *mut inode, _filp: *mut file) -> c_int { 0 }

unsafe fn aoechr_read(f: *mut file, buf: *mut c_char, cnt: usize, _off: *mut LoffT) -> SsizeT {
    if (*f).private_data as usize != MINOR_ERR { return -14; }
    let mut flags = 0usize;
    spin_lock_irqsave(&mut emsgs_lock, &mut flags);
    loop {
        let em = &mut emsgs[emsgs_head_idx as usize];
        if em.flags & EMFL_VALID != 0 { break; }
        if (*f).f_flags & 0x800 != 0 { spin_unlock_irqrestore(&mut emsgs_lock, flags); return -11; }
        nblocked_emsgs_readers += 1;
        spin_unlock_irqrestore(&mut emsgs_lock, flags);
        let n = wait_for_completion_interruptible(&mut emsgs_comp);
        spin_lock_irqsave(&mut emsgs_lock, &mut flags);
        nblocked_emsgs_readers -= 1;
        if n != 0 { spin_unlock_irqrestore(&mut emsgs_lock, flags); return -512; }
    }
    let em = &mut emsgs[emsgs_head_idx as usize];
    if em.len as usize > cnt { spin_unlock_irqrestore(&mut emsgs_lock, flags); return -11; }
    let mp = em.msg; let len = em.len as usize;
    em.msg = core::ptr::null_mut(); em.flags &= !EMFL_VALID;
    emsgs_head_idx = (emsgs_head_idx + 1) % NMSG as c_int;
    spin_unlock_irqrestore(&mut emsgs_lock, flags);
    let n = copy_to_user(buf as *mut c_void, mp as *const c_void, len);
    kfree(mp as *mut c_void);
    if n == 0 { len as SsizeT } else { -14 }
}

#[no_mangle]
pub unsafe extern "C" fn aoechr_init() -> c_int {
    let n = register_chrdev(0, b"aoechr\0".as_ptr() as *const c_char, core::ptr::null());
    if n < 0 { return n; }
    init_completion(&mut emsgs_comp); spin_lock_init(&mut emsgs_lock); class_register(&aoe_class)
}

#[no_mangle]
pub unsafe extern "C" fn aoechr_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
