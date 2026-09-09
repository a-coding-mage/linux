// SPDX-License-Identifier: GPL-2.0
/*
 * ELF loader for kexec_file_load system call.
 *
 * Copyright IBM Corp. 2018
 *
 * Author(s): Philipp Rudo <prudo@linux.vnet.ibm.com>
 */

unsafe fn kexec_file_add_kernel_elf(
    image: *mut kimage,
    data: *mut s390_load_data,
) -> i32 {
    let mut buf: kexec_buf = core::mem::zeroed();
    let ehdr: *const Elf_Ehdr;
    let mut phdr: *const Elf_Phdr;
    let entry: Elf_Addr;
    let kernel: *mut u8;
    let mut i: i32;
    let ret: i32;

    kernel = (*image).kernel_buf as *mut u8;
    ehdr = kernel as *const Elf_Ehdr;
    buf.image = image;
    if (*image).type_ == KEXEC_TYPE_CRASH {
        entry = STARTUP_KDUMP_OFFSET;
    } else {
        entry = (*ehdr).e_entry;
    }

    phdr = (ehdr as *const u8).add((*ehdr).e_phoff as usize) as *const Elf_Phdr;
    i = 0;
    while i < (*ehdr).e_phnum as i32 {
        if (*phdr).p_type != PT_LOAD {
            phdr = phdr.add(1);
            i += 1;
            continue;
        }

        buf.buffer = kernel.add((*phdr).p_offset as usize) as *mut core::ffi::c_void;
        buf.bufsz = (*phdr).p_filesz;

        buf.mem = ALIGN((*phdr).p_paddr, (*phdr).p_align);
        /* CONFIG_CRASH_DUMP */
        if (*image).type_ == KEXEC_TYPE_CRASH {
            buf.mem = buf.mem.wrapping_add(crashk_res.start);
        }
        buf.memsz = (*phdr).p_memsz;
        (*data).memsz = ALIGN((*data).memsz, (*phdr).p_align).wrapping_add(buf.memsz);

        if entry.wrapping_sub((*phdr).p_paddr) < (*phdr).p_memsz {
            (*data).kernel_buf = buf.buffer;
            (*data).kernel_mem = buf.mem;
            (*data).parm = (buf.buffer as *mut u8).add(PARMAREA as usize)
                as *mut core::ffi::c_void;
        }

        ipl_report_add_component(
            (*data).report,
            &mut buf,
            IPL_RB_COMPONENT_FLAG_SIGNED | IPL_RB_COMPONENT_FLAG_VERIFIED,
            IPL_RB_CERT_UNKNOWN,
        );
        ret = kexec_add_buffer(&mut buf);
        if ret != 0 {
            return ret;
        }

        phdr = phdr.add(1);
        i += 1;
    }

    if (*data).memsz != 0 { 0 } else { -EINVAL }
}

unsafe fn s390_elf_load(
    image: *mut kimage,
    kernel: *mut i8,
    kernel_len: usize,
    _initrd: *mut i8,
    _initrd_len: usize,
    _cmdline: *mut i8,
    _cmdline_len: usize,
) -> *mut core::ffi::c_void {
    let ehdr: *const Elf_Ehdr;
    let mut phdr: *const Elf_Phdr;
    let mut size: usize;
    let mut i: i32;

    /* image->fobs->probe already checked for valid ELF magic number. */
    ehdr = kernel as *const Elf_Ehdr;

    if (*ehdr).e_type != ET_EXEC
        || (*ehdr).e_ident[EI_CLASS as usize] != ELFCLASS64
        || !elf_check_arch(ehdr)
    {
        return ERR_PTR(-EINVAL);
    }

    if (*ehdr).e_phnum == 0 || (*ehdr).e_phentsize as usize != core::mem::size_of::<Elf_Phdr>() {
        return ERR_PTR(-EINVAL);
    }

    size = (*ehdr).e_ehsize as usize + (*ehdr).e_phoff as usize;
    size += (*ehdr).e_phentsize as usize * (*ehdr).e_phnum as usize;
    if size > kernel_len {
        return ERR_PTR(-EINVAL);
    }

    phdr = (ehdr as *const u8).add((*ehdr).e_phoff as usize) as *const Elf_Phdr;
    size = ALIGN(size, (*phdr).p_align as usize);
    i = 0;
    while i < (*ehdr).e_phnum as i32 {
        if (*phdr).p_type == PT_INTERP {
            return ERR_PTR(-EINVAL);
        }

        if (*phdr).p_offset as usize > kernel_len {
            return ERR_PTR(-EINVAL);
        }

        size += ALIGN((*phdr).p_filesz as usize, (*phdr).p_align as usize);
        phdr = phdr.add(1);
        i += 1;
    }

    if size > kernel_len {
        return ERR_PTR(-EINVAL);
    }

    kexec_file_add_components(image, kexec_file_add_kernel_elf)
}

unsafe fn s390_elf_probe(buf: *const i8, len: usize) -> i32 {
    let ehdr: *const Elf_Ehdr;

    if len < core::mem::size_of::<Elf_Ehdr>() {
        return -ENOEXEC;
    }

    ehdr = buf as *const Elf_Ehdr;

    /* Only check the ELF magic number here and do proper validity check
     * in the loader. Any check here that fails would send the erroneous
     * ELF file to the image loader that does not care what it gets.
     * (Most likely) causing behavior not intended by the user.
     */
    if memcmp((*ehdr).e_ident.as_ptr() as *const core::ffi::c_void,
              ELFMAG.as_ptr() as *const core::ffi::c_void, SELFMAG) != 0
    {
        return -ENOEXEC;
    }

    0
}

pub static s390_kexec_elf_ops: kexec_file_ops = kexec_file_ops {
    probe: Some(s390_elf_probe),
    load: Some(s390_elf_load),
    /* CONFIG_KEXEC_SIG: verify_sig = s390_verify_sig */
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
