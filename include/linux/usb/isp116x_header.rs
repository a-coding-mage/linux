/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Board initialization code should put one of these into dev->platform_data
 * and place the isp116x onto platform_bus.
 */

/* C header guard: __LINUX_USB_ISP116X_H */

/* Supplied by the platform/device subsystem. */
pub struct device;

#[repr(C)]
pub struct isp116x_platform_data {
	/* The C declaration uses five one-bit unsigned bit-fields.  Rust has no
	 * native bit-field syntax, so they are represented by their containing
	 * unsigned storage unit and exposed through the same logical flags below.
	 */
	pub flags: u32,
	/* Enable internal resistors on downstream ports: bit 0 */
	/* On-chip overcurrent detection: bit 1 */
	/* INT output polarity: bit 2 */
	/* INT edge or level triggered: bit 3 */
	/* Enable wakeup by devices on usb bus (e.g. wakeup
	 * by attachment/detachment or by device activity
	 * such as moving a mouse). When chosen, this option
	 * prevents stopping internal clock, increasing
	 * thereby power consumption in suspended state: bit 4 */
	/* Inter-io delay (ns). The chip is picky about access timings; it
	 * expects at least:
	 * 150ns delay between consecutive accesses to DATA_REG,
	 * 300ns delay between access to ADDR_REG and DATA_REG
	 * OE, WE MUST NOT be changed during these intervals
	 */
	pub delay: Option<unsafe extern "C" fn(dev: *mut device, delay: i32)>,
}

pub const ISP116X_SEL15KRES: u32 = 1 << 0;
pub const ISP116X_OC_ENABLE: u32 = 1 << 1;
pub const ISP116X_INT_ACT_HIGH: u32 = 1 << 2;
pub const ISP116X_INT_EDGE_TRIGGERED: u32 = 1 << 3;
pub const ISP116X_REMOTE_WAKEUP_ENABLE: u32 = 1 << 4;

/* C bit-field accessors, preserving one-bit unsigned storage semantics. */
impl isp116x_platform_data {
	pub const fn sel15Kres(&self) -> bool {
		(self.flags & ISP116X_SEL15KRES) != 0
	}
	pub fn set_sel15Kres(&mut self, value: bool) {
		self.flags = (self.flags & !ISP116X_SEL15KRES) | ((value as u32) << 0);
	}
	pub const fn oc_enable(&self) -> bool {
		(self.flags & ISP116X_OC_ENABLE) != 0
	}
	pub fn set_oc_enable(&mut self, value: bool) {
		self.flags = (self.flags & !ISP116X_OC_ENABLE) | ((value as u32) << 1);
	}
	pub const fn int_act_high(&self) -> bool {
		(self.flags & ISP116X_INT_ACT_HIGH) != 0
	}
	pub fn set_int_act_high(&mut self, value: bool) {
		self.flags = (self.flags & !ISP116X_INT_ACT_HIGH) | ((value as u32) << 2);
	}
	pub const fn int_edge_triggered(&self) -> bool {
		(self.flags & ISP116X_INT_EDGE_TRIGGERED) != 0
	}
	pub fn set_int_edge_triggered(&mut self, value: bool) {
		self.flags = (self.flags & !ISP116X_INT_EDGE_TRIGGERED) | ((value as u32) << 3);
	}
	pub const fn remote_wakeup_enable(&self) -> bool {
		(self.flags & ISP116X_REMOTE_WAKEUP_ENABLE) != 0
	}
	pub fn set_remote_wakeup_enable(&mut self, value: bool) {
		self.flags = (self.flags & !ISP116X_REMOTE_WAKEUP_ENABLE) | ((value as u32) << 4);
	}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
