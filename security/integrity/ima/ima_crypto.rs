// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2005,2006,2007,2008 IBM Corporation
 *
 * Authors:
 * Mimi Zohar <zohar@us.ibm.com>
 * Kylene Hall <kjhall@us.ibm.com>
 *
 * File: ima_crypto.c
 *	Calculates md5/sha1 file hash, template hash, boot-aggreate hash
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong};
use core::ptr;

// External types from kernel headers
extern "C" {
    type crypto_shash;
    type file;
    type ima_digest_data;
    type ima_field_data;
    type ima_template_entry;
    type ima_template_desc;
    type tpm_digest;
    type ima_algo_desc;
    type shash_desc;
    type inode;
    type path;
    type cred;
    type tpm_chip_struct;
}

type loff_t = i64;
type u8 = u8;
type u16 = u16;
type u32 = u32;

// External variables from other kernel modules
extern "C" {
    static hash_algo_name: *const *const c_char;
    static hash_digest_size: *const c_uint;
    static ima_tpm_chip: *mut tpm_chip_struct;
    static mut ima_canonical_fmt: i32;
}

// External functions from kernel headers
extern "C" {
    fn crypto_alloc_shash(alg_name: *const c_char, type_: c_uint, mask: c_uint) -> *mut crypto_shash;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> i32;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> i64;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_devel(fmt: *const c_char, ...);
    fn crypto_free_shash(tfm: *mut crypto_shash);
    fn crypto_shash_digestsize(tfm: *mut crypto_shash) -> c_uint;
    fn crypto_shash_init(desc: *mut shash_desc) -> c_int;
    fn crypto_shash_update(desc: *mut shash_desc, data: *const u8, len: c_uint) -> c_int;
    fn crypto_shash_final(desc: *mut shash_desc, out: *mut u8) -> c_int;
    fn i_size_read(inode: *mut inode) -> loff_t;
    fn file_inode(file: *mut file) -> *mut inode;
    fn kzalloc(size: c_ulong, flags: c_uint) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn integrity_kernel_read(file: *mut file, offset: loff_t, buf: *mut c_char, len: c_uint) -> c_int;
    fn dentry_open(path: *const path, flags: c_int, cred: *const cred) -> *mut file;
    fn fput(f: *mut file);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: c_ulong) -> *mut core::ffi::c_void;
    fn tpm_pcr_read(chip: *mut tpm_chip_struct, pcr_idx: u32, digest: *mut tpm_digest) -> c_int;
    fn cpu_to_le32(val: u32) -> u32;
}

// Macros from kernel headers - preserved as comments since C preprocessor macros
// are mapped to Rust where possible
// #define HASH_ALGO__LAST (external enum value)
// #define HASH_ALGO_SHA1 (external enum value)
// #define HASH_ALGO_SHA256 (external enum value)
// #define IMA_TEMPLATE_IMA_NAME (external constant)
// #define IMA_EVENT_NAME_LEN_MAX (external value)
// #define TPM_ALG_SHA1 (external constant)
// #define TPM_DIGEST_SIZE (external value)
// #define SHA1_DIGEST_SIZE (external value)
// #define TPM_PCR0 (external constant)
// #define TPM_PCR8 (external constant)
// #define TPM_PCR10 (external constant)
// #define PAGE_SIZE (external value)
// #define GFP_KERNEL (external flag)
// #define O_DIRECT (external flag)
// #define O_RDONLY (external flag)
// #define O_WRONLY (external flag)
// #define O_APPEND (external flag)
// #define O_TRUNC (external flag)
// #define O_CREAT (external flag)
// #define O_NOCTTY (external flag)
// #define O_EXCL (external flag)
// #define ENOMEM (-12)
// #define EINVAL (-22)
// #define NR_BANKS(chip) (chip->nr_allocated_banks)
// #define SHASH_DESC_ON_STACK(shash, tfm) char shash[sizeof(struct shash_desc) + crypto_shash_descsize(tfm)]

