// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Linux driver for TerraTec DMX 6Fire USB
 *
 * Firmware loader
 *
 * Author:	Torsten Schenk <torsten.schenk@zoho.com>
 * Created:	Jan 01, 2011
 * Copyright:	(C) Torsten Schenk
 */

// Linux kernel module firmware declarations
// MODULE_FIRMWARE("6fire/dmx6firel2.ihx");
// MODULE_FIRMWARE("6fire/dmx6fireap.ihx");
// MODULE_FIRMWARE("6fire/dmx6firecf.bin");

const FPGA_BUFSIZE: usize = 512;
const FPGA_EP: u32 = 2;

/*
 * wMaxPacketSize of pcm endpoints.
 * keep synced with rates_in_packet_size and rates_out_packet_size in pcm.c
 * fpp: frames per isopacket
 *
 * CAUTION: keep sizeof <= buffer[] in usb6fire_fw_init
 */
static EP_W_MAX_PACKET_SIZE: [u8; 12] = [
	0xe4, 0x00, 0xe4, 0x00, /* alt 1: 228 EP2 and EP6 (7 fpp) */
	0xa4, 0x01, 0xa4, 0x01, /* alt 2: 420 EP2 and EP6 (13 fpp)*/
	0x94, 0x01, 0x5c, 0x02  /* alt 3: 404 EP2 and 604 EP6 (25 fpp) */
];

static KNOWN_FW_VERSIONS: [[u8; 2]; 1] = [
	[0x03, 0x01]
];

#[repr(C)]
struct IhexRecord {
	address: u16,
	len: u8,
	data: [u8; 256],
	error: u8, /* true if an error occurred parsing this record */

	max_len: u8, /* maximum record length in whole ihex */

	/* private */
	txt_data: *const u8,
	txt_length: usize,
	txt_offset: usize, /* current position in txt_data */
}

// External types (opaque)
#[repr(C)]
struct UsbDevice;

#[repr(C)]
struct UsbInterface;

#[repr(C)]
struct FirmwareStruct {
	data: *const u8,
	size: usize,
}

// External kernel functions
extern "C" {
	fn hex_to_bin(ch: u8) -> i32;
	fn kmalloc_obj(size: usize) -> *mut core::ffi::c_void;
	fn kmalloc(size: usize, flags: i32) -> *mut core::ffi::c_void;
	fn kfree(ptr: *mut core::ffi::c_void);
	fn request_firmware(fw_p: *mut *const FirmwareStruct, name: *const u8, device: *mut core::ffi::c_void) -> i32;
	fn interface_to_usbdev(intf: *mut UsbInterface) -> *mut UsbDevice;
	fn usb_control_msg_send(dev: *mut UsbDevice, ep: u32, request: i32, requesttype: i32,
		value: i32, index: i32, data: *mut u8, len: i32, timeout: i32, mem_flags: i32) -> i32;
	fn usb_control_msg_recv(dev: *mut UsbDevice, ep: u32, request: i32, requesttype: i32,
		value: i32, index: i32, data: *mut u8, len: i32, timeout: i32, mem_flags: i32) -> i32;
	fn usb_bulk_msg(usb_dev: *mut UsbDevice, pipe: u32, data: *mut u8, len: i32,
		actual_length: *mut i32, timeout: i32) -> i32;
	fn usb_sndbulkpipe(dev: *mut UsbDevice, endpoint: u32) -> u32;
	fn bitrev8(byte: u8) -> u8;
	fn dev_err(dev: *const core::ffi::c_void, format: *const u8, ...);
	fn printk(fmt: *const u8, ...);
}

// Kernel constants (from external headers)
const EINVAL: i32 = -22;
const ENOMEM: i32 = -12;
const EIO: i32 = -5;
const GFP_KERNEL: i32 = 0xd0;
const USB_DIR_OUT: i32 = 0;
const USB_DIR_IN: i32 = 0x80;
const USB_TYPE_VENDOR: i32 = 0x40;
const USB_RECIP_DEVICE: i32 = 0;
const KERN_CONT: *const u8 = b"c\0" as *const u8;
const FW_NOT_READY: i32 = -2;

fn usb6fire_fw_ihex_hex(data: *const u8, crc: &mut u8) -> u8 {
	let mut val: u8 = 0;

	unsafe {
		let hval = hex_to_bin(*data);
		if hval >= 0 {
			val |= (hval as u8) << 4;
		}

		let hval = hex_to_bin(*data.add(1));
		if hval >= 0 {
			val |= hval as u8;
		}
	}

	*crc = crc.wrapping_add(val);
	val
}

/*
 * returns true if record is available, false otherwise.
 * iff an error occurred, false will be returned and record->error will be true.
 */
