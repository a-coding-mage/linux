// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * BCM947xx nvram variable access
 *
 * Copyright (C) 2005 Broadcom Corporation
 * Copyright (C) 2006 Felix Fietkau <nbd@openwrt.org>
 * Copyright (C) 2010-2012 Hauke Mehrtens <hauke@hauke-m.de>
 */

// Linux dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_void};

const NVRAM_MAGIC: u32 = 0x48534C46;
const NVRAM_SPACE: usize = 0x10000;
const NVRAM_MAX_GPIO_ENTRIES: c_int = 32;
const NVRAM_MAX_GPIO_VALUE_LEN: usize = 30;
const FLASH_MIN: usize = 0x00020000;

#[repr(C)]
struct nvram_header {
    magic: u32,
    len: u32,
    crc_ver_init: u32,
    config_refresh: u32,
    config_ncdl: u32,
}

static mut nvram_buf: [c_char; NVRAM_SPACE] = [0; NVRAM_SPACE];
static mut nvram_len: usize = 0;
static nvram_sizes: [usize; 4] = [0x6000, 0x8000, 0xF000, 0x10000];

extern "C" {
    fn __ioread32_copy(to: *mut c_void, from: *const c_void, count: usize);
    fn ioremap(base: u32, size: u32) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn vmalloc(size: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strchr(s: *mut c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
}

unsafe fn bcm47xx_nvram_is_valid(nvram: *const c_void) -> bool {
    (*(nvram as *const nvram_header)).magic == NVRAM_MAGIC
}

unsafe fn bcm47xx_nvram_copy(nvram_start: *const c_void, res_size: usize) {
    let header = nvram_start as *const nvram_header;
    let mut copy_size = (*header).len as usize;
    if copy_size > res_size {
        pr_err(b"The nvram size according to the header seems to be bigger than the partition on flash\0".as_ptr() as *const c_char);
        copy_size = res_size;
    }
    if copy_size >= NVRAM_SPACE {
        pr_err(b"nvram on flash (%zu bytes) is bigger than the reserved space in memory, will just copy the first %i bytes\0".as_ptr() as *const c_char, copy_size, NVRAM_SPACE - 1);
        copy_size = NVRAM_SPACE - 1;
    }
    __ioread32_copy(nvram_buf.as_mut_ptr() as *mut c_void, (nvram_start as *const u8).add(0) as *const c_void, (copy_size + 3) / 4);
    nvram_buf[NVRAM_SPACE - 1] = 0;
    nvram_len = copy_size;
}

unsafe fn bcm47xx_nvram_find_and_copy(flash_start: *const c_void, res_size: usize) -> c_int {
    if nvram_len != 0 {
        pr_warn(b"nvram already initialized\n\0".as_ptr() as *const c_char);
        return -17;
    }
    let mut flash_size = FLASH_MIN;
    while flash_size <= res_size {
        for size in nvram_sizes.iter() {
            let offset = flash_size - *size;
            if bcm47xx_nvram_is_valid((flash_start as *const u8).add(offset) as *const c_void) {
                bcm47xx_nvram_copy((flash_start as *const u8).add(offset) as *const c_void, res_size - offset);
                return 0;
            }
        }
        flash_size <<= 1;
    }
    for offset in [4096usize, 1024usize] {
        if bcm47xx_nvram_is_valid((flash_start as *const u8).add(offset) as *const c_void) {
            bcm47xx_nvram_copy((flash_start as *const u8).add(offset) as *const c_void, res_size - offset);
            return 0;
        }
    }
    pr_err(b"no nvram found\n\0".as_ptr() as *const c_char);
    -6
}

#[no_mangle]
pub unsafe extern "C" fn bcm47xx_nvram_init_from_iomem(nvram_start: *const c_void, res_size: usize) -> c_int {
    if nvram_len != 0 { pr_warn(b"nvram already initialized\n\0".as_ptr() as *const c_char); return -17; }
    if !bcm47xx_nvram_is_valid(nvram_start) { pr_err(b"No valid NVRAM found\0".as_ptr() as *const c_char); return -2; }
    bcm47xx_nvram_copy(nvram_start, res_size); 0
}

#[no_mangle]
pub unsafe extern "C" fn bcm47xx_nvram_init_from_mem(base: u32, lim: u32) -> c_int {
    let iobase = ioremap(base, lim);
    if iobase.is_null() { return -12; }
    let err = bcm47xx_nvram_find_and_copy(iobase, lim as usize);
    iounmap(iobase);
    err
}

unsafe fn nvram_init() -> c_int {
    // CONFIG_MTD implementation is supplied by the kernel build configuration.
    -6
}

#[no_mangle]
pub unsafe extern "C" fn bcm47xx_nvram_getenv(name: *const c_char, val: *mut c_char, val_len: usize) -> c_int {
    if name.is_null() { return -22; }
    if nvram_len == 0 { let err = nvram_init(); if err != 0 { return err; } }
    let mut var = nvram_buf.as_mut_ptr().add(core::mem::size_of::<nvram_header>());
    let end = nvram_buf.as_mut_ptr().add(NVRAM_SPACE);
    while var < end && *var != 0 {
        let eq = strchr(var, '=' as c_int);
        if eq.is_null() { break; }
        let value = eq.add(1);
        if (eq.offset_from(var) as usize) == strlen(name) && strncmp(var, name, eq.offset_from(var) as usize) == 0 {
            return snprintf(val, val_len, b"%s\0".as_ptr() as *const c_char, value);
        }
        var = value.add(strlen(value));
        var = var.add(1);
    }
    -2
}

#[no_mangle]
pub unsafe extern "C" fn bcm47xx_nvram_gpio_pin(name: *const c_char) -> c_int {
    let mut nvram_var = [0 as c_char; 7];
    let mut buf = [0 as c_char; NVRAM_MAX_GPIO_VALUE_LEN];
    for i in 0..NVRAM_MAX_GPIO_ENTRIES {
        let err = snprintf(nvram_var.as_mut_ptr(), nvram_var.len(), b"gpio%i\0".as_ptr() as *const c_char, i);
        if err <= 0 { continue; }
        let err = bcm47xx_nvram_getenv(nvram_var.as_ptr(), buf.as_mut_ptr(), buf.len());
        if err <= 0 { continue; }
        if strcmp(name, buf.as_ptr()) == 0 { return i; }
    }
    -2
}

#[no_mangle]
pub unsafe extern "C" fn bcm47xx_nvram_get_contents(nvram_size: *mut usize) -> *mut c_char {
    if nvram_len == 0 { let err = nvram_init(); if err != 0 { return core::ptr::null_mut(); } }
    *nvram_size = nvram_len - core::mem::size_of::<nvram_header>();
    let nvram = vmalloc(*nvram_size) as *mut c_char;
    if nvram.is_null() { return core::ptr::null_mut(); }
    memcpy(nvram as *mut c_void, nvram_buf.as_ptr().add(core::mem::size_of::<nvram_header>()) as *const c_void, *nvram_size);
    nvram
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
