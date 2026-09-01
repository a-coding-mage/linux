// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Sound core KUnit test
 * Author: Ivan Orlov <ivan.orlov0322@gmail.com>
 */

// C dependencies: <kunit/test.h>, <sound/core.h>, <sound/pcm.h>

use core::ffi::{c_char, c_int, c_ulong, c_void};

type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;
type size_t = usize;
type snd_pcm_format_t = c_int;
type snd_pcm_uframes_t = c_ulong;

const SILENCE_BUFFER_MAX_FRAMES: u32 = 260;
const SILENCE_BUFFER_SIZE: size_t = core::mem::size_of::<u64>() * SILENCE_BUFFER_MAX_FRAMES as size_t;

const WRONG_FORMAT_1: snd_pcm_format_t = SNDRV_PCM_FORMAT_LAST + 1;
const WRONG_FORMAT_2: snd_pcm_format_t = -1;

const VALID_NAME: *const c_char = b"ValidName\0".as_ptr() as *const c_char;
const NAME_W_SPEC_CHARS: *const c_char = b"In%v@1id name\0".as_ptr() as *const c_char;
const NAME_W_SPACE: *const c_char = b"Test name\0".as_ptr() as *const c_char;
const NAME_W_SPACE_REMOVED: *const c_char = b"Testname\0".as_ptr() as *const c_char;

const TEST_FIRST_COMPONENT: *const c_char = b"Component1\0".as_ptr() as *const c_char;
const TEST_SECOND_COMPONENT: *const c_char = b"Component2\0".as_ptr() as *const c_char;
const TEST_COMPONENTS: *const c_char = b"Component1 Component2\0".as_ptr() as *const c_char;

#[repr(C)]
struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_mmap_status {
    hw_ptr: snd_pcm_uframes_t,
}

#[repr(C)]
struct snd_pcm_mmap_control {
    appl_ptr: snd_pcm_uframes_t,
}

#[repr(C)]
struct snd_pcm_runtime {
    status: *mut snd_pcm_mmap_status,
    control: *mut snd_pcm_mmap_control,
    buffer_size: snd_pcm_uframes_t,
    boundary: snd_pcm_uframes_t,
}

#[repr(C)]
struct snd_card {
    id: [c_char; 16],
    components: [c_char; 128],
}

#[repr(C)]
struct kunit_case {
    run_case: Option<unsafe extern "C" fn(*mut kunit)>,
}

#[repr(C)]
struct kunit_suite {
    name: *const c_char,
    test_cases: *mut kunit_case,
}

#[repr(C)]
struct snd_format_test_data {
    format: snd_pcm_format_t,
    physical_bits: c_int,
    width: c_int,
    le: c_int,
    sd: c_int,
    silence: [u8; 8],
    name: *const c_char,
}

#[repr(C)]
struct avail_test_data {
    buffer_size: snd_pcm_uframes_t,
    hw_ptr: snd_pcm_uframes_t,
    appl_ptr: snd_pcm_uframes_t,
    expected_avail: snd_pcm_uframes_t,
}

macro_rules! silence {
    () => {
        [0, 0, 0, 0, 0, 0, 0, 0]
    };
    ($($value:expr),+ $(,)?) => {{
        let mut arr = [0u8; 8];
        let values = [$($value as u8),+];
        let mut i = 0;
        while i < values.len() {
            arr[i] = values[i];
            i += 1;
        }
        arr
    }};
}

macro_rules! define_format {
    ($fmt:ident, $name:expr, $pbits:expr, $wd:expr, $endianness:expr, $signd:expr, $silence_arr:expr) => {
        snd_format_test_data {
            format: $fmt,
            physical_bits: $pbits,
            width: $wd,
            le: $endianness,
            sd: $signd,
            silence: $silence_arr,
            name: concat!($name, "\0").as_ptr() as *const c_char,
        }
    };
}

macro_rules! kunit_case {
    ($name:ident) => {
        kunit_case {
            run_case: Some($name),
        }
    };
}

