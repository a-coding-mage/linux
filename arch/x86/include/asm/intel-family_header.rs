/* SPDX-License-Identifier: GPL-2.0 */

/*
 * "Big Core" Processors (Branded as Core, Xeon, etc...)
 *
 * The identifiers below follow the Intel family/model naming scheme from the
 * original C header.  VFM_MAKE!, X86_VENDOR_INTEL, X86_FAMILY_ANY, and
 * X86_MODEL_ANY are supplied by the surrounding translation unit.
 */

macro_rules! IFM {
	($fam:expr, $model:expr) => {
		VFM_MAKE!(X86_VENDOR_INTEL, $fam, $model)
	};
}

/* Wildcard match so X86_MATCH_VFM(ANY) works */
pub const INTEL_ANY: u16 = IFM!(X86_FAMILY_ANY, X86_MODEL_ANY);

/* Family 5 */
pub const INTEL_FAM5_START: u32 = IFM!(5, 0x00); /* Notational marker, also P5 A-step */
pub const INTEL_PENTIUM_75: u32 = IFM!(5, 0x02); /* P54C */
pub const INTEL_PENTIUM_MMX: u32 = IFM!(5, 0x04); /* P55C */
pub const INTEL_QUARK_X1000: u32 = IFM!(5, 0x09); /* Quark X1000 SoC */

/* Family 6, 18, 19 */
pub const INTEL_PENTIUM_PRO: u32 = IFM!(6, 0x01);
pub const INTEL_PENTIUM_II_KLAMATH: u32 = IFM!(6, 0x03);
pub const INTEL_PENTIUM_III_DESCHUTES: u32 = IFM!(6, 0x05);
pub const INTEL_PENTIUM_III_TUALATIN: u32 = IFM!(6, 0x0B);
pub const INTEL_PENTIUM_M_DOTHAN: u32 = IFM!(6, 0x0D);
pub const INTEL_CORE_YONAH: u32 = IFM!(6, 0x0E);
pub const INTEL_CORE2_MEROM: u32 = IFM!(6, 0x0F);
pub const INTEL_CORE2_MEROM_L: u32 = IFM!(6, 0x16);
pub const INTEL_CORE2_PENRYN: u32 = IFM!(6, 0x17);
pub const INTEL_CORE2_DUNNINGTON: u32 = IFM!(6, 0x1D);
pub const INTEL_NEHALEM: u32 = IFM!(6, 0x1E);
pub const INTEL_NEHALEM_G: u32 = IFM!(6, 0x1F); /* Auburndale / Havendale */
pub const INTEL_NEHALEM_EP: u32 = IFM!(6, 0x1A);
pub const INTEL_NEHALEM_EX: u32 = IFM!(6, 0x2E);
pub const INTEL_WESTMERE: u32 = IFM!(6, 0x25);
pub const INTEL_WESTMERE_EP: u32 = IFM!(6, 0x2C);
pub const INTEL_WESTMERE_EX: u32 = IFM!(6, 0x2F);
pub const INTEL_SANDYBRIDGE: u32 = IFM!(6, 0x2A);
pub const INTEL_SANDYBRIDGE_X: u32 = IFM!(6, 0x2D);
pub const INTEL_IVYBRIDGE: u32 = IFM!(6, 0x3A);
pub const INTEL_IVYBRIDGE_X: u32 = IFM!(6, 0x3E);
pub const INTEL_HASWELL: u32 = IFM!(6, 0x3C);
pub const INTEL_HASWELL_X: u32 = IFM!(6, 0x3F);
pub const INTEL_HASWELL_L: u32 = IFM!(6, 0x45);
pub const INTEL_HASWELL_G: u32 = IFM!(6, 0x46);
pub const INTEL_BROADWELL: u32 = IFM!(6, 0x3D);
pub const INTEL_BROADWELL_G: u32 = IFM!(6, 0x47);
pub const INTEL_BROADWELL_X: u32 = IFM!(6, 0x4F);
pub const INTEL_BROADWELL_D: u32 = IFM!(6, 0x56);
pub const INTEL_SKYLAKE_L: u32 = IFM!(6, 0x4E); /* Sky Lake */
pub const INTEL_SKYLAKE: u32 = IFM!(6, 0x5E); /* Sky Lake */
pub const INTEL_SKYLAKE_X: u32 = IFM!(6, 0x55); /* Sky Lake */
/* CASCADELAKE_X 0x55 Sky Lake -- s: 7 */
/* COOPERLAKE_X  0x55 Sky Lake -- s: 11 */
pub const INTEL_KABYLAKE_L: u32 = IFM!(6, 0x8E); /* Sky Lake */
/* AMBERLAKE_L 0x8E Sky Lake -- s: 9 */
/* COFFEELAKE_L 0x8E Sky Lake -- s: 10 */
/* WHISKEYLAKE_L 0x8E Sky Lake -- s: 11,12 */
pub const INTEL_KABYLAKE: u32 = IFM!(6, 0x9E); /* Sky Lake */
/* COFFEELAKE 0x9E Sky Lake -- s: 10-13 */
pub const INTEL_COMETLAKE: u32 = IFM!(6, 0xA5); /* Sky Lake */
pub const INTEL_COMETLAKE_L: u32 = IFM!(6, 0xA6); /* Sky Lake */
pub const INTEL_CANNONLAKE_L: u32 = IFM!(6, 0x66); /* Palm Cove */
pub const INTEL_ICELAKE_X: u32 = IFM!(6, 0x6A); /* Sunny Cove */
pub const INTEL_ICELAKE_D: u32 = IFM!(6, 0x6C); /* Sunny Cove */
pub const INTEL_ICELAKE: u32 = IFM!(6, 0x7D); /* Sunny Cove */
pub const INTEL_ICELAKE_L: u32 = IFM!(6, 0x7E); /* Sunny Cove */
pub const INTEL_ICELAKE_NNPI: u32 = IFM!(6, 0x9D); /* Sunny Cove */
pub const INTEL_ROCKETLAKE: u32 = IFM!(6, 0xA7); /* Cypress Cove */
pub const INTEL_TIGERLAKE_L: u32 = IFM!(6, 0x8C); /* Willow Cove */
pub const INTEL_TIGERLAKE: u32 = IFM!(6, 0x8D); /* Willow Cove */
pub const INTEL_SAPPHIRERAPIDS_X: u32 = IFM!(6, 0x8F); /* Golden Cove */
pub const INTEL_EMERALDRAPIDS_X: u32 = IFM!(6, 0xCF); /* Raptor Cove */
pub const INTEL_GRANITERAPIDS_X: u32 = IFM!(6, 0xAD); /* Redwood Cove */
pub const INTEL_GRANITERAPIDS_D: u32 = IFM!(6, 0xAE);
pub const INTEL_DIAMONDRAPIDS_X: u32 = IFM!(19, 0x01); /* Panther Cove */
pub const INTEL_BARTLETTLAKE: u32 = IFM!(6, 0xD7); /* Raptor Cove */

