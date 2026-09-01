// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 *  hda_intel.c - Implementation of primary alsa driver code base
 *                for Intel HD Audio.
 *
 *  Copyright(c) 2004 Intel Corporation
 *
 *  Copyright (c) 2004 Takashi Iwai <tiwai@suse.de>
 *                     PeiSen Hou <pshou@realtek.com.tw>
 *
 *  CONTACTS:
 *
 *  Matt Jared		matt.jared@intel.com
 *  Andy Kopp		andy.kopp@intel.com
 *  Dan Kogan		dan.d.kogan@intel.com
 *
 *  CHANGES:
 *
 *  2004.12.01	Major rewrite by tiwai, merged the work of pshou
 */

// Rust translation of hda/controllers/intel.c. External kernel, ALSA, PCI,
// ACPI, vga_switcheroo, firmware, tracing, and local intel.h/intel_trace.h
// items are intentionally referenced by their original names.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

const POS_FIX_AUTO: c_int = 0;
const POS_FIX_LPIB: c_int = 1;
const POS_FIX_POSBUF: c_int = 2;
const POS_FIX_VIACOMBO: c_int = 3;
const POS_FIX_COMBO: c_int = 4;
const POS_FIX_SKL: c_int = 5;
const POS_FIX_FIFO: c_int = 6;

const ATI_SB450_HDAUDIO_MISC_CNTR2_ADDR: c_uint = 0x42;
const ATI_SB450_HDAUDIO_ENABLE_SNOOP: u8 = 0x02;
const NVIDIA_HDA_TRANSREG_ADDR: c_uint = 0x4e;
const NVIDIA_HDA_ENABLE_COHBITS: u8 = 0x0f;
const NVIDIA_HDA_ISTRM_COH: c_uint = 0x4d;
const NVIDIA_HDA_OSTRM_COH: c_uint = 0x4c;
const NVIDIA_HDA_ENABLE_COHBIT: u8 = 0x01;
const INTEL_HDA_CGCTL: c_uint = 0x48;
const INTEL_HDA_CGCTL_MISCBDCGE: u32 = 0x1 << 6;
const INTEL_SCH_HDA_DEVC: c_uint = 0x78;
const INTEL_SCH_HDA_DEVC_NOSNOOP: u16 = 0x1 << 11;
const ICH6_NUM_CAPTURE: c_int = 4;
const ICH6_NUM_PLAYBACK: c_int = 4;
const ULI_NUM_CAPTURE: c_int = 5;
const ULI_NUM_PLAYBACK: c_int = 6;
const ATIHDMI_NUM_CAPTURE: c_int = 0;
const ATIHDMI_NUM_PLAYBACK: c_int = 8;
const PCI_DEVICE_ID_HYGON_18H_M05H_HDA: c_uint = 0x14a9;
const AMD_FIFO_SIZE: c_uint = 32;
const AZX_FORCE_CODEC_MASK: c_int = 0x100;

const AZX_DRIVER_ICH: c_int = 0;
const AZX_DRIVER_PCH: c_int = 1;
const AZX_DRIVER_SCH: c_int = 2;
const AZX_DRIVER_SKL: c_int = 3;
const AZX_DRIVER_HDMI: c_int = 4;
const AZX_DRIVER_ATI: c_int = 5;
const AZX_DRIVER_ATIHDMI: c_int = 6;
const AZX_DRIVER_ATIHDMI_NS: c_int = 7;
const AZX_DRIVER_GFHDMI: c_int = 8;
const AZX_DRIVER_VIA: c_int = 9;
const AZX_DRIVER_SIS: c_int = 10;
const AZX_DRIVER_ULI: c_int = 11;
const AZX_DRIVER_NVIDIA: c_int = 12;
const AZX_DRIVER_TERA: c_int = 13;
const AZX_DRIVER_CTX: c_int = 14;
const AZX_DRIVER_CTHDA: c_int = 15;
const AZX_DRIVER_CMEDIA: c_int = 16;
const AZX_DRIVER_ZHAOXIN: c_int = 17;
const AZX_DRIVER_ZHAOXINHDMI: c_int = 18;
const AZX_DRIVER_LOONGSON: c_int = 19;
const AZX_DRIVER_HYGON: c_int = 20;
const AZX_DRIVER_GENERIC: c_int = 21;
const AZX_NUM_DRIVERS: usize = 22;

const fn azx_get_snoop_type_caps(driver_caps: c_uint) -> c_uint {
    (driver_caps & AZX_DCAPS_SNOOP_MASK) >> 10
}
const fn AZX_DCAPS_SNOOP_TYPE(type_: c_uint) -> c_uint {
    type_ << 10
}

const AZX_DCAPS_INTEL_ICH: c_uint = AZX_DCAPS_OLD_SSYNC | AZX_DCAPS_NO_ALIGN_BUFSIZE;
const AZX_DCAPS_INTEL_PCH_BASE: c_uint =
    AZX_DCAPS_NO_ALIGN_BUFSIZE | AZX_DCAPS_COUNT_LPIB_DELAY | AZX_DCAPS_SNOOP_TYPE(AZX_SNOOP_TYPE_SCH);
const AZX_DCAPS_INTEL_PCH_NOPM: c_uint = AZX_DCAPS_INTEL_PCH_BASE | AZX_DCAPS_I915_COMPONENT;
const AZX_DCAPS_INTEL_PCH: c_uint = AZX_DCAPS_INTEL_PCH_BASE | AZX_DCAPS_PM_RUNTIME;
const AZX_DCAPS_INTEL_HASWELL: c_uint =
    AZX_DCAPS_COUNT_LPIB_DELAY | AZX_DCAPS_PM_RUNTIME | AZX_DCAPS_I915_COMPONENT |
    AZX_DCAPS_SNOOP_TYPE(AZX_SNOOP_TYPE_SCH);
const AZX_DCAPS_INTEL_BROADWELL: c_uint =
    AZX_DCAPS_POSFIX_LPIB | AZX_DCAPS_PM_RUNTIME | AZX_DCAPS_I915_COMPONENT |
    AZX_DCAPS_SNOOP_TYPE(AZX_SNOOP_TYPE_SCH);
const AZX_DCAPS_INTEL_BAYTRAIL: c_uint = AZX_DCAPS_INTEL_PCH_BASE | AZX_DCAPS_I915_COMPONENT;
const AZX_DCAPS_INTEL_BRASWELL: c_uint =
    AZX_DCAPS_INTEL_PCH_BASE | AZX_DCAPS_PM_RUNTIME | AZX_DCAPS_I915_COMPONENT;
const AZX_DCAPS_INTEL_SKYLAKE: c_uint =
    AZX_DCAPS_INTEL_PCH_BASE | AZX_DCAPS_PM_RUNTIME | AZX_DCAPS_SEPARATE_STREAM_TAG |
    AZX_DCAPS_I915_COMPONENT;
const AZX_DCAPS_INTEL_BROXTON: c_uint = AZX_DCAPS_INTEL_SKYLAKE;
const AZX_DCAPS_INTEL_LNL: c_uint = AZX_DCAPS_INTEL_SKYLAKE | AZX_DCAPS_PIO_COMMANDS;
const AZX_DCAPS_INTEL_NVL: c_uint = AZX_DCAPS_INTEL_LNL & !AZX_DCAPS_NO_ALIGN_BUFSIZE;
const AZX_DCAPS_PRESET_ATI_SB: c_uint =
    AZX_DCAPS_NO_TCSEL | AZX_DCAPS_POSFIX_LPIB | AZX_DCAPS_SNOOP_TYPE(AZX_SNOOP_TYPE_ATI);
const AZX_DCAPS_PRESET_ATI_HDMI: c_uint =
    AZX_DCAPS_NO_TCSEL | AZX_DCAPS_POSFIX_LPIB | AZX_DCAPS_NO_MSI64;
const AZX_DCAPS_PRESET_ATI_HDMI_NS: c_uint =
    AZX_DCAPS_PRESET_ATI_HDMI | AZX_DCAPS_SNOOP_OFF;
const AZX_DCAPS_PRESET_AMD_SB: c_uint =
    AZX_DCAPS_NO_TCSEL | AZX_DCAPS_AMD_WORKAROUND | AZX_DCAPS_SNOOP_TYPE(AZX_SNOOP_TYPE_ATI) |
    AZX_DCAPS_PM_RUNTIME | AZX_DCAPS_RETRY_PROBE;
const AZX_DCAPS_PRESET_NVIDIA: c_uint =
    AZX_DCAPS_NO_MSI | AZX_DCAPS_CORBRP_SELF_CLEAR | AZX_DCAPS_SNOOP_TYPE(AZX_SNOOP_TYPE_NVIDIA);
