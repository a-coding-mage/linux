// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * eCryptfs: Linux filesystem encryption layer
 *
 * Copyright (C) 2008 International Business Machines Corp.
 *   Author(s): Michael A. Halcrow <mahalcro@us.ibm.com>
 */

// Dependencies supplied by the Linux kernel and eCryptfs headers.

#[repr(C)]
pub struct ecryptfs_open_req {
    pub lower_file: *mut *mut file,
    pub path: path,
    pub done: completion,
    pub kthread_ctl_list: list_head,
}

#[repr(C)]
struct ecryptfs_kthread_ctl {
    pub flags: u32,
    pub mux: mutex,
    pub req_list: list_head,
    pub wait: wait_queue_head_t,
}

const ECRYPTFS_KTHREAD_ZOMBIE: u32 = 0x00000001;

static mut ecryptfs_kthread_ctl: ecryptfs_kthread_ctl = ecryptfs_kthread_ctl {
    flags: 0,
    mux: mutex {},
    req_list: list_head {},
    wait: wait_queue_head_t {},
};

static mut ecryptfs_kthread: *mut task_struct = core::ptr::null_mut();

extern "C" {
    fn ecryptfs_threadfn(ignored: *mut core::ffi::c_void) -> i32;
}

/**
 * ecryptfs_threadfn
 * @ignored: ignored
 *
 * The eCryptfs kernel thread that has the responsibility of getting
 * the lower file with RW permissions.
 *
 * Returns zero on success; non-zero otherwise
 */
unsafe fn ecryptfs_threadfn_impl(_ignored: *mut core::ffi::c_void) -> i32 {
    set_freezable();
    loop {
        let mut req: *mut ecryptfs_open_req;

        wait_event_freezable(
            &mut ecryptfs_kthread_ctl.wait,
            (!list_empty(&ecryptfs_kthread_ctl.req_list)
                || kthread_should_stop()),
        );
        mutex_lock(&mut ecryptfs_kthread_ctl.mux);
        if (ecryptfs_kthread_ctl.flags & ECRYPTFS_KTHREAD_ZOMBIE) != 0 {
            mutex_unlock(&mut ecryptfs_kthread_ctl.mux);
            break;
        }
        while !list_empty(&ecryptfs_kthread_ctl.req_list) {
            req = list_first_entry(
                &mut ecryptfs_kthread_ctl.req_list,
                ecryptfs_open_req,
                kthread_ctl_list,
            );
            list_del(&mut (*req).kthread_ctl_list);
            *(*req).lower_file = dentry_open(
                &mut (*req).path,
                O_RDWR | O_LARGEFILE,
                current_cred(),
            );
            complete(&mut (*req).done);
        }
        mutex_unlock(&mut ecryptfs_kthread_ctl.mux);
    }
    0
}

pub unsafe fn ecryptfs_init_kthread() -> i32 {
    let mut rc: i32 = 0;

    mutex_init(&mut ecryptfs_kthread_ctl.mux);
    init_waitqueue_head(&mut ecryptfs_kthread_ctl.wait);
    INIT_LIST_HEAD(&mut ecryptfs_kthread_ctl.req_list);
    ecryptfs_kthread = kthread_run(
        ecryptfs_threadfn_impl,
        core::ptr::null_mut(),
        b"ecryptfs-kthread\\0".as_ptr() as *const i8,
    );
    if IS_ERR(ecryptfs_kthread) {
        rc = PTR_ERR(ecryptfs_kthread);
        printk!(KERN_ERR, "%s: Failed to create kernel thread; rc = [%d]\n", __func__, rc);
    }
    rc
}

pub unsafe fn ecryptfs_destroy_kthread() {
    let mut req: *mut ecryptfs_open_req;
    let mut tmp: *mut ecryptfs_open_req;

    mutex_lock(&mut ecryptfs_kthread_ctl.mux);
    ecryptfs_kthread_ctl.flags |= ECRYPTFS_KTHREAD_ZOMBIE;
    list_for_each_entry_safe!(req, tmp, &mut ecryptfs_kthread_ctl.req_list, kthread_ctl_list, {
        list_del(&mut (*req).kthread_ctl_list);
        *(*req).lower_file = ERR_PTR(-EIO);
        complete(&mut (*req).done);
    });
    mutex_unlock(&mut ecryptfs_kthread_ctl.mux);
    kthread_stop(ecryptfs_kthread);
    wake_up(&mut ecryptfs_kthread_ctl.wait);
}

/**
 * ecryptfs_privileged_open
 * @lower_file: Result of dentry_open by root on lower dentry
 * @lower_dentry: Lower dentry for file to open
 * @lower_mnt: Lower vfsmount for file to open
 * @cred: credential to use for this call
 *
 * This function gets a r/w file opened against the lower dentry.
 *
 * Returns zero on success; non-zero otherwise
 */
pub unsafe fn ecryptfs_privileged_open(
    lower_file: *mut *mut file,
    lower_dentry: *mut dentry,
    lower_mnt: *mut vfsmount,
    cred: *const cred,
) -> i32 {
    let mut req: ecryptfs_open_req = core::mem::zeroed();
    let mut flags: i32 = O_LARGEFILE;
    let mut rc: i32 = 0;

    init_completion(&mut req.done);
    req.lower_file = lower_file;
    req.path.dentry = lower_dentry;
    req.path.mnt = lower_mnt;

    /* Corresponding dput() and mntput() are done when the
     * lower file is fput() when all eCryptfs files for the inode are
     * released. */
    flags |= if IS_RDONLY(d_inode(lower_dentry)) { O_RDONLY } else { O_RDWR };
    *lower_file = dentry_open(&mut req.path, flags, cred);
    if !IS_ERR(*lower_file) {
        return rc;
    }
    if (flags & O_ACCMODE) == O_RDONLY {
        rc = PTR_ERR(*lower_file);
        return rc;
    }
    mutex_lock(&mut ecryptfs_kthread_ctl.mux);
    if (ecryptfs_kthread_ctl.flags & ECRYPTFS_KTHREAD_ZOMBIE) != 0 {
        rc = -EIO;
        mutex_unlock(&mut ecryptfs_kthread_ctl.mux);
        printk!(KERN_ERR, "%s: We are in the middle of shutting down; aborting privileged request to open lower file\n", __func__);
        return rc;
    }
    list_add_tail(&mut req.kthread_ctl_list, &mut ecryptfs_kthread_ctl.req_list);
    mutex_unlock(&mut ecryptfs_kthread_ctl.mux);
    wake_up(&mut ecryptfs_kthread_ctl.wait);
    wait_for_completion(&mut req.done);
    if IS_ERR(*lower_file) {
        rc = PTR_ERR(*lower_file);
    }
    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
