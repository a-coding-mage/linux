/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/interrupt.h, linux/mmc/host.h

pub struct device;

/* board specific SDHC data, optional.
 * If not present, a writable card with 3,3V is assumed.
 */
#[repr(C)]
pub struct imxmmc_platform_data {
	/* Return values for the get_ro callback should be:
	 *   0 for a read/write card
	 *   1 for a read-only card
	 *   -ENOSYS when not supported (equal to NULL callback)
	 *   or a negative errno value when something bad happened
	 */
	pub get_ro: Option<unsafe extern "C" fn(*mut device) -> ::core::ffi::c_int>,

	/* board specific hook to (de)initialize the SD slot.
	 * The board code can call 'handler' on a card detection
	 * change giving data as argument.
	 */
	pub init: Option<
		unsafe extern "C" fn(
			*mut device,
			irq_handler_t,
			*mut ::core::ffi::c_void,
		) -> ::core::ffi::c_int,
	>,
	pub exit: Option<unsafe extern "C" fn(*mut device, *mut ::core::ffi::c_void)>,

	/* available voltages. If not given, assume
	 * MMC_VDD_32_33 | MMC_VDD_33_34
	 */
	pub ocr_avail: ::core::ffi::c_uint,

	/* adjust slot voltage */
	pub setpower:
		Option<unsafe extern "C" fn(*mut device, ::core::ffi::c_uint)>,

	/* enable card detect using DAT3 */
	pub dat3_card_detect: ::core::ffi::c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
