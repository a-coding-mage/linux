// SPDX-License-Identifier: GPL-2.0
// tscs454.h -- TSCS454 ALSA SoC Audio driver
// Copyright 2018 Tempo Semiconductor, Inc.
// Author: Steven Eckhoff <steven.eckhoff.opensource@gmail.com>

pub const VIRT_BASE: u32 = 0x00;
pub const PAGE_LEN: u32 = 0x100;
pub const fn VIRT_PAGE_BASE(page: u32) -> u32 { VIRT_BASE + (PAGE_LEN * page) }
pub const fn VIRT_ADDR(page: u32, address: u32) -> u32 { VIRT_PAGE_BASE(page) + address }
pub const fn ADDR(page: u32, virt_address: u32) -> u32 { virt_address - VIRT_PAGE_BASE(page) }




pub const R_PAGESEL: u32 = 0x0;
pub const R_RESET: u32 = VIRT_ADDR(0x0, 0x1);
pub const R_IRQEN: u32 = VIRT_ADDR(0x0, 0x2);
pub const R_IRQMASK: u32 = VIRT_ADDR(0x0, 0x3);
pub const R_IRQSTAT: u32 = VIRT_ADDR(0x0, 0x4);
pub const R_DEVADD0: u32 = VIRT_ADDR(0x0, 0x6);
pub const R_DEVID: u32 = VIRT_ADDR(0x0, 0x8);
pub const R_DEVREV: u32 = VIRT_ADDR(0x0, 0x9);
pub const R_PLLSTAT: u32 = VIRT_ADDR(0x0, 0x0A);
pub const R_PLL1CTL: u32 = VIRT_ADDR(0x0, 0x0B);
pub const R_PLL1RDIV: u32 = VIRT_ADDR(0x0, 0x0C);
pub const R_PLL1ODIV: u32 = VIRT_ADDR(0x0, 0x0D);
pub const R_PLL1FDIVL: u32 = VIRT_ADDR(0x0, 0x0E);
pub const R_PLL1FDIVH: u32 = VIRT_ADDR(0x0, 0x0F);
pub const R_PLL2CTL: u32 = VIRT_ADDR(0x0, 0x10);
pub const R_PLL2RDIV: u32 = VIRT_ADDR(0x0, 0x11);
pub const R_PLL2ODIV: u32 = VIRT_ADDR(0x0, 0x12);
pub const R_PLL2FDIVL: u32 = VIRT_ADDR(0x0, 0x13);
pub const R_PLL2FDIVH: u32 = VIRT_ADDR(0x0, 0x14);
pub const R_PLLCTL: u32 = VIRT_ADDR(0x0, 0x15);
pub const R_ISRC: u32 = VIRT_ADDR(0x0, 0x16);
pub const R_SCLKCTL: u32 = VIRT_ADDR(0x0, 0x18);
pub const R_TIMEBASE: u32 = VIRT_ADDR(0x0, 0x19);
pub const R_I2SP1CTL: u32 = VIRT_ADDR(0x0, 0x1A);
pub const R_I2SP2CTL: u32 = VIRT_ADDR(0x0, 0x1B);
pub const R_I2SP3CTL: u32 = VIRT_ADDR(0x0, 0x1C);
pub const R_I2S1MRATE: u32 = VIRT_ADDR(0x0, 0x1D);
pub const R_I2S2MRATE: u32 = VIRT_ADDR(0x0, 0x1E);
pub const R_I2S3MRATE: u32 = VIRT_ADDR(0x0, 0x1F);
pub const R_I2SCMC: u32 = VIRT_ADDR(0x0, 0x20);
pub const R_MCLK2PINC: u32 = VIRT_ADDR(0x0, 0x21);
pub const R_I2SPINC0: u32 = VIRT_ADDR(0x0, 0x22);
pub const R_I2SPINC1: u32 = VIRT_ADDR(0x0, 0x23);
pub const R_I2SPINC2: u32 = VIRT_ADDR(0x0, 0x24);
pub const R_GPIOCTL0: u32 = VIRT_ADDR(0x0, 0x25);
pub const R_GPIOCTL1: u32 = VIRT_ADDR(0x0, 0x26);
pub const R_ASRC: u32 = VIRT_ADDR(0x0, 0x28);
pub const R_TDMCTL0: u32 = VIRT_ADDR(0x0, 0x2D);
pub const R_TDMCTL1: u32 = VIRT_ADDR(0x0, 0x2E);
pub const R_PCMP2CTL0: u32 = VIRT_ADDR(0x0, 0x2F);
pub const R_PCMP2CTL1: u32 = VIRT_ADDR(0x0, 0x30);
pub const R_PCMP3CTL0: u32 = VIRT_ADDR(0x0, 0x31);
pub const R_PCMP3CTL1: u32 = VIRT_ADDR(0x0, 0x32);
pub const R_PWRM0: u32 = VIRT_ADDR(0x0, 0x33);
pub const R_PWRM1: u32 = VIRT_ADDR(0x0, 0x34);
pub const R_PWRM2: u32 = VIRT_ADDR(0x0, 0x35);
pub const R_PWRM3: u32 = VIRT_ADDR(0x0, 0x36);
pub const R_PWRM4: u32 = VIRT_ADDR(0x0, 0x37);
pub const R_I2SIDCTL: u32 = VIRT_ADDR(0x0, 0x38);
pub const R_I2SODCTL: u32 = VIRT_ADDR(0x0, 0x39);
pub const R_AUDIOMUX1: u32 = VIRT_ADDR(0x0, 0x3A);
pub const R_AUDIOMUX2: u32 = VIRT_ADDR(0x0, 0x3B);
pub const R_AUDIOMUX3: u32 = VIRT_ADDR(0x0, 0x3C);
pub const R_HSDCTL1: u32 = VIRT_ADDR(0x1, 0x1);
pub const R_HSDCTL2: u32 = VIRT_ADDR(0x1, 0x2);
pub const R_HSDSTAT: u32 = VIRT_ADDR(0x1, 0x3);
pub const R_HSDDELAY: u32 = VIRT_ADDR(0x1, 0x4);
pub const R_BUTCTL: u32 = VIRT_ADDR(0x1, 0x5);
pub const R_CH0AIC: u32 = VIRT_ADDR(0x1, 0x6);
pub const R_CH1AIC: u32 = VIRT_ADDR(0x1, 0x7);
pub const R_CH2AIC: u32 = VIRT_ADDR(0x1, 0x8);
pub const R_CH3AIC: u32 = VIRT_ADDR(0x1, 0x9);
pub const R_ICTL0: u32 = VIRT_ADDR(0x1, 0x0A);
pub const R_ICTL1: u32 = VIRT_ADDR(0x1, 0x0B);
pub const R_MICBIAS: u32 = VIRT_ADDR(0x1, 0x0C);
pub const R_PGACTL0: u32 = VIRT_ADDR(0x1, 0x0D);
pub const R_PGACTL1: u32 = VIRT_ADDR(0x1, 0x0E);
pub const R_PGACTL2: u32 = VIRT_ADDR(0x1, 0x0F);
pub const R_PGACTL3: u32 = VIRT_ADDR(0x1, 0x10);
pub const R_PGAZ: u32 = VIRT_ADDR(0x1, 0x11);
pub const R_ICH0VOL: u32 = VIRT_ADDR(0x1, 0x12);
pub const R_ICH1VOL: u32 = VIRT_ADDR(0x1, 0x13);
pub const R_ICH2VOL: u32 = VIRT_ADDR(0x1, 0x14);
pub const R_ICH3VOL: u32 = VIRT_ADDR(0x1, 0x15);
pub const R_ASRCILVOL: u32 = VIRT_ADDR(0x1, 0x16);
pub const R_ASRCIRVOL: u32 = VIRT_ADDR(0x1, 0x17);
pub const R_ASRCOLVOL: u32 = VIRT_ADDR(0x1, 0x18);
pub const R_ASRCORVOL: u32 = VIRT_ADDR(0x1, 0x19);
pub const R_IVOLCTLU: u32 = VIRT_ADDR(0x1, 0x1C);
pub const R_ALCCTL0: u32 = VIRT_ADDR(0x1, 0x1D);
pub const R_ALCCTL1: u32 = VIRT_ADDR(0x1, 0x1E);
pub const R_ALCCTL2: u32 = VIRT_ADDR(0x1, 0x1F);
pub const R_ALCCTL3: u32 = VIRT_ADDR(0x1, 0x20);
pub const R_NGATE: u32 = VIRT_ADDR(0x1, 0x21);
pub const R_DMICCTL: u32 = VIRT_ADDR(0x1, 0x22);
pub const R_DACCTL: u32 = VIRT_ADDR(0x2, 0x1);
pub const R_SPKCTL: u32 = VIRT_ADDR(0x2, 0x2);
pub const R_SUBCTL: u32 = VIRT_ADDR(0x2, 0x3);
pub const R_DCCTL: u32 = VIRT_ADDR(0x2, 0x4);
pub const R_OVOLCTLU: u32 = VIRT_ADDR(0x2, 0x6);
pub const R_MUTEC: u32 = VIRT_ADDR(0x2, 0x7);
pub const R_MVOLL: u32 = VIRT_ADDR(0x2, 0x8);
pub const R_MVOLR: u32 = VIRT_ADDR(0x2, 0x9);
pub const R_HPVOLL: u32 = VIRT_ADDR(0x2, 0x0A);
pub const R_HPVOLR: u32 = VIRT_ADDR(0x2, 0x0B);
pub const R_SPKVOLL: u32 = VIRT_ADDR(0x2, 0x0C);
pub const R_SPKVOLR: u32 = VIRT_ADDR(0x2, 0x0D);
pub const R_SUBVOL: u32 = VIRT_ADDR(0x2, 0x10);
pub const R_COP0: u32 = VIRT_ADDR(0x2, 0x11);
pub const R_COP1: u32 = VIRT_ADDR(0x2, 0x12);
pub const R_COPSTAT: u32 = VIRT_ADDR(0x2, 0x13);
pub const R_PWM0: u32 = VIRT_ADDR(0x2, 0x14);
pub const R_PWM1: u32 = VIRT_ADDR(0x2, 0x15);
pub const R_PWM2: u32 = VIRT_ADDR(0x2, 0x16);
pub const R_PWM3: u32 = VIRT_ADDR(0x2, 0x17);
pub const R_HPSW: u32 = VIRT_ADDR(0x2, 0x18);
pub const R_THERMTS: u32 = VIRT_ADDR(0x2, 0x19);
pub const R_THERMSPK1: u32 = VIRT_ADDR(0x2, 0x1A);
pub const R_THERMSTAT: u32 = VIRT_ADDR(0x2, 0x1B);
pub const R_SCSTAT: u32 = VIRT_ADDR(0x2, 0x1C);
pub const R_SDMON: u32 = VIRT_ADDR(0x2, 0x1D);
pub const R_SPKEQFILT: u32 = VIRT_ADDR(0x3, 0x1);
pub const R_SPKCRWDL: u32 = VIRT_ADDR(0x3, 0x2);
pub const R_SPKCRWDM: u32 = VIRT_ADDR(0x3, 0x3);
pub const R_SPKCRWDH: u32 = VIRT_ADDR(0x3, 0x4);
pub const R_SPKCRRDL: u32 = VIRT_ADDR(0x3, 0x5);
pub const R_SPKCRRDM: u32 = VIRT_ADDR(0x3, 0x6);
pub const R_SPKCRRDH: u32 = VIRT_ADDR(0x3, 0x7);
pub const R_SPKCRADD: u32 = VIRT_ADDR(0x3, 0x8);
pub const R_SPKCRS: u32 = VIRT_ADDR(0x3, 0x9);
pub const R_SPKMBCEN: u32 = VIRT_ADDR(0x3, 0x0A);
pub const R_SPKMBCCTL: u32 = VIRT_ADDR(0x3, 0x0B);
pub const R_SPKMBCMUG1: u32 = VIRT_ADDR(0x3, 0x0C);
pub const R_SPKMBCTHR1: u32 = VIRT_ADDR(0x3, 0x0D);
pub const R_SPKMBCRAT1: u32 = VIRT_ADDR(0x3, 0x0E);
pub const R_SPKMBCATK1L: u32 = VIRT_ADDR(0x3, 0x0F);
pub const R_SPKMBCATK1H: u32 = VIRT_ADDR(0x3, 0x10);
pub const R_SPKMBCREL1L: u32 = VIRT_ADDR(0x3, 0x11);
pub const R_SPKMBCREL1H: u32 = VIRT_ADDR(0x3, 0x12);
pub const R_SPKMBCMUG2: u32 = VIRT_ADDR(0x3, 0x13);
pub const R_SPKMBCTHR2: u32 = VIRT_ADDR(0x3, 0x14);
pub const R_SPKMBCRAT2: u32 = VIRT_ADDR(0x3, 0x15);
pub const R_SPKMBCATK2L: u32 = VIRT_ADDR(0x3, 0x16);
pub const R_SPKMBCATK2H: u32 = VIRT_ADDR(0x3, 0x17);
pub const R_SPKMBCREL2L: u32 = VIRT_ADDR(0x3, 0x18);
pub const R_SPKMBCREL2H: u32 = VIRT_ADDR(0x3, 0x19);
pub const R_SPKMBCMUG3: u32 = VIRT_ADDR(0x3, 0x1A);
pub const R_SPKMBCTHR3: u32 = VIRT_ADDR(0x3, 0x1B);
pub const R_SPKMBCRAT3: u32 = VIRT_ADDR(0x3, 0x1C);
pub const R_SPKMBCATK3L: u32 = VIRT_ADDR(0x3, 0x1D);
pub const R_SPKMBCATK3H: u32 = VIRT_ADDR(0x3, 0x1E);
pub const R_SPKMBCREL3L: u32 = VIRT_ADDR(0x3, 0x1F);
pub const R_SPKMBCREL3H: u32 = VIRT_ADDR(0x3, 0x20);
pub const R_SPKCLECTL: u32 = VIRT_ADDR(0x3, 0x21);
pub const R_SPKCLEMUG: u32 = VIRT_ADDR(0x3, 0x22);
pub const R_SPKCOMPTHR: u32 = VIRT_ADDR(0x3, 0x23);
pub const R_SPKCOMPRAT: u32 = VIRT_ADDR(0x3, 0x24);
pub const R_SPKCOMPATKL: u32 = VIRT_ADDR(0x3, 0x25);
pub const R_SPKCOMPATKH: u32 = VIRT_ADDR(0x3, 0x26);
pub const R_SPKCOMPRELL: u32 = VIRT_ADDR(0x3, 0x27);
pub const R_SPKCOMPRELH: u32 = VIRT_ADDR(0x3, 0x28);
pub const R_SPKLIMTHR: u32 = VIRT_ADDR(0x3, 0x29);
pub const R_SPKLIMTGT: u32 = VIRT_ADDR(0x3, 0x2A);
pub const R_SPKLIMATKL: u32 = VIRT_ADDR(0x3, 0x2B);
pub const R_SPKLIMATKH: u32 = VIRT_ADDR(0x3, 0x2C);
pub const R_SPKLIMRELL: u32 = VIRT_ADDR(0x3, 0x2D);
pub const R_SPKLIMRELH: u32 = VIRT_ADDR(0x3, 0x2E);
pub const R_SPKEXPTHR: u32 = VIRT_ADDR(0x3, 0x2F);
pub const R_SPKEXPRAT: u32 = VIRT_ADDR(0x3, 0x30);
pub const R_SPKEXPATKL: u32 = VIRT_ADDR(0x3, 0x31);
pub const R_SPKEXPATKH: u32 = VIRT_ADDR(0x3, 0x32);
pub const R_SPKEXPRELL: u32 = VIRT_ADDR(0x3, 0x33);
pub const R_SPKEXPRELH: u32 = VIRT_ADDR(0x3, 0x34);
pub const R_SPKFXCTL: u32 = VIRT_ADDR(0x3, 0x35);
pub const R_DACEQFILT: u32 = VIRT_ADDR(0x4, 0x1);
pub const R_DACCRWDL: u32 = VIRT_ADDR(0x4, 0x2);
pub const R_DACCRWDM: u32 = VIRT_ADDR(0x4, 0x3);
pub const R_DACCRWDH: u32 = VIRT_ADDR(0x4, 0x4);
pub const R_DACCRRDL: u32 = VIRT_ADDR(0x4, 0x5);
pub const R_DACCRRDM: u32 = VIRT_ADDR(0x4, 0x6);
pub const R_DACCRRDH: u32 = VIRT_ADDR(0x4, 0x7);
pub const R_DACCRADD: u32 = VIRT_ADDR(0x4, 0x8);
pub const R_DACCRS: u32 = VIRT_ADDR(0x4, 0x9);
pub const R_DACMBCEN: u32 = VIRT_ADDR(0x4, 0x0A);
pub const R_DACMBCCTL: u32 = VIRT_ADDR(0x4, 0x0B);
pub const R_DACMBCMUG1: u32 = VIRT_ADDR(0x4, 0x0C);
pub const R_DACMBCTHR1: u32 = VIRT_ADDR(0x4, 0x0D);
pub const R_DACMBCRAT1: u32 = VIRT_ADDR(0x4, 0x0E);
pub const R_DACMBCATK1L: u32 = VIRT_ADDR(0x4, 0x0F);
pub const R_DACMBCATK1H: u32 = VIRT_ADDR(0x4, 0x10);
pub const R_DACMBCREL1L: u32 = VIRT_ADDR(0x4, 0x11);
pub const R_DACMBCREL1H: u32 = VIRT_ADDR(0x4, 0x12);
pub const R_DACMBCMUG2: u32 = VIRT_ADDR(0x4, 0x13);
pub const R_DACMBCTHR2: u32 = VIRT_ADDR(0x4, 0x14);
pub const R_DACMBCRAT2: u32 = VIRT_ADDR(0x4, 0x15);
pub const R_DACMBCATK2L: u32 = VIRT_ADDR(0x4, 0x16);
pub const R_DACMBCATK2H: u32 = VIRT_ADDR(0x4, 0x17);
pub const R_DACMBCREL2L: u32 = VIRT_ADDR(0x4, 0x18);
pub const R_DACMBCREL2H: u32 = VIRT_ADDR(0x4, 0x19);
pub const R_DACMBCMUG3: u32 = VIRT_ADDR(0x4, 0x1A);
pub const R_DACMBCTHR3: u32 = VIRT_ADDR(0x4, 0x1B);
pub const R_DACMBCRAT3: u32 = VIRT_ADDR(0x4, 0x1C);
pub const R_DACMBCATK3L: u32 = VIRT_ADDR(0x4, 0x1D);
pub const R_DACMBCATK3H: u32 = VIRT_ADDR(0x4, 0x1E);
pub const R_DACMBCREL3L: u32 = VIRT_ADDR(0x4, 0x1F);
pub const R_DACMBCREL3H: u32 = VIRT_ADDR(0x4, 0x20);
pub const R_DACCLECTL: u32 = VIRT_ADDR(0x4, 0x21);
pub const R_DACCLEMUG: u32 = VIRT_ADDR(0x4, 0x22);
pub const R_DACCOMPTHR: u32 = VIRT_ADDR(0x4, 0x23);
pub const R_DACCOMPRAT: u32 = VIRT_ADDR(0x4, 0x24);
pub const R_DACCOMPATKL: u32 = VIRT_ADDR(0x4, 0x25);
pub const R_DACCOMPATKH: u32 = VIRT_ADDR(0x4, 0x26);
pub const R_DACCOMPRELL: u32 = VIRT_ADDR(0x4, 0x27);
pub const R_DACCOMPRELH: u32 = VIRT_ADDR(0x4, 0x28);
pub const R_DACLIMTHR: u32 = VIRT_ADDR(0x4, 0x29);
pub const R_DACLIMTGT: u32 = VIRT_ADDR(0x4, 0x2A);
pub const R_DACLIMATKL: u32 = VIRT_ADDR(0x4, 0x2B);
pub const R_DACLIMATKH: u32 = VIRT_ADDR(0x4, 0x2C);
pub const R_DACLIMRELL: u32 = VIRT_ADDR(0x4, 0x2D);
pub const R_DACLIMRELH: u32 = VIRT_ADDR(0x4, 0x2E);
pub const R_DACEXPTHR: u32 = VIRT_ADDR(0x4, 0x2F);
pub const R_DACEXPRAT: u32 = VIRT_ADDR(0x4, 0x30);
pub const R_DACEXPATKL: u32 = VIRT_ADDR(0x4, 0x31);
pub const R_DACEXPATKH: u32 = VIRT_ADDR(0x4, 0x32);
pub const R_DACEXPRELL: u32 = VIRT_ADDR(0x4, 0x33);
pub const R_DACEXPRELH: u32 = VIRT_ADDR(0x4, 0x34);
pub const R_DACFXCTL: u32 = VIRT_ADDR(0x4, 0x35);
pub const R_SUBEQFILT: u32 = VIRT_ADDR(0x5, 0x1);
pub const R_SUBCRWDL: u32 = VIRT_ADDR(0x5, 0x2);
pub const R_SUBCRWDM: u32 = VIRT_ADDR(0x5, 0x3);
pub const R_SUBCRWDH: u32 = VIRT_ADDR(0x5, 0x4);
pub const R_SUBCRRDL: u32 = VIRT_ADDR(0x5, 0x5);
pub const R_SUBCRRDM: u32 = VIRT_ADDR(0x5, 0x6);
pub const R_SUBCRRDH: u32 = VIRT_ADDR(0x5, 0x7);
pub const R_SUBCRADD: u32 = VIRT_ADDR(0x5, 0x8);
pub const R_SUBCRS: u32 = VIRT_ADDR(0x5, 0x9);
pub const R_SUBMBCEN: u32 = VIRT_ADDR(0x5, 0x0A);
pub const R_SUBMBCCTL: u32 = VIRT_ADDR(0x5, 0x0B);
pub const R_SUBMBCMUG1: u32 = VIRT_ADDR(0x5, 0x0C);
pub const R_SUBMBCTHR1: u32 = VIRT_ADDR(0x5, 0x0D);
pub const R_SUBMBCRAT1: u32 = VIRT_ADDR(0x5, 0x0E);
pub const R_SUBMBCATK1L: u32 = VIRT_ADDR(0x5, 0x0F);
pub const R_SUBMBCATK1H: u32 = VIRT_ADDR(0x5, 0x10);
pub const R_SUBMBCREL1L: u32 = VIRT_ADDR(0x5, 0x11);
pub const R_SUBMBCREL1H: u32 = VIRT_ADDR(0x5, 0x12);
pub const R_SUBMBCMUG2: u32 = VIRT_ADDR(0x5, 0x13);
pub const R_SUBMBCTHR2: u32 = VIRT_ADDR(0x5, 0x14);
pub const R_SUBMBCRAT2: u32 = VIRT_ADDR(0x5, 0x15);
pub const R_SUBMBCATK2L: u32 = VIRT_ADDR(0x5, 0x16);
pub const R_SUBMBCATK2H: u32 = VIRT_ADDR(0x5, 0x17);
pub const R_SUBMBCREL2L: u32 = VIRT_ADDR(0x5, 0x18);
pub const R_SUBMBCREL2H: u32 = VIRT_ADDR(0x5, 0x19);
pub const R_SUBMBCMUG3: u32 = VIRT_ADDR(0x5, 0x1A);
pub const R_SUBMBCTHR3: u32 = VIRT_ADDR(0x5, 0x1B);
pub const R_SUBMBCRAT3: u32 = VIRT_ADDR(0x5, 0x1C);
pub const R_SUBMBCATK3L: u32 = VIRT_ADDR(0x5, 0x1D);
pub const R_SUBMBCATK3H: u32 = VIRT_ADDR(0x5, 0x1E);
pub const R_SUBMBCREL3L: u32 = VIRT_ADDR(0x5, 0x1F);
pub const R_SUBMBCREL3H: u32 = VIRT_ADDR(0x5, 0x20);
pub const R_SUBCLECTL: u32 = VIRT_ADDR(0x5, 0x21);
pub const R_SUBCLEMUG: u32 = VIRT_ADDR(0x5, 0x22);
pub const R_SUBCOMPTHR: u32 = VIRT_ADDR(0x5, 0x23);
pub const R_SUBCOMPRAT: u32 = VIRT_ADDR(0x5, 0x24);
pub const R_SUBCOMPATKL: u32 = VIRT_ADDR(0x5, 0x25);
pub const R_SUBCOMPATKH: u32 = VIRT_ADDR(0x5, 0x26);
pub const R_SUBCOMPRELL: u32 = VIRT_ADDR(0x5, 0x27);
pub const R_SUBCOMPRELH: u32 = VIRT_ADDR(0x5, 0x28);
pub const R_SUBLIMTHR: u32 = VIRT_ADDR(0x5, 0x29);
pub const R_SUBLIMTGT: u32 = VIRT_ADDR(0x5, 0x2A);
pub const R_SUBLIMATKL: u32 = VIRT_ADDR(0x5, 0x2B);
pub const R_SUBLIMATKH: u32 = VIRT_ADDR(0x5, 0x2C);
pub const R_SUBLIMRELL: u32 = VIRT_ADDR(0x5, 0x2D);
pub const R_SUBLIMRELH: u32 = VIRT_ADDR(0x5, 0x2E);
pub const R_SUBEXPTHR: u32 = VIRT_ADDR(0x5, 0x2F);
pub const R_SUBEXPRAT: u32 = VIRT_ADDR(0x5, 0x30);
pub const R_SUBEXPATKL: u32 = VIRT_ADDR(0x5, 0x31);
pub const R_SUBEXPATKH: u32 = VIRT_ADDR(0x5, 0x32);
pub const R_SUBEXPRELL: u32 = VIRT_ADDR(0x5, 0x33);
pub const R_SUBEXPRELH: u32 = VIRT_ADDR(0x5, 0x34);
pub const R_SUBFXCTL: u32 = VIRT_ADDR(0x5, 0x35);

