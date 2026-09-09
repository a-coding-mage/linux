// SPDX-License-Identifier: GPL-2.0-only
/*
 * PPC32 code to handle Linux booting another kernel.
 *
 * Copyright (C) 2002-2003 Eric Biederman  <ebiederm@xmission.com>
 * GameCube/ppc32 port Copyright (C) 2004 Albert Herranz
 * Copyright (C) 2005 IBM Corporation.
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::{c_int, c_uint, c_ulong, c_void};

pub type relocate_new_kernel_t = unsafe extern "C" fn(
    indirection_page: c_ulong,
    reboot_code_buffer: c_ulong,
    start_address: c_ulong,
) -> !;

#[repr(C)]
pub struct kimage {
    pub head: c_ulong,
    pub control_code_page: *mut c_void,
    pub start: c_ulong,
}

pub const KEXEC_CONTROL_PAGE_SIZE: usize = 4096;

extern "C" {
    pub static relocate_new_kernel_size: c_uint;

    fn local_irq_disable();
    fn machine_kexec_mask_interrupts();
    fn page_address(page: *mut c_void) -> *mut c_void;
    fn virt_to_phys(address: *mut c_void) -> c_ulong;
    fn memcpy(destination: *mut c_void, source: *const c_void, count: usize) -> *mut c_void;
    fn flush_icache_range(start: c_ulong, end: c_ulong);
    fn printk(format: *const u8, ...) -> c_int;
    fn relocate_new_kernel(
        indirection_page: c_ulong,
        reboot_code_buffer: c_ulong,
        start_address: c_ulong,
    ) -> !;
}

// This is a generic machine_kexec function suitable at least for
// non-OpenFirmware embedded platforms.
// It merely copies the image relocation code to the control page and
// jumps to it.
// A platform specific function may just call this one.
pub unsafe extern "C" fn default_machine_kexec(image: *mut kimage) {
    let mut page_list: c_ulong;
    let reboot_code_buffer: c_ulong;
    let reboot_code_buffer_phys: c_ulong;
    let rnk: relocate_new_kernel_t;

    // Interrupts aren't acceptable while we reboot
    local_irq_disable();

    // mask each interrupt so we are in a more sane state for the
    // kexec kernel
    machine_kexec_mask_interrupts();

    page_list = (*image).head;

    // we need both effective and real address here
    reboot_code_buffer = page_address((*image).control_code_page) as c_ulong;
    reboot_code_buffer_phys = virt_to_phys(reboot_code_buffer as *mut c_void);

    // copy our kernel relocation code to the control code page
    memcpy(
        reboot_code_buffer as *mut c_void,
        relocate_new_kernel as *const c_void,
        relocate_new_kernel_size as usize,
    );

    flush_icache_range(
        reboot_code_buffer,
        reboot_code_buffer.wrapping_add(KEXEC_CONTROL_PAGE_SIZE as c_ulong),
    );
    printk(b"Bye!\0".as_ptr());

    // Build-time condition preserved from !IS_ENABLED(CONFIG_PPC_85xx) &&
    // !IS_ENABLED(CONFIG_44x).
    relocate_new_kernel(page_list, reboot_code_buffer_phys, (*image).start);

    // now call it
    rnk = core::mem::transmute(reboot_code_buffer);
    rnk(page_list, reboot_code_buffer_phys, (*image).start);
}

pub unsafe extern "C" fn machine_kexec_prepare(_image: *mut kimage) -> c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
