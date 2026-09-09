/* infutil.h -- types and macros common to blocks and codes
 * Copyright (C) 1995-1998 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */

/* WARNING: this file should *not* be used by applications. It is
   part of the implementation of the compression library and is
   subject to change. Applications should only use zlib.h.
 */

/* C dependencies: linux/zlib.h, and, when CONFIG_ZLIB_DFLTCC is enabled,
 * ../zlib_dfltcc/dfltcc.h and asm/page.h. These symbols are supplied by
 * other translation units.
 */

/* memory allocation for inflation */

#[repr(C)]
pub struct inflate_workspace {
    pub inflate_state: inflate_state,
    #[cfg(feature = "CONFIG_ZLIB_DFLTCC")]
    pub dfltcc_state: dfltcc_state,
    #[cfg(feature = "CONFIG_ZLIB_DFLTCC")]
    pub working_window: [u8; (1usize << MAX_WBITS) + PAGE_SIZE],
    #[cfg(not(feature = "CONFIG_ZLIB_DFLTCC"))]
    pub working_window: [u8; 1usize << MAX_WBITS],
}

#[cfg(feature = "CONFIG_ZLIB_DFLTCC")]
/* dfltcc_state must be doubleword aligned for DFLTCC call */
const _: () = {
    assert!(core::mem::offset_of!(inflate_workspace, dfltcc_state) % 8 == 0);
};

#[macro_export]
macro_rules! WS {
    ($strm:expr) => {
        ($strm.workspace as *mut inflate_workspace)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