// External constants (values defined in kernel headers)
extern "C" {
    static HASH_ALGO__LAST: i32;
    static HASH_ALGO_SHA1: i32;
    static HASH_ALGO_SHA256: i32;
    static IMA_TEMPLATE_IMA_NAME: *const c_char;
    static IMA_EVENT_NAME_LEN_MAX: usize;
    static TPM_ALG_SHA1: u16;
    static TPM_DIGEST_SIZE: usize;
    static SHA1_DIGEST_SIZE: u32;
    static TPM_PCR0: u32;
    static TPM_PCR8: u32;
    static TPM_PCR10: u32;
    static PAGE_SIZE: usize;
    static GFP_KERNEL: c_uint;
    static O_DIRECT: c_int;
    static O_RDONLY: c_int;
    static O_WRONLY: c_int;
    static O_APPEND: c_int;
    static O_TRUNC: c_int;
    static O_CREAT: c_int;
    static O_NOCTTY: c_int;
    static O_EXCL: c_int;
    static FMODE_READ: c_ulong;
}

const ENOMEM: i64 = -12;
const EINVAL: i64 = -22;

// Static global variable
static mut ima_shash_tfm: *mut crypto_shash = ptr::null_mut();

// Variables marked __ro_after_init - in Rust we use static mut but document the constraint
static mut ima_sha1_idx: c_int = 0;
static mut ima_hash_algo_idx: c_int = 0;

// Additional number of slots reserved, as needed, for SHA1
// and IMA default algo.
static mut ima_extra_slots: c_int = 0;

static mut ima_algo_array: *mut ima_algo_desc = ptr::null_mut();

// Compile-time constant declarations for external enums
// These values are defined in other kernel modules
extern "C" {
    static ima_hash_algo: i32;
}

// Equivalent of the NR_BANKS(chip) macro.
unsafe fn nr_banks(chip: *mut tpm_chip_struct) -> i32 {
    (*(chip)).nr_allocated_banks as i32
}

unsafe fn ima_init_ima_crypto() -> i64 {
    let rc: i64;

    ima_shash_tfm = crypto_alloc_shash(
        *hash_algo_name.add(ima_hash_algo as usize),
        0,
        0,
    );
    if IS_ERR(ima_shash_tfm as *const core::ffi::c_void) != 0 {
        rc = PTR_ERR(ima_shash_tfm as *const core::ffi::c_void);
        pr_err(
            b"Can not allocate %s (reason: %ld)\n\0".as_ptr() as *const c_char,
            *hash_algo_name.add(ima_hash_algo as usize),
            rc,
        );
        return rc;
    }
    pr_info(
        b"Allocated hash algorithm: %s\n\0".as_ptr() as *const c_char,
        *hash_algo_name.add(ima_hash_algo as usize),
    );
    0
}

unsafe fn ima_alloc_tfm(mut algo: i32) -> *mut crypto_shash {
    let mut tfm: *mut crypto_shash = ima_shash_tfm;
    let mut rc: i64;
    let mut i: i32;

    if algo < 0 || algo >= HASH_ALGO__LAST {
        algo = ima_hash_algo;
    }

    if algo == ima_hash_algo {
        return tfm;
    }

    let banks = nr_banks(ima_tpm_chip);
    i = 0;
    while i < banks + ima_extra_slots {
        if !(*ima_algo_array.add(i as usize)).tfm.is_null()
            && (*ima_algo_array.add(i as usize)).algo == algo
        {
            return (*ima_algo_array.add(i as usize)).tfm;
        }
        i += 1;
    }

    tfm = crypto_alloc_shash(
        *hash_algo_name.add(algo as usize),
        0,
        0,
    );
    if IS_ERR(tfm as *const core::ffi::c_void) != 0 {
        rc = PTR_ERR(tfm as *const core::ffi::c_void);
        pr_err(
            b"Can not allocate %s (reason: %d)\n\0".as_ptr() as *const c_char,
            *hash_algo_name.add(algo as usize),
            rc as c_int,
        );
    }
    tfm
}

