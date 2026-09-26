// Rust translation of at_hdmac.c; external kernel dependencies remain unresolved.
/* Source-level declarations and register definitions retained from the C implementation. */
// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for the Atmel AHB DMA Controller (aka HDMA or DMAC on AT91 systems)
 *
 * Copyright (C) 2008 Atmel Corporation
 * Copyright (C) 2022 Microchip Technology, Inc. and its subsidiaries
 *
 * This supports the Atmel AHB DMA Controller found in several Atmel SoCs.
 * The only Atmel DMA Controller that is not covered by this driver is the one
 * found on AT91SAM9263.
 */

#include <dt-bindings/dma/at91.h>
#include <linux/bitfield.h>
#include <linux/clk.h>
#include <linux/dmaengine.h>
#include <linux/dmapool.h>
#include <linux/dma-mapping.h>
#include <linux/interrupt.h>
#include <linux/module.h>
#include <linux/of.h>
#include <linux/overflow.h>
#include <linux/of_platform.h>
#include <linux/of_dma.h>
#include <linux/platform_device.h>
#include <linux/slab.h>

#include "dmaengine.h"
#include "virt-dma.h"

/*
 * Glossary
 * --------
 *
 * at_hdmac		: Name of the ATmel AHB DMA Controller
 * at_dma_ / atdma	: ATmel DMA controller entity related
 * atc_	/ atchan	: ATmel DMA Channel entity related
 */

pub const AT_DMA_MAX_NR_CHANNELS: u32 = 8;

/* Global Configuration Register */
pub const AT_DMA_GCFG: u32 = 0x00;
#define AT_DMA_IF_BIGEND(i)	BIT((i))	/* AHB-Lite Interface i in Big-endian mode */
pub const AT_DMA_ARB_CFG: u32 = 1 << 4;  /* Arbiter mode. */

/* Controller Enable Register */
pub const AT_DMA_EN: u32 = 0x04;
pub const AT_DMA_ENABLE: u32 = 1 << 0;

/* Software Single Request Register */
pub const AT_DMA_SREQ: u32 = 0x08;
#define AT_DMA_SSREQ(x)		BIT((x) << 1)		/* Request a source single transfer on channel x */
#define AT_DMA_DSREQ(x)		BIT(1 + ((x) << 1))	/* Request a destination single transfer on channel x */

/* Software Chunk Transfer Request Register */
pub const AT_DMA_CREQ: u32 = 0x0c;
#define AT_DMA_SCREQ(x)		BIT((x) << 1)		/* Request a source chunk transfer on channel x */
#define AT_DMA_DCREQ(x)		BIT(1 + ((x) << 1))	/* Request a destination chunk transfer on channel x */

/* Software Last Transfer Flag Register */
pub const AT_DMA_LAST: u32 = 0x10;
#define AT_DMA_SLAST(x)		BIT((x) << 1)		/* This src rq is last tx of buffer on channel x */
#define AT_DMA_DLAST(x)		BIT(1 + ((x) << 1))	/* This dst rq is last tx of buffer on channel x */

/* Request Synchronization Register */
pub const AT_DMA_SYNC: u32 = 0x14;
#define AT_DMA_SYR(h)		BIT((h))		/* Synchronize handshake line h */

/* Error, Chained Buffer transfer completed and Buffer transfer completed Interrupt registers */
pub const AT_DMA_EBCIER: u32 = 0x18;  /* Enable register */
pub const AT_DMA_EBCIDR: u32 = 0x1c;  /* Disable register */
pub const AT_DMA_EBCIMR: u32 = 0x20;  /* Mask Register */
pub const AT_DMA_EBCISR: u32 = 0x24;  /* Status Register */
pub const AT_DMA_CBTC_OFFSET: u32 = 8;
pub const AT_DMA_ERR_OFFSET: u32 = 16;
#define AT_DMA_BTC(x)		BIT((x))
#define AT_DMA_CBTC(x)		BIT(AT_DMA_CBTC_OFFSET + (x))
#define AT_DMA_ERR(x)		BIT(AT_DMA_ERR_OFFSET + (x))

/* Channel Handler Enable Register */
pub const AT_DMA_CHER: u32 = 0x28;
#define AT_DMA_ENA(x)		BIT((x))
#define AT_DMA_SUSP(x)		BIT(8 + (x))
#define AT_DMA_KEEP(x)		BIT(24 + (x))

/* Channel Handler Disable Register */
pub const AT_DMA_CHDR: u32 = 0x2c;
#define AT_DMA_DIS(x)		BIT(x)
#define AT_DMA_RES(x)		BIT(8 + (x))

