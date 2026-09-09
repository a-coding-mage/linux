/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */

pub const SNDRV_CTL_TLVT_CONTAINER: u32 = 0;
pub const SNDRV_CTL_TLVT_DB_SCALE: u32 = 1;
pub const SNDRV_CTL_TLVT_DB_LINEAR: u32 = 2;
pub const SNDRV_CTL_TLVT_DB_RANGE: u32 = 3;
pub const SNDRV_CTL_TLVT_DB_MINMAX: u32 = 4;
pub const SNDRV_CTL_TLVT_DB_MINMAX_MUTE: u32 = 5;

/* channel-mapping TLV items
 *  TLV length must match with num_channels
 */
pub const SNDRV_CTL_TLVT_CHMAP_FIXED: u32 = 0x101;
pub const SNDRV_CTL_TLVT_CHMAP_VAR: u32 = 0x102;
pub const SNDRV_CTL_TLVT_CHMAP_PAIRED: u32 = 0x103;

pub const SNDRV_CTL_TLVT_FCP_CHANNEL_LABELS: u32 = 0x110;

/*
 * TLV structure is right behind the struct snd_ctl_tlv:
 *   unsigned int type   - see SNDRV_CTL_TLVT_*
 *   unsigned int length
 *   .... data aligned to sizeof(unsigned int), use
 *        block_length = (length + (sizeof(unsigned int) - 1)) &
 *                       ~(sizeof(unsigned int) - 1)) ....
 */
#[macro_export]
macro_rules! SNDRV_CTL_TLVD_LENGTH {
    ($($value:expr),* $(,)?) => {
        (($($value,)*).len() as u32 * core::mem::size_of::<u32>() as u32)
    };
}

#[macro_export]
macro_rules! SNDRV_CTL_TLVD_ITEM {
    ($type:expr, $($value:expr),* $(,)?) => {
        [$type, SNDRV_CTL_TLVD_LENGTH!($($value),*), $($value),*]
    };
}

pub const SNDRV_CTL_TLVO_TYPE: u32 = 0;
pub const SNDRV_CTL_TLVO_LEN: u32 = 1;

#[macro_export]
macro_rules! SNDRV_CTL_TLVD_CONTAINER_ITEM {
    ($($value:expr),* $(,)?) => {
        SNDRV_CTL_TLVD_ITEM!(SNDRV_CTL_TLVT_CONTAINER, $($value),*)
    };
}

#[macro_export]
macro_rules! SNDRV_CTL_TLVD_DECLARE_CONTAINER {
    ($name:ident, $($value:expr),* $(,)?) => {
        let $name: [u32; 2 + <[u32; 0]>::len(&[]) $(+ 1)*] =
            SNDRV_CTL_TLVD_CONTAINER_ITEM!($($value),*);
    };
}

pub const SNDRV_CTL_TLVD_DB_SCALE_MASK: u32 = 0xffff;
pub const SNDRV_CTL_TLVD_DB_SCALE_MUTE: u32 = 0x10000;

#[macro_export]
macro_rules! SNDRV_CTL_TLVD_DB_SCALE_ITEM {
    ($min:expr, $step:expr, $mute:expr) => {
        SNDRV_CTL_TLVD_ITEM!(SNDRV_CTL_TLVT_DB_SCALE, $min,
            (($step) & SNDRV_CTL_TLVD_DB_SCALE_MASK) |
            (if $mute { SNDRV_CTL_TLVD_DB_SCALE_MUTE } else { 0 }))
    };
}

#[macro_export]
macro_rules! SNDRV_CTL_TLVD_DECLARE_DB_SCALE {
    ($name:ident, $min:expr, $step:expr, $mute:expr) => {
        let $name = SNDRV_CTL_TLVD_DB_SCALE_ITEM!($min, $step, $mute);
    };
}

pub const SNDRV_CTL_TLVO_DB_SCALE_MIN: u32 = 2;
pub const SNDRV_CTL_TLVO_DB_SCALE_MUTE_AND_STEP: u32 = 3;

#[macro_export]
macro_rules! SNDRV_CTL_TLVD_DB_MINMAX_ITEM {
    ($min_db:expr, $max_db:expr) => {
        SNDRV_CTL_TLVD_ITEM!(SNDRV_CTL_TLVT_DB_MINMAX, $min_db, $max_db)
    };
}

#[macro_export]
macro_rules! SNDRV_CTL_TLVD_DB_MINMAX_MUTE_ITEM {
    ($min_db:expr, $max_db:expr) => {
        SNDRV_CTL_TLVD_ITEM!(SNDRV_CTL_TLVT_DB_MINMAX_MUTE, $min_db, $max_db)
    };
}

#[macro_export]
macro_rules! SNDRV_CTL_TLVD_DECLARE_DB_MINMAX {
    ($name:ident, $min_db:expr, $max_db:expr) => {
        let $name = SNDRV_CTL_TLVD_DB_MINMAX_ITEM!($min_db, $max_db);
    };
}

#[macro_export]
macro_rules! SNDRV_CTL_TLVD_DECLARE_DB_MINMAX_MUTE {
    ($name:ident, $min_db:expr, $max_db:expr) => {
        let $name = SNDRV_CTL_TLVD_DB_MINMAX_MUTE_ITEM!($min_db, $max_db);
    };
}

pub const SNDRV_CTL_TLVO_DB_MINMAX_MIN: u32 = 2;
pub const SNDRV_CTL_TLVO_DB_MINMAX_MAX: u32 = 3;

/* linear volume between min_dB and max_dB (.01dB unit) */
#[macro_export]
macro_rules! SNDRV_CTL_TLVD_DB_LINEAR_ITEM {
    ($min_db:expr, $max_db:expr) => {
        SNDRV_CTL_TLVD_ITEM!(SNDRV_CTL_TLVT_DB_LINEAR, $min_db, $max_db)
    };
}

#[macro_export]
macro_rules! SNDRV_CTL_TLVD_DECLARE_DB_LINEAR {
    ($name:ident, $min_db:expr, $max_db:expr) => {
        let $name = SNDRV_CTL_TLVD_DB_LINEAR_ITEM!($min_db, $max_db);
    };
}

pub const SNDRV_CTL_TLVO_DB_LINEAR_MIN: u32 = 2;
pub const SNDRV_CTL_TLVO_DB_LINEAR_MAX: u32 = 3;

/* dB range container:
 * Items in dB range container must be ordered by their values and by their
 * dB values. This implies that larger values must correspond with larger
 * dB values (which is also required for all other mixer controls).
 */
/* Each item is: <min> <max> <TLV> */
#[macro_export]
macro_rules! SNDRV_CTL_TLVD_DB_RANGE_ITEM {
    ($($value:expr),* $(,)?) => {
        SNDRV_CTL_TLVD_ITEM!(SNDRV_CTL_TLVT_DB_RANGE, $($value),*)
    };
}

#[macro_export]
macro_rules! SNDRV_CTL_TLVD_DECLARE_DB_RANGE {
    ($name:ident, $($value:expr),* $(,)?) => {
        let $name = SNDRV_CTL_TLVD_DB_RANGE_ITEM!($($value),*);
    };
}

pub const SNDRV_CTL_TLVD_DB_GAIN_MUTE: i32 = -9999999;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
