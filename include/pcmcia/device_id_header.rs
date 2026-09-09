/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * device_id.h -- PCMCIA driver matching helpers
 *
 * Rust translation of the kernel-only C header.  The referenced match
 * constants and containing device-id type are supplied by other bindings.
 */

/* The original definitions are active only when __KERNEL__ is defined. */

macro_rules! PCMCIA_DEVICE_MANF_CARD {
    ($manf:expr, $card:expr) => {{ match_flags: PCMCIA_DEV_ID_MATCH_MANF_ID | PCMCIA_DEV_ID_MATCH_CARD_ID, manf_id: $manf, card_id: $card }};
}
macro_rules! PCMCIA_DEVICE_FUNC_ID { ($func:expr) => {{ match_flags: PCMCIA_DEV_ID_MATCH_FUNC_ID, func_id: $func }}; }

macro_rules! pcpcmcia_prod {
    ($flags:expr, [$($p:expr),*], [$($h:expr),*]) => {{ match_flags: $flags, prod_id: [$($p),*], prod_id_hash: [$($h),*] }};
}
macro_rules! PCMCIA_DEVICE_PROD_ID1 { ($v:expr, $h:expr) => { pcpcmcia_prod!(PCMCIA_DEV_ID_MATCH_PROD_ID1, [$v, ::core::ptr::null_mut(), ::core::ptr::null_mut(), ::core::ptr::null_mut()], [$h, 0, 0, 0]) }; }
macro_rules! PCMCIA_DEVICE_PROD_ID2 { ($v:expr, $h:expr) => { pcpcmcia_prod!(PCMCIA_DEV_ID_MATCH_PROD_ID2, [::core::ptr::null_mut(), $v, ::core::ptr::null_mut(), ::core::ptr::null_mut()], [0, $h, 0, 0]) }; }
macro_rules! PCMCIA_DEVICE_PROD_ID3 { ($v:expr, $h:expr) => { pcpcmcia_prod!(PCMCIA_DEV_ID_MATCH_PROD_ID3, [::core::ptr::null_mut(), ::core::ptr::null_mut(), $v, ::core::ptr::null_mut()], [0, 0, $h, 0]) }; }
macro_rules! PCMCIA_DEVICE_PROD_ID12 { ($a:expr,$b:expr,$ha:expr,$hb:expr) => { pcpcmcia_prod!(PCMCIA_DEV_ID_MATCH_PROD_ID1|PCMCIA_DEV_ID_MATCH_PROD_ID2, [$a,$b,::core::ptr::null_mut(),::core::ptr::null_mut()], [$ha,$hb,0,0]) }; }
macro_rules! PCMCIA_DEVICE_PROD_ID13 { ($a:expr,$b:expr,$ha:expr,$hb:expr) => { pcpcmcia_prod!(PCMCIA_DEV_ID_MATCH_PROD_ID1|PCMCIA_DEV_ID_MATCH_PROD_ID3, [$a,::core::ptr::null_mut(),$b,::core::ptr::null_mut()], [$ha,0,$hb,0]) }; }
macro_rules! PCMCIA_DEVICE_PROD_ID14 { ($a:expr,$b:expr,$ha:expr,$hb:expr) => { pcpcmcia_prod!(PCMCIA_DEV_ID_MATCH_PROD_ID1|PCMCIA_DEV_ID_MATCH_PROD_ID4, [$a,::core::ptr::null_mut(),::core::ptr::null_mut(),$b], [$ha,0,0,$hb]) }; }
macro_rules! PCMCIA_DEVICE_PROD_ID123 { ($a:expr,$b:expr,$c:expr,$ha:expr,$hb:expr,$hc:expr) => { pcpcmcia_prod!(PCMCIA_DEV_ID_MATCH_PROD_ID1|PCMCIA_DEV_ID_MATCH_PROD_ID2|PCMCIA_DEV_ID_MATCH_PROD_ID3, [$a,$b,$c,::core::ptr::null_mut()], [$ha,$hb,$hc,0]) }; }
macro_rules! PCMCIA_DEVICE_PROD_ID124 { ($a:expr,$b:expr,$c:expr,$ha:expr,$hb:expr,$hc:expr) => { pcpcmcia_prod!(PCMCIA_DEV_ID_MATCH_PROD_ID1|PCMCIA_DEV_ID_MATCH_PROD_ID2|PCMCIA_DEV_ID_MATCH_PROD_ID4, [$a,$b,::core::ptr::null_mut(),$c], [$ha,$hb,0,$hc]) }; }
macro_rules! PCMCIA_DEVICE_PROD_ID134 { ($a:expr,$b:expr,$c:expr,$ha:expr,$hb:expr,$hc:expr) => { pcpcmcia_prod!(PCMCIA_DEV_ID_MATCH_PROD_ID1|PCMCIA_DEV_ID_MATCH_PROD_ID3|PCMCIA_DEV_ID_MATCH_PROD_ID4, [$a,::core::ptr::null_mut(),$b,$c], [$ha,0,$hb,$hc]) }; }
macro_rules! PCMCIA_DEVICE_PROD_ID1234 { ($a:expr,$b:expr,$c:expr,$d:expr,$ha:expr,$hb:expr,$hc:expr,$hd:expr) => { pcpcmcia_prod!(PCMCIA_DEV_ID_MATCH_PROD_ID1|PCMCIA_DEV_ID_MATCH_PROD_ID2|PCMCIA_DEV_ID_MATCH_PROD_ID3|PCMCIA_DEV_ID_MATCH_PROD_ID4, [$a,$b,$c,$d], [$ha,$hb,$hc,$hd]) }; }

