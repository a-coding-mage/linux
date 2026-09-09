/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* ELF register definitions. */
/* C headers: linux/types.h, asm/ptrace.h, asm/cputable.h, asm/auxvec.h. */

/* PowerPC relocations defined by the ABIs. */
pub const R_PPC_NONE: u32 = 0;
pub const R_PPC_ADDR32: u32 = 1; pub const R_PPC_ADDR24: u32 = 2;
pub const R_PPC_ADDR16: u32 = 3; pub const R_PPC_ADDR16_LO: u32 = 4;
pub const R_PPC_ADDR16_HI: u32 = 5; pub const R_PPC_ADDR16_HA: u32 = 6;
pub const R_PPC_ADDR14: u32 = 7; pub const R_PPC_ADDR14_BRTAKEN: u32 = 8;
pub const R_PPC_ADDR14_BRNTAKEN: u32 = 9; pub const R_PPC_REL24: u32 = 10;
pub const R_PPC_REL14: u32 = 11; pub const R_PPC_REL14_BRTAKEN: u32 = 12;
pub const R_PPC_REL14_BRNTAKEN: u32 = 13; pub const R_PPC_GOT16: u32 = 14;
pub const R_PPC_GOT16_LO: u32 = 15; pub const R_PPC_GOT16_HI: u32 = 16;
pub const R_PPC_GOT16_HA: u32 = 17; pub const R_PPC_PLTREL24: u32 = 18;
pub const R_PPC_COPY: u32 = 19; pub const R_PPC_GLOB_DAT: u32 = 20;
pub const R_PPC_JMP_SLOT: u32 = 21; pub const R_PPC_RELATIVE: u32 = 22;
pub const R_PPC_LOCAL24PC: u32 = 23; pub const R_PPC_UADDR32: u32 = 24;
pub const R_PPC_UADDR16: u32 = 25; pub const R_PPC_REL32: u32 = 26;
pub const R_PPC_PLT32: u32 = 27; pub const R_PPC_PLTREL32: u32 = 28;
pub const R_PPC_PLT16_LO: u32 = 29; pub const R_PPC_PLT16_HI: u32 = 30;
pub const R_PPC_PLT16_HA: u32 = 31; pub const R_PPC_SDAREL16: u32 = 32;
pub const R_PPC_SECTOFF: u32 = 33; pub const R_PPC_SECTOFF_LO: u32 = 34;
pub const R_PPC_SECTOFF_HI: u32 = 35; pub const R_PPC_SECTOFF_HA: u32 = 36;

/* PowerPC TLS relocation ABI. */
pub const R_PPC_TLS: u32 = 67; pub const R_PPC_DTPMOD32: u32 = 68;
pub const R_PPC_TPREL16: u32 = 69; pub const R_PPC_TPREL16_LO: u32 = 70;
pub const R_PPC_TPREL16_HI: u32 = 71; pub const R_PPC_TPREL16_HA: u32 = 72;
pub const R_PPC_TPREL32: u32 = 73; pub const R_PPC_DTPREL16: u32 = 74;
pub const R_PPC_DTPREL16_LO: u32 = 75; pub const R_PPC_DTPREL16_HI: u32 = 76;
pub const R_PPC_DTPREL16_HA: u32 = 77; pub const R_PPC_DTPREL32: u32 = 78;
pub const R_PPC_GOT_TLSGD16: u32 = 79; pub const R_PPC_GOT_TLSGD16_LO: u32 = 80;
pub const R_PPC_GOT_TLSGD16_HI: u32 = 81; pub const R_PPC_GOT_TLSGD16_HA: u32 = 82;
pub const R_PPC_GOT_TLSLD16: u32 = 83; pub const R_PPC_GOT_TLSLD16_LO: u32 = 84;
pub const R_PPC_GOT_TLSLD16_HI: u32 = 85; pub const R_PPC_GOT_TLSLD16_HA: u32 = 86;
pub const R_PPC_GOT_TPREL16: u32 = 87; pub const R_PPC_GOT_TPREL16_LO: u32 = 88;
pub const R_PPC_GOT_TPREL16_HI: u32 = 89; pub const R_PPC_GOT_TPREL16_HA: u32 = 90;
pub const R_PPC_GOT_DTPREL16: u32 = 91; pub const R_PPC_GOT_DTPREL16_LO: u32 = 92;
pub const R_PPC_GOT_DTPREL16_HI: u32 = 93; pub const R_PPC_GOT_DTPREL16_HA: u32 = 94;
pub const R_PPC_NUM: u32 = 95;