// *** PLLCTL ***
pub const FB_PLLCTL_VCCI_PLL: u32 = 6;
pub const FM_PLLCTL_VCCI_PLL: u32 = 0xC0;

pub const FB_PLLCTL_RZ_PLL: u32 = 3;
pub const FM_PLLCTL_RZ_PLL: u32 = 0x38;

pub const FB_PLLCTL_CP_PLL: u32 = 0;
pub const FM_PLLCTL_CP_PLL: u32 = 0x7;

// *** PLLRDIV ***
pub const FB_PLLRDIV_REFDIV_PLL: u32 = 0;
pub const FM_PLLRDIV_REFDIV_PLL: u32 = 0xFF;

// *** PLLODIV ***
pub const FB_PLLODIV_OUTDIV_PLL: u32 = 0;
pub const FM_PLLODIV_OUTDIV_PLL: u32 = 0xFF;

// *** PLLFDIVL ***
pub const FB_PLLFDIVL_FBDIVL_PLL: u32 = 0;
pub const FM_PLLFDIVL_FBDIVL_PLL: u32 = 0xFF;

// *** PLLFDIVH ***
pub const FB_PLLFDIVH_FBDIVH_PLL: u32 = 0;
pub const FM_PLLFDIVH_FBDIVH_PLL: u32 = 0xF;

// *** I2SPCTL ***
pub const FB_I2SPCTL_BCLKSTAT: u32 = 7;
pub const FM_I2SPCTL_BCLKSTAT: u32 = 0x80;
pub const FV_BCLKSTAT_LOST: u32 = 0x80;
pub const FV_BCLKSTAT_NOT_LOST: u32 = 0x0;

pub const FB_I2SPCTL_BCLKP: u32 = 6;
pub const FM_I2SPCTL_BCLKP: u32 = 0x40;
pub const FV_BCLKP_NOT_INVERTED: u32 = 0x0;
pub const FV_BCLKP_INVERTED: u32 = 0x40;

pub const FB_I2SPCTL_PORTMS: u32 = 5;
pub const FM_I2SPCTL_PORTMS: u32 = 0x20;
pub const FV_PORTMS_SLAVE: u32 = 0x0;
pub const FV_PORTMS_MASTER: u32 = 0x20;

pub const FB_I2SPCTL_LRCLKP: u32 = 4;
pub const FM_I2SPCTL_LRCLKP: u32 = 0x10;
pub const FV_LRCLKP_NOT_INVERTED: u32 = 0x0;
pub const FV_LRCLKP_INVERTED: u32 = 0x10;

pub const FB_I2SPCTL_WL: u32 = 2;
pub const FM_I2SPCTL_WL: u32 = 0xC;
pub const FV_WL_16: u32 = 0x0;
pub const FV_WL_20: u32 = 0x4;
pub const FV_WL_24: u32 = 0x8;
pub const FV_WL_32: u32 = 0xC;

pub const FB_I2SPCTL_FORMAT: u32 = 0;
pub const FM_I2SPCTL_FORMAT: u32 = 0x3;
pub const FV_FORMAT_RIGHT: u32 = 0x0;
pub const FV_FORMAT_LEFT: u32 = 0x1;
pub const FV_FORMAT_I2S: u32 = 0x2;
pub const FV_FORMAT_TDM: u32 = 0x3;

// *** I2SMRATE ***
pub const FB_I2SMRATE_I2SMCLKHALF: u32 = 7;
pub const FM_I2SMRATE_I2SMCLKHALF: u32 = 0x80;
pub const FV_I2SMCLKHALF_I2S1MCLKDIV_DIV_2: u32 = 0x0;
pub const FV_I2SMCLKHALF_I2S1MCLKDIV_ONLY: u32 = 0x80;

pub const FB_I2SMRATE_I2SMCLKDIV: u32 = 5;
pub const FM_I2SMRATE_I2SMCLKDIV: u32 = 0x60;
pub const FV_I2SMCLKDIV_125: u32 = 0x0;
pub const FV_I2SMCLKDIV_128: u32 = 0x20;
pub const FV_I2SMCLKDIV_136: u32 = 0x40;
pub const FV_I2SMCLKDIV_192: u32 = 0x60;

pub const FB_I2SMRATE_I2SMBR: u32 = 3;
pub const FM_I2SMRATE_I2SMBR: u32 = 0x18;
pub const FV_I2SMBR_32: u32 = 0x0;
pub const FV_I2SMBR_44PT1: u32 = 0x8;
pub const FV_I2SMBR_48: u32 = 0x10;
pub const FV_I2SMBR_MCLK_MODE: u32 = 0x18;

pub const FB_I2SMRATE_I2SMBM: u32 = 0;
pub const FM_I2SMRATE_I2SMBM: u32 = 0x3;
pub const FV_I2SMBM_0PT25: u32 = 0x0;
pub const FV_I2SMBM_0PT5: u32 = 0x1;
pub const FV_I2SMBM_1: u32 = 0x2;
pub const FV_I2SMBM_2: u32 = 0x3;

// *** PCMPCTL0 ***
pub const FB_PCMPCTL0_PCMFLENP: u32 = 2;
pub const FM_PCMPCTL0_PCMFLENP: u32 = 0x4;
pub const FV_PCMFLENP_128: u32 = 0x0;
pub const FV_PCMFLENP_256: u32 = 0x4;

pub const FB_PCMPCTL0_SLSYNCP: u32 = 1;
pub const FM_PCMPCTL0_SLSYNCP: u32 = 0x2;
pub const FV_SLSYNCP_SHORT: u32 = 0x0;
pub const FV_SLSYNCP_LONG: u32 = 0x2;

pub const FB_PCMPCTL0_BDELAYP: u32 = 0;
pub const FM_PCMPCTL0_BDELAYP: u32 = 0x1;
pub const FV_BDELAYP_NO_DELAY: u32 = 0x0;
pub const FV_BDELAYP_1BCLK_DELAY: u32 = 0x1;

// *** PCMPCTL1 ***
pub const FB_PCMPCTL1_PCMMOMP: u32 = 6;
pub const FM_PCMPCTL1_PCMMOMP: u32 = 0x40;

pub const FB_PCMPCTL1_PCMSOP: u32 = 5;
pub const FM_PCMPCTL1_PCMSOP: u32 = 0x20;
pub const FV_PCMSOP_1: u32 = 0x0;
pub const FV_PCMSOP_2: u32 = 0x20;

pub const FB_PCMPCTL1_PCMDSSP: u32 = 3;
pub const FM_PCMPCTL1_PCMDSSP: u32 = 0x18;
pub const FV_PCMDSSP_16: u32 = 0x0;
pub const FV_PCMDSSP_24: u32 = 0x8;
pub const FV_PCMDSSP_32: u32 = 0x10;

