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
 * /

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
 * /

#define	AT_DMA_MAX_NR_CHANNELS	8

/* Global Configuration Register * /
#define AT_DMA_GCFG		0x00
#define AT_DMA_IF_BIGEND(i)	BIT((i))	/* AHB-Lite Interface i in Big-endian mode * /
#define AT_DMA_ARB_CFG		BIT(4)		/* Arbiter mode. * /

/* Controller Enable Register * /
#define AT_DMA_EN		0x04
#define AT_DMA_ENABLE		BIT(0)

/* Software Single Request Register * /
#define AT_DMA_SREQ		0x08
#define AT_DMA_SSREQ(x)		BIT((x) << 1)		/* Request a source single transfer on channel x * /
#define AT_DMA_DSREQ(x)		BIT(1 + ((x) << 1))	/* Request a destination single transfer on channel x * /

/* Software Chunk Transfer Request Register * /
#define AT_DMA_CREQ		0x0c
#define AT_DMA_SCREQ(x)		BIT((x) << 1)		/* Request a source chunk transfer on channel x * /
#define AT_DMA_DCREQ(x)		BIT(1 + ((x) << 1))	/* Request a destination chunk transfer on channel x * /

/* Software Last Transfer Flag Register * /
#define AT_DMA_LAST		0x10
#define AT_DMA_SLAST(x)		BIT((x) << 1)		/* This src rq is last tx of buffer on channel x * /
#define AT_DMA_DLAST(x)		BIT(1 + ((x) << 1))	/* This dst rq is last tx of buffer on channel x * /

/* Request Synchronization Register * /
#define AT_DMA_SYNC		0x14
#define AT_DMA_SYR(h)		BIT((h))		/* Synchronize handshake line h * /

/* Error, Chained Buffer transfer completed and Buffer transfer completed Interrupt registers * /
#define AT_DMA_EBCIER		0x18			/* Enable register * /
#define AT_DMA_EBCIDR		0x1c			/* Disable register * /
#define AT_DMA_EBCIMR		0x20			/* Mask Register * /
#define AT_DMA_EBCISR		0x24			/* Status Register * /
#define AT_DMA_CBTC_OFFSET	8
#define AT_DMA_ERR_OFFSET	16
#define AT_DMA_BTC(x)		BIT((x))
#define AT_DMA_CBTC(x)		BIT(AT_DMA_CBTC_OFFSET + (x))
#define AT_DMA_ERR(x)		BIT(AT_DMA_ERR_OFFSET + (x))

/* Channel Handler Enable Register * /
#define AT_DMA_CHER		0x28
#define AT_DMA_ENA(x)		BIT((x))
#define AT_DMA_SUSP(x)		BIT(8 + (x))
#define AT_DMA_KEEP(x)		BIT(24 + (x))

/* Channel Handler Disable Register * /
#define AT_DMA_CHDR		0x2c
#define AT_DMA_DIS(x)		BIT(x)
#define AT_DMA_RES(x)		BIT(8 + (x))

/* Channel Handler Status Register * /
#define AT_DMA_CHSR		0x30
#define AT_DMA_EMPT(x)		BIT(16 + (x))
#define AT_DMA_STAL(x)		BIT(24 + (x))

/* Channel registers base address * /
#define AT_DMA_CH_REGS_BASE	0x3c
#define ch_regs(x)		(AT_DMA_CH_REGS_BASE + (x) * 0x28) /* Channel x base addr * /

/* Hardware register offset for each channel * /
#define ATC_SADDR_OFFSET	0x00	/* Source Address Register * /
#define ATC_DADDR_OFFSET	0x04	/* Destination Address Register * /
#define ATC_DSCR_OFFSET		0x08	/* Descriptor Address Register * /
#define ATC_CTRLA_OFFSET	0x0c	/* Control A Register * /
#define ATC_CTRLB_OFFSET	0x10	/* Control B Register * /
#define ATC_CFG_OFFSET		0x14	/* Configuration Register * /
#define ATC_SPIP_OFFSET		0x18	/* Src PIP Configuration Register * /
#define ATC_DPIP_OFFSET		0x1c	/* Dst PIP Configuration Register * /


/* Bitfield definitions * /

/* Bitfields in DSCR * /
#define ATC_DSCR_IF		GENMASK(1, 0)	/* Dsc feched via AHB-Lite Interface * /

/* Bitfields in CTRLA * /
#define ATC_BTSIZE_MAX		GENMASK(15, 0)	/* Maximum Buffer Transfer Size * /
#define ATC_BTSIZE		GENMASK(15, 0)	/* Buffer Transfer Size * /
#define ATC_SCSIZE		GENMASK(18, 16)	/* Source Chunk Transfer Size * /
#define ATC_DCSIZE		GENMASK(22, 20)	/* Destination Chunk Transfer Size * /
#define ATC_SRC_WIDTH		GENMASK(25, 24)	/* Source Single Transfer Size * /
#define ATC_DST_WIDTH		GENMASK(29, 28)	/* Destination Single Transfer Size * /
#define ATC_DONE		BIT(31)	/* Tx Done (only written back in descriptor) * /

/* Bitfields in CTRLB * /
#define ATC_SIF			GENMASK(1, 0)	/* Src tx done via AHB-Lite Interface i * /
#define ATC_DIF			GENMASK(5, 4)	/* Dst tx done via AHB-Lite Interface i * /
#define AT_DMA_MEM_IF		0x0		/* interface 0 as memory interface * /
#define AT_DMA_PER_IF		0x1		/* interface 1 as peripheral interface * /
#define ATC_SRC_PIP		BIT(8)		/* Source Picture-in-Picture enabled * /
#define ATC_DST_PIP		BIT(12)		/* Destination Picture-in-Picture enabled * /
#define ATC_SRC_DSCR_DIS	BIT(16)		/* Src Descriptor fetch disable * /
#define ATC_DST_DSCR_DIS	BIT(20)		/* Dst Descriptor fetch disable * /
#define ATC_FC			GENMASK(23, 21)	/* Choose Flow Controller * /
#define ATC_FC_MEM2MEM		0x0		/* Mem-to-Mem (DMA) * /
#define ATC_FC_MEM2PER		0x1		/* Mem-to-Periph (DMA) * /
#define ATC_FC_PER2MEM		0x2		/* Periph-to-Mem (DMA) * /
#define ATC_FC_PER2PER		0x3		/* Periph-to-Periph (DMA) * /
#define ATC_FC_PER2MEM_PER	0x4		/* Periph-to-Mem (Peripheral) * /
#define ATC_FC_MEM2PER_PER	0x5		/* Mem-to-Periph (Peripheral) * /
#define ATC_FC_PER2PER_SRCPER	0x6		/* Periph-to-Periph (Src Peripheral) * /
#define ATC_FC_PER2PER_DSTPER	0x7		/* Periph-to-Periph (Dst Peripheral) * /
#define ATC_SRC_ADDR_MODE	GENMASK(25, 24)
#define ATC_SRC_ADDR_MODE_INCR	0x0		/* Incrementing Mode * /
#define ATC_SRC_ADDR_MODE_DECR	0x1		/* Decrementing Mode * /
#define ATC_SRC_ADDR_MODE_FIXED	0x2		/* Fixed Mode * /
#define ATC_DST_ADDR_MODE	GENMASK(29, 28)
#define ATC_DST_ADDR_MODE_INCR	0x0		/* Incrementing Mode * /
#define ATC_DST_ADDR_MODE_DECR	0x1		/* Decrementing Mode * /
#define ATC_DST_ADDR_MODE_FIXED	0x2		/* Fixed Mode * /
#define ATC_IEN			BIT(30)		/* BTC interrupt enable (active low) * /
#define ATC_AUTO		BIT(31)		/* Auto multiple buffer tx enable * /

/* Bitfields in CFG * /
#define ATC_SRC_PER		GENMASK(3, 0)	/* Channel src rq associated with periph handshaking ifc h * /
#define ATC_DST_PER		GENMASK(7, 4)	/* Channel dst rq associated with periph handshaking ifc h * /
#define ATC_SRC_REP		BIT(8)		/* Source Replay Mod * /
#define ATC_SRC_H2SEL		BIT(9)		/* Source Handshaking Mod * /
#define ATC_SRC_PER_MSB		GENMASK(11, 10)	/* Channel src rq (most significant bits) * /
#define ATC_DST_REP		BIT(12)		/* Destination Replay Mod * /
#define ATC_DST_H2SEL		BIT(13)		/* Destination Handshaking Mod * /
#define ATC_DST_PER_MSB		GENMASK(15, 14)	/* Channel dst rq (most significant bits) * /
#define ATC_SOD			BIT(16)		/* Stop On Done * /
#define ATC_LOCK_IF		BIT(20)		/* Interface Lock * /
#define ATC_LOCK_B		BIT(21)		/* AHB Bus Lock * /
#define ATC_LOCK_IF_L		BIT(22)		/* Master Interface Arbiter Lock * /
#define ATC_AHB_PROT		GENMASK(26, 24)	/* AHB Protection * /
#define ATC_FIFOCFG		GENMASK(29, 28)	/* FIFO Request Configuration * /
#define ATC_FIFOCFG_LARGESTBURST	0x0
#define ATC_FIFOCFG_HALFFIFO		0x1
#define ATC_FIFOCFG_ENOUGHSPACE		0x2

/* Bitfields in SPIP * /
#define ATC_SPIP_HOLE		GENMASK(15, 0)
#define ATC_SPIP_BOUNDARY	GENMASK(25, 16)

/* Bitfields in DPIP * /
#define ATC_DPIP_HOLE		GENMASK(15, 0)
#define ATC_DPIP_BOUNDARY	GENMASK(25, 16)

#define ATC_PER_MSB		GENMASK(5, 4)	/* Extract MSBs of a handshaking identifier * /
#define ATC_SRC_PER_ID(id)					       \
	({ typeof(id) _id = (id);				       \
	   FIELD_PREP(ATC_SRC_PER_MSB, FIELD_GET(ATC_PER_MSB, _id)) |  \
	   FIELD_PREP(ATC_SRC_PER, _id); })
#define ATC_DST_PER_ID(id)					       \
	({ typeof(id) _id = (id);				       \
	   FIELD_PREP(ATC_DST_PER_MSB, FIELD_GET(ATC_PER_MSB, _id)) |  \
	   FIELD_PREP(ATC_DST_PER, _id); })



/*--  descriptors  -----------------------------------------------------* /

/* LLI == Linked List Item; aka DMA buffer descriptor * /
struct at_lli {
	/* values that are not changed by hardware * /
	u32 saddr;
	u32 daddr;
	/* value that may get written back: * /
	u32 ctrla;
	/* more values that are not changed by hardware * /
	u32 ctrlb;
	u32 dscr;	/* chain to next lli * /
};

/**
 * struct atdma_sg - atdma scatter gather entry
 * @len: length of the current Linked List Item.
 * @lli: linked list item that is passed to the DMA controller
 * @lli_phys: physical address of the LLI.
 * /
struct atdma_sg {
	unsigned int len;
	struct at_lli *lli;
	dma_addr_t lli_phys;
};

/**

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