pub const ELF_NGREG: usize = 48; pub const ELF_NFPREG: usize = 33;
pub const ELF_NVMX: usize = 34; pub const ELF_NVSX: usize = 32;
pub const ELF_NTMSPRREG: usize = 3; pub const ELF_NEBB: usize = 3;
pub const ELF_NPMU: usize = 5; pub const ELF_NPKEY: usize = 3;
pub const ELF_NDEXCR: usize = 2; pub const ELF_NHASHKEYR: usize = 1;

pub type elf_greg_t64 = c_ulong;
pub type elf_gregset_t64 = [elf_greg_t64; ELF_NGREG];
pub type elf_greg_t32 = c_uint;
pub type elf_gregset_t32 = [elf_greg_t32; ELF_NGREG];
pub type compat_elf_gregset_t = elf_gregset_t32;

/* ELF_ARCH, CLASS, and DATA are build-time ABI parameters. */
#[cfg(target_pointer_width = "64")]
pub const ELF_NVRREG32: usize = 33;
#[cfg(target_pointer_width = "64")]
pub const ELF_NVRREG: usize = 34;
#[cfg(target_pointer_width = "64")]
pub const ELF_NVSRHALFREG: usize = 32;
#[cfg(target_pointer_width = "64")]
pub type ELF_GREG_TYPE = elf_greg_t64;
#[cfg(target_pointer_width = "64")]
pub const ELF_ARCH: u32 = EM_PPC64;
#[cfg(target_pointer_width = "64")]
pub const ELF_CLASS: u32 = ELFCLASS64;
#[cfg(target_pointer_width = "64")]
pub type elf_greg_t = elf_greg_t64;
#[cfg(target_pointer_width = "64")]
pub type elf_gregset_t = elf_gregset_t64;
#[cfg(not(target_pointer_width = "64"))]
pub const ELF_NEVRREG: usize = 34;
#[cfg(not(target_pointer_width = "64"))]
pub const ELF_NVRREG: usize = 33;
#[cfg(not(target_pointer_width = "64"))]
pub const ELF_ARCH: u32 = EM_PPC;
#[cfg(not(target_pointer_width = "64"))]
pub const ELF_CLASS: u32 = ELFCLASS32;
#[cfg(not(target_pointer_width = "64"))]
pub type elf_greg_t = elf_greg_t32;
#[cfg(not(target_pointer_width = "64"))]
pub type elf_gregset_t = elf_gregset_t32;
#[cfg(target_endian = "big")]
pub const ELF_DATA: u32 = ELFDATA2MSB;
#[cfg(target_endian = "little")]
pub const ELF_DATA: u32 = ELFDATA2LSB;

pub type elf_fpreg_t = f64;
pub type elf_fpregset_t = [elf_fpreg_t; ELF_NFPREG];

/* __vector128 is supplied by the PowerPC vector ABI. */
pub type elf_vrreg_t = __vector128;
pub type elf_vrregset_t = [elf_vrreg_t; ELF_NVRREG];
#[cfg(target_pointer_width = "64")]
pub type elf_vrregset_t32 = [elf_vrreg_t; ELF_NVRREG32];
#[cfg(target_pointer_width = "64")]
pub type elf_vsrreghalf_t32 = [elf_fpreg_t; ELF_NVSRHALFREG];

