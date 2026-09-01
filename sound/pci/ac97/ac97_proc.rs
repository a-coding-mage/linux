// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Universal interface for Audio Codec '97
 *
 *  For more details look to AC '97 component specification revision 2.2
 *  by Intel Corporation (http://developer.intel.com).
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/*
 * Dependencies from linux/mutex.h, sound/core.h, sound/ac97_codec.h,
 * sound/asoundef.h, ac97_local.h, and ac97_id.h are intentionally referenced
 * as external items supplied by the surrounding kernel translation.
 */

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub proc_root: *mut snd_info_entry,
}

#[repr(C)]
pub struct snd_ac97_bus {
    pub card: *mut snd_card,
    pub proc: *mut snd_info_entry,
    pub num: c_int,
}

#[repr(C)]
pub struct snd_info_entry_text {
    pub write: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
}

#[repr(C)]
pub union snd_info_entry_c {
    pub text: snd_info_entry_text,
}

#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut c_void,
    pub mode: c_uint,
    pub c: snd_info_entry_c,
}

#[repr(C)]
pub struct snd_ac97_ad18xx {
    pub id: [c_uint; 3],
    pub unchained: [c_uint; 3],
    pub chained: [c_uint; 3],
}

#[repr(C)]
pub union snd_ac97_spec {
    pub ad18xx: snd_ac97_ad18xx,
}

#[repr(C)]
pub struct snd_ac97 {
    pub id: c_uint,
    pub addr: c_uint,
    pub num: c_uint,
    pub scaps: c_uint,
    pub subsystem_vendor: c_uint,
    pub subsystem_device: c_uint,
    pub flags: c_uint,
    pub ext_id: c_uint,
    pub caps: c_uint,
    pub regs: [c_ushort; 0x80],
    pub spec: snd_ac97_spec,
    pub page_mutex: mutex,
    pub bus: *mut snd_ac97_bus,
    pub proc: *mut snd_info_entry,
    pub proc_regs: *mut snd_info_entry,
}

#[allow(non_camel_case_types)]
pub type c_ushort = u16;

unsafe extern "C" {
    fn snd_ac97_write(ac97: *mut snd_ac97, reg: c_uint, val: c_uint);
    fn snd_ac97_write_cache(ac97: *mut snd_ac97, reg: c_uint, val: c_uint);
    fn snd_ac97_read(ac97: *mut snd_ac97, reg: c_uint) -> c_ushort;
    fn snd_ac97_update_bits(ac97: *mut snd_ac97, reg: c_uint, mask: c_uint, value: c_uint) -> c_int;
    fn snd_ac97_get_name(
        bus: *mut c_void,
        id: c_uint,
        name: *mut c_char,
        len: usize,
        modem: c_int,
    );
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_info_get_line(buffer: *mut snd_info_buffer, line: *mut c_char, len: c_int) -> c_int;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snd_info_create_card_entry(
        card: *mut snd_card,
        name: *const c_char,
        parent: *mut snd_info_entry,
    ) -> *mut snd_info_entry;
    fn snd_info_set_text_ops(
        entry: *mut snd_info_entry,
        private_data: *mut snd_ac97,
        read: Option<unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer)>,
    );
    fn snd_info_free_entry(entry: *mut snd_info_entry);
    fn ac97_is_audio(ac97: *mut snd_ac97) -> c_int;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
}

struct MutexGuard {
    lock: *mut mutex,
}

impl MutexGuard {
    unsafe fn new(lock: *mut mutex) -> Self {
        unsafe { mutex_lock(lock) };
        Self { lock }
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe { mutex_unlock(self.lock) };
    }
}

const fn str_on_off(x: c_uint) -> *const c_char {
    if x != 0 {
        c"on".as_ptr()
    } else {
        c"off".as_ptr()
    }
}

// External AC97 constants are provided by translated headers.

/*
 * proc interface
 */

unsafe fn snd_ac97_proc_read_functions(ac97: *mut snd_ac97, buffer: *mut snd_info_buffer) {
    let mut header: c_int = 0;
    let mut function: c_int;
    let mut info: c_ushort;
    let mut sense_info: c_ushort;
    static FUNCTION_NAMES: [*const c_char; 12] = [
        c"Master Out".as_ptr(),
        c"AUX Out".as_ptr(),
        c"Center/LFE Out".as_ptr(),
        c"SPDIF Out".as_ptr(),
        c"Phone In".as_ptr(),
        c"Mic 1".as_ptr(),
        c"Mic 2".as_ptr(),
        c"Line In".as_ptr(),
        c"CD In".as_ptr(),
        c"Video In".as_ptr(),
        c"Aux In".as_ptr(),
        c"Mono Out".as_ptr(),
    ];
    static LOCATIONS: [*const c_char; 8] = [
        c"Rear I/O Panel".as_ptr(),
        c"Front Panel".as_ptr(),
        c"Motherboard".as_ptr(),
        c"Dock/External".as_ptr(),
        c"reserved".as_ptr(),
        c"reserved".as_ptr(),
        c"reserved".as_ptr(),
        c"NC/unused".as_ptr(),
    ];

    function = 0;
    while function < 12 {
        unsafe { snd_ac97_write(ac97, AC97_FUNC_SELECT, (function << 1) as c_uint) };
        info = unsafe { snd_ac97_read(ac97, AC97_FUNC_INFO) };
        if (info & 0x0001) == 0 {
            function += 1;
            continue;
        }
        if header == 0 {
            unsafe {
                snd_iprintf(
                    buffer,
                    c"\n                    Gain     Inverted  Buffer delay  Location\n".as_ptr(),
                )
            };
            header = 1;
        }
        sense_info = unsafe { snd_ac97_read(ac97, AC97_SENSE_INFO) };
        unsafe {
            snd_iprintf(
                buffer,
                c"%-17s: %3d.%d dBV    %c      %2d/fs         %s\n".as_ptr(),
                FUNCTION_NAMES[function as usize],
                (if (info & 0x8000) != 0 { -1 } else { 1 })
                    * (((info & 0x7000) >> 12) as c_int)
                    * 3
                    / 2,
                (((info & 0x0800) >> 11) * 5) as c_int,
                if (info & 0x0400) != 0 { b'X' as c_int } else { b'-' as c_int },
                ((info & 0x03e0) >> 5) as c_int,
                LOCATIONS[(sense_info >> 13) as usize],
            )
        };
        function += 1;
    }
}

