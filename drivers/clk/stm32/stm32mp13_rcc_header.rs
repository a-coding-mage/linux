/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/*
 * Copyright (C) 2020, STMicroelectronics - All Rights Reserved
 *
 * Configuration settings for the STM32MP13x CPU
 */

/* RCC registers */
pub const RCC_SECCFGR: u32 = 0x0;
pub const RCC_MP_SREQSETR: u32 = 0x100;
pub const RCC_MP_SREQCLRR: u32 = 0x104;
pub const RCC_MP_APRSTCR: u32 = 0x108;
pub const RCC_MP_APRSTSR: u32 = 0x10c;
pub const RCC_PWRLPDLYCR: u32 = 0x110;
pub const RCC_MP_GRSTCSETR: u32 = 0x114;
pub const RCC_BR_RSTSCLRR: u32 = 0x118;
pub const RCC_MP_RSTSSETR: u32 = 0x11c;
pub const RCC_MP_RSTSCLRR: u32 = 0x120;
pub const RCC_MP_IWDGFZSETR: u32 = 0x124;
pub const RCC_MP_IWDGFZCLRR: u32 = 0x128;
pub const RCC_MP_CIER: u32 = 0x200;
pub const RCC_MP_CIFR: u32 = 0x204;
pub const RCC_BDCR: u32 = 0x400;
pub const RCC_RDLSICR: u32 = 0x404;
pub const RCC_OCENSETR: u32 = 0x420;
pub const RCC_OCENCLRR: u32 = 0x424;
pub const RCC_OCRDYR: u32 = 0x428;
pub const RCC_HSICFGR: u32 = 0x440;
pub const RCC_CSICFGR: u32 = 0x444;
pub const RCC_MCO1CFGR: u32 = 0x460;
pub const RCC_MCO2CFGR: u32 = 0x464;
pub const RCC_DBGCFGR: u32 = 0x468;
pub const RCC_RCK12SELR: u32 = 0x480;
pub const RCC_RCK3SELR: u32 = 0x484;
pub const RCC_RCK4SELR: u32 = 0x488;
pub const RCC_PLL1CR: u32 = 0x4a0;
pub const RCC_PLL1CFGR1: u32 = 0x4a4;
pub const RCC_PLL1CFGR2: u32 = 0x4a8;
pub const RCC_PLL1FRACR: u32 = 0x4ac;
pub const RCC_PLL1CSGR: u32 = 0x4b0;
pub const RCC_PLL2CR: u32 = 0x4d0;
pub const RCC_PLL2CFGR1: u32 = 0x4d4;
pub const RCC_PLL2CFGR2: u32 = 0x4d8;
pub const RCC_PLL2FRACR: u32 = 0x4dc;
pub const RCC_PLL2CSGR: u32 = 0x4e0;
pub const RCC_PLL3CR: u32 = 0x500;
pub const RCC_PLL3CFGR1: u32 = 0x504;
pub const RCC_PLL3CFGR2: u32 = 0x508;
pub const RCC_PLL3FRACR: u32 = 0x50c;
pub const RCC_PLL3CSGR: u32 = 0x510;
pub const RCC_PLL4CR: u32 = 0x520;
pub const RCC_PLL4CFGR1: u32 = 0x524;
pub const RCC_PLL4CFGR2: u32 = 0x528;
pub const RCC_PLL4FRACR: u32 = 0x52c;
pub const RCC_PLL4CSGR: u32 = 0x530;
pub const RCC_MPCKSELR: u32 = 0x540;
pub const RCC_ASSCKSELR: u32 = 0x544;
pub const RCC_MSSCKSELR: u32 = 0x548;
pub const RCC_CPERCKSELR: u32 = 0x54c;
pub const RCC_RTCDIVR: u32 = 0x560;
pub const RCC_MPCKDIVR: u32 = 0x564;
pub const RCC_AXIDIVR: u32 = 0x568;
pub const RCC_MLAHBDIVR: u32 = 0x56c;
pub const RCC_APB1DIVR: u32 = 0x570;
pub const RCC_APB2DIVR: u32 = 0x574;
pub const RCC_APB3DIVR: u32 = 0x578;
pub const RCC_APB4DIVR: u32 = 0x57c;
pub const RCC_APB5DIVR: u32 = 0x580;
pub const RCC_APB6DIVR: u32 = 0x584;
pub const RCC_TIMG1PRER: u32 = 0x5a0;
pub const RCC_TIMG2PRER: u32 = 0x5a4;
pub const RCC_TIMG3PRER: u32 = 0x5a8;
pub const RCC_DDRITFCR: u32 = 0x5c0;
pub const RCC_I2C12CKSELR: u32 = 0x600;
pub const RCC_I2C345CKSELR: u32 = 0x604;
pub const RCC_SPI2S1CKSELR: u32 = 0x608;
pub const RCC_SPI2S23CKSELR: u32 = 0x60c;
pub const RCC_SPI45CKSELR: u32 = 0x610;
pub const RCC_UART12CKSELR: u32 = 0x614;
pub const RCC_UART35CKSELR: u32 = 0x618;
pub const RCC_UART4CKSELR: u32 = 0x61c;
pub const RCC_UART6CKSELR: u32 = 0x620;
pub const RCC_UART78CKSELR: u32 = 0x624;
pub const RCC_LPTIM1CKSELR: u32 = 0x628;
pub const RCC_LPTIM23CKSELR: u32 = 0x62c;
pub const RCC_LPTIM45CKSELR: u32 = 0x630;
pub const RCC_SAI1CKSELR: u32 = 0x634;
pub const RCC_SAI2CKSELR: u32 = 0x638;
pub const RCC_FDCANCKSELR: u32 = 0x63c;
pub const RCC_SPDIFCKSELR: u32 = 0x640;
pub const RCC_ADC12CKSELR: u32 = 0x644;
pub const RCC_SDMMC12CKSELR: u32 = 0x648;
pub const RCC_ETH12CKSELR: u32 = 0x64c;
pub const RCC_USBCKSELR: u32 = 0x650;
pub const RCC_QSPICKSELR: u32 = 0x654;
pub const RCC_FMCCKSELR: u32 = 0x658;
pub const RCC_RNG1CKSELR: u32 = 0x65c;
pub const RCC_STGENCKSELR: u32 = 0x660;
pub const RCC_DCMIPPCKSELR: u32 = 0x664;
pub const RCC_SAESCKSELR: u32 = 0x668;
pub const RCC_APB1RSTSETR: u32 = 0x6a0;
pub const RCC_APB1RSTCLRR: u32 = 0x6a4;
pub const RCC_APB2RSTSETR: u32 = 0x6a8;
pub const RCC_APB2RSTCLRR: u32 = 0x6ac;
pub const RCC_APB3RSTSETR: u32 = 0x6b0;
pub const RCC_APB3RSTCLRR: u32 = 0x6b4;
pub const RCC_APB4RSTSETR: u32 = 0x6b8;
pub const RCC_APB4RSTCLRR: u32 = 0x6bc;
pub const RCC_APB5RSTSETR: u32 = 0x6c0;
pub const RCC_APB5RSTCLRR: u32 = 0x6c4;
pub const RCC_APB6RSTSETR: u32 = 0x6c8;
pub const RCC_APB6RSTCLRR: u32 = 0x6cc;
pub const RCC_AHB2RSTSETR: u32 = 0x6d0;
pub const RCC_AHB2RSTCLRR: u32 = 0x6d4;
pub const RCC_AHB4RSTSETR: u32 = 0x6e0;
pub const RCC_AHB4RSTCLRR: u32 = 0x6e4;
pub const RCC_AHB5RSTSETR: u32 = 0x6e8;
pub const RCC_AHB5RSTCLRR: u32 = 0x6ec;
pub const RCC_AHB6RSTSETR: u32 = 0x6f0;
pub const RCC_AHB6RSTCLRR: u32 = 0x6f4;
pub const RCC_MP_APB1ENSETR: u32 = 0x700;
pub const RCC_MP_APB1ENCLRR: u32 = 0x704;
pub const RCC_MP_APB2ENSETR: u32 = 0x708;
pub const RCC_MP_APB2ENCLRR: u32 = 0x70c;
pub const RCC_MP_APB3ENSETR: u32 = 0x710;
pub const RCC_MP_APB3ENCLRR: u32 = 0x714;
pub const RCC_MP_S_APB3ENSETR: u32 = 0x718;
pub const RCC_MP_S_APB3ENCLRR: u32 = 0x71c;
pub const RCC_MP_NS_APB3ENSETR: u32 = 0x720;
pub const RCC_MP_NS_APB3ENCLRR: u32 = 0x724;
pub const RCC_MP_APB4ENSETR: u32 = 0x728;
pub const RCC_MP_APB4ENCLRR: u32 = 0x72c;
pub const RCC_MP_S_APB4ENSETR: u32 = 0x730;
pub const RCC_MP_S_APB4ENCLRR: u32 = 0x734;
pub const RCC_MP_NS_APB4ENSETR: u32 = 0x738;
pub const RCC_MP_NS_APB4ENCLRR: u32 = 0x73c;
pub const RCC_MP_APB5ENSETR: u32 = 0x740;
pub const RCC_MP_APB5ENCLRR: u32 = 0x744;
pub const RCC_MP_APB6ENSETR: u32 = 0x748;
pub const RCC_MP_APB6ENCLRR: u32 = 0x74c;
pub const RCC_MP_AHB2ENSETR: u32 = 0x750;
pub const RCC_MP_AHB2ENCLRR: u32 = 0x754;
pub const RCC_MP_AHB4ENSETR: u32 = 0x760;
pub const RCC_MP_AHB4ENCLRR: u32 = 0x764;
pub const RCC_MP_S_AHB4ENSETR: u32 = 0x768;
pub const RCC_MP_S_AHB4ENCLRR: u32 = 0x76c;
pub const RCC_MP_NS_AHB4ENSETR: u32 = 0x770;
pub const RCC_MP_NS_AHB4ENCLRR: u32 = 0x774;
pub const RCC_MP_AHB5ENSETR: u32 = 0x778;
pub const RCC_MP_AHB5ENCLRR: u32 = 0x77c;
pub const RCC_MP_AHB6ENSETR: u32 = 0x780;
pub const RCC_MP_AHB6ENCLRR: u32 = 0x784;
pub const RCC_MP_S_AHB6ENSETR: u32 = 0x788;
pub const RCC_MP_S_AHB6ENCLRR: u32 = 0x78c;
pub const RCC_MP_NS_AHB6ENSETR: u32 = 0x790;
pub const RCC_MP_NS_AHB6ENCLRR: u32 = 0x794;
pub const RCC_MP_APB1LPENSETR: u32 = 0x800;
pub const RCC_MP_APB1LPENCLRR: u32 = 0x804;
pub const RCC_MP_APB2LPENSETR: u32 = 0x808;
pub const RCC_MP_APB2LPENCLRR: u32 = 0x80c;
pub const RCC_MP_APB3LPENSETR: u32 = 0x810;
pub const RCC_MP_APB3LPENCLRR: u32 = 0x814;
pub const RCC_MP_S_APB3LPENSETR: u32 = 0x818;
pub const RCC_MP_S_APB3LPENCLRR: u32 = 0x81c;
pub const RCC_MP_NS_APB3LPENSETR: u32 = 0x820;
pub const RCC_MP_NS_APB3LPENCLRR: u32 = 0x824;
pub const RCC_MP_APB4LPENSETR: u32 = 0x828;
pub const RCC_MP_APB4LPENCLRR: u32 = 0x82c;
pub const RCC_MP_S_APB4LPENSETR: u32 = 0x830;
pub const RCC_MP_S_APB4LPENCLRR: u32 = 0x834;
pub const RCC_MP_NS_APB4LPENSETR: u32 = 0x838;
pub const RCC_MP_NS_APB4LPENCLRR: u32 = 0x83c;
pub const RCC_MP_APB5LPENSETR: u32 = 0x840;
pub const RCC_MP_APB5LPENCLRR: u32 = 0x844;
pub const RCC_MP_APB6LPENSETR: u32 = 0x848;
pub const RCC_MP_APB6LPENCLRR: u32 = 0x84c;
pub const RCC_MP_AHB2LPENSETR: u32 = 0x850;
pub const RCC_MP_AHB2LPENCLRR: u32 = 0x854;
pub const RCC_MP_AHB4LPENSETR: u32 = 0x858;
pub const RCC_MP_AHB4LPENCLRR: u32 = 0x85c;
pub const RCC_MP_S_AHB4LPENSETR: u32 = 0x868;
pub const RCC_MP_S_AHB4LPENCLRR: u32 = 0x86c;
pub const RCC_MP_NS_AHB4LPENSETR: u32 = 0x870;
pub const RCC_MP_NS_AHB4LPENCLRR: u32 = 0x874;
pub const RCC_MP_AHB5LPENSETR: u32 = 0x878;
pub const RCC_MP_AHB5LPENCLRR: u32 = 0x87c;
pub const RCC_MP_AHB6LPENSETR: u32 = 0x880;
pub const RCC_MP_AHB6LPENCLRR: u32 = 0x884;
pub const RCC_MP_S_AHB6LPENSETR: u32 = 0x888;
pub const RCC_MP_S_AHB6LPENCLRR: u32 = 0x88c;
pub const RCC_MP_NS_AHB6LPENSETR: u32 = 0x890;
pub const RCC_MP_NS_AHB6LPENCLRR: u32 = 0x894;
pub const RCC_MP_S_AXIMLPENSETR: u32 = 0x898;
pub const RCC_MP_S_AXIMLPENCLRR: u32 = 0x89c;
pub const RCC_MP_NS_AXIMLPENSETR: u32 = 0x8a0;
pub const RCC_MP_NS_AXIMLPENCLRR: u32 = 0x8a4;
pub const RCC_MP_MLAHBLPENSETR: u32 = 0x8a8;
pub const RCC_MP_MLAHBLPENCLRR: u32 = 0x8ac;
pub const RCC_APB3SECSR: u32 = 0x8c0;
pub const RCC_APB4SECSR: u32 = 0x8c4;
pub const RCC_APB5SECSR: u32 = 0x8c8;
pub const RCC_APB6SECSR: u32 = 0x8cc;
pub const RCC_AHB2SECSR: u32 = 0x8d0;
pub const RCC_AHB4SECSR: u32 = 0x8d4;
pub const RCC_AHB5SECSR: u32 = 0x8d8;
pub const RCC_AHB6SECSR: u32 = 0x8dc;
pub const RCC_VERR: u32 = 0xff4;
pub const RCC_IDR: u32 = 0xff8;
pub const RCC_SIDR: u32 = 0xffc;

/* RCC_SECCFGR register fields */
pub const RCC_SECCFGR_HSISEC: u32 = 0;
pub const RCC_SECCFGR_CSISEC: u32 = 1;
pub const RCC_SECCFGR_HSESEC: u32 = 2;
pub const RCC_SECCFGR_LSISEC: u32 = 3;
pub const RCC_SECCFGR_LSESEC: u32 = 4;
pub const RCC_SECCFGR_PLL12SEC: u32 = 8;
pub const RCC_SECCFGR_PLL3SEC: u32 = 9;
pub const RCC_SECCFGR_PLL4SEC: u32 = 10;
pub const RCC_SECCFGR_MPUSEC: u32 = 11;
pub const RCC_SECCFGR_AXISEC: u32 = 12;
pub const RCC_SECCFGR_MLAHBSEC: u32 = 13;
pub const RCC_SECCFGR_APB3DIVSEC: u32 = 16;
pub const RCC_SECCFGR_APB4DIVSEC: u32 = 17;
pub const RCC_SECCFGR_APB5DIVSEC: u32 = 18;
pub const RCC_SECCFGR_APB6DIVSEC: u32 = 19;
pub const RCC_SECCFGR_TIMG3SEC: u32 = 20;
pub const RCC_SECCFGR_CPERSEC: u32 = 21;
pub const RCC_SECCFGR_MCO1SEC: u32 = 22;
pub const RCC_SECCFGR_MCO2SEC: u32 = 23;
pub const RCC_SECCFGR_STPSEC: u32 = 24;
pub const RCC_SECCFGR_RSTSEC: u32 = 25;
pub const RCC_SECCFGR_PWRSEC: u32 = 31;

/* RCC_MP_SREQSETR register fields */
pub const RCC_MP_SREQSETR_STPREQ_P0: u32 = (1u32 << 0);

/* RCC_MP_SREQCLRR register fields */
pub const RCC_MP_SREQCLRR_STPREQ_P0: u32 = (1u32 << 0);

/* RCC_MP_APRSTCR register fields */
pub const RCC_MP_APRSTCR_RDCTLEN: u32 = (1u32 << 0);
pub const RCC_MP_APRSTCR_RSTTO_MASK: u32 = (((1u32 << (14 - 8 + 1)) - 1) << 8);
pub const RCC_MP_APRSTCR_RSTTO_SHIFT: u32 = 8;

/* RCC_MP_APRSTSR register fields */
pub const RCC_MP_APRSTSR_RSTTOV_MASK: u32 = (((1u32 << (14 - 8 + 1)) - 1) << 8);
pub const RCC_MP_APRSTSR_RSTTOV_SHIFT: u32 = 8;

/* RCC_PWRLPDLYCR register fields */
pub const RCC_PWRLPDLYCR_PWRLP_DLY_MASK: u32 = (((1u32 << (21 - 0 + 1)) - 1) << 0);
pub const RCC_PWRLPDLYCR_PWRLP_DLY_SHIFT: u32 = 0;