/* PowerPC64 relocations defined by the ABIs. */
pub const R_PPC64_NONE: u32 = R_PPC_NONE; pub const R_PPC64_ADDR32: u32 = R_PPC_ADDR32;
pub const R_PPC64_ADDR24: u32 = R_PPC_ADDR24; pub const R_PPC64_ADDR16: u32 = R_PPC_ADDR16;
pub const R_PPC64_ADDR16_LO: u32 = R_PPC_ADDR16_LO; pub const R_PPC64_ADDR16_HI: u32 = R_PPC_ADDR16_HI;
pub const R_PPC64_ADDR16_HA: u32 = R_PPC_ADDR16_HA; pub const R_PPC64_ADDR14: u32 = R_PPC_ADDR14;
pub const R_PPC64_ADDR14_BRTAKEN: u32 = R_PPC_ADDR14_BRTAKEN; pub const R_PPC64_ADDR14_BRNTAKEN: u32 = R_PPC_ADDR14_BRNTAKEN;
pub const R_PPC64_REL24: u32 = R_PPC_REL24; pub const R_PPC64_REL14: u32 = R_PPC_REL14;
pub const R_PPC64_REL14_BRTAKEN: u32 = R_PPC_REL14_BRTAKEN; pub const R_PPC64_REL14_BRNTAKEN: u32 = R_PPC_REL14_BRNTAKEN;
pub const R_PPC64_GOT16: u32 = R_PPC_GOT16; pub const R_PPC64_GOT16_LO: u32 = R_PPC_GOT16_LO;
pub const R_PPC64_GOT16_HI: u32 = R_PPC_GOT16_HI; pub const R_PPC64_GOT16_HA: u32 = R_PPC_GOT16_HA;
pub const R_PPC64_COPY: u32 = R_PPC_COPY; pub const R_PPC64_GLOB_DAT: u32 = R_PPC_GLOB_DAT;
pub const R_PPC64_JMP_SLOT: u32 = R_PPC_JMP_SLOT; pub const R_PPC64_RELATIVE: u32 = R_PPC_RELATIVE;
pub const R_PPC64_UADDR32: u32 = R_PPC_UADDR32; pub const R_PPC64_UADDR16: u32 = R_PPC_UADDR16;
pub const R_PPC64_REL32: u32 = R_PPC_REL32; pub const R_PPC64_PLT32: u32 = R_PPC_PLT32;
pub const R_PPC64_PLTREL32: u32 = R_PPC_PLTREL32; pub const R_PPC64_PLT16_LO: u32 = R_PPC_PLT16_LO;
pub const R_PPC64_PLT16_HI: u32 = R_PPC_PLT16_HI; pub const R_PPC64_PLT16_HA: u32 = R_PPC_PLT16_HA;
pub const R_PPC64_SECTOFF: u32 = R_PPC_SECTOFF; pub const R_PPC64_SECTOFF_LO: u32 = R_PPC_SECTOFF_LO;
pub const R_PPC64_SECTOFF_HI: u32 = R_PPC_SECTOFF_HI; pub const R_PPC64_SECTOFF_HA: u32 = R_PPC_SECTOFF_HA;
pub const R_PPC64_ADDR30: u32 = 37; pub const R_PPC64_ADDR64: u32 = 38;
pub const R_PPC64_ADDR16_HIGHER: u32 = 39; pub const R_PPC64_ADDR16_HIGHERA: u32 = 40;
pub const R_PPC64_ADDR16_HIGHEST: u32 = 41; pub const R_PPC64_ADDR16_HIGHESTA: u32 = 42;
pub const R_PPC64_UADDR64: u32 = 43; pub const R_PPC64_REL64: u32 = 44;
pub const R_PPC64_PLT64: u32 = 45; pub const R_PPC64_PLTREL64: u32 = 46;
pub const R_PPC64_TOC16: u32 = 47; pub const R_PPC64_TOC16_LO: u32 = 48;
pub const R_PPC64_TOC16_HI: u32 = 49; pub const R_PPC64_TOC16_HA: u32 = 50;
pub const R_PPC64_TOC: u32 = 51; pub const R_PPC64_PLTGOT16: u32 = 52;
pub const R_PPC64_PLTGOT16_LO: u32 = 53; pub const R_PPC64_PLTGOT16_HI: u32 = 54;
pub const R_PPC64_PLTGOT16_HA: u32 = 55; pub const R_PPC64_ADDR16_DS: u32 = 56;
pub const R_PPC64_ADDR16_LO_DS: u32 = 57; pub const R_PPC64_GOT16_DS: u32 = 58;
pub const R_PPC64_GOT16_LO_DS: u32 = 59; pub const R_PPC64_PLT16_LO_DS: u32 = 60;
pub const R_PPC64_SECTOFF_DS: u32 = 61; pub const R_PPC64_SECTOFF_LO_DS: u32 = 62;
pub const R_PPC64_TOC16_DS: u32 = 63; pub const R_PPC64_TOC16_LO_DS: u32 = 64;
pub const R_PPC64_PLTGOT16_DS: u32 = 65; pub const R_PPC64_PLTGOT16_LO_DS: u32 = 66;

