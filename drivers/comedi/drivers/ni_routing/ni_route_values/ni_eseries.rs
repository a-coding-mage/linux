// SPDX-License-Identifier: GPL-2.0+
/*
 *  comedi/drivers/ni_routing/ni_route_values/ni_eseries.rs
 *  Route information for NI_ESERIES boards.
 *
 *  This is a direct Rust representation of the generated route table.  The
 *  route-value types and signal-value macros are supplied by the surrounding
 *  ni_route_values implementation.
 */

// The e-series backplane TRIGGER_LINE(6) is generally not connected to RTSI(6).

#[allow(non_upper_case_globals)]
pub const ni_eseries_route_values: family_route_values = family_route_values {
    family: "ni_eseries",
    register_values: register_values! {
        B(NI_PFI(0)) => { B(NI_AI_StartTrigger) => I(NI_PFI_OUTPUT_AI_START1) },
        B(NI_PFI(1)) => { B(NI_AI_ReferenceTrigger) => I(NI_PFI_OUTPUT_AI_START2) },
        B(NI_PFI(2)) => { B(NI_AI_ConvertClock) => I(NI_PFI_OUTPUT_AI_CONVERT) },
        B(NI_PFI(3)) => { B(NI_CtrSource(1)) => I(NI_PFI_OUTPUT_G_SRC1) },
        B(NI_PFI(4)) => { B(NI_CtrGate(1)) => I(NI_PFI_OUTPUT_G_GATE1) },
        B(NI_PFI(5)) => { B(NI_AO_SampleClock) => I(NI_PFI_OUTPUT_AO_UPDATE_N) },
        B(NI_PFI(6)) => { B(NI_AO_StartTrigger) => I(NI_PFI_OUTPUT_AO_START1) },
        B(NI_PFI(7)) => { B(NI_AI_SampleClock) => I(NI_PFI_OUTPUT_AI_START_PULSE) },
        B(NI_PFI(8)) => { B(NI_CtrSource(0)) => I(NI_PFI_OUTPUT_G_SRC0) },
        B(NI_PFI(9)) => { B(NI_CtrGate(0)) => I(NI_PFI_OUTPUT_G_GATE0) },
        B(TRIGGER_LINE(7)) => { B(NI_20MHzTimebase) => I(NI_RTSI_OUTPUT_RTSI_OSC) },
        B(NI_CtrOut(0)) => {
            B(NI_CtrInternalOutput(0)) => I(0),
            B(TRIGGER_LINE(0)) => I(1), B(TRIGGER_LINE(1)) => I(2),
            B(TRIGGER_LINE(2)) => I(3), B(TRIGGER_LINE(3)) => I(4),
            B(TRIGGER_LINE(4)) => I(5), B(TRIGGER_LINE(5)) => I(6),
            B(TRIGGER_LINE(6)) => I(7), B(PXI_Star) => I(7)
        },
        B(NI_CtrOut(1)) => { B(NI_CtrInternalOutput(1)) => I(0) },
        B(NI_CtrSource(0)) => {
            /* These are not currently implemented in ni modules. */
            B(NI_PFI(0)) => U(1), B(NI_PFI(1)) => U(2), B(NI_PFI(2)) => U(3),
            B(NI_PFI(3)) => U(4), B(NI_PFI(4)) => U(5), B(NI_PFI(5)) => U(6),
            B(NI_PFI(6)) => U(7), B(NI_PFI(7)) => U(8), B(NI_PFI(8)) => U(9),
            B(NI_PFI(9)) => U(10), B(TRIGGER_LINE(0)) => U(11),
            B(TRIGGER_LINE(1)) => U(12), B(TRIGGER_LINE(2)) => U(13),
            B(TRIGGER_LINE(3)) => U(14), B(TRIGGER_LINE(4)) => U(15),
            B(TRIGGER_LINE(5)) => U(16), B(TRIGGER_LINE(6)) => U(17),
            B(NI_CtrInternalOutput(1)) => U(19), B(PXI_Star) => U(17),
            B(NI_20MHzTimebase) => U(0), B(NI_100kHzTimebase) => U(18),
            B(NI_LogicLow) => U(31)
        },
        B(NI_CtrSource(1)) => {
            /* These are not currently implemented in ni modules. */
            B(NI_PFI(0)) => U(1), B(NI_PFI(1)) => U(2), B(NI_PFI(2)) => U(3),
            B(NI_PFI(3)) => U(4), B(NI_PFI(4)) => U(5), B(NI_PFI(5)) => U(6),
            B(NI_PFI(6)) => U(7), B(NI_PFI(7)) => U(8), B(NI_PFI(8)) => U(9),
            B(NI_PFI(9)) => U(10), B(TRIGGER_LINE(0)) => U(11),
            B(TRIGGER_LINE(1)) => U(12), B(TRIGGER_LINE(2)) => U(13),
            B(TRIGGER_LINE(3)) => U(14), B(TRIGGER_LINE(4)) => U(15),
            B(TRIGGER_LINE(5)) => U(16), B(TRIGGER_LINE(6)) => U(17),
            B(NI_CtrInternalOutput(0)) => U(19), B(PXI_Star) => U(17),
            B(NI_20MHzTimebase) => U(0), B(NI_100kHzTimebase) => U(18),
            B(NI_LogicLow) => U(31)
        },
        B(NI_MasterTimebase) => {
            /* These are not currently implemented in ni modules. */
            B(TRIGGER_LINE(7)) => U(1), B(PXI_Star) => U(2),
            B(PXI_Clk10) => U(3), B(NI_10MHzRefClock) => U(0)
        },
        B(NI_RGOUT0) => {
            B(NI_CtrInternalOutput(0)) => I(0), B(NI_CtrOut(0)) => I(1)
        }
    }
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
