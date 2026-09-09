/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Marvell Octeon CN10K DPI driver
 *
 * Copyright (C) 2024 Marvell.
 *
 */

// The Linux ioctl encoding helper `_IOW` is supplied by the surrounding
// environment; this header translation preserves its use below.

pub const DPI_MAX_ENGINES: usize = 6;

#[repr(C)]
pub struct dpi_mps_mrrs_cfg {
    pub max_read_req_sz: u16, /* Max read request size */
    pub max_payload_sz: u16,  /* Max payload size */
    pub port: u16, /* Ebus port */
    pub reserved: u16, /* Reserved */
}

#[repr(C)]
pub struct dpi_engine_cfg {
    pub fifo_mask: u64, /* FIFO size mask in KBytes */
    pub molr: [u16; DPI_MAX_ENGINES], /* Max outstanding load requests */
    pub update_molr: u16, /* '1' to update engine MOLR */
    pub reserved: u16, /* Reserved */
}

/* DPI ioctl numbers */
pub const DPI_MAGIC_NUM: u32 = 0xB8;

/* Set MPS & MRRS parameters */
pub const DPI_MPS_MRRS_CFG: _ = _IOW(DPI_MAGIC_NUM, 1, dpi_mps_mrrs_cfg);

/* Set Engine FIFO configuration */
pub const DPI_ENGINE_CFG: _ = _IOW(DPI_MAGIC_NUM, 2, dpi_engine_cfg);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
