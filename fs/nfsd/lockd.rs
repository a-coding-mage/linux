// SPDX-License-Identifier: GPL-2.0
/*
 * This file contains all the stubs needed when communicating with lockd.
 * This level of indirection is necessary so we can run nfsd+lockd without
 * requiring the nfs client to be compiled in/loaded, and vice versa.
 *
 * Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
 */

// Dependencies supplied by the surrounding kernel/NFSD translation unit:
// linux/file.h, linux/lockd/bind.h, nfsd.h, and vfs.h.

const NFSDDBG_FACILITY: i32 = NFSDDBG_LOCKD;

/**
 * nlm_fopen - Open an NFSD file
 * @rqstp: NLM RPC procedure execution context
 * @f: NFS file handle to be opened
 * @filp: OUT: an opened struct file
 * @flags: the POSIX open flags to use
 *
 * nlm_fopen() holds the dentry reference until nlm_fclose() releases it.
 *
 * Returns zero on success or a negative errno value if the file
 * cannot be opened.
 */
unsafe fn nlm_fopen(
    rqstp: *mut svc_rqst,
    f: *mut nfs_fh,
    filp: *mut *mut file,
    flags: i32,
) -> i32 {
    let nfserr: __be32;
    let mut access: i32;
    let mut fh: svc_fh = core::mem::zeroed();

    // must initialize before using! but maxsize doesn't matter
    fh_init(&mut fh, 0);
    (*fh.fh_handle).fh_size = (*f).size;
    core::ptr::copy_nonoverlapping(
        (*f).data.as_ptr(),
        (*fh.fh_handle).fh_raw.as_mut_ptr(),
        (*f).size as usize,
    );
    fh.fh_export = core::ptr::null_mut();

    /*
     * Allow BYPASS_GSS as some client implementations use AUTH_SYS
     * for NLM even when GSS is used for NFS.
     * Allow OWNER_OVERRIDE as permission might have been changed
     * after the file was opened.
     * Pass MAY_NLM so that authentication can be completely bypassed
     * if NFSEXP_NOAUTHNLM is set.  Some older clients use AUTH_NULL
     * for NLM requests.
     */
    access = if flags == O_WRONLY { NFSD_MAY_WRITE } else { NFSD_MAY_READ };
    access |= NFSD_MAY_NLM | NFSD_MAY_OWNER_OVERRIDE | NFSD_MAY_BYPASS_GSS;
    nfserr = nfsd_open(rqstp, &mut fh, S_IFREG, access, filp);
    fh_put(&mut fh);

    match nfserr {
        nfs_ok => {}
        nfserr_jukebox => {
            /*
             * This error can indicate a presence of a conflicting
             * delegation to an NLM lock request. Options are:
             * (1) For now, drop this request and make the client
             * retry. When delegation is returned, client's lock retry
             * will complete.
             * (2) NLM4_DENIED as per "spec" signals to the client
             * that the lock is unavailable now but client can retry.
             * Linux client implementation does not. It treats
             * NLM4_DENIED same as NLM4_FAILED and fails the request.
             * (3) For the future, treat this as blocked lock and try
             * to callback when the delegation is returned but might
             * not have a proper lock request to block on.
             */
            return -EWOULDBLOCK;
        }
        nfserr_stale => return -ESTALE,
        _ => return -ENOLCK,
    }

    0
}

/**
 * nlm_fclose - Close an NFSD file
 * @filp: a struct file that was opened by nlm_fopen()
 */
unsafe fn nlm_fclose(filp: *mut file) {
    fput(filp);
}

static nfsd_nlm_ops: nlmsvc_binding = nlmsvc_binding {
    owner: THIS_MODULE,
    fopen: Some(nlm_fopen), // open file for locking
    fclose: Some(nlm_fclose), // close file
};

unsafe fn nfsd_lockd_init() {
    dprintk!("nfsd: initializing lockd\n");
    rcu_assign_pointer!(nlmsvc_ops, &nfsd_nlm_ops);
}

unsafe fn nfsd_lockd_shutdown() {
    RCU_INIT_POINTER!(nlmsvc_ops, core::ptr::null_mut());
    synchronize_rcu();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