pub unsafe fn ima_init_crypto() -> i64 {
    let mut digest_size: c_uint;
    let mut algo: i32;
    let mut rc: i64;
    let mut i: i32;

    rc = ima_init_ima_crypto();
    if rc != 0 {
        return rc;
    }

    ima_sha1_idx = -1;
    ima_hash_algo_idx = -1;

    let banks = nr_banks(ima_tpm_chip);
    i = 0;
    while i < banks {
        // Access allocated_banks array from ima_tpm_chip
        // This assumes the field exists; exact access depends on struct definition
        algo = (*ima_tpm_chip).allocated_banks[i as usize].crypto_id;
        if algo == HASH_ALGO_SHA1 {
            ima_sha1_idx = i;
        }

        if algo == ima_hash_algo {
            ima_hash_algo_idx = i;
        }
        i += 1;
    }

    if ima_sha1_idx < 0 {
        ima_sha1_idx = banks + ima_extra_slots;
        ima_extra_slots += 1;
        if ima_hash_algo == HASH_ALGO_SHA1 {
            ima_hash_algo_idx = ima_sha1_idx;
        }
    }

    if ima_hash_algo_idx < 0 {
        ima_hash_algo_idx = banks + ima_extra_slots;
        ima_extra_slots += 1;
    }

    ima_algo_array = kzalloc(
        (banks + ima_extra_slots) as c_ulong * core::mem::size_of::<ima_algo_desc>() as c_ulong,
        GFP_KERNEL,
    ) as *mut ima_algo_desc;
    if ima_algo_array.is_null() {
        rc = ENOMEM;
        // goto out
        goto_out(rc);
        return rc;
    }

    i = 0;
    while i < banks {
        algo = (*ima_tpm_chip).allocated_banks[i as usize].crypto_id;
        digest_size = (*ima_tpm_chip).allocated_banks[i as usize].digest_size;
        (*ima_algo_array.add(i as usize)).algo = algo;
        (*ima_algo_array.add(i as usize)).digest_size = digest_size;

        // unknown TPM algorithm
        if algo == HASH_ALGO__LAST {
            i += 1;
            continue;
        }

        if algo == ima_hash_algo {
            (*ima_algo_array.add(i as usize)).tfm = ima_shash_tfm;
            i += 1;
            continue;
        }

        (*ima_algo_array.add(i as usize)).tfm = ima_alloc_tfm(algo);
        if IS_ERR((*ima_algo_array.add(i as usize)).tfm as *const core::ffi::c_void) != 0 {
            if algo == HASH_ALGO_SHA1 {
                rc = PTR_ERR((*ima_algo_array.add(i as usize)).tfm as *const core::ffi::c_void);
                (*ima_algo_array.add(i as usize)).tfm = ptr::null_mut();
                goto_out_array(rc);
                return rc;
            }

            (*ima_algo_array.add(i as usize)).tfm = ptr::null_mut();
        }
        i += 1;
    }

    if ima_sha1_idx >= banks {
        if ima_hash_algo == HASH_ALGO_SHA1 {
            (*ima_algo_array.add(ima_sha1_idx as usize)).tfm = ima_shash_tfm;
        } else {
            (*ima_algo_array.add(ima_sha1_idx as usize)).tfm =
                ima_alloc_tfm(HASH_ALGO_SHA1);
            if IS_ERR((*ima_algo_array.add(ima_sha1_idx as usize)).tfm as *const core::ffi::c_void)
                != 0
            {
                rc = PTR_ERR((*ima_algo_array.add(ima_sha1_idx as usize)).tfm as *const core::ffi::c_void);
                goto_out_array(rc);
                return rc;
            }
        }

        (*ima_algo_array.add(ima_sha1_idx as usize)).algo = HASH_ALGO_SHA1;
        (*ima_algo_array.add(ima_sha1_idx as usize)).digest_size = SHA1_DIGEST_SIZE;
    }

    if ima_hash_algo_idx >= banks && ima_hash_algo_idx != ima_sha1_idx {
        digest_size = *hash_digest_size.add(ima_hash_algo as usize);
        (*ima_algo_array.add(ima_hash_algo_idx as usize)).tfm = ima_shash_tfm;
        (*ima_algo_array.add(ima_hash_algo_idx as usize)).algo = ima_hash_algo;
        (*ima_algo_array.add(ima_hash_algo_idx as usize)).digest_size = digest_size;
    }

    0
}