pub const FB_PCMPCTL1_PCMMIMP: u32 = 1;
pub const FM_PCMPCTL1_PCMMIMP: u32 = 0x2;

pub const FB_PCMPCTL1_PCMSIP: u32 = 0;
pub const FM_PCMPCTL1_PCMSIP: u32 = 0x1;
pub const FV_PCMSIP_1: u32 = 0x0;
pub const FV_PCMSIP_2: u32 = 0x1;

// *** CHAIC ***
pub const FB_CHAIC_MICBST: u32 = 4;
pub const FM_CHAIC_MICBST: u32 = 0x30;

// *** PGACTL ***
pub const FB_PGACTL_PGAMUTE: u32 = 7;
pub const FM_PGACTL_PGAMUTE: u32 = 0x80;

pub const FB_PGACTL_PGAVOL: u32 = 0;
pub const FM_PGACTL_PGAVOL: u32 = 0x3F;

// *** ICHVOL ***
pub const FB_ICHVOL_ICHVOL: u32 = 0;
pub const FM_ICHVOL_ICHVOL: u32 = 0xFF;

// *** SPKMBCMUG ***
pub const FB_SPKMBCMUG_PHASE: u32 = 5;
pub const FM_SPKMBCMUG_PHASE: u32 = 0x20;

pub const FB_SPKMBCMUG_MUGAIN: u32 = 0;
pub const FM_SPKMBCMUG_MUGAIN: u32 = 0x1F;

// *** SPKMBCTHR ***
pub const FB_SPKMBCTHR_THRESH: u32 = 0;
pub const FM_SPKMBCTHR_THRESH: u32 = 0xFF;

// *** SPKMBCRAT ***
pub const FB_SPKMBCRAT_RATIO: u32 = 0;
pub const FM_SPKMBCRAT_RATIO: u32 = 0x1F;

// *** SPKMBCATKL ***
pub const FB_SPKMBCATKL_TCATKL: u32 = 0;
pub const FM_SPKMBCATKL_TCATKL: u32 = 0xFF;

// *** SPKMBCATKH ***
pub const FB_SPKMBCATKH_TCATKH: u32 = 0;
pub const FM_SPKMBCATKH_TCATKH: u32 = 0xFF;

// *** SPKMBCRELL ***
pub const FB_SPKMBCRELL_TCRELL: u32 = 0;
pub const FM_SPKMBCRELL_TCRELL: u32 = 0xFF;

// *** SPKMBCRELH ***
pub const FB_SPKMBCRELH_TCRELH: u32 = 0;
pub const FM_SPKMBCRELH_TCRELH: u32 = 0xFF;

// *** DACMBCMUG ***
pub const FB_DACMBCMUG_PHASE: u32 = 5;
pub const FM_DACMBCMUG_PHASE: u32 = 0x20;

pub const FB_DACMBCMUG_MUGAIN: u32 = 0;
pub const FM_DACMBCMUG_MUGAIN: u32 = 0x1F;

// *** DACMBCTHR ***
pub const FB_DACMBCTHR_THRESH: u32 = 0;
pub const FM_DACMBCTHR_THRESH: u32 = 0xFF;

// *** DACMBCRAT ***
pub const FB_DACMBCRAT_RATIO: u32 = 0;
pub const FM_DACMBCRAT_RATIO: u32 = 0x1F;

// *** DACMBCATKL ***
pub const FB_DACMBCATKL_TCATKL: u32 = 0;
pub const FM_DACMBCATKL_TCATKL: u32 = 0xFF;

// *** DACMBCATKH ***
pub const FB_DACMBCATKH_TCATKH: u32 = 0;
pub const FM_DACMBCATKH_TCATKH: u32 = 0xFF;

// *** DACMBCRELL ***
pub const FB_DACMBCRELL_TCRELL: u32 = 0;
pub const FM_DACMBCRELL_TCRELL: u32 = 0xFF;

// *** DACMBCRELH ***
pub const FB_DACMBCRELH_TCRELH: u32 = 0;
pub const FM_DACMBCRELH_TCRELH: u32 = 0xFF;

// *** SUBMBCMUG ***
pub const FB_SUBMBCMUG_PHASE: u32 = 5;
pub const FM_SUBMBCMUG_PHASE: u32 = 0x20;

pub const FB_SUBMBCMUG_MUGAIN: u32 = 0;
pub const FM_SUBMBCMUG_MUGAIN: u32 = 0x1F;

// *** SUBMBCTHR ***
pub const FB_SUBMBCTHR_THRESH: u32 = 0;
pub const FM_SUBMBCTHR_THRESH: u32 = 0xFF;

// *** SUBMBCRAT ***
pub const FB_SUBMBCRAT_RATIO: u32 = 0;
pub const FM_SUBMBCRAT_RATIO: u32 = 0x1F;

// *** SUBMBCATKL ***
pub const FB_SUBMBCATKL_TCATKL: u32 = 0;
pub const FM_SUBMBCATKL_TCATKL: u32 = 0xFF;

// *** SUBMBCATKH ***
pub const FB_SUBMBCATKH_TCATKH: u32 = 0;
pub const FM_SUBMBCATKH_TCATKH: u32 = 0xFF;

// *** SUBMBCRELL ***
pub const FB_SUBMBCRELL_TCRELL: u32 = 0;
pub const FM_SUBMBCRELL_TCRELL: u32 = 0xFF;

// *** SUBMBCRELH ***
pub const FB_SUBMBCRELH_TCRELH: u32 = 0;
pub const FM_SUBMBCRELH_TCRELH: u32 = 0xFF;

// *** PAGESEL ***
pub const FB_PAGESEL_PAGESEL: u32 = 0;
pub const FM_PAGESEL_PAGESEL: u32 = 0xFF;

// *** RESET ***
pub const FB_RESET_RESET: u32 = 0;
pub const FM_RESET_RESET: u32 = 0xFF;
pub const FV_RESET_PWR_ON_DEFAULTS: u32 = 0x85;

// *** IRQEN ***
pub const FB_IRQEN_THRMINTEN: u32 = 6;
pub const FM_IRQEN_THRMINTEN: u32 = 0x40;
pub const FV_THRMINTEN_ENABLED: u32 = 0x40;
pub const FV_THRMINTEN_DISABLED: u32 = 0x0;

pub const FB_IRQEN_HBPINTEN: u32 = 5;
pub const FM_IRQEN_HBPINTEN: u32 = 0x20;
pub const FV_HBPINTEN_ENABLED: u32 = 0x20;
pub const FV_HBPINTEN_DISABLED: u32 = 0x0;

pub const FB_IRQEN_HSDINTEN: u32 = 4;
pub const FM_IRQEN_HSDINTEN: u32 = 0x10;
pub const FV_HSDINTEN_ENABLED: u32 = 0x10;
pub const FV_HSDINTEN_DISABLED: u32 = 0x0;

pub const FB_IRQEN_HPDINTEN: u32 = 3;
pub const FM_IRQEN_HPDINTEN: u32 = 0x8;
pub const FV_HPDINTEN_ENABLED: u32 = 0x8;
pub const FV_HPDINTEN_DISABLED: u32 = 0x0;

pub const FB_IRQEN_GPIO3INTEN: u32 = 1;
pub const FM_IRQEN_GPIO3INTEN: u32 = 0x2;
pub const FV_GPIO3INTEN_ENABLED: u32 = 0x2;
pub const FV_GPIO3INTEN_DISABLED: u32 = 0x0;

pub const FB_IRQEN_GPIO2INTEN: u32 = 0;
pub const FM_IRQEN_GPIO2INTEN: u32 = 0x1;
pub const FV_GPIO2INTEN_ENABLED: u32 = 0x1;
pub const FV_GPIO2INTEN_DISABLED: u32 = 0x0;

pub const IRQEN_GPIOINTEN_ENABLED: u32 = 0x1;
pub const IRQEN_GPIOINTEN_DISABLED: u32 = 0x0;

// *** IRQMASK ***
pub const FB_IRQMASK_THRMIM: u32 = 6;
pub const FM_IRQMASK_THRMIM: u32 = 0x40;
pub const FV_THRMIM_MASKED: u32 = 0x0;
pub const FV_THRMIM_NOT_MASKED: u32 = 0x40;

pub const FB_IRQMASK_HBPIM: u32 = 5;
pub const FM_IRQMASK_HBPIM: u32 = 0x20;
pub const FV_HBPIM_MASKED: u32 = 0x0;
pub const FV_HBPIM_NOT_MASKED: u32 = 0x20;

pub const FB_IRQMASK_HSDIM: u32 = 4;
pub const FM_IRQMASK_HSDIM: u32 = 0x10;
pub const FV_HSDIM_MASKED: u32 = 0x0;
pub const FV_HSDIM_NOT_MASKED: u32 = 0x10;

pub const FB_IRQMASK_HPDIM: u32 = 3;
pub const FM_IRQMASK_HPDIM: u32 = 0x8;
pub const FV_HPDIM_MASKED: u32 = 0x0;
pub const FV_HPDIM_NOT_MASKED: u32 = 0x8;

pub const FB_IRQMASK_GPIO3M: u32 = 1;
pub const FM_IRQMASK_GPIO3M: u32 = 0x2;
pub const FV_GPIO3M_MASKED: u32 = 0x0;
pub const FV_GPIO3M_NOT_MASKED: u32 = 0x2;

pub const FB_IRQMASK_GPIO2M: u32 = 0;
pub const FM_IRQMASK_GPIO2M: u32 = 0x1;
pub const FV_GPIO2M_MASKED: u32 = 0x0;
pub const FV_GPIO2M_NOT_MASKED: u32 = 0x1;

pub const IRQMASK_GPIOM_MASKED: u32 = 0x0;
pub const IRQMASK_GPIOM_NOT_MASKED: u32 = 0x1;

// *** IRQSTAT ***
pub const FB_IRQSTAT_THRMINT: u32 = 6;
pub const FM_IRQSTAT_THRMINT: u32 = 0x40;
pub const FV_THRMINT_INTERRUPTED: u32 = 0x40;
pub const FV_THRMINT_NOT_INTERRUPTED: u32 = 0x0;

pub const FB_IRQSTAT_HBPINT: u32 = 5;
pub const FM_IRQSTAT_HBPINT: u32 = 0x20;
pub const FV_HBPINT_INTERRUPTED: u32 = 0x20;
pub const FV_HBPINT_NOT_INTERRUPTED: u32 = 0x0;

pub const FB_IRQSTAT_HSDINT: u32 = 4;
pub const FM_IRQSTAT_HSDINT: u32 = 0x10;
pub const FV_HSDINT_INTERRUPTED: u32 = 0x10;
pub const FV_HSDINT_NOT_INTERRUPTED: u32 = 0x0;

pub const FB_IRQSTAT_HPDINT: u32 = 3;
pub const FM_IRQSTAT_HPDINT: u32 = 0x8;
pub const FV_HPDINT_INTERRUPTED: u32 = 0x8;
pub const FV_HPDINT_NOT_INTERRUPTED: u32 = 0x0;

pub const FB_IRQSTAT_GPIO3INT: u32 = 1;
pub const FM_IRQSTAT_GPIO3INT: u32 = 0x2;
pub const FV_GPIO3INT_INTERRUPTED: u32 = 0x2;
pub const FV_GPIO3INT_NOT_INTERRUPTED: u32 = 0x0;

pub const FB_IRQSTAT_GPIO2INT: u32 = 0;
pub const FM_IRQSTAT_GPIO2INT: u32 = 0x1;
pub const FV_GPIO2INT_INTERRUPTED: u32 = 0x1;
pub const FV_GPIO2INT_NOT_INTERRUPTED: u32 = 0x0;

pub const IRQSTAT_GPIOINT_INTERRUPTED: u32 = 0x1;
pub const IRQSTAT_GPIOINT_NOT_INTERRUPTED: u32 = 0x0;

// *** DEVADD0 ***
pub const FB_DEVADD0_DEVADD0: u32 = 1;
pub const FM_DEVADD0_DEVADD0: u32 = 0xFE;

pub const FB_DEVADD0_I2C_ADDRLK: u32 = 0;
pub const FM_DEVADD0_I2C_ADDRLK: u32 = 0x1;
pub const FV_I2C_ADDRLK_LOCK: u32 = 0x1;

// *** DEVID ***
pub const FB_DEVID_DEV_ID: u32 = 0;
pub const FM_DEVID_DEV_ID: u32 = 0xFF;

// *** DEVREV ***
pub const FB_DEVREV_MAJ_REV: u32 = 4;
pub const FM_DEVREV_MAJ_REV: u32 = 0xF0;

pub const FB_DEVREV_MIN_REV: u32 = 0;
pub const FM_DEVREV_MIN_REV: u32 = 0xF;

// *** PLLSTAT ***
pub const FB_PLLSTAT_PLL2LK: u32 = 1;
pub const FM_PLLSTAT_PLL2LK: u32 = 0x2;
pub const FV_PLL2LK_LOCKED: u32 = 0x2;
pub const FV_PLL2LK_UNLOCKED: u32 = 0x0;

pub const FB_PLLSTAT_PLL1LK: u32 = 0;
pub const FM_PLLSTAT_PLL1LK: u32 = 0x1;
pub const FV_PLL1LK_LOCKED: u32 = 0x1;
pub const FV_PLL1LK_UNLOCKED: u32 = 0x0;

pub const PLLSTAT_PLLLK_LOCKED: u32 = 0x1;
pub const PLLSTAT_PLLLK_UNLOCKED: u32 = 0x0;

// *** PLLCTL ***
pub const FB_PLLCTL_PU_PLL2: u32 = 7;
pub const FM_PLLCTL_PU_PLL2: u32 = 0x80;
pub const FV_PU_PLL2_PWR_UP: u32 = 0x80;
pub const FV_PU_PLL2_PWR_DWN: u32 = 0x0;

pub const FB_PLLCTL_PU_PLL1: u32 = 6;
pub const FM_PLLCTL_PU_PLL1: u32 = 0x40;
pub const FV_PU_PLL1_PWR_UP: u32 = 0x40;
pub const FV_PU_PLL1_PWR_DWN: u32 = 0x0;

pub const FB_PLLCTL_PLL2CLKEN: u32 = 5;
pub const FM_PLLCTL_PLL2CLKEN: u32 = 0x20;
pub const FV_PLL2CLKEN_ENABLE: u32 = 0x20;
pub const FV_PLL2CLKEN_DISABLE: u32 = 0x0;

pub const FB_PLLCTL_PLL1CLKEN: u32 = 4;
pub const FM_PLLCTL_PLL1CLKEN: u32 = 0x10;
pub const FV_PLL1CLKEN_ENABLE: u32 = 0x10;
pub const FV_PLL1CLKEN_DISABLE: u32 = 0x0;

pub const FB_PLLCTL_BCLKSEL: u32 = 2;
pub const FM_PLLCTL_BCLKSEL: u32 = 0xC;
pub const FV_BCLKSEL_BCLK1: u32 = 0x0;
pub const FV_BCLKSEL_BCLK2: u32 = 0x4;
pub const FV_BCLKSEL_BCLK3: u32 = 0x8;

pub const FB_PLLCTL_PLLISEL: u32 = 0;
pub const FM_PLLCTL_PLLISEL: u32 = 0x3;
pub const FV_PLLISEL_XTAL: u32 = 0x0;
pub const FV_PLLISEL_MCLK1: u32 = 0x1;
pub const FV_PLLISEL_MCLK2: u32 = 0x2;
pub const FV_PLLISEL_BCLK: u32 = 0x3;

pub const PLLCTL_PU_PLL_PWR_UP: u32 = 0x1;
pub const PLLCTL_PU_PLL_PWR_DWN: u32 = 0x0;
pub const PLLCTL_PLLCLKEN_ENABLE: u32 = 0x1;
pub const PLLCTL_PLLCLKEN_DISABLE: u32 = 0x0;

// *** ISRC ***
pub const FB_ISRC_IBR: u32 = 2;
pub const FM_ISRC_IBR: u32 = 0x4;
pub const FV_IBR_44PT1: u32 = 0x0;
pub const FV_IBR_48: u32 = 0x4;

