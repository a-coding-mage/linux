// SPDX-License-Identifier: GPL-2.0
/*
 * Functions related to generic timeout handling of requests.
 */

// Dependencies supplied by the kernel block layer and related translation units.

#[cfg(CONFIG_FAIL_IO_TIMEOUT)]
static mut fail_io_timeout: FaultAttr = DECLARE_FAULT_ATTR!();

#[cfg(CONFIG_FAIL_IO_TIMEOUT)]
unsafe extern "C" fn setup_fail_io_timeout(str_: *mut core::ffi::c_char) -> i32 {
    setup_fault_attr(&raw mut fail_io_timeout, str_)
}

// __setup("fail_io_timeout=", setup_fail_io_timeout);

#[cfg(CONFIG_FAIL_IO_TIMEOUT)]
pub unsafe extern "C" fn __blk_should_fake_timeout(q: *mut request_queue) -> bool {
    should_fail(&raw mut fail_io_timeout, 1)
}

// EXPORT_SYMBOL_GPL(__blk_should_fake_timeout);

#[cfg(CONFIG_FAIL_IO_TIMEOUT)]
unsafe extern "C" fn fail_io_timeout_debugfs() -> i32 {
    let dir: *mut dentry = fault_create_debugfs_attr(
        c"fail_io_timeout".as_ptr(),
        core::ptr::null_mut(),
        &raw mut fail_io_timeout,
    );

    PTR_ERR_OR_ZERO(dir)
}

// late_initcall(fail_io_timeout_debugfs);

#[cfg(CONFIG_FAIL_IO_TIMEOUT)]
pub unsafe extern "C" fn part_timeout_show(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut core::ffi::c_char,
) -> isize {
    let disk: *mut gendisk = dev_to_disk(dev);
    let set: i32 = test_bit(QUEUE_FLAG_FAIL_IO, (*(*disk).queue).queue_flags.as_ptr());

    sprintf(buf, c"%d\n".as_ptr(), (set != 0) as i32)
}

#[cfg(CONFIG_FAIL_IO_TIMEOUT)]
pub unsafe extern "C" fn part_timeout_store(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *const core::ffi::c_char,
    count: usize,
) -> usize {
    let disk: *mut gendisk = dev_to_disk(dev);
    let mut val: i32 = 0;

    if count != 0 {
        let q: *mut request_queue = (*disk).queue;
        let mut p: *mut core::ffi::c_char = buf as *mut core::ffi::c_char;

        val = simple_strtoul(p, &mut p, 10) as i32;
        if val != 0 {
            blk_queue_flag_set(QUEUE_FLAG_FAIL_IO, q);
        } else {
            blk_queue_flag_clear(QUEUE_FLAG_FAIL_IO, q);
        }
    }

    count
}

/**
 * blk_abort_request - Request recovery for the specified command
 * @req: pointer to the request of interest
 *
 * This function requests that the block layer start recovery for the
 * request by deleting the timer and calling the q's timeout function.
 * LLDDs who implement their own error recovery MAY ignore the timeout
 * event if they generated blk_abort_request.
 */
pub unsafe extern "C" fn blk_abort_request(req: *mut request) {
    /*
     * All we need to ensure is that timeout scan takes place
     * immediately and that scan sees the new timeout value.
     * No need for fancy synchronizations.
     */
    WRITE_ONCE!((*req).deadline, jiffies);
    kblockd_schedule_work(&mut (*(*req).q).timeout_work);
}

// EXPORT_SYMBOL_GPL(blk_abort_request);

static mut blk_timeout_mask: usize = 0;

unsafe extern "C" fn blk_timeout_init() -> i32 {
    blk_timeout_mask = roundup_pow_of_two(HZ) - 1;
    0
}

// late_initcall(blk_timeout_init);

/*
 * Just a rough estimate, we don't care about specific values for timeouts.
 */
#[inline]
unsafe fn blk_round_jiffies(j: usize) -> usize {
    (j + blk_timeout_mask) + 1
}

pub unsafe extern "C" fn blk_rq_timeout(mut timeout: usize) -> usize {
    let maxt: usize;

    maxt = blk_round_jiffies(jiffies + BLK_MAX_TIMEOUT);
    if time_after(timeout, maxt) {
        timeout = maxt;
    }

    timeout
}

/**
 * blk_add_timer - Start timeout timer for a single request
 * @req: request that is about to start running.
 *
 * Notes:
 *    Each request has its own timer, and as it is added to the queue, we
 *    set up the timer. When the request completes, we cancel the timer.
 */
pub unsafe extern "C" fn blk_add_timer(req: *mut request) {
    let q: *mut request_queue = (*req).q;
    let mut expiry: usize;

    /*
     * Some LLDs, like scsi, peek at the timeout to prevent a
     * command from being retried forever.
     */
    if (*req).timeout == 0 {
        (*req).timeout = (*q).rq_timeout;
    }

    (*req).rq_flags &= !RQF_TIMED_OUT;

    expiry = jiffies + (*req).timeout;
    WRITE_ONCE!((*req).deadline, expiry);

    /*
     * If the timer isn't already pending or this timeout is earlier
     * than an existing one, modify the timer. Round up to next nearest
     * second.
     */
    expiry = blk_rq_timeout(blk_round_jiffies(expiry));

    if !timer_pending(&mut (*q).timeout)
        || time_before(expiry, (*q).timeout.expires)
    {
        let diff: usize = (*q).timeout.expires - expiry;

        /*
         * Due to added timer slack to group timers, the timer
         * will often be a little in front of what we asked for.
         * So apply some tolerance here too, otherwise we keep
         * modifying the timer because expires for value X
         * will be X + something.
         */
        if !timer_pending(&mut (*q).timeout) || diff >= HZ / 2 {
            mod_timer(&mut (*q).timeout, expiry);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
