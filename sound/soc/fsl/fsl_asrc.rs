// SPDX-License-Identifier: GPL-2.0
//
// Freescale ASRC ALSA SoC Digital Audio Interface (DAI) driver
//
// Copyright (C) 2014 Freescale Semiconductor, Inc.
//
// Author: Nicolin Chen <nicoleotsuka@gmail.com>

// Translated from C implementation source. Linux, ASoC, regmap, clock,
// platform, PM, and fsl_asrc.h symbols are external dependencies.

pub const IDEAL_RATIO_DECIMAL_DEPTH: u32 = 26;
pub const DIVIDER_NUM: usize = 64;
pub const INIT_RETRY_NUM: i32 = 50;

macro_rules! pair_err {
    ($asrc:expr, $index:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {
        dev_err(&mut (*(*$asrc).pdev).dev, concat!("Pair %c: ", $fmt), b'A' as i32 + $index as i32 $(, $arg)*)
    };
}

macro_rules! pair_dbg {
    ($asrc:expr, $index:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {
        dev_dbg(&mut (*(*$asrc).pdev).dev, concat!("Pair %c: ", $fmt), b'A' as i32 + $index as i32 $(, $arg)*)
    };
}

macro_rules! pair_warn {
    ($asrc:expr, $index:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {
        dev_warn(&mut (*(*$asrc).pdev).dev, concat!("Pair %c: ", $fmt), b'A' as i32 + $index as i32 $(, $arg)*)
    };
}

type bool_ = bool;
type u8_ = u8;
type u32_ = u32;
type u64_ = u64;
type ulong = c_ulong;

#[allow(non_camel_case_types)]
type c_int = i32;
#[allow(non_camel_case_types)]
type c_uint = u32;
#[allow(non_camel_case_types)]
type c_ulong = u64;
#[allow(non_camel_case_types)]
type c_char = i8;
#[allow(non_camel_case_types)]
type c_void = core::ffi::c_void;

// Corresponding to process_option
static mut supported_asrc_rate: [c_uint; 16] = [
    5512, 8000, 11025, 12000, 16000, 22050, 24000, 32000, 44100, 48000,
    64000, 88200, 96000, 128000, 176400, 192000,
];

static mut fsl_asrc_rate_constraints: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: unsafe { ARRAY_SIZE(&supported_asrc_rate) },
    list: unsafe { supported_asrc_rate.as_ptr() },
};

/*
 * The following tables map the relationship between asrc_inclk/asrc_outclk in
 * fsl_asrc.h and the registers of ASRCSR
 */
static mut input_clk_map_imx35: [u8_; ASRC_CLK_MAP_LEN] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
];

static mut output_clk_map_imx35: [u8_; ASRC_CLK_MAP_LEN] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
];

/* i.MX53 uses the same map for input and output */
static mut input_clk_map_imx53: [u8_; ASRC_CLK_MAP_LEN] = [
    /* 0x0  0x1  0x2  0x3  0x4  0x5  0x6  0x7  0x8  0x9  0xa  0xb  0xc  0xd  0xe  0xf */
    0x0, 0x1, 0x2, 0x7, 0x4, 0x5, 0x6, 0x3, 0x8, 0x9, 0xa, 0xb, 0xc, 0xf, 0xe, 0xd,
    0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7,
    0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7,
];

static mut output_clk_map_imx53: [u8_; ASRC_CLK_MAP_LEN] = [
    /* 0x0  0x1  0x2  0x3  0x4  0x5  0x6  0x7  0x8  0x9  0xa  0xb  0xc  0xd  0xe  0xf */
    0x8, 0x9, 0xa, 0x7, 0xc, 0x5, 0x6, 0xb, 0x0, 0x1, 0x2, 0x3, 0x4, 0xf, 0xe, 0xd,
    0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7,
    0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7, 0x7,
];

/*
 * i.MX8QM/i.MX8QXP uses the same map for input and output.
 * clk_map_imx8qm[0] is for i.MX8QM asrc0
 * clk_map_imx8qm[1] is for i.MX8QM asrc1
 * clk_map_imx8qxp[0] is for i.MX8QXP asrc0
 * clk_map_imx8qxp[1] is for i.MX8QXP asrc1
 */
static mut clk_map_imx8qm: [[u8_; ASRC_CLK_MAP_LEN]; 2] = [
    [
        0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0x0,
        0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf,
        0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf,
    ],
    [
        0xf, 0xf, 0xf, 0xf, 0xf, 0x7, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0x0,
        0x0, 0x1, 0x2, 0x3, 0xb, 0xc, 0xf, 0xf, 0xd, 0xe, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf,
        0x4, 0x5, 0x6, 0xf, 0x8, 0x9, 0xa, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf,
    ],
];

static mut clk_map_imx8qxp: [[u8_; ASRC_CLK_MAP_LEN]; 2] = [
    [
        0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0x0,
        0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0xf, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xf, 0xf,
        0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf,
    ],
    [
        0xf, 0xf, 0xf, 0xf, 0xf, 0x7, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0x0,
        0x0, 0x1, 0x2, 0x3, 0x7, 0x8, 0xf, 0xf, 0x9, 0xa, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf,
        0xf, 0xf, 0x6, 0xf, 0xf, 0xf, 0xa, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf,
    ],
];

static mut clk_map_imx952: [u8_; ASRC_CLK_MAP_LEN] = [
    0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0x0,
    0x0, 0x1, 0x2, 0x3, 0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0x4, 0x5, 0x6, 0x8, 0xf, 0xf,
    0xf, 0xf, 0xf, 0xf, 0xf, 0xf, 0x7, 0x9, 0xa, 0xb, 0xc, 0xd, 0xf, 0xf, 0xf, 0xf,
];

/*
 * According to RM, the divider range is 1 ~ 8,
 * prescaler is power of 2 from 1 ~ 128.
 */
static mut asrc_clk_divider: [c_int; DIVIDER_NUM] = [
    1, 2, 4, 8, 16, 32, 64, 128,
    2, 4, 8, 16, 32, 64, 128, 256,
    3, 6, 12, 24, 48, 96, 192, 384,
    4, 8, 16, 32, 64, 128, 256, 512,
    5, 10, 20, 40, 80, 160, 320, 640,
    6, 12, 24, 48, 96, 192, 384, 768,
    7, 14, 28, 56, 112, 224, 448, 896,
    8, 16, 32, 64, 128, 256, 512, 1024,
];

/*
 * Check if the divider is available for internal ratio mode
 */
unsafe fn fsl_asrc_divider_avail(clk_rate: c_int, rate: c_int, div: *mut c_int) -> bool_ {
    let mut rem: u32_;
    let mut i: u32_;
    let mut n: u64_;

    if !div.is_null() {
        *div = 0;
    }

    if clk_rate == 0 || rate == 0 {
        return false;
    }

    n = clk_rate as u64_;
    rem = do_div(&mut n, rate as u32_) as u32_;

    if !div.is_null() {
        *div = n as c_int;
    }

    if rem != 0 {
        return false;
    }

    i = 0;
    while (i as usize) < DIVIDER_NUM {
        if n == asrc_clk_divider[i as usize] as u64_ {
            break;
        }
        i += 1;
    }

    if (i as usize) == DIVIDER_NUM {
        return false;
    }

    true
}

/**
 * fsl_asrc_sel_proc - Select the pre-processing and post-processing options
 * @inrate: input sample rate
 * @outrate: output sample rate
 * @pre_proc: return value for pre-processing option
 * @post_proc: return value for post-processing option
 *
 * Make sure to exclude following unsupported cases before
 * calling this function:
 * 1) inrate > 8.125 * outrate
 * 2) inrate > 16.125 * outrate
 *
 */
unsafe fn fsl_asrc_sel_proc(inrate: c_int, outrate: c_int, pre_proc: *mut c_int, post_proc: *mut c_int) {
    let post_proc_cond2: bool_;
    let post_proc_cond0: bool_;

    /* select pre_proc between [0, 2] */
    if inrate * 8 > 33 * outrate {
        *pre_proc = 2;
    } else if inrate * 8 > 15 * outrate {
        if inrate > 152000 {
            *pre_proc = 2;
        } else {
            *pre_proc = 1;
        }
    } else if inrate < 76000 {
        *pre_proc = 0;
    } else if inrate > 152000 {
        *pre_proc = 2;
    } else {
        *pre_proc = 1;
    }

    /* Condition for selection of post-processing */
    post_proc_cond2 = (inrate * 15 > outrate * 16 && outrate < 56000) ||
        (inrate > 56000 && outrate < 56000);
    post_proc_cond0 = inrate * 23 < outrate * 8;

    if post_proc_cond2 {
        *post_proc = 2;
    } else if post_proc_cond0 {
        *post_proc = 0;
    } else {
        *post_proc = 1;
    }
}

