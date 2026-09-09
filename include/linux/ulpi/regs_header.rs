/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Macros for Set and Clear
 * See ULPI 1.1 specification to find the registers with Set and Clear offsets
 */
macro_rules! ULPI_SET {
    ($a:expr) => { ($a + 1) };
}

macro_rules! ULPI_CLR {
    ($a:expr) => { ($a + 2) };
}

/*
 * Register Map
 */
pub const ULPI_VENDOR_ID_LOW: u32 = 0x00;
pub const ULPI_VENDOR_ID_HIGH: u32 = 0x01;
pub const ULPI_PRODUCT_ID_LOW: u32 = 0x02;
pub const ULPI_PRODUCT_ID_HIGH: u32 = 0x03;
pub const ULPI_FUNC_CTRL: u32 = 0x04;
pub const ULPI_IFC_CTRL: u32 = 0x07;
pub const ULPI_OTG_CTRL: u32 = 0x0a;
pub const ULPI_USB_INT_EN_RISE: u32 = 0x0d;
pub const ULPI_USB_INT_EN_FALL: u32 = 0x10;
pub const ULPI_USB_INT_STS: u32 = 0x13;
pub const ULPI_USB_INT_LATCH: u32 = 0x14;
pub const ULPI_DEBUG: u32 = 0x15;
pub const ULPI_SCRATCH: u32 = 0x16;
/* Optional Carkit Registers */
pub const ULPI_CARKIT_CTRL: u32 = 0x19;
pub const ULPI_CARKIT_INT_DELAY: u32 = 0x1c;
pub const ULPI_CARKIT_INT_EN: u32 = 0x1d;
pub const ULPI_CARKIT_INT_STS: u32 = 0x20;
pub const ULPI_CARKIT_INT_LATCH: u32 = 0x21;
pub const ULPI_CARKIT_PLS_CTRL: u32 = 0x22;
/* Other Optional Registers */
pub const ULPI_TX_POS_WIDTH: u32 = 0x25;
pub const ULPI_TX_NEG_WIDTH: u32 = 0x26;
pub const ULPI_POLARITY_RECOVERY: u32 = 0x27;
/* Access Extended Register Set */
pub const ULPI_ACCESS_EXTENDED: u32 = 0x2f;
/* Vendor Specific */
pub const ULPI_VENDOR_SPECIFIC: u32 = 0x30;
/* Extended Registers */
pub const ULPI_EXT_VENDOR_SPECIFIC: u32 = 0x80;

/*
 * Register Bits
 */

/* Function Control */
pub const ULPI_FUNC_CTRL_XCVRSEL: u32 = 1u32 << 0;
pub const ULPI_FUNC_CTRL_XCVRSEL_MASK: u32 = 0x3;
pub const ULPI_FUNC_CTRL_HIGH_SPEED: u32 = 0x0;
pub const ULPI_FUNC_CTRL_FULL_SPEED: u32 = 0x1;
pub const ULPI_FUNC_CTRL_LOW_SPEED: u32 = 0x2;
pub const ULPI_FUNC_CTRL_FS4LS: u32 = 0x3;
pub const ULPI_FUNC_CTRL_TERMSELECT: u32 = 1u32 << 2;
pub const ULPI_FUNC_CTRL_OPMODE: u32 = 1u32 << 3;
pub const ULPI_FUNC_CTRL_OPMODE_MASK: u32 = 0x3 << 3;
pub const ULPI_FUNC_CTRL_OPMODE_NORMAL: u32 = 0x0 << 3;
pub const ULPI_FUNC_CTRL_OPMODE_NONDRIVING: u32 = 0x1 << 3;
pub const ULPI_FUNC_CTRL_OPMODE_DISABLE_NRZI: u32 = 0x2 << 3;
pub const ULPI_FUNC_CTRL_OPMODE_NOSYNC_NOEOP: u32 = 0x3 << 3;
pub const ULPI_FUNC_CTRL_RESET: u32 = 1u32 << 5;
pub const ULPI_FUNC_CTRL_SUSPENDM: u32 = 1u32 << 6;