static SND_AC97_STEREO_ENHANCEMENTS: [*const c_char; 32] = [
    /*   0 */ c"No 3D Stereo Enhancement".as_ptr(),
    /*   1 */ c"Analog Devices Phat Stereo".as_ptr(),
    /*   2 */ c"Creative Stereo Enhancement".as_ptr(),
    /*   3 */ c"National Semi 3D Stereo Enhancement".as_ptr(),
    /*   4 */ c"YAMAHA Ymersion".as_ptr(),
    /*   5 */ c"BBE 3D Stereo Enhancement".as_ptr(),
    /*   6 */ c"Crystal Semi 3D Stereo Enhancement".as_ptr(),
    /*   7 */ c"Qsound QXpander".as_ptr(),
    /*   8 */ c"Spatializer 3D Stereo Enhancement".as_ptr(),
    /*   9 */ c"SRS 3D Stereo Enhancement".as_ptr(),
    /*  10 */ c"Platform Tech 3D Stereo Enhancement".as_ptr(),
    /*  11 */ c"AKM 3D Audio".as_ptr(),
    /*  12 */ c"Aureal Stereo Enhancement".as_ptr(),
    /*  13 */ c"Aztech 3D Enhancement".as_ptr(),
    /*  14 */ c"Binaura 3D Audio Enhancement".as_ptr(),
    /*  15 */ c"ESS Technology Stereo Enhancement".as_ptr(),
    /*  16 */ c"Harman International VMAx".as_ptr(),
    /*  17 */ c"Nvidea/IC Ensemble/KS Waves 3D Stereo Enhancement".as_ptr(),
    /*  18 */ c"Philips Incredible Sound".as_ptr(),
    /*  19 */ c"Texas Instruments 3D Stereo Enhancement".as_ptr(),
    /*  20 */ c"VLSI Technology 3D Stereo Enhancement".as_ptr(),
    /*  21 */ c"TriTech 3D Stereo Enhancement".as_ptr(),
    /*  22 */ c"Realtek 3D Stereo Enhancement".as_ptr(),
    /*  23 */ c"Samsung 3D Stereo Enhancement".as_ptr(),
    /*  24 */ c"Wolfson Microelectronics 3D Enhancement".as_ptr(),
    /*  25 */ c"Delta Integration 3D Enhancement".as_ptr(),
    /*  26 */ c"SigmaTel 3D Enhancement".as_ptr(),
    /*  27 */ c"IC Ensemble/KS Waves".as_ptr(),
    /*  28 */ c"Rockwell 3D Stereo Enhancement".as_ptr(),
    /*  29 */ c"Reserved 29".as_ptr(),
    /*  30 */ c"Reserved 30".as_ptr(),
    /*  31 */ c"Reserved 31".as_ptr(),
];

