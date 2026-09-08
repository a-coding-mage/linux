// SPDX-License-Identifier: GPL-2.0
//
// device_cgroup.rs - device cgroup subsystem
//
// Copyright 2007 IBM Corp

// #include <linux/bpf-cgroup.h>
// #include <linux/device_cgroup.h>
// #include <linux/cgroup.h>
// #include <linux/ctype.h>
// #include <linux/list.h>
// #include <linux/uaccess.h>
// #include <linux/seq_file.h>
// #include <linux/slab.h>
// #include <linux/rcupdate.h>
// #include <linux/mutex.h>

// #ifdef CONFIG_CGROUP_DEVICE

static DEVCGROUP_MUTEX: core::sync::atomic::AtomicPtr<()> = core::sync::atomic::AtomicPtr::new(core::ptr::null_mut());

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevcgBehavior {
    DevcgDefaultNone = 0,
    DevcgDefaultAllow = 1,
    DevcgDefaultDeny = 2,
}

//
// exception list locking rules:
// hold devcgroup_mutex for update/read.
// hold rcu_read_lock() for read.
//

#[repr(C)]
pub struct DevExceptionItem {
    pub major: u32,
    pub minor: u32,
    pub dev_type: i16,
    pub access: i16,
    pub list: ListHead,
    pub rcu: RcuHead,
}

pub struct ListHead {
    pub next: *mut ListHead,
    pub prev: *mut ListHead,
}

pub struct RcuHead {
    pub next: *mut RcuHead,
    pub func: Option<unsafe extern "C" fn(*mut RcuHead)>,
}

#[repr(C)]
pub struct DevCgroup {
    pub css: CgroupSubsysState,
    pub exceptions: ListHead,
    pub behavior: DevcgBehavior,
}

pub struct CgroupSubsysState {
    pub parent: *mut CgroupSubsysState,
}

unsafe fn css_to_devcgroup(s: *mut CgroupSubsysState) -> *mut DevCgroup {
    if s.is_null() {
        core::ptr::null_mut()
    } else {
        // container_of equivalent
        (s as *mut u8).sub(core::mem::offset_of!(DevCgroup, css)) as *mut DevCgroup
    }
}

unsafe fn task_devcgroup(task: *mut TaskStruct) -> *mut DevCgroup {
    css_to_devcgroup(task_css(task, DEVICES_CGRP_ID))
}

//
// called under devcgroup_mutex
//
unsafe fn dev_exceptions_copy(
    dest: *mut ListHead,
    orig: *mut ListHead,
) -> i32 {
    // lockdep_assert_held(&devcgroup_mutex);

    let mut ex: *mut DevExceptionItem;
    let mut new: *mut DevExceptionItem;

    let mut pos = (*orig).next;
    while pos != orig {
        ex = (pos as *mut u8).sub(core::mem::offset_of!(DevExceptionItem, list)) as *mut DevExceptionItem;

        new = kmemdup(ex, core::mem::size_of::<DevExceptionItem>(), GFP_KERNEL) as *mut DevExceptionItem;
        if new.is_null() {
            goto_free_and_exit(dest);
            return -12; // -ENOMEM
        }
        list_add_tail(&mut (*new).list, dest);
        pos = (*pos).next;
    }

    return 0;

    // free_and_exit label
}

unsafe fn goto_free_and_exit(dest: *mut ListHead) {
    let mut pos = (*dest).next;
    let mut tmp: *mut ListHead;

    while pos != dest {
        tmp = (*pos).next;
        let ex = (pos as *mut u8).sub(core::mem::offset_of!(DevExceptionItem, list)) as *mut DevExceptionItem;
        list_del(&mut (*ex).list);
        kfree(ex as *mut core::ffi::c_void);
        pos = tmp;
    }
}

unsafe fn dev_exceptions_move(dest: *mut ListHead, orig: *mut ListHead) {
    let mut ex: *mut DevExceptionItem;
    let mut pos = (*orig).next;
    let mut tmp: *mut ListHead;

    // lockdep_assert_held(&devcgroup_mutex);

    while pos != orig {
        tmp = (*pos).next;
        ex = (pos as *mut u8).sub(core::mem::offset_of!(DevExceptionItem, list)) as *mut DevExceptionItem;
        list_move_tail(&mut (*ex).list, dest);
        pos = tmp;
    }
}

