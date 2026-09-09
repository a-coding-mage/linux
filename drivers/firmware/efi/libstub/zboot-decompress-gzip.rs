// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding EFI and zlib code:
// linux/efi.h, linux/zlib.h, asm/efi.h, efistub.h, inftrees.c, inffast.c,
// and inflate.c.

extern "C" {
    static mut _gzdata_start: *mut u8;
    static mut _gzdata_end: *mut u8;
    #[repr(align(1))]
    static mut payload_size: u32;

    fn efi_allocate_pages(
        size: usize,
        addr: *mut usize,
        max: usize,
    ) -> efi_status_t;
    fn zlib_inflate_workspacesize() -> usize;
    fn zlib_inflate_init2(stream: *mut z_stream_s, window_bits: i32) -> i32;
    fn zlib_inflate(stream: *mut z_stream_s, flush: i32) -> i32;
    fn zlib_inflate_end(stream: *mut z_stream_s) -> i32;
    fn efi_free(size: usize, addr: usize);
    fn efi_err(format: *const u8, ...);
    fn efi_cache_sync_image(addr: usize, len: usize);
}

static mut stream: z_stream_s = z_stream_s {
    next_in: core::ptr::null_mut(),
    avail_in: 0,
    next_out: core::ptr::null_mut(),
    avail_out: 0,
    workspace: core::ptr::null_mut(),
};

// The concrete layout is supplied by linux/zlib.h.
#[repr(C)]
struct z_stream_s {
    next_in: *mut u8,
    avail_in: usize,
    next_out: *mut u8,
    avail_out: usize,
    workspace: *mut core::ffi::c_void,
}

type efi_status_t = usize;
type u8 = core::ffi::c_uchar;

const EFI_SUCCESS: efi_status_t = 0;
const EFI_LOAD_ERROR: efi_status_t = 1;
const Z_OK: i32 = 0;
const Z_STREAM_END: i32 = 1;
const MAX_WBITS: i32 = 15;
const ULONG_MAX: usize = usize::MAX;

unsafe fn zlib_inflateInit2(stream: *mut z_stream_s, window_bits: i32) -> i32 {
    zlib_inflate_init2(stream, window_bits)
}

unsafe fn zlib_inflateEnd(stream: *mut z_stream_s) -> i32 {
    zlib_inflate_end(stream)
}

unsafe fn efi_zboot_decompress_init(alloc_size: *mut usize) -> efi_status_t {
    let status: efi_status_t;
    let rc: i32;

    /* skip the 10 byte header, assume no recorded filename */
    stream.next_in = _gzdata_start.add(10);
    stream.avail_in = _gzdata_end.offset_from(stream.next_in) as usize;

    status = efi_allocate_pages(
        zlib_inflate_workspacesize(),
        &mut stream.workspace as *mut *mut core::ffi::c_void as *mut usize,
        ULONG_MAX,
    );
    if status != EFI_SUCCESS {
        return status;
    }

    rc = zlib_inflateInit2(&mut stream, -MAX_WBITS);
    if rc != Z_OK {
        efi_err(b"failed to initialize GZIP decompressor: %d\n\0".as_ptr(), rc);
        status = EFI_LOAD_ERROR;
        return {
            efi_free(zlib_inflate_workspacesize(), stream.workspace as usize);
            status
        };
    }

    *alloc_size = payload_size as usize;
    EFI_SUCCESS
}

unsafe fn efi_zboot_decompress(out: *mut u8, outlen: usize) -> efi_status_t {
    let rc: i32;

    stream.next_out = out;
    stream.avail_out = outlen;

    rc = zlib_inflate(&mut stream, 0);
    zlib_inflateEnd(&mut stream);

    efi_free(zlib_inflate_workspacesize(), stream.workspace as usize);

    if rc != Z_STREAM_END {
        efi_err(b"GZIP decompression failed with status %d\n\0".as_ptr(), rc);
        return EFI_LOAD_ERROR;
    }

    efi_cache_sync_image(out as usize, outlen);

    EFI_SUCCESS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
