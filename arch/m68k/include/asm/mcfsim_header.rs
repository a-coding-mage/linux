/* SPDX-License-Identifier: GPL-2.0 */
/****************************************************************************/

/*
 *	mcfsim.h -- ColdFire System Integration Module support.
 *
 *	(C) Copyright 1999-2003, Greg Ungerer (gerg@snapgear.com)
 * 	(C) Copyright 2000, Lineo Inc. (www.lineo.com)
 */

/****************************************************************************/

/*
 * Include the appropriate ColdFire CPU specific System Integration Module
 * (SIM) definitions.
 *
 * The original C preprocessor selects the corresponding architecture
 * specific headers below. Those external dependencies are intentionally
 * left to the surrounding Rust build configuration.
 */
// CONFIG_M5206 or CONFIG_M5206e: asm/m5206sim.h and asm/mcfintc.h
// CONFIG_M520x:                 asm/m520xsim.h
// CONFIG_M523x:                 asm/m523xsim.h and asm/mcfintc.h
// CONFIG_M5249 or CONFIG_M525x: asm/m525xsim.h and asm/mcfintc.h
// CONFIG_M527x:                 asm/m527xsim.h
// CONFIG_M5272:                 asm/m5272sim.h
// CONFIG_M528x:                 asm/m528xsim.h
// CONFIG_M5307:                 asm/m5307sim.h and asm/mcfintc.h
// CONFIG_M53xx:                 asm/m53xxsim.h
// CONFIG_M5407:                 asm/m5407sim.h and asm/mcfintc.h
// CONFIG_M54xx:                 asm/m54xxsim.h
// CONFIG_M5441x:                asm/m5441xsim.h


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