pub const FB_ISRC_IBM: u32 = 0;
pub const FM_ISRC_IBM: u32 = 0x3;
pub const FV_IBM_0PT25: u32 = 0x0;
pub const FV_IBM_0PT5: u32 = 0x1;
pub const FV_IBM_1: u32 = 0x2;
pub const FV_IBM_2: u32 = 0x3;

// *** SCLKCTL ***
pub const FB_SCLKCTL_ASDM: u32 = 6;
pub const FM_SCLKCTL_ASDM: u32 = 0xC0;
pub const FV_ASDM_HALF: u32 = 0x40;
pub const FV_ASDM_FULL: u32 = 0x80;
pub const FV_ASDM_AUTO: u32 = 0xC0;

pub const FB_SCLKCTL_DSDM: u32 = 4;
pub const FM_SCLKCTL_DSDM: u32 = 0x30;
pub const FV_DSDM_HALF: u32 = 0x10;
pub const FV_DSDM_FULL: u32 = 0x20;
pub const FV_DSDM_AUTO: u32 = 0x30;

// *** TIMEBASE ***
pub const FB_TIMEBASE_TIMEBASE: u32 = 0;
pub const FM_TIMEBASE_TIMEBASE: u32 = 0xFF;

// *** I2SCMC ***
pub const FB_I2SCMC_BCMP3: u32 = 4;
pub const FM_I2SCMC_BCMP3: u32 = 0x30;
pub const FV_BCMP3_AUTO: u32 = 0x0;
pub const FV_BCMP3_32X: u32 = 0x10;
pub const FV_BCMP3_40X: u32 = 0x20;
pub const FV_BCMP3_64X: u32 = 0x30;

pub const FB_I2SCMC_BCMP2: u32 = 2;
pub const FM_I2SCMC_BCMP2: u32 = 0xC;
pub const FV_BCMP2_AUTO: u32 = 0x0;
pub const FV_BCMP2_32X: u32 = 0x4;
pub const FV_BCMP2_40X: u32 = 0x8;
pub const FV_BCMP2_64X: u32 = 0xC;

pub const FB_I2SCMC_BCMP1: u32 = 0;
pub const FM_I2SCMC_BCMP1: u32 = 0x3;
pub const FV_BCMP1_AUTO: u32 = 0x0;
pub const FV_BCMP1_32X: u32 = 0x1;
pub const FV_BCMP1_40X: u32 = 0x2;
pub const FV_BCMP1_64X: u32 = 0x3;

pub const I2SCMC_BCMP_AUTO: u32 = 0x0;
pub const I2SCMC_BCMP_32X: u32 = 0x1;
pub const I2SCMC_BCMP_40X: u32 = 0x2;
pub const I2SCMC_BCMP_64X: u32 = 0x3;

// *** MCLK2PINC ***
pub const FB_MCLK2PINC_SLEWOUT: u32 = 4;
pub const FM_MCLK2PINC_SLEWOUT: u32 = 0xF0;

pub const FB_MCLK2PINC_MCLK2IO: u32 = 2;
pub const FM_MCLK2PINC_MCLK2IO: u32 = 0x4;
pub const FV_MCLK2IO_INPUT: u32 = 0x0;
pub const FV_MCLK2IO_OUTPUT: u32 = 0x4;

pub const FB_MCLK2PINC_MCLK2OS: u32 = 0;
pub const FM_MCLK2PINC_MCLK2OS: u32 = 0x3;
pub const FV_MCLK2OS_24PT576: u32 = 0x0;
pub const FV_MCLK2OS_22PT5792: u32 = 0x1;
pub const FV_MCLK2OS_PLL2: u32 = 0x2;

// *** I2SPINC0 ***
pub const FB_I2SPINC0_SDO3TRI: u32 = 7;
pub const FM_I2SPINC0_SDO3TRI: u32 = 0x80;

pub const FB_I2SPINC0_SDO2TRI: u32 = 6;
pub const FM_I2SPINC0_SDO2TRI: u32 = 0x40;

pub const FB_I2SPINC0_SDO1TRI: u32 = 5;
pub const FM_I2SPINC0_SDO1TRI: u32 = 0x20;

pub const FB_I2SPINC0_PCM3TRI: u32 = 2;
pub const FM_I2SPINC0_PCM3TRI: u32 = 0x4;

pub const FB_I2SPINC0_PCM2TRI: u32 = 1;
pub const FM_I2SPINC0_PCM2TRI: u32 = 0x2;

pub const FB_I2SPINC0_PCM1TRI: u32 = 0;
pub const FM_I2SPINC0_PCM1TRI: u32 = 0x1;

// *** I2SPINC1 ***
pub const FB_I2SPINC1_SDO3PDD: u32 = 2;
pub const FM_I2SPINC1_SDO3PDD: u32 = 0x4;

pub const FB_I2SPINC1_SDO2PDD: u32 = 1;
pub const FM_I2SPINC1_SDO2PDD: u32 = 0x2;

pub const FB_I2SPINC1_SDO1PDD: u32 = 0;
pub const FM_I2SPINC1_SDO1PDD: u32 = 0x1;

// *** I2SPINC2 ***
pub const FB_I2SPINC2_LR3PDD: u32 = 5;
pub const FM_I2SPINC2_LR3PDD: u32 = 0x20;

pub const FB_I2SPINC2_BC3PDD: u32 = 4;
pub const FM_I2SPINC2_BC3PDD: u32 = 0x10;

pub const FB_I2SPINC2_LR2PDD: u32 = 3;
pub const FM_I2SPINC2_LR2PDD: u32 = 0x8;

pub const FB_I2SPINC2_BC2PDD: u32 = 2;
pub const FM_I2SPINC2_BC2PDD: u32 = 0x4;

pub const FB_I2SPINC2_LR1PDD: u32 = 1;
pub const FM_I2SPINC2_LR1PDD: u32 = 0x2;

pub const FB_I2SPINC2_BC1PDD: u32 = 0;
pub const FM_I2SPINC2_BC1PDD: u32 = 0x1;

// *** GPIOCTL0 ***
pub const FB_GPIOCTL0_GPIO3INTP: u32 = 7;
pub const FM_GPIOCTL0_GPIO3INTP: u32 = 0x80;

pub const FB_GPIOCTL0_GPIO2INTP: u32 = 6;
pub const FM_GPIOCTL0_GPIO2INTP: u32 = 0x40;

pub const FB_GPIOCTL0_GPIO3CFG: u32 = 5;
pub const FM_GPIOCTL0_GPIO3CFG: u32 = 0x20;

pub const FB_GPIOCTL0_GPIO2CFG: u32 = 4;
pub const FM_GPIOCTL0_GPIO2CFG: u32 = 0x10;

pub const FB_GPIOCTL0_GPIO3IO: u32 = 3;
pub const FM_GPIOCTL0_GPIO3IO: u32 = 0x8;

pub const FB_GPIOCTL0_GPIO2IO: u32 = 2;
pub const FM_GPIOCTL0_GPIO2IO: u32 = 0x4;

pub const FB_GPIOCTL0_GPIO1IO: u32 = 1;
pub const FM_GPIOCTL0_GPIO1IO: u32 = 0x2;

pub const FB_GPIOCTL0_GPIO0IO: u32 = 0;
pub const FM_GPIOCTL0_GPIO0IO: u32 = 0x1;

// *** GPIOCTL1 ***
pub const FB_GPIOCTL1_GPIO3: u32 = 7;
pub const FM_GPIOCTL1_GPIO3: u32 = 0x80;

pub const FB_GPIOCTL1_GPIO2: u32 = 6;
pub const FM_GPIOCTL1_GPIO2: u32 = 0x40;

pub const FB_GPIOCTL1_GPIO1: u32 = 5;
pub const FM_GPIOCTL1_GPIO1: u32 = 0x20;

pub const FB_GPIOCTL1_GPIO0: u32 = 4;
pub const FM_GPIOCTL1_GPIO0: u32 = 0x10;

pub const FB_GPIOCTL1_GPIO3RD: u32 = 3;
pub const FM_GPIOCTL1_GPIO3RD: u32 = 0x8;

pub const FB_GPIOCTL1_GPIO2RD: u32 = 2;
pub const FM_GPIOCTL1_GPIO2RD: u32 = 0x4;

pub const FB_GPIOCTL1_GPIO1RD: u32 = 1;
pub const FM_GPIOCTL1_GPIO1RD: u32 = 0x2;

pub const FB_GPIOCTL1_GPIO0RD: u32 = 0;
pub const FM_GPIOCTL1_GPIO0RD: u32 = 0x1;

// *** ASRC ***
pub const FB_ASRC_ASRCOBW: u32 = 7;
pub const FM_ASRC_ASRCOBW: u32 = 0x80;

pub const FB_ASRC_ASRCIBW: u32 = 6;
pub const FM_ASRC_ASRCIBW: u32 = 0x40;

pub const FB_ASRC_ASRCOB: u32 = 5;
pub const FM_ASRC_ASRCOB: u32 = 0x20;
pub const FV_ASRCOB_ACTIVE: u32 = 0x0;
pub const FV_ASRCOB_BYPASSED: u32 = 0x20;

pub const FB_ASRC_ASRCIB: u32 = 4;
pub const FM_ASRC_ASRCIB: u32 = 0x10;
pub const FV_ASRCIB_ACTIVE: u32 = 0x0;
pub const FV_ASRCIB_BYPASSED: u32 = 0x10;

pub const FB_ASRC_ASRCOL: u32 = 3;
pub const FM_ASRC_ASRCOL: u32 = 0x8;

pub const FB_ASRC_ASRCIL: u32 = 2;
pub const FM_ASRC_ASRCIL: u32 = 0x4;

// *** TDMCTL0 ***
pub const FB_TDMCTL0_TDMMD: u32 = 2;
pub const FM_TDMCTL0_TDMMD: u32 = 0x4;
pub const FV_TDMMD_200: u32 = 0x0;
pub const FV_TDMMD_256: u32 = 0x4;

pub const FB_TDMCTL0_SLSYNC: u32 = 1;
pub const FM_TDMCTL0_SLSYNC: u32 = 0x2;
pub const FV_SLSYNC_SHORT: u32 = 0x0;
pub const FV_SLSYNC_LONG: u32 = 0x2;

pub const FB_TDMCTL0_BDELAY: u32 = 0;
pub const FM_TDMCTL0_BDELAY: u32 = 0x1;
pub const FV_BDELAY_NO_DELAY: u32 = 0x0;
pub const FV_BDELAY_1BCLK_DELAY: u32 = 0x1;

// *** TDMCTL1 ***
pub const FB_TDMCTL1_TDMSO: u32 = 5;
pub const FM_TDMCTL1_TDMSO: u32 = 0x60;
pub const FV_TDMSO_2: u32 = 0x0;
pub const FV_TDMSO_4: u32 = 0x20;
pub const FV_TDMSO_6: u32 = 0x40;

pub const FB_TDMCTL1_TDMDSS: u32 = 3;
pub const FM_TDMCTL1_TDMDSS: u32 = 0x18;
pub const FV_TDMDSS_16: u32 = 0x0;
pub const FV_TDMDSS_24: u32 = 0x10;
pub const FV_TDMDSS_32: u32 = 0x18;

pub const FB_TDMCTL1_TDMSI: u32 = 0;
pub const FM_TDMCTL1_TDMSI: u32 = 0x3;
pub const FV_TDMSI_2: u32 = 0x0;
pub const FV_TDMSI_4: u32 = 0x1;
pub const FV_TDMSI_6: u32 = 0x2;

// *** PWRM0 ***
pub const FB_PWRM0_INPROC3PU: u32 = 6;
pub const FM_PWRM0_INPROC3PU: u32 = 0x40;

pub const FB_PWRM0_INPROC2PU: u32 = 5;
pub const FM_PWRM0_INPROC2PU: u32 = 0x20;

pub const FB_PWRM0_INPROC1PU: u32 = 4;
pub const FM_PWRM0_INPROC1PU: u32 = 0x10;

pub const FB_PWRM0_INPROC0PU: u32 = 3;
pub const FM_PWRM0_INPROC0PU: u32 = 0x8;

pub const FB_PWRM0_MICB2PU: u32 = 2;
pub const FM_PWRM0_MICB2PU: u32 = 0x4;

pub const FB_PWRM0_MICB1PU: u32 = 1;
pub const FM_PWRM0_MICB1PU: u32 = 0x2;

pub const FB_PWRM0_MCLKPEN: u32 = 0;
pub const FM_PWRM0_MCLKPEN: u32 = 0x1;

// *** PWRM1 ***
pub const FB_PWRM1_SUBPU: u32 = 7;
pub const FM_PWRM1_SUBPU: u32 = 0x80;

pub const FB_PWRM1_HPLPU: u32 = 6;
pub const FM_PWRM1_HPLPU: u32 = 0x40;

pub const FB_PWRM1_HPRPU: u32 = 5;
pub const FM_PWRM1_HPRPU: u32 = 0x20;

pub const FB_PWRM1_SPKLPU: u32 = 4;
pub const FM_PWRM1_SPKLPU: u32 = 0x10;

pub const FB_PWRM1_SPKRPU: u32 = 3;
pub const FM_PWRM1_SPKRPU: u32 = 0x8;

pub const FB_PWRM1_D2S2PU: u32 = 2;
pub const FM_PWRM1_D2S2PU: u32 = 0x4;

pub const FB_PWRM1_D2S1PU: u32 = 1;
pub const FM_PWRM1_D2S1PU: u32 = 0x2;

pub const FB_PWRM1_VREFPU: u32 = 0;
pub const FM_PWRM1_VREFPU: u32 = 0x1;

// *** PWRM2 ***
pub const FB_PWRM2_I2S3OPU: u32 = 5;
pub const FM_PWRM2_I2S3OPU: u32 = 0x20;
pub const FV_I2S3OPU_PWR_DOWN: u32 = 0x0;
pub const FV_I2S3OPU_PWR_UP: u32 = 0x20;

pub const FB_PWRM2_I2S2OPU: u32 = 4;
pub const FM_PWRM2_I2S2OPU: u32 = 0x10;
pub const FV_I2S2OPU_PWR_DOWN: u32 = 0x0;
pub const FV_I2S2OPU_PWR_UP: u32 = 0x10;

pub const FB_PWRM2_I2S1OPU: u32 = 3;
pub const FM_PWRM2_I2S1OPU: u32 = 0x8;
pub const FV_I2S1OPU_PWR_DOWN: u32 = 0x0;
pub const FV_I2S1OPU_PWR_UP: u32 = 0x8;

pub const FB_PWRM2_I2S3IPU: u32 = 2;
pub const FM_PWRM2_I2S3IPU: u32 = 0x4;
pub const FV_I2S3IPU_PWR_DOWN: u32 = 0x0;
pub const FV_I2S3IPU_PWR_UP: u32 = 0x4;

pub const FB_PWRM2_I2S2IPU: u32 = 1;
pub const FM_PWRM2_I2S2IPU: u32 = 0x2;
pub const FV_I2S2IPU_PWR_DOWN: u32 = 0x0;
pub const FV_I2S2IPU_PWR_UP: u32 = 0x2;

pub const FB_PWRM2_I2S1IPU: u32 = 0;
pub const FM_PWRM2_I2S1IPU: u32 = 0x1;
pub const FV_I2S1IPU_PWR_DOWN: u32 = 0x0;
pub const FV_I2S1IPU_PWR_UP: u32 = 0x1;

pub const PWRM2_I2SOPU_PWR_DOWN: u32 = 0x0;
pub const PWRM2_I2SOPU_PWR_UP: u32 = 0x1;
pub const PWRM2_I2SIPU_PWR_DOWN: u32 = 0x0;
pub const PWRM2_I2SIPU_PWR_UP: u32 = 0x1;

// *** PWRM3 ***
pub const FB_PWRM3_BGSBUP: u32 = 6;
pub const FM_PWRM3_BGSBUP: u32 = 0x40;
pub const FV_BGSBUP_ON: u32 = 0x0;
pub const FV_BGSBUP_OFF: u32 = 0x40;

pub const FB_PWRM3_VGBAPU: u32 = 5;
pub const FM_PWRM3_VGBAPU: u32 = 0x20;
pub const FV_VGBAPU_ON: u32 = 0x0;
pub const FV_VGBAPU_OFF: u32 = 0x20;

