// SPDX-License-Identifier: GPL-2.0
/*
 * linux/drivers/char/misc.c
 *
 * Generic misc open routine by Johan Myreen
 *
 * Based on code from Linus
 *
 * Teemu Rantanen's Microsoft Busmouse support and Derrick Cole's
 *   changes incorporated into 0.97pl4
 *   by Peter Cervasio (pete%q106fm.uucp@wupost.wustl.edu) (08SEP92)
 *   See busmouse.c for particulars.
 *
 * Made things a lot mode modular - easy to compile in just one or two
 * of the misc drivers, as they are now completely independent. Linus.
 *
 * Support for loadable modules. 8-Sep-95 Philip Blundell <pjb27@cam.ac.uk>
 *
 * Fixed a failing symbol register to free the device registration
 *\t\tAlan Cox <alan@lxorguk.ukuu.org.uk> 21-Jan-96
 *
 * Dynamic minors and /proc/mice by Alessandro Rubini. 26-Mar-96
 *
 * Renamed to misc and miscdevice to be more accurate. Alan Cox 26-Mar-96
 *
 * Handling of mouse minor numbers for kerneld:
 *  Idea by Jacques Gelinas <jack@solucorp.qc.ca>,
 *  adapted by Bjorn Ekwall <bj0rn@blox.se>
 *  corrected by Alan Cox <alan@lxorguk.ukuu.org.uk>
 *
 * Changes for kmod (from kerneld):
 *\tCyrus Durgin <cider@speakeasy.org>
 *
 * Added devfs support. Richard Gooch <rgooch@atnf.csiro.au>  10-Jan-1998
 */

// Kernel includes translated as external dependencies supplied by other files.

/* Head entry for the doubly linked miscdevice list */
static mut MISC_LIST: ListHead = LIST_HEAD_INIT;
static mut MISC_MTX: Mutex = DEFINE_MUTEX_INIT;

/* Assigned numbers. */
static mut MISC_MINORS_IDA: Ida = DEFINE_IDA_INIT;

unsafe fn misc_minor_alloc(minor: i32) -> i32 {
    let mut ret: i32 = 0;

    if minor == MISC_DYNAMIC_MINOR {
        /* allocate free id */
        ret = ida_alloc_range(
            &raw mut MISC_MINORS_IDA,
            MISC_DYNAMIC_MINOR + 1,
            MINORMASK,
            GFP_KERNEL,
        );
    } else {
        ret = ida_alloc_range(&raw mut MISC_MINORS_IDA, minor, minor, GFP_KERNEL);
    }
    ret
}

unsafe fn misc_minor_free(minor: i32) {
    ida_free(&raw mut MISC_MINORS_IDA, minor);
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn misc_seq_start(seq: *mut SeqFile, pos: *mut LoffT) -> *mut core::ffi::c_void {
    mutex_lock(&raw mut MISC_MTX);
    seq_list_start(&raw mut MISC_LIST, *pos)
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn misc_seq_next(
    seq: *mut SeqFile,
    v: *mut core::ffi::c_void,
    pos: *mut LoffT,
) -> *mut core::ffi::c_void {
    seq_list_next(v, &raw mut MISC_LIST, pos)
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn misc_seq_stop(seq: *mut SeqFile, v: *mut core::ffi::c_void) {
    mutex_unlock(&raw mut MISC_MTX);
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn misc_seq_show(seq: *mut SeqFile, v: *mut core::ffi::c_void) -> i32 {
    let p: *const MiscDevice = list_entry(v, MiscDevice, list);
    seq_printf(seq, "%3i %s\n", (*p).minor, if !(*p).name.is_null() { (*p).name } else { b"\0".as_ptr() as *const i8 });
    0
}

#[cfg(CONFIG_PROC_FS)]
static MISC_SEQ_OPS: SeqOperations = SeqOperations {
    start: Some(misc_seq_start),
    next: Some(misc_seq_next),
    stop: Some(misc_seq_stop),
    show: Some(misc_seq_show),
};

unsafe fn misc_open(inode: *mut Inode, file: *mut File) -> i32 {
    let minor: i32 = iminor(inode);
    let mut c: *mut MiscDevice = core::ptr::null_mut();
    let mut iter: *mut MiscDevice;
    let mut err: i32 = -ENODEV;
    let mut new_fops: *const FileOperations = core::ptr::null();

    mutex_lock(&raw mut MISC_MTX);

    list_for_each_entry!(iter, &raw mut MISC_LIST, list, {
        if (*iter).minor != minor { continue; }
        c = iter;
        new_fops = fops_get((*iter).fops);
        break;
    });

    /* Only request module for fixed minor code */
    if new_fops.is_null() && minor < MISC_DYNAMIC_MINOR {
        mutex_unlock(&raw mut MISC_MTX);
        request_module!("char-major-%d-%d", MISC_MAJOR, minor);
        mutex_lock(&raw mut MISC_MTX);

        list_for_each_entry!(iter, &raw mut MISC_LIST, list, {
            if (*iter).minor != minor { continue; }
            c = iter;
            new_fops = fops_get((*iter).fops);
            break;
        });
    }

    if new_fops.is_null() {
        mutex_unlock(&raw mut MISC_MTX);
        return err;
    }

    /* Place the miscdevice in the file's private_data. */
    (*file).private_data = c as *mut core::ffi::c_void;
    err = 0;
    replace_fops(file, new_fops);
    if let Some(open) = (*(*file).f_op).open {
        err = open(inode, file);
    }
    mutex_unlock(&raw mut MISC_MTX);
    err
}

unsafe fn misc_devnode(dev: *const Device, mode: *mut UmodeT) -> *mut i8 {
    let c: *const MiscDevice = dev_get_drvdata(dev);
    if !mode.is_null() && (*c).mode != 0 { *mode = (*c).mode; }
    if !(*c).nodename.is_null() { return kstrdup((*c).nodename, GFP_KERNEL); }
    core::ptr::null_mut()
}

static MISC_CLASS: Class = Class {
    name: b"misc\0".as_ptr() as *const i8,
    devnode: Some(misc_devnode),
};

static MISC_FOPS: FileOperations = FileOperations {
    owner: THIS_MODULE,
    open: Some(misc_open),
    llseek: Some(noop_llseek),
};

pub unsafe fn misc_register(misc: *mut MiscDevice) -> i32 {
    let mut dev: DevT;
    let mut err: i32 = 0;
    let is_dynamic: bool = (*misc).minor == MISC_DYNAMIC_MINOR;

    if (*misc).minor > MISC_DYNAMIC_MINOR {
        pr_err!("Invalid fixed minor %d for miscdevice '%s'\n", (*misc).minor, (*misc).name);
        return -EINVAL;
    }

    INIT_LIST_HEAD!(&mut (*misc).list);
    mutex_lock(&raw mut MISC_MTX);

    if is_dynamic {
        let i = misc_minor_alloc((*misc).minor);
        if i < 0 { err = -EBUSY; mutex_unlock(&raw mut MISC_MTX); return err; }
        (*misc).minor = i;
    } else {
        let mut c: *mut MiscDevice;
        list_for_each_entry!(c, &raw mut MISC_LIST, list, {
            if (*c).minor == (*misc).minor { err = -EBUSY; break; }
        });
        if err != 0 { mutex_unlock(&raw mut MISC_MTX); return err; }
        let i = misc_minor_alloc((*misc).minor);
        if i < 0 { err = -EBUSY; mutex_unlock(&raw mut MISC_MTX); return err; }
    }

    dev = MKDEV(MISC_MAJOR, (*misc).minor);
    (*misc).this_device = device_create_with_groups(&MISC_CLASS, (*misc).parent, dev, misc, (*misc).groups, (*misc).name);
    if IS_ERR((*misc).this_device) {
        misc_minor_free((*misc).minor);
        if is_dynamic { (*misc).minor = MISC_DYNAMIC_MINOR; }
        err = PTR_ERR((*misc).this_device);
        mutex_unlock(&raw mut MISC_MTX);
        return err;
    }

    /* Add it to the front, so that later devices can override earlier defaults. */
    list_add(&mut (*misc).list, &raw mut MISC_LIST);
    mutex_unlock(&raw mut MISC_MTX);
    err
}

pub unsafe fn misc_deregister(misc: *mut MiscDevice) {
    mutex_lock(&raw mut MISC_MTX);
    list_del_init(&mut (*misc).list);
    device_destroy(&MISC_CLASS, MKDEV(MISC_MAJOR, (*misc).minor));
    misc_minor_free((*misc).minor);
    if (*misc).minor > MISC_DYNAMIC_MINOR { (*misc).minor = MISC_DYNAMIC_MINOR; }
    mutex_unlock(&raw mut MISC_MTX);
}

unsafe fn misc_init() -> i32 {
    let mut err: i32;
    let mut misc_proc_file: *mut ProcDirEntry;

    misc_proc_file = proc_create_seq!("misc", 0, core::ptr::null_mut(), &MISC_SEQ_OPS);
    err = class_register(&MISC_CLASS);
    if err != 0 { goto_fail_remove!(misc_proc_file, err); }
    err = __register_chrdev(MISC_MAJOR, 0, MINORMASK + 1, b"misc\0".as_ptr() as *const i8, &MISC_FOPS);
    if err < 0 {
        pr_err!("unable to get major %d for misc devices\n", MISC_MAJOR);
        class_unregister(&MISC_CLASS);
        if !misc_proc_file.is_null() { remove_proc_entry!("misc", core::ptr::null_mut()); }
        return err;
    }
    0
}

subsys_initcall!(misc_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
