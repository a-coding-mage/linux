/* SPDX-License-Identifier: GPL-2.0 */
/*
 *    Filename: ks0108.h
 *     Version: 0.1.0
 * Description: ks0108 LCD Controller driver header
 *
 *      Author: Copyright (C) Miguel Ojeda <ojeda@kernel.org>
 *        Date: 2006-10-31
 */

/* Write a byte to the data port */
unsafe extern "C" {
    pub fn ks0108_writedata(byte: u8);
}

/* Write a byte to the control port */
unsafe extern "C" {
    pub fn ks0108_writecontrol(byte: u8);
}

/* Set the controller's current display state (0..1) */
unsafe extern "C" {
    pub fn ks0108_displaystate(state: u8);
}

/* Set the controller's current startline (0..63) */
unsafe extern "C" {
    pub fn ks0108_startline(startline: u8);
}

/* Set the controller's current address (0..63) */
unsafe extern "C" {
    pub fn ks0108_address(address: u8);
}

/* Set the controller's current page (0..7) */
unsafe extern "C" {
    pub fn ks0108_page(page: u8);
}

/* Is the module inited? */
unsafe extern "C" {
    pub fn ks0108_isinited() -> u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