/* Interface Control */
pub const ULPI_IFC_CTRL_6_PIN_SERIAL_MODE: u32 = 1u32 << 0;
pub const ULPI_IFC_CTRL_3_PIN_SERIAL_MODE: u32 = 1u32 << 1;
pub const ULPI_IFC_CTRL_CARKITMODE: u32 = 1u32 << 2;
pub const ULPI_IFC_CTRL_CLOCKSUSPENDM: u32 = 1u32 << 3;
pub const ULPI_IFC_CTRL_AUTORESUME: u32 = 1u32 << 4;
pub const ULPI_IFC_CTRL_EXTERNAL_VBUS: u32 = 1u32 << 5;
pub const ULPI_IFC_CTRL_PASSTHRU: u32 = 1u32 << 6;
pub const ULPI_IFC_CTRL_PROTECT_IFC_DISABLE: u32 = 1u32 << 7;

/* OTG Control */
pub const ULPI_OTG_CTRL_ID_PULLUP: u32 = 1u32 << 0;
pub const ULPI_OTG_CTRL_DP_PULLDOWN: u32 = 1u32 << 1;
pub const ULPI_OTG_CTRL_DM_PULLDOWN: u32 = 1u32 << 2;
pub const ULPI_OTG_CTRL_DISCHRGVBUS: u32 = 1u32 << 3;
pub const ULPI_OTG_CTRL_CHRGVBUS: u32 = 1u32 << 4;
pub const ULPI_OTG_CTRL_DRVVBUS: u32 = 1u32 << 5;
pub const ULPI_OTG_CTRL_DRVVBUS_EXT: u32 = 1u32 << 6;
pub const ULPI_OTG_CTRL_EXTVBUSIND: u32 = 1u32 << 7;

/* USB Interrupt Enable Rising,
 * USB Interrupt Enable Falling,
 * USB Interrupt Status and
 * USB Interrupt Latch
 */
pub const ULPI_INT_HOST_DISCONNECT: u32 = 1u32 << 0;
pub const ULPI_INT_VBUS_VALID: u32 = 1u32 << 1;
pub const ULPI_INT_SESS_VALID: u32 = 1u32 << 2;
pub const ULPI_INT_SESS_END: u32 = 1u32 << 3;
pub const ULPI_INT_IDGRD: u32 = 1u32 << 4;

/* Debug */
pub const ULPI_DEBUG_LINESTATE0: u32 = 1u32 << 0;
pub const ULPI_DEBUG_LINESTATE1: u32 = 1u32 << 1;

/* Carkit Control */
pub const ULPI_CARKIT_CTRL_CARKITPWR: u32 = 1u32 << 0;
pub const ULPI_CARKIT_CTRL_IDGNDDRV: u32 = 1u32 << 1;
pub const ULPI_CARKIT_CTRL_TXDEN: u32 = 1u32 << 2;
pub const ULPI_CARKIT_CTRL_RXDEN: u32 = 1u32 << 3;
pub const ULPI_CARKIT_CTRL_SPKLEFTEN: u32 = 1u32 << 4;
pub const ULPI_CARKIT_CTRL_SPKRIGHTEN: u32 = 1u32 << 5;
pub const ULPI_CARKIT_CTRL_MICEN: u32 = 1u32 << 6;

/* Carkit Interrupt Enable */
pub const ULPI_CARKIT_INT_EN_IDFLOAT_RISE: u32 = 1u32 << 0;
pub const ULPI_CARKIT_INT_EN_IDFLOAT_FALL: u32 = 1u32 << 1;
pub const ULPI_CARKIT_INT_EN_CARINTDET: u32 = 1u32 << 2;
pub const ULPI_CARKIT_INT_EN_DP_RISE: u32 = 1u32 << 3;
pub const ULPI_CARKIT_INT_EN_DP_FALL: u32 = 1u32 << 4;

/* Carkit Interrupt Status and
 * Carkit Interrupt Latch
 */
pub const ULPI_CARKIT_INT_IDFLOAT: u32 = 1u32 << 0;
pub const ULPI_CARKIT_INT_CARINTDET: u32 = 1u32 << 1;
pub const ULPI_CARKIT_INT_DP: u32 = 1u32 << 2;

/* Carkit Pulse Control*/
pub const ULPI_CARKIT_PLS_CTRL_TXPLSEN: u32 = 1u32 << 0;
pub const ULPI_CARKIT_PLS_CTRL_RXPLSEN: u32 = 1u32 << 1;
pub const ULPI_CARKIT_PLS_CTRL_SPKRLEFT_BIASEN: u32 = 1u32 << 2;
pub const ULPI_CARKIT_PLS_CTRL_SPKRRIGHT_BIASEN: u32 = 1u32 << 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
