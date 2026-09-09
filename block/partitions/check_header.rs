/* SPDX-License-Identifier: GPL-2.0 */
// C dependencies supplied by the surrounding kernel translation unit:
// linux/pagemap.h, linux/blkdev.h, linux/seq_buf.h, and ../blk.h

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct gendisk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct folio {
    _private: [u8; 0],
}

#[repr(C)]
pub struct partition_meta_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_buf {
    _private: [u8; 0],
}

pub type sector_t = u64;

pub const BDEVNAME_SIZE: usize = 32;

#[repr(C)]
pub struct parsed_partitions_part {
    pub from: sector_t,
    pub size: sector_t,
    pub flags: c_int,
    pub has_info: bool,
    pub info: partition_meta_info,
}

#[repr(C)]
pub struct parsed_partitions {
    pub disk: *mut gendisk,
    pub name: [c_char; BDEVNAME_SIZE],
    pub parts: *mut parsed_partitions_part,
    pub next: c_int,
    pub limit: c_int,
    pub access_beyond_eod: bool,
    pub pp_buf: seq_buf,
}

#[repr(C)]
pub struct Sector {
    pub v: *mut folio,
}

extern "C" {
    pub fn read_part_sector(
        state: *mut parsed_partitions,
        n: sector_t,
        p: *mut Sector,
    ) -> *mut c_void;

    pub fn folio_put(v: *mut folio);
    pub fn seq_buf_printf(buf: *mut seq_buf, fmt: *const c_char, ...);

    pub fn adfspart_check_ADFS(state: *mut parsed_partitions) -> c_int;
    pub fn adfspart_check_CUMANA(state: *mut parsed_partitions) -> c_int;
    pub fn adfspart_check_EESOX(state: *mut parsed_partitions) -> c_int;
    pub fn adfspart_check_ICS(state: *mut parsed_partitions) -> c_int;
    pub fn adfspart_check_POWERTEC(state: *mut parsed_partitions) -> c_int;
    pub fn aix_partition(state: *mut parsed_partitions) -> c_int;
    pub fn amiga_partition(state: *mut parsed_partitions) -> c_int;
    pub fn atari_partition(state: *mut parsed_partitions) -> c_int;
    pub fn cmdline_partition(state: *mut parsed_partitions) -> c_int;
    pub fn efi_partition(state: *mut parsed_partitions) -> c_int;
    pub fn ibm_partition(state: *mut parsed_partitions) -> c_int;
    pub fn karma_partition(state: *mut parsed_partitions) -> c_int;
    pub fn ldm_partition(state: *mut parsed_partitions) -> c_int;
    pub fn mac_partition(state: *mut parsed_partitions) -> c_int;
    pub fn msdos_partition(state: *mut parsed_partitions) -> c_int;
    pub fn of_partition(state: *mut parsed_partitions) -> c_int;
    pub fn osf_partition(state: *mut parsed_partitions) -> c_int;
    pub fn sgi_partition(state: *mut parsed_partitions) -> c_int;
    pub fn sun_partition(state: *mut parsed_partitions) -> c_int;
    pub fn sysv68_partition(state: *mut parsed_partitions) -> c_int;
    pub fn ultrix_partition(state: *mut parsed_partitions) -> c_int;
}

#[inline]
pub unsafe fn put_dev_sector(p: Sector) {
    folio_put(p.v);
}

#[inline]
pub unsafe fn put_partition(
    p: *mut parsed_partitions,
    n: c_int,
    from: sector_t,
    size: sector_t,
) {
    if n < (*p).limit {
        let part = (*p).parts.add(n as usize);
        (*part).from = from;
        (*part).size = size;
        seq_buf_printf(
            &mut (*p).pp_buf,
            b" %s%d\0".as_ptr() as *const c_char,
            (*p).name.as_ptr(),
            n,
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
