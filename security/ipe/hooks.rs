// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2024 Microsoft Corporation. All rights reserved.
 */

// Translated from linux/fs.h, linux/fs_struct.h, linux/types.h,
// linux/binfmts.h, linux/mman.h, linux/blk_types.h, and local IPE headers.
// The concrete types, constants, and external helpers are supplied by the
// surrounding kernel/IPE translation.

use core::ffi::{c_void, c_ulong};
use core::ptr;

/**
 * ipe_bprm_check_security() - ipe security hook function for bprm check.
 * @bprm: Supplies a pointer to a linux_binprm structure to source the file
 *	  being evaluated.
 *
 * This LSM hook is called when a binary is loaded through the exec
 * family of system calls.
 *
 * Return:
 * * %0		- Success
 * * %-EACCES	- Did not pass IPE policy
 */
pub unsafe extern "C" fn ipe_bprm_check_security(bprm: *mut linux_binprm) -> i32 {
    let mut ctx: ipe_eval_ctx = IPE_EVAL_CTX_INIT;

    unsafe {
        ipe_build_eval_ctx(
            &mut ctx,
            (*bprm).file,
            IPE_OP_EXEC,
            IPE_HOOK_BPRM_CHECK,
        );
        ipe_evaluate_event(&mut ctx)
    }
}

/**
 * ipe_bprm_creds_for_exec() - ipe security hook function for bprm creds check.
 * @bprm: Supplies a pointer to a linux_binprm structure to source the file
 *	  being evaluated.
 *
 * This LSM hook is called when userspace signals the kernel to check a file
 * for execution through the execveat syscall with the AT_EXECVE_CHECK flag.
 * The hook triggers IPE policy evaluation on the script file and returns
 * the policy decision to userspace. The userspace program receives the
 * return code and can decide whether to proceed with script execution.
 *
 * Return:
 * * %0		- Success
 * * %-EACCES	- Did not pass IPE policy
 */
pub unsafe extern "C" fn ipe_bprm_creds_for_exec(bprm: *mut linux_binprm) -> i32 {
    let mut ctx: ipe_eval_ctx = IPE_EVAL_CTX_INIT;

    unsafe {
        if !(*bprm).is_check {
            return 0;
        }

        ipe_build_eval_ctx(
            &mut ctx,
            (*bprm).file,
            IPE_OP_EXEC,
            IPE_HOOK_BPRM_CREDS_FOR_EXEC,
        );
        ipe_evaluate_event(&mut ctx)
    }
}

/**
 * ipe_mmap_file() - ipe security hook function for mmap check.
 * @f: File being mmap'd. Can be NULL in the case of anonymous memory.
 * @reqprot: The requested protection on the mmap, passed from usermode.
 * @prot: The effective protection on the mmap, resolved from reqprot and
 *	  system configuration.
 * @flags: Unused.
 *
 * This hook is called when a file is loaded through the mmap
 * family of system calls.
 *
 * Return:
 * * %0		- Success
 * * %-EACCES	- Did not pass IPE policy
 */
pub unsafe extern "C" fn ipe_mmap_file(
    f: *mut file,
    _reqprot: c_ulong,
    prot: c_ulong,
    _flags: c_ulong,
) -> i32 {
    let mut ctx: ipe_eval_ctx = IPE_EVAL_CTX_INIT;

    if prot & PROT_EXEC != 0 {
        unsafe {
            ipe_build_eval_ctx(&mut ctx, f, IPE_OP_EXEC, IPE_HOOK_MMAP);
            return ipe_evaluate_event(&mut ctx);
        }
    }

    0
}

/**
 * ipe_file_mprotect() - ipe security hook function for mprotect check.
 * @vma: Existing virtual memory area created by mmap or similar.
 * @reqprot: The requested protection on the mmap, passed from usermode.
 * @prot: The effective protection on the mmap, resolved from reqprot and
 *	  system configuration.
 *
 * This LSM hook is called when a mmap'd region of memory is changing
 * its protections via mprotect.
 *
 * Return:
 * * %0		- Success
 * * %-EACCES	- Did not pass IPE policy
 */
pub unsafe extern "C" fn ipe_file_mprotect(
    vma: *mut vm_area_struct,
    _reqprot: c_ulong,
    prot: c_ulong,
) -> i32 {
    let mut ctx: ipe_eval_ctx = IPE_EVAL_CTX_INIT;

    unsafe {
        /* Already Executable */
        if (*vma).vm_flags & VM_EXEC != 0 {
            return 0;
        }

        if prot & PROT_EXEC != 0 {
            ipe_build_eval_ctx(
                &mut ctx,
                (*vma).vm_file,
                IPE_OP_EXEC,
                IPE_HOOK_MPROTECT,
            );
            return ipe_evaluate_event(&mut ctx);
        }
    }

    0
}