//
// called under devcgroup_mutex
//
unsafe fn dev_exception_add(
    dev_cgroup: *mut DevCgroup,
    ex: *mut DevExceptionItem,
) -> i32 {
    let mut excopy: *mut DevExceptionItem;
    let mut walk: *mut DevExceptionItem;

    // lockdep_assert_held(&devcgroup_mutex);

    excopy = kmemdup(ex, core::mem::size_of::<DevExceptionItem>(), GFP_KERNEL) as *mut DevExceptionItem;
    if excopy.is_null() {
        return -12; // -ENOMEM
    }

    let mut pos = (*(*dev_cgroup).exceptions).next;
    while pos != &mut (*dev_cgroup).exceptions {
        walk = (pos as *mut u8).sub(core::mem::offset_of!(DevExceptionItem, list)) as *mut DevExceptionItem;

        if (*walk).dev_type != (*ex).dev_type {
            pos = (*pos).next;
            continue;
        }
        if (*walk).major != (*ex).major {
            pos = (*pos).next;
            continue;
        }
        if (*walk).minor != (*ex).minor {
            pos = (*pos).next;
            continue;
        }

        (*walk).access |= (*ex).access;
        kfree(excopy as *mut core::ffi::c_void);
        excopy = core::ptr::null_mut();
        break;
    }

    if !excopy.is_null() {
        list_add_tail_rcu(&mut (*excopy).list, &mut (*dev_cgroup).exceptions);
    }
    return 0;
}

//
// called under devcgroup_mutex
//
unsafe fn dev_exception_rm(dev_cgroup: *mut DevCgroup, ex: *mut DevExceptionItem) {
    let mut walk: *mut DevExceptionItem;
    let mut pos = (*(*dev_cgroup).exceptions).next;
    let mut tmp: *mut ListHead;

    // lockdep_assert_held(&devcgroup_mutex);

    while pos != &mut (*dev_cgroup).exceptions {
        tmp = (*pos).next;
        walk = (pos as *mut u8).sub(core::mem::offset_of!(DevExceptionItem, list)) as *mut DevExceptionItem;

        if (*walk).dev_type != (*ex).dev_type {
            pos = tmp;
            continue;
        }
        if (*walk).major != (*ex).major {
            pos = tmp;
            continue;
        }
        if (*walk).minor != (*ex).minor {
            pos = tmp;
            continue;
        }

        (*walk).access &= !(*ex).access;
        if (*walk).access == 0 {
            list_del_rcu(&mut (*walk).list);
            kfree_rcu(walk as *mut core::ffi::c_void, rcu);
        }

        pos = tmp;
    }
}

unsafe fn __dev_exception_clean(dev_cgroup: *mut DevCgroup) {
    let mut ex: *mut DevExceptionItem;
    let mut pos = (*(*dev_cgroup).exceptions).next;
    let mut tmp: *mut ListHead;

    while pos != &mut (*dev_cgroup).exceptions {
        tmp = (*pos).next;
        ex = (pos as *mut u8).sub(core::mem::offset_of!(DevExceptionItem, list)) as *mut DevExceptionItem;
        list_del_rcu(&mut (*ex).list);
        kfree_rcu(ex as *mut core::ffi::c_void, rcu);
        pos = tmp;
    }
}

///
/// dev_exception_clean - frees all entries of the exception list
/// @dev_cgroup: dev_cgroup with the exception list to be cleaned
///
/// called under devcgroup_mutex
///
unsafe fn dev_exception_clean(dev_cgroup: *mut DevCgroup) {
    // lockdep_assert_held(&devcgroup_mutex);

    __dev_exception_clean(dev_cgroup);
}

unsafe fn is_devcg_online(devcg: *const DevCgroup) -> bool {
    (*devcg).behavior != DevcgBehavior::DevcgDefaultNone
}

///
/// devcgroup_online - initializes devcgroup's behavior and exceptions based on
/// 		      parent's
/// @css: css getting online
/// returns 0 in case of success, error code otherwise
///
unsafe fn devcgroup_online(css: *mut CgroupSubsysState) -> i32 {
    let dev_cgroup = css_to_devcgroup(css);
    let parent_dev_cgroup = css_to_devcgroup((*css).parent);
    let mut ret: i32 = 0;

    mutex_lock(&DEVCGROUP_MUTEX);

    if parent_dev_cgroup.is_null() {
        (*dev_cgroup).behavior = DevcgBehavior::DevcgDefaultAllow;
    } else {
        ret = dev_exceptions_copy(
            &mut (*dev_cgroup).exceptions,
            &mut (*parent_dev_cgroup).exceptions,
        );
        if ret == 0 {
            (*dev_cgroup).behavior = (*parent_dev_cgroup).behavior;
        }
    }

    mutex_unlock(&DEVCGROUP_MUTEX);

    return ret;
}

unsafe fn devcgroup_offline(css: *mut CgroupSubsysState) {
    let dev_cgroup = css_to_devcgroup(css);

    mutex_lock(&DEVCGROUP_MUTEX);
    (*dev_cgroup).behavior = DevcgBehavior::DevcgDefaultNone;
    mutex_unlock(&DEVCGROUP_MUTEX);
}

