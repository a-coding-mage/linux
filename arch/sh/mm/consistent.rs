/*
 * Copyright (C) 2004 - 2007  Paul Mundt
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    static mut boot_command_line: *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn memparse(ptr: *const c_char, retptr: *mut *mut c_char) -> c_ulong;
    fn dma_alloc_coherent(
        dev: *mut device,
        size: c_ulong,
        dma_handle: *mut dma_addr_t,
        flags: c_ulong,
    ) -> *mut c_void;
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
}

type dma_addr_t = c_ulong;

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct resource {
    start: dma_addr_t,
    end: dma_addr_t,
    name: *mut c_char,
    flags: c_ulong,
}

#[repr(C)]
struct platform_device {
    _private: [u8; 0],
}

// The following field access mirrors the kernel's platform_device layout.
#[repr(C)]
struct platform_device_layout {
    _dev: device,
    _name: *mut c_char,
    _id: c_int,
    _num_resources: c_int,
    resource: *mut resource,
    num_resources: c_int,
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const IORESOURCE_MEM: c_ulong = 0x0000_0200;
const GFP_KERNEL: c_ulong = 0x0000_00d0;

unsafe fn memchunk_setup(_str: *mut c_char) -> c_int {
    1 /* accept anything that begins with "memchunk." */
}

// __setup("memchunk.", memchunk_setup);

unsafe fn memchunk_cmdline_override(name: *mut c_char, sizep: *mut c_ulong) {
    let mut p = boot_command_line;
    let k = strlen(name);

    loop {
        p = strstr(p, b"memchunk.\0".as_ptr() as *const c_char);
        if p.is_null() {
            break;
        }
        p = p.add(9); /* strlen("memchunk.") */
        if strncmp(name, p, k) == 0 && *p.add(k) as u8 == b'=' {
            p = p.add(k + 1);
            *sizep = memparse(p, core::ptr::null_mut());
            pr_info(
                b"%s: forcing memory chunk size to 0x%08lx\n\0".as_ptr() as *const c_char,
                name,
                *sizep,
            );
            break;
        }
    }
}

pub unsafe fn platform_resource_setup_memory(
    pdev: *mut platform_device,
    name: *mut c_char,
    mut memsize: c_ulong,
) -> c_int {
    let pdev = pdev as *mut platform_device_layout;
    let mut dma_handle: dma_addr_t = 0;
    let r: *mut resource = (*pdev).resource.add((*pdev).num_resources as usize - 1);
    let buf: *mut c_void;

    if (*r).flags != 0 {
        pr_warn(
            b"%s: unable to find empty space for resource\n\0".as_ptr() as *const c_char,
            name,
        );
        return -EINVAL;
    }

    memchunk_cmdline_override(name, &mut memsize);
    if memsize == 0 {
        return 0;
    }

    buf = dma_alloc_coherent(
        &mut (*pdev)._dev,
        memsize,
        &mut dma_handle,
        GFP_KERNEL,
    );
    if buf.is_null() {
        pr_warn(
            b"%s: unable to allocate memory\n\0".as_ptr() as *const c_char,
            name,
        );
        return -ENOMEM;
    }

    (*r).flags = IORESOURCE_MEM;
    (*r).start = dma_handle;
    (*r).end = (*r).start + memsize - 1;
    (*r).name = name;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
