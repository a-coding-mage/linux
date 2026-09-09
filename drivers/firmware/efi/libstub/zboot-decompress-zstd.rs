// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding EFI, zstd, and decompression
// interfaces are intentionally referenced here rather than reimplemented.

use core::ffi::c_void;

#[repr(C)]
pub struct zstd_dctx {
    _opaque: [u8; 0],
}

extern "C" {
    static mut _gzdata_start: u8;
    static mut _gzdata_end: u8;
    static mut payload_size: u32;

    fn zstd_dctx_workspace_bound() -> usize;
    fn efi_allocate_pages(size: usize, addr: *mut usize, max_addr: usize) -> efi_status_t;
    fn zstd_init_dctx(wksp: *mut c_void, wksp_size: usize) -> *mut zstd_dctx;
    fn zstd_decompress_dctx(
        dctx: *mut zstd_dctx,
        dst: *mut u8,
        dst_size: usize,
        src: *const u8,
        src_size: usize,
    ) -> usize;
    fn efi_free(size: usize, addr: usize);
    fn zstd_get_error_code(result: usize) -> i32;
    fn efi_err(fmt: *const u8, ...);
    fn efi_cache_sync_image(addr: usize, size: usize);
}

// Supplied by the EFI interfaces.
type efi_status_t = usize;
const EFI_SUCCESS: efi_status_t = 0;
const EFI_LOAD_ERROR: efi_status_t = 1;

static mut wksp_size: usize = 0;
static mut wksp: *mut c_void = core::ptr::null_mut();

pub unsafe extern "C" fn efi_zboot_decompress_init(alloc_size: *mut usize) -> efi_status_t {
    let status: efi_status_t;

    wksp_size = zstd_dctx_workspace_bound();
    status = efi_allocate_pages(wksp_size, &mut wksp as *mut *mut c_void as *mut usize, usize::MAX);
    if status != EFI_SUCCESS {
        return status;
    }

    *alloc_size = payload_size as usize;
    EFI_SUCCESS
}

pub unsafe extern "C" fn efi_zboot_decompress(out: *mut u8, outlen: usize) -> efi_status_t {
    let dctx: *mut zstd_dctx = zstd_init_dctx(wksp, wksp_size);
    let ret: usize;
    let retval: i32;

    ret = zstd_decompress_dctx(
        dctx,
        out,
        outlen,
        &_gzdata_start as *const u8,
        (&_gzdata_end as *const u8).offset_from(&_gzdata_start as *const u8) as usize - 4,
    );
    efi_free(wksp_size, wksp as usize);

    retval = zstd_get_error_code(ret);
    if retval != 0 {
        efi_err(b"ZSTD-decompression failed with status %d\n\0".as_ptr(), retval);
        return EFI_LOAD_ERROR;
    }

    efi_cache_sync_image(out as usize, outlen);

    EFI_SUCCESS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
