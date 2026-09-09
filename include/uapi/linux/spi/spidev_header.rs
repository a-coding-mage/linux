/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * include/linux/spi/spidev.h
 *
 * Copyright (C) 2006 SWAPP
 *	Andrea Paterniani <a.paterniani@swapp-eng.it>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
 */

/* Dependencies: linux/types.h, linux/ioctl.h, and linux/spi/spi.h. */

/* IOCTL commands */

pub const SPI_IOC_MAGIC: u8 = b'k';

/**
 * struct spi_ioc_transfer - describes a single SPI transfer
 * @tx_buf: Holds pointer to userspace buffer with transmit data, or null.
 * @rx_buf: Holds pointer to userspace buffer for receive data, or null.
 * @len: Length of tx and rx buffers, in bytes.
 * @speed_hz: Temporary override of the device's bitrate.
 * @bits_per_word: Temporary override of the device's wordsize.
 * @delay_usecs: If nonzero, how long to delay after the last bit transfer.
 * @cs_change: True to deselect device before starting the next transfer.
 * @word_delay_usecs: If nonzero, how long to wait between words within one transfer.
 *
 * This structure is mapped directly to the kernel spi_transfer structure.
 * Zero-initialize the structure, including currently unused fields.
 */
#[repr(C)]
pub struct spi_ioc_transfer {
    pub tx_buf: u64,
    pub rx_buf: u64,
    pub len: u32,
    pub speed_hz: u32,
    pub delay_usecs: u16,
    pub bits_per_word: u8,
    pub cs_change: u8,
    pub tx_nbits: u8,
    pub rx_nbits: u8,
    pub word_delay_usecs: u8,
    pub pad: u8,
}

/* Not all platforms use asm-generic/ioctl.h or _IOC_TYPECHECK(). */
#[macro_export]
macro_rules! SPI_MSGSIZE {
    ($n:expr) => {
        if (($n) * core::mem::size_of::<$crate::spi_ioc_transfer>()) < (1usize << _IOC_SIZEBITS) {
            ($n) * core::mem::size_of::<$crate::spi_ioc_transfer>()
        } else {
            0usize
        }
    };
}

#[macro_export]
macro_rules! SPI_IOC_MESSAGE {
    ($n:expr) => {
        _IOW!(SPI_IOC_MAGIC, 0, [u8; SPI_MSGSIZE!($n)])
    };
}

/* Read / Write of SPI mode (SPI_MODE_0..SPI_MODE_3) (limited to 8 bits) */
pub const SPI_IOC_RD_MODE: _ = _IOR!(SPI_IOC_MAGIC, 1, u8);
pub const SPI_IOC_WR_MODE: _ = _IOW!(SPI_IOC_MAGIC, 1, u8);

/* Read / Write SPI bit justification */
pub const SPI_IOC_RD_LSB_FIRST: _ = _IOR!(SPI_IOC_MAGIC, 2, u8);
pub const SPI_IOC_WR_LSB_FIRST: _ = _IOW!(SPI_IOC_MAGIC, 2, u8);

/* Read / Write SPI device word length (1..N) */
pub const SPI_IOC_RD_BITS_PER_WORD: _ = _IOR!(SPI_IOC_MAGIC, 3, u8);
pub const SPI_IOC_WR_BITS_PER_WORD: _ = _IOW!(SPI_IOC_MAGIC, 3, u8);

/* Read / Write SPI device default max speed hz */
pub const SPI_IOC_RD_MAX_SPEED_HZ: _ = _IOR!(SPI_IOC_MAGIC, 4, u32);
pub const SPI_IOC_WR_MAX_SPEED_HZ: _ = _IOW!(SPI_IOC_MAGIC, 4, u32);

/* Read / Write of the SPI mode field */
pub const SPI_IOC_RD_MODE32: _ = _IOR!(SPI_IOC_MAGIC, 5, u32);
pub const SPI_IOC_WR_MODE32: _ = _IOW!(SPI_IOC_MAGIC, 5, u32);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
