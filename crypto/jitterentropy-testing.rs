/* SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause */
/*
 * Test interface for Jitter RNG.
 *
 * Copyright (C) 2023, Stephan Mueller <smueller@chronox.de>
 */

/* Linux kernel dependencies are supplied by the surrounding build. */

const JENT_TEST_RINGBUFFER_SIZE: usize = 1 << 10;
const JENT_TEST_RINGBUFFER_MASK: u32 = (JENT_TEST_RINGBUFFER_SIZE - 1) as u32;

#[repr(C)]
struct JentTesting {
    jent_testing_rb: [u64; JENT_TEST_RINGBUFFER_SIZE],
    rb_reader: u32,
    rb_writer: atomic_t,
    jent_testing_enabled: atomic_t,
    lock: spinlock_t,
    read_wait: wait_queue_head_t,
}

static mut jent_raw_debugfs_root: *mut dentry = core::ptr::null_mut();

/*
 * boot variable:
 * 0 ==> No boot test, gathering of runtime data allowed
 * 1 ==> Boot test enabled and ready for collecting data, gathering runtime
 *      data is disabled
 * 2 ==> Boot test completed and disabled, gathering of runtime data is
 *      disabled
 */

unsafe fn jent_testing_reset(data: *mut JentTesting) {
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&mut (*data).lock, &mut flags);
    (*data).rb_reader = 0;
    atomic_set(&mut (*data).rb_writer, 0);
    spin_unlock_irqrestore(&mut (*data).lock, flags);
}

unsafe fn jent_testing_data_init(data: *mut JentTesting, boot: u32) {
    /*
     * The boot time testing implies we have a running test. If the
     * caller wants to clear it, he has to unset the boot_test flag
     * at runtime via sysfs to enable regular runtime testing
     */
    if boot != 0 {
        return;
    }

    jent_testing_reset(data);
    atomic_set(&mut (*data).jent_testing_enabled, 1);
    pr_warn!("Enabling data collection\n");
}

unsafe fn jent_testing_fini(data: *mut JentTesting, boot: u32) {
    /* If we have boot data, we do not reset yet to allow data to be read */
    if boot != 0 {
        return;
    }

    atomic_set(&mut (*data).jent_testing_enabled, 0);
    jent_testing_reset(data);
    pr_warn!("Disabling data collection\n");
}

unsafe fn jent_testing_store(data: *mut JentTesting, value: u64, boot: *mut u32) -> bool {
    let mut flags: c_ulong = 0;

    if atomic_read(&(*data).jent_testing_enabled) == 0 && *boot != 1 {
        return false;
    }

    spin_lock_irqsave(&mut (*data).lock, &mut flags);

    /* Disable entropy testing for boot time testing after ring buffer is filled. */
    if *boot != 0 {
        if (atomic_read(&(*data).rb_writer) as u32) > JENT_TEST_RINGBUFFER_SIZE as u32 {
            *boot = 2;
            pr_warn_once!("One time data collection test disabled\n");
            spin_unlock_irqrestore(&mut (*data).lock, flags);
            return false;
        }

        if atomic_read(&(*data).rb_writer) == 1 {
            pr_warn!("One time data collection test enabled\n");
        }
    }

    (*data).jent_testing_rb[(atomic_read(&(*data).rb_writer) as u32 & JENT_TEST_RINGBUFFER_MASK) as usize] = value;
    atomic_inc(&mut (*data).rb_writer);

    spin_unlock_irqrestore(&mut (*data).lock, flags);

    if wq_has_sleeper(&(*data).read_wait) {
        wake_up_interruptible(&(*data).read_wait);
    }

    true
}

unsafe fn jent_testing_have_data(data: *mut JentTesting) -> bool {
    ((atomic_read(&(*data).rb_writer) as u32 & JENT_TEST_RINGBUFFER_MASK)
        != ((*data).rb_reader & JENT_TEST_RINGBUFFER_MASK))
}

