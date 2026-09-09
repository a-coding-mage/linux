// SPDX-License-Identifier: GPL-2.0
/*
 * Load ELF vmlinux file for the kexec_file_load syscall.
 *
 * Copyright (c) 2019 Sven Schnelle <svens@stackframe.org>
 *
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn elf_load(
    image: *mut kimage,
    kernel_buf: *mut core::ffi::c_char,
    kernel_len: c_ulong,
    initrd: *mut core::ffi::c_char,
    initrd_len: c_ulong,
    cmdline: *mut core::ffi::c_char,
    cmdline_len: c_ulong,
) -> *mut core::ffi::c_void {
    let mut ret: c_int;
    let mut i: c_int;
    let mut kernel_load_addr: c_ulong = 0;
    let mut ehdr: elfhdr = core::mem::zeroed();
    let mut elf_info: kexec_elf_info = core::mem::zeroed();
    let mut kbuf: kexec_buf = kexec_buf {
        image,
        buf_min: 0,
        buf_max: !0,
        ..core::mem::zeroed()
    };

    ret = kexec_build_elf_info(kernel_buf, kernel_len, &mut ehdr, &mut elf_info);
    if ret != 0 {
        return core::ptr::null_mut();
    }

    ret = kexec_elf_load(image, &ehdr, &elf_info, &mut kbuf, &mut kernel_load_addr);
    if ret != 0 {
        return core::ptr::null_mut();
    }

    (*image).start = __pa((*elf_info.ehdr).e_entry);

    i = 0;
    while i < (*image).nr_segments {
        (*image).segment.add(i as usize).as_mut().unwrap().mem =
            __pa((*image).segment.add(i as usize).as_ref().unwrap().mem);
        i += 1;
    }

    kexec_dprintk(
        b"Loaded the kernel at 0x%lx, entry at 0x%lx\n\0".as_ptr() as *const c_char,
        kernel_load_addr,
        (*image).start,
    );

    if !initrd.is_null() {
        kbuf.buffer = initrd;
        kbuf.bufsz = initrd_len;
        kbuf.memsz = initrd_len;
        kbuf.buf_align = PAGE_SIZE;
        kbuf.top_down = false;
        kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
        ret = kexec_add_buffer(&mut kbuf);
        if ret != 0 {
            return core::ptr::null_mut();
        }

        kexec_dprintk(
            b"Loaded initrd at 0x%lx\n\0".as_ptr() as *const c_char,
            kbuf.mem,
        );
        (*image).arch.initrd_start = kbuf.mem;
        (*image).arch.initrd_end = kbuf.mem + initrd_len;
    }

    if !cmdline.is_null() {
        kbuf.buffer = cmdline;
        kbuf.bufsz = ALIGN(cmdline_len, 8);
        kbuf.memsz = ALIGN(cmdline_len, 8);
        kbuf.buf_align = PAGE_SIZE;
        kbuf.buf_min = (*PAGE0).mem_free + PAGE_SIZE;
        kbuf.buf_max = kernel_load_addr;
        kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
        ret = kexec_add_buffer(&mut kbuf);
        if ret != 0 {
            return core::ptr::null_mut();
        }

        kexec_dprintk(
            b"Loaded cmdline at 0x%lx\n\0".as_ptr() as *const c_char,
            kbuf.mem,
        );
        (*image).arch.cmdline = kbuf.mem;
    }

    core::ptr::null_mut()
}

pub static kexec_elf_ops: kexec_file_ops = kexec_file_ops {
    probe: Some(kexec_elf_probe),
    load: Some(elf_load),
};

pub static kexec_file_loaders: [*const kexec_file_ops; 2] = [
    &kexec_elf_ops,
    core::ptr::null(),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