/* Channel Handler Status Register */
pub const AT_DMA_CHSR: u32 = 0x30;
#define AT_DMA_EMPT(x)		BIT(16 + (x))
#define AT_DMA_STAL(x)		BIT(24 + (x))

/* Channel registers base address */
pub const AT_DMA_CH_REGS_BASE: u32 = 0x3c;
#define ch_regs(x)		(AT_DMA_CH_REGS_BASE + (x) * 0x28) /* Channel x base addr */

/* Hardware register offset for each channel */
pub const ATC_SADDR_OFFSET: u32 = 0x00;  /* Source Address Register */
pub const ATC_DADDR_OFFSET: u32 = 0x04;  /* Destination Address Register */
pub const ATC_DSCR_OFFSET: u32 = 0x08;  /* Descriptor Address Register */
pub const ATC_CTRLA_OFFSET: u32 = 0x0c;  /* Control A Register */
pub const ATC_CTRLB_OFFSET: u32 = 0x10;  /* Control B Register */
pub const ATC_CFG_OFFSET: u32 = 0x14;  /* Configuration Register */
pub const ATC_SPIP_OFFSET: u32 = 0x18;  /* Src PIP Configuration Register */
pub const ATC_DPIP_OFFSET: u32 = 0x1c;  /* Dst PIP Configuration Register */


/* Bitfield definitions */

/* Bitfields in DSCR */
#define ATC_DSCR_IF		GENMASK(1, 0)	/* Dsc feched via AHB-Lite Interface */

/* Bitfields in CTRLA */
#define ATC_BTSIZE_MAX		GENMASK(15, 0)	/* Maximum Buffer Transfer Size */
#define ATC_BTSIZE		GENMASK(15, 0)	/* Buffer Transfer Size */
#define ATC_SCSIZE		GENMASK(18, 16)	/* Source Chunk Transfer Size */
#define ATC_DCSIZE		GENMASK(22, 20)	/* Destination Chunk Transfer Size */
#define ATC_SRC_WIDTH		GENMASK(25, 24)	/* Source Single Transfer Size */
#define ATC_DST_WIDTH		GENMASK(29, 28)	/* Destination Single Transfer Size */
pub const ATC_DONE: u64 = 1 << 31;  /* Tx Done (only written back in descriptor) */

/* Bitfields in CTRLB */
#define ATC_SIF			GENMASK(1, 0)	/* Src tx done via AHB-Lite Interface i */
#define ATC_DIF			GENMASK(5, 4)	/* Dst tx done via AHB-Lite Interface i */
pub const AT_DMA_MEM_IF: u32 = 0x0;  /* interface 0 as memory interface */
pub const AT_DMA_PER_IF: u32 = 0x1;  /* interface 1 as peripheral interface */
pub const ATC_SRC_PIP: u32 = 1 << 8;  /* Source Picture-in-Picture enabled */
pub const ATC_DST_PIP: u32 = 1 << 12;  /* Destination Picture-in-Picture enabled */
pub const ATC_SRC_DSCR_DIS: u32 = 1 << 16;  /* Src Descriptor fetch disable */
pub const ATC_DST_DSCR_DIS: u32 = 1 << 20;  /* Dst Descriptor fetch disable */
#define ATC_FC			GENMASK(23, 21)	/* Choose Flow Controller */
pub const ATC_FC_MEM2MEM: u32 = 0x0;  /* Mem-to-Mem (DMA) */
pub const ATC_FC_MEM2PER: u32 = 0x1;  /* Mem-to-Periph (DMA) */
pub const ATC_FC_PER2MEM: u32 = 0x2;  /* Periph-to-Mem (DMA) */
pub const ATC_FC_PER2PER: u32 = 0x3;  /* Periph-to-Periph (DMA) */
pub const ATC_FC_PER2MEM_PER: u32 = 0x4;  /* Periph-to-Mem (Peripheral) */
pub const ATC_FC_MEM2PER_PER: u32 = 0x5;  /* Mem-to-Periph (Peripheral) */
pub const ATC_FC_PER2PER_SRCPER: u32 = 0x6;  /* Periph-to-Periph (Src Peripheral) */
pub const ATC_FC_PER2PER_DSTPER: u32 = 0x7;  /* Periph-to-Periph (Dst Peripheral) */
#define ATC_SRC_ADDR_MODE	GENMASK(25, 24)
pub const ATC_SRC_ADDR_MODE_INCR: u32 = 0x0;  /* Incrementing Mode */
pub const ATC_SRC_ADDR_MODE_DECR: u32 = 0x1;  /* Decrementing Mode */
pub const ATC_SRC_ADDR_MODE_FIXED: u32 = 0x2;  /* Fixed Mode */
#define ATC_DST_ADDR_MODE	GENMASK(29, 28)
pub const ATC_DST_ADDR_MODE_INCR: u32 = 0x0;  /* Incrementing Mode */
pub const ATC_DST_ADDR_MODE_DECR: u32 = 0x1;  /* Decrementing Mode */
pub const ATC_DST_ADDR_MODE_FIXED: u32 = 0x2;  /* Fixed Mode */
pub const ATC_IEN: u32 = 1 << 30;  /* BTC interrupt enable (active low) */
pub const ATC_AUTO: u64 = 1 << 31;  /* Auto multiple buffer tx enable */

