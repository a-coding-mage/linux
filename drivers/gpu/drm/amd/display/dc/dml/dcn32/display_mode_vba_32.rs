/*
 * Copyright 2022 Advanced Micro Devices, Inc. All rights reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 *
 */
// external dependency: 
#include "dc.h"
// external dependency: #include "../display_mode_lib.h"
// external dependency: #include "display_mode_vba_32.h"
// external dependency: #include "../dml_inline_defs.h"
// external dependency: #include "display_mode_vba_util_32.h"
// external dependency: 
#include "../dml1_frl_cap_chk.h"

pub unsafe fn dml32_recalculate(// struct display_mode_libmode_lib);
unsafe fn DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation(
		// struct display_mode_libmode_lib);
pub unsafe fn dml32_ModeSupportAndSystemConfigurationFull(// struct display_mode_libmode_lib);

pub unsafe fn dml32_recalculate(// struct display_mode_libmode_lib)
{
	ModeSupportAndSystemConfiguration(mode_lib);

	dml32_CalculateMaxDETAndMinCompressedBufferSize(mode_lib.vba.ConfigReturnBufferSizeInKByte,
			mode_lib.vba.ROBBufferSizeInKByte,
			DC__NUM_DPP,
			false, //mode_lib.vba.override_setting.nomDETInKByteOverrideEnable,
			0, //mode_lib.vba.override_setting.nomDETInKByteOverrideValue,

			/* Output */
			&mode_lib.vba.MaxTotalDETInKByte, &mode_lib.vba.nomDETInKByte,
			&mode_lib.vba.MinCompressedBufferSizeInKByte);

	PixelClockAdjustmentForProgressiveToInterlaceUnit(mode_lib);
#ifdef __DML_VBA_DEBUG__
	dml_print("DML::%s: Calling DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation\n", file!);
#endif
	DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation(mode_lib);
}

unsafe fn DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation(
		// struct display_mode_libmode_lib)
{
	// struct vba_vars_stv = &mode_lib.let mut vba = Default::default();
	unsigned int j, let mut k = Default::default();
	bool let mut ImmediateFlipRequirementFinal = Default::default();
	int let mut iteration = Default::default();
	double let mut MaxTotalRDBandwidth = Default::default();
	unsigned int let mut NextPrefetchMode = Default::default();
	double MaxTotalRDBandwidthNoUrgentBurst = 0.0;
	bool DestinationLineTimesForPrefetchLessThan2 = let mut false = Default::default();
	bool VRatioPrefetchMoreThanMax = let mut false = Default::default();
	double let mut TWait = Default::default();
	double TotalWRBandwidth = 0;
	double WRBandwidth = 0;

#ifdef __DML_VBA_DEBUG__
	dml_print("DML::%s: --- START -= 1-\n", file!);
	dml_print("DML::%s: mode_lib.vba.PrefetchMode = %d\n", file!, mode_lib.vba.PrefetchMode);
	dml_print("DML::%s: mode_lib.vba.ImmediateFlipSupport = %d\n", file!, mode_lib.vba.ImmediateFlipSupport);
	dml_print("DML::%s: mode_lib.vba.VoltageLevel = %d\n", file!, mode_lib.vba.VoltageLevel);
#endif

	v.WritebackDISPCLK = 0.0;
	v.GlobalDPPCLK = 0.0;

	// DISPCLK and DPPCLK Calculation
	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		if mode_lib.vba.WritebackEnable[k] {
			v.WritebackDISPCLK = dml_max(v.WritebackDISPCLK,
					dml32_CalculateWriteBackDISPCLK(
							mode_lib.vba.WritebackPixelFormat[k],
							mode_lib.vba.PixelClock[k], mode_lib.vba.WritebackHRatio[k],
							mode_lib.vba.WritebackVRatio[k],
							mode_lib.vba.WritebackHTaps[k],
							mode_lib.vba.WritebackVTaps[k],
							mode_lib.vba.WritebackSourceWidth[k],
							(long)mode_lib.vba.WritebackDestinationWidth[k],
							mode_lib.vba.HTotal[k], (unsigned int)mode_lib.vba.WritebackLineBufferSize,
							mode_lib.vba.DISPCLKDPPCLKVCOSpeed));
		}
	}

	v.DISPCLK_calculated = v.let mut WritebackDISPCLK = Default::default();

	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		if mode_lib.vba.BlendingAndTiming[k] == k {
			v.DISPCLK_calculated = dml_max(v.DISPCLK_calculated,
					dml32_CalculateRequiredDispclk(
							mode_lib.vba.ODMCombineEnabled[k],
							mode_lib.vba.PixelClock[k],
							mode_lib.vba.DISPCLKDPPCLKDSCCLKDownSpreading,
							mode_lib.vba.DISPCLKRampingMargin,
							mode_lib.vba.DISPCLKDPPCLKVCOSpeed,
							mode_lib.vba.MaxDppclk[v.soc.num_states - 1]));
		}
	}

	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		dml32_CalculateSinglePipeDPPCLKAndSCLThroughput(mode_lib.vba.HRatio[k],
				mode_lib.vba.HRatioChroma[k],
				mode_lib.vba.VRatio[k],
				mode_lib.vba.VRatioChroma[k],
				mode_lib.vba.MaxDCHUBToPSCLThroughput,
				mode_lib.vba.MaxPSCLToLBThroughput,
				mode_lib.vba.PixelClock[k],
				mode_lib.vba.SourcePixelFormat[k],
				mode_lib.vba.htaps[k],
				mode_lib.vba.HTAPsChroma[k],
				mode_lib.vba.vtaps[k],
				mode_lib.vba.VTAPsChroma[k],

				/* Output */
				&v.PSCL_THROUGHPUT_LUMA[k], &v.PSCL_THROUGHPUT_CHROMA[k],
				&v.DPPCLKUsingSingleDPP[k]);
	}

	dml32_CalculateDPPCLK(mode_lib.vba.NumberOfActiveSurfaces, mode_lib.vba.DISPCLKDPPCLKDSCCLKDownSpreading,
			mode_lib.vba.DISPCLKDPPCLKVCOSpeed, v.DPPCLKUsingSingleDPP, mode_lib.vba.DPPPerPlane,
			/* Output */
			&v.GlobalDPPCLK, v.DPPCLK);

	for k in 0..v.NumberOfActiveSurfaces {
		v.DPPCLK_calculated[k] = v.DPPCLK[k];
	}

	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		dml32_CalculateBytePerPixelAndBlockSizes(
				mode_lib.vba.SourcePixelFormat[k],
				mode_lib.vba.SurfaceTiling[k],

				/* Output */
				&v.BytePerPixelY[k],
				&v.BytePerPixelC[k],
				&v.BytePerPixelDETY[k],
				&v.BytePerPixelDETC[k],
				&v.BlockHeight256BytesY[k],
				&v.BlockHeight256BytesC[k],
				&v.BlockWidth256BytesY[k],
				&v.BlockWidth256BytesC[k],
				&v.BlockHeightY[k],
				&v.BlockHeightC[k],
				&v.BlockWidthY[k],
				&v.BlockWidthC[k]);
	}

#ifdef __DML_VBA_DEBUG__
	dml_print("DML::%s: %d\n", file!, __LINE__);
#endif
	dml32_CalculateSwathWidth(
			false,  // ForceSingleDPP
			mode_lib.vba.NumberOfActiveSurfaces,
			mode_lib.vba.SourcePixelFormat,
			mode_lib.vba.SourceRotation,
			mode_lib.vba.ViewportStationary,
			mode_lib.vba.ViewportWidth,
			mode_lib.vba.ViewportHeight,
			mode_lib.vba.ViewportXStartY,
			mode_lib.vba.ViewportYStartY,
			mode_lib.vba.ViewportXStartC,
			mode_lib.vba.ViewportYStartC,
			mode_lib.vba.SurfaceWidthY,
			mode_lib.vba.SurfaceWidthC,
			mode_lib.vba.SurfaceHeightY,
			mode_lib.vba.SurfaceHeightC,
			mode_lib.vba.ODMCombineEnabled,
			v.BytePerPixelY,
			v.BytePerPixelC,
			v.BlockHeight256BytesY,
			v.BlockHeight256BytesC,
			v.BlockWidth256BytesY,
			v.BlockWidth256BytesC,
			mode_lib.vba.BlendingAndTiming,
			mode_lib.vba.HActive,
			mode_lib.vba.HRatio,
			mode_lib.vba.DPPPerPlane,

			/* Output */
			v.SwathWidthSingleDPPY, v.SwathWidthSingleDPPC, v.SwathWidthY, v.SwathWidthC,
			v.dummy_vars
				.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation
				.dummy_integer_array[0], // Integer             MaximumSwathHeightY[]
			v.dummy_vars
				.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation
				.dummy_integer_array[1], // Integer             MaximumSwathHeightC[]
			v.swath_width_luma_ub, v.swath_width_chroma_ub);

	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		v.ReadBandwidthSurfaceLuma[k] = v.SwathWidthSingleDPPY[k] * v.BytePerPixelY[k]
				/ (mode_lib.vba.HTotal[k] / mode_lib.vba.PixelClock[k]) * mode_lib.vba.VRatio[k];
		v.ReadBandwidthSurfaceChroma[k] = v.SwathWidthSingleDPPC[k] * v.BytePerPixelC[k]
				/ (mode_lib.vba.HTotal[k] / mode_lib.vba.PixelClock[k])
				* mode_lib.vba.VRatioChroma[k];
#ifdef __DML_VBA_DEBUG__
		dml_print("DML::%s: ReadBandwidthSurfaceLuma[%i] = %fBps\n",
				file!, k, v.ReadBandwidthSurfaceLuma[k]);
		dml_print("DML::%s: ReadBandwidthSurfaceChroma[%i] = %fBps\n",
				file!, k, v.ReadBandwidthSurfaceChroma[k]);
#endif
	}

	{
		// VBA_DELTA
		// Calculate DET size, swath height
		dml32_CalculateSwathAndDETConfiguration(
				mode_lib.vba.DETSizeOverride,
				mode_lib.vba.UsesMALLForPStateChange,
				mode_lib.vba.ConfigReturnBufferSizeInKByte,
				mode_lib.vba.MaxTotalDETInKByte,
				mode_lib.vba.MinCompressedBufferSizeInKByte,
				false, /* ForceSingleDPP */
				mode_lib.vba.NumberOfActiveSurfaces,
				mode_lib.vba.nomDETInKByte,
				mode_lib.vba.UseUnboundedRequesting,
				mode_lib.vba.DisableUnboundRequestIfCompBufReservedSpaceNeedAdjustment,
				mode_lib.vba.ip.pixel_chunk_size_kbytes,
				mode_lib.vba.ip.rob_buffer_size_kbytes,
				mode_lib.vba.CompressedBufferSegmentSizeInkByteFinal,
				v.dummy_vars
					.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation
					.dummy_output_encoder_array, /* output_encoder_class Output[] */
				v.ReadBandwidthSurfaceLuma,
				v.ReadBandwidthSurfaceChroma,
				v.dummy_vars
					.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation
					.dummy_single_array[0], /* Single MaximumSwathWidthLuma[] */
				v.dummy_vars
					.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation
					.dummy_single_array[1], /* Single MaximumSwathWidthChroma[] */
				mode_lib.vba.SourceRotation,
				mode_lib.vba.ViewportStationary,
				mode_lib.vba.SourcePixelFormat,
				mode_lib.vba.SurfaceTiling,
				mode_lib.vba.ViewportWidth,
				mode_lib.vba.ViewportHeight,
				mode_lib.vba.ViewportXStartY,
				mode_lib.vba.ViewportYStartY,
				mode_lib.vba.ViewportXStartC,
				mode_lib.vba.ViewportYStartC,
				mode_lib.vba.SurfaceWidthY,
				mode_lib.vba.SurfaceWidthC,
				mode_lib.vba.SurfaceHeightY,
				mode_lib.vba.SurfaceHeightC,
				v.BlockHeight256BytesY,
				v.BlockHeight256BytesC,
				v.BlockWidth256BytesY,
				v.BlockWidth256BytesC,
				mode_lib.vba.ODMCombineEnabled,
				mode_lib.vba.BlendingAndTiming,
				v.BytePerPixelY,
				v.BytePerPixelC,
				v.BytePerPixelDETY,
				v.BytePerPixelDETC,
				mode_lib.vba.HActive,
				mode_lib.vba.HRatio,
				mode_lib.vba.HRatioChroma,
				mode_lib.vba.DPPPerPlane,

				/* Output */
				v.dummy_vars
					.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation
					.dummy_long_array[0], /* Long swath_width_luma_ub[] */
				v.dummy_vars
					.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation
					.dummy_long_array[1], /* Long swath_width_chroma_ub[] */
				v.dummy_vars
					.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation
					.dummy_double_array[0], /* Long SwathWidth[] */
				v.dummy_vars
					.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation
					.dummy_double_array[1], /* Long SwathWidthChroma[] */
				mode_lib.vba.SwathHeightY,
				mode_lib.vba.SwathHeightC,
				mode_lib.vba.DETBufferSizeInKByte,
				mode_lib.vba.DETBufferSizeY,
				mode_lib.vba.DETBufferSizeC,
				&v.UnboundedRequestEnabled,
				&v.CompressedBufferSizeInkByte,
				&v.CompBufReservedSpaceKBytes,
				&v.dummy_vars
					.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation
					.dummy_boolean,       /* bool *CompBufReservedSpaceNeedAjustment */
				v.dummy_vars
					.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation
					.dummy_boolean_array, /* bool ViewportSizeSupportPerSurface[] */
				&v.dummy_vars
					 .DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation
					 .dummy_boolean); /* bool *ViewportSizeSupport */
	}

	v.CompBufReservedSpaceZs     = (unsigned int)(v.CompBufReservedSpaceKBytes * 1024.0 / 256.0);
	v.CompBufReservedSpace64B    = (unsigned int)(v.CompBufReservedSpaceKBytes * 1024.0 / 64.0);

	// DCFCLK Deep Sleep
	dml32_CalculateDCFCLKDeepSleep(
			mode_lib.vba.NumberOfActiveSurfaces,
			v.BytePerPixelY,
			v.BytePerPixelC,
			mode_lib.vba.VRatio,
			mode_lib.vba.VRatioChroma,
			v.SwathWidthY,
			v.SwathWidthC,
			mode_lib.vba.DPPPerPlane,
			mode_lib.vba.HRatio,
			mode_lib.vba.HRatioChroma,
			mode_lib.vba.PixelClock,
			v.PSCL_THROUGHPUT_LUMA,
			v.PSCL_THROUGHPUT_CHROMA,
			mode_lib.vba.DPPCLK,
			v.ReadBandwidthSurfaceLuma,
			v.ReadBandwidthSurfaceChroma,
			(int)mode_lib.vba.ReturnBusWidth,

			/* Output */
			&v.DCFCLKDeepSleep);

	// DSCCLK
	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		if (mode_lib.vba.BlendingAndTiming[k] != k) || !mode_lib.vba.DSCEnabled[k] {
			v.DSCCLK_calculated[k] = 0.0;
		} else {
			if mode_lib.vba.OutputFormat[k] == dm_420
				mode_lib.vba.DSCFormatFactor = 2;
			else if mode_lib.vba.OutputFormat[k] == dm_444
				mode_lib.vba.DSCFormatFactor = 1;
			else if mode_lib.vba.OutputFormat[k] == dm_n422
				mode_lib.vba.DSCFormatFactor = 2;
			else if mode_lib.vba.Output[k] == dm_hdmifrl
				mode_lib.vba.DSCFormatFactor = 2;
			else
				mode_lib.vba.DSCFormatFactor = 1;
			if mode_lib.vba.ODMCombineEnabled[k] == dm_odm_combine_mode_4to1
				v.DSCCLK_calculated[k] = mode_lib.vba.PixelClockBackEnd[k] / 12
						/ mode_lib.vba.DSCFormatFactor
						/ (1 - mode_lib.vba.DISPCLKDPPCLKDSCCLKDownSpreading / 100);
			else if mode_lib.vba.ODMCombineEnabled[k] == dm_odm_combine_mode_2to1
				v.DSCCLK_calculated[k] = mode_lib.vba.PixelClockBackEnd[k] / 6
						/ mode_lib.vba.DSCFormatFactor
						/ (1 - mode_lib.vba.DISPCLKDPPCLKDSCCLKDownSpreading / 100);
			else
				v.DSCCLK_calculated[k] = mode_lib.vba.PixelClockBackEnd[k] / 3
						/ mode_lib.vba.DSCFormatFactor
						/ (1 - mode_lib.vba.DISPCLKDPPCLKDSCCLKDownSpreading / 100);
		}
	}

	// DSC Delay
	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		v.DSCDelay[k] = dml32_DSCDelayRequirement(mode_lib.vba.DSCEnabled[k],
				mode_lib.vba.ODMCombineEnabled[k], mode_lib.vba.DSCInputBitPerComponent[k],
				mode_lib.vba.OutputBppPerState[mode_lib.vba.VoltageLevel][k],
				mode_lib.vba.HActive[k], mode_lib.vba.HTotal[k],
				mode_lib.vba.NumberOfDSCSlices[k], mode_lib.vba.OutputFormat[k],
				mode_lib.vba.Output[k], mode_lib.vba.PixelClock[k],
				mode_lib.vba.PixelClockBackEnd[k], mode_lib.vba.ip.dsc_delay_factor_wa);
	}

	for k in 0..mode_lib.vba.NumberOfActiveSurfaces
		for j in 0..mode_lib.vba.NumberOfActiveSurfaces // NumberOfSurfaces
			if j != k && mode_lib.vba.BlendingAndTiming[k] == j && mode_lib.vba.DSCEnabled[j]
				v.DSCDelay[k] = v.DSCDelay[j];

	//Immediate Flip
	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		v.ImmediateFlipSupportedSurface[k] = mode_lib.vba.ImmediateFlipSupport
				&& (mode_lib.vba.ImmediateFlipRequirement[k] != dm_immediate_flip_not_required);
	}

	// Prefetch
	dml32_CalculateSurfaceSizeInMall(
				mode_lib.vba.NumberOfActiveSurfaces,
				mode_lib.vba.MALLAllocatedForDCNFinal,
				mode_lib.vba.UseMALLForStaticScreen,
				mode_lib.vba.UsesMALLForPStateChange,
				mode_lib.vba.DCCEnable,
				mode_lib.vba.ViewportStationary,
				mode_lib.vba.ViewportXStartY,
				mode_lib.vba.ViewportYStartY,
				mode_lib.vba.ViewportXStartC,
				mode_lib.vba.ViewportYStartC,
				mode_lib.vba.ViewportWidth,
				mode_lib.vba.ViewportHeight,
				v.BytePerPixelY,
				mode_lib.vba.ViewportWidthChroma,
				mode_lib.vba.ViewportHeightChroma,
				v.BytePerPixelC,
				mode_lib.vba.SurfaceWidthY,
				mode_lib.vba.SurfaceWidthC,
				mode_lib.vba.SurfaceHeightY,
				mode_lib.vba.SurfaceHeightC,
				v.BlockWidth256BytesY,
				v.BlockWidth256BytesC,
				v.BlockHeight256BytesY,
				v.BlockHeight256BytesC,
				v.BlockWidthY,
				v.BlockWidthC,
				v.BlockHeightY,
				v.BlockHeightC,
				mode_lib.vba.DCCMetaPitchY,
				mode_lib.vba.DCCMetaPitchC,

				/* Output */
				v.SurfaceSizeInMALL,
				&v.dummy_vars.
				DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation
				.dummy_boolean2); /* Boolean *ExceededMALLSize */

	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].PixelClock = mode_lib.vba.PixelClock[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].DPPPerSurface = mode_lib.vba.DPPPerPlane[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].SourceRotation = mode_lib.vba.SourceRotation[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].ViewportHeight = mode_lib.vba.ViewportHeight[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].ViewportHeightChroma = mode_lib.vba.ViewportHeightChroma[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].BlockWidth256BytesY = v.BlockWidth256BytesY[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].BlockHeight256BytesY = v.BlockHeight256BytesY[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].BlockWidth256BytesC = v.BlockWidth256BytesC[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].BlockHeight256BytesC = v.BlockHeight256BytesC[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].BlockWidthY = v.BlockWidthY[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].BlockHeightY = v.BlockHeightY[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].BlockWidthC = v.BlockWidthC[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].BlockHeightC = v.BlockHeightC[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].InterlaceEnable = mode_lib.vba.Interlace[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].HTotal = mode_lib.vba.HTotal[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].DCCEnable = mode_lib.vba.DCCEnable[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].SourcePixelFormat = mode_lib.vba.SourcePixelFormat[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].SurfaceTiling = mode_lib.vba.SurfaceTiling[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].BytePerPixelY = v.BytePerPixelY[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].BytePerPixelC = v.BytePerPixelC[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].ProgressiveToInterlaceUnitInOPP = mode_lib.vba.let mut ProgressiveToInterlaceUnitInOPP = Default::default();
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].VRatio = mode_lib.vba.VRatio[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].VRatioChroma = mode_lib.vba.VRatioChroma[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].VTaps = mode_lib.vba.vtaps[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].VTapsChroma = mode_lib.vba.VTAPsChroma[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].PitchY = mode_lib.vba.PitchY[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].DCCMetaPitchY = mode_lib.vba.DCCMetaPitchY[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].PitchC = mode_lib.vba.PitchC[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].DCCMetaPitchC = mode_lib.vba.DCCMetaPitchC[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].ViewportStationary = mode_lib.vba.ViewportStationary[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].ViewportXStart = mode_lib.vba.ViewportXStartY[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].ViewportYStart = mode_lib.vba.ViewportYStartY[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].ViewportXStartC = mode_lib.vba.ViewportXStartC[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].ViewportYStartC = mode_lib.vba.ViewportYStartC[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].FORCE_ONE_ROW_FOR_FRAME = mode_lib.vba.ForceOneRowForFrame[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].SwathHeightY = mode_lib.vba.SwathHeightY[k];
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters[k].SwathHeightC = mode_lib.vba.SwathHeightC[k];
	}

	{

		dml32_CalculateVMRowAndSwath(
				mode_lib.vba.NumberOfActiveSurfaces,
				v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.SurfaceParameters,
				v.SurfaceSizeInMALL,
				mode_lib.vba.PTEBufferSizeInRequestsLuma,
				mode_lib.vba.PTEBufferSizeInRequestsChroma,
				mode_lib.vba.DCCMetaBufferSizeBytes,
				mode_lib.vba.UseMALLForStaticScreen,
				mode_lib.vba.UsesMALLForPStateChange,
				mode_lib.vba.MALLAllocatedForDCNFinal,
				v.SwathWidthY,
				v.SwathWidthC,
				mode_lib.vba.GPUVMEnable,
				mode_lib.vba.HostVMEnable,
				mode_lib.vba.HostVMMaxNonCachedPageTableLevels,
				mode_lib.vba.GPUVMMaxPageTableLevels,
				mode_lib.vba.GPUVMMinPageSizeKBytes,
				(unsigned int)mode_lib.vba.HostVMMinPageSize,

				/* Output */
				v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_boolean_array2[0],  // Boolean PTEBufferSizeNotExceeded[]
				v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_boolean_array2[1],  // Boolean DCCMetaBufferSizeNotExceeded[]
				v.dpte_row_width_luma_ub,
				v.dpte_row_width_chroma_ub,
				v.dpte_row_height,
				v.dpte_row_height_chroma,
				v.dpte_row_height_linear,
				v.dpte_row_height_linear_chroma,
				v.meta_req_width,
				v.meta_req_width_chroma,
				v.meta_req_height,
				v.meta_req_height_chroma,
				v.meta_row_width,
				v.meta_row_width_chroma,
				v.meta_row_height,
				v.meta_row_height_chroma,
				v.vm_group_bytes,
				v.dpte_group_bytes,
				v.PixelPTEReqWidthY,
				v.PixelPTEReqHeightY,
				v.PTERequestSizeY,
				v.PixelPTEReqWidthC,
				v.PixelPTEReqHeightC,
				v.PTERequestSizeC,
				v.dpde0_bytes_per_frame_ub_l,
				v.meta_pte_bytes_per_frame_ub_l,
				v.dpde0_bytes_per_frame_ub_c,
				v.meta_pte_bytes_per_frame_ub_c,
				v.PrefetchSourceLinesY,
				v.PrefetchSourceLinesC,
				v.VInitPreFillY, v.VInitPreFillC,
				v.MaxNumSwathY,
				v.MaxNumSwathC,
				v.meta_row_bw,
				v.dpte_row_bw,
				v.PixelPTEBytesPerRow,
				v.PDEAndMetaPTEBytesFrame,
				v.MetaRowByte,
				v.Use_One_Row_For_Frame,
				v.Use_One_Row_For_Frame_Flip,
				v.UsesMALLForStaticScreen,
				v.PTE_BUFFER_MODE,
				v.BIGK_FRAGMENT_SIZE);
	}


	v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.ReorderBytes = (unsigned int)(mode_lib.vba.NumberOfChannels
			* dml_max3(mode_lib.vba.UrgentOutOfOrderReturnPerChannelPixelDataOnly,
					mode_lib.vba.UrgentOutOfOrderReturnPerChannelPixelMixedWithVMData,
					mode_lib.vba.UrgentOutOfOrderReturnPerChannelVMDataOnly));

	v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.VMDataOnlyReturnBW = (unsigned int)dml32_get_return_bw_mbps_vm_only(
			&mode_lib.vba.soc,
			mode_lib.vba.VoltageLevel,
			mode_lib.vba.DCFCLK,
			mode_lib.vba.FabricClock,
			mode_lib.vba.DRAMSpeed);

#ifdef __DML_VBA_DEBUG__
	dml_print("DML::%s: mode_lib.vba.ReturnBusWidth = %f\n", file!, mode_lib.vba.ReturnBusWidth);
	dml_print("DML::%s: mode_lib.vba.DCFCLK = %f\n", file!, mode_lib.vba.DCFCLK);
	dml_print("DML::%s: mode_lib.vba.FabricClock = %f\n", file!, mode_lib.vba.FabricClock);
	dml_print("DML::%s: mode_lib.vba.FabricDatapathToDCNDataReturn = %f\n", file!,
			mode_lib.vba.FabricDatapathToDCNDataReturn);
	dml_print("DML::%s: mode_lib.vba.PercentOfIdealSDPPortBWReceivedAfterUrgLatency = %f\n",
			file!, mode_lib.vba.PercentOfIdealSDPPortBWReceivedAfterUrgLatency);
	dml_print("DML::%s: mode_lib.vba.DRAMSpeed = %f\n", file!, mode_lib.vba.DRAMSpeed);
	dml_print("DML::%s: mode_lib.vba.NumberOfChannels = %f\n", file!, mode_lib.vba.NumberOfChannels);
	dml_print("DML::%s: mode_lib.vba.DRAMChannelWidth = %f\n", file!, mode_lib.vba.DRAMChannelWidth);
	dml_print("DML::%s: mode_lib.vba.PercentOfIdealDRAMBWReceivedAfterUrgLatencyVMDataOnly = %f\n",
			file!, mode_lib.vba.PercentOfIdealDRAMBWReceivedAfterUrgLatencyVMDataOnly);
	dml_print("DML::%s: VMDataOnlyReturnBW = %f\n", file!, VMDataOnlyReturnBW);
	dml_print("DML::%s: ReturnBW = %f\n", file!, mode_lib.vba.ReturnBW);
