/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_BLK_WBT controls whether the real writeback-throttling interface is
 * available.  The declarations below assume that configuration is enabled. */

#[cfg(CONFIG_BLK_WBT)]
extern "C" {
    pub fn wbt_init_enable_default(disk: *mut gendisk);
    pub fn wbt_disable_default(disk: *mut gendisk);
    pub fn wbt_enable_default(disk: *mut gendisk);

    pub fn wbt_get_min_lat(q: *mut request_queue) -> u64;
    pub fn wbt_disabled(q: *mut request_queue) -> bool;
    pub fn wbt_set_lat(disk: *mut gendisk, val: i64) -> i32;
}

#[cfg(not(CONFIG_BLK_WBT))]
pub unsafe fn wbt_init_enable_default(_disk: *mut gendisk) {}

#[cfg(not(CONFIG_BLK_WBT))]
pub unsafe fn wbt_disable_default(_disk: *mut gendisk) {}

#[cfg(not(CONFIG_BLK_WBT))]
pub unsafe fn wbt_enable_default(_disk: *mut gendisk) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
