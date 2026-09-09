// SPDX-License-Identifier: GPL-2.0-only
/*
 * /dev/mcelog driver
 *
 * K8 parts Copyright 2002,2003 Andi Kleen, SuSE Labs.
 * Rest from unknown author(s).
 * 2004 Andi Kleen. Rewrote most of it.
 * Copyright 2008 Intel Corporation
 * Author: Andi Kleen
 */

// Linux kernel dependencies supplied by other translation units.

static mut MCE_INJECTOR_CHAIN: BlockingNotifierHead = BlockingNotifierHead::new();
static mut MCE_CHRDEV_READ_MUTEX: Mutex = Mutex::new();

static mut MCE_HELPER: [c_char; 128] = [0; 128];
static mut MCE_HELPER_ARGV: [*mut c_char; 2] = [core::ptr::null_mut(), core::ptr::null_mut()];

/*
 * Lockless MCE logging infrastructure.
 * This avoids deadlocks on printk locks without having to break locks. Also
 * separate MCEs from kernel messages to avoid bogus bug reports.
 */

static mut MCELOG: *mut MceLogBuffer = core::ptr::null_mut();
static mut MCE_CHRDEV_WAIT: WaitQueueHead = WaitQueueHead::new();

unsafe extern "C" fn dev_mce_log(_nb: *mut NotifierBlock, _val: c_ulong, data: *mut c_void) -> c_int {
    let mce = data as *mut Mce;
    let entry: c_uint;

    if (*mce).kflags & MCE_HANDLED_CEC != 0 {
        return NOTIFY_DONE;
    }

    mutex_lock(&raw mut MCE_CHRDEV_READ_MUTEX);
    entry = (*MCELOG).next;

    /*
     * When the buffer fills up discard new entries. Assume that the
     * earlier errors are the more interesting ones:
     */
    if entry >= (*MCELOG).len {
        set_bit(MCE_OVERFLOW, &raw mut (*MCELOG).flags as *mut c_ulong);
        mutex_unlock(&raw mut MCE_CHRDEV_READ_MUTEX);
        if BOOT_CPU_DATA.x86_vendor != X86_VENDOR_AMD {
            (*mce).kflags |= MCE_HANDLED_MCELOG;
        }
        return NOTIFY_OK;
    }

    (*MCELOG).next = entry + 1;
    memcpy((*MCELOG).entry.add(entry as usize) as *mut c_void, mce as *const c_void, core::mem::size_of::<Mce>());
    (*MCELOG).entry.add(entry as usize).write((*MCELOG).entry.add(entry as usize).read());
    (*MCELOG).entry.add(entry as usize).as_mut().unwrap().finished = 1;
    (*MCELOG).entry.add(entry as usize).as_mut().unwrap().kflags = 0;

    /* wake processes polling /dev/mcelog */
    wake_up_interruptible(&raw mut MCE_CHRDEV_WAIT);
    mutex_unlock(&raw mut MCE_CHRDEV_READ_MUTEX);

    if BOOT_CPU_DATA.x86_vendor != X86_VENDOR_AMD {
        (*mce).kflags |= MCE_HANDLED_MCELOG;
    }
    NOTIFY_OK
}

static mut DEV_MCELOG_NB: NotifierBlock = NotifierBlock {
    notifier_call: Some(dev_mce_log),
    priority: MCE_PRIO_MCELOG,
};

unsafe extern "C" fn mce_do_trigger(_work: *mut WorkStruct) {
    call_usermodehelper(MCE_HELPER.as_mut_ptr(), MCE_HELPER_ARGV.as_mut_ptr(), core::ptr::null_mut(), UMH_NO_WAIT);
}

static mut MCE_TRIGGER_WORK: WorkStruct = WorkStruct::new(mce_do_trigger);

pub unsafe extern "C" fn mce_work_trigger() {
    if MCE_HELPER[0] != 0 {
        schedule_work(&raw mut MCE_TRIGGER_WORK);
    }
}

unsafe extern "C" fn show_trigger(_s: *mut Device, _attr: *mut DeviceAttribute, buf: *mut c_char) -> isize {
    strcpy(buf, MCE_HELPER.as_ptr());
    strcat(buf, b"\n\0".as_ptr() as *const c_char);
    strlen(MCE_HELPER.as_ptr()) as isize + 1
}