#endif

	v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.HostVMInefficiencyFactor = 1.0;

	if mode_lib.vba.GPUVMEnable && mode_lib.vba.HostVMEnable
		v.dummy_vars
			.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation
			.HostVMInefficiencyFactor =
			mode_lib.vba.ReturnBW / v.dummy_vars
				.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation
				.let mut VMDataOnlyReturnBW = Default::default();

	mode_lib.vba.TotalDCCActiveDPP = 0;
	mode_lib.vba.TotalActiveDPP = 0;
	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		mode_lib.vba.TotalActiveDPP = mode_lib.vba.TotalActiveDPP + mode_lib.vba.DPPPerPlane[k];
		if mode_lib.vba.DCCEnable[k]
			mode_lib.vba.TotalDCCActiveDPP = mode_lib.vba.TotalDCCActiveDPP
					+ mode_lib.vba.DPPPerPlane[k];
	}

	v.UrgentExtraLatency = dml32_CalculateExtraLatency(
			(int)mode_lib.vba.RoundTripPingLatencyCycles,
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.ReorderBytes,
			mode_lib.vba.DCFCLK,
			mode_lib.vba.TotalActiveDPP,
			mode_lib.vba.PixelChunkSizeInKByte,
			mode_lib.vba.TotalDCCActiveDPP,
			mode_lib.vba.MetaChunkSize,
			mode_lib.vba.ReturnBW,
			mode_lib.vba.GPUVMEnable,
			mode_lib.vba.HostVMEnable,
			mode_lib.vba.NumberOfActiveSurfaces,
			mode_lib.vba.DPPPerPlane,
			v.dpte_group_bytes,
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.HostVMInefficiencyFactor,
			(unsigned int)mode_lib.vba.HostVMMinPageSize,
			mode_lib.vba.HostVMMaxNonCachedPageTableLevels);

	mode_lib.vba.TCalc = 24.0 / v.let mut DCFCLKDeepSleep = Default::default();

	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		if mode_lib.vba.BlendingAndTiming[k] == k {
			if mode_lib.vba.WritebackEnable[k] == true {
				v.WritebackDelay[mode_lib.vba.VoltageLevel][k] = mode_lib.vba.WritebackLatency
						+ dml32_CalculateWriteBackDelay(
								mode_lib.vba.WritebackPixelFormat[k],
								mode_lib.vba.WritebackHRatio[k],
								mode_lib.vba.WritebackVRatio[k],
								mode_lib.vba.WritebackVTaps[k],
								(long)mode_lib.vba.WritebackDestinationWidth[k],
								(int)mode_lib.vba.WritebackDestinationHeight[k],
								(int)mode_lib.vba.WritebackSourceHeight[k],
								mode_lib.vba.HTotal[k]) / mode_lib.vba.let mut DISPCLK = Default::default();
			} else
				v.WritebackDelay[mode_lib.vba.VoltageLevel][k] = 0;
			for j in 0..mode_lib.vba.NumberOfActiveSurfaces {
				if (mode_lib.vba.BlendingAndTiming[j] == k && mode_lib.vba.WritebackEnable[j] == true) {
					v.WritebackDelay[mode_lib.vba.VoltageLevel][k] =
						dml_max(v.WritebackDelay[mode_lib.vba.VoltageLevel][k],
						mode_lib.vba.WritebackLatency +
						dml32_CalculateWriteBackDelay(
								mode_lib.vba.WritebackPixelFormat[j],
								mode_lib.vba.WritebackHRatio[j],
								mode_lib.vba.WritebackVRatio[j],
								mode_lib.vba.WritebackVTaps[j],
								(int)mode_lib.vba.WritebackDestinationWidth[j],
								(int)mode_lib.vba.WritebackDestinationHeight[j],
								(int)mode_lib.vba.WritebackSourceHeight[j],
								mode_lib.vba.HTotal[k]) / mode_lib.vba.DISPCLK);
				}
			}
		}
	}

	for k in 0..mode_lib.vba.NumberOfActiveSurfaces
		for j in 0..mode_lib.vba.NumberOfActiveSurfaces
			if mode_lib.vba.BlendingAndTiming[k] == j
				v.WritebackDelay[mode_lib.vba.VoltageLevel][k] =
						v.WritebackDelay[mode_lib.vba.VoltageLevel][j];

	v.UrgentLatency = dml32_CalculateUrgentLatency(mode_lib.vba.UrgentLatencyPixelDataOnly,
			mode_lib.vba.UrgentLatencyPixelMixedWithVMData,
			mode_lib.vba.UrgentLatencyVMDataOnly,
			mode_lib.vba.DoUrgentLatencyAdjustment,
			mode_lib.vba.UrgentLatencyAdjustmentFabricClockComponent,
			mode_lib.vba.UrgentLatencyAdjustmentFabricClockReference,
			mode_lib.vba.FabricClock);

	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		dml32_CalculateUrgentBurstFactor(mode_lib.vba.UsesMALLForPStateChange[k],
				v.swath_width_luma_ub[k],
				v.swath_width_chroma_ub[k],
				mode_lib.vba.SwathHeightY[k],
				mode_lib.vba.SwathHeightC[k],
				mode_lib.vba.HTotal[k] / mode_lib.vba.PixelClock[k],
				v.UrgentLatency,
				mode_lib.vba.CursorBufferSize,
				mode_lib.vba.CursorWidth[k][0],
				mode_lib.vba.CursorBPP[k][0],
				mode_lib.vba.VRatio[k],
				mode_lib.vba.VRatioChroma[k],
				v.BytePerPixelDETY[k],
				v.BytePerPixelDETC[k],
				mode_lib.vba.DETBufferSizeY[k],
				mode_lib.vba.DETBufferSizeC[k],

				/* output */
				&v.UrgBurstFactorCursor[k],
				&v.UrgBurstFactorLuma[k],
				&v.UrgBurstFactorChroma[k],
				&v.NoUrgentLatencyHiding[k]);

		v.cursor_bw[k] = mode_lib.vba.NumberOfCursors[k] * mode_lib.vba.CursorWidth[k][0] * mode_lib.vba.CursorBPP[k][0] / 8 / (mode_lib.vba.HTotal[k] / mode_lib.vba.PixelClock[k]) * mode_lib.vba.VRatio[k];
	}

	v.NotEnoughDETSwathFillLatencyHiding = dml32_CalculateDETSwathFillLatencyHiding(
						mode_lib.vba.NumberOfActiveSurfaces,
						mode_lib.vba.ReturnBW,
						v.UrgentLatency,
						mode_lib.vba.SwathHeightY,
						mode_lib.vba.SwathHeightC,
						v.swath_width_luma_ub,
						v.swath_width_chroma_ub,
						v.BytePerPixelDETY,
						v.BytePerPixelDETC,
						mode_lib.vba.DETBufferSizeY,
						mode_lib.vba.DETBufferSizeC,
						mode_lib.vba.DPPPerPlane,
						mode_lib.vba.HTotal,
						mode_lib.vba.PixelClock,
						mode_lib.vba.VRatio,
						mode_lib.vba.VRatioChroma,
						mode_lib.vba.UsesMALLForPStateChange,
						mode_lib.vba.UseUnboundedRequesting);

	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		v.MaxVStartupLines[k] = (unsigned int)(((mode_lib.vba.Interlace[k] &&
				!mode_lib.vba.ProgressiveToInterlaceUnitInOPP) ?
				dml_floor((mode_lib.vba.VTotal[k] - mode_lib.vba.VActive[k]) / 2.0, 1.0) :
				mode_lib.vba.VTotal[k] - mode_lib.vba.VActive[k]) - dml_max(1.0,
				dml_ceil((double) v.WritebackDelay[mode_lib.vba.VoltageLevel][k]
				/ (mode_lib.vba.HTotal[k] / mode_lib.vba.PixelClock[k]), 1)));

		// Clamp to max OTG vstartup register limit
		if v.MaxVStartupLines[k] > 1023
			v.MaxVStartupLines[k] = 1023;

#ifdef __DML_VBA_DEBUG__
		dml_print("DML::%s: k=%d MaxVStartupLines = %d\n", file!, k, v.MaxVStartupLines[k]);
		dml_print("DML::%s: k=%d VoltageLevel = %d\n", file!, k, mode_lib.vba.VoltageLevel);
		dml_print("DML::%s: k=%d WritebackDelay = %f\n", file!,
				k, v.WritebackDelay[mode_lib.vba.VoltageLevel][k]);
#endif
	}

	v.MaximumMaxVStartupLines = 0;
	for k in 0..mode_lib.vba.NumberOfActiveSurfaces
		v.MaximumMaxVStartupLines = (unsigned int)dml_max(v.MaximumMaxVStartupLines, v.MaxVStartupLines[k]);

	ImmediateFlipRequirementFinal = let mut false = Default::default();

	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		ImmediateFlipRequirementFinal = ImmediateFlipRequirementFinal
				|| (mode_lib.vba.ImmediateFlipRequirement[k] == dm_immediate_flip_required);
	}
#ifdef __DML_VBA_DEBUG__
	dml_print("DML::%s: ImmediateFlipRequirementFinal = %d\n", file!, ImmediateFlipRequirementFinal);
#endif
	// ModeProgramming will not repeat the schedule calculation using different prefetch mode,
	//it is just calcualated once with given prefetch mode
	dml32_CalculateMinAndMaxPrefetchMode(
			mode_lib.vba.AllowForPStateChangeOrStutterInVBlankFinal,
			&mode_lib.vba.MinPrefetchMode,
			&mode_lib.vba.MaxPrefetchMode);

	v.VStartupLines = let mut __DML_VBA_MIN_VSTARTUP__ = Default::default();

	iteration = 0;
	MaxTotalRDBandwidth = 0;
	NextPrefetchMode = mode_lib.vba.PrefetchModePerState[mode_lib.vba.VoltageLevel][mode_lib.vba.maxMpcComb];

	do {
		MaxTotalRDBandwidth = 0;
		DestinationLineTimesForPrefetchLessThan2 = let mut false = Default::default();
		VRatioPrefetchMoreThanMax = let mut false = Default::default();
#ifdef __DML_VBA_DEBUG__
		dml_print("DML::%s: Start loop: VStartup = %d\n", file!, mode_lib.vba.VStartupLines);
#endif
		for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
			/* NOTE PerfetchMode variable is invalid in DAL as per the input received.
			 * Hence the direction is to use PrefetchModePerState.
			 */
			TWait = dml32_CalculateTWait(
				mode_lib.vba.PrefetchModePerState[mode_lib.vba.VoltageLevel][mode_lib.vba.maxMpcComb],
				mode_lib.vba.UsesMALLForPStateChange[k],
				mode_lib.vba.SynchronizeDRRDisplaysForUCLKPStateChangeFinal,
				mode_lib.vba.DRRDisplay[k],
				mode_lib.vba.DRAMClockChangeLatency,
				mode_lib.vba.FCLKChangeLatency, v.UrgentLatency,
				mode_lib.vba.SREnterPlusExitTime);

			memset(&v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe, 0, core::mem::size_of::<DmlPipe>());

			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.Dppclk = mode_lib.vba.DPPCLK[k];
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.Dispclk = mode_lib.vba.let mut DISPCLK = Default::default();
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.PixelClock = mode_lib.vba.PixelClock[k];
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.DCFClkDeepSleep = v.let mut DCFCLKDeepSleep = Default::default();
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.DPPPerSurface = mode_lib.vba.DPPPerPlane[k];
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.ScalerEnabled = mode_lib.vba.ScalerEnabled[k];
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.SourceRotation = mode_lib.vba.SourceRotation[k];
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.BlockWidth256BytesY = v.BlockWidth256BytesY[k];
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.BlockHeight256BytesY = v.BlockHeight256BytesY[k];
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.BlockWidth256BytesC = v.BlockWidth256BytesC[k];
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.BlockHeight256BytesC = v.BlockHeight256BytesC[k];
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.InterlaceEnable = mode_lib.vba.Interlace[k];
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.NumberOfCursors = mode_lib.vba.NumberOfCursors[k];
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.VBlank = mode_lib.vba.VTotal[k] - mode_lib.vba.VActive[k];
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.HTotal = mode_lib.vba.HTotal[k];
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.HActive = mode_lib.vba.HActive[k];
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.DCCEnable = mode_lib.vba.DCCEnable[k];
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.ODMMode = mode_lib.vba.ODMCombineEnabled[k];
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.SourcePixelFormat = mode_lib.vba.SourcePixelFormat[k];
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.BytePerPixelY = v.BytePerPixelY[k];
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.BytePerPixelC = v.BytePerPixelC[k];
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe.ProgressiveToInterlaceUnitInOPP = mode_lib.vba.let mut ProgressiveToInterlaceUnitInOPP = Default::default();
			v.ErrorResult[k] = dml32_CalculatePrefetchSchedule(
					v,
					k,
					v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.HostVMInefficiencyFactor,
					&v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.myPipe,
					v.DSCDelay[k],
					(unsigned int) (v.SwathWidthY[k] / v.HRatio[k]),
					(unsigned int)dml_min(v.VStartupLines, v.MaxVStartupLines[k]),
					v.MaxVStartupLines[k],
					v.UrgentLatency,
					v.UrgentExtraLatency,
					v.TCalc,
					(unsigned int)v.PDEAndMetaPTEBytesFrame[k],
					(unsigned int)v.MetaRowByte[k],
					(unsigned int)v.PixelPTEBytesPerRow[k],
					v.PrefetchSourceLinesY[k],
					(unsigned int)v.SwathWidthY[k],
					(unsigned int)v.VInitPreFillY[k],
					v.MaxNumSwathY[k],
					v.PrefetchSourceLinesC[k],
					(unsigned int)v.SwathWidthC[k],
					(unsigned int)v.VInitPreFillC[k],
					v.MaxNumSwathC[k],
					v.swath_width_luma_ub[k],
					v.swath_width_chroma_ub[k],
					v.SwathHeightY[k],
					v.SwathHeightC[k],
					TWait,
					(v.DRAMSpeedPerState[mode_lib.vba.VoltageLevel] <= MEM_STROBE_FREQ_MHZ || v.DCFCLKPerState[mode_lib.vba.VoltageLevel] <= DCFCLK_FREQ_EXTRA_PREFETCH_REQ_MHZ) ?
							mode_lib.vba.ip.min_prefetch_in_strobe_us : 0,
					mode_lib.vba.PrefetchModePerState[mode_lib.vba.VoltageLevel][mode_lib.vba.maxMpcComb] > 0 || mode_lib.vba.DRAMClockChangeRequirementFinal == false,

					/* Output */
					&v.DSTXAfterScaler[k],
					&v.DSTYAfterScaler[k],
					&v.DestinationLinesForPrefetch[k],
					&v.PrefetchBandwidth[k],
					&v.DestinationLinesToRequestVMInVBlank[k],
					&v.DestinationLinesToRequestRowInVBlank[k],
					&v.VRatioPrefetchY[k],
					&v.VRatioPrefetchC[k],
					&v.RequiredPrefetchPixDataBWLuma[k],
					&v.RequiredPrefetchPixDataBWChroma[k],
					&v.NotEnoughTimeForDynamicMetadata[k],
					&v.Tno_bw[k], &v.prefetch_vmrow_bw[k],
					&v.Tdmdl_vm[k],
					&v.Tdmdl[k],
					&v.TSetup[k],
					&v.VUpdateOffsetPix[k],
					&v.VUpdateWidthPix[k],
					&v.VReadyOffsetPix[k]);

#ifdef __DML_VBA_DEBUG__
			dml_print("DML::%s: k=%0d Prefetch calculation errResult=%0d\n",
					file!, k, mode_lib.vba.ErrorResult[k]);
#endif
			v.VStartup[k] = (unsigned int)dml_min(v.VStartupLines, v.MaxVStartupLines[k]);
		}

		for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
			dml32_CalculateUrgentBurstFactor(mode_lib.vba.UsesMALLForPStateChange[k],
					v.swath_width_luma_ub[k],
					v.swath_width_chroma_ub[k],
					mode_lib.vba.SwathHeightY[k],
					mode_lib.vba.SwathHeightC[k],
					mode_lib.vba.HTotal[k] / mode_lib.vba.PixelClock[k],
					v.UrgentLatency,
					mode_lib.vba.CursorBufferSize,
					mode_lib.vba.CursorWidth[k][0],
					mode_lib.vba.CursorBPP[k][0],
					v.VRatioPrefetchY[k],
					v.VRatioPrefetchC[k],
					v.BytePerPixelDETY[k],
					v.BytePerPixelDETC[k],
					mode_lib.vba.DETBufferSizeY[k],
					mode_lib.vba.DETBufferSizeC[k],
					/* Output */
					&v.UrgBurstFactorCursorPre[k],
					&v.UrgBurstFactorLumaPre[k],
					&v.UrgBurstFactorChromaPre[k],
					&v.NoUrgentLatencyHidingPre[k]);

			v.cursor_bw_pre[k] = mode_lib.vba.NumberOfCursors[k] * mode_lib.vba.CursorWidth[k][0] * mode_lib.vba.CursorBPP[k][0] /
					8.0 / (mode_lib.vba.HTotal[k] / mode_lib.vba.PixelClock[k]) * v.VRatioPrefetchY[k];

#ifdef __DML_VBA_DEBUG__
			dml_print("DML::%s: k=%0d DPPPerSurface=%d\n", file!, k, mode_lib.vba.DPPPerPlane[k]);
			dml_print("DML::%s: k=%0d UrgBurstFactorLuma=%f\n", file!, k, v.UrgBurstFactorLuma[k]);
			dml_print("DML::%s: k=%0d UrgBurstFactorChroma=%f\n", file!, k, v.UrgBurstFactorChroma[k]);
			dml_print("DML::%s: k=%0d UrgBurstFactorLumaPre=%f\n", file!, k,
					v.UrgBurstFactorLumaPre[k]);
			dml_print("DML::%s: k=%0d UrgBurstFactorChromaPre=%f\n", file!, k,
					v.UrgBurstFactorChromaPre[k]);

			dml_print("DML::%s: k=%0d VRatioPrefetchY=%f\n", file!, k, v.VRatioPrefetchY[k]);
			dml_print("DML::%s: k=%0d VRatioY=%f\n", file!, k, mode_lib.vba.VRatio[k]);

			dml_print("DML::%s: k=%0d prefetch_vmrow_bw=%f\n", file!, k, v.prefetch_vmrow_bw[k]);
			dml_print("DML::%s: k=%0d ReadBandwidthSurfaceLuma=%f\n", file!, k,
					v.ReadBandwidthSurfaceLuma[k]);
			dml_print("DML::%s: k=%0d ReadBandwidthSurfaceChroma=%f\n", file!, k,
					v.ReadBandwidthSurfaceChroma[k]);
			dml_print("DML::%s: k=%0d cursor_bw=%f\n", file!, k, v.cursor_bw[k]);
			dml_print("DML::%s: k=%0d meta_row_bw=%f\n", file!, k, v.meta_row_bw[k]);
			dml_print("DML::%s: k=%0d dpte_row_bw=%f\n", file!, k, v.dpte_row_bw[k]);
			dml_print("DML::%s: k=%0d RequiredPrefetchPixDataBWLuma=%f\n", file!, k,
					v.RequiredPrefetchPixDataBWLuma[k]);
			dml_print("DML::%s: k=%0d RequiredPrefetchPixDataBWChroma=%f\n", file!, k,
					v.RequiredPrefetchPixDataBWChroma[k]);
			dml_print("DML::%s: k=%0d cursor_bw_pre=%f\n", file!, k, v.cursor_bw_pre[k]);
			dml_print("DML::%s: k=%0d MaxTotalRDBandwidthNoUrgentBurst=%f\n", file!, k,
					MaxTotalRDBandwidthNoUrgentBurst);
#endif
			if v.DestinationLinesForPrefetch[k] < 2
				DestinationLineTimesForPrefetchLessThan2 = let mut true = Default::default();

			if (v.VRatioPrefetchY[k] > v.MaxVRatioPre || v.VRatioPrefetchC[k] > v.MaxVRatioPre)
				VRatioPrefetchMoreThanMax = let mut true = Default::default();

			//bool DestinationLinesToRequestVMInVBlankEqualOrMoreThan32 = let mut false = Default::default();
			//bool DestinationLinesToRequestRowInVBlankEqualOrMoreThan16 = let mut false = Default::default();
			//if v.DestinationLinesToRequestVMInVBlank[k] >= 32 {
			//    DestinationLinesToRequestVMInVBlankEqualOrMoreThan32 = let mut true = Default::default();
			//}

			//if v.DestinationLinesToRequestRowInVBlank[k] >= 16 {
			//    DestinationLinesToRequestRowInVBlankEqualOrMoreThan16 = let mut true = Default::default();
			//}
		}

		v.FractionOfUrgentBandwidth = MaxTotalRDBandwidthNoUrgentBurst / mode_lib.vba.let mut ReturnBW = Default::default();

#ifdef __DML_VBA_DEBUG__
		dml_print("DML::%s: MaxTotalRDBandwidthNoUrgentBurst=%f\n",
				file!, MaxTotalRDBandwidthNoUrgentBurst);
		dml_print("DML::%s: ReturnBW=%f\n", file!, mode_lib.vba.ReturnBW);
		dml_print("DML::%s: FractionOfUrgentBandwidth=%f\n",
				file!, mode_lib.vba.FractionOfUrgentBandwidth);
#endif

		{
			dml32_CalculatePrefetchBandwithSupport(
					mode_lib.vba.NumberOfActiveSurfaces,
					mode_lib.vba.ReturnBW,
					v.NoUrgentLatencyHidingPre,
					v.ReadBandwidthSurfaceLuma,
					v.ReadBandwidthSurfaceChroma,
					v.RequiredPrefetchPixDataBWLuma,
					v.RequiredPrefetchPixDataBWChroma,
					v.cursor_bw,
					v.meta_row_bw,
					v.dpte_row_bw,
					v.cursor_bw_pre,
					v.prefetch_vmrow_bw,
					mode_lib.vba.DPPPerPlane,
					v.UrgBurstFactorLuma,
					v.UrgBurstFactorChroma,
					v.UrgBurstFactorCursor,
					v.UrgBurstFactorLumaPre,
					v.UrgBurstFactorChromaPre,
					v.UrgBurstFactorCursorPre,
					v.PrefetchBandwidth,
					v.VRatio,
					v.MaxVRatioPre,

					/* output */
					&MaxTotalRDBandwidth,
					&v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_single[0],
					&v.PrefetchModeSupported);
		}

		for k in 0..mode_lib.vba.NumberOfActiveSurfaces
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_unit_vector[k] = 1.0;

		{
			dml32_CalculatePrefetchBandwithSupport(mode_lib.vba.NumberOfActiveSurfaces,
					mode_lib.vba.ReturnBW,
					v.NoUrgentLatencyHidingPre,
					v.ReadBandwidthSurfaceLuma,
					v.ReadBandwidthSurfaceChroma,
					v.RequiredPrefetchPixDataBWLuma,
					v.RequiredPrefetchPixDataBWChroma,
					v.cursor_bw,
					v.meta_row_bw,
					v.dpte_row_bw,
					v.cursor_bw_pre,
					v.prefetch_vmrow_bw,
					mode_lib.vba.DPPPerPlane,
					v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_unit_vector,
					v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_unit_vector,
					v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_unit_vector,
					v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_unit_vector,
					v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_unit_vector,
					v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_unit_vector,
					v.PrefetchBandwidth,
					v.VRatio,
					v.MaxVRatioPre,

					/* output */
					&v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_single[0],
					&v.FractionOfUrgentBandwidth,
					&v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_boolean);
		}

		if VRatioPrefetchMoreThanMax != false || DestinationLineTimesForPrefetchLessThan2 != false {
			v.PrefetchModeSupported = let mut false = Default::default();
		}

		for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
			if v.ErrorResult[k] == true || v.NotEnoughTimeForDynamicMetadata[k] {
				v.PrefetchModeSupported = let mut false = Default::default();
			}
		}

		if v.PrefetchModeSupported == true && mode_lib.vba.ImmediateFlipSupport == true {
			mode_lib.vba.BandwidthAvailableForImmediateFlip = dml32_CalculateBandwidthAvailableForImmediateFlip(
					mode_lib.vba.NumberOfActiveSurfaces,
					mode_lib.vba.ReturnBW,
					v.ReadBandwidthSurfaceLuma,
					v.ReadBandwidthSurfaceChroma,
					v.RequiredPrefetchPixDataBWLuma,
					v.RequiredPrefetchPixDataBWChroma,
					v.cursor_bw,
					v.cursor_bw_pre,
					mode_lib.vba.DPPPerPlane,
					v.UrgBurstFactorLuma,
					v.UrgBurstFactorChroma,
					v.UrgBurstFactorCursor,
					v.UrgBurstFactorLumaPre,
					v.UrgBurstFactorChromaPre,
					v.UrgBurstFactorCursorPre);

			mode_lib.vba.TotImmediateFlipBytes = 0;
			for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
				if mode_lib.vba.ImmediateFlipRequirement[k] != dm_immediate_flip_not_required {
					mode_lib.vba.TotImmediateFlipBytes = (unsigned int)(mode_lib.vba.TotImmediateFlipBytes
							+ mode_lib.vba.DPPPerPlane[k]
									* (v.PDEAndMetaPTEBytesFrame[k]
											+ v.MetaRowByte[k]));
					if v.use_one_row_for_frame_flip[k][0][0] {
						mode_lib.vba.TotImmediateFlipBytes =
								(unsigned int)(mode_lib.vba.TotImmediateFlipBytes
										+ 2 * v.PixelPTEBytesPerRow[k]);
					} else {
						mode_lib.vba.TotImmediateFlipBytes =
								(unsigned int)(mode_lib.vba.TotImmediateFlipBytes
										+ v.PixelPTEBytesPerRow[k]);
					}
				}
			}
			for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
				dml32_CalculateFlipSchedule(v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.HostVMInefficiencyFactor,
						v.UrgentExtraLatency,
						v.UrgentLatency,
						mode_lib.vba.GPUVMMaxPageTableLevels,
						mode_lib.vba.HostVMEnable,
						mode_lib.vba.HostVMMaxNonCachedPageTableLevels,
						mode_lib.vba.GPUVMEnable,
						(unsigned int)mode_lib.vba.HostVMMinPageSize,
						(unsigned int)v.PDEAndMetaPTEBytesFrame[k],
						(unsigned int)v.MetaRowByte[k],
						(unsigned int)v.PixelPTEBytesPerRow[k],
						mode_lib.vba.BandwidthAvailableForImmediateFlip,
						mode_lib.vba.TotImmediateFlipBytes,
						mode_lib.vba.SourcePixelFormat[k],
						mode_lib.vba.HTotal[k] / mode_lib.vba.PixelClock[k],
						mode_lib.vba.VRatio[k],
						mode_lib.vba.VRatioChroma[k],
						v.Tno_bw[k],
						mode_lib.vba.DCCEnable[k],
						v.dpte_row_height[k],
						v.meta_row_height[k],
						v.dpte_row_height_chroma[k],
						v.meta_row_height_chroma[k],
						v.Use_One_Row_For_Frame_Flip[k],

						/* Output */
						&v.DestinationLinesToRequestVMInImmediateFlip[k],
						&v.DestinationLinesToRequestRowInImmediateFlip[k],
						&v.final_flip_bw[k],
						&v.ImmediateFlipSupportedForPipe[k]);
			}

			{
				dml32_CalculateImmediateFlipBandwithSupport(mode_lib.vba.NumberOfActiveSurfaces,
						mode_lib.vba.ReturnBW,
						mode_lib.vba.ImmediateFlipRequirement,
						v.final_flip_bw,
						v.ReadBandwidthSurfaceLuma,
						v.ReadBandwidthSurfaceChroma,
						v.RequiredPrefetchPixDataBWLuma,
						v.RequiredPrefetchPixDataBWChroma,
						v.cursor_bw,
						v.meta_row_bw,
						v.dpte_row_bw,
						v.cursor_bw_pre,
						v.prefetch_vmrow_bw,
						mode_lib.vba.DPPPerPlane,
						v.UrgBurstFactorLuma,
						v.UrgBurstFactorChroma,
						v.UrgBurstFactorCursor,
						v.UrgBurstFactorLumaPre,
						v.UrgBurstFactorChromaPre,
						v.UrgBurstFactorCursorPre,

						/* output */
						&v.total_dcn_read_bw_with_flip,    // Single  *TotalBandwidth
						&v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_single[0],                        // Single  *FractionOfUrgentBandwidth
						&v.ImmediateFlipSupported);        // Boolean *ImmediateFlipBandwidthSupport

				dml32_CalculateImmediateFlipBandwithSupport(mode_lib.vba.NumberOfActiveSurfaces,
						mode_lib.vba.ReturnBW,
						mode_lib.vba.ImmediateFlipRequirement,
						v.final_flip_bw,
						v.ReadBandwidthSurfaceLuma,
						v.ReadBandwidthSurfaceChroma,
						v.RequiredPrefetchPixDataBWLuma,
						v.RequiredPrefetchPixDataBWChroma,
						v.cursor_bw,
						v.meta_row_bw,
						v.dpte_row_bw,
						v.cursor_bw_pre,
						v.prefetch_vmrow_bw,
						mode_lib.vba.DPPPerPlane,
						v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_unit_vector,
						v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_unit_vector,
						v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_unit_vector,
						v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_unit_vector,
						v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_unit_vector,
						v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_unit_vector,

						/* output */
						&v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_single[1],                                // Single  *TotalBandwidth
						&v.FractionOfUrgentBandwidthImmediateFlip, // Single  *FractionOfUrgentBandwidth
						&v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_boolean);                              // Boolean *ImmediateFlipBandwidthSupport
			}

			for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
				if mode_lib.vba.ImmediateFlipRequirement[k] != dm_immediate_flip_not_required && v.ImmediateFlipSupportedForPipe[k] == false {
					v.ImmediateFlipSupported = let mut false = Default::default();
#ifdef __DML_VBA_DEBUG__
					dml_print("DML::%s: Pipe %0d not supporting iflip\n", file!, k);
#endif
				}
			}
		} else {
			v.ImmediateFlipSupported = let mut false = Default::default();
		}

		/* consider flip support is okay if the flip bw is ok or (when user does't require a iflip and there is no host vm) */
		v.PrefetchAndImmediateFlipSupported = (v.PrefetchModeSupported == true &&
				((!mode_lib.vba.ImmediateFlipSupport && !mode_lib.vba.HostVMEnable && !ImmediateFlipRequirementFinal) ||
						v.ImmediateFlipSupported)) ? true : let mut false = Default::default();

