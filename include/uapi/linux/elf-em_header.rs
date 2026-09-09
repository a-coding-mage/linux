/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* These constants define the various ELF target machines */
pub const EM_NONE: u16 = 0;
pub const EM_M32: u16 = 1;
pub const EM_SPARC: u16 = 2;
pub const EM_386: u16 = 3;
pub const EM_68K: u16 = 4;
pub const EM_88K: u16 = 5;
pub const EM_486: u16 = 6; /* Perhaps disused */
pub const EM_860: u16 = 7;
pub const EM_MIPS: u16 = 8; /* MIPS R3000 (officially, big-endian only) */
/* Next two are historical and binaries and
   modules of these types will be rejected by Linux. */
pub const EM_MIPS_RS3_LE: u16 = 10; /* MIPS R3000 little-endian */
pub const EM_MIPS_RS4_BE: u16 = 10; /* MIPS R4000 big-endian */

pub const EM_PARISC: u16 = 15; /* HPPA */
pub const EM_SPARC32PLUS: u16 = 18; /* Sun's "v8plus" */
pub const EM_PPC: u16 = 20; /* PowerPC */
pub const EM_PPC64: u16 = 21; /* PowerPC64 */
pub const EM_SPU: u16 = 23; /* Cell BE SPU */
pub const EM_ARM: u16 = 40; /* ARM 32 bit */
pub const EM_SH: u16 = 42; /* SuperH */
pub const EM_SPARCV9: u16 = 43; /* SPARC v9 64-bit */
pub const EM_H8_300: u16 = 46; /* Renesas H8/300 */
pub const EM_IA_64: u16 = 50; /* HP/Intel IA-64 */
pub const EM_X86_64: u16 = 62; /* AMD x86-64 */
pub const EM_S390: u16 = 22; /* IBM S/390 */
pub const EM_CRIS: u16 = 76; /* Axis Communications 32-bit embedded processor */
pub const EM_M32R: u16 = 88; /* Renesas M32R */
pub const EM_MN10300: u16 = 89; /* Panasonic/MEI MN10300, AM33 */
pub const EM_OPENRISC: u16 = 92; /* OpenRISC 32-bit embedded processor */
pub const EM_ARCOMPACT: u16 = 93; /* ARCompact processor */
pub const EM_XTENSA: u16 = 94; /* Tensilica Xtensa Architecture */
pub const EM_BLACKFIN: u16 = 106; /* ADI Blackfin Processor */
pub const EM_UNICORE: u16 = 110; /* UniCore-32 */
pub const EM_ALTERA_NIOS2: u16 = 113; /* Altera Nios II soft-core processor */
pub const EM_TI_C6000: u16 = 140; /* TI C6X DSPs */
pub const EM_HEXAGON: u16 = 164; /* QUALCOMM Hexagon */
pub const EM_NDS32: u16 = 167; /* Andes Technology compact code size
                                  embedded RISC processor family */
pub const EM_AARCH64: u16 = 183; /* ARM 64 bit */
pub const EM_TILEPRO: u16 = 188; /* Tilera TILEPro */
pub const EM_MICROBLAZE: u16 = 189; /* Xilinx MicroBlaze */
pub const EM_TILEGX: u16 = 191; /* Tilera TILE-Gx */
pub const EM_ARCV2: u16 = 195; /* ARCv2 Cores */
pub const EM_RISCV: u16 = 243; /* RISC-V */
pub const EM_BPF: u16 = 247; /* Linux BPF - in-kernel virtual machine */
pub const EM_CSKY: u16 = 252; /* C-SKY */
pub const EM_LOONGARCH: u16 = 258; /* LoongArch */
pub const EM_FRV: u16 = 0x5441; /* Fujitsu FR-V */

/*
 * This is an interim value that we will use until the committee comes
 * up with a final number.
 */
pub const EM_ALPHA: u16 = 0x9026;

/* Bogus old m32r magic number, used by old tools. */
pub const EM_CYGNUS_M32R: u16 = 0x9041;
/* This is the old interim value for S/390 architecture */
pub const EM_S390_OLD: u16 = 0xA390;
/* Also Panasonic/MEI MN10300, AM33 */
pub const EM_CYGNUS_MN10300: u16 = 0xbeef;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
