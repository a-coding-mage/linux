/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

/* Original C header included <linux/types.h> for __u16 and __u32. */
pub type __u16 = u16;
pub type __u32 = u32;

/* Represents an open zip archive.
 * Only basic ZIP files are supported, in particular the following are not
 * supported:
 * - encryption
 * - streaming
 * - multi-part ZIP files
 * - ZIP64
 */
#[repr(C)]
pub struct zip_archive {
    _private: [u8; 0],
}

/* Carries information on name, compression method, and data corresponding to a
 * file in a zip archive.
 */
#[repr(C)]
pub struct zip_entry {
    /* Compression method as defined in pkzip spec. 0 means data is uncompressed. */
    pub compression: __u16,

    /* Non-null terminated name of the file. */
    pub name: *const ::std::os::raw::c_char,
    /* Length of the file name. */
    pub name_length: __u16,

    /* Pointer to the file data. */
    pub data: *const ::std::os::raw::c_void,
    /* Length of the file data. */
    pub data_length: __u32,
    /* Offset of the file data within the archive. */
    pub data_offset: __u32,
}

unsafe extern "C" {
    /* Open a zip archive. Returns NULL in case of an error. */
    pub fn zip_archive_open(path: *const ::std::os::raw::c_char) -> *mut zip_archive;

    /* Close a zip archive and release resources. */
    pub fn zip_archive_close(archive: *mut zip_archive);

    /* Look up an entry corresponding to a file in given zip archive. */
    pub fn zip_archive_find_entry(
        archive: *mut zip_archive,
        name: *const ::std::os::raw::c_char,
        out: *mut zip_entry,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