/**
 * fsl_asrc_request_pair - Request ASRC pair
 * @channels: number of channels
 * @pair: pointer to pair
 *
 * It assigns pair by the order of A->C->B because allocation of pair B,
 * within range [ANCA, ANCA+ANCB-1], depends on the channels of pair A
 * while pair A and pair C are comparatively independent.
 */
unsafe fn fsl_asrc_request_pair(channels: c_int, pair: *mut fsl_asrc_pair) -> c_int {
    let mut index: asrc_pair_index = ASRC_INVALID_PAIR;
    let asrc: *mut fsl_asrc = (*pair).asrc;
    let dev: *mut device = &mut (*(*asrc).pdev).dev;
    let mut i: c_int;
    let mut ret: c_int = 0;

    guard_spinlock_irqsave(&mut (*asrc).lock);

    i = ASRC_PAIR_A;
    while i < ASRC_PAIR_MAX_NUM {
        if !(*asrc).pair[i as usize].is_null() {
            i += 1;
            continue;
        }

        index = i;

        if i != ASRC_PAIR_B {
            break;
        }
        i += 1;
    }

    if index == ASRC_INVALID_PAIR {
        dev_err(dev, "all pairs are busy now\n");
        ret = -EBUSY;
    } else if (*asrc).channel_avail < channels {
        dev_err(dev, "can't afford required channels: %d\n", channels);
        ret = -EINVAL;
    } else {
        (*asrc).channel_avail -= channels;
        (*asrc).pair[index as usize] = pair;
        (*pair).channels = channels;
        (*pair).index = index;
    }

    ret
}

/**
 * fsl_asrc_release_pair - Release ASRC pair
 * @pair: pair to release
 *
 * It clears the resource from asrc and releases the occupied channels.
 */
unsafe fn fsl_asrc_release_pair(pair: *mut fsl_asrc_pair) {
    let asrc: *mut fsl_asrc = (*pair).asrc;
    let index: asrc_pair_index = (*pair).index;

    /* Make sure the pair is disabled */
    regmap_update_bits((*asrc).regmap, REG_ASRCTR, ASRCTR_ASRCEi_MASK(index), 0);

    guard_spinlock_irqsave(&mut (*asrc).lock);

    (*asrc).channel_avail += (*pair).channels;
    (*asrc).pair[index as usize] = core::ptr::null_mut();
    (*pair).error = 0;
}

/**
 * fsl_asrc_set_watermarks- configure input and output thresholds
 * @pair: pointer to pair
 * @in: input threshold
 * @out: output threshold
 */
unsafe fn fsl_asrc_set_watermarks(pair: *mut fsl_asrc_pair, in_: u32_, out: u32_) {
    let asrc: *mut fsl_asrc = (*pair).asrc;
    let index: asrc_pair_index = (*pair).index;

    regmap_update_bits(
        (*asrc).regmap,
        REG_ASRMCR(index),
        ASRMCRi_EXTTHRSHi_MASK | ASRMCRi_INFIFO_THRESHOLD_MASK | ASRMCRi_OUTFIFO_THRESHOLD_MASK,
        ASRMCRi_EXTTHRSHi | ASRMCRi_INFIFO_THRESHOLD(in_) | ASRMCRi_OUTFIFO_THRESHOLD(out),
    );
}

/**
 * fsl_asrc_cal_asrck_divisor - Calculate the total divisor between asrck clock rate and sample rate
 * @pair: pointer to pair
 * @div: divider
 *
 * It follows the formula clk_rate = samplerate * (2 ^ prescaler) * divider
 */
unsafe fn fsl_asrc_cal_asrck_divisor(_pair: *mut fsl_asrc_pair, mut div: u32_) -> u32_ {
    let mut ps: u32_ = 0;

    /* Calculate the divisors: prescaler [2^0, 2^7], divder [1, 8] */
    while div > 8 {
        ps += 1;
        div >>= 1;
    }

    ((div - 1) << ASRCDRi_AxCPi_WIDTH) | ps
}

/**
 * fsl_asrc_set_ideal_ratio - Calculate and set the ratio for Ideal Ratio mode only
 * @pair: pointer to pair
 * @inrate: input rate
 * @outrate: output rate
 *
 * The ratio is a 32-bit fixed point value with 26 fractional bits.
 */
unsafe fn fsl_asrc_set_ideal_ratio(pair: *mut fsl_asrc_pair, mut inrate: c_int, outrate: c_int) -> c_int {
    let asrc: *mut fsl_asrc = (*pair).asrc;
    let index: asrc_pair_index = (*pair).index;
    let mut ratio: ulong;
    let mut i: c_int;

    if outrate == 0 {
        pair_err!(asrc, index, "output rate should not be zero\n");
        return -EINVAL;
    }

    /* Calculate the intergal part of the ratio */
    ratio = ((inrate / outrate) as ulong) << IDEAL_RATIO_DECIMAL_DEPTH;

    /* ... and then the 26 depth decimal part */
    inrate %= outrate;

    i = 1;
    while i <= IDEAL_RATIO_DECIMAL_DEPTH as c_int {
        inrate <<= 1;

        if inrate < outrate {
            i += 1;
            continue;
        }

        ratio |= (1 as ulong) << (IDEAL_RATIO_DECIMAL_DEPTH as c_int - i);
        inrate -= outrate;

        if inrate == 0 {
            break;
        }
        i += 1;
    }

    regmap_write((*asrc).regmap, REG_ASRIDRL(index), ratio as c_uint);
    regmap_write((*asrc).regmap, REG_ASRIDRH(index), (ratio >> 24) as c_uint);

    0
}

