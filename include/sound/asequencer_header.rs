/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Main header file for the ALSA sequencer
 *  Copyright (c) 1998-1999 by Frank van de Pol <fvdpol@coil.demon.nl>
 *            (c) 1998-1999 by Jaroslav Kysela <perex@perex.cz>
 */

// Dependencies supplied by the Linux ALSA headers:
// linux/ioctl.h, sound/asound.h, and uapi/sound/asequencer.h

/*
 * type check macros
 */
/* result events: 0-4 */
macro_rules! snd_seq_ev_is_result_type { ($ev:expr) => {{ unsafe { (*$ev).r#type < 5 } }}; }
/* channel specific events: 5-19 */
macro_rules! snd_seq_ev_is_channel_type { ($ev:expr) => {{ unsafe { (*$ev).r#type >= 5 && (*$ev).r#type < 20 } }}; }
/* note events: 5-9 */
macro_rules! snd_seq_ev_is_note_type { ($ev:expr) => {{ unsafe { (*$ev).r#type >= 5 && (*$ev).r#type < 10 } }}; }
/* control events: 10-19 */
macro_rules! snd_seq_ev_is_control_type { ($ev:expr) => {{ unsafe { (*$ev).r#type >= 10 && (*$ev).r#type < 20 } }}; }
/* queue control events: 30-39 */
macro_rules! snd_seq_ev_is_queue_type { ($ev:expr) => {{ unsafe { (*$ev).r#type >= 30 && (*$ev).r#type < 40 } }}; }
/* system status messages */
macro_rules! snd_seq_ev_is_message_type { ($ev:expr) => {{ unsafe { (*$ev).r#type >= 60 && (*$ev).r#type < 69 } }}; }
/* sample messages */
macro_rules! snd_seq_ev_is_sample_type { ($ev:expr) => {{ unsafe { (*$ev).r#type >= 70 && (*$ev).r#type < 79 } }}; }
/* user-defined messages */
macro_rules! snd_seq_ev_is_user_type { ($ev:expr) => {{ unsafe { (*$ev).r#type >= 90 && (*$ev).r#type < 99 } }}; }
/* fixed length events: 0-99 */
macro_rules! snd_seq_ev_is_fixed_type { ($ev:expr) => {{ unsafe { (*$ev).r#type < 100 } }}; }
/* variable length events: 130-139 */
macro_rules! snd_seq_ev_is_variable_type { ($ev:expr) => {{ unsafe { (*$ev).r#type >= 130 && (*$ev).r#type < 140 } }}; }
/* reserved for kernel */
macro_rules! snd_seq_ev_is_reserved { ($ev:expr) => {{ unsafe { (*$ev).r#type >= 150 } }}; }

/* direct dispatched events */
macro_rules! snd_seq_ev_is_direct { ($ev:expr) => {{ unsafe { (*$ev).queue == SNDRV_SEQ_QUEUE_DIRECT } }}; }

/*
 * macros to check event flags
 */
/* prior events */
macro_rules! snd_seq_ev_is_prior {
    ($ev:expr) => {{ unsafe { ((*$ev).flags & SNDRV_SEQ_PRIORITY_MASK) == SNDRV_SEQ_PRIORITY_HIGH } }};
}

/* event length type */
macro_rules! snd_seq_ev_length_type { ($ev:expr) => {{ unsafe { (*$ev).flags & SNDRV_SEQ_EVENT_LENGTH_MASK } }}; }
macro_rules! snd_seq_ev_is_fixed { ($ev:expr) => {{ snd_seq_ev_length_type!($ev) == SNDRV_SEQ_EVENT_LENGTH_FIXED }}; }
macro_rules! snd_seq_ev_is_variable { ($ev:expr) => {{ snd_seq_ev_length_type!($ev) == SNDRV_SEQ_EVENT_LENGTH_VARIABLE }}; }
macro_rules! snd_seq_ev_is_varusr { ($ev:expr) => {{ snd_seq_ev_length_type!($ev) == SNDRV_SEQ_EVENT_LENGTH_VARUSR }}; }

/* time-stamp type */
macro_rules! snd_seq_ev_timestamp_type { ($ev:expr) => {{ unsafe { (*$ev).flags & SNDRV_SEQ_TIME_STAMP_MASK } }}; }
macro_rules! snd_seq_ev_is_tick { ($ev:expr) => {{ snd_seq_ev_timestamp_type!($ev) == SNDRV_SEQ_TIME_STAMP_TICK }}; }
macro_rules! snd_seq_ev_is_real { ($ev:expr) => {{ snd_seq_ev_timestamp_type!($ev) == SNDRV_SEQ_TIME_STAMP_REAL }}; }

/* time-mode type */
macro_rules! snd_seq_ev_timemode_type { ($ev:expr) => {{ unsafe { (*$ev).flags & SNDRV_SEQ_TIME_MODE_MASK } }}; }
macro_rules! snd_seq_ev_is_abstime { ($ev:expr) => {{ snd_seq_ev_timemode_type!($ev) == SNDRV_SEQ_TIME_MODE_ABS }}; }
macro_rules! snd_seq_ev_is_reltime { ($ev:expr) => {{ snd_seq_ev_timemode_type!($ev) == SNDRV_SEQ_TIME_MODE_REL }}; }

/* check whether the given event is a UMP event */
macro_rules! snd_seq_ev_is_ump {
    ($ev:expr) => {{
        // CONFIG_SND_SEQ_UMP is a build-time configuration supplied externally.
        cfg!(feature = "CONFIG_SND_SEQ_UMP") && unsafe { (*$ev).flags & SNDRV_SEQ_EVENT_UMP != 0 }
    }};
}

/* queue sync port */
macro_rules! snd_seq_queue_sync_port { ($q:expr) => { ($q) + 16 }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