fn usb6fire_fw_ihex_next_record(record: &mut IhexRecord) -> bool {
	let mut crc: u8 = 0;
	let mut type_: u8;
	let mut i: usize;

	record.error = 0;

	/* find begin of record (marked by a colon) */
	unsafe {
		while record.txt_offset < record.txt_length
				&& *record.txt_data.add(record.txt_offset) != b':' {
			record.txt_offset += 1;
		}
	}
	if record.txt_offset == record.txt_length {
		return false;
	}

	/* number of characters needed for len, addr and type entries */
	record.txt_offset += 1;
	if record.txt_offset + 8 > record.txt_length {
		record.error = 1;
		return false;
	}

	unsafe {
		record.len = usb6fire_fw_ihex_hex(record.txt_data.add(record.txt_offset), &mut crc);
	}
	record.txt_offset += 2;
	unsafe {
		let val = usb6fire_fw_ihex_hex(record.txt_data.add(record.txt_offset), &mut crc);
		record.address = ((val as u16) << 8);
	}
	record.txt_offset += 2;
	unsafe {
		let val = usb6fire_fw_ihex_hex(record.txt_data.add(record.txt_offset), &mut crc);
		record.address |= val as u16;
	}
	record.txt_offset += 2;
	unsafe {
		type_ = usb6fire_fw_ihex_hex(record.txt_data.add(record.txt_offset), &mut crc);
	}
	record.txt_offset += 2;

	/* number of characters needed for data and crc entries */
	if record.txt_offset + 2 * (record.len as usize + 1) > record.txt_length {
		record.error = 1;
		return false;
	}
	i = 0;
	while i < record.len as usize {
		unsafe {
			record.data[i] = usb6fire_fw_ihex_hex(
					record.txt_data.add(record.txt_offset), &mut crc);
		}
		record.txt_offset += 2;
		i += 1;
	}
	unsafe {
		usb6fire_fw_ihex_hex(record.txt_data.add(record.txt_offset), &mut crc);
	}
	if crc != 0 {
		record.error = 1;
		return false;
	}

	if type_ == 1 || record.len == 0 {
		/* eof */
		false
	} else if type_ == 0 {
		true
	} else {
		record.error = 1;
		false
	}
}

fn usb6fire_fw_ihex_init(fw: *const FirmwareStruct, record: &mut IhexRecord) -> i32 {
	unsafe {
		record.txt_data = (*fw).data;
		record.txt_length = (*fw).size;
	}
	record.txt_offset = 0;
	record.max_len = 0;
	/* read all records, if loop ends, record->error indicates,
	 * whether ihex is valid. */
	while usb6fire_fw_ihex_next_record(record) {
		if record.len > record.max_len {
			record.max_len = record.len;
		}
	}
	if record.error != 0 {
		return EINVAL;
	}
	record.txt_offset = 0;
	0
}

fn usb6fire_fw_ezusb_write(device: *mut UsbDevice,
		type_: i32, value: i32, data: *mut u8, len: i32) -> i32 {
	unsafe {
		usb_control_msg_send(device, 0, type_,
					    USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
					    value, 0, data, len, 1000, GFP_KERNEL)
	}
}

fn usb6fire_fw_ezusb_read(device: *mut UsbDevice,
		type_: i32, value: i32, data: *mut u8, len: i32) -> i32 {
	unsafe {
		usb_control_msg_recv(device, 0, type_,
					    USB_DIR_IN | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
					    value, 0, data, len, 1000, GFP_KERNEL)
	}
}

fn usb6fire_fw_fpga_write(device: *mut UsbDevice,
		data: *mut u8, len: i32) -> i32 {
	let mut actual_len: i32 = 0;
	let ret: i32;

	unsafe {
		ret = usb_bulk_msg(device, usb_sndbulkpipe(device, FPGA_EP), data, len,
				&mut actual_len, 1000);
	}
	if ret < 0 {
		return ret;
	} else if actual_len != len {
		return EIO;
	}
	0
}

