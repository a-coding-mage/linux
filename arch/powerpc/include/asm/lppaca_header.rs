/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * lppaca.h
 * Copyright (C) 2001  Mike Corrigan IBM Corporation
 */

/* These definitions relate to hypervisors that only exist when using a
 * server type processor.  The original declarations are kernel/Book3S
 * conditional; those conditions are retained here as source-level intent.
 */

/*
 * The lppaca is the "virtual processor area" registered with the hypervisor,
 * H_REGISTER_VPA etc.
 *
 * According to PAPR, the structure is 640 bytes long, must be L1 cache line
 * aligned, and must not cross a 4kB boundary. Its size field must be at
 * least 640 bytes (but may be more).
 *
 * Pre-v4.14 KVM hypervisors reject the VPA if its size field is smaller than
 * 1kB, so we dynamically allocate 1kB and advertise size as 1kB, but keep
 * this structure as the canonical 640 byte size.
 */
#[repr(C, align(128))]
pub struct lppaca {
    /* cacheline 1 contains read-only data */
    pub desc: __be32,                 /* Eye catcher 0xD397D781 */
    pub size: __be16,                 /* Size of this struct */
    pub reserved1: [u8; 3],
    pub __old_status: u8,             /* Old status, including shared proc */
    pub reserved3: [u8; 14],
    pub dyn_hw_node_id: __be32,       /* Dynamic hardware node id (volatile) */
    pub dyn_hw_proc_id: __be32,       /* Dynamic hardware proc id (volatile) */
    pub reserved4: [u8; 56],
    pub vphn_assoc_counts: [u8; 8],   /* Virtual processor home node */
    /* associativity change counters */
    pub reserved5: [u8; 32],

    /* cacheline 2 contains local read-write data */
    pub reserved6: [u8; 48],
    pub cede_latency_hint: u8,
    pub ebb_regs_in_use: u8,
    pub reserved7: [u8; 6],
    pub dtl_enable_mask: u8,          /* Dispatch Trace Log mask */
    pub donate_dedicated_cpu: u8,     /* Donate dedicated CPU cycles */
    pub fpregs_in_use: u8,
    pub pmcregs_in_use: u8,
    pub l2_counters_enable: u8,       /* Enable usage of counters for KVM guest */
    pub reserved8: [u8; 27],
    pub wait_state_cycles: __be64,    /* Wait cycles for this proc */
    pub reserved9: [u8; 28],
    pub slb_count: __be16,            /* # of SLBs to maintain */
    pub idle: u8,                     /* Indicate OS is idle */
    pub vmxregs_in_use: u8,

    /* cacheline 3 is shared with other processors */
    /* Volatile fields retain the C volatile-access requirement. */
    pub yield_count: __be32,
    pub dispersion_count: __be32,     /* dispatch changed physical cpu */
    pub cmo_faults: __be64,           /* CMO page fault count */
    pub cmo_fault_time: __be64,       /* CMO page fault time */
    pub reserved10: [u8; 64],         /* [S]PURR expropriated/donated */
    pub enqueue_dispatch_tb: __be64,  /* Total TB enqueue->dispatch */
    pub ready_enqueue_tb: __be64,     /* Total TB ready->enqueue */
    pub wait_ready_tb: __be64,        /* Total TB wait->ready */
    pub reserved11: [u8; 16],

    /* cacheline 4-5 */
    pub page_ins: __be32,             /* CMO Hint - # page ins by OS */
    pub reserved12: [u8; 28],
    pub l1_to_l2_cs_tb: __be64,
    pub l2_to_l1_cs_tb: __be64,
    pub l2_runtime_tb: __be64,
    pub reserved13: [u8; 96],
    pub dtl_idx: __be64,              /* Dispatch Trace Log head index */
    pub reserved14: [u8; 96],
}

#[macro_export]
macro_rules! lppaca_of {
    ($cpu:expr) => {
        (*paca_ptrs[$cpu].lppaca_ptr)
    };
}

/* We are using a non architected field to determine if a partition is
 * shared or dedicated. This currently works on both KVM and PHYP, but
 * we will have to transition to something better.
 */
pub const LPPACA_OLD_SHARED_PROC: u32 = 2;

/* CONFIG_PPC_PSERIES: all CPUs should have the same shared proc value, so
 * directly access the PACA to avoid false positives from DEBUG_PREEMPT.
 */
/* `lppaca_shared_proc` depends on external kernel symbols.  Its source-level
 * body is preserved here; the referenced PACA and firmware symbols are
 * supplied by the surrounding kernel translation.
 *
 * unsafe fn lppaca_shared_proc() -> bool {
 *     let l = local_paca.lppaca_ptr;
 *     if !firmware_has_feature(FW_FEATURE_SPLPAR) { return false; }
 *     return (l.__old_status & LPPACA_OLD_SHARED_PROC as u8) != 0;
 * }
 */
/* #define get_lppaca() (get_paca().lppaca_ptr) */

/*
 * SLB shadow buffer structure as defined in the PAPR.  The save_area
 * contains adjacent ESID and VSID pairs for each shadowed SLB.  The
 * ESID is stored in the lower 64bits, then the VSID.
 */
#[repr(C, align(128))]
pub struct slb_shadow {
    pub persistent: __be32,           /* Number of persistent SLBs */
    pub buffer_length: __be32,         /* Total shadow buffer length */
    pub reserved: __be64,
    pub save_area: [slb_shadow_save_area; SLB_NUM_BOLTED],
}

#[repr(C)]
pub struct slb_shadow_save_area {
    pub esid: __be64,
    pub vsid: __be64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