macro_rules! PCMCIA_DEVICE_MANF_CARD_PROD_ID1 { ($m:expr,$c:expr,$v:expr,$h:expr) => {{ match_flags: PCMCIA_DEV_ID_MATCH_MANF_ID|PCMCIA_DEV_ID_MATCH_CARD_ID|PCMCIA_DEV_ID_MATCH_PROD_ID1, manf_id:$m, card_id:$c, prod_id:[$v,::core::ptr::null_mut(),::core::ptr::null_mut(),::core::ptr::null_mut()], prod_id_hash:[$h,0,0,0] }}; }
macro_rules! PCMCIA_DEVICE_MANF_CARD_PROD_ID3 { ($m:expr,$c:expr,$v:expr,$h:expr) => {{ match_flags: PCMCIA_DEV_ID_MATCH_MANF_ID|PCMCIA_DEV_ID_MATCH_CARD_ID|PCMCIA_DEV_ID_MATCH_PROD_ID3, manf_id:$m, card_id:$c, prod_id:[::core::ptr::null_mut(),::core::ptr::null_mut(),$v,::core::ptr::null_mut()], prod_id_hash:[0,0,$h,0] }}; }

macro_rules! pcpcmcia_mfc { ($flags:expr,$m:expr,$c:expr,$v:expr,$h:expr) => {{ match_flags:$flags, manf_id:$m, card_id:$c, function:$v, prod_id:$h }}; }
macro_rules! PCMCIA_MFC_DEVICE_MANF_CARD { ($f:expr,$m:expr,$c:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_MANF_ID|PCMCIA_DEV_ID_MATCH_CARD_ID|PCMCIA_DEV_ID_MATCH_FUNCTION, manf_id:$m, card_id:$c, function:$f }}; }
macro_rules! PCMCIA_MFC_DEVICE_PROD_ID1 { ($f:expr,$v:expr,$h:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_PROD_ID1|PCMCIA_DEV_ID_MATCH_FUNCTION, prod_id:[$v,::core::ptr::null_mut(),::core::ptr::null_mut(),::core::ptr::null_mut()], prod_id_hash:[$h,0,0,0], function:$f }}; }
macro_rules! PCMCIA_MFC_DEVICE_PROD_ID2 { ($f:expr,$v:expr,$h:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_PROD_ID2|PCMCIA_DEV_ID_MATCH_FUNCTION, prod_id:[::core::ptr::null_mut(),$v,::core::ptr::null_mut(),::core::ptr::null_mut()], prod_id_hash:[0,$h,0,0], function:$f }}; }
macro_rules! PCMCIA_MFC_DEVICE_PROD_ID12 { ($f:expr,$a:expr,$b:expr,$ha:expr,$hb:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_PROD_ID1|PCMCIA_DEV_ID_MATCH_PROD_ID2|PCMCIA_DEV_ID_MATCH_FUNCTION, prod_id:[$a,$b,::core::ptr::null_mut(),::core::ptr::null_mut()], prod_id_hash:[$ha,$hb,0,0], function:$f }}; }
macro_rules! PCMCIA_MFC_DEVICE_PROD_ID13 { ($f:expr,$a:expr,$b:expr,$ha:expr,$hb:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_PROD_ID1|PCMCIA_DEV_ID_MATCH_PROD_ID3|PCMCIA_DEV_ID_MATCH_FUNCTION, prod_id:[$a,::core::ptr::null_mut(),$b,::core::ptr::null_mut()], prod_id_hash:[$ha,0,$hb,0], function:$f }}; }
macro_rules! PCMCIA_MFC_DEVICE_PROD_ID123 { ($f:expr,$a:expr,$b:expr,$c:expr,$ha:expr,$hb:expr,$hc:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_PROD_ID1|PCMCIA_DEV_ID_MATCH_PROD_ID2|PCMCIA_DEV_ID_MATCH_PROD_ID3|PCMCIA_DEV_ID_MATCH_FUNCTION, prod_id:[$a,$b,$c,::core::ptr::null_mut()], prod_id_hash:[$ha,$hb,$hc,0], function:$f }}; }

