// SPDX-License-Identifier: Zlib
//
// C dependency: dfltcc.h

/* External functions */
unsafe extern "C" {
    pub fn dfltcc_can_deflate(strm: z_streamp) -> ::std::os::raw::c_int;
    pub fn dfltcc_deflate(
        strm: z_streamp,
        flush: ::std::os::raw::c_int,
        result: *mut block_state,
    ) -> ::std::os::raw::c_int;
    pub fn dfltcc_reset_deflate_state(strm: z_streamp);
}

#[macro_export]
macro_rules! DEFLATE_RESET_HOOK {
    ($strm:expr) => {
        unsafe { $crate::dfltcc_reset_deflate_state($strm) }
    };
}

#[macro_export]
macro_rules! DEFLATE_HOOK {
    () => {
        $crate::dfltcc_deflate
    };
}

#[macro_export]
macro_rules! DEFLATE_NEED_CHECKSUM {
    ($strm:expr) => {
        unsafe { $crate::dfltcc_can_deflate($strm) == 0 }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
