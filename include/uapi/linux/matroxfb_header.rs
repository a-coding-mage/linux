/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependencies corresponding to the C header includes are supplied externally. */

#[repr(C)]
pub struct matroxioc_output_mode {
    pub output: u32, /* which output */
    pub mode: u32,   /* which mode */
}

pub const MATROXFB_OUTPUT_PRIMARY: u32 = 0x0000;
pub const MATROXFB_OUTPUT_SECONDARY: u32 = 0x0001;
pub const MATROXFB_OUTPUT_DFP: u32 = 0x0002;

pub const MATROXFB_OUTPUT_MODE_PAL: u32 = 0x0001;
pub const MATROXFB_OUTPUT_MODE_NTSC: u32 = 0x0002;
pub const MATROXFB_OUTPUT_MODE_MONITOR: u32 = 0x0080;

/* ioctl encodings: equivalent to _IOW/_IOWR('n', request, size_t). */
pub const MATROXFB_SET_OUTPUT_MODE: usize = ioctl_iow!(b'n', 0xFA, usize);
pub const MATROXFB_GET_OUTPUT_MODE: usize = ioctl_iowr!(b'n', 0xFA, usize);

/* bitfield */
pub const MATROXFB_OUTPUT_CONN_PRIMARY: u32 = 1 << MATROXFB_OUTPUT_PRIMARY;
pub const MATROXFB_OUTPUT_CONN_SECONDARY: u32 = 1 << MATROXFB_OUTPUT_SECONDARY;
pub const MATROXFB_OUTPUT_CONN_DFP: u32 = 1 << MATROXFB_OUTPUT_DFP;

/* connect these outputs to this framebuffer */
pub const MATROXFB_SET_OUTPUT_CONNECTION: usize = ioctl_iow!(b'n', 0xF8, usize);
/* which outputs are connected to this framebuffer */
pub const MATROXFB_GET_OUTPUT_CONNECTION: usize = ioctl_ior!(b'n', 0xF8, usize);
/* which outputs are available for this framebuffer */
pub const MATROXFB_GET_AVAILABLE_OUTPUTS: usize = ioctl_ior!(b'n', 0xF9, usize);
/* which outputs exist on this framebuffer */
pub const MATROXFB_GET_ALL_OUTPUTS: usize = ioctl_ior!(b'n', 0xFB, usize);

#[repr(i32)]
pub enum matroxfb_ctrl_id {
    MATROXFB_CID_TESTOUT = V4L2_CID_PRIVATE_BASE,
    MATROXFB_CID_DEFLICKER,
    MATROXFB_CID_LAST,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
