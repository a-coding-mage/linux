// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2025 Šerif Rami <ramiserifpersia@gmail.com>

// Dependency: us144mkii

extern "C" {
    /// Hardware capabilities for TASCAM US-144MKII PCM.
    ///
    /// Defines the supported PCM formats, rates, channels, and buffer/period sizes
    /// for the TASCAM US-144MKII audio interface.
    pub static mut tascam_pcm_hw: snd_pcm_hardware;

    /// ALSA PCM operations for playback.
    ///
    /// This structure defines the callback functions for playback stream operations.
    pub static mut tascam_playback_ops: snd_pcm_ops;

    /// ALSA PCM operations for capture.
    ///
    /// This structure defines the callback functions for capture stream operations.
    pub static mut tascam_capture_ops: snd_pcm_ops;

    /// Completion handler for playback isochronous URBs.
    ///
    /// # Arguments
    /// * `urb` - the completed URB
    ///
    /// This function runs in interrupt context. It calculates the number of bytes
    /// to send in the next set of packets based on the feedback-driven clock,
    /// copies the audio data from the ALSA ring buffer, and resubmits the URB.
    pub fn playback_urb_complete(urb: *mut urb);

    /// Completion handler for feedback isochronous URBs.
    ///
    /// # Arguments
    /// * `urb` - the completed URB
    ///
    /// This is the master clock for the driver. It runs in interrupt context.
    /// It reads the feedback value from the device, which indicates how many
    /// samples the device has consumed. This information is used to adjust the
    /// playback rate and to advance the capture stream pointer, keeping both
    /// streams in sync. It then calls snd_pcm_period_elapsed if necessary and
    /// resubmits itself.
    pub fn feedback_urb_complete(urb: *mut urb);

    /// Completion handler for capture bulk URBs.
    ///
    /// # Arguments
    /// * `urb` - the completed URB
    ///
    /// This function runs in interrupt context. It copies the received raw data
    /// into an intermediate ring buffer and then schedules the workqueue to process
    /// it. It then resubmits the URB to receive more data.
    pub fn capture_urb_complete(urb: *mut urb);

    /// Work handler to stop PCM streams.
    ///
    /// # Arguments
    /// * `work` - Pointer to the work_struct.
    ///
    /// This function is scheduled to stop PCM streams (playback and capture)
    /// from a workqueue context, avoiding blocking operations in interrupt context.
    pub fn tascam_stop_pcm_work_handler(work: *mut work_struct);

    /// Initializes the ALSA PCM device.
    ///
    /// # Arguments
    /// * `pcm` - Pointer to the ALSA PCM device to initialize.
    ///
    /// This function sets up the PCM operations, adds ALSA controls for routing
    /// and sample rate, and preallocates pages for the PCM buffer.
    ///
    /// # Returns
    /// 0 on success, or a negative error code on failure.
    pub fn tascam_init_pcm(pcm: *mut snd_pcm) -> i32;

    /// Set sample rate via USB control msgs
    ///
    /// # Arguments
    /// * `tascam` - the tascam_card instance
    /// * `rate` - the target sample rate (e.g., 44100, 96000)
    ///
    /// This function sends a sequence of vendor-specific and UAC control messages
    /// to configure the device hardware for the specified sample rate.
    ///
    /// # Returns
    /// 0 on success, or a negative error code on failure.
    pub fn us144mkii_configure_device_for_rate(
        tascam: *mut tascam_card,
        rate: i32,
    ) -> i32;

    /// Apply playback routing matrix
    ///
    /// # Arguments
    /// * `tascam` - The driver instance.
    /// * `src_buffer` - Buffer containing 4 channels of S24_3LE audio from ALSA.
    /// * `dst_buffer` - Buffer to be filled for the USB device.
    /// * `frames` - Number of frames to process.
    pub fn process_playback_routing_us144mkii(
        tascam: *mut tascam_card,
        src_buffer: *const u8,
        dst_buffer: *mut u8,
        frames: usize,
    );

    /// Apply capture routing matrix
    ///
    /// # Arguments
    /// * `tascam` - The driver instance.
    /// * `decoded_block` - Buffer containing 4 channels of S32LE decoded audio.
    /// * `routed_block` - Buffer to be filled for ALSA.
    pub fn process_capture_routing_us144mkii(
        tascam: *mut tascam_card,
        decoded_block: *const i32,
        routed_block: *mut i32,
    );

    /// Configures hardware parameters for PCM streams.
    ///
    /// # Arguments
    /// * `substream` - The ALSA PCM substream.
    /// * `params` - The hardware parameters to apply.
    ///
    /// This function allocates pages for the PCM buffer and, for playback streams,
    /// selects the appropriate feedback patterns based on the requested sample rate.
    /// It also configures the device hardware for the selected sample rate if it
    /// has changed.
    ///
    /// # Returns
    /// 0 on success, or a negative error code on failure.
    pub fn tascam_pcm_hw_params(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
    ) -> i32;

    /// Frees hardware parameters for PCM streams.
    ///
    /// # Arguments
    /// * `substream` - The ALSA PCM substream.
    ///
    /// This function is a stub for freeing hardware-related resources.
    ///
    /// # Returns
    /// 0 on success.
    pub fn tascam_pcm_hw_free(substream: *mut snd_pcm_substream) -> i32;

    /// Triggers the start or stop of PCM streams.
    ///
    /// # Arguments
    /// * `substream` - The ALSA PCM substream.
    /// * `cmd` - The trigger command (e.g., SNDRV_PCM_TRIGGER_START).
    ///
    /// This function handles starting and stopping of playback and capture streams
    /// by submitting or killing the associated URBs.
    ///
    /// # Returns
    /// 0 on success, or a negative error code on failure.
    pub fn tascam_pcm_trigger(substream: *mut snd_pcm_substream, cmd: i32) -> i32;

    /// Deferred work for processing capture data.
    ///
    /// # Arguments
    /// * `work` - the work_struct instance
    ///
    /// This function runs in a kernel thread context, not an IRQ context. It reads
    /// raw data from the capture ring buffer, decodes it, applies routing, and
    /// copies the final audio data into the ALSA capture ring buffer. This offloads
    /// the CPU-intensive decoding from the time-sensitive URB completion handlers.
    pub fn tascam_capture_work_handler(work: *mut work_struct);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