// Helper function to simulate goto out_array
unsafe fn goto_out_array(rc: i64) {
    let banks = nr_banks(ima_tpm_chip);
    let mut i: i32 = 0;
    while i < banks + ima_extra_slots {
        if !(*ima_algo_array.add(i as usize)).tfm.is_null()
            && (*ima_algo_array.add(i as usize)).tfm != ima_shash_tfm
        {
            // continue
        } else {
            i += 1;
            continue;
        }

        crypto_free_shash((*ima_algo_array.add(i as usize)).tfm);
        i += 1;
    }
    kfree(ima_algo_array as *mut core::ffi::c_void);
}

// Helper function to simulate goto out
unsafe fn goto_out(rc: i64) {
    crypto_free_shash(ima_shash_tfm);
}

unsafe fn ima_free_tfm(tfm: *mut crypto_shash) {
    let mut i: i32;

    if tfm == ima_shash_tfm {
        return;
    }

    let banks = nr_banks(ima_tpm_chip);
    i = 0;
    while i < banks + ima_extra_slots {
        if (*ima_algo_array.add(i as usize)).tfm == tfm {
            return;
        }
        i += 1;
    }

    crypto_free_shash(tfm);
}

unsafe fn ima_calc_file_hash_tfm(
    file: *mut file,
    hash: *mut ima_digest_data,
    tfm: *mut crypto_shash,
) -> i64 {
    let mut i_size: loff_t;
    let mut offset: loff_t = 0;
    let mut rbuf: *mut c_char;
    let mut rc: i64;
    let mut shash: *mut shash_desc;

    shash = core::alloc::alloc(
        core::alloc::Layout::new::<shash_desc>()
    ) as *mut shash_desc;

    (*shash).tfm = tfm;

    (*hash).length = crypto_shash_digestsize(tfm);

    rc = crypto_shash_init(shash) as i64;
    if rc != 0 {
        return rc;
    }

    i_size = i_size_read(file_inode(file));

    if i_size == 0 {
        return 0;
    }

    rbuf = kzalloc(PAGE_SIZE as c_ulong, GFP_KERNEL) as *mut c_char;
    if rbuf.is_null() {
        return ENOMEM;
    }

    while offset < i_size {
        let mut rbuf_len: c_int;

        rbuf_len = integrity_kernel_read(file, offset, rbuf, PAGE_SIZE as c_uint);
        if rbuf_len < 0 {
            rc = rbuf_len as i64;
            break;
        }
        if rbuf_len == 0 {
            rc = EINVAL;
            break;
        }
        offset += rbuf_len as loff_t;

        rc = crypto_shash_update(shash, rbuf as *const u8, rbuf_len as c_uint) as i64;
        if rc != 0 {
            break;
        }
    }
    kfree(rbuf as *mut core::ffi::c_void);

    if rc == 0 {
        rc = crypto_shash_final(shash, (*hash).digest as *mut u8) as i64;
    }
    rc
}

