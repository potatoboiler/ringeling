#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types)]
extern crate libgeling;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type LGL;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::core::ffi::VaList) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stdin: *mut FILE;
    fn lglrelease(_: *mut LGL);
    fn lglminit(mem_0: *mut libc::c_void, _: lglalloc, _: lglrealloc, _: lgldealloc) -> *mut LGL;
    fn lglclone(_: *mut LGL) -> *mut LGL;
    fn lglversion() -> *const libc::c_char;
    fn lglbnr(name_0: *const libc::c_char, prefix: *const libc::c_char, file_0: *mut FILE);
    fn lglopts(_: *mut LGL, prefix: *const libc::c_char, _: libc::c_int);
    fn lglsetopt(_: *mut LGL, _: *const libc::c_char, _: libc::c_int);
    fn lglgetopt(_: *mut LGL, _: *const libc::c_char) -> libc::c_int;
    fn lglhasopt(_: *mut LGL, _: *const libc::c_char) -> libc::c_int;
    fn lglsetid(_: *mut LGL, tid: libc::c_int, tids: libc::c_int);
    fn lglsetconsumedunits(
        _: *mut LGL,
        consumed: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
        _: *mut libc::c_void,
    );
    fn lglsat(_: *mut LGL) -> libc::c_int;
    fn lglderef(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglstats(_: *mut LGL);
    fn lglgetconfs(_: *mut LGL) -> int64_t;
    fn lglgetdecs(_: *mut LGL) -> int64_t;
    fn lglgetprops(_: *mut LGL) -> int64_t;
    fn lglbytes(_: *mut LGL) -> size_t;
    fn lglmb(_: *mut LGL) -> libc::c_double;
    fn lglmaxmb(_: *mut LGL) -> libc::c_double;
    fn lglprocesstime() -> libc::c_double;
    fn lglseterm(
        _: *mut LGL,
        term_0: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
        _: *mut libc::c_void,
    );
    fn lglsetproduceunit(
        _: *mut LGL,
        produce: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
        _: *mut libc::c_void,
    );
    fn lglsetconsumeunits(
        _: *mut LGL,
        consume: Option<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *mut *mut libc::c_int,
                *mut *mut libc::c_int,
            ) -> (),
        >,
        _: *mut libc::c_void,
    );
    fn lgladd(_: *mut LGL, lit: libc::c_int);
    fn lglsetproducecls(
        _: *mut LGL,
        produce: Option<
            unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_int, libc::c_int) -> (),
        >,
        _: *mut libc::c_void,
    );
    fn lglsetconsumecls(
        _: *mut LGL,
        consume: Option<
            unsafe extern "C" fn(*mut libc::c_void, *mut *mut libc::c_int, *mut libc::c_int) -> (),
        >,
        _: *mut libc::c_void,
    );
    fn lglsetconsumedcls(
        _: *mut LGL,
        consumed: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
        _: *mut libc::c_void,
    );
    fn lglsetlockeq(
        _: *mut LGL,
        lock: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_int>,
        _: *mut libc::c_void,
    );
    fn lglsetunlockeq(
        _: *mut LGL,
        unlock: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int, libc::c_int) -> ()>,
        _: *mut libc::c_void,
    );
    fn lglsetmsglock(
        _: *mut LGL,
        lock: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        unlock: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    );
    fn lglsetime(_: *mut LGL, time: Option<unsafe extern "C" fn() -> libc::c_double>);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn getpid() -> __pid_t;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __gnuc_va_list = __builtin_va_list;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type va_list = __gnuc_va_list;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>;