unsafe extern "C" fn set_trigger(_s: *mut Device, _attr: *mut DeviceAttribute, buf: *const c_char, _siz: usize) -> isize {
    strscpy(MCE_HELPER.as_mut_ptr(), buf, MCE_HELPER.len());
    let p = strchr(MCE_HELPER.as_mut_ptr(), b'\n' as c_int);
    if !p.is_null() { *p = 0; }
    strlen(MCE_HELPER.as_ptr()) as isize + (!p.is_null()) as isize
}

static TRIGGER: DeviceAttribute = device_attr!(trigger, 0o644, show_trigger, set_trigger);

/* mce_chrdev: Character device /dev/mcelog to read and clear the MCE log. */
static mut MCE_CHRDEV_STATE_LOCK: SpinLock = SpinLock::new();
static mut MCE_CHRDEV_OPEN_COUNT: c_int = 0; /* #times opened */
static mut MCE_CHRDEV_OPEN_EXCLU: c_int = 0; /* already open exclusive? */

unsafe extern "C" fn mce_chrdev_open(inode: *mut Inode, file: *mut File) -> c_int {
    spin_lock(&raw mut MCE_CHRDEV_STATE_LOCK);
    if MCE_CHRDEV_OPEN_EXCLU != 0 || (MCE_CHRDEV_OPEN_COUNT != 0 && (*file).f_flags & O_EXCL != 0) {
        spin_unlock(&raw mut MCE_CHRDEV_STATE_LOCK); return -EBUSY;
    }
    if (*file).f_flags & O_EXCL != 0 { MCE_CHRDEV_OPEN_EXCLU = 1; }
    MCE_CHRDEV_OPEN_COUNT += 1;
    spin_unlock(&raw mut MCE_CHRDEV_STATE_LOCK);
    nonseekable_open(inode, file)
}

unsafe extern "C" fn mce_chrdev_release(_inode: *mut Inode, _file: *mut File) -> c_int {
    spin_lock(&raw mut MCE_CHRDEV_STATE_LOCK);
    MCE_CHRDEV_OPEN_COUNT -= 1; MCE_CHRDEV_OPEN_EXCLU = 0;
    spin_unlock(&raw mut MCE_CHRDEV_STATE_LOCK); 0
}

static mut MCE_APEI_READ_DONE: c_int = 0;

/* Collect MCE record of previous boot in persistent storage via APEI ERST. */
unsafe fn __mce_read_apei(ubuf: *mut *mut c_char, usize_: usize) -> c_int {
    if usize_ < core::mem::size_of::<Mce>() { return -EINVAL; }
    let mut record_id: u64 = 0; let mut m = Mce::default();
    let mut rc = apei_read_mce(&mut m, &mut record_id);
    if rc <= 0 { MCE_APEI_READ_DONE = 1; if rc == -ENODEV { return 0; } return rc; }
    rc = -EFAULT;
    if copy_to_user(*ubuf, &m as *const Mce as *const c_void, core::mem::size_of::<Mce>()) != 0 { return rc; }
    rc = apei_clear_mce(record_id); if rc != 0 { MCE_APEI_READ_DONE = 1; return rc; }
    *ubuf = (*ubuf).add(core::mem::size_of::<Mce>()); 0
}

unsafe extern "C" fn mce_chrdev_read(_filp: *mut File, ubuf: *mut c_char, usize_: usize, off: *mut LoffT) -> isize {
    let mut buf = ubuf; let mut err: c_int;
    mutex_lock(&raw mut MCE_CHRDEV_READ_MUTEX);
    if MCE_APEI_READ_DONE == 0 { err = __mce_read_apei(&mut buf, usize_); if err != 0 || buf != ubuf { mutex_unlock(&raw mut MCE_CHRDEV_READ_MUTEX); return if err != 0 { err as isize } else { buf.offset_from(ubuf) }; } }
    err = -EINVAL;
    if *off != 0 || usize_ < (*MCELOG).len as usize * core::mem::size_of::<Mce>() { mutex_unlock(&raw mut MCE_CHRDEV_READ_MUTEX); return err as isize; }
    let next = (*MCELOG).next;
    err = 0;
    for i in 0..next { let m = (*MCELOG).entry.add(i as usize); err |= copy_to_user(buf, m as *const c_void, core::mem::size_of::<Mce>()) as c_int; buf = buf.add(core::mem::size_of::<Mce>()); }
    memset((*MCELOG).entry as *mut c_void, 0, next as usize * core::mem::size_of::<Mce>()); (*MCELOG).next = 0;
    if err != 0 { err = -EFAULT; }
    mutex_unlock(&raw mut MCE_CHRDEV_READ_MUTEX); if err != 0 { err as isize } else { buf.offset_from(ubuf) }
}

