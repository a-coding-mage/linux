/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: the original header includes "types.h" for the C u64 type.

pub const PLANETCORE_KEY_BOARD_TYPE: &str = "BO";
pub const PLANETCORE_KEY_BOARD_REV: &str = "BR";
pub const PLANETCORE_KEY_MB_RAM: &str = "D1";
pub const PLANETCORE_KEY_MAC_ADDR: &str = "EA";
pub const PLANETCORE_KEY_FLASH_SPEED: &str = "FS";
pub const PLANETCORE_KEY_IP_ADDR: &str = "IP";
pub const PLANETCORE_KEY_KB_NVRAM: &str = "NV";
pub const PLANETCORE_KEY_PROCESSOR: &str = "PR";
pub const PLANETCORE_KEY_PROC_VARIANT: &str = "PV";
pub const PLANETCORE_KEY_SERIAL_BAUD: &str = "SB";
pub const PLANETCORE_KEY_SERIAL_PORT: &str = "SP";
pub const PLANETCORE_KEY_SWITCH: &str = "SW";
pub const PLANETCORE_KEY_TEMP_OFFSET: &str = "TC";
pub const PLANETCORE_KEY_TARGET_IP: &str = "TIP";
pub const PLANETCORE_KEY_CRYSTAL_HZ: &str = "XT";

/* Prepare the table for processing, by turning all newlines
 * into NULL bytes.
 */
extern "C" {
    pub fn planetcore_prepare_table(table: *mut std::os::raw::c_char);

    /* Return the value associated with a given key in text,
     * decimal, or hex format.
     *
     * Returns zero/NULL on failure, non-zero on success.
     */
    pub fn planetcore_get_key(
        table: *const std::os::raw::c_char,
        key: *const std::os::raw::c_char,
    ) -> *const std::os::raw::c_char;
    pub fn planetcore_get_decimal(
        table: *const std::os::raw::c_char,
        key: *const std::os::raw::c_char,
        val: *mut u64,
    ) -> std::os::raw::c_int;
    pub fn planetcore_get_hex(
        table: *const std::os::raw::c_char,
        key: *const std::os::raw::c_char,
        val: *mut u64,
    ) -> std::os::raw::c_int;

    /* Updates the device tree local-mac-address properties based
     * on the EA tag.
     */
    pub fn planetcore_set_mac_addrs(table: *const std::os::raw::c_char);

    /* Sets the linux,stdout-path in the /chosen node.  This requires
     * the linux,planetcore-label property in each serial node.
     */
    pub fn planetcore_set_stdout_path(table: *const std::os::raw::c_char);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
