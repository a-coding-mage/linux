/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2013-2014 Renesas Electronics Europe Ltd.
 * Author: Guennadi Liakhovetski <g.liakhovetski@gmx.de>
 */

/**
 * Use "#dma-cells = <2>;" with the second integer defining slave DMA flags:
 */
pub const NBPF_SLAVE_RQ_HIGH: u32 = 1;
pub const NBPF_SLAVE_RQ_LOW: u32 = 2;
pub const NBPF_SLAVE_RQ_LEVEL: u32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
