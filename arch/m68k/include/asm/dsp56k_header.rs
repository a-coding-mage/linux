/*
 * linux/include/asm-m68k/dsp56k.h - defines and declarations for
 *                                   DSP56k device driver
 *
 * Copyright (C) 1996,1997 Fredrik Noring, lars brinkhoff & Tomas Berndtsson
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

/* Used for uploading DSP binary code */
#[repr(C)]
pub struct dsp56k_upload {
    pub len: i32,
    pub bin: *mut core::ffi::c_char,
}

/* For the DSP host flags */
#[repr(C)]
pub struct dsp56k_host_flags {
    pub dir: i32, /* Bit field. 1 = write output bit, 0 = do nothing.
                  * 0x0000 means reading only, 0x0011 means
                  * writing the bits stored in `out' on HF0 and HF1.
                  * Note that HF2 and HF3 can only be read.
                  */
    pub out: i32,    /* Bit field like above. */
    pub status: i32, /* Host register's current state is returned */
}

/* ioctl command codes */
pub const DSP56K_UPLOAD: i32 = 1; /* Upload DSP binary program       */
pub const DSP56K_SET_TX_WSIZE: i32 = 2; /* Host transmit word size (1-4)   */
pub const DSP56K_SET_RX_WSIZE: i32 = 3; /* Host receive word size (1-4)    */
pub const DSP56K_HOST_FLAGS: i32 = 4; /* Host flag registers             */
pub const DSP56K_HOST_CMD: i32 = 5; /* Trig Host Command (0-31)        */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
