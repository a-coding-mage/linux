/*
 *  Mapping of UID to name and vice versa.
 *
 *  Copyright (c) 2002, 2003 The Regents of the University of
 *  Michigan.  All rights reserved.
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

// Dependencies supplied by the surrounding kernel translation:
// linux/in.h, linux/sunrpc/svc.h, and linux/nfs_idmap.h.

#[cfg(CONFIG_NFSD_V4)]
unsafe extern "C" {
    pub fn nfsd_idmap_init(net: *mut struct net) -> i32;
    pub fn nfsd_idmap_shutdown(net: *mut struct net);
}

#[cfg(not(CONFIG_NFSD_V4))]
pub unsafe fn nfsd_idmap_init(_net: *mut struct net) -> i32 {
    0
}

#[cfg(not(CONFIG_NFSD_V4))]
pub unsafe fn nfsd_idmap_shutdown(_net: *mut struct net) {}

unsafe extern "C" {
    pub fn nfsd_map_name_to_uid(
        rqst: *mut struct svc_rqst,
        name: *const core::ffi::c_char,
        len: usize,
        uid: *mut kuid_t,
    ) -> __be32;
    pub fn nfsd_map_name_to_gid(
        rqst: *mut struct svc_rqst,
        name: *const core::ffi::c_char,
        len: usize,
        gid: *mut kgid_t,
    ) -> __be32;
    pub fn nfsd4_encode_user(
        xdr: *mut struct xdr_stream,
        rqst: *mut struct svc_rqst,
        uid: kuid_t,
    ) -> __be32;
    pub fn nfsd4_encode_group(
        xdr: *mut struct xdr_stream,
        rqst: *mut struct svc_rqst,
        gid: kgid_t,
    ) -> __be32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