/* RCC_MP_GRSTCSETR register fields */
pub const RCC_MP_GRSTCSETR_MPSYSRST: u32 = (1u32 << 0);
pub const RCC_MP_GRSTCSETR_MPUP0RST: u32 = (1u32 << 4);

/* RCC_BR_RSTSCLRR register fields */
pub const RCC_BR_RSTSCLRR_PORRSTF: u32 = (1u32 << 0);
pub const RCC_BR_RSTSCLRR_BORRSTF: u32 = (1u32 << 1);
pub const RCC_BR_RSTSCLRR_PADRSTF: u32 = (1u32 << 2);
pub const RCC_BR_RSTSCLRR_HCSSRSTF: u32 = (1u32 << 3);
pub const RCC_BR_RSTSCLRR_VCORERSTF: u32 = (1u32 << 4);
pub const RCC_BR_RSTSCLRR_VCPURSTF: u32 = (1u32 << 5);
pub const RCC_BR_RSTSCLRR_MPSYSRSTF: u32 = (1u32 << 6);
pub const RCC_BR_RSTSCLRR_IWDG1RSTF: u32 = (1u32 << 8);
pub const RCC_BR_RSTSCLRR_IWDG2RSTF: u32 = (1u32 << 9);
pub const RCC_BR_RSTSCLRR_MPUP0RSTF: u32 = (1u32 << 13);

/* RCC_MP_RSTSSETR register fields */
pub const RCC_MP_RSTSSETR_PORRSTF: u32 = (1u32 << 0);
pub const RCC_MP_RSTSSETR_BORRSTF: u32 = (1u32 << 1);
pub const RCC_MP_RSTSSETR_PADRSTF: u32 = (1u32 << 2);
pub const RCC_MP_RSTSSETR_HCSSRSTF: u32 = (1u32 << 3);
pub const RCC_MP_RSTSSETR_VCORERSTF: u32 = (1u32 << 4);
pub const RCC_MP_RSTSSETR_VCPURSTF: u32 = (1u32 << 5);
pub const RCC_MP_RSTSSETR_MPSYSRSTF: u32 = (1u32 << 6);
pub const RCC_MP_RSTSSETR_IWDG1RSTF: u32 = (1u32 << 8);
pub const RCC_MP_RSTSSETR_IWDG2RSTF: u32 = (1u32 << 9);
pub const RCC_MP_RSTSSETR_STP2RSTF: u32 = (1u32 << 10);
pub const RCC_MP_RSTSSETR_STDBYRSTF: u32 = (1u32 << 11);
pub const RCC_MP_RSTSSETR_CSTDBYRSTF: u32 = (1u32 << 12);
pub const RCC_MP_RSTSSETR_MPUP0RSTF: u32 = (1u32 << 13);
pub const RCC_MP_RSTSSETR_SPARE: u32 = (1u32 << 15);

/* RCC_MP_RSTSCLRR register fields */
pub const RCC_MP_RSTSCLRR_PORRSTF: u32 = (1u32 << 0);
pub const RCC_MP_RSTSCLRR_BORRSTF: u32 = (1u32 << 1);
pub const RCC_MP_RSTSCLRR_PADRSTF: u32 = (1u32 << 2);
pub const RCC_MP_RSTSCLRR_HCSSRSTF: u32 = (1u32 << 3);
pub const RCC_MP_RSTSCLRR_VCORERSTF: u32 = (1u32 << 4);
pub const RCC_MP_RSTSCLRR_VCPURSTF: u32 = (1u32 << 5);
pub const RCC_MP_RSTSCLRR_MPSYSRSTF: u32 = (1u32 << 6);
pub const RCC_MP_RSTSCLRR_IWDG1RSTF: u32 = (1u32 << 8);
pub const RCC_MP_RSTSCLRR_IWDG2RSTF: u32 = (1u32 << 9);
pub const RCC_MP_RSTSCLRR_STP2RSTF: u32 = (1u32 << 10);
pub const RCC_MP_RSTSCLRR_STDBYRSTF: u32 = (1u32 << 11);
pub const RCC_MP_RSTSCLRR_CSTDBYRSTF: u32 = (1u32 << 12);
pub const RCC_MP_RSTSCLRR_MPUP0RSTF: u32 = (1u32 << 13);
pub const RCC_MP_RSTSCLRR_SPARE: u32 = (1u32 << 15);

/* RCC_MP_IWDGFZSETR register fields */
pub const RCC_MP_IWDGFZSETR_FZ_IWDG1: u32 = (1u32 << 0);
pub const RCC_MP_IWDGFZSETR_FZ_IWDG2: u32 = (1u32 << 1);

/* RCC_MP_IWDGFZCLRR register fields */
pub const RCC_MP_IWDGFZCLRR_FZ_IWDG1: u32 = (1u32 << 0);
pub const RCC_MP_IWDGFZCLRR_FZ_IWDG2: u32 = (1u32 << 1);

/* RCC_MP_CIER register fields */
pub const RCC_MP_CIER_LSIRDYIE: u32 = (1u32 << 0);
pub const RCC_MP_CIER_LSERDYIE: u32 = (1u32 << 1);
pub const RCC_MP_CIER_HSIRDYIE: u32 = (1u32 << 2);
pub const RCC_MP_CIER_HSERDYIE: u32 = (1u32 << 3);
pub const RCC_MP_CIER_CSIRDYIE: u32 = (1u32 << 4);
pub const RCC_MP_CIER_PLL1DYIE: u32 = (1u32 << 8);
pub const RCC_MP_CIER_PLL2DYIE: u32 = (1u32 << 9);
pub const RCC_MP_CIER_PLL3DYIE: u32 = (1u32 << 10);
pub const RCC_MP_CIER_PLL4DYIE: u32 = (1u32 << 11);
pub const RCC_MP_CIER_LSECSSIE: u32 = (1u32 << 16);
pub const RCC_MP_CIER_WKUPIE: u32 = (1u32 << 20);

/* RCC_MP_CIFR register fields */
pub const RCC_MP_CIFR_LSIRDYF: u32 = (1u32 << 0);
pub const RCC_MP_CIFR_LSERDYF: u32 = (1u32 << 1);
pub const RCC_MP_CIFR_HSIRDYF: u32 = (1u32 << 2);
pub const RCC_MP_CIFR_HSERDYF: u32 = (1u32 << 3);
pub const RCC_MP_CIFR_CSIRDYF: u32 = (1u32 << 4);
pub const RCC_MP_CIFR_PLL1DYF: u32 = (1u32 << 8);
pub const RCC_MP_CIFR_PLL2DYF: u32 = (1u32 << 9);
pub const RCC_MP_CIFR_PLL3DYF: u32 = (1u32 << 10);
pub const RCC_MP_CIFR_PLL4DYF: u32 = (1u32 << 11);
pub const RCC_MP_CIFR_LSECSSF: u32 = (1u32 << 16);
pub const RCC_MP_CIFR_WKUPF: u32 = (1u32 << 20);

/* RCC_BDCR register fields */
pub const RCC_BDCR_LSEON: u32 = (1u32 << 0);
pub const RCC_BDCR_LSEBYP: u32 = (1u32 << 1);
pub const RCC_BDCR_LSERDY: u32 = (1u32 << 2);
pub const RCC_BDCR_DIGBYP: u32 = (1u32 << 3);
pub const RCC_BDCR_LSEDRV_MASK: u32 = (((1u32 << (5 - 4 + 1)) - 1) << 4);
pub const RCC_BDCR_LSECSSON: u32 = (1u32 << 8);
pub const RCC_BDCR_LSECSSD: u32 = (1u32 << 9);
pub const RCC_BDCR_RTCSRC_MASK: u32 = (((1u32 << (17 - 16 + 1)) - 1) << 16);
pub const RCC_BDCR_RTCCKEN: u32 = (1u32 << 20);
pub const RCC_BDCR_VSWRST: u32 = (1u32 << 31);
pub const RCC_BDCR_LSEDRV_SHIFT: u32 = 4;
pub const RCC_BDCR_RTCSRC_SHIFT: u32 = 16;

/* RCC_RDLSICR register fields */
pub const RCC_RDLSICR_LSION: u32 = (1u32 << 0);
pub const RCC_RDLSICR_LSIRDY: u32 = (1u32 << 1);
pub const RCC_RDLSICR_MRD_MASK: u32 = (((1u32 << (20 - 16 + 1)) - 1) << 16);
pub const RCC_RDLSICR_EADLY_MASK: u32 = (((1u32 << (26 - 24 + 1)) - 1) << 24);
pub const RCC_RDLSICR_SPARE_MASK: u32 = (((1u32 << (31 - 27 + 1)) - 1) << 27);
pub const RCC_RDLSICR_MRD_SHIFT: u32 = 16;
pub const RCC_RDLSICR_EADLY_SHIFT: u32 = 24;
pub const RCC_RDLSICR_SPARE_SHIFT: u32 = 27;

/* RCC_OCENSETR register fields */
pub const RCC_OCENSETR_HSION: u32 = (1u32 << 0);
pub const RCC_OCENSETR_HSIKERON: u32 = (1u32 << 1);
pub const RCC_OCENSETR_CSION: u32 = (1u32 << 4);
pub const RCC_OCENSETR_CSIKERON: u32 = (1u32 << 5);
pub const RCC_OCENSETR_DIGBYP: u32 = (1u32 << 7);
pub const RCC_OCENSETR_HSEON: u32 = (1u32 << 8);
pub const RCC_OCENSETR_HSEKERON: u32 = (1u32 << 9);
pub const RCC_OCENSETR_HSEBYP: u32 = (1u32 << 10);
pub const RCC_OCENSETR_HSECSSON: u32 = (1u32 << 11);

/* RCC_OCENCLRR register fields */
pub const RCC_OCENCLRR_HSION: u32 = (1u32 << 0);
pub const RCC_OCENCLRR_HSIKERON: u32 = (1u32 << 1);
pub const RCC_OCENCLRR_CSION: u32 = (1u32 << 4);
pub const RCC_OCENCLRR_CSIKERON: u32 = (1u32 << 5);
pub const RCC_OCENCLRR_DIGBYP: u32 = (1u32 << 7);
pub const RCC_OCENCLRR_HSEON: u32 = (1u32 << 8);
pub const RCC_OCENCLRR_HSEKERON: u32 = (1u32 << 9);
pub const RCC_OCENCLRR_HSEBYP: u32 = (1u32 << 10);

/* RCC_OCRDYR register fields */
pub const RCC_OCRDYR_HSIRDY: u32 = (1u32 << 0);
pub const RCC_OCRDYR_HSIDIVRDY: u32 = (1u32 << 2);
pub const RCC_OCRDYR_CSIRDY: u32 = (1u32 << 4);
pub const RCC_OCRDYR_HSERDY: u32 = (1u32 << 8);
pub const RCC_OCRDYR_MPUCKRDY: u32 = (1u32 << 23);
pub const RCC_OCRDYR_AXICKRDY: u32 = (1u32 << 24);

/* RCC_HSICFGR register fields */
pub const RCC_HSICFGR_HSIDIV_MASK: u32 = (((1u32 << (1 - 0 + 1)) - 1) << 0);
pub const RCC_HSICFGR_HSITRIM_MASK: u32 = (((1u32 << (14 - 8 + 1)) - 1) << 8);
pub const RCC_HSICFGR_HSICAL_MASK: u32 = (((1u32 << (27 - 16 + 1)) - 1) << 16);
pub const RCC_HSICFGR_HSIDIV_SHIFT: u32 = 0;
pub const RCC_HSICFGR_HSITRIM_SHIFT: u32 = 8;
pub const RCC_HSICFGR_HSICAL_SHIFT: u32 = 16;

/* RCC_CSICFGR register fields */
pub const RCC_CSICFGR_CSITRIM_MASK: u32 = (((1u32 << (12 - 8 + 1)) - 1) << 8);
pub const RCC_CSICFGR_CSICAL_MASK: u32 = (((1u32 << (23 - 16 + 1)) - 1) << 16);
pub const RCC_CSICFGR_CSITRIM_SHIFT: u32 = 8;
pub const RCC_CSICFGR_CSICAL_SHIFT: u32 = 16;

/* RCC_MCO1CFGR register fields */
pub const RCC_MCO1CFGR_MCO1SEL_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_MCO1CFGR_MCO1DIV_MASK: u32 = (((1u32 << (7 - 4 + 1)) - 1) << 4);
pub const RCC_MCO1CFGR_MCO1ON: u32 = (1u32 << 12);
pub const RCC_MCO1CFGR_MCO1SEL_SHIFT: u32 = 0;
pub const RCC_MCO1CFGR_MCO1DIV_SHIFT: u32 = 4;

/* RCC_MCO2CFGR register fields */
pub const RCC_MCO2CFGR_MCO2SEL_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_MCO2CFGR_MCO2DIV_MASK: u32 = (((1u32 << (7 - 4 + 1)) - 1) << 4);
pub const RCC_MCO2CFGR_MCO2ON: u32 = (1u32 << 12);
pub const RCC_MCO2CFGR_MCO2SEL_SHIFT: u32 = 0;
pub const RCC_MCO2CFGR_MCO2DIV_SHIFT: u32 = 4;

/* RCC_DBGCFGR register fields */
pub const RCC_DBGCFGR_TRACEDIV_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_DBGCFGR_DBGCKEN: u32 = (1u32 << 8);
pub const RCC_DBGCFGR_TRACECKEN: u32 = (1u32 << 9);
pub const RCC_DBGCFGR_DBGRST: u32 = (1u32 << 12);
pub const RCC_DBGCFGR_TRACEDIV_SHIFT: u32 = 0;

/* RCC_RCK12SELR register fields */
pub const RCC_RCK12SELR_PLL12SRC_MASK: u32 = (((1u32 << (1 - 0 + 1)) - 1) << 0);
pub const RCC_RCK12SELR_PLL12SRCRDY: u32 = (1u32 << 31);
pub const RCC_RCK12SELR_PLL12SRC_SHIFT: u32 = 0;

/* RCC_RCK3SELR register fields */
pub const RCC_RCK3SELR_PLL3SRC_MASK: u32 = (((1u32 << (1 - 0 + 1)) - 1) << 0);
pub const RCC_RCK3SELR_PLL3SRCRDY: u32 = (1u32 << 31);
pub const RCC_RCK3SELR_PLL3SRC_SHIFT: u32 = 0;

/* RCC_RCK4SELR register fields */
pub const RCC_RCK4SELR_PLL4SRC_MASK: u32 = (((1u32 << (1 - 0 + 1)) - 1) << 0);
pub const RCC_RCK4SELR_PLL4SRCRDY: u32 = (1u32 << 31);
pub const RCC_RCK4SELR_PLL4SRC_SHIFT: u32 = 0;

/* RCC_PLL1CR register fields */
pub const RCC_PLL1CR_PLLON: u32 = (1u32 << 0);
pub const RCC_PLL1CR_PLL1RDY: u32 = (1u32 << 1);
pub const RCC_PLL1CR_SSCG_CTRL: u32 = (1u32 << 2);
pub const RCC_PLL1CR_DIVPEN: u32 = (1u32 << 4);
pub const RCC_PLL1CR_DIVQEN: u32 = (1u32 << 5);
pub const RCC_PLL1CR_DIVREN: u32 = (1u32 << 6);

/* RCC_PLL1CFGR1 register fields */
pub const RCC_PLL1CFGR1_DIVN_MASK: u32 = (((1u32 << (8 - 0 + 1)) - 1) << 0);
pub const RCC_PLL1CFGR1_DIVM1_MASK: u32 = (((1u32 << (21 - 16 + 1)) - 1) << 16);
pub const RCC_PLL1CFGR1_DIVN_SHIFT: u32 = 0;
pub const RCC_PLL1CFGR1_DIVM1_SHIFT: u32 = 16;

/* RCC_PLL1CFGR2 register fields */
pub const RCC_PLL1CFGR2_DIVP_MASK: u32 = (((1u32 << (6 - 0 + 1)) - 1) << 0);
pub const RCC_PLL1CFGR2_DIVQ_MASK: u32 = (((1u32 << (14 - 8 + 1)) - 1) << 8);
pub const RCC_PLL1CFGR2_DIVR_MASK: u32 = (((1u32 << (22 - 16 + 1)) - 1) << 16);
pub const RCC_PLL1CFGR2_DIVP_SHIFT: u32 = 0;
pub const RCC_PLL1CFGR2_DIVQ_SHIFT: u32 = 8;
pub const RCC_PLL1CFGR2_DIVR_SHIFT: u32 = 16;

/* RCC_PLL1FRACR register fields */
pub const RCC_PLL1FRACR_FRACV_MASK: u32 = (((1u32 << (15 - 3 + 1)) - 1) << 3);
pub const RCC_PLL1FRACR_FRACLE: u32 = (1u32 << 16);
pub const RCC_PLL1FRACR_FRACV_SHIFT: u32 = 3;

