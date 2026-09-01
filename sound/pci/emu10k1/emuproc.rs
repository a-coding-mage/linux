// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *                   Lee Revell <rlrevell@joe-job.com>
 *                   James Courtier-Dutton <James@superbug.co.uk>
 *                   Oswald Buddenhagen <oswald.buddenhagen@gmx.de>
 *                   Creative Labs, Inc.
 *
 *  Routines for control of EMU10K1 chips / proc interface routines
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

type u32 = c_uint;
type s8 = i8;
type ssize_t = isize;
type size_t = usize;
type loff_t = i64;

#[repr(C)]
pub struct snd_emu10k1 {
    pub audigy: c_int,
    pub card_capabilities: *mut snd_emu10k1_card_capabilities,
    pub fx8010: snd_emu10k1_fx8010,
    pub efx_voices_mask: [c_uint; 2],
    pub voices: *mut snd_emu10k1_voice,
    pub card: *mut snd_card,
    pub port: c_ulong,
    pub emu_lock: spinlock_t,
}

#[repr(C)]
pub struct snd_emu10k1_card_capabilities {
    pub emu_model: c_int,
    pub ecard: c_int,
    pub sblive51: c_int,
    pub emu10k2_chip: c_int,
    pub ca0151_chip: c_int,
    pub ca0108_chip: c_int,
}

#[repr(C)]
pub struct snd_emu10k1_fx8010 {
    pub extin_mask: u16,
    pub extout_mask: u16,
    pub itram_size: c_uint,
    pub etram_pages: snd_dma_buffer,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_dma_buffer {
    pub bytes: usize,
}

#[repr(C)]
pub struct snd_emu10k1_voice {
    pub dirty: c_uint,
    pub last: c_uint,
    pub use_: usize,
}

#[repr(C)]
pub struct snd_info_entry {
    pub name: *const c_char,
    pub private_data: *mut snd_emu10k1,
    pub content: c_int,
    pub mode: c_uint,
    pub size: c_ulong,
    pub c: snd_info_entry_c,
}

#[repr(C)]
pub struct snd_info_entry_c {
    pub ops: *const snd_info_entry_ops,
}

#[repr(C)]
pub struct snd_info_entry_ops {
    pub read: Option<
        unsafe extern "C" fn(
            *mut snd_info_entry,
            *mut c_void,
            *mut file,
            *mut c_char,
            size_t,
            loff_t,
        ) -> ssize_t,
    >,
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    static snd_emu10k1_audigy_ins: *const *const c_char;
    static snd_emu10k1_sblive_ins: *const *const c_char;
    static snd_emu10k1_audigy_outs: *const *const c_char;
    static snd_emu10k1_sblive_outs: *const *const c_char;
    static snd_emu10k1_fxbus: *const *const c_char;
    static snd_emu10k1_sblive51_fxbus2_map: *const s8;

    fn snd_emu10k1_ptr_read(emu: *mut snd_emu10k1, reg: c_uint, chn: c_uint) -> c_uint;
    fn snd_emu10k1_ptr20_read(emu: *mut snd_emu10k1, reg: c_uint, chn: c_uint) -> c_uint;
    fn snd_emu10k1_efx_read(emu: *mut snd_emu10k1, pc: u32) -> u32;
    fn snd_emu1010_fpga_read(emu: *mut snd_emu10k1, reg: c_uint, value: *mut u32) -> c_int;
    fn snd_emu1010_get_raw_rate(emu: *mut snd_emu10k1, source: c_uint) -> c_int;
    fn snd_emu1010_fpga_link_dst_src_read(emu: *mut snd_emu10k1, dst: u32) -> u32;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn kmalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn copy_to_user(to: *mut c_char, from: *const c_void, n: size_t) -> c_ulong;
    fn snd_info_get_line(buffer: *mut snd_info_buffer, line: *mut c_char, len: c_int) -> c_int;
    fn snd_card_ro_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        data: *mut snd_emu10k1,
        read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer),
    );
    fn snd_card_rw_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        data: *mut snd_emu10k1,
        read: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer),
        write: unsafe extern "C" fn(*mut snd_info_entry, *mut snd_info_buffer),
    );
    fn snd_card_proc_new(
        card: *mut snd_card,
        name: *const c_char,
        entryp: *mut *mut snd_info_entry,
    ) -> c_int;
    fn inl(port: c_ulong) -> c_ulong;
    fn outl(value: c_uint, port: c_ulong);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
}

const SPCS_PROFESSIONAL: c_uint = 0x00000001;
const SPCS_NOTAUDIODATA: c_uint = 0x00000002;
const SPCS_COPYRIGHT: c_uint = 0x00000004;
const SPCS_EMPHASISMASK: c_uint = 0x00000038;
const SPCS_MODEMASK: c_uint = 0x000000c0;
const SPCS_CATEGORYCODEMASK: c_uint = 0x0000ff00;
const SPCS_GENERATIONSTATUS: c_uint = 0x00008000;
const SPCS_SOURCENUMMASK: c_uint = 0x000f0000;
const SPCS_CHANNELNUMMASK: c_uint = 0x00f00000;
const SPCS_SAMPLERATEMASK: c_uint = 0x0f000000;
const SPCS_CLKACCYMASK: c_uint = 0xf0000000;
const SRCS_SPDIFVALID: c_uint = 0x00000100;
const SRCS_SPDIFLOCKED: c_uint = 0x00000200;
const SRCS_RATELOCKED: c_uint = 0x00000080;
const NUM_G: c_int = 64;
const EMU10K1_NUM_TYPES: usize = 6;
const EMU_MODEL_EMU0404: c_int = 1;
const EMU_MODEL_EMU1010: c_int = 2;
const EMU_MODEL_EMU1010B: c_int = 3;
const EMU_MODEL_EMU1616: c_int = 4;
const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EFAULT: c_int = 14;
const SNDRV_INFO_CONTENT_DATA: c_int = 2;
const S_IFREG: c_uint = 0o100000;
const PTR: c_ulong = 0;
const DATA: c_ulong = 4;

