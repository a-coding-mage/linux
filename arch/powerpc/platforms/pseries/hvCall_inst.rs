// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2006 Mike Kravetz IBM Corporation
 *
 * Hypervisor Call Instrumentation
 */

// Kernel and architecture dependencies supplied by other translation units.

/* For hcall instrumentation. One structure per-hcall, per-CPU */
#[repr(C)]
pub struct hcall_stats {
    pub num_calls: ::core::primitive::usize, /* number of calls (on this CPU) */
    pub tb_total: ::core::primitive::usize,  /* total wall time (mftb) of calls. */
    pub purr_total: ::core::primitive::usize, /* total cpu time (PURR) of calls. */
    pub tb_start: ::core::primitive::usize,
    pub purr_start: ::core::primitive::usize,
}

pub const HCALL_STAT_ARRAY_SIZE: usize = ((MAX_HCALL_OPCODE >> 2) + 1) as usize;

// DEFINE_PER_CPU(struct hcall_stats[HCALL_STAT_ARRAY_SIZE], hcall_stats);
static mut hcall_stats_per_cpu: [hcall_stats; HCALL_STAT_ARRAY_SIZE] =
    [hcall_stats {
        num_calls: 0,
        tb_total: 0,
        purr_total: 0,
        tb_start: 0,
        purr_start: 0,
    }; HCALL_STAT_ARRAY_SIZE];

/*
 * Routines for displaying the statistics in debugfs
 */
unsafe fn hc_start(m: *mut seq_file, pos: *mut loff_t) -> *mut ::core::ffi::c_void {
    if (*pos as i32) < (HCALL_STAT_ARRAY_SIZE as i32 - 1) {
        return ((*pos + 1) as usize) as *mut ::core::ffi::c_void;
    }

    core::ptr::null_mut()
}

unsafe fn hc_next(
    m: *mut seq_file,
    p: *mut ::core::ffi::c_void,
    pos: *mut loff_t,
) -> *mut ::core::ffi::c_void {
    *pos += 1;

    hc_start(m, pos)
}

unsafe fn hc_stop(m: *mut seq_file, p: *mut ::core::ffi::c_void) {
}

unsafe fn hc_show(m: *mut seq_file, p: *mut ::core::ffi::c_void) -> i32 {
    let h_num = p as usize;
    let hs = (*m).private as *mut hcall_stats;

    if (*hs.add(h_num)).num_calls != 0 {
        if cpu_has_feature(CPU_FTR_PURR) {
            seq_printf(
                m,
                "%lu %lu %lu %lu\n",
                (h_num << 2),
                (*hs.add(h_num)).num_calls,
                (*hs.add(h_num)).tb_total,
                (*hs.add(h_num)).purr_total,
            );
        } else {
            seq_printf(
                m,
                "%lu %lu %lu\n",
                (h_num << 2),
                (*hs.add(h_num)).num_calls,
                (*hs.add(h_num)).tb_total,
            );
        }
    }

    0
}

// static const struct seq_operations hcall_inst_sops = { start, next, stop, show };
// DEFINE_SEQ_ATTRIBUTE(hcall_inst);

pub const HCALL_ROOT_DIR: *const u8 = b"hcall_inst\0".as_ptr();
pub const CPU_NAME_BUF_SIZE: usize = 32;

unsafe fn probe_hcall_entry(
    ignored: *mut ::core::ffi::c_void,
    opcode: usize,
    args: *mut usize,
) {
    if opcode > MAX_HCALL_OPCODE as usize {
        return;
    }

    let h = &mut hcall_stats_per_cpu[opcode / 4];
    h.tb_start = mftb();
    h.purr_start = mfspr(SPRN_PURR);
}

unsafe fn probe_hcall_exit(
    ignored: *mut ::core::ffi::c_void,
    opcode: usize,
    retval: isize,
    retbuf: *mut usize,
) {
    if opcode > MAX_HCALL_OPCODE as usize {
        return;
    }

    let h = &mut hcall_stats_per_cpu[opcode / 4];
    h.num_calls += 1;
    h.tb_total += mftb() - h.tb_start;
    h.purr_total += mfspr(SPRN_PURR) - h.purr_start;
}

unsafe fn hcall_inst_init() -> i32 {
    let mut hcall_root: *mut dentry;
    let mut cpu_name_buf = [0u8; CPU_NAME_BUF_SIZE];
    let mut cpu: i32;

    if !firmware_has_feature(FW_FEATURE_LPAR) {
        return 0;
    }

    if register_trace_hcall_entry(Some(probe_hcall_entry), core::ptr::null_mut()) != 0 {
        return -EINVAL;
    }

    if register_trace_hcall_exit(Some(probe_hcall_exit), core::ptr::null_mut()) != 0 {
        unregister_trace_hcall_entry(Some(probe_hcall_entry), core::ptr::null_mut());
        return -EINVAL;
    }

    hcall_root = debugfs_create_dir(HCALL_ROOT_DIR, core::ptr::null_mut());

    for_each_possible_cpu!(cpu, {
        snprintf(cpu_name_buf.as_mut_ptr(), CPU_NAME_BUF_SIZE, b"cpu%d\0".as_ptr(), cpu);
        debugfs_create_file(
            cpu_name_buf.as_ptr(),
            0o444,
            hcall_root,
            per_cpu(hcall_stats_per_cpu, cpu),
            &hcall_inst_fops,
        );
    });

    0
}

// machine_device_initcall(pseries, hcall_inst_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
