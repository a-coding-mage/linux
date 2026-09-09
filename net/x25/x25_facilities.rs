// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * X.25 Packet Layer release 002
 *
 * This is ALPHA test software. This code may break your machine,
 * randomly fail to work with new releases, misbehave and/or generally
 * screw up. It might even work.
 *
 * History
 * X.25 001 Split from x25_subr.c
 * mar/20/00 Daniela Squassoni Disabling/enabling of facilities negotiation.
 * apr/14/05 Shaun Pereira - Allow fast select with no restriction on response.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// struct sk_buff, struct sock, struct x25_facilities, struct x25_dte_facilities,
// struct x25_sock, struct x25_neigh, facility constants, and logging/helpers.

#[repr(C)]
pub struct sk_buff {
    pub data: *mut u8,
}

#[repr(C)]
pub struct sock;

#[repr(C)]
pub struct x25_facilities {
    pub reverse: u8,
    pub throughput: u8,
    pub pacsize_in: u8,
    pub pacsize_out: u8,
    pub winsize_in: u8,
    pub winsize_out: u8,
}

#[repr(C)]
pub struct x25_dte_facilities {
    pub calling_len: u8,
    pub called_len: u8,
    pub calling_ae: [u8; 64],
    pub called_ae: [u8; 64],
}

#[repr(C)]
pub struct x25_sock {
    pub facilities: x25_facilities,
    pub vc_facil_mask: usize,
}

#[repr(C)]
pub struct x25_neigh {
    pub extended: bool,
}

extern "C" {
    fn pskb_may_pull(skb: *mut sk_buff, len: usize) -> bool;
    fn x25_sk(sk: *mut sock) -> *mut x25_sock;
}

const X25_FAC_CLASS_MASK: u8 = 0xc0;
const X25_FAC_CLASS_A: u8 = 0x00;
const X25_FAC_CLASS_B: u8 = 0x40;
const X25_FAC_CLASS_C: u8 = 0x80;
const X25_FAC_CLASS_D: u8 = 0xc0;

// Facility values and masks are supplied by <net/x25.h> in the original source.
extern "C" {
    static X25_FAC_REVERSE: u8;
    static X25_FAC_THROUGHPUT: u8;
    static X25_MARKER: u8;
    static X25_FAC_PACKET_SIZE: u8;
    static X25_FAC_WINDOW_SIZE: u8;
    static X25_FAC_CALLING_AE: u8;
    static X25_FAC_CALLED_AE: u8;
    static X25_DTE_SERVICES: u8;
    static X25_DEFAULT_REVERSE: u8;
    static X25_MAX_DTE_FACIL_LEN: u8;
    static X25_MAX_AE_LEN: u8;
    static X25_MASK_REVERSE: usize;
    static X25_MASK_THROUGHPUT: usize;
    static X25_MASK_PACKET_SIZE: usize;
    static X25_MASK_WINDOW_SIZE: usize;
    static X25_MASK_CALLING_AE: usize;
    static X25_MASK_CALLED_AE: usize;
}

