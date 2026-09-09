// SPDX-License-Identifier: GPL-2.0
/*
 * Hypervisor filesystem for Linux on s390
 *
 * Diag 0C implementation
 *
 * Copyright IBM Corp. 2014
 */

// C dependencies supplied by the surrounding kernel translation.

const DBFS_D0C_HDR_VERSION: u32 = 0;

/*
 * Get hypfs_diag0c_entry from CPU vector and store diag0c data
 */
unsafe fn diag0c_fn(data: *mut core::ffi::c_void) {
    diag0c((*(data as *mut *mut core::ffi::c_void)).add(smp_processor_id()));
}

/*
 * Allocate buffer and store diag 0c data
 */
unsafe fn diag0c_store(count: *mut u32) -> *mut hypfs_diag0c_data {
    let mut diag0c_data: *mut hypfs_diag0c_data;
    let cpu_count: u32;
    let mut cpu: u32;
    let mut i: u32;
    let cpu_vec: *mut *mut core::ffi::c_void;

    cpus_read_lock();
    cpu_count = num_online_cpus();
    cpu_vec = kmalloc_objs(num_possible_cpus());
    if cpu_vec.is_null() {
        cpus_read_unlock();
        return ERR_PTR(-12);
    }
    /* Note: Diag 0c needs 8 byte alignment and real storage */
    diag0c_data = kzalloc_flex(cpu_count);
    if diag0c_data.is_null() {
        kfree(cpu_vec as *mut core::ffi::c_void);
        cpus_read_unlock();
        return ERR_PTR(-12);
    }
    i = 0;
    /* Fill CPU vector for each online CPU */
    for_each_online_cpu!(cpu) {
        (*diag0c_data).entry.add(i as usize).as_mut().unwrap().cpu = cpu;
        *cpu_vec.add(cpu as usize) = (*diag0c_data).entry.add(i as usize) as *mut core::ffi::c_void;
        i = i.wrapping_add(1);
    }
    /* Collect data all CPUs */
    on_each_cpu(Some(diag0c_fn), cpu_vec as *mut core::ffi::c_void, 1);
    *count = cpu_count;
    kfree(cpu_vec as *mut core::ffi::c_void);
    cpus_read_unlock();
    diag0c_data
}

/*
 * Hypfs DBFS callback: Free diag 0c data
 */
unsafe extern "C" fn dbfs_diag0c_free(data: *const core::ffi::c_void) {
    kfree(data as *mut core::ffi::c_void);
}

/*
 * Hypfs DBFS callback: Create diag 0c data
 */
unsafe extern "C" fn dbfs_diag0c_create(
    data: *mut *mut core::ffi::c_void,
    data_free_ptr: *mut *mut core::ffi::c_void,
    size: *mut usize,
) -> i32 {
    let mut count: u32 = 0;
    let diag0c_data = diag0c_store(&mut count);
    if IS_ERR(diag0c_data as *const core::ffi::c_void) {
        return PTR_ERR(diag0c_data as *const core::ffi::c_void);
    }
    memset(
        &mut (*diag0c_data).hdr as *mut _ as *mut core::ffi::c_void,
        0,
        core::mem::size_of_val(&(*diag0c_data).hdr),
    );
    store_tod_clock_ext((*diag0c_data).hdr.tod_ext.as_mut_ptr() as *mut tod_clock);
    (*diag0c_data).hdr.len = count as usize * core::mem::size_of::<hypfs_diag0c_entry>();
    (*diag0c_data).hdr.version = DBFS_D0C_HDR_VERSION;
    (*diag0c_data).hdr.count = count;
    *data = diag0c_data as *mut core::ffi::c_void;
    *data_free_ptr = diag0c_data as *mut core::ffi::c_void;
    *size = (*diag0c_data).hdr.len + core::mem::size_of::<hypfs_diag0c_hdr>();
    0
}

/*
 * Hypfs DBFS file structure
 */
static mut dbfs_file_0c: hypfs_dbfs_file = hypfs_dbfs_file {
    name: b"diag_0c\0".as_ptr() as *const core::ffi::c_char,
    data_create: Some(dbfs_diag0c_create),
    data_free: Some(dbfs_diag0c_free),
};

/*
 * Initialize diag 0c interface for z/VM
 */
unsafe extern "C" fn hypfs_diag0c_init() -> i32 {
    if !machine_is_vm() {
        return 0;
    }
    hypfs_dbfs_create_file(&mut dbfs_file_0c);
    0
}

/*
 * Shutdown diag 0c interface for z/VM
 */
unsafe extern "C" fn hypfs_diag0c_exit() {
    if !machine_is_vm() {
        return;
    }
    hypfs_dbfs_remove_file(&mut dbfs_file_0c);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
