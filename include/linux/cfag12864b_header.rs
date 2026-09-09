/* SPDX-License-Identifier: GPL-2.0 */
/*
 *    Filename: cfag12864b.h
 *     Version: 0.1.0
 * Description: cfag12864b LCD driver header
 *
 *      Author: Copyright (C) Miguel Ojeda <ojeda@kernel.org>
 *        Date: 2006-10-12
 */

pub const CFAG12864B_WIDTH: usize = 128;
pub const CFAG12864B_HEIGHT: usize = 64;
pub const CFAG12864B_CONTROLLERS: usize = 2;
pub const CFAG12864B_PAGES: usize = 8;
pub const CFAG12864B_ADDRESSES: usize = 64;
pub const CFAG12864B_SIZE: usize = CFAG12864B_CONTROLLERS
    * CFAG12864B_PAGES
    * CFAG12864B_ADDRESSES;

/*
 * The driver will blit this buffer to the LCD
 *
 * Its size is CFAG12864B_SIZE.
 */
extern "C" {
    pub static mut cfag12864b_buffer: *mut u8;
}

/*
 * Enable refreshing
 *
 * Returns 0 if successful (anyone was using it),
 * or != 0 if failed (someone is using it).
 */
extern "C" {
    pub fn cfag12864b_enable() -> u8;
}

/*
 * Disable refreshing
 *
 * You should call this only when you finish using the LCD.
 */
extern "C" {
    pub fn cfag12864b_disable();
}

/*
 * Is the module inited?
 */
extern "C" {
    pub fn cfag12864b_isinited() -> u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