/* Bitfields in CFG */
#define ATC_SRC_PER		GENMASK(3, 0)	/* Channel src rq associated with periph handshaking ifc h */
#define ATC_DST_PER		GENMASK(7, 4)	/* Channel dst rq associated with periph handshaking ifc h */
pub const ATC_SRC_REP: u32 = 1 << 8;  /* Source Replay Mod */
pub const ATC_SRC_H2SEL: u32 = 1 << 9;  /* Source Handshaking Mod */
#define ATC_SRC_PER_MSB		GENMASK(11, 10)	/* Channel src rq (most significant bits) */
pub const ATC_DST_REP: u32 = 1 << 12;  /* Destination Replay Mod */
pub const ATC_DST_H2SEL: u32 = 1 << 13;  /* Destination Handshaking Mod */
#define ATC_DST_PER_MSB		GENMASK(15, 14)	/* Channel dst rq (most significant bits) */
pub const ATC_SOD: u32 = 1 << 16;  /* Stop On Done */
pub const ATC_LOCK_IF: u32 = 1 << 20;  /* Interface Lock */
pub const ATC_LOCK_B: u32 = 1 << 21;  /* AHB Bus Lock */
pub const ATC_LOCK_IF_L: u32 = 1 << 22;  /* Master Interface Arbiter Lock */
#define ATC_AHB_PROT		GENMASK(26, 24)	/* AHB Protection */
#define ATC_FIFOCFG		GENMASK(29, 28)	/* FIFO Request Configuration */
pub const ATC_FIFOCFG_LARGESTBURST: u32 = 0x0;
pub const ATC_FIFOCFG_HALFFIFO: u32 = 0x1;
pub const ATC_FIFOCFG_ENOUGHSPACE: u32 = 0x2;

/* Bitfields in SPIP */
#define ATC_SPIP_HOLE		GENMASK(15, 0)
#define ATC_SPIP_BOUNDARY	GENMASK(25, 16)

/* Bitfields in DPIP */
#define ATC_DPIP_HOLE		GENMASK(15, 0)
#define ATC_DPIP_BOUNDARY	GENMASK(25, 16)

#define ATC_PER_MSB		GENMASK(5, 4)	/* Extract MSBs of a handshaking identifier */
#define ATC_SRC_PER_ID(id)					       \
	({ typeof(id) _id = (id);				       \
	   FIELD_PREP(ATC_SRC_PER_MSB, FIELD_GET(ATC_PER_MSB, _id)) |  \
	   FIELD_PREP(ATC_SRC_PER, _id); })
#define ATC_DST_PER_ID(id)					       \
	({ typeof(id) _id = (id);				       \
	   FIELD_PREP(ATC_DST_PER_MSB, FIELD_GET(ATC_PER_MSB, _id)) |  \
	   FIELD_PREP(ATC_DST_PER, _id); })



/*--  descriptors  -----------------------------------------------------*/

/* LLI == Linked List Item; aka DMA buffer descriptor */
struct at_lli {
	/* values that are not changed by hardware */
	u32 saddr;
	u32 daddr;
	/* value that may get written back: */
	u32 ctrla;
	/* more values that are not changed by hardware */
	u32 ctrlb;
	u32 dscr;	/* chain to next lli */
};

/**
 * struct atdma_sg - atdma scatter gather entry
 * @len: length of the current Linked List Item.
 * @lli: linked list item that is passed to the DMA controller
 * @lli_phys: physical address of the LLI.
 */
struct atdma_sg {
	core::ffi::c_uint len;
	struct at_lli *lli;
	dma_addr_t lli_phys;
};

/**

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