const CDCS: c_uint = 0;
const CDSRCS: c_int = 0;
const GPSCS: c_uint = 0;
const GPSRCS: c_int = 0;
const CAPTURE_RATE_STATUS: c_uint = 0;
const PTRX: c_uint = 0;
const PSST: c_uint = 0;
const DSL: c_uint = 0;
const A_FXRT1: c_uint = 0;
const A_FXRT2: c_uint = 0;
const A_SENDAMOUNTS: c_uint = 0;
const FXRT: c_uint = 0;
const PTRX_FXSENDAMOUNT_A: c_uint = 0;
const PTRX_FXSENDAMOUNT_B: c_uint = 0;
const PSST_FXSENDAMOUNT_C: c_uint = 0;
const DSL_FXSENDAMOUNT_D: c_uint = 0;
const EMU_HANA_IRQ_STATUS: c_uint = 0;
const EMU_HANA_LOCK_STS_LO: c_uint = 0;
const EMU_HANA_LOCK_STS_HI: c_uint = 0;
const EMU_HANA_WCLOCK_HANA_SPDIF_IN: c_uint = 0;
const EMU_HANA_WCLOCK_HANA_ADAT_IN: c_uint = 0;
const EMU_HANA_WCLOCK_2ND_HANA: c_uint = 0;
const EMU_HANA_WCLOCK_SYNC_BNC: c_uint = 0;
const EMU_HANA_SPDIF_MODE: c_uint = 0;
const EMU_HANA_SPDIF_MODE_RX_INVALID: c_uint = 0;
const EMU_HANA_SPDIF_MODE_RX_PRO: c_uint = 0;
const EMU_HANA_SPDIF_MODE_RX_NOCOPY: c_uint = 0;
const TANKMEMADDRREGBASE: c_uint = 0;
const TANKMEMDATAREGBASE: c_uint = 0;
const A_MICROCODEBASE: c_uint = 0;
const MICROCODEBASE: c_uint = 0;
const A_FXGPREGBASE: c_uint = 0;
const FXGPREGBASE: c_uint = 0;

unsafe fn str_yes_no(v: c_uint) -> *const c_char {
    if v != 0 { c"yes".as_ptr() } else { c"no".as_ptr() }
}

unsafe fn str_on_off(v: c_uint) -> *const c_char {
    if v != 0 { c"on".as_ptr() } else { c"off".as_ptr() }
}

fn reg_val_get(_field: c_uint, val: c_uint) -> c_uint {
    val
}

fn clamp(v: c_int, lo: c_int, hi: c_int) -> c_int {
    if v < lo {
        lo
    } else if v > hi {
        hi
    } else {
        v
    }
}