//
// called from kernel/cgroup/cgroup.c with cgroup_lock() held.
//
unsafe fn devcgroup_css_alloc(parent_css: *mut CgroupSubsysState) -> *mut CgroupSubsysState {
    let dev_cgroup: *mut DevCgroup = kzalloc(core::mem::size_of::<DevCgroup>(), GFP_KERNEL) as *mut DevCgroup;
    if dev_cgroup.is_null() {
        return ERR_PTR(-12); // -ENOMEM
    }
    INIT_LIST_HEAD(&mut (*dev_cgroup).exceptions);
    (*dev_cgroup).behavior = DevcgBehavior::DevcgDefaultNone;

    return &mut (*dev_cgroup).css;
}

unsafe fn devcgroup_css_free(css: *mut CgroupSubsysState) {
    let dev_cgroup = css_to_devcgroup(css);

    __dev_exception_clean(dev_cgroup);
    kfree(dev_cgroup as *mut core::ffi::c_void);
}

const DEVCG_ALLOW: i32 = 1;
const DEVCG_DENY: i32 = 2;
const DEVCG_LIST: i32 = 3;

unsafe fn seq_putaccess(m: *mut SeqFile, access: i16) {
    if (access & DEVCG_ACC_READ) != 0 {
        seq_putc(m, b'r' as i32);
    }
    if (access & DEVCG_ACC_WRITE) != 0 {
        seq_putc(m, b'w' as i32);
    }
    if (access & DEVCG_ACC_MKNOD) != 0 {
        seq_putc(m, b'm' as i32);
    }
}

unsafe fn seq_puttype(m: *mut SeqFile, dev_type: i16) {
    if dev_type == DEVCG_DEV_ALL {
        seq_putc(m, b'a' as i32);
    } else if dev_type == DEVCG_DEV_CHAR {
        seq_putc(m, b'c' as i32);
    } else if dev_type == DEVCG_DEV_BLOCK {
        seq_putc(m, b'b' as i32);
    } else {
        seq_putc(m, b'X' as i32);
    }
}

unsafe fn seq_putversion(m: *mut SeqFile, version: u32) {
    if version == !0 {
        seq_putc(m, b'*' as i32);
    } else {
        seq_printf(m, "%u\0".as_ptr() as *const i8, version);
    }
}

unsafe fn devcgroup_seq_show(m: *mut SeqFile, v: *mut core::ffi::c_void) -> i32 {
    let devcgroup = css_to_devcgroup(seq_css(m));
    let mut ex: *mut DevExceptionItem;

    rcu_read_lock();
    //
    // To preserve the compatibility:
    // - Only show the "all devices" when the default policy is to allow
    // - List the exceptions in case the default policy is to deny
    // This way, the file remains as a "whitelist of devices"
    //
    if (*devcgroup).behavior == DevcgBehavior::DevcgDefaultAllow {
        seq_puts(m, "a *:* rwm\n\0".as_ptr() as *const i8);
    } else {
        let mut pos = (*(*devcgroup).exceptions).next;
        while pos != &mut (*devcgroup).exceptions {
            ex = (pos as *mut u8).sub(core::mem::offset_of!(DevExceptionItem, list)) as *mut DevExceptionItem;
            seq_puttype(m, (*ex).dev_type);
            seq_putc(m, b' ' as i32);
            seq_putversion(m, (*ex).major);
            seq_putc(m, b':' as i32);
            seq_putversion(m, (*ex).minor);
            seq_putc(m, b' ' as i32);
            seq_putaccess(m, (*ex).access);
            seq_putc(m, b'\n' as i32);
            pos = (*pos).next;
        }
    }
    rcu_read_unlock();

    return 0;
}

///
/// match_exception	- iterates the exception list trying to find a complete match
/// @exceptions: list of exceptions
/// @type: device type (DEVCG_DEV_BLOCK or DEVCG_DEV_CHAR)
/// @major: device file major number, ~0 to match all
/// @minor: device file minor number, ~0 to match all
/// @access: permission mask (DEVCG_ACC_READ, DEVCG_ACC_WRITE, DEVCG_ACC_MKNOD)
///
/// It is considered a complete match if an exception is found that will
/// contain the entire range of provided parameters.
///
/// Return: true in case it matches an exception completely
///
unsafe fn match_exception(
    exceptions: *mut ListHead,
    dev_type: i16,
    major: u32,
    minor: u32,
    access: i16,
) -> bool {
    let mut ex: *mut DevExceptionItem;

    let mut pos = (*exceptions).next;
    while pos != exceptions {
        ex = (pos as *mut u8).sub(core::mem::offset_of!(DevExceptionItem, list)) as *mut DevExceptionItem;

        if (dev_type & DEVCG_DEV_BLOCK) != 0 && ((*ex).dev_type & DEVCG_DEV_BLOCK) == 0 {
            pos = (*pos).next;
            continue;
        }
        if (dev_type & DEVCG_DEV_CHAR) != 0 && ((*ex).dev_type & DEVCG_DEV_CHAR) == 0 {
            pos = (*pos).next;
            continue;
        }
        if (*ex).major != !0 && (*ex).major != major {
            pos = (*pos).next;
            continue;
        }
        if (*ex).minor != !0 && (*ex).minor != minor {
            pos = (*pos).next;
            continue;
        }
        // provided access cannot have more than the exception rule
        if (access & (!(*ex).access)) != 0 {
            pos = (*pos).next;
            continue;
        }
        return true;
    }
    return false;
}