// ima_calc_file_hash - calculate file hash
pub unsafe fn ima_calc_file_hash(
    file: *mut file,
    hash: *mut ima_digest_data,
) -> i64 {
    let mut rc: i64;
    let mut f: *mut file = file;
    let mut new_file_instance: bool = false;
    let mut tfm: *mut crypto_shash;

    // For consistency, fail file's opened with the O_DIRECT flag on
    // filesystems mounted with/without DAX option.
    if (*file).f_flags & O_DIRECT != 0 {
        (*hash).length = *hash_digest_size.add(ima_hash_algo as usize);
        (*hash).algo = ima_hash_algo;
        return EINVAL;
    }

    // Open a new file instance in O_RDONLY if we cannot read
    if (*file).f_mode & FMODE_READ == 0 {
        let mut flags: c_int = (*file).f_flags & !(O_WRONLY | O_APPEND | O_TRUNC | O_CREAT | O_NOCTTY | O_EXCL);
        flags |= O_RDONLY;
        f = dentry_open(&(*file).f_path, flags, (*file).f_cred);
        if IS_ERR(f as *const core::ffi::c_void) != 0 {
            return PTR_ERR(f as *const core::ffi::c_void);
        }

        new_file_instance = true;
    }

    tfm = ima_alloc_tfm((*hash).algo);
    if IS_ERR(tfm as *const core::ffi::c_void) != 0 {
        rc = PTR_ERR(tfm as *const core::ffi::c_void);
    } else {
        rc = ima_calc_file_hash_tfm(f, hash, tfm);
        ima_free_tfm(tfm);
    }
    if new_file_instance {
        fput(f);
    }
    rc
}

// Calculate the hash of template data
unsafe fn ima_calc_field_array_hash_tfm(
    field_data: *mut ima_field_data,
    entry: *mut ima_template_entry,
    tfm_idx: i32,
) -> i64 {
    let mut shash: *mut shash_desc;
    let td: *mut ima_template_desc = (*entry).template_desc;
    let num_fields: i32 = (*(*entry).template_desc).num_fields;
    let mut rc: i64;
    let mut i: i32;

    shash = core::alloc::alloc(
        core::alloc::Layout::new::<shash_desc>()
    ) as *mut shash_desc;

    (*shash).tfm = (*ima_algo_array.add(tfm_idx as usize)).tfm;

    rc = crypto_shash_init(shash) as i64;
    if rc != 0 {
        return rc;
    }

    i = 0;
    while i < num_fields {
        let mut buffer: [u8; 256] = [0; 256]; // IMA_EVENT_NAME_LEN_MAX + 1, using placeholder
        let mut data_to_hash: *const u8 = (*field_data.add(i as usize)).data as *const u8;
        let mut datalen: u32 = (*field_data.add(i as usize)).len;
        let datalen_to_hash: u32 = if ima_canonical_fmt == 0 {
            datalen
        } else {
            cpu_to_le32(datalen)
        };

        if strcmp((*td).name, IMA_TEMPLATE_IMA_NAME) != 0 {
            rc = crypto_shash_update(
                shash,
                &datalen_to_hash as *const u32 as *const u8,
                core::mem::size_of::<u32>() as c_uint,
            ) as i64;
            if rc != 0 {
                break;
            }
        } else if strcmp((*(*td).fields.add(i as usize)).field_id, b"n\0".as_ptr() as *const c_char) == 0 {
            memcpy(
                buffer.as_mut_ptr() as *mut core::ffi::c_void,
                data_to_hash as *const core::ffi::c_void,
                datalen as c_ulong,
            );
            data_to_hash = buffer.as_ptr();
            datalen = 256; // IMA_EVENT_NAME_LEN_MAX + 1
        }
        rc = crypto_shash_update(shash, data_to_hash, datalen as c_uint) as i64;
        if rc != 0 {
            break;
        }
        i += 1;
    }

    if rc == 0 {
        rc = crypto_shash_final(shash, (*(*entry).digests.add(tfm_idx as usize)).digest as *mut u8) as i64;
    }

    rc
}

