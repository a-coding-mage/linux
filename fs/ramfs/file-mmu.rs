/* file-mmu.rs: ramfs MMU-based file operations
 *
 * Resizable simple ram filesystem for Linux.
 *
 * Copyright (C) 2000 Linus Torvalds.
 *               2000 Transmeta Corp.
 *
 * Usage limits added by David Gibson, Linuxcare Australia.
 * This file is released under the GPL.
 */

/*
 * NOTE! This filesystem is probably most useful
 * not as a real filesystem, but as an example of
 * how virtual filesystems can be written.
 *
 * It doesn't get much simpler than this. Consider
 * that this file implements the full semantics of
 * a POSIX-compliant read-write filesystem.
 *
 * Note in particular how the filesystem does not
 * need to implement any data structures of its own
 * to keep track of the virtual data: using the VFS
 * caches is sufficient.
 */

// The declarations below are supplied by the Linux filesystem/VFS dependencies.

extern "C" {
    fn mm_get_unmapped_area(
        file: *mut file,
        addr: ::core::ffi::c_ulong,
        len: ::core::ffi::c_ulong,
        pgoff: ::core::ffi::c_ulong,
        flags: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong;

    fn generic_file_read_iter(file: *mut file, iocb: *mut kiocb, iter: *mut iov_iter) -> isize;
    fn generic_file_write_iter(file: *mut file, iocb: *mut kiocb, iter: *mut iov_iter) -> isize;
    fn generic_file_mmap_prepare(vma: *mut vm_area_struct, file: *mut file) -> ::core::ffi::c_int;
    fn noop_fsync(file: *mut file, start: ::core::ffi::c_long, end: ::core::ffi::c_long, datasync: bool) -> ::core::ffi::c_int;
    fn filemap_splice_read(pipe: *mut pipe_inode_info, out: *mut splice_desc, flags: ::core::ffi::c_uint) -> isize;
    fn iter_file_splice_write(pipe: *mut pipe_inode_info, out: *mut splice_desc, flags: ::core::ffi::c_uint) -> isize;
    fn generic_file_llseek(file: *mut file, offset: ::core::ffi::c_long, whence: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn simple_setattr(id: *mut inode, attr: *mut iattr) -> ::core::ffi::c_int;
    fn simple_getattr(mnt: *mut vfsmount, dentry: *mut dentry, stat: *mut kstat) -> ::core::ffi::c_int;
}

unsafe fn ramfs_mmu_get_unmapped_area(
    file: *mut file,
    addr: ::core::ffi::c_ulong,
    len: ::core::ffi::c_ulong,
    pgoff: ::core::ffi::c_ulong,
    flags: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    mm_get_unmapped_area(file, addr, len, pgoff, flags)
}

pub static ramfs_file_operations: file_operations = file_operations {
    read_iter: Some(generic_file_read_iter),
    write_iter: Some(generic_file_write_iter),
    mmap_prepare: Some(generic_file_mmap_prepare),
    fsync: Some(noop_fsync),
    splice_read: Some(filemap_splice_read),
    splice_write: Some(iter_file_splice_write),
    llseek: Some(generic_file_llseek),
    get_unmapped_area: Some(ramfs_mmu_get_unmapped_area),
};

pub static ramfs_file_inode_operations: inode_operations = inode_operations {
    setattr: Some(simple_setattr),
    getattr: Some(simple_getattr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