pub const FB_PWRM3_LLINEPU: u32 = 4;
pub const FM_PWRM3_LLINEPU: u32 = 0x10;

pub const FB_PWRM3_RLINEPU: u32 = 3;
pub const FM_PWRM3_RLINEPU: u32 = 0x8;

// *** PWRM4 ***
pub const FB_PWRM4_OPSUBPU: u32 = 4;
pub const FM_PWRM4_OPSUBPU: u32 = 0x10;

pub const FB_PWRM4_OPDACLPU: u32 = 3;
pub const FM_PWRM4_OPDACLPU: u32 = 0x8;

pub const FB_PWRM4_OPDACRPU: u32 = 2;
pub const FM_PWRM4_OPDACRPU: u32 = 0x4;

pub const FB_PWRM4_OPSPKLPU: u32 = 1;
pub const FM_PWRM4_OPSPKLPU: u32 = 0x2;

pub const FB_PWRM4_OPSPKRPU: u32 = 0;
pub const FM_PWRM4_OPSPKRPU: u32 = 0x1;

// *** I2SIDCTL ***
pub const FB_I2SIDCTL_I2SI3DCTL: u32 = 4;
pub const FM_I2SIDCTL_I2SI3DCTL: u32 = 0x30;

pub const FB_I2SIDCTL_I2SI2DCTL: u32 = 2;
pub const FM_I2SIDCTL_I2SI2DCTL: u32 = 0xC;

pub const FB_I2SIDCTL_I2SI1DCTL: u32 = 0;
pub const FM_I2SIDCTL_I2SI1DCTL: u32 = 0x3;

// *** I2SODCTL ***
pub const FB_I2SODCTL_I2SO3DCTL: u32 = 4;
pub const FM_I2SODCTL_I2SO3DCTL: u32 = 0x30;

pub const FB_I2SODCTL_I2SO2DCTL: u32 = 2;
pub const FM_I2SODCTL_I2SO2DCTL: u32 = 0xC;

pub const FB_I2SODCTL_I2SO1DCTL: u32 = 0;
pub const FM_I2SODCTL_I2SO1DCTL: u32 = 0x3;

// *** AUDIOMUX1 ***
pub const FB_AUDIOMUX1_ASRCIMUX: u32 = 6;
pub const FM_AUDIOMUX1_ASRCIMUX: u32 = 0xC0;
pub const FV_ASRCIMUX_NONE: u32 = 0x0;
pub const FV_ASRCIMUX_I2S1: u32 = 0x40;
pub const FV_ASRCIMUX_I2S2: u32 = 0x80;
pub const FV_ASRCIMUX_I2S3: u32 = 0xC0;

pub const FB_AUDIOMUX1_I2S2MUX: u32 = 3;
pub const FM_AUDIOMUX1_I2S2MUX: u32 = 0x38;
pub const FV_I2S2MUX_I2S1: u32 = 0x0;
pub const FV_I2S2MUX_I2S2: u32 = 0x8;
pub const FV_I2S2MUX_I2S3: u32 = 0x10;
pub const FV_I2S2MUX_ADC_DMIC: u32 = 0x18;
pub const FV_I2S2MUX_DMIC2: u32 = 0x20;
pub const FV_I2S2MUX_CLASSD_DSP: u32 = 0x28;
pub const FV_I2S2MUX_DAC_DSP: u32 = 0x30;
pub const FV_I2S2MUX_SUB_DSP: u32 = 0x38;

pub const FB_AUDIOMUX1_I2S1MUX: u32 = 0;
pub const FM_AUDIOMUX1_I2S1MUX: u32 = 0x7;
pub const FV_I2S1MUX_I2S1: u32 = 0x0;
pub const FV_I2S1MUX_I2S2: u32 = 0x1;
pub const FV_I2S1MUX_I2S3: u32 = 0x2;
pub const FV_I2S1MUX_ADC_DMIC: u32 = 0x3;
pub const FV_I2S1MUX_DMIC2: u32 = 0x4;
pub const FV_I2S1MUX_CLASSD_DSP: u32 = 0x5;
pub const FV_I2S1MUX_DAC_DSP: u32 = 0x6;
pub const FV_I2S1MUX_SUB_DSP: u32 = 0x7;

pub const AUDIOMUX1_I2SMUX_I2S1: u32 = 0x0;
pub const AUDIOMUX1_I2SMUX_I2S2: u32 = 0x1;
pub const AUDIOMUX1_I2SMUX_I2S3: u32 = 0x2;
pub const AUDIOMUX1_I2SMUX_ADC_DMIC: u32 = 0x3;
pub const AUDIOMUX1_I2SMUX_DMIC2: u32 = 0x4;
pub const AUDIOMUX1_I2SMUX_CLASSD_DSP: u32 = 0x5;
pub const AUDIOMUX1_I2SMUX_DAC_DSP: u32 = 0x6;
pub const AUDIOMUX1_I2SMUX_SUB_DSP: u32 = 0x7;

// *** AUDIOMUX2 ***
pub const FB_AUDIOMUX2_ASRCOMUX: u32 = 6;
pub const FM_AUDIOMUX2_ASRCOMUX: u32 = 0xC0;
pub const FV_ASRCOMUX_NONE: u32 = 0x0;
pub const FV_ASRCOMUX_I2S1: u32 = 0x40;
pub const FV_ASRCOMUX_I2S2: u32 = 0x80;
pub const FV_ASRCOMUX_I2S3: u32 = 0xC0;

pub const FB_AUDIOMUX2_DACMUX: u32 = 3;
pub const FM_AUDIOMUX2_DACMUX: u32 = 0x38;
pub const FV_DACMUX_I2S1: u32 = 0x0;
pub const FV_DACMUX_I2S2: u32 = 0x8;
pub const FV_DACMUX_I2S3: u32 = 0x10;
pub const FV_DACMUX_ADC_DMIC: u32 = 0x18;
pub const FV_DACMUX_DMIC2: u32 = 0x20;
pub const FV_DACMUX_CLASSD_DSP: u32 = 0x28;
pub const FV_DACMUX_DAC_DSP: u32 = 0x30;
pub const FV_DACMUX_SUB_DSP: u32 = 0x38;

pub const FB_AUDIOMUX2_I2S3MUX: u32 = 0;
pub const FM_AUDIOMUX2_I2S3MUX: u32 = 0x7;
pub const FV_I2S3MUX_I2S1: u32 = 0x0;
pub const FV_I2S3MUX_I2S2: u32 = 0x1;
pub const FV_I2S3MUX_I2S3: u32 = 0x2;
pub const FV_I2S3MUX_ADC_DMIC: u32 = 0x3;
pub const FV_I2S3MUX_DMIC2: u32 = 0x4;
pub const FV_I2S3MUX_CLASSD_DSP: u32 = 0x5;
pub const FV_I2S3MUX_DAC_DSP: u32 = 0x6;
pub const FV_I2S3MUX_SUB_DSP: u32 = 0x7;

// *** AUDIOMUX3 ***
pub const FB_AUDIOMUX3_SUBMUX: u32 = 3;
pub const FM_AUDIOMUX3_SUBMUX: u32 = 0xF8;
pub const FV_SUBMUX_I2S1_L: u32 = 0x0;
pub const FV_SUBMUX_I2S1_R: u32 = 0x8;
pub const FV_SUBMUX_I2S1_LR: u32 = 0x10;
pub const FV_SUBMUX_I2S2_L: u32 = 0x18;
pub const FV_SUBMUX_I2S2_R: u32 = 0x20;
pub const FV_SUBMUX_I2S2_LR: u32 = 0x28;
pub const FV_SUBMUX_I2S3_L: u32 = 0x30;
pub const FV_SUBMUX_I2S3_R: u32 = 0x38;
pub const FV_SUBMUX_I2S3_LR: u32 = 0x40;
pub const FV_SUBMUX_ADC_DMIC_L: u32 = 0x48;
pub const FV_SUBMUX_ADC_DMIC_R: u32 = 0x50;
pub const FV_SUBMUX_ADC_DMIC_LR: u32 = 0x58;
pub const FV_SUBMUX_DMIC_L: u32 = 0x60;
pub const FV_SUBMUX_DMIC_R: u32 = 0x68;
pub const FV_SUBMUX_DMIC_LR: u32 = 0x70;
pub const FV_SUBMUX_CLASSD_DSP_L: u32 = 0x78;
pub const FV_SUBMUX_CLASSD_DSP_R: u32 = 0x80;
pub const FV_SUBMUX_CLASSD_DSP_LR: u32 = 0x88;

pub const FB_AUDIOMUX3_CLSSDMUX: u32 = 0;
pub const FM_AUDIOMUX3_CLSSDMUX: u32 = 0x7;
pub const FV_CLSSDMUX_I2S1: u32 = 0x0;
pub const FV_CLSSDMUX_I2S2: u32 = 0x1;
pub const FV_CLSSDMUX_I2S3: u32 = 0x2;
pub const FV_CLSSDMUX_ADC_DMIC: u32 = 0x3;
pub const FV_CLSSDMUX_DMIC2: u32 = 0x4;
pub const FV_CLSSDMUX_CLASSD_DSP: u32 = 0x5;
pub const FV_CLSSDMUX_DAC_DSP: u32 = 0x6;
pub const FV_CLSSDMUX_SUB_DSP: u32 = 0x7;

// *** HSDCTL1 ***
pub const FB_HSDCTL1_HPJKTYPE: u32 = 7;
pub const FM_HSDCTL1_HPJKTYPE: u32 = 0x80;

pub const FB_HSDCTL1_CON_DET_PWD: u32 = 6;
pub const FM_HSDCTL1_CON_DET_PWD: u32 = 0x40;

pub const FB_HSDCTL1_DETCYC: u32 = 4;
pub const FM_HSDCTL1_DETCYC: u32 = 0x30;

pub const FB_HSDCTL1_HPDLYBYP: u32 = 3;
pub const FM_HSDCTL1_HPDLYBYP: u32 = 0x8;

pub const FB_HSDCTL1_HSDETPOL: u32 = 2;
pub const FM_HSDCTL1_HSDETPOL: u32 = 0x4;

pub const FB_HSDCTL1_HPID_EN: u32 = 1;
pub const FM_HSDCTL1_HPID_EN: u32 = 0x2;

pub const FB_HSDCTL1_GBLHS_EN: u32 = 0;
pub const FM_HSDCTL1_GBLHS_EN: u32 = 0x1;

// *** HSDCTL2 ***
pub const FB_HSDCTL2_FMICBIAS1: u32 = 6;
pub const FM_HSDCTL2_FMICBIAS1: u32 = 0xC0;

pub const FB_HSDCTL2_MB1MODE: u32 = 5;
pub const FM_HSDCTL2_MB1MODE: u32 = 0x20;
pub const FV_MB1MODE_AUTO: u32 = 0x0;
pub const FV_MB1MODE_MANUAL: u32 = 0x20;

pub const FB_HSDCTL2_FORCETRG: u32 = 4;
pub const FM_HSDCTL2_FORCETRG: u32 = 0x10;

pub const FB_HSDCTL2_SWMODE: u32 = 3;
pub const FM_HSDCTL2_SWMODE: u32 = 0x8;

pub const FB_HSDCTL2_GHSHIZ: u32 = 2;
pub const FM_HSDCTL2_GHSHIZ: u32 = 0x4;

pub const FB_HSDCTL2_FPLUGTYPE: u32 = 0;
pub const FM_HSDCTL2_FPLUGTYPE: u32 = 0x3;

// *** HSDSTAT ***
pub const FB_HSDSTAT_MBIAS1DRV: u32 = 5;
pub const FM_HSDSTAT_MBIAS1DRV: u32 = 0x60;

pub const FB_HSDSTAT_HSDETSTAT: u32 = 3;
pub const FM_HSDSTAT_HSDETSTAT: u32 = 0x8;

pub const FB_HSDSTAT_PLUGTYPE: u32 = 1;
pub const FM_HSDSTAT_PLUGTYPE: u32 = 0x6;

pub const FB_HSDSTAT_HSDETDONE: u32 = 0;
pub const FM_HSDSTAT_HSDETDONE: u32 = 0x1;

// *** HSDDELAY ***
pub const FB_HSDDELAY_T_STABLE: u32 = 0;
pub const FM_HSDDELAY_T_STABLE: u32 = 0x7;

// *** BUTCTL ***
pub const FB_BUTCTL_BPUSHSTAT: u32 = 7;
pub const FM_BUTCTL_BPUSHSTAT: u32 = 0x80;

pub const FB_BUTCTL_BPUSHDET: u32 = 6;
pub const FM_BUTCTL_BPUSHDET: u32 = 0x40;

pub const FB_BUTCTL_BPUSHEN: u32 = 5;
pub const FM_BUTCTL_BPUSHEN: u32 = 0x20;

pub const FB_BUTCTL_BSTABLE_L: u32 = 3;
pub const FM_BUTCTL_BSTABLE_L: u32 = 0x18;

pub const FB_BUTCTL_BSTABLE_S: u32 = 0;
pub const FM_BUTCTL_BSTABLE_S: u32 = 0x7;

// *** CH0AIC ***
pub const FB_CH0AIC_INSELL: u32 = 6;
pub const FM_CH0AIC_INSELL: u32 = 0xC0;

pub const FB_CH0AIC_MICBST0: u32 = 4;
pub const FM_CH0AIC_MICBST0: u32 = 0x30;

pub const FB_CH0AIC_LADCIN: u32 = 2;
pub const FM_CH0AIC_LADCIN: u32 = 0xC;

pub const FB_CH0AIC_IN_BYPS_L_SEL: u32 = 1;
pub const FM_CH0AIC_IN_BYPS_L_SEL: u32 = 0x2;

pub const FB_CH0AIC_IPCH0S: u32 = 0;
pub const FM_CH0AIC_IPCH0S: u32 = 0x1;

// *** CH1AIC ***
pub const FB_CH1AIC_INSELR: u32 = 6;
pub const FM_CH1AIC_INSELR: u32 = 0xC0;

pub const FB_CH1AIC_MICBST1: u32 = 4;
pub const FM_CH1AIC_MICBST1: u32 = 0x30;

pub const FB_CH1AIC_RADCIN: u32 = 2;
pub const FM_CH1AIC_RADCIN: u32 = 0xC;

pub const FB_CH1AIC_IN_BYPS_R_SEL: u32 = 1;
pub const FM_CH1AIC_IN_BYPS_R_SEL: u32 = 0x2;

pub const FB_CH1AIC_IPCH1S: u32 = 0;
pub const FM_CH1AIC_IPCH1S: u32 = 0x1;

// *** ICTL0 ***
pub const FB_ICTL0_IN1POL: u32 = 7;
pub const FM_ICTL0_IN1POL: u32 = 0x80;

pub const FB_ICTL0_IN0POL: u32 = 6;
pub const FM_ICTL0_IN0POL: u32 = 0x40;

pub const FB_ICTL0_INPCH10SEL: u32 = 4;
pub const FM_ICTL0_INPCH10SEL: u32 = 0x30;

pub const FB_ICTL0_IN1MUTE: u32 = 3;
pub const FM_ICTL0_IN1MUTE: u32 = 0x8;

pub const FB_ICTL0_IN0MUTE: u32 = 2;
pub const FM_ICTL0_IN0MUTE: u32 = 0x4;

pub const FB_ICTL0_IN1HP: u32 = 1;
pub const FM_ICTL0_IN1HP: u32 = 0x2;

pub const FB_ICTL0_IN0HP: u32 = 0;
pub const FM_ICTL0_IN0HP: u32 = 0x1;

// *** ICTL1 ***
pub const FB_ICTL1_IN3POL: u32 = 7;
pub const FM_ICTL1_IN3POL: u32 = 0x80;

pub const FB_ICTL1_IN2POL: u32 = 6;
pub const FM_ICTL1_IN2POL: u32 = 0x40;

pub const FB_ICTL1_INPCH32SEL: u32 = 4;
pub const FM_ICTL1_INPCH32SEL: u32 = 0x30;

pub const FB_ICTL1_IN3MUTE: u32 = 3;
pub const FM_ICTL1_IN3MUTE: u32 = 0x8;

pub const FB_ICTL1_IN2MUTE: u32 = 2;
pub const FM_ICTL1_IN2MUTE: u32 = 0x4;

