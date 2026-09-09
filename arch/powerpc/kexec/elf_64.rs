// SPDX-License-Identifier: GPL-2.0-only
/*
 * Load ELF vmlinux file for the kexec_file_load syscall.
 *
 * Copyright (C) 2004  Adam Litke (agl@us.ibm.com)
 * Copyright (C) 2004  IBM Corp.
 * Copyright (C) 2005  R Sharada (sharada@in.ibm.com)
 * Copyright (C) 2006  Mohan Kumar M (mohan@in.ibm.com)
 * Copyright (C) 2016  IBM Corporation
 *
 * Based on kexec-tools' kexec-elf-exec.c and kexec-elf-ppc64.c.
 * Heavily modified for the kernel by
 * Thiago Jung Bauermann <bauerman@linux.vnet.ibm.com>.
 */

// pr_fmt(fmt) = "kexec_elf: " fmt

use core::ffi::{c_char, c_void};

unsafe fn elf64_load(
    image: *mut kimage,
    mut kernel_buf: *mut c_char,
    kernel_len: usize,
    mut initrd: *mut c_char,
    initrd_len: usize,
    mut cmdline: *mut c_char,
    cmdline_len: usize,
) -> *mut c_void {
    let mut ret: i32;
    let mut kernel_load_addr: usize = 0;
    let mut initrd_load_addr: usize = 0;
    let mut fdt_load_addr: usize;
    let mut fdt: *mut c_void;
    let mut slave_code: *const c_void;
    let mut ehdr: elfhdr = core::mem::zeroed();
    let mut modified_cmdline: *mut c_char = core::ptr::null_mut();
    let mut rmem: *mut crash_mem = core::ptr::null_mut();
    let mut elf_info: kexec_elf_info = core::mem::zeroed();
    let mut kbuf = kexec_buf {
        image,
        buf_min: 0,
        buf_max: ppc64_rma_size,
        ..core::mem::zeroed()
    };
    let mut pbuf = kexec_buf {
        image,
        buf_min: 0,
        buf_max: ppc64_rma_size,
        top_down: true,
        mem: KEXEC_BUF_MEM_UNKNOWN,
        ..core::mem::zeroed()
    };

    ret = kexec_build_elf_info(kernel_buf, kernel_len, &mut ehdr, &mut elf_info);
    if ret != 0 {
        return ERR_PTR(ret);
    }

    if IS_ENABLED(CONFIG_CRASH_DUMP) && (*image).type_ == KEXEC_TYPE_CRASH {
        /* min & max buffer values for kdump case */
        kbuf.buf_min = crashk_res.start;
        pbuf.buf_min = crashk_res.start;
        kbuf.buf_max = if crashk_res.end < ppc64_rma_size {
            crashk_res.end
        } else {
            ppc64_rma_size - 1
        };
        pbuf.buf_max = kbuf.buf_max;
    }

    ret = kexec_elf_load(image, &ehdr, &elf_info, &mut kbuf, &mut kernel_load_addr);
    if ret != 0 {
        goto_out!();
    }

    kexec_dprintk!("Loaded the kernel at 0x%lx\n", kernel_load_addr);

    ret = kexec_load_purgatory(image, &mut pbuf);
    if ret != 0 {
        pr_err!("Loading purgatory failed.\n");
        goto_out!();
    }

    kexec_dprintk!("Loaded purgatory at 0x%lx\n", pbuf.mem);

    /* Load additional segments needed for panic kernel */
    if IS_ENABLED(CONFIG_CRASH_DUMP) && (*image).type_ == KEXEC_TYPE_CRASH {
        ret = load_crashdump_segments_ppc64(image, &mut kbuf);
        if ret != 0 {
            pr_err!("Failed to load kdump kernel segments\n");
            goto_out!();
        }

        ret = crash_load_dm_crypt_keys(image);
        if ret != 0 {
            goto_out!();
        }

        /* Setup cmdline for kdump kernel case */
        modified_cmdline = setup_kdump_cmdline(image, cmdline, cmdline_len);
        if modified_cmdline.is_null() {
            pr_err!("Setting up cmdline for kdump kernel failed\n");
            ret = -EINVAL;
            goto_out!();
        }
        cmdline = modified_cmdline;
    }

    if !initrd.is_null() {
        kbuf.buffer = initrd;
        kbuf.bufsz = initrd_len;
        kbuf.memsz = initrd_len;
        kbuf.buf_align = PAGE_SIZE;
        kbuf.top_down = false;
        kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
        ret = kexec_add_buffer(&mut kbuf);
        if ret != 0 {
            goto_out!();
        }
        initrd_load_addr = kbuf.mem;

        kexec_dprintk!("Loaded initrd at 0x%lx\n", initrd_load_addr);
    }

    ret = get_reserved_memory_ranges(&mut rmem);
    if ret != 0 {
        goto_out!();
    }

    fdt = of_kexec_alloc_and_setup_fdt(
        image,
        initrd_load_addr,
        initrd_len,
        cmdline,
        kexec_extra_fdt_size_ppc64(image, rmem),
    );
    if fdt.is_null() {
        pr_err!("Error setting up the new device tree.\n");
        ret = -EINVAL;
        goto_out!();
    }

    ret = setup_new_fdt_ppc64(image, fdt, rmem);
    if ret != 0 {
        kvfree(fdt);
        goto_out!();
    }

    if !IS_ENABLED(CONFIG_CRASH_HOTPLUG) || (*image).type_ != KEXEC_TYPE_CRASH {
        fdt_pack(fdt);
    }

    kbuf.buffer = fdt;
    kbuf.bufsz = fdt_totalsize(fdt);
    kbuf.memsz = kbuf.bufsz;
    kbuf.buf_align = PAGE_SIZE;
    kbuf.top_down = true;
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    ret = kexec_add_buffer(&mut kbuf);
    if ret != 0 {
        kvfree(fdt);
        goto_out!();
    }

    /* FDT will be freed in arch_kimage_file_post_load_cleanup */
    (*image).arch.fdt = fdt;

    fdt_load_addr = kbuf.mem;

    kexec_dprintk!("Loaded device tree at 0x%lx\n", fdt_load_addr);

    slave_code = (*elf_info.buffer.add((*elf_info.proghdrs).p_offset as usize)) as *const c_void;
    ret = setup_purgatory_ppc64(image, slave_code, fdt, kernel_load_addr, fdt_load_addr);
    if ret != 0 {
        pr_err!("Error setting up the purgatory.\n");
    }

    goto_out!();

    macro_rules! goto_out {
        () => {{
            if !rmem.is_null() {
                kfree(rmem as *mut c_void);
            }
            if !modified_cmdline.is_null() {
                kfree(modified_cmdline as *mut c_void);
            }
            kexec_free_elf_info(&mut elf_info);
            return if ret != 0 { ERR_PTR(ret) } else { core::ptr::null_mut() };
        }};
    }
}

const kexec_elf64_ops: kexec_file_ops = kexec_file_ops {
    probe: kexec_elf_probe,
    load: elf64_load,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