/**
 * ipe_kernel_read_file() - ipe security hook function for kernel read.
 * @file: Supplies a pointer to the file structure being read in from disk.
 * @id: Supplies the enumeration identifying the purpose of the read.
 * @contents: Unused.
 *
 * This LSM hook is called when a file is read from disk in the kernel.
 *
 * Return:
 * * %0		- Success
 * * %-EACCES	- Did not pass IPE policy
 */
pub unsafe extern "C" fn ipe_kernel_read_file(
    file: *mut file,
    id: kernel_read_file_id,
    _contents: bool,
) -> i32 {
    let mut ctx: ipe_eval_ctx = IPE_EVAL_CTX_INIT;
    let op: ipe_op_type;

    match id {
        READING_FIRMWARE => {
            op = IPE_OP_FIRMWARE;
        }
        READING_MODULE | READING_MODULE_COMPRESSED => {
            op = IPE_OP_KERNEL_MODULE;
        }
        READING_KEXEC_INITRAMFS => {
            op = IPE_OP_KEXEC_INITRAMFS;
        }
        READING_KEXEC_IMAGE => {
            op = IPE_OP_KEXEC_IMAGE;
        }
        READING_POLICY => {
            op = IPE_OP_POLICY;
        }
        READING_X509_CERTIFICATE => {
            op = IPE_OP_X509;
        }
        _ => {
            op = IPE_OP_INVALID;
            WARN(
                1,
                c"no rule setup for kernel_read_file enum %d".as_ptr(),
                id,
            );
        }
    }

    unsafe {
        ipe_build_eval_ctx(&mut ctx, file, op, IPE_HOOK_KERNEL_READ);
        ipe_evaluate_event(&mut ctx)
    }
}

/**
 * ipe_kernel_load_data() - ipe security hook function for kernel load data.
 * @id: Supplies the enumeration identifying the purpose of the load.
 * @contents: Unused.
 *
 * This LSM hook is called when a data buffer provided by userspace is loading
 * into the kernel.
 *
 * Return:
 * * %0		- Success
 * * %-EACCES	- Did not pass IPE policy
 */
pub unsafe extern "C" fn ipe_kernel_load_data(id: kernel_load_data_id, _contents: bool) -> i32 {
    let mut ctx: ipe_eval_ctx = IPE_EVAL_CTX_INIT;
    let op: ipe_op_type;

    match id {
        LOADING_FIRMWARE => {
            op = IPE_OP_FIRMWARE;
        }
        LOADING_MODULE => {
            op = IPE_OP_KERNEL_MODULE;
        }
        LOADING_KEXEC_INITRAMFS => {
            op = IPE_OP_KEXEC_INITRAMFS;
        }
        LOADING_KEXEC_IMAGE => {
            op = IPE_OP_KEXEC_IMAGE;
        }
        LOADING_POLICY => {
            op = IPE_OP_POLICY;
        }
        LOADING_X509_CERTIFICATE => {
            op = IPE_OP_X509;
        }
        _ => {
            op = IPE_OP_INVALID;
            WARN(
                1,
                c"no rule setup for kernel_load_data enum %d".as_ptr(),
                id,
            );
        }
    }

    unsafe {
        ipe_build_eval_ctx(&mut ctx, ptr::null_mut(), op, IPE_HOOK_KERNEL_LOAD);
        ipe_evaluate_event(&mut ctx)
    }
}

/**
 * ipe_unpack_initramfs() - Mark the current rootfs as initramfs.
 */
pub unsafe extern "C" fn ipe_unpack_initramfs() {
    unsafe {
        (*ipe_sb((*(*(*(*current).fs).root.mnt).mnt_sb))).initramfs = true;
    }
}

// Original conditional: CONFIG_IPE_PROP_DM_VERITY
#[cfg(CONFIG_IPE_PROP_DM_VERITY)]
/**
 * ipe_bdev_free_security() - Free IPE's LSM blob of block_devices.
 * @bdev: Supplies a pointer to a block_device that contains the structure
 *	  to free.
 */
pub unsafe extern "C" fn ipe_bdev_free_security(bdev: *mut block_device) {
    let blob: *mut ipe_bdev = unsafe { ipe_bdev(bdev) };

    unsafe {
        ipe_digest_free((*blob).root_hash);
    }
}

