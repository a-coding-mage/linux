/* SPDX-License-Identifier: GPL-2.0-only */
/****************************************************************************

   Copyright Echo Digital Audio Corporation (c) 1998 - 2004
   All rights reserved
   www.echoaudio.com

   This file is part of Echo Digital Audio's generic driver library.
 ****************************************************************************

 Translation from C++ and adaptation for use in ALSA-Driver
 were made by Giuliano Pochini <pochini@shiny.it>

 ****************************************************************************


   Here's a block diagram of how most of the cards work:

                  +-----------+
           record |           |<-------------------- Inputs
          <-------|           |        |
     PCI          | Transport |        |
     bus          |  engine   |       \|/
          ------->|           |    +-------+
            play  |           |--->|monitor|-------> Outputs
                  +-----------+    | mixer |
                                   +-------+

   The lines going to and from the PCI bus represent "pipes".  A pipe performs
   audio transport - moving audio data to and from buffers on the host via
   bus mastering.

   The inputs and outputs on the right represent input and output "busses."
   A bus is a physical, real connection to the outside world.  An example
   of a bus would be the 1/4" analog connectors on the back of Layla or
   an RCA S/PDIF connector.

   For most cards, there is a one-to-one correspondence between outputs
   and busses; that is, each individual pipe is hard-wired to a single bus.

   Cards that work this way are Darla20, Gina20, Layla20, Darla24, Gina24,
   Layla24, Mona, and Indigo.


   Mia has a feature called "virtual outputs."


                  +-----------+
           record |           |<----------------------------- Inputs
          <-------|           |                  |
     PCI          | Transport |                  |
     bus          |  engine   |                 \|/
          ------->|           |   +------+   +-------+
            play  |           |-->|vmixer|-->|monitor|-------> Outputs
                  +-----------+   +------+   | mixer |
                                             +-------+


   Obviously, the difference here is the box labeled "vmixer."  Vmixer is
   short for "virtual output mixer."  For Mia, pipes are *not* hard-wired
   to a single bus; the vmixer lets you mix any pipe to any bus in any
   combination.

   Note, however, that the left-hand side of the diagram is unchanged.
   Transport works exactly the same way - the difference is in the mixer stage.


   Pipes and busses are numbered starting at zero.



   Pipe index
   ==========

   A number of calls in CEchoGals refer to a "pipe index".  A pipe index is
   a unique number for a pipe that unambiguously refers to a playback or record
   pipe.  Pipe indices are numbered starting with analog outputs, followed by
   digital outputs, then analog inputs, then digital inputs.

   Take Gina24 as an example:

   Pipe index

   0-7            Analog outputs (0 .. FirstDigitalBusOut-1)
   8-15           Digital outputs (FirstDigitalBusOut .. NumBussesOut-1)
   16-17          Analog inputs
   18-25          Digital inputs


   You get the pipe index by calling CEchoGals::OpenAudio; the other transport
   functions take the pipe index as a parameter.  If you need a pipe index for
   some other reason, use the handy Makepipe_index method.


   Some calls take a CChannelMask parameter; CChannelMask is a handy way to
   group pipe indices.



   Digital mode switch
   ===================

   Some cards (right now, Gina24, Layla24, and Mona) have a Digital Mode Switch
   or DMS.  Cards with a DMS can be set to one of three mutually exclusive
   digital modes: S/PDIF RCA, S/PDIF optical, or ADAT optical.

   This may create some confusion since ADAT optical is 8 channels wide and
   S/PDIF is only two channels wide.  Gina24, Layla24, and Mona handle this
   by acting as if they always have 8 digital outs and ins.  If you are in
   either S/PDIF mode, the last 6 channels don't do anything - data sent
   out these channels is thrown away and you will always record zeros.

   Note that with Gina24, Layla24, and Mona, sample rates above 50 kHz are
   only available if you have the card configured for S/PDIF optical or S/PDIF
   RCA.



   Double speed mode
   =================

   Some of the cards support 88.2 kHz and 96 kHz sampling (Darla24, Gina24,
   Layla24, Mona, Mia, and Indigo).  For these cards, the driver sometimes has
   to worry about "double speed mode"; double speed mode applies whenever the
   sampling rate is above 50 kHz.

   For instance, Mona and Layla24 support word clock sync.  However, they
   actually support two different word clock modes - single speed (below
   50 kHz) and double speed (above 50 kHz).  The hardware detects if a single
   or double speed word clock signal is present; the generic code uses that
   information to determine which mode to use.

   The generic code takes care of all this for you.
*/