unsafe fn fsl_asrc_config_pair(pair: *mut fsl_asrc_pair, use_ideal_rate: bool_) -> c_int {
    let pair_priv: *mut fsl_asrc_pair_priv = (*pair).private;
    let config: *mut asrc_config = (*pair_priv).config;
    let asrc: *mut fsl_asrc = (*pair).asrc;
    let asrc_priv: *mut fsl_asrc_priv = (*asrc).private;
    let index: asrc_pair_index = (*pair).index;
    let input_word_width: asrc_word_width;
    let output_word_width: asrc_word_width;
    let mut inrate: u32_;
    let mut outrate: u32_;
    let indiv: u32_;
    let outdiv: u32_;
    let mut clk_index: [u32_; 2] = [0; 2];
    let mut div: [u32_; 2] = [0; 2];
    let mut clk_rate: u64_;
    let mut in_: c_int;
    let mut out: c_int;
    let mut channels: c_int;
    let mut pre_proc: c_int = 0;
    let mut post_proc: c_int = 0;
    let mut clk: *mut clk;
    let ideal: bool_;
    let mut div_avail: bool_;

    if config.is_null() {
        pair_err!(asrc, index, "invalid pair config\n");
        return -EINVAL;
    }

    /* Validate channels */
    if (*config).channel_num < 1 || (*config).channel_num > 10 {
        pair_err!(asrc, index, "does not support %d channels\n", (*config).channel_num);
        return -EINVAL;
    }

    match snd_pcm_format_width((*config).input_format) {
        8 => input_word_width = ASRC_WIDTH_8_BIT,
        16 => input_word_width = ASRC_WIDTH_16_BIT,
        24 => input_word_width = ASRC_WIDTH_24_BIT,
        _ => {
            pair_err!(asrc, index, "does not support this input format, %d\n", (*config).input_format);
            return -EINVAL;
        }
    }

    match snd_pcm_format_width((*config).output_format) {
        16 => output_word_width = ASRC_WIDTH_16_BIT,
        24 => output_word_width = ASRC_WIDTH_24_BIT,
        _ => {
            pair_err!(asrc, index, "does not support this output format, %d\n", (*config).output_format);
            return -EINVAL;
        }
    }

    inrate = (*config).input_sample_rate;
    outrate = (*config).output_sample_rate;
    ideal = (*config).inclk == INCLK_NONE;

    /* Validate input and output sample rates */
    in_ = 0;
    while (in_ as usize) < ARRAY_SIZE(&supported_asrc_rate) {
        if inrate == supported_asrc_rate[in_ as usize] {
            break;
        }
        in_ += 1;
    }

    if (in_ as usize) == ARRAY_SIZE(&supported_asrc_rate) {
        pair_err!(asrc, index, "unsupported input sample rate: %dHz\n", inrate);
        return -EINVAL;
    }

    out = 0;
    while (out as usize) < ARRAY_SIZE(&supported_asrc_rate) {
        if outrate == supported_asrc_rate[out as usize] {
            break;
        }
        out += 1;
    }

    if (out as usize) == ARRAY_SIZE(&supported_asrc_rate) {
        pair_err!(asrc, index, "unsupported output sample rate: %dHz\n", outrate);
        return -EINVAL;
    }

    if (outrate >= 5512 && outrate <= 30000) && (outrate > 24 * inrate || inrate > 8 * outrate) {
        pair_err!(asrc, index, "exceed supported ratio range [1/24, 8] for \t\t\t\tinrate/outrate: %d/%d\n", inrate, outrate);
        return -EINVAL;
    }

    /* Validate input and output clock sources */
    clk_index[IN as usize] = *(*asrc_priv).clk_map[IN as usize].add((*config).inclk as usize) as u32_;
    clk_index[OUT as usize] = *(*asrc_priv).clk_map[OUT as usize].add((*config).outclk as usize) as u32_;

    /* We only have output clock for ideal ratio mode */
    clk = (*asrc_priv).asrck_clk[clk_index[if ideal { OUT } else { IN } as usize] as usize];

    clk_rate = clk_get_rate(clk) as u64_;
    div_avail = fsl_asrc_divider_avail(clk_rate as c_int, inrate as c_int, &mut div[IN as usize] as *mut u32_ as *mut c_int);

    if div[IN as usize] == 0 || (!ideal && !div_avail) {
        pair_err!(asrc, index, "failed to support input sample rate %dHz by asrck_%x\n", inrate, clk_index[if ideal { OUT } else { IN } as usize]);
        return -EINVAL;
    }

    div[IN as usize] = min_t_u32(1024, div[IN as usize]);

    clk = (*asrc_priv).asrck_clk[clk_index[OUT as usize] as usize];
    clk_rate = clk_get_rate(clk) as u64_;
    if ideal && use_ideal_rate {
        div_avail = fsl_asrc_divider_avail(clk_rate as c_int, IDEAL_RATIO_RATE, &mut div[OUT as usize] as *mut u32_ as *mut c_int);
    } else {
        div_avail = fsl_asrc_divider_avail(clk_rate as c_int, outrate as c_int, &mut div[OUT as usize] as *mut u32_ as *mut c_int);
    }

    /* Output divider has the same limitation as the input one */
    if div[OUT as usize] == 0 || (!ideal && !div_avail) {
        pair_err!(asrc, index, "failed to support output sample rate %dHz by asrck_%x\n", outrate, clk_index[OUT as usize]);
        return -EINVAL;
    }

    div[OUT as usize] = min_t_u32(1024, div[OUT as usize]);

    /* Set the channel number */
    channels = (*config).channel_num;

    if (*(*asrc_priv).soc).channel_bits < 4 {
        channels /= 2;
    }

    /* Update channels for current pair */
    regmap_update_bits(
        (*asrc).regmap,
        REG_ASRCNCR,
        ASRCNCR_ANCi_MASK(index, (*(*asrc_priv).soc).channel_bits),
        ASRCNCR_ANCi(index, channels, (*(*asrc_priv).soc).channel_bits),
    );

    /* Default setting: Automatic selection for processing mode */
    regmap_update_bits((*asrc).regmap, REG_ASRCTR, ASRCTR_ATSi_MASK(index), ASRCTR_ATS(index));
    regmap_update_bits((*asrc).regmap, REG_ASRCTR, ASRCTR_IDRi_MASK(index) | ASRCTR_USRi_MASK(index), ASRCTR_USR(index));

    /* Set the input and output clock sources */
    regmap_update_bits(
        (*asrc).regmap,
        REG_ASRCSR,
        ASRCSR_AICSi_MASK(index) | ASRCSR_AOCSi_MASK(index),
        ASRCSR_AICS(index, clk_index[IN as usize]) | ASRCSR_AOCS(index, clk_index[OUT as usize]),
    );

    /* Calculate the input clock divisors */
    indiv = fsl_asrc_cal_asrck_divisor(pair, div[IN as usize]);
    outdiv = fsl_asrc_cal_asrck_divisor(pair, div[OUT as usize]);

    /* Suppose indiv and outdiv includes prescaler, so add its MASK too */
    regmap_update_bits(
        (*asrc).regmap,
        REG_ASRCDR(index),
        ASRCDRi_AOCPi_MASK(index) | ASRCDRi_AICPi_MASK(index) | ASRCDRi_AOCDi_MASK(index) | ASRCDRi_AICDi_MASK(index),
        ASRCDRi_AOCP(index, outdiv) | ASRCDRi_AICP(index, indiv),
    );

    /* Implement word_width configurations */
    regmap_update_bits(
        (*asrc).regmap,
        REG_ASRMCR1(index),
        ASRMCR1i_OW16_MASK | ASRMCR1i_IWD_MASK,
        ASRMCR1i_OW16(output_word_width) | ASRMCR1i_IWD(input_word_width),
    );

    /* Enable BUFFER STALL */
    regmap_update_bits((*asrc).regmap, REG_ASRMCR(index), ASRMCRi_BUFSTALLi_MASK, ASRMCRi_BUFSTALLi);

    /* Set default thresholds for input and output FIFO */
    fsl_asrc_set_watermarks(pair, ASRC_INPUTFIFO_THRESHOLD, ASRC_INPUTFIFO_THRESHOLD);

    /* Configure the following only for Ideal Ratio mode */
    if !ideal {
        return 0;
    }

    /* Clear ASTSx bit to use Ideal Ratio mode */
    regmap_update_bits((*asrc).regmap, REG_ASRCTR, ASRCTR_ATSi_MASK(index), 0);

    /* Enable Ideal Ratio mode */
    regmap_update_bits(
        (*asrc).regmap,
        REG_ASRCTR,
        ASRCTR_IDRi_MASK(index) | ASRCTR_USRi_MASK(index),
        ASRCTR_IDR(index) | ASRCTR_USR(index),
    );

    fsl_asrc_sel_proc(inrate as c_int, outrate as c_int, &mut pre_proc, &mut post_proc);

    /* Apply configurations for pre- and post-processing */
    regmap_update_bits(
        (*asrc).regmap,
        REG_ASRCFG,
        ASRCFG_PREMODi_MASK(index) | ASRCFG_POSTMODi_MASK(index),
        ASRCFG_PREMOD(index, pre_proc) | ASRCFG_POSTMOD(index, post_proc),
    );

    fsl_asrc_set_ideal_ratio(pair, inrate as c_int, outrate as c_int)
}

unsafe fn fsl_asrc_start_pair(pair: *mut fsl_asrc_pair) {
    let asrc: *mut fsl_asrc = (*pair).asrc;
    let index: asrc_pair_index = (*pair).index;
    let mut reg: c_int = 0;
    let mut retry: c_int = INIT_RETRY_NUM;
    let mut i: c_int;

    /* Enable the current pair */
    regmap_update_bits((*asrc).regmap, REG_ASRCTR, ASRCTR_ASRCEi_MASK(index), ASRCTR_ASRCE(index));

    /* Wait for status of initialization */
    loop {
        udelay(5);
        regmap_read((*asrc).regmap, REG_ASRCFG, &mut reg);
        reg &= ASRCFG_INIRQi_MASK(index);
        retry -= 1;
        if reg != 0 || retry == 0 {
            break;
        }
    }

    /* NOTE: Doesn't treat initialization timeout as an error */
    if retry == 0 {
        pair_warn!(asrc, index, "initialization isn't finished\n");
    }

    /* Make the input fifo to ASRC STALL level */
    regmap_read((*asrc).regmap, REG_ASRCNCR, &mut reg);
    i = 0;
    while i < (*pair).channels * 4 {
        regmap_write((*asrc).regmap, REG_ASRDI(index), 0);
        i += 1;
    }

    /* Enable overload interrupt */
    regmap_write((*asrc).regmap, REG_ASRIER, ASRIER_AOLIE);
}

