// SPDX-License-Identifier: GPL-2.0
/*
 * kexec_file for arm64
 *
 * Copyright (C) 2018 Linaro Limited
 * Author: AKASHI Takahiro <takahiro.akashi@linaro.org>
 *
 * Most code is derived from arm64 port of kexec-tools
 */

// C headers are supplied by other translation units.

#[repr(C)]
pub struct KexecFileOps {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct KimageArch {
    pub dtb: *mut core::ffi::c_void,
    pub dtb_mem: usize,
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct Kimage {
    pub arch: KimageArch,
    pub elf_headers: *mut core::ffi::c_void,
    pub elf_headers_sz: usize,
    pub nr_segments: usize,
    pub r#type: i32,
    pub elf_load_addr: usize,
}

#[repr(C)]
pub struct KexecBuf {
    pub image: *mut Kimage,
    pub buffer: *mut core::ffi::c_void,
    pub bufsz: usize,
    pub mem: usize,
    pub memsz: usize,
    pub buf_min: usize,
    pub buf_max: usize,
    pub buf_align: usize,
    pub top_down: bool,
}

#[repr(C)]
pub struct CrashMemRange {
    pub start: u64,
    pub end: u64,
}

#[repr(C)]
pub struct CrashMem {
    pub ranges: *mut CrashMemRange,
    pub nr_ranges: usize,
}

unsafe extern "C" {
    pub static kexec_image_ops: KexecFileOps;
    pub static mut crashk_cma_cnt: u32;
    pub static mut ULONG_MAX: usize;

    fn kvfree(addr: *mut core::ffi::c_void);
    fn vfree(addr: *mut core::ffi::c_void);
    fn kexec_image_post_load_cleanup_default(image: *mut Kimage) -> i32;
    fn crash_prepare_headers(need_backup: bool, headers: *mut *mut core::ffi::c_void,
                             headers_sz: *mut usize, arg: *mut core::ffi::c_void) -> i32;
    fn kexec_add_buffer(kbuf: *mut KexecBuf) -> i32;
    fn crash_load_dm_crypt_keys(image: *mut Kimage) -> i32;
    fn of_kexec_alloc_and_setup_fdt(image: *mut Kimage, initrd_load_addr: usize,
                                    initrd_len: usize, cmdline: *mut i8, flags: u32)
                                    -> *mut core::ffi::c_void;
    fn fdt_pack(dtb: *mut core::ffi::c_void) -> i32;
    fn fdt_totalsize(dtb: *const core::ffi::c_void) -> usize;
}

pub const KEXEC_BUF_MEM_UNKNOWN: usize = usize::MAX;
pub const KEXEC_TYPE_CRASH: i32 = 1;
pub const SZ_64K: usize = 64 * 1024;
pub const SZ_1G: usize = 1024 * 1024 * 1024;
pub const SZ_2M: usize = 2 * 1024 * 1024;

#[no_mangle]
pub static kexec_file_loaders: [*const KexecFileOps; 2] = [
    unsafe { &kexec_image_ops as *const KexecFileOps },
    core::ptr::null(),
];

#[no_mangle]
pub unsafe extern "C" fn arch_kimage_file_post_load_cleanup(image: *mut Kimage) -> i32 {
    kvfree((*image).arch.dtb);
    (*image).arch.dtb = core::ptr::null_mut();

    vfree((*image).elf_headers);
    (*image).elf_headers = core::ptr::null_mut();
    (*image).elf_headers_sz = 0;

    kexec_image_post_load_cleanup_default(image)
}

// CONFIG_CRASH_DUMP conditional declarations and definitions.
#[cfg(feature = "CONFIG_CRASH_DUMP")]
pub unsafe extern "C" fn arch_get_system_nr_ranges() -> u32 {
    // for exclusion of crashkernel region
    let mut nr_ranges: u32 = 2 + crashk_cma_cnt;
    let mut i: u64 = 0;
    let mut start: u64 = 0;
    let mut end: u64 = 0;
    while for_each_mem_range(&mut i, &mut start, &mut end) {
        nr_ranges += 1;
    }
    nr_ranges
}

#[cfg(feature = "CONFIG_CRASH_DUMP")]
pub unsafe extern "C" fn arch_crash_populate_cmem(cmem: *mut CrashMem) -> i32 {
    let mut i: u64 = 0;
    let mut start: u64 = 0;
    let mut end: u64 = 0;
    while for_each_mem_range(&mut i, &mut start, &mut end) {
        let range = (*cmem).ranges.add((*cmem).nr_ranges);
        (*range).start = start;
        (*range).end = end - 1;
        (*cmem).nr_ranges += 1;
    }
    0
}

unsafe extern "C" {
    fn for_each_mem_range(index: *mut u64, start: *mut u64, end: *mut u64) -> bool;
}

pub unsafe extern "C" fn load_other_segments(
    image: *mut Kimage,
    kernel_load_addr: usize,
    kernel_size: usize,
    initrd: *mut i8,
    initrd_len: usize,
    cmdline: *mut i8,
) -> i32 {
    let mut kbuf: KexecBuf = core::mem::zeroed();
    let mut dtb: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut initrd_load_addr: usize = 0;
    let mut dtb_len: usize;
    let orig_segments = (*image).nr_segments;
    let mut ret = 0;

    kbuf.image = image;
    kbuf.buf_min = kernel_load_addr.wrapping_add(kernel_size);

    #[cfg(feature = "CONFIG_CRASH_DUMP")]
    if (*image).r#type == KEXEC_TYPE_CRASH {
        let mut headers: *mut core::ffi::c_void = core::ptr::null_mut();
        let mut headers_sz: usize = 0;
        ret = crash_prepare_headers(true, &mut headers, &mut headers_sz, core::ptr::null_mut());
        if ret != 0 { return goto_out_err(image, &mut dtb, orig_segments, ret); }
        kbuf.buffer = headers;
        kbuf.bufsz = headers_sz;
        kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
        kbuf.memsz = headers_sz;
        kbuf.buf_align = SZ_64K;
        kbuf.buf_max = usize::MAX;
        kbuf.top_down = true;
        ret = kexec_add_buffer(&mut kbuf);
        if ret != 0 {
            vfree(headers);
            return goto_out_err(image, &mut dtb, orig_segments, ret);
        }
        (*image).elf_headers = headers;
        (*image).elf_load_addr = kbuf.mem;
        ret = crash_load_dm_crypt_keys(image);
        if ret != 0 { return goto_out_err(image, &mut dtb, orig_segments, ret); }
    }

