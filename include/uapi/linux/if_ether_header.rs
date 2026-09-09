/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* Global definitions for the Ethernet IEEE 802.3 interface. */

/* IEEE 802.3 Ethernet magic constants. */
pub const ETH_ALEN: usize = 6;
pub const ETH_TLEN: usize = 2;
pub const ETH_HLEN: usize = 14;
pub const ETH_ZLEN: usize = 60;
pub const ETH_DATA_LEN: usize = 1500;
pub const ETH_FRAME_LEN: usize = 1514;
pub const ETH_FCS_LEN: usize = 4;
pub const ETH_MIN_MTU: usize = 68;
pub const ETH_MAX_MTU: u32 = 0xFFFF;

/* Ethernet Protocol IDs. */
pub const ETH_P_LOOP: u16 = 0x0060; pub const ETH_P_PUP: u16 = 0x0200;
pub const ETH_P_PUPAT: u16 = 0x0201; pub const ETH_P_TSN: u16 = 0x22F0;
pub const ETH_P_ERSPAN2: u16 = 0x22EB; pub const ETH_P_IP: u16 = 0x0800;
pub const ETH_P_X25: u16 = 0x0805; pub const ETH_P_ARP: u16 = 0x0806;
pub const ETH_P_BPQ: u16 = 0x08FF; pub const ETH_P_IEEEPUP: u16 = 0x0a00;
pub const ETH_P_IEEEPUPAT: u16 = 0x0a01; pub const ETH_P_BATMAN: u16 = 0x4305;
pub const ETH_P_DEC: u16 = 0x6000; pub const ETH_P_DNA_DL: u16 = 0x6001;
pub const ETH_P_DNA_RC: u16 = 0x6002; pub const ETH_P_DNA_RT: u16 = 0x6003;
pub const ETH_P_LAT: u16 = 0x6004; pub const ETH_P_DIAG: u16 = 0x6005;
pub const ETH_P_CUST: u16 = 0x6006; pub const ETH_P_SCA: u16 = 0x6007;
pub const ETH_P_TEB: u16 = 0x6558; pub const ETH_P_RARP: u16 = 0x8035;
pub const ETH_P_ATALK: u16 = 0x809B; pub const ETH_P_AARP: u16 = 0x80F3;
pub const ETH_P_8021Q: u16 = 0x8100; pub const ETH_P_ERSPAN: u16 = 0x88BE;
pub const ETH_P_IPX: u16 = 0x8137; pub const ETH_P_IPV6: u16 = 0x86DD;
pub const ETH_P_PAUSE: u16 = 0x8808; pub const ETH_P_SLOW: u16 = 0x8809;
pub const ETH_P_WCCP: u16 = 0x883E; pub const ETH_P_MPLS_UC: u16 = 0x8847;
pub const ETH_P_MPLS_MC: u16 = 0x8848; pub const ETH_P_ATMMPOA: u16 = 0x884c;
pub const ETH_P_PPP_DISC: u16 = 0x8863; pub const ETH_P_PPP_SES: u16 = 0x8864;
pub const ETH_P_LINK_CTL: u16 = 0x886c; pub const ETH_P_8021AC: u16 = 0x8870;
pub const ETH_P_ATMFATE: u16 = 0x8884; pub const ETH_P_PAE: u16 = 0x888E;
pub const ETH_P_PROFINET: u16 = 0x8892; pub const ETH_P_REALTEK: u16 = 0x8899;
pub const ETH_P_AOE: u16 = 0x88A2; pub const ETH_P_ETHERCAT: u16 = 0x88A4;
pub const ETH_P_8021AD: u16 = 0x88A8; pub const ETH_P_802_EX1: u16 = 0x88B5;
pub const ETH_P_MXLGSW: u16 = 0x88C3; pub const ETH_P_PREAUTH: u16 = 0x88C7;
pub const ETH_P_TIPC: u16 = 0x88CA; pub const ETH_P_LLDP: u16 = 0x88CC;
pub const ETH_P_MRP: u16 = 0x88E3; pub const ETH_P_MACSEC: u16 = 0x88E5;
pub const ETH_P_8021AH: u16 = 0x88E7; pub const ETH_P_MVRP: u16 = 0x88F5;
pub const ETH_P_1588: u16 = 0x88F7; pub const ETH_P_NCSI: u16 = 0x88F8;
pub const ETH_P_PRP: u16 = 0x88FB; pub const ETH_P_CFM: u16 = 0x8902;
pub const ETH_P_FCOE: u16 = 0x8906; pub const ETH_P_IBOE: u16 = 0x8915;
pub const ETH_P_TDLS: u16 = 0x890D; pub const ETH_P_FIP: u16 = 0x8914;
pub const ETH_P_80221: u16 = 0x8917; pub const ETH_P_HSR: u16 = 0x892F;
pub const ETH_P_NSH: u16 = 0x894F; pub const ETH_P_LOOPBACK: u16 = 0x9000;
pub const ETH_P_QINQ1: u16 = 0x9100; pub const ETH_P_QINQ2: u16 = 0x9200;
pub const ETH_P_QINQ3: u16 = 0x9300; pub const ETH_P_YT921X: u16 = 0x9988;
pub const ETH_P_EDSA: u16 = 0xDADA; pub const ETH_P_DSA_8021Q: u16 = 0xDADB;
pub const ETH_P_DSA_A5PSW: u16 = 0xE001; pub const ETH_P_IFE: u16 = 0xED3E;
pub const ETH_P_AF_IUCV: u16 = 0xFBFB; pub const ETH_P_NXP_NETC: u16 = 0xFD3A;
pub const ETH_P_802_3_MIN: u16 = 0x0600;