unsafe extern "C" fn snd_emu10k1_proc_spdif_status(
    emu: *mut snd_emu10k1,
    buffer: *mut snd_info_buffer,
    title: *mut c_char,
    status_reg: c_int,
    rate_reg: c_int,
) {
    static CLKACCY: [*const c_char; 4] = [
        c"1000ppm".as_ptr(),
        c"50ppm".as_ptr(),
        c"variable".as_ptr(),
        c"unknown".as_ptr(),
    ];
    static SAMPLERATE: [c_int; 16] = [44100, 1, 48000, 32000, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    static CHANNEL: [*const c_char; 16] = [
        c"unspec".as_ptr(), c"left".as_ptr(), c"right".as_ptr(), c"3".as_ptr(),
        c"4".as_ptr(), c"5".as_ptr(), c"6".as_ptr(), c"7".as_ptr(),
        c"8".as_ptr(), c"9".as_ptr(), c"10".as_ptr(), c"11".as_ptr(),
        c"12".as_ptr(), c"13".as_ptr(), c"14".as_ptr(), c"15".as_ptr(),
    ];
    static EMPHASIS: [*const c_char; 8] = [
        c"none".as_ptr(), c"50/15 usec 2 channel".as_ptr(), c"2".as_ptr(), c"3".as_ptr(),
        c"4".as_ptr(), c"5".as_ptr(), c"6".as_ptr(), c"7".as_ptr(),
    ];
    let status = snd_emu10k1_ptr_read(emu, status_reg as c_uint, 0);
    let mut rate: c_uint;

    snd_iprintf(buffer, c"\n%s\n".as_ptr(), title);

    if status != 0xffffffff {
        snd_iprintf(buffer, c"Professional Mode     : %s\n".as_ptr(), str_yes_no(status & SPCS_PROFESSIONAL));
        snd_iprintf(buffer, c"Not Audio Data        : %s\n".as_ptr(), str_yes_no(status & SPCS_NOTAUDIODATA));
        snd_iprintf(buffer, c"Copyright             : %s\n".as_ptr(), str_yes_no(status & SPCS_COPYRIGHT));
        snd_iprintf(buffer, c"Emphasis              : %s\n".as_ptr(), EMPHASIS[((status & SPCS_EMPHASISMASK) >> 3) as usize]);
        snd_iprintf(buffer, c"Mode                  : %i\n".as_ptr(), (status & SPCS_MODEMASK) >> 6);
        snd_iprintf(buffer, c"Category Code         : 0x%x\n".as_ptr(), (status & SPCS_CATEGORYCODEMASK) >> 8);
        snd_iprintf(buffer, c"Generation Status     : %s\n".as_ptr(), if status & SPCS_GENERATIONSTATUS != 0 { c"original".as_ptr() } else { c"copy".as_ptr() });
        snd_iprintf(buffer, c"Source Mask           : %i\n".as_ptr(), (status & SPCS_SOURCENUMMASK) >> 16);
        snd_iprintf(buffer, c"Channel Number        : %s\n".as_ptr(), CHANNEL[((status & SPCS_CHANNELNUMMASK) >> 20) as usize]);
        snd_iprintf(buffer, c"Sample Rate           : %iHz\n".as_ptr(), SAMPLERATE[((status & SPCS_SAMPLERATEMASK) >> 24) as usize]);
        snd_iprintf(buffer, c"Clock Accuracy        : %s\n".as_ptr(), CLKACCY[((status & SPCS_CLKACCYMASK) >> 28) as usize]);

        if rate_reg > 0 {
            rate = snd_emu10k1_ptr_read(emu, rate_reg as c_uint, 0);
            snd_iprintf(buffer, c"S/PDIF Valid          : %s\n".as_ptr(), str_on_off(rate & SRCS_SPDIFVALID));
            snd_iprintf(buffer, c"S/PDIF Locked         : %s\n".as_ptr(), str_on_off(rate & SRCS_SPDIFLOCKED));
            snd_iprintf(buffer, c"Rate Locked           : %s\n".as_ptr(), str_on_off(rate & SRCS_RATELOCKED));
            /* From ((Rate * 48000 ) / 262144); */
            snd_iprintf(buffer, c"Estimated Sample Rate : %d\n".as_ptr(), ((rate & 0xFFFFF) * 375) >> 11);
        }
    } else {
        snd_iprintf(buffer, c"No signal detected.\n".as_ptr());
    }
}

unsafe extern "C" fn snd_emu10k1_proc_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let emu = (*entry).private_data;
    let inputs = if (*emu).audigy != 0 { snd_emu10k1_audigy_ins } else { snd_emu10k1_sblive_ins };
    let outputs = if (*emu).audigy != 0 { snd_emu10k1_audigy_outs } else { snd_emu10k1_sblive_outs };
    let extin_mask: u16 = if (*emu).audigy != 0 { !0 } else { (*emu).fx8010.extin_mask };
    let extout_mask: u16 = if (*emu).audigy != 0 { !0 } else { (*emu).fx8010.extout_mask };
    let mut val: c_uint;
    let mut val1: c_uint;
    let mut ptrx: c_uint;
    let mut psst: c_uint;
    let mut dsl: c_uint;
    let mut snda: c_uint;
    let nefx = if (*emu).audigy != 0 { 32 } else { 16 };

    snd_iprintf(buffer, c"EMU10K1\n\n".as_ptr());
    snd_iprintf(
        buffer,
        c"Card                  : %s\n".as_ptr(),
        if (*(*emu).card_capabilities).emu_model != 0 {
            c"E-MU D.A.S.".as_ptr()
        } else if (*(*emu).card_capabilities).ecard != 0 {
            c"E-MU A.P.S.".as_ptr()
        } else if (*emu).audigy != 0 {
            c"SB Audigy".as_ptr()
        } else {
            c"SB Live!".as_ptr()
        },
    );
    snd_iprintf(buffer, c"Internal TRAM (words) : 0x%x\n".as_ptr(), (*emu).fx8010.itram_size);
    snd_iprintf(buffer, c"External TRAM (words) : 0x%x\n".as_ptr(), ((*emu).fx8010.etram_pages.bytes as c_int) / 2);

    snd_iprintf(buffer, c"\nEffect Send Routing & Amounts:\n".as_ptr());
    for idx in 0..NUM_G {
        ptrx = snd_emu10k1_ptr_read(emu, PTRX, idx as c_uint);
        psst = snd_emu10k1_ptr_read(emu, PSST, idx as c_uint);
        dsl = snd_emu10k1_ptr_read(emu, DSL, idx as c_uint);
        if (*emu).audigy != 0 {
            val = snd_emu10k1_ptr_read(emu, A_FXRT1, idx as c_uint);
            val1 = snd_emu10k1_ptr_read(emu, A_FXRT2, idx as c_uint);
            snda = snd_emu10k1_ptr_read(emu, A_SENDAMOUNTS, idx as c_uint);
            snd_iprintf(buffer, c"Ch%-2i: A=%2i:%02x, B=%2i:%02x, C=%2i:%02x, D=%2i:%02x, ".as_ptr(),
                idx, val & 0x3f, reg_val_get(PTRX_FXSENDAMOUNT_A, ptrx),
                (val >> 8) & 0x3f, reg_val_get(PTRX_FXSENDAMOUNT_B, ptrx),
                (val >> 16) & 0x3f, reg_val_get(PSST_FXSENDAMOUNT_C, psst),
                (val >> 24) & 0x3f, reg_val_get(DSL_FXSENDAMOUNT_D, dsl));
            snd_iprintf(buffer, c"E=%2i:%02x, F=%2i:%02x, G=%2i:%02x, H=%2i:%02x\n".as_ptr(),
                val1 & 0x3f, (snda >> 24) & 0xff,
                (val1 >> 8) & 0x3f, (snda >> 16) & 0xff,
                (val1 >> 16) & 0x3f, (snda >> 8) & 0xff,
                (val1 >> 24) & 0x3f, snda & 0xff);
        } else {
            val = snd_emu10k1_ptr_read(emu, FXRT, idx as c_uint);
            snd_iprintf(buffer, c"Ch%-2i: A=%2i:%02x, B=%2i:%02x, C=%2i:%02x, D=%2i:%02x\n".as_ptr(),
                idx, (val >> 16) & 0x0f, reg_val_get(PTRX_FXSENDAMOUNT_A, ptrx),
                (val >> 20) & 0x0f, reg_val_get(PTRX_FXSENDAMOUNT_B, ptrx),
                (val >> 24) & 0x0f, reg_val_get(PSST_FXSENDAMOUNT_C, psst),
                (val >> 28) & 0x0f, reg_val_get(DSL_FXSENDAMOUNT_D, dsl));
        }
    }
    snd_iprintf(buffer, c"\nEffect Send Targets:\n".as_ptr());
    // Audigy actually has 64, but we don't use them all.
    for idx in 0..32 {
        let c = *snd_emu10k1_fxbus.add(idx);
        if !c.is_null() {
            snd_iprintf(buffer, c"  Channel %02i [%s]\n".as_ptr(), idx as c_int, c);
        }
    }
    if (*(*emu).card_capabilities).emu_model == 0 {
        snd_iprintf(buffer, c"\nOutput Channels:\n".as_ptr());
        for idx in 0..32 {
            if !(*outputs.add(idx)).is_null() && ((extout_mask as c_int) & (1 << idx)) != 0 {
                snd_iprintf(buffer, c"  Channel %02i [%s]\n".as_ptr(), idx as c_int, *outputs.add(idx));
            }
        }
        snd_iprintf(buffer, c"\nInput Channels:\n".as_ptr());
        for idx in 0..16 {
            if !(*inputs.add(idx)).is_null() && ((extin_mask as c_int) & (1 << idx)) != 0 {
                snd_iprintf(buffer, c"  Channel %02i [%s]\n".as_ptr(), idx as c_int, *inputs.add(idx));
            }
        }
        snd_iprintf(buffer, c"\nMultichannel Capture Sources:\n".as_ptr());
        for idx in 0..nefx {
            if ((*emu).efx_voices_mask[0] & (1 << idx)) != 0 {
                snd_iprintf(buffer, c"  Channel %02i [Output: %s]\n".as_ptr(), idx, if !(*outputs.add(idx as usize)).is_null() { *outputs.add(idx as usize) } else { c"???".as_ptr() });
            }
        }
        if (*emu).audigy != 0 {
            for idx in 0..32 {
                if ((*emu).efx_voices_mask[1] & (1 << idx)) != 0 {
                    snd_iprintf(buffer, c"  Channel %02i [Input: %s]\n".as_ptr(), idx + 32, if !(*inputs.add(idx as usize)).is_null() { *inputs.add(idx as usize) } else { c"???".as_ptr() });
                }
            }
        } else {
            for idx in 0..16 {
                if ((*emu).efx_voices_mask[0] & ((1 << 16) << idx)) != 0 {
                    if (*(*emu).card_capabilities).sblive51 != 0 {
                        let c = *snd_emu10k1_sblive51_fxbus2_map.add(idx as usize);
                        if c == -1 {
                            snd_iprintf(buffer, c"  Channel %02i [Output: %s]\n".as_ptr(), idx + 16, *outputs.add((idx + 16) as usize));
                        } else {
                            snd_iprintf(buffer, c"  Channel %02i [Input: %s]\n".as_ptr(), idx + 16, *inputs.add(c as usize));
                        }
                    } else {
                        snd_iprintf(buffer, c"  Channel %02i [Input: %s]\n".as_ptr(), idx + 16, if !(*inputs.add(idx as usize)).is_null() { *inputs.add(idx as usize) } else { c"???".as_ptr() });
                    }
                }
            }
        }
    }
}