pub unsafe fn x25_parse_facilities(
    skb: *mut sk_buff,
    facilities: *mut x25_facilities,
    dte_facs: *mut x25_dte_facilities,
    vc_fac_mask: *mut usize,
) -> isize {
    *vc_fac_mask = 0;
    (*dte_facs).calling_len = 0;
    (*dte_facs).called_len = 0;
    (*dte_facs).called_ae.fill(0);
    (*dte_facs).calling_ae.fill(0);
    if !pskb_may_pull(skb, 1) { return 0; }
    let mut len = *(*skb).data as usize;
    if !pskb_may_pull(skb, 1 + len) { return -1; }
    let base = (*skb).data;
    let mut p = base.add(1);
    while len > 0 {
        match *p & X25_FAC_CLASS_MASK {
            X25_FAC_CLASS_A => {
                if len < 2 { return -1; }
                match *p {
                    x if x == X25_FAC_REVERSE => {
                        if (*p.add(1) & 0x81) == 0x81 { (*facilities).reverse = *p.add(1) & 0x81; }
                        else if (*p.add(1) & 1) == 1 { (*facilities).reverse = *p.add(1) & 1; }
                        else if (*p.add(1) & 0x80) == 0x80 { (*facilities).reverse = *p.add(1) & 0x80; }
                        else if *p.add(1) == 0 { (*facilities).reverse = X25_DEFAULT_REVERSE; }
                        else { (*facilities).throughput = *p.add(1); *vc_fac_mask |= X25_MASK_THROUGHPUT; p = p.add(2); len -= 2; continue; }
                        *vc_fac_mask |= X25_MASK_REVERSE;
                    }
                    x if x == X25_FAC_THROUGHPUT => { (*facilities).throughput = *p.add(1); *vc_fac_mask |= X25_MASK_THROUGHPUT; }
                    _ => {}
                }
                p = p.add(2); len -= 2;
            }
            X25_FAC_CLASS_B => {
                if len < 3 { return -1; }
                if *p == X25_FAC_PACKET_SIZE { (*facilities).pacsize_in=*p.add(1); (*facilities).pacsize_out=*p.add(2); *vc_fac_mask |= X25_MASK_PACKET_SIZE; }
                else if *p == X25_FAC_WINDOW_SIZE { (*facilities).winsize_in=*p.add(1); (*facilities).winsize_out=*p.add(2); *vc_fac_mask |= X25_MASK_WINDOW_SIZE; }
                p=p.add(3); len-=3;
            }
            X25_FAC_CLASS_C => { if len < 4 { return -1; } p=p.add(4); len-=4; }
            X25_FAC_CLASS_D => {
                if len < (*p.add(1) as usize)+2 { return -1; }
                if *p == X25_FAC_CALLING_AE || *p == X25_FAC_CALLED_AE {
                    if *p.add(1) > X25_MAX_DTE_FACIL_LEN || *p.add(1) <= 1 || *p.add(2) > X25_MAX_AE_LEN { return -1; }
                    let n=*p.add(1) as usize; let ae_len=*p.add(2);
                    if *p == X25_FAC_CALLING_AE { (*dte_facs).calling_len=ae_len; core::ptr::copy_nonoverlapping(p.add(3), (*dte_facs).calling_ae.as_mut_ptr(), n-1); *vc_fac_mask |= X25_MASK_CALLING_AE; }
                    else { (*dte_facs).called_len=ae_len; core::ptr::copy_nonoverlapping(p.add(3), (*dte_facs).called_ae.as_mut_ptr(), n-1); *vc_fac_mask |= X25_MASK_CALLED_AE; }
                }
                let n=*p.add(1) as usize+2; len-=n; p=p.add(n);
            }
            _ => {}
        }
    }
    p.offset_from(base) as isize
}