///
/// match_exception_partial - iterates the exception list trying to find a partial match
/// @exceptions: list of exceptions
/// @type: device type (DEVCG_DEV_BLOCK or DEVCG_DEV_CHAR)
/// @major: device file major number, ~0 to match all
/// @minor: device file minor number, ~0 to match all
/// @access: permission mask (DEVCG_ACC_READ, DEVCG_ACC_WRITE, DEVCG_ACC_MKNOD)
///
/// It is considered a partial match if an exception's range is found to
/// contain *any* of the devices specified by provided parameters. This is
/// used to make sure no extra access is being granted that is forbidden by
/// any of the exception list.
///
/// Return: true in case the provided range mat matches an exception completely
///
unsafe fn match_exception_partial(
    exceptions: *mut ListHead,
    dev_type: i16,
    major: u32,
    minor: u32,
    access: i16,
) -> bool {
    let mut ex: *mut DevExceptionItem;

    let mut pos = (*exceptions).next;
    while pos != exceptions {
        ex = (pos as *mut u8).sub(core::mem::offset_of!(DevExceptionItem, list)) as *mut DevExceptionItem;

        if (dev_type & DEVCG_DEV_BLOCK) != 0 && ((*ex).dev_type & DEVCG_DEV_BLOCK) == 0 {
            pos = (*pos).next;
            continue;
        }
        if (dev_type & DEVCG_DEV_CHAR) != 0 && ((*ex).dev_type & DEVCG_DEV_CHAR) == 0 {
            pos = (*pos).next;
            continue;
        }
        //
        // We must be sure that both the exception and the provided
        // range aren't masking all devices
        //
        if (*ex).major != !0 && major != !0 && (*ex).major != major {
            pos = (*pos).next;
            continue;
        }
        if (*ex).minor != !0 && minor != !0 && (*ex).minor != minor {
            pos = (*pos).next;
            continue;
        }
        //
        // In order to make sure the provided range isn't matching
        // an exception, all its access bits shouldn't match the
        // exception's access bits
        //
        if (access & (*ex).access) == 0 {
            pos = (*pos).next;
            continue;
        }
        return true;
    }
    return false;
}

///
/// verify_new_ex - verifies if a new exception is allowed by parent cgroup's permissions
/// @dev_cgroup: dev cgroup to be tested against
/// @refex: new exception
/// @behavior: behavior of the exception's dev_cgroup
///
/// This is used to make sure a child cgroup won't have more privileges
/// than its parent
///
unsafe fn verify_new_ex(
    dev_cgroup: *mut DevCgroup,
    refex: *mut DevExceptionItem,
    behavior: DevcgBehavior,
) -> bool {
    let mut matching = false;

    // RCU_LOCKDEP_WARN(!rcu_read_lock_held() &&
    //                  !lockdep_is_held(&devcgroup_mutex),
    //                  "device_cgroup:verify_new_ex called without proper synchronization");

    if (*dev_cgroup).behavior == DevcgBehavior::DevcgDefaultAllow {
        if behavior == DevcgBehavior::DevcgDefaultAllow {
            //
            // new exception in the child doesn't matter, only
            // adding extra restrictions
            //
            return true;
        } else {
            //
            // new exception in the child will add more devices
            // that can be accessed, so it can't match any of
            // parent's exceptions, even slightly
            //
            matching = match_exception_partial(
                &mut (*dev_cgroup).exceptions,
                (*refex).dev_type,
                (*refex).major,
                (*refex).minor,
                (*refex).access,
            );

            if matching {
                return false;
            }
            return true;
        }
    } else {
        //
        // Only behavior == DEVCG_DEFAULT_DENY allowed here, therefore
        // the new exception will add access to more devices and must
        // be contained completely in an parent's exception to be
        // allowed
        //
        matching = match_exception(
            &mut (*dev_cgroup).exceptions,
            (*refex).dev_type,
            (*refex).major,
            (*refex).minor,
            (*refex).access,
        );

        if matching {
            // parent has an exception that matches the proposed
            return true;
        } else {
            return false;
        }
    }
}

//
// parent_has_perm:
// when adding a new allow rule to a device exception list, the rule
// must be allowed in the parent device
//
unsafe fn parent_has_perm(childcg: *mut DevCgroup, ex: *mut DevExceptionItem) -> i32 {
    let parent = css_to_devcgroup((*(*childcg).css).parent);

    if parent.is_null() {
        return 1;
    }
    return if verify_new_ex(parent, ex, (*childcg).behavior) { 1 } else { 0 };
}