unsafe extern "C" fn snd_emu10k1_proc_spdif_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let emu = (*entry).private_data;
    let mut value: u32 = 0;
    let mut value2: u32 = 0;

    if (*(*emu).card_capabilities).emu_model != 0 {
        snd_emu1010_fpga_read(emu, EMU_HANA_IRQ_STATUS, &mut value);
        snd_iprintf(buffer, c"Lock status 1: %#x\n".as_ptr(), value & 0x10);

        // Bit 0x1 in LO being 0 is supposedly for ADAT lock.
        // The registers are always all zero on 0404b.
        snd_emu1010_fpga_read(emu, EMU_HANA_LOCK_STS_LO, &mut value);
        snd_emu1010_fpga_read(emu, EMU_HANA_LOCK_STS_HI, &mut value2);
        snd_iprintf(buffer, c"Lock status 2: %#x %#x\n".as_ptr(), value, value2);

        snd_iprintf(buffer, c"S/PDIF rate: %dHz\n".as_ptr(), snd_emu1010_get_raw_rate(emu, EMU_HANA_WCLOCK_HANA_SPDIF_IN));
        if (*(*emu).card_capabilities).emu_model != EMU_MODEL_EMU0404 {
            snd_iprintf(buffer, c"ADAT rate: %dHz\n".as_ptr(), snd_emu1010_get_raw_rate(emu, EMU_HANA_WCLOCK_HANA_ADAT_IN));
            snd_iprintf(buffer, c"Dock rate: %dHz\n".as_ptr(), snd_emu1010_get_raw_rate(emu, EMU_HANA_WCLOCK_2ND_HANA));
        }
        if (*(*emu).card_capabilities).emu_model == EMU_MODEL_EMU0404 ||
            (*(*emu).card_capabilities).emu_model == EMU_MODEL_EMU1010 {
            snd_iprintf(buffer, c"BNC rate: %dHz\n".as_ptr(), snd_emu1010_get_raw_rate(emu, EMU_HANA_WCLOCK_SYNC_BNC));
        }

        snd_emu1010_fpga_read(emu, EMU_HANA_SPDIF_MODE, &mut value);
        if value & EMU_HANA_SPDIF_MODE_RX_INVALID != 0 {
            snd_iprintf(buffer, c"\nS/PDIF input invalid\n".as_ptr());
        } else {
            snd_iprintf(buffer, c"\nS/PDIF mode: %s%s\n".as_ptr(),
                if value & EMU_HANA_SPDIF_MODE_RX_PRO != 0 { c"professional".as_ptr() } else { c"consumer".as_ptr() },
                if value & EMU_HANA_SPDIF_MODE_RX_NOCOPY != 0 { c", no copy".as_ptr() } else { c"".as_ptr() });
        }
    } else {
        snd_emu10k1_proc_spdif_status(emu, buffer, c"CD-ROM S/PDIF In".as_ptr() as *mut c_char, CDCS as c_int, CDSRCS);
        snd_emu10k1_proc_spdif_status(emu, buffer, c"Optical or Coax S/PDIF In".as_ptr() as *mut c_char, GPSCS as c_int, GPSRCS);
    }
    /*
     * Original #if 0 block:
     * val = snd_emu10k1_ptr_read(emu, ZVSRCS, 0);
     * print Zoomed Video rate lock and estimated sample rate.
     */
}

unsafe extern "C" fn snd_emu10k1_proc_rates_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    static SAMPLERATE: [c_int; 8] = [44100, 48000, 96000, 192000, 4, 5, 6, 7];
    let emu = (*entry).private_data;
    let val = snd_emu10k1_ptr20_read(emu, CAPTURE_RATE_STATUS, 0);
    for n in 0..4 {
        let tmp = val >> (16 + (n * 4));
        if tmp & 0x8 != 0 {
            snd_iprintf(buffer, c"Channel %d: Rate=%d\n".as_ptr(), n, SAMPLERATE[(tmp & 0x7) as usize]);
        } else {
            snd_iprintf(buffer, c"Channel %d: No input\n".as_ptr(), n);
        }
    }
}

#[repr(C)]
struct emu10k1_reg_entry {
    base: u16,
    size: u16,
    name: *const c_char,
}

static SBLIVE_REG_ENTRIES: [emu10k1_reg_entry; 10] = [
    emu10k1_reg_entry { base: 0, size: 0x10, name: c"FXBUS".as_ptr() },
    emu10k1_reg_entry { base: 0x10, size: 0x10, name: c"EXTIN".as_ptr() },
    emu10k1_reg_entry { base: 0x20, size: 0x10, name: c"EXTOUT".as_ptr() },
    emu10k1_reg_entry { base: 0x30, size: 0x10, name: c"FXBUS2".as_ptr() },
    emu10k1_reg_entry { base: 0x40, size: 0x20, name: ptr::null() }, // Constants
    emu10k1_reg_entry { base: 0x100, size: 0x100, name: c"GPR".as_ptr() },
    emu10k1_reg_entry { base: 0x200, size: 0x80, name: c"ITRAM_DATA".as_ptr() },
    emu10k1_reg_entry { base: 0x280, size: 0x20, name: c"ETRAM_DATA".as_ptr() },
    emu10k1_reg_entry { base: 0x300, size: 0x80, name: c"ITRAM_ADDR".as_ptr() },
    emu10k1_reg_entry { base: 0x400, size: 0, name: ptr::null() },
];

