// Copyright © 2016 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// Done as of 10.0.14393.0
#![cfg(feature = "shared.wtypesbase")]
use ctypes::{ c_double, c_short, c_uchar, c_ushort };
use shared::minwindef::{ BYTE, DWORD };
use shared::rpcndr::{ boolean, byte, hyper };
use um::winnt::{ LONG, LPWSTR, WCHAR };
pub type OLECHAR = WCHAR;
pub type LPOLESTR = *mut OLECHAR;
pub type LPCOLESTR = *const OLECHAR;
pub type UCHAR = c_uchar;
pub type SHORT = c_short;
pub type USHORT = c_ushort;
pub type ULONG = DWORD;
pub type DOUBLE = c_double;
STRUCT!{struct COAUTHIDENTITY {
    User: *mut USHORT,
    UserLength: ULONG,
    Domain: *mut USHORT,
    DomainLength: ULONG,
    Password: *mut USHORT,
    PasswordLength: ULONG,
    Flags: ULONG,
}}
STRUCT!{struct COAUTHINFO {
    dwAuthnSvc: DWORD,
    dwAuthzSvc: DWORD,
    pwszServerPrincName: LPWSTR,
    dwAuthnLevel: DWORD,
    dwImpersonationLevel: DWORD,
    pAuthIdentityData: *mut COAUTHIDENTITY,
    dwCapabilities: DWORD,
}}
pub type SCODE = LONG;
pub type PSCODE = *mut SCODE;
ENUM!{enum MEMCTX {
    MEMCTX_TASK = 1,
    MEMCTX_SHARED = 2,
    MEMCTX_MACSYSTEM = 3,
    MEMCTX_UNKNOWN = -1i32 as u32,
    MEMCTX_SAME = -2i32 as u32,
}}
pub const ROTREGFLAGS_ALLOWANYCLIENT: DWORD = 0x1;
pub const APPIDREGFLAGS_ACTIVATE_IUSERVER_INDESKTOP: DWORD = 0x1;
pub const APPIDREGFLAGS_SECURE_SERVER_PROCESS_SD_AND_BIND: DWORD = 0x2;
pub const APPIDREGFLAGS_ISSUE_ACTIVATION_RPC_AT_IDENTIFY: DWORD = 0x4;
pub const APPIDREGFLAGS_IUSERVER_UNMODIFIED_LOGON_TOKEN: DWORD = 0x8;
pub const APPIDREGFLAGS_IUSERVER_SELF_SID_IN_LAUNCH_PERMISSION: DWORD = 0x10;
pub const APPIDREGFLAGS_IUSERVER_ACTIVATE_IN_CLIENT_SESSION_ONLY: DWORD = 0x20;
pub const APPIDREGFLAGS_RESERVED1: DWORD = 0x40;
pub const APPIDREGFLAGS_RESERVED2: DWORD = 0x80;
pub const APPIDREGFLAGS_RESERVED3: DWORD = 0x100;
pub const APPIDREGFLAGS_RESERVED4: DWORD = 0x200;
pub const APPIDREGFLAGS_RESERVED5: DWORD = 0x400;
pub const APPIDREGFLAGS_RESERVED6: DWORD = 0x800;
pub const DCOMSCM_ACTIVATION_USE_ALL_AUTHNSERVICES: DWORD = 0x1;
pub const DCOMSCM_ACTIVATION_DISALLOW_UNSECURE_CALL: DWORD = 0x2;
pub const DCOMSCM_RESOLVE_USE_ALL_AUTHNSERVICES: DWORD = 0x4;
pub const DCOMSCM_RESOLVE_DISALLOW_UNSECURE_CALL: DWORD = 0x8;
pub const DCOMSCM_PING_USE_MID_AUTHNSERVICE: DWORD = 0x10;
pub const DCOMSCM_PING_DISALLOW_UNSECURE_CALL: DWORD = 0x20;
ENUM!{enum CLSCTX {
    CLSCTX_INPROC_SERVER = 0x1,
    CLSCTX_INPROC_HANDLER = 0x2,
    CLSCTX_LOCAL_SERVER = 0x4,
    CLSCTX_INPROC_SERVER16 = 0x8,
    CLSCTX_REMOTE_SERVER = 0x10,
    CLSCTX_INPROC_HANDLER16 = 0x20,
    CLSCTX_RESERVED1 = 0x40,
    CLSCTX_RESERVED2 = 0x80,
    CLSCTX_RESERVED3 = 0x100,
    CLSCTX_RESERVED4 = 0x200,
    CLSCTX_NO_CODE_DOWNLOAD = 0x400,
    CLSCTX_RESERVED5 = 0x800,
    CLSCTX_NO_CUSTOM_MARSHAL = 0x1000,
    CLSCTX_ENABLE_CODE_DOWNLOAD = 0x2000,
    CLSCTX_NO_FAILURE_LOG = 0x4000,
    CLSCTX_DISABLE_AAA = 0x8000,
    CLSCTX_ENABLE_AAA = 0x10000,
    CLSCTX_FROM_DEFAULT_CONTEXT = 0x20000,
    CLSCTX_ACTIVATE_32_BIT_SERVER = 0x40000,
    CLSCTX_ACTIVATE_64_BIT_SERVER = 0x80000,
    CLSCTX_ENABLE_CLOAKING = 0x100000,
    CLSCTX_APPCONTAINER = 0x400000,
    CLSCTX_ACTIVATE_AAA_AS_IU = 0x800000,
    CLSCTX_PS_DLL = 0x80000000,
}}
pub const CLSCTX_VALID_MASK: CLSCTX = CLSCTX_INPROC_SERVER | CLSCTX_INPROC_HANDLER
    | CLSCTX_LOCAL_SERVER | CLSCTX_INPROC_SERVER16 | CLSCTX_REMOTE_SERVER
    | CLSCTX_NO_CODE_DOWNLOAD | CLSCTX_NO_CUSTOM_MARSHAL | CLSCTX_ENABLE_CODE_DOWNLOAD
    | CLSCTX_NO_FAILURE_LOG | CLSCTX_DISABLE_AAA | CLSCTX_ENABLE_AAA | CLSCTX_FROM_DEFAULT_CONTEXT
    | CLSCTX_ACTIVATE_32_BIT_SERVER | CLSCTX_ACTIVATE_64_BIT_SERVER | CLSCTX_ENABLE_CLOAKING
    | CLSCTX_APPCONTAINER | CLSCTX_ACTIVATE_AAA_AS_IU | CLSCTX_PS_DLL;
