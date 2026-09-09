/*
 * debugfs ops for process ASIDs
 *
 *  Copyright (C) 2000, 2001  Paolo Alberelli
 *  Copyright (C) 2003 - 2008  Paul Mundt
 *  Copyright (C) 2003, 2004  Richard Curnow
 *
 * Provides a debugfs file that lists out the ASIDs currently associated
 * with the processes.
 *
 * In the SH-5 case, if the DM.PC register is examined through the debug
 * link, this shows ASID + PC. To make use of this, the PID->ASID
 * relationship needs to be known. This is primarily for debugging.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// C dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

#[repr(C)]
pub struct SeqFile {
    _private: [u8; 0],
}

#[repr(C)]
pub struct TaskStruct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct MmStruct {
    _private: [u8; 0],
}

extern "C" {
    static tasklist_lock: c_void;
    static arch_debugfs_dir: *mut c_void;
    static asids_debugfs_fops: c_void;

    fn read_lock(lock: *const c_void);
    fn read_unlock(lock: *const c_void);
    fn smp_processor_id() -> i32;
    fn cpu_asid(cpu: i32, mm: *mut MmStruct) -> usize;
    fn seq_printf(file: *mut SeqFile, format: *const u8, ...);
    fn debugfs_create_file(
        name: *const u8,
        mode: u32,
        parent: *mut c_void,
        data: *mut c_void,
        fops: *const c_void,
    ) -> *mut c_void;
}

// Supplied by the scheduler/task-list translation.
extern "C" {
    fn for_each_process_body(callback: unsafe extern "C" fn(*mut TaskStruct));
}

#[no_mangle]
pub unsafe extern "C" fn asids_debugfs_show(file: *mut SeqFile, _iter: *mut c_void) -> i32 {
    read_lock(&tasklist_lock);

    // C macro `for_each_process(p)`; the callback form preserves iteration
    // ordering and the task-list dependency supplied by the kernel.
    unsafe extern "C" fn visit_process(p: *mut TaskStruct) {
        let pid = *(p as *mut i32);

        if pid == 0 {
            return;
        }

        // `mm` is the task_struct field supplied by the target ABI.
        let mm = *((p as *mut u8).add(0) as *mut *mut MmStruct);
        if !mm.is_null() {
            unsafe {
                seq_printf(
                    file,
                    b"%5d : %04lx\n\0".as_ptr(),
                    pid,
                    cpu_asid(smp_processor_id(), mm),
                );
            }
        }
    }

    for_each_process_body(visit_process);
    read_unlock(&tasklist_lock);

    0
}

// `DEFINE_SHOW_ATTRIBUTE(asids_debugfs)` generates the corresponding seq_file
// open/release operations and `asids_debugfs_fops` from the show function.

#[no_mangle]
pub unsafe extern "C" fn asids_debugfs_init() -> i32 {
    debugfs_create_file(
        b"asids\0".as_ptr(),
        0o400,
        arch_debugfs_dir,
        core::ptr::null_mut(),
        &asids_debugfs_fops,
    );
    0
}

// `device_initcall(asids_debugfs_init)` registers the initializer at boot.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