const AZX_DCAPS_PRESET_CTHDA: c_uint =
    AZX_DCAPS_NO_MSI | AZX_DCAPS_POSFIX_LPIB | AZX_DCAPS_NO_64BIT |
    AZX_DCAPS_4K_BDLE_BOUNDARY | AZX_DCAPS_SNOOP_OFF;

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR;
static mut enable: [bool; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;
static mut model: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
static mut position_fix: [c_int; SNDRV_CARDS] = [-1; SNDRV_CARDS];
static mut bdl_pos_adj: [c_int; SNDRV_CARDS] = [-1; SNDRV_CARDS];
static mut probe_mask: [c_int; SNDRV_CARDS] = [-1; SNDRV_CARDS];
static mut probe_only: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
static mut jackpoll_ms: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
static mut single_cmd: c_int = -1;
static mut enable_msi: c_int = -1;
// CONFIG_SND_HDA_PATCH_LOADER: static mut patch: [*mut c_char; SNDRV_CARDS].
// CONFIG_SND_HDA_INPUT_BEEP: static mut beep_mode initialized from CONFIG_SND_HDA_INPUT_BEEP_MODE.
static mut dmic_detect: bool = true;
static mut ctl_dev_id: bool = IS_ENABLED_CONFIG_SND_HDA_CTL_DEV_ID;
// Module parameter declarations from C are preserved as dependency comments:
// module_param_array(index,id,enable,model,position_fix,bdl_pos_adj,probe_mask,probe_only,jackpoll_ms)
// module_param(single_cmd), module_param(enable_msi), module_param(dmic_detect), module_param(ctl_dev_id)

// CONFIG_PM:
static mut power_save: c_int = CONFIG_SND_HDA_POWER_SAVE_DEFAULT;
static mut pm_blacklist: c_int = -1;
static mut power_save_controller: bool = true;
// !CONFIG_PM maps these to 0, 0, false in the original C preprocessor branch.
static mut align_buffer_size: c_int = -1;
// CONFIG_X86: static mut hda_snoop: c_int = -1; otherwise hda_snoop is true.
static mut hda_snoop: c_int = -1;

static driver_short_names: [&[u8]; AZX_NUM_DRIVERS] = [
    b"HDA Intel\0",
    b"HDA Intel PCH\0",
    b"HDA Intel MID\0",
    b"HDA Intel PCH\0",
    b"HDA Intel HDMI\0",
    b"HDA ATI SB\0",
    b"HDA ATI HDMI\0",
    b"HDA ATI HDMI\0",
    b"HDA GF HDMI\0",
    b"HDA VIA VT82xx\0",
    b"HDA SIS966\0",
    b"HDA ULI M5461\0",
    b"HDA NVidia\0",
    b"HDA Teradici\0",
    b"HDA Creative\0",
    b"HDA Creative\0",
    b"HDA C-Media\0",
    b"HDA Zhaoxin\0",
    b"HDA Zhaoxin HDMI\0",
    b"HDA Loongson\0",
    b"HDA Hygon\0",
    b"HD-Audio Generic\0",
];

unsafe fn update_pci_byte(pci: *mut pci_dev, reg: c_uint, mask: u8, val: u8) {
    let mut data: u8 = 0;
    pci_read_config_byte(pci, reg, &mut data);
    data &= !mask;
    data |= val & mask;
    pci_write_config_byte(pci, reg, data);
}

unsafe fn azx_init_pci(chip: *mut azx) {
    let snoop_type = azx_get_snoop_type_caps((*chip).driver_caps);
    if ((*chip).driver_caps & AZX_DCAPS_NO_TCSEL) == 0 {
        dev_dbg((*(*chip).card).dev, c"Clearing TCSEL\n".as_ptr());
        update_pci_byte((*chip).pci, AZX_PCIREG_TCSEL, 0x07, 0);
    }
    if snoop_type == AZX_SNOOP_TYPE_ATI {
        dev_dbg((*(*chip).card).dev, c"Setting ATI snoop: %d\n".as_ptr(), azx_snoop(chip));
        update_pci_byte(
            (*chip).pci,
            ATI_SB450_HDAUDIO_MISC_CNTR2_ADDR,
            0x07,
            if azx_snoop(chip) != 0 { ATI_SB450_HDAUDIO_ENABLE_SNOOP } else { 0 },
        );
    }
    if snoop_type == AZX_SNOOP_TYPE_NVIDIA {
        dev_dbg((*(*chip).card).dev, c"Setting Nvidia snoop: %d\n".as_ptr(), azx_snoop(chip));
        update_pci_byte((*chip).pci, NVIDIA_HDA_TRANSREG_ADDR, 0x0f, NVIDIA_HDA_ENABLE_COHBITS);
        update_pci_byte((*chip).pci, NVIDIA_HDA_ISTRM_COH, 0x01, NVIDIA_HDA_ENABLE_COHBIT);
        update_pci_byte((*chip).pci, NVIDIA_HDA_OSTRM_COH, 0x01, NVIDIA_HDA_ENABLE_COHBIT);
    }
    if snoop_type == AZX_SNOOP_TYPE_SCH {
        let mut snoop: u16 = 0;
        pci_read_config_word((*chip).pci, INTEL_SCH_HDA_DEVC, &mut snoop);
        if (azx_snoop(chip) == 0 && (snoop & INTEL_SCH_HDA_DEVC_NOSNOOP) == 0) ||
            (azx_snoop(chip) != 0 && (snoop & INTEL_SCH_HDA_DEVC_NOSNOOP) != 0)
        {
            snoop &= !INTEL_SCH_HDA_DEVC_NOSNOOP;
            if azx_snoop(chip) == 0 {
                snoop |= INTEL_SCH_HDA_DEVC_NOSNOOP;
            }
            pci_write_config_word((*chip).pci, INTEL_SCH_HDA_DEVC, snoop);
            pci_read_config_word((*chip).pci, INTEL_SCH_HDA_DEVC, &mut snoop);
        }
        dev_dbg(
            (*(*chip).card).dev,
            c"SCH snoop: %s\n".as_ptr(),
            if (snoop & INTEL_SCH_HDA_DEVC_NOSNOOP) != 0 { c"Disabled".as_ptr() } else { c"Enabled".as_ptr() },
        );
    }
}

unsafe fn bxt_reduce_dma_latency(chip: *mut azx) {
    let mut val: u32 = azx_readl(chip, VS_EM4L);
    val &= 0x3 << 20;
    azx_writel(chip, VS_EM4L, val);
}

unsafe fn intel_get_lctl_scf(chip: *mut azx) -> c_int {
    let bus: *mut hdac_bus = azx_bus(chip);
    let preferred_bits: [c_int; 5] = [2, 3, 1, 4, 5];
    let val: u32 = readl((*bus).mlcap.add(AZX_ML_BASE + AZX_REG_ML_LCAP));
    for t in preferred_bits {
        if (val & (1 << t)) != 0 {
            return t;
        }
    }
    dev_warn((*(*chip).card).dev, c"set audio clock frequency to 6MHz".as_ptr());
    0
}

unsafe fn intel_ml_lctl_set_power(chip: *mut azx, state: c_int) -> c_int {
    let bus: *mut hdac_bus = azx_bus(chip);
    let mut val: u32 = readl((*bus).mlcap.add(AZX_ML_BASE + AZX_REG_ML_LCTL));
    val &= !AZX_ML_LCTL_SPA;
    val |= (state as u32) << AZX_ML_LCTL_SPA_SHIFT;
    writel(val, (*bus).mlcap.add(AZX_ML_BASE + AZX_REG_ML_LCTL));
    let mut timeout = 50;
    while timeout != 0 {
        if (readl((*bus).mlcap.add(AZX_ML_BASE + AZX_REG_ML_LCTL)) & AZX_ML_LCTL_CPA)
            == ((state as u32) << AZX_ML_LCTL_CPA_SHIFT)
        {
            return 0;
        }
        timeout -= 1;
        udelay(10);
    }
    -1
}

unsafe fn intel_init_lctl(chip: *mut azx) {
    let bus: *mut hdac_bus = azx_bus(chip);
    let mut val: u32 = readl((*bus).mlcap.add(AZX_ML_BASE + AZX_REG_ML_LCTL));
    if (val & AZX_ML_LCTL_SCF) != 0 {
        return;
    }
    if ((val & AZX_ML_LCTL_SPA) >> AZX_ML_LCTL_SPA_SHIFT)
        != ((val & AZX_ML_LCTL_CPA) >> AZX_ML_LCTL_CPA_SHIFT)
    {
        return;
    }
    let ret = intel_ml_lctl_set_power(chip, 0);
    udelay(100);
    if ret == 0 {
        val &= !AZX_ML_LCTL_SCF;
        val |= intel_get_lctl_scf(chip) as u32;
        writel(val, (*bus).mlcap.add(AZX_ML_BASE + AZX_REG_ML_LCTL));
    }
    intel_ml_lctl_set_power(chip, 1);
    udelay(100);
}

unsafe fn hda_intel_init_chip(chip: *mut azx, full_reset: bool) {
    let bus = azx_bus(chip);
    let pci = (*chip).pci;
    let mut val: u32 = 0;
    snd_hdac_set_codec_wakeup(bus, true);
    if (*chip).driver_type == AZX_DRIVER_SKL {
        pci_read_config_dword(pci, INTEL_HDA_CGCTL, &mut val);
        val &= !INTEL_HDA_CGCTL_MISCBDCGE;
        pci_write_config_dword(pci, INTEL_HDA_CGCTL, val);
    }
    azx_init_chip(chip, full_reset);
    if (*chip).driver_type == AZX_DRIVER_SKL {
        pci_read_config_dword(pci, INTEL_HDA_CGCTL, &mut val);
        val |= INTEL_HDA_CGCTL_MISCBDCGE;
        pci_write_config_dword(pci, INTEL_HDA_CGCTL, val);
    }
    snd_hdac_set_codec_wakeup(bus, false);
    if HDA_CONTROLLER_IS_APL(pci) {
        bxt_reduce_dma_latency(chip);
    }
    if !(*bus).mlcap.is_null() {
        intel_init_lctl(chip);
    }
}

unsafe fn azx_get_delay_from_lpib(chip: *mut azx, azx_dev: *mut azx_dev, pos: c_uint) -> c_int {
    let substream = (*azx_dev).core.substream;
    let stream = (*substream).stream;
    let lpib_pos = azx_get_pos_lpib(chip, azx_dev);
    let mut delay: c_int = if stream == SNDRV_PCM_STREAM_PLAYBACK {
        pos.wrapping_sub(lpib_pos) as c_int
    } else {
        lpib_pos.wrapping_sub(pos) as c_int
    };
    if delay < 0 {
        if delay >= (*azx_dev).core.delay_negative_threshold {
            delay = 0;
        } else {
            delay += (*azx_dev).core.bufsize as c_int;
        }
    }
    if delay as c_uint >= (*azx_dev).core.period_bytes {
        dev_info((*(*chip).card).dev, c"Unstable LPIB (%d >= %d); disabling LPIB delay counting\n".as_ptr(), delay, (*azx_dev).core.period_bytes);
        delay = 0;
        (*chip).driver_caps &= !AZX_DCAPS_COUNT_LPIB_DELAY;
        (*chip).get_delay[stream as usize] = None;
    }
    bytes_to_frames((*substream).runtime, delay as c_uint) as c_int
}

unsafe fn azx_position_check(chip: *mut azx, azx_dev: *mut azx_dev) -> c_int {
    let istream = azx_dev_to_istream(azx_dev);
    let ok = azx_position_ok(chip, azx_dev);
    if ok == 1 {
        (*istream).irq_pending = false;
        return ok;
    } else if ok == 0 {
        (*istream).irq_pending = true;
        schedule_work(&mut (*istream).irq_pending_work);
    }
    0
}

unsafe fn display_power(chip: *mut azx, enable: bool) {
    snd_hdac_display_power(azx_bus(chip), HDA_CODEC_IDX_CONTROLLER, enable);
}

unsafe fn azx_position_ok(chip: *mut azx, azx_dev: *mut azx_dev) -> c_int {
    let substream = (*azx_dev).core.substream;
    let runtime = (*substream).runtime;
    let stream = (*substream).stream as usize;
    if (*chip).driver_type == AZX_DRIVER_LOONGSON {
        return 1;
    }
    let wallclk = azx_readl(chip, WALLCLK).wrapping_sub((*azx_dev).core.start_wallclk);
    if wallclk < ((*azx_dev).core.period_wallclk * 2) / 3 {
        return -1;
    }
    let mut pos: c_uint;
    if let Some(get_position) = (*chip).get_position[stream] {
        pos = get_position(chip, azx_dev);
    } else {
        pos = azx_get_pos_posbuf(chip, azx_dev);
        if pos == 0 || pos == !0u32 {
            dev_info((*(*chip).card).dev, c"Invalid position buffer, using LPIB read method instead.\n".as_ptr());
            (*chip).get_position[stream] = Some(azx_get_pos_lpib);
            if (*chip).get_position[0] == Some(azx_get_pos_lpib) &&
                (*chip).get_position[1] == Some(azx_get_pos_lpib)
            {
                (*azx_bus(chip)).use_posbuf = false;
            }
            pos = azx_get_pos_lpib(chip, azx_dev);
            (*chip).get_delay[stream] = None;
        } else {
            (*chip).get_position[stream] = Some(azx_get_pos_posbuf);
            if ((*chip).driver_caps & AZX_DCAPS_COUNT_LPIB_DELAY) != 0 {
                (*chip).get_delay[stream] = Some(azx_get_delay_from_lpib);
            }
        }
    }
    if pos >= (*azx_dev).core.bufsize {
        pos = 0;
    }
    if WARN_ONCE((*azx_dev).core.period_bytes == 0, c"hda-intel: zero azx_dev->period_bytes".as_ptr()) {
        return -1;
    }
    if wallclk < ((*azx_dev).core.period_wallclk * 5) / 4 &&
        pos % (*azx_dev).core.period_bytes > (*azx_dev).core.period_bytes / 2
    {
        return if (*chip).bdl_pos_adj != 0 { 0 } else { -1 };
    }
    (*azx_dev).core.start_wallclk = (*azx_dev).core.start_wallclk.wrapping_add(wallclk);
    if (*azx_dev).core.no_period_wakeup {
        return 1;
    }
    if (*runtime).hw_ptr_base != (*runtime).hw_ptr_interrupt {
        return 1;
    }
    let frames = bytes_to_frames(runtime, pos);
    let mut hwptr = (*runtime).hw_ptr_base + frames;
    if hwptr < (*(*runtime).status).hw_ptr {
        hwptr += (*runtime).buffer_size;
    }
    let target = (*runtime).hw_ptr_interrupt + (*runtime).period_size;
    if hwptr < target {
        return if (*chip).bdl_pos_adj != 0 { 0 } else { -1 };
    }
    1
}

unsafe fn azx_irq_pending_work(work: *mut work_struct) {
    let istream = container_of_hda_intel_stream_irq_pending_work(work);
    let azx_dev = &mut (*istream).azx_dev as *mut azx_dev;
    let hda = (*istream).hda;
    let chip = &mut (*hda).chip as *mut azx;
    let bus = azx_bus(chip);
    let mut ok: c_int;
    if !(*hda).irq_pending_warned {
        dev_info((*(*chip).card).dev, c"IRQ timing workaround is activated for card #%d. Suggest a bigger bdl_pos_adj.\n".as_ptr(), (*(*chip).card).number);
        (*hda).irq_pending_warned = 1;
    }
    loop {
        spin_lock_irq(&mut (*bus).reg_lock);
        if !(*istream).irq_pending || (*azx_dev).core.substream.is_null() || !(*azx_dev).core.running {
            spin_unlock_irq(&mut (*bus).reg_lock);
            return;
        }
        ok = azx_position_ok(chip, azx_dev);
        if ok < 0 {
            spin_unlock_irq(&mut (*bus).reg_lock);
            return;
        }
        if ok > 0 {
            (*istream).irq_pending = false;
        }
        spin_unlock_irq(&mut (*bus).reg_lock);
        if ok != 0 {
            snd_pcm_period_elapsed((*azx_dev).core.substream);
            return;
        }
        msleep(1);
    }
}

unsafe fn hda_intel_stream_clear_irq_pending(azx_dev: *mut azx_dev) {
    let istream = azx_dev_to_istream(azx_dev);
    (*istream).irq_pending = false;
    cancel_work_sync(&mut (*istream).irq_pending_work);
}

unsafe fn hda_intel_pcm_close(_chip: *mut azx, azx_dev: *mut azx_dev) {
    hda_intel_stream_clear_irq_pending(azx_dev);
}

unsafe fn azx_clear_irq_pending(chip: *mut azx) {
    let bus = azx_bus(chip);
    // list_for_each_entry(s, &bus->stream_list, list)
    list_for_each_hdac_stream(&mut (*bus).stream_list, |s| {
        hda_intel_stream_clear_irq_pending(stream_to_azx_dev(s));
    });
}

unsafe fn azx_acquire_irq(chip: *mut azx, do_disconnect: c_int) -> c_int {
    let bus = azx_bus(chip);
    let mut ret: c_int;
    if (*chip).msi == 0 || pci_alloc_irq_vectors((*chip).pci, 1, 1, PCI_IRQ_MSI) < 0 {
        ret = pci_alloc_irq_vectors((*chip).pci, 1, 1, PCI_IRQ_INTX);
        if ret < 0 {
            return ret;
        }
        (*chip).msi = 0;
    }
    if request_irq((*(*chip).pci).irq, Some(azx_interrupt), if (*chip).msi != 0 { 0 } else { IRQF_SHARED }, (*(*chip).card).irq_descr, chip as *mut c_void) != 0 {
        dev_err((*(*chip).card).dev, c"unable to grab IRQ %d, disabling device\n".as_ptr(), (*(*chip).pci).irq);
        if do_disconnect != 0 {
            snd_card_disconnect((*chip).card);
        }
        return -1;
    }
    (*bus).irq = (*(*chip).pci).irq;
    (*(*chip).card).sync_irq = (*bus).irq;
    0
}

unsafe fn azx_via_get_position(_chip: *mut azx, azx_dev: *mut azx_dev) -> c_uint {
    let link_pos = snd_hdac_stream_get_pos_lpib(azx_stream(azx_dev));
    if (*(*azx_dev).core.substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        return link_pos;
    }
    let mut mod_dma_pos = le32_to_cpu(*(*azx_dev).core.posbuf);
    mod_dma_pos %= (*azx_dev).core.period_bytes;
    let fifo_size = (*azx_stream(azx_dev)).fifo_size;
    if (*azx_dev).insufficient != 0 {
        if link_pos <= fifo_size {
            return 0;
        }
        (*azx_dev).insufficient = 0;
    }
    let mini_pos = if link_pos <= fifo_size {
        (*azx_dev).core.bufsize + link_pos - fifo_size
    } else {
        link_pos - fifo_size
    };
    let mod_mini_pos = mini_pos % (*azx_dev).core.period_bytes;
    let mod_link_pos = link_pos % (*azx_dev).core.period_bytes;
    let mut bound_pos = if mod_link_pos >= fifo_size {
        link_pos - mod_link_pos
    } else if mod_dma_pos >= mod_mini_pos {
        mini_pos - mod_mini_pos
    } else {
        let mut p = mini_pos - mod_mini_pos + (*azx_dev).core.period_bytes;
        if p >= (*azx_dev).core.bufsize {
            p = 0;
        }
        p
    };
    bound_pos + mod_dma_pos
}

unsafe fn azx_get_pos_fifo(_chip: *mut azx, azx_dev: *mut azx_dev) -> c_uint {
    let substream = (*azx_dev).core.substream;
    let runtime = (*substream).runtime;
    let mut pos = snd_hdac_stream_get_pos_lpib(azx_stream(azx_dev));
    if runtime.is_null() {
        return pos;
    }
    (*runtime).delay = AMD_FIFO_SIZE as _;
    let mut delay = frames_to_bytes(runtime, AMD_FIFO_SIZE);
    if (*azx_dev).insufficient != 0 {
        if pos < delay {
            delay = pos;
            (*runtime).delay = bytes_to_frames(runtime, pos) as _;
        } else {
            (*azx_dev).insufficient = 0;
        }
    }
    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        if pos < delay {
            pos += (*azx_dev).core.bufsize;
        }
        pos -= delay;
    }
    pos
}