fn usb6fire_fw_ezusb_upload(
		intf: *mut UsbInterface, fwname: *const u8,
		postaddr: u32, postdata: *mut u8, postlen: u32) -> i32 {
	let mut ret: i32;
	let mut data: u8;
	let device: *mut UsbDevice;
	let rec: *mut IhexRecord;

	unsafe {
		device = interface_to_usbdev(intf);
		rec = kmalloc_obj(core::mem::size_of::<IhexRecord>()) as *mut IhexRecord;
	}

	if rec.is_null() {
		return ENOMEM;
	}

	let mut fw: *const FirmwareStruct = core::ptr::null();
	unsafe {
		ret = request_firmware(&mut fw, fwname, &(*device) as *const _ as *mut core::ffi::c_void);
	}
	if ret < 0 {
		unsafe {
			dev_err(&(*intf) as *const _ as *const core::ffi::c_void,
				b"error requesting ezusb firmware %s.\n\0" as *const u8, fwname);
			kfree(rec as *mut core::ffi::c_void);
		}
		return ret;
	}
	unsafe {
		ret = usb6fire_fw_ihex_init(fw, &mut *rec);
	}
	if ret < 0 {
		unsafe {
			dev_err(&(*intf) as *const _ as *const core::ffi::c_void,
				b"error validating ezusb firmware %s.\n\0" as *const u8, fwname);
			kfree(rec as *mut core::ffi::c_void);
		}
		return ret;
	}
	/* upload firmware image */
	data = 0x01; /* stop ezusb cpu */
	ret = usb6fire_fw_ezusb_write(device, 0xa0, 0xe600, &mut data, 1);
	if ret != 0 {
		unsafe {
			dev_err(&(*intf) as *const _ as *const core::ffi::c_void,
				b"unable to upload ezusb firmware %s: begin message.\n\0" as *const u8,
				fwname);
			kfree(rec as *mut core::ffi::c_void);
		}
		return ret;
	}

	unsafe {
		while usb6fire_fw_ihex_next_record(&mut *rec) { /* write firmware */
			ret = usb6fire_fw_ezusb_write(device, 0xa0, (*rec).address as i32,
					(*rec).data.as_mut_ptr(), (*rec).len as i32);
			if ret != 0 {
				dev_err(&(*intf) as *const _ as *const core::ffi::c_void,
					b"unable to upload ezusb firmware %s: data urb.\n\0" as *const u8,
					fwname);
				kfree(rec as *mut core::ffi::c_void);
				return ret;
			}
		}
	}

	if !postdata.is_null() { /* write data after firmware has been uploaded */
		ret = usb6fire_fw_ezusb_write(device, 0xa0, postaddr as i32,
				postdata, postlen as i32);
		if ret != 0 {
			unsafe {
				dev_err(&(*intf) as *const _ as *const core::ffi::c_void,
					b"unable to upload ezusb firmware %s: post urb.\n\0" as *const u8,
					fwname);
				kfree(rec as *mut core::ffi::c_void);
			}
			return ret;
		}
	}

	data = 0x00; /* resume ezusb cpu */
	ret = usb6fire_fw_ezusb_write(device, 0xa0, 0xe600, &mut data, 1);
	if ret != 0 {
		unsafe {
			dev_err(&(*intf) as *const _ as *const core::ffi::c_void,
				b"unable to upload ezusb firmware %s: end message.\n\0" as *const u8,
				fwname);
			kfree(rec as *mut core::ffi::c_void);
		}
		return ret;
	}
	unsafe {
		kfree(rec as *mut core::ffi::c_void);
	}
	0
}

fn usb6fire_fw_fpga_upload(
		intf: *mut UsbInterface, fwname: *const u8) -> i32 {
	let mut ret: i32;
	let mut i: usize;
	let device: *mut UsbDevice;
	let buffer: *mut u8;
	let mut c: *const u8;
	let end: *const u8;

	unsafe {
		device = interface_to_usbdev(intf);
		buffer = kmalloc(FPGA_BUFSIZE, GFP_KERNEL) as *mut u8;
	}

	if buffer.is_null() {
		return ENOMEM;
	}

	let mut fw: *const FirmwareStruct = core::ptr::null();
	unsafe {
		ret = request_firmware(&mut fw, fwname, &(*device) as *const _ as *mut core::ffi::c_void);
	}
	if ret < 0 {
		unsafe {
			dev_err(&(*intf) as *const _ as *const core::ffi::c_void, b"unable to get fpga firmware %s.\n\0" as *const u8,
					fwname);
			kfree(buffer as *mut core::ffi::c_void);
		}
		return EIO;
	}

	unsafe {
		c = (*fw).data;
		end = (*fw).data.add((*fw).size);
	}

	ret = usb6fire_fw_ezusb_write(device, 8, 0, core::ptr::null_mut(), 0);
	if ret != 0 {
		unsafe {
			dev_err(&(*intf) as *const _ as *const core::ffi::c_void,
				b"unable to upload fpga firmware: begin urb.\n\0" as *const u8);
			kfree(buffer as *mut core::ffi::c_void);
		}
		return ret;
	}

	unsafe {
		while c != end {
			i = 0;
			while c != end && i < FPGA_BUFSIZE {
				*buffer.add(i) = bitrev8(*c);
				c = c.add(1);
				i += 1;
			}

			ret = usb6fire_fw_fpga_write(device, buffer, i as i32);
			if ret < 0 {
				dev_err(&(*intf) as *const _ as *const core::ffi::c_void,
					b"unable to upload fpga firmware: fw urb.\n\0" as *const u8);
				kfree(buffer as *mut core::ffi::c_void);
				return ret;
			}
		}
	}

	ret = usb6fire_fw_ezusb_write(device, 9, 0, core::ptr::null_mut(), 0);
	if ret != 0 {
		unsafe {
			dev_err(&(*intf) as *const _ as *const core::ffi::c_void,
				b"unable to upload fpga firmware: end urb.\n\0" as *const u8);
			kfree(buffer as *mut core::ffi::c_void);
		}
		return ret;
	}
	unsafe {
		kfree(buffer as *mut core::ffi::c_void);
	}
	0
}

