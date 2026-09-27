// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// External declarations supplied by the surrounding UML kernel.

static mut physmem_fd: ::kernel::ffi::c_int = -1;

/* Changed during early boot */
pub static mut high_physmem: ::kernel::ffi::c_ulong = 0;

extern "C" {
    static mut physmem_size: ::kernel::ffi::c_ulong;
    static mut min_low_pfn: ::kernel::ffi::c_ulong;
    static mut max_low_pfn: ::kernel::ffi::c_ulong;

    fn os_map_memory(
        virt: *mut ::kernel::ffi::c_void,
        fd: ::kernel::ffi::c_int,
        offset: u64,
        len: ::kernel::ffi::c_ulong,
        r: ::kernel::ffi::c_int,
        w: ::kernel::ffi::c_int,
        x: ::kernel::ffi::c_int,
    ) -> ::kernel::ffi::c_int;
    fn printk(fmt: *const ::kernel::ffi::c_char, ...);
    fn panic(fmt: *const ::kernel::ffi::c_char, ... ) -> !;
    fn os_warn(fmt: *const ::kernel::ffi::c_char, ...);
    fn exit(status: ::kernel::ffi::c_int) -> !;
    fn create_mem_file(len: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
    fn os_seek_file(fd: ::kernel::ffi::c_int, offset: ::kernel::ffi::c_ulong);
    fn os_write_file(
        fd: ::kernel::ffi::c_int,
        buf: *const ::kernel::ffi::c_void,
        len: ::kernel::ffi::c_ulong,
    );
    fn memparse(
        ptr: *const ::kernel::ffi::c_char,
        retptr: *mut *mut ::kernel::ffi::c_char,
    ) -> ::kernel::ffi::c_ulong;
    fn memblock_add(base: ::kernel::ffi::c_ulong, size: ::kernel::ffi::c_ulong);
    fn memblock_reserve(base: ::kernel::ffi::c_ulong, size: ::kernel::ffi::c_ulong);
    fn __pa(addr: *const ::kernel::ffi::c_void) -> ::kernel::ffi::c_ulong;
    static __syscall_stub_start: ::kernel::ffi::c_uchar;
}

const PAGE_SIZE: ::kernel::ffi::c_ulong = 4096;
const PAGE_SHIFT: u32 = 12;

pub unsafe fn map_memory(
    virt: ::kernel::ffi::c_ulong,
    phys: ::kernel::ffi::c_ulong,
    len: ::kernel::ffi::c_ulong,
    r: ::kernel::ffi::c_int,
    w: ::kernel::ffi::c_int,
    x: ::kernel::ffi::c_int,
) {
    let mut offset: u64 = 0;
    let fd = phys_mapping(phys, &mut offset);
    let err = os_map_memory(virt as *mut ::kernel::ffi::c_void, fd, offset, len, r, w, x);
    if err != 0 {
        if err == -12 {
            printk(b"try increasing the host's /proc/sys/vm/max_map_count to <physical memory size>/4096\n\0".as_ptr() as *const _);
        }
        panic(
            b"map_memory(0x%lx, %d, 0x%llx, %ld, %d, %d, %d) failed, err = %d\n\0".as_ptr() as *const _,
            virt, fd, offset, len, r, w, x, err,
        );
    }
}

/**
 * setup_physmem() - Setup physical memory for UML
 * @start: Start address of the physical kernel memory, i.e start address of the executable image.
 * @reserve_end: end address of the physical kernel memory.
 * @len: Length of total physical memory that should be mapped/made available, in bytes.
 */
pub unsafe fn setup_physmem(
    start: ::kernel::ffi::c_ulong,
    reserve_end: ::kernel::ffi::c_ulong,
    len: ::kernel::ffi::c_ulong,
) {
    let reserve = reserve_end - start;
    let map_size = len - reserve;

    if len <= reserve {
        os_warn(b"Too few physical memory! Needed=%lu, given=%lu\n\0".as_ptr() as *const _, reserve, len);
        exit(1);
    }

    physmem_fd = create_mem_file(len);
    let err = os_map_memory(reserve_end as *mut ::kernel::ffi::c_void, physmem_fd, reserve, map_size, 1, 1, 1);
    if err < 0 {
        os_warn(
            b"setup_physmem - mapping %lu bytes of memory at 0x%p failed - errno = %d\n\0".as_ptr() as *const _,
            map_size, reserve_end as *mut ::kernel::ffi::c_void, err,
        );
        exit(1);
    }

    /*
     * Special kludge - This page will be mapped in to userspace processes
     * from physmem_fd, so it needs to be written out there.
     */
    os_seek_file(physmem_fd, __pa(&__syscall_stub_start));
    os_write_file(physmem_fd, &__syscall_stub_start as *const _ as *const ::kernel::ffi::c_void, PAGE_SIZE);

    memblock_add(__pa(start as *const ::kernel::ffi::c_void), len);
    memblock_reserve(__pa(start as *const ::kernel::ffi::c_void), reserve);

    min_low_pfn = ( __pa(reserve_end as *const ::kernel::ffi::c_void) + PAGE_SIZE - 1 ) >> PAGE_SHIFT;
    max_low_pfn = min_low_pfn + (map_size >> PAGE_SHIFT);
}

pub unsafe fn phys_mapping(phys: ::kernel::ffi::c_ulong, offset_out: *mut u64) -> ::kernel::ffi::c_int {
    let mut fd = -1;
    if phys < physmem_size {
        fd = physmem_fd;
        *offset_out = phys as u64;
    }
    fd
}

unsafe fn uml_mem_setup(line: *mut ::kernel::ffi::c_char, add: *mut ::kernel::ffi::c_int) -> ::kernel::ffi::c_int {
    let mut retptr: *mut ::kernel::ffi::c_char = core::ptr::null_mut();
    *add = 0;
    physmem_size = memparse(line, &mut retptr);
    0
}

// __uml_setup("mem=", uml_mem_setup, "mem=<Amount of desired ram>\n...")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
