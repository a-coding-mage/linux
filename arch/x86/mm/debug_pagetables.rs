// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the Linux kernel headers are referenced externally.

use core::ffi::c_void;

#[repr(C)]
pub struct SeqFile {
    _private: [u8; 0],
}

#[repr(C)]
pub struct MmStruct {
    pub pgd: *mut c_void,
}

#[repr(C)]
pub struct Dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FileOperations {
    _private: [u8; 0],
}

extern "C" {
    static mut init_mm: MmStruct;
    static mut current: *mut TaskStruct;
    #[cfg(all(feature = "CONFIG_EFI", target_arch = "x86_64"))]
    static mut efi_mm: MmStruct;

    fn ptdump_walk_pgd_level_debugfs(
        m: *mut SeqFile,
        mm: *mut MmStruct,
        user: bool,
    );
    fn debugfs_create_dir(name: *const u8, parent: *mut Dentry) -> *mut Dentry;
    fn debugfs_create_file(
        name: *const u8,
        mode: u32,
        parent: *mut Dentry,
        data: *mut c_void,
        fops: *const FileOperations,
    ) -> *mut Dentry;
    fn debugfs_remove_recursive(dir: *mut Dentry);
}

#[repr(C)]
pub struct TaskStruct {
    pub mm: *mut MmStruct,
}

extern "C" {
    static ptdump_fops: FileOperations;
    static ptdump_curknl_fops: FileOperations;
    #[cfg(feature = "CONFIG_MITIGATION_PAGE_TABLE_ISOLATION")]
    static ptdump_curusr_fops: FileOperations;
    #[cfg(all(feature = "CONFIG_EFI", target_arch = "x86_64"))]
    static ptdump_efi_fops: FileOperations;
}

unsafe extern "C" fn ptdump_show(m: *mut SeqFile, _v: *mut c_void) -> i32 {
    ptdump_walk_pgd_level_debugfs(m, &mut init_mm, false);
    0
}

// Equivalent of DEFINE_SHOW_ATTRIBUTE(ptdump).

unsafe extern "C" fn ptdump_curknl_show(m: *mut SeqFile, _v: *mut c_void) -> i32 {
    if !(*(*current).mm).pgd.is_null() {
        ptdump_walk_pgd_level_debugfs(m, (*current).mm, false);
    }
    0
}

// Equivalent of DEFINE_SHOW_ATTRIBUTE(ptdump_curknl).

#[cfg(feature = "CONFIG_MITIGATION_PAGE_TABLE_ISOLATION")]
unsafe extern "C" fn ptdump_curusr_show(m: *mut SeqFile, _v: *mut c_void) -> i32 {
    if !(*(*current).mm).pgd.is_null() {
        ptdump_walk_pgd_level_debugfs(m, (*current).mm, true);
    }
    0
}

// Equivalent of DEFINE_SHOW_ATTRIBUTE(ptdump_curusr).

#[cfg(all(feature = "CONFIG_EFI", target_arch = "x86_64"))]
unsafe extern "C" fn ptdump_efi_show(m: *mut SeqFile, _v: *mut c_void) -> i32 {
    if !efi_mm.pgd.is_null() {
        ptdump_walk_pgd_level_debugfs(m, &mut efi_mm, false);
    }
    0
}

// Equivalent of DEFINE_SHOW_ATTRIBUTE(ptdump_efi).

static mut dir: *mut Dentry = core::ptr::null_mut();

unsafe extern "C" fn pt_dump_debug_init() -> i32 {
    dir = debugfs_create_dir(b"page_tables\0".as_ptr(), core::ptr::null_mut());

    debugfs_create_file(
        b"kernel\0".as_ptr(),
        0o400,
        dir,
        core::ptr::null_mut(),
        &ptdump_fops,
    );
    debugfs_create_file(
        b"current_kernel\0".as_ptr(),
        0o400,
        dir,
        core::ptr::null_mut(),
        &ptdump_curknl_fops,
    );

    #[cfg(feature = "CONFIG_MITIGATION_PAGE_TABLE_ISOLATION")]
    debugfs_create_file(
        b"current_user\0".as_ptr(),
        0o400,
        dir,
        core::ptr::null_mut(),
        &ptdump_curusr_fops,
    );
    #[cfg(all(feature = "CONFIG_EFI", target_arch = "x86_64"))]
    debugfs_create_file(
        b"efi\0".as_ptr(),
        0o400,
        dir,
        core::ptr::null_mut(),
        &ptdump_efi_fops,
    );
    0
}

unsafe extern "C" fn pt_dump_debug_exit() {
    debugfs_remove_recursive(dir);
}

// Equivalent of module_init(pt_dump_debug_init).
// Equivalent of module_exit(pt_dump_debug_exit).
// MODULE_AUTHOR("Arjan van de Ven <arjan@linux.intel.com>");
// MODULE_DESCRIPTION("Kernel debugging helper that dumps pagetables");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