/* Pseudo multi-function devices use device_no in place of function. */
macro_rules! PCMCIA_PFC_DEVICE_MANF_CARD { ($n:expr,$m:expr,$c:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_MANF_ID|PCMCIA_DEV_ID_MATCH_CARD_ID|PCMCIA_DEV_ID_MATCH_DEVICE_NO, manf_id:$m, card_id:$c, device_no:$n }}; }
macro_rules! PCMCIA_PFC_DEVICE_PROD_ID1 { ($n:expr,$v:expr,$h:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_PROD_ID1|PCMCIA_DEV_ID_MATCH_DEVICE_NO, prod_id:[$v,::core::ptr::null_mut(),::core::ptr::null_mut(),::core::ptr::null_mut()], prod_id_hash:[$h,0,0,0], device_no:$n }}; }
macro_rules! PCMCIA_PFC_DEVICE_PROD_ID2 { ($n:expr,$v:expr,$h:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_PROD_ID2|PCMCIA_DEV_ID_MATCH_DEVICE_NO, prod_id:[::core::ptr::null_mut(),$v,::core::ptr::null_mut(),::core::ptr::null_mut()], prod_id_hash:[0,$h,0,0], device_no:$n }}; }
macro_rules! PCMCIA_PFC_DEVICE_PROD_ID12 { ($n:expr,$a:expr,$b:expr,$ha:expr,$hb:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_PROD_ID1|PCMCIA_DEV_ID_MATCH_PROD_ID2|PCMCIA_DEV_ID_MATCH_DEVICE_NO, prod_id:[$a,$b,::core::ptr::null_mut(),::core::ptr::null_mut()], prod_id_hash:[$ha,$hb,0,0], device_no:$n }}; }
macro_rules! PCMCIA_PFC_DEVICE_PROD_ID13 { ($n:expr,$a:expr,$b:expr,$ha:expr,$hb:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_PROD_ID1|PCMCIA_DEV_ID_MATCH_PROD_ID3|PCMCIA_DEV_ID_MATCH_DEVICE_NO, prod_id:[$a,::core::ptr::null_mut(),$b,::core::ptr::null_mut()], prod_id_hash:[$ha,0,$hb,0], device_no:$n }}; }
macro_rules! PCMCIA_PFC_DEVICE_PROD_ID123 { ($n:expr,$a:expr,$b:expr,$c:expr,$ha:expr,$hb:expr,$hc:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_PROD_ID1|PCMCIA_DEV_ID_MATCH_PROD_ID2|PCMCIA_DEV_ID_MATCH_PROD_ID3|PCMCIA_DEV_ID_MATCH_DEVICE_NO, prod_id:[$a,$b,$c,::core::ptr::null_mut()], prod_id_hash:[$ha,$hb,$hc,0], device_no:$n }}; }