unsafe fn fsl_asrc_stop_pair(pair: *mut fsl_asrc_pair) {
    let asrc: *mut fsl_asrc = (*pair).asrc;
    let index: asrc_pair_index = (*pair).index;

    /* Stop the current pair */
    regmap_update_bits((*asrc).regmap, REG_ASRCTR, ASRCTR_ASRCEi_MASK(index), 0);
}

unsafe fn fsl_asrc_get_dma_channel(pair: *mut fsl_asrc_pair, dir: bool_) -> *mut dma_chan {
    let asrc: *mut fsl_asrc = (*pair).asrc;
    let index: asrc_pair_index = (*pair).index;
    let mut name: [c_char; 4] = [0; 4];

    sprintf(name.as_mut_ptr(), "%cx%c", if dir == (IN != 0) { 'r' as c_int } else { 't' as c_int }, index + 'a' as c_int);

    dma_request_slave_channel(&mut (*(*asrc).pdev).dev, name.as_mut_ptr())
}

unsafe fn fsl_asrc_dai_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let asrc: *mut fsl_asrc = snd_soc_dai_get_drvdata(dai);
    let asrc_priv: *mut fsl_asrc_priv = (*asrc).private;

    /* Odd channel number is not valid for older ASRC (channel_bits==3) */
    if (*(*asrc_priv).soc).channel_bits == 3 {
        snd_pcm_hw_constraint_step((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, 2);
    }

    snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &mut fsl_asrc_rate_constraints)
}

/* Select proper clock source for internal ratio mode */
unsafe fn fsl_asrc_select_clk(asrc_priv: *mut fsl_asrc_priv, pair: *mut fsl_asrc_pair, in_rate: c_int, out_rate: c_int) {
    let pair_priv: *mut fsl_asrc_pair_priv = (*pair).private;
    let config: *mut asrc_config = (*pair_priv).config;
    let mut rate: [c_int; 2] = [0; 2];
    let mut select_clk: [c_int; 2] = [0; 2];
    let mut clk_rate: c_int;
    let mut clk_index: c_int;
    let mut i: c_int;
    let mut j: c_int;

    rate[IN as usize] = in_rate;
    rate[OUT as usize] = out_rate;

    /* Select proper clock source for internal ratio mode */
    j = 0;
    while j < 2 {
        i = 0;
        while i < ASRC_CLK_MAP_LEN as c_int {
            clk_index = *(*asrc_priv).clk_map[j as usize].add(i as usize) as c_int;
            clk_rate = clk_get_rate((*asrc_priv).asrck_clk[clk_index as usize]) as c_int;
            /* Only match a perfect clock source with no remainder */
            if fsl_asrc_divider_avail(clk_rate, rate[j as usize], core::ptr::null_mut()) {
                break;
            }
            i += 1;
        }

        select_clk[j as usize] = i;
        j += 1;
    }

    /* Switch to ideal ratio mode if there is no proper clock source */
    if select_clk[IN as usize] == ASRC_CLK_MAP_LEN as c_int || select_clk[OUT as usize] == ASRC_CLK_MAP_LEN as c_int {
        select_clk[IN as usize] = INCLK_NONE;
        select_clk[OUT as usize] = OUTCLK_ASRCK1_CLK;
    }

    (*config).inclk = select_clk[IN as usize];
    (*config).outclk = select_clk[OUT as usize];
}

unsafe fn fsl_asrc_dai_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int {
    let asrc: *mut fsl_asrc = snd_soc_dai_get_drvdata(dai);
    let asrc_priv: *mut fsl_asrc_priv = (*asrc).private;
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let pair: *mut fsl_asrc_pair = (*runtime).private_data;
    let pair_priv: *mut fsl_asrc_pair_priv = (*pair).private;
    let channels: c_uint = params_channels(params);
    let rate: c_uint = params_rate(params);
    let mut config: asrc_config = core::mem::zeroed();
    let mut ret: c_int;

    ret = fsl_asrc_request_pair(channels as c_int, pair);
    if ret != 0 {
        dev_err((*dai).dev, "fail to request asrc pair\n");
        return ret;
    }

    (*pair_priv).config = &mut config;

    config.pair = (*pair).index;
    config.channel_num = channels as c_int;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        config.input_format = params_format(params);
        config.output_format = (*asrc).asrc_format;
        config.input_sample_rate = rate;
        config.output_sample_rate = (*asrc).asrc_rate;
    } else {
        config.input_format = (*asrc).asrc_format;
        config.output_format = params_format(params);
        config.input_sample_rate = (*asrc).asrc_rate;
        config.output_sample_rate = rate;
    }

    fsl_asrc_select_clk(asrc_priv, pair, config.input_sample_rate as c_int, config.output_sample_rate as c_int);

    ret = fsl_asrc_config_pair(pair, false);
    if ret != 0 {
        dev_err((*dai).dev, "fail to config asrc pair\n");
        return ret;
    }

    0
}

unsafe fn fsl_asrc_dai_hw_free(substream: *mut snd_pcm_substream, _dai: *mut snd_soc_dai) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let pair: *mut fsl_asrc_pair = (*runtime).private_data;

    if !pair.is_null() {
        fsl_asrc_release_pair(pair);
    }

    0
}

unsafe fn fsl_asrc_dai_trigger(substream: *mut snd_pcm_substream, cmd: c_int, _dai: *mut snd_soc_dai) -> c_int {
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let pair: *mut fsl_asrc_pair = (*runtime).private_data;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => fsl_asrc_start_pair(pair),
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => fsl_asrc_stop_pair(pair),
        _ => return -EINVAL,
    }

    0
}

unsafe fn fsl_asrc_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let asrc: *mut fsl_asrc = snd_soc_dai_get_drvdata(dai);

    snd_soc_dai_init_dma_data(dai, &mut (*asrc).dma_params_tx, &mut (*asrc).dma_params_rx);

    0
}

static mut fsl_asrc_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(fsl_asrc_dai_probe),
    startup: Some(fsl_asrc_dai_startup),
    hw_params: Some(fsl_asrc_dai_hw_params),
    hw_free: Some(fsl_asrc_dai_hw_free),
    trigger: Some(fsl_asrc_dai_trigger),
};

pub const FSL_ASRC_FORMATS: u64 = SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_3LE;

static mut fsl_asrc_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        stream_name: "ASRC-Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 10,
        rate_min: 5512,
        rate_max: 192000,
        rates: SNDRV_PCM_RATE_KNOT,
        formats: FSL_ASRC_FORMATS | SNDRV_PCM_FMTBIT_S8,
    },
    capture: snd_soc_pcm_stream {
        stream_name: "ASRC-Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 10,
        rate_min: 5512,
        rate_max: 192000,
        rates: SNDRV_PCM_RATE_KNOT,
        formats: FSL_ASRC_FORMATS,
    },
    ops: unsafe { &mut fsl_asrc_dai_ops },
};

unsafe fn fsl_asrc_readable_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        REG_ASRCTR | REG_ASRIER | REG_ASRCNCR | REG_ASRCFG | REG_ASRCSR | REG_ASRCDR1 | REG_ASRCDR2 |
        REG_ASRSTR | REG_ASRPM1 | REG_ASRPM2 | REG_ASRPM3 | REG_ASRPM4 | REG_ASRPM5 | REG_ASRTFR1 |
        REG_ASRCCR | REG_ASRDOA | REG_ASRDOB | REG_ASRDOC | REG_ASRIDRHA | REG_ASRIDRLA |
        REG_ASRIDRHB | REG_ASRIDRLB | REG_ASRIDRHC | REG_ASRIDRLC | REG_ASR76K | REG_ASR56K |
        REG_ASRMCRA | REG_ASRFSTA | REG_ASRMCRB | REG_ASRFSTB | REG_ASRMCRC | REG_ASRFSTC |
        REG_ASRMCR1A | REG_ASRMCR1B | REG_ASRMCR1C => true,
        _ => false,
    }
}

unsafe fn fsl_asrc_volatile_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        REG_ASRSTR | REG_ASRDIA | REG_ASRDIB | REG_ASRDIC | REG_ASRDOA | REG_ASRDOB | REG_ASRDOC |
        REG_ASRFSTA | REG_ASRFSTB | REG_ASRFSTC | REG_ASRCFG => true,
        _ => false,
    }
}

