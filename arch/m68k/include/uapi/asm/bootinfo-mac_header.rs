/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
** asm/bootinfo-mac.h -- Macintosh-specific boot information definitions
*/

/* Macintosh-specific tags (all __be32). */
pub const BI_MAC_MODEL: u32 = 0x8000;
pub const BI_MAC_VADDR: u32 = 0x8001;
pub const BI_MAC_VDEPTH: u32 = 0x8002;
pub const BI_MAC_VROW: u32 = 0x8003;
pub const BI_MAC_VDIM: u32 = 0x8004;
pub const BI_MAC_VLOGICAL: u32 = 0x8005;
pub const BI_MAC_SCCBASE: u32 = 0x8006;
pub const BI_MAC_BTIME: u32 = 0x8007;
pub const BI_MAC_GMTBIAS: u32 = 0x8008;
pub const BI_MAC_MEMSIZE: u32 = 0x8009;
pub const BI_MAC_CPUID: u32 = 0x800a;
pub const BI_MAC_ROMBASE: u32 = 0x800b;

/*
 * Macintosh hardware profile data - unused, see macintosh.h for
 * reasonable type values.
 */
pub const BI_MAC_VIA1BASE: u32 = 0x8010;
pub const BI_MAC_VIA2BASE: u32 = 0x8011;
pub const BI_MAC_VIA2TYPE: u32 = 0x8012;
pub const BI_MAC_ADBTYPE: u32 = 0x8013;
pub const BI_MAC_ASCBASE: u32 = 0x8014;
pub const BI_MAC_SCSI5380: u32 = 0x8015;
pub const BI_MAC_SCSIDMA: u32 = 0x8016;
pub const BI_MAC_SCSI5396: u32 = 0x8017;
pub const BI_MAC_IDETYPE: u32 = 0x8018;
pub const BI_MAC_IDEBASE: u32 = 0x8019;
pub const BI_MAC_NUBUS: u32 = 0x801a;
pub const BI_MAC_SLOTMASK: u32 = 0x801b;
pub const BI_MAC_SCCTYPE: u32 = 0x801c;
pub const BI_MAC_ETHTYPE: u32 = 0x801d;
pub const BI_MAC_ETHBASE: u32 = 0x801e;
pub const BI_MAC_PMU: u32 = 0x801f;
pub const BI_MAC_IOP_SWIM: u32 = 0x8020;
pub const BI_MAC_IOP_ADB: u32 = 0x8021;

/* Macintosh Gestalt numbers (BI_MAC_MODEL). */
pub const MAC_MODEL_II: u32 = 6;
pub const MAC_MODEL_IIX: u32 = 7;
pub const MAC_MODEL_IICX: u32 = 8;
pub const MAC_MODEL_SE30: u32 = 9;
pub const MAC_MODEL_IICI: u32 = 11;
pub const MAC_MODEL_IIFX: u32 = 13; /* And well numbered it is too */
pub const MAC_MODEL_IISI: u32 = 18;
pub const MAC_MODEL_LC: u32 = 19;
pub const MAC_MODEL_Q900: u32 = 20;
pub const MAC_MODEL_PB170: u32 = 21;
pub const MAC_MODEL_Q700: u32 = 22;
pub const MAC_MODEL_CLII: u32 = 23; /* aka: P200 */
pub const MAC_MODEL_PB140: u32 = 25;
pub const MAC_MODEL_Q950: u32 = 26; /* aka: WGS95 */
pub const MAC_MODEL_LCIII: u32 = 27; /* aka: P450 */
pub const MAC_MODEL_PB210: u32 = 29;
pub const MAC_MODEL_C650: u32 = 30;
pub const MAC_MODEL_PB230: u32 = 32;
pub const MAC_MODEL_PB180: u32 = 33;
pub const MAC_MODEL_PB160: u32 = 34;
pub const MAC_MODEL_Q800: u32 = 35; /* aka: WGS80 */
pub const MAC_MODEL_Q650: u32 = 36;
pub const MAC_MODEL_LCII: u32 = 37; /* aka: P400/405/410/430 */
pub const MAC_MODEL_PB250: u32 = 38;
pub const MAC_MODEL_IIVI: u32 = 44;
pub const MAC_MODEL_P600: u32 = 45; /* aka: P600CD */
pub const MAC_MODEL_IIVX: u32 = 48;
pub const MAC_MODEL_CCL: u32 = 49; /* aka: P250 */
pub const MAC_MODEL_PB165C: u32 = 50;
pub const MAC_MODEL_C610: u32 = 52; /* aka: WGS60 */
pub const MAC_MODEL_Q610: u32 = 53;
pub const MAC_MODEL_PB145: u32 = 54; /* aka: PB145B */
pub const MAC_MODEL_P520: u32 = 56; /* aka: LC520 */
pub const MAC_MODEL_C660: u32 = 60;
pub const MAC_MODEL_P460: u32 = 62; /* aka: LCIII+, P466/P467 */
pub const MAC_MODEL_PB180C: u32 = 71;
pub const MAC_MODEL_PB520: u32 = 72; /* aka: PB520C, PB540, PB540C, PB550C */
pub const MAC_MODEL_PB270C: u32 = 77;
pub const MAC_MODEL_Q840: u32 = 78;
pub const MAC_MODEL_P550: u32 = 80; /* aka: LC550, P560 */
pub const MAC_MODEL_CCLII: u32 = 83; /* aka: P275 */
pub const MAC_MODEL_PB165: u32 = 84;
pub const MAC_MODEL_PB190: u32 = 85; /* aka: PB190CS */
pub const MAC_MODEL_TV: u32 = 88;
pub const MAC_MODEL_P475: u32 = 89; /* aka: LC475, P476 */
pub const MAC_MODEL_P475F: u32 = 90; /* aka: P475 w/ FPU (no LC040) */
pub const MAC_MODEL_P575: u32 = 92; /* aka: LC575, P577/P578 */
pub const MAC_MODEL_Q605: u32 = 94;
pub const MAC_MODEL_Q605_ACC: u32 = 95; /* Q605 accelerated to 33 MHz */
pub const MAC_MODEL_Q630: u32 = 98; /* aka: LC630, P630/631/635/636/637/638/640 */
pub const MAC_MODEL_P588: u32 = 99; /* aka: LC580, P580 */
pub const MAC_MODEL_PB280: u32 = 102;
pub const MAC_MODEL_PB280C: u32 = 103;
pub const MAC_MODEL_PB150: u32 = 115;

/* Latest Macintosh bootinfo version. */
pub const MAC_BOOTI_VERSION: u32 = MK_BI_VERSION!(2, 0);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