unsafe fn snd_ac97_proc_read_main(ac97: *mut snd_ac97, buffer: *mut snd_info_buffer, subidx: c_int) {
    let mut name: [c_char; 64] = [0; 64];
    let mut val: c_ushort;
    let mut tmp: c_ushort;
    let ext: c_ushort;
    let mut mext: c_ushort;
    static SPDIF_SLOTS: [*const c_char; 4] = [
        c" SPDIF=3/4".as_ptr(),
        c" SPDIF=7/8".as_ptr(),
        c" SPDIF=6/9".as_ptr(),
        c" SPDIF=10/11".as_ptr(),
    ];
    static SPDIF_RATES: [*const c_char; 4] = [
        c" Rate=44.1kHz".as_ptr(),
        c" Rate=res".as_ptr(),
        c" Rate=48kHz".as_ptr(),
        c" Rate=32kHz".as_ptr(),
    ];
    static SPDIF_RATES_CS4205: [*const c_char; 4] = [
        c" Rate=48kHz".as_ptr(),
        c" Rate=44.1kHz".as_ptr(),
        c" Rate=res".as_ptr(),
        c" Rate=res".as_ptr(),
    ];
    static DOUBLE_RATE_SLOTS: [*const c_char; 4] = [
        c"10/11".as_ptr(),
        c"7/8".as_ptr(),
        c"reserved".as_ptr(),
        c"reserved".as_ptr(),
    ];

    unsafe { snd_ac97_get_name(core::ptr::null_mut(), (*ac97).id, name.as_mut_ptr(), name.len(), 0) };
    unsafe {
        snd_iprintf(
            buffer,
            c"%d-%d/%d: %s\n\n".as_ptr(),
            (*ac97).addr,
            (*ac97).num,
            subidx,
            name.as_ptr(),
        )
    };

    if ((*ac97).scaps & AC97_SCAP_AUDIO) == 0 {
        mext = unsafe { snd_ac97_read(ac97, AC97_EXTENDED_MID) };
    } else {
        unsafe {
            snd_iprintf(buffer, c"PCI Subsys Vendor: 0x%04x\n".as_ptr(), (*ac97).subsystem_vendor);
            snd_iprintf(buffer, c"PCI Subsys Device: 0x%04x\n\n".as_ptr(), (*ac97).subsystem_device);
            snd_iprintf(buffer, c"Flags: %x\n".as_ptr(), (*ac97).flags);
        }

        if ((*ac97).ext_id & AC97_EI_REV_MASK) >= AC97_EI_REV_23 {
            val = unsafe { snd_ac97_read(ac97, AC97_INT_PAGING) };
            unsafe { snd_ac97_update_bits(ac97, AC97_INT_PAGING, AC97_PAGE_MASK, AC97_PAGE_1) };
            tmp = unsafe { snd_ac97_read(ac97, AC97_CODEC_CLASS_REV) };
            unsafe {
                snd_iprintf(buffer, c"Revision         : 0x%02x\n".as_ptr(), tmp & 0xff);
                snd_iprintf(buffer, c"Compat. Class    : 0x%02x\n".as_ptr(), (tmp >> 8) & 0x1f);
                snd_iprintf(buffer, c"Subsys. Vendor ID: 0x%04x\n".as_ptr(), snd_ac97_read(ac97, AC97_PCI_SVID));
                snd_iprintf(buffer, c"Subsys. ID       : 0x%04x\n\n".as_ptr(), snd_ac97_read(ac97, AC97_PCI_SID));
                snd_ac97_update_bits(ac97, AC97_INT_PAGING, AC97_PAGE_MASK, (val as c_uint) & AC97_PAGE_MASK);
            }
        }

        // val = snd_ac97_read(ac97, AC97_RESET);
        val = (*ac97).caps as c_ushort;
        unsafe {
            snd_iprintf(
                buffer,
                c"Capabilities     :%s%s%s%s%s%s\n".as_ptr(),
                if (val as c_uint & AC97_BC_DEDICATED_MIC) != 0 { c" -dedicated MIC PCM IN channel-".as_ptr() } else { c"".as_ptr() },
                if (val as c_uint & AC97_BC_RESERVED1) != 0 { c" -reserved1-".as_ptr() } else { c"".as_ptr() },
                if (val as c_uint & AC97_BC_BASS_TREBLE) != 0 { c" -bass & treble-".as_ptr() } else { c"".as_ptr() },
                if (val as c_uint & AC97_BC_SIM_STEREO) != 0 { c" -simulated stereo-".as_ptr() } else { c"".as_ptr() },
                if (val as c_uint & AC97_BC_HEADPHONE) != 0 { c" -headphone out-".as_ptr() } else { c"".as_ptr() },
                if (val as c_uint & AC97_BC_LOUDNESS) != 0 { c" -loudness-".as_ptr() } else { c"".as_ptr() },
            );
        }
        tmp = ((*ac97).caps & AC97_BC_DAC_MASK) as c_ushort;
        unsafe {
            snd_iprintf(
                buffer,
                c"DAC resolution   : %s%s%s%s\n".as_ptr(),
                if tmp as c_uint == AC97_BC_16BIT_DAC { c"16-bit".as_ptr() } else { c"".as_ptr() },
                if tmp as c_uint == AC97_BC_18BIT_DAC { c"18-bit".as_ptr() } else { c"".as_ptr() },
                if tmp as c_uint == AC97_BC_20BIT_DAC { c"20-bit".as_ptr() } else { c"".as_ptr() },
                if tmp as c_uint == AC97_BC_DAC_MASK { c"???".as_ptr() } else { c"".as_ptr() },
            );
        }
        tmp = ((*ac97).caps & AC97_BC_ADC_MASK) as c_ushort;
        unsafe {
            snd_iprintf(
                buffer,
                c"ADC resolution   : %s%s%s%s\n".as_ptr(),
                if tmp as c_uint == AC97_BC_16BIT_ADC { c"16-bit".as_ptr() } else { c"".as_ptr() },
                if tmp as c_uint == AC97_BC_18BIT_ADC { c"18-bit".as_ptr() } else { c"".as_ptr() },
                if tmp as c_uint == AC97_BC_20BIT_ADC { c"20-bit".as_ptr() } else { c"".as_ptr() },
                if tmp as c_uint == AC97_BC_ADC_MASK { c"???".as_ptr() } else { c"".as_ptr() },
            );
            snd_iprintf(
                buffer,
                c"3D enhancement   : %s\n".as_ptr(),
                SND_AC97_STEREO_ENHANCEMENTS[((val >> 10) & 0x1f) as usize],
            );
            snd_iprintf(buffer, c"\nCurrent setup\n".as_ptr());
        }
        val = unsafe { snd_ac97_read(ac97, AC97_MIC) };
        unsafe {
            snd_iprintf(
                buffer,
                c"Mic gain         : %s [%s]\n".as_ptr(),
                if (val & 0x0040) != 0 { c"+20dB".as_ptr() } else { c"+0dB".as_ptr() },
                if ((*ac97).regs[AC97_MIC as usize] & 0x0040) != 0 { c"+20dB".as_ptr() } else { c"+0dB".as_ptr() },
            );
        }
        val = unsafe { snd_ac97_read(ac97, AC97_GENERAL_PURPOSE) };
        unsafe {
            snd_iprintf(
                buffer,
                c"POP path         : %s 3D\nSim. stereo      : %s\n3D enhancement   : %s\nLoudness         : %s\nMono output      : %s\nMic select       : %s\nADC/DAC loopback : %s\n".as_ptr(),
                if (val & 0x8000) != 0 { c"post".as_ptr() } else { c"pre".as_ptr() },
                str_on_off((val & 0x4000) as c_uint),
                str_on_off((val & 0x2000) as c_uint),
                str_on_off((val & 0x1000) as c_uint),
                if (val & 0x0200) != 0 { c"Mic".as_ptr() } else { c"MIX".as_ptr() },
                if (val & 0x0100) != 0 { c"Mic2".as_ptr() } else { c"Mic1".as_ptr() },
                str_on_off((val & 0x0080) as c_uint),
            );
        }
        if ((*ac97).ext_id & AC97_EI_DRA) != 0 {
            unsafe {
                snd_iprintf(
                    buffer,
                    c"Double rate slots: %s\n".as_ptr(),
                    DOUBLE_RATE_SLOTS[((val >> 10) & 3) as usize],
                )
            };
        }

        ext = unsafe { snd_ac97_read(ac97, AC97_EXTENDED_ID) };
        if ext != 0 {
            unsafe {
                snd_iprintf(
                    buffer,
                    c"Extended ID      : codec=%i rev=%i%s%s%s%s DSA=%i%s%s%s%s\n".as_ptr(),
                    ((ext as c_uint & AC97_EI_ADDR_MASK) >> AC97_EI_ADDR_SHIFT) as c_int,
                    ((ext as c_uint & AC97_EI_REV_MASK) >> AC97_EI_REV_SHIFT) as c_int,
                    if (ext as c_uint & AC97_EI_AMAP) != 0 { c" AMAP".as_ptr() } else { c"".as_ptr() },
                    if (ext as c_uint & AC97_EI_LDAC) != 0 { c" LDAC".as_ptr() } else { c"".as_ptr() },
                    if (ext as c_uint & AC97_EI_SDAC) != 0 { c" SDAC".as_ptr() } else { c"".as_ptr() },
                    if (ext as c_uint & AC97_EI_CDAC) != 0 { c" CDAC".as_ptr() } else { c"".as_ptr() },
                    ((ext as c_uint & AC97_EI_DACS_SLOT_MASK) >> AC97_EI_DACS_SLOT_SHIFT) as c_int,
                    if (ext as c_uint & AC97_EI_VRM) != 0 { c" VRM".as_ptr() } else { c"".as_ptr() },
                    if (ext as c_uint & AC97_EI_SPDIF) != 0 { c" SPDIF".as_ptr() } else { c"".as_ptr() },
                    if (ext as c_uint & AC97_EI_DRA) != 0 { c" DRA".as_ptr() } else { c"".as_ptr() },
                    if (ext as c_uint & AC97_EI_VRA) != 0 { c" VRA".as_ptr() } else { c"".as_ptr() },
                );
            }
            val = unsafe { snd_ac97_read(ac97, AC97_EXTENDED_STATUS) };
            unsafe {
                snd_iprintf(
                    buffer,
                    c"Extended status  :%s%s%s%s%s%s%s%s%s%s%s%s%s%s\n".as_ptr(),
                    if (val as c_uint & AC97_EA_PRL) != 0 { c" PRL".as_ptr() } else { c"".as_ptr() },
                    if (val as c_uint & AC97_EA_PRK) != 0 { c" PRK".as_ptr() } else { c"".as_ptr() },
                    if (val as c_uint & AC97_EA_PRJ) != 0 { c" PRJ".as_ptr() } else { c"".as_ptr() },
                    if (val as c_uint & AC97_EA_PRI) != 0 { c" PRI".as_ptr() } else { c"".as_ptr() },
                    if (val as c_uint & AC97_EA_SPCV) != 0 { c" SPCV".as_ptr() } else { c"".as_ptr() },
                    if (val as c_uint & AC97_EA_MDAC) != 0 { c" MADC".as_ptr() } else { c"".as_ptr() },
                    if (val as c_uint & AC97_EA_LDAC) != 0 { c" LDAC".as_ptr() } else { c"".as_ptr() },
                    if (val as c_uint & AC97_EA_SDAC) != 0 { c" SDAC".as_ptr() } else { c"".as_ptr() },
                    if (val as c_uint & AC97_EA_CDAC) != 0 { c" CDAC".as_ptr() } else { c"".as_ptr() },
                    if (ext as c_uint & AC97_EI_SPDIF) != 0 { SPDIF_SLOTS[((val as c_uint & AC97_EA_SPSA_SLOT_MASK) >> AC97_EA_SPSA_SLOT_SHIFT) as usize] } else { c"".as_ptr() },
                    if (val as c_uint & AC97_EA_VRM) != 0 { c" VRM".as_ptr() } else { c"".as_ptr() },
                    if (val as c_uint & AC97_EA_SPDIF) != 0 { c" SPDIF".as_ptr() } else { c"".as_ptr() },
                    if (val as c_uint & AC97_EA_DRA) != 0 { c" DRA".as_ptr() } else { c"".as_ptr() },
                    if (val as c_uint & AC97_EA_VRA) != 0 { c" VRA".as_ptr() } else { c"".as_ptr() },
                );
            }
            if (ext as c_uint & AC97_EI_VRA) != 0 {
                /* VRA */
                val = unsafe { snd_ac97_read(ac97, AC97_PCM_FRONT_DAC_RATE) };
                unsafe { snd_iprintf(buffer, c"PCM front DAC    : %iHz\n".as_ptr(), val as c_int) };
                if (ext as c_uint & AC97_EI_SDAC) != 0 {
                    val = unsafe { snd_ac97_read(ac97, AC97_PCM_SURR_DAC_RATE) };
                    unsafe { snd_iprintf(buffer, c"PCM Surr DAC     : %iHz\n".as_ptr(), val as c_int) };
                }
                if (ext as c_uint & AC97_EI_LDAC) != 0 {
                    val = unsafe { snd_ac97_read(ac97, AC97_PCM_LFE_DAC_RATE) };
                    unsafe { snd_iprintf(buffer, c"PCM LFE DAC      : %iHz\n".as_ptr(), val as c_int) };
                }
                val = unsafe { snd_ac97_read(ac97, AC97_PCM_LR_ADC_RATE) };
                unsafe { snd_iprintf(buffer, c"PCM ADC          : %iHz\n".as_ptr(), val as c_int) };
            }
            if (ext as c_uint & AC97_EI_VRM) != 0 {
                val = unsafe { snd_ac97_read(ac97, AC97_PCM_MIC_ADC_RATE) };
                unsafe { snd_iprintf(buffer, c"PCM MIC ADC      : %iHz\n".as_ptr(), val as c_int) };
            }
            if (ext as c_uint & AC97_EI_SPDIF) != 0 || ((*ac97).flags & AC97_CS_SPDIF) != 0 || (*ac97).id == AC97_ID_YMF743 {
                if ((*ac97).flags & AC97_CS_SPDIF) != 0 {
                    val = unsafe { snd_ac97_read(ac97, AC97_CSR_SPDIF) };
                } else if (*ac97).id == AC97_ID_YMF743 {
                    val = unsafe { snd_ac97_read(ac97, AC97_YMF7X3_DIT_CTRL) };
                    val = (0x2000 | ((val & 0xff00) >> 4) | ((val & 0x38) >> 2)) as c_ushort;
                } else {
                    val = unsafe { snd_ac97_read(ac97, AC97_SPDIF) };
                }

                unsafe {
                    snd_iprintf(
                        buffer,
                        c"SPDIF Control    :%s%s%s%s Category=0x%x Generation=%i%s%s%s\n".as_ptr(),
                        if (val as c_uint & AC97_SC_PRO) != 0 { c" PRO".as_ptr() } else { c" Consumer".as_ptr() },
                        if (val as c_uint & AC97_SC_NAUDIO) != 0 { c" Non-audio".as_ptr() } else { c" PCM".as_ptr() },
                        if (val as c_uint & AC97_SC_COPY) != 0 { c"".as_ptr() } else { c" Copyright".as_ptr() },
                        if (val as c_uint & AC97_SC_PRE) != 0 { c" Preemph50/15".as_ptr() } else { c"".as_ptr() },
                        ((val as c_uint & AC97_SC_CC_MASK) >> AC97_SC_CC_SHIFT) as c_int,
                        ((val as c_uint & AC97_SC_L) >> 11) as c_int,
                        if ((*ac97).flags & AC97_CS_SPDIF) != 0 {
                            SPDIF_RATES_CS4205[((val as c_uint & AC97_SC_SPSR_MASK) >> AC97_SC_SPSR_SHIFT) as usize]
                        } else {
                            SPDIF_RATES[((val as c_uint & AC97_SC_SPSR_MASK) >> AC97_SC_SPSR_SHIFT) as usize]
                        },
                        if ((*ac97).flags & AC97_CS_SPDIF) != 0 {
                            if (val as c_uint & AC97_SC_DRS) != 0 { c" Validity".as_ptr() } else { c"".as_ptr() }
                        } else if (val as c_uint & AC97_SC_DRS) != 0 { c" DRS".as_ptr() } else { c"".as_ptr() },
                        if ((*ac97).flags & AC97_CS_SPDIF) != 0 {
                            if (val as c_uint & AC97_SC_V) != 0 { c" Enabled".as_ptr() } else { c"".as_ptr() }
                        } else if (val as c_uint & AC97_SC_V) != 0 { c" Validity".as_ptr() } else { c"".as_ptr() },
                    );
                }
                /* ALC650 specific*/
                if ((*ac97).id & 0xfffffff0) == 0x414c4720
                    && (unsafe { snd_ac97_read(ac97, AC97_ALC650_CLOCK) } & 0x01) != 0
                {
                    val = unsafe { snd_ac97_read(ac97, AC97_ALC650_SPDIF_INPUT_STATUS2) };
                    if (val as c_uint & AC97_ALC650_CLOCK_LOCK) != 0 {
                        val = unsafe { snd_ac97_read(ac97, AC97_ALC650_SPDIF_INPUT_STATUS1) };
                        unsafe {
                            snd_iprintf(
                                buffer,
                                c"SPDIF In Status  :%s%s%s%s Category=0x%x Generation=%i".as_ptr(),
                                if (val as c_uint & AC97_ALC650_PRO) != 0 { c" PRO".as_ptr() } else { c" Consumer".as_ptr() },
                                if (val as c_uint & AC97_ALC650_NAUDIO) != 0 { c" Non-audio".as_ptr() } else { c" PCM".as_ptr() },
                                if (val as c_uint & AC97_ALC650_COPY) != 0 { c"".as_ptr() } else { c" Copyright".as_ptr() },
                                if (val as c_uint & AC97_ALC650_PRE) != 0 { c" Preemph50/15".as_ptr() } else { c"".as_ptr() },
                                ((val as c_uint & AC97_ALC650_CC_MASK) >> AC97_ALC650_CC_SHIFT) as c_int,
                                ((val as c_uint & AC97_ALC650_L) >> 15) as c_int,
                            );
                        }
                        val = unsafe { snd_ac97_read(ac97, AC97_ALC650_SPDIF_INPUT_STATUS2) };
                        unsafe {
                            snd_iprintf(
                                buffer,
                                c"%s Accuracy=%i%s%s\n".as_ptr(),
                                SPDIF_RATES[((val as c_uint & AC97_ALC650_SPSR_MASK) >> AC97_ALC650_SPSR_SHIFT) as usize],
                                ((val as c_uint & AC97_ALC650_CLOCK_ACCURACY) >> AC97_ALC650_CLOCK_SHIFT) as c_int,
                                if (val as c_uint & AC97_ALC650_CLOCK_LOCK) != 0 { c" Locked".as_ptr() } else { c" Unlocked".as_ptr() },
                                if (val as c_uint & AC97_ALC650_V) != 0 { c" Validity?".as_ptr() } else { c"".as_ptr() },
                            );
                        }
                    } else {
                        unsafe { snd_iprintf(buffer, c"SPDIF In Status  : Not Locked\n".as_ptr()) };
                    }
                }
            }
            if ((*ac97).ext_id & AC97_EI_REV_MASK) >= AC97_EI_REV_23 {
                val = unsafe { snd_ac97_read(ac97, AC97_INT_PAGING) };
                unsafe { snd_ac97_update_bits(ac97, AC97_INT_PAGING, AC97_PAGE_MASK, AC97_PAGE_1) };
                unsafe { snd_ac97_proc_read_functions(ac97, buffer) };
                unsafe { snd_ac97_update_bits(ac97, AC97_INT_PAGING, AC97_PAGE_MASK, (val as c_uint) & AC97_PAGE_MASK) };
            }
        }
        mext = unsafe { snd_ac97_read(ac97, AC97_EXTENDED_MID) };
    }

    if mext == 0 {
        return;
    }

    unsafe {
        snd_iprintf(
            buffer,
            c"Extended modem ID: codec=%i%s%s%s%s%s\n".as_ptr(),
            ((mext as c_uint & AC97_MEI_ADDR_MASK) >> AC97_MEI_ADDR_SHIFT) as c_int,
            if (mext as c_uint & AC97_MEI_CID2) != 0 { c" CID2".as_ptr() } else { c"".as_ptr() },
            if (mext as c_uint & AC97_MEI_CID1) != 0 { c" CID1".as_ptr() } else { c"".as_ptr() },
            if (mext as c_uint & AC97_MEI_HANDSET) != 0 { c" HSET".as_ptr() } else { c"".as_ptr() },
            if (mext as c_uint & AC97_MEI_LINE2) != 0 { c" LIN2".as_ptr() } else { c"".as_ptr() },
            if (mext as c_uint & AC97_MEI_LINE1) != 0 { c" LIN1".as_ptr() } else { c"".as_ptr() },
        );
    }
    val = unsafe { snd_ac97_read(ac97, AC97_EXTENDED_MSTATUS) };
    unsafe {
        snd_iprintf(
            buffer,
            c"Modem status     :%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s\n".as_ptr(),
            if (val as c_uint & AC97_MEA_GPIO) != 0 { c" GPIO".as_ptr() } else { c"".as_ptr() },
            if (val as c_uint & AC97_MEA_MREF) != 0 { c" MREF".as_ptr() } else { c"".as_ptr() },
            if (val as c_uint & AC97_MEA_ADC1) != 0 { c" ADC1".as_ptr() } else { c"".as_ptr() },
            if (val as c_uint & AC97_MEA_DAC1) != 0 { c" DAC1".as_ptr() } else { c"".as_ptr() },
            if (val as c_uint & AC97_MEA_ADC2) != 0 { c" ADC2".as_ptr() } else { c"".as_ptr() },
            if (val as c_uint & AC97_MEA_DAC2) != 0 { c" DAC2".as_ptr() } else { c"".as_ptr() },
            if (val as c_uint & AC97_MEA_HADC) != 0 { c" HADC".as_ptr() } else { c"".as_ptr() },
            if (val as c_uint & AC97_MEA_HDAC) != 0 { c" HDAC".as_ptr() } else { c"".as_ptr() },
            if (val as c_uint & AC97_MEA_PRA) != 0 { c" PRA(GPIO)".as_ptr() } else { c"".as_ptr() },
            if (val as c_uint & AC97_MEA_PRB) != 0 { c" PRB(res)".as_ptr() } else { c"".as_ptr() },
            if (val as c_uint & AC97_MEA_PRC) != 0 { c" PRC(ADC1)".as_ptr() } else { c"".as_ptr() },
            if (val as c_uint & AC97_MEA_PRD) != 0 { c" PRD(DAC1)".as_ptr() } else { c"".as_ptr() },
            if (val as c_uint & AC97_MEA_PRE) != 0 { c" PRE(ADC2)".as_ptr() } else { c"".as_ptr() },
            if (val as c_uint & AC97_MEA_PRF) != 0 { c" PRF(DAC2)".as_ptr() } else { c"".as_ptr() },
            if (val as c_uint & AC97_MEA_PRG) != 0 { c" PRG(HADC)".as_ptr() } else { c"".as_ptr() },
            if (val as c_uint & AC97_MEA_PRH) != 0 { c" PRH(HDAC)".as_ptr() } else { c"".as_ptr() },
        );
    }
    if (mext as c_uint & AC97_MEI_LINE1) != 0 {
        val = unsafe { snd_ac97_read(ac97, AC97_LINE1_RATE) };
        unsafe { snd_iprintf(buffer, c"Line1 rate       : %iHz\n".as_ptr(), val as c_int) };
    }
    if (mext as c_uint & AC97_MEI_LINE2) != 0 {
        val = unsafe { snd_ac97_read(ac97, AC97_LINE2_RATE) };
        unsafe { snd_iprintf(buffer, c"Line2 rate       : %iHz\n".as_ptr(), val as c_int) };
    }
    if (mext as c_uint & AC97_MEI_HANDSET) != 0 {
        val = unsafe { snd_ac97_read(ac97, AC97_HANDSET_RATE) };
        unsafe { snd_iprintf(buffer, c"Headset rate     : %iHz\n".as_ptr(), val as c_int) };
    }
}

