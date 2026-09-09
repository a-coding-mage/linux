/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Machine dependent access functions for RTC registers.
 */

// Dependencies supplied by the corresponding Linux RTC and time modules:
// `rtc_time`, `time64_t`, `mc146818_get_time`, `rtc_tm_to_time64`, and `pr_err`.

#[cfg(feature = "CONFIG_RTC_MC146818_LIB")]
#[inline]
pub unsafe fn mc146818_get_cmos_time() -> time64_t {
    let mut tm: rtc_time = core::mem::zeroed();

    if mc146818_get_time(&mut tm, 1000) != 0 {
        pr_err(b"Unable to read current time from RTC\n\0".as_ptr());
        return 0;
    }

    rtc_tm_to_time64(&tm)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
