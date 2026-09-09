/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Dependency intent from the original header:
 *   linux/uaccess.h
 *   asm/termios.h
 */

/*
 * intr=^C        quit=^\\       erase=del     kill=^U
 * eof=^D         vtime=\\0      vmin=\\1      sxtc=\\0
 * start=^Q       stop=^S        susp=^Z       eol=\\0
 * reprint=^R     discard=^O     werase=^W     lnext=^V
 * eol2=\\0
 */

/* The original VDSUSP conditional is a build-time C preprocessor condition. */
#[cfg(feature = "VDSUSP")]
#[macro_export]
macro_rules! INIT_C_CC_VDSUSP_EXTRA {
    ($cc:expr) => {
        $cc[VDSUSP as usize] = b'Y' - 0x40;
    };
}

#[cfg(not(feature = "VDSUSP"))]
#[macro_export]
macro_rules! INIT_C_CC_VDSUSP_EXTRA {
    ($cc:expr) => {};
}

/* Translation of the C initializer macro. */
#[macro_export]
macro_rules! INIT_C_CC {
    () => {{
        let mut cc = [0u8; NCCS as usize];
        cc[VINTR as usize] = b'C' - 0x40;
        cc[VQUIT as usize] = b'\\' - 0x40;
        cc[VERASE as usize] = 0o177;
        cc[VKILL as usize] = b'U' - 0x40;
        cc[VEOF as usize] = b'D' - 0x40;
        cc[VSTART as usize] = b'Q' - 0x40;
        cc[VSTOP as usize] = b'S' - 0x40;
        cc[VSUSP as usize] = b'Z' - 0x40;
        cc[VREPRINT as usize] = b'R' - 0x40;
        cc[VDISCARD as usize] = b'O' - 0x40;
        cc[VWERASE as usize] = b'W' - 0x40;
        cc[VLNEXT as usize] = b'V' - 0x40;
        INIT_C_CC_VDSUSP_EXTRA!(cc);
        cc[VMIN as usize] = 1;
        cc
    }};
}

/* Opaque types supplied by the corresponding kernel headers. */
#[repr(C)]
pub struct ktermios {
    _private: [u8; 0],
}
#[repr(C)]
pub struct termio {
    _private: [u8; 0],
}
#[repr(C)]
pub struct termios {
    _private: [u8; 0],
}
#[repr(C)]
pub struct termios2 {
    _private: [u8; 0],
}

extern "C" {
    pub fn user_termio_to_kernel_termios(
        kernel: *mut ktermios,
        user: *mut termio,
    ) -> ::core::ffi::c_int;
    pub fn kernel_termios_to_user_termio(
        user: *mut termio,
        kernel: *mut ktermios,
    ) -> ::core::ffi::c_int;

    /* The TCGETS2 conditional is preserved as build-time intent. */
    #[cfg(feature = "TCGETS2")]
    pub fn user_termios_to_kernel_termios(
        kernel: *mut ktermios,
        user: *mut termios2,
    ) -> ::core::ffi::c_int;
    #[cfg(feature = "TCGETS2")]
    pub fn kernel_termios_to_user_termios(
        user: *mut termios2,
        kernel: *mut ktermios,
    ) -> ::core::ffi::c_int;
    #[cfg(feature = "TCGETS2")]
    pub fn user_termios_to_kernel_termios_1(
        kernel: *mut ktermios,
        user: *mut termios,
    ) -> ::core::ffi::c_int;
    #[cfg(feature = "TCGETS2")]
    pub fn kernel_termios_to_user_termios_1(
        user: *mut termios,
        kernel: *mut ktermios,
    ) -> ::core::ffi::c_int;

    #[cfg(not(feature = "TCGETS2"))]
    pub fn user_termios_to_kernel_termios(
        kernel: *mut ktermios,
        user: *mut termios,
    ) -> ::core::ffi::c_int;
    #[cfg(not(feature = "TCGETS2"))]
    pub fn kernel_termios_to_user_termios(
        user: *mut termios,
        kernel: *mut ktermios,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