macro_rules! PCMCIA_DEVICE_CIS_MANF_CARD { ($m:expr,$c:expr,$cis:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_FAKE_CIS|PCMCIA_DEV_ID_MATCH_MANF_ID|PCMCIA_DEV_ID_MATCH_CARD_ID, manf_id:$m, card_id:$c, cisfile:$cis }}; }
macro_rules! PCMCIA_DEVICE_CIS_PROD_ID12 { ($a:expr,$b:expr,$ha:expr,$hb:expr,$cis:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_FAKE_CIS|PCMCIA_DEV_ID_MATCH_PROD_ID1|PCMCIA_DEV_ID_MATCH_PROD_ID2, prod_id:[$a,$b,::core::ptr::null_mut(),::core::ptr::null_mut()], prod_id_hash:[$ha,$hb,0,0], cisfile:$cis }}; }
macro_rules! PCMCIA_DEVICE_CIS_PROD_ID123 { ($a:expr,$b:expr,$c:expr,$ha:expr,$hb:expr,$hc:expr,$cis:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_FAKE_CIS|PCMCIA_DEV_ID_MATCH_PROD_ID1|PCMCIA_DEV_ID_MATCH_PROD_ID2|PCMCIA_DEV_ID_MATCH_PROD_ID3, prod_id:[$a,$b,$c,::core::ptr::null_mut()], prod_id_hash:[$ha,$hb,$hc,0], cisfile:$cis }}; }
macro_rules! PCMCIA_DEVICE_CIS_PROD_ID2 { ($v:expr,$h:expr,$cis:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_FAKE_CIS|PCMCIA_DEV_ID_MATCH_PROD_ID2, prod_id:[::core::ptr::null_mut(),$v,::core::ptr::null_mut(),::core::ptr::null_mut()], prod_id_hash:[0,$h,0,0], cisfile:$cis }}; }
macro_rules! PCMCIA_PFC_DEVICE_CIS_PROD_ID12 { ($n:expr,$a:expr,$b:expr,$ha:expr,$hb:expr,$cis:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_FAKE_CIS|PCMCIA_DEV_ID_MATCH_PROD_ID1|PCMCIA_DEV_ID_MATCH_PROD_ID2|PCMCIA_DEV_ID_MATCH_DEVICE_NO, prod_id:[$a,$b,::core::ptr::null_mut(),::core::ptr::null_mut()], prod_id_hash:[$ha,$hb,0,0], device_no:$n, cisfile:$cis }}; }
macro_rules! PCMCIA_MFC_DEVICE_CIS_MANF_CARD { ($f:expr,$m:expr,$c:expr,$cis:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_FAKE_CIS|PCMCIA_DEV_ID_MATCH_MANF_ID|PCMCIA_DEV_ID_MATCH_CARD_ID|PCMCIA_DEV_ID_MATCH_FUNCTION, manf_id:$m, card_id:$c, function:$f, cisfile:$cis }}; }
macro_rules! PCMCIA_MFC_DEVICE_CIS_PROD_ID12 { ($f:expr,$a:expr,$b:expr,$ha:expr,$hb:expr,$cis:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_FAKE_CIS|PCMCIA_DEV_ID_MATCH_PROD_ID1|PCMCIA_DEV_ID_MATCH_PROD_ID2|PCMCIA_DEV_ID_MATCH_FUNCTION, prod_id:[$a,$b,::core::ptr::null_mut(),::core::ptr::null_mut()], prod_id_hash:[$ha,$hb,0,0], function:$f, cisfile:$cis }}; }
macro_rules! PCMCIA_MFC_DEVICE_CIS_PROD_ID4 { ($f:expr,$v:expr,$h:expr,$cis:expr) => {{ match_flags:PCMCIA_DEV_ID_MATCH_FAKE_CIS|PCMCIA_DEV_ID_MATCH_PROD_ID4|PCMCIA_DEV_ID_MATCH_FUNCTION, prod_id:[::core::ptr::null_mut(),::core::ptr::null_mut(),::core::ptr::null_mut(),$v], prod_id_hash:[0,0,0,$h], function:$f, cisfile:$cis }}; }

macro_rules! PCMCIA_DEVICE_NULL { () => {{ match_flags: 0 }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
