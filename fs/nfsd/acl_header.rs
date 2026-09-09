/*
 *  Common NFSv4 ACL handling definitions.
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
 *     notice, this list of conditions and the following disclaimer in
 *     the documentation and/or other materials provided with the distribution.
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

// C header guard: LINUX_NFS4_ACL_H

#[repr(C)]
pub struct nfs4_acl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct svc_fh {
    _private: [u8; 0],
}

#[repr(C)]
pub struct svc_rqst {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nfsd_attrs {
    _private: [u8; 0],
}

// The definition of enum nfs_ftype4 is supplied by another header.
pub type nfs_ftype4 = i32;

extern "C" {
    pub fn nfs4_acl_bytes(entries: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn nfs4_acl_get_whotype(
        who: *mut ::std::os::raw::c_char,
        value: u32,
    ) -> ::std::os::raw::c_int;
    pub fn nfs4_acl_write_who(
        xdr: *mut xdr_stream,
        who: ::std::os::raw::c_int,
    ) -> __be32;

    pub fn nfsd4_get_nfs4_acl(
        rqstp: *mut svc_rqst,
        dentry: *mut dentry,
        acl: *mut *mut nfs4_acl,
    ) -> __be32;
    pub fn nfsd4_acl_to_attr(
        type_: nfs_ftype4,
        acl: *mut nfs4_acl,
        attr: *mut nfsd_attrs,
    ) -> __be32;
    pub fn sort_pacl_range(
        pacl: *mut posix_acl,
        start: ::std::os::raw::c_int,
        end: ::std::os::raw::c_int,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
