/* SPDX-License-Identifier: GPL-2.0
 *
 *  include/asm-sh/cpu-sh3/gpio.h
 *
 *  Copyright (C) 2007  Markus Brunner, Mark Jonas
 *
 *  Addresses for the Pin Function Controller
 */

// Original C conditional: CONFIG_CPU_SUBTYPE_SH7720 || CONFIG_CPU_SUBTYPE_SH7721
#[cfg(any(feature = "CONFIG_CPU_SUBTYPE_SH7720", feature = "CONFIG_CPU_SUBTYPE_SH7721"))]
mod sh7720_sh7721 {
    /* Control registers */
    pub const PORT_PACR: usize = 0xA4050100;
    pub const PORT_PBCR: usize = 0xA4050102;
    pub const PORT_PCCR: usize = 0xA4050104;
    pub const PORT_PDCR: usize = 0xA4050106;
    pub const PORT_PECR: usize = 0xA4050108;
    pub const PORT_PFCR: usize = 0xA405010A;
    pub const PORT_PGCR: usize = 0xA405010C;
    pub const PORT_PHCR: usize = 0xA405010E;
    pub const PORT_PJCR: usize = 0xA4050110;
    pub const PORT_PKCR: usize = 0xA4050112;
    pub const PORT_PLCR: usize = 0xA4050114;
    pub const PORT_PMCR: usize = 0xA4050116;
    pub const PORT_PPCR: usize = 0xA4050118;
    pub const PORT_PRCR: usize = 0xA405011A;
    pub const PORT_PSCR: usize = 0xA405011C;
    pub const PORT_PTCR: usize = 0xA405011E;
    pub const PORT_PUCR: usize = 0xA4050120;
    pub const PORT_PVCR: usize = 0xA4050122;

    /* Data registers */
    pub const PORT_PADR: usize = 0xA4050140;
    // Address of PORT_PBDR is wrong in the datasheet, see errata 2005-09-21
    pub const PORT_PBDR: usize = 0xA4050142;
    pub const PORT_PCDR: usize = 0xA4050144;
    pub const PORT_PDDR: usize = 0xA4050146;
    pub const PORT_PEDR: usize = 0xA4050148;
    pub const PORT_PFDR: usize = 0xA405014A;
    pub const PORT_PGDR: usize = 0xA405014C;
    pub const PORT_PHDR: usize = 0xA405014E;
    pub const PORT_PJDR: usize = 0xA4050150;
    pub const PORT_PKDR: usize = 0xA4050152;
    pub const PORT_PLDR: usize = 0xA4050154;
    pub const PORT_PMDR: usize = 0xA4050156;
    pub const PORT_PPDR: usize = 0xA4050158;
    pub const PORT_PRDR: usize = 0xA405015A;
    pub const PORT_PSDR: usize = 0xA405015C;
    pub const PORT_PTDR: usize = 0xA405015E;
    pub const PORT_PUDR: usize = 0xA4050160;
    pub const PORT_PVDR: usize = 0xA4050162;

    /* Pin Select Registers */
    pub const PORT_PSELA: usize = 0xA4050124;
    pub const PORT_PSELB: usize = 0xA4050126;
    pub const PORT_PSELC: usize = 0xA4050128;
    pub const PORT_PSELD: usize = 0xA405012A;
}

// Original C conditional: CONFIG_CPU_SUBTYPE_SH7709
#[cfg(feature = "CONFIG_CPU_SUBTYPE_SH7709")]
mod sh7709 {
    /* Control registers */
    pub const PORT_PACR: usize = 0xA4000100;
    pub const PORT_PBCR: usize = 0xA4000102;
    pub const PORT_PCCR: usize = 0xA4000104;
    pub const PORT_PFCR: usize = 0xA400010A;

    /* Data registers */
    pub const PORT_PADR: usize = 0xA4000120;
    pub const PORT_PBDR: usize = 0xA4000122;
    pub const PORT_PCDR: usize = 0xA4000124;
    pub const PORT_PFDR: usize = 0xA400012A;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
