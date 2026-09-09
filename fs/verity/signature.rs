// SPDX-License-Identifier: GPL-2.0
/*
 * Verification of builtin signatures
 *
 * Copyright 2019 Google LLC
 */

/*
 * This file implements verification of fs-verity builtin signatures.  Please
 * take great care before using this feature.  It is not the only way to do
 * signatures with fs-verity, and the alternatives (such as userspace signature
 * verification, and IMA appraisal) can be much better.  For details about the
 * limitations of this feature, see Documentation/filesystems/fsverity.rst.
 */

// Dependency declarations supplied by the surrounding kernel translation.

/*
 * /proc/sys/fs/verity/require_signatures
 * If 1, all verity files must have a valid builtin signature.
 */
pub static mut fsverity_require_signatures: i32 = 0;

/*
 * Keyring that contains the trusted X.509 certificates.
 *
 * Only root (kuid=0) can modify this.  Also, root may use
 * keyctl_restrict_keyring() to prevent any more additions.
 */
static mut fsverity_keyring: *mut key = core::ptr::null_mut();

/**
 * fsverity_verify_signature() - check a verity file's signature
 * @vi: the file's fsverity_info
 * @signature: the file's built-in signature
 * @sig_size: size of signature in bytes, or 0 if no signature
 *
 * If the file includes a signature of its fs-verity file digest, verify it
 * against the certificates in the fs-verity keyring. Note that signatures
 * are verified regardless of the state of the 'fsverity_require_signatures'
 * variable and the LSM subsystem relies on this behavior to help enforce
 * file integrity policies. Please discuss changes with the LSM list
 * (thank you!).
 *
 * Return: 0 on success (signature valid or not required); -errno on failure
 */
pub unsafe fn fsverity_verify_signature(
    vi: *const fsverity_info,
    signature: *const u8,
    sig_size: usize,
) -> i32 {
    let inode = (*vi).inode;
    let hash_alg = (*vi).tree_params.hash_alg;
    let mut d: *mut fsverity_formatted_digest;
    let mut err: i32;

    if sig_size == 0 {
        if fsverity_require_signatures != 0 {
            fsverity_err(inode, "require_signatures=1, rejecting unsigned file!");
            return -EPERM;
        }
        return 0;
    }

    if (*fsverity_keyring).keys.nr_leaves_on_tree == 0 {
        /*
         * The ".fs-verity" keyring is empty, due to builtin signatures
         * being supported by the kernel but not actually being used.
         * In this case, verify_pkcs7_signature() would always return an
         * error, usually ENOKEY.  It could also be EBADMSG if the
         * PKCS#7 is malformed, but that isn't very important to
         * distinguish.  So, just skip to ENOKEY to avoid the attack
         * surface of the PKCS#7 parser, which would otherwise be
         * reachable by any task able to execute FS_IOC_ENABLE_VERITY.
         */
        fsverity_err(inode, "fs-verity keyring is empty, rejecting signed file!");
        return -ENOKEY;
    }

    d = kzalloc(core::mem::size_of::<fsverity_formatted_digest>() + (*hash_alg).digest_size as usize, GFP_KERNEL);
    if d.is_null() {
        return -ENOMEM;
    }
    core::ptr::copy_nonoverlapping(b"FSVerity".as_ptr(), (*d).magic.as_mut_ptr(), 8);
    (*d).digest_algorithm = cpu_to_le16(hash_alg.offset_from(fsverity_hash_algs) as u16);
    (*d).digest_size = cpu_to_le16((*hash_alg).digest_size);
    core::ptr::copy_nonoverlapping((*vi).file_digest.as_ptr(), (*d).digest.as_mut_ptr(), (*hash_alg).digest_size as usize);

    err = verify_pkcs7_signature(
        d as *const _,
        core::mem::size_of::<fsverity_formatted_digest>() + (*hash_alg).digest_size as usize,
        signature,
        sig_size,
        fsverity_keyring,
        VERIFYING_UNSPECIFIED_SIGNATURE,
        core::ptr::null(),
        core::ptr::null_mut(),
    );
    kfree(d as *mut core::ffi::c_void);

    if err != 0 {
        if err == -ENOKEY {
            fsverity_err(inode, "File's signing cert isn't in the fs-verity keyring");
        } else if err == -EKEYREJECTED {
            fsverity_err(inode, "Incorrect file signature");
        } else if err == -EBADMSG {
            fsverity_err(inode, "Malformed file signature");
        } else {
            fsverity_err(inode, "Error %d verifying file signature", err);
        }
        return err;
    }

    err = security_inode_setintegrity(
        inode,
        LSM_INT_FSVERITY_BUILTINSIG_VALID,
        signature,
        sig_size,
    );

    if err != 0 {
        fsverity_err(inode, "Error %d exposing file signature to LSMs", err);
        return err;
    }

    0
}

pub unsafe fn fsverity_init_signature() {
    fsverity_keyring = keyring_alloc(
        ".fs-verity",
        KUIDT_INIT(0),
        KGIDT_INIT(0),
        current_cred(),
        KEY_POS_SEARCH | KEY_USR_VIEW | KEY_USR_READ | KEY_USR_WRITE |
            KEY_USR_SEARCH | KEY_USR_SETATTR,
        KEY_ALLOC_NOT_IN_QUOTA,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    );
    if IS_ERR(fsverity_keyring) {
        panic!("failed to allocate \".fs-verity\" keyring");
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
