/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Under the kernel build, C defines kernel_ulong_t as unsigned long.
 * Rust's usize has the corresponding target-dependent width.
 */
pub type kernel_ulong_t = usize;

/*
 * Device table entry for "new style" table-driven USB drivers.
 * User mode code can read these tables to choose which modules to load.
 * Declare the table as a MODULE_DEVICE_TABLE.
 *
 * A probe() parameter will point to a matching entry from this table.
 * Use the driver_info field for each match to hold information tied
 * to that match: device quirks, etc.
 *
 * Terminate the driver's table with an all-zeroes entry.
 * Use the flag values to control which fields are compared.
 */

/**
 * struct usb_device_id - identifies USB devices for probing and hotplugging
 * @match_flags: Bit mask controlling which of the other fields are used to
 * match against new devices. Any field except for driver_info may be
 * used, although some only make sense in conjunction with other fields.
 * This is usually set by a USB_DEVICE_*() macro, which sets all
 * other fields in this structure except for driver_info.
 * @idVendor: USB vendor ID for a device; numbers are assigned
 * by the USB forum to its members.
 * @idProduct: Vendor-assigned product ID.
 * @bcdDevice_lo: Low end of range of vendor-assigned product version numbers.
 * This is also used to identify individual product versions, for
 * a range consisting of a single device.
 * @bcdDevice_hi: High end of version number range. The range of product
 * versions is inclusive.
 * @bDeviceClass: Class of device; numbers are assigned
 * by the USB forum. Products may choose to implement classes,
 * or be vendor-specific. Device classes specify behavior of all
 * the interfaces on a device.
 * @bDeviceSubClass: Subclass of device; associated with bDeviceClass.
 * @bDeviceProtocol: Protocol of device; associated with bDeviceClass.
 * @bInterfaceClass: Class of interface; numbers are assigned
 * by the USB forum. Products may choose to implement classes,
 * or be vendor-specific. Interface classes specify behavior only
 * of a given interface; other interfaces may support other classes.
 * @bInterfaceSubClass: Subclass of interface; associated with bInterfaceClass.
 * @bInterfaceProtocol: Protocol of interface; associated with bInterfaceClass.
 * @bInterfaceNumber: Number of interface; composite devices may use
 * fixed interface numbers to differentiate between vendor-specific
 * interfaces.
 * @driver_info: Holds information used by the driver. Usually it holds
 * a pointer to a descriptor understood by the driver, or perhaps
 * device flags.
 *
 * In most cases, drivers will create a table of device IDs by using
 * USB_DEVICE(), or similar macros designed for that purpose.
 * They will then export it to userspace using MODULE_DEVICE_TABLE(),
 * and provide it to the USB core through their usb_driver structure.
 *
 * See the usb_match_id() function for information about how matches are
 * performed. Briefly, you will normally use one of several macros to help
 * construct these entries. Each entry you provide will either identify
 * one or more specific products, or will identify a class of products
 * which have agreed to behave the same. You should put the more specific
 * matches towards the beginning of your table, so that driver_info can
 * record quirks of specific products.
 */
#[repr(C)]
pub struct usb_device_id {
    /* which fields to match against? */
    pub match_flags: u16,

    /* Used for product specific matches; range is inclusive */
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice_lo: u16,
    pub bcdDevice_hi: u16,

    /* Used for device class matches */
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,

    /* Used for interface class matches */
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,

    /* Used for vendor-specific interface matches */
    pub bInterfaceNumber: u8,

    /* not matched against */
    pub driver_info: kernel_ulong_t,
}

/* Some useful macros to use to create struct usb_device_id */
pub const USB_DEVICE_ID_MATCH_VENDOR: u16 = 0x0001;
pub const USB_DEVICE_ID_MATCH_PRODUCT: u16 = 0x0002;
pub const USB_DEVICE_ID_MATCH_DEV_LO: u16 = 0x0004;
pub const USB_DEVICE_ID_MATCH_DEV_HI: u16 = 0x0008;
pub const USB_DEVICE_ID_MATCH_DEV_CLASS: u16 = 0x0010;
pub const USB_DEVICE_ID_MATCH_DEV_SUBCLASS: u16 = 0x0020;
pub const USB_DEVICE_ID_MATCH_DEV_PROTOCOL: u16 = 0x0040;
pub const USB_DEVICE_ID_MATCH_INT_CLASS: u16 = 0x0080;
pub const USB_DEVICE_ID_MATCH_INT_SUBCLASS: u16 = 0x0100;
pub const USB_DEVICE_ID_MATCH_INT_PROTOCOL: u16 = 0x0200;
pub const USB_DEVICE_ID_MATCH_INT_NUMBER: u16 = 0x0400;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
