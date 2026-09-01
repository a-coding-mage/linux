// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020, Linaro Limited

// Translated from C implementation source. Original includes:
// <sound/pcm.h>, <sound/soc.h>, <sound/pcm_params.h>,
// <dt-bindings/sound/qcom,q6afe.h>, "q6dsp-lpass-ports.h"

use core::ffi::{c_char, c_int};

const EINVAL: c_int = 22;

macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! playback {
    ($stream_name:expr, $rates:expr, $formats:expr, $channels_min:expr, $channels_max:expr, $rate_min:expr, $rate_max:expr) => {
        snd_soc_pcm_stream {
            stream_name: c_str!($stream_name),
            rates: $rates,
            formats: $formats,
            channels_min: $channels_min,
            channels_max: $channels_max,
            rate_min: $rate_min,
            rate_max: $rate_max,
        }
    };
}

macro_rules! capture {
    ($stream_name:expr, $rates:expr, $formats:expr, $channels_min:expr, $channels_max:expr, $rate_min:expr, $rate_max:expr) => {
        snd_soc_pcm_stream {
            stream_name: c_str!($stream_name),
            rates: $rates,
            formats: $formats,
            channels_min: $channels_min,
            channels_max: $channels_max,
            rate_min: $rate_min,
            rate_max: $rate_max,
        }
    };
}