unsafe extern "C" fn snd_ac97_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let ac97: *mut snd_ac97 = unsafe { (*entry).private_data as *mut snd_ac97 };

    let _guard = unsafe { MutexGuard::new(&mut (*ac97).page_mutex) };
    if ((*ac97).id & 0xffffff40) == AC97_ID_AD1881 {
        // Analog Devices AD1881/85/86
        let mut idx: c_int = 0;
        while idx < 3 {
            if unsafe { (*ac97).spec.ad18xx.id[idx as usize] } != 0 {
                /* select single codec */
                unsafe {
                    snd_ac97_update_bits(
                        ac97,
                        AC97_AD_SERIAL_CFG,
                        0x7000,
                        (*ac97).spec.ad18xx.unchained[idx as usize]
                            | (*ac97).spec.ad18xx.chained[idx as usize],
                    );
                    snd_ac97_proc_read_main(ac97, buffer, idx);
                    snd_iprintf(buffer, c"\n\n".as_ptr());
                }
            }
            idx += 1;
        }
        /* select all codecs */
        unsafe { snd_ac97_update_bits(ac97, AC97_AD_SERIAL_CFG, 0x7000, 0x7000) };

        unsafe {
            snd_iprintf(buffer, c"\nAD18XX configuration\n".as_ptr());
            snd_iprintf(
                buffer,
                c"Unchained        : 0x%04x,0x%04x,0x%04x\n".as_ptr(),
                (*ac97).spec.ad18xx.unchained[0],
                (*ac97).spec.ad18xx.unchained[1],
                (*ac97).spec.ad18xx.unchained[2],
            );
            snd_iprintf(
                buffer,
                c"Chained          : 0x%04x,0x%04x,0x%04x\n".as_ptr(),
                (*ac97).spec.ad18xx.chained[0],
                (*ac97).spec.ad18xx.chained[1],
                (*ac97).spec.ad18xx.chained[2],
            );
        }
    } else {
        unsafe { snd_ac97_proc_read_main(ac97, buffer, 0) };
    }
}