pub const FB_ICTL1_IN3HP: u32 = 1;
pub const FM_ICTL1_IN3HP: u32 = 0x2;

pub const FB_ICTL1_IN2HP: u32 = 0;
pub const FM_ICTL1_IN2HP: u32 = 0x1;

// *** MICBIAS ***
pub const FB_MICBIAS_MICBOV2: u32 = 4;
pub const FM_MICBIAS_MICBOV2: u32 = 0x30;

pub const FB_MICBIAS_MICBOV1: u32 = 6;
pub const FM_MICBIAS_MICBOV1: u32 = 0xC0;

pub const FB_MICBIAS_SPARE1: u32 = 2;
pub const FM_MICBIAS_SPARE1: u32 = 0xC;

pub const FB_MICBIAS_SPARE2: u32 = 0;
pub const FM_MICBIAS_SPARE2: u32 = 0x3;

// *** PGAZ ***
pub const FB_PGAZ_INHPOR: u32 = 1;
pub const FM_PGAZ_INHPOR: u32 = 0x2;

pub const FB_PGAZ_TOEN: u32 = 0;
pub const FM_PGAZ_TOEN: u32 = 0x1;

// *** ASRCILVOL ***
pub const FB_ASRCILVOL_ASRCILVOL: u32 = 0;
pub const FM_ASRCILVOL_ASRCILVOL: u32 = 0xFF;

// *** ASRCIRVOL ***
pub const FB_ASRCIRVOL_ASRCIRVOL: u32 = 0;
pub const FM_ASRCIRVOL_ASRCIRVOL: u32 = 0xFF;

// *** ASRCOLVOL ***
pub const FB_ASRCOLVOL_ASRCOLVOL: u32 = 0;
pub const FM_ASRCOLVOL_ASRCOLVOL: u32 = 0xFF;

// *** ASRCORVOL ***
pub const FB_ASRCORVOL_ASRCOLVOL: u32 = 0;
pub const FM_ASRCORVOL_ASRCOLVOL: u32 = 0xFF;

// *** IVOLCTLU ***
pub const FB_IVOLCTLU_IFADE: u32 = 3;
pub const FM_IVOLCTLU_IFADE: u32 = 0x8;

pub const FB_IVOLCTLU_INPVOLU: u32 = 2;
pub const FM_IVOLCTLU_INPVOLU: u32 = 0x4;

pub const FB_IVOLCTLU_PGAVOLU: u32 = 1;
pub const FM_IVOLCTLU_PGAVOLU: u32 = 0x2;

pub const FB_IVOLCTLU_ASRCVOLU: u32 = 0;
pub const FM_IVOLCTLU_ASRCVOLU: u32 = 0x1;

// *** ALCCTL0 ***
pub const FB_ALCCTL0_ALCMODE: u32 = 7;
pub const FM_ALCCTL0_ALCMODE: u32 = 0x80;

pub const FB_ALCCTL0_ALCREF: u32 = 4;
pub const FM_ALCCTL0_ALCREF: u32 = 0x70;

pub const FB_ALCCTL0_ALCEN3: u32 = 3;
pub const FM_ALCCTL0_ALCEN3: u32 = 0x8;

pub const FB_ALCCTL0_ALCEN2: u32 = 2;
pub const FM_ALCCTL0_ALCEN2: u32 = 0x4;

pub const FB_ALCCTL0_ALCEN1: u32 = 1;
pub const FM_ALCCTL0_ALCEN1: u32 = 0x2;

pub const FB_ALCCTL0_ALCEN0: u32 = 0;
pub const FM_ALCCTL0_ALCEN0: u32 = 0x1;

// *** ALCCTL1 ***
pub const FB_ALCCTL1_MAXGAIN: u32 = 4;
pub const FM_ALCCTL1_MAXGAIN: u32 = 0x70;

pub const FB_ALCCTL1_ALCL: u32 = 0;
pub const FM_ALCCTL1_ALCL: u32 = 0xF;

// *** ALCCTL2 ***
pub const FB_ALCCTL2_ALCZC: u32 = 7;
pub const FM_ALCCTL2_ALCZC: u32 = 0x80;

pub const FB_ALCCTL2_MINGAIN: u32 = 4;
pub const FM_ALCCTL2_MINGAIN: u32 = 0x70;

pub const FB_ALCCTL2_HLD: u32 = 0;
pub const FM_ALCCTL2_HLD: u32 = 0xF;

// *** ALCCTL3 ***
pub const FB_ALCCTL3_DCY: u32 = 4;
pub const FM_ALCCTL3_DCY: u32 = 0xF0;

pub const FB_ALCCTL3_ATK: u32 = 0;
pub const FM_ALCCTL3_ATK: u32 = 0xF;

// *** NGATE ***
pub const FB_NGATE_NGTH: u32 = 3;
pub const FM_NGATE_NGTH: u32 = 0xF8;

pub const FB_NGATE_NGG: u32 = 1;
pub const FM_NGATE_NGG: u32 = 0x6;

pub const FB_NGATE_NGAT: u32 = 0;
pub const FM_NGATE_NGAT: u32 = 0x1;

// *** DMICCTL ***
pub const FB_DMICCTL_DMIC2EN: u32 = 7;
pub const FM_DMICCTL_DMIC2EN: u32 = 0x80;

pub const FB_DMICCTL_DMIC1EN: u32 = 6;
pub const FM_DMICCTL_DMIC1EN: u32 = 0x40;

pub const FB_DMICCTL_DMONO: u32 = 4;
pub const FM_DMICCTL_DMONO: u32 = 0x10;

pub const FB_DMICCTL_DMDCLK: u32 = 2;
pub const FM_DMICCTL_DMDCLK: u32 = 0xC;

pub const FB_DMICCTL_DMRATE: u32 = 0;
pub const FM_DMICCTL_DMRATE: u32 = 0x3;

// *** DACCTL ***
pub const FB_DACCTL_DACPOLR: u32 = 7;
pub const FM_DACCTL_DACPOLR: u32 = 0x80;
pub const FV_DACPOLR_NORMAL: u32 = 0x0;
pub const FV_DACPOLR_INVERTED: u32 = 0x80;

pub const FB_DACCTL_DACPOLL: u32 = 6;
pub const FM_DACCTL_DACPOLL: u32 = 0x40;
pub const FV_DACPOLL_NORMAL: u32 = 0x0;
pub const FV_DACPOLL_INVERTED: u32 = 0x40;

pub const FB_DACCTL_DACDITH: u32 = 4;
pub const FM_DACCTL_DACDITH: u32 = 0x30;
pub const FV_DACDITH_DYNAMIC_HALF: u32 = 0x0;
pub const FV_DACDITH_DYNAMIC_FULL: u32 = 0x10;
pub const FV_DACDITH_DISABLED: u32 = 0x20;
pub const FV_DACDITH_STATIC: u32 = 0x30;

pub const FB_DACCTL_DACMUTE: u32 = 3;
pub const FM_DACCTL_DACMUTE: u32 = 0x8;
pub const FV_DACMUTE_ENABLE: u32 = 0x8;
pub const FV_DACMUTE_DISABLE: u32 = 0x0;

pub const FB_DACCTL_DACDEM: u32 = 2;
pub const FM_DACCTL_DACDEM: u32 = 0x4;
pub const FV_DACDEM_ENABLE: u32 = 0x4;
pub const FV_DACDEM_DISABLE: u32 = 0x0;

pub const FB_DACCTL_ABYPASS: u32 = 0;
pub const FM_DACCTL_ABYPASS: u32 = 0x1;

// *** SPKCTL ***
pub const FB_SPKCTL_SPKPOLR: u32 = 7;
pub const FM_SPKCTL_SPKPOLR: u32 = 0x80;
pub const FV_SPKPOLR_NORMAL: u32 = 0x0;
pub const FV_SPKPOLR_INVERTED: u32 = 0x80;

pub const FB_SPKCTL_SPKPOLL: u32 = 6;
pub const FM_SPKCTL_SPKPOLL: u32 = 0x40;
pub const FV_SPKPOLL_NORMAL: u32 = 0x0;
pub const FV_SPKPOLL_INVERTED: u32 = 0x40;

pub const FB_SPKCTL_SPKMUTE: u32 = 3;
pub const FM_SPKCTL_SPKMUTE: u32 = 0x8;
pub const FV_SPKMUTE_ENABLE: u32 = 0x8;
pub const FV_SPKMUTE_DISABLE: u32 = 0x0;

pub const FB_SPKCTL_SPKDEM: u32 = 2;
pub const FM_SPKCTL_SPKDEM: u32 = 0x4;
pub const FV_SPKDEM_ENABLE: u32 = 0x4;
pub const FV_SPKDEM_DISABLE: u32 = 0x0;

// *** SUBCTL ***
pub const FB_SUBCTL_SUBPOL: u32 = 7;
pub const FM_SUBCTL_SUBPOL: u32 = 0x80;

pub const FB_SUBCTL_SUBMUTE: u32 = 3;
pub const FM_SUBCTL_SUBMUTE: u32 = 0x8;

pub const FB_SUBCTL_SUBDEM: u32 = 2;
pub const FM_SUBCTL_SUBDEM: u32 = 0x4;

pub const FB_SUBCTL_SUBMUX: u32 = 1;
pub const FM_SUBCTL_SUBMUX: u32 = 0x2;

pub const FB_SUBCTL_SUBILMDIS: u32 = 0;
pub const FM_SUBCTL_SUBILMDIS: u32 = 0x1;

// *** DCCTL ***
pub const FB_DCCTL_SUBDCBYP: u32 = 7;
pub const FM_DCCTL_SUBDCBYP: u32 = 0x80;

pub const FB_DCCTL_DACDCBYP: u32 = 6;
pub const FM_DCCTL_DACDCBYP: u32 = 0x40;

pub const FB_DCCTL_SPKDCBYP: u32 = 5;
pub const FM_DCCTL_SPKDCBYP: u32 = 0x20;

pub const FB_DCCTL_DCCOEFSEL: u32 = 0;
pub const FM_DCCTL_DCCOEFSEL: u32 = 0x7;

// *** OVOLCTLU ***
pub const FB_OVOLCTLU_OFADE: u32 = 4;
pub const FM_OVOLCTLU_OFADE: u32 = 0x10;

pub const FB_OVOLCTLU_SUBVOLU: u32 = 3;
pub const FM_OVOLCTLU_SUBVOLU: u32 = 0x8;

pub const FB_OVOLCTLU_MVOLU: u32 = 2;
pub const FM_OVOLCTLU_MVOLU: u32 = 0x4;

pub const FB_OVOLCTLU_SPKVOLU: u32 = 1;
pub const FM_OVOLCTLU_SPKVOLU: u32 = 0x2;

pub const FB_OVOLCTLU_HPVOLU: u32 = 0;
pub const FM_OVOLCTLU_HPVOLU: u32 = 0x1;

// *** MUTEC ***
pub const FB_MUTEC_ZDSTAT: u32 = 7;
pub const FM_MUTEC_ZDSTAT: u32 = 0x80;

pub const FB_MUTEC_ZDLEN: u32 = 4;
pub const FM_MUTEC_ZDLEN: u32 = 0x30;

pub const FB_MUTEC_APWD: u32 = 3;
pub const FM_MUTEC_APWD: u32 = 0x8;

pub const FB_MUTEC_AMUTE: u32 = 2;
pub const FM_MUTEC_AMUTE: u32 = 0x4;

// *** MVOLL ***
pub const FB_MVOLL_MVOL_L: u32 = 0;
pub const FM_MVOLL_MVOL_L: u32 = 0xFF;

// *** MVOLR ***
pub const FB_MVOLR_MVOL_R: u32 = 0;
pub const FM_MVOLR_MVOL_R: u32 = 0xFF;

// *** HPVOLL ***
pub const FB_HPVOLL_HPVOL_L: u32 = 0;
pub const FM_HPVOLL_HPVOL_L: u32 = 0x7F;

// *** HPVOLR ***
pub const FB_HPVOLR_HPVOL_R: u32 = 0;
pub const FM_HPVOLR_HPVOL_R: u32 = 0x7F;

// *** SPKVOLL ***
pub const FB_SPKVOLL_SPKVOL_L: u32 = 0;
pub const FM_SPKVOLL_SPKVOL_L: u32 = 0x7F;

// *** SPKVOLR ***
pub const FB_SPKVOLR_SPKVOL_R: u32 = 0;
pub const FM_SPKVOLR_SPKVOL_R: u32 = 0x7F;

// *** SUBVOL ***
pub const FB_SUBVOL_SUBVOL: u32 = 0;
pub const FM_SUBVOL_SUBVOL: u32 = 0x7F;

// *** COP0 ***
pub const FB_COP0_COPATTEN: u32 = 7;
pub const FM_COP0_COPATTEN: u32 = 0x80;

pub const FB_COP0_COPGAIN: u32 = 6;
pub const FM_COP0_COPGAIN: u32 = 0x40;

pub const FB_COP0_HDELTAEN: u32 = 5;
pub const FM_COP0_HDELTAEN: u32 = 0x20;

pub const FB_COP0_COPTARGET: u32 = 0;
pub const FM_COP0_COPTARGET: u32 = 0x1F;

// *** COP1 ***
pub const FB_COP1_HDCOMPMODE: u32 = 6;
pub const FM_COP1_HDCOMPMODE: u32 = 0x40;

pub const FB_COP1_AVGLENGTH: u32 = 2;
pub const FM_COP1_AVGLENGTH: u32 = 0x3C;

pub const FB_COP1_MONRATE: u32 = 0;
pub const FM_COP1_MONRATE: u32 = 0x3;

// *** COPSTAT ***
pub const FB_COPSTAT_HDELTADET: u32 = 7;
pub const FM_COPSTAT_HDELTADET: u32 = 0x80;

pub const FB_COPSTAT_UV: u32 = 6;
pub const FM_COPSTAT_UV: u32 = 0x40;

pub const FB_COPSTAT_COPADJ: u32 = 0;
pub const FM_COPSTAT_COPADJ: u32 = 0x3F;

// *** PWM0 ***
pub const FB_PWM0_SCTO: u32 = 6;
pub const FM_PWM0_SCTO: u32 = 0xC0;

pub const FB_PWM0_UVLO: u32 = 5;
pub const FM_PWM0_UVLO: u32 = 0x20;

pub const FB_PWM0_BFDIS: u32 = 3;
pub const FM_PWM0_BFDIS: u32 = 0x8;

pub const FB_PWM0_PWMMODE: u32 = 2;
pub const FM_PWM0_PWMMODE: u32 = 0x4;

pub const FB_PWM0_NOOFFSET: u32 = 0;
pub const FM_PWM0_NOOFFSET: u32 = 0x1;

// *** PWM1 ***
pub const FB_PWM1_DITHPOS: u32 = 4;
pub const FM_PWM1_DITHPOS: u32 = 0x70;

pub const FB_PWM1_DYNDITH: u32 = 1;
pub const FM_PWM1_DYNDITH: u32 = 0x2;

pub const FB_PWM1_DITHDIS: u32 = 0;
pub const FM_PWM1_DITHDIS: u32 = 0x1;

// *** PWM2 ***
// *** PWM3 ***
pub const FB_PWM3_PWMMUX: u32 = 6;
pub const FM_PWM3_PWMMUX: u32 = 0xC0;

pub const FB_PWM3_CVALUE: u32 = 0;
pub const FM_PWM3_CVALUE: u32 = 0x7;

// *** HPSW ***
pub const FB_HPSW_HPDETSTATE: u32 = 4;
pub const FM_HPSW_HPDETSTATE: u32 = 0x10;

pub const FB_HPSW_HPSWEN: u32 = 2;
pub const FM_HPSW_HPSWEN: u32 = 0xC;

pub const FB_HPSW_HPSWPOL: u32 = 1;
pub const FM_HPSW_HPSWPOL: u32 = 0x2;

pub const FB_HPSW_TSDEN: u32 = 0;
pub const FM_HPSW_TSDEN: u32 = 0x1;

// *** THERMTS ***
pub const FB_THERMTS_TRIPHS: u32 = 7;
pub const FM_THERMTS_TRIPHS: u32 = 0x80;

pub const FB_THERMTS_TRIPLS: u32 = 6;
pub const FM_THERMTS_TRIPLS: u32 = 0x40;

