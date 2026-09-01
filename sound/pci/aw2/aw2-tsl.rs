// SPDX-License-Identifier: GPL-2.0-only
/*****************************************************************************
 *
 * Copyright (C) 2008 Cedric Bregardis <cedric.bregardis@free.fr> and
 * Jean-Christian Hassler <jhassler@free.fr>
 * Copyright 1998 Emagic Soft- und Hardware GmbH
 * Copyright 2002 Martijn Sipkema
 *
 * This file is part of the Audiowerk2 ALSA driver
 *
 *****************************************************************************/

const TSL_WS0: i32 = (1u32 << 31) as i32;
const TSL_WS1: i32 = (1u32 << 30) as i32;
const TSL_WS2: i32 = (1u32 << 29) as i32;
const TSL_WS3: i32 = (1u32 << 28) as i32;
const TSL_WS4: i32 = (1u32 << 27) as i32;
const TSL_DIS_A1: i32 = (1u32 << 24) as i32;
const TSL_SDW_A1: i32 = (1u32 << 23) as i32;
const TSL_SIB_A1: i32 = (1u32 << 22) as i32;
const TSL_SF_A1: i32 = (1u32 << 21) as i32;
const TSL_LF_A1: i32 = (1u32 << 20) as i32;
const TSL_BSEL_A1: i32 = (1u32 << 17) as i32;
const TSL_DOD_A1: i32 = (1u32 << 15) as i32;
const TSL_LOW_A1: i32 = (1u32 << 14) as i32;
const TSL_DIS_A2: i32 = (1u32 << 11) as i32;
const TSL_SDW_A2: i32 = (1u32 << 10) as i32;
const TSL_SIB_A2: i32 = (1u32 << 9) as i32;
const TSL_SF_A2: i32 = (1u32 << 8) as i32;
const TSL_LF_A2: i32 = (1u32 << 7) as i32;
const TSL_BSEL_A2: i32 = (1u32 << 4) as i32;
const TSL_DOD_A2: i32 = (1u32 << 2) as i32;
const TSL_LOW_A2: i32 = (1u32 << 1) as i32;
const TSL_EOS: i32 = (1u32 << 0) as i32;

/* Audiowerk8 hardware setup: */
/*      WS0, SD4, TSL1  - Analog/ digital in */
/*      WS1, SD0, TSL1  - Analog out #1, digital out */
/*      WS2, SD2, TSL1  - Analog out #2 */
/*      WS3, SD1, TSL2  - Analog out #3 */
/*      WS4, SD3, TSL2  - Analog out #4 */

/* Audiowerk8 timing: */
/*      Timeslot:     | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | ... */

/*      A1_INPUT: */
/*      SD4:          <_ADC-L_>-------<_ADC-R_>-------< */
/*      WS0:          _______________/---------------\_ */

/*      A1_OUTPUT: */
/*      SD0:          <_1-L___>-------<_1-R___>-------< */
/*      WS1:          _______________/---------------\_ */
/*      SD2:          >-------<_2-L___>-------<_2-R___> */
/*      WS2:          -------\_______________/--------- */

/*      A2_OUTPUT: */
/*      SD1:          <_3-L___>-------<_3-R___>-------< */
/*      WS3:          _______________/---------------\_ */
/*      SD3:          >-------<_4-L___>-------<_4-R___> */
/*      WS4:          -------\_______________/--------- */

static TSL1: [i32; 8] = [
    1 * TSL_SDW_A1 | 3 * TSL_BSEL_A1 |
    0 * TSL_DIS_A1 | 0 * TSL_DOD_A1 | TSL_LF_A1,

    1 * TSL_SDW_A1 | 2 * TSL_BSEL_A1 |
    0 * TSL_DIS_A1 | 0 * TSL_DOD_A1,

    0 * TSL_SDW_A1 | 3 * TSL_BSEL_A1 |
    0 * TSL_DIS_A1 | 0 * TSL_DOD_A1,

    0 * TSL_SDW_A1 | 2 * TSL_BSEL_A1 |
    0 * TSL_DIS_A1 | 0 * TSL_DOD_A1,

    1 * TSL_SDW_A1 | 1 * TSL_BSEL_A1 |
    0 * TSL_DIS_A1 | 0 * TSL_DOD_A1 | TSL_WS1 | TSL_WS0,

    1 * TSL_SDW_A1 | 0 * TSL_BSEL_A1 |
    0 * TSL_DIS_A1 | 0 * TSL_DOD_A1 | TSL_WS1 | TSL_WS0,

    0 * TSL_SDW_A1 | 1 * TSL_BSEL_A1 |
    0 * TSL_DIS_A1 | 0 * TSL_DOD_A1 | TSL_WS1 | TSL_WS0,

    0 * TSL_SDW_A1 | 0 * TSL_BSEL_A1 | 0 * TSL_DIS_A1 |
    0 * TSL_DOD_A1 | TSL_WS1 | TSL_WS0 | TSL_SF_A1 | TSL_EOS,
];

static TSL2: [i32; 8] = [
    0 * TSL_SDW_A2 | 3 * TSL_BSEL_A2 | 2 * TSL_DOD_A2 | TSL_LF_A2,
    0 * TSL_SDW_A2 | 2 * TSL_BSEL_A2 | 2 * TSL_DOD_A2,
    0 * TSL_SDW_A2 | 3 * TSL_BSEL_A2 | 2 * TSL_DOD_A2,
    0 * TSL_SDW_A2 | 2 * TSL_BSEL_A2 | 2 * TSL_DOD_A2,
    0 * TSL_SDW_A2 | 1 * TSL_BSEL_A2 | 2 * TSL_DOD_A2 | TSL_WS2,
    0 * TSL_SDW_A2 | 0 * TSL_BSEL_A2 | 2 * TSL_DOD_A2 | TSL_WS2,
    0 * TSL_SDW_A2 | 1 * TSL_BSEL_A2 | 2 * TSL_DOD_A2 | TSL_WS2,
    0 * TSL_SDW_A2 | 0 * TSL_BSEL_A2 | 2 * TSL_DOD_A2 | TSL_WS2 | TSL_EOS,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