/* C include dependency preserved: "echoaudio_dsp.h" */

/***********************************************************************

	PCI configuration space

***********************************************************************/

/*
 * PCI vendor ID and device IDs for the hardware
 */
pub const VENDOR_ID: u32 = 0x1057;
pub const DEVICE_ID_56301: u32 = 0x1801;
pub const DEVICE_ID_56361: u32 = 0x3410;
pub const SUBVENDOR_ID: u32 = 0xECC0;

/*
 * Valid Echo PCI subsystem card IDs
 */
pub const DARLA20: u32 = 0x0010;
pub const GINA20: u32 = 0x0020;
pub const LAYLA20: u32 = 0x0030;
pub const DARLA24: u32 = 0x0040;
pub const GINA24: u32 = 0x0050;
pub const LAYLA24: u32 = 0x0060;
pub const MONA: u32 = 0x0070;
pub const MIA: u32 = 0x0080;
pub const INDIGO: u32 = 0x0090;
pub const INDIGO_IO: u32 = 0x00a0;
pub const INDIGO_DJ: u32 = 0x00b0;
pub const DC8: u32 = 0x00c0;
pub const INDIGO_IOX: u32 = 0x00d0;
pub const INDIGO_DJX: u32 = 0x00e0;
pub const ECHO3G: u32 = 0x0100;

/************************************************************************

	Array sizes and so forth

***********************************************************************/

/*
 * Sizes
 */
pub const ECHO_MAXAUDIOINPUTS: usize = 32; /* Max audio input channels */
pub const ECHO_MAXAUDIOOUTPUTS: usize = 32; /* Max audio output channels */
pub const ECHO_MAXAUDIOPIPES: usize = 32; /* Max number of input and output
					   * pipes */
pub const E3G_MAX_OUTPUTS: usize = 16;
pub const ECHO_MAXMIDIJACKS: usize = 1; /* Max MIDI ports */
pub const ECHO_MIDI_QUEUE_SZ: usize = 512; /* Max MIDI input queue entries */
pub const ECHO_MTC_QUEUE_SZ: usize = 32; /* Max MIDI time code input queue
					  * entries */

/*
 * MIDI activity indicator timeout
 */
pub const MIDI_ACTIVITY_TIMEOUT_USEC: u32 = 200000;

/****************************************************************************

   Clocks

*****************************************************************************/

/*
 * Clock numbers
 */
pub const ECHO_CLOCK_INTERNAL: u32 = 0;
pub const ECHO_CLOCK_WORD: u32 = 1;
pub const ECHO_CLOCK_SUPER: u32 = 2;
pub const ECHO_CLOCK_SPDIF: u32 = 3;
pub const ECHO_CLOCK_ADAT: u32 = 4;
pub const ECHO_CLOCK_ESYNC: u32 = 5;
pub const ECHO_CLOCK_ESYNC96: u32 = 6;
pub const ECHO_CLOCK_MTC: u32 = 7;
pub const ECHO_CLOCK_NUMBER: u32 = 8;
pub const ECHO_CLOCKS: u32 = 0xffff;

/*
 * Clock bit numbers - used to report capabilities and whatever clocks
 * are being detected dynamically.
 */
