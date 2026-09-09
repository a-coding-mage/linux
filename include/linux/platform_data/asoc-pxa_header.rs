/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <sound/core.h>
// #include <sound/pcm.h>
// #include <sound/ac97_codec.h>

// Opaque type supplied by the GPIO dependency.
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

extern "C" {
    pub fn pxa27x_configure_ac97reset(
        reset_gpio: *mut gpio_desc,
        to_gpio: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
