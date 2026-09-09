// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2019 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */
// Dependency declarations supplied by the surrounding kernel/XFS sources are
// intentionally omitted; the corresponding C headers are included here only
// as source-level dependency intent.

/*
 * Parallel Work Queue
 * ===================
 *
 * Abstract away the details of running a large and "obviously" parallelizable
 * task across multiple CPUs.  Callers initialize the pwork control object with
 * a desired level of parallelization and a work function.  Next, they embed
 * struct xfs_pwork in whatever structure they use to pass work context to a
 * worker thread and queue that pwork.  The work function will be passed the
 * pwork item when it is run (from process context) and any returned error will
 * be recorded in xfs_pwork_ctl.error.  Work functions should check for errors
 * and abort if necessary; the non-zeroness of xfs_pwork_ctl.error does not
 * stop workqueue item processing.
 *
 * This is the rough equivalent of the xfsprogs workqueue code, though we can't
 * reuse that name here.
 */

/* Invoke our caller's function. */
unsafe extern "C" fn xfs_pwork_work(work: *mut work_struct) {
    let pwork: *mut xfs_pwork = container_of(work, xfs_pwork_work_field_offset());
    let pctl: *mut xfs_pwork_ctl = (*pwork).pctl;
    let error: i32 = ((*pctl).work_fn)((*pctl).mp, pwork);
    if error != 0 && (*pctl).error == 0 {
        (*pctl).error = error;
    }
    if atomic_dec_and_test(&mut (*pctl).nr_work) {
        wake_up(&mut (*pctl).poll_wait);
    }
}

/*
 * Set up control data for parallel work.  @work_fn is the function that will
 * be called.  @tag will be written into the kernel threads.  @nr_threads is
 * the level of parallelism desired, or 0 for no limit.
 */
unsafe extern "C" fn xfs_pwork_init(
    mp: *mut xfs_mount,
    pctl: *mut xfs_pwork_ctl,
    work_fn: xfs_pwork_work_fn,
    tag: *const c_char,
) -> i32 {
    let mut nr_threads: u32 = 0;

    // #ifdef DEBUG
    if xfs_globals.pwork_threads >= 0 {
        nr_threads = xfs_globals.pwork_threads as u32;
    }
    // #endif
    trace_xfs_pwork_init(mp, nr_threads, current_pid());

    (*pctl).wq = alloc_workqueue(
        "%s-%d\0".as_ptr() as *const c_char,
        WQ_UNBOUND | WQ_SYSFS | WQ_FREEZABLE,
        nr_threads,
        tag,
        current_pid(),
    );
    if (*pctl).wq.is_null() {
        return -ENOMEM;
    }
    (*pctl).work_fn = work_fn;
    (*pctl).error = 0;
    (*pctl).mp = mp;
    atomic_set(&mut (*pctl).nr_work, 0);
    init_waitqueue_head(&mut (*pctl).poll_wait);

    0
}

/* Queue some parallel work. */
unsafe extern "C" fn xfs_pwork_queue(
    pctl: *mut xfs_pwork_ctl,
    pwork: *mut xfs_pwork,
) {
    init_work(&mut (*pwork).work, Some(xfs_pwork_work));
    (*pwork).pctl = pctl;
    atomic_inc(&mut (*pctl).nr_work);
    queue_work((*pctl).wq, &mut (*pwork).work);
}

/* Wait for the work to finish and tear down the control structure. */
unsafe extern "C" fn xfs_pwork_destroy(pctl: *mut xfs_pwork_ctl) -> i32 {
    destroy_workqueue((*pctl).wq);
    (*pctl).wq = core::ptr::null_mut();
    (*pctl).error
}

/*
 * Wait for the work to finish by polling completion status and touch the soft
 * lockup watchdog.  This is for callers such as mount which hold locks.
 */
unsafe extern "C" fn xfs_pwork_poll(pctl: *mut xfs_pwork_ctl) {
    while wait_event_timeout(
        &mut (*pctl).poll_wait,
        atomic_read(&(*pctl).nr_work) == 0,
        HZ,
    ) == 0 {
        touch_softlockup_watchdog();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