pub const ECHO_CLOCK_BIT_INTERNAL: u32 = 1 << ECHO_CLOCK_INTERNAL;
pub const ECHO_CLOCK_BIT_WORD: u32 = 1 << ECHO_CLOCK_WORD;
pub const ECHO_CLOCK_BIT_SUPER: u32 = 1 << ECHO_CLOCK_SUPER;
pub const ECHO_CLOCK_BIT_SPDIF: u32 = 1 << ECHO_CLOCK_SPDIF;
pub const ECHO_CLOCK_BIT_ADAT: u32 = 1 << ECHO_CLOCK_ADAT;
pub const ECHO_CLOCK_BIT_ESYNC: u32 = 1 << ECHO_CLOCK_ESYNC;
pub const ECHO_CLOCK_BIT_ESYNC96: u32 = 1 << ECHO_CLOCK_ESYNC96;
pub const ECHO_CLOCK_BIT_MTC: u32 = 1 << ECHO_CLOCK_MTC;

/***************************************************************************

   Digital modes

****************************************************************************/

/*
 * Digital modes for Mona, Layla24, and Gina24
 */
pub const DIGITAL_MODE_NONE: u32 = 0xFF;
pub const DIGITAL_MODE_SPDIF_RCA: u32 = 0;
pub const DIGITAL_MODE_SPDIF_OPTICAL: u32 = 1;
pub const DIGITAL_MODE_ADAT: u32 = 2;
pub const DIGITAL_MODE_SPDIF_CDROM: u32 = 3;
pub const DIGITAL_MODES: u32 = 4;

/*
 * Digital mode capability masks
 */
pub const ECHOCAPS_HAS_DIGITAL_MODE_SPDIF_RCA: u32 = 1 << DIGITAL_MODE_SPDIF_RCA;
pub const ECHOCAPS_HAS_DIGITAL_MODE_SPDIF_OPTICAL: u32 = 1 << DIGITAL_MODE_SPDIF_OPTICAL;
pub const ECHOCAPS_HAS_DIGITAL_MODE_ADAT: u32 = 1 << DIGITAL_MODE_ADAT;
pub const ECHOCAPS_HAS_DIGITAL_MODE_SPDIF_CDROM: u32 = 1 << DIGITAL_MODE_SPDIF_CDROM;

pub const EXT_3GBOX_NC: u32 = 0x01; /* 3G box not connected */
pub const EXT_3GBOX_NOT_SET: u32 = 0x02; /* 3G box not detected yet */

pub const ECHOGAIN_MUTED: i32 = -128; /* Minimum possible gain */
pub const ECHOGAIN_MINOUT: i32 = -128; /* Min output gain (dB) */
pub const ECHOGAIN_MAXOUT: i32 = 6; /* Max output gain (dB) */
pub const ECHOGAIN_MININP: i32 = -50; /* Min input gain (0.5 dB) */
pub const ECHOGAIN_MAXINP: i32 = 50; /* Max input gain (0.5 dB) */

pub const PIPE_STATE_STOPPED: i32 = 0; /* Pipe has been reset */
pub const PIPE_STATE_PAUSED: i32 = 1; /* Pipe has been stopped */
pub const PIPE_STATE_STARTED: i32 = 2; /* Pipe has been started */
pub const PIPE_STATE_PENDING: i32 = 3; /* Pipe has pending start */

#[repr(C)]
pub struct audiopipe {
    pub dma_counter: *mut __le32, /* Commpage register that contains
                                   * the current dma position
                                   * (lower 32 bits only)
                                   */
    pub last_period: u32, /* Counter position last time a
                           * period elapsed
                           */
    pub last_counter: u32, /* Used exclusively by pcm_pointer
                            * under PCM core locks.
                            * The last position, which is used
                            * to compute...
                            */
    pub position: u32, /* ...the number of bytes tranferred
                        * by the DMA engine, modulo the
                        * buffer size
                        */
    pub index: ::std::os::raw::c_short, /* Index of the first channel or <0
                                         * if hw is not configured yet
                                         */
    pub interleave: ::std::os::raw::c_short,
    pub sgpage: snd_dma_buffer, /* Room for the scatter-gather list */
    pub hw: snd_pcm_hardware,
    pub constr: snd_pcm_hw_constraint_list,
    pub sglist_head: ::std::os::raw::c_short,
    pub state: ::std::os::raw::c_char, /* pipe state */
}

