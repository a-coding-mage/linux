/*
 * Virtio IDs
 *
 * This header is BSD licensed so anyone can use the definitions to implement
 * compatible drivers/servers.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of IBM nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL IBM OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE. */

pub const VIRTIO_ID_NET: u32 = 1; /* virtio net */
pub const VIRTIO_ID_BLOCK: u32 = 2; /* virtio block */
pub const VIRTIO_ID_CONSOLE: u32 = 3; /* virtio console */
pub const VIRTIO_ID_RNG: u32 = 4; /* virtio rng */
pub const VIRTIO_ID_BALLOON: u32 = 5; /* virtio balloon */
pub const VIRTIO_ID_IOMEM: u32 = 6; /* virtio ioMemory */
pub const VIRTIO_ID_RPMSG: u32 = 7; /* virtio remote processor messaging */
pub const VIRTIO_ID_SCSI: u32 = 8; /* virtio scsi */
pub const VIRTIO_ID_9P: u32 = 9; /* 9p virtio console */
pub const VIRTIO_ID_MAC80211_WLAN: u32 = 10; /* virtio WLAN MAC */
pub const VIRTIO_ID_RPROC_SERIAL: u32 = 11; /* virtio remoteproc serial link */
pub const VIRTIO_ID_CAIF: u32 = 12; /* Virtio caif */
pub const VIRTIO_ID_MEMORY_BALLOON: u32 = 13; /* virtio memory balloon */
pub const VIRTIO_ID_GPU: u32 = 16; /* virtio GPU */
pub const VIRTIO_ID_CLOCK: u32 = 17; /* virtio clock/timer */
pub const VIRTIO_ID_INPUT: u32 = 18; /* virtio input */
pub const VIRTIO_ID_VSOCK: u32 = 19; /* virtio vsock transport */
pub const VIRTIO_ID_CRYPTO: u32 = 20; /* virtio crypto */
pub const VIRTIO_ID_SIGNAL_DIST: u32 = 21; /* virtio signal distribution device */
pub const VIRTIO_ID_PSTORE: u32 = 22; /* virtio pstore device */
pub const VIRTIO_ID_IOMMU: u32 = 23; /* virtio IOMMU */
pub const VIRTIO_ID_MEM: u32 = 24; /* virtio mem */
pub const VIRTIO_ID_SOUND: u32 = 25; /* virtio sound */
pub const VIRTIO_ID_FS: u32 = 26; /* virtio filesystem */
pub const VIRTIO_ID_PMEM: u32 = 27; /* virtio pmem */
pub const VIRTIO_ID_RPMB: u32 = 28; /* virtio rpmb */
pub const VIRTIO_ID_MAC80211_HWSIM: u32 = 29; /* virtio mac80211-hwsim */
pub const VIRTIO_ID_VIDEO_ENCODER: u32 = 30; /* virtio video encoder */
pub const VIRTIO_ID_VIDEO_DECODER: u32 = 31; /* virtio video decoder */
pub const VIRTIO_ID_SCMI: u32 = 32; /* virtio SCMI */
pub const VIRTIO_ID_NITRO_SEC_MOD: u32 = 33; /* virtio nitro secure module*/
pub const VIRTIO_ID_I2C_ADAPTER: u32 = 34; /* virtio i2c adapter */
pub const VIRTIO_ID_WATCHDOG: u32 = 35; /* virtio watchdog */
pub const VIRTIO_ID_CAN: u32 = 36; /* virtio can */
pub const VIRTIO_ID_DMABUF: u32 = 37; /* virtio dmabuf */
pub const VIRTIO_ID_PARAM_SERV: u32 = 38; /* virtio parameter server */
pub const VIRTIO_ID_AUDIO_POLICY: u32 = 39; /* virtio audio policy */
pub const VIRTIO_ID_BT: u32 = 40; /* virtio bluetooth */
pub const VIRTIO_ID_GPIO: u32 = 41; /* virtio gpio */
pub const VIRTIO_ID_SPI: u32 = 45; /* virtio spi */
pub const VIRTIO_ID_MEDIA: u32 = 48; /* virtio media */

/*
 * Virtio Transitional IDs
 */

pub const VIRTIO_TRANS_ID_NET: u32 = 0x1000; /* transitional virtio net */
pub const VIRTIO_TRANS_ID_BLOCK: u32 = 0x1001; /* transitional virtio block */
pub const VIRTIO_TRANS_ID_BALLOON: u32 = 0x1002; /* transitional virtio balloon */
pub const VIRTIO_TRANS_ID_CONSOLE: u32 = 0x1003; /* transitional virtio console */
pub const VIRTIO_TRANS_ID_SCSI: u32 = 0x1004; /* transitional virtio SCSI */
pub const VIRTIO_TRANS_ID_RNG: u32 = 0x1005; /* transitional virtio rng */
pub const VIRTIO_TRANS_ID_9P: u32 = 0x1009; /* transitional virtio 9p console */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
