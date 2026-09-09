/* SPDX-License-Identifier: GPL-2.0 */

// If CONFIG_44x is enabled:
// 44x-style software loaded TLB
// Dependency supplied externally: <asm/nohash/32/mmu-44x.h>

// If CONFIG_PPC_E500 is enabled:
// Freescale Book-E software loaded TLB or Book-3e (ISA 2.06+) MMU
// Dependency supplied externally: <asm/nohash/mmu-e500.h>

// If CONFIG_PPC_8xx is enabled:
// Motorola/Freescale 8xx software loaded TLB
// Dependency supplied externally: <asm/nohash/32/mmu-8xx.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