pub const R_PPC64_TLS: u32 = 67; pub const R_PPC64_DTPMOD64: u32 = 68;
pub const R_PPC64_TPREL16: u32 = 69; pub const R_PPC64_TPREL16_LO: u32 = 70;
pub const R_PPC64_TPREL16_HI: u32 = 71; pub const R_PPC64_TPREL16_HA: u32 = 72;
pub const R_PPC64_TPREL64: u32 = 73; pub const R_PPC64_DTPREL16: u32 = 74;
pub const R_PPC64_DTPREL16_LO: u32 = 75; pub const R_PPC64_DTPREL16_HI: u32 = 76;
pub const R_PPC64_DTPREL16_HA: u32 = 77; pub const R_PPC64_DTPREL64: u32 = 78;
pub const R_PPC64_GOT_TLSGD16: u32 = 79; pub const R_PPC64_GOT_TLSGD16_LO: u32 = 80;
pub const R_PPC64_GOT_TLSGD16_HI: u32 = 81; pub const R_PPC64_GOT_TLSGD16_HA: u32 = 82;
pub const R_PPC64_GOT_TLSLD16: u32 = 83; pub const R_PPC64_GOT_TLSLD16_LO: u32 = 84;
pub const R_PPC64_GOT_TLSLD16_HI: u32 = 85; pub const R_PPC64_GOT_TLSLD16_HA: u32 = 86;
pub const R_PPC64_GOT_TPREL16_DS: u32 = 87; pub const R_PPC64_GOT_TPREL16_LO_DS: u32 = 88;
pub const R_PPC64_GOT_TPREL16_HI: u32 = 89; pub const R_PPC64_GOT_TPREL16_HA: u32 = 90;
pub const R_PPC64_GOT_DTPREL16_DS: u32 = 91; pub const R_PPC64_GOT_DTPREL16_LO_DS: u32 = 92;
pub const R_PPC64_GOT_DTPREL16_HI: u32 = 93; pub const R_PPC64_GOT_DTPREL16_HA: u32 = 94;
pub const R_PPC64_TPREL16_DS: u32 = 95; pub const R_PPC64_TPREL16_LO_DS: u32 = 96;
pub const R_PPC64_TPREL16_HIGHER: u32 = 97; pub const R_PPC64_TPREL16_HIGHERA: u32 = 98;
pub const R_PPC64_TPREL16_HIGHEST: u32 = 99; pub const R_PPC64_TPREL16_HIGHESTA: u32 = 100;
pub const R_PPC64_DTPREL16_DS: u32 = 101; pub const R_PPC64_DTPREL16_LO_DS: u32 = 102;
pub const R_PPC64_DTPREL16_HIGHER: u32 = 103; pub const R_PPC64_DTPREL16_HIGHERA: u32 = 104;
pub const R_PPC64_DTPREL16_HIGHEST: u32 = 105; pub const R_PPC64_DTPREL16_HIGHESTA: u32 = 106;
pub const R_PPC64_TLSGD: u32 = 107; pub const R_PPC64_TLSLD: u32 = 108;
pub const R_PPC64_TOCSAVE: u32 = 109; pub const R_PPC64_REL24_NOTOC: u32 = 116;
pub const R_PPC64_ENTRY: u32 = 118; pub const R_PPC64_PCREL34: u32 = 132;
pub const R_PPC64_GOT_PCREL34: u32 = 133; pub const R_PPC64_REL16: u32 = 249;
pub const R_PPC64_REL16_LO: u32 = 250; pub const R_PPC64_REL16_HI: u32 = 251;
pub const R_PPC64_REL16_HA: u32 = 252; pub const R_PPC64_NUM: u32 = 253;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
