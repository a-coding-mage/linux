// SPDX-License-Identifier: GPL-2.0-only
/*
 * s390 TRNG device driver
 *
 * Driver for the TRNG (true random number generation) command
 * available via CPACF extension MSA 7 on the s390 arch.
 *
 * Copyright IBM Corp. 2017
 * Author(s): Harald Freudenberger <freude@de.ibm.com>
 */

// C headers provide the kernel types, constants, and external functions used below.

MODULE_LICENSE!("GPL v2");
MODULE_AUTHOR!("IBM Corporation");
MODULE_DESCRIPTION!("s390 CPACF TRNG device driver");

/* trng related debug feature things */

static mut debug_info: *mut debug_info_t = core::ptr::null_mut();

macro_rules! DEBUG_DBG { ($($arg:tt)*) => { debug_sprintf_event(unsafe { debug_info }, 6, $($arg)*) }; }
macro_rules! DEBUG_INFO { ($($arg:tt)*) => { debug_sprintf_event(unsafe { debug_info }, 5, $($arg)*) }; }
macro_rules! DEBUG_WARN { ($($arg:tt)*) => { debug_sprintf_event(unsafe { debug_info }, 4, $($arg)*) }; }
macro_rules! DEBUG_ERR { ($($arg:tt)*) => { debug_sprintf_event(unsafe { debug_info }, 3, $($arg)*) }; }

/* trng helpers */

static trng_dev_counter: atomic64_t = ATOMIC64_INIT!(0);
static trng_hwrng_counter: atomic64_t = ATOMIC64_INIT!(0);

/* file io functions */

unsafe extern "C" fn trng_open(inode: *mut inode, file: *mut file) -> c_int {
    nonseekable_open(inode, file)
}

unsafe extern "C" fn trng_read(
    file: *mut file,
    mut ubuf: *mut c_char,
    mut nbytes: usize,
    ppos: *mut loff_t,
) -> ssize_t {
    let mut buf = [0u8; 32];
    let mut p = buf.as_mut_ptr();
    let mut n: c_uint;
    let mut ret: ssize_t = 0;

    /*
     * use buf for requests <= sizeof(buf),
     * otherwise allocate one page and fetch
     * pagewise.
     */
    if nbytes > core::mem::size_of_val(&buf) {
        p = kmalloc(PAGE_SIZE, GFP_KERNEL) as *mut u8;
        if p.is_null() {
            return -ENOMEM as ssize_t;
        }
    }

    while nbytes != 0 {
        if need_resched() {
            if signal_pending(current) {
                if ret == 0 { ret = -ERESTARTSYS as ssize_t; }
                break;
            }
            schedule();
        }
        n = if nbytes > PAGE_SIZE { PAGE_SIZE } else { nbytes } as c_uint;
        cpacf_trng(core::ptr::null_mut(), 0, p, n);
        atomic64_add(n as i64, &trng_dev_counter);
        if copy_to_user(ubuf, p as *const c_void, n as usize) != 0 {
            ret = -EFAULT as ssize_t;
            break;
        }
        nbytes -= n as usize;
        ubuf = ubuf.add(n as usize);
        ret += n as ssize_t;
    }

    if p != buf.as_mut_ptr() { kfree(p as *mut c_void); }
    DEBUG_DBG!("trng_read()=%zd\n", ret);
    ret
}

/* sysfs */

unsafe extern "C" fn trng_counter_show(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut c_char,
) -> ssize_t {
    let dev_counter = atomic64_read(&trng_dev_counter) as u64;
    let hwrng_counter = atomic64_read(&trng_hwrng_counter) as u64;
    let arch_counter = atomic64_read(&s390_arch_random_counter) as u64;
    sysfs_emit(buf, "trng:  %llu\nhwrng: %llu\narch:  %llu\ntotal: %llu\n",
               dev_counter, hwrng_counter, arch_counter,
               dev_counter + hwrng_counter + arch_counter)
}

static DEVICE_ATTR!(byte_counter, 0444, trng_counter_show, core::ptr::null_mut());

