// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright IBM Corp. 2008
 *
 * Authors: Hollis Blanchard <hollisb@us.ibm.com>
 *          Christian Ehrhardt <ehrhardt@linux.vnet.ibm.com>
 */

// C dependencies: linux/kvm_host.h, linux/fs.h, linux/seq_file.h,
// linux/debugfs.h, linux/uaccess.h, linux/module.h, asm/time.h,
// asm-generic/div64.h, and timing.h.

pub unsafe fn kvmppc_init_timing_stats(vcpu: *mut kvm_vcpu) {
    let mut i: i32;

    /* Take a lock to avoid concurrent updates */
    mutex_lock(&mut (*vcpu).arch.exit_timing_lock);

    (*vcpu).arch.last_exit_type = 0xDEAD;
    i = 0;
    while i < __NUMBER_OF_KVM_EXIT_TYPES {
        (*vcpu).arch.timing_count_type[i as usize] = 0;
        (*vcpu).arch.timing_max_duration[i as usize] = 0;
        (*vcpu).arch.timing_min_duration[i as usize] = 0xFFFFFFFF;
        (*vcpu).arch.timing_sum_duration[i as usize] = 0;
        (*vcpu).arch.timing_sum_quad_duration[i as usize] = 0;
        i += 1;
    }
    (*vcpu).arch.timing_last_exit = 0;
    (*vcpu).arch.timing_exit.tv64 = 0;
    (*vcpu).arch.timing_last_enter.tv64 = 0;

    mutex_unlock(&mut (*vcpu).arch.exit_timing_lock);
}

unsafe fn add_exit_timing(vcpu: *mut kvm_vcpu, duration: u64, type_: i32) {
    let mut old: u64;
    let index = type_ as usize;

    mutex_lock(&mut (*vcpu).arch.exit_timing_lock);

    (*vcpu).arch.timing_count_type[index] += 1;

    /* sum */
    old = (*vcpu).arch.timing_sum_duration[index];
    (*vcpu).arch.timing_sum_duration[index] =
        (*vcpu).arch.timing_sum_duration[index].wrapping_add(duration);
    if old > (*vcpu).arch.timing_sum_duration[index] {
        printk(KERN_ERR "%s - wrap adding sum of durations old %lld new %lld type %d exit # of type %d\n",
            __func__, old, (*vcpu).arch.timing_sum_duration[index], type_,
            (*vcpu).arch.timing_count_type[index]);
    }

    /* square sum */
    old = (*vcpu).arch.timing_sum_quad_duration[index];
    (*vcpu).arch.timing_sum_quad_duration[index] =
        (*vcpu).arch.timing_sum_quad_duration[index]
            .wrapping_add(duration.wrapping_mul(duration));
    if old > (*vcpu).arch.timing_sum_quad_duration[index] {
        printk(KERN_ERR "%s - wrap adding sum of squared durations old %lld new %lld type %d exit # of type %d\n",
            __func__, old, (*vcpu).arch.timing_sum_quad_duration[index], type_,
            (*vcpu).arch.timing_count_type[index]);
    }

    /* set min/max */
    if duration < (*vcpu).arch.timing_min_duration[index] {
        (*vcpu).arch.timing_min_duration[index] = duration;
    }
    if duration > (*vcpu).arch.timing_max_duration[index] {
        (*vcpu).arch.timing_max_duration[index] = duration;
    }

    mutex_unlock(&mut (*vcpu).arch.exit_timing_lock);
}

pub unsafe fn kvmppc_update_timing_stats(vcpu: *mut kvm_vcpu) {
    let exit = (*vcpu).arch.timing_last_exit;
    let enter = (*vcpu).arch.timing_last_enter.tv64;

    /* save exit time, used next exit when the reenter time is known */
    (*vcpu).arch.timing_last_exit = (*vcpu).arch.timing_exit.tv64;

    if (*vcpu).arch.last_exit_type == 0xDEAD || exit == 0 {
        return; /* skip incomplete cycle (e.g. after reset) */
    }

    /* update statistics for average and standard deviation */
    add_exit_timing(vcpu, enter.wrapping_sub(exit), (*vcpu).arch.last_exit_type);
    /* enter -> timing_last_exit is time spent in guest - log this too */
    add_exit_timing(vcpu, (*vcpu).arch.timing_last_exit.wrapping_sub(enter), TIMEINGUEST);
}

