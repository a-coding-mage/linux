/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from snd_wavefront.h. C include dependencies are supplied externally. */

/* MIDI interface */

pub struct _snd_wavefront_midi;
pub struct _snd_wavefront_card;
pub struct _snd_wavefront;
pub struct snd_wss;

pub type snd_wavefront_midi_t = _snd_wavefront_midi;
pub type snd_wavefront_card_t = _snd_wavefront_card;
pub type snd_wavefront_t = _snd_wavefront;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum snd_wavefront_mpu_id {
    internal_mpu = 0,
    external_mpu = 1,
}

#[repr(C)]
pub struct _snd_wavefront_midi {
    pub base: ::core::ffi::c_ulong, /* I/O port address */
    pub isvirtual: ::core::ffi::c_char, /* doing virtual MIDI stuff ? */
    pub istimer: ::core::ffi::c_char, /* timer is used */
    pub output_mpu: snd_wavefront_mpu_id, /* most-recently-used */
    pub input_mpu: snd_wavefront_mpu_id, /* most-recently-used */
    pub mode: [::core::ffi::c_uint; 2], /* MPU401_MODE_XXX */
    pub substream_output: [*mut snd_rawmidi_substream; 2],
    pub substream_input: [*mut snd_rawmidi_substream; 2],
    pub timer: timer_list,
    pub timer_card: *mut snd_wavefront_card_t,
    pub open: spinlock_t,
    pub virtual_: spinlock_t, /* protects isvirtual */
}

pub const OUTPUT_READY: u32 = 0x40;
pub const INPUT_AVAIL: u32 = 0x80;
pub const MPU_ACK: u32 = 0xfe;
pub const UART_MODE_ON: u32 = 0x3f;

extern "C" {
    pub static snd_wavefront_midi_output: snd_rawmidi_ops;
    pub static snd_wavefront_midi_input: snd_rawmidi_ops;

    pub fn snd_wavefront_midi_enable_virtual(card: *mut snd_wavefront_card_t);
    pub fn snd_wavefront_midi_disable_virtual(card: *mut snd_wavefront_card_t);
    pub fn snd_wavefront_midi_interrupt(card: *mut snd_wavefront_card_t);
    pub fn snd_wavefront_midi_start(card: *mut snd_wavefront_card_t) -> ::core::ffi::c_int;
    pub fn snd_wavefront_midi_suspend(card: *mut snd_wavefront_card_t);
    pub fn snd_wavefront_midi_resume(card: *mut snd_wavefront_card_t);
}

#[repr(C)]
pub struct _snd_wavefront {
    pub irq: ::core::ffi::c_ulong, /* "you were one, one of the few ..." */
    pub base: ::core::ffi::c_ulong, /* low i/o port address */
    pub res_base: *mut resource, /* i/o port resource allocation */

    pub irq_ok: ::core::ffi::c_int, /* set by interrupt handler */
    pub irq_cnt: ::core::ffi::c_int, /* ditto */
    pub debug: ::core::ffi::c_char, /* debugging flags */
    pub freemem: ::core::ffi::c_int, /* installed RAM, in bytes */
    pub fw_version: [::core::ffi::c_char; 2], /* major = [0], minor = [1] */
    pub hw_version: [::core::ffi::c_char; 2], /* major = [0], minor = [1] */
    pub israw: ::core::ffi::c_char, /* needs Motorola microcode */
    pub has_fx: ::core::ffi::c_char, /* has FX processor (Tropez+) */
    pub fx_initialized: ::core::ffi::c_char, /* FX's register pages initialized */
    pub prog_status: [::core::ffi::c_char; WF_MAX_PROGRAM], /* WF_SLOT_* */
    pub patch_status: [::core::ffi::c_char; WF_MAX_PATCH], /* WF_SLOT_* */
    pub sample_status: [::core::ffi::c_char; WF_MAX_SAMPLE], /* WF_ST_* | WF_SLOT_* */
    pub samples_used: ::core::ffi::c_int, /* how many */
    pub interrupts_are_midi: ::core::ffi::c_char, /* h/w MPU interrupts enabled ? */
    pub rom_samples_rdonly: ::core::ffi::c_char, /* can we write on ROM samples */
    pub midi_in_to_synth: ::core::ffi::c_char, /* route external MIDI to synth */
    pub irq_lock: spinlock_t,
    pub interrupt_sleeper: wait_queue_head_t,
    pub midi: snd_wavefront_midi_t, /* ICS2115 MIDI interface */
    pub card: *mut snd_card,
}