unsafe fn fsl_asrc_writeable_reg(_dev: *mut device, reg: c_uint) -> bool_ {
    match reg {
        REG_ASRCTR | REG_ASRIER | REG_ASRCNCR | REG_ASRCFG | REG_ASRCSR | REG_ASRCDR1 | REG_ASRCDR2 |
        REG_ASRSTR | REG_ASRPM1 | REG_ASRPM2 | REG_ASRPM3 | REG_ASRPM4 | REG_ASRPM5 | REG_ASRTFR1 |
        REG_ASRCCR | REG_ASRDIA | REG_ASRDIB | REG_ASRDIC | REG_ASRIDRHA | REG_ASRIDRLA |
        REG_ASRIDRHB | REG_ASRIDRLB | REG_ASRIDRHC | REG_ASRIDRLC | REG_ASR76K | REG_ASR56K |
        REG_ASRMCRA | REG_ASRMCRB | REG_ASRMCRC | REG_ASRMCR1A | REG_ASRMCR1B | REG_ASRMCR1C => true,
        _ => false,
    }
}

static mut fsl_asrc_reg: [reg_default; 41] = [
    reg_default { reg: REG_ASRCTR, def: 0x0000 }, reg_default { reg: REG_ASRIER, def: 0x0000 },
    reg_default { reg: REG_ASRCNCR, def: 0x0000 }, reg_default { reg: REG_ASRCFG, def: 0x0000 },
    reg_default { reg: REG_ASRCSR, def: 0x0000 }, reg_default { reg: REG_ASRCDR1, def: 0x0000 },
    reg_default { reg: REG_ASRCDR2, def: 0x0000 }, reg_default { reg: REG_ASRSTR, def: 0x0000 },
    reg_default { reg: REG_ASRRA, def: 0x0000 }, reg_default { reg: REG_ASRRB, def: 0x0000 },
    reg_default { reg: REG_ASRRC, def: 0x0000 }, reg_default { reg: REG_ASRPM1, def: 0x0000 },
    reg_default { reg: REG_ASRPM2, def: 0x0000 }, reg_default { reg: REG_ASRPM3, def: 0x0000 },
    reg_default { reg: REG_ASRPM4, def: 0x0000 }, reg_default { reg: REG_ASRPM5, def: 0x0000 },
    reg_default { reg: REG_ASRTFR1, def: 0x0000 }, reg_default { reg: REG_ASRCCR, def: 0x0000 },
    reg_default { reg: REG_ASRDIA, def: 0x0000 }, reg_default { reg: REG_ASRDOA, def: 0x0000 },
    reg_default { reg: REG_ASRDIB, def: 0x0000 }, reg_default { reg: REG_ASRDOB, def: 0x0000 },
    reg_default { reg: REG_ASRDIC, def: 0x0000 }, reg_default { reg: REG_ASRDOC, def: 0x0000 },
    reg_default { reg: REG_ASRIDRHA, def: 0x0000 }, reg_default { reg: REG_ASRIDRLA, def: 0x0000 },
    reg_default { reg: REG_ASRIDRHB, def: 0x0000 }, reg_default { reg: REG_ASRIDRLB, def: 0x0000 },
    reg_default { reg: REG_ASRIDRHC, def: 0x0000 }, reg_default { reg: REG_ASRIDRLC, def: 0x0000 },
    reg_default { reg: REG_ASR76K, def: 0x0A47 }, reg_default { reg: REG_ASR56K, def: 0x0DF3 },
    reg_default { reg: REG_ASRMCRA, def: 0x0000 }, reg_default { reg: REG_ASRFSTA, def: 0x0000 },
    reg_default { reg: REG_ASRMCRB, def: 0x0000 }, reg_default { reg: REG_ASRFSTB, def: 0x0000 },
    reg_default { reg: REG_ASRMCRC, def: 0x0000 }, reg_default { reg: REG_ASRFSTC, def: 0x0000 },
    reg_default { reg: REG_ASRMCR1A, def: 0x0000 }, reg_default { reg: REG_ASRMCR1B, def: 0x0000 },
    reg_default { reg: REG_ASRMCR1C, def: 0x0000 },
];

static mut fsl_asrc_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: REG_ASRMCR1C,
    reg_defaults: unsafe { fsl_asrc_reg.as_ptr() },
    num_reg_defaults: unsafe { ARRAY_SIZE(&fsl_asrc_reg) },
    readable_reg: Some(fsl_asrc_readable_reg),
    volatile_reg: Some(fsl_asrc_volatile_reg),
    writeable_reg: Some(fsl_asrc_writeable_reg),
    cache_type: REGCACHE_FLAT,
};

unsafe fn fsl_asrc_init(asrc: *mut fsl_asrc) -> c_int {
    let ipg_rate: ulong;

    /* Halt ASRC internal FP when input FIFO needs data for pair A, B, C */
    regmap_write((*asrc).regmap, REG_ASRCTR, ASRCTR_ASRCEN);

    /* Disable interrupt by default */
    regmap_write((*asrc).regmap, REG_ASRIER, 0x0);

    /* Apply recommended settings for parameters from Reference Manual */
    regmap_write((*asrc).regmap, REG_ASRPM1, 0x7fffff);
    regmap_write((*asrc).regmap, REG_ASRPM2, 0x255555);
    regmap_write((*asrc).regmap, REG_ASRPM3, 0xff7280);
    regmap_write((*asrc).regmap, REG_ASRPM4, 0xff7280);
    regmap_write((*asrc).regmap, REG_ASRPM5, 0xff7280);

    /* Base address for task queue FIFO. Set to 0x7C */
    regmap_update_bits((*asrc).regmap, REG_ASRTFR1, ASRTFR1_TF_BASE_MASK, ASRTFR1_TF_BASE(0xfc));

    /*
     * Set the period of the 76KHz and 56KHz sampling clocks based on
     * the ASRC processing clock.
     * On iMX6, ipg_clk = 133MHz, REG_ASR76K = 0x06D6, REG_ASR56K = 0x0947
     */
    ipg_rate = clk_get_rate((*asrc).ipg_clk);
    regmap_write((*asrc).regmap, REG_ASR76K, (ipg_rate / 76000) as c_uint);
    regmap_write((*asrc).regmap, REG_ASR56K, (ipg_rate / 56000) as c_uint)
}

unsafe fn fsl_asrc_isr(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let asrc: *mut fsl_asrc = dev_id as *mut fsl_asrc;
    let dev: *mut device = &mut (*(*asrc).pdev).dev;
    let mut index: asrc_pair_index;
    let mut status: u32_ = 0;

    regmap_read((*asrc).regmap, REG_ASRSTR, &mut status);

    /* Clean overload error */
    regmap_write((*asrc).regmap, REG_ASRSTR, ASRSTR_AOLE);

    index = ASRC_PAIR_A;
    while index < ASRC_PAIR_MAX_NUM {
        if (*asrc).pair[index as usize].is_null() {
            index += 1;
            continue;
        }

        if status & ASRSTR_ATQOL != 0 {
            (*(*asrc).pair[index as usize]).error |= ASRC_TASK_Q_OVERLOAD;
            dev_dbg(dev, "ASRC Task Queue FIFO overload\n");
        }

        if status & ASRSTR_AOOL(index) != 0 {
            (*(*asrc).pair[index as usize]).error |= ASRC_OUTPUT_TASK_OVERLOAD;
            pair_dbg!(asrc, index, "Output Task Overload\n");
        }

        if status & ASRSTR_AIOL(index) != 0 {
            (*(*asrc).pair[index as usize]).error |= ASRC_INPUT_TASK_OVERLOAD;
            pair_dbg!(asrc, index, "Input Task Overload\n");
        }

        if status & ASRSTR_AODO(index) != 0 {
            (*(*asrc).pair[index as usize]).error |= ASRC_OUTPUT_BUFFER_OVERFLOW;
            pair_dbg!(asrc, index, "Output Data Buffer has overflowed\n");
        }

        if status & ASRSTR_AIDU(index) != 0 {
            (*(*asrc).pair[index as usize]).error |= ASRC_INPUT_BUFFER_UNDERRUN;
            pair_dbg!(asrc, index, "Input Data Buffer has underflowed\n");
        }
        index += 1;
    }

    IRQ_HANDLED
}

