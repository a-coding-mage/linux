// SPDX-License-Identifier: GPL-2.0-only
/*
 * bebob.c - a part of driver for BeBoB based devices
 *
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

/*
 * BeBoB is 'BridgeCo enhanced Breakout Box'. This is installed to firewire
 * devices with DM1000/DM1100/DM1500 chipset. It gives common way for host
 * system to handle BeBoB based devices.
 */

// Rust translation of the implementation source. Declarations normally supplied
// by bebob.h and kernel headers are left as external dependencies.

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const MODULE_DESCRIPTION_TEXT: &[u8] = b"BridgeCo BeBoB driver\0";
const MODULE_AUTHOR_TEXT: &[u8] = b"Takashi Sakamoto <o-takashi@sakamocchi.jp>\0";
const MODULE_LICENSE_TEXT: &[u8] = b"GPL\0";

type bool_t = bool;
type u32 = u32;
type kernel_ulong_t = c_ulong;

const SNDRV_CARDS: usize = 32;
const SNDRV_DEFAULT_IDX: [c_int; SNDRV_CARDS] = [0; SNDRV_CARDS];
const SNDRV_DEFAULT_STR: [*mut c_char; SNDRV_CARDS] = [ptr::null_mut(); SNDRV_CARDS];
const SNDRV_DEFAULT_ENABLE_PNP: [bool_t; SNDRV_CARDS] = [true; SNDRV_CARDS];