///
/// parent_allows_removal - verify if it's ok to remove an exception
/// @childcg: child cgroup from where the exception will be removed
/// @ex: exception being removed
///
/// When removing an exception in cgroups with default ALLOW policy, it must
/// be checked if removing it will give the child cgroup more access than the
/// parent.
///
/// Return: true if it's ok to remove exception, false otherwise
///
unsafe fn parent_allows_removal(childcg: *mut DevCgroup, ex: *mut DevExceptionItem) -> bool {
    let parent = css_to_devcgroup((*(*childcg).css).parent);

    if parent.is_null() {
        return true;
    }

    // It's always allowed to remove access to devices
    if (*childcg).behavior == DevcgBehavior::DevcgDefaultDeny {
        return true;
    }

    //
    // Make sure you're not removing part or a whole exception existing in
    // the parent cgroup
    //
    return !match_exception_partial(
        &mut (*parent).exceptions,
        (*ex).dev_type,
        (*ex).major,
        (*ex).minor,
        (*ex).access,
    );
}

///
/// may_allow_all - checks if it's possible to change the behavior to
///		   allow based on parent's rules.
/// @parent: device cgroup's parent
/// returns: != 0 in case it's allowed, 0 otherwise
///
unsafe fn may_allow_all(parent: *mut DevCgroup) -> i32 {
    if parent.is_null() {
        return 1;
    }
    return if (*parent).behavior == DevcgBehavior::DevcgDefaultAllow { 1 } else { 0 };
}

///
/// revalidate_active_exceptions - walks through the active exception list and
/// 				  revalidates the exceptions based on parent's
/// 				  behavior and exceptions. The exceptions that
/// 				  are no longer valid will be removed.
/// 				  Called with devcgroup_mutex held.
/// @devcg: cgroup which exceptions will be checked
///
/// This is one of the three key functions for hierarchy implementation.
/// This function is responsible for re-evaluating all the cgroup's active
/// exceptions due to a parent's exception change.
/// Refer to Documentation/admin-guide/cgroup-v1/devices.rst for more details.
///
unsafe fn revalidate_active_exceptions(devcg: *mut DevCgroup) {
    let mut ex: *mut DevExceptionItem;
    let mut pos = (*(*devcg).exceptions).next;
    let mut tmp: *mut ListHead;

    while pos != &mut (*devcg).exceptions {
        tmp = (*pos).next;
        ex = (pos as *mut u8).sub(core::mem::offset_of!(DevExceptionItem, list)) as *mut DevExceptionItem;
        if parent_has_perm(devcg, ex) == 0 {
            dev_exception_rm(devcg, ex);
        }
        pos = tmp;
    }
}

///
/// propagate_exception - propagates a new exception to the children
/// @devcg_root: device cgroup that added a new exception
/// @ex: new exception to be propagated
///
/// returns: 0 in case of success, != 0 in case of error
///
unsafe fn propagate_exception(devcg_root: *mut DevCgroup, ex: *mut DevExceptionItem) -> i32 {
    let mut pos: *mut CgroupSubsysState;
    let mut rc: i32 = 0;

    rcu_read_lock();

    css_for_each_descendant_pre(&mut pos, &mut (*devcg_root).css);
    while !pos.is_null() {
        let devcg = css_to_devcgroup(pos);

        //
        // Because devcgroup_mutex is held, no devcg will become
        // online or offline during the tree walk (see on/offline
        // methods), and online ones are safe to access outside RCU
        // read lock without bumping refcnt.
        //
        if pos == &mut (*devcg_root).css || !is_devcg_online(devcg) {
            css_for_each_descendant_pre(&mut pos, pos);
            continue;
        }

        rcu_read_unlock();

        //
        // in case both root's behavior and devcg is allow, a new
        // restriction means adding to the exception list
        //
        if (*devcg_root).behavior == DevcgBehavior::DevcgDefaultAllow
            && (*devcg).behavior == DevcgBehavior::DevcgDefaultAllow
        {
            rc = dev_exception_add(devcg, ex);
            if rc != 0 {
                return rc;
            }
        } else {
            //
            // in the other possible cases:
            // root's behavior: allow, devcg's: deny
            // root's behavior: deny, devcg's: deny
            // the exception will be removed
            //
            dev_exception_rm(devcg, ex);
        }
        revalidate_active_exceptions(devcg);

        rcu_read_lock();

        css_for_each_descendant_pre(&mut pos, pos);
    }

    rcu_read_unlock();
    return rc;
}