unsafe fn fsl_asrc_get_fifo_addr(dir: u8_, index: asrc_pair_index) -> c_int {
    REG_ASRDx(dir, index)
}

/* Get sample numbers in FIFO */
unsafe fn fsl_asrc_get_output_fifo_size(pair: *mut fsl_asrc_pair) -> c_uint {
    let asrc: *mut fsl_asrc = (*pair).asrc;
    let index: asrc_pair_index = (*pair).index;
    let mut val: u32_ = 0;

    regmap_read((*asrc).regmap, REG_ASRFST(index), &mut val);
    val &= ASRFSTi_OUTPUT_FIFO_MASK;
    val >> ASRFSTi_OUTPUT_FIFO_SHIFT
}

unsafe fn fsl_asrc_m2m_output_ready(pair: *mut fsl_asrc_pair) -> bool_ {
    let asrc: *mut fsl_asrc = (*pair).asrc;
    let index: asrc_pair_index = (*pair).index;
    let mut val: u32_ = 0;
    let ret: c_int;

    /* Check output fifo status if it exceeds the watermark. */
    ret = regmap_read_poll_timeout((*asrc).regmap, REG_ASRFST(index), &mut val, ASRFSTi_OUTPUT_FIFO_FILL(val) >= ASRC_M2M_OUTPUTFIFO_WML, 1, 1000);

    if ret != 0 {
        pair_warn!(asrc, index, "output is not ready\n");
        return false;
    }

    true
}

unsafe fn fsl_asrc_m2m_prepare(pair: *mut fsl_asrc_pair) -> c_int {
    let pair_priv: *mut fsl_asrc_pair_priv = (*pair).private;
    let asrc: *mut fsl_asrc = (*pair).asrc;
    let dev: *mut device = &mut (*(*asrc).pdev).dev;
    let mut config: asrc_config = core::mem::zeroed();
    let ret: c_int;

    /* fill config */
    config.pair = (*pair).index;
    config.channel_num = (*pair).channels;
    config.input_sample_rate = (*pair).rate[IN as usize];
    config.output_sample_rate = (*pair).rate[OUT as usize];
    config.input_format = (*pair).sample_format[IN as usize];
    config.output_format = (*pair).sample_format[OUT as usize];
    config.inclk = INCLK_NONE;
    config.outclk = OUTCLK_ASRCK1_CLK;

    (*pair_priv).config = &mut config;
    ret = fsl_asrc_config_pair(pair, true);
    if ret != 0 {
        dev_err(dev, "failed to config pair: %d\n", ret);
        return ret;
    }

    (*pair).first_convert = 1;

    0
}

unsafe fn fsl_asrc_m2m_start(pair: *mut fsl_asrc_pair) -> c_int {
    if (*pair).first_convert != 0 {
        fsl_asrc_start_pair(pair);
        (*pair).first_convert = 0;
    }
    fsl_asrc_set_watermarks(pair, ASRC_FIFO_THRESHOLD_MIN, ASRC_FIFO_THRESHOLD_MAX);
    fsl_asrc_set_watermarks(pair, ASRC_M2M_INPUTFIFO_WML, ASRC_M2M_OUTPUTFIFO_WML);
    0
}

unsafe fn fsl_asrc_m2m_stop(pair: *mut fsl_asrc_pair) -> c_int {
    if (*pair).first_convert == 0 {
        fsl_asrc_stop_pair(pair);
        (*pair).first_convert = 1;
    }

    0
}

/* calculate capture data length according to output data length and sample rate */
unsafe fn fsl_asrc_m2m_calc_out_len(pair: *mut fsl_asrc_pair, input_buffer_length: c_int) -> c_int {
    let in_width: c_uint;
    let out_width: c_uint;
    let channels: c_uint = (*pair).channels as c_uint;
    let in_samples: c_uint;
    let out_samples: c_uint;
    let out_length: c_uint;

    in_width = snd_pcm_format_physical_width((*pair).sample_format[IN as usize]) / 8;
    out_width = snd_pcm_format_physical_width((*pair).sample_format[OUT as usize]) / 8;

    in_samples = input_buffer_length as c_uint / in_width / channels;
    out_samples = (*pair).rate[OUT as usize] * in_samples / (*pair).rate[IN as usize];
    out_length = (out_samples - ASRC_OUTPUT_LAST_SAMPLE) * out_width * channels;

    out_length as c_int
}

unsafe fn fsl_asrc_m2m_get_maxburst(dir: u8_, pair: *mut fsl_asrc_pair) -> c_int {
    let asrc: *mut fsl_asrc = (*pair).asrc;
    let asrc_priv: *mut fsl_asrc_priv = (*asrc).private;
    let wml: c_int = if dir == IN as u8_ { ASRC_M2M_INPUTFIFO_WML as c_int } else { ASRC_M2M_OUTPUTFIFO_WML as c_int };

    if !(*(*asrc_priv).soc).use_edma {
        wml * (*pair).channels
    } else {
        1
    }
}

unsafe fn fsl_asrc_m2m_get_cap(cap: *mut fsl_asrc_m2m_cap) -> c_int {
    (*cap).fmt_in = FSL_ASRC_FORMATS;
    (*cap).fmt_out = FSL_ASRC_FORMATS | SNDRV_PCM_FMTBIT_S8;

    (*cap).rate_in = supported_asrc_rate.as_mut_ptr();
    (*cap).rate_in_count = ARRAY_SIZE(&supported_asrc_rate);
    (*cap).rate_out = supported_asrc_rate.as_mut_ptr();
    (*cap).rate_out_count = ARRAY_SIZE(&supported_asrc_rate);
    (*cap).chan_min = 1;
    (*cap).chan_max = 10;

    0
}

unsafe fn fsl_asrc_m2m_pair_resume(pair: *mut fsl_asrc_pair) -> c_int {
    let asrc: *mut fsl_asrc = (*pair).asrc;
    let mut i: c_int;

    i = 0;
    while i < (*pair).channels * 4 {
        regmap_write((*asrc).regmap, REG_ASRDI((*pair).index), 0);
        i += 1;
    }

    (*pair).first_convert = 1;
    0
}

unsafe fn fsl_asrc_runtime_resume(dev: *mut device) -> c_int {
    let asrc: *mut fsl_asrc = dev_get_drvdata(dev);
    let asrc_priv: *mut fsl_asrc_priv = (*asrc).private;
    let mut reg: c_int = 0;
    let mut retry: c_int = INIT_RETRY_NUM;
    let mut i: c_int;
    let mut ret: c_int;
    let mut asrctr: u32_ = 0;

    ret = clk_prepare_enable((*asrc).mem_clk);
    if ret != 0 {
        return ret;
    }
    ret = clk_prepare_enable((*asrc).ipg_clk);
    if ret != 0 {
        goto_disable_mem_clk(asrc, ret)
    } else {
        if !IS_ERR((*asrc).spba_clk) {
            ret = clk_prepare_enable((*asrc).spba_clk);
            if ret != 0 {
                clk_disable_unprepare((*asrc).ipg_clk);
                clk_disable_unprepare((*asrc).mem_clk);
                return ret;
            }
        }
        i = 0;
        while i < ASRC_CLK_MAX_NUM {
            ret = clk_prepare_enable((*asrc_priv).asrck_clk[i as usize]);
            if ret != 0 {
                i -= 1;
                while i >= 0 {
                    clk_disable_unprepare((*asrc_priv).asrck_clk[i as usize]);
                    i -= 1;
                }
                if !IS_ERR((*asrc).spba_clk) {
                    clk_disable_unprepare((*asrc).spba_clk);
                }
                clk_disable_unprepare((*asrc).ipg_clk);
                clk_disable_unprepare((*asrc).mem_clk);
                return ret;
            }
            i += 1;
        }

        /* Stop all pairs provisionally */
        regmap_read((*asrc).regmap, REG_ASRCTR, &mut asrctr);
        regmap_update_bits((*asrc).regmap, REG_ASRCTR, ASRCTR_ASRCEi_ALL_MASK, 0);

        /* Restore all registers */
        regcache_cache_only((*asrc).regmap, false);
        regcache_mark_dirty((*asrc).regmap);
        regcache_sync((*asrc).regmap);

        regmap_update_bits(
            (*asrc).regmap,
            REG_ASRCFG,
            ASRCFG_NDPRi_ALL_MASK | ASRCFG_POSTMODi_ALL_MASK | ASRCFG_PREMODi_ALL_MASK,
            (*asrc_priv).regcache_cfg,
        );

        /* Restart enabled pairs */
        regmap_update_bits((*asrc).regmap, REG_ASRCTR, ASRCTR_ASRCEi_ALL_MASK, asrctr);

        /* Wait for status of initialization for all enabled pairs */
        loop {
            udelay(5);
            regmap_read((*asrc).regmap, REG_ASRCFG, &mut reg);
            reg = (reg >> ASRCFG_INIRQi_SHIFT(0)) & 0x7;
            retry -= 1;
            if reg == ((asrctr >> ASRCTR_ASRCEi_SHIFT(0)) & 0x7) as c_int || retry == 0 {
                break;
            }
        }

        if retry == 0 {
            i = ASRC_PAIR_A;
            while i < ASRC_PAIR_MAX_NUM {
                if (asrctr & ASRCTR_ASRCEi_MASK(i)) != 0 && (reg & (1 << i)) == 0 {
                    dev_warn(dev, "Pair %c initialization isn't finished\n", b'A' as c_int + i);
                }
                i += 1;
            }
        }

        0
    }
}