/* Non DIX types. */
pub const ETH_P_802_3: u16 = 0x0001; pub const ETH_P_AX25: u16 = 0x0002;
pub const ETH_P_ALL: u16 = 0x0003; pub const ETH_P_802_2: u16 = 0x0004;
pub const ETH_P_SNAP: u16 = 0x0005; pub const ETH_P_DDCMP: u16 = 0x0006;
pub const ETH_P_WAN_PPP: u16 = 0x0007; pub const ETH_P_PPP_MP: u16 = 0x0008;
pub const ETH_P_LOCALTALK: u16 = 0x0009; pub const ETH_P_CAN: u16 = 0x000C;
pub const ETH_P_CANFD: u16 = 0x000D; pub const ETH_P_CANXL: u16 = 0x000E;
pub const ETH_P_PPPTALK: u16 = 0x0010; pub const ETH_P_TR_802_2: u16 = 0x0011;
pub const ETH_P_MOBITEX: u16 = 0x0015; pub const ETH_P_CONTROL: u16 = 0x0016;
pub const ETH_P_IRDA: u16 = 0x0017; pub const ETH_P_ECONET: u16 = 0x0018;
pub const ETH_P_HDLC: u16 = 0x0019; pub const ETH_P_ARCNET: u16 = 0x001A;
pub const ETH_P_DSA: u16 = 0x001B; pub const ETH_P_TRAILER: u16 = 0x001C;
pub const ETH_P_PHONET: u16 = 0x00F5; pub const ETH_P_IEEE802154: u16 = 0x00F6;
pub const ETH_P_CAIF: u16 = 0x00F7; pub const ETH_P_XDSA: u16 = 0x00F8;
pub const ETH_P_MAP: u16 = 0x00F9; pub const ETH_P_MCTP: u16 = 0x00FA;
pub const ETH_P_GRE_OSI: u16 = 0x00FE;

#[repr(C, packed)]
pub struct ethhdr {
    pub h_dest: [u8; ETH_ALEN],
    pub h_source: [u8; ETH_ALEN],
    pub h_proto: __be16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