// Original nested conditional: CONFIG_IPE_PROP_DM_VERITY_SIGNATURE
#[cfg(all(CONFIG_IPE_PROP_DM_VERITY, CONFIG_IPE_PROP_DM_VERITY_SIGNATURE))]
unsafe fn ipe_set_dmverity_signature(blob: *mut ipe_bdev, value: *const c_void, size: usize) {
    unsafe {
        (*blob).dm_verity_signed = size > 0 && !value.is_null();
    }
}

#[cfg(all(CONFIG_IPE_PROP_DM_VERITY, not(CONFIG_IPE_PROP_DM_VERITY_SIGNATURE)))]
#[inline]
unsafe fn ipe_set_dmverity_signature(_blob: *mut ipe_bdev, _value: *const c_void, _size: usize) {}

#[cfg(CONFIG_IPE_PROP_DM_VERITY)]
/**
 * ipe_bdev_setintegrity() - Save integrity data from a bdev to IPE's LSM blob.
 * @bdev: Supplies a pointer to a block_device that contains the LSM blob.
 * @type: Supplies the integrity type.
 * @value: Supplies the value to store.
 * @size: The size of @value.
 *
 * This hook is currently used to save dm-verity's root hash or the existence
 * of a validated signed dm-verity root hash into LSM blob.
 *
 * Return: %0 on success. If an error occurs, the function will return the
 * -errno.
 */
pub unsafe extern "C" fn ipe_bdev_setintegrity(
    bdev: *mut block_device,
    r#type: lsm_integrity_type,
    value: *const c_void,
    size: usize,
) -> i32 {
    let mut digest: *const dm_verity_digest = ptr::null();
    let blob: *mut ipe_bdev = unsafe { ipe_bdev(bdev) };
    let mut info: *mut digest_info = ptr::null_mut();

    if r#type == LSM_INT_DMVERITY_SIG_VALID {
        unsafe {
            ipe_set_dmverity_signature(blob, value, size);
        }

        return 0;
    }

    if r#type != LSM_INT_DMVERITY_ROOTHASH {
        return -EINVAL;
    }

    unsafe {
        if value.is_null() {
            ipe_digest_free((*blob).root_hash);
            (*blob).root_hash = ptr::null_mut();

            return 0;
        }
        digest = value as *const dm_verity_digest;

        info = kzalloc_obj::<digest_info>();
        if info.is_null() {
            return -ENOMEM;
        }

        (*info).digest = kmemdup((*digest).digest, (*digest).digest_len, GFP_KERNEL);
        if (*info).digest.is_null() {
            ipe_bdev_setintegrity_err(info);
            return -ENOMEM;
        }

        (*info).alg = kstrdup((*digest).alg, GFP_KERNEL);
        if (*info).alg.is_null() {
            ipe_bdev_setintegrity_err(info);
            return -ENOMEM;
        }

        (*info).digest_len = (*digest).digest_len;

        ipe_digest_free((*blob).root_hash);
        (*blob).root_hash = info;
    }

    0
}

#[cfg(CONFIG_IPE_PROP_DM_VERITY)]
unsafe fn ipe_bdev_setintegrity_err(info: *mut digest_info) {
    unsafe {
        ipe_digest_free(info);
    }
}

// Original conditional: CONFIG_IPE_PROP_FS_VERITY_BUILTIN_SIG
#[cfg(CONFIG_IPE_PROP_FS_VERITY_BUILTIN_SIG)]
/**
 * ipe_inode_setintegrity() - save integrity data from a inode to IPE's LSM blob.
 * @inode: The inode to source the security blob from.
 * @type: Supplies the integrity type.
 * @value: The value to be stored.
 * @size: The size of @value.
 *
 * This hook is currently used to save the existence of a validated fs-verity
 * builtin signature into LSM blob.
 *
 * Return: %0 on success. If an error occurs, the function will return the
 * -errno.
 */
pub unsafe extern "C" fn ipe_inode_setintegrity(
    inode: *const inode,
    r#type: lsm_integrity_type,
    value: *const c_void,
    size: usize,
) -> i32 {
    let inode_sec: *mut ipe_inode = unsafe { ipe_inode(inode) };

    if r#type == LSM_INT_FSVERITY_BUILTINSIG_VALID {
        unsafe {
            (*inode_sec).fs_verity_signed = size > 0 && !value.is_null();
        }
        return 0;
    }

    -EINVAL
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