unsafe fn goto_disable_mem_clk(asrc: *mut fsl_asrc, ret: c_int) -> c_int {
    clk_disable_unprepare((*asrc).mem_clk);
    ret
}

unsafe fn fsl_asrc_runtime_suspend(dev: *mut device) -> c_int {
    let asrc: *mut fsl_asrc = dev_get_drvdata(dev);
    let asrc_priv: *mut fsl_asrc_priv = (*asrc).private;
    let mut i: c_int;

    regmap_read((*asrc).regmap, REG_ASRCFG, &mut (*asrc_priv).regcache_cfg);

    regcache_cache_only((*asrc).regmap, true);

    i = 0;
    while i < ASRC_CLK_MAX_NUM {
        clk_disable_unprepare((*asrc_priv).asrck_clk[i as usize]);
        i += 1;
    }
    if !IS_ERR((*asrc).spba_clk) {
        clk_disable_unprepare((*asrc).spba_clk);
    }
    clk_disable_unprepare((*asrc).ipg_clk);
    clk_disable_unprepare((*asrc).mem_clk);

    0
}

unsafe fn fsl_asrc_suspend(dev: *mut device) -> c_int {
    let asrc: *mut fsl_asrc = dev_get_drvdata(dev);
    let ret: c_int;

    fsl_asrc_m2m_suspend(asrc);
    ret = pm_runtime_force_suspend(dev);
    ret
}

unsafe fn fsl_asrc_resume(dev: *mut device) -> c_int {
    let asrc: *mut fsl_asrc = dev_get_drvdata(dev);
    let ret: c_int;

    ret = pm_runtime_force_resume(dev);
    fsl_asrc_m2m_resume(asrc);
    ret
}

static mut fsl_asrc_pm: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(fsl_asrc_runtime_suspend),
    runtime_resume: Some(fsl_asrc_runtime_resume),
    suspend: Some(fsl_asrc_suspend),
    resume: Some(fsl_asrc_resume),
};

static mut fsl_asrc_imx35_data: fsl_asrc_soc_data = fsl_asrc_soc_data {
    use_edma: false,
    channel_bits: 3,
    start_before_dma: false,
};

static mut fsl_asrc_imx53_data: fsl_asrc_soc_data = fsl_asrc_soc_data {
    use_edma: false,
    channel_bits: 4,
    start_before_dma: false,
};

static mut fsl_asrc_imx8qm_data: fsl_asrc_soc_data = fsl_asrc_soc_data {
    use_edma: true,
    channel_bits: 4,
    start_before_dma: false,
};

static mut fsl_asrc_imx8qxp_data: fsl_asrc_soc_data = fsl_asrc_soc_data {
    use_edma: true,
    channel_bits: 4,
    start_before_dma: false,
};

static mut fsl_asrc_imx952_data: fsl_asrc_soc_data = fsl_asrc_soc_data {
    use_edma: true,
    channel_bits: 4,
    start_before_dma: true,
};

