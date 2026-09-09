/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This board has 2 physical memory maps.
 * It can be changed with DIP switch(S2-5).
 *
 * phys address                 | S2-5 = OFF   | S2-5 = ON
 * ----------------------------+---------------+---------------
 * 0x00000000 - 0x03ffffff(CS0)| NOR Flash     | NOR Flash
 * 0x04000000 - 0x05ffffff(CS1)| PLD           | PLD
 * 0x06000000 - 0x07ffffff(CS1)| I2C           | I2C
 * 0x08000000 - 0x0bffffff(CS2)| USB           | DDR SDRAM
 * 0x0c000000 - 0x0fffffff(CS3)| SD            | DDR SDRAM
 * 0x10000000 - 0x13ffffff(CS4)| SM107         | SM107
 * 0x14000000 - 0x17ffffff(CS5)| reserved      | USB
 * 0x18000000 - 0x1bffffff(CS6)| reserved      | SD
 * 0x40000000 - 0x5fffffff     | DDR SDRAM     | (cannot use)
 */

pub const NOR_FLASH_ADDR: u32 = 0x00000000;
pub const NOR_FLASH_SIZE: u32 = 0x04000000;

pub const PLD_BASE_ADDR: u32 = 0x04000000;
pub const PLD_PCICR: u32 = PLD_BASE_ADDR + 0x00;
pub const PLD_LCD_BK_CONTR: u32 = PLD_BASE_ADDR + 0x02;
pub const PLD_LOCALCR: u32 = PLD_BASE_ADDR + 0x04;
pub const PLD_POFCR: u32 = PLD_BASE_ADDR + 0x06;
pub const PLD_LEDCR: u32 = PLD_BASE_ADDR + 0x08;
pub const PLD_SWSR: u32 = PLD_BASE_ADDR + 0x0a;
pub const PLD_VERSR: u32 = PLD_BASE_ADDR + 0x0c;
pub const PLD_MMSR: u32 = PLD_BASE_ADDR + 0x0e;

pub const PCA9564_ADDR: u32 = 0x06000000; /* I2C */
pub const PCA9564_SIZE: u32 = 0x00000100;

pub const PCA9564_PROTO_32BIT_ADDR: u32 = 0x14000000;

pub const SM107_MEM_ADDR: u32 = 0x10000000;
pub const SM107_MEM_SIZE: u32 = 0x00e00000;
pub const SM107_REG_ADDR: u32 = 0x13e00000;
pub const SM107_REG_SIZE: u32 = 0x00200000;

/* Preserved from CONFIG_SH_SH7785LCR_29BIT_PHYSMAPS. */
#[cfg(feature = "CONFIG_SH_SH7785LCR_29BIT_PHYSMAPS")]
pub const R8A66597_ADDR: u32 = 0x14000000; /* USB */
#[cfg(feature = "CONFIG_SH_SH7785LCR_29BIT_PHYSMAPS")]
pub const CG200_ADDR: u32 = 0x18000000; /* SD */

#[cfg(not(feature = "CONFIG_SH_SH7785LCR_29BIT_PHYSMAPS"))]
pub const R8A66597_ADDR: u32 = 0x08000000;
#[cfg(not(feature = "CONFIG_SH_SH7785LCR_29BIT_PHYSMAPS"))]
pub const CG200_ADDR: u32 = 0x0c000000;

pub const R8A66597_SIZE: u32 = 0x00000100;
pub const CG200_SIZE: u32 = 0x00010000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