/* check, if the firmware version the devices has currently loaded
 * is known by this driver. 'version' needs to have 4 bytes version
 * info data. */
fn usb6fire_fw_check(intf: *mut UsbInterface, version: *const u8) -> i32 {
	let mut i: usize;

	for i in 0..KNOWN_FW_VERSIONS.len() {
		unsafe {
			if core::slice::from_raw_parts(version, 2) == KNOWN_FW_VERSIONS[i] {
				return 0;
			}
		}
	}

	unsafe {
		dev_err(&(*intf) as *const _ as *const core::ffi::c_void,
			b"invalid firmware version in device: %4ph. please reconnect to power. if this failure still happens, check your firmware installation.\0" as *const u8,
			version);
	}
	EINVAL
}

pub fn usb6fire_fw_init(intf: *mut UsbInterface) -> i32 {
	let mut i: usize;
	let mut ret: i32;
	let device: *mut UsbDevice;
	/* buffer: 8 receiving bytes from device and
	 * sizeof(EP_W_MAX_PACKET_SIZE) bytes for non-const copy */
	let mut buffer: [u8; 12] = [0; 12];

	unsafe {
		device = interface_to_usbdev(intf);
	}

	ret = usb6fire_fw_ezusb_read(device, 1, 0, buffer.as_mut_ptr(), 8);
	if ret != 0 {
		unsafe {
			dev_err(&(*intf) as *const _ as *const core::ffi::c_void,
				b"unable to receive device firmware state.\n\0" as *const u8);
		}
		return ret;
	}
	if buffer[0] != 0xeb || buffer[1] != 0xaa || buffer[2] != 0x55 {
		unsafe {
			dev_err(&(*intf) as *const _ as *const core::ffi::c_void,
				b"unknown device firmware state received from device:\0" as *const u8);
			i = 0;
			while i < 8 {
				printk(b"c\0" as *const u8, buffer[i]);
				i += 1;
			}
			printk(b"c\n\0" as *const u8);
		}
		return EIO;
	}
	/* do we need fpga loader ezusb firmware? */
	if buffer[3] == 0x01 {
		ret = usb6fire_fw_ezusb_upload(intf,
				b"6fire/dmx6firel2.ihx\0" as *const u8, 0, core::ptr::null_mut(), 0);
		if ret < 0 {
			return ret;
		}
		return FW_NOT_READY;
	}
	/* do we need fpga firmware and application ezusb firmware? */
	else if buffer[3] == 0x02 {
		ret = usb6fire_fw_check(intf, &buffer[4]);
		if ret < 0 {
			return ret;
		}
		ret = usb6fire_fw_fpga_upload(intf, b"6fire/dmx6firecf.bin\0" as *const u8);
		if ret < 0 {
			return ret;
		}
		unsafe {
			core::ptr::copy_nonoverlapping(
				EP_W_MAX_PACKET_SIZE.as_ptr(),
				buffer.as_mut_ptr(),
				EP_W_MAX_PACKET_SIZE.len()
			);
		}
		ret = usb6fire_fw_ezusb_upload(intf, b"6fire/dmx6fireap.ihx\0" as *const u8,
				0x0003, buffer.as_mut_ptr(), EP_W_MAX_PACKET_SIZE.len() as u32);
		if ret < 0 {
			return ret;
		}
		return FW_NOT_READY;
	}
	/* all fw loaded? */
	else if buffer[3] == 0x03 {
		return usb6fire_fw_check(intf, &buffer[4]);
	}
	/* unknown data? */
	else {
		unsafe {
			dev_err(&(*intf) as *const _ as *const core::ffi::c_void,
				b"unknown device firmware state received from device: \0" as *const u8);
			i = 0;
			while i < 8 {
				printk(b"c\0" as *const u8, buffer[i]);
				i += 1;
			}
			printk(b"c\n\0" as *const u8);
		}
		return EIO;
	}
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