/* RCC_PLL1CSGR register fields */
pub const RCC_PLL1CSGR_MOD_PER_MASK: u32 = (((1u32 << (12 - 0 + 1)) - 1) << 0);
pub const RCC_PLL1CSGR_TPDFN_DIS: u32 = (1u32 << 13);
pub const RCC_PLL1CSGR_RPDFN_DIS: u32 = (1u32 << 14);
pub const RCC_PLL1CSGR_SSCG_MODE: u32 = (1u32 << 15);
pub const RCC_PLL1CSGR_INC_STEP_MASK: u32 = (((1u32 << (30 - 16 + 1)) - 1) << 16);
pub const RCC_PLL1CSGR_MOD_PER_SHIFT: u32 = 0;
pub const RCC_PLL1CSGR_INC_STEP_SHIFT: u32 = 16;

/* RCC_PLL2CR register fields */
pub const RCC_PLL2CR_PLLON: u32 = (1u32 << 0);
pub const RCC_PLL2CR_PLL2RDY: u32 = (1u32 << 1);
pub const RCC_PLL2CR_SSCG_CTRL: u32 = (1u32 << 2);
pub const RCC_PLL2CR_DIVPEN: u32 = (1u32 << 4);
pub const RCC_PLL2CR_DIVQEN: u32 = (1u32 << 5);
pub const RCC_PLL2CR_DIVREN: u32 = (1u32 << 6);

/* RCC_PLL2CFGR1 register fields */
pub const RCC_PLL2CFGR1_DIVN_MASK: u32 = (((1u32 << (8 - 0 + 1)) - 1) << 0);
pub const RCC_PLL2CFGR1_DIVM2_MASK: u32 = (((1u32 << (21 - 16 + 1)) - 1) << 16);
pub const RCC_PLL2CFGR1_DIVN_SHIFT: u32 = 0;
pub const RCC_PLL2CFGR1_DIVM2_SHIFT: u32 = 16;

/* RCC_PLL2CFGR2 register fields */
pub const RCC_PLL2CFGR2_DIVP_MASK: u32 = (((1u32 << (6 - 0 + 1)) - 1) << 0);
pub const RCC_PLL2CFGR2_DIVQ_MASK: u32 = (((1u32 << (14 - 8 + 1)) - 1) << 8);
pub const RCC_PLL2CFGR2_DIVR_MASK: u32 = (((1u32 << (22 - 16 + 1)) - 1) << 16);
pub const RCC_PLL2CFGR2_DIVP_SHIFT: u32 = 0;
pub const RCC_PLL2CFGR2_DIVQ_SHIFT: u32 = 8;
pub const RCC_PLL2CFGR2_DIVR_SHIFT: u32 = 16;

/* RCC_PLL2FRACR register fields */
pub const RCC_PLL2FRACR_FRACV_MASK: u32 = (((1u32 << (15 - 3 + 1)) - 1) << 3);
pub const RCC_PLL2FRACR_FRACLE: u32 = (1u32 << 16);
pub const RCC_PLL2FRACR_FRACV_SHIFT: u32 = 3;

/* RCC_PLL2CSGR register fields */
pub const RCC_PLL2CSGR_MOD_PER_MASK: u32 = (((1u32 << (12 - 0 + 1)) - 1) << 0);
pub const RCC_PLL2CSGR_TPDFN_DIS: u32 = (1u32 << 13);
pub const RCC_PLL2CSGR_RPDFN_DIS: u32 = (1u32 << 14);
pub const RCC_PLL2CSGR_SSCG_MODE: u32 = (1u32 << 15);
pub const RCC_PLL2CSGR_INC_STEP_MASK: u32 = (((1u32 << (30 - 16 + 1)) - 1) << 16);
pub const RCC_PLL2CSGR_MOD_PER_SHIFT: u32 = 0;
pub const RCC_PLL2CSGR_INC_STEP_SHIFT: u32 = 16;

/* RCC_PLL3CR register fields */
pub const RCC_PLL3CR_PLLON: u32 = (1u32 << 0);
pub const RCC_PLL3CR_PLL3RDY: u32 = (1u32 << 1);
pub const RCC_PLL3CR_SSCG_CTRL: u32 = (1u32 << 2);
pub const RCC_PLL3CR_DIVPEN: u32 = (1u32 << 4);
pub const RCC_PLL3CR_DIVQEN: u32 = (1u32 << 5);
pub const RCC_PLL3CR_DIVREN: u32 = (1u32 << 6);

/* RCC_PLL3CFGR1 register fields */
pub const RCC_PLL3CFGR1_DIVN_MASK: u32 = (((1u32 << (8 - 0 + 1)) - 1) << 0);
pub const RCC_PLL3CFGR1_DIVM3_MASK: u32 = (((1u32 << (21 - 16 + 1)) - 1) << 16);
pub const RCC_PLL3CFGR1_IFRGE_MASK: u32 = (((1u32 << (25 - 24 + 1)) - 1) << 24);
pub const RCC_PLL3CFGR1_DIVN_SHIFT: u32 = 0;
pub const RCC_PLL3CFGR1_DIVM3_SHIFT: u32 = 16;
pub const RCC_PLL3CFGR1_IFRGE_SHIFT: u32 = 24;

/* RCC_PLL3CFGR2 register fields */
pub const RCC_PLL3CFGR2_DIVP_MASK: u32 = (((1u32 << (6 - 0 + 1)) - 1) << 0);
pub const RCC_PLL3CFGR2_DIVQ_MASK: u32 = (((1u32 << (14 - 8 + 1)) - 1) << 8);
pub const RCC_PLL3CFGR2_DIVR_MASK: u32 = (((1u32 << (22 - 16 + 1)) - 1) << 16);
pub const RCC_PLL3CFGR2_DIVP_SHIFT: u32 = 0;
pub const RCC_PLL3CFGR2_DIVQ_SHIFT: u32 = 8;
pub const RCC_PLL3CFGR2_DIVR_SHIFT: u32 = 16;

/* RCC_PLL3FRACR register fields */
pub const RCC_PLL3FRACR_FRACV_MASK: u32 = (((1u32 << (15 - 3 + 1)) - 1) << 3);
pub const RCC_PLL3FRACR_FRACLE: u32 = (1u32 << 16);
pub const RCC_PLL3FRACR_FRACV_SHIFT: u32 = 3;

/* RCC_PLL3CSGR register fields */
pub const RCC_PLL3CSGR_MOD_PER_MASK: u32 = (((1u32 << (12 - 0 + 1)) - 1) << 0);
pub const RCC_PLL3CSGR_TPDFN_DIS: u32 = (1u32 << 13);
pub const RCC_PLL3CSGR_RPDFN_DIS: u32 = (1u32 << 14);
pub const RCC_PLL3CSGR_SSCG_MODE: u32 = (1u32 << 15);
pub const RCC_PLL3CSGR_INC_STEP_MASK: u32 = (((1u32 << (30 - 16 + 1)) - 1) << 16);
pub const RCC_PLL3CSGR_MOD_PER_SHIFT: u32 = 0;
pub const RCC_PLL3CSGR_INC_STEP_SHIFT: u32 = 16;

/* RCC_PLL4CR register fields */
pub const RCC_PLL4CR_PLLON: u32 = (1u32 << 0);
pub const RCC_PLL4CR_PLL4RDY: u32 = (1u32 << 1);
pub const RCC_PLL4CR_SSCG_CTRL: u32 = (1u32 << 2);
pub const RCC_PLL4CR_DIVPEN: u32 = (1u32 << 4);
pub const RCC_PLL4CR_DIVQEN: u32 = (1u32 << 5);
pub const RCC_PLL4CR_DIVREN: u32 = (1u32 << 6);

/* RCC_PLL4CFGR1 register fields */
pub const RCC_PLL4CFGR1_DIVN_MASK: u32 = (((1u32 << (8 - 0 + 1)) - 1) << 0);
pub const RCC_PLL4CFGR1_DIVM4_MASK: u32 = (((1u32 << (21 - 16 + 1)) - 1) << 16);
pub const RCC_PLL4CFGR1_IFRGE_MASK: u32 = (((1u32 << (25 - 24 + 1)) - 1) << 24);
pub const RCC_PLL4CFGR1_DIVN_SHIFT: u32 = 0;
pub const RCC_PLL4CFGR1_DIVM4_SHIFT: u32 = 16;
pub const RCC_PLL4CFGR1_IFRGE_SHIFT: u32 = 24;

/* RCC_PLL4CFGR2 register fields */
pub const RCC_PLL4CFGR2_DIVP_MASK: u32 = (((1u32 << (6 - 0 + 1)) - 1) << 0);
pub const RCC_PLL4CFGR2_DIVQ_MASK: u32 = (((1u32 << (14 - 8 + 1)) - 1) << 8);
pub const RCC_PLL4CFGR2_DIVR_MASK: u32 = (((1u32 << (22 - 16 + 1)) - 1) << 16);
pub const RCC_PLL4CFGR2_DIVP_SHIFT: u32 = 0;
pub const RCC_PLL4CFGR2_DIVQ_SHIFT: u32 = 8;
pub const RCC_PLL4CFGR2_DIVR_SHIFT: u32 = 16;

/* RCC_PLL4FRACR register fields */
pub const RCC_PLL4FRACR_FRACV_MASK: u32 = (((1u32 << (15 - 3 + 1)) - 1) << 3);
pub const RCC_PLL4FRACR_FRACLE: u32 = (1u32 << 16);
pub const RCC_PLL4FRACR_FRACV_SHIFT: u32 = 3;

/* RCC_PLL4CSGR register fields */
pub const RCC_PLL4CSGR_MOD_PER_MASK: u32 = (((1u32 << (12 - 0 + 1)) - 1) << 0);
pub const RCC_PLL4CSGR_TPDFN_DIS: u32 = (1u32 << 13);
pub const RCC_PLL4CSGR_RPDFN_DIS: u32 = (1u32 << 14);
pub const RCC_PLL4CSGR_SSCG_MODE: u32 = (1u32 << 15);
pub const RCC_PLL4CSGR_INC_STEP_MASK: u32 = (((1u32 << (30 - 16 + 1)) - 1) << 16);
pub const RCC_PLL4CSGR_MOD_PER_SHIFT: u32 = 0;
pub const RCC_PLL4CSGR_INC_STEP_SHIFT: u32 = 16;

/* RCC_MPCKSELR register fields */
pub const RCC_MPCKSELR_MPUSRC_MASK: u32 = (((1u32 << (1 - 0 + 1)) - 1) << 0);
pub const RCC_MPCKSELR_MPUSRCRDY: u32 = (1u32 << 31);
pub const RCC_MPCKSELR_MPUSRC_SHIFT: u32 = 0;

/* RCC_ASSCKSELR register fields */
pub const RCC_ASSCKSELR_AXISSRC_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_ASSCKSELR_AXISSRCRDY: u32 = (1u32 << 31);
pub const RCC_ASSCKSELR_AXISSRC_SHIFT: u32 = 0;

/* RCC_MSSCKSELR register fields */
pub const RCC_MSSCKSELR_MLAHBSSRC_MASK: u32 = (((1u32 << (1 - 0 + 1)) - 1) << 0);
pub const RCC_MSSCKSELR_MLAHBSSRCRDY: u32 = (1u32 << 31);
pub const RCC_MSSCKSELR_MLAHBSSRC_SHIFT: u32 = 0;

/* RCC_CPERCKSELR register fields */
pub const RCC_CPERCKSELR_CKPERSRC_MASK: u32 = (((1u32 << (1 - 0 + 1)) - 1) << 0);
pub const RCC_CPERCKSELR_CKPERSRC_SHIFT: u32 = 0;

/* RCC_RTCDIVR register fields */
pub const RCC_RTCDIVR_RTCDIV_MASK: u32 = (((1u32 << (5 - 0 + 1)) - 1) << 0);
pub const RCC_RTCDIVR_RTCDIV_SHIFT: u32 = 0;

/* RCC_MPCKDIVR register fields */
pub const RCC_MPCKDIVR_MPUDIV_MASK: u32 = (((1u32 << (3 - 0 + 1)) - 1) << 0);
pub const RCC_MPCKDIVR_MPUDIVRDY: u32 = (1u32 << 31);
pub const RCC_MPCKDIVR_MPUDIV_SHIFT: u32 = 0;

/* RCC_AXIDIVR register fields */
pub const RCC_AXIDIVR_AXIDIV_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_AXIDIVR_AXIDIVRDY: u32 = (1u32 << 31);
pub const RCC_AXIDIVR_AXIDIV_SHIFT: u32 = 0;

/* RCC_MLAHBDIVR register fields */
pub const RCC_MLAHBDIVR_MLAHBDIV_MASK: u32 = (((1u32 << (3 - 0 + 1)) - 1) << 0);
pub const RCC_MLAHBDIVR_MLAHBDIVRDY: u32 = (1u32 << 31);
pub const RCC_MLAHBDIVR_MLAHBDIV_SHIFT: u32 = 0;

/* RCC_APB1DIVR register fields */
pub const RCC_APB1DIVR_APB1DIV_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_APB1DIVR_APB1DIVRDY: u32 = (1u32 << 31);
pub const RCC_APB1DIVR_APB1DIV_SHIFT: u32 = 0;

/* RCC_APB2DIVR register fields */
pub const RCC_APB2DIVR_APB2DIV_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_APB2DIVR_APB2DIVRDY: u32 = (1u32 << 31);
pub const RCC_APB2DIVR_APB2DIV_SHIFT: u32 = 0;

/* RCC_APB3DIVR register fields */
pub const RCC_APB3DIVR_APB3DIV_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_APB3DIVR_APB3DIVRDY: u32 = (1u32 << 31);
pub const RCC_APB3DIVR_APB3DIV_SHIFT: u32 = 0;

/* RCC_APB4DIVR register fields */
pub const RCC_APB4DIVR_APB4DIV_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_APB4DIVR_APB4DIVRDY: u32 = (1u32 << 31);
pub const RCC_APB4DIVR_APB4DIV_SHIFT: u32 = 0;

/* RCC_APB5DIVR register fields */
pub const RCC_APB5DIVR_APB5DIV_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_APB5DIVR_APB5DIVRDY: u32 = (1u32 << 31);
pub const RCC_APB5DIVR_APB5DIV_SHIFT: u32 = 0;

/* RCC_APB6DIVR register fields */
pub const RCC_APB6DIVR_APB6DIV_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_APB6DIVR_APB6DIVRDY: u32 = (1u32 << 31);
pub const RCC_APB6DIVR_APB6DIV_SHIFT: u32 = 0;

/* RCC_TIMG1PRER register fields */
pub const RCC_TIMG1PRER_TIMG1PRE: u32 = (1u32 << 0);
pub const RCC_TIMG1PRER_TIMG1PRERDY: u32 = (1u32 << 31);

/* RCC_TIMG2PRER register fields */
pub const RCC_TIMG2PRER_TIMG2PRE: u32 = (1u32 << 0);
pub const RCC_TIMG2PRER_TIMG2PRERDY: u32 = (1u32 << 31);

/* RCC_TIMG3PRER register fields */
pub const RCC_TIMG3PRER_TIMG3PRE: u32 = (1u32 << 0);
pub const RCC_TIMG3PRER_TIMG3PRERDY: u32 = (1u32 << 31);

/* RCC_DDRITFCR register fields */
pub const RCC_DDRITFCR_DDRC1EN: u32 = (1u32 << 0);
pub const RCC_DDRITFCR_DDRC1LPEN: u32 = (1u32 << 1);
pub const RCC_DDRITFCR_DDRPHYCEN: u32 = (1u32 << 4);
pub const RCC_DDRITFCR_DDRPHYCLPEN: u32 = (1u32 << 5);
pub const RCC_DDRITFCR_DDRCAPBEN: u32 = (1u32 << 6);
pub const RCC_DDRITFCR_DDRCAPBLPEN: u32 = (1u32 << 7);
pub const RCC_DDRITFCR_AXIDCGEN: u32 = (1u32 << 8);
pub const RCC_DDRITFCR_DDRPHYCAPBEN: u32 = (1u32 << 9);
pub const RCC_DDRITFCR_DDRPHYCAPBLPEN: u32 = (1u32 << 10);
pub const RCC_DDRITFCR_KERDCG_DLY_MASK: u32 = (((1u32 << (13 - 11 + 1)) - 1) << 11);
pub const RCC_DDRITFCR_DDRCAPBRST: u32 = (1u32 << 14);
pub const RCC_DDRITFCR_DDRCAXIRST: u32 = (1u32 << 15);
pub const RCC_DDRITFCR_DDRCORERST: u32 = (1u32 << 16);
pub const RCC_DDRITFCR_DPHYAPBRST: u32 = (1u32 << 17);
pub const RCC_DDRITFCR_DPHYRST: u32 = (1u32 << 18);
pub const RCC_DDRITFCR_DPHYCTLRST: u32 = (1u32 << 19);
pub const RCC_DDRITFCR_DDRCKMOD_MASK: u32 = (((1u32 << (22 - 20 + 1)) - 1) << 20);
pub const RCC_DDRITFCR_GSKPMOD: u32 = (1u32 << 23);
pub const RCC_DDRITFCR_GSKPCTRL: u32 = (1u32 << 24);
pub const RCC_DDRITFCR_DFILP_WIDTH_MASK: u32 = (((1u32 << (27 - 25 + 1)) - 1) << 25);
pub const RCC_DDRITFCR_GSKP_DUR_MASK: u32 = (((1u32 << (31 - 28 + 1)) - 1) << 28);
pub const RCC_DDRITFCR_KERDCG_DLY_SHIFT: u32 = 11;
pub const RCC_DDRITFCR_DDRCKMOD_SHIFT: u32 = 20;
pub const RCC_DDRITFCR_DFILP_WIDTH_SHIFT: u32 = 25;
pub const RCC_DDRITFCR_GSKP_DUR_SHIFT: u32 = 28;