// CONFIG_SND_DEBUG: direct register write for debugging.
#[cfg(CONFIG_SND_DEBUG)]
unsafe extern "C" fn snd_ac97_proc_regs_write(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let ac97: *mut snd_ac97 = unsafe { (*entry).private_data as *mut snd_ac97 };
    let mut line: [c_char; 64] = [0; 64];
    let mut reg: c_uint = 0;
    let mut val: c_uint = 0;

    let _guard = unsafe { MutexGuard::new(&mut (*ac97).page_mutex) };
    while unsafe { snd_info_get_line(buffer, line.as_mut_ptr(), line.len() as c_int) } == 0 {
        if unsafe { sscanf(line.as_ptr(), c"%x %x".as_ptr(), &mut reg, &mut val) } != 2 {
            continue;
        }
        /* register must be even */
        if reg < 0x80 && (reg & 1) == 0 && val <= 0xffff {
            unsafe { snd_ac97_write_cache(ac97, reg, val) };
        }
    }
}

unsafe fn snd_ac97_proc_regs_read_main(ac97: *mut snd_ac97, buffer: *mut snd_info_buffer, subidx: c_int) {
    let mut reg: c_int = 0;
    let mut val: c_int;

    while reg < 0x80 {
        val = unsafe { snd_ac97_read(ac97, reg as c_uint) } as c_int;
        unsafe { snd_iprintf(buffer, c"%i:%02x = %04x\n".as_ptr(), subidx, reg, val) };
        reg += 2;
    }
}

