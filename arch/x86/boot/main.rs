// SPDX-License-Identifier: GPL-2.0-only
/* -*- linux-c -*- ------------------------------------------------------- *
 *
 *   Copyright (C) 1991, 1992 Linus Torvalds
 *   Copyright 2007 rPath, Inc. - All Rights Reserved
 *   Copyright 2009 Intel Corporation; author H. Peter Anvin
 *
 * ----------------------------------------------------------------------- */

/*
 * Main module for the real-mode kernel code
 */
// Dependencies supplied by the surrounding boot code are intentionally not
// implemented here.

#[repr(C, packed)]
struct OldCmdline {
    cl_magic: u16,
    cl_offset: u16,
}

#[no_mangle]
pub static mut boot_params: boot_params = unsafe { core::mem::zeroed() };

#[no_mangle]
pub static mut pio_ops: port_io_ops = unsafe { core::mem::zeroed() };

#[no_mangle]
pub static mut HEAP: *mut u8 = _end;
#[no_mangle]
pub static mut heap_end: *mut u8 = _end; // Default end of heap = no heap

unsafe fn copy_boot_params() {
    let oldcmd = absolute_pointer(OLD_CL_ADDRESS) as *const OldCmdline;

    // BUILD_BUG_ON(sizeof(boot_params) != 4096);
    memcpy(
        core::ptr::addr_of_mut!(boot_params.hdr) as *mut core::ffi::c_void,
        core::ptr::addr_of!(hdr) as *const core::ffi::c_void,
        core::mem::size_of_val(&hdr),
    );

    if (*core::ptr::addr_of!(boot_params.hdr)).cmd_line_ptr == 0
        && (*oldcmd).cl_magic == OLD_CL_MAGIC
    {
        let cmdline_seg: u16;

        if (*oldcmd).cl_offset < (*core::ptr::addr_of!(boot_params.hdr)).setup_move_size {
            cmdline_seg = ds();
        } else {
            cmdline_seg = 0x9000;
        }

        (*core::ptr::addr_of_mut!(boot_params.hdr)).cmd_line_ptr =
            ((cmdline_seg as u32) << 4) + (*oldcmd).cl_offset as u32;
    }
}

unsafe fn keyboard_init() {
    let mut ireg: biosregs = core::mem::zeroed();
    let mut oreg: biosregs = core::mem::zeroed();

    initregs(&mut ireg);

    ireg.ah = 0x02;
    intcall(0x16, &mut ireg, &mut oreg);
    (*core::ptr::addr_of_mut!(boot_params)).kbd_status = oreg.al;

    ireg.ax = 0x0305;
    intcall(0x16, &mut ireg, core::ptr::null_mut());
}

unsafe fn query_ist() {
    let mut ireg: biosregs = core::mem::zeroed();
    let mut oreg: biosregs = core::mem::zeroed();

    if cpu.level < 6 {
        return;
    }

    initregs(&mut ireg);
    ireg.ax = 0xe980;
    ireg.edx = 0x47534943;
    intcall(0x15, &mut ireg, &mut oreg);

    (*core::ptr::addr_of_mut!(boot_params)).ist_info.signature = oreg.eax;
    (*core::ptr::addr_of_mut!(boot_params)).ist_info.command = oreg.ebx;
    (*core::ptr::addr_of_mut!(boot_params)).ist_info.event = oreg.ecx;
    (*core::ptr::addr_of_mut!(boot_params)).ist_info.perf_level = oreg.edx;
}

unsafe fn set_bios_mode() {
    // CONFIG_X86_64 conditional: retained as the source build-time intent.
    #[cfg(CONFIG_X86_64)]
    {
        let mut ireg: biosregs = core::mem::zeroed();

        initregs(&mut ireg);
        ireg.ax = 0xec00;
        ireg.bx = 2;
        intcall(0x15, &mut ireg, core::ptr::null_mut());
    }
}

unsafe fn init_heap() {
    let mut stack_end: *mut u8;

    if (*core::ptr::addr_of!(boot_params.hdr)).loadflags & CAN_USE_HEAP != 0 {
        stack_end = (current_stack_pointer - STACK_SIZE) as *mut u8;
        heap_end = ((*core::ptr::addr_of!(boot_params.hdr)).heap_end_ptr as usize + 0x200)
            as *mut u8;
        if (heap_end as usize) > (stack_end as usize) {
            heap_end = stack_end;
        }
    } else {
        puts("WARNING: Ancient bootloader, some functionality may be limited!\n");
    }
}

#[no_mangle]
pub unsafe extern "C" fn main() {
    init_default_io_ops();
    copy_boot_params();
    console_init();
    if cmdline_find_option_bool("debug") {
        puts("early console in setup code\n");
    }
    init_heap();

    if validate_cpu() {
        puts("Unable to boot - please use a kernel appropriate for your CPU.\n");
        die();
    }

    set_bios_mode();
    detect_memory();
    keyboard_init();
    query_ist();

    if IS_ENABLED(CONFIG_APM) {
        query_apm_bios();
    }
    if IS_ENABLED(CONFIG_EDD) {
        query_edd();
    }

    set_video();
    go_to_protected_mode();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
