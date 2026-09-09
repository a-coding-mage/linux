/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation:
// #include <linux/fs_parser.h> // bleh...

extern "C" {
    pub fn ramfs_get_inode(
        sb: *mut super_block,
        dir: *const inode,
        mode: umode_t,
        dev: dev_t,
    ) -> *mut inode;

    pub fn ramfs_init_fs_context(fc: *mut fs_context) -> ::core::ffi::c_int;
    pub fn ramfs_kill_sb(sb: *mut super_block);

    // CONFIG_MMU selects the inline definition below; otherwise this is an
    // externally provided function declaration.
    #[cfg(not(CONFIG_MMU))]
    pub fn ramfs_nommu_expand_for_mapping(
        inode: *mut inode,
        newsize: usize,
    ) -> ::core::ffi::c_int;

    pub static ramfs_fs_parameters: [fs_parameter_spec];
    pub static ramfs_file_operations: file_operations;
    pub static generic_file_vm_ops: vm_operations_struct;
}

#[cfg(CONFIG_MMU)]
#[inline]
pub unsafe fn ramfs_nommu_expand_for_mapping(
    _inode: *mut inode,
    _newsize: usize,
) -> ::core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
