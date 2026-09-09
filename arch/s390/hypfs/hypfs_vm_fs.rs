// SPDX-License-Identifier: GPL-2.0
/*
 *    Hypervisor filesystem for Linux on s390. z/VM implementation.
 *
 *    Copyright IBM Corp. 2006
 *    Author(s): Michael Holzheu <holzheu@de.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/types.h, linux/errno.h, linux/string.h, linux/vmalloc.h,
// asm/extable.h, asm/diag.h, asm/ebcdic.h, asm/timex.h, hypfs_vm.h, hypfs.h

macro_rules! attribute {
    ($dir:expr, $name:expr, $member:expr) => {{
        let rc = hypfs_create_u64($dir, $name, $member);
        if rc != 0 {
            return rc;
        }
    }};
}

macro_rules! goto_failed {
    ($data:expr, $rc:expr) => {{
        diag2fc_free($data);
        return $rc;
    }};
}

unsafe fn hypfs_vm_create_guest(
    systems_dir: *mut dentry,
    data: *mut diag2fc_data,
) -> i32 {
    let mut guest_name = [0u8; DIAG2FC_NAME_LEN + 1];
    let mut guest_dir: *mut dentry;
    let mut cpus_dir: *mut dentry;
    let mut samples_dir: *mut dentry;
    let mut mem_dir: *mut dentry;
    let dedicated_flag: i32;
    let capped_value: i32;

    capped_value = ((*data).flags & 0x00000006) >> 1;
    dedicated_flag = ((*data).flags & 0x00000008) >> 3;

    /* guest dir */
    core::ptr::copy_nonoverlapping(
        (*data).guest_name.as_ptr(),
        guest_name.as_mut_ptr(),
        DIAG2FC_NAME_LEN,
    );
    EBCASC(guest_name.as_mut_ptr(), DIAG2FC_NAME_LEN);
    strim(guest_name.as_mut_ptr());
    guest_dir = hypfs_mkdir(systems_dir, guest_name.as_ptr());
    if IS_ERR(guest_dir) {
        return PTR_ERR(guest_dir);
    }
    attribute!(guest_dir, "onlinetime_us", (*data).el_time);

    /* logical cpu information */
    cpus_dir = hypfs_mkdir(guest_dir, b"cpus\0".as_ptr());
    if IS_ERR(cpus_dir) {
        return PTR_ERR(cpus_dir);
    }
    attribute!(cpus_dir, "cputime_us", (*data).used_cpu);
    attribute!(cpus_dir, "capped", capped_value);
    attribute!(cpus_dir, "dedicated", dedicated_flag);
    attribute!(cpus_dir, "count", (*data).vcpus);
    /*
     * Note: The "weight_min" attribute got the wrong name.
     * The value represents the number of non-stopped (operating)
     * CPUS.
     */
    attribute!(cpus_dir, "weight_min", (*data).ocpus);
    attribute!(cpus_dir, "weight_max", (*data).cpu_max);
    attribute!(cpus_dir, "weight_cur", (*data).cpu_shares);

    /* memory information */
    mem_dir = hypfs_mkdir(guest_dir, b"mem\0".as_ptr());
    if IS_ERR(mem_dir) {
        return PTR_ERR(mem_dir);
    }
    attribute!(mem_dir, "min_KiB", (*data).mem_min_kb);
    attribute!(mem_dir, "max_KiB", (*data).mem_max_kb);
    attribute!(mem_dir, "used_KiB", (*data).mem_used_kb);
    attribute!(mem_dir, "share_KiB", (*data).mem_share_kb);

    /* samples */
    samples_dir = hypfs_mkdir(guest_dir, b"samples\0".as_ptr());
    if IS_ERR(samples_dir) {
        return PTR_ERR(samples_dir);
    }
    attribute!(samples_dir, "cpu_using", (*data).cpu_use_samp);
    attribute!(samples_dir, "cpu_delay", (*data).cpu_delay_samp);
    attribute!(samples_dir, "mem_delay", (*data).page_wait_samp);
    attribute!(samples_dir, "idle", (*data).idle_samp);
    attribute!(samples_dir, "other", (*data).other_samp);
    attribute!(samples_dir, "total", (*data).total_samp);
    0
}

pub unsafe fn hypfs_vm_create_files(root: *mut dentry) -> i32 {
    let mut dir: *mut dentry;
    let data: *mut diag2fc_data;
    let mut count: u32 = 0;
    let rc: i32;
    let mut i: i32;

    data = diag2fc_store(diag2fc_guest_query, &mut count, 0);
    if IS_ERR(data) {
        return PTR_ERR(data);
    }

    /* Hypervisor Info */
    dir = hypfs_mkdir(root, b"hyp\0".as_ptr());
    if IS_ERR(dir) {
        rc = PTR_ERR(dir);
        goto_failed!(data, rc);
    }
    rc = hypfs_create_str(dir, "type", "z/VM Hypervisor");
    if rc != 0 {
        goto_failed!(data, rc);
    }

    /* physical cpus */
    dir = hypfs_mkdir(root, b"cpus\0".as_ptr());
    if IS_ERR(dir) {
        rc = PTR_ERR(dir);
        goto_failed!(data, rc);
    }
    rc = hypfs_create_u64(dir, "count", (*data).lcpus);
    if rc != 0 {
        goto_failed!(data, rc);
    }

    /* guests */
    dir = hypfs_mkdir(root, b"systems\0".as_ptr());
    if IS_ERR(dir) {
        rc = PTR_ERR(dir);
        goto_failed!(data, rc);
    }

    i = 0;
    while i < count as i32 {
        rc = hypfs_vm_create_guest(dir, data.add(i as usize));
        if rc != 0 {
            goto_failed!(data, rc);
        }
        i += 1;
    }
    diag2fc_free(data);
    return 0;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