unsafe fn azx_get_delay_from_fifo(_chip: *mut azx, azx_dev: *mut azx_dev, _pos: c_uint) -> c_int {
    (*(*(*azx_dev).core.substream).runtime).delay
}

unsafe fn __azx_shutdown_chip(chip: *mut azx, skip_link_reset: bool) {
    azx_stop_chip(chip);
    if !skip_link_reset {
        azx_enter_link_reset(chip);
    }
    azx_clear_irq_pending(chip);
    display_power(chip, false);
}

// DEFINE_MUTEX(card_list_lock); LIST_HEAD(card_list);
static mut card_list_lock: mutex = unsafe { core::mem::zeroed() };
static mut card_list: list_head = unsafe { core::mem::zeroed() };

unsafe fn azx_shutdown_chip(chip: *mut azx) {
    __azx_shutdown_chip(chip, false);
}

unsafe fn azx_add_card_list(chip: *mut azx) {
    let hda = container_of_hda_intel_chip(chip);
    mutex_lock(&mut card_list_lock);
    list_add(&mut (*hda).list, &mut card_list);
    mutex_unlock(&mut card_list_lock);
}

unsafe fn azx_del_card_list(chip: *mut azx) {
    let hda = container_of_hda_intel_chip(chip);
    mutex_lock(&mut card_list_lock);
    list_del_init(&mut (*hda).list);
    mutex_unlock(&mut card_list_lock);
}

unsafe fn param_set_xint(val: *const c_char, kp: *const kernel_param) -> c_int {
    let prev = power_save;
    let ret = param_set_int(val, kp);
    if ret != 0 || prev == power_save {
        return ret;
    }
    if pm_blacklist > 0 {
        return 0;
    }
    mutex_lock(&mut card_list_lock);
    list_for_each_hda_intel(&mut card_list, |hda| {
        let chip = &mut (*hda).chip as *mut azx;
        if (*hda).probe_continued == 0 || (*chip).disabled || (*hda).runtime_pm_disabled {
            return;
        }
        snd_hda_set_power_save(&mut (*chip).bus, power_save * 1000);
    });
    mutex_unlock(&mut card_list_lock);
    0
}

unsafe fn azx_is_pm_ready(card: *mut snd_card) -> bool {
    if card.is_null() {
        return false;
    }
    let chip = (*card).private_data as *mut azx;
    let hda = container_of_hda_intel_chip(chip);
    !(*chip).disabled && !(*hda).init_failed && (*chip).running != 0
}

unsafe fn __azx_runtime_resume(chip: *mut azx) {
    let hda = container_of_hda_intel_chip(chip);
    let bus = azx_bus(chip);
    display_power(chip, true);
    if (*hda).need_i915_power {
        snd_hdac_i915_set_bclk(bus);
    }
    let status = azx_readw(chip, STATESTS);
    azx_init_pci(chip);
    hda_intel_init_chip(chip, true);
    if (*chip).pm_prepared == 0 {
        list_for_each_codec(&mut (*chip).bus, |codec| {
            if (*codec).relaxed_resume {
                return;
            }
            if (*codec).forced_resume || (status & (1 << (*codec).addr)) != 0 {
                pm_request_resume(hda_codec_dev(codec));
            }
        });
    }
    if !(*hda).need_i915_power {
        display_power(chip, false);
    }
}

unsafe fn azx_prepare(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    if !azx_is_pm_ready(card) {
        return 0;
    }
    let chip = (*card).private_data as *mut azx;
    (*chip).pm_prepared = 1;
    snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    flush_work(&mut (*azx_bus(chip)).unsol_work);
    0
}

unsafe fn azx_complete(dev: *mut device) {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    if !azx_is_pm_ready(card) {
        return;
    }
    let chip = (*card).private_data as *mut azx;
    snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    (*chip).pm_prepared = 0;
}

unsafe fn azx_suspend(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    if !azx_is_pm_ready(card) {
        return 0;
    }
    let chip = (*card).private_data as *mut azx;
    azx_shutdown_chip(chip);
    trace_azx_suspend(chip);
    0
}

unsafe fn azx_resume(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    if !azx_is_pm_ready(card) {
        return 0;
    }
    let chip = (*card).private_data as *mut azx;
    __azx_runtime_resume(chip);
    trace_azx_resume(chip);
    0
}

unsafe fn azx_freeze_noirq(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let chip = (*card).private_data as *mut azx;
    let pci = to_pci_dev(dev);
    if !azx_is_pm_ready(card) {
        return 0;
    }
    if (*chip).driver_type == AZX_DRIVER_SKL {
        pci_set_power_state(pci, PCI_D3hot);
    }
    0
}

unsafe fn azx_thaw_noirq(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    let chip = (*card).private_data as *mut azx;
    let pci = to_pci_dev(dev);
    if !azx_is_pm_ready(card) {
        return 0;
    }
    if (*chip).driver_type == AZX_DRIVER_SKL {
        pci_set_power_state(pci, PCI_D0);
    }
    0
}

unsafe fn azx_runtime_suspend(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    if !azx_is_pm_ready(card) {
        return 0;
    }
    let chip = (*card).private_data as *mut azx;
    azx_writew(chip, WAKEEN, azx_readw(chip, WAKEEN) | STATESTS_INT_MASK);
    azx_shutdown_chip(chip);
    trace_azx_runtime_suspend(chip);
    0
}

unsafe fn azx_runtime_resume(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    if !azx_is_pm_ready(card) {
        return 0;
    }
    let chip = (*card).private_data as *mut azx;
    __azx_runtime_resume(chip);
    azx_writew(chip, WAKEEN, azx_readw(chip, WAKEEN) & !STATESTS_INT_MASK);
    trace_azx_runtime_resume(chip);
    0
}

unsafe fn azx_runtime_idle(dev: *mut device) -> c_int {
    let card = dev_get_drvdata(dev) as *mut snd_card;
    if card.is_null() {
        return 0;
    }
    let chip = (*card).private_data as *mut azx;
    let hda = container_of_hda_intel_chip(chip);
    if (*chip).disabled || (*hda).init_failed {
        return 0;
    }
    if !power_save_controller || !azx_has_pm_runtime(chip) ||
        (*azx_bus(chip)).codec_powered || (*chip).running == 0
    {
        return -EBUSY;
    }
    if needs_eld_notify_link(chip) {
        return -EBUSY;
    }
    0
}

static azx_pm: dev_pm_ops = dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(azx_suspend, azx_resume)
    prepare: Some(azx_prepare),
    complete: Some(azx_complete),
    freeze_noirq: Some(azx_freeze_noirq),
    thaw_noirq: Some(azx_thaw_noirq),
    // RUNTIME_PM_OPS(azx_runtime_suspend, azx_runtime_resume, azx_runtime_idle)
    runtime_suspend: Some(azx_runtime_suspend),
    runtime_resume: Some(azx_runtime_resume),
    runtime_idle: Some(azx_runtime_idle),
    ..DEV_PM_OPS_ZERO
};