extern "C" {
    static SNDRV_PCM_FORMAT_LAST: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S8: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_U8: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S16_BE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_U16_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_U16_BE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S24_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S24_BE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_U24_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_U24_BE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S32_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S32_BE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_U32_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_U32_BE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_FLOAT_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_FLOAT_BE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_FLOAT64_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_FLOAT64_BE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_IEC958_SUBFRAME_BE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_MU_LAW: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_A_LAW: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_IMA_ADPCM: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_G723_24: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_G723_40: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_DSD_U8: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_DSD_U16_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_DSD_U32_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_DSD_U16_BE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_DSD_U32_BE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S20_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S20_BE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_U20_LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_U20_BE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S24_3LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S24_3BE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_U24_3LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_U24_3BE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S20_3LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S20_3BE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_U20_3LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_U20_3BE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S18_3LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_S18_3BE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_U18_3LE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_U18_3BE: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_G723_24_1B: snd_pcm_format_t;
    static SNDRV_PCM_FORMAT_G723_40_1B: snd_pcm_format_t;
    static GFP_KERNEL: c_int;
    static EINVAL: c_int;

    fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_width(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_signed(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_unsigned(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_little_endian(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_big_endian(format: snd_pcm_format_t) -> c_int;
    fn snd_pcm_format_set_silence(format: snd_pcm_format_t, data: *mut c_void, samples: size_t) -> c_int;
    fn snd_pcm_playback_avail(runtime: *mut snd_pcm_runtime) -> snd_pcm_uframes_t;
    fn snd_pcm_capture_avail(runtime: *mut snd_pcm_runtime) -> snd_pcm_uframes_t;
    fn snd_card_set_id(card: *mut snd_card, id: *const c_char);
    fn snd_pcm_format_name(format: snd_pcm_format_t) -> *const c_char;
    fn snd_component_add(card: *mut snd_card, component: *const c_char) -> c_int;
    fn kunit_kzalloc(test: *mut kunit, size: size_t, flags: c_int) -> *mut c_void;
    fn kunit_info(test: *mut kunit, fmt: *const c_char, ...);

    fn KUNIT_EXPECT_EQ(test: *mut kunit, left: c_ulong, right: c_ulong);
    fn KUNIT_ASSERT_EQ(test: *mut kunit, left: c_int, right: c_int);
    fn KUNIT_ASSERT_NOT_ERR_OR_NULL(test: *mut kunit, ptr: *const c_void);
    fn KUNIT_EXPECT_STREQ(test: *mut kunit, left: *const c_char, right: *const c_char);
    fn KUNIT_EXPECT_STRNEQ(test: *mut kunit, left: *const c_char, right: *const c_char);
    fn KUNIT_ASSERT_NOT_NULL_MSG(test: *mut kunit, ptr: *const c_void, fmt: *const c_char, ...);
    fn KUNIT_ASSERT_STREQ(test: *mut kunit, left: *const c_char, right: *const c_char);
    fn kunit_test_suite(suite: *mut kunit_suite);
    fn MODULE_DESCRIPTION(description: *const c_char);
    fn MODULE_AUTHOR(author: *const c_char);
    fn MODULE_LICENSE(license: *const c_char);
}

static VALID_FMT: [snd_format_test_data; 49] = unsafe {
    [
        define_format!(SNDRV_PCM_FORMAT_S8, "S8", 8, 8, -1, 1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_U8, "U8", 8, 8, -1, 0, silence!(0x80)),
        define_format!(SNDRV_PCM_FORMAT_S16_LE, "S16_LE", 16, 16, 1, 1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_S16_BE, "S16_BE", 16, 16, 0, 1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_U16_LE, "U16_LE", 16, 16, 1, 0, silence!(0x00, 0x80)),
        define_format!(SNDRV_PCM_FORMAT_U16_BE, "U16_BE", 16, 16, 0, 0, silence!(0x80, 0x00)),
        define_format!(SNDRV_PCM_FORMAT_S24_LE, "S24_LE", 32, 24, 1, 1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_S24_BE, "S24_BE", 32, 24, 0, 1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_U24_LE, "U24_LE", 32, 24, 1, 0, silence!(0x00, 0x00, 0x80)),
        define_format!(SNDRV_PCM_FORMAT_U24_BE, "U24_BE", 32, 24, 0, 0, silence!(0x00, 0x80, 0x00, 0x00)),
        define_format!(SNDRV_PCM_FORMAT_S32_LE, "S32_LE", 32, 32, 1, 1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_S32_BE, "S32_BE", 32, 32, 0, 1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_U32_LE, "U32_LE", 32, 32, 1, 0, silence!(0x00, 0x00, 0x00, 0x80)),
        define_format!(SNDRV_PCM_FORMAT_U32_BE, "U32_BE", 32, 32, 0, 0, silence!(0x80, 0x00, 0x00, 0x00)),
        define_format!(SNDRV_PCM_FORMAT_FLOAT_LE, "FLOAT_LE", 32, 32, 1, -1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_FLOAT_BE, "FLOAT_BE", 32, 32, 0, -1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_FLOAT64_LE, "FLOAT64_LE", 64, 64, 1, -1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_FLOAT64_BE, "FLOAT64_BE", 64, 64, 0, -1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_IEC958_SUBFRAME_LE, "IEC958_SUBFRAME_LE", 32, 32, 1, -1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_IEC958_SUBFRAME_BE, "IEC958_SUBFRAME_BE", 32, 32, 0, -1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_MU_LAW, "MU_LAW", 8, 8, -1, -1, silence!(0x7f)),
        define_format!(SNDRV_PCM_FORMAT_A_LAW, "A_LAW", 8, 8, -1, -1, silence!(0x55)),
        define_format!(SNDRV_PCM_FORMAT_IMA_ADPCM, "IMA_ADPCM", 4, 4, -1, -1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_G723_24, "G723_24", 3, 3, -1, -1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_G723_40, "G723_40", 5, 5, -1, -1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_DSD_U8, "DSD_U8", 8, 8, 1, 0, silence!(0x69)),
        define_format!(SNDRV_PCM_FORMAT_DSD_U16_LE, "DSD_U16_LE", 16, 16, 1, 0, silence!(0x69, 0x69)),
        define_format!(SNDRV_PCM_FORMAT_DSD_U32_LE, "DSD_U32_LE", 32, 32, 1, 0, silence!(0x69, 0x69, 0x69, 0x69)),
        define_format!(SNDRV_PCM_FORMAT_DSD_U16_BE, "DSD_U16_BE", 16, 16, 0, 0, silence!(0x69, 0x69)),
        define_format!(SNDRV_PCM_FORMAT_DSD_U32_BE, "DSD_U32_BE", 32, 32, 0, 0, silence!(0x69, 0x69, 0x69, 0x69)),
        define_format!(SNDRV_PCM_FORMAT_S20_LE, "S20_LE", 32, 20, 1, 1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_S20_BE, "S20_BE", 32, 20, 0, 1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_U20_LE, "U20_LE", 32, 20, 1, 0, silence!(0x00, 0x00, 0x08, 0x00)),
        define_format!(SNDRV_PCM_FORMAT_U20_BE, "U20_BE", 32, 20, 0, 0, silence!(0x00, 0x08, 0x00, 0x00)),
        define_format!(SNDRV_PCM_FORMAT_S24_3LE, "S24_3LE", 24, 24, 1, 1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_S24_3BE, "S24_3BE", 24, 24, 0, 1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_U24_3LE, "U24_3LE", 24, 24, 1, 0, silence!(0x00, 0x00, 0x80)),
        define_format!(SNDRV_PCM_FORMAT_U24_3BE, "U24_3BE", 24, 24, 0, 0, silence!(0x80, 0x00, 0x00)),
        define_format!(SNDRV_PCM_FORMAT_S20_3LE, "S20_3LE", 24, 20, 1, 1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_S20_3BE, "S20_3BE", 24, 20, 0, 1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_U20_3LE, "U20_3LE", 24, 20, 1, 0, silence!(0x00, 0x00, 0x08)),
        define_format!(SNDRV_PCM_FORMAT_U20_3BE, "U20_3BE", 24, 20, 0, 0, silence!(0x08, 0x00, 0x00)),
        define_format!(SNDRV_PCM_FORMAT_S18_3LE, "S18_3LE", 24, 18, 1, 1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_S18_3BE, "S18_3BE", 24, 18, 0, 1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_U18_3LE, "U18_3LE", 24, 18, 1, 0, silence!(0x00, 0x00, 0x02)),
        define_format!(SNDRV_PCM_FORMAT_U18_3BE, "U18_3BE", 24, 18, 0, 0, silence!(0x02, 0x00, 0x00)),
        define_format!(SNDRV_PCM_FORMAT_G723_24_1B, "G723_24_1B", 8, 3, -1, -1, silence!()),
        define_format!(SNDRV_PCM_FORMAT_G723_40_1B, "G723_40_1B", 8, 5, -1, -1, silence!()),
    ]
};

unsafe extern "C" fn test_phys_format_size(test: *mut kunit) {
    let mut i: u32 = 0;

    while (i as usize) < VALID_FMT.len() {
        KUNIT_EXPECT_EQ(
            test,
            snd_pcm_format_physical_width(VALID_FMT[i as usize].format) as c_ulong,
            VALID_FMT[i as usize].physical_bits as c_ulong,
        );
        i += 1;
    }

    KUNIT_EXPECT_EQ(test, snd_pcm_format_physical_width(WRONG_FORMAT_1) as c_ulong, -EINVAL as c_ulong);
    KUNIT_EXPECT_EQ(test, snd_pcm_format_physical_width(WRONG_FORMAT_2) as c_ulong, -EINVAL as c_ulong);
}

unsafe extern "C" fn test_format_width(test: *mut kunit) {
    let mut i: u32 = 0;

    while (i as usize) < VALID_FMT.len() {
        KUNIT_EXPECT_EQ(
            test,
            snd_pcm_format_width(VALID_FMT[i as usize].format) as c_ulong,
            VALID_FMT[i as usize].width as c_ulong,
        );
        i += 1;
    }

    KUNIT_EXPECT_EQ(test, snd_pcm_format_width(WRONG_FORMAT_1) as c_ulong, -EINVAL as c_ulong);
    KUNIT_EXPECT_EQ(test, snd_pcm_format_width(WRONG_FORMAT_2) as c_ulong, -EINVAL as c_ulong);
}

unsafe extern "C" fn test_format_signed(test: *mut kunit) {
    let mut i: u32 = 0;

    while (i as usize) < VALID_FMT.len() {
        KUNIT_EXPECT_EQ(
            test,
            snd_pcm_format_signed(VALID_FMT[i as usize].format) as c_ulong,
            (if VALID_FMT[i as usize].sd < 0 { -EINVAL } else { VALID_FMT[i as usize].sd }) as c_ulong,
        );
        KUNIT_EXPECT_EQ(
            test,
            snd_pcm_format_unsigned(VALID_FMT[i as usize].format) as c_ulong,
            (if VALID_FMT[i as usize].sd < 0 { -EINVAL } else { 1 - VALID_FMT[i as usize].sd }) as c_ulong,
        );
        i += 1;
    }

    KUNIT_EXPECT_EQ(test, snd_pcm_format_width(WRONG_FORMAT_1) as c_ulong, -EINVAL as c_ulong);
    KUNIT_EXPECT_EQ(test, snd_pcm_format_width(WRONG_FORMAT_2) as c_ulong, -EINVAL as c_ulong);
}

unsafe extern "C" fn test_format_endianness(test: *mut kunit) {
    let mut i: u32 = 0;

    while (i as usize) < VALID_FMT.len() {
        KUNIT_EXPECT_EQ(
            test,
            snd_pcm_format_little_endian(VALID_FMT[i as usize].format) as c_ulong,
            (if VALID_FMT[i as usize].le < 0 { -EINVAL } else { VALID_FMT[i as usize].le }) as c_ulong,
        );
        KUNIT_EXPECT_EQ(
            test,
            snd_pcm_format_big_endian(VALID_FMT[i as usize].format) as c_ulong,
            (if VALID_FMT[i as usize].le < 0 { -EINVAL } else { 1 - VALID_FMT[i as usize].le }) as c_ulong,
        );
        i += 1;
    }

    KUNIT_EXPECT_EQ(test, snd_pcm_format_little_endian(WRONG_FORMAT_1) as c_ulong, -EINVAL as c_ulong);
    KUNIT_EXPECT_EQ(test, snd_pcm_format_little_endian(WRONG_FORMAT_2) as c_ulong, -EINVAL as c_ulong);
    KUNIT_EXPECT_EQ(test, snd_pcm_format_big_endian(WRONG_FORMAT_1) as c_ulong, -EINVAL as c_ulong);
    KUNIT_EXPECT_EQ(test, snd_pcm_format_big_endian(WRONG_FORMAT_2) as c_ulong, -EINVAL as c_ulong);
}

unsafe extern "C" fn _test_fill_silence(
    test: *mut kunit,
    data: *const snd_format_test_data,
    buffer: *mut u8,
    samples_count: size_t,
) {
    let sample_bytes: size_t = ((*data).physical_bits >> 3) as size_t;
    let mut i: u32 = 0;

    KUNIT_ASSERT_EQ(
        test,
        snd_pcm_format_set_silence((*data).format, buffer as *mut c_void, samples_count),
        0,
    );
    while (i as size_t) < samples_count * sample_bytes {
        KUNIT_EXPECT_EQ(
            test,
            *buffer.add(i as usize) as c_ulong,
            (*data).silence[(i as size_t % sample_bytes) as usize] as c_ulong,
        );
        i += 1;
    }
}

unsafe extern "C" fn test_format_fill_silence(test: *mut kunit) {
    static BUF_SAMPLES: [u32; 6] = [10, 20, 32, 64, 129, SILENCE_BUFFER_MAX_FRAMES];
    let buffer: *mut u8;
    let mut i: u32;
    let mut j: u32;

    buffer = kunit_kzalloc(test, SILENCE_BUFFER_SIZE, GFP_KERNEL) as *mut u8;
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, buffer as *const c_void);

    i = 0;
    while (i as usize) < BUF_SAMPLES.len() {
        j = 0;
        while (j as usize) < VALID_FMT.len() {
            _test_fill_silence(test, &VALID_FMT[j as usize], buffer, BUF_SAMPLES[i as usize] as size_t);
            j += 1;
        }
        i += 1;
    }

    KUNIT_EXPECT_EQ(
        test,
        snd_pcm_format_set_silence(WRONG_FORMAT_1, buffer as *mut c_void, 20) as c_ulong,
        -EINVAL as c_ulong,
    );
    KUNIT_EXPECT_EQ(
        test,
        snd_pcm_format_set_silence(SNDRV_PCM_FORMAT_LAST, buffer as *mut c_void, 0) as c_ulong,
        0,
    );
}

fn calculate_boundary(buffer_size: snd_pcm_uframes_t) -> snd_pcm_uframes_t {
    let mut boundary: snd_pcm_uframes_t = buffer_size;

    while boundary * 2 <= 0x7fffffff_u64 as snd_pcm_uframes_t - buffer_size {
        boundary *= 2;
    }
    boundary
}

static P_AVAIL_DATA: [avail_test_data; 3] = [
    /* buf_size + hw_ptr < appl_ptr => avail = buf_size + hw_ptr - appl_ptr + boundary */
    avail_test_data {
        buffer_size: 128,
        hw_ptr: 1000,
        appl_ptr: 1129,
        expected_avail: 1073741824_u64 as snd_pcm_uframes_t - 1,
    },
    /*
     * buf_size + hw_ptr - appl_ptr >= boundary =>
     * => avail = buf_size + hw_ptr - appl_ptr - boundary
     */
    avail_test_data {
        buffer_size: 128,
        hw_ptr: 1073741824_u64 as snd_pcm_uframes_t,
        appl_ptr: 10,
        expected_avail: 118,
    },
    /* standard case: avail = buf_size + hw_ptr - appl_ptr */
    avail_test_data {
        buffer_size: 128,
        hw_ptr: 1000,
        appl_ptr: 1001,
        expected_avail: 127,
    },
];

unsafe extern "C" fn test_playback_avail(test: *mut kunit) {
    let r: *mut snd_pcm_runtime = kunit_kzalloc(test, core::mem::size_of::<snd_pcm_runtime>(), GFP_KERNEL)
        as *mut snd_pcm_runtime;
    let mut i: u32;

    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, r as *const c_void);

    (*r).status = kunit_kzalloc(test, core::mem::size_of::<snd_pcm_mmap_status>(), GFP_KERNEL)
        as *mut snd_pcm_mmap_status;
    (*r).control = kunit_kzalloc(test, core::mem::size_of::<snd_pcm_mmap_control>(), GFP_KERNEL)
        as *mut snd_pcm_mmap_control;
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, (*r).status as *const c_void);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, (*r).control as *const c_void);

    i = 0;
    while (i as usize) < P_AVAIL_DATA.len() {
        (*r).buffer_size = P_AVAIL_DATA[i as usize].buffer_size;
        (*r).boundary = calculate_boundary((*r).buffer_size);
        (*(*r).status).hw_ptr = P_AVAIL_DATA[i as usize].hw_ptr;
        (*(*r).control).appl_ptr = P_AVAIL_DATA[i as usize].appl_ptr;
        KUNIT_EXPECT_EQ(
            test,
            snd_pcm_playback_avail(r) as c_ulong,
            P_AVAIL_DATA[i as usize].expected_avail as c_ulong,
        );
        i += 1;
    }
}

