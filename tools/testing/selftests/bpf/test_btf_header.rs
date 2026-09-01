// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

pub const BTF_END_RAW: u32 = 0xdeadbeef;

macro_rules! BTF_INFO_ENC {
    ($kind:expr, $kind_flag:expr, $vlen:expr) => {
        (((($kind_flag) != 0) as u32) << 31) | (($kind) << 24) | (($vlen) & BTF_MAX_VLEN)
    };
}

macro_rules! BTF_TYPE_ENC {
    ($name:expr, $info:expr, $size_or_type:expr) => {
        ($name), ($info), ($size_or_type)
    };
}

macro_rules! BTF_INT_ENC {
    ($encoding:expr, $bits_offset:expr, $nr_bits:expr) => {
        (($encoding) << 24 | ($bits_offset) << 16 | ($nr_bits))
    };
}

macro_rules! BTF_TYPE_INT_ENC {
    ($name:expr, $encoding:expr, $bits_offset:expr, $bits:expr, $sz:expr) => {
        BTF_TYPE_ENC!($name, BTF_INFO_ENC!(BTF_KIND_INT, 0, 0), $sz),
        BTF_INT_ENC!($encoding, $bits_offset, $bits)
    };
}

macro_rules! BTF_FWD_ENC {
    ($name:expr, $kind_flag:expr) => {
        BTF_TYPE_ENC!($name, BTF_INFO_ENC!(BTF_KIND_FWD, $kind_flag, 0), 0)
    };
}

macro_rules! BTF_ARRAY_ENC {
    ($type:expr, $index_type:expr, $nr_elems:expr) => {
        ($type), ($index_type), ($nr_elems)
    };
}

macro_rules! BTF_TYPE_ARRAY_ENC {
    ($type:expr, $index_type:expr, $nr_elems:expr) => {
        BTF_TYPE_ENC!(0, BTF_INFO_ENC!(BTF_KIND_ARRAY, 0, 0), 0),
        BTF_ARRAY_ENC!($type, $index_type, $nr_elems)
    };
}

macro_rules! BTF_STRUCT_ENC {
    ($name:expr, $nr_elems:expr, $sz:expr) => {
        BTF_TYPE_ENC!($name, BTF_INFO_ENC!(BTF_KIND_STRUCT, 0, $nr_elems), $sz)
    };
}

macro_rules! BTF_UNION_ENC {
    ($name:expr, $nr_elems:expr, $sz:expr) => {
        BTF_TYPE_ENC!($name, BTF_INFO_ENC!(BTF_KIND_UNION, 0, $nr_elems), $sz)
    };
}

macro_rules! BTF_VAR_ENC {
    ($name:expr, $type:expr, $linkage:expr) => {
        BTF_TYPE_ENC!($name, BTF_INFO_ENC!(BTF_KIND_VAR, 0, 0), $type), ($linkage)
    };
}

macro_rules! BTF_VAR_SECINFO_ENC {
    ($type:expr, $offset:expr, $size:expr) => {
        ($type), ($offset), ($size)
    };
}

macro_rules! BTF_MEMBER_ENC {
    ($name:expr, $type:expr, $bits_offset:expr) => {
        ($name), ($type), ($bits_offset)
    };
}

macro_rules! BTF_ENUM_ENC {
    ($name:expr, $val:expr) => {
        ($name), ($val)
    };
}

macro_rules! BTF_ENUM64_ENC {
    ($name:expr, $val_lo32:expr, $val_hi32:expr) => {
        ($name), ($val_lo32), ($val_hi32)
    };
}

macro_rules! BTF_MEMBER_OFFSET {
    ($bitfield_size:expr, $bits_offset:expr) => {
        (($bitfield_size) << 24 | ($bits_offset))
    };
}

macro_rules! BTF_TYPEDEF_ENC {
    ($name:expr, $type:expr) => {
        BTF_TYPE_ENC!($name, BTF_INFO_ENC!(BTF_KIND_TYPEDEF, 0, 0), $type)
    };
}

macro_rules! BTF_PTR_ENC {
    ($type:expr) => {
        BTF_TYPE_ENC!(0, BTF_INFO_ENC!(BTF_KIND_PTR, 0, 0), $type)
    };
}

macro_rules! BTF_CONST_ENC {
    ($type:expr) => {
        BTF_TYPE_ENC!(0, BTF_INFO_ENC!(BTF_KIND_CONST, 0, 0), $type)
    };
}

macro_rules! BTF_VOLATILE_ENC {
    ($type:expr) => {
        BTF_TYPE_ENC!(0, BTF_INFO_ENC!(BTF_KIND_VOLATILE, 0, 0), $type)
    };
}

macro_rules! BTF_RESTRICT_ENC {
    ($type:expr) => {
        BTF_TYPE_ENC!(0, BTF_INFO_ENC!(BTF_KIND_RESTRICT, 0, 0), $type)
    };
}

macro_rules! BTF_FUNC_PROTO_ENC {
    ($ret_type:expr, $nargs:expr) => {
        BTF_TYPE_ENC!(0, BTF_INFO_ENC!(BTF_KIND_FUNC_PROTO, 0, $nargs), $ret_type)
    };
}

macro_rules! BTF_FUNC_PROTO_ARG_ENC {
    ($name:expr, $type:expr) => {
        ($name), ($type)
    };
}

macro_rules! BTF_FUNC_ENC {
    ($name:expr, $func_proto:expr) => {
        BTF_TYPE_ENC!($name, BTF_INFO_ENC!(BTF_KIND_FUNC, 0, 0), $func_proto)
    };
}

macro_rules! BTF_TYPE_FLOAT_ENC {
    ($name:expr, $sz:expr) => {
        BTF_TYPE_ENC!($name, BTF_INFO_ENC!(BTF_KIND_FLOAT, 0, 0), $sz)
    };
}

macro_rules! BTF_DECL_ATTR_ENC {
    ($value:expr, $type:expr, $component_idx:expr) => {
        BTF_TYPE_ENC!($value, BTF_INFO_ENC!(BTF_KIND_DECL_TAG, 1, 0), $type), ($component_idx)
    };
}

macro_rules! BTF_DECL_TAG_ENC {
    ($value:expr, $type:expr, $component_idx:expr) => {
        BTF_TYPE_ENC!($value, BTF_INFO_ENC!(BTF_KIND_DECL_TAG, 0, 0), $type), ($component_idx)
    };
}

macro_rules! BTF_TYPE_ATTR_ENC {
    ($value:expr, $type:expr) => {
        BTF_TYPE_ENC!($value, BTF_INFO_ENC!(BTF_KIND_TYPE_TAG, 1, 0), $type)
    };
}

macro_rules! BTF_TYPE_TAG_ENC {
    ($value:expr, $type:expr) => {
        BTF_TYPE_ENC!($value, BTF_INFO_ENC!(BTF_KIND_TYPE_TAG, 0, 0), $type)
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
