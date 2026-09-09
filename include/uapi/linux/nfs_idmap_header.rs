/*
 * include/uapi/linux/nfs_idmap.h
 *
 *  UID and GID to name mapping for clients.
 *
 *  Copyright (c) 2002 The Regents of the University of Michigan.
 *  All rights reserved.
 *
 *  Marius Aamodt Eriksen <marius@umich.edu>
 *
 *  Redistribution and use in source and binary forms, with or without
 *  modification, are permitted provided that the following conditions
 *  are met:
 *
 *  1. Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *  2. Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *  3. Neither the name of the University nor the names of its
 *     contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 *  THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
 *  WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 *  MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 *  DISCLAIMED. IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 *  FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 *  CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 *  SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
 *  BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 *  LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 *  NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 *  SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

// C header guard: _UAPINFS_IDMAP_H
// Dependency: <linux/types.h> supplies the C __u8 and __u32 types.

/* XXX from bits/utmp.h  */
pub const IDMAP_NAMESZ: usize = 128;

pub const IDMAP_TYPE_USER: u8 = 0;
pub const IDMAP_TYPE_GROUP: u8 = 1;

pub const IDMAP_CONV_IDTONAME: u8 = 0;
pub const IDMAP_CONV_NAMETOID: u8 = 1;

pub const IDMAP_STATUS_INVALIDMSG: u8 = 0x01;
pub const IDMAP_STATUS_AGAIN: u8 = 0x02;
pub const IDMAP_STATUS_LOOKUPFAIL: u8 = 0x04;
pub const IDMAP_STATUS_SUCCESS: u8 = 0x08;

#[repr(C)]
pub struct idmap_msg {
    pub im_type: u8,
    pub im_conv: u8,
    pub im_name: [i8; IDMAP_NAMESZ],
    pub im_id: u32,
    pub im_status: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