static C_AVAIL_DATA: [avail_test_data; 2] = [
    /* hw_ptr - appl_ptr < 0 => avail = hw_ptr - appl_ptr + boundary */
    avail_test_data {
        buffer_size: 128,
        hw_ptr: 1000,
        appl_ptr: 1001,
        expected_avail: 1073741824_u64 as snd_pcm_uframes_t - 1,
    },
    /* standard case: avail = hw_ptr - appl_ptr */
    avail_test_data {
        buffer_size: 128,
        hw_ptr: 1001,
        appl_ptr: 1000,
        expected_avail: 1,
    },
];

unsafe extern "C" fn test_capture_avail(test: *mut kunit) {
    let r: *mut snd_pcm_runtime = kunit_kzalloc(test, core::mem::size_of::<snd_pcm_runtime>(), GFP_KERNEL)
        as *mut snd_pcm_runtime;
    let mut i: u32;

    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, r as *const c_void);

    (*r).status = kunit_kzalloc(test, core::mem::size_of::<snd_pcm_mmap_status>(), GFP_KERNEL)
        as *mut snd_pcm_mmap_status;
    (*r).control = kunit_kzalloc(test, core::mem::size_of::<snd_pcm_mmap_control>(), GFP_KERNEL)
        as *mut snd_pcm_mmap_control;
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, (*r).status as *const c_void);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, (*r).control as *const c_void);

    i = 0;
    while (i as usize) < C_AVAIL_DATA.len() {
        (*r).buffer_size = C_AVAIL_DATA[i as usize].buffer_size;
        (*r).boundary = calculate_boundary((*r).buffer_size);
        (*(*r).status).hw_ptr = C_AVAIL_DATA[i as usize].hw_ptr;
        (*(*r).control).appl_ptr = C_AVAIL_DATA[i as usize].appl_ptr;
        KUNIT_EXPECT_EQ(
            test,
            snd_pcm_capture_avail(r) as c_ulong,
            C_AVAIL_DATA[i as usize].expected_avail as c_ulong,
        );
        i += 1;
    }
}