/* RCC_I2C12CKSELR register fields */
pub const RCC_I2C12CKSELR_I2C12SRC_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_I2C12CKSELR_I2C12SRC_SHIFT: u32 = 0;

/* RCC_I2C345CKSELR register fields */
pub const RCC_I2C345CKSELR_I2C3SRC_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_I2C345CKSELR_I2C4SRC_MASK: u32 = (((1u32 << (5 - 3 + 1)) - 1) << 3);
pub const RCC_I2C345CKSELR_I2C5SRC_MASK: u32 = (((1u32 << (8 - 6 + 1)) - 1) << 6);
pub const RCC_I2C345CKSELR_I2C3SRC_SHIFT: u32 = 0;
pub const RCC_I2C345CKSELR_I2C4SRC_SHIFT: u32 = 3;
pub const RCC_I2C345CKSELR_I2C5SRC_SHIFT: u32 = 6;

/* RCC_SPI2S1CKSELR register fields */
pub const RCC_SPI2S1CKSELR_SPI1SRC_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_SPI2S1CKSELR_SPI1SRC_SHIFT: u32 = 0;

/* RCC_SPI2S23CKSELR register fields */
pub const RCC_SPI2S23CKSELR_SPI23SRC_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_SPI2S23CKSELR_SPI23SRC_SHIFT: u32 = 0;

/* RCC_SPI45CKSELR register fields */
pub const RCC_SPI45CKSELR_SPI4SRC_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_SPI45CKSELR_SPI5SRC_MASK: u32 = (((1u32 << (5 - 3 + 1)) - 1) << 3);
pub const RCC_SPI45CKSELR_SPI4SRC_SHIFT: u32 = 0;
pub const RCC_SPI45CKSELR_SPI5SRC_SHIFT: u32 = 3;

/* RCC_UART12CKSELR register fields */
pub const RCC_UART12CKSELR_UART1SRC_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_UART12CKSELR_UART2SRC_MASK: u32 = (((1u32 << (5 - 3 + 1)) - 1) << 3);
pub const RCC_UART12CKSELR_UART1SRC_SHIFT: u32 = 0;
pub const RCC_UART12CKSELR_UART2SRC_SHIFT: u32 = 3;

/* RCC_UART35CKSELR register fields */
pub const RCC_UART35CKSELR_UART35SRC_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_UART35CKSELR_UART35SRC_SHIFT: u32 = 0;

/* RCC_UART4CKSELR register fields */
pub const RCC_UART4CKSELR_UART4SRC_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_UART4CKSELR_UART4SRC_SHIFT: u32 = 0;

/* RCC_UART6CKSELR register fields */
pub const RCC_UART6CKSELR_UART6SRC_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_UART6CKSELR_UART6SRC_SHIFT: u32 = 0;

/* RCC_UART78CKSELR register fields */
pub const RCC_UART78CKSELR_UART78SRC_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_UART78CKSELR_UART78SRC_SHIFT: u32 = 0;

/* RCC_LPTIM1CKSELR register fields */
pub const RCC_LPTIM1CKSELR_LPTIM1SRC_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_LPTIM1CKSELR_LPTIM1SRC_SHIFT: u32 = 0;

/* RCC_LPTIM23CKSELR register fields */
pub const RCC_LPTIM23CKSELR_LPTIM2SRC_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_LPTIM23CKSELR_LPTIM3SRC_MASK: u32 = (((1u32 << (5 - 3 + 1)) - 1) << 3);
pub const RCC_LPTIM23CKSELR_LPTIM2SRC_SHIFT: u32 = 0;
pub const RCC_LPTIM23CKSELR_LPTIM3SRC_SHIFT: u32 = 3;

/* RCC_LPTIM45CKSELR register fields */
pub const RCC_LPTIM45CKSELR_LPTIM45SRC_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_LPTIM45CKSELR_LPTIM45SRC_SHIFT: u32 = 0;

/* RCC_SAI1CKSELR register fields */
pub const RCC_SAI1CKSELR_SAI1SRC_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_SAI1CKSELR_SAI1SRC_SHIFT: u32 = 0;

/* RCC_SAI2CKSELR register fields */
pub const RCC_SAI2CKSELR_SAI2SRC_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_SAI2CKSELR_SAI2SRC_SHIFT: u32 = 0;

/* RCC_FDCANCKSELR register fields */
pub const RCC_FDCANCKSELR_FDCANSRC_MASK: u32 = (((1u32 << (1 - 0 + 1)) - 1) << 0);
pub const RCC_FDCANCKSELR_FDCANSRC_SHIFT: u32 = 0;

/* RCC_SPDIFCKSELR register fields */
pub const RCC_SPDIFCKSELR_SPDIFSRC_MASK: u32 = (((1u32 << (1 - 0 + 1)) - 1) << 0);
pub const RCC_SPDIFCKSELR_SPDIFSRC_SHIFT: u32 = 0;

/* RCC_ADC12CKSELR register fields */
pub const RCC_ADC12CKSELR_ADC1SRC_MASK: u32 = (((1u32 << (1 - 0 + 1)) - 1) << 0);
pub const RCC_ADC12CKSELR_ADC2SRC_MASK: u32 = (((1u32 << (3 - 2 + 1)) - 1) << 2);
pub const RCC_ADC12CKSELR_ADC1SRC_SHIFT: u32 = 0;
pub const RCC_ADC12CKSELR_ADC2SRC_SHIFT: u32 = 2;

/* RCC_SDMMC12CKSELR register fields */
pub const RCC_SDMMC12CKSELR_SDMMC1SRC_MASK: u32 = (((1u32 << (2 - 0 + 1)) - 1) << 0);
pub const RCC_SDMMC12CKSELR_SDMMC2SRC_MASK: u32 = (((1u32 << (5 - 3 + 1)) - 1) << 3);
pub const RCC_SDMMC12CKSELR_SDMMC1SRC_SHIFT: u32 = 0;
pub const RCC_SDMMC12CKSELR_SDMMC2SRC_SHIFT: u32 = 3;

/* RCC_ETH12CKSELR register fields */
pub const RCC_ETH12CKSELR_ETH1SRC_MASK: u32 = (((1u32 << (1 - 0 + 1)) - 1) << 0);
pub const RCC_ETH12CKSELR_ETH1PTPDIV_MASK: u32 = (((1u32 << (7 - 4 + 1)) - 1) << 4);
pub const RCC_ETH12CKSELR_ETH2SRC_MASK: u32 = (((1u32 << (9 - 8 + 1)) - 1) << 8);
pub const RCC_ETH12CKSELR_ETH2PTPDIV_MASK: u32 = (((1u32 << (15 - 12 + 1)) - 1) << 12);
pub const RCC_ETH12CKSELR_ETH1SRC_SHIFT: u32 = 0;
pub const RCC_ETH12CKSELR_ETH1PTPDIV_SHIFT: u32 = 4;
pub const RCC_ETH12CKSELR_ETH2SRC_SHIFT: u32 = 8;
pub const RCC_ETH12CKSELR_ETH2PTPDIV_SHIFT: u32 = 12;

/* RCC_USBCKSELR register fields */
pub const RCC_USBCKSELR_USBPHYSRC_MASK: u32 = (((1u32 << (1 - 0 + 1)) - 1) << 0);
pub const RCC_USBCKSELR_USBOSRC: u32 = (1u32 << 4);
pub const RCC_USBCKSELR_USBPHYSRC_SHIFT: u32 = 0;

/* RCC_QSPICKSELR register fields */
pub const RCC_QSPICKSELR_QSPISRC_MASK: u32 = (((1u32 << (1 - 0 + 1)) - 1) << 0);
pub const RCC_QSPICKSELR_QSPISRC_SHIFT: u32 = 0;

/* RCC_FMCCKSELR register fields */
pub const RCC_FMCCKSELR_FMCSRC_MASK: u32 = (((1u32 << (1 - 0 + 1)) - 1) << 0);
pub const RCC_FMCCKSELR_FMCSRC_SHIFT: u32 = 0;

/* RCC_RNG1CKSELR register fields */
pub const RCC_RNG1CKSELR_RNG1SRC_MASK: u32 = (((1u32 << (1 - 0 + 1)) - 1) << 0);
pub const RCC_RNG1CKSELR_RNG1SRC_SHIFT: u32 = 0;

/* RCC_STGENCKSELR register fields */
pub const RCC_STGENCKSELR_STGENSRC_MASK: u32 = (((1u32 << (1 - 0 + 1)) - 1) << 0);
pub const RCC_STGENCKSELR_STGENSRC_SHIFT: u32 = 0;

/* RCC_DCMIPPCKSELR register fields */
pub const RCC_DCMIPPCKSELR_DCMIPPSRC_MASK: u32 = (((1u32 << (1 - 0 + 1)) - 1) << 0);
pub const RCC_DCMIPPCKSELR_DCMIPPSRC_SHIFT: u32 = 0;

/* RCC_SAESCKSELR register fields */
pub const RCC_SAESCKSELR_SAESSRC_MASK: u32 = (((1u32 << (1 - 0 + 1)) - 1) << 0);
pub const RCC_SAESCKSELR_SAESSRC_SHIFT: u32 = 0;

/* RCC_APB1RSTSETR register fields */
pub const RCC_APB1RSTSETR_TIM2RST: u32 = (1u32 << 0);
pub const RCC_APB1RSTSETR_TIM3RST: u32 = (1u32 << 1);
pub const RCC_APB1RSTSETR_TIM4RST: u32 = (1u32 << 2);
pub const RCC_APB1RSTSETR_TIM5RST: u32 = (1u32 << 3);
pub const RCC_APB1RSTSETR_TIM6RST: u32 = (1u32 << 4);
pub const RCC_APB1RSTSETR_TIM7RST: u32 = (1u32 << 5);
pub const RCC_APB1RSTSETR_LPTIM1RST: u32 = (1u32 << 9);
pub const RCC_APB1RSTSETR_SPI2RST: u32 = (1u32 << 11);
pub const RCC_APB1RSTSETR_SPI3RST: u32 = (1u32 << 12);
pub const RCC_APB1RSTSETR_USART3RST: u32 = (1u32 << 15);
pub const RCC_APB1RSTSETR_UART4RST: u32 = (1u32 << 16);
pub const RCC_APB1RSTSETR_UART5RST: u32 = (1u32 << 17);
pub const RCC_APB1RSTSETR_UART7RST: u32 = (1u32 << 18);
pub const RCC_APB1RSTSETR_UART8RST: u32 = (1u32 << 19);
pub const RCC_APB1RSTSETR_I2C1RST: u32 = (1u32 << 21);
pub const RCC_APB1RSTSETR_I2C2RST: u32 = (1u32 << 22);
pub const RCC_APB1RSTSETR_SPDIFRST: u32 = (1u32 << 26);

/* RCC_APB1RSTCLRR register fields */
pub const RCC_APB1RSTCLRR_TIM2RST: u32 = (1u32 << 0);
pub const RCC_APB1RSTCLRR_TIM3RST: u32 = (1u32 << 1);
pub const RCC_APB1RSTCLRR_TIM4RST: u32 = (1u32 << 2);
pub const RCC_APB1RSTCLRR_TIM5RST: u32 = (1u32 << 3);
pub const RCC_APB1RSTCLRR_TIM6RST: u32 = (1u32 << 4);
pub const RCC_APB1RSTCLRR_TIM7RST: u32 = (1u32 << 5);
pub const RCC_APB1RSTCLRR_LPTIM1RST: u32 = (1u32 << 9);
pub const RCC_APB1RSTCLRR_SPI2RST: u32 = (1u32 << 11);
pub const RCC_APB1RSTCLRR_SPI3RST: u32 = (1u32 << 12);
pub const RCC_APB1RSTCLRR_USART3RST: u32 = (1u32 << 15);
pub const RCC_APB1RSTCLRR_UART4RST: u32 = (1u32 << 16);
pub const RCC_APB1RSTCLRR_UART5RST: u32 = (1u32 << 17);
pub const RCC_APB1RSTCLRR_UART7RST: u32 = (1u32 << 18);
pub const RCC_APB1RSTCLRR_UART8RST: u32 = (1u32 << 19);
pub const RCC_APB1RSTCLRR_I2C1RST: u32 = (1u32 << 21);
pub const RCC_APB1RSTCLRR_I2C2RST: u32 = (1u32 << 22);
pub const RCC_APB1RSTCLRR_SPDIFRST: u32 = (1u32 << 26);

/* RCC_APB2RSTSETR register fields */
pub const RCC_APB2RSTSETR_TIM1RST: u32 = (1u32 << 0);
pub const RCC_APB2RSTSETR_TIM8RST: u32 = (1u32 << 1);
pub const RCC_APB2RSTSETR_SPI1RST: u32 = (1u32 << 8);
pub const RCC_APB2RSTSETR_USART6RST: u32 = (1u32 << 13);
pub const RCC_APB2RSTSETR_SAI1RST: u32 = (1u32 << 16);
pub const RCC_APB2RSTSETR_SAI2RST: u32 = (1u32 << 17);
pub const RCC_APB2RSTSETR_DFSDMRST: u32 = (1u32 << 20);
pub const RCC_APB2RSTSETR_FDCANRST: u32 = (1u32 << 24);

/* RCC_APB2RSTCLRR register fields */
pub const RCC_APB2RSTCLRR_TIM1RST: u32 = (1u32 << 0);
pub const RCC_APB2RSTCLRR_TIM8RST: u32 = (1u32 << 1);
pub const RCC_APB2RSTCLRR_SPI1RST: u32 = (1u32 << 8);
pub const RCC_APB2RSTCLRR_USART6RST: u32 = (1u32 << 13);
pub const RCC_APB2RSTCLRR_SAI1RST: u32 = (1u32 << 16);
pub const RCC_APB2RSTCLRR_SAI2RST: u32 = (1u32 << 17);
pub const RCC_APB2RSTCLRR_DFSDMRST: u32 = (1u32 << 20);
pub const RCC_APB2RSTCLRR_FDCANRST: u32 = (1u32 << 24);

/* RCC_APB3RSTSETR register fields */
pub const RCC_APB3RSTSETR_LPTIM2RST: u32 = (1u32 << 0);
pub const RCC_APB3RSTSETR_LPTIM3RST: u32 = (1u32 << 1);
pub const RCC_APB3RSTSETR_LPTIM4RST: u32 = (1u32 << 2);
pub const RCC_APB3RSTSETR_LPTIM5RST: u32 = (1u32 << 3);
pub const RCC_APB3RSTSETR_SYSCFGRST: u32 = (1u32 << 11);
pub const RCC_APB3RSTSETR_VREFRST: u32 = (1u32 << 13);
pub const RCC_APB3RSTSETR_DTSRST: u32 = (1u32 << 16);
pub const RCC_APB3RSTSETR_PMBCTRLRST: u32 = (1u32 << 17);

/* RCC_APB3RSTCLRR register fields */
pub const RCC_APB3RSTCLRR_LPTIM2RST: u32 = (1u32 << 0);
pub const RCC_APB3RSTCLRR_LPTIM3RST: u32 = (1u32 << 1);
pub const RCC_APB3RSTCLRR_LPTIM4RST: u32 = (1u32 << 2);
pub const RCC_APB3RSTCLRR_LPTIM5RST: u32 = (1u32 << 3);
pub const RCC_APB3RSTCLRR_SYSCFGRST: u32 = (1u32 << 11);
pub const RCC_APB3RSTCLRR_VREFRST: u32 = (1u32 << 13);
pub const RCC_APB3RSTCLRR_DTSRST: u32 = (1u32 << 16);
pub const RCC_APB3RSTCLRR_PMBCTRLRST: u32 = (1u32 << 17);

/* RCC_APB4RSTSETR register fields */
pub const RCC_APB4RSTSETR_LTDCRST: u32 = (1u32 << 0);
pub const RCC_APB4RSTSETR_DCMIPPRST: u32 = (1u32 << 1);
pub const RCC_APB4RSTSETR_DDRPERFMRST: u32 = (1u32 << 8);
pub const RCC_APB4RSTSETR_USBPHYRST: u32 = (1u32 << 16);

/* RCC_APB4RSTCLRR register fields */
pub const RCC_APB4RSTCLRR_LTDCRST: u32 = (1u32 << 0);
pub const RCC_APB4RSTCLRR_DCMIPPRST: u32 = (1u32 << 1);
pub const RCC_APB4RSTCLRR_DDRPERFMRST: u32 = (1u32 << 8);
pub const RCC_APB4RSTCLRR_USBPHYRST: u32 = (1u32 << 16);

/* RCC_APB5RSTSETR register fields */
pub const RCC_APB5RSTSETR_STGENRST: u32 = (1u32 << 20);

/* RCC_APB5RSTCLRR register fields */
pub const RCC_APB5RSTCLRR_STGENRST: u32 = (1u32 << 20);