//
// Modify the exception list using allow/deny rules.
// CAP_SYS_ADMIN is needed for this.  It's at least separate from CAP_MKNOD
// so we can give a container CAP_MKNOD to let it create devices but not
// modify the exception list.
// It seems likely we'll want to add a CAP_CONTAINER capability to allow
// us to also grant CAP_SYS_ADMIN to containers without giving away the
// device exception list controls, but for now we'll stick with CAP_SYS_ADMIN
//
// Taking rules away is always allowed (given CAP_SYS_ADMIN).  Granting
// new access is only allowed if you're in the top-level cgroup, or your
// parent cgroup has the access you're asking for.
//
unsafe fn devcgroup_update_access(
    devcgroup: *mut DevCgroup,
    filetype: i32,
    buffer: *mut i8,
) -> i32 {
    let mut b = buffer as *const u8;
    let mut temp: [u8; 12] = [0; 12]; // 11 + 1 characters needed for a u32
    let mut count = 0;
    let mut rc: i32 = 0;
    let mut ex: DevExceptionItem = core::mem::zeroed();
    let parent = css_to_devcgroup((*(*devcgroup).css).parent);
    let mut tmp_devcgrp: DevCgroup = core::mem::zeroed();

    if !capable(CAP_SYS_ADMIN) {
        return -1; // -EPERM
    }

    core::ptr::write_bytes(&mut ex, 0, 1);
    core::ptr::write_bytes(&mut tmp_devcgrp, 0, 1);

    match *b as i32 {
        97 => { // 'a'
            match filetype {
                DEVCG_ALLOW => {
                    if css_has_online_children(&mut (*devcgroup).css) {
                        return -22; // -EINVAL
                    }

                    if may_allow_all(parent) == 0 {
                        return -1; // -EPERM
                    }
                    if parent.is_null() {
                        (*devcgroup).behavior = DevcgBehavior::DevcgDefaultAllow;
                        dev_exception_clean(devcgroup);
                        b = b.add(1);
                    } else {
                        INIT_LIST_HEAD(&mut tmp_devcgrp.exceptions);
                        rc = dev_exceptions_copy(&mut tmp_devcgrp.exceptions, &mut (*devcgroup).exceptions);
                        if rc != 0 {
                            return rc;
                        }
                        dev_exception_clean(devcgroup);
                        rc = dev_exceptions_copy(&mut (*devcgroup).exceptions, &mut (*parent).exceptions);
                        if rc != 0 {
                            dev_exceptions_move(&mut (*devcgroup).exceptions, &mut tmp_devcgrp.exceptions);
                            return rc;
                        }
                        (*devcgroup).behavior = DevcgBehavior::DevcgDefaultAllow;
                        dev_exception_clean(&mut tmp_devcgrp);
                        b = b.add(1);
                    }
                }
                DEVCG_DENY => {
                    if css_has_online_children(&mut (*devcgroup).css) {
                        return -22; // -EINVAL
                    }

                    dev_exception_clean(devcgroup);
                    (*devcgroup).behavior = DevcgBehavior::DevcgDefaultDeny;
                    b = b.add(1);
                }
                _ => {
                    return -22; // -EINVAL
                }
            }
            return 0;
        }
        98 => { // 'b'
            ex.dev_type = DEVCG_DEV_BLOCK;
        }
        99 => { // 'c'
            ex.dev_type = DEVCG_DEV_CHAR;
        }
        _ => {
            return -22; // -EINVAL
        }
    }
    b = b.add(1);
    if !isspace(*b) {
        return -22; // -EINVAL
    }
    b = b.add(1);
    if *b == b'*' as u8 {
        ex.major = !0;
        b = b.add(1);
    } else if isdigit(*b) {
        core::ptr::write_bytes(temp.as_mut_ptr(), 0, temp.len());
        for count in 0..(temp.len() - 1) {
            temp[count] = *b;
            b = b.add(1);
            if !isdigit(*b) {
                break;
            }
        }
        rc = kstrtou32(temp.as_ptr(), 10, &mut ex.major);
        if rc != 0 {
            return -22; // -EINVAL
        }
    } else {
        return -22; // -EINVAL
    }
    if *b != b':' as u8 {
        return -22; // -EINVAL
    }
    b = b.add(1);

    // read minor
    if *b == b'*' as u8 {
        ex.minor = !0;
        b = b.add(1);
    } else if isdigit(*b) {
        core::ptr::write_bytes(temp.as_mut_ptr(), 0, temp.len());
        for count in 0..(temp.len() - 1) {
            temp[count] = *b;
            b = b.add(1);
            if !isdigit(*b) {
                break;
            }
        }
        rc = kstrtou32(temp.as_ptr(), 10, &mut ex.minor);
        if rc != 0 {
            return -22; // -EINVAL
        }
    } else {
        return -22; // -EINVAL
    }
    if !isspace(*b) {
        return -22; // -EINVAL
    }
    b = b.add(1);
    for count in 0..3 {
        match *b as i32 {
            114 => { // 'r'
                ex.access |= DEVCG_ACC_READ;
            }
            119 => { // 'w'
                ex.access |= DEVCG_ACC_WRITE;
            }
            109 => { // 'm'
                ex.access |= DEVCG_ACC_MKNOD;
            }
            10 | 0 => { // '\n' or '\0'
                count = 3;
            }
            _ => {
                return -22; // -EINVAL
            }
        }
        if count < 3 {
            b = b.add(1);
        }
    }

    match filetype {
        DEVCG_ALLOW => {
            //
            // If the default policy is to allow by default, try to remove
            // an matching exception instead. And be silent about it: we
            // don't want to break compatibility
            //
            if (*devcgroup).behavior == DevcgBehavior::DevcgDefaultAllow {
                // Check if the parent allows removing it first
                if !parent_allows_removal(devcgroup, &mut ex) {
                    return -1; // -EPERM
                }
                dev_exception_rm(devcgroup, &mut ex);
                return 0;
            }

            if parent_has_perm(devcgroup, &mut ex) == 0 {
                return -1; // -EPERM
            }
            rc = dev_exception_add(devcgroup, &mut ex);
        }
        DEVCG_DENY => {
            //
            // If the default policy is to deny by default, try to remove
            // an matching exception instead. And be silent about it: we
            // don't want to break compatibility
            //
            if (*devcgroup).behavior == DevcgBehavior::DevcgDefaultDeny {
                dev_exception_rm(devcgroup, &mut ex);
            } else {
                rc = dev_exception_add(devcgroup, &mut ex);
            }

            if rc != 0 {
                return rc;
            }
            // we only propagate new restrictions
            rc = propagate_exception(devcgroup, &mut ex);
        }
        _ => {
            rc = -22; // -EINVAL
        }
    }
    return rc;
}

