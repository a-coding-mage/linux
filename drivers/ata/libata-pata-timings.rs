// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Helper library for PATA timings
 *
 *  Copyright 2003-2004 Red Hat, Inc.  All rights reserved.
 *  Copyright 2003-2004 Jeff Garzik
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * This mode timing computation functionality is ported over from
 * drivers/ide/ide-timing.h and was originally written by Vojtech Pavlik
 */
/*
 * PIO 0-4, MWDMA 0-2 and UDMA 0-6 timings (in nanoseconds).
 * These were taken from ATA/ATAPI-6 standard, rev 0a, except
 * for UDMA6, which is currently supported only by Maxtor drives.
 *
 * For PIO 5/6 MWDMA 3/4 see the CFA specification 3.0.
 */

static mut ata_timing: [ata_timing; 24] = [
    // { XFER_PIO_SLOW, 120, 290, 240, 960, 290, 240, 0, 960, 0 },
    ata_timing { mode: XFER_PIO_0, setup: 70, act8b: 290, rec8b: 240, cyc8b: 600, active: 165, recover: 150, dmack_hold: 0, cycle: 600, udma: 0 },
    ata_timing { mode: XFER_PIO_1, setup: 50, act8b: 290, rec8b: 93, cyc8b: 383, active: 125, recover: 100, dmack_hold: 0, cycle: 383, udma: 0 },
    ata_timing { mode: XFER_PIO_2, setup: 30, act8b: 290, rec8b: 40, cyc8b: 330, active: 100, recover: 90, dmack_hold: 0, cycle: 240, udma: 0 },
    ata_timing { mode: XFER_PIO_3, setup: 30, act8b: 80, rec8b: 70, cyc8b: 180, active: 80, recover: 70, dmack_hold: 0, cycle: 180, udma: 0 },
    ata_timing { mode: XFER_PIO_4, setup: 25, act8b: 70, rec8b: 25, cyc8b: 120, active: 70, recover: 25, dmack_hold: 0, cycle: 120, udma: 0 },
    ata_timing { mode: XFER_PIO_5, setup: 15, act8b: 65, rec8b: 25, cyc8b: 100, active: 65, recover: 25, dmack_hold: 0, cycle: 100, udma: 0 },
    ata_timing { mode: XFER_PIO_6, setup: 10, act8b: 55, rec8b: 20, cyc8b: 80, active: 55, recover: 20, dmack_hold: 0, cycle: 80, udma: 0 },
    ata_timing { mode: XFER_SW_DMA_0, setup: 120, act8b: 0, rec8b: 0, cyc8b: 0, active: 480, recover: 480, dmack_hold: 50, cycle: 960, udma: 0 },
    ata_timing { mode: XFER_SW_DMA_1, setup: 90, act8b: 0, rec8b: 0, cyc8b: 0, active: 240, recover: 240, dmack_hold: 30, cycle: 480, udma: 0 },
    ata_timing { mode: XFER_SW_DMA_2, setup: 60, act8b: 0, rec8b: 0, cyc8b: 0, active: 120, recover: 120, dmack_hold: 20, cycle: 240, udma: 0 },
    ata_timing { mode: XFER_MW_DMA_0, setup: 60, act8b: 0, rec8b: 0, cyc8b: 0, active: 215, recover: 215, dmack_hold: 20, cycle: 480, udma: 0 },
    ata_timing { mode: XFER_MW_DMA_1, setup: 45, act8b: 0, rec8b: 0, cyc8b: 0, active: 80, recover: 50, dmack_hold: 5, cycle: 150, udma: 0 },
    ata_timing { mode: XFER_MW_DMA_2, setup: 25, act8b: 0, rec8b: 0, cyc8b: 0, active: 70, recover: 25, dmack_hold: 5, cycle: 120, udma: 0 },
    ata_timing { mode: XFER_MW_DMA_3, setup: 25, act8b: 0, rec8b: 0, cyc8b: 0, active: 65, recover: 25, dmack_hold: 5, cycle: 100, udma: 0 },
    ata_timing { mode: XFER_MW_DMA_4, setup: 25, act8b: 0, rec8b: 0, cyc8b: 0, active: 55, recover: 20, dmack_hold: 5, cycle: 80, udma: 0 },
    // { XFER_UDMA_SLOW, 0, 0, 0, 0, 0, 0, 0, 0, 150 },
    ata_timing { mode: XFER_UDMA_0, setup: 0, act8b: 0, rec8b: 0, cyc8b: 0, active: 0, recover: 0, dmack_hold: 0, cycle: 0, udma: 120 },
    ata_timing { mode: XFER_UDMA_1, setup: 0, act8b: 0, rec8b: 0, cyc8b: 0, active: 0, recover: 0, dmack_hold: 0, cycle: 0, udma: 80 },
    ata_timing { mode: XFER_UDMA_2, setup: 0, act8b: 0, rec8b: 0, cyc8b: 0, active: 0, recover: 0, dmack_hold: 0, cycle: 0, udma: 60 },
    ata_timing { mode: XFER_UDMA_3, setup: 0, act8b: 0, rec8b: 0, cyc8b: 0, active: 0, recover: 0, dmack_hold: 0, cycle: 0, udma: 45 },
    ata_timing { mode: XFER_UDMA_4, setup: 0, act8b: 0, rec8b: 0, cyc8b: 0, active: 0, recover: 0, dmack_hold: 0, cycle: 0, udma: 30 },
    ata_timing { mode: XFER_UDMA_5, setup: 0, act8b: 0, rec8b: 0, cyc8b: 0, active: 0, recover: 0, dmack_hold: 0, cycle: 0, udma: 20 },
    ata_timing { mode: XFER_UDMA_6, setup: 0, act8b: 0, rec8b: 0, cyc8b: 0, active: 0, recover: 0, dmack_hold: 0, cycle: 0, udma: 15 },
    ata_timing { mode: 0xFF, ..Default::default() },
];