static AUDIGY_REG_ENTRIES: [emu10k1_reg_entry; 16] = [
    emu10k1_reg_entry { base: 0, size: 0x40, name: c"FXBUS".as_ptr() },
    emu10k1_reg_entry { base: 0x40, size: 0x10, name: c"EXTIN".as_ptr() },
    emu10k1_reg_entry { base: 0x50, size: 0x10, name: c"P16VIN".as_ptr() },
    emu10k1_reg_entry { base: 0x60, size: 0x20, name: c"EXTOUT".as_ptr() },
    emu10k1_reg_entry { base: 0x80, size: 0x20, name: c"FXBUS2".as_ptr() },
    emu10k1_reg_entry { base: 0xa0, size: 0x10, name: c"EMU32OUTH".as_ptr() },
    emu10k1_reg_entry { base: 0xb0, size: 0x10, name: c"EMU32OUTL".as_ptr() },
    emu10k1_reg_entry { base: 0xc0, size: 0x20, name: ptr::null() }, // Constants
    // This can't be quite right - overlap.
    // { 0x100, 0xc0, "ITRAM_CTL" },
    // { 0x1c0, 0x40, "ETRAM_CTL" },
    emu10k1_reg_entry { base: 0x160, size: 0x20, name: c"A3_EMU32IN".as_ptr() },
    emu10k1_reg_entry { base: 0x1e0, size: 0x20, name: c"A3_EMU32OUT".as_ptr() },
    emu10k1_reg_entry { base: 0x200, size: 0xc0, name: c"ITRAM_DATA".as_ptr() },
    emu10k1_reg_entry { base: 0x2c0, size: 0x40, name: c"ETRAM_DATA".as_ptr() },
    emu10k1_reg_entry { base: 0x300, size: 0xc0, name: c"ITRAM_ADDR".as_ptr() },
    emu10k1_reg_entry { base: 0x3c0, size: 0x40, name: c"ETRAM_ADDR".as_ptr() },
    emu10k1_reg_entry { base: 0x400, size: 0x200, name: c"GPR".as_ptr() },
    emu10k1_reg_entry { base: 0x600, size: 0, name: ptr::null() },
];

static EMU10K1_CONST_ENTRIES: [*const c_char; 29] = [
    c"C_00000000".as_ptr(), c"C_00000001".as_ptr(), c"C_00000002".as_ptr(),
    c"C_00000003".as_ptr(), c"C_00000004".as_ptr(), c"C_00000008".as_ptr(),
    c"C_00000010".as_ptr(), c"C_00000020".as_ptr(), c"C_00000100".as_ptr(),
    c"C_00010000".as_ptr(), c"C_00000800".as_ptr(), c"C_10000000".as_ptr(),
    c"C_20000000".as_ptr(), c"C_40000000".as_ptr(), c"C_80000000".as_ptr(),
    c"C_7fffffff".as_ptr(), c"C_ffffffff".as_ptr(), c"C_fffffffe".as_ptr(),
    c"C_c0000000".as_ptr(), c"C_4f1bbcdc".as_ptr(), c"C_5a7ef9db".as_ptr(),
    c"C_00100000".as_ptr(), c"GPR_ACCU".as_ptr(), c"GPR_COND".as_ptr(),
    c"GPR_NOISE0".as_ptr(), c"GPR_NOISE1".as_ptr(), c"GPR_IRQ".as_ptr(),
    c"GPR_DBAC".as_ptr(), c"GPR_DBACE".as_ptr(),
];

unsafe fn disasm_emu10k1_reg(buffer: *mut c_char, entries: *const emu10k1_reg_entry, mut reg: c_uint, pfx: *const c_char) -> c_int {
    let mut i = 0usize;
    loop {
        let base = (*entries.add(i)).base as c_uint;
        let size = (*entries.add(i)).size as c_uint;
        if size == 0 {
            return sprintf(buffer, c"%s0x%03x".as_ptr(), pfx, reg);
        }
        if reg >= base && reg < base + size {
            let name = (*entries.add(i)).name;
            reg -= base;
            if !name.is_null() {
                return sprintf(buffer, c"%s%s(%u)".as_ptr(), pfx, name, reg);
            }
            return sprintf(buffer, c"%s%s".as_ptr(), pfx, EMU10K1_CONST_ENTRIES[reg as usize]);
        }
        i += 1;
    }
}

unsafe fn disasm_sblive_reg(buffer: *mut c_char, reg: c_uint, pfx: *const c_char) -> c_int {
    disasm_emu10k1_reg(buffer, SBLIVE_REG_ENTRIES.as_ptr(), reg, pfx)
}

unsafe fn disasm_audigy_reg(buffer: *mut c_char, reg: c_uint, pfx: *const c_char) -> c_int {
    disasm_emu10k1_reg(buffer, AUDIGY_REG_ENTRIES.as_ptr(), reg, pfx)
}