static mut index: [c_int; SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
static mut id: [*mut c_char; SNDRV_CARDS] = SNDRV_DEFAULT_STR;
static mut enable: [bool_t; SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;

// module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "card index");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "enable BeBoB sound card");

static mut devices_mutex: mutex = mutex {};
static mut devices_used: [c_ulong; (SNDRV_CARDS + c_ulong::BITS as usize - 1) / c_ulong::BITS as usize] =
    [0; (SNDRV_CARDS + c_ulong::BITS as usize - 1) / c_ulong::BITS as usize];

/* Offsets from information register. */
const INFO_OFFSET_BEBOB_VERSION: u32 = 0x08;
const INFO_OFFSET_GUID: u32 = 0x10;
const INFO_OFFSET_HW_MODEL_ID: u32 = 0x18;
const INFO_OFFSET_HW_MODEL_REVISION: u32 = 0x1c;

const VEN_EDIROL: u32 = 0x000040ab;
const VEN_PRESONUS: u32 = 0x00000a92;
const VEN_BRIDGECO: u32 = 0x000007f5;
const VEN_MACKIE: u32 = 0x00000ff2;
const VEN_STANTON: u32 = 0x00001260;
const VEN_TASCAM: u32 = 0x0000022e;
const VEN_BEHRINGER: u32 = 0x00001564;
const VEN_APOGEE: u32 = 0x000003db;
const VEN_ESI: u32 = 0x00000f1b;
const VEN_CME: u32 = 0x0000000a;
const VEN_PHONIC: u32 = 0x00001496;
const VEN_LYNX: u32 = 0x000019e5;
const VEN_ICON: u32 = 0x00001a9e;
const VEN_PRISMSOUND: u32 = 0x00001198;
const VEN_TERRATEC: u32 = 0x00000aac;
const VEN_YAMAHA: u32 = 0x0000a0de;
const VEN_FOCUSRITE: u32 = 0x0000130e;
const VEN_MAUDIO: u32 = 0x00000d6c;
const VEN_DIGIDESIGN: u32 = 0x00a07e;
const OUI_SHOUYO: u32 = 0x002327;

const MODEL_FOCUSRITE_SAFFIRE_BOTH: u32 = 0x00000000;
const MODEL_MAUDIO_AUDIOPHILE_BOTH: u32 = 0x00010060;
const MODEL_MAUDIO_FW1814: u32 = 0x00010071;
const MODEL_MAUDIO_PROJECTMIX: u32 = 0x00010091;
const MODEL_MAUDIO_PROFIRELIGHTBRIDGE: u32 = 0x000100a1;

const CSR_VENDOR: c_int = 0;
const CSR_MODEL: c_int = 0;
const ENODEV: c_int = 19;
const ENOENT: c_int = 2;
const IEEE1394_MATCH_VENDOR_ID: u32 = 0x0001;
const IEEE1394_MATCH_MODEL_ID: u32 = 0x0002;
const IEEE1394_MATCH_SPECIFIER_ID: u32 = 0x0004;
const SND_BEBOB_QUIRK_INITIAL_DISCONTINUOUS_DBC: u32 = 0x00000001;
const SND_BEBOB_QUIRK_WRONG_DBC: u32 = 0x00000002;
const SPECIFIER_1394TA: u32 = 0x00a02d;

#[repr(C)]
pub struct mutex {}
#[repr(C)]
pub struct spinlock_t {}
#[repr(C)]
pub struct wait_queue_head_t {}
#[repr(C)]
pub struct module {}
#[repr(C)]
pub struct bus_type {}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct fw_card {
    _private: [u8; 0],
}
#[repr(C)]
pub struct fw_device {
    pub config_rom: *mut u32,
    pub max_speed: c_int,
    pub card: *mut fw_card,
}
#[repr(C)]
pub struct fw_unit {
    pub device: device,
    pub directory: *mut u32,
}
#[repr(C)]
pub struct snd_card {
    pub driver: [c_char; 16],
    pub shortname: [c_char; 32],
    pub mixername: [c_char; 80],
    pub longname: [c_char; 80],
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_card)>,
}
#[repr(C)]
pub struct snd_bebob_rate_spec {
    pub get: Option<unsafe extern "C" fn(*mut snd_bebob, *mut u32) -> c_int>,
    pub set: Option<unsafe extern "C" fn(*mut snd_bebob, u32) -> c_int>,
}
#[repr(C)]
pub struct snd_bebob_spec {
    pub clock: *const c_void,
    pub rate: *const snd_bebob_rate_spec,
    pub meter: *const c_void,
}
#[repr(C)]
pub struct snd_bebob {
    pub unit: *mut fw_unit,
    pub card: *mut snd_card,
    pub card_index: u32,
    pub spec: *const snd_bebob_spec,
    pub mutex: mutex,
    pub lock: spinlock_t,
    pub hwdep_wait: wait_queue_head_t,
    pub quirks: u32,
    pub midi_input_ports: u32,
    pub midi_output_ports: u32,
}
#[repr(C)]
pub struct ieee1394_device_id {
    pub match_flags: u32,
    pub vendor_id: u32,
    pub model_id: u32,
    pub specifier_id: u32,
    pub version: u32,
    pub driver_data: kernel_ulong_t,
}
#[repr(C)]
pub struct driver {
    pub owner: *mut module,
    pub name: *const c_char,
    pub bus: *mut bus_type,
}
#[repr(C)]
pub struct fw_driver {
    pub driver: driver,
    pub probe: Option<unsafe extern "C" fn(*mut fw_unit, *const ieee1394_device_id) -> c_int>,
    pub update: Option<unsafe extern "C" fn(*mut fw_unit)>,
    pub remove: Option<unsafe extern "C" fn(*mut fw_unit)>,
    pub id_table: *const ieee1394_device_id,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static mut fw_bus_type: bus_type;
    static KBUILD_MODNAME: [c_char; 0];

    static saffire_le_spec: snd_bebob_spec;
    static saffire_spec: snd_bebob_spec;
    static maudio_special_spec: snd_bebob_spec;
    static phase88_rack_spec: snd_bebob_spec;
    static yamaha_terratec_spec: snd_bebob_spec;
    static saffirepro_26_spec: snd_bebob_spec;
    static saffirepro_10_spec: snd_bebob_spec;
    static maudio_fw410_spec: snd_bebob_spec;
    static maudio_audiophile_spec: snd_bebob_spec;
    static maudio_solo_spec: snd_bebob_spec;
    static maudio_ozonic_spec: snd_bebob_spec;
    static maudio_nrv10_spec: snd_bebob_spec;

    fn fw_parent_device(unit: *mut fw_unit) -> *mut fw_device;
    fn fw_csr_string(directory: *mut u32, key: c_int, buf: *mut c_char, size: usize) -> c_int;
    fn snd_bebob_read_quad(unit: *mut fw_unit, offset: u32, value: *mut u32) -> c_int;
    fn snd_bebob_read_block(unit: *mut fw_unit, offset: u32, data: *mut u32, size: usize) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn dev_name(dev: *const device) -> *const c_char;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn clear_bit(nr: u32, addr: *mut c_ulong);
    fn set_bit(nr: u32, addr: *mut c_ulong);
    fn test_bit(nr: u32, addr: *const c_ulong) -> bool_t;
    fn snd_bebob_stream_destroy_duplex(bebob: *mut snd_bebob);
    fn mutex_destroy(lock: *mut mutex);
    fn fw_unit_put(unit: *mut fw_unit);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn snd_bebob_maudio_load_firmware(unit: *mut fw_unit) -> c_int;
    fn snd_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut module,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn fw_unit_get(unit: *mut fw_unit) -> *mut fw_unit;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn mutex_init(lock: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn init_waitqueue_head(head: *mut wait_queue_head_t);
    fn snd_bebob_maudio_special_discover(bebob: *mut snd_bebob, is1814: bool_t) -> c_int;
    fn snd_bebob_stream_discover(bebob: *mut snd_bebob) -> c_int;
    fn snd_bebob_stream_init_duplex(bebob: *mut snd_bebob) -> c_int;
    fn snd_bebob_proc_init(bebob: *mut snd_bebob);
    fn snd_bebob_create_midi_devices(bebob: *mut snd_bebob) -> c_int;
    fn snd_bebob_create_pcm_devices(bebob: *mut snd_bebob) -> c_int;
    fn snd_bebob_create_hwdep_device(bebob: *mut snd_bebob) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn fw_schedule_bus_reset(card: *mut fw_card, delayed: bool_t, short_reset: bool_t);
    fn snd_card_free(card: *mut snd_card);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn fcp_bus_reset(unit: *mut fw_unit);
    fn snd_bebob_stream_get_rate(bebob: *mut snd_bebob, rate: *mut u32) -> c_int;
    fn snd_bebob_stream_set_rate(bebob: *mut snd_bebob, rate: u32) -> c_int;
    fn driver_register(driver: *mut driver) -> c_int;
    fn driver_unregister(driver: *mut driver);
}

unsafe fn name_device(bebob: *mut snd_bebob) -> c_int {
    let fw_dev = fw_parent_device((*bebob).unit);
    let mut vendor: [c_char; 24] = [0; 24];
    let mut model: [c_char; 32] = [0; 32];
    let mut hw_id: u32 = 0;
    let mut data: [u32; 2] = [0; 2];
    let mut revision: u32 = 0;
    let mut err: c_int;

    /* get vendor name from root directory */
    err = fw_csr_string((*fw_dev).config_rom.add(5), CSR_VENDOR, vendor.as_mut_ptr(), vendor.len());
    if err < 0 {
        return err;
    }

    /* get model name from unit directory */
    err = fw_csr_string((*(*bebob).unit).directory, CSR_MODEL, model.as_mut_ptr(), model.len());
    if err < 0 {
        return err;
    }

    /* get hardware id */
    err = snd_bebob_read_quad((*bebob).unit, INFO_OFFSET_HW_MODEL_ID, &mut hw_id);
    if err < 0 {
        return err;
    }

    /* get hardware revision */
    err = snd_bebob_read_quad((*bebob).unit, INFO_OFFSET_HW_MODEL_REVISION, &mut revision);
    if err < 0 {
        return err;
    }

    /* get GUID */
    err = snd_bebob_read_block((*bebob).unit, INFO_OFFSET_GUID, data.as_mut_ptr(), size_of::<[u32; 2]>());
    if err < 0 {
        return err;
    }

    strscpy((*(*bebob).card).driver.as_mut_ptr(), b"BeBoB\0".as_ptr() as *const c_char);
    strscpy((*(*bebob).card).shortname.as_mut_ptr(), model.as_ptr());
    strscpy((*(*bebob).card).mixername.as_mut_ptr(), model.as_ptr());
    snprintf(
        (*(*bebob).card).longname.as_mut_ptr(),
        (*(*bebob).card).longname.len(),
        b"%s %s (id:%d, rev:%d), GUID %08x%08x at %s, S%d\0".as_ptr() as *const c_char,
        vendor.as_ptr(),
        model.as_ptr(),
        hw_id,
        revision,
        data[0],
        data[1],
        dev_name(&mut (*(*bebob).unit).device),
        100 << (*fw_dev).max_speed,
    );

    err
}

unsafe extern "C" fn bebob_card_free(card: *mut snd_card) {
    let bebob = (*card).private_data as *mut snd_bebob;

    mutex_lock(&mut devices_mutex);
    clear_bit((*bebob).card_index, devices_used.as_mut_ptr());
    mutex_unlock(&mut devices_mutex);

    snd_bebob_stream_destroy_duplex(bebob);

    mutex_destroy(&mut (*bebob).mutex);
    fw_unit_put((*bebob).unit);
}

unsafe fn get_saffire_spec(unit: *mut fw_unit) -> *const snd_bebob_spec {
    let mut name: [c_char; 24] = [0; 24];

    if fw_csr_string((*unit).directory, CSR_MODEL, name.as_mut_ptr(), name.len()) < 0 {
        return ptr::null();
    }

    if strcmp(name.as_ptr(), b"SaffireLE\0".as_ptr() as *const c_char) == 0 {
        &saffire_le_spec
    } else {
        &saffire_spec
    }
}

unsafe fn check_audiophile_booted(unit: *mut fw_unit) -> bool_t {
    let mut name: [c_char; 28] = [0; 28];

    if fw_csr_string((*unit).directory, CSR_MODEL, name.as_mut_ptr(), name.len()) < 0 {
        return false;
    }

    strncmp(
        name.as_ptr(),
        b"FW Audiophile Bootloader\0".as_ptr() as *const c_char,
        24,
    ) != 0
}

unsafe fn detect_quirks(bebob: *mut snd_bebob, entry: *const ieee1394_device_id) -> c_int {
    if (*entry).vendor_id == VEN_MAUDIO {
        match (*entry).model_id {
            MODEL_MAUDIO_PROFIRELIGHTBRIDGE => {
                // M-Audio ProFire Lightbridge has a quirk to transfer packets with
                // discontinuous cycle or data block counter in early stage of packet
                // streaming. The cycle span from the first packet with event is variable.
                (*bebob).quirks |= SND_BEBOB_QUIRK_INITIAL_DISCONTINUOUS_DBC;
            }
            MODEL_MAUDIO_FW1814 | MODEL_MAUDIO_PROJECTMIX => {
                // At high sampling rate, M-Audio special firmware transmits empty packet
                // with the value of dbc incremented by 8.
                (*bebob).quirks |= SND_BEBOB_QUIRK_WRONG_DBC;
            }
            _ => {}
        }
    }

    0
}

unsafe extern "C" fn bebob_probe(unit: *mut fw_unit, entry: *const ieee1394_device_id) -> c_int {
    let mut card_index: u32;
    let mut card: *mut snd_card = ptr::null_mut();
    let bebob: *mut snd_bebob;
    let spec: *const snd_bebob_spec;
    let mut err: c_int;

    if (*entry).vendor_id == VEN_FOCUSRITE && (*entry).model_id == MODEL_FOCUSRITE_SAFFIRE_BOTH {
        spec = get_saffire_spec(unit);
    } else if (*entry).vendor_id == VEN_MAUDIO
        && (*entry).model_id == MODEL_MAUDIO_AUDIOPHILE_BOTH
        && !check_audiophile_booted(unit)
    {
        spec = ptr::null();
    } else {
        spec = (*entry).driver_data as *const snd_bebob_spec;
    }

    if spec.is_null() {
        // To boot up M-Audio models.
        if (*entry).vendor_id == VEN_MAUDIO || (*entry).vendor_id == VEN_BRIDGECO {
            return snd_bebob_maudio_load_firmware(unit);
        } else {
            return -ENODEV;
        }
    }

    mutex_lock(&mut devices_mutex);
    card_index = 0;
    while (card_index as usize) < SNDRV_CARDS {
        if !test_bit(card_index, devices_used.as_ptr()) && enable[card_index as usize] {
            break;
        }
        card_index += 1;
    }
    if (card_index as usize) >= SNDRV_CARDS {
        mutex_unlock(&mut devices_mutex);
        return -ENOENT;
    }

    err = snd_card_new(
        &mut (*unit).device,
        index[card_index as usize],
        id[card_index as usize],
        THIS_MODULE,
        size_of::<snd_bebob>(),
        &mut card,
    );
    if err < 0 {
        mutex_unlock(&mut devices_mutex);
        return err;
    }
    (*card).private_free = Some(bebob_card_free);
    set_bit(card_index, devices_used.as_mut_ptr());
    mutex_unlock(&mut devices_mutex);

    bebob = (*card).private_data as *mut snd_bebob;
    (*bebob).unit = fw_unit_get(unit);
    dev_set_drvdata(&mut (*unit).device, bebob as *mut c_void);
    (*bebob).card = card;
    (*bebob).card_index = card_index;

    (*bebob).spec = spec;
    mutex_init(&mut (*bebob).mutex);
    spin_lock_init(&mut (*bebob).lock);
    init_waitqueue_head(&mut (*bebob).hwdep_wait);

    err = name_device(bebob);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    err = detect_quirks(bebob, entry);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    if (*bebob).spec == (&maudio_special_spec as *const snd_bebob_spec) {
        if (*entry).model_id == MODEL_MAUDIO_FW1814 {
            err = snd_bebob_maudio_special_discover(bebob, true);
        } else {
            err = snd_bebob_maudio_special_discover(bebob, false);
        }
    } else {
        err = snd_bebob_stream_discover(bebob);
    }
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    err = snd_bebob_stream_init_duplex(bebob);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    snd_bebob_proc_init(bebob);

    if (*bebob).midi_input_ports > 0 || (*bebob).midi_output_ports > 0 {
        err = snd_bebob_create_midi_devices(bebob);
        if err < 0 {
            snd_card_free(card);
            return err;
        }
    }

    err = snd_bebob_create_pcm_devices(bebob);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    err = snd_bebob_create_hwdep_device(bebob);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    err = snd_card_register(card);
    if err < 0 {
        snd_card_free(card);
        return err;
    }

    if (*entry).vendor_id == VEN_MAUDIO
        && ((*entry).model_id == MODEL_MAUDIO_FW1814 || (*entry).model_id == MODEL_MAUDIO_PROJECTMIX)
    {
        // This is a workaround. This bus reset seems to have an effect to make devices
        // correctly handling transactions. Without this, the devices have gap_count
        // mismatch. This causes much failure of transaction.
        //
        // Just after registration, user-land application receive signals from dbus and
        // starts I/Os. To avoid I/Os till the future bus reset, registration is done in
        // next update().
        fw_schedule_bus_reset((*fw_parent_device((*bebob).unit)).card, false, true);
    }

    0
}

/*
 * This driver doesn't update streams in bus reset handler.
 *
 * DM1000/ DM1100/DM1500 chipsets with BeBoB firmware transfer packets with
 * discontinued counter at bus reset. This discontinuity is immediately
 * detected in packet streaming layer, then it sets XRUN to PCM substream.
 *
 * ALSA PCM applications can know the XRUN by getting -EPIPE from PCM operation.
 * Then, they can recover the PCM substream by executing ioctl(2) with
 * SNDRV_PCM_IOCTL_PREPARE. 'struct snd_pcm_ops.prepare' is called and drivers
 * restart packet streaming.
 *
 * The above processing may be executed before this bus-reset handler is
 * executed. When this handler updates streams with current isochronous
 * channels, the streams already have the current ones.
 */
unsafe extern "C" fn bebob_update(unit: *mut fw_unit) {
    let bebob = dev_get_drvdata(&mut (*unit).device) as *mut snd_bebob;

    if bebob.is_null() {
        return;
    }

    fcp_bus_reset((*bebob).unit);
}

unsafe extern "C" fn bebob_remove(unit: *mut fw_unit) {
    let bebob = dev_get_drvdata(&mut (*unit).device) as *mut snd_bebob;

    if bebob.is_null() {
        return;
    }

    // Block till all of ALSA character devices are released.
    snd_card_free((*bebob).card);
}

static normal_rate_spec: snd_bebob_rate_spec = snd_bebob_rate_spec {
    get: Some(snd_bebob_stream_get_rate),
    set: Some(snd_bebob_stream_set_rate),
};
static spec_normal: snd_bebob_spec = snd_bebob_spec {
    clock: ptr::null(),
    rate: &normal_rate_spec,
    meter: ptr::null(),
};

const fn snd_bebob_dev_entry(
    vendor: u32,
    model: u32,
    data: *const snd_bebob_spec,
) -> ieee1394_device_id {
    ieee1394_device_id {
        match_flags: IEEE1394_MATCH_VENDOR_ID | IEEE1394_MATCH_MODEL_ID | IEEE1394_MATCH_SPECIFIER_ID,
        vendor_id: vendor,
        model_id: model,
        specifier_id: SPECIFIER_1394TA,
        version: 0,
        driver_data: data as kernel_ulong_t,
    }
}

static bebob_id_table: [ieee1394_device_id; 67] = [
    /* Edirol, FA-66 */
    snd_bebob_dev_entry(VEN_EDIROL, 0x00010049, &spec_normal),
    /* Edirol, FA-101 */
    snd_bebob_dev_entry(VEN_EDIROL, 0x00010048, &spec_normal),
    /* Presonus, FIREBOX */
    snd_bebob_dev_entry(VEN_PRESONUS, 0x00010000, &spec_normal),
    /* PreSonus, FIREPOD/FP10 */
    snd_bebob_dev_entry(VEN_PRESONUS, 0x00010066, &spec_normal),
    /* PreSonus, Inspire1394 */
    snd_bebob_dev_entry(VEN_PRESONUS, 0x00010001, &spec_normal),
    /* BridgeCo, RDAudio1 */
    snd_bebob_dev_entry(VEN_BRIDGECO, 0x00010048, &spec_normal),
    /* BridgeCo, Audio5 */
    snd_bebob_dev_entry(VEN_BRIDGECO, 0x00010049, &spec_normal),
    /* Mackie, Onyx 1220/1620/1640 (Firewire I/O Card) */
    snd_bebob_dev_entry(VEN_MACKIE, 0x00010065, &spec_normal),
    // Mackie, d.2 (optional Firewire card with DM1000).
    snd_bebob_dev_entry(VEN_MACKIE, 0x00010067, &spec_normal),
    /* Stanton, ScratchAmp */
    snd_bebob_dev_entry(VEN_STANTON, 0x00000001, &spec_normal),
    /* Tascam, IF-FW DM */
    snd_bebob_dev_entry(VEN_TASCAM, 0x00010067, &spec_normal),
    /* Behringer, XENIX UFX 1204 */
    snd_bebob_dev_entry(VEN_BEHRINGER, 0x00001204, &spec_normal),
    /* Behringer, XENIX UFX 1604 */
    snd_bebob_dev_entry(VEN_BEHRINGER, 0x00001604, &spec_normal),
    /* Behringer, Digital Mixer X32 series (X-UF Card) */
    snd_bebob_dev_entry(VEN_BEHRINGER, 0x00000006, &spec_normal),
    /*  Behringer, F-Control Audio 1616 */
    snd_bebob_dev_entry(VEN_BEHRINGER, 0x001616, &spec_normal),
    /*  Behringer, F-Control Audio 610 */
    snd_bebob_dev_entry(VEN_BEHRINGER, 0x000610, &spec_normal),
    /* Apogee Electronics, Rosetta 200/400 (X-FireWire card) */
    /* Apogee Electronics, DA/AD/DD-16X (X-FireWire card) */
    snd_bebob_dev_entry(VEN_APOGEE, 0x00010048, &spec_normal),
    /* Apogee Electronics, Ensemble */
    snd_bebob_dev_entry(VEN_APOGEE, 0x01eeee, &spec_normal),
    /* ESI, Quatafire610 */
    snd_bebob_dev_entry(VEN_ESI, 0x00010064, &spec_normal),
    /* CME, MatrixKFW */
    snd_bebob_dev_entry(VEN_CME, 0x00030000, &spec_normal),
    // Phonic Helix Board 12 FireWire MkII.
    snd_bebob_dev_entry(VEN_PHONIC, 0x00050000, &spec_normal),
    // Phonic Helix Board 18 FireWire MkII.
    snd_bebob_dev_entry(VEN_PHONIC, 0x00060000, &spec_normal),
    // Phonic Helix Board 24 FireWire MkII.
    snd_bebob_dev_entry(VEN_PHONIC, 0x00070000, &spec_normal),
    // Phonic FireFly 808 FireWire.
    snd_bebob_dev_entry(VEN_PHONIC, 0x00080000, &spec_normal),
    // Phonic FireFly 202, 302, 808 Universal.
    // Phinic Helix Board 12/18/24 FireWire, 12/18/24 Universal
    snd_bebob_dev_entry(VEN_PHONIC, 0x00000000, &spec_normal),
    /* Lynx, Aurora 8/16 (LT-FW) */
    snd_bebob_dev_entry(VEN_LYNX, 0x00000001, &spec_normal),
    /* ICON, FireXon */
    snd_bebob_dev_entry(VEN_ICON, 0x00000001, &spec_normal),
    /* PrismSound, Orpheus */
    snd_bebob_dev_entry(VEN_PRISMSOUND, 0x00010048, &spec_normal),
    /* PrismSound, ADA-8XR */
    snd_bebob_dev_entry(VEN_PRISMSOUND, 0x0000ada8, &spec_normal),
    /* TerraTec Electronic GmbH, PHASE 88 Rack FW */
    snd_bebob_dev_entry(VEN_TERRATEC, 0x00000003, &phase88_rack_spec),
    /* TerraTec Electronic GmbH, PHASE 24 FW */
    snd_bebob_dev_entry(VEN_TERRATEC, 0x00000004, &yamaha_terratec_spec),
    /* TerraTec Electronic GmbH, Phase X24 FW */
    snd_bebob_dev_entry(VEN_TERRATEC, 0x00000007, &yamaha_terratec_spec),
    /* TerraTec Electronic GmbH, EWS MIC2/MIC8 */
    snd_bebob_dev_entry(VEN_TERRATEC, 0x00000005, &spec_normal),
    // Terratec Electronic GmbH, Aureon 7.1 Firewire.
    // AcousticReality, eAR Master One, Eroica, Figaro, and Ciaccona. Perhaps Terratec OEM.
    snd_bebob_dev_entry(VEN_TERRATEC, 0x00000002, &spec_normal),
    /* Yamaha, GO44 */
    snd_bebob_dev_entry(VEN_YAMAHA, 0x0010000b, &yamaha_terratec_spec),
    /* YAMAHA, GO46 */
    snd_bebob_dev_entry(VEN_YAMAHA, 0x0010000c, &yamaha_terratec_spec),
    /* Focusrite, SaffirePro 26 I/O */
    snd_bebob_dev_entry(VEN_FOCUSRITE, 0x00000003, &saffirepro_26_spec),
    /* Focusrite, SaffirePro 10 I/O */
    snd_bebob_dev_entry(VEN_FOCUSRITE, 0x000006, &saffirepro_10_spec),
    /* Focusrite, Saffire(no label and LE) */
    snd_bebob_dev_entry(VEN_FOCUSRITE, MODEL_FOCUSRITE_SAFFIRE_BOTH, &saffire_spec),
    // M-Audio, Firewire 410. The vendor field is left as BridgeCo. AG.
    snd_bebob_dev_entry(VEN_BRIDGECO, 0x00010058, ptr::null()),
    snd_bebob_dev_entry(VEN_BRIDGECO, 0x00010046, &maudio_fw410_spec),
    /* M-Audio, Firewire Audiophile */
    snd_bebob_dev_entry(VEN_MAUDIO, MODEL_MAUDIO_AUDIOPHILE_BOTH, &maudio_audiophile_spec),
    /* M-Audio, Firewire Solo */
    snd_bebob_dev_entry(VEN_MAUDIO, 0x00010062, &maudio_solo_spec),
    /* M-Audio, Ozonic */
    snd_bebob_dev_entry(VEN_MAUDIO, 0x0000000a, &maudio_ozonic_spec),
    /* M-Audio NRV10 */
    snd_bebob_dev_entry(VEN_MAUDIO, 0x00010081, &maudio_nrv10_spec),
    /* M-Audio, ProFireLightbridge */
    snd_bebob_dev_entry(VEN_MAUDIO, MODEL_MAUDIO_PROFIRELIGHTBRIDGE, &spec_normal),
    /* Firewire 1814 */
    snd_bebob_dev_entry(VEN_MAUDIO, 0x00010070, ptr::null()), /* bootloader */
    snd_bebob_dev_entry(VEN_MAUDIO, MODEL_MAUDIO_FW1814, &maudio_special_spec),
    /* M-Audio ProjectMix */
    snd_bebob_dev_entry(VEN_MAUDIO, MODEL_MAUDIO_PROJECTMIX, &maudio_special_spec),
    /* Digidesign Mbox 2 Pro */
    snd_bebob_dev_entry(VEN_DIGIDESIGN, 0x0000a9, &spec_normal),
    // Toneweal FW66.
    snd_bebob_dev_entry(OUI_SHOUYO, 0x020002, &spec_normal),
    /* IDs are unknown but able to be supported */
    /*  Apogee, Mini-ME Firewire */
    /*  Apogee, Mini-DAC Firewire */
    /*  Cakawalk, Sonar Power Studio 66 */
    /*  CME, UF400e */
    /*  ESI, Quotafire XL */
    /*  Infrasonic, DewX */
    /*  Infrasonic, Windy6 */
    /*  Mackie, Digital X Bus x.200 */
    /*  Mackie, Digital X Bus x.400 */
    /*  Rolf Spuler, Firewire Guitar */
    ieee1394_device_id {
        match_flags: 0,
        vendor_id: 0,
        model_id: 0,
        specifier_id: 0,
        version: 0,
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(ieee1394, bebob_id_table);

static mut bebob_driver: fw_driver = fw_driver {
    driver: driver {
        owner: ptr::null_mut(),
        name: ptr::null(),
        bus: ptr::null_mut(),
    },
    probe: Some(bebob_probe),
    update: Some(bebob_update),
    remove: Some(bebob_remove),
    id_table: bebob_id_table.as_ptr(),
};

unsafe fn snd_bebob_init() -> c_int {
    bebob_driver.driver.owner = THIS_MODULE;
    bebob_driver.driver.name = KBUILD_MODNAME.as_ptr();
    bebob_driver.driver.bus = &mut fw_bus_type;
    driver_register(&mut bebob_driver.driver)
}

unsafe fn snd_bebob_exit() {
    driver_unregister(&mut bebob_driver.driver);
}

// module_init(snd_bebob_init);
// module_exit(snd_bebob_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
