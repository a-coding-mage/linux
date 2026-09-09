// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by bcm47xx_private.h, linux/gpio/legacy.h,
// bcm47xx_board.h, and bcm47xx.h are referenced externally.

unsafe extern "C" {
    fn gpio_request_one(gpio: ::core::ffi::c_int, flags: ::core::ffi::c_ulong, label: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn gpio_free(gpio: ::core::ffi::c_uint);
    fn bcm47xx_board_get() -> bcm47xx_board;
}

// C enum bcm47xx_board, GPIOF_OUT_INIT_HIGH, and bcm47xx board constants are
// supplied by the corresponding external headers.
type bcm47xx_board = ::core::ffi::c_int;

unsafe fn bcm47xx_workarounds_enable_usb_power(usb_power: ::core::ffi::c_int) {
    let err: ::core::ffi::c_int;

    err = unsafe {
        gpio_request_one(
            usb_power,
            GPIOF_OUT_INIT_HIGH,
            b"usb_power\0".as_ptr() as *const ::core::ffi::c_char,
        )
    };
    if err != 0 {
        // Equivalent of pr_err("Failed to request USB power gpio: %d\n", err).
        pr_err!("Failed to request USB power gpio: {}\n", err);
    } else {
        unsafe { gpio_free(usb_power as ::core::ffi::c_uint) };
    }
}

pub unsafe fn bcm47xx_workarounds() {
    let board: bcm47xx_board = unsafe { bcm47xx_board_get() };

    match board {
        BCM47XX_BOARD_NETGEAR_WNR3500L
        | BCM47XX_BOARD_NETGEAR_WNR3500L_V2 => unsafe {
            bcm47xx_workarounds_enable_usb_power(12);
        },
        BCM47XX_BOARD_NETGEAR_WNDR3400V2
        | BCM47XX_BOARD_NETGEAR_WNDR3400_V3 => unsafe {
            bcm47xx_workarounds_enable_usb_power(21);
        },
        _ => {
            // No workaround(s) needed
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