macro_rules! dai {
    (playback: $playback:expr, name: $name:expr, id: $id:expr) => {
        snd_soc_dai_driver {
            playback: $playback,
            name: c_str!($name),
            id: $id,
            ..unsafe { core::mem::zeroed() }
        }
    };
    (capture: $capture:expr, name: $name:expr, id: $id:expr) => {
        snd_soc_dai_driver {
            capture: $capture,
            name: c_str!($name),
            id: $id,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

macro_rules! Q6AFE_TDM_PB_DAI {
    ($pre:expr, $num:expr, $did:ident) => {
        dai!(
            playback: playback!(
                concat!($pre, " TDM", stringify!($num), " Playback"),
                SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 |
                    SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_176400,
                SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
                1,
                8,
                8000,
                176400
            ),
            name: stringify!($did),
            id: $did
        )
    };
}

macro_rules! Q6AFE_TDM_CAP_DAI {
    ($pre:expr, $num:expr, $did:ident) => {
        dai!(
            capture: capture!(
                concat!($pre, " TDM", stringify!($num), " Capture"),
                SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 |
                    SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_176400,
                SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
                1,
                8,
                8000,
                176400
            ),
            name: stringify!($did),
            id: $did
        )
    };
}

macro_rules! Q6AFE_CDC_DMA_RX_DAI {
    ($did:ident) => {
        dai!(
            playback: playback!(
                concat!(stringify!($did), " Playback"),
                SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 |
                    SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_176400,
                SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
                1,
                8,
                8000,
                176400
            ),
            name: stringify!($did),
            id: $did
        )
    };
}

macro_rules! Q6AFE_CDC_DMA_TX_DAI {
    ($did:ident) => {
        dai!(
            capture: capture!(
                concat!(stringify!($did), " Capture"),
                SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 |
                    SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_176400,
                SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
                1,
                8,
                8000,
                176400
            ),
            name: stringify!($did),
            id: $did
        )
    };
}

macro_rules! Q6AFE_DP_RX_DAI {
    ($did:ident) => {
        dai!(
            playback: playback!(
                concat!(stringify!($did), " Playback"),
                SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000,
                SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE,
                2,
                8,
                48000,
                192000
            ),
            name: stringify!($did),
            id: $did
        )
    };
}

macro_rules! Q6AFE_MI2S_RX_DAI {
    ($pre:expr, $did:ident) => {
        dai!(
            playback: playback!(
                concat!($pre, " MI2S Playback"),
                SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 |
                    SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_176400,
                SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
                1,
                8,
                8000,
                176400
            ),
            name: stringify!($did),
            id: $did
        )
    };
}

macro_rules! Q6AFE_MI2S_TX_DAI {
    ($pre:expr, $did:ident) => {
        dai!(
            capture: capture!(
                concat!($pre, " MI2S Capture"),
                SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 |
                    SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_176400,
                SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
                1,
                8,
                8000,
                176400
            ),
            name: stringify!($did),
            id: $did
        )
    };
}

static mut q6dsp_audio_fe_dais: [snd_soc_dai_driver; 152] = [
    dai!(
        playback: playback!(
            "USB Playback",
            SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_11025 | SNDRV_PCM_RATE_16000 |
                SNDRV_PCM_RATE_22050 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 |
                SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000,
            SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_U16_LE |
                SNDRV_PCM_FMTBIT_U16_BE | SNDRV_PCM_FMTBIT_S24_LE |
                SNDRV_PCM_FMTBIT_S24_BE | SNDRV_PCM_FMTBIT_U24_LE | SNDRV_PCM_FMTBIT_U24_BE,
            1,
            2,
            8000,
            192000
        ),
        name: "USB_RX",
        id: USB_RX
    ),
    dai!(
        playback: playback!(
            "HDMI Playback",
            SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000,
            SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE,
            2,
            8,
            48000,
            192000
        ),
        name: "HDMI",
        id: HDMI_RX
    ),
    dai!(playback: playback!("Slimbus Playback", SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE, 1, 8, 8000, 192000), name: "SLIMBUS_0_RX", id: SLIMBUS_0_RX),
    dai!(capture: capture!("Slimbus Capture", SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE, 1, 8, 8000, 192000), name: "SLIMBUS_0_TX", id: SLIMBUS_0_TX),
    dai!(playback: playback!("Slimbus1 Playback", SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE, 1, 2, 8000, 192000), name: "SLIMBUS_1_RX", id: SLIMBUS_1_RX),
    dai!(capture: capture!("Slimbus1 Capture", SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE, 1, 8, 8000, 192000), name: "SLIMBUS_1_TX", id: SLIMBUS_1_TX),
    dai!(playback: playback!("Slimbus2 Playback", SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE, 1, 8, 8000, 192000), name: "SLIMBUS_2_RX", id: SLIMBUS_2_RX),
    dai!(capture: capture!("Slimbus2 Capture", SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE, 1, 8, 8000, 192000), name: "SLIMBUS_2_TX", id: SLIMBUS_2_TX),
    dai!(playback: playback!("Slimbus3 Playback", SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE, 1, 2, 8000, 192000), name: "SLIMBUS_3_RX", id: SLIMBUS_3_RX),
    dai!(capture: capture!("Slimbus3 Capture", SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE, 1, 8, 8000, 192000), name: "SLIMBUS_3_TX", id: SLIMBUS_3_TX),
    dai!(playback: playback!("Slimbus4 Playback", SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE, 1, 2, 8000, 192000), name: "SLIMBUS_4_RX", id: SLIMBUS_4_RX),
    dai!(capture: capture!("Slimbus4 Capture", SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE, 1, 8, 8000, 192000), name: "SLIMBUS_4_TX", id: SLIMBUS_4_TX),
    dai!(playback: playback!("Slimbus5 Playback", SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE, 1, 2, 8000, 192000), name: "SLIMBUS_5_RX", id: SLIMBUS_5_RX),
    dai!(capture: capture!("Slimbus5 Capture", SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE, 1, 8, 8000, 192000), name: "SLIMBUS_5_TX", id: SLIMBUS_5_TX),
    dai!(playback: playback!("Slimbus6 Playback", SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE, 1, 2, 8000, 192000), name: "SLIMBUS_6_RX", id: SLIMBUS_6_RX),
    dai!(capture: capture!("Slimbus6 Capture", SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE, 1, 8, 8000, 192000), name: "SLIMBUS_6_TX", id: SLIMBUS_6_TX),
    dai!(playback: playback!("Primary MI2S Playback", SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE, 1, 8, 8000, 48000), name: "PRI_MI2S_RX", id: PRIMARY_MI2S_RX),
    dai!(capture: capture!("Primary MI2S Capture", SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE, 1, 8, 8000, 48000), name: "PRI_MI2S_TX", id: PRIMARY_MI2S_TX),
    dai!(playback: playback!("Secondary MI2S Playback", SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000, SNDRV_PCM_FMTBIT_S16_LE, 1, 8, 8000, 48000), name: "SEC_MI2S_RX", id: SECONDARY_MI2S_RX),
    dai!(capture: capture!("Secondary MI2S Capture", SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE, 1, 8, 8000, 48000), name: "SEC_MI2S_TX", id: SECONDARY_MI2S_TX),
    dai!(playback: playback!("Tertiary MI2S Playback", SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000, SNDRV_PCM_FMTBIT_S16_LE, 1, 8, 8000, 48000), name: "TERT_MI2S_RX", id: TERTIARY_MI2S_RX),
    dai!(capture: capture!("Tertiary MI2S Capture", SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE, 1, 8, 8000, 48000), name: "TERT_MI2S_TX", id: TERTIARY_MI2S_TX),
    dai!(playback: playback!("Quaternary MI2S Playback", SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000, SNDRV_PCM_FMTBIT_S16_LE, 1, 8, 8000, 48000), name: "QUAT_MI2S_RX", id: QUATERNARY_MI2S_RX),
    dai!(capture: capture!("Quaternary MI2S Capture", SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE, 1, 8, 8000, 48000), name: "QUAT_MI2S_TX", id: QUATERNARY_MI2S_TX),
    dai!(playback: playback!("Quinary MI2S Playback", SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_192000, SNDRV_PCM_FMTBIT_S16_LE, 1, 8, 8000, 192000), name: "QUIN_MI2S_RX", id: QUINARY_MI2S_RX),
    dai!(capture: capture!("Quinary MI2S Capture", SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000, SNDRV_PCM_FMTBIT_S16_LE, 1, 8, 8000, 48000), name: "QUIN_MI2S_TX", id: QUINARY_MI2S_TX),
    dai!(playback: playback!("Senary MI2S Playback", SNDRV_PCM_RATE_8000_192000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE, 1, 8, 8000, 192000), name: "SEN_MI2S_RX", id: SENARY_MI2S_RX),
    dai!(capture: capture!("Senary MI2S Capture", SNDRV_PCM_RATE_8000_192000, SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE, 1, 8, 8000, 192000), name: "SEN_MI2S_TX", id: SENARY_MI2S_TX),
    Q6AFE_MI2S_RX_DAI!("LPI RX0", LPI_MI2S_RX_0), Q6AFE_MI2S_RX_DAI!("LPI RX1", LPI_MI2S_RX_1),
    Q6AFE_MI2S_RX_DAI!("LPI RX2", LPI_MI2S_RX_2), Q6AFE_MI2S_RX_DAI!("LPI RX3", LPI_MI2S_RX_3),
    Q6AFE_MI2S_RX_DAI!("LPI RX4", LPI_MI2S_RX_4), Q6AFE_MI2S_RX_DAI!("LPI RX5", LPI_MI2S_RX_5),
    Q6AFE_MI2S_RX_DAI!("LPI RX6", LPI_MI2S_RX_6), Q6AFE_MI2S_TX_DAI!("LPI TX0", LPI_MI2S_TX_0),
    Q6AFE_MI2S_TX_DAI!("LPI TX1", LPI_MI2S_TX_1), Q6AFE_MI2S_TX_DAI!("LPI TX2", LPI_MI2S_TX_2),
    Q6AFE_MI2S_TX_DAI!("LPI TX3", LPI_MI2S_TX_3), Q6AFE_MI2S_TX_DAI!("LPI TX4", LPI_MI2S_TX_4),
    Q6AFE_MI2S_TX_DAI!("LPI TX5", LPI_MI2S_TX_5), Q6AFE_MI2S_TX_DAI!("LPI TX6", LPI_MI2S_TX_6),
    Q6AFE_TDM_PB_DAI!("Primary", 0, PRIMARY_TDM_RX_0), Q6AFE_TDM_PB_DAI!("Primary", 1, PRIMARY_TDM_RX_1),
    Q6AFE_TDM_PB_DAI!("Primary", 2, PRIMARY_TDM_RX_2), Q6AFE_TDM_PB_DAI!("Primary", 3, PRIMARY_TDM_RX_3),
    Q6AFE_TDM_PB_DAI!("Primary", 4, PRIMARY_TDM_RX_4), Q6AFE_TDM_PB_DAI!("Primary", 5, PRIMARY_TDM_RX_5),
    Q6AFE_TDM_PB_DAI!("Primary", 6, PRIMARY_TDM_RX_6), Q6AFE_TDM_PB_DAI!("Primary", 7, PRIMARY_TDM_RX_7),
    Q6AFE_TDM_CAP_DAI!("Primary", 0, PRIMARY_TDM_TX_0), Q6AFE_TDM_CAP_DAI!("Primary", 1, PRIMARY_TDM_TX_1),
    Q6AFE_TDM_CAP_DAI!("Primary", 2, PRIMARY_TDM_TX_2), Q6AFE_TDM_CAP_DAI!("Primary", 3, PRIMARY_TDM_TX_3),
    Q6AFE_TDM_CAP_DAI!("Primary", 4, PRIMARY_TDM_TX_4), Q6AFE_TDM_CAP_DAI!("Primary", 5, PRIMARY_TDM_TX_5),
    Q6AFE_TDM_CAP_DAI!("Primary", 6, PRIMARY_TDM_TX_6), Q6AFE_TDM_CAP_DAI!("Primary", 7, PRIMARY_TDM_TX_7),
    Q6AFE_TDM_PB_DAI!("Secondary", 0, SECONDARY_TDM_RX_0), Q6AFE_TDM_PB_DAI!("Secondary", 1, SECONDARY_TDM_RX_1),
    Q6AFE_TDM_PB_DAI!("Secondary", 2, SECONDARY_TDM_RX_2), Q6AFE_TDM_PB_DAI!("Secondary", 3, SECONDARY_TDM_RX_3),
    Q6AFE_TDM_PB_DAI!("Secondary", 4, SECONDARY_TDM_RX_4), Q6AFE_TDM_PB_DAI!("Secondary", 5, SECONDARY_TDM_RX_5),
    Q6AFE_TDM_PB_DAI!("Secondary", 6, SECONDARY_TDM_RX_6), Q6AFE_TDM_PB_DAI!("Secondary", 7, SECONDARY_TDM_RX_7),
    Q6AFE_TDM_CAP_DAI!("Secondary", 0, SECONDARY_TDM_TX_0), Q6AFE_TDM_CAP_DAI!("Secondary", 1, SECONDARY_TDM_TX_1),
    Q6AFE_TDM_CAP_DAI!("Secondary", 2, SECONDARY_TDM_TX_2), Q6AFE_TDM_CAP_DAI!("Secondary", 3, SECONDARY_TDM_TX_3),
    Q6AFE_TDM_CAP_DAI!("Secondary", 4, SECONDARY_TDM_TX_4), Q6AFE_TDM_CAP_DAI!("Secondary", 5, SECONDARY_TDM_TX_5),
    Q6AFE_TDM_CAP_DAI!("Secondary", 6, SECONDARY_TDM_TX_6), Q6AFE_TDM_CAP_DAI!("Secondary", 7, SECONDARY_TDM_TX_7),
    Q6AFE_TDM_PB_DAI!("Tertiary", 0, TERTIARY_TDM_RX_0), Q6AFE_TDM_PB_DAI!("Tertiary", 1, TERTIARY_TDM_RX_1),
    Q6AFE_TDM_PB_DAI!("Tertiary", 2, TERTIARY_TDM_RX_2), Q6AFE_TDM_PB_DAI!("Tertiary", 3, TERTIARY_TDM_RX_3),
    Q6AFE_TDM_PB_DAI!("Tertiary", 4, TERTIARY_TDM_RX_4), Q6AFE_TDM_PB_DAI!("Tertiary", 5, TERTIARY_TDM_RX_5),
    Q6AFE_TDM_PB_DAI!("Tertiary", 6, TERTIARY_TDM_RX_6), Q6AFE_TDM_PB_DAI!("Tertiary", 7, TERTIARY_TDM_RX_7),
    Q6AFE_TDM_CAP_DAI!("Tertiary", 0, TERTIARY_TDM_TX_0), Q6AFE_TDM_CAP_DAI!("Tertiary", 1, TERTIARY_TDM_TX_1),
    Q6AFE_TDM_CAP_DAI!("Tertiary", 2, TERTIARY_TDM_TX_2), Q6AFE_TDM_CAP_DAI!("Tertiary", 3, TERTIARY_TDM_TX_3),
    Q6AFE_TDM_CAP_DAI!("Tertiary", 4, TERTIARY_TDM_TX_4), Q6AFE_TDM_CAP_DAI!("Tertiary", 5, TERTIARY_TDM_TX_5),
    Q6AFE_TDM_CAP_DAI!("Tertiary", 6, TERTIARY_TDM_TX_6), Q6AFE_TDM_CAP_DAI!("Tertiary", 7, TERTIARY_TDM_TX_7),
    Q6AFE_TDM_PB_DAI!("Quaternary", 0, QUATERNARY_TDM_RX_0), Q6AFE_TDM_PB_DAI!("Quaternary", 1, QUATERNARY_TDM_RX_1),
    Q6AFE_TDM_PB_DAI!("Quaternary", 2, QUATERNARY_TDM_RX_2), Q6AFE_TDM_PB_DAI!("Quaternary", 3, QUATERNARY_TDM_RX_3),
    Q6AFE_TDM_PB_DAI!("Quaternary", 4, QUATERNARY_TDM_RX_4), Q6AFE_TDM_PB_DAI!("Quaternary", 5, QUATERNARY_TDM_RX_5),
    Q6AFE_TDM_PB_DAI!("Quaternary", 6, QUATERNARY_TDM_RX_6), Q6AFE_TDM_PB_DAI!("Quaternary", 7, QUATERNARY_TDM_RX_7),
    Q6AFE_TDM_CAP_DAI!("Quaternary", 0, QUATERNARY_TDM_TX_0), Q6AFE_TDM_CAP_DAI!("Quaternary", 1, QUATERNARY_TDM_TX_1),
    Q6AFE_TDM_CAP_DAI!("Quaternary", 2, QUATERNARY_TDM_TX_2), Q6AFE_TDM_CAP_DAI!("Quaternary", 3, QUATERNARY_TDM_TX_3),
    Q6AFE_TDM_CAP_DAI!("Quaternary", 4, QUATERNARY_TDM_TX_4), Q6AFE_TDM_CAP_DAI!("Quaternary", 5, QUATERNARY_TDM_TX_5),
    Q6AFE_TDM_CAP_DAI!("Quaternary", 6, QUATERNARY_TDM_TX_6), Q6AFE_TDM_CAP_DAI!("Quaternary", 7, QUATERNARY_TDM_TX_7),
    Q6AFE_TDM_PB_DAI!("Quinary", 0, QUINARY_TDM_RX_0), Q6AFE_TDM_PB_DAI!("Quinary", 1, QUINARY_TDM_RX_1),
    Q6AFE_TDM_PB_DAI!("Quinary", 2, QUINARY_TDM_RX_2), Q6AFE_TDM_PB_DAI!("Quinary", 3, QUINARY_TDM_RX_3),
    Q6AFE_TDM_PB_DAI!("Quinary", 4, QUINARY_TDM_RX_4), Q6AFE_TDM_PB_DAI!("Quinary", 5, QUINARY_TDM_RX_5),
    Q6AFE_TDM_PB_DAI!("Quinary", 6, QUINARY_TDM_RX_6), Q6AFE_TDM_PB_DAI!("Quinary", 7, QUINARY_TDM_RX_7),
    Q6AFE_TDM_CAP_DAI!("Quinary", 0, QUINARY_TDM_TX_0), Q6AFE_TDM_CAP_DAI!("Quinary", 1, QUINARY_TDM_TX_1),
    Q6AFE_TDM_CAP_DAI!("Quinary", 2, QUINARY_TDM_TX_2), Q6AFE_TDM_CAP_DAI!("Quinary", 3, QUINARY_TDM_TX_3),
    Q6AFE_TDM_CAP_DAI!("Quinary", 4, QUINARY_TDM_TX_4), Q6AFE_TDM_CAP_DAI!("Quinary", 5, QUINARY_TDM_TX_5),
    Q6AFE_TDM_CAP_DAI!("Quinary", 6, QUINARY_TDM_TX_6), Q6AFE_TDM_CAP_DAI!("Quinary", 7, QUINARY_TDM_TX_7),
    Q6AFE_DP_RX_DAI!(DISPLAY_PORT_RX_0), Q6AFE_DP_RX_DAI!(DISPLAY_PORT_RX_1),
    Q6AFE_DP_RX_DAI!(DISPLAY_PORT_RX_2), Q6AFE_DP_RX_DAI!(DISPLAY_PORT_RX_3),
    Q6AFE_DP_RX_DAI!(DISPLAY_PORT_RX_4), Q6AFE_DP_RX_DAI!(DISPLAY_PORT_RX_5),
    Q6AFE_DP_RX_DAI!(DISPLAY_PORT_RX_6), Q6AFE_DP_RX_DAI!(DISPLAY_PORT_RX_7),
    Q6AFE_CDC_DMA_RX_DAI!(WSA_CODEC_DMA_RX_0), Q6AFE_CDC_DMA_TX_DAI!(WSA_CODEC_DMA_TX_0),
    Q6AFE_CDC_DMA_RX_DAI!(WSA_CODEC_DMA_RX_1), Q6AFE_CDC_DMA_TX_DAI!(WSA_CODEC_DMA_TX_1),
    Q6AFE_CDC_DMA_TX_DAI!(WSA_CODEC_DMA_TX_2), Q6AFE_CDC_DMA_TX_DAI!(VA_CODEC_DMA_TX_0),
    Q6AFE_CDC_DMA_TX_DAI!(VA_CODEC_DMA_TX_1), Q6AFE_CDC_DMA_TX_DAI!(VA_CODEC_DMA_TX_2),
    Q6AFE_CDC_DMA_RX_DAI!(RX_CODEC_DMA_RX_0), Q6AFE_CDC_DMA_TX_DAI!(TX_CODEC_DMA_TX_0),
    Q6AFE_CDC_DMA_RX_DAI!(RX_CODEC_DMA_RX_1), Q6AFE_CDC_DMA_TX_DAI!(TX_CODEC_DMA_TX_1),
    Q6AFE_CDC_DMA_RX_DAI!(RX_CODEC_DMA_RX_2), Q6AFE_CDC_DMA_TX_DAI!(TX_CODEC_DMA_TX_2),
    Q6AFE_CDC_DMA_RX_DAI!(RX_CODEC_DMA_RX_3), Q6AFE_CDC_DMA_TX_DAI!(TX_CODEC_DMA_TX_3),
    Q6AFE_CDC_DMA_RX_DAI!(RX_CODEC_DMA_RX_4), Q6AFE_CDC_DMA_TX_DAI!(TX_CODEC_DMA_TX_4),
    Q6AFE_CDC_DMA_RX_DAI!(RX_CODEC_DMA_RX_5), Q6AFE_CDC_DMA_TX_DAI!(TX_CODEC_DMA_TX_5),
    Q6AFE_CDC_DMA_RX_DAI!(RX_CODEC_DMA_RX_6), Q6AFE_CDC_DMA_RX_DAI!(RX_CODEC_DMA_RX_7),
];

#[no_mangle]
pub unsafe extern "C" fn q6dsp_audio_ports_of_xlate_dai_name(
    _component: *mut snd_soc_component,
    args: *const of_phandle_args,
    dai_name: *mut *const c_char,
) -> c_int {
    let id: c_int = (*args).args[0] as c_int;
    let mut ret: c_int = -EINVAL;
    let mut i: usize = 0;

    while i < q6dsp_audio_fe_dais.len() {
        if q6dsp_audio_fe_dais[i].id == id {
            *dai_name = q6dsp_audio_fe_dais[i].name;
            ret = 0;
            break;
        }
        i += 1;
    }

    ret
}
// EXPORT_SYMBOL_GPL(q6dsp_audio_ports_of_xlate_dai_name);

#[no_mangle]
pub unsafe extern "C" fn q6dsp_audio_ports_set_config(
    _dev: *mut device,
    cfg: *mut q6dsp_audio_port_dai_driver_config,
    num_dais: *mut c_int,
) -> *mut snd_soc_dai_driver {
    let mut i: usize = 0;

    while i < q6dsp_audio_fe_dais.len() {
        let id = q6dsp_audio_fe_dais[i].id;

        if id == HDMI_RX || id == DISPLAY_PORT_RX {
            q6dsp_audio_fe_dais[i].ops = (*cfg).q6hdmi_ops;
        } else if id >= DISPLAY_PORT_RX_1 && id <= DISPLAY_PORT_RX_7 {
            q6dsp_audio_fe_dais[i].ops = (*cfg).q6hdmi_ops;
        } else if id >= SLIMBUS_0_RX && id <= SLIMBUS_6_TX {
            q6dsp_audio_fe_dais[i].ops = (*cfg).q6slim_ops;
        } else if (id >= SENARY_MI2S_RX && id <= SENARY_MI2S_TX)
            || (id >= QUINARY_MI2S_RX && id <= QUINARY_MI2S_TX)
            || (id >= PRIMARY_MI2S_RX && id <= QUATERNARY_MI2S_TX)
            || (id >= LPI_MI2S_RX_0 && id <= LPI_MI2S_TX_4)
            || (id >= LPI_MI2S_RX_5 && id <= LPI_MI2S_TX_6)
        {
            q6dsp_audio_fe_dais[i].ops = (*cfg).q6i2s_ops;
        } else if id >= PRIMARY_TDM_RX_0 && id <= QUINARY_TDM_TX_7 {
            q6dsp_audio_fe_dais[i].ops = (*cfg).q6tdm_ops;
        } else if id >= WSA_CODEC_DMA_RX_0 && id <= RX_CODEC_DMA_RX_7 {
            q6dsp_audio_fe_dais[i].ops = (*cfg).q6dma_ops;
        } else if id == USB_RX {
            q6dsp_audio_fe_dais[i].ops = (*cfg).q6usb_ops;
        }

        i += 1;
    }

    *num_dais = q6dsp_audio_fe_dais.len() as c_int;
    q6dsp_audio_fe_dais.as_mut_ptr()
}
// EXPORT_SYMBOL_GPL(q6dsp_audio_ports_set_config);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