static mut trng_dev_attrs: [*mut attribute; 2] = [
    &mut dev_attr_byte_counter.attr,
    core::ptr::null_mut(),
];

static trng_dev_attr_group: attribute_group = attribute_group { attrs: trng_dev_attrs.as_mut_ptr() };
static mut trng_dev_attr_groups: [*const attribute_group; 2] = [
    &trng_dev_attr_group,
    core::ptr::null(),
];

static trng_fops: file_operations = file_operations {
    owner: THIS_MODULE,
    open: Some(trng_open),
    release: None,
    read: Some(trng_read),
    llseek: Some(noop_llseek),
};

static mut trng_dev: miscdevice = miscdevice {
    name: b"trng\0".as_ptr() as *const c_char,
    minor: MISC_DYNAMIC_MINOR,
    mode: 0o444,
    fops: &trng_fops,
    groups: trng_dev_attr_groups.as_ptr(),
};

/* hwrng_register */

unsafe fn _trng_hwrng_read(buf: *mut u8, len: usize) {
    cpacf_trng(core::ptr::null_mut(), 0, buf, len as c_uint);
    atomic64_add(len as i64, &trng_hwrng_counter);
}

unsafe extern "C" fn trng_hwrng_data_read(rng: *mut hwrng, data: *mut u32) -> c_int {
    let len = core::mem::size_of::<u32>();
    _trng_hwrng_read(data as *mut u8, len);
    DEBUG_DBG!("trng_hwrng_data_read()=%zu\n", len);
    len as c_int
}

unsafe extern "C" fn trng_hwrng_read(
    rng: *mut hwrng, data: *mut c_void, max: usize, wait: bool,
) -> c_int {
    let len = if max <= PAGE_SIZE { max } else { PAGE_SIZE };
    _trng_hwrng_read(data as *mut u8, len);
    DEBUG_DBG!("trng_hwrng_read()=%zu\n", len);
    len as c_int
}

/*
 * hwrng register struct
 * The trng is supposed to have 100% entropy, and thus we register with a very
 * high quality value. If we ever have a better driver in the future, we should
 * change this value again when we merge this driver.
 */
static mut trng_hwrng_dev: hwrng = hwrng {
    name: b"s390-trng\0".as_ptr() as *const c_char,
    data_read: Some(trng_hwrng_data_read),
    read: Some(trng_hwrng_read),
};

/* init and exit */

unsafe fn trng_debug_init() {
    debug_info = debug_register(b"trng\0".as_ptr() as *const c_char, 1, 1, 4 * core::mem::size_of::<c_long>());
    debug_register_view(debug_info, &debug_sprintf_view);
    debug_set_level(debug_info, 3);
}

unsafe fn trng_debug_exit() { debug_unregister(debug_info); }

unsafe extern "C" fn trng_init() -> c_int {
    let mut ret: c_int;
    trng_debug_init();

    /* check if subfunction CPACF_PRNO_TRNG is available */
    if !cpacf_query_func(CPACF_PRNO, CPACF_PRNO_TRNG) {
        DEBUG_INFO!("trng_init CPACF_PRNO_TRNG not available\n");
        ret = -ENODEV;
        goto_out_dbg!(ret);
    }

    ret = misc_register(&mut trng_dev);
    if ret != 0 {
        DEBUG_WARN!("trng_init misc_register() failed rc=%d\n", ret);
        goto_out_dbg!(ret);
    }

    ret = hwrng_register(&mut trng_hwrng_dev);
    if ret != 0 {
        DEBUG_WARN!("trng_init hwrng_register() failed rc=%d\n", ret);
        misc_deregister(&mut trng_dev);
        trng_debug_exit();
        return ret;
    }

    DEBUG_DBG!("trng_init successful\n");
    return 0;

    macro_rules! goto_out_dbg { ($value:expr) => {{ trng_debug_exit(); return $value; }}; }
}

unsafe extern "C" fn trng_exit() {
    hwrng_unregister(&mut trng_hwrng_dev);
    misc_deregister(&mut trng_dev);
    trng_debug_exit();
}

module_cpu_feature_match!(S390_CPU_FEATURE_MSA, trng_init);
module_exit!(trng_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
