// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency equivalent of: #include "dml2_internal_shared_types.h"

extern "C" {
    pub fn dcn5_calculate_output_link(
        s: *mut dml2_core_internal_scratch,
        PHYCLK: f64,
        PHYCLKD18: f64,
        PHYCLKD32: f64,
        Downspreading: f64,
        Output: dml2_output_encoder_class,
        OutputFormat: dml2_output_format_class,
        HTotal: u32,
        HActive: u32,
        PixelClockBackEnd: f64,
        ForcedOutputLinkBPP: f64,
        DSCInputBitPerComponent: u32,
        NumberOfDSCSlices: u32,
        AudioSampleRate: f64,
        AudioSampleLayout: u32,
        ODMModeNoDSC: dml2_odm_mode,
        ODMModeDSC: dml2_odm_mode,
        DSCEnable: dml2_dsc_enable_option,
        OutputLinkDPLanes: u32,
        OutputLinkDPRate: dml2_output_link_dp_rate,
        // Output
        RequiresDSC: *mut bool,
        RequiresFEC: *mut bool,
        OutBpp: *mut f64,
        OutputType: *mut dml2_core_internal_output_type,
        OutputRate: *mut dml2_core_internal_output_type_rate,
        RequiredSlots: *mut u32,
    );

    pub fn dcn5_calculate_odm_mode(
        MaximumPixelsPerLinePerDSCUnit: u32,
        HActive: u32,
        OutFormat: dml2_output_format_class,
        Output: dml2_output_encoder_class,
        ODMUse: dml2_odm_mode,
        MaxDispclk: f64,
        DSCEnable: bool,
        TotalNumberOfActiveDPP: u32,
        MaxNumDPP: u32,
        PixelClock: f64,
        MaximumSlicesPerDSCUnit: u32,
        NumberOfDSCSlices: u32,
        odm_combine_support_mask: u32,
        // Output
        TotalAvailablePipesSupport: *mut bool,
        NumberOfDPP: *mut u32,
        ODMMode: *mut dml2_odm_mode,
        RequiredDISPCLKPerSurface: *mut f64,
    );

    pub fn dcn5_calculate_required_dtbclk(
        DSCEnable: bool,
        PixelClock: f64,
        OutputFormat: dml2_output_format_class,
        OutputBpp: f64,
        DSCSlices: u32,
        HTotal: u32,
        HActive: u32,
        AudioRate: u32,
        AudioLayout: u32,
    ) -> f64;

    pub fn dcn5_calculate_required_dispclk(
        ODMMode: dml2_odm_mode,
        PixelClock: f64,
        isTMDS420: bool,
    ) -> f64;

    pub fn dcn5_calculate_write_back_dispclk(
        WritebackPixelFormat: dml2_source_format_class,
        PixelClock: f64,
        ODMMode: dml2_odm_mode,
        WritebackHRatio: f64,
        WritebackVRatio: f64,
        WritebackHTaps: u32,
        WritebackVTaps: u32,
        WritebackHTapsChroma: u32,
        WritebackVTapsChroma: u32,
        WritebackSourceWidth: u32,
        WritebackDestinationWidth: u32,
        HTotal: u32,
        WritebackLineBufferSize: u32,
    ) -> f64;

    pub fn dcn5_calculate_dsc_delay_requirement(
        DSCEnabled: bool,
        ODMMode: dml2_odm_mode,
        DSCInputBitPerComponent: u32,
        OutputBpp: f64,
        HActive: u32,
        HTotal: u32,
        NumberOfDSCSlices: u32,
        OutputFormat: dml2_output_format_class,
        Output: dml2_output_encoder_class,
        PixelClock: f64,
        PixelClockBackEnd: f64,
        use_legacy_dsc_delay_formula: bool,
    ) -> u32;

    pub fn dcn5_calculate_single_pipe_dppclk_and_scl_throughput(
        HRatio: f64,
        HRatioChroma: f64,
        VRatio: f64,
        VRatioChroma: f64,
        MaxDCHUBToPSCLThroughput: f64,
        MaxPSCLToLBThroughput: f64,
        PixelClock: f64,
        SourcePixelFormat: dml2_source_format_class,
        HTaps: u32,
        HTapsChroma: u32,
        VTaps: u32,
        VTapsChroma: u32,
        // Output
        PSCL_THROUGHPUT: *mut f64,
        PSCL_THROUGHPUT_CHROMA: *mut f64,
        DPPCLKUsingSingleDPP: *mut f64,
    );

    pub fn dcn5_calculate_pixel_delivery_times(
        display_cfg: *const dml2_display_cfg,
        NoOfDPP: *mut u32,
        NumberOfActiveSurfaces: u32,
        VRatioPrefetchY: *mut f64,
        VRatioPrefetchC: *mut f64,
        swath_width_luma_ub: *mut u32,
        swath_width_chroma_ub: *mut u32,
        PSCL_THROUGHPUT: *mut f64,
        PSCL_THROUGHPUT_CHROMA: *mut f64,
        Dppclk: *mut f64,
        DCFCLKDeepSleep: f64,
        BytePerPixelY: *mut u32,
        BytePerPixelC: *mut u32,
        req_per_swath_ub_l: *mut u32,
        req_per_swath_ub_c: *mut u32,
        // Output
        DisplayPipeLineDeliveryTimeLuma: *mut f64,
        DisplayPipeLineDeliveryTimeChroma: *mut f64,
        DisplayPipeLineDeliveryTimeLumaPrefetch: *mut f64,
        DisplayPipeLineDeliveryTimeChromaPrefetch: *mut f64,
        DisplayPipeRequestDeliveryTimeLuma: *mut f64,
        DisplayPipeRequestDeliveryTimeChroma: *mut f64,
        DisplayPipeRequestDeliveryTimeLumaPrefetch: *mut f64,
        DisplayPipeRequestDeliveryTimeChromaPrefetch: *mut f64,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
