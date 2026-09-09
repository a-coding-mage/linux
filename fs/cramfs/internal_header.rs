/* Uncompression interfaces to the underlying zlib */
extern "C" {
    pub fn cramfs_uncompress_block(
        dst: *mut core::ffi::c_void,
        dstlen: core::ffi::c_int,
        src: *mut core::ffi::c_void,
        srclen: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn cramfs_uncompress_init() -> core::ffi::c_int;
    pub fn cramfs_uncompress_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
