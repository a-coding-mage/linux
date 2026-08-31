/* SPDX-License-Identifier: GPL-2.0-or-later */

// C header guard removed: _OBJTOOL_KLP_H.

pub const SHF_RELA_LIVEPATCH: u32 = 0x00100000;
pub const SHN_LIVEPATCH: u32 = 0xff20;

/*
 * .init.klp_objects and .init.klp_funcs are created by klp diff and used by the
 * patch module init code to build the klp_patch, klp_object and klp_func
 * structs needed by the livepatch API.
 */
pub const KLP_OBJECTS_SEC: &str = ".init.klp_objects";
pub const KLP_FUNCS_SEC: &str = ".init.klp_funcs";

/*
 * __klp_relocs.<objname> are intermediate sections which are created by klp
 * diff and converted into KLP symbols/relas by "objtool klp post-link".  This
 * is needed to work around the linker, which doesn't preserve SHN_LIVEPATCH or
 * SHF_RELA_LIVEPATCH, nor does it support having two RELA sections for a
 * single PROGBITS section.
 *
 * "objname" is the object whose loading gates the relocation: "vmlinux" for
 * references to vmlinux symbols, otherwise the name of the module being
 * patched.  post-link uses it to name the resulting
 * .klp.rela.objname.section_name sections.
 */
pub const KLP_RELOCS_SEC: &str = "__klp_relocs";
pub const KLP_STRINGS_SEC: &str = ".rodata.klp.str1.1";

pub const KLP_TOMBSTONE_PREFIX: &str = ".klp.tombstone.";

#[repr(C)]
pub struct klp_reloc {
    pub offset: *mut core::ffi::c_void,
    pub sym: *mut core::ffi::c_void,
    pub type_: u32,
}

/*
 * .klp.symid is used to correlate symbols between vmlinux.o and vmlinux, for
 * calculating sympos to disambiguate duplicately-named symbols.
 */
pub const KLP_SYMID_SEC: &str = ".klp.symid";

#[repr(C)]
pub struct klp_symid {
    pub id: u64,
    pub addr: u64,
}

#[repr(C)]
pub struct objtool_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct elf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn klp_create_symid_sections(file: *mut objtool_file) -> core::ffi::c_int;

    pub fn klp_sympos_init(orig: *mut elf) -> core::ffi::c_int;
    pub fn klp_find_sympos(elf: *mut elf, sym: *mut symbol) -> core::ffi::c_ulong;

    pub fn cmd_klp_checksum(argc: core::ffi::c_int, argv: *mut *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn cmd_klp_diff(argc: core::ffi::c_int, argv: *mut *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn cmd_klp_post_link(argc: core::ffi::c_int, argv: *mut *const core::ffi::c_char) -> core::ffi::c_int;
}
