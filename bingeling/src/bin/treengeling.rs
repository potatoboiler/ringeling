#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, core_intrinsics, extern_types)]
extern crate libgeling;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type LGL;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::core::ffi::VaList) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn lglsetime(_: *mut LGL, time: Option<unsafe extern "C" fn() -> libc::c_double>);
    fn lglsetmsglock(
        _: *mut LGL,
        lock_0: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        unlock: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    );
    fn lglsetconsumecls(
        _: *mut LGL,
        consume: Option<
            unsafe extern "C" fn(*mut libc::c_void, *mut *mut libc::c_int, *mut libc::c_int) -> (),
        >,
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
    fn lglsetproduceunit(
        _: *mut LGL,
        produce: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
        _: *mut libc::c_void,
    );
    fn lglseterm(
        _: *mut LGL,
        term_0: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
        _: *mut libc::c_void,
    );
    fn lglprocesstime() -> libc::c_double;
    fn lglnvars(_: *mut LGL) -> libc::c_int;
    fn lglbytes(_: *mut LGL) -> size_t;
    fn lglgetprops(_: *mut LGL) -> int64_t;
    fn lglgetdecs(_: *mut LGL) -> int64_t;
    fn lglgetconfs(_: *mut LGL) -> int64_t;
    fn lglstats(_: *mut LGL);
    fn lglookahead(_: *mut LGL) -> libc::c_int;
    fn lglreducecache(_: *mut LGL);
    fn lglinconsistent(_: *mut LGL) -> libc::c_int;
    fn lglderef(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglsimp(_: *mut LGL, iterations: libc::c_int) -> libc::c_int;
    fn lglsat(_: *mut LGL) -> libc::c_int;
    fn lgladd(_: *mut LGL, lit: libc::c_int);
    fn lgldefopt(_: *mut LGL, _: *const libc::c_char) -> libc::c_int;
    fn lglgetopt(_: *mut LGL, _: *const libc::c_char) -> libc::c_int;
    fn lglsetopt(_: *mut LGL, _: *const libc::c_char, _: libc::c_int);
    fn lglsetprefix(_: *mut LGL, _: *const libc::c_char);
    fn lglbnr(name: *const libc::c_char, prefix: *const libc::c_char, file_0: *mut FILE);
    fn lglversion() -> *const libc::c_char;
    fn lgljoin(parent: *mut LGL, child: *mut LGL) -> libc::c_int;
    fn lglfork(parent: *mut LGL) -> *mut LGL;
    fn lglclone(_: *mut LGL) -> *mut LGL;
    fn lglminit(mem: *mut libc::c_void, _: lglalloc, _: lglrealloc, _: lgldealloc) -> *mut LGL;
    fn lglrelease(_: *mut LGL);
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn exit(_: libc::c_int) -> !;
    fn abort() -> !;
    fn free(_: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(__cond: *mut pthread_cond_t, __mutex: *mut pthread_mutex_t)
        -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
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
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: libc::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>;
pub type lglalloc = Option<unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void>;
pub type lgldealloc =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, size_t) -> ()>;
pub type lglrealloc = Option<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, size_t, size_t) -> *mut libc::c_void,
>;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
pub type __sighandler_t = Option<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub type State = libc::c_uint;
pub const SEARCH: State = 5;
pub const SPLIT: State = 4;
pub const LKHD: State = 3;
pub const SIMP: State = 2;
pub const READY: State = 1;
pub const FREE: State = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub state: State,
    pub pos: libc::c_int,
    pub lookahead: libc::c_int,
    pub depth: libc::c_int,
    pub res: libc::c_int,
    pub simplified: libc::c_int,
    pub consumed: libc::c_int,
    pub id: int64_t,
    pub decisions: int64_t,
    pub conflicts: int64_t,
    pub propagations: int64_t,
    pub cube: *mut libc::c_int,
    pub lgl: *mut LGL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Leaf {
    pub id: int64_t,
    pub next: *mut Leaf,
    pub prev: *mut Leaf,
    pub lits: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Parallel {
    pub decisions: int64_t,
    pub conflicts: int64_t,
    pub propagations: int64_t,
    pub consumed: C2RustUnnamed_2,
    pub produced: C2RustUnnamed_2,
    pub res: libc::c_int,
    pub nunits: libc::c_int,
    pub units: *mut libc::c_int,
    pub thread: pthread_t,
    pub lgl: *mut LGL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub leafs: int64_t,
    pub units: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Job {
    pub pos: libc::c_int,
    pub state: State,
    pub node: *mut Node,
    pub fun: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    pub thread: pthread_t,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Lock {
    pub mutex: pthread_mutex_t,
    pub locked: libc::c_int,
    pub waited: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub cnt: int64_t,
    pub lkhd: int64_t,
    pub split: int64_t,
    pub simp: int64_t,
    pub search: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub epoch: libc::c_double,
    pub simp: libc::c_double,
    pub lkhd: libc::c_double,
    pub split: libc::c_double,
    pub search: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub set: int64_t,
    pub def: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub num: libc::c_int,
    pub max: libc::c_int,
    pub count: int64_t,
    pub first: *mut Leaf,
    pub last: *mut Leaf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub z: libc::c_uint,
    pub w: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub confs: Lock,
    pub done: Lock,
    pub leafs: Lock,
    pub mem: Lock,
    pub msg: Lock,
    pub nodes: Lock,
    pub opts: Lock,
    pub parleafs: Lock,
    pub parstats: Lock,
    pub parunits: Lock,
    pub simplified: Lock,
    pub stats: Lock,
    pub workers: Lock,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Opt {
    pub name: *const libc::c_char,
    pub val: libc::c_int,
}
#[inline]
unsafe extern "C" fn vprintf(
    mut __fmt: *const libc::c_char,
    mut __arg: ::core::ffi::VaList,
) -> libc::c_int {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char) -> libc::c_longlong {
    return strtoll(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
static mut verbose: libc::c_int = 0;
static mut balance: libc::c_int = 0;
static mut showstats: libc::c_int = 0;
static mut nowitness: libc::c_int = 0;
static mut ncores: libc::c_int = 0;
static mut randswap: libc::c_int = 0;
static mut treelookdepth: libc::c_int = 0;
static mut treelookdepthset: libc::c_int = 0;
static mut locslkhd: libc::c_int = 0;
static mut relevancelkhd: libc::c_int = 1 as libc::c_int;
static mut reducecache: libc::c_int = 0;
static mut nosimp: libc::c_int = 0;
static mut forcesimp: libc::c_int = 0;
static mut nosearch: libc::c_int = 0;
static mut noparallel: libc::c_int = 0;
static mut fullint: libc::c_int = 10 as libc::c_int;
static mut asymmetric: libc::c_int = 1 as libc::c_int;
static mut eager: libc::c_int = 1 as libc::c_int;
static mut splitsuccessful: libc::c_int = 1 as libc::c_int;
static mut branches: libc::c_int = -(1 as libc::c_int);
static mut portfolio: libc::c_int = 0 as libc::c_int;
static mut optimize: libc::c_int = -(1 as libc::c_int);
static mut clim: libc::c_int = 0;
static mut newclim: libc::c_int = 0;
static mut forcedclim: libc::c_int = 0;
static mut thisclim: libc::c_int = 0;
static mut initclim: libc::c_int = 0;
static mut maxclim: libc::c_int = 0;
static mut minclim: libc::c_int = 0;
static mut nvars: libc::c_int = 0;
static mut nclauses: libc::c_int = 0;
static mut root: *mut LGL = 0 as *const LGL as *mut LGL;
static mut nodes: *mut *mut Node = 0 as *const *mut Node as *mut *mut Node;
static mut numnodes: libc::c_int = 0;
static mut maxnumnodes: libc::c_int = 0;
static mut sizenodes: libc::c_int = 0;
static mut rootconsumed: libc::c_int = 0;
static mut parallel: Parallel = Parallel {
    decisions: 0,
    conflicts: 0,
    propagations: 0,
    consumed: C2RustUnnamed_2 { leafs: 0, units: 0 },
    produced: C2RustUnnamed_2 { leafs: 0, units: 0 },
    res: 0,
    nunits: 0,
    units: 0 as *const libc::c_int as *mut libc::c_int,
    thread: 0,
    lgl: 0 as *const LGL as *mut LGL,
};
static mut nparallel: libc::c_int = 0;
static mut maxactive: libc::c_int = 0;
static mut firstosplit: libc::c_int = 0;
static mut numtosplit: libc::c_int = 0;
static mut lastosplit: libc::c_int = 0;
static mut maxworkers: libc::c_int = 0;
static mut maxworkers2: libc::c_int = 0;
static mut numworkers: libc::c_int = 0;
static mut maxnumworkers: libc::c_int = 0;
static mut jobs: *mut *mut Job = 0 as *const *mut Job as *mut *mut Job;
static mut numjobs: libc::c_int = 0;
static mut sizejobs: libc::c_int = 0;
static mut js: C2RustUnnamed_3 = C2RustUnnamed_3 {
    cnt: 0,
    lkhd: 0,
    split: 0,
    simp: 0,
    search: 0,
};
static mut totalkhd: int64_t = 0;
static mut treelkhd: int64_t = 0;
static mut confstack: *mut int64_t = 0 as *const int64_t as *mut int64_t;
static mut numconfstack: libc::c_int = 0;
static mut sizeconfstack: libc::c_int = 0;
static mut fname: *const libc::c_char = 0 as *const libc::c_char;
static mut file: *mut FILE = 0 as *const FILE as *mut FILE;
static mut lineno: libc::c_int = 0;
#[no_mangle]
pub static mut maxbytes: size_t = 0;
#[no_mangle]
pub static mut hardlimbytes: size_t = 0;
#[no_mangle]
pub static mut softlimbytes: size_t = 0;
#[no_mangle]
pub static mut splitlimbytes: size_t = 0;
#[no_mangle]
pub static mut currentbytes: size_t = 0;
static mut ids: int64_t = 0;
static mut threads: int64_t = 0;
static mut conflicts: int64_t = 0;
static mut decisions: int64_t = 0;
static mut propagations: int64_t = 0;
static mut sumclims: int64_t = 0;
static mut inclims: int64_t = 0;
static mut declims: int64_t = 0;
static mut forcedclims: int64_t = 0;
static mut sumsimplified: int64_t = 0;
static mut wct: C2RustUnnamed_4 = C2RustUnnamed_4 {
    epoch: 0.,
    simp: 0.,
    lkhd: 0.,
    split: 0.,
    search: 0.,
};
static mut round: libc::c_int = 0;
static mut started: libc::c_int = 0;
static mut deleted: libc::c_int = 0;
static mut simplified: libc::c_int = 0;
static mut added: libc::c_int = 0;
static mut startimeptr: *mut libc::c_double = 0 as *const libc::c_double as *mut libc::c_double;
static mut startime: libc::c_double = 0.;
static mut opts: C2RustUnnamed_5 = C2RustUnnamed_5 { set: 0, def: 0 };
#[no_mangle]
pub static mut leafs: C2RustUnnamed_6 = C2RustUnnamed_6 {
    num: 0,
    max: 0,
    count: 0,
    first: 0 as *const Leaf as *mut Leaf,
    last: 0 as *const Leaf as *mut Leaf,
};
static mut done: libc::c_int = 0;
static mut stop: libc::c_int = 0;
static mut rng: C2RustUnnamed_7 = C2RustUnnamed_7 { z: 0, w: 0 };
static mut lock: C2RustUnnamed_8 = C2RustUnnamed_8 {
    confs: Lock {
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                },
            },
        },
        locked: 0,
        waited: 0,
    },
    done: Lock {
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                },
            },
        },
        locked: 0,
        waited: 0,
    },
    leafs: Lock {
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                },
            },
        },
        locked: 0,
        waited: 0,
    },
    mem: Lock {
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                },
            },
        },
        locked: 0,
        waited: 0,
    },
    msg: Lock {
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                },
            },
        },
        locked: 0,
        waited: 0,
    },
    nodes: Lock {
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                },
            },
        },
        locked: 0,
        waited: 0,
    },
    opts: Lock {
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                },
            },
        },
        locked: 0,
        waited: 0,
    },
    parleafs: Lock {
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                },
            },
        },
        locked: 0,
        waited: 0,
    },
    parstats: Lock {
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                },
            },
        },
        locked: 0,
        waited: 0,
    },
    parunits: Lock {
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                },
            },
        },
        locked: 0,
        waited: 0,
    },
    simplified: Lock {
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                },
            },
        },
        locked: 0,
        waited: 0,
    },
    stats: Lock {
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                },
            },
        },
        locked: 0,
        waited: 0,
    },
    workers: Lock {
        mutex: pthread_mutex_t {
            __data: __pthread_mutex_s {
                __lock: 0,
                __count: 0,
                __owner: 0,
                __nusers: 0,
                __kind: 0,
                __spins: 0,
                __elision: 0,
                __list: __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                },
            },
        },
        locked: 0,
        waited: 0,
    },
};
static mut workerscond: pthread_cond_t = pthread_cond_t {
    __data: __pthread_cond_s {
        __wseq: __atomic_wide_counter { __value64: 0 },
        __g1_start: __atomic_wide_counter { __value64: 0 },
        __g_refs: [0; 2],
        __g_size: [0; 2],
        __g1_orig_size: 0,
        __wrefs: 0,
        __g_signals: [0; 2],
    },
};
unsafe extern "C" fn currentime() -> libc::c_double {
    let mut res: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    if gettimeofday(&mut tv, 0 as *mut libc::c_void) == 0 {
        res = 1e-6f64 * tv.tv_usec as libc::c_double;
        res += tv.tv_sec as libc::c_double;
    }
    return res;
}
unsafe extern "C" fn getime() -> libc::c_double {
    return currentime() - wct.epoch;
}
unsafe extern "C" fn warn(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    fputs(
        b"c *** warning *** \0" as *const u8 as *const libc::c_char,
        stdout,
    );
    ap = args.clone();
    vprintf(fmt, ap.as_va_list());
    fputc('\n' as i32, stdout);
    fflush(stdout);
}
unsafe extern "C" fn startimer(mut timptr: *mut libc::c_double) {
    startimeptr = timptr;
    startime = currentime();
    started = 1 as libc::c_int;
}
unsafe extern "C" fn deltatime(mut start: libc::c_double) -> libc::c_double {
    let mut res: libc::c_double = currentime() - start;
    if res < 0 as libc::c_int as libc::c_double {
        res = -res;
    }
    return res;
}
unsafe extern "C" fn stoptimer() -> libc::c_double {
    let mut ptr: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut res: libc::c_double = deltatime(startime);
    started = 0 as libc::c_int;
    ptr = startimeptr;
    if !ptr.is_null() {
        *ptr += res;
    }
    startimeptr = 0 as *mut libc::c_double;
    return res;
}
unsafe extern "C" fn lockgen(mut lock_0: *mut Lock, mut name: *const libc::c_char) {
    if pthread_mutex_lock(&mut (*lock_0).mutex) != 0 {
        warn(
            b"failed to lock '%s' mutex\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
    (*lock_0).locked += 1;
    (*lock_0).locked;
}
unsafe extern "C" fn unlockgen(mut lock_0: *mut Lock, mut name: *const libc::c_char) {
    (*lock_0).locked -= 1;
    (*lock_0).locked;
    if pthread_mutex_unlock(&mut (*lock_0).mutex) != 0 {
        warn(
            b"failed to lock '%s' mutex\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
}
unsafe extern "C" fn lockconfs() {
    lockgen(
        &mut lock.confs,
        b"confs\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn lockdone() {
    lockgen(
        &mut lock.done,
        b"done\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn lockleafs() {
    lockgen(
        &mut lock.leafs,
        b"leafs\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn lockmem() {
    lockgen(&mut lock.mem, b"mem\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn lockmsg() {
    lockgen(&mut lock.msg, b"msg\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn locknodes() {
    lockgen(
        &mut lock.nodes,
        b"nodes\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn lockopts() {
    lockgen(
        &mut lock.opts,
        b"opts\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn lockparleafs() {
    lockgen(
        &mut lock.parleafs,
        b"parleafs\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn lockparstats() {
    lockgen(
        &mut lock.parstats,
        b"parstats\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn lockparunits() {
    lockgen(
        &mut lock.parunits,
        b"parunits\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn locksimplified() {
    lockgen(
        &mut lock.simplified,
        b"simplified\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn lockstats() {
    lockgen(
        &mut lock.stats,
        b"stats\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn lockworkers() {
    lockgen(
        &mut lock.workers,
        b"workers\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn unlockconfs() {
    unlockgen(
        &mut lock.confs,
        b"confs\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn unlockdone() {
    unlockgen(
        &mut lock.done,
        b"done\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn unlockleafs() {
    unlockgen(
        &mut lock.leafs,
        b"leafs\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn unlockmem() {
    unlockgen(&mut lock.mem, b"mem\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn unlockmsg() {
    unlockgen(&mut lock.msg, b"msg\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn unlocknodes() {
    unlockgen(
        &mut lock.nodes,
        b"nodes\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn unlockopts() {
    unlockgen(
        &mut lock.opts,
        b"opts\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn unlockparleafs() {
    unlockgen(
        &mut lock.parleafs,
        b"parleafs\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn unlockparstats() {
    unlockgen(
        &mut lock.parstats,
        b"parstats\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn unlockparunits() {
    unlockgen(
        &mut lock.parunits,
        b"parunits\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn unlocksimplified() {
    unlockgen(
        &mut lock.simplified,
        b"simplified\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn unlockstats() {
    unlockgen(
        &mut lock.stats,
        b"stats\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn unlockworkers() {
    unlockgen(
        &mut lock.workers,
        b"workers\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn err(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    lockmsg();
    fputs(b"c *** \0" as *const u8 as *const libc::c_char, stdout);
    ap = args.clone();
    vprintf(fmt, ap.as_va_list());
    fputc('\n' as i32, stdout);
    fflush(stdout);
    unlockmsg();
    exit(1 as libc::c_int);
}
unsafe extern "C" fn smsg() {
    let mut t: libc::c_double = getime();
    let mut m: libc::c_double = 0.;
    lockmem();
    m = currentbytes as libc::c_double
        / ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_double;
    unlockmem();
    printf(
        b"(%.1f %d %lld %d %d %.0f) \0" as *const u8 as *const libc::c_char,
        t,
        round,
        ids as libc::c_longlong,
        numnodes,
        clim,
        m,
    );
}
unsafe extern "C" fn msg(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    lockmsg();
    fputs(b"c \0" as *const u8 as *const libc::c_char, stdout);
    ap = args.clone();
    vprintf(fmt, ap.as_va_list());
    fputc('\n' as i32, stdout);
    fflush(stdout);
    unlockmsg();
}
unsafe extern "C" fn vrb(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    if verbose == 0 {
        return;
    }
    lockmsg();
    fputs(b"c \0" as *const u8 as *const libc::c_char, stdout);
    smsg();
    ap = args.clone();
    vprintf(fmt, ap.as_va_list());
    fputc('\n' as i32, stdout);
    fflush(stdout);
    unlockmsg();
}
unsafe extern "C" fn nmsg(mut node: *mut Node, mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    if verbose == 0 {
        return;
    }
    lockmsg();
    printf(
        b"c [%d %lld] \0" as *const u8 as *const libc::c_char,
        (*node).depth,
        (*node).id as libc::c_longlong,
    );
    smsg();
    ap = args.clone();
    vprintf(fmt, ap.as_va_list());
    fputc('\n' as i32, stdout);
    fflush(stdout);
    unlockmsg();
}
unsafe extern "C" fn jmsg(mut job: *mut Job, mut msg_0: *const libc::c_char) {
    let mut node: *mut Node = (*job).node;
    if verbose == 0 {
        return;
    }
    lockmsg();
    printf(
        b"c [%d %lld] \0" as *const u8 as *const libc::c_char,
        (*node).depth,
        (*node).id as libc::c_longlong,
    );
    smsg();
    printf(
        b"%s %s job %d\n\0" as *const u8 as *const libc::c_char,
        msg_0,
        (*job).name,
        (*job).pos,
    );
    fflush(stdout);
    unlockmsg();
}
unsafe extern "C" fn mmsg(mut msg_0: *const libc::c_char, mut node: *mut Node) {
    if verbose == 0 {
        return;
    }
    lockmsg();
    printf(b"c \0" as *const u8 as *const libc::c_char);
    smsg();
    printf(
        b"%s [%d %lld]\n\0" as *const u8 as *const libc::c_char,
        msg_0,
        (*node).depth,
        (*node).id as libc::c_longlong,
    );
    fflush(stdout);
    unlockmsg();
}
unsafe extern "C" fn perr(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    printf(
        b"c *** parse error in '%s' at line %d: \0" as *const u8 as *const libc::c_char,
        fname,
        lineno,
    );
    ap = args.clone();
    vprintf(fmt, ap.as_va_list());
    fputc('\n' as i32, stdout);
    fflush(stdout);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn skipnode(mut node: *mut Node) -> libc::c_int {
    if lglinconsistent((*node).lgl) != 0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn incround() {
    round += 1;
    round;
    vrb(b"\0" as *const u8 as *const libc::c_char);
    vrb(
        b"=================== [ round %d ] ===================\0" as *const u8
            as *const libc::c_char,
        round,
    );
    vrb(b"\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn startphase(mut phase: *const libc::c_char) {
    if verbose == 0 {
        return;
    }
    vrb(b"\0" as *const u8 as *const libc::c_char);
    vrb(
        b"------------------- [ %s %d ] -------------------\0" as *const u8 as *const libc::c_char,
        phase,
        round,
    );
    vrb(b"\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn avg(mut a: libc::c_double, mut b: libc::c_double) -> libc::c_double {
    return if b > 0 as libc::c_int as libc::c_double {
        a / b
    } else {
        0.0f64
    };
}
unsafe extern "C" fn pcnt(mut a: libc::c_double, mut b: libc::c_double) -> libc::c_double {
    return avg(100.0f64 * a, b);
}
unsafe extern "C" fn incmem(mut bytes: size_t) {
    lockmem();
    currentbytes = currentbytes.wrapping_add(bytes);
    if currentbytes > maxbytes {
        maxbytes = currentbytes;
    }
    unlockmem();
}
unsafe extern "C" fn decmem(mut bytes: size_t) {
    lockmem();
    currentbytes = currentbytes.wrapping_sub(bytes);
    unlockmem();
}
unsafe extern "C" fn alloc(mut dummy: *mut libc::c_void, mut bytes: size_t) -> *mut libc::c_void {
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut BYTES: size_t =
        bytes.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong);
    res = malloc(BYTES) as *mut libc::c_char;
    if res.is_null() {
        err(b"out of memory\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memset(res as *mut libc::c_void, 0 as libc::c_int, BYTES);
    incmem(BYTES);
    return res as *mut libc::c_void;
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
    lockmem();
    currentbytes = currentbytes.wrapping_sub(old_bytes);
    currentbytes = currentbytes.wrapping_add(new_bytes);
    if currentbytes > maxbytes {
        maxbytes = currentbytes;
    }
    unlockmem();
    return realloc(ptr, new_bytes);
}
unsafe extern "C" fn getotalmem(mut explain: libc::c_int) -> int64_t {
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
                b"%lld KB total memory according to '/proc/meminfo'\0" as *const u8
                    as *const libc::c_char,
                res,
            );
        }
        res <<= 10 as libc::c_int;
    } else {
        res = (12 as libc::c_longlong) << 30 as libc::c_int;
        if explain != 0 {
            msg(
                b"assuming compiled in memory size of %d GB\0" as *const u8 as *const libc::c_char,
                12 as libc::c_longlong,
            );
        }
    }
    if !p.is_null() {
        pclose(p);
    }
    return res as int64_t;
}
unsafe extern "C" fn getcores(mut explain: libc::c_int) -> libc::c_int {
    let mut syscores: libc::c_int = 0;
    let mut coreids: libc::c_int = 0;
    let mut physids: libc::c_int = 0;
    let mut procpuinfocores: libc::c_int = 0;
    let mut usesyscores: libc::c_int = 0;
    let mut useprocpuinfo: libc::c_int = 0;
    let mut amd: libc::c_int = 0;
    let mut intel: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut p: *mut FILE = 0 as *mut FILE;
    syscores = sysconf(_SC_NPROCESSORS_ONLN as libc::c_int) as libc::c_int;
    if explain != 0 {
        if syscores > 0 as libc::c_int {
            msg(
                b"'sysconf' reports %d processors online\0" as *const u8 as *const libc::c_char,
                syscores,
            );
        } else {
            msg(
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
                    b"found %d unique core ids in '/proc/cpuinfo'\0" as *const u8
                        as *const libc::c_char,
                    coreids,
                );
            } else {
                msg(
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
                    b"found %d unique physical ids in '/proc/cpuinfo'\0" as *const u8
                        as *const libc::c_char,
                    physids,
                );
            } else {
                msg(
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
                b"'sysconf' and '/proc/cpuinfo' results match\0" as *const u8
                    as *const libc::c_char,
            );
        }
        usesyscores = 1 as libc::c_int;
    } else if procpuinfocores > 0 as libc::c_int && syscores <= 0 as libc::c_int {
        if explain != 0 {
            msg(b"only '/proc/cpuinfo' result valid\0" as *const u8 as *const libc::c_char);
        }
        useprocpuinfo = 1 as libc::c_int;
    } else if procpuinfocores <= 0 as libc::c_int && syscores > 0 as libc::c_int {
        if explain != 0 {
            msg(b"only 'sysconf' result valid\0" as *const u8 as *const libc::c_char);
        }
        usesyscores = 1 as libc::c_int;
    } else if procpuinfocores > 0 as libc::c_int && syscores > 0 as libc::c_int {
        intel = (system(
            b"grep vendor /proc/cpuinfo 2>/dev/null|grep -q Intel\0" as *const u8
                as *const libc::c_char,
        ) == 0) as libc::c_int;
        if intel != 0 && explain != 0 {
            msg(b"found Intel as vendor in '/proc/cpuinfo'\0" as *const u8 as *const libc::c_char);
        }
        amd = (system(
            b"grep vendor /proc/cpuinfo 2>/dev/null|grep -q AMD\0" as *const u8
                as *const libc::c_char,
        ) == 0) as libc::c_int;
        if amd != 0 && explain != 0 {
            msg(b"found AMD as vendor in '/proc/cpuinfo'\0" as *const u8 as *const libc::c_char);
        }
        if amd != 0 {
            if explain != 0 {
                msg(b"trusting 'sysconf' on AMD\0" as *const u8 as *const libc::c_char);
            }
            usesyscores = 1 as libc::c_int;
        } else if intel != 0 {
            if explain != 0 {
                msg(
                    b"'sysconf' result off by a factor of %f on Intel\0" as *const u8
                        as *const libc::c_char,
                    syscores as libc::c_double / procpuinfocores as libc::c_double,
                );
                msg(
                    b"trusting 'sysconf' on Intel (assuming HyperThreading)\0" as *const u8
                        as *const libc::c_char,
                );
            }
            usesyscores = 1 as libc::c_int;
        } else {
            if explain != 0 {
                msg(
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
                b"assuming cores = core * physical ids in '/proc/cpuinfo' = %d\0" as *const u8
                    as *const libc::c_char,
                procpuinfocores,
            );
        }
        res = procpuinfocores;
    } else if usesyscores != 0 {
        if explain != 0 {
            msg(
                b"assuming cores = number of processors reported by 'sysconf' = %d\0" as *const u8
                    as *const libc::c_char,
                syscores,
            );
        }
        res = syscores;
    } else {
        if explain != 0 {
            msg(
                b"using compiled in default value of assumed %d cores\0" as *const u8
                    as *const libc::c_char,
                8 as libc::c_int,
            );
        }
        res = 8 as libc::c_int;
    }
    return res;
}
unsafe extern "C" fn usage() {
    let mut b: int64_t = getotalmem(0 as libc::c_int);
    let mut m: libc::c_longlong = (b + ((1 as libc::c_int) << 20 as libc::c_int) as int64_t
        - 1 as libc::c_int as int64_t
        >> 20 as libc::c_int) as libc::c_longlong;
    let mut g: libc::c_longlong = (b + ((1 as libc::c_int) << 30 as libc::c_int) as int64_t
        - 1 as libc::c_int as int64_t
        >> 30 as libc::c_int) as libc::c_longlong;
    let mut c: libc::c_int = getcores(0 as libc::c_int);
    printf(
        b"usage: treengeling [<option> ...] [<file> [<workers>]]\n\nwhere <option> is one of the following\n\n  -h             print option summary\n  --version      print version and exit\n  -v             increase verbose level\n  -S             print statistics for each solver instance too\n  -n             do not print satisfying assignments\n\n  -t <workers>   maximum number actual worker threads (system default %d)\n  -a <nodes>     maximum number active nodes (system default %d)\n\n  -m <mb>        assumed memory in mega bytes (system default %lld MB)\n  -g <gb>        assumed memory in giga bytes (system default %lld GB)\n\n  -r <posnum>    randomize splits by swapping <posnum> nodes\n  -s <seed>      unsigned 64 bit seed for randomizing splits (default 0)\n\n  -b <branches>  percentage of nodes split (default %d%%)\n\n  --balance      split larger nodes first\n  --symmetric    symmetric splitting%s\n  --asymmetric   asymmetric splitting%s\n  --eager        eager splitting by forced reduction of limit (default)\n  --lazy         disable eager splitting (opposite of '--eager')\n  --portfolio    use portfolio style option fuzzing (default off)\n\n  --locslkhd      use local searchf or look-ahead\n  --no-relevancelkhd do not use relevance (VSIDS/VMTF) look-ahead\n  --treelook=<d>  maximum depth for tree-based look-ahead (default %d)\n\n  --min=<lim>    minimum conflict limit per search (compiled default %d)\n  --init=<lim>   initial conflict limit per search (compiled default %d)\n  --max=<lim>    maximum conflict limit per search (compiled default %d)\n\n  --reduce       reduce learned clause cache for all right branches\n  --force-simp   force simplification even after light simplification\n  --no-simp      do not explicitly simplify in each round\n  --no-search    do not even search in each round\n  --no-parallel  disable additional parallel solver instance\n  --no-full      no full rounds every %d rounds\n  -f <fullint>   full round interval (default %d)\n\nand the <file> is a DIMACS file.  If the name of the file has a '.gz'\nrespectively '.bz2' suffix, it is assumed to be a file compressed with\n'gzip' respectively 'bzip2'.  In this case the parser will open a pipe\nand execute 'gunzip' respectively 'bzcat'.\n\0"
            as *const u8 as *const libc::c_char,
        c,
        8 as libc::c_int * c,
        m,
        g,
        50 as libc::c_int,
        if 1 as libc::c_int != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b" (default)\0" as *const u8 as *const libc::c_char
        },
        if 1 as libc::c_int != 0 {
            b" (default)\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        10 as libc::c_int,
        1000 as libc::c_int,
        10000 as libc::c_int,
        100000 as libc::c_int,
        10 as libc::c_int,
        10 as libc::c_int,
    );
    fflush(stdout);
    exit(0 as libc::c_int);
}
unsafe extern "C" fn version() {
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, lglversion());
    exit(0 as libc::c_int);
}
unsafe extern "C" fn next() -> libc::c_int {
    let mut res: libc::c_int = getc(file);
    if res == '\n' as i32 {
        lineno += 1;
        lineno;
    }
    return res;
}
unsafe extern "C" fn ws(mut ch: libc::c_int) -> libc::c_int {
    return (ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 || ch == '\r' as i32)
        as libc::c_int;
}
unsafe extern "C" fn parse(mut lgl: *mut LGL) {
    let mut sclauses: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut lit: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    let mut nlits: libc::c_int = 0;
    lineno = 1 as libc::c_int;
    loop {
        ch = next();
        if ch == -(1 as libc::c_int) {
            perr(b"unexpected end-of-file before header\0" as *const u8 as *const libc::c_char);
        }
        if !(ch == 'c' as i32) {
            break;
        }
        loop {
            ch = next();
            if !(ch != '\n' as i32) {
                break;
            }
            if ch == -(1 as libc::c_int) {
                perr(
                    b"unexpected end-of-file in header comment\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
    }
    if ch != 'p' as i32 {
        perr(
            b"unexpected character 0x%02x in header\0" as *const u8 as *const libc::c_char,
            ch,
        );
    }
    if fscanf(
        file,
        b" cnf %d %d\0" as *const u8 as *const libc::c_char,
        &mut nvars as *mut libc::c_int,
        &mut sclauses as *mut libc::c_int,
    ) != 2 as libc::c_int
        || nvars < 0 as libc::c_int
        || sclauses < 0 as libc::c_int
    {
        perr(b"invalid header\0" as *const u8 as *const libc::c_char);
    }
    msg(
        b"found 'p cnf %d %d' header\0" as *const u8 as *const libc::c_char,
        nvars,
        sclauses,
    );
    last = 0 as libc::c_int;
    nlits = last;
    loop {
        ch = next();
        if ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 || ch == '\r' as i32 {
            continue;
        }
        if ch == -(1 as libc::c_int) {
            if last != 0 {
                perr(
                    b"zero missing after %d in last clause\0" as *const u8 as *const libc::c_char,
                    last,
                );
            }
            if nclauses < sclauses {
                perr(
                    b"%d clauses missing\0" as *const u8 as *const libc::c_char,
                    sclauses - nclauses,
                );
            }
            msg(
                b"parsed %d literals in %d clauses in %.2f seconds\0" as *const u8
                    as *const libc::c_char,
                nlits,
                nclauses,
                getime(),
            );
            return;
        }
        if ch == 'c' as i32 {
            loop {
                ch = next();
                if !(ch != '\n' as i32) {
                    break;
                }
                if ch == -(1 as libc::c_int) {
                    perr(
                        b"unexpected end-of-file in body comment\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
        } else {
            sign = 1 as libc::c_int;
            if ch == '-' as i32 {
                ch = next();
                if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                    == 0
                {
                    perr(b"expected digit after '-'\0" as *const u8 as *const libc::c_char);
                }
                sign = -(1 as libc::c_int);
            } else if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                == 0
            {
                perr(b"expected digit or '-'\0" as *const u8 as *const libc::c_char);
            }
            lit = ch - '0' as i32;
            loop {
                ch = next();
                if !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                    != 0)
                {
                    break;
                }
                lit = 10 as libc::c_int * lit + (ch - '0' as i32);
            }
            if lit > nvars {
                perr(
                    b"variable %d exceeds maximum %d\0" as *const u8 as *const libc::c_char,
                    lit,
                    nvars,
                );
            }
            if nclauses == sclauses {
                perr(b"too many clauses\0" as *const u8 as *const libc::c_char);
            }
            if lit != 0 {
                lit *= sign;
                nlits += 1;
                nlits;
            } else {
                nclauses += 1;
                nclauses;
            }
            if ch != -(1 as libc::c_int) && ws(ch) == 0 {
                perr(
                    b"expected white space after %d\0" as *const u8 as *const libc::c_char,
                    lit,
                );
            }
            lgladd(lgl, lit);
            last = lit;
        }
    }
}
unsafe extern "C" fn isnum(mut str: *const libc::c_char) -> libc::c_int {
    let mut p: *const libc::c_char = str;
    if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        == 0
    {
        return 0 as libc::c_int;
    }
    loop {
        p = p.offset(1);
        if !(*p != 0) {
            break;
        }
        if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn exists(mut str: *const libc::c_char) -> libc::c_int {
    let mut buf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    return (stat(str, &mut buf) == 0) as libc::c_int;
}
unsafe extern "C" fn term(mut dummy: *mut libc::c_void) -> libc::c_int {
    let mut res: libc::c_int = 0;
    lockdone();
    res = (done != 0 || stop != 0) as libc::c_int;
    unlockdone();
    return res;
}
unsafe extern "C" fn produceunit(mut voidptr: *mut libc::c_void, mut lit: libc::c_int) {
    lockparunits();
    let fresh0 = parallel.nunits;
    parallel.nunits = parallel.nunits + 1;
    *(parallel.units).offset(fresh0 as isize) = lit;
    parallel.produced.units += 1;
    parallel.produced.units;
    unlockparunits();
}
unsafe extern "C" fn consumeunits(
    mut voidptr: *mut libc::c_void,
    mut fromptr: *mut *mut libc::c_int,
    mut toptr: *mut *mut libc::c_int,
) {
    let mut consumedptr: *mut libc::c_int = voidptr as *mut libc::c_int;
    let mut produced: libc::c_int = 0;
    let mut consumed: libc::c_int = 0;
    lockparunits();
    produced = parallel.nunits;
    consumed = produced - *consumedptr;
    parallel.consumed.units += consumed as int64_t;
    unlockparunits();
    *fromptr = (parallel.units).offset(*consumedptr as isize);
    *toptr = (parallel.units).offset(produced as isize);
    *consumedptr = produced;
}
unsafe extern "C" fn intslen(mut ints: *const libc::c_int) -> libc::c_int {
    let mut p: *const libc::c_int = 0 as *const libc::c_int;
    p = ints;
    while *p != 0 {
        p = p.offset(1);
        p;
    }
    return p.offset_from(ints) as libc::c_long as libc::c_int;
}
unsafe extern "C" fn appendint(mut ints: *mut libc::c_int, mut i: libc::c_int) -> *mut libc::c_int {
    let mut len: libc::c_int = 0;
    let mut res: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut q: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut other: libc::c_int = 0;
    let mut p: *const libc::c_int = 0 as *const libc::c_int;
    len = intslen(ints) + 1 as libc::c_int;
    let mut BYTES: size_t = ((len + 1 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    res = malloc(BYTES) as *mut libc::c_int;
    if res.is_null() {
        err(b"out of memory\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memset(res as *mut libc::c_void, 0 as libc::c_int, BYTES);
    incmem(BYTES);
    q = res;
    p = ints;
    loop {
        other = *p;
        if !(other != 0) {
            break;
        }
        let fresh1 = q;
        q = q.offset(1);
        *fresh1 = other;
        p = p.offset(1);
        p;
    }
    let fresh2 = q;
    q = q.offset(1);
    *fresh2 = i;
    let fresh3 = q;
    q = q.offset(1);
    *fresh3 = 0 as libc::c_int;
    return res;
}
unsafe extern "C" fn addint(mut intsptr: *mut *mut libc::c_int, mut i: libc::c_int) {
    let mut oldints: *mut libc::c_int = *intsptr;
    let mut len: libc::c_int = intslen(oldints);
    let mut p: *const libc::c_int = 0 as *const libc::c_int;
    let mut q: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut o: libc::c_int = 0;
    let mut newints: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut BYTES: size_t = ((len + 2 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    newints = malloc(BYTES) as *mut libc::c_int;
    if newints.is_null() {
        err(b"out of memory\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memset(newints as *mut libc::c_void, 0 as libc::c_int, BYTES);
    incmem(BYTES);
    q = newints;
    p = oldints;
    loop {
        o = *p;
        if !(o != 0) {
            break;
        }
        let fresh4 = q;
        q = q.offset(1);
        *fresh4 = o;
        p = p.offset(1);
        p;
    }
    let fresh5 = q;
    q = q.offset(1);
    *fresh5 = i;
    let mut BYTES_0: size_t = ((len + 1 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    decmem(BYTES_0);
    free(oldints as *mut libc::c_void);
    *intsptr = newints;
}
unsafe extern "C" fn cubemsg(mut node: *mut Node, mut str: *const libc::c_char) {
    let mut p: *const libc::c_int = 0 as *const libc::c_int;
    if verbose == 0 {
        return;
    }
    lockmsg();
    printf(
        b"c [%d %lld] \0" as *const u8 as *const libc::c_char,
        (*node).depth,
        (*node).id as libc::c_longlong,
    );
    smsg();
    fputs(str, stdout);
    if !((*node).cube).is_null() {
        p = (*node).cube;
        while *p != 0 {
            printf(b" %d\0" as *const u8 as *const libc::c_char, *p);
            p = p.offset(1);
            p;
        }
        fputs(b" 0\0" as *const u8 as *const libc::c_char, stdout);
    } else {
        fputs(
            b" <cube-already-deleted>\0" as *const u8 as *const libc::c_char,
            stdout,
        );
    }
    fputc('\n' as i32, stdout);
    fflush(stdout);
    unlockmsg();
}
unsafe extern "C" fn initroot() {
    msg(b"initializing root solver instance\0" as *const u8 as *const libc::c_char);
    root = lglminit(
        0 as *mut libc::c_void,
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
        root,
        b"druplig\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    lglsetopt(
        root,
        b"classify\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    if verbose != 0 {
        lglsetopt(
            root,
            b"verbose\0" as *const u8 as *const libc::c_char,
            verbose,
        );
    } else if showstats == 0 {
        lglsetopt(
            root,
            b"profile\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    lglsetopt(
        root,
        b"abstime\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    lglsetopt(
        root,
        b"trep\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    lglsetopt(
        root,
        b"compact\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    if noparallel == 0 {
        lglsetopt(
            root,
            b"bca\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        lglseterm(
            root,
            Some(term as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int),
            0 as *mut libc::c_void,
        );
        lglsetconsumeunits(
            root,
            Some(
                consumeunits
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut libc::c_int,
                        *mut *mut libc::c_int,
                    ) -> (),
            ),
            &mut rootconsumed as *mut libc::c_int as *mut libc::c_void,
        );
        lglsetmsglock(
            root,
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            >(Some(::core::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(lockmsg))),
            ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            >(Some(::core::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(unlockmsg))),
            0 as *mut libc::c_void,
        );
    }
    lglsetime(
        root,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> libc::c_double>,
            Option<unsafe extern "C" fn() -> libc::c_double>,
        >(Some(::core::mem::transmute::<
            unsafe extern "C" fn() -> libc::c_double,
            unsafe extern "C" fn() -> libc::c_double,
        >(getime))),
    );
    lglsetprefix(root, b"c (root) \0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn newnode(mut parent: *mut Node, mut decision: libc::c_int) -> *mut Node {
    let mut prefix: [libc::c_char; 80] = [0; 80];
    let mut res: *mut Node = 0 as *mut Node;
    locknodes();
    let mut BYTES: size_t = (1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<Node>() as libc::c_ulong);
    res = malloc(BYTES) as *mut Node;
    if res.is_null() {
        err(b"out of memory\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memset(res as *mut libc::c_void, 0 as libc::c_int, BYTES);
    incmem(BYTES);
    (*res).state = READY;
    if !parent.is_null() {
        (*res).depth = (*parent).depth + 1 as libc::c_int;
    }
    let fresh6 = ids;
    ids = ids + 1;
    (*res).id = fresh6;
    (*res).pos = numnodes;
    if sizenodes == numnodes {
        let mut NEW_SIZE: size_t = sizenodes as size_t;
        let mut OLD_BYTES: size_t =
            NEW_SIZE.wrapping_mul(::core::mem::size_of::<*mut Node>() as libc::c_ulong);
        let mut NEW_BYTES: size_t = 0;
        if NEW_SIZE != 0 {
            NEW_SIZE = NEW_SIZE * 2 as libc::c_int as size_t;
        } else {
            NEW_SIZE = 1 as libc::c_int as size_t;
        }
        NEW_BYTES = NEW_SIZE.wrapping_mul(::core::mem::size_of::<*mut Node>() as libc::c_ulong);
        decmem(OLD_BYTES);
        nodes = realloc(nodes as *mut libc::c_void, NEW_BYTES) as *mut *mut Node;
        if nodes.is_null() {
            err(b"out of memory\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        incmem(NEW_BYTES);
        sizenodes = NEW_SIZE as libc::c_int;
    }
    let fresh7 = numnodes;
    numnodes = numnodes + 1;
    let ref mut fresh8 = *nodes.offset(fresh7 as isize);
    *fresh8 = res;
    if numnodes > maxnumnodes {
        maxnumnodes = numnodes;
    }
    unlocknodes();
    nmsg(res, b"new node\0" as *const u8 as *const libc::c_char);
    if !parent.is_null() {
        (*res).lgl = lglclone((*parent).lgl);
        (*res).decisions = lglgetdecs((*res).lgl);
        (*res).conflicts = lglgetconfs((*res).lgl);
        (*res).propagations = lglgetprops((*res).lgl);
        (*res).consumed = (*parent).consumed;
        (*res).cube = appendint((*parent).cube, decision);
        lgladd((*res).lgl, decision);
        lgladd((*res).lgl, 0 as libc::c_int);
    } else {
        msg(b"forking root solver instance\0" as *const u8 as *const libc::c_char);
        let mut BYTES_0: size_t = (1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
        (*res).cube = malloc(BYTES_0) as *mut libc::c_int;
        if ((*res).cube).is_null() {
            err(b"out of memory\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        memset((*res).cube as *mut libc::c_void, 0 as libc::c_int, BYTES_0);
        incmem(BYTES_0);
        (*res).lgl = lglfork(root);
    }
    sprintf(
        prefix.as_mut_ptr(),
        b"c (%d %lld) \0" as *const u8 as *const libc::c_char,
        (*res).depth,
        (*res).id as libc::c_longlong,
    );
    lglsetprefix((*res).lgl, prefix.as_mut_ptr());
    lglseterm(
        (*res).lgl,
        Some(term as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int),
        0 as *mut libc::c_void,
    );
    lglsetmsglock(
        (*res).lgl,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(::core::mem::transmute::<
            unsafe extern "C" fn() -> (),
            unsafe extern "C" fn() -> (),
        >(lockmsg))),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(::core::mem::transmute::<
            unsafe extern "C" fn() -> (),
            unsafe extern "C" fn() -> (),
        >(unlockmsg))),
        0 as *mut libc::c_void,
    );
    if noparallel == 0 {
        lglsetconsumeunits(
            (*res).lgl,
            Some(
                consumeunits
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut libc::c_int,
                        *mut *mut libc::c_int,
                    ) -> (),
            ),
            &mut (*res).consumed as *mut libc::c_int as *mut libc::c_void,
        );
    }
    cubemsg(res, b"opened cube\0" as *const u8 as *const libc::c_char);
    added += 1;
    added;
    return res;
}
unsafe extern "C" fn updstats(mut node: *mut Node) {
    let mut ptr: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut now: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    let mut lgl: *mut LGL = (*node).lgl;
    lockstats();
    decisions += lglgetdecs(lgl) - (*node).decisions;
    conflicts += lglgetconfs(lgl) - (*node).conflicts;
    propagations += lglgetprops(lgl) - (*node).propagations;
    if started != 0 && {
        ptr = startimeptr;
        !ptr.is_null()
    } {
        now = currentime();
        delta = now - startime;
        startime = now;
        *ptr += delta;
    }
    unlockstats();
}
unsafe extern "C" fn delnode(mut node: *mut Node) {
    let mut last: *mut Node = 0 as *mut Node;
    let mut lastpos: libc::c_int = 0;
    let mut lgl: *mut LGL = 0 as *mut LGL;
    cubemsg(node, b"closed cube\0" as *const u8 as *const libc::c_char);
    nmsg(node, b"delete node\0" as *const u8 as *const libc::c_char);
    locknodes();
    numnodes -= 1;
    lastpos = numnodes;
    last = *nodes.offset(lastpos as isize);
    if node != last {
        let ref mut fresh9 = *nodes.offset((*node).pos as isize);
        *fresh9 = last;
        (*last).pos = (*node).pos;
    }
    let ref mut fresh10 = *nodes.offset(lastpos as isize);
    *fresh10 = 0 as *mut Node;
    unlocknodes();
    lgl = (*node).lgl;
    updstats(node);
    (*node).lgl = 0 as *mut LGL;
    if !((*node).cube).is_null() {
        let mut BYTES: size_t = ((intslen((*node).cube) + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
        decmem(BYTES);
        free((*node).cube as *mut libc::c_void);
    }
    let mut BYTES_0: size_t = (1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<Node>() as libc::c_ulong);
    decmem(BYTES_0);
    free(node as *mut libc::c_void);
    if showstats != 0 {
        lglstats(lgl);
    }
    lglrelease(lgl);
    deleted += 1;
    deleted;
}
unsafe extern "C" fn leafmsg(mut leaf: *mut Leaf, mut str: *const libc::c_char) {
    let mut p: *const libc::c_int = 0 as *const libc::c_int;
    if verbose == 0 {
        return;
    }
    lockmsg();
    printf(
        b"c %s leaf %lld clause\0" as *const u8 as *const libc::c_char,
        str,
        (*leaf).id as libc::c_longlong,
    );
    if !((*leaf).lits).is_null() {
        p = (*leaf).lits;
        while *p != 0 {
            printf(b" %d\0" as *const u8 as *const libc::c_char, *p);
            p = p.offset(1);
            p;
        }
        printf(b" 0\0" as *const u8 as *const libc::c_char);
    } else {
        fputs(
            b" <literals-already-deleted>\0" as *const u8 as *const libc::c_char,
            stdout,
        );
    }
    fputc('\n' as i32, stdout);
    fflush(stdout);
    unlockmsg();
}
unsafe extern "C" fn newleaf(mut node: *mut Node) -> *mut Leaf {
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut lit: libc::c_int = 0;
    let mut res: *mut Leaf = 0 as *mut Leaf;
    let mut BYTES: size_t = (1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<Leaf>() as libc::c_ulong);
    res = malloc(BYTES) as *mut Leaf;
    if res.is_null() {
        err(b"out of memory\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memset(res as *mut libc::c_void, 0 as libc::c_int, BYTES);
    incmem(BYTES);
    leafs.num += 1;
    if leafs.num > leafs.max {
        leafs.max = leafs.num;
    }
    leafs.count += 1;
    (*res).id = leafs.count;
    (*res).lits = (*node).cube;
    (*node).cube = 0 as *mut libc::c_int;
    p = (*res).lits;
    loop {
        lit = *p;
        if !(lit != 0) {
            break;
        }
        *p = -lit;
        p = p.offset(1);
        p;
    }
    leafmsg(res, b"new\0" as *const u8 as *const libc::c_char);
    return res;
}
unsafe extern "C" fn deleaf(mut leaf: *mut Leaf) {
    leafmsg(leaf, b"delete\0" as *const u8 as *const libc::c_char);
    if !((*leaf).lits).is_null() {
        let mut BYTES: size_t = ((intslen((*leaf).lits) + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
        decmem(BYTES);
        free((*leaf).lits as *mut libc::c_void);
    }
    let mut BYTES_0: size_t = (1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<Leaf>() as libc::c_ulong);
    decmem(BYTES_0);
    free(leaf as *mut libc::c_void);
    leafs.count -= 1;
    leafs.count;
}
unsafe extern "C" fn enqleaf(mut leaf: *mut Leaf) {
    leafmsg(leaf, b"enqueue\0" as *const u8 as *const libc::c_char);
    lockleafs();
    if !(leafs.last).is_null() {
        (*leafs.last).next = leaf;
    } else {
        leafs.first = leaf;
    }
    (*leaf).prev = leafs.last;
    leafs.last = leaf;
    unlockleafs();
    lockparleafs();
    parallel.produced.leafs += 1;
    parallel.produced.leafs;
    unlockparleafs();
}
unsafe extern "C" fn deqleaf() -> *mut Leaf {
    let mut res: *mut Leaf = 0 as *mut Leaf;
    lockleafs();
    res = leafs.first;
    if !res.is_null() {
        if !((*res).next).is_null() {
            (*(*res).next).prev = 0 as *mut Leaf;
        } else {
            leafs.last = 0 as *mut Leaf;
        }
        leafs.first = (*res).next;
    }
    unlockleafs();
    if !res.is_null() {
        leafmsg(res, b"dequeue\0" as *const u8 as *const libc::c_char);
    }
    return res;
}
unsafe extern "C" fn enqnewleafromnode(mut node: *mut Node) {
    enqleaf(newleaf(node));
}
static mut clausetoconsume: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
unsafe extern "C" fn consumecls(
    mut voidptr: *mut libc::c_void,
    mut cptr: *mut *mut libc::c_int,
    mut glueptr: *mut libc::c_int,
) {
    let mut leaf: *mut Leaf = deqleaf();
    if !leaf.is_null() {
        if !clausetoconsume.is_null() {
            let mut BYTES: size_t = ((intslen(clausetoconsume) + 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
            decmem(BYTES);
            free(clausetoconsume as *mut libc::c_void);
        }
        clausetoconsume = (*leaf).lits;
        (*leaf).lits = 0 as *mut libc::c_int;
        *cptr = clausetoconsume;
        *glueptr = 0 as libc::c_int;
        deleaf(leaf);
        parallel.consumed.leafs += 1;
        parallel.consumed.leafs;
    } else {
        *cptr = 0 as *mut libc::c_int;
    };
}
unsafe extern "C" fn runparallel(mut dummy: *mut libc::c_void) -> *mut libc::c_void {
    let mut res: libc::c_int = 0;
    res = lglsat(parallel.lgl);
    if res != 0 {
        vrb(
            b"parallel solver search result %d\0" as *const u8 as *const libc::c_char,
            res,
        );
        lockdone();
        parallel.res = res;
        done = parallel.res;
        unlockdone();
    }
    return dummy;
}
unsafe extern "C" fn startparallel(mut lgl: *mut LGL) {
    let mut prefix: [libc::c_char; 80] = [0; 80];
    msg(
        b"cloning and starting %s additional parallel solver instance\0" as *const u8
            as *const libc::c_char,
        if nparallel != 0 {
            b"second\0" as *const u8 as *const libc::c_char
        } else {
            b"first\0" as *const u8 as *const libc::c_char
        },
    );
    nparallel += 1;
    nparallel;
    let mut BYTES: size_t = (nvars as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    parallel.units = malloc(BYTES) as *mut libc::c_int;
    if (parallel.units).is_null() {
        err(b"out of memory\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memset(parallel.units as *mut libc::c_void, 0 as libc::c_int, BYTES);
    incmem(BYTES);
    parallel.lgl = lglclone(lgl);
    sprintf(
        prefix.as_mut_ptr(),
        b"c (parallel%d) \0" as *const u8 as *const libc::c_char,
        nparallel,
    );
    lglsetprefix(parallel.lgl, prefix.as_mut_ptr());
    lglsetopt(
        parallel.lgl,
        b"locs\0" as *const u8 as *const libc::c_char,
        -(1 as libc::c_int),
    );
    lglsetopt(
        parallel.lgl,
        b"locsbanner\0" as *const u8 as *const libc::c_char,
        (nparallel == 1 as libc::c_int) as libc::c_int,
    );
    lglsetopt(
        parallel.lgl,
        b"locsmaxeff\0" as *const u8 as *const libc::c_char,
        1000000 as libc::c_int,
    );
    lglsetopt(
        parallel.lgl,
        b"locsmineff\0" as *const u8 as *const libc::c_char,
        10000 as libc::c_int,
    );
    lglsetopt(
        parallel.lgl,
        b"locsreleff\0" as *const u8 as *const libc::c_char,
        20 as libc::c_int,
    );
    lglsetopt(
        parallel.lgl,
        b"locsvared\0" as *const u8 as *const libc::c_char,
        1000 as libc::c_int,
    );
    lglsetopt(
        parallel.lgl,
        b"locswait\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    lglsetopt(
        parallel.lgl,
        b"block\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    lglsetopt(
        parallel.lgl,
        b"bca\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    lglseterm(
        parallel.lgl,
        Some(term as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int),
        0 as *mut libc::c_void,
    );
    lglsetproduceunit(
        parallel.lgl,
        Some(produceunit as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()),
        0 as *mut libc::c_void,
    );
    lglsetconsumecls(
        parallel.lgl,
        Some(
            consumecls
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *mut libc::c_int,
                    *mut libc::c_int,
                ) -> (),
        ),
        0 as *mut libc::c_void,
    );
    lglsetmsglock(
        parallel.lgl,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(::core::mem::transmute::<
            unsafe extern "C" fn() -> (),
            unsafe extern "C" fn() -> (),
        >(lockmsg))),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> ()>,
            Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(::core::mem::transmute::<
            unsafe extern "C" fn() -> (),
            unsafe extern "C" fn() -> (),
        >(unlockmsg))),
        0 as *mut libc::c_void,
    );
    parallel.decisions = lglgetdecs(parallel.lgl);
    parallel.conflicts = lglgetconfs(parallel.lgl);
    parallel.propagations = lglgetprops(parallel.lgl);
    lockdone();
    stop = 0 as libc::c_int;
    unlockdone();
    if pthread_create(
        &mut parallel.thread,
        0 as *const pthread_attr_t,
        Some(runparallel as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    ) != 0
    {
        err(
            b"failed to create thread for additional parallel solver instance\0" as *const u8
                as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn joinparallel() -> libc::c_int {
    let mut res: libc::c_int = 0;
    lockdone();
    stop = 1 as libc::c_int;
    unlockdone();
    if pthread_join(parallel.thread, 0 as *mut *mut libc::c_void) != 0 {
        err(
            b"failed to join additional parallel solver instance thread\0" as *const u8
                as *const libc::c_char,
        );
    }
    res = parallel.res;
    vrb(
        b"joined parallel solver instance with result %d\0" as *const u8 as *const libc::c_char,
        res,
    );
    return res;
}
unsafe extern "C" fn releaseparallel() {
    let mut lgl: *mut LGL = 0 as *mut LGL;
    lockparstats();
    lgl = parallel.lgl;
    parallel.lgl = 0 as *mut LGL;
    parallel.res = 0 as libc::c_int;
    decisions += lglgetdecs(lgl) - parallel.decisions;
    conflicts += lglgetconfs(lgl) - parallel.conflicts;
    propagations += lglgetprops(lgl) - parallel.propagations;
    unlockparstats();
    if showstats != 0 {
        lglstats(lgl);
    }
    lglrelease(lgl);
    let mut BYTES: size_t = (nvars as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    decmem(BYTES);
    free(parallel.units as *mut libc::c_void);
    parallel.nunits = 0 as libc::c_int;
    vrb(b"released parallel solver instance\0" as *const u8 as *const libc::c_char);
}
static mut wbuf: [libc::c_char; 80] = [0; 80];
static mut wlen: libc::c_int = 0;
unsafe extern "C" fn wflush() {
    let mut i: libc::c_int = 0;
    fputc('v' as i32, stdout);
    i = 0 as libc::c_int;
    while i < wlen {
        fputc(wbuf[i as usize] as libc::c_int, stdout);
        i += 1;
        i;
    }
    fputc('\n' as i32, stdout);
    wlen = 0 as libc::c_int;
}
unsafe extern "C" fn wprint(mut lit: libc::c_int) {
    let mut str: [libc::c_char; 20] = [0; 20];
    let mut len: libc::c_int = 0;
    sprintf(
        str.as_mut_ptr(),
        b" %d\0" as *const u8 as *const libc::c_char,
        lit,
    );
    len = strlen(str.as_mut_ptr()) as libc::c_int;
    if wlen + len > 74 as libc::c_int {
        wflush();
    }
    strcpy(wbuf.as_mut_ptr().offset(wlen as isize), str.as_mut_ptr());
    wlen += len;
}
unsafe extern "C" fn witness(mut lgl: *mut LGL) {
    let mut idx: libc::c_int = 0;
    let mut lit: libc::c_int = 0;
    idx = 1 as libc::c_int;
    while idx <= nvars {
        lit = if lglderef(lgl, idx) < 0 as libc::c_int {
            -idx
        } else {
            idx
        };
        wprint(lit);
        idx += 1;
        idx;
    }
    wprint(0 as libc::c_int);
    if wlen != 0 {
        wflush();
    }
}
unsafe extern "C" fn nowfull() -> libc::c_int {
    return (fullint != 0 && round % fullint == 0) as libc::c_int;
}
unsafe extern "C" fn schedulejob(
    mut node: *mut Node,
    mut fun: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    mut name: *const libc::c_char,
    mut state: State,
) {
    let mut job: *mut Job = 0 as *mut Job;
    js.cnt += 1;
    js.cnt;
    let mut BYTES: size_t = (1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<Job>() as libc::c_ulong);
    job = malloc(BYTES) as *mut Job;
    if job.is_null() {
        err(b"out of memory\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    memset(job as *mut libc::c_void, 0 as libc::c_int, BYTES);
    incmem(BYTES);
    (*job).node = node;
    (*job).fun = fun;
    (*job).name = name;
    (*job).state = state;
    (*job).pos = numjobs;
    if sizejobs == numjobs {
        let mut NEW_SIZE: size_t = sizejobs as size_t;
        let mut OLD_BYTES: size_t =
            NEW_SIZE.wrapping_mul(::core::mem::size_of::<*mut Job>() as libc::c_ulong);
        let mut NEW_BYTES: size_t = 0;
        if NEW_SIZE != 0 {
            NEW_SIZE = NEW_SIZE * 2 as libc::c_int as size_t;
        } else {
            NEW_SIZE = 1 as libc::c_int as size_t;
        }
        NEW_BYTES = NEW_SIZE.wrapping_mul(::core::mem::size_of::<*mut Job>() as libc::c_ulong);
        decmem(OLD_BYTES);
        jobs = realloc(jobs as *mut libc::c_void, NEW_BYTES) as *mut *mut Job;
        if jobs.is_null() {
            err(b"out of memory\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        incmem(NEW_BYTES);
        sizejobs = NEW_SIZE as libc::c_int;
    }
    let fresh11 = numjobs;
    numjobs = numjobs + 1;
    let ref mut fresh12 = *jobs.offset(fresh11 as isize);
    *fresh12 = job;
    jmsg(job, b"scheduled\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn runjob(mut job: *mut Job) {
    let mut node: *mut Node = (*job).node;
    jmsg(job, b"start\0" as *const u8 as *const libc::c_char);
    (*node).state = (*job).state;
    threads += 1;
    threads;
    if pthread_create(
        &mut (*job).thread,
        0 as *const pthread_attr_t,
        (*job).fun,
        node as *mut libc::c_void,
    ) != 0
    {
        err(
            b"failed to create thread job %d %s [%d %lld]\0" as *const u8 as *const libc::c_char,
            (*job).pos,
            (*job).name,
            (*node).depth,
            (*node).id,
        );
    }
    jmsg(job, b"end\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn incworkers() {
    lockworkers();
    while numworkers >= maxworkers {
        lock.workers.waited += 1;
        lock.workers.waited;
        if pthread_cond_wait(&mut workerscond, &mut lock.workers.mutex) != 0 {
            err(
                b"failed to wait on decrease of the number of workers\0" as *const u8
                    as *const libc::c_char,
            );
        }
        lock.workers.waited -= 1;
        lock.workers.waited;
    }
    numworkers += 1;
    numworkers;
    if numworkers > maxnumworkers {
        maxnumworkers = numworkers;
    }
    vrb(
        b"number of workers increased to %d\0" as *const u8 as *const libc::c_char,
        numworkers,
    );
    unlockworkers();
}
unsafe extern "C" fn decworkers() {
    lockworkers();
    numworkers -= 1;
    numworkers;
    vrb(
        b"number of workers decreased to %d\0" as *const u8 as *const libc::c_char,
        numworkers,
    );
    if pthread_cond_signal(&mut workerscond) != 0 {
        err(
            b"failed to signal decrease of the number of workers\0" as *const u8
                as *const libc::c_char,
        );
    }
    unlockworkers();
}
unsafe extern "C" fn nodebytes(mut n: *mut Node) -> size_t {
    return lglbytes((*n).lgl);
}
unsafe extern "C" fn nodevars(mut n: *mut Node) -> size_t {
    return lglnvars((*n).lgl) as size_t;
}
unsafe extern "C" fn cmpnodes(
    mut m: *mut Node,
    mut n: *mut Node,
    mut dir: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut skipm: libc::c_int = skipnode(m);
    let mut skipn: libc::c_int = skipnode(n);
    res = skipm - skipn;
    if res != 0 {
        return res;
    }
    if skipn == 0 && skipm == 0 && {
        res = (nodevars(m)).wrapping_sub(nodevars(n)) as libc::c_int;
        res != 0
    } {
        return dir * res;
    }
    res = (*n).depth - (*m).depth;
    if res != 0 {
        return res;
    }
    if (*m).id > (*n).id {
        return -(1 as libc::c_int);
    }
    if (*m).id < (*n).id {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cmpjobs4qsort(
    mut p: *const libc::c_void,
    mut q: *const libc::c_void,
) -> libc::c_int {
    let mut j: *mut Job = *(p as *mut *mut Job);
    let mut k: *mut Job = *(q as *mut *mut Job);
    return cmpnodes((*j).node, (*k).node, -(1 as libc::c_int));
}
unsafe extern "C" fn fixjobspos() {
    let mut job: *mut Job = 0 as *mut Job;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < numjobs {
        job = *jobs.offset(i as isize);
        (*job).pos = i;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bytes2mb(mut bytes: size_t) -> libc::c_int {
    let mut res: size_t = bytes;
    res >>= 20 as libc::c_int;
    if res > 2147483647 as libc::c_int as size_t {
        res = 2147483647 as libc::c_int as size_t;
    }
    return res as libc::c_int;
}
unsafe extern "C" fn printnode(mut prefix: *const libc::c_char, mut node: *mut Node) {
    msg(
        b"  %s:  node[%d] = [%d %lld]  (%d vars, %d MB)\0" as *const u8 as *const libc::c_char,
        prefix,
        (*node).pos,
        (*node).depth,
        (*node).id as libc::c_longlong,
        nodevars(node),
        bytes2mb(nodebytes(node)),
    );
}
unsafe extern "C" fn printjobs() {
    let mut str: [libc::c_char; 80] = [0; 80];
    let mut job: *mut Job = 0 as *mut Job;
    let mut i: libc::c_int = 0;
    msg(b"\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < numjobs {
        job = *jobs.offset(i as isize);
        sprintf(
            str.as_mut_ptr(),
            b"job[%d] round %d\0" as *const u8 as *const libc::c_char,
            i,
            round,
        );
        printnode(str.as_mut_ptr(), (*job).node);
        i += 1;
        i;
    }
    msg(b"\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn sortjobs() {
    qsort(
        jobs as *mut libc::c_void,
        numjobs as size_t,
        ::core::mem::size_of::<*mut Job>() as libc::c_ulong,
        Some(
            cmpjobs4qsort
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        ),
    );
    fixjobspos();
    if verbose != 0 {
        printjobs();
    }
}
unsafe extern "C" fn runjobs() {
    let mut job: *mut Job = 0 as *mut Job;
    let mut i: libc::c_int = 0;
    sortjobs();
    vrb(
        b"running %d jobs\0" as *const u8 as *const libc::c_char,
        numjobs,
    );
    numworkers = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < numjobs {
        incworkers();
        job = *jobs.offset(i as isize);
        runjob(job);
        i += 1;
        i;
    }
    vrb(
        b"started running %d jobs\0" as *const u8 as *const libc::c_char,
        numjobs,
    );
}
unsafe extern "C" fn joinjob(mut job: *mut Job) {
    let mut node: *mut Node = (*job).node;
    if (*node).state as libc::c_uint == SEARCH as libc::c_int as libc::c_uint {
        mmsg(b"join search\0" as *const u8 as *const libc::c_char, node);
    } else if (*node).state as libc::c_uint == SIMP as libc::c_int as libc::c_uint {
        mmsg(b"join simp\0" as *const u8 as *const libc::c_char, node);
    } else if (*node).state as libc::c_uint == LKHD as libc::c_int as libc::c_uint {
        mmsg(b"join simp\0" as *const u8 as *const libc::c_char, node);
    } else {
        mmsg(b"join split\0" as *const u8 as *const libc::c_char, node);
    }
    if pthread_join((*job).thread, 0 as *mut *mut libc::c_void) != 0 {
        err(b"failed to join thread\0" as *const u8 as *const libc::c_char);
    }
    (*node).state = READY;
    if (*node).res == 20 as libc::c_int {
        mmsg(b"unsatisfiable\0" as *const u8 as *const libc::c_char, node);
    } else if (*node).res == 10 as libc::c_int {
        lockdone();
        mmsg(b"satisfiable\0" as *const u8 as *const libc::c_char, node);
        done = 10 as libc::c_int;
        unlockdone();
    } else {
        mmsg(b"unknown\0" as *const u8 as *const libc::c_char, node);
    }
    let mut BYTES: size_t = (1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<Job>() as libc::c_ulong);
    decmem(BYTES);
    free(job as *mut libc::c_void);
}
unsafe extern "C" fn joinjobs() {
    let mut i: libc::c_int = 0;
    vrb(
        b"joining %d jobs in round %d\0" as *const u8 as *const libc::c_char,
        numjobs,
        round,
    );
    i = 0 as libc::c_int;
    while i < numjobs {
        joinjob(*jobs.offset(i as isize));
        i += 1;
        i;
    }
    vrb(
        b"finished joining %d nodes in round %d\0" as *const u8 as *const libc::c_char,
        numjobs,
        round,
    );
    numjobs = 0 as libc::c_int;
}
unsafe extern "C" fn incmpnodes(
    mut p: *const libc::c_void,
    mut q: *const libc::c_void,
) -> libc::c_int {
    return cmpnodes(
        *(p as *mut *mut Node),
        *(q as *mut *mut Node),
        1 as libc::c_int,
    );
}
unsafe extern "C" fn decmpnodes(
    mut p: *const libc::c_void,
    mut q: *const libc::c_void,
) -> libc::c_int {
    return cmpnodes(
        *(p as *mut *mut Node),
        *(q as *mut *mut Node),
        -(1 as libc::c_int),
    );
}
unsafe extern "C" fn printnodes(mut prefix: *const libc::c_char) {
    let mut node: *mut Node = 0 as *mut Node;
    let mut i: libc::c_int = 0;
    msg(b"\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < numnodes {
        node = *nodes.offset(i as isize);
        if !(skipnode(node) != 0) {
            printnode(prefix, node);
        }
        i += 1;
        i;
    }
    msg(b"\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn fixnodespos() {
    let mut node: *mut Node = 0 as *mut Node;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < numnodes {
        node = *nodes.offset(i as isize);
        (*node).pos = i;
        i += 1;
        i;
    }
}
unsafe extern "C" fn sortnodes(
    mut name: *const libc::c_char,
    mut cmp: Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>,
) {
    let mut prefix: [libc::c_char; 80] = [0; 80];
    qsort(
        nodes as *mut libc::c_void,
        numnodes as size_t,
        ::core::mem::size_of::<*mut Node>() as libc::c_ulong,
        cmp,
    );
    fixnodespos();
    if verbose == 0 {
        return;
    }
    sprintf(
        prefix.as_mut_ptr(),
        b"%s(%d) sorted\0" as *const u8 as *const libc::c_char,
        name,
        round,
    );
    printnodes(prefix.as_mut_ptr());
}
unsafe extern "C" fn simpnode(mut voidptr: *mut libc::c_void) -> *mut libc::c_void {
    let mut node: *mut Node = voidptr as *mut Node;
    nmsg(node, b"simp\0" as *const u8 as *const libc::c_char);
    (*node).res = lglsimp((*node).lgl, 1 as libc::c_int);
    (*node).simplified = 1 as libc::c_int;
    nmsg(
        node,
        b"simp result %d\0" as *const u8 as *const libc::c_char,
        (*node).res,
    );
    if (*node).res == 10 as libc::c_int {
        lockdone();
        done = 10 as libc::c_int;
        unlockdone();
    } else if (*node).res == 20 as libc::c_int {
        locksimplified();
        simplified += 1;
        simplified;
        sumsimplified += 1;
        sumsimplified;
        unlocksimplified();
    }
    decworkers();
    return node as *mut libc::c_void;
}
unsafe extern "C" fn simp() {
    let mut node: *mut Node = 0 as *mut Node;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    if nosimp != 0 {
        return;
    }
    startimer(&mut wct.simp);
    startphase(b"simp\0" as *const u8 as *const libc::c_char);
    sortnodes(
        b"simp\0" as *const u8 as *const libc::c_char,
        Some(
            incmpnodes
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        ),
    );
    l = maxactive;
    if nowfull() != 0 {
        l *= 4 as libc::c_int;
    }
    if l > numnodes {
        l = numnodes;
    }
    msg(
        b" %d simplify  %d out of %d nodes %.0f%%\0" as *const u8 as *const libc::c_char,
        round,
        l,
        numnodes,
        pcnt(l as libc::c_double, numnodes as libc::c_double),
    );
    i = 0 as libc::c_int;
    while i < numnodes && numjobs < l {
        node = *nodes.offset(i as isize);
        if !(skipnode(node) != 0) {
            if !(forcesimp == 0 && (*node).simplified != 0) {
                js.simp += 1;
                js.simp;
                schedulejob(
                    node,
                    Some(simpnode as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
                    b"simp\0" as *const u8 as *const libc::c_char,
                    SIMP,
                );
            }
        }
        i += 1;
        i;
    }
    vrb(
        b"scheduled %d simplification jobs in round %d\0" as *const u8 as *const libc::c_char,
        numjobs,
        round,
    );
    runjobs();
    joinjobs();
    stoptimer();
}
#[no_mangle]
pub unsafe extern "C" fn mysrand(mut seed: libc::c_ulonglong) {
    let mut z: libc::c_uint = (seed >> 32 as libc::c_int) as libc::c_uint;
    let mut w: libc::c_uint = seed as libc::c_uint;
    if z == 0 {
        z = !z;
    }
    if w == 0 {
        w = !w;
    }
    rng.z = z;
    rng.w = w;
}
unsafe extern "C" fn myrand() -> libc::c_uint {
    let mut res: libc::c_uint = 0;
    rng.z = (36969 as libc::c_int as libc::c_uint)
        .wrapping_mul(rng.z & 65535 as libc::c_int as libc::c_uint)
        .wrapping_add(rng.z >> 16 as libc::c_int);
    rng.w = (18000 as libc::c_int as libc::c_uint)
        .wrapping_mul(rng.w & 65535 as libc::c_int as libc::c_uint)
        .wrapping_add(rng.w >> 16 as libc::c_int);
    res = (rng.z << 16 as libc::c_int).wrapping_add(rng.w);
    return res;
}
unsafe extern "C" fn myrandmod(mut mod_0: libc::c_uint) -> libc::c_uint {
    let mut res: libc::c_uint = 0;
    if mod_0 <= 1 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as libc::c_uint;
    }
    res = myrand();
    res = res.wrapping_rem(mod_0);
    return res;
}
unsafe extern "C" fn lookaheadnode(mut voidptr: *mut libc::c_void) -> *mut libc::c_void {
    let mut oldvars: libc::c_int = 0;
    let mut newvars: libc::c_int = 0;
    let mut redpermille: libc::c_int = 0;
    let mut node: *mut Node = voidptr as *mut Node;
    oldvars = lglnvars((*node).lgl);
    if (*node).depth <= treelookdepth {
        nmsg(
            node,
            b"tree-based look-ahead at depth %d\0" as *const u8 as *const libc::c_char,
            (*node).depth,
        );
        lglsetopt(
            (*node).lgl,
            b"lkhd\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int,
        );
        ::core::intrinsics::atomic_xadd_seqcst(&mut treelkhd, 1 as libc::c_int as int64_t);
    } else if locslkhd != 0 {
        nmsg(
            node,
            b"local-search based look-ahead at depth %d\0" as *const u8 as *const libc::c_char,
            (*node).depth,
        );
        lglsetopt(
            (*node).lgl,
            b"lkhd\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
    } else if relevancelkhd != 0 {
        nmsg(
            node,
            b"relevance based look-ahead at depth %d\0" as *const u8 as *const libc::c_char,
            (*node).depth,
        );
        lglsetopt(
            (*node).lgl,
            b"lkhd\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int,
        );
    } else {
        nmsg(
            node,
            b"JWH based look-ahead at depth %d\0" as *const u8 as *const libc::c_char,
            (*node).depth,
        );
        lglsetopt(
            (*node).lgl,
            b"lkhd\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
        lglsetopt(
            (*node).lgl,
            b"jwhred\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    ::core::intrinsics::atomic_xadd_seqcst(&mut totalkhd, 1 as libc::c_int as int64_t);
    (*node).lookahead = lglookahead((*node).lgl);
    nmsg(
        node,
        b"lookahead literal %d\0" as *const u8 as *const libc::c_char,
        (*node).lookahead,
    );
    newvars = lglnvars((*node).lgl);
    if oldvars == 0 {
        redpermille = 0 as libc::c_int;
    } else {
        redpermille = (1000 as libc::c_longlong * (oldvars - newvars) as libc::c_longlong
            / oldvars as libc::c_longlong) as libc::c_int;
    }
    nmsg(
        node,
        b"lookahead reduced %d variables to %d variables %.1f%%\0" as *const u8
            as *const libc::c_char,
        oldvars,
        newvars,
        redpermille as libc::c_double / 10.0f64,
    );
    decworkers();
    return node as *mut libc::c_void;
}
unsafe extern "C" fn firstlkhd() -> libc::c_int {
    let mut numactive: libc::c_int = if maxactive > numnodes {
        numnodes
    } else {
        maxactive
    };
    numtosplit = (numactive * branches + 99 as libc::c_int) / 100 as libc::c_int;
    if numnodes < maxactive {
        let mut delta: libc::c_int = maxactive - numnodes;
        if numtosplit <= delta {
            numtosplit = delta;
        }
        if numtosplit > numnodes {
            numtosplit = numnodes;
        }
    }
    if asymmetric != 0 {
        lastosplit = numactive - 1 as libc::c_int;
        firstosplit = lastosplit - numtosplit + 1 as libc::c_int;
    } else {
        firstosplit = 0 as libc::c_int;
        lastosplit = numtosplit - 1 as libc::c_int;
        if numtosplit > numnodes {
            lastosplit = numnodes - 1 as libc::c_int;
        }
    }
    msg(
        b" %d lookahead %d out of %d nodes %.0f%% on nodes [%d..%d]\0" as *const u8
            as *const libc::c_char,
        round,
        numtosplit,
        numnodes,
        pcnt(numtosplit as libc::c_double, numnodes as libc::c_double),
        firstosplit,
        lastosplit,
    );
    fflush(stdout);
    return firstosplit;
}
unsafe extern "C" fn donelkhd(mut i: libc::c_int) -> libc::c_int {
    return (i > lastosplit) as libc::c_int;
}
unsafe extern "C" fn nextlkhd(mut i: libc::c_int) -> libc::c_int {
    return i + 1 as libc::c_int;
}
unsafe extern "C" fn lookahead() {
    let mut sumbytes: size_t = currentbytes;
    let mut expected: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut node: *mut Node = 0 as *mut Node;
    startimer(&mut wct.lkhd);
    i = 0 as libc::c_int;
    while i < numnodes {
        node = *nodes.offset(i as isize);
        (*node).lookahead = 0 as libc::c_int;
        i += 1;
        i;
    }
    startphase(b"lookahead\0" as *const u8 as *const libc::c_char);
    sortnodes(
        b"lookahead\0" as *const u8 as *const libc::c_char,
        if balance != 0 {
            Some(
                decmpnodes
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            )
        } else {
            Some(
                incmpnodes
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            )
        },
    );
    if randswap != 0 {
        vrb(
            b"swapping %d random node pairs\0" as *const u8 as *const libc::c_char,
            randswap,
        );
        k = 0 as libc::c_int;
        while k < randswap {
            i = myrandmod(numnodes as libc::c_uint) as libc::c_int;
            j = myrandmod(numnodes as libc::c_uint) as libc::c_int;
            vrb(
                b"swapping nodes %d and %d\0" as *const u8 as *const libc::c_char,
                i,
                j,
            );
            node = *nodes.offset(i as isize);
            let ref mut fresh13 = *nodes.offset(i as isize);
            *fresh13 = *nodes.offset(j as isize);
            let ref mut fresh14 = *nodes.offset(j as isize);
            *fresh14 = node;
            k += 1;
            k;
        }
        fixnodespos();
    }
    vrb(
        b"starting lookahead round %d\0" as *const u8 as *const libc::c_char,
        round,
    );
    i = firstlkhd();
    while donelkhd(i) == 0 {
        node = *nodes.offset(i as isize);
        expected = nodebytes(node);
        nmsg(
            node,
            b"cloning might add %d MB\0" as *const u8 as *const libc::c_char,
            bytes2mb(expected),
        );
        nmsg(
            node,
            b"plus already scheduled %d MB gives %d MB\0" as *const u8 as *const libc::c_char,
            bytes2mb(sumbytes),
            bytes2mb(expected.wrapping_add(sumbytes)),
        );
        if sumbytes.wrapping_add(expected) > softlimbytes {
            vrb(
                b"resulting size %d MB exceeds soft memory limit %d MB\0" as *const u8
                    as *const libc::c_char,
                bytes2mb(sumbytes.wrapping_add(expected)),
                bytes2mb(softlimbytes),
            );
            if splitlimbytes == 0 || splitlimbytes > sumbytes {
                splitlimbytes = sumbytes;
            }
        } else {
            sumbytes = sumbytes.wrapping_add(expected);
            js.lkhd += 1;
            js.lkhd;
            schedulejob(
                node,
                Some(lookaheadnode as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
                b"lookahead\0" as *const u8 as *const libc::c_char,
                LKHD,
            );
        }
        i = nextlkhd(i);
    }
    vrb(
        b"final size is expected not to exceed %d MB\0" as *const u8 as *const libc::c_char,
        bytes2mb(sumbytes),
    );
    vrb(
        b"scheduled %d lookahead jobs out of %d in round %d\0" as *const u8 as *const libc::c_char,
        numjobs,
        numnodes,
        round,
    );
    runjobs();
    joinjobs();
    stoptimer();
}
unsafe extern "C" fn splitnode(mut voidptr: *mut libc::c_void) -> *mut libc::c_void {
    let mut node: *mut Node = voidptr as *mut Node;
    let mut child: *mut Node = 0 as *mut Node;
    child = newnode(node, -(*node).lookahead);
    addint(&mut (*node).cube, (*node).lookahead);
    cubemsg(node, b"extended\0" as *const u8 as *const libc::c_char);
    lgladd((*node).lgl, (*node).lookahead);
    lgladd((*node).lgl, 0 as libc::c_int);
    nmsg(
        node,
        b"light simplification\0" as *const u8 as *const libc::c_char,
    );
    lglsimp((*node).lgl, 0 as libc::c_int);
    if reducecache != 0 {
        nmsg(
            child,
            b"reducing cache\0" as *const u8 as *const libc::c_char,
        );
        lglreducecache((*child).lgl);
        lglreducecache((*node).lgl);
    }
    nmsg(
        child,
        b"light simplification\0" as *const u8 as *const libc::c_char,
    );
    lglsimp((*child).lgl, 0 as libc::c_int);
    nmsg(
        node,
        b"cloned and lightly simplified node and child\0" as *const u8 as *const libc::c_char,
    );
    (*node).simplified = 0 as libc::c_int;
    (*child).simplified = (*node).simplified;
    decworkers();
    splitsuccessful = 1 as libc::c_int;
    return node as *mut libc::c_void;
}
unsafe extern "C" fn split() {
    let mut i: libc::c_int = 0;
    let mut tosplit: libc::c_int = 0 as libc::c_int;
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut node: *mut Node = 0 as *mut Node;
    startimer(&mut wct.split);
    startphase(b"split\0" as *const u8 as *const libc::c_char);
    splitsuccessful = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < numnodes {
        node = *nodes.offset(i as isize);
        tosplit += ((*node).lookahead != 0 as libc::c_int) as libc::c_int;
        i += 1;
        i;
    }
    msg(
        b" %d splitting %d out of %d nodes %.0f%%\0" as *const u8 as *const libc::c_char,
        round,
        tosplit,
        numnodes,
        pcnt(tosplit as libc::c_double, numnodes as libc::c_double),
    );
    i = 0 as libc::c_int;
    while i < numnodes {
        node = *nodes.offset(i as isize);
        if !(skipnode(node) != 0) {
            found += 1;
            found;
            if !((*node).lookahead == 0) {
                js.split += 1;
                js.split;
                schedulejob(
                    node,
                    Some(splitnode as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
                    b"split\0" as *const u8 as *const libc::c_char,
                    SPLIT,
                );
            }
        }
        i += 1;
        i;
    }
    vrb(
        b"scheduled %d cloning jobs for %d / %d nodes in round %d\0" as *const u8
            as *const libc::c_char,
        numjobs,
        tosplit,
        found,
        round,
    );
    runjobs();
    joinjobs();
    stoptimer();
}
unsafe extern "C" fn varspan(
    mut minvarsptr: *mut libc::c_int,
    mut actvarsptr: *mut libc::c_int,
    mut maxvarsptr: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut minvars: libc::c_int = 0;
    let mut actvars: libc::c_int = 0;
    let mut maxvars: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    sortnodes(
        b"varspan\0" as *const u8 as *const libc::c_char,
        Some(
            incmpnodes
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < numnodes {
        if skipnode(*nodes.offset(i as isize)) == 0 {
            break;
        }
        i += 1;
        i;
    }
    minvars = if i < numnodes {
        lglnvars((**nodes.offset(i as isize)).lgl)
    } else {
        0 as libc::c_int
    };
    i = numnodes - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if skipnode(*nodes.offset(i as isize)) == 0 {
            break;
        }
        i -= 1;
        i;
    }
    maxvars = if i < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        lglnvars((**nodes.offset(i as isize)).lgl)
    };
    j = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < numnodes {
        if skipnode(*nodes.offset(i as isize)) == 0 && {
            j += 1;
            j == maxactive
        } {
            break;
        }
        i += 1;
        i;
    }
    actvars = if j == maxactive {
        lglnvars((**nodes.offset(i as isize)).lgl)
    } else {
        maxvars
    };
    *minvarsptr = minvars;
    *actvarsptr = actvars;
    *maxvarsptr = maxvars;
}
unsafe extern "C" fn report() {
    let mut minvars: libc::c_int = 0;
    let mut maxvars: libc::c_int = 0;
    let mut actvars: libc::c_int = 0;
    let mut isfull: libc::c_int = nowfull();
    let mut t: libc::c_double = getime();
    let mut m: libc::c_double = 0.;
    lockmem();
    m = currentbytes as libc::c_double
        / ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_double;
    unlockmem();
    varspan(&mut minvars, &mut actvars, &mut maxvars);
    msg(
        b"%c%d %lld%c %.1f sec, %.0f MB, %d nodes +%d -%d s%d, vars[%d..%d..%d]\0" as *const u8
            as *const libc::c_char,
        if isfull != 0 { '[' as i32 } else { '(' as i32 },
        round,
        ids as libc::c_longlong,
        if isfull != 0 { ']' as i32 } else { ')' as i32 },
        t,
        m,
        numnodes,
        added,
        deleted,
        simplified,
        minvars,
        actvars,
        maxvars,
    );
}
static mut lopts: [Opt; 3] = [
    {
        let mut init = Opt {
            name: b"restartint\0" as *const u8 as *const libc::c_char,
            val: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = Opt {
            name: b"restartint\0" as *const u8 as *const libc::c_char,
            val: 1000 as libc::c_int,
        };
        init
    },
    {
        let mut init = Opt {
            name: b"phase\0" as *const u8 as *const libc::c_char,
            val: -(1 as libc::c_int),
        };
        init
    },
];
static mut nopts: libc::c_int = 0;
unsafe extern "C" fn hashtwo64(
    mut a: libc::c_ulonglong,
    mut b: libc::c_ulonglong,
) -> libc::c_ulonglong {
    return (123369937 as libc::c_ulonglong)
        .wrapping_mul(a)
        .wrapping_add((4443739543 as libc::c_long as libc::c_ulonglong).wrapping_mul(b))
        .wrapping_add(346961 as libc::c_int as libc::c_ulonglong);
}
unsafe extern "C" fn setopts(mut node: *mut Node) {
    let mut r: libc::c_ulonglong =
        hashtwo64((*node).id as libc::c_ulonglong, round as libc::c_ulonglong);
    let mut def: libc::c_int =
        (r.wrapping_rem(2 as libc::c_int as libc::c_ulonglong) == 0) as libc::c_int;
    let mut newval: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    let mut lgl: *mut LGL = (*node).lgl;
    let mut o: *mut Opt = 0 as *mut Opt;
    lockopts();
    let fresh15 = opts.set;
    opts.set = opts.set + 1;
    first = (fresh15 == 0) as libc::c_int;
    if def != 0 {
        opts.def += 1;
        opts.def;
    }
    unlockopts();
    if first != 0 {
        msg(b"\0" as *const u8 as *const libc::c_char);
        msg(
            b"using %d options for portfolio solving:\0" as *const u8 as *const libc::c_char,
            nopts,
        );
        msg(b"\0" as *const u8 as *const libc::c_char);
        o = lopts.as_mut_ptr();
        while o < lopts.as_mut_ptr().offset(nopts as isize) {
            msg(
                b"  --%s=%d\0" as *const u8 as *const libc::c_char,
                (*o).name,
                (*o).val,
            );
            o = o.offset(1);
            o;
        }
        msg(b"\0" as *const u8 as *const libc::c_char);
    }
    if def != 0 {
        o = lopts.as_mut_ptr();
        while o < lopts.as_mut_ptr().offset(nopts as isize) {
            newval = lgldefopt(lgl, (*o).name);
            if !(lglgetopt(lgl, (*o).name) == newval) {
                nmsg(
                    node,
                    b"resetting option --%s=%d (default)\0" as *const u8 as *const libc::c_char,
                    (*o).name,
                    newval,
                );
                lglsetopt(lgl, (*o).name, newval);
            }
            o = o.offset(1);
            o;
        }
    } else {
        o = lopts
            .as_mut_ptr()
            .offset(r.wrapping_rem(nopts as libc::c_ulonglong) as isize);
        nmsg(
            node,
            b"setting option --%s=%d%s\0" as *const u8 as *const libc::c_char,
            (*o).name,
            (*o).val,
            if def != 0 {
                b" (default)\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        lglsetopt(lgl, (*o).name, (*o).val);
        lglsetopt(lgl, b"seed\0" as *const u8 as *const libc::c_char, def);
    };
}
unsafe extern "C" fn searchnode(mut voidptr: *mut libc::c_void) -> *mut libc::c_void {
    let mut node: *mut Node = voidptr as *mut Node;
    let mut oldconfs: int64_t = lglgetconfs((*node).lgl);
    let mut deltaconfs: int64_t = 0;
    if portfolio != 0 {
        setopts(node);
    }
    nmsg(
        node,
        b"search for %d conflicts\0" as *const u8 as *const libc::c_char,
        thisclim,
    );
    lglsetopt(
        (*node).lgl,
        b"clim\0" as *const u8 as *const libc::c_char,
        thisclim,
    );
    (*node).res = lglsat((*node).lgl);
    (*node).simplified = 0 as libc::c_int;
    nmsg(
        node,
        b"search result %d\0" as *const u8 as *const libc::c_char,
        (*node).res,
    );
    if (*node).res == 10 as libc::c_int {
        lockdone();
        done = 10 as libc::c_int;
        unlockdone();
    }
    decworkers();
    lockconfs();
    deltaconfs = lglgetconfs((*node).lgl) - oldconfs;
    if sizeconfstack == numconfstack {
        let mut NEW_SIZE: size_t = sizeconfstack as size_t;
        let mut OLD_BYTES: size_t =
            NEW_SIZE.wrapping_mul(::core::mem::size_of::<int64_t>() as libc::c_ulong);
        let mut NEW_BYTES: size_t = 0;
        if NEW_SIZE != 0 {
            NEW_SIZE = NEW_SIZE * 2 as libc::c_int as size_t;
        } else {
            NEW_SIZE = 1 as libc::c_int as size_t;
        }
        NEW_BYTES = NEW_SIZE.wrapping_mul(::core::mem::size_of::<int64_t>() as libc::c_ulong);
        decmem(OLD_BYTES);
        confstack = realloc(confstack as *mut libc::c_void, NEW_BYTES) as *mut int64_t;
        if confstack.is_null() {
            err(b"out of memory\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        incmem(NEW_BYTES);
        sizeconfstack = NEW_SIZE as libc::c_int;
    }
    let fresh16 = numconfstack;
    numconfstack = numconfstack + 1;
    *confstack.offset(fresh16 as isize) = deltaconfs;
    unlockconfs();
    return node as *mut libc::c_void;
}
unsafe extern "C" fn cmpint64(
    mut p: *const libc::c_void,
    mut q: *const libc::c_void,
) -> libc::c_int {
    let mut a: int64_t = *(p as *mut int64_t);
    let mut b: int64_t = *(q as *mut int64_t);
    if a < b {
        return -(1 as libc::c_int);
    }
    if a > b {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn search() {
    let mut avgconfs: int64_t = 0;
    let mut medianconfs: int64_t = 0;
    let mut node: *mut Node = 0 as *mut Node;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    lockconfs();
    numconfstack = 0 as libc::c_int;
    if eager != 0
        && clim > minclim
        && numnodes < maxactive
        && (softlimbytes == 0 || currentbytes < softlimbytes)
    {
        forcedclim = minclim;
        vrb(
            b" %d forcing limit %d instead of %d\0" as *const u8 as *const libc::c_char,
            round,
            forcedclim,
            clim,
        );
    } else {
        forcedclim = 0 as libc::c_int;
    }
    thisclim = if forcedclim != 0 { forcedclim } else { clim };
    newclim = clim;
    unlockconfs();
    if nosearch != 0 {
        return;
    }
    startimer(&mut wct.search);
    startphase(b"search\0" as *const u8 as *const libc::c_char);
    sortnodes(
        b"search\0" as *const u8 as *const libc::c_char,
        Some(
            incmpnodes
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        ),
    );
    l = maxactive;
    if nowfull() != 0 {
        l *= 2 as libc::c_int;
    }
    if l > numnodes {
        l = numnodes;
    }
    msg(
        b" %d searching %d out of %d nodes %.0f%% limit %d%s\0" as *const u8 as *const libc::c_char,
        round,
        l,
        numnodes,
        pcnt(l as libc::c_double, numnodes as libc::c_double),
        thisclim,
        if forcedclim != 0 {
            b" forced\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    i = 0 as libc::c_int;
    while i < numnodes && numjobs < l {
        node = *nodes.offset(i as isize);
        if !(skipnode(node) != 0) {
            js.search += 1;
            js.search;
            schedulejob(
                node,
                Some(searchnode as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
                b"search\0" as *const u8 as *const libc::c_char,
                SEARCH,
            );
        }
        i += 1;
        i;
    }
    vrb(
        b"scheduled %d search jobs in round %d\0" as *const u8 as *const libc::c_char,
        numjobs,
        round,
    );
    runjobs();
    joinjobs();
    lockconfs();
    if numconfstack != 0 {
        avgconfs = 0 as libc::c_int as int64_t;
        i = 0 as libc::c_int;
        while i < numconfstack {
            avgconfs += *confstack.offset(i as isize);
            i += 1;
            i;
        }
        avgconfs = if numconfstack != 0 {
            avgconfs / numconfstack as int64_t
        } else {
            0 as libc::c_int as int64_t
        };
        qsort(
            confstack as *mut libc::c_void,
            numconfstack as size_t,
            ::core::mem::size_of::<int64_t>() as libc::c_ulong,
            Some(
                cmpint64
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        medianconfs = *confstack.offset((numconfstack / 2 as libc::c_int) as isize);
        msg(
            b" %d average actual conflicts %lld median %lld\0" as *const u8 as *const libc::c_char,
            round,
            avgconfs as libc::c_longlong,
            medianconfs as libc::c_longlong,
        );
        newclim = medianconfs as libc::c_int;
    }
    unlockconfs();
    stoptimer();
}
unsafe extern "C" fn updateclim() {
    let mut oldclim: libc::c_int = clim;
    let mut delta: libc::c_int = 0;
    if forcedclim != 0 {
        forcedclim = 0 as libc::c_int;
        forcedclims += 1;
        forcedclims;
    } else {
        delta = deleted - simplified;
        if delta > added || splitsuccessful == 0 {
            newclim *= 2 as libc::c_int;
        } else if delta == 0 {
            newclim /= 2 as libc::c_int;
        } else if delta < added {
            newclim = 9 as libc::c_int * newclim / 10 as libc::c_int;
        }
        if newclim < minclim {
            clim = minclim;
        } else if newclim > maxclim {
            clim = maxclim;
        } else {
            clim = newclim;
        }
        if newclim < oldclim {
            declims += 1;
            declims;
        }
        if newclim > oldclim {
            inclims += 1;
            inclims;
        }
    }
    if oldclim != clim {
        vrb(
            b"new conflict limit set to %d\0" as *const u8 as *const libc::c_char,
            clim,
        );
    } else {
        vrb(
            b"conflict limit remains at %d\0" as *const u8 as *const libc::c_char,
            clim,
        );
    }
    sumclims += clim as int64_t;
    simplified = 0 as libc::c_int;
    deleted = simplified;
    added = deleted;
}
unsafe extern "C" fn flush() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut before: libc::c_int = numnodes;
    let mut flushed: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut node: *mut Node = 0 as *mut Node;
    i = 0 as libc::c_int;
    while i < numnodes {
        node = *nodes.offset(i as isize);
        if (*node).res == 20 as libc::c_int {
            enqnewleafromnode(node);
            delnode(node);
        } else if lglinconsistent((*node).lgl) != 0 {
            mmsg(b"inconsistent\0" as *const u8 as *const libc::c_char, node);
            enqnewleafromnode(node);
            delnode(node);
        } else {
            i += 1;
            i;
        }
    }
    flushed = before - numnodes;
    if flushed != 0 {
        vrb(
            b"flushed %d nodes\0" as *const u8 as *const libc::c_char,
            flushed,
        );
    }
    if numnodes != 0 {
        res = 0 as libc::c_int;
        vrb(
            b"still %d nodes left\0" as *const u8 as *const libc::c_char,
            numnodes,
        );
    } else {
        res = 20 as libc::c_int;
        msg(b"no more nodes left\0" as *const u8 as *const libc::c_char);
        printf(b"s UNSATISFIABLE\n\0" as *const u8 as *const libc::c_char);
        fflush(stdout);
    }
    return res;
}
unsafe extern "C" fn mergestats() {
    let mut node: *mut Node = 0 as *mut Node;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < numnodes {
        node = *nodes.offset(i as isize);
        if !((*node).lgl).is_null() {
            updstats(node);
        }
        i += 1;
        i;
    }
    lockparstats();
    if noparallel == 0 && !(parallel.lgl).is_null() {
        decisions += lglgetdecs(parallel.lgl) - parallel.decisions;
        conflicts += lglgetconfs(parallel.lgl) - parallel.conflicts;
        propagations += lglgetprops(parallel.lgl) - parallel.propagations;
    }
    unlockparstats();
}
unsafe extern "C" fn stats() {
    let mut w: libc::c_double = getime();
    let mut t: libc::c_double = lglprocesstime();
    msg(b"\0" as *const u8 as *const libc::c_char);
    msg(
        b"%d rounds, %lld nodes (%d max), %lld threads (%d max)\0" as *const u8
            as *const libc::c_char,
        round,
        ids as libc::c_longlong,
        maxnumnodes,
        (noparallel == 0) as libc::c_int as libc::c_longlong + threads as libc::c_longlong,
        maxnumworkers + (noparallel == 0) as libc::c_int,
    );
    msg(
        b"%.0f avg clim, %lld incs, %lld decs, %lld forced, %lld simp\0" as *const u8
            as *const libc::c_char,
        avg(
            sumclims as libc::c_double,
            (inclims + declims + 1 as libc::c_int as int64_t) as libc::c_double,
        ),
        inclims as libc::c_longlong,
        declims as libc::c_longlong,
        forcedclims as libc::c_longlong,
        sumsimplified as libc::c_longlong,
    );
    msg(b"\0" as *const u8 as *const libc::c_char);
    msg(
        b"%.2f wall clock time, %.2f process time\0" as *const u8 as *const libc::c_char,
        w,
        t,
    );
    msg(
        b"%.0f%% utilization for %d%s worker threads on %d cores\0" as *const u8
            as *const libc::c_char,
        pcnt(
            t,
            (maxworkers + (noparallel == 0) as libc::c_int) as libc::c_double * w,
        ),
        maxworkers,
        if noparallel == 0 {
            b" + 1\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        ncores,
    );
    msg(b"\0" as *const u8 as *const libc::c_char);
    if noparallel == 0 {
        msg(
            b"started %d additional parallel solver instances\0" as *const u8
                as *const libc::c_char,
            nparallel,
        );
        msg(
            b"%lld units consumed %.0f%% of %lld produced\0" as *const u8 as *const libc::c_char,
            parallel.consumed.units as libc::c_longlong,
            pcnt(
                parallel.consumed.units as libc::c_double,
                parallel.produced.units as libc::c_double,
            ),
            parallel.produced.units as libc::c_longlong,
        );
        msg(
            b"%lld leaf clauses consumed %.0f%% of %lld produced\0" as *const u8
                as *const libc::c_char,
            parallel.consumed.leafs as libc::c_longlong,
            pcnt(
                parallel.consumed.leafs as libc::c_double,
                parallel.produced.leafs as libc::c_double,
            ),
            parallel.produced.leafs as libc::c_longlong,
        );
    }
    msg(b"\0" as *const u8 as *const libc::c_char);
    msg(
        b"%lld conflicts, %.0f conflicts per second\0" as *const u8 as *const libc::c_char,
        conflicts as libc::c_longlong,
        avg(conflicts as libc::c_double, w),
    );
    msg(
        b"%lld decisions, %.0f decisions per second\0" as *const u8 as *const libc::c_char,
        decisions as libc::c_longlong,
        avg(decisions as libc::c_double, w),
    );
    msg(
        b"%lld propagations, %.1f million propagations per second\0" as *const u8
            as *const libc::c_char,
        propagations as libc::c_longlong,
        avg(propagations as libc::c_double / 1e6f64, w),
    );
    msg(b"\0" as *const u8 as *const libc::c_char);
    msg(
        b"%lld tree-based look-ahead %.0f%% out of %d look-aheads\0" as *const u8
            as *const libc::c_char,
        treelkhd,
        pcnt(treelkhd as libc::c_double, totalkhd as libc::c_double),
        totalkhd,
    );
    let mut otherlkhdstr: *const libc::c_char = 0 as *const libc::c_char;
    if relevancelkhd != 0 {
        otherlkhdstr = b"relevance\0" as *const u8 as *const libc::c_char;
    } else if locslkhd != 0 {
        otherlkhdstr = b"local-search\0" as *const u8 as *const libc::c_char;
    } else {
        otherlkhdstr = b"JWH\0" as *const u8 as *const libc::c_char;
    }
    msg(
        b"%lld %s based look-ahead %.0f%% out of %d look-aheads\0" as *const u8
            as *const libc::c_char,
        totalkhd - treelkhd,
        otherlkhdstr,
        pcnt(
            (totalkhd - treelkhd) as libc::c_double,
            totalkhd as libc::c_double,
        ),
        totalkhd,
    );
    msg(b"\0" as *const u8 as *const libc::c_char);
    msg(
        b"%7d %3.0f%% lookaheads      %7.2f seconds %4.0f%%\0" as *const u8 as *const libc::c_char,
        js.lkhd,
        pcnt(js.lkhd as libc::c_double, js.cnt as libc::c_double),
        wct.lkhd,
        pcnt(wct.lkhd, w),
    );
    msg(
        b"%7d %3.0f%% splits          %7.2f seconds %4.0f%%\0" as *const u8 as *const libc::c_char,
        js.split,
        pcnt(js.split as libc::c_double, js.cnt as libc::c_double),
        wct.split,
        pcnt(wct.split, w),
    );
    msg(
        b"%7d %3.0f%% simplifications %7.2f seconds %4.0f%%\0" as *const u8 as *const libc::c_char,
        js.simp,
        pcnt(js.simp as libc::c_double, js.cnt as libc::c_double),
        wct.simp,
        pcnt(wct.simp, w),
    );
    msg(
        b"%7d %3.0f%% searches        %7.2f seconds %4.0f%%\0" as *const u8 as *const libc::c_char,
        js.search,
        pcnt(js.search as libc::c_double, js.cnt as libc::c_double),
        wct.search,
        pcnt(wct.search, w),
    );
    msg(
        b"======================================================\0" as *const u8
            as *const libc::c_char,
    );
    msg(
        b"%7d 100%% scheduled jobs  %7.2f seconds, %.0f MB\0" as *const u8 as *const libc::c_char,
        js.cnt,
        w,
        maxbytes as libc::c_double / ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_double,
    );
}
unsafe extern "C" fn parallelwins(mut res: libc::c_int) {
    msg(
        b"%s parallel solver instance wins with result %d after %.2f seconds\0" as *const u8
            as *const libc::c_char,
        if nparallel == 1 as libc::c_int {
            b"first\0" as *const u8 as *const libc::c_char
        } else {
            b"second\0" as *const u8 as *const libc::c_char
        },
        res,
        getime(),
    );
    if res == 10 as libc::c_int {
        printf(b"s SATISFIABLE\n\0" as *const u8 as *const libc::c_char);
        if nowitness == 0 {
            fflush(stdout);
            if nparallel > 1 as libc::c_int {
                lgljoin(root, parallel.lgl);
                witness(root);
            } else {
                witness(parallel.lgl);
            }
        }
    } else if res == 20 as libc::c_int {
        printf(b"s UNSATISFIABLE\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"c s UNKNOWN\n\0" as *const u8 as *const libc::c_char);
        abort();
    }
    fflush(stdout);
}
unsafe extern "C" fn finish() -> libc::c_int {
    let mut node: *mut Node = 0 as *mut Node;
    let mut i: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    res = flush();
    if res != 0 {
        return res;
    }
    i = 0 as libc::c_int;
    while i < numnodes {
        node = *nodes.offset(i as isize);
        if (*node).res == 10 as libc::c_int {
            break;
        }
        i += 1;
        i;
    }
    if i < numnodes {
        vrb(b"node %d sat\0" as *const u8 as *const libc::c_char, i);
        res = 10 as libc::c_int;
        node = *nodes.offset(i as isize);
        msg(
            b"winner [%d %lld] satisfiable in round %d after %lld nodes and %.2f seconds\0"
                as *const u8 as *const libc::c_char,
            (*node).depth,
            (*node).id as libc::c_longlong,
            round,
            ids as libc::c_longlong,
            getime(),
        );
        printf(b"s SATISFIABLE\n\0" as *const u8 as *const libc::c_char);
        if nowitness == 0 {
            fflush(stdout);
            lgljoin(root, (*node).lgl);
            witness(root);
        }
        fflush(stdout);
    } else if noparallel == 0 {
        lockdone();
        res = parallel.res;
        unlockdone();
        vrb(
            b"no node result thus parallel result %d\0" as *const u8 as *const libc::c_char,
            res,
        );
        if res != 0 {
            parallelwins(res);
        }
    } else {
        vrb(
            b"no satisfiable node found but still %d nodes left in round %d\0" as *const u8
                as *const libc::c_char,
            numnodes,
            round,
        );
    }
    return res;
}
static mut catchedsig: libc::c_int = 0;
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
        mergestats();
        stats();
        caughtsigmsg(sig);
    }
    resetsighandlers();
    raise(sig);
}
unsafe extern "C" fn setsighandlers() {
    sig_int_handler = signal(
        2 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    sig_segv_handler = signal(
        11 as libc::c_int,
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
unsafe extern "C" fn init() {
    wct.epoch = currentime();
    pthread_mutex_init(&mut lock.confs.mutex, 0 as *const pthread_mutexattr_t);
    pthread_mutex_init(&mut lock.done.mutex, 0 as *const pthread_mutexattr_t);
    pthread_mutex_init(&mut lock.leafs.mutex, 0 as *const pthread_mutexattr_t);
    pthread_mutex_init(&mut lock.mem.mutex, 0 as *const pthread_mutexattr_t);
    pthread_mutex_init(&mut lock.msg.mutex, 0 as *const pthread_mutexattr_t);
    pthread_mutex_init(&mut lock.nodes.mutex, 0 as *const pthread_mutexattr_t);
    pthread_mutex_init(&mut lock.parleafs.mutex, 0 as *const pthread_mutexattr_t);
    pthread_mutex_init(&mut lock.parstats.mutex, 0 as *const pthread_mutexattr_t);
    pthread_mutex_init(&mut lock.parunits.mutex, 0 as *const pthread_mutexattr_t);
    pthread_mutex_init(&mut lock.simplified.mutex, 0 as *const pthread_mutexattr_t);
    pthread_mutex_init(&mut lock.stats.mutex, 0 as *const pthread_mutexattr_t);
    pthread_mutex_init(&mut lock.workers.mutex, 0 as *const pthread_mutexattr_t);
    pthread_cond_init(&mut workerscond, 0 as *const pthread_condattr_t);
}
unsafe extern "C" fn has(
    mut str: *const libc::c_char,
    mut suffix: *const libc::c_char,
) -> libc::c_int {
    let mut l: libc::c_int = strlen(str) as libc::c_int;
    let mut k: libc::c_int = strlen(suffix) as libc::c_int;
    if l < k {
        return 0 as libc::c_int;
    }
    return (strcmp(str.offset(l as isize).offset(-(k as isize)), suffix) == 0) as libc::c_int;
}
unsafe extern "C" fn cmd(mut fmt: *const libc::c_char, mut name: *const libc::c_char) -> *mut FILE {
    let mut res: *mut FILE = 0 as *mut FILE;
    let mut s: *mut libc::c_char = malloc(
        (strlen(fmt))
            .wrapping_add(strlen(name))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    sprintf(s, fmt, name);
    res = popen(s, b"r\0" as *const u8 as *const libc::c_char);
    free(s as *mut libc::c_void);
    return res;
}
unsafe extern "C" fn parselopt(
    mut arg: *const libc::c_char,
    mut resptr: *mut libc::c_int,
    mut opt: *const libc::c_char,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    if *arg.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
        && *arg.offset(1 as libc::c_int as isize) as libc::c_int != '-' as i32
    {
        return 0 as libc::c_int;
    }
    p = arg.offset(2 as libc::c_int as isize);
    q = opt;
    while *p as libc::c_int == *q as libc::c_int {
        p = p.offset(1);
        p;
        q = q.offset(1);
        q;
    }
    if *q != 0 {
        return 0 as libc::c_int;
    }
    let fresh17 = p;
    p = p.offset(1);
    if *fresh17 as libc::c_int != '=' as i32 {
        return 0 as libc::c_int;
    }
    if *resptr != 0 {
        err(
            b"multiple '--%s=...' options\0" as *const u8 as *const libc::c_char,
            opt,
        );
    }
    if isnum(p) == 0 {
        err(
            b"expected number as argument in '%s'\0" as *const u8 as *const libc::c_char,
            arg,
        );
    }
    *resptr = atoi(p);
    if *resptr <= 0 as libc::c_int {
        err(
            b"expected positive number in '%s'\0" as *const u8 as *const libc::c_char,
            arg,
        );
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn parseu64(
    mut arg: *const libc::c_char,
    mut opt: *const libc::c_char,
) -> libc::c_int {
    let mut res: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut p: *const libc::c_char = arg;
    let mut ch: libc::c_int = 0;
    let fresh18 = p;
    p = p.offset(1);
    ch = *fresh18 as libc::c_int;
    if ch == 0 {
        err(
            b"empty string argument to '%s'\0" as *const u8 as *const libc::c_char,
            opt,
        );
    }
    res = (ch - '0' as i32) as libc::c_ulonglong;
    loop {
        let fresh19 = p;
        p = p.offset(1);
        ch = *fresh19 as libc::c_int;
        if !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0)
        {
            break;
        }
        let mut current_block_9: u64;
        let mut digit: libc::c_int = 0;
        if (!(0 as libc::c_ulonglong) >> 1 as libc::c_int)
            .wrapping_div(10 as libc::c_int as libc::c_ulonglong)
            < res
        {
            current_block_9 = 5801668696158960501;
        } else {
            current_block_9 = 4906268039856690917;
        }
        loop {
            match current_block_9 {
                5801668696158960501 => {
                    err(
                        b"argument to '%s' too large\0" as *const u8 as *const libc::c_char,
                        opt,
                    );
                    current_block_9 = 4906268039856690917;
                }
                _ => {
                    res = res.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    digit = ch - '0' as i32;
                    if (!(0 as libc::c_ulonglong) >> 1 as libc::c_int)
                        .wrapping_sub(digit as libc::c_ulonglong)
                        < res
                    {
                        current_block_9 = 5801668696158960501;
                    } else {
                        break;
                    }
                }
            }
        }
        res = res.wrapping_add(digit as libc::c_ulonglong);
    }
    if ch != 0 {
        err(
            b"invalid unsigned 64 bit number in '%s %s'\0" as *const u8 as *const libc::c_char,
            opt,
            arg,
        );
    }
    return res as libc::c_int;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut seed: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut i: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut clf: libc::c_int = 0;
    let mut node: *mut Node = 0 as *mut Node;
    let mut leaf: *mut Leaf = 0 as *mut Leaf;
    let mut sec: libc::c_double = 0.;
    init();
    i = 1 as libc::c_int;
    while i < argc {
        if strcmp(
            *argv.offset(i as isize),
            b"-h\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            usage();
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
            b"-S\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            showstats = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-n\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            nowitness = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-O\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            optimize = 1 as libc::c_int;
        } else if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32
            && *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize) as libc::c_int
                == 'O' as i32
            && isnum((*argv.offset(i as isize)).offset(2 as libc::c_int as isize)) != 0
        {
            if optimize >= 0 as libc::c_int {
                err(b"multiple '-O...' options\0" as *const u8 as *const libc::c_char);
            }
            optimize = atoi((*argv.offset(i as isize)).offset(2 as libc::c_int as isize));
            if optimize < 0 as libc::c_int {
                err(
                    b"invalid number in '%s'\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
        } else if strcmp(
            *argv.offset(i as isize),
            b"--reduce\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            reducecache = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--locslkhd\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            locslkhd = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--no-relevancelkhd\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            relevancelkhd = 0 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--force-simp\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            forcesimp = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--no-simp\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            nosimp = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--no-search\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            nosearch = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--no-parallel\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            noparallel = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--no-full\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            fullint = 0 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-f\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            i += 1;
            if i == argc {
                err(b"argument to '-f' missing\0" as *const u8 as *const libc::c_char);
            }
            if isnum(*argv.offset(i as isize)) == 0 || {
                fullint = atoi(*argv.offset(i as isize));
                fullint < 0 as libc::c_int
            } {
                err(
                    b"expected (non negative) number as argument in '-f %s'\0" as *const u8
                        as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
        } else if strcmp(
            *argv.offset(i as isize),
            b"--balance\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            balance = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--symmetric\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            asymmetric = 0 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--asymmetric\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            asymmetric = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--portfolio\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            portfolio = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--eager\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            eager = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--lazy\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            eager = 0 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-m\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if hardlimbytes != 0 {
                err(b"multiple memory limit specification\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
            if i == argc {
                err(b"argument to '-m' missing\0" as *const u8 as *const libc::c_char);
            }
            if isnum(*argv.offset(i as isize)) == 0 {
                err(
                    b"expected number as argument in '-m %s'\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
            hardlimbytes = atoll(*argv.offset(i as isize)) as size_t;
            if hardlimbytes > 0 as libc::c_int as size_t {
                hardlimbytes <<= 20 as libc::c_int;
            }
            if hardlimbytes <= 0 as libc::c_int as size_t {
                err(
                    b"invalid number of MB in '-m %s'\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
        } else if strcmp(
            *argv.offset(i as isize),
            b"-g\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if hardlimbytes != 0 {
                err(b"multiple memory limit specification\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
            if i == argc {
                err(b"argument to '-g' missing\0" as *const u8 as *const libc::c_char);
            }
            if isnum(*argv.offset(i as isize)) == 0 {
                err(
                    b"expected number as argument in '-g %s'\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
            hardlimbytes = atoll(*argv.offset(i as isize)) as size_t;
            if hardlimbytes > 0 as libc::c_int as size_t {
                hardlimbytes <<= 30 as libc::c_int;
            }
            if hardlimbytes <= 0 as libc::c_int as size_t {
                err(
                    b"invalid number of GB in '-g %s'\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
        } else if strcmp(
            *argv.offset(i as isize),
            b"-t\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if maxworkers > 0 as libc::c_int {
                err(b"multiple '-t <workers>' options\0" as *const u8 as *const libc::c_char);
            }
            if maxworkers2 > 0 as libc::c_int {
                err(b"both '-t <workers>' and '<workers>'\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
            if i == argc {
                err(b"argument to '-t' missing\0" as *const u8 as *const libc::c_char);
            }
            if isnum(*argv.offset(i as isize)) == 0 {
                err(
                    b"expected number as argument in '-t %s'\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
            maxworkers = atoi(*argv.offset(i as isize));
            if maxworkers <= 0 as libc::c_int {
                err(
                    b"expected positive number in '-t %s'\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
        } else if strcmp(
            *argv.offset(i as isize),
            b"-a\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if maxactive > 0 as libc::c_int {
                err(b"multiple '-a <nodes>' options\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
            if i == argc {
                err(b"argument to '-a' missing\0" as *const u8 as *const libc::c_char);
            }
            if isnum(*argv.offset(i as isize)) == 0 {
                err(
                    b"expected number as argument in '-a %s'\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
            maxactive = atoi(*argv.offset(i as isize));
            if maxactive <= 0 as libc::c_int {
                err(
                    b"expected positive number in '-a %s'\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
        } else if strcmp(
            *argv.offset(i as isize),
            b"-b\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if branches >= 0 as libc::c_int {
                err(b"multiple '-b <branches>' options\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
            if i == argc {
                err(b"argument to '-b' missing\0" as *const u8 as *const libc::c_char);
            }
            if isnum(*argv.offset(i as isize)) == 0 {
                err(
                    b"expected number as argument in '-b %s'\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
            branches = atoi(*argv.offset(i as isize));
            if branches <= 0 as libc::c_int {
                err(
                    b"expected positive number in '-b %s'\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
            if branches > 100 as libc::c_int {
                err(
                    b"expected number not larger than 100 in '-b %s'\0" as *const u8
                        as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
        } else if strcmp(
            *argv.offset(i as isize),
            b"-r\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if randswap > 0 as libc::c_int {
                err(b"multiple '-r <posnum>' options\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
            if i == argc {
                err(b"argument to '-r' missing\0" as *const u8 as *const libc::c_char);
            }
            if isnum(*argv.offset(i as isize)) == 0 {
                err(
                    b"expected number as argument in '-r %s'\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
            randswap = atoi(*argv.offset(i as isize));
            if randswap <= 0 as libc::c_int {
                err(
                    b"expected positive number in '-r %s'\0" as *const u8 as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
        } else if strcmp(
            *argv.offset(i as isize),
            b"-s\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            i += 1;
            if i == argc {
                err(b"argument to '-s' missing (try '-h')\0" as *const u8 as *const libc::c_char);
            }
            seed = parseu64(
                *argv.offset(i as isize),
                b"-s\0" as *const u8 as *const libc::c_char,
            ) as libc::c_ulonglong;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--treelook=-1\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            treelookdepth = -(1 as libc::c_int);
            treelookdepthset = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"--treelook=0\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            treelookdepth = 0 as libc::c_int;
            treelookdepthset = 1 as libc::c_int;
        } else if parselopt(
            *argv.offset(i as isize),
            &mut treelookdepth,
            b"treelook\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            treelookdepthset = 1 as libc::c_int;
        } else if !(parselopt(
            *argv.offset(i as isize),
            &mut minclim,
            b"min\0" as *const u8 as *const libc::c_char,
        ) != 0)
        {
            if !(parselopt(
                *argv.offset(i as isize),
                &mut initclim,
                b"init\0" as *const u8 as *const libc::c_char,
            ) != 0)
            {
                if !(parselopt(
                    *argv.offset(i as isize),
                    &mut maxclim,
                    b"max\0" as *const u8 as *const libc::c_char,
                ) != 0)
                {
                    if **argv.offset(i as isize) as libc::c_int == '-' as i32 {
                        err(
                            b"invalid command line option '%s' (try '-h')\0" as *const u8
                                as *const libc::c_char,
                            *argv.offset(i as isize),
                        );
                    } else if fname.is_null() && isnum(*argv.offset(i as isize)) != 0 {
                        err(
                            b"<file> file name can not be a positive number '%s'\0" as *const u8
                                as *const libc::c_char,
                            *argv.offset(i as isize),
                        );
                    } else if !fname.is_null() && maxworkers2 != 0 {
                        err(
                            b"too many arguments (including <file> and <workers>)\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else if !fname.is_null() && isnum(*argv.offset(i as isize)) == 0 {
                        err(
                            b"expected positive number for <workers> but got '%s'\0" as *const u8
                                as *const libc::c_char,
                            *argv.offset(i as isize),
                        );
                    } else if !fname.is_null() {
                        maxworkers2 = atoi(*argv.offset(i as isize));
                        if maxworkers2 <= 0 as libc::c_int {
                            err(
                                b"invalid number '%s' for <workers>\0" as *const u8
                                    as *const libc::c_char,
                                *argv.offset(i as isize),
                            );
                        }
                    } else if exists(*argv.offset(i as isize)) == 0 {
                        err(
                            b"can not stat file '%s'\0" as *const u8 as *const libc::c_char,
                            *argv.offset(i as isize),
                        );
                    } else {
                        fname = *argv.offset(i as isize);
                    }
                }
            }
        }
        i += 1;
        i;
    }
    if locslkhd != 0 && relevancelkhd != 0 {
        err(
            b"can not combine '--relevancelkhd' and '--locslkhd'\0" as *const u8
                as *const libc::c_char,
        );
    }
    if treelookdepthset == 0 {
        treelookdepth = 10 as libc::c_int;
    }
    if treelookdepth < 0 as libc::c_int {
        treelookdepth = -(1 as libc::c_int);
    }
    if maxworkers2 != 0 {
        maxworkers = maxworkers2;
    }
    if fname.is_null() {
        file = stdin;
        fname = b"<stdin>\0" as *const u8 as *const libc::c_char;
        clf = 0 as libc::c_int;
    } else if has(fname, b".gz\0" as *const u8 as *const libc::c_char) != 0 {
        file = cmd(b"gunzip -c %s\0" as *const u8 as *const libc::c_char, fname);
        clf = 2 as libc::c_int;
    } else if has(fname, b".xz\0" as *const u8 as *const libc::c_char) != 0 {
        file = cmd(b"xz -d -c %s\0" as *const u8 as *const libc::c_char, fname);
        clf = 2 as libc::c_int;
    } else if has(fname, b".bz2\0" as *const u8 as *const libc::c_char) != 0 {
        file = cmd(b"bzcat %s\0" as *const u8 as *const libc::c_char, fname);
        clf = 2 as libc::c_int;
    } else if has(fname, b".7z\0" as *const u8 as *const libc::c_char) != 0 {
        file = cmd(
            b"7z x -so %s 2>/dev/null\0" as *const u8 as *const libc::c_char,
            fname,
        );
        clf = 2 as libc::c_int;
    } else {
        file = fopen(fname, b"r\0" as *const u8 as *const libc::c_char);
        clf = 1 as libc::c_int;
    }
    if file.is_null() {
        err(
            b"can not read '%s'\0" as *const u8 as *const libc::c_char,
            fname,
        );
    }
    lglbnr(
        b"Treengeling Cube and Conquer SAT Solver\0" as *const u8 as *const libc::c_char,
        b"c \0" as *const u8 as *const libc::c_char,
        stdout,
    );
    msg(
        b"verbose level %d\0" as *const u8 as *const libc::c_char,
        verbose,
    );
    msg(
        b"will %sprint statistics for each solver instance\0" as *const u8 as *const libc::c_char,
        if showstats != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"NOT \0" as *const u8 as *const libc::c_char
        },
    );
    msg(
        b"will %sprint satisfying assignment\0" as *const u8 as *const libc::c_char,
        if nowitness != 0 {
            b"NOT \0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    msg(b"\0" as *const u8 as *const libc::c_char);
    if balance != 0 {
        msg(b"splitting large nodes first ('-b' option)\0" as *const u8 as *const libc::c_char);
    } else {
        msg(b"splitting small nodes first (no '-b' option)\0" as *const u8 as *const libc::c_char);
    }
    if asymmetric != 0 {
        msg(b"asymmetric splitting ('--asymmetric')\0" as *const u8 as *const libc::c_char);
    } else {
        msg(b"symmetric splitting ('--symmetric')\0" as *const u8 as *const libc::c_char);
    }
    if portfolio != 0 {
        msg(
            b"portfolio option fuzzing enabled ('--portfolio'\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        msg(
            b"portfolio option fuzzing disabled (no '--portfolio' option)\0" as *const u8
                as *const libc::c_char,
        );
    }
    if fullint != 0 {
        msg(
            b"full search/simplification round interval %d\0" as *const u8 as *const libc::c_char,
            fullint,
        );
    } else {
        msg(b"no full search/simplification\0" as *const u8 as *const libc::c_char);
    }
    if treelookdepthset < 0 as libc::c_int {
        msg(b"no tree-based look-ahead at all\0" as *const u8 as *const libc::c_char);
    } else {
        msg(
            b"tree-based look-ahead up to depth %d\0" as *const u8 as *const libc::c_char,
            treelookdepth,
        );
    }
    if relevancelkhd != 0 {
        msg(
            b"relevance based look-ahead starts at depth %d\0" as *const u8 as *const libc::c_char,
            treelookdepth + 1 as libc::c_int,
        );
    } else if locslkhd != 0 {
        msg(
            b"local-search based look-ahead starts at depth %d\0" as *const u8
                as *const libc::c_char,
            treelookdepth + 1 as libc::c_int,
        );
    } else {
        msg(
            b"JWH based look-ahead starts at depth %d\0" as *const u8 as *const libc::c_char,
            treelookdepth + 1 as libc::c_int,
        );
    }
    if randswap != 0 {
        msg(
            b"will swap %d nodes randomly during lookahead\0" as *const u8 as *const libc::c_char,
            randswap,
        );
        msg(
            b"random seed %llu\0" as *const u8 as *const libc::c_char,
            seed,
        );
        mysrand(seed);
    } else {
        msg(b"no randomization in lookahead\0" as *const u8 as *const libc::c_char);
    }
    msg(b"\0" as *const u8 as *const libc::c_char);
    ncores = getcores(1 as libc::c_int);
    if maxworkers == 0 {
        maxworkers = ncores;
        msg(
            b"maximum %d workers (no '-t <worker>' option)\0" as *const u8 as *const libc::c_char,
            maxworkers,
        );
    } else if maxworkers2 != 0 {
        msg(
            b"maximum %d workers as specified ('%d')\0" as *const u8 as *const libc::c_char,
            maxworkers,
            maxworkers2,
        );
    } else {
        msg(
            b"maximum %d workers as specified ('-t %d')\0" as *const u8 as *const libc::c_char,
            maxworkers,
            maxworkers,
        );
    }
    if noparallel != 0 {
        msg(
            b"not using additional parallel solver instances ('--no-parallel')\0" as *const u8
                as *const libc::c_char,
        );
    } else if maxworkers == 1 as libc::c_int {
        msg(
            b"not using additional parallel solver instances ('-t 1')\0" as *const u8
                as *const libc::c_char,
        );
        noparallel = 1 as libc::c_int;
    } else {
        msg(
            b"using one worker for additional parallel solver instance\0" as *const u8
                as *const libc::c_char,
        );
        maxworkers -= 1;
        maxworkers;
    }
    if maxactive == 0 {
        maxactive = 8 as libc::c_int * maxworkers;
        msg(
            b"maximum of %d active nodes (no '-a <nodes>' option)\0" as *const u8
                as *const libc::c_char,
            maxactive,
            maxactive,
        );
    } else {
        msg(
            b"maximum of %d active nodes as specified ('-a %d')\0" as *const u8
                as *const libc::c_char,
            maxactive,
            maxactive,
        );
    }
    if branches >= 0 as libc::c_int {
        msg(
            b"ratio of split nodes is %d%% as specified ('-b %d')\0" as *const u8
                as *const libc::c_char,
            branches,
            branches,
        );
    } else {
        branches = 50 as libc::c_int;
        msg(
            b"default ratio of split nodes is %d%%\0" as *const u8 as *const libc::c_char,
            branches,
        );
    }
    msg(b"\0" as *const u8 as *const libc::c_char);
    if hardlimbytes == 0 {
        hardlimbytes = getotalmem(1 as libc::c_int) as size_t;
        msg(
            b"hard memory limit of %d MB (no '-g' nor '-m' option)\0" as *const u8
                as *const libc::c_char,
            bytes2mb(hardlimbytes),
        );
    } else {
        msg(
            b"hard memory limit %d MB as specified\0" as *const u8 as *const libc::c_char,
            bytes2mb(hardlimbytes),
        );
    }
    softlimbytes =
        hardlimbytes.wrapping_add(2 as libc::c_int as size_t) / 3 as libc::c_int as size_t;
    msg(
        b"soft memory limit of %d MB\0" as *const u8 as *const libc::c_char,
        bytes2mb(softlimbytes),
    );
    msg(b"\0" as *const u8 as *const libc::c_char);
    if minclim == 0 {
        minclim = 1000 as libc::c_int;
        msg(
            b"default minimum conflict limit of %d conflicts\0" as *const u8 as *const libc::c_char,
            minclim,
        );
    } else {
        msg(
            b"minimum conflict limit set to %d ('--min=%d')\0" as *const u8 as *const libc::c_char,
            minclim,
            minclim,
        );
    }
    if initclim == 0 {
        initclim = 10000 as libc::c_int;
        msg(
            b"default initial conflict limit of %d conflicts\0" as *const u8 as *const libc::c_char,
            initclim,
        );
    } else {
        msg(
            b"initial conflict limit set to %d ('--init=%d')\0" as *const u8 as *const libc::c_char,
            initclim,
            initclim,
        );
    }
    if maxclim == 0 {
        maxclim = 100000 as libc::c_int;
        msg(
            b"default maximum conflict limit of %d conflicts\0" as *const u8 as *const libc::c_char,
            maxclim,
        );
    } else {
        msg(
            b"maximum conflict limit set to %d ('--max=%d')\0" as *const u8 as *const libc::c_char,
            maxclim,
            maxclim,
        );
    }
    msg(b"\0" as *const u8 as *const libc::c_char);
    if optimize < 0 as libc::c_int {
        optimize = 10 as libc::c_int;
        msg(
            b"default optimization level %d\0" as *const u8 as *const libc::c_char,
            optimize,
        );
    } else {
        msg(
            b"optimization level %d as specified ('-O%d')\0" as *const u8 as *const libc::c_char,
            optimize,
            optimize,
        );
    }
    msg(b"\0" as *const u8 as *const libc::c_char);
    setsighandlers();
    initroot();
    msg(b"reading %s\0" as *const u8 as *const libc::c_char, fname);
    parse(root);
    if clf == 1 as libc::c_int {
        fclose(file);
    }
    if clf == 2 as libc::c_int {
        pclose(file);
    }
    if noparallel == 0 {
        startparallel(root);
    }
    msg(
        b"simplifying root solver instance with optimization level %d\0" as *const u8
            as *const libc::c_char,
        optimize,
    );
    startimer(&mut wct.simp);
    res = lglsimp(root, optimize);
    sec = stoptimer();
    js.cnt += 1;
    js.cnt;
    js.simp += 1;
    js.simp;
    propagations += lglgetprops(root);
    msg(
        b"root solver instance optimization with result %d took %.2f seconds\0" as *const u8
            as *const libc::c_char,
        res,
        sec,
    );
    tmp = 0 as libc::c_int;
    if noparallel == 0 {
        tmp = joinparallel();
        if res == 0 && tmp != 0 {
            parallelwins(tmp);
        }
        releaseparallel();
    }
    if res == 0 && tmp != 0 {
        res = tmp;
    } else if res == 10 as libc::c_int {
        printf(b"s SATISFIABLE\n\0" as *const u8 as *const libc::c_char);
        if nowitness == 0 {
            fflush(stdout);
            witness(root);
        }
        fflush(stdout);
    } else if res == 20 as libc::c_int {
        printf(b"s UNSATISFIABLE\n\0" as *const u8 as *const libc::c_char);
        fflush(stdout);
    } else {
        node = newnode(0 as *mut Node, 0 as libc::c_int);
        if noparallel == 0 {
            startparallel((*node).lgl);
        }
        lglsetopt(
            (*node).lgl,
            b"block\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        clim = initclim;
        sumclims += clim as int64_t;
        report();
        res = flush();
        if res == 0 {
            search();
            loop {
                res = finish();
                if !(res == 0) {
                    break;
                }
                incround();
                simp();
                res = flush();
                if res != 0 {
                    break;
                }
                lookahead();
                split();
                res = flush();
                if res != 0 {
                    break;
                }
                report();
                updateclim();
                search();
            }
        }
        if noparallel == 0 {
            joinparallel();
            releaseparallel();
        }
        msg(b"\0" as *const u8 as *const libc::c_char);
        msg(
            b"cleaning up after %d rounds\0" as *const u8 as *const libc::c_char,
            round,
        );
        while numnodes != 0 {
            delnode(*nodes.offset(0 as libc::c_int as isize));
        }
        loop {
            leaf = deqleaf();
            if leaf.is_null() {
                break;
            }
            deleaf(leaf);
        }
    }
    lglrelease(root);
    resetsighandlers();
    stats();
    let mut BYTES: size_t = (sizejobs as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<*mut Job>() as libc::c_ulong);
    decmem(BYTES);
    free(jobs as *mut libc::c_void);
    let mut BYTES_0: size_t = (sizenodes as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<*mut Node>() as libc::c_ulong);
    decmem(BYTES_0);
    free(nodes as *mut libc::c_void);
    let mut BYTES_1: size_t = (sizeconfstack as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<int64_t>() as libc::c_ulong);
    decmem(BYTES_1);
    free(confstack as *mut libc::c_void);
    msg(b"\0" as *const u8 as *const libc::c_char);
    msg(b"result %d\0" as *const u8 as *const libc::c_char, res);
    return res;
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
unsafe extern "C" fn run_static_initializers() {
    nopts = (::core::mem::size_of::<[Opt; 3]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<Opt>() as libc::c_ulong) as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
