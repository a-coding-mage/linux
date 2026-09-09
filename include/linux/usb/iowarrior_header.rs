/* SPDX-License-Identifier: GPL-2.0 */

/* Linux USB iowarrior header translation. */

pub const CODEMERCS_MAGIC_NUMBER: u8 = 0xC0; /* like COde Mercenaries */

/* Define the ioctl commands for reading and writing data.
 *
 * `_IOW` is a C preprocessor ioctl-encoding macro supplied by the Linux
 * ioctl headers.  Its encoded values depend on those external definitions;
 * retain the declarations here for the corresponding future dependency.
 */
pub const IOW_WRITE: u32 = 0; // _IOW(CODEMERCS_MAGIC_NUMBER, 1, __u8 *)
pub const IOW_READ: u32 = 0; // _IOW(CODEMERCS_MAGIC_NUMBER, 2, __u8 *)

/*
   A struct for available device info which is read
   with the ioctl IOW_GETINFO.
   To be compatible with 2.4 userspace which didn't have an easy way to get
   this information.
*/
#[repr(C)]
pub struct iowarrior_info {
	/* vendor id : supposed to be USB_VENDOR_ID_CODEMERCS in all cases */
	pub vendor: u32,
	/* product id : depends on type of chip (USB_DEVICE_ID_CODEMERCS_X) */
	pub product: u32,
	/* the serial number of our chip (if a serial-number is not available
	 * this is empty string) */
	pub serial: [u8; 9],
	/* revision number of the chip */
	pub revision: u32,
	/* USB-speed of the device (0=UNKNOWN, 1=LOW, 2=FULL 3=HIGH) */
	pub speed: u32,
	/* power consumption of the device in mA */
	pub power: u32,
	/* the number of the endpoint */
	pub if_num: u32,
	/* size of the data-packets on this interface */
	pub report_size: u32,
}

/*
  Get some device-information (product-id , serial-number etc.)
  in order to identify a chip.

  `_IOR(CODEMERCS_MAGIC_NUMBER, 3, struct iowarrior_info)` is the original
  Linux ioctl encoding; its value is supplied by the external ioctl headers.
*/
pub const IOW_GETINFO: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
