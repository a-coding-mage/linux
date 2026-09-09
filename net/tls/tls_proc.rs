// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (C) 2019 Netronome Systems, Inc. */

// Dependencies supplied by the Linux kernel and the surrounding TLS
// implementation are intentionally left as external symbols.

// CONFIG_PROC_FS
#[cfg(feature = "CONFIG_PROC_FS")]
static TLS_MIB_LIST: [SnmpMib; 17] = [
    SnmpMibItem("TlsCurrTxSw", LINUX_MIB_TLSCURRTXSW),
    SnmpMibItem("TlsCurrRxSw", LINUX_MIB_TLSCURRRXSW),
    SnmpMibItem("TlsCurrTxDevice", LINUX_MIB_TLSCURRTXDEVICE),
    SnmpMibItem("TlsCurrRxDevice", LINUX_MIB_TLSCURRRXDEVICE),
    SnmpMibItem("TlsTxSw", LINUX_MIB_TLSTXSW),
    SnmpMibItem("TlsRxSw", LINUX_MIB_TLSRXSW),
    SnmpMibItem("TlsTxDevice", LINUX_MIB_TLSTXDEVICE),
    SnmpMibItem("TlsRxDevice", LINUX_MIB_TLSRXDEVICE),
    SnmpMibItem("TlsDecryptError", LINUX_MIB_TLSDECRYPTERROR),
    SnmpMibItem("TlsRxDeviceResync", LINUX_MIB_TLSRXDEVICERESYNC),
    SnmpMibItem("TlsDecryptRetry", LINUX_MIB_TLSDECRYPTRETRY),
    SnmpMibItem("TlsRxNoPadViolation", LINUX_MIB_TLSRXNOPADVIOL),
    SnmpMibItem("TlsRxRekeyOk", LINUX_MIB_TLSRXREKEYOK),
    SnmpMibItem("TlsRxRekeyError", LINUX_MIB_TLSRXREKEYERROR),
    SnmpMibItem("TlsTxRekeyOk", LINUX_MIB_TLSTXREKEYOK),
    SnmpMibItem("TlsTxRekeyError", LINUX_MIB_TLSTXREKEYERROR),
    SnmpMibItem("TlsRxRekeyReceived", LINUX_MIB_TLSRXREKEYRECEIVED),
];

#[cfg(feature = "CONFIG_PROC_FS")]
unsafe fn tls_statistics_seq_show(seq: *mut SeqFile, _v: *mut core::ffi::c_void) -> i32 {
    let mut buf: [c_ulong; TLS_MIB_LIST.len()] = [0; TLS_MIB_LIST.len()];
    let cnt: i32 = TLS_MIB_LIST.len() as i32;
    let net: *mut Net = (*seq).private;

    memset(
        buf.as_mut_ptr() as *mut core::ffi::c_void,
        0,
        core::mem::size_of_val(&buf),
    );
    snmp_get_cpu_field_batch_cnt(
        buf.as_mut_ptr(),
        TLS_MIB_LIST.as_ptr(),
        cnt,
        (*net).mib.tls_statistics,
    );
    let mut i = 0;
    while i < cnt {
        seq_printf(
            seq,
            "%-32s\t%lu\n",
            TLS_MIB_LIST[i as usize].name,
            buf[i as usize],
        );
        i += 1;
    }

    0
}

pub unsafe fn tls_proc_init(net: *mut Net) -> i32 {
    // CONFIG_PROC_FS
    #[cfg(feature = "CONFIG_PROC_FS")]
    {
        if proc_create_net_single(
            "tls_stat",
            0o444,
            (*net).proc_net,
            tls_statistics_seq_show,
            core::ptr::null_mut(),
        )
        .is_null()
        {
            return -ENOMEM;
        }
    }

    0
}

pub unsafe fn tls_proc_fini(net: *mut Net) {
    remove_proc_entry("tls_stat", (*net).proc_net);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