#[no_mangle]
pub static kvm_exit_names: [*const u8; __NUMBER_OF_KVM_EXIT_TYPES as usize] = [
    [MMIO_EXITS] = b"MMIO\0".as_ptr(),
    [SIGNAL_EXITS] = b"SIGNAL\0".as_ptr(),
    [ITLB_REAL_MISS_EXITS] = b"ITLBREAL\0".as_ptr(),
    [ITLB_VIRT_MISS_EXITS] = b"ITLBVIRT\0".as_ptr(),
    [DTLB_REAL_MISS_EXITS] = b"DTLBREAL\0".as_ptr(),
    [DTLB_VIRT_MISS_EXITS] = b"DTLBVIRT\0".as_ptr(),
    [SYSCALL_EXITS] = b"SYSCALL\0".as_ptr(),
    [ISI_EXITS] = b"ISI\0".as_ptr(), [DSI_EXITS] = b"DSI\0".as_ptr(),
    [EMULATED_INST_EXITS] = b"EMULINST\0".as_ptr(),
    [EMULATED_MTMSRWE_EXITS] = b"EMUL_WAIT\0".as_ptr(),
    [EMULATED_WRTEE_EXITS] = b"EMUL_WRTEE\0".as_ptr(),
    [EMULATED_MTSPR_EXITS] = b"EMUL_MTSPR\0".as_ptr(),
    [EMULATED_MFSPR_EXITS] = b"EMUL_MFSPR\0".as_ptr(),
    [EMULATED_MTMSR_EXITS] = b"EMUL_MTMSR\0".as_ptr(),
    [EMULATED_MFMSR_EXITS] = b"EMUL_MFMSR\0".as_ptr(),
    [EMULATED_TLBSX_EXITS] = b"EMUL_TLBSX\0".as_ptr(),
    [EMULATED_TLBWE_EXITS] = b"EMUL_TLBWE\0".as_ptr(),
    [EMULATED_RFI_EXITS] = b"EMUL_RFI\0".as_ptr(), [DEC_EXITS] = b"DEC\0".as_ptr(),
    [EXT_INTR_EXITS] = b"EXTINT\0".as_ptr(), [HALT_WAKEUP] = b"HALT\0".as_ptr(),
    [USR_PR_INST] = b"USR_PR_INST\0".as_ptr(), [FP_UNAVAIL] = b"FP_UNAVAIL\0".as_ptr(),
    [DEBUG_EXITS] = b"DEBUG\0".as_ptr(), [TIMEINGUEST] = b"TIMEINGUEST\0".as_ptr(),
];

unsafe fn kvmppc_exit_timing_show(m: *mut seq_file, _private: *mut core::ffi::c_void) -> i32 {
    let vcpu = (*m).private as *mut kvm_vcpu;
    let mut i = 0;
    seq_puts(m, b"type\tcount\tmin\tmax\tsum\tsum_squared\n\0".as_ptr());
    while i < __NUMBER_OF_KVM_EXIT_TYPES {
        let index = i as usize;
        let mut min = (*vcpu).arch.timing_min_duration[index];
        let mut max = (*vcpu).arch.timing_max_duration[index];
        let mut sum = (*vcpu).arch.timing_sum_duration[index];
        let mut sum_quad = (*vcpu).arch.timing_sum_quad_duration[index];
        min /= tb_ticks_per_usec; max /= tb_ticks_per_usec;
        sum /= tb_ticks_per_usec; sum_quad /= tb_ticks_per_usec;
        seq_printf(m, b"%12s\t%10d\t%10lld\t%10lld\t%20lld\t%20lld\n\0".as_ptr(),
            kvm_exit_names[index], (*vcpu).arch.timing_count_type[index], min, max, sum, sum_quad);
        i += 1;
    }
    0
}

unsafe fn kvmppc_exit_timing_open(inode: *mut inode, file: *mut file) -> i32 {
    single_open(file, kvmppc_exit_timing_show, (*inode).i_private)
}

static kvmppc_exit_timing_fops: file_operations = file_operations {
    owner: THIS_MODULE, open: Some(kvmppc_exit_timing_open), read: Some(seq_read),
    write: Some(kvmppc_exit_timing_write), llseek: Some(seq_lseek), release: Some(single_release),
};

/* Write 'c' to clear the timing statistics. */
pub unsafe fn kvmppc_exit_timing_write(
    file: *mut file, user_buf: *const u8, count: usize, _ppos: *mut loff_t,
) -> isize {
    let mut err: isize = -EINVAL as isize;
    let mut c: u8 = 0;

    if count > 1 { return err; }
    if get_user(&mut c, user_buf) != 0 { return -EFAULT as isize; }

    if c == b'c' {
        let seqf = (*file).private_data as *mut seq_file;
        let vcpu = (*seqf).private as *mut kvm_vcpu;
        /* Write does not affect our buffers previously generated with
         * show. seq_file is locked here to prevent races of init with
         * a show call */
        mutex_lock(&mut (*seqf).lock);
        kvmppc_init_timing_stats(vcpu);
        mutex_unlock(&mut (*seqf).lock);
        err = count as isize;
    }
    err
}

pub unsafe fn kvmppc_create_vcpu_debugfs_e500(
    vcpu: *mut kvm_vcpu, debugfs_dentry: *mut dentry,
) -> i32 {
    debugfs_create_file(b"timing\0".as_ptr(), 0o666, debugfs_dentry,
        vcpu as *mut core::ffi::c_void, &kvmppc_exit_timing_fops);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
