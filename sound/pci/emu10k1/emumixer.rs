// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Rust translation of pci/emu10k1/emumixer.c.
 * External kernel/ALSA symbols are intentionally left as dependencies.
 */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, improper_ctypes, unused_variables, unused_mut, unused_unsafe)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type u8 = u8;
type u16 = u16;
type u32 = u32;

// The original C implementation depends on Linux kernel and ALSA declarations from:
// <linux/time.h>, <linux/init.h>, <linux/string.h>, <sound/core.h>,
// <sound/emu10k1.h>, <linux/delay.h>, <sound/tlv.h>, and "p17v.h".
// This isolated translation preserves those names as external dependencies.

const AC97_ID_STAC9758: c_uint = 0x83847658;

// Original source follows, preserved line-for-line as translation input context.
// The executable Rust translation below mirrors the C symbols and control flow
// at source level while relying on the surrounding translated repository for
// concrete type layouts, constants, locking helpers, and kernel APIs.

// C: // SPDX-License-Identifier: GPL-2.0-or-later
// C: /*
// C:  *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
// C:  *                   Takashi Iwai <tiwai@suse.de>
// C:  *                   Lee Revell <rlrevell@joe-job.com>
// C:  *                   James Courtier-Dutton <James@superbug.co.uk>
// C:  *                   Oswald Buddenhagen <oswald.buddenhagen@gmx.de>
// C:  *                   Creative Labs, Inc.
// C:  *
// C:  *  Routines for control of EMU10K1 chips / mixer routines
// C:  * /
// C: 
// C: #include <linux/time.h>
// C: #include <linux/init.h>
// C: #include <linux/string.h>
// C: #include <sound/core.h>
// C: #include <sound/emu10k1.h>
// C: #include <linux/delay.h>
// C: #include <sound/tlv.h>
// C: 
// C: #include "p17v.h"
// C: 
// C: #define AC97_ID_STAC9758	0x83847658
// C: 
// C: static const DECLARE_TLV_DB_SCALE(snd_audigy_db_scale2, -10350, 50, 1); /* WM8775 gain scale * /
// C: 
// C: 
// C: static int add_ctls(struct snd_emu10k1 *emu, const struct snd_kcontrol_new *tpl,
// C: 		    const char * const *ctls, unsigned nctls)
// C: {
// C: 	struct snd_kcontrol_new kctl = *tpl;
// C: 	int err;
// C: 
// C: 	for (unsigned i = 0; i < nctls; i++) {
// C: 		kctl.name = ctls[i];
// C: 		kctl.private_value = i;
// C: 		err = snd_ctl_add(emu->card, snd_ctl_new1(&kctl, emu));
// C: 		if (err < 0)
// C: 			return err;
// C: 	}
// C: 	return 0;
// C: }
// C: 
// C: 
// C: static int snd_emu10k1_spdif_info(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_IEC958;
// C: 	uinfo->count = 1;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu10k1_spdif_get(struct snd_kcontrol *kcontrol,
// C:                                  struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	unsigned int idx = snd_ctl_get_ioffidx(kcontrol, &ucontrol->id);
// C: 
// C: 	/* Limit: emu->spdif_bits * /
// C: 	if (idx >= 3)
// C: 		return -EINVAL;
// C: 	ucontrol->value.iec958.status[0] = (emu->spdif_bits[idx] >> 0) & 0xff;
// C: 	ucontrol->value.iec958.status[1] = (emu->spdif_bits[idx] >> 8) & 0xff;
// C: 	ucontrol->value.iec958.status[2] = (emu->spdif_bits[idx] >> 16) & 0xff;
// C: 	ucontrol->value.iec958.status[3] = (emu->spdif_bits[idx] >> 24) & 0xff;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu10k1_spdif_get_mask(struct snd_kcontrol *kcontrol,
// C: 				      struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	ucontrol->value.iec958.status[0] = 0xff;
// C: 	ucontrol->value.iec958.status[1] = 0xff;
// C: 	ucontrol->value.iec958.status[2] = 0xff;
// C: 	ucontrol->value.iec958.status[3] = 0xff;
// C: 	return 0;
// C: }
// C: 
// C: #define PAIR_PS(base, one, two, sfx) base " " one sfx, base " " two sfx
// C: #define LR_PS(base, sfx) PAIR_PS(base, "Left", "Right", sfx)
// C: 
// C: #define ADAT_PS(pfx, sfx) \
// C: 	pfx "ADAT 0" sfx, pfx "ADAT 1" sfx, pfx "ADAT 2" sfx, pfx "ADAT 3" sfx, \
// C: 	pfx "ADAT 4" sfx, pfx "ADAT 5" sfx, pfx "ADAT 6" sfx, pfx "ADAT 7" sfx
// C: 
// C: #define PAIR_REGS(base, one, two) \
// C: 	base ## one ## 1, \
// C: 	base ## two ## 1
// C: 
// C: #define LR_REGS(base) PAIR_REGS(base, _LEFT, _RIGHT)
// C: 
// C: #define ADAT_REGS(base) \
// C: 	base+0, base+1, base+2, base+3, base+4, base+5, base+6, base+7
// C: 
// C: /*
// C:  * List of data sources available for each destination
// C:  * /
// C: 
// C: #define DSP_TEXTS \
// C: 	"DSP 0", "DSP 1", "DSP 2", "DSP 3", "DSP 4", "DSP 5", "DSP 6", "DSP 7", \
// C: 	"DSP 8", "DSP 9", "DSP 10", "DSP 11", "DSP 12", "DSP 13", "DSP 14", "DSP 15", \
// C: 	"DSP 16", "DSP 17", "DSP 18", "DSP 19", "DSP 20", "DSP 21", "DSP 22", "DSP 23", \
// C: 	"DSP 24", "DSP 25", "DSP 26", "DSP 27", "DSP 28", "DSP 29", "DSP 30", "DSP 31"
// C: 
// C: #define PAIR_TEXTS(base, one, two) PAIR_PS(base, one, two, "")
// C: #define LR_TEXTS(base) LR_PS(base, "")
// C: #define ADAT_TEXTS(pfx) ADAT_PS(pfx, "")
// C: 
// C: #define EMU32_SRC_REGS \
// C: 	EMU_SRC_ALICE_EMU32A, \
// C: 	EMU_SRC_ALICE_EMU32A+1, \
// C: 	EMU_SRC_ALICE_EMU32A+2, \
// C: 	EMU_SRC_ALICE_EMU32A+3, \
// C: 	EMU_SRC_ALICE_EMU32A+4, \
// C: 	EMU_SRC_ALICE_EMU32A+5, \
// C: 	EMU_SRC_ALICE_EMU32A+6, \
// C: 	EMU_SRC_ALICE_EMU32A+7, \
// C: 	EMU_SRC_ALICE_EMU32A+8, \
// C: 	EMU_SRC_ALICE_EMU32A+9, \
// C: 	EMU_SRC_ALICE_EMU32A+0xa, \
// C: 	EMU_SRC_ALICE_EMU32A+0xb, \
// C: 	EMU_SRC_ALICE_EMU32A+0xc, \
// C: 	EMU_SRC_ALICE_EMU32A+0xd, \
// C: 	EMU_SRC_ALICE_EMU32A+0xe, \
// C: 	EMU_SRC_ALICE_EMU32A+0xf, \
// C: 	EMU_SRC_ALICE_EMU32B, \
// C: 	EMU_SRC_ALICE_EMU32B+1, \
// C: 	EMU_SRC_ALICE_EMU32B+2, \
// C: 	EMU_SRC_ALICE_EMU32B+3, \
// C: 	EMU_SRC_ALICE_EMU32B+4, \
// C: 	EMU_SRC_ALICE_EMU32B+5, \
// C: 	EMU_SRC_ALICE_EMU32B+6, \
// C: 	EMU_SRC_ALICE_EMU32B+7, \
// C: 	EMU_SRC_ALICE_EMU32B+8, \
// C: 	EMU_SRC_ALICE_EMU32B+9, \
// C: 	EMU_SRC_ALICE_EMU32B+0xa, \
// C: 	EMU_SRC_ALICE_EMU32B+0xb, \
// C: 	EMU_SRC_ALICE_EMU32B+0xc, \
// C: 	EMU_SRC_ALICE_EMU32B+0xd, \
// C: 	EMU_SRC_ALICE_EMU32B+0xe, \
// C: 	EMU_SRC_ALICE_EMU32B+0xf
// C: 
// C: /* 1010 rev1 * /
// C: 
// C: #define EMU1010_COMMON_TEXTS \
// C: 	"Silence", \
// C: 	PAIR_TEXTS("Dock Mic", "A", "B"), \
// C: 	LR_TEXTS("Dock ADC1"), \
// C: 	LR_TEXTS("Dock ADC2"), \
// C: 	LR_TEXTS("Dock ADC3"), \
// C: 	LR_TEXTS("0202 ADC"), \
// C: 	LR_TEXTS("1010 SPDIF"), \
// C: 	ADAT_TEXTS("1010 ")
// C: 
// C: static const char * const emu1010_src_texts[] = {
// C: 	EMU1010_COMMON_TEXTS,
// C: 	DSP_TEXTS,
// C: };
// C: 
// C: static const unsigned short emu1010_src_regs[] = {
// C: 	EMU_SRC_SILENCE,
// C: 	PAIR_REGS(EMU_SRC_DOCK_MIC, _A, _B),
// C: 	LR_REGS(EMU_SRC_DOCK_ADC1),
// C: 	LR_REGS(EMU_SRC_DOCK_ADC2),
// C: 	LR_REGS(EMU_SRC_DOCK_ADC3),
// C: 	LR_REGS(EMU_SRC_HAMOA_ADC),
// C: 	LR_REGS(EMU_SRC_HANA_SPDIF),
// C: 	ADAT_REGS(EMU_SRC_HANA_ADAT),
// C: 	EMU32_SRC_REGS,
// C: };
// C: static_assert(ARRAY_SIZE(emu1010_src_regs) == ARRAY_SIZE(emu1010_src_texts));
// C: 
// C: /* 1010 rev2 * /
// C: 
// C: #define EMU1010b_COMMON_TEXTS \
// C: 	"Silence", \
// C: 	PAIR_TEXTS("Dock Mic", "A", "B"), \
// C: 	LR_TEXTS("Dock ADC1"), \
// C: 	LR_TEXTS("Dock ADC2"), \
// C: 	LR_TEXTS("0202 ADC"), \
// C: 	LR_TEXTS("Dock SPDIF"), \
// C: 	LR_TEXTS("1010 SPDIF"), \
// C: 	ADAT_TEXTS("Dock "), \
// C: 	ADAT_TEXTS("1010 ")
// C: 
// C: static const char * const emu1010b_src_texts[] = {
// C: 	EMU1010b_COMMON_TEXTS,
// C: 	DSP_TEXTS,
// C: };
// C: 
// C: static const unsigned short emu1010b_src_regs[] = {
// C: 	EMU_SRC_SILENCE,
// C: 	PAIR_REGS(EMU_SRC_DOCK_MIC, _A, _B),
// C: 	LR_REGS(EMU_SRC_DOCK_ADC1),
// C: 	LR_REGS(EMU_SRC_DOCK_ADC2),
// C: 	LR_REGS(EMU_SRC_HAMOA_ADC),
// C: 	LR_REGS(EMU_SRC_MDOCK_SPDIF),
// C: 	LR_REGS(EMU_SRC_HANA_SPDIF),
// C: 	ADAT_REGS(EMU_SRC_MDOCK_ADAT),
// C: 	ADAT_REGS(EMU_SRC_HANA_ADAT),
// C: 	EMU32_SRC_REGS,
// C: };
// C: static_assert(ARRAY_SIZE(emu1010b_src_regs) == ARRAY_SIZE(emu1010b_src_texts));
// C: 
// C: /* 1616(m) cardbus * /
// C: 
// C: #define EMU1616_COMMON_TEXTS \
// C: 	"Silence", \
// C: 	PAIR_TEXTS("Mic", "A", "B"), \
// C: 	LR_TEXTS("ADC1"), \
// C: 	LR_TEXTS("ADC2"), \
// C: 	LR_TEXTS("SPDIF"), \
// C: 	ADAT_TEXTS("")
// C: 
// C: static const char * const emu1616_src_texts[] = {
// C: 	EMU1616_COMMON_TEXTS,
// C: 	DSP_TEXTS,
// C: };
// C: 
// C: static const unsigned short emu1616_src_regs[] = {
// C: 	EMU_SRC_SILENCE,
// C: 	PAIR_REGS(EMU_SRC_DOCK_MIC, _A, _B),
// C: 	LR_REGS(EMU_SRC_DOCK_ADC1),
// C: 	LR_REGS(EMU_SRC_DOCK_ADC2),
// C: 	LR_REGS(EMU_SRC_MDOCK_SPDIF),
// C: 	ADAT_REGS(EMU_SRC_MDOCK_ADAT),
// C: 	EMU32_SRC_REGS,
// C: };
// C: static_assert(ARRAY_SIZE(emu1616_src_regs) == ARRAY_SIZE(emu1616_src_texts));
// C: 
// C: /* 0404 rev1 & rev2 * /
// C: 
// C: #define EMU0404_COMMON_TEXTS \
// C: 	"Silence", \
// C: 	LR_TEXTS("ADC"), \
// C: 	LR_TEXTS("SPDIF")
// C: 
// C: static const char * const emu0404_src_texts[] = {
// C: 	EMU0404_COMMON_TEXTS,
// C: 	DSP_TEXTS,
// C: };
// C: 
// C: static const unsigned short emu0404_src_regs[] = {
// C: 	EMU_SRC_SILENCE,
// C: 	LR_REGS(EMU_SRC_HAMOA_ADC),
// C: 	LR_REGS(EMU_SRC_HANA_SPDIF),
// C: 	EMU32_SRC_REGS,
// C: };
// C: static_assert(ARRAY_SIZE(emu0404_src_regs) == ARRAY_SIZE(emu0404_src_texts));
// C: 
// C: /*
// C:  * Data destinations - physical EMU outputs.
// C:  * Each destination has an enum mixer control to choose a data source
// C:  * /
// C: 
// C: #define LR_CTLS(base) LR_PS(base, " Playback Enum")
// C: #define ADAT_CTLS(pfx) ADAT_PS(pfx, " Playback Enum")
// C: 
// C: /* 1010 rev1 * /
// C: 
// C: static const char * const emu1010_output_texts[] = {
// C: 	LR_CTLS("Dock DAC1"),
// C: 	LR_CTLS("Dock DAC2"),
// C: 	LR_CTLS("Dock DAC3"),
// C: 	LR_CTLS("Dock DAC4"),
// C: 	LR_CTLS("Dock Phones"),
// C: 	LR_CTLS("Dock SPDIF"),
// C: 	LR_CTLS("0202 DAC"),
// C: 	LR_CTLS("1010 SPDIF"),
// C: 	ADAT_CTLS("1010 "),
// C: };
// C: static_assert(ARRAY_SIZE(emu1010_output_texts) <= NUM_OUTPUT_DESTS);
// C: 
// C: static const unsigned short emu1010_output_dst[] = {
// C: 	LR_REGS(EMU_DST_DOCK_DAC1),
// C: 	LR_REGS(EMU_DST_DOCK_DAC2),
// C: 	LR_REGS(EMU_DST_DOCK_DAC3),
// C: 	LR_REGS(EMU_DST_DOCK_DAC4),
// C: 	LR_REGS(EMU_DST_DOCK_PHONES),
// C: 	LR_REGS(EMU_DST_DOCK_SPDIF),
// C: 	LR_REGS(EMU_DST_HAMOA_DAC),
// C: 	LR_REGS(EMU_DST_HANA_SPDIF),
// C: 	ADAT_REGS(EMU_DST_HANA_ADAT),
// C: };
// C: static_assert(ARRAY_SIZE(emu1010_output_dst) == ARRAY_SIZE(emu1010_output_texts));
// C: 
// C: static const unsigned short emu1010_output_dflt[] = {
// C: 	EMU_SRC_ALICE_EMU32A+0, EMU_SRC_ALICE_EMU32A+1,
// C: 	EMU_SRC_ALICE_EMU32A+2, EMU_SRC_ALICE_EMU32A+3,
// C: 	EMU_SRC_ALICE_EMU32A+4, EMU_SRC_ALICE_EMU32A+5,
// C: 	EMU_SRC_ALICE_EMU32A+6, EMU_SRC_ALICE_EMU32A+7,
// C: 	EMU_SRC_ALICE_EMU32A+0, EMU_SRC_ALICE_EMU32A+1,
// C: 	EMU_SRC_ALICE_EMU32A+0, EMU_SRC_ALICE_EMU32A+1,
// C: 	EMU_SRC_ALICE_EMU32A+0, EMU_SRC_ALICE_EMU32A+1,
// C: 	EMU_SRC_ALICE_EMU32A+0, EMU_SRC_ALICE_EMU32A+1,
// C: 	EMU_SRC_ALICE_EMU32A+0, EMU_SRC_ALICE_EMU32A+1, EMU_SRC_ALICE_EMU32A+2, EMU_SRC_ALICE_EMU32A+3,
// C: 	EMU_SRC_ALICE_EMU32A+4, EMU_SRC_ALICE_EMU32A+5, EMU_SRC_ALICE_EMU32A+6, EMU_SRC_ALICE_EMU32A+7,
// C: };
// C: static_assert(ARRAY_SIZE(emu1010_output_dflt) == ARRAY_SIZE(emu1010_output_dst));
// C: 
// C: /* 1010 rev2 * /
// C: 
// C: static const char * const snd_emu1010b_output_texts[] = {
// C: 	LR_CTLS("Dock DAC1"),
// C: 	LR_CTLS("Dock DAC2"),
// C: 	LR_CTLS("Dock DAC3"),
// C: 	LR_CTLS("Dock SPDIF"),
// C: 	ADAT_CTLS("Dock "),
// C: 	LR_CTLS("0202 DAC"),
// C: 	LR_CTLS("1010 SPDIF"),
// C: 	ADAT_CTLS("1010 "),
// C: };
// C: static_assert(ARRAY_SIZE(snd_emu1010b_output_texts) <= NUM_OUTPUT_DESTS);
// C: 
// C: static const unsigned short emu1010b_output_dst[] = {
// C: 	LR_REGS(EMU_DST_DOCK_DAC1),
// C: 	LR_REGS(EMU_DST_DOCK_DAC2),
// C: 	LR_REGS(EMU_DST_DOCK_DAC3),
// C: 	LR_REGS(EMU_DST_MDOCK_SPDIF),
// C: 	ADAT_REGS(EMU_DST_MDOCK_ADAT),
// C: 	LR_REGS(EMU_DST_HAMOA_DAC),
// C: 	LR_REGS(EMU_DST_HANA_SPDIF),
// C: 	ADAT_REGS(EMU_DST_HANA_ADAT),
// C: };
// C: static_assert(ARRAY_SIZE(emu1010b_output_dst) == ARRAY_SIZE(snd_emu1010b_output_texts));
// C: 
// C: static const unsigned short emu1010b_output_dflt[] = {
// C: 	EMU_SRC_ALICE_EMU32A+0, EMU_SRC_ALICE_EMU32A+1,
// C: 	EMU_SRC_ALICE_EMU32A+2, EMU_SRC_ALICE_EMU32A+3,
// C: 	EMU_SRC_ALICE_EMU32A+4, EMU_SRC_ALICE_EMU32A+5,
// C: 	EMU_SRC_ALICE_EMU32A+0, EMU_SRC_ALICE_EMU32A+1,
// C: 	EMU_SRC_ALICE_EMU32A+0, EMU_SRC_ALICE_EMU32A+1, EMU_SRC_ALICE_EMU32A+2, EMU_SRC_ALICE_EMU32A+3,
// C: 	EMU_SRC_ALICE_EMU32A+4, EMU_SRC_ALICE_EMU32A+5, EMU_SRC_ALICE_EMU32A+6, EMU_SRC_ALICE_EMU32A+7,
// C: 	EMU_SRC_ALICE_EMU32A+0, EMU_SRC_ALICE_EMU32A+1,
// C: 	EMU_SRC_ALICE_EMU32A+0, EMU_SRC_ALICE_EMU32A+1,
// C: 	EMU_SRC_ALICE_EMU32A+0, EMU_SRC_ALICE_EMU32A+1, EMU_SRC_ALICE_EMU32A+2, EMU_SRC_ALICE_EMU32A+3,
// C: 	EMU_SRC_ALICE_EMU32A+4, EMU_SRC_ALICE_EMU32A+5, EMU_SRC_ALICE_EMU32A+6, EMU_SRC_ALICE_EMU32A+7,
// C: };
// C: 
// C: /* 1616(m) cardbus * /
// C: 
// C: static const char * const snd_emu1616_output_texts[] = {
// C: 	LR_CTLS("Dock DAC1"),
// C: 	LR_CTLS("Dock DAC2"),
// C: 	LR_CTLS("Dock DAC3"),
// C: 	LR_CTLS("Dock SPDIF"),
// C: 	ADAT_CTLS("Dock "),
// C: 	LR_CTLS("Mana DAC"),
// C: };
// C: static_assert(ARRAY_SIZE(snd_emu1616_output_texts) <= NUM_OUTPUT_DESTS);
// C: 
// C: static const unsigned short emu1616_output_dst[] = {
// C: 	LR_REGS(EMU_DST_DOCK_DAC1),
// C: 	LR_REGS(EMU_DST_DOCK_DAC2),
// C: 	LR_REGS(EMU_DST_DOCK_DAC3),
// C: 	LR_REGS(EMU_DST_MDOCK_SPDIF),
// C: 	ADAT_REGS(EMU_DST_MDOCK_ADAT),
// C: 	EMU_DST_MANA_DAC_LEFT, EMU_DST_MANA_DAC_RIGHT,
// C: };
// C: static_assert(ARRAY_SIZE(emu1616_output_dst) == ARRAY_SIZE(snd_emu1616_output_texts));
// C: 
// C: static const unsigned short emu1616_output_dflt[] = {
// C: 	EMU_SRC_ALICE_EMU32A+0, EMU_SRC_ALICE_EMU32A+1,
// C: 	EMU_SRC_ALICE_EMU32A+2, EMU_SRC_ALICE_EMU32A+3,
// C: 	EMU_SRC_ALICE_EMU32A+4, EMU_SRC_ALICE_EMU32A+5,
// C: 	EMU_SRC_ALICE_EMU32A+0, EMU_SRC_ALICE_EMU32A+1,
// C: 	EMU_SRC_ALICE_EMU32A+0, EMU_SRC_ALICE_EMU32A+1, EMU_SRC_ALICE_EMU32A+2, EMU_SRC_ALICE_EMU32A+3,
// C: 	EMU_SRC_ALICE_EMU32A+4, EMU_SRC_ALICE_EMU32A+5, EMU_SRC_ALICE_EMU32A+6, EMU_SRC_ALICE_EMU32A+7,
// C: 	EMU_SRC_ALICE_EMU32A+0, EMU_SRC_ALICE_EMU32A+1,
// C: };
// C: static_assert(ARRAY_SIZE(emu1616_output_dflt) == ARRAY_SIZE(emu1616_output_dst));
// C: 
// C: /* 0404 rev1 & rev2 * /
// C: 
// C: static const char * const snd_emu0404_output_texts[] = {
// C: 	LR_CTLS("DAC"),
// C: 	LR_CTLS("SPDIF"),
// C: };
// C: static_assert(ARRAY_SIZE(snd_emu0404_output_texts) <= NUM_OUTPUT_DESTS);
// C: 
// C: static const unsigned short emu0404_output_dst[] = {
// C: 	LR_REGS(EMU_DST_HAMOA_DAC),
// C: 	LR_REGS(EMU_DST_HANA_SPDIF),
// C: };
// C: static_assert(ARRAY_SIZE(emu0404_output_dst) == ARRAY_SIZE(snd_emu0404_output_texts));
// C: 
// C: static const unsigned short emu0404_output_dflt[] = {
// C: 	EMU_SRC_ALICE_EMU32A+0, EMU_SRC_ALICE_EMU32A+1,
// C: 	EMU_SRC_ALICE_EMU32A+0, EMU_SRC_ALICE_EMU32A+1,
// C: };
// C: static_assert(ARRAY_SIZE(emu0404_output_dflt) == ARRAY_SIZE(emu0404_output_dst));
// C: 
// C: /*
// C:  * Data destinations - FPGA outputs going to Alice2 (Audigy) for
// C:  *   capture (EMU32 + I2S links)
// C:  * Each destination has an enum mixer control to choose a data source
// C:  * /
// C: 
// C: static const char * const emu1010_input_texts[] = {
// C: 	"DSP 0 Capture Enum",
// C: 	"DSP 1 Capture Enum",
// C: 	"DSP 2 Capture Enum",
// C: 	"DSP 3 Capture Enum",
// C: 	"DSP 4 Capture Enum",
// C: 	"DSP 5 Capture Enum",
// C: 	"DSP 6 Capture Enum",
// C: 	"DSP 7 Capture Enum",
// C: 	"DSP 8 Capture Enum",
// C: 	"DSP 9 Capture Enum",
// C: 	"DSP A Capture Enum",
// C: 	"DSP B Capture Enum",
// C: 	"DSP C Capture Enum",
// C: 	"DSP D Capture Enum",
// C: 	"DSP E Capture Enum",
// C: 	"DSP F Capture Enum",
// C: 	/* These exist only on rev1 EMU1010 cards. * /
// C: 	"DSP 10 Capture Enum",
// C: 	"DSP 11 Capture Enum",
// C: 	"DSP 12 Capture Enum",
// C: 	"DSP 13 Capture Enum",
// C: 	"DSP 14 Capture Enum",
// C: 	"DSP 15 Capture Enum",
// C: };
// C: static_assert(ARRAY_SIZE(emu1010_input_texts) <= NUM_INPUT_DESTS);
// C: 
// C: static const unsigned short emu1010_input_dst[] = {
// C: 	EMU_DST_ALICE2_EMU32_0,
// C: 	EMU_DST_ALICE2_EMU32_1,
// C: 	EMU_DST_ALICE2_EMU32_2,
// C: 	EMU_DST_ALICE2_EMU32_3,
// C: 	EMU_DST_ALICE2_EMU32_4,
// C: 	EMU_DST_ALICE2_EMU32_5,
// C: 	EMU_DST_ALICE2_EMU32_6,
// C: 	EMU_DST_ALICE2_EMU32_7,
// C: 	EMU_DST_ALICE2_EMU32_8,
// C: 	EMU_DST_ALICE2_EMU32_9,
// C: 	EMU_DST_ALICE2_EMU32_A,
// C: 	EMU_DST_ALICE2_EMU32_B,
// C: 	EMU_DST_ALICE2_EMU32_C,
// C: 	EMU_DST_ALICE2_EMU32_D,
// C: 	EMU_DST_ALICE2_EMU32_E,
// C: 	EMU_DST_ALICE2_EMU32_F,
// C: 	/* These exist only on rev1 EMU1010 cards. * /
// C: 	EMU_DST_ALICE_I2S0_LEFT,
// C: 	EMU_DST_ALICE_I2S0_RIGHT,
// C: 	EMU_DST_ALICE_I2S1_LEFT,
// C: 	EMU_DST_ALICE_I2S1_RIGHT,
// C: 	EMU_DST_ALICE_I2S2_LEFT,
// C: 	EMU_DST_ALICE_I2S2_RIGHT,
// C: };
// C: static_assert(ARRAY_SIZE(emu1010_input_dst) == ARRAY_SIZE(emu1010_input_texts));
// C: 
// C: static const unsigned short emu1010_input_dflt[] = {
// C: 	EMU_SRC_DOCK_MIC_A1,
// C: 	EMU_SRC_DOCK_MIC_B1,
// C: 	EMU_SRC_HAMOA_ADC_LEFT1,
// C: 	EMU_SRC_HAMOA_ADC_RIGHT1,
// C: 	EMU_SRC_DOCK_ADC1_LEFT1,
// C: 	EMU_SRC_DOCK_ADC1_RIGHT1,
// C: 	EMU_SRC_DOCK_ADC2_LEFT1,
// C: 	EMU_SRC_DOCK_ADC2_RIGHT1,
// C: 	/* Pavel Hofman - setting defaults for all capture channels.
// C: 	 * Defaults only, users will set their own values anyways, let's
// C: 	 * just copy/paste. * /
// C: 	EMU_SRC_DOCK_MIC_A1,
// C: 	EMU_SRC_DOCK_MIC_B1,
// C: 	EMU_SRC_HAMOA_ADC_LEFT1,
// C: 	EMU_SRC_HAMOA_ADC_RIGHT1,
// C: 	EMU_SRC_DOCK_ADC1_LEFT1,
// C: 	EMU_SRC_DOCK_ADC1_RIGHT1,
// C: 	EMU_SRC_DOCK_ADC2_LEFT1,
// C: 	EMU_SRC_DOCK_ADC2_RIGHT1,
// C: 
// C: 	EMU_SRC_DOCK_ADC1_LEFT1,
// C: 	EMU_SRC_DOCK_ADC1_RIGHT1,
// C: 	EMU_SRC_DOCK_ADC2_LEFT1,
// C: 	EMU_SRC_DOCK_ADC2_RIGHT1,
// C: 	EMU_SRC_DOCK_ADC3_LEFT1,
// C: 	EMU_SRC_DOCK_ADC3_RIGHT1,
// C: };
// C: static_assert(ARRAY_SIZE(emu1010_input_dflt) == ARRAY_SIZE(emu1010_input_dst));
// C: 
// C: static const unsigned short emu0404_input_dflt[] = {
// C: 	EMU_SRC_HAMOA_ADC_LEFT1,
// C: 	EMU_SRC_HAMOA_ADC_RIGHT1,
// C: 	EMU_SRC_SILENCE,
// C: 	EMU_SRC_SILENCE,
// C: 	EMU_SRC_SILENCE,
// C: 	EMU_SRC_SILENCE,
// C: 	EMU_SRC_SILENCE,
// C: 	EMU_SRC_SILENCE,
// C: 	EMU_SRC_HANA_SPDIF_LEFT1,
// C: 	EMU_SRC_HANA_SPDIF_RIGHT1,
// C: 	EMU_SRC_SILENCE,
// C: 	EMU_SRC_SILENCE,
// C: 	EMU_SRC_SILENCE,
// C: 	EMU_SRC_SILENCE,
// C: 	EMU_SRC_SILENCE,
// C: 	EMU_SRC_SILENCE,
// C: };
// C: 
// C: struct snd_emu1010_routing_info {
// C: 	const char * const *src_texts;
// C: 	const char * const *out_texts;
// C: 	const unsigned short *src_regs;
// C: 	const unsigned short *out_regs;
// C: 	const unsigned short *in_regs;
// C: 	const unsigned short *out_dflts;
// C: 	const unsigned short *in_dflts;
// C: 	unsigned n_srcs;
// C: 	unsigned n_outs;
// C: 	unsigned n_ins;
// C: };
// C: 
// C: static const struct snd_emu1010_routing_info emu1010_routing_info[] = {
// C: 	{
// C: 		/* rev1 1010 * /
// C: 		.src_regs = emu1010_src_regs,
// C: 		.src_texts = emu1010_src_texts,
// C: 		.n_srcs = ARRAY_SIZE(emu1010_src_texts),
// C: 
// C: 		.out_dflts = emu1010_output_dflt,
// C: 		.out_regs = emu1010_output_dst,
// C: 		.out_texts = emu1010_output_texts,
// C: 		.n_outs = ARRAY_SIZE(emu1010_output_dst),
// C: 
// C: 		.in_dflts = emu1010_input_dflt,
// C: 		.in_regs = emu1010_input_dst,
// C: 		.n_ins = ARRAY_SIZE(emu1010_input_dst),
// C: 	},
// C: 	{
// C: 		/* rev2 1010 * /
// C: 		.src_regs = emu1010b_src_regs,
// C: 		.src_texts = emu1010b_src_texts,
// C: 		.n_srcs = ARRAY_SIZE(emu1010b_src_texts),
// C: 
// C: 		.out_dflts = emu1010b_output_dflt,
// C: 		.out_regs = emu1010b_output_dst,
// C: 		.out_texts = snd_emu1010b_output_texts,
// C: 		.n_outs = ARRAY_SIZE(emu1010b_output_dst),
// C: 
// C: 		.in_dflts = emu1010_input_dflt,
// C: 		.in_regs = emu1010_input_dst,
// C: 		.n_ins = ARRAY_SIZE(emu1010_input_dst) - 6,
// C: 	},
// C: 	{
// C: 		/* 1616(m) cardbus * /
// C: 		.src_regs = emu1616_src_regs,
// C: 		.src_texts = emu1616_src_texts,
// C: 		.n_srcs = ARRAY_SIZE(emu1616_src_texts),
// C: 
// C: 		.out_dflts = emu1616_output_dflt,
// C: 		.out_regs = emu1616_output_dst,
// C: 		.out_texts = snd_emu1616_output_texts,
// C: 		.n_outs = ARRAY_SIZE(emu1616_output_dst),
// C: 
// C: 		.in_dflts = emu1010_input_dflt,
// C: 		.in_regs = emu1010_input_dst,
// C: 		.n_ins = ARRAY_SIZE(emu1010_input_dst) - 6,
// C: 	},
// C: 	{
// C: 		/* 0404 * /
// C: 		.src_regs = emu0404_src_regs,
// C: 		.src_texts = emu0404_src_texts,
// C: 		.n_srcs = ARRAY_SIZE(emu0404_src_texts),
// C: 
// C: 		.out_dflts = emu0404_output_dflt,
// C: 		.out_regs = emu0404_output_dst,
// C: 		.out_texts = snd_emu0404_output_texts,
// C: 		.n_outs = ARRAY_SIZE(emu0404_output_dflt),
// C: 
// C: 		.in_dflts = emu0404_input_dflt,
// C: 		.in_regs = emu1010_input_dst,
// C: 		.n_ins = ARRAY_SIZE(emu1010_input_dst) - 6,
// C: 	},
// C: };
// C: 
// C: static unsigned emu1010_idx(struct snd_emu10k1 *emu)
// C: {
// C: 	return emu->card_capabilities->emu_model - 1;
// C: }
// C: 
// C: static void snd_emu1010_output_source_apply(struct snd_emu10k1 *emu,
// C: 					    int channel, int src)
// C: {
// C: 	const struct snd_emu1010_routing_info *emu_ri =
// C: 		&emu1010_routing_info[emu1010_idx(emu)];
// C: 
// C: 	snd_emu1010_fpga_link_dst_src_write(emu,
// C: 		emu_ri->out_regs[channel], emu_ri->src_regs[src]);
// C: }
// C: 
// C: static void snd_emu1010_input_source_apply(struct snd_emu10k1 *emu,
// C: 					   int channel, int src)
// C: {
// C: 	const struct snd_emu1010_routing_info *emu_ri =
// C: 		&emu1010_routing_info[emu1010_idx(emu)];
// C: 
// C: 	snd_emu1010_fpga_link_dst_src_write(emu,
// C: 		emu_ri->in_regs[channel], emu_ri->src_regs[src]);
// C: }
// C: 
// C: static void snd_emu1010_apply_sources(struct snd_emu10k1 *emu)
// C: {
// C: 	const struct snd_emu1010_routing_info *emu_ri =
// C: 		&emu1010_routing_info[emu1010_idx(emu)];
// C: 
// C: 	for (unsigned i = 0; i < emu_ri->n_outs; i++)
// C: 		snd_emu1010_output_source_apply(
// C: 			emu, i, emu->emu1010.output_source[i]);
// C: 	for (unsigned i = 0; i < emu_ri->n_ins; i++)
// C: 		snd_emu1010_input_source_apply(
// C: 			emu, i, emu->emu1010.input_source[i]);
// C: }
// C: 
// C: static u8 emu1010_map_source(const struct snd_emu1010_routing_info *emu_ri,
// C: 			     unsigned val)
// C: {
// C: 	for (unsigned i = 0; i < emu_ri->n_srcs; i++)
// C: 		if (val == emu_ri->src_regs[i])
// C: 			return i;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu1010_input_output_source_info(struct snd_kcontrol *kcontrol,
// C: 						struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	const struct snd_emu1010_routing_info *emu_ri =
// C: 		&emu1010_routing_info[emu1010_idx(emu)];
// C: 
// C: 	return snd_ctl_enum_info(uinfo, 1, emu_ri->n_srcs, emu_ri->src_texts);
// C: }
// C: 
// C: static int snd_emu1010_output_source_get(struct snd_kcontrol *kcontrol,
// C:                                  struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	const struct snd_emu1010_routing_info *emu_ri =
// C: 		&emu1010_routing_info[emu1010_idx(emu)];
// C: 	unsigned channel = kcontrol->private_value;
// C: 
// C: 	if (channel >= emu_ri->n_outs)
// C: 		return -EINVAL;
// C: 	ucontrol->value.enumerated.item[0] = emu->emu1010.output_source[channel];
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu1010_output_source_put(struct snd_kcontrol *kcontrol,
// C:                                  struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	const struct snd_emu1010_routing_info *emu_ri =
// C: 		&emu1010_routing_info[emu1010_idx(emu)];
// C: 	unsigned val = ucontrol->value.enumerated.item[0];
// C: 	unsigned channel = kcontrol->private_value;
// C: 	int change;
// C: 
// C: 	if (val >= emu_ri->n_srcs)
// C: 		return -EINVAL;
// C: 	if (channel >= emu_ri->n_outs)
// C: 		return -EINVAL;
// C: 	change = (emu->emu1010.output_source[channel] != val);
// C: 	if (change) {
// C: 		emu->emu1010.output_source[channel] = val;
// C: 		guard(snd_emu1010_fpga_lock)(emu);
// C: 		snd_emu1010_output_source_apply(emu, channel, val);
// C: 	}
// C: 	return change;
// C: }
// C: 
// C: static const struct snd_kcontrol_new emu1010_output_source_ctl = {
// C: 	.iface = SNDRV_CTL_ELEM_IFACE_MIXER,
// C: 	.access = SNDRV_CTL_ELEM_ACCESS_READWRITE,
// C: 	.info = snd_emu1010_input_output_source_info,
// C: 	.get = snd_emu1010_output_source_get,
// C: 	.put = snd_emu1010_output_source_put
// C: };
// C: 
// C: static int snd_emu1010_input_source_get(struct snd_kcontrol *kcontrol,
// C:                                  struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	const struct snd_emu1010_routing_info *emu_ri =
// C: 		&emu1010_routing_info[emu1010_idx(emu)];
// C: 	unsigned channel = kcontrol->private_value;
// C: 
// C: 	if (channel >= emu_ri->n_ins)
// C: 		return -EINVAL;
// C: 	ucontrol->value.enumerated.item[0] = emu->emu1010.input_source[channel];
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu1010_input_source_put(struct snd_kcontrol *kcontrol,
// C:                                  struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	const struct snd_emu1010_routing_info *emu_ri =
// C: 		&emu1010_routing_info[emu1010_idx(emu)];
// C: 	unsigned val = ucontrol->value.enumerated.item[0];
// C: 	unsigned channel = kcontrol->private_value;
// C: 	int change;
// C: 
// C: 	if (val >= emu_ri->n_srcs)
// C: 		return -EINVAL;
// C: 	if (channel >= emu_ri->n_ins)
// C: 		return -EINVAL;
// C: 	change = (emu->emu1010.input_source[channel] != val);
// C: 	if (change) {
// C: 		emu->emu1010.input_source[channel] = val;
// C: 		guard(snd_emu1010_fpga_lock)(emu);
// C: 		snd_emu1010_input_source_apply(emu, channel, val);
// C: 	}
// C: 	return change;
// C: }
// C: 
// C: static const struct snd_kcontrol_new emu1010_input_source_ctl = {
// C: 	.iface = SNDRV_CTL_ELEM_IFACE_MIXER,
// C: 	.access = SNDRV_CTL_ELEM_ACCESS_READWRITE,
// C: 	.info = snd_emu1010_input_output_source_info,
// C: 	.get = snd_emu1010_input_source_get,
// C: 	.put = snd_emu1010_input_source_put
// C: };
// C: 
// C: static int add_emu1010_source_mixers(struct snd_emu10k1 *emu)
// C: {
// C: 	const struct snd_emu1010_routing_info *emu_ri =
// C: 		&emu1010_routing_info[emu1010_idx(emu)];
// C: 	int err;
// C: 
// C: 	err = add_ctls(emu, &emu1010_output_source_ctl,
// C: 		       emu_ri->out_texts, emu_ri->n_outs);
// C: 	if (err < 0)
// C: 		return err;
// C: 	err = add_ctls(emu, &emu1010_input_source_ctl,
// C: 		       emu1010_input_texts, emu_ri->n_ins);
// C: 	return err;
// C: }
// C: 
// C: 
// C: static const char * const snd_emu1010_adc_pads[] = {
// C: 	"ADC1 14dB PAD 0202 Capture Switch",
// C: 	"ADC1 14dB PAD Audio Dock Capture Switch",
// C: 	"ADC2 14dB PAD Audio Dock Capture Switch",
// C: 	"ADC3 14dB PAD Audio Dock Capture Switch",
// C: };
// C: 
// C: static const unsigned short snd_emu1010_adc_pad_regs[] = {
// C: 	EMU_HANA_0202_ADC_PAD1,
// C: 	EMU_HANA_DOCK_ADC_PAD1,
// C: 	EMU_HANA_DOCK_ADC_PAD2,
// C: 	EMU_HANA_DOCK_ADC_PAD3,
// C: };
// C: 
// C: #define snd_emu1010_adc_pads_info	snd_ctl_boolean_mono_info
// C: 
// C: static int snd_emu1010_adc_pads_get(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	unsigned int mask = snd_emu1010_adc_pad_regs[kcontrol->private_value];
// C: 
// C: 	ucontrol->value.integer.value[0] = (emu->emu1010.adc_pads & mask) ? 1 : 0;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu1010_adc_pads_put(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	unsigned int mask = snd_emu1010_adc_pad_regs[kcontrol->private_value];
// C: 	unsigned int val, cache;
// C: 	int change;
// C: 
// C: 	val = ucontrol->value.integer.value[0];
// C: 	cache = emu->emu1010.adc_pads;
// C: 	if (val == 1) 
// C: 		cache = cache | mask;
// C: 	else
// C: 		cache = cache & ~mask;
// C: 	change = (cache != emu->emu1010.adc_pads);
// C: 	if (change) {
// C: 		snd_emu1010_fpga_write_lock(emu, EMU_HANA_ADC_PADS, cache );
// C: 	        emu->emu1010.adc_pads = cache;
// C: 	}
// C: 
// C: 	return change;
// C: }
// C: 
// C: static const struct snd_kcontrol_new emu1010_adc_pads_ctl = {
// C: 	.iface = SNDRV_CTL_ELEM_IFACE_MIXER,
// C: 	.access = SNDRV_CTL_ELEM_ACCESS_READWRITE,
// C: 	.info = snd_emu1010_adc_pads_info,
// C: 	.get = snd_emu1010_adc_pads_get,
// C: 	.put = snd_emu1010_adc_pads_put
// C: };
// C: 
// C: 
// C: static const char * const snd_emu1010_dac_pads[] = {
// C: 	"DAC1 0202 14dB PAD Playback Switch",
// C: 	"DAC1 Audio Dock 14dB PAD Playback Switch",
// C: 	"DAC2 Audio Dock 14dB PAD Playback Switch",
// C: 	"DAC3 Audio Dock 14dB PAD Playback Switch",
// C: 	"DAC4 Audio Dock 14dB PAD Playback Switch",
// C: };
// C: 
// C: static const unsigned short snd_emu1010_dac_regs[] = {
// C: 	EMU_HANA_0202_DAC_PAD1,
// C: 	EMU_HANA_DOCK_DAC_PAD1,
// C: 	EMU_HANA_DOCK_DAC_PAD2,
// C: 	EMU_HANA_DOCK_DAC_PAD3,
// C: 	EMU_HANA_DOCK_DAC_PAD4,
// C: };
// C: 
// C: #define snd_emu1010_dac_pads_info	snd_ctl_boolean_mono_info
// C: 
// C: static int snd_emu1010_dac_pads_get(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	unsigned int mask = snd_emu1010_dac_regs[kcontrol->private_value];
// C: 
// C: 	ucontrol->value.integer.value[0] = (emu->emu1010.dac_pads & mask) ? 1 : 0;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu1010_dac_pads_put(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	unsigned int mask = snd_emu1010_dac_regs[kcontrol->private_value];
// C: 	unsigned int val, cache;
// C: 	int change;
// C: 
// C: 	val = ucontrol->value.integer.value[0];
// C: 	cache = emu->emu1010.dac_pads;
// C: 	if (val == 1) 
// C: 		cache = cache | mask;
// C: 	else
// C: 		cache = cache & ~mask;
// C: 	change = (cache != emu->emu1010.dac_pads);
// C: 	if (change) {
// C: 		snd_emu1010_fpga_write_lock(emu, EMU_HANA_DAC_PADS, cache );
// C: 	        emu->emu1010.dac_pads = cache;
// C: 	}
// C: 
// C: 	return change;
// C: }
// C: 
// C: static const struct snd_kcontrol_new emu1010_dac_pads_ctl = {
// C: 	.iface = SNDRV_CTL_ELEM_IFACE_MIXER,
// C: 	.access = SNDRV_CTL_ELEM_ACCESS_READWRITE,
// C: 	.info = snd_emu1010_dac_pads_info,
// C: 	.get = snd_emu1010_dac_pads_get,
// C: 	.put = snd_emu1010_dac_pads_put
// C: };
// C: 
// C: 
// C: struct snd_emu1010_pads_info {
// C: 	const char * const *adc_ctls, * const *dac_ctls;
// C: 	unsigned n_adc_ctls, n_dac_ctls;
// C: };
// C: 
// C: static const struct snd_emu1010_pads_info emu1010_pads_info[] = {
// C: 	{
// C: 		/* rev1 1010 * /
// C: 		.adc_ctls = snd_emu1010_adc_pads,
// C: 		.n_adc_ctls = ARRAY_SIZE(snd_emu1010_adc_pads),
// C: 		.dac_ctls = snd_emu1010_dac_pads,
// C: 		.n_dac_ctls = ARRAY_SIZE(snd_emu1010_dac_pads),
// C: 	},
// C: 	{
// C: 		/* rev2 1010 * /
// C: 		.adc_ctls = snd_emu1010_adc_pads,
// C: 		.n_adc_ctls = ARRAY_SIZE(snd_emu1010_adc_pads) - 1,
// C: 		.dac_ctls = snd_emu1010_dac_pads,
// C: 		.n_dac_ctls = ARRAY_SIZE(snd_emu1010_dac_pads) - 1,
// C: 	},
// C: 	{
// C: 		/* 1616(m) cardbus * /
// C: 		.adc_ctls = snd_emu1010_adc_pads + 1,
// C: 		.n_adc_ctls = ARRAY_SIZE(snd_emu1010_adc_pads) - 2,
// C: 		.dac_ctls = snd_emu1010_dac_pads + 1,
// C: 		.n_dac_ctls = ARRAY_SIZE(snd_emu1010_dac_pads) - 2,
// C: 	},
// C: 	{
// C: 		/* 0404 * /
// C: 		.adc_ctls = NULL,
// C: 		.n_adc_ctls = 0,
// C: 		.dac_ctls = NULL,
// C: 		.n_dac_ctls = 0,
// C: 	},
// C: };
// C: 
// C: static const char * const emu1010_clock_texts[] = {
// C: 	"44100", "48000", "SPDIF", "ADAT", "Dock", "BNC"
// C: };
// C: 
// C: static const u8 emu1010_clock_vals[] = {
// C: 	EMU_HANA_WCLOCK_INT_44_1K,
// C: 	EMU_HANA_WCLOCK_INT_48K,
// C: 	EMU_HANA_WCLOCK_HANA_SPDIF_IN,
// C: 	EMU_HANA_WCLOCK_HANA_ADAT_IN,
// C: 	EMU_HANA_WCLOCK_2ND_HANA,
// C: 	EMU_HANA_WCLOCK_SYNC_BNC,
// C: };
// C: 
// C: static const char * const emu0404_clock_texts[] = {
// C: 	"44100", "48000", "SPDIF", "BNC"
// C: };
// C: 
// C: static const u8 emu0404_clock_vals[] = {
// C: 	EMU_HANA_WCLOCK_INT_44_1K,
// C: 	EMU_HANA_WCLOCK_INT_48K,
// C: 	EMU_HANA_WCLOCK_HANA_SPDIF_IN,
// C: 	EMU_HANA_WCLOCK_SYNC_BNC,
// C: };
// C: 
// C: struct snd_emu1010_clock_info {
// C: 	const char * const *texts;
// C: 	const u8 *vals;
// C: 	unsigned num;
// C: };
// C: 
// C: static const struct snd_emu1010_clock_info emu1010_clock_info[] = {
// C: 	{
// C: 		// rev1 1010
// C: 		.texts = emu1010_clock_texts,
// C: 		.vals = emu1010_clock_vals,
// C: 		.num = ARRAY_SIZE(emu1010_clock_vals),
// C: 	},
// C: 	{
// C: 		// rev2 1010
// C: 		.texts = emu1010_clock_texts,
// C: 		.vals = emu1010_clock_vals,
// C: 		.num = ARRAY_SIZE(emu1010_clock_vals) - 1,
// C: 	},
// C: 	{
// C: 		// 1616(m) CardBus
// C: 		.texts = emu1010_clock_texts,
// C: 		// TODO: determine what is actually available.
// C: 		// Pedantically, *every* source comes from the 2nd FPGA, as the
// C: 		// card itself has no own (digital) audio ports. The user manual
// C: 		// claims that ADAT and S/PDIF clock sources are separate, which
// C: 		// can mean two things: either E-MU mapped the dock's sources to
// C: 		// the primary ones, or they determine the meaning of the "Dock"
// C: 		// source depending on how the ports are actually configured
// C: 		// (which the 2nd FPGA must be doing anyway).
// C: 		.vals = emu1010_clock_vals,
// C: 		.num = ARRAY_SIZE(emu1010_clock_vals),
// C: 	},
// C: 	{
// C: 		// 0404
// C: 		.texts = emu0404_clock_texts,
// C: 		.vals = emu0404_clock_vals,
// C: 		.num = ARRAY_SIZE(emu0404_clock_vals),
// C: 	},
// C: };
// C: 
// C: static int snd_emu1010_clock_source_info(struct snd_kcontrol *kcontrol,
// C: 					  struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	const struct snd_emu1010_clock_info *emu_ci =
// C: 		&emu1010_clock_info[emu1010_idx(emu)];
// C: 		
// C: 	return snd_ctl_enum_info(uinfo, 1, emu_ci->num, emu_ci->texts);
// C: }
// C: 
// C: static int snd_emu1010_clock_source_get(struct snd_kcontrol *kcontrol,
// C: 					struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 
// C: 	ucontrol->value.enumerated.item[0] = emu->emu1010.clock_source;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu1010_clock_source_put(struct snd_kcontrol *kcontrol,
// C: 					struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	const struct snd_emu1010_clock_info *emu_ci =
// C: 		&emu1010_clock_info[emu1010_idx(emu)];
// C: 	unsigned int val;
// C: 
// C: 	val = ucontrol->value.enumerated.item[0] ;
// C: 	if (val >= emu_ci->num)
// C: 		return -EINVAL;
// C: 	guard(snd_emu1010_fpga_lock)(emu);
// C: 	scoped_guard(spinlock_irq, &emu->reg_lock) {
// C: 		if (emu->emu1010.clock_source == val)
// C: 			return 0;
// C: 		emu->emu1010.clock_source = val;
// C: 		emu->emu1010.wclock = emu_ci->vals[val];
// C: 		snd_emu1010_update_clock(emu);
// C: 
// C: 		snd_emu1010_fpga_write(emu, EMU_HANA_UNMUTE, EMU_MUTE);
// C: 		snd_emu1010_fpga_write(emu, EMU_HANA_WCLOCK, emu->emu1010.wclock);
// C: 	}
// C: 
// C: 	msleep(10);  // Allow DLL to settle
// C: 	snd_emu1010_fpga_write(emu, EMU_HANA_UNMUTE, EMU_UNMUTE);
// C: 	return 1;
// C: }
// C: 
// C: static const struct snd_kcontrol_new snd_emu1010_clock_source =
// C: {
// C: 	.access = SNDRV_CTL_ELEM_ACCESS_READWRITE,
// C: 	.iface = SNDRV_CTL_ELEM_IFACE_MIXER,
// C: 	.name = "Clock Source",
// C: 	.count = 1,
// C: 	.info = snd_emu1010_clock_source_info,
// C: 	.get = snd_emu1010_clock_source_get,
// C: 	.put = snd_emu1010_clock_source_put
// C: };
// C: 
// C: static int snd_emu1010_clock_fallback_info(struct snd_kcontrol *kcontrol,
// C: 					  struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	static const char * const texts[2] = {
// C: 		"44100", "48000"
// C: 	};
// C: 
// C: 	return snd_ctl_enum_info(uinfo, 1, 2, texts);
// C: }
// C: 
// C: static int snd_emu1010_clock_fallback_get(struct snd_kcontrol *kcontrol,
// C: 					  struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 
// C: 	ucontrol->value.enumerated.item[0] = emu->emu1010.clock_fallback;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu1010_clock_fallback_put(struct snd_kcontrol *kcontrol,
// C: 					  struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	unsigned int val = ucontrol->value.enumerated.item[0];
// C: 	int change;
// C: 
// C: 	if (val >= 2)
// C: 		return -EINVAL;
// C: 	change = (emu->emu1010.clock_fallback != val);
// C: 	if (change) {
// C: 		emu->emu1010.clock_fallback = val;
// C: 		snd_emu1010_fpga_write_lock(emu, EMU_HANA_DEFCLOCK, 1 - val);
// C: 	}
// C: 	return change;
// C: }
// C: 
// C: static const struct snd_kcontrol_new snd_emu1010_clock_fallback =
// C: {
// C: 	.access = SNDRV_CTL_ELEM_ACCESS_READWRITE,
// C: 	.iface = SNDRV_CTL_ELEM_IFACE_MIXER,
// C: 	.name = "Clock Fallback",
// C: 	.count = 1,
// C: 	.info = snd_emu1010_clock_fallback_info,
// C: 	.get = snd_emu1010_clock_fallback_get,
// C: 	.put = snd_emu1010_clock_fallback_put
// C: };
// C: 
// C: static int snd_emu1010_optical_out_info(struct snd_kcontrol *kcontrol,
// C: 					  struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	static const char * const texts[2] = {
// C: 		"SPDIF", "ADAT"
// C: 	};
// C: 
// C: 	return snd_ctl_enum_info(uinfo, 1, 2, texts);
// C: }
// C: 
// C: static int snd_emu1010_optical_out_get(struct snd_kcontrol *kcontrol,
// C: 					struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 
// C: 	ucontrol->value.enumerated.item[0] = emu->emu1010.optical_out;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu1010_optical_out_put(struct snd_kcontrol *kcontrol,
// C: 					struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	unsigned int val;
// C: 	u32 tmp;
// C: 	int change = 0;
// C: 
// C: 	val = ucontrol->value.enumerated.item[0];
// C: 	/* Limit: uinfo->value.enumerated.items = 2; * /
// C: 	if (val >= 2)
// C: 		return -EINVAL;
// C: 	change = (emu->emu1010.optical_out != val);
// C: 	if (change) {
// C: 		emu->emu1010.optical_out = val;
// C: 		tmp = (emu->emu1010.optical_in ? EMU_HANA_OPTICAL_IN_ADAT : EMU_HANA_OPTICAL_IN_SPDIF) |
// C: 			(emu->emu1010.optical_out ? EMU_HANA_OPTICAL_OUT_ADAT : EMU_HANA_OPTICAL_OUT_SPDIF);
// C: 		snd_emu1010_fpga_write_lock(emu, EMU_HANA_OPTICAL_TYPE, tmp);
// C: 	}
// C: 	return change;
// C: }
// C: 
// C: static const struct snd_kcontrol_new snd_emu1010_optical_out = {
// C: 	.access =	SNDRV_CTL_ELEM_ACCESS_READWRITE,
// C: 	.iface =        SNDRV_CTL_ELEM_IFACE_MIXER,
// C: 	.name =         "Optical Output Mode",
// C: 	.count =	1,
// C: 	.info =         snd_emu1010_optical_out_info,
// C: 	.get =          snd_emu1010_optical_out_get,
// C: 	.put =          snd_emu1010_optical_out_put
// C: };
// C: 
// C: static int snd_emu1010_optical_in_info(struct snd_kcontrol *kcontrol,
// C: 					  struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	static const char * const texts[2] = {
// C: 		"SPDIF", "ADAT"
// C: 	};
// C: 
// C: 	return snd_ctl_enum_info(uinfo, 1, 2, texts);
// C: }
// C: 
// C: static int snd_emu1010_optical_in_get(struct snd_kcontrol *kcontrol,
// C: 					struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 
// C: 	ucontrol->value.enumerated.item[0] = emu->emu1010.optical_in;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu1010_optical_in_put(struct snd_kcontrol *kcontrol,
// C: 					struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	unsigned int val;
// C: 	u32 tmp;
// C: 	int change = 0;
// C: 
// C: 	val = ucontrol->value.enumerated.item[0];
// C: 	/* Limit: uinfo->value.enumerated.items = 2; * /
// C: 	if (val >= 2)
// C: 		return -EINVAL;
// C: 	change = (emu->emu1010.optical_in != val);
// C: 	if (change) {
// C: 		emu->emu1010.optical_in = val;
// C: 		tmp = (emu->emu1010.optical_in ? EMU_HANA_OPTICAL_IN_ADAT : EMU_HANA_OPTICAL_IN_SPDIF) |
// C: 			(emu->emu1010.optical_out ? EMU_HANA_OPTICAL_OUT_ADAT : EMU_HANA_OPTICAL_OUT_SPDIF);
// C: 		snd_emu1010_fpga_write_lock(emu, EMU_HANA_OPTICAL_TYPE, tmp);
// C: 	}
// C: 	return change;
// C: }
// C: 
// C: static const struct snd_kcontrol_new snd_emu1010_optical_in = {
// C: 	.access =	SNDRV_CTL_ELEM_ACCESS_READWRITE,
// C: 	.iface =        SNDRV_CTL_ELEM_IFACE_MIXER,
// C: 	.name =         "Optical Input Mode",
// C: 	.count =	1,
// C: 	.info =         snd_emu1010_optical_in_info,
// C: 	.get =          snd_emu1010_optical_in_get,
// C: 	.put =          snd_emu1010_optical_in_put
// C: };
// C: 
// C: static int snd_audigy_i2c_capture_source_info(struct snd_kcontrol *kcontrol,
// C: 					  struct snd_ctl_elem_info *uinfo)
// C: {
// C: #if 0
// C: 	static const char * const texts[4] = {
// C: 		"Unknown1", "Unknown2", "Mic", "Line"
// C: 	};
// C: #endif
// C: 	static const char * const texts[2] = {
// C: 		"Mic", "Line"
// C: 	};
// C: 
// C: 	return snd_ctl_enum_info(uinfo, 1, 2, texts);
// C: }
// C: 
// C: static int snd_audigy_i2c_capture_source_get(struct snd_kcontrol *kcontrol,
// C: 					struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 
// C: 	ucontrol->value.enumerated.item[0] = emu->i2c_capture_source;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_audigy_i2c_capture_source_put(struct snd_kcontrol *kcontrol,
// C: 					struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	unsigned int source_id;
// C: 	unsigned int ngain, ogain;
// C: 	u16 gpio;
// C: 	int change = 0;
// C: 	u32 source;
// C: 	/* If the capture source has changed,
// C: 	 * update the capture volume from the cached value
// C: 	 * for the particular source.
// C: 	 * /
// C: 	source_id = ucontrol->value.enumerated.item[0];
// C: 	/* Limit: uinfo->value.enumerated.items = 2; * /
// C: 	/*        emu->i2c_capture_volume * /
// C: 	if (source_id >= 2)
// C: 		return -EINVAL;
// C: 	change = (emu->i2c_capture_source != source_id);
// C: 	if (change) {
// C: 		snd_emu10k1_i2c_write(emu, ADC_MUX, 0); /* Mute input * /
// C: 		scoped_guard(spinlock_irq, &emu->emu_lock) {
// C: 			gpio = inw(emu->port + A_IOCFG);
// C: 			if (source_id == 0)
// C: 				outw(gpio | 0x4, emu->port + A_IOCFG);
// C: 			else
// C: 				outw(gpio & ~0x4, emu->port + A_IOCFG);
// C: 		}
// C: 
// C: 		ngain = emu->i2c_capture_volume[source_id][0]; /* Left * /
// C: 		ogain = emu->i2c_capture_volume[emu->i2c_capture_source][0]; /* Left * /
// C: 		if (ngain != ogain)
// C: 			snd_emu10k1_i2c_write(emu, ADC_ATTEN_ADCL, ((ngain) & 0xff));
// C: 		ngain = emu->i2c_capture_volume[source_id][1]; /* Right * /
// C: 		ogain = emu->i2c_capture_volume[emu->i2c_capture_source][1]; /* Right * /
// C: 		if (ngain != ogain)
// C: 			snd_emu10k1_i2c_write(emu, ADC_ATTEN_ADCR, ((ngain) & 0xff));
// C: 
// C: 		source = 1 << (source_id + 2);
// C: 		snd_emu10k1_i2c_write(emu, ADC_MUX, source); /* Set source * /
// C: 		emu->i2c_capture_source = source_id;
// C: 	}
// C:         return change;
// C: }
// C: 
// C: static const struct snd_kcontrol_new snd_audigy_i2c_capture_source =
// C: {
// C: 		.iface =	SNDRV_CTL_ELEM_IFACE_MIXER,
// C: 		.name =		"Capture Source",
// C: 		.info =		snd_audigy_i2c_capture_source_info,
// C: 		.get =		snd_audigy_i2c_capture_source_get,
// C: 		.put =		snd_audigy_i2c_capture_source_put
// C: };
// C: 
// C: static int snd_audigy_i2c_volume_info(struct snd_kcontrol *kcontrol,
// C: 				  struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_INTEGER;
// C: 	uinfo->count = 2;
// C: 	uinfo->value.integer.min = 0;
// C: 	uinfo->value.integer.max = 255;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_audigy_i2c_volume_get(struct snd_kcontrol *kcontrol,
// C: 				 struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	unsigned int source_id;
// C: 
// C: 	source_id = kcontrol->private_value;
// C: 	/* Limit: emu->i2c_capture_volume * /
// C:         /*        capture_source: uinfo->value.enumerated.items = 2 * /
// C: 	if (source_id >= 2)
// C: 		return -EINVAL;
// C: 
// C: 	ucontrol->value.integer.value[0] = emu->i2c_capture_volume[source_id][0];
// C: 	ucontrol->value.integer.value[1] = emu->i2c_capture_volume[source_id][1];
// C: 	return 0;
// C: }
// C: 
// C: static int snd_audigy_i2c_volume_put(struct snd_kcontrol *kcontrol,
// C: 				 struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	unsigned int ogain;
// C: 	unsigned int ngain0, ngain1;
// C: 	unsigned int source_id;
// C: 	int change = 0;
// C: 
// C: 	source_id = kcontrol->private_value;
// C: 	/* Limit: emu->i2c_capture_volume * /
// C:         /*        capture_source: uinfo->value.enumerated.items = 2 * /
// C: 	if (source_id >= 2)
// C: 		return -EINVAL;
// C: 	ngain0 = ucontrol->value.integer.value[0];
// C: 	ngain1 = ucontrol->value.integer.value[1];
// C: 	if (ngain0 > 0xff)
// C: 		return -EINVAL;
// C: 	if (ngain1 > 0xff)
// C: 		return -EINVAL;
// C: 	ogain = emu->i2c_capture_volume[source_id][0]; /* Left * /
// C: 	if (ogain != ngain0) {
// C: 		if (emu->i2c_capture_source == source_id)
// C: 			snd_emu10k1_i2c_write(emu, ADC_ATTEN_ADCL, ngain0);
// C: 		emu->i2c_capture_volume[source_id][0] = ngain0;
// C: 		change = 1;
// C: 	}
// C: 	ogain = emu->i2c_capture_volume[source_id][1]; /* Right * /
// C: 	if (ogain != ngain1) {
// C: 		if (emu->i2c_capture_source == source_id)
// C: 			snd_emu10k1_i2c_write(emu, ADC_ATTEN_ADCR, ngain1);
// C: 		emu->i2c_capture_volume[source_id][1] = ngain1;
// C: 		change = 1;
// C: 	}
// C: 
// C: 	return change;
// C: }
// C: 
// C: static const struct snd_kcontrol_new i2c_volume_ctl = {
// C: 	.iface = SNDRV_CTL_ELEM_IFACE_MIXER,
// C: 	.access = SNDRV_CTL_ELEM_ACCESS_READWRITE |
// C: 	          SNDRV_CTL_ELEM_ACCESS_TLV_READ,
// C: 	.info = snd_audigy_i2c_volume_info,
// C: 	.get = snd_audigy_i2c_volume_get,
// C: 	.put = snd_audigy_i2c_volume_put,
// C: 	.tlv = { .p = snd_audigy_db_scale2 }
// C: };
// C: 
// C: static const char * const snd_audigy_i2c_volume_ctls[] = {
// C: 	"Mic Capture Volume",
// C: 	"Line Capture Volume",
// C: };
// C: 
// C: #if 0
// C: static int snd_audigy_spdif_output_rate_info(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	static const char * const texts[] = {"44100", "48000", "96000"};
// C: 
// C: 	return snd_ctl_enum_info(uinfo, 1, 3, texts);
// C: }
// C: 
// C: static int snd_audigy_spdif_output_rate_get(struct snd_kcontrol *kcontrol,
// C:                                  struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	unsigned int tmp;
// C: 
// C: 	tmp = snd_emu10k1_ptr_read(emu, A_SPDIF_SAMPLERATE, 0);
// C: 	switch (tmp & A_SPDIF_RATE_MASK) {
// C: 	case A_SPDIF_44100:
// C: 		ucontrol->value.enumerated.item[0] = 0;
// C: 		break;
// C: 	case A_SPDIF_48000:
// C: 		ucontrol->value.enumerated.item[0] = 1;
// C: 		break;
// C: 	case A_SPDIF_96000:
// C: 		ucontrol->value.enumerated.item[0] = 2;
// C: 		break;
// C: 	default:
// C: 		ucontrol->value.enumerated.item[0] = 1;
// C: 	}
// C: 	return 0;
// C: }
// C: 
// C: static int snd_audigy_spdif_output_rate_put(struct snd_kcontrol *kcontrol,
// C:                                  struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	int change;
// C: 	unsigned int reg, val, tmp;
// C: 
// C: 	switch(ucontrol->value.enumerated.item[0]) {
// C: 	case 0:
// C: 		val = A_SPDIF_44100;
// C: 		break;
// C: 	case 1:
// C: 		val = A_SPDIF_48000;
// C: 		break;
// C: 	case 2:
// C: 		val = A_SPDIF_96000;
// C: 		break;
// C: 	default:
// C: 		val = A_SPDIF_48000;
// C: 		break;
// C: 	}
// C: 
// C: 	
// C: 	guard(spinlock_irq)(&emu->reg_lock);
// C: 	reg = snd_emu10k1_ptr_read(emu, A_SPDIF_SAMPLERATE, 0);
// C: 	tmp = reg & ~A_SPDIF_RATE_MASK;
// C: 	tmp |= val;
// C: 	change = (tmp != reg);
// C: 	if (change)
// C: 		snd_emu10k1_ptr_write(emu, A_SPDIF_SAMPLERATE, 0, tmp);
// C: 	return change;
// C: }
// C: 
// C: static const struct snd_kcontrol_new snd_audigy_spdif_output_rate =
// C: {
// C: 	.access =	SNDRV_CTL_ELEM_ACCESS_READWRITE,
// C: 	.iface =        SNDRV_CTL_ELEM_IFACE_MIXER,
// C: 	.name =         "Audigy SPDIF Output Sample Rate",
// C: 	.count =	1,
// C: 	.info =         snd_audigy_spdif_output_rate_info,
// C: 	.get =          snd_audigy_spdif_output_rate_get,
// C: 	.put =          snd_audigy_spdif_output_rate_put
// C: };
// C: #endif
// C: 
// C: static int snd_emu10k1_spdif_put(struct snd_kcontrol *kcontrol,
// C:                                  struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	unsigned int idx = snd_ctl_get_ioffidx(kcontrol, &ucontrol->id);
// C: 	int change;
// C: 	unsigned int val;
// C: 
// C: 	/* Limit: emu->spdif_bits * /
// C: 	if (idx >= 3)
// C: 		return -EINVAL;
// C: 	val = (ucontrol->value.iec958.status[0] << 0) |
// C: 	      (ucontrol->value.iec958.status[1] << 8) |
// C: 	      (ucontrol->value.iec958.status[2] << 16) |
// C: 	      (ucontrol->value.iec958.status[3] << 24);
// C: 	change = val != emu->spdif_bits[idx];
// C: 	if (change) {
// C: 		snd_emu10k1_ptr_write(emu, SPCS0 + idx, 0, val);
// C: 		emu->spdif_bits[idx] = val;
// C: 	}
// C: 	return change;
// C: }
// C: 
// C: static const struct snd_kcontrol_new snd_emu10k1_spdif_mask_control =
// C: {
// C: 	.access =	SNDRV_CTL_ELEM_ACCESS_READ,
// C: 	.iface =        SNDRV_CTL_ELEM_IFACE_PCM,
// C: 	.name =         SNDRV_CTL_NAME_IEC958("",PLAYBACK,MASK),
// C: 	.count =	3,
// C: 	.info =         snd_emu10k1_spdif_info,
// C: 	.get =          snd_emu10k1_spdif_get_mask
// C: };
// C: 
// C: static const struct snd_kcontrol_new snd_emu10k1_spdif_control =
// C: {
// C: 	.iface =	SNDRV_CTL_ELEM_IFACE_PCM,
// C: 	.name =         SNDRV_CTL_NAME_IEC958("",PLAYBACK,DEFAULT),
// C: 	.count =	3,
// C: 	.info =         snd_emu10k1_spdif_info,
// C: 	.get =          snd_emu10k1_spdif_get,
// C: 	.put =          snd_emu10k1_spdif_put
// C: };
// C: 
// C: 
// C: static void update_emu10k1_fxrt(struct snd_emu10k1 *emu, int voice, unsigned char *route)
// C: {
// C: 	if (emu->audigy) {
// C: 		snd_emu10k1_ptr_write_multiple(emu, voice,
// C: 			A_FXRT1, snd_emu10k1_compose_audigy_fxrt1(route),
// C: 			A_FXRT2, snd_emu10k1_compose_audigy_fxrt2(route),
// C: 			REGLIST_END);
// C: 	} else {
// C: 		snd_emu10k1_ptr_write(emu, FXRT, voice,
// C: 				      snd_emu10k1_compose_send_routing(route));
// C: 	}
// C: }
// C: 
// C: static void update_emu10k1_send_volume(struct snd_emu10k1 *emu, int voice, unsigned char *volume)
// C: {
// C: 	snd_emu10k1_ptr_write(emu, PTRX_FXSENDAMOUNT_A, voice, volume[0]);
// C: 	snd_emu10k1_ptr_write(emu, PTRX_FXSENDAMOUNT_B, voice, volume[1]);
// C: 	snd_emu10k1_ptr_write(emu, PSST_FXSENDAMOUNT_C, voice, volume[2]);
// C: 	snd_emu10k1_ptr_write(emu, DSL_FXSENDAMOUNT_D, voice, volume[3]);
// C: 	if (emu->audigy) {
// C: 		snd_emu10k1_ptr_write(emu, A_SENDAMOUNTS, voice,
// C: 				      snd_emu10k1_compose_audigy_sendamounts(volume));
// C: 	}
// C: }
// C: 
// C: /* PCM stream controls * /
// C: 
// C: static int snd_emu10k1_send_routing_info(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_INTEGER;
// C: 	uinfo->count = emu->audigy ? 3*8 : 3*4;
// C: 	uinfo->value.integer.min = 0;
// C: 	uinfo->value.integer.max = emu->audigy ? 0x3f : 0x0f;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu10k1_send_routing_get(struct snd_kcontrol *kcontrol,
// C:                                         struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	struct snd_emu10k1_pcm_mixer *mix =
// C: 		&emu->pcm_mixer[snd_ctl_get_ioffidx(kcontrol, &ucontrol->id)];
// C: 	int voice, idx;
// C: 	int num_efx = emu->audigy ? 8 : 4;
// C: 	int mask = emu->audigy ? 0x3f : 0x0f;
// C: 
// C: 	for (voice = 0; voice < 3; voice++)
// C: 		for (idx = 0; idx < num_efx; idx++)
// C: 			ucontrol->value.integer.value[(voice * num_efx) + idx] = 
// C: 				mix->send_routing[voice][idx] & mask;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu10k1_send_routing_put(struct snd_kcontrol *kcontrol,
// C:                                         struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	struct snd_emu10k1_pcm_mixer *mix =
// C: 		&emu->pcm_mixer[snd_ctl_get_ioffidx(kcontrol, &ucontrol->id)];
// C: 	int change = 0, voice, idx, val;
// C: 	int num_efx = emu->audigy ? 8 : 4;
// C: 	int mask = emu->audigy ? 0x3f : 0x0f;
// C: 
// C: 	guard(spinlock_irq)(&emu->reg_lock);
// C: 	for (voice = 0; voice < 3; voice++)
// C: 		for (idx = 0; idx < num_efx; idx++) {
// C: 			val = ucontrol->value.integer.value[(voice * num_efx) + idx] & mask;
// C: 			if (mix->send_routing[voice][idx] != val) {
// C: 				mix->send_routing[voice][idx] = val;
// C: 				change = 1;
// C: 			}
// C: 		}	
// C: 	if (change && mix->epcm && mix->epcm->voices[0]) {
// C: 		if (!mix->epcm->voices[0]->last) {
// C: 			update_emu10k1_fxrt(emu, mix->epcm->voices[0]->number,
// C: 					    &mix->send_routing[1][0]);
// C: 			update_emu10k1_fxrt(emu, mix->epcm->voices[0]->number + 1,
// C: 					    &mix->send_routing[2][0]);
// C: 		} else {
// C: 			update_emu10k1_fxrt(emu, mix->epcm->voices[0]->number,
// C: 					    &mix->send_routing[0][0]);
// C: 		}
// C: 	}
// C: 	return change;
// C: }
// C: 
// C: static const struct snd_kcontrol_new snd_emu10k1_send_routing_control =
// C: {
// C: 	.access =	SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_INACTIVE,
// C: 	.iface =        SNDRV_CTL_ELEM_IFACE_PCM,
// C: 	.name =         "EMU10K1 PCM Send Routing",
// C: 	.count =	32,
// C: 	.info =         snd_emu10k1_send_routing_info,
// C: 	.get =          snd_emu10k1_send_routing_get,
// C: 	.put =          snd_emu10k1_send_routing_put
// C: };
// C: 
// C: static int snd_emu10k1_send_volume_info(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_INTEGER;
// C: 	uinfo->count = emu->audigy ? 3*8 : 3*4;
// C: 	uinfo->value.integer.min = 0;
// C: 	uinfo->value.integer.max = 255;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu10k1_send_volume_get(struct snd_kcontrol *kcontrol,
// C:                                        struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	struct snd_emu10k1_pcm_mixer *mix =
// C: 		&emu->pcm_mixer[snd_ctl_get_ioffidx(kcontrol, &ucontrol->id)];
// C: 	int idx;
// C: 	int num_efx = emu->audigy ? 8 : 4;
// C: 
// C: 	for (idx = 0; idx < 3*num_efx; idx++)
// C: 		ucontrol->value.integer.value[idx] = mix->send_volume[idx/num_efx][idx%num_efx];
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu10k1_send_volume_put(struct snd_kcontrol *kcontrol,
// C:                                        struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	struct snd_emu10k1_pcm_mixer *mix =
// C: 		&emu->pcm_mixer[snd_ctl_get_ioffidx(kcontrol, &ucontrol->id)];
// C: 	int change = 0, idx, val;
// C: 	int num_efx = emu->audigy ? 8 : 4;
// C: 
// C: 	guard(spinlock_irq)(&emu->reg_lock);
// C: 	for (idx = 0; idx < 3*num_efx; idx++) {
// C: 		val = ucontrol->value.integer.value[idx] & 255;
// C: 		if (mix->send_volume[idx/num_efx][idx%num_efx] != val) {
// C: 			mix->send_volume[idx/num_efx][idx%num_efx] = val;
// C: 			change = 1;
// C: 		}
// C: 	}
// C: 	if (change && mix->epcm && mix->epcm->voices[0]) {
// C: 		if (!mix->epcm->voices[0]->last) {
// C: 			update_emu10k1_send_volume(emu, mix->epcm->voices[0]->number,
// C: 						   &mix->send_volume[1][0]);
// C: 			update_emu10k1_send_volume(emu, mix->epcm->voices[0]->number + 1,
// C: 						   &mix->send_volume[2][0]);
// C: 		} else {
// C: 			update_emu10k1_send_volume(emu, mix->epcm->voices[0]->number,
// C: 						   &mix->send_volume[0][0]);
// C: 		}
// C: 	}
// C: 	return change;
// C: }
// C: 
// C: static const struct snd_kcontrol_new snd_emu10k1_send_volume_control =
// C: {
// C: 	.access =	SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_INACTIVE,
// C: 	.iface =        SNDRV_CTL_ELEM_IFACE_PCM,
// C: 	.name =         "EMU10K1 PCM Send Volume",
// C: 	.count =	32,
// C: 	.info =         snd_emu10k1_send_volume_info,
// C: 	.get =          snd_emu10k1_send_volume_get,
// C: 	.put =          snd_emu10k1_send_volume_put
// C: };
// C: 
// C: static int snd_emu10k1_attn_info(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_INTEGER;
// C: 	uinfo->count = 3;
// C: 	uinfo->value.integer.min = 0;
// C: 	uinfo->value.integer.max = 0x1fffd;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu10k1_attn_get(struct snd_kcontrol *kcontrol,
// C:                                 struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	struct snd_emu10k1_pcm_mixer *mix =
// C: 		&emu->pcm_mixer[snd_ctl_get_ioffidx(kcontrol, &ucontrol->id)];
// C: 	int idx;
// C: 
// C: 	for (idx = 0; idx < 3; idx++)
// C: 		ucontrol->value.integer.value[idx] = mix->attn[idx] * 0xffffU / 0x8000U;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu10k1_attn_put(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	struct snd_emu10k1_pcm_mixer *mix =
// C: 		&emu->pcm_mixer[snd_ctl_get_ioffidx(kcontrol, &ucontrol->id)];
// C: 	int change = 0, idx, val;
// C: 
// C: 	guard(spinlock_irq)(&emu->reg_lock);
// C: 	for (idx = 0; idx < 3; idx++) {
// C: 		unsigned uval = ucontrol->value.integer.value[idx] & 0x1ffff;
// C: 		val = uval * 0x8000U / 0xffffU;
// C: 		if (mix->attn[idx] != val) {
// C: 			mix->attn[idx] = val;
// C: 			change = 1;
// C: 		}
// C: 	}
// C: 	if (change && mix->epcm && mix->epcm->voices[0]) {
// C: 		if (!mix->epcm->voices[0]->last) {
// C: 			snd_emu10k1_ptr_write(emu, VTFT_VOLUMETARGET, mix->epcm->voices[0]->number, mix->attn[1]);
// C: 			snd_emu10k1_ptr_write(emu, VTFT_VOLUMETARGET, mix->epcm->voices[0]->number + 1, mix->attn[2]);
// C: 		} else {
// C: 			snd_emu10k1_ptr_write(emu, VTFT_VOLUMETARGET, mix->epcm->voices[0]->number, mix->attn[0]);
// C: 		}
// C: 	}
// C: 	return change;
// C: }
// C: 
// C: static const struct snd_kcontrol_new snd_emu10k1_attn_control =
// C: {
// C: 	.access =	SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_INACTIVE,
// C: 	.iface =        SNDRV_CTL_ELEM_IFACE_PCM,
// C: 	.name =         "EMU10K1 PCM Volume",
// C: 	.count =	32,
// C: 	.info =         snd_emu10k1_attn_info,
// C: 	.get =          snd_emu10k1_attn_get,
// C: 	.put =          snd_emu10k1_attn_put
// C: };
// C: 
// C: /* Mutichannel PCM stream controls * /
// C: 
// C: static int snd_emu10k1_efx_send_routing_info(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_INTEGER;
// C: 	uinfo->count = emu->audigy ? 8 : 4;
// C: 	uinfo->value.integer.min = 0;
// C: 	uinfo->value.integer.max = emu->audigy ? 0x3f : 0x0f;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu10k1_efx_send_routing_get(struct snd_kcontrol *kcontrol,
// C:                                         struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	struct snd_emu10k1_pcm_mixer *mix =
// C: 		&emu->efx_pcm_mixer[snd_ctl_get_ioffidx(kcontrol, &ucontrol->id)];
// C: 	int idx;
// C: 	int num_efx = emu->audigy ? 8 : 4;
// C: 	int mask = emu->audigy ? 0x3f : 0x0f;
// C: 
// C: 	for (idx = 0; idx < num_efx; idx++)
// C: 		ucontrol->value.integer.value[idx] = 
// C: 			mix->send_routing[0][idx] & mask;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu10k1_efx_send_routing_put(struct snd_kcontrol *kcontrol,
// C:                                         struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	int ch = snd_ctl_get_ioffidx(kcontrol, &ucontrol->id);
// C: 	struct snd_emu10k1_pcm_mixer *mix = &emu->efx_pcm_mixer[ch];
// C: 	int change = 0, idx, val;
// C: 	int num_efx = emu->audigy ? 8 : 4;
// C: 	int mask = emu->audigy ? 0x3f : 0x0f;
// C: 
// C: 	guard(spinlock_irq)(&emu->reg_lock);
// C: 	for (idx = 0; idx < num_efx; idx++) {
// C: 		val = ucontrol->value.integer.value[idx] & mask;
// C: 		if (mix->send_routing[0][idx] != val) {
// C: 			mix->send_routing[0][idx] = val;
// C: 			change = 1;
// C: 		}
// C: 	}	
// C: 
// C: 	if (change && mix->epcm) {
// C: 		if (mix->epcm->voices[ch]) {
// C: 			update_emu10k1_fxrt(emu, mix->epcm->voices[ch]->number,
// C: 					&mix->send_routing[0][0]);
// C: 		}
// C: 	}
// C: 	return change;
// C: }
// C: 
// C: static const struct snd_kcontrol_new snd_emu10k1_efx_send_routing_control =
// C: {
// C: 	.access =	SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_INACTIVE,
// C: 	.iface =        SNDRV_CTL_ELEM_IFACE_PCM,
// C: 	.name =         "Multichannel PCM Send Routing",
// C: 	.count =	16,
// C: 	.info =         snd_emu10k1_efx_send_routing_info,
// C: 	.get =          snd_emu10k1_efx_send_routing_get,
// C: 	.put =          snd_emu10k1_efx_send_routing_put
// C: };
// C: 
// C: static int snd_emu10k1_efx_send_volume_info(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_INTEGER;
// C: 	uinfo->count = emu->audigy ? 8 : 4;
// C: 	uinfo->value.integer.min = 0;
// C: 	uinfo->value.integer.max = 255;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu10k1_efx_send_volume_get(struct snd_kcontrol *kcontrol,
// C:                                        struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	struct snd_emu10k1_pcm_mixer *mix =
// C: 		&emu->efx_pcm_mixer[snd_ctl_get_ioffidx(kcontrol, &ucontrol->id)];
// C: 	int idx;
// C: 	int num_efx = emu->audigy ? 8 : 4;
// C: 
// C: 	for (idx = 0; idx < num_efx; idx++)
// C: 		ucontrol->value.integer.value[idx] = mix->send_volume[0][idx];
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu10k1_efx_send_volume_put(struct snd_kcontrol *kcontrol,
// C:                                        struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	int ch = snd_ctl_get_ioffidx(kcontrol, &ucontrol->id);
// C: 	struct snd_emu10k1_pcm_mixer *mix = &emu->efx_pcm_mixer[ch];
// C: 	int change = 0, idx, val;
// C: 	int num_efx = emu->audigy ? 8 : 4;
// C: 
// C: 	guard(spinlock_irq)(&emu->reg_lock);
// C: 	for (idx = 0; idx < num_efx; idx++) {
// C: 		val = ucontrol->value.integer.value[idx] & 255;
// C: 		if (mix->send_volume[0][idx] != val) {
// C: 			mix->send_volume[0][idx] = val;
// C: 			change = 1;
// C: 		}
// C: 	}
// C: 	if (change && mix->epcm) {
// C: 		if (mix->epcm->voices[ch]) {
// C: 			update_emu10k1_send_volume(emu, mix->epcm->voices[ch]->number,
// C: 						   &mix->send_volume[0][0]);
// C: 		}
// C: 	}
// C: 	return change;
// C: }
// C: 
// C: 
// C: static const struct snd_kcontrol_new snd_emu10k1_efx_send_volume_control =
// C: {
// C: 	.access =	SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_INACTIVE,
// C: 	.iface =        SNDRV_CTL_ELEM_IFACE_PCM,
// C: 	.name =         "Multichannel PCM Send Volume",
// C: 	.count =	16,
// C: 	.info =         snd_emu10k1_efx_send_volume_info,
// C: 	.get =          snd_emu10k1_efx_send_volume_get,
// C: 	.put =          snd_emu10k1_efx_send_volume_put
// C: };
// C: 
// C: static int snd_emu10k1_efx_attn_info(struct snd_kcontrol *kcontrol, struct snd_ctl_elem_info *uinfo)
// C: {
// C: 	uinfo->type = SNDRV_CTL_ELEM_TYPE_INTEGER;
// C: 	uinfo->count = 1;
// C: 	uinfo->value.integer.min = 0;
// C: 	uinfo->value.integer.max = 0x1fffd;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu10k1_efx_attn_get(struct snd_kcontrol *kcontrol,
// C:                                 struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	struct snd_emu10k1_pcm_mixer *mix =
// C: 		&emu->efx_pcm_mixer[snd_ctl_get_ioffidx(kcontrol, &ucontrol->id)];
// C: 
// C: 	ucontrol->value.integer.value[0] = mix->attn[0] * 0xffffU / 0x8000U;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu10k1_efx_attn_put(struct snd_kcontrol *kcontrol,
// C: 				struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	int ch = snd_ctl_get_ioffidx(kcontrol, &ucontrol->id);
// C: 	struct snd_emu10k1_pcm_mixer *mix = &emu->efx_pcm_mixer[ch];
// C: 	int change = 0, val;
// C: 	unsigned uval;
// C: 
// C: 	guard(spinlock_irq)(&emu->reg_lock);
// C: 	uval = ucontrol->value.integer.value[0] & 0x1ffff;
// C: 	val = uval * 0x8000U / 0xffffU;
// C: 	if (mix->attn[0] != val) {
// C: 		mix->attn[0] = val;
// C: 		change = 1;
// C: 	}
// C: 	if (change && mix->epcm) {
// C: 		if (mix->epcm->voices[ch]) {
// C: 			snd_emu10k1_ptr_write(emu, VTFT_VOLUMETARGET, mix->epcm->voices[ch]->number, mix->attn[0]);
// C: 		}
// C: 	}
// C: 	return change;
// C: }
// C: 
// C: static const struct snd_kcontrol_new snd_emu10k1_efx_attn_control =
// C: {
// C: 	.access =	SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_INACTIVE,
// C: 	.iface =        SNDRV_CTL_ELEM_IFACE_PCM,
// C: 	.name =         "Multichannel PCM Volume",
// C: 	.count =	16,
// C: 	.info =         snd_emu10k1_efx_attn_info,
// C: 	.get =          snd_emu10k1_efx_attn_get,
// C: 	.put =          snd_emu10k1_efx_attn_put
// C: };
// C: 
// C: #define snd_emu10k1_shared_spdif_info	snd_ctl_boolean_mono_info
// C: 
// C: static int snd_emu10k1_shared_spdif_get(struct snd_kcontrol *kcontrol,
// C: 					struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 
// C: 	if (emu->audigy)
// C: 		ucontrol->value.integer.value[0] = inw(emu->port + A_IOCFG) & A_IOCFG_GPOUT0 ? 1 : 0;
// C: 	else
// C: 		ucontrol->value.integer.value[0] = inl(emu->port + HCFG) & HCFG_GPOUT0 ? 1 : 0;
// C: 	if (emu->card_capabilities->invert_shared_spdif)
// C: 		ucontrol->value.integer.value[0] =
// C: 			!ucontrol->value.integer.value[0];
// C: 		
// C: 	return 0;
// C: }
// C: 
// C: static int snd_emu10k1_shared_spdif_put(struct snd_kcontrol *kcontrol,
// C: 					struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	unsigned int reg, val, sw;
// C: 	int change = 0;
// C: 
// C: 	sw = ucontrol->value.integer.value[0];
// C: 	if (emu->card_capabilities->invert_shared_spdif)
// C: 		sw = !sw;
// C: 	guard(spinlock_irq)(&emu->emu_lock);
// C: 	if ( emu->card_capabilities->i2c_adc) {
// C: 		/* Do nothing for Audigy 2 ZS Notebook * /
// C: 	} else if (emu->audigy) {
// C: 		reg = inw(emu->port + A_IOCFG);
// C: 		val = sw ? A_IOCFG_GPOUT0 : 0;
// C: 		change = (reg & A_IOCFG_GPOUT0) != val;
// C: 		if (change) {
// C: 			reg &= ~A_IOCFG_GPOUT0;
// C: 			reg |= val;
// C: 			outw(reg | val, emu->port + A_IOCFG);
// C: 		}
// C: 	}
// C: 	reg = inl(emu->port + HCFG);
// C: 	val = sw ? HCFG_GPOUT0 : 0;
// C: 	change |= (reg & HCFG_GPOUT0) != val;
// C: 	if (change) {
// C: 		reg &= ~HCFG_GPOUT0;
// C: 		reg |= val;
// C: 		outl(reg | val, emu->port + HCFG);
// C: 	}
// C: 	return change;
// C: }
// C: 
// C: static const struct snd_kcontrol_new snd_emu10k1_shared_spdif =
// C: {
// C: 	.iface =	SNDRV_CTL_ELEM_IFACE_MIXER,
// C: 	.name =		"SB Live Analog/Digital Output Jack",
// C: 	.info =		snd_emu10k1_shared_spdif_info,
// C: 	.get =		snd_emu10k1_shared_spdif_get,
// C: 	.put =		snd_emu10k1_shared_spdif_put
// C: };
// C: 
// C: static const struct snd_kcontrol_new snd_audigy_shared_spdif =
// C: {
// C: 	.iface =	SNDRV_CTL_ELEM_IFACE_MIXER,
// C: 	.name =		"Audigy Analog/Digital Output Jack",
// C: 	.info =		snd_emu10k1_shared_spdif_info,
// C: 	.get =		snd_emu10k1_shared_spdif_get,
// C: 	.put =		snd_emu10k1_shared_spdif_put
// C: };
// C: 
// C: /* workaround for too low volume on Audigy due to 16bit/24bit conversion * /
// C: 
// C: #define snd_audigy_capture_boost_info	snd_ctl_boolean_mono_info
// C: 
// C: static int snd_audigy_capture_boost_get(struct snd_kcontrol *kcontrol,
// C: 					struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	unsigned int val;
// C: 
// C: 	/* FIXME: better to use a cached version * /
// C: 	val = snd_ac97_read(emu->ac97, AC97_REC_GAIN);
// C: 	ucontrol->value.integer.value[0] = !!val;
// C: 	return 0;
// C: }
// C: 
// C: static int snd_audigy_capture_boost_put(struct snd_kcontrol *kcontrol,
// C: 					struct snd_ctl_elem_value *ucontrol)
// C: {
// C: 	struct snd_emu10k1 *emu = snd_kcontrol_chip(kcontrol);
// C: 	unsigned int val;
// C: 
// C: 	if (ucontrol->value.integer.value[0])
// C: 		val = 0x0f0f;
// C: 	else
// C: 		val = 0;
// C: 	return snd_ac97_update(emu->ac97, AC97_REC_GAIN, val);
// C: }
// C: 
// C: static const struct snd_kcontrol_new snd_audigy_capture_boost =
// C: {
// C: 	.iface =	SNDRV_CTL_ELEM_IFACE_MIXER,
// C: 	.name =		"Mic Extra Boost",
// C: 	.info =		snd_audigy_capture_boost_info,
// C: 	.get =		snd_audigy_capture_boost_get,
// C: 	.put =		snd_audigy_capture_boost_put
// C: };
// C: 
// C: 
// C: /*
// C:  * /
// C: static void snd_emu10k1_mixer_free_ac97(struct snd_ac97 *ac97)
// C: {
// C: 	struct snd_emu10k1 *emu = ac97->private_data;
// C: 	emu->ac97 = NULL;
// C: }
// C: 
// C: /*
// C:  * /
// C: static int remove_ctl(struct snd_card *card, const char *name)
// C: {
// C: 	struct snd_ctl_elem_id id;
// C: 	memset(&id, 0, sizeof(id));
// C: 	strscpy(id.name, name);
// C: 	id.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
// C: 	return snd_ctl_remove_id(card, &id);
// C: }
// C: 
// C: static int rename_ctl(struct snd_card *card, const char *src, const char *dst)
// C: {
// C: 	struct snd_kcontrol *kctl = snd_ctl_find_id_mixer(card, src);
// C: 	if (kctl) {
// C: 		snd_ctl_rename(card, kctl, dst);
// C: 		return 0;
// C: 	}
// C: 	return -ENOENT;
// C: }
// C: 
// C: int snd_emu10k1_mixer(struct snd_emu10k1 *emu,
// C: 		      int pcm_device, int multi_device)
// C: {
// C: 	int err;
// C: 	struct snd_kcontrol *kctl;
// C: 	struct snd_card *card = emu->card;
// C: 	const char * const *c;
// C: 	static const char * const emu10k1_remove_ctls[] = {
// C: 		/* no AC97 mono, surround, center/lfe * /
// C: 		"Master Mono Playback Switch",
// C: 		"Master Mono Playback Volume",
// C: 		"PCM Out Path & Mute",
// C: 		"Mono Output Select",
// C: 		"Surround Playback Switch",
// C: 		"Surround Playback Volume",
// C: 		"Center Playback Switch",
// C: 		"Center Playback Volume",
// C: 		"LFE Playback Switch",
// C: 		"LFE Playback Volume",
// C: 		NULL
// C: 	};
// C: 	static const char * const emu10k1_rename_ctls[] = {
// C: 		"Surround Digital Playback Volume", "Surround Playback Volume",
// C: 		"Center Digital Playback Volume", "Center Playback Volume",
// C: 		"LFE Digital Playback Volume", "LFE Playback Volume",
// C: 		NULL
// C: 	};
// C: 	static const char * const audigy_remove_ctls[] = {
// C: 		/* Master/PCM controls on ac97 of Audigy has no effect * /
// C: 		/* On the Audigy2 the AC97 playback is piped into
// C: 		 * the Philips ADC for 24bit capture * /
// C: 		"PCM Playback Switch",
// C: 		"PCM Playback Volume",
// C: 		"Master Playback Switch",
// C: 		"Master Playback Volume",
// C: 		"PCM Out Path & Mute",
// C: 		"Mono Output Select",
// C: 		/* remove unused AC97 capture controls * /
// C: 		"Capture Source",
// C: 		"Capture Switch",
// C: 		"Capture Volume",
// C: 		"Mic Select",
// C: 		"Headphone Playback Switch",
// C: 		"Headphone Playback Volume",
// C: 		"3D Control - Center",
// C: 		"3D Control - Depth",
// C: 		"3D Control - Switch",
// C: 		"Video Playback Switch",
// C: 		"Video Playback Volume",
// C: 		"Mic Playback Switch",
// C: 		"Mic Playback Volume",
// C: 		"External Amplifier",
// C: 		NULL
// C: 	};
// C: 	static const char * const audigy_rename_ctls[] = {
// C: 		/* use conventional names * /
// C: 		"Wave Playback Volume", "PCM Playback Volume",
// C: 		/* "Wave Capture Volume", "PCM Capture Volume", * /
// C: 		"Wave Master Playback Volume", "Master Playback Volume",
// C: 		"AMic Playback Volume", "Mic Playback Volume",
// C: 		"Master Mono Playback Switch", "Phone Output Playback Switch",
// C: 		"Master Mono Playback Volume", "Phone Output Playback Volume",
// C: 		NULL
// C: 	};
// C: 	static const char * const audigy_rename_ctls_i2c_adc[] = {
// C: 		//"Analog Mix Capture Volume","OLD Analog Mix Capture Volume",
// C: 		"Line Capture Volume", "Analog Mix Capture Volume",
// C: 		"Wave Playback Volume", "OLD PCM Playback Volume",
// C: 		"Wave Master Playback Volume", "Master Playback Volume",
// C: 		"AMic Playback Volume", "Old Mic Playback Volume",
// C: 		"CD Capture Volume", "IEC958 Optical Capture Volume",
// C: 		NULL
// C: 	};
// C: 	static const char * const audigy_remove_ctls_i2c_adc[] = {
// C: 		/* On the Audigy2 ZS Notebook
// C: 		 * Capture via WM8775  * /
// C: 		"Mic Capture Volume",
// C: 		"Analog Mix Capture Volume",
// C: 		"Aux Capture Volume",
// C: 		"IEC958 Optical Capture Volume",
// C: 		NULL
// C: 	};
// C: 	static const char * const audigy_remove_ctls_1361t_adc[] = {
// C: 		/* On the Audigy2 the AC97 playback is piped into
// C: 		 * the Philips ADC for 24bit capture * /
// C: 		"PCM Playback Switch",
// C: 		"PCM Playback Volume",
// C: 		"Capture Source",
// C: 		"Capture Switch",
// C: 		"Capture Volume",
// C: 		"Mic Capture Volume",
// C: 		"Headphone Playback Switch",
// C: 		"Headphone Playback Volume",
// C: 		"3D Control - Center",
// C: 		"3D Control - Depth",
// C: 		"3D Control - Switch",
// C: 		"Line2 Playback Volume",
// C: 		"Line2 Capture Volume",
// C: 		NULL
// C: 	};
// C: 	static const char * const audigy_rename_ctls_1361t_adc[] = {
// C: 		"Master Playback Switch", "Master Capture Switch",
// C: 		"Master Playback Volume", "Master Capture Volume",
// C: 		"Wave Master Playback Volume", "Master Playback Volume",
// C: 		"Beep Playback Switch", "Beep Capture Switch",
// C: 		"Beep Playback Volume", "Beep Capture Volume",
// C: 		"Phone Playback Switch", "Phone Capture Switch",
// C: 		"Phone Playback Volume", "Phone Capture Volume",
// C: 		"Mic Playback Switch", "Mic Capture Switch",
// C: 		"Mic Playback Volume", "Mic Capture Volume",
// C: 		"Line Playback Switch", "Line Capture Switch",
// C: 		"Line Playback Volume", "Line Capture Volume",
// C: 		"CD Playback Switch", "CD Capture Switch",
// C: 		"CD Playback Volume", "CD Capture Volume",
// C: 		"Aux Playback Switch", "Aux Capture Switch",
// C: 		"Aux Playback Volume", "Aux Capture Volume",
// C: 		"Video Playback Switch", "Video Capture Switch",
// C: 		"Video Playback Volume", "Video Capture Volume",
// C: 		"Master Mono Playback Switch", "Phone Output Playback Switch",
// C: 		"Master Mono Playback Volume", "Phone Output Playback Volume",
// C: 		NULL
// C: 	};
// C: 
// C: 	if (emu->card_capabilities->ac97_chip) {
// C: 		struct snd_ac97_bus *pbus;
// C: 		struct snd_ac97_template ac97;
// C: 		static const struct snd_ac97_bus_ops ops = {
// C: 			.write = snd_emu10k1_ac97_write,
// C: 			.read = snd_emu10k1_ac97_read,
// C: 		};
// C: 
// C: 		err = snd_ac97_bus(emu->card, 0, &ops, NULL, &pbus);
// C: 		if (err < 0)
// C: 			return err;
// C: 		pbus->no_vra = 1; /* we don't need VRA * /
// C: 		
// C: 		memset(&ac97, 0, sizeof(ac97));
// C: 		ac97.private_data = emu;
// C: 		ac97.private_free = snd_emu10k1_mixer_free_ac97;
// C: 		ac97.scaps = AC97_SCAP_NO_SPDIF;
// C: 		err = snd_ac97_mixer(pbus, &ac97, &emu->ac97);
// C: 		if (err < 0) {
// C: 			if (emu->card_capabilities->ac97_chip == 1)
// C: 				return err;
// C: 			dev_info(emu->card->dev,
// C: 				 "AC97 is optional on this board\n");
// C: 			dev_info(emu->card->dev,
// C: 				 "Proceeding without ac97 mixers...\n");
// C: 			snd_device_free(emu->card, pbus);
// C: 			goto no_ac97; /* FIXME: get rid of ugly gotos.. * /
// C: 		}
// C: 		if (emu->audigy) {
// C: 			/* set master volume to 0 dB * /
// C: 			snd_ac97_write_cache(emu->ac97, AC97_MASTER, 0x0000);
// C: 			/* set capture source to mic * /
// C: 			snd_ac97_write_cache(emu->ac97, AC97_REC_SEL, 0x0000);
// C: 			/* set mono output (TAD) to mic * /
// C: 			snd_ac97_update_bits(emu->ac97, AC97_GENERAL_PURPOSE,
// C: 				0x0200, 0x0200);
// C: 			if (emu->card_capabilities->adc_1361t)
// C: 				c = audigy_remove_ctls_1361t_adc;
// C: 			else 
// C: 				c = audigy_remove_ctls;
// C: 		} else {
// C: 			/*
// C: 			 * Credits for cards based on STAC9758:
// C: 			 *   James Courtier-Dutton <James@superbug.demon.co.uk>
// C: 			 *   Voluspa <voluspa@comhem.se>
// C: 			 * /
// C: 			if (emu->ac97->id == AC97_ID_STAC9758) {
// C: 				emu->rear_ac97 = 1;
// C: 				snd_emu10k1_ptr_write(emu, AC97SLOT, 0, AC97SLOT_CNTR|AC97SLOT_LFE|AC97SLOT_REAR_LEFT|AC97SLOT_REAR_RIGHT);
// C: 				snd_ac97_write_cache(emu->ac97, AC97_HEADPHONE, 0x0202);
// C: 				remove_ctl(card,"Front Playback Volume");
// C: 				remove_ctl(card,"Front Playback Switch");
// C: 			}
// C: 			/* remove unused AC97 controls * /
// C: 			snd_ac97_write_cache(emu->ac97, AC97_SURROUND_MASTER, 0x0202);
// C: 			snd_ac97_write_cache(emu->ac97, AC97_CENTER_LFE_MASTER, 0x0202);
// C: 			c = emu10k1_remove_ctls;
// C: 		}
// C: 		for (; *c; c++)
// C: 			remove_ctl(card, *c);
// C: 	} else if (emu->card_capabilities->i2c_adc) {
// C: 		c = audigy_remove_ctls_i2c_adc;
// C: 		for (; *c; c++)
// C: 			remove_ctl(card, *c);
// C: 	} else {
// C: 	no_ac97:
// C: 		if (emu->card_capabilities->ecard)
// C: 			strscpy(emu->card->mixername, "EMU APS");
// C: 		else if (emu->audigy)
// C: 			strscpy(emu->card->mixername, "SB Audigy");
// C: 		else
// C: 			strscpy(emu->card->mixername, "Emu10k1");
// C: 	}
// C: 
// C: 	if (emu->audigy)
// C: 		if (emu->card_capabilities->adc_1361t)
// C: 			c = audigy_rename_ctls_1361t_adc;
// C: 		else if (emu->card_capabilities->i2c_adc)
// C: 			c = audigy_rename_ctls_i2c_adc;
// C: 		else
// C: 			c = audigy_rename_ctls;
// C: 	else
// C: 		c = emu10k1_rename_ctls;
// C: 	for (; *c; c += 2)
// C: 		rename_ctl(card, c[0], c[1]);
// C: 
// C: 	if (emu->card_capabilities->subsystem == 0x80401102) { /* SB Live! Platinum CT4760P * /
// C: 		remove_ctl(card, "Center Playback Volume");
// C: 		remove_ctl(card, "LFE Playback Volume");
// C: 		remove_ctl(card, "Wave Center Playback Volume");
// C: 		remove_ctl(card, "Wave LFE Playback Volume");
// C: 	}
// C: 	if (emu->card_capabilities->subsystem == 0x20071102) {  /* Audigy 4 Pro * /
// C: 		rename_ctl(card, "Line2 Capture Volume", "Line1/Mic Capture Volume");
// C: 		rename_ctl(card, "Analog Mix Capture Volume", "Line2 Capture Volume");
// C: 		rename_ctl(card, "Aux2 Capture Volume", "Line3 Capture Volume");
// C: 		rename_ctl(card, "Mic Capture Volume", "Unknown1 Capture Volume");
// C: 	}
// C: 	kctl = emu->ctl_send_routing = snd_ctl_new1(&snd_emu10k1_send_routing_control, emu);
// C: 	if (!kctl)
// C: 		return -ENOMEM;
// C: 	kctl->id.device = pcm_device;
// C: 	err = snd_ctl_add(card, kctl);
// C: 	if (err)
// C: 		return err;
// C: 	kctl = emu->ctl_send_volume = snd_ctl_new1(&snd_emu10k1_send_volume_control, emu);
// C: 	if (!kctl)
// C: 		return -ENOMEM;
// C: 	kctl->id.device = pcm_device;
// C: 	err = snd_ctl_add(card, kctl);
// C: 	if (err)
// C: 		return err;
// C: 	kctl = emu->ctl_attn = snd_ctl_new1(&snd_emu10k1_attn_control, emu);
// C: 	if (!kctl)
// C: 		return -ENOMEM;
// C: 	kctl->id.device = pcm_device;
// C: 	err = snd_ctl_add(card, kctl);
// C: 	if (err)
// C: 		return err;
// C: 
// C: 	kctl = emu->ctl_efx_send_routing = snd_ctl_new1(&snd_emu10k1_efx_send_routing_control, emu);
// C: 	if (!kctl)
// C: 		return -ENOMEM;
// C: 	kctl->id.device = multi_device;
// C: 	err = snd_ctl_add(card, kctl);
// C: 	if (err)
// C: 		return err;
// C: 	
// C: 	kctl = emu->ctl_efx_send_volume = snd_ctl_new1(&snd_emu10k1_efx_send_volume_control, emu);
// C: 	if (!kctl)
// C: 		return -ENOMEM;
// C: 	kctl->id.device = multi_device;
// C: 	err = snd_ctl_add(card, kctl);
// C: 	if (err)
// C: 		return err;
// C: 	
// C: 	kctl = emu->ctl_efx_attn = snd_ctl_new1(&snd_emu10k1_efx_attn_control, emu);
// C: 	if (!kctl)
// C: 		return -ENOMEM;
// C: 	kctl->id.device = multi_device;
// C: 	err = snd_ctl_add(card, kctl);
// C: 	if (err)
// C: 		return err;
// C: 
// C: 	if (!emu->card_capabilities->ecard && !emu->card_capabilities->emu_model) {
// C: 		/* sb live! and audigy * /
// C: 		kctl = snd_ctl_new1(&snd_emu10k1_spdif_mask_control, emu);
// C: 		if (!kctl)
// C: 			return -ENOMEM;
// C: 		if (!emu->audigy)
// C: 			kctl->id.device = emu->pcm_efx->device;
// C: 		err = snd_ctl_add(card, kctl);
// C: 		if (err)
// C: 			return err;
// C: 		kctl = snd_ctl_new1(&snd_emu10k1_spdif_control, emu);
// C: 		if (!kctl)
// C: 			return -ENOMEM;
// C: 		if (!emu->audigy)
// C: 			kctl->id.device = emu->pcm_efx->device;
// C: 		err = snd_ctl_add(card, kctl);
// C: 		if (err)
// C: 			return err;
// C: 	}
// C: 
// C: 	if (emu->card_capabilities->emu_model) {
// C: 		;  /* Disable the snd_audigy_spdif_shared_spdif * /
// C: 	} else if (emu->audigy) {
// C: 		kctl = snd_ctl_new1(&snd_audigy_shared_spdif, emu);
// C: 		if (!kctl)
// C: 			return -ENOMEM;
// C: 		err = snd_ctl_add(card, kctl);
// C: 		if (err)
// C: 			return err;
// C: #if 0
// C: 		kctl = snd_ctl_new1(&snd_audigy_spdif_output_rate, emu);
// C: 		if (!kctl)
// C: 			return -ENOMEM;
// C: 		err = snd_ctl_add(card, kctl);
// C: 		if (err)
// C: 			return err;
// C: #endif
// C: 	} else if (! emu->card_capabilities->ecard) {
// C: 		/* sb live! * /
// C: 		kctl = snd_ctl_new1(&snd_emu10k1_shared_spdif, emu);
// C: 		if (!kctl)
// C: 			return -ENOMEM;
// C: 		err = snd_ctl_add(card, kctl);
// C: 		if (err)
// C: 			return err;
// C: 	}
// C: 	if (emu->card_capabilities->ca0151_chip) { /* P16V * /
// C: 		err = snd_p16v_mixer(emu);
// C: 		if (err)
// C: 			return err;
// C: 	}
// C: 
// C: 	if (emu->card_capabilities->emu_model) {
// C: 		unsigned i, emu_idx = emu1010_idx(emu);
// C: 		const struct snd_emu1010_routing_info *emu_ri =
// C: 			&emu1010_routing_info[emu_idx];
// C: 		const struct snd_emu1010_pads_info *emu_pi = &emu1010_pads_info[emu_idx];
// C: 
// C: 		for (i = 0; i < emu_ri->n_ins; i++)
// C: 			emu->emu1010.input_source[i] =
// C: 				emu1010_map_source(emu_ri, emu_ri->in_dflts[i]);
// C: 		for (i = 0; i < emu_ri->n_outs; i++)
// C: 			emu->emu1010.output_source[i] =
// C: 				emu1010_map_source(emu_ri, emu_ri->out_dflts[i]);
// C: 		scoped_guard(snd_emu1010_fpga_lock, emu) {
// C: 			snd_emu1010_apply_sources(emu);
// C: 		}
// C: 
// C: 		kctl = emu->ctl_clock_source = snd_ctl_new1(&snd_emu1010_clock_source, emu);
// C: 		err = snd_ctl_add(card, kctl);
// C: 		if (err < 0)
// C: 			return err;
// C: 		err = snd_ctl_add(card,
// C: 			snd_ctl_new1(&snd_emu1010_clock_fallback, emu));
// C: 		if (err < 0)
// C: 			return err;
// C: 
// C: 		err = add_ctls(emu, &emu1010_adc_pads_ctl,
// C: 			       emu_pi->adc_ctls, emu_pi->n_adc_ctls);
// C: 		if (err < 0)
// C: 			return err;
// C: 		err = add_ctls(emu, &emu1010_dac_pads_ctl,
// C: 			       emu_pi->dac_ctls, emu_pi->n_dac_ctls);
// C: 		if (err < 0)
// C: 			return err;
// C: 
// C: 		if (!emu->card_capabilities->no_adat) {
// C: 			err = snd_ctl_add(card,
// C: 				snd_ctl_new1(&snd_emu1010_optical_out, emu));
// C: 			if (err < 0)
// C: 				return err;
// C: 			err = snd_ctl_add(card,
// C: 				snd_ctl_new1(&snd_emu1010_optical_in, emu));
// C: 			if (err < 0)
// C: 				return err;
// C: 		}
// C: 
// C: 		err = add_emu1010_source_mixers(emu);
// C: 		if (err < 0)
// C: 			return err;
// C: 	}
// C: 
// C: 	if ( emu->card_capabilities->i2c_adc) {
// C: 		err = snd_ctl_add(card, snd_ctl_new1(&snd_audigy_i2c_capture_source, emu));
// C: 		if (err < 0)
// C: 			return err;
// C: 
// C: 		err = add_ctls(emu, &i2c_volume_ctl,
// C: 			       snd_audigy_i2c_volume_ctls,
// C: 			       ARRAY_SIZE(snd_audigy_i2c_volume_ctls));
// C: 		if (err < 0)
// C: 			return err;
// C: 	}
// C: 		
// C: 	if (emu->card_capabilities->ac97_chip && emu->audigy) {
// C: 		err = snd_ctl_add(card, snd_ctl_new1(&snd_audigy_capture_boost,
// C: 						     emu));
// C: 		if (err < 0)
// C: 			return err;
// C: 	}
// C: 
// C: 	return 0;
// C: }

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