unsafe extern "C" fn snd_ac97_proc_regs_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let ac97: *mut snd_ac97 = unsafe { (*entry).private_data as *mut snd_ac97 };

    let _guard = unsafe { MutexGuard::new(&mut (*ac97).page_mutex) };
    if ((*ac97).id & 0xffffff40) == AC97_ID_AD1881 {
        // Analog Devices AD1881/85/86

        let mut idx: c_int = 0;
        while idx < 3 {
            if unsafe { (*ac97).spec.ad18xx.id[idx as usize] } != 0 {
                /* select single codec */
                unsafe {
                    snd_ac97_update_bits(
                        ac97,
                        AC97_AD_SERIAL_CFG,
                        0x7000,
                        (*ac97).spec.ad18xx.unchained[idx as usize]
                            | (*ac97).spec.ad18xx.chained[idx as usize],
                    );
                    snd_ac97_proc_regs_read_main(ac97, buffer, idx);
                }
            }
            idx += 1;
        }
        /* select all codecs */
        unsafe { snd_ac97_update_bits(ac97, AC97_AD_SERIAL_CFG, 0x7000, 0x7000) };
    } else {
        unsafe { snd_ac97_proc_regs_read_main(ac97, buffer, 0) };
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_ac97_proc_init(ac97: *mut snd_ac97) {
    let mut entry: *mut snd_info_entry;
    let mut name: [c_char; 32] = [0; 32];
    let prefix: *const c_char;

    if unsafe { (*(*ac97).bus).proc }.is_null() {
        return;
    }
    prefix = if unsafe { ac97_is_audio(ac97) } != 0 { c"ac97".as_ptr() } else { c"mc97".as_ptr() };
    unsafe { sprintf(name.as_mut_ptr(), c"%s#%d-%d".as_ptr(), prefix, (*ac97).addr, (*ac97).num) };
    entry = unsafe { snd_info_create_card_entry((*(*ac97).bus).card, name.as_ptr(), (*(*ac97).bus).proc) };
    if !entry.is_null() {
        unsafe { snd_info_set_text_ops(entry, ac97, Some(snd_ac97_proc_read)) };
    }
    unsafe { (*ac97).proc = entry };
    unsafe { sprintf(name.as_mut_ptr(), c"%s#%d-%d+regs".as_ptr(), prefix, (*ac97).addr, (*ac97).num) };
    entry = unsafe { snd_info_create_card_entry((*(*ac97).bus).card, name.as_ptr(), (*(*ac97).bus).proc) };
    if !entry.is_null() {
        unsafe { snd_info_set_text_ops(entry, ac97, Some(snd_ac97_proc_regs_read)) };
        #[cfg(CONFIG_SND_DEBUG)]
        unsafe {
            (*entry).mode |= 0o200;
            (*entry).c.text.write = Some(snd_ac97_proc_regs_write);
        }
    }
    unsafe { (*ac97).proc_regs = entry };
}

#[no_mangle]
pub unsafe extern "C" fn snd_ac97_proc_done(ac97: *mut snd_ac97) {
    unsafe { snd_info_free_entry((*ac97).proc_regs) };
    unsafe { (*ac97).proc_regs = core::ptr::null_mut() };
    unsafe { snd_info_free_entry((*ac97).proc) };
    unsafe { (*ac97).proc = core::ptr::null_mut() };
}

#[no_mangle]
pub unsafe extern "C" fn snd_ac97_bus_proc_init(bus: *mut snd_ac97_bus) {
    let mut entry: *mut snd_info_entry;
    let mut name: [c_char; 32] = [0; 32];

    unsafe { sprintf(name.as_mut_ptr(), c"codec97#%d".as_ptr(), (*bus).num) };
    entry = unsafe { snd_info_create_card_entry((*bus).card, name.as_ptr(), (*(*bus).card).proc_root) };
    if !entry.is_null() {
        unsafe { (*entry).mode = S_IFDIR | 0o555 };
    }
    unsafe { (*bus).proc = entry };
}

#[no_mangle]
pub unsafe extern "C" fn snd_ac97_bus_proc_done(bus: *mut snd_ac97_bus) {
    unsafe { snd_info_free_entry((*bus).proc) };
    unsafe { (*bus).proc = core::ptr::null_mut() };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
