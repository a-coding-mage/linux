// SPDX-License-Identifier: GPL-2.0
/*
 * 32 bit compatibility code for System V IPC
 *
 * Copyright (C) 1997,1998\tJakub Jelinek (jj@sunsite.mff.cuni.cz)
 * Copyright (C) 1997\t\tDavid S. Miller (davem@caip.rutgers.edu)
 * Copyright (C) 1999\t\tArun Sharma <arun.sharma@intel.com>
 * Copyright (C) 2000\t\tVA Linux Co
 * Copyright (C) 2000\t\tDon Dugger <n0ano@valinux.com>
 * Copyright (C) 2000           Hewlett-Packard Co.
 * Copyright (C) 2000           David Mosberger-Tang <davidm@hpl.hp.com>
 * Copyright (C) 2000           Gerhard Tonn (ton@de.ibm.com)
 * Copyright (C) 2000-2002      Andi Kleen, SuSE Labs (x86-64 port)
 * Copyright (C) 2000\t\tSilicon Graphics, Inc.
 * Copyright (C) 2001\t\tIBM
 * Copyright (C) 2004\t\tIBM Deutschland Entwicklung GmbH, IBM Corporation
 * Copyright (C) 2004\t\tArnd Bergmann (arnd@arndb.de)
 *
 * This code is collected from the versions for sparc64, mips64, s390x, ia64,
 * ppc64 and x86_64, all of which are based on the original sparc64 version
 * by Jakub Jelinek.
 *
 */
// Dependencies supplied by the surrounding kernel translation:
// linux/compat.h, linux/errno.h, linux/highuid.h, linux/init.h, linux/msg.h,
// linux/shm.h, linux/syscalls.h, linux/ptrace.h, linux/mutex.h,
// linux/uaccess.h, and util.h.

pub unsafe fn get_compat_ipc64_perm(
    to: *mut ipc64_perm,
    from: *mut compat_ipc64_perm,
) -> i32 {
    let mut v: compat_ipc64_perm = core::mem::zeroed();
    if copy_from_user(
        (&mut v as *mut compat_ipc64_perm).cast(),
        (from as *const compat_ipc64_perm).cast(),
        core::mem::size_of::<compat_ipc64_perm>(),
    ) != 0
    {
        return -EFAULT;
    }
    (*to).uid = v.uid;
    (*to).gid = v.gid;
    (*to).mode = v.mode;
    0
}

pub unsafe fn get_compat_ipc_perm(
    to: *mut ipc64_perm,
    from: *mut compat_ipc_perm,
) -> i32 {
    let mut v: compat_ipc_perm = core::mem::zeroed();
    if copy_from_user(
        (&mut v as *mut compat_ipc_perm).cast(),
        (from as *const compat_ipc_perm).cast(),
        core::mem::size_of::<compat_ipc_perm>(),
    ) != 0
    {
        return -EFAULT;
    }
    (*to).uid = v.uid;
    (*to).gid = v.gid;
    (*to).mode = v.mode;
    0
}

pub unsafe fn to_compat_ipc64_perm(to: *mut compat_ipc64_perm, from: *mut ipc64_perm) {
    (*to).key = (*from).key;
    (*to).uid = (*from).uid;
    (*to).gid = (*from).gid;
    (*to).cuid = (*from).cuid;
    (*to).cgid = (*from).cgid;
    (*to).mode = (*from).mode;
    (*to).seq = (*from).seq;
}

pub unsafe fn to_compat_ipc_perm(to: *mut compat_ipc_perm, from: *mut ipc64_perm) {
    (*to).key = (*from).key;
    (*to).uid = (*from).uid;
    (*to).gid = (*from).gid;
    (*to).cuid = (*from).cuid;
    (*to).cgid = (*from).cgid;
    (*to).mode = (*from).mode;
    (*to).seq = (*from).seq;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