#[repr(C)]
pub struct audioformat {
    pub interleave: u8, /* How the data is arranged in memory:
                         * mono = 1, stereo = 2, ...
                         */
    pub bits_per_sample: u8, /* 8, 16, 24, 32 (24 bits left aligned) */
    pub mono_to_stereo: ::std::os::raw::c_char, /* Only used if interleave is 1 and
                                                 * if this is an output pipe.
                                                 */
    pub data_are_bigendian: ::std::os::raw::c_char, /* 1 = big endian, 0 = little endian */
}

#[repr(C)]
pub struct echoaudio {
    pub lock: spinlock_t,
    pub substream: [*mut snd_pcm_substream; DSP_MAXPIPES],
    pub mode_mutex: mutex,
    pub num_digital_modes: u16,
    pub digital_mode_list: [u16; 6],
    pub num_clock_sources: u16,
    pub clock_source_list: [u16; 10],
    pub opencount: ::std::os::raw::c_uint, /* protected by mode_mutex */
    pub clock_src_ctl: *mut snd_kcontrol,
    pub analog_pcm: *mut snd_pcm,
    pub digital_pcm: *mut snd_pcm,
    pub card: *mut snd_card,
    pub card_name: *const ::std::os::raw::c_char,
    pub pci: *mut pci_dev,
    pub dsp_registers_phys: ::std::os::raw::c_ulong,
    pub iores: *mut resource,
    pub commpage_dma_buf: *mut snd_dma_buffer,
    pub irq: ::std::os::raw::c_int,
    /*
     * #ifdef ECHOCARD_HAS_MIDI
     * struct snd_rawmidi *rmidi;
     * struct snd_rawmidi_substream *midi_in, *midi_out;
     * #endif
     */
    pub timer: timer_list,
    pub tinuse: ::std::os::raw::c_char, /* Timer in use */
    pub midi_full: ::std::os::raw::c_char, /* MIDI output buffer is full */
    pub can_set_rate: ::std::os::raw::c_char, /* protected by mode_mutex */
    pub rate_set: ::std::os::raw::c_char, /* protected by mode_mutex */

    /* This stuff is used mainly by the lowlevel code */
    pub comm_page: *mut comm_page, /* Virtual address of the memory
                                    * seen by DSP
                                    */
    pub pipe_alloc_mask: u32, /* Bitmask of allocated pipes */
    pub pipe_cyclic_mask: u32, /* Bitmask of pipes with cyclic
                                * buffers
                                */
    pub sample_rate: u32, /* Card sample rate in Hz */
    pub digital_mode: u8, /* Current digital mode
                           * (see DIGITAL_MODE_*)
                           */
    pub spdif_status: u8, /* Gina20, Darla20, Darla24 - only */
    pub clock_state: u8, /* Gina20, Darla20, Darla24 - only */
    pub input_clock: u8, /* Currently selected sample clock
                          * source
                          */
    pub output_clock: u8, /* Layla20 only */
    pub meters_enabled: ::std::os::raw::c_char, /* VU-meters status */
    pub asic_loaded: ::std::os::raw::c_char, /* Set true when ASIC loaded */
    pub bad_board: ::std::os::raw::c_char, /* Set true if DSP won't load */
    pub professional_spdif: ::std::os::raw::c_char, /* 0 = consumer; 1 = professional */
    pub non_audio_spdif: ::std::os::raw::c_char, /* 3G - only */
    pub digital_in_automute: ::std::os::raw::c_char, /* Gina24, Layla24, Mona - only */
    pub has_phantom_power: ::std::os::raw::c_char,
    pub hasnt_input_nominal_level: ::std::os::raw::c_char, /* Gina3G */
    pub phantom_power: ::std::os::raw::c_char, /* Gina3G - only */
    pub has_midi: ::std::os::raw::c_char,
    pub midi_input_enabled: ::std::os::raw::c_char,