// SUPPORT_VGA_SWITCHEROO section translated with original conditional intent.
unsafe fn azx_vs_set_state(pci: *mut pci_dev, state: vga_switcheroo_state) {
    let card = pci_get_drvdata(pci) as *mut snd_card;
    let chip = (*card).private_data as *mut azx;
    let hda = container_of_hda_intel_chip(chip);
    wait_for_completion(&mut (*hda).probe_wait);
    if (*hda).init_failed {
        return;
    }
    let disabled = state == VGA_SWITCHEROO_OFF;
    if (*chip).disabled == disabled {
        return;
    }
    if (*hda).probe_continued == 0 {
        (*chip).disabled = disabled;
        if !disabled {
            dev_info((*(*chip).card).dev, c"Start delayed initialization\n".as_ptr());
            if azx_probe_continue(chip) < 0 {
                dev_err((*(*chip).card).dev, c"initialization error\n".as_ptr());
            }
        }
    } else {
        dev_info((*(*chip).card).dev, c"%s via vga_switcheroo\n".as_ptr(), if disabled { c"Disabling".as_ptr() } else { c"Enabling".as_ptr() });
        if disabled {
            list_for_each_codec(&mut (*chip).bus, |codec| {
                pm_runtime_suspend(hda_codec_dev(codec));
                pm_runtime_disable(hda_codec_dev(codec));
            });
            pm_runtime_suspend((*card).dev);
            pm_runtime_disable((*card).dev);
            (*pci).current_state = PCI_D3cold;
            (*chip).disabled = true;
            if snd_hda_lock_devices(&mut (*chip).bus) != 0 {
                dev_warn((*(*chip).card).dev, c"Cannot lock devices!\n".as_ptr());
            }
        } else {
            snd_hda_unlock_devices(&mut (*chip).bus);
            (*chip).disabled = false;
            pm_runtime_enable((*card).dev);
            list_for_each_codec(&mut (*chip).bus, |codec| {
                pm_runtime_enable(hda_codec_dev(codec));
                pm_runtime_resume(hda_codec_dev(codec));
            });
        }
    }
}

unsafe fn azx_vs_can_switch(pci: *mut pci_dev) -> bool {
    let card = pci_get_drvdata(pci) as *mut snd_card;
    let chip = (*card).private_data as *mut azx;
    let hda = container_of_hda_intel_chip(chip);
    wait_for_completion(&mut (*hda).probe_wait);
    if (*hda).init_failed {
        return false;
    }
    if (*chip).disabled || (*hda).probe_continued == 0 {
        return true;
    }
    if snd_hda_lock_devices(&mut (*chip).bus) != 0 {
        return false;
    }
    snd_hda_unlock_devices(&mut (*chip).bus);
    true
}

unsafe fn setup_vga_switcheroo_runtime_pm(chip: *mut azx) {
    let hda = container_of_hda_intel_chip(chip);
    if (*hda).use_vga_switcheroo && !needs_eld_notify_link(chip) {
        list_for_each_codec(&mut (*chip).bus, |codec| {
            (*codec).auto_runtime_pm = 1;
        });
        if (*chip).running != 0 {
            set_default_power_save(chip);
        }
    }
}

unsafe fn azx_vs_gpu_bound(pci: *mut pci_dev, client_id: vga_switcheroo_client_id) {
    let card = pci_get_drvdata(pci) as *mut snd_card;
    let chip = (*card).private_data as *mut azx;
    if client_id == VGA_SWITCHEROO_DIS {
        (*chip).bus.keep_power = 0;
    }
    setup_vga_switcheroo_runtime_pm(chip);
}

unsafe fn init_vga_switcheroo(chip: *mut azx) {
    let hda = container_of_hda_intel_chip(chip);
    let p = get_bound_vga((*chip).pci);
    if !p.is_null() {
        dev_info((*(*chip).card).dev, c"Handle vga_switcheroo audio client\n".as_ptr());
        (*hda).use_vga_switcheroo = true;
        let parent = pci_upstream_bridge(p);
        (*chip).bus.keep_power = if !parent.is_null() { (!pci_pr3_present(parent)) as c_int } else { 1 };
        (*chip).driver_caps |= AZX_DCAPS_PM_RUNTIME;
        pci_dev_put(p);
    }
}

static azx_vs_ops: vga_switcheroo_client_ops = vga_switcheroo_client_ops {
    set_gpu_state: Some(azx_vs_set_state),
    can_switch: Some(azx_vs_can_switch),
    gpu_bound: Some(azx_vs_gpu_bound),
    ..VGA_SWITCHEROO_CLIENT_OPS_ZERO
};

unsafe fn register_vga_switcheroo(chip: *mut azx) -> c_int {
    let hda = container_of_hda_intel_chip(chip);
    if !(*hda).use_vga_switcheroo {
        return 0;
    }
    let p = get_bound_vga((*chip).pci);
    let err = vga_switcheroo_register_audio_client((*chip).pci, &azx_vs_ops, p);
    pci_dev_put(p);
    if err < 0 {
        return err;
    }
    (*hda).vga_switcheroo_registered = 1;
    0
}

unsafe fn azx_free(chip: *mut azx) {
    let pci = (*chip).pci;
    let hda = container_of_hda_intel_chip(chip);
    let bus = azx_bus(chip);
    if (*hda).freed {
        return;
    }
    if azx_has_pm_runtime(chip) && (*chip).running != 0 {
        pm_runtime_get_noresume(&mut (*pci).dev);
        pm_runtime_forbid(&mut (*pci).dev);
        pm_runtime_dont_use_autosuspend(&mut (*pci).dev);
    }
    (*chip).running = 0;
    azx_del_card_list(chip);
    (*hda).init_failed = 1;
    complete_all(&mut (*hda).probe_wait);
    if use_vga_switcheroo(hda) {
        if (*chip).disabled && (*hda).probe_continued != 0 {
            snd_hda_unlock_devices(&mut (*chip).bus);
        }
        if (*hda).vga_switcheroo_registered != 0 {
            vga_switcheroo_unregister_client((*chip).pci);
            dev_warn(&mut (*pci).dev, c"GPU sound probed, but not operational: please add a quirk to driver_denylist\n".as_ptr());
            pm_runtime_disable(&mut (*pci).dev);
            pm_runtime_set_suspended(&mut (*pci).dev);
            pm_runtime_enable(&mut (*pci).dev);
        }
    }
    if (*bus).chip_init {
        azx_clear_irq_pending(chip);
        azx_stop_all_streams(chip);
        azx_stop_chip(chip);
    }
    if (*bus).irq >= 0 {
        free_irq((*bus).irq, chip as *mut c_void);
    }
    azx_free_stream_pages(chip);
    azx_free_streams(chip);
    snd_hdac_bus_exit(bus);
    display_power(chip, false);
    if ((*chip).driver_caps & AZX_DCAPS_I915_COMPONENT) != 0 {
        snd_hdac_i915_exit(bus);
    }
    (*hda).freed = 1;
}

unsafe fn azx_dev_disconnect(device: *mut snd_device) -> c_int {
    let chip = (*device).device_data as *mut azx;
    let bus = azx_bus(chip);
    (*chip).bus.shutdown = 1;
    cancel_work_sync(&mut (*bus).unsol_work);
    0
}

unsafe fn azx_dev_free(device: *mut snd_device) -> c_int {
    azx_free((*device).device_data as *mut azx);
    0
}

unsafe fn atpx_present() -> bool {
    let mut pdev: *mut pci_dev = ptr::null_mut();
    loop {
        pdev = pci_get_base_class(PCI_BASE_CLASS_DISPLAY, pdev);
        if pdev.is_null() {
            break;
        }
        if (*pdev).class != (PCI_CLASS_DISPLAY_VGA << 8) && (*pdev).class != (PCI_CLASS_DISPLAY_OTHER << 8) {
            continue;
        }
        let dhandle = ACPI_HANDLE(&mut (*pdev).dev);
        if !dhandle.is_null() {
            let mut atpx_handle: acpi_handle = ptr::null_mut();
            let status = acpi_get_handle(dhandle, c"ATPX".as_ptr(), &mut atpx_handle);
            if ACPI_SUCCESS(status) {
                pci_dev_put(pdev);
                return true;
            }
        }
    }
    false
}

unsafe fn get_bound_vga(pci: *mut pci_dev) -> *mut pci_dev {
    match (*pci).vendor {
        PCI_VENDOR_ID_ATI | PCI_VENDOR_ID_AMD => {
            if (*pci).devfn == 1 {
                let p = pci_get_domain_bus_and_slot(pci_domain_nr((*pci).bus), (*(*pci).bus).number, 0);
                if !p.is_null() {
                    if pci_is_display(p) && (atpx_present() || apple_gmux_detect(ptr::null_mut(), ptr::null_mut()) != 0) {
                        return p;
                    }
                    pci_dev_put(p);
                }
            }
        }
        PCI_VENDOR_ID_NVIDIA => {
            if (*pci).devfn == 1 {
                let p = pci_get_domain_bus_and_slot(pci_domain_nr((*pci).bus), (*(*pci).bus).number, 0);
                if !p.is_null() {
                    if pci_is_display(p) {
                        return p;
                    }
                    pci_dev_put(p);
                }
            }
        }
        _ => {}
    }
    ptr::null_mut()
}

unsafe fn check_hdmi_disabled(pci: *mut pci_dev) -> bool {
    let mut vga_inactive = false;
    let p = get_bound_vga(pci);
    if !p.is_null() {
        if vga_switcheroo_get_client_state(p) == VGA_SWITCHEROO_OFF {
            vga_inactive = true;
        }
        pci_dev_put(p);
    }
    vga_inactive
}

// Quirk tables retain the original table data and macro constructors.
static position_fix_list: &[snd_pci_quirk] = &[
    SND_PCI_QUIRK(0x1028, 0x01cc, c"Dell D820".as_ptr(), POS_FIX_LPIB),
    SND_PCI_QUIRK(0x1028, 0x01de, c"Dell Precision 390".as_ptr(), POS_FIX_LPIB),
    SND_PCI_QUIRK(0x103c, 0x306d, c"HP dv3".as_ptr(), POS_FIX_LPIB),
    SND_PCI_QUIRK(0x1043, 0x813d, c"ASUS P5AD2".as_ptr(), POS_FIX_LPIB),
    SND_PCI_QUIRK(0x1043, 0x81b3, c"ASUS".as_ptr(), POS_FIX_LPIB),
    SND_PCI_QUIRK(0x1043, 0x81e7, c"ASUS M2V".as_ptr(), POS_FIX_LPIB),
    SND_PCI_QUIRK(0x104d, 0x9069, c"Sony VPCS11V9E".as_ptr(), POS_FIX_LPIB),
    SND_PCI_QUIRK(0x10de, 0xcb89, c"Macbook Pro 7,1".as_ptr(), POS_FIX_LPIB),
    SND_PCI_QUIRK(0x1297, 0x3166, c"Shuttle".as_ptr(), POS_FIX_LPIB),
    SND_PCI_QUIRK(0x1458, 0xa022, c"ga-ma770-ud3".as_ptr(), POS_FIX_LPIB),
    SND_PCI_QUIRK(0x1462, 0x1002, c"MSI Wind U115".as_ptr(), POS_FIX_LPIB),
    SND_PCI_QUIRK(0x1565, 0x8218, c"Biostar Microtech".as_ptr(), POS_FIX_LPIB),
    SND_PCI_QUIRK(0x1849, 0x0888, c"775Dual-VSTA".as_ptr(), POS_FIX_LPIB),
    SND_PCI_QUIRK(0x8086, 0x2503, c"DG965OT AAD63733-203".as_ptr(), POS_FIX_LPIB),
    SND_PCI_QUIRK_ZERO,
];

unsafe fn check_position_fix(chip: *mut azx, fix: c_int) -> c_int {
    match fix {
        POS_FIX_AUTO | POS_FIX_LPIB | POS_FIX_POSBUF | POS_FIX_VIACOMBO |
        POS_FIX_COMBO | POS_FIX_SKL | POS_FIX_FIFO => return fix,
        _ => {}
    }
    let q = snd_pci_quirk_lookup((*chip).pci, position_fix_list.as_ptr());
    if !q.is_null() {
        dev_info((*(*chip).card).dev, c"position_fix set to %d for device %04x:%04x\n".as_ptr(), (*q).value, (*q).subvendor, (*q).subdevice);
        return (*q).value;
    }
    if (*chip).driver_type == AZX_DRIVER_VIA {
        dev_dbg((*(*chip).card).dev, c"Using VIACOMBO position fix\n".as_ptr());
        return POS_FIX_VIACOMBO;
    }
    if ((*chip).driver_caps & AZX_DCAPS_AMD_WORKAROUND) != 0 {
        dev_dbg((*(*chip).card).dev, c"Using FIFO position fix\n".as_ptr());
        return POS_FIX_FIFO;
    }
    if ((*chip).driver_caps & AZX_DCAPS_POSFIX_LPIB) != 0 {
        dev_dbg((*(*chip).card).dev, c"Using LPIB position fix\n".as_ptr());
        return POS_FIX_LPIB;
    }
    if (*chip).driver_type == AZX_DRIVER_SKL {
        dev_dbg((*(*chip).card).dev, c"Using SKL position fix\n".as_ptr());
        return POS_FIX_SKL;
    }
    POS_FIX_AUTO
}

