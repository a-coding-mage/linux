/* This header is BSD licensed so anyone can use the definitions to implement
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
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL IBM OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF
 * USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT
 * OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE. */

// Dependency supplied by the Linux type definitions: __u8, __le16, and __le32.

#[repr(u32)]
pub enum virtio_input_config_select {
    VIRTIO_INPUT_CFG_UNSET = 0x00,
    VIRTIO_INPUT_CFG_ID_NAME = 0x01,
    VIRTIO_INPUT_CFG_ID_SERIAL = 0x02,
    VIRTIO_INPUT_CFG_ID_DEVIDS = 0x03,
    VIRTIO_INPUT_CFG_PROP_BITS = 0x10,
    VIRTIO_INPUT_CFG_EV_BITS = 0x11,
    VIRTIO_INPUT_CFG_ABS_INFO = 0x12,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_input_absinfo {
    pub min: __le32,
    pub max: __le32,
    pub fuzz: __le32,
    pub flat: __le32,
    pub res: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_input_devids {
    pub bustype: __le16,
    pub vendor: __le16,
    pub product: __le16,
    pub version: __le16,
}

#[repr(C)]
pub union virtio_input_config_u {
    pub string: [::core::ffi::c_char; 128],
    pub bitmap: [__u8; 128],
    pub abs: virtio_input_absinfo,
    pub ids: virtio_input_devids,
}

#[repr(C)]
pub struct virtio_input_config {
    pub select: __u8,
    pub subsel: __u8,
    pub size: __u8,
    pub reserved: [__u8; 5],
    pub u: virtio_input_config_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_input_event {
    pub type_: __le16,
    pub code: __le16,
    pub value: __le32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
