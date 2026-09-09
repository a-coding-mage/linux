// SPDX-License-Identifier: GPL-2.0-only

// Linux kernel dependencies supplied by other translation units.

unsafe fn mte_vma_tag_dump_size(m: *mut core_vma_metadata) -> c_ulong {
    ((*m).dump_size >> PAGE_SHIFT) * MTE_PAGE_TAG_STORAGE
}

/* Derived from dump_user_range(); start/end must be page-aligned */
unsafe fn mte_dump_tag_range(
    cprm: *mut coredump_params,
    start: c_ulong,
    len: c_ulong,
) -> c_int {
    let mut ret: c_int = 1;
    let mut addr: c_ulong;
    let mut tags: *mut c_void = core::ptr::null_mut();
    let mut locked: c_int = 0;

    addr = start;
    while addr < start + len {
        let page: *mut page = get_dump_page(addr, &mut locked);

        /*
         * get_dump_page() returns NULL when encountering an empty
         * page table entry that would otherwise have been filled with
         * the zero page. Skip the equivalent tag dump which would
         * have been all zeros.
         */
        if page.is_null() {
            dump_skip(cprm, MTE_PAGE_TAG_STORAGE);
            addr += PAGE_SIZE;
            continue;
        }

        /*
         * Pages mapped in user space as !pte_access_permitted() (e.g.
         * PROT_EXEC only) may not have the PG_mte_tagged flag set.
         */
        if !page_mte_tagged(page) {
            put_page(page);
            dump_skip(cprm, MTE_PAGE_TAG_STORAGE);
            addr += PAGE_SIZE;
            continue;
        }

        if tags.is_null() {
            tags = mte_allocate_tag_storage();
            if tags.is_null() {
                put_page(page);
                ret = 0;
                break;
            }
        }

        mte_save_page_tags(page_address(page), tags);
        put_page(page);
        if !dump_emit(cprm, tags, MTE_PAGE_TAG_STORAGE) {
            ret = 0;
            break;
        }

        addr += PAGE_SIZE;
    }

    if !tags.is_null() {
        mte_free_tag_storage(tags);
    }

    ret
}

pub unsafe fn elf_core_extra_phdrs(cprm: *mut coredump_params) -> Elf_Half {
    let mut i: c_int;
    let mut m: *mut core_vma_metadata;
    let mut vma_count: c_int = 0;

    if system_supports_mte() {
        i = 0;
        m = (*cprm).vma_meta;
        while i < (*cprm).vma_count {
            if (*m).flags & VM_MTE != 0 {
                vma_count += 1;
            }
            i += 1;
            m = (*cprm).vma_meta.add(i as usize);
        }
    }

    vma_count as Elf_Half
}

pub unsafe fn elf_core_write_extra_phdrs(
    cprm: *mut coredump_params,
    mut offset: loff_t,
) -> c_int {
    let mut i: c_int;
    let mut m: *mut core_vma_metadata;

    if system_supports_mte() {
        i = 0;
        m = (*cprm).vma_meta;
        while i < (*cprm).vma_count {
            if (*m).flags & VM_MTE != 0 {
                let mut phdr: elf_phdr = core::mem::zeroed();

                phdr.p_type = PT_AARCH64_MEMTAG_MTE;
                phdr.p_offset = offset;
                phdr.p_vaddr = (*m).start;
                phdr.p_paddr = 0;
                phdr.p_filesz = mte_vma_tag_dump_size(m);
                phdr.p_memsz = (*m).end - (*m).start;
                offset += phdr.p_filesz;
                phdr.p_flags = 0;
                phdr.p_align = 0;

                if !dump_emit(cprm, &mut phdr as *mut elf_phdr as *const c_void, core::mem::size_of::<elf_phdr>()) {
                    return 0;
                }
            }
            i += 1;
            m = (*cprm).vma_meta.add(i as usize);
        }
    }

    1
}

pub unsafe fn elf_core_extra_data_size(cprm: *mut coredump_params) -> size_t {
    let mut i: c_int;
    let mut m: *mut core_vma_metadata;
    let mut data_size: size_t = 0;

    if system_supports_mte() {
        i = 0;
        m = (*cprm).vma_meta;
        while i < (*cprm).vma_count {
            if (*m).flags & VM_MTE != 0 {
                data_size += mte_vma_tag_dump_size(m) as size_t;
            }
            i += 1;
            m = (*cprm).vma_meta.add(i as usize);
        }
    }

    data_size
}

pub unsafe fn elf_core_write_extra_data(cprm: *mut coredump_params) -> c_int {
    let mut i: c_int;
    let mut m: *mut core_vma_metadata;

    if system_supports_mte() {
        i = 0;
        m = (*cprm).vma_meta;
        while i < (*cprm).vma_count {
            if (*m).flags & VM_MTE != 0 {
                if mte_dump_tag_range(cprm, (*m).start, (*m).dump_size) == 0 {
                    return 0;
                }
            }
            i += 1;
            m = (*cprm).vma_meta.add(i as usize);
        }
    }

    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
