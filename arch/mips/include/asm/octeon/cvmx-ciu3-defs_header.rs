/* Translated from cvmx-ciu3-defs.h. */

macro_rules! CVMX_CIU3_FUSE { () => { CVMX_ADD_IO_SEG(0x00010100000001A0u64) }; }
macro_rules! CVMX_CIU3_BIST { () => { CVMX_ADD_IO_SEG(0x00010100000001C0u64) }; }
macro_rules! CVMX_CIU3_CONST { () => { CVMX_ADD_IO_SEG(0x0001010000000220u64) }; }
macro_rules! CVMX_CIU3_CTL { () => { CVMX_ADD_IO_SEG(0x00010100000000E0u64) }; }
macro_rules! CVMX_CIU3_DESTX_IO_INT { ($offset:expr) => { CVMX_ADD_IO_SEG(0x0001010000210000u64) + (($offset) & 7) * 8 }; }
macro_rules! CVMX_CIU3_DESTX_PP_INT { ($offset:expr) => { CVMX_ADD_IO_SEG(0x0001010000200000u64) + (($offset) & 255) * 8 }; }
macro_rules! CVMX_CIU3_GSTOP { () => { CVMX_ADD_IO_SEG(0x0001010000000140u64) }; }
macro_rules! CVMX_CIU3_IDTX_CTL { ($offset:expr) => { CVMX_ADD_IO_SEG(0x0001010000110000u64) + (($offset) & 255) * 8 }; }
macro_rules! CVMX_CIU3_IDTX_IO { ($offset:expr) => { CVMX_ADD_IO_SEG(0x0001010000130000u64) + (($offset) & 255) * 8 }; }
macro_rules! CVMX_CIU3_IDTX_PPX { ($offset:expr, $block_id:expr) => { CVMX_ADD_IO_SEG(0x0001010000120000u64) + (($block_id) & 255) * 0x20u64 }; }
macro_rules! CVMX_CIU3_INTR_RAM_ECC_CTL { () => { CVMX_ADD_IO_SEG(0x0001010000000260u64) }; }
macro_rules! CVMX_CIU3_INTR_RAM_ECC_ST { () => { CVMX_ADD_IO_SEG(0x0001010000000280u64) }; }
macro_rules! CVMX_CIU3_INTR_READY { () => { CVMX_ADD_IO_SEG(0x00010100000002A0u64) }; }
macro_rules! CVMX_CIU3_INTR_SLOWDOWN { () => { CVMX_ADD_IO_SEG(0x0001010000000240u64) }; }
macro_rules! CVMX_CIU3_ISCX_CTL { ($offset:expr) => { CVMX_ADD_IO_SEG(0x0001010080000000u64) + (($offset) & 1048575) * 8 }; }
macro_rules! CVMX_CIU3_ISCX_W1C { ($offset:expr) => { CVMX_ADD_IO_SEG(0x0001010090000000u64) + (($offset) & 1048575) * 8 }; }
macro_rules! CVMX_CIU3_ISCX_W1S { ($offset:expr) => { CVMX_ADD_IO_SEG(0x00010100A0000000u64) + (($offset) & 1048575) * 8 }; }
macro_rules! CVMX_CIU3_NMI { () => { CVMX_ADD_IO_SEG(0x0001010000000160u64) }; }
macro_rules! CVMX_CIU3_SISCX { ($offset:expr) => { CVMX_ADD_IO_SEG(0x0001010000220000u64) + (($offset) & 255) * 8 }; }
macro_rules! CVMX_CIU3_TIMX { ($offset:expr) => { CVMX_ADD_IO_SEG(0x0001010000010000u64) + (($offset) & 15) * 8 }; }

/* C bit-fields are represented by their containing 64-bit word.  The listed
 * ranges preserve the source layout and are intentionally left as raw words. */
macro_rules! cvmx_reg_union { ($u:ident, $s:ident) => {
    #[repr(C)] pub union $u { pub u64: u64, pub s: $s }
    #[repr(C)] #[derive(Copy, Clone)] pub struct $s { pub bits: u64 }
}; }

cvmx_reg_union!(cvmx_ciu3_bist, cvmx_ciu3_bist_s); // bist:9, reserved_9_63:55
cvmx_reg_union!(cvmx_ciu3_const, cvmx_ciu3_const_s); // idt:16, dests_pp:16, pintsn:16, dests_io:16
cvmx_reg_union!(cvmx_ciu3_ctl, cvmx_ciu3_ctl_s); // cclk_dis:1, seq_dis:1, iscmem_le:1, mcd_sel:2, reserved:59
cvmx_reg_union!(cvmx_ciu3_destx_io_int, cvmx_ciu3_destx_io_int_s); // intr:1, newint:1, intidt:8, reserved:22, intsn:20, reserved:12
cvmx_reg_union!(cvmx_ciu3_destx_pp_int, cvmx_ciu3_destx_pp_int_s);
cvmx_reg_union!(cvmx_ciu3_gstop, cvmx_ciu3_gstop_s); // gstop:1, reserved:63
cvmx_reg_union!(cvmx_ciu3_idtx_ctl, cvmx_ciu3_idtx_ctl_s); // ip_num:2, newint:1, intr:1, reserved:28, intsn:20, reserved:12
cvmx_reg_union!(cvmx_ciu3_idtx_io, cvmx_ciu3_idtx_io_s); // io:5, reserved:59
cvmx_reg_union!(cvmx_ciu3_idtx_ppx, cvmx_ciu3_idtx_ppx_s); // pp:48, reserved:16
cvmx_reg_union!(cvmx_ciu3_intr_ram_ecc_ctl, cvmx_ciu3_intr_ram_ecc_ctl_s); // ecc_ena:1, flip_synd:2, reserved:61
cvmx_reg_union!(cvmx_ciu3_intr_ram_ecc_st, cvmx_ciu3_intr_ram_ecc_st_s); // isc_sbe:1, isc_dbe:1, idt_sbe:1, idt_dbe:1, sisc_sbe:1, sisc_dbe:1, reserved:26, addr:20, reserved:12
cvmx_reg_union!(cvmx_ciu3_intr_ready, cvmx_ciu3_intr_ready_s); // ready:1, reserved:31, index:14, reserved:18
cvmx_reg_union!(cvmx_ciu3_intr_slowdown, cvmx_ciu3_intr_slowdown_s); // ctl:3, reserved:61
cvmx_reg_union!(cvmx_ciu3_iscx_ctl, cvmx_ciu3_iscx_ctl_s); // raw:1, en:1, reserved:13, imp:1, idt:8, reserved:40
cvmx_reg_union!(cvmx_ciu3_iscx_w1c, cvmx_ciu3_iscx_w1c_s); // raw:1, en:1, reserved:62
cvmx_reg_union!(cvmx_ciu3_iscx_w1s, cvmx_ciu3_iscx_w1s_s); // raw:1, en:1, reserved:62
cvmx_reg_union!(cvmx_ciu3_nmi, cvmx_ciu3_nmi_s); // nmi:48, reserved:16
cvmx_reg_union!(cvmx_ciu3_siscx, cvmx_ciu3_siscx_s); // en:64
cvmx_reg_union!(cvmx_ciu3_timx, cvmx_ciu3_timx_s); // len:36, one_shot:1, reserved:27

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