unsafe extern "C" fn test_card_set_id(test: *mut kunit) {
    let card: *mut snd_card = kunit_kzalloc(test, core::mem::size_of::<snd_card>(), GFP_KERNEL) as *mut snd_card;
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, card as *const c_void);

    snd_card_set_id(card, VALID_NAME);
    KUNIT_EXPECT_STREQ(test, (*card).id.as_ptr(), VALID_NAME);

    /* clear the first id character so we can set it again */
    (*card).id[0] = b'\0' as c_char;
    snd_card_set_id(card, NAME_W_SPEC_CHARS);
    KUNIT_EXPECT_STRNEQ(test, (*card).id.as_ptr(), NAME_W_SPEC_CHARS);

    (*card).id[0] = b'\0' as c_char;
    snd_card_set_id(card, NAME_W_SPACE);
    kunit_info(test, b"%s\0".as_ptr() as *const c_char, (*card).id.as_ptr());
    KUNIT_EXPECT_STREQ(test, (*card).id.as_ptr(), NAME_W_SPACE_REMOVED);
}

unsafe extern "C" fn test_pcm_format_name(test: *mut kunit) {
    let mut i: u32;
    let mut name: *const c_char;

    i = 0;
    while (i as usize) < VALID_FMT.len() {
        name = snd_pcm_format_name(VALID_FMT[i as usize].format);
        KUNIT_ASSERT_NOT_NULL_MSG(
            test,
            name as *const c_void,
            b"Don't have name for %s\0".as_ptr() as *const c_char,
            VALID_FMT[i as usize].name,
        );
        KUNIT_EXPECT_STREQ(test, name, VALID_FMT[i as usize].name);
        i += 1;
    }

    KUNIT_ASSERT_STREQ(test, snd_pcm_format_name(WRONG_FORMAT_1), b"Unknown\0".as_ptr() as *const c_char);
    KUNIT_ASSERT_STREQ(test, snd_pcm_format_name(WRONG_FORMAT_2), b"Unknown\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn test_card_add_component(test: *mut kunit) {
    let card: *mut snd_card = kunit_kzalloc(test, core::mem::size_of::<snd_card>(), GFP_KERNEL) as *mut snd_card;
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, card as *const c_void);

    snd_component_add(card, TEST_FIRST_COMPONENT);
    KUNIT_ASSERT_STREQ(test, (*card).components.as_ptr(), TEST_FIRST_COMPONENT);

    snd_component_add(card, TEST_SECOND_COMPONENT);
    KUNIT_ASSERT_STREQ(test, (*card).components.as_ptr(), TEST_COMPONENTS);
}

