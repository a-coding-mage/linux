/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * include/media/si476x-platform.h -- Definitions of the data formats
 * returned by debugfs hooks
 *
 * Copyright (C) 2013 Andrey Smirnov
 *
 * Author: Andrey Smirnov <andrew.smirnov@gmail.com>
 */

/**
 * struct si476x_rsq_status - structure containing received signal
 * quality
 * @multhint:   Multipath Detect High.
 *              true  - Indicatedes that the value is below
 *                      FM_RSQ_MULTIPATH_HIGH_THRESHOLD
 *              false - Indicatedes that the value is above
 *                      FM_RSQ_MULTIPATH_HIGH_THRESHOLD
 * @multlint:   Multipath Detect Low.
 *              true  - Indicatedes that the value is below
 *                      FM_RSQ_MULTIPATH_LOW_THRESHOLD
 *              false - Indicatedes that the value is above
 *                      FM_RSQ_MULTIPATH_LOW_THRESHOLD
 * @snrhint:    SNR Detect High.
 *              true  - Indicatedes that the value is below
 *                      FM_RSQ_SNR_HIGH_THRESHOLD
 *              false - Indicatedes that the value is above
 *                      FM_RSQ_SNR_HIGH_THRESHOLD
 * @snrlint:    SNR Detect Low.
 *              true  - Indicatedes that the value is below
 *                      FM_RSQ_SNR_LOW_THRESHOLD
 *              false - Indicatedes that the value is above
 *                      FM_RSQ_SNR_LOW_THRESHOLD
 * @rssihint:   RSSI Detect High.
 *              true  - Indicatedes that the value is below
 *                      FM_RSQ_RSSI_HIGH_THRESHOLD
 *              false - Indicatedes that the value is above
 *                      FM_RSQ_RSSI_HIGH_THRESHOLD
 * @rssilint:   RSSI Detect Low.
 *              true  - Indicatedes that the value is below
 *                      FM_RSQ_RSSI_LOW_THRESHOLD
 *              false - Indicatedes that the value is above
 *                      FM_RSQ_RSSI_LOW_THRESHOLD
 * @bltf:       Band Limit.
 *              Set if seek command hits the band limit or wrapped to
 *              the original frequency.
 * @snr_ready:  SNR measurement in progress.
 * @rssiready:  RSSI measurement in progress.
 * @afcrl:      Set if FREQOFF >= MAX_TUNE_ERROR
 * @valid:      Set if the channel is valid
 *               rssi < FM_VALID_RSSI_THRESHOLD
 *               snr  < FM_VALID_SNR_THRESHOLD
 *               tune_error < FM_VALID_MAX_TUNE_ERROR
 * @readfreq:   Current tuned frequency.
 * @freqoff:    Signed frequency offset.
 * @rssi:       Received Signal Strength Indicator(dBuV).
 * @snr:        RF SNR Indicator(dB).
 * @lassi:
 * @hassi:      Low/High side Adjacent(100 kHz) Channel Strength Indicator
 * @mult:       Multipath indicator
 * @dev:        Who knows? But values may vary.
 * @readantcap: Antenna tuning capacity value.
 * @assi:       Adjacent Channel(+/- 200kHz) Strength Indicator
 * @usn:        Ultrasonic Noise Inticator in -DBFS
 */
#[repr(C, packed)]
pub struct si476x_rsq_status_report {
    pub multhint: u8,
    pub multlint: u8,
    pub snrhint: u8,
    pub snrlint: u8,
    pub rssihint: u8,
    pub rssilint: u8,
    pub bltf: u8,
    pub snr_ready: u8,
    pub rssiready: u8,
    pub injside: u8,
    pub afcrl: u8,
    pub valid: u8,
    pub readfreq: u16,
    pub freqoff: i8,
    pub rssi: i8,
    pub snr: i8,
    pub issi: i8,
    pub lassi: i8,
    pub hassi: i8,
    pub mult: i8,
    pub dev: u8,
    pub readantcap: u16,
    pub assi: i8,
    pub usn: i8,
    pub pilotdev: u8,
    pub rdsdev: u8,
    pub assidev: u8,
    pub strongdev: u8,
    pub rdspi: u16,
}

/**
 * si476x_acf_status_report - ACF report results
 *
 * @blend_int: If set, indicates that stereo separation has crossed
 * below the blend threshold as set by FM_ACF_BLEND_THRESHOLD
 * @hblend_int: If set, indicates that HiBlend cutoff frequency is
 * lower than threshold as set by FM_ACF_HBLEND_THRESHOLD
 * @hicut_int:  If set, indicates that HiCut cutoff frequency is lower
 * than the threshold set by ACF_
 */
#[repr(C, packed)]
pub struct si476x_acf_status_report {
    pub blend_int: u8,
    pub hblend_int: u8,
    pub hicut_int: u8,
    pub chbw_int: u8,
    pub softmute_int: u8,
    pub smute: u8,
    pub smattn: u8,
    pub chbw: u8,
    pub hicut: u8,
    pub hiblend: u8,
    pub pilot: u8,
    pub stblend: u8,
}

#[repr(i32)]
pub enum si476x_fmagc {
    SI476X_FMAGC_10K_OHM = 0,
    SI476X_FMAGC_800_OHM = 1,
    SI476X_FMAGC_400_OHM = 2,
    SI476X_FMAGC_200_OHM = 4,
    SI476X_FMAGC_100_OHM = 8,
    SI476X_FMAGC_50_OHM = 16,
    SI476X_FMAGC_25_OHM = 32,
    SI476X_FMAGC_12P5_OHM = 64,
    SI476X_FMAGC_6P25_OHM = 128,
}

#[repr(C, packed)]
pub struct si476x_agc_status_report {
    pub mxhi: u8,
    pub mxlo: u8,
    pub lnahi: u8,
    pub lnalo: u8,
    pub fmagc1: u8,
    pub fmagc2: u8,
    pub pgagain: u8,
    pub fmwblang: u8,
}

#[repr(C, packed)]
pub struct si476x_rds_blockcount_report {
    pub expected: u16,
    pub received: u16,
    pub uncorrectable: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
