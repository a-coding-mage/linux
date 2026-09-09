/* SPDX-License-Identifier: GPL-2.0 */

// The C preprocessor undefines and redefines HYP_EVENT around the inclusion of
// asm/kvm_hypevents.h. Rust has no direct equivalent of identifier token
// pasting (hyp_event_id_##__name), so the declaration-producing macro is kept
// as a source-level macro requiring the generated identifier explicitly.
macro_rules! HYP_EVENT {
    ($id:ident, $name:literal) => {
        #[link_section = concat!(".hyp.event_ids.", $name)]
        static mut $id: hyp_event_id = hyp_event_id {
            enabled: ATOMIC_INIT(0),
        };
    };
}

// #define HYP_EVENT_MULTI_READ
// #include <asm/kvm_hypevents.h>
// The included declarations are supplied by the dependent translation unit.

// #undef HYP_EVENT_MULTI_READ
// #undef HYP_EVENT

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