unsafe fn devcgroup_access_write(
    of: *mut KernfsOpenFile,
    buf: *mut i8,
    nbytes: usize,
    off: i64,
) -> isize {
    let mut retval: i32;

    mutex_lock(&DEVCGROUP_MUTEX);
    retval = devcgroup_update_access(
        css_to_devcgroup(of_css(of)),
        of_cft_private(of),
        strstrip(buf),
    );
    mutex_unlock(&DEVCGROUP_MUTEX);
    return if retval == 0 { nbytes as isize } else { retval as isize };
}

#[repr(C)]
pub struct Cftype {
    pub name: *const i8,
    pub write: Option<unsafe extern "C" fn(*mut KernfsOpenFile, *mut i8, usize, i64) -> isize>,
    pub seq_show: Option<unsafe extern "C" fn(*mut SeqFile, *mut core::ffi::c_void) -> i32>,
    pub private: i32,
}

pub struct SeqFile;
pub struct KernfsOpenFile;
pub struct TaskStruct;

static DEV_CGROUP_FILES: [Cftype; 4] = [
    Cftype {
        name: "allow\0".as_ptr() as *const i8,
        write: Some(devcgroup_access_write),
        seq_show: None,
        private: DEVCG_ALLOW,
    },
    Cftype {
        name: "deny\0".as_ptr() as *const i8,
        write: Some(devcgroup_access_write),
        seq_show: None,
        private: DEVCG_DENY,
    },
    Cftype {
        name: "list\0".as_ptr() as *const i8,
        write: None,
        seq_show: Some(devcgroup_seq_show),
        private: DEVCG_LIST,
    },
    Cftype {
        name: core::ptr::null(),
        write: None,
        seq_show: None,
        private: 0,
    },
];

pub struct CgroupSubsys {
    pub css_alloc: Option<unsafe extern "C" fn(*mut CgroupSubsysState) -> *mut CgroupSubsysState>,
    pub css_free: Option<unsafe extern "C" fn(*mut CgroupSubsysState)>,
    pub css_online: Option<unsafe extern "C" fn(*mut CgroupSubsysState) -> i32>,
    pub css_offline: Option<unsafe extern "C" fn(*mut CgroupSubsysState)>,
    pub legacy_cftypes: *const Cftype,
}

pub static DEVICES_CGRP_SUBSYS: CgroupSubsys = CgroupSubsys {
    css_alloc: Some(devcgroup_css_alloc),
    css_free: Some(devcgroup_css_free),
    css_online: Some(devcgroup_online),
    css_offline: Some(devcgroup_offline),
    legacy_cftypes: &DEV_CGROUP_FILES[0],
};

///
/// devcgroup_legacy_check_permission - checks if an inode operation is permitted
/// @type: device type
/// @major: device major number
/// @minor: device minor number
/// @access: combination of DEVCG_ACC_WRITE, DEVCG_ACC_READ and DEVCG_ACC_MKNOD
///
/// returns 0 on success, -EPERM case the operation is not permitted
///
unsafe fn devcgroup_legacy_check_permission(
    dev_type: i16,
    major: u32,
    minor: u32,
    access: i16,
) -> i32 {
    let dev_cgroup: *mut DevCgroup;
    let rc: bool;

    rcu_read_lock();
    dev_cgroup = task_devcgroup(current as *mut TaskStruct);
    if (*dev_cgroup).behavior == DevcgBehavior::DevcgDefaultAllow {
        // Can't match any of the exceptions, even partially
        rc = !match_exception_partial(&mut (*dev_cgroup).exceptions, dev_type, major, minor, access);
    } else {
        // Need to match completely one exception to be allowed
        rc = match_exception(&mut (*dev_cgroup).exceptions, dev_type, major, minor, access);
    }
    rcu_read_unlock();

    if !rc {
        return -1; // -EPERM
    }

    return 0;
}

