// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Module livepatch support
 *
 * Copyright (C) 2016 Jessica Yu <jeyu@redhat.com>
 */

/* Dependencies supplied by the kernel module environment and internal headers. */

/*
 * Persist ELF information about a module. Copy the ELF header,
 * section header table, section string table, and symtab section
 * index from info to mod->klp_info.
 */
pub unsafe fn copy_module_elf(mod_: *mut module, info: *mut load_info) -> i32 {
    let mut size: u32;
    let mut symndx: u32;
    let ret: i32;

    size = core::mem::size_of::<klp_info>() as u32;
    (*mod_).klp_info = kmalloc(size, GFP_KERNEL) as *mut klp_info;
    if (*mod_).klp_info.is_null() {
        return -12;
    }

    /* ELF header */
    size = core::mem::size_of_val(&(*(*mod_).klp_info).hdr) as u32;
    memcpy(
        &mut (*(*mod_).klp_info).hdr as *mut _,
        (*info).hdr as *const _,
        size as usize,
    );

    /* ELF section header table */
    size = (core::mem::size_of::<Elf_Shdr>() as u32)
        .wrapping_mul((*(*info).hdr).e_shnum as u32);
    (*(*mod_).klp_info).sechdrs =
        kmemdup((*info).sechdrs as *const _, size as usize, GFP_KERNEL) as *mut Elf_Shdr;
    if (*(*mod_).klp_info).sechdrs.is_null() {
        ret = -12;
        goto_free_info: {
            kfree((*mod_).klp_info as *mut _);
            return ret;
        }
    }

    /* ELF section name string table */
    size = (*info).sechdrs[(*(*info).hdr).e_shstrndx as usize].sh_size as u32;
    (*(*mod_).klp_info).secstrings =
        kmemdup((*info).secstrings as *const _, size as usize, GFP_KERNEL) as *mut _;
    if (*(*mod_).klp_info).secstrings.is_null() {
        ret = -12;
        kfree((*(*mod_).klp_info).sechdrs as *mut _);
        kfree((*mod_).klp_info as *mut _);
        return ret;
    }

    /* ELF symbol section index */
    symndx = (*info).index.sym as u32;
    (*(*mod_).klp_info).symndx = symndx;

    /*
     * For livepatch modules, core_kallsyms.symtab is a complete
     * copy of the original symbol table. Adjust sh_addr to point
     * to core_kallsyms.symtab since the copy of the symtab in module
     * init memory is freed at the end of do_init_module().
     */
    (*(*mod_).klp_info).sechdrs[symndx as usize].sh_addr =
        (*mod_).core_kallsyms.symtab as usize as _;

    return 0;
}

pub unsafe fn free_module_elf(mod_: *mut module) {
    kfree((*(*mod_).klp_info).sechdrs as *mut _);
    kfree((*(*mod_).klp_info).secstrings as *mut _);
    kfree((*mod_).klp_info as *mut _);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