pub unsafe fn ima_calc_field_array_hash(
    field_data: *mut ima_field_data,
    entry: *mut ima_template_entry,
) -> i64 {
    let mut alg_id: u16;
    let mut rc: i64;
    let mut i: i32;

    rc = ima_calc_field_array_hash_tfm(field_data, entry, ima_sha1_idx);
    if rc != 0 {
        return rc;
    }

    (*(*entry).digests.add(ima_sha1_idx as usize)).alg_id = TPM_ALG_SHA1;

    let banks = nr_banks(ima_tpm_chip);
    i = 0;
    while i < banks + ima_extra_slots {
        if i == ima_sha1_idx {
            i += 1;
            continue;
        }

        if i < banks {
            alg_id = (*ima_tpm_chip).allocated_banks[i as usize].alg_id;
            (*(*entry).digests.add(i as usize)).alg_id = alg_id;
        }

        // for unmapped TPM algorithms digest is still a padded SHA1
        if (*ima_algo_array.add(i as usize)).tfm.is_null() {
            memcpy(
                (*(*entry).digests.add(i as usize)).digest as *mut core::ffi::c_void,
                (*(*entry).digests.add(ima_sha1_idx as usize)).digest as *const core::ffi::c_void,
                TPM_DIGEST_SIZE as c_ulong,
            );
            i += 1;
            continue;
        }

        rc = ima_calc_field_array_hash_tfm(field_data, entry, i);
        if rc != 0 {
            return rc;
        }
        i += 1;
    }
    rc
}

unsafe fn calc_buffer_shash_tfm(
    mut buf: *const core::ffi::c_void,
    mut size: loff_t,
    hash: *mut ima_digest_data,
    tfm: *mut crypto_shash,
) -> i64 {
    let mut shash: *mut shash_desc;
    let mut len: c_uint;
    let mut rc: i64;

    shash = core::alloc::alloc(
        core::alloc::Layout::new::<shash_desc>()
    ) as *mut shash_desc;

    (*shash).tfm = tfm;

    (*hash).length = crypto_shash_digestsize(tfm);

    rc = crypto_shash_init(shash) as i64;
    if rc != 0 {
        return rc;
    }

    while size > 0 {
        len = if size < PAGE_SIZE as loff_t { size as c_uint } else { PAGE_SIZE as c_uint };
        rc = crypto_shash_update(shash, buf as *const u8, len) as i64;
        if rc != 0 {
            break;
        }
        buf = (buf as *const u8).add(len as usize) as *const core::ffi::c_void;
        size -= len as loff_t;
    }

    if rc == 0 {
        rc = crypto_shash_final(shash, (*hash).digest as *mut u8) as i64;
    }
    rc
}

pub unsafe fn ima_calc_buffer_hash(
    buf: *const core::ffi::c_void,
    len: loff_t,
    hash: *mut ima_digest_data,
) -> i64 {
    let mut tfm: *mut crypto_shash;
    let mut rc: i64;

    tfm = ima_alloc_tfm((*hash).algo);
    if IS_ERR(tfm as *const core::ffi::c_void) != 0 {
        return PTR_ERR(tfm as *const core::ffi::c_void);
    }

    rc = calc_buffer_shash_tfm(buf, len, hash, tfm);

    ima_free_tfm(tfm);
    rc
}

unsafe fn ima_pcrread(idx: u32, d: *mut tpm_digest) {
    if ima_tpm_chip.is_null() {
        return;
    }

    if tpm_pcr_read(ima_tpm_chip, idx, d) != 0 {
        pr_err(b"Error Communicating to TPM chip\n\0".as_ptr() as *const c_char);
    }
}