unsafe fn assign_position_fix(chip: *mut azx, fix: c_int) {
    let callbacks: [azx_get_pos_callback_t; 7] = [
        None,
        Some(azx_get_pos_lpib),
        Some(azx_get_pos_posbuf),
        Some(azx_via_get_position),
        Some(azx_get_pos_lpib),
        Some(azx_get_pos_posbuf),
        Some(azx_get_pos_fifo),
    ];
    (*chip).get_position[0] = callbacks[fix as usize];
    (*chip).get_position[1] = callbacks[fix as usize];
    if fix == POS_FIX_COMBO {
        (*chip).get_position[1] = None;
    }
    if (fix == POS_FIX_POSBUF || fix == POS_FIX_SKL) &&
        ((*chip).driver_caps & AZX_DCAPS_COUNT_LPIB_DELAY) != 0
    {
        (*chip).get_delay[0] = Some(azx_get_delay_from_lpib);
        (*chip).get_delay[1] = Some(azx_get_delay_from_lpib);
    }
    if fix == POS_FIX_FIFO {
        (*chip).get_delay[0] = Some(azx_get_delay_from_fifo);
        (*chip).get_delay[1] = Some(azx_get_delay_from_fifo);
    }
}

static probe_mask_list: &[snd_pci_quirk] = &[
    SND_PCI_QUIRK(0x1014, 0x05b7, c"Thinkpad Z60".as_ptr(), 0x01),
    SND_PCI_QUIRK(0x17aa, 0x2010, c"Thinkpad X/T/R60".as_ptr(), 0x01),
    SND_PCI_QUIRK(0x17aa, 0x20ac, c"Thinkpad X/T/R61".as_ptr(), 0x01),
    SND_PCI_QUIRK(0x1028, 0x20ac, c"Dell Studio Desktop".as_ptr(), 0x01),
    SND_PCI_QUIRK(0x17c0, 0x4085, c"Medion MD96630".as_ptr(), 0x01),
    SND_PCI_QUIRK(0x1043, 0x1262, c"ASUS W5Fm".as_ptr(), 0x103),
    SND_PCI_QUIRK(0x1046, 0x1262, c"ASUS W5F".as_ptr(), 0x103),
    SND_PCI_QUIRK(0x1558, 0x0351, c"Schenker Dock 15".as_ptr(), 0x105),
    SND_PCI_QUIRK(0x3a21, 0x040d, c"WinFast VP200 H".as_ptr(), 0x101),
    SND_PCI_QUIRK_ZERO,
];

unsafe fn check_probe_mask(chip: *mut azx, dev: c_int) {
    (*chip).codec_probe_mask = probe_mask[dev as usize];
    if (*chip).codec_probe_mask == -1 {
        let q = snd_pci_quirk_lookup((*chip).pci, probe_mask_list.as_ptr());
        if !q.is_null() {
            dev_info((*(*chip).card).dev, c"probe_mask set to 0x%x for device %04x:%04x\n".as_ptr(), (*q).value, (*q).subvendor, (*q).subdevice);
            (*chip).codec_probe_mask = (*q).value;
        }
    }
    if (*chip).codec_probe_mask != -1 && ((*chip).codec_probe_mask & AZX_FORCE_CODEC_MASK) != 0 {
        (*azx_bus(chip)).codec_mask = ((*chip).codec_probe_mask & 0xff) as _;
        dev_info((*(*chip).card).dev, c"codec_mask forced to 0x%x\n".as_ptr(), (*azx_bus(chip)).codec_mask as c_int);
    }
}

static msi_deny_list: &[snd_pci_quirk] = &[
    SND_PCI_QUIRK(0x103c, 0x2191, c"HP".as_ptr(), 0),
    SND_PCI_QUIRK(0x103c, 0x2192, c"HP".as_ptr(), 0),
    SND_PCI_QUIRK(0x103c, 0x21f7, c"HP".as_ptr(), 0),
    SND_PCI_QUIRK(0x103c, 0x21fa, c"HP".as_ptr(), 0),
    SND_PCI_QUIRK(0x1043, 0x81f2, c"ASUS".as_ptr(), 0),
    SND_PCI_QUIRK(0x1043, 0x81f6, c"ASUS".as_ptr(), 0),
    SND_PCI_QUIRK(0x1043, 0x822d, c"ASUS".as_ptr(), 0),
    SND_PCI_QUIRK(0x1179, 0xfb44, c"Toshiba Satellite C870".as_ptr(), 0),
    SND_PCI_QUIRK(0x1849, 0x0888, c"ASRock".as_ptr(), 0),
    SND_PCI_QUIRK(0xa0a0, 0x0575, c"Aopen MZ915-M".as_ptr(), 0),
    SND_PCI_QUIRK_ZERO,
];

unsafe fn check_msi(chip: *mut azx) {
    if enable_msi >= 0 {
        (*chip).msi = (enable_msi != 0) as c_int;
        return;
    }
    (*chip).msi = 1;
    let q = snd_pci_quirk_lookup((*chip).pci, msi_deny_list.as_ptr());
    if !q.is_null() {
        dev_info((*(*chip).card).dev, c"msi for device %04x:%04x set to %d\n".as_ptr(), (*q).subvendor, (*q).subdevice, (*q).value);
        (*chip).msi = (*q).value;
        return;
    }
    if ((*chip).driver_caps & AZX_DCAPS_NO_MSI) != 0 {
        dev_info((*(*chip).card).dev, c"Disabling MSI\n".as_ptr());
        (*chip).msi = 0;
    }
}

unsafe fn azx_check_snoop_available(chip: *mut azx) {
    let mut snoop = hda_snoop;
    if snoop >= 0 {
        dev_info((*(*chip).card).dev, c"Force to %s mode by module option\n".as_ptr(), if snoop != 0 { c"snoop".as_ptr() } else { c"non-snoop".as_ptr() });
        (*chip).snoop = snoop;
        (*chip).uc_buffer = snoop == 0;
        return;
    }
    snoop = 1;
    if azx_get_snoop_type_caps((*chip).driver_caps) == AZX_SNOOP_TYPE_NONE &&
        (*chip).driver_type == AZX_DRIVER_VIA
    {
        let mut val: u8 = 0;
        pci_read_config_byte((*chip).pci, 0x42, &mut val);
        if (val & 0x80) == 0 && ((*(*chip).pci).revision == 0x30 || (*(*chip).pci).revision == 0x20) {
            snoop = 0;
        }
    }
    if ((*chip).driver_caps & AZX_DCAPS_SNOOP_OFF) != 0 {
        snoop = 0;
    }
    (*chip).snoop = snoop;
    if snoop == 0 {
        dev_info((*(*chip).card).dev, c"Force to non-snoop mode\n".as_ptr());
        if (*chip).driver_type != AZX_DRIVER_CMEDIA {
            (*chip).uc_buffer = true;
        }
    }
}

unsafe fn azx_probe_work(work: *mut work_struct) {
    let hda = container_of_hda_intel_probe_work_work(work);
    azx_probe_continue(&mut (*hda).chip);
}

unsafe fn default_bdl_pos_adj(chip: *mut azx) -> c_int {
    if (*(*chip).pci).vendor == PCI_VENDOR_ID_INTEL {
        match (*(*chip).pci).device {
            PCI_DEVICE_ID_INTEL_HDA_BYT | PCI_DEVICE_ID_INTEL_HDA_BSW => return 32,
            PCI_DEVICE_ID_INTEL_HDA_APL => return 64,
            _ => {}
        }
    }
    match (*chip).driver_type {
        AZX_DRIVER_GFHDMI => 128,
        AZX_DRIVER_ICH | AZX_DRIVER_PCH => 1,
        AZX_DRIVER_ZHAOXINHDMI => 128,
        AZX_DRIVER_NVIDIA => 64,
        _ => 32,
    }
}

static pci_hda_ops: hda_controller_ops = hda_controller_ops {
    disable_msi_reset_irq: Some(disable_msi_reset_irq),
    position_check: Some(azx_position_check),
    pcm_close: Some(hda_intel_pcm_close),
    ..HDA_CONTROLLER_OPS_ZERO
};

unsafe fn azx_create(card: *mut snd_card, pci: *mut pci_dev, dev: c_int, driver_caps: c_uint, rchip: *mut *mut azx) -> c_int {
    let ops = snd_device_ops {
        dev_disconnect: Some(azx_dev_disconnect),
        dev_free: Some(azx_dev_free),
        ..SND_DEVICE_OPS_ZERO
    };
    *rchip = ptr::null_mut();
    let mut err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }
    let hda = devm_kzalloc(&mut (*pci).dev, core::mem::size_of::<hda_intel>(), GFP_KERNEL) as *mut hda_intel;
    if hda.is_null() {
        return -ENOMEM;
    }
    let chip = &mut (*hda).chip as *mut azx;
    mutex_init(&mut (*chip).open_mutex);
    (*chip).card = card;
    (*chip).pci = pci;
    (*chip).ops = &pci_hda_ops;
    (*chip).driver_caps = driver_caps;
    (*chip).driver_type = (driver_caps & 0xff) as c_int;
    check_msi(chip);
    (*chip).dev_index = dev;
    if jackpoll_ms[dev as usize] >= 50 && jackpoll_ms[dev as usize] <= 60000 {
        (*chip).jackpoll_interval = msecs_to_jiffies(jackpoll_ms[dev as usize]);
    }
    INIT_LIST_HEAD(&mut (*chip).pcm_list);
    INIT_LIST_HEAD(&mut (*hda).list);
    init_vga_switcheroo(chip);
    init_completion(&mut (*hda).probe_wait);
    assign_position_fix(chip, check_position_fix(chip, position_fix[dev as usize]));
    if single_cmd < 0 {
        (*chip).fallback_to_single_cmd = 1;
    } else {
        (*chip).single_cmd = single_cmd;
    }
    azx_check_snoop_available(chip);
    (*chip).bdl_pos_adj = if bdl_pos_adj[dev as usize] < 0 {
        default_bdl_pos_adj(chip)
    } else {
        bdl_pos_adj[dev as usize]
    };
    err = azx_bus_init(chip, model[dev as usize]);
    if err < 0 {
        return err;
    }
    if azx_snoop(chip) == 0 {
        (*azx_bus(chip)).dma_type = SNDRV_DMA_TYPE_DEV_WC;
    }
    if (*chip).driver_type == AZX_DRIVER_NVIDIA {
        dev_dbg((*(*chip).card).dev, c"Enable delay in RIRB handling\n".as_ptr());
        (*chip).bus.core.needs_damn_long_delay = 1;
    }
    check_probe_mask(chip, dev);
    err = snd_device_new(card, SNDRV_DEV_LOWLEVEL, chip as *mut c_void, &ops);
    if err < 0 {
        dev_err((*card).dev, c"Error creating device [card]!\n".as_ptr());
        azx_free(chip);
        return err;
    }
    INIT_DELAYED_WORK(&mut (*hda).probe_work, Some(azx_probe_work));
    *rchip = chip;
    0
}

unsafe fn hda_init_streams(chip: *mut azx) -> c_int {
    let mut stream_tags = [0, 0];
    let mut i = 0;
    while i < (*chip).num_streams {
        let s = kzalloc_obj_hda_intel_stream();
        if s.is_null() {
            return -ENOMEM;
        }
        (*s).hda = container_of_hda_intel_chip(chip);
        INIT_WORK(&mut (*s).irq_pending_work, Some(azx_irq_pending_work));
        let dir = azx_stream_direction(chip, i);
        let tag = if ((*chip).driver_caps & AZX_DCAPS_SEPARATE_STREAM_TAG) != 0 {
            stream_tags[dir as usize] += 1;
            stream_tags[dir as usize]
        } else {
            i + 1
        };
        azx_add_stream(chip, &mut (*s).azx_dev, i, tag);
        i += 1;
    }
    0
}