    /*
     * #ifdef ECHOCARD_ECHO3G
     * External module -dependent pipe and bus indexes
     * char px_digital_out, px_analog_in, px_digital_in, px_num;
     * char bx_digital_out, bx_analog_in, bx_digital_in, bx_num;
     * #endif
     */

    pub nominal_level: [::std::os::raw::c_char; ECHO_MAXAUDIOPIPES], /* True == -10dBV
                                                                      * False == +4dBu */
    pub input_gain: [i8; ECHO_MAXAUDIOINPUTS], /* Input level -50..+50
                                                * unit is 0.5dB */
    pub output_gain: [i8; ECHO_MAXAUDIOOUTPUTS], /* Output level -128..+6 dB
                                                  * (-128=muted) */
    pub monitor_gain: [[i8; ECHO_MAXAUDIOINPUTS]; ECHO_MAXAUDIOOUTPUTS],
    /* -128..+6 dB */
    pub vmixer_gain: [[i8; ECHO_MAXAUDIOOUTPUTS]; ECHO_MAXAUDIOOUTPUTS],
    /* -128..+6 dB */

    pub digital_modes: u16, /* Bitmask of supported modes
                             * (see ECHOCAPS_HAS_DIGITAL_MODE_*) */
    pub input_clock_types: u16, /* Suppoted input clock types */
    pub output_clock_types: u16, /* Suppoted output clock types -
                                  * Layla20 only */
    pub device_id: u16,
    pub subdevice_id: u16,
    pub dsp_code: *mut u16, /* Current DSP code loaded,
                             * NULL if nothing loaded */
    pub dsp_code_to_load: ::std::os::raw::c_short, /* DSP code to load */
    pub asic_code: ::std::os::raw::c_short, /* Current ASIC code */
    pub comm_page_phys: u32, /* Physical address of the
                              * memory seen by DSP */
    pub dsp_registers: *mut u32, /* DSP's register base, originally __iomem */
    pub active_mask: u32, /* Chs. active mask or
                           * punks out */
    pub fw_cache: [*const firmware; 8], /* Cached firmwares */

    /*
     * #ifdef ECHOCARD_HAS_MIDI
     * u16 mtc_state;
     * u8 midi_buffer[MIDI_IN_BUFFER_SIZE];
     * #endif
     */
}

unsafe extern "C" {
    pub fn init_dsp_comm_page(chip: *mut echoaudio) -> ::std::os::raw::c_int;
    pub fn init_line_levels(chip: *mut echoaudio) -> ::std::os::raw::c_int;
    pub fn free_pipes(chip: *mut echoaudio, pipe: *mut audiopipe) -> ::std::os::raw::c_int;
    pub fn load_firmware(chip: *mut echoaudio) -> ::std::os::raw::c_int;
    pub fn wait_handshake(chip: *mut echoaudio) -> ::std::os::raw::c_int;
    pub fn send_vector(chip: *mut echoaudio, command: u32) -> ::std::os::raw::c_int;
    pub fn get_firmware(
        fw_entry: *mut *const firmware,
        chip: *mut echoaudio,
        fw_index: ::std::os::raw::c_short,
    ) -> ::std::os::raw::c_int;
    pub fn free_firmware(fw_entry: *const firmware, chip: *mut echoaudio);

    /*
     * #ifdef ECHOCARD_HAS_MIDI
     * static int enable_midi_input(struct echoaudio *chip, char enable);
     * static void snd_echo_midi_output_trigger(
     *			struct snd_rawmidi_substream *substream, int up);
     * static int midi_service_irq(struct echoaudio *chip);
     * static int snd_echo_midi_create(struct snd_card *card,
     *				struct echoaudio *chip);
     * #endif
     */
}

#[inline]
pub unsafe fn clear_handshake(chip: *mut echoaudio) {
    unsafe {
        (*(*chip).comm_page).handshake = 0;
    }
}

