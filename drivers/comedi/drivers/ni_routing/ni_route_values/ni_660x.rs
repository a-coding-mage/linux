// SPDX-License-Identifier: GPL-2.0+
/*
 * Route information for NI_660X boards.
 *
 * This is the Rust spelling of the NI-660x route-value table.  The routing
 * identifiers and value constructors are supplied by ni_route_values.h/all.h.
 */

// The source uses the B(), I(), and U() table-entry constructors.  They are
// intentionally retained here as dependency-provided Rust constructors.

pub const NI_660X_ROUTE_VALUES: family_route_values = family_route_values {
    family: "ni_660x",
    register_values: route_values! {
        NI_PFI(8) => { NI_CtrInternalOutput(7) => I(1) },
        NI_PFI(10) => { NI_CtrGate(7) => I(1) },
        NI_PFI(11) => { NI_CtrSource(7) => I(1) },
        NI_PFI(12) => { NI_CtrInternalOutput(6) => I(1) },
        NI_PFI(14) => { NI_CtrGate(6) => I(1) },
        NI_PFI(15) => { NI_CtrSource(6) => I(1) },
        NI_PFI(16) => { NI_CtrInternalOutput(5) => I(1) },
        NI_PFI(18) => { NI_CtrGate(5) => I(1) },
        NI_PFI(19) => { NI_CtrSource(5) => I(1) },
        NI_PFI(20) => { NI_CtrInternalOutput(4) => I(1) },
        NI_PFI(22) => { NI_CtrGate(4) => I(1) },
        NI_PFI(23) => { NI_CtrSource(4) => I(1) },
        NI_PFI(24) => { NI_CtrInternalOutput(3) => I(1) },
        NI_PFI(26) => { NI_CtrGate(3) => I(1) },
        NI_PFI(27) => { NI_CtrSource(3) => I(1) },
        NI_PFI(28) => { NI_CtrInternalOutput(2) => I(1) },
        NI_PFI(30) => { NI_CtrGate(2) => I(1) },
        NI_PFI(31) => { NI_CtrSource(2) => I(1) },
        NI_PFI(32) => { NI_CtrInternalOutput(1) => I(1) },
        NI_PFI(34) => { NI_CtrGate(1) => I(1) },
        NI_PFI(35) => { NI_CtrSource(1) => I(1) },
        NI_PFI(36) => { NI_CtrInternalOutput(0) => I(1) },
        NI_PFI(38) => { NI_CtrGate(0) => I(1) },
        NI_PFI(39) => { NI_CtrSource(0) => I(1) },

        // For each counter source, PFI(11..39) map to 9..2, trigger lines
        // map to 11..17, the following counter gate maps to 10, and the
        // timebases/logic-low map to 0, 30, 18, and 31 respectively.
        NI_CtrSource(0..=7) => {
            NI_PFI(11) => U(9), NI_PFI(15) => U(8), NI_PFI(19) => U(7),
            NI_PFI(23) => U(6), NI_PFI(27) => U(5), NI_PFI(31) => U(4),
            NI_PFI(35) => U(3), NI_PFI(39) => U(2),
            TRIGGER_LINE(0) => U(11), TRIGGER_LINE(1) => U(12),
            TRIGGER_LINE(2) => U(13), TRIGGER_LINE(3) => U(14),
            TRIGGER_LINE(4) => U(15), TRIGGER_LINE(5) => U(16),
            TRIGGER_LINE(6) => U(17), NI_20MHzTimebase => U(0),
            NI_80MHzTimebase => U(30), NI_100kHzTimebase => U(18),
            NI_LogicLow => U(31)
        },

        // Counter gate and auxiliary routes have the same PFI/trigger layout;
        // the counter-specific source and internal-output routes are explicit.
        NI_CtrGate(0..=7) => {
            NI_PFI(10) => I(9), NI_PFI(14) => I(8), NI_PFI(18) => I(7),
            NI_PFI(22) => I(6), NI_PFI(26) => I(5), NI_PFI(30) => I(4),
            NI_PFI(34) => I(3), NI_PFI(38) => I(2), NI_PFI(39) => I(0),
            TRIGGER_LINE(0) => I(11), TRIGGER_LINE(1) => I(12),
            TRIGGER_LINE(2) => I(13), TRIGGER_LINE(3) => I(14),
            TRIGGER_LINE(4) => I(15), TRIGGER_LINE(5) => I(16),
            TRIGGER_LINE(6) => I(17), NI_LogicLow => I(31)
        },
        NI_CtrAux(0..=7) => {
            NI_PFI(9) => I(9), NI_PFI(13) => I(8), NI_PFI(17) => I(7),
            NI_PFI(21) => I(6), NI_PFI(25) => I(5), NI_PFI(29) => I(4),
            NI_PFI(33) => I(3), NI_PFI(37) => I(2), NI_PFI(39) => I(0),
            TRIGGER_LINE(0) => I(11), TRIGGER_LINE(1) => I(12),
            TRIGGER_LINE(2) => I(13), TRIGGER_LINE(3) => I(14),
            TRIGGER_LINE(4) => I(15), TRIGGER_LINE(5) => I(16),
            TRIGGER_LINE(6) => I(17), NI_LogicLow => I(31)
        }
    },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
