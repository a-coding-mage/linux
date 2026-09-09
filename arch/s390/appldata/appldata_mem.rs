// SPDX-License-Identifier: GPL-2.0
/*
 * Data gathering module for Linux-VM Monitor Stream, Stage 1.
 * Collects data related to memory management.
 *
 * Copyright IBM Corp. 2003, 2006
 *
 * Author: Gerald Schaefer <gerald.schaefer@de.ibm.com>
 */

// Linux kernel dependencies supplied by the surrounding repository.

const fn p2k(x: u64) -> u64 {
    x << (PAGE_SHIFT - 10)
}

/*
 * Memory data
 *
 * This is accessed as binary data by z/VM. If changes to it can't be avoided,
 * the structure version (product ID, see appldata_base.c) needs to be changed
 * as well and all documentation and z/VM applications using it must be
 * updated.
 */
#[repr(C, packed)]
struct AppldataMemData {
    timestamp: u64,
    sync_count_1: u32,
    sync_count_2: u32,

    pgpgin: u64,
    pgpgout: u64,
    pswpin: u64,
    pswpout: u64,

    sharedram: u64,

    totalram: u64,
    freeram: u64,
    totalhigh: u64,
    freehigh: u64,

    bufferram: u64,
    cached: u64,
    totalswap: u64,
    freeswap: u64,

    // New in 2.6 -->
    pgalloc: u64,
    pgfault: u64,
    pgmajfault: u64,
    // <-- New in 2.6
}

/*
 * appldata_get_mem_data()
 *
 * gather memory data
 */
unsafe fn appldata_get_mem_data(data: *mut core::ffi::c_void) {
    /*
     * don't put large structures on the stack, we are
     * serialized through the appldata_ops_mutex and can use static
     */
    static mut VAL: Sysinfo = Sysinfo::zeroed();
    let mut ev: [c_ulong; NR_VM_EVENT_ITEMS] = [0; NR_VM_EVENT_ITEMS];
    let mem_data = data as *mut AppldataMemData;

    (*mem_data).sync_count_1 += 1;

    all_vm_events(ev.as_mut_ptr());
    (*mem_data).pgpgin = ev[PGPGIN] >> 1;
    (*mem_data).pgpgout = ev[PGPGOUT] >> 1;
    (*mem_data).pswpin = ev[PSWPIN];
    (*mem_data).pswpout = ev[PSWPOUT];
    (*mem_data).pgalloc = ev[PGALLOC_NORMAL];
    (*mem_data).pgalloc += ev[PGALLOC_DMA];
    (*mem_data).pgfault = ev[PGFAULT];
    (*mem_data).pgmajfault = ev[PGMAJFAULT];

    si_meminfo(&raw mut VAL);
    (*mem_data).sharedram = VAL.sharedram;
    (*mem_data).totalram = p2k(VAL.totalram);
    (*mem_data).freeram = p2k(VAL.freeram);
    (*mem_data).totalhigh = p2k(VAL.totalhigh);
    (*mem_data).freehigh = p2k(VAL.freehigh);
    (*mem_data).bufferram = p2k(VAL.bufferram);
    (*mem_data).cached = p2k(global_node_page_state(NR_FILE_PAGES) - VAL.bufferram);

    si_swapinfo(&raw mut VAL);
    (*mem_data).totalswap = p2k(VAL.totalswap);
    (*mem_data).freeswap = p2k(VAL.freeswap);

    (*mem_data).timestamp = get_tod_clock();
    (*mem_data).sync_count_2 += 1;
}

static mut OPS: AppldataOps = AppldataOps {
    name: b"mem\0".as_ptr(),
    record_nr: APPLDATA_RECORD_MEM_ID,
    size: core::mem::size_of::<AppldataMemData>(),
    callback: Some(appldata_get_mem_data),
    owner: THIS_MODULE,
    mod_lvl: [0xF0, 0xF0], // EBCDIC "00"
    data: core::ptr::null_mut(),
};

/*
 * appldata_mem_init()
 *
 * init_data, register ops
 */
unsafe fn appldata_mem_init() -> c_int {
    let mut ret: c_int;

    OPS.data = kzalloc_obj::<AppldataMemData>();
    if OPS.data.is_null() {
        return -ENOMEM;
    }

    ret = appldata_register_ops(&raw mut OPS);
    if ret != 0 {
        kfree(OPS.data);
    }

    ret
}

/*
 * appldata_mem_exit()
 *
 * unregister ops
 */
unsafe fn appldata_mem_exit() {
    appldata_unregister_ops(&raw mut OPS);
    kfree(OPS.data);
}

module_init!(appldata_mem_init);
module_exit!(appldata_mem_exit);

module_license!("GPL");
module_author!("Gerald Schaefer");
module_description!("Linux-VM Monitor Stream, MEMORY statistics");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