/* RCC_APB6RSTSETR register fields */
pub const RCC_APB6RSTSETR_USART1RST: u32 = (1u32 << 0);
pub const RCC_APB6RSTSETR_USART2RST: u32 = (1u32 << 1);
pub const RCC_APB6RSTSETR_SPI4RST: u32 = (1u32 << 2);
pub const RCC_APB6RSTSETR_SPI5RST: u32 = (1u32 << 3);
pub const RCC_APB6RSTSETR_I2C3RST: u32 = (1u32 << 4);
pub const RCC_APB6RSTSETR_I2C4RST: u32 = (1u32 << 5);
pub const RCC_APB6RSTSETR_I2C5RST: u32 = (1u32 << 6);
pub const RCC_APB6RSTSETR_TIM12RST: u32 = (1u32 << 7);
pub const RCC_APB6RSTSETR_TIM13RST: u32 = (1u32 << 8);
pub const RCC_APB6RSTSETR_TIM14RST: u32 = (1u32 << 9);
pub const RCC_APB6RSTSETR_TIM15RST: u32 = (1u32 << 10);
pub const RCC_APB6RSTSETR_TIM16RST: u32 = (1u32 << 11);
pub const RCC_APB6RSTSETR_TIM17RST: u32 = (1u32 << 12);

/* RCC_APB6RSTCLRR register fields */
pub const RCC_APB6RSTCLRR_USART1RST: u32 = (1u32 << 0);
pub const RCC_APB6RSTCLRR_USART2RST: u32 = (1u32 << 1);
pub const RCC_APB6RSTCLRR_SPI4RST: u32 = (1u32 << 2);
pub const RCC_APB6RSTCLRR_SPI5RST: u32 = (1u32 << 3);
pub const RCC_APB6RSTCLRR_I2C3RST: u32 = (1u32 << 4);
pub const RCC_APB6RSTCLRR_I2C4RST: u32 = (1u32 << 5);
pub const RCC_APB6RSTCLRR_I2C5RST: u32 = (1u32 << 6);
pub const RCC_APB6RSTCLRR_TIM12RST: u32 = (1u32 << 7);
pub const RCC_APB6RSTCLRR_TIM13RST: u32 = (1u32 << 8);
pub const RCC_APB6RSTCLRR_TIM14RST: u32 = (1u32 << 9);
pub const RCC_APB6RSTCLRR_TIM15RST: u32 = (1u32 << 10);
pub const RCC_APB6RSTCLRR_TIM16RST: u32 = (1u32 << 11);
pub const RCC_APB6RSTCLRR_TIM17RST: u32 = (1u32 << 12);

/* RCC_AHB2RSTSETR register fields */
pub const RCC_AHB2RSTSETR_DMA1RST: u32 = (1u32 << 0);
pub const RCC_AHB2RSTSETR_DMA2RST: u32 = (1u32 << 1);
pub const RCC_AHB2RSTSETR_DMAMUX1RST: u32 = (1u32 << 2);
pub const RCC_AHB2RSTSETR_DMA3RST: u32 = (1u32 << 3);
pub const RCC_AHB2RSTSETR_DMAMUX2RST: u32 = (1u32 << 4);
pub const RCC_AHB2RSTSETR_ADC1RST: u32 = (1u32 << 5);
pub const RCC_AHB2RSTSETR_ADC2RST: u32 = (1u32 << 6);
pub const RCC_AHB2RSTSETR_USBORST: u32 = (1u32 << 8);

/* RCC_AHB2RSTCLRR register fields */
pub const RCC_AHB2RSTCLRR_DMA1RST: u32 = (1u32 << 0);
pub const RCC_AHB2RSTCLRR_DMA2RST: u32 = (1u32 << 1);
pub const RCC_AHB2RSTCLRR_DMAMUX1RST: u32 = (1u32 << 2);
pub const RCC_AHB2RSTCLRR_DMA3RST: u32 = (1u32 << 3);
pub const RCC_AHB2RSTCLRR_DMAMUX2RST: u32 = (1u32 << 4);
pub const RCC_AHB2RSTCLRR_ADC1RST: u32 = (1u32 << 5);
pub const RCC_AHB2RSTCLRR_ADC2RST: u32 = (1u32 << 6);
pub const RCC_AHB2RSTCLRR_USBORST: u32 = (1u32 << 8);

/* RCC_AHB4RSTSETR register fields */
pub const RCC_AHB4RSTSETR_GPIOARST: u32 = (1u32 << 0);
pub const RCC_AHB4RSTSETR_GPIOBRST: u32 = (1u32 << 1);
pub const RCC_AHB4RSTSETR_GPIOCRST: u32 = (1u32 << 2);
pub const RCC_AHB4RSTSETR_GPIODRST: u32 = (1u32 << 3);
pub const RCC_AHB4RSTSETR_GPIOERST: u32 = (1u32 << 4);
pub const RCC_AHB4RSTSETR_GPIOFRST: u32 = (1u32 << 5);
pub const RCC_AHB4RSTSETR_GPIOGRST: u32 = (1u32 << 6);
pub const RCC_AHB4RSTSETR_GPIOHRST: u32 = (1u32 << 7);
pub const RCC_AHB4RSTSETR_GPIOIRST: u32 = (1u32 << 8);
pub const RCC_AHB4RSTSETR_TSCRST: u32 = (1u32 << 15);

/* RCC_AHB4RSTCLRR register fields */
pub const RCC_AHB4RSTCLRR_GPIOARST: u32 = (1u32 << 0);
pub const RCC_AHB4RSTCLRR_GPIOBRST: u32 = (1u32 << 1);
pub const RCC_AHB4RSTCLRR_GPIOCRST: u32 = (1u32 << 2);
pub const RCC_AHB4RSTCLRR_GPIODRST: u32 = (1u32 << 3);
pub const RCC_AHB4RSTCLRR_GPIOERST: u32 = (1u32 << 4);
pub const RCC_AHB4RSTCLRR_GPIOFRST: u32 = (1u32 << 5);
pub const RCC_AHB4RSTCLRR_GPIOGRST: u32 = (1u32 << 6);
pub const RCC_AHB4RSTCLRR_GPIOHRST: u32 = (1u32 << 7);
pub const RCC_AHB4RSTCLRR_GPIOIRST: u32 = (1u32 << 8);
pub const RCC_AHB4RSTCLRR_TSCRST: u32 = (1u32 << 15);

/* RCC_AHB5RSTSETR register fields */
pub const RCC_AHB5RSTSETR_PKARST: u32 = (1u32 << 2);
pub const RCC_AHB5RSTSETR_SAESRST: u32 = (1u32 << 3);
pub const RCC_AHB5RSTSETR_CRYP1RST: u32 = (1u32 << 4);
pub const RCC_AHB5RSTSETR_HASH1RST: u32 = (1u32 << 5);
pub const RCC_AHB5RSTSETR_RNG1RST: u32 = (1u32 << 6);
pub const RCC_AHB5RSTSETR_AXIMCRST: u32 = (1u32 << 16);

/* RCC_AHB5RSTCLRR register fields */
pub const RCC_AHB5RSTCLRR_PKARST: u32 = (1u32 << 2);
pub const RCC_AHB5RSTCLRR_SAESRST: u32 = (1u32 << 3);
pub const RCC_AHB5RSTCLRR_CRYP1RST: u32 = (1u32 << 4);
pub const RCC_AHB5RSTCLRR_HASH1RST: u32 = (1u32 << 5);
pub const RCC_AHB5RSTCLRR_RNG1RST: u32 = (1u32 << 6);
pub const RCC_AHB5RSTCLRR_AXIMCRST: u32 = (1u32 << 16);

/* RCC_AHB6RSTSETR register fields */
pub const RCC_AHB6RSTSETR_MDMARST: u32 = (1u32 << 0);
pub const RCC_AHB6RSTSETR_MCERST: u32 = (1u32 << 1);
pub const RCC_AHB6RSTSETR_ETH1MACRST: u32 = (1u32 << 10);
pub const RCC_AHB6RSTSETR_FMCRST: u32 = (1u32 << 12);
pub const RCC_AHB6RSTSETR_QSPIRST: u32 = (1u32 << 14);
pub const RCC_AHB6RSTSETR_SDMMC1RST: u32 = (1u32 << 16);
pub const RCC_AHB6RSTSETR_SDMMC2RST: u32 = (1u32 << 17);
pub const RCC_AHB6RSTSETR_CRC1RST: u32 = (1u32 << 20);
pub const RCC_AHB6RSTSETR_USBHRST: u32 = (1u32 << 24);
pub const RCC_AHB6RSTSETR_ETH2MACRST: u32 = (1u32 << 30);

/* RCC_AHB6RSTCLRR register fields */
pub const RCC_AHB6RSTCLRR_MDMARST: u32 = (1u32 << 0);
pub const RCC_AHB6RSTCLRR_MCERST: u32 = (1u32 << 1);
pub const RCC_AHB6RSTCLRR_ETH1MACRST: u32 = (1u32 << 10);
pub const RCC_AHB6RSTCLRR_FMCRST: u32 = (1u32 << 12);
pub const RCC_AHB6RSTCLRR_QSPIRST: u32 = (1u32 << 14);
pub const RCC_AHB6RSTCLRR_SDMMC1RST: u32 = (1u32 << 16);
pub const RCC_AHB6RSTCLRR_SDMMC2RST: u32 = (1u32 << 17);
pub const RCC_AHB6RSTCLRR_CRC1RST: u32 = (1u32 << 20);
pub const RCC_AHB6RSTCLRR_USBHRST: u32 = (1u32 << 24);
pub const RCC_AHB6RSTCLRR_ETH2MACRST: u32 = (1u32 << 30);

/* RCC_MP_APB1ENSETR register fields */
pub const RCC_MP_APB1ENSETR_TIM2EN: u32 = (1u32 << 0);
pub const RCC_MP_APB1ENSETR_TIM3EN: u32 = (1u32 << 1);
pub const RCC_MP_APB1ENSETR_TIM4EN: u32 = (1u32 << 2);
pub const RCC_MP_APB1ENSETR_TIM5EN: u32 = (1u32 << 3);
pub const RCC_MP_APB1ENSETR_TIM6EN: u32 = (1u32 << 4);
pub const RCC_MP_APB1ENSETR_TIM7EN: u32 = (1u32 << 5);
pub const RCC_MP_APB1ENSETR_LPTIM1EN: u32 = (1u32 << 9);
pub const RCC_MP_APB1ENSETR_SPI2EN: u32 = (1u32 << 11);
pub const RCC_MP_APB1ENSETR_SPI3EN: u32 = (1u32 << 12);
pub const RCC_MP_APB1ENSETR_USART3EN: u32 = (1u32 << 15);
pub const RCC_MP_APB1ENSETR_UART4EN: u32 = (1u32 << 16);
pub const RCC_MP_APB1ENSETR_UART5EN: u32 = (1u32 << 17);
pub const RCC_MP_APB1ENSETR_UART7EN: u32 = (1u32 << 18);
pub const RCC_MP_APB1ENSETR_UART8EN: u32 = (1u32 << 19);
pub const RCC_MP_APB1ENSETR_I2C1EN: u32 = (1u32 << 21);
pub const RCC_MP_APB1ENSETR_I2C2EN: u32 = (1u32 << 22);
pub const RCC_MP_APB1ENSETR_SPDIFEN: u32 = (1u32 << 26);

/* RCC_MP_APB1ENCLRR register fields */
pub const RCC_MP_APB1ENCLRR_TIM2EN: u32 = (1u32 << 0);
pub const RCC_MP_APB1ENCLRR_TIM3EN: u32 = (1u32 << 1);
pub const RCC_MP_APB1ENCLRR_TIM4EN: u32 = (1u32 << 2);
pub const RCC_MP_APB1ENCLRR_TIM5EN: u32 = (1u32 << 3);
pub const RCC_MP_APB1ENCLRR_TIM6EN: u32 = (1u32 << 4);
pub const RCC_MP_APB1ENCLRR_TIM7EN: u32 = (1u32 << 5);
pub const RCC_MP_APB1ENCLRR_LPTIM1EN: u32 = (1u32 << 9);
pub const RCC_MP_APB1ENCLRR_SPI2EN: u32 = (1u32 << 11);
pub const RCC_MP_APB1ENCLRR_SPI3EN: u32 = (1u32 << 12);
pub const RCC_MP_APB1ENCLRR_USART3EN: u32 = (1u32 << 15);
pub const RCC_MP_APB1ENCLRR_UART4EN: u32 = (1u32 << 16);
pub const RCC_MP_APB1ENCLRR_UART5EN: u32 = (1u32 << 17);
pub const RCC_MP_APB1ENCLRR_UART7EN: u32 = (1u32 << 18);
pub const RCC_MP_APB1ENCLRR_UART8EN: u32 = (1u32 << 19);
pub const RCC_MP_APB1ENCLRR_I2C1EN: u32 = (1u32 << 21);
pub const RCC_MP_APB1ENCLRR_I2C2EN: u32 = (1u32 << 22);
pub const RCC_MP_APB1ENCLRR_SPDIFEN: u32 = (1u32 << 26);

/* RCC_MP_APB2ENSETR register fields */
pub const RCC_MP_APB2ENSETR_TIM1EN: u32 = (1u32 << 0);
pub const RCC_MP_APB2ENSETR_TIM8EN: u32 = (1u32 << 1);
pub const RCC_MP_APB2ENSETR_SPI1EN: u32 = (1u32 << 8);
pub const RCC_MP_APB2ENSETR_USART6EN: u32 = (1u32 << 13);
pub const RCC_MP_APB2ENSETR_SAI1EN: u32 = (1u32 << 16);
pub const RCC_MP_APB2ENSETR_SAI2EN: u32 = (1u32 << 17);
pub const RCC_MP_APB2ENSETR_DFSDMEN: u32 = (1u32 << 20);
pub const RCC_MP_APB2ENSETR_ADFSDMEN: u32 = (1u32 << 21);
pub const RCC_MP_APB2ENSETR_FDCANEN: u32 = (1u32 << 24);

/* RCC_MP_APB2ENCLRR register fields */
pub const RCC_MP_APB2ENCLRR_TIM1EN: u32 = (1u32 << 0);
pub const RCC_MP_APB2ENCLRR_TIM8EN: u32 = (1u32 << 1);
pub const RCC_MP_APB2ENCLRR_SPI1EN: u32 = (1u32 << 8);
pub const RCC_MP_APB2ENCLRR_USART6EN: u32 = (1u32 << 13);
pub const RCC_MP_APB2ENCLRR_SAI1EN: u32 = (1u32 << 16);
pub const RCC_MP_APB2ENCLRR_SAI2EN: u32 = (1u32 << 17);
pub const RCC_MP_APB2ENCLRR_DFSDMEN: u32 = (1u32 << 20);
pub const RCC_MP_APB2ENCLRR_ADFSDMEN: u32 = (1u32 << 21);
pub const RCC_MP_APB2ENCLRR_FDCANEN: u32 = (1u32 << 24);

/* RCC_MP_APB3ENSETR register fields */
pub const RCC_MP_APB3ENSETR_LPTIM2EN: u32 = (1u32 << 0);
pub const RCC_MP_APB3ENSETR_LPTIM3EN: u32 = (1u32 << 1);
pub const RCC_MP_APB3ENSETR_LPTIM4EN: u32 = (1u32 << 2);
pub const RCC_MP_APB3ENSETR_LPTIM5EN: u32 = (1u32 << 3);
pub const RCC_MP_APB3ENSETR_VREFEN: u32 = (1u32 << 13);
pub const RCC_MP_APB3ENSETR_DTSEN: u32 = (1u32 << 16);
pub const RCC_MP_APB3ENSETR_PMBCTRLEN: u32 = (1u32 << 17);
pub const RCC_MP_APB3ENSETR_HDPEN: u32 = (1u32 << 20);

/* RCC_MP_APB3ENCLRR register fields */
pub const RCC_MP_APB3ENCLRR_LPTIM2EN: u32 = (1u32 << 0);
pub const RCC_MP_APB3ENCLRR_LPTIM3EN: u32 = (1u32 << 1);
pub const RCC_MP_APB3ENCLRR_LPTIM4EN: u32 = (1u32 << 2);
pub const RCC_MP_APB3ENCLRR_LPTIM5EN: u32 = (1u32 << 3);
pub const RCC_MP_APB3ENCLRR_VREFEN: u32 = (1u32 << 13);
pub const RCC_MP_APB3ENCLRR_DTSEN: u32 = (1u32 << 16);
pub const RCC_MP_APB3ENCLRR_PMBCTRLEN: u32 = (1u32 << 17);
pub const RCC_MP_APB3ENCLRR_HDPEN: u32 = (1u32 << 20);

/* RCC_MP_S_APB3ENSETR register fields */
pub const RCC_MP_S_APB3ENSETR_SYSCFGEN: u32 = (1u32 << 0);

/* RCC_MP_S_APB3ENCLRR register fields */
pub const RCC_MP_S_APB3ENCLRR_SYSCFGEN: u32 = (1u32 << 0);

/* RCC_MP_NS_APB3ENSETR register fields */
pub const RCC_MP_NS_APB3ENSETR_SYSCFGEN: u32 = (1u32 << 0);

/* RCC_MP_NS_APB3ENCLRR register fields */
pub const RCC_MP_NS_APB3ENCLRR_SYSCFGEN: u32 = (1u32 << 0);

/* RCC_MP_APB4ENSETR register fields */
pub const RCC_MP_APB4ENSETR_DCMIPPEN: u32 = (1u32 << 1);
pub const RCC_MP_APB4ENSETR_DDRPERFMEN: u32 = (1u32 << 8);
pub const RCC_MP_APB4ENSETR_IWDG2APBEN: u32 = (1u32 << 15);
pub const RCC_MP_APB4ENSETR_USBPHYEN: u32 = (1u32 << 16);
pub const RCC_MP_APB4ENSETR_STGENROEN: u32 = (1u32 << 20);