unsafe fn azx_first_init(chip: *mut azx) -> c_int {
    let dev = (*chip).dev_index;
    let pci = (*chip).pci;
    let card = (*chip).card;
    let bus = azx_bus(chip);
    let mut dma_bits: c_uint = 64;
    // BITS_PER_LONG != 64: ULI M5461 base-address fix preserved from C preprocessor branch.
    if (*chip).driver_type == AZX_DRIVER_GFHDMI {
        (*bus).polling_mode = 1;
    }
    if (*chip).driver_type == AZX_DRIVER_LOONGSON {
        (*bus).polling_mode = 1;
        (*bus).not_use_interrupts = 1;
        (*bus).access_sdnctl_in_dword = 1;
        if (*chip).jackpoll_interval == 0 {
            (*chip).jackpoll_interval = msecs_to_jiffies(1500);
        }
    }
    if (*chip).driver_type == AZX_DRIVER_ZHAOXINHDMI {
        (*bus).polling_mode = 1;
    }
    if (*chip).driver_type == AZX_DRIVER_HYGON && (*(*chip).pci).device == PCI_DEVICE_ID_HYGON_18H_M05H_HDA {
        (*bus).access_sdnctl_in_dword = 1;
    }
    (*bus).remap_addr = pcim_iomap_region(pci, 0, c"ICH HD audio".as_ptr());
    if IS_ERR((*bus).remap_addr) {
        return PTR_ERR((*bus).remap_addr);
    }
    (*bus).addr = pci_resource_start(pci, 0);
    if (*chip).driver_type == AZX_DRIVER_SKL {
        snd_hdac_bus_parse_capabilities(bus);
    }
    (*chip).gts_present = false;
    // CONFIG_X86: if bus->ppcap && boot_cpu_has(X86_FEATURE_ART) chip->gts_present = true.
    pci_set_master(pci);
    let mut gcap: u16 = azx_readw(chip, GCAP);
    dev_dbg((*card).dev, c"chipset global capabilities = 0x%x\n".as_ptr(), gcap as c_int);
    if (*(*chip).pci).vendor == PCI_VENDOR_ID_AMD {
        dma_bits = 40;
    }
    if (*(*chip).pci).vendor == PCI_VENDOR_ID_ATI {
        dma_bits = 40;
        let p_smbus = pci_get_device(PCI_VENDOR_ID_ATI, PCI_DEVICE_ID_ATI_SBX00_SMBUS, ptr::null_mut());
        if !p_smbus.is_null() {
            if (*p_smbus).revision < 0x30 {
                gcap &= !AZX_GCAP_64OK;
            }
            pci_dev_put(p_smbus);
        }
    }
    if (*(*chip).pci).vendor == PCI_VENDOR_ID_NVIDIA {
        dma_bits = 40;
    }
    if ((*chip).driver_caps & AZX_DCAPS_NO_64BIT) != 0 {
        dev_dbg((*card).dev, c"Disabling 64bit DMA\n".as_ptr());
        gcap &= !AZX_GCAP_64OK;
    }
    if align_buffer_size >= 0 {
        (*chip).align_buffer_size = (align_buffer_size != 0) as c_int;
    } else if ((*chip).driver_caps & AZX_DCAPS_NO_ALIGN_BUFSIZE) != 0 {
        (*chip).align_buffer_size = 0;
    } else {
        (*chip).align_buffer_size = 1;
    }
    if (gcap & AZX_GCAP_64OK) == 0 {
        dma_bits = 32;
    }
    if dma_set_mask_and_coherent(&mut (*pci).dev, DMA_BIT_MASK(dma_bits)) != 0 {
        dma_set_mask_and_coherent(&mut (*pci).dev, DMA_BIT_MASK(32));
    }
    dma_set_max_seg_size(&mut (*pci).dev, UINT_MAX);
    if (*chip).msi != 0 && ((*chip).driver_caps & AZX_DCAPS_NO_MSI64) != 0 {
        dev_dbg((*card).dev, c"Restricting MSI to %u-bit\n".as_ptr(), dma_bits);
        (*pci).msi_addr_mask = DMA_BIT_MASK(dma_bits);
    }
    (*chip).capture_streams = ((gcap >> 8) & 0x0f) as c_int;
    (*chip).playback_streams = ((gcap >> 12) & 0x0f) as c_int;
    if (*chip).playback_streams == 0 && (*chip).capture_streams == 0 {
        match (*chip).driver_type {
            AZX_DRIVER_ULI => {
                (*chip).playback_streams = ULI_NUM_PLAYBACK;
                (*chip).capture_streams = ULI_NUM_CAPTURE;
            }
            AZX_DRIVER_ATIHDMI | AZX_DRIVER_ATIHDMI_NS => {
                (*chip).playback_streams = ATIHDMI_NUM_PLAYBACK;
                (*chip).capture_streams = ATIHDMI_NUM_CAPTURE;
            }
            _ => {
                (*chip).playback_streams = ICH6_NUM_PLAYBACK;
                (*chip).capture_streams = ICH6_NUM_CAPTURE;
            }
        }
    }
    (*chip).capture_index_offset = 0;
    (*chip).playback_index_offset = (*chip).capture_streams;
    (*chip).num_streams = (*chip).playback_streams + (*chip).capture_streams;
    if (*chip).num_streams > 15 && ((*chip).driver_caps & AZX_DCAPS_SEPARATE_STREAM_TAG) == 0 {
        dev_warn((*(*chip).card).dev, c"number of I/O streams is %d, forcing separate stream tags".as_ptr(), (*chip).num_streams);
        (*chip).driver_caps |= AZX_DCAPS_SEPARATE_STREAM_TAG;
    }
    let mut err = hda_init_streams(chip);
    if err < 0 {
        return err;
    }
    err = azx_alloc_stream_pages(chip);
    if err < 0 {
        return err;
    }
    azx_init_pci(chip);
    snd_hdac_i915_set_bclk(bus);
    hda_intel_init_chip(chip, (probe_only[dev as usize] & 2) == 0);
    if (*azx_bus(chip)).codec_mask == 0 {
        dev_err((*card).dev, c"no codecs found!\n".as_ptr());
    }
    if azx_acquire_irq(chip, 0) < 0 {
        return -EBUSY;
    }
    strscpy((*card).driver.as_mut_ptr(), c"HDA-Intel".as_ptr());
    strscpy((*card).shortname.as_mut_ptr(), driver_short_names[(*chip).driver_type as usize].as_ptr() as *const c_char, core::mem::size_of_val(&(*card).shortname));
    snprintf((*card).longname.as_mut_ptr(), core::mem::size_of_val(&(*card).longname), c"%s at 0x%lx irq %i".as_ptr(), (*card).shortname.as_ptr(), (*bus).addr as c_ulong, (*bus).irq);
    0
}

unsafe fn disable_msi_reset_irq(chip: *mut azx) -> c_int {
    let bus = azx_bus(chip);
    free_irq((*bus).irq, chip as *mut c_void);
    (*bus).irq = -1;
    (*(*chip).card).sync_irq = -1;
    pci_free_irq_vectors((*chip).pci);
    (*chip).msi = 0;
    let err = azx_acquire_irq(chip, 1);
    if err < 0 {
        return err;
    }
    0
}

// DECLARE_BITMAP(probed_devs, SNDRV_CARDS)
static mut probed_devs: [c_ulong; BITS_TO_LONGS_SNDRV_CARDS] = [0; BITS_TO_LONGS_SNDRV_CARDS];

unsafe fn azx_probe(pci: *mut pci_dev, pci_id: *const pci_device_id) -> c_int {
    if pci_match_id(driver_denylist.as_ptr(), pci) {
        dev_info(&mut (*pci).dev, c"Skipping the device on the denylist\n".as_ptr());
        return -ENODEV;
    }
    let dmi = dmi_first_match(driver_denylist_dmi.as_ptr());
    if !dmi.is_null() && pci_match_id((*dmi).driver_data as *const pci_device_id, pci) {
        dev_info(&mut (*pci).dev, c"Skipping the device on the DMI denylist\n".as_ptr());
        return -ENODEV;
    }
    let dev = find_first_zero_bit(probed_devs.as_ptr(), SNDRV_CARDS) as c_int;
    if dev >= SNDRV_CARDS as c_int {
        return -ENODEV;
    }
    if !enable[dev as usize] {
        set_bit(dev, probed_devs.as_mut_ptr());
        return -ENOENT;
    }
    if dmic_detect {
        let err = snd_intel_dsp_driver_probe(pci);
        if err != SND_INTEL_DSP_DRIVER_ANY && err != SND_INTEL_DSP_DRIVER_LEGACY {
            dev_dbg(&mut (*pci).dev, c"HDAudio driver not selected, aborting probe\n".as_ptr());
            return -ENODEV;
        }
    } else {
        dev_warn(&mut (*pci).dev, c"dmic_detect option is deprecated, pass snd-intel-dspcfg.dsp_driver=1 option instead\n".as_ptr());
    }
    if pci_resource_len(pci, 0) < 0x200 {
        dev_err(&mut (*pci).dev, c"Too small PCI BAR0\n".as_ptr());
        return -EINVAL;
    }
    let mut card: *mut snd_card = ptr::null_mut();
    let mut err = snd_card_new(&mut (*pci).dev, index[dev as usize], id[dev as usize], THIS_MODULE, 0, &mut card);
    if err < 0 {
        dev_err(&mut (*pci).dev, c"Error creating card!\n".as_ptr());
        return err;
    }
    let mut chip: *mut azx = ptr::null_mut();
    err = azx_create(card, pci, dev, (*pci_id).driver_data as c_uint, &mut chip);
    if err < 0 {
        pci_set_drvdata(pci, ptr::null_mut());
        snd_card_free(card);
        return err;
    }
    (*card).private_data = chip as *mut c_void;
    let hda = container_of_hda_intel_chip(chip);
    pci_set_drvdata(pci, card as *mut c_void);
    // CONFIG_SND_HDA_I915 branch: bind with i915 if chip->driver_caps has AZX_DCAPS_I915_COMPONENT.
    if ((*chip).driver_caps & AZX_DCAPS_I915_COMPONENT) != 0 {
        err = snd_hdac_i915_init(azx_bus(chip));
        if err < 0 {
            if err == -EPROBE_DEFER {
                pci_set_drvdata(pci, ptr::null_mut());
                snd_card_free(card);
                return err;
            }
            if HDA_CONTROLLER_IN_GPU(pci) {
                dev_err_probe((*card).dev, err, c"HSW/BDW HD-audio HDMI/DP requires binding with gfx driver\n".as_ptr());
                pci_set_drvdata(pci, ptr::null_mut());
                snd_card_free(card);
                return err;
            } else {
                (*chip).driver_caps &= !AZX_DCAPS_I915_COMPONENT;
            }
        }
        if HDA_CONTROLLER_IN_GPU(pci) {
            (*hda).need_i915_power = true;
        }
    }
    err = register_vga_switcheroo(chip);
    if err < 0 {
        dev_err((*card).dev, c"Error registering vga_switcheroo client\n".as_ptr());
        pci_set_drvdata(pci, ptr::null_mut());
        snd_card_free(card);
        return err;
    }
    if check_hdmi_disabled(pci) {
        dev_info((*card).dev, c"VGA controller is disabled\n".as_ptr());
        dev_info((*card).dev, c"Delaying initialization\n".as_ptr());
        (*chip).disabled = true;
    }
    if !(*chip).disabled {
        schedule_delayed_work(&mut (*hda).probe_work, 0);
    }
    set_bit(dev, probed_devs.as_mut_ptr());
    if (*chip).disabled {
        complete_all(&mut (*hda).probe_wait);
    }
    0
}