#ifdef __DML_VBA_DEBUG__
		dml_print("DML::%s: PrefetchModeSupported = %d\n", file!, locals.PrefetchModeSupported);
		for (uint k = 0; k < mode_lib.vba.let mut NumberOfActiveSurfaces = Default::default(); ++k)
			dml_print("DML::%s: ImmediateFlipRequirement[%d] = %d\n", file!, k,  mode_lib.vba.ImmediateFlipRequirement[k] == dm_immediate_flip_required);
		dml_print("DML::%s: ImmediateFlipSupported = %d\n", file!, locals.ImmediateFlipSupported);
		dml_print("DML::%s: ImmediateFlipSupport = %d\n", file!, mode_lib.vba.ImmediateFlipSupport);
		dml_print("DML::%s: HostVMEnable = %d\n", file!, mode_lib.vba.HostVMEnable);
		dml_print("DML::%s: PrefetchAndImmediateFlipSupported = %d\n", file!, locals.PrefetchAndImmediateFlipSupported);
		dml_print("DML::%s: Done loop: Vstartup=%d, Max Vstartup=%d\n", file!, locals.VStartupLines, locals.MaximumMaxVStartupLines);
#endif

		v.VStartupLines = v.VStartupLines + 1;

		if v.VStartupLines > v.MaximumMaxVStartupLines {
#ifdef __DML_VBA_DEBUG__
			dml_print("DML::%s: Vstartup exceeds max vstartup, exiting loop\n", file!);
#endif
			let mut break = Default::default(); // VBA_DELTA: Implementation divergence! Gabe is *still* iterating across prefetch modes which we don't care to do
		}
		iteration += 1;
		if iteration > 2500 {
#ifdef __DML_VBA_DEBUG__
			dml_print("DML::%s: too many errors, exit now\n", file!);
			assert(0);
#endif
		}
	} while !(v.PrefetchAndImmediateFlipSupported || NextPrefetchMode > mode_lib.vba.MaxPrefetchMode);


	if v.VStartupLines <= v.MaximumMaxVStartupLines {
#ifdef __DML_VBA_DEBUG__
		dml_print("DML::%s: Good, Prefetch and flip scheduling found solution at VStartupLines=%d\n", file!, locals.VStartupLines-1);
#endif
	}


	//Watermarks and NB P-State/DRAM Clock Change Support
	{
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.mmSOCParameters.UrgentLatency = v.let mut UrgentLatency = Default::default();
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.mmSOCParameters.ExtraLatency = v.let mut UrgentExtraLatency = Default::default();
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.mmSOCParameters.WritebackLatency = mode_lib.vba.let mut WritebackLatency = Default::default();
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.mmSOCParameters.DRAMClockChangeLatency = mode_lib.vba.let mut DRAMClockChangeLatency = Default::default();
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.mmSOCParameters.FCLKChangeLatency = mode_lib.vba.let mut FCLKChangeLatency = Default::default();
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.mmSOCParameters.SRExitTime = mode_lib.vba.let mut SRExitTime = Default::default();
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.mmSOCParameters.SREnterPlusExitTime = mode_lib.vba.let mut SREnterPlusExitTime = Default::default();
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.mmSOCParameters.SRExitZ8Time = mode_lib.vba.let mut SRExitZ8Time = Default::default();
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.mmSOCParameters.SREnterPlusExitZ8Time = mode_lib.vba.let mut SREnterPlusExitZ8Time = Default::default();
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.mmSOCParameters.USRRetrainingLatency = mode_lib.vba.let mut USRRetrainingLatency = Default::default();
		v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.mmSOCParameters.SMNLatency = mode_lib.vba.let mut SMNLatency = Default::default();

		dml32_CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport(
			v,
			v.PrefetchModePerState[v.VoltageLevel][v.maxMpcComb],
			v.DCFCLK,
			v.ReturnBW,
			v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.mmSOCParameters,
			v.SOCCLK,
			v.DCFCLKDeepSleep,
			v.DETBufferSizeY,
			v.DETBufferSizeC,
			v.SwathHeightY,
			v.SwathHeightC,
			v.SwathWidthY,
			v.SwathWidthC,
			v.DPPPerPlane,
			v.BytePerPixelDETY,
			v.BytePerPixelDETC,
			v.DSTXAfterScaler,
			v.DSTYAfterScaler,
			v.UnboundedRequestEnabled,
			v.CompressedBufferSizeInkByte,

			/* Output */
			&v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_dramchange_support,
			v.MaxActiveDRAMClockChangeLatencySupported,
			v.SubViewportLinesNeededInMALL,
			&v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_fclkchange_support,
			&v.MinActiveFCLKChangeLatencySupported,
			&v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_USRRetrainingSupport,
			mode_lib.vba.ActiveDRAMClockChangeLatencyMargin);

		/* DCN32 has a new // struct Watermarks (typedef) which is used to store
		 * calculated WM values. Copy over values from // struct to vba varaibles
		 * to ensure that the DCN32 getters return the correct value.
		 */
		v.UrgentWatermark = v.Watermark.let mut UrgentWatermark = Default::default();
		v.WritebackUrgentWatermark = v.Watermark.let mut WritebackUrgentWatermark = Default::default();
		v.DRAMClockChangeWatermark = v.Watermark.let mut DRAMClockChangeWatermark = Default::default();
		v.WritebackDRAMClockChangeWatermark = v.Watermark.let mut WritebackDRAMClockChangeWatermark = Default::default();
		v.StutterExitWatermark = v.Watermark.let mut StutterExitWatermark = Default::default();
		v.StutterEnterPlusExitWatermark = v.Watermark.let mut StutterEnterPlusExitWatermark = Default::default();
		v.Z8StutterExitWatermark = v.Watermark.let mut Z8StutterExitWatermark = Default::default();
		v.Z8StutterEnterPlusExitWatermark = v.Watermark.let mut Z8StutterEnterPlusExitWatermark = Default::default();

		for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
			if mode_lib.vba.WritebackEnable[k] == true {
				v.WritebackAllowDRAMClockChangeEndPosition[k] = dml_max(0,
						v.VStartup[k] * mode_lib.vba.HTotal[k] / mode_lib.vba.PixelClock[k]
								- v.Watermark.WritebackDRAMClockChangeWatermark);
				v.WritebackAllowFCLKChangeEndPosition[k] = dml_max(0,
						v.VStartup[k] * mode_lib.vba.HTotal[k] / mode_lib.vba.PixelClock[k]
								- v.Watermark.WritebackFCLKChangeWatermark);
			} else {
				v.WritebackAllowDRAMClockChangeEndPosition[k] = 0;
				v.WritebackAllowFCLKChangeEndPosition[k] = 0;
			}
		}
	}

	//Display Pipeline Delivery Time in Prefetch, Groups
	dml32_CalculatePixelDeliveryTimes(
			mode_lib.vba.NumberOfActiveSurfaces,
			mode_lib.vba.VRatio,
			mode_lib.vba.VRatioChroma,
			v.VRatioPrefetchY,
			v.VRatioPrefetchC,
			v.swath_width_luma_ub,
			v.swath_width_chroma_ub,
			mode_lib.vba.DPPPerPlane,
			mode_lib.vba.HRatio,
			mode_lib.vba.HRatioChroma,
			mode_lib.vba.PixelClock,
			v.PSCL_THROUGHPUT_LUMA,
			v.PSCL_THROUGHPUT_CHROMA,
			mode_lib.vba.DPPCLK,
			v.BytePerPixelC,
			mode_lib.vba.SourceRotation,
			mode_lib.vba.NumberOfCursors,
			mode_lib.vba.CursorWidth,
			mode_lib.vba.CursorBPP,
			v.BlockWidth256BytesY,
			v.BlockHeight256BytesY,
			v.BlockWidth256BytesC,
			v.BlockHeight256BytesC,

			/* Output */
			v.DisplayPipeLineDeliveryTimeLuma,
			v.DisplayPipeLineDeliveryTimeChroma,
			v.DisplayPipeLineDeliveryTimeLumaPrefetch,
			v.DisplayPipeLineDeliveryTimeChromaPrefetch,
			v.DisplayPipeRequestDeliveryTimeLuma,
			v.DisplayPipeRequestDeliveryTimeChroma,
			v.DisplayPipeRequestDeliveryTimeLumaPrefetch,
			v.DisplayPipeRequestDeliveryTimeChromaPrefetch,
			v.CursorRequestDeliveryTime,
			v.CursorRequestDeliveryTimePrefetch);

	dml32_CalculateMetaAndPTETimes(v.Use_One_Row_For_Frame,
			mode_lib.vba.NumberOfActiveSurfaces,
			mode_lib.vba.GPUVMEnable,
			mode_lib.vba.MetaChunkSize,
			mode_lib.vba.MinMetaChunkSizeBytes,
			mode_lib.vba.HTotal,
			mode_lib.vba.VRatio,
			mode_lib.vba.VRatioChroma,
			v.DestinationLinesToRequestRowInVBlank,
			v.DestinationLinesToRequestRowInImmediateFlip,
			mode_lib.vba.DCCEnable,
			mode_lib.vba.PixelClock,
			v.BytePerPixelY,
			v.BytePerPixelC,
			mode_lib.vba.SourceRotation,
			v.dpte_row_height,
			v.dpte_row_height_chroma,
			v.meta_row_width,
			v.meta_row_width_chroma,
			v.meta_row_height,
			v.meta_row_height_chroma,
			v.meta_req_width,
			v.meta_req_width_chroma,
			v.meta_req_height,
			v.meta_req_height_chroma,
			v.dpte_group_bytes,
			v.PTERequestSizeY,
			v.PTERequestSizeC,
			v.PixelPTEReqWidthY,
			v.PixelPTEReqHeightY,
			v.PixelPTEReqWidthC,
			v.PixelPTEReqHeightC,
			v.dpte_row_width_luma_ub,
			v.dpte_row_width_chroma_ub,

			/* Output */
			v.DST_Y_PER_PTE_ROW_NOM_L,
			v.DST_Y_PER_PTE_ROW_NOM_C,
			v.DST_Y_PER_META_ROW_NOM_L,
			v.DST_Y_PER_META_ROW_NOM_C,
			v.TimePerMetaChunkNominal,
			v.TimePerChromaMetaChunkNominal,
			v.TimePerMetaChunkVBlank,
			v.TimePerChromaMetaChunkVBlank,
			v.TimePerMetaChunkFlip,
			v.TimePerChromaMetaChunkFlip,
			v.time_per_pte_group_nom_luma,
			v.time_per_pte_group_vblank_luma,
			v.time_per_pte_group_flip_luma,
			v.time_per_pte_group_nom_chroma,
			v.time_per_pte_group_vblank_chroma,
			v.time_per_pte_group_flip_chroma);

	dml32_CalculateVMGroupAndRequestTimes(
			mode_lib.vba.NumberOfActiveSurfaces,
			mode_lib.vba.GPUVMEnable,
			mode_lib.vba.GPUVMMaxPageTableLevels,
			mode_lib.vba.HTotal,
			v.BytePerPixelC,
			v.DestinationLinesToRequestVMInVBlank,
			v.DestinationLinesToRequestVMInImmediateFlip,
			mode_lib.vba.DCCEnable,
			mode_lib.vba.PixelClock,
			v.dpte_row_width_luma_ub,
			v.dpte_row_width_chroma_ub,
			v.vm_group_bytes,
			v.dpde0_bytes_per_frame_ub_l,
			v.dpde0_bytes_per_frame_ub_c,
			v.meta_pte_bytes_per_frame_ub_l,
			v.meta_pte_bytes_per_frame_ub_c,

			/* Output */
			v.TimePerVMGroupVBlank,
			v.TimePerVMGroupFlip,
			v.TimePerVMRequestVBlank,
			v.TimePerVMRequestFlip);

	// Min TTUVBlank
	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		if mode_lib.vba.PrefetchModePerState[mode_lib.vba.VoltageLevel][mode_lib.vba.maxMpcComb] == 0 {
			v.MinTTUVBlank[k] = dml_max4(v.Watermark.DRAMClockChangeWatermark,
					v.Watermark.FCLKChangeWatermark, v.Watermark.StutterEnterPlusExitWatermark,
					v.Watermark.UrgentWatermark);
		} else if (mode_lib.vba.PrefetchModePerState[mode_lib.vba.VoltageLevel][mode_lib.vba.maxMpcComb]
				== 1) {
			v.MinTTUVBlank[k] = dml_max3(v.Watermark.FCLKChangeWatermark,
					v.Watermark.StutterEnterPlusExitWatermark, v.Watermark.UrgentWatermark);
		} else if (mode_lib.vba.PrefetchModePerState[mode_lib.vba.VoltageLevel][mode_lib.vba.maxMpcComb]
				== 2) {
			v.MinTTUVBlank[k] = dml_max(v.Watermark.StutterEnterPlusExitWatermark,
					v.Watermark.UrgentWatermark);
		} else {
			v.MinTTUVBlank[k] = v.Watermark.let mut UrgentWatermark = Default::default();
		}
		if !mode_lib.vba.DynamicMetadataEnable[k]
			v.MinTTUVBlank[k] = mode_lib.vba.TCalc + v.MinTTUVBlank[k];
	}

	// DCC Configuration
	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
#ifdef __DML_VBA_DEBUG__
		dml_print("DML::%s: Calculate DCC configuration for surface k=%d\n", file!, k);
#endif
		dml32_CalculateDCCConfiguration(
				mode_lib.vba.DCCEnable[k],
				mode_lib.vba.DCCProgrammingAssumesScanDirectionUnknownFinal,
				mode_lib.vba.SourcePixelFormat[k], mode_lib.vba.SurfaceWidthY[k],
				mode_lib.vba.SurfaceWidthC[k],
				mode_lib.vba.SurfaceHeightY[k],
				mode_lib.vba.SurfaceHeightC[k],
				mode_lib.vba.nomDETInKByte,
				v.BlockHeight256BytesY[k],
				v.BlockHeight256BytesC[k],
				mode_lib.vba.SurfaceTiling[k],
				v.BytePerPixelY[k],
				v.BytePerPixelC[k],
				v.BytePerPixelDETY[k],
				v.BytePerPixelDETC[k],
				(enum dm_rotation_angle) mode_lib.vba.SourceScan[k],
				/* Output */
				&v.DCCYMaxUncompressedBlock[k],
				&v.DCCCMaxUncompressedBlock[k],
				&v.DCCYMaxCompressedBlock[k],
				&v.DCCCMaxCompressedBlock[k],
				&v.DCCYIndependentBlock[k],
				&v.DCCCIndependentBlock[k]);
	}

	// VStartup Adjustment
	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		bool let mut isInterlaceTiming = Default::default();
		double Tvstartup_margin = (v.MaxVStartupLines[k] - v.VStartup[k]) * mode_lib.vba.HTotal[k]
				/ mode_lib.vba.PixelClock[k];
#ifdef __DML_VBA_DEBUG__
		dml_print("DML::%s: k=%d, MinTTUVBlank = %f (before vstartup margin)\n", file!, k,
				v.MinTTUVBlank[k]);
#endif

		v.MinTTUVBlank[k] = v.MinTTUVBlank[k] + let mut Tvstartup_margin = Default::default();

#ifdef __DML_VBA_DEBUG__
		dml_print("DML::%s: k=%d, Tvstartup_margin = %f\n", file!, k, Tvstartup_margin);
		dml_print("DML::%s: k=%d, MaxVStartupLines = %d\n", file!, k, v.MaxVStartupLines[k]);
		dml_print("DML::%s: k=%d, VStartup = %d\n", file!, k, v.VStartup[k]);
		dml_print("DML::%s: k=%d, MinTTUVBlank = %f\n", file!, k, v.MinTTUVBlank[k]);
#endif

		v.Tdmdl[k] = v.Tdmdl[k] + let mut Tvstartup_margin = Default::default();
		if mode_lib.vba.DynamicMetadataEnable[k] && mode_lib.vba.DynamicMetadataVMEnabled
			v.Tdmdl_vm[k] = v.Tdmdl_vm[k] + let mut Tvstartup_margin = Default::default();

		isInterlaceTiming = (mode_lib.vba.Interlace[k] &&
				!mode_lib.vba.ProgressiveToInterlaceUnitInOPP);

		v.MIN_DST_Y_NEXT_START[k] = (unsigned int)(((isInterlaceTiming ? dml_floor((mode_lib.vba.VTotal[k] -
						mode_lib.vba.VFrontPorch[k]) / 2.0, 1.0) :
						mode_lib.vba.VTotal[k]) - mode_lib.vba.VFrontPorch[k])
						+ dml_max(1.0,
						dml_ceil(v.WritebackDelay[mode_lib.vba.VoltageLevel][k]
						/ (mode_lib.vba.HTotal[k] / mode_lib.vba.PixelClock[k]), 1.0))
						+ dml_floor(4.0 * v.TSetup[k] / (mode_lib.vba.HTotal[k]
						/ mode_lib.vba.PixelClock[k]), 1.0) / 4.0);

		v.VStartup[k] = (isInterlaceTiming ? (2 * v.MaxVStartupLines[k]) : v.MaxVStartupLines[k]);

		if ((v.VUpdateOffsetPix[k] + v.VUpdateWidthPix[k] + v.VReadyOffsetPix[k]
			/ mode_lib.vba.HTotal[k]) <= (isInterlaceTiming ? dml_floor((mode_lib.vba.VTotal[k]
			- mode_lib.vba.VActive[k] - mode_lib.vba.VFrontPorch[k] - v.VStartup[k]) / 2.0, 1.0) :
			(int) (mode_lib.vba.VTotal[k] - mode_lib.vba.VActive[k]
		       - mode_lib.vba.VFrontPorch[k] - v.VStartup[k]))) {
			v.VREADY_AT_OR_AFTER_VSYNC[k] = let mut true = Default::default();
		} else {
			v.VREADY_AT_OR_AFTER_VSYNC[k] = let mut false = Default::default();
		}
#ifdef __DML_VBA_DEBUG__
		dml_print("DML::%s: k=%d, VStartup = %d (max)\n", file!, k, v.VStartup[k]);
		dml_print("DML::%s: k=%d, VUpdateOffsetPix = %d\n", file!, k, v.VUpdateOffsetPix[k]);
		dml_print("DML::%s: k=%d, VUpdateWidthPix = %d\n", file!, k, v.VUpdateWidthPix[k]);
		dml_print("DML::%s: k=%d, VReadyOffsetPix = %d\n", file!, k, v.VReadyOffsetPix[k]);
		dml_print("DML::%s: k=%d, HTotal = %d\n", file!, k, mode_lib.vba.HTotal[k]);
		dml_print("DML::%s: k=%d, VTotal = %d\n", file!, k, mode_lib.vba.VTotal[k]);
		dml_print("DML::%s: k=%d, VActive = %d\n", file!, k, mode_lib.vba.VActive[k]);
		dml_print("DML::%s: k=%d, VFrontPorch = %d\n", file!, k, mode_lib.vba.VFrontPorch[k]);
		dml_print("DML::%s: k=%d, VStartup = %d\n", file!, k, v.VStartup[k]);
		dml_print("DML::%s: k=%d, TSetup = %f\n", file!, k, v.TSetup[k]);
		dml_print("DML::%s: k=%d, MIN_DST_Y_NEXT_START = %f\n", file!, k, v.MIN_DST_Y_NEXT_START[k]);
		dml_print("DML::%s: k=%d, VREADY_AT_OR_AFTER_VSYNC = %d\n", file!, k,
				v.VREADY_AT_OR_AFTER_VSYNC[k]);
#endif
	}

	{
		//Maximum Bandwidth Used
		for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
			if (mode_lib.vba.WritebackEnable[k] == true && mode_lib.vba.WritebackPixelFormat[k] == dm_444_32) {
				WRBandwidth = mode_lib.vba.WritebackDestinationWidth[k]
						* mode_lib.vba.WritebackDestinationHeight[k]
						/ (mode_lib.vba.HTotal[k] * mode_lib.vba.WritebackSourceHeight[k]
								/ mode_lib.vba.PixelClock[k]) * 4;
			} else if mode_lib.vba.WritebackEnable[k] == true {
				WRBandwidth = mode_lib.vba.WritebackDestinationWidth[k]
						* mode_lib.vba.WritebackDestinationHeight[k]
						/ (mode_lib.vba.HTotal[k] * mode_lib.vba.WritebackSourceHeight[k]
								/ mode_lib.vba.PixelClock[k]) * 8;
			}
			TotalWRBandwidth = TotalWRBandwidth + let mut WRBandwidth = Default::default();
		}

		v.TotalDataReadBandwidth = 0;
		for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
			v.TotalDataReadBandwidth = v.TotalDataReadBandwidth + v.ReadBandwidthSurfaceLuma[k]
					+ v.ReadBandwidthSurfaceChroma[k];
#ifdef __DML_VBA_DEBUG__
			dml_print("DML::%s: k=%d, TotalDataReadBandwidth = %f\n",
					file!, k, v.TotalDataReadBandwidth);
			dml_print("DML::%s: k=%d, ReadBandwidthSurfaceLuma = %f\n",
					file!, k, v.ReadBandwidthSurfaceLuma[k]);
			dml_print("DML::%s: k=%d, ReadBandwidthSurfaceChroma = %f\n",
					file!, k, v.ReadBandwidthSurfaceChroma[k]);
