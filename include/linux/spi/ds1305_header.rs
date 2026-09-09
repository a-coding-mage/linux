/* SPDX-License-Identifier: GPL-2.0 */

/*
 * One-time configuration for ds1305 and ds1306 RTC chips.
 *
 * Put a pointer to this in spi_board_info.platform_data if you want to
 * be sure that Linux (re)initializes this as needed ... after losing
 * backup power, and potentially on the first boot.
 */

/*
 * Trickle charge configuration: it's OK to leave out the MAGIC
 * bitmask; mask in either DS1 or DS2, and then one of 2K/4k/8K.
 */
pub const DS1305_TRICKLE_MAGIC: u8 = 0xa0;
pub const DS1305_TRICKLE_DS2: u8 = 0x08; /* two diodes */
pub const DS1305_TRICKLE_DS1: u8 = 0x04; /* one diode */
pub const DS1305_TRICKLE_2K: u8 = 0x01; /* 2 KOhm resistance */
pub const DS1305_TRICKLE_4K: u8 = 0x02; /* 4 KOhm resistance */
pub const DS1305_TRICKLE_8K: u8 = 0x03; /* 8 KOhm resistance */

#[repr(C)]
pub struct ds1305_platform_data {
    pub trickle: u8,

    /* set only on ds1306 parts */
    pub is_ds1306: bool,

    /* ds1306 only: enable 1 Hz output */
    pub en_1hz: bool,

    /* REVISIT: the driver currently expects nINT0 to be wired
     * as the alarm IRQ. ALM1 may also need to be set up ...
     */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