pub unsafe fn x25_create_facilities(buffer: *mut u8, facilities: *mut x25_facilities, dte: *mut x25_dte_facilities, mask: usize) -> isize {
    let mut p=buffer.add(1);
    if mask==0 { *buffer=0; return 1; }
    if (*facilities).reverse!=0 && mask & X25_MASK_REVERSE != 0 { *p=X25_FAC_REVERSE; p=p.add(1); *p=(*facilities).reverse; p=p.add(1); }
    if (*facilities).throughput!=0 && mask & X25_MASK_THROUGHPUT != 0 { *p=X25_FAC_THROUGHPUT; p=p.add(1); *p=(*facilities).throughput; p=p.add(1); }
    if ((*facilities).pacsize_in!=0 || (*facilities).pacsize_out!=0) && mask & X25_MASK_PACKET_SIZE != 0 { *p=X25_FAC_PACKET_SIZE; *p.add(1)=if (*facilities).pacsize_in!=0 {(*facilities).pacsize_in} else {(*facilities).pacsize_out}; *p.add(2)=if (*facilities).pacsize_out!=0 {(*facilities).pacsize_out} else {(*facilities).pacsize_in}; p=p.add(3); }
    if ((*facilities).winsize_in!=0 || (*facilities).winsize_out!=0) && mask & X25_MASK_WINDOW_SIZE != 0 { *p=X25_FAC_WINDOW_SIZE; *p.add(1)=if (*facilities).winsize_in!=0 {(*facilities).winsize_in} else {(*facilities).winsize_out}; *p.add(2)=if (*facilities).winsize_out!=0 {(*facilities).winsize_out} else {(*facilities).winsize_in}; p=p.add(3); }
    if mask & (X25_MASK_CALLING_AE|X25_MASK_CALLED_AE) != 0 { *p=X25_MARKER; *p.add(1)=X25_DTE_SERVICES; p=p.add(2); }
    if (*dte).calling_len!=0 && mask & X25_MASK_CALLING_AE != 0 { let n=((*dte).calling_len as usize+1)>>1; *p=X25_FAC_CALLING_AE; *p.add(1)=(1+n) as u8; *p.add(2)=(*dte).calling_len; core::ptr::copy_nonoverlapping((*dte).calling_ae.as_ptr(),p.add(3),n); p=p.add(3+n); }
    if (*dte).called_len!=0 && mask & X25_MASK_CALLED_AE != 0 { let n=((*dte).called_len as usize+1)>>1; *p=X25_FAC_CALLED_AE; *p.add(1)=(1+n) as u8; *p.add(2)=(*dte).called_len; core::ptr::copy_nonoverlapping((*dte).called_ae.as_ptr(),p.add(3),n); p=p.add(3+n); }
    let len=p.offset_from(buffer) as isize; *buffer=(len-1) as u8; len
}

pub unsafe fn x25_negotiate_facilities(skb:*mut sk_buff, sk:*mut sock, new:*mut x25_facilities, dte:*mut x25_dte_facilities)->isize {
    let x25=x25_sk(sk); let ours=&(*x25).facilities as *const _; let mut theirs=x25_facilities{reverse:0,throughput:0,pacsize_in:0,pacsize_out:0,winsize_in:0,winsize_out:0}; core::ptr::copy_nonoverlapping(ours,new,1); core::ptr::write_bytes(dte as *mut u8,0,core::mem::size_of::<x25_dte_facilities>());
    let len=x25_parse_facilities(skb,&mut theirs,dte,&mut (*x25).vc_facil_mask); if len<0{return len;}
    if theirs.reverse&1!=0 && (*ours).reverse&1!=0{return -1;} (*new).reverse=theirs.reverse;
    if theirs.throughput!=0 { let ti=theirs.throughput&15; let to=theirs.throughput&240; let oi=(*ours).throughput&15; let oo=(*ours).throughput&240; if oi==0||ti<oi {(*new).throughput=((*new).throughput&240)|ti;} if oo==0||to<oo {(*new).throughput=((*new).throughput&15)|to;} }
    if theirs.pacsize_in!=0&&theirs.pacsize_out!=0 {if theirs.pacsize_in<(*ours).pacsize_in{(*new).pacsize_in=theirs.pacsize_in;} if theirs.pacsize_out<(*ours).pacsize_out{(*new).pacsize_out=theirs.pacsize_out;}}
    if theirs.winsize_in!=0&&theirs.winsize_out!=0 {if theirs.winsize_in<(*ours).winsize_in{(*new).winsize_in=theirs.winsize_in;} if theirs.winsize_out<(*ours).winsize_out{(*new).winsize_out=theirs.winsize_out;}}
    len
}

pub unsafe fn x25_limit_facilities(facilities:*mut x25_facilities, nb:*mut x25_neigh) { if !(*nb).extended { if (*facilities).winsize_in>7 {(*facilities).winsize_in=7;} if (*facilities).winsize_out>7 {(*facilities).winsize_out=7;} } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