unsafe extern "C" fn snd_emu10k1_proc_acode_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let emu = (*entry).private_data;
    static INSNS: [*const c_char; 16] = [
        c"MAC0".as_ptr(), c"MAC1".as_ptr(), c"MAC2".as_ptr(), c"MAC3".as_ptr(),
        c"MACINT0".as_ptr(), c"MACINT1".as_ptr(), c"ACC3".as_ptr(), c"MACMV".as_ptr(),
        c"ANDXOR".as_ptr(), c"TSTNEG".as_ptr(), c"LIMITGE".as_ptr(), c"LIMITLT".as_ptr(),
        c"LOG".as_ptr(), c"EXP".as_ptr(), c"INTERP".as_ptr(), c"SKIP".as_ptr(),
    ];
    static SPACES: &[u8] = b"                              \0";
    let nspaces = SPACES.len() as c_int - 1;

    snd_iprintf(buffer, c"FX8010 Instruction List '%s'\n".as_ptr(), (*emu).fx8010.name);
    snd_iprintf(buffer, c"  Code dump      :\n".as_ptr());
    for pc in 0..(if (*emu).audigy != 0 { 1024 } else { 512 }) {
        let low = snd_emu10k1_efx_read(emu, pc * 2);
        let high = snd_emu10k1_efx_read(emu, pc * 2 + 1);
        let mut buf = [0 as c_char; 100];
        let mut bufp = buf.as_mut_ptr();
        if (*emu).audigy != 0 {
            bufp = bufp.add(sprintf(bufp, c"    %-7s  ".as_ptr(), INSNS[((high >> 24) & 0x0f) as usize]) as usize);
            bufp = bufp.add(disasm_audigy_reg(bufp, (high >> 12) & 0x7ff, c"".as_ptr()) as usize);
            bufp = bufp.add(disasm_audigy_reg(bufp, (high >> 0) & 0x7ff, c", ".as_ptr()) as usize);
            bufp = bufp.add(disasm_audigy_reg(bufp, (low >> 12) & 0x7ff, c", ".as_ptr()) as usize);
            bufp = bufp.add(disasm_audigy_reg(bufp, (low >> 0) & 0x7ff, c", ".as_ptr()) as usize);
        } else {
            bufp = bufp.add(sprintf(bufp, c"    %-7s  ".as_ptr(), INSNS[((high >> 20) & 0x0f) as usize]) as usize);
            bufp = bufp.add(disasm_sblive_reg(bufp, (high >> 10) & 0x3ff, c"".as_ptr()) as usize);
            bufp = bufp.add(disasm_sblive_reg(bufp, (high >> 0) & 0x3ff, c", ".as_ptr()) as usize);
            bufp = bufp.add(disasm_sblive_reg(bufp, (low >> 10) & 0x3ff, c", ".as_ptr()) as usize);
            bufp = bufp.add(disasm_sblive_reg(bufp, (low >> 0) & 0x3ff, c", ".as_ptr()) as usize);
        }
        let len = bufp.offset_from(buf.as_ptr()) as c_int;
        let space_idx = nspaces - clamp(65 - len, 0, nspaces);
        snd_iprintf(buffer, c"%s %s /* 0x%04x: 0x%08x%08x */\n".as_ptr(),
            buf.as_ptr(), SPACES.as_ptr().add(space_idx as usize) as *const c_char, pc, high, low);
    }
}

const TOTAL_SIZE_GPR: c_ulong = 0x100 * 4;
const A_TOTAL_SIZE_GPR: c_ulong = 0x200 * 4;
const TOTAL_SIZE_TANKMEM_DATA: c_ulong = 0xa0 * 4;
const TOTAL_SIZE_TANKMEM_ADDR: c_ulong = 0xa0 * 4;
const A_TOTAL_SIZE_TANKMEM_DATA: c_ulong = 0x100 * 4;
const A_TOTAL_SIZE_TANKMEM_ADDR: c_ulong = 0x100 * 4;
const TOTAL_SIZE_CODE: c_ulong = 0x200 * 8;
const A_TOTAL_SIZE_CODE: c_ulong = 0x400 * 8;

unsafe extern "C" fn snd_emu10k1_fx8010_read(
    entry: *mut snd_info_entry,
    _file_private_data: *mut c_void,
    _file: *mut file,
    buf: *mut c_char,
    count: size_t,
    pos: loff_t,
) -> ssize_t {
    let emu = (*entry).private_data;
    let mut tram_addr = 0;
    let offset: c_uint;

    if strcmp((*entry).name, c"fx8010_tram_addr".as_ptr()) == 0 {
        offset = TANKMEMADDRREGBASE;
        tram_addr = 1;
    } else if strcmp((*entry).name, c"fx8010_tram_data".as_ptr()) == 0 {
        offset = TANKMEMDATAREGBASE;
    } else if strcmp((*entry).name, c"fx8010_code".as_ptr()) == 0 {
        offset = if (*emu).audigy != 0 { A_MICROCODEBASE } else { MICROCODEBASE };
    } else {
        offset = if (*emu).audigy != 0 { A_FXGPREGBASE } else { FXGPREGBASE };
    }

    let tmp = kmalloc(count + 8, GFP_KERNEL) as *mut c_uint;
    if tmp.is_null() {
        return -(ENOMEM as ssize_t);
    }
    let words = (((pos as size_t) & 3) + count + 3) >> 2;
    for idx in 0..words {
        let mut val = snd_emu10k1_ptr_read(emu, offset + idx as c_uint + ((pos >> 2) as c_uint), 0);
        if tram_addr != 0 && (*emu).audigy != 0 {
            val >>= 11;
            val |= snd_emu10k1_ptr_read(emu, 0x100 + idx as c_uint + ((pos >> 2) as c_uint), 0) << 20;
        }
        *tmp.add(idx) = val;
    }
    let res = if copy_to_user(buf, (tmp as *const c_char).add((pos as usize) & 3) as *const c_void, count) != 0 {
        -(EFAULT as ssize_t)
    } else {
        count as ssize_t
    };
    kfree(tmp as *mut c_void);
    res
}

unsafe extern "C" fn snd_emu10k1_proc_voices_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let emu = (*entry).private_data;
    static TYPES: [*const c_char; 6] = [
        c"Unused".as_ptr(), c"EFX".as_ptr(), c"EFX IRQ".as_ptr(),
        c"PCM".as_ptr(), c"PCM IRQ".as_ptr(), c"Synth".as_ptr(),
    ];
    const _: [(); EMU10K1_NUM_TYPES] = [(); 6];

    snd_iprintf(buffer, c"ch\tdirty\tlast\tuse\n".as_ptr());
    for idx in 0..NUM_G {
        let voice = (*emu).voices.add(idx as usize);
        snd_iprintf(buffer, c"%i\t%u\t%u\t%s\n".as_ptr(),
            idx, (*voice).dirty, (*voice).last, TYPES[(*voice).use_]);
    }
}

/* CONFIG_SND_DEBUG conditional code from the original file. */

unsafe extern "C" fn snd_emu_proc_emu1010_link_read(
    emu: *mut snd_emu10k1,
    buffer: *mut snd_info_buffer,
    dst: u32,
) {
    let src = snd_emu1010_fpga_link_dst_src_read(emu, dst);
    snd_iprintf(buffer, c"%04x: %04x\n".as_ptr(), dst, src);
}