unsafe extern "C" fn mce_chrdev_poll(file: *mut File, wait: *mut PollTable) -> __PollT {
    poll_wait(file, &raw mut MCE_CHRDEV_WAIT, wait);
    if READ_ONCE((*MCELOG).next) != 0 || (MCE_APEI_READ_DONE == 0 && apei_check_mce() != 0) { return EPOLLIN | EPOLLRDNORM; } 0
}

unsafe extern "C" fn mce_chrdev_ioctl(_f: *mut File, cmd: c_uint, arg: c_ulong) -> c_long {
    let p = arg as *mut c_int;
    if !capable(CAP_SYS_ADMIN) { return -EPERM as c_long; }
    match cmd { MCE_GET_RECORD_LEN => put_user(core::mem::size_of::<Mce>() as c_int, p), MCE_GET_LOG_LEN => put_user((*MCELOG).len as c_int, p), MCE_GETCLEAR_FLAGS => put_user(xchg(&raw mut (*MCELOG).flags, 0), p), _ => -ENOTTY as c_long }
}

pub unsafe extern "C" fn mce_register_injector_chain(nb: *mut NotifierBlock) { blocking_notifier_chain_register(&raw mut MCE_INJECTOR_CHAIN, nb); }
pub unsafe extern "C" fn mce_unregister_injector_chain(nb: *mut NotifierBlock) { blocking_notifier_chain_unregister(&raw mut MCE_INJECTOR_CHAIN, nb); }

unsafe extern "C" fn mce_chrdev_write(_filp: *mut File, ubuf: *const c_char, mut usize_: usize, _off: *mut LoffT) -> isize {
    let mut m = Mce::default();
    if !capable(CAP_SYS_ADMIN) { return -EPERM as isize; }
    if !boot_cpu_has(X86_FEATURE_MCE) || !boot_cpu_has(X86_FEATURE_MCA) { return -EIO as isize; }
    if usize_ > core::mem::size_of::<Mce>() { usize_ = core::mem::size_of::<Mce>(); }
    if copy_from_user(&mut m as *mut Mce as *mut c_void, ubuf, usize_) != 0 { return -EFAULT as isize; }
    if m.extcpu >= num_possible_cpus() || !cpu_online(m.extcpu) { return -EINVAL as isize; }
    schedule_timeout(2); blocking_notifier_call_chain(&raw mut MCE_INJECTOR_CHAIN, 0, &mut m); usize_ as isize
}

static MCE_CHRDEV_OPS: FileOperations = FileOperations::new(mce_chrdev_open, mce_chrdev_release, mce_chrdev_read, mce_chrdev_write, mce_chrdev_poll, mce_chrdev_ioctl, compat_ptr_ioctl);
static mut MCE_CHRDEV_DEVICE: MiscDevice = MiscDevice { minor: MISC_MCELOG_MINOR, name: b"mcelog\0".as_ptr(), fops: &MCE_CHRDEV_OPS };

unsafe extern "C" fn dev_mcelog_init_device() -> c_int {
    let mce_log_len = max(MCE_LOG_MIN_LEN, num_online_cpus());
    MCELOG = kzalloc_flex(mce_log_len); if MCELOG.is_null() { return -ENOMEM; }
    memcpy((*MCELOG).signature.as_mut_ptr() as *mut c_void, MCE_LOG_SIGNATURE.as_ptr() as *const c_void, (*MCELOG).signature.len());
    (*MCELOG).len = mce_log_len; (*MCELOG).recordlen = core::mem::size_of::<Mce>();
    let err = misc_register(&raw mut MCE_CHRDEV_DEVICE);
    if err != 0 { if err == -EBUSY { pr_info!("Unable to init device /dev/mcelog, already registered"); } else { pr_err!("Unable to init device /dev/mcelog (rc: %d)\n", err); } kfree(MCELOG as *mut c_void); return err; }
    mce_register_decode_chain(&raw mut DEV_MCELOG_NB); 0
}

device_initcall_sync!(dev_mcelog_init_device);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