/* RCC_MP_APB4ENCLRR register fields */
pub const RCC_MP_APB4ENCLRR_DCMIPPEN: u32 = (1u32 << 1);
pub const RCC_MP_APB4ENCLRR_DDRPERFMEN: u32 = (1u32 << 8);
pub const RCC_MP_APB4ENCLRR_IWDG2APBEN: u32 = (1u32 << 15);
pub const RCC_MP_APB4ENCLRR_USBPHYEN: u32 = (1u32 << 16);
pub const RCC_MP_APB4ENCLRR_STGENROEN: u32 = (1u32 << 20);

/* RCC_MP_S_APB4ENSETR register fields */
pub const RCC_MP_S_APB4ENSETR_LTDCEN: u32 = (1u32 << 0);

/* RCC_MP_S_APB4ENCLRR register fields */
pub const RCC_MP_S_APB4ENCLRR_LTDCEN: u32 = (1u32 << 0);

/* RCC_MP_NS_APB4ENSETR register fields */
pub const RCC_MP_NS_APB4ENSETR_LTDCEN: u32 = (1u32 << 0);

/* RCC_MP_NS_APB4ENCLRR register fields */
pub const RCC_MP_NS_APB4ENCLRR_LTDCEN: u32 = (1u32 << 0);

/* RCC_MP_APB5ENSETR register fields */
pub const RCC_MP_APB5ENSETR_RTCAPBEN: u32 = (1u32 << 8);
pub const RCC_MP_APB5ENSETR_TZCEN: u32 = (1u32 << 11);
pub const RCC_MP_APB5ENSETR_ETZPCEN: u32 = (1u32 << 13);
pub const RCC_MP_APB5ENSETR_IWDG1APBEN: u32 = (1u32 << 15);
pub const RCC_MP_APB5ENSETR_BSECEN: u32 = (1u32 << 16);
pub const RCC_MP_APB5ENSETR_STGENCEN: u32 = (1u32 << 20);

/* RCC_MP_APB5ENCLRR register fields */
pub const RCC_MP_APB5ENCLRR_RTCAPBEN: u32 = (1u32 << 8);
pub const RCC_MP_APB5ENCLRR_TZCEN: u32 = (1u32 << 11);
pub const RCC_MP_APB5ENCLRR_ETZPCEN: u32 = (1u32 << 13);
pub const RCC_MP_APB5ENCLRR_IWDG1APBEN: u32 = (1u32 << 15);
pub const RCC_MP_APB5ENCLRR_BSECEN: u32 = (1u32 << 16);
pub const RCC_MP_APB5ENCLRR_STGENCEN: u32 = (1u32 << 20);

/* RCC_MP_APB6ENSETR register fields */
pub const RCC_MP_APB6ENSETR_USART1EN: u32 = (1u32 << 0);
pub const RCC_MP_APB6ENSETR_USART2EN: u32 = (1u32 << 1);
pub const RCC_MP_APB6ENSETR_SPI4EN: u32 = (1u32 << 2);
pub const RCC_MP_APB6ENSETR_SPI5EN: u32 = (1u32 << 3);
pub const RCC_MP_APB6ENSETR_I2C3EN: u32 = (1u32 << 4);
pub const RCC_MP_APB6ENSETR_I2C4EN: u32 = (1u32 << 5);
pub const RCC_MP_APB6ENSETR_I2C5EN: u32 = (1u32 << 6);
pub const RCC_MP_APB6ENSETR_TIM12EN: u32 = (1u32 << 7);
pub const RCC_MP_APB6ENSETR_TIM13EN: u32 = (1u32 << 8);
pub const RCC_MP_APB6ENSETR_TIM14EN: u32 = (1u32 << 9);
pub const RCC_MP_APB6ENSETR_TIM15EN: u32 = (1u32 << 10);
pub const RCC_MP_APB6ENSETR_TIM16EN: u32 = (1u32 << 11);
pub const RCC_MP_APB6ENSETR_TIM17EN: u32 = (1u32 << 12);

/* RCC_MP_APB6ENCLRR register fields */
pub const RCC_MP_APB6ENCLRR_USART1EN: u32 = (1u32 << 0);
pub const RCC_MP_APB6ENCLRR_USART2EN: u32 = (1u32 << 1);
pub const RCC_MP_APB6ENCLRR_SPI4EN: u32 = (1u32 << 2);
pub const RCC_MP_APB6ENCLRR_SPI5EN: u32 = (1u32 << 3);
pub const RCC_MP_APB6ENCLRR_I2C3EN: u32 = (1u32 << 4);
pub const RCC_MP_APB6ENCLRR_I2C4EN: u32 = (1u32 << 5);
pub const RCC_MP_APB6ENCLRR_I2C5EN: u32 = (1u32 << 6);
pub const RCC_MP_APB6ENCLRR_TIM12EN: u32 = (1u32 << 7);
pub const RCC_MP_APB6ENCLRR_TIM13EN: u32 = (1u32 << 8);
pub const RCC_MP_APB6ENCLRR_TIM14EN: u32 = (1u32 << 9);
pub const RCC_MP_APB6ENCLRR_TIM15EN: u32 = (1u32 << 10);
pub const RCC_MP_APB6ENCLRR_TIM16EN: u32 = (1u32 << 11);
pub const RCC_MP_APB6ENCLRR_TIM17EN: u32 = (1u32 << 12);

/* RCC_MP_AHB2ENSETR register fields */
pub const RCC_MP_AHB2ENSETR_DMA1EN: u32 = (1u32 << 0);
pub const RCC_MP_AHB2ENSETR_DMA2EN: u32 = (1u32 << 1);
pub const RCC_MP_AHB2ENSETR_DMAMUX1EN: u32 = (1u32 << 2);
pub const RCC_MP_AHB2ENSETR_DMA3EN: u32 = (1u32 << 3);
pub const RCC_MP_AHB2ENSETR_DMAMUX2EN: u32 = (1u32 << 4);
pub const RCC_MP_AHB2ENSETR_ADC1EN: u32 = (1u32 << 5);
pub const RCC_MP_AHB2ENSETR_ADC2EN: u32 = (1u32 << 6);
pub const RCC_MP_AHB2ENSETR_USBOEN: u32 = (1u32 << 8);

/* RCC_MP_AHB2ENCLRR register fields */
pub const RCC_MP_AHB2ENCLRR_DMA1EN: u32 = (1u32 << 0);
pub const RCC_MP_AHB2ENCLRR_DMA2EN: u32 = (1u32 << 1);
pub const RCC_MP_AHB2ENCLRR_DMAMUX1EN: u32 = (1u32 << 2);
pub const RCC_MP_AHB2ENCLRR_DMA3EN: u32 = (1u32 << 3);
pub const RCC_MP_AHB2ENCLRR_DMAMUX2EN: u32 = (1u32 << 4);
pub const RCC_MP_AHB2ENCLRR_ADC1EN: u32 = (1u32 << 5);
pub const RCC_MP_AHB2ENCLRR_ADC2EN: u32 = (1u32 << 6);
pub const RCC_MP_AHB2ENCLRR_USBOEN: u32 = (1u32 << 8);

/* RCC_MP_AHB4ENSETR register fields */
pub const RCC_MP_AHB4ENSETR_TSCEN: u32 = (1u32 << 15);

/* RCC_MP_AHB4ENCLRR register fields */
pub const RCC_MP_AHB4ENCLRR_TSCEN: u32 = (1u32 << 15);

/* RCC_MP_S_AHB4ENSETR register fields */
pub const RCC_MP_S_AHB4ENSETR_GPIOAEN: u32 = (1u32 << 0);
pub const RCC_MP_S_AHB4ENSETR_GPIOBEN: u32 = (1u32 << 1);
pub const RCC_MP_S_AHB4ENSETR_GPIOCEN: u32 = (1u32 << 2);
pub const RCC_MP_S_AHB4ENSETR_GPIODEN: u32 = (1u32 << 3);
pub const RCC_MP_S_AHB4ENSETR_GPIOEEN: u32 = (1u32 << 4);
pub const RCC_MP_S_AHB4ENSETR_GPIOFEN: u32 = (1u32 << 5);
pub const RCC_MP_S_AHB4ENSETR_GPIOGEN: u32 = (1u32 << 6);
pub const RCC_MP_S_AHB4ENSETR_GPIOHEN: u32 = (1u32 << 7);
pub const RCC_MP_S_AHB4ENSETR_GPIOIEN: u32 = (1u32 << 8);

/* RCC_MP_S_AHB4ENCLRR register fields */
pub const RCC_MP_S_AHB4ENCLRR_GPIOAEN: u32 = (1u32 << 0);
pub const RCC_MP_S_AHB4ENCLRR_GPIOBEN: u32 = (1u32 << 1);
pub const RCC_MP_S_AHB4ENCLRR_GPIOCEN: u32 = (1u32 << 2);
pub const RCC_MP_S_AHB4ENCLRR_GPIODEN: u32 = (1u32 << 3);
pub const RCC_MP_S_AHB4ENCLRR_GPIOEEN: u32 = (1u32 << 4);
pub const RCC_MP_S_AHB4ENCLRR_GPIOFEN: u32 = (1u32 << 5);
pub const RCC_MP_S_AHB4ENCLRR_GPIOGEN: u32 = (1u32 << 6);
pub const RCC_MP_S_AHB4ENCLRR_GPIOHEN: u32 = (1u32 << 7);
pub const RCC_MP_S_AHB4ENCLRR_GPIOIEN: u32 = (1u32 << 8);

/* RCC_MP_NS_AHB4ENSETR register fields */
pub const RCC_MP_NS_AHB4ENSETR_GPIOAEN: u32 = (1u32 << 0);
pub const RCC_MP_NS_AHB4ENSETR_GPIOBEN: u32 = (1u32 << 1);
pub const RCC_MP_NS_AHB4ENSETR_GPIOCEN: u32 = (1u32 << 2);
pub const RCC_MP_NS_AHB4ENSETR_GPIODEN: u32 = (1u32 << 3);
pub const RCC_MP_NS_AHB4ENSETR_GPIOEEN: u32 = (1u32 << 4);
pub const RCC_MP_NS_AHB4ENSETR_GPIOFEN: u32 = (1u32 << 5);
pub const RCC_MP_NS_AHB4ENSETR_GPIOGEN: u32 = (1u32 << 6);
pub const RCC_MP_NS_AHB4ENSETR_GPIOHEN: u32 = (1u32 << 7);
pub const RCC_MP_NS_AHB4ENSETR_GPIOIEN: u32 = (1u32 << 8);

/* RCC_MP_NS_AHB4ENCLRR register fields */
pub const RCC_MP_NS_AHB4ENCLRR_GPIOAEN: u32 = (1u32 << 0);
pub const RCC_MP_NS_AHB4ENCLRR_GPIOBEN: u32 = (1u32 << 1);
pub const RCC_MP_NS_AHB4ENCLRR_GPIOCEN: u32 = (1u32 << 2);
pub const RCC_MP_NS_AHB4ENCLRR_GPIODEN: u32 = (1u32 << 3);
pub const RCC_MP_NS_AHB4ENCLRR_GPIOEEN: u32 = (1u32 << 4);
pub const RCC_MP_NS_AHB4ENCLRR_GPIOFEN: u32 = (1u32 << 5);
pub const RCC_MP_NS_AHB4ENCLRR_GPIOGEN: u32 = (1u32 << 6);
pub const RCC_MP_NS_AHB4ENCLRR_GPIOHEN: u32 = (1u32 << 7);
pub const RCC_MP_NS_AHB4ENCLRR_GPIOIEN: u32 = (1u32 << 8);

/* RCC_MP_AHB5ENSETR register fields */
pub const RCC_MP_AHB5ENSETR_PKAEN: u32 = (1u32 << 2);
pub const RCC_MP_AHB5ENSETR_SAESEN: u32 = (1u32 << 3);
pub const RCC_MP_AHB5ENSETR_CRYP1EN: u32 = (1u32 << 4);
pub const RCC_MP_AHB5ENSETR_HASH1EN: u32 = (1u32 << 5);
pub const RCC_MP_AHB5ENSETR_RNG1EN: u32 = (1u32 << 6);
pub const RCC_MP_AHB5ENSETR_BKPSRAMEN: u32 = (1u32 << 8);
pub const RCC_MP_AHB5ENSETR_AXIMCEN: u32 = (1u32 << 16);

/* RCC_MP_AHB5ENCLRR register fields */
pub const RCC_MP_AHB5ENCLRR_PKAEN: u32 = (1u32 << 2);
pub const RCC_MP_AHB5ENCLRR_SAESEN: u32 = (1u32 << 3);
pub const RCC_MP_AHB5ENCLRR_CRYP1EN: u32 = (1u32 << 4);
pub const RCC_MP_AHB5ENCLRR_HASH1EN: u32 = (1u32 << 5);
pub const RCC_MP_AHB5ENCLRR_RNG1EN: u32 = (1u32 << 6);
pub const RCC_MP_AHB5ENCLRR_BKPSRAMEN: u32 = (1u32 << 8);
pub const RCC_MP_AHB5ENCLRR_AXIMCEN: u32 = (1u32 << 16);

/* RCC_MP_AHB6ENSETR register fields */
pub const RCC_MP_AHB6ENSETR_MCEEN: u32 = (1u32 << 1);
pub const RCC_MP_AHB6ENSETR_ETH1CKEN: u32 = (1u32 << 7);
pub const RCC_MP_AHB6ENSETR_ETH1TXEN: u32 = (1u32 << 8);
pub const RCC_MP_AHB6ENSETR_ETH1RXEN: u32 = (1u32 << 9);
pub const RCC_MP_AHB6ENSETR_ETH1MACEN: u32 = (1u32 << 10);
pub const RCC_MP_AHB6ENSETR_FMCEN: u32 = (1u32 << 12);
pub const RCC_MP_AHB6ENSETR_QSPIEN: u32 = (1u32 << 14);
pub const RCC_MP_AHB6ENSETR_SDMMC1EN: u32 = (1u32 << 16);
pub const RCC_MP_AHB6ENSETR_SDMMC2EN: u32 = (1u32 << 17);
pub const RCC_MP_AHB6ENSETR_CRC1EN: u32 = (1u32 << 20);
pub const RCC_MP_AHB6ENSETR_USBHEN: u32 = (1u32 << 24);
pub const RCC_MP_AHB6ENSETR_ETH2CKEN: u32 = (1u32 << 27);
pub const RCC_MP_AHB6ENSETR_ETH2TXEN: u32 = (1u32 << 28);
pub const RCC_MP_AHB6ENSETR_ETH2RXEN: u32 = (1u32 << 29);
pub const RCC_MP_AHB6ENSETR_ETH2MACEN: u32 = (1u32 << 30);

/* RCC_MP_AHB6ENCLRR register fields */
pub const RCC_MP_AHB6ENCLRR_MCEEN: u32 = (1u32 << 1);
pub const RCC_MP_AHB6ENCLRR_ETH1CKEN: u32 = (1u32 << 7);
pub const RCC_MP_AHB6ENCLRR_ETH1TXEN: u32 = (1u32 << 8);
pub const RCC_MP_AHB6ENCLRR_ETH1RXEN: u32 = (1u32 << 9);
pub const RCC_MP_AHB6ENCLRR_ETH1MACEN: u32 = (1u32 << 10);
pub const RCC_MP_AHB6ENCLRR_FMCEN: u32 = (1u32 << 12);
pub const RCC_MP_AHB6ENCLRR_QSPIEN: u32 = (1u32 << 14);
pub const RCC_MP_AHB6ENCLRR_SDMMC1EN: u32 = (1u32 << 16);
pub const RCC_MP_AHB6ENCLRR_SDMMC2EN: u32 = (1u32 << 17);
pub const RCC_MP_AHB6ENCLRR_CRC1EN: u32 = (1u32 << 20);
pub const RCC_MP_AHB6ENCLRR_USBHEN: u32 = (1u32 << 24);
pub const RCC_MP_AHB6ENCLRR_ETH2CKEN: u32 = (1u32 << 27);
pub const RCC_MP_AHB6ENCLRR_ETH2TXEN: u32 = (1u32 << 28);
pub const RCC_MP_AHB6ENCLRR_ETH2RXEN: u32 = (1u32 << 29);
pub const RCC_MP_AHB6ENCLRR_ETH2MACEN: u32 = (1u32 << 30);

/* RCC_MP_S_AHB6ENSETR register fields */
pub const RCC_MP_S_AHB6ENSETR_MDMAEN: u32 = (1u32 << 0);

/* RCC_MP_S_AHB6ENCLRR register fields */
pub const RCC_MP_S_AHB6ENCLRR_MDMAEN: u32 = (1u32 << 0);

/* RCC_MP_NS_AHB6ENSETR register fields */
pub const RCC_MP_NS_AHB6ENSETR_MDMAEN: u32 = (1u32 << 0);

/* RCC_MP_NS_AHB6ENCLRR register fields */
pub const RCC_MP_NS_AHB6ENCLRR_MDMAEN: u32 = (1u32 << 0);

