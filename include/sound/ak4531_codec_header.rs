/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Universal interface for Audio Codec '97.
 *
 * ASAHI KASEI - AK4531 codec.  Not really an AC'97 codec, but it uses a
 * very similar interface as AC'97.
 *
 * C dependencies from the original header are supplied by other files.
 */

pub const AK4531_LMASTER: u16 = 0x00; /* master volume left */
pub const AK4531_RMASTER: u16 = 0x01; /* master volume right */
pub const AK4531_LVOICE: u16 = 0x02; /* channel volume left */
pub const AK4531_RVOICE: u16 = 0x03; /* channel volume right */
pub const AK4531_LFM: u16 = 0x04; /* FM volume left */
pub const AK4531_RFM: u16 = 0x05; /* FM volume right */
pub const AK4531_LCD: u16 = 0x06; /* CD volume left */
pub const AK4531_RCD: u16 = 0x07; /* CD volume right */
pub const AK4531_LLINE: u16 = 0x08; /* LINE volume left */
pub const AK4531_RLINE: u16 = 0x09; /* LINE volume right */
pub const AK4531_LAUXA: u16 = 0x0a; /* AUXA volume left */
pub const AK4531_RAUXA: u16 = 0x0b; /* AUXA volume right */
pub const AK4531_MONO1: u16 = 0x0c; /* MONO1 volume left */
pub const AK4531_MONO2: u16 = 0x0d; /* MONO1 volume right */
pub const AK4531_MIC: u16 = 0x0e; /* MIC volume */
pub const AK4531_MONO_OUT: u16 = 0x0f; /* Mono-out volume */
pub const AK4531_OUT_SW1: u16 = 0x10; /* Output mixer switch 1 */
pub const AK4531_OUT_SW2: u16 = 0x11; /* Output mixer switch 2 */
pub const AK4531_LIN_SW1: u16 = 0x12; /* Input left mixer switch 1 */
pub const AK4531_RIN_SW1: u16 = 0x13; /* Input right mixer switch 1 */
pub const AK4531_LIN_SW2: u16 = 0x14; /* Input left mixer switch 2 */
pub const AK4531_RIN_SW2: u16 = 0x15; /* Input right mixer switch 2 */
pub const AK4531_RESET: u16 = 0x16; /* Reset & power down */
pub const AK4531_CLOCK: u16 = 0x17; /* Clock select */
pub const AK4531_AD_IN: u16 = 0x18; /* AD input select */
pub const AK4531_MIC_GAIN: u16 = 0x19; /* MIC amplified gain */

#[repr(C)]
pub struct snd_ak4531 {
    pub write: Option<unsafe extern "C" fn(*mut snd_ak4531, u16, u16)>,
    pub private_data: *mut core::ffi::c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_ak4531)>,
    pub regs: [u8; 0x20],
    pub reg_mutex: crate::mutex,
}

pub unsafe extern "C" fn snd_ak4531_mixer(
    card: *mut crate::snd_card,
    _ak4531: *mut snd_ak4531,
    rak4531: *mut *mut snd_ak4531,
) -> core::ffi::c_int;

#[cfg(feature = "CONFIG_PM")]
pub unsafe extern "C" fn snd_ak4531_suspend(ak4531: *mut snd_ak4531);

#[cfg(feature = "CONFIG_PM")]
pub unsafe extern "C" fn snd_ak4531_resume(ak4531: *mut snd_ak4531);

#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub unsafe fn snd_ak4531_suspend(_ak4531: *mut snd_ak4531) {}

#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub unsafe fn snd_ak4531_resume(_ak4531: *mut snd_ak4531) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