#endif
		}
	}

	// Stutter Efficiency
	dml32_CalculateStutterEfficiency(v.CompressedBufferSizeInkByte,
			mode_lib.vba.UsesMALLForPStateChange,
			v.UnboundedRequestEnabled,
			mode_lib.vba.MetaFIFOSizeInKEntries,
			mode_lib.vba.ZeroSizeBufferEntries,
			mode_lib.vba.PixelChunkSizeInKByte,
			mode_lib.vba.NumberOfActiveSurfaces,
			mode_lib.vba.ROBBufferSizeInKByte,
			v.TotalDataReadBandwidth,
			mode_lib.vba.DCFCLK,
			mode_lib.vba.ReturnBW,
			v.CompbufReservedSpace64B,
			v.CompbufReservedSpaceZs,
			mode_lib.vba.SRExitTime,
			mode_lib.vba.SRExitZ8Time,
			mode_lib.vba.SynchronizeTimingsFinal,
			mode_lib.vba.BlendingAndTiming,
			v.Watermark.StutterEnterPlusExitWatermark,
			v.Watermark.Z8StutterEnterPlusExitWatermark,
			mode_lib.vba.ProgressiveToInterlaceUnitInOPP,
			mode_lib.vba.Interlace,
			v.MinTTUVBlank, mode_lib.vba.DPPPerPlane,
			mode_lib.vba.DETBufferSizeY,
			v.BytePerPixelY,
			v.BytePerPixelDETY,
			v.SwathWidthY,
			mode_lib.vba.SwathHeightY,
			mode_lib.vba.SwathHeightC,
			mode_lib.vba.DCCRateLuma,
			mode_lib.vba.DCCRateChroma,
			mode_lib.vba.DCCFractionOfZeroSizeRequestsLuma,
			mode_lib.vba.DCCFractionOfZeroSizeRequestsChroma,
			mode_lib.vba.HTotal, mode_lib.vba.VTotal,
			mode_lib.vba.PixelClock,
			mode_lib.vba.VRatio,
			mode_lib.vba.SourceRotation,
			v.BlockHeight256BytesY,
			v.BlockWidth256BytesY,
			v.BlockHeight256BytesC,
			v.BlockWidth256BytesC,
			v.DCCYMaxUncompressedBlock,
			v.DCCCMaxUncompressedBlock,
			mode_lib.vba.VActive,
			mode_lib.vba.DCCEnable,
			mode_lib.vba.WritebackEnable,
			v.ReadBandwidthSurfaceLuma,
			v.ReadBandwidthSurfaceChroma,
			v.meta_row_bw,
			v.dpte_row_bw,
			/* Output */
			&v.StutterEfficiencyNotIncludingVBlank,
			&v.StutterEfficiency,
			&v.NumberOfStutterBurstsPerFrame,
			&v.Z8StutterEfficiencyNotIncludingVBlank,
			&v.Z8StutterEfficiency,
			&v.Z8NumberOfStutterBurstsPerFrame,
			&v.StutterPeriod,
			&v.DCHUBBUB_ARB_CSTATE_MAX_CAP_MODE);

#ifdef __DML_VBA_ALLOW_DELTA__
	{
		unsigned int dummy_integer[1];

		// Calculate z8 stutter eff assuming 0 reserved space
		dml32_CalculateStutterEfficiency(v.CompressedBufferSizeInkByte,
				mode_lib.vba.UsesMALLForPStateChange,
				v.UnboundedRequestEnabled,
				mode_lib.vba.MetaFIFOSizeInKEntries,
				mode_lib.vba.ZeroSizeBufferEntries,
				mode_lib.vba.PixelChunkSizeInKByte,
				mode_lib.vba.NumberOfActiveSurfaces,
				mode_lib.vba.ROBBufferSizeInKByte,
				v.TotalDataReadBandwidth,
				mode_lib.vba.DCFCLK,
				mode_lib.vba.ReturnBW,
				0, //CompbufReservedSpace64B,
				0, //CompbufReservedSpaceZs,
				mode_lib.vba.SRExitTime,
				mode_lib.vba.SRExitZ8Time,
				mode_lib.vba.SynchronizeTimingsFinal,
				mode_lib.vba.BlendingAndTiming,
				v.Watermark.StutterEnterPlusExitWatermark,
				v.Watermark.Z8StutterEnterPlusExitWatermark,
				mode_lib.vba.ProgressiveToInterlaceUnitInOPP,
				mode_lib.vba.Interlace,
				v.MinTTUVBlank,
				mode_lib.vba.DPPPerPlane,
				mode_lib.vba.DETBufferSizeY,
				v.BytePerPixelY, v.BytePerPixelDETY,
				v.SwathWidthY, mode_lib.vba.SwathHeightY,
				mode_lib.vba.SwathHeightC,
				mode_lib.vba.DCCRateLuma,
				mode_lib.vba.DCCRateChroma,
				mode_lib.vba.DCCFractionOfZeroSizeRequestsLuma,
				mode_lib.vba.DCCFractionOfZeroSizeRequestsChroma,
				mode_lib.vba.HTotal,
				mode_lib.vba.VTotal,
				mode_lib.vba.PixelClock,
				mode_lib.vba.VRatio,
				mode_lib.vba.SourceRotation,
				v.BlockHeight256BytesY,
				v.BlockWidth256BytesY,
				v.BlockHeight256BytesC,
				v.BlockWidth256BytesC,
				v.DCCYMaxUncompressedBlock,
				v.DCCCMaxUncompressedBlock,
				mode_lib.vba.VActive,
				mode_lib.vba.DCCEnable,
				mode_lib.vba.WritebackEnable,
				v.ReadBandwidthSurfaceLuma,
				v.ReadBandwidthSurfaceChroma,
				v.meta_row_bw, v.dpte_row_bw,

				/* Output */
				&v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_single[0],
				&v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_single[1],
				&dummy_integer[0],
				&v.Z8StutterEfficiencyNotIncludingVBlankBestCase,
				&v.Z8StutterEfficiencyBestCase,
				&v.Z8NumberOfStutterBurstsPerFrameBestCase,
				&v.StutterPeriodBestCase,
				&v.dummy_vars.DISPCLKDPPCLKDCFCLKDeepSleepPrefetchParametersWatermarksAndPerformanceCalculation.dummy_boolean);
	}
#else
	v.Z8StutterEfficiencyNotIncludingVBlankBestCase = v.let mut Z8StutterEfficiencyNotIncludingVBlank = Default::default();
	v.Z8StutterEfficiencyBestCase = v.let mut Z8StutterEfficiency = Default::default();
	v.Z8NumberOfStutterBurstsPerFrameBestCase = v.let mut Z8NumberOfStutterBurstsPerFrame = Default::default();
	v.StutterPeriodBestCase = v.let mut StutterPeriod = Default::default();
#endif

#ifdef __DML_VBA_DEBUG__
	dml_print("DML::%s: --- END -= 1-\n", file!);
#endif
}

