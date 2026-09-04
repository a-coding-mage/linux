// SPDX-License-Identifier: GPL-2.0-or-later
//
//  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//                   James Courtier-Dutton <James@superbug.co.uk>
//                   Oswald Buddenhagen <oswald.buddenhagen@gmx.de>
//                   Creative Labs, Inc.
//
//  Routines for control of EMU10K1 chips

// External dependencies from linux kernel headers
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_ulong;
use core::ptr::null_mut;
use core::ptr::null;

// Linux kernel types
#[repr(C)]
pub struct SndEmu10k1 {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct SndCard {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct PciDev {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct Firmware {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct SndCtlElemId {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct WorkStruct {
    _opaque: [u8; 0],
}

// Firmware filenames
const HANA_FILENAME: &str = "emu/hana.fw";
const DOCK_FILENAME: &str = "emu/audio_dock.fw";
const EMU1010B_FILENAME: &str = "emu/emu1010b.fw";
const MICRO_DOCK_FILENAME: &str = "emu/micro_dock.fw";
const EMU0404_FILENAME: &str = "emu/emu0404.fw";
const EMU1010_NOTEBOOK_FILENAME: &str = "emu/emu1010_notebook.fw";

// MODULE_FIRMWARE macros are translated to constants
const _FIRMWARE_HANA: &str = HANA_FILENAME;
const _FIRMWARE_DOCK: &str = DOCK_FILENAME;
const _FIRMWARE_EMU1010B: &str = EMU1010B_FILENAME;
const _FIRMWARE_MICRO_DOCK: &str = MICRO_DOCK_FILENAME;
const _FIRMWARE_EMU0404: &str = EMU0404_FILENAME;
const _FIRMWARE_EMU1010_NOTEBOOK: &str = EMU1010_NOTEBOOK_FILENAME;

//
// EMU10K1 init / done
//

// External function declarations
extern "C" {
    fn snd_emu10k1_ptr_write_multiple(emu: *mut SndEmu10k1, ch: c_int, ...);
    fn snd_emu10k1_ptr_write(emu: *mut SndEmu10k1, reg: c_uint, ch: c_int, data: u32);
    fn snd_emu10k1_ptr20_write(emu: *mut SndEmu10k1, reg: c_uint, ch: c_int, data: u32);
    fn snd_emu10k1_ptr_read(emu: *mut SndEmu10k1, reg: c_uint, ch: c_int) -> u32;
    fn snd_emu10k1_spi_write(emu: *mut SndEmu10k1, data: c_uint) -> c_int;
    fn snd_emu10k1_i2c_write(emu: *mut SndEmu10k1, addr: c_uint, data: c_uint);
    fn snd_emu10k1_wait(emu: *mut SndEmu10k1, ticks: c_uint);
    fn snd_emu10k1_intr_enable(emu: *mut SndEmu10k1, intrenb: u32);
    fn snd_emu10k1_free_efx(emu: *mut SndEmu10k1);
    fn snd_emu10k1_fx8010_tram_setup(emu: *mut SndEmu10k1, size: c_uint);
    fn snd_emu1010_load_firmware_entry(emu: *mut SndEmu10k1, dock: c_int, fw: *const Firmware);
    fn snd_emu1010_fpga_write(emu: *mut SndEmu10k1, reg: u32, value: u32);
    fn snd_emu1010_fpga_read(emu: *mut SndEmu10k1, reg: u32, val: *mut u32);
    fn snd_emu1010_update_clock(emu: *mut SndEmu10k1);
    fn snd_emu1010_fpga_write_lock(emu: *mut SndEmu10k1, reg: u32, value: u32);
    fn snd_util_memhdr_free(hdr: *mut core::ffi::c_void);
    fn snd_util_memhdr_new(size: c_ulong) -> *mut core::ffi::c_void;
    fn snd_ctl_build_ioff(id: *mut SndCtlElemId, kctl: *mut core::ffi::c_void, offset: c_uint);
    fn snd_ctl_notify(card: *mut SndCard, mask: c_uint, id: *mut SndCtlElemId);
    fn snd_emu10k1_init_efx(emu: *mut SndEmu10k1) -> c_int;
    fn snd_emu10k1_proc_init(emu: *mut SndEmu10k1);
    fn snd_emu10k1_efx_alloc_pm_buffer(emu: *mut SndEmu10k1) -> c_int;
    fn snd_emu10k1_efx_free_pm_buffer(emu: *mut SndEmu10k1);
    fn snd_p16v_alloc_pm_buffer(emu: *mut SndEmu10k1) -> c_int;
    fn snd_p16v_free_pm_buffer(emu: *mut SndEmu10k1);
    fn snd_emu10k1_alloc_pages_maybe_wider(emu: *mut SndEmu10k1, size: c_ulong, dma: *mut core::ffi::c_void) -> c_int;
    fn snd_dma_free_pages(dma: *mut core::ffi::c_void);
    fn dev_info(dev: *mut core::ffi::c_void, fmt: *const c_int, ...);
    fn dev_dbg(dev: *mut core::ffi::c_void, fmt: *const c_int, ...);
    fn dev_err(dev: *mut core::ffi::c_void, fmt: *const c_int, ...);
    fn dev_notice(dev: *mut core::ffi::c_void, fmt: *const c_int, ...);
    fn outl(value: u32, port: c_ulong);
    fn outw(value: u16, port: c_ulong);
    fn inl(port: c_ulong) -> u32;
    fn inw(port: c_ulong) -> u16;
    fn udelay(usecs: c_uint);
    fn msleep(msecs: c_uint);
    fn request_firmware(fw: *mut *const Firmware, name: *const u8, device: *mut core::ffi::c_void) -> c_int;
    fn release_firmware(fw: *const Firmware);
    fn vfree(ptr: *mut core::ffi::c_void);
    fn vmalloc(size: c_ulong) -> *mut core::ffi::c_void;
    fn pcim_enable_device(pci: *mut PciDev) -> c_int;
    fn pcim_request_all_regions(pci: *mut PciDev, name: *const u8) -> c_int;
    fn pci_resource_start(pci: *mut PciDev, bar: c_int) -> c_ulong;
    fn pci_set_master(pci: *mut PciDev);
    fn pci_read_config_dword(pci: *mut PciDev, where_: u32, val: *mut u32);
    fn pci_read_config_word(pci: *mut PciDev, where_: u32, val: *mut u16);
    fn dma_set_mask_and_coherent(dev: *mut core::ffi::c_void, mask: c_ulong) -> c_int;
    fn devm_request_irq(dev: *mut core::ffi::c_void, irq: c_uint, handler: unsafe extern "C" fn(*mut core::ffi::c_void) -> c_int, irqflags: c_ulong, devname: *const u8, dev_id: *mut core::ffi::c_void) -> c_int;
    fn schedule_work(work: *mut WorkStruct) -> bool;
    fn cancel_work_sync(work: *mut WorkStruct) -> bool;
    fn iommu_get_domain_for_dev(dev: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn strscpy(dest: *mut u8, src: *const u8, count: c_ulong) -> c_int;
    fn array_size(a: c_ulong, b: c_ulong) -> c_ulong;
    fn array3_size(a: c_ulong, b: c_ulong, c: c_ulong) -> c_ulong;
}

// Device type enums
const EMU_MODEL_EMU1010: u32 = 0;
const EMU_MODEL_EMU1010B: u32 = 1;
const EMU_MODEL_EMU1616: u32 = 2;
const EMU_MODEL_EMU0404: u32 = 3;

// Register constants - these would be defined in headers
const DCYSUSV: u32 = 0; // Placeholder
const VTFT: u32 = 1;
const VTFT_FILTERTARGET_MASK: u32 = 0xFFFF;
const CVCF: u32 = 2;
const CVCF_CURRENTFILTER_MASK: u32 = 0xFFFF;
const PTRX: u32 = 3;
const CPF: u32 = 4;
const CCR: u32 = 5;
const PSST: u32 = 6;
const DSL: u32 = 7;
const CCCA: u32 = 8;
const Z1: u32 = 9;
const Z2: u32 = 10;
const FXRT: u32 = 11;
const DCYSUSM: u32 = 12;
const ATKHLDV: u32 = 13;
const ATKHLDM: u32 = 14;
const IP: u32 = 15;
const IFATN: u32 = 16;
const IFATN_FILTERCUTOFF_MASK: u32 = 0xFF;
const IFATN_ATTENUATION_MASK: u32 = 0xFF00;
const PEFE: u32 = 17;
const FMMOD: u32 = 18;
const TREMFRQ: u32 = 19;
const FM2FRQ2: u32 = 20;
const LFOVAL2: u32 = 21;
const LFOVAL1: u32 = 22;
const ENVVOL: u32 = 23;
const ENVVAL: u32 = 24;
const A_CSBA: u32 = 25;
const A_CSDC: u32 = 26;
const A_CSFE: u32 = 27;
const A_CSHG: u32 = 28;
const A_FXRT1: u32 = 29;
const A_FXRT2: u32 = 30;
const A_SENDAMOUNTS: u32 = 31;
const REGLIST_END: u32 = 0;
const NUM_G: c_int = 64;
const MICBS: u32 = 32;
const ADCBS_BUFSIZE_NONE: u32 = 0;
const MICBA: u32 = 33;
const FXBS: u32 = 34;
const FXBA: u32 = 35;
const ADCBS: u32 = 36;
const ADCBA: u32 = 37;
const CLIEL: u32 = 38;
const CLIEH: u32 = 39;
const SOLEL: u32 = 40;
const SOLEH: u32 = 41;
const SPBYPASS: u32 = 42;
const SPBYPASS_FORMAT: u32 = 0;
const AC97SLOT: u32 = 43;
const AC97SLOT_REAR_RIGHT: u32 = 0x01;
const AC97SLOT_REAR_LEFT: u32 = 0x02;
const AC97SLOT_CNTR: u32 = 0x04;
const AC97SLOT_LFE: u32 = 0x08;
const SPCS0: u32 = 44;
const SPCS1: u32 = 45;
const SPCS2: u32 = 46;
const A_I2S_CAPTURE_RATE: u32 = 47;
const A_I2S_CAPTURE_96000: u32 = 0;
const SRCSel: u32 = 48;
const SRCMULTI_ENABLE: u32 = 49;
const HCFG2: u32 = 50;
const CAPTURE_P16V_SOURCE: u32 = 51;
const P17V_SRCSel: u32 = 52;
const P17V_MIXER_I2S_ENABLE: u32 = 53;
const P17V_MIXER_SPDIF_ENABLE: u32 = 54;
const A_IOCFG: u32 = 55;
const A_IOCFG_GPOUT2: u16 = 0x0004;
const A_IOCFG_GPOUT1: u16 = 0x0002;
const A_IOCFG_GPOUT0: u16 = 0x0001;
const HCFG: u32 = 56;
const HCFG_LOCKSOUNDCACHE: u32 = 0x00000004;
const HCFG_LOCKTANKCACHE_MASK: u32 = 0x00000002;
const HCFG_MUTEBUTTONENABLE: u32 = 0x00000400;
const HCFG_AUDIOENABLE: u32 = 0x00000001;
const HCFG_AUTOMUTE: u32 = 0x00000800;
const HCFG_JOYENABLE: u32 = 0x00000200;
const HCFG_AC3ENABLE_CDSPDIF: u32 = 0x00000100;
const HCFG_AC3ENABLE_GPSPDIF: u32 = 0x00000080;
const HCFG_EXPANDED_MEM: u32 = 0x00000020;
const HCFG_AUTOMUTE_ASYNC: u32 = 0x00001000;
const HCFG_EMU32_SLAVE: u32 = 0x00000010;
const HCFG_CODECFORMAT_I2S: u32 = 0x00000002;
const HCFG_GPINPUT0: u32 = 0x00004000;
const HCFG_GPINPUT1: u32 = 0x00008000;
const HCFG_GPOUT2: u32 = 0x00001000;
const HCFG_GPOUT1: u32 = 0x00000800;
const INTE: u32 = 57;
const PTB: u32 = 58;
const TCB: u32 = 59;
const TCBS: u32 = 60;
const TCBS_BUFFSIZE_256K: u32 = 0x8000;
const TCBS_BUFFSIZE_16K: u32 = 0x0000;
const MAPA: u32 = 61;
const MAPB: u32 = 62;
const MAP_PTI_MASK0: u32 = 0;
const MAP_PTI_MASK1: u32 = 1;
const DBG: u32 = 63;
const EMU10K1_DBG_SINGLE_STEP: u32 = 0x00000001;
const A_DBG: u32 = 64;
const A_DBG_SINGLE_STEP: u32 = 0x00000001;
const INTE_PCIERRORENABLE: u32 = 0x00000200;
const INTE_A_GPIOENABLE: u32 = 0x00000020;
const P17V_I2S_SRC_SEL: u32 = 65;
const ADC_MUX_2: u8 = 0x02;
const ARRAY_SIZE_SPI_DAC_INIT: usize = 21;
const ARRAY_SIZE_I2C_ADC_INIT: usize = 15;

// ECARD constants
const HOOKN_BIT: u32 = 1 << 12;
const HANDN_BIT: u32 = 1 << 11;
const PULSEN_BIT: u32 = 1 << 10;
const EC_GDI1: u32 = 1 << 13;
const EC_GDI0: u32 = 1 << 14;
const EC_NUM_CONTROL_BITS: u16 = 20;
const EC_AC3_DATA_SELN: u32 = 0x0001;
const EC_EE_DATA_SEL: u32 = 0x0002;
const EC_EE_CNTRL_SELN: u32 = 0x0004;
const EC_EECLK: u32 = 0x0008;
const EC_EECS: u32 = 0x0010;
const EC_EESDO: u32 = 0x0020;
const EC_TRIM_CSN: u32 = 0x0040;
const EC_TRIM_SCLK: u32 = 0x0080;
const EC_TRIM_SDATA: u32 = 0x0100;
const EC_TRIM_MUTEN: u32 = 0x0200;
const EC_ADCCAL: u32 = 0x0400;
const EC_ADCRSTN: u32 = 0x0800;
const EC_DACCAL: u32 = 0x1000;
const EC_DACMUTEN: u32 = 0x2000;
const EC_LEDN: u32 = 0x4000;
const EC_SPDIF0_SEL_SHIFT: u32 = 15;
const EC_SPDIF1_SEL_SHIFT: u32 = 17;
const EC_SPDIF0_SEL_MASK: u32 = 0x3 << EC_SPDIF0_SEL_SHIFT;
const EC_SPDIF1_SEL_MASK: u32 = 0x7 << EC_SPDIF1_SEL_SHIFT;
const EC_CURRENT_PROM_VERSION: u32 = 0x01;
const EC_EEPROM_SIZE: u32 = 0x40;
const EC_PROM_VERSION_ADDR: u32 = 0x20;
const EC_BOARDREV0_ADDR: u32 = 0x21;
const EC_BOARDREV1_ADDR: u32 = 0x22;
const EC_LAST_PROMFILE_ADDR: u32 = 0x2f;
const EC_SERIALNUM_ADDR: u32 = 0x30;
const EC_CHECKSUM_ADDR: u32 = 0x3f;
const EC_RAW_RUN_MODE: u32 = EC_DACMUTEN | EC_ADCRSTN | EC_TRIM_MUTEN | EC_TRIM_CSN;
const EC_DEFAULT_ADC_GAIN: u16 = 0xC4C4;
const EC_DEFAULT_SPDIF0_SEL: u32 = 0x0;
const EC_DEFAULT_SPDIF1_SEL: u32 = 0x4;

// EMU1010 constants
const EMU_HANA_FPGA_CONFIG: u32 = 0;
const EMU_HANA_ID: u32 = 1;
const EMU_HANA_OPTION_CARDS: u32 = 2;
const EMU_HANA_OPTION_DOCK_OFFLINE: u32 = 0x01;
const EMU_HANA_OPTION_DOCK_ONLINE: u32 = 0x02;
const EMU_HANA_DOCK_MAJOR_REV: u32 = 3;
const EMU_HANA_DOCK_MINOR_REV: u32 = 4;
const EMU_HANA_DOCK_PWR: u32 = 5;
const EMU_HANA_DOCK_PWR_ON: u32 = 0x01;
const EMU_HANA_OPTICAL_TYPE: u32 = 6;
const EMU_HANA_OPTICAL_IN_ADAT: u32 = 0x01;
const EMU_HANA_OPTICAL_IN_SPDIF: u32 = 0x00;
const EMU_HANA_OPTICAL_OUT_ADAT: u32 = 0x02;
const EMU_HANA_OPTICAL_OUT_SPDIF: u32 = 0x00;
const EMU_HANA_ADC_PADS: u32 = 7;
const EMU_HANA_DOCK_MISC: u32 = 8;
const EMU_HANA_DOCK_PHONES_192_DAC4: u32 = 0x01;
const EMU_HANA_DAC_PADS: u32 = 9;
const EMU_HANA_DOCK_DAC_PAD1: u32 = 0x01;
const EMU_HANA_DOCK_DAC_PAD2: u32 = 0x02;
const EMU_HANA_DOCK_DAC_PAD3: u32 = 0x04;
const EMU_HANA_DOCK_DAC_PAD4: u32 = 0x08;
const EMU_HANA_SPDIF_MODE: u32 = 10;
const EMU_HANA_SPDIF_MODE_RX_INVALID: u32 = 0x01;
const EMU_HANA_MIDI_IN: u32 = 11;
const EMU_HANA_MIDI_INA_FROM_HAMOA: u32 = 0x01;
const EMU_HANA_MIDI_INB_FROM_DOCK2: u32 = 0x02;
const EMU_HANA_MIDI_OUT: u32 = 12;
const EMU_HANA_MIDI_OUT_DOCK2: u32 = 0x01;
const EMU_HANA_MIDI_OUT_SYNC2: u32 = 0x02;
const EMU_HANA_IRQ_ENABLE: u32 = 13;
const EMU_HANA_IRQ_DOCK: u32 = 0x01;
const EMU_HANA_IRQ_DOCK_LOST: u32 = 0x02;
const EMU_HANA_IRQ_WCLK_CHANGED: u32 = 0x04;
const EMU_HANA_IRQ_STATUS: u32 = 14;
const EMU_HANA_DEFCLOCK: u32 = 15;
const EMU_HANA_DEFCLOCK_48K: u32 = 0x00;
const EMU_HANA_WCLOCK: u32 = 16;
const EMU_HANA_WCLOCK_INT_48K: u32 = 0x00;
const EMU_HANA_WCLOCK_4X: u32 = 0x01;
const EMU_HANA_MAJOR_REV: u32 = 17;
const EMU_HANA_MINOR_REV: u32 = 18;
const EMU_UNMUTE: u32 = 0x01;

// SPDIF constants
const SPCS_CLKACCY_1000PPM: u32 = 0x00000000;
const SPCS_SAMPLERATE_48: u32 = 0x00000002;
const SPCS_CHANNELNUM_LEFT: u32 = 0x00000000;
const SPCS_SOURCENUM_UNSPEC: u32 = 0x00000000;
const SPCS_GENERATIONSTATUS: u32 = 0x00000200;
const SPCS_EMPHASIS_NONE: u32 = 0x00000000;
const SPCS_COPYRIGHT: u32 = 0x00000004;

// Audigy specific constants
const AUDIGY_DMA_MASK: c_ulong = 0xffffffff;
const EMU10K1_DMA_MASK: c_ulong = 0xffffffff;
const FXGPREGBASE: u32 = 0x100;
const A_FXGPREGBASE: u32 = 0x400;
const MAXPAGES0: usize = 4096;
const MAXPAGES1: usize = 8192;
const EMUPAGESIZE: c_ulong = 4096;
const PAGE_SHIFT: u32 = 12;

// SPI DAC initialization array
static SPI_DAC_INIT: &[u32] = &[
    0x00ff, 0x02ff, 0x0400, 0x0520, 0x0600, 0x08ff, 0x0aff, 0x0cff,
    0x0eff, 0x10ff, 0x1200, 0x1400, 0x1480, 0x1800, 0x1aff, 0x1cff,
    0x1e00, 0x0530, 0x0602, 0x0622, 0x1400,
];

// I2C ADC initialization array
static I2C_ADC_INIT: &[[u32; 2]] = &[
    [0x17, 0x00],
    [0x07, 0x00],
    [0x0b, 0x22],
    [0x0c, 0x22],
    [0x0d, 0x08],
    [0x0e, 0xcf],
    [0x0f, 0xcf],
    [0x10, 0x7b],
    [0x11, 0x00],
    [0x12, 0x32],
    [0x13, 0x00],
    [0x14, 0xa6],
    [0x15, ADC_MUX_2 as u32],
];

// Firmware file names per model
static FIRMWARE_NAMES: &[&[&str; 2]] = &[
    &[HANA_FILENAME, DOCK_FILENAME],                          // EMU_MODEL_EMU1010
    &[EMU1010B_FILENAME, MICRO_DOCK_FILENAME],               // EMU_MODEL_EMU1010B
    &[EMU1010_NOTEBOOK_FILENAME, MICRO_DOCK_FILENAME],       // EMU_MODEL_EMU1616
    &[EMU0404_FILENAME, ""],                                  // EMU_MODEL_EMU0404
];

pub unsafe fn snd_emu10k1_voice_init(emu: *mut SndEmu10k1, ch: c_int) {
    snd_emu10k1_ptr_write_multiple(
        emu, ch,
        DCYSUSV, 0,
        VTFT, VTFT_FILTERTARGET_MASK,
        CVCF, CVCF_CURRENTFILTER_MASK,
        PTRX, 0,
        CPF, 0,
        CCR, 0,
        PSST, 0,
        DSL, 0x10,
        CCCA, 0,
        Z1, 0,
        Z2, 0,
        FXRT, 0x32100000,
        DCYSUSM, 0,
        ATKHLDV, 0,
        ATKHLDM, 0,
        IP, 0,
        IFATN, IFATN_FILTERCUTOFF_MASK | IFATN_ATTENUATION_MASK,
        PEFE, 0,
        FMMOD, 0,
        TREMFRQ, 24,
        FM2FRQ2, 24,
        LFOVAL2, 0,
        LFOVAL1, 0,
        ENVVOL, 0,
        ENVVAL, 0,
        REGLIST_END,
    );

    // Audigy extra stuffs
    if (*emu).audigy != 0 {
        snd_emu10k1_ptr_write_multiple(
            emu, ch,
            A_CSBA, 0,
            A_CSDC, 0,
            A_CSFE, 0,
            A_CSHG, 0,
            A_FXRT1, 0x03020100,
            A_FXRT2, 0x07060504,
            A_SENDAMOUNTS, 0,
            REGLIST_END,
        );
    }
}

pub unsafe fn snd_emu10k1_init(emu: *mut SndEmu10k1, enable_ir: c_int) -> c_int {
    let mut silent_page: u32;
    let mut ch: c_int;
    let mut tmp: u32;

    outl(
        HCFG_LOCKSOUNDCACHE | HCFG_LOCKTANKCACHE_MASK | HCFG_MUTEBUTTONENABLE,
        (*emu).port + HCFG as c_ulong,
    );

    outl(0, (*emu).port + INTE as c_ulong);

    snd_emu10k1_ptr_write_multiple(
        emu, 0,
        MICBS, ADCBS_BUFSIZE_NONE,
        MICBA, 0,
        FXBS, ADCBS_BUFSIZE_NONE,
        FXBA, 0,
        ADCBS, ADCBS_BUFSIZE_NONE,
        ADCBA, 0,
        CLIEL, 0,
        CLIEH, 0,
        SOLEL, 0,
        SOLEH, 0,
        REGLIST_END,
    );

    if (*emu).audigy != 0 {
        snd_emu10k1_ptr_write(emu, SPBYPASS, 0, SPBYPASS_FORMAT);
        snd_emu10k1_ptr_write(
            emu,
            AC97SLOT,
            0,
            AC97SLOT_REAR_RIGHT | AC97SLOT_REAR_LEFT,
        );
    }

    ch = 0;
    while ch < NUM_G {
        snd_emu10k1_voice_init(emu, ch);
        ch += 1;
    }

    snd_emu10k1_ptr_write_multiple(
        emu, 0,
        SPCS0, (*emu).spdif_bits[0],
        SPCS1, (*emu).spdif_bits[1],
        SPCS2, (*emu).spdif_bits[2],
        REGLIST_END,
    );

    if (*(*emu).card_capabilities).emu_model != 0 {
    } else if (*(*emu).card_capabilities).ca0151_chip != 0 {
        snd_emu10k1_ptr_write(emu, A_I2S_CAPTURE_RATE, 0, A_I2S_CAPTURE_96000);
        snd_emu10k1_ptr20_write(emu, SRCSel, 0, 0x14);
        snd_emu10k1_ptr20_write(emu, SRCMULTI_ENABLE, 0, 0xFFFFFFFF);
        outl(0x0201, (*emu).port + HCFG2 as c_ulong);
        snd_emu10k1_ptr20_write(emu, CAPTURE_P16V_SOURCE, 0, 0x78e4);
    } else if (*(*emu).card_capabilities).ca0108_chip != 0 {
        dev_info((*(*emu).card).dev as *mut core::ffi::c_void, b"Audigy2 value: Special config.\n" as *const u8 as *const c_int);
        snd_emu10k1_ptr_write(emu, A_I2S_CAPTURE_RATE, 0, A_I2S_CAPTURE_96000);
        snd_emu10k1_ptr20_write(emu, P17V_SRCSel, 0, 0x14);
        snd_emu10k1_ptr20_write(emu, P17V_MIXER_I2S_ENABLE, 0, 0xFF000000);
        snd_emu10k1_ptr20_write(emu, P17V_MIXER_SPDIF_ENABLE, 0, 0xFF000000);
        tmp = inw((*emu).port + A_IOCFG as c_ulong) & !0x8;
        outw(tmp as u16, (*emu).port + A_IOCFG as c_ulong);
    }

    if (*(*emu).card_capabilities).spi_dac != 0 {
        let mut size: c_int = SPI_DAC_INIT.len() as c_int;
        let mut n: c_int = 0;
        while n < size {
            snd_emu10k1_spi_write(emu, SPI_DAC_INIT[n as usize]);
            n += 1;
        }
        snd_emu10k1_ptr20_write(emu, 0x60, 0, 0x10);
        outw(0x76, (*emu).port + A_IOCFG as c_ulong);
    }

    if (*(*emu).card_capabilities).i2c_adc != 0 {
        let size: c_int = I2C_ADC_INIT.len() as c_int;
        let mut n: c_int;
        snd_emu10k1_ptr20_write(emu, P17V_I2S_SRC_SEL, 0, 0x2020205f);
        tmp = inw((*emu).port + A_IOCFG as c_ulong);
        outw((tmp | 0x4) as u16, (*emu).port + A_IOCFG as c_ulong);
        tmp = inw((*emu).port + A_IOCFG as c_ulong);
        n = 0;
        while n < size {
            snd_emu10k1_i2c_write(emu, I2C_ADC_INIT[n as usize][0], I2C_ADC_INIT[n as usize][1]);
            n += 1;
        }
        n = 0;
        while n < 4 {
            (*emu).i2c_capture_volume[n as usize][0] = 0xcf;
            (*emu).i2c_capture_volume[n as usize][1] = 0xcf;
            n += 1;
        }
    }

    snd_emu10k1_ptr_write(emu, PTB, 0, (*emu).ptb_pages.addr);
    snd_emu10k1_ptr_write(emu, TCB, 0, 0);
    snd_emu10k1_ptr_write(emu, TCBS, 0, TCBS_BUFFSIZE_256K);

    silent_page = ((*emu).silent_page.addr << (*emu).address_mode)
        | (if (*emu).address_mode != 0 {
            MAP_PTI_MASK1
        } else {
            MAP_PTI_MASK0
        });
    ch = 0;
    while ch < NUM_G {
        snd_emu10k1_ptr_write(emu, MAPA, ch, silent_page);
        snd_emu10k1_ptr_write(emu, MAPB, ch, silent_page);
        ch += 1;
    }

    if (*(*emu).card_capabilities).emu_model != 0 {
        outl(
            HCFG_AUTOMUTE_ASYNC | HCFG_EMU32_SLAVE | HCFG_AUDIOENABLE,
            (*emu).port + HCFG as c_ulong,
        );
    } else if (*emu).audigy != 0 {
        if (*emu).revision == 4 {
            outl(
                HCFG_AUDIOENABLE
                    | HCFG_AC3ENABLE_CDSPDIF
                    | HCFG_AC3ENABLE_GPSPDIF
                    | HCFG_AUTOMUTE
                    | HCFG_JOYENABLE,
                (*emu).port + HCFG as c_ulong,
            );
        } else {
            outl(
                HCFG_AUTOMUTE | HCFG_JOYENABLE,
                (*emu).port + HCFG as c_ulong,
            );
        }
    } else if (*emu).model == 0x20
        || (*emu).model == 0xc400
        || ((*emu).model == 0x21 && (*emu).revision < 6)
    {
        outl(
            HCFG_LOCKTANKCACHE_MASK | HCFG_AUTOMUTE,
            (*emu).port + HCFG as c_ulong,
        );
    } else {
        outl(
            HCFG_LOCKTANKCACHE_MASK | HCFG_AUTOMUTE | HCFG_JOYENABLE,
            (*emu).port + HCFG as c_ulong,
        );
    }

    if enable_ir != 0 {
        if (*(*emu).card_capabilities).emu_model != 0 {
        } else if (*(*emu).card_capabilities).i2c_adc != 0 {
        } else if (*emu).audigy != 0 {
            let mut reg: u16 = inw((*emu).port + A_IOCFG as c_ulong);
            outw(reg | A_IOCFG_GPOUT2, (*emu).port + A_IOCFG as c_ulong);
            udelay(500);
            outw(
                reg | A_IOCFG_GPOUT1 | A_IOCFG_GPOUT2,
                (*emu).port + A_IOCFG as c_ulong,
            );
            udelay(100);
            outw(reg, (*emu).port + A_IOCFG as c_ulong);
        } else {
            let mut reg: u32 = inl((*emu).port + HCFG as c_ulong);
            outl(reg | HCFG_GPOUT2, (*emu).port + HCFG as c_ulong);
            udelay(500);
            outl(
                reg | HCFG_GPOUT1 | HCFG_GPOUT2,
                (*emu).port + HCFG as c_ulong,
            );
            udelay(100);
            outl(reg, (*emu).port + HCFG as c_ulong);
        }
    }

    if (*(*emu).card_capabilities).emu_model != 0 {
    } else if (*(*emu).card_capabilities).i2c_adc != 0 {
    } else if (*emu).audigy != 0 {
        let reg: u16 = inw((*emu).port + A_IOCFG as c_ulong);
        outw(reg | A_IOCFG_GPOUT0, (*emu).port + A_IOCFG as c_ulong);
    }

    if (*emu).address_mode == 0 {
        outl(
            inl((*emu).port + HCFG as c_ulong) | HCFG_EXPANDED_MEM,
            (*emu).port + HCFG as c_ulong,
        );
    }

    0
}

pub unsafe fn snd_emu10k1_audio_enable(emu: *mut SndEmu10k1) {
    outl(
        inl((*emu).port + HCFG as c_ulong) | HCFG_AUDIOENABLE,
        (*emu).port + HCFG as c_ulong,
    );

    if (*(*emu).card_capabilities).emu_model != 0 {
    } else if (*(*emu).card_capabilities).i2c_adc != 0 {
    } else if (*emu).audigy != 0 {
        outw(
            inw((*emu).port + A_IOCFG as c_ulong) & !0x44,
            (*emu).port + A_IOCFG as c_ulong,
        );

        if (*(*emu).card_capabilities).ca0151_chip != 0 {
            outw(
                inw((*emu).port + A_IOCFG as c_ulong) | 0x0040,
                (*emu).port + A_IOCFG as c_ulong,
            );
        } else if (*(*emu).card_capabilities).ca0108_chip != 0 {
            outw(
                inw((*emu).port + A_IOCFG as c_ulong) | 0x0060,
                (*emu).port + A_IOCFG as c_ulong,
            );
        } else {
            outw(
                inw((*emu).port + A_IOCFG as c_ulong) | 0x0080,
                (*emu).port + A_IOCFG as c_ulong,
            );
        }
    }

    if (*(*emu).card_capabilities).emu_model != 0 {
        snd_emu10k1_intr_enable(emu, INTE_PCIERRORENABLE | INTE_A_GPIOENABLE);
    } else {
        snd_emu10k1_intr_enable(emu, INTE_PCIERRORENABLE);
    }
}

pub unsafe fn snd_emu10k1_done(emu: *mut SndEmu10k1) -> c_int {
    let mut ch: c_int;

    outl(0, (*emu).port + INTE as c_ulong);

    ch = 0;
    while ch < NUM_G {
        snd_emu10k1_ptr_write_multiple(
            emu, ch,
            DCYSUSV, 0,
            VTFT, 0,
            CVCF, 0,
            PTRX, 0,
            CPF, 0,
            REGLIST_END,
        );
        ch += 1;
    }

    if (*emu).audigy != 0 {
        snd_emu10k1_ptr_write(emu, A_DBG, 0, A_DBG_SINGLE_STEP);
    } else {
        snd_emu10k1_ptr_write(emu, DBG, 0, EMU10K1_DBG_SINGLE_STEP);
    }

    snd_emu10k1_ptr_write_multiple(
        emu, 0,
        MICBS, 0,
        MICBA, 0,
        FXBS, 0,
        FXBA, 0,
        FXWC, 0,
        ADCBS, ADCBS_BUFSIZE_NONE,
        ADCBA, 0,
        TCBS, TCBS_BUFFSIZE_16K,
        TCB, 0,
        CLIEL, 0,
        CLIEH, 0,
        SOLEL, 0,
        SOLEH, 0,
        PTB, 0,
        REGLIST_END,
    );

    outl(
        HCFG_LOCKSOUNDCACHE | HCFG_LOCKTANKCACHE_MASK | HCFG_MUTEBUTTONENABLE,
        (*emu).port + HCFG as c_ulong,
    );

    0
}

const FXWC: u32 = 66;

unsafe fn snd_emu10k1_ecard_write(emu: *mut SndEmu10k1, value: u32) {
    let mut count: u16;
    let mut data: u32;
    let hc_port: c_ulong = (*emu).port + HCFG as c_ulong;
    let mut hc_value: u32 = inl(hc_port) & !(HOOKN_BIT | HANDN_BIT | PULSEN_BIT);
    outl(hc_value, hc_port);

    count = 0;
    let mut shifted_value = value;
    while count < EC_NUM_CONTROL_BITS {
        data = if shifted_value & 0x1 != 0 {
            PULSEN_BIT
        } else {
            0
        };
        shifted_value >>= 1;

        outl(hc_value | data, hc_port);
        outl(hc_value | data | HANDN_BIT, hc_port);
        outl(hc_value | data, hc_port);

        count += 1;
    }

    outl(hc_value | HOOKN_BIT, hc_port);
    outl(hc_value, hc_port);
}

unsafe fn snd_emu10k1_ecard_setadcgain(emu: *mut SndEmu10k1, gain: u16) {
    let mut bit: u32;

    snd_emu10k1_ecard_write(emu, (*emu).ecard_ctrl & !EC_TRIM_CSN);
    snd_emu10k1_ecard_write(emu, (*emu).ecard_ctrl & !EC_TRIM_CSN);

    bit = 1 << 15;
    while bit != 0 {
        let mut value: u32 = (*emu).ecard_ctrl & !(EC_TRIM_CSN | EC_TRIM_SDATA);

        if (gain as u32) & bit != 0 {
            value |= EC_TRIM_SDATA;
        }

        snd_emu10k1_ecard_write(emu, value);
        snd_emu10k1_ecard_write(emu, value | EC_TRIM_SCLK);
        snd_emu10k1_ecard_write(emu, value);

        bit >>= 1;
    }

    snd_emu10k1_ecard_write(emu, (*emu).ecard_ctrl);
}

pub unsafe fn snd_emu10k1_ecard_init(emu: *mut SndEmu10k1) -> c_int {
    let hc_value: u32;

    (*emu).ecard_ctrl = EC_RAW_RUN_MODE
        | ((EC_DEFAULT_SPDIF0_SEL << EC_SPDIF0_SEL_SHIFT) & EC_SPDIF0_SEL_MASK)
        | ((EC_DEFAULT_SPDIF1_SEL << EC_SPDIF1_SEL_SHIFT) & EC_SPDIF1_SEL_MASK);

    let hc_value_read = inl((*emu).port + HCFG as c_ulong);
    outl(
        hc_value_read | HCFG_AUDIOENABLE | HCFG_CODECFORMAT_I2S,
        (*emu).port + HCFG as c_ulong,
    );
    let _ = inl((*emu).port + HCFG as c_ulong);

    snd_emu10k1_ecard_write(emu, EC_ADCCAL | EC_LEDN | EC_TRIM_CSN);
    snd_emu10k1_ecard_write(emu, EC_DACCAL | EC_LEDN | EC_TRIM_CSN);

    snd_emu10k1_wait(emu, 48000);

    snd_emu10k1_ecard_write(emu, EC_ADCCAL | EC_LEDN | EC_TRIM_CSN);
    snd_emu10k1_ecard_write(emu, (*emu).ecard_ctrl);
    snd_emu10k1_ecard_setadcgain(emu, EC_DEFAULT_ADC_GAIN);

    0
}

pub unsafe fn snd_emu10k1_cardbus_init(emu: *mut SndEmu10k1) -> c_int {
    let special_port: c_ulong = (*emu).port + 0x38;
    let mut _value: u32;

    _value = inl(special_port);
    outl(0x00d00000, special_port);
    _value = inl(special_port);
    outl(0x00d00001, special_port);
    _value = inl(special_port);
    outl(0x00d0005f, special_port);
    _value = inl(special_port);
    outl(0x00d0007f, special_port);
    _value = inl(special_port);
    outl(0x0090007f, special_port);
    _value = inl(special_port);

    snd_emu10k1_ptr20_write(emu, TINA2_VOLUME, 0, 0xfefefefe);
    msleep(200);
    0
}

const TINA2_VOLUME: u32 = 67;

pub unsafe fn snd_emu1010_load_firmware(
    emu: *mut SndEmu10k1,
    dock: c_int,
    fw: *mut *const Firmware,
) -> c_int {
    let filename: *const u8;
    let err: c_int;

    if *fw == null() {
        let emu_model = (*(*emu).card_capabilities).emu_model as usize;
        if emu_model < FIRMWARE_NAMES.len() {
            filename = if dock != 0 {
                FIRMWARE_NAMES[emu_model][1].as_ptr() as *const u8
            } else {
                FIRMWARE_NAMES[emu_model][0].as_ptr() as *const u8
            };
            if filename as *const u8 == b"" as *const u8 {
                return 0;
            }
            err = request_firmware(fw, filename, (*(*emu).pci).dev as *mut core::ffi::c_void);
            if err != 0 {
                return err;
            }
        }
    }

    snd_emu1010_load_firmware_entry(emu, dock, *fw);
    0
}

pub unsafe fn snd_emu1010_load_dock_firmware(emu: *mut SndEmu10k1) {
    let mut tmp: u32 = 0;
    let mut tmp2: u32 = 0;
    let err: c_int;

    msleep(200);

    dev_info((*(*emu).card).dev as *mut core::ffi::c_void, b"emu1010: Loading Audio Dock Firmware\n" as *const u8 as *const c_int);
    err = snd_emu1010_load_firmware(emu, 1, &mut (*emu).dock_fw);
    if err < 0 {
        return;
    }
    snd_emu1010_fpga_write(emu, EMU_HANA_FPGA_CONFIG, 0);

    snd_emu1010_fpga_read(emu, EMU_HANA_ID, &mut tmp);
    dev_dbg((*(*emu).card).dev as *mut core::ffi::c_void, b"emu1010: EMU_HANA+DOCK_ID = 0x%x\n" as *const u8 as *const c_int, tmp);
    if (tmp & 0x1f) != 0x15 {
        dev_err(
            (*(*emu).card).dev as *mut core::ffi::c_void,
            b"emu1010: Loading Audio Dock Firmware failed, reg = 0x%x\n" as *const u8 as *const c_int,
            tmp,
        );
        return;
    }
    dev_info((*(*emu).card).dev as *mut core::ffi::c_void, b"emu1010: Audio Dock Firmware loaded\n" as *const u8 as *const c_int);

    snd_emu1010_fpga_read(emu, EMU_HANA_DOCK_MAJOR_REV, &mut tmp);
    snd_emu1010_fpga_read(emu, EMU_HANA_DOCK_MINOR_REV, &mut tmp2);
    dev_info((*(*emu).card).dev as *mut core::ffi::c_void, b"Audio Dock ver: %u.%u\n" as *const u8 as *const c_int, tmp, tmp2);

    msleep(10);
}

unsafe fn emu1010_dock_event(emu: *mut SndEmu10k1) {
    let mut reg: u32 = 0;

    snd_emu1010_fpga_read(emu, EMU_HANA_OPTION_CARDS, &mut reg);
    if reg & EMU_HANA_OPTION_DOCK_OFFLINE != 0 {
        snd_emu1010_load_dock_firmware(emu);
        snd_emu1010_fpga_write(emu, EMU_HANA_UNMUTE, EMU_UNMUTE);
    } else if reg & EMU_HANA_OPTION_DOCK_ONLINE == 0 {
        dev_info((*(*emu).card).dev as *mut core::ffi::c_void, b"emu1010: Audio Dock detached\n" as *const u8 as *const c_int);
        snd_emu1010_fpga_write(emu, EMU_HANA_UNMUTE, EMU_UNMUTE);
    }
}

unsafe fn emu1010_clock_event(emu: *mut SndEmu10k1) {
    let mut id: SndCtlElemId = core::mem::zeroed();

    {
        // scoped_guard(spinlock_irq, &emu->reg_lock)
        (*emu).emu1010.clock_source = (*emu).emu1010.clock_fallback;
        (*emu).emu1010.wclock = 1 - (*emu).emu1010.clock_source;
        snd_emu1010_update_clock(emu);
    }
    snd_ctl_build_ioff(&mut id, (*emu).ctl_clock_source, 0);
    snd_ctl_notify((*emu).card, SNDRV_CTL_EVENT_MASK_VALUE, &mut id);
}

const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 0;

unsafe fn emu1010_work(work: *mut WorkStruct) {
    let emu: *mut SndEmu10k1;
    let mut sts: u32 = 0;

    emu = container_of(work, SndEmu10k1, emu1010.work);
    if (*(*emu).card).shutdown != 0 {
        return;
    }

    {
        // guard(snd_emu1010_fpga_lock)(emu);
        snd_emu1010_fpga_read(emu, EMU_HANA_IRQ_STATUS, &mut sts);

        if sts & (EMU_HANA_IRQ_DOCK | EMU_HANA_IRQ_DOCK_LOST) != 0 {
            emu1010_dock_event(emu);
        }

        if sts & EMU_HANA_IRQ_WCLK_CHANGED != 0 {
            emu1010_clock_event(emu);
        }
    }
}

#[inline]
unsafe fn container_of<T, U>(ptr: *mut T, _member: U) -> *mut T {
    ptr
}

unsafe fn emu1010_interrupt(emu: *mut SndEmu10k1) {
    let sts: u16 = inw((*emu).port + A_IOCFG as c_ulong) as u16;
    let bit: u16 = if (*(*emu).card_capabilities).ca0108_chip != 0 {
        0x2000
    } else {
        0x8000
    };
    if sts & bit == 0 {
        return;
    }

    schedule_work(&mut (*emu).emu1010.work);
}

pub unsafe fn snd_emu10k1_emu1010_init(emu: *mut SndEmu10k1) -> c_int {
    let mut tmp: u32 = 0;
    let mut tmp2: u32 = 0;
    let mut reg: u32 = 0;
    let err: c_int;

    dev_info((*(*emu).card).dev as *mut core::ffi::c_void, b"emu1010: Special config.\n" as *const u8 as *const c_int);

    outl(
        HCFG_LOCKSOUNDCACHE | HCFG_LOCKTANKCACHE_MASK,
        (*emu).port + HCFG as c_ulong,
    );

    {
        // guard(snd_emu1010_fpga_lock)(emu);

        dev_info((*(*emu).card).dev as *mut core::ffi::c_void, b"emu1010: Loading Hana Firmware\n" as *const u8 as *const c_int);
        err = snd_emu1010_load_firmware(emu, 0, &mut (*emu).firmware);
        if err < 0 {
            dev_info((*(*emu).card).dev as *mut core::ffi::c_void, b"emu1010: Loading Firmware failed\n" as *const u8 as *const c_int);
            return err;
        }

        snd_emu1010_fpga_read(emu, EMU_HANA_ID, &mut reg);
        if (reg & 0x3f) != 0x15 {
            dev_info(
                (*(*emu).card).dev as *mut core::ffi::c_void,
                b"emu1010: Loading Hana Firmware file failed, reg = 0x%x\n" as *const u8 as *const c_int,
                reg,
            );
            return -19; // -ENODEV
        }

        dev_info((*(*emu).card).dev as *mut core::ffi::c_void, b"emu1010: Hana Firmware loaded\n" as *const u8 as *const c_int);
        snd_emu1010_fpga_read(emu, EMU_HANA_MAJOR_REV, &mut tmp);
        snd_emu1010_fpga_read(emu, EMU_HANA_MINOR_REV, &mut tmp2);
        dev_info((*(*emu).card).dev as *mut core::ffi::c_void, b"emu1010: Hana version: %u.%u\n" as *const u8 as *const c_int, tmp, tmp2);
        snd_emu1010_fpga_write(emu, EMU_HANA_DOCK_PWR, EMU_HANA_DOCK_PWR_ON);

        snd_emu1010_fpga_read(emu, EMU_HANA_OPTION_CARDS, &mut reg);
        dev_info((*(*emu).card).dev as *mut core::ffi::c_void, b"emu1010: Card options = 0x%x\n" as *const u8 as *const c_int, reg);
        if reg & EMU_HANA_OPTION_DOCK_OFFLINE != 0 {
            snd_emu1010_load_dock_firmware(emu);
        }
        if (*(*emu).card_capabilities).no_adat != 0 {
            (*emu).emu1010.optical_in = 0;
            (*emu).emu1010.optical_out = 0;
        } else {
            (*emu).emu1010.optical_in = 1;
            (*emu).emu1010.optical_out = 1;
        }
        tmp = (if (*emu).emu1010.optical_in != 0 {
            EMU_HANA_OPTICAL_IN_ADAT
        } else {
            EMU_HANA_OPTICAL_IN_SPDIF
        }) | (if (*emu).emu1010.optical_out != 0 {
            EMU_HANA_OPTICAL_OUT_ADAT
        } else {
            EMU_HANA_OPTICAL_OUT_SPDIF
        });
        snd_emu1010_fpga_write(emu, EMU_HANA_OPTICAL_TYPE, tmp);
        (*emu).emu1010.adc_pads = 0x00;
        snd_emu1010_fpga_write(emu, EMU_HANA_ADC_PADS, (*emu).emu1010.adc_pads);
        snd_emu1010_fpga_write(emu, EMU_HANA_DOCK_MISC, EMU_HANA_DOCK_PHONES_192_DAC4);
        (*emu).emu1010.dac_pads = EMU_HANA_DOCK_DAC_PAD1
            | EMU_HANA_DOCK_DAC_PAD2
            | EMU_HANA_DOCK_DAC_PAD3
            | EMU_HANA_DOCK_DAC_PAD4;
        snd_emu1010_fpga_write(emu, EMU_HANA_DAC_PADS, (*emu).emu1010.dac_pads);
        snd_emu1010_fpga_write(emu, EMU_HANA_SPDIF_MODE, EMU_HANA_SPDIF_MODE_RX_INVALID);
        snd_emu1010_fpga_write(
            emu,
            EMU_HANA_MIDI_IN,
            EMU_HANA_MIDI_INA_FROM_HAMOA | EMU_HANA_MIDI_INB_FROM_DOCK2,
        );
        snd_emu1010_fpga_write(
            emu,
            EMU_HANA_MIDI_OUT,
            EMU_HANA_MIDI_OUT_DOCK2 | EMU_HANA_MIDI_OUT_SYNC2,
        );

        (*emu).gpio_interrupt = Some(emu1010_interrupt);
        snd_emu1010_fpga_write(
            emu,
            EMU_HANA_IRQ_ENABLE,
            EMU_HANA_IRQ_DOCK | EMU_HANA_IRQ_DOCK_LOST | EMU_HANA_IRQ_WCLK_CHANGED,
        );
        snd_emu1010_fpga_read(emu, EMU_HANA_IRQ_STATUS, &mut reg);

        (*emu).emu1010.clock_source = 1;
        (*emu).emu1010.clock_fallback = 1;
        snd_emu1010_fpga_write(emu, EMU_HANA_DEFCLOCK, EMU_HANA_DEFCLOCK_48K);
        (*emu).emu1010.wclock = EMU_HANA_WCLOCK_INT_48K;
        snd_emu1010_fpga_write(emu, EMU_HANA_WCLOCK, EMU_HANA_WCLOCK_INT_48K);
        snd_emu1010_update_clock(emu);

        snd_emu1010_fpga_write(emu, EMU_HANA_UNMUTE, EMU_UNMUTE);
    }

    0
}

pub unsafe fn snd_emu10k1_free(card: *mut SndCard) {
    let emu: *mut SndEmu10k1 = (*card).private_data as *mut SndEmu10k1;

    if (*emu).port != 0 {
        snd_emu10k1_fx8010_tram_setup(emu, 0);
        snd_emu10k1_done(emu);
        snd_emu10k1_free_efx(emu);
    }
    if (*(*emu).card_capabilities).emu_model == EMU_MODEL_EMU1010 {
        snd_emu1010_fpga_write_lock(emu, EMU_HANA_DOCK_PWR, 0);
    }
    cancel_work_sync(&mut (*emu).emu1010.work);
    // mutex_destroy(&emu->emu1010.lock);
    release_firmware((*emu).firmware);
    release_firmware((*emu).dock_fw);
    snd_util_memhdr_free((*emu).memhdr);
    if !((*emu).silent_page.area).is_null() {
        snd_dma_free_pages(&mut (*emu).silent_page);
    }
    if !((*emu).ptb_pages.area).is_null() {
        snd_dma_free_pages(&mut (*emu).ptb_pages);
    }
    vfree((*emu).page_ptr_table as *mut core::ffi::c_void);
    vfree((*emu).page_addr_table as *mut core::ffi::c_void);
}

#[repr(C)]
pub struct SndEmuChipDetails {
    pub vendor: u16,
    pub device: u16,
    pub subsystem: u32,
    pub revision: u8,
    pub driver: *const u8,
    pub name: *const u8,
    pub id: *const u8,
    pub emu10k1_chip: u8,
    pub emu10k2_chip: u8,
    pub ca0102_chip: u8,
    pub ca0108_chip: u8,
    pub ca0151_chip: u8,
    pub spk71: u8,
    pub spk20: u8,
    pub spdif_bug: u8,
    pub ac97_chip: u8,
    pub ecard: u8,
    pub ca_cardbus_chip: u8,
    pub spi_dac: u8,
    pub i2c_adc: u8,
    pub adc_1361t: u8,
    pub invert_shared_spdif: u8,
    pub no_adat: u8,
    pub emu_model: u32,
}

static EMU_CHIP_DETAILS: &[SndEmuChipDetails] = &[
    SndEmuChipDetails {
        vendor: 0x1102,
        device: 0x0008,
        subsystem: 0x10241102,
        revision: 0,
        driver: b"Audigy2\0" as *const u8,
        name: b"SB Audigy 5/Rx [SB1550]\0" as *const u8,
        id: b"Audigy2\0" as *const u8,
        emu10k1_chip: 0,
        emu10k2_chip: 1,
        ca0102_chip: 0,
        ca0108_chip: 1,
        ca0151_chip: 0,
        spk71: 1,
        spk20: 0,
        spdif_bug: 0,
        ac97_chip: 1,
        ecard: 0,
        ca_cardbus_chip: 0,
        spi_dac: 0,
        i2c_adc: 0,
        adc_1361t: 1,
        invert_shared_spdif: 0,
        no_adat: 0,
        emu_model: 0,
    },
    // Additional entries would follow here - truncated for brevity
];

pub unsafe fn snd_emu10k1_detect_iommu(emu: *mut SndEmu10k1) {
    let domain: *mut core::ffi::c_void;

    (*emu).iommu_workaround = false as u8;

    domain = iommu_get_domain_for_dev((*(*emu).card).dev as *mut core::ffi::c_void);
    if domain.is_null() {
        return;
    }

    dev_notice(
        (*(*emu).card).dev as *mut core::ffi::c_void,
        b"non-passthrough IOMMU detected, widening DMA allocations" as *const u8 as *const c_int,
    );
    (*emu).iommu_workaround = true as u8;
}

pub unsafe fn snd_emu10k1_create(
    card: *mut SndCard,
    pci: *mut PciDev,
    extin_mask: u16,
    extout_mask: u16,
    max_cache_bytes: c_int,
    enable_ir: c_int,
    subsystem: u32,
) -> c_int {
    let emu: *mut SndEmu10k1 = (*card).private_data as *mut SndEmu10k1;
    let mut idx: c_int;
    let mut err: c_int;
    let is_audigy: c_int;
    let mut page_table_size: c_ulong;
    let mut pgtbl: *mut u32;
    let mut silent_page: u32;
    let mut c: *const SndEmuChipDetails;

    err = pcim_enable_device(pci);
    if err < 0 {
        return err;
    }

    (*card).private_free = Some(snd_emu10k1_free);
    (*emu).card = card;
    // spin_lock_init(&emu->reg_lock);
    // Additional initializations...
    (*emu).pci = pci;
    (*emu).irq = -1;
    (*emu).synth = null_mut();
    (*emu).get_synth_voice = None;
    // INIT_WORK(&emu->emu1010.work, emu1010_work);
    // mutex_init(&emu->emu1010.lock);

    (*emu).revision = (*pci).revision;
    pci_read_config_dword(pci, 0x2c, &mut (*emu).serial);
    pci_read_config_word(pci, 0x2e, &mut (*emu).model as *mut u16);
    dev_dbg(
        (*card).dev as *mut core::ffi::c_void,
        b"vendor = 0x%x, device = 0x%x, subsystem_vendor_id = 0x%x, subsystem_id = 0x%x\n"
            as *const u8 as *const c_int,
        (*pci).vendor,
        (*pci).device,
        (*emu).serial,
        (*emu).model,
    );

    c = EMU_CHIP_DETAILS.as_ptr();
    while (*c).vendor != 0 {
        if (*c).vendor == (*pci).vendor && (*c).device == (*pci).device {
            if subsystem != 0 {
                if (*c).subsystem != 0 && (*c).subsystem == subsystem {
                    break;
                } else {
                    c = c.add(1);
                    continue;
                }
            } else {
                if (*c).subsystem != 0 && (*c).subsystem != (*emu).serial {
                    c = c.add(1);
                    continue;
                }
                if (*c).revision != 0 && (*c).revision != (*emu).revision {
                    c = c.add(1);
                    continue;
                }
            }
            break;
        }
        c = c.add(1);
    }
    if (*c).vendor == 0 {
        dev_err((*card).dev as *mut core::ffi::c_void, b"emu10k1: Card not recognised\n" as *const u8 as *const c_int);
        return -2; // -ENOENT
    }
    (*emu).card_capabilities = c;
    if (*c).subsystem != 0 && subsystem == 0 {
        dev_dbg((*card).dev as *mut core::ffi::c_void, b"Sound card name = %s\n" as *const u8 as *const c_int, (*c).name);
    } else if subsystem != 0 {
        dev_dbg(
            (*card).dev as *mut core::ffi::c_void,
            b"Sound card name = %s, vendor = 0x%x, device = 0x%x, subsystem = 0x%x. Forced to subsystem = 0x%x\n"
                as *const u8 as *const c_int,
            (*c).name,
            (*pci).vendor,
            (*pci).device,
            (*emu).serial,
            (*c).subsystem,
        );
    } else {
        dev_dbg(
            (*card).dev as *mut core::ffi::c_void,
            b"Sound card name = %s, vendor = 0x%x, device = 0x%x, subsystem = 0x%x.\n"
                as *const u8 as *const c_int,
            (*c).name,
            (*pci).vendor,
            (*pci).device,
            (*emu).serial,
        );
    }

    if (*card).id[0] == 0 && !(*c).id.is_null() {
        strscpy(
            (*card).id.as_mut_ptr(),
            (*c).id,
            (*card).id.len() as c_ulong,
        );
    }

    is_audigy = (*c).emu10k2_chip as c_int;
    (*emu).audigy = is_audigy;

    snd_emu10k1_detect_iommu(emu);

    (*emu).address_mode = if is_audigy != 0 { 0 } else { 1 };
    (*emu).dma_mask = if (*emu).address_mode != 0 {
        EMU10K1_DMA_MASK
    } else {
        AUDIGY_DMA_MASK
    };
    if dma_set_mask_and_coherent(&mut (*pci).dev as *mut core::ffi::c_void, (*emu).dma_mask) < 0 {
        dev_err(
            (*card).dev as *mut core::ffi::c_void,
            b"architecture does not support PCI busmaster DMA with mask 0x%lx\n"
                as *const u8 as *const c_int,
            (*emu).dma_mask,
        );
        return -6; // -ENXIO
    }
    if is_audigy != 0 {
        (*emu).gpr_base = A_FXGPREGBASE;
    } else {
        (*emu).gpr_base = FXGPREGBASE;
    }

    err = pcim_request_all_regions(pci, b"EMU10K1\0" as *const u8);
    if err < 0 {
        return err;
    }
    (*emu).port = pci_resource_start(pci, 0);

    (*emu).max_cache_pages = (max_cache_bytes as c_ulong) >> PAGE_SHIFT;

    page_table_size = (4 as c_ulong) * (if (*emu).address_mode != 0 {
        MAXPAGES1 as c_ulong
    } else {
        MAXPAGES0 as c_ulong
    });
    if snd_emu10k1_alloc_pages_maybe_wider(emu, page_table_size, &mut (*emu).ptb_pages as *mut core::ffi::c_void)
        < 0
    {
        return -12; // -ENOMEM
    }
    dev_dbg(
        (*card).dev as *mut core::ffi::c_void,
        b"page table address range is %.8lx:%.8lx\n" as *const u8 as *const c_int,
        (*emu).ptb_pages.addr,
        (*emu).ptb_pages.addr + (*emu).ptb_pages.bytes,
    );

    (*emu).page_ptr_table = vmalloc(array_size(
        core::mem::size_of::<*mut core::ffi::c_void>() as c_ulong,
        (*emu).max_cache_pages as c_ulong,
    ));
    (*emu).page_addr_table = vmalloc(array_size(
        core::mem::size_of::<c_ulong>() as c_ulong,
        (*emu).max_cache_pages as c_ulong,
    ));
    if (*emu).page_ptr_table.is_null() || (*emu).page_addr_table.is_null() {
        return -12; // -ENOMEM
    }

    if snd_emu10k1_alloc_pages_maybe_wider(emu, EMUPAGESIZE, &mut (*emu).silent_page as *mut core::ffi::c_void)
        < 0
    {
        return -12; // -ENOMEM
    }
    dev_dbg(
        (*card).dev as *mut core::ffi::c_void,
        b"silent page range is %.8lx:%.8lx\n" as *const u8 as *const c_int,
        (*emu).silent_page.addr,
        (*emu).silent_page.addr + (*emu).silent_page.bytes,
    );

    (*emu).memhdr = snd_util_memhdr_new((*emu).max_cache_pages as c_ulong * EMUPAGESIZE);
    if (*emu).memhdr.is_null() {
        return -12; // -ENOMEM
    }

    pci_set_master(pci);

    let mut extin_mask_local = extin_mask;
    let mut extout_mask_local = extout_mask;
    if extin_mask_local == 0 {
        extin_mask_local = 0x3fcf;
    }
    if extout_mask_local == 0 {
        extout_mask_local = 0x7fff;
    }
    (*emu).fx8010.extin_mask = extin_mask_local;
    (*emu).fx8010.extout_mask = extout_mask_local;
    (*emu).enable_ir = enable_ir;

    if (*(*emu).card_capabilities).ca_cardbus_chip != 0 {
        err = snd_emu10k1_cardbus_init(emu);
        if err < 0 {
            return err;
        }
    }
    if (*(*emu).card_capabilities).ecard != 0 {
        err = snd_emu10k1_ecard_init(emu);
        if err < 0 {
            return err;
        }
    } else if (*(*emu).card_capabilities).emu_model != 0 {
        err = snd_emu10k1_emu1010_init(emu);
        if err < 0 {
            return err;
        }
    } else {
        snd_emu10k1_ptr_write(emu, AC97SLOT, 0, AC97SLOT_CNTR | AC97SLOT_LFE);
    }

    (*emu).fx8010.itram_size = (16 * 1024) / 2;

    if devm_request_irq(
        &mut (*pci).dev as *mut core::ffi::c_void,
        (*pci).irq,
        snd_emu10k1_interrupt,
        0x00000002,
        b"emu10k1\0" as *const u8,
        emu as *mut core::ffi::c_void,
    ) != 0
    {
        return -16; // -EBUSY
    }
    (*emu).irq = (*pci).irq as c_int;
    (*card).sync_irq = (*pci).irq;

    (*emu).spdif_bits[0] = SPCS_CLKACCY_1000PPM
        | SPCS_SAMPLERATE_48
        | SPCS_CHANNELNUM_LEFT
        | SPCS_SOURCENUM_UNSPEC
        | SPCS_GENERATIONSTATUS
        | 0x00001200
        | 0x00000000
        | SPCS_EMPHASIS_NONE
        | SPCS_COPYRIGHT;
    (*emu).spdif_bits[1] = (*emu).spdif_bits[0];
    (*emu).spdif_bits[2] = (*emu).spdif_bits[0];

    core::ptr::write_bytes((*emu).silent_page.area as *mut u8, 0, (*emu).silent_page.bytes as usize);
    silent_page = (*emu).silent_page.addr << (*emu).address_mode;
    pgtbl = (*emu).ptb_pages.area as *mut u32;
    idx = 0;
    while idx < (if (*emu).address_mode != 0 {
        MAXPAGES1
    } else {
        MAXPAGES0
    }) as c_int
    {
        *pgtbl.add(idx as usize) = (silent_page | idx as u32).to_le();
        idx += 1;
    }

    idx = 0;
    while idx < NUM_G {
        (*emu).voices[idx as usize].number = idx;
        idx += 1;
    }

    err = snd_emu10k1_init(emu, enable_ir);
    if err < 0 {
        return err;
    }

    err = snd_emu10k1_init_efx(emu);
    if err < 0 {
        return err;
    }
    snd_emu10k1_audio_enable(emu);

    snd_emu10k1_proc_init(emu);
    0
}

// External interrupt handler
extern "C" fn snd_emu10k1_interrupt(_dev: *mut core::ffi::c_void) -> c_int {
    0
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
static SAVED_REGS: &[u8] = &[
    CPF as u8, PTRX as u8, CVCF as u8, VTFT as u8, Z1 as u8, Z2 as u8, PSST as u8, DSL as u8,
    CCCA as u8, CCR as u8, 0, FXRT as u8, MAPA as u8, MAPB as u8, ENVVOL as u8, ATKHLDV as u8,
    DCYSUSV as u8, LFOVAL1 as u8, ENVVAL as u8, ATKHLDM as u8, DCYSUSM as u8, LFOVAL2 as u8,
    IP as u8, IFATN as u8, PEFE as u8, FMMOD as u8, TREMFRQ as u8, FM2FRQ2 as u8, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff,
];

#[cfg(feature = "CONFIG_PM_SLEEP")]
static SAVED_REGS_AUDIGY: &[u8] = &[0, 0, 0, 0, 0, 0, 0, 0, 0xff];

#[cfg(feature = "CONFIG_PM_SLEEP")]
pub unsafe fn snd_emu10k1_suspend_regs(emu: *mut SndEmu10k1) {
    let mut i: c_int;
    let mut reg: *const u8;
    let mut val: *mut u32;

    val = (*emu).saved_ptr as *mut u32;
    reg = SAVED_REGS.as_ptr();
    while *reg != 0xff {
        i = 0;
        while i < NUM_G {
            *val = snd_emu10k1_ptr_read(emu, *reg as u32, i);
            val = val.add(1);
            i += 1;
        }
        reg = reg.add(1);
    }
    if (*emu).audigy != 0 {
        reg = SAVED_REGS_AUDIGY.as_ptr();
        while *reg != 0xff {
            i = 0;
            while i < NUM_G {
                *val = snd_emu10k1_ptr_read(emu, *reg as u32, i);
                val = val.add(1);
                i += 1;
            }
            reg = reg.add(1);
        }
    }
    if (*emu).audigy != 0 {
        (*emu).saved_a_iocfg = inw((*emu).port + A_IOCFG as c_ulong);
    }
    (*emu).saved_hcfg = inl((*emu).port + HCFG as c_ulong);
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
pub unsafe fn snd_emu10k1_resume_init(emu: *mut SndEmu10k1) {
    if (*(*emu).card_capabilities).ca_cardbus_chip != 0 {
        snd_emu10k1_cardbus_init(emu);
    }
    if (*(*emu).card_capabilities).ecard != 0 {
        snd_emu10k1_ecard_init(emu);
    } else if (*(*emu).card_capabilities).emu_model != 0 {
        snd_emu10k1_emu1010_init(emu);
    } else {
        snd_emu10k1_ptr_write(emu, AC97SLOT, 0, AC97SLOT_CNTR | AC97SLOT_LFE);
    }
    snd_emu10k1_init(emu, (*emu).enable_ir);
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
pub unsafe fn snd_emu10k1_resume_regs(emu: *mut SndEmu10k1) {
    let mut i: c_int;
    let mut reg: *const u8;
    let mut val: *mut u32;

    snd_emu10k1_audio_enable(emu);

    if (*emu).audigy != 0 {
        outw((*emu).saved_a_iocfg, (*emu).port + A_IOCFG as c_ulong);
    }
    outl((*emu).saved_hcfg, (*emu).port + HCFG as c_ulong);

    val = (*emu).saved_ptr as *mut u32;
    reg = SAVED_REGS.as_ptr();
    while *reg != 0xff {
        i = 0;
        while i < NUM_G {
            snd_emu10k1_ptr_write(emu, *reg as u32, i, *val);
            val = val.add(1);
            i += 1;
        }
        reg = reg.add(1);
    }
    if (*emu).audigy != 0 {
        reg = SAVED_REGS_AUDIGY.as_ptr();
        while *reg != 0xff {
            i = 0;
            while i < NUM_G {
                snd_emu10k1_ptr_write(emu, *reg as u32, i, *val);
                val = val.add(1);
                i += 1;
            }
            reg = reg.add(1);
        }
    }
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
pub unsafe fn alloc_pm_buffer(emu: *mut SndEmu10k1) -> c_int {
    let mut size: c_int = SAVED_REGS.len() as c_int;

    if (*emu).audigy != 0 {
        size += SAVED_REGS_AUDIGY.len() as c_int;
    }
    (*emu).saved_ptr = vmalloc(array3_size(4, NUM_G as c_ulong, size as c_ulong));
    if (*emu).saved_ptr.is_null() {
        return -12; // -ENOMEM
    }
    if snd_emu10k1_efx_alloc_pm_buffer(emu) < 0 {
        return -12; // -ENOMEM
    }
    if (*(*emu).card_capabilities).ca0151_chip != 0 && snd_p16v_alloc_pm_buffer(emu) < 0 {
        return -12; // -ENOMEM
    }
    0
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
pub unsafe fn free_pm_buffer(emu: *mut SndEmu10k1) {
    vfree((*emu).saved_ptr as *mut core::ffi::c_void);
    snd_emu10k1_efx_free_pm_buffer(emu);
    if (*(*emu).card_capabilities).ca0151_chip != 0 {
        snd_p16v_free_pm_buffer(emu);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