pub const FB_THERMTS_TRIPSPLIT: u32 = 4;
pub const FM_THERMTS_TRIPSPLIT: u32 = 0x30;

pub const FB_THERMTS_TRIPSHIFT: u32 = 2;
pub const FM_THERMTS_TRIPSHIFT: u32 = 0xC;

pub const FB_THERMTS_TSPOLL: u32 = 0;
pub const FM_THERMTS_TSPOLL: u32 = 0x3;

// *** THERMSPK1 ***
pub const FB_THERMSPK1_FORCEPWD: u32 = 7;
pub const FM_THERMSPK1_FORCEPWD: u32 = 0x80;

pub const FB_THERMSPK1_INSTCUTMODE: u32 = 6;
pub const FM_THERMSPK1_INSTCUTMODE: u32 = 0x40;

pub const FB_THERMSPK1_INCRATIO: u32 = 4;
pub const FM_THERMSPK1_INCRATIO: u32 = 0x30;

pub const FB_THERMSPK1_INCSTEP: u32 = 2;
pub const FM_THERMSPK1_INCSTEP: u32 = 0xC;

pub const FB_THERMSPK1_DECSTEP: u32 = 0;
pub const FM_THERMSPK1_DECSTEP: u32 = 0x3;

// *** THERMSTAT ***
pub const FB_THERMSTAT_FPWDS: u32 = 7;
pub const FM_THERMSTAT_FPWDS: u32 = 0x80;

pub const FB_THERMSTAT_VOLSTAT: u32 = 0;
pub const FM_THERMSTAT_VOLSTAT: u32 = 0x7F;

// *** SCSTAT ***
pub const FB_SCSTAT_ESDF: u32 = 3;
pub const FM_SCSTAT_ESDF: u32 = 0x18;

pub const FB_SCSTAT_CPF: u32 = 2;
pub const FM_SCSTAT_CPF: u32 = 0x4;

pub const FB_SCSTAT_CLSDF: u32 = 0;
pub const FM_SCSTAT_CLSDF: u32 = 0x3;

// *** SDMON ***
pub const FB_SDMON_SDFORCE: u32 = 7;
pub const FM_SDMON_SDFORCE: u32 = 0x80;

pub const FB_SDMON_SDVALUE: u32 = 0;
pub const FM_SDMON_SDVALUE: u32 = 0x1F;

// *** SPKEQFILT ***
pub const FB_SPKEQFILT_EQ2EN: u32 = 7;
pub const FM_SPKEQFILT_EQ2EN: u32 = 0x80;
pub const FV_EQ2EN_ENABLE: u32 = 0x80;
pub const FV_EQ2EN_DISABLE: u32 = 0x0;

pub const FB_SPKEQFILT_EQ2BE: u32 = 4;
pub const FM_SPKEQFILT_EQ2BE: u32 = 0x70;

pub const FB_SPKEQFILT_EQ1EN: u32 = 3;
pub const FM_SPKEQFILT_EQ1EN: u32 = 0x8;
pub const FV_EQ1EN_ENABLE: u32 = 0x8;
pub const FV_EQ1EN_DISABLE: u32 = 0x0;

pub const FB_SPKEQFILT_EQ1BE: u32 = 0;
pub const FM_SPKEQFILT_EQ1BE: u32 = 0x7;

pub const SPKEQFILT_EQEN_ENABLE: u32 = 0x1;
pub const SPKEQFILT_EQEN_DISABLE: u32 = 0x0;

// *** SPKCRWDL ***
pub const FB_SPKCRWDL_WDATA_L: u32 = 0;
pub const FM_SPKCRWDL_WDATA_L: u32 = 0xFF;

// *** SPKCRWDM ***
pub const FB_SPKCRWDM_WDATA_M: u32 = 0;
pub const FM_SPKCRWDM_WDATA_M: u32 = 0xFF;

// *** SPKCRWDH ***
pub const FB_SPKCRWDH_WDATA_H: u32 = 0;
pub const FM_SPKCRWDH_WDATA_H: u32 = 0xFF;

// *** SPKCRRDL ***
pub const FB_SPKCRRDL_RDATA_L: u32 = 0;
pub const FM_SPKCRRDL_RDATA_L: u32 = 0xFF;

// *** SPKCRRDM ***
pub const FB_SPKCRRDM_RDATA_M: u32 = 0;
pub const FM_SPKCRRDM_RDATA_M: u32 = 0xFF;

// *** SPKCRRDH ***
pub const FB_SPKCRRDH_RDATA_H: u32 = 0;
pub const FM_SPKCRRDH_RDATA_H: u32 = 0xFF;

// *** SPKCRADD ***
pub const FB_SPKCRADD_ADDRESS: u32 = 0;
pub const FM_SPKCRADD_ADDRESS: u32 = 0xFF;

// *** SPKCRS ***
pub const FB_SPKCRS_ACCSTAT: u32 = 7;
pub const FM_SPKCRS_ACCSTAT: u32 = 0x80;

// *** SPKMBCEN ***
pub const FB_SPKMBCEN_MBCEN3: u32 = 2;
pub const FM_SPKMBCEN_MBCEN3: u32 = 0x4;
pub const FV_MBCEN3_ENABLE: u32 = 0x4;
pub const FV_MBCEN3_DISABLE: u32 = 0x0;

pub const FB_SPKMBCEN_MBCEN2: u32 = 1;
pub const FM_SPKMBCEN_MBCEN2: u32 = 0x2;
pub const FV_MBCEN2_ENABLE: u32 = 0x2;
pub const FV_MBCEN2_DISABLE: u32 = 0x0;

pub const FB_SPKMBCEN_MBCEN1: u32 = 0;
pub const FM_SPKMBCEN_MBCEN1: u32 = 0x1;
pub const FV_MBCEN1_ENABLE: u32 = 0x1;
pub const FV_MBCEN1_DISABLE: u32 = 0x0;

pub const SPKMBCEN_MBCEN_ENABLE: u32 = 0x1;
pub const SPKMBCEN_MBCEN_DISABLE: u32 = 0x0;

// *** SPKMBCCTL ***
pub const FB_SPKMBCCTL_LVLMODE3: u32 = 5;
pub const FM_SPKMBCCTL_LVLMODE3: u32 = 0x20;

pub const FB_SPKMBCCTL_WINSEL3: u32 = 4;
pub const FM_SPKMBCCTL_WINSEL3: u32 = 0x10;

pub const FB_SPKMBCCTL_LVLMODE2: u32 = 3;
pub const FM_SPKMBCCTL_LVLMODE2: u32 = 0x8;

pub const FB_SPKMBCCTL_WINSEL2: u32 = 2;
pub const FM_SPKMBCCTL_WINSEL2: u32 = 0x4;

pub const FB_SPKMBCCTL_LVLMODE1: u32 = 1;
pub const FM_SPKMBCCTL_LVLMODE1: u32 = 0x2;

pub const FB_SPKMBCCTL_WINSEL1: u32 = 0;
pub const FM_SPKMBCCTL_WINSEL1: u32 = 0x1;

// *** SPKCLECTL ***
pub const FB_SPKCLECTL_LVLMODE: u32 = 4;
pub const FM_SPKCLECTL_LVLMODE: u32 = 0x10;

pub const FB_SPKCLECTL_WINSEL: u32 = 3;
pub const FM_SPKCLECTL_WINSEL: u32 = 0x8;

pub const FB_SPKCLECTL_EXPEN: u32 = 2;
pub const FM_SPKCLECTL_EXPEN: u32 = 0x4;
pub const FV_EXPEN_ENABLE: u32 = 0x4;
pub const FV_EXPEN_DISABLE: u32 = 0x0;

pub const FB_SPKCLECTL_LIMEN: u32 = 1;
pub const FM_SPKCLECTL_LIMEN: u32 = 0x2;
pub const FV_LIMEN_ENABLE: u32 = 0x2;
pub const FV_LIMEN_DISABLE: u32 = 0x0;

pub const FB_SPKCLECTL_COMPEN: u32 = 0;
pub const FM_SPKCLECTL_COMPEN: u32 = 0x1;
pub const FV_COMPEN_ENABLE: u32 = 0x1;
pub const FV_COMPEN_DISABLE: u32 = 0x0;

// *** SPKCLEMUG ***
pub const FB_SPKCLEMUG_MUGAIN: u32 = 0;
pub const FM_SPKCLEMUG_MUGAIN: u32 = 0x1F;

// *** SPKCOMPTHR ***
pub const FB_SPKCOMPTHR_THRESH: u32 = 0;
pub const FM_SPKCOMPTHR_THRESH: u32 = 0xFF;

// *** SPKCOMPRAT ***
pub const FB_SPKCOMPRAT_RATIO: u32 = 0;
pub const FM_SPKCOMPRAT_RATIO: u32 = 0x1F;

// *** SPKCOMPATKL ***
pub const FB_SPKCOMPATKL_TCATKL: u32 = 0;
pub const FM_SPKCOMPATKL_TCATKL: u32 = 0xFF;

// *** SPKCOMPATKH ***
pub const FB_SPKCOMPATKH_TCATKH: u32 = 0;
pub const FM_SPKCOMPATKH_TCATKH: u32 = 0xFF;

// *** SPKCOMPRELL ***
pub const FB_SPKCOMPRELL_TCRELL: u32 = 0;
pub const FM_SPKCOMPRELL_TCRELL: u32 = 0xFF;

// *** SPKCOMPRELH ***
pub const FB_SPKCOMPRELH_TCRELH: u32 = 0;
pub const FM_SPKCOMPRELH_TCRELH: u32 = 0xFF;

// *** SPKLIMTHR ***
pub const FB_SPKLIMTHR_THRESH: u32 = 0;
pub const FM_SPKLIMTHR_THRESH: u32 = 0xFF;

// *** SPKLIMTGT ***
pub const FB_SPKLIMTGT_TARGET: u32 = 0;
pub const FM_SPKLIMTGT_TARGET: u32 = 0xFF;

// *** SPKLIMATKL ***
pub const FB_SPKLIMATKL_TCATKL: u32 = 0;
pub const FM_SPKLIMATKL_TCATKL: u32 = 0xFF;

// *** SPKLIMATKH ***
pub const FB_SPKLIMATKH_TCATKH: u32 = 0;
pub const FM_SPKLIMATKH_TCATKH: u32 = 0xFF;

// *** SPKLIMRELL ***
pub const FB_SPKLIMRELL_TCRELL: u32 = 0;
pub const FM_SPKLIMRELL_TCRELL: u32 = 0xFF;

// *** SPKLIMRELH ***
pub const FB_SPKLIMRELH_TCRELH: u32 = 0;
pub const FM_SPKLIMRELH_TCRELH: u32 = 0xFF;

// *** SPKEXPTHR ***
pub const FB_SPKEXPTHR_THRESH: u32 = 0;
pub const FM_SPKEXPTHR_THRESH: u32 = 0xFF;

// *** SPKEXPRAT ***
pub const FB_SPKEXPRAT_RATIO: u32 = 0;
pub const FM_SPKEXPRAT_RATIO: u32 = 0x7;

// *** SPKEXPATKL ***
pub const FB_SPKEXPATKL_TCATKL: u32 = 0;
pub const FM_SPKEXPATKL_TCATKL: u32 = 0xFF;

// *** SPKEXPATKH ***
pub const FB_SPKEXPATKH_TCATKH: u32 = 0;
pub const FM_SPKEXPATKH_TCATKH: u32 = 0xFF;

// *** SPKEXPRELL ***
pub const FB_SPKEXPRELL_TCRELL: u32 = 0;
pub const FM_SPKEXPRELL_TCRELL: u32 = 0xFF;

// *** SPKEXPRELH ***
pub const FB_SPKEXPRELH_TCRELH: u32 = 0;
pub const FM_SPKEXPRELH_TCRELH: u32 = 0xFF;

// *** SPKFXCTL ***
pub const FB_SPKFXCTL_3DEN: u32 = 4;
pub const FM_SPKFXCTL_3DEN: u32 = 0x10;

pub const FB_SPKFXCTL_TEEN: u32 = 3;
pub const FM_SPKFXCTL_TEEN: u32 = 0x8;

pub const FB_SPKFXCTL_TNLFBYP: u32 = 2;
pub const FM_SPKFXCTL_TNLFBYP: u32 = 0x4;

pub const FB_SPKFXCTL_BEEN: u32 = 1;
pub const FM_SPKFXCTL_BEEN: u32 = 0x2;

pub const FB_SPKFXCTL_BNLFBYP: u32 = 0;
pub const FM_SPKFXCTL_BNLFBYP: u32 = 0x1;

// *** DACEQFILT ***
pub const FB_DACEQFILT_EQ2EN: u32 = 7;
pub const FM_DACEQFILT_EQ2EN: u32 = 0x80;
pub const FV_EQ2EN_ENABLE: u32 = 0x80;
pub const FV_EQ2EN_DISABLE: u32 = 0x0;

pub const FB_DACEQFILT_EQ2BE: u32 = 4;
pub const FM_DACEQFILT_EQ2BE: u32 = 0x70;

pub const FB_DACEQFILT_EQ1EN: u32 = 3;
pub const FM_DACEQFILT_EQ1EN: u32 = 0x8;
pub const FV_EQ1EN_ENABLE: u32 = 0x8;
pub const FV_EQ1EN_DISABLE: u32 = 0x0;

pub const FB_DACEQFILT_EQ1BE: u32 = 0;
pub const FM_DACEQFILT_EQ1BE: u32 = 0x7;

pub const DACEQFILT_EQEN_ENABLE: u32 = 0x1;
pub const DACEQFILT_EQEN_DISABLE: u32 = 0x0;

// *** DACCRWDL ***
pub const FB_DACCRWDL_WDATA_L: u32 = 0;
pub const FM_DACCRWDL_WDATA_L: u32 = 0xFF;

// *** DACCRWDM ***
pub const FB_DACCRWDM_WDATA_M: u32 = 0;
pub const FM_DACCRWDM_WDATA_M: u32 = 0xFF;

// *** DACCRWDH ***
pub const FB_DACCRWDH_WDATA_H: u32 = 0;
pub const FM_DACCRWDH_WDATA_H: u32 = 0xFF;

// *** DACCRRDL ***
pub const FB_DACCRRDL_RDATA_L: u32 = 0;
pub const FM_DACCRRDL_RDATA_L: u32 = 0xFF;

// *** DACCRRDM ***
pub const FB_DACCRRDM_RDATA_M: u32 = 0;
pub const FM_DACCRRDM_RDATA_M: u32 = 0xFF;

// *** DACCRRDH ***
pub const FB_DACCRRDH_RDATA_H: u32 = 0;
pub const FM_DACCRRDH_RDATA_H: u32 = 0xFF;

// *** DACCRADD ***
pub const FB_DACCRADD_ADDRESS: u32 = 0;
pub const FM_DACCRADD_ADDRESS: u32 = 0xFF;

// *** DACCRS ***
pub const FB_DACCRS_ACCSTAT: u32 = 7;
pub const FM_DACCRS_ACCSTAT: u32 = 0x80;

// *** DACMBCEN ***
pub const FB_DACMBCEN_MBCEN3: u32 = 2;
pub const FM_DACMBCEN_MBCEN3: u32 = 0x4;
pub const FV_MBCEN3_ENABLE: u32 = 0x4;
pub const FV_MBCEN3_DISABLE: u32 = 0x0;

pub const FB_DACMBCEN_MBCEN2: u32 = 1;
pub const FM_DACMBCEN_MBCEN2: u32 = 0x2;
pub const FV_MBCEN2_ENABLE: u32 = 0x2;
pub const FV_MBCEN2_DISABLE: u32 = 0x0;

pub const FB_DACMBCEN_MBCEN1: u32 = 0;
pub const FM_DACMBCEN_MBCEN1: u32 = 0x1;
pub const FV_MBCEN1_ENABLE: u32 = 0x1;
pub const FV_MBCEN1_DISABLE: u32 = 0x0;

pub const DACMBCEN_MBCEN_ENABLE: u32 = 0x1;
pub const DACMBCEN_MBCEN_DISABLE: u32 = 0x0;

// *** DACMBCCTL ***
pub const FB_DACMBCCTL_LVLMODE3: u32 = 5;
pub const FM_DACMBCCTL_LVLMODE3: u32 = 0x20;