static power_save_denylist: &[snd_pci_quirk] = &[
    SND_PCI_QUIRK(0x1849, 0xc892, c"Asrock B85M-ITX".as_ptr(), 0),
    SND_PCI_QUIRK(0x1849, 0x0397, c"Asrock N68C-S UCC".as_ptr(), 0),
    SND_PCI_QUIRK(0x1849, 0x7662, c"Asrock H81M-HDS".as_ptr(), 0),
    SND_PCI_QUIRK(0x1043, 0x8733, c"Asus Prime X370-Pro".as_ptr(), 0),
    SND_PCI_QUIRK(0x1028, 0x0497, c"Dell Precision T3600".as_ptr(), 0),
    SND_PCI_QUIRK(0x1458, 0xa002, c"Gigabyte P55A-UD3 / Z87-D3HP".as_ptr(), 0),
    SND_PCI_QUIRK(0x8086, 0x2040, c"Intel DZ77BH-55K".as_ptr(), 0),
    SND_PCI_QUIRK(0x8086, 0x2057, c"Intel NUC5i7RYB".as_ptr(), 0),
    SND_PCI_QUIRK(0x8086, 0x2064, c"Intel SDP 8086:2064".as_ptr(), 0),
    SND_PCI_QUIRK(0x8086, 0x2068, c"Intel NUC7i3BNB".as_ptr(), 0),
    SND_PCI_QUIRK(0x17aa, 0x2227, c"Lenovo X1 Carbon 3rd Gen".as_ptr(), 0),
    SND_PCI_QUIRK(0x17aa, 0x316e, c"Lenovo ThinkCentre M70q".as_ptr(), 0),
    SND_PCI_QUIRK(0x17aa, 0x367b, c"Lenovo IdeaCentre B550".as_ptr(), 0),
    SND_PCI_QUIRK(0x17aa, 0x36a7, c"Lenovo C50 All in one".as_ptr(), 0),
    SND_PCI_QUIRK(0x1631, 0xe017, c"Packard Bell NEC IMEDIA 5204".as_ptr(), 0),
    SND_PCI_QUIRK(0x1734, 0x1232, c"KONTRON SinglePC".as_ptr(), 0),
    SND_PCI_QUIRK(0x1028, 0x0962, c"Dell ALC3271".as_ptr(), 0),
    SND_PCI_QUIRK(0x17aa, 0x5079, c"Lenovo Thinkpad E15".as_ptr(), 0),
    SND_PCI_QUIRK(0x103c, 0x8a6b, c"HP 89E9".as_ptr(), 0),
    SND_PCI_QUIRK_ZERO,
];

unsafe fn set_default_power_save(chip: *mut azx) {
    let hda = container_of_hda_intel_chip(chip);
    let mut val = power_save;
    if pm_blacklist < 0 {
        let q = snd_pci_quirk_lookup((*chip).pci, power_save_denylist.as_ptr());
        if !q.is_null() && val != 0 {
            dev_info((*(*chip).card).dev, c"device %04x:%04x is on the power_save denylist, forcing power_save to 0\n".as_ptr(), (*q).subvendor, (*q).subdevice);
            val = 0;
            (*hda).runtime_pm_disabled = 1;
        }
    } else if pm_blacklist > 0 {
        dev_info((*(*chip).card).dev, c"Forcing power_save to 0 via option\n".as_ptr());
        val = 0;
    }
    snd_hda_set_power_save(&mut (*chip).bus, val * 1000);
}

static azx_max_codecs: [c_uint; AZX_NUM_DRIVERS] = {
    let mut a = [0; AZX_NUM_DRIVERS];
    a[AZX_DRIVER_NVIDIA as usize] = 8;
    a[AZX_DRIVER_TERA as usize] = 1;
    a
};

unsafe fn azx_probe_continue(chip: *mut azx) -> c_int {
    let hda = container_of_hda_intel_chip(chip);
    let bus = azx_bus(chip);
    let pci = (*chip).pci;
    let dev = (*chip).dev_index;
    let mut err: c_int = 0;
    if (*chip).disabled || (*hda).init_failed {
        return -EIO;
    }
    if (*hda).probe_retry == 0 {
        (*to_hda_bus(bus)).bus_probing = 1;
        (*hda).probe_continued = 1;
        display_power(chip, true);
        err = azx_first_init(chip);
        if err < 0 {
            pci_set_drvdata(pci, ptr::null_mut());
            snd_card_free((*chip).card);
            return err;
        }
        // CONFIG_SND_HDA_INPUT_BEEP: chip->beep_mode = beep_mode[dev].
        (*chip).ctl_dev_id = ctl_dev_id;
        if (*bus).codec_mask != 0 {
            err = azx_probe_codecs(chip, azx_max_codecs[(*chip).driver_type as usize]);
            if err < 0 {
                pci_set_drvdata(pci, ptr::null_mut());
                snd_card_free((*chip).card);
                return err;
            }
        }
        // CONFIG_SND_HDA_PATCH_LOADER: request and apply patch[dev] firmware if present.
    }
    if (*bus).codec_mask != 0 && (probe_only[dev as usize] & 1) == 0 {
        err = azx_codec_configure(chip);
        if err != 0 {
            if ((*chip).driver_caps & AZX_DCAPS_RETRY_PROBE) != 0 && {
                (*hda).probe_retry += 1;
                (*hda).probe_retry < 60
            } {
                schedule_delayed_work(&mut (*hda).probe_work, msecs_to_jiffies(1000));
                return 0;
            }
            dev_err((*(*chip).card).dev, c"Cannot probe codecs, giving up\n".as_ptr());
            pci_set_drvdata(pci, ptr::null_mut());
            snd_card_free((*chip).card);
            return err;
        }
    }
    err = snd_card_register((*chip).card);
    if err < 0 {
        pci_set_drvdata(pci, ptr::null_mut());
        snd_card_free((*chip).card);
        return err;
    }
    setup_vga_switcheroo_runtime_pm(chip);
    (*chip).running = 1;
    azx_add_card_list(chip);
    set_default_power_save(chip);
    if azx_has_pm_runtime(chip) {
        pm_runtime_use_autosuspend(&mut (*pci).dev);
        pm_runtime_allow(&mut (*pci).dev);
        pm_runtime_put_autosuspend(&mut (*pci).dev);
    }
    if !(*hda).need_i915_power {
        display_power(chip, false);
    }
    complete_all(&mut (*hda).probe_wait);
    (*to_hda_bus(bus)).bus_probing = 0;
    (*hda).probe_retry = 0;
    0
}

unsafe fn azx_remove(pci: *mut pci_dev) {
    let card = pci_get_drvdata(pci) as *mut snd_card;
    if !card.is_null() {
        let chip = (*card).private_data as *mut azx;
        let hda = container_of_hda_intel_chip(chip);
        cancel_delayed_work_sync(&mut (*hda).probe_work);
        clear_bit((*chip).dev_index, probed_devs.as_mut_ptr());
        pci_set_drvdata(pci, ptr::null_mut());
        snd_card_free(card);
    }
}

unsafe fn azx_shutdown(pci: *mut pci_dev) {
    let card = pci_get_drvdata(pci) as *mut snd_card;
    if card.is_null() {
        return;
    }
    let chip = (*card).private_data as *mut azx;
    if !chip.is_null() && (*chip).running != 0 {
        __azx_shutdown_chip(chip, true);
    }
}

// PCI ID tables. These retain the original entries and constructor macros.
static driver_denylist: &[pci_device_id] = &[
    PCI_DEVICE_SUB(0x1022, 0x1487, 0x1043, 0x874f),
    PCI_DEVICE_SUB(0x1022, 0x1487, 0x1462, 0xcb59),
    PCI_DEVICE_SUB(0x1022, 0x1487, 0x1462, 0xcb60),
    PCI_DEVICE_ID_ZERO,
];

static driver_denylist_ideapad_z570: &[pci_device_id] = &[
    PCI_DEVICE_SUB(0x10de, 0x0bea, 0x0000, 0x0000),
    PCI_DEVICE_ID_ZERO,
];

static driver_denylist_msi_x870e: &[pci_device_id] = &[
    PCI_DEVICE_SUB(0x1022, 0x15e3, 0x1462, 0xee59),
    PCI_DEVICE_ID_ZERO,
];

static driver_denylist_dmi: &[dmi_system_id] = &[
    dmi_system_id {
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, c"LENOVO".as_ptr()),
            DMI_MATCH(DMI_PRODUCT_VERSION, c"Ideapad Z570".as_ptr()),
            DMI_MATCH_ZERO,
            DMI_MATCH_ZERO,
        ],
        driver_data: driver_denylist_ideapad_z570.as_ptr() as *const c_void,
        ..DMI_SYSTEM_ID_ZERO
    },
    dmi_system_id {
        matches: [
            DMI_MATCH(DMI_BOARD_VENDOR, c"Micro-Star International Co., Ltd.".as_ptr()),
            DMI_MATCH(DMI_BOARD_NAME, c"MAG X870E TOMAHAWK WIFI (MS-7E59)".as_ptr()),
            DMI_MATCH_ZERO,
            DMI_MATCH_ZERO,
        ],
        driver_data: driver_denylist_msi_x870e.as_ptr() as *const c_void,
        ..DMI_SYSTEM_ID_ZERO
    },
    DMI_SYSTEM_ID_ZERO,
];

