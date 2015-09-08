// Copyright © 2015, skdltmxn
// Licensed under the MIT License <LICENSE.md>

// 1204
pub const CSIDL_DESKTOP: ::c_int = 0x0000;
pub const CSIDL_INTERNET: ::c_int = 0x0001;
pub const CSIDL_PROGRAMS: ::c_int = 0x0002;
pub const CSIDL_CONTROLS: ::c_int = 0x0003;
pub const CSIDL_PRINTERS: ::c_int = 0x0004;
pub const CSIDL_PERSONAL: ::c_int = 0x0005;
pub const CSIDL_FAVORITES: ::c_int = 0x0006;
pub const CSIDL_STARTUP: ::c_int = 0x0007;
pub const CSIDL_RECENT: ::c_int = 0x0008;
pub const CSIDL_SENDTO: ::c_int = 0x0009;
pub const CSIDL_BITBUCKET: ::c_int = 0x000a;
pub const CSIDL_STARTMENU: ::c_int = 0x000b;
pub const CSIDL_MYDOCUMENTS: ::c_int = CSIDL_PERSONAL;
pub const CSIDL_MYMUSIC: ::c_int = 0x000d;
pub const CSIDL_MYVIDEO: ::c_int = 0x000e;
pub const CSIDL_DESKTOPDIRECTORY: ::c_int = 0x0010;
pub const CSIDL_DRIVES: ::c_int = 0x0011;
pub const CSIDL_NETWORK: ::c_int = 0x0012;
pub const CSIDL_NETHOOD: ::c_int = 0x0013;
pub const CSIDL_FONTS: ::c_int = 0x0014;
pub const CSIDL_TEMPLATES: ::c_int = 0x0015;
pub const CSIDL_COMMON_STARTMENU: ::c_int = 0x0016;
pub const CSIDL_COMMON_PROGRAMS: ::c_int = 0x0017;
pub const CSIDL_COMMON_STARTUP: ::c_int = 0x0018;
pub const CSIDL_COMMON_DESKTOPDIRECTORY: ::c_int = 0x0019;
pub const CSIDL_APPDATA: ::c_int = 0x001a;
pub const CSIDL_PRINTHOOD: ::c_int = 0x001b;
pub const CSIDL_LOCAL_APPDATA: ::c_int = 0x001c;
pub const CSIDL_ALTSTARTUP: ::c_int = 0x001d;
pub const CSIDL_COMMON_ALTSTARTUP: ::c_int = 0x001e;
pub const CSIDL_COMMON_FAVORITES: ::c_int = 0x001f;
pub const CSIDL_INTERNET_CACHE: ::c_int = 0x0020;
pub const CSIDL_COOKIES: ::c_int = 0x0021;
pub const CSIDL_HISTORY: ::c_int = 0x0022;
pub const CSIDL_COMMON_APPDATA: ::c_int = 0x0023;
pub const CSIDL_WINDOWS: ::c_int = 0x0024;
pub const CSIDL_SYSTEM: ::c_int = 0x0025;
pub const CSIDL_PROGRAM_FILES: ::c_int = 0x0026;
pub const CSIDL_MYPICTURES: ::c_int = 0x0027;
pub const CSIDL_PROFILE: ::c_int = 0x0028;
pub const CSIDL_SYSTEMX86: ::c_int = 0x0029;
pub const CSIDL_PROGRAM_FILESX86: ::c_int = 0x002a;
pub const CSIDL_PROGRAM_FILES_COMMON: ::c_int = 0x002b;
pub const CSIDL_PROGRAM_FILES_COMMONX86: ::c_int = 0x002c;
pub const CSIDL_COMMON_TEMPLATES: ::c_int = 0x002d;
pub const CSIDL_COMMON_DOCUMENTS: ::c_int = 0x002e;
pub const CSIDL_COMMON_ADMINTOOLS: ::c_int = 0x002f;
pub const CSIDL_ADMINTOOLS: ::c_int = 0x0030;
pub const CSIDL_CONNECTIONS: ::c_int = 0x0031;
pub const CSIDL_COMMON_MUSIC: ::c_int = 0x0035;
pub const CSIDL_COMMON_PICTURES: ::c_int = 0x0036;
pub const CSIDL_COMMON_VIDEO: ::c_int = 0x0037;
pub const CSIDL_RESOURCES: ::c_int = 0x0038;
pub const CSIDL_RESOURCES_LOCALIZED: ::c_int = 0x0039;
pub const CSIDL_COMMON_OEM_LINKS: ::c_int = 0x003a;
pub const CSIDL_CDBURN_AREA: ::c_int = 0x003b;
pub const CSIDL_COMPUTERSNEARME: ::c_int = 0x003d;
pub const CSIDL_FLAG_CREATE: ::c_int = 0x8000;
pub const CSIDL_FLAG_DONT_VERIFY: ::c_int = 0x4000;
pub const CSIDL_FLAG_DONT_UNEXPAND: ::c_int = 0x2000;
pub const CSIDL_FLAG_NO_ALIAS: ::c_int = 0x1000;
pub const CSIDL_FLAG_PER_USER_INIT: ::c_int = 0x0800;
pub const CSIDL_FLAG_MASK: ::c_int = 0xff00;
//1312
pub const SHGFP_TYPE_CURRENT: ::DWORD = 0;
pub const SHGFP_TYPE_DEFAULT: ::DWORD = 1;