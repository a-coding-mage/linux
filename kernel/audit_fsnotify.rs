// SPDX-License-Identifier: GPL-2.0-or-later
/* audit_fsnotify.c -- tracking inodes
 *
 * Copyright 2003-2009,2014-2015 Red Hat, Inc.
 * Copyright 2005 Hewlett-Packard Development Company, L.P.
 * Copyright 2005 IBM Corporation
 */

// Linux kernel dependencies supplied by the surrounding translation.

#[repr(C)]
pub struct audit_fsnotify_mark {
    pub dev: dev_t,
    pub ino: u64,
    pub path: *mut c_char,
    pub mark: fsnotify_mark,
    pub rule: *mut audit_krule,
}

static mut audit_fsnotify_group: *mut fsnotify_group = core::ptr::null_mut();

const AUDIT_FS_EVENTS: u32 = FS_MOVE | FS_CREATE | FS_DELETE | FS_DELETE_SELF | FS_MOVE_SELF;

unsafe fn audit_fsnotify_mark_free(audit_mark: *mut audit_fsnotify_mark) {
    kfree((*audit_mark).path as *mut c_void);
    kfree(audit_mark as *mut c_void);
}

unsafe extern "C" fn audit_fsnotify_free_mark(mark: *mut fsnotify_mark) {
    let audit_mark = container_of!(mark, audit_fsnotify_mark, mark);
    audit_fsnotify_mark_free(audit_mark);
}

pub unsafe extern "C" fn audit_mark_path(mark: *mut audit_fsnotify_mark) -> *mut c_char {
    (*mark).path
}

pub unsafe extern "C" fn audit_mark_compare(
    mark: *mut audit_fsnotify_mark,
    ino: u64,
    dev: dev_t,
) -> c_int {
    if (*mark).ino == AUDIT_INO_UNSET {
        return 0;
    }
    ((*mark).ino == ino && (*mark).dev == dev) as c_int
}

unsafe fn audit_update_mark(audit_mark: *mut audit_fsnotify_mark, inode: *const inode) {
    (*audit_mark).dev = if !inode.is_null() {
        (*(*inode).i_sb).s_dev
    } else {
        AUDIT_DEV_UNSET
    };
    (*audit_mark).ino = if !inode.is_null() {
        (*inode).i_ino
    } else {
        AUDIT_INO_UNSET
    };
}

pub unsafe extern "C" fn audit_alloc_mark(
    krule: *mut audit_krule,
    pathname: *mut c_char,
    len: c_int,
    ctx: *mut audit_watch_ctx,
) -> *mut audit_fsnotify_mark {
    let mut path = core::mem::MaybeUninit::<path>::uninit();
    let mut dentry: *mut dentry = core::ptr::null_mut();
    let mut dir: *mut inode;
    let mut child: *mut inode;
    let mut allow_dups: c_int;

    if *pathname as u8 != b'/' || *pathname.add(len as usize - 1) as u8 == b'/' {
        return ERR_PTR(-EINVAL);
    }

    if ctx.is_null() {
        dentry = kern_path_parent(pathname, path.as_mut_ptr());
        if IS_ERR(dentry) {
            return ERR_CAST(dentry);
        }
        let path_ref = path.assume_init_ref();
        dir = d_inode(path_ref.dentry);
        child = d_inode(dentry);
        allow_dups = 0;
    } else {
        dir = (*ctx).dir;
        child = (*ctx).child;
        allow_dups = 1;
    }

    let audit_mark = kzalloc_obj::<audit_fsnotify_mark>();
    if unlikely(audit_mark.is_null()) {
        let audit_mark = ERR_PTR(-ENOMEM);
        if ctx.is_null() {
            dput(dentry);
            path_put(path.assume_init_mut());
        }
        return audit_mark;
    }

    fsnotify_init_mark(&mut (*audit_mark).mark, audit_fsnotify_group);
    (*audit_mark).mark.mask = AUDIT_FS_EVENTS;
    (*audit_mark).path = pathname;
    (*audit_mark).rule = krule;

    audit_update_mark(audit_mark, child);
    let ret = fsnotify_add_inode_mark(&mut (*audit_mark).mark, dir, allow_dups);
    let result = if ret < 0 {
        (*audit_mark).path = core::ptr::null_mut();
        fsnotify_put_mark(&mut (*audit_mark).mark);
        ERR_PTR(ret)
    } else {
        audit_mark
    };

    if ctx.is_null() {
        dput(dentry);
        path_put(path.assume_init_mut());
    }
    result
}

unsafe fn audit_mark_log_rule_change(audit_mark: *mut audit_fsnotify_mark, op: *mut c_char) {
    let rule = (*audit_mark).rule;
    if !audit_enabled {
        return;
    }
    let ab = audit_log_start(audit_context(), GFP_NOFS, AUDIT_CONFIG_CHANGE);
    if unlikely(ab.is_null()) {
        return;
    }
    audit_log_session_info(ab);
    audit_log_format(ab, c" op=%s path=".as_ptr(), op);
    audit_log_untrustedstring(ab, (*audit_mark).path);
    audit_log_key(ab, (*rule).filterkey);
    audit_log_format(ab, c" list=%d res=1".as_ptr(), (*rule).listnr);
    audit_log_end(ab);
}

pub unsafe extern "C" fn audit_remove_mark(audit_mark: *mut audit_fsnotify_mark) {
    fsnotify_destroy_mark(&mut (*audit_mark).mark, audit_fsnotify_group);
    fsnotify_put_mark(&mut (*audit_mark).mark);
}

pub unsafe extern "C" fn audit_remove_mark_rule(krule: *mut audit_krule) {
    audit_remove_mark((*krule).exe as *mut audit_fsnotify_mark);
}

unsafe fn audit_autoremove_mark_rule(audit_mark: *mut audit_fsnotify_mark) {
    let rule = (*audit_mark).rule;
    let entry = container_of!(rule, audit_entry, rule);
    audit_mark_log_rule_change(audit_mark, c"autoremove_rule".as_ptr() as *mut c_char);
    audit_del_rule(entry);
}

unsafe extern "C" fn audit_mark_handle_event(
    inode_mark: *mut fsnotify_mark,
    mask: u32,
    inode: *mut inode,
    _dir: *mut inode,
    dname: *const qstr,
    _cookie: u32,
) -> c_int {
    let audit_mark = container_of!(inode_mark, audit_fsnotify_mark, mark);
    if WARN_ON_ONCE((*inode_mark).group != audit_fsnotify_group) {
        return 0;
    }
    if mask & (FS_CREATE | FS_MOVED_TO | FS_DELETE | FS_MOVED_FROM) != 0 {
        if audit_compare_dname_path(dname, (*audit_mark).path, AUDIT_NAME_FULL) {
            return 0;
        }
        audit_update_mark(audit_mark, inode);
    } else if mask & (FS_DELETE_SELF | FS_UNMOUNT | FS_MOVE_SELF) != 0 {
        audit_autoremove_mark_rule(audit_mark);
    }
    0
}

#[repr(C)]
static audit_mark_fsnotify_ops: fsnotify_ops = fsnotify_ops {
    handle_inode_event: Some(audit_mark_handle_event),
    free_mark: Some(audit_fsnotify_free_mark),
};

unsafe extern "C" fn audit_fsnotify_init() -> c_int {
    audit_fsnotify_group = fsnotify_alloc_group(&audit_mark_fsnotify_ops, FSNOTIFY_GROUP_DUPS);
    if IS_ERR(audit_fsnotify_group) {
        audit_fsnotify_group = core::ptr::null_mut();
        audit_panic(c"cannot create audit fsnotify group".as_ptr());
    }
    0
}

// device_initcall(audit_fsnotify_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
