// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * BSC913xRDB Board Setup
 *
 * Author: Priyanka Jain <Priyanka.Jain@freescale.com>
 *
 * Copyright 2011-2012 Freescale Semiconductor Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::c_void;

extern "C" {
    fn mpic_alloc(
        node: *mut c_void,
        flags: u32,
        flags2: u32,
        irq_offset: u32,
        irq_count: u32,
        name: *const i8,
    ) -> *mut mpic;
    fn mpic_init(mpic: *mut mpic);
    fn mpic_get_irq() -> i32;
    fn udbg_progress(message: *const i8, value: u32);
    fn mpc85xx_common_publish_devices() -> i32;
    fn pr_err(message: *const i8);
    fn pr_info(message: *const i8);
}

#[repr(C)]
pub struct mpic {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ppc_md_struct {
    pub progress: Option<unsafe extern "C" fn(*const i8, u32)>,
}

extern "C" {
    static mut ppc_md: ppc_md_struct;
}

const MPIC_BIG_ENDIAN: u32 = 1 << 0;
const MPIC_SINGLE_DEST_CPU: u32 = 1 << 1;

unsafe fn bsc913x_rdb_pic_init() {
    let mpic = mpic_alloc(
        core::ptr::null_mut(),
        0,
        MPIC_BIG_ENDIAN | MPIC_SINGLE_DEST_CPU,
        0,
        256,
        b" OpenPIC  \0".as_ptr() as *const i8,
    );

    if mpic.is_null() {
        pr_err(b"bsc913x: Failed to allocate MPIC structure\n\0".as_ptr() as *const i8);
    } else {
        mpic_init(mpic);
    }
}

/*
 * Setup the architecture
 */
unsafe fn bsc913x_rdb_setup_arch() {
    if let Some(progress) = ppc_md.progress {
        progress(
            b"bsc913x_rdb_setup_arch()\0".as_ptr() as *const i8,
            0,
        );
    }

    pr_info(b"bsc913x board from Freescale Semiconductor\n\0".as_ptr() as *const i8);
}

// machine_device_initcall(bsc9131_rdb, mpc85xx_common_publish_devices);
#[allow(dead_code)]
unsafe fn bsc9131_rdb_device_initcall() -> i32 {
    mpc85xx_common_publish_devices()
}

#[repr(C)]
pub struct MachineDesc {
    pub name: *const i8,
    pub compatible: *const i8,
    pub setup_arch: unsafe fn(),
    pub init_irq: unsafe fn(),
    pub get_irq: unsafe extern "C" fn() -> i32,
    pub progress: unsafe extern "C" fn(*const i8, u32),
}

#[no_mangle]
pub static bsc9131_rdb: MachineDesc = MachineDesc {
    .name: b"BSC9131 RDB\0".as_ptr() as *const i8,
    .compatible: b"fsl,bsc9131rdb\0".as_ptr() as *const i8,
    .setup_arch: bsc913x_rdb_setup_arch,
    .init_irq: bsc913x_rdb_pic_init,
    .get_irq: mpic_get_irq,
    .progress: udbg_progress,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
