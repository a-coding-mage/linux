/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2009 Wind River Systems,
 *   written by Ralf Baechle <ralf@linux-mips.org>
 */

// Declarations supplied by the surrounding kernel translation.
use core::ffi::c_void;

extern "C" {
    fn prefetch(address: *const c_void);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn read_c0_status() -> u32;
    fn write_c0_status(status: u32);
    fn octeon_cop2_restore(cp2: *mut c_void);
    fn cu2_notifier(call: unsafe extern "C" fn(*mut notifier_block, usize, *mut c_void) -> i32,
                    priority: i32) -> i32;
    fn current_task() -> *mut task_struct;
    fn current_cp2(task: *mut task_struct) -> *mut c_void;
    fn kstk_status(task: *mut task_struct) -> *mut u32;
}

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

pub const CU2_EXCEPTION: usize = 0;
pub const NOTIFY_BAD: i32 = 0x0002;
pub const NOTIFY_OK: i32 = 0x0001;
pub const ST0_CU2: u32 = 0x20000000;

unsafe extern "C" fn cnmips_cu2_call(
    _nfb: *mut notifier_block,
    action: usize,
    _data: *mut c_void,
) -> i32 {
    let mut flags: usize = 0;
    let mut status: u32;

    match action {
        CU2_EXCEPTION => {
            let current = current_task();
            prefetch(current_cp2(current));
            local_irq_save(&mut flags);
            *kstk_status(current) |= ST0_CU2;
            status = read_c0_status();
            write_c0_status(status | ST0_CU2);
            octeon_cop2_restore(current_cp2(current));
            write_c0_status(status & !ST0_CU2);
            local_irq_restore(flags);

            return NOTIFY_BAD; // Don't call default notifier
        }
        _ => {}
    }

    NOTIFY_OK // Let default notifier send signals
}

#[link_section = ".init.text"]
unsafe extern "C" fn cnmips_cu2_setup() -> i32 {
    cu2_notifier(cnmips_cu2_call, 0)
}

// early_initcall(cnmips_cu2_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
