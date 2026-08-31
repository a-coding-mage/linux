/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

pub fn ptr_to_u64<T>(ptr: *const T) -> __u64 {
    ptr as usize as __u64
}

/*
 * Original C condition:
 *   #ifdef HAVE_LIBBPF_SUPPORT
 *
 * Original dependencies:
 *   #include <bpf/libbpf.h>
 *   #include <bpf/libbpf_version.h>
 */

/*
 * Original macro:
 *   #define LIBBPF_CURRENT_VERSION_GEQ(major, minor) \
 *      (LIBBPF_MAJOR_VERSION > (major) || \
 *       (LIBBPF_MAJOR_VERSION == (major) && LIBBPF_MINOR_VERSION >= (minor)))
 */
pub const fn LIBBPF_CURRENT_VERSION_GEQ(major: u32, minor: u32) -> bool {
    LIBBPF_MAJOR_VERSION > major
        || (LIBBPF_MAJOR_VERSION == major && LIBBPF_MINOR_VERSION >= minor)
}

/*
 * Original C condition:
 *   #if LIBBPF_CURRENT_VERSION_GEQ(1, 7)
 *
 * libbpf 1.7+ support the btf_dump_type_data_opts.emit_strings option.
 */
pub const HAVE_LIBBPF_STRINGS_SUPPORT: u32 = 1;

/*
 * Get bpf_prog_info in continuous memory
 *
 * struct bpf_prog_info has multiple arrays. The user has option to choose
 * arrays to fetch from kernel. The following APIs provide an uniform way to
 * fetch these data. All arrays in bpf_prog_info are stored in a single
 * continuous memory region. This makes it easy to store the info in a
 * file.
 *
 * Before writing perf_bpil to files, it is necessary to
 * translate pointers in bpf_prog_info to offsets. Helper functions
 * bpil_addr_to_offs() and bpil_offs_to_addr()
 * are introduced to switch between pointers and offsets.
 *
 * Examples:
 *   # To fetch map_ids and prog_tags:
 *   __u64 arrays = (1UL << PERF_BPIL_MAP_IDS) |
 *           (1UL << PERF_BPIL_PROG_TAGS);
 *   struct perf_bpil *info_linear =
 *           get_bpf_prog_info_linear(fd, arrays);
 *
 *   # To save data in file
 *   bpil_addr_to_offs(info_linear);
 *   write(f, info_linear, sizeof(*info_linear) + info_linear->data_len);
 *
 *   # To read data from file
 *   read(f, info_linear, <proper_size>);
 *   bpil_offs_to_addr(info_linear);
 */
pub type perf_bpil_array_types = ::core::ffi::c_uint;

pub const PERF_BPIL_FIRST_ARRAY: perf_bpil_array_types = 0;
pub const PERF_BPIL_JITED_INSNS: perf_bpil_array_types = 0;
pub const PERF_BPIL_XLATED_INSNS: perf_bpil_array_types = 1;
pub const PERF_BPIL_MAP_IDS: perf_bpil_array_types = 2;
pub const PERF_BPIL_JITED_KSYMS: perf_bpil_array_types = 3;
pub const PERF_BPIL_JITED_FUNC_LENS: perf_bpil_array_types = 4;
pub const PERF_BPIL_FUNC_INFO: perf_bpil_array_types = 5;
pub const PERF_BPIL_LINE_INFO: perf_bpil_array_types = 6;
pub const PERF_BPIL_JITED_LINE_INFO: perf_bpil_array_types = 7;
pub const PERF_BPIL_PROG_TAGS: perf_bpil_array_types = 8;
pub const PERF_BPIL_LAST_ARRAY: perf_bpil_array_types = 9;

#[repr(C)]
pub struct perf_bpil {
    /* size of struct bpf_prog_info, when the tool is compiled */
    pub info_len: __u32,
    /* total bytes allocated for data, round up to 8 bytes */
    pub data_len: __u32,
    /* which arrays are included in data */
    pub arrays: __u64,
    pub info: bpf_prog_info,
    pub data: [__u8; 0],
}

unsafe extern "C" {
    pub fn get_bpf_prog_info_linear(
        fd: ::core::ffi::c_int,
        arrays: __u64,
    ) -> *mut perf_bpil;

    pub fn bpil_addr_to_offs(info_linear: *mut perf_bpil);

    pub fn bpil_offs_to_addr(info_linear: *mut perf_bpil);
}