/* "Hybrid" Processors (P-Core/E-Core) */
pub const INTEL_LAKEFIELD: u32 = IFM!(6, 0x8A); /* Sunny Cove / Tremont */
pub const INTEL_ALDERLAKE: u32 = IFM!(6, 0x97); /* Golden Cove / Gracemont */
pub const INTEL_ALDERLAKE_L: u32 = IFM!(6, 0x9A); /* Golden Cove / Gracemont */
pub const INTEL_RAPTORLAKE: u32 = IFM!(6, 0xB7); /* Raptor Cove / Enhanced Gracemont */
pub const INTEL_RAPTORLAKE_P: u32 = IFM!(6, 0xBA);
pub const INTEL_RAPTORLAKE_S: u32 = IFM!(6, 0xBF);
pub const INTEL_METEORLAKE: u32 = IFM!(6, 0xAC); /* Redwood Cove / Crestmont */
pub const INTEL_METEORLAKE_L: u32 = IFM!(6, 0xAA);
pub const INTEL_ARROWLAKE_H: u32 = IFM!(6, 0xC5); /* Lion Cove / Skymont */
pub const INTEL_ARROWLAKE: u32 = IFM!(6, 0xC6);
pub const INTEL_ARROWLAKE_U: u32 = IFM!(6, 0xB5);
pub const INTEL_LUNARLAKE_M: u32 = IFM!(6, 0xBD); /* Lion Cove / Skymont */
pub const INTEL_PANTHERLAKE_L: u32 = IFM!(6, 0xCC); /* Cougar Cove / Darkmont */
pub const INTEL_PANTHERLAKE_R: u32 = IFM!(6, 0xE5); /* Cougar Cove / Darkmont */
pub const INTEL_WILDCATLAKE_L: u32 = IFM!(6, 0xD5);
pub const INTEL_NOVALAKE: u32 = IFM!(18, 0x01); /* Coyote Cove / Arctic Wolf */
pub const INTEL_NOVALAKE_L: u32 = IFM!(18, 0x03); /* Coyote Cove / Arctic Wolf */