// #endif /* CONFIG_CGROUP_DEVICE */

// #if defined(CONFIG_CGROUP_DEVICE) || defined(CONFIG_CGROUP_BPF)

pub extern "C" fn devcgroup_check_permission(
    dev_type: i16,
    major: u32,
    minor: u32,
    access: i16,
) -> i32 {
    let rc = unsafe {
        BPF_CGROUP_RUN_PROG_DEVICE_CGROUP(dev_type, major, minor, access)
    };

    if rc != 0 {
        return rc;
    }

    // #ifdef CONFIG_CGROUP_DEVICE
    return unsafe { devcgroup_legacy_check_permission(dev_type, major, minor, access) };

    // #else /* CONFIG_CGROUP_DEVICE */
    // return 0;

    // #endif /* CONFIG_CGROUP_DEVICE */
}
// EXPORT_SYMBOL(devcgroup_check_permission);
// #endif /* defined(CONFIG_CGROUP_DEVICE) || defined(CONFIG_CGROUP_BPF) */

// External function stubs and constants from kernel headers

unsafe fn INIT_LIST_HEAD(head: *mut ListHead) {
    (*head).next = head;
    (*head).prev = head;
}

unsafe fn list_add_tail(new: *mut ListHead, head: *mut ListHead) {
    (*new).next = head;
    (*new).prev = (*head).prev;
    (*(*head).prev).next = new;
    (*head).prev = new;
}

unsafe fn list_add_tail_rcu(new: *mut ListHead, head: *mut ListHead) {
    list_add_tail(new, head);
}

unsafe fn list_del(entry: *mut ListHead) {
    (*(*entry).prev).next = (*entry).next;
    (*(*entry).next).prev = (*entry).prev;
}

unsafe fn list_del_rcu(entry: *mut ListHead) {
    list_del(entry);
}

unsafe fn list_move_tail(list: *mut ListHead, head: *mut ListHead) {
    list_del(list);
    list_add_tail(list, head);
}

const GFP_KERNEL: i32 = 0xd0;
const DEVCG_ACC_READ: i16 = 1;
const DEVCG_ACC_WRITE: i16 = 2;
const DEVCG_ACC_MKNOD: i16 = 4;
const DEVCG_DEV_ALL: i16 = 1;
const DEVCG_DEV_CHAR: i16 = 2;
const DEVCG_DEV_BLOCK: i16 = 4;
const CAP_SYS_ADMIN: i32 = 21;
const DEVICES_CGRP_ID: i32 = 0;

extern "C" {
    fn kmemdup(src: *const core::ffi::c_void, len: usize, flags: i32) -> *mut core::ffi::c_void;
    fn kzalloc(size: usize, flags: i32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn kfree_rcu(ptr: *mut core::ffi::c_void, member_name: &str);
    fn mutex_lock(lock: &core::sync::atomic::AtomicPtr<()>);
    fn mutex_unlock(lock: &core::sync::atomic::AtomicPtr<()>);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn task_css(task: *mut TaskStruct, id: i32) -> *mut CgroupSubsysState;
    fn seq_putc(m: *mut SeqFile, c: i32);
    fn seq_puts(m: *mut SeqFile, s: *const i8);
    fn seq_printf(m: *mut SeqFile, fmt: *const i8, ...);
    fn seq_css(m: *mut SeqFile) -> *mut CgroupSubsysState;
    fn css_for_each_descendant_pre(pos: *mut *mut CgroupSubsysState, root: *mut CgroupSubsysState);
    fn css_has_online_children(css: *mut CgroupSubsysState) -> bool;
    fn isspace(c: u8) -> bool;
    fn isdigit(c: u8) -> bool;
    fn capable(cap: i32) -> bool;
    fn kstrtou32(s: *const u8, base: i32, res: *mut u32) -> i32;
    fn strstrip(str: *mut i8) -> *mut i8;
    fn of_css(of: *mut KernfsOpenFile) -> *mut CgroupSubsysState;
    fn of_cft_private(of: *mut KernfsOpenFile) -> i32;
    fn BPF_CGROUP_RUN_PROG_DEVICE_CGROUP(
        type_: i16,
        major: u32,
        minor: u32,
        access: i16,
    ) -> i32;
    static current: *mut TaskStruct;
}

fn ERR_PTR(err: i32) -> *mut CgroupSubsysState {
    err as *mut CgroupSubsysState
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