unsafe fn mode_support_configuration(// struct vba_vars_stv,
				  // struct display_mode_libmode_lib)
{
	int i, j, let mut start_state = Default::default();

	if mode_lib.validate_max_state
		start_state = v.soc.num_states - 1;
	else
		start_state = 0;

	for (i = v.soc.num_states - 1; i >= let mut start_state = Default::default(); i -= 1) {
		for (j = 0; j < 2; j += 1) {
			if (mode_lib.vba.ScaleRatioAndTapsSupport == true && mode_lib.vba.SourceFormatPixelAndScanSupport == true && mode_lib.vba.ViewportSizeSupport[i][j] == true
				&& !mode_lib.vba.LinkRateDoesNotMatchDPVersion
				&& !mode_lib.vba.LinkRateForMultistreamNotIndicated
				&& !mode_lib.vba.BPPForMultistreamNotIndicated
				&& !mode_lib.vba.MultistreamWithHDMIOreDP
				&& !mode_lib.vba.ExceededMultistreamSlots[i]
				&& !mode_lib.vba.MSOOrODMSplitWithNonDPLink
				&& !mode_lib.vba.NotEnoughLanesForMSO && mode_lib.vba.LinkCapacitySupport[i] == true && !mode_lib.vba.P2IWith420
				//&& !mode_lib.vba.DSCOnlyIfNecessaryWithBPP
				&& !mode_lib.vba.DSC422NativeNotSupported
				&& !mode_lib.vba.MPCCombineMethodIncompatible && mode_lib.vba.ODMCombine2To1SupportCheckOK[i] == true && mode_lib.vba.ODMCombine4To1SupportCheckOK[i] == true && mode_lib.vba.NotEnoughDSCUnits[i] == false
				&& !mode_lib.vba.NotEnoughDSCSlices[i]
				&& !mode_lib.vba.ImmediateFlipOrHostVMAndPStateWithMALLFullFrameOrPhantomPipe
				&& !mode_lib.vba.InvalidCombinationOfMALLUseForPStateAndStaticScreen && mode_lib.vba.DSCCLKRequiredMoreThanSupported[i] == false && mode_lib.vba.PixelsPerLinePerDSCUnitSupport[i]
				&& mode_lib.vba.DTBCLKRequiredMoreThanSupported[i] == false
				&& !mode_lib.vba.InvalidCombinationOfMALLUseForPState
				&& !mode_lib.vba.ImmediateFlipRequiredButTheRequirementForEachSurfaceIsNotSpecified && mode_lib.vba.ROBSupport[i][j] == true && mode_lib.vba.DISPCLK_DPPCLK_Support[i][j] == true && mode_lib.vba.TotalAvailablePipesSupport[i][j] == true && mode_lib.vba.NumberOfOTGSupport == true && mode_lib.vba.NumberOfHDMIFRLSupport == true && mode_lib.vba.EnoughWritebackUnits == true && mode_lib.vba.WritebackLatencySupport == true && mode_lib.vba.WritebackScaleRatioAndTapsSupport == true && mode_lib.vba.CursorSupport == true && mode_lib.vba.PitchSupport == true && mode_lib.vba.ViewportExceedsSurface == false && mode_lib.vba.PrefetchSupported[i][j] == true && mode_lib.vba.VActiveBandwithSupport[i][j] == true && mode_lib.vba.DynamicMetadataSupported[i][j] == true && mode_lib.vba.TotalVerticalActiveBandwidthSupport[i][j] == true && mode_lib.vba.VRatioInPrefetchSupported[i][j] == true && mode_lib.vba.PTEBufferSizeNotExceeded[i][j] == true && mode_lib.vba.DCCMetaBufferSizeNotExceeded[i][j] == true && mode_lib.vba.NonsupportedDSCInputBPC == false
				&& !mode_lib.vba.ExceededMALLSize
				&& (mode_lib.vba.NotEnoughDETSwathFillLatencyHidingPerState[i][j] == false || i == v.soc.num_states - 1)
				&& ((mode_lib.vba.HostVMEnable == false
				&& !mode_lib.vba.ImmediateFlipRequiredFinal)
				|| mode_lib.vba.ImmediateFlipSupportedForState[i][j])
				&& (!mode_lib.vba.DRAMClockChangeRequirementFinal || i == v.soc.num_states - 1 || mode_lib.vba.DRAMClockChangeSupport[i][j] != dm_dram_clock_change_unsupported)
				&& (!mode_lib.vba.FCLKChangeRequirementFinal || i == v.soc.num_states - 1 || mode_lib.vba.FCLKChangeSupport[i][j] != dm_fclock_change_unsupported)
				&& (!mode_lib.vba.USRRetrainingRequiredFinal || mode_lib.vba.USRRetrainingSupport[i][j])) {
				mode_lib.vba.ModeSupport[i][j] = let mut true = Default::default();
			} else {
				mode_lib.vba.ModeSupport[i][j] = let mut false = Default::default();
			}
		}
	}
}

pub unsafe fn dml32_ModeSupportAndSystemConfigurationFull(// struct display_mode_libmode_lib)
{
	// struct vba_vars_stv = &mode_lib.let mut vba = Default::default();
	int i, let mut start_state = Default::default();
	unsigned int let mut j = Default::default();
	unsigned int k, let mut m = Default::default();
	unsigned int let mut MaximumMPCCombine = Default::default();
	unsigned int let mut NumberOfNonCombinedSurfaceOfMaximumBandwidth = Default::default();
	unsigned int let mut TotalSlots = Default::default();
	bool let mut CompBufReservedSpaceNeedAdjustment = Default::default();
	bool let mut CompBufReservedSpaceNeedAdjustmentSingleDPP = Default::default();

#ifdef __DML_VBA_DEBUG__
	dml_print("DML::%s: called\n", file!);
#endif

	/*MODE SUPPORT, VOLTAGE STATE AND SOC CONFIGURATION*/
	if mode_lib.validate_max_state
		start_state = v.soc.num_states - 1;
	else
		start_state = 0;

	/*Scale Ratio, taps Support Check*/

	mode_lib.vba.ScaleRatioAndTapsSupport = let mut true = Default::default();
	for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
		if (mode_lib.vba.ScalerEnabled[k] == false
				&& ((mode_lib.vba.SourcePixelFormat[k] != dm_444_64 && mode_lib.vba.SourcePixelFormat[k] != dm_444_32 && mode_lib.vba.SourcePixelFormat[k] != dm_444_16 && mode_lib.vba.SourcePixelFormat[k] != dm_mono_16 && mode_lib.vba.SourcePixelFormat[k] != dm_mono_8 && mode_lib.vba.SourcePixelFormat[k] != dm_rgbe && mode_lib.vba.SourcePixelFormat[k] != dm_rgbe_alpha)
						|| mode_lib.vba.HRatio[k] != 1.0 || mode_lib.vba.htaps[k] != 1.0 || mode_lib.vba.VRatio[k] != 1.0 || mode_lib.vba.vtaps[k] != 1.0)) {
			mode_lib.vba.ScaleRatioAndTapsSupport = let mut false = Default::default();
		} else if (mode_lib.vba.vtaps[k] < 1.0 || mode_lib.vba.vtaps[k] > 8.0 || mode_lib.vba.htaps[k] < 1.0 || mode_lib.vba.htaps[k] > 8.0
				|| (mode_lib.vba.htaps[k] > 1.0 && (mode_lib.vba.htaps[k] % 2) == 1)
				|| mode_lib.vba.HRatio[k] > mode_lib.vba.MaxHSCLRatio || mode_lib.vba.VRatio[k] > mode_lib.vba.MaxVSCLRatio || mode_lib.vba.HRatio[k] > mode_lib.vba.htaps[k]
				|| mode_lib.vba.VRatio[k] > mode_lib.vba.vtaps[k]
				|| (mode_lib.vba.SourcePixelFormat[k] != dm_444_64 && mode_lib.vba.SourcePixelFormat[k] != dm_444_32 && mode_lib.vba.SourcePixelFormat[k] != dm_444_16 && mode_lib.vba.SourcePixelFormat[k] != dm_mono_16 && mode_lib.vba.SourcePixelFormat[k] != dm_mono_8 && mode_lib.vba.SourcePixelFormat[k] != dm_rgbe
						&& (mode_lib.vba.VTAPsChroma[k] < 1 || mode_lib.vba.VTAPsChroma[k] > 8 || mode_lib.vba.HTAPsChroma[k] < 1 || mode_lib.vba.HTAPsChroma[k] > 8
								|| (mode_lib.vba.HTAPsChroma[k] > 1 && mode_lib.vba.HTAPsChroma[k] % 2
												== 1)
								|| mode_lib.vba.HRatioChroma[k]
										> mode_lib.vba.MaxHSCLRatio || mode_lib.vba.VRatioChroma[k]
										> mode_lib.vba.MaxVSCLRatio || mode_lib.vba.HRatioChroma[k]
										> mode_lib.vba.HTAPsChroma[k]
								|| mode_lib.vba.VRatioChroma[k]
										> mode_lib.vba.VTAPsChroma[k]))) {
			mode_lib.vba.ScaleRatioAndTapsSupport = let mut false = Default::default();
		}
	}

	/*Source Format, Pixel Format and Scan Support Check*/
	mode_lib.vba.SourceFormatPixelAndScanSupport = let mut true = Default::default();
	for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
		if (mode_lib.vba.SurfaceTiling[k] == dm_sw_linear
			&& (!(!IsVertical((enum dm_rotation_angle) mode_lib.vba.SourceScan[k]))
				|| mode_lib.vba.DCCEnable[k] == true)) {
			mode_lib.vba.SourceFormatPixelAndScanSupport = let mut false = Default::default();
		}
	}

	for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
		dml32_CalculateBytePerPixelAndBlockSizes(
				mode_lib.vba.SourcePixelFormat[k],
				mode_lib.vba.SurfaceTiling[k],

				/* Output */
				&mode_lib.vba.BytePerPixelY[k],
				&mode_lib.vba.BytePerPixelC[k],
				&mode_lib.vba.BytePerPixelInDETY[k],
				&mode_lib.vba.BytePerPixelInDETC[k],
				&mode_lib.vba.Read256BlockHeightY[k],
				&mode_lib.vba.Read256BlockHeightC[k],
				&mode_lib.vba.Read256BlockWidthY[k],
				&mode_lib.vba.Read256BlockWidthC[k],
				&mode_lib.vba.MacroTileHeightY[k],
				&mode_lib.vba.MacroTileHeightC[k],
				&mode_lib.vba.MacroTileWidthY[k],
				&mode_lib.vba.MacroTileWidthC[k]);
	}

	/*Bandwidth Support Check*/
	for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
		if !IsVertical(mode_lib.vba.SourceRotation[k]) {
			v.SwathWidthYSingleDPP[k] = mode_lib.vba.ViewportWidth[k];
			v.SwathWidthCSingleDPP[k] = mode_lib.vba.ViewportWidthChroma[k];
		} else {
			v.SwathWidthYSingleDPP[k] = mode_lib.vba.ViewportHeight[k];
			v.SwathWidthCSingleDPP[k] = mode_lib.vba.ViewportHeightChroma[k];
		}
	}
	for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
		v.ReadBandwidthLuma[k] = v.SwathWidthYSingleDPP[k] * dml_ceil(v.BytePerPixelInDETY[k], 1.0)
				/ (mode_lib.vba.HTotal[k] / mode_lib.vba.PixelClock[k]) * mode_lib.vba.VRatio[k];
		v.ReadBandwidthChroma[k] = v.SwathWidthYSingleDPP[k] / 2 * dml_ceil(v.BytePerPixelInDETC[k], 2.0)
				/ (mode_lib.vba.HTotal[k] / mode_lib.vba.PixelClock[k]) * mode_lib.vba.VRatio[k]
				/ 2.0;
	}
	for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
		if mode_lib.vba.WritebackEnable[k] == true && mode_lib.vba.WritebackPixelFormat[k] == dm_444_64 {
			v.WriteBandwidth[k] = mode_lib.vba.WritebackDestinationWidth[k]
					* mode_lib.vba.WritebackDestinationHeight[k]
					/ (mode_lib.vba.WritebackSourceHeight[k] * mode_lib.vba.HTotal[k]
							/ mode_lib.vba.PixelClock[k]) * 8.0;
		} else if mode_lib.vba.WritebackEnable[k] == true {
			v.WriteBandwidth[k] = mode_lib.vba.WritebackDestinationWidth[k]
					* mode_lib.vba.WritebackDestinationHeight[k]
					/ (mode_lib.vba.WritebackSourceHeight[k] * mode_lib.vba.HTotal[k]
							/ mode_lib.vba.PixelClock[k]) * 4.0;
		} else {
			v.WriteBandwidth[k] = 0.0;
		}
	}

	/*Writeback Latency support check*/

	mode_lib.vba.WritebackLatencySupport = let mut true = Default::default();
	for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
		if (mode_lib.vba.WritebackEnable[k] == true
				&& (v.WriteBandwidth[k]
						> mode_lib.vba.WritebackInterfaceBufferSize * 1024
								/ mode_lib.vba.WritebackLatency)) {
			mode_lib.vba.WritebackLatencySupport = let mut false = Default::default();
		}
	}

	/*Writeback Mode Support Check*/
	mode_lib.vba.EnoughWritebackUnits = let mut true = Default::default();
	mode_lib.vba.TotalNumberOfActiveWriteback = 0;
	for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
		if mode_lib.vba.WritebackEnable[k] == true
			mode_lib.vba.TotalNumberOfActiveWriteback = mode_lib.vba.TotalNumberOfActiveWriteback + 1;
	}

	if mode_lib.vba.TotalNumberOfActiveWriteback > mode_lib.vba.MaxNumWriteback
		mode_lib.vba.EnoughWritebackUnits = let mut false = Default::default();

	/*Writeback Scale Ratio and Taps Support Check*/
	mode_lib.vba.WritebackScaleRatioAndTapsSupport = let mut true = Default::default();
	for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
		if mode_lib.vba.WritebackEnable[k] == true {
			if (mode_lib.vba.WritebackHRatio[k] > mode_lib.vba.WritebackMaxHSCLRatio || mode_lib.vba.WritebackVRatio[k] > mode_lib.vba.WritebackMaxVSCLRatio || mode_lib.vba.WritebackHRatio[k] < mode_lib.vba.WritebackMinHSCLRatio || mode_lib.vba.WritebackVRatio[k] < mode_lib.vba.WritebackMinVSCLRatio || mode_lib.vba.WritebackHTaps[k] > mode_lib.vba.WritebackMaxHSCLTaps || mode_lib.vba.WritebackVTaps[k] > mode_lib.vba.WritebackMaxVSCLTaps || mode_lib.vba.WritebackHRatio[k] > mode_lib.vba.WritebackHTaps[k]
					|| mode_lib.vba.WritebackVRatio[k] > mode_lib.vba.WritebackVTaps[k]
					|| (mode_lib.vba.WritebackHTaps[k] > 2.0
							&& ((mode_lib.vba.WritebackHTaps[k] % 2) == 1))) {
				mode_lib.vba.WritebackScaleRatioAndTapsSupport = let mut false = Default::default();
			}
			if 2.0 * mode_lib.vba.WritebackDestinationWidth[k] * (mode_lib.vba.WritebackVTaps[k] - 1
					* 57 > mode_lib.vba.WritebackLineBufferSize) {
				mode_lib.vba.WritebackScaleRatioAndTapsSupport = let mut false = Default::default();
			}
		}
	}

	for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
		dml32_CalculateSinglePipeDPPCLKAndSCLThroughput(mode_lib.vba.HRatio[k], mode_lib.vba.HRatioChroma[k],
				mode_lib.vba.VRatio[k], mode_lib.vba.VRatioChroma[k],
				mode_lib.vba.MaxDCHUBToPSCLThroughput, mode_lib.vba.MaxPSCLToLBThroughput,
				mode_lib.vba.PixelClock[k], mode_lib.vba.SourcePixelFormat[k],
				mode_lib.vba.htaps[k], mode_lib.vba.HTAPsChroma[k], mode_lib.vba.vtaps[k],
				mode_lib.vba.VTAPsChroma[k],
				/* Output */
				&mode_lib.vba.PSCL_FACTOR[k], &mode_lib.vba.PSCL_FACTOR_CHROMA[k],
				&mode_lib.vba.MinDPPCLKUsingSingleDPP[k]);
	}

	for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {

		if mode_lib.vba.SurfaceTiling[k] == dm_sw_linear {
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.MaximumSwathWidthSupportLuma = 8192;
		} else if !IsVertical(mode_lib.vba.SourceRotation[k] && v.BytePerPixelC[k] > 0 && mode_lib.vba.SourcePixelFormat[k] != dm_rgbe_alpha) {
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.MaximumSwathWidthSupportLuma = 7680;
		} else if IsVertical(mode_lib.vba.SourceRotation[k] && v.BytePerPixelC[k] > 0 && mode_lib.vba.SourcePixelFormat[k] != dm_rgbe_alpha) {
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.MaximumSwathWidthSupportLuma = 4320;
		} else if mode_lib.vba.SourcePixelFormat[k] == dm_rgbe_alpha {
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.MaximumSwathWidthSupportLuma = 3840;
		} else if IsVertical(mode_lib.vba.SourceRotation[k] && v.BytePerPixelY[k] == 8 && mode_lib.vba.DCCEnable[k] == true) {
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.MaximumSwathWidthSupportLuma = 3072;
		} else {
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.MaximumSwathWidthSupportLuma = 6144;
		}

		if (mode_lib.vba.SourcePixelFormat[k] == dm_420_8 || mode_lib.vba.SourcePixelFormat[k] == dm_420_10 || mode_lib.vba.SourcePixelFormat[k] == dm_420_12) {
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.MaximumSwathWidthSupportChroma = (unsigned int)(v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.MaximumSwathWidthSupportLuma / 2.0);
		} else {
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.MaximumSwathWidthSupportChroma = v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.let mut MaximumSwathWidthSupportLuma = Default::default();
		}
		v.MaximumSwathWidthInLineBufferLuma = mode_lib.vba.LineBufferSizeFinal
				* dml_max(mode_lib.vba.HRatio[k], 1.0) / mode_lib.vba.LBBitPerPixel[k]
				/ (mode_lib.vba.vtaps[k] + dml_max(dml_ceil(mode_lib.vba.VRatio[k], 1.0) - 2, 0.0));
		if v.BytePerPixelC[k] == 0.0 {
			v.MaximumSwathWidthInLineBufferChroma = 0;
		} else {
			v.MaximumSwathWidthInLineBufferChroma = mode_lib.vba.LineBufferSizeFinal
					* dml_max(mode_lib.vba.HRatioChroma[k], 1.0) / mode_lib.vba.LBBitPerPixel[k]
					/ (mode_lib.vba.VTAPsChroma[k]
							+ dml_max(dml_ceil(mode_lib.vba.VRatioChroma[k], 1.0) - 2,
									0.0));
		}
		v.MaximumSwathWidthLuma[k] = dml_min(v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.MaximumSwathWidthSupportLuma,
				v.MaximumSwathWidthInLineBufferLuma);
		v.MaximumSwathWidthChroma[k] = dml_min(v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.MaximumSwathWidthSupportChroma,
				v.MaximumSwathWidthInLineBufferChroma);
	}

	dml32_CalculateSwathAndDETConfiguration(
			mode_lib.vba.DETSizeOverride,
			mode_lib.vba.UsesMALLForPStateChange,
			mode_lib.vba.ConfigReturnBufferSizeInKByte,
			mode_lib.vba.MaxTotalDETInKByte,
			mode_lib.vba.MinCompressedBufferSizeInKByte,
			1, /* ForceSingleDPP */
			mode_lib.vba.NumberOfActiveSurfaces,
			mode_lib.vba.nomDETInKByte,
			mode_lib.vba.UseUnboundedRequesting,
			mode_lib.vba.DisableUnboundRequestIfCompBufReservedSpaceNeedAdjustment,
			mode_lib.vba.ip.pixel_chunk_size_kbytes,
			mode_lib.vba.ip.rob_buffer_size_kbytes,
			mode_lib.vba.CompressedBufferSegmentSizeInkByteFinal,
			mode_lib.vba.Output,
			mode_lib.vba.ReadBandwidthLuma,
			mode_lib.vba.ReadBandwidthChroma,
			mode_lib.vba.MaximumSwathWidthLuma,
			mode_lib.vba.MaximumSwathWidthChroma,
			mode_lib.vba.SourceRotation,
			mode_lib.vba.ViewportStationary,
			mode_lib.vba.SourcePixelFormat,
			mode_lib.vba.SurfaceTiling,
			mode_lib.vba.ViewportWidth,
			mode_lib.vba.ViewportHeight,
			mode_lib.vba.ViewportXStartY,
			mode_lib.vba.ViewportYStartY,
			mode_lib.vba.ViewportXStartC,
			mode_lib.vba.ViewportYStartC,
			mode_lib.vba.SurfaceWidthY,
			mode_lib.vba.SurfaceWidthC,
			mode_lib.vba.SurfaceHeightY,
			mode_lib.vba.SurfaceHeightC,
			mode_lib.vba.Read256BlockHeightY,
			mode_lib.vba.Read256BlockHeightC,
			mode_lib.vba.Read256BlockWidthY,
			mode_lib.vba.Read256BlockWidthC,
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_odm_mode,
			mode_lib.vba.BlendingAndTiming,
			mode_lib.vba.BytePerPixelY,
			mode_lib.vba.BytePerPixelC,
			mode_lib.vba.BytePerPixelInDETY,
			mode_lib.vba.BytePerPixelInDETC,
			mode_lib.vba.HActive,
			mode_lib.vba.HRatio,
			mode_lib.vba.HRatioChroma,
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[0], /*  Integer DPPPerSurface[] */

			/* Output */
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[1], /* Long            swath_width_luma_ub[] */
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[2], /* Long            swath_width_chroma_ub[]  */
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_double_array[0], /* Long            SwathWidth[]  */
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_double_array[1], /* Long            SwathWidthChroma[]  */
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[3], /* Integer         SwathHeightY[]  */
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[4], /* Integer         SwathHeightC[]  */
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[5], /* Long            DETBufferSizeInKByte[]  */
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[6], /* Long            DETBufferSizeY[]  */
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[7], /* Long            DETBufferSizeC[]  */
			&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_boolean_array[0][0], /* bool           *UnboundedRequestEnabled  */
			&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[0][0], /* Long           *CompressedBufferSizeInkByte  */
			&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[1][0], /* Long           *CompBufReservedSpaceKBytes */
			&CompBufReservedSpaceNeedAdjustmentSingleDPP,
			mode_lib.vba.SingleDPPViewportSizeSupportPerSurface,/* bool ViewportSizeSupportPerSurface[] */
			&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_boolean_array[1][0]); /* bool           *ViewportSizeSupport */

	v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.MPCCombineMethodAsNeededForPStateChangeAndVoltage = let mut false = Default::default();
	v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.MPCCombineMethodAsPossible = let mut false = Default::default();

	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		if mode_lib.vba.MPCCombineUse[k] == dm_mpc_reduce_voltage_and_clocks
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.MPCCombineMethodAsNeededForPStateChangeAndVoltage = let mut true = Default::default();
		if mode_lib.vba.MPCCombineUse[k] == dm_mpc_always_when_possible
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.MPCCombineMethodAsPossible = let mut true = Default::default();
	}
	mode_lib.vba.MPCCombineMethodIncompatible = v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.MPCCombineMethodAsNeededForPStateChangeAndVoltage && v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.let mut MPCCombineMethodAsPossible = Default::default();

	for (i = let mut start_state = Default::default(); i < (int)v.soc.let mut num_states = Default::default(); i += 1) {
		for (j = 0; j < 2; j += 1) {
			mode_lib.vba.TotalNumberOfActiveDPP[i][j] = 0;
			mode_lib.vba.TotalAvailablePipesSupport[i][j] = let mut true = Default::default();
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.ODMModeNoDSC = let mut dm_odm_combine_mode_disabled = Default::default();
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.ODMModeDSC = let mut dm_odm_combine_mode_disabled = Default::default();

			for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
				dml32_CalculateODMMode(
						mode_lib.vba.MaximumPixelsPerLinePerDSCUnit,
						mode_lib.vba.HActive[k],
						mode_lib.vba.OutputFormat[k],
						mode_lib.vba.Output[k],
						mode_lib.vba.ODMUse[k],
						mode_lib.vba.MaxDispclk[i],
						mode_lib.vba.MaxDispclk[v.soc.num_states - 1],
						false,
						mode_lib.vba.TotalNumberOfActiveDPP[i][j],
						mode_lib.vba.MaxNumDPP,
						mode_lib.vba.PixelClock[k],
						mode_lib.vba.DISPCLKDPPCLKDSCCLKDownSpreading,
						mode_lib.vba.DISPCLKRampingMargin,
						mode_lib.vba.DISPCLKDPPCLKVCOSpeed,
						mode_lib.vba.NumberOfDSCSlices[k],

						/* Output */
						&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalAvailablePipesSupportNoDSC,
						&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.NumberOfDPPNoDSC,
						&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.ODMModeNoDSC,
						&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.RequiredDISPCLKPerSurfaceNoDSC);

				dml32_CalculateODMMode(
						mode_lib.vba.MaximumPixelsPerLinePerDSCUnit,
						mode_lib.vba.HActive[k],
						mode_lib.vba.OutputFormat[k],
						mode_lib.vba.Output[k],
						mode_lib.vba.ODMUse[k],
						mode_lib.vba.MaxDispclk[i],
						mode_lib.vba.MaxDispclk[v.soc.num_states - 1],
						true,
						mode_lib.vba.TotalNumberOfActiveDPP[i][j],
						mode_lib.vba.MaxNumDPP,
						mode_lib.vba.PixelClock[k],
						mode_lib.vba.DISPCLKDPPCLKDSCCLKDownSpreading,
						mode_lib.vba.DISPCLKRampingMargin,
						mode_lib.vba.DISPCLKDPPCLKVCOSpeed,
						mode_lib.vba.NumberOfDSCSlices[k],

						/* Output */
						&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalAvailablePipesSupportDSC,
						&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.NumberOfDPPDSC,
						&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.ODMModeDSC,
						&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.RequiredDISPCLKPerSurfaceDSC);

				dml32_CalculateOutputLink(
						mode_lib.vba.PHYCLKPerState[i],
						mode_lib.vba.PHYCLKD18PerState[i],
						mode_lib.vba.PHYCLKD32PerState[i],
						mode_lib.vba.Downspreading,
						(mode_lib.vba.BlendingAndTiming[k] == k),
						mode_lib.vba.Output[k],
						mode_lib.vba.OutputFormat[k],
						mode_lib.vba.HTotal[k],
						mode_lib.vba.HActive[k],
						mode_lib.vba.PixelClockBackEnd[k],
						mode_lib.vba.ForcedOutputLinkBPP[k],
						mode_lib.vba.DSCInputBitPerComponent[k],
						mode_lib.vba.NumberOfDSCSlices[k],
						mode_lib.vba.AudioSampleRate[k],
						mode_lib.vba.AudioSampleLayout[k],
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.ODMModeNoDSC,
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.ODMModeDSC,
						mode_lib.vba.DSCEnable[k],
						(int)mode_lib.vba.OutputLinkDPLanes[k],
						mode_lib.vba.OutputLinkDPRate[k],

						/* Output */
						&mode_lib.vba.RequiresDSC[i][k],
						&mode_lib.vba.RequiresFEC[i][k],
						&mode_lib.vba.OutputBppPerState[i][k],
						&mode_lib.vba.OutputTypePerState[i][k],
						&mode_lib.vba.OutputRatePerState[i][k],
						&mode_lib.vba.RequiredSlots[i][k]);

				if mode_lib.vba.RequiresDSC[i][k] == false {
					mode_lib.vba.ODMCombineEnablePerState[i][k] = v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.let mut ODMModeNoDSC = Default::default();
					mode_lib.vba.RequiredDISPCLKPerSurface[i][j][k] =
							v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.let mut RequiredDISPCLKPerSurfaceNoDSC = Default::default();
					if !v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalAvailablePipesSupportNoDSC
						mode_lib.vba.TotalAvailablePipesSupport[i][j] = let mut false = Default::default();
					mode_lib.vba.TotalNumberOfActiveDPP[i][j] =
							mode_lib.vba.TotalNumberOfActiveDPP[i][j] + v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.let mut NumberOfDPPNoDSC = Default::default();
				} else {
					mode_lib.vba.ODMCombineEnablePerState[i][k] = v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.let mut ODMModeDSC = Default::default();
					mode_lib.vba.RequiredDISPCLKPerSurface[i][j][k] =
							v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.let mut RequiredDISPCLKPerSurfaceDSC = Default::default();
					if !v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalAvailablePipesSupportDSC
						mode_lib.vba.TotalAvailablePipesSupport[i][j] = let mut false = Default::default();
					mode_lib.vba.TotalNumberOfActiveDPP[i][j] =
							mode_lib.vba.TotalNumberOfActiveDPP[i][j] + v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.let mut NumberOfDPPDSC = Default::default();
				}
			}

			for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
				if mode_lib.vba.ODMCombineEnablePerState[i][k] == dm_odm_combine_mode_4to1 {
					mode_lib.vba.MPCCombine[i][j][k] = let mut false = Default::default();
					mode_lib.vba.NoOfDPP[i][j][k] = 4;
				} else if mode_lib.vba.ODMCombineEnablePerState[i][k] == dm_odm_combine_mode_2to1 {
					mode_lib.vba.MPCCombine[i][j][k] = let mut false = Default::default();
					mode_lib.vba.NoOfDPP[i][j][k] = 2;
				} else if mode_lib.vba.MPCCombineUse[k] == dm_mpc_never {
					mode_lib.vba.MPCCombine[i][j][k] = let mut false = Default::default();
					mode_lib.vba.NoOfDPP[i][j][k] = 1;
				} else if (dml32_RoundToDFSGranularity(
						mode_lib.vba.MinDPPCLKUsingSingleDPP[k]
								* (1 + mode_lib.vba.DISPCLKDPPCLKDSCCLKDownSpreading
												/ 100), 1,
						mode_lib.vba.DISPCLKDPPCLKVCOSpeed) <= mode_lib.vba.MaxDppclk[i] &&
				mode_lib.vba.SingleDPPViewportSizeSupportPerSurface[k] == true) {
					mode_lib.vba.MPCCombine[i][j][k] = let mut false = Default::default();
					mode_lib.vba.NoOfDPP[i][j][k] = 1;
				} else if mode_lib.vba.TotalNumberOfActiveDPP[i][j] < mode_lib.vba.MaxNumDPP {
					mode_lib.vba.MPCCombine[i][j][k] = let mut true = Default::default();
					mode_lib.vba.NoOfDPP[i][j][k] = 2;
					mode_lib.vba.TotalNumberOfActiveDPP[i][j] =
							mode_lib.vba.TotalNumberOfActiveDPP[i][j] + 1;
				} else {
					mode_lib.vba.MPCCombine[i][j][k] = let mut false = Default::default();
					mode_lib.vba.NoOfDPP[i][j][k] = 1;
					mode_lib.vba.TotalAvailablePipesSupport[i][j] = let mut false = Default::default();
				}
			}

			mode_lib.vba.TotalNumberOfSingleDPPSurfaces[i][j] = 0;
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.NoChroma = let mut true = Default::default();

			for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
				if mode_lib.vba.NoOfDPP[i][j][k] == 1
					mode_lib.vba.TotalNumberOfSingleDPPSurfaces[i][j] =
							mode_lib.vba.TotalNumberOfSingleDPPSurfaces[i][j] + 1;
				if (mode_lib.vba.SourcePixelFormat[k] == dm_420_8 || mode_lib.vba.SourcePixelFormat[k] == dm_420_10 || mode_lib.vba.SourcePixelFormat[k] == dm_420_12 || mode_lib.vba.SourcePixelFormat[k] == dm_rgbe_alpha) {
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.NoChroma = let mut false = Default::default();
				}
			}

			// if TotalNumberOfActiveDPP is > 1, then there should be no unbounded req mode (hw limitation), the comp buf reserved adjustment is not needed regardless
			// if TotalNumberOfActiveDPP is == 1, then will use the SingleDPP version of unbounded_req for the decision
			CompBufReservedSpaceNeedAdjustment = (mode_lib.vba.TotalNumberOfActiveDPP[i][j] > 1) ? 0 : let mut CompBufReservedSpaceNeedAdjustmentSingleDPP = Default::default();



			if (j == 1 && !dml32_UnboundedRequest(mode_lib.vba.UseUnboundedRequesting,
					mode_lib.vba.TotalNumberOfActiveDPP[i][j], v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.NoChroma,
					mode_lib.vba.Output[0],
					mode_lib.vba.SurfaceTiling[0],
					CompBufReservedSpaceNeedAdjustment,
					mode_lib.vba.DisableUnboundRequestIfCompBufReservedSpaceNeedAdjustment)) {
				while (!(mode_lib.vba.TotalNumberOfActiveDPP[i][j] >= mode_lib.vba.MaxNumDPP || mode_lib.vba.TotalNumberOfSingleDPPSurfaces[i][j] == 0)) {
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.BWOfNonCombinedSurfaceOfMaximumBandwidth = 0;
					NumberOfNonCombinedSurfaceOfMaximumBandwidth = 0;

					for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
						if (mode_lib.vba.MPCCombineUse[k]
							!= dm_mpc_never && mode_lib.vba.MPCCombineUse[k] != dm_mpc_reduce_voltage && mode_lib.vba.ReadBandwidthLuma[k] +
							mode_lib.vba.ReadBandwidthChroma[k] >
							v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.BWOfNonCombinedSurfaceOfMaximumBandwidth &&
							(mode_lib.vba.ODMCombineEnablePerState[i][k] !=
							dm_odm_combine_mode_2to1 && mode_lib.vba.ODMCombineEnablePerState[i][k] !=
							dm_odm_combine_mode_4to1) &&
								mode_lib.vba.MPCCombine[i][j][k] == false) {
							v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.BWOfNonCombinedSurfaceOfMaximumBandwidth =
								mode_lib.vba.ReadBandwidthLuma[k]
								+ mode_lib.vba.ReadBandwidthChroma[k];
							NumberOfNonCombinedSurfaceOfMaximumBandwidth = let mut k = Default::default();
						}
					}
					mode_lib.vba.MPCCombine[i][j][NumberOfNonCombinedSurfaceOfMaximumBandwidth] =
							let mut true = Default::default();
					mode_lib.vba.NoOfDPP[i][j][NumberOfNonCombinedSurfaceOfMaximumBandwidth] = 2;
					mode_lib.vba.TotalNumberOfActiveDPP[i][j] =
							mode_lib.vba.TotalNumberOfActiveDPP[i][j] + 1;
					mode_lib.vba.TotalNumberOfSingleDPPSurfaces[i][j] =
							mode_lib.vba.TotalNumberOfSingleDPPSurfaces[i][j] - 1;
				}
			}

			//DISPCLK/DPPCLK
			mode_lib.vba.WritebackRequiredDISPCLK = 0;
			for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
				if mode_lib.vba.WritebackEnable[k] {
					mode_lib.vba.WritebackRequiredDISPCLK = dml_max(
							mode_lib.vba.WritebackRequiredDISPCLK,
							dml32_CalculateWriteBackDISPCLK(
									mode_lib.vba.WritebackPixelFormat[k],
									mode_lib.vba.PixelClock[k],
									mode_lib.vba.WritebackHRatio[k],
									mode_lib.vba.WritebackVRatio[k],
									mode_lib.vba.WritebackHTaps[k],
									mode_lib.vba.WritebackVTaps[k],
									mode_lib.vba.WritebackSourceWidth[k],
									(long)mode_lib.vba.WritebackDestinationWidth[k],
									mode_lib.vba.HTotal[k],
									(unsigned int)mode_lib.vba.WritebackLineBufferSize,
									mode_lib.vba.DISPCLKDPPCLKVCOSpeed));
				}
			}

			mode_lib.vba.RequiredDISPCLK[i][j] = mode_lib.vba.let mut WritebackRequiredDISPCLK = Default::default();
			for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
				mode_lib.vba.RequiredDISPCLK[i][j] = dml_max(mode_lib.vba.RequiredDISPCLK[i][j],
						mode_lib.vba.RequiredDISPCLKPerSurface[i][j][k]);
			}

			for k in 0..mode_lib.vba.NumberOfActiveSurfaces
				mode_lib.vba.NoOfDPPThisState[k] = mode_lib.vba.NoOfDPP[i][j][k];

			dml32_CalculateDPPCLK(mode_lib.vba.NumberOfActiveSurfaces,
					mode_lib.vba.DISPCLKDPPCLKDSCCLKDownSpreading,
					mode_lib.vba.DISPCLKDPPCLKVCOSpeed, mode_lib.vba.MinDPPCLKUsingSingleDPP,
					mode_lib.vba.NoOfDPPThisState,
					/* Output */
					&mode_lib.vba.GlobalDPPCLK, mode_lib.vba.RequiredDPPCLKThisState);

			for k in 0..mode_lib.vba.NumberOfActiveSurfaces
				mode_lib.vba.RequiredDPPCLK[i][j][k] = mode_lib.vba.RequiredDPPCLKThisState[k];

			mode_lib.vba.DISPCLK_DPPCLK_Support[i][j] = !((mode_lib.vba.RequiredDISPCLK[i][j]
					> mode_lib.vba.MaxDispclk[i])
					|| (mode_lib.vba.GlobalDPPCLK > mode_lib.vba.MaxDppclk[i]));

			if mode_lib.vba.TotalNumberOfActiveDPP[i][j] > mode_lib.vba.MaxNumDPP
				mode_lib.vba.TotalAvailablePipesSupport[i][j] = let mut false = Default::default();
		} // j
	} // i (VOLTAGE_STATE)

	/* Total Available OTG, HDMIFRL, DP Support Check */
	v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalNumberOfActiveOTG = 0;
	v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalNumberOfActiveHDMIFRL = 0;
	v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalNumberOfActiveDP2p0 = 0;
	v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalNumberOfActiveDP2p0Outputs = 0;

	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		if mode_lib.vba.BlendingAndTiming[k] == k {
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalNumberOfActiveOTG = v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalNumberOfActiveOTG + 1;
			if mode_lib.vba.Output[k] == dm_hdmifrl
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalNumberOfActiveDP2p0 = v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalNumberOfActiveDP2p0 + 1;
			if mode_lib.vba.Output[k] == dm_dp2p0 {
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalNumberOfActiveDP2p0 = v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalNumberOfActiveDP2p0 + 1;
				if (mode_lib.vba.OutputMultistreamId[k]
						== k || mode_lib.vba.OutputMultistreamEn[k] == false) {
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalNumberOfActiveDP2p0Outputs = v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalNumberOfActiveDP2p0Outputs + 1;
				}
			}
		}
	}

	mode_lib.vba.NumberOfOTGSupport = (v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalNumberOfActiveOTG <= mode_lib.vba.MaxNumOTG);
	mode_lib.vba.NumberOfHDMIFRLSupport = (v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalNumberOfActiveHDMIFRL <= mode_lib.vba.MaxNumHDMIFRLOutputs);
	mode_lib.vba.NumberOfDP2p0Support = (v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalNumberOfActiveDP2p0 <= mode_lib.vba.MaxNumDP2p0Streams && v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalNumberOfActiveDP2p0Outputs <= mode_lib.vba.MaxNumDP2p0Outputs);

	/* Display IO and DSC Support Check */
	mode_lib.vba.NonsupportedDSCInputBPC = let mut false = Default::default();
	for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
		if (!(mode_lib.vba.DSCInputBitPerComponent[k] == 12.0 || mode_lib.vba.DSCInputBitPerComponent[k] == 10.0 || mode_lib.vba.DSCInputBitPerComponent[k] == 8.0)
				|| mode_lib.vba.DSCInputBitPerComponent[k] > mode_lib.vba.MaximumDSCBitsPerComponent) {
			mode_lib.vba.NonsupportedDSCInputBPC = let mut true = Default::default();
		}
	}

	for i in start_state..(int)v.soc.num_states {
		mode_lib.vba.ExceededMultistreamSlots[i] = let mut false = Default::default();
		for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
			if mode_lib.vba.OutputMultistreamEn[k] == true && mode_lib.vba.OutputMultistreamId[k] == k {
				TotalSlots = mode_lib.vba.RequiredSlots[i][k];
				for j in 0..mode_lib.vba.NumberOfActiveSurfaces {
					if mode_lib.vba.OutputMultistreamId[j] == k
						TotalSlots = TotalSlots + mode_lib.vba.RequiredSlots[i][j];
				}
				if mode_lib.vba.Output[k] == dm_dp && TotalSlots > 63
					mode_lib.vba.ExceededMultistreamSlots[i] = let mut true = Default::default();
				if mode_lib.vba.Output[k] == dm_dp2p0 && TotalSlots > 64
					mode_lib.vba.ExceededMultistreamSlots[i] = let mut true = Default::default();
			}
		}
		mode_lib.vba.LinkCapacitySupport[i] = let mut true = Default::default();
		for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
			if (mode_lib.vba.BlendingAndTiming[k] == k
				&& (mode_lib.vba.Output[k] == dm_dp || mode_lib.vba.Output[k] == dm_dp2p0 || mode_lib.vba.Output[k] == dm_edp || mode_lib.vba.Output[k] == dm_hdmifrl || mode_lib.vba.Output[k] == dm_hdmi)
				&& mode_lib.vba.OutputBppPerState[i][k] == 0 &&
				(mode_lib.vba.UsesMALLForPStateChange[k] != dm_use_mall_pstate_change_phantom_pipe)) {
				/* Phantom pipes don't consider DSC in DML, so it could fail link check.
				 * However, we don't care about the link for phantom pipes.
				 */
				mode_lib.vba.LinkCapacitySupport[i] = let mut false = Default::default();
			}
		}
	}

	mode_lib.vba.P2IWith420 = let mut false = Default::default();
	mode_lib.vba.DSCOnlyIfNecessaryWithBPP = let mut false = Default::default();
	mode_lib.vba.DSC422NativeNotSupported = let mut false = Default::default();
	mode_lib.vba.LinkRateDoesNotMatchDPVersion = let mut false = Default::default();
	mode_lib.vba.LinkRateForMultistreamNotIndicated = let mut false = Default::default();
	mode_lib.vba.BPPForMultistreamNotIndicated = let mut false = Default::default();
	mode_lib.vba.MultistreamWithHDMIOreDP = let mut false = Default::default();
	mode_lib.vba.MSOOrODMSplitWithNonDPLink = let mut false = Default::default();
	mode_lib.vba.NotEnoughLanesForMSO = let mut false = Default::default();

	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		if (mode_lib.vba.BlendingAndTiming[k] == k
				&& (mode_lib.vba.Output[k] == dm_dp || mode_lib.vba.Output[k] == dm_dp2p0 || mode_lib.vba.Output[k] == dm_edp || mode_lib.vba.Output[k] == dm_hdmifrl || mode_lib.vba.Output[k] == dm_hdmi)) {
			if (mode_lib.vba.OutputFormat[k]
					== dm_420 && mode_lib.vba.Interlace[k] == 1 && mode_lib.vba.ProgressiveToInterlaceUnitInOPP == true)
				mode_lib.vba.P2IWith420 = let mut true = Default::default();

			if mode_lib.vba.DSCEnable[k] && mode_lib.vba.ForcedOutputLinkBPP[k] != 0
				mode_lib.vba.DSCOnlyIfNecessaryWithBPP = let mut true = Default::default();
			if (mode_lib.vba.DSCEnable[k] && mode_lib.vba.OutputFormat[k] == dm_n422
					&& !mode_lib.vba.DSC422NativeSupport)
				mode_lib.vba.DSC422NativeNotSupported = let mut true = Default::default();

			if (((mode_lib.vba.OutputLinkDPRate[k] == dm_dp_rate_hbr || mode_lib.vba.OutputLinkDPRate[k] == dm_dp_rate_hbr2 || mode_lib.vba.OutputLinkDPRate[k] == dm_dp_rate_hbr3)
					&& mode_lib.vba.Output[k] != dm_dp && mode_lib.vba.Output[k] != dm_edp)
					|| ((mode_lib.vba.OutputLinkDPRate[k] == dm_dp_rate_uhbr10 || mode_lib.vba.OutputLinkDPRate[k] == dm_dp_rate_uhbr13p5 || mode_lib.vba.OutputLinkDPRate[k] == dm_dp_rate_uhbr20)
							&& mode_lib.vba.Output[k] != dm_dp2p0))
				mode_lib.vba.LinkRateDoesNotMatchDPVersion = let mut true = Default::default();

			if mode_lib.vba.OutputMultistreamEn[k] == true {
				if (mode_lib.vba.OutputMultistreamId[k] == k && mode_lib.vba.OutputLinkDPRate[k] == dm_dp_rate_na)
					mode_lib.vba.LinkRateForMultistreamNotIndicated = let mut true = Default::default();
				if mode_lib.vba.OutputMultistreamId[k] == k && mode_lib.vba.ForcedOutputLinkBPP[k] == 0
					mode_lib.vba.BPPForMultistreamNotIndicated = let mut true = Default::default();
				for j in 0..mode_lib.vba.NumberOfActiveSurfaces {
					if (mode_lib.vba.OutputMultistreamId[k] == j && mode_lib.vba.ForcedOutputLinkBPP[k] == 0)
						mode_lib.vba.BPPForMultistreamNotIndicated = let mut true = Default::default();
				}
			}

			if ((mode_lib.vba.Output[k] == dm_edp || mode_lib.vba.Output[k] == dm_hdmifrl || mode_lib.vba.Output[k] == dm_hdmi)) {
				if mode_lib.vba.OutputMultistreamEn[k] == true && mode_lib.vba.OutputMultistreamId[k] == k
					mode_lib.vba.MultistreamWithHDMIOreDP = let mut true = Default::default();
				for j in 0..mode_lib.vba.NumberOfActiveSurfaces {
					if mode_lib.vba.OutputMultistreamEn[k] == true && mode_lib.vba.OutputMultistreamId[k] == j
						mode_lib.vba.MultistreamWithHDMIOreDP = let mut true = Default::default();
				}
			}

			if (mode_lib.vba.Output[k] != dm_dp
					&& (mode_lib.vba.ODMUse[k] == dm_odm_split_policy_1to2 || mode_lib.vba.ODMUse[k] == dm_odm_mso_policy_1to2 || mode_lib.vba.ODMUse[k] == dm_odm_mso_policy_1to4))
				mode_lib.vba.MSOOrODMSplitWithNonDPLink = let mut true = Default::default();

			if ((mode_lib.vba.ODMUse[k] == dm_odm_mso_policy_1to2 && mode_lib.vba.OutputLinkDPLanes[k] < 2)
					|| (mode_lib.vba.ODMUse[k] == dm_odm_mso_policy_1to4 && mode_lib.vba.OutputLinkDPLanes[k] < 4))
				mode_lib.vba.NotEnoughLanesForMSO = let mut true = Default::default();
		}
	}

	for i in start_state..(int)v.soc.num_states {
		mode_lib.vba.DTBCLKRequiredMoreThanSupported[i] = let mut false = Default::default();
		for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
			if (mode_lib.vba.BlendingAndTiming[k] == k && mode_lib.vba.Output[k] == dm_hdmifrl && dml32_RequiredDTBCLK(mode_lib.vba.RequiresDSC[i][k],
							mode_lib.vba.PixelClockBackEnd[k],
							mode_lib.vba.OutputFormat[k],
							mode_lib.vba.OutputBppPerState[i][k],
							mode_lib.vba.NumberOfDSCSlices[k], mode_lib.vba.HTotal[k],
							mode_lib.vba.HActive[k], mode_lib.vba.AudioSampleRate[k],
							mode_lib.vba.AudioSampleLayout[k])
							> mode_lib.vba.DTBCLKPerState[i]) {
				mode_lib.vba.DTBCLKRequiredMoreThanSupported[i] = let mut true = Default::default();
			}
		}
	}

	for i in start_state..(int)v.soc.num_states {
		mode_lib.vba.ODMCombine2To1SupportCheckOK[i] = let mut true = Default::default();
		mode_lib.vba.ODMCombine4To1SupportCheckOK[i] = let mut true = Default::default();
		for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
			if (mode_lib.vba.BlendingAndTiming[k] == k && mode_lib.vba.ODMCombineEnablePerState[i][k] == dm_odm_combine_mode_2to1 && mode_lib.vba.Output[k] == dm_hdmi) {
				mode_lib.vba.ODMCombine2To1SupportCheckOK[i] = let mut false = Default::default();
			}
			if (mode_lib.vba.BlendingAndTiming[k] == k && mode_lib.vba.ODMCombineEnablePerState[i][k] == dm_odm_combine_mode_4to1
					&& (mode_lib.vba.Output[k] == dm_dp || mode_lib.vba.Output[k] == dm_edp || mode_lib.vba.Output[k] == dm_hdmi)) {
				mode_lib.vba.ODMCombine4To1SupportCheckOK[i] = let mut false = Default::default();
			}
		}
	}

	for (i = let mut start_state = Default::default(); i < (int)v.soc.let mut num_states = Default::default(); i += 1) {
		mode_lib.vba.DSCCLKRequiredMoreThanSupported[i] = let mut false = Default::default();
		for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
			if mode_lib.vba.BlendingAndTiming[k] == k {
				if (mode_lib.vba.Output[k] == dm_dp || mode_lib.vba.Output[k] == dm_dp2p0 || mode_lib.vba.Output[k] == dm_hdmifrl || mode_lib.vba.Output[k] == dm_edp) {
					if mode_lib.vba.OutputFormat[k] == dm_420 {
						mode_lib.vba.DSCFormatFactor = 2;
					} else if mode_lib.vba.OutputFormat[k] == dm_444 {
						mode_lib.vba.DSCFormatFactor = 1;
					} else if mode_lib.vba.OutputFormat[k] == dm_n422 {
						mode_lib.vba.DSCFormatFactor = 2;
					} else if mode_lib.vba.Output[k] == dm_hdmifrl {
						mode_lib.vba.DSCFormatFactor = 2;
					} else {
						mode_lib.vba.DSCFormatFactor = 1;
					}
					if mode_lib.vba.RequiresDSC[i][k] == true {
						if (mode_lib.vba.ODMCombineEnablePerState[i][k]
								== dm_odm_combine_mode_4to1) {
							if mode_lib.vba.PixelClockBackEnd[k] / 12.0 / mode_lib.vba.DSCFormatFactor > (1.0 - mode_lib.vba.DISPCLKDPPCLKDSCCLKDownSpreading / 100.0) * mode_lib.vba.MaxDSCCLK[i]
								mode_lib.vba.DSCCLKRequiredMoreThanSupported[i] = let mut true = Default::default();
						} else if (mode_lib.vba.ODMCombineEnablePerState[i][k]
								== dm_odm_combine_mode_2to1) {
							if mode_lib.vba.PixelClockBackEnd[k] / 6.0 / mode_lib.vba.DSCFormatFactor > (1.0 - mode_lib.vba.DISPCLKDPPCLKDSCCLKDownSpreading / 100.0) * mode_lib.vba.MaxDSCCLK[i]
								mode_lib.vba.DSCCLKRequiredMoreThanSupported[i] = let mut true = Default::default();
						} else {
							if mode_lib.vba.PixelClockBackEnd[k] / 3.0 / mode_lib.vba.DSCFormatFactor > (1.0 - mode_lib.vba.DISPCLKDPPCLKDSCCLKDownSpreading / 100.0) * mode_lib.vba.MaxDSCCLK[i]
								mode_lib.vba.DSCCLKRequiredMoreThanSupported[i] = let mut true = Default::default();
						}
					}
				}
			}
		}
	}

	/* Check DSC Unit and Slices Support */
	v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalDSCUnitsRequired = 0;

	for i in start_state..(int)v.soc.num_states {
		mode_lib.vba.NotEnoughDSCUnits[i] = let mut false = Default::default();
		mode_lib.vba.NotEnoughDSCSlices[i] = let mut false = Default::default();
		v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalDSCUnitsRequired = 0;
		mode_lib.vba.PixelsPerLinePerDSCUnitSupport[i] = let mut true = Default::default();
		for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
			if mode_lib.vba.RequiresDSC[i][k] == true {
				if mode_lib.vba.ODMCombineEnablePerState[i][k] == dm_odm_combine_mode_4to1 {
					if (mode_lib.vba.HActive[k]
							> 4 * mode_lib.vba.MaximumPixelsPerLinePerDSCUnit)
						mode_lib.vba.PixelsPerLinePerDSCUnitSupport[i] = let mut false = Default::default();
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalDSCUnitsRequired = v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalDSCUnitsRequired + 4;
					if mode_lib.vba.NumberOfDSCSlices[k] > 16
						mode_lib.vba.NotEnoughDSCSlices[i] = let mut true = Default::default();
				} else if mode_lib.vba.ODMCombineEnablePerState[i][k] == dm_odm_combine_mode_2to1 {
					if (mode_lib.vba.HActive[k]
							> 2 * mode_lib.vba.MaximumPixelsPerLinePerDSCUnit)
						mode_lib.vba.PixelsPerLinePerDSCUnitSupport[i] = let mut false = Default::default();
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalDSCUnitsRequired = v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalDSCUnitsRequired + 2;
					if mode_lib.vba.NumberOfDSCSlices[k] > 8
						mode_lib.vba.NotEnoughDSCSlices[i] = let mut true = Default::default();
				} else {
					if mode_lib.vba.HActive[k] > mode_lib.vba.MaximumPixelsPerLinePerDSCUnit
						mode_lib.vba.PixelsPerLinePerDSCUnitSupport[i] = let mut false = Default::default();
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalDSCUnitsRequired = v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalDSCUnitsRequired + 1;
					if mode_lib.vba.NumberOfDSCSlices[k] > 4
						mode_lib.vba.NotEnoughDSCSlices[i] = let mut true = Default::default();
				}
			}
		}
		if v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.TotalDSCUnitsRequired > mode_lib.vba.NumberOfDSC
			mode_lib.vba.NotEnoughDSCUnits[i] = let mut true = Default::default();
	}

	/*DSC Delay per state*/
	for i in start_state..(int)v.soc.num_states {
		for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
			mode_lib.vba.DSCDelayPerState[i][k] = dml32_DSCDelayRequirement(
					mode_lib.vba.RequiresDSC[i][k], mode_lib.vba.ODMCombineEnablePerState[i][k],
					mode_lib.vba.DSCInputBitPerComponent[k],
					mode_lib.vba.OutputBppPerState[i][k], mode_lib.vba.HActive[k],
					mode_lib.vba.HTotal[k], mode_lib.vba.NumberOfDSCSlices[k],
					mode_lib.vba.OutputFormat[k], mode_lib.vba.Output[k],
					mode_lib.vba.PixelClock[k], mode_lib.vba.PixelClockBackEnd[k],
					mode_lib.vba.ip.dsc_delay_factor_wa);
		}

		for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
			for (m = 0; m <= mode_lib.vba.NumberOfActiveSurfaces - 1; m += 1) {
				for (j = 0; j < mode_lib.vba.let mut NumberOfActiveSurfaces = Default::default(); j += 1) {
					if (mode_lib.vba.BlendingAndTiming[k] == m && mode_lib.vba.RequiresDSC[i][m] == true) {
						mode_lib.vba.DSCDelayPerState[i][k] =
							mode_lib.vba.DSCDelayPerState[i][m];
					}
				}
			}
		}
	}

	//Calculate Swath, DET Configuration, DCFCLKDeepSleep
	//
	for i in start_state..(int) v.soc.num_states {
		for j in 0..= 1 {
			for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
				mode_lib.vba.RequiredDPPCLKThisState[k] = mode_lib.vba.RequiredDPPCLK[i][j][k];
				mode_lib.vba.NoOfDPPThisState[k] = mode_lib.vba.NoOfDPP[i][j][k];
				mode_lib.vba.ODMCombineEnableThisState[k] =
						mode_lib.vba.ODMCombineEnablePerState[i][k];
			}

			dml32_CalculateSwathAndDETConfiguration(
					mode_lib.vba.DETSizeOverride,
					mode_lib.vba.UsesMALLForPStateChange,
					mode_lib.vba.ConfigReturnBufferSizeInKByte,
					mode_lib.vba.MaxTotalDETInKByte,
					mode_lib.vba.MinCompressedBufferSizeInKByte,
					false, /* ForceSingleDPP */
					mode_lib.vba.NumberOfActiveSurfaces,
					mode_lib.vba.nomDETInKByte,
					mode_lib.vba.UseUnboundedRequesting,
					mode_lib.vba.DisableUnboundRequestIfCompBufReservedSpaceNeedAdjustment,
					mode_lib.vba.ip.pixel_chunk_size_kbytes,
					mode_lib.vba.ip.rob_buffer_size_kbytes,
					mode_lib.vba.CompressedBufferSegmentSizeInkByteFinal,
					mode_lib.vba.Output,
					mode_lib.vba.ReadBandwidthLuma,
					mode_lib.vba.ReadBandwidthChroma,
					mode_lib.vba.MaximumSwathWidthLuma,
					mode_lib.vba.MaximumSwathWidthChroma,
					mode_lib.vba.SourceRotation,
					mode_lib.vba.ViewportStationary,
					mode_lib.vba.SourcePixelFormat,
					mode_lib.vba.SurfaceTiling,
					mode_lib.vba.ViewportWidth,
					mode_lib.vba.ViewportHeight,
					mode_lib.vba.ViewportXStartY,
					mode_lib.vba.ViewportYStartY,
					mode_lib.vba.ViewportXStartC,
					mode_lib.vba.ViewportYStartC,
					mode_lib.vba.SurfaceWidthY,
					mode_lib.vba.SurfaceWidthC,
					mode_lib.vba.SurfaceHeightY,
					mode_lib.vba.SurfaceHeightC,
					mode_lib.vba.Read256BlockHeightY,
					mode_lib.vba.Read256BlockHeightC,
					mode_lib.vba.Read256BlockWidthY,
					mode_lib.vba.Read256BlockWidthC,
					mode_lib.vba.ODMCombineEnableThisState,
					mode_lib.vba.BlendingAndTiming,
					mode_lib.vba.BytePerPixelY,
					mode_lib.vba.BytePerPixelC,
					mode_lib.vba.BytePerPixelInDETY,
					mode_lib.vba.BytePerPixelInDETC,
					mode_lib.vba.HActive,
					mode_lib.vba.HRatio,
					mode_lib.vba.HRatioChroma,
					mode_lib.vba.NoOfDPPThisState,
					/* Output */
					mode_lib.vba.swath_width_luma_ub_this_state,
					mode_lib.vba.swath_width_chroma_ub_this_state,
					mode_lib.vba.SwathWidthYThisState,
					mode_lib.vba.SwathWidthCThisState,
					mode_lib.vba.SwathHeightYThisState,
					mode_lib.vba.SwathHeightCThisState,
					mode_lib.vba.DETBufferSizeInKByteThisState,
					mode_lib.vba.DETBufferSizeYThisState,
					mode_lib.vba.DETBufferSizeCThisState,
					&mode_lib.vba.UnboundedRequestEnabledThisState,
					&mode_lib.vba.CompressedBufferSizeInkByteThisState,
					&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer[0], /* Long CompBufReservedSpaceKBytes */
					&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_boolean[0], /* bool CompBufReservedSpaceNeedAdjustment/
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_boolean_array[0],
					&mode_lib.vba.ViewportSizeSupport[i][j]);

			for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
				mode_lib.vba.swath_width_luma_ub_all_states[i][j][k] =
						mode_lib.vba.swath_width_luma_ub_this_state[k];
				mode_lib.vba.swath_width_chroma_ub_all_states[i][j][k] =
						mode_lib.vba.swath_width_chroma_ub_this_state[k];
				mode_lib.vba.SwathWidthYAllStates[i][j][k] = (unsigned int)mode_lib.vba.SwathWidthYThisState[k];
				mode_lib.vba.SwathWidthCAllStates[i][j][k] = (unsigned int)mode_lib.vba.SwathWidthCThisState[k];
				mode_lib.vba.SwathHeightYAllStates[i][j][k] = mode_lib.vba.SwathHeightYThisState[k];
				mode_lib.vba.SwathHeightCAllStates[i][j][k] = mode_lib.vba.SwathHeightCThisState[k];
				mode_lib.vba.UnboundedRequestEnabledAllStates[i][j] =
						mode_lib.vba.let mut UnboundedRequestEnabledThisState = Default::default();
				mode_lib.vba.CompressedBufferSizeInkByteAllStates[i][j] =
						mode_lib.vba.let mut CompressedBufferSizeInkByteThisState = Default::default();
				mode_lib.vba.DETBufferSizeInKByteAllStates[i][j][k] =
						mode_lib.vba.DETBufferSizeInKByteThisState[k];
				mode_lib.vba.DETBufferSizeYAllStates[i][j][k] =
						mode_lib.vba.DETBufferSizeYThisState[k];
				mode_lib.vba.DETBufferSizeCAllStates[i][j][k] =
						mode_lib.vba.DETBufferSizeCThisState[k];
			}
		}
	}

	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		mode_lib.vba.cursor_bw[k] = mode_lib.vba.NumberOfCursors[k] * mode_lib.vba.CursorWidth[k][0]
				* mode_lib.vba.CursorBPP[k][0] / 8.0
				/ (mode_lib.vba.HTotal[k] / mode_lib.vba.PixelClock[k]) * mode_lib.vba.VRatio[k];
	}

	dml32_CalculateSurfaceSizeInMall(
			mode_lib.vba.NumberOfActiveSurfaces,
			mode_lib.vba.MALLAllocatedForDCNFinal,
			mode_lib.vba.UseMALLForStaticScreen,
			mode_lib.vba.UsesMALLForPStateChange,
			mode_lib.vba.DCCEnable,
			mode_lib.vba.ViewportStationary,
			mode_lib.vba.ViewportXStartY,
			mode_lib.vba.ViewportYStartY,
			mode_lib.vba.ViewportXStartC,
			mode_lib.vba.ViewportYStartC,
			mode_lib.vba.ViewportWidth,
			mode_lib.vba.ViewportHeight,
			mode_lib.vba.BytePerPixelY,
			mode_lib.vba.ViewportWidthChroma,
			mode_lib.vba.ViewportHeightChroma,
			mode_lib.vba.BytePerPixelC,
			mode_lib.vba.SurfaceWidthY,
			mode_lib.vba.SurfaceWidthC,
			mode_lib.vba.SurfaceHeightY,
			mode_lib.vba.SurfaceHeightC,
			mode_lib.vba.Read256BlockWidthY,
			mode_lib.vba.Read256BlockWidthC,
			mode_lib.vba.Read256BlockHeightY,
			mode_lib.vba.Read256BlockHeightC,
			mode_lib.vba.MacroTileWidthY,
			mode_lib.vba.MacroTileWidthC,
			mode_lib.vba.MacroTileHeightY,
			mode_lib.vba.MacroTileHeightC,
			mode_lib.vba.DCCMetaPitchY,
			mode_lib.vba.DCCMetaPitchC,

			/* Output */
			mode_lib.vba.SurfaceSizeInMALL,
			&mode_lib.vba.ExceededMALLSize);

	for (i = let mut start_state = Default::default(); i < (int)v.soc.let mut num_states = Default::default(); i += 1) {
		for (j = 0; j < 2; j += 1) {
			for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
				mode_lib.vba.swath_width_luma_ub_this_state[k] =
						mode_lib.vba.swath_width_luma_ub_all_states[i][j][k];
				mode_lib.vba.swath_width_chroma_ub_this_state[k] =
						mode_lib.vba.swath_width_chroma_ub_all_states[i][j][k];
				mode_lib.vba.SwathWidthYThisState[k] = mode_lib.vba.SwathWidthYAllStates[i][j][k];
				mode_lib.vba.SwathWidthCThisState[k] = mode_lib.vba.SwathWidthCAllStates[i][j][k];
				mode_lib.vba.SwathHeightYThisState[k] = mode_lib.vba.SwathHeightYAllStates[i][j][k];
				mode_lib.vba.SwathHeightCThisState[k] = mode_lib.vba.SwathHeightCAllStates[i][j][k];
				mode_lib.vba.DETBufferSizeInKByteThisState[k] =
						mode_lib.vba.DETBufferSizeInKByteAllStates[i][j][k];
				mode_lib.vba.DETBufferSizeYThisState[k] =
						(unsigned int)mode_lib.vba.DETBufferSizeYAllStates[i][j][k];
				mode_lib.vba.DETBufferSizeCThisState[k] =
						(unsigned int)mode_lib.vba.DETBufferSizeCAllStates[i][j][k];
				mode_lib.vba.RequiredDPPCLKThisState[k] = mode_lib.vba.RequiredDPPCLK[i][j][k];
				mode_lib.vba.NoOfDPPThisState[k] = mode_lib.vba.NoOfDPP[i][j][k];
			}

			mode_lib.vba.TotalNumberOfDCCActiveDPP[i][j] = 0;
			for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
				if mode_lib.vba.DCCEnable[k] == true {
					mode_lib.vba.TotalNumberOfDCCActiveDPP[i][j] =
							mode_lib.vba.TotalNumberOfDCCActiveDPP[i][j]
									+ mode_lib.vba.NoOfDPP[i][j][k];
				}
			}


			for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].PixelClock = mode_lib.vba.PixelClock[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].DPPPerSurface = mode_lib.vba.NoOfDPP[i][j][k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].SourceRotation = mode_lib.vba.SourceRotation[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].ViewportHeight = mode_lib.vba.ViewportHeight[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].ViewportHeightChroma = mode_lib.vba.ViewportHeightChroma[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].BlockWidth256BytesY = mode_lib.vba.Read256BlockWidthY[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].BlockHeight256BytesY = mode_lib.vba.Read256BlockHeightY[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].BlockWidth256BytesC = mode_lib.vba.Read256BlockWidthC[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].BlockHeight256BytesC = mode_lib.vba.Read256BlockHeightC[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].BlockWidthY = mode_lib.vba.MacroTileWidthY[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].BlockHeightY = mode_lib.vba.MacroTileHeightY[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].BlockWidthC = mode_lib.vba.MacroTileWidthC[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].BlockHeightC = mode_lib.vba.MacroTileHeightC[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].InterlaceEnable = mode_lib.vba.Interlace[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].HTotal = mode_lib.vba.HTotal[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].DCCEnable = mode_lib.vba.DCCEnable[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].SourcePixelFormat = mode_lib.vba.SourcePixelFormat[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].SurfaceTiling = mode_lib.vba.SurfaceTiling[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].BytePerPixelY = mode_lib.vba.BytePerPixelY[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].BytePerPixelC = mode_lib.vba.BytePerPixelC[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].ProgressiveToInterlaceUnitInOPP =
				mode_lib.vba.let mut ProgressiveToInterlaceUnitInOPP = Default::default();
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].VRatio = mode_lib.vba.VRatio[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].VRatioChroma = mode_lib.vba.VRatioChroma[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].VTaps = mode_lib.vba.vtaps[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].VTapsChroma = mode_lib.vba.VTAPsChroma[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].PitchY = mode_lib.vba.PitchY[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].DCCMetaPitchY = mode_lib.vba.DCCMetaPitchY[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].PitchC = mode_lib.vba.PitchC[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].DCCMetaPitchC = mode_lib.vba.DCCMetaPitchC[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].ViewportStationary = mode_lib.vba.ViewportStationary[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].ViewportXStart = mode_lib.vba.ViewportXStartY[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].ViewportYStart = mode_lib.vba.ViewportYStartY[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].ViewportXStartC = mode_lib.vba.ViewportXStartC[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].ViewportYStartC = mode_lib.vba.ViewportYStartC[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].FORCE_ONE_ROW_FOR_FRAME = mode_lib.vba.ForceOneRowForFrame[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].SwathHeightY = mode_lib.vba.SwathHeightYThisState[k];
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters[k].SwathHeightC = mode_lib.vba.SwathHeightCThisState[k];
			}

			{
				dml32_CalculateVMRowAndSwath(
						mode_lib.vba.NumberOfActiveSurfaces,
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SurfParameters,
						mode_lib.vba.SurfaceSizeInMALL,
						mode_lib.vba.PTEBufferSizeInRequestsLuma,
						mode_lib.vba.PTEBufferSizeInRequestsChroma,
						mode_lib.vba.DCCMetaBufferSizeBytes,
						mode_lib.vba.UseMALLForStaticScreen,
						mode_lib.vba.UsesMALLForPStateChange,
						mode_lib.vba.MALLAllocatedForDCNFinal,
						mode_lib.vba.SwathWidthYThisState,
						mode_lib.vba.SwathWidthCThisState,
						mode_lib.vba.GPUVMEnable,
						mode_lib.vba.HostVMEnable,
						mode_lib.vba.HostVMMaxNonCachedPageTableLevels,
						mode_lib.vba.GPUVMMaxPageTableLevels,
						mode_lib.vba.GPUVMMinPageSizeKBytes,
						(unsigned int)mode_lib.vba.HostVMMinPageSize,

						/* Output */
						mode_lib.vba.PTEBufferSizeNotExceededPerState,
						mode_lib.vba.DCCMetaBufferSizeNotExceededPerState,
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[0],
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[1],
						mode_lib.vba.dpte_row_height,
						mode_lib.vba.dpte_row_height_chroma,
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[2],
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[3],
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[4],
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[5],
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[6],
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[7],
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[8],
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[9],
						mode_lib.vba.meta_row_height,
						mode_lib.vba.meta_row_height_chroma,
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[10],
						mode_lib.vba.dpte_group_bytes,
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[11],
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[12],
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[13],
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[14],
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[15],
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[16],
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[17],
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[18],
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[19],
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[20],
						mode_lib.vba.PrefetchLinesYThisState,
						mode_lib.vba.PrefetchLinesCThisState,
						mode_lib.vba.PrefillY,
						mode_lib.vba.PrefillC,
						mode_lib.vba.MaxNumSwY,
						mode_lib.vba.MaxNumSwC,
						mode_lib.vba.meta_row_bandwidth_this_state,
						mode_lib.vba.dpte_row_bandwidth_this_state,
						mode_lib.vba.DPTEBytesPerRowThisState,
						mode_lib.vba.PDEAndMetaPTEBytesPerFrameThisState,
						mode_lib.vba.MetaRowBytesThisState,
						mode_lib.vba.use_one_row_for_frame_this_state,
						mode_lib.vba.use_one_row_for_frame_flip_this_state,
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_boolean_array[0], // Boolean UsesMALLForStaticScreen[]
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_boolean_array[1], // Boolean PTE_BUFFER_MODE[]
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer_array[21]); // Long BIGK_FRAGMENT_SIZE[]
			}

			for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
				mode_lib.vba.PrefetchLinesY[i][j][k] = mode_lib.vba.PrefetchLinesYThisState[k];
				mode_lib.vba.PrefetchLinesC[i][j][k] = mode_lib.vba.PrefetchLinesCThisState[k];
				mode_lib.vba.meta_row_bandwidth[i][j][k] =
						mode_lib.vba.meta_row_bandwidth_this_state[k];
				mode_lib.vba.dpte_row_bandwidth[i][j][k] =
						mode_lib.vba.dpte_row_bandwidth_this_state[k];
				mode_lib.vba.DPTEBytesPerRow[i][j][k] = mode_lib.vba.DPTEBytesPerRowThisState[k];
				mode_lib.vba.PDEAndMetaPTEBytesPerFrame[i][j][k] =
						mode_lib.vba.PDEAndMetaPTEBytesPerFrameThisState[k];
				mode_lib.vba.MetaRowBytes[i][j][k] = mode_lib.vba.MetaRowBytesThisState[k];
				mode_lib.vba.use_one_row_for_frame[i][j][k] =
						mode_lib.vba.use_one_row_for_frame_this_state[k];
				mode_lib.vba.use_one_row_for_frame_flip[i][j][k] =
						mode_lib.vba.use_one_row_for_frame_flip_this_state[k];
			}

			mode_lib.vba.PTEBufferSizeNotExceeded[i][j] = let mut true = Default::default();
			for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
				if mode_lib.vba.PTEBufferSizeNotExceededPerState[k] == false
					mode_lib.vba.PTEBufferSizeNotExceeded[i][j] = let mut false = Default::default();
			}

			mode_lib.vba.DCCMetaBufferSizeNotExceeded[i][j] = let mut true = Default::default();
			for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
				if mode_lib.vba.DCCMetaBufferSizeNotExceededPerState[k] == false
					mode_lib.vba.DCCMetaBufferSizeNotExceeded[i][j] = let mut false = Default::default();
			}

			mode_lib.vba.UrgLatency[i] = dml32_CalculateUrgentLatency(
					mode_lib.vba.UrgentLatencyPixelDataOnly,
					mode_lib.vba.UrgentLatencyPixelMixedWithVMData,
					mode_lib.vba.UrgentLatencyVMDataOnly, mode_lib.vba.DoUrgentLatencyAdjustment,
					mode_lib.vba.UrgentLatencyAdjustmentFabricClockComponent,
					mode_lib.vba.UrgentLatencyAdjustmentFabricClockReference,
					mode_lib.vba.FabricClockPerState[i]);

			//bool   NotUrgentLatencyHiding[DC__NUM_DPP__MAX];
			for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
				dml32_CalculateUrgentBurstFactor(
						mode_lib.vba.UsesMALLForPStateChange[k],
						mode_lib.vba.swath_width_luma_ub_this_state[k],
						mode_lib.vba.swath_width_chroma_ub_this_state[k],
						mode_lib.vba.SwathHeightYThisState[k],
						mode_lib.vba.SwathHeightCThisState[k],
						(double) mode_lib.vba.HTotal[k] / mode_lib.vba.PixelClock[k],
						mode_lib.vba.UrgLatency[i],
						mode_lib.vba.CursorBufferSize,
						mode_lib.vba.CursorWidth[k][0],
						mode_lib.vba.CursorBPP[k][0],
						mode_lib.vba.VRatio[k],
						mode_lib.vba.VRatioChroma[k],
						mode_lib.vba.BytePerPixelInDETY[k],
						mode_lib.vba.BytePerPixelInDETC[k],
						mode_lib.vba.DETBufferSizeYThisState[k],
						mode_lib.vba.DETBufferSizeCThisState[k],
						/* Output */
						&mode_lib.vba.UrgentBurstFactorCursor[k],
						&mode_lib.vba.UrgentBurstFactorLuma[k],
						&mode_lib.vba.UrgentBurstFactorChroma[k],
						&mode_lib.vba.NoUrgentLatencyHiding[k]);
			}

			dml32_CalculateDCFCLKDeepSleep(
					mode_lib.vba.NumberOfActiveSurfaces,
					mode_lib.vba.BytePerPixelY,
					mode_lib.vba.BytePerPixelC,
					mode_lib.vba.VRatio,
					mode_lib.vba.VRatioChroma,
					mode_lib.vba.SwathWidthYThisState,
					mode_lib.vba.SwathWidthCThisState,
					mode_lib.vba.NoOfDPPThisState,
					mode_lib.vba.HRatio,
					mode_lib.vba.HRatioChroma,
					mode_lib.vba.PixelClock,
					mode_lib.vba.PSCL_FACTOR,
					mode_lib.vba.PSCL_FACTOR_CHROMA,
					mode_lib.vba.RequiredDPPCLKThisState,
					mode_lib.vba.ReadBandwidthLuma,
					mode_lib.vba.ReadBandwidthChroma,
					(int)mode_lib.vba.ReturnBusWidth,

					/* Output */
					&mode_lib.vba.ProjectedDCFCLKDeepSleep[i][j]);
		}
	}

	//Calculate Return BW
	for i in start_state..(int) v.soc.num_states {
		for j in 0..= 1 {
			for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
				if mode_lib.vba.BlendingAndTiming[k] == k {
					if mode_lib.vba.WritebackEnable[k] == true {
						mode_lib.vba.WritebackDelayTime[k] =
							mode_lib.vba.WritebackLatency
						+ dml32_CalculateWriteBackDelay(
							mode_lib.vba.WritebackPixelFormat[k],
							mode_lib.vba.WritebackHRatio[k],
							mode_lib.vba.WritebackVRatio[k],
							mode_lib.vba.WritebackVTaps[k],
							(long)mode_lib.vba.WritebackDestinationWidth[k],
							(int)mode_lib.vba.WritebackDestinationHeight[k],
							(int)mode_lib.vba.WritebackSourceHeight[k],
							mode_lib.vba.HTotal[k])
							/ mode_lib.vba.RequiredDISPCLK[i][j];
					} else {
						mode_lib.vba.WritebackDelayTime[k] = 0.0;
					}
					for (m = 0; m <= mode_lib.vba.NumberOfActiveSurfaces - 1; m += 1) {
						if (mode_lib.vba.BlendingAndTiming[m]
								== k && mode_lib.vba.WritebackEnable[m] == true) {
							mode_lib.vba.WritebackDelayTime[k] =
								dml_max(mode_lib.vba.WritebackDelayTime[k],
									mode_lib.vba.WritebackLatency
								+ dml32_CalculateWriteBackDelay(
									mode_lib.vba.WritebackPixelFormat[m],
									mode_lib.vba.WritebackHRatio[m],
									mode_lib.vba.WritebackVRatio[m],
									mode_lib.vba.WritebackVTaps[m],
									(int)mode_lib.vba.WritebackDestinationWidth[m],
									(int)mode_lib.vba.WritebackDestinationHeight[m],
									(int)mode_lib.vba.WritebackSourceHeight[m],
									mode_lib.vba.HTotal[m]) /
									mode_lib.vba.RequiredDISPCLK[i][j]);
						}
					}
				}
			}
			for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
				for (m = 0; m <= mode_lib.vba.NumberOfActiveSurfaces - 1; m += 1) {
					if mode_lib.vba.BlendingAndTiming[k] == m {
						mode_lib.vba.WritebackDelayTime[k] =
								mode_lib.vba.WritebackDelayTime[m];
					}
				}
			}
			mode_lib.vba.MaxMaxVStartup[i][j] = 0;
			for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
				mode_lib.vba.MaximumVStartup[i][j][k] = ((mode_lib.vba.Interlace[k] &&
								!mode_lib.vba.ProgressiveToInterlaceUnitInOPP) ?
								dml_floor((mode_lib.vba.VTotal[k] -
									mode_lib.vba.VActive[k]) / 2.0, 1.0) :
								mode_lib.vba.VTotal[k] - mode_lib.vba.VActive[k])
								- dml_max(1.0, dml_ceil(1.0 *
									mode_lib.vba.WritebackDelayTime[k] /
									(mode_lib.vba.HTotal[k] /
									mode_lib.vba.PixelClock[k]), 1.0));

				// Clamp to max OTG vstartup register limit
				if mode_lib.vba.MaximumVStartup[i][j][k] > 1023
					mode_lib.vba.MaximumVStartup[i][j][k] = 1023;

				mode_lib.vba.MaxMaxVStartup[i][j] = dml_max(mode_lib.vba.MaxMaxVStartup[i][j],
						mode_lib.vba.MaximumVStartup[i][j][k]);
			}
		}
	}

	v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.ReorderingBytes = (unsigned int)(mode_lib.vba.NumberOfChannels
			* dml_max3(mode_lib.vba.UrgentOutOfOrderReturnPerChannelPixelDataOnly,
					mode_lib.vba.UrgentOutOfOrderReturnPerChannelPixelMixedWithVMData,
					mode_lib.vba.UrgentOutOfOrderReturnPerChannelVMDataOnly));

	dml32_CalculateMinAndMaxPrefetchMode(mode_lib.vba.AllowForPStateChangeOrStutterInVBlankFinal,
			&mode_lib.vba.MinPrefetchMode,
			&mode_lib.vba.MaxPrefetchMode);

	for i in start_state..(int) v.soc.num_states {
		for j in 0..= 1
			mode_lib.vba.DCFCLKState[i][j] = mode_lib.vba.DCFCLKPerState[i];
	}

	/* Immediate Flip and MALL parameters */
	mode_lib.vba.ImmediateFlipRequiredFinal = let mut false = Default::default();
	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		mode_lib.vba.ImmediateFlipRequiredFinal = mode_lib.vba.ImmediateFlipRequiredFinal
				|| (mode_lib.vba.ImmediateFlipRequirement[k] == dm_immediate_flip_required);
	}

	mode_lib.vba.ImmediateFlipRequiredButTheRequirementForEachSurfaceIsNotSpecified = let mut false = Default::default();
	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		mode_lib.vba.ImmediateFlipRequiredButTheRequirementForEachSurfaceIsNotSpecified =
				mode_lib.vba.ImmediateFlipRequiredButTheRequirementForEachSurfaceIsNotSpecified
						|| ((mode_lib.vba.ImmediateFlipRequirement[k]
								!= dm_immediate_flip_required)
								&& (mode_lib.vba.ImmediateFlipRequirement[k]
										!= dm_immediate_flip_not_required));
	}
	mode_lib.vba.ImmediateFlipRequiredButTheRequirementForEachSurfaceIsNotSpecified =
			mode_lib.vba.ImmediateFlipRequiredButTheRequirementForEachSurfaceIsNotSpecified && mode_lib.vba.let mut ImmediateFlipRequiredFinal = Default::default();

	mode_lib.vba.ImmediateFlipOrHostVMAndPStateWithMALLFullFrameOrPhantomPipe = let mut false = Default::default();
	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		mode_lib.vba.ImmediateFlipOrHostVMAndPStateWithMALLFullFrameOrPhantomPipe =
			mode_lib.vba.ImmediateFlipOrHostVMAndPStateWithMALLFullFrameOrPhantomPipe ||
			((mode_lib.vba.HostVMEnable == true || mode_lib.vba.ImmediateFlipRequirement[k] !=
					dm_immediate_flip_not_required) &&
			(mode_lib.vba.UsesMALLForPStateChange[k] == dm_use_mall_pstate_change_full_frame || mode_lib.vba.UsesMALLForPStateChange[k] == dm_use_mall_pstate_change_phantom_pipe));
	}

	mode_lib.vba.InvalidCombinationOfMALLUseForPStateAndStaticScreen = let mut false = Default::default();
	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		mode_lib.vba.InvalidCombinationOfMALLUseForPStateAndStaticScreen =
			mode_lib.vba.InvalidCombinationOfMALLUseForPStateAndStaticScreen
			|| ((mode_lib.vba.UseMALLForStaticScreen[k] == dm_use_mall_static_screen_enable || mode_lib.vba.UseMALLForStaticScreen[k] == dm_use_mall_static_screen_optimize)
			&& (mode_lib.vba.UsesMALLForPStateChange[k] == dm_use_mall_pstate_change_phantom_pipe))
			|| ((mode_lib.vba.UseMALLForStaticScreen[k] == dm_use_mall_static_screen_disable || mode_lib.vba.UseMALLForStaticScreen[k] == dm_use_mall_static_screen_optimize)
			&& (mode_lib.vba.UsesMALLForPStateChange[k] == dm_use_mall_pstate_change_full_frame));
	}

	v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.FullFrameMALLPStateMethod = let mut false = Default::default();
	v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SubViewportMALLPStateMethod = let mut false = Default::default();
	v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.PhantomPipeMALLPStateMethod = let mut false = Default::default();

	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		if mode_lib.vba.UsesMALLForPStateChange[k] == dm_use_mall_pstate_change_full_frame
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.FullFrameMALLPStateMethod = let mut true = Default::default();
		if mode_lib.vba.UsesMALLForPStateChange[k] == dm_use_mall_pstate_change_sub_viewport
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SubViewportMALLPStateMethod = let mut true = Default::default();
		if mode_lib.vba.UsesMALLForPStateChange[k] == dm_use_mall_pstate_change_phantom_pipe
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.PhantomPipeMALLPStateMethod = let mut true = Default::default();
	}
	mode_lib.vba.InvalidCombinationOfMALLUseForPState = (v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SubViewportMALLPStateMethod
			!= v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.PhantomPipeMALLPStateMethod) || (v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.SubViewportMALLPStateMethod && v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.FullFrameMALLPStateMethod);

	if mode_lib.vba.UseMinimumRequiredDCFCLK == true {
		dml32_UseMinimumDCFCLK(
				mode_lib.vba.UsesMALLForPStateChange,
				mode_lib.vba.DRRDisplay,
				mode_lib.vba.SynchronizeDRRDisplaysForUCLKPStateChangeFinal,
				mode_lib.vba.MaxInterDCNTileRepeaters,
				mode_lib.vba.MaxPrefetchMode,
				mode_lib.vba.DRAMClockChangeLatency,
				mode_lib.vba.FCLKChangeLatency,
				mode_lib.vba.SREnterPlusExitTime,
				(int)mode_lib.vba.ReturnBusWidth,
				(int)mode_lib.vba.RoundTripPingLatencyCycles,
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.ReorderingBytes,
				mode_lib.vba.PixelChunkSizeInKByte,
				mode_lib.vba.MetaChunkSize,
				mode_lib.vba.GPUVMEnable,
				mode_lib.vba.GPUVMMaxPageTableLevels,
				mode_lib.vba.HostVMEnable,
				mode_lib.vba.NumberOfActiveSurfaces,
				(unsigned int)mode_lib.vba.HostVMMinPageSize,
				mode_lib.vba.HostVMMaxNonCachedPageTableLevels,
				mode_lib.vba.DynamicMetadataVMEnabled,
				mode_lib.vba.ImmediateFlipRequiredFinal,
				mode_lib.vba.ProgressiveToInterlaceUnitInOPP,
				mode_lib.vba.MaxAveragePercentOfIdealSDPPortBWDisplayCanUseInNormalSystemOperation,
				mode_lib.vba.PercentOfIdealFabricAndSDPPortBWReceivedAfterUrgLatency,
				mode_lib.vba.VTotal,
				mode_lib.vba.VActive,
				mode_lib.vba.DynamicMetadataTransmittedBytes,
				mode_lib.vba.DynamicMetadataLinesBeforeActiveRequired,
				mode_lib.vba.Interlace,
				mode_lib.vba.RequiredDPPCLK,
				mode_lib.vba.RequiredDISPCLK,
				mode_lib.vba.UrgLatency,
				mode_lib.vba.NoOfDPP,
				mode_lib.vba.ProjectedDCFCLKDeepSleep,
				mode_lib.vba.MaximumVStartup,
				mode_lib.vba.TotalNumberOfActiveDPP,
				mode_lib.vba.TotalNumberOfDCCActiveDPP,
				mode_lib.vba.dpte_group_bytes,
				mode_lib.vba.PrefetchLinesY,
				mode_lib.vba.PrefetchLinesC,
				mode_lib.vba.swath_width_luma_ub_all_states,
				mode_lib.vba.swath_width_chroma_ub_all_states,
				mode_lib.vba.BytePerPixelY,
				mode_lib.vba.BytePerPixelC,
				mode_lib.vba.HTotal,
				mode_lib.vba.PixelClock,
				mode_lib.vba.PDEAndMetaPTEBytesPerFrame,
				mode_lib.vba.DPTEBytesPerRow,
				mode_lib.vba.MetaRowBytes,
				mode_lib.vba.DynamicMetadataEnable,
				mode_lib.vba.ReadBandwidthLuma,
				mode_lib.vba.ReadBandwidthChroma,
				mode_lib.vba.DCFCLKPerState,

				/* Output */
				mode_lib.vba.DCFCLKState);
	} // UseMinimumRequiredDCFCLK == true

	for i in start_state..(int) v.soc.num_states {
		for j in 0..= 1 {
			mode_lib.vba.ReturnBWPerState[i][j] = dml32_get_return_bw_mbps(&mode_lib.vba.soc, i,
					mode_lib.vba.HostVMEnable, mode_lib.vba.DCFCLKState[i][j],
					mode_lib.vba.FabricClockPerState[i], mode_lib.vba.DRAMSpeedPerState[i]);
		}
	}

	//Re-ordering Buffer Support Check
	for i in start_state..(int) v.soc.num_states {
		for j in 0..= 1 {
			if (mode_lib.vba.ROBBufferSizeInKByte - mode_lib.vba.PixelChunkSizeInKByte * 1024
					/ mode_lib.vba.ReturnBWPerState[i][j]
					> (mode_lib.vba.RoundTripPingLatencyCycles + 32)
							/ mode_lib.vba.DCFCLKState[i][j]
							+ v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.ReorderingBytes / mode_lib.vba.ReturnBWPerState[i][j]) {
				mode_lib.vba.ROBSupport[i][j] = let mut true = Default::default();
			} else {
				mode_lib.vba.ROBSupport[i][j] = let mut false = Default::default();
			}
		}
	}

	//Vertical Active BW support check
	v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.MaxTotalVActiveRDBandwidth = 0;

	for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
		v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.MaxTotalVActiveRDBandwidth += mode_lib.vba.ReadBandwidthLuma[k]
				+ mode_lib.vba.ReadBandwidthChroma[k];
	}

	for i in start_state..(int) v.soc.num_states {
		for j in 0..= 1 {
			mode_lib.vba.MaxTotalVerticalActiveAvailableBandwidth[i][j] =
				dml_min3(mode_lib.vba.ReturnBusWidth * mode_lib.vba.DCFCLKState[i][j]
					* mode_lib.vba.MaxAveragePercentOfIdealSDPPortBWDisplayCanUseInNormalSystemOperation / 100,
					mode_lib.vba.FabricClockPerState[i]
					* mode_lib.vba.FabricDatapathToDCNDataReturn
					* mode_lib.vba.MaxAveragePercentOfIdealFabricBWDisplayCanUseInNormalSystemOperation / 100,
					mode_lib.vba.DRAMSpeedPerState[i]
					* mode_lib.vba.NumberOfChannels
					* mode_lib.vba.DRAMChannelWidth
					* (i < 2 ? mode_lib.vba.MaxAveragePercentOfIdealDRAMBWDisplayCanUseInNormalSystemOperationSTROBE : mode_lib.vba.MaxAveragePercentOfIdealDRAMBWDisplayCanUseInNormalSystemOperation) / 100);

			if (v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.MaxTotalVActiveRDBandwidth
					<= mode_lib.vba.MaxTotalVerticalActiveAvailableBandwidth[i][j]) {
				mode_lib.vba.TotalVerticalActiveBandwidthSupport[i][j] = let mut true = Default::default();
			} else {
				mode_lib.vba.TotalVerticalActiveBandwidthSupport[i][j] = let mut false = Default::default();
			}
		}
	}

	/* Prefetch Check */

	for i in start_state..(int) v.soc.num_states {
		for j in 0..= 1 {

			mode_lib.vba.TimeCalc = 24 / mode_lib.vba.ProjectedDCFCLKDeepSleep[i][j];

			for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
				mode_lib.vba.NoOfDPPThisState[k] = mode_lib.vba.NoOfDPP[i][j][k];
				mode_lib.vba.swath_width_luma_ub_this_state[k] =
						mode_lib.vba.swath_width_luma_ub_all_states[i][j][k];
				mode_lib.vba.swath_width_chroma_ub_this_state[k] =
						mode_lib.vba.swath_width_chroma_ub_all_states[i][j][k];
				mode_lib.vba.SwathWidthYThisState[k] = mode_lib.vba.SwathWidthYAllStates[i][j][k];
				mode_lib.vba.SwathWidthCThisState[k] = mode_lib.vba.SwathWidthCAllStates[i][j][k];
				mode_lib.vba.SwathHeightYThisState[k] = mode_lib.vba.SwathHeightYAllStates[i][j][k];
				mode_lib.vba.SwathHeightCThisState[k] = mode_lib.vba.SwathHeightCAllStates[i][j][k];
				mode_lib.vba.UnboundedRequestEnabledThisState =
						mode_lib.vba.UnboundedRequestEnabledAllStates[i][j];
				mode_lib.vba.CompressedBufferSizeInkByteThisState =
						mode_lib.vba.CompressedBufferSizeInkByteAllStates[i][j];
				mode_lib.vba.DETBufferSizeInKByteThisState[k] =
						mode_lib.vba.DETBufferSizeInKByteAllStates[i][j][k];
				mode_lib.vba.DETBufferSizeYThisState[k] =
						(unsigned int)mode_lib.vba.DETBufferSizeYAllStates[i][j][k];
				mode_lib.vba.DETBufferSizeCThisState[k] =
						(unsigned int)mode_lib.vba.DETBufferSizeCAllStates[i][j][k];
			}

			mode_lib.vba.VActiveBandwithSupport[i][j] = dml32_CalculateVActiveBandwithSupport(
					mode_lib.vba.NumberOfActiveSurfaces,
					mode_lib.vba.ReturnBWPerState[i][j],
					mode_lib.vba.NoUrgentLatencyHiding,
					mode_lib.vba.ReadBandwidthLuma,
					mode_lib.vba.ReadBandwidthChroma,
					mode_lib.vba.cursor_bw,
					mode_lib.vba.meta_row_bandwidth_this_state,
					mode_lib.vba.dpte_row_bandwidth_this_state,
					mode_lib.vba.NoOfDPPThisState,
					mode_lib.vba.UrgentBurstFactorLuma,
					mode_lib.vba.UrgentBurstFactorChroma,
					mode_lib.vba.UrgentBurstFactorCursor);

			mode_lib.vba.NotEnoughDETSwathFillLatencyHidingPerState[i][j] = dml32_CalculateDETSwathFillLatencyHiding(
					mode_lib.vba.NumberOfActiveSurfaces,
					mode_lib.vba.ReturnBWPerState[i][j],
					mode_lib.vba.UrgLatency[i],
					mode_lib.vba.SwathHeightYThisState,
					mode_lib.vba.SwathHeightCThisState,
					mode_lib.vba.swath_width_luma_ub_this_state,
					mode_lib.vba.swath_width_chroma_ub_this_state,
					mode_lib.vba.BytePerPixelInDETY,
					mode_lib.vba.BytePerPixelInDETC,
					mode_lib.vba.DETBufferSizeYThisState,
					mode_lib.vba.DETBufferSizeCThisState,
					mode_lib.vba.NoOfDPPThisState,
					mode_lib.vba.HTotal,
					mode_lib.vba.PixelClock,
					mode_lib.vba.VRatio,
					mode_lib.vba.VRatioChroma,
					mode_lib.vba.UsesMALLForPStateChange,
					mode_lib.vba.UseUnboundedRequesting);

			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.VMDataOnlyReturnBWPerState = dml32_get_return_bw_mbps_vm_only(&mode_lib.vba.soc, i,
					mode_lib.vba.DCFCLKState[i][j], mode_lib.vba.FabricClockPerState[i],
					mode_lib.vba.DRAMSpeedPerState[i]);
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.HostVMInefficiencyFactor = 1;

			if mode_lib.vba.GPUVMEnable && mode_lib.vba.HostVMEnable
				v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.HostVMInefficiencyFactor = mode_lib.vba.ReturnBWPerState[i][j]
						/ v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.let mut VMDataOnlyReturnBWPerState = Default::default();

			mode_lib.vba.ExtraLatency = dml32_CalculateExtraLatency(
					(int)mode_lib.vba.RoundTripPingLatencyCycles, v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.ReorderingBytes,
					mode_lib.vba.DCFCLKState[i][j], mode_lib.vba.TotalNumberOfActiveDPP[i][j],
					mode_lib.vba.PixelChunkSizeInKByte,
					mode_lib.vba.TotalNumberOfDCCActiveDPP[i][j], mode_lib.vba.MetaChunkSize,
					mode_lib.vba.ReturnBWPerState[i][j], mode_lib.vba.GPUVMEnable,
					mode_lib.vba.HostVMEnable, mode_lib.vba.NumberOfActiveSurfaces,
					mode_lib.vba.NoOfDPPThisState, mode_lib.vba.dpte_group_bytes,
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.HostVMInefficiencyFactor, (unsigned int)mode_lib.vba.HostVMMinPageSize,
					mode_lib.vba.HostVMMaxNonCachedPageTableLevels);

			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.NextPrefetchModeState = mode_lib.vba.let mut MinPrefetchMode = Default::default();

			mode_lib.vba.NextMaxVStartup = mode_lib.vba.MaxMaxVStartup[i][j];

			do {
				mode_lib.vba.PrefetchModePerState[i][j] = v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.let mut NextPrefetchModeState = Default::default();
				mode_lib.vba.MaxVStartup = mode_lib.vba.let mut NextMaxVStartup = Default::default();

				for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
					mode_lib.vba.TWait = dml32_CalculateTWait(
							mode_lib.vba.PrefetchModePerState[i][j],
							mode_lib.vba.UsesMALLForPStateChange[k],
							mode_lib.vba.SynchronizeDRRDisplaysForUCLKPStateChangeFinal,
							mode_lib.vba.DRRDisplay[k],
							mode_lib.vba.DRAMClockChangeLatency,
							mode_lib.vba.FCLKChangeLatency, mode_lib.vba.UrgLatency[i],
							mode_lib.vba.SREnterPlusExitTime);

					memset(&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull, 0, core::mem::size_of::<DmlPipe>());
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.Dppclk = mode_lib.vba.RequiredDPPCLK[i][j][k];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.Dispclk = mode_lib.vba.RequiredDISPCLK[i][j];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.PixelClock = mode_lib.vba.PixelClock[k];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.DCFClkDeepSleep = mode_lib.vba.ProjectedDCFCLKDeepSleep[i][j];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.DPPPerSurface = mode_lib.vba.NoOfDPP[i][j][k];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.ScalerEnabled = mode_lib.vba.ScalerEnabled[k];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.SourceRotation = mode_lib.vba.SourceRotation[k];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.BlockWidth256BytesY = mode_lib.vba.Read256BlockWidthY[k];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.BlockHeight256BytesY = mode_lib.vba.Read256BlockHeightY[k];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.BlockWidth256BytesC = mode_lib.vba.Read256BlockWidthC[k];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.BlockHeight256BytesC = mode_lib.vba.Read256BlockHeightC[k];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.InterlaceEnable = mode_lib.vba.Interlace[k];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.NumberOfCursors = mode_lib.vba.NumberOfCursors[k];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.VBlank = mode_lib.vba.VTotal[k] - mode_lib.vba.VActive[k];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.HTotal = mode_lib.vba.HTotal[k];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.HActive = mode_lib.vba.HActive[k];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.DCCEnable = mode_lib.vba.DCCEnable[k];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.ODMMode = mode_lib.vba.ODMCombineEnablePerState[i][k];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.SourcePixelFormat = mode_lib.vba.SourcePixelFormat[k];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.BytePerPixelY = mode_lib.vba.BytePerPixelY[k];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.BytePerPixelC = mode_lib.vba.BytePerPixelC[k];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe.ProgressiveToInterlaceUnitInOPP =
							mode_lib.vba.let mut ProgressiveToInterlaceUnitInOPP = Default::default();

					mode_lib.vba.NoTimeForPrefetch[i][j][k] =
						dml32_CalculatePrefetchSchedule(
							v,
							k,
							v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.HostVMInefficiencyFactor,
							&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.myPipe,
							(unsigned int)v.DSCDelayPerState[i][k],
							(unsigned int)(v.SwathWidthYThisState[k] / v.HRatio[k]),
							(unsigned int)dml_min(v.MaxVStartup, v.MaximumVStartup[i][j][k]),
							(unsigned int)v.MaximumVStartup[i][j][k],
							v.UrgLatency[i],
							v.ExtraLatency,
							v.TimeCalc,
							(unsigned int)v.PDEAndMetaPTEBytesPerFrame[i][j][k],
							(unsigned int)v.MetaRowBytes[i][j][k],
							(unsigned int)v.DPTEBytesPerRow[i][j][k],
							v.PrefetchLinesY[i][j][k],
							(unsigned int)v.SwathWidthYThisState[k],
							(unsigned int)v.PrefillY[k],
							v.MaxNumSwY[k],
							v.PrefetchLinesC[i][j][k],
							(unsigned int)v.SwathWidthCThisState[k],
							(unsigned int)v.PrefillC[k],
							v.MaxNumSwC[k],
							v.swath_width_luma_ub_this_state[k],
							v.swath_width_chroma_ub_this_state[k],
							v.SwathHeightYThisState[k],
							v.SwathHeightCThisState[k], v.TWait,
							(v.DRAMSpeedPerState[i] <= MEM_STROBE_FREQ_MHZ || v.DCFCLKState[i][j] <= DCFCLK_FREQ_EXTRA_PREFETCH_REQ_MHZ) ?
									mode_lib.vba.ip.min_prefetch_in_strobe_us : 0,
							mode_lib.vba.PrefetchModePerState[i][j] > 0 || mode_lib.vba.DRAMClockChangeRequirementFinal == false,

							/* Output */
							&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.DSTXAfterScaler[k],
							&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.DSTYAfterScaler[k],
							&v.LineTimesForPrefetch[k],
							&v.PrefetchBW[k],
							&v.LinesForMetaPTE[k],
							&v.LinesForMetaAndDPTERow[k],
							&v.VRatioPreY[i][j][k],
							&v.VRatioPreC[i][j][k],
							&v.RequiredPrefetchPixelDataBWLuma[0][0][k],
							&v.RequiredPrefetchPixelDataBWChroma[0][0][k],
							&v.NoTimeForDynamicMetadata[i][j][k],
							&v.Tno_bw[k],
							&v.prefetch_vmrow_bw[k],
							&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_single[0],         // double *Tdmdl_vm
							&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_single[1],         // double *Tdmdl
							&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_single[2],         // double *TSetup
							&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer[0],         							    // unsigned int   *VUpdateOffsetPix
							&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_single[3],         // unsigned int   *VUpdateWidthPix
							&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_single[4]);        // unsigned int   *VReadyOffsetPix
				}

				for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
					dml32_CalculateUrgentBurstFactor(
							mode_lib.vba.UsesMALLForPStateChange[k],
							mode_lib.vba.swath_width_luma_ub_this_state[k],
							mode_lib.vba.swath_width_chroma_ub_this_state[k],
							mode_lib.vba.SwathHeightYThisState[k],
							mode_lib.vba.SwathHeightCThisState[k],
							mode_lib.vba.HTotal[k] / mode_lib.vba.PixelClock[k],
							mode_lib.vba.UrgLatency[i], mode_lib.vba.CursorBufferSize,
							mode_lib.vba.CursorWidth[k][0], mode_lib.vba.CursorBPP[k][0],
							mode_lib.vba.VRatioPreY[i][j][k],
							mode_lib.vba.VRatioPreC[i][j][k],
							mode_lib.vba.BytePerPixelInDETY[k],
							mode_lib.vba.BytePerPixelInDETC[k],
							mode_lib.vba.DETBufferSizeYThisState[k],
							mode_lib.vba.DETBufferSizeCThisState[k],
							/* Output */
							&mode_lib.vba.UrgentBurstFactorCursorPre[k],
							&mode_lib.vba.UrgentBurstFactorLumaPre[k],
							&mode_lib.vba.UrgentBurstFactorChromaPre[k],
							&mode_lib.vba.NotUrgentLatencyHidingPre[k]);

					v.cursor_bw_pre[k] = mode_lib.vba.NumberOfCursors[k] * mode_lib.vba.CursorWidth[k][0] * mode_lib.vba.CursorBPP[k][0] /
							8.0 / (mode_lib.vba.HTotal[k] / mode_lib.vba.PixelClock[k]) * v.VRatioPreY[i][j][k];
				}

				{
					dml32_CalculatePrefetchBandwithSupport(
							mode_lib.vba.NumberOfActiveSurfaces,
							mode_lib.vba.ReturnBWPerState[i][j],
							mode_lib.vba.NotUrgentLatencyHidingPre,
							mode_lib.vba.ReadBandwidthLuma,
							mode_lib.vba.ReadBandwidthChroma,
							mode_lib.vba.RequiredPrefetchPixelDataBWLuma[0][0],
							mode_lib.vba.RequiredPrefetchPixelDataBWChroma[0][0],
							mode_lib.vba.cursor_bw,
							mode_lib.vba.meta_row_bandwidth_this_state,
							mode_lib.vba.dpte_row_bandwidth_this_state,
							mode_lib.vba.cursor_bw_pre,
							mode_lib.vba.prefetch_vmrow_bw,
							mode_lib.vba.NoOfDPPThisState,
							mode_lib.vba.UrgentBurstFactorLuma,
							mode_lib.vba.UrgentBurstFactorChroma,
							mode_lib.vba.UrgentBurstFactorCursor,
							mode_lib.vba.UrgentBurstFactorLumaPre,
							mode_lib.vba.UrgentBurstFactorChromaPre,
							mode_lib.vba.UrgentBurstFactorCursorPre,
							v.PrefetchBW,
							v.VRatio,
							v.MaxVRatioPre,

							/* output */
							&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_single[0],   // Single  *PrefetchBandwidth
							&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_single[1],   // Single  *FractionOfUrgentBandwidth
							&mode_lib.vba.PrefetchSupported[i][j]);
				}

				for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
					if (mode_lib.vba.LineTimesForPrefetch[k]
							< 2.0 || mode_lib.vba.LinesForMetaPTE[k] >= 32.0 || mode_lib.vba.LinesForMetaAndDPTERow[k] >= 16.0 || mode_lib.vba.NoTimeForPrefetch[i][j][k] == true) {
						mode_lib.vba.PrefetchSupported[i][j] = let mut false = Default::default();
					}
				}

				mode_lib.vba.DynamicMetadataSupported[i][j] = let mut true = Default::default();
				for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
					if mode_lib.vba.NoTimeForDynamicMetadata[i][j][k] == true
						mode_lib.vba.DynamicMetadataSupported[i][j] = let mut false = Default::default();
				}

				mode_lib.vba.VRatioInPrefetchSupported[i][j] = let mut true = Default::default();
				for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
					if (mode_lib.vba.VRatioPreY[i][j][k] > mode_lib.vba.MaxVRatioPre || mode_lib.vba.VRatioPreC[i][j][k] > mode_lib.vba.MaxVRatioPre || mode_lib.vba.NoTimeForPrefetch[i][j][k] == true) {
						mode_lib.vba.VRatioInPrefetchSupported[i][j] = let mut false = Default::default();
					}
				}
				mode_lib.vba.AnyLinesForVMOrRowTooLarge = let mut false = Default::default();
				for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
					if (mode_lib.vba.LinesForMetaAndDPTERow[k] >= 16 || mode_lib.vba.LinesForMetaPTE[k] >= 32) {
						mode_lib.vba.AnyLinesForVMOrRowTooLarge = let mut true = Default::default();
					}
				}

				if (mode_lib.vba.PrefetchSupported[i][j] == true && mode_lib.vba.VRatioInPrefetchSupported[i][j] == true) {
					mode_lib.vba.BandwidthAvailableForImmediateFlip =
							dml32_CalculateBandwidthAvailableForImmediateFlip(
							mode_lib.vba.NumberOfActiveSurfaces,
							mode_lib.vba.ReturnBWPerState[i][j],
							mode_lib.vba.ReadBandwidthLuma,
							mode_lib.vba.ReadBandwidthChroma,
							mode_lib.vba.RequiredPrefetchPixelDataBWLuma[0][0],
							mode_lib.vba.RequiredPrefetchPixelDataBWChroma[0][0],
							mode_lib.vba.cursor_bw,
							mode_lib.vba.cursor_bw_pre,
							mode_lib.vba.NoOfDPPThisState,
							mode_lib.vba.UrgentBurstFactorLuma,
							mode_lib.vba.UrgentBurstFactorChroma,
							mode_lib.vba.UrgentBurstFactorCursor,
							mode_lib.vba.UrgentBurstFactorLumaPre,
							mode_lib.vba.UrgentBurstFactorChromaPre,
							mode_lib.vba.UrgentBurstFactorCursorPre);

					mode_lib.vba.TotImmediateFlipBytes = 0;
					for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
						if (!(mode_lib.vba.ImmediateFlipRequirement[k] ==
								dm_immediate_flip_not_required)) {
							mode_lib.vba.TotImmediateFlipBytes =
									(unsigned int)(mode_lib.vba.TotImmediateFlipBytes
								+ mode_lib.vba.NoOfDPP[i][j][k]
								* mode_lib.vba.PDEAndMetaPTEBytesPerFrame[i][j][k]
								+ mode_lib.vba.MetaRowBytes[i][j][k]);
							if mode_lib.vba.use_one_row_for_frame_flip[i][j][k] {
								mode_lib.vba.TotImmediateFlipBytes =
									(unsigned int)(mode_lib.vba.TotImmediateFlipBytes + 2
								* mode_lib.vba.DPTEBytesPerRow[i][j][k]);
							} else {
								mode_lib.vba.TotImmediateFlipBytes =
									(unsigned int)(mode_lib.vba.TotImmediateFlipBytes
								+ mode_lib.vba.DPTEBytesPerRow[i][j][k]);
							}
						}
					}

					for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
						dml32_CalculateFlipSchedule(v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.HostVMInefficiencyFactor,
							mode_lib.vba.ExtraLatency,
							mode_lib.vba.UrgLatency[i],
							mode_lib.vba.GPUVMMaxPageTableLevels,
							mode_lib.vba.HostVMEnable,
							mode_lib.vba.HostVMMaxNonCachedPageTableLevels,
							mode_lib.vba.GPUVMEnable,
							(unsigned int)mode_lib.vba.HostVMMinPageSize,
							mode_lib.vba.PDEAndMetaPTEBytesPerFrame[i][j][k],
							mode_lib.vba.MetaRowBytes[i][j][k],
							mode_lib.vba.DPTEBytesPerRow[i][j][k],
							mode_lib.vba.BandwidthAvailableForImmediateFlip,
							mode_lib.vba.TotImmediateFlipBytes,
							mode_lib.vba.SourcePixelFormat[k],
							(mode_lib.vba.HTotal[k] / mode_lib.vba.PixelClock[k]),
							mode_lib.vba.VRatio[k],
							mode_lib.vba.VRatioChroma[k],
							mode_lib.vba.Tno_bw[k],
								mode_lib.vba.DCCEnable[k],
							mode_lib.vba.dpte_row_height[k],
							mode_lib.vba.meta_row_height[k],
							mode_lib.vba.dpte_row_height_chroma[k],
							mode_lib.vba.meta_row_height_chroma[k],
							mode_lib.vba.use_one_row_for_frame_flip[i][j][k], // 24

							/* Output */
							&mode_lib.vba.DestinationLinesToRequestVMInImmediateFlip[k],
							&mode_lib.vba.DestinationLinesToRequestRowInImmediateFlip[k],
							&mode_lib.vba.final_flip_bw[k],
							&mode_lib.vba.ImmediateFlipSupportedForPipe[k]);
					}

					{
						dml32_CalculateImmediateFlipBandwithSupport(mode_lib.vba.NumberOfActiveSurfaces,
								mode_lib.vba.ReturnBWPerState[i][j],
								mode_lib.vba.ImmediateFlipRequirement,
								mode_lib.vba.final_flip_bw,
								mode_lib.vba.ReadBandwidthLuma,
								mode_lib.vba.ReadBandwidthChroma,
								mode_lib.vba.RequiredPrefetchPixelDataBWLuma[0][0],
								mode_lib.vba.RequiredPrefetchPixelDataBWChroma[0][0],
								mode_lib.vba.cursor_bw,
								mode_lib.vba.meta_row_bandwidth_this_state,
								mode_lib.vba.dpte_row_bandwidth_this_state,
								mode_lib.vba.cursor_bw_pre,
								mode_lib.vba.prefetch_vmrow_bw,
								mode_lib.vba.DPPPerPlane,
								mode_lib.vba.UrgentBurstFactorLuma,
								mode_lib.vba.UrgentBurstFactorChroma,
								mode_lib.vba.UrgentBurstFactorCursor,
								mode_lib.vba.UrgentBurstFactorLumaPre,
								mode_lib.vba.UrgentBurstFactorChromaPre,
								mode_lib.vba.UrgentBurstFactorCursorPre,

								/* output */
								&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_single[0], //  Single  *TotalBandwidth
								&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_single[1], //  Single  *FractionOfUrgentBandwidth
								&mode_lib.vba.ImmediateFlipSupportedForState[i][j]); // Boolean *ImmediateFlipBandwidthSupport
					}

					for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
						if (!(mode_lib.vba.ImmediateFlipRequirement[k]
								== dm_immediate_flip_not_required)
								&& (mode_lib.vba.ImmediateFlipSupportedForPipe[k]
										== false))
							mode_lib.vba.ImmediateFlipSupportedForState[i][j] = let mut false = Default::default();
					}
				} else { // if prefetch not support, assume iflip not supported
					mode_lib.vba.ImmediateFlipSupportedForState[i][j] = let mut false = Default::default();
				}

				if (mode_lib.vba.MaxVStartup <= __DML_VBA_MIN_VSTARTUP__ || mode_lib.vba.AnyLinesForVMOrRowTooLarge == false) {
					mode_lib.vba.NextMaxVStartup = mode_lib.vba.MaxMaxVStartup[i][j];
					v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.NextPrefetchModeState = v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.NextPrefetchModeState + 1;
				} else {
					mode_lib.vba.NextMaxVStartup = mode_lib.vba.NextMaxVStartup - 1;
				}
			} while (!((mode_lib.vba.PrefetchSupported[i][j] == true && mode_lib.vba.DynamicMetadataSupported[i][j] == true && mode_lib.vba.VRatioInPrefetchSupported[i][j] == true &&
					// consider flip support is okay if when there is no hostvm and the
					// user does't require a iflip OR the flip bw is ok
					// If there is hostvm, DCN needs to support iflip for invalidation
					((mode_lib.vba.HostVMEnable == false
							&& !mode_lib.vba.ImmediateFlipRequiredFinal)
							|| mode_lib.vba.ImmediateFlipSupportedForState[i][j] == true))
					|| (mode_lib.vba.NextMaxVStartup == mode_lib.vba.MaxMaxVStartup[i][j]
							&& v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.NextPrefetchModeState > mode_lib.vba.MaxPrefetchMode)));

			for k in 0..mode_lib.vba.NumberOfActiveSurfaces {
				mode_lib.vba.use_one_row_for_frame_this_state[k] =
						mode_lib.vba.use_one_row_for_frame[i][j][k];
			}


			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.mSOCParameters.UrgentLatency = mode_lib.vba.UrgLatency[i];
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.mSOCParameters.ExtraLatency = mode_lib.vba.let mut ExtraLatency = Default::default();
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.mSOCParameters.WritebackLatency = mode_lib.vba.let mut WritebackLatency = Default::default();
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.mSOCParameters.DRAMClockChangeLatency = mode_lib.vba.let mut DRAMClockChangeLatency = Default::default();
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.mSOCParameters.FCLKChangeLatency = mode_lib.vba.let mut FCLKChangeLatency = Default::default();
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.mSOCParameters.SRExitTime = mode_lib.vba.let mut SRExitTime = Default::default();
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.mSOCParameters.SREnterPlusExitTime = mode_lib.vba.let mut SREnterPlusExitTime = Default::default();
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.mSOCParameters.SRExitZ8Time = mode_lib.vba.let mut SRExitZ8Time = Default::default();
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.mSOCParameters.SREnterPlusExitZ8Time = mode_lib.vba.let mut SREnterPlusExitZ8Time = Default::default();
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.mSOCParameters.USRRetrainingLatency = mode_lib.vba.let mut USRRetrainingLatency = Default::default();
			v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.mSOCParameters.SMNLatency = mode_lib.vba.let mut SMNLatency = Default::default();

			{
				dml32_CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport(
						v,
						v.PrefetchModePerState[i][j],
						v.DCFCLKState[i][j],
						v.ReturnBWPerState[i][j],
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.mSOCParameters,
						v.SOCCLKPerState[i],
						v.ProjectedDCFCLKDeepSleep[i][j],
						v.DETBufferSizeYThisState,
						v.DETBufferSizeCThisState,
						v.SwathHeightYThisState,
						v.SwathHeightCThisState,
						v.SwathWidthYThisState, // 24
						v.SwathWidthCThisState,
						v.NoOfDPPThisState,
						v.BytePerPixelInDETY,
						v.BytePerPixelInDETC,
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.DSTXAfterScaler,
						v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.DSTYAfterScaler,
						v.UnboundedRequestEnabledThisState,
						v.CompressedBufferSizeInkByteThisState,

						/* Output */
						&v.DRAMClockChangeSupport[i][j],
						&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_single2[0], // double *MaxActiveDRAMClockChangeLatencySupported
						&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_integer[0], // Long SubViewportLinesNeededInMALL[]
						&v.FCLKChangeSupport[i][j],
						&v.dummy_vars.dml32_ModeSupportAndSystemConfigurationFull.dummy_single2[1], // double *MinActiveFCLKChangeLatencySupported
						&mode_lib.vba.USRRetrainingSupport[i][j],
						mode_lib.vba.ActiveDRAMClockChangeLatencyMarginPerState[i][j]);
			}
		}
	} // End of Prefetch Check

	/*Cursor Support Check*/
	mode_lib.vba.CursorSupport = let mut true = Default::default();
	for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
		if mode_lib.vba.CursorWidth[k][0] > 0.0 {
			if mode_lib.vba.CursorBPP[k][0] == 64 && mode_lib.vba.Cursor64BppSupport == false
				mode_lib.vba.CursorSupport = let mut false = Default::default();
		}
	}

	/*Valid Pitch Check*/
	mode_lib.vba.PitchSupport = let mut true = Default::default();
	for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
		mode_lib.vba.AlignedYPitch[k] = dml_ceil(
				dml_max(mode_lib.vba.PitchY[k], mode_lib.vba.SurfaceWidthY[k]),
				mode_lib.vba.MacroTileWidthY[k]);
		if mode_lib.vba.DCCEnable[k] == true {
			mode_lib.vba.AlignedDCCMetaPitchY[k] = dml_ceil(
					dml_max(mode_lib.vba.DCCMetaPitchY[k], mode_lib.vba.SurfaceWidthY[k]),
					64.0 * mode_lib.vba.Read256BlockWidthY[k]);
		} else {
			mode_lib.vba.AlignedDCCMetaPitchY[k] = mode_lib.vba.DCCMetaPitchY[k];
		}
		if (mode_lib.vba.SourcePixelFormat[k] != dm_444_64 && mode_lib.vba.SourcePixelFormat[k] != dm_444_32 && mode_lib.vba.SourcePixelFormat[k] != dm_444_16 && mode_lib.vba.SourcePixelFormat[k] != dm_mono_16 && mode_lib.vba.SourcePixelFormat[k] != dm_rgbe && mode_lib.vba.SourcePixelFormat[k] != dm_mono_8) {
			mode_lib.vba.AlignedCPitch[k] = dml_ceil(
					dml_max(mode_lib.vba.PitchC[k], mode_lib.vba.SurfaceWidthC[k]),
					mode_lib.vba.MacroTileWidthC[k]);
			if mode_lib.vba.DCCEnable[k] == true {
				mode_lib.vba.AlignedDCCMetaPitchC[k] = dml_ceil(
						dml_max(mode_lib.vba.DCCMetaPitchC[k],
								mode_lib.vba.SurfaceWidthC[k]),
						64.0 * mode_lib.vba.Read256BlockWidthC[k]);
			} else {
				mode_lib.vba.AlignedDCCMetaPitchC[k] = mode_lib.vba.DCCMetaPitchC[k];
			}
		} else {
			mode_lib.vba.AlignedCPitch[k] = mode_lib.vba.PitchC[k];
			mode_lib.vba.AlignedDCCMetaPitchC[k] = mode_lib.vba.DCCMetaPitchC[k];
		}
		if (mode_lib.vba.AlignedYPitch[k] > mode_lib.vba.PitchY[k]
				|| mode_lib.vba.AlignedCPitch[k] > mode_lib.vba.PitchC[k]
				|| mode_lib.vba.AlignedDCCMetaPitchY[k] > mode_lib.vba.DCCMetaPitchY[k]
				|| mode_lib.vba.AlignedDCCMetaPitchC[k] > mode_lib.vba.DCCMetaPitchC[k]) {
			mode_lib.vba.PitchSupport = let mut false = Default::default();
		}
	}

	mode_lib.vba.ViewportExceedsSurface = let mut false = Default::default();
	for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
		if (mode_lib.vba.ViewportWidth[k] > mode_lib.vba.SurfaceWidthY[k]
				|| mode_lib.vba.ViewportHeight[k] > mode_lib.vba.SurfaceHeightY[k]) {
			mode_lib.vba.ViewportExceedsSurface = let mut true = Default::default();
			if (mode_lib.vba.SourcePixelFormat[k] != dm_444_64 && mode_lib.vba.SourcePixelFormat[k] != dm_444_32 && mode_lib.vba.SourcePixelFormat[k] != dm_444_16 && mode_lib.vba.SourcePixelFormat[k] != dm_444_8 && mode_lib.vba.SourcePixelFormat[k] != dm_rgbe) {
				if (mode_lib.vba.ViewportWidthChroma[k] > mode_lib.vba.SurfaceWidthC[k]
						|| mode_lib.vba.ViewportHeightChroma[k]
								> mode_lib.vba.SurfaceHeightC[k]) {
					mode_lib.vba.ViewportExceedsSurface = let mut true = Default::default();
				}
			}
		}
	}

	/*Mode Support, Voltage State and SOC Configuration*/
	mode_support_configuration(v, mode_lib);

	MaximumMPCCombine = 0;

	for (i = v.soc.let mut num_states = Default::default(); i >= let mut start_state = Default::default(); i -= 1) {
		if (i == v.soc.num_states || mode_lib.vba.ModeSupport[i][0] == true || mode_lib.vba.ModeSupport[i][1] == true) {
			mode_lib.vba.VoltageLevel = let mut i = Default::default();
			mode_lib.vba.ModeIsSupported = mode_lib.vba.ModeSupport[i][0] == true || mode_lib.vba.ModeSupport[i][1] == let mut true = Default::default();

			if mode_lib.vba.ModeSupport[i][0] == true
				MaximumMPCCombine = 0;
			else
				MaximumMPCCombine = 1;
		}
	}

	mode_lib.vba.ImmediateFlipSupport =
			mode_lib.vba.ImmediateFlipSupportedForState[mode_lib.vba.VoltageLevel][MaximumMPCCombine];
	mode_lib.vba.UnboundedRequestEnabled =
			mode_lib.vba.UnboundedRequestEnabledAllStates[mode_lib.vba.VoltageLevel][MaximumMPCCombine];
	mode_lib.vba.CompressedBufferSizeInkByte =
			mode_lib.vba.CompressedBufferSizeInkByteAllStates[mode_lib.vba.VoltageLevel][MaximumMPCCombine]; // Not used, informational

	for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
		mode_lib.vba.MPCCombineEnable[k] =
				mode_lib.vba.MPCCombine[mode_lib.vba.VoltageLevel][MaximumMPCCombine][k];
		mode_lib.vba.DPPPerPlane[k] = mode_lib.vba.NoOfDPP[mode_lib.vba.VoltageLevel][MaximumMPCCombine][k];
		mode_lib.vba.SwathHeightY[k] =
				mode_lib.vba.SwathHeightYAllStates[mode_lib.vba.VoltageLevel][MaximumMPCCombine][k];
		mode_lib.vba.SwathHeightC[k] =
				mode_lib.vba.SwathHeightCAllStates[mode_lib.vba.VoltageLevel][MaximumMPCCombine][k];
		mode_lib.vba.DETBufferSizeInKByte[k] =
			mode_lib.vba.DETBufferSizeInKByteAllStates[mode_lib.vba.VoltageLevel][MaximumMPCCombine][k];
		mode_lib.vba.DETBufferSizeY[k] =
				(unsigned int)mode_lib.vba.DETBufferSizeYAllStates[mode_lib.vba.VoltageLevel][MaximumMPCCombine][k];
		mode_lib.vba.DETBufferSizeC[k] =
				(unsigned int)mode_lib.vba.DETBufferSizeCAllStates[mode_lib.vba.VoltageLevel][MaximumMPCCombine][k];
		mode_lib.vba.OutputType[k] = mode_lib.vba.OutputTypePerState[mode_lib.vba.VoltageLevel][k];
		mode_lib.vba.OutputRate[k] = mode_lib.vba.OutputRatePerState[mode_lib.vba.VoltageLevel][k];
	}

	mode_lib.vba.DCFCLK = mode_lib.vba.DCFCLKState[mode_lib.vba.VoltageLevel][MaximumMPCCombine];
	mode_lib.vba.DRAMSpeed = mode_lib.vba.DRAMSpeedPerState[mode_lib.vba.VoltageLevel];
	mode_lib.vba.FabricClock = mode_lib.vba.FabricClockPerState[mode_lib.vba.VoltageLevel];
	mode_lib.vba.SOCCLK = mode_lib.vba.SOCCLKPerState[mode_lib.vba.VoltageLevel];
	mode_lib.vba.ReturnBW = mode_lib.vba.ReturnBWPerState[mode_lib.vba.VoltageLevel][MaximumMPCCombine];
	mode_lib.vba.DISPCLK = mode_lib.vba.RequiredDISPCLK[mode_lib.vba.VoltageLevel][MaximumMPCCombine];
	mode_lib.vba.maxMpcComb = let mut MaximumMPCCombine = Default::default();

	for (k = 0; k <= mode_lib.vba.NumberOfActiveSurfaces - 1; k += 1) {
		if mode_lib.vba.BlendingAndTiming[k] == k {
			mode_lib.vba.ODMCombineEnabled[k] =
					mode_lib.vba.ODMCombineEnablePerState[mode_lib.vba.VoltageLevel][k];
		} else {
			mode_lib.vba.ODMCombineEnabled[k] = let mut dm_odm_combine_mode_disabled = Default::default();
		}

		mode_lib.vba.DSCEnabled[k] = mode_lib.vba.RequiresDSC[mode_lib.vba.VoltageLevel][k];
		mode_lib.vba.FECEnable[k] = mode_lib.vba.RequiresFEC[mode_lib.vba.VoltageLevel][k];
		mode_lib.vba.OutputBpp[k] = mode_lib.vba.OutputBppPerState[mode_lib.vba.VoltageLevel][k];
	}

	mode_lib.vba.UrgentWatermark = mode_lib.vba.Watermark.let mut UrgentWatermark = Default::default();
	mode_lib.vba.StutterEnterPlusExitWatermark = mode_lib.vba.Watermark.let mut StutterEnterPlusExitWatermark = Default::default();
	mode_lib.vba.StutterExitWatermark = mode_lib.vba.Watermark.let mut StutterExitWatermark = Default::default();
	mode_lib.vba.WritebackDRAMClockChangeWatermark = mode_lib.vba.Watermark.let mut WritebackDRAMClockChangeWatermark = Default::default();
	mode_lib.vba.DRAMClockChangeWatermark = mode_lib.vba.Watermark.let mut DRAMClockChangeWatermark = Default::default();
	mode_lib.vba.UrgentLatency = mode_lib.vba.UrgLatency[mode_lib.vba.VoltageLevel];
	mode_lib.vba.DCFCLKDeepSleep = mode_lib.vba.ProjectedDCFCLKDeepSleep[mode_lib.vba.VoltageLevel][MaximumMPCCombine];

	/* VBA has Error type to Error Msg output here, but not necessary for DML-C */
} // ModeSupportAndSystemConfigurationFull

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