static mut fsl_asrc_ids: [of_device_id; 6] = [
    of_device_id { compatible: "fsl,imx35-asrc\0".as_ptr() as *const c_char, data: unsafe { &mut fsl_asrc_imx35_data as *mut _ as *const c_void } },
    of_device_id { compatible: "fsl,imx53-asrc\0".as_ptr() as *const c_char, data: unsafe { &mut fsl_asrc_imx53_data as *mut _ as *const c_void } },
    of_device_id { compatible: "fsl,imx8qm-asrc\0".as_ptr() as *const c_char, data: unsafe { &mut fsl_asrc_imx8qm_data as *mut _ as *const c_void } },
    of_device_id { compatible: "fsl,imx8qxp-asrc\0".as_ptr() as *const c_char, data: unsafe { &mut fsl_asrc_imx8qxp_data as *mut _ as *const c_void } },
    of_device_id { compatible: "fsl,imx952-asrc\0".as_ptr() as *const c_char, data: unsafe { &mut fsl_asrc_imx952_data as *mut _ as *const c_void } },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe fn fsl_asrc_probe(pdev: *mut platform_device) -> c_int {
    let np: *mut device_node = (*pdev).dev.of_node;
    let mut asrc_priv: *mut fsl_asrc_priv;
    let mut asrc: *mut fsl_asrc;
    let mut res: *mut resource = core::ptr::null_mut();
    let regs: *mut c_void;
    let mut irq: c_int;
    let mut ret: c_int;
    let mut i: c_int;
    let mut asrc_fmt: u32_ = 0;
    let mut map_idx: u32_ = 0;
    let mut tmp: [c_char; 16] = [0; 16];
    let mut width: u32_ = 0;

    asrc = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<fsl_asrc>(), GFP_KERNEL) as *mut fsl_asrc;
    if asrc.is_null() {
        return -ENOMEM;
    }

    asrc_priv = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<fsl_asrc_priv>(), GFP_KERNEL) as *mut fsl_asrc_priv;
    if asrc_priv.is_null() {
        return -ENOMEM;
    }

    (*asrc).pdev = pdev;
    (*asrc).private = asrc_priv;

    /* Get the addresses and IRQ */
    regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(regs) {
        return PTR_ERR(regs);
    }

    (*asrc).paddr = (*res).start;

    (*asrc).regmap = devm_regmap_init_mmio(&mut (*pdev).dev, regs, &mut fsl_asrc_regmap_config);
    if IS_ERR((*asrc).regmap) {
        dev_err(&mut (*pdev).dev, "failed to init regmap\n");
        return PTR_ERR((*asrc).regmap);
    }

    irq = platform_get_irq(pdev, 0);
    if irq < 0 {
        return irq;
    }

    ret = devm_request_irq(&mut (*pdev).dev, irq, Some(fsl_asrc_isr), 0, dev_name(&mut (*pdev).dev), asrc as *mut c_void);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, "failed to claim irq %u: %d\n", irq, ret);
        return ret;
    }

    (*asrc).mem_clk = devm_clk_get(&mut (*pdev).dev, "mem\0".as_ptr() as *const c_char);
    if IS_ERR((*asrc).mem_clk) {
        dev_err(&mut (*pdev).dev, "failed to get mem clock\n");
        return PTR_ERR((*asrc).mem_clk);
    }

    (*asrc).ipg_clk = devm_clk_get(&mut (*pdev).dev, "ipg\0".as_ptr() as *const c_char);
    if IS_ERR((*asrc).ipg_clk) {
        dev_err(&mut (*pdev).dev, "failed to get ipg clock\n");
        return PTR_ERR((*asrc).ipg_clk);
    }

    (*asrc).spba_clk = devm_clk_get(&mut (*pdev).dev, "spba\0".as_ptr() as *const c_char);
    if IS_ERR((*asrc).spba_clk) {
        dev_warn(&mut (*pdev).dev, "failed to get spba clock\n");
    }

    i = 0;
    while i < ASRC_CLK_MAX_NUM {
        sprintf(tmp.as_mut_ptr(), "asrck_%x", i);
        (*asrc_priv).asrck_clk[i as usize] = devm_clk_get(&mut (*pdev).dev, tmp.as_mut_ptr());
        if IS_ERR((*asrc_priv).asrck_clk[i as usize]) {
            dev_err(&mut (*pdev).dev, "failed to get %s clock\n", tmp.as_mut_ptr());
            return PTR_ERR((*asrc_priv).asrck_clk[i as usize]);
        }
        i += 1;
    }

    (*asrc_priv).soc = of_device_get_match_data(&mut (*pdev).dev) as *mut fsl_asrc_soc_data;
    (*asrc).use_edma = (*(*asrc_priv).soc).use_edma;
    (*asrc).start_before_dma = (*(*asrc_priv).soc).start_before_dma;
    (*asrc).get_dma_channel = Some(fsl_asrc_get_dma_channel);
    (*asrc).request_pair = Some(fsl_asrc_request_pair);
    (*asrc).release_pair = Some(fsl_asrc_release_pair);
    (*asrc).get_fifo_addr = Some(fsl_asrc_get_fifo_addr);
    (*asrc).pair_priv_size = core::mem::size_of::<fsl_asrc_pair_priv>();

    (*asrc).m2m_prepare = Some(fsl_asrc_m2m_prepare);
    (*asrc).m2m_start = Some(fsl_asrc_m2m_start);
    (*asrc).m2m_stop = Some(fsl_asrc_m2m_stop);
    (*asrc).get_output_fifo_size = Some(fsl_asrc_get_output_fifo_size);
    (*asrc).m2m_calc_out_len = Some(fsl_asrc_m2m_calc_out_len);
    (*asrc).m2m_get_maxburst = Some(fsl_asrc_m2m_get_maxburst);
    (*asrc).m2m_pair_resume = Some(fsl_asrc_m2m_pair_resume);
    (*asrc).m2m_get_cap = Some(fsl_asrc_m2m_get_cap);
    (*asrc).m2m_output_ready = Some(fsl_asrc_m2m_output_ready);

    if of_device_is_compatible(np, "fsl,imx35-asrc\0".as_ptr() as *const c_char) {
        (*asrc_priv).clk_map[IN as usize] = input_clk_map_imx35.as_mut_ptr();
        (*asrc_priv).clk_map[OUT as usize] = output_clk_map_imx35.as_mut_ptr();
    } else if of_device_is_compatible(np, "fsl,imx53-asrc\0".as_ptr() as *const c_char) {
        (*asrc_priv).clk_map[IN as usize] = input_clk_map_imx53.as_mut_ptr();
        (*asrc_priv).clk_map[OUT as usize] = output_clk_map_imx53.as_mut_ptr();
    } else if of_device_is_compatible(np, "fsl,imx8qm-asrc\0".as_ptr() as *const c_char) ||
        of_device_is_compatible(np, "fsl,imx8qxp-asrc\0".as_ptr() as *const c_char) {
        ret = of_property_read_u32(np, "fsl,asrc-clk-map\0".as_ptr() as *const c_char, &mut map_idx);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, "failed to get clk map index\n");
            return ret;
        }

        if map_idx > 1 {
            dev_err(&mut (*pdev).dev, "unsupported clk map index\n");
            return -EINVAL;
        }
        if of_device_is_compatible(np, "fsl,imx8qm-asrc\0".as_ptr() as *const c_char) {
            (*asrc_priv).clk_map[IN as usize] = clk_map_imx8qm[map_idx as usize].as_mut_ptr();
            (*asrc_priv).clk_map[OUT as usize] = clk_map_imx8qm[map_idx as usize].as_mut_ptr();
        } else {
            (*asrc_priv).clk_map[IN as usize] = clk_map_imx8qxp[map_idx as usize].as_mut_ptr();
            (*asrc_priv).clk_map[OUT as usize] = clk_map_imx8qxp[map_idx as usize].as_mut_ptr();
        }
    } else if of_device_is_compatible(np, "fsl,imx952-asrc\0".as_ptr() as *const c_char) {
        (*asrc_priv).clk_map[IN as usize] = clk_map_imx952.as_mut_ptr();
        (*asrc_priv).clk_map[OUT as usize] = clk_map_imx952.as_mut_ptr();
    }

    (*asrc).channel_avail = 10;

    ret = of_property_read_u32(np, "fsl,asrc-rate\0".as_ptr() as *const c_char, &mut (*asrc).asrc_rate);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, "failed to get output rate\n");
        return ret;
    }

    ret = of_property_read_u32(np, "fsl,asrc-format\0".as_ptr() as *const c_char, &mut asrc_fmt);
    (*asrc).asrc_format = asrc_fmt;
    if ret != 0 {
        ret = of_property_read_u32(np, "fsl,asrc-width\0".as_ptr() as *const c_char, &mut width);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, "failed to decide output format\n");
            return ret;
        }

        match width {
            16 => (*asrc).asrc_format = SNDRV_PCM_FORMAT_S16_LE,
            24 => (*asrc).asrc_format = SNDRV_PCM_FORMAT_S24_LE,
            _ => {
                dev_warn(&mut (*pdev).dev, "unsupported width, use default S24_LE\n");
                (*asrc).asrc_format = SNDRV_PCM_FORMAT_S24_LE;
            }
        }
    }

    if (FSL_ASRC_FORMATS & pcm_format_to_bits((*asrc).asrc_format)) == 0 {
        dev_warn(&mut (*pdev).dev, "unsupported width, use default S24_LE\n");
        (*asrc).asrc_format = SNDRV_PCM_FORMAT_S24_LE;
    }

    platform_set_drvdata(pdev, asrc);
    spin_lock_init(&mut (*asrc).lock);
    pm_runtime_enable(&mut (*pdev).dev);
    if !pm_runtime_enabled(&mut (*pdev).dev) {
        ret = fsl_asrc_runtime_resume(&mut (*pdev).dev);
        if ret != 0 {
            pm_runtime_disable(&mut (*pdev).dev);
            return ret;
        }
    }

    ret = pm_runtime_resume_and_get(&mut (*pdev).dev);
    if ret < 0 {
        if !pm_runtime_status_suspended(&mut (*pdev).dev) {
            fsl_asrc_runtime_suspend(&mut (*pdev).dev);
        }
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    ret = fsl_asrc_init(asrc);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, "failed to init asrc %d\n", ret);
        if !pm_runtime_status_suspended(&mut (*pdev).dev) {
            fsl_asrc_runtime_suspend(&mut (*pdev).dev);
        }
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    ret = pm_runtime_put_sync(&mut (*pdev).dev);
    if ret < 0 && ret != -ENOSYS {
        if !pm_runtime_status_suspended(&mut (*pdev).dev) {
            fsl_asrc_runtime_suspend(&mut (*pdev).dev);
        }
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    ret = devm_snd_soc_register_component(&mut (*pdev).dev, &mut fsl_asrc_component, &mut fsl_asrc_dai, 1);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, "failed to register ASoC DAI\n");
        if !pm_runtime_status_suspended(&mut (*pdev).dev) {
            fsl_asrc_runtime_suspend(&mut (*pdev).dev);
        }
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    ret = fsl_asrc_m2m_init(asrc);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, "failed to init m2m device %d\n", ret);
        if !pm_runtime_status_suspended(&mut (*pdev).dev) {
            fsl_asrc_runtime_suspend(&mut (*pdev).dev);
        }
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    0
}

unsafe fn fsl_asrc_remove(pdev: *mut platform_device) {
    let asrc: *mut fsl_asrc = dev_get_drvdata(&mut (*pdev).dev);

    fsl_asrc_m2m_exit(asrc);

    pm_runtime_disable(&mut (*pdev).dev);
    if !pm_runtime_status_suspended(&mut (*pdev).dev) {
        fsl_asrc_runtime_suspend(&mut (*pdev).dev);
    }
}

static mut fsl_asrc_driver: platform_driver = platform_driver {
    probe: Some(fsl_asrc_probe),
    remove: Some(fsl_asrc_remove),
    driver: device_driver {
        name: "fsl-asrc\0".as_ptr() as *const c_char,
        of_match_table: unsafe { fsl_asrc_ids.as_ptr() },
        pm: unsafe { &mut fsl_asrc_pm },
    },
};

// MODULE_DEVICE_TABLE(of, fsl_asrc_ids);
// module_platform_driver(fsl_asrc_driver);
// MODULE_DESCRIPTION("Freescale ASRC ASoC driver");
// MODULE_AUTHOR("Nicolin Chen <nicoleotsuka@gmail.com>");
// MODULE_ALIAS("platform:fsl-asrc");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
