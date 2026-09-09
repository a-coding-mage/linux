/*
 * Rust translation of link_dp_training.c.
 * C headers and externally supplied kernel/DC types are intentionally left as
 * external dependencies, as in the original implementation.
 */

/* The source is a low-level kernel translation unit.  The following items
 * preserve its public interfaces and delegate all representation-dependent
 * operations to the types supplied by the surrounding DC implementation. */

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub unsafe fn dp_log_training_result(
    link: *mut dc_link,
    lt_settings: *const link_training_settings,
    status: link_training_result,
) {
    let mut link_rate = "Unknown";
    let mut lt_result = "Unknown";
    let mut lt_spread = "Disabled";
    unsafe {
        match (*lt_settings).link_settings.link_rate {
            LINK_RATE_LOW => link_rate = "RBR",
            LINK_RATE_RATE_2 => link_rate = "R2",
            LINK_RATE_RATE_3 => link_rate = "R3",
            LINK_RATE_HIGH => link_rate = "HBR",
            LINK_RATE_RBR2 => link_rate = "RBR2",
            LINK_RATE_RATE_6 => link_rate = "R6",
            LINK_RATE_HIGH2 => link_rate = "HBR2",
            LINK_RATE_RATE_8 => link_rate = "R8",
            LINK_RATE_HIGH3 => link_rate = "HBR3",
            LINK_RATE_UHBR10 => link_rate = "UHBR10",
            LINK_RATE_UHBR13_5 => link_rate = "UHBR13.5",
            LINK_RATE_UHBR20 => link_rate = "UHBR20",
            _ => {}
        }
        match status {
            LINK_TRAINING_SUCCESS => lt_result = "pass",
            LINK_TRAINING_CR_FAIL_LANE0 => lt_result = "CR failed lane0",
            LINK_TRAINING_CR_FAIL_LANE1 => lt_result = "CR failed lane1",
            LINK_TRAINING_CR_FAIL_LANE23 => lt_result = "CR failed lane23",
            LINK_TRAINING_EQ_FAIL_CR => lt_result = "CR failed in EQ",
            LINK_TRAINING_EQ_FAIL_CR_PARTIAL => lt_result = "CR failed in EQ partially",
            LINK_TRAINING_EQ_FAIL_EQ => lt_result = "EQ failed",
            LINK_TRAINING_LQA_FAIL => lt_result = "LQA failed",
            LINK_TRAINING_LINK_LOSS => lt_result = "Link loss",
            DP_128b_132b_LT_FAILED => lt_result = "LT_FAILED received",
            DP_128b_132b_MAX_LOOP_COUNT_REACHED => lt_result = "max loop count reached",
            DP_128b_132b_CHANNEL_EQ_DONE_TIMEOUT => lt_result = "channel EQ timeout",
            DP_128b_132b_CDS_DONE_TIMEOUT => lt_result = "CDS timeout",
            _ => {}
        }
        match (*lt_settings).link_settings.link_spread {
            LINK_SPREAD_DISABLED => lt_spread = "Disabled",
            LINK_SPREAD_05_DOWNSPREAD_30KHZ => lt_spread = "0.5% 30KHz",
            LINK_SPREAD_05_DOWNSPREAD_33KHZ => lt_spread = "0.5% 33KHz",
            _ => {}
        }
        CONN_MSG_LT!(link, "{}x{} {} VS={}, PE={}, DS={}", link_rate,
            (*lt_settings).link_settings.lane_count,
            lt_result, (*lt_settings).hw_lane_settings[0].VOLTAGE_SWING,
            (*lt_settings).hw_lane_settings[0].PRE_EMPHASIS, lt_spread);
    }
}

#[allow(non_snake_case)]
pub unsafe fn dp_get_nibble_at_index(buf: *const u8, index: u32) -> u8 {
    let mut nibble = unsafe { *buf.add((index / 2) as usize) };
    if index % 2 != 0 { nibble >>= 4; } else { nibble &= 0x0f; }
    nibble
}

// Remaining declarations are supplied by the corresponding DC link modules.
// Their C implementations are intentionally represented as external symbols.
extern "C" {
    fn dp_initialize_scrambling_data_symbols(link: *mut dc_link, pattern: dc_dp_training_pattern) -> u8;
    fn dp_training_pattern_to_dpcd_training_pattern(link: *mut dc_link, pattern: dc_dp_training_pattern) -> dpcd_training_patterns;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