unsafe fn jent_testing_reader(data: *mut JentTesting, boot: *mut u32, mut outbuf: *mut u8, mut outbuflen: u32) -> c_int {
    let mut flags: c_ulong = 0;
    let mut collected_data: c_int = 0;

    jent_testing_data_init(data, *boot);

    while outbuflen != 0 {
        let writer = atomic_read(&(*data).rb_writer) as u32;

        spin_lock_irqsave(&mut (*data).lock, &mut flags);

        if writer == 0 || writer == (*data).rb_reader {
            spin_unlock_irqrestore(&mut (*data).lock, flags);

            if *boot != 0 {
                *boot = 0;
                break;
            }

            wait_event_interruptible!((*data).read_wait, jent_testing_have_data(data));
            if signal_pending(current()) {
                collected_data = -ERESTARTSYS;
                break;
            }

            continue;
        }

        if outbuflen < core::mem::size_of::<u64>() as u32 {
            spin_unlock_irqrestore(&mut (*data).lock, flags);
            break;
        }

        core::ptr::copy_nonoverlapping(
            (&(*data).jent_testing_rb[(*data).rb_reader as usize]) as *const u64 as *const u8,
            outbuf,
            core::mem::size_of::<u64>(),
        );
        (*data).rb_reader = (*data).rb_reader.wrapping_add(1);

        spin_unlock_irqrestore(&mut (*data).lock, flags);

        outbuf = outbuf.add(core::mem::size_of::<u64>());
        outbuflen -= core::mem::size_of::<u64>() as u32;
        collected_data += core::mem::size_of::<u64>() as c_int;
    }

    jent_testing_fini(data, *boot);
    collected_data
}

unsafe fn jent_testing_extract_user(
    file: *mut file,
    mut buf: *mut c_char,
    mut nbytes: usize,
    ppos: *mut loff_t,
    reader: unsafe fn(*mut u8, u32) -> c_int,
) -> c_int {
    let mut ret: c_int = 0;
    let large_request = nbytes > 256;

    if nbytes == 0 {
        return 0;
    }

    let tmp = kmalloc(125 * core::mem::size_of::<u64>() + core::mem::size_of::<u64>(), GFP_KERNEL);
    if tmp.is_null() {
        return -ENOMEM;
    }

    let tmp_aligned = PTR_ALIGN(tmp, core::mem::size_of::<u64>());

    while nbytes != 0 {
        let mut i: c_int;

        if large_request && need_resched() {
            if signal_pending(current()) {
                if ret == 0 {
                    ret = -ERESTARTSYS;
                }
                break;
            }
            schedule();
        }

        i = core::cmp::min(nbytes, 125 * core::mem::size_of::<u64>()) as c_int;
        i = reader(tmp_aligned as *mut u8, i as u32);
        if i <= 0 {
            if i < 0 {
                ret = i;
            }
            break;
        }
        if copy_to_user(buf, tmp_aligned as *const u8, i as usize) != 0 {
            ret = -EFAULT;
            break;
        }

        nbytes -= i as usize;
        buf = buf.add(i as usize);
        ret += i;
    }

    kfree_sensitive(tmp);

    if ret > 0 {
        *ppos += ret as loff_t;
    }

    ret
}

/* Raw high-resolution timer entropy data handling. */

static mut boot_raw_hires_test: u32 = 0;

static mut jent_raw_hires: JentTesting = JentTesting {
    jent_testing_rb: [0; JENT_TEST_RINGBUFFER_SIZE],
    rb_reader: 0,
    rb_writer: ATOMIC_INIT(0),
    jent_testing_enabled: ATOMIC_INIT(0),
    lock: __SPIN_LOCK_UNLOCKED!(),
    read_wait: __WAIT_QUEUE_HEAD_INITIALIZER!(),
};

#[no_mangle]
pub unsafe extern "C" fn jent_raw_hires_entropy_store(value: u64) -> c_int {
    if jent_testing_store(&mut jent_raw_hires, value, &mut boot_raw_hires_test) { 1 } else { 0 }
}

unsafe fn jent_raw_hires_entropy_reader(outbuf: *mut u8, outbuflen: u32) -> c_int {
    jent_testing_reader(&mut jent_raw_hires, &mut boot_raw_hires_test, outbuf, outbuflen)
}

unsafe fn jent_raw_hires_read(file: *mut file, to: *mut c_char, count: usize, ppos: *mut loff_t) -> isize {
    jent_testing_extract_user(file, to, count, ppos, jent_raw_hires_entropy_reader) as isize
}

static jent_raw_hires_fops: file_operations = file_operations {
    owner: THIS_MODULE,
    read: Some(jent_raw_hires_read),
};

#[no_mangle]
pub unsafe extern "C" fn jent_testing_init() {
    jent_raw_debugfs_root = debugfs_create_dir(KBUILD_MODNAME, core::ptr::null_mut());
    debugfs_create_file_unsafe("jent_raw_hires", 0o400, jent_raw_debugfs_root, core::ptr::null_mut(), &jent_raw_hires_fops);
}

#[no_mangle]
pub unsafe extern "C" fn jent_testing_exit() {
    debugfs_remove_recursive(jent_raw_debugfs_root);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
