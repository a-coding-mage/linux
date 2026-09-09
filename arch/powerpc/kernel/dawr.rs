// SPDX-License-Identifier: GPL-2.0+
/*
 * DAWR infrastructure
 *
 * Copyright 2019, Michael Neuling, IBM Corporation.
 */

// Dependencies supplied by the surrounding kernel translation.

pub static mut dawr_force_enable: bool = false;

pub unsafe fn set_dawr(nr: i32, brk: *mut arch_hw_breakpoint) -> i32 {
    let dawr: u64;
    let mut dawrx: u64;
    let mrd: u64;

    dawr = (*brk).address;

    dawrx = (((*brk).type_ & (HW_BRK_TYPE_READ | HW_BRK_TYPE_WRITE)) as u64)
        << (63 - 58);
    dawrx |= ((((*brk).type_ & HW_BRK_TYPE_TRANSLATE) >> 2) as u64) << (63 - 59);
    dawrx |= ((*brk).type_ & HW_BRK_TYPE_PRIV_ALL) as u64 >> 3;
    /*
     * DAWR length is stored in field MDR bits 48:53.  Matches range in
     * doublewords (64 bits) biased by -1 eg. 0b000000=1DW and
     * 0b111111=64DW.
     * brk->hw_len is in bytes.
     * This aligns up to double word size, shifts and does the bias.
     */
    mrd = (((*brk).hw_len + 7) >> 3) - 1;
    dawrx |= (mrd & 0x3f) << (63 - 53);

    if ppc_md.set_dawr.is_some() {
        return ppc_md.set_dawr.unwrap()(nr, dawr, dawrx);
    }

    if nr == 0 {
        mtspr(SPRN_DAWR0, dawr);
        mtspr(SPRN_DAWRX0, dawrx);
    } else {
        mtspr(SPRN_DAWR1, dawr);
        mtspr(SPRN_DAWRX1, dawrx);
    }

    0
}

unsafe fn disable_dawrs_cb(_info: *mut core::ffi::c_void) {
    let mut null_brk: arch_hw_breakpoint = core::mem::zeroed();
    let mut i = 0;

    while i < nr_wp_slots() {
        set_dawr(i, &mut null_brk);
        i += 1;
    }
}

unsafe fn dawr_write_file_bool(
    file: *mut file,
    user_buf: *const core::ffi::c_char,
    count: usize,
    ppos: *mut loff_t,
) -> isize {
    let mut null_brk: arch_hw_breakpoint = core::mem::zeroed();
    let rc: isize;

    /* Send error to user if they hypervisor won't allow us to write DAWR */
    if !dawr_force_enable
        && firmware_has_feature(FW_FEATURE_LPAR)
        && set_dawr(0, &mut null_brk) != H_SUCCESS
    {
        return -ENODEV;
    }

    rc = debugfs_write_file_bool(file, user_buf, count, ppos);
    if rc != 0 {
        return rc;
    }

    /* If we are clearing, make sure all CPUs have the DAWR cleared */
    if !dawr_force_enable {
        smp_call_function(disable_dawrs_cb, core::ptr::null_mut(), 0);
    }

    rc
}

static dawr_enable_fops: file_operations = file_operations {
    read: Some(debugfs_read_file_bool),
    write: Some(dawr_write_file_bool),
    open: Some(simple_open),
    llseek: Some(default_llseek),
};

unsafe fn dawr_force_setup() -> i32 {
    if cpu_has_feature(CPU_FTR_DAWR) {
        /* Don't setup sysfs file for user control on P8 */
        dawr_force_enable = true;
        return 0;
    }

    if PVR_VER(mfspr(SPRN_PVR)) == PVR_POWER9 {
        /* Turn DAWR off by default, but allow admin to turn it on */
        debugfs_create_file_unsafe(
            b"dawr_enable_dangerous\\0".as_ptr() as *const core::ffi::c_char,
            0o600,
            arch_debugfs_dir,
            &mut dawr_force_enable as *mut _ as *mut core::ffi::c_void,
            &dawr_enable_fops,
        );
    }
    0
}

// arch_initcall(dawr_force_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