unsafe extern "C" fn snd_emu_proc_emu1010_reg_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let emu = (*entry).private_data;
    let mut value: u32 = 0;

    snd_iprintf(buffer, c"EMU1010 Registers:\n\n".as_ptr());
    for i in 0..0x40 {
        snd_emu1010_fpga_read(emu, i, &mut value);
        snd_iprintf(buffer, c"%02x: %02x\n".as_ptr(), i, value);
    }

    snd_iprintf(buffer, c"\nEMU1010 Routes:\n\n".as_ptr());
    for i in 0..16 {
        snd_emu_proc_emu1010_link_read(emu, buffer, i);
    }
    if (*(*emu).card_capabilities).emu_model != EMU_MODEL_EMU0404 {
        for i in 0..32 {
            snd_emu_proc_emu1010_link_read(emu, buffer, 0x100 + i);
        }
    }
    if (*(*emu).card_capabilities).emu_model != EMU_MODEL_EMU1616 {
        for i in 0..8 {
            snd_emu_proc_emu1010_link_read(emu, buffer, 0x200 + i);
        }
    }
    for i in 0..8 {
        snd_emu_proc_emu1010_link_read(emu, buffer, 0x300 + i);
    }
    if (*(*emu).card_capabilities).emu_model == EMU_MODEL_EMU1616 {
        for i in 0..16 {
            snd_emu_proc_emu1010_link_read(emu, buffer, 0x400 + i);
        }
    } else if (*(*emu).card_capabilities).emu_model != EMU_MODEL_EMU0404 {
        for i in 0..8 {
            snd_emu_proc_emu1010_link_read(emu, buffer, 0x400 + i);
        }
        if (*(*emu).card_capabilities).emu_model == EMU_MODEL_EMU1010B {
            for i in 0..16 {
                snd_emu_proc_emu1010_link_read(emu, buffer, 0x500 + i);
            }
        } else {
            // To Alice2 via I2S
            snd_emu_proc_emu1010_link_read(emu, buffer, 0x500);
            snd_emu_proc_emu1010_link_read(emu, buffer, 0x501);
            snd_emu_proc_emu1010_link_read(emu, buffer, 0x600);
            snd_emu_proc_emu1010_link_read(emu, buffer, 0x601);
            snd_emu_proc_emu1010_link_read(emu, buffer, 0x700);
            snd_emu_proc_emu1010_link_read(emu, buffer, 0x701);
        }
    }
}

unsafe extern "C" fn snd_emu_proc_io_reg_read(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let emu = (*entry).private_data;
    snd_iprintf(buffer, c"IO Registers:\n\n".as_ptr());
    let mut i = 0;
    while i < 0x40 {
        let value = inl((*emu).port + i);
        snd_iprintf(buffer, c"%02X: %08lX\n".as_ptr(), i, value);
        i += 4;
    }
}

unsafe extern "C" fn snd_emu_proc_io_reg_write(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let emu = (*entry).private_data;
    let mut line = [0 as c_char; 64];
    let mut reg: u32 = 0;
    let mut val: u32 = 0;
    while snd_info_get_line(buffer, line.as_mut_ptr(), line.len() as c_int) == 0 {
        if sscanf(line.as_ptr(), c"%x %x".as_ptr(), &mut reg, &mut val) != 2 {
            continue;
        }
        if reg < 0x40 && val <= 0xffffffff {
            outl(val, (*emu).port + ((reg & 0xfffffffc) as c_ulong));
        }
    }
}

unsafe fn snd_ptr_read(emu: *mut snd_emu10k1, iobase: c_uint, reg: c_uint, chn: c_uint) -> c_uint {
    let regptr = (reg << 16) | chn;

    spin_lock_irq(&mut (*emu).emu_lock);
    outl(regptr, (*emu).port + iobase as c_ulong + PTR);
    let ret = inl((*emu).port + iobase as c_ulong + DATA) as c_uint;
    spin_unlock_irq(&mut (*emu).emu_lock);
    ret
}

unsafe fn snd_ptr_write(emu: *mut snd_emu10k1, iobase: c_uint, reg: c_uint, chn: c_uint, data: c_uint) {
    let regptr = (reg << 16) | chn;

    spin_lock_irq(&mut (*emu).emu_lock);
    outl(regptr, (*emu).port + iobase as c_ulong + PTR);
    outl(data, (*emu).port + iobase as c_ulong + DATA);
    spin_unlock_irq(&mut (*emu).emu_lock);
}

unsafe fn snd_emu_proc_ptr_reg_read(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
    iobase: c_int,
    offset: c_int,
    length: c_int,
    voices: c_int,
) {
    let emu = (*entry).private_data;
    if offset + length > 0xa0 {
        snd_iprintf(buffer, c"Input values out of range\n".as_ptr());
        return;
    }
    snd_iprintf(buffer, c"Registers 0x%x\n".as_ptr(), iobase);
    for i in offset..offset + length {
        snd_iprintf(buffer, c"%02X: ".as_ptr(), i);
        for j in 0..voices {
            let value = snd_ptr_read(emu, iobase as c_uint, i as c_uint, j as c_uint) as c_ulong;
            snd_iprintf(buffer, c"%08lX ".as_ptr(), value);
        }
        snd_iprintf(buffer, c"\n".as_ptr());
    }
}

unsafe fn snd_emu_proc_ptr_reg_write(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
    iobase: c_int,
    length: c_int,
    voices: c_int,
) {
    let emu = (*entry).private_data;
    let mut line = [0 as c_char; 64];
    let mut reg: c_uint = 0;
    let mut channel_id: c_uint = 0;
    let mut val: c_uint = 0;
    while snd_info_get_line(buffer, line.as_mut_ptr(), line.len() as c_int) == 0 {
        if sscanf(line.as_ptr(), c"%x %x %x".as_ptr(), &mut reg, &mut channel_id, &mut val) != 3 {
            continue;
        }
        if reg < length as c_uint && channel_id < voices as c_uint {
            snd_ptr_write(emu, iobase as c_uint, reg, channel_id, val);
        }
    }
}

unsafe extern "C" fn snd_emu_proc_ptr_reg_write00(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    snd_emu_proc_ptr_reg_write(entry, buffer, 0, 0x80, 64);
}

unsafe extern "C" fn snd_emu_proc_ptr_reg_write20(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    let emu = (*entry).private_data;
    snd_emu_proc_ptr_reg_write(
        entry,
        buffer,
        0x20,
        if (*(*emu).card_capabilities).ca0108_chip != 0 { 0xa0 } else { 0x80 },
        4,
    );
}