    if !initrd.is_null() {
        kbuf.buffer = initrd.cast();
        kbuf.bufsz = initrd_len;
        kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
        kbuf.memsz = initrd_len;
        kbuf.buf_align = 0;
        kbuf.buf_max = (kernel_load_addr & !(SZ_1G - 1)).wrapping_add(SZ_1G * 32);
        kbuf.top_down = false;
        ret = kexec_add_buffer(&mut kbuf);
        if ret != 0 { return goto_out_err(image, &mut dtb, orig_segments, ret); }
        initrd_load_addr = kbuf.mem;
    }

    dtb = of_kexec_alloc_and_setup_fdt(image, initrd_load_addr, initrd_len, cmdline, 0);
    if dtb.is_null() {
        ret = -22;
        return goto_out_err(image, &mut dtb, orig_segments, ret);
    }
    fdt_pack(dtb);
    dtb_len = fdt_totalsize(dtb);
    kbuf.buffer = dtb;
    kbuf.bufsz = dtb_len;
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    kbuf.memsz = dtb_len;
    kbuf.buf_align = SZ_2M;
    kbuf.buf_max = usize::MAX;
    kbuf.top_down = true;
    ret = kexec_add_buffer(&mut kbuf);
    if ret != 0 { return goto_out_err(image, &mut dtb, orig_segments, ret); }
    (*image).arch.dtb = dtb;
    (*image).arch.dtb_mem = kbuf.mem;
    ret = 0;
    ret
}

unsafe fn goto_out_err(image: *mut Kimage, dtb: &mut *mut core::ffi::c_void,
                       orig_segments: usize, ret: i32) -> i32 {
    (*image).nr_segments = orig_segments;
    kvfree(*dtb);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