static mut SOUND_UTILS_CASES: [kunit_case; 11] = [
    kunit_case!(test_phys_format_size),
    kunit_case!(test_format_width),
    kunit_case!(test_format_endianness),
    kunit_case!(test_format_signed),
    kunit_case!(test_format_fill_silence),
    kunit_case!(test_playback_avail),
    kunit_case!(test_capture_avail),
    kunit_case!(test_card_set_id),
    kunit_case!(test_pcm_format_name),
    kunit_case!(test_card_add_component),
    kunit_case { run_case: None },
];

static mut SOUND_UTILS_SUITE: kunit_suite = kunit_suite {
    name: b"sound-core-test\0".as_ptr() as *const c_char,
    test_cases: unsafe { SOUND_UTILS_CASES.as_mut_ptr() },
};

#[used]
static SOUND_UTILS_SUITE_REGISTRATION: unsafe extern "C" fn() = {
    unsafe extern "C" fn register() {
        kunit_test_suite(&mut SOUND_UTILS_SUITE);
        MODULE_DESCRIPTION(b"Sound core KUnit test\0".as_ptr() as *const c_char);
        MODULE_AUTHOR(b"Ivan Orlov\0".as_ptr() as *const c_char);
        MODULE_LICENSE(b"GPL\0".as_ptr() as *const c_char);
    }
    register
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
