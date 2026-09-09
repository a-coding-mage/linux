// SPDX-License-Identifier: GPL-2.0
/*
 * Data gathering module for Linux-VM Monitor Stream, Stage 1.
 * Collects misc. OS related data (CPU utilization, running processes).
 *
 * Copyright IBM Corp. 2003, 2006
 *
 * Author: Gerald Schaefer <gerald.schaefer@de.ibm.com>
 */

// #define pr_fmt(fmt) "appldata: " fmt
// Kernel and architecture dependencies are supplied by the surrounding build.

#[repr(C, packed)]
pub struct appldata_os_per_cpu {
    pub per_cpu_user: u32,   // timer ticks spent in user mode
    pub per_cpu_nice: u32,   // ... spent with modified priority
    pub per_cpu_system: u32, // ... spent in kernel mode
    pub per_cpu_idle: u32,   // ... spent in idle mode

    // New in 2.6
    pub per_cpu_irq: u32,     // ... spent in interrupts
    pub per_cpu_softirq: u32, // ... spent in softirqs
    pub per_cpu_iowait: u32,  // ... spent while waiting for I/O

    // New in modification level 01
    pub per_cpu_steal: u32, // ... stolen by hypervisor
    pub cpu_id: u32,        // number of this CPU
}

#[repr(C, packed)]
pub struct appldata_os_data {
    pub timestamp: u64,
    pub sync_count_1: u32, // after VM collected the record data,
    pub sync_count_2: u32, // sync_count_1 and sync_count_2 should be the
                           // same. If not, the record has been updated on
                           // the Linux side while VM was collecting the
                           // (possibly corrupt) data

    pub nr_cpus: u32,     // number of (virtual) CPUs
    pub per_cpu_size: u32, // size of the per-cpu data struct
    pub cpu_offset: u32,  // offset of the first per-cpu data struct

    pub nr_running: u32, // number of runnable threads
    pub nr_threads: u32, // number of threads
    pub avenrun: [u32; 3], // average nr. of running processes during
                           // the last 1, 5 and 15 minutes

    // New in 2.6
    pub nr_iowait: u32, // number of blocked threads (waiting for I/O)

    // Flexible array member: per-cpu data.
    pub os_cpu: [appldata_os_per_cpu; 0],
}

static mut appldata_os_data: *mut appldata_os_data = core::ptr::null_mut();

static mut ops: appldata_ops = appldata_ops {
    name: "os",
    record_nr: APPLDATA_RECORD_OS_ID,
    owner: THIS_MODULE,
    mod_lvl: [0xF0, 0xF1], // EBCDIC "01"
    ..unsafe { core::mem::zeroed() }
};

/*
 * appldata_get_os_data()
 *
 * gather OS data
 */
unsafe fn appldata_get_os_data(data: *mut core::ffi::c_void) {
    let os_data = data as *mut appldata_os_data;
    (*os_data).sync_count_1 = (*os_data).sync_count_1.wrapping_add(1);

    (*os_data).nr_threads = nr_threads;
    (*os_data).nr_running = nr_running();
    (*os_data).nr_iowait = nr_iowait();
    (*os_data).avenrun[0] = avenrun[0].wrapping_add(FIXED_1 / 200);
    (*os_data).avenrun[1] = avenrun[1].wrapping_add(FIXED_1 / 200);
    (*os_data).avenrun[2] = avenrun[2].wrapping_add(FIXED_1 / 200);

    let mut j: usize = 0;
    // for_each_online_cpu(i)
    for i in online_cpu_iter() {
        let cpu = (*os_data).os_cpu.as_mut_ptr().add(j);
        (*cpu).per_cpu_user = nsecs_to_jiffies(kcpustat_cpu(i).cpustat[CPUTIME_USER]);
        (*cpu).per_cpu_nice = nsecs_to_jiffies(kcpustat_cpu(i).cpustat[CPUTIME_NICE]);
        (*cpu).per_cpu_system = nsecs_to_jiffies(kcpustat_cpu(i).cpustat[CPUTIME_SYSTEM]);
        (*cpu).per_cpu_idle = nsecs_to_jiffies(kcpustat_cpu(i).cpustat[CPUTIME_IDLE]);
        (*cpu).per_cpu_irq = nsecs_to_jiffies(kcpustat_cpu(i).cpustat[CPUTIME_IRQ]);
        (*cpu).per_cpu_softirq = nsecs_to_jiffies(kcpustat_cpu(i).cpustat[CPUTIME_SOFTIRQ]);
        (*cpu).per_cpu_iowait = nsecs_to_jiffies(kcpustat_cpu(i).cpustat[CPUTIME_IOWAIT]);
        (*cpu).per_cpu_steal = nsecs_to_jiffies(kcpustat_cpu(i).cpustat[CPUTIME_STEAL]);
        (*cpu).cpu_id = i;
        j += 1;
    }

    (*os_data).nr_cpus = j as u32;

    let new_size = struct_size(os_data, os_cpu, (*os_data).nr_cpus);
    if ops.size != new_size {
        if ops.active {
            let mut rc = appldata_diag(
                APPLDATA_RECORD_OS_ID,
                APPLDATA_START_INTERVAL_REC,
                ops.data as usize,
                new_size,
                ops.mod_lvl,
            );
            if rc != 0 {
                pr_err!("Starting a new OS data collection failed with rc=%d\n", rc);
            }
            rc = appldata_diag(
                APPLDATA_RECORD_OS_ID,
                APPLDATA_STOP_REC,
                ops.data as usize,
                ops.size,
                ops.mod_lvl,
            );
            if rc != 0 {
                pr_err!("Stopping a faulty OS data collection failed with rc=%d\n", rc);
            }
        }
        ops.size = new_size;
    }
    (*os_data).timestamp = get_tod_clock();
    (*os_data).sync_count_2 = (*os_data).sync_count_2.wrapping_add(1);
}

/*
 * appldata_os_init()
 *
 * init data, register ops
 */
unsafe fn appldata_os_init() -> i32 {
    let max_size = struct_size(appldata_os_data, os_cpu, num_possible_cpus());
    if max_size > APPLDATA_MAX_REC_SIZE {
        pr_err!("Maximum OS record size %i exceeds the maximum record size %i\n", max_size, APPLDATA_MAX_REC_SIZE);
        return -ENOMEM;
    }

    appldata_os_data = kzalloc(max_size, GFP_KERNEL | GFP_DMA);
    if appldata_os_data.is_null() {
        return -ENOMEM;
    }

    (*appldata_os_data).per_cpu_size = core::mem::size_of::<appldata_os_per_cpu>() as u32;
    (*appldata_os_data).cpu_offset = core::mem::offset_of!(appldata_os_data, os_cpu) as u32;

    ops.data = appldata_os_data as *mut core::ffi::c_void;
    ops.callback = Some(appldata_get_os_data);
    let rc = appldata_register_ops(&mut ops);
    if rc != 0 {
        kfree(appldata_os_data as *mut core::ffi::c_void);
    }
    rc
}

/*
 * appldata_os_exit()
 *
 * unregister ops
 */
unsafe fn appldata_os_exit() {
    appldata_unregister_ops(&mut ops);
    kfree(appldata_os_data as *mut core::ffi::c_void);
}

// module_init(appldata_os_init);
// module_exit(appldata_os_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Gerald Schaefer");
// MODULE_DESCRIPTION("Linux-VM Monitor Stream, OS statistics");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