// The boot_aggregate is a cumulative hash over TPM registers 0 - 7.  With
// TPM 1.2 the boot_aggregate was based on reading the SHA1 PCRs, but with
// TPM 2.0 hash agility, TPM chips could support multiple TPM PCR banks,
// allowing firmware to configure and enable different banks.
//
// Knowing which TPM bank is read to calculate the boot_aggregate digest
// needs to be conveyed to a verifier.  For this reason, use the same
// hash algorithm for reading the TPM PCRs as for calculating the boot
// aggregate digest as stored in the measurement list.
unsafe fn ima_calc_boot_aggregate_tfm(
    digest: *mut c_char,
    alg_id: u16,
    tfm: *mut crypto_shash,
) -> i64 {
    let mut d: tpm_digest = core::mem::zeroed();
    d.alg_id = alg_id;
    let mut rc: i64;
    let mut i: u32;
    let mut shash: *mut shash_desc;

    shash = core::alloc::alloc(
        core::alloc::Layout::new::<shash_desc>()
    ) as *mut shash_desc;

    (*shash).tfm = tfm;

    pr_devel(
        b"calculating the boot-aggregate based on TPM bank: %04x\n\0".as_ptr() as *const c_char,
        d.alg_id,
    );

    rc = crypto_shash_init(shash) as i64;
    if rc != 0 {
        return rc;
    }

    // cumulative digest over TPM registers 0-7
    i = TPM_PCR0;
    while i < TPM_PCR8 {
        ima_pcrread(i, &mut d);
        // now accumulate with current aggregate
        rc = crypto_shash_update(
            shash,
            d.digest.as_ptr(),
            crypto_shash_digestsize(tfm),
        ) as i64;
        if rc != 0 {
            return rc;
        }
        i += 1;
    }

    // Extend cumulative digest over TPM registers 8-9, which contain
    // measurement for the kernel command line (reg. 8) and image (reg. 9)
    // in a typical PCR allocation. Registers 8-9 are only included in
    // non-SHA1 boot_aggregate digests to avoid ambiguity.
    if alg_id != TPM_ALG_SHA1 {
        i = TPM_PCR8;
        while i < TPM_PCR10 {
            ima_pcrread(i, &mut d);
            rc = crypto_shash_update(
                shash,
                d.digest.as_ptr(),
                crypto_shash_digestsize(tfm),
            ) as i64;
            if rc != 0 {
                return rc;
            }
            i += 1;
        }
    }

    if rc == 0 {
        rc = crypto_shash_final(shash, digest as *mut u8) as i64;
    }
    rc
}

pub unsafe fn ima_calc_boot_aggregate(hash: *mut ima_digest_data) -> i64 {
    let mut tfm: *mut crypto_shash;
    let mut crypto_id: u16;
    let mut alg_id: u16;
    let mut rc: i64;
    let mut i: i32;
    let mut bank_idx: i32 = -1;

    i = 0;
    while i < (*ima_tpm_chip).nr_allocated_banks as i32 {
        crypto_id = (*ima_tpm_chip).allocated_banks[i as usize].crypto_id as u16;
        if crypto_id as i32 == (*hash).algo {
            bank_idx = i;
            break;
        }

        if crypto_id as i32 == HASH_ALGO_SHA256 {
            bank_idx = i;
        }

        if bank_idx == -1 && crypto_id as i32 == HASH_ALGO_SHA1 {
            bank_idx = i;
        }
        i += 1;
    }

    if bank_idx == -1 {
        pr_err(b"No suitable TPM algorithm for boot aggregate\n\0".as_ptr() as *const c_char);
        return 0;
    }

    (*hash).algo = (*ima_tpm_chip).allocated_banks[bank_idx as usize].crypto_id;

    tfm = ima_alloc_tfm((*hash).algo);
    if IS_ERR(tfm as *const core::ffi::c_void) != 0 {
        return PTR_ERR(tfm as *const core::ffi::c_void);
    }

    (*hash).length = crypto_shash_digestsize(tfm);
    alg_id = (*ima_tpm_chip).allocated_banks[bank_idx as usize].alg_id;
    rc = ima_calc_boot_aggregate_tfm((*hash).digest, alg_id, tfm);

    ima_free_tfm(tfm);

    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
