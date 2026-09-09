/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) Microsoft Corporation
 */

// Dependencies supplied by the surrounding kernel bindings:
// linux/tee_drv.h, linux/tpm.h, and linux/uuid.h

/* The TAFs ID implemented in this TA */
pub const FTPM_OPTEE_TA_SUBMIT_COMMAND: u32 = 0;
pub const FTPM_OPTEE_TA_EMULATE_PPI: u32 = 1;

/* max. buffer size supported by fTPM */
pub const MAX_COMMAND_SIZE: usize = 4096;
pub const MAX_RESPONSE_SIZE: usize = 4096;

/**
 * struct ftpm_tee_private - fTPM's private data
 * @chip:     struct tpm_chip instance registered with tpm framework.
 * @session:  fTPM TA session identifier.
 * @ctx:      TEE context handler.
 * @shm:      Memory pool shared with fTPM TA in TEE.
 */
#[repr(C)]
pub struct ftpm_tee_private {
	pub chip: *mut tpm_chip,
	pub session: u32,
	pub ctx: *mut tee_context,
	pub shm: *mut tee_shm,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
