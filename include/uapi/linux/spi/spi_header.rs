/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

// Dependency: <linux/const.h> provides _BITUL(n), equivalent here to 1usize << n.

pub const SPI_CPHA: usize = 1usize << 0; /* clock phase */
pub const SPI_CPOL: usize = 1usize << 1; /* clock polarity */

pub const SPI_MODE_0: usize = 0 | 0; /* (original MicroWire) */
pub const SPI_MODE_1: usize = 0 | SPI_CPHA;
pub const SPI_MODE_2: usize = SPI_CPOL | 0;
pub const SPI_MODE_3: usize = SPI_CPOL | SPI_CPHA;
pub const SPI_MODE_X_MASK: usize = SPI_CPOL | SPI_CPHA;

pub const SPI_CS_HIGH: usize = 1usize << 2; /* chipselect active high? */
pub const SPI_LSB_FIRST: usize = 1usize << 3; /* per-word bits-on-wire */
pub const SPI_3WIRE: usize = 1usize << 4; /* SI/SO signals shared */
pub const SPI_LOOP: usize = 1usize << 5; /* loopback mode */
pub const SPI_NO_CS: usize = 1usize << 6; /* 1 dev/bus, no chipselect */
pub const SPI_READY: usize = 1usize << 7; /* slave pulls low to pause */
pub const SPI_TX_DUAL: usize = 1usize << 8; /* transmit with 2 wires */
pub const SPI_TX_QUAD: usize = 1usize << 9; /* transmit with 4 wires */
pub const SPI_RX_DUAL: usize = 1usize << 10; /* receive with 2 wires */
pub const SPI_RX_QUAD: usize = 1usize << 11; /* receive with 4 wires */
pub const SPI_CS_WORD: usize = 1usize << 12; /* toggle cs after each word */
pub const SPI_TX_OCTAL: usize = 1usize << 13; /* transmit with 8 wires */
pub const SPI_RX_OCTAL: usize = 1usize << 14; /* receive with 8 wires */
pub const SPI_3WIRE_HIZ: usize = 1usize << 15; /* high impedance turnaround */
pub const SPI_RX_CPHA_FLIP: usize = 1usize << 16; /* flip CPHA on Rx only xfer */
pub const SPI_MOSI_IDLE_LOW: usize = 1usize << 17; /* leave MOSI line low when idle */
pub const SPI_MOSI_IDLE_HIGH: usize = 1usize << 18; /* leave MOSI line high when idle */

/*
 * All the bits defined above should be covered by SPI_MODE_USER_MASK.
 * The SPI_MODE_USER_MASK has the SPI_MODE_KERNEL_MASK counterpart in
 * 'include/linux/spi/spi.h'. The bits defined here are from bit 0 upwards
 * while in SPI_MODE_KERNEL_MASK they are from the other end downwards.
 * These bits must not overlap. A static assert check should make sure of that.
 * If adding extra bits, make sure to increase the bit index below as well.
 */
pub const SPI_MODE_USER_MASK: usize = (1usize << 19) - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
