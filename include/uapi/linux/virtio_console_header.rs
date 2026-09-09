/*
 * This header, excluding the #ifdef __KERNEL__ part, is BSD licensed so
 * anyone can use the definitions to implement compatible drivers/servers:
 *
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
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL IBM OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 * Copyright (C) Red Hat, Inc., 2009, 2010, 2011
 * Copyright (C) Amit Shah <amit.shah@redhat.com>, 2009, 2010, 2011
 */

// Dependencies supplied by the surrounding translation unit:
// linux/types.h, linux/virtio_types.h, linux/virtio_ids.h, linux/virtio_config.h

/* Feature bits */
pub const VIRTIO_CONSOLE_F_SIZE: u32 = 0; /* Does host provide console size? */
pub const VIRTIO_CONSOLE_F_MULTIPORT: u32 = 1; /* Does host provide multiple ports? */
pub const VIRTIO_CONSOLE_F_EMERG_WRITE: u32 = 2; /* Does host support emergency write? */

pub const VIRTIO_CONSOLE_BAD_ID: u32 = !0u32;

#[repr(C, packed)]
pub struct virtio_console_config {
    /* columns of the screens */
    pub cols: __virtio16,
    /* rows of the screens */
    pub rows: __virtio16,
    /* max. number of ports this device can hold */
    pub max_nr_ports: __virtio32,
    /* emergency write register */
    pub emerg_wr: __virtio32,
}

/*
 * A message that's passed between the Host and the Guest for a
 * particular port.
 */
#[repr(C)]
pub struct virtio_console_control {
    pub id: __virtio32, /* Port number */
    pub event: __virtio16, /* The kind of control event (see below) */
    pub value: __virtio16, /* Extra information for the key */
}

/* Some events for control messages */
pub const VIRTIO_CONSOLE_DEVICE_READY: u16 = 0;
pub const VIRTIO_CONSOLE_PORT_ADD: u16 = 1;
pub const VIRTIO_CONSOLE_PORT_REMOVE: u16 = 2;
pub const VIRTIO_CONSOLE_PORT_READY: u16 = 3;
pub const VIRTIO_CONSOLE_CONSOLE_PORT: u16 = 4;
pub const VIRTIO_CONSOLE_RESIZE: u16 = 5;
pub const VIRTIO_CONSOLE_PORT_OPEN: u16 = 6;
pub const VIRTIO_CONSOLE_PORT_NAME: u16 = 7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