pub type lglalloc = Option<unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void>;
pub type lgldealloc =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, size_t) -> ()>;
pub type lglrealloc = Option<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, size_t, size_t) -> *mut libc::c_void,
>;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_0 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_0 = 0;
pub type __sighandler_t = Option<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const _SC_SIGSTKSZ: C2RustUnnamed_1 = 250;
pub const _SC_MINSIGSTKSZ: C2RustUnnamed_1 = 249;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_1 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_1 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_1 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_1 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_1 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_1 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_1 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_1 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_1 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_1 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_1 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_1 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_1 = 236;
pub const _SC_IPV6: C2RustUnnamed_1 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_1 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_1 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_1 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_1 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_1 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_1 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_1 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_1 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_1 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_1 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_1 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_1 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_1 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_1 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_1 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_1 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_1 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_1 = 182;
pub const _SC_TRACE: C2RustUnnamed_1 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_1 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_1 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_1 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_1 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_1 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_1 = 175;
pub const _SC_STREAMS: C2RustUnnamed_1 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_1 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_1 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_1 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_1 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_1 = 169;
pub const _SC_2_PBS: C2RustUnnamed_1 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_1 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_1 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_1 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_1 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_1 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_1 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_1 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_1 = 160;
pub const _SC_SPAWN: C2RustUnnamed_1 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_1 = 158;
pub const _SC_SHELL: C2RustUnnamed_1 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_1 = 156;
pub const _SC_REGEXP: C2RustUnnamed_1 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_1 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_1 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_1 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_1 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_1 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_1 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_1 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_1 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_1 = 146;
pub const _SC_PIPE: C2RustUnnamed_1 = 145;
pub const _SC_FIFO: C2RustUnnamed_1 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_1 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_1 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_1 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_1 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_1 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_1 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_1 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_1 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_1 = 135;
pub const _SC_BASE: C2RustUnnamed_1 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_1 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_1 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_1 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_1 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_1 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_1 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_1 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_1 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_1 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_1 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_1 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_1 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_1 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_1 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_1 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_1 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_1 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_1 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_1 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_1 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_1 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_1 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_1 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_1 = 110;
pub const _SC_NZERO: C2RustUnnamed_1 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_1 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_1 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_1 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_1 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_1 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_1 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_1 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_1 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_1 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_1 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_1 = 98;
pub const _SC_2_UPE: C2RustUnnamed_1 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_1 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_1 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_1 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_1 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_1 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_1 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_1 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_1 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_1 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_1 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_1 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_1 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_1 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_1 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_1 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_1 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_1 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_1 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_1 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_1 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_1 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_1 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_1 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_1 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_1 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_1 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_1 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_1 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_1 = 68;
pub const _SC_THREADS: C2RustUnnamed_1 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_1 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_1 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_1 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_1 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_1 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_1 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_1 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_1 = 60;
pub const _SC_SELECT: C2RustUnnamed_1 = 59;
pub const _SC_POLL: C2RustUnnamed_1 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_1 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_1 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_1 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_1 = 54;
pub const _SC_PII: C2RustUnnamed_1 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_1 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_1 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_1 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_1 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_1 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_1 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_1 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_1 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_1 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_1 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_1 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_1 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_1 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_1 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_1 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_1 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_1 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_1 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_1 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_1 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_1 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_1 = 31;
pub const _SC_PAGESIZE: C2RustUnnamed_1 = 30;
pub const _SC_VERSION: C2RustUnnamed_1 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_1 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_1 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_1 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_1 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_1 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_1 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_1 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_1 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_1 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_1 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_1 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_1 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_1 = 16;
pub const _SC_FSYNC: C2RustUnnamed_1 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_1 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_1 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_1 = 12;
pub const _SC_TIMERS: C2RustUnnamed_1 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_1 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_1 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_1 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_1 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_1 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_1 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_1 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_1 = 3;
pub const _SC_CLK_TCK: C2RustUnnamed_1 = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed_1 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Cls {
    pub wid: libc::c_int,
    pub count: libc::c_int,
    pub glue: libc::c_int,
    pub lits: [libc::c_int; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Worker {
    pub lgl: *mut LGL,
    pub thread: pthread_t,
    pub res: libc::c_int,
    pub fixed: libc::c_int,
    pub units: [libc::c_int; 512],
    pub nunits: libc::c_int,
    pub cls: *mut libc::c_int,
    pub szcls: libc::c_int,
    pub clsimported: libc::c_long,
    pub dead: *mut Cls,
    pub stats: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub units: C2RustUnnamed_4,
    pub cls: C2RustUnnamed_3,
    pub eqs: C2RustUnnamed_3,
    pub produced: libc::c_int,
    pub consumed: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub produced: libc::c_int,
    pub consumed: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub calls: libc::c_int,
    pub produced: libc::c_int,
    pub consumed: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub start: *mut *mut Cls,
    pub first: libc::c_long,
    pub num: libc::c_long,
    pub added: libc::c_long,
    pub collected: libc::c_long,
    pub size: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub max: size_t,
    pub current: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub units: libc::c_int,
    pub cls: libc::c_int,
    pub eqs: libc::c_int,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    strtol(
        __nptr,
        std::ptr::null_mut::<libc::c_void>() as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int
}
#[inline]
unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char) -> libc::c_longlong {
    strtoll(
        __nptr,
        std::ptr::null_mut::<libc::c_void>() as *mut *mut libc::c_char,
        10 as libc::c_int,
    )
}
static mut verbose: libc::c_int = 0;
static mut plain: libc::c_int = 0;
static mut nounits: libc::c_int = 0;
static mut noeqs: libc::c_int = 0;
static mut nocls: libc::c_int = 0;
static mut gclim: libc::c_int = 1 as libc::c_int;
static mut gclimset: libc::c_int = 0;
static mut nworkers: libc::c_int = 0;
static mut nconsumers: libc::c_int = 0;
static mut leavebehind: libc::c_int = 0;
static mut locs: libc::c_int = 0;
static mut memlimit: int64_t = 0;
static mut softmemlimit: int64_t = 0;
static mut workers: *mut Worker = 0 as *const Worker as *mut Worker;
static mut nvars: libc::c_int = 0;
static mut nclauses: libc::c_int = 0;
static mut vals: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut fixed: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut repr: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
#[no_mangle]
pub static mut clauses: C2RustUnnamed_5 = C2RustUnnamed_5 {
    start: 0 as *const *mut Cls as *mut *mut Cls,
    first: 0,
    num: 0,
    added: 0,
    collected: 0,
    size: 0,
};
static mut nfixed: libc::c_int = 0;
static mut globalres: libc::c_int = 0;
static mut gcs: libc::c_int = 0;
static mut name: *const libc::c_char = 0 as *const libc::c_char;
static mut nworkers2: libc::c_int = 0;
#[no_mangle]
pub static mut mem: C2RustUnnamed_6 = C2RustUnnamed_6 { max: 0, current: 0 };
static mut catchedsig: libc::c_int = 0;
static mut start: libc::c_double = 0.;
static mut file: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut syncs: C2RustUnnamed_7 = C2RustUnnamed_7 {
    units: 0,
    cls: 0,
    eqs: 0,
};
static mut done: libc::c_int = 0;
static mut termchks: libc::c_int = 0;
static mut units: libc::c_int = 0;
static mut eqs: libc::c_int = 0;
static mut flushed: libc::c_int = 0;
static mut donemutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        
        __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                
                __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                }
            },
        }
    },
};
static mut msgmutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        
        __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                
                __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                }
            },
        }
    },
};
static mut fixedmutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        
        __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                
                __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                }
            },
        }
    },
};
static mut reprmutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        
        __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                
                __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                }
            },
        }
    },
};
static mut memutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        
        __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                
                __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                }
            },
        }
    },
};
static mut clsmutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        
        __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                
                __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                }
            },
        }
    },
};
unsafe extern "C" fn currentime() -> libc::c_double {
    let mut res: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    if gettimeofday(&mut tv, std::ptr::null_mut::<libc::c_void>()) == 0 {
        res = 1e-6f64 * tv.tv_usec as libc::c_double;
        res += tv.tv_sec as libc::c_double;
    }
    res
}
unsafe extern "C" fn getime() -> libc::c_double {
    currentime() - start
}
unsafe extern "C" fn msg(
    mut wid: libc::c_int,
    mut level: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    if verbose < level {
        return;
    }
    pthread_mutex_lock(&mut msgmutex);
    fflush(stderr);
    if wid < 0 as libc::c_int {
        printf(b"c - \0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"c %d \0" as *const u8 as *const libc::c_char, wid);
    }
    printf(b"W %6.1f \0" as *const u8 as *const libc::c_char, getime());
    ap = args.clone();
    vfprintf(stdout, fmt, ap.as_va_list());
    fputc('\n' as i32, stdout);
    fflush(stdout);
    pthread_mutex_unlock(&mut msgmutex);
}
unsafe extern "C" fn die(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    fflush(stdout);
    fputs(
        b"c *** plingeling error: \0" as *const u8 as *const libc::c_char,
        stderr,
    );
    ap = args.clone();
    vfprintf(stderr, fmt, ap.as_va_list());
    fputc('\n' as i32, stderr);
    fflush(stderr);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn warn(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    fflush(stdout);
    fputs(
        b"c *** plingeling warning: \0" as *const u8 as *const libc::c_char,
        stderr,
    );
    ap = args.clone();
    vfprintf(stderr, fmt, ap.as_va_list());
    fputc('\n' as i32, stderr);
    fflush(stderr);
}
unsafe extern "C" fn percent(mut a: libc::c_double, mut b: libc::c_double) -> libc::c_double {
    if b != 0. {
        100 as libc::c_int as libc::c_double * a / b
    } else {
        0 as libc::c_int as libc::c_double
    }
}
unsafe extern "C" fn stats() {
    let mut real: libc::c_double = 0.;
    let mut process: libc::c_double = 0.;
    let mut mpps: libc::c_double = 0.;
    let mut cps: libc::c_double = 0.;
    let mut mb: libc::c_double = 0.;
    let mut decs: int64_t = 0;
    let mut confs: int64_t = 0;
    let mut props: int64_t = 0;
    let mut i: libc::c_int = 0;
    let mut unitcalls: libc::c_int = 0;
    let mut w: *mut Worker = std::ptr::null_mut::<Worker>();
    confs = 0 as libc::c_int as int64_t;
    decs = confs;
    unitcalls = decs as libc::c_int;
    props = 0 as libc::c_int as int64_t;
    mb = mem.max as libc::c_double / ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_double;
    i = 0 as libc::c_int;
    while i < nworkers {
        w = workers.offset(i as isize);
        if !((*w).lgl).is_null() {
            decs += lglgetdecs((*w).lgl);
            confs += lglgetconfs((*w).lgl);
            props += lglgetprops((*w).lgl);
            mb += lglmaxmb((*w).lgl);
            unitcalls += (*w).stats.units.calls;
        }
        i += 1;
        i;
    }
    real = getime();
    process = lglprocesstime();
    cps = if real > 0 as libc::c_int as libc::c_double {
        confs as libc::c_double / real
    } else {
        0 as libc::c_int as libc::c_double
    };
    mpps = if real > 0 as libc::c_int as libc::c_double {
        props as libc::c_double / 1e6f64 / real
    } else {
        0 as libc::c_int as libc::c_double
    };
    printf(
        b"c %d termination checks\n\0" as *const u8 as *const libc::c_char,
        termchks,
    );
    printf(b"c\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"c units: %d found, %d publications, %d syncs, %d flushed\n\0" as *const u8
            as *const libc::c_char,
        units,
        unitcalls,
        syncs.units,
        flushed,
    );
    printf(
        b"c clauses: %ld clauses added, %ld collected %.0f%%, %d gcs\n\0" as *const u8
            as *const libc::c_char,
        clauses.added,
        clauses.collected,
        percent(
            clauses.collected as libc::c_double,
            clauses.added as libc::c_double,
        ),
        gcs,
    );
    printf(
        b"c equivalences: %d found, %d syncs\n\0" as *const u8 as *const libc::c_char,
        eqs,
        syncs.eqs,
    );
    printf(b"c\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"c %lld decisions, %lld conflicts, %.1f conflicts/sec\n\0" as *const u8
            as *const libc::c_char,
        decs as libc::c_longlong,
        confs as libc::c_longlong,
        cps,
    );
    printf(
        b"c %lld0 propagations, %.1f megaprops/sec\n\0" as *const u8 as *const libc::c_char,
        props as libc::c_longlong,
        mpps,
    );
    printf(
        b"c %.1f process time, %.0f%% utilization\n\0" as *const u8 as *const libc::c_char,
        process,
        if real > 0 as libc::c_int as libc::c_double {
            100.0f64 * process / real / nworkers as libc::c_double
        } else {
            0.0f64
        },
    );
    printf(b"c\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"c %.1f seconds, %.1f MB\n\0" as *const u8 as *const libc::c_char,
        real,
        mb,
    );
    fflush(stdout);
}
unsafe extern "C" fn incmem(mut bytes: size_t) {
    if pthread_mutex_lock(&mut memutex) != 0 {
        warn(b"failed to lock 'mem' mutex in 'incmem'\0" as *const u8 as *const libc::c_char);
    }
    mem.current = (mem.current).wrapping_add(bytes);
    if mem.current > mem.max {
        mem.max = mem.current;
    }
    if pthread_mutex_unlock(&mut memutex) != 0 {
        warn(b"failed to unlock 'mem' mutex in 'incmem'\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn decmem(mut bytes: size_t) {
    if pthread_mutex_lock(&mut memutex) != 0 {
        warn(b"failed to lock 'mem' mutex in 'decmem'\0" as *const u8 as *const libc::c_char);
    }
    mem.current = (mem.current).wrapping_sub(bytes);
    if pthread_mutex_unlock(&mut memutex) != 0 {
        warn(b"failed to unlock 'mem' mutex in 'decmem'\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn alloc(mut dummy: *mut libc::c_void, mut bytes: size_t) -> *mut libc::c_void {
    let mut res: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut BYTES: size_t =
        bytes.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong);
    res = malloc(BYTES) as *mut libc::c_char;
    if res.is_null() {
        die(b"out of memory\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memset(res as *mut libc::c_void, 0 as libc::c_int, BYTES);
    incmem(BYTES);
    res as *mut libc::c_void
}
unsafe extern "C" fn dealloc(
    mut dummy: *mut libc::c_void,
    mut void_ptr: *mut libc::c_void,
    mut bytes: size_t,
) {
    let mut char_ptr: *mut libc::c_char = void_ptr as *mut libc::c_char;
    let mut BYTES: size_t =
        bytes.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong);
    decmem(BYTES);
    free(char_ptr as *mut libc::c_void);
}
unsafe extern "C" fn resize(
    mut dummy: *mut libc::c_void,
    mut ptr: *mut libc::c_void,
    mut old_bytes: size_t,
    mut new_bytes: size_t,
) -> *mut libc::c_void {
    let mut res: *mut libc::c_void = std::ptr::null_mut::<libc::c_void>();
    if pthread_mutex_lock(&mut memutex) != 0 {
        warn(b"failed to lock 'mem' mutex in 'resize'\0" as *const u8 as *const libc::c_char);
    }
    mem.current = (mem.current).wrapping_sub(old_bytes);
    mem.current = (mem.current).wrapping_add(new_bytes);
    if mem.current > mem.max {
        mem.max = mem.current;
    }
    if pthread_mutex_unlock(&mut memutex) != 0 {
        warn(b"failed to unlock 'mem' mutex in 'resize'\0" as *const u8 as *const libc::c_char);
    }
    res = realloc(ptr, new_bytes);
    res
}
static mut sig_int_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut sig_segv_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut sig_abrt_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut sig_term_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
unsafe extern "C" fn resetsighandlers() {
    signal(2 as libc::c_int, sig_int_handler);
    signal(11 as libc::c_int, sig_segv_handler);
    signal(6 as libc::c_int, sig_abrt_handler);
    signal(15 as libc::c_int, sig_term_handler);
}
unsafe extern "C" fn caughtsigmsg(mut sig: libc::c_int) {
    if verbose == 0 {
        return;
    }
    printf(
        b"c\nc CAUGHT SIGNAL %d\nc\n\0" as *const u8 as *const libc::c_char,
        sig,
    );
    fflush(stdout);
}
unsafe extern "C" fn catchsig(mut sig: libc::c_int) {
    if catchedsig == 0 {
        fputs(
            b"c s UNKNOWN\n\0" as *const u8 as *const libc::c_char,
            stdout,
        );
        fflush(stdout);
        catchedsig = 1 as libc::c_int;
        caughtsigmsg(sig);
        stats();
        caughtsigmsg(sig);
    }
    resetsighandlers();
    if !(getenv(b"LGLSLEEPONABORT\0" as *const u8 as *const libc::c_char)).is_null() {
        let sec: libc::c_int = 1000 as libc::c_int;
        fprintf(
            stderr,
            b"plingeling: plingeling.c:%d: process %ld sleeping for %d seconds\n\0" as *const u8
                as *const libc::c_char,
            292 as libc::c_int,
            getpid() as libc::c_long,
            sec,
        );
        fflush(stderr);
        sleep(sec as libc::c_uint);
    }
    if (getenv(b"LGLNABORT\0" as *const u8 as *const libc::c_char)).is_null() {
        raise(sig);
    } else {
        exit(1 as libc::c_int);
    };
}
unsafe extern "C" fn setsighandlers() {
    sig_int_handler = signal(
        2 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    sig_abrt_handler = signal(
        6 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    sig_term_handler = signal(
        15 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
unsafe extern "C" fn parse() -> *const libc::c_char {
    let mut ch: libc::c_int = 0;
    let mut lit: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut minlen: libc::c_int = 0;
    let mut maxlen: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut occs: *mut libc::c_int = std::ptr::null_mut::<libc::c_int>();
    loop {
        ch = getc(file);
        if ch != 'c' as i32 {
            break;
        }
        loop {
            ch = getc(file);
            if ch == '\n' as i32 {
                break;
            }
            if ch == -(1 as libc::c_int) {
                return b"EOF in comment\0" as *const u8 as *const libc::c_char;
            }
        }
    }
    if ch != 'p' as i32 {
        return b"expected header or comment\0" as *const u8 as *const libc::c_char;
    }
    ungetc(ch, file);
    if fscanf(
        file,
        b"p cnf %d %d\0" as *const u8 as *const libc::c_char,
        &mut nvars as *mut libc::c_int,
        &mut nclauses as *mut libc::c_int,
    ) != 2 as libc::c_int
    {
        return b"can not parse header\0" as *const u8 as *const libc::c_char;
    }
    msg(
        -(1 as libc::c_int),
        1 as libc::c_int,
        b"p cnf %d %d\0" as *const u8 as *const libc::c_char,
        nvars,
        nclauses,
    );
    let mut BYTES: size_t = ((nvars + 1 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    fixed = malloc(BYTES) as *mut libc::c_int;
    if fixed.is_null() {
        die(b"out of memory\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memset(fixed as *mut libc::c_void, 0 as libc::c_int, BYTES);
    incmem(BYTES);
    let mut BYTES_0: size_t = ((nvars + 1 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    vals = malloc(BYTES_0) as *mut libc::c_int;
    if vals.is_null() {
        die(b"out of memory\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memset(vals as *mut libc::c_void, 0 as libc::c_int, BYTES_0);
    incmem(BYTES_0);
    let mut BYTES_1: size_t = ((2 as libc::c_int * nvars + 1 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    occs = malloc(BYTES_1) as *mut libc::c_int;
    if occs.is_null() {
        die(b"out of memory\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memset(occs as *mut libc::c_void, 0 as libc::c_int, BYTES_1);
    incmem(BYTES_1);
    occs = occs.offset(nvars as isize);
    if noeqs == 0 {
        let mut BYTES_2: size_t = ((nvars + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
        repr = malloc(BYTES_2) as *mut libc::c_int;
        if repr.is_null() {
            die(b"out of memory\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        memset(repr as *mut libc::c_void, 0 as libc::c_int, BYTES_2);
        incmem(BYTES_2);
    }
    minlen = 2147483647 as libc::c_int;
    maxlen = -(1 as libc::c_int);
    len = 0 as libc::c_int;
    loop {
        ch = getc(file);
        if ch == 'c' as i32 {
            loop {
                ch = getc(file);
                if ch == '\n' as i32 {
                    break;
                }
                if ch == -(1 as libc::c_int) {
                    return b"EOF in comment\0" as *const u8 as *const libc::c_char;
                }
            }
        } else {
            if ch == ' ' as i32 || ch == '\n' as i32 || ch == '\r' as i32 || ch == '\t' as i32 {
                continue;
            }
            if ch == -(1 as libc::c_int) {
                let mut avg: libc::c_double = 0.;
                let mut std: libc::c_double = 0.;
                if nclauses > 0 as libc::c_int {
                    return b"not enough clauses\0" as *const u8 as *const libc::c_char;
                }
                if minlen == maxlen {
                    msg(
                        -(1 as libc::c_int),
                        0 as libc::c_int,
                        b"uniform clause length %d\0" as *const u8 as *const libc::c_char,
                        minlen,
                    );
                } else {
                    msg(
                        -(1 as libc::c_int),
                        0 as libc::c_int,
                        b"clause length between %d and %d\0" as *const u8 as *const libc::c_char,
                        minlen,
                        maxlen,
                    );
                }
                if nvars > 0 as libc::c_int {
                    avg = 0 as libc::c_int as libc::c_double;
                    lit = -nvars;
                    while lit <= nvars {
                        if lit != 0 {
                            avg += *occs.offset(lit as isize) as libc::c_double;
                        }
                        lit += 1;
                        lit;
                    }
                    avg /= (2 as libc::c_int * nvars) as libc::c_double;
                    msg(
                        -(1 as libc::c_int),
                        0 as libc::c_int,
                        b"average literal occurrence %.2f\0" as *const u8 as *const libc::c_char,
                        avg,
                    );
                    std = 0 as libc::c_int as libc::c_double;
                    lit = -nvars;
                    while lit <= nvars {
                        if lit != 0 {
                            let mut diff: libc::c_double =
                                avg - *occs.offset(lit as isize) as libc::c_double;
                            std += diff * diff;
                        }
                        lit += 1;
                        lit;
                    }
                    std = sqrt(std / (2 as libc::c_int * nvars) as libc::c_double);
                    msg(
                        -(1 as libc::c_int),
                        0 as libc::c_int,
                        b"literal occurrence standard deviation %.2f\0" as *const u8
                            as *const libc::c_char,
                        std,
                    );
                    if avg > 0 as libc::c_int as libc::c_double {
                        msg(
                            -(1 as libc::c_int),
                            0 as libc::c_int,
                            b"relative literal occurrence standard deviation %.2f%%\0" as *const u8
                                as *const libc::c_char,
                            100.0f64 * std / avg,
                        );
                    }
                } else {
                    std = 0 as libc::c_int as libc::c_double;
                    avg = std;
                }
                occs = occs.offset(-(nvars as isize));
                let mut BYTES_3: size_t = ((2 as libc::c_int * nvars + 1 as libc::c_int)
                    as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
                decmem(BYTES_3);
                free(occs as *mut libc::c_void);
                if avg > 5 as libc::c_int as libc::c_double && std / avg < 0.5f64 {
                    if minlen == maxlen {
                        locs = 2 as libc::c_int;
                        msg(
                            -(1 as libc::c_int),
                            0 as libc::c_int,
                            b"looks like a real uniform random instance\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        locs = 1 as libc::c_int;
                        msg(
                            -(1 as libc::c_int),
                            0 as libc::c_int,
                            b"looks like random instance without uniform clause length\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                } else {
                    locs = 0 as libc::c_int;
                    msg(
                        -(1 as libc::c_int),
                        0 as libc::c_int,
                        b"instance does not seem to be uniform random\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                msg(
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    b"finished parsing in %.1f seconds\0" as *const u8 as *const libc::c_char,
                    getime(),
                );
                return std::ptr::null::<libc::c_char>();
            }
            if ch == '-' as i32 {
                ch = getc(file);
                sign = -(1 as libc::c_int);
            } else {
                sign = 1 as libc::c_int;
            }
            if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                == 0
            {
                return b"expected digit\0" as *const u8 as *const libc::c_char;
            }
            if nclauses == 0 {
                return b"too many clauses\0" as *const u8 as *const libc::c_char;
            }
            lit = ch - '0' as i32;
            loop {
                ch = getc(file);
                if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    break;
                }
                lit = 10 as libc::c_int * lit + (ch - '0' as i32);
            }
            if lit < 0 as libc::c_int || lit > nvars {
                return b"invalid variable index\0" as *const u8 as *const libc::c_char;
            }
            lit *= sign;
            lgladd((*workers.offset(0 as libc::c_int as isize)).lgl, lit);
            if lit == 0 {
                nclauses -= 1;
                nclauses;
                if len > maxlen {
                    maxlen = len;
                }
                if len < minlen {
                    minlen = len;
                }
                len = 0 as libc::c_int;
            } else {
                len += 1;
                len;
                let fresh0 = &mut (*occs.offset(lit as isize));
                *fresh0 += 1;
                *fresh0;
            }
        }
    }
}
unsafe extern "C" fn isposnum(mut str: *const libc::c_char) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    let fresh1 = str;
    str = str.offset(1);
    ch = *fresh1 as libc::c_int;
    if ch == 0
        || *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        return 0 as libc::c_int;
    }
    loop {
        let fresh2 = str;
        str = str.offset(1);
        ch = *fresh2 as libc::c_int;
        if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            break;
        }
    }
    (ch == 0) as libc::c_int
}
unsafe extern "C" fn term(mut voidptr: *mut libc::c_void) -> libc::c_int {
    let mut worker: *mut Worker = voidptr as *mut Worker;
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    let mut res: libc::c_int = 0;
    msg(
        wid,
        3 as libc::c_int,
        b"checking early termination\0" as *const u8 as *const libc::c_char,
    );
    if pthread_mutex_lock(&mut donemutex) != 0 {
        warn(
            b"failed to lock 'done' mutex in termination check\0" as *const u8
                as *const libc::c_char,
        );
    }
    res = done;
    termchks += 1;
    termchks;
    if pthread_mutex_unlock(&mut donemutex) != 0 {
        warn(
            b"failed to unlock 'done' mutex in termination check\0" as *const u8
                as *const libc::c_char,
        );
    }
    msg(
        wid,
        3 as libc::c_int,
        b"early termination check %s\0" as *const u8 as *const libc::c_char,
        if res != 0 {
            b"succeeded\0" as *const u8 as *const libc::c_char
        } else {
            b"failed\0" as *const u8 as *const libc::c_char
        },
    );
    res
}
unsafe extern "C" fn flush(mut worker: *mut Worker, mut keep_locked: libc::c_int) {
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    let mut lit: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    msg(
        wid,
        2 as libc::c_int,
        b"flushing %d units\0" as *const u8 as *const libc::c_char,
        (*worker).nunits,
    );
    if pthread_mutex_lock(&mut fixedmutex) != 0 {
        warn(b"failed to lock 'fixed' mutex in flush\0" as *const u8 as *const libc::c_char);
    }
    flushed += 1;
    flushed;
    i = 0 as libc::c_int;
    while i < (*worker).nunits {
        lit = (*worker).units[i as usize];
        idx = abs(lit);
        (*worker).stats.units.calls += 1;
        (*worker).stats.units.calls;
        val = if lit < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        };
        tmp = *vals.offset(idx as isize);
        if tmp == 0 {
            let fresh3 = nfixed;
            nfixed += 1;
            *fixed.offset(fresh3 as isize) = lit;
            *vals.offset(idx as isize) = val;
            (*worker).stats.units.produced += 1;
            (*worker).stats.units.produced;
            (*worker).stats.produced += 1;
            (*worker).stats.produced;
            units += 1;
            units;
        } else if tmp == -val {
            if pthread_mutex_lock(&mut donemutex) != 0 {
                warn(
                    b"failed to lock 'done' mutex flushing unit\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if globalres == 0 {
                msg(
                    wid,
                    1 as libc::c_int,
                    b"mismatched unit\0" as *const u8 as *const libc::c_char,
                );
            }
            globalres = 20 as libc::c_int;
            done = 1 as libc::c_int;
            if pthread_mutex_unlock(&mut donemutex) != 0 {
                warn(
                    b"failed to unlock 'done' mutex flushing unit\0" as *const u8
                        as *const libc::c_char,
                );
            }
            break;
        }
        i += 1;
        i;
    }
    (*worker).nunits = 0 as libc::c_int;
    if keep_locked != 0 {
        return;
    }
    if pthread_mutex_unlock(&mut fixedmutex) != 0 {
        warn(b"failed to unlock 'fixed' mutex in flush\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn produceunit(mut voidptr: *mut libc::c_void, mut lit: libc::c_int) {
    let mut worker: *mut Worker = voidptr as *mut Worker;
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    let fresh4 = (*worker).nunits;
    (*worker).nunits += 1;
    (*worker).units[fresh4 as usize] = lit;
    msg(
        wid,
        3 as libc::c_int,
        b"producing unit %d\0" as *const u8 as *const libc::c_char,
        lit,
    );
    if (*worker).nunits == (1 as libc::c_int) << 9 as libc::c_int {
        flush(worker, 0 as libc::c_int);
    }
}
unsafe extern "C" fn consumeunits(
    mut voidptr: *mut libc::c_void,
    mut fromptr: *mut *mut libc::c_int,
    mut toptr: *mut *mut libc::c_int,
) {
    let mut worker: *mut Worker = voidptr as *mut Worker;
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    if (*worker).nunits != 0 {
        flush(worker, 1 as libc::c_int);
    } else if pthread_mutex_lock(&mut fixedmutex) != 0 {
        warn(b"failed to lock 'fixed' mutex in consume\0" as *const u8 as *const libc::c_char);
    }
    msg(
        wid,
        3 as libc::c_int,
        b"starting unit synchronization\0" as *const u8 as *const libc::c_char,
    );
    syncs.units += 1;
    syncs.units;
    *fromptr = fixed.offset((*worker).fixed as isize);
    *toptr = fixed.offset(nfixed as isize);
    if pthread_mutex_unlock(&mut fixedmutex) != 0 {
        warn(b"failed to unlock 'fixed' in 'consumeunits'\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn consumedunits(mut voidptr: *mut libc::c_void, mut consumed: libc::c_int) {
    let mut worker: *mut Worker = voidptr as *mut Worker;
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    (*worker).stats.units.consumed += consumed;
    (*worker).stats.consumed += consumed;
    msg(
        wid,
        3 as libc::c_int,
        b"consuming %d units\0" as *const u8 as *const libc::c_char,
        consumed,
    );
}
unsafe extern "C" fn sizecls(mut len: libc::c_int) -> size_t {
    (::core::mem::size_of::<Cls>() as libc::c_ulong).wrapping_add(
        (len as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    )
}
unsafe extern "C" fn lencls(mut c: *mut libc::c_int) -> libc::c_int {
    let mut res: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh5 = c;
        c = c.offset(1);
        if *fresh5 == 0 {
            break;
        }
        res += 1;
        res;
    }
    res
}
unsafe extern "C" fn producecls(
    mut voidptr: *mut libc::c_void,
    mut c: *mut libc::c_int,
    mut glue: libc::c_int,
) {
    let mut worker: *mut Worker = voidptr as *mut Worker;
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    let mut len: libc::c_int = 0;
    let mut q: *mut libc::c_int = std::ptr::null_mut::<libc::c_int>();
    let mut lit: libc::c_int = 0;
    let mut p: *const libc::c_int = std::ptr::null::<libc::c_int>();
    let mut bytes: size_t = 0;
    let mut cls: *mut Cls = std::ptr::null_mut::<Cls>();
    len = lencls(c);
    bytes = sizecls(len);
    cls = malloc(bytes) as *mut Cls;
    if cls.is_null() {
        die(b"out of memory in 'producecls'\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    incmem(bytes);
    (*cls).wid = wid;
    (*cls).glue = glue;
    (*cls).count = 0 as libc::c_int;
    p = c;
    q = ((*cls).lits).as_mut_ptr();
    loop {
        let fresh6 = p;
        p = p.offset(1);
        lit = *fresh6;
        if lit == 0 {
            break;
        }
        let fresh7 = q;
        q = q.offset(1);
        *fresh7 = lit;
    }
    let fresh8 = q;
    q = q.offset(1);
    *fresh8 = 0 as libc::c_int;
    if pthread_mutex_lock(&mut clsmutex) != 0 {
        warn(b"failed to lock 'cls' mutex in 'producecls'\0" as *const u8 as *const libc::c_char);
    }
    msg(
        wid,
        3 as libc::c_int,
        b"producing glue %d length %d clause %ld\0" as *const u8 as *const libc::c_char,
        glue,
        len,
        clauses.added,
    );
    if clauses.num == clauses.size {
        let mut newsize: libc::c_int = (if clauses.size != 0 {
            2 as libc::c_int as libc::c_long * clauses.size
        } else {
            1 as libc::c_int as libc::c_long
        }) as libc::c_int;
        let mut old_bytes: size_t = (clauses.size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Cls>() as libc::c_ulong);
        let mut new_bytes: size_t = (newsize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Cls>() as libc::c_ulong);
        clauses.start = resize(
            std::ptr::null_mut::<libc::c_void>(),
            clauses.start as *mut libc::c_void,
            old_bytes,
            new_bytes,
        ) as *mut *mut Cls;
        clauses.size = newsize as libc::c_long;
    }
    let fresh9 = clauses.num;
    clauses.num += 1;
    let fresh10 = &mut (*(clauses.start).offset(fresh9 as isize));
    *fresh10 = cls;
    clauses.added += 1;
    clauses.added;
    if pthread_mutex_unlock(&mut clsmutex) != 0 {
        warn(b"failed to unlock 'cls' mutex in 'producecls'\0" as *const u8 as *const libc::c_char);
    }
    (*worker).stats.cls.produced += 1;
    (*worker).stats.cls.produced;
    (*worker).stats.produced += 1;
    (*worker).stats.produced;
}
unsafe extern "C" fn deletecls(mut cls: *mut Cls) {
    let mut len: libc::c_int = lencls(((*cls).lits).as_mut_ptr());
    let mut bytes: size_t = sizecls(len);
    decmem(bytes);
    free(cls as *mut libc::c_void);
}
unsafe extern "C" fn deleteallcls() {
    let mut c: *mut Cls = std::ptr::null_mut::<Cls>();
    let mut i: libc::c_int = 0;
    i = clauses.first as libc::c_int;
    while (i as libc::c_long) < clauses.num {
        c = *(clauses.start).offset(i as isize);
        if !c.is_null() {
            deletecls(c);
        }
        i += 1;
        i;
    }
    let mut BYTES: size_t = (clauses.size as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<*mut Cls>() as libc::c_ulong);
    decmem(BYTES);
    free(clauses.start as *mut libc::c_void);
}
unsafe extern "C" fn gcls() {
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut l: libc::c_long = 0;
    let mut c: *mut Cls = std::ptr::null_mut::<Cls>();
    let mut i: libc::c_int = 0;
    gcs += 1;
    gcs;
    msg(
        -(1 as libc::c_int),
        1 as libc::c_int,
        b"garbage collecting clauses: first=%ld, num=%ld, SIZE=%ld\0" as *const u8
            as *const libc::c_char,
        clauses.first,
        clauses.num,
        clauses.size,
    );
    k = 0 as libc::c_int as libc::c_long;
    while k < clauses.first {
        k += 1;
        k;
    }
    while k < clauses.num {
        c = *(clauses.start).offset(k as isize);
        if (*c).count > gclim {
            break;
        }
        let fresh11 = k;
        k += 1;
        let fresh12 = &mut (*(clauses.start).offset(fresh11 as isize));
        *fresh12 = std::ptr::null_mut::<Cls>();
        deletecls(c);
    }
    i = 0 as libc::c_int;
    while i < nworkers {
        let mut worker: *mut Worker = workers.offset(i as isize);
        if k < (*worker).clsimported {
            (*worker).clsimported -= k;
        } else {
            (*worker).clsimported = 0 as libc::c_int as libc::c_long;
        }
        i += 1;
        i;
    }
    j = 0 as libc::c_int as libc::c_long;
    l = k;
    while l < clauses.num {
        let fresh13 = j;
        j += 1;
        let fresh14 = &mut (*(clauses.start).offset(fresh13 as isize));
        *fresh14 = *(clauses.start).offset(l as isize);
        l += 1;
        l;
    }
    clauses.collected += k;
    clauses.first = 0 as libc::c_int as libc::c_long;
    clauses.num -= k;
}
unsafe extern "C" fn consumecls(
    mut voidptr: *mut libc::c_void,
    mut cptr: *mut *mut libc::c_int,
    mut glueptr: *mut libc::c_int,
) {
    let mut worker: *mut Worker = voidptr as *mut Worker;
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    let mut res: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut newsize: libc::c_int = 0;
    let mut cls: *mut Cls = std::ptr::null_mut::<Cls>();
    if pthread_mutex_lock(&mut clsmutex) != 0 {
        warn(b"failed to lock 'cls' mutex in 'consumecls'\0" as *const u8 as *const libc::c_char);
    }
    loop {
        if !((*worker).dead).is_null() {
            deletecls((*worker).dead);
            (*worker).dead = std::ptr::null_mut::<Cls>();
        }
        if (*worker).clsimported == clauses.num {
            msg(
                wid,
                3 as libc::c_int,
                b"all %d clauses already consumed\0" as *const u8 as *const libc::c_char,
                clauses.num,
            );
            *cptr = std::ptr::null_mut::<libc::c_int>();
            break;
        } else {
            let fresh15 = (*worker).clsimported;
            (*worker).clsimported += 1;
            res = fresh15 as libc::c_int;
            cls = *(clauses.start).offset(res as isize);
            if cls.is_null() {
                continue;
            }
            (*cls).count += 1;
            (*cls).count;
            if (*cls).count + leavebehind >= nconsumers {
                let fresh16 = &mut (*(clauses.start).offset(res as isize));
                *fresh16 = std::ptr::null_mut::<Cls>();
                (*worker).dead = cls;
                clauses.first += 1;
                clauses.first;
                if clauses.num > 10000 as libc::c_int as libc::c_long
                    && clauses.first > clauses.num / (nconsumers + 1 as libc::c_int) as libc::c_long
                {
                    gcls();
                }
            }
            if (*cls).wid == wid {
                continue;
            }
            len = lencls(((*cls).lits).as_mut_ptr());
            if len + 1 as libc::c_int >= (*worker).szcls {
                newsize = 2 as libc::c_int * (len + 1 as libc::c_int);
                (*worker).cls = resize(
                    std::ptr::null_mut::<libc::c_void>(),
                    (*worker).cls as *mut libc::c_void,
                    ((*worker).szcls as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
                    (newsize as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
                ) as *mut libc::c_int;
                (*worker).szcls = newsize;
            }
            memcpy(
                (*worker).cls as *mut libc::c_void,
                ((*cls).lits).as_mut_ptr() as *const libc::c_void,
                ((len + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            *cptr = (*worker).cls;
            *glueptr = (*cls).glue;
            msg(
                wid,
                3 as libc::c_int,
                b"consuming glue %d length %d clause %d\0" as *const u8 as *const libc::c_char,
                *glueptr,
                len,
                res,
            );
            break;
        }
    }
    if pthread_mutex_unlock(&mut clsmutex) != 0 {
        warn(b"failed to unlock 'cls' mutex in 'consumecls'\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn consumedcls(mut voidptr: *mut libc::c_void, mut consumed: libc::c_int) {
    let mut worker: *mut Worker = voidptr as *mut Worker;
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    (*worker).stats.cls.consumed += consumed;
    (*worker).stats.consumed += consumed;
    msg(
        wid,
        3 as libc::c_int,
        b"consuming %d clause\0" as *const u8 as *const libc::c_char,
        consumed,
    );
}
unsafe extern "C" fn lockrepr(mut voidptr: *mut libc::c_void) -> *mut libc::c_int {
    let mut worker: *mut Worker = voidptr as *mut Worker;
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    if pthread_mutex_lock(&mut reprmutex) != 0 {
        warn(b"failed to lock 'repr' mutex\0" as *const u8 as *const libc::c_char);
    }
    msg(
        wid,
        3 as libc::c_int,
        b"starting equivalences synchronization\0" as *const u8 as *const libc::c_char,
    );
    syncs.eqs += 1;
    syncs.eqs;
    repr
}
unsafe extern "C" fn unlockrepr(
    mut voidptr: *mut libc::c_void,
    mut consumed: libc::c_int,
    mut produced: libc::c_int,
) {
    let mut worker: *mut Worker = voidptr as *mut Worker;
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    msg(
        wid,
        3 as libc::c_int,
        b"finished equivalences synchronization: %d consumed, %d produced\0" as *const u8
            as *const libc::c_char,
        consumed,
        produced,
    );
    (*worker).stats.eqs.consumed += consumed;
    (*worker).stats.eqs.produced += produced;
    (*worker).stats.consumed += consumed;
    (*worker).stats.produced += produced;
    eqs += produced;
    if pthread_mutex_unlock(&mut reprmutex) != 0 {
        warn(b"failed to unlock 'repr' mutex\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn msglock(mut voidptr: *mut libc::c_void) {
    pthread_mutex_lock(&mut msgmutex);
}
unsafe extern "C" fn msgunlock(mut voidptr: *mut libc::c_void) {
    pthread_mutex_unlock(&mut msgmutex);
}
unsafe extern "C" fn work(mut voidptr: *mut libc::c_void) -> *mut libc::c_void {
    let mut worker: *mut Worker = voidptr as *mut Worker;
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    let mut lgl: *mut LGL = (*worker).lgl;
    msg(
        wid,
        1 as libc::c_int,
        b"running\0" as *const u8 as *const libc::c_char,
    );
    (*worker).res = lglsat(lgl);
    msg(
        wid,
        1 as libc::c_int,
        b"result %d\0" as *const u8 as *const libc::c_char,
        (*worker).res,
    );
    if (*worker).res == 0 {
        return std::ptr::null_mut::<libc::c_void>();
    }
    if pthread_mutex_lock(&mut donemutex) != 0 {
        warn(b"failed to lock 'done' mutex in worker\0" as *const u8 as *const libc::c_char);
    }
    done = 1 as libc::c_int;
    if pthread_mutex_unlock(&mut donemutex) != 0 {
        warn(b"failed to unlock 'done' mutex in worker\0" as *const u8 as *const libc::c_char);
    }
    msg(
        wid,
        2 as libc::c_int,
        b"%d decisions, %d conflicts, %.0f props, %.1f MB\0" as *const u8 as *const libc::c_char,
        lglgetdecs(lgl),
        lglgetconfs(lgl),
        lglgetprops(lgl),
        lglmb(lgl),
    );
    if verbose >= 2 as libc::c_int {
        if pthread_mutex_lock(&mut fixedmutex) != 0 {
            warn(b"failed to lock 'fixed' in work\0" as *const u8 as *const libc::c_char);
        }
        msg(
            wid,
            2 as libc::c_int,
            b"consumed %d units %.0f%%, produced %d units %.0f%%\0" as *const u8
                as *const libc::c_char,
            (*worker).stats.units.consumed,
            percent(
                (*worker).stats.units.consumed as libc::c_double,
                nfixed as libc::c_double,
            ),
            (*worker).stats.units.produced,
            percent(
                (*worker).stats.units.produced as libc::c_double,
                nfixed as libc::c_double,
            ),
        );
        if pthread_mutex_unlock(&mut fixedmutex) != 0 {
            warn(b"failed to unlock 'fixed' in work\0" as *const u8 as *const libc::c_char);
        }
    }
    worker as *mut libc::c_void
}
unsafe extern "C" fn getsystemtotalmem(mut explain: libc::c_int) -> int64_t {
    let mut res: libc::c_longlong = 0;
    let mut p: *mut FILE = popen(
        b"grep MemTotal /proc/meminfo\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if !p.is_null()
        && fscanf(
            p,
            b"MemTotal: %lld kB\0" as *const u8 as *const libc::c_char,
            &mut res as *mut libc::c_longlong,
        ) == 1 as libc::c_int
    {
        if explain != 0 {
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"%lld KB total memory according to /proc/meminfo\0" as *const u8
                    as *const libc::c_char,
                res,
            );
        }
        res <<= 10 as libc::c_int;
    } else {
        res = (12 as libc::c_int as libc::c_longlong) << 30 as libc::c_int;
        if explain != 0 {
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"assuming compiled in memory size of %d GB\0" as *const u8 as *const libc::c_char,
                12 as libc::c_int,
            );
        }
    }
    if !p.is_null() {
        pclose(p);
    }
    res as int64_t
}
unsafe extern "C" fn getsystemcores(mut explain: libc::c_int) -> libc::c_int {
    let mut syscores: libc::c_int = 0;
    let mut coreids: libc::c_int = 0;
    let mut physids: libc::c_int = 0;
    let mut procpuinfocores: libc::c_int = 0;
    let mut usesyscores: libc::c_int = 0;
    let mut useprocpuinfo: libc::c_int = 0;
    let mut amd: libc::c_int = 0;
    let mut intel: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut p: *mut FILE = std::ptr::null_mut::<FILE>();
    syscores = sysconf(_SC_NPROCESSORS_ONLN as libc::c_int) as libc::c_int;
    if explain != 0 {
        if syscores > 0 as libc::c_int {
            msg(
                -(1 as libc::c_int),
                1 as libc::c_int,
                b"'sysconf' reports %d processors online\0" as *const u8 as *const libc::c_char,
                syscores,
            );
        } else {
            msg(
                -(1 as libc::c_int),
                1 as libc::c_int,
                b"'sysconf' fails to determine number of online processors\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    p = popen(
        b"grep '^core id' /proc/cpuinfo 2>/dev/null|sort|uniq|wc -l\0" as *const u8
            as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if !p.is_null() {
        if fscanf(
            p,
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut coreids as *mut libc::c_int,
        ) != 1 as libc::c_int
        {
            coreids = 0 as libc::c_int;
        }
        if explain != 0 {
            if coreids > 0 as libc::c_int {
                msg(
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    b"found %d unique core ids in '/proc/cpuinfo'\0" as *const u8
                        as *const libc::c_char,
                    coreids,
                );
            } else {
                msg(
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    b"failed to extract core ids from '/proc/cpuinfo'\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        pclose(p);
    } else {
        coreids = 0 as libc::c_int;
    }
    p = popen(
        b"grep '^physical id' /proc/cpuinfo 2>/dev/null|sort|uniq|wc -l\0" as *const u8
            as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if !p.is_null() {
        if fscanf(
            p,
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut physids as *mut libc::c_int,
        ) != 1 as libc::c_int
        {
            physids = 0 as libc::c_int;
        }
        if explain != 0 {
            if physids > 0 as libc::c_int {
                msg(
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    b"found %d unique physical ids in '/proc/cpuinfo'\0" as *const u8
                        as *const libc::c_char,
                    physids,
                );
            } else {
                msg(
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    b"failed to extract physical ids from '/proc/cpuinfo'\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        pclose(p);
    } else {
        physids = 0 as libc::c_int;
    }
    if coreids > 0 as libc::c_int && physids > 0 as libc::c_int && {
        procpuinfocores = coreids * physids;
        procpuinfocores > 0 as libc::c_int
    } {
        if explain != 0 {
            msg(
                -(1 as libc::c_int),
                1 as libc::c_int,
                b"%d cores = %d core times %d physical ids in '/proc/cpuinfo'\0" as *const u8
                    as *const libc::c_char,
                procpuinfocores,
                coreids,
                physids,
            );
        }
    } else {
        procpuinfocores = 0 as libc::c_int;
    }
    useprocpuinfo = 0 as libc::c_int;
    usesyscores = useprocpuinfo;
    if procpuinfocores > 0 as libc::c_int && procpuinfocores == syscores {
        if explain != 0 {
            msg(
                -(1 as libc::c_int),
                1 as libc::c_int,
                b"'sysconf' and '/proc/cpuinfo' results match\0" as *const u8
                    as *const libc::c_char,
            );
        }
        usesyscores = 1 as libc::c_int;
    } else if procpuinfocores > 0 as libc::c_int && syscores <= 0 as libc::c_int {
        if explain != 0 {
            msg(
                -(1 as libc::c_int),
                1 as libc::c_int,
                b"only '/proc/cpuinfo' result valid\0" as *const u8 as *const libc::c_char,
            );
        }
        useprocpuinfo = 1 as libc::c_int;
    } else if procpuinfocores <= 0 as libc::c_int && syscores > 0 as libc::c_int {
        if explain != 0 {
            msg(
                -(1 as libc::c_int),
                1 as libc::c_int,
                b"only 'sysconf' result valid\0" as *const u8 as *const libc::c_char,
            );
        }
        usesyscores = 1 as libc::c_int;
    } else {
        intel = (system(
            b"grep vendor /proc/cpuinfo 2>/dev/null|grep -q Intel\0" as *const u8
                as *const libc::c_char,
        ) == 0) as libc::c_int;
        if intel != 0 && explain != 0 {
            msg(
                -(1 as libc::c_int),
                1 as libc::c_int,
                b"found Intel as vendor in '/proc/cpuinfo'\0" as *const u8 as *const libc::c_char,
            );
        }
        amd = (system(
            b"grep vendor /proc/cpuinfo 2>/dev/null|grep -q AMD\0" as *const u8
                as *const libc::c_char,
        ) == 0) as libc::c_int;
        if amd != 0 && explain != 0 {
            msg(
                -(1 as libc::c_int),
                1 as libc::c_int,
                b"found AMD as vendor in '/proc/cpuinfo'\0" as *const u8 as *const libc::c_char,
            );
        }
        if amd != 0 {
            if explain != 0 {
                msg(
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    b"trusting 'sysconf' on AMD\0" as *const u8 as *const libc::c_char,
                );
            }
            usesyscores = 1 as libc::c_int;
        } else if intel != 0 {
            if explain != 0 {
                msg(
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    b"'sysconf' result off by a factor of %f on Intel\0" as *const u8
                        as *const libc::c_char,
                    syscores as libc::c_double / procpuinfocores as libc::c_double,
                );
                msg(
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    b"trusting '/proc/cpuinfo' on Intel\0" as *const u8 as *const libc::c_char,
                );
            }
            useprocpuinfo = 1 as libc::c_int;
        } else {
            if explain != 0 {
                msg(
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    b"trusting 'sysconf' on unknown vendor machine\0" as *const u8
                        as *const libc::c_char,
                );
            }
            usesyscores = 1 as libc::c_int;
        }
    }
    if useprocpuinfo != 0 {
        if explain != 0 {
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"assuming cores = core * physical ids in '/proc/cpuinfo' = %d\0" as *const u8
                    as *const libc::c_char,
                procpuinfocores,
            );
        }
        res = procpuinfocores;
    } else if usesyscores != 0 {
        if explain != 0 {
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"assuming cores = number of processors reported by 'sysconf' = %d\0" as *const u8
                    as *const libc::c_char,
                syscores,
            );
        }
        res = syscores;
    } else {
        if explain != 0 {
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"using compiled in default value of %d workers\0" as *const u8
                    as *const libc::c_char,
                8 as libc::c_int,
            );
        }
        res = 8 as libc::c_int;
    }
    res
}
unsafe extern "C" fn cmproduced(
    mut p: *const libc::c_void,
    mut q: *const libc::c_void,
) -> libc::c_int {
    let mut u: *mut Worker = *(p as *mut *mut Worker);
    let mut v: *mut Worker = *(q as *mut *mut Worker);
    let mut res: libc::c_int = (*v).stats.produced - (*u).stats.produced;
    if res != 0 {
        return res;
    }
    u.offset_from(v) as libc::c_long as libc::c_int
}
unsafe extern "C" fn cmpconsumed(
    mut p: *const libc::c_void,
    mut q: *const libc::c_void,
) -> libc::c_int {
    let mut u: *mut Worker = *(p as *mut *mut Worker);
    let mut v: *mut Worker = *(q as *mut *mut Worker);
    let mut res: libc::c_int = (*v).stats.consumed - (*u).stats.consumed;
    if res != 0 {
        return res;
    }
    u.offset_from(v) as libc::c_long as libc::c_int
}
unsafe extern "C" fn parsenbcoreenv() -> libc::c_int {
    let mut str: *const libc::c_char = getenv(b"NBCORE\0" as *const u8 as *const libc::c_char);
    if str.is_null() {
        return 0 as libc::c_int;
    }
    if isposnum(str) == 0 {
        die(
            b"invalid value '%s' for environment variable NBCORE\0" as *const u8
                as *const libc::c_char,
            str,
        );
    }
    atoi(str)
}
unsafe extern "C" fn setopt(
    mut i: libc::c_int,
    mut lgl: *mut LGL,
    mut opt: *const libc::c_char,
    mut val: libc::c_int,
) {
    let mut oldval: libc::c_int = 0;
    let mut newval: libc::c_int = 0;
    if lglhasopt(lgl, opt) == 0 {
        msg(
            i,
            1 as libc::c_int,
            b"option '%s' does not exist\0" as *const u8 as *const libc::c_char,
            opt,
        );
    } else {
        oldval = lglgetopt(lgl, opt);
        if oldval == val {
            msg(
                i,
                1 as libc::c_int,
                b"option '%s' already set to '%d'\0" as *const u8 as *const libc::c_char,
                opt,
                val,
            );
        } else {
            lglsetopt(lgl, opt, val);
            newval = lglgetopt(lgl, opt);
            if newval != val {
                msg(
                    i,
                    1 as libc::c_int,
                    b"option '%s' set to '%d' (but requested %d)\0" as *const u8
                        as *const libc::c_char,
                    opt,
                    newval,
                    val,
                );
            } else {
                msg(
                    i,
                    1 as libc::c_int,
                    b"option '%s' set to '%d'\0" as *const u8 as *const libc::c_char,
                    opt,
                    val,
                );
            }
        }
    };
}
unsafe extern "C" fn setopts(mut lgl: *mut LGL, mut i: libc::c_int) {
    let mut w: *mut Worker = workers.offset(i as isize);
    let mut j: libc::c_int = 0;
    (*w).lgl = lgl;
    lglsetid(lgl, i, nworkers);
    lglsetime(
        lgl,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> libc::c_double>,
            Option<unsafe extern "C" fn() -> libc::c_double>,
        >(Some(::core::mem::transmute::<
            unsafe extern "C" fn() -> libc::c_double,
            unsafe extern "C" fn() -> libc::c_double,
        >(getime))),
    );
    setopt(
        i,
        lgl,
        b"verbose\0" as *const u8 as *const libc::c_char,
        verbose,
    );
    setopt(i, lgl, b"seed\0" as *const u8 as *const libc::c_char, i);
    setopt(
        i,
        lgl,
        b"classify\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    if i != 0
        && (locs >= 2 as libc::c_int
            || locs != 0 && (i > 7 as libc::c_int || i & 1 as libc::c_int != 0))
    {
        setopt(
            i,
            lgl,
            b"plain\0" as *const u8 as *const libc::c_char,
            (locs == 2 as libc::c_int || i & 3 as libc::c_int == 1 as libc::c_int) as libc::c_int,
        );
        setopt(
            i,
            lgl,
            b"locs\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
        setopt(
            i,
            lgl,
            b"locsrtc\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        setopt(
            i,
            lgl,
            b"locswait\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        setopt(
            i,
            lgl,
            b"locsclim\0" as *const u8 as *const libc::c_char,
            (1 as libc::c_int) << 24 as libc::c_int,
        );
    } else {
        j = if locs != 0 { i / 2 as libc::c_int } else { i };
        match j % 13 as libc::c_int {
            1 => {
                setopt(
                    i,
                    lgl,
                    b"plain\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                );
                setopt(
                    i,
                    lgl,
                    b"decompose\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                );
            }
            2 => {
                setopt(
                    i,
                    lgl,
                    b"restartint\0" as *const u8 as *const libc::c_char,
                    1000 as libc::c_int,
                );
            }
            3 => {
                setopt(
                    i,
                    lgl,
                    b"elmresched\0" as *const u8 as *const libc::c_char,
                    7 as libc::c_int,
                );
            }
            4 => {
                setopt(
                    i,
                    lgl,
                    b"scincincmin\0" as *const u8 as *const libc::c_char,
                    250 as libc::c_int,
                );
            }
            5 => {
                setopt(
                    i,
                    lgl,
                    b"block\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                );
                setopt(
                    i,
                    lgl,
                    b"cce\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                );
            }
            6 => {
                setopt(
                    i,
                    lgl,
                    b"scincinc\0" as *const u8 as *const libc::c_char,
                    50 as libc::c_int,
                );
            }
            7 => {
                setopt(
                    i,
                    lgl,
                    b"phase\0" as *const u8 as *const libc::c_char,
                    -(1 as libc::c_int),
                );
            }
            8 => {
                setopt(
                    i,
                    lgl,
                    b"phase\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                );
            }
            9 => {
                setopt(
                    i,
                    lgl,
                    b"sweeprtc\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                );
            }
            10 => {
                setopt(
                    i,
                    lgl,
                    b"restartint\0" as *const u8 as *const libc::c_char,
                    100 as libc::c_int,
                );
            }
            11 => {
                setopt(
                    i,
                    lgl,
                    b"reduceinit\0" as *const u8 as *const libc::c_char,
                    10000 as libc::c_int,
                );
                setopt(
                    i,
                    lgl,
                    b"reducefixed\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                );
            }
            12 => {
                setopt(
                    i,
                    lgl,
                    b"restartint\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int,
                );
            }
            0 | _ => {}
        }
    }
    lglseterm(
        lgl,
        Some(term as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int),
        w as *mut libc::c_void,
    );
    lglsetmsglock(
        lgl,
        Some(msglock as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        Some(msgunlock as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        w as *mut libc::c_void,
    );
    if nounits == 0 {
        lglsetproduceunit(
            lgl,
            Some(produceunit as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()),
            w as *mut libc::c_void,
        );
        lglsetconsumeunits(
            lgl,
            Some(
                consumeunits
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut libc::c_int,
                        *mut *mut libc::c_int,
                    ) -> (),
            ),
            w as *mut libc::c_void,
        );
        lglsetconsumedunits(
            lgl,
            Some(consumedunits as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()),
            w as *mut libc::c_void,
        );
    }
    if nocls == 0 {
        lglsetproducecls(
            lgl,
            Some(
                producecls
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_int, libc::c_int) -> (),
            ),
            w as *mut libc::c_void,
        );
        lglsetconsumecls(
            lgl,
            Some(
                consumecls
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut libc::c_int,
                        *mut libc::c_int,
                    ) -> (),
            ),
            w as *mut libc::c_void,
        );
        lglsetconsumedcls(
            lgl,
            Some(consumedcls as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()),
            w as *mut libc::c_void,
        );
    }
    if noeqs == 0 {
        lglsetlockeq(
            lgl,
            Some(lockrepr as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_int),
            w as *mut libc::c_void,
        );
        lglsetunlockeq(
            lgl,
            Some(
                unlockrepr
                    as unsafe extern "C" fn(*mut libc::c_void, libc::c_int, libc::c_int) -> (),
            ),
            w as *mut libc::c_void,
        );
    }
    msg(
        i,
        2 as libc::c_int,
        b"initialized\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn bytes2mbll(mut bytes: int64_t) -> libc::c_longlong {
    (bytes as libc::c_longlong + ((1 as libc::c_longlong) << 20 as libc::c_int)
        - 1 as libc::c_int as libc::c_longlong) >> 20 as libc::c_int
}
unsafe extern "C" fn bytes2gbll(mut bytes: int64_t) -> libc::c_longlong {
    (bytes as libc::c_longlong + ((1 as libc::c_longlong) << 30 as libc::c_int)
        - 1 as libc::c_int as libc::c_longlong) >> 30 as libc::c_int
}
unsafe extern "C" fn cloneworker(mut i: libc::c_int) -> libc::c_int {
    msg(
        -(1 as libc::c_int),
        0 as libc::c_int,
        b"trying to clone worker %d from worker 0\0" as *const u8 as *const libc::c_char,
        i,
    );
    msg(
        -(1 as libc::c_int),
        0 as libc::c_int,
        b"prediction: %lld MB to be cloned + allocated %lld MB = %lld MB\0" as *const u8
            as *const libc::c_char,
        bytes2mbll(lglbytes((*workers.offset(0 as libc::c_int as isize)).lgl) as int64_t),
        bytes2mbll(mem.current as int64_t),
        bytes2mbll(
            (lglbytes((*workers.offset(0 as libc::c_int as isize)).lgl)).wrapping_add(mem.current)
                as int64_t,
        ),
    );
    if (lglbytes((*workers.offset(0 as libc::c_int as isize)).lgl)).wrapping_add(mem.current)
        >= softmemlimit as size_t
    {
        msg(
            -(1 as libc::c_int),
            0 as libc::c_int,
            b"will not clone worker %d since soft memory limit %lld MB would be hit\0" as *const u8
                as *const libc::c_char,
            i,
            bytes2mbll(softmemlimit),
        );
        return 0 as libc::c_int;
    }
    let fresh17 = &mut (*workers.offset(i as isize)).lgl;
    *fresh17 = lglclone((*workers.offset(0 as libc::c_int as isize)).lgl);
    setopts((*workers.offset(i as isize)).lgl, i);
    msg(
        -(1 as libc::c_int),
        0 as libc::c_int,
        b"%lld MB total allocated memory after cloning worker %d from worker 0\0" as *const u8
            as *const libc::c_char,
        bytes2mbll(mem.current as int64_t),
        i,
    );
    1 as libc::c_int
}
unsafe extern "C" fn version() {
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, lglversion());
    exit(0 as libc::c_int);
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut clin: libc::c_int = 0;
    let mut lit: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut id: libc::c_int = 0;
    let mut nbcore: libc::c_int = 0;
    let mut witness: libc::c_int = 1 as libc::c_int;
    let mut tobecloned: libc::c_int = 0;
    let mut tobestarted: libc::c_int = 0;
    let mut w: *mut Worker = std::ptr::null_mut::<Worker>();
    let mut winner: *mut Worker = std::ptr::null_mut::<Worker>();
    let mut maxconsumer: *mut Worker = std::ptr::null_mut::<Worker>();
    let mut maxproducer: *mut Worker = std::ptr::null_mut::<Worker>();
    let mut sorted: *mut *mut Worker = std::ptr::null_mut::<*mut Worker>();
    let mut earlyworker: *mut Worker = std::ptr::null_mut::<Worker>();
    let mut sumconsumed: libc::c_int = 0;
    let mut sumconsumedunits: libc::c_int = 0;
    let mut sumconsumedcls: libc::c_int = 0;
    let mut sumconsumedeqs: libc::c_int = 0;
    let mut errstr: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut arg: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut bytes: size_t = 0;
    let mut cmd: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    start = currentime();
    clin = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < argc {
        let mut totalmem: size_t = getsystemtotalmem(0 as libc::c_int) as size_t;
        if strcmp(
            *argv.offset(i as isize),
            b"-h\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            printf(
                b"usage: plingeling [ <option> ... ] [ <dimacs>[.gz|.bz2|.7z|.zip|.lzma] [<threads>] ]\n\nwhere <option> is one of the following:\n\n  -h         print this command line option summary\n  --version  print version and exit\n  -v         increase verbose level\n  -n         do not print solution / witness\n\n  -t <num>   number of worker threads (default %d on this machine)\n  -m <num>   maximal memory in MB (default %lld MB on this machine)\n  -g <num>   maximal memory in GB (default %lld GB on this machine)\n  -l <num>   limit on number of workers before collecting shared clause\n\n  -p         plain portfolio, no sharing, e.g. implies the following:\n\n                --do-not-share-units\n                --do-not-share-clauses\n                --do-not-share-equivalences\n\0"
                    as *const u8 as *const libc::c_char,
                getsystemcores(0 as libc::c_int),
                bytes2mbll(totalmem as int64_t),
                bytes2gbll(totalmem as int64_t),
            );
            exit(0 as libc::c_int);
        } else if strcmp(
            *argv.offset(i as isize),
            b"--version\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            version();
        } else if strcmp(
            *argv.offset(i as isize),
            b"-v\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            verbose += 1;
            verbose;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-p\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            plain = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-n\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            witness = 0 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--do-not-share-units\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            nounits = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--do-not-share-clauses\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            nocls = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--do-not-share-equivalences\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            noeqs = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-t\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if nworkers != 0 {
                die(b"multiple '-t' options\0" as *const u8 as *const libc::c_char);
            }
            if nworkers2 != 0 {
                die(b"number of threads and '-t' option given\0" as *const u8
                    as *const libc::c_char);
            }
            if i + 1 as libc::c_int == argc {
                die(b"argument to '-t' missing\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
            arg = *argv.offset(i as isize);
            if isposnum(arg) == 0 || {
                nworkers = atoi(arg);
                nworkers <= 0 as libc::c_int
            } {
                die(
                    b"invalid argument '%s' to '-t'\0" as *const u8 as *const libc::c_char,
                    arg,
                );
            }
        } else if strcmp(
            *argv.offset(i as isize),
            b"-m\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if memlimit != 0 {
                die(b"multiple '-m' or '-g' options\0" as *const u8 as *const libc::c_char);
            }
            if i + 1 as libc::c_int == argc {
                die(b"argument to '-m' missing\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
            arg = *argv.offset(i as isize);
            if isposnum(arg) == 0 || {
                memlimit = (atoll(arg) << 20 as libc::c_int) as int64_t;
                memlimit <= 0 as libc::c_int as int64_t
            } {
                die(
                    b"invalid argument '%s' to '-m'\0" as *const u8 as *const libc::c_char,
                    arg,
                );
            }
        } else if strcmp(
            *argv.offset(i as isize),
            b"-g\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if memlimit != 0 {
                die(b"multiple '-g' or '-m' options\0" as *const u8 as *const libc::c_char);
            }
            if i + 1 as libc::c_int == argc {
                die(b"argument to '-g' missing\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
            arg = *argv.offset(i as isize);
            if isposnum(arg) == 0 || {
                memlimit = (atoll(arg) << 30 as libc::c_int) as int64_t;
                memlimit <= 0 as libc::c_int as int64_t
            } {
                die(
                    b"invalid argument '%s' to '-g'\0" as *const u8 as *const libc::c_char,
                    arg,
                );
            }
        } else if strcmp(
            *argv.offset(i as isize),
            b"-l\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if gclimset != 0 {
                die(b"multiple '-l' options\0" as *const u8 as *const libc::c_char);
            }
            if i + 1 as libc::c_int == argc {
                die(b"argument to '-l' missing\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
            if isposnum(*argv.offset(i as isize)) == 0 {
                die(
                    b"invalid argument '%s' to '-l'\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
            gclim = atoi(*argv.offset(i as isize));
            gclimset = 1 as libc::c_int;
        } else if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32
        {
            die(
                b"invalid option '%s' (try '-h')\0" as *const u8 as *const libc::c_char,
                *argv.offset(i as isize),
            );
        } else if name.is_null() && isposnum(*argv.offset(i as isize)) != 0 {
            die(
                b"<dimacs> file name can not be a positive number '%s'\0" as *const u8
                    as *const libc::c_char,
                *argv.offset(i as isize),
            );
        } else if !name.is_null() && nworkers2 != 0 {
            die(
                b"too many arguments (including <dimacs> and <threads>)\0" as *const u8
                    as *const libc::c_char,
            );
        } else if !name.is_null() && isposnum(*argv.offset(i as isize)) == 0 {
            die(
                b"expected positive number of threads but got '%s'\0" as *const u8
                    as *const libc::c_char,
                *argv.offset(i as isize),
            );
        } else if !name.is_null() {
            nworkers2 = atoi(*argv.offset(i as isize));
            if nworkers2 <= 0 as libc::c_int {
                die(
                    b"invalid number of threads '%s'\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
        } else {
            name = *argv.offset(i as isize);
        }
        i += 1;
        i;
    }
    if nworkers2 != 0 {
        nworkers = nworkers2;
    }
    lglbnr(
        b"Plingeling Parallel SAT Solver\0" as *const u8 as *const libc::c_char,
        b"c \0" as *const u8 as *const libc::c_char,
        stdout,
    );
    fflush(stdout);
    if verbose != 0 {
        printf(b"c\n\0" as *const u8 as *const libc::c_char);
    }
    nbcore = parsenbcoreenv();
    if nworkers != 0 {
        msg(
            -(1 as libc::c_int),
            1 as libc::c_int,
            b"command line option '-t %d' overwrites system default %d\0" as *const u8
                as *const libc::c_char,
            nworkers,
            getsystemcores(0 as libc::c_int),
        );
        if nbcore != 0 {
            msg(
                -(1 as libc::c_int),
                1 as libc::c_int,
                b"and also overwrites environment variable NBCORE=%d\0" as *const u8
                    as *const libc::c_char,
                nbcore,
            );
        }
    } else if nbcore != 0 {
        msg(
            -(1 as libc::c_int),
            1 as libc::c_int,
            b"environment variable NBCORE=%d overwrites system default %d\0" as *const u8
                as *const libc::c_char,
            nbcore,
            getsystemcores(0 as libc::c_int),
        );
        nworkers = nbcore;
    } else {
        msg(
            -(1 as libc::c_int),
            1 as libc::c_int,
            b"no explicit specification of number of workers\0" as *const u8 as *const libc::c_char,
        );
        nworkers = getsystemcores(1 as libc::c_int);
    }
    msg(
        -(1 as libc::c_int),
        0 as libc::c_int,
        b"USING %d WORKER THREADS\0" as *const u8 as *const libc::c_char,
        nworkers,
    );
    if memlimit != 0 {
        msg(
            -(1 as libc::c_int),
            1 as libc::c_int,
            b"memory limit %lld MB ('-g %lld') overwrites system default %lld MB\0" as *const u8
                as *const libc::c_char,
            bytes2mbll(memlimit),
            bytes2gbll(memlimit),
            bytes2mbll(getsystemtotalmem(0 as libc::c_int)),
        );
    } else {
        memlimit = getsystemtotalmem(1 as libc::c_int);
        msg(
            -(1 as libc::c_int),
            1 as libc::c_int,
            b"memory limit set to system default of %lld MB total memory\0" as *const u8
                as *const libc::c_char,
            bytes2mbll(memlimit),
        );
    }
    softmemlimit = (memlimit + 2 as libc::c_int as int64_t) / 3 as libc::c_int as int64_t;
    msg(
        -(1 as libc::c_int),
        0 as libc::c_int,
        b"soft memory limit set to %lld MB\0" as *const u8 as *const libc::c_char,
        bytes2mbll(softmemlimit),
    );
    if gclimset != 0 {
        msg(
            -(1 as libc::c_int),
            0 as libc::c_int,
            b"garbage collection limit %d ('-l %d')\0" as *const u8 as *const libc::c_char,
            gclim,
            gclim,
        );
    } else {
        msg(
            -(1 as libc::c_int),
            0 as libc::c_int,
            b"default garbage collection limit %d\0" as *const u8 as *const libc::c_char,
            gclim,
        );
    }
    if plain != 0 {
        noeqs = 1 as libc::c_int;
        nocls = noeqs;
        nounits = nocls;
        msg(
            -(1 as libc::c_int),
            0 as libc::c_int,
            b"not sharing anything in plain portolio mode ('-p')\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        if noeqs != 0 {
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"not sharing units ('--do-not-share-units')\0" as *const u8 as *const libc::c_char,
            );
        } else {
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"sharing of units enabled\0" as *const u8 as *const libc::c_char,
            );
        }
        if nocls != 0 {
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"not sharing clauses ('--do-not-share-clauses')\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"sharing of clauses enabled\0" as *const u8 as *const libc::c_char,
            );
        }
        if noeqs != 0 {
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"not sharing equivalences ('--do-not-share-equivalences')\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"sharing of equivalences enabled\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    let mut BYTES: size_t =
        (nworkers as libc::c_ulong).wrapping_mul(::core::mem::size_of::<Worker>() as libc::c_ulong);
    workers = malloc(BYTES) as *mut Worker;
    if workers.is_null() {
        die(b"out of memory\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memset(workers as *mut libc::c_void, 0 as libc::c_int, BYTES);
    incmem(BYTES);
    let fresh18 = &mut (*workers.offset(0 as libc::c_int as isize)).lgl;
    *fresh18 = lglminit(
        std::ptr::null_mut::<libc::c_void>(),
        Some(alloc as unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void),
        Some(
            resize
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    size_t,
                    size_t,
                ) -> *mut libc::c_void,
        ),
        Some(dealloc as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, size_t) -> ()),
    );
    lglsetopt(
        (*workers.offset(0 as libc::c_int as isize)).lgl,
        b"druplig\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    setopt(
        0 as libc::c_int,
        (*workers.offset(0 as libc::c_int as isize)).lgl,
        b"bca\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    if verbose == 0 {
        setopt(
            0 as libc::c_int,
            (*workers.offset(0 as libc::c_int as isize)).lgl,
            b"trep\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    if verbose == 0 {
        setopt(
            0 as libc::c_int,
            (*workers.offset(0 as libc::c_int as isize)).lgl,
            b"profile\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if verbose != 0 {
        lglopts(
            (*workers.offset(0 as libc::c_int as isize)).lgl,
            b"c 0 \0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    setsighandlers();
    if !name.is_null() {
        if strlen(name) >= 3 as libc::c_int as libc::c_ulong
            && strcmp(
                name.offset(strlen(name) as isize)
                    .offset(-(3 as libc::c_int as isize)),
                b".gz\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            bytes = (strlen(name)).wrapping_add(30 as libc::c_int as libc::c_ulong);
            cmd = alloc(std::ptr::null_mut::<libc::c_void>(), bytes) as *mut libc::c_char;
            sprintf(
                cmd,
                b"gunzip -c %s\0" as *const u8 as *const libc::c_char,
                name,
            );
            file = popen(cmd, b"r\0" as *const u8 as *const libc::c_char);
            dealloc(std::ptr::null_mut::<libc::c_void>(), cmd as *mut libc::c_void, bytes);
            clin = 4 as libc::c_int;
        } else if strlen(name) >= 4 as libc::c_int as libc::c_ulong
            && strcmp(
                name.offset(strlen(name) as isize)
                    .offset(-(4 as libc::c_int as isize)),
                b".zip\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            bytes = (strlen(name)).wrapping_add(30 as libc::c_int as libc::c_ulong);
            cmd = alloc(std::ptr::null_mut::<libc::c_void>(), bytes) as *mut libc::c_char;
            sprintf(
                cmd,
                b"unzip -p %s\0" as *const u8 as *const libc::c_char,
                name,
            );
            file = popen(cmd, b"r\0" as *const u8 as *const libc::c_char);
            dealloc(std::ptr::null_mut::<libc::c_void>(), cmd as *mut libc::c_void, bytes);
            clin = 4 as libc::c_int;
        } else if strlen(name) >= 5 as libc::c_int as libc::c_ulong
            && strcmp(
                name.offset(strlen(name) as isize)
                    .offset(-(5 as libc::c_int as isize)),
                b".lzma\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            bytes = (strlen(name)).wrapping_add(30 as libc::c_int as libc::c_ulong);
            cmd = alloc(std::ptr::null_mut::<libc::c_void>(), bytes) as *mut libc::c_char;
            sprintf(cmd, b"lzcat %s\0" as *const u8 as *const libc::c_char, name);
            file = popen(cmd, b"r\0" as *const u8 as *const libc::c_char);
            dealloc(std::ptr::null_mut::<libc::c_void>(), cmd as *mut libc::c_void, bytes);
            clin = 4 as libc::c_int;
        } else if strlen(name) >= 4 as libc::c_int as libc::c_ulong
            && strcmp(
                name.offset(strlen(name) as isize)
                    .offset(-(4 as libc::c_int as isize)),
                b".bz2\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            bytes = (strlen(name)).wrapping_add(30 as libc::c_int as libc::c_ulong);
            cmd = alloc(std::ptr::null_mut::<libc::c_void>(), bytes) as *mut libc::c_char;
            sprintf(cmd, b"bzcat %s\0" as *const u8 as *const libc::c_char, name);
            file = popen(cmd, b"r\0" as *const u8 as *const libc::c_char);
            dealloc(std::ptr::null_mut::<libc::c_void>(), cmd as *mut libc::c_void, bytes);
            clin = 4 as libc::c_int;
        } else if strlen(name) >= 3 as libc::c_int as libc::c_ulong
            && strcmp(
                name.offset(strlen(name) as isize)
                    .offset(-(3 as libc::c_int as isize)),
                b".7z\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            bytes = (strlen(name)).wrapping_add(40 as libc::c_int as libc::c_ulong);
            cmd = alloc(std::ptr::null_mut::<libc::c_void>(), bytes) as *mut libc::c_char;
            sprintf(
                cmd,
                b"7z x -so %s 2>/dev/null\0" as *const u8 as *const libc::c_char,
                name,
            );
            file = popen(cmd, b"r\0" as *const u8 as *const libc::c_char);
            dealloc(std::ptr::null_mut::<libc::c_void>(), cmd as *mut libc::c_void, bytes);
            clin = 4 as libc::c_int;
        } else {
            file = fopen(name, b"r\0" as *const u8 as *const libc::c_char);
            clin = 1 as libc::c_int;
        }
        if file.is_null() {
            die(
                b"can not read %s\0" as *const u8 as *const libc::c_char,
                name,
            );
        }
    } else {
        file = stdin;
        name = b"<stdin>\0" as *const u8 as *const libc::c_char;
    }
    msg(
        -(1 as libc::c_int),
        0 as libc::c_int,
        b"parsing %s\0" as *const u8 as *const libc::c_char,
        name,
    );
    errstr = parse();
    if !errstr.is_null() {
        die(
            b"parse error: %s\0" as *const u8 as *const libc::c_char,
            errstr,
        );
    }
    if clin == 1 as libc::c_int {
        fclose(file);
    }
    if clin == 2 as libc::c_int {
        pclose(file);
    }
    locs = 0 as libc::c_int;
    setopts(
        (*workers.offset(0 as libc::c_int as isize)).lgl,
        0 as libc::c_int,
    );
    if locs >= 2 as libc::c_int {
        nconsumers = 1 as libc::c_int;
    } else if locs == 1 as libc::c_int {
        nconsumers = 1 as libc::c_int;
        i = 1 as libc::c_int;
        while i < nworkers {
            if i < 8 as libc::c_int && i & 1 as libc::c_int == 0 {
                nconsumers += 1;
                nconsumers;
            }
            i += 1;
            i;
        }
    } else {
        nconsumers = nworkers;
    }
    msg(
        -(1 as libc::c_int),
        1 as libc::c_int,
        b"assuming %d consumers out of %d workers\0" as *const u8 as *const libc::c_char,
        nconsumers,
        nworkers,
    );
    leavebehind = nconsumers / 4 as libc::c_int;
    msg(
        -(1 as libc::c_int),
        1 as libc::c_int,
        b"will leave behind %d consumers out of %d\0" as *const u8 as *const libc::c_char,
        leavebehind,
        nconsumers,
    );
    earlyworker = if nworkers > 1 as libc::c_int && cloneworker(1 as libc::c_int) != 0 {
        workers.offset(1 as libc::c_int as isize)
    } else {
        std::ptr::null_mut::<Worker>()
    };
    if !earlyworker.is_null() {
        if pthread_create(
            &mut (*earlyworker).thread,
            std::ptr::null::<pthread_attr_t>(),
            Some(work as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
            earlyworker as *mut libc::c_void,
        ) != 0
        {
            die(
                b"failed to create additional early worker thread 1\0" as *const u8
                    as *const libc::c_char,
            );
        }
        msg(
            -(1 as libc::c_int),
            2 as libc::c_int,
            b"started additional early worker 1\0" as *const u8 as *const libc::c_char,
        );
    }
    if locs == 0 {
        msg(
            -(1 as libc::c_int),
            0 as libc::c_int,
            b"simplifying original formula with worker 0\0" as *const u8 as *const libc::c_char,
        );
        lglsetopt(
            (*workers.offset(0 as libc::c_int as isize)).lgl,
            b"clim\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        work(workers.offset(0 as libc::c_int as isize) as *mut libc::c_void);
        lglsetopt(
            (*workers.offset(0 as libc::c_int as isize)).lgl,
            b"clim\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
    }
    res = (*workers.offset(0 as libc::c_int as isize)).res;
    if res != 0 {
        msg(
            -(1 as libc::c_int),
            1 as libc::c_int,
            b"simplification of first worker 0 produced %d\0" as *const u8 as *const libc::c_char,
            res,
        );
        if !earlyworker.is_null() {
            if pthread_join((*earlyworker).thread, std::ptr::null_mut::<*mut libc::c_void>()) != 0 {
                die(b"failed to join early worker thread 1\0" as *const u8 as *const libc::c_char);
            }
            msg(
                -(1 as libc::c_int),
                2 as libc::c_int,
                b"joined early worker 1\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        tobecloned = nworkers - 2 as libc::c_int + earlyworker.is_null() as libc::c_int;
        if tobecloned > 0 as libc::c_int {
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"trying to clone %d workers\0" as *const u8 as *const libc::c_char,
                tobecloned,
            );
            i = 1 as libc::c_int;
            while i < nworkers {
                if earlyworker != workers.offset(i as isize) && cloneworker(i) == 0 {
                    break;
                }
                i += 1;
                i;
            }
        }
        tobestarted = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < nworkers {
            if workers.offset(i as isize) != earlyworker {
                tobestarted += 1;
                tobestarted;
            }
            i += 1;
            i;
        }
        msg(
            -(1 as libc::c_int),
            2 as libc::c_int,
            b"starting %d worker threads\0" as *const u8 as *const libc::c_char,
            tobestarted,
        );
        i = 0 as libc::c_int;
        while i < nworkers {
            w = workers.offset(i as isize);
            if w != earlyworker && !((*w).lgl).is_null() {
                if pthread_create(
                    &mut (*w).thread,
                    std::ptr::null::<pthread_attr_t>(),
                    Some(work as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
                    w as *mut libc::c_void,
                ) != 0
                {
                    die(
                        b"failed to create worker thread %d\0" as *const u8
                            as *const libc::c_char,
                        i,
                    );
                }
                msg(
                    -(1 as libc::c_int),
                    2 as libc::c_int,
                    b"started worker %d\0" as *const u8 as *const libc::c_char,
                    i,
                );
            }
            i += 1;
            i;
        }
        msg(
            -(1 as libc::c_int),
            2 as libc::c_int,
            b"joining %d workers\0" as *const u8 as *const libc::c_char,
            tobestarted + 1 as libc::c_int - earlyworker.is_null() as libc::c_int,
        );
        i = 0 as libc::c_int;
        while i < nworkers {
            w = workers.offset(i as isize);
            if !((*w).lgl).is_null() {
                if pthread_join((*w).thread, std::ptr::null_mut::<*mut libc::c_void>()) != 0 {
                    die(
                        b"failed to join worker thread %d\0" as *const u8 as *const libc::c_char,
                        i,
                    );
                }
                msg(
                    -(1 as libc::c_int),
                    2 as libc::c_int,
                    b"joined worker %d\0" as *const u8 as *const libc::c_char,
                    i,
                );
            }
            i += 1;
            i;
        }
    }
    winner = std::ptr::null_mut::<Worker>();
    maxconsumer = winner;
    maxproducer = maxconsumer;
    i = 0 as libc::c_int;
    while i < nworkers {
        w = workers.offset(i as isize);
        if !((*w).lgl).is_null() {
            if (*w).res != 0 {
                if res == 0 {
                    res = (*w).res;
                    winner = w;
                    msg(
                        -(1 as libc::c_int),
                        0 as libc::c_int,
                        b"worker %d is the WINNER with result %d\0" as *const u8
                            as *const libc::c_char,
                        i,
                        res,
                    );
                } else if res != (*w).res {
                    die(b"result discrepancy\0" as *const u8 as *const libc::c_char);
                }
            }
            if maxconsumer.is_null() || (*w).stats.consumed > (*maxconsumer).stats.consumed {
                maxconsumer = w;
            }
            if maxproducer.is_null() || (*w).stats.produced > (*maxproducer).stats.produced {
                maxproducer = w;
            }
        }
        i += 1;
        i;
    }
    let mut BYTES_0: size_t = (nworkers as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<*mut Worker>() as libc::c_ulong);
    sorted = malloc(BYTES_0) as *mut *mut Worker;
    if sorted.is_null() {
        die(b"out of memory\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memset(sorted as *mut libc::c_void, 0 as libc::c_int, BYTES_0);
    incmem(BYTES_0);
    i = 0 as libc::c_int;
    while i < nworkers {
        let fresh19 = &mut (*sorted.offset(i as isize));
        *fresh19 = workers.offset(i as isize);
        i += 1;
        i;
    }
    printf(b"c\n\0" as *const u8 as *const libc::c_char);
    qsort(
        sorted as *mut libc::c_void,
        nworkers as size_t,
        ::core::mem::size_of::<*mut Worker>() as libc::c_ulong,
        Some(
            cmproduced
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < nworkers {
        w = *sorted.offset(i as isize);
        if !((*w).lgl).is_null() {
            id = w.offset_from(workers) as libc::c_long as libc::c_int;
            printf(
                b"c %2d %s %7d %3.0f%% = %7d units %3.0f%% + %7d cls %3.0f%% + %7d eqs %3.0f%%\n\0"
                    as *const u8 as *const libc::c_char,
                id,
                if w == maxproducer {
                    b"PROD\0" as *const u8 as *const libc::c_char
                } else {
                    b"prod\0" as *const u8 as *const libc::c_char
                },
                (*w).stats.produced,
                percent(
                    (*w).stats.produced as libc::c_double,
                    ((units + eqs) as libc::c_long + clauses.added) as libc::c_double,
                ),
                (*w).stats.units.produced,
                percent(
                    (*w).stats.units.produced as libc::c_double,
                    units as libc::c_double,
                ),
                (*w).stats.cls.produced,
                percent(
                    (*w).stats.cls.produced as libc::c_double,
                    clauses.added as libc::c_double,
                ),
                (*w).stats.eqs.produced,
                percent(
                    (*w).stats.eqs.produced as libc::c_double,
                    eqs as libc::c_double,
                ),
            );
        }
        i += 1;
        i;
    }
    fputs(b"c \0" as *const u8 as *const libc::c_char, stdout);
    i = 0 as libc::c_int;
    while i < 79 as libc::c_int {
        fputc('-' as i32, stdout);
        i += 1;
        i;
    }
    fputc('\n' as i32, stdout);
    printf(
        b"c    prod %7lld 100%% = %7d units 100%% + %7lld cls 100%% + %7d eqs 100%%\n\0"
            as *const u8 as *const libc::c_char,
        units as libc::c_longlong + eqs as libc::c_longlong + clauses.added as libc::c_longlong,
        units,
        clauses.added as libc::c_longlong,
        eqs,
    );
    printf(b"c\n\0" as *const u8 as *const libc::c_char);
    qsort(
        sorted as *mut libc::c_void,
        nworkers as size_t,
        ::core::mem::size_of::<*mut Worker>() as libc::c_ulong,
        Some(
            cmpconsumed
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        ),
    );
    sumconsumedeqs = 0 as libc::c_int;
    sumconsumedcls = sumconsumedeqs;
    sumconsumedunits = sumconsumedcls;
    sumconsumed = sumconsumedunits;
    i = 0 as libc::c_int;
    while i < nworkers {
        w = *sorted.offset(i as isize);
        if !((*w).lgl).is_null() {
            id = w.offset_from(workers) as libc::c_long as libc::c_int;
            sumconsumed += (*w).stats.consumed;
            sumconsumedeqs += (*w).stats.eqs.consumed;
            sumconsumedcls += (*w).stats.cls.consumed;
            sumconsumedunits += (*w).stats.units.consumed;
            printf(
                b"c %2d %s %7d %3.0f%% = %7d units %3.0f%% + %7d cls %3.0f%% + %7d eqs %3.0f%%\n\0"
                    as *const u8 as *const libc::c_char,
                id,
                if w == maxconsumer {
                    b"CONS\0" as *const u8 as *const libc::c_char
                } else {
                    b"cons\0" as *const u8 as *const libc::c_char
                },
                (*w).stats.consumed,
                percent(
                    (*w).stats.consumed as libc::c_double,
                    ((units + eqs) as libc::c_long + clauses.added) as libc::c_double,
                ),
                (*w).stats.units.consumed,
                percent(
                    (*w).stats.units.consumed as libc::c_double,
                    units as libc::c_double,
                ),
                (*w).stats.cls.consumed,
                percent(
                    (*w).stats.cls.consumed as libc::c_double,
                    clauses.added as libc::c_double,
                ),
                (*w).stats.eqs.consumed,
                percent(
                    (*w).stats.eqs.consumed as libc::c_double,
                    eqs as libc::c_double,
                ),
            );
        }
        i += 1;
        i;
    }
    fputs(b"c \0" as *const u8 as *const libc::c_char, stdout);
    i = 0 as libc::c_int;
    while i < 79 as libc::c_int {
        fputc('-' as i32, stdout);
        i += 1;
        i;
    }
    fputc('\n' as i32, stdout);
    printf(
        b"c    cons %7d %3.0f%% = %7d units %3.0f%% + %7d cls %3.0f%% + %7d eqs %3.0f%%\n\0"
            as *const u8 as *const libc::c_char,
        sumconsumed,
        percent(
            sumconsumed as libc::c_double,
            ((units + eqs) as libc::c_long + clauses.added) as libc::c_double,
        ),
        sumconsumedunits,
        percent(sumconsumedunits as libc::c_double, units as libc::c_double),
        sumconsumedcls,
        percent(
            sumconsumedcls as libc::c_double,
            clauses.added as libc::c_double,
        ),
        sumconsumedeqs,
        percent(sumconsumedeqs as libc::c_double, eqs as libc::c_double),
    );
    let mut BYTES_1: size_t = (nworkers as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<*mut Worker>() as libc::c_ulong);
    decmem(BYTES_1);
    free(sorted as *mut libc::c_void);
    fflush(stdout);
    if res == 0 {
        res = globalres;
    }
    if res == 0 {
        die(b"no result by any worker\0" as *const u8 as *const libc::c_char);
    }
    msg(
        -(1 as libc::c_int),
        2 as libc::c_int,
        b"copying assignment\0" as *const u8 as *const libc::c_char,
    );
    if !winner.is_null() && res == 10 as libc::c_int {
        i = 1 as libc::c_int;
        while i <= nvars {
            val = lglderef((*winner).lgl, i);
            vals.offset(i as isize);0;
            *vals.offset(i as isize) = val;
            i += 1;
            i;
        }
    }
    resetsighandlers();
    if res == 10 as libc::c_int {
        printf(b"s SATISFIABLE\n\0" as *const u8 as *const libc::c_char);
        if witness != 0 {
            fflush(stdout);
            if nvars != 0 {
                printf(b"v\0" as *const u8 as *const libc::c_char);
                i = 1 as libc::c_int;
                while i <= nvars {
                    if i & 7 as libc::c_int == 0 {
                        fputs(b"\nv\0" as *const u8 as *const libc::c_char, stdout);
                    }
                    lit = if *vals.offset(i as isize) < 0 as libc::c_int {
                        -i
                    } else {
                        i
                    };
                    printf(b" %d\0" as *const u8 as *const libc::c_char, lit);
                    i += 1;
                    i;
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            printf(b"v 0\n\0" as *const u8 as *const libc::c_char);
        }
    } else if res == 20 as libc::c_int {
        printf(b"s UNSATISFIABLE\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"c s UNKNOWN\n\0" as *const u8 as *const libc::c_char);
    }
    fflush(stdout);
    if verbose != 0 {
        i = 0 as libc::c_int;
        while i < nworkers {
            if !((*workers.offset(i as isize)).lgl).is_null() {
                printf(
                    b"c\nc ------------[worker %d statistics]------------ \nc\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                );
                lglstats((*workers.offset(i as isize)).lgl);
            }
            i += 1;
            i;
        }
        printf(
            b"c\nc -------------[overall statistics]------------- \nc\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        printf(b"c\n\0" as *const u8 as *const libc::c_char);
    }
    stats();
    if verbose >= 2 as libc::c_int {
        printf(b"c\n\0" as *const u8 as *const libc::c_char);
    }
    msg(
        -(1 as libc::c_int),
        2 as libc::c_int,
        b"releasing %d workers\0" as *const u8 as *const libc::c_char,
        nworkers,
    );
    i = 0 as libc::c_int;
    while i < nworkers {
        w = workers.offset(i as isize);
        if !((*w).lgl).is_null() {
            lglrelease((*w).lgl);
            msg(
                -(1 as libc::c_int),
                2 as libc::c_int,
                b"released worker %d\0" as *const u8 as *const libc::c_char,
                i,
            );
            if !((*w).dead).is_null() {
                deletecls((*w).dead);
                (*w).dead = std::ptr::null_mut::<Cls>();
            }
            let mut BYTES_2: size_t = ((*w).szcls as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
            decmem(BYTES_2);
            free((*w).cls as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    let mut BYTES_3: size_t =
        (nworkers as libc::c_ulong).wrapping_mul(::core::mem::size_of::<Worker>() as libc::c_ulong);
    decmem(BYTES_3);
    free(workers as *mut libc::c_void);
    let mut BYTES_4: size_t = ((nvars + 1 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    decmem(BYTES_4);
    free(fixed as *mut libc::c_void);
    if noeqs == 0 {
        let mut BYTES_5: size_t = ((nvars + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
        decmem(BYTES_5);
        free(repr as *mut libc::c_void);
    }
    let mut BYTES_6: size_t = ((nvars + 1 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    decmem(BYTES_6);
    free(vals as *mut libc::c_void);
    deleteallcls();
    res
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
