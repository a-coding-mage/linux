/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * sata_promise.h - Promise SATA common definitions and inline funcs
 *
 * Translated from the C header.
 */

#[repr(u32)]
pub enum PdcPacketBits {
    PDC_PKT_READ = 1 << 2,
    PDC_PKT_NODATA = 1 << 3,

    PDC_PKT_SIZEMASK = (1 << 7) | (1 << 6) | (1 << 5),
    PDC_PKT_CLEAR_BSY = 1 << 4,
    PDC_PKT_WAIT_DRDY = (1 << 3) | (1 << 4),
    PDC_LAST_REG = 1 << 3,

    PDC_REG_DEVCTL = (1 << 3) | (1 << 2) | (1 << 1),
}

pub unsafe fn pdc_pkt_header(
    tf: *mut ata_taskfile,
    sg_table: dma_addr_t,
    devno: u32,
    buf: *mut u8,
) -> u32 {
    let dev_reg: u8;
    let buf32 = buf as *mut u32;
    let protocol = (*tf).protocol;

    /* set control bits (byte 0), zero delay seq id (byte 3),
     * and seq id (byte 2)
     */
    match protocol {
        ATA_PROT_DMA => {
            if ((*tf).flags & ATA_TFLAG_WRITE) == 0 {
                buf32.write_unaligned((PdcPacketBits::PDC_PKT_READ as u32).to_le());
            } else {
                buf32.write_unaligned(0);
            }
        }
        ATA_PROT_NODATA => {
            buf32.write_unaligned((PdcPacketBits::PDC_PKT_NODATA as u32).to_le());
        }
        _ => {
            BUG();
        }
    }

    buf32.add(1).write_unaligned((sg_table as u32).to_le()); /* S/G table addr */
    buf32.add(2).write_unaligned(0); /* no next-packet */

    if devno == 0 {
        dev_reg = ATA_DEVICE_OBS;
    } else {
        dev_reg = ATA_DEVICE_OBS | ATA_DEV1;
    }

    /* select device */
    *buf.add(12) = (1 << 5) | (PdcPacketBits::PDC_PKT_CLEAR_BSY as u8) | ATA_REG_DEVICE;
    *buf.add(13) = dev_reg;

    /* device control register */
    *buf.add(14) = (1 << 5) | (PdcPacketBits::PDC_REG_DEVCTL as u8);
    *buf.add(15) = (*tf).ctl;

    16 /* offset of next byte */
}

pub unsafe fn pdc_pkt_footer(tf: *mut ata_taskfile, buf: *mut u8, mut i: u32) -> u32 {
    if ((*tf).flags & ATA_TFLAG_DEVICE) != 0 {
        *buf.add(i as usize) = (1 << 5) | ATA_REG_DEVICE;
        i += 1;
        *buf.add(i as usize) = (*tf).device;
        i += 1;
    }

    /* and finally the command itself; also includes end-of-pkt marker */
    *buf.add(i as usize) = (1 << 5) | (PdcPacketBits::PDC_LAST_REG as u8) | ATA_REG_CMD;
    i += 1;
    *buf.add(i as usize) = (*tf).command;
    i += 1;

    i
}

pub unsafe fn pdc_prep_lba28(tf: *mut ata_taskfile, buf: *mut u8, mut i: u32) -> u32 {
    /* the "(1 << 5)" should be read "(count << 5)" */

    /* ATA command block registers */
    *buf.add(i as usize) = (1 << 5) | ATA_REG_FEATURE;
    i += 1;
    *buf.add(i as usize) = (*tf).feature;
    i += 1;

    *buf.add(i as usize) = (1 << 5) | ATA_REG_NSECT;
    i += 1;
    *buf.add(i as usize) = (*tf).nsect;
    i += 1;

    *buf.add(i as usize) = (1 << 5) | ATA_REG_LBAL;
    i += 1;
    *buf.add(i as usize) = (*tf).lbal;
    i += 1;

    *buf.add(i as usize) = (1 << 5) | ATA_REG_LBAM;
    i += 1;
    *buf.add(i as usize) = (*tf).lbam;
    i += 1;

    *buf.add(i as usize) = (1 << 5) | ATA_REG_LBAH;
    i += 1;
    *buf.add(i as usize) = (*tf).lbah;
    i += 1;

    i
}

pub unsafe fn pdc_prep_lba48(tf: *mut ata_taskfile, buf: *mut u8, mut i: u32) -> u32 {
    /* the "(2 << 5)" should be read "(count << 5)" */

    /* ATA command block registers */
    *buf.add(i as usize) = (2 << 5) | ATA_REG_FEATURE;
    i += 1;
    *buf.add(i as usize) = (*tf).hob_feature;
    i += 1;
    *buf.add(i as usize) = (*tf).feature;
    i += 1;

    *buf.add(i as usize) = (2 << 5) | ATA_REG_NSECT;
    i += 1;
    *buf.add(i as usize) = (*tf).hob_nsect;
    i += 1;
    *buf.add(i as usize) = (*tf).nsect;
    i += 1;

    *buf.add(i as usize) = (2 << 5) | ATA_REG_LBAL;
    i += 1;
    *buf.add(i as usize) = (*tf).hob_lbal;
    i += 1;
    *buf.add(i as usize) = (*tf).lbal;
    i += 1;

    *buf.add(i as usize) = (2 << 5) | ATA_REG_LBAM;
    i += 1;
    *buf.add(i as usize) = (*tf).hob_lbam;
    i += 1;
    *buf.add(i as usize) = (*tf).lbam;
    i += 1;

    *buf.add(i as usize) = (2 << 5) | ATA_REG_LBAH;
    i += 1;
    *buf.add(i as usize) = (*tf).hob_lbah;
    i += 1;
    *buf.add(i as usize) = (*tf).lbah;
    i += 1;

    i
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