/* RCC_MP_APB1LPENSETR register fields */
pub const RCC_MP_APB1LPENSETR_TIM2LPEN: u32 = (1u32 << 0);
pub const RCC_MP_APB1LPENSETR_TIM3LPEN: u32 = (1u32 << 1);
pub const RCC_MP_APB1LPENSETR_TIM4LPEN: u32 = (1u32 << 2);
pub const RCC_MP_APB1LPENSETR_TIM5LPEN: u32 = (1u32 << 3);
pub const RCC_MP_APB1LPENSETR_TIM6LPEN: u32 = (1u32 << 4);
pub const RCC_MP_APB1LPENSETR_TIM7LPEN: u32 = (1u32 << 5);
pub const RCC_MP_APB1LPENSETR_LPTIM1LPEN: u32 = (1u32 << 9);
pub const RCC_MP_APB1LPENSETR_SPI2LPEN: u32 = (1u32 << 11);
pub const RCC_MP_APB1LPENSETR_SPI3LPEN: u32 = (1u32 << 12);
pub const RCC_MP_APB1LPENSETR_USART3LPEN: u32 = (1u32 << 15);
pub const RCC_MP_APB1LPENSETR_UART4LPEN: u32 = (1u32 << 16);
pub const RCC_MP_APB1LPENSETR_UART5LPEN: u32 = (1u32 << 17);
pub const RCC_MP_APB1LPENSETR_UART7LPEN: u32 = (1u32 << 18);
pub const RCC_MP_APB1LPENSETR_UART8LPEN: u32 = (1u32 << 19);
pub const RCC_MP_APB1LPENSETR_I2C1LPEN: u32 = (1u32 << 21);
pub const RCC_MP_APB1LPENSETR_I2C2LPEN: u32 = (1u32 << 22);
pub const RCC_MP_APB1LPENSETR_SPDIFLPEN: u32 = (1u32 << 26);

/* RCC_MP_APB1LPENCLRR register fields */
pub const RCC_MP_APB1LPENCLRR_TIM2LPEN: u32 = (1u32 << 0);
pub const RCC_MP_APB1LPENCLRR_TIM3LPEN: u32 = (1u32 << 1);
pub const RCC_MP_APB1LPENCLRR_TIM4LPEN: u32 = (1u32 << 2);
pub const RCC_MP_APB1LPENCLRR_TIM5LPEN: u32 = (1u32 << 3);
pub const RCC_MP_APB1LPENCLRR_TIM6LPEN: u32 = (1u32 << 4);
pub const RCC_MP_APB1LPENCLRR_TIM7LPEN: u32 = (1u32 << 5);
pub const RCC_MP_APB1LPENCLRR_LPTIM1LPEN: u32 = (1u32 << 9);
pub const RCC_MP_APB1LPENCLRR_SPI2LPEN: u32 = (1u32 << 11);
pub const RCC_MP_APB1LPENCLRR_SPI3LPEN: u32 = (1u32 << 12);
pub const RCC_MP_APB1LPENCLRR_USART3LPEN: u32 = (1u32 << 15);
pub const RCC_MP_APB1LPENCLRR_UART4LPEN: u32 = (1u32 << 16);
pub const RCC_MP_APB1LPENCLRR_UART5LPEN: u32 = (1u32 << 17);
pub const RCC_MP_APB1LPENCLRR_UART7LPEN: u32 = (1u32 << 18);
pub const RCC_MP_APB1LPENCLRR_UART8LPEN: u32 = (1u32 << 19);
pub const RCC_MP_APB1LPENCLRR_I2C1LPEN: u32 = (1u32 << 21);
pub const RCC_MP_APB1LPENCLRR_I2C2LPEN: u32 = (1u32 << 22);
pub const RCC_MP_APB1LPENCLRR_SPDIFLPEN: u32 = (1u32 << 26);

/* RCC_MP_APB2LPENSETR register fields */
pub const RCC_MP_APB2LPENSETR_TIM1LPEN: u32 = (1u32 << 0);
pub const RCC_MP_APB2LPENSETR_TIM8LPEN: u32 = (1u32 << 1);
pub const RCC_MP_APB2LPENSETR_SPI1LPEN: u32 = (1u32 << 8);
pub const RCC_MP_APB2LPENSETR_USART6LPEN: u32 = (1u32 << 13);
pub const RCC_MP_APB2LPENSETR_SAI1LPEN: u32 = (1u32 << 16);
pub const RCC_MP_APB2LPENSETR_SAI2LPEN: u32 = (1u32 << 17);
pub const RCC_MP_APB2LPENSETR_DFSDMLPEN: u32 = (1u32 << 20);
pub const RCC_MP_APB2LPENSETR_ADFSDMLPEN: u32 = (1u32 << 21);
pub const RCC_MP_APB2LPENSETR_FDCANLPEN: u32 = (1u32 << 24);

/* RCC_MP_APB2LPENCLRR register fields */
pub const RCC_MP_APB2LPENCLRR_TIM1LPEN: u32 = (1u32 << 0);
pub const RCC_MP_APB2LPENCLRR_TIM8LPEN: u32 = (1u32 << 1);
pub const RCC_MP_APB2LPENCLRR_SPI1LPEN: u32 = (1u32 << 8);
pub const RCC_MP_APB2LPENCLRR_USART6LPEN: u32 = (1u32 << 13);
pub const RCC_MP_APB2LPENCLRR_SAI1LPEN: u32 = (1u32 << 16);
pub const RCC_MP_APB2LPENCLRR_SAI2LPEN: u32 = (1u32 << 17);
pub const RCC_MP_APB2LPENCLRR_DFSDMLPEN: u32 = (1u32 << 20);
pub const RCC_MP_APB2LPENCLRR_ADFSDMLPEN: u32 = (1u32 << 21);
pub const RCC_MP_APB2LPENCLRR_FDCANLPEN: u32 = (1u32 << 24);

/* RCC_MP_APB3LPENSETR register fields */
pub const RCC_MP_APB3LPENSETR_LPTIM2LPEN: u32 = (1u32 << 0);
pub const RCC_MP_APB3LPENSETR_LPTIM3LPEN: u32 = (1u32 << 1);
pub const RCC_MP_APB3LPENSETR_LPTIM4LPEN: u32 = (1u32 << 2);
pub const RCC_MP_APB3LPENSETR_LPTIM5LPEN: u32 = (1u32 << 3);
pub const RCC_MP_APB3LPENSETR_VREFLPEN: u32 = (1u32 << 13);
pub const RCC_MP_APB3LPENSETR_DTSLPEN: u32 = (1u32 << 16);
pub const RCC_MP_APB3LPENSETR_PMBCTRLLPEN: u32 = (1u32 << 17);

/* RCC_MP_APB3LPENCLRR register fields */
pub const RCC_MP_APB3LPENCLRR_LPTIM2LPEN: u32 = (1u32 << 0);
pub const RCC_MP_APB3LPENCLRR_LPTIM3LPEN: u32 = (1u32 << 1);
pub const RCC_MP_APB3LPENCLRR_LPTIM4LPEN: u32 = (1u32 << 2);
pub const RCC_MP_APB3LPENCLRR_LPTIM5LPEN: u32 = (1u32 << 3);
pub const RCC_MP_APB3LPENCLRR_VREFLPEN: u32 = (1u32 << 13);
pub const RCC_MP_APB3LPENCLRR_DTSLPEN: u32 = (1u32 << 16);
pub const RCC_MP_APB3LPENCLRR_PMBCTRLLPEN: u32 = (1u32 << 17);

/* RCC_MP_S_APB3LPENSETR register fields */
pub const RCC_MP_S_APB3LPENSETR_SYSCFGLPEN: u32 = (1u32 << 0);

/* RCC_MP_S_APB3LPENCLRR register fields */
pub const RCC_MP_S_APB3LPENCLRR_SYSCFGLPEN: u32 = (1u32 << 0);

/* RCC_MP_NS_APB3LPENSETR register fields */
pub const RCC_MP_NS_APB3LPENSETR_SYSCFGLPEN: u32 = (1u32 << 0);

/* RCC_MP_NS_APB3LPENCLRR register fields */
pub const RCC_MP_NS_APB3LPENCLRR_SYSCFGLPEN: u32 = (1u32 << 0);

/* RCC_MP_APB4LPENSETR register fields */
pub const RCC_MP_APB4LPENSETR_DCMIPPLPEN: u32 = (1u32 << 1);
pub const RCC_MP_APB4LPENSETR_DDRPERFMLPEN: u32 = (1u32 << 8);
pub const RCC_MP_APB4LPENSETR_IWDG2APBLPEN: u32 = (1u32 << 15);
pub const RCC_MP_APB4LPENSETR_USBPHYLPEN: u32 = (1u32 << 16);
pub const RCC_MP_APB4LPENSETR_STGENROLPEN: u32 = (1u32 << 20);
pub const RCC_MP_APB4LPENSETR_STGENROSTPEN: u32 = (1u32 << 21);

/* RCC_MP_APB4LPENCLRR register fields */
pub const RCC_MP_APB4LPENCLRR_DCMIPPLPEN: u32 = (1u32 << 1);
pub const RCC_MP_APB4LPENCLRR_DDRPERFMLPEN: u32 = (1u32 << 8);
pub const RCC_MP_APB4LPENCLRR_IWDG2APBLPEN: u32 = (1u32 << 15);
pub const RCC_MP_APB4LPENCLRR_USBPHYLPEN: u32 = (1u32 << 16);
pub const RCC_MP_APB4LPENCLRR_STGENROLPEN: u32 = (1u32 << 20);
pub const RCC_MP_APB4LPENCLRR_STGENROSTPEN: u32 = (1u32 << 21);

/* RCC_MP_S_APB4LPENSETR register fields */
pub const RCC_MP_S_APB4LPENSETR_LTDCLPEN: u32 = (1u32 << 0);

/* RCC_MP_S_APB4LPENCLRR register fields */
pub const RCC_MP_S_APB4LPENCLRR_LTDCLPEN: u32 = (1u32 << 0);

/* RCC_MP_NS_APB4LPENSETR register fields */
pub const RCC_MP_NS_APB4LPENSETR_LTDCLPEN: u32 = (1u32 << 0);

/* RCC_MP_NS_APB4LPENCLRR register fields */
pub const RCC_MP_NS_APB4LPENCLRR_LTDCLPEN: u32 = (1u32 << 0);

/* RCC_MP_APB5LPENSETR register fields */
pub const RCC_MP_APB5LPENSETR_RTCAPBLPEN: u32 = (1u32 << 8);
pub const RCC_MP_APB5LPENSETR_TZCLPEN: u32 = (1u32 << 11);
pub const RCC_MP_APB5LPENSETR_ETZPCLPEN: u32 = (1u32 << 13);
pub const RCC_MP_APB5LPENSETR_IWDG1APBLPEN: u32 = (1u32 << 15);
pub const RCC_MP_APB5LPENSETR_BSECLPEN: u32 = (1u32 << 16);
pub const RCC_MP_APB5LPENSETR_STGENCLPEN: u32 = (1u32 << 20);
pub const RCC_MP_APB5LPENSETR_STGENCSTPEN: u32 = (1u32 << 21);

/* RCC_MP_APB5LPENCLRR register fields */
pub const RCC_MP_APB5LPENCLRR_RTCAPBLPEN: u32 = (1u32 << 8);
pub const RCC_MP_APB5LPENCLRR_TZCLPEN: u32 = (1u32 << 11);
pub const RCC_MP_APB5LPENCLRR_ETZPCLPEN: u32 = (1u32 << 13);
pub const RCC_MP_APB5LPENCLRR_IWDG1APBLPEN: u32 = (1u32 << 15);
pub const RCC_MP_APB5LPENCLRR_BSECLPEN: u32 = (1u32 << 16);
pub const RCC_MP_APB5LPENCLRR_STGENCLPEN: u32 = (1u32 << 20);
pub const RCC_MP_APB5LPENCLRR_STGENCSTPEN: u32 = (1u32 << 21);

/* RCC_MP_APB6LPENSETR register fields */
pub const RCC_MP_APB6LPENSETR_USART1LPEN: u32 = (1u32 << 0);
pub const RCC_MP_APB6LPENSETR_USART2LPEN: u32 = (1u32 << 1);
pub const RCC_MP_APB6LPENSETR_SPI4LPEN: u32 = (1u32 << 2);
pub const RCC_MP_APB6LPENSETR_SPI5LPEN: u32 = (1u32 << 3);
pub const RCC_MP_APB6LPENSETR_I2C3LPEN: u32 = (1u32 << 4);
pub const RCC_MP_APB6LPENSETR_I2C4LPEN: u32 = (1u32 << 5);
pub const RCC_MP_APB6LPENSETR_I2C5LPEN: u32 = (1u32 << 6);
pub const RCC_MP_APB6LPENSETR_TIM12LPEN: u32 = (1u32 << 7);
pub const RCC_MP_APB6LPENSETR_TIM13LPEN: u32 = (1u32 << 8);
pub const RCC_MP_APB6LPENSETR_TIM14LPEN: u32 = (1u32 << 9);
pub const RCC_MP_APB6LPENSETR_TIM15LPEN: u32 = (1u32 << 10);
pub const RCC_MP_APB6LPENSETR_TIM16LPEN: u32 = (1u32 << 11);
pub const RCC_MP_APB6LPENSETR_TIM17LPEN: u32 = (1u32 << 12);

/* RCC_MP_APB6LPENCLRR register fields */
pub const RCC_MP_APB6LPENCLRR_USART1LPEN: u32 = (1u32 << 0);
pub const RCC_MP_APB6LPENCLRR_USART2LPEN: u32 = (1u32 << 1);
pub const RCC_MP_APB6LPENCLRR_SPI4LPEN: u32 = (1u32 << 2);
pub const RCC_MP_APB6LPENCLRR_SPI5LPEN: u32 = (1u32 << 3);
pub const RCC_MP_APB6LPENCLRR_I2C3LPEN: u32 = (1u32 << 4);
pub const RCC_MP_APB6LPENCLRR_I2C4LPEN: u32 = (1u32 << 5);
pub const RCC_MP_APB6LPENCLRR_I2C5LPEN: u32 = (1u32 << 6);
pub const RCC_MP_APB6LPENCLRR_TIM12LPEN: u32 = (1u32 << 7);
pub const RCC_MP_APB6LPENCLRR_TIM13LPEN: u32 = (1u32 << 8);
pub const RCC_MP_APB6LPENCLRR_TIM14LPEN: u32 = (1u32 << 9);
pub const RCC_MP_APB6LPENCLRR_TIM15LPEN: u32 = (1u32 << 10);
pub const RCC_MP_APB6LPENCLRR_TIM16LPEN: u32 = (1u32 << 11);
pub const RCC_MP_APB6LPENCLRR_TIM17LPEN: u32 = (1u32 << 12);

/* RCC_MP_AHB2LPENSETR register fields */
pub const RCC_MP_AHB2LPENSETR_DMA1LPEN: u32 = (1u32 << 0);
pub const RCC_MP_AHB2LPENSETR_DMA2LPEN: u32 = (1u32 << 1);
pub const RCC_MP_AHB2LPENSETR_DMAMUX1LPEN: u32 = (1u32 << 2);
pub const RCC_MP_AHB2LPENSETR_DMA3LPEN: u32 = (1u32 << 3);
pub const RCC_MP_AHB2LPENSETR_DMAMUX2LPEN: u32 = (1u32 << 4);
pub const RCC_MP_AHB2LPENSETR_ADC1LPEN: u32 = (1u32 << 5);
pub const RCC_MP_AHB2LPENSETR_ADC2LPEN: u32 = (1u32 << 6);
pub const RCC_MP_AHB2LPENSETR_USBOLPEN: u32 = (1u32 << 8);

/* RCC_MP_AHB2LPENCLRR register fields */
pub const RCC_MP_AHB2LPENCLRR_DMA1LPEN: u32 = (1u32 << 0);
pub const RCC_MP_AHB2LPENCLRR_DMA2LPEN: u32 = (1u32 << 1);
pub const RCC_MP_AHB2LPENCLRR_DMAMUX1LPEN: u32 = (1u32 << 2);
pub const RCC_MP_AHB2LPENCLRR_DMA3LPEN: u32 = (1u32 << 3);
pub const RCC_MP_AHB2LPENCLRR_DMAMUX2LPEN: u32 = (1u32 << 4);
pub const RCC_MP_AHB2LPENCLRR_ADC1LPEN: u32 = (1u32 << 5);
pub const RCC_MP_AHB2LPENCLRR_ADC2LPEN: u32 = (1u32 << 6);
pub const RCC_MP_AHB2LPENCLRR_USBOLPEN: u32 = (1u32 << 8);

/* RCC_MP_AHB4LPENSETR register fields */
pub const RCC_MP_AHB4LPENSETR_TSCLPEN: u32 = (1u32 << 15);

/* RCC_MP_AHB4LPENCLRR register fields */
pub const RCC_MP_AHB4LPENCLRR_TSCLPEN: u32 = (1u32 << 15);

/* RCC_MP_S_AHB4LPENSETR register fields */
pub const RCC_MP_S_AHB4LPENSETR_GPIOALPEN: u32 = (1u32 << 0);
pub const RCC_MP_S_AHB4LPENSETR_GPIOBLPEN: u32 = (1u32 << 1);
pub const RCC_MP_S_AHB4LPENSETR_GPIOCLPEN: u32 = (1u32 << 2);
pub const RCC_MP_S_AHB4LPENSETR_GPIODLPEN: u32 = (1u32 << 3);
pub const RCC_MP_S_AHB4LPENSETR_GPIOELPEN: u32 = (1u32 << 4);
pub const RCC_MP_S_AHB4LPENSETR_GPIOFLPEN: u32 = (1u32 << 5);
pub const RCC_MP_S_AHB4LPENSETR_GPIOGLPEN: u32 = (1u32 << 6);
pub const RCC_MP_S_AHB4LPENSETR_GPIOHLPEN: u32 = (1u32 << 7);
pub const RCC_MP_S_AHB4LPENSETR_GPIOILPEN: u32 = (1u32 << 8);

