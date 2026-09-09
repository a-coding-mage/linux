/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2015-2018, Intel Corporation.
 */

// Dependency types and the BIT macro are supplied by the surrounding kernel
// environment; their declarations are intentionally not implemented here.

pub const KCS_BMC_EVENT_TYPE_OBE: u32 = 1 << 0;
pub const KCS_BMC_EVENT_TYPE_IBF: u32 = 1 << 1;

pub const KCS_BMC_STR_OBF: u32 = 1 << 0;
pub const KCS_BMC_STR_IBF: u32 = 1 << 1;
pub const KCS_BMC_STR_CMD_DAT: u32 = 1 << 3;

/* IPMI 2.0 - 9.5, KCS Interface Registers
 * @idr: Input Data Register
 * @odr: Output Data Register
 * @str: Status Register
 */
#[repr(C)]
pub struct kcs_ioreg {
	pub idr: u32,
	pub odr: u32,
	pub str_: u32,
}

pub enum kcs_bmc_device_ops {}
pub enum kcs_bmc_client {}

#[repr(C)]
pub struct kcs_bmc_device {
	pub entry: list_head,

	pub dev: *mut device,
	pub channel: u32,

	pub ioreg: kcs_ioreg,

	pub ops: *const kcs_bmc_device_ops,

	pub lock: spinlock_t,
	pub client: *mut kcs_bmc_client,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