pub const FB_DACMBCCTL_WINSEL3: u32 = 4;
pub const FM_DACMBCCTL_WINSEL3: u32 = 0x10;

pub const FB_DACMBCCTL_LVLMODE2: u32 = 3;
pub const FM_DACMBCCTL_LVLMODE2: u32 = 0x8;

pub const FB_DACMBCCTL_WINSEL2: u32 = 2;
pub const FM_DACMBCCTL_WINSEL2: u32 = 0x4;

pub const FB_DACMBCCTL_LVLMODE1: u32 = 1;
pub const FM_DACMBCCTL_LVLMODE1: u32 = 0x2;

pub const FB_DACMBCCTL_WINSEL1: u32 = 0;
pub const FM_DACMBCCTL_WINSEL1: u32 = 0x1;

// *** DACCLECTL ***
pub const FB_DACCLECTL_LVLMODE: u32 = 4;
pub const FM_DACCLECTL_LVLMODE: u32 = 0x10;

pub const FB_DACCLECTL_WINSEL: u32 = 3;
pub const FM_DACCLECTL_WINSEL: u32 = 0x8;

pub const FB_DACCLECTL_EXPEN: u32 = 2;
pub const FM_DACCLECTL_EXPEN: u32 = 0x4;
pub const FV_EXPEN_ENABLE: u32 = 0x4;
pub const FV_EXPEN_DISABLE: u32 = 0x0;

pub const FB_DACCLECTL_LIMEN: u32 = 1;
pub const FM_DACCLECTL_LIMEN: u32 = 0x2;
pub const FV_LIMEN_ENABLE: u32 = 0x2;
pub const FV_LIMEN_DISABLE: u32 = 0x0;

pub const FB_DACCLECTL_COMPEN: u32 = 0;
pub const FM_DACCLECTL_COMPEN: u32 = 0x1;
pub const FV_COMPEN_ENABLE: u32 = 0x1;
pub const FV_COMPEN_DISABLE: u32 = 0x0;

// *** DACCLEMUG ***
pub const FB_DACCLEMUG_MUGAIN: u32 = 0;
pub const FM_DACCLEMUG_MUGAIN: u32 = 0x1F;

// *** DACCOMPTHR ***
pub const FB_DACCOMPTHR_THRESH: u32 = 0;
pub const FM_DACCOMPTHR_THRESH: u32 = 0xFF;

// *** DACCOMPRAT ***
pub const FB_DACCOMPRAT_RATIO: u32 = 0;
pub const FM_DACCOMPRAT_RATIO: u32 = 0x1F;

// *** DACCOMPATKL ***
pub const FB_DACCOMPATKL_TCATKL: u32 = 0;
pub const FM_DACCOMPATKL_TCATKL: u32 = 0xFF;

// *** DACCOMPATKH ***
pub const FB_DACCOMPATKH_TCATKH: u32 = 0;
pub const FM_DACCOMPATKH_TCATKH: u32 = 0xFF;

// *** DACCOMPRELL ***
pub const FB_DACCOMPRELL_TCRELL: u32 = 0;
pub const FM_DACCOMPRELL_TCRELL: u32 = 0xFF;

// *** DACCOMPRELH ***
pub const FB_DACCOMPRELH_TCRELH: u32 = 0;
pub const FM_DACCOMPRELH_TCRELH: u32 = 0xFF;

// *** DACLIMTHR ***
pub const FB_DACLIMTHR_THRESH: u32 = 0;
pub const FM_DACLIMTHR_THRESH: u32 = 0xFF;

// *** DACLIMTGT ***
pub const FB_DACLIMTGT_TARGET: u32 = 0;
pub const FM_DACLIMTGT_TARGET: u32 = 0xFF;

// *** DACLIMATKL ***
pub const FB_DACLIMATKL_TCATKL: u32 = 0;
pub const FM_DACLIMATKL_TCATKL: u32 = 0xFF;

// *** DACLIMATKH ***
pub const FB_DACLIMATKH_TCATKH: u32 = 0;
pub const FM_DACLIMATKH_TCATKH: u32 = 0xFF;

// *** DACLIMRELL ***
pub const FB_DACLIMRELL_TCRELL: u32 = 0;
pub const FM_DACLIMRELL_TCRELL: u32 = 0xFF;

// *** DACLIMRELH ***
pub const FB_DACLIMRELH_TCRELH: u32 = 0;
pub const FM_DACLIMRELH_TCRELH: u32 = 0xFF;

// *** DACEXPTHR ***
pub const FB_DACEXPTHR_THRESH: u32 = 0;
pub const FM_DACEXPTHR_THRESH: u32 = 0xFF;

// *** DACEXPRAT ***
pub const FB_DACEXPRAT_RATIO: u32 = 0;
pub const FM_DACEXPRAT_RATIO: u32 = 0x7;

// *** DACEXPATKL ***
pub const FB_DACEXPATKL_TCATKL: u32 = 0;
pub const FM_DACEXPATKL_TCATKL: u32 = 0xFF;

// *** DACEXPATKH ***
pub const FB_DACEXPATKH_TCATKH: u32 = 0;
pub const FM_DACEXPATKH_TCATKH: u32 = 0xFF;

// *** DACEXPRELL ***
pub const FB_DACEXPRELL_TCRELL: u32 = 0;
pub const FM_DACEXPRELL_TCRELL: u32 = 0xFF;

// *** DACEXPRELH ***
pub const FB_DACEXPRELH_TCRELH: u32 = 0;
pub const FM_DACEXPRELH_TCRELH: u32 = 0xFF;

// *** DACFXCTL ***
pub const FB_DACFXCTL_3DEN: u32 = 4;
pub const FM_DACFXCTL_3DEN: u32 = 0x10;

pub const FB_DACFXCTL_TEEN: u32 = 3;
pub const FM_DACFXCTL_TEEN: u32 = 0x8;

pub const FB_DACFXCTL_TNLFBYP: u32 = 2;
pub const FM_DACFXCTL_TNLFBYP: u32 = 0x4;

pub const FB_DACFXCTL_BEEN: u32 = 1;
pub const FM_DACFXCTL_BEEN: u32 = 0x2;

pub const FB_DACFXCTL_BNLFBYP: u32 = 0;
pub const FM_DACFXCTL_BNLFBYP: u32 = 0x1;

// *** SUBEQFILT ***
pub const FB_SUBEQFILT_EQ2EN: u32 = 7;
pub const FM_SUBEQFILT_EQ2EN: u32 = 0x80;
pub const FV_EQ2EN_ENABLE: u32 = 0x80;
pub const FV_EQ2EN_DISABLE: u32 = 0x0;

pub const FB_SUBEQFILT_EQ2BE: u32 = 4;
pub const FM_SUBEQFILT_EQ2BE: u32 = 0x70;

pub const FB_SUBEQFILT_EQ1EN: u32 = 3;
pub const FM_SUBEQFILT_EQ1EN: u32 = 0x8;
pub const FV_EQ1EN_ENABLE: u32 = 0x8;
pub const FV_EQ1EN_DISABLE: u32 = 0x0;

pub const FB_SUBEQFILT_EQ1BE: u32 = 0;
pub const FM_SUBEQFILT_EQ1BE: u32 = 0x7;

pub const SUBEQFILT_EQEN_ENABLE: u32 = 0x1;
pub const SUBEQFILT_EQEN_DISABLE: u32 = 0x0;

// *** SUBCRWDL ***
pub const FB_SUBCRWDL_WDATA_L: u32 = 0;
pub const FM_SUBCRWDL_WDATA_L: u32 = 0xFF;

// *** SUBCRWDM ***
pub const FB_SUBCRWDM_WDATA_M: u32 = 0;
pub const FM_SUBCRWDM_WDATA_M: u32 = 0xFF;

// *** SUBCRWDH ***
pub const FB_SUBCRWDH_WDATA_H: u32 = 0;
pub const FM_SUBCRWDH_WDATA_H: u32 = 0xFF;

// *** SUBCRRDL ***
pub const FB_SUBCRRDL_RDATA_L: u32 = 0;
pub const FM_SUBCRRDL_RDATA_L: u32 = 0xFF;

// *** SUBCRRDM ***
pub const FB_SUBCRRDM_RDATA_M: u32 = 0;
pub const FM_SUBCRRDM_RDATA_M: u32 = 0xFF;

// *** SUBCRRDH ***
pub const FB_SUBCRRDH_RDATA_H: u32 = 0;
pub const FM_SUBCRRDH_RDATA_H: u32 = 0xFF;

// *** SUBCRADD ***
pub const FB_SUBCRADD_ADDRESS: u32 = 0;
pub const FM_SUBCRADD_ADDRESS: u32 = 0xFF;

// *** SUBCRS ***
pub const FB_SUBCRS_ACCSTAT: u32 = 7;
pub const FM_SUBCRS_ACCSTAT: u32 = 0x80;

// *** SUBMBCEN ***
pub const FB_SUBMBCEN_MBCEN3: u32 = 2;
pub const FM_SUBMBCEN_MBCEN3: u32 = 0x4;
pub const FV_MBCEN3_ENABLE: u32 = 0x4;
pub const FV_MBCEN3_DISABLE: u32 = 0x0;

pub const FB_SUBMBCEN_MBCEN2: u32 = 1;
pub const FM_SUBMBCEN_MBCEN2: u32 = 0x2;
pub const FV_MBCEN2_ENABLE: u32 = 0x2;
pub const FV_MBCEN2_DISABLE: u32 = 0x0;

pub const FB_SUBMBCEN_MBCEN1: u32 = 0;
pub const FM_SUBMBCEN_MBCEN1: u32 = 0x1;
pub const FV_MBCEN1_ENABLE: u32 = 0x1;
pub const FV_MBCEN1_DISABLE: u32 = 0x0;

pub const SUBMBCEN_MBCEN_ENABLE: u32 = 0x1;
pub const SUBMBCEN_MBCEN_DISABLE: u32 = 0x0;

// *** SUBMBCCTL ***
pub const FB_SUBMBCCTL_LVLMODE3: u32 = 5;
pub const FM_SUBMBCCTL_LVLMODE3: u32 = 0x20;

pub const FB_SUBMBCCTL_WINSEL3: u32 = 4;
pub const FM_SUBMBCCTL_WINSEL3: u32 = 0x10;

pub const FB_SUBMBCCTL_LVLMODE2: u32 = 3;
pub const FM_SUBMBCCTL_LVLMODE2: u32 = 0x8;

pub const FB_SUBMBCCTL_WINSEL2: u32 = 2;
pub const FM_SUBMBCCTL_WINSEL2: u32 = 0x4;

pub const FB_SUBMBCCTL_LVLMODE1: u32 = 1;
pub const FM_SUBMBCCTL_LVLMODE1: u32 = 0x2;

pub const FB_SUBMBCCTL_WINSEL1: u32 = 0;
pub const FM_SUBMBCCTL_WINSEL1: u32 = 0x1;

// *** SUBCLECTL ***
pub const FB_SUBCLECTL_LVLMODE: u32 = 4;
pub const FM_SUBCLECTL_LVLMODE: u32 = 0x10;

pub const FB_SUBCLECTL_WINSEL: u32 = 3;
pub const FM_SUBCLECTL_WINSEL: u32 = 0x8;

pub const FB_SUBCLECTL_EXPEN: u32 = 2;
pub const FM_SUBCLECTL_EXPEN: u32 = 0x4;
pub const FV_EXPEN_ENABLE: u32 = 0x4;
pub const FV_EXPEN_DISABLE: u32 = 0x0;

pub const FB_SUBCLECTL_LIMEN: u32 = 1;
pub const FM_SUBCLECTL_LIMEN: u32 = 0x2;
pub const FV_LIMEN_ENABLE: u32 = 0x2;
pub const FV_LIMEN_DISABLE: u32 = 0x0;

pub const FB_SUBCLECTL_COMPEN: u32 = 0;
pub const FM_SUBCLECTL_COMPEN: u32 = 0x1;
pub const FV_COMPEN_ENABLE: u32 = 0x1;
pub const FV_COMPEN_DISABLE: u32 = 0x0;

// *** SUBCLEMUG ***
pub const FB_SUBCLEMUG_MUGAIN: u32 = 0;
pub const FM_SUBCLEMUG_MUGAIN: u32 = 0x1F;

// *** SUBCOMPTHR ***
pub const FB_SUBCOMPTHR_THRESH: u32 = 0;
pub const FM_SUBCOMPTHR_THRESH: u32 = 0xFF;

// *** SUBCOMPRAT ***
pub const FB_SUBCOMPRAT_RATIO: u32 = 0;
pub const FM_SUBCOMPRAT_RATIO: u32 = 0x1F;

// *** SUBCOMPATKL ***
pub const FB_SUBCOMPATKL_TCATKL: u32 = 0;
pub const FM_SUBCOMPATKL_TCATKL: u32 = 0xFF;

// *** SUBCOMPATKH ***
pub const FB_SUBCOMPATKH_TCATKH: u32 = 0;
pub const FM_SUBCOMPATKH_TCATKH: u32 = 0xFF;

// *** SUBCOMPRELL ***
pub const FB_SUBCOMPRELL_TCRELL: u32 = 0;
pub const FM_SUBCOMPRELL_TCRELL: u32 = 0xFF;

// *** SUBCOMPRELH ***
pub const FB_SUBCOMPRELH_TCRELH: u32 = 0;
pub const FM_SUBCOMPRELH_TCRELH: u32 = 0xFF;

// *** SUBLIMTHR ***
pub const FB_SUBLIMTHR_THRESH: u32 = 0;
pub const FM_SUBLIMTHR_THRESH: u32 = 0xFF;

// *** SUBLIMTGT ***
pub const FB_SUBLIMTGT_TARGET: u32 = 0;
pub const FM_SUBLIMTGT_TARGET: u32 = 0xFF;

// *** SUBLIMATKL ***
pub const FB_SUBLIMATKL_TCATKL: u32 = 0;
pub const FM_SUBLIMATKL_TCATKL: u32 = 0xFF;

// *** SUBLIMATKH ***
pub const FB_SUBLIMATKH_TCATKH: u32 = 0;
pub const FM_SUBLIMATKH_TCATKH: u32 = 0xFF;

// *** SUBLIMRELL ***
pub const FB_SUBLIMRELL_TCRELL: u32 = 0;
pub const FM_SUBLIMRELL_TCRELL: u32 = 0xFF;

// *** SUBLIMRELH ***
pub const FB_SUBLIMRELH_TCRELH: u32 = 0;
pub const FM_SUBLIMRELH_TCRELH: u32 = 0xFF;

// *** SUBEXPTHR ***
pub const FB_SUBEXPTHR_THRESH: u32 = 0;
pub const FM_SUBEXPTHR_THRESH: u32 = 0xFF;

// *** SUBEXPRAT ***
pub const FB_SUBEXPRAT_RATIO: u32 = 0;
pub const FM_SUBEXPRAT_RATIO: u32 = 0x7;

// *** SUBEXPATKL ***
pub const FB_SUBEXPATKL_TCATKL: u32 = 0;
pub const FM_SUBEXPATKL_TCATKL: u32 = 0xFF;

// *** SUBEXPATKH ***
pub const FB_SUBEXPATKH_TCATKH: u32 = 0;
pub const FM_SUBEXPATKH_TCATKH: u32 = 0xFF;

// *** SUBEXPRELL ***
pub const FB_SUBEXPRELL_TCRELL: u32 = 0;
pub const FM_SUBEXPRELL_TCRELL: u32 = 0xFF;

// *** SUBEXPRELH ***
pub const FB_SUBEXPRELH_TCRELH: u32 = 0;
pub const FM_SUBEXPRELH_TCRELH: u32 = 0xFF;

// *** SUBFXCTL ***
pub const FB_SUBFXCTL_TEEN: u32 = 3;
pub const FM_SUBFXCTL_TEEN: u32 = 0x8;

pub const FB_SUBFXCTL_TNLFBYP: u32 = 2;
pub const FM_SUBFXCTL_TNLFBYP: u32 = 0x4;

pub const FB_SUBFXCTL_BEEN: u32 = 1;
pub const FM_SUBFXCTL_BEEN: u32 = 0x2;

pub const FB_SUBFXCTL_BNLFBYP: u32 = 0;
pub const FM_SUBFXCTL_BNLFBYP: u32 = 0x1;


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