/* RCC_MP_S_AHB4LPENCLRR register fields */
pub const RCC_MP_S_AHB4LPENCLRR_GPIOALPEN: u32 = (1u32 << 0);
pub const RCC_MP_S_AHB4LPENCLRR_GPIOBLPEN: u32 = (1u32 << 1);
pub const RCC_MP_S_AHB4LPENCLRR_GPIOCLPEN: u32 = (1u32 << 2);
pub const RCC_MP_S_AHB4LPENCLRR_GPIODLPEN: u32 = (1u32 << 3);
pub const RCC_MP_S_AHB4LPENCLRR_GPIOELPEN: u32 = (1u32 << 4);
pub const RCC_MP_S_AHB4LPENCLRR_GPIOFLPEN: u32 = (1u32 << 5);
pub const RCC_MP_S_AHB4LPENCLRR_GPIOGLPEN: u32 = (1u32 << 6);
pub const RCC_MP_S_AHB4LPENCLRR_GPIOHLPEN: u32 = (1u32 << 7);
pub const RCC_MP_S_AHB4LPENCLRR_GPIOILPEN: u32 = (1u32 << 8);

/* RCC_MP_NS_AHB4LPENSETR register fields */
pub const RCC_MP_NS_AHB4LPENSETR_GPIOALPEN: u32 = (1u32 << 0);
pub const RCC_MP_NS_AHB4LPENSETR_GPIOBLPEN: u32 = (1u32 << 1);
pub const RCC_MP_NS_AHB4LPENSETR_GPIOCLPEN: u32 = (1u32 << 2);
pub const RCC_MP_NS_AHB4LPENSETR_GPIODLPEN: u32 = (1u32 << 3);
pub const RCC_MP_NS_AHB4LPENSETR_GPIOELPEN: u32 = (1u32 << 4);
pub const RCC_MP_NS_AHB4LPENSETR_GPIOFLPEN: u32 = (1u32 << 5);
pub const RCC_MP_NS_AHB4LPENSETR_GPIOGLPEN: u32 = (1u32 << 6);
pub const RCC_MP_NS_AHB4LPENSETR_GPIOHLPEN: u32 = (1u32 << 7);
pub const RCC_MP_NS_AHB4LPENSETR_GPIOILPEN: u32 = (1u32 << 8);

/* RCC_MP_NS_AHB4LPENCLRR register fields */
pub const RCC_MP_NS_AHB4LPENCLRR_GPIOALPEN: u32 = (1u32 << 0);
pub const RCC_MP_NS_AHB4LPENCLRR_GPIOBLPEN: u32 = (1u32 << 1);
pub const RCC_MP_NS_AHB4LPENCLRR_GPIOCLPEN: u32 = (1u32 << 2);
pub const RCC_MP_NS_AHB4LPENCLRR_GPIODLPEN: u32 = (1u32 << 3);
pub const RCC_MP_NS_AHB4LPENCLRR_GPIOELPEN: u32 = (1u32 << 4);
pub const RCC_MP_NS_AHB4LPENCLRR_GPIOFLPEN: u32 = (1u32 << 5);
pub const RCC_MP_NS_AHB4LPENCLRR_GPIOGLPEN: u32 = (1u32 << 6);
pub const RCC_MP_NS_AHB4LPENCLRR_GPIOHLPEN: u32 = (1u32 << 7);
pub const RCC_MP_NS_AHB4LPENCLRR_GPIOILPEN: u32 = (1u32 << 8);

/* RCC_MP_AHB5LPENSETR register fields */
pub const RCC_MP_AHB5LPENSETR_PKALPEN: u32 = (1u32 << 2);
pub const RCC_MP_AHB5LPENSETR_SAESLPEN: u32 = (1u32 << 3);
pub const RCC_MP_AHB5LPENSETR_CRYP1LPEN: u32 = (1u32 << 4);
pub const RCC_MP_AHB5LPENSETR_HASH1LPEN: u32 = (1u32 << 5);
pub const RCC_MP_AHB5LPENSETR_RNG1LPEN: u32 = (1u32 << 6);
pub const RCC_MP_AHB5LPENSETR_BKPSRAMLPEN: u32 = (1u32 << 8);

/* RCC_MP_AHB5LPENCLRR register fields */
pub const RCC_MP_AHB5LPENCLRR_PKALPEN: u32 = (1u32 << 2);
pub const RCC_MP_AHB5LPENCLRR_SAESLPEN: u32 = (1u32 << 3);
pub const RCC_MP_AHB5LPENCLRR_CRYP1LPEN: u32 = (1u32 << 4);
pub const RCC_MP_AHB5LPENCLRR_HASH1LPEN: u32 = (1u32 << 5);
pub const RCC_MP_AHB5LPENCLRR_RNG1LPEN: u32 = (1u32 << 6);
pub const RCC_MP_AHB5LPENCLRR_BKPSRAMLPEN: u32 = (1u32 << 8);

/* RCC_MP_AHB6LPENSETR register fields */
pub const RCC_MP_AHB6LPENSETR_MCELPEN: u32 = (1u32 << 1);
pub const RCC_MP_AHB6LPENSETR_ETH1CKLPEN: u32 = (1u32 << 7);
pub const RCC_MP_AHB6LPENSETR_ETH1TXLPEN: u32 = (1u32 << 8);
pub const RCC_MP_AHB6LPENSETR_ETH1RXLPEN: u32 = (1u32 << 9);
pub const RCC_MP_AHB6LPENSETR_ETH1MACLPEN: u32 = (1u32 << 10);
pub const RCC_MP_AHB6LPENSETR_ETH1STPEN: u32 = (1u32 << 11);
pub const RCC_MP_AHB6LPENSETR_FMCLPEN: u32 = (1u32 << 12);
pub const RCC_MP_AHB6LPENSETR_QSPILPEN: u32 = (1u32 << 14);
pub const RCC_MP_AHB6LPENSETR_SDMMC1LPEN: u32 = (1u32 << 16);
pub const RCC_MP_AHB6LPENSETR_SDMMC2LPEN: u32 = (1u32 << 17);
pub const RCC_MP_AHB6LPENSETR_CRC1LPEN: u32 = (1u32 << 20);
pub const RCC_MP_AHB6LPENSETR_USBHLPEN: u32 = (1u32 << 24);
pub const RCC_MP_AHB6LPENSETR_ETH2CKLPEN: u32 = (1u32 << 27);
pub const RCC_MP_AHB6LPENSETR_ETH2TXLPEN: u32 = (1u32 << 28);
pub const RCC_MP_AHB6LPENSETR_ETH2RXLPEN: u32 = (1u32 << 29);
pub const RCC_MP_AHB6LPENSETR_ETH2MACLPEN: u32 = (1u32 << 30);
pub const RCC_MP_AHB6LPENSETR_ETH2STPEN: u32 = (1u32 << 31);

/* RCC_MP_AHB6LPENCLRR register fields */
pub const RCC_MP_AHB6LPENCLRR_MCELPEN: u32 = (1u32 << 1);
pub const RCC_MP_AHB6LPENCLRR_ETH1CKLPEN: u32 = (1u32 << 7);
pub const RCC_MP_AHB6LPENCLRR_ETH1TXLPEN: u32 = (1u32 << 8);
pub const RCC_MP_AHB6LPENCLRR_ETH1RXLPEN: u32 = (1u32 << 9);
pub const RCC_MP_AHB6LPENCLRR_ETH1MACLPEN: u32 = (1u32 << 10);
pub const RCC_MP_AHB6LPENCLRR_ETH1STPEN: u32 = (1u32 << 11);
pub const RCC_MP_AHB6LPENCLRR_FMCLPEN: u32 = (1u32 << 12);
pub const RCC_MP_AHB6LPENCLRR_QSPILPEN: u32 = (1u32 << 14);
pub const RCC_MP_AHB6LPENCLRR_SDMMC1LPEN: u32 = (1u32 << 16);
pub const RCC_MP_AHB6LPENCLRR_SDMMC2LPEN: u32 = (1u32 << 17);
pub const RCC_MP_AHB6LPENCLRR_CRC1LPEN: u32 = (1u32 << 20);
pub const RCC_MP_AHB6LPENCLRR_USBHLPEN: u32 = (1u32 << 24);
pub const RCC_MP_AHB6LPENCLRR_ETH2CKLPEN: u32 = (1u32 << 27);
pub const RCC_MP_AHB6LPENCLRR_ETH2TXLPEN: u32 = (1u32 << 28);
pub const RCC_MP_AHB6LPENCLRR_ETH2RXLPEN: u32 = (1u32 << 29);
pub const RCC_MP_AHB6LPENCLRR_ETH2MACLPEN: u32 = (1u32 << 30);
pub const RCC_MP_AHB6LPENCLRR_ETH2STPEN: u32 = (1u32 << 31);

/* RCC_MP_S_AHB6LPENSETR register fields */
pub const RCC_MP_S_AHB6LPENSETR_MDMALPEN: u32 = (1u32 << 0);

/* RCC_MP_S_AHB6LPENCLRR register fields */
pub const RCC_MP_S_AHB6LPENCLRR_MDMALPEN: u32 = (1u32 << 0);

/* RCC_MP_NS_AHB6LPENSETR register fields */
pub const RCC_MP_NS_AHB6LPENSETR_MDMALPEN: u32 = (1u32 << 0);

/* RCC_MP_NS_AHB6LPENCLRR register fields */
pub const RCC_MP_NS_AHB6LPENCLRR_MDMALPEN: u32 = (1u32 << 0);

/* RCC_MP_S_AXIMLPENSETR register fields */
pub const RCC_MP_S_AXIMLPENSETR_SYSRAMLPEN: u32 = (1u32 << 0);

/* RCC_MP_S_AXIMLPENCLRR register fields */
pub const RCC_MP_S_AXIMLPENCLRR_SYSRAMLPEN: u32 = (1u32 << 0);

/* RCC_MP_NS_AXIMLPENSETR register fields */
pub const RCC_MP_NS_AXIMLPENSETR_SYSRAMLPEN: u32 = (1u32 << 0);

/* RCC_MP_NS_AXIMLPENCLRR register fields */
pub const RCC_MP_NS_AXIMLPENCLRR_SYSRAMLPEN: u32 = (1u32 << 0);

/* RCC_MP_MLAHBLPENSETR register fields */
pub const RCC_MP_MLAHBLPENSETR_SRAM1LPEN: u32 = (1u32 << 0);
pub const RCC_MP_MLAHBLPENSETR_SRAM2LPEN: u32 = (1u32 << 1);
pub const RCC_MP_MLAHBLPENSETR_SRAM3LPEN: u32 = (1u32 << 2);

/* RCC_MP_MLAHBLPENCLRR register fields */
pub const RCC_MP_MLAHBLPENCLRR_SRAM1LPEN: u32 = (1u32 << 0);
pub const RCC_MP_MLAHBLPENCLRR_SRAM2LPEN: u32 = (1u32 << 1);
pub const RCC_MP_MLAHBLPENCLRR_SRAM3LPEN: u32 = (1u32 << 2);

/* RCC_APB3SECSR register fields */
pub const RCC_APB3SECSR_LPTIM2SECF: u32 = 0;
pub const RCC_APB3SECSR_LPTIM3SECF: u32 = 1;
pub const RCC_APB3SECSR_VREFSECF: u32 = 13;

/* RCC_APB4SECSR register fields */
pub const RCC_APB4SECSR_DCMIPPSECF: u32 = 1;
pub const RCC_APB4SECSR_USBPHYSECF: u32 = 16;

/* RCC_APB5SECSR register fields */
pub const RCC_APB5SECSR_RTCSECF: u32 = 8;
pub const RCC_APB5SECSR_TZCSECF: u32 = 11;
pub const RCC_APB5SECSR_ETZPCSECF: u32 = 13;
pub const RCC_APB5SECSR_IWDG1SECF: u32 = 15;
pub const RCC_APB5SECSR_BSECSECF: u32 = 16;
pub const RCC_APB5SECSR_STGENCSECF_MASK: u32 = (((1u32 << (21 - 20 + 1)) - 1) << 20);
pub const RCC_APB5SECSR_STGENCSECF: u32 = 20;
pub const RCC_APB5SECSR_STGENROSECF: u32 = 21;

/* RCC_APB6SECSR register fields */
pub const RCC_APB6SECSR_USART1SECF: u32 = 0;
pub const RCC_APB6SECSR_USART2SECF: u32 = 1;
pub const RCC_APB6SECSR_SPI4SECF: u32 = 2;
pub const RCC_APB6SECSR_SPI5SECF: u32 = 3;
pub const RCC_APB6SECSR_I2C3SECF: u32 = 4;
pub const RCC_APB6SECSR_I2C4SECF: u32 = 5;
pub const RCC_APB6SECSR_I2C5SECF: u32 = 6;
pub const RCC_APB6SECSR_TIM12SECF: u32 = 7;
pub const RCC_APB6SECSR_TIM13SECF: u32 = 8;
pub const RCC_APB6SECSR_TIM14SECF: u32 = 9;
pub const RCC_APB6SECSR_TIM15SECF: u32 = 10;
pub const RCC_APB6SECSR_TIM16SECF: u32 = 11;
pub const RCC_APB6SECSR_TIM17SECF: u32 = 12;

/* RCC_AHB2SECSR register fields */
pub const RCC_AHB2SECSR_DMA3SECF: u32 = 3;
pub const RCC_AHB2SECSR_DMAMUX2SECF: u32 = 4;
pub const RCC_AHB2SECSR_ADC1SECF: u32 = 5;
pub const RCC_AHB2SECSR_ADC2SECF: u32 = 6;
pub const RCC_AHB2SECSR_USBOSECF: u32 = 8;

/* RCC_AHB4SECSR register fields */
pub const RCC_AHB4SECSR_TSCSECF: u32 = 15;

/* RCC_AHB5SECSR register fields */
pub const RCC_AHB5SECSR_PKASECF: u32 = 2;
pub const RCC_AHB5SECSR_SAESSECF: u32 = 3;
pub const RCC_AHB5SECSR_CRYP1SECF: u32 = 4;
pub const RCC_AHB5SECSR_HASH1SECF: u32 = 5;
pub const RCC_AHB5SECSR_RNG1SECF: u32 = 6;
pub const RCC_AHB5SECSR_BKPSRAMSECF: u32 = 8;

/* RCC_AHB6SECSR register fields */
pub const RCC_AHB6SECSR_MCESECF: u32 = 1;
pub const RCC_AHB6SECSR_FMCSECF: u32 = 12;
pub const RCC_AHB6SECSR_QSPISECF: u32 = 14;
pub const RCC_AHB6SECSR_SDMMC1SECF: u32 = 16;
pub const RCC_AHB6SECSR_SDMMC2SECF: u32 = 17;

pub const RCC_AHB6SECSR_ETH1SECF_MASK: u32 = (((1u32 << (11 - 7 + 1)) - 1) << 7);
pub const RCC_AHB6SECSR_ETH2SECF_MASK: u32 = (((1u32 << (31 - 27 + 1)) - 1) << 27);
pub const RCC_AHB6SECSR_ETH1SECF_SHIFT: u32 = 7;
pub const RCC_AHB6SECSR_ETH2SECF_SHIFT: u32 = 27;

pub const RCC_AHB6SECSR_ETH1CKSECF: u32 = 7;
pub const RCC_AHB6SECSR_ETH1TXSECF: u32 = 8;
pub const RCC_AHB6SECSR_ETH1RXSECF: u32 = 9;
pub const RCC_AHB6SECSR_ETH1MACSECF: u32 = 10;
pub const RCC_AHB6SECSR_ETH1STPSECF: u32 = 11;

pub const RCC_AHB6SECSR_ETH2CKSECF: u32 = 27;
pub const RCC_AHB6SECSR_ETH2TXSECF: u32 = 28;
pub const RCC_AHB6SECSR_ETH2RXSECF: u32 = 29;
pub const RCC_AHB6SECSR_ETH2MACSECF: u32 = 30;
pub const RCC_AHB6SECSR_ETH2STPSECF: u32 = 31;

/* RCC_VERR register fields */
pub const RCC_VERR_MINREV_MASK: u32 = (((1u32 << (3 - 0 + 1)) - 1) << 0);
pub const RCC_VERR_MAJREV_MASK: u32 = (((1u32 << (7 - 4 + 1)) - 1) << 4);
pub const RCC_VERR_MINREV_SHIFT: u32 = 0;
pub const RCC_VERR_MAJREV_SHIFT: u32 = 4;

/* RCC_IDR register fields */
pub const RCC_IDR_ID_MASK: u32 = (((1u32 << (31 - 0 + 1)) - 1) << 0);
pub const RCC_IDR_ID_SHIFT: u32 = 0;

/* RCC_SIDR register fields */
pub const RCC_SIDR_SID_MASK: u32 = (((1u32 << (31 - 0 + 1)) - 1) << 0);
pub const RCC_SIDR_SID_SHIFT: u32 = 0;



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