ENUM!{enum MSHLFLAGS {
    MSHLFLAGS_NORMAL = 0,
    MSHLFLAGS_TABLESTRONG = 1,
    MSHLFLAGS_TABLEWEAK = 2,
    MSHLFLAGS_NOPING = 4,
    MSHLFLAGS_RESERVED1 = 8,
    MSHLFLAGS_RESERVED2 = 16,
    MSHLFLAGS_RESERVED3 = 32,
    MSHLFLAGS_RESERVED4 = 64,
}}
ENUM!{enum MSHCTX {
    MSHCTX_LOCAL = 0,
    MSHCTX_NOSHAREDMEM = 1,
    MSHCTX_DIFFERENTMACHINE = 2,
    MSHCTX_INPROC = 3,
    MSHCTX_CROSSCTX = 4,
}}
STRUCT!{struct BYTE_BLOB {
    clSize: ULONG,
    abData: [byte; 1],
}}
pub type UP_BYTE_BLOB = *mut BYTE_BLOB;
STRUCT!{struct WORD_BLOB {
    clSize: ULONG,
    asData: [c_ushort; 1],
}}
pub type UP_WORD_BLOB = *mut WORD_BLOB;
STRUCT!{struct DWORD_BLOB {
    clSize: ULONG,
    alData: [ULONG; 1],
}}
pub type UP_DWORD_BLOB = *mut DWORD_BLOB;
STRUCT!{struct FLAGGED_BYTE_BLOB {
    fFlags: ULONG,
    clSize: ULONG,
    abData: [byte; 1],
}}
pub type UP_FLAGGED_BYTE_BLOB = *mut FLAGGED_BYTE_BLOB;
STRUCT!{struct FLAGGED_WORD_BLOB {
    fFlags: ULONG,
    clSize: ULONG,
    alData: [ULONG; 1],
}}
pub type UP_FLAGGED_WORD_BLOB = *mut FLAGGED_WORD_BLOB;
STRUCT!{struct BYTE_SIZEDARR {
    clSize: ULONG,
    pData: *mut byte,
}}
STRUCT!{struct WORD_SIZEDARR {
    clSize: ULONG,
    pData: *mut c_ushort,
}}
STRUCT!{struct DWORD_SIZEDARR {
    clSize: ULONG,
    pData: *mut ULONG,
}}
STRUCT!{struct HYPER_SIZEDARR {
    clSize: ULONG,
    pData: *mut hyper,
}}
pub type BOOLEAN = boolean;
STRUCT!{struct BLOB {
    cbSize: ULONG,
    pBlobData: *mut BYTE,
}}
pub type LPBLOB = *mut BLOB;