unsafe extern "C" fn snd_emu_proc_ptr_reg_read00a(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    snd_emu_proc_ptr_reg_read(entry, buffer, 0, 0, 0x40, 64);
}

unsafe extern "C" fn snd_emu_proc_ptr_reg_read00b(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    snd_emu_proc_ptr_reg_read(entry, buffer, 0, 0x40, 0x40, 64);
}

unsafe extern "C" fn snd_emu_proc_ptr_reg_read20a(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    snd_emu_proc_ptr_reg_read(entry, buffer, 0x20, 0, 0x40, 4);
}

unsafe extern "C" fn snd_emu_proc_ptr_reg_read20b(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    snd_emu_proc_ptr_reg_read(entry, buffer, 0x20, 0x40, 0x40, 4);
}

unsafe extern "C" fn snd_emu_proc_ptr_reg_read20c(entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    snd_emu_proc_ptr_reg_read(entry, buffer, 0x20, 0x80, 0x20, 4);
}

static SND_EMU10K1_PROC_OPS_FX8010: snd_info_entry_ops = snd_info_entry_ops {
    read: Some(snd_emu10k1_fx8010_read),
};

#[no_mangle]
pub unsafe extern "C" fn snd_emu10k1_proc_init(emu: *mut snd_emu10k1) -> c_int {
    let mut entry: *mut snd_info_entry = ptr::null_mut();

    /* CONFIG_SND_DEBUG conditional registration from the original file. */
    if (*(*emu).card_capabilities).emu_model != 0 {
        snd_card_ro_proc_new((*emu).card, c"emu1010_regs".as_ptr(), emu, snd_emu_proc_emu1010_reg_read);
    }
    snd_card_rw_proc_new((*emu).card, c"io_regs".as_ptr(), emu, snd_emu_proc_io_reg_read, snd_emu_proc_io_reg_write);
    snd_card_rw_proc_new((*emu).card, c"ptr_regs00a".as_ptr(), emu, snd_emu_proc_ptr_reg_read00a, snd_emu_proc_ptr_reg_write00);
    snd_card_rw_proc_new((*emu).card, c"ptr_regs00b".as_ptr(), emu, snd_emu_proc_ptr_reg_read00b, snd_emu_proc_ptr_reg_write00);
    if (*(*emu).card_capabilities).emu_model == 0 &&
        ((*(*emu).card_capabilities).ca0151_chip != 0 || (*(*emu).card_capabilities).ca0108_chip != 0) {
        snd_card_rw_proc_new((*emu).card, c"ptr_regs20a".as_ptr(), emu, snd_emu_proc_ptr_reg_read20a, snd_emu_proc_ptr_reg_write20);
        snd_card_rw_proc_new((*emu).card, c"ptr_regs20b".as_ptr(), emu, snd_emu_proc_ptr_reg_read20b, snd_emu_proc_ptr_reg_write20);
        if (*(*emu).card_capabilities).ca0108_chip != 0 {
            snd_card_rw_proc_new((*emu).card, c"ptr_regs20c".as_ptr(), emu, snd_emu_proc_ptr_reg_read20c, snd_emu_proc_ptr_reg_write20);
        }
    }

    snd_card_ro_proc_new((*emu).card, c"emu10k1".as_ptr(), emu, snd_emu10k1_proc_read);

    if (*(*emu).card_capabilities).emu10k2_chip != 0 {
        snd_card_ro_proc_new((*emu).card, c"spdif-in".as_ptr(), emu, snd_emu10k1_proc_spdif_read);
    }
    if (*(*emu).card_capabilities).ca0151_chip != 0 {
        snd_card_ro_proc_new((*emu).card, c"capture-rates".as_ptr(), emu, snd_emu10k1_proc_rates_read);
    }

    snd_card_ro_proc_new((*emu).card, c"voices".as_ptr(), emu, snd_emu10k1_proc_voices_read);

    if snd_card_proc_new((*emu).card, c"fx8010_gpr".as_ptr(), &mut entry) == 0 {
        (*entry).content = SNDRV_INFO_CONTENT_DATA;
        (*entry).private_data = emu;
        (*entry).mode = S_IFREG | 0o444 /*| S_IWUSR*/;
        (*entry).size = if (*emu).audigy != 0 { A_TOTAL_SIZE_GPR } else { TOTAL_SIZE_GPR };
        (*entry).c.ops = &SND_EMU10K1_PROC_OPS_FX8010;
    }
    if snd_card_proc_new((*emu).card, c"fx8010_tram_data".as_ptr(), &mut entry) == 0 {
        (*entry).content = SNDRV_INFO_CONTENT_DATA;
        (*entry).private_data = emu;
        (*entry).mode = S_IFREG | 0o444 /*| S_IWUSR*/;
        (*entry).size = if (*emu).audigy != 0 { A_TOTAL_SIZE_TANKMEM_DATA } else { TOTAL_SIZE_TANKMEM_DATA };
        (*entry).c.ops = &SND_EMU10K1_PROC_OPS_FX8010;
    }
    if snd_card_proc_new((*emu).card, c"fx8010_tram_addr".as_ptr(), &mut entry) == 0 {
        (*entry).content = SNDRV_INFO_CONTENT_DATA;
        (*entry).private_data = emu;
        (*entry).mode = S_IFREG | 0o444 /*| S_IWUSR*/;
        (*entry).size = if (*emu).audigy != 0 { A_TOTAL_SIZE_TANKMEM_ADDR } else { TOTAL_SIZE_TANKMEM_ADDR };
        (*entry).c.ops = &SND_EMU10K1_PROC_OPS_FX8010;
    }
    if snd_card_proc_new((*emu).card, c"fx8010_code".as_ptr(), &mut entry) == 0 {
        (*entry).content = SNDRV_INFO_CONTENT_DATA;
        (*entry).private_data = emu;
        (*entry).mode = S_IFREG | 0o444 /*| S_IWUSR*/;
        (*entry).size = if (*emu).audigy != 0 { A_TOTAL_SIZE_CODE } else { TOTAL_SIZE_CODE };
        (*entry).c.ops = &SND_EMU10K1_PROC_OPS_FX8010;
    }
    snd_card_ro_proc_new((*emu).card, c"fx8010_acode".as_ptr(), emu, snd_emu10k1_proc_acode_read);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