/* "Small Core" Processors (Atom/E-Core) */
pub const INTEL_ATOM_BONNELL: u32 = IFM!(6, 0x1C); /* Diamondville, Pineview */
pub const INTEL_ATOM_BONNELL_MID: u32 = IFM!(6, 0x26); /* Silverthorne, Lincroft */
pub const INTEL_ATOM_SALTWELL: u32 = IFM!(6, 0x36); /* Cedarview */
pub const INTEL_ATOM_SALTWELL_MID: u32 = IFM!(6, 0x27); /* Penwell */
pub const INTEL_ATOM_SALTWELL_TABLET: u32 = IFM!(6, 0x35); /* Cloverview */
pub const INTEL_ATOM_SILVERMONT: u32 = IFM!(6, 0x37); /* Bay Trail, Valleyview */
pub const INTEL_ATOM_SILVERMONT_D: u32 = IFM!(6, 0x4D); /* Avaton, Rangely */
pub const INTEL_ATOM_SILVERMONT_MID: u32 = IFM!(6, 0x4A); /* Merriefield */
pub const INTEL_ATOM_SILVERMONT_MID2: u32 = IFM!(6, 0x5A); /* Anniedale */
pub const INTEL_ATOM_AIRMONT: u32 = IFM!(6, 0x4C); /* Cherry Trail, Braswell */
pub const INTEL_ATOM_AIRMONT_NP: u32 = IFM!(6, 0x75); /* Lightning Mountain */
pub const INTEL_ATOM_GOLDMONT: u32 = IFM!(6, 0x5C); /* Apollo Lake */
pub const INTEL_ATOM_GOLDMONT_D: u32 = IFM!(6, 0x5F); /* Denverton */
/* Note: the micro-architecture is "Goldmont Plus" */
pub const INTEL_ATOM_GOLDMONT_PLUS: u32 = IFM!(6, 0x7A); /* Gemini Lake */
pub const INTEL_ATOM_TREMONT_D: u32 = IFM!(6, 0x86); /* Jacobsville */
pub const INTEL_ATOM_TREMONT: u32 = IFM!(6, 0x96); /* Elkhart Lake */
pub const INTEL_ATOM_TREMONT_L: u32 = IFM!(6, 0x9C); /* Jasper Lake */
pub const INTEL_ATOM_GRACEMONT: u32 = IFM!(6, 0xBE); /* Alderlake N */
pub const INTEL_ATOM_CRESTMONT_X: u32 = IFM!(6, 0xAF); /* Sierra Forest */
pub const INTEL_ATOM_CRESTMONT: u32 = IFM!(6, 0xB6); /* Grand Ridge */
pub const INTEL_ATOM_DARKMONT_X: u32 = IFM!(6, 0xDD); /* Clearwater Forest */

/* Xeon Phi */
pub const INTEL_XEON_PHI_KNL: u32 = IFM!(6, 0x57); /* Knights Landing */
pub const INTEL_XEON_PHI_KNM: u32 = IFM!(6, 0x85); /* Knights Mill */

/* Notational marker denoting the last Family 6 model */
pub const INTEL_FAM6_LAST: u32 = IFM!(6, 0xFF);

/* Family 15 - NetBurst */
pub const INTEL_P4_WILLAMETTE: u32 = IFM!(15, 0x01); /* Also Xeon Foster */
pub const INTEL_P4_PRESCOTT: u32 = IFM!(15, 0x03);
pub const INTEL_P4_PRESCOTT_2M: u32 = IFM!(15, 0x04);
pub const INTEL_P4_CEDARMILL: u32 = IFM!(15, 0x06); /* Also Xeon Dempsey */

/*
 * Intel CPU core types
 *
 * CPUID.1AH.EAX[31:0] uniquely identifies the microarchitecture
 * of the core. Bits 31-24 indicates its core type (Core or Atom)
 * and Bits [23:0] indicates the native model ID of the core.
 * Core type and native model ID are defined in below enumerations.
 */
#[repr(C)]
pub enum intel_cpu_type {
	INTEL_CPU_TYPE_UNKNOWN,
	INTEL_CPU_TYPE_ATOM = 0x20,
	INTEL_CPU_TYPE_CORE = 0x40,
}

#[repr(C)]
pub enum intel_native_id {
	INTEL_ATOM_CMT_NATIVE_ID = 0x2, /* Crestmont */
	INTEL_ATOM_SKT_NATIVE_ID = 0x3, /* Skymont */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