/* Port macros, expressed as Rust macros to preserve their base-relative behavior. */
macro_rules! mpu_data_port { ($base:expr) => { $base }; }
macro_rules! mpu_command_port { ($base:expr) => { $base + 1 }; }
macro_rules! mpu_status_port { ($base:expr) => { $base + 1 }; }
macro_rules! data_port { ($base:expr) => { $base + 2 }; }
macro_rules! status_port { ($base:expr) => { $base + 3 }; }
macro_rules! control_port { ($base:expr) => { $base + 3 }; }
macro_rules! block_port { ($base:expr) => { $base + 4 }; }
macro_rules! last_block_port { ($base:expr) => { $base + 6 }; }
macro_rules! fx_status { ($base:expr) => { $base + 8 }; }
macro_rules! fx_op { ($base:expr) => { $base + 8 }; }
macro_rules! fx_lcr { ($base:expr) => { $base + 9 }; }
macro_rules! fx_dsp_addr { ($base:expr) => { $base + 0xa }; }
macro_rules! fx_dsp_page { ($base:expr) => { $base + 0xb }; }
macro_rules! fx_dsp_lsb { ($base:expr) => { $base + 0xc }; }
macro_rules! fx_dsp_msb { ($base:expr) => { $base + 0xd }; }
macro_rules! fx_mod_addr { ($base:expr) => { $base + 0xe }; }
macro_rules! fx_mod_data { ($base:expr) => { $base + 0xf }; }

#[repr(C)]
pub struct _snd_wavefront_card {
    pub wavefront: snd_wavefront_t,
    pub chip: *mut snd_wss,
    #[cfg(CONFIG_PNP)]
    pub wss: *mut pnp_dev,
    #[cfg(CONFIG_PNP)]
    pub ctrl: *mut pnp_dev,
    #[cfg(CONFIG_PNP)]
    pub mpu: *mut pnp_dev,
    #[cfg(CONFIG_PNP)]
    pub synth: *mut pnp_dev,
}

extern "C" {
    pub fn snd_wavefront_internal_interrupt(card: *mut snd_wavefront_card_t);
    pub fn snd_wavefront_cache_firmware(dev: *mut snd_wavefront_t);
    pub fn snd_wavefront_start(dev: *mut snd_wavefront_t) -> ::core::ffi::c_int;
    pub fn snd_wavefront_detect(card: *mut snd_wavefront_card_t) -> ::core::ffi::c_int;
    pub fn snd_wavefront_resume_synth(card: *mut snd_wavefront_card_t) -> ::core::ffi::c_int;
    pub fn snd_wavefront_cmd(
        dev: *mut snd_wavefront_t,
        command: ::core::ffi::c_int,
        cmd: *mut u8,
        response: *mut u8,
    ) -> ::core::ffi::c_int;

    pub fn snd_wavefront_synth_ioctl(
        hw: *mut snd_hwdep,
        file: *mut file,
        cmd: ::core::ffi::c_uint,
        arg: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    pub fn snd_wavefront_synth_open(hw: *mut snd_hwdep, file: *mut file) -> ::core::ffi::c_int;
    pub fn snd_wavefront_synth_release(hw: *mut snd_hwdep, file: *mut file) -> ::core::ffi::c_int;

    /* FX processor - see also yss225.[ch] */
    pub fn snd_wavefront_fx_start(dev: *mut snd_wavefront_t) -> ::core::ffi::c_int;
    pub fn snd_wavefront_fx_detect(dev: *mut snd_wavefront_t) -> ::core::ffi::c_int;
    pub fn snd_wavefront_fx_ioctl(
        hw: *mut snd_hwdep,
        file: *mut file,
        cmd: ::core::ffi::c_uint,
        arg: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    pub fn snd_wavefront_fx_open(hw: *mut snd_hwdep, file: *mut file) -> ::core::ffi::c_int;
    pub fn snd_wavefront_fx_release(hw: *mut snd_hwdep, file: *mut file) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
