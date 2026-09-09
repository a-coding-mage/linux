/*
 * Copyright 2020 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding display implementation.

pub unsafe fn afmt3_setup_hdmi_audio(afmt: *mut afmt) {
    let afmt3 = DCN30_AFMT_FROM_AFMT(afmt);

    if (*afmt).funcs.afmt_poweron.is_some() {
        ((*afmt).funcs.afmt_poweron.unwrap())(afmt);
    }

    REG_UPDATE!(afmt3, AFMT_AUDIO_PACKET_CONTROL, AFMT_60958_CS_UPDATE, 1);
    REG_UPDATE_2!(afmt3, AFMT_AUDIO_PACKET_CONTROL2,
        AFMT_AUDIO_LAYOUT_OVRD, 0, AFMT_60958_OSF_OVRD, 0);
    REG_UPDATE_2!(afmt3, AFMT_60958_0,
        AFMT_60958_CS_CHANNEL_NUMBER_L, 1, AFMT_60958_CS_CLOCK_ACCURACY, 0);
    REG_UPDATE!(afmt3, AFMT_60958_1, AFMT_60958_CS_CHANNEL_NUMBER_R, 2);
    REG_UPDATE_6!(afmt3, AFMT_60958_2,
        AFMT_60958_CS_CHANNEL_NUMBER_2, 3,
        AFMT_60958_CS_CHANNEL_NUMBER_3, 4,
        AFMT_60958_CS_CHANNEL_NUMBER_4, 5,
        AFMT_60958_CS_CHANNEL_NUMBER_5, 6,
        AFMT_60958_CS_CHANNEL_NUMBER_6, 7,
        AFMT_60958_CS_CHANNEL_NUMBER_7, 8);
}

unsafe fn speakers_to_channels(speaker_flags: audio_speaker_flags) -> audio_cea_channels {
    let mut cea_channels = audio_cea_channels::default();
    cea_channels.channels.FL = speaker_flags.FL_FR;
    cea_channels.channels.FR = speaker_flags.FL_FR;
    cea_channels.channels.LFE = speaker_flags.LFE;
    cea_channels.channels.FC = speaker_flags.FC;

    if speaker_flags.RL_RR {
        cea_channels.channels.RL_RC = speaker_flags.RL_RR;
        cea_channels.channels.RR = speaker_flags.RL_RR;
        cea_channels.channels.RC_RLC_FLC = speaker_flags.RC;
    } else {
        cea_channels.channels.RL_RC = speaker_flags.RC;
    }

    if speaker_flags.FLC_FRC {
        cea_channels.channels.RC_RLC_FLC = speaker_flags.FLC_FRC;
        cea_channels.channels.RRC_FRC = speaker_flags.FLC_FRC;
    } else {
        cea_channels.channels.RC_RLC_FLC = speaker_flags.RLC_RRC;
        cea_channels.channels.RRC_FRC = speaker_flags.RLC_RRC;
    }
    cea_channels
}

pub unsafe fn afmt3_se_audio_setup(
    afmt: *mut afmt,
    az_inst: u32,
    audio_info: *mut audio_info,
) {
    let afmt3 = DCN30_AFMT_FROM_AFMT(afmt);
    let mut channels: u32 = 0;

    ASSERT!(!audio_info.is_null());
    if audio_info.is_null() { return; }
    channels = speakers_to_channels((*audio_info).flags.speaker_flags).all;
    REG_SET!(afmt3, AFMT_AUDIO_SRC_CONTROL, 0, AFMT_AUDIO_SRC_SELECT, az_inst);
    REG_UPDATE!(afmt3, AFMT_AUDIO_PACKET_CONTROL2, AFMT_AUDIO_CHANNEL_ENABLE, channels);
    if (*afmt).funcs.afmt_poweron.is_none() {
        REG_UPDATE!(afmt3, AFMT_MEM_PWR, AFMT_MEM_PWR_FORCE, 0);
    }
}

pub unsafe fn afmt3_audio_mute_control(afmt: *mut afmt, mute: bool) {
    let afmt3 = DCN30_AFMT_FROM_AFMT(afmt);
    if mute && (*afmt).funcs.afmt_powerdown.is_some() {
        ((*afmt).funcs.afmt_powerdown.unwrap())(afmt);
    }
    if !mute && (*afmt).funcs.afmt_poweron.is_some() {
        ((*afmt).funcs.afmt_poweron.unwrap())(afmt);
    }
    REG_UPDATE!(afmt3, AFMT_AUDIO_PACKET_CONTROL, AFMT_AUDIO_SAMPLE_SEND, !mute);
}

pub unsafe fn afmt3_audio_info_immediate_update(afmt: *mut afmt) {
    let afmt3 = DCN30_AFMT_FROM_AFMT(afmt);
    REG_UPDATE!(afmt3, AFMT_INFOFRAME_CONTROL0, AFMT_AUDIO_INFO_UPDATE, 1);
}

pub unsafe fn afmt3_setup_dp_audio(afmt: *mut afmt) {
    let afmt3 = DCN30_AFMT_FROM_AFMT(afmt);
    if (*afmt).funcs.afmt_poweron.is_some() {
        ((*afmt).funcs.afmt_poweron.unwrap())(afmt);
    }
    REG_UPDATE!(afmt3, AFMT_AUDIO_PACKET_CONTROL, AFMT_60958_CS_UPDATE, 1);
    REG_UPDATE_2!(afmt3, AFMT_AUDIO_PACKET_CONTROL2,
        AFMT_AUDIO_LAYOUT_OVRD, 0, AFMT_60958_OSF_OVRD, 0);
    REG_UPDATE!(afmt3, AFMT_INFOFRAME_CONTROL0, AFMT_AUDIO_INFO_UPDATE, 1);
    REG_UPDATE!(afmt3, AFMT_60958_0, AFMT_60958_CS_CLOCK_ACCURACY, 0);
}

static mut DCN30_AFMT_FUNCS: afmt_funcs = afmt_funcs {
    setup_hdmi_audio: Some(afmt3_setup_hdmi_audio),
    se_audio_setup: Some(afmt3_se_audio_setup),
    audio_mute_control: Some(afmt3_audio_mute_control),
    audio_info_immediate_update: Some(afmt3_audio_info_immediate_update),
    setup_dp_audio: Some(afmt3_setup_dp_audio),
};

pub unsafe fn afmt3_construct(
    afmt3: *mut dcn30_afmt,
    ctx: *mut dc_context,
    inst: u32,
    afmt_regs: *const dcn30_afmt_registers,
    afmt_shift: *const dcn30_afmt_shift,
    afmt_mask: *const dcn30_afmt_mask,
) {
    (*afmt3).base.ctx = ctx;
    (*afmt3).base.inst = inst;
    (*afmt3).base.funcs = &raw mut DCN30_AFMT_FUNCS;
    (*afmt3).regs = afmt_regs;
    (*afmt3).afmt_shift = afmt_shift;
    (*afmt3).afmt_mask = afmt_mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
