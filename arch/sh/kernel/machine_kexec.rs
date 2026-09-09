// SPDX-License-Identifier: GPL-2.0
/*
 * machine_kexec.c - handle transition of Linux booting another kernel
 * Copyright (C) 2002-2003 Eric Biederman  <ebiederm@xmission.com>
 *
 * GameCube/ppc32 port Copyright (C) 2004 Albert Herranz
 * LANDISK/sh4 supported by kogiidena
 */

type RelocateNewKernelT = unsafe extern "C" fn(
    indirection_page: libc::c_ulong,
    reboot_code_buffer: libc::c_ulong,
    start_address: libc::c_ulong,
);

extern "C" {
    static relocate_new_kernel: libc::c_uchar;
    static relocate_new_kernel_size: libc::c_uint;
    static mut vbr_base: *mut libc::c_void;
}

pub unsafe extern "C" fn native_machine_crash_shutdown(_regs: *mut pt_regs) {
    /* Nothing to do for UP, but definitely broken for SMP.. */
}

/*
 * Do what every setup is needed on image and the
 * reboot code buffer to allow us to avoid allocations
 * later.
 */
pub unsafe extern "C" fn machine_kexec_prepare(_image: *mut kimage) -> libc::c_int {
    0
}

pub unsafe extern "C" fn machine_kexec_cleanup(_image: *mut kimage) {}

unsafe fn kexec_info(image: *mut kimage) {
    let mut i: libc::c_int;
    printk(b"kexec information\0".as_ptr() as *const libc::c_char);
    i = 0;
    while i < (*image).nr_segments {
        printk(
            b"  segment[%d]: 0x%08x - 0x%08x (0x%08x)\n\0".as_ptr()
                as *const libc::c_char,
            i,
            (*image).segment[i as usize].mem as libc::c_uint,
            ((*image).segment[i as usize].mem
                + (*image).segment[i as usize].memsz) as libc::c_uint,
            (*image).segment[i as usize].memsz as libc::c_uint,
        );
        i += 1;
    }
    printk(
        b"  start     : 0x%08x\n\n\0".as_ptr() as *const libc::c_char,
        (*image).start as libc::c_uint,
    );
}

/*
 * Do not allocate memory (or fail in any way) in machine_kexec().
 * We are past the point of no return, committed to rebooting now.
 */
pub unsafe extern "C" fn machine_kexec(image: *mut kimage) {
    let page_list: libc::c_ulong;
    let reboot_code_buffer: libc::c_ulong;
    let rnk: RelocateNewKernelT;
    let mut entry: libc::c_ulong;
    let mut ptr: *mut libc::c_ulong;
    let save_ftrace_enabled: libc::c_int;

    /*
     * Nicked from the mips version of machine_kexec():
     * The generic kexec code builds a page list with physical
     * addresses. Use phys_to_virt() to convert them to virtual.
     */
    ptr = &mut (*image).head;
    entry = *ptr;
    while entry != 0 && (entry & IND_DONE) == 0 {
        if (*ptr & IND_SOURCE) != 0 || (*ptr & IND_INDIRECTION) != 0 || (*ptr & IND_DESTINATION) != 0 {
            *ptr = phys_to_virt(*ptr) as libc::c_ulong;
        }
        entry = *ptr;
        ptr = if (entry & IND_INDIRECTION) != 0 {
            phys_to_virt(entry & PAGE_MASK) as *mut libc::c_ulong
        } else {
            ptr.add(1)
        };
    }

    // CONFIG_KEXEC_JUMP conditional code is preserved here for the target build.
    #[cfg(CONFIG_KEXEC_JUMP)]
    if (*image).preserve_context {
        save_processor_state();
    }

    save_ftrace_enabled = __ftrace_enabled_save();
    local_irq_disable();
    page_list = (*image).head;
    reboot_code_buffer = page_address((*image).control_code_page) as libc::c_ulong;
    memcpy(
        reboot_code_buffer as *mut libc::c_void,
        &relocate_new_kernel as *const _ as *const libc::c_void,
        relocate_new_kernel_size as usize,
    );

    kexec_info(image);
    flush_cache_all();
    sh_bios_vbr_reload();
    rnk = core::mem::transmute(reboot_code_buffer);
    rnk(page_list, reboot_code_buffer, phys_to_virt((*image).start) as libc::c_ulong);

    #[cfg(CONFIG_KEXEC_JUMP)]
    {
        core::arch::asm!("ldc {0}, vbr", in(reg) &vbr_base, options(nostack, preserves_flags));
        if (*image).preserve_context {
            restore_processor_state();
        }
        ptr = &mut (*image).head;
        entry = *ptr;
        while entry != 0 && (entry & IND_DONE) == 0 {
            if (*ptr & IND_SOURCE) != 0 || (*ptr & IND_INDIRECTION) != 0 || (*ptr & IND_DESTINATION) != 0 {
                *ptr = virt_to_phys(*ptr as *mut libc::c_void) as libc::c_ulong;
            }
            entry = *ptr;
            ptr = if (*ptr & IND_INDIRECTION) != 0 {
                phys_to_virt(*ptr & PAGE_MASK) as *mut libc::c_ulong
            } else {
                ptr.add(1)
            };
        }
    }
    __ftrace_enabled_restore(save_ftrace_enabled);
}

pub unsafe extern "C" fn reserve_crashkernel() {
    let mut crash_size: libc::c_ulonglong;
    let mut crash_base: libc::c_ulonglong;
    let mut ret: libc::c_int;

    if !IS_ENABLED(CONFIG_CRASH_RESERVE) { return; }
    ret = parse_crashkernel(boot_command_line, memblock_phys_mem_size(), &mut crash_size, &mut crash_base, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    if ret == 0 && crash_size > 0 { crashk_res.start = crash_base; crashk_res.end = crash_base + crash_size - 1; }
    if crashk_res.end == crashk_res.start { crashk_res.start = 0; crashk_res.end = 0; return; }
    crash_size = PAGE_ALIGN(resource_size(&crashk_res));
    if crashk_res.start == 0 {
        let max = memblock_end_of_DRAM() - memory_limit;
        crashk_res.start = memblock_phys_alloc_range(crash_size, PAGE_SIZE, 0, max);
        if crashk_res.start == 0 { pr_err(b"crashkernel allocation failed\n\0".as_ptr() as *const libc::c_char); crashk_res.start = 0; crashk_res.end = 0; return; }
    } else {
        ret = memblock_reserve(crashk_res.start, crash_size);
        if ret < 0 { pr_err(b"crashkernel reservation failed - memory is in use\n\0".as_ptr() as *const libc::c_char); crashk_res.start = 0; crashk_res.end = 0; return; }
    }
    crashk_res.end = crashk_res.start + crash_size - 1;
    if memblock_end_of_DRAM() - memory_limit <= crashk_res.end { memory_limit = 0; pr_info(b"Disabled memory limit for crashkernel\n\0".as_ptr() as *const libc::c_char); }
    pr_info(b"Reserving %ldMB of memory at 0x%08lx for crashkernel (System RAM: %ldMB)\n\0".as_ptr() as *const libc::c_char, (crash_size >> 20) as libc::c_ulong, crashk_res.start as libc::c_ulong, (memblock_phys_mem_size() >> 20) as libc::c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
