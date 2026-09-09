// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation.

unsafe extern "C" {
    fn relocate_new_kernel(head: ::core::ffi::c_ulong,
                           start: ::core::ffi::c_ulong,
                           phys: ::core::ffi::c_ulong);

    static relocate_new_kernel_size: ::core::ffi::c_uint;
    static mut kexec_initrd_start_offset: ::core::ffi::c_uint;
    static mut kexec_initrd_end_offset: ::core::ffi::c_uint;
    static mut kexec_cmdline_offset: ::core::ffi::c_uint;
    static mut kexec_free_mem_offset: ::core::ffi::c_uint;
}

unsafe fn kexec_show_segment_info(kimage: *const kimage, n: ::core::ffi::c_ulong) {
    pr_debug!(
        "    segment[%lu]: %016lx - %016lx, 0x%lx bytes, %lu pages\n",
        n,
        (*kimage).segment[n as usize].mem,
        (*kimage).segment[n as usize].mem + (*kimage).segment[n as usize].memsz,
        (*kimage).segment[n as usize].memsz as ::core::ffi::c_ulong,
        (*kimage).segment[n as usize].memsz as ::core::ffi::c_ulong / PAGE_SIZE,
    );
}

unsafe fn kexec_image_info(kimage: *const kimage) {
    let mut i: ::core::ffi::c_ulong = 0;

    pr_debug!("kexec kimage info:\n");
    pr_debug!("  type:        %d\n", (*kimage).type_);
    pr_debug!("  start:       %lx\n", (*kimage).start);
    pr_debug!("  head:        %lx\n", (*kimage).head);
    pr_debug!("  nr_segments: %lu\n", (*kimage).nr_segments);

    while i < (*kimage).nr_segments {
        kexec_show_segment_info(kimage, i);
        i += 1;
    }

    // #ifdef CONFIG_KEXEC_FILE
    #[cfg(CONFIG_KEXEC_FILE)]
    if (*kimage).file_mode {
        pr_debug!(
            "cmdline: %.*s\n",
            (*kimage).cmdline_buf_len as ::core::ffi::c_int,
            (*kimage).cmdline_buf,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn machine_kexec_cleanup(_kimage: *mut kimage) {}

#[no_mangle]
pub unsafe extern "C" fn machine_crash_shutdown(_regs: *mut pt_regs) {}

#[no_mangle]
pub unsafe extern "C" fn machine_shutdown() {
    smp_send_stop();
    while num_online_cpus() > 1 {
        cpu_relax();
        mdelay(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn machine_kexec(image: *mut kimage) {
    // #ifdef CONFIG_64BIT
    #[cfg(CONFIG_64BIT)]
    let mut desc: Elf64_Fdesc = ::core::mem::zeroed();

    let reloc: unsafe extern "C" fn(::core::ffi::c_ulong,
                                     ::core::ffi::c_ulong,
                                     ::core::ffi::c_ulong);

    let phys = page_to_phys((*image).control_code_page);
    let virt = __fix_to_virt(FIX_TEXT_KEXEC) as *mut ::core::ffi::c_void;
    let arch: *mut kimage_arch = &mut (*image).arch;

    set_fixmap(FIX_TEXT_KEXEC, phys);

    flush_cache_all();

    // #ifdef CONFIG_64BIT
    #[cfg(CONFIG_64BIT)]
    {
        reloc = ::core::mem::transmute(&desc);
        desc.addr = virt as ::core::ffi::c_longlong;
    }
    // #else
    #[cfg(not(CONFIG_64BIT))]
    {
        reloc = ::core::mem::transmute(virt);
    }

    memcpy(
        virt,
        dereference_function_descriptor(relocate_new_kernel as *const ::core::ffi::c_void),
        relocate_new_kernel_size as usize,
    );

    *((virt as *mut u8).add(kexec_cmdline_offset as usize) as *mut ::core::ffi::c_ulong) = (*arch).cmdline;
    *((virt as *mut u8).add(kexec_initrd_start_offset as usize) as *mut ::core::ffi::c_ulong) = (*arch).initrd_start;
    *((virt as *mut u8).add(kexec_initrd_end_offset as usize) as *mut ::core::ffi::c_ulong) = (*arch).initrd_end;
    *((virt as *mut u8).add(kexec_free_mem_offset as usize) as *mut ::core::ffi::c_ulong) = PAGE0.mem_free;

    flush_cache_all();
    flush_tlb_all();
    local_irq_disable();

    reloc((*image).head & PAGE_MASK, (*image).start, phys);
}

#[no_mangle]
pub unsafe extern "C" fn machine_kexec_prepare(image: *mut kimage) -> ::core::ffi::c_int {
    kexec_image_info(image);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
