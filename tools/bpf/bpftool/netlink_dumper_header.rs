/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
// Copyright (C) 2018 Facebook

// Translated from C header macros. External symbols such as json_output,
// json_wtr, stdout, jsonw_* and fprintf are expected from the surrounding code.

macro_rules! NET_START_OBJECT {
    () => {{
        if json_output {
            jsonw_start_object(json_wtr);
        }
    }};
}

macro_rules! NET_START_OBJECT_NESTED {
    ($name:expr) => {{
        if json_output {
            jsonw_name(json_wtr, $name);
            jsonw_start_object(json_wtr);
        } else {
            fprintf(stdout, "%s {", $name);
        }
    }};
}

macro_rules! NET_START_OBJECT_NESTED2 {
    () => {{
        if json_output {
            jsonw_start_object(json_wtr);
        } else {
            fprintf(stdout, "{");
        }
    }};
}

macro_rules! NET_END_OBJECT_NESTED {
    () => {{
        if json_output {
            jsonw_end_object(json_wtr);
        } else {
            fprintf(stdout, "}");
        }
    }};
}

macro_rules! NET_END_OBJECT {
    () => {{
        if json_output {
            jsonw_end_object(json_wtr);
        }
    }};
}

macro_rules! NET_END_OBJECT_FINAL {
    () => {{
        if json_output {
            jsonw_end_object(json_wtr);
        } else {
            fprintf(stdout, "\n");
        }
    }};
}

macro_rules! NET_START_ARRAY {
    ($name:expr, $fmt_str:expr) => {{
        if json_output {
            jsonw_name(json_wtr, $name);
            jsonw_start_array(json_wtr);
        } else {
            fprintf(stdout, $fmt_str, $name);
        }
    }};
}

macro_rules! NET_END_ARRAY {
    ($endstr:expr) => {{
        if json_output {
            jsonw_end_array(json_wtr);
        } else {
            fprintf(stdout, "%s", $endstr);
        }
    }};
}

macro_rules! NET_DUMP_UINT {
    ($name:expr, $fmt_str:expr, $val:expr) => {{
        if json_output {
            jsonw_uint_field(json_wtr, $name, $val);
        } else {
            fprintf(stdout, $fmt_str, $val);
        }
    }};
}

macro_rules! NET_DUMP_UINT_ONLY {
    ($str:expr) => {{
        if json_output {
            jsonw_uint(json_wtr, $str);
        } else {
            fprintf(stdout, "%u ", $str);
        }
    }};
}

macro_rules! NET_DUMP_STR {
    ($name:expr, $fmt_str:expr, $str:expr) => {{
        if json_output {
            jsonw_string_field(json_wtr, $name, $str);
        } else {
            fprintf(stdout, $fmt_str, $str);
        }
    }};
}

macro_rules! NET_DUMP_STR_ONLY {
    ($str:expr) => {{
        if json_output {
            jsonw_string(json_wtr, $str);
        } else {
            fprintf(stdout, "%s ", $str);
        }
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