#[inline]
pub unsafe fn get_dsp_register(chip: *mut echoaudio, index: u32) -> u32 {
    unsafe { readl((*chip).dsp_registers.add(index as usize)) }
}

#[inline]
pub unsafe fn set_dsp_register(chip: *mut echoaudio, index: u32, value: u32) {
    unsafe {
        writel(value, (*chip).dsp_registers.add(index as usize));
    }
}

/* Pipe and bus indexes. PX_* and BX_* are defined as chip->px_* and chip->bx_*
for 3G cards because they depend on the external box. They are integer
constants for all other cards.
Never use those defines directly, use the following functions instead. */

#[inline]
pub unsafe fn px_digital_out(chip: *const echoaudio) -> ::std::os::raw::c_int {
    let _ = chip;
    PX_DIGITAL_OUT
}

#[inline]
pub unsafe fn px_analog_in(chip: *const echoaudio) -> ::std::os::raw::c_int {
    let _ = chip;
    PX_ANALOG_IN
}

#[inline]
pub unsafe fn px_digital_in(chip: *const echoaudio) -> ::std::os::raw::c_int {
    let _ = chip;
    PX_DIGITAL_IN
}

#[inline]
pub unsafe fn px_num(chip: *const echoaudio) -> ::std::os::raw::c_int {
    let _ = chip;
    PX_NUM
}

#[inline]
pub unsafe fn bx_digital_out(chip: *const echoaudio) -> ::std::os::raw::c_int {
    let _ = chip;
    BX_DIGITAL_OUT
}

#[inline]
pub unsafe fn bx_analog_in(chip: *const echoaudio) -> ::std::os::raw::c_int {
    let _ = chip;
    BX_ANALOG_IN
}

#[inline]
pub unsafe fn bx_digital_in(chip: *const echoaudio) -> ::std::os::raw::c_int {
    let _ = chip;
    BX_DIGITAL_IN
}

#[inline]
pub unsafe fn bx_num(chip: *const echoaudio) -> ::std::os::raw::c_int {
    let _ = chip;
    BX_NUM
}

#[inline]
pub unsafe fn num_pipes_out(chip: *const echoaudio) -> ::std::os::raw::c_int {
    unsafe { px_analog_in(chip) }
}

#[inline]
pub unsafe fn num_pipes_in(chip: *const echoaudio) -> ::std::os::raw::c_int {
    unsafe { px_num(chip) - px_analog_in(chip) }
}

#[inline]
pub unsafe fn num_busses_out(chip: *const echoaudio) -> ::std::os::raw::c_int {
    unsafe { bx_analog_in(chip) }
}

#[inline]
pub unsafe fn num_busses_in(chip: *const echoaudio) -> ::std::os::raw::c_int {
    unsafe { bx_num(chip) - bx_analog_in(chip) }
}

#[inline]
pub unsafe fn num_analog_busses_out(chip: *const echoaudio) -> ::std::os::raw::c_int {
    unsafe { bx_digital_out(chip) }
}

#[inline]
pub unsafe fn num_analog_busses_in(chip: *const echoaudio) -> ::std::os::raw::c_int {
    unsafe { bx_digital_in(chip) - bx_analog_in(chip) }
}

#[inline]
pub unsafe fn num_digital_busses_out(chip: *const echoaudio) -> ::std::os::raw::c_int {
    unsafe { num_busses_out(chip) - num_analog_busses_out(chip) }
}

#[inline]
pub unsafe fn num_digital_busses_in(chip: *const echoaudio) -> ::std::os::raw::c_int {
    unsafe { num_busses_in(chip) - num_analog_busses_in(chip) }
}

/* The monitor array is a one-dimensional array; compute the offset
 * into the array */
#[inline]
pub unsafe fn monitor_index(
    chip: *const echoaudio,
    out: ::std::os::raw::c_int,
    in_: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { out * num_busses_in(chip) + in_ }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
