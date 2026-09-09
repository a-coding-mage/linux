/* SPDX-License-Identifier: GPL-2.0 */
/* The stuff needed for archs to support modules. */

/* Dependencies supplied by the kernel module and ELF headers. */

pub const ENOEXEC: i32 = 8;

extern "C" {
    pub fn module_elf_check_arch(hdr: *mut Elf_Ehdr) -> bool;

    pub fn module_frob_arch_sections(
        hdr: *mut Elf_Ehdr,
        sechdrs: *mut Elf_Shdr,
        secstrings: *mut i8,
        mod_: *mut module,
    ) -> i32;

    pub fn arch_mod_section_prepend(mod_: *mut module, section: u32) -> u32;

    pub fn module_init_section(name: *const i8) -> bool;
    pub fn module_exit_section(name: *const i8) -> bool;
    pub fn module_init_layout_section(sname: *const i8) -> bool;

    pub fn module_finalize(
        hdr: *const Elf_Ehdr,
        sechdrs: *const Elf_Shdr,
        mod_: *mut module,
    ) -> i32;

    pub fn module_arch_cleanup(mod_: *mut module);
    pub fn module_arch_freeing_init(mod_: *mut module);
}

#[cfg(CONFIG_MODULES_USE_ELF_REL)]
extern "C" {
    pub fn apply_relocate(
        sechdrs: *mut Elf_Shdr,
        strtab: *const i8,
        symindex: u32,
        relsec: u32,
        mod_: *mut module,
    ) -> i32;
}

#[cfg(not(CONFIG_MODULES_USE_ELF_REL))]
pub unsafe fn apply_relocate(
    _sechdrs: *mut Elf_Shdr,
    _strtab: *const i8,
    _symindex: u32,
    _relsec: u32,
    me: *mut module,
) -> i32 {
    printk(b"module %s: REL relocation unsupported\0".as_ptr() as *const i8, module_name(me));
    -ENOEXEC
}

#[cfg(CONFIG_MODULES_USE_ELF_RELA)]
extern "C" {
    pub fn apply_relocate_add(
        sechdrs: *mut Elf_Shdr,
        strtab: *const i8,
        symindex: u32,
        relsec: u32,
        mod_: *mut module,
    ) -> i32;
}

#[cfg(CONFIG_MODULES_USE_ELF_RELA)]
#[cfg(CONFIG_LIVEPATCH)]
extern "C" {
    pub fn clear_relocate_add(
        sechdrs: *mut Elf_Shdr,
        strtab: *const i8,
        symindex: u32,
        relsec: u32,
        me: *mut module,
    );
}

#[cfg(not(CONFIG_MODULES_USE_ELF_RELA))]
pub unsafe fn apply_relocate_add(
    _sechdrs: *mut Elf_Shdr,
    _strtab: *const i8,
    _symindex: u32,
    _relsec: u32,
    me: *mut module,
) -> i32 {
    printk(b"module %s: REL relocation unsupported\0".as_ptr() as *const i8, module_name(me));
    -ENOEXEC
}

#[cfg(CONFIG_MODULES)]
extern "C" {
    pub fn flush_module_init_free_work();
}

#[cfg(not(CONFIG_MODULES))]
pub unsafe fn flush_module_init_free_work() {}

extern "C" {
    fn printk(fmt: *const i8, ...);
    fn module_name(mod_: *mut module) -> *const i8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