#[inline]
fn enough(v: i32, unit: i32) -> i32 { (v - 1) / unit + 1 }

#[inline]
fn ez(v: i32, unit: i32) -> i32 { if v != 0 { enough(v * 1000, unit) } else { 0 } }

unsafe fn ata_timing_quantize(t: *const ata_timing, q: *mut ata_timing, T: i32, UT: i32) {
    (*q).setup = ez((*t).setup, T); (*q).act8b = ez((*t).act8b, T);
    (*q).rec8b = ez((*t).rec8b, T); (*q).cyc8b = ez((*t).cyc8b, T);
    (*q).active = ez((*t).active, T); (*q).recover = ez((*t).recover, T);
    (*q).dmack_hold = ez((*t).dmack_hold, T); (*q).cycle = ez((*t).cycle, T);
    (*q).udma = ez((*t).udma, UT);
}

pub unsafe fn ata_timing_merge(a: *const ata_timing, b: *const ata_timing, m: *mut ata_timing, what: u32) {
    if what & ATA_TIMING_SETUP != 0 { (*m).setup = (*a).setup.max((*b).setup); }
    if what & ATA_TIMING_ACT8B != 0 { (*m).act8b = (*a).act8b.max((*b).act8b); }
    if what & ATA_TIMING_REC8B != 0 { (*m).rec8b = (*a).rec8b.max((*b).rec8b); }
    if what & ATA_TIMING_CYC8B != 0 { (*m).cyc8b = (*a).cyc8b.max((*b).cyc8b); }
    if what & ATA_TIMING_ACTIVE != 0 { (*m).active = (*a).active.max((*b).active); }
    if what & ATA_TIMING_RECOVER != 0 { (*m).recover = (*a).recover.max((*b).recover); }
    if what & ATA_TIMING_DMACK_HOLD != 0 { (*m).dmack_hold = (*a).dmack_hold.max((*b).dmack_hold); }
    if what & ATA_TIMING_CYCLE != 0 { (*m).cycle = (*a).cycle.max((*b).cycle); }
    if what & ATA_TIMING_UDMA != 0 { (*m).udma = (*a).udma.max((*b).udma); }
}

pub unsafe fn ata_timing_find_mode(xfer_mode: u8) -> *const ata_timing {
    let mut t = ata_timing.as_ptr();
    while xfer_mode > (*t).mode { t = t.add(1); }
    if xfer_mode == (*t).mode { return t; }
    warn_once!(true, "ata_timing_find_mode: unable to find timing for xfer_mode 0x{:x}\n", xfer_mode);
    core::ptr::null()
}

pub unsafe fn ata_timing_compute(adev: *mut ata_device, speed: u16, t: *mut ata_timing, T: i32, UT: i32) -> i32 {
    let id = (*adev).id;
    let s = ata_timing_find_mode(speed);
    if s.is_null() { return -EINVAL; }
    core::ptr::copy_nonoverlapping(s, t, 1);
    let mut p: ata_timing = core::mem::zeroed();
    if (*id.add(ATA_ID_FIELD_VALID as usize) & 2) != 0 {
        if speed >= XFER_PIO_0 && speed < XFER_SW_DMA_0 {
            if speed <= XFER_PIO_2 { p.cycle = p.cyc8b = *id.add(ATA_ID_EIDE_PIO as usize); }
            else if speed <= XFER_PIO_4 || (speed == XFER_PIO_5 && !ata_id_is_cfa(id)) { p.cycle = p.cyc8b = *id.add(ATA_ID_EIDE_PIO_IORDY as usize); }
        } else if speed >= XFER_MW_DMA_0 && speed <= XFER_MW_DMA_2 { p.cycle = *id.add(ATA_ID_EIDE_DMA_MIN as usize); }
        ata_timing_merge(&p, t, t, ATA_TIMING_CYCLE | ATA_TIMING_CYC8B);
    }
    ata_timing_quantize(t, t, T, UT);
    if speed > XFER_PIO_6 { ata_timing_compute(adev, (*adev).pio_mode, &mut p, T, UT); ata_timing_merge(&p, t, t, ATA_TIMING_ALL); }
    if (*t).act8b + (*t).rec8b < (*t).cyc8b { (*t).act8b += ((*t).cyc8b - ((*t).act8b + (*t).rec8b)) / 2; (*t).rec8b = (*t).cyc8b - (*t).act8b; }
    if (*t).active + (*t).recover < (*t).cycle { (*t).active += ((*t).cycle - ((*t).active + (*t).recover)) / 2; (*t).recover = (*t).cycle - (*t).active; }
    if (*t).active + (*t).recover > (*t).cycle { (*t).cycle = (*t).active + (*t).recover; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