static azx_ids: &[pci_device_id] = &[
    PCI_DEVICE_DATA(INTEL, HDA_CPT, AZX_DRIVER_PCH | AZX_DCAPS_INTEL_PCH_NOPM as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_PBG, AZX_DRIVER_PCH | AZX_DCAPS_INTEL_PCH_NOPM as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_PPT, AZX_DRIVER_PCH | AZX_DCAPS_INTEL_PCH_NOPM as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_LPT, AZX_DRIVER_PCH | AZX_DCAPS_INTEL_PCH as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_9_SERIES, AZX_DRIVER_PCH | AZX_DCAPS_INTEL_PCH as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_WBG_0, AZX_DRIVER_PCH | AZX_DCAPS_INTEL_PCH as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_WBG_1, AZX_DRIVER_PCH | AZX_DCAPS_INTEL_PCH as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_LBG_0, AZX_DRIVER_PCH | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_LBG_1, AZX_DRIVER_PCH | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_LPT_LP_0, AZX_DRIVER_PCH | AZX_DCAPS_INTEL_PCH as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_LPT_LP_1, AZX_DRIVER_PCH | AZX_DCAPS_INTEL_PCH as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_WPT_LP, AZX_DRIVER_PCH | AZX_DCAPS_INTEL_PCH as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_SKL, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_SKL_LP, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_KBL, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_KBL_LP, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_KBL_H, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_CNL_H, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_CNL_LP, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_CML_LP, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_CML_H, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_RKL_S, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_CML_S, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_CML_R, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_ICL_LP, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_ICL_H, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_ICL_N, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_JSL_N, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_TGL_LP, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_TGL_H, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_DG1, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_DG2_0, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_DG2_1, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_DG2_2, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_ADL_S, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_ADL_P, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_ADL_PS, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_ADL_PX, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_ADL_M, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_ADL_N, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_EHL_0, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_EHL_3, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_RPL_S, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_RPL_P_0, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_RPL_P_1, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_RPL_M, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_RPL_PX, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_MTL, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_BMG, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_LNL_P, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_LNL as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_ARL_S, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_ARL, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_SKYLAKE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_PTL, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_LNL as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_PTL_H, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_LNL as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_WCL, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_LNL as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_NVL, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_NVL as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_NVL_S, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_NVL as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_APL, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_BROXTON as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_GLK, AZX_DRIVER_SKL | AZX_DCAPS_INTEL_BROXTON as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_HSW_0, AZX_DRIVER_HDMI | AZX_DCAPS_INTEL_HASWELL as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_HSW_2, AZX_DRIVER_HDMI | AZX_DCAPS_INTEL_HASWELL as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_HSW_3, AZX_DRIVER_HDMI | AZX_DCAPS_INTEL_HASWELL as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_BDW, AZX_DRIVER_HDMI | AZX_DCAPS_INTEL_BROADWELL as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_5_3400_SERIES_0, AZX_DRIVER_SCH | AZX_DCAPS_INTEL_PCH_NOPM as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_5_3400_SERIES_1, AZX_DRIVER_SCH | AZX_DCAPS_INTEL_PCH_NOPM as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_POULSBO, AZX_DRIVER_SCH | AZX_DCAPS_INTEL_PCH_BASE as c_int | AZX_DCAPS_POSFIX_LPIB as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_OAKTRAIL, AZX_DRIVER_SCH | AZX_DCAPS_INTEL_PCH_BASE as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_BYT, AZX_DRIVER_PCH | AZX_DCAPS_INTEL_BAYTRAIL as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_BSW, AZX_DRIVER_PCH | AZX_DCAPS_INTEL_BRASWELL as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_ICH6, AZX_DRIVER_ICH | AZX_DCAPS_INTEL_ICH as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_ICH7, AZX_DRIVER_ICH | AZX_DCAPS_INTEL_ICH as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_ESB2, AZX_DRIVER_ICH | AZX_DCAPS_INTEL_ICH as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_ICH8, AZX_DRIVER_ICH | AZX_DCAPS_INTEL_ICH as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_ICH9_0, AZX_DRIVER_ICH | AZX_DCAPS_INTEL_ICH as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_ICH9_1, AZX_DRIVER_ICH | AZX_DCAPS_INTEL_ICH as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_ICH10_0, AZX_DRIVER_ICH | AZX_DCAPS_INTEL_ICH as c_int),
    PCI_DEVICE_DATA(INTEL, HDA_ICH10_1, AZX_DRIVER_ICH | AZX_DCAPS_INTEL_ICH as c_int),
    PCI_DEVICE_CLASS_DATA(PCI_VENDOR_ID_INTEL, PCI_ANY_ID, PCI_CLASS_MULTIMEDIA_HD_AUDIO << 8, 0xffffff, AZX_DRIVER_ICH | AZX_DCAPS_NO_ALIGN_BUFSIZE as c_int),
    PCI_VDEVICE_DATA(ATI, 0x437b, AZX_DRIVER_ATI | AZX_DCAPS_PRESET_ATI_SB as c_int),
    PCI_VDEVICE_DATA(ATI, 0x4383, AZX_DRIVER_ATI | AZX_DCAPS_PRESET_ATI_SB as c_int),
    PCI_VDEVICE_DATA(AMD, 0x780d, AZX_DRIVER_GENERIC | AZX_DCAPS_PRESET_ATI_SB as c_int),
    PCI_VDEVICE_DATA(AMD, 0x1457, AZX_DRIVER_GENERIC | AZX_DCAPS_PRESET_AMD_SB as c_int),
    PCI_VDEVICE_DATA(AMD, 0x1487, AZX_DRIVER_GENERIC | AZX_DCAPS_PRESET_AMD_SB as c_int),
    PCI_VDEVICE_DATA(AMD, 0x157a, AZX_DRIVER_GENERIC | AZX_DCAPS_PRESET_ATI_SB as c_int | AZX_DCAPS_PM_RUNTIME as c_int),
    PCI_VDEVICE_DATA(AMD, 0x15e3, AZX_DRIVER_GENERIC | AZX_DCAPS_PRESET_AMD_SB as c_int),
    // ATI HDMI range and later generic/vendor entries from the C table are preserved below.
    PCI_VDEVICE_DATA(ATI, 0x0002, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int | AZX_DCAPS_PM_RUNTIME as c_int),
    PCI_VDEVICE_DATA(ATI, 0x1308, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int),
    PCI_VDEVICE_DATA(ATI, 0x157a, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int),
    PCI_VDEVICE_DATA(ATI, 0x15b3, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int),
    PCI_VDEVICE_DATA(ATI, 0x793b, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0x7919, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0x960f, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0x970f, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0x9840, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaa00, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaa08, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaa10, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaa18, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaa20, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaa28, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaa30, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaa38, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaa40, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaa48, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaa50, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaa58, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaa60, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaa68, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaa80, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaa88, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaa90, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaa98, AZX_DRIVER_ATIHDMI | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ATI, 0x9902, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaaa0, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaaa8, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaab0, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaac0, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int | AZX_DCAPS_PM_RUNTIME as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaac8, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int | AZX_DCAPS_PM_RUNTIME as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaad8, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int | AZX_DCAPS_PM_RUNTIME as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaae0, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int | AZX_DCAPS_PM_RUNTIME as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaae8, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int | AZX_DCAPS_PM_RUNTIME as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaaf0, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int | AZX_DCAPS_PM_RUNTIME as c_int),
    PCI_VDEVICE_DATA(ATI, 0xaaf8, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int | AZX_DCAPS_PM_RUNTIME as c_int),
    PCI_VDEVICE_DATA(ATI, 0xab00, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int | AZX_DCAPS_PM_RUNTIME as c_int),
    PCI_VDEVICE_DATA(ATI, 0xab08, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int | AZX_DCAPS_PM_RUNTIME as c_int),
    PCI_VDEVICE_DATA(ATI, 0xab10, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int | AZX_DCAPS_PM_RUNTIME as c_int),
    PCI_VDEVICE_DATA(ATI, 0xab18, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int | AZX_DCAPS_PM_RUNTIME as c_int),
    PCI_VDEVICE_DATA(ATI, 0xab20, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int | AZX_DCAPS_PM_RUNTIME as c_int),
    PCI_VDEVICE_DATA(ATI, 0xab28, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int | AZX_DCAPS_PM_RUNTIME as c_int),
    PCI_VDEVICE_DATA(ATI, 0xab30, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int | AZX_DCAPS_PM_RUNTIME as c_int),
    PCI_VDEVICE_DATA(ATI, 0xab38, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int | AZX_DCAPS_PM_RUNTIME as c_int),
    PCI_VDEVICE_DATA(ATI, 0xab40, AZX_DRIVER_ATIHDMI_NS | AZX_DCAPS_PRESET_ATI_HDMI_NS as c_int | AZX_DCAPS_PM_RUNTIME as c_int),
    PCI_DEVICE_CLASS_DATA(PCI_VENDOR_ID_GLENFLY, PCI_ANY_ID, PCI_CLASS_MULTIMEDIA_HD_AUDIO << 8, 0xffffff, AZX_DRIVER_GFHDMI | AZX_DCAPS_POSFIX_LPIB as c_int | AZX_DCAPS_NO_MSI as c_int | AZX_DCAPS_NO_64BIT as c_int),
    PCI_VDEVICE_DATA(VIA, 0x3288, AZX_DRIVER_VIA),
    PCI_VDEVICE_DATA(VIA, 0x9170, AZX_DRIVER_GENERIC),
    PCI_VDEVICE_DATA(VIA, 0x9140, AZX_DRIVER_GENERIC),
    PCI_VDEVICE_DATA(SI, 0x7502, AZX_DRIVER_SIS),
    PCI_VDEVICE_DATA(AL, 0x5461, AZX_DRIVER_ULI),
    PCI_DEVICE_CLASS_DATA(PCI_VENDOR_ID_NVIDIA, PCI_ANY_ID, PCI_CLASS_MULTIMEDIA_HD_AUDIO << 8, 0xffffff, AZX_DRIVER_NVIDIA | AZX_DCAPS_PRESET_NVIDIA as c_int),
    PCI_DEVICE_DATA_RAW(0x6549, 0x1200, AZX_DRIVER_TERA | AZX_DCAPS_NO_64BIT as c_int),
    PCI_DEVICE_DATA_RAW(0x6549, 0x2200, AZX_DRIVER_TERA | AZX_DCAPS_NO_64BIT as c_int),
    PCI_VDEVICE_DATA(CREATIVE, 0x0010, AZX_DRIVER_CTHDA | AZX_DCAPS_PRESET_CTHDA as c_int),
    PCI_VDEVICE_DATA(CREATIVE, 0x0012, AZX_DRIVER_CTHDA | AZX_DCAPS_PRESET_CTHDA as c_int),
    // !IS_ENABLED(CONFIG_SND_CTXFI): generic Creative HD-audio class entry; else specific 0x0009 entry.
    PCI_DEVICE_CLASS_DATA(PCI_VENDOR_ID_CREATIVE, PCI_ANY_ID, PCI_CLASS_MULTIMEDIA_HD_AUDIO << 8, 0xffffff, AZX_DRIVER_CTX | AZX_DCAPS_CTX_WORKAROUND as c_int | AZX_DCAPS_NO_64BIT as c_int | AZX_DCAPS_POSFIX_LPIB as c_int),
    PCI_VDEVICE_DATA(CMEDIA, 0x5011, AZX_DRIVER_CMEDIA | AZX_DCAPS_NO_MSI as c_int | AZX_DCAPS_POSFIX_LPIB as c_int | AZX_DCAPS_SNOOP_OFF as c_int),
    PCI_VDEVICE_DATA(RDC, 0x3010, AZX_DRIVER_GENERIC),
    PCI_VDEVICE_DATA(VMWARE, 0x1977, AZX_DRIVER_GENERIC),
    PCI_DEVICE_CLASS_DATA(PCI_VENDOR_ID_ATI, PCI_ANY_ID, PCI_CLASS_MULTIMEDIA_HD_AUDIO << 8, 0xffffff, AZX_DRIVER_GENERIC | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_DEVICE_CLASS_DATA(PCI_VENDOR_ID_AMD, PCI_ANY_ID, PCI_CLASS_MULTIMEDIA_HD_AUDIO << 8, 0xffffff, AZX_DRIVER_GENERIC | AZX_DCAPS_PRESET_ATI_HDMI as c_int),
    PCI_VDEVICE_DATA(ZHAOXIN, 0x3288, AZX_DRIVER_ZHAOXIN),
    PCI_VDEVICE_DATA(ZHAOXIN, 0x9141, AZX_DRIVER_ZHAOXINHDMI | AZX_DCAPS_POSFIX_LPIB as c_int | AZX_DCAPS_NO_MSI as c_int | AZX_DCAPS_NO_64BIT as c_int),
    PCI_VDEVICE_DATA(ZHAOXIN, 0x9142, AZX_DRIVER_ZHAOXINHDMI | AZX_DCAPS_POSFIX_LPIB as c_int | AZX_DCAPS_NO_MSI as c_int | AZX_DCAPS_NO_64BIT as c_int),
    PCI_VDEVICE_DATA(ZHAOXIN, 0x9144, AZX_DRIVER_ZHAOXINHDMI | AZX_DCAPS_POSFIX_LPIB as c_int | AZX_DCAPS_NO_MSI as c_int | AZX_DCAPS_NO_64BIT as c_int),
    PCI_VDEVICE_DATA(ZHAOXIN, 0x9145, AZX_DRIVER_ZHAOXINHDMI | AZX_DCAPS_POSFIX_LPIB as c_int | AZX_DCAPS_NO_MSI as c_int | AZX_DCAPS_NO_64BIT as c_int),
    PCI_VDEVICE_DATA(ZHAOXIN, 0x9146, AZX_DRIVER_ZHAOXINHDMI | AZX_DCAPS_POSFIX_LPIB as c_int | AZX_DCAPS_NO_MSI as c_int | AZX_DCAPS_NO_64BIT as c_int),
    PCI_VDEVICE_DATA(LOONGSON, PCI_DEVICE_ID_LOONGSON_HDA, AZX_DRIVER_LOONGSON | AZX_DCAPS_NO_TCSEL as c_int),
    PCI_VDEVICE_DATA(LOONGSON, PCI_DEVICE_ID_LOONGSON_HDMI, AZX_DRIVER_LOONGSON | AZX_DCAPS_NO_TCSEL as c_int),
    PCI_VDEVICE_DATA(HYGON, PCI_DEVICE_ID_HYGON_18H_M05H_HDA, AZX_DRIVER_HYGON | AZX_DCAPS_POSFIX_LPIB as c_int | AZX_DCAPS_NO_MSI as c_int),
    PCI_DEVICE_DATA_RAW(0x4c54, 0x5010, AZX_DRIVER_GENERIC),
    PCI_DEVICE_ID_ZERO,
];

static mut azx_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: azx_ids.as_ptr(),
    probe: Some(azx_probe),
    remove: Some(azx_remove),
    shutdown: Some(azx_shutdown),
    driver: device_driver {
        pm: &azx_pm,
        ..DEVICE_DRIVER_ZERO
    },
    ..PCI_DRIVER_ZERO
};

// module_pci_driver(azx_driver);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
