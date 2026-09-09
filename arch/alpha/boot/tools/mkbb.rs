// SPDX-License-Identifier: GPL-2.0
/* This utility makes a bootblock suitable for the SRM console/miniloader */

/* Usage:
 *	mkbb <device> <lxboot>
 *
 * Where <device> is the name of the device to install the bootblock on,
 * and <lxboot> is the name of a bootblock to merge in.  This bootblock
 * contains the offset and size of the bootloader.  It must be exactly
 * 512 bytes long.
 */

use std::ffi::CStr;

const O_RDONLY: i32 = 0;
const O_RDWR: i32 = 2;
const SEEK_SET: i32 = 0;
const MAXPARTITIONS: usize = 8;

#[repr(C)]
#[derive(Copy, Clone)]
struct DPartition {
    p_size: u32,
    p_offset: u32,
    p_fsize: u32,
    p_fstype: u8,
    p_frag: u8,
    p_cpg: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Disklabel {
    d_magic: u32,
    d_type: u16,
    d_subtype: u16,
    d_typename: [u8; 16],
    d_packname: [u8; 16],
    d_secsize: u32,
    d_nsectors: u32,
    d_ntracks: u32,
    d_ncylinders: u32,
    d_secpercyl: u32,
    d_secprtunit: u32,
    d_sparespertrack: u16,
    d_sparespercyl: u16,
    d_acylinders: u32,
    d_rpm: u16,
    d_interleave: u16,
    d_trackskew: u16,
    d_cylskew: u16,
    d_headswitch: u32,
    d_trkseek: u32,
    d_flags: u32,
    d_drivedata: [u32; 5],
    d_spare: [u32; 5],
    d_magic2: u32,
    d_checksum: u16,
    d_npartitions: u16,
    d_bbsize: u32,
    d_sbsize: u32,
    d_partitions: [DPartition; MAXPARTITIONS],
}

#[repr(C)]
struct BootblockU1 {
    __pad1: [i8; 64],
    __label: Disklabel,
}

#[repr(C)]
struct BootblockU2 {
    __pad2: [c_ulong; 63],
    __checksum: c_ulong,
}

type CLong = std::ffi::c_long;
type c_ulong = std::ffi::c_ulong;

#[repr(C)]
union Bootblock {
    __u1: std::mem::ManuallyDrop<BootblockU1>,
    __u2: std::mem::ManuallyDrop<BootblockU2>,
    bootblock_bytes: [i8; 512],
    bootblock_quadwords: [c_ulong; 64],
}

unsafe extern "C" {
    fn open(path: *const i8, flags: i32, ...) -> i32;
    fn close(fd: i32) -> i32;
    fn read(fd: i32, buf: *mut std::ffi::c_void, count: usize) -> isize;
    fn write(fd: i32, buf: *const std::ffi::c_void, count: usize) -> isize;
    fn lseek(fd: i32, offset: CLong, whence: i32) -> CLong;
    fn perror(s: *const i8);
    fn fprintf(stream: *mut std::ffi::c_void, format: *const i8, ...);
    fn exit(status: i32) -> !;
}

unsafe fn bootblock_label(b: *mut Bootblock) -> *mut Disklabel {
    &mut (*std::ptr::addr_of_mut!((*b).__u1)).__label
}

unsafe fn bootblock_checksum(b: *mut Bootblock) -> *mut c_ulong {
    &mut (*std::ptr::addr_of_mut!((*b).__u2)).__checksum
}

unsafe fn c_arg(args: &[*mut i8], index: usize) -> *const i8 {
    args[index]
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: i32, argv: *mut *mut i8) -> i32 {
    let args = std::slice::from_raw_parts(argv, argc as usize);
    let mut bootblock_from_disk: Bootblock = std::mem::zeroed();
    let mut bootloader_image: Bootblock = std::mem::zeroed();
    let mut dev: i32;
    let mut fd: i32;
    let mut nread: isize;

    if argc != 3 {
        fprintf(std::ptr::null_mut(), b"Usage: %s device lxboot\n\0".as_ptr() as *const i8, args[0]);
        exit(0);
    }

    dev = open(c_arg(args, 1), O_RDWR);
    if dev < 0 {
        perror(c_arg(args, 1));
        exit(0);
    }

    fd = open(c_arg(args, 2), O_RDONLY);
    if fd < 0 {
        perror(c_arg(args, 2));
        close(dev);
        exit(0);
    }

    nread = read(fd, &mut bootloader_image as *mut _ as *mut std::ffi::c_void, std::mem::size_of::<Bootblock>());
    if nread != std::mem::size_of::<Bootblock>() as isize {
        perror(b"lxboot read\0".as_ptr() as *const i8);
        exit(0);
    }

    nread = read(dev, &mut bootblock_from_disk as *mut _ as *mut std::ffi::c_void, std::mem::size_of::<Bootblock>());
    if nread != std::mem::size_of::<Bootblock>() as isize {
        perror(b"bootblock read\0".as_ptr() as *const i8);
        exit(0);
    }

    *bootblock_label(&mut bootloader_image) = *bootblock_label(&mut bootblock_from_disk);
    *bootblock_checksum(&mut bootloader_image) = 0;
    for i in 0..63 {
        *bootblock_checksum(&mut bootloader_image) = (*bootblock_checksum(&mut bootloader_image))
            .wrapping_add(bootloader_image.bootblock_quadwords[i]);
    }

    lseek(dev, 0, SEEK_SET);
    if write(dev, &bootloader_image as *const _ as *const std::ffi::c_void, std::mem::size_of::<Bootblock>())
        != std::mem::size_of::<Bootblock>() as isize
    {
        perror(b"bootblock write\0".as_ptr() as *const i8);
        exit(0);
    }

    close(fd);
    close(dev);
    exit(0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
