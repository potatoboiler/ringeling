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
pub struct __va_list_tag<'h0,'h1> {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: &'h0 (libc::c_void),
    // 159: overflow_arg_area: typeof(overflow_arg_area) = *mut {g241} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 159: overflow_arg_area:   g241 = UNIQUE | NON_NULL, (empty)
    pub reg_save_area: &'h1 (libc::c_void),
    // 160: reg_save_area: typeof(reg_save_area) = *mut {g242} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 160: reg_save_area:   g242 = UNIQUE | NON_NULL, (empty)
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
    // 174: _IO_read_ptr: typeof(_IO_read_ptr) = *mut {g243} i8
    // 174: _IO_read_ptr:   g243 = UNIQUE | NON_NULL, FIXED
    pub _IO_read_end: *mut libc::c_char,
    // 175: _IO_read_end: typeof(_IO_read_end) = *mut {g244} i8
    // 175: _IO_read_end:   g244 = UNIQUE | NON_NULL, FIXED
    pub _IO_read_base: *mut libc::c_char,
    // 176: _IO_read_base: typeof(_IO_read_base) = *mut {g245} i8
    // 176: _IO_read_base:   g245 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_base: *mut libc::c_char,
    // 177: _IO_write_base: typeof(_IO_write_base) = *mut {g246} i8
    // 177: _IO_write_base:   g246 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_ptr: *mut libc::c_char,
    // 178: _IO_write_ptr: typeof(_IO_write_ptr) = *mut {g247} i8
    // 178: _IO_write_ptr:   g247 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_end: *mut libc::c_char,
    // 179: _IO_write_end: typeof(_IO_write_end) = *mut {g248} i8
    // 179: _IO_write_end:   g248 = UNIQUE | NON_NULL, FIXED
    pub _IO_buf_base: *mut libc::c_char,
    // 180: _IO_buf_base: typeof(_IO_buf_base) = *mut {g249} i8
    // 180: _IO_buf_base:   g249 = UNIQUE | NON_NULL, FIXED
    pub _IO_buf_end: *mut libc::c_char,
    // 181: _IO_buf_end: typeof(_IO_buf_end) = *mut {g250} i8
    // 181: _IO_buf_end:   g250 = UNIQUE | NON_NULL, FIXED
    pub _IO_save_base: *mut libc::c_char,
    // 182: _IO_save_base: typeof(_IO_save_base) = *mut {g251} i8
    // 182: _IO_save_base:   g251 = UNIQUE | NON_NULL, FIXED
    pub _IO_backup_base: *mut libc::c_char,
    // 183: _IO_backup_base: typeof(_IO_backup_base) = *mut {g252} i8
    // 183: _IO_backup_base:   g252 = UNIQUE | NON_NULL, FIXED
    pub _IO_save_end: *mut libc::c_char,
    // 184: _IO_save_end: typeof(_IO_save_end) = *mut {g253} i8
    // 184: _IO_save_end:   g253 = UNIQUE | NON_NULL, FIXED
    pub _markers: *mut _IO_marker,
    // 185: _markers: typeof(_markers) = *mut {g254} _IO_marker
    // 185: _markers:   g254 = UNIQUE | NON_NULL, FIXED
    pub _chain: *mut _IO_FILE,
    // 186: _chain: typeof(_chain) = *mut {g255} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 186: _chain:   g255 = UNIQUE | NON_NULL, FIXED
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    // 193: _lock: typeof(_lock) = *mut {g256} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 193: _lock:   g256 = UNIQUE | NON_NULL, FIXED
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    // 195: _codecvt: typeof(_codecvt) = *mut {g257} _IO_codecvt
    // 195: _codecvt:   g257 = UNIQUE | NON_NULL, FIXED
    pub _wide_data: *mut _IO_wide_data,
    // 196: _wide_data: typeof(_wide_data) = *mut {g258} _IO_wide_data
    // 196: _wide_data:   g258 = UNIQUE | NON_NULL, FIXED
    pub _freeres_list: *mut _IO_FILE,
    // 197: _freeres_list: typeof(_freeres_list) = *mut {g259} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 197: _freeres_list:   g259 = UNIQUE | NON_NULL, FIXED
    pub _freeres_buf: *mut libc::c_void,
    // 198: _freeres_buf: typeof(_freeres_buf) = *mut {g260} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 198: _freeres_buf:   g260 = UNIQUE | NON_NULL, FIXED
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
    // 216: __prev: typeof(__prev) = *mut {g261} DefId(0:492 ~ plingeling[18f5]::__pthread_internal_list)
    // 216: __prev:   g261 = UNIQUE | NON_NULL, FIXED
    pub __next: *mut __pthread_internal_list,
    // 217: __next: typeof(__next) = *mut {g262} DefId(0:492 ~ plingeling[18f5]::__pthread_internal_list)
    // 217: __next:   g262 = UNIQUE | NON_NULL, FIXED
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
pub struct Worker<'h2,'h3,'h4> {
    pub lgl: *mut LGL,
    // 506: lgl: typeof(lgl) = *mut {g263} LGL
    // 506: lgl:   g263 = UNIQUE | NON_NULL, FIXED
    pub thread: pthread_t,
    pub res: libc::c_int,
    pub fixed: libc::c_int,
    pub units: [libc::c_int; 512],
    pub nunits: libc::c_int,
    pub cls: *mut libc::c_int,
    // 512: cls: typeof(cls) = *mut {g264} i32
    // 512: cls:   g264 = UNIQUE | NON_NULL, FIXED
    pub szcls: libc::c_int,
    pub clsimported: libc::c_long,
    pub dead: *mut Cls,
    // 515: dead: typeof(dead) = *mut {g265} DefId(0:525 ~ plingeling[18f5]::Cls)
    // 515: dead:   g265 = UNIQUE | NON_NULL, FIXED
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
pub struct C2RustUnnamed_5<'h5,'h6> {
    pub start: *mut *mut Cls,
    // 543: start: typeof(start) = *mut {g266} *mut {g267} DefId(0:525 ~ plingeling[18f5]::Cls)
    // 543: start:   g266 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | FREE | NON_NULL, FIXED
    // 543: start:   g267 = READ | FREE, FIXED
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
// 564: mut __nptr: typeof(_1) = *const {g0} i8
// 564: mut __nptr:   g0 = READ | OFFSET_ADD, FIXED
    return strtol(
        __nptr,
        // 566: __nptr: typeof(_4) = *const {l4} i8
        // 566: __nptr:   l4 = READ | OFFSET_ADD, (empty)
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        // 567: 0 as *mut libc: ... _char: typeof(_5) = *mut {l6} *mut {g50} i8
        // 567: 0 as *mut libc: ... _char:   l6 = WRITE | UNIQUE, (empty)
        // 567: 0 as *mut libc: ... _char:   g50 = WRITE | OFFSET_ADD, CELL | FIXED
        // 567: 0 as *mut libc: ... _void: typeof(_6) = *mut {l8} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 567: 0 as *mut libc: ... _void:   l8 = WRITE | UNIQUE, (empty)
        // 567: 0 as *mut libc: ... _void: typeof(_6 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l11} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 567: 0 as *mut libc: ... _void:   l11 = WRITE | UNIQUE, (empty)
        // 567: 0 as *mut libc: ... _char: typeof(_5 = move _6 as *mut *mut i8 (Misc)) = *mut {l12} *mut {g50} i8
        // 567: 0 as *mut libc: ... _char:   l12 = WRITE | UNIQUE, (empty)
        // 567: 0 as *mut libc: ... _char:   g50 = WRITE | OFFSET_ADD, CELL | FIXED
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char) -> libc::c_longlong {
// 572: mut __nptr: typeof(_1) = *const {g1} i8
// 572: mut __nptr:   g1 = READ | UNIQUE | OFFSET_ADD | NON_NULL, FIXED
    return strtoll(
        __nptr,
        // 574: __nptr: typeof(_3) = *const {l3} i8
        // 574: __nptr:   l3 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        // 575: 0 as *mut libc: ... _char: typeof(_4) = *mut {l5} *mut {g53} i8
        // 575: 0 as *mut libc: ... _char:   l5 = WRITE | UNIQUE, (empty)
        // 575: 0 as *mut libc: ... _char:   g53 = WRITE | OFFSET_ADD, CELL | FIXED
        // 575: 0 as *mut libc: ... _void: typeof(_5) = *mut {l7} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 575: 0 as *mut libc: ... _void:   l7 = WRITE | UNIQUE, (empty)
        // 575: 0 as *mut libc: ... _void: typeof(_5 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l10} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 575: 0 as *mut libc: ... _void:   l10 = WRITE | UNIQUE, (empty)
        // 575: 0 as *mut libc: ... _char: typeof(_4 = move _5 as *mut *mut i8 (Misc)) = *mut {l11} *mut {g53} i8
        // 575: 0 as *mut libc: ... _char:   l11 = WRITE | UNIQUE, (empty)
        // 575: 0 as *mut libc: ... _char:   g53 = WRITE | OFFSET_ADD, CELL | FIXED
        10 as libc::c_int,
    );
}
static verbose: libc::c_int = 0;
static plain: libc::c_int = 0;
static nounits: libc::c_int = 0;
static noeqs: libc::c_int = 0;
static nocls: libc::c_int = 0;
static gclim: libc::c_int = 1 as libc::c_int;
static gclimset: libc::c_int = 0;
static nworkers: libc::c_int = 0;
static nconsumers: libc::c_int = 0;
static leavebehind: libc::c_int = 0;
static locs: libc::c_int = 0;
static memlimit: int64_t = 0;
static softmemlimit: int64_t = 0;
static workers: *mut Worker = 0 as *const Worker as *mut Worker;
static nvars: libc::c_int = 0;
static nclauses: libc::c_int = 0;
static vals: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static fixed: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static repr: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
#[no_mangle]
pub static mut clauses: C2RustUnnamed_5 = C2RustUnnamed_5 {
    start: 0 as *const *mut Cls as *mut *mut Cls,
    first: 0,
    num: 0,
    added: 0,
    collected: 0,
    size: 0,
};
static nfixed: libc::c_int = 0;
static globalres: libc::c_int = 0;
static mut gcs: libc::c_int = 0;
static name: *const libc::c_char = 0 as *const libc::c_char;
static nworkers2: libc::c_int = 0;
#[no_mangle]
static mem: C2RustUnnamed_6 = C2RustUnnamed_6 { max: 0, current: 0 };
static catchedsig: libc::c_int = 0;
static start: libc::c_double = 0.;
static file: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
static syncs: C2RustUnnamed_7 = C2RustUnnamed_7 {
    units: 0,
    cls: 0,
    eqs: 0,
};
static done: libc::c_int = 0;
static termchks: libc::c_int = 0;
static units: libc::c_int = 0;
static eqs: libc::c_int = 0;
static flushed: libc::c_int = 0;
static donemutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static msgmutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static fixedmutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static reprmutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static memutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static clsmutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
unsafe extern "C" fn currentime() -> libc::c_double {
    let mut res: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    if gettimeofday(&mut tv, 0 as *mut libc::c_void) == 0 {
    // 760: &mut tv: typeof(_8) = *mut {l8} DefId(0:486 ~ plingeling[18f5]::timeval)
    // 760: &mut tv:   l8 = UNIQUE | NON_NULL, (empty)
    // 760: &mut tv: typeof(_9) = &mut {l10} DefId(0:486 ~ plingeling[18f5]::timeval)
    // 760: &mut tv:   l10 = UNIQUE | NON_NULL, (empty)
    // 760: 0 as *mut libc: ... _void: typeof(_10) = *mut {l12} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 760: 0 as *mut libc: ... _void:   l12 = UNIQUE | NON_NULL, (empty)
    // 760: &mut tv: typeof(_9 = &mut _4) = &mut {l18} DefId(0:486 ~ plingeling[18f5]::timeval)
    // 760: &mut tv:   l18 = UNIQUE | NON_NULL, (empty)
    // 760: 0 as *mut libc: ... _void: typeof(_10 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l20} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 760: 0 as *mut libc: ... _void:   l20 = UNIQUE | NON_NULL, (empty)
    // 760: &mut tv: typeof(_8 = &raw mut (*_9)) = *mut {l19} DefId(0:486 ~ plingeling[18f5]::timeval)
    // 760: &mut tv:   l19 = UNIQUE | NON_NULL, (empty)
        res = 1e-6f64 * tv.tv_usec as libc::c_double;
        res += tv.tv_sec as libc::c_double;
    }
    return res;
}
unsafe extern "C" fn getime() -> libc::c_double {
    return currentime() - start;
    // 767: start: typeof(_4) = *mut {l4} f64
    // 767: start:   l4 = READ | UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn msg(
    mut wid: libc::c_int,
    mut level: libc::c_int,
    mut fmt: *const libc::c_char,
    // 772: mut fmt: typeof(_3) = *const {g2} i8
    // 772: mut fmt:   g2 = UNIQUE | NON_NULL, FIXED
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    if verbose < level {
    // 776: verbose: typeof(_9) = *mut {l9} i32
    // 776: verbose:   l9 = UNIQUE | NON_NULL, (empty)
        return;
    }
    pthread_mutex_lock(&mut msgmutex);
    // 779: &mut msgmutex: typeof(_13) = *mut {l14} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 779: &mut msgmutex:   l14 = UNIQUE | NON_NULL, (empty)
    // 779: &mut msgmutex: typeof(_14) = &mut {l16} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 779: &mut msgmutex:   l16 = UNIQUE | NON_NULL, (empty)
    // 779: msgmutex: typeof(_15) = *mut {l18} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 779: msgmutex:   l18 = UNIQUE | NON_NULL, (empty)
    // 779: &mut msgmutex: typeof(_13 = &raw mut (*_14)) = *mut {l94} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 779: &mut msgmutex:   l94 = UNIQUE | NON_NULL, (empty)
    // 779: &mut msgmutex: typeof(_14 = &mut (*_15)) = &mut {l93} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 779: &mut msgmutex:   l93 = UNIQUE | NON_NULL, (empty)
    fflush(stderr);
    // 780: stderr: typeof(_17) = *mut {l21} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 780: stderr:   l21 = UNIQUE | NON_NULL, (empty)
    // 780: stderr: typeof(_18) = *mut {l23} *mut {l24} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 780: stderr:   l23 = UNIQUE | NON_NULL, (empty)
    // 780: stderr:   l24 = UNIQUE | NON_NULL, (empty)
    if wid < 0 as libc::c_int {
        printf(b"c - \0" as *const u8 as *const libc::c_char);
        // 782: b"c - \0" as *c ... _char: typeof(_24) = *const {l31} i8
        // 782: b"c - \0" as *c ... _char:   l31 = UNIQUE | NON_NULL, (empty)
        // 782: b"c - \0" as *c ... st u8: typeof(_25) = *const {l33} u8
        // 782: b"c - \0" as *c ... st u8:   l33 = UNIQUE | NON_NULL, (empty)
        // 782: b"c - \0": typeof(_26) = *const {l35} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 782: b"c - \0":   l35 = UNIQUE | NON_NULL, (empty)
        // 782: b"c - \0": typeof(_27) = & {l37} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 782: b"c - \0":   l37 = UNIQUE | NON_NULL, FIXED
        // 782: b"c - \0": typeof(_26 = &raw const (*_27)) = *const {l96} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 782: b"c - \0":   l96 = UNIQUE | NON_NULL, (empty)
        // 782: b"c - \0": typeof(_27 = const b"c - \x00") = & {l95} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 782: b"c - \0":   l95 = UNIQUE | NON_NULL, (empty)
        // 782: b"c - \0" as *c ... st u8: typeof(_25 = move _26 as *const u8 (Pointer(ArrayToPointer))) = *const {l97} u8
        // 782: b"c - \0" as *c ... st u8:   l97 = UNIQUE | NON_NULL, (empty)
        // 782: b"c - \0" as *c ... _char: typeof(_24 = move _25 as *const i8 (Misc)) = *const {l98} i8
        // 782: b"c - \0" as *c ... _char:   l98 = UNIQUE | NON_NULL, (empty)
    } else {
        printf(b"c %d \0" as *const u8 as *const libc::c_char, wid);
        // 784: b"c %d \0" as * ... _char: typeof(_29) = *const {l40} i8
        // 784: b"c %d \0" as * ... _char:   l40 = UNIQUE | NON_NULL, (empty)
        // 784: b"c %d \0" as * ... st u8: typeof(_30) = *const {l42} u8
        // 784: b"c %d \0" as * ... st u8:   l42 = UNIQUE | NON_NULL, (empty)
        // 784: b"c %d \0": typeof(_31) = *const {l44} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 784: b"c %d \0":   l44 = UNIQUE | NON_NULL, (empty)
        // 784: b"c %d \0": typeof(_32) = & {l46} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 784: b"c %d \0":   l46 = UNIQUE | NON_NULL, FIXED
        // 784: b"c %d \0": typeof(_32 = const b"c %d \x00") = & {l99} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 784: b"c %d \0":   l99 = UNIQUE | NON_NULL, (empty)
        // 784: b"c %d \0" as * ... _char: typeof(_29 = move _30 as *const i8 (Misc)) = *const {l102} i8
        // 784: b"c %d \0" as * ... _char:   l102 = UNIQUE | NON_NULL, (empty)
        // 784: b"c %d \0": typeof(_31 = &raw const (*_32)) = *const {l100} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 784: b"c %d \0":   l100 = UNIQUE | NON_NULL, (empty)
        // 784: b"c %d \0" as * ... st u8: typeof(_30 = move _31 as *const u8 (Pointer(ArrayToPointer))) = *const {l101} u8
        // 784: b"c %d \0" as * ... st u8:   l101 = UNIQUE | NON_NULL, (empty)
    }
    printf(b"W %6.1f \0" as *const u8 as *const libc::c_char, getime());
    // 786: b"W %6.1f \0" a ... _char: typeof(_35) = *const {l50} i8
    // 786: b"W %6.1f \0" a ... _char:   l50 = UNIQUE | NON_NULL, (empty)
    // 786: b"W %6.1f \0" a ... st u8: typeof(_36) = *const {l52} u8
    // 786: b"W %6.1f \0" a ... st u8:   l52 = UNIQUE | NON_NULL, (empty)
    // 786: b"W %6.1f \0": typeof(_37) = *const {l54} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
    // 786: b"W %6.1f \0":   l54 = UNIQUE | NON_NULL, (empty)
    // 786: b"W %6.1f \0": typeof(_38) = & {l56} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
    // 786: b"W %6.1f \0":   l56 = UNIQUE | NON_NULL, FIXED
    // 786: b"W %6.1f \0" a ... _char: typeof(_35 = move _36 as *const i8 (Misc)) = *const {l106} i8
    // 786: b"W %6.1f \0" a ... _char:   l106 = UNIQUE | NON_NULL, (empty)
    // 786: b"W %6.1f \0" a ... st u8: typeof(_36 = move _37 as *const u8 (Pointer(ArrayToPointer))) = *const {l105} u8
    // 786: b"W %6.1f \0" a ... st u8:   l105 = UNIQUE | NON_NULL, (empty)
    // 786: b"W %6.1f \0": typeof(_38 = const b"W %6.1f \x00") = & {l103} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
    // 786: b"W %6.1f \0":   l103 = UNIQUE | NON_NULL, (empty)
    // 786: b"W %6.1f \0": typeof(_37 = &raw const (*_38)) = *const {l104} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
    // 786: b"W %6.1f \0":   l104 = UNIQUE | NON_NULL, (empty)
    ap = args.clone();
    // 787: args.clone(): typeof(_41) = & {l60} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 787: args.clone():   l60 = UNIQUE | NON_NULL, (empty)
    // 787: args.clone(): typeof(_41 = &_4) = & {l107} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 787: args.clone():   l107 = UNIQUE | NON_NULL, (empty)
    vfprintf(stdout, fmt, ap.as_va_list());
    // 788: stdout: typeof(_43) = *mut {l63} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 788: stdout:   l63 = UNIQUE | NON_NULL, (empty)
    // 788: stdout: typeof(_44) = *mut {l65} *mut {l66} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 788: stdout:   l65 = UNIQUE | NON_NULL, (empty)
    // 788: stdout:   l66 = UNIQUE | NON_NULL, (empty)
    // 788: fmt: typeof(_45) = *const {l68} i8
    // 788: fmt:   l68 = UNIQUE | NON_NULL, (empty)
    // 788: ap.as_va_list(): typeof(_47) = &mut {l71} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 788: ap.as_va_list():   l71 = UNIQUE | NON_NULL, (empty)
    // 788: ap.as_va_list(): typeof(_47 = &mut _5) = &mut {l108} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 788: ap.as_va_list():   l108 = UNIQUE | NON_NULL, (empty)
    fputc('\n' as i32, stdout);
    // 789: stdout: typeof(_50) = *mut {l75} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 789: stdout:   l75 = UNIQUE | NON_NULL, (empty)
    // 789: stdout: typeof(_51) = *mut {l77} *mut {l78} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 789: stdout:   l77 = UNIQUE | NON_NULL, (empty)
    // 789: stdout:   l78 = UNIQUE | NON_NULL, (empty)
    fflush(stdout);
    // 790: stdout: typeof(_53) = *mut {l81} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 790: stdout:   l81 = UNIQUE | NON_NULL, (empty)
    // 790: stdout: typeof(_54) = *mut {l83} *mut {l84} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 790: stdout:   l83 = UNIQUE | NON_NULL, (empty)
    // 790: stdout:   l84 = UNIQUE | NON_NULL, (empty)
    pthread_mutex_unlock(&mut msgmutex);
    // 791: &mut msgmutex: typeof(_56) = *mut {l87} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 791: &mut msgmutex:   l87 = UNIQUE | NON_NULL, (empty)
    // 791: &mut msgmutex: typeof(_57) = &mut {l89} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 791: &mut msgmutex:   l89 = UNIQUE | NON_NULL, (empty)
    // 791: msgmutex: typeof(_58) = *mut {l91} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 791: msgmutex:   l91 = UNIQUE | NON_NULL, (empty)
    // 791: &mut msgmutex: typeof(_56 = &raw mut (*_57)) = *mut {l110} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 791: &mut msgmutex:   l110 = UNIQUE | NON_NULL, (empty)
    // 791: &mut msgmutex: typeof(_57 = &mut (*_58)) = &mut {l109} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 791: &mut msgmutex:   l109 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn die(mut fmt: *const libc::c_char, mut args: ...) {
// 793: mut fmt: typeof(_1) = *const {g3} i8
// 793: mut fmt:   g3 = UNIQUE | NON_NULL, FIXED
    let mut ap: ::core::ffi::VaListImpl;
    fflush(stdout);
    // 795: stdout: typeof(_6) = *mut {l6} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 795: stdout:   l6 = UNIQUE | NON_NULL, (empty)
    // 795: stdout: typeof(_7) = *mut {l8} *mut {l9} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 795: stdout:   l8 = UNIQUE | NON_NULL, (empty)
    // 795: stdout:   l9 = UNIQUE | NON_NULL, (empty)
    fputs(
        b"c *** plingeling error: \0" as *const u8 as *const libc::c_char,
        // 797: b"c *** plingel ... _char: typeof(_9) = *const {l12} i8
        // 797: b"c *** plingel ... _char:   l12 = UNIQUE | NON_NULL, (empty)
        // 797: b"c *** plingel ... st u8: typeof(_10) = *const {l14} u8
        // 797: b"c *** plingel ... st u8:   l14 = UNIQUE | NON_NULL, (empty)
        // 797: b"c *** plingel ... : \0": typeof(_11) = *const {l16} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
        // 797: b"c *** plingel ... : \0":   l16 = UNIQUE | NON_NULL, (empty)
        // 797: b"c *** plingel ... : \0": typeof(_12) = & {l18} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
        // 797: b"c *** plingel ... : \0":   l18 = UNIQUE | NON_NULL, FIXED
        // 797: b"c *** plingel ... st u8: typeof(_10 = move _11 as *const u8 (Pointer(ArrayToPointer))) = *const {l56} u8
        // 797: b"c *** plingel ... st u8:   l56 = UNIQUE | NON_NULL, (empty)
        // 797: b"c *** plingel ... _char: typeof(_9 = move _10 as *const i8 (Misc)) = *const {l57} i8
        // 797: b"c *** plingel ... _char:   l57 = UNIQUE | NON_NULL, (empty)
        // 797: b"c *** plingel ... : \0": typeof(_11 = &raw const (*_12)) = *const {l55} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
        // 797: b"c *** plingel ... : \0":   l55 = UNIQUE | NON_NULL, (empty)
        // 797: b"c *** plingel ... : \0": typeof(_12 = const b"c *** plingeling error: \x00") = & {l54} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
        // 797: b"c *** plingel ... : \0":   l54 = UNIQUE | NON_NULL, (empty)
        stderr,
        // 798: stderr: typeof(_13) = *mut {l20} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 798: stderr:   l20 = UNIQUE | NON_NULL, (empty)
        // 798: stderr: typeof(_14) = *mut {l22} *mut {l23} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 798: stderr:   l22 = UNIQUE | NON_NULL, (empty)
        // 798: stderr:   l23 = UNIQUE | NON_NULL, (empty)
    );
    ap = args.clone();
    // 800: args.clone(): typeof(_16) = & {l26} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 800: args.clone():   l26 = UNIQUE | NON_NULL, (empty)
    // 800: args.clone(): typeof(_16 = &_2) = & {l58} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 800: args.clone():   l58 = UNIQUE | NON_NULL, (empty)
    vfprintf(stderr, fmt, ap.as_va_list());
    // 801: stderr: typeof(_18) = *mut {l29} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 801: stderr:   l29 = UNIQUE | NON_NULL, (empty)
    // 801: stderr: typeof(_19) = *mut {l31} *mut {l32} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 801: stderr:   l31 = UNIQUE | NON_NULL, (empty)
    // 801: stderr:   l32 = UNIQUE | NON_NULL, (empty)
    // 801: fmt: typeof(_20) = *const {l34} i8
    // 801: fmt:   l34 = UNIQUE | NON_NULL, (empty)
    // 801: ap.as_va_list(): typeof(_22) = &mut {l37} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 801: ap.as_va_list():   l37 = UNIQUE | NON_NULL, (empty)
    // 801: ap.as_va_list(): typeof(_22 = &mut _4) = &mut {l59} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 801: ap.as_va_list():   l59 = UNIQUE | NON_NULL, (empty)
    fputc('\n' as i32, stderr);
    // 802: stderr: typeof(_25) = *mut {l41} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 802: stderr:   l41 = UNIQUE | NON_NULL, (empty)
    // 802: stderr: typeof(_26) = *mut {l43} *mut {l44} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 802: stderr:   l43 = UNIQUE | NON_NULL, (empty)
    // 802: stderr:   l44 = UNIQUE | NON_NULL, (empty)
    fflush(stderr);
    // 803: stderr: typeof(_28) = *mut {l47} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 803: stderr:   l47 = UNIQUE | NON_NULL, (empty)
    // 803: stderr: typeof(_29) = *mut {l49} *mut {l50} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 803: stderr:   l49 = UNIQUE | NON_NULL, (empty)
    // 803: stderr:   l50 = UNIQUE | NON_NULL, (empty)
    exit(1 as libc::c_int);
}
unsafe extern "C" fn warn(mut fmt: *const libc::c_char, mut args: ...) {
// 806: mut fmt: typeof(_1) = *const {g4} i8
// 806: mut fmt:   g4 = UNIQUE | NON_NULL, FIXED
    let mut ap: ::core::ffi::VaListImpl;
    fflush(stdout);
    // 808: stdout: typeof(_5) = *mut {l5} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 808: stdout:   l5 = UNIQUE | NON_NULL, (empty)
    // 808: stdout: typeof(_6) = *mut {l7} *mut {l8} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 808: stdout:   l7 = UNIQUE | NON_NULL, (empty)
    // 808: stdout:   l8 = UNIQUE | NON_NULL, (empty)
    fputs(
        b"c *** plingeling warning: \0" as *const u8 as *const libc::c_char,
        // 810: b"c *** plingel ... _char: typeof(_8) = *const {l11} i8
        // 810: b"c *** plingel ... _char:   l11 = UNIQUE | NON_NULL, (empty)
        // 810: b"c *** plingel ... st u8: typeof(_9) = *const {l13} u8
        // 810: b"c *** plingel ... st u8:   l13 = UNIQUE | NON_NULL, (empty)
        // 810: b"c *** plingel ... : \0": typeof(_10) = *const {l15} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 810: b"c *** plingel ... : \0":   l15 = UNIQUE | NON_NULL, (empty)
        // 810: b"c *** plingel ... : \0": typeof(_11) = & {l17} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 810: b"c *** plingel ... : \0":   l17 = UNIQUE | NON_NULL, FIXED
        // 810: b"c *** plingel ... st u8: typeof(_9 = move _10 as *const u8 (Pointer(ArrayToPointer))) = *const {l53} u8
        // 810: b"c *** plingel ... st u8:   l53 = UNIQUE | NON_NULL, (empty)
        // 810: b"c *** plingel ... : \0": typeof(_10 = &raw const (*_11)) = *const {l52} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 810: b"c *** plingel ... : \0":   l52 = UNIQUE | NON_NULL, (empty)
        // 810: b"c *** plingel ... _char: typeof(_8 = move _9 as *const i8 (Misc)) = *const {l54} i8
        // 810: b"c *** plingel ... _char:   l54 = UNIQUE | NON_NULL, (empty)
        // 810: b"c *** plingel ... : \0": typeof(_11 = const b"c *** plingeling warning: \x00") = & {l51} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 810: b"c *** plingel ... : \0":   l51 = UNIQUE | NON_NULL, (empty)
        stderr,
        // 811: stderr: typeof(_12) = *mut {l19} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 811: stderr:   l19 = UNIQUE | NON_NULL, (empty)
        // 811: stderr: typeof(_13) = *mut {l21} *mut {l22} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 811: stderr:   l21 = UNIQUE | NON_NULL, (empty)
        // 811: stderr:   l22 = UNIQUE | NON_NULL, (empty)
    );
    ap = args.clone();
    // 813: args.clone(): typeof(_15) = & {l25} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 813: args.clone():   l25 = UNIQUE | NON_NULL, (empty)
    // 813: args.clone(): typeof(_15 = &_2) = & {l55} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 813: args.clone():   l55 = UNIQUE | NON_NULL, (empty)
    vfprintf(stderr, fmt, ap.as_va_list());
    // 814: stderr: typeof(_17) = *mut {l28} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 814: stderr:   l28 = UNIQUE | NON_NULL, (empty)
    // 814: stderr: typeof(_18) = *mut {l30} *mut {l31} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 814: stderr:   l30 = UNIQUE | NON_NULL, (empty)
    // 814: stderr:   l31 = UNIQUE | NON_NULL, (empty)
    // 814: fmt: typeof(_19) = *const {l33} i8
    // 814: fmt:   l33 = UNIQUE | NON_NULL, (empty)
    // 814: ap.as_va_list(): typeof(_21) = &mut {l36} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 814: ap.as_va_list():   l36 = UNIQUE | NON_NULL, (empty)
    // 814: ap.as_va_list(): typeof(_21 = &mut _3) = &mut {l56} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 814: ap.as_va_list():   l56 = UNIQUE | NON_NULL, (empty)
    fputc('\n' as i32, stderr);
    // 815: stderr: typeof(_24) = *mut {l40} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 815: stderr:   l40 = UNIQUE | NON_NULL, (empty)
    // 815: stderr: typeof(_25) = *mut {l42} *mut {l43} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 815: stderr:   l42 = UNIQUE | NON_NULL, (empty)
    // 815: stderr:   l43 = UNIQUE | NON_NULL, (empty)
    fflush(stderr);
    // 816: stderr: typeof(_27) = *mut {l46} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 816: stderr:   l46 = UNIQUE | NON_NULL, (empty)
    // 816: stderr: typeof(_28) = *mut {l48} *mut {l49} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 816: stderr:   l48 = UNIQUE | NON_NULL, (empty)
    // 816: stderr:   l49 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn percent(mut a: libc::c_double, mut b: libc::c_double) -> libc::c_double {
    return if b != 0. {
        100 as libc::c_int as libc::c_double * a / b
    } else {
        0 as libc::c_int as libc::c_double
    };
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
    let mut w: *mut Worker = 0 as *mut Worker;
    // 836: mut w: typeof(_11) = *mut {l11} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 836: mut w:   l11 = UNIQUE | NON_NULL, (empty)
    // 836: 0 as *mut Worker: typeof(_11 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l254} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 836: 0 as *mut Worker:   l254 = UNIQUE | NON_NULL, (empty)
    confs = 0 as libc::c_int as int64_t;
    decs = confs;
    unitcalls = decs as libc::c_int;
    props = 0 as libc::c_int as int64_t;
    mb = mem.max as libc::c_double / ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_double;
    // 841: mem: typeof(_18) = *mut {l19} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
    // 841: mem:   l19 = UNIQUE | NON_NULL, (empty)
    i = 0 as libc::c_int;
    while i < nworkers {
    // 843: nworkers: typeof(_30) = *mut {l32} i32
    // 843: nworkers:   l32 = UNIQUE | NON_NULL, (empty)
        w = workers.offset(i as isize);
        // 844: workers.offset( ... size): typeof(_31) = *mut {l34} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 844: workers.offset( ... size):   l34 = UNIQUE | NON_NULL, (empty)
        // 844: workers: typeof(_32) = *mut {l36} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 844: workers:   l36 = UNIQUE | NON_NULL, (empty)
        // 844: workers: typeof(_33) = *mut {l38} *mut {l39} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 844: workers:   l38 = UNIQUE | NON_NULL, (empty)
        // 844: workers:   l39 = UNIQUE | NON_NULL, (empty)
        if !((*w).lgl).is_null() {
        // 845: ((*w).lgl): typeof(_39) = *mut {l46} LGL
        // 845: ((*w).lgl):   l46 = UNIQUE | NON_NULL, (empty)
            decs += lglgetdecs((*w).lgl);
            // 846: (*w).lgl: typeof(_41) = *mut {l49} LGL
            // 846: (*w).lgl:   l49 = UNIQUE | NON_NULL, (empty)
            confs += lglgetconfs((*w).lgl);
            // 847: (*w).lgl: typeof(_44) = *mut {l53} LGL
            // 847: (*w).lgl:   l53 = UNIQUE | NON_NULL, (empty)
            props += lglgetprops((*w).lgl);
            // 848: (*w).lgl: typeof(_47) = *mut {l57} LGL
            // 848: (*w).lgl:   l57 = UNIQUE | NON_NULL, (empty)
            mb += lglmaxmb((*w).lgl);
            // 849: (*w).lgl: typeof(_50) = *mut {l61} LGL
            // 849: (*w).lgl:   l61 = UNIQUE | NON_NULL, (empty)
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
        // 868: b"c %d terminat ... _char: typeof(_80) = *const {l92} i8
        // 868: b"c %d terminat ... _char:   l92 = UNIQUE | NON_NULL, (empty)
        // 868: b"c %d terminat ... st u8: typeof(_81) = *const {l94} u8
        // 868: b"c %d terminat ... st u8:   l94 = UNIQUE | NON_NULL, (empty)
        // 868: b"c %d terminat ... \n\0": typeof(_82) = *const {l96} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
        // 868: b"c %d terminat ... \n\0":   l96 = UNIQUE | NON_NULL, (empty)
        // 868: b"c %d terminat ... \n\0": typeof(_83) = & {l98} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
        // 868: b"c %d terminat ... \n\0":   l98 = UNIQUE | NON_NULL, FIXED
        // 868: b"c %d terminat ... \n\0": typeof(_82 = &raw const (*_83)) = *const {l256} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
        // 868: b"c %d terminat ... \n\0":   l256 = UNIQUE | NON_NULL, (empty)
        // 868: b"c %d terminat ... _char: typeof(_80 = move _81 as *const i8 (Misc)) = *const {l258} i8
        // 868: b"c %d terminat ... _char:   l258 = UNIQUE | NON_NULL, (empty)
        // 868: b"c %d terminat ... \n\0": typeof(_83 = const b"c %d termination checks\n\x00") = & {l255} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
        // 868: b"c %d terminat ... \n\0":   l255 = UNIQUE | NON_NULL, (empty)
        // 868: b"c %d terminat ... st u8: typeof(_81 = move _82 as *const u8 (Pointer(ArrayToPointer))) = *const {l257} u8
        // 868: b"c %d terminat ... st u8:   l257 = UNIQUE | NON_NULL, (empty)
        termchks,
        // 869: termchks: typeof(_85) = *mut {l101} i32
        // 869: termchks:   l101 = UNIQUE | NON_NULL, (empty)
    );
    printf(b"c\n\0" as *const u8 as *const libc::c_char);
    // 871: b"c\n\0" as *co ... _char: typeof(_87) = *const {l104} i8
    // 871: b"c\n\0" as *co ... _char:   l104 = UNIQUE | NON_NULL, (empty)
    // 871: b"c\n\0" as *co ... st u8: typeof(_88) = *const {l106} u8
    // 871: b"c\n\0" as *co ... st u8:   l106 = UNIQUE | NON_NULL, (empty)
    // 871: b"c\n\0": typeof(_89) = *const {l108} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 871: b"c\n\0":   l108 = UNIQUE | NON_NULL, (empty)
    // 871: b"c\n\0": typeof(_90) = & {l110} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 871: b"c\n\0":   l110 = UNIQUE | NON_NULL, FIXED
    // 871: b"c\n\0" as *co ... st u8: typeof(_88 = move _89 as *const u8 (Pointer(ArrayToPointer))) = *const {l261} u8
    // 871: b"c\n\0" as *co ... st u8:   l261 = UNIQUE | NON_NULL, (empty)
    // 871: b"c\n\0" as *co ... _char: typeof(_87 = move _88 as *const i8 (Misc)) = *const {l262} i8
    // 871: b"c\n\0" as *co ... _char:   l262 = UNIQUE | NON_NULL, (empty)
    // 871: b"c\n\0": typeof(_89 = &raw const (*_90)) = *const {l260} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 871: b"c\n\0":   l260 = UNIQUE | NON_NULL, (empty)
    // 871: b"c\n\0": typeof(_90 = const b"c\n\x00") = & {l259} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 871: b"c\n\0":   l259 = UNIQUE | NON_NULL, (empty)
    printf(
        b"c units: %d found, %d publications, %d syncs, %d flushed\n\0" as *const u8
        // 873: b"c units: %d f ... _char: typeof(_92) = *const {l113} i8
        // 873: b"c units: %d f ... _char:   l113 = UNIQUE | NON_NULL, (empty)
        // 873: b"c units: %d f ... st u8: typeof(_93) = *const {l115} u8
        // 873: b"c units: %d f ... st u8:   l115 = UNIQUE | NON_NULL, (empty)
        // 873: b"c units: %d f ... \n\0": typeof(_94) = *const {l117} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003a)) }]
        // 873: b"c units: %d f ... \n\0":   l117 = UNIQUE | NON_NULL, (empty)
        // 873: b"c units: %d f ... \n\0": typeof(_95) = & {l119} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003a)) }]
        // 873: b"c units: %d f ... \n\0":   l119 = UNIQUE | NON_NULL, FIXED
        // 873: b"c units: %d f ... \n\0": typeof(_95 = const b"c units: %d found, %d publications, %d syncs, %d flushed\n\x00") = & {l263} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003a)) }]
        // 873: b"c units: %d f ... \n\0":   l263 = UNIQUE | NON_NULL, (empty)
        // 873: b"c units: %d f ... st u8: typeof(_93 = move _94 as *const u8 (Pointer(ArrayToPointer))) = *const {l265} u8
        // 873: b"c units: %d f ... st u8:   l265 = UNIQUE | NON_NULL, (empty)
        // 873: b"c units: %d f ... \n\0": typeof(_94 = &raw const (*_95)) = *const {l264} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003a)) }]
        // 873: b"c units: %d f ... \n\0":   l264 = UNIQUE | NON_NULL, (empty)
        // 873: b"c units: %d f ... _char: typeof(_92 = move _93 as *const i8 (Misc)) = *const {l266} i8
        // 873: b"c units: %d f ... _char:   l266 = UNIQUE | NON_NULL, (empty)
            as *const libc::c_char,
        units,
        // 875: units: typeof(_97) = *mut {l122} i32
        // 875: units:   l122 = UNIQUE | NON_NULL, (empty)
        unitcalls,
        syncs.units,
        // 877: syncs: typeof(_100) = *mut {l126} DefId(0:590 ~ plingeling[18f5]::C2RustUnnamed_7)
        // 877: syncs:   l126 = UNIQUE | NON_NULL, (empty)
        flushed,
        // 878: flushed: typeof(_102) = *mut {l129} i32
        // 878: flushed:   l129 = UNIQUE | NON_NULL, (empty)
    );
    printf(
        b"c clauses: %ld clauses added, %ld collected %.0f%%, %d gcs\n\0" as *const u8
        // 881: b"c clauses: %l ... _char: typeof(_104) = *const {l132} i8
        // 881: b"c clauses: %l ... _char:   l132 = UNIQUE | NON_NULL, (empty)
        // 881: b"c clauses: %l ... st u8: typeof(_105) = *const {l134} u8
        // 881: b"c clauses: %l ... st u8:   l134 = UNIQUE | NON_NULL, (empty)
        // 881: b"c clauses: %l ... \n\0": typeof(_106) = *const {l136} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003c)) }]
        // 881: b"c clauses: %l ... \n\0":   l136 = UNIQUE | NON_NULL, (empty)
        // 881: b"c clauses: %l ... \n\0": typeof(_107) = & {l138} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003c)) }]
        // 881: b"c clauses: %l ... \n\0":   l138 = UNIQUE | NON_NULL, FIXED
        // 881: b"c clauses: %l ... st u8: typeof(_105 = move _106 as *const u8 (Pointer(ArrayToPointer))) = *const {l269} u8
        // 881: b"c clauses: %l ... st u8:   l269 = UNIQUE | NON_NULL, (empty)
        // 881: b"c clauses: %l ... \n\0": typeof(_107 = const b"c clauses: %ld clauses added, %ld collected %.0f%%, %d gcs\n\x00") = & {l267} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003c)) }]
        // 881: b"c clauses: %l ... \n\0":   l267 = UNIQUE | NON_NULL, (empty)
        // 881: b"c clauses: %l ... \n\0": typeof(_106 = &raw const (*_107)) = *const {l268} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003c)) }]
        // 881: b"c clauses: %l ... \n\0":   l268 = UNIQUE | NON_NULL, (empty)
        // 881: b"c clauses: %l ... _char: typeof(_104 = move _105 as *const i8 (Misc)) = *const {l270} i8
        // 881: b"c clauses: %l ... _char:   l270 = UNIQUE | NON_NULL, (empty)
            as *const libc::c_char,
        clauses.added,
        // 883: clauses: typeof(_109) = *mut {l141} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
        // 883: clauses:   l141 = UNIQUE | NON_NULL, (empty)
        clauses.collected,
        // 884: clauses: typeof(_111) = *mut {l144} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
        // 884: clauses:   l144 = UNIQUE | NON_NULL, (empty)
        percent(
            clauses.collected as libc::c_double,
            // 886: clauses: typeof(_115) = *mut {l149} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
            // 886: clauses:   l149 = UNIQUE | NON_NULL, (empty)
            clauses.added as libc::c_double,
            // 887: clauses: typeof(_118) = *mut {l153} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
            // 887: clauses:   l153 = UNIQUE | NON_NULL, (empty)
        ),
        gcs,
        // 889: gcs: typeof(_120) = *mut {l156} i32
        // 889: gcs:   l156 = UNIQUE | NON_NULL, (empty)
    );
    printf(
        b"c equivalences: %d found, %d syncs\n\0" as *const u8 as *const libc::c_char,
        // 892: b"c equivalence ... _char: typeof(_122) = *const {l159} i8
        // 892: b"c equivalence ... _char:   l159 = UNIQUE | NON_NULL, (empty)
        // 892: b"c equivalence ... st u8: typeof(_123) = *const {l161} u8
        // 892: b"c equivalence ... st u8:   l161 = UNIQUE | NON_NULL, (empty)
        // 892: b"c equivalence ... \n\0": typeof(_124) = *const {l163} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
        // 892: b"c equivalence ... \n\0":   l163 = UNIQUE | NON_NULL, (empty)
        // 892: b"c equivalence ... \n\0": typeof(_125) = & {l165} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
        // 892: b"c equivalence ... \n\0":   l165 = UNIQUE | NON_NULL, FIXED
        // 892: b"c equivalence ... _char: typeof(_122 = move _123 as *const i8 (Misc)) = *const {l274} i8
        // 892: b"c equivalence ... _char:   l274 = UNIQUE | NON_NULL, (empty)
        // 892: b"c equivalence ... \n\0": typeof(_125 = const b"c equivalences: %d found, %d syncs\n\x00") = & {l271} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
        // 892: b"c equivalence ... \n\0":   l271 = UNIQUE | NON_NULL, (empty)
        // 892: b"c equivalence ... \n\0": typeof(_124 = &raw const (*_125)) = *const {l272} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
        // 892: b"c equivalence ... \n\0":   l272 = UNIQUE | NON_NULL, (empty)
        // 892: b"c equivalence ... st u8: typeof(_123 = move _124 as *const u8 (Pointer(ArrayToPointer))) = *const {l273} u8
        // 892: b"c equivalence ... st u8:   l273 = UNIQUE | NON_NULL, (empty)
        eqs,
        // 893: eqs: typeof(_127) = *mut {l168} i32
        // 893: eqs:   l168 = UNIQUE | NON_NULL, (empty)
        syncs.eqs,
        // 894: syncs: typeof(_129) = *mut {l171} DefId(0:590 ~ plingeling[18f5]::C2RustUnnamed_7)
        // 894: syncs:   l171 = UNIQUE | NON_NULL, (empty)
    );
    printf(b"c\n\0" as *const u8 as *const libc::c_char);
    // 896: b"c\n\0" as *co ... _char: typeof(_131) = *const {l174} i8
    // 896: b"c\n\0" as *co ... _char:   l174 = UNIQUE | NON_NULL, (empty)
    // 896: b"c\n\0" as *co ... st u8: typeof(_132) = *const {l176} u8
    // 896: b"c\n\0" as *co ... st u8:   l176 = UNIQUE | NON_NULL, (empty)
    // 896: b"c\n\0": typeof(_133) = *const {l178} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 896: b"c\n\0":   l178 = UNIQUE | NON_NULL, (empty)
    // 896: b"c\n\0": typeof(_134) = & {l180} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 896: b"c\n\0":   l180 = UNIQUE | NON_NULL, FIXED
    // 896: b"c\n\0" as *co ... _char: typeof(_131 = move _132 as *const i8 (Misc)) = *const {l278} i8
    // 896: b"c\n\0" as *co ... _char:   l278 = UNIQUE | NON_NULL, (empty)
    // 896: b"c\n\0": typeof(_134 = const b"c\n\x00") = & {l275} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 896: b"c\n\0":   l275 = UNIQUE | NON_NULL, (empty)
    // 896: b"c\n\0": typeof(_133 = &raw const (*_134)) = *const {l276} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 896: b"c\n\0":   l276 = UNIQUE | NON_NULL, (empty)
    // 896: b"c\n\0" as *co ... st u8: typeof(_132 = move _133 as *const u8 (Pointer(ArrayToPointer))) = *const {l277} u8
    // 896: b"c\n\0" as *co ... st u8:   l277 = UNIQUE | NON_NULL, (empty)
    printf(
        b"c %lld decisions, %lld conflicts, %.1f conflicts/sec\n\0" as *const u8
        // 898: b"c %lld decisi ... _char: typeof(_136) = *const {l183} i8
        // 898: b"c %lld decisi ... _char:   l183 = UNIQUE | NON_NULL, (empty)
        // 898: b"c %lld decisi ... st u8: typeof(_137) = *const {l185} u8
        // 898: b"c %lld decisi ... st u8:   l185 = UNIQUE | NON_NULL, (empty)
        // 898: b"c %lld decisi ... \n\0": typeof(_138) = *const {l187} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000036)) }]
        // 898: b"c %lld decisi ... \n\0":   l187 = UNIQUE | NON_NULL, (empty)
        // 898: b"c %lld decisi ... \n\0": typeof(_139) = & {l189} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000036)) }]
        // 898: b"c %lld decisi ... \n\0":   l189 = UNIQUE | NON_NULL, FIXED
        // 898: b"c %lld decisi ... \n\0": typeof(_139 = const b"c %lld decisions, %lld conflicts, %.1f conflicts/sec\n\x00") = & {l279} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000036)) }]
        // 898: b"c %lld decisi ... \n\0":   l279 = UNIQUE | NON_NULL, (empty)
        // 898: b"c %lld decisi ... \n\0": typeof(_138 = &raw const (*_139)) = *const {l280} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000036)) }]
        // 898: b"c %lld decisi ... \n\0":   l280 = UNIQUE | NON_NULL, (empty)
        // 898: b"c %lld decisi ... st u8: typeof(_137 = move _138 as *const u8 (Pointer(ArrayToPointer))) = *const {l281} u8
        // 898: b"c %lld decisi ... st u8:   l281 = UNIQUE | NON_NULL, (empty)
        // 898: b"c %lld decisi ... _char: typeof(_136 = move _137 as *const i8 (Misc)) = *const {l282} i8
        // 898: b"c %lld decisi ... _char:   l282 = UNIQUE | NON_NULL, (empty)
            as *const libc::c_char,
        decs as libc::c_longlong,
        confs as libc::c_longlong,
        cps,
    );
    printf(
        b"c %lld0 propagations, %.1f megaprops/sec\n\0" as *const u8 as *const libc::c_char,
        // 905: b"c %lld0 propa ... _char: typeof(_144) = *const {l195} i8
        // 905: b"c %lld0 propa ... _char:   l195 = UNIQUE | NON_NULL, (empty)
        // 905: b"c %lld0 propa ... st u8: typeof(_145) = *const {l197} u8
        // 905: b"c %lld0 propa ... st u8:   l197 = UNIQUE | NON_NULL, (empty)
        // 905: b"c %lld0 propa ... \n\0": typeof(_146) = *const {l199} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
        // 905: b"c %lld0 propa ... \n\0":   l199 = UNIQUE | NON_NULL, (empty)
        // 905: b"c %lld0 propa ... \n\0": typeof(_147) = & {l201} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
        // 905: b"c %lld0 propa ... \n\0":   l201 = UNIQUE | NON_NULL, FIXED
        // 905: b"c %lld0 propa ... \n\0": typeof(_146 = &raw const (*_147)) = *const {l284} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
        // 905: b"c %lld0 propa ... \n\0":   l284 = UNIQUE | NON_NULL, (empty)
        // 905: b"c %lld0 propa ... \n\0": typeof(_147 = const b"c %lld0 propagations, %.1f megaprops/sec\n\x00") = & {l283} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
        // 905: b"c %lld0 propa ... \n\0":   l283 = UNIQUE | NON_NULL, (empty)
        // 905: b"c %lld0 propa ... _char: typeof(_144 = move _145 as *const i8 (Misc)) = *const {l286} i8
        // 905: b"c %lld0 propa ... _char:   l286 = UNIQUE | NON_NULL, (empty)
        // 905: b"c %lld0 propa ... st u8: typeof(_145 = move _146 as *const u8 (Pointer(ArrayToPointer))) = *const {l285} u8
        // 905: b"c %lld0 propa ... st u8:   l285 = UNIQUE | NON_NULL, (empty)
        props as libc::c_longlong,
        mpps,
    );
    printf(
        b"c %.1f process time, %.0f%% utilization\n\0" as *const u8 as *const libc::c_char,
        // 910: b"c %.1f proces ... _char: typeof(_151) = *const {l206} i8
        // 910: b"c %.1f proces ... _char:   l206 = UNIQUE | NON_NULL, (empty)
        // 910: b"c %.1f proces ... st u8: typeof(_152) = *const {l208} u8
        // 910: b"c %.1f proces ... st u8:   l208 = UNIQUE | NON_NULL, (empty)
        // 910: b"c %.1f proces ... \n\0": typeof(_153) = *const {l210} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 910: b"c %.1f proces ... \n\0":   l210 = UNIQUE | NON_NULL, (empty)
        // 910: b"c %.1f proces ... \n\0": typeof(_154) = & {l212} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 910: b"c %.1f proces ... \n\0":   l212 = UNIQUE | NON_NULL, FIXED
        // 910: b"c %.1f proces ... _char: typeof(_151 = move _152 as *const i8 (Misc)) = *const {l290} i8
        // 910: b"c %.1f proces ... _char:   l290 = UNIQUE | NON_NULL, (empty)
        // 910: b"c %.1f proces ... st u8: typeof(_152 = move _153 as *const u8 (Pointer(ArrayToPointer))) = *const {l289} u8
        // 910: b"c %.1f proces ... st u8:   l289 = UNIQUE | NON_NULL, (empty)
        // 910: b"c %.1f proces ... \n\0": typeof(_153 = &raw const (*_154)) = *const {l288} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 910: b"c %.1f proces ... \n\0":   l288 = UNIQUE | NON_NULL, (empty)
        // 910: b"c %.1f proces ... \n\0": typeof(_154 = const b"c %.1f process time, %.0f%% utilization\n\x00") = & {l287} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 910: b"c %.1f proces ... \n\0":   l287 = UNIQUE | NON_NULL, (empty)
        process,
        if real > 0 as libc::c_int as libc::c_double {
            100.0f64 * process / real / nworkers as libc::c_double
            // 913: nworkers: typeof(_167) = *mut {l226} i32
            // 913: nworkers:   l226 = UNIQUE | NON_NULL, (empty)
        } else {
            0.0f64
        },
    );
    printf(b"c\n\0" as *const u8 as *const libc::c_char);
    // 918: b"c\n\0" as *co ... _char: typeof(_169) = *const {l229} i8
    // 918: b"c\n\0" as *co ... _char:   l229 = UNIQUE | NON_NULL, (empty)
    // 918: b"c\n\0" as *co ... st u8: typeof(_170) = *const {l231} u8
    // 918: b"c\n\0" as *co ... st u8:   l231 = UNIQUE | NON_NULL, (empty)
    // 918: b"c\n\0": typeof(_171) = *const {l233} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 918: b"c\n\0":   l233 = UNIQUE | NON_NULL, (empty)
    // 918: b"c\n\0": typeof(_172) = & {l235} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 918: b"c\n\0":   l235 = UNIQUE | NON_NULL, FIXED
    // 918: b"c\n\0": typeof(_172 = const b"c\n\x00") = & {l291} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 918: b"c\n\0":   l291 = UNIQUE | NON_NULL, (empty)
    // 918: b"c\n\0": typeof(_171 = &raw const (*_172)) = *const {l292} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 918: b"c\n\0":   l292 = UNIQUE | NON_NULL, (empty)
    // 918: b"c\n\0" as *co ... st u8: typeof(_170 = move _171 as *const u8 (Pointer(ArrayToPointer))) = *const {l293} u8
    // 918: b"c\n\0" as *co ... st u8:   l293 = UNIQUE | NON_NULL, (empty)
    // 918: b"c\n\0" as *co ... _char: typeof(_169 = move _170 as *const i8 (Misc)) = *const {l294} i8
    // 918: b"c\n\0" as *co ... _char:   l294 = UNIQUE | NON_NULL, (empty)
    printf(
        b"c %.1f seconds, %.1f MB\n\0" as *const u8 as *const libc::c_char,
        // 920: b"c %.1f second ... _char: typeof(_174) = *const {l238} i8
        // 920: b"c %.1f second ... _char:   l238 = UNIQUE | NON_NULL, (empty)
        // 920: b"c %.1f second ... st u8: typeof(_175) = *const {l240} u8
        // 920: b"c %.1f second ... st u8:   l240 = UNIQUE | NON_NULL, (empty)
        // 920: b"c %.1f second ... \n\0": typeof(_176) = *const {l242} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
        // 920: b"c %.1f second ... \n\0":   l242 = UNIQUE | NON_NULL, (empty)
        // 920: b"c %.1f second ... \n\0": typeof(_177) = & {l244} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
        // 920: b"c %.1f second ... \n\0":   l244 = UNIQUE | NON_NULL, FIXED
        // 920: b"c %.1f second ... \n\0": typeof(_176 = &raw const (*_177)) = *const {l296} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
        // 920: b"c %.1f second ... \n\0":   l296 = UNIQUE | NON_NULL, (empty)
        // 920: b"c %.1f second ... \n\0": typeof(_177 = const b"c %.1f seconds, %.1f MB\n\x00") = & {l295} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
        // 920: b"c %.1f second ... \n\0":   l295 = UNIQUE | NON_NULL, (empty)
        // 920: b"c %.1f second ... _char: typeof(_174 = move _175 as *const i8 (Misc)) = *const {l298} i8
        // 920: b"c %.1f second ... _char:   l298 = UNIQUE | NON_NULL, (empty)
        // 920: b"c %.1f second ... st u8: typeof(_175 = move _176 as *const u8 (Pointer(ArrayToPointer))) = *const {l297} u8
        // 920: b"c %.1f second ... st u8:   l297 = UNIQUE | NON_NULL, (empty)
        real,
        mb,
    );
    fflush(stdout);
    // 924: stdout: typeof(_181) = *mut {l249} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 924: stdout:   l249 = UNIQUE | NON_NULL, (empty)
    // 924: stdout: typeof(_182) = *mut {l251} *mut {l252} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 924: stdout:   l251 = UNIQUE | NON_NULL, (empty)
    // 924: stdout:   l252 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn incmem(mut bytes: size_t) {
    if pthread_mutex_lock(&mut memutex) != 0 {
    // 927: &mut memutex: typeof(_5) = *mut {l5} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 927: &mut memutex:   l5 = UNIQUE | NON_NULL, (empty)
    // 927: &mut memutex: typeof(_6) = &mut {l7} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 927: &mut memutex:   l7 = UNIQUE | NON_NULL, (empty)
    // 927: memutex: typeof(_7) = *mut {l9} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 927: memutex:   l9 = UNIQUE | NON_NULL, (empty)
    // 927: &mut memutex: typeof(_5 = &raw mut (*_6)) = *mut {l58} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 927: &mut memutex:   l58 = UNIQUE | NON_NULL, (empty)
    // 927: &mut memutex: typeof(_6 = &mut (*_7)) = &mut {l57} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 927: &mut memutex:   l57 = UNIQUE | NON_NULL, (empty)
        warn(b"failed to lock 'mem' mutex in 'incmem'\0" as *const u8 as *const libc::c_char);
        // 928: b"failed to loc ... _char: typeof(_9) = *const {l12} i8
        // 928: b"failed to loc ... _char:   l12 = UNIQUE | NON_NULL, (empty)
        // 928: b"failed to loc ... st u8: typeof(_10) = *const {l14} u8
        // 928: b"failed to loc ... st u8:   l14 = UNIQUE | NON_NULL, (empty)
        // 928: b"failed to loc ... m'\0": typeof(_11) = *const {l16} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
        // 928: b"failed to loc ... m'\0":   l16 = UNIQUE | NON_NULL, (empty)
        // 928: b"failed to loc ... m'\0": typeof(_12) = & {l18} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
        // 928: b"failed to loc ... m'\0":   l18 = UNIQUE | NON_NULL, FIXED
        // 928: b"failed to loc ... m'\0": typeof(_12 = const b"failed to lock \'mem\' mutex in \'incmem\'\x00") = & {l59} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
        // 928: b"failed to loc ... m'\0":   l59 = UNIQUE | NON_NULL, (empty)
        // 928: b"failed to loc ... m'\0": typeof(_11 = &raw const (*_12)) = *const {l60} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
        // 928: b"failed to loc ... m'\0":   l60 = UNIQUE | NON_NULL, (empty)
        // 928: b"failed to loc ... _char: typeof(_9 = move _10 as *const i8 (Misc)) = *const {l62} i8
        // 928: b"failed to loc ... _char:   l62 = UNIQUE | NON_NULL, (empty)
        // 928: b"failed to loc ... st u8: typeof(_10 = move _11 as *const u8 (Pointer(ArrayToPointer))) = *const {l61} u8
        // 928: b"failed to loc ... st u8:   l61 = UNIQUE | NON_NULL, (empty)
    }
    mem.current = (mem.current).wrapping_add(bytes);
    // 930: mem: typeof(_15) = *mut {l22} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
    // 930: mem:   l22 = UNIQUE | NON_NULL, (empty)
    // 930: mem: typeof(_17) = *mut {l25} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
    // 930: mem:   l25 = UNIQUE | NON_NULL, (empty)
    if mem.current > mem.max {
    // 931: mem: typeof(_21) = *mut {l30} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
    // 931: mem:   l30 = UNIQUE | NON_NULL, (empty)
    // 931: mem: typeof(_23) = *mut {l33} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
    // 931: mem:   l33 = UNIQUE | NON_NULL, (empty)
        mem.max = mem.current;
        // 932: mem: typeof(_25) = *mut {l36} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
        // 932: mem:   l36 = UNIQUE | NON_NULL, (empty)
        // 932: mem: typeof(_26) = *mut {l38} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
        // 932: mem:   l38 = UNIQUE | NON_NULL, (empty)
    }
    if pthread_mutex_unlock(&mut memutex) != 0 {
    // 934: &mut memutex: typeof(_29) = *mut {l42} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 934: &mut memutex:   l42 = UNIQUE | NON_NULL, (empty)
    // 934: &mut memutex: typeof(_30) = &mut {l44} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 934: &mut memutex:   l44 = UNIQUE | NON_NULL, (empty)
    // 934: memutex: typeof(_31) = *mut {l46} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 934: memutex:   l46 = UNIQUE | NON_NULL, (empty)
    // 934: &mut memutex: typeof(_30 = &mut (*_31)) = &mut {l63} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 934: &mut memutex:   l63 = UNIQUE | NON_NULL, (empty)
    // 934: &mut memutex: typeof(_29 = &raw mut (*_30)) = *mut {l64} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 934: &mut memutex:   l64 = UNIQUE | NON_NULL, (empty)
        warn(b"failed to unlock 'mem' mutex in 'incmem'\0" as *const u8 as *const libc::c_char);
        // 935: b"failed to unl ... _char: typeof(_33) = *const {l49} i8
        // 935: b"failed to unl ... _char:   l49 = UNIQUE | NON_NULL, (empty)
        // 935: b"failed to unl ... st u8: typeof(_34) = *const {l51} u8
        // 935: b"failed to unl ... st u8:   l51 = UNIQUE | NON_NULL, (empty)
        // 935: b"failed to unl ... m'\0": typeof(_35) = *const {l53} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 935: b"failed to unl ... m'\0":   l53 = UNIQUE | NON_NULL, (empty)
        // 935: b"failed to unl ... m'\0": typeof(_36) = & {l55} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 935: b"failed to unl ... m'\0":   l55 = UNIQUE | NON_NULL, FIXED
        // 935: b"failed to unl ... m'\0": typeof(_36 = const b"failed to unlock \'mem\' mutex in \'incmem\'\x00") = & {l65} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 935: b"failed to unl ... m'\0":   l65 = UNIQUE | NON_NULL, (empty)
        // 935: b"failed to unl ... m'\0": typeof(_35 = &raw const (*_36)) = *const {l66} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 935: b"failed to unl ... m'\0":   l66 = UNIQUE | NON_NULL, (empty)
        // 935: b"failed to unl ... _char: typeof(_33 = move _34 as *const i8 (Misc)) = *const {l68} i8
        // 935: b"failed to unl ... _char:   l68 = UNIQUE | NON_NULL, (empty)
        // 935: b"failed to unl ... st u8: typeof(_34 = move _35 as *const u8 (Pointer(ArrayToPointer))) = *const {l67} u8
        // 935: b"failed to unl ... st u8:   l67 = UNIQUE | NON_NULL, (empty)
    }
}
unsafe extern "C" fn decmem(mut bytes: size_t) {
    if pthread_mutex_lock(&mut memutex) != 0 {
    // 939: &mut memutex: typeof(_5) = *mut {l5} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 939: &mut memutex:   l5 = UNIQUE | NON_NULL, (empty)
    // 939: &mut memutex: typeof(_6) = &mut {l7} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 939: &mut memutex:   l7 = UNIQUE | NON_NULL, (empty)
    // 939: memutex: typeof(_7) = *mut {l9} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 939: memutex:   l9 = UNIQUE | NON_NULL, (empty)
    // 939: &mut memutex: typeof(_5 = &raw mut (*_6)) = *mut {l45} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 939: &mut memutex:   l45 = UNIQUE | NON_NULL, (empty)
    // 939: &mut memutex: typeof(_6 = &mut (*_7)) = &mut {l44} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 939: &mut memutex:   l44 = UNIQUE | NON_NULL, (empty)
        warn(b"failed to lock 'mem' mutex in 'decmem'\0" as *const u8 as *const libc::c_char);
        // 940: b"failed to loc ... _char: typeof(_9) = *const {l12} i8
        // 940: b"failed to loc ... _char:   l12 = UNIQUE | NON_NULL, (empty)
        // 940: b"failed to loc ... st u8: typeof(_10) = *const {l14} u8
        // 940: b"failed to loc ... st u8:   l14 = UNIQUE | NON_NULL, (empty)
        // 940: b"failed to loc ... m'\0": typeof(_11) = *const {l16} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
        // 940: b"failed to loc ... m'\0":   l16 = UNIQUE | NON_NULL, (empty)
        // 940: b"failed to loc ... m'\0": typeof(_12) = & {l18} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
        // 940: b"failed to loc ... m'\0":   l18 = UNIQUE | NON_NULL, FIXED
        // 940: b"failed to loc ... st u8: typeof(_10 = move _11 as *const u8 (Pointer(ArrayToPointer))) = *const {l48} u8
        // 940: b"failed to loc ... st u8:   l48 = UNIQUE | NON_NULL, (empty)
        // 940: b"failed to loc ... m'\0": typeof(_11 = &raw const (*_12)) = *const {l47} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
        // 940: b"failed to loc ... m'\0":   l47 = UNIQUE | NON_NULL, (empty)
        // 940: b"failed to loc ... _char: typeof(_9 = move _10 as *const i8 (Misc)) = *const {l49} i8
        // 940: b"failed to loc ... _char:   l49 = UNIQUE | NON_NULL, (empty)
        // 940: b"failed to loc ... m'\0": typeof(_12 = const b"failed to lock \'mem\' mutex in \'decmem\'\x00") = & {l46} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
        // 940: b"failed to loc ... m'\0":   l46 = UNIQUE | NON_NULL, (empty)
    }
    mem.current = (mem.current).wrapping_sub(bytes);
    // 942: mem: typeof(_15) = *mut {l22} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
    // 942: mem:   l22 = UNIQUE | NON_NULL, (empty)
    // 942: mem: typeof(_17) = *mut {l25} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
    // 942: mem:   l25 = UNIQUE | NON_NULL, (empty)
    if pthread_mutex_unlock(&mut memutex) != 0 {
    // 943: &mut memutex: typeof(_20) = *mut {l29} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 943: &mut memutex:   l29 = UNIQUE | NON_NULL, (empty)
    // 943: &mut memutex: typeof(_21) = &mut {l31} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 943: &mut memutex:   l31 = UNIQUE | NON_NULL, (empty)
    // 943: memutex: typeof(_22) = *mut {l33} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 943: memutex:   l33 = UNIQUE | NON_NULL, (empty)
    // 943: &mut memutex: typeof(_20 = &raw mut (*_21)) = *mut {l51} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 943: &mut memutex:   l51 = UNIQUE | NON_NULL, (empty)
    // 943: &mut memutex: typeof(_21 = &mut (*_22)) = &mut {l50} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 943: &mut memutex:   l50 = UNIQUE | NON_NULL, (empty)
        warn(b"failed to unlock 'mem' mutex in 'decmem'\0" as *const u8 as *const libc::c_char);
        // 944: b"failed to unl ... _char: typeof(_24) = *const {l36} i8
        // 944: b"failed to unl ... _char:   l36 = UNIQUE | NON_NULL, (empty)
        // 944: b"failed to unl ... st u8: typeof(_25) = *const {l38} u8
        // 944: b"failed to unl ... st u8:   l38 = UNIQUE | NON_NULL, (empty)
        // 944: b"failed to unl ... m'\0": typeof(_26) = *const {l40} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 944: b"failed to unl ... m'\0":   l40 = UNIQUE | NON_NULL, (empty)
        // 944: b"failed to unl ... m'\0": typeof(_27) = & {l42} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 944: b"failed to unl ... m'\0":   l42 = UNIQUE | NON_NULL, FIXED
        // 944: b"failed to unl ... m'\0": typeof(_27 = const b"failed to unlock \'mem\' mutex in \'decmem\'\x00") = & {l52} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 944: b"failed to unl ... m'\0":   l52 = UNIQUE | NON_NULL, (empty)
        // 944: b"failed to unl ... m'\0": typeof(_26 = &raw const (*_27)) = *const {l53} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 944: b"failed to unl ... m'\0":   l53 = UNIQUE | NON_NULL, (empty)
        // 944: b"failed to unl ... st u8: typeof(_25 = move _26 as *const u8 (Pointer(ArrayToPointer))) = *const {l54} u8
        // 944: b"failed to unl ... st u8:   l54 = UNIQUE | NON_NULL, (empty)
        // 944: b"failed to unl ... _char: typeof(_24 = move _25 as *const i8 (Misc)) = *const {l55} i8
        // 944: b"failed to unl ... _char:   l55 = UNIQUE | NON_NULL, (empty)
    }
}
unsafe extern "C" fn alloc<'h0,'h1>(mut dummy: &'h0 (libc::c_void), mut bytes: size_t) -> core::option::Option<&'h1 (libc::c_void)> {
// 947: *mut libc::c_void: typeof(_0) = *mut {g6} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 947: *mut libc::c_void:   g6 = UNIQUE, (empty)
// 947: mut dummy: typeof(_1) = *mut {g5} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 947: mut dummy:   g5 = UNIQUE | NON_NULL, (empty)
    let mut res: core::option::Option<&mut [(libc::c_char)]> = None;
    // 948: mut res: typeof(_4) = *mut {l4} i8
    // 948: mut res:   l4 = WRITE | UNIQUE | OFFSET_ADD, (empty)
    // 948: 0 as *mut libc: ... _char: typeof(_4 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l41} i8
    // 948: 0 as *mut libc: ... _char:   l41 = WRITE | UNIQUE | OFFSET_ADD, (empty)
    let mut BYTES: size_t =
        bytes.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong);
    res = std::option::Option::Some((malloc(BYTES) as *mut libc::c_char));
    // 951: malloc(BYTES): typeof(_9) = *mut {l10} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 951: malloc(BYTES):   l10 = WRITE | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
    // 951: res = malloc(BY ... _char: typeof(_4 = move _9 as *mut i8 (Misc)) = *mut {l42} i8
    // 951: res = malloc(BY ... _char:   l42 = WRITE | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
    if (res).as_deref().map(|__ptr| &__ptr[0]).is_none() {
    // 952: res: typeof(_13) = *mut {l15} i8
    // 952: res:   l15 = UNIQUE, (empty)
        die(core::ptr::addr_of!(*(&*(b"out of memory\0") as *const u8 as *const libc::c_char)));
        // 953: b"out of memory ... _char: typeof(_16) = *const {l19} i8
        // 953: b"out of memory ... _char:   l19 = UNIQUE | NON_NULL, (empty)
        // 953: b"out of memory ... st u8: typeof(_17) = *const {l21} u8
        // 953: b"out of memory ... st u8:   l21 = UNIQUE | NON_NULL, (empty)
        // 953: b"out of memory\0": typeof(_18) = *const {l23} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 953: b"out of memory\0":   l23 = UNIQUE | NON_NULL, (empty)
        // 953: b"out of memory\0": typeof(_19) = & {l25} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 953: b"out of memory\0":   l25 = UNIQUE | NON_NULL, FIXED
        // 953: b"out of memory ... _char: typeof(_16 = move _17 as *const i8 (Misc)) = *const {l46} i8
        // 953: b"out of memory ... _char:   l46 = UNIQUE | NON_NULL, (empty)
        // 953: b"out of memory ... st u8: typeof(_17 = move _18 as *const u8 (Pointer(ArrayToPointer))) = *const {l45} u8
        // 953: b"out of memory ... st u8:   l45 = UNIQUE | NON_NULL, (empty)
        // 953: b"out of memory\0": typeof(_19 = const b"out of memory\x00") = & {l43} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 953: b"out of memory\0":   l43 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
        // 953: b"out of memory\0": typeof(_18 = &raw const (*_19)) = *const {l44} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 953: b"out of memory\0":   l44 = UNIQUE | NON_NULL, (empty)
        exit(1 as libc::c_int);
    }
    memset(res as *mut libc::c_void, 0 as libc::c_int, BYTES);
    // 956: memset(res as * ... YTES): typeof(_22) = *mut {l29} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 956: memset(res as * ... YTES):   l29 = UNIQUE | NON_NULL, (empty)
    // 956: res as *mut lib ... _void: typeof(_23) = *mut {l31} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 956: res as *mut lib ... _void:   l31 = WRITE | UNIQUE | OFFSET_ADD, (empty)
    // 956: res: typeof(_24) = *mut {l33} i8
    // 956: res:   l33 = WRITE | UNIQUE | OFFSET_ADD, (empty)
    // 956: res as *mut lib ... _void: typeof(_23 = move _24 as *mut libc::c_void (Misc)) = *mut {l47} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 956: res as *mut lib ... _void:   l47 = WRITE | UNIQUE | OFFSET_ADD, (empty)
    incmem(BYTES);
    return (res).as_deref().map(|__ptr| &__ptr[0]) as *mut libc::c_void;
    // 958: res: typeof(_29) = *mut {l39} i8
    // 958: res:   l39 = UNIQUE, (empty)
    // 958: res as *mut lib ... _void: typeof(_0 = move _29 as *mut libc::c_void (Misc)) = *mut {l48} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 958: res as *mut lib ... _void:   l48 = UNIQUE, (empty)
}
unsafe fn alloc_shim(arg0: *mut libc::c_void, arg1: size_t) -> *mut libc::c_void {
    {
    let safe_arg0 = &*arg0.cast_const();
    let safe_arg1 = arg1;
    let safe_result = alloc(safe_arg0,safe_arg1);
    let result = core::ptr::addr_of!(*safe_result.unwrap()).cast_mut();
    result
}
}

unsafe extern "C" fn dealloc<'h0,'h1>(
    mut dummy: &'h0 (libc::c_void),
    // 961: mut dummy: typeof(_1) = *mut {g7} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 961: mut dummy:   g7 = UNIQUE | NON_NULL, (empty)
    mut void_ptr: &'h1 (libc::c_void),
    // 962: mut void_ptr: typeof(_2) = *mut {g8} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 962: mut void_ptr:   g8 = UNIQUE | FREE | NON_NULL, (empty)
    mut bytes: size_t,
) {
    let mut char_ptr: &(libc::c_char) = void_ptr as *mut libc::c_char;
    // 965: mut char_ptr: typeof(_4) = *mut {l4} i8
    // 965: mut char_ptr:   l4 = UNIQUE | FREE | NON_NULL, (empty)
    // 965: void_ptr: typeof(_5) = *mut {l6} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 965: void_ptr:   l6 = UNIQUE | FREE | NON_NULL, (empty)
    // 965: void_ptr as *mu ... _char: typeof(_4 = move _5 as *mut i8 (Misc)) = *mut {l19} i8
    // 965: void_ptr as *mu ... _char:   l19 = UNIQUE | FREE | NON_NULL, (empty)
    let mut BYTES: size_t =
        bytes.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong);
    decmem(BYTES);
    free(char_ptr as *mut libc::c_void);
    // 969: char_ptr as *mu ... _void: typeof(_13) = *mut {l15} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 969: char_ptr as *mu ... _void:   l15 = UNIQUE | FREE | NON_NULL, (empty)
    // 969: char_ptr: typeof(_14) = *mut {l17} i8
    // 969: char_ptr:   l17 = UNIQUE | FREE | NON_NULL, (empty)
    // 969: char_ptr as *mu ... _void: typeof(_13 = move _14 as *mut libc::c_void (Misc)) = *mut {l20} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 969: char_ptr as *mu ... _void:   l20 = UNIQUE | FREE | NON_NULL, (empty)
}
unsafe fn dealloc_shim(arg0: *mut libc::c_void, arg1: *mut libc::c_void, arg2: size_t) {
    {
    let safe_arg0 = &*arg0.cast_const();
    let safe_arg1 = &*arg1.cast_const();
    let safe_arg2 = arg2;
    let safe_result = dealloc(safe_arg0,safe_arg1,safe_arg2);
    let result = safe_result;
    result
}
}

unsafe extern "C" fn resize(
    mut dummy: *mut libc::c_void,
    // 972: mut dummy: typeof(_1) = *mut {g9} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 972: mut dummy:   g9 = UNIQUE | NON_NULL, FIXED
    mut ptr: *mut libc::c_void,
    // 973: mut ptr: typeof(_2) = *mut {g10} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 973: mut ptr:   g10 = UNIQUE | NON_NULL, FIXED
    mut old_bytes: size_t,
    mut new_bytes: size_t,
) -> *mut libc::c_void {
// 976: *mut libc::c_void: typeof(_0) = *mut {g11} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 976: *mut libc::c_void:   g11 = UNIQUE | NON_NULL, FIXED
    let mut res: *mut libc::c_void = 0 as *mut libc::c_void;
    // 977: mut res: typeof(_6) = *mut {l6} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 977: mut res:   l6 = UNIQUE | NON_NULL, (empty)
    // 977: 0 as *mut libc: ... _void: typeof(_6 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l76} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 977: 0 as *mut libc: ... _void:   l76 = UNIQUE | NON_NULL, (empty)
    if pthread_mutex_lock(&mut memutex) != 0 {
    // 978: &mut memutex: typeof(_10) = *mut {l11} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 978: &mut memutex:   l11 = UNIQUE | NON_NULL, (empty)
    // 978: &mut memutex: typeof(_11) = &mut {l13} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 978: &mut memutex:   l13 = UNIQUE | NON_NULL, (empty)
    // 978: memutex: typeof(_12) = *mut {l15} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 978: memutex:   l15 = UNIQUE | NON_NULL, (empty)
    // 978: &mut memutex: typeof(_11 = &mut (*_12)) = &mut {l77} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 978: &mut memutex:   l77 = UNIQUE | NON_NULL, (empty)
    // 978: &mut memutex: typeof(_10 = &raw mut (*_11)) = *mut {l78} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 978: &mut memutex:   l78 = UNIQUE | NON_NULL, (empty)
        warn(b"failed to lock 'mem' mutex in 'resize'\0" as *const u8 as *const libc::c_char);
        // 979: b"failed to loc ... _char: typeof(_14) = *const {l18} i8
        // 979: b"failed to loc ... _char:   l18 = UNIQUE | NON_NULL, (empty)
        // 979: b"failed to loc ... st u8: typeof(_15) = *const {l20} u8
        // 979: b"failed to loc ... st u8:   l20 = UNIQUE | NON_NULL, (empty)
        // 979: b"failed to loc ... e'\0": typeof(_16) = *const {l22} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
        // 979: b"failed to loc ... e'\0":   l22 = UNIQUE | NON_NULL, (empty)
        // 979: b"failed to loc ... e'\0": typeof(_17) = & {l24} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
        // 979: b"failed to loc ... e'\0":   l24 = UNIQUE | NON_NULL, FIXED
        // 979: b"failed to loc ... e'\0": typeof(_17 = const b"failed to lock \'mem\' mutex in \'resize\'\x00") = & {l79} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
        // 979: b"failed to loc ... e'\0":   l79 = UNIQUE | NON_NULL, (empty)
        // 979: b"failed to loc ... e'\0": typeof(_16 = &raw const (*_17)) = *const {l80} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
        // 979: b"failed to loc ... e'\0":   l80 = UNIQUE | NON_NULL, (empty)
        // 979: b"failed to loc ... st u8: typeof(_15 = move _16 as *const u8 (Pointer(ArrayToPointer))) = *const {l81} u8
        // 979: b"failed to loc ... st u8:   l81 = UNIQUE | NON_NULL, (empty)
        // 979: b"failed to loc ... _char: typeof(_14 = move _15 as *const i8 (Misc)) = *const {l82} i8
        // 979: b"failed to loc ... _char:   l82 = UNIQUE | NON_NULL, (empty)
    }
    mem.current = (mem.current).wrapping_sub(old_bytes);
    // 981: mem: typeof(_20) = *mut {l28} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
    // 981: mem:   l28 = UNIQUE | NON_NULL, (empty)
    // 981: mem: typeof(_22) = *mut {l31} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
    // 981: mem:   l31 = UNIQUE | NON_NULL, (empty)
    mem.current = (mem.current).wrapping_add(new_bytes);
    // 982: mem: typeof(_25) = *mut {l35} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
    // 982: mem:   l35 = UNIQUE | NON_NULL, (empty)
    // 982: mem: typeof(_27) = *mut {l38} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
    // 982: mem:   l38 = UNIQUE | NON_NULL, (empty)
    if mem.current > mem.max {
    // 983: mem: typeof(_31) = *mut {l43} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
    // 983: mem:   l43 = UNIQUE | NON_NULL, (empty)
    // 983: mem: typeof(_33) = *mut {l46} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
    // 983: mem:   l46 = UNIQUE | NON_NULL, (empty)
        mem.max = mem.current;
        // 984: mem: typeof(_35) = *mut {l49} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
        // 984: mem:   l49 = UNIQUE | NON_NULL, (empty)
        // 984: mem: typeof(_36) = *mut {l51} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
        // 984: mem:   l51 = UNIQUE | NON_NULL, (empty)
    }
    if pthread_mutex_unlock(&mut memutex) != 0 {
    // 986: &mut memutex: typeof(_40) = *mut {l56} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 986: &mut memutex:   l56 = UNIQUE | NON_NULL, (empty)
    // 986: &mut memutex: typeof(_41) = &mut {l58} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 986: &mut memutex:   l58 = UNIQUE | NON_NULL, (empty)
    // 986: memutex: typeof(_42) = *mut {l60} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 986: memutex:   l60 = UNIQUE | NON_NULL, (empty)
    // 986: &mut memutex: typeof(_41 = &mut (*_42)) = &mut {l83} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 986: &mut memutex:   l83 = UNIQUE | NON_NULL, (empty)
    // 986: &mut memutex: typeof(_40 = &raw mut (*_41)) = *mut {l84} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 986: &mut memutex:   l84 = UNIQUE | NON_NULL, (empty)
        warn(b"failed to unlock 'mem' mutex in 'resize'\0" as *const u8 as *const libc::c_char);
        // 987: b"failed to unl ... _char: typeof(_44) = *const {l63} i8
        // 987: b"failed to unl ... _char:   l63 = UNIQUE | NON_NULL, (empty)
        // 987: b"failed to unl ... st u8: typeof(_45) = *const {l65} u8
        // 987: b"failed to unl ... st u8:   l65 = UNIQUE | NON_NULL, (empty)
        // 987: b"failed to unl ... e'\0": typeof(_46) = *const {l67} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 987: b"failed to unl ... e'\0":   l67 = UNIQUE | NON_NULL, (empty)
        // 987: b"failed to unl ... e'\0": typeof(_47) = & {l69} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 987: b"failed to unl ... e'\0":   l69 = UNIQUE | NON_NULL, FIXED
        // 987: b"failed to unl ... e'\0": typeof(_47 = const b"failed to unlock \'mem\' mutex in \'resize\'\x00") = & {l85} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 987: b"failed to unl ... e'\0":   l85 = UNIQUE | NON_NULL, (empty)
        // 987: b"failed to unl ... e'\0": typeof(_46 = &raw const (*_47)) = *const {l86} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 987: b"failed to unl ... e'\0":   l86 = UNIQUE | NON_NULL, (empty)
        // 987: b"failed to unl ... st u8: typeof(_45 = move _46 as *const u8 (Pointer(ArrayToPointer))) = *const {l87} u8
        // 987: b"failed to unl ... st u8:   l87 = UNIQUE | NON_NULL, (empty)
        // 987: b"failed to unl ... _char: typeof(_44 = move _45 as *const i8 (Misc)) = *const {l88} i8
        // 987: b"failed to unl ... _char:   l88 = UNIQUE | NON_NULL, (empty)
    }
    res = realloc(ptr, new_bytes);
    // 989: realloc(ptr, ne ... ytes): typeof(_48) = *mut {l71} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 989: realloc(ptr, ne ... ytes):   l71 = UNIQUE | NON_NULL, (empty)
    // 989: ptr: typeof(_49) = *mut {l73} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 989: ptr:   l73 = UNIQUE | NON_NULL, (empty)
    return res;
}
static sig_int_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static sig_segv_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static sig_abrt_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static sig_term_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
unsafe extern "C" fn resetsighandlers() {
    signal(2 as libc::c_int, sig_int_handler);
    // 997: sig_int_handler: typeof(_4) = *mut {l4} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 997: sig_int_handler:   l4 = UNIQUE | NON_NULL, (empty)
    signal(11 as libc::c_int, sig_segv_handler);
    // 998: sig_segv_handler: typeof(_8) = *mut {l9} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 998: sig_segv_handler:   l9 = UNIQUE | NON_NULL, (empty)
    signal(6 as libc::c_int, sig_abrt_handler);
    // 999: sig_abrt_handler: typeof(_12) = *mut {l14} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 999: sig_abrt_handler:   l14 = UNIQUE | NON_NULL, (empty)
    signal(15 as libc::c_int, sig_term_handler);
    // 1000: sig_term_handler: typeof(_16) = *mut {l19} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 1000: sig_term_handler:   l19 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn caughtsigmsg(mut sig: libc::c_int) {
    if verbose == 0 {
    // 1003: verbose: typeof(_5) = *mut {l5} i32
    // 1003: verbose:   l5 = UNIQUE | NON_NULL, (empty)
        return;
    }
    printf(
        b"c\nc CAUGHT SIGNAL %d\nc\n\0" as *const u8 as *const libc::c_char,
        // 1007: b"c\nc CAUGHT S ... _char: typeof(_8) = *const {l9} i8
        // 1007: b"c\nc CAUGHT S ... _char:   l9 = UNIQUE | NON_NULL, (empty)
        // 1007: b"c\nc CAUGHT S ... st u8: typeof(_9) = *const {l11} u8
        // 1007: b"c\nc CAUGHT S ... st u8:   l11 = UNIQUE | NON_NULL, (empty)
        // 1007: b"c\nc CAUGHT S ... \n\0": typeof(_10) = *const {l13} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 1007: b"c\nc CAUGHT S ... \n\0":   l13 = UNIQUE | NON_NULL, (empty)
        // 1007: b"c\nc CAUGHT S ... \n\0": typeof(_11) = & {l15} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 1007: b"c\nc CAUGHT S ... \n\0":   l15 = UNIQUE | NON_NULL, FIXED
        // 1007: b"c\nc CAUGHT S ... \n\0": typeof(_10 = &raw const (*_11)) = *const {l25} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 1007: b"c\nc CAUGHT S ... \n\0":   l25 = UNIQUE | NON_NULL, (empty)
        // 1007: b"c\nc CAUGHT S ... st u8: typeof(_9 = move _10 as *const u8 (Pointer(ArrayToPointer))) = *const {l26} u8
        // 1007: b"c\nc CAUGHT S ... st u8:   l26 = UNIQUE | NON_NULL, (empty)
        // 1007: b"c\nc CAUGHT S ... \n\0": typeof(_11 = const b"c\nc CAUGHT SIGNAL %d\nc\n\x00") = & {l24} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 1007: b"c\nc CAUGHT S ... \n\0":   l24 = UNIQUE | NON_NULL, (empty)
        // 1007: b"c\nc CAUGHT S ... _char: typeof(_8 = move _9 as *const i8 (Misc)) = *const {l27} i8
        // 1007: b"c\nc CAUGHT S ... _char:   l27 = UNIQUE | NON_NULL, (empty)
        sig,
    );
    fflush(stdout);
    // 1010: stdout: typeof(_14) = *mut {l19} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 1010: stdout:   l19 = UNIQUE | NON_NULL, (empty)
    // 1010: stdout: typeof(_15) = *mut {l21} *mut {l22} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 1010: stdout:   l21 = UNIQUE | NON_NULL, (empty)
    // 1010: stdout:   l22 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn catchsig(mut sig: libc::c_int) {
    if catchedsig == 0 {
    // 1013: catchedsig: typeof(_5) = *mut {l5} i32
    // 1013: catchedsig:   l5 = UNIQUE | NON_NULL, (empty)
        fputs(
            b"c s UNKNOWN\n\0" as *const u8 as *const libc::c_char,
            // 1015: b"c s UNKNOWN\n ... _char: typeof(_7) = *const {l8} i8
            // 1015: b"c s UNKNOWN\n ... _char:   l8 = UNIQUE | NON_NULL, (empty)
            // 1015: b"c s UNKNOWN\n ... st u8: typeof(_8) = *const {l10} u8
            // 1015: b"c s UNKNOWN\n ... st u8:   l10 = UNIQUE | NON_NULL, (empty)
            // 1015: b"c s UNKNOWN\n\0": typeof(_9) = *const {l12} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 1015: b"c s UNKNOWN\n\0":   l12 = UNIQUE | NON_NULL, (empty)
            // 1015: b"c s UNKNOWN\n\0": typeof(_10) = & {l14} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 1015: b"c s UNKNOWN\n\0":   l14 = UNIQUE | NON_NULL, FIXED
            // 1015: b"c s UNKNOWN\n\0": typeof(_10 = const b"c s UNKNOWN\n\x00") = & {l94} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 1015: b"c s UNKNOWN\n\0":   l94 = UNIQUE | NON_NULL, (empty)
            // 1015: b"c s UNKNOWN\n ... _char: typeof(_7 = move _8 as *const i8 (Misc)) = *const {l97} i8
            // 1015: b"c s UNKNOWN\n ... _char:   l97 = UNIQUE | NON_NULL, (empty)
            // 1015: b"c s UNKNOWN\n\0": typeof(_9 = &raw const (*_10)) = *const {l95} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 1015: b"c s UNKNOWN\n\0":   l95 = UNIQUE | NON_NULL, (empty)
            // 1015: b"c s UNKNOWN\n ... st u8: typeof(_8 = move _9 as *const u8 (Pointer(ArrayToPointer))) = *const {l96} u8
            // 1015: b"c s UNKNOWN\n ... st u8:   l96 = UNIQUE | NON_NULL, (empty)
            stdout,
            // 1016: stdout: typeof(_11) = *mut {l16} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 1016: stdout:   l16 = UNIQUE | NON_NULL, (empty)
            // 1016: stdout: typeof(_12) = *mut {l18} *mut {l19} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 1016: stdout:   l18 = UNIQUE | NON_NULL, (empty)
            // 1016: stdout:   l19 = UNIQUE | NON_NULL, (empty)
        );
        fflush(stdout);
        // 1018: stdout: typeof(_14) = *mut {l22} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 1018: stdout:   l22 = UNIQUE | NON_NULL, (empty)
        // 1018: stdout: typeof(_15) = *mut {l24} *mut {l25} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 1018: stdout:   l24 = UNIQUE | NON_NULL, (empty)
        // 1018: stdout:   l25 = UNIQUE | NON_NULL, (empty)
        catchedsig = 1 as libc::c_int;
        // 1019: catchedsig: typeof(_17) = *mut {l28} i32
        // 1019: catchedsig:   l28 = UNIQUE | NON_NULL, (empty)
        caughtsigmsg(sig);
        stats();
        caughtsigmsg(sig);
    }
    resetsighandlers();
    if !(getenv(b"LGLSLEEPONABORT\0" as *const u8 as *const libc::c_char)).is_null() {
    // 1025: (getenv(b"LGLSL ... har)): typeof(_27) = *mut {l39} i8
    // 1025: (getenv(b"LGLSL ... har)):   l39 = UNIQUE | NON_NULL, (empty)
    // 1025: b"LGLSLEEPONABO ... _char: typeof(_28) = *const {l41} i8
    // 1025: b"LGLSLEEPONABO ... _char:   l41 = UNIQUE | NON_NULL, (empty)
    // 1025: b"LGLSLEEPONABO ... st u8: typeof(_29) = *const {l43} u8
    // 1025: b"LGLSLEEPONABO ... st u8:   l43 = UNIQUE | NON_NULL, (empty)
    // 1025: b"LGLSLEEPONABORT\0": typeof(_30) = *const {l45} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
    // 1025: b"LGLSLEEPONABORT\0":   l45 = UNIQUE | NON_NULL, (empty)
    // 1025: b"LGLSLEEPONABORT\0": typeof(_31) = & {l47} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
    // 1025: b"LGLSLEEPONABORT\0":   l47 = UNIQUE | NON_NULL, FIXED
    // 1025: b"LGLSLEEPONABORT\0": typeof(_31 = const b"LGLSLEEPONABORT\x00") = & {l98} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
    // 1025: b"LGLSLEEPONABORT\0":   l98 = UNIQUE | NON_NULL, (empty)
    // 1025: b"LGLSLEEPONABO ... st u8: typeof(_29 = move _30 as *const u8 (Pointer(ArrayToPointer))) = *const {l100} u8
    // 1025: b"LGLSLEEPONABO ... st u8:   l100 = UNIQUE | NON_NULL, (empty)
    // 1025: b"LGLSLEEPONABO ... _char: typeof(_28 = move _29 as *const i8 (Misc)) = *const {l101} i8
    // 1025: b"LGLSLEEPONABO ... _char:   l101 = UNIQUE | NON_NULL, (empty)
    // 1025: b"LGLSLEEPONABORT\0": typeof(_30 = &raw const (*_31)) = *const {l99} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
    // 1025: b"LGLSLEEPONABORT\0":   l99 = UNIQUE | NON_NULL, (empty)
        let sec: libc::c_int = 1000 as libc::c_int;
        fprintf(
            stderr,
            // 1028: stderr: typeof(_34) = *mut {l51} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 1028: stderr:   l51 = UNIQUE | NON_NULL, (empty)
            // 1028: stderr: typeof(_35) = *mut {l53} *mut {l54} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 1028: stderr:   l53 = UNIQUE | NON_NULL, (empty)
            // 1028: stderr:   l54 = UNIQUE | NON_NULL, (empty)
            b"plingeling: plingeling.c:%d: process %ld sleeping for %d seconds\n\0" as *const u8
            // 1029: b"plingeling: p ... _char: typeof(_36) = *const {l56} i8
            // 1029: b"plingeling: p ... _char:   l56 = UNIQUE | NON_NULL, (empty)
            // 1029: b"plingeling: p ... st u8: typeof(_37) = *const {l58} u8
            // 1029: b"plingeling: p ... st u8:   l58 = UNIQUE | NON_NULL, (empty)
            // 1029: b"plingeling: p ... \n\0": typeof(_38) = *const {l60} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000042)) }]
            // 1029: b"plingeling: p ... \n\0":   l60 = UNIQUE | NON_NULL, (empty)
            // 1029: b"plingeling: p ... \n\0": typeof(_39) = & {l62} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000042)) }]
            // 1029: b"plingeling: p ... \n\0":   l62 = UNIQUE | NON_NULL, FIXED
            // 1029: b"plingeling: p ... _char: typeof(_36 = move _37 as *const i8 (Misc)) = *const {l105} i8
            // 1029: b"plingeling: p ... _char:   l105 = UNIQUE | NON_NULL, (empty)
            // 1029: b"plingeling: p ... \n\0": typeof(_39 = const b"plingeling: plingeling.c:%d: process %ld sleeping for %d seconds\n\x00") = & {l102} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000042)) }]
            // 1029: b"plingeling: p ... \n\0":   l102 = UNIQUE | NON_NULL, (empty)
            // 1029: b"plingeling: p ... \n\0": typeof(_38 = &raw const (*_39)) = *const {l103} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000042)) }]
            // 1029: b"plingeling: p ... \n\0":   l103 = UNIQUE | NON_NULL, (empty)
            // 1029: b"plingeling: p ... st u8: typeof(_37 = move _38 as *const u8 (Pointer(ArrayToPointer))) = *const {l104} u8
            // 1029: b"plingeling: p ... st u8:   l104 = UNIQUE | NON_NULL, (empty)
                as *const libc::c_char,
            292 as libc::c_int,
            getpid() as libc::c_long,
            sec,
        );
        fflush(stderr);
        // 1035: stderr: typeof(_45) = *mut {l69} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 1035: stderr:   l69 = UNIQUE | NON_NULL, (empty)
        // 1035: stderr: typeof(_46) = *mut {l71} *mut {l72} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 1035: stderr:   l71 = UNIQUE | NON_NULL, (empty)
        // 1035: stderr:   l72 = UNIQUE | NON_NULL, (empty)
        sleep(sec as libc::c_uint);
    }
    if (getenv(b"LGLNABORT\0" as *const u8 as *const libc::c_char)).is_null() {
    // 1038: (getenv(b"LGLNA ... har)): typeof(_52) = *mut {l79} i8
    // 1038: (getenv(b"LGLNA ... har)):   l79 = UNIQUE | NON_NULL, (empty)
    // 1038: b"LGLNABORT\0"  ... _char: typeof(_53) = *const {l81} i8
    // 1038: b"LGLNABORT\0"  ... _char:   l81 = UNIQUE | NON_NULL, (empty)
    // 1038: b"LGLNABORT\0"  ... st u8: typeof(_54) = *const {l83} u8
    // 1038: b"LGLNABORT\0"  ... st u8:   l83 = UNIQUE | NON_NULL, (empty)
    // 1038: b"LGLNABORT\0": typeof(_55) = *const {l85} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
    // 1038: b"LGLNABORT\0":   l85 = UNIQUE | NON_NULL, (empty)
    // 1038: b"LGLNABORT\0": typeof(_56) = & {l87} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
    // 1038: b"LGLNABORT\0":   l87 = UNIQUE | NON_NULL, FIXED
    // 1038: b"LGLNABORT\0": typeof(_55 = &raw const (*_56)) = *const {l107} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
    // 1038: b"LGLNABORT\0":   l107 = UNIQUE | NON_NULL, (empty)
    // 1038: b"LGLNABORT\0"  ... _char: typeof(_53 = move _54 as *const i8 (Misc)) = *const {l109} i8
    // 1038: b"LGLNABORT\0"  ... _char:   l109 = UNIQUE | NON_NULL, (empty)
    // 1038: b"LGLNABORT\0": typeof(_56 = const b"LGLNABORT\x00") = & {l106} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
    // 1038: b"LGLNABORT\0":   l106 = UNIQUE | NON_NULL, (empty)
    // 1038: b"LGLNABORT\0"  ... st u8: typeof(_54 = move _55 as *const u8 (Pointer(ArrayToPointer))) = *const {l108} u8
    // 1038: b"LGLNABORT\0"  ... st u8:   l108 = UNIQUE | NON_NULL, (empty)
        raise(sig);
    } else {
        exit(1 as libc::c_int);
    };
}
unsafe extern "C" fn setsighandlers() {
    sig_int_handler = signal(
    // 1045: sig_int_handler: typeof(_5) = *mut {l5} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 1045: sig_int_handler:   l5 = UNIQUE | NON_NULL, (empty)
        2 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    sig_abrt_handler = signal(
    // 1049: sig_abrt_handler: typeof(_10) = *mut {l11} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 1049: sig_abrt_handler:   l11 = UNIQUE | NON_NULL, (empty)
        6 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    sig_term_handler = signal(
    // 1053: sig_term_handler: typeof(_15) = *mut {l17} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 1053: sig_term_handler:   l17 = UNIQUE | NON_NULL, (empty)
        15 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
unsafe extern "C" fn parse() -> *const libc::c_char {
// 1058: *const libc::c_char: typeof(_0) = *const {g12} i8
// 1058: *const libc::c_char:   g12 = UNIQUE | NON_NULL, FIXED
    let mut ch: libc::c_int = 0;
    let mut lit: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut minlen: libc::c_int = 0;
    let mut maxlen: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut occs: *mut libc::c_int = 0 as *mut libc::c_int;
    // 1065: mut occs: typeof(_7) = *mut {l7} i32
    // 1065: mut occs:   l7 = UNIQUE | NON_NULL, (empty)
    // 1065: 0 as *mut libc: ... c_int: typeof(_7 = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l817} i32
    // 1065: 0 as *mut libc: ... c_int:   l817 = UNIQUE | NON_NULL, (empty)
    loop {
        ch = getc(file);
        // 1067: file: typeof(_11) = *mut {l12} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 1067: file:   l12 = UNIQUE | NON_NULL, (empty)
        // 1067: file: typeof(_12) = *mut {l14} *mut {l15} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 1067: file:   l14 = UNIQUE | NON_NULL, (empty)
        // 1067: file:   l15 = UNIQUE | NON_NULL, (empty)
        if !(ch == 'c' as i32) {
            break;
        }
        loop {
            ch = getc(file);
            // 1072: file: typeof(_20) = *mut {l24} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 1072: file:   l24 = UNIQUE | NON_NULL, (empty)
            // 1072: file: typeof(_21) = *mut {l26} *mut {l27} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 1072: file:   l26 = UNIQUE | NON_NULL, (empty)
            // 1072: file:   l27 = UNIQUE | NON_NULL, (empty)
            if !(ch != '\n' as i32) {
                break;
            }
            if ch == -(1 as libc::c_int) {
                return b"EOF in comment\0" as *const u8 as *const libc::c_char;
                // 1077: b"EOF in commen ... st u8: typeof(_34) = *const {l41} u8
                // 1077: b"EOF in commen ... st u8:   l41 = UNIQUE | NON_NULL, (empty)
                // 1077: b"EOF in comment\0": typeof(_35) = *const {l43} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                // 1077: b"EOF in comment\0":   l43 = UNIQUE | NON_NULL, (empty)
                // 1077: b"EOF in comment\0": typeof(_36) = & {l45} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                // 1077: b"EOF in comment\0":   l45 = UNIQUE | NON_NULL, FIXED
                // 1077: b"EOF in commen ... st u8: typeof(_34 = move _35 as *const u8 (Pointer(ArrayToPointer))) = *const {l820} u8
                // 1077: b"EOF in commen ... st u8:   l820 = UNIQUE | NON_NULL, (empty)
                // 1077: b"EOF in comment\0": typeof(_35 = &raw const (*_36)) = *const {l819} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                // 1077: b"EOF in comment\0":   l819 = UNIQUE | NON_NULL, (empty)
                // 1077: b"EOF in comment\0": typeof(_36 = const b"EOF in comment\x00") = & {l818} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                // 1077: b"EOF in comment\0":   l818 = UNIQUE | NON_NULL, (empty)
                // 1077: b"EOF in commen ... _char: typeof(_0 = move _34 as *const i8 (Misc)) = *const {l821} i8
                // 1077: b"EOF in commen ... _char:   l821 = UNIQUE | NON_NULL, (empty)
            }
        }
    }
    if ch != 'p' as i32 {
        return b"expected header or comment\0" as *const u8 as *const libc::c_char;
        // 1082: b"expected head ... st u8: typeof(_42) = *const {l52} u8
        // 1082: b"expected head ... st u8:   l52 = UNIQUE | NON_NULL, (empty)
        // 1082: b"expected head ... nt\0": typeof(_43) = *const {l54} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 1082: b"expected head ... nt\0":   l54 = UNIQUE | NON_NULL, (empty)
        // 1082: b"expected head ... nt\0": typeof(_44) = & {l56} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 1082: b"expected head ... nt\0":   l56 = UNIQUE | NON_NULL, FIXED
        // 1082: b"expected head ... st u8: typeof(_42 = move _43 as *const u8 (Pointer(ArrayToPointer))) = *const {l824} u8
        // 1082: b"expected head ... st u8:   l824 = UNIQUE | NON_NULL, (empty)
        // 1082: b"expected head ... nt\0": typeof(_43 = &raw const (*_44)) = *const {l823} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 1082: b"expected head ... nt\0":   l823 = UNIQUE | NON_NULL, (empty)
        // 1082: b"expected head ... _char: typeof(_0 = move _42 as *const i8 (Misc)) = *const {l825} i8
        // 1082: b"expected head ... _char:   l825 = UNIQUE | NON_NULL, (empty)
        // 1082: b"expected head ... nt\0": typeof(_44 = const b"expected header or comment\x00") = & {l822} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 1082: b"expected head ... nt\0":   l822 = UNIQUE | NON_NULL, (empty)
    }
    ungetc(ch, file);
    // 1084: file: typeof(_47) = *mut {l60} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 1084: file:   l60 = UNIQUE | NON_NULL, (empty)
    // 1084: file: typeof(_48) = *mut {l62} *mut {l63} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 1084: file:   l62 = UNIQUE | NON_NULL, (empty)
    // 1084: file:   l63 = UNIQUE | NON_NULL, (empty)
    if fscanf(
        file,
        // 1086: file: typeof(_52) = *mut {l68} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 1086: file:   l68 = UNIQUE | NON_NULL, (empty)
        // 1086: file: typeof(_53) = *mut {l70} *mut {l71} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 1086: file:   l70 = UNIQUE | NON_NULL, (empty)
        // 1086: file:   l71 = UNIQUE | NON_NULL, (empty)
        b"p cnf %d %d\0" as *const u8 as *const libc::c_char,
        // 1087: b"p cnf %d %d\0 ... _char: typeof(_54) = *const {l73} i8
        // 1087: b"p cnf %d %d\0 ... _char:   l73 = UNIQUE | NON_NULL, (empty)
        // 1087: b"p cnf %d %d\0 ... st u8: typeof(_55) = *const {l75} u8
        // 1087: b"p cnf %d %d\0 ... st u8:   l75 = UNIQUE | NON_NULL, (empty)
        // 1087: b"p cnf %d %d\0": typeof(_56) = *const {l77} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 1087: b"p cnf %d %d\0":   l77 = UNIQUE | NON_NULL, (empty)
        // 1087: b"p cnf %d %d\0": typeof(_57) = & {l79} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 1087: b"p cnf %d %d\0":   l79 = UNIQUE | NON_NULL, FIXED
        // 1087: b"p cnf %d %d\0 ... st u8: typeof(_55 = move _56 as *const u8 (Pointer(ArrayToPointer))) = *const {l828} u8
        // 1087: b"p cnf %d %d\0 ... st u8:   l828 = UNIQUE | NON_NULL, (empty)
        // 1087: b"p cnf %d %d\0 ... _char: typeof(_54 = move _55 as *const i8 (Misc)) = *const {l829} i8
        // 1087: b"p cnf %d %d\0 ... _char:   l829 = UNIQUE | NON_NULL, (empty)
        // 1087: b"p cnf %d %d\0": typeof(_56 = &raw const (*_57)) = *const {l827} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 1087: b"p cnf %d %d\0":   l827 = UNIQUE | NON_NULL, (empty)
        // 1087: b"p cnf %d %d\0": typeof(_57 = const b"p cnf %d %d\x00") = & {l826} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 1087: b"p cnf %d %d\0":   l826 = UNIQUE | NON_NULL, (empty)
        &mut nvars as *mut libc::c_int,
        // 1088: &mut nvars as * ... c_int: typeof(_58) = *mut {l81} i32
        // 1088: &mut nvars as * ... c_int:   l81 = UNIQUE | NON_NULL, (empty)
        // 1088: &mut nvars: typeof(_59) = &mut {l83} i32
        // 1088: &mut nvars:   l83 = UNIQUE | NON_NULL, (empty)
        // 1088: nvars: typeof(_60) = *mut {l85} i32
        // 1088: nvars:   l85 = UNIQUE | NON_NULL, (empty)
        // 1088: &mut nvars: typeof(_59 = &mut (*_60)) = &mut {l830} i32
        // 1088: &mut nvars:   l830 = UNIQUE | NON_NULL, (empty)
        // 1088: &mut nvars: typeof(_58 = &raw mut (*_59)) = *mut {l831} i32
        // 1088: &mut nvars:   l831 = UNIQUE | NON_NULL, (empty)
        &mut nclauses as *mut libc::c_int,
        // 1089: &mut nclauses a ... c_int: typeof(_61) = *mut {l87} i32
        // 1089: &mut nclauses a ... c_int:   l87 = UNIQUE | NON_NULL, (empty)
        // 1089: &mut nclauses: typeof(_62) = &mut {l89} i32
        // 1089: &mut nclauses:   l89 = UNIQUE | NON_NULL, (empty)
        // 1089: nclauses: typeof(_63) = *mut {l91} i32
        // 1089: nclauses:   l91 = UNIQUE | NON_NULL, (empty)
        // 1089: &mut nclauses: typeof(_61 = &raw mut (*_62)) = *mut {l833} i32
        // 1089: &mut nclauses:   l833 = UNIQUE | NON_NULL, (empty)
        // 1089: &mut nclauses: typeof(_62 = &mut (*_63)) = &mut {l832} i32
        // 1089: &mut nclauses:   l832 = UNIQUE | NON_NULL, (empty)
    ) != 2 as libc::c_int
    {
        return b"can not parse header\0" as *const u8 as *const libc::c_char;
        // 1092: b"can not parse ... st u8: typeof(_66) = *const {l95} u8
        // 1092: b"can not parse ... st u8:   l95 = UNIQUE | NON_NULL, (empty)
        // 1092: b"can not parse ... er\0": typeof(_67) = *const {l97} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
        // 1092: b"can not parse ... er\0":   l97 = UNIQUE | NON_NULL, (empty)
        // 1092: b"can not parse ... er\0": typeof(_68) = & {l99} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
        // 1092: b"can not parse ... er\0":   l99 = UNIQUE | NON_NULL, FIXED
        // 1092: b"can not parse ... _char: typeof(_0 = move _66 as *const i8 (Misc)) = *const {l837} i8
        // 1092: b"can not parse ... _char:   l837 = UNIQUE | NON_NULL, (empty)
        // 1092: b"can not parse ... er\0": typeof(_67 = &raw const (*_68)) = *const {l835} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
        // 1092: b"can not parse ... er\0":   l835 = UNIQUE | NON_NULL, (empty)
        // 1092: b"can not parse ... st u8: typeof(_66 = move _67 as *const u8 (Pointer(ArrayToPointer))) = *const {l836} u8
        // 1092: b"can not parse ... st u8:   l836 = UNIQUE | NON_NULL, (empty)
        // 1092: b"can not parse ... er\0": typeof(_68 = const b"can not parse header\x00") = & {l834} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
        // 1092: b"can not parse ... er\0":   l834 = UNIQUE | NON_NULL, (empty)
    }
    msg(
        -(1 as libc::c_int),
        1 as libc::c_int,
        b"p cnf %d %d\0" as *const u8 as *const libc::c_char,
        // 1097: b"p cnf %d %d\0 ... _char: typeof(_74) = *const {l106} i8
        // 1097: b"p cnf %d %d\0 ... _char:   l106 = UNIQUE | NON_NULL, (empty)
        // 1097: b"p cnf %d %d\0 ... st u8: typeof(_75) = *const {l108} u8
        // 1097: b"p cnf %d %d\0 ... st u8:   l108 = UNIQUE | NON_NULL, (empty)
        // 1097: b"p cnf %d %d\0": typeof(_76) = *const {l110} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 1097: b"p cnf %d %d\0":   l110 = UNIQUE | NON_NULL, (empty)
        // 1097: b"p cnf %d %d\0": typeof(_77) = & {l112} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 1097: b"p cnf %d %d\0":   l112 = UNIQUE | NON_NULL, FIXED
        // 1097: b"p cnf %d %d\0 ... st u8: typeof(_75 = move _76 as *const u8 (Pointer(ArrayToPointer))) = *const {l840} u8
        // 1097: b"p cnf %d %d\0 ... st u8:   l840 = UNIQUE | NON_NULL, (empty)
        // 1097: b"p cnf %d %d\0 ... _char: typeof(_74 = move _75 as *const i8 (Misc)) = *const {l841} i8
        // 1097: b"p cnf %d %d\0 ... _char:   l841 = UNIQUE | NON_NULL, (empty)
        // 1097: b"p cnf %d %d\0": typeof(_76 = &raw const (*_77)) = *const {l839} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 1097: b"p cnf %d %d\0":   l839 = UNIQUE | NON_NULL, (empty)
        // 1097: b"p cnf %d %d\0": typeof(_77 = const b"p cnf %d %d\x00") = & {l838} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 1097: b"p cnf %d %d\0":   l838 = UNIQUE | NON_NULL, (empty)
        nvars,
        // 1098: nvars: typeof(_79) = *mut {l115} i32
        // 1098: nvars:   l115 = UNIQUE | NON_NULL, (empty)
        nclauses,
        // 1099: nclauses: typeof(_81) = *mut {l118} i32
        // 1099: nclauses:   l118 = UNIQUE | NON_NULL, (empty)
    );
    let mut BYTES: size_t = ((nvars + 1 as libc::c_int) as libc::c_ulong)
    // 1101: nvars: typeof(_86) = *mut {l124} i32
    // 1101: nvars:   l124 = UNIQUE | NON_NULL, (empty)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    fixed = malloc(BYTES) as *mut libc::c_int;
    // 1103: malloc(BYTES): typeof(_91) = *mut {l130} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1103: malloc(BYTES):   l130 = UNIQUE | NON_NULL, (empty)
    // 1103: fixed: typeof(_93) = *mut {l133} *mut {l134} i32
    // 1103: fixed:   l133 = UNIQUE | NON_NULL, (empty)
    // 1103: fixed:   l134 = UNIQUE | NON_NULL, (empty)
    // 1103: fixed = malloc( ... c_int: typeof((*_93) = move _91 as *mut i32 (Misc)) = *mut {l842} i32
    // 1103: fixed = malloc( ... c_int:   l842 = UNIQUE | NON_NULL, (empty)
    if fixed.is_null() {
    // 1104: fixed: typeof(_96) = *mut {l138} i32
    // 1104: fixed:   l138 = UNIQUE | NON_NULL, (empty)
    // 1104: fixed: typeof(_97) = *mut {l140} *mut {l141} i32
    // 1104: fixed:   l140 = UNIQUE | NON_NULL, (empty)
    // 1104: fixed:   l141 = UNIQUE | NON_NULL, (empty)
        die(b"out of memory\0" as *const u8 as *const libc::c_char);
        // 1105: b"out of memory ... _char: typeof(_100) = *const {l145} i8
        // 1105: b"out of memory ... _char:   l145 = UNIQUE | NON_NULL, (empty)
        // 1105: b"out of memory ... st u8: typeof(_101) = *const {l147} u8
        // 1105: b"out of memory ... st u8:   l147 = UNIQUE | NON_NULL, (empty)
        // 1105: b"out of memory\0": typeof(_102) = *const {l149} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1105: b"out of memory\0":   l149 = UNIQUE | NON_NULL, (empty)
        // 1105: b"out of memory\0": typeof(_103) = & {l151} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1105: b"out of memory\0":   l151 = UNIQUE | NON_NULL, FIXED
        // 1105: b"out of memory\0": typeof(_102 = &raw const (*_103)) = *const {l844} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1105: b"out of memory\0":   l844 = UNIQUE | NON_NULL, (empty)
        // 1105: b"out of memory ... st u8: typeof(_101 = move _102 as *const u8 (Pointer(ArrayToPointer))) = *const {l845} u8
        // 1105: b"out of memory ... st u8:   l845 = UNIQUE | NON_NULL, (empty)
        // 1105: b"out of memory ... _char: typeof(_100 = move _101 as *const i8 (Misc)) = *const {l846} i8
        // 1105: b"out of memory ... _char:   l846 = UNIQUE | NON_NULL, (empty)
        // 1105: b"out of memory\0": typeof(_103 = const b"out of memory\x00") = & {l843} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1105: b"out of memory\0":   l843 = UNIQUE | NON_NULL, (empty)
        exit(1 as libc::c_int);
    }
    memset(fixed as *mut libc::c_void, 0 as libc::c_int, BYTES);
    // 1108: memset(fixed as ... YTES): typeof(_106) = *mut {l155} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1108: memset(fixed as ... YTES):   l155 = UNIQUE | NON_NULL, (empty)
    // 1108: fixed as *mut l ... _void: typeof(_107) = *mut {l157} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1108: fixed as *mut l ... _void:   l157 = UNIQUE | NON_NULL, (empty)
    // 1108: fixed: typeof(_108) = *mut {l159} i32
    // 1108: fixed:   l159 = UNIQUE | NON_NULL, (empty)
    // 1108: fixed: typeof(_109) = *mut {l161} *mut {l162} i32
    // 1108: fixed:   l161 = UNIQUE | NON_NULL, (empty)
    // 1108: fixed:   l162 = UNIQUE | NON_NULL, (empty)
    // 1108: fixed as *mut l ... _void: typeof(_107 = move _108 as *mut libc::c_void (Misc)) = *mut {l847} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1108: fixed as *mut l ... _void:   l847 = UNIQUE | NON_NULL, (empty)
    incmem(BYTES);
    let mut BYTES_0: size_t = ((nvars + 1 as libc::c_int) as libc::c_ulong)
    // 1110: nvars: typeof(_118) = *mut {l172} i32
    // 1110: nvars:   l172 = UNIQUE | NON_NULL, (empty)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    vals = malloc(BYTES_0) as *mut libc::c_int;
    // 1112: malloc(BYTES_0): typeof(_123) = *mut {l178} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1112: malloc(BYTES_0):   l178 = UNIQUE | NON_NULL, (empty)
    // 1112: vals: typeof(_125) = *mut {l181} *mut {l182} i32
    // 1112: vals:   l181 = UNIQUE | NON_NULL, (empty)
    // 1112: vals:   l182 = UNIQUE | NON_NULL, (empty)
    // 1112: vals = malloc(B ... c_int: typeof((*_125) = move _123 as *mut i32 (Misc)) = *mut {l848} i32
    // 1112: vals = malloc(B ... c_int:   l848 = UNIQUE | NON_NULL, (empty)
    if vals.is_null() {
    // 1113: vals: typeof(_128) = *mut {l186} i32
    // 1113: vals:   l186 = UNIQUE | NON_NULL, (empty)
    // 1113: vals: typeof(_129) = *mut {l188} *mut {l189} i32
    // 1113: vals:   l188 = UNIQUE | NON_NULL, (empty)
    // 1113: vals:   l189 = UNIQUE | NON_NULL, (empty)
        die(b"out of memory\0" as *const u8 as *const libc::c_char);
        // 1114: b"out of memory ... _char: typeof(_132) = *const {l193} i8
        // 1114: b"out of memory ... _char:   l193 = UNIQUE | NON_NULL, (empty)
        // 1114: b"out of memory ... st u8: typeof(_133) = *const {l195} u8
        // 1114: b"out of memory ... st u8:   l195 = UNIQUE | NON_NULL, (empty)
        // 1114: b"out of memory\0": typeof(_134) = *const {l197} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1114: b"out of memory\0":   l197 = UNIQUE | NON_NULL, (empty)
        // 1114: b"out of memory\0": typeof(_135) = & {l199} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1114: b"out of memory\0":   l199 = UNIQUE | NON_NULL, FIXED
        // 1114: b"out of memory\0": typeof(_135 = const b"out of memory\x00") = & {l849} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1114: b"out of memory\0":   l849 = UNIQUE | NON_NULL, (empty)
        // 1114: b"out of memory ... _char: typeof(_132 = move _133 as *const i8 (Misc)) = *const {l852} i8
        // 1114: b"out of memory ... _char:   l852 = UNIQUE | NON_NULL, (empty)
        // 1114: b"out of memory ... st u8: typeof(_133 = move _134 as *const u8 (Pointer(ArrayToPointer))) = *const {l851} u8
        // 1114: b"out of memory ... st u8:   l851 = UNIQUE | NON_NULL, (empty)
        // 1114: b"out of memory\0": typeof(_134 = &raw const (*_135)) = *const {l850} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1114: b"out of memory\0":   l850 = UNIQUE | NON_NULL, (empty)
        exit(1 as libc::c_int);
    }
    memset(vals as *mut libc::c_void, 0 as libc::c_int, BYTES_0);
    // 1117: memset(vals as  ... ES_0): typeof(_138) = *mut {l203} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1117: memset(vals as  ... ES_0):   l203 = UNIQUE | NON_NULL, (empty)
    // 1117: vals as *mut li ... _void: typeof(_139) = *mut {l205} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1117: vals as *mut li ... _void:   l205 = UNIQUE | NON_NULL, (empty)
    // 1117: vals: typeof(_140) = *mut {l207} i32
    // 1117: vals:   l207 = UNIQUE | NON_NULL, (empty)
    // 1117: vals: typeof(_141) = *mut {l209} *mut {l210} i32
    // 1117: vals:   l209 = UNIQUE | NON_NULL, (empty)
    // 1117: vals:   l210 = UNIQUE | NON_NULL, (empty)
    // 1117: vals as *mut li ... _void: typeof(_139 = move _140 as *mut libc::c_void (Misc)) = *mut {l853} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1117: vals as *mut li ... _void:   l853 = UNIQUE | NON_NULL, (empty)
    incmem(BYTES_0);
    let mut BYTES_1: size_t = ((2 as libc::c_int * nvars + 1 as libc::c_int) as libc::c_ulong)
    // 1119: nvars: typeof(_152) = *mut {l222} i32
    // 1119: nvars:   l222 = UNIQUE | NON_NULL, (empty)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    occs = malloc(BYTES_1) as *mut libc::c_int;
    // 1121: malloc(BYTES_1): typeof(_158) = *mut {l229} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1121: malloc(BYTES_1):   l229 = UNIQUE | NON_NULL, (empty)
    // 1121: occs = malloc(B ... c_int: typeof(_7 = move _158 as *mut i32 (Misc)) = *mut {l854} i32
    // 1121: occs = malloc(B ... c_int:   l854 = UNIQUE | NON_NULL, (empty)
    if occs.is_null() {
    // 1122: occs: typeof(_162) = *mut {l234} i32
    // 1122: occs:   l234 = UNIQUE | NON_NULL, (empty)
        die(b"out of memory\0" as *const u8 as *const libc::c_char);
        // 1123: b"out of memory ... _char: typeof(_165) = *const {l238} i8
        // 1123: b"out of memory ... _char:   l238 = UNIQUE | NON_NULL, (empty)
        // 1123: b"out of memory ... st u8: typeof(_166) = *const {l240} u8
        // 1123: b"out of memory ... st u8:   l240 = UNIQUE | NON_NULL, (empty)
        // 1123: b"out of memory\0": typeof(_167) = *const {l242} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1123: b"out of memory\0":   l242 = UNIQUE | NON_NULL, (empty)
        // 1123: b"out of memory\0": typeof(_168) = & {l244} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1123: b"out of memory\0":   l244 = UNIQUE | NON_NULL, FIXED
        // 1123: b"out of memory\0": typeof(_167 = &raw const (*_168)) = *const {l856} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1123: b"out of memory\0":   l856 = UNIQUE | NON_NULL, (empty)
        // 1123: b"out of memory\0": typeof(_168 = const b"out of memory\x00") = & {l855} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1123: b"out of memory\0":   l855 = UNIQUE | NON_NULL, (empty)
        // 1123: b"out of memory ... _char: typeof(_165 = move _166 as *const i8 (Misc)) = *const {l858} i8
        // 1123: b"out of memory ... _char:   l858 = UNIQUE | NON_NULL, (empty)
        // 1123: b"out of memory ... st u8: typeof(_166 = move _167 as *const u8 (Pointer(ArrayToPointer))) = *const {l857} u8
        // 1123: b"out of memory ... st u8:   l857 = UNIQUE | NON_NULL, (empty)
        exit(1 as libc::c_int);
    }
    memset(occs as *mut libc::c_void, 0 as libc::c_int, BYTES_1);
    // 1126: memset(occs as  ... ES_1): typeof(_171) = *mut {l248} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1126: memset(occs as  ... ES_1):   l248 = UNIQUE | NON_NULL, (empty)
    // 1126: occs as *mut li ... _void: typeof(_172) = *mut {l250} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1126: occs as *mut li ... _void:   l250 = UNIQUE | NON_NULL, (empty)
    // 1126: occs: typeof(_173) = *mut {l252} i32
    // 1126: occs:   l252 = UNIQUE | NON_NULL, (empty)
    // 1126: occs as *mut li ... _void: typeof(_172 = move _173 as *mut libc::c_void (Misc)) = *mut {l859} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1126: occs as *mut li ... _void:   l859 = UNIQUE | NON_NULL, (empty)
    incmem(BYTES_1);
    occs = occs.offset(nvars as isize);
    // 1128: occs.offset(nva ... size): typeof(_178) = *mut {l258} i32
    // 1128: occs.offset(nva ... size):   l258 = UNIQUE | NON_NULL, (empty)
    // 1128: occs: typeof(_179) = *mut {l260} i32
    // 1128: occs:   l260 = UNIQUE | NON_NULL, (empty)
    // 1128: nvars: typeof(_182) = *mut {l264} i32
    // 1128: nvars:   l264 = UNIQUE | NON_NULL, (empty)
    if noeqs == 0 {
    // 1129: noeqs: typeof(_186) = *mut {l269} i32
    // 1129: noeqs:   l269 = UNIQUE | NON_NULL, (empty)
        let mut BYTES_2: size_t = ((nvars + 1 as libc::c_int) as libc::c_ulong)
        // 1130: nvars: typeof(_191) = *mut {l275} i32
        // 1130: nvars:   l275 = UNIQUE | NON_NULL, (empty)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
        repr = malloc(BYTES_2) as *mut libc::c_int;
        // 1132: malloc(BYTES_2): typeof(_196) = *mut {l281} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1132: malloc(BYTES_2):   l281 = UNIQUE | NON_NULL, (empty)
        // 1132: repr: typeof(_198) = *mut {l284} *mut {l285} i32
        // 1132: repr:   l284 = UNIQUE | NON_NULL, (empty)
        // 1132: repr:   l285 = UNIQUE | NON_NULL, (empty)
        // 1132: repr = malloc(B ... c_int: typeof((*_198) = move _196 as *mut i32 (Misc)) = *mut {l860} i32
        // 1132: repr = malloc(B ... c_int:   l860 = UNIQUE | NON_NULL, (empty)
        if repr.is_null() {
        // 1133: repr: typeof(_201) = *mut {l289} i32
        // 1133: repr:   l289 = UNIQUE | NON_NULL, (empty)
        // 1133: repr: typeof(_202) = *mut {l291} *mut {l292} i32
        // 1133: repr:   l291 = UNIQUE | NON_NULL, (empty)
        // 1133: repr:   l292 = UNIQUE | NON_NULL, (empty)
            die(b"out of memory\0" as *const u8 as *const libc::c_char);
            // 1134: b"out of memory ... _char: typeof(_205) = *const {l296} i8
            // 1134: b"out of memory ... _char:   l296 = UNIQUE | NON_NULL, (empty)
            // 1134: b"out of memory ... st u8: typeof(_206) = *const {l298} u8
            // 1134: b"out of memory ... st u8:   l298 = UNIQUE | NON_NULL, (empty)
            // 1134: b"out of memory\0": typeof(_207) = *const {l300} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
            // 1134: b"out of memory\0":   l300 = UNIQUE | NON_NULL, (empty)
            // 1134: b"out of memory\0": typeof(_208) = & {l302} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
            // 1134: b"out of memory\0":   l302 = UNIQUE | NON_NULL, FIXED
            // 1134: b"out of memory\0": typeof(_208 = const b"out of memory\x00") = & {l861} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
            // 1134: b"out of memory\0":   l861 = UNIQUE | NON_NULL, (empty)
            // 1134: b"out of memory ... _char: typeof(_205 = move _206 as *const i8 (Misc)) = *const {l864} i8
            // 1134: b"out of memory ... _char:   l864 = UNIQUE | NON_NULL, (empty)
            // 1134: b"out of memory\0": typeof(_207 = &raw const (*_208)) = *const {l862} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
            // 1134: b"out of memory\0":   l862 = UNIQUE | NON_NULL, (empty)
            // 1134: b"out of memory ... st u8: typeof(_206 = move _207 as *const u8 (Pointer(ArrayToPointer))) = *const {l863} u8
            // 1134: b"out of memory ... st u8:   l863 = UNIQUE | NON_NULL, (empty)
            exit(1 as libc::c_int);
        }
        memset(repr as *mut libc::c_void, 0 as libc::c_int, BYTES_2);
        // 1137: memset(repr as  ... ES_2): typeof(_211) = *mut {l306} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1137: memset(repr as  ... ES_2):   l306 = UNIQUE | NON_NULL, (empty)
        // 1137: repr as *mut li ... _void: typeof(_212) = *mut {l308} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1137: repr as *mut li ... _void:   l308 = UNIQUE | NON_NULL, (empty)
        // 1137: repr: typeof(_213) = *mut {l310} i32
        // 1137: repr:   l310 = UNIQUE | NON_NULL, (empty)
        // 1137: repr: typeof(_214) = *mut {l312} *mut {l313} i32
        // 1137: repr:   l312 = UNIQUE | NON_NULL, (empty)
        // 1137: repr:   l313 = UNIQUE | NON_NULL, (empty)
        // 1137: repr as *mut li ... _void: typeof(_212 = move _213 as *mut libc::c_void (Misc)) = *mut {l865} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1137: repr as *mut li ... _void:   l865 = UNIQUE | NON_NULL, (empty)
        incmem(BYTES_2);
    }
    minlen = 2147483647 as libc::c_int;
    maxlen = -(1 as libc::c_int);
    len = 0 as libc::c_int;
    loop {
        ch = getc(file);
        // 1144: file: typeof(_225) = *mut {l325} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 1144: file:   l325 = UNIQUE | NON_NULL, (empty)
        // 1144: file: typeof(_226) = *mut {l327} *mut {l328} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 1144: file:   l327 = UNIQUE | NON_NULL, (empty)
        // 1144: file:   l328 = UNIQUE | NON_NULL, (empty)
        if ch == 'c' as i32 {
            loop {
                ch = getc(file);
                // 1147: file: typeof(_231) = *mut {l334} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
                // 1147: file:   l334 = UNIQUE | NON_NULL, (empty)
                // 1147: file: typeof(_232) = *mut {l336} *mut {l337} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
                // 1147: file:   l336 = UNIQUE | NON_NULL, (empty)
                // 1147: file:   l337 = UNIQUE | NON_NULL, (empty)
                if !(ch != '\n' as i32) {
                    break;
                }
                if ch == -(1 as libc::c_int) {
                    return b"EOF in comment\0" as *const u8 as *const libc::c_char;
                    // 1152: b"EOF in commen ... st u8: typeof(_245) = *const {l351} u8
                    // 1152: b"EOF in commen ... st u8:   l351 = UNIQUE | NON_NULL, (empty)
                    // 1152: b"EOF in comment\0": typeof(_246) = *const {l353} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                    // 1152: b"EOF in comment\0":   l353 = UNIQUE | NON_NULL, (empty)
                    // 1152: b"EOF in comment\0": typeof(_247) = & {l355} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                    // 1152: b"EOF in comment\0":   l355 = UNIQUE | NON_NULL, FIXED
                    // 1152: b"EOF in comment\0": typeof(_247 = const b"EOF in comment\x00") = & {l866} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                    // 1152: b"EOF in comment\0":   l866 = UNIQUE | NON_NULL, (empty)
                    // 1152: b"EOF in commen ... st u8: typeof(_245 = move _246 as *const u8 (Pointer(ArrayToPointer))) = *const {l868} u8
                    // 1152: b"EOF in commen ... st u8:   l868 = UNIQUE | NON_NULL, (empty)
                    // 1152: b"EOF in commen ... _char: typeof(_0 = move _245 as *const i8 (Misc)) = *const {l869} i8
                    // 1152: b"EOF in commen ... _char:   l869 = UNIQUE | NON_NULL, (empty)
                    // 1152: b"EOF in comment\0": typeof(_246 = &raw const (*_247)) = *const {l867} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                    // 1152: b"EOF in comment\0":   l867 = UNIQUE | NON_NULL, (empty)
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
                // 1162: nclauses: typeof(_277) = *mut {l386} i32
                // 1162: nclauses:   l386 = UNIQUE | NON_NULL, (empty)
                    return b"not enough clauses\0" as *const u8 as *const libc::c_char;
                    // 1163: b"not enough cl ... st u8: typeof(_280) = *const {l390} u8
                    // 1163: b"not enough cl ... st u8:   l390 = UNIQUE | NON_NULL, (empty)
                    // 1163: b"not enough cl ... es\0": typeof(_281) = *const {l392} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 1163: b"not enough cl ... es\0":   l392 = UNIQUE | NON_NULL, (empty)
                    // 1163: b"not enough cl ... es\0": typeof(_282) = & {l394} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 1163: b"not enough cl ... es\0":   l394 = UNIQUE | NON_NULL, FIXED
                    // 1163: b"not enough cl ... es\0": typeof(_282 = const b"not enough clauses\x00") = & {l870} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 1163: b"not enough cl ... es\0":   l870 = UNIQUE | NON_NULL, (empty)
                    // 1163: b"not enough cl ... st u8: typeof(_280 = move _281 as *const u8 (Pointer(ArrayToPointer))) = *const {l872} u8
                    // 1163: b"not enough cl ... st u8:   l872 = UNIQUE | NON_NULL, (empty)
                    // 1163: b"not enough cl ... _char: typeof(_0 = move _280 as *const i8 (Misc)) = *const {l873} i8
                    // 1163: b"not enough cl ... _char:   l873 = UNIQUE | NON_NULL, (empty)
                    // 1163: b"not enough cl ... es\0": typeof(_281 = &raw const (*_282)) = *const {l871} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 1163: b"not enough cl ... es\0":   l871 = UNIQUE | NON_NULL, (empty)
                }
                if minlen == maxlen {
                    msg(
                        -(1 as libc::c_int),
                        0 as libc::c_int,
                        b"uniform clause length %d\0" as *const u8 as *const libc::c_char,
                        // 1169: b"uniform claus ... _char: typeof(_292) = *const {l405} i8
                        // 1169: b"uniform claus ... _char:   l405 = UNIQUE | NON_NULL, (empty)
                        // 1169: b"uniform claus ... st u8: typeof(_293) = *const {l407} u8
                        // 1169: b"uniform claus ... st u8:   l407 = UNIQUE | NON_NULL, (empty)
                        // 1169: b"uniform claus ... %d\0": typeof(_294) = *const {l409} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                        // 1169: b"uniform claus ... %d\0":   l409 = UNIQUE | NON_NULL, (empty)
                        // 1169: b"uniform claus ... %d\0": typeof(_295) = & {l411} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                        // 1169: b"uniform claus ... %d\0":   l411 = UNIQUE | NON_NULL, FIXED
                        // 1169: b"uniform claus ... _char: typeof(_292 = move _293 as *const i8 (Misc)) = *const {l877} i8
                        // 1169: b"uniform claus ... _char:   l877 = UNIQUE | NON_NULL, (empty)
                        // 1169: b"uniform claus ... %d\0": typeof(_294 = &raw const (*_295)) = *const {l875} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                        // 1169: b"uniform claus ... %d\0":   l875 = UNIQUE | NON_NULL, (empty)
                        // 1169: b"uniform claus ... %d\0": typeof(_295 = const b"uniform clause length %d\x00") = & {l874} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                        // 1169: b"uniform claus ... %d\0":   l874 = UNIQUE | NON_NULL, (empty)
                        // 1169: b"uniform claus ... st u8: typeof(_293 = move _294 as *const u8 (Pointer(ArrayToPointer))) = *const {l876} u8
                        // 1169: b"uniform claus ... st u8:   l876 = UNIQUE | NON_NULL, (empty)
                        minlen,
                    );
                } else {
                    msg(
                        -(1 as libc::c_int),
                        0 as libc::c_int,
                        b"clause length between %d and %d\0" as *const u8 as *const libc::c_char,
                        // 1176: b"clause length ... _char: typeof(_302) = *const {l419} i8
                        // 1176: b"clause length ... _char:   l419 = UNIQUE | NON_NULL, (empty)
                        // 1176: b"clause length ... st u8: typeof(_303) = *const {l421} u8
                        // 1176: b"clause length ... st u8:   l421 = UNIQUE | NON_NULL, (empty)
                        // 1176: b"clause length ... %d\0": typeof(_304) = *const {l423} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1176: b"clause length ... %d\0":   l423 = UNIQUE | NON_NULL, (empty)
                        // 1176: b"clause length ... %d\0": typeof(_305) = & {l425} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1176: b"clause length ... %d\0":   l425 = UNIQUE | NON_NULL, FIXED
                        // 1176: b"clause length ... %d\0": typeof(_304 = &raw const (*_305)) = *const {l879} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1176: b"clause length ... %d\0":   l879 = UNIQUE | NON_NULL, (empty)
                        // 1176: b"clause length ... %d\0": typeof(_305 = const b"clause length between %d and %d\x00") = & {l878} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1176: b"clause length ... %d\0":   l878 = UNIQUE | NON_NULL, (empty)
                        // 1176: b"clause length ... _char: typeof(_302 = move _303 as *const i8 (Misc)) = *const {l881} i8
                        // 1176: b"clause length ... _char:   l881 = UNIQUE | NON_NULL, (empty)
                        // 1176: b"clause length ... st u8: typeof(_303 = move _304 as *const u8 (Pointer(ArrayToPointer))) = *const {l880} u8
                        // 1176: b"clause length ... st u8:   l880 = UNIQUE | NON_NULL, (empty)
                        minlen,
                        maxlen,
                    );
                }
                if nvars > 0 as libc::c_int {
                // 1181: nvars: typeof(_311) = *mut {l432} i32
                // 1181: nvars:   l432 = UNIQUE | NON_NULL, (empty)
                    avg = 0 as libc::c_int as libc::c_double;
                    lit = -nvars;
                    // 1183: nvars: typeof(_315) = *mut {l437} i32
                    // 1183: nvars:   l437 = UNIQUE | NON_NULL, (empty)
                    while lit <= nvars {
                    // 1184: nvars: typeof(_321) = *mut {l444} i32
                    // 1184: nvars:   l444 = UNIQUE | NON_NULL, (empty)
                        if lit != 0 {
                            avg += *occs.offset(lit as isize) as libc::c_double;
                            // 1186: occs.offset(lit ... size): typeof(_327) = *mut {l451} i32
                            // 1186: occs.offset(lit ... size):   l451 = UNIQUE | NON_NULL, (empty)
                            // 1186: occs: typeof(_328) = *mut {l453} i32
                            // 1186: occs:   l453 = UNIQUE | NON_NULL, (empty)
                        }
                        lit += 1;
                        lit;
                    }
                    avg /= (2 as libc::c_int * nvars) as libc::c_double;
                    // 1191: nvars: typeof(_340) = *mut {l466} i32
                    // 1191: nvars:   l466 = UNIQUE | NON_NULL, (empty)
                    msg(
                        -(1 as libc::c_int),
                        0 as libc::c_int,
                        b"average literal occurrence %.2f\0" as *const u8 as *const libc::c_char,
                        // 1195: b"average liter ... _char: typeof(_347) = *const {l474} i8
                        // 1195: b"average liter ... _char:   l474 = UNIQUE | NON_NULL, (empty)
                        // 1195: b"average liter ... st u8: typeof(_348) = *const {l476} u8
                        // 1195: b"average liter ... st u8:   l476 = UNIQUE | NON_NULL, (empty)
                        // 1195: b"average liter ... 2f\0": typeof(_349) = *const {l478} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1195: b"average liter ... 2f\0":   l478 = UNIQUE | NON_NULL, (empty)
                        // 1195: b"average liter ... 2f\0": typeof(_350) = & {l480} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1195: b"average liter ... 2f\0":   l480 = UNIQUE | NON_NULL, FIXED
                        // 1195: b"average liter ... _char: typeof(_347 = move _348 as *const i8 (Misc)) = *const {l885} i8
                        // 1195: b"average liter ... _char:   l885 = UNIQUE | NON_NULL, (empty)
                        // 1195: b"average liter ... 2f\0": typeof(_350 = const b"average literal occurrence %.2f\x00") = & {l882} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1195: b"average liter ... 2f\0":   l882 = UNIQUE | NON_NULL, (empty)
                        // 1195: b"average liter ... st u8: typeof(_348 = move _349 as *const u8 (Pointer(ArrayToPointer))) = *const {l884} u8
                        // 1195: b"average liter ... st u8:   l884 = UNIQUE | NON_NULL, (empty)
                        // 1195: b"average liter ... 2f\0": typeof(_349 = &raw const (*_350)) = *const {l883} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1195: b"average liter ... 2f\0":   l883 = UNIQUE | NON_NULL, (empty)
                        avg,
                    );
                    std = 0 as libc::c_int as libc::c_double;
                    lit = -nvars;
                    // 1199: nvars: typeof(_354) = *mut {l485} i32
                    // 1199: nvars:   l485 = UNIQUE | NON_NULL, (empty)
                    while lit <= nvars {
                    // 1200: nvars: typeof(_360) = *mut {l492} i32
                    // 1200: nvars:   l492 = UNIQUE | NON_NULL, (empty)
                        if lit != 0 {
                            let mut diff: libc::c_double =
                                avg - *occs.offset(lit as isize) as libc::c_double;
                                // 1203: occs.offset(lit ... size): typeof(_368) = *mut {l501} i32
                                // 1203: occs.offset(lit ... size):   l501 = UNIQUE | NON_NULL, (empty)
                                // 1203: occs: typeof(_369) = *mut {l503} i32
                                // 1203: occs:   l503 = UNIQUE | NON_NULL, (empty)
                            std += diff * diff;
                        }
                        lit += 1;
                        lit;
                    }
                    std = sqrt(std / (2 as libc::c_int * nvars) as libc::c_double);
                    // 1209: nvars: typeof(_387) = *mut {l522} i32
                    // 1209: nvars:   l522 = UNIQUE | NON_NULL, (empty)
                    msg(
                        -(1 as libc::c_int),
                        0 as libc::c_int,
                        b"literal occurrence standard deviation %.2f\0" as *const u8
                        // 1213: b"literal occur ... _char: typeof(_394) = *const {l530} i8
                        // 1213: b"literal occur ... _char:   l530 = UNIQUE | NON_NULL, (empty)
                        // 1213: b"literal occur ... st u8: typeof(_395) = *const {l532} u8
                        // 1213: b"literal occur ... st u8:   l532 = UNIQUE | NON_NULL, (empty)
                        // 1213: b"literal occur ... 2f\0": typeof(_396) = *const {l534} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                        // 1213: b"literal occur ... 2f\0":   l534 = UNIQUE | NON_NULL, (empty)
                        // 1213: b"literal occur ... 2f\0": typeof(_397) = & {l536} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                        // 1213: b"literal occur ... 2f\0":   l536 = UNIQUE | NON_NULL, FIXED
                        // 1213: b"literal occur ... _char: typeof(_394 = move _395 as *const i8 (Misc)) = *const {l889} i8
                        // 1213: b"literal occur ... _char:   l889 = UNIQUE | NON_NULL, (empty)
                        // 1213: b"literal occur ... st u8: typeof(_395 = move _396 as *const u8 (Pointer(ArrayToPointer))) = *const {l888} u8
                        // 1213: b"literal occur ... st u8:   l888 = UNIQUE | NON_NULL, (empty)
                        // 1213: b"literal occur ... 2f\0": typeof(_397 = const b"literal occurrence standard deviation %.2f\x00") = & {l886} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                        // 1213: b"literal occur ... 2f\0":   l886 = UNIQUE | NON_NULL, (empty)
                        // 1213: b"literal occur ... 2f\0": typeof(_396 = &raw const (*_397)) = *const {l887} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                        // 1213: b"literal occur ... 2f\0":   l887 = UNIQUE | NON_NULL, (empty)
                            as *const libc::c_char,
                        std,
                    );
                    if avg > 0 as libc::c_int as libc::c_double {
                        msg(
                            -(1 as libc::c_int),
                            0 as libc::c_int,
                            b"relative literal occurrence standard deviation %.2f%%\0" as *const u8
                            // 1221: b"relative lite ... _char: typeof(_408) = *const {l548} i8
                            // 1221: b"relative lite ... _char:   l548 = UNIQUE | NON_NULL, (empty)
                            // 1221: b"relative lite ... st u8: typeof(_409) = *const {l550} u8
                            // 1221: b"relative lite ... st u8:   l550 = UNIQUE | NON_NULL, (empty)
                            // 1221: b"relative lite ... %%\0": typeof(_410) = *const {l552} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000036)) }]
                            // 1221: b"relative lite ... %%\0":   l552 = UNIQUE | NON_NULL, (empty)
                            // 1221: b"relative lite ... %%\0": typeof(_411) = & {l554} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000036)) }]
                            // 1221: b"relative lite ... %%\0":   l554 = UNIQUE | NON_NULL, FIXED
                            // 1221: b"relative lite ... st u8: typeof(_409 = move _410 as *const u8 (Pointer(ArrayToPointer))) = *const {l892} u8
                            // 1221: b"relative lite ... st u8:   l892 = UNIQUE | NON_NULL, (empty)
                            // 1221: b"relative lite ... _char: typeof(_408 = move _409 as *const i8 (Misc)) = *const {l893} i8
                            // 1221: b"relative lite ... _char:   l893 = UNIQUE | NON_NULL, (empty)
                            // 1221: b"relative lite ... %%\0": typeof(_410 = &raw const (*_411)) = *const {l891} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000036)) }]
                            // 1221: b"relative lite ... %%\0":   l891 = UNIQUE | NON_NULL, (empty)
                            // 1221: b"relative lite ... %%\0": typeof(_411 = const b"relative literal occurrence standard deviation %.2f%%\x00") = & {l890} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000036)) }]
                            // 1221: b"relative lite ... %%\0":   l890 = UNIQUE | NON_NULL, (empty)
                                as *const libc::c_char,
                            100.0f64 * std / avg,
                        );
                    }
                } else {
                    std = 0 as libc::c_int as libc::c_double;
                    avg = std;
                }
                occs = occs.offset(-(nvars as isize));
                // 1230: occs.offset(-(n ... ize)): typeof(_418) = *mut {l562} i32
                // 1230: occs.offset(-(n ... ize)):   l562 = UNIQUE | NON_NULL, (empty)
                // 1230: occs: typeof(_419) = *mut {l564} i32
                // 1230: occs:   l564 = UNIQUE | NON_NULL, (empty)
                // 1230: nvars: typeof(_423) = *mut {l569} i32
                // 1230: nvars:   l569 = UNIQUE | NON_NULL, (empty)
                let mut BYTES_3: size_t = ((2 as libc::c_int * nvars + 1 as libc::c_int)
                // 1231: nvars: typeof(_431) = *mut {l578} i32
                // 1231: nvars:   l578 = UNIQUE | NON_NULL, (empty)
                    as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
                decmem(BYTES_3);
                free(occs as *mut libc::c_void);
                // 1235: occs as *mut li ... _void: typeof(_440) = *mut {l588} DefId(2:5583 ~ core[480a]::ffi::c_void)
                // 1235: occs as *mut li ... _void:   l588 = UNIQUE | NON_NULL, (empty)
                // 1235: occs: typeof(_441) = *mut {l590} i32
                // 1235: occs:   l590 = UNIQUE | NON_NULL, (empty)
                // 1235: occs as *mut li ... _void: typeof(_440 = move _441 as *mut libc::c_void (Misc)) = *mut {l894} DefId(2:5583 ~ core[480a]::ffi::c_void)
                // 1235: occs as *mut li ... _void:   l894 = UNIQUE | NON_NULL, (empty)
                if avg > 5 as libc::c_int as libc::c_double && std / avg < 0.5f64 {
                    if minlen == maxlen {
                        locs = 2 as libc::c_int;
                        // 1238: locs: typeof(_456) = *mut {l606} i32
                        // 1238: locs:   l606 = UNIQUE | NON_NULL, (empty)
                        msg(
                            -(1 as libc::c_int),
                            0 as libc::c_int,
                            b"looks like a real uniform random instance\0" as *const u8
                            // 1242: b"looks like a  ... _char: typeof(_462) = *const {l613} i8
                            // 1242: b"looks like a  ... _char:   l613 = UNIQUE | NON_NULL, (empty)
                            // 1242: b"looks like a  ... st u8: typeof(_463) = *const {l615} u8
                            // 1242: b"looks like a  ... st u8:   l615 = UNIQUE | NON_NULL, (empty)
                            // 1242: b"looks like a  ... ce\0": typeof(_464) = *const {l617} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                            // 1242: b"looks like a  ... ce\0":   l617 = UNIQUE | NON_NULL, (empty)
                            // 1242: b"looks like a  ... ce\0": typeof(_465) = & {l619} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                            // 1242: b"looks like a  ... ce\0":   l619 = UNIQUE | NON_NULL, FIXED
                            // 1242: b"looks like a  ... ce\0": typeof(_464 = &raw const (*_465)) = *const {l896} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                            // 1242: b"looks like a  ... ce\0":   l896 = UNIQUE | NON_NULL, (empty)
                            // 1242: b"looks like a  ... st u8: typeof(_463 = move _464 as *const u8 (Pointer(ArrayToPointer))) = *const {l897} u8
                            // 1242: b"looks like a  ... st u8:   l897 = UNIQUE | NON_NULL, (empty)
                            // 1242: b"looks like a  ... ce\0": typeof(_465 = const b"looks like a real uniform random instance\x00") = & {l895} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                            // 1242: b"looks like a  ... ce\0":   l895 = UNIQUE | NON_NULL, (empty)
                            // 1242: b"looks like a  ... _char: typeof(_462 = move _463 as *const i8 (Misc)) = *const {l898} i8
                            // 1242: b"looks like a  ... _char:   l898 = UNIQUE | NON_NULL, (empty)
                                as *const libc::c_char,
                        );
                    } else {
                        locs = 1 as libc::c_int;
                        // 1246: locs: typeof(_467) = *mut {l622} i32
                        // 1246: locs:   l622 = UNIQUE | NON_NULL, (empty)
                        msg(
                            -(1 as libc::c_int),
                            0 as libc::c_int,
                            b"looks like random instance without uniform clause length\0"
                            // 1250: b"looks like ra ... _char: typeof(_473) = *const {l629} i8
                            // 1250: b"looks like ra ... _char:   l629 = UNIQUE | NON_NULL, (empty)
                            // 1250: b"looks like ra ... st u8: typeof(_474) = *const {l631} u8
                            // 1250: b"looks like ra ... st u8:   l631 = UNIQUE | NON_NULL, (empty)
                            // 1250: b"looks like ra ... th\0": typeof(_475) = *const {l633} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                            // 1250: b"looks like ra ... th\0":   l633 = UNIQUE | NON_NULL, (empty)
                            // 1250: b"looks like ra ... th\0": typeof(_476) = & {l635} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                            // 1250: b"looks like ra ... th\0":   l635 = UNIQUE | NON_NULL, FIXED
                            // 1250: b"looks like ra ... th\0": typeof(_476 = const b"looks like random instance without uniform clause length\x00") = & {l899} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                            // 1250: b"looks like ra ... th\0":   l899 = UNIQUE | NON_NULL, (empty)
                            // 1250: b"looks like ra ... st u8: typeof(_474 = move _475 as *const u8 (Pointer(ArrayToPointer))) = *const {l901} u8
                            // 1250: b"looks like ra ... st u8:   l901 = UNIQUE | NON_NULL, (empty)
                            // 1250: b"looks like ra ... th\0": typeof(_475 = &raw const (*_476)) = *const {l900} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                            // 1250: b"looks like ra ... th\0":   l900 = UNIQUE | NON_NULL, (empty)
                            // 1250: b"looks like ra ... _char: typeof(_473 = move _474 as *const i8 (Misc)) = *const {l902} i8
                            // 1250: b"looks like ra ... _char:   l902 = UNIQUE | NON_NULL, (empty)
                                as *const u8 as *const libc::c_char,
                        );
                    }
                } else {
                    locs = 0 as libc::c_int;
                    // 1255: locs: typeof(_478) = *mut {l638} i32
                    // 1255: locs:   l638 = UNIQUE | NON_NULL, (empty)
                    msg(
                        -(1 as libc::c_int),
                        0 as libc::c_int,
                        b"instance does not seem to be uniform random\0" as *const u8
                        // 1259: b"instance does ... _char: typeof(_484) = *const {l645} i8
                        // 1259: b"instance does ... _char:   l645 = UNIQUE | NON_NULL, (empty)
                        // 1259: b"instance does ... st u8: typeof(_485) = *const {l647} u8
                        // 1259: b"instance does ... st u8:   l647 = UNIQUE | NON_NULL, (empty)
                        // 1259: b"instance does ... om\0": typeof(_486) = *const {l649} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                        // 1259: b"instance does ... om\0":   l649 = UNIQUE | NON_NULL, (empty)
                        // 1259: b"instance does ... om\0": typeof(_487) = & {l651} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                        // 1259: b"instance does ... om\0":   l651 = UNIQUE | NON_NULL, FIXED
                        // 1259: b"instance does ... om\0": typeof(_487 = const b"instance does not seem to be uniform random\x00") = & {l903} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                        // 1259: b"instance does ... om\0":   l903 = UNIQUE | NON_NULL, (empty)
                        // 1259: b"instance does ... _char: typeof(_484 = move _485 as *const i8 (Misc)) = *const {l906} i8
                        // 1259: b"instance does ... _char:   l906 = UNIQUE | NON_NULL, (empty)
                        // 1259: b"instance does ... st u8: typeof(_485 = move _486 as *const u8 (Pointer(ArrayToPointer))) = *const {l905} u8
                        // 1259: b"instance does ... st u8:   l905 = UNIQUE | NON_NULL, (empty)
                        // 1259: b"instance does ... om\0": typeof(_486 = &raw const (*_487)) = *const {l904} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                        // 1259: b"instance does ... om\0":   l904 = UNIQUE | NON_NULL, (empty)
                            as *const libc::c_char,
                    );
                }
                msg(
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    b"finished parsing in %.1f seconds\0" as *const u8 as *const libc::c_char,
                    // 1266: b"finished pars ... _char: typeof(_493) = *const {l658} i8
                    // 1266: b"finished pars ... _char:   l658 = UNIQUE | NON_NULL, (empty)
                    // 1266: b"finished pars ... st u8: typeof(_494) = *const {l660} u8
                    // 1266: b"finished pars ... st u8:   l660 = UNIQUE | NON_NULL, (empty)
                    // 1266: b"finished pars ... ds\0": typeof(_495) = *const {l662} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                    // 1266: b"finished pars ... ds\0":   l662 = UNIQUE | NON_NULL, (empty)
                    // 1266: b"finished pars ... ds\0": typeof(_496) = & {l664} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                    // 1266: b"finished pars ... ds\0":   l664 = UNIQUE | NON_NULL, FIXED
                    // 1266: b"finished pars ... _char: typeof(_493 = move _494 as *const i8 (Misc)) = *const {l910} i8
                    // 1266: b"finished pars ... _char:   l910 = UNIQUE | NON_NULL, (empty)
                    // 1266: b"finished pars ... st u8: typeof(_494 = move _495 as *const u8 (Pointer(ArrayToPointer))) = *const {l909} u8
                    // 1266: b"finished pars ... st u8:   l909 = UNIQUE | NON_NULL, (empty)
                    // 1266: b"finished pars ... ds\0": typeof(_496 = const b"finished parsing in %.1f seconds\x00") = & {l907} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                    // 1266: b"finished pars ... ds\0":   l907 = UNIQUE | NON_NULL, (empty)
                    // 1266: b"finished pars ... ds\0": typeof(_495 = &raw const (*_496)) = *const {l908} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                    // 1266: b"finished pars ... ds\0":   l908 = UNIQUE | NON_NULL, (empty)
                    getime(),
                );
                return 0 as *const libc::c_char;
                // 1269: 0 as *const lib ... _char: typeof(_0 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l911} i8
                // 1269: 0 as *const lib ... _char:   l911 = UNIQUE | NON_NULL, (empty)
            }
            if ch == '-' as i32 {
                ch = getc(file);
                // 1272: file: typeof(_503) = *mut {l672} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
                // 1272: file:   l672 = UNIQUE | NON_NULL, (empty)
                // 1272: file: typeof(_504) = *mut {l674} *mut {l675} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
                // 1272: file:   l674 = UNIQUE | NON_NULL, (empty)
                // 1272: file:   l675 = UNIQUE | NON_NULL, (empty)
                sign = -(1 as libc::c_int);
            } else {
                sign = 1 as libc::c_int;
            }
            if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
            // 1277: (*__ctype_b_loc ... size): typeof(_513) = *const {l685} u16
            // 1277: (*__ctype_b_loc ... size):   l685 = UNIQUE | NON_NULL, (empty)
            // 1277: (*__ctype_b_loc()): typeof(_514) = *const {l687} u16
            // 1277: (*__ctype_b_loc()):   l687 = UNIQUE | NON_NULL, (empty)
            // 1277: __ctype_b_loc(): typeof(_515) = *mut {l689} *const {l690} u16
            // 1277: __ctype_b_loc():   l689 = UNIQUE | NON_NULL, (empty)
            // 1277: __ctype_b_loc():   l690 = UNIQUE | NON_NULL, (empty)
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                == 0
            {
                return b"expected digit\0" as *const u8 as *const libc::c_char;
                // 1281: b"expected digi ... st u8: typeof(_522) = *const {l698} u8
                // 1281: b"expected digi ... st u8:   l698 = UNIQUE | NON_NULL, (empty)
                // 1281: b"expected digit\0": typeof(_523) = *const {l700} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                // 1281: b"expected digit\0":   l700 = UNIQUE | NON_NULL, (empty)
                // 1281: b"expected digit\0": typeof(_524) = & {l702} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                // 1281: b"expected digit\0":   l702 = UNIQUE | NON_NULL, FIXED
                // 1281: b"expected digit\0": typeof(_523 = &raw const (*_524)) = *const {l913} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                // 1281: b"expected digit\0":   l913 = UNIQUE | NON_NULL, (empty)
                // 1281: b"expected digi ... st u8: typeof(_522 = move _523 as *const u8 (Pointer(ArrayToPointer))) = *const {l914} u8
                // 1281: b"expected digi ... st u8:   l914 = UNIQUE | NON_NULL, (empty)
                // 1281: b"expected digi ... _char: typeof(_0 = move _522 as *const i8 (Misc)) = *const {l915} i8
                // 1281: b"expected digi ... _char:   l915 = UNIQUE | NON_NULL, (empty)
                // 1281: b"expected digit\0": typeof(_524 = const b"expected digit\x00") = & {l912} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                // 1281: b"expected digit\0":   l912 = UNIQUE | NON_NULL, (empty)
            }
            if nclauses == 0 {
            // 1283: nclauses: typeof(_528) = *mut {l707} i32
            // 1283: nclauses:   l707 = UNIQUE | NON_NULL, (empty)
                return b"too many clauses\0" as *const u8 as *const libc::c_char;
                // 1284: b"too many clau ... st u8: typeof(_530) = *const {l710} u8
                // 1284: b"too many clau ... st u8:   l710 = UNIQUE | NON_NULL, (empty)
                // 1284: b"too many clau ... es\0": typeof(_531) = *const {l712} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 1284: b"too many clau ... es\0":   l712 = UNIQUE | NON_NULL, (empty)
                // 1284: b"too many clau ... es\0": typeof(_532) = & {l714} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 1284: b"too many clau ... es\0":   l714 = UNIQUE | NON_NULL, FIXED
                // 1284: b"too many clau ... es\0": typeof(_532 = const b"too many clauses\x00") = & {l916} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 1284: b"too many clau ... es\0":   l916 = UNIQUE | NON_NULL, (empty)
                // 1284: b"too many clau ... _char: typeof(_0 = move _530 as *const i8 (Misc)) = *const {l919} i8
                // 1284: b"too many clau ... _char:   l919 = UNIQUE | NON_NULL, (empty)
                // 1284: b"too many clau ... st u8: typeof(_530 = move _531 as *const u8 (Pointer(ArrayToPointer))) = *const {l918} u8
                // 1284: b"too many clau ... st u8:   l918 = UNIQUE | NON_NULL, (empty)
                // 1284: b"too many clau ... es\0": typeof(_531 = &raw const (*_532)) = *const {l917} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 1284: b"too many clau ... es\0":   l917 = UNIQUE | NON_NULL, (empty)
            }
            lit = ch - '0' as i32;
            loop {
                ch = getc(file);
                // 1288: file: typeof(_538) = *mut {l721} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
                // 1288: file:   l721 = UNIQUE | NON_NULL, (empty)
                // 1288: file: typeof(_539) = *mut {l723} *mut {l724} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
                // 1288: file:   l723 = UNIQUE | NON_NULL, (empty)
                // 1288: file:   l724 = UNIQUE | NON_NULL, (empty)
                if !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                // 1289: (*__ctype_b_loc ... size): typeof(_546) = *const {l732} u16
                // 1289: (*__ctype_b_loc ... size):   l732 = UNIQUE | NON_NULL, (empty)
                // 1289: (*__ctype_b_loc()): typeof(_547) = *const {l734} u16
                // 1289: (*__ctype_b_loc()):   l734 = UNIQUE | NON_NULL, (empty)
                // 1289: __ctype_b_loc(): typeof(_548) = *mut {l736} *const {l737} u16
                // 1289: __ctype_b_loc():   l736 = UNIQUE | NON_NULL, (empty)
                // 1289: __ctype_b_loc():   l737 = UNIQUE | NON_NULL, (empty)
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                    != 0)
                {
                    break;
                }
                lit = 10 as libc::c_int * lit + (ch - '0' as i32);
            }
            if lit < 0 as libc::c_int || lit > nvars {
            // 1297: nvars: typeof(_572) = *mut {l762} i32
            // 1297: nvars:   l762 = UNIQUE | NON_NULL, (empty)
                return b"invalid variable index\0" as *const u8 as *const libc::c_char;
                // 1298: b"invalid varia ... st u8: typeof(_574) = *const {l765} u8
                // 1298: b"invalid varia ... st u8:   l765 = UNIQUE | NON_NULL, (empty)
                // 1298: b"invalid varia ... ex\0": typeof(_575) = *const {l767} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                // 1298: b"invalid varia ... ex\0":   l767 = UNIQUE | NON_NULL, (empty)
                // 1298: b"invalid varia ... ex\0": typeof(_576) = & {l769} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                // 1298: b"invalid varia ... ex\0":   l769 = UNIQUE | NON_NULL, FIXED
                // 1298: b"invalid varia ... st u8: typeof(_574 = move _575 as *const u8 (Pointer(ArrayToPointer))) = *const {l922} u8
                // 1298: b"invalid varia ... st u8:   l922 = UNIQUE | NON_NULL, (empty)
                // 1298: b"invalid varia ... ex\0": typeof(_576 = const b"invalid variable index\x00") = & {l920} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                // 1298: b"invalid varia ... ex\0":   l920 = UNIQUE | NON_NULL, (empty)
                // 1298: b"invalid varia ... ex\0": typeof(_575 = &raw const (*_576)) = *const {l921} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                // 1298: b"invalid varia ... ex\0":   l921 = UNIQUE | NON_NULL, (empty)
                // 1298: b"invalid varia ... _char: typeof(_0 = move _574 as *const i8 (Misc)) = *const {l923} i8
                // 1298: b"invalid varia ... _char:   l923 = UNIQUE | NON_NULL, (empty)
            }
            lit *= sign;
            lgladd((*workers.offset(0 as libc::c_int as isize)).lgl, lit);
            // 1301: (*workers.offse ... ).lgl: typeof(_580) = *mut {l774} LGL
            // 1301: (*workers.offse ... ).lgl:   l774 = UNIQUE | NON_NULL, (empty)
            // 1301: workers.offset( ... size): typeof(_581) = *mut {l776} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 1301: workers.offset( ... size):   l776 = UNIQUE | NON_NULL, (empty)
            // 1301: workers: typeof(_582) = *mut {l778} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 1301: workers:   l778 = UNIQUE | NON_NULL, (empty)
            // 1301: workers: typeof(_583) = *mut {l780} *mut {l781} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 1301: workers:   l780 = UNIQUE | NON_NULL, (empty)
            // 1301: workers:   l781 = UNIQUE | NON_NULL, (empty)
            if lit == 0 {
                nclauses -= 1;
                // 1303: nclauses: typeof(_589) = *mut {l788} i32
                // 1303: nclauses:   l788 = UNIQUE | NON_NULL, (empty)
                nclauses;
                // 1304: nclauses: typeof(_592) = *mut {l792} i32
                // 1304: nclauses:   l792 = UNIQUE | NON_NULL, (empty)
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
                let ref mut fresh0 = *occs.offset(lit as isize);
                // 1315: ref mut fresh0: typeof(_606) = &mut {l807} i32
                // 1315: ref mut fresh0:   l807 = UNIQUE | NON_NULL, FIXED
                // 1315: occs.offset(lit ... size): typeof(_607) = *mut {l809} i32
                // 1315: occs.offset(lit ... size):   l809 = UNIQUE | NON_NULL, (empty)
                // 1315: occs: typeof(_608) = *mut {l811} i32
                // 1315: occs:   l811 = UNIQUE | NON_NULL, (empty)
                // 1315: ref mut fresh0: typeof(_606 = &mut (*_607)) = &mut {l924} i32
                // 1315: ref mut fresh0:   l924 = UNIQUE | NON_NULL, (empty)
                *fresh0 += 1;
                *fresh0;
            }
        }
    }
}
unsafe extern "C" fn isposnum(mut str: *const libc::c_char) -> libc::c_int {
// 1322: mut str: typeof(_1) = *const {g13} i8
// 1322: mut str:   g13 = (empty), FIXED
    let mut ch: libc::c_int = 0;
    let fresh1 = str;
    // 1324: fresh1: typeof(_4) = *const {l4} i8
    // 1324: fresh1:   l4 = UNIQUE | NON_NULL, (empty)
    str = str.offset(1);
    // 1325: str.offset(1): typeof(_5) = *const {l6} i8
    // 1325: str.offset(1):   l6 = UNIQUE | NON_NULL, (empty)
    // 1325: str: typeof(_6) = *const {l8} i8
    // 1325: str:   l8 = UNIQUE | NON_NULL, (empty)
    ch = *fresh1 as libc::c_int;
    if ch == 0
        || *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
        // 1328: (*__ctype_b_loc ... size): typeof(_16) = *const {l19} u16
        // 1328: (*__ctype_b_loc ... size):   l19 = UNIQUE | NON_NULL, (empty)
        // 1328: (*__ctype_b_loc()): typeof(_17) = *const {l21} u16
        // 1328: (*__ctype_b_loc()):   l21 = UNIQUE | NON_NULL, (empty)
        // 1328: __ctype_b_loc(): typeof(_18) = *mut {l23} *const {l24} u16
        // 1328: __ctype_b_loc():   l23 = UNIQUE | NON_NULL, (empty)
        // 1328: __ctype_b_loc():   l24 = UNIQUE | NON_NULL, (empty)
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        return 0 as libc::c_int;
    }
    loop {
        let fresh2 = str;
        // 1335: fresh2: typeof(_27) = *const {l34} i8
        // 1335: fresh2:   l34 = UNIQUE | NON_NULL, (empty)
        str = str.offset(1);
        // 1336: str.offset(1): typeof(_28) = *const {l36} i8
        // 1336: str.offset(1):   l36 = UNIQUE | NON_NULL, (empty)
        // 1336: str: typeof(_29) = *const {l38} i8
        // 1336: str:   l38 = UNIQUE | NON_NULL, (empty)
        ch = *fresh2 as libc::c_int;
        if !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
        // 1338: (*__ctype_b_loc ... size): typeof(_36) = *const {l46} u16
        // 1338: (*__ctype_b_loc ... size):   l46 = UNIQUE | NON_NULL, (empty)
        // 1338: (*__ctype_b_loc()): typeof(_37) = *const {l48} u16
        // 1338: (*__ctype_b_loc()):   l48 = UNIQUE | NON_NULL, (empty)
        // 1338: __ctype_b_loc(): typeof(_38) = *mut {l50} *const {l51} u16
        // 1338: __ctype_b_loc():   l50 = UNIQUE | NON_NULL, (empty)
        // 1338: __ctype_b_loc():   l51 = UNIQUE | NON_NULL, (empty)
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0)
        {
            break;
        }
    }
    return (ch == 0) as libc::c_int;
}
unsafe extern "C" fn term(mut voidptr: *mut libc::c_void) -> libc::c_int {
// 1347: mut voidptr: typeof(_1) = *mut {g14} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 1347: mut voidptr:   g14 = UNIQUE | NON_NULL, FIXED
    let mut worker: *mut Worker = voidptr as *mut Worker;
    // 1348: mut worker: typeof(_3) = *mut {l3} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1348: mut worker:   l3 = UNIQUE | NON_NULL, (empty)
    // 1348: voidptr: typeof(_4) = *mut {l5} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1348: voidptr:   l5 = UNIQUE | NON_NULL, (empty)
    // 1348: voidptr as *mut ... orker: typeof(_3 = move _4 as *mut Worker (Misc)) = *mut {l103} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1348: voidptr as *mut ... orker:   l103 = UNIQUE | NON_NULL, (empty)
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    // 1349: worker: typeof(_8) = *mut {l10} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1349: worker:   l10 = UNIQUE | NON_NULL, (empty)
    // 1349: workers: typeof(_9) = *const {l12} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1349: workers:   l12 = UNIQUE | NON_NULL, (empty)
    // 1349: workers: typeof(_10) = *mut {l14} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1349: workers:   l14 = UNIQUE | NON_NULL, (empty)
    // 1349: workers: typeof(_11) = *mut {l16} *mut {l17} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1349: workers:   l16 = UNIQUE | NON_NULL, (empty)
    // 1349: workers:   l17 = UNIQUE | NON_NULL, (empty)
    // 1349: workers: typeof(_9 = move _10 as *const Worker (Pointer(MutToConstPointer))) = *const {l104} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1349: workers:   l104 = UNIQUE | NON_NULL, (empty)
    let mut res: libc::c_int = 0;
    msg(
        wid,
        3 as libc::c_int,
        b"checking early termination\0" as *const u8 as *const libc::c_char,
        // 1354: b"checking earl ... _char: typeof(_16) = *const {l23} i8
        // 1354: b"checking earl ... _char:   l23 = UNIQUE | NON_NULL, (empty)
        // 1354: b"checking earl ... st u8: typeof(_17) = *const {l25} u8
        // 1354: b"checking earl ... st u8:   l25 = UNIQUE | NON_NULL, (empty)
        // 1354: b"checking earl ... on\0": typeof(_18) = *const {l27} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 1354: b"checking earl ... on\0":   l27 = UNIQUE | NON_NULL, (empty)
        // 1354: b"checking earl ... on\0": typeof(_19) = & {l29} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 1354: b"checking earl ... on\0":   l29 = UNIQUE | NON_NULL, FIXED
        // 1354: b"checking earl ... on\0": typeof(_18 = &raw const (*_19)) = *const {l106} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 1354: b"checking earl ... on\0":   l106 = UNIQUE | NON_NULL, (empty)
        // 1354: b"checking earl ... on\0": typeof(_19 = const b"checking early termination\x00") = & {l105} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 1354: b"checking earl ... on\0":   l105 = UNIQUE | NON_NULL, (empty)
        // 1354: b"checking earl ... _char: typeof(_16 = move _17 as *const i8 (Misc)) = *const {l108} i8
        // 1354: b"checking earl ... _char:   l108 = UNIQUE | NON_NULL, (empty)
        // 1354: b"checking earl ... st u8: typeof(_17 = move _18 as *const u8 (Pointer(ArrayToPointer))) = *const {l107} u8
        // 1354: b"checking earl ... st u8:   l107 = UNIQUE | NON_NULL, (empty)
    );
    if pthread_mutex_lock(&mut donemutex) != 0 {
    // 1356: &mut donemutex: typeof(_23) = *mut {l34} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1356: &mut donemutex:   l34 = UNIQUE | NON_NULL, (empty)
    // 1356: &mut donemutex: typeof(_24) = &mut {l36} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1356: &mut donemutex:   l36 = UNIQUE | NON_NULL, (empty)
    // 1356: donemutex: typeof(_25) = *mut {l38} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1356: donemutex:   l38 = UNIQUE | NON_NULL, (empty)
    // 1356: &mut donemutex: typeof(_24 = &mut (*_25)) = &mut {l109} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1356: &mut donemutex:   l109 = UNIQUE | NON_NULL, (empty)
    // 1356: &mut donemutex: typeof(_23 = &raw mut (*_24)) = *mut {l110} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1356: &mut donemutex:   l110 = UNIQUE | NON_NULL, (empty)
        warn(
            b"failed to lock 'done' mutex in termination check\0" as *const u8
            // 1358: b"failed to loc ... _char: typeof(_27) = *const {l41} i8
            // 1358: b"failed to loc ... _char:   l41 = UNIQUE | NON_NULL, (empty)
            // 1358: b"failed to loc ... st u8: typeof(_28) = *const {l43} u8
            // 1358: b"failed to loc ... st u8:   l43 = UNIQUE | NON_NULL, (empty)
            // 1358: b"failed to loc ... ck\0": typeof(_29) = *const {l45} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
            // 1358: b"failed to loc ... ck\0":   l45 = UNIQUE | NON_NULL, (empty)
            // 1358: b"failed to loc ... ck\0": typeof(_30) = & {l47} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
            // 1358: b"failed to loc ... ck\0":   l47 = UNIQUE | NON_NULL, FIXED
            // 1358: b"failed to loc ... _char: typeof(_27 = move _28 as *const i8 (Misc)) = *const {l114} i8
            // 1358: b"failed to loc ... _char:   l114 = UNIQUE | NON_NULL, (empty)
            // 1358: b"failed to loc ... ck\0": typeof(_30 = const b"failed to lock \'done\' mutex in termination check\x00") = & {l111} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
            // 1358: b"failed to loc ... ck\0":   l111 = UNIQUE | NON_NULL, (empty)
            // 1358: b"failed to loc ... ck\0": typeof(_29 = &raw const (*_30)) = *const {l112} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
            // 1358: b"failed to loc ... ck\0":   l112 = UNIQUE | NON_NULL, (empty)
            // 1358: b"failed to loc ... st u8: typeof(_28 = move _29 as *const u8 (Pointer(ArrayToPointer))) = *const {l113} u8
            // 1358: b"failed to loc ... st u8:   l113 = UNIQUE | NON_NULL, (empty)
                as *const libc::c_char,
        );
    }
    res = done;
    // 1362: done: typeof(_32) = *mut {l50} i32
    // 1362: done:   l50 = UNIQUE | NON_NULL, (empty)
    termchks += 1;
    // 1363: termchks: typeof(_33) = *mut {l52} i32
    // 1363: termchks:   l52 = UNIQUE | NON_NULL, (empty)
    termchks;
    // 1364: termchks: typeof(_36) = *mut {l56} i32
    // 1364: termchks:   l56 = UNIQUE | NON_NULL, (empty)
    if pthread_mutex_unlock(&mut donemutex) != 0 {
    // 1365: &mut donemutex: typeof(_40) = *mut {l61} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1365: &mut donemutex:   l61 = UNIQUE | NON_NULL, (empty)
    // 1365: &mut donemutex: typeof(_41) = &mut {l63} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1365: &mut donemutex:   l63 = UNIQUE | NON_NULL, (empty)
    // 1365: donemutex: typeof(_42) = *mut {l65} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1365: donemutex:   l65 = UNIQUE | NON_NULL, (empty)
    // 1365: &mut donemutex: typeof(_40 = &raw mut (*_41)) = *mut {l116} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1365: &mut donemutex:   l116 = UNIQUE | NON_NULL, (empty)
    // 1365: &mut donemutex: typeof(_41 = &mut (*_42)) = &mut {l115} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1365: &mut donemutex:   l115 = UNIQUE | NON_NULL, (empty)
        warn(
            b"failed to unlock 'done' mutex in termination check\0" as *const u8
            // 1367: b"failed to unl ... _char: typeof(_44) = *const {l68} i8
            // 1367: b"failed to unl ... _char:   l68 = UNIQUE | NON_NULL, (empty)
            // 1367: b"failed to unl ... st u8: typeof(_45) = *const {l70} u8
            // 1367: b"failed to unl ... st u8:   l70 = UNIQUE | NON_NULL, (empty)
            // 1367: b"failed to unl ... ck\0": typeof(_46) = *const {l72} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 1367: b"failed to unl ... ck\0":   l72 = UNIQUE | NON_NULL, (empty)
            // 1367: b"failed to unl ... ck\0": typeof(_47) = & {l74} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 1367: b"failed to unl ... ck\0":   l74 = UNIQUE | NON_NULL, FIXED
            // 1367: b"failed to unl ... st u8: typeof(_45 = move _46 as *const u8 (Pointer(ArrayToPointer))) = *const {l119} u8
            // 1367: b"failed to unl ... st u8:   l119 = UNIQUE | NON_NULL, (empty)
            // 1367: b"failed to unl ... ck\0": typeof(_46 = &raw const (*_47)) = *const {l118} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 1367: b"failed to unl ... ck\0":   l118 = UNIQUE | NON_NULL, (empty)
            // 1367: b"failed to unl ... _char: typeof(_44 = move _45 as *const i8 (Misc)) = *const {l120} i8
            // 1367: b"failed to unl ... _char:   l120 = UNIQUE | NON_NULL, (empty)
            // 1367: b"failed to unl ... ck\0": typeof(_47 = const b"failed to unlock \'done\' mutex in termination check\x00") = & {l117} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 1367: b"failed to unl ... ck\0":   l117 = UNIQUE | NON_NULL, (empty)
                as *const libc::c_char,
        );
    }
    msg(
        wid,
        3 as libc::c_int,
        b"early termination check %s\0" as *const u8 as *const libc::c_char,
        // 1374: b"early termina ... _char: typeof(_51) = *const {l79} i8
        // 1374: b"early termina ... _char:   l79 = UNIQUE | NON_NULL, (empty)
        // 1374: b"early termina ... st u8: typeof(_52) = *const {l81} u8
        // 1374: b"early termina ... st u8:   l81 = UNIQUE | NON_NULL, (empty)
        // 1374: b"early termina ... %s\0": typeof(_53) = *const {l83} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 1374: b"early termina ... %s\0":   l83 = UNIQUE | NON_NULL, (empty)
        // 1374: b"early termina ... %s\0": typeof(_54) = & {l85} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 1374: b"early termina ... %s\0":   l85 = UNIQUE | NON_NULL, FIXED
        // 1374: b"early termina ... %s\0": typeof(_54 = const b"early termination check %s\x00") = & {l121} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 1374: b"early termina ... %s\0":   l121 = UNIQUE | NON_NULL, (empty)
        // 1374: b"early termina ... st u8: typeof(_52 = move _53 as *const u8 (Pointer(ArrayToPointer))) = *const {l123} u8
        // 1374: b"early termina ... st u8:   l123 = UNIQUE | NON_NULL, (empty)
        // 1374: b"early termina ... _char: typeof(_51 = move _52 as *const i8 (Misc)) = *const {l124} i8
        // 1374: b"early termina ... _char:   l124 = UNIQUE | NON_NULL, (empty)
        // 1374: b"early termina ... %s\0": typeof(_53 = &raw const (*_54)) = *const {l122} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 1374: b"early termina ... %s\0":   l122 = UNIQUE | NON_NULL, (empty)
        if res != 0 {
        // 1375: if res != 0 { b ... har }: typeof(_55) = *const {l87} i8
        // 1375: if res != 0 { b ... har }:   l87 = UNIQUE | NON_NULL, (empty)
            b"succeeded\0" as *const u8 as *const libc::c_char
            // 1376: b"succeeded\0"  ... st u8: typeof(_58) = *const {l91} u8
            // 1376: b"succeeded\0"  ... st u8:   l91 = UNIQUE | NON_NULL, (empty)
            // 1376: b"succeeded\0": typeof(_59) = *const {l93} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 1376: b"succeeded\0":   l93 = UNIQUE | NON_NULL, (empty)
            // 1376: b"succeeded\0": typeof(_60) = & {l95} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 1376: b"succeeded\0":   l95 = UNIQUE | NON_NULL, FIXED
            // 1376: b"succeeded\0": typeof(_59 = &raw const (*_60)) = *const {l126} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 1376: b"succeeded\0":   l126 = UNIQUE | NON_NULL, (empty)
            // 1376: b"succeeded\0"  ... st u8: typeof(_58 = move _59 as *const u8 (Pointer(ArrayToPointer))) = *const {l127} u8
            // 1376: b"succeeded\0"  ... st u8:   l127 = UNIQUE | NON_NULL, (empty)
            // 1376: b"succeeded\0": typeof(_60 = const b"succeeded\x00") = & {l125} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 1376: b"succeeded\0":   l125 = UNIQUE | NON_NULL, (empty)
            // 1376: b"succeeded\0"  ... _char: typeof(_55 = move _58 as *const i8 (Misc)) = *const {l128} i8
            // 1376: b"succeeded\0"  ... _char:   l128 = UNIQUE | NON_NULL, (empty)
        } else {
            b"failed\0" as *const u8 as *const libc::c_char
            // 1378: b"failed\0" as  ... st u8: typeof(_61) = *const {l97} u8
            // 1378: b"failed\0" as  ... st u8:   l97 = UNIQUE | NON_NULL, (empty)
            // 1378: b"failed\0": typeof(_62) = *const {l99} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 1378: b"failed\0":   l99 = UNIQUE | NON_NULL, (empty)
            // 1378: b"failed\0": typeof(_63) = & {l101} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 1378: b"failed\0":   l101 = UNIQUE | NON_NULL, FIXED
            // 1378: b"failed\0": typeof(_63 = const b"failed\x00") = & {l129} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 1378: b"failed\0":   l129 = UNIQUE | NON_NULL, (empty)
            // 1378: b"failed\0" as  ... st u8: typeof(_61 = move _62 as *const u8 (Pointer(ArrayToPointer))) = *const {l131} u8
            // 1378: b"failed\0" as  ... st u8:   l131 = UNIQUE | NON_NULL, (empty)
            // 1378: b"failed\0" as  ... _char: typeof(_55 = move _61 as *const i8 (Misc)) = *const {l132} i8
            // 1378: b"failed\0" as  ... _char:   l132 = UNIQUE | NON_NULL, (empty)
            // 1378: b"failed\0": typeof(_62 = &raw const (*_63)) = *const {l130} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 1378: b"failed\0":   l130 = UNIQUE | NON_NULL, (empty)
        },
    );
    return res;
}
unsafe extern "C" fn flush(mut worker: *mut Worker, mut keep_locked: libc::c_int) {
// 1383: mut worker: typeof(_1) = *mut {g15} DefId(0:535 ~ plingeling[18f5]::Worker)
// 1383: mut worker:   g15 = UNIQUE | NON_NULL, FIXED
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    // 1384: worker: typeof(_6) = *mut {l6} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1384: worker:   l6 = UNIQUE | NON_NULL, (empty)
    // 1384: workers: typeof(_7) = *const {l8} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1384: workers:   l8 = UNIQUE | NON_NULL, (empty)
    // 1384: workers: typeof(_8) = *mut {l10} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1384: workers:   l10 = UNIQUE | NON_NULL, (empty)
    // 1384: workers: typeof(_9) = *mut {l12} *mut {l13} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1384: workers:   l12 = UNIQUE | NON_NULL, (empty)
    // 1384: workers:   l13 = UNIQUE | NON_NULL, (empty)
    // 1384: workers: typeof(_7 = move _8 as *const Worker (Pointer(MutToConstPointer))) = *const {l220} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1384: workers:   l220 = UNIQUE | NON_NULL, (empty)
    let mut lit: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    msg(
        wid,
        2 as libc::c_int,
        b"flushing %d units\0" as *const u8 as *const libc::c_char,
        // 1393: b"flushing %d u ... _char: typeof(_18) = *const {l23} i8
        // 1393: b"flushing %d u ... _char:   l23 = UNIQUE | NON_NULL, (empty)
        // 1393: b"flushing %d u ... st u8: typeof(_19) = *const {l25} u8
        // 1393: b"flushing %d u ... st u8:   l25 = UNIQUE | NON_NULL, (empty)
        // 1393: b"flushing %d u ... ts\0": typeof(_20) = *const {l27} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
        // 1393: b"flushing %d u ... ts\0":   l27 = UNIQUE | NON_NULL, (empty)
        // 1393: b"flushing %d u ... ts\0": typeof(_21) = & {l29} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
        // 1393: b"flushing %d u ... ts\0":   l29 = UNIQUE | NON_NULL, FIXED
        // 1393: b"flushing %d u ... st u8: typeof(_19 = move _20 as *const u8 (Pointer(ArrayToPointer))) = *const {l223} u8
        // 1393: b"flushing %d u ... st u8:   l223 = UNIQUE | NON_NULL, (empty)
        // 1393: b"flushing %d u ... _char: typeof(_18 = move _19 as *const i8 (Misc)) = *const {l224} i8
        // 1393: b"flushing %d u ... _char:   l224 = UNIQUE | NON_NULL, (empty)
        // 1393: b"flushing %d u ... ts\0": typeof(_21 = const b"flushing %d units\x00") = & {l221} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
        // 1393: b"flushing %d u ... ts\0":   l221 = UNIQUE | NON_NULL, (empty)
        // 1393: b"flushing %d u ... ts\0": typeof(_20 = &raw const (*_21)) = *const {l222} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
        // 1393: b"flushing %d u ... ts\0":   l222 = UNIQUE | NON_NULL, (empty)
        (*worker).nunits,
    );
    if pthread_mutex_lock(&mut fixedmutex) != 0 {
    // 1396: &mut fixedmutex: typeof(_26) = *mut {l35} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1396: &mut fixedmutex:   l35 = UNIQUE | NON_NULL, (empty)
    // 1396: &mut fixedmutex: typeof(_27) = &mut {l37} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1396: &mut fixedmutex:   l37 = UNIQUE | NON_NULL, (empty)
    // 1396: fixedmutex: typeof(_28) = *mut {l39} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1396: fixedmutex:   l39 = UNIQUE | NON_NULL, (empty)
    // 1396: &mut fixedmutex: typeof(_26 = &raw mut (*_27)) = *mut {l226} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1396: &mut fixedmutex:   l226 = UNIQUE | NON_NULL, (empty)
    // 1396: &mut fixedmutex: typeof(_27 = &mut (*_28)) = &mut {l225} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1396: &mut fixedmutex:   l225 = UNIQUE | NON_NULL, (empty)
        warn(b"failed to lock 'fixed' mutex in flush\0" as *const u8 as *const libc::c_char);
        // 1397: b"failed to loc ... _char: typeof(_30) = *const {l42} i8
        // 1397: b"failed to loc ... _char:   l42 = UNIQUE | NON_NULL, (empty)
        // 1397: b"failed to loc ... st u8: typeof(_31) = *const {l44} u8
        // 1397: b"failed to loc ... st u8:   l44 = UNIQUE | NON_NULL, (empty)
        // 1397: b"failed to loc ... sh\0": typeof(_32) = *const {l46} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
        // 1397: b"failed to loc ... sh\0":   l46 = UNIQUE | NON_NULL, (empty)
        // 1397: b"failed to loc ... sh\0": typeof(_33) = & {l48} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
        // 1397: b"failed to loc ... sh\0":   l48 = UNIQUE | NON_NULL, FIXED
        // 1397: b"failed to loc ... sh\0": typeof(_32 = &raw const (*_33)) = *const {l228} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
        // 1397: b"failed to loc ... sh\0":   l228 = UNIQUE | NON_NULL, (empty)
        // 1397: b"failed to loc ... st u8: typeof(_31 = move _32 as *const u8 (Pointer(ArrayToPointer))) = *const {l229} u8
        // 1397: b"failed to loc ... st u8:   l229 = UNIQUE | NON_NULL, (empty)
        // 1397: b"failed to loc ... sh\0": typeof(_33 = const b"failed to lock \'fixed\' mutex in flush\x00") = & {l227} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
        // 1397: b"failed to loc ... sh\0":   l227 = UNIQUE | NON_NULL, (empty)
        // 1397: b"failed to loc ... _char: typeof(_30 = move _31 as *const i8 (Misc)) = *const {l230} i8
        // 1397: b"failed to loc ... _char:   l230 = UNIQUE | NON_NULL, (empty)
    }
    flushed += 1;
    // 1399: flushed: typeof(_34) = *mut {l50} i32
    // 1399: flushed:   l50 = UNIQUE | NON_NULL, (empty)
    flushed;
    // 1400: flushed: typeof(_37) = *mut {l54} i32
    // 1400: flushed:   l54 = UNIQUE | NON_NULL, (empty)
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
        // 1412: vals.offset(idx ... size): typeof(_60) = *mut {l78} i32
        // 1412: vals.offset(idx ... size):   l78 = UNIQUE | NON_NULL, (empty)
        // 1412: vals: typeof(_61) = *mut {l80} i32
        // 1412: vals:   l80 = UNIQUE | NON_NULL, (empty)
        // 1412: vals: typeof(_62) = *mut {l82} *mut {l83} i32
        // 1412: vals:   l82 = UNIQUE | NON_NULL, (empty)
        // 1412: vals:   l83 = UNIQUE | NON_NULL, (empty)
        if tmp == 0 {
            let fresh3 = nfixed;
            // 1414: nfixed: typeof(_69) = *mut {l91} i32
            // 1414: nfixed:   l91 = UNIQUE | NON_NULL, (empty)
            nfixed = nfixed + 1;
            // 1415: nfixed: typeof(_71) = *mut {l94} i32
            // 1415: nfixed:   l94 = UNIQUE | NON_NULL, (empty)
            // 1415: nfixed: typeof(_73) = *mut {l97} i32
            // 1415: nfixed:   l97 = UNIQUE | NON_NULL, (empty)
            *fixed.offset(fresh3 as isize) = lit;
            // 1416: fixed.offset(fr ... size): typeof(_75) = *mut {l100} i32
            // 1416: fixed.offset(fr ... size):   l100 = UNIQUE | NON_NULL, (empty)
            // 1416: fixed: typeof(_76) = *mut {l102} i32
            // 1416: fixed:   l102 = UNIQUE | NON_NULL, (empty)
            // 1416: fixed: typeof(_77) = *mut {l104} *mut {l105} i32
            // 1416: fixed:   l104 = UNIQUE | NON_NULL, (empty)
            // 1416: fixed:   l105 = UNIQUE | NON_NULL, (empty)
            *vals.offset(idx as isize) = val;
            // 1417: vals.offset(idx ... size): typeof(_81) = *mut {l110} i32
            // 1417: vals.offset(idx ... size):   l110 = UNIQUE | NON_NULL, (empty)
            // 1417: vals: typeof(_82) = *mut {l112} i32
            // 1417: vals:   l112 = UNIQUE | NON_NULL, (empty)
            // 1417: vals: typeof(_83) = *mut {l114} *mut {l115} i32
            // 1417: vals:   l114 = UNIQUE | NON_NULL, (empty)
            // 1417: vals:   l115 = UNIQUE | NON_NULL, (empty)
            (*worker).stats.units.produced += 1;
            (*worker).stats.units.produced;
            (*worker).stats.produced += 1;
            (*worker).stats.produced;
            units += 1;
            // 1422: units: typeof(_90) = *mut {l123} i32
            // 1422: units:   l123 = UNIQUE | NON_NULL, (empty)
            units;
            // 1423: units: typeof(_93) = *mut {l127} i32
            // 1423: units:   l127 = UNIQUE | NON_NULL, (empty)
        } else if tmp == -val {
            if pthread_mutex_lock(&mut donemutex) != 0 {
            // 1425: &mut donemutex: typeof(_103) = *mut {l138} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
            // 1425: &mut donemutex:   l138 = UNIQUE | NON_NULL, (empty)
            // 1425: &mut donemutex: typeof(_104) = &mut {l140} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
            // 1425: &mut donemutex:   l140 = UNIQUE | NON_NULL, (empty)
            // 1425: donemutex: typeof(_105) = *mut {l142} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
            // 1425: donemutex:   l142 = UNIQUE | NON_NULL, (empty)
            // 1425: &mut donemutex: typeof(_103 = &raw mut (*_104)) = *mut {l232} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
            // 1425: &mut donemutex:   l232 = UNIQUE | NON_NULL, (empty)
            // 1425: &mut donemutex: typeof(_104 = &mut (*_105)) = &mut {l231} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
            // 1425: &mut donemutex:   l231 = UNIQUE | NON_NULL, (empty)
                warn(
                    b"failed to lock 'done' mutex flushing unit\0" as *const u8
                    // 1427: b"failed to loc ... _char: typeof(_107) = *const {l145} i8
                    // 1427: b"failed to loc ... _char:   l145 = UNIQUE | NON_NULL, (empty)
                    // 1427: b"failed to loc ... st u8: typeof(_108) = *const {l147} u8
                    // 1427: b"failed to loc ... st u8:   l147 = UNIQUE | NON_NULL, (empty)
                    // 1427: b"failed to loc ... it\0": typeof(_109) = *const {l149} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                    // 1427: b"failed to loc ... it\0":   l149 = UNIQUE | NON_NULL, (empty)
                    // 1427: b"failed to loc ... it\0": typeof(_110) = & {l151} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                    // 1427: b"failed to loc ... it\0":   l151 = UNIQUE | NON_NULL, FIXED
                    // 1427: b"failed to loc ... st u8: typeof(_108 = move _109 as *const u8 (Pointer(ArrayToPointer))) = *const {l235} u8
                    // 1427: b"failed to loc ... st u8:   l235 = UNIQUE | NON_NULL, (empty)
                    // 1427: b"failed to loc ... it\0": typeof(_110 = const b"failed to lock \'done\' mutex flushing unit\x00") = & {l233} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                    // 1427: b"failed to loc ... it\0":   l233 = UNIQUE | NON_NULL, (empty)
                    // 1427: b"failed to loc ... it\0": typeof(_109 = &raw const (*_110)) = *const {l234} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                    // 1427: b"failed to loc ... it\0":   l234 = UNIQUE | NON_NULL, (empty)
                    // 1427: b"failed to loc ... _char: typeof(_107 = move _108 as *const i8 (Misc)) = *const {l236} i8
                    // 1427: b"failed to loc ... _char:   l236 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                );
            }
            if globalres == 0 {
            // 1431: globalres: typeof(_114) = *mut {l156} i32
            // 1431: globalres:   l156 = UNIQUE | NON_NULL, (empty)
                msg(
                    wid,
                    1 as libc::c_int,
                    b"mismatched unit\0" as *const u8 as *const libc::c_char,
                    // 1435: b"mismatched un ... _char: typeof(_118) = *const {l161} i8
                    // 1435: b"mismatched un ... _char:   l161 = UNIQUE | NON_NULL, (empty)
                    // 1435: b"mismatched un ... st u8: typeof(_119) = *const {l163} u8
                    // 1435: b"mismatched un ... st u8:   l163 = UNIQUE | NON_NULL, (empty)
                    // 1435: b"mismatched unit\0": typeof(_120) = *const {l165} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
                    // 1435: b"mismatched unit\0":   l165 = UNIQUE | NON_NULL, (empty)
                    // 1435: b"mismatched unit\0": typeof(_121) = & {l167} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
                    // 1435: b"mismatched unit\0":   l167 = UNIQUE | NON_NULL, FIXED
                    // 1435: b"mismatched unit\0": typeof(_120 = &raw const (*_121)) = *const {l238} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
                    // 1435: b"mismatched unit\0":   l238 = UNIQUE | NON_NULL, (empty)
                    // 1435: b"mismatched un ... _char: typeof(_118 = move _119 as *const i8 (Misc)) = *const {l240} i8
                    // 1435: b"mismatched un ... _char:   l240 = UNIQUE | NON_NULL, (empty)
                    // 1435: b"mismatched un ... st u8: typeof(_119 = move _120 as *const u8 (Pointer(ArrayToPointer))) = *const {l239} u8
                    // 1435: b"mismatched un ... st u8:   l239 = UNIQUE | NON_NULL, (empty)
                    // 1435: b"mismatched unit\0": typeof(_121 = const b"mismatched unit\x00") = & {l237} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
                    // 1435: b"mismatched unit\0":   l237 = UNIQUE | NON_NULL, (empty)
                );
            }
            globalres = 20 as libc::c_int;
            // 1438: globalres: typeof(_123) = *mut {l170} i32
            // 1438: globalres:   l170 = UNIQUE | NON_NULL, (empty)
            done = 1 as libc::c_int;
            // 1439: done: typeof(_125) = *mut {l173} i32
            // 1439: done:   l173 = UNIQUE | NON_NULL, (empty)
            if pthread_mutex_unlock(&mut donemutex) != 0 {
            // 1440: &mut donemutex: typeof(_129) = *mut {l178} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
            // 1440: &mut donemutex:   l178 = UNIQUE | NON_NULL, (empty)
            // 1440: &mut donemutex: typeof(_130) = &mut {l180} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
            // 1440: &mut donemutex:   l180 = UNIQUE | NON_NULL, (empty)
            // 1440: donemutex: typeof(_131) = *mut {l182} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
            // 1440: donemutex:   l182 = UNIQUE | NON_NULL, (empty)
            // 1440: &mut donemutex: typeof(_129 = &raw mut (*_130)) = *mut {l242} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
            // 1440: &mut donemutex:   l242 = UNIQUE | NON_NULL, (empty)
            // 1440: &mut donemutex: typeof(_130 = &mut (*_131)) = &mut {l241} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
            // 1440: &mut donemutex:   l241 = UNIQUE | NON_NULL, (empty)
                warn(
                    b"failed to unlock 'done' mutex flushing unit\0" as *const u8
                    // 1442: b"failed to unl ... _char: typeof(_133) = *const {l185} i8
                    // 1442: b"failed to unl ... _char:   l185 = UNIQUE | NON_NULL, (empty)
                    // 1442: b"failed to unl ... st u8: typeof(_134) = *const {l187} u8
                    // 1442: b"failed to unl ... st u8:   l187 = UNIQUE | NON_NULL, (empty)
                    // 1442: b"failed to unl ... it\0": typeof(_135) = *const {l189} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                    // 1442: b"failed to unl ... it\0":   l189 = UNIQUE | NON_NULL, (empty)
                    // 1442: b"failed to unl ... it\0": typeof(_136) = & {l191} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                    // 1442: b"failed to unl ... it\0":   l191 = UNIQUE | NON_NULL, FIXED
                    // 1442: b"failed to unl ... _char: typeof(_133 = move _134 as *const i8 (Misc)) = *const {l246} i8
                    // 1442: b"failed to unl ... _char:   l246 = UNIQUE | NON_NULL, (empty)
                    // 1442: b"failed to unl ... st u8: typeof(_134 = move _135 as *const u8 (Pointer(ArrayToPointer))) = *const {l245} u8
                    // 1442: b"failed to unl ... st u8:   l245 = UNIQUE | NON_NULL, (empty)
                    // 1442: b"failed to unl ... it\0": typeof(_135 = &raw const (*_136)) = *const {l244} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                    // 1442: b"failed to unl ... it\0":   l244 = UNIQUE | NON_NULL, (empty)
                    // 1442: b"failed to unl ... it\0": typeof(_136 = const b"failed to unlock \'done\' mutex flushing unit\x00") = & {l243} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                    // 1442: b"failed to unl ... it\0":   l243 = UNIQUE | NON_NULL, (empty)
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
    // 1455: &mut fixedmutex: typeof(_149) = *mut {l205} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1455: &mut fixedmutex:   l205 = UNIQUE | NON_NULL, (empty)
    // 1455: &mut fixedmutex: typeof(_150) = &mut {l207} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1455: &mut fixedmutex:   l207 = UNIQUE | NON_NULL, (empty)
    // 1455: fixedmutex: typeof(_151) = *mut {l209} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1455: fixedmutex:   l209 = UNIQUE | NON_NULL, (empty)
    // 1455: &mut fixedmutex: typeof(_150 = &mut (*_151)) = &mut {l247} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1455: &mut fixedmutex:   l247 = UNIQUE | NON_NULL, (empty)
    // 1455: &mut fixedmutex: typeof(_149 = &raw mut (*_150)) = *mut {l248} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1455: &mut fixedmutex:   l248 = UNIQUE | NON_NULL, (empty)
        warn(b"failed to unlock 'fixed' mutex in flush\0" as *const u8 as *const libc::c_char);
        // 1456: b"failed to unl ... _char: typeof(_153) = *const {l212} i8
        // 1456: b"failed to unl ... _char:   l212 = UNIQUE | NON_NULL, (empty)
        // 1456: b"failed to unl ... st u8: typeof(_154) = *const {l214} u8
        // 1456: b"failed to unl ... st u8:   l214 = UNIQUE | NON_NULL, (empty)
        // 1456: b"failed to unl ... sh\0": typeof(_155) = *const {l216} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 1456: b"failed to unl ... sh\0":   l216 = UNIQUE | NON_NULL, (empty)
        // 1456: b"failed to unl ... sh\0": typeof(_156) = & {l218} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 1456: b"failed to unl ... sh\0":   l218 = UNIQUE | NON_NULL, FIXED
        // 1456: b"failed to unl ... _char: typeof(_153 = move _154 as *const i8 (Misc)) = *const {l252} i8
        // 1456: b"failed to unl ... _char:   l252 = UNIQUE | NON_NULL, (empty)
        // 1456: b"failed to unl ... st u8: typeof(_154 = move _155 as *const u8 (Pointer(ArrayToPointer))) = *const {l251} u8
        // 1456: b"failed to unl ... st u8:   l251 = UNIQUE | NON_NULL, (empty)
        // 1456: b"failed to unl ... sh\0": typeof(_155 = &raw const (*_156)) = *const {l250} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 1456: b"failed to unl ... sh\0":   l250 = UNIQUE | NON_NULL, (empty)
        // 1456: b"failed to unl ... sh\0": typeof(_156 = const b"failed to unlock \'fixed\' mutex in flush\x00") = & {l249} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 1456: b"failed to unl ... sh\0":   l249 = UNIQUE | NON_NULL, (empty)
    }
}
unsafe extern "C" fn produceunit(mut voidptr: *mut libc::c_void, mut lit: libc::c_int) {
// 1459: mut voidptr: typeof(_1) = *mut {g16} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 1459: mut voidptr:   g16 = UNIQUE | NON_NULL, FIXED
    let mut worker: *mut Worker = voidptr as *mut Worker;
    // 1460: mut worker: typeof(_3) = *mut {l3} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1460: mut worker:   l3 = UNIQUE | NON_NULL, (empty)
    // 1460: voidptr: typeof(_4) = *mut {l5} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1460: voidptr:   l5 = UNIQUE | NON_NULL, (empty)
    // 1460: voidptr as *mut ... orker: typeof(_3 = move _4 as *mut Worker (Misc)) = *mut {l49} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1460: voidptr as *mut ... orker:   l49 = UNIQUE | NON_NULL, (empty)
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    // 1461: worker: typeof(_8) = *mut {l10} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1461: worker:   l10 = UNIQUE | NON_NULL, (empty)
    // 1461: workers: typeof(_9) = *const {l12} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1461: workers:   l12 = UNIQUE | NON_NULL, (empty)
    // 1461: workers: typeof(_10) = *mut {l14} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1461: workers:   l14 = UNIQUE | NON_NULL, (empty)
    // 1461: workers: typeof(_11) = *mut {l16} *mut {l17} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1461: workers:   l16 = UNIQUE | NON_NULL, (empty)
    // 1461: workers:   l17 = UNIQUE | NON_NULL, (empty)
    // 1461: workers: typeof(_9 = move _10 as *const Worker (Pointer(MutToConstPointer))) = *const {l50} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1461: workers:   l50 = UNIQUE | NON_NULL, (empty)
    let fresh4 = (*worker).nunits;
    (*worker).nunits = (*worker).nunits + 1;
    (*worker).units[fresh4 as usize] = lit;
    msg(
        wid,
        3 as libc::c_int,
        b"producing unit %d\0" as *const u8 as *const libc::c_char,
        // 1468: b"producing uni ... _char: typeof(_23) = *const {l30} i8
        // 1468: b"producing uni ... _char:   l30 = UNIQUE | NON_NULL, (empty)
        // 1468: b"producing uni ... st u8: typeof(_24) = *const {l32} u8
        // 1468: b"producing uni ... st u8:   l32 = UNIQUE | NON_NULL, (empty)
        // 1468: b"producing uni ... %d\0": typeof(_25) = *const {l34} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
        // 1468: b"producing uni ... %d\0":   l34 = UNIQUE | NON_NULL, (empty)
        // 1468: b"producing uni ... %d\0": typeof(_26) = & {l36} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
        // 1468: b"producing uni ... %d\0":   l36 = UNIQUE | NON_NULL, FIXED
        // 1468: b"producing uni ... %d\0": typeof(_25 = &raw const (*_26)) = *const {l52} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
        // 1468: b"producing uni ... %d\0":   l52 = UNIQUE | NON_NULL, (empty)
        // 1468: b"producing uni ... _char: typeof(_23 = move _24 as *const i8 (Misc)) = *const {l54} i8
        // 1468: b"producing uni ... _char:   l54 = UNIQUE | NON_NULL, (empty)
        // 1468: b"producing uni ... st u8: typeof(_24 = move _25 as *const u8 (Pointer(ArrayToPointer))) = *const {l53} u8
        // 1468: b"producing uni ... st u8:   l53 = UNIQUE | NON_NULL, (empty)
        // 1468: b"producing uni ... %d\0": typeof(_26 = const b"producing unit %d\x00") = & {l51} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
        // 1468: b"producing uni ... %d\0":   l51 = UNIQUE | NON_NULL, (empty)
        lit,
    );
    if (*worker).nunits == (1 as libc::c_int) << 9 as libc::c_int {
        flush(worker, 0 as libc::c_int);
        // 1472: worker: typeof(_35) = *mut {l46} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 1472: worker:   l46 = UNIQUE | NON_NULL, (empty)
    }
}
unsafe extern "C" fn consumeunits(
    mut voidptr: *mut libc::c_void,
    // 1476: mut voidptr: typeof(_1) = *mut {g17} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1476: mut voidptr:   g17 = UNIQUE | NON_NULL, FIXED
    mut fromptr: *mut *mut libc::c_int,
    // 1477: mut fromptr: typeof(_2) = *mut {g18} *mut {g19} i32
    // 1477: mut fromptr:   g18 = UNIQUE | NON_NULL, FIXED
    // 1477: mut fromptr:   g19 = UNIQUE | NON_NULL, FIXED
    mut toptr: *mut *mut libc::c_int,
    // 1478: mut toptr: typeof(_3) = *mut {g20} *mut {g21} i32
    // 1478: mut toptr:   g20 = UNIQUE | NON_NULL, FIXED
    // 1478: mut toptr:   g21 = UNIQUE | NON_NULL, FIXED
) {
    let mut worker: *mut Worker = voidptr as *mut Worker;
    // 1480: mut worker: typeof(_4) = *mut {l4} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1480: mut worker:   l4 = UNIQUE | NON_NULL, (empty)
    // 1480: voidptr: typeof(_5) = *mut {l6} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1480: voidptr:   l6 = UNIQUE | NON_NULL, (empty)
    // 1480: voidptr as *mut ... orker: typeof(_4 = move _5 as *mut Worker (Misc)) = *mut {l98} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1480: voidptr as *mut ... orker:   l98 = UNIQUE | NON_NULL, (empty)
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    // 1481: worker: typeof(_9) = *mut {l11} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1481: worker:   l11 = UNIQUE | NON_NULL, (empty)
    // 1481: workers: typeof(_10) = *const {l13} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1481: workers:   l13 = UNIQUE | NON_NULL, (empty)
    // 1481: workers: typeof(_11) = *mut {l15} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1481: workers:   l15 = UNIQUE | NON_NULL, (empty)
    // 1481: workers: typeof(_12) = *mut {l17} *mut {l18} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1481: workers:   l17 = UNIQUE | NON_NULL, (empty)
    // 1481: workers:   l18 = UNIQUE | NON_NULL, (empty)
    // 1481: workers: typeof(_10 = move _11 as *const Worker (Pointer(MutToConstPointer))) = *const {l99} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1481: workers:   l99 = UNIQUE | NON_NULL, (empty)
    if (*worker).nunits != 0 {
        flush(worker, 1 as libc::c_int);
        // 1483: worker: typeof(_17) = *mut {l24} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 1483: worker:   l24 = UNIQUE | NON_NULL, (empty)
    } else if pthread_mutex_lock(&mut fixedmutex) != 0 {
    // 1484: &mut fixedmutex: typeof(_21) = *mut {l29} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1484: &mut fixedmutex:   l29 = UNIQUE | NON_NULL, (empty)
    // 1484: &mut fixedmutex: typeof(_22) = &mut {l31} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1484: &mut fixedmutex:   l31 = UNIQUE | NON_NULL, (empty)
    // 1484: fixedmutex: typeof(_23) = *mut {l33} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1484: fixedmutex:   l33 = UNIQUE | NON_NULL, (empty)
    // 1484: &mut fixedmutex: typeof(_22 = &mut (*_23)) = &mut {l100} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1484: &mut fixedmutex:   l100 = UNIQUE | NON_NULL, (empty)
    // 1484: &mut fixedmutex: typeof(_21 = &raw mut (*_22)) = *mut {l101} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1484: &mut fixedmutex:   l101 = UNIQUE | NON_NULL, (empty)
        warn(b"failed to lock 'fixed' mutex in consume\0" as *const u8 as *const libc::c_char);
        // 1485: b"failed to loc ... _char: typeof(_25) = *const {l36} i8
        // 1485: b"failed to loc ... _char:   l36 = UNIQUE | NON_NULL, (empty)
        // 1485: b"failed to loc ... st u8: typeof(_26) = *const {l38} u8
        // 1485: b"failed to loc ... st u8:   l38 = UNIQUE | NON_NULL, (empty)
        // 1485: b"failed to loc ... me\0": typeof(_27) = *const {l40} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 1485: b"failed to loc ... me\0":   l40 = UNIQUE | NON_NULL, (empty)
        // 1485: b"failed to loc ... me\0": typeof(_28) = & {l42} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 1485: b"failed to loc ... me\0":   l42 = UNIQUE | NON_NULL, FIXED
        // 1485: b"failed to loc ... me\0": typeof(_27 = &raw const (*_28)) = *const {l103} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 1485: b"failed to loc ... me\0":   l103 = UNIQUE | NON_NULL, (empty)
        // 1485: b"failed to loc ... me\0": typeof(_28 = const b"failed to lock \'fixed\' mutex in consume\x00") = & {l102} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 1485: b"failed to loc ... me\0":   l102 = UNIQUE | NON_NULL, (empty)
        // 1485: b"failed to loc ... st u8: typeof(_26 = move _27 as *const u8 (Pointer(ArrayToPointer))) = *const {l104} u8
        // 1485: b"failed to loc ... st u8:   l104 = UNIQUE | NON_NULL, (empty)
        // 1485: b"failed to loc ... _char: typeof(_25 = move _26 as *const i8 (Misc)) = *const {l105} i8
        // 1485: b"failed to loc ... _char:   l105 = UNIQUE | NON_NULL, (empty)
    }
    msg(
        wid,
        3 as libc::c_int,
        b"starting unit synchronization\0" as *const u8 as *const libc::c_char,
        // 1490: b"starting unit ... _char: typeof(_32) = *const {l47} i8
        // 1490: b"starting unit ... _char:   l47 = UNIQUE | NON_NULL, (empty)
        // 1490: b"starting unit ... st u8: typeof(_33) = *const {l49} u8
        // 1490: b"starting unit ... st u8:   l49 = UNIQUE | NON_NULL, (empty)
        // 1490: b"starting unit ... on\0": typeof(_34) = *const {l51} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
        // 1490: b"starting unit ... on\0":   l51 = UNIQUE | NON_NULL, (empty)
        // 1490: b"starting unit ... on\0": typeof(_35) = & {l53} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
        // 1490: b"starting unit ... on\0":   l53 = UNIQUE | NON_NULL, FIXED
        // 1490: b"starting unit ... on\0": typeof(_35 = const b"starting unit synchronization\x00") = & {l106} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
        // 1490: b"starting unit ... on\0":   l106 = UNIQUE | NON_NULL, (empty)
        // 1490: b"starting unit ... _char: typeof(_32 = move _33 as *const i8 (Misc)) = *const {l109} i8
        // 1490: b"starting unit ... _char:   l109 = UNIQUE | NON_NULL, (empty)
        // 1490: b"starting unit ... st u8: typeof(_33 = move _34 as *const u8 (Pointer(ArrayToPointer))) = *const {l108} u8
        // 1490: b"starting unit ... st u8:   l108 = UNIQUE | NON_NULL, (empty)
        // 1490: b"starting unit ... on\0": typeof(_34 = &raw const (*_35)) = *const {l107} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
        // 1490: b"starting unit ... on\0":   l107 = UNIQUE | NON_NULL, (empty)
    );
    syncs.units += 1;
    // 1492: syncs: typeof(_36) = *mut {l55} DefId(0:590 ~ plingeling[18f5]::C2RustUnnamed_7)
    // 1492: syncs:   l55 = UNIQUE | NON_NULL, (empty)
    syncs.units;
    // 1493: syncs: typeof(_39) = *mut {l59} DefId(0:590 ~ plingeling[18f5]::C2RustUnnamed_7)
    // 1493: syncs:   l59 = UNIQUE | NON_NULL, (empty)
    *fromptr = fixed.offset((*worker).fixed as isize);
    // 1494: fixed.offset((* ... size): typeof(_40) = *mut {l61} i32
    // 1494: fixed.offset((* ... size):   l61 = UNIQUE | NON_NULL, (empty)
    // 1494: fixed: typeof(_41) = *mut {l63} i32
    // 1494: fixed:   l63 = UNIQUE | NON_NULL, (empty)
    // 1494: fixed: typeof(_42) = *mut {l65} *mut {l66} i32
    // 1494: fixed:   l65 = UNIQUE | NON_NULL, (empty)
    // 1494: fixed:   l66 = UNIQUE | NON_NULL, (empty)
    *toptr = fixed.offset(nfixed as isize);
    // 1495: fixed.offset(nf ... size): typeof(_45) = *mut {l70} i32
    // 1495: fixed.offset(nf ... size):   l70 = UNIQUE | NON_NULL, (empty)
    // 1495: fixed: typeof(_46) = *mut {l72} i32
    // 1495: fixed:   l72 = UNIQUE | NON_NULL, (empty)
    // 1495: fixed: typeof(_47) = *mut {l74} *mut {l75} i32
    // 1495: fixed:   l74 = UNIQUE | NON_NULL, (empty)
    // 1495: fixed:   l75 = UNIQUE | NON_NULL, (empty)
    // 1495: nfixed: typeof(_50) = *mut {l79} i32
    // 1495: nfixed:   l79 = UNIQUE | NON_NULL, (empty)
    if pthread_mutex_unlock(&mut fixedmutex) != 0 {
    // 1496: &mut fixedmutex: typeof(_53) = *mut {l83} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1496: &mut fixedmutex:   l83 = UNIQUE | NON_NULL, (empty)
    // 1496: &mut fixedmutex: typeof(_54) = &mut {l85} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1496: &mut fixedmutex:   l85 = UNIQUE | NON_NULL, (empty)
    // 1496: fixedmutex: typeof(_55) = *mut {l87} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1496: fixedmutex:   l87 = UNIQUE | NON_NULL, (empty)
    // 1496: &mut fixedmutex: typeof(_53 = &raw mut (*_54)) = *mut {l111} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1496: &mut fixedmutex:   l111 = UNIQUE | NON_NULL, (empty)
    // 1496: &mut fixedmutex: typeof(_54 = &mut (*_55)) = &mut {l110} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1496: &mut fixedmutex:   l110 = UNIQUE | NON_NULL, (empty)
        warn(b"failed to unlock 'fixed' in 'consumeunits'\0" as *const u8 as *const libc::c_char);
        // 1497: b"failed to unl ... _char: typeof(_57) = *const {l90} i8
        // 1497: b"failed to unl ... _char:   l90 = UNIQUE | NON_NULL, (empty)
        // 1497: b"failed to unl ... st u8: typeof(_58) = *const {l92} u8
        // 1497: b"failed to unl ... st u8:   l92 = UNIQUE | NON_NULL, (empty)
        // 1497: b"failed to unl ... s'\0": typeof(_59) = *const {l94} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
        // 1497: b"failed to unl ... s'\0":   l94 = UNIQUE | NON_NULL, (empty)
        // 1497: b"failed to unl ... s'\0": typeof(_60) = & {l96} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
        // 1497: b"failed to unl ... s'\0":   l96 = UNIQUE | NON_NULL, FIXED
        // 1497: b"failed to unl ... st u8: typeof(_58 = move _59 as *const u8 (Pointer(ArrayToPointer))) = *const {l114} u8
        // 1497: b"failed to unl ... st u8:   l114 = UNIQUE | NON_NULL, (empty)
        // 1497: b"failed to unl ... s'\0": typeof(_59 = &raw const (*_60)) = *const {l113} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
        // 1497: b"failed to unl ... s'\0":   l113 = UNIQUE | NON_NULL, (empty)
        // 1497: b"failed to unl ... s'\0": typeof(_60 = const b"failed to unlock \'fixed\' in \'consumeunits\'\x00") = & {l112} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
        // 1497: b"failed to unl ... s'\0":   l112 = UNIQUE | NON_NULL, (empty)
        // 1497: b"failed to unl ... _char: typeof(_57 = move _58 as *const i8 (Misc)) = *const {l115} i8
        // 1497: b"failed to unl ... _char:   l115 = UNIQUE | NON_NULL, (empty)
    }
}
unsafe extern "C" fn consumedunits(mut voidptr: *mut libc::c_void, mut consumed: libc::c_int) {
// 1500: mut voidptr: typeof(_1) = *mut {g22} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 1500: mut voidptr:   g22 = UNIQUE | NON_NULL, FIXED
    let mut worker: *mut Worker = voidptr as *mut Worker;
    // 1501: mut worker: typeof(_3) = *mut {l3} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1501: mut worker:   l3 = UNIQUE | NON_NULL, (empty)
    // 1501: voidptr: typeof(_4) = *mut {l5} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1501: voidptr:   l5 = UNIQUE | NON_NULL, (empty)
    // 1501: voidptr as *mut ... orker: typeof(_3 = move _4 as *mut Worker (Misc)) = *mut {l35} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1501: voidptr as *mut ... orker:   l35 = UNIQUE | NON_NULL, (empty)
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    // 1502: worker: typeof(_8) = *mut {l10} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1502: worker:   l10 = UNIQUE | NON_NULL, (empty)
    // 1502: workers: typeof(_9) = *const {l12} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1502: workers:   l12 = UNIQUE | NON_NULL, (empty)
    // 1502: workers: typeof(_10) = *mut {l14} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1502: workers:   l14 = UNIQUE | NON_NULL, (empty)
    // 1502: workers: typeof(_11) = *mut {l16} *mut {l17} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1502: workers:   l16 = UNIQUE | NON_NULL, (empty)
    // 1502: workers:   l17 = UNIQUE | NON_NULL, (empty)
    // 1502: workers: typeof(_9 = move _10 as *const Worker (Pointer(MutToConstPointer))) = *const {l36} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1502: workers:   l36 = UNIQUE | NON_NULL, (empty)
    (*worker).stats.units.consumed += consumed;
    (*worker).stats.consumed += consumed;
    msg(
        wid,
        3 as libc::c_int,
        b"consuming %d units\0" as *const u8 as *const libc::c_char,
        // 1508: b"consuming %d  ... _char: typeof(_19) = *const {l26} i8
        // 1508: b"consuming %d  ... _char:   l26 = UNIQUE | NON_NULL, (empty)
        // 1508: b"consuming %d  ... st u8: typeof(_20) = *const {l28} u8
        // 1508: b"consuming %d  ... st u8:   l28 = UNIQUE | NON_NULL, (empty)
        // 1508: b"consuming %d  ... ts\0": typeof(_21) = *const {l30} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
        // 1508: b"consuming %d  ... ts\0":   l30 = UNIQUE | NON_NULL, (empty)
        // 1508: b"consuming %d  ... ts\0": typeof(_22) = & {l32} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
        // 1508: b"consuming %d  ... ts\0":   l32 = UNIQUE | NON_NULL, FIXED
        // 1508: b"consuming %d  ... ts\0": typeof(_21 = &raw const (*_22)) = *const {l38} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
        // 1508: b"consuming %d  ... ts\0":   l38 = UNIQUE | NON_NULL, (empty)
        // 1508: b"consuming %d  ... ts\0": typeof(_22 = const b"consuming %d units\x00") = & {l37} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
        // 1508: b"consuming %d  ... ts\0":   l37 = UNIQUE | NON_NULL, (empty)
        // 1508: b"consuming %d  ... st u8: typeof(_20 = move _21 as *const u8 (Pointer(ArrayToPointer))) = *const {l39} u8
        // 1508: b"consuming %d  ... st u8:   l39 = UNIQUE | NON_NULL, (empty)
        // 1508: b"consuming %d  ... _char: typeof(_19 = move _20 as *const i8 (Misc)) = *const {l40} i8
        // 1508: b"consuming %d  ... _char:   l40 = UNIQUE | NON_NULL, (empty)
        consumed,
    );
}
unsafe extern "C" fn sizecls(mut len: libc::c_int) -> size_t {
    return (::core::mem::size_of::<Cls>() as libc::c_ulong).wrapping_add(
        (len as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
}
unsafe extern "C" fn lencls(mut c: *mut libc::c_int) -> libc::c_int {
// 1517: mut c: typeof(_1) = *mut {g23} i32
// 1517: mut c:   g23 = READ | OFFSET_ADD | OFFSET_SUB, FIXED
    let mut res: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh5 = c;
        // 1520: fresh5: typeof(_6) = *mut {l6} i32
        // 1520: fresh5:   l6 = READ, (empty)
        c = c.offset(1);
        // 1521: c.offset(1): typeof(_7) = *mut {l8} i32
        // 1521: c.offset(1):   l8 = READ | OFFSET_ADD | OFFSET_SUB, (empty)
        // 1521: c: typeof(_8) = *mut {l10} i32
        // 1521: c:   l10 = READ | OFFSET_ADD | OFFSET_SUB, (empty)
        if !(*fresh5 != 0) {
            break;
        }
        res += 1;
        res;
    }
    return res;
}
unsafe extern "C" fn producecls(
    mut voidptr: *mut libc::c_void,
    // 1531: mut voidptr: typeof(_1) = *mut {g24} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1531: mut voidptr:   g24 = UNIQUE | NON_NULL, FIXED
    mut c: *mut libc::c_int,
    // 1532: mut c: typeof(_2) = *mut {g25} i32
    // 1532: mut c:   g25 = UNIQUE | NON_NULL, FIXED
    mut glue: libc::c_int,
) {
    let mut worker: *mut Worker = voidptr as *mut Worker;
    // 1535: mut worker: typeof(_4) = *mut {l4} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1535: mut worker:   l4 = UNIQUE | NON_NULL, (empty)
    // 1535: voidptr: typeof(_5) = *mut {l6} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1535: voidptr:   l6 = UNIQUE | NON_NULL, (empty)
    // 1535: voidptr as *mut ... orker: typeof(_4 = move _5 as *mut Worker (Misc)) = *mut {l231} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1535: voidptr as *mut ... orker:   l231 = UNIQUE | NON_NULL, (empty)
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    // 1536: worker: typeof(_9) = *mut {l11} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1536: worker:   l11 = UNIQUE | NON_NULL, (empty)
    // 1536: workers: typeof(_10) = *const {l13} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1536: workers:   l13 = UNIQUE | NON_NULL, (empty)
    // 1536: workers: typeof(_11) = *mut {l15} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1536: workers:   l15 = UNIQUE | NON_NULL, (empty)
    // 1536: workers: typeof(_12) = *mut {l17} *mut {l18} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1536: workers:   l17 = UNIQUE | NON_NULL, (empty)
    // 1536: workers:   l18 = UNIQUE | NON_NULL, (empty)
    // 1536: workers: typeof(_10 = move _11 as *const Worker (Pointer(MutToConstPointer))) = *const {l232} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1536: workers:   l232 = UNIQUE | NON_NULL, (empty)
    let mut len: libc::c_int = 0;
    let mut q: *mut libc::c_int = 0 as *mut libc::c_int;
    // 1538: mut q: typeof(_14) = *mut {l21} i32
    // 1538: mut q:   l21 = UNIQUE | NON_NULL, (empty)
    // 1538: 0 as *mut libc: ... c_int: typeof(_14 = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l233} i32
    // 1538: 0 as *mut libc: ... c_int:   l233 = UNIQUE | NON_NULL, (empty)
    let mut lit: libc::c_int = 0;
    let mut p: *const libc::c_int = 0 as *const libc::c_int;
    // 1540: mut p: typeof(_16) = *const {l24} i32
    // 1540: mut p:   l24 = UNIQUE | NON_NULL, (empty)
    // 1540: 0 as *const lib ... c_int: typeof(_16 = const 0_usize as *const i32 (PointerFromExposedAddress)) = *const {l234} i32
    // 1540: 0 as *const lib ... c_int:   l234 = UNIQUE | NON_NULL, (empty)
    let mut bytes: size_t = 0;
    let mut cls: *mut Cls = 0 as *mut Cls;
    // 1542: mut cls: typeof(_18) = *mut {l27} DefId(0:525 ~ plingeling[18f5]::Cls)
    // 1542: mut cls:   l27 = UNIQUE | NON_NULL, (empty)
    // 1542: 0 as *mut Cls: typeof(_18 = const 0_usize as *mut Cls (PointerFromExposedAddress)) = *mut {l235} DefId(0:525 ~ plingeling[18f5]::Cls)
    // 1542: 0 as *mut Cls:   l235 = UNIQUE | NON_NULL, (empty)
    len = lencls(c);
    // 1543: c: typeof(_20) = *mut {l30} i32
    // 1543: c:   l30 = UNIQUE | NON_NULL, (empty)
    bytes = sizecls(len);
    cls = malloc(bytes) as *mut Cls;
    // 1545: malloc(bytes): typeof(_23) = *mut {l34} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1545: malloc(bytes):   l34 = UNIQUE | NON_NULL, (empty)
    // 1545: cls = malloc(by ... t Cls: typeof(_18 = move _23 as *mut Cls (Misc)) = *mut {l236} DefId(0:525 ~ plingeling[18f5]::Cls)
    // 1545: cls = malloc(by ... t Cls:   l236 = UNIQUE | NON_NULL, (empty)
    if cls.is_null() {
    // 1546: cls: typeof(_27) = *mut {l39} DefId(0:525 ~ plingeling[18f5]::Cls)
    // 1546: cls:   l39 = UNIQUE | NON_NULL, (empty)
        die(b"out of memory in 'producecls'\0" as *const u8 as *const libc::c_char);
        // 1547: b"out of memory ... _char: typeof(_30) = *const {l43} i8
        // 1547: b"out of memory ... _char:   l43 = UNIQUE | NON_NULL, (empty)
        // 1547: b"out of memory ... st u8: typeof(_31) = *const {l45} u8
        // 1547: b"out of memory ... st u8:   l45 = UNIQUE | NON_NULL, (empty)
        // 1547: b"out of memory ... s'\0": typeof(_32) = *const {l47} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
        // 1547: b"out of memory ... s'\0":   l47 = UNIQUE | NON_NULL, (empty)
        // 1547: b"out of memory ... s'\0": typeof(_33) = & {l49} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
        // 1547: b"out of memory ... s'\0":   l49 = UNIQUE | NON_NULL, FIXED
        // 1547: b"out of memory ... _char: typeof(_30 = move _31 as *const i8 (Misc)) = *const {l240} i8
        // 1547: b"out of memory ... _char:   l240 = UNIQUE | NON_NULL, (empty)
        // 1547: b"out of memory ... s'\0": typeof(_32 = &raw const (*_33)) = *const {l238} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
        // 1547: b"out of memory ... s'\0":   l238 = UNIQUE | NON_NULL, (empty)
        // 1547: b"out of memory ... st u8: typeof(_31 = move _32 as *const u8 (Pointer(ArrayToPointer))) = *const {l239} u8
        // 1547: b"out of memory ... st u8:   l239 = UNIQUE | NON_NULL, (empty)
        // 1547: b"out of memory ... s'\0": typeof(_33 = const b"out of memory in \'producecls\'\x00") = & {l237} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
        // 1547: b"out of memory ... s'\0":   l237 = UNIQUE | NON_NULL, (empty)
        exit(1 as libc::c_int);
    }
    incmem(bytes);
    (*cls).wid = wid;
    (*cls).glue = glue;
    (*cls).count = 0 as libc::c_int;
    p = c;
    // 1554: c: typeof(_41) = *mut {l58} i32
    // 1554: c:   l58 = UNIQUE | NON_NULL, (empty)
    // 1554: p = c: typeof(_16 = move _41 as *const i32 (Pointer(MutToConstPointer))) = *const {l241} i32
    // 1554: p = c:   l241 = UNIQUE | NON_NULL, (empty)
    q = ((*cls).lits).as_mut_ptr();
    // 1555: ((*cls).lits).a ... ptr(): typeof(_42) = *mut {l60} i32
    // 1555: ((*cls).lits).a ... ptr():   l60 = UNIQUE | NON_NULL, (empty)
    // 1555: ((*cls).lits).a ... ptr(): typeof(_43) = &mut {l62} [i32]
    // 1555: ((*cls).lits).a ... ptr():   l62 = UNIQUE | NON_NULL, FIXED
    // 1555: ((*cls).lits).a ... ptr(): typeof(_44) = &mut {l64} [i32; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
    // 1555: ((*cls).lits).a ... ptr():   l64 = UNIQUE | NON_NULL, (empty)
    // 1555: ((*cls).lits).a ... ptr(): typeof(_43 = move _44 as &mut [i32] (Pointer(Unsize))) = &mut {l243} [i32]
    // 1555: ((*cls).lits).a ... ptr():   l243 = UNIQUE | NON_NULL, FIXED
    // 1555: ((*cls).lits).a ... ptr(): typeof(_44 = &mut ((*_18).3: [i32; 1])) = &mut {l242} [i32; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
    // 1555: ((*cls).lits).a ... ptr():   l242 = UNIQUE | NON_NULL, (empty)
    loop {
        let fresh6 = p;
        // 1557: fresh6: typeof(_47) = *const {l68} i32
        // 1557: fresh6:   l68 = UNIQUE | NON_NULL, (empty)
        p = p.offset(1);
        // 1558: p.offset(1): typeof(_48) = *const {l70} i32
        // 1558: p.offset(1):   l70 = UNIQUE | NON_NULL, (empty)
        // 1558: p: typeof(_49) = *const {l72} i32
        // 1558: p:   l72 = UNIQUE | NON_NULL, (empty)
        lit = *fresh6;
        if !(lit != 0) {
            break;
        }
        let fresh7 = q;
        // 1563: fresh7: typeof(_56) = *mut {l80} i32
        // 1563: fresh7:   l80 = UNIQUE | NON_NULL, (empty)
        q = q.offset(1);
        // 1564: q.offset(1): typeof(_57) = *mut {l82} i32
        // 1564: q.offset(1):   l82 = UNIQUE | NON_NULL, (empty)
        // 1564: q: typeof(_58) = *mut {l84} i32
        // 1564: q:   l84 = UNIQUE | NON_NULL, (empty)
        *fresh7 = lit;
    }
    let fresh8 = q;
    // 1567: fresh8: typeof(_60) = *mut {l87} i32
    // 1567: fresh8:   l87 = UNIQUE | NON_NULL, (empty)
    q = q.offset(1);
    // 1568: q.offset(1): typeof(_61) = *mut {l89} i32
    // 1568: q.offset(1):   l89 = UNIQUE | NON_NULL, (empty)
    // 1568: q: typeof(_62) = *mut {l91} i32
    // 1568: q:   l91 = UNIQUE | NON_NULL, (empty)
    *fresh8 = 0 as libc::c_int;
    if pthread_mutex_lock(&mut clsmutex) != 0 {
    // 1570: &mut clsmutex: typeof(_67) = *mut {l97} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1570: &mut clsmutex:   l97 = UNIQUE | NON_NULL, (empty)
    // 1570: &mut clsmutex: typeof(_68) = &mut {l99} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1570: &mut clsmutex:   l99 = UNIQUE | NON_NULL, (empty)
    // 1570: clsmutex: typeof(_69) = *mut {l101} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1570: clsmutex:   l101 = UNIQUE | NON_NULL, (empty)
    // 1570: &mut clsmutex: typeof(_68 = &mut (*_69)) = &mut {l244} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1570: &mut clsmutex:   l244 = UNIQUE | NON_NULL, (empty)
    // 1570: &mut clsmutex: typeof(_67 = &raw mut (*_68)) = *mut {l245} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1570: &mut clsmutex:   l245 = UNIQUE | NON_NULL, (empty)
        warn(b"failed to lock 'cls' mutex in 'producecls'\0" as *const u8 as *const libc::c_char);
        // 1571: b"failed to loc ... _char: typeof(_71) = *const {l104} i8
        // 1571: b"failed to loc ... _char:   l104 = UNIQUE | NON_NULL, (empty)
        // 1571: b"failed to loc ... st u8: typeof(_72) = *const {l106} u8
        // 1571: b"failed to loc ... st u8:   l106 = UNIQUE | NON_NULL, (empty)
        // 1571: b"failed to loc ... s'\0": typeof(_73) = *const {l108} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
        // 1571: b"failed to loc ... s'\0":   l108 = UNIQUE | NON_NULL, (empty)
        // 1571: b"failed to loc ... s'\0": typeof(_74) = & {l110} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
        // 1571: b"failed to loc ... s'\0":   l110 = UNIQUE | NON_NULL, FIXED
        // 1571: b"failed to loc ... s'\0": typeof(_74 = const b"failed to lock \'cls\' mutex in \'producecls\'\x00") = & {l246} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
        // 1571: b"failed to loc ... s'\0":   l246 = UNIQUE | NON_NULL, (empty)
        // 1571: b"failed to loc ... _char: typeof(_71 = move _72 as *const i8 (Misc)) = *const {l249} i8
        // 1571: b"failed to loc ... _char:   l249 = UNIQUE | NON_NULL, (empty)
        // 1571: b"failed to loc ... st u8: typeof(_72 = move _73 as *const u8 (Pointer(ArrayToPointer))) = *const {l248} u8
        // 1571: b"failed to loc ... st u8:   l248 = UNIQUE | NON_NULL, (empty)
        // 1571: b"failed to loc ... s'\0": typeof(_73 = &raw const (*_74)) = *const {l247} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
        // 1571: b"failed to loc ... s'\0":   l247 = UNIQUE | NON_NULL, (empty)
    }
    msg(
        wid,
        3 as libc::c_int,
        b"producing glue %d length %d clause %ld\0" as *const u8 as *const libc::c_char,
        // 1576: b"producing glu ... _char: typeof(_78) = *const {l115} i8
        // 1576: b"producing glu ... _char:   l115 = UNIQUE | NON_NULL, (empty)
        // 1576: b"producing glu ... st u8: typeof(_79) = *const {l117} u8
        // 1576: b"producing glu ... st u8:   l117 = UNIQUE | NON_NULL, (empty)
        // 1576: b"producing glu ... ld\0": typeof(_80) = *const {l119} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
        // 1576: b"producing glu ... ld\0":   l119 = UNIQUE | NON_NULL, (empty)
        // 1576: b"producing glu ... ld\0": typeof(_81) = & {l121} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
        // 1576: b"producing glu ... ld\0":   l121 = UNIQUE | NON_NULL, FIXED
        // 1576: b"producing glu ... _char: typeof(_78 = move _79 as *const i8 (Misc)) = *const {l253} i8
        // 1576: b"producing glu ... _char:   l253 = UNIQUE | NON_NULL, (empty)
        // 1576: b"producing glu ... st u8: typeof(_79 = move _80 as *const u8 (Pointer(ArrayToPointer))) = *const {l252} u8
        // 1576: b"producing glu ... st u8:   l252 = UNIQUE | NON_NULL, (empty)
        // 1576: b"producing glu ... ld\0": typeof(_81 = const b"producing glue %d length %d clause %ld\x00") = & {l250} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
        // 1576: b"producing glu ... ld\0":   l250 = UNIQUE | NON_NULL, (empty)
        // 1576: b"producing glu ... ld\0": typeof(_80 = &raw const (*_81)) = *const {l251} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
        // 1576: b"producing glu ... ld\0":   l251 = UNIQUE | NON_NULL, (empty)
        glue,
        len,
        clauses.added,
        // 1579: clauses: typeof(_85) = *mut {l126} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
        // 1579: clauses:   l126 = UNIQUE | NON_NULL, (empty)
    );
    if clauses.num == clauses.size {
    // 1581: clauses: typeof(_89) = *mut {l131} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
    // 1581: clauses:   l131 = UNIQUE | NON_NULL, (empty)
    // 1581: clauses: typeof(_91) = *mut {l134} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
    // 1581: clauses:   l134 = UNIQUE | NON_NULL, (empty)
        let mut newsize: libc::c_int = (if clauses.size != 0 {
        // 1582: clauses: typeof(_96) = *mut {l140} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
        // 1582: clauses:   l140 = UNIQUE | NON_NULL, (empty)
            2 as libc::c_int as libc::c_long * clauses.size
            // 1583: clauses: typeof(_100) = *mut {l145} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
            // 1583: clauses:   l145 = UNIQUE | NON_NULL, (empty)
        } else {
            1 as libc::c_int as libc::c_long
        }) as libc::c_int;
        let mut old_bytes: size_t = (clauses.size as libc::c_ulong)
        // 1587: clauses: typeof(_106) = *mut {l152} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
        // 1587: clauses:   l152 = UNIQUE | NON_NULL, (empty)
            .wrapping_mul(::core::mem::size_of::<*mut Cls>() as libc::c_ulong);
        let mut new_bytes: size_t = (newsize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Cls>() as libc::c_ulong);
        clauses.start = resize(
        // 1591: resize( 0 as *m ... es, ): typeof(_114) = *mut {l161} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1591: resize( 0 as *m ... es, ):   l161 = UNIQUE | NON_NULL, (empty)
        // 1591: clauses: typeof(_121) = *mut {l174} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
        // 1591: clauses:   l174 = UNIQUE | NON_NULL, (empty)
        // 1591: clauses.start = ... t Cls: typeof(((*_121).0: *mut *mut Cls) = move _114 as *mut *mut Cls (Misc)) = *mut {l256} *mut {l257} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1591: clauses.start = ... t Cls:   l256 = UNIQUE | NON_NULL, (empty)
        // 1591: clauses.start = ... t Cls:   l257 = UNIQUE | NON_NULL, (empty)
            0 as *mut libc::c_void,
            // 1592: 0 as *mut libc: ... _void: typeof(_115) = *mut {l163} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1592: 0 as *mut libc: ... _void:   l163 = UNIQUE | NON_NULL, (empty)
            // 1592: 0 as *mut libc: ... _void: typeof(_115 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l254} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1592: 0 as *mut libc: ... _void:   l254 = UNIQUE | NON_NULL, (empty)
            clauses.start as *mut libc::c_void,
            // 1593: clauses.start a ... _void: typeof(_116) = *mut {l165} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1593: clauses.start a ... _void:   l165 = UNIQUE | NON_NULL, (empty)
            // 1593: clauses.start: typeof(_117) = *mut {l167} *mut {l168} DefId(0:525 ~ plingeling[18f5]::Cls)
            // 1593: clauses.start:   l167 = UNIQUE | NON_NULL, (empty)
            // 1593: clauses.start:   l168 = UNIQUE | NON_NULL, (empty)
            // 1593: clauses: typeof(_118) = *mut {l170} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
            // 1593: clauses:   l170 = UNIQUE | NON_NULL, (empty)
            // 1593: clauses.start a ... _void: typeof(_116 = move _117 as *mut libc::c_void (Misc)) = *mut {l255} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1593: clauses.start a ... _void:   l255 = UNIQUE | NON_NULL, (empty)
            old_bytes,
            new_bytes,
        ) as *mut *mut Cls;
        clauses.size = newsize as libc::c_long;
        // 1597: clauses: typeof(_123) = *mut {l177} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
        // 1597: clauses:   l177 = UNIQUE | NON_NULL, (empty)
    }
    let fresh9 = clauses.num;
    // 1599: clauses: typeof(_125) = *mut {l180} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
    // 1599: clauses:   l180 = UNIQUE | NON_NULL, (empty)
    clauses.num = clauses.num + 1;
    // 1600: clauses: typeof(_127) = *mut {l183} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
    // 1600: clauses:   l183 = UNIQUE | NON_NULL, (empty)
    // 1600: clauses: typeof(_129) = *mut {l186} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
    // 1600: clauses:   l186 = UNIQUE | NON_NULL, (empty)
    let ref mut fresh10 = *(clauses.start).offset(fresh9 as isize);
    // 1601: ref mut fresh10: typeof(_130) = &mut {l188} *mut {l189} DefId(0:525 ~ plingeling[18f5]::Cls)
    // 1601: ref mut fresh10:   l188 = UNIQUE | NON_NULL, FIXED
    // 1601: ref mut fresh10:   l189 = UNIQUE | NON_NULL, (empty)
    // 1601: (clauses.start) ... size): typeof(_131) = *mut {l191} *mut {l192} DefId(0:525 ~ plingeling[18f5]::Cls)
    // 1601: (clauses.start) ... size):   l191 = UNIQUE | NON_NULL, (empty)
    // 1601: (clauses.start) ... size):   l192 = UNIQUE | NON_NULL, (empty)
    // 1601: (clauses.start): typeof(_132) = *mut {l194} *mut {l195} DefId(0:525 ~ plingeling[18f5]::Cls)
    // 1601: (clauses.start):   l194 = UNIQUE | NON_NULL, (empty)
    // 1601: (clauses.start):   l195 = UNIQUE | NON_NULL, (empty)
    // 1601: clauses: typeof(_133) = *mut {l197} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
    // 1601: clauses:   l197 = UNIQUE | NON_NULL, (empty)
    // 1601: ref mut fresh10: typeof(_130 = &mut (*_131)) = &mut {l258} *mut {l259} DefId(0:525 ~ plingeling[18f5]::Cls)
    // 1601: ref mut fresh10:   l258 = UNIQUE | NON_NULL, (empty)
    // 1601: ref mut fresh10:   l259 = UNIQUE | NON_NULL, (empty)
    *fresh10 = cls;
    // 1602: cls: typeof(_136) = *mut {l201} DefId(0:525 ~ plingeling[18f5]::Cls)
    // 1602: cls:   l201 = UNIQUE | NON_NULL, (empty)
    clauses.added += 1;
    // 1603: clauses: typeof(_137) = *mut {l203} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
    // 1603: clauses:   l203 = UNIQUE | NON_NULL, (empty)
    clauses.added;
    // 1604: clauses: typeof(_140) = *mut {l207} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
    // 1604: clauses:   l207 = UNIQUE | NON_NULL, (empty)
    if pthread_mutex_unlock(&mut clsmutex) != 0 {
    // 1605: &mut clsmutex: typeof(_144) = *mut {l212} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1605: &mut clsmutex:   l212 = UNIQUE | NON_NULL, (empty)
    // 1605: &mut clsmutex: typeof(_145) = &mut {l214} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1605: &mut clsmutex:   l214 = UNIQUE | NON_NULL, (empty)
    // 1605: clsmutex: typeof(_146) = *mut {l216} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1605: clsmutex:   l216 = UNIQUE | NON_NULL, (empty)
    // 1605: &mut clsmutex: typeof(_145 = &mut (*_146)) = &mut {l260} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1605: &mut clsmutex:   l260 = UNIQUE | NON_NULL, (empty)
    // 1605: &mut clsmutex: typeof(_144 = &raw mut (*_145)) = *mut {l261} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1605: &mut clsmutex:   l261 = UNIQUE | NON_NULL, (empty)
        warn(b"failed to unlock 'cls' mutex in 'producecls'\0" as *const u8 as *const libc::c_char);
        // 1606: b"failed to unl ... _char: typeof(_148) = *const {l219} i8
        // 1606: b"failed to unl ... _char:   l219 = UNIQUE | NON_NULL, (empty)
        // 1606: b"failed to unl ... st u8: typeof(_149) = *const {l221} u8
        // 1606: b"failed to unl ... st u8:   l221 = UNIQUE | NON_NULL, (empty)
        // 1606: b"failed to unl ... s'\0": typeof(_150) = *const {l223} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
        // 1606: b"failed to unl ... s'\0":   l223 = UNIQUE | NON_NULL, (empty)
        // 1606: b"failed to unl ... s'\0": typeof(_151) = & {l225} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
        // 1606: b"failed to unl ... s'\0":   l225 = UNIQUE | NON_NULL, FIXED
        // 1606: b"failed to unl ... s'\0": typeof(_151 = const b"failed to unlock \'cls\' mutex in \'producecls\'\x00") = & {l262} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
        // 1606: b"failed to unl ... s'\0":   l262 = UNIQUE | NON_NULL, (empty)
        // 1606: b"failed to unl ... st u8: typeof(_149 = move _150 as *const u8 (Pointer(ArrayToPointer))) = *const {l264} u8
        // 1606: b"failed to unl ... st u8:   l264 = UNIQUE | NON_NULL, (empty)
        // 1606: b"failed to unl ... _char: typeof(_148 = move _149 as *const i8 (Misc)) = *const {l265} i8
        // 1606: b"failed to unl ... _char:   l265 = UNIQUE | NON_NULL, (empty)
        // 1606: b"failed to unl ... s'\0": typeof(_150 = &raw const (*_151)) = *const {l263} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
        // 1606: b"failed to unl ... s'\0":   l263 = UNIQUE | NON_NULL, (empty)
    }
    (*worker).stats.cls.produced += 1;
    (*worker).stats.cls.produced;
    (*worker).stats.produced += 1;
    (*worker).stats.produced;
}
unsafe extern "C" fn deletecls<'h0>(mut cls: core::option::Option<&'h0 (Cls)>) {
// 1613: mut cls: typeof(_1) = *mut {g26} DefId(0:525 ~ plingeling[18f5]::Cls)
// 1613: mut cls:   g26 = READ | FREE, (empty)
    let mut len: libc::c_int = lencls(core::ptr::addr_of!(*&std::option::Option::Some(&*(std::option::Option::Some(&mut (((*(cls).unwrap()).lits))) as &mut [i32])).unwrap()[0]).cast_mut());
    // 1614: ((*cls).lits).a ... ptr(): typeof(_3) = *mut {l3} i32
    // 1614: ((*cls).lits).a ... ptr():   l3 = READ | OFFSET_ADD | OFFSET_SUB, (empty)
    // 1614: ((*cls).lits).a ... ptr(): typeof(_4) = &mut {l5} [i32]
    // 1614: ((*cls).lits).a ... ptr():   l5 = READ | OFFSET_ADD | OFFSET_SUB, FIXED
    // 1614: ((*cls).lits).a ... ptr(): typeof(_5) = &mut {l7} [i32; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
    // 1614: ((*cls).lits).a ... ptr():   l7 = READ | OFFSET_ADD | OFFSET_SUB, (empty)
    // 1614: ((*cls).lits).a ... ptr(): typeof(_5 = &mut ((*_1).3: [i32; 1])) = &mut {l18} [i32; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
    // 1614: ((*cls).lits).a ... ptr():   l18 = READ, (empty)
    // 1614: ((*cls).lits).a ... ptr(): typeof(_4 = move _5 as &mut [i32] (Pointer(Unsize))) = &mut {l19} [i32]
    // 1614: ((*cls).lits).a ... ptr():   l19 = READ | OFFSET_ADD | OFFSET_SUB, FIXED
    let mut bytes: size_t = sizecls(len);
    decmem(bytes);
    free(cls as *mut libc::c_void);
    // 1617: cls as *mut lib ... _void: typeof(_11) = *mut {l14} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1617: cls as *mut lib ... _void:   l14 = FREE, (empty)
    // 1617: cls: typeof(_12) = *mut {l16} DefId(0:525 ~ plingeling[18f5]::Cls)
    // 1617: cls:   l16 = FREE, (empty)
    // 1617: cls as *mut lib ... _void: typeof(_11 = move _12 as *mut libc::c_void (Misc)) = *mut {l20} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1617: cls as *mut lib ... _void:   l20 = FREE, (empty)
}
unsafe fn deletecls_shim(arg0: *mut Cls) {
    {
    let safe_arg0 = std::option::Option::Some(&*arg0.cast_const());
    let safe_result = deletecls(safe_arg0);
    let result = safe_result;
    result
}
}

unsafe extern "C" fn deleteallcls() {
    let mut c: *mut Cls = 0 as *mut Cls;
    // 1620: mut c: typeof(_1) = *mut {l1} DefId(0:525 ~ plingeling[18f5]::Cls)
    // 1620: mut c:   l1 = READ | FREE, (empty)
    // 1620: 0 as *mut Cls: typeof(_1 = const 0_usize as *mut Cls (PointerFromExposedAddress)) = *mut {l54} DefId(0:525 ~ plingeling[18f5]::Cls)
    // 1620: 0 as *mut Cls:   l54 = READ | UNIQUE | FREE, (empty)
    let mut i: libc::c_int = 0;
    i = clauses.first as libc::c_int;
    // 1622: clauses: typeof(_4) = *mut {l5} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
    // 1622: clauses:   l5 = READ | UNIQUE | NON_NULL, (empty)
    while (i as libc::c_long) < clauses.num {
    // 1623: clauses: typeof(_11) = *mut {l13} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
    // 1623: clauses:   l13 = READ | UNIQUE | NON_NULL, (empty)
        c = *(clauses.start).offset(i as isize);
        // 1624: *(clauses.start ... size): typeof(_12) = *mut {l15} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1624: *(clauses.start ... size):   l15 = READ | FREE, (empty)
        // 1624: (clauses.start) ... size): typeof(_13) = *mut {l17} *mut {g267} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1624: (clauses.start) ... size):   l17 = READ | UNIQUE | NON_NULL, (empty)
        // 1624: (clauses.start) ... size):   g267 = READ | FREE, FIXED
        // 1624: (clauses.start): typeof(_14) = *mut {l19} *mut {g267} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1624: (clauses.start):   l19 = READ | UNIQUE | OFFSET_ADD | OFFSET_SUB | NON_NULL, (empty)
        // 1624: (clauses.start):   g267 = READ | FREE, FIXED
        // 1624: clauses: typeof(_15) = *mut {l21} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
        // 1624: clauses:   l21 = READ | UNIQUE | NON_NULL, (empty)
        if !c.is_null() {
        // 1625: c: typeof(_21) = *mut {l28} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1625: c:   l28 = (empty), (empty)
            deletecls_shim(c);
            // 1626: c: typeof(_23) = *mut {l31} DefId(0:525 ~ plingeling[18f5]::Cls)
            // 1626: c:   l31 = READ | FREE, (empty)
        }
        i += 1;
        i;
    }
    let mut BYTES: size_t = (clauses.size as libc::c_ulong)
    // 1631: clauses: typeof(_32) = *mut {l41} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
    // 1631: clauses:   l41 = READ | UNIQUE | NON_NULL, (empty)
        .wrapping_mul(::core::mem::size_of::<*mut Cls>() as libc::c_ulong);
    decmem(BYTES);
    free(clauses.start as *mut libc::c_void);
    // 1634: clauses.start a ... _void: typeof(_38) = *mut {l48} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1634: clauses.start a ... _void:   l48 = UNIQUE | FREE | NON_NULL, (empty)
    // 1634: clauses.start: typeof(_39) = *mut {l50} *mut {g267} DefId(0:525 ~ plingeling[18f5]::Cls)
    // 1634: clauses.start:   l50 = UNIQUE | FREE | NON_NULL, (empty)
    // 1634: clauses.start:   g267 = READ | FREE, FIXED
    // 1634: clauses: typeof(_40) = *mut {l52} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
    // 1634: clauses:   l52 = READ | UNIQUE | NON_NULL, (empty)
    // 1634: clauses.start a ... _void: typeof(_38 = move _39 as *mut libc::c_void (Misc)) = *mut {l55} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1634: clauses.start a ... _void:   l55 = UNIQUE | FREE | NON_NULL, (empty)
}
unsafe extern "C" fn gcls() {
    let mut j: libc::c_long = 0;
    let mut k: libc::c_long = 0;
    let mut l: libc::c_long = 0;
    let mut c: *mut Cls = 0 as *mut Cls;
    // 1640: mut c: typeof(_4) = *mut {l4} DefId(0:525 ~ plingeling[18f5]::Cls)
    // 1640: mut c:   l4 = READ | FREE, (empty)
    // 1640: 0 as *mut Cls: typeof(_4 = const 0_usize as *mut Cls (PointerFromExposedAddress)) = *mut {l164} DefId(0:525 ~ plingeling[18f5]::Cls)
    // 1640: 0 as *mut Cls:   l164 = READ | UNIQUE | FREE, (empty)
    let mut i: libc::c_int = 0;
    gcs += 1;
    // 1642: gcs: typeof(_6) = *mut {l7} i32
    // 1642: gcs:   l7 = READ | WRITE | UNIQUE | NON_NULL, (empty)
    gcs;
    // 1643: gcs: typeof(_9) = *mut {l11} i32
    // 1643: gcs:   l11 = READ | UNIQUE | NON_NULL, (empty)
    msg(
        -(1 as libc::c_int),
        1 as libc::c_int,
        b"garbage collecting clauses: first=%ld, num=%ld, SIZE=%ld\0" as *const u8
        // 1647: b"garbage colle ... _char: typeof(_15) = *const {l18} i8
        // 1647: b"garbage colle ... _char:   l18 = UNIQUE | NON_NULL, (empty)
        // 1647: b"garbage colle ... st u8: typeof(_16) = *const {l20} u8
        // 1647: b"garbage colle ... st u8:   l20 = UNIQUE | NON_NULL, (empty)
        // 1647: b"garbage colle ... ld\0": typeof(_17) = *const {l22} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
        // 1647: b"garbage colle ... ld\0":   l22 = UNIQUE | NON_NULL, (empty)
        // 1647: b"garbage colle ... ld\0": typeof(_18) = & {l24} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
        // 1647: b"garbage colle ... ld\0":   l24 = UNIQUE | NON_NULL, FIXED
        // 1647: b"garbage colle ... ld\0": typeof(_17 = &raw const (*_18)) = *const {l166} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
        // 1647: b"garbage colle ... ld\0":   l166 = UNIQUE | NON_NULL, (empty)
        // 1647: b"garbage colle ... _char: typeof(_15 = move _16 as *const i8 (Misc)) = *const {l168} i8
        // 1647: b"garbage colle ... _char:   l168 = UNIQUE | NON_NULL, (empty)
        // 1647: b"garbage colle ... ld\0": typeof(_18 = const b"garbage collecting clauses: first=%ld, num=%ld, SIZE=%ld\x00") = & {l165} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
        // 1647: b"garbage colle ... ld\0":   l165 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
        // 1647: b"garbage colle ... st u8: typeof(_16 = move _17 as *const u8 (Pointer(ArrayToPointer))) = *const {l167} u8
        // 1647: b"garbage colle ... st u8:   l167 = UNIQUE | NON_NULL, (empty)
            as *const libc::c_char,
        clauses.first,
        // 1649: clauses: typeof(_20) = *mut {l27} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
        // 1649: clauses:   l27 = READ | UNIQUE | NON_NULL, (empty)
        clauses.num,
        // 1650: clauses: typeof(_22) = *mut {l30} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
        // 1650: clauses:   l30 = READ | UNIQUE | NON_NULL, (empty)
        clauses.size,
        // 1651: clauses: typeof(_24) = *mut {l33} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
        // 1651: clauses:   l33 = READ | UNIQUE | NON_NULL, (empty)
    );
    k = 0 as libc::c_int as libc::c_long;
    while k < clauses.first {
    // 1654: clauses: typeof(_31) = *mut {l41} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
    // 1654: clauses:   l41 = READ | UNIQUE | NON_NULL, (empty)
        k += 1;
        k;
    }
    while k < clauses.num {
    // 1658: clauses: typeof(_41) = *mut {l52} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
    // 1658: clauses:   l52 = READ | UNIQUE | NON_NULL, (empty)
        c = *(clauses.start).offset(k as isize);
        // 1659: *(clauses.start ... size): typeof(_42) = *mut {l54} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1659: *(clauses.start ... size):   l54 = READ | FREE, (empty)
        // 1659: (clauses.start) ... size): typeof(_43) = *mut {l56} *mut {g267} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1659: (clauses.start) ... size):   l56 = READ | UNIQUE | NON_NULL, (empty)
        // 1659: (clauses.start) ... size):   g267 = READ | FREE, FIXED
        // 1659: (clauses.start): typeof(_44) = *mut {l58} *mut {g267} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1659: (clauses.start):   l58 = READ | UNIQUE | OFFSET_ADD | OFFSET_SUB | NON_NULL, (empty)
        // 1659: (clauses.start):   g267 = READ | FREE, FIXED
        // 1659: clauses: typeof(_45) = *mut {l60} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
        // 1659: clauses:   l60 = READ | UNIQUE | NON_NULL, (empty)
        if (*c).count > gclim {
        // 1660: gclim: typeof(_52) = *mut {l68} i32
        // 1660: gclim:   l68 = READ | UNIQUE | NON_NULL, (empty)
            break;
        }
        let fresh11 = k;
        k = k + 1;
        let ref mut fresh12 = *(clauses.start).offset(fresh11 as isize);
        // 1665: ref mut fresh12: typeof(_57) = &mut {l74} *mut {g267} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1665: ref mut fresh12:   l74 = READ | WRITE | UNIQUE | NON_NULL, FIXED
        // 1665: ref mut fresh12:   g267 = READ | FREE, FIXED
        // 1665: (clauses.start) ... size): typeof(_58) = *mut {l76} *mut {g267} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1665: (clauses.start) ... size):   l76 = READ | WRITE | UNIQUE | NON_NULL, (empty)
        // 1665: (clauses.start) ... size):   g267 = READ | FREE, FIXED
        // 1665: (clauses.start): typeof(_59) = *mut {l78} *mut {g267} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1665: (clauses.start):   l78 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | NON_NULL, (empty)
        // 1665: (clauses.start):   g267 = READ | FREE, FIXED
        // 1665: clauses: typeof(_60) = *mut {l80} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
        // 1665: clauses:   l80 = READ | UNIQUE | NON_NULL, (empty)
        // 1665: ref mut fresh12: typeof(_57 = &mut (*_58)) = &mut {l169} *mut {g267} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1665: ref mut fresh12:   l169 = READ | WRITE | UNIQUE | NON_NULL, (empty)
        // 1665: ref mut fresh12:   g267 = READ | FREE, FIXED
        *fresh12 = 0 as *mut Cls;
        // 1666: *fresh12 = 0 as ... t Cls: typeof((*_57) = const 0_usize as *mut Cls (PointerFromExposedAddress)) = *mut {l170} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1666: *fresh12 = 0 as ... t Cls:   l170 = READ | UNIQUE | FREE, (empty)
        deletecls_shim(c);
        // 1667: c: typeof(_64) = *mut {l85} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1667: c:   l85 = READ | FREE, (empty)
    }
    i = 0 as libc::c_int;
    while i < nworkers {
    // 1670: nworkers: typeof(_73) = *mut {l95} i32
    // 1670: nworkers:   l95 = READ | UNIQUE | NON_NULL, (empty)
        let mut worker: *mut Worker = workers.offset(i as isize);
        // 1671: mut worker: typeof(_74) = *mut {l97} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 1671: mut worker:   l97 = READ | WRITE | NON_NULL, CELL
        // 1671: workers: typeof(_75) = *mut {l99} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 1671: workers:   l99 = READ | WRITE | OFFSET_ADD | OFFSET_SUB | NON_NULL, CELL
        // 1671: workers: typeof(_76) = *mut {l101} *mut {g197} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 1671: workers:   l101 = READ | UNIQUE | NON_NULL, (empty)
        // 1671: workers:   g197 = READ | WRITE | OFFSET_ADD | OFFSET_SUB | NON_NULL, CELL
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
    // 1682: clauses: typeof(_97) = *mut {l123} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
    // 1682: clauses:   l123 = READ | UNIQUE | NON_NULL, (empty)
        let fresh13 = j;
        j = j + 1;
        let ref mut fresh14 = *(clauses.start).offset(fresh13 as isize);
        // 1685: ref mut fresh14: typeof(_101) = &mut {l128} *mut {g267} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1685: ref mut fresh14:   l128 = READ | WRITE | UNIQUE | NON_NULL, FIXED
        // 1685: ref mut fresh14:   g267 = READ | FREE, FIXED
        // 1685: (clauses.start) ... size): typeof(_102) = *mut {l130} *mut {g267} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1685: (clauses.start) ... size):   l130 = READ | WRITE | UNIQUE | NON_NULL, (empty)
        // 1685: (clauses.start) ... size):   g267 = READ | FREE, FIXED
        // 1685: (clauses.start): typeof(_103) = *mut {l132} *mut {g267} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1685: (clauses.start):   l132 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | NON_NULL, (empty)
        // 1685: (clauses.start):   g267 = READ | FREE, FIXED
        // 1685: clauses: typeof(_104) = *mut {l134} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
        // 1685: clauses:   l134 = READ | UNIQUE | NON_NULL, (empty)
        // 1685: ref mut fresh14: typeof(_101 = &mut (*_102)) = &mut {l171} *mut {g267} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1685: ref mut fresh14:   l171 = READ | WRITE | UNIQUE | NON_NULL, (empty)
        // 1685: ref mut fresh14:   g267 = READ | FREE, FIXED
        *fresh14 = *(clauses.start).offset(l as isize);
        // 1686: *(clauses.start ... size): typeof(_107) = *mut {l138} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1686: *(clauses.start ... size):   l138 = READ | FREE, (empty)
        // 1686: (clauses.start) ... size): typeof(_108) = *mut {l140} *mut {g267} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1686: (clauses.start) ... size):   l140 = READ | UNIQUE | NON_NULL, (empty)
        // 1686: (clauses.start) ... size):   g267 = READ | FREE, FIXED
        // 1686: (clauses.start): typeof(_109) = *mut {l142} *mut {g267} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1686: (clauses.start):   l142 = READ | UNIQUE | OFFSET_ADD | OFFSET_SUB | NON_NULL, (empty)
        // 1686: (clauses.start):   g267 = READ | FREE, FIXED
        // 1686: clauses: typeof(_110) = *mut {l144} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
        // 1686: clauses:   l144 = READ | UNIQUE | NON_NULL, (empty)
        l += 1;
        l;
    }
    clauses.collected += k;
    // 1690: clauses: typeof(_119) = *mut {l154} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
    // 1690: clauses:   l154 = READ | WRITE | UNIQUE | NON_NULL, (empty)
    clauses.first = 0 as libc::c_int as libc::c_long;
    // 1691: clauses: typeof(_122) = *mut {l158} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
    // 1691: clauses:   l158 = READ | WRITE | UNIQUE | NON_NULL, (empty)
    clauses.num -= k;
    // 1692: clauses: typeof(_124) = *mut {l161} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
    // 1692: clauses:   l161 = READ | WRITE | UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn consumecls(
    mut voidptr: *mut libc::c_void,
    // 1695: mut voidptr: typeof(_1) = *mut {g27} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1695: mut voidptr:   g27 = UNIQUE | NON_NULL, FIXED
    mut cptr: *mut *mut libc::c_int,
    // 1696: mut cptr: typeof(_2) = *mut {g28} *mut {g29} i32
    // 1696: mut cptr:   g28 = UNIQUE | NON_NULL, FIXED
    // 1696: mut cptr:   g29 = UNIQUE | NON_NULL, FIXED
    mut glueptr: *mut libc::c_int,
    // 1697: mut glueptr: typeof(_3) = *mut {g30} i32
    // 1697: mut glueptr:   g30 = UNIQUE | NON_NULL, FIXED
) {
    let mut worker: *mut Worker = voidptr as *mut Worker;
    // 1699: mut worker: typeof(_4) = *mut {l4} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1699: mut worker:   l4 = UNIQUE | NON_NULL, (empty)
    // 1699: voidptr: typeof(_5) = *mut {l6} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1699: voidptr:   l6 = UNIQUE | NON_NULL, (empty)
    // 1699: voidptr as *mut ... orker: typeof(_4 = move _5 as *mut Worker (Misc)) = *mut {l256} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1699: voidptr as *mut ... orker:   l256 = UNIQUE | NON_NULL, (empty)
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    // 1700: worker: typeof(_9) = *mut {l11} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1700: worker:   l11 = UNIQUE | NON_NULL, (empty)
    // 1700: workers: typeof(_10) = *const {l13} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1700: workers:   l13 = UNIQUE | NON_NULL, (empty)
    // 1700: workers: typeof(_11) = *mut {l15} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1700: workers:   l15 = UNIQUE | NON_NULL, (empty)
    // 1700: workers: typeof(_12) = *mut {l17} *mut {l18} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1700: workers:   l17 = UNIQUE | NON_NULL, (empty)
    // 1700: workers:   l18 = UNIQUE | NON_NULL, (empty)
    // 1700: workers: typeof(_10 = move _11 as *const Worker (Pointer(MutToConstPointer))) = *const {l257} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1700: workers:   l257 = UNIQUE | NON_NULL, (empty)
    let mut res: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut newsize: libc::c_int = 0;
    let mut cls: *mut Cls = 0 as *mut Cls;
    // 1704: mut cls: typeof(_16) = *mut {l23} DefId(0:525 ~ plingeling[18f5]::Cls)
    // 1704: mut cls:   l23 = UNIQUE | NON_NULL, (empty)
    // 1704: 0 as *mut Cls: typeof(_16 = const 0_usize as *mut Cls (PointerFromExposedAddress)) = *mut {l258} DefId(0:525 ~ plingeling[18f5]::Cls)
    // 1704: 0 as *mut Cls:   l258 = UNIQUE | NON_NULL, (empty)
    if pthread_mutex_lock(&mut clsmutex) != 0 {
    // 1705: &mut clsmutex: typeof(_20) = *mut {l28} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1705: &mut clsmutex:   l28 = UNIQUE | NON_NULL, (empty)
    // 1705: &mut clsmutex: typeof(_21) = &mut {l30} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1705: &mut clsmutex:   l30 = UNIQUE | NON_NULL, (empty)
    // 1705: clsmutex: typeof(_22) = *mut {l32} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1705: clsmutex:   l32 = UNIQUE | NON_NULL, (empty)
    // 1705: &mut clsmutex: typeof(_20 = &raw mut (*_21)) = *mut {l260} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1705: &mut clsmutex:   l260 = UNIQUE | NON_NULL, (empty)
    // 1705: &mut clsmutex: typeof(_21 = &mut (*_22)) = &mut {l259} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1705: &mut clsmutex:   l259 = UNIQUE | NON_NULL, (empty)
        warn(b"failed to lock 'cls' mutex in 'consumecls'\0" as *const u8 as *const libc::c_char);
        // 1706: b"failed to loc ... _char: typeof(_24) = *const {l35} i8
        // 1706: b"failed to loc ... _char:   l35 = UNIQUE | NON_NULL, (empty)
        // 1706: b"failed to loc ... st u8: typeof(_25) = *const {l37} u8
        // 1706: b"failed to loc ... st u8:   l37 = UNIQUE | NON_NULL, (empty)
        // 1706: b"failed to loc ... s'\0": typeof(_26) = *const {l39} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
        // 1706: b"failed to loc ... s'\0":   l39 = UNIQUE | NON_NULL, (empty)
        // 1706: b"failed to loc ... s'\0": typeof(_27) = & {l41} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
        // 1706: b"failed to loc ... s'\0":   l41 = UNIQUE | NON_NULL, FIXED
        // 1706: b"failed to loc ... s'\0": typeof(_26 = &raw const (*_27)) = *const {l262} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
        // 1706: b"failed to loc ... s'\0":   l262 = UNIQUE | NON_NULL, (empty)
        // 1706: b"failed to loc ... st u8: typeof(_25 = move _26 as *const u8 (Pointer(ArrayToPointer))) = *const {l263} u8
        // 1706: b"failed to loc ... st u8:   l263 = UNIQUE | NON_NULL, (empty)
        // 1706: b"failed to loc ... _char: typeof(_24 = move _25 as *const i8 (Misc)) = *const {l264} i8
        // 1706: b"failed to loc ... _char:   l264 = UNIQUE | NON_NULL, (empty)
        // 1706: b"failed to loc ... s'\0": typeof(_27 = const b"failed to lock \'cls\' mutex in \'consumecls\'\x00") = & {l261} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
        // 1706: b"failed to loc ... s'\0":   l261 = UNIQUE | NON_NULL, (empty)
    }
    loop {
        if !((*worker).dead).is_null() {
        // 1709: ((*worker).dead): typeof(_33) = *mut {l48} DefId(0:525 ~ plingeling[18f5]::Cls)
        // 1709: ((*worker).dead):   l48 = UNIQUE | NON_NULL, (empty)
            deletecls_shim((*worker).dead);
            // 1710: (*worker).dead: typeof(_35) = *mut {l51} DefId(0:525 ~ plingeling[18f5]::Cls)
            // 1710: (*worker).dead:   l51 = UNIQUE | NON_NULL, (empty)
            (*worker).dead = 0 as *mut Cls;
            // 1711: (*worker).dead  ... t Cls: typeof(((*_4).9: *mut Cls) = const 0_usize as *mut Cls (PointerFromExposedAddress)) = *mut {l265} DefId(0:525 ~ plingeling[18f5]::Cls)
            // 1711: (*worker).dead  ... t Cls:   l265 = UNIQUE | NON_NULL, (empty)
        }
        if (*worker).clsimported == clauses.num {
        // 1713: clauses: typeof(_39) = *mut {l56} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
        // 1713: clauses:   l56 = UNIQUE | NON_NULL, (empty)
            msg(
                wid,
                3 as libc::c_int,
                b"all %d clauses already consumed\0" as *const u8 as *const libc::c_char,
                // 1717: b"all %d clause ... _char: typeof(_44) = *const {l62} i8
                // 1717: b"all %d clause ... _char:   l62 = UNIQUE | NON_NULL, (empty)
                // 1717: b"all %d clause ... st u8: typeof(_45) = *const {l64} u8
                // 1717: b"all %d clause ... st u8:   l64 = UNIQUE | NON_NULL, (empty)
                // 1717: b"all %d clause ... ed\0": typeof(_46) = *const {l66} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1717: b"all %d clause ... ed\0":   l66 = UNIQUE | NON_NULL, (empty)
                // 1717: b"all %d clause ... ed\0": typeof(_47) = & {l68} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1717: b"all %d clause ... ed\0":   l68 = UNIQUE | NON_NULL, FIXED
                // 1717: b"all %d clause ... st u8: typeof(_45 = move _46 as *const u8 (Pointer(ArrayToPointer))) = *const {l268} u8
                // 1717: b"all %d clause ... st u8:   l268 = UNIQUE | NON_NULL, (empty)
                // 1717: b"all %d clause ... ed\0": typeof(_47 = const b"all %d clauses already consumed\x00") = & {l266} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1717: b"all %d clause ... ed\0":   l266 = UNIQUE | NON_NULL, (empty)
                // 1717: b"all %d clause ... ed\0": typeof(_46 = &raw const (*_47)) = *const {l267} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1717: b"all %d clause ... ed\0":   l267 = UNIQUE | NON_NULL, (empty)
                // 1717: b"all %d clause ... _char: typeof(_44 = move _45 as *const i8 (Misc)) = *const {l269} i8
                // 1717: b"all %d clause ... _char:   l269 = UNIQUE | NON_NULL, (empty)
                clauses.num,
                // 1718: clauses: typeof(_49) = *mut {l71} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
                // 1718: clauses:   l71 = UNIQUE | NON_NULL, (empty)
            );
            *cptr = 0 as *mut libc::c_int;
            // 1720: *cptr = 0 as *m ... c_int: typeof((*_2) = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l270} i32
            // 1720: *cptr = 0 as *m ... c_int:   l270 = UNIQUE | NON_NULL, (empty)
            break;
        } else {
            let fresh15 = (*worker).clsimported;
            (*worker).clsimported = (*worker).clsimported + 1;
            res = fresh15 as libc::c_int;
            cls = *(clauses.start).offset(res as isize);
            // 1726: *(clauses.start ... size): typeof(_55) = *mut {l78} DefId(0:525 ~ plingeling[18f5]::Cls)
            // 1726: *(clauses.start ... size):   l78 = UNIQUE | NON_NULL, (empty)
            // 1726: (clauses.start) ... size): typeof(_56) = *mut {l80} *mut {l81} DefId(0:525 ~ plingeling[18f5]::Cls)
            // 1726: (clauses.start) ... size):   l80 = UNIQUE | NON_NULL, (empty)
            // 1726: (clauses.start) ... size):   l81 = UNIQUE | NON_NULL, (empty)
            // 1726: (clauses.start): typeof(_57) = *mut {l83} *mut {l84} DefId(0:525 ~ plingeling[18f5]::Cls)
            // 1726: (clauses.start):   l83 = UNIQUE | NON_NULL, (empty)
            // 1726: (clauses.start):   l84 = UNIQUE | NON_NULL, (empty)
            // 1726: clauses: typeof(_58) = *mut {l86} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
            // 1726: clauses:   l86 = UNIQUE | NON_NULL, (empty)
            if cls.is_null() {
            // 1727: cls: typeof(_63) = *mut {l92} DefId(0:525 ~ plingeling[18f5]::Cls)
            // 1727: cls:   l92 = UNIQUE | NON_NULL, (empty)
                continue;
            }
            (*cls).count += 1;
            (*cls).count;
            if (*cls).count + leavebehind >= nconsumers {
            // 1732: leavebehind: typeof(_72) = *mut {l102} i32
            // 1732: leavebehind:   l102 = UNIQUE | NON_NULL, (empty)
            // 1732: nconsumers: typeof(_75) = *mut {l106} i32
            // 1732: nconsumers:   l106 = UNIQUE | NON_NULL, (empty)
                let ref mut fresh16 = *(clauses.start).offset(res as isize);
                // 1733: ref mut fresh16: typeof(_76) = &mut {l108} *mut {l109} DefId(0:525 ~ plingeling[18f5]::Cls)
                // 1733: ref mut fresh16:   l108 = UNIQUE | NON_NULL, FIXED
                // 1733: ref mut fresh16:   l109 = UNIQUE | NON_NULL, (empty)
                // 1733: (clauses.start) ... size): typeof(_77) = *mut {l111} *mut {l112} DefId(0:525 ~ plingeling[18f5]::Cls)
                // 1733: (clauses.start) ... size):   l111 = UNIQUE | NON_NULL, (empty)
                // 1733: (clauses.start) ... size):   l112 = UNIQUE | NON_NULL, (empty)
                // 1733: (clauses.start): typeof(_78) = *mut {l114} *mut {l115} DefId(0:525 ~ plingeling[18f5]::Cls)
                // 1733: (clauses.start):   l114 = UNIQUE | NON_NULL, (empty)
                // 1733: (clauses.start):   l115 = UNIQUE | NON_NULL, (empty)
                // 1733: clauses: typeof(_79) = *mut {l117} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
                // 1733: clauses:   l117 = UNIQUE | NON_NULL, (empty)
                // 1733: ref mut fresh16: typeof(_76 = &mut (*_77)) = &mut {l271} *mut {l272} DefId(0:525 ~ plingeling[18f5]::Cls)
                // 1733: ref mut fresh16:   l271 = UNIQUE | NON_NULL, (empty)
                // 1733: ref mut fresh16:   l272 = UNIQUE | NON_NULL, (empty)
                *fresh16 = 0 as *mut Cls;
                // 1734: *fresh16 = 0 as ... t Cls: typeof((*_76) = const 0_usize as *mut Cls (PointerFromExposedAddress)) = *mut {l273} DefId(0:525 ~ plingeling[18f5]::Cls)
                // 1734: *fresh16 = 0 as ... t Cls:   l273 = UNIQUE | NON_NULL, (empty)
                (*worker).dead = cls;
                // 1735: cls: typeof(_82) = *mut {l121} DefId(0:525 ~ plingeling[18f5]::Cls)
                // 1735: cls:   l121 = UNIQUE | NON_NULL, (empty)
                clauses.first += 1;
                // 1736: clauses: typeof(_83) = *mut {l123} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
                // 1736: clauses:   l123 = UNIQUE | NON_NULL, (empty)
                clauses.first;
                // 1737: clauses: typeof(_86) = *mut {l127} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
                // 1737: clauses:   l127 = UNIQUE | NON_NULL, (empty)
                if clauses.num > 10000 as libc::c_int as libc::c_long
                // 1738: clauses: typeof(_90) = *mut {l132} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
                // 1738: clauses:   l132 = UNIQUE | NON_NULL, (empty)
                    && clauses.first > clauses.num / (nconsumers + 1 as libc::c_int) as libc::c_long
                    // 1739: clauses: typeof(_95) = *mut {l138} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
                    // 1739: clauses:   l138 = UNIQUE | NON_NULL, (empty)
                    // 1739: clauses: typeof(_98) = *mut {l142} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
                    // 1739: clauses:   l142 = UNIQUE | NON_NULL, (empty)
                    // 1739: nconsumers: typeof(_102) = *mut {l147} i32
                    // 1739: nconsumers:   l147 = UNIQUE | NON_NULL, (empty)
                {
                    gcls();
                }
            }
            if (*cls).wid == wid {
                continue;
            }
            len = lencls(((*cls).lits).as_mut_ptr());
            // 1747: ((*cls).lits).a ... ptr(): typeof(_116) = *mut {l162} i32
            // 1747: ((*cls).lits).a ... ptr():   l162 = UNIQUE | NON_NULL, (empty)
            // 1747: ((*cls).lits).a ... ptr(): typeof(_117) = &mut {l164} [i32]
            // 1747: ((*cls).lits).a ... ptr():   l164 = UNIQUE | NON_NULL, FIXED
            // 1747: ((*cls).lits).a ... ptr(): typeof(_118) = &mut {l166} [i32; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
            // 1747: ((*cls).lits).a ... ptr():   l166 = UNIQUE | NON_NULL, (empty)
            // 1747: ((*cls).lits).a ... ptr(): typeof(_117 = move _118 as &mut [i32] (Pointer(Unsize))) = &mut {l275} [i32]
            // 1747: ((*cls).lits).a ... ptr():   l275 = UNIQUE | NON_NULL, FIXED
            // 1747: ((*cls).lits).a ... ptr(): typeof(_118 = &mut ((*_16).3: [i32; 1])) = &mut {l274} [i32; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
            // 1747: ((*cls).lits).a ... ptr():   l274 = UNIQUE | NON_NULL, (empty)
            if len + 1 as libc::c_int >= (*worker).szcls {
                newsize = 2 as libc::c_int * (len + 1 as libc::c_int);
                (*worker).cls = resize(
                // 1750: resize( 0 as *m ... g), ): typeof(_132) = *mut {l181} DefId(2:5583 ~ core[480a]::ffi::c_void)
                // 1750: resize( 0 as *m ... g), ):   l181 = UNIQUE | NON_NULL, (empty)
                // 1750: (*worker).cls = ... c_int: typeof(((*_4).6: *mut i32) = move _132 as *mut i32 (Misc)) = *mut {l278} i32
                // 1750: (*worker).cls = ... c_int:   l278 = UNIQUE | NON_NULL, (empty)
                    0 as *mut libc::c_void,
                    // 1751: 0 as *mut libc: ... _void: typeof(_133) = *mut {l183} DefId(2:5583 ~ core[480a]::ffi::c_void)
                    // 1751: 0 as *mut libc: ... _void:   l183 = UNIQUE | NON_NULL, (empty)
                    // 1751: 0 as *mut libc: ... _void: typeof(_133 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l276} DefId(2:5583 ~ core[480a]::ffi::c_void)
                    // 1751: 0 as *mut libc: ... _void:   l276 = UNIQUE | NON_NULL, (empty)
                    (*worker).cls as *mut libc::c_void,
                    // 1752: (*worker).cls a ... _void: typeof(_134) = *mut {l185} DefId(2:5583 ~ core[480a]::ffi::c_void)
                    // 1752: (*worker).cls a ... _void:   l185 = UNIQUE | NON_NULL, (empty)
                    // 1752: (*worker).cls: typeof(_135) = *mut {l187} i32
                    // 1752: (*worker).cls:   l187 = UNIQUE | NON_NULL, (empty)
                    // 1752: (*worker).cls a ... _void: typeof(_134 = move _135 as *mut libc::c_void (Misc)) = *mut {l277} DefId(2:5583 ~ core[480a]::ffi::c_void)
                    // 1752: (*worker).cls a ... _void:   l277 = UNIQUE | NON_NULL, (empty)
                    ((*worker).szcls as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
                    (newsize as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
                ) as *mut libc::c_int;
                (*worker).szcls = newsize;
            }
            memcpy(
            // 1760: memcpy( (*worke ... g), ): typeof(_147) = *mut {l200} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1760: memcpy( (*worke ... g), ):   l200 = UNIQUE | NON_NULL, (empty)
                (*worker).cls as *mut libc::c_void,
                // 1761: (*worker).cls a ... _void: typeof(_148) = *mut {l202} DefId(2:5583 ~ core[480a]::ffi::c_void)
                // 1761: (*worker).cls a ... _void:   l202 = UNIQUE | NON_NULL, (empty)
                // 1761: (*worker).cls: typeof(_149) = *mut {l204} i32
                // 1761: (*worker).cls:   l204 = UNIQUE | NON_NULL, (empty)
                // 1761: (*worker).cls a ... _void: typeof(_148 = move _149 as *mut libc::c_void (Misc)) = *mut {l279} DefId(2:5583 ~ core[480a]::ffi::c_void)
                // 1761: (*worker).cls a ... _void:   l279 = UNIQUE | NON_NULL, (empty)
                ((*cls).lits).as_mut_ptr() as *const libc::c_void,
                // 1762: ((*cls).lits).a ... _void: typeof(_150) = *const {l206} DefId(2:5583 ~ core[480a]::ffi::c_void)
                // 1762: ((*cls).lits).a ... _void:   l206 = UNIQUE | NON_NULL, (empty)
                // 1762: ((*cls).lits).a ... ptr(): typeof(_151) = *mut {l208} i32
                // 1762: ((*cls).lits).a ... ptr():   l208 = UNIQUE | NON_NULL, (empty)
                // 1762: ((*cls).lits).a ... ptr(): typeof(_152) = &mut {l210} [i32]
                // 1762: ((*cls).lits).a ... ptr():   l210 = UNIQUE | NON_NULL, FIXED
                // 1762: ((*cls).lits).a ... ptr(): typeof(_153) = &mut {l212} [i32; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
                // 1762: ((*cls).lits).a ... ptr():   l212 = UNIQUE | NON_NULL, (empty)
                // 1762: ((*cls).lits).a ... _void: typeof(_150 = move _151 as *const libc::c_void (Misc)) = *const {l282} DefId(2:5583 ~ core[480a]::ffi::c_void)
                // 1762: ((*cls).lits).a ... _void:   l282 = UNIQUE | NON_NULL, (empty)
                // 1762: ((*cls).lits).a ... ptr(): typeof(_152 = move _153 as &mut [i32] (Pointer(Unsize))) = &mut {l281} [i32]
                // 1762: ((*cls).lits).a ... ptr():   l281 = UNIQUE | NON_NULL, FIXED
                // 1762: ((*cls).lits).a ... ptr(): typeof(_153 = &mut ((*_16).3: [i32; 1])) = &mut {l280} [i32; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
                // 1762: ((*cls).lits).a ... ptr():   l280 = UNIQUE | NON_NULL, (empty)
                ((len + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            *cptr = (*worker).cls;
            // 1766: (*worker).cls: typeof(_162) = *mut {l222} i32
            // 1766: (*worker).cls:   l222 = UNIQUE | NON_NULL, (empty)
            *glueptr = (*cls).glue;
            msg(
                wid,
                3 as libc::c_int,
                b"consuming glue %d length %d clause %d\0" as *const u8 as *const libc::c_char,
                // 1771: b"consuming glu ... _char: typeof(_167) = *const {l228} i8
                // 1771: b"consuming glu ... _char:   l228 = UNIQUE | NON_NULL, (empty)
                // 1771: b"consuming glu ... st u8: typeof(_168) = *const {l230} u8
                // 1771: b"consuming glu ... st u8:   l230 = UNIQUE | NON_NULL, (empty)
                // 1771: b"consuming glu ... %d\0": typeof(_169) = *const {l232} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                // 1771: b"consuming glu ... %d\0":   l232 = UNIQUE | NON_NULL, (empty)
                // 1771: b"consuming glu ... %d\0": typeof(_170) = & {l234} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                // 1771: b"consuming glu ... %d\0":   l234 = UNIQUE | NON_NULL, FIXED
                // 1771: b"consuming glu ... %d\0": typeof(_170 = const b"consuming glue %d length %d clause %d\x00") = & {l283} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                // 1771: b"consuming glu ... %d\0":   l283 = UNIQUE | NON_NULL, (empty)
                // 1771: b"consuming glu ... %d\0": typeof(_169 = &raw const (*_170)) = *const {l284} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                // 1771: b"consuming glu ... %d\0":   l284 = UNIQUE | NON_NULL, (empty)
                // 1771: b"consuming glu ... st u8: typeof(_168 = move _169 as *const u8 (Pointer(ArrayToPointer))) = *const {l285} u8
                // 1771: b"consuming glu ... st u8:   l285 = UNIQUE | NON_NULL, (empty)
                // 1771: b"consuming glu ... _char: typeof(_167 = move _168 as *const i8 (Misc)) = *const {l286} i8
                // 1771: b"consuming glu ... _char:   l286 = UNIQUE | NON_NULL, (empty)
                *glueptr,
                len,
                res,
            );
            break;
        }
    }
    if pthread_mutex_unlock(&mut clsmutex) != 0 {
    // 1779: &mut clsmutex: typeof(_176) = *mut {l241} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1779: &mut clsmutex:   l241 = UNIQUE | NON_NULL, (empty)
    // 1779: &mut clsmutex: typeof(_177) = &mut {l243} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1779: &mut clsmutex:   l243 = UNIQUE | NON_NULL, (empty)
    // 1779: clsmutex: typeof(_178) = *mut {l245} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1779: clsmutex:   l245 = UNIQUE | NON_NULL, (empty)
    // 1779: &mut clsmutex: typeof(_176 = &raw mut (*_177)) = *mut {l288} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1779: &mut clsmutex:   l288 = UNIQUE | NON_NULL, (empty)
    // 1779: &mut clsmutex: typeof(_177 = &mut (*_178)) = &mut {l287} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1779: &mut clsmutex:   l287 = UNIQUE | NON_NULL, (empty)
        warn(b"failed to unlock 'cls' mutex in 'consumecls'\0" as *const u8 as *const libc::c_char);
        // 1780: b"failed to unl ... _char: typeof(_180) = *const {l248} i8
        // 1780: b"failed to unl ... _char:   l248 = UNIQUE | NON_NULL, (empty)
        // 1780: b"failed to unl ... st u8: typeof(_181) = *const {l250} u8
        // 1780: b"failed to unl ... st u8:   l250 = UNIQUE | NON_NULL, (empty)
        // 1780: b"failed to unl ... s'\0": typeof(_182) = *const {l252} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
        // 1780: b"failed to unl ... s'\0":   l252 = UNIQUE | NON_NULL, (empty)
        // 1780: b"failed to unl ... s'\0": typeof(_183) = & {l254} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
        // 1780: b"failed to unl ... s'\0":   l254 = UNIQUE | NON_NULL, FIXED
        // 1780: b"failed to unl ... s'\0": typeof(_182 = &raw const (*_183)) = *const {l290} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
        // 1780: b"failed to unl ... s'\0":   l290 = UNIQUE | NON_NULL, (empty)
        // 1780: b"failed to unl ... _char: typeof(_180 = move _181 as *const i8 (Misc)) = *const {l292} i8
        // 1780: b"failed to unl ... _char:   l292 = UNIQUE | NON_NULL, (empty)
        // 1780: b"failed to unl ... s'\0": typeof(_183 = const b"failed to unlock \'cls\' mutex in \'consumecls\'\x00") = & {l289} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
        // 1780: b"failed to unl ... s'\0":   l289 = UNIQUE | NON_NULL, (empty)
        // 1780: b"failed to unl ... st u8: typeof(_181 = move _182 as *const u8 (Pointer(ArrayToPointer))) = *const {l291} u8
        // 1780: b"failed to unl ... st u8:   l291 = UNIQUE | NON_NULL, (empty)
    }
}
unsafe extern "C" fn consumedcls(mut voidptr: *mut libc::c_void, mut consumed: libc::c_int) {
// 1783: mut voidptr: typeof(_1) = *mut {g31} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 1783: mut voidptr:   g31 = UNIQUE | NON_NULL, FIXED
    let mut worker: *mut Worker = voidptr as *mut Worker;
    // 1784: mut worker: typeof(_3) = *mut {l3} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1784: mut worker:   l3 = UNIQUE | NON_NULL, (empty)
    // 1784: voidptr: typeof(_4) = *mut {l5} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1784: voidptr:   l5 = UNIQUE | NON_NULL, (empty)
    // 1784: voidptr as *mut ... orker: typeof(_3 = move _4 as *mut Worker (Misc)) = *mut {l35} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1784: voidptr as *mut ... orker:   l35 = UNIQUE | NON_NULL, (empty)
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    // 1785: worker: typeof(_8) = *mut {l10} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1785: worker:   l10 = UNIQUE | NON_NULL, (empty)
    // 1785: workers: typeof(_9) = *const {l12} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1785: workers:   l12 = UNIQUE | NON_NULL, (empty)
    // 1785: workers: typeof(_10) = *mut {l14} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1785: workers:   l14 = UNIQUE | NON_NULL, (empty)
    // 1785: workers: typeof(_11) = *mut {l16} *mut {l17} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1785: workers:   l16 = UNIQUE | NON_NULL, (empty)
    // 1785: workers:   l17 = UNIQUE | NON_NULL, (empty)
    // 1785: workers: typeof(_9 = move _10 as *const Worker (Pointer(MutToConstPointer))) = *const {l36} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1785: workers:   l36 = UNIQUE | NON_NULL, (empty)
    (*worker).stats.cls.consumed += consumed;
    (*worker).stats.consumed += consumed;
    msg(
        wid,
        3 as libc::c_int,
        b"consuming %d clause\0" as *const u8 as *const libc::c_char,
        // 1791: b"consuming %d  ... _char: typeof(_19) = *const {l26} i8
        // 1791: b"consuming %d  ... _char:   l26 = UNIQUE | NON_NULL, (empty)
        // 1791: b"consuming %d  ... st u8: typeof(_20) = *const {l28} u8
        // 1791: b"consuming %d  ... st u8:   l28 = UNIQUE | NON_NULL, (empty)
        // 1791: b"consuming %d  ... se\0": typeof(_21) = *const {l30} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
        // 1791: b"consuming %d  ... se\0":   l30 = UNIQUE | NON_NULL, (empty)
        // 1791: b"consuming %d  ... se\0": typeof(_22) = & {l32} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
        // 1791: b"consuming %d  ... se\0":   l32 = UNIQUE | NON_NULL, FIXED
        // 1791: b"consuming %d  ... se\0": typeof(_22 = const b"consuming %d clause\x00") = & {l37} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
        // 1791: b"consuming %d  ... se\0":   l37 = UNIQUE | NON_NULL, (empty)
        // 1791: b"consuming %d  ... se\0": typeof(_21 = &raw const (*_22)) = *const {l38} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
        // 1791: b"consuming %d  ... se\0":   l38 = UNIQUE | NON_NULL, (empty)
        // 1791: b"consuming %d  ... _char: typeof(_19 = move _20 as *const i8 (Misc)) = *const {l40} i8
        // 1791: b"consuming %d  ... _char:   l40 = UNIQUE | NON_NULL, (empty)
        // 1791: b"consuming %d  ... st u8: typeof(_20 = move _21 as *const u8 (Pointer(ArrayToPointer))) = *const {l39} u8
        // 1791: b"consuming %d  ... st u8:   l39 = UNIQUE | NON_NULL, (empty)
        consumed,
    );
}
unsafe extern "C" fn lockrepr(mut voidptr: *mut libc::c_void) -> *mut libc::c_int {
// 1795: *mut libc::c_int: typeof(_0) = *mut {g33} i32
// 1795: *mut libc::c_int:   g33 = UNIQUE | NON_NULL, FIXED
// 1795: mut voidptr: typeof(_1) = *mut {g32} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 1795: mut voidptr:   g32 = UNIQUE | NON_NULL, FIXED
    let mut worker: *mut Worker = voidptr as *mut Worker;
    // 1796: mut worker: typeof(_3) = *mut {l3} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1796: mut worker:   l3 = UNIQUE | NON_NULL, (empty)
    // 1796: voidptr: typeof(_4) = *mut {l5} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1796: voidptr:   l5 = UNIQUE | NON_NULL, (empty)
    // 1796: voidptr as *mut ... orker: typeof(_3 = move _4 as *mut Worker (Misc)) = *mut {l57} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1796: voidptr as *mut ... orker:   l57 = UNIQUE | NON_NULL, (empty)
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    // 1797: worker: typeof(_8) = *mut {l10} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1797: worker:   l10 = UNIQUE | NON_NULL, (empty)
    // 1797: workers: typeof(_9) = *const {l12} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1797: workers:   l12 = UNIQUE | NON_NULL, (empty)
    // 1797: workers: typeof(_10) = *mut {l14} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1797: workers:   l14 = UNIQUE | NON_NULL, (empty)
    // 1797: workers: typeof(_11) = *mut {l16} *mut {l17} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1797: workers:   l16 = UNIQUE | NON_NULL, (empty)
    // 1797: workers:   l17 = UNIQUE | NON_NULL, (empty)
    // 1797: workers: typeof(_9 = move _10 as *const Worker (Pointer(MutToConstPointer))) = *const {l58} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1797: workers:   l58 = UNIQUE | NON_NULL, (empty)
    if pthread_mutex_lock(&mut reprmutex) != 0 {
    // 1798: &mut reprmutex: typeof(_15) = *mut {l22} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1798: &mut reprmutex:   l22 = UNIQUE | NON_NULL, (empty)
    // 1798: &mut reprmutex: typeof(_16) = &mut {l24} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1798: &mut reprmutex:   l24 = UNIQUE | NON_NULL, (empty)
    // 1798: reprmutex: typeof(_17) = *mut {l26} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1798: reprmutex:   l26 = UNIQUE | NON_NULL, (empty)
    // 1798: &mut reprmutex: typeof(_16 = &mut (*_17)) = &mut {l59} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1798: &mut reprmutex:   l59 = UNIQUE | NON_NULL, (empty)
    // 1798: &mut reprmutex: typeof(_15 = &raw mut (*_16)) = *mut {l60} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1798: &mut reprmutex:   l60 = UNIQUE | NON_NULL, (empty)
        warn(b"failed to lock 'repr' mutex\0" as *const u8 as *const libc::c_char);
        // 1799: b"failed to loc ... _char: typeof(_19) = *const {l29} i8
        // 1799: b"failed to loc ... _char:   l29 = UNIQUE | NON_NULL, (empty)
        // 1799: b"failed to loc ... st u8: typeof(_20) = *const {l31} u8
        // 1799: b"failed to loc ... st u8:   l31 = UNIQUE | NON_NULL, (empty)
        // 1799: b"failed to loc ... ex\0": typeof(_21) = *const {l33} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
        // 1799: b"failed to loc ... ex\0":   l33 = UNIQUE | NON_NULL, (empty)
        // 1799: b"failed to loc ... ex\0": typeof(_22) = & {l35} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
        // 1799: b"failed to loc ... ex\0":   l35 = UNIQUE | NON_NULL, FIXED
        // 1799: b"failed to loc ... ex\0": typeof(_22 = const b"failed to lock \'repr\' mutex\x00") = & {l61} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
        // 1799: b"failed to loc ... ex\0":   l61 = UNIQUE | NON_NULL, (empty)
        // 1799: b"failed to loc ... st u8: typeof(_20 = move _21 as *const u8 (Pointer(ArrayToPointer))) = *const {l63} u8
        // 1799: b"failed to loc ... st u8:   l63 = UNIQUE | NON_NULL, (empty)
        // 1799: b"failed to loc ... ex\0": typeof(_21 = &raw const (*_22)) = *const {l62} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
        // 1799: b"failed to loc ... ex\0":   l62 = UNIQUE | NON_NULL, (empty)
        // 1799: b"failed to loc ... _char: typeof(_19 = move _20 as *const i8 (Misc)) = *const {l64} i8
        // 1799: b"failed to loc ... _char:   l64 = UNIQUE | NON_NULL, (empty)
    }
    msg(
        wid,
        3 as libc::c_int,
        b"starting equivalences synchronization\0" as *const u8 as *const libc::c_char,
        // 1804: b"starting equi ... _char: typeof(_26) = *const {l40} i8
        // 1804: b"starting equi ... _char:   l40 = UNIQUE | NON_NULL, (empty)
        // 1804: b"starting equi ... st u8: typeof(_27) = *const {l42} u8
        // 1804: b"starting equi ... st u8:   l42 = UNIQUE | NON_NULL, (empty)
        // 1804: b"starting equi ... on\0": typeof(_28) = *const {l44} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
        // 1804: b"starting equi ... on\0":   l44 = UNIQUE | NON_NULL, (empty)
        // 1804: b"starting equi ... on\0": typeof(_29) = & {l46} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
        // 1804: b"starting equi ... on\0":   l46 = UNIQUE | NON_NULL, FIXED
        // 1804: b"starting equi ... on\0": typeof(_29 = const b"starting equivalences synchronization\x00") = & {l65} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
        // 1804: b"starting equi ... on\0":   l65 = UNIQUE | NON_NULL, (empty)
        // 1804: b"starting equi ... _char: typeof(_26 = move _27 as *const i8 (Misc)) = *const {l68} i8
        // 1804: b"starting equi ... _char:   l68 = UNIQUE | NON_NULL, (empty)
        // 1804: b"starting equi ... on\0": typeof(_28 = &raw const (*_29)) = *const {l66} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
        // 1804: b"starting equi ... on\0":   l66 = UNIQUE | NON_NULL, (empty)
        // 1804: b"starting equi ... st u8: typeof(_27 = move _28 as *const u8 (Pointer(ArrayToPointer))) = *const {l67} u8
        // 1804: b"starting equi ... st u8:   l67 = UNIQUE | NON_NULL, (empty)
    );
    syncs.eqs += 1;
    // 1806: syncs: typeof(_30) = *mut {l48} DefId(0:590 ~ plingeling[18f5]::C2RustUnnamed_7)
    // 1806: syncs:   l48 = UNIQUE | NON_NULL, (empty)
    syncs.eqs;
    // 1807: syncs: typeof(_33) = *mut {l52} DefId(0:590 ~ plingeling[18f5]::C2RustUnnamed_7)
    // 1807: syncs:   l52 = UNIQUE | NON_NULL, (empty)
    return repr;
    // 1808: repr: typeof(_34) = *mut {l54} *mut {l55} i32
    // 1808: repr:   l54 = UNIQUE | NON_NULL, (empty)
    // 1808: repr:   l55 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn unlockrepr(
    mut voidptr: *mut libc::c_void,
    // 1811: mut voidptr: typeof(_1) = *mut {g34} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1811: mut voidptr:   g34 = UNIQUE | NON_NULL, FIXED
    mut consumed: libc::c_int,
    mut produced: libc::c_int,
) {
    let mut worker: *mut Worker = voidptr as *mut Worker;
    // 1815: mut worker: typeof(_4) = *mut {l4} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1815: mut worker:   l4 = UNIQUE | NON_NULL, (empty)
    // 1815: voidptr: typeof(_5) = *mut {l6} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1815: voidptr:   l6 = UNIQUE | NON_NULL, (empty)
    // 1815: voidptr as *mut ... orker: typeof(_4 = move _5 as *mut Worker (Misc)) = *mut {l62} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1815: voidptr as *mut ... orker:   l62 = UNIQUE | NON_NULL, (empty)
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    // 1816: worker: typeof(_9) = *mut {l11} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1816: worker:   l11 = UNIQUE | NON_NULL, (empty)
    // 1816: workers: typeof(_10) = *const {l13} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1816: workers:   l13 = UNIQUE | NON_NULL, (empty)
    // 1816: workers: typeof(_11) = *mut {l15} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1816: workers:   l15 = UNIQUE | NON_NULL, (empty)
    // 1816: workers: typeof(_12) = *mut {l17} *mut {l18} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1816: workers:   l17 = UNIQUE | NON_NULL, (empty)
    // 1816: workers:   l18 = UNIQUE | NON_NULL, (empty)
    // 1816: workers: typeof(_10 = move _11 as *const Worker (Pointer(MutToConstPointer))) = *const {l63} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1816: workers:   l63 = UNIQUE | NON_NULL, (empty)
    msg(
        wid,
        3 as libc::c_int,
        b"finished equivalences synchronization: %d consumed, %d produced\0" as *const u8
        // 1820: b"finished equi ... _char: typeof(_16) = *const {l23} i8
        // 1820: b"finished equi ... _char:   l23 = UNIQUE | NON_NULL, (empty)
        // 1820: b"finished equi ... st u8: typeof(_17) = *const {l25} u8
        // 1820: b"finished equi ... st u8:   l25 = UNIQUE | NON_NULL, (empty)
        // 1820: b"finished equi ... ed\0": typeof(_18) = *const {l27} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000040)) }]
        // 1820: b"finished equi ... ed\0":   l27 = UNIQUE | NON_NULL, (empty)
        // 1820: b"finished equi ... ed\0": typeof(_19) = & {l29} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000040)) }]
        // 1820: b"finished equi ... ed\0":   l29 = UNIQUE | NON_NULL, FIXED
        // 1820: b"finished equi ... st u8: typeof(_17 = move _18 as *const u8 (Pointer(ArrayToPointer))) = *const {l66} u8
        // 1820: b"finished equi ... st u8:   l66 = UNIQUE | NON_NULL, (empty)
        // 1820: b"finished equi ... ed\0": typeof(_19 = const b"finished equivalences synchronization: %d consumed, %d produced\x00") = & {l64} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000040)) }]
        // 1820: b"finished equi ... ed\0":   l64 = UNIQUE | NON_NULL, (empty)
        // 1820: b"finished equi ... _char: typeof(_16 = move _17 as *const i8 (Misc)) = *const {l67} i8
        // 1820: b"finished equi ... _char:   l67 = UNIQUE | NON_NULL, (empty)
        // 1820: b"finished equi ... ed\0": typeof(_18 = &raw const (*_19)) = *const {l65} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000040)) }]
        // 1820: b"finished equi ... ed\0":   l65 = UNIQUE | NON_NULL, (empty)
            as *const libc::c_char,
        consumed,
        produced,
    );
    (*worker).stats.eqs.consumed += consumed;
    (*worker).stats.eqs.produced += produced;
    (*worker).stats.consumed += consumed;
    (*worker).stats.produced += produced;
    eqs += produced;
    // 1829: eqs: typeof(_31) = *mut {l42} i32
    // 1829: eqs:   l42 = UNIQUE | NON_NULL, (empty)
    if pthread_mutex_unlock(&mut reprmutex) != 0 {
    // 1830: &mut reprmutex: typeof(_35) = *mut {l47} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1830: &mut reprmutex:   l47 = UNIQUE | NON_NULL, (empty)
    // 1830: &mut reprmutex: typeof(_36) = &mut {l49} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1830: &mut reprmutex:   l49 = UNIQUE | NON_NULL, (empty)
    // 1830: reprmutex: typeof(_37) = *mut {l51} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1830: reprmutex:   l51 = UNIQUE | NON_NULL, (empty)
    // 1830: &mut reprmutex: typeof(_36 = &mut (*_37)) = &mut {l68} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1830: &mut reprmutex:   l68 = UNIQUE | NON_NULL, (empty)
    // 1830: &mut reprmutex: typeof(_35 = &raw mut (*_36)) = *mut {l69} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1830: &mut reprmutex:   l69 = UNIQUE | NON_NULL, (empty)
        warn(b"failed to unlock 'repr' mutex\0" as *const u8 as *const libc::c_char);
        // 1831: b"failed to unl ... _char: typeof(_39) = *const {l54} i8
        // 1831: b"failed to unl ... _char:   l54 = UNIQUE | NON_NULL, (empty)
        // 1831: b"failed to unl ... st u8: typeof(_40) = *const {l56} u8
        // 1831: b"failed to unl ... st u8:   l56 = UNIQUE | NON_NULL, (empty)
        // 1831: b"failed to unl ... ex\0": typeof(_41) = *const {l58} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
        // 1831: b"failed to unl ... ex\0":   l58 = UNIQUE | NON_NULL, (empty)
        // 1831: b"failed to unl ... ex\0": typeof(_42) = & {l60} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
        // 1831: b"failed to unl ... ex\0":   l60 = UNIQUE | NON_NULL, FIXED
        // 1831: b"failed to unl ... ex\0": typeof(_42 = const b"failed to unlock \'repr\' mutex\x00") = & {l70} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
        // 1831: b"failed to unl ... ex\0":   l70 = UNIQUE | NON_NULL, (empty)
        // 1831: b"failed to unl ... ex\0": typeof(_41 = &raw const (*_42)) = *const {l71} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
        // 1831: b"failed to unl ... ex\0":   l71 = UNIQUE | NON_NULL, (empty)
        // 1831: b"failed to unl ... st u8: typeof(_40 = move _41 as *const u8 (Pointer(ArrayToPointer))) = *const {l72} u8
        // 1831: b"failed to unl ... st u8:   l72 = UNIQUE | NON_NULL, (empty)
        // 1831: b"failed to unl ... _char: typeof(_39 = move _40 as *const i8 (Misc)) = *const {l73} i8
        // 1831: b"failed to unl ... _char:   l73 = UNIQUE | NON_NULL, (empty)
    }
}
unsafe extern "C" fn msglock(mut voidptr: *mut libc::c_void) {
// 1834: mut voidptr: typeof(_1) = *mut {g35} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 1834: mut voidptr:   g35 = UNIQUE | NON_NULL, FIXED
    pthread_mutex_lock(&mut msgmutex);
    // 1835: &mut msgmutex: typeof(_3) = *mut {l3} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1835: &mut msgmutex:   l3 = UNIQUE | NON_NULL, (empty)
    // 1835: &mut msgmutex: typeof(_4) = &mut {l5} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1835: &mut msgmutex:   l5 = UNIQUE | NON_NULL, (empty)
    // 1835: msgmutex: typeof(_5) = *mut {l7} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1835: msgmutex:   l7 = UNIQUE | NON_NULL, (empty)
    // 1835: &mut msgmutex: typeof(_4 = &mut (*_5)) = &mut {l9} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1835: &mut msgmutex:   l9 = UNIQUE | NON_NULL, (empty)
    // 1835: &mut msgmutex: typeof(_3 = &raw mut (*_4)) = *mut {l10} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1835: &mut msgmutex:   l10 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn msgunlock(mut voidptr: *mut libc::c_void) {
// 1837: mut voidptr: typeof(_1) = *mut {g36} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 1837: mut voidptr:   g36 = UNIQUE | NON_NULL, FIXED
    pthread_mutex_unlock(&mut msgmutex);
    // 1838: &mut msgmutex: typeof(_3) = *mut {l3} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1838: &mut msgmutex:   l3 = UNIQUE | NON_NULL, (empty)
    // 1838: &mut msgmutex: typeof(_4) = &mut {l5} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1838: &mut msgmutex:   l5 = UNIQUE | NON_NULL, (empty)
    // 1838: msgmutex: typeof(_5) = *mut {l7} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1838: msgmutex:   l7 = UNIQUE | NON_NULL, (empty)
    // 1838: &mut msgmutex: typeof(_3 = &raw mut (*_4)) = *mut {l10} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1838: &mut msgmutex:   l10 = UNIQUE | NON_NULL, (empty)
    // 1838: &mut msgmutex: typeof(_4 = &mut (*_5)) = &mut {l9} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1838: &mut msgmutex:   l9 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn work(mut voidptr: *mut libc::c_void) -> *mut libc::c_void {
// 1840: *mut libc::c_void: typeof(_0) = *mut {g38} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 1840: *mut libc::c_void:   g38 = UNIQUE | NON_NULL, FIXED
// 1840: mut voidptr: typeof(_1) = *mut {g37} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 1840: mut voidptr:   g37 = UNIQUE | NON_NULL, FIXED
    let mut worker: *mut Worker = voidptr as *mut Worker;
    // 1841: mut worker: typeof(_3) = *mut {l3} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1841: mut worker:   l3 = UNIQUE | NON_NULL, (empty)
    // 1841: voidptr: typeof(_4) = *mut {l5} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1841: voidptr:   l5 = UNIQUE | NON_NULL, (empty)
    // 1841: voidptr as *mut ... orker: typeof(_3 = move _4 as *mut Worker (Misc)) = *mut {l183} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1841: voidptr as *mut ... orker:   l183 = UNIQUE | NON_NULL, (empty)
    let mut wid: libc::c_int = worker.offset_from(workers) as libc::c_long as libc::c_int;
    // 1842: worker: typeof(_8) = *mut {l10} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1842: worker:   l10 = UNIQUE | NON_NULL, (empty)
    // 1842: workers: typeof(_9) = *const {l12} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1842: workers:   l12 = UNIQUE | NON_NULL, (empty)
    // 1842: workers: typeof(_10) = *mut {l14} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1842: workers:   l14 = UNIQUE | NON_NULL, (empty)
    // 1842: workers: typeof(_11) = *mut {l16} *mut {l17} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1842: workers:   l16 = UNIQUE | NON_NULL, (empty)
    // 1842: workers:   l17 = UNIQUE | NON_NULL, (empty)
    // 1842: workers: typeof(_9 = move _10 as *const Worker (Pointer(MutToConstPointer))) = *const {l184} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1842: workers:   l184 = UNIQUE | NON_NULL, (empty)
    let mut lgl: *mut LGL = (*worker).lgl;
    // 1843: mut lgl: typeof(_12) = *mut {l19} LGL
    // 1843: mut lgl:   l19 = UNIQUE | NON_NULL, (empty)
    msg(
        wid,
        1 as libc::c_int,
        b"running\0" as *const u8 as *const libc::c_char,
        // 1847: b"running\0" as ... _char: typeof(_16) = *const {l24} i8
        // 1847: b"running\0" as ... _char:   l24 = UNIQUE | NON_NULL, (empty)
        // 1847: b"running\0" as ... st u8: typeof(_17) = *const {l26} u8
        // 1847: b"running\0" as ... st u8:   l26 = UNIQUE | NON_NULL, (empty)
        // 1847: b"running\0": typeof(_18) = *const {l28} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 1847: b"running\0":   l28 = UNIQUE | NON_NULL, (empty)
        // 1847: b"running\0": typeof(_19) = & {l30} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 1847: b"running\0":   l30 = UNIQUE | NON_NULL, FIXED
        // 1847: b"running\0" as ... st u8: typeof(_17 = move _18 as *const u8 (Pointer(ArrayToPointer))) = *const {l187} u8
        // 1847: b"running\0" as ... st u8:   l187 = UNIQUE | NON_NULL, (empty)
        // 1847: b"running\0": typeof(_19 = const b"running\x00") = & {l185} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 1847: b"running\0":   l185 = UNIQUE | NON_NULL, (empty)
        // 1847: b"running\0" as ... _char: typeof(_16 = move _17 as *const i8 (Misc)) = *const {l188} i8
        // 1847: b"running\0" as ... _char:   l188 = UNIQUE | NON_NULL, (empty)
        // 1847: b"running\0": typeof(_18 = &raw const (*_19)) = *const {l186} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 1847: b"running\0":   l186 = UNIQUE | NON_NULL, (empty)
    );
    (*worker).res = lglsat(lgl);
    // 1849: lgl: typeof(_21) = *mut {l33} LGL
    // 1849: lgl:   l33 = UNIQUE | NON_NULL, (empty)
    msg(
        wid,
        1 as libc::c_int,
        b"result %d\0" as *const u8 as *const libc::c_char,
        // 1853: b"result %d\0"  ... _char: typeof(_25) = *const {l38} i8
        // 1853: b"result %d\0"  ... _char:   l38 = UNIQUE | NON_NULL, (empty)
        // 1853: b"result %d\0"  ... st u8: typeof(_26) = *const {l40} u8
        // 1853: b"result %d\0"  ... st u8:   l40 = UNIQUE | NON_NULL, (empty)
        // 1853: b"result %d\0": typeof(_27) = *const {l42} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1853: b"result %d\0":   l42 = UNIQUE | NON_NULL, (empty)
        // 1853: b"result %d\0": typeof(_28) = & {l44} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1853: b"result %d\0":   l44 = UNIQUE | NON_NULL, FIXED
        // 1853: b"result %d\0"  ... st u8: typeof(_26 = move _27 as *const u8 (Pointer(ArrayToPointer))) = *const {l191} u8
        // 1853: b"result %d\0"  ... st u8:   l191 = UNIQUE | NON_NULL, (empty)
        // 1853: b"result %d\0"  ... _char: typeof(_25 = move _26 as *const i8 (Misc)) = *const {l192} i8
        // 1853: b"result %d\0"  ... _char:   l192 = UNIQUE | NON_NULL, (empty)
        // 1853: b"result %d\0": typeof(_28 = const b"result %d\x00") = & {l189} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1853: b"result %d\0":   l189 = UNIQUE | NON_NULL, (empty)
        // 1853: b"result %d\0": typeof(_27 = &raw const (*_28)) = *const {l190} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1853: b"result %d\0":   l190 = UNIQUE | NON_NULL, (empty)
        (*worker).res,
    );
    if (*worker).res == 0 {
        return 0 as *mut libc::c_void;
        // 1857: 0 as *mut libc: ... _void: typeof(_0 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l193} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1857: 0 as *mut libc: ... _void:   l193 = UNIQUE | NON_NULL, (empty)
    }
    if pthread_mutex_lock(&mut donemutex) != 0 {
    // 1859: &mut donemutex: typeof(_37) = *mut {l54} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1859: &mut donemutex:   l54 = UNIQUE | NON_NULL, (empty)
    // 1859: &mut donemutex: typeof(_38) = &mut {l56} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1859: &mut donemutex:   l56 = UNIQUE | NON_NULL, (empty)
    // 1859: donemutex: typeof(_39) = *mut {l58} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1859: donemutex:   l58 = UNIQUE | NON_NULL, (empty)
    // 1859: &mut donemutex: typeof(_38 = &mut (*_39)) = &mut {l194} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1859: &mut donemutex:   l194 = UNIQUE | NON_NULL, (empty)
    // 1859: &mut donemutex: typeof(_37 = &raw mut (*_38)) = *mut {l195} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1859: &mut donemutex:   l195 = UNIQUE | NON_NULL, (empty)
        warn(b"failed to lock 'done' mutex in worker\0" as *const u8 as *const libc::c_char);
        // 1860: b"failed to loc ... _char: typeof(_41) = *const {l61} i8
        // 1860: b"failed to loc ... _char:   l61 = UNIQUE | NON_NULL, (empty)
        // 1860: b"failed to loc ... st u8: typeof(_42) = *const {l63} u8
        // 1860: b"failed to loc ... st u8:   l63 = UNIQUE | NON_NULL, (empty)
        // 1860: b"failed to loc ... er\0": typeof(_43) = *const {l65} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
        // 1860: b"failed to loc ... er\0":   l65 = UNIQUE | NON_NULL, (empty)
        // 1860: b"failed to loc ... er\0": typeof(_44) = & {l67} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
        // 1860: b"failed to loc ... er\0":   l67 = UNIQUE | NON_NULL, FIXED
        // 1860: b"failed to loc ... st u8: typeof(_42 = move _43 as *const u8 (Pointer(ArrayToPointer))) = *const {l198} u8
        // 1860: b"failed to loc ... st u8:   l198 = UNIQUE | NON_NULL, (empty)
        // 1860: b"failed to loc ... _char: typeof(_41 = move _42 as *const i8 (Misc)) = *const {l199} i8
        // 1860: b"failed to loc ... _char:   l199 = UNIQUE | NON_NULL, (empty)
        // 1860: b"failed to loc ... er\0": typeof(_43 = &raw const (*_44)) = *const {l197} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
        // 1860: b"failed to loc ... er\0":   l197 = UNIQUE | NON_NULL, (empty)
        // 1860: b"failed to loc ... er\0": typeof(_44 = const b"failed to lock \'done\' mutex in worker\x00") = & {l196} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
        // 1860: b"failed to loc ... er\0":   l196 = UNIQUE | NON_NULL, (empty)
    }
    done = 1 as libc::c_int;
    // 1862: done: typeof(_46) = *mut {l70} i32
    // 1862: done:   l70 = UNIQUE | NON_NULL, (empty)
    if pthread_mutex_unlock(&mut donemutex) != 0 {
    // 1863: &mut donemutex: typeof(_50) = *mut {l75} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1863: &mut donemutex:   l75 = UNIQUE | NON_NULL, (empty)
    // 1863: &mut donemutex: typeof(_51) = &mut {l77} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1863: &mut donemutex:   l77 = UNIQUE | NON_NULL, (empty)
    // 1863: donemutex: typeof(_52) = *mut {l79} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1863: donemutex:   l79 = UNIQUE | NON_NULL, (empty)
    // 1863: &mut donemutex: typeof(_51 = &mut (*_52)) = &mut {l200} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1863: &mut donemutex:   l200 = UNIQUE | NON_NULL, (empty)
    // 1863: &mut donemutex: typeof(_50 = &raw mut (*_51)) = *mut {l201} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
    // 1863: &mut donemutex:   l201 = UNIQUE | NON_NULL, (empty)
        warn(b"failed to unlock 'done' mutex in worker\0" as *const u8 as *const libc::c_char);
        // 1864: b"failed to unl ... _char: typeof(_54) = *const {l82} i8
        // 1864: b"failed to unl ... _char:   l82 = UNIQUE | NON_NULL, (empty)
        // 1864: b"failed to unl ... st u8: typeof(_55) = *const {l84} u8
        // 1864: b"failed to unl ... st u8:   l84 = UNIQUE | NON_NULL, (empty)
        // 1864: b"failed to unl ... er\0": typeof(_56) = *const {l86} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 1864: b"failed to unl ... er\0":   l86 = UNIQUE | NON_NULL, (empty)
        // 1864: b"failed to unl ... er\0": typeof(_57) = & {l88} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 1864: b"failed to unl ... er\0":   l88 = UNIQUE | NON_NULL, FIXED
        // 1864: b"failed to unl ... st u8: typeof(_55 = move _56 as *const u8 (Pointer(ArrayToPointer))) = *const {l204} u8
        // 1864: b"failed to unl ... st u8:   l204 = UNIQUE | NON_NULL, (empty)
        // 1864: b"failed to unl ... _char: typeof(_54 = move _55 as *const i8 (Misc)) = *const {l205} i8
        // 1864: b"failed to unl ... _char:   l205 = UNIQUE | NON_NULL, (empty)
        // 1864: b"failed to unl ... er\0": typeof(_56 = &raw const (*_57)) = *const {l203} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 1864: b"failed to unl ... er\0":   l203 = UNIQUE | NON_NULL, (empty)
        // 1864: b"failed to unl ... er\0": typeof(_57 = const b"failed to unlock \'done\' mutex in worker\x00") = & {l202} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 1864: b"failed to unl ... er\0":   l202 = UNIQUE | NON_NULL, (empty)
    }
    msg(
        wid,
        2 as libc::c_int,
        b"%d decisions, %d conflicts, %.0f props, %.1f MB\0" as *const u8 as *const libc::c_char,
        // 1869: b"%d decisions, ... _char: typeof(_61) = *const {l93} i8
        // 1869: b"%d decisions, ... _char:   l93 = UNIQUE | NON_NULL, (empty)
        // 1869: b"%d decisions, ... st u8: typeof(_62) = *const {l95} u8
        // 1869: b"%d decisions, ... st u8:   l95 = UNIQUE | NON_NULL, (empty)
        // 1869: b"%d decisions, ... MB\0": typeof(_63) = *const {l97} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
        // 1869: b"%d decisions, ... MB\0":   l97 = UNIQUE | NON_NULL, (empty)
        // 1869: b"%d decisions, ... MB\0": typeof(_64) = & {l99} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
        // 1869: b"%d decisions, ... MB\0":   l99 = UNIQUE | NON_NULL, FIXED
        // 1869: b"%d decisions, ... MB\0": typeof(_64 = const b"%d decisions, %d conflicts, %.0f props, %.1f MB\x00") = & {l206} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
        // 1869: b"%d decisions, ... MB\0":   l206 = UNIQUE | NON_NULL, (empty)
        // 1869: b"%d decisions, ... MB\0": typeof(_63 = &raw const (*_64)) = *const {l207} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
        // 1869: b"%d decisions, ... MB\0":   l207 = UNIQUE | NON_NULL, (empty)
        // 1869: b"%d decisions, ... st u8: typeof(_62 = move _63 as *const u8 (Pointer(ArrayToPointer))) = *const {l208} u8
        // 1869: b"%d decisions, ... st u8:   l208 = UNIQUE | NON_NULL, (empty)
        // 1869: b"%d decisions, ... _char: typeof(_61 = move _62 as *const i8 (Misc)) = *const {l209} i8
        // 1869: b"%d decisions, ... _char:   l209 = UNIQUE | NON_NULL, (empty)
        lglgetdecs(lgl),
        // 1870: lgl: typeof(_66) = *mut {l102} LGL
        // 1870: lgl:   l102 = UNIQUE | NON_NULL, (empty)
        lglgetconfs(lgl),
        // 1871: lgl: typeof(_68) = *mut {l105} LGL
        // 1871: lgl:   l105 = UNIQUE | NON_NULL, (empty)
        lglgetprops(lgl),
        // 1872: lgl: typeof(_70) = *mut {l108} LGL
        // 1872: lgl:   l108 = UNIQUE | NON_NULL, (empty)
        lglmb(lgl),
        // 1873: lgl: typeof(_72) = *mut {l111} LGL
        // 1873: lgl:   l111 = UNIQUE | NON_NULL, (empty)
    );
    if verbose >= 2 as libc::c_int {
    // 1875: verbose: typeof(_76) = *mut {l116} i32
    // 1875: verbose:   l116 = UNIQUE | NON_NULL, (empty)
        if pthread_mutex_lock(&mut fixedmutex) != 0 {
        // 1876: &mut fixedmutex: typeof(_81) = *mut {l122} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
        // 1876: &mut fixedmutex:   l122 = UNIQUE | NON_NULL, (empty)
        // 1876: &mut fixedmutex: typeof(_82) = &mut {l124} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
        // 1876: &mut fixedmutex:   l124 = UNIQUE | NON_NULL, (empty)
        // 1876: fixedmutex: typeof(_83) = *mut {l126} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
        // 1876: fixedmutex:   l126 = UNIQUE | NON_NULL, (empty)
        // 1876: &mut fixedmutex: typeof(_81 = &raw mut (*_82)) = *mut {l211} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
        // 1876: &mut fixedmutex:   l211 = UNIQUE | NON_NULL, (empty)
        // 1876: &mut fixedmutex: typeof(_82 = &mut (*_83)) = &mut {l210} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
        // 1876: &mut fixedmutex:   l210 = UNIQUE | NON_NULL, (empty)
            warn(b"failed to lock 'fixed' in work\0" as *const u8 as *const libc::c_char);
            // 1877: b"failed to loc ... _char: typeof(_85) = *const {l129} i8
            // 1877: b"failed to loc ... _char:   l129 = UNIQUE | NON_NULL, (empty)
            // 1877: b"failed to loc ... st u8: typeof(_86) = *const {l131} u8
            // 1877: b"failed to loc ... st u8:   l131 = UNIQUE | NON_NULL, (empty)
            // 1877: b"failed to loc ... rk\0": typeof(_87) = *const {l133} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
            // 1877: b"failed to loc ... rk\0":   l133 = UNIQUE | NON_NULL, (empty)
            // 1877: b"failed to loc ... rk\0": typeof(_88) = & {l135} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
            // 1877: b"failed to loc ... rk\0":   l135 = UNIQUE | NON_NULL, FIXED
            // 1877: b"failed to loc ... _char: typeof(_85 = move _86 as *const i8 (Misc)) = *const {l215} i8
            // 1877: b"failed to loc ... _char:   l215 = UNIQUE | NON_NULL, (empty)
            // 1877: b"failed to loc ... rk\0": typeof(_88 = const b"failed to lock \'fixed\' in work\x00") = & {l212} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
            // 1877: b"failed to loc ... rk\0":   l212 = UNIQUE | NON_NULL, (empty)
            // 1877: b"failed to loc ... st u8: typeof(_86 = move _87 as *const u8 (Pointer(ArrayToPointer))) = *const {l214} u8
            // 1877: b"failed to loc ... st u8:   l214 = UNIQUE | NON_NULL, (empty)
            // 1877: b"failed to loc ... rk\0": typeof(_87 = &raw const (*_88)) = *const {l213} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
            // 1877: b"failed to loc ... rk\0":   l213 = UNIQUE | NON_NULL, (empty)
        }
        msg(
            wid,
            2 as libc::c_int,
            b"consumed %d units %.0f%%, produced %d units %.0f%%\0" as *const u8
            // 1882: b"consumed %d u ... _char: typeof(_92) = *const {l140} i8
            // 1882: b"consumed %d u ... _char:   l140 = UNIQUE | NON_NULL, (empty)
            // 1882: b"consumed %d u ... st u8: typeof(_93) = *const {l142} u8
            // 1882: b"consumed %d u ... st u8:   l142 = UNIQUE | NON_NULL, (empty)
            // 1882: b"consumed %d u ... %%\0": typeof(_94) = *const {l144} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 1882: b"consumed %d u ... %%\0":   l144 = UNIQUE | NON_NULL, (empty)
            // 1882: b"consumed %d u ... %%\0": typeof(_95) = & {l146} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 1882: b"consumed %d u ... %%\0":   l146 = UNIQUE | NON_NULL, FIXED
            // 1882: b"consumed %d u ... %%\0": typeof(_94 = &raw const (*_95)) = *const {l217} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 1882: b"consumed %d u ... %%\0":   l217 = UNIQUE | NON_NULL, (empty)
            // 1882: b"consumed %d u ... st u8: typeof(_93 = move _94 as *const u8 (Pointer(ArrayToPointer))) = *const {l218} u8
            // 1882: b"consumed %d u ... st u8:   l218 = UNIQUE | NON_NULL, (empty)
            // 1882: b"consumed %d u ... _char: typeof(_92 = move _93 as *const i8 (Misc)) = *const {l219} i8
            // 1882: b"consumed %d u ... _char:   l219 = UNIQUE | NON_NULL, (empty)
            // 1882: b"consumed %d u ... %%\0": typeof(_95 = const b"consumed %d units %.0f%%, produced %d units %.0f%%\x00") = & {l216} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 1882: b"consumed %d u ... %%\0":   l216 = UNIQUE | NON_NULL, (empty)
                as *const libc::c_char,
            (*worker).stats.units.consumed,
            percent(
                (*worker).stats.units.consumed as libc::c_double,
                nfixed as libc::c_double,
                // 1887: nfixed: typeof(_102) = *mut {l154} i32
                // 1887: nfixed:   l154 = UNIQUE | NON_NULL, (empty)
            ),
            (*worker).stats.units.produced,
            percent(
                (*worker).stats.units.produced as libc::c_double,
                nfixed as libc::c_double,
                // 1892: nfixed: typeof(_109) = *mut {l162} i32
                // 1892: nfixed:   l162 = UNIQUE | NON_NULL, (empty)
            ),
        );
        if pthread_mutex_unlock(&mut fixedmutex) != 0 {
        // 1895: &mut fixedmutex: typeof(_112) = *mut {l166} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
        // 1895: &mut fixedmutex:   l166 = UNIQUE | NON_NULL, (empty)
        // 1895: &mut fixedmutex: typeof(_113) = &mut {l168} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
        // 1895: &mut fixedmutex:   l168 = UNIQUE | NON_NULL, (empty)
        // 1895: fixedmutex: typeof(_114) = *mut {l170} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
        // 1895: fixedmutex:   l170 = UNIQUE | NON_NULL, (empty)
        // 1895: &mut fixedmutex: typeof(_112 = &raw mut (*_113)) = *mut {l221} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
        // 1895: &mut fixedmutex:   l221 = UNIQUE | NON_NULL, (empty)
        // 1895: &mut fixedmutex: typeof(_113 = &mut (*_114)) = &mut {l220} DefId(0:517 ~ plingeling[18f5]::pthread_mutex_t)
        // 1895: &mut fixedmutex:   l220 = UNIQUE | NON_NULL, (empty)
            warn(b"failed to unlock 'fixed' in work\0" as *const u8 as *const libc::c_char);
            // 1896: b"failed to unl ... _char: typeof(_116) = *const {l173} i8
            // 1896: b"failed to unl ... _char:   l173 = UNIQUE | NON_NULL, (empty)
            // 1896: b"failed to unl ... st u8: typeof(_117) = *const {l175} u8
            // 1896: b"failed to unl ... st u8:   l175 = UNIQUE | NON_NULL, (empty)
            // 1896: b"failed to unl ... rk\0": typeof(_118) = *const {l177} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
            // 1896: b"failed to unl ... rk\0":   l177 = UNIQUE | NON_NULL, (empty)
            // 1896: b"failed to unl ... rk\0": typeof(_119) = & {l179} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
            // 1896: b"failed to unl ... rk\0":   l179 = UNIQUE | NON_NULL, FIXED
            // 1896: b"failed to unl ... rk\0": typeof(_118 = &raw const (*_119)) = *const {l223} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
            // 1896: b"failed to unl ... rk\0":   l223 = UNIQUE | NON_NULL, (empty)
            // 1896: b"failed to unl ... _char: typeof(_116 = move _117 as *const i8 (Misc)) = *const {l225} i8
            // 1896: b"failed to unl ... _char:   l225 = UNIQUE | NON_NULL, (empty)
            // 1896: b"failed to unl ... rk\0": typeof(_119 = const b"failed to unlock \'fixed\' in work\x00") = & {l222} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
            // 1896: b"failed to unl ... rk\0":   l222 = UNIQUE | NON_NULL, (empty)
            // 1896: b"failed to unl ... st u8: typeof(_117 = move _118 as *const u8 (Pointer(ArrayToPointer))) = *const {l224} u8
            // 1896: b"failed to unl ... st u8:   l224 = UNIQUE | NON_NULL, (empty)
        }
    }
    return worker as *mut libc::c_void;
    // 1899: worker: typeof(_120) = *mut {l181} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 1899: worker:   l181 = UNIQUE | NON_NULL, (empty)
    // 1899: worker as *mut  ... _void: typeof(_0 = move _120 as *mut libc::c_void (Misc)) = *mut {l226} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1899: worker as *mut  ... _void:   l226 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn getsystemtotalmem(mut explain: libc::c_int) -> int64_t {
    let mut res: libc::c_longlong = 0;
    let mut p: *mut FILE = popen(
    // 1903: mut p: typeof(_4) = *mut {l4} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 1903: mut p:   l4 = UNIQUE | NON_NULL, (empty)
        b"grep MemTotal /proc/meminfo\0" as *const u8 as *const libc::c_char,
        // 1904: b"grep MemTotal ... _char: typeof(_5) = *const {l6} i8
        // 1904: b"grep MemTotal ... _char:   l6 = UNIQUE | NON_NULL, (empty)
        // 1904: b"grep MemTotal ... st u8: typeof(_6) = *const {l8} u8
        // 1904: b"grep MemTotal ... st u8:   l8 = UNIQUE | NON_NULL, (empty)
        // 1904: b"grep MemTotal ... fo\0": typeof(_7) = *const {l10} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
        // 1904: b"grep MemTotal ... fo\0":   l10 = UNIQUE | NON_NULL, (empty)
        // 1904: b"grep MemTotal ... fo\0": typeof(_8) = & {l12} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
        // 1904: b"grep MemTotal ... fo\0":   l12 = UNIQUE | NON_NULL, FIXED
        // 1904: b"grep MemTotal ... fo\0": typeof(_7 = &raw const (*_8)) = *const {l93} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
        // 1904: b"grep MemTotal ... fo\0":   l93 = UNIQUE | NON_NULL, (empty)
        // 1904: b"grep MemTotal ... fo\0": typeof(_8 = const b"grep MemTotal /proc/meminfo\x00") = & {l92} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
        // 1904: b"grep MemTotal ... fo\0":   l92 = UNIQUE | NON_NULL, (empty)
        // 1904: b"grep MemTotal ... st u8: typeof(_6 = move _7 as *const u8 (Pointer(ArrayToPointer))) = *const {l94} u8
        // 1904: b"grep MemTotal ... st u8:   l94 = UNIQUE | NON_NULL, (empty)
        // 1904: b"grep MemTotal ... _char: typeof(_5 = move _6 as *const i8 (Misc)) = *const {l95} i8
        // 1904: b"grep MemTotal ... _char:   l95 = UNIQUE | NON_NULL, (empty)
        b"r\0" as *const u8 as *const libc::c_char,
        // 1905: b"r\0" as *cons ... _char: typeof(_9) = *const {l14} i8
        // 1905: b"r\0" as *cons ... _char:   l14 = UNIQUE | NON_NULL, (empty)
        // 1905: b"r\0" as *const u8: typeof(_10) = *const {l16} u8
        // 1905: b"r\0" as *const u8:   l16 = UNIQUE | NON_NULL, (empty)
        // 1905: b"r\0": typeof(_11) = *const {l18} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1905: b"r\0":   l18 = UNIQUE | NON_NULL, (empty)
        // 1905: b"r\0": typeof(_12) = & {l20} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1905: b"r\0":   l20 = UNIQUE | NON_NULL, FIXED
        // 1905: b"r\0": typeof(_11 = &raw const (*_12)) = *const {l97} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1905: b"r\0":   l97 = UNIQUE | NON_NULL, (empty)
        // 1905: b"r\0" as *cons ... _char: typeof(_9 = move _10 as *const i8 (Misc)) = *const {l99} i8
        // 1905: b"r\0" as *cons ... _char:   l99 = UNIQUE | NON_NULL, (empty)
        // 1905: b"r\0" as *const u8: typeof(_10 = move _11 as *const u8 (Pointer(ArrayToPointer))) = *const {l98} u8
        // 1905: b"r\0" as *const u8:   l98 = UNIQUE | NON_NULL, (empty)
        // 1905: b"r\0": typeof(_12 = const b"r\x00") = & {l96} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1905: b"r\0":   l96 = UNIQUE | NON_NULL, (empty)
    );
    if !p.is_null()
    // 1907: p: typeof(_17) = *mut {l26} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 1907: p:   l26 = UNIQUE | NON_NULL, (empty)
        && fscanf(
            p,
            // 1909: p: typeof(_20) = *mut {l30} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 1909: p:   l30 = UNIQUE | NON_NULL, (empty)
            b"MemTotal: %lld kB\0" as *const u8 as *const libc::c_char,
            // 1910: b"MemTotal: %ll ... _char: typeof(_21) = *const {l32} i8
            // 1910: b"MemTotal: %ll ... _char:   l32 = UNIQUE | NON_NULL, (empty)
            // 1910: b"MemTotal: %ll ... st u8: typeof(_22) = *const {l34} u8
            // 1910: b"MemTotal: %ll ... st u8:   l34 = UNIQUE | NON_NULL, (empty)
            // 1910: b"MemTotal: %ll ... kB\0": typeof(_23) = *const {l36} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 1910: b"MemTotal: %ll ... kB\0":   l36 = UNIQUE | NON_NULL, (empty)
            // 1910: b"MemTotal: %ll ... kB\0": typeof(_24) = & {l38} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 1910: b"MemTotal: %ll ... kB\0":   l38 = UNIQUE | NON_NULL, FIXED
            // 1910: b"MemTotal: %ll ... st u8: typeof(_22 = move _23 as *const u8 (Pointer(ArrayToPointer))) = *const {l102} u8
            // 1910: b"MemTotal: %ll ... st u8:   l102 = UNIQUE | NON_NULL, (empty)
            // 1910: b"MemTotal: %ll ... kB\0": typeof(_23 = &raw const (*_24)) = *const {l101} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 1910: b"MemTotal: %ll ... kB\0":   l101 = UNIQUE | NON_NULL, (empty)
            // 1910: b"MemTotal: %ll ... kB\0": typeof(_24 = const b"MemTotal: %lld kB\x00") = & {l100} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 1910: b"MemTotal: %ll ... kB\0":   l100 = UNIQUE | NON_NULL, (empty)
            // 1910: b"MemTotal: %ll ... _char: typeof(_21 = move _22 as *const i8 (Misc)) = *const {l103} i8
            // 1910: b"MemTotal: %ll ... _char:   l103 = UNIQUE | NON_NULL, (empty)
            &mut res as *mut libc::c_longlong,
            // 1911: &mut res as *mu ... glong: typeof(_25) = *mut {l40} i64
            // 1911: &mut res as *mu ... glong:   l40 = UNIQUE | NON_NULL, (empty)
            // 1911: &mut res: typeof(_26) = &mut {l42} i64
            // 1911: &mut res:   l42 = UNIQUE | NON_NULL, (empty)
            // 1911: &mut res: typeof(_25 = &raw mut (*_26)) = *mut {l105} i64
            // 1911: &mut res:   l105 = UNIQUE | NON_NULL, (empty)
            // 1911: &mut res: typeof(_26 = &mut _3) = &mut {l104} i64
            // 1911: &mut res:   l104 = UNIQUE | NON_NULL, (empty)
        ) == 1 as libc::c_int
    {
        if explain != 0 {
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"%lld KB total memory according to /proc/meminfo\0" as *const u8
                // 1918: b"%lld KB total ... _char: typeof(_36) = *const {l53} i8
                // 1918: b"%lld KB total ... _char:   l53 = UNIQUE | NON_NULL, (empty)
                // 1918: b"%lld KB total ... st u8: typeof(_37) = *const {l55} u8
                // 1918: b"%lld KB total ... st u8:   l55 = UNIQUE | NON_NULL, (empty)
                // 1918: b"%lld KB total ... fo\0": typeof(_38) = *const {l57} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                // 1918: b"%lld KB total ... fo\0":   l57 = UNIQUE | NON_NULL, (empty)
                // 1918: b"%lld KB total ... fo\0": typeof(_39) = & {l59} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                // 1918: b"%lld KB total ... fo\0":   l59 = UNIQUE | NON_NULL, FIXED
                // 1918: b"%lld KB total ... fo\0": typeof(_39 = const b"%lld KB total memory according to /proc/meminfo\x00") = & {l106} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                // 1918: b"%lld KB total ... fo\0":   l106 = UNIQUE | NON_NULL, (empty)
                // 1918: b"%lld KB total ... _char: typeof(_36 = move _37 as *const i8 (Misc)) = *const {l109} i8
                // 1918: b"%lld KB total ... _char:   l109 = UNIQUE | NON_NULL, (empty)
                // 1918: b"%lld KB total ... st u8: typeof(_37 = move _38 as *const u8 (Pointer(ArrayToPointer))) = *const {l108} u8
                // 1918: b"%lld KB total ... st u8:   l108 = UNIQUE | NON_NULL, (empty)
                // 1918: b"%lld KB total ... fo\0": typeof(_38 = &raw const (*_39)) = *const {l107} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                // 1918: b"%lld KB total ... fo\0":   l107 = UNIQUE | NON_NULL, (empty)
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
                // 1930: b"assuming comp ... _char: typeof(_54) = *const {l75} i8
                // 1930: b"assuming comp ... _char:   l75 = UNIQUE | NON_NULL, (empty)
                // 1930: b"assuming comp ... st u8: typeof(_55) = *const {l77} u8
                // 1930: b"assuming comp ... st u8:   l77 = UNIQUE | NON_NULL, (empty)
                // 1930: b"assuming comp ... GB\0": typeof(_56) = *const {l79} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                // 1930: b"assuming comp ... GB\0":   l79 = UNIQUE | NON_NULL, (empty)
                // 1930: b"assuming comp ... GB\0": typeof(_57) = & {l81} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                // 1930: b"assuming comp ... GB\0":   l81 = UNIQUE | NON_NULL, FIXED
                // 1930: b"assuming comp ... GB\0": typeof(_57 = const b"assuming compiled in memory size of %d GB\x00") = & {l110} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                // 1930: b"assuming comp ... GB\0":   l110 = UNIQUE | NON_NULL, (empty)
                // 1930: b"assuming comp ... st u8: typeof(_55 = move _56 as *const u8 (Pointer(ArrayToPointer))) = *const {l112} u8
                // 1930: b"assuming comp ... st u8:   l112 = UNIQUE | NON_NULL, (empty)
                // 1930: b"assuming comp ... _char: typeof(_54 = move _55 as *const i8 (Misc)) = *const {l113} i8
                // 1930: b"assuming comp ... _char:   l113 = UNIQUE | NON_NULL, (empty)
                // 1930: b"assuming comp ... GB\0": typeof(_56 = &raw const (*_57)) = *const {l111} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                // 1930: b"assuming comp ... GB\0":   l111 = UNIQUE | NON_NULL, (empty)
                12 as libc::c_int,
            );
        }
    }
    if !p.is_null() {
    // 1935: p: typeof(_62) = *mut {l87} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 1935: p:   l87 = UNIQUE | NON_NULL, (empty)
        pclose(p);
        // 1936: p: typeof(_64) = *mut {l90} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 1936: p:   l90 = UNIQUE | NON_NULL, (empty)
    }
    return res as int64_t;
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
    let mut p: *mut FILE = 0 as *mut FILE;
    // 1950: mut p: typeof(_12) = *mut {l12} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 1950: mut p:   l12 = UNIQUE | NON_NULL, (empty)
    // 1950: 0 as *mut FILE: typeof(_12 = const 0_usize as *mut _IO_FILE (PointerFromExposedAddress)) = *mut {l506} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 1950: 0 as *mut FILE:   l506 = UNIQUE | NON_NULL, (empty)
    syscores = sysconf(_SC_NPROCESSORS_ONLN as libc::c_int) as libc::c_int;
    if explain != 0 {
        if syscores > 0 as libc::c_int {
            msg(
                -(1 as libc::c_int),
                1 as libc::c_int,
                b"'sysconf' reports %d processors online\0" as *const u8 as *const libc::c_char,
                // 1957: b"'sysconf' rep ... _char: typeof(_26) = *const {l27} i8
                // 1957: b"'sysconf' rep ... _char:   l27 = UNIQUE | NON_NULL, (empty)
                // 1957: b"'sysconf' rep ... st u8: typeof(_27) = *const {l29} u8
                // 1957: b"'sysconf' rep ... st u8:   l29 = UNIQUE | NON_NULL, (empty)
                // 1957: b"'sysconf' rep ... ne\0": typeof(_28) = *const {l31} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1957: b"'sysconf' rep ... ne\0":   l31 = UNIQUE | NON_NULL, (empty)
                // 1957: b"'sysconf' rep ... ne\0": typeof(_29) = & {l33} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1957: b"'sysconf' rep ... ne\0":   l33 = UNIQUE | NON_NULL, FIXED
                // 1957: b"'sysconf' rep ... ne\0": typeof(_29 = const b"\'sysconf\' reports %d processors online\x00") = & {l507} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1957: b"'sysconf' rep ... ne\0":   l507 = UNIQUE | NON_NULL, (empty)
                // 1957: b"'sysconf' rep ... _char: typeof(_26 = move _27 as *const i8 (Misc)) = *const {l510} i8
                // 1957: b"'sysconf' rep ... _char:   l510 = UNIQUE | NON_NULL, (empty)
                // 1957: b"'sysconf' rep ... ne\0": typeof(_28 = &raw const (*_29)) = *const {l508} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1957: b"'sysconf' rep ... ne\0":   l508 = UNIQUE | NON_NULL, (empty)
                // 1957: b"'sysconf' rep ... st u8: typeof(_27 = move _28 as *const u8 (Pointer(ArrayToPointer))) = *const {l509} u8
                // 1957: b"'sysconf' rep ... st u8:   l509 = UNIQUE | NON_NULL, (empty)
                syscores,
            );
        } else {
            msg(
                -(1 as libc::c_int),
                1 as libc::c_int,
                b"'sysconf' fails to determine number of online processors\0" as *const u8
                // 1964: b"'sysconf' fai ... _char: typeof(_36) = *const {l41} i8
                // 1964: b"'sysconf' fai ... _char:   l41 = UNIQUE | NON_NULL, (empty)
                // 1964: b"'sysconf' fai ... st u8: typeof(_37) = *const {l43} u8
                // 1964: b"'sysconf' fai ... st u8:   l43 = UNIQUE | NON_NULL, (empty)
                // 1964: b"'sysconf' fai ... rs\0": typeof(_38) = *const {l45} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                // 1964: b"'sysconf' fai ... rs\0":   l45 = UNIQUE | NON_NULL, (empty)
                // 1964: b"'sysconf' fai ... rs\0": typeof(_39) = & {l47} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                // 1964: b"'sysconf' fai ... rs\0":   l47 = UNIQUE | NON_NULL, FIXED
                // 1964: b"'sysconf' fai ... rs\0": typeof(_39 = const b"\'sysconf\' fails to determine number of online processors\x00") = & {l511} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                // 1964: b"'sysconf' fai ... rs\0":   l511 = UNIQUE | NON_NULL, (empty)
                // 1964: b"'sysconf' fai ... _char: typeof(_36 = move _37 as *const i8 (Misc)) = *const {l514} i8
                // 1964: b"'sysconf' fai ... _char:   l514 = UNIQUE | NON_NULL, (empty)
                // 1964: b"'sysconf' fai ... st u8: typeof(_37 = move _38 as *const u8 (Pointer(ArrayToPointer))) = *const {l513} u8
                // 1964: b"'sysconf' fai ... st u8:   l513 = UNIQUE | NON_NULL, (empty)
                // 1964: b"'sysconf' fai ... rs\0": typeof(_38 = &raw const (*_39)) = *const {l512} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                // 1964: b"'sysconf' fai ... rs\0":   l512 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
        }
    }
    p = popen(
    // 1969: popen( b"grep ' ... ar, ): typeof(_40) = *mut {l49} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 1969: popen( b"grep ' ... ar, ):   l49 = UNIQUE | NON_NULL, (empty)
        b"grep '^core id' /proc/cpuinfo 2>/dev/null|sort|uniq|wc -l\0" as *const u8
        // 1970: b"grep '^core i ... _char: typeof(_41) = *const {l51} i8
        // 1970: b"grep '^core i ... _char:   l51 = UNIQUE | NON_NULL, (empty)
        // 1970: b"grep '^core i ... st u8: typeof(_42) = *const {l53} u8
        // 1970: b"grep '^core i ... st u8:   l53 = UNIQUE | NON_NULL, (empty)
        // 1970: b"grep '^core i ... -l\0": typeof(_43) = *const {l55} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003a)) }]
        // 1970: b"grep '^core i ... -l\0":   l55 = UNIQUE | NON_NULL, (empty)
        // 1970: b"grep '^core i ... -l\0": typeof(_44) = & {l57} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003a)) }]
        // 1970: b"grep '^core i ... -l\0":   l57 = UNIQUE | NON_NULL, FIXED
        // 1970: b"grep '^core i ... _char: typeof(_41 = move _42 as *const i8 (Misc)) = *const {l518} i8
        // 1970: b"grep '^core i ... _char:   l518 = UNIQUE | NON_NULL, (empty)
        // 1970: b"grep '^core i ... -l\0": typeof(_43 = &raw const (*_44)) = *const {l516} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003a)) }]
        // 1970: b"grep '^core i ... -l\0":   l516 = UNIQUE | NON_NULL, (empty)
        // 1970: b"grep '^core i ... st u8: typeof(_42 = move _43 as *const u8 (Pointer(ArrayToPointer))) = *const {l517} u8
        // 1970: b"grep '^core i ... st u8:   l517 = UNIQUE | NON_NULL, (empty)
        // 1970: b"grep '^core i ... -l\0": typeof(_44 = const b"grep \'^core id\' /proc/cpuinfo 2>/dev/null|sort|uniq|wc -l\x00") = & {l515} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003a)) }]
        // 1970: b"grep '^core i ... -l\0":   l515 = UNIQUE | NON_NULL, (empty)
            as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
        // 1972: b"r\0" as *cons ... _char: typeof(_45) = *const {l59} i8
        // 1972: b"r\0" as *cons ... _char:   l59 = UNIQUE | NON_NULL, (empty)
        // 1972: b"r\0" as *const u8: typeof(_46) = *const {l61} u8
        // 1972: b"r\0" as *const u8:   l61 = UNIQUE | NON_NULL, (empty)
        // 1972: b"r\0": typeof(_47) = *const {l63} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1972: b"r\0":   l63 = UNIQUE | NON_NULL, (empty)
        // 1972: b"r\0": typeof(_48) = & {l65} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1972: b"r\0":   l65 = UNIQUE | NON_NULL, FIXED
        // 1972: b"r\0" as *cons ... _char: typeof(_45 = move _46 as *const i8 (Misc)) = *const {l522} i8
        // 1972: b"r\0" as *cons ... _char:   l522 = UNIQUE | NON_NULL, (empty)
        // 1972: b"r\0": typeof(_48 = const b"r\x00") = & {l519} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1972: b"r\0":   l519 = UNIQUE | NON_NULL, (empty)
        // 1972: b"r\0": typeof(_47 = &raw const (*_48)) = *const {l520} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1972: b"r\0":   l520 = UNIQUE | NON_NULL, (empty)
        // 1972: b"r\0" as *const u8: typeof(_46 = move _47 as *const u8 (Pointer(ArrayToPointer))) = *const {l521} u8
        // 1972: b"r\0" as *const u8:   l521 = UNIQUE | NON_NULL, (empty)
    );
    if !p.is_null() {
    // 1974: p: typeof(_52) = *mut {l70} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 1974: p:   l70 = UNIQUE | NON_NULL, (empty)
        if fscanf(
            p,
            // 1976: p: typeof(_56) = *mut {l75} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 1976: p:   l75 = UNIQUE | NON_NULL, (empty)
            b"%d\0" as *const u8 as *const libc::c_char,
            // 1977: b"%d\0" as *con ... _char: typeof(_57) = *const {l77} i8
            // 1977: b"%d\0" as *con ... _char:   l77 = UNIQUE | NON_NULL, (empty)
            // 1977: b"%d\0" as *const u8: typeof(_58) = *const {l79} u8
            // 1977: b"%d\0" as *const u8:   l79 = UNIQUE | NON_NULL, (empty)
            // 1977: b"%d\0": typeof(_59) = *const {l81} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 1977: b"%d\0":   l81 = UNIQUE | NON_NULL, (empty)
            // 1977: b"%d\0": typeof(_60) = & {l83} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 1977: b"%d\0":   l83 = UNIQUE | NON_NULL, FIXED
            // 1977: b"%d\0": typeof(_60 = const b"%d\x00") = & {l523} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 1977: b"%d\0":   l523 = UNIQUE | NON_NULL, (empty)
            // 1977: b"%d\0" as *const u8: typeof(_58 = move _59 as *const u8 (Pointer(ArrayToPointer))) = *const {l525} u8
            // 1977: b"%d\0" as *const u8:   l525 = UNIQUE | NON_NULL, (empty)
            // 1977: b"%d\0": typeof(_59 = &raw const (*_60)) = *const {l524} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 1977: b"%d\0":   l524 = UNIQUE | NON_NULL, (empty)
            // 1977: b"%d\0" as *con ... _char: typeof(_57 = move _58 as *const i8 (Misc)) = *const {l526} i8
            // 1977: b"%d\0" as *con ... _char:   l526 = UNIQUE | NON_NULL, (empty)
            &mut coreids as *mut libc::c_int,
            // 1978: &mut coreids as ... c_int: typeof(_61) = *mut {l85} i32
            // 1978: &mut coreids as ... c_int:   l85 = UNIQUE | NON_NULL, (empty)
            // 1978: &mut coreids: typeof(_62) = &mut {l87} i32
            // 1978: &mut coreids:   l87 = UNIQUE | NON_NULL, (empty)
            // 1978: &mut coreids: typeof(_61 = &raw mut (*_62)) = *mut {l528} i32
            // 1978: &mut coreids:   l528 = UNIQUE | NON_NULL, (empty)
            // 1978: &mut coreids: typeof(_62 = &mut _4) = &mut {l527} i32
            // 1978: &mut coreids:   l527 = UNIQUE | NON_NULL, (empty)
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
                    // 1988: b"found %d uniq ... _char: typeof(_76) = *const {l102} i8
                    // 1988: b"found %d uniq ... _char:   l102 = UNIQUE | NON_NULL, (empty)
                    // 1988: b"found %d uniq ... st u8: typeof(_77) = *const {l104} u8
                    // 1988: b"found %d uniq ... st u8:   l104 = UNIQUE | NON_NULL, (empty)
                    // 1988: b"found %d uniq ... o'\0": typeof(_78) = *const {l106} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                    // 1988: b"found %d uniq ... o'\0":   l106 = UNIQUE | NON_NULL, (empty)
                    // 1988: b"found %d uniq ... o'\0": typeof(_79) = & {l108} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                    // 1988: b"found %d uniq ... o'\0":   l108 = UNIQUE | NON_NULL, FIXED
                    // 1988: b"found %d uniq ... _char: typeof(_76 = move _77 as *const i8 (Misc)) = *const {l532} i8
                    // 1988: b"found %d uniq ... _char:   l532 = UNIQUE | NON_NULL, (empty)
                    // 1988: b"found %d uniq ... o'\0": typeof(_78 = &raw const (*_79)) = *const {l530} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                    // 1988: b"found %d uniq ... o'\0":   l530 = UNIQUE | NON_NULL, (empty)
                    // 1988: b"found %d uniq ... st u8: typeof(_77 = move _78 as *const u8 (Pointer(ArrayToPointer))) = *const {l531} u8
                    // 1988: b"found %d uniq ... st u8:   l531 = UNIQUE | NON_NULL, (empty)
                    // 1988: b"found %d uniq ... o'\0": typeof(_79 = const b"found %d unique core ids in \'/proc/cpuinfo\'\x00") = & {l529} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                    // 1988: b"found %d uniq ... o'\0":   l529 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                    coreids,
                );
            } else {
                msg(
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    b"failed to extract core ids from '/proc/cpuinfo'\0" as *const u8
                    // 1996: b"failed to ext ... _char: typeof(_86) = *const {l116} i8
                    // 1996: b"failed to ext ... _char:   l116 = UNIQUE | NON_NULL, (empty)
                    // 1996: b"failed to ext ... st u8: typeof(_87) = *const {l118} u8
                    // 1996: b"failed to ext ... st u8:   l118 = UNIQUE | NON_NULL, (empty)
                    // 1996: b"failed to ext ... o'\0": typeof(_88) = *const {l120} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                    // 1996: b"failed to ext ... o'\0":   l120 = UNIQUE | NON_NULL, (empty)
                    // 1996: b"failed to ext ... o'\0": typeof(_89) = & {l122} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                    // 1996: b"failed to ext ... o'\0":   l122 = UNIQUE | NON_NULL, FIXED
                    // 1996: b"failed to ext ... _char: typeof(_86 = move _87 as *const i8 (Misc)) = *const {l536} i8
                    // 1996: b"failed to ext ... _char:   l536 = UNIQUE | NON_NULL, (empty)
                    // 1996: b"failed to ext ... o'\0": typeof(_88 = &raw const (*_89)) = *const {l534} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                    // 1996: b"failed to ext ... o'\0":   l534 = UNIQUE | NON_NULL, (empty)
                    // 1996: b"failed to ext ... o'\0": typeof(_89 = const b"failed to extract core ids from \'/proc/cpuinfo\'\x00") = & {l533} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                    // 1996: b"failed to ext ... o'\0":   l533 = UNIQUE | NON_NULL, (empty)
                    // 1996: b"failed to ext ... st u8: typeof(_87 = move _88 as *const u8 (Pointer(ArrayToPointer))) = *const {l535} u8
                    // 1996: b"failed to ext ... st u8:   l535 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                );
            }
        }
        pclose(p);
        // 2001: p: typeof(_91) = *mut {l125} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 2001: p:   l125 = UNIQUE | NON_NULL, (empty)
    } else {
        coreids = 0 as libc::c_int;
    }
    p = popen(
    // 2005: popen( b"grep ' ... ar, ): typeof(_93) = *mut {l128} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 2005: popen( b"grep ' ... ar, ):   l128 = UNIQUE | NON_NULL, (empty)
        b"grep '^physical id' /proc/cpuinfo 2>/dev/null|sort|uniq|wc -l\0" as *const u8
        // 2006: b"grep '^physic ... _char: typeof(_94) = *const {l130} i8
        // 2006: b"grep '^physic ... _char:   l130 = UNIQUE | NON_NULL, (empty)
        // 2006: b"grep '^physic ... st u8: typeof(_95) = *const {l132} u8
        // 2006: b"grep '^physic ... st u8:   l132 = UNIQUE | NON_NULL, (empty)
        // 2006: b"grep '^physic ... -l\0": typeof(_96) = *const {l134} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003e)) }]
        // 2006: b"grep '^physic ... -l\0":   l134 = UNIQUE | NON_NULL, (empty)
        // 2006: b"grep '^physic ... -l\0": typeof(_97) = & {l136} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003e)) }]
        // 2006: b"grep '^physic ... -l\0":   l136 = UNIQUE | NON_NULL, FIXED
        // 2006: b"grep '^physic ... -l\0": typeof(_96 = &raw const (*_97)) = *const {l538} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003e)) }]
        // 2006: b"grep '^physic ... -l\0":   l538 = UNIQUE | NON_NULL, (empty)
        // 2006: b"grep '^physic ... -l\0": typeof(_97 = const b"grep \'^physical id\' /proc/cpuinfo 2>/dev/null|sort|uniq|wc -l\x00") = & {l537} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003e)) }]
        // 2006: b"grep '^physic ... -l\0":   l537 = UNIQUE | NON_NULL, (empty)
        // 2006: b"grep '^physic ... st u8: typeof(_95 = move _96 as *const u8 (Pointer(ArrayToPointer))) = *const {l539} u8
        // 2006: b"grep '^physic ... st u8:   l539 = UNIQUE | NON_NULL, (empty)
        // 2006: b"grep '^physic ... _char: typeof(_94 = move _95 as *const i8 (Misc)) = *const {l540} i8
        // 2006: b"grep '^physic ... _char:   l540 = UNIQUE | NON_NULL, (empty)
            as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
        // 2008: b"r\0" as *cons ... _char: typeof(_98) = *const {l138} i8
        // 2008: b"r\0" as *cons ... _char:   l138 = UNIQUE | NON_NULL, (empty)
        // 2008: b"r\0" as *const u8: typeof(_99) = *const {l140} u8
        // 2008: b"r\0" as *const u8:   l140 = UNIQUE | NON_NULL, (empty)
        // 2008: b"r\0": typeof(_100) = *const {l142} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2008: b"r\0":   l142 = UNIQUE | NON_NULL, (empty)
        // 2008: b"r\0": typeof(_101) = & {l144} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2008: b"r\0":   l144 = UNIQUE | NON_NULL, FIXED
        // 2008: b"r\0" as *const u8: typeof(_99 = move _100 as *const u8 (Pointer(ArrayToPointer))) = *const {l543} u8
        // 2008: b"r\0" as *const u8:   l543 = UNIQUE | NON_NULL, (empty)
        // 2008: b"r\0" as *cons ... _char: typeof(_98 = move _99 as *const i8 (Misc)) = *const {l544} i8
        // 2008: b"r\0" as *cons ... _char:   l544 = UNIQUE | NON_NULL, (empty)
        // 2008: b"r\0": typeof(_101 = const b"r\x00") = & {l541} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2008: b"r\0":   l541 = UNIQUE | NON_NULL, (empty)
        // 2008: b"r\0": typeof(_100 = &raw const (*_101)) = *const {l542} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2008: b"r\0":   l542 = UNIQUE | NON_NULL, (empty)
    );
    if !p.is_null() {
    // 2010: p: typeof(_105) = *mut {l149} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 2010: p:   l149 = UNIQUE | NON_NULL, (empty)
        if fscanf(
            p,
            // 2012: p: typeof(_109) = *mut {l154} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 2012: p:   l154 = UNIQUE | NON_NULL, (empty)
            b"%d\0" as *const u8 as *const libc::c_char,
            // 2013: b"%d\0" as *con ... _char: typeof(_110) = *const {l156} i8
            // 2013: b"%d\0" as *con ... _char:   l156 = UNIQUE | NON_NULL, (empty)
            // 2013: b"%d\0" as *const u8: typeof(_111) = *const {l158} u8
            // 2013: b"%d\0" as *const u8:   l158 = UNIQUE | NON_NULL, (empty)
            // 2013: b"%d\0": typeof(_112) = *const {l160} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2013: b"%d\0":   l160 = UNIQUE | NON_NULL, (empty)
            // 2013: b"%d\0": typeof(_113) = & {l162} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2013: b"%d\0":   l162 = UNIQUE | NON_NULL, FIXED
            // 2013: b"%d\0": typeof(_112 = &raw const (*_113)) = *const {l546} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2013: b"%d\0":   l546 = UNIQUE | NON_NULL, (empty)
            // 2013: b"%d\0" as *const u8: typeof(_111 = move _112 as *const u8 (Pointer(ArrayToPointer))) = *const {l547} u8
            // 2013: b"%d\0" as *const u8:   l547 = UNIQUE | NON_NULL, (empty)
            // 2013: b"%d\0" as *con ... _char: typeof(_110 = move _111 as *const i8 (Misc)) = *const {l548} i8
            // 2013: b"%d\0" as *con ... _char:   l548 = UNIQUE | NON_NULL, (empty)
            // 2013: b"%d\0": typeof(_113 = const b"%d\x00") = & {l545} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2013: b"%d\0":   l545 = UNIQUE | NON_NULL, (empty)
            &mut physids as *mut libc::c_int,
            // 2014: &mut physids as ... c_int: typeof(_114) = *mut {l164} i32
            // 2014: &mut physids as ... c_int:   l164 = UNIQUE | NON_NULL, (empty)
            // 2014: &mut physids: typeof(_115) = &mut {l166} i32
            // 2014: &mut physids:   l166 = UNIQUE | NON_NULL, (empty)
            // 2014: &mut physids: typeof(_115 = &mut _5) = &mut {l549} i32
            // 2014: &mut physids:   l549 = UNIQUE | NON_NULL, (empty)
            // 2014: &mut physids: typeof(_114 = &raw mut (*_115)) = *mut {l550} i32
            // 2014: &mut physids:   l550 = UNIQUE | NON_NULL, (empty)
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
                    // 2024: b"found %d uniq ... _char: typeof(_129) = *const {l181} i8
                    // 2024: b"found %d uniq ... _char:   l181 = UNIQUE | NON_NULL, (empty)
                    // 2024: b"found %d uniq ... st u8: typeof(_130) = *const {l183} u8
                    // 2024: b"found %d uniq ... st u8:   l183 = UNIQUE | NON_NULL, (empty)
                    // 2024: b"found %d uniq ... o'\0": typeof(_131) = *const {l185} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                    // 2024: b"found %d uniq ... o'\0":   l185 = UNIQUE | NON_NULL, (empty)
                    // 2024: b"found %d uniq ... o'\0": typeof(_132) = & {l187} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                    // 2024: b"found %d uniq ... o'\0":   l187 = UNIQUE | NON_NULL, FIXED
                    // 2024: b"found %d uniq ... st u8: typeof(_130 = move _131 as *const u8 (Pointer(ArrayToPointer))) = *const {l553} u8
                    // 2024: b"found %d uniq ... st u8:   l553 = UNIQUE | NON_NULL, (empty)
                    // 2024: b"found %d uniq ... _char: typeof(_129 = move _130 as *const i8 (Misc)) = *const {l554} i8
                    // 2024: b"found %d uniq ... _char:   l554 = UNIQUE | NON_NULL, (empty)
                    // 2024: b"found %d uniq ... o'\0": typeof(_132 = const b"found %d unique physical ids in \'/proc/cpuinfo\'\x00") = & {l551} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                    // 2024: b"found %d uniq ... o'\0":   l551 = UNIQUE | NON_NULL, (empty)
                    // 2024: b"found %d uniq ... o'\0": typeof(_131 = &raw const (*_132)) = *const {l552} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                    // 2024: b"found %d uniq ... o'\0":   l552 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                    physids,
                );
            } else {
                msg(
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    b"failed to extract physical ids from '/proc/cpuinfo'\0" as *const u8
                    // 2032: b"failed to ext ... _char: typeof(_139) = *const {l195} i8
                    // 2032: b"failed to ext ... _char:   l195 = UNIQUE | NON_NULL, (empty)
                    // 2032: b"failed to ext ... st u8: typeof(_140) = *const {l197} u8
                    // 2032: b"failed to ext ... st u8:   l197 = UNIQUE | NON_NULL, (empty)
                    // 2032: b"failed to ext ... o'\0": typeof(_141) = *const {l199} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                    // 2032: b"failed to ext ... o'\0":   l199 = UNIQUE | NON_NULL, (empty)
                    // 2032: b"failed to ext ... o'\0": typeof(_142) = & {l201} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                    // 2032: b"failed to ext ... o'\0":   l201 = UNIQUE | NON_NULL, FIXED
                    // 2032: b"failed to ext ... o'\0": typeof(_142 = const b"failed to extract physical ids from \'/proc/cpuinfo\'\x00") = & {l555} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                    // 2032: b"failed to ext ... o'\0":   l555 = UNIQUE | NON_NULL, (empty)
                    // 2032: b"failed to ext ... st u8: typeof(_140 = move _141 as *const u8 (Pointer(ArrayToPointer))) = *const {l557} u8
                    // 2032: b"failed to ext ... st u8:   l557 = UNIQUE | NON_NULL, (empty)
                    // 2032: b"failed to ext ... o'\0": typeof(_141 = &raw const (*_142)) = *const {l556} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                    // 2032: b"failed to ext ... o'\0":   l556 = UNIQUE | NON_NULL, (empty)
                    // 2032: b"failed to ext ... _char: typeof(_139 = move _140 as *const i8 (Misc)) = *const {l558} i8
                    // 2032: b"failed to ext ... _char:   l558 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                );
            }
        }
        pclose(p);
        // 2037: p: typeof(_144) = *mut {l204} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 2037: p:   l204 = UNIQUE | NON_NULL, (empty)
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
                // 2049: b"%d cores = %d ... _char: typeof(_168) = *const {l229} i8
                // 2049: b"%d cores = %d ... _char:   l229 = UNIQUE | NON_NULL, (empty)
                // 2049: b"%d cores = %d ... st u8: typeof(_169) = *const {l231} u8
                // 2049: b"%d cores = %d ... st u8:   l231 = UNIQUE | NON_NULL, (empty)
                // 2049: b"%d cores = %d ... o'\0": typeof(_170) = *const {l233} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003c)) }]
                // 2049: b"%d cores = %d ... o'\0":   l233 = UNIQUE | NON_NULL, (empty)
                // 2049: b"%d cores = %d ... o'\0": typeof(_171) = & {l235} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003c)) }]
                // 2049: b"%d cores = %d ... o'\0":   l235 = UNIQUE | NON_NULL, FIXED
                // 2049: b"%d cores = %d ... st u8: typeof(_169 = move _170 as *const u8 (Pointer(ArrayToPointer))) = *const {l561} u8
                // 2049: b"%d cores = %d ... st u8:   l561 = UNIQUE | NON_NULL, (empty)
                // 2049: b"%d cores = %d ... o'\0": typeof(_171 = const b"%d cores = %d core times %d physical ids in \'/proc/cpuinfo\'\x00") = & {l559} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003c)) }]
                // 2049: b"%d cores = %d ... o'\0":   l559 = UNIQUE | NON_NULL, (empty)
                // 2049: b"%d cores = %d ... _char: typeof(_168 = move _169 as *const i8 (Misc)) = *const {l562} i8
                // 2049: b"%d cores = %d ... _char:   l562 = UNIQUE | NON_NULL, (empty)
                // 2049: b"%d cores = %d ... o'\0": typeof(_170 = &raw const (*_171)) = *const {l560} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003c)) }]
                // 2049: b"%d cores = %d ... o'\0":   l560 = UNIQUE | NON_NULL, (empty)
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
                // 2066: b"'sysconf' and ... _char: typeof(_194) = *const {l259} i8
                // 2066: b"'sysconf' and ... _char:   l259 = UNIQUE | NON_NULL, (empty)
                // 2066: b"'sysconf' and ... st u8: typeof(_195) = *const {l261} u8
                // 2066: b"'sysconf' and ... st u8:   l261 = UNIQUE | NON_NULL, (empty)
                // 2066: b"'sysconf' and ... ch\0": typeof(_196) = *const {l263} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 2066: b"'sysconf' and ... ch\0":   l263 = UNIQUE | NON_NULL, (empty)
                // 2066: b"'sysconf' and ... ch\0": typeof(_197) = & {l265} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 2066: b"'sysconf' and ... ch\0":   l265 = UNIQUE | NON_NULL, FIXED
                // 2066: b"'sysconf' and ... ch\0": typeof(_196 = &raw const (*_197)) = *const {l564} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 2066: b"'sysconf' and ... ch\0":   l564 = UNIQUE | NON_NULL, (empty)
                // 2066: b"'sysconf' and ... ch\0": typeof(_197 = const b"\'sysconf\' and \'/proc/cpuinfo\' results match\x00") = & {l563} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 2066: b"'sysconf' and ... ch\0":   l563 = UNIQUE | NON_NULL, (empty)
                // 2066: b"'sysconf' and ... st u8: typeof(_195 = move _196 as *const u8 (Pointer(ArrayToPointer))) = *const {l565} u8
                // 2066: b"'sysconf' and ... st u8:   l565 = UNIQUE | NON_NULL, (empty)
                // 2066: b"'sysconf' and ... _char: typeof(_194 = move _195 as *const i8 (Misc)) = *const {l566} i8
                // 2066: b"'sysconf' and ... _char:   l566 = UNIQUE | NON_NULL, (empty)
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
                // 2076: b"only '/proc/c ... _char: typeof(_214) = *const {l283} i8
                // 2076: b"only '/proc/c ... _char:   l283 = UNIQUE | NON_NULL, (empty)
                // 2076: b"only '/proc/c ... st u8: typeof(_215) = *const {l285} u8
                // 2076: b"only '/proc/c ... st u8:   l285 = UNIQUE | NON_NULL, (empty)
                // 2076: b"only '/proc/c ... id\0": typeof(_216) = *const {l287} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                // 2076: b"only '/proc/c ... id\0":   l287 = UNIQUE | NON_NULL, (empty)
                // 2076: b"only '/proc/c ... id\0": typeof(_217) = & {l289} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                // 2076: b"only '/proc/c ... id\0":   l289 = UNIQUE | NON_NULL, FIXED
                // 2076: b"only '/proc/c ... id\0": typeof(_217 = const b"only \'/proc/cpuinfo\' result valid\x00") = & {l567} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                // 2076: b"only '/proc/c ... id\0":   l567 = UNIQUE | NON_NULL, (empty)
                // 2076: b"only '/proc/c ... _char: typeof(_214 = move _215 as *const i8 (Misc)) = *const {l570} i8
                // 2076: b"only '/proc/c ... _char:   l570 = UNIQUE | NON_NULL, (empty)
                // 2076: b"only '/proc/c ... id\0": typeof(_216 = &raw const (*_217)) = *const {l568} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                // 2076: b"only '/proc/c ... id\0":   l568 = UNIQUE | NON_NULL, (empty)
                // 2076: b"only '/proc/c ... st u8: typeof(_215 = move _216 as *const u8 (Pointer(ArrayToPointer))) = *const {l569} u8
                // 2076: b"only '/proc/c ... st u8:   l569 = UNIQUE | NON_NULL, (empty)
            );
        }
        useprocpuinfo = 1 as libc::c_int;
    } else if procpuinfocores <= 0 as libc::c_int && syscores > 0 as libc::c_int {
        if explain != 0 {
            msg(
                -(1 as libc::c_int),
                1 as libc::c_int,
                b"only 'sysconf' result valid\0" as *const u8 as *const libc::c_char,
                // 2085: b"only 'sysconf ... _char: typeof(_234) = *const {l307} i8
                // 2085: b"only 'sysconf ... _char:   l307 = UNIQUE | NON_NULL, (empty)
                // 2085: b"only 'sysconf ... st u8: typeof(_235) = *const {l309} u8
                // 2085: b"only 'sysconf ... st u8:   l309 = UNIQUE | NON_NULL, (empty)
                // 2085: b"only 'sysconf ... id\0": typeof(_236) = *const {l311} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                // 2085: b"only 'sysconf ... id\0":   l311 = UNIQUE | NON_NULL, (empty)
                // 2085: b"only 'sysconf ... id\0": typeof(_237) = & {l313} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                // 2085: b"only 'sysconf ... id\0":   l313 = UNIQUE | NON_NULL, FIXED
                // 2085: b"only 'sysconf ... _char: typeof(_234 = move _235 as *const i8 (Misc)) = *const {l574} i8
                // 2085: b"only 'sysconf ... _char:   l574 = UNIQUE | NON_NULL, (empty)
                // 2085: b"only 'sysconf ... id\0": typeof(_236 = &raw const (*_237)) = *const {l572} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                // 2085: b"only 'sysconf ... id\0":   l572 = UNIQUE | NON_NULL, (empty)
                // 2085: b"only 'sysconf ... st u8: typeof(_235 = move _236 as *const u8 (Pointer(ArrayToPointer))) = *const {l573} u8
                // 2085: b"only 'sysconf ... st u8:   l573 = UNIQUE | NON_NULL, (empty)
                // 2085: b"only 'sysconf ... id\0": typeof(_237 = const b"only \'sysconf\' result valid\x00") = & {l571} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                // 2085: b"only 'sysconf ... id\0":   l571 = UNIQUE | NON_NULL, (empty)
            );
        }
        usesyscores = 1 as libc::c_int;
    } else {
        intel = (system(
            b"grep vendor /proc/cpuinfo 2>/dev/null|grep -q Intel\0" as *const u8
            // 2091: b"grep vendor / ... _char: typeof(_241) = *const {l318} i8
            // 2091: b"grep vendor / ... _char:   l318 = UNIQUE | NON_NULL, (empty)
            // 2091: b"grep vendor / ... st u8: typeof(_242) = *const {l320} u8
            // 2091: b"grep vendor / ... st u8:   l320 = UNIQUE | NON_NULL, (empty)
            // 2091: b"grep vendor / ... el\0": typeof(_243) = *const {l322} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
            // 2091: b"grep vendor / ... el\0":   l322 = UNIQUE | NON_NULL, (empty)
            // 2091: b"grep vendor / ... el\0": typeof(_244) = & {l324} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
            // 2091: b"grep vendor / ... el\0":   l324 = UNIQUE | NON_NULL, FIXED
            // 2091: b"grep vendor / ... el\0": typeof(_244 = const b"grep vendor /proc/cpuinfo 2>/dev/null|grep -q Intel\x00") = & {l575} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
            // 2091: b"grep vendor / ... el\0":   l575 = UNIQUE | NON_NULL, (empty)
            // 2091: b"grep vendor / ... el\0": typeof(_243 = &raw const (*_244)) = *const {l576} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
            // 2091: b"grep vendor / ... el\0":   l576 = UNIQUE | NON_NULL, (empty)
            // 2091: b"grep vendor / ... _char: typeof(_241 = move _242 as *const i8 (Misc)) = *const {l578} i8
            // 2091: b"grep vendor / ... _char:   l578 = UNIQUE | NON_NULL, (empty)
            // 2091: b"grep vendor / ... st u8: typeof(_242 = move _243 as *const u8 (Pointer(ArrayToPointer))) = *const {l577} u8
            // 2091: b"grep vendor / ... st u8:   l577 = UNIQUE | NON_NULL, (empty)
                as *const libc::c_char,
        ) == 0) as libc::c_int;
        if intel != 0 && explain != 0 {
            msg(
                -(1 as libc::c_int),
                1 as libc::c_int,
                b"found Intel as vendor in '/proc/cpuinfo'\0" as *const u8 as *const libc::c_char,
                // 2098: b"found Intel a ... _char: typeof(_256) = *const {l337} i8
                // 2098: b"found Intel a ... _char:   l337 = UNIQUE | NON_NULL, (empty)
                // 2098: b"found Intel a ... st u8: typeof(_257) = *const {l339} u8
                // 2098: b"found Intel a ... st u8:   l339 = UNIQUE | NON_NULL, (empty)
                // 2098: b"found Intel a ... o'\0": typeof(_258) = *const {l341} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 2098: b"found Intel a ... o'\0":   l341 = UNIQUE | NON_NULL, (empty)
                // 2098: b"found Intel a ... o'\0": typeof(_259) = & {l343} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 2098: b"found Intel a ... o'\0":   l343 = UNIQUE | NON_NULL, FIXED
                // 2098: b"found Intel a ... st u8: typeof(_257 = move _258 as *const u8 (Pointer(ArrayToPointer))) = *const {l581} u8
                // 2098: b"found Intel a ... st u8:   l581 = UNIQUE | NON_NULL, (empty)
                // 2098: b"found Intel a ... _char: typeof(_256 = move _257 as *const i8 (Misc)) = *const {l582} i8
                // 2098: b"found Intel a ... _char:   l582 = UNIQUE | NON_NULL, (empty)
                // 2098: b"found Intel a ... o'\0": typeof(_258 = &raw const (*_259)) = *const {l580} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 2098: b"found Intel a ... o'\0":   l580 = UNIQUE | NON_NULL, (empty)
                // 2098: b"found Intel a ... o'\0": typeof(_259 = const b"found Intel as vendor in \'/proc/cpuinfo\'\x00") = & {l579} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 2098: b"found Intel a ... o'\0":   l579 = UNIQUE | NON_NULL, (empty)
            );
        }
        amd = (system(
            b"grep vendor /proc/cpuinfo 2>/dev/null|grep -q AMD\0" as *const u8
            // 2102: b"grep vendor / ... _char: typeof(_262) = *const {l347} i8
            // 2102: b"grep vendor / ... _char:   l347 = UNIQUE | NON_NULL, (empty)
            // 2102: b"grep vendor / ... st u8: typeof(_263) = *const {l349} u8
            // 2102: b"grep vendor / ... st u8:   l349 = UNIQUE | NON_NULL, (empty)
            // 2102: b"grep vendor / ... MD\0": typeof(_264) = *const {l351} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
            // 2102: b"grep vendor / ... MD\0":   l351 = UNIQUE | NON_NULL, (empty)
            // 2102: b"grep vendor / ... MD\0": typeof(_265) = & {l353} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
            // 2102: b"grep vendor / ... MD\0":   l353 = UNIQUE | NON_NULL, FIXED
            // 2102: b"grep vendor / ... _char: typeof(_262 = move _263 as *const i8 (Misc)) = *const {l586} i8
            // 2102: b"grep vendor / ... _char:   l586 = UNIQUE | NON_NULL, (empty)
            // 2102: b"grep vendor / ... MD\0": typeof(_265 = const b"grep vendor /proc/cpuinfo 2>/dev/null|grep -q AMD\x00") = & {l583} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
            // 2102: b"grep vendor / ... MD\0":   l583 = UNIQUE | NON_NULL, (empty)
            // 2102: b"grep vendor / ... st u8: typeof(_263 = move _264 as *const u8 (Pointer(ArrayToPointer))) = *const {l585} u8
            // 2102: b"grep vendor / ... st u8:   l585 = UNIQUE | NON_NULL, (empty)
            // 2102: b"grep vendor / ... MD\0": typeof(_264 = &raw const (*_265)) = *const {l584} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
            // 2102: b"grep vendor / ... MD\0":   l584 = UNIQUE | NON_NULL, (empty)
                as *const libc::c_char,
        ) == 0) as libc::c_int;
        if amd != 0 && explain != 0 {
            msg(
                -(1 as libc::c_int),
                1 as libc::c_int,
                b"found AMD as vendor in '/proc/cpuinfo'\0" as *const u8 as *const libc::c_char,
                // 2109: b"found AMD as  ... _char: typeof(_277) = *const {l366} i8
                // 2109: b"found AMD as  ... _char:   l366 = UNIQUE | NON_NULL, (empty)
                // 2109: b"found AMD as  ... st u8: typeof(_278) = *const {l368} u8
                // 2109: b"found AMD as  ... st u8:   l368 = UNIQUE | NON_NULL, (empty)
                // 2109: b"found AMD as  ... o'\0": typeof(_279) = *const {l370} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 2109: b"found AMD as  ... o'\0":   l370 = UNIQUE | NON_NULL, (empty)
                // 2109: b"found AMD as  ... o'\0": typeof(_280) = & {l372} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 2109: b"found AMD as  ... o'\0":   l372 = UNIQUE | NON_NULL, FIXED
                // 2109: b"found AMD as  ... o'\0": typeof(_279 = &raw const (*_280)) = *const {l588} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 2109: b"found AMD as  ... o'\0":   l588 = UNIQUE | NON_NULL, (empty)
                // 2109: b"found AMD as  ... o'\0": typeof(_280 = const b"found AMD as vendor in \'/proc/cpuinfo\'\x00") = & {l587} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 2109: b"found AMD as  ... o'\0":   l587 = UNIQUE | NON_NULL, (empty)
                // 2109: b"found AMD as  ... st u8: typeof(_278 = move _279 as *const u8 (Pointer(ArrayToPointer))) = *const {l589} u8
                // 2109: b"found AMD as  ... st u8:   l589 = UNIQUE | NON_NULL, (empty)
                // 2109: b"found AMD as  ... _char: typeof(_277 = move _278 as *const i8 (Misc)) = *const {l590} i8
                // 2109: b"found AMD as  ... _char:   l590 = UNIQUE | NON_NULL, (empty)
            );
        }
        if amd != 0 {
            if explain != 0 {
                msg(
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    b"trusting 'sysconf' on AMD\0" as *const u8 as *const libc::c_char,
                    // 2117: b"trusting 'sys ... _char: typeof(_291) = *const {l384} i8
                    // 2117: b"trusting 'sys ... _char:   l384 = UNIQUE | NON_NULL, (empty)
                    // 2117: b"trusting 'sys ... st u8: typeof(_292) = *const {l386} u8
                    // 2117: b"trusting 'sys ... st u8:   l386 = UNIQUE | NON_NULL, (empty)
                    // 2117: b"trusting 'sys ... MD\0": typeof(_293) = *const {l388} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
                    // 2117: b"trusting 'sys ... MD\0":   l388 = UNIQUE | NON_NULL, (empty)
                    // 2117: b"trusting 'sys ... MD\0": typeof(_294) = & {l390} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
                    // 2117: b"trusting 'sys ... MD\0":   l390 = UNIQUE | NON_NULL, FIXED
                    // 2117: b"trusting 'sys ... MD\0": typeof(_293 = &raw const (*_294)) = *const {l592} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
                    // 2117: b"trusting 'sys ... MD\0":   l592 = UNIQUE | NON_NULL, (empty)
                    // 2117: b"trusting 'sys ... _char: typeof(_291 = move _292 as *const i8 (Misc)) = *const {l594} i8
                    // 2117: b"trusting 'sys ... _char:   l594 = UNIQUE | NON_NULL, (empty)
                    // 2117: b"trusting 'sys ... st u8: typeof(_292 = move _293 as *const u8 (Pointer(ArrayToPointer))) = *const {l593} u8
                    // 2117: b"trusting 'sys ... st u8:   l593 = UNIQUE | NON_NULL, (empty)
                    // 2117: b"trusting 'sys ... MD\0": typeof(_294 = const b"trusting \'sysconf\' on AMD\x00") = & {l591} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
                    // 2117: b"trusting 'sys ... MD\0":   l591 = UNIQUE | NON_NULL, (empty)
                );
            }
            usesyscores = 1 as libc::c_int;
        } else if intel != 0 {
            if explain != 0 {
                msg(
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    b"'sysconf' result off by a factor of %f on Intel\0" as *const u8
                    // 2126: b"'sysconf' res ... _char: typeof(_306) = *const {l403} i8
                    // 2126: b"'sysconf' res ... _char:   l403 = UNIQUE | NON_NULL, (empty)
                    // 2126: b"'sysconf' res ... st u8: typeof(_307) = *const {l405} u8
                    // 2126: b"'sysconf' res ... st u8:   l405 = UNIQUE | NON_NULL, (empty)
                    // 2126: b"'sysconf' res ... el\0": typeof(_308) = *const {l407} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                    // 2126: b"'sysconf' res ... el\0":   l407 = UNIQUE | NON_NULL, (empty)
                    // 2126: b"'sysconf' res ... el\0": typeof(_309) = & {l409} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                    // 2126: b"'sysconf' res ... el\0":   l409 = UNIQUE | NON_NULL, FIXED
                    // 2126: b"'sysconf' res ... el\0": typeof(_308 = &raw const (*_309)) = *const {l596} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                    // 2126: b"'sysconf' res ... el\0":   l596 = UNIQUE | NON_NULL, (empty)
                    // 2126: b"'sysconf' res ... st u8: typeof(_307 = move _308 as *const u8 (Pointer(ArrayToPointer))) = *const {l597} u8
                    // 2126: b"'sysconf' res ... st u8:   l597 = UNIQUE | NON_NULL, (empty)
                    // 2126: b"'sysconf' res ... el\0": typeof(_309 = const b"\'sysconf\' result off by a factor of %f on Intel\x00") = & {l595} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                    // 2126: b"'sysconf' res ... el\0":   l595 = UNIQUE | NON_NULL, (empty)
                    // 2126: b"'sysconf' res ... _char: typeof(_306 = move _307 as *const i8 (Misc)) = *const {l598} i8
                    // 2126: b"'sysconf' res ... _char:   l598 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                    syscores as libc::c_double / procpuinfocores as libc::c_double,
                );
                msg(
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    b"trusting '/proc/cpuinfo' on Intel\0" as *const u8 as *const libc::c_char,
                    // 2133: b"trusting '/pr ... _char: typeof(_320) = *const {l421} i8
                    // 2133: b"trusting '/pr ... _char:   l421 = UNIQUE | NON_NULL, (empty)
                    // 2133: b"trusting '/pr ... st u8: typeof(_321) = *const {l423} u8
                    // 2133: b"trusting '/pr ... st u8:   l423 = UNIQUE | NON_NULL, (empty)
                    // 2133: b"trusting '/pr ... el\0": typeof(_322) = *const {l425} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                    // 2133: b"trusting '/pr ... el\0":   l425 = UNIQUE | NON_NULL, (empty)
                    // 2133: b"trusting '/pr ... el\0": typeof(_323) = & {l427} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                    // 2133: b"trusting '/pr ... el\0":   l427 = UNIQUE | NON_NULL, FIXED
                    // 2133: b"trusting '/pr ... _char: typeof(_320 = move _321 as *const i8 (Misc)) = *const {l602} i8
                    // 2133: b"trusting '/pr ... _char:   l602 = UNIQUE | NON_NULL, (empty)
                    // 2133: b"trusting '/pr ... el\0": typeof(_322 = &raw const (*_323)) = *const {l600} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                    // 2133: b"trusting '/pr ... el\0":   l600 = UNIQUE | NON_NULL, (empty)
                    // 2133: b"trusting '/pr ... el\0": typeof(_323 = const b"trusting \'/proc/cpuinfo\' on Intel\x00") = & {l599} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                    // 2133: b"trusting '/pr ... el\0":   l599 = UNIQUE | NON_NULL, (empty)
                    // 2133: b"trusting '/pr ... st u8: typeof(_321 = move _322 as *const u8 (Pointer(ArrayToPointer))) = *const {l601} u8
                    // 2133: b"trusting '/pr ... st u8:   l601 = UNIQUE | NON_NULL, (empty)
                );
            }
            useprocpuinfo = 1 as libc::c_int;
        } else {
            if explain != 0 {
                msg(
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    b"trusting 'sysconf' on unknown vendor machine\0" as *const u8
                    // 2142: b"trusting 'sys ... _char: typeof(_333) = *const {l438} i8
                    // 2142: b"trusting 'sys ... _char:   l438 = UNIQUE | NON_NULL, (empty)
                    // 2142: b"trusting 'sys ... st u8: typeof(_334) = *const {l440} u8
                    // 2142: b"trusting 'sys ... st u8:   l440 = UNIQUE | NON_NULL, (empty)
                    // 2142: b"trusting 'sys ... ne\0": typeof(_335) = *const {l442} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
                    // 2142: b"trusting 'sys ... ne\0":   l442 = UNIQUE | NON_NULL, (empty)
                    // 2142: b"trusting 'sys ... ne\0": typeof(_336) = & {l444} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
                    // 2142: b"trusting 'sys ... ne\0":   l444 = UNIQUE | NON_NULL, FIXED
                    // 2142: b"trusting 'sys ... st u8: typeof(_334 = move _335 as *const u8 (Pointer(ArrayToPointer))) = *const {l605} u8
                    // 2142: b"trusting 'sys ... st u8:   l605 = UNIQUE | NON_NULL, (empty)
                    // 2142: b"trusting 'sys ... ne\0": typeof(_336 = const b"trusting \'sysconf\' on unknown vendor machine\x00") = & {l603} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
                    // 2142: b"trusting 'sys ... ne\0":   l603 = UNIQUE | NON_NULL, (empty)
                    // 2142: b"trusting 'sys ... _char: typeof(_333 = move _334 as *const i8 (Misc)) = *const {l606} i8
                    // 2142: b"trusting 'sys ... _char:   l606 = UNIQUE | NON_NULL, (empty)
                    // 2142: b"trusting 'sys ... ne\0": typeof(_335 = &raw const (*_336)) = *const {l604} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
                    // 2142: b"trusting 'sys ... ne\0":   l604 = UNIQUE | NON_NULL, (empty)
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
                // 2154: b"assuming core ... _char: typeof(_349) = *const {l458} i8
                // 2154: b"assuming core ... _char:   l458 = UNIQUE | NON_NULL, (empty)
                // 2154: b"assuming core ... st u8: typeof(_350) = *const {l460} u8
                // 2154: b"assuming core ... st u8:   l460 = UNIQUE | NON_NULL, (empty)
                // 2154: b"assuming core ... %d\0": typeof(_351) = *const {l462} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003d)) }]
                // 2154: b"assuming core ... %d\0":   l462 = UNIQUE | NON_NULL, (empty)
                // 2154: b"assuming core ... %d\0": typeof(_352) = & {l464} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003d)) }]
                // 2154: b"assuming core ... %d\0":   l464 = UNIQUE | NON_NULL, FIXED
                // 2154: b"assuming core ... %d\0": typeof(_352 = const b"assuming cores = core * physical ids in \'/proc/cpuinfo\' = %d\x00") = & {l607} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003d)) }]
                // 2154: b"assuming core ... %d\0":   l607 = UNIQUE | NON_NULL, (empty)
                // 2154: b"assuming core ... %d\0": typeof(_351 = &raw const (*_352)) = *const {l608} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003d)) }]
                // 2154: b"assuming core ... %d\0":   l608 = UNIQUE | NON_NULL, (empty)
                // 2154: b"assuming core ... _char: typeof(_349 = move _350 as *const i8 (Misc)) = *const {l610} i8
                // 2154: b"assuming core ... _char:   l610 = UNIQUE | NON_NULL, (empty)
                // 2154: b"assuming core ... st u8: typeof(_350 = move _351 as *const u8 (Pointer(ArrayToPointer))) = *const {l609} u8
                // 2154: b"assuming core ... st u8:   l609 = UNIQUE | NON_NULL, (empty)
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
                // 2165: b"assuming core ... _char: typeof(_365) = *const {l478} i8
                // 2165: b"assuming core ... _char:   l478 = UNIQUE | NON_NULL, (empty)
                // 2165: b"assuming core ... st u8: typeof(_366) = *const {l480} u8
                // 2165: b"assuming core ... st u8:   l480 = UNIQUE | NON_NULL, (empty)
                // 2165: b"assuming core ... %d\0": typeof(_367) = *const {l482} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000041)) }]
                // 2165: b"assuming core ... %d\0":   l482 = UNIQUE | NON_NULL, (empty)
                // 2165: b"assuming core ... %d\0": typeof(_368) = & {l484} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000041)) }]
                // 2165: b"assuming core ... %d\0":   l484 = UNIQUE | NON_NULL, FIXED
                // 2165: b"assuming core ... st u8: typeof(_366 = move _367 as *const u8 (Pointer(ArrayToPointer))) = *const {l613} u8
                // 2165: b"assuming core ... st u8:   l613 = UNIQUE | NON_NULL, (empty)
                // 2165: b"assuming core ... %d\0": typeof(_367 = &raw const (*_368)) = *const {l612} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000041)) }]
                // 2165: b"assuming core ... %d\0":   l612 = UNIQUE | NON_NULL, (empty)
                // 2165: b"assuming core ... %d\0": typeof(_368 = const b"assuming cores = number of processors reported by \'sysconf\' = %d\x00") = & {l611} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000041)) }]
                // 2165: b"assuming core ... %d\0":   l611 = UNIQUE | NON_NULL, (empty)
                // 2165: b"assuming core ... _char: typeof(_365 = move _366 as *const i8 (Misc)) = *const {l614} i8
                // 2165: b"assuming core ... _char:   l614 = UNIQUE | NON_NULL, (empty)
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
                // 2176: b"using compile ... _char: typeof(_379) = *const {l496} i8
                // 2176: b"using compile ... _char:   l496 = UNIQUE | NON_NULL, (empty)
                // 2176: b"using compile ... st u8: typeof(_380) = *const {l498} u8
                // 2176: b"using compile ... st u8:   l498 = UNIQUE | NON_NULL, (empty)
                // 2176: b"using compile ... rs\0": typeof(_381) = *const {l500} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                // 2176: b"using compile ... rs\0":   l500 = UNIQUE | NON_NULL, (empty)
                // 2176: b"using compile ... rs\0": typeof(_382) = & {l502} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                // 2176: b"using compile ... rs\0":   l502 = UNIQUE | NON_NULL, FIXED
                // 2176: b"using compile ... rs\0": typeof(_381 = &raw const (*_382)) = *const {l616} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                // 2176: b"using compile ... rs\0":   l616 = UNIQUE | NON_NULL, (empty)
                // 2176: b"using compile ... rs\0": typeof(_382 = const b"using compiled in default value of %d workers\x00") = & {l615} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                // 2176: b"using compile ... rs\0":   l615 = UNIQUE | NON_NULL, (empty)
                // 2176: b"using compile ... st u8: typeof(_380 = move _381 as *const u8 (Pointer(ArrayToPointer))) = *const {l617} u8
                // 2176: b"using compile ... st u8:   l617 = UNIQUE | NON_NULL, (empty)
                // 2176: b"using compile ... _char: typeof(_379 = move _380 as *const i8 (Misc)) = *const {l618} i8
                // 2176: b"using compile ... _char:   l618 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
                8 as libc::c_int,
            );
        }
        res = 8 as libc::c_int;
    }
    return res;
}
unsafe extern "C" fn cmproduced(
    mut p: *const libc::c_void,
    // 2186: mut p: typeof(_1) = *const {g39} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2186: mut p:   g39 = UNIQUE | NON_NULL, FIXED
    mut q: *const libc::c_void,
    // 2187: mut q: typeof(_2) = *const {g40} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2187: mut q:   g40 = UNIQUE | NON_NULL, FIXED
) -> libc::c_int {
    let mut u: *mut Worker = *(p as *mut *mut Worker);
    // 2189: mut u: typeof(_4) = *mut {l4} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2189: mut u:   l4 = UNIQUE | NON_NULL, (empty)
    // 2189: (p as *mut *mut ... rker): typeof(_5) = *mut {l6} *mut {l7} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2189: (p as *mut *mut ... rker):   l6 = UNIQUE | NON_NULL, (empty)
    // 2189: (p as *mut *mut ... rker):   l7 = UNIQUE | NON_NULL, (empty)
    // 2189: p: typeof(_6) = *const {l9} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2189: p:   l9 = UNIQUE | NON_NULL, (empty)
    // 2189: (p as *mut *mut ... rker): typeof(_5 = move _6 as *mut *mut Worker (Misc)) = *mut {l34} *mut {l35} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2189: (p as *mut *mut ... rker):   l34 = UNIQUE | NON_NULL, (empty)
    // 2189: (p as *mut *mut ... rker):   l35 = UNIQUE | NON_NULL, (empty)
    let mut v: *mut Worker = *(q as *mut *mut Worker);
    // 2190: mut v: typeof(_7) = *mut {l11} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2190: mut v:   l11 = UNIQUE | NON_NULL, (empty)
    // 2190: (q as *mut *mut ... rker): typeof(_8) = *mut {l13} *mut {l14} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2190: (q as *mut *mut ... rker):   l13 = UNIQUE | NON_NULL, (empty)
    // 2190: (q as *mut *mut ... rker):   l14 = UNIQUE | NON_NULL, (empty)
    // 2190: q: typeof(_9) = *const {l16} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2190: q:   l16 = UNIQUE | NON_NULL, (empty)
    // 2190: (q as *mut *mut ... rker): typeof(_8 = move _9 as *mut *mut Worker (Misc)) = *mut {l36} *mut {l37} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2190: (q as *mut *mut ... rker):   l36 = UNIQUE | NON_NULL, (empty)
    // 2190: (q as *mut *mut ... rker):   l37 = UNIQUE | NON_NULL, (empty)
    let mut res: libc::c_int = (*v).stats.produced - (*u).stats.produced;
    if res != 0 {
        return res;
    }
    return u.offset_from(v) as libc::c_long as libc::c_int;
    // 2195: u: typeof(_20) = *mut {l28} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2195: u:   l28 = UNIQUE | NON_NULL, (empty)
    // 2195: v: typeof(_21) = *const {l30} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2195: v:   l30 = UNIQUE | NON_NULL, (empty)
    // 2195: v: typeof(_22) = *mut {l32} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2195: v:   l32 = UNIQUE | NON_NULL, (empty)
    // 2195: v: typeof(_21 = move _22 as *const Worker (Pointer(MutToConstPointer))) = *const {l38} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2195: v:   l38 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn cmpconsumed(
    mut p: *const libc::c_void,
    // 2198: mut p: typeof(_1) = *const {g41} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2198: mut p:   g41 = UNIQUE | NON_NULL, FIXED
    mut q: *const libc::c_void,
    // 2199: mut q: typeof(_2) = *const {g42} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2199: mut q:   g42 = UNIQUE | NON_NULL, FIXED
) -> libc::c_int {
    let mut u: *mut Worker = *(p as *mut *mut Worker);
    // 2201: mut u: typeof(_4) = *mut {l4} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2201: mut u:   l4 = UNIQUE | NON_NULL, (empty)
    // 2201: (p as *mut *mut ... rker): typeof(_5) = *mut {l6} *mut {l7} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2201: (p as *mut *mut ... rker):   l6 = UNIQUE | NON_NULL, (empty)
    // 2201: (p as *mut *mut ... rker):   l7 = UNIQUE | NON_NULL, (empty)
    // 2201: p: typeof(_6) = *const {l9} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2201: p:   l9 = UNIQUE | NON_NULL, (empty)
    // 2201: (p as *mut *mut ... rker): typeof(_5 = move _6 as *mut *mut Worker (Misc)) = *mut {l34} *mut {l35} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2201: (p as *mut *mut ... rker):   l34 = UNIQUE | NON_NULL, (empty)
    // 2201: (p as *mut *mut ... rker):   l35 = UNIQUE | NON_NULL, (empty)
    let mut v: *mut Worker = *(q as *mut *mut Worker);
    // 2202: mut v: typeof(_7) = *mut {l11} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2202: mut v:   l11 = UNIQUE | NON_NULL, (empty)
    // 2202: (q as *mut *mut ... rker): typeof(_8) = *mut {l13} *mut {l14} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2202: (q as *mut *mut ... rker):   l13 = UNIQUE | NON_NULL, (empty)
    // 2202: (q as *mut *mut ... rker):   l14 = UNIQUE | NON_NULL, (empty)
    // 2202: q: typeof(_9) = *const {l16} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2202: q:   l16 = UNIQUE | NON_NULL, (empty)
    // 2202: (q as *mut *mut ... rker): typeof(_8 = move _9 as *mut *mut Worker (Misc)) = *mut {l36} *mut {l37} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2202: (q as *mut *mut ... rker):   l36 = UNIQUE | NON_NULL, (empty)
    // 2202: (q as *mut *mut ... rker):   l37 = UNIQUE | NON_NULL, (empty)
    let mut res: libc::c_int = (*v).stats.consumed - (*u).stats.consumed;
    if res != 0 {
        return res;
    }
    return u.offset_from(v) as libc::c_long as libc::c_int;
    // 2207: u: typeof(_20) = *mut {l28} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2207: u:   l28 = UNIQUE | NON_NULL, (empty)
    // 2207: v: typeof(_21) = *const {l30} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2207: v:   l30 = UNIQUE | NON_NULL, (empty)
    // 2207: v: typeof(_22) = *mut {l32} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2207: v:   l32 = UNIQUE | NON_NULL, (empty)
    // 2207: v: typeof(_21 = move _22 as *const Worker (Pointer(MutToConstPointer))) = *const {l38} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2207: v:   l38 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn parsenbcoreenv() -> libc::c_int {
    let mut str: core::option::Option<&[(libc::c_char)]> = getenv(&*(b"NBCORE\0") as *const u8 as *const libc::c_char);
    // 2210: mut str: typeof(_2) = *const {l2} i8
    // 2210: mut str:   l2 = READ | OFFSET_ADD, (empty)
    // 2210: getenv(b"NBCORE ... char): typeof(_3) = *mut {l4} i8
    // 2210: getenv(b"NBCORE ... char):   l4 = READ | OFFSET_ADD, (empty)
    // 2210: b"NBCORE\0" as  ... _char: typeof(_4) = *const {l6} i8
    // 2210: b"NBCORE\0" as  ... _char:   l6 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
    // 2210: b"NBCORE\0" as  ... st u8: typeof(_5) = *const {l8} u8
    // 2210: b"NBCORE\0" as  ... st u8:   l8 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
    // 2210: b"NBCORE\0": typeof(_6) = *const {l10} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
    // 2210: b"NBCORE\0":   l10 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
    // 2210: b"NBCORE\0": typeof(_7) = & {l12} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
    // 2210: b"NBCORE\0":   l12 = READ | UNIQUE | OFFSET_ADD | NON_NULL, FIXED
    // 2210: b"NBCORE\0": typeof(_6 = &raw const (*_7)) = *const {l38} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
    // 2210: b"NBCORE\0":   l38 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
    // 2210: b"NBCORE\0": typeof(_7 = const b"NBCORE\x00") = & {l37} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
    // 2210: b"NBCORE\0":   l37 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
    // 2210: b"NBCORE\0" as  ... st u8: typeof(_5 = move _6 as *const u8 (Pointer(ArrayToPointer))) = *const {l39} u8
    // 2210: b"NBCORE\0" as  ... st u8:   l39 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
    // 2210: b"NBCORE\0" as  ... _char: typeof(_4 = move _5 as *const i8 (Misc)) = *const {l40} i8
    // 2210: b"NBCORE\0" as  ... _char:   l40 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
    // 2210: getenv(b"NBCORE ... char): typeof(_2 = move _3 as *const i8 (Pointer(MutToConstPointer))) = *const {l41} i8
    // 2210: getenv(b"NBCORE ... char):   l41 = READ | OFFSET_ADD, (empty)
    if (str).map(|__ptr| &__ptr[0]).is_none() {
    // 2211: str: typeof(_10) = *const {l16} i8
    // 2211: str:   l16 = (empty), (empty)
        return 0 as libc::c_int;
    }
    if isposnum(core::ptr::addr_of!(*(str).map(|__ptr| &__ptr[0]).unwrap())) == 0 {
    // 2214: str: typeof(_15) = *const {l22} i8
    // 2214: str:   l22 = (empty), (empty)
        die(
            core::ptr::addr_of!(*(&*(b"invalid value '%s' for environment variable NBCORE\0") as *const u8
            // 2216: b"invalid value ... _char: typeof(_17) = *const {l25} i8
            // 2216: b"invalid value ... _char:   l25 = UNIQUE | NON_NULL, (empty)
            // 2216: b"invalid value ... st u8: typeof(_18) = *const {l27} u8
            // 2216: b"invalid value ... st u8:   l27 = UNIQUE | NON_NULL, (empty)
            // 2216: b"invalid value ... RE\0": typeof(_19) = *const {l29} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 2216: b"invalid value ... RE\0":   l29 = UNIQUE | NON_NULL, (empty)
            // 2216: b"invalid value ... RE\0": typeof(_20) = & {l31} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 2216: b"invalid value ... RE\0":   l31 = UNIQUE | NON_NULL, FIXED
            // 2216: b"invalid value ... _char: typeof(_17 = move _18 as *const i8 (Misc)) = *const {l45} i8
            // 2216: b"invalid value ... _char:   l45 = UNIQUE | NON_NULL, (empty)
            // 2216: b"invalid value ... RE\0": typeof(_20 = const b"invalid value \'%s\' for environment variable NBCORE\x00") = & {l42} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 2216: b"invalid value ... RE\0":   l42 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
            // 2216: b"invalid value ... st u8: typeof(_18 = move _19 as *const u8 (Pointer(ArrayToPointer))) = *const {l44} u8
            // 2216: b"invalid value ... st u8:   l44 = UNIQUE | NON_NULL, (empty)
            // 2216: b"invalid value ... RE\0": typeof(_19 = &raw const (*_20)) = *const {l43} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 2216: b"invalid value ... RE\0":   l43 = UNIQUE | NON_NULL, (empty)
                as *const libc::c_char)),
            (str).map(|__ptr| &__ptr[0]),
            // 2218: str: typeof(_21) = *const {l33} i8
            // 2218: str:   l33 = (empty), (empty)
        );
    }
    return atoi(core::ptr::addr_of!(*&(str).unwrap()[0]));
    // 2221: str: typeof(_22) = *const {l35} i8
    // 2221: str:   l35 = READ | OFFSET_ADD, (empty)
}
unsafe extern "C" fn setopt(
    mut i: libc::c_int,
    mut lgl: *mut LGL,
    // 2225: mut lgl: typeof(_2) = *mut {g43} LGL
    // 2225: mut lgl:   g43 = UNIQUE | NON_NULL, FIXED
    mut opt: *const libc::c_char,
    // 2226: mut opt: typeof(_3) = *const {g44} i8
    // 2226: mut opt:   g44 = UNIQUE | NON_NULL, FIXED
    mut val: libc::c_int,
) {
    let mut oldval: libc::c_int = 0;
    let mut newval: libc::c_int = 0;
    if lglhasopt(lgl, opt) == 0 {
    // 2231: lgl: typeof(_10) = *mut {l10} LGL
    // 2231: lgl:   l10 = UNIQUE | NON_NULL, (empty)
    // 2231: opt: typeof(_11) = *const {l12} i8
    // 2231: opt:   l12 = UNIQUE | NON_NULL, (empty)
        msg(
            i,
            1 as libc::c_int,
            b"option '%s' does not exist\0" as *const u8 as *const libc::c_char,
            // 2235: b"option '%s' d ... _char: typeof(_15) = *const {l17} i8
            // 2235: b"option '%s' d ... _char:   l17 = UNIQUE | NON_NULL, (empty)
            // 2235: b"option '%s' d ... st u8: typeof(_16) = *const {l19} u8
            // 2235: b"option '%s' d ... st u8:   l19 = UNIQUE | NON_NULL, (empty)
            // 2235: b"option '%s' d ... st\0": typeof(_17) = *const {l21} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
            // 2235: b"option '%s' d ... st\0":   l21 = UNIQUE | NON_NULL, (empty)
            // 2235: b"option '%s' d ... st\0": typeof(_18) = & {l23} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
            // 2235: b"option '%s' d ... st\0":   l23 = UNIQUE | NON_NULL, FIXED
            // 2235: b"option '%s' d ... st\0": typeof(_17 = &raw const (*_18)) = *const {l93} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
            // 2235: b"option '%s' d ... st\0":   l93 = UNIQUE | NON_NULL, (empty)
            // 2235: b"option '%s' d ... st u8: typeof(_16 = move _17 as *const u8 (Pointer(ArrayToPointer))) = *const {l94} u8
            // 2235: b"option '%s' d ... st u8:   l94 = UNIQUE | NON_NULL, (empty)
            // 2235: b"option '%s' d ... st\0": typeof(_18 = const b"option \'%s\' does not exist\x00") = & {l92} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
            // 2235: b"option '%s' d ... st\0":   l92 = UNIQUE | NON_NULL, (empty)
            // 2235: b"option '%s' d ... _char: typeof(_15 = move _16 as *const i8 (Misc)) = *const {l95} i8
            // 2235: b"option '%s' d ... _char:   l95 = UNIQUE | NON_NULL, (empty)
            opt,
            // 2236: opt: typeof(_19) = *const {l25} i8
            // 2236: opt:   l25 = UNIQUE | NON_NULL, (empty)
        );
    } else {
        oldval = lglgetopt(lgl, opt);
        // 2239: lgl: typeof(_21) = *mut {l28} LGL
        // 2239: lgl:   l28 = UNIQUE | NON_NULL, (empty)
        // 2239: opt: typeof(_22) = *const {l30} i8
        // 2239: opt:   l30 = UNIQUE | NON_NULL, (empty)
        if oldval == val {
            msg(
                i,
                1 as libc::c_int,
                b"option '%s' already set to '%d'\0" as *const u8 as *const libc::c_char,
                // 2244: b"option '%s' a ... _char: typeof(_29) = *const {l38} i8
                // 2244: b"option '%s' a ... _char:   l38 = UNIQUE | NON_NULL, (empty)
                // 2244: b"option '%s' a ... st u8: typeof(_30) = *const {l40} u8
                // 2244: b"option '%s' a ... st u8:   l40 = UNIQUE | NON_NULL, (empty)
                // 2244: b"option '%s' a ... d'\0": typeof(_31) = *const {l42} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 2244: b"option '%s' a ... d'\0":   l42 = UNIQUE | NON_NULL, (empty)
                // 2244: b"option '%s' a ... d'\0": typeof(_32) = & {l44} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 2244: b"option '%s' a ... d'\0":   l44 = UNIQUE | NON_NULL, FIXED
                // 2244: b"option '%s' a ... st u8: typeof(_30 = move _31 as *const u8 (Pointer(ArrayToPointer))) = *const {l98} u8
                // 2244: b"option '%s' a ... st u8:   l98 = UNIQUE | NON_NULL, (empty)
                // 2244: b"option '%s' a ... _char: typeof(_29 = move _30 as *const i8 (Misc)) = *const {l99} i8
                // 2244: b"option '%s' a ... _char:   l99 = UNIQUE | NON_NULL, (empty)
                // 2244: b"option '%s' a ... d'\0": typeof(_32 = const b"option \'%s\' already set to \'%d\'\x00") = & {l96} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 2244: b"option '%s' a ... d'\0":   l96 = UNIQUE | NON_NULL, (empty)
                // 2244: b"option '%s' a ... d'\0": typeof(_31 = &raw const (*_32)) = *const {l97} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 2244: b"option '%s' a ... d'\0":   l97 = UNIQUE | NON_NULL, (empty)
                opt,
                // 2245: opt: typeof(_33) = *const {l46} i8
                // 2245: opt:   l46 = UNIQUE | NON_NULL, (empty)
                val,
            );
        } else {
            lglsetopt(lgl, opt, val);
            // 2249: lgl: typeof(_36) = *mut {l50} LGL
            // 2249: lgl:   l50 = UNIQUE | NON_NULL, (empty)
            // 2249: opt: typeof(_37) = *const {l52} i8
            // 2249: opt:   l52 = UNIQUE | NON_NULL, (empty)
            newval = lglgetopt(lgl, opt);
            // 2250: lgl: typeof(_40) = *mut {l56} LGL
            // 2250: lgl:   l56 = UNIQUE | NON_NULL, (empty)
            // 2250: opt: typeof(_41) = *const {l58} i8
            // 2250: opt:   l58 = UNIQUE | NON_NULL, (empty)
            if newval != val {
                msg(
                    i,
                    1 as libc::c_int,
                    b"option '%s' set to '%d' (but requested %d)\0" as *const u8
                    // 2255: b"option '%s' s ... _char: typeof(_48) = *const {l66} i8
                    // 2255: b"option '%s' s ... _char:   l66 = UNIQUE | NON_NULL, (empty)
                    // 2255: b"option '%s' s ... st u8: typeof(_49) = *const {l68} u8
                    // 2255: b"option '%s' s ... st u8:   l68 = UNIQUE | NON_NULL, (empty)
                    // 2255: b"option '%s' s ... d)\0": typeof(_50) = *const {l70} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                    // 2255: b"option '%s' s ... d)\0":   l70 = UNIQUE | NON_NULL, (empty)
                    // 2255: b"option '%s' s ... d)\0": typeof(_51) = & {l72} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                    // 2255: b"option '%s' s ... d)\0":   l72 = UNIQUE | NON_NULL, FIXED
                    // 2255: b"option '%s' s ... st u8: typeof(_49 = move _50 as *const u8 (Pointer(ArrayToPointer))) = *const {l102} u8
                    // 2255: b"option '%s' s ... st u8:   l102 = UNIQUE | NON_NULL, (empty)
                    // 2255: b"option '%s' s ... _char: typeof(_48 = move _49 as *const i8 (Misc)) = *const {l103} i8
                    // 2255: b"option '%s' s ... _char:   l103 = UNIQUE | NON_NULL, (empty)
                    // 2255: b"option '%s' s ... d)\0": typeof(_51 = const b"option \'%s\' set to \'%d\' (but requested %d)\x00") = & {l100} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                    // 2255: b"option '%s' s ... d)\0":   l100 = UNIQUE | NON_NULL, (empty)
                    // 2255: b"option '%s' s ... d)\0": typeof(_50 = &raw const (*_51)) = *const {l101} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                    // 2255: b"option '%s' s ... d)\0":   l101 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                    opt,
                    // 2257: opt: typeof(_52) = *const {l74} i8
                    // 2257: opt:   l74 = UNIQUE | NON_NULL, (empty)
                    newval,
                    val,
                );
            } else {
                msg(
                    i,
                    1 as libc::c_int,
                    b"option '%s' set to '%d'\0" as *const u8 as *const libc::c_char,
                    // 2265: b"option '%s' s ... _char: typeof(_58) = *const {l81} i8
                    // 2265: b"option '%s' s ... _char:   l81 = UNIQUE | NON_NULL, (empty)
                    // 2265: b"option '%s' s ... st u8: typeof(_59) = *const {l83} u8
                    // 2265: b"option '%s' s ... st u8:   l83 = UNIQUE | NON_NULL, (empty)
                    // 2265: b"option '%s' s ... d'\0": typeof(_60) = *const {l85} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
                    // 2265: b"option '%s' s ... d'\0":   l85 = UNIQUE | NON_NULL, (empty)
                    // 2265: b"option '%s' s ... d'\0": typeof(_61) = & {l87} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
                    // 2265: b"option '%s' s ... d'\0":   l87 = UNIQUE | NON_NULL, FIXED
                    // 2265: b"option '%s' s ... d'\0": typeof(_61 = const b"option \'%s\' set to \'%d\'\x00") = & {l104} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
                    // 2265: b"option '%s' s ... d'\0":   l104 = UNIQUE | NON_NULL, (empty)
                    // 2265: b"option '%s' s ... d'\0": typeof(_60 = &raw const (*_61)) = *const {l105} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
                    // 2265: b"option '%s' s ... d'\0":   l105 = UNIQUE | NON_NULL, (empty)
                    // 2265: b"option '%s' s ... st u8: typeof(_59 = move _60 as *const u8 (Pointer(ArrayToPointer))) = *const {l106} u8
                    // 2265: b"option '%s' s ... st u8:   l106 = UNIQUE | NON_NULL, (empty)
                    // 2265: b"option '%s' s ... _char: typeof(_58 = move _59 as *const i8 (Misc)) = *const {l107} i8
                    // 2265: b"option '%s' s ... _char:   l107 = UNIQUE | NON_NULL, (empty)
                    opt,
                    // 2266: opt: typeof(_62) = *const {l89} i8
                    // 2266: opt:   l89 = UNIQUE | NON_NULL, (empty)
                    val,
                );
            }
        }
    };
}
unsafe extern "C" fn setopts(mut lgl: *mut LGL, mut i: libc::c_int) {
// 2273: mut lgl: typeof(_1) = *mut {g45} LGL
// 2273: mut lgl:   g45 = UNIQUE | NON_NULL, FIXED
    let mut w: *mut Worker = workers.offset(i as isize);
    // 2274: mut w: typeof(_3) = *mut {l3} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2274: mut w:   l3 = UNIQUE | NON_NULL, (empty)
    // 2274: workers: typeof(_4) = *mut {l5} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2274: workers:   l5 = UNIQUE | NON_NULL, (empty)
    // 2274: workers: typeof(_5) = *mut {l7} *mut {l8} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2274: workers:   l7 = UNIQUE | NON_NULL, (empty)
    // 2274: workers:   l8 = UNIQUE | NON_NULL, (empty)
    let mut j: libc::c_int = 0;
    (*w).lgl = lgl;
    // 2276: lgl: typeof(_9) = *mut {l13} LGL
    // 2276: lgl:   l13 = UNIQUE | NON_NULL, (empty)
    lglsetid(lgl, i, nworkers);
    // 2277: lgl: typeof(_11) = *mut {l16} LGL
    // 2277: lgl:   l16 = UNIQUE | NON_NULL, (empty)
    // 2277: nworkers: typeof(_14) = *mut {l20} i32
    // 2277: nworkers:   l20 = UNIQUE | NON_NULL, (empty)
    lglsetime(
        lgl,
        // 2279: lgl: typeof(_16) = *mut {l23} LGL
        // 2279: lgl:   l23 = UNIQUE | NON_NULL, (empty)
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
        // 2290: lgl: typeof(_23) = *mut {l31} LGL
        // 2290: lgl:   l31 = UNIQUE | NON_NULL, (empty)
        b"verbose\0" as *const u8 as *const libc::c_char,
        // 2291: b"verbose\0" as ... _char: typeof(_24) = *const {l33} i8
        // 2291: b"verbose\0" as ... _char:   l33 = UNIQUE | NON_NULL, (empty)
        // 2291: b"verbose\0" as ... st u8: typeof(_25) = *const {l35} u8
        // 2291: b"verbose\0" as ... st u8:   l35 = UNIQUE | NON_NULL, (empty)
        // 2291: b"verbose\0": typeof(_26) = *const {l37} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 2291: b"verbose\0":   l37 = UNIQUE | NON_NULL, (empty)
        // 2291: b"verbose\0": typeof(_27) = & {l39} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 2291: b"verbose\0":   l39 = UNIQUE | NON_NULL, FIXED
        // 2291: b"verbose\0" as ... _char: typeof(_24 = move _25 as *const i8 (Misc)) = *const {l550} i8
        // 2291: b"verbose\0" as ... _char:   l550 = UNIQUE | NON_NULL, (empty)
        // 2291: b"verbose\0": typeof(_26 = &raw const (*_27)) = *const {l548} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 2291: b"verbose\0":   l548 = UNIQUE | NON_NULL, (empty)
        // 2291: b"verbose\0" as ... st u8: typeof(_25 = move _26 as *const u8 (Pointer(ArrayToPointer))) = *const {l549} u8
        // 2291: b"verbose\0" as ... st u8:   l549 = UNIQUE | NON_NULL, (empty)
        // 2291: b"verbose\0": typeof(_27 = const b"verbose\x00") = & {l547} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 2291: b"verbose\0":   l547 = UNIQUE | NON_NULL, (empty)
        verbose,
        // 2292: verbose: typeof(_29) = *mut {l42} i32
        // 2292: verbose:   l42 = UNIQUE | NON_NULL, (empty)
    );
    setopt(i, lgl, b"seed\0" as *const u8 as *const libc::c_char, i);
    // 2294: lgl: typeof(_32) = *mut {l46} LGL
    // 2294: lgl:   l46 = UNIQUE | NON_NULL, (empty)
    // 2294: b"seed\0" as *c ... _char: typeof(_33) = *const {l48} i8
    // 2294: b"seed\0" as *c ... _char:   l48 = UNIQUE | NON_NULL, (empty)
    // 2294: b"seed\0" as *c ... st u8: typeof(_34) = *const {l50} u8
    // 2294: b"seed\0" as *c ... st u8:   l50 = UNIQUE | NON_NULL, (empty)
    // 2294: b"seed\0": typeof(_35) = *const {l52} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
    // 2294: b"seed\0":   l52 = UNIQUE | NON_NULL, (empty)
    // 2294: b"seed\0": typeof(_36) = & {l54} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
    // 2294: b"seed\0":   l54 = UNIQUE | NON_NULL, FIXED
    // 2294: b"seed\0": typeof(_36 = const b"seed\x00") = & {l551} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
    // 2294: b"seed\0":   l551 = UNIQUE | NON_NULL, (empty)
    // 2294: b"seed\0" as *c ... _char: typeof(_33 = move _34 as *const i8 (Misc)) = *const {l554} i8
    // 2294: b"seed\0" as *c ... _char:   l554 = UNIQUE | NON_NULL, (empty)
    // 2294: b"seed\0": typeof(_35 = &raw const (*_36)) = *const {l552} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
    // 2294: b"seed\0":   l552 = UNIQUE | NON_NULL, (empty)
    // 2294: b"seed\0" as *c ... st u8: typeof(_34 = move _35 as *const u8 (Pointer(ArrayToPointer))) = *const {l553} u8
    // 2294: b"seed\0" as *c ... st u8:   l553 = UNIQUE | NON_NULL, (empty)
    setopt(
        i,
        lgl,
        // 2297: lgl: typeof(_40) = *mut {l59} LGL
        // 2297: lgl:   l59 = UNIQUE | NON_NULL, (empty)
        b"classify\0" as *const u8 as *const libc::c_char,
        // 2298: b"classify\0" a ... _char: typeof(_41) = *const {l61} i8
        // 2298: b"classify\0" a ... _char:   l61 = UNIQUE | NON_NULL, (empty)
        // 2298: b"classify\0" a ... st u8: typeof(_42) = *const {l63} u8
        // 2298: b"classify\0" a ... st u8:   l63 = UNIQUE | NON_NULL, (empty)
        // 2298: b"classify\0": typeof(_43) = *const {l65} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 2298: b"classify\0":   l65 = UNIQUE | NON_NULL, (empty)
        // 2298: b"classify\0": typeof(_44) = & {l67} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 2298: b"classify\0":   l67 = UNIQUE | NON_NULL, FIXED
        // 2298: b"classify\0" a ... _char: typeof(_41 = move _42 as *const i8 (Misc)) = *const {l558} i8
        // 2298: b"classify\0" a ... _char:   l558 = UNIQUE | NON_NULL, (empty)
        // 2298: b"classify\0": typeof(_44 = const b"classify\x00") = & {l555} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 2298: b"classify\0":   l555 = UNIQUE | NON_NULL, (empty)
        // 2298: b"classify\0" a ... st u8: typeof(_42 = move _43 as *const u8 (Pointer(ArrayToPointer))) = *const {l557} u8
        // 2298: b"classify\0" a ... st u8:   l557 = UNIQUE | NON_NULL, (empty)
        // 2298: b"classify\0": typeof(_43 = &raw const (*_44)) = *const {l556} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 2298: b"classify\0":   l556 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int,
    );
    if i != 0
        && (locs >= 2 as libc::c_int
        // 2302: locs: typeof(_53) = *mut {l77} i32
        // 2302: locs:   l77 = UNIQUE | NON_NULL, (empty)
            || locs != 0 && (i > 7 as libc::c_int || i & 1 as libc::c_int != 0))
            // 2303: locs: typeof(_58) = *mut {l83} i32
            // 2303: locs:   l83 = UNIQUE | NON_NULL, (empty)
    {
        setopt(
            i,
            lgl,
            // 2307: lgl: typeof(_69) = *mut {l95} LGL
            // 2307: lgl:   l95 = UNIQUE | NON_NULL, (empty)
            b"plain\0" as *const u8 as *const libc::c_char,
            // 2308: b"plain\0" as * ... _char: typeof(_70) = *const {l97} i8
            // 2308: b"plain\0" as * ... _char:   l97 = UNIQUE | NON_NULL, (empty)
            // 2308: b"plain\0" as * ... st u8: typeof(_71) = *const {l99} u8
            // 2308: b"plain\0" as * ... st u8:   l99 = UNIQUE | NON_NULL, (empty)
            // 2308: b"plain\0": typeof(_72) = *const {l101} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2308: b"plain\0":   l101 = UNIQUE | NON_NULL, (empty)
            // 2308: b"plain\0": typeof(_73) = & {l103} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2308: b"plain\0":   l103 = UNIQUE | NON_NULL, FIXED
            // 2308: b"plain\0" as * ... _char: typeof(_70 = move _71 as *const i8 (Misc)) = *const {l562} i8
            // 2308: b"plain\0" as * ... _char:   l562 = UNIQUE | NON_NULL, (empty)
            // 2308: b"plain\0": typeof(_72 = &raw const (*_73)) = *const {l560} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2308: b"plain\0":   l560 = UNIQUE | NON_NULL, (empty)
            // 2308: b"plain\0": typeof(_73 = const b"plain\x00") = & {l559} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2308: b"plain\0":   l559 = UNIQUE | NON_NULL, (empty)
            // 2308: b"plain\0" as * ... st u8: typeof(_71 = move _72 as *const u8 (Pointer(ArrayToPointer))) = *const {l561} u8
            // 2308: b"plain\0" as * ... st u8:   l561 = UNIQUE | NON_NULL, (empty)
            (locs == 2 as libc::c_int || i & 3 as libc::c_int == 1 as libc::c_int) as libc::c_int,
            // 2309: locs: typeof(_78) = *mut {l109} i32
            // 2309: locs:   l109 = UNIQUE | NON_NULL, (empty)
        );
        setopt(
            i,
            lgl,
            // 2313: lgl: typeof(_87) = *mut {l119} LGL
            // 2313: lgl:   l119 = UNIQUE | NON_NULL, (empty)
            b"locs\0" as *const u8 as *const libc::c_char,
            // 2314: b"locs\0" as *c ... _char: typeof(_88) = *const {l121} i8
            // 2314: b"locs\0" as *c ... _char:   l121 = UNIQUE | NON_NULL, (empty)
            // 2314: b"locs\0" as *c ... st u8: typeof(_89) = *const {l123} u8
            // 2314: b"locs\0" as *c ... st u8:   l123 = UNIQUE | NON_NULL, (empty)
            // 2314: b"locs\0": typeof(_90) = *const {l125} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2314: b"locs\0":   l125 = UNIQUE | NON_NULL, (empty)
            // 2314: b"locs\0": typeof(_91) = & {l127} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2314: b"locs\0":   l127 = UNIQUE | NON_NULL, FIXED
            // 2314: b"locs\0": typeof(_91 = const b"locs\x00") = & {l563} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2314: b"locs\0":   l563 = UNIQUE | NON_NULL, (empty)
            // 2314: b"locs\0": typeof(_90 = &raw const (*_91)) = *const {l564} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2314: b"locs\0":   l564 = UNIQUE | NON_NULL, (empty)
            // 2314: b"locs\0" as *c ... _char: typeof(_88 = move _89 as *const i8 (Misc)) = *const {l566} i8
            // 2314: b"locs\0" as *c ... _char:   l566 = UNIQUE | NON_NULL, (empty)
            // 2314: b"locs\0" as *c ... st u8: typeof(_89 = move _90 as *const u8 (Pointer(ArrayToPointer))) = *const {l565} u8
            // 2314: b"locs\0" as *c ... st u8:   l565 = UNIQUE | NON_NULL, (empty)
            -(1 as libc::c_int),
        );
        setopt(
            i,
            lgl,
            // 2319: lgl: typeof(_97) = *mut {l134} LGL
            // 2319: lgl:   l134 = UNIQUE | NON_NULL, (empty)
            b"locsrtc\0" as *const u8 as *const libc::c_char,
            // 2320: b"locsrtc\0" as ... _char: typeof(_98) = *const {l136} i8
            // 2320: b"locsrtc\0" as ... _char:   l136 = UNIQUE | NON_NULL, (empty)
            // 2320: b"locsrtc\0" as ... st u8: typeof(_99) = *const {l138} u8
            // 2320: b"locsrtc\0" as ... st u8:   l138 = UNIQUE | NON_NULL, (empty)
            // 2320: b"locsrtc\0": typeof(_100) = *const {l140} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 2320: b"locsrtc\0":   l140 = UNIQUE | NON_NULL, (empty)
            // 2320: b"locsrtc\0": typeof(_101) = & {l142} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 2320: b"locsrtc\0":   l142 = UNIQUE | NON_NULL, FIXED
            // 2320: b"locsrtc\0" as ... _char: typeof(_98 = move _99 as *const i8 (Misc)) = *const {l570} i8
            // 2320: b"locsrtc\0" as ... _char:   l570 = UNIQUE | NON_NULL, (empty)
            // 2320: b"locsrtc\0": typeof(_100 = &raw const (*_101)) = *const {l568} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 2320: b"locsrtc\0":   l568 = UNIQUE | NON_NULL, (empty)
            // 2320: b"locsrtc\0" as ... st u8: typeof(_99 = move _100 as *const u8 (Pointer(ArrayToPointer))) = *const {l569} u8
            // 2320: b"locsrtc\0" as ... st u8:   l569 = UNIQUE | NON_NULL, (empty)
            // 2320: b"locsrtc\0": typeof(_101 = const b"locsrtc\x00") = & {l567} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 2320: b"locsrtc\0":   l567 = UNIQUE | NON_NULL, (empty)
            1 as libc::c_int,
        );
        setopt(
            i,
            lgl,
            // 2325: lgl: typeof(_105) = *mut {l147} LGL
            // 2325: lgl:   l147 = UNIQUE | NON_NULL, (empty)
            b"locswait\0" as *const u8 as *const libc::c_char,
            // 2326: b"locswait\0" a ... _char: typeof(_106) = *const {l149} i8
            // 2326: b"locswait\0" a ... _char:   l149 = UNIQUE | NON_NULL, (empty)
            // 2326: b"locswait\0" a ... st u8: typeof(_107) = *const {l151} u8
            // 2326: b"locswait\0" a ... st u8:   l151 = UNIQUE | NON_NULL, (empty)
            // 2326: b"locswait\0": typeof(_108) = *const {l153} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2326: b"locswait\0":   l153 = UNIQUE | NON_NULL, (empty)
            // 2326: b"locswait\0": typeof(_109) = & {l155} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2326: b"locswait\0":   l155 = UNIQUE | NON_NULL, FIXED
            // 2326: b"locswait\0" a ... st u8: typeof(_107 = move _108 as *const u8 (Pointer(ArrayToPointer))) = *const {l573} u8
            // 2326: b"locswait\0" a ... st u8:   l573 = UNIQUE | NON_NULL, (empty)
            // 2326: b"locswait\0": typeof(_108 = &raw const (*_109)) = *const {l572} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2326: b"locswait\0":   l572 = UNIQUE | NON_NULL, (empty)
            // 2326: b"locswait\0": typeof(_109 = const b"locswait\x00") = & {l571} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2326: b"locswait\0":   l571 = UNIQUE | NON_NULL, (empty)
            // 2326: b"locswait\0" a ... _char: typeof(_106 = move _107 as *const i8 (Misc)) = *const {l574} i8
            // 2326: b"locswait\0" a ... _char:   l574 = UNIQUE | NON_NULL, (empty)
            0 as libc::c_int,
        );
        setopt(
            i,
            lgl,
            // 2331: lgl: typeof(_113) = *mut {l160} LGL
            // 2331: lgl:   l160 = UNIQUE | NON_NULL, (empty)
            b"locsclim\0" as *const u8 as *const libc::c_char,
            // 2332: b"locsclim\0" a ... _char: typeof(_114) = *const {l162} i8
            // 2332: b"locsclim\0" a ... _char:   l162 = UNIQUE | NON_NULL, (empty)
            // 2332: b"locsclim\0" a ... st u8: typeof(_115) = *const {l164} u8
            // 2332: b"locsclim\0" a ... st u8:   l164 = UNIQUE | NON_NULL, (empty)
            // 2332: b"locsclim\0": typeof(_116) = *const {l166} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2332: b"locsclim\0":   l166 = UNIQUE | NON_NULL, (empty)
            // 2332: b"locsclim\0": typeof(_117) = & {l168} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2332: b"locsclim\0":   l168 = UNIQUE | NON_NULL, FIXED
            // 2332: b"locsclim\0": typeof(_117 = const b"locsclim\x00") = & {l575} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2332: b"locsclim\0":   l575 = UNIQUE | NON_NULL, (empty)
            // 2332: b"locsclim\0" a ... _char: typeof(_114 = move _115 as *const i8 (Misc)) = *const {l578} i8
            // 2332: b"locsclim\0" a ... _char:   l578 = UNIQUE | NON_NULL, (empty)
            // 2332: b"locsclim\0" a ... st u8: typeof(_115 = move _116 as *const u8 (Pointer(ArrayToPointer))) = *const {l577} u8
            // 2332: b"locsclim\0" a ... st u8:   l577 = UNIQUE | NON_NULL, (empty)
            // 2332: b"locsclim\0": typeof(_116 = &raw const (*_117)) = *const {l576} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2332: b"locsclim\0":   l576 = UNIQUE | NON_NULL, (empty)
            (1 as libc::c_int) << 24 as libc::c_int,
        );
    } else {
        j = if locs != 0 { i / 2 as libc::c_int } else { i };
        // 2336: locs: typeof(_125) = *mut {l177} i32
        // 2336: locs:   l177 = UNIQUE | NON_NULL, (empty)
        match j % 13 as libc::c_int {
            1 => {
                setopt(
                    i,
                    lgl,
                    // 2341: lgl: typeof(_141) = *mut {l194} LGL
                    // 2341: lgl:   l194 = UNIQUE | NON_NULL, (empty)
                    b"plain\0" as *const u8 as *const libc::c_char,
                    // 2342: b"plain\0" as * ... _char: typeof(_142) = *const {l196} i8
                    // 2342: b"plain\0" as * ... _char:   l196 = UNIQUE | NON_NULL, (empty)
                    // 2342: b"plain\0" as * ... st u8: typeof(_143) = *const {l198} u8
                    // 2342: b"plain\0" as * ... st u8:   l198 = UNIQUE | NON_NULL, (empty)
                    // 2342: b"plain\0": typeof(_144) = *const {l200} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2342: b"plain\0":   l200 = UNIQUE | NON_NULL, (empty)
                    // 2342: b"plain\0": typeof(_145) = & {l202} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2342: b"plain\0":   l202 = UNIQUE | NON_NULL, FIXED
                    // 2342: b"plain\0" as * ... _char: typeof(_142 = move _143 as *const i8 (Misc)) = *const {l582} i8
                    // 2342: b"plain\0" as * ... _char:   l582 = UNIQUE | NON_NULL, (empty)
                    // 2342: b"plain\0" as * ... st u8: typeof(_143 = move _144 as *const u8 (Pointer(ArrayToPointer))) = *const {l581} u8
                    // 2342: b"plain\0" as * ... st u8:   l581 = UNIQUE | NON_NULL, (empty)
                    // 2342: b"plain\0": typeof(_144 = &raw const (*_145)) = *const {l580} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2342: b"plain\0":   l580 = UNIQUE | NON_NULL, (empty)
                    // 2342: b"plain\0": typeof(_145 = const b"plain\x00") = & {l579} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2342: b"plain\0":   l579 = UNIQUE | NON_NULL, (empty)
                    1 as libc::c_int,
                );
                setopt(
                    i,
                    lgl,
                    // 2347: lgl: typeof(_149) = *mut {l207} LGL
                    // 2347: lgl:   l207 = UNIQUE | NON_NULL, (empty)
                    b"decompose\0" as *const u8 as *const libc::c_char,
                    // 2348: b"decompose\0"  ... _char: typeof(_150) = *const {l209} i8
                    // 2348: b"decompose\0"  ... _char:   l209 = UNIQUE | NON_NULL, (empty)
                    // 2348: b"decompose\0"  ... st u8: typeof(_151) = *const {l211} u8
                    // 2348: b"decompose\0"  ... st u8:   l211 = UNIQUE | NON_NULL, (empty)
                    // 2348: b"decompose\0": typeof(_152) = *const {l213} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                    // 2348: b"decompose\0":   l213 = UNIQUE | NON_NULL, (empty)
                    // 2348: b"decompose\0": typeof(_153) = & {l215} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                    // 2348: b"decompose\0":   l215 = UNIQUE | NON_NULL, FIXED
                    // 2348: b"decompose\0"  ... st u8: typeof(_151 = move _152 as *const u8 (Pointer(ArrayToPointer))) = *const {l585} u8
                    // 2348: b"decompose\0"  ... st u8:   l585 = UNIQUE | NON_NULL, (empty)
                    // 2348: b"decompose\0"  ... _char: typeof(_150 = move _151 as *const i8 (Misc)) = *const {l586} i8
                    // 2348: b"decompose\0"  ... _char:   l586 = UNIQUE | NON_NULL, (empty)
                    // 2348: b"decompose\0": typeof(_153 = const b"decompose\x00") = & {l583} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                    // 2348: b"decompose\0":   l583 = UNIQUE | NON_NULL, (empty)
                    // 2348: b"decompose\0": typeof(_152 = &raw const (*_153)) = *const {l584} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                    // 2348: b"decompose\0":   l584 = UNIQUE | NON_NULL, (empty)
                    1 as libc::c_int,
                );
            }
            2 => {
                setopt(
                    i,
                    lgl,
                    // 2355: lgl: typeof(_157) = *mut {l220} LGL
                    // 2355: lgl:   l220 = UNIQUE | NON_NULL, (empty)
                    b"restartint\0" as *const u8 as *const libc::c_char,
                    // 2356: b"restartint\0" ... _char: typeof(_158) = *const {l222} i8
                    // 2356: b"restartint\0" ... _char:   l222 = UNIQUE | NON_NULL, (empty)
                    // 2356: b"restartint\0" ... st u8: typeof(_159) = *const {l224} u8
                    // 2356: b"restartint\0" ... st u8:   l224 = UNIQUE | NON_NULL, (empty)
                    // 2356: b"restartint\0": typeof(_160) = *const {l226} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2356: b"restartint\0":   l226 = UNIQUE | NON_NULL, (empty)
                    // 2356: b"restartint\0": typeof(_161) = & {l228} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2356: b"restartint\0":   l228 = UNIQUE | NON_NULL, FIXED
                    // 2356: b"restartint\0": typeof(_161 = const b"restartint\x00") = & {l587} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2356: b"restartint\0":   l587 = UNIQUE | NON_NULL, (empty)
                    // 2356: b"restartint\0": typeof(_160 = &raw const (*_161)) = *const {l588} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2356: b"restartint\0":   l588 = UNIQUE | NON_NULL, (empty)
                    // 2356: b"restartint\0" ... st u8: typeof(_159 = move _160 as *const u8 (Pointer(ArrayToPointer))) = *const {l589} u8
                    // 2356: b"restartint\0" ... st u8:   l589 = UNIQUE | NON_NULL, (empty)
                    // 2356: b"restartint\0" ... _char: typeof(_158 = move _159 as *const i8 (Misc)) = *const {l590} i8
                    // 2356: b"restartint\0" ... _char:   l590 = UNIQUE | NON_NULL, (empty)
                    1000 as libc::c_int,
                );
            }
            3 => {
                setopt(
                    i,
                    lgl,
                    // 2363: lgl: typeof(_165) = *mut {l233} LGL
                    // 2363: lgl:   l233 = UNIQUE | NON_NULL, (empty)
                    b"elmresched\0" as *const u8 as *const libc::c_char,
                    // 2364: b"elmresched\0" ... _char: typeof(_166) = *const {l235} i8
                    // 2364: b"elmresched\0" ... _char:   l235 = UNIQUE | NON_NULL, (empty)
                    // 2364: b"elmresched\0" ... st u8: typeof(_167) = *const {l237} u8
                    // 2364: b"elmresched\0" ... st u8:   l237 = UNIQUE | NON_NULL, (empty)
                    // 2364: b"elmresched\0": typeof(_168) = *const {l239} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2364: b"elmresched\0":   l239 = UNIQUE | NON_NULL, (empty)
                    // 2364: b"elmresched\0": typeof(_169) = & {l241} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2364: b"elmresched\0":   l241 = UNIQUE | NON_NULL, FIXED
                    // 2364: b"elmresched\0" ... st u8: typeof(_167 = move _168 as *const u8 (Pointer(ArrayToPointer))) = *const {l593} u8
                    // 2364: b"elmresched\0" ... st u8:   l593 = UNIQUE | NON_NULL, (empty)
                    // 2364: b"elmresched\0": typeof(_169 = const b"elmresched\x00") = & {l591} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2364: b"elmresched\0":   l591 = UNIQUE | NON_NULL, (empty)
                    // 2364: b"elmresched\0": typeof(_168 = &raw const (*_169)) = *const {l592} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2364: b"elmresched\0":   l592 = UNIQUE | NON_NULL, (empty)
                    // 2364: b"elmresched\0" ... _char: typeof(_166 = move _167 as *const i8 (Misc)) = *const {l594} i8
                    // 2364: b"elmresched\0" ... _char:   l594 = UNIQUE | NON_NULL, (empty)
                    7 as libc::c_int,
                );
            }
            4 => {
                setopt(
                    i,
                    lgl,
                    // 2371: lgl: typeof(_173) = *mut {l246} LGL
                    // 2371: lgl:   l246 = UNIQUE | NON_NULL, (empty)
                    b"scincincmin\0" as *const u8 as *const libc::c_char,
                    // 2372: b"scincincmin\0 ... _char: typeof(_174) = *const {l248} i8
                    // 2372: b"scincincmin\0 ... _char:   l248 = UNIQUE | NON_NULL, (empty)
                    // 2372: b"scincincmin\0 ... st u8: typeof(_175) = *const {l250} u8
                    // 2372: b"scincincmin\0 ... st u8:   l250 = UNIQUE | NON_NULL, (empty)
                    // 2372: b"scincincmin\0": typeof(_176) = *const {l252} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                    // 2372: b"scincincmin\0":   l252 = UNIQUE | NON_NULL, (empty)
                    // 2372: b"scincincmin\0": typeof(_177) = & {l254} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                    // 2372: b"scincincmin\0":   l254 = UNIQUE | NON_NULL, FIXED
                    // 2372: b"scincincmin\0": typeof(_176 = &raw const (*_177)) = *const {l596} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                    // 2372: b"scincincmin\0":   l596 = UNIQUE | NON_NULL, (empty)
                    // 2372: b"scincincmin\0 ... st u8: typeof(_175 = move _176 as *const u8 (Pointer(ArrayToPointer))) = *const {l597} u8
                    // 2372: b"scincincmin\0 ... st u8:   l597 = UNIQUE | NON_NULL, (empty)
                    // 2372: b"scincincmin\0": typeof(_177 = const b"scincincmin\x00") = & {l595} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                    // 2372: b"scincincmin\0":   l595 = UNIQUE | NON_NULL, (empty)
                    // 2372: b"scincincmin\0 ... _char: typeof(_174 = move _175 as *const i8 (Misc)) = *const {l598} i8
                    // 2372: b"scincincmin\0 ... _char:   l598 = UNIQUE | NON_NULL, (empty)
                    250 as libc::c_int,
                );
            }
            5 => {
                setopt(
                    i,
                    lgl,
                    // 2379: lgl: typeof(_181) = *mut {l259} LGL
                    // 2379: lgl:   l259 = UNIQUE | NON_NULL, (empty)
                    b"block\0" as *const u8 as *const libc::c_char,
                    // 2380: b"block\0" as * ... _char: typeof(_182) = *const {l261} i8
                    // 2380: b"block\0" as * ... _char:   l261 = UNIQUE | NON_NULL, (empty)
                    // 2380: b"block\0" as * ... st u8: typeof(_183) = *const {l263} u8
                    // 2380: b"block\0" as * ... st u8:   l263 = UNIQUE | NON_NULL, (empty)
                    // 2380: b"block\0": typeof(_184) = *const {l265} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2380: b"block\0":   l265 = UNIQUE | NON_NULL, (empty)
                    // 2380: b"block\0": typeof(_185) = & {l267} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2380: b"block\0":   l267 = UNIQUE | NON_NULL, FIXED
                    // 2380: b"block\0" as * ... _char: typeof(_182 = move _183 as *const i8 (Misc)) = *const {l602} i8
                    // 2380: b"block\0" as * ... _char:   l602 = UNIQUE | NON_NULL, (empty)
                    // 2380: b"block\0": typeof(_185 = const b"block\x00") = & {l599} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2380: b"block\0":   l599 = UNIQUE | NON_NULL, (empty)
                    // 2380: b"block\0" as * ... st u8: typeof(_183 = move _184 as *const u8 (Pointer(ArrayToPointer))) = *const {l601} u8
                    // 2380: b"block\0" as * ... st u8:   l601 = UNIQUE | NON_NULL, (empty)
                    // 2380: b"block\0": typeof(_184 = &raw const (*_185)) = *const {l600} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2380: b"block\0":   l600 = UNIQUE | NON_NULL, (empty)
                    0 as libc::c_int,
                );
                setopt(
                    i,
                    lgl,
                    // 2385: lgl: typeof(_189) = *mut {l272} LGL
                    // 2385: lgl:   l272 = UNIQUE | NON_NULL, (empty)
                    b"cce\0" as *const u8 as *const libc::c_char,
                    // 2386: b"cce\0" as *co ... _char: typeof(_190) = *const {l274} i8
                    // 2386: b"cce\0" as *co ... _char:   l274 = UNIQUE | NON_NULL, (empty)
                    // 2386: b"cce\0" as *co ... st u8: typeof(_191) = *const {l276} u8
                    // 2386: b"cce\0" as *co ... st u8:   l276 = UNIQUE | NON_NULL, (empty)
                    // 2386: b"cce\0": typeof(_192) = *const {l278} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 2386: b"cce\0":   l278 = UNIQUE | NON_NULL, (empty)
                    // 2386: b"cce\0": typeof(_193) = & {l280} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 2386: b"cce\0":   l280 = UNIQUE | NON_NULL, FIXED
                    // 2386: b"cce\0" as *co ... st u8: typeof(_191 = move _192 as *const u8 (Pointer(ArrayToPointer))) = *const {l605} u8
                    // 2386: b"cce\0" as *co ... st u8:   l605 = UNIQUE | NON_NULL, (empty)
                    // 2386: b"cce\0": typeof(_193 = const b"cce\x00") = & {l603} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 2386: b"cce\0":   l603 = UNIQUE | NON_NULL, (empty)
                    // 2386: b"cce\0" as *co ... _char: typeof(_190 = move _191 as *const i8 (Misc)) = *const {l606} i8
                    // 2386: b"cce\0" as *co ... _char:   l606 = UNIQUE | NON_NULL, (empty)
                    // 2386: b"cce\0": typeof(_192 = &raw const (*_193)) = *const {l604} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 2386: b"cce\0":   l604 = UNIQUE | NON_NULL, (empty)
                    0 as libc::c_int,
                );
            }
            6 => {
                setopt(
                    i,
                    lgl,
                    // 2393: lgl: typeof(_197) = *mut {l285} LGL
                    // 2393: lgl:   l285 = UNIQUE | NON_NULL, (empty)
                    b"scincinc\0" as *const u8 as *const libc::c_char,
                    // 2394: b"scincinc\0" a ... _char: typeof(_198) = *const {l287} i8
                    // 2394: b"scincinc\0" a ... _char:   l287 = UNIQUE | NON_NULL, (empty)
                    // 2394: b"scincinc\0" a ... st u8: typeof(_199) = *const {l289} u8
                    // 2394: b"scincinc\0" a ... st u8:   l289 = UNIQUE | NON_NULL, (empty)
                    // 2394: b"scincinc\0": typeof(_200) = *const {l291} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 2394: b"scincinc\0":   l291 = UNIQUE | NON_NULL, (empty)
                    // 2394: b"scincinc\0": typeof(_201) = & {l293} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 2394: b"scincinc\0":   l293 = UNIQUE | NON_NULL, FIXED
                    // 2394: b"scincinc\0": typeof(_200 = &raw const (*_201)) = *const {l608} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 2394: b"scincinc\0":   l608 = UNIQUE | NON_NULL, (empty)
                    // 2394: b"scincinc\0": typeof(_201 = const b"scincinc\x00") = & {l607} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 2394: b"scincinc\0":   l607 = UNIQUE | NON_NULL, (empty)
                    // 2394: b"scincinc\0" a ... st u8: typeof(_199 = move _200 as *const u8 (Pointer(ArrayToPointer))) = *const {l609} u8
                    // 2394: b"scincinc\0" a ... st u8:   l609 = UNIQUE | NON_NULL, (empty)
                    // 2394: b"scincinc\0" a ... _char: typeof(_198 = move _199 as *const i8 (Misc)) = *const {l610} i8
                    // 2394: b"scincinc\0" a ... _char:   l610 = UNIQUE | NON_NULL, (empty)
                    50 as libc::c_int,
                );
            }
            7 => {
                setopt(
                    i,
                    lgl,
                    // 2401: lgl: typeof(_205) = *mut {l298} LGL
                    // 2401: lgl:   l298 = UNIQUE | NON_NULL, (empty)
                    b"phase\0" as *const u8 as *const libc::c_char,
                    // 2402: b"phase\0" as * ... _char: typeof(_206) = *const {l300} i8
                    // 2402: b"phase\0" as * ... _char:   l300 = UNIQUE | NON_NULL, (empty)
                    // 2402: b"phase\0" as * ... st u8: typeof(_207) = *const {l302} u8
                    // 2402: b"phase\0" as * ... st u8:   l302 = UNIQUE | NON_NULL, (empty)
                    // 2402: b"phase\0": typeof(_208) = *const {l304} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2402: b"phase\0":   l304 = UNIQUE | NON_NULL, (empty)
                    // 2402: b"phase\0": typeof(_209) = & {l306} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2402: b"phase\0":   l306 = UNIQUE | NON_NULL, FIXED
                    // 2402: b"phase\0" as * ... st u8: typeof(_207 = move _208 as *const u8 (Pointer(ArrayToPointer))) = *const {l613} u8
                    // 2402: b"phase\0" as * ... st u8:   l613 = UNIQUE | NON_NULL, (empty)
                    // 2402: b"phase\0": typeof(_208 = &raw const (*_209)) = *const {l612} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2402: b"phase\0":   l612 = UNIQUE | NON_NULL, (empty)
                    // 2402: b"phase\0": typeof(_209 = const b"phase\x00") = & {l611} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2402: b"phase\0":   l611 = UNIQUE | NON_NULL, (empty)
                    // 2402: b"phase\0" as * ... _char: typeof(_206 = move _207 as *const i8 (Misc)) = *const {l614} i8
                    // 2402: b"phase\0" as * ... _char:   l614 = UNIQUE | NON_NULL, (empty)
                    -(1 as libc::c_int),
                );
            }
            8 => {
                setopt(
                    i,
                    lgl,
                    // 2409: lgl: typeof(_215) = *mut {l313} LGL
                    // 2409: lgl:   l313 = UNIQUE | NON_NULL, (empty)
                    b"phase\0" as *const u8 as *const libc::c_char,
                    // 2410: b"phase\0" as * ... _char: typeof(_216) = *const {l315} i8
                    // 2410: b"phase\0" as * ... _char:   l315 = UNIQUE | NON_NULL, (empty)
                    // 2410: b"phase\0" as * ... st u8: typeof(_217) = *const {l317} u8
                    // 2410: b"phase\0" as * ... st u8:   l317 = UNIQUE | NON_NULL, (empty)
                    // 2410: b"phase\0": typeof(_218) = *const {l319} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2410: b"phase\0":   l319 = UNIQUE | NON_NULL, (empty)
                    // 2410: b"phase\0": typeof(_219) = & {l321} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2410: b"phase\0":   l321 = UNIQUE | NON_NULL, FIXED
                    // 2410: b"phase\0": typeof(_219 = const b"phase\x00") = & {l615} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2410: b"phase\0":   l615 = UNIQUE | NON_NULL, (empty)
                    // 2410: b"phase\0" as * ... _char: typeof(_216 = move _217 as *const i8 (Misc)) = *const {l618} i8
                    // 2410: b"phase\0" as * ... _char:   l618 = UNIQUE | NON_NULL, (empty)
                    // 2410: b"phase\0": typeof(_218 = &raw const (*_219)) = *const {l616} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2410: b"phase\0":   l616 = UNIQUE | NON_NULL, (empty)
                    // 2410: b"phase\0" as * ... st u8: typeof(_217 = move _218 as *const u8 (Pointer(ArrayToPointer))) = *const {l617} u8
                    // 2410: b"phase\0" as * ... st u8:   l617 = UNIQUE | NON_NULL, (empty)
                    1 as libc::c_int,
                );
            }
            9 => {
                setopt(
                    i,
                    lgl,
                    // 2417: lgl: typeof(_223) = *mut {l326} LGL
                    // 2417: lgl:   l326 = UNIQUE | NON_NULL, (empty)
                    b"sweeprtc\0" as *const u8 as *const libc::c_char,
                    // 2418: b"sweeprtc\0" a ... _char: typeof(_224) = *const {l328} i8
                    // 2418: b"sweeprtc\0" a ... _char:   l328 = UNIQUE | NON_NULL, (empty)
                    // 2418: b"sweeprtc\0" a ... st u8: typeof(_225) = *const {l330} u8
                    // 2418: b"sweeprtc\0" a ... st u8:   l330 = UNIQUE | NON_NULL, (empty)
                    // 2418: b"sweeprtc\0": typeof(_226) = *const {l332} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 2418: b"sweeprtc\0":   l332 = UNIQUE | NON_NULL, (empty)
                    // 2418: b"sweeprtc\0": typeof(_227) = & {l334} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 2418: b"sweeprtc\0":   l334 = UNIQUE | NON_NULL, FIXED
                    // 2418: b"sweeprtc\0": typeof(_226 = &raw const (*_227)) = *const {l620} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 2418: b"sweeprtc\0":   l620 = UNIQUE | NON_NULL, (empty)
                    // 2418: b"sweeprtc\0": typeof(_227 = const b"sweeprtc\x00") = & {l619} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 2418: b"sweeprtc\0":   l619 = UNIQUE | NON_NULL, (empty)
                    // 2418: b"sweeprtc\0" a ... st u8: typeof(_225 = move _226 as *const u8 (Pointer(ArrayToPointer))) = *const {l621} u8
                    // 2418: b"sweeprtc\0" a ... st u8:   l621 = UNIQUE | NON_NULL, (empty)
                    // 2418: b"sweeprtc\0" a ... _char: typeof(_224 = move _225 as *const i8 (Misc)) = *const {l622} i8
                    // 2418: b"sweeprtc\0" a ... _char:   l622 = UNIQUE | NON_NULL, (empty)
                    1 as libc::c_int,
                );
            }
            10 => {
                setopt(
                    i,
                    lgl,
                    // 2425: lgl: typeof(_231) = *mut {l339} LGL
                    // 2425: lgl:   l339 = UNIQUE | NON_NULL, (empty)
                    b"restartint\0" as *const u8 as *const libc::c_char,
                    // 2426: b"restartint\0" ... _char: typeof(_232) = *const {l341} i8
                    // 2426: b"restartint\0" ... _char:   l341 = UNIQUE | NON_NULL, (empty)
                    // 2426: b"restartint\0" ... st u8: typeof(_233) = *const {l343} u8
                    // 2426: b"restartint\0" ... st u8:   l343 = UNIQUE | NON_NULL, (empty)
                    // 2426: b"restartint\0": typeof(_234) = *const {l345} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2426: b"restartint\0":   l345 = UNIQUE | NON_NULL, (empty)
                    // 2426: b"restartint\0": typeof(_235) = & {l347} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2426: b"restartint\0":   l347 = UNIQUE | NON_NULL, FIXED
                    // 2426: b"restartint\0": typeof(_234 = &raw const (*_235)) = *const {l624} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2426: b"restartint\0":   l624 = UNIQUE | NON_NULL, (empty)
                    // 2426: b"restartint\0" ... _char: typeof(_232 = move _233 as *const i8 (Misc)) = *const {l626} i8
                    // 2426: b"restartint\0" ... _char:   l626 = UNIQUE | NON_NULL, (empty)
                    // 2426: b"restartint\0" ... st u8: typeof(_233 = move _234 as *const u8 (Pointer(ArrayToPointer))) = *const {l625} u8
                    // 2426: b"restartint\0" ... st u8:   l625 = UNIQUE | NON_NULL, (empty)
                    // 2426: b"restartint\0": typeof(_235 = const b"restartint\x00") = & {l623} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2426: b"restartint\0":   l623 = UNIQUE | NON_NULL, (empty)
                    100 as libc::c_int,
                );
            }
            11 => {
                setopt(
                    i,
                    lgl,
                    // 2433: lgl: typeof(_239) = *mut {l352} LGL
                    // 2433: lgl:   l352 = UNIQUE | NON_NULL, (empty)
                    b"reduceinit\0" as *const u8 as *const libc::c_char,
                    // 2434: b"reduceinit\0" ... _char: typeof(_240) = *const {l354} i8
                    // 2434: b"reduceinit\0" ... _char:   l354 = UNIQUE | NON_NULL, (empty)
                    // 2434: b"reduceinit\0" ... st u8: typeof(_241) = *const {l356} u8
                    // 2434: b"reduceinit\0" ... st u8:   l356 = UNIQUE | NON_NULL, (empty)
                    // 2434: b"reduceinit\0": typeof(_242) = *const {l358} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2434: b"reduceinit\0":   l358 = UNIQUE | NON_NULL, (empty)
                    // 2434: b"reduceinit\0": typeof(_243) = & {l360} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2434: b"reduceinit\0":   l360 = UNIQUE | NON_NULL, FIXED
                    // 2434: b"reduceinit\0" ... _char: typeof(_240 = move _241 as *const i8 (Misc)) = *const {l630} i8
                    // 2434: b"reduceinit\0" ... _char:   l630 = UNIQUE | NON_NULL, (empty)
                    // 2434: b"reduceinit\0" ... st u8: typeof(_241 = move _242 as *const u8 (Pointer(ArrayToPointer))) = *const {l629} u8
                    // 2434: b"reduceinit\0" ... st u8:   l629 = UNIQUE | NON_NULL, (empty)
                    // 2434: b"reduceinit\0": typeof(_242 = &raw const (*_243)) = *const {l628} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2434: b"reduceinit\0":   l628 = UNIQUE | NON_NULL, (empty)
                    // 2434: b"reduceinit\0": typeof(_243 = const b"reduceinit\x00") = & {l627} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2434: b"reduceinit\0":   l627 = UNIQUE | NON_NULL, (empty)
                    10000 as libc::c_int,
                );
                setopt(
                    i,
                    lgl,
                    // 2439: lgl: typeof(_247) = *mut {l365} LGL
                    // 2439: lgl:   l365 = UNIQUE | NON_NULL, (empty)
                    b"reducefixed\0" as *const u8 as *const libc::c_char,
                    // 2440: b"reducefixed\0 ... _char: typeof(_248) = *const {l367} i8
                    // 2440: b"reducefixed\0 ... _char:   l367 = UNIQUE | NON_NULL, (empty)
                    // 2440: b"reducefixed\0 ... st u8: typeof(_249) = *const {l369} u8
                    // 2440: b"reducefixed\0 ... st u8:   l369 = UNIQUE | NON_NULL, (empty)
                    // 2440: b"reducefixed\0": typeof(_250) = *const {l371} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                    // 2440: b"reducefixed\0":   l371 = UNIQUE | NON_NULL, (empty)
                    // 2440: b"reducefixed\0": typeof(_251) = & {l373} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                    // 2440: b"reducefixed\0":   l373 = UNIQUE | NON_NULL, FIXED
                    // 2440: b"reducefixed\0 ... st u8: typeof(_249 = move _250 as *const u8 (Pointer(ArrayToPointer))) = *const {l633} u8
                    // 2440: b"reducefixed\0 ... st u8:   l633 = UNIQUE | NON_NULL, (empty)
                    // 2440: b"reducefixed\0": typeof(_251 = const b"reducefixed\x00") = & {l631} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                    // 2440: b"reducefixed\0":   l631 = UNIQUE | NON_NULL, (empty)
                    // 2440: b"reducefixed\0": typeof(_250 = &raw const (*_251)) = *const {l632} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                    // 2440: b"reducefixed\0":   l632 = UNIQUE | NON_NULL, (empty)
                    // 2440: b"reducefixed\0 ... _char: typeof(_248 = move _249 as *const i8 (Misc)) = *const {l634} i8
                    // 2440: b"reducefixed\0 ... _char:   l634 = UNIQUE | NON_NULL, (empty)
                    1 as libc::c_int,
                );
            }
            12 => {
                setopt(
                    i,
                    lgl,
                    // 2447: lgl: typeof(_255) = *mut {l378} LGL
                    // 2447: lgl:   l378 = UNIQUE | NON_NULL, (empty)
                    b"restartint\0" as *const u8 as *const libc::c_char,
                    // 2448: b"restartint\0" ... _char: typeof(_256) = *const {l380} i8
                    // 2448: b"restartint\0" ... _char:   l380 = UNIQUE | NON_NULL, (empty)
                    // 2448: b"restartint\0" ... st u8: typeof(_257) = *const {l382} u8
                    // 2448: b"restartint\0" ... st u8:   l382 = UNIQUE | NON_NULL, (empty)
                    // 2448: b"restartint\0": typeof(_258) = *const {l384} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2448: b"restartint\0":   l384 = UNIQUE | NON_NULL, (empty)
                    // 2448: b"restartint\0": typeof(_259) = & {l386} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2448: b"restartint\0":   l386 = UNIQUE | NON_NULL, FIXED
                    // 2448: b"restartint\0": typeof(_259 = const b"restartint\x00") = & {l635} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2448: b"restartint\0":   l635 = UNIQUE | NON_NULL, (empty)
                    // 2448: b"restartint\0" ... _char: typeof(_256 = move _257 as *const i8 (Misc)) = *const {l638} i8
                    // 2448: b"restartint\0" ... _char:   l638 = UNIQUE | NON_NULL, (empty)
                    // 2448: b"restartint\0": typeof(_258 = &raw const (*_259)) = *const {l636} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 2448: b"restartint\0":   l636 = UNIQUE | NON_NULL, (empty)
                    // 2448: b"restartint\0" ... st u8: typeof(_257 = move _258 as *const u8 (Pointer(ArrayToPointer))) = *const {l637} u8
                    // 2448: b"restartint\0" ... st u8:   l637 = UNIQUE | NON_NULL, (empty)
                    4 as libc::c_int,
                );
            }
            0 | _ => {}
        }
    }
    lglseterm(
        lgl,
        // 2456: lgl: typeof(_262) = *mut {l390} LGL
        // 2456: lgl:   l390 = UNIQUE | NON_NULL, (empty)
        Some(term as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int),
        // 2457: Some(term as un ... _int): typeof(_263) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l392} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32>
        // 2457: Some(term as un ... _int):   l392 = UNIQUE | NON_NULL, (empty)
        // 2457: term as unsafe  ... c_int: typeof(_264) = fn(*mut {l394} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32
        // 2457: term as unsafe  ... c_int:   l394 = UNIQUE | NON_NULL, (empty)
        // 2457: Some(term as un ... _int): typeof(_263 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void) -> i32>::Some(move _264)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l640} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32>
        // 2457: Some(term as un ... _int):   l640 = UNIQUE | NON_NULL, (empty)
        // 2457: term: typeof(_264 = term as unsafe extern "C" fn(*mut libc::c_void) -> i32 (Pointer(ReifyFnPointer))) = fn(*mut {l639} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32
        // 2457: term:   l639 = UNIQUE | NON_NULL, (empty)
        w as *mut libc::c_void,
        // 2458: w as *mut libc: ... _void: typeof(_265) = *mut {l396} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2458: w as *mut libc: ... _void:   l396 = UNIQUE | NON_NULL, (empty)
        // 2458: w: typeof(_266) = *mut {l398} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 2458: w:   l398 = UNIQUE | NON_NULL, (empty)
        // 2458: w as *mut libc: ... _void: typeof(_265 = move _266 as *mut libc::c_void (Misc)) = *mut {l641} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2458: w as *mut libc: ... _void:   l641 = UNIQUE | NON_NULL, (empty)
    );
    lglsetmsglock(
        lgl,
        // 2461: lgl: typeof(_268) = *mut {l401} LGL
        // 2461: lgl:   l401 = UNIQUE | NON_NULL, (empty)
        Some(msglock as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        // 2462: Some(msglock as ... > ()): typeof(_269) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l403} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()>
        // 2462: Some(msglock as ... > ()):   l403 = UNIQUE | NON_NULL, (empty)
        // 2462: msglock as unsa ... -> (): typeof(_270) = fn(*mut {l405} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()
        // 2462: msglock as unsa ... -> ():   l405 = UNIQUE | NON_NULL, (empty)
        // 2462: msglock: typeof(_270 = msglock as unsafe extern "C" fn(*mut libc::c_void) (Pointer(ReifyFnPointer))) = fn(*mut {l642} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()
        // 2462: msglock:   l642 = UNIQUE | NON_NULL, (empty)
        // 2462: Some(msglock as ... > ()): typeof(_269 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void)>::Some(move _270)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l643} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()>
        // 2462: Some(msglock as ... > ()):   l643 = UNIQUE | NON_NULL, (empty)
        Some(msgunlock as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        // 2463: Some(msgunlock  ... > ()): typeof(_271) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l407} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()>
        // 2463: Some(msgunlock  ... > ()):   l407 = UNIQUE | NON_NULL, (empty)
        // 2463: msgunlock as un ... -> (): typeof(_272) = fn(*mut {l409} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()
        // 2463: msgunlock as un ... -> ():   l409 = UNIQUE | NON_NULL, (empty)
        // 2463: Some(msgunlock  ... > ()): typeof(_271 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void)>::Some(move _272)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l645} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()>
        // 2463: Some(msgunlock  ... > ()):   l645 = UNIQUE | NON_NULL, (empty)
        // 2463: msgunlock: typeof(_272 = msgunlock as unsafe extern "C" fn(*mut libc::c_void) (Pointer(ReifyFnPointer))) = fn(*mut {l644} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()
        // 2463: msgunlock:   l644 = UNIQUE | NON_NULL, (empty)
        w as *mut libc::c_void,
        // 2464: w as *mut libc: ... _void: typeof(_273) = *mut {l411} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2464: w as *mut libc: ... _void:   l411 = UNIQUE | NON_NULL, (empty)
        // 2464: w: typeof(_274) = *mut {l413} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 2464: w:   l413 = UNIQUE | NON_NULL, (empty)
        // 2464: w as *mut libc: ... _void: typeof(_273 = move _274 as *mut libc::c_void (Misc)) = *mut {l646} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2464: w as *mut libc: ... _void:   l646 = UNIQUE | NON_NULL, (empty)
    );
    if nounits == 0 {
    // 2466: nounits: typeof(_278) = *mut {l418} i32
    // 2466: nounits:   l418 = UNIQUE | NON_NULL, (empty)
        lglsetproduceunit(
            lgl,
            // 2468: lgl: typeof(_280) = *mut {l421} LGL
            // 2468: lgl:   l421 = UNIQUE | NON_NULL, (empty)
            Some(produceunit as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()),
            // 2469: Some(produceuni ... > ()): typeof(_281) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l423} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()>
            // 2469: Some(produceuni ... > ()):   l423 = UNIQUE | NON_NULL, (empty)
            // 2469: produceunit as  ... -> (): typeof(_282) = fn(*mut {l425} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()
            // 2469: produceunit as  ... -> ():   l425 = UNIQUE | NON_NULL, (empty)
            // 2469: Some(produceuni ... > ()): typeof(_281 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void, i32)>::Some(move _282)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l648} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()>
            // 2469: Some(produceuni ... > ()):   l648 = UNIQUE | NON_NULL, (empty)
            // 2469: produceunit: typeof(_282 = produceunit as unsafe extern "C" fn(*mut libc::c_void, i32) (Pointer(ReifyFnPointer))) = fn(*mut {l647} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()
            // 2469: produceunit:   l647 = UNIQUE | NON_NULL, (empty)
            w as *mut libc::c_void,
            // 2470: w as *mut libc: ... _void: typeof(_283) = *mut {l427} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2470: w as *mut libc: ... _void:   l427 = UNIQUE | NON_NULL, (empty)
            // 2470: w: typeof(_284) = *mut {l429} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 2470: w:   l429 = UNIQUE | NON_NULL, (empty)
            // 2470: w as *mut libc: ... _void: typeof(_283 = move _284 as *mut libc::c_void (Misc)) = *mut {l649} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2470: w as *mut libc: ... _void:   l649 = UNIQUE | NON_NULL, (empty)
        );
        lglsetconsumeunits(
            lgl,
            // 2473: lgl: typeof(_286) = *mut {l432} LGL
            // 2473: lgl:   l432 = UNIQUE | NON_NULL, (empty)
            Some(
            // 2474: Some( consumeun ... (), ): typeof(_287) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l434} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l435} *mut {l436} i32, *mut {l437} *mut {l438} i32) -> ()>
            // 2474: Some( consumeun ... (), ):   l434 = UNIQUE | NON_NULL, (empty)
            // 2474: Some( consumeun ... (), ):   l435 = UNIQUE | NON_NULL, (empty)
            // 2474: Some( consumeun ... (), ):   l436 = UNIQUE | NON_NULL, (empty)
            // 2474: Some( consumeun ... (), ):   l437 = UNIQUE | NON_NULL, (empty)
            // 2474: Some( consumeun ... (), ):   l438 = UNIQUE | NON_NULL, (empty)
            // 2474: Some( consumeun ... (), ): typeof(_287 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void, *mut *mut i32, *mut *mut i32)>::Some(move _288)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l655} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l656} *mut {l657} i32, *mut {l658} *mut {l659} i32) -> ()>
            // 2474: Some( consumeun ... (), ):   l655 = UNIQUE | NON_NULL, (empty)
            // 2474: Some( consumeun ... (), ):   l656 = UNIQUE | NON_NULL, (empty)
            // 2474: Some( consumeun ... (), ):   l657 = UNIQUE | NON_NULL, (empty)
            // 2474: Some( consumeun ... (), ):   l658 = UNIQUE | NON_NULL, (empty)
            // 2474: Some( consumeun ... (), ):   l659 = UNIQUE | NON_NULL, (empty)
                consumeunits
                // 2475: consumeunits as ... -> (): typeof(_288) = fn(*mut {l440} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l441} *mut {l442} i32, *mut {l443} *mut {l444} i32) -> ()
                // 2475: consumeunits as ... -> ():   l440 = UNIQUE | NON_NULL, (empty)
                // 2475: consumeunits as ... -> ():   l441 = UNIQUE | NON_NULL, (empty)
                // 2475: consumeunits as ... -> ():   l442 = UNIQUE | NON_NULL, (empty)
                // 2475: consumeunits as ... -> ():   l443 = UNIQUE | NON_NULL, (empty)
                // 2475: consumeunits as ... -> ():   l444 = UNIQUE | NON_NULL, (empty)
                // 2475: consumeunits: typeof(_288 = consumeunits as unsafe extern "C" fn(*mut libc::c_void, *mut *mut i32, *mut *mut i32) (Pointer(ReifyFnPointer))) = fn(*mut {l650} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l651} *mut {l652} i32, *mut {l653} *mut {l654} i32) -> ()
                // 2475: consumeunits:   l650 = UNIQUE | NON_NULL, (empty)
                // 2475: consumeunits:   l651 = UNIQUE | NON_NULL, (empty)
                // 2475: consumeunits:   l652 = UNIQUE | NON_NULL, (empty)
                // 2475: consumeunits:   l653 = UNIQUE | NON_NULL, (empty)
                // 2475: consumeunits:   l654 = UNIQUE | NON_NULL, (empty)
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut libc::c_int,
                        *mut *mut libc::c_int,
                    ) -> (),
            ),
            w as *mut libc::c_void,
            // 2482: w as *mut libc: ... _void: typeof(_289) = *mut {l446} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2482: w as *mut libc: ... _void:   l446 = UNIQUE | NON_NULL, (empty)
            // 2482: w: typeof(_290) = *mut {l448} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 2482: w:   l448 = UNIQUE | NON_NULL, (empty)
            // 2482: w as *mut libc: ... _void: typeof(_289 = move _290 as *mut libc::c_void (Misc)) = *mut {l660} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2482: w as *mut libc: ... _void:   l660 = UNIQUE | NON_NULL, (empty)
        );
        lglsetconsumedunits(
            lgl,
            // 2485: lgl: typeof(_292) = *mut {l451} LGL
            // 2485: lgl:   l451 = UNIQUE | NON_NULL, (empty)
            Some(consumedunits as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()),
            // 2486: Some(consumedun ... > ()): typeof(_293) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l453} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()>
            // 2486: Some(consumedun ... > ()):   l453 = UNIQUE | NON_NULL, (empty)
            // 2486: consumedunits a ... -> (): typeof(_294) = fn(*mut {l455} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()
            // 2486: consumedunits a ... -> ():   l455 = UNIQUE | NON_NULL, (empty)
            // 2486: Some(consumedun ... > ()): typeof(_293 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void, i32)>::Some(move _294)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l662} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()>
            // 2486: Some(consumedun ... > ()):   l662 = UNIQUE | NON_NULL, (empty)
            // 2486: consumedunits: typeof(_294 = consumedunits as unsafe extern "C" fn(*mut libc::c_void, i32) (Pointer(ReifyFnPointer))) = fn(*mut {l661} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()
            // 2486: consumedunits:   l661 = UNIQUE | NON_NULL, (empty)
            w as *mut libc::c_void,
            // 2487: w as *mut libc: ... _void: typeof(_295) = *mut {l457} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2487: w as *mut libc: ... _void:   l457 = UNIQUE | NON_NULL, (empty)
            // 2487: w: typeof(_296) = *mut {l459} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 2487: w:   l459 = UNIQUE | NON_NULL, (empty)
            // 2487: w as *mut libc: ... _void: typeof(_295 = move _296 as *mut libc::c_void (Misc)) = *mut {l663} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2487: w as *mut libc: ... _void:   l663 = UNIQUE | NON_NULL, (empty)
        );
    }
    if nocls == 0 {
    // 2490: nocls: typeof(_300) = *mut {l464} i32
    // 2490: nocls:   l464 = UNIQUE | NON_NULL, (empty)
        lglsetproducecls(
            lgl,
            // 2492: lgl: typeof(_302) = *mut {l467} LGL
            // 2492: lgl:   l467 = UNIQUE | NON_NULL, (empty)
            Some(
            // 2493: Some( producecl ... (), ): typeof(_303) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l469} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l470} i32, i32) -> ()>
            // 2493: Some( producecl ... (), ):   l469 = UNIQUE | NON_NULL, (empty)
            // 2493: Some( producecl ... (), ):   l470 = UNIQUE | NON_NULL, (empty)
            // 2493: Some( producecl ... (), ): typeof(_303 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void, *mut i32, i32)>::Some(move _304)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l666} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l667} i32, i32) -> ()>
            // 2493: Some( producecl ... (), ):   l666 = UNIQUE | NON_NULL, (empty)
            // 2493: Some( producecl ... (), ):   l667 = UNIQUE | NON_NULL, (empty)
                producecls
                // 2494: producecls as u ... -> (): typeof(_304) = fn(*mut {l472} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l473} i32, i32) -> ()
                // 2494: producecls as u ... -> ():   l472 = UNIQUE | NON_NULL, (empty)
                // 2494: producecls as u ... -> ():   l473 = UNIQUE | NON_NULL, (empty)
                // 2494: producecls: typeof(_304 = producecls as unsafe extern "C" fn(*mut libc::c_void, *mut i32, i32) (Pointer(ReifyFnPointer))) = fn(*mut {l664} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l665} i32, i32) -> ()
                // 2494: producecls:   l664 = UNIQUE | NON_NULL, (empty)
                // 2494: producecls:   l665 = UNIQUE | NON_NULL, (empty)
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_int, libc::c_int) -> (),
            ),
            w as *mut libc::c_void,
            // 2497: w as *mut libc: ... _void: typeof(_305) = *mut {l475} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2497: w as *mut libc: ... _void:   l475 = UNIQUE | NON_NULL, (empty)
            // 2497: w: typeof(_306) = *mut {l477} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 2497: w:   l477 = UNIQUE | NON_NULL, (empty)
            // 2497: w as *mut libc: ... _void: typeof(_305 = move _306 as *mut libc::c_void (Misc)) = *mut {l668} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2497: w as *mut libc: ... _void:   l668 = UNIQUE | NON_NULL, (empty)
        );
        lglsetconsumecls(
            lgl,
            // 2500: lgl: typeof(_308) = *mut {l480} LGL
            // 2500: lgl:   l480 = UNIQUE | NON_NULL, (empty)
            Some(
            // 2501: Some( consumecl ... (), ): typeof(_309) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l482} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l483} *mut {l484} i32, *mut {l485} i32) -> ()>
            // 2501: Some( consumecl ... (), ):   l482 = UNIQUE | NON_NULL, (empty)
            // 2501: Some( consumecl ... (), ):   l483 = UNIQUE | NON_NULL, (empty)
            // 2501: Some( consumecl ... (), ):   l484 = UNIQUE | NON_NULL, (empty)
            // 2501: Some( consumecl ... (), ):   l485 = UNIQUE | NON_NULL, (empty)
            // 2501: Some( consumecl ... (), ): typeof(_309 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void, *mut *mut i32, *mut i32)>::Some(move _310)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l673} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l674} *mut {l675} i32, *mut {l676} i32) -> ()>
            // 2501: Some( consumecl ... (), ):   l673 = UNIQUE | NON_NULL, (empty)
            // 2501: Some( consumecl ... (), ):   l674 = UNIQUE | NON_NULL, (empty)
            // 2501: Some( consumecl ... (), ):   l675 = UNIQUE | NON_NULL, (empty)
            // 2501: Some( consumecl ... (), ):   l676 = UNIQUE | NON_NULL, (empty)
                consumecls
                // 2502: consumecls as u ... -> (): typeof(_310) = fn(*mut {l487} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l488} *mut {l489} i32, *mut {l490} i32) -> ()
                // 2502: consumecls as u ... -> ():   l487 = UNIQUE | NON_NULL, (empty)
                // 2502: consumecls as u ... -> ():   l488 = UNIQUE | NON_NULL, (empty)
                // 2502: consumecls as u ... -> ():   l489 = UNIQUE | NON_NULL, (empty)
                // 2502: consumecls as u ... -> ():   l490 = UNIQUE | NON_NULL, (empty)
                // 2502: consumecls: typeof(_310 = consumecls as unsafe extern "C" fn(*mut libc::c_void, *mut *mut i32, *mut i32) (Pointer(ReifyFnPointer))) = fn(*mut {l669} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l670} *mut {l671} i32, *mut {l672} i32) -> ()
                // 2502: consumecls:   l669 = UNIQUE | NON_NULL, (empty)
                // 2502: consumecls:   l670 = UNIQUE | NON_NULL, (empty)
                // 2502: consumecls:   l671 = UNIQUE | NON_NULL, (empty)
                // 2502: consumecls:   l672 = UNIQUE | NON_NULL, (empty)
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut *mut libc::c_int,
                        *mut libc::c_int,
                    ) -> (),
            ),
            w as *mut libc::c_void,
            // 2509: w as *mut libc: ... _void: typeof(_311) = *mut {l492} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2509: w as *mut libc: ... _void:   l492 = UNIQUE | NON_NULL, (empty)
            // 2509: w: typeof(_312) = *mut {l494} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 2509: w:   l494 = UNIQUE | NON_NULL, (empty)
            // 2509: w as *mut libc: ... _void: typeof(_311 = move _312 as *mut libc::c_void (Misc)) = *mut {l677} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2509: w as *mut libc: ... _void:   l677 = UNIQUE | NON_NULL, (empty)
        );
        lglsetconsumedcls(
            lgl,
            // 2512: lgl: typeof(_314) = *mut {l497} LGL
            // 2512: lgl:   l497 = UNIQUE | NON_NULL, (empty)
            Some(consumedcls as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()),
            // 2513: Some(consumedcl ... > ()): typeof(_315) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l499} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()>
            // 2513: Some(consumedcl ... > ()):   l499 = UNIQUE | NON_NULL, (empty)
            // 2513: consumedcls as  ... -> (): typeof(_316) = fn(*mut {l501} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()
            // 2513: consumedcls as  ... -> ():   l501 = UNIQUE | NON_NULL, (empty)
            // 2513: Some(consumedcl ... > ()): typeof(_315 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void, i32)>::Some(move _316)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l679} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()>
            // 2513: Some(consumedcl ... > ()):   l679 = UNIQUE | NON_NULL, (empty)
            // 2513: consumedcls: typeof(_316 = consumedcls as unsafe extern "C" fn(*mut libc::c_void, i32) (Pointer(ReifyFnPointer))) = fn(*mut {l678} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()
            // 2513: consumedcls:   l678 = UNIQUE | NON_NULL, (empty)
            w as *mut libc::c_void,
            // 2514: w as *mut libc: ... _void: typeof(_317) = *mut {l503} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2514: w as *mut libc: ... _void:   l503 = UNIQUE | NON_NULL, (empty)
            // 2514: w: typeof(_318) = *mut {l505} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 2514: w:   l505 = UNIQUE | NON_NULL, (empty)
            // 2514: w as *mut libc: ... _void: typeof(_317 = move _318 as *mut libc::c_void (Misc)) = *mut {l680} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2514: w as *mut libc: ... _void:   l680 = UNIQUE | NON_NULL, (empty)
        );
    }
    if noeqs == 0 {
    // 2517: noeqs: typeof(_322) = *mut {l510} i32
    // 2517: noeqs:   l510 = UNIQUE | NON_NULL, (empty)
        lglsetlockeq(
            lgl,
            // 2519: lgl: typeof(_324) = *mut {l513} LGL
            // 2519: lgl:   l513 = UNIQUE | NON_NULL, (empty)
            Some(lockrepr as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_int),
            // 2520: Some(lockrepr a ... _int): typeof(_325) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l515} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> *mut {l516} i32>
            // 2520: Some(lockrepr a ... _int):   l515 = UNIQUE | NON_NULL, (empty)
            // 2520: Some(lockrepr a ... _int):   l516 = UNIQUE | NON_NULL, (empty)
            // 2520: lockrepr as uns ... c_int: typeof(_326) = fn(*mut {l518} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> *mut {l519} i32
            // 2520: lockrepr as uns ... c_int:   l518 = UNIQUE | NON_NULL, (empty)
            // 2520: lockrepr as uns ... c_int:   l519 = UNIQUE | NON_NULL, (empty)
            // 2520: lockrepr: typeof(_326 = lockrepr as unsafe extern "C" fn(*mut libc::c_void) -> *mut i32 (Pointer(ReifyFnPointer))) = fn(*mut {l681} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> *mut {l682} i32
            // 2520: lockrepr:   l681 = UNIQUE | NON_NULL, (empty)
            // 2520: lockrepr:   l682 = UNIQUE | NON_NULL, (empty)
            // 2520: Some(lockrepr a ... _int): typeof(_325 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut i32>::Some(move _326)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l683} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> *mut {l684} i32>
            // 2520: Some(lockrepr a ... _int):   l683 = UNIQUE | NON_NULL, (empty)
            // 2520: Some(lockrepr a ... _int):   l684 = UNIQUE | NON_NULL, (empty)
            w as *mut libc::c_void,
            // 2521: w as *mut libc: ... _void: typeof(_327) = *mut {l521} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2521: w as *mut libc: ... _void:   l521 = UNIQUE | NON_NULL, (empty)
            // 2521: w: typeof(_328) = *mut {l523} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 2521: w:   l523 = UNIQUE | NON_NULL, (empty)
            // 2521: w as *mut libc: ... _void: typeof(_327 = move _328 as *mut libc::c_void (Misc)) = *mut {l685} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2521: w as *mut libc: ... _void:   l685 = UNIQUE | NON_NULL, (empty)
        );
        lglsetunlockeq(
            lgl,
            // 2524: lgl: typeof(_330) = *mut {l526} LGL
            // 2524: lgl:   l526 = UNIQUE | NON_NULL, (empty)
            Some(
            // 2525: Some( unlockrep ... (), ): typeof(_331) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l528} DefId(2:5583 ~ core[480a]::ffi::c_void), i32, i32) -> ()>
            // 2525: Some( unlockrep ... (), ):   l528 = UNIQUE | NON_NULL, (empty)
            // 2525: Some( unlockrep ... (), ): typeof(_331 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void, i32, i32)>::Some(move _332)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l687} DefId(2:5583 ~ core[480a]::ffi::c_void), i32, i32) -> ()>
            // 2525: Some( unlockrep ... (), ):   l687 = UNIQUE | NON_NULL, (empty)
                unlockrepr
                // 2526: unlockrepr as u ... -> (): typeof(_332) = fn(*mut {l530} DefId(2:5583 ~ core[480a]::ffi::c_void), i32, i32) -> ()
                // 2526: unlockrepr as u ... -> ():   l530 = UNIQUE | NON_NULL, (empty)
                // 2526: unlockrepr: typeof(_332 = unlockrepr as unsafe extern "C" fn(*mut libc::c_void, i32, i32) (Pointer(ReifyFnPointer))) = fn(*mut {l686} DefId(2:5583 ~ core[480a]::ffi::c_void), i32, i32) -> ()
                // 2526: unlockrepr:   l686 = UNIQUE | NON_NULL, (empty)
                    as unsafe extern "C" fn(*mut libc::c_void, libc::c_int, libc::c_int) -> (),
            ),
            w as *mut libc::c_void,
            // 2529: w as *mut libc: ... _void: typeof(_333) = *mut {l532} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2529: w as *mut libc: ... _void:   l532 = UNIQUE | NON_NULL, (empty)
            // 2529: w: typeof(_334) = *mut {l534} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 2529: w:   l534 = UNIQUE | NON_NULL, (empty)
            // 2529: w as *mut libc: ... _void: typeof(_333 = move _334 as *mut libc::c_void (Misc)) = *mut {l688} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2529: w as *mut libc: ... _void:   l688 = UNIQUE | NON_NULL, (empty)
        );
    }
    msg(
        i,
        2 as libc::c_int,
        b"initialized\0" as *const u8 as *const libc::c_char,
        // 2535: b"initialized\0 ... _char: typeof(_338) = *const {l539} i8
        // 2535: b"initialized\0 ... _char:   l539 = UNIQUE | NON_NULL, (empty)
        // 2535: b"initialized\0 ... st u8: typeof(_339) = *const {l541} u8
        // 2535: b"initialized\0 ... st u8:   l541 = UNIQUE | NON_NULL, (empty)
        // 2535: b"initialized\0": typeof(_340) = *const {l543} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 2535: b"initialized\0":   l543 = UNIQUE | NON_NULL, (empty)
        // 2535: b"initialized\0": typeof(_341) = & {l545} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 2535: b"initialized\0":   l545 = UNIQUE | NON_NULL, FIXED
        // 2535: b"initialized\0 ... _char: typeof(_338 = move _339 as *const i8 (Misc)) = *const {l692} i8
        // 2535: b"initialized\0 ... _char:   l692 = UNIQUE | NON_NULL, (empty)
        // 2535: b"initialized\0 ... st u8: typeof(_339 = move _340 as *const u8 (Pointer(ArrayToPointer))) = *const {l691} u8
        // 2535: b"initialized\0 ... st u8:   l691 = UNIQUE | NON_NULL, (empty)
        // 2535: b"initialized\0": typeof(_341 = const b"initialized\x00") = & {l689} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 2535: b"initialized\0":   l689 = UNIQUE | NON_NULL, (empty)
        // 2535: b"initialized\0": typeof(_340 = &raw const (*_341)) = *const {l690} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 2535: b"initialized\0":   l690 = UNIQUE | NON_NULL, (empty)
    );
}
unsafe extern "C" fn bytes2mbll(mut bytes: int64_t) -> libc::c_longlong {
    return bytes as libc::c_longlong + ((1 as libc::c_longlong) << 20 as libc::c_int)
        - 1 as libc::c_int as libc::c_longlong
        >> 20 as libc::c_int;
}
unsafe extern "C" fn bytes2gbll(mut bytes: int64_t) -> libc::c_longlong {
    return bytes as libc::c_longlong + ((1 as libc::c_longlong) << 30 as libc::c_int)
        - 1 as libc::c_int as libc::c_longlong
        >> 30 as libc::c_int;
}
unsafe extern "C" fn cloneworker(mut i: libc::c_int) -> libc::c_int {
    msg(
        -(1 as libc::c_int),
        0 as libc::c_int,
        b"trying to clone worker %d from worker 0\0" as *const u8 as *const libc::c_char,
        // 2552: b"trying to clo ... _char: typeof(_8) = *const {l8} i8
        // 2552: b"trying to clo ... _char:   l8 = UNIQUE | NON_NULL, (empty)
        // 2552: b"trying to clo ... st u8: typeof(_9) = *const {l10} u8
        // 2552: b"trying to clo ... st u8:   l10 = UNIQUE | NON_NULL, (empty)
        // 2552: b"trying to clo ...  0\0": typeof(_10) = *const {l12} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 2552: b"trying to clo ...  0\0":   l12 = UNIQUE | NON_NULL, (empty)
        // 2552: b"trying to clo ...  0\0": typeof(_11) = & {l14} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 2552: b"trying to clo ...  0\0":   l14 = UNIQUE | NON_NULL, FIXED
        // 2552: b"trying to clo ... _char: typeof(_8 = move _9 as *const i8 (Misc)) = *const {l168} i8
        // 2552: b"trying to clo ... _char:   l168 = UNIQUE | NON_NULL, (empty)
        // 2552: b"trying to clo ...  0\0": typeof(_10 = &raw const (*_11)) = *const {l166} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 2552: b"trying to clo ...  0\0":   l166 = UNIQUE | NON_NULL, (empty)
        // 2552: b"trying to clo ... st u8: typeof(_9 = move _10 as *const u8 (Pointer(ArrayToPointer))) = *const {l167} u8
        // 2552: b"trying to clo ... st u8:   l167 = UNIQUE | NON_NULL, (empty)
        // 2552: b"trying to clo ...  0\0": typeof(_11 = const b"trying to clone worker %d from worker 0\x00") = & {l165} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 2552: b"trying to clo ...  0\0":   l165 = UNIQUE | NON_NULL, (empty)
        i,
    );
    msg(
        -(1 as libc::c_int),
        0 as libc::c_int,
        b"prediction: %lld MB to be cloned + allocated %lld MB = %lld MB\0" as *const u8
        // 2558: b"prediction: % ... _char: typeof(_18) = *const {l22} i8
        // 2558: b"prediction: % ... _char:   l22 = UNIQUE | NON_NULL, (empty)
        // 2558: b"prediction: % ... st u8: typeof(_19) = *const {l24} u8
        // 2558: b"prediction: % ... st u8:   l24 = UNIQUE | NON_NULL, (empty)
        // 2558: b"prediction: % ... MB\0": typeof(_20) = *const {l26} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003f)) }]
        // 2558: b"prediction: % ... MB\0":   l26 = UNIQUE | NON_NULL, (empty)
        // 2558: b"prediction: % ... MB\0": typeof(_21) = & {l28} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003f)) }]
        // 2558: b"prediction: % ... MB\0":   l28 = UNIQUE | NON_NULL, FIXED
        // 2558: b"prediction: % ... MB\0": typeof(_20 = &raw const (*_21)) = *const {l170} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003f)) }]
        // 2558: b"prediction: % ... MB\0":   l170 = UNIQUE | NON_NULL, (empty)
        // 2558: b"prediction: % ... MB\0": typeof(_21 = const b"prediction: %lld MB to be cloned + allocated %lld MB = %lld MB\x00") = & {l169} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003f)) }]
        // 2558: b"prediction: % ... MB\0":   l169 = UNIQUE | NON_NULL, (empty)
        // 2558: b"prediction: % ... _char: typeof(_18 = move _19 as *const i8 (Misc)) = *const {l172} i8
        // 2558: b"prediction: % ... _char:   l172 = UNIQUE | NON_NULL, (empty)
        // 2558: b"prediction: % ... st u8: typeof(_19 = move _20 as *const u8 (Pointer(ArrayToPointer))) = *const {l171} u8
        // 2558: b"prediction: % ... st u8:   l171 = UNIQUE | NON_NULL, (empty)
            as *const libc::c_char,
        bytes2mbll(lglbytes((*workers.offset(0 as libc::c_int as isize)).lgl) as int64_t),
        // 2560: (*workers.offse ... ).lgl: typeof(_25) = *mut {l33} LGL
        // 2560: (*workers.offse ... ).lgl:   l33 = UNIQUE | NON_NULL, (empty)
        // 2560: workers.offset( ... size): typeof(_26) = *mut {l35} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 2560: workers.offset( ... size):   l35 = UNIQUE | NON_NULL, (empty)
        // 2560: workers: typeof(_27) = *mut {l37} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 2560: workers:   l37 = UNIQUE | NON_NULL, (empty)
        // 2560: workers: typeof(_28) = *mut {l39} *mut {l40} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 2560: workers:   l39 = UNIQUE | NON_NULL, (empty)
        // 2560: workers:   l40 = UNIQUE | NON_NULL, (empty)
        bytes2mbll(mem.current as int64_t),
        // 2561: mem: typeof(_34) = *mut {l47} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
        // 2561: mem:   l47 = UNIQUE | NON_NULL, (empty)
        bytes2mbll(
            (lglbytes((*workers.offset(0 as libc::c_int as isize)).lgl)).wrapping_add(mem.current)
            // 2563: (*workers.offse ... ).lgl: typeof(_39) = *mut {l53} LGL
            // 2563: (*workers.offse ... ).lgl:   l53 = UNIQUE | NON_NULL, (empty)
            // 2563: workers.offset( ... size): typeof(_40) = *mut {l55} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 2563: workers.offset( ... size):   l55 = UNIQUE | NON_NULL, (empty)
            // 2563: workers: typeof(_41) = *mut {l57} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 2563: workers:   l57 = UNIQUE | NON_NULL, (empty)
            // 2563: workers: typeof(_42) = *mut {l59} *mut {l60} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 2563: workers:   l59 = UNIQUE | NON_NULL, (empty)
            // 2563: workers:   l60 = UNIQUE | NON_NULL, (empty)
            // 2563: mem: typeof(_46) = *mut {l65} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
            // 2563: mem:   l65 = UNIQUE | NON_NULL, (empty)
                as int64_t,
        ),
    );
    if (lglbytes((*workers.offset(0 as libc::c_int as isize)).lgl)).wrapping_add(mem.current)
    // 2567: (*workers.offse ... ).lgl: typeof(_51) = *mut {l71} LGL
    // 2567: (*workers.offse ... ).lgl:   l71 = UNIQUE | NON_NULL, (empty)
    // 2567: workers.offset( ... size): typeof(_52) = *mut {l73} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2567: workers.offset( ... size):   l73 = UNIQUE | NON_NULL, (empty)
    // 2567: workers: typeof(_53) = *mut {l75} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2567: workers:   l75 = UNIQUE | NON_NULL, (empty)
    // 2567: workers: typeof(_54) = *mut {l77} *mut {l78} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2567: workers:   l77 = UNIQUE | NON_NULL, (empty)
    // 2567: workers:   l78 = UNIQUE | NON_NULL, (empty)
    // 2567: mem: typeof(_58) = *mut {l83} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
    // 2567: mem:   l83 = UNIQUE | NON_NULL, (empty)
        >= softmemlimit as size_t
        // 2568: softmemlimit: typeof(_61) = *mut {l87} i64
        // 2568: softmemlimit:   l87 = UNIQUE | NON_NULL, (empty)
    {
        msg(
            -(1 as libc::c_int),
            0 as libc::c_int,
            b"will not clone worker %d since soft memory limit %lld MB would be hit\0" as *const u8
            // 2573: b"will not clon ... _char: typeof(_68) = *const {l95} i8
            // 2573: b"will not clon ... _char:   l95 = UNIQUE | NON_NULL, (empty)
            // 2573: b"will not clon ... st u8: typeof(_69) = *const {l97} u8
            // 2573: b"will not clon ... st u8:   l97 = UNIQUE | NON_NULL, (empty)
            // 2573: b"will not clon ... it\0": typeof(_70) = *const {l99} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000046)) }]
            // 2573: b"will not clon ... it\0":   l99 = UNIQUE | NON_NULL, (empty)
            // 2573: b"will not clon ... it\0": typeof(_71) = & {l101} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000046)) }]
            // 2573: b"will not clon ... it\0":   l101 = UNIQUE | NON_NULL, FIXED
            // 2573: b"will not clon ... st u8: typeof(_69 = move _70 as *const u8 (Pointer(ArrayToPointer))) = *const {l175} u8
            // 2573: b"will not clon ... st u8:   l175 = UNIQUE | NON_NULL, (empty)
            // 2573: b"will not clon ... _char: typeof(_68 = move _69 as *const i8 (Misc)) = *const {l176} i8
            // 2573: b"will not clon ... _char:   l176 = UNIQUE | NON_NULL, (empty)
            // 2573: b"will not clon ... it\0": typeof(_71 = const b"will not clone worker %d since soft memory limit %lld MB would be hit\x00") = & {l173} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000046)) }]
            // 2573: b"will not clon ... it\0":   l173 = UNIQUE | NON_NULL, (empty)
            // 2573: b"will not clon ... it\0": typeof(_70 = &raw const (*_71)) = *const {l174} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000046)) }]
            // 2573: b"will not clon ... it\0":   l174 = UNIQUE | NON_NULL, (empty)
                as *const libc::c_char,
            i,
            bytes2mbll(softmemlimit),
            // 2576: softmemlimit: typeof(_75) = *mut {l106} i64
            // 2576: softmemlimit:   l106 = UNIQUE | NON_NULL, (empty)
        );
        return 0 as libc::c_int;
    }
    let ref mut fresh17 = (*workers.offset(i as isize)).lgl;
    // 2580: ref mut fresh17: typeof(_76) = &mut {l108} *mut {l109} LGL
    // 2580: ref mut fresh17:   l108 = UNIQUE | NON_NULL, FIXED
    // 2580: ref mut fresh17:   l109 = UNIQUE | NON_NULL, (empty)
    // 2580: workers.offset( ... size): typeof(_77) = *mut {l111} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2580: workers.offset( ... size):   l111 = UNIQUE | NON_NULL, (empty)
    // 2580: workers: typeof(_78) = *mut {l113} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2580: workers:   l113 = UNIQUE | NON_NULL, (empty)
    // 2580: workers: typeof(_79) = *mut {l115} *mut {l116} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2580: workers:   l115 = UNIQUE | NON_NULL, (empty)
    // 2580: workers:   l116 = UNIQUE | NON_NULL, (empty)
    // 2580: ref mut fresh17: typeof(_76 = &mut ((*_77).0: *mut LGL)) = &mut {l177} *mut {l178} LGL
    // 2580: ref mut fresh17:   l177 = UNIQUE | NON_NULL, (empty)
    // 2580: ref mut fresh17:   l178 = UNIQUE | NON_NULL, (empty)
    *fresh17 = lglclone((*workers.offset(0 as libc::c_int as isize)).lgl);
    // 2581: lglclone((*work ... .lgl): typeof(_82) = *mut {l120} LGL
    // 2581: lglclone((*work ... .lgl):   l120 = UNIQUE | NON_NULL, (empty)
    // 2581: (*workers.offse ... ).lgl: typeof(_83) = *mut {l122} LGL
    // 2581: (*workers.offse ... ).lgl:   l122 = UNIQUE | NON_NULL, (empty)
    // 2581: workers.offset( ... size): typeof(_84) = *mut {l124} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2581: workers.offset( ... size):   l124 = UNIQUE | NON_NULL, (empty)
    // 2581: workers: typeof(_85) = *mut {l126} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2581: workers:   l126 = UNIQUE | NON_NULL, (empty)
    // 2581: workers: typeof(_86) = *mut {l128} *mut {l129} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2581: workers:   l128 = UNIQUE | NON_NULL, (empty)
    // 2581: workers:   l129 = UNIQUE | NON_NULL, (empty)
    setopts((*workers.offset(i as isize)).lgl, i);
    // 2582: (*workers.offse ... ).lgl: typeof(_90) = *mut {l134} LGL
    // 2582: (*workers.offse ... ).lgl:   l134 = UNIQUE | NON_NULL, (empty)
    // 2582: workers.offset( ... size): typeof(_91) = *mut {l136} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2582: workers.offset( ... size):   l136 = UNIQUE | NON_NULL, (empty)
    // 2582: workers: typeof(_92) = *mut {l138} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2582: workers:   l138 = UNIQUE | NON_NULL, (empty)
    // 2582: workers: typeof(_93) = *mut {l140} *mut {l141} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2582: workers:   l140 = UNIQUE | NON_NULL, (empty)
    // 2582: workers:   l141 = UNIQUE | NON_NULL, (empty)
    msg(
        -(1 as libc::c_int),
        0 as libc::c_int,
        b"%lld MB total allocated memory after cloning worker %d from worker 0\0" as *const u8
        // 2586: b"%lld MB total ... _char: typeof(_102) = *const {l151} i8
        // 2586: b"%lld MB total ... _char:   l151 = UNIQUE | NON_NULL, (empty)
        // 2586: b"%lld MB total ... st u8: typeof(_103) = *const {l153} u8
        // 2586: b"%lld MB total ... st u8:   l153 = UNIQUE | NON_NULL, (empty)
        // 2586: b"%lld MB total ...  0\0": typeof(_104) = *const {l155} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000045)) }]
        // 2586: b"%lld MB total ...  0\0":   l155 = UNIQUE | NON_NULL, (empty)
        // 2586: b"%lld MB total ...  0\0": typeof(_105) = & {l157} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000045)) }]
        // 2586: b"%lld MB total ...  0\0":   l157 = UNIQUE | NON_NULL, FIXED
        // 2586: b"%lld MB total ...  0\0": typeof(_105 = const b"%lld MB total allocated memory after cloning worker %d from worker 0\x00") = & {l179} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000045)) }]
        // 2586: b"%lld MB total ...  0\0":   l179 = UNIQUE | NON_NULL, (empty)
        // 2586: b"%lld MB total ...  0\0": typeof(_104 = &raw const (*_105)) = *const {l180} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000045)) }]
        // 2586: b"%lld MB total ...  0\0":   l180 = UNIQUE | NON_NULL, (empty)
        // 2586: b"%lld MB total ... st u8: typeof(_103 = move _104 as *const u8 (Pointer(ArrayToPointer))) = *const {l181} u8
        // 2586: b"%lld MB total ... st u8:   l181 = UNIQUE | NON_NULL, (empty)
        // 2586: b"%lld MB total ... _char: typeof(_102 = move _103 as *const i8 (Misc)) = *const {l182} i8
        // 2586: b"%lld MB total ... _char:   l182 = UNIQUE | NON_NULL, (empty)
            as *const libc::c_char,
        bytes2mbll(mem.current as int64_t),
        // 2588: mem: typeof(_109) = *mut {l162} DefId(0:584 ~ plingeling[18f5]::C2RustUnnamed_6)
        // 2588: mem:   l162 = UNIQUE | NON_NULL, (empty)
        i,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn version() {
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, lglversion());
    // 2594: b"%s\n\0" as *c ... _char: typeof(_3) = *const {l3} i8
    // 2594: b"%s\n\0" as *c ... _char:   l3 = UNIQUE | NON_NULL, (empty)
    // 2594: b"%s\n\0" as *c ... st u8: typeof(_4) = *const {l5} u8
    // 2594: b"%s\n\0" as *c ... st u8:   l5 = UNIQUE | NON_NULL, (empty)
    // 2594: b"%s\n\0": typeof(_5) = *const {l7} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 2594: b"%s\n\0":   l7 = UNIQUE | NON_NULL, (empty)
    // 2594: b"%s\n\0": typeof(_6) = & {l9} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 2594: b"%s\n\0":   l9 = UNIQUE | NON_NULL, FIXED
    // 2594: lglversion(): typeof(_7) = *const {l11} i8
    // 2594: lglversion():   l11 = UNIQUE | NON_NULL, (empty)
    // 2594: b"%s\n\0" as *c ... st u8: typeof(_4 = move _5 as *const u8 (Pointer(ArrayToPointer))) = *const {l17} u8
    // 2594: b"%s\n\0" as *c ... st u8:   l17 = UNIQUE | NON_NULL, (empty)
    // 2594: b"%s\n\0": typeof(_6 = const b"%s\n\x00") = & {l15} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 2594: b"%s\n\0":   l15 = UNIQUE | NON_NULL, (empty)
    // 2594: b"%s\n\0" as *c ... _char: typeof(_3 = move _4 as *const i8 (Misc)) = *const {l18} i8
    // 2594: b"%s\n\0" as *c ... _char:   l18 = UNIQUE | NON_NULL, (empty)
    // 2594: b"%s\n\0": typeof(_5 = &raw const (*_6)) = *const {l16} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 2594: b"%s\n\0":   l16 = UNIQUE | NON_NULL, (empty)
    exit(0 as libc::c_int);
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
// 2597: mut argv: typeof(_2) = *mut {g46} *mut {g47} i8
// 2597: mut argv:   g46 = UNIQUE | NON_NULL, FIXED
// 2597: mut argv:   g47 = UNIQUE | NON_NULL, FIXED
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
    let mut w: *mut Worker = 0 as *mut Worker;
    // 2608: mut w: typeof(_14) = *mut {l14} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2608: mut w:   l14 = UNIQUE | NON_NULL, (empty)
    // 2608: 0 as *mut Worker: typeof(_14 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l3711} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2608: 0 as *mut Worker:   l3711 = UNIQUE | NON_NULL, (empty)
    let mut winner: *mut Worker = 0 as *mut Worker;
    // 2609: mut winner: typeof(_15) = *mut {l16} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2609: mut winner:   l16 = UNIQUE | NON_NULL, (empty)
    // 2609: 0 as *mut Worker: typeof(_15 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l3712} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2609: 0 as *mut Worker:   l3712 = UNIQUE | NON_NULL, (empty)
    let mut maxconsumer: *mut Worker = 0 as *mut Worker;
    // 2610: mut maxconsumer: typeof(_16) = *mut {l18} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2610: mut maxconsumer:   l18 = UNIQUE | NON_NULL, (empty)
    // 2610: 0 as *mut Worker: typeof(_16 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l3713} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2610: 0 as *mut Worker:   l3713 = UNIQUE | NON_NULL, (empty)
    let mut maxproducer: *mut Worker = 0 as *mut Worker;
    // 2611: mut maxproducer: typeof(_17) = *mut {l20} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2611: mut maxproducer:   l20 = UNIQUE | NON_NULL, (empty)
    // 2611: 0 as *mut Worker: typeof(_17 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l3714} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2611: 0 as *mut Worker:   l3714 = UNIQUE | NON_NULL, (empty)
    let mut sorted: *mut *mut Worker = 0 as *mut *mut Worker;
    // 2612: mut sorted: typeof(_18) = *mut {l22} *mut {l23} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2612: mut sorted:   l22 = UNIQUE | NON_NULL, (empty)
    // 2612: mut sorted:   l23 = UNIQUE | NON_NULL, (empty)
    // 2612: 0 as *mut *mut  ... orker: typeof(_18 = const 0_usize as *mut *mut Worker (PointerFromExposedAddress)) = *mut {l3715} *mut {l3716} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2612: 0 as *mut *mut  ... orker:   l3715 = UNIQUE | NON_NULL, (empty)
    // 2612: 0 as *mut *mut  ... orker:   l3716 = UNIQUE | NON_NULL, (empty)
    let mut earlyworker: *mut Worker = 0 as *mut Worker;
    // 2613: mut earlyworker: typeof(_19) = *mut {l25} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2613: mut earlyworker:   l25 = UNIQUE | NON_NULL, (empty)
    // 2613: 0 as *mut Worker: typeof(_19 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l3717} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2613: 0 as *mut Worker:   l3717 = UNIQUE | NON_NULL, (empty)
    let mut sumconsumed: libc::c_int = 0;
    let mut sumconsumedunits: libc::c_int = 0;
    let mut sumconsumedcls: libc::c_int = 0;
    let mut sumconsumedeqs: libc::c_int = 0;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    // 2618: mut errstr: typeof(_24) = *const {l31} i8
    // 2618: mut errstr:   l31 = UNIQUE | NON_NULL, (empty)
    // 2618: 0 as *const lib ... _char: typeof(_24 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l3718} i8
    // 2618: 0 as *const lib ... _char:   l3718 = UNIQUE | NON_NULL, (empty)
    let mut arg: *const libc::c_char = 0 as *const libc::c_char;
    // 2619: mut arg: typeof(_25) = *const {l33} i8
    // 2619: mut arg:   l33 = UNIQUE | NON_NULL, (empty)
    // 2619: 0 as *const lib ... _char: typeof(_25 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l3719} i8
    // 2619: 0 as *const lib ... _char:   l3719 = UNIQUE | NON_NULL, (empty)
    let mut bytes: size_t = 0;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    // 2621: mut cmd: typeof(_27) = *mut {l36} i8
    // 2621: mut cmd:   l36 = UNIQUE | NON_NULL, (empty)
    // 2621: 0 as *mut libc: ... _char: typeof(_27 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l3720} i8
    // 2621: 0 as *mut libc: ... _char:   l3720 = UNIQUE | NON_NULL, (empty)
    start = currentime();
    // 2622: start: typeof(_29) = *mut {l39} f64
    // 2622: start:   l39 = UNIQUE | NON_NULL, (empty)
    clin = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < argc {
        let mut totalmem: size_t = getsystemtotalmem(0 as libc::c_int) as size_t;
        if strcmp(
            *argv.offset(i as isize),
            // 2628: *argv.offset(i  ... size): typeof(_43) = *const {l54} i8
            // 2628: *argv.offset(i  ... size):   l54 = UNIQUE | NON_NULL, (empty)
            // 2628: *argv.offset(i  ... size): typeof(_44) = *mut {l56} i8
            // 2628: *argv.offset(i  ... size):   l56 = UNIQUE | NON_NULL, (empty)
            // 2628: argv.offset(i a ... size): typeof(_45) = *mut {l58} *mut {l59} i8
            // 2628: argv.offset(i a ... size):   l58 = UNIQUE | NON_NULL, (empty)
            // 2628: argv.offset(i a ... size):   l59 = UNIQUE | NON_NULL, (empty)
            // 2628: argv: typeof(_46) = *mut {l61} *mut {l62} i8
            // 2628: argv:   l61 = UNIQUE | NON_NULL, (empty)
            // 2628: argv:   l62 = UNIQUE | NON_NULL, (empty)
            // 2628: *argv.offset(i  ... size): typeof(_43 = move _44 as *const i8 (Pointer(MutToConstPointer))) = *const {l3721} i8
            // 2628: *argv.offset(i  ... size):   l3721 = UNIQUE | NON_NULL, (empty)
            b"-h\0" as *const u8 as *const libc::c_char,
            // 2629: b"-h\0" as *con ... _char: typeof(_49) = *const {l66} i8
            // 2629: b"-h\0" as *con ... _char:   l66 = UNIQUE | NON_NULL, (empty)
            // 2629: b"-h\0" as *const u8: typeof(_50) = *const {l68} u8
            // 2629: b"-h\0" as *const u8:   l68 = UNIQUE | NON_NULL, (empty)
            // 2629: b"-h\0": typeof(_51) = *const {l70} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2629: b"-h\0":   l70 = UNIQUE | NON_NULL, (empty)
            // 2629: b"-h\0": typeof(_52) = & {l72} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2629: b"-h\0":   l72 = UNIQUE | NON_NULL, FIXED
            // 2629: b"-h\0" as *const u8: typeof(_50 = move _51 as *const u8 (Pointer(ArrayToPointer))) = *const {l3724} u8
            // 2629: b"-h\0" as *const u8:   l3724 = UNIQUE | NON_NULL, (empty)
            // 2629: b"-h\0" as *con ... _char: typeof(_49 = move _50 as *const i8 (Misc)) = *const {l3725} i8
            // 2629: b"-h\0" as *con ... _char:   l3725 = UNIQUE | NON_NULL, (empty)
            // 2629: b"-h\0": typeof(_52 = const b"-h\x00") = & {l3722} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2629: b"-h\0":   l3722 = UNIQUE | NON_NULL, (empty)
            // 2629: b"-h\0": typeof(_51 = &raw const (*_52)) = *const {l3723} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2629: b"-h\0":   l3723 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            printf(
                b"usage: plingeling [ <option> ... ] [ <dimacs>[.gz|.bz2|.7z|.zip|.lzma] [<threads>] ]\n\nwhere <option> is one of the following:\n\n  -h         print this command line option summary\n  --version  print version and exit\n  -v         increase verbose level\n  -n         do not print solution / witness\n\n  -t <num>   number of worker threads (default %d on this machine)\n  -m <num>   maximal memory in MB (default %lld MB on this machine)\n  -g <num>   maximal memory in GB (default %lld GB on this machine)\n  -l <num>   limit on number of workers before collecting shared clause\n\n  -p         plain portfolio, no sharing, e.g. implies the following:\n\n                --do-not-share-units\n                --do-not-share-clauses\n                --do-not-share-equivalences\n\0"
                // 2633: b"usage: plinge ... _char: typeof(_55) = *const {l76} i8
                // 2633: b"usage: plinge ... _char:   l76 = UNIQUE | NON_NULL, (empty)
                // 2633: b"usage: plinge ... st u8: typeof(_56) = *const {l78} u8
                // 2633: b"usage: plinge ... st u8:   l78 = UNIQUE | NON_NULL, (empty)
                // 2633: b"usage: plinge ... \n\0": typeof(_57) = *const {l80} [u8; Const { ty: usize, kind: Value(Leaf(0x00000000000002fd)) }]
                // 2633: b"usage: plinge ... \n\0":   l80 = UNIQUE | NON_NULL, (empty)
                // 2633: b"usage: plinge ... \n\0": typeof(_58) = & {l82} [u8; Const { ty: usize, kind: Value(Leaf(0x00000000000002fd)) }]
                // 2633: b"usage: plinge ... \n\0":   l82 = UNIQUE | NON_NULL, FIXED
                // 2633: b"usage: plinge ... st u8: typeof(_56 = move _57 as *const u8 (Pointer(ArrayToPointer))) = *const {l3728} u8
                // 2633: b"usage: plinge ... st u8:   l3728 = UNIQUE | NON_NULL, (empty)
                // 2633: b"usage: plinge ... \n\0": typeof(_57 = &raw const (*_58)) = *const {l3727} [u8; Const { ty: usize, kind: Value(Leaf(0x00000000000002fd)) }]
                // 2633: b"usage: plinge ... \n\0":   l3727 = UNIQUE | NON_NULL, (empty)
                // 2633: b"usage: plinge ... \n\0": typeof(_58 = const b"usage: plingeling [ <option> ... ] [ <dimacs>[.gz|.bz2|.7z|.zip|.lzma] [<threads>] ]\n\nwhere <option> is one of the following:\n\n  -h         print this command line option summary\n  --version  print version and exit\n  -v         increase verbose level\n  -n         do not print solution / witness\n\n  -t <num>   number of worker threads (default %d on this machine)\n  -m <num>   maximal memory in MB (default %lld MB on this machine)\n  -g <num>   maximal memory in GB (default %lld GB on this machine)\n  -l <num>   limit on number of workers before collecting shared clause\n\n  -p         plain portfolio, no sharing, e.g. implies the following:\n\n                --do-not-share-units\n                --do-not-share-clauses\n                --do-not-share-equivalences\n\x00") = & {l3726} [u8; Const { ty: usize, kind: Value(Leaf(0x00000000000002fd)) }]
                // 2633: b"usage: plinge ... \n\0":   l3726 = UNIQUE | NON_NULL, (empty)
                // 2633: b"usage: plinge ... _char: typeof(_55 = move _56 as *const i8 (Misc)) = *const {l3729} i8
                // 2633: b"usage: plinge ... _char:   l3729 = UNIQUE | NON_NULL, (empty)
                    as *const u8 as *const libc::c_char,
                getsystemcores(0 as libc::c_int),
                bytes2mbll(totalmem as int64_t),
                bytes2gbll(totalmem as int64_t),
            );
            exit(0 as libc::c_int);
        } else if strcmp(
            *argv.offset(i as isize),
            // 2641: *argv.offset(i  ... size): typeof(_71) = *const {l96} i8
            // 2641: *argv.offset(i  ... size):   l96 = UNIQUE | NON_NULL, (empty)
            // 2641: *argv.offset(i  ... size): typeof(_72) = *mut {l98} i8
            // 2641: *argv.offset(i  ... size):   l98 = UNIQUE | NON_NULL, (empty)
            // 2641: argv.offset(i a ... size): typeof(_73) = *mut {l100} *mut {l101} i8
            // 2641: argv.offset(i a ... size):   l100 = UNIQUE | NON_NULL, (empty)
            // 2641: argv.offset(i a ... size):   l101 = UNIQUE | NON_NULL, (empty)
            // 2641: argv: typeof(_74) = *mut {l103} *mut {l104} i8
            // 2641: argv:   l103 = UNIQUE | NON_NULL, (empty)
            // 2641: argv:   l104 = UNIQUE | NON_NULL, (empty)
            // 2641: *argv.offset(i  ... size): typeof(_71 = move _72 as *const i8 (Pointer(MutToConstPointer))) = *const {l3730} i8
            // 2641: *argv.offset(i  ... size):   l3730 = UNIQUE | NON_NULL, (empty)
            b"--version\0" as *const u8 as *const libc::c_char,
            // 2642: b"--version\0"  ... _char: typeof(_77) = *const {l108} i8
            // 2642: b"--version\0"  ... _char:   l108 = UNIQUE | NON_NULL, (empty)
            // 2642: b"--version\0"  ... st u8: typeof(_78) = *const {l110} u8
            // 2642: b"--version\0"  ... st u8:   l110 = UNIQUE | NON_NULL, (empty)
            // 2642: b"--version\0": typeof(_79) = *const {l112} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 2642: b"--version\0":   l112 = UNIQUE | NON_NULL, (empty)
            // 2642: b"--version\0": typeof(_80) = & {l114} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 2642: b"--version\0":   l114 = UNIQUE | NON_NULL, FIXED
            // 2642: b"--version\0"  ... st u8: typeof(_78 = move _79 as *const u8 (Pointer(ArrayToPointer))) = *const {l3733} u8
            // 2642: b"--version\0"  ... st u8:   l3733 = UNIQUE | NON_NULL, (empty)
            // 2642: b"--version\0": typeof(_79 = &raw const (*_80)) = *const {l3732} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 2642: b"--version\0":   l3732 = UNIQUE | NON_NULL, (empty)
            // 2642: b"--version\0"  ... _char: typeof(_77 = move _78 as *const i8 (Misc)) = *const {l3734} i8
            // 2642: b"--version\0"  ... _char:   l3734 = UNIQUE | NON_NULL, (empty)
            // 2642: b"--version\0": typeof(_80 = const b"--version\x00") = & {l3731} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 2642: b"--version\0":   l3731 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            version();
        } else if strcmp(
            *argv.offset(i as isize),
            // 2647: *argv.offset(i  ... size): typeof(_84) = *const {l119} i8
            // 2647: *argv.offset(i  ... size):   l119 = UNIQUE | NON_NULL, (empty)
            // 2647: *argv.offset(i  ... size): typeof(_85) = *mut {l121} i8
            // 2647: *argv.offset(i  ... size):   l121 = UNIQUE | NON_NULL, (empty)
            // 2647: argv.offset(i a ... size): typeof(_86) = *mut {l123} *mut {l124} i8
            // 2647: argv.offset(i a ... size):   l123 = UNIQUE | NON_NULL, (empty)
            // 2647: argv.offset(i a ... size):   l124 = UNIQUE | NON_NULL, (empty)
            // 2647: argv: typeof(_87) = *mut {l126} *mut {l127} i8
            // 2647: argv:   l126 = UNIQUE | NON_NULL, (empty)
            // 2647: argv:   l127 = UNIQUE | NON_NULL, (empty)
            // 2647: *argv.offset(i  ... size): typeof(_84 = move _85 as *const i8 (Pointer(MutToConstPointer))) = *const {l3735} i8
            // 2647: *argv.offset(i  ... size):   l3735 = UNIQUE | NON_NULL, (empty)
            b"-v\0" as *const u8 as *const libc::c_char,
            // 2648: b"-v\0" as *con ... _char: typeof(_90) = *const {l131} i8
            // 2648: b"-v\0" as *con ... _char:   l131 = UNIQUE | NON_NULL, (empty)
            // 2648: b"-v\0" as *const u8: typeof(_91) = *const {l133} u8
            // 2648: b"-v\0" as *const u8:   l133 = UNIQUE | NON_NULL, (empty)
            // 2648: b"-v\0": typeof(_92) = *const {l135} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2648: b"-v\0":   l135 = UNIQUE | NON_NULL, (empty)
            // 2648: b"-v\0": typeof(_93) = & {l137} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2648: b"-v\0":   l137 = UNIQUE | NON_NULL, FIXED
            // 2648: b"-v\0" as *con ... _char: typeof(_90 = move _91 as *const i8 (Misc)) = *const {l3739} i8
            // 2648: b"-v\0" as *con ... _char:   l3739 = UNIQUE | NON_NULL, (empty)
            // 2648: b"-v\0": typeof(_92 = &raw const (*_93)) = *const {l3737} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2648: b"-v\0":   l3737 = UNIQUE | NON_NULL, (empty)
            // 2648: b"-v\0": typeof(_93 = const b"-v\x00") = & {l3736} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2648: b"-v\0":   l3736 = UNIQUE | NON_NULL, (empty)
            // 2648: b"-v\0" as *const u8: typeof(_91 = move _92 as *const u8 (Pointer(ArrayToPointer))) = *const {l3738} u8
            // 2648: b"-v\0" as *const u8:   l3738 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            verbose += 1;
            // 2651: verbose: typeof(_94) = *mut {l139} i32
            // 2651: verbose:   l139 = UNIQUE | NON_NULL, (empty)
            verbose;
            // 2652: verbose: typeof(_97) = *mut {l143} i32
            // 2652: verbose:   l143 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2654: *argv.offset(i  ... size): typeof(_100) = *const {l147} i8
            // 2654: *argv.offset(i  ... size):   l147 = UNIQUE | NON_NULL, (empty)
            // 2654: *argv.offset(i  ... size): typeof(_101) = *mut {l149} i8
            // 2654: *argv.offset(i  ... size):   l149 = UNIQUE | NON_NULL, (empty)
            // 2654: argv.offset(i a ... size): typeof(_102) = *mut {l151} *mut {l152} i8
            // 2654: argv.offset(i a ... size):   l151 = UNIQUE | NON_NULL, (empty)
            // 2654: argv.offset(i a ... size):   l152 = UNIQUE | NON_NULL, (empty)
            // 2654: argv: typeof(_103) = *mut {l154} *mut {l155} i8
            // 2654: argv:   l154 = UNIQUE | NON_NULL, (empty)
            // 2654: argv:   l155 = UNIQUE | NON_NULL, (empty)
            // 2654: *argv.offset(i  ... size): typeof(_100 = move _101 as *const i8 (Pointer(MutToConstPointer))) = *const {l3740} i8
            // 2654: *argv.offset(i  ... size):   l3740 = UNIQUE | NON_NULL, (empty)
            b"-p\0" as *const u8 as *const libc::c_char,
            // 2655: b"-p\0" as *con ... _char: typeof(_106) = *const {l159} i8
            // 2655: b"-p\0" as *con ... _char:   l159 = UNIQUE | NON_NULL, (empty)
            // 2655: b"-p\0" as *const u8: typeof(_107) = *const {l161} u8
            // 2655: b"-p\0" as *const u8:   l161 = UNIQUE | NON_NULL, (empty)
            // 2655: b"-p\0": typeof(_108) = *const {l163} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2655: b"-p\0":   l163 = UNIQUE | NON_NULL, (empty)
            // 2655: b"-p\0": typeof(_109) = & {l165} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2655: b"-p\0":   l165 = UNIQUE | NON_NULL, FIXED
            // 2655: b"-p\0" as *const u8: typeof(_107 = move _108 as *const u8 (Pointer(ArrayToPointer))) = *const {l3743} u8
            // 2655: b"-p\0" as *const u8:   l3743 = UNIQUE | NON_NULL, (empty)
            // 2655: b"-p\0": typeof(_109 = const b"-p\x00") = & {l3741} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2655: b"-p\0":   l3741 = UNIQUE | NON_NULL, (empty)
            // 2655: b"-p\0": typeof(_108 = &raw const (*_109)) = *const {l3742} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2655: b"-p\0":   l3742 = UNIQUE | NON_NULL, (empty)
            // 2655: b"-p\0" as *con ... _char: typeof(_106 = move _107 as *const i8 (Misc)) = *const {l3744} i8
            // 2655: b"-p\0" as *con ... _char:   l3744 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            plain = 1 as libc::c_int;
            // 2658: plain: typeof(_111) = *mut {l168} i32
            // 2658: plain:   l168 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2660: *argv.offset(i  ... size): typeof(_114) = *const {l172} i8
            // 2660: *argv.offset(i  ... size):   l172 = UNIQUE | NON_NULL, (empty)
            // 2660: *argv.offset(i  ... size): typeof(_115) = *mut {l174} i8
            // 2660: *argv.offset(i  ... size):   l174 = UNIQUE | NON_NULL, (empty)
            // 2660: argv.offset(i a ... size): typeof(_116) = *mut {l176} *mut {l177} i8
            // 2660: argv.offset(i a ... size):   l176 = UNIQUE | NON_NULL, (empty)
            // 2660: argv.offset(i a ... size):   l177 = UNIQUE | NON_NULL, (empty)
            // 2660: argv: typeof(_117) = *mut {l179} *mut {l180} i8
            // 2660: argv:   l179 = UNIQUE | NON_NULL, (empty)
            // 2660: argv:   l180 = UNIQUE | NON_NULL, (empty)
            // 2660: *argv.offset(i  ... size): typeof(_114 = move _115 as *const i8 (Pointer(MutToConstPointer))) = *const {l3745} i8
            // 2660: *argv.offset(i  ... size):   l3745 = UNIQUE | NON_NULL, (empty)
            b"-n\0" as *const u8 as *const libc::c_char,
            // 2661: b"-n\0" as *con ... _char: typeof(_120) = *const {l184} i8
            // 2661: b"-n\0" as *con ... _char:   l184 = UNIQUE | NON_NULL, (empty)
            // 2661: b"-n\0" as *const u8: typeof(_121) = *const {l186} u8
            // 2661: b"-n\0" as *const u8:   l186 = UNIQUE | NON_NULL, (empty)
            // 2661: b"-n\0": typeof(_122) = *const {l188} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2661: b"-n\0":   l188 = UNIQUE | NON_NULL, (empty)
            // 2661: b"-n\0": typeof(_123) = & {l190} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2661: b"-n\0":   l190 = UNIQUE | NON_NULL, FIXED
            // 2661: b"-n\0" as *con ... _char: typeof(_120 = move _121 as *const i8 (Misc)) = *const {l3749} i8
            // 2661: b"-n\0" as *con ... _char:   l3749 = UNIQUE | NON_NULL, (empty)
            // 2661: b"-n\0": typeof(_122 = &raw const (*_123)) = *const {l3747} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2661: b"-n\0":   l3747 = UNIQUE | NON_NULL, (empty)
            // 2661: b"-n\0" as *const u8: typeof(_121 = move _122 as *const u8 (Pointer(ArrayToPointer))) = *const {l3748} u8
            // 2661: b"-n\0" as *const u8:   l3748 = UNIQUE | NON_NULL, (empty)
            // 2661: b"-n\0": typeof(_123 = const b"-n\x00") = & {l3746} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2661: b"-n\0":   l3746 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            witness = 0 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            // 2666: *argv.offset(i  ... size): typeof(_127) = *const {l195} i8
            // 2666: *argv.offset(i  ... size):   l195 = UNIQUE | NON_NULL, (empty)
            // 2666: *argv.offset(i  ... size): typeof(_128) = *mut {l197} i8
            // 2666: *argv.offset(i  ... size):   l197 = UNIQUE | NON_NULL, (empty)
            // 2666: argv.offset(i a ... size): typeof(_129) = *mut {l199} *mut {l200} i8
            // 2666: argv.offset(i a ... size):   l199 = UNIQUE | NON_NULL, (empty)
            // 2666: argv.offset(i a ... size):   l200 = UNIQUE | NON_NULL, (empty)
            // 2666: argv: typeof(_130) = *mut {l202} *mut {l203} i8
            // 2666: argv:   l202 = UNIQUE | NON_NULL, (empty)
            // 2666: argv:   l203 = UNIQUE | NON_NULL, (empty)
            // 2666: *argv.offset(i  ... size): typeof(_127 = move _128 as *const i8 (Pointer(MutToConstPointer))) = *const {l3750} i8
            // 2666: *argv.offset(i  ... size):   l3750 = UNIQUE | NON_NULL, (empty)
            b"--do-not-share-units\0" as *const u8 as *const libc::c_char,
            // 2667: b"--do-not-shar ... _char: typeof(_133) = *const {l207} i8
            // 2667: b"--do-not-shar ... _char:   l207 = UNIQUE | NON_NULL, (empty)
            // 2667: b"--do-not-shar ... st u8: typeof(_134) = *const {l209} u8
            // 2667: b"--do-not-shar ... st u8:   l209 = UNIQUE | NON_NULL, (empty)
            // 2667: b"--do-not-shar ... ts\0": typeof(_135) = *const {l211} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
            // 2667: b"--do-not-shar ... ts\0":   l211 = UNIQUE | NON_NULL, (empty)
            // 2667: b"--do-not-shar ... ts\0": typeof(_136) = & {l213} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
            // 2667: b"--do-not-shar ... ts\0":   l213 = UNIQUE | NON_NULL, FIXED
            // 2667: b"--do-not-shar ... ts\0": typeof(_135 = &raw const (*_136)) = *const {l3752} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
            // 2667: b"--do-not-shar ... ts\0":   l3752 = UNIQUE | NON_NULL, (empty)
            // 2667: b"--do-not-shar ... _char: typeof(_133 = move _134 as *const i8 (Misc)) = *const {l3754} i8
            // 2667: b"--do-not-shar ... _char:   l3754 = UNIQUE | NON_NULL, (empty)
            // 2667: b"--do-not-shar ... ts\0": typeof(_136 = const b"--do-not-share-units\x00") = & {l3751} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
            // 2667: b"--do-not-shar ... ts\0":   l3751 = UNIQUE | NON_NULL, (empty)
            // 2667: b"--do-not-shar ... st u8: typeof(_134 = move _135 as *const u8 (Pointer(ArrayToPointer))) = *const {l3753} u8
            // 2667: b"--do-not-shar ... st u8:   l3753 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            nounits = 1 as libc::c_int;
            // 2670: nounits: typeof(_138) = *mut {l216} i32
            // 2670: nounits:   l216 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2672: *argv.offset(i  ... size): typeof(_141) = *const {l220} i8
            // 2672: *argv.offset(i  ... size):   l220 = UNIQUE | NON_NULL, (empty)
            // 2672: *argv.offset(i  ... size): typeof(_142) = *mut {l222} i8
            // 2672: *argv.offset(i  ... size):   l222 = UNIQUE | NON_NULL, (empty)
            // 2672: argv.offset(i a ... size): typeof(_143) = *mut {l224} *mut {l225} i8
            // 2672: argv.offset(i a ... size):   l224 = UNIQUE | NON_NULL, (empty)
            // 2672: argv.offset(i a ... size):   l225 = UNIQUE | NON_NULL, (empty)
            // 2672: argv: typeof(_144) = *mut {l227} *mut {l228} i8
            // 2672: argv:   l227 = UNIQUE | NON_NULL, (empty)
            // 2672: argv:   l228 = UNIQUE | NON_NULL, (empty)
            // 2672: *argv.offset(i  ... size): typeof(_141 = move _142 as *const i8 (Pointer(MutToConstPointer))) = *const {l3755} i8
            // 2672: *argv.offset(i  ... size):   l3755 = UNIQUE | NON_NULL, (empty)
            b"--do-not-share-clauses\0" as *const u8 as *const libc::c_char,
            // 2673: b"--do-not-shar ... _char: typeof(_147) = *const {l232} i8
            // 2673: b"--do-not-shar ... _char:   l232 = UNIQUE | NON_NULL, (empty)
            // 2673: b"--do-not-shar ... st u8: typeof(_148) = *const {l234} u8
            // 2673: b"--do-not-shar ... st u8:   l234 = UNIQUE | NON_NULL, (empty)
            // 2673: b"--do-not-shar ... es\0": typeof(_149) = *const {l236} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
            // 2673: b"--do-not-shar ... es\0":   l236 = UNIQUE | NON_NULL, (empty)
            // 2673: b"--do-not-shar ... es\0": typeof(_150) = & {l238} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
            // 2673: b"--do-not-shar ... es\0":   l238 = UNIQUE | NON_NULL, FIXED
            // 2673: b"--do-not-shar ... es\0": typeof(_149 = &raw const (*_150)) = *const {l3757} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
            // 2673: b"--do-not-shar ... es\0":   l3757 = UNIQUE | NON_NULL, (empty)
            // 2673: b"--do-not-shar ... _char: typeof(_147 = move _148 as *const i8 (Misc)) = *const {l3759} i8
            // 2673: b"--do-not-shar ... _char:   l3759 = UNIQUE | NON_NULL, (empty)
            // 2673: b"--do-not-shar ... st u8: typeof(_148 = move _149 as *const u8 (Pointer(ArrayToPointer))) = *const {l3758} u8
            // 2673: b"--do-not-shar ... st u8:   l3758 = UNIQUE | NON_NULL, (empty)
            // 2673: b"--do-not-shar ... es\0": typeof(_150 = const b"--do-not-share-clauses\x00") = & {l3756} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
            // 2673: b"--do-not-shar ... es\0":   l3756 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            nocls = 1 as libc::c_int;
            // 2676: nocls: typeof(_152) = *mut {l241} i32
            // 2676: nocls:   l241 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2678: *argv.offset(i  ... size): typeof(_155) = *const {l245} i8
            // 2678: *argv.offset(i  ... size):   l245 = UNIQUE | NON_NULL, (empty)
            // 2678: *argv.offset(i  ... size): typeof(_156) = *mut {l247} i8
            // 2678: *argv.offset(i  ... size):   l247 = UNIQUE | NON_NULL, (empty)
            // 2678: argv.offset(i a ... size): typeof(_157) = *mut {l249} *mut {l250} i8
            // 2678: argv.offset(i a ... size):   l249 = UNIQUE | NON_NULL, (empty)
            // 2678: argv.offset(i a ... size):   l250 = UNIQUE | NON_NULL, (empty)
            // 2678: argv: typeof(_158) = *mut {l252} *mut {l253} i8
            // 2678: argv:   l252 = UNIQUE | NON_NULL, (empty)
            // 2678: argv:   l253 = UNIQUE | NON_NULL, (empty)
            // 2678: *argv.offset(i  ... size): typeof(_155 = move _156 as *const i8 (Pointer(MutToConstPointer))) = *const {l3760} i8
            // 2678: *argv.offset(i  ... size):   l3760 = UNIQUE | NON_NULL, (empty)
            b"--do-not-share-equivalences\0" as *const u8 as *const libc::c_char,
            // 2679: b"--do-not-shar ... _char: typeof(_161) = *const {l257} i8
            // 2679: b"--do-not-shar ... _char:   l257 = UNIQUE | NON_NULL, (empty)
            // 2679: b"--do-not-shar ... st u8: typeof(_162) = *const {l259} u8
            // 2679: b"--do-not-shar ... st u8:   l259 = UNIQUE | NON_NULL, (empty)
            // 2679: b"--do-not-shar ... es\0": typeof(_163) = *const {l261} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
            // 2679: b"--do-not-shar ... es\0":   l261 = UNIQUE | NON_NULL, (empty)
            // 2679: b"--do-not-shar ... es\0": typeof(_164) = & {l263} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
            // 2679: b"--do-not-shar ... es\0":   l263 = UNIQUE | NON_NULL, FIXED
            // 2679: b"--do-not-shar ... es\0": typeof(_164 = const b"--do-not-share-equivalences\x00") = & {l3761} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
            // 2679: b"--do-not-shar ... es\0":   l3761 = UNIQUE | NON_NULL, (empty)
            // 2679: b"--do-not-shar ... _char: typeof(_161 = move _162 as *const i8 (Misc)) = *const {l3764} i8
            // 2679: b"--do-not-shar ... _char:   l3764 = UNIQUE | NON_NULL, (empty)
            // 2679: b"--do-not-shar ... es\0": typeof(_163 = &raw const (*_164)) = *const {l3762} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
            // 2679: b"--do-not-shar ... es\0":   l3762 = UNIQUE | NON_NULL, (empty)
            // 2679: b"--do-not-shar ... st u8: typeof(_162 = move _163 as *const u8 (Pointer(ArrayToPointer))) = *const {l3763} u8
            // 2679: b"--do-not-shar ... st u8:   l3763 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            noeqs = 1 as libc::c_int;
            // 2682: noeqs: typeof(_166) = *mut {l266} i32
            // 2682: noeqs:   l266 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2684: *argv.offset(i  ... size): typeof(_169) = *const {l270} i8
            // 2684: *argv.offset(i  ... size):   l270 = UNIQUE | NON_NULL, (empty)
            // 2684: *argv.offset(i  ... size): typeof(_170) = *mut {l272} i8
            // 2684: *argv.offset(i  ... size):   l272 = UNIQUE | NON_NULL, (empty)
            // 2684: argv.offset(i a ... size): typeof(_171) = *mut {l274} *mut {l275} i8
            // 2684: argv.offset(i a ... size):   l274 = UNIQUE | NON_NULL, (empty)
            // 2684: argv.offset(i a ... size):   l275 = UNIQUE | NON_NULL, (empty)
            // 2684: argv: typeof(_172) = *mut {l277} *mut {l278} i8
            // 2684: argv:   l277 = UNIQUE | NON_NULL, (empty)
            // 2684: argv:   l278 = UNIQUE | NON_NULL, (empty)
            // 2684: *argv.offset(i  ... size): typeof(_169 = move _170 as *const i8 (Pointer(MutToConstPointer))) = *const {l3765} i8
            // 2684: *argv.offset(i  ... size):   l3765 = UNIQUE | NON_NULL, (empty)
            b"-t\0" as *const u8 as *const libc::c_char,
            // 2685: b"-t\0" as *con ... _char: typeof(_175) = *const {l282} i8
            // 2685: b"-t\0" as *con ... _char:   l282 = UNIQUE | NON_NULL, (empty)
            // 2685: b"-t\0" as *const u8: typeof(_176) = *const {l284} u8
            // 2685: b"-t\0" as *const u8:   l284 = UNIQUE | NON_NULL, (empty)
            // 2685: b"-t\0": typeof(_177) = *const {l286} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2685: b"-t\0":   l286 = UNIQUE | NON_NULL, (empty)
            // 2685: b"-t\0": typeof(_178) = & {l288} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2685: b"-t\0":   l288 = UNIQUE | NON_NULL, FIXED
            // 2685: b"-t\0": typeof(_178 = const b"-t\x00") = & {l3766} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2685: b"-t\0":   l3766 = UNIQUE | NON_NULL, (empty)
            // 2685: b"-t\0" as *con ... _char: typeof(_175 = move _176 as *const i8 (Misc)) = *const {l3769} i8
            // 2685: b"-t\0" as *con ... _char:   l3769 = UNIQUE | NON_NULL, (empty)
            // 2685: b"-t\0" as *const u8: typeof(_176 = move _177 as *const u8 (Pointer(ArrayToPointer))) = *const {l3768} u8
            // 2685: b"-t\0" as *const u8:   l3768 = UNIQUE | NON_NULL, (empty)
            // 2685: b"-t\0": typeof(_177 = &raw const (*_178)) = *const {l3767} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2685: b"-t\0":   l3767 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            if nworkers != 0 {
            // 2688: nworkers: typeof(_182) = *mut {l293} i32
            // 2688: nworkers:   l293 = UNIQUE | NON_NULL, (empty)
                die(b"multiple '-t' options\0" as *const u8 as *const libc::c_char);
                // 2689: b"multiple '-t' ... _char: typeof(_184) = *const {l296} i8
                // 2689: b"multiple '-t' ... _char:   l296 = UNIQUE | NON_NULL, (empty)
                // 2689: b"multiple '-t' ... st u8: typeof(_185) = *const {l298} u8
                // 2689: b"multiple '-t' ... st u8:   l298 = UNIQUE | NON_NULL, (empty)
                // 2689: b"multiple '-t' ... ns\0": typeof(_186) = *const {l300} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 2689: b"multiple '-t' ... ns\0":   l300 = UNIQUE | NON_NULL, (empty)
                // 2689: b"multiple '-t' ... ns\0": typeof(_187) = & {l302} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 2689: b"multiple '-t' ... ns\0":   l302 = UNIQUE | NON_NULL, FIXED
                // 2689: b"multiple '-t' ... ns\0": typeof(_187 = const b"multiple \'-t\' options\x00") = & {l3770} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 2689: b"multiple '-t' ... ns\0":   l3770 = UNIQUE | NON_NULL, (empty)
                // 2689: b"multiple '-t' ... ns\0": typeof(_186 = &raw const (*_187)) = *const {l3771} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 2689: b"multiple '-t' ... ns\0":   l3771 = UNIQUE | NON_NULL, (empty)
                // 2689: b"multiple '-t' ... _char: typeof(_184 = move _185 as *const i8 (Misc)) = *const {l3773} i8
                // 2689: b"multiple '-t' ... _char:   l3773 = UNIQUE | NON_NULL, (empty)
                // 2689: b"multiple '-t' ... st u8: typeof(_185 = move _186 as *const u8 (Pointer(ArrayToPointer))) = *const {l3772} u8
                // 2689: b"multiple '-t' ... st u8:   l3772 = UNIQUE | NON_NULL, (empty)
            }
            if nworkers2 != 0 {
            // 2691: nworkers2: typeof(_191) = *mut {l307} i32
            // 2691: nworkers2:   l307 = UNIQUE | NON_NULL, (empty)
                die(b"number of threads and '-t' option given\0" as *const u8
                // 2692: b"number of thr ... _char: typeof(_193) = *const {l310} i8
                // 2692: b"number of thr ... _char:   l310 = UNIQUE | NON_NULL, (empty)
                // 2692: b"number of thr ... st u8: typeof(_194) = *const {l312} u8
                // 2692: b"number of thr ... st u8:   l312 = UNIQUE | NON_NULL, (empty)
                // 2692: b"number of thr ... en\0": typeof(_195) = *const {l314} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 2692: b"number of thr ... en\0":   l314 = UNIQUE | NON_NULL, (empty)
                // 2692: b"number of thr ... en\0": typeof(_196) = & {l316} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 2692: b"number of thr ... en\0":   l316 = UNIQUE | NON_NULL, FIXED
                // 2692: b"number of thr ... st u8: typeof(_194 = move _195 as *const u8 (Pointer(ArrayToPointer))) = *const {l3776} u8
                // 2692: b"number of thr ... st u8:   l3776 = UNIQUE | NON_NULL, (empty)
                // 2692: b"number of thr ... en\0": typeof(_196 = const b"number of threads and \'-t\' option given\x00") = & {l3774} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 2692: b"number of thr ... en\0":   l3774 = UNIQUE | NON_NULL, (empty)
                // 2692: b"number of thr ... _char: typeof(_193 = move _194 as *const i8 (Misc)) = *const {l3777} i8
                // 2692: b"number of thr ... _char:   l3777 = UNIQUE | NON_NULL, (empty)
                // 2692: b"number of thr ... en\0": typeof(_195 = &raw const (*_196)) = *const {l3775} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 2692: b"number of thr ... en\0":   l3775 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char);
            }
            if i + 1 as libc::c_int == argc {
                die(b"argument to '-t' missing\0" as *const u8 as *const libc::c_char);
                // 2696: b"argument to ' ... _char: typeof(_205) = *const {l326} i8
                // 2696: b"argument to ' ... _char:   l326 = UNIQUE | NON_NULL, (empty)
                // 2696: b"argument to ' ... st u8: typeof(_206) = *const {l328} u8
                // 2696: b"argument to ' ... st u8:   l328 = UNIQUE | NON_NULL, (empty)
                // 2696: b"argument to ' ... ng\0": typeof(_207) = *const {l330} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2696: b"argument to ' ... ng\0":   l330 = UNIQUE | NON_NULL, (empty)
                // 2696: b"argument to ' ... ng\0": typeof(_208) = & {l332} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2696: b"argument to ' ... ng\0":   l332 = UNIQUE | NON_NULL, FIXED
                // 2696: b"argument to ' ... st u8: typeof(_206 = move _207 as *const u8 (Pointer(ArrayToPointer))) = *const {l3780} u8
                // 2696: b"argument to ' ... st u8:   l3780 = UNIQUE | NON_NULL, (empty)
                // 2696: b"argument to ' ... _char: typeof(_205 = move _206 as *const i8 (Misc)) = *const {l3781} i8
                // 2696: b"argument to ' ... _char:   l3781 = UNIQUE | NON_NULL, (empty)
                // 2696: b"argument to ' ... ng\0": typeof(_208 = const b"argument to \'-t\' missing\x00") = & {l3778} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2696: b"argument to ' ... ng\0":   l3778 = UNIQUE | NON_NULL, (empty)
                // 2696: b"argument to ' ... ng\0": typeof(_207 = &raw const (*_208)) = *const {l3779} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2696: b"argument to ' ... ng\0":   l3779 = UNIQUE | NON_NULL, (empty)
            }
            i += 1;
            arg = *argv.offset(i as isize);
            // 2699: *argv.offset(i  ... size): typeof(_210) = *mut {l335} i8
            // 2699: *argv.offset(i  ... size):   l335 = UNIQUE | NON_NULL, (empty)
            // 2699: argv.offset(i a ... size): typeof(_211) = *mut {l337} *mut {l338} i8
            // 2699: argv.offset(i a ... size):   l337 = UNIQUE | NON_NULL, (empty)
            // 2699: argv.offset(i a ... size):   l338 = UNIQUE | NON_NULL, (empty)
            // 2699: argv: typeof(_212) = *mut {l340} *mut {l341} i8
            // 2699: argv:   l340 = UNIQUE | NON_NULL, (empty)
            // 2699: argv:   l341 = UNIQUE | NON_NULL, (empty)
            // 2699: arg = *argv.off ... size): typeof(_25 = move _210 as *const i8 (Pointer(MutToConstPointer))) = *const {l3782} i8
            // 2699: arg = *argv.off ... size):   l3782 = UNIQUE | NON_NULL, (empty)
            if isposnum(arg) == 0 || {
            // 2700: arg: typeof(_218) = *const {l348} i8
            // 2700: arg:   l348 = UNIQUE | NON_NULL, (empty)
                nworkers = atoi(arg);
                // 2701: arg: typeof(_221) = *const {l352} i8
                // 2701: arg:   l352 = UNIQUE | NON_NULL, (empty)
                // 2701: nworkers: typeof(_222) = *mut {l354} i32
                // 2701: nworkers:   l354 = UNIQUE | NON_NULL, (empty)
                nworkers <= 0 as libc::c_int
                // 2702: nworkers: typeof(_224) = *mut {l357} i32
                // 2702: nworkers:   l357 = UNIQUE | NON_NULL, (empty)
            } {
                die(
                    b"invalid argument '%s' to '-t'\0" as *const u8 as *const libc::c_char,
                    // 2705: b"invalid argum ... _char: typeof(_227) = *const {l361} i8
                    // 2705: b"invalid argum ... _char:   l361 = UNIQUE | NON_NULL, (empty)
                    // 2705: b"invalid argum ... st u8: typeof(_228) = *const {l363} u8
                    // 2705: b"invalid argum ... st u8:   l363 = UNIQUE | NON_NULL, (empty)
                    // 2705: b"invalid argum ... t'\0": typeof(_229) = *const {l365} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                    // 2705: b"invalid argum ... t'\0":   l365 = UNIQUE | NON_NULL, (empty)
                    // 2705: b"invalid argum ... t'\0": typeof(_230) = & {l367} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                    // 2705: b"invalid argum ... t'\0":   l367 = UNIQUE | NON_NULL, FIXED
                    // 2705: b"invalid argum ... st u8: typeof(_228 = move _229 as *const u8 (Pointer(ArrayToPointer))) = *const {l3785} u8
                    // 2705: b"invalid argum ... st u8:   l3785 = UNIQUE | NON_NULL, (empty)
                    // 2705: b"invalid argum ... _char: typeof(_227 = move _228 as *const i8 (Misc)) = *const {l3786} i8
                    // 2705: b"invalid argum ... _char:   l3786 = UNIQUE | NON_NULL, (empty)
                    // 2705: b"invalid argum ... t'\0": typeof(_230 = const b"invalid argument \'%s\' to \'-t\'\x00") = & {l3783} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                    // 2705: b"invalid argum ... t'\0":   l3783 = UNIQUE | NON_NULL, (empty)
                    // 2705: b"invalid argum ... t'\0": typeof(_229 = &raw const (*_230)) = *const {l3784} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                    // 2705: b"invalid argum ... t'\0":   l3784 = UNIQUE | NON_NULL, (empty)
                    arg,
                    // 2706: arg: typeof(_231) = *const {l369} i8
                    // 2706: arg:   l369 = UNIQUE | NON_NULL, (empty)
                );
            }
        } else if strcmp(
            *argv.offset(i as isize),
            // 2710: *argv.offset(i  ... size): typeof(_234) = *const {l373} i8
            // 2710: *argv.offset(i  ... size):   l373 = UNIQUE | NON_NULL, (empty)
            // 2710: *argv.offset(i  ... size): typeof(_235) = *mut {l375} i8
            // 2710: *argv.offset(i  ... size):   l375 = UNIQUE | NON_NULL, (empty)
            // 2710: argv.offset(i a ... size): typeof(_236) = *mut {l377} *mut {l378} i8
            // 2710: argv.offset(i a ... size):   l377 = UNIQUE | NON_NULL, (empty)
            // 2710: argv.offset(i a ... size):   l378 = UNIQUE | NON_NULL, (empty)
            // 2710: argv: typeof(_237) = *mut {l380} *mut {l381} i8
            // 2710: argv:   l380 = UNIQUE | NON_NULL, (empty)
            // 2710: argv:   l381 = UNIQUE | NON_NULL, (empty)
            // 2710: *argv.offset(i  ... size): typeof(_234 = move _235 as *const i8 (Pointer(MutToConstPointer))) = *const {l3787} i8
            // 2710: *argv.offset(i  ... size):   l3787 = UNIQUE | NON_NULL, (empty)
            b"-m\0" as *const u8 as *const libc::c_char,
            // 2711: b"-m\0" as *con ... _char: typeof(_240) = *const {l385} i8
            // 2711: b"-m\0" as *con ... _char:   l385 = UNIQUE | NON_NULL, (empty)
            // 2711: b"-m\0" as *const u8: typeof(_241) = *const {l387} u8
            // 2711: b"-m\0" as *const u8:   l387 = UNIQUE | NON_NULL, (empty)
            // 2711: b"-m\0": typeof(_242) = *const {l389} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2711: b"-m\0":   l389 = UNIQUE | NON_NULL, (empty)
            // 2711: b"-m\0": typeof(_243) = & {l391} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2711: b"-m\0":   l391 = UNIQUE | NON_NULL, FIXED
            // 2711: b"-m\0": typeof(_243 = const b"-m\x00") = & {l3788} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2711: b"-m\0":   l3788 = UNIQUE | NON_NULL, (empty)
            // 2711: b"-m\0" as *con ... _char: typeof(_240 = move _241 as *const i8 (Misc)) = *const {l3791} i8
            // 2711: b"-m\0" as *con ... _char:   l3791 = UNIQUE | NON_NULL, (empty)
            // 2711: b"-m\0": typeof(_242 = &raw const (*_243)) = *const {l3789} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2711: b"-m\0":   l3789 = UNIQUE | NON_NULL, (empty)
            // 2711: b"-m\0" as *const u8: typeof(_241 = move _242 as *const u8 (Pointer(ArrayToPointer))) = *const {l3790} u8
            // 2711: b"-m\0" as *const u8:   l3790 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            if memlimit != 0 {
            // 2714: memlimit: typeof(_247) = *mut {l396} i64
            // 2714: memlimit:   l396 = UNIQUE | NON_NULL, (empty)
                die(b"multiple '-m' or '-g' options\0" as *const u8 as *const libc::c_char);
                // 2715: b"multiple '-m' ... _char: typeof(_249) = *const {l399} i8
                // 2715: b"multiple '-m' ... _char:   l399 = UNIQUE | NON_NULL, (empty)
                // 2715: b"multiple '-m' ... st u8: typeof(_250) = *const {l401} u8
                // 2715: b"multiple '-m' ... st u8:   l401 = UNIQUE | NON_NULL, (empty)
                // 2715: b"multiple '-m' ... ns\0": typeof(_251) = *const {l403} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                // 2715: b"multiple '-m' ... ns\0":   l403 = UNIQUE | NON_NULL, (empty)
                // 2715: b"multiple '-m' ... ns\0": typeof(_252) = & {l405} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                // 2715: b"multiple '-m' ... ns\0":   l405 = UNIQUE | NON_NULL, FIXED
                // 2715: b"multiple '-m' ... _char: typeof(_249 = move _250 as *const i8 (Misc)) = *const {l3795} i8
                // 2715: b"multiple '-m' ... _char:   l3795 = UNIQUE | NON_NULL, (empty)
                // 2715: b"multiple '-m' ... ns\0": typeof(_252 = const b"multiple \'-m\' or \'-g\' options\x00") = & {l3792} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                // 2715: b"multiple '-m' ... ns\0":   l3792 = UNIQUE | NON_NULL, (empty)
                // 2715: b"multiple '-m' ... ns\0": typeof(_251 = &raw const (*_252)) = *const {l3793} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                // 2715: b"multiple '-m' ... ns\0":   l3793 = UNIQUE | NON_NULL, (empty)
                // 2715: b"multiple '-m' ... st u8: typeof(_250 = move _251 as *const u8 (Pointer(ArrayToPointer))) = *const {l3794} u8
                // 2715: b"multiple '-m' ... st u8:   l3794 = UNIQUE | NON_NULL, (empty)
            }
            if i + 1 as libc::c_int == argc {
                die(b"argument to '-m' missing\0" as *const u8 as *const libc::c_char);
                // 2718: b"argument to ' ... _char: typeof(_261) = *const {l415} i8
                // 2718: b"argument to ' ... _char:   l415 = UNIQUE | NON_NULL, (empty)
                // 2718: b"argument to ' ... st u8: typeof(_262) = *const {l417} u8
                // 2718: b"argument to ' ... st u8:   l417 = UNIQUE | NON_NULL, (empty)
                // 2718: b"argument to ' ... ng\0": typeof(_263) = *const {l419} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2718: b"argument to ' ... ng\0":   l419 = UNIQUE | NON_NULL, (empty)
                // 2718: b"argument to ' ... ng\0": typeof(_264) = & {l421} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2718: b"argument to ' ... ng\0":   l421 = UNIQUE | NON_NULL, FIXED
                // 2718: b"argument to ' ... st u8: typeof(_262 = move _263 as *const u8 (Pointer(ArrayToPointer))) = *const {l3798} u8
                // 2718: b"argument to ' ... st u8:   l3798 = UNIQUE | NON_NULL, (empty)
                // 2718: b"argument to ' ... _char: typeof(_261 = move _262 as *const i8 (Misc)) = *const {l3799} i8
                // 2718: b"argument to ' ... _char:   l3799 = UNIQUE | NON_NULL, (empty)
                // 2718: b"argument to ' ... ng\0": typeof(_264 = const b"argument to \'-m\' missing\x00") = & {l3796} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2718: b"argument to ' ... ng\0":   l3796 = UNIQUE | NON_NULL, (empty)
                // 2718: b"argument to ' ... ng\0": typeof(_263 = &raw const (*_264)) = *const {l3797} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2718: b"argument to ' ... ng\0":   l3797 = UNIQUE | NON_NULL, (empty)
            }
            i += 1;
            arg = *argv.offset(i as isize);
            // 2721: *argv.offset(i  ... size): typeof(_266) = *mut {l424} i8
            // 2721: *argv.offset(i  ... size):   l424 = UNIQUE | NON_NULL, (empty)
            // 2721: argv.offset(i a ... size): typeof(_267) = *mut {l426} *mut {l427} i8
            // 2721: argv.offset(i a ... size):   l426 = UNIQUE | NON_NULL, (empty)
            // 2721: argv.offset(i a ... size):   l427 = UNIQUE | NON_NULL, (empty)
            // 2721: argv: typeof(_268) = *mut {l429} *mut {l430} i8
            // 2721: argv:   l429 = UNIQUE | NON_NULL, (empty)
            // 2721: argv:   l430 = UNIQUE | NON_NULL, (empty)
            // 2721: arg = *argv.off ... size): typeof(_25 = move _266 as *const i8 (Pointer(MutToConstPointer))) = *const {l3800} i8
            // 2721: arg = *argv.off ... size):   l3800 = UNIQUE | NON_NULL, (empty)
            if isposnum(arg) == 0 || {
            // 2722: arg: typeof(_274) = *const {l437} i8
            // 2722: arg:   l437 = UNIQUE | NON_NULL, (empty)
                memlimit = (atoll(arg) << 20 as libc::c_int) as int64_t;
                // 2723: arg: typeof(_278) = *const {l442} i8
                // 2723: arg:   l442 = UNIQUE | NON_NULL, (empty)
                // 2723: memlimit: typeof(_281) = *mut {l446} i64
                // 2723: memlimit:   l446 = UNIQUE | NON_NULL, (empty)
                memlimit <= 0 as libc::c_int as int64_t
                // 2724: memlimit: typeof(_283) = *mut {l449} i64
                // 2724: memlimit:   l449 = UNIQUE | NON_NULL, (empty)
            } {
                die(
                    b"invalid argument '%s' to '-m'\0" as *const u8 as *const libc::c_char,
                    // 2727: b"invalid argum ... _char: typeof(_287) = *const {l454} i8
                    // 2727: b"invalid argum ... _char:   l454 = UNIQUE | NON_NULL, (empty)
                    // 2727: b"invalid argum ... st u8: typeof(_288) = *const {l456} u8
                    // 2727: b"invalid argum ... st u8:   l456 = UNIQUE | NON_NULL, (empty)
                    // 2727: b"invalid argum ... m'\0": typeof(_289) = *const {l458} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                    // 2727: b"invalid argum ... m'\0":   l458 = UNIQUE | NON_NULL, (empty)
                    // 2727: b"invalid argum ... m'\0": typeof(_290) = & {l460} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                    // 2727: b"invalid argum ... m'\0":   l460 = UNIQUE | NON_NULL, FIXED
                    // 2727: b"invalid argum ... _char: typeof(_287 = move _288 as *const i8 (Misc)) = *const {l3804} i8
                    // 2727: b"invalid argum ... _char:   l3804 = UNIQUE | NON_NULL, (empty)
                    // 2727: b"invalid argum ... m'\0": typeof(_289 = &raw const (*_290)) = *const {l3802} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                    // 2727: b"invalid argum ... m'\0":   l3802 = UNIQUE | NON_NULL, (empty)
                    // 2727: b"invalid argum ... m'\0": typeof(_290 = const b"invalid argument \'%s\' to \'-m\'\x00") = & {l3801} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                    // 2727: b"invalid argum ... m'\0":   l3801 = UNIQUE | NON_NULL, (empty)
                    // 2727: b"invalid argum ... st u8: typeof(_288 = move _289 as *const u8 (Pointer(ArrayToPointer))) = *const {l3803} u8
                    // 2727: b"invalid argum ... st u8:   l3803 = UNIQUE | NON_NULL, (empty)
                    arg,
                    // 2728: arg: typeof(_291) = *const {l462} i8
                    // 2728: arg:   l462 = UNIQUE | NON_NULL, (empty)
                );
            }
        } else if strcmp(
            *argv.offset(i as isize),
            // 2732: *argv.offset(i  ... size): typeof(_294) = *const {l466} i8
            // 2732: *argv.offset(i  ... size):   l466 = UNIQUE | NON_NULL, (empty)
            // 2732: *argv.offset(i  ... size): typeof(_295) = *mut {l468} i8
            // 2732: *argv.offset(i  ... size):   l468 = UNIQUE | NON_NULL, (empty)
            // 2732: argv.offset(i a ... size): typeof(_296) = *mut {l470} *mut {l471} i8
            // 2732: argv.offset(i a ... size):   l470 = UNIQUE | NON_NULL, (empty)
            // 2732: argv.offset(i a ... size):   l471 = UNIQUE | NON_NULL, (empty)
            // 2732: argv: typeof(_297) = *mut {l473} *mut {l474} i8
            // 2732: argv:   l473 = UNIQUE | NON_NULL, (empty)
            // 2732: argv:   l474 = UNIQUE | NON_NULL, (empty)
            // 2732: *argv.offset(i  ... size): typeof(_294 = move _295 as *const i8 (Pointer(MutToConstPointer))) = *const {l3805} i8
            // 2732: *argv.offset(i  ... size):   l3805 = UNIQUE | NON_NULL, (empty)
            b"-g\0" as *const u8 as *const libc::c_char,
            // 2733: b"-g\0" as *con ... _char: typeof(_300) = *const {l478} i8
            // 2733: b"-g\0" as *con ... _char:   l478 = UNIQUE | NON_NULL, (empty)
            // 2733: b"-g\0" as *const u8: typeof(_301) = *const {l480} u8
            // 2733: b"-g\0" as *const u8:   l480 = UNIQUE | NON_NULL, (empty)
            // 2733: b"-g\0": typeof(_302) = *const {l482} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2733: b"-g\0":   l482 = UNIQUE | NON_NULL, (empty)
            // 2733: b"-g\0": typeof(_303) = & {l484} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2733: b"-g\0":   l484 = UNIQUE | NON_NULL, FIXED
            // 2733: b"-g\0": typeof(_302 = &raw const (*_303)) = *const {l3807} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2733: b"-g\0":   l3807 = UNIQUE | NON_NULL, (empty)
            // 2733: b"-g\0" as *con ... _char: typeof(_300 = move _301 as *const i8 (Misc)) = *const {l3809} i8
            // 2733: b"-g\0" as *con ... _char:   l3809 = UNIQUE | NON_NULL, (empty)
            // 2733: b"-g\0": typeof(_303 = const b"-g\x00") = & {l3806} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2733: b"-g\0":   l3806 = UNIQUE | NON_NULL, (empty)
            // 2733: b"-g\0" as *const u8: typeof(_301 = move _302 as *const u8 (Pointer(ArrayToPointer))) = *const {l3808} u8
            // 2733: b"-g\0" as *const u8:   l3808 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            if memlimit != 0 {
            // 2736: memlimit: typeof(_307) = *mut {l489} i64
            // 2736: memlimit:   l489 = UNIQUE | NON_NULL, (empty)
                die(b"multiple '-g' or '-m' options\0" as *const u8 as *const libc::c_char);
                // 2737: b"multiple '-g' ... _char: typeof(_309) = *const {l492} i8
                // 2737: b"multiple '-g' ... _char:   l492 = UNIQUE | NON_NULL, (empty)
                // 2737: b"multiple '-g' ... st u8: typeof(_310) = *const {l494} u8
                // 2737: b"multiple '-g' ... st u8:   l494 = UNIQUE | NON_NULL, (empty)
                // 2737: b"multiple '-g' ... ns\0": typeof(_311) = *const {l496} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                // 2737: b"multiple '-g' ... ns\0":   l496 = UNIQUE | NON_NULL, (empty)
                // 2737: b"multiple '-g' ... ns\0": typeof(_312) = & {l498} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                // 2737: b"multiple '-g' ... ns\0":   l498 = UNIQUE | NON_NULL, FIXED
                // 2737: b"multiple '-g' ... ns\0": typeof(_311 = &raw const (*_312)) = *const {l3811} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                // 2737: b"multiple '-g' ... ns\0":   l3811 = UNIQUE | NON_NULL, (empty)
                // 2737: b"multiple '-g' ... st u8: typeof(_310 = move _311 as *const u8 (Pointer(ArrayToPointer))) = *const {l3812} u8
                // 2737: b"multiple '-g' ... st u8:   l3812 = UNIQUE | NON_NULL, (empty)
                // 2737: b"multiple '-g' ... ns\0": typeof(_312 = const b"multiple \'-g\' or \'-m\' options\x00") = & {l3810} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                // 2737: b"multiple '-g' ... ns\0":   l3810 = UNIQUE | NON_NULL, (empty)
                // 2737: b"multiple '-g' ... _char: typeof(_309 = move _310 as *const i8 (Misc)) = *const {l3813} i8
                // 2737: b"multiple '-g' ... _char:   l3813 = UNIQUE | NON_NULL, (empty)
            }
            if i + 1 as libc::c_int == argc {
                die(b"argument to '-g' missing\0" as *const u8 as *const libc::c_char);
                // 2740: b"argument to ' ... _char: typeof(_321) = *const {l508} i8
                // 2740: b"argument to ' ... _char:   l508 = UNIQUE | NON_NULL, (empty)
                // 2740: b"argument to ' ... st u8: typeof(_322) = *const {l510} u8
                // 2740: b"argument to ' ... st u8:   l510 = UNIQUE | NON_NULL, (empty)
                // 2740: b"argument to ' ... ng\0": typeof(_323) = *const {l512} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2740: b"argument to ' ... ng\0":   l512 = UNIQUE | NON_NULL, (empty)
                // 2740: b"argument to ' ... ng\0": typeof(_324) = & {l514} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2740: b"argument to ' ... ng\0":   l514 = UNIQUE | NON_NULL, FIXED
                // 2740: b"argument to ' ... ng\0": typeof(_324 = const b"argument to \'-g\' missing\x00") = & {l3814} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2740: b"argument to ' ... ng\0":   l3814 = UNIQUE | NON_NULL, (empty)
                // 2740: b"argument to ' ... _char: typeof(_321 = move _322 as *const i8 (Misc)) = *const {l3817} i8
                // 2740: b"argument to ' ... _char:   l3817 = UNIQUE | NON_NULL, (empty)
                // 2740: b"argument to ' ... st u8: typeof(_322 = move _323 as *const u8 (Pointer(ArrayToPointer))) = *const {l3816} u8
                // 2740: b"argument to ' ... st u8:   l3816 = UNIQUE | NON_NULL, (empty)
                // 2740: b"argument to ' ... ng\0": typeof(_323 = &raw const (*_324)) = *const {l3815} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2740: b"argument to ' ... ng\0":   l3815 = UNIQUE | NON_NULL, (empty)
            }
            i += 1;
            arg = *argv.offset(i as isize);
            // 2743: *argv.offset(i  ... size): typeof(_326) = *mut {l517} i8
            // 2743: *argv.offset(i  ... size):   l517 = UNIQUE | NON_NULL, (empty)
            // 2743: argv.offset(i a ... size): typeof(_327) = *mut {l519} *mut {l520} i8
            // 2743: argv.offset(i a ... size):   l519 = UNIQUE | NON_NULL, (empty)
            // 2743: argv.offset(i a ... size):   l520 = UNIQUE | NON_NULL, (empty)
            // 2743: argv: typeof(_328) = *mut {l522} *mut {l523} i8
            // 2743: argv:   l522 = UNIQUE | NON_NULL, (empty)
            // 2743: argv:   l523 = UNIQUE | NON_NULL, (empty)
            // 2743: arg = *argv.off ... size): typeof(_25 = move _326 as *const i8 (Pointer(MutToConstPointer))) = *const {l3818} i8
            // 2743: arg = *argv.off ... size):   l3818 = UNIQUE | NON_NULL, (empty)
            if isposnum(arg) == 0 || {
            // 2744: arg: typeof(_334) = *const {l530} i8
            // 2744: arg:   l530 = UNIQUE | NON_NULL, (empty)
                memlimit = (atoll(arg) << 30 as libc::c_int) as int64_t;
                // 2745: arg: typeof(_338) = *const {l535} i8
                // 2745: arg:   l535 = UNIQUE | NON_NULL, (empty)
                // 2745: memlimit: typeof(_341) = *mut {l539} i64
                // 2745: memlimit:   l539 = UNIQUE | NON_NULL, (empty)
                memlimit <= 0 as libc::c_int as int64_t
                // 2746: memlimit: typeof(_343) = *mut {l542} i64
                // 2746: memlimit:   l542 = UNIQUE | NON_NULL, (empty)
            } {
                die(
                    b"invalid argument '%s' to '-g'\0" as *const u8 as *const libc::c_char,
                    // 2749: b"invalid argum ... _char: typeof(_347) = *const {l547} i8
                    // 2749: b"invalid argum ... _char:   l547 = UNIQUE | NON_NULL, (empty)
                    // 2749: b"invalid argum ... st u8: typeof(_348) = *const {l549} u8
                    // 2749: b"invalid argum ... st u8:   l549 = UNIQUE | NON_NULL, (empty)
                    // 2749: b"invalid argum ... g'\0": typeof(_349) = *const {l551} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                    // 2749: b"invalid argum ... g'\0":   l551 = UNIQUE | NON_NULL, (empty)
                    // 2749: b"invalid argum ... g'\0": typeof(_350) = & {l553} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                    // 2749: b"invalid argum ... g'\0":   l553 = UNIQUE | NON_NULL, FIXED
                    // 2749: b"invalid argum ... g'\0": typeof(_350 = const b"invalid argument \'%s\' to \'-g\'\x00") = & {l3819} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                    // 2749: b"invalid argum ... g'\0":   l3819 = UNIQUE | NON_NULL, (empty)
                    // 2749: b"invalid argum ... st u8: typeof(_348 = move _349 as *const u8 (Pointer(ArrayToPointer))) = *const {l3821} u8
                    // 2749: b"invalid argum ... st u8:   l3821 = UNIQUE | NON_NULL, (empty)
                    // 2749: b"invalid argum ... _char: typeof(_347 = move _348 as *const i8 (Misc)) = *const {l3822} i8
                    // 2749: b"invalid argum ... _char:   l3822 = UNIQUE | NON_NULL, (empty)
                    // 2749: b"invalid argum ... g'\0": typeof(_349 = &raw const (*_350)) = *const {l3820} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                    // 2749: b"invalid argum ... g'\0":   l3820 = UNIQUE | NON_NULL, (empty)
                    arg,
                    // 2750: arg: typeof(_351) = *const {l555} i8
                    // 2750: arg:   l555 = UNIQUE | NON_NULL, (empty)
                );
            }
        } else if strcmp(
            *argv.offset(i as isize),
            // 2754: *argv.offset(i  ... size): typeof(_354) = *const {l559} i8
            // 2754: *argv.offset(i  ... size):   l559 = UNIQUE | NON_NULL, (empty)
            // 2754: *argv.offset(i  ... size): typeof(_355) = *mut {l561} i8
            // 2754: *argv.offset(i  ... size):   l561 = UNIQUE | NON_NULL, (empty)
            // 2754: argv.offset(i a ... size): typeof(_356) = *mut {l563} *mut {l564} i8
            // 2754: argv.offset(i a ... size):   l563 = UNIQUE | NON_NULL, (empty)
            // 2754: argv.offset(i a ... size):   l564 = UNIQUE | NON_NULL, (empty)
            // 2754: argv: typeof(_357) = *mut {l566} *mut {l567} i8
            // 2754: argv:   l566 = UNIQUE | NON_NULL, (empty)
            // 2754: argv:   l567 = UNIQUE | NON_NULL, (empty)
            // 2754: *argv.offset(i  ... size): typeof(_354 = move _355 as *const i8 (Pointer(MutToConstPointer))) = *const {l3823} i8
            // 2754: *argv.offset(i  ... size):   l3823 = UNIQUE | NON_NULL, (empty)
            b"-l\0" as *const u8 as *const libc::c_char,
            // 2755: b"-l\0" as *con ... _char: typeof(_360) = *const {l571} i8
            // 2755: b"-l\0" as *con ... _char:   l571 = UNIQUE | NON_NULL, (empty)
            // 2755: b"-l\0" as *const u8: typeof(_361) = *const {l573} u8
            // 2755: b"-l\0" as *const u8:   l573 = UNIQUE | NON_NULL, (empty)
            // 2755: b"-l\0": typeof(_362) = *const {l575} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2755: b"-l\0":   l575 = UNIQUE | NON_NULL, (empty)
            // 2755: b"-l\0": typeof(_363) = & {l577} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2755: b"-l\0":   l577 = UNIQUE | NON_NULL, FIXED
            // 2755: b"-l\0": typeof(_362 = &raw const (*_363)) = *const {l3825} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2755: b"-l\0":   l3825 = UNIQUE | NON_NULL, (empty)
            // 2755: b"-l\0": typeof(_363 = const b"-l\x00") = & {l3824} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2755: b"-l\0":   l3824 = UNIQUE | NON_NULL, (empty)
            // 2755: b"-l\0" as *con ... _char: typeof(_360 = move _361 as *const i8 (Misc)) = *const {l3827} i8
            // 2755: b"-l\0" as *con ... _char:   l3827 = UNIQUE | NON_NULL, (empty)
            // 2755: b"-l\0" as *const u8: typeof(_361 = move _362 as *const u8 (Pointer(ArrayToPointer))) = *const {l3826} u8
            // 2755: b"-l\0" as *const u8:   l3826 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            if gclimset != 0 {
            // 2758: gclimset: typeof(_367) = *mut {l582} i32
            // 2758: gclimset:   l582 = UNIQUE | NON_NULL, (empty)
                die(b"multiple '-l' options\0" as *const u8 as *const libc::c_char);
                // 2759: b"multiple '-l' ... _char: typeof(_369) = *const {l585} i8
                // 2759: b"multiple '-l' ... _char:   l585 = UNIQUE | NON_NULL, (empty)
                // 2759: b"multiple '-l' ... st u8: typeof(_370) = *const {l587} u8
                // 2759: b"multiple '-l' ... st u8:   l587 = UNIQUE | NON_NULL, (empty)
                // 2759: b"multiple '-l' ... ns\0": typeof(_371) = *const {l589} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 2759: b"multiple '-l' ... ns\0":   l589 = UNIQUE | NON_NULL, (empty)
                // 2759: b"multiple '-l' ... ns\0": typeof(_372) = & {l591} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 2759: b"multiple '-l' ... ns\0":   l591 = UNIQUE | NON_NULL, FIXED
                // 2759: b"multiple '-l' ... ns\0": typeof(_372 = const b"multiple \'-l\' options\x00") = & {l3828} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 2759: b"multiple '-l' ... ns\0":   l3828 = UNIQUE | NON_NULL, (empty)
                // 2759: b"multiple '-l' ... st u8: typeof(_370 = move _371 as *const u8 (Pointer(ArrayToPointer))) = *const {l3830} u8
                // 2759: b"multiple '-l' ... st u8:   l3830 = UNIQUE | NON_NULL, (empty)
                // 2759: b"multiple '-l' ... ns\0": typeof(_371 = &raw const (*_372)) = *const {l3829} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 2759: b"multiple '-l' ... ns\0":   l3829 = UNIQUE | NON_NULL, (empty)
                // 2759: b"multiple '-l' ... _char: typeof(_369 = move _370 as *const i8 (Misc)) = *const {l3831} i8
                // 2759: b"multiple '-l' ... _char:   l3831 = UNIQUE | NON_NULL, (empty)
            }
            if i + 1 as libc::c_int == argc {
                die(b"argument to '-l' missing\0" as *const u8 as *const libc::c_char);
                // 2762: b"argument to ' ... _char: typeof(_381) = *const {l601} i8
                // 2762: b"argument to ' ... _char:   l601 = UNIQUE | NON_NULL, (empty)
                // 2762: b"argument to ' ... st u8: typeof(_382) = *const {l603} u8
                // 2762: b"argument to ' ... st u8:   l603 = UNIQUE | NON_NULL, (empty)
                // 2762: b"argument to ' ... ng\0": typeof(_383) = *const {l605} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2762: b"argument to ' ... ng\0":   l605 = UNIQUE | NON_NULL, (empty)
                // 2762: b"argument to ' ... ng\0": typeof(_384) = & {l607} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2762: b"argument to ' ... ng\0":   l607 = UNIQUE | NON_NULL, FIXED
                // 2762: b"argument to ' ... st u8: typeof(_382 = move _383 as *const u8 (Pointer(ArrayToPointer))) = *const {l3834} u8
                // 2762: b"argument to ' ... st u8:   l3834 = UNIQUE | NON_NULL, (empty)
                // 2762: b"argument to ' ... ng\0": typeof(_383 = &raw const (*_384)) = *const {l3833} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2762: b"argument to ' ... ng\0":   l3833 = UNIQUE | NON_NULL, (empty)
                // 2762: b"argument to ' ... _char: typeof(_381 = move _382 as *const i8 (Misc)) = *const {l3835} i8
                // 2762: b"argument to ' ... _char:   l3835 = UNIQUE | NON_NULL, (empty)
                // 2762: b"argument to ' ... ng\0": typeof(_384 = const b"argument to \'-l\' missing\x00") = & {l3832} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2762: b"argument to ' ... ng\0":   l3832 = UNIQUE | NON_NULL, (empty)
            }
            i += 1;
            if isposnum(*argv.offset(i as isize)) == 0 {
            // 2765: *argv.offset(i  ... size): typeof(_389) = *const {l613} i8
            // 2765: *argv.offset(i  ... size):   l613 = UNIQUE | NON_NULL, (empty)
            // 2765: *argv.offset(i  ... size): typeof(_390) = *mut {l615} i8
            // 2765: *argv.offset(i  ... size):   l615 = UNIQUE | NON_NULL, (empty)
            // 2765: argv.offset(i a ... size): typeof(_391) = *mut {l617} *mut {l618} i8
            // 2765: argv.offset(i a ... size):   l617 = UNIQUE | NON_NULL, (empty)
            // 2765: argv.offset(i a ... size):   l618 = UNIQUE | NON_NULL, (empty)
            // 2765: argv: typeof(_392) = *mut {l620} *mut {l621} i8
            // 2765: argv:   l620 = UNIQUE | NON_NULL, (empty)
            // 2765: argv:   l621 = UNIQUE | NON_NULL, (empty)
            // 2765: *argv.offset(i  ... size): typeof(_389 = move _390 as *const i8 (Pointer(MutToConstPointer))) = *const {l3836} i8
            // 2765: *argv.offset(i  ... size):   l3836 = UNIQUE | NON_NULL, (empty)
                die(
                    b"invalid argument '%s' to '-l'\0" as *const u8 as *const libc::c_char,
                    // 2767: b"invalid argum ... _char: typeof(_396) = *const {l626} i8
                    // 2767: b"invalid argum ... _char:   l626 = UNIQUE | NON_NULL, (empty)
                    // 2767: b"invalid argum ... st u8: typeof(_397) = *const {l628} u8
                    // 2767: b"invalid argum ... st u8:   l628 = UNIQUE | NON_NULL, (empty)
                    // 2767: b"invalid argum ... l'\0": typeof(_398) = *const {l630} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                    // 2767: b"invalid argum ... l'\0":   l630 = UNIQUE | NON_NULL, (empty)
                    // 2767: b"invalid argum ... l'\0": typeof(_399) = & {l632} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                    // 2767: b"invalid argum ... l'\0":   l632 = UNIQUE | NON_NULL, FIXED
                    // 2767: b"invalid argum ... l'\0": typeof(_398 = &raw const (*_399)) = *const {l3838} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                    // 2767: b"invalid argum ... l'\0":   l3838 = UNIQUE | NON_NULL, (empty)
                    // 2767: b"invalid argum ... l'\0": typeof(_399 = const b"invalid argument \'%s\' to \'-l\'\x00") = & {l3837} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
                    // 2767: b"invalid argum ... l'\0":   l3837 = UNIQUE | NON_NULL, (empty)
                    // 2767: b"invalid argum ... st u8: typeof(_397 = move _398 as *const u8 (Pointer(ArrayToPointer))) = *const {l3839} u8
                    // 2767: b"invalid argum ... st u8:   l3839 = UNIQUE | NON_NULL, (empty)
                    // 2767: b"invalid argum ... _char: typeof(_396 = move _397 as *const i8 (Misc)) = *const {l3840} i8
                    // 2767: b"invalid argum ... _char:   l3840 = UNIQUE | NON_NULL, (empty)
                    *argv.offset(i as isize),
                    // 2768: *argv.offset(i  ... size): typeof(_400) = *mut {l634} i8
                    // 2768: *argv.offset(i  ... size):   l634 = UNIQUE | NON_NULL, (empty)
                    // 2768: argv.offset(i a ... size): typeof(_401) = *mut {l636} *mut {l637} i8
                    // 2768: argv.offset(i a ... size):   l636 = UNIQUE | NON_NULL, (empty)
                    // 2768: argv.offset(i a ... size):   l637 = UNIQUE | NON_NULL, (empty)
                    // 2768: argv: typeof(_402) = *mut {l639} *mut {l640} i8
                    // 2768: argv:   l639 = UNIQUE | NON_NULL, (empty)
                    // 2768: argv:   l640 = UNIQUE | NON_NULL, (empty)
                );
            }
            gclim = atoi(*argv.offset(i as isize));
            // 2771: *argv.offset(i  ... size): typeof(_406) = *const {l645} i8
            // 2771: *argv.offset(i  ... size):   l645 = UNIQUE | NON_NULL, (empty)
            // 2771: *argv.offset(i  ... size): typeof(_407) = *mut {l647} i8
            // 2771: *argv.offset(i  ... size):   l647 = UNIQUE | NON_NULL, (empty)
            // 2771: argv.offset(i a ... size): typeof(_408) = *mut {l649} *mut {l650} i8
            // 2771: argv.offset(i a ... size):   l649 = UNIQUE | NON_NULL, (empty)
            // 2771: argv.offset(i a ... size):   l650 = UNIQUE | NON_NULL, (empty)
            // 2771: argv: typeof(_409) = *mut {l652} *mut {l653} i8
            // 2771: argv:   l652 = UNIQUE | NON_NULL, (empty)
            // 2771: argv:   l653 = UNIQUE | NON_NULL, (empty)
            // 2771: gclim: typeof(_412) = *mut {l657} i32
            // 2771: gclim:   l657 = UNIQUE | NON_NULL, (empty)
            // 2771: *argv.offset(i  ... size): typeof(_406 = move _407 as *const i8 (Pointer(MutToConstPointer))) = *const {l3841} i8
            // 2771: *argv.offset(i  ... size):   l3841 = UNIQUE | NON_NULL, (empty)
            gclimset = 1 as libc::c_int;
            // 2772: gclimset: typeof(_414) = *mut {l660} i32
            // 2772: gclimset:   l660 = UNIQUE | NON_NULL, (empty)
        } else if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
        // 2773: (*argv.offset(i ... size): typeof(_418) = *mut {l665} i8
        // 2773: (*argv.offset(i ... size):   l665 = UNIQUE | NON_NULL, (empty)
        // 2773: (*argv.offset(i ... ize)): typeof(_419) = *mut {l667} i8
        // 2773: (*argv.offset(i ... ize)):   l667 = UNIQUE | NON_NULL, (empty)
        // 2773: argv.offset(i a ... size): typeof(_420) = *mut {l669} *mut {l670} i8
        // 2773: argv.offset(i a ... size):   l669 = UNIQUE | NON_NULL, (empty)
        // 2773: argv.offset(i a ... size):   l670 = UNIQUE | NON_NULL, (empty)
        // 2773: argv: typeof(_421) = *mut {l672} *mut {l673} i8
        // 2773: argv:   l672 = UNIQUE | NON_NULL, (empty)
        // 2773: argv:   l673 = UNIQUE | NON_NULL, (empty)
            == '-' as i32
        {
            die(
                b"invalid option '%s' (try '-h')\0" as *const u8 as *const libc::c_char,
                // 2777: b"invalid optio ... _char: typeof(_428) = *const {l681} i8
                // 2777: b"invalid optio ... _char:   l681 = UNIQUE | NON_NULL, (empty)
                // 2777: b"invalid optio ... st u8: typeof(_429) = *const {l683} u8
                // 2777: b"invalid optio ... st u8:   l683 = UNIQUE | NON_NULL, (empty)
                // 2777: b"invalid optio ... ')\0": typeof(_430) = *const {l685} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                // 2777: b"invalid optio ... ')\0":   l685 = UNIQUE | NON_NULL, (empty)
                // 2777: b"invalid optio ... ')\0": typeof(_431) = & {l687} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                // 2777: b"invalid optio ... ')\0":   l687 = UNIQUE | NON_NULL, FIXED
                // 2777: b"invalid optio ... ')\0": typeof(_431 = const b"invalid option \'%s\' (try \'-h\')\x00") = & {l3842} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                // 2777: b"invalid optio ... ')\0":   l3842 = UNIQUE | NON_NULL, (empty)
                // 2777: b"invalid optio ... ')\0": typeof(_430 = &raw const (*_431)) = *const {l3843} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                // 2777: b"invalid optio ... ')\0":   l3843 = UNIQUE | NON_NULL, (empty)
                // 2777: b"invalid optio ... _char: typeof(_428 = move _429 as *const i8 (Misc)) = *const {l3845} i8
                // 2777: b"invalid optio ... _char:   l3845 = UNIQUE | NON_NULL, (empty)
                // 2777: b"invalid optio ... st u8: typeof(_429 = move _430 as *const u8 (Pointer(ArrayToPointer))) = *const {l3844} u8
                // 2777: b"invalid optio ... st u8:   l3844 = UNIQUE | NON_NULL, (empty)
                *argv.offset(i as isize),
                // 2778: *argv.offset(i  ... size): typeof(_432) = *mut {l689} i8
                // 2778: *argv.offset(i  ... size):   l689 = UNIQUE | NON_NULL, (empty)
                // 2778: argv.offset(i a ... size): typeof(_433) = *mut {l691} *mut {l692} i8
                // 2778: argv.offset(i a ... size):   l691 = UNIQUE | NON_NULL, (empty)
                // 2778: argv.offset(i a ... size):   l692 = UNIQUE | NON_NULL, (empty)
                // 2778: argv: typeof(_434) = *mut {l694} *mut {l695} i8
                // 2778: argv:   l694 = UNIQUE | NON_NULL, (empty)
                // 2778: argv:   l695 = UNIQUE | NON_NULL, (empty)
            );
        } else if name.is_null() && isposnum(*argv.offset(i as isize)) != 0 {
        // 2780: name: typeof(_439) = *const {l701} i8
        // 2780: name:   l701 = UNIQUE | NON_NULL, (empty)
        // 2780: name: typeof(_440) = *mut {l703} *const {l704} i8
        // 2780: name:   l703 = UNIQUE | NON_NULL, (empty)
        // 2780: name:   l704 = UNIQUE | NON_NULL, (empty)
        // 2780: *argv.offset(i  ... size): typeof(_443) = *const {l708} i8
        // 2780: *argv.offset(i  ... size):   l708 = UNIQUE | NON_NULL, (empty)
        // 2780: *argv.offset(i  ... size): typeof(_444) = *mut {l710} i8
        // 2780: *argv.offset(i  ... size):   l710 = UNIQUE | NON_NULL, (empty)
        // 2780: argv.offset(i a ... size): typeof(_445) = *mut {l712} *mut {l713} i8
        // 2780: argv.offset(i a ... size):   l712 = UNIQUE | NON_NULL, (empty)
        // 2780: argv.offset(i a ... size):   l713 = UNIQUE | NON_NULL, (empty)
        // 2780: argv: typeof(_446) = *mut {l715} *mut {l716} i8
        // 2780: argv:   l715 = UNIQUE | NON_NULL, (empty)
        // 2780: argv:   l716 = UNIQUE | NON_NULL, (empty)
        // 2780: *argv.offset(i  ... size): typeof(_443 = move _444 as *const i8 (Pointer(MutToConstPointer))) = *const {l3846} i8
        // 2780: *argv.offset(i  ... size):   l3846 = UNIQUE | NON_NULL, (empty)
            die(
                b"<dimacs> file name can not be a positive number '%s'\0" as *const u8
                // 2782: b"<dimacs> file ... _char: typeof(_450) = *const {l721} i8
                // 2782: b"<dimacs> file ... _char:   l721 = UNIQUE | NON_NULL, (empty)
                // 2782: b"<dimacs> file ... st u8: typeof(_451) = *const {l723} u8
                // 2782: b"<dimacs> file ... st u8:   l723 = UNIQUE | NON_NULL, (empty)
                // 2782: b"<dimacs> file ... s'\0": typeof(_452) = *const {l725} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000035)) }]
                // 2782: b"<dimacs> file ... s'\0":   l725 = UNIQUE | NON_NULL, (empty)
                // 2782: b"<dimacs> file ... s'\0": typeof(_453) = & {l727} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000035)) }]
                // 2782: b"<dimacs> file ... s'\0":   l727 = UNIQUE | NON_NULL, FIXED
                // 2782: b"<dimacs> file ... s'\0": typeof(_453 = const b"<dimacs> file name can not be a positive number \'%s\'\x00") = & {l3847} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000035)) }]
                // 2782: b"<dimacs> file ... s'\0":   l3847 = UNIQUE | NON_NULL, (empty)
                // 2782: b"<dimacs> file ... st u8: typeof(_451 = move _452 as *const u8 (Pointer(ArrayToPointer))) = *const {l3849} u8
                // 2782: b"<dimacs> file ... st u8:   l3849 = UNIQUE | NON_NULL, (empty)
                // 2782: b"<dimacs> file ... _char: typeof(_450 = move _451 as *const i8 (Misc)) = *const {l3850} i8
                // 2782: b"<dimacs> file ... _char:   l3850 = UNIQUE | NON_NULL, (empty)
                // 2782: b"<dimacs> file ... s'\0": typeof(_452 = &raw const (*_453)) = *const {l3848} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000035)) }]
                // 2782: b"<dimacs> file ... s'\0":   l3848 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
                *argv.offset(i as isize),
                // 2784: *argv.offset(i  ... size): typeof(_454) = *mut {l729} i8
                // 2784: *argv.offset(i  ... size):   l729 = UNIQUE | NON_NULL, (empty)
                // 2784: argv.offset(i a ... size): typeof(_455) = *mut {l731} *mut {l732} i8
                // 2784: argv.offset(i a ... size):   l731 = UNIQUE | NON_NULL, (empty)
                // 2784: argv.offset(i a ... size):   l732 = UNIQUE | NON_NULL, (empty)
                // 2784: argv: typeof(_456) = *mut {l734} *mut {l735} i8
                // 2784: argv:   l734 = UNIQUE | NON_NULL, (empty)
                // 2784: argv:   l735 = UNIQUE | NON_NULL, (empty)
            );
        } else if !name.is_null() && nworkers2 != 0 {
        // 2786: name: typeof(_462) = *const {l742} i8
        // 2786: name:   l742 = UNIQUE | NON_NULL, (empty)
        // 2786: name: typeof(_463) = *mut {l744} *const {l745} i8
        // 2786: name:   l744 = UNIQUE | NON_NULL, (empty)
        // 2786: name:   l745 = UNIQUE | NON_NULL, (empty)
        // 2786: nworkers2: typeof(_466) = *mut {l749} i32
        // 2786: nworkers2:   l749 = UNIQUE | NON_NULL, (empty)
            die(
                b"too many arguments (including <dimacs> and <threads>)\0" as *const u8
                // 2788: b"too many argu ... _char: typeof(_468) = *const {l752} i8
                // 2788: b"too many argu ... _char:   l752 = UNIQUE | NON_NULL, (empty)
                // 2788: b"too many argu ... st u8: typeof(_469) = *const {l754} u8
                // 2788: b"too many argu ... st u8:   l754 = UNIQUE | NON_NULL, (empty)
                // 2788: b"too many argu ... >)\0": typeof(_470) = *const {l756} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000036)) }]
                // 2788: b"too many argu ... >)\0":   l756 = UNIQUE | NON_NULL, (empty)
                // 2788: b"too many argu ... >)\0": typeof(_471) = & {l758} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000036)) }]
                // 2788: b"too many argu ... >)\0":   l758 = UNIQUE | NON_NULL, FIXED
                // 2788: b"too many argu ... >)\0": typeof(_470 = &raw const (*_471)) = *const {l3852} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000036)) }]
                // 2788: b"too many argu ... >)\0":   l3852 = UNIQUE | NON_NULL, (empty)
                // 2788: b"too many argu ... _char: typeof(_468 = move _469 as *const i8 (Misc)) = *const {l3854} i8
                // 2788: b"too many argu ... _char:   l3854 = UNIQUE | NON_NULL, (empty)
                // 2788: b"too many argu ... >)\0": typeof(_471 = const b"too many arguments (including <dimacs> and <threads>)\x00") = & {l3851} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000036)) }]
                // 2788: b"too many argu ... >)\0":   l3851 = UNIQUE | NON_NULL, (empty)
                // 2788: b"too many argu ... st u8: typeof(_469 = move _470 as *const u8 (Pointer(ArrayToPointer))) = *const {l3853} u8
                // 2788: b"too many argu ... st u8:   l3853 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
        } else if !name.is_null() && isposnum(*argv.offset(i as isize)) == 0 {
        // 2791: name: typeof(_475) = *const {l763} i8
        // 2791: name:   l763 = UNIQUE | NON_NULL, (empty)
        // 2791: name: typeof(_476) = *mut {l765} *const {l766} i8
        // 2791: name:   l765 = UNIQUE | NON_NULL, (empty)
        // 2791: name:   l766 = UNIQUE | NON_NULL, (empty)
        // 2791: *argv.offset(i  ... size): typeof(_479) = *const {l770} i8
        // 2791: *argv.offset(i  ... size):   l770 = UNIQUE | NON_NULL, (empty)
        // 2791: *argv.offset(i  ... size): typeof(_480) = *mut {l772} i8
        // 2791: *argv.offset(i  ... size):   l772 = UNIQUE | NON_NULL, (empty)
        // 2791: argv.offset(i a ... size): typeof(_481) = *mut {l774} *mut {l775} i8
        // 2791: argv.offset(i a ... size):   l774 = UNIQUE | NON_NULL, (empty)
        // 2791: argv.offset(i a ... size):   l775 = UNIQUE | NON_NULL, (empty)
        // 2791: argv: typeof(_482) = *mut {l777} *mut {l778} i8
        // 2791: argv:   l777 = UNIQUE | NON_NULL, (empty)
        // 2791: argv:   l778 = UNIQUE | NON_NULL, (empty)
        // 2791: *argv.offset(i  ... size): typeof(_479 = move _480 as *const i8 (Pointer(MutToConstPointer))) = *const {l3855} i8
        // 2791: *argv.offset(i  ... size):   l3855 = UNIQUE | NON_NULL, (empty)
            die(
                b"expected positive number of threads but got '%s'\0" as *const u8
                // 2793: b"expected posi ... _char: typeof(_486) = *const {l783} i8
                // 2793: b"expected posi ... _char:   l783 = UNIQUE | NON_NULL, (empty)
                // 2793: b"expected posi ... st u8: typeof(_487) = *const {l785} u8
                // 2793: b"expected posi ... st u8:   l785 = UNIQUE | NON_NULL, (empty)
                // 2793: b"expected posi ... s'\0": typeof(_488) = *const {l787} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                // 2793: b"expected posi ... s'\0":   l787 = UNIQUE | NON_NULL, (empty)
                // 2793: b"expected posi ... s'\0": typeof(_489) = & {l789} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                // 2793: b"expected posi ... s'\0":   l789 = UNIQUE | NON_NULL, FIXED
                // 2793: b"expected posi ... st u8: typeof(_487 = move _488 as *const u8 (Pointer(ArrayToPointer))) = *const {l3858} u8
                // 2793: b"expected posi ... st u8:   l3858 = UNIQUE | NON_NULL, (empty)
                // 2793: b"expected posi ... _char: typeof(_486 = move _487 as *const i8 (Misc)) = *const {l3859} i8
                // 2793: b"expected posi ... _char:   l3859 = UNIQUE | NON_NULL, (empty)
                // 2793: b"expected posi ... s'\0": typeof(_488 = &raw const (*_489)) = *const {l3857} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                // 2793: b"expected posi ... s'\0":   l3857 = UNIQUE | NON_NULL, (empty)
                // 2793: b"expected posi ... s'\0": typeof(_489 = const b"expected positive number of threads but got \'%s\'\x00") = & {l3856} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                // 2793: b"expected posi ... s'\0":   l3856 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
                *argv.offset(i as isize),
                // 2795: *argv.offset(i  ... size): typeof(_490) = *mut {l791} i8
                // 2795: *argv.offset(i  ... size):   l791 = UNIQUE | NON_NULL, (empty)
                // 2795: argv.offset(i a ... size): typeof(_491) = *mut {l793} *mut {l794} i8
                // 2795: argv.offset(i a ... size):   l793 = UNIQUE | NON_NULL, (empty)
                // 2795: argv.offset(i a ... size):   l794 = UNIQUE | NON_NULL, (empty)
                // 2795: argv: typeof(_492) = *mut {l796} *mut {l797} i8
                // 2795: argv:   l796 = UNIQUE | NON_NULL, (empty)
                // 2795: argv:   l797 = UNIQUE | NON_NULL, (empty)
            );
        } else if !name.is_null() {
        // 2797: name: typeof(_497) = *const {l803} i8
        // 2797: name:   l803 = UNIQUE | NON_NULL, (empty)
        // 2797: name: typeof(_498) = *mut {l805} *const {l806} i8
        // 2797: name:   l805 = UNIQUE | NON_NULL, (empty)
        // 2797: name:   l806 = UNIQUE | NON_NULL, (empty)
            nworkers2 = atoi(*argv.offset(i as isize));
            // 2798: *argv.offset(i  ... size): typeof(_500) = *const {l809} i8
            // 2798: *argv.offset(i  ... size):   l809 = UNIQUE | NON_NULL, (empty)
            // 2798: *argv.offset(i  ... size): typeof(_501) = *mut {l811} i8
            // 2798: *argv.offset(i  ... size):   l811 = UNIQUE | NON_NULL, (empty)
            // 2798: argv.offset(i a ... size): typeof(_502) = *mut {l813} *mut {l814} i8
            // 2798: argv.offset(i a ... size):   l813 = UNIQUE | NON_NULL, (empty)
            // 2798: argv.offset(i a ... size):   l814 = UNIQUE | NON_NULL, (empty)
            // 2798: argv: typeof(_503) = *mut {l816} *mut {l817} i8
            // 2798: argv:   l816 = UNIQUE | NON_NULL, (empty)
            // 2798: argv:   l817 = UNIQUE | NON_NULL, (empty)
            // 2798: nworkers2: typeof(_506) = *mut {l821} i32
            // 2798: nworkers2:   l821 = UNIQUE | NON_NULL, (empty)
            // 2798: *argv.offset(i  ... size): typeof(_500 = move _501 as *const i8 (Pointer(MutToConstPointer))) = *const {l3860} i8
            // 2798: *argv.offset(i  ... size):   l3860 = UNIQUE | NON_NULL, (empty)
            if nworkers2 <= 0 as libc::c_int {
            // 2799: nworkers2: typeof(_509) = *mut {l825} i32
            // 2799: nworkers2:   l825 = UNIQUE | NON_NULL, (empty)
                die(
                    b"invalid number of threads '%s'\0" as *const u8 as *const libc::c_char,
                    // 2801: b"invalid numbe ... _char: typeof(_512) = *const {l829} i8
                    // 2801: b"invalid numbe ... _char:   l829 = UNIQUE | NON_NULL, (empty)
                    // 2801: b"invalid numbe ... st u8: typeof(_513) = *const {l831} u8
                    // 2801: b"invalid numbe ... st u8:   l831 = UNIQUE | NON_NULL, (empty)
                    // 2801: b"invalid numbe ... s'\0": typeof(_514) = *const {l833} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                    // 2801: b"invalid numbe ... s'\0":   l833 = UNIQUE | NON_NULL, (empty)
                    // 2801: b"invalid numbe ... s'\0": typeof(_515) = & {l835} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                    // 2801: b"invalid numbe ... s'\0":   l835 = UNIQUE | NON_NULL, FIXED
                    // 2801: b"invalid numbe ... s'\0": typeof(_514 = &raw const (*_515)) = *const {l3862} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                    // 2801: b"invalid numbe ... s'\0":   l3862 = UNIQUE | NON_NULL, (empty)
                    // 2801: b"invalid numbe ... st u8: typeof(_513 = move _514 as *const u8 (Pointer(ArrayToPointer))) = *const {l3863} u8
                    // 2801: b"invalid numbe ... st u8:   l3863 = UNIQUE | NON_NULL, (empty)
                    // 2801: b"invalid numbe ... _char: typeof(_512 = move _513 as *const i8 (Misc)) = *const {l3864} i8
                    // 2801: b"invalid numbe ... _char:   l3864 = UNIQUE | NON_NULL, (empty)
                    // 2801: b"invalid numbe ... s'\0": typeof(_515 = const b"invalid number of threads \'%s\'\x00") = & {l3861} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                    // 2801: b"invalid numbe ... s'\0":   l3861 = UNIQUE | NON_NULL, (empty)
                    *argv.offset(i as isize),
                    // 2802: *argv.offset(i  ... size): typeof(_516) = *mut {l837} i8
                    // 2802: *argv.offset(i  ... size):   l837 = UNIQUE | NON_NULL, (empty)
                    // 2802: argv.offset(i a ... size): typeof(_517) = *mut {l839} *mut {l840} i8
                    // 2802: argv.offset(i a ... size):   l839 = UNIQUE | NON_NULL, (empty)
                    // 2802: argv.offset(i a ... size):   l840 = UNIQUE | NON_NULL, (empty)
                    // 2802: argv: typeof(_518) = *mut {l842} *mut {l843} i8
                    // 2802: argv:   l842 = UNIQUE | NON_NULL, (empty)
                    // 2802: argv:   l843 = UNIQUE | NON_NULL, (empty)
                );
            }
        } else {
            name = *argv.offset(i as isize);
            // 2806: *argv.offset(i  ... size): typeof(_521) = *mut {l847} i8
            // 2806: *argv.offset(i  ... size):   l847 = UNIQUE | NON_NULL, (empty)
            // 2806: argv.offset(i a ... size): typeof(_522) = *mut {l849} *mut {l850} i8
            // 2806: argv.offset(i a ... size):   l849 = UNIQUE | NON_NULL, (empty)
            // 2806: argv.offset(i a ... size):   l850 = UNIQUE | NON_NULL, (empty)
            // 2806: argv: typeof(_523) = *mut {l852} *mut {l853} i8
            // 2806: argv:   l852 = UNIQUE | NON_NULL, (empty)
            // 2806: argv:   l853 = UNIQUE | NON_NULL, (empty)
            // 2806: name: typeof(_526) = *mut {l857} *const {l858} i8
            // 2806: name:   l857 = UNIQUE | NON_NULL, (empty)
            // 2806: name:   l858 = UNIQUE | NON_NULL, (empty)
            // 2806: name = *argv.of ... size): typeof((*_526) = move _521 as *const i8 (Pointer(MutToConstPointer))) = *const {l3865} i8
            // 2806: name = *argv.of ... size):   l3865 = UNIQUE | NON_NULL, (empty)
        }
        i += 1;
        i;
    }
    if nworkers2 != 0 {
    // 2811: nworkers2: typeof(_535) = *mut {l868} i32
    // 2811: nworkers2:   l868 = UNIQUE | NON_NULL, (empty)
        nworkers = nworkers2;
        // 2812: nworkers2: typeof(_537) = *mut {l871} i32
        // 2812: nworkers2:   l871 = UNIQUE | NON_NULL, (empty)
        // 2812: nworkers: typeof(_538) = *mut {l873} i32
        // 2812: nworkers:   l873 = UNIQUE | NON_NULL, (empty)
    }
    lglbnr(
        b"Plingeling Parallel SAT Solver\0" as *const u8 as *const libc::c_char,
        // 2815: b"Plingeling Pa ... _char: typeof(_540) = *const {l876} i8
        // 2815: b"Plingeling Pa ... _char:   l876 = UNIQUE | NON_NULL, (empty)
        // 2815: b"Plingeling Pa ... st u8: typeof(_541) = *const {l878} u8
        // 2815: b"Plingeling Pa ... st u8:   l878 = UNIQUE | NON_NULL, (empty)
        // 2815: b"Plingeling Pa ... er\0": typeof(_542) = *const {l880} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
        // 2815: b"Plingeling Pa ... er\0":   l880 = UNIQUE | NON_NULL, (empty)
        // 2815: b"Plingeling Pa ... er\0": typeof(_543) = & {l882} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
        // 2815: b"Plingeling Pa ... er\0":   l882 = UNIQUE | NON_NULL, FIXED
        // 2815: b"Plingeling Pa ... er\0": typeof(_542 = &raw const (*_543)) = *const {l3867} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
        // 2815: b"Plingeling Pa ... er\0":   l3867 = UNIQUE | NON_NULL, (empty)
        // 2815: b"Plingeling Pa ... _char: typeof(_540 = move _541 as *const i8 (Misc)) = *const {l3869} i8
        // 2815: b"Plingeling Pa ... _char:   l3869 = UNIQUE | NON_NULL, (empty)
        // 2815: b"Plingeling Pa ... st u8: typeof(_541 = move _542 as *const u8 (Pointer(ArrayToPointer))) = *const {l3868} u8
        // 2815: b"Plingeling Pa ... st u8:   l3868 = UNIQUE | NON_NULL, (empty)
        // 2815: b"Plingeling Pa ... er\0": typeof(_543 = const b"Plingeling Parallel SAT Solver\x00") = & {l3866} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
        // 2815: b"Plingeling Pa ... er\0":   l3866 = UNIQUE | NON_NULL, (empty)
        b"c \0" as *const u8 as *const libc::c_char,
        // 2816: b"c \0" as *con ... _char: typeof(_544) = *const {l884} i8
        // 2816: b"c \0" as *con ... _char:   l884 = UNIQUE | NON_NULL, (empty)
        // 2816: b"c \0" as *const u8: typeof(_545) = *const {l886} u8
        // 2816: b"c \0" as *const u8:   l886 = UNIQUE | NON_NULL, (empty)
        // 2816: b"c \0": typeof(_546) = *const {l888} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 2816: b"c \0":   l888 = UNIQUE | NON_NULL, (empty)
        // 2816: b"c \0": typeof(_547) = & {l890} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 2816: b"c \0":   l890 = UNIQUE | NON_NULL, FIXED
        // 2816: b"c \0": typeof(_547 = const b"c \x00") = & {l3870} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 2816: b"c \0":   l3870 = UNIQUE | NON_NULL, (empty)
        // 2816: b"c \0": typeof(_546 = &raw const (*_547)) = *const {l3871} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 2816: b"c \0":   l3871 = UNIQUE | NON_NULL, (empty)
        // 2816: b"c \0" as *con ... _char: typeof(_544 = move _545 as *const i8 (Misc)) = *const {l3873} i8
        // 2816: b"c \0" as *con ... _char:   l3873 = UNIQUE | NON_NULL, (empty)
        // 2816: b"c \0" as *const u8: typeof(_545 = move _546 as *const u8 (Pointer(ArrayToPointer))) = *const {l3872} u8
        // 2816: b"c \0" as *const u8:   l3872 = UNIQUE | NON_NULL, (empty)
        stdout,
        // 2817: stdout: typeof(_548) = *mut {l892} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 2817: stdout:   l892 = UNIQUE | NON_NULL, (empty)
        // 2817: stdout: typeof(_549) = *mut {l894} *mut {l895} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 2817: stdout:   l894 = UNIQUE | NON_NULL, (empty)
        // 2817: stdout:   l895 = UNIQUE | NON_NULL, (empty)
    );
    fflush(stdout);
    // 2819: stdout: typeof(_551) = *mut {l898} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 2819: stdout:   l898 = UNIQUE | NON_NULL, (empty)
    // 2819: stdout: typeof(_552) = *mut {l900} *mut {l901} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 2819: stdout:   l900 = UNIQUE | NON_NULL, (empty)
    // 2819: stdout:   l901 = UNIQUE | NON_NULL, (empty)
    if verbose != 0 {
    // 2820: verbose: typeof(_556) = *mut {l906} i32
    // 2820: verbose:   l906 = UNIQUE | NON_NULL, (empty)
        printf(b"c\n\0" as *const u8 as *const libc::c_char);
        // 2821: b"c\n\0" as *co ... _char: typeof(_558) = *const {l909} i8
        // 2821: b"c\n\0" as *co ... _char:   l909 = UNIQUE | NON_NULL, (empty)
        // 2821: b"c\n\0" as *co ... st u8: typeof(_559) = *const {l911} u8
        // 2821: b"c\n\0" as *co ... st u8:   l911 = UNIQUE | NON_NULL, (empty)
        // 2821: b"c\n\0": typeof(_560) = *const {l913} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 2821: b"c\n\0":   l913 = UNIQUE | NON_NULL, (empty)
        // 2821: b"c\n\0": typeof(_561) = & {l915} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 2821: b"c\n\0":   l915 = UNIQUE | NON_NULL, FIXED
        // 2821: b"c\n\0": typeof(_560 = &raw const (*_561)) = *const {l3875} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 2821: b"c\n\0":   l3875 = UNIQUE | NON_NULL, (empty)
        // 2821: b"c\n\0": typeof(_561 = const b"c\n\x00") = & {l3874} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 2821: b"c\n\0":   l3874 = UNIQUE | NON_NULL, (empty)
        // 2821: b"c\n\0" as *co ... st u8: typeof(_559 = move _560 as *const u8 (Pointer(ArrayToPointer))) = *const {l3876} u8
        // 2821: b"c\n\0" as *co ... st u8:   l3876 = UNIQUE | NON_NULL, (empty)
        // 2821: b"c\n\0" as *co ... _char: typeof(_558 = move _559 as *const i8 (Misc)) = *const {l3877} i8
        // 2821: b"c\n\0" as *co ... _char:   l3877 = UNIQUE | NON_NULL, (empty)
    }
    nbcore = parsenbcoreenv();
    if nworkers != 0 {
    // 2824: nworkers: typeof(_566) = *mut {l921} i32
    // 2824: nworkers:   l921 = UNIQUE | NON_NULL, (empty)
        msg(
            -(1 as libc::c_int),
            1 as libc::c_int,
            b"command line option '-t %d' overwrites system default %d\0" as *const u8
            // 2828: b"command line  ... _char: typeof(_572) = *const {l928} i8
            // 2828: b"command line  ... _char:   l928 = UNIQUE | NON_NULL, (empty)
            // 2828: b"command line  ... st u8: typeof(_573) = *const {l930} u8
            // 2828: b"command line  ... st u8:   l930 = UNIQUE | NON_NULL, (empty)
            // 2828: b"command line  ... %d\0": typeof(_574) = *const {l932} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
            // 2828: b"command line  ... %d\0":   l932 = UNIQUE | NON_NULL, (empty)
            // 2828: b"command line  ... %d\0": typeof(_575) = & {l934} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
            // 2828: b"command line  ... %d\0":   l934 = UNIQUE | NON_NULL, FIXED
            // 2828: b"command line  ... %d\0": typeof(_574 = &raw const (*_575)) = *const {l3879} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
            // 2828: b"command line  ... %d\0":   l3879 = UNIQUE | NON_NULL, (empty)
            // 2828: b"command line  ... st u8: typeof(_573 = move _574 as *const u8 (Pointer(ArrayToPointer))) = *const {l3880} u8
            // 2828: b"command line  ... st u8:   l3880 = UNIQUE | NON_NULL, (empty)
            // 2828: b"command line  ... _char: typeof(_572 = move _573 as *const i8 (Misc)) = *const {l3881} i8
            // 2828: b"command line  ... _char:   l3881 = UNIQUE | NON_NULL, (empty)
            // 2828: b"command line  ... %d\0": typeof(_575 = const b"command line option \'-t %d\' overwrites system default %d\x00") = & {l3878} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
            // 2828: b"command line  ... %d\0":   l3878 = UNIQUE | NON_NULL, (empty)
                as *const libc::c_char,
            nworkers,
            // 2830: nworkers: typeof(_577) = *mut {l937} i32
            // 2830: nworkers:   l937 = UNIQUE | NON_NULL, (empty)
            getsystemcores(0 as libc::c_int),
        );
        if nbcore != 0 {
            msg(
                -(1 as libc::c_int),
                1 as libc::c_int,
                b"and also overwrites environment variable NBCORE=%d\0" as *const u8
                // 2837: b"and also over ... _char: typeof(_587) = *const {l948} i8
                // 2837: b"and also over ... _char:   l948 = UNIQUE | NON_NULL, (empty)
                // 2837: b"and also over ... st u8: typeof(_588) = *const {l950} u8
                // 2837: b"and also over ... st u8:   l950 = UNIQUE | NON_NULL, (empty)
                // 2837: b"and also over ... %d\0": typeof(_589) = *const {l952} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
                // 2837: b"and also over ... %d\0":   l952 = UNIQUE | NON_NULL, (empty)
                // 2837: b"and also over ... %d\0": typeof(_590) = & {l954} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
                // 2837: b"and also over ... %d\0":   l954 = UNIQUE | NON_NULL, FIXED
                // 2837: b"and also over ... %d\0": typeof(_590 = const b"and also overwrites environment variable NBCORE=%d\x00") = & {l3882} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
                // 2837: b"and also over ... %d\0":   l3882 = UNIQUE | NON_NULL, (empty)
                // 2837: b"and also over ... _char: typeof(_587 = move _588 as *const i8 (Misc)) = *const {l3885} i8
                // 2837: b"and also over ... _char:   l3885 = UNIQUE | NON_NULL, (empty)
                // 2837: b"and also over ... %d\0": typeof(_589 = &raw const (*_590)) = *const {l3883} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
                // 2837: b"and also over ... %d\0":   l3883 = UNIQUE | NON_NULL, (empty)
                // 2837: b"and also over ... st u8: typeof(_588 = move _589 as *const u8 (Pointer(ArrayToPointer))) = *const {l3884} u8
                // 2837: b"and also over ... st u8:   l3884 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
                nbcore,
            );
        }
    } else if nbcore != 0 {
        msg(
            -(1 as libc::c_int),
            1 as libc::c_int,
            b"environment variable NBCORE=%d overwrites system default %d\0" as *const u8
            // 2846: b"environment v ... _char: typeof(_599) = *const {l964} i8
            // 2846: b"environment v ... _char:   l964 = UNIQUE | NON_NULL, (empty)
            // 2846: b"environment v ... st u8: typeof(_600) = *const {l966} u8
            // 2846: b"environment v ... st u8:   l966 = UNIQUE | NON_NULL, (empty)
            // 2846: b"environment v ... %d\0": typeof(_601) = *const {l968} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003c)) }]
            // 2846: b"environment v ... %d\0":   l968 = UNIQUE | NON_NULL, (empty)
            // 2846: b"environment v ... %d\0": typeof(_602) = & {l970} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003c)) }]
            // 2846: b"environment v ... %d\0":   l970 = UNIQUE | NON_NULL, FIXED
            // 2846: b"environment v ... st u8: typeof(_600 = move _601 as *const u8 (Pointer(ArrayToPointer))) = *const {l3888} u8
            // 2846: b"environment v ... st u8:   l3888 = UNIQUE | NON_NULL, (empty)
            // 2846: b"environment v ... %d\0": typeof(_602 = const b"environment variable NBCORE=%d overwrites system default %d\x00") = & {l3886} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003c)) }]
            // 2846: b"environment v ... %d\0":   l3886 = UNIQUE | NON_NULL, (empty)
            // 2846: b"environment v ... %d\0": typeof(_601 = &raw const (*_602)) = *const {l3887} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003c)) }]
            // 2846: b"environment v ... %d\0":   l3887 = UNIQUE | NON_NULL, (empty)
            // 2846: b"environment v ... _char: typeof(_599 = move _600 as *const i8 (Misc)) = *const {l3889} i8
            // 2846: b"environment v ... _char:   l3889 = UNIQUE | NON_NULL, (empty)
                as *const libc::c_char,
            nbcore,
            getsystemcores(0 as libc::c_int),
        );
        nworkers = nbcore;
        // 2851: nworkers: typeof(_607) = *mut {l976} i32
        // 2851: nworkers:   l976 = UNIQUE | NON_NULL, (empty)
    } else {
        msg(
            -(1 as libc::c_int),
            1 as libc::c_int,
            b"no explicit specification of number of workers\0" as *const u8 as *const libc::c_char,
            // 2856: b"no explicit s ... _char: typeof(_613) = *const {l983} i8
            // 2856: b"no explicit s ... _char:   l983 = UNIQUE | NON_NULL, (empty)
            // 2856: b"no explicit s ... st u8: typeof(_614) = *const {l985} u8
            // 2856: b"no explicit s ... st u8:   l985 = UNIQUE | NON_NULL, (empty)
            // 2856: b"no explicit s ... rs\0": typeof(_615) = *const {l987} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
            // 2856: b"no explicit s ... rs\0":   l987 = UNIQUE | NON_NULL, (empty)
            // 2856: b"no explicit s ... rs\0": typeof(_616) = & {l989} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
            // 2856: b"no explicit s ... rs\0":   l989 = UNIQUE | NON_NULL, FIXED
            // 2856: b"no explicit s ... rs\0": typeof(_616 = const b"no explicit specification of number of workers\x00") = & {l3890} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
            // 2856: b"no explicit s ... rs\0":   l3890 = UNIQUE | NON_NULL, (empty)
            // 2856: b"no explicit s ... st u8: typeof(_614 = move _615 as *const u8 (Pointer(ArrayToPointer))) = *const {l3892} u8
            // 2856: b"no explicit s ... st u8:   l3892 = UNIQUE | NON_NULL, (empty)
            // 2856: b"no explicit s ... rs\0": typeof(_615 = &raw const (*_616)) = *const {l3891} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
            // 2856: b"no explicit s ... rs\0":   l3891 = UNIQUE | NON_NULL, (empty)
            // 2856: b"no explicit s ... _char: typeof(_613 = move _614 as *const i8 (Misc)) = *const {l3893} i8
            // 2856: b"no explicit s ... _char:   l3893 = UNIQUE | NON_NULL, (empty)
        );
        nworkers = getsystemcores(1 as libc::c_int);
        // 2858: nworkers: typeof(_619) = *mut {l993} i32
        // 2858: nworkers:   l993 = UNIQUE | NON_NULL, (empty)
    }
    msg(
        -(1 as libc::c_int),
        0 as libc::c_int,
        b"USING %d WORKER THREADS\0" as *const u8 as *const libc::c_char,
        // 2863: b"USING %d WORK ... _char: typeof(_625) = *const {l1000} i8
        // 2863: b"USING %d WORK ... _char:   l1000 = UNIQUE | NON_NULL, (empty)
        // 2863: b"USING %d WORK ... st u8: typeof(_626) = *const {l1002} u8
        // 2863: b"USING %d WORK ... st u8:   l1002 = UNIQUE | NON_NULL, (empty)
        // 2863: b"USING %d WORK ... DS\0": typeof(_627) = *const {l1004} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 2863: b"USING %d WORK ... DS\0":   l1004 = UNIQUE | NON_NULL, (empty)
        // 2863: b"USING %d WORK ... DS\0": typeof(_628) = & {l1006} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 2863: b"USING %d WORK ... DS\0":   l1006 = UNIQUE | NON_NULL, FIXED
        // 2863: b"USING %d WORK ... DS\0": typeof(_628 = const b"USING %d WORKER THREADS\x00") = & {l3894} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 2863: b"USING %d WORK ... DS\0":   l3894 = UNIQUE | NON_NULL, (empty)
        // 2863: b"USING %d WORK ... _char: typeof(_625 = move _626 as *const i8 (Misc)) = *const {l3897} i8
        // 2863: b"USING %d WORK ... _char:   l3897 = UNIQUE | NON_NULL, (empty)
        // 2863: b"USING %d WORK ... DS\0": typeof(_627 = &raw const (*_628)) = *const {l3895} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 2863: b"USING %d WORK ... DS\0":   l3895 = UNIQUE | NON_NULL, (empty)
        // 2863: b"USING %d WORK ... st u8: typeof(_626 = move _627 as *const u8 (Pointer(ArrayToPointer))) = *const {l3896} u8
        // 2863: b"USING %d WORK ... st u8:   l3896 = UNIQUE | NON_NULL, (empty)
        nworkers,
        // 2864: nworkers: typeof(_630) = *mut {l1009} i32
        // 2864: nworkers:   l1009 = UNIQUE | NON_NULL, (empty)
    );
    if memlimit != 0 {
    // 2866: memlimit: typeof(_634) = *mut {l1014} i64
    // 2866: memlimit:   l1014 = UNIQUE | NON_NULL, (empty)
        msg(
            -(1 as libc::c_int),
            1 as libc::c_int,
            b"memory limit %lld MB ('-g %lld') overwrites system default %lld MB\0" as *const u8
            // 2870: b"memory limit  ... _char: typeof(_640) = *const {l1021} i8
            // 2870: b"memory limit  ... _char:   l1021 = UNIQUE | NON_NULL, (empty)
            // 2870: b"memory limit  ... st u8: typeof(_641) = *const {l1023} u8
            // 2870: b"memory limit  ... st u8:   l1023 = UNIQUE | NON_NULL, (empty)
            // 2870: b"memory limit  ... MB\0": typeof(_642) = *const {l1025} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000043)) }]
            // 2870: b"memory limit  ... MB\0":   l1025 = UNIQUE | NON_NULL, (empty)
            // 2870: b"memory limit  ... MB\0": typeof(_643) = & {l1027} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000043)) }]
            // 2870: b"memory limit  ... MB\0":   l1027 = UNIQUE | NON_NULL, FIXED
            // 2870: b"memory limit  ... st u8: typeof(_641 = move _642 as *const u8 (Pointer(ArrayToPointer))) = *const {l3900} u8
            // 2870: b"memory limit  ... st u8:   l3900 = UNIQUE | NON_NULL, (empty)
            // 2870: b"memory limit  ... MB\0": typeof(_642 = &raw const (*_643)) = *const {l3899} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000043)) }]
            // 2870: b"memory limit  ... MB\0":   l3899 = UNIQUE | NON_NULL, (empty)
            // 2870: b"memory limit  ... MB\0": typeof(_643 = const b"memory limit %lld MB (\'-g %lld\') overwrites system default %lld MB\x00") = & {l3898} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000043)) }]
            // 2870: b"memory limit  ... MB\0":   l3898 = UNIQUE | NON_NULL, (empty)
            // 2870: b"memory limit  ... _char: typeof(_640 = move _641 as *const i8 (Misc)) = *const {l3901} i8
            // 2870: b"memory limit  ... _char:   l3901 = UNIQUE | NON_NULL, (empty)
                as *const libc::c_char,
            bytes2mbll(memlimit),
            // 2872: memlimit: typeof(_646) = *mut {l1031} i64
            // 2872: memlimit:   l1031 = UNIQUE | NON_NULL, (empty)
            bytes2gbll(memlimit),
            // 2873: memlimit: typeof(_649) = *mut {l1035} i64
            // 2873: memlimit:   l1035 = UNIQUE | NON_NULL, (empty)
            bytes2mbll(getsystemtotalmem(0 as libc::c_int)),
        );
    } else {
        memlimit = getsystemtotalmem(1 as libc::c_int);
        // 2877: memlimit: typeof(_655) = *mut {l1042} i64
        // 2877: memlimit:   l1042 = UNIQUE | NON_NULL, (empty)
        msg(
            -(1 as libc::c_int),
            1 as libc::c_int,
            b"memory limit set to system default of %lld MB total memory\0" as *const u8
            // 2881: b"memory limit  ... _char: typeof(_661) = *const {l1049} i8
            // 2881: b"memory limit  ... _char:   l1049 = UNIQUE | NON_NULL, (empty)
            // 2881: b"memory limit  ... st u8: typeof(_662) = *const {l1051} u8
            // 2881: b"memory limit  ... st u8:   l1051 = UNIQUE | NON_NULL, (empty)
            // 2881: b"memory limit  ... ry\0": typeof(_663) = *const {l1053} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003b)) }]
            // 2881: b"memory limit  ... ry\0":   l1053 = UNIQUE | NON_NULL, (empty)
            // 2881: b"memory limit  ... ry\0": typeof(_664) = & {l1055} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003b)) }]
            // 2881: b"memory limit  ... ry\0":   l1055 = UNIQUE | NON_NULL, FIXED
            // 2881: b"memory limit  ... st u8: typeof(_662 = move _663 as *const u8 (Pointer(ArrayToPointer))) = *const {l3904} u8
            // 2881: b"memory limit  ... st u8:   l3904 = UNIQUE | NON_NULL, (empty)
            // 2881: b"memory limit  ... ry\0": typeof(_663 = &raw const (*_664)) = *const {l3903} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003b)) }]
            // 2881: b"memory limit  ... ry\0":   l3903 = UNIQUE | NON_NULL, (empty)
            // 2881: b"memory limit  ... ry\0": typeof(_664 = const b"memory limit set to system default of %lld MB total memory\x00") = & {l3902} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003b)) }]
            // 2881: b"memory limit  ... ry\0":   l3902 = UNIQUE | NON_NULL, (empty)
            // 2881: b"memory limit  ... _char: typeof(_661 = move _662 as *const i8 (Misc)) = *const {l3905} i8
            // 2881: b"memory limit  ... _char:   l3905 = UNIQUE | NON_NULL, (empty)
                as *const libc::c_char,
            bytes2mbll(memlimit),
            // 2883: memlimit: typeof(_667) = *mut {l1059} i64
            // 2883: memlimit:   l1059 = UNIQUE | NON_NULL, (empty)
        );
    }
    softmemlimit = (memlimit + 2 as libc::c_int as int64_t) / 3 as libc::c_int as int64_t;
    // 2886: memlimit: typeof(_670) = *mut {l1063} i64
    // 2886: memlimit:   l1063 = UNIQUE | NON_NULL, (empty)
    // 2886: softmemlimit: typeof(_680) = *mut {l1074} i64
    // 2886: softmemlimit:   l1074 = UNIQUE | NON_NULL, (empty)
    msg(
        -(1 as libc::c_int),
        0 as libc::c_int,
        b"soft memory limit set to %lld MB\0" as *const u8 as *const libc::c_char,
        // 2890: b"soft memory l ... _char: typeof(_686) = *const {l1081} i8
        // 2890: b"soft memory l ... _char:   l1081 = UNIQUE | NON_NULL, (empty)
        // 2890: b"soft memory l ... st u8: typeof(_687) = *const {l1083} u8
        // 2890: b"soft memory l ... st u8:   l1083 = UNIQUE | NON_NULL, (empty)
        // 2890: b"soft memory l ... MB\0": typeof(_688) = *const {l1085} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
        // 2890: b"soft memory l ... MB\0":   l1085 = UNIQUE | NON_NULL, (empty)
        // 2890: b"soft memory l ... MB\0": typeof(_689) = & {l1087} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
        // 2890: b"soft memory l ... MB\0":   l1087 = UNIQUE | NON_NULL, FIXED
        // 2890: b"soft memory l ... st u8: typeof(_687 = move _688 as *const u8 (Pointer(ArrayToPointer))) = *const {l3908} u8
        // 2890: b"soft memory l ... st u8:   l3908 = UNIQUE | NON_NULL, (empty)
        // 2890: b"soft memory l ... MB\0": typeof(_689 = const b"soft memory limit set to %lld MB\x00") = & {l3906} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
        // 2890: b"soft memory l ... MB\0":   l3906 = UNIQUE | NON_NULL, (empty)
        // 2890: b"soft memory l ... MB\0": typeof(_688 = &raw const (*_689)) = *const {l3907} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
        // 2890: b"soft memory l ... MB\0":   l3907 = UNIQUE | NON_NULL, (empty)
        // 2890: b"soft memory l ... _char: typeof(_686 = move _687 as *const i8 (Misc)) = *const {l3909} i8
        // 2890: b"soft memory l ... _char:   l3909 = UNIQUE | NON_NULL, (empty)
        bytes2mbll(softmemlimit),
        // 2891: softmemlimit: typeof(_692) = *mut {l1091} i64
        // 2891: softmemlimit:   l1091 = UNIQUE | NON_NULL, (empty)
    );
    if gclimset != 0 {
    // 2893: gclimset: typeof(_696) = *mut {l1096} i32
    // 2893: gclimset:   l1096 = UNIQUE | NON_NULL, (empty)
        msg(
            -(1 as libc::c_int),
            0 as libc::c_int,
            b"garbage collection limit %d ('-l %d')\0" as *const u8 as *const libc::c_char,
            // 2897: b"garbage colle ... _char: typeof(_702) = *const {l1103} i8
            // 2897: b"garbage colle ... _char:   l1103 = UNIQUE | NON_NULL, (empty)
            // 2897: b"garbage colle ... st u8: typeof(_703) = *const {l1105} u8
            // 2897: b"garbage colle ... st u8:   l1105 = UNIQUE | NON_NULL, (empty)
            // 2897: b"garbage colle ... ')\0": typeof(_704) = *const {l1107} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
            // 2897: b"garbage colle ... ')\0":   l1107 = UNIQUE | NON_NULL, (empty)
            // 2897: b"garbage colle ... ')\0": typeof(_705) = & {l1109} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
            // 2897: b"garbage colle ... ')\0":   l1109 = UNIQUE | NON_NULL, FIXED
            // 2897: b"garbage colle ... ')\0": typeof(_704 = &raw const (*_705)) = *const {l3911} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
            // 2897: b"garbage colle ... ')\0":   l3911 = UNIQUE | NON_NULL, (empty)
            // 2897: b"garbage colle ... _char: typeof(_702 = move _703 as *const i8 (Misc)) = *const {l3913} i8
            // 2897: b"garbage colle ... _char:   l3913 = UNIQUE | NON_NULL, (empty)
            // 2897: b"garbage colle ... st u8: typeof(_703 = move _704 as *const u8 (Pointer(ArrayToPointer))) = *const {l3912} u8
            // 2897: b"garbage colle ... st u8:   l3912 = UNIQUE | NON_NULL, (empty)
            // 2897: b"garbage colle ... ')\0": typeof(_705 = const b"garbage collection limit %d (\'-l %d\')\x00") = & {l3910} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
            // 2897: b"garbage colle ... ')\0":   l3910 = UNIQUE | NON_NULL, (empty)
            gclim,
            // 2898: gclim: typeof(_707) = *mut {l1112} i32
            // 2898: gclim:   l1112 = UNIQUE | NON_NULL, (empty)
            gclim,
            // 2899: gclim: typeof(_709) = *mut {l1115} i32
            // 2899: gclim:   l1115 = UNIQUE | NON_NULL, (empty)
        );
    } else {
        msg(
            -(1 as libc::c_int),
            0 as libc::c_int,
            b"default garbage collection limit %d\0" as *const u8 as *const libc::c_char,
            // 2905: b"default garba ... _char: typeof(_715) = *const {l1122} i8
            // 2905: b"default garba ... _char:   l1122 = UNIQUE | NON_NULL, (empty)
            // 2905: b"default garba ... st u8: typeof(_716) = *const {l1124} u8
            // 2905: b"default garba ... st u8:   l1124 = UNIQUE | NON_NULL, (empty)
            // 2905: b"default garba ... %d\0": typeof(_717) = *const {l1126} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
            // 2905: b"default garba ... %d\0":   l1126 = UNIQUE | NON_NULL, (empty)
            // 2905: b"default garba ... %d\0": typeof(_718) = & {l1128} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
            // 2905: b"default garba ... %d\0":   l1128 = UNIQUE | NON_NULL, FIXED
            // 2905: b"default garba ... st u8: typeof(_716 = move _717 as *const u8 (Pointer(ArrayToPointer))) = *const {l3916} u8
            // 2905: b"default garba ... st u8:   l3916 = UNIQUE | NON_NULL, (empty)
            // 2905: b"default garba ... %d\0": typeof(_717 = &raw const (*_718)) = *const {l3915} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
            // 2905: b"default garba ... %d\0":   l3915 = UNIQUE | NON_NULL, (empty)
            // 2905: b"default garba ... _char: typeof(_715 = move _716 as *const i8 (Misc)) = *const {l3917} i8
            // 2905: b"default garba ... _char:   l3917 = UNIQUE | NON_NULL, (empty)
            // 2905: b"default garba ... %d\0": typeof(_718 = const b"default garbage collection limit %d\x00") = & {l3914} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
            // 2905: b"default garba ... %d\0":   l3914 = UNIQUE | NON_NULL, (empty)
            gclim,
            // 2906: gclim: typeof(_720) = *mut {l1131} i32
            // 2906: gclim:   l1131 = UNIQUE | NON_NULL, (empty)
        );
    }
    if plain != 0 {
    // 2909: plain: typeof(_724) = *mut {l1136} i32
    // 2909: plain:   l1136 = UNIQUE | NON_NULL, (empty)
        noeqs = 1 as libc::c_int;
        // 2910: noeqs: typeof(_726) = *mut {l1139} i32
        // 2910: noeqs:   l1139 = UNIQUE | NON_NULL, (empty)
        nocls = noeqs;
        // 2911: noeqs: typeof(_728) = *mut {l1142} i32
        // 2911: noeqs:   l1142 = UNIQUE | NON_NULL, (empty)
        // 2911: nocls: typeof(_729) = *mut {l1144} i32
        // 2911: nocls:   l1144 = UNIQUE | NON_NULL, (empty)
        nounits = nocls;
        // 2912: nocls: typeof(_731) = *mut {l1147} i32
        // 2912: nocls:   l1147 = UNIQUE | NON_NULL, (empty)
        // 2912: nounits: typeof(_732) = *mut {l1149} i32
        // 2912: nounits:   l1149 = UNIQUE | NON_NULL, (empty)
        msg(
            -(1 as libc::c_int),
            0 as libc::c_int,
            b"not sharing anything in plain portolio mode ('-p')\0" as *const u8
            // 2916: b"not sharing a ... _char: typeof(_738) = *const {l1156} i8
            // 2916: b"not sharing a ... _char:   l1156 = UNIQUE | NON_NULL, (empty)
            // 2916: b"not sharing a ... st u8: typeof(_739) = *const {l1158} u8
            // 2916: b"not sharing a ... st u8:   l1158 = UNIQUE | NON_NULL, (empty)
            // 2916: b"not sharing a ... ')\0": typeof(_740) = *const {l1160} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 2916: b"not sharing a ... ')\0":   l1160 = UNIQUE | NON_NULL, (empty)
            // 2916: b"not sharing a ... ')\0": typeof(_741) = & {l1162} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 2916: b"not sharing a ... ')\0":   l1162 = UNIQUE | NON_NULL, FIXED
            // 2916: b"not sharing a ... ')\0": typeof(_740 = &raw const (*_741)) = *const {l3919} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 2916: b"not sharing a ... ')\0":   l3919 = UNIQUE | NON_NULL, (empty)
            // 2916: b"not sharing a ... ')\0": typeof(_741 = const b"not sharing anything in plain portolio mode (\'-p\')\x00") = & {l3918} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 2916: b"not sharing a ... ')\0":   l3918 = UNIQUE | NON_NULL, (empty)
            // 2916: b"not sharing a ... st u8: typeof(_739 = move _740 as *const u8 (Pointer(ArrayToPointer))) = *const {l3920} u8
            // 2916: b"not sharing a ... st u8:   l3920 = UNIQUE | NON_NULL, (empty)
            // 2916: b"not sharing a ... _char: typeof(_738 = move _739 as *const i8 (Misc)) = *const {l3921} i8
            // 2916: b"not sharing a ... _char:   l3921 = UNIQUE | NON_NULL, (empty)
                as *const libc::c_char,
        );
    } else {
        if noeqs != 0 {
        // 2920: noeqs: typeof(_745) = *mut {l1167} i32
        // 2920: noeqs:   l1167 = UNIQUE | NON_NULL, (empty)
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"not sharing units ('--do-not-share-units')\0" as *const u8 as *const libc::c_char,
                // 2924: b"not sharing u ... _char: typeof(_751) = *const {l1174} i8
                // 2924: b"not sharing u ... _char:   l1174 = UNIQUE | NON_NULL, (empty)
                // 2924: b"not sharing u ... st u8: typeof(_752) = *const {l1176} u8
                // 2924: b"not sharing u ... st u8:   l1176 = UNIQUE | NON_NULL, (empty)
                // 2924: b"not sharing u ... ')\0": typeof(_753) = *const {l1178} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                // 2924: b"not sharing u ... ')\0":   l1178 = UNIQUE | NON_NULL, (empty)
                // 2924: b"not sharing u ... ')\0": typeof(_754) = & {l1180} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                // 2924: b"not sharing u ... ')\0":   l1180 = UNIQUE | NON_NULL, FIXED
                // 2924: b"not sharing u ... _char: typeof(_751 = move _752 as *const i8 (Misc)) = *const {l3925} i8
                // 2924: b"not sharing u ... _char:   l3925 = UNIQUE | NON_NULL, (empty)
                // 2924: b"not sharing u ... ')\0": typeof(_754 = const b"not sharing units (\'--do-not-share-units\')\x00") = & {l3922} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                // 2924: b"not sharing u ... ')\0":   l3922 = UNIQUE | NON_NULL, (empty)
                // 2924: b"not sharing u ... st u8: typeof(_752 = move _753 as *const u8 (Pointer(ArrayToPointer))) = *const {l3924} u8
                // 2924: b"not sharing u ... st u8:   l3924 = UNIQUE | NON_NULL, (empty)
                // 2924: b"not sharing u ... ')\0": typeof(_753 = &raw const (*_754)) = *const {l3923} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                // 2924: b"not sharing u ... ')\0":   l3923 = UNIQUE | NON_NULL, (empty)
            );
        } else {
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"sharing of units enabled\0" as *const u8 as *const libc::c_char,
                // 2930: b"sharing of un ... _char: typeof(_760) = *const {l1187} i8
                // 2930: b"sharing of un ... _char:   l1187 = UNIQUE | NON_NULL, (empty)
                // 2930: b"sharing of un ... st u8: typeof(_761) = *const {l1189} u8
                // 2930: b"sharing of un ... st u8:   l1189 = UNIQUE | NON_NULL, (empty)
                // 2930: b"sharing of un ... ed\0": typeof(_762) = *const {l1191} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2930: b"sharing of un ... ed\0":   l1191 = UNIQUE | NON_NULL, (empty)
                // 2930: b"sharing of un ... ed\0": typeof(_763) = & {l1193} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2930: b"sharing of un ... ed\0":   l1193 = UNIQUE | NON_NULL, FIXED
                // 2930: b"sharing of un ... ed\0": typeof(_762 = &raw const (*_763)) = *const {l3927} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2930: b"sharing of un ... ed\0":   l3927 = UNIQUE | NON_NULL, (empty)
                // 2930: b"sharing of un ... ed\0": typeof(_763 = const b"sharing of units enabled\x00") = & {l3926} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2930: b"sharing of un ... ed\0":   l3926 = UNIQUE | NON_NULL, (empty)
                // 2930: b"sharing of un ... st u8: typeof(_761 = move _762 as *const u8 (Pointer(ArrayToPointer))) = *const {l3928} u8
                // 2930: b"sharing of un ... st u8:   l3928 = UNIQUE | NON_NULL, (empty)
                // 2930: b"sharing of un ... _char: typeof(_760 = move _761 as *const i8 (Misc)) = *const {l3929} i8
                // 2930: b"sharing of un ... _char:   l3929 = UNIQUE | NON_NULL, (empty)
            );
        }
        if nocls != 0 {
        // 2933: nocls: typeof(_767) = *mut {l1198} i32
        // 2933: nocls:   l1198 = UNIQUE | NON_NULL, (empty)
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"not sharing clauses ('--do-not-share-clauses')\0" as *const u8
                // 2937: b"not sharing c ... _char: typeof(_773) = *const {l1205} i8
                // 2937: b"not sharing c ... _char:   l1205 = UNIQUE | NON_NULL, (empty)
                // 2937: b"not sharing c ... st u8: typeof(_774) = *const {l1207} u8
                // 2937: b"not sharing c ... st u8:   l1207 = UNIQUE | NON_NULL, (empty)
                // 2937: b"not sharing c ... ')\0": typeof(_775) = *const {l1209} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                // 2937: b"not sharing c ... ')\0":   l1209 = UNIQUE | NON_NULL, (empty)
                // 2937: b"not sharing c ... ')\0": typeof(_776) = & {l1211} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                // 2937: b"not sharing c ... ')\0":   l1211 = UNIQUE | NON_NULL, FIXED
                // 2937: b"not sharing c ... st u8: typeof(_774 = move _775 as *const u8 (Pointer(ArrayToPointer))) = *const {l3932} u8
                // 2937: b"not sharing c ... st u8:   l3932 = UNIQUE | NON_NULL, (empty)
                // 2937: b"not sharing c ... ')\0": typeof(_776 = const b"not sharing clauses (\'--do-not-share-clauses\')\x00") = & {l3930} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                // 2937: b"not sharing c ... ')\0":   l3930 = UNIQUE | NON_NULL, (empty)
                // 2937: b"not sharing c ... _char: typeof(_773 = move _774 as *const i8 (Misc)) = *const {l3933} i8
                // 2937: b"not sharing c ... _char:   l3933 = UNIQUE | NON_NULL, (empty)
                // 2937: b"not sharing c ... ')\0": typeof(_775 = &raw const (*_776)) = *const {l3931} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                // 2937: b"not sharing c ... ')\0":   l3931 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
        } else {
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"sharing of clauses enabled\0" as *const u8 as *const libc::c_char,
                // 2944: b"sharing of cl ... _char: typeof(_782) = *const {l1218} i8
                // 2944: b"sharing of cl ... _char:   l1218 = UNIQUE | NON_NULL, (empty)
                // 2944: b"sharing of cl ... st u8: typeof(_783) = *const {l1220} u8
                // 2944: b"sharing of cl ... st u8:   l1220 = UNIQUE | NON_NULL, (empty)
                // 2944: b"sharing of cl ... ed\0": typeof(_784) = *const {l1222} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                // 2944: b"sharing of cl ... ed\0":   l1222 = UNIQUE | NON_NULL, (empty)
                // 2944: b"sharing of cl ... ed\0": typeof(_785) = & {l1224} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                // 2944: b"sharing of cl ... ed\0":   l1224 = UNIQUE | NON_NULL, FIXED
                // 2944: b"sharing of cl ... ed\0": typeof(_785 = const b"sharing of clauses enabled\x00") = & {l3934} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                // 2944: b"sharing of cl ... ed\0":   l3934 = UNIQUE | NON_NULL, (empty)
                // 2944: b"sharing of cl ... ed\0": typeof(_784 = &raw const (*_785)) = *const {l3935} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                // 2944: b"sharing of cl ... ed\0":   l3935 = UNIQUE | NON_NULL, (empty)
                // 2944: b"sharing of cl ... _char: typeof(_782 = move _783 as *const i8 (Misc)) = *const {l3937} i8
                // 2944: b"sharing of cl ... _char:   l3937 = UNIQUE | NON_NULL, (empty)
                // 2944: b"sharing of cl ... st u8: typeof(_783 = move _784 as *const u8 (Pointer(ArrayToPointer))) = *const {l3936} u8
                // 2944: b"sharing of cl ... st u8:   l3936 = UNIQUE | NON_NULL, (empty)
            );
        }
        if noeqs != 0 {
        // 2947: noeqs: typeof(_788) = *mut {l1228} i32
        // 2947: noeqs:   l1228 = UNIQUE | NON_NULL, (empty)
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"not sharing equivalences ('--do-not-share-equivalences')\0" as *const u8
                // 2951: b"not sharing e ... _char: typeof(_794) = *const {l1235} i8
                // 2951: b"not sharing e ... _char:   l1235 = UNIQUE | NON_NULL, (empty)
                // 2951: b"not sharing e ... st u8: typeof(_795) = *const {l1237} u8
                // 2951: b"not sharing e ... st u8:   l1237 = UNIQUE | NON_NULL, (empty)
                // 2951: b"not sharing e ... ')\0": typeof(_796) = *const {l1239} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                // 2951: b"not sharing e ... ')\0":   l1239 = UNIQUE | NON_NULL, (empty)
                // 2951: b"not sharing e ... ')\0": typeof(_797) = & {l1241} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                // 2951: b"not sharing e ... ')\0":   l1241 = UNIQUE | NON_NULL, FIXED
                // 2951: b"not sharing e ... st u8: typeof(_795 = move _796 as *const u8 (Pointer(ArrayToPointer))) = *const {l3940} u8
                // 2951: b"not sharing e ... st u8:   l3940 = UNIQUE | NON_NULL, (empty)
                // 2951: b"not sharing e ... ')\0": typeof(_796 = &raw const (*_797)) = *const {l3939} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                // 2951: b"not sharing e ... ')\0":   l3939 = UNIQUE | NON_NULL, (empty)
                // 2951: b"not sharing e ... _char: typeof(_794 = move _795 as *const i8 (Misc)) = *const {l3941} i8
                // 2951: b"not sharing e ... _char:   l3941 = UNIQUE | NON_NULL, (empty)
                // 2951: b"not sharing e ... ')\0": typeof(_797 = const b"not sharing equivalences (\'--do-not-share-equivalences\')\x00") = & {l3938} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                // 2951: b"not sharing e ... ')\0":   l3938 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
        } else {
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"sharing of equivalences enabled\0" as *const u8 as *const libc::c_char,
                // 2958: b"sharing of eq ... _char: typeof(_803) = *const {l1248} i8
                // 2958: b"sharing of eq ... _char:   l1248 = UNIQUE | NON_NULL, (empty)
                // 2958: b"sharing of eq ... st u8: typeof(_804) = *const {l1250} u8
                // 2958: b"sharing of eq ... st u8:   l1250 = UNIQUE | NON_NULL, (empty)
                // 2958: b"sharing of eq ... ed\0": typeof(_805) = *const {l1252} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 2958: b"sharing of eq ... ed\0":   l1252 = UNIQUE | NON_NULL, (empty)
                // 2958: b"sharing of eq ... ed\0": typeof(_806) = & {l1254} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 2958: b"sharing of eq ... ed\0":   l1254 = UNIQUE | NON_NULL, FIXED
                // 2958: b"sharing of eq ... ed\0": typeof(_805 = &raw const (*_806)) = *const {l3943} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 2958: b"sharing of eq ... ed\0":   l3943 = UNIQUE | NON_NULL, (empty)
                // 2958: b"sharing of eq ... st u8: typeof(_804 = move _805 as *const u8 (Pointer(ArrayToPointer))) = *const {l3944} u8
                // 2958: b"sharing of eq ... st u8:   l3944 = UNIQUE | NON_NULL, (empty)
                // 2958: b"sharing of eq ... _char: typeof(_803 = move _804 as *const i8 (Misc)) = *const {l3945} i8
                // 2958: b"sharing of eq ... _char:   l3945 = UNIQUE | NON_NULL, (empty)
                // 2958: b"sharing of eq ... ed\0": typeof(_806 = const b"sharing of equivalences enabled\x00") = & {l3942} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 2958: b"sharing of eq ... ed\0":   l3942 = UNIQUE | NON_NULL, (empty)
            );
        }
    }
    let mut BYTES: size_t =
        (nworkers as libc::c_ulong).wrapping_mul(::core::mem::size_of::<Worker>() as libc::c_ulong);
        // 2963: nworkers: typeof(_810) = *mut {l1259} i32
        // 2963: nworkers:   l1259 = UNIQUE | NON_NULL, (empty)
    workers = malloc(BYTES) as *mut Worker;
    // 2964: malloc(BYTES): typeof(_813) = *mut {l1263} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2964: malloc(BYTES):   l1263 = UNIQUE | NON_NULL, (empty)
    // 2964: workers: typeof(_815) = *mut {l1266} *mut {l1267} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2964: workers:   l1266 = UNIQUE | NON_NULL, (empty)
    // 2964: workers:   l1267 = UNIQUE | NON_NULL, (empty)
    // 2964: workers = mallo ... orker: typeof((*_815) = move _813 as *mut Worker (Misc)) = *mut {l3946} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2964: workers = mallo ... orker:   l3946 = UNIQUE | NON_NULL, (empty)
    if workers.is_null() {
    // 2965: workers: typeof(_818) = *mut {l1271} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2965: workers:   l1271 = UNIQUE | NON_NULL, (empty)
    // 2965: workers: typeof(_819) = *mut {l1273} *mut {l1274} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2965: workers:   l1273 = UNIQUE | NON_NULL, (empty)
    // 2965: workers:   l1274 = UNIQUE | NON_NULL, (empty)
        die(b"out of memory\0" as *const u8 as *const libc::c_char);
        // 2966: b"out of memory ... _char: typeof(_822) = *const {l1278} i8
        // 2966: b"out of memory ... _char:   l1278 = UNIQUE | NON_NULL, (empty)
        // 2966: b"out of memory ... st u8: typeof(_823) = *const {l1280} u8
        // 2966: b"out of memory ... st u8:   l1280 = UNIQUE | NON_NULL, (empty)
        // 2966: b"out of memory\0": typeof(_824) = *const {l1282} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 2966: b"out of memory\0":   l1282 = UNIQUE | NON_NULL, (empty)
        // 2966: b"out of memory\0": typeof(_825) = & {l1284} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 2966: b"out of memory\0":   l1284 = UNIQUE | NON_NULL, FIXED
        // 2966: b"out of memory ... st u8: typeof(_823 = move _824 as *const u8 (Pointer(ArrayToPointer))) = *const {l3949} u8
        // 2966: b"out of memory ... st u8:   l3949 = UNIQUE | NON_NULL, (empty)
        // 2966: b"out of memory\0": typeof(_825 = const b"out of memory\x00") = & {l3947} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 2966: b"out of memory\0":   l3947 = UNIQUE | NON_NULL, (empty)
        // 2966: b"out of memory ... _char: typeof(_822 = move _823 as *const i8 (Misc)) = *const {l3950} i8
        // 2966: b"out of memory ... _char:   l3950 = UNIQUE | NON_NULL, (empty)
        // 2966: b"out of memory\0": typeof(_824 = &raw const (*_825)) = *const {l3948} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 2966: b"out of memory\0":   l3948 = UNIQUE | NON_NULL, (empty)
        exit(1 as libc::c_int);
    }
    memset(workers as *mut libc::c_void, 0 as libc::c_int, BYTES);
    // 2969: memset(workers  ... YTES): typeof(_828) = *mut {l1288} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2969: memset(workers  ... YTES):   l1288 = UNIQUE | NON_NULL, (empty)
    // 2969: workers as *mut ... _void: typeof(_829) = *mut {l1290} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2969: workers as *mut ... _void:   l1290 = UNIQUE | NON_NULL, (empty)
    // 2969: workers: typeof(_830) = *mut {l1292} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2969: workers:   l1292 = UNIQUE | NON_NULL, (empty)
    // 2969: workers: typeof(_831) = *mut {l1294} *mut {l1295} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2969: workers:   l1294 = UNIQUE | NON_NULL, (empty)
    // 2969: workers:   l1295 = UNIQUE | NON_NULL, (empty)
    // 2969: workers as *mut ... _void: typeof(_829 = move _830 as *mut libc::c_void (Misc)) = *mut {l3951} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2969: workers as *mut ... _void:   l3951 = UNIQUE | NON_NULL, (empty)
    incmem(BYTES);
    let ref mut fresh18 = (*workers.offset(0 as libc::c_int as isize)).lgl;
    // 2971: ref mut fresh18: typeof(_836) = &mut {l1301} *mut {l1302} LGL
    // 2971: ref mut fresh18:   l1301 = UNIQUE | NON_NULL, FIXED
    // 2971: ref mut fresh18:   l1302 = UNIQUE | NON_NULL, (empty)
    // 2971: workers.offset( ... size): typeof(_837) = *mut {l1304} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2971: workers.offset( ... size):   l1304 = UNIQUE | NON_NULL, (empty)
    // 2971: workers: typeof(_838) = *mut {l1306} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2971: workers:   l1306 = UNIQUE | NON_NULL, (empty)
    // 2971: workers: typeof(_839) = *mut {l1308} *mut {l1309} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 2971: workers:   l1308 = UNIQUE | NON_NULL, (empty)
    // 2971: workers:   l1309 = UNIQUE | NON_NULL, (empty)
    // 2971: ref mut fresh18: typeof(_836 = &mut ((*_837).0: *mut LGL)) = &mut {l3952} *mut {l3953} LGL
    // 2971: ref mut fresh18:   l3952 = UNIQUE | NON_NULL, (empty)
    // 2971: ref mut fresh18:   l3953 = UNIQUE | NON_NULL, (empty)
    *fresh18 = lglminit(
    // 2972: lglminit( 0 as  ... )), ): typeof(_842) = *mut {l1313} LGL
    // 2972: lglminit( 0 as  ... )), ):   l1313 = UNIQUE | NON_NULL, (empty)
        0 as *mut libc::c_void,
        // 2973: 0 as *mut libc: ... _void: typeof(_843) = *mut {l1315} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2973: 0 as *mut libc: ... _void:   l1315 = UNIQUE | NON_NULL, (empty)
        // 2973: 0 as *mut libc: ... _void: typeof(_843 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l3954} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2973: 0 as *mut libc: ... _void:   l3954 = UNIQUE | NON_NULL, (empty)
        Some(alloc_shim as unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void),
        // 2974: Some(alloc as u ... void): typeof(_844) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l1317} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {l1318} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 2974: Some(alloc as u ... void):   l1317 = UNIQUE | NON_NULL, (empty)
        // 2974: Some(alloc as u ... void):   l1318 = UNIQUE | NON_NULL, (empty)
        // 2974: alloc as unsafe ... _void: typeof(_845) = fn(*mut {l1320} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {l1321} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2974: alloc as unsafe ... _void:   l1320 = UNIQUE | NON_NULL, (empty)
        // 2974: alloc as unsafe ... _void:   l1321 = UNIQUE | NON_NULL, (empty)
        // 2974: Some(alloc as u ... void): typeof(_844 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void>::Some(move _845)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l3957} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {l3958} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 2974: Some(alloc as u ... void):   l3957 = UNIQUE | NON_NULL, (empty)
        // 2974: Some(alloc as u ... void):   l3958 = UNIQUE | NON_NULL, (empty)
        // 2974: alloc: typeof(_845 = alloc as unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l3955} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {l3956} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2974: alloc:   l3955 = UNIQUE | NON_NULL, (empty)
        // 2974: alloc:   l3956 = UNIQUE | NON_NULL, (empty)
        Some(
        // 2975: Some( resize as ... id, ): typeof(_846) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l1323} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l1324} DefId(2:5583 ~ core[480a]::ffi::c_void), u64, u64) -> *mut {l1325} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 2975: Some( resize as ... id, ):   l1323 = UNIQUE | NON_NULL, (empty)
        // 2975: Some( resize as ... id, ):   l1324 = UNIQUE | NON_NULL, (empty)
        // 2975: Some( resize as ... id, ):   l1325 = UNIQUE | NON_NULL, (empty)
        // 2975: Some( resize as ... id, ): typeof(_846 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, u64, u64) -> *mut libc::c_void>::Some(move _847)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l3962} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l3963} DefId(2:5583 ~ core[480a]::ffi::c_void), u64, u64) -> *mut {l3964} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 2975: Some( resize as ... id, ):   l3962 = UNIQUE | NON_NULL, (empty)
        // 2975: Some( resize as ... id, ):   l3963 = UNIQUE | NON_NULL, (empty)
        // 2975: Some( resize as ... id, ):   l3964 = UNIQUE | NON_NULL, (empty)
            resize
            // 2976: resize as unsaf ... _void: typeof(_847) = fn(*mut {l1327} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l1328} DefId(2:5583 ~ core[480a]::ffi::c_void), u64, u64) -> *mut {l1329} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2976: resize as unsaf ... _void:   l1327 = UNIQUE | NON_NULL, (empty)
            // 2976: resize as unsaf ... _void:   l1328 = UNIQUE | NON_NULL, (empty)
            // 2976: resize as unsaf ... _void:   l1329 = UNIQUE | NON_NULL, (empty)
            // 2976: resize: typeof(_847 = resize as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, u64, u64) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l3959} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l3960} DefId(2:5583 ~ core[480a]::ffi::c_void), u64, u64) -> *mut {l3961} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2976: resize:   l3959 = UNIQUE | NON_NULL, (empty)
            // 2976: resize:   l3960 = UNIQUE | NON_NULL, (empty)
            // 2976: resize:   l3961 = UNIQUE | NON_NULL, (empty)
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    size_t,
                    size_t,
                ) -> *mut libc::c_void,
        ),
        Some(dealloc_shim as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, size_t) -> ()),
        // 2984: Some(dealloc as ... > ()): typeof(_848) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l1331} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l1332} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> ()>
        // 2984: Some(dealloc as ... > ()):   l1331 = UNIQUE | NON_NULL, (empty)
        // 2984: Some(dealloc as ... > ()):   l1332 = UNIQUE | NON_NULL, (empty)
        // 2984: dealloc as unsa ... -> (): typeof(_849) = fn(*mut {l1334} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l1335} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> ()
        // 2984: dealloc as unsa ... -> ():   l1334 = UNIQUE | NON_NULL, (empty)
        // 2984: dealloc as unsa ... -> ():   l1335 = UNIQUE | NON_NULL, (empty)
        // 2984: Some(dealloc as ... > ()): typeof(_848 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, u64)>::Some(move _849)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l3967} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l3968} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> ()>
        // 2984: Some(dealloc as ... > ()):   l3967 = UNIQUE | NON_NULL, (empty)
        // 2984: Some(dealloc as ... > ()):   l3968 = UNIQUE | NON_NULL, (empty)
        // 2984: dealloc: typeof(_849 = dealloc as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, u64) (Pointer(ReifyFnPointer))) = fn(*mut {l3965} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l3966} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> ()
        // 2984: dealloc:   l3965 = UNIQUE | NON_NULL, (empty)
        // 2984: dealloc:   l3966 = UNIQUE | NON_NULL, (empty)
    );
    lglsetopt(
        (*workers.offset(0 as libc::c_int as isize)).lgl,
        // 2987: (*workers.offse ... ).lgl: typeof(_851) = *mut {l1338} LGL
        // 2987: (*workers.offse ... ).lgl:   l1338 = UNIQUE | NON_NULL, (empty)
        // 2987: workers.offset( ... size): typeof(_852) = *mut {l1340} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 2987: workers.offset( ... size):   l1340 = UNIQUE | NON_NULL, (empty)
        // 2987: workers: typeof(_853) = *mut {l1342} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 2987: workers:   l1342 = UNIQUE | NON_NULL, (empty)
        // 2987: workers: typeof(_854) = *mut {l1344} *mut {l1345} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 2987: workers:   l1344 = UNIQUE | NON_NULL, (empty)
        // 2987: workers:   l1345 = UNIQUE | NON_NULL, (empty)
        b"druplig\0" as *const u8 as *const libc::c_char,
        // 2988: b"druplig\0" as ... _char: typeof(_857) = *const {l1349} i8
        // 2988: b"druplig\0" as ... _char:   l1349 = UNIQUE | NON_NULL, (empty)
        // 2988: b"druplig\0" as ... st u8: typeof(_858) = *const {l1351} u8
        // 2988: b"druplig\0" as ... st u8:   l1351 = UNIQUE | NON_NULL, (empty)
        // 2988: b"druplig\0": typeof(_859) = *const {l1353} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 2988: b"druplig\0":   l1353 = UNIQUE | NON_NULL, (empty)
        // 2988: b"druplig\0": typeof(_860) = & {l1355} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 2988: b"druplig\0":   l1355 = UNIQUE | NON_NULL, FIXED
        // 2988: b"druplig\0" as ... st u8: typeof(_858 = move _859 as *const u8 (Pointer(ArrayToPointer))) = *const {l3971} u8
        // 2988: b"druplig\0" as ... st u8:   l3971 = UNIQUE | NON_NULL, (empty)
        // 2988: b"druplig\0": typeof(_859 = &raw const (*_860)) = *const {l3970} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 2988: b"druplig\0":   l3970 = UNIQUE | NON_NULL, (empty)
        // 2988: b"druplig\0" as ... _char: typeof(_857 = move _858 as *const i8 (Misc)) = *const {l3972} i8
        // 2988: b"druplig\0" as ... _char:   l3972 = UNIQUE | NON_NULL, (empty)
        // 2988: b"druplig\0": typeof(_860 = const b"druplig\x00") = & {l3969} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 2988: b"druplig\0":   l3969 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int,
    );
    setopt(
        0 as libc::c_int,
        (*workers.offset(0 as libc::c_int as isize)).lgl,
        // 2993: (*workers.offse ... ).lgl: typeof(_864) = *mut {l1360} LGL
        // 2993: (*workers.offse ... ).lgl:   l1360 = UNIQUE | NON_NULL, (empty)
        // 2993: workers.offset( ... size): typeof(_865) = *mut {l1362} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 2993: workers.offset( ... size):   l1362 = UNIQUE | NON_NULL, (empty)
        // 2993: workers: typeof(_866) = *mut {l1364} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 2993: workers:   l1364 = UNIQUE | NON_NULL, (empty)
        // 2993: workers: typeof(_867) = *mut {l1366} *mut {l1367} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 2993: workers:   l1366 = UNIQUE | NON_NULL, (empty)
        // 2993: workers:   l1367 = UNIQUE | NON_NULL, (empty)
        b"bca\0" as *const u8 as *const libc::c_char,
        // 2994: b"bca\0" as *co ... _char: typeof(_870) = *const {l1371} i8
        // 2994: b"bca\0" as *co ... _char:   l1371 = UNIQUE | NON_NULL, (empty)
        // 2994: b"bca\0" as *co ... st u8: typeof(_871) = *const {l1373} u8
        // 2994: b"bca\0" as *co ... st u8:   l1373 = UNIQUE | NON_NULL, (empty)
        // 2994: b"bca\0": typeof(_872) = *const {l1375} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
        // 2994: b"bca\0":   l1375 = UNIQUE | NON_NULL, (empty)
        // 2994: b"bca\0": typeof(_873) = & {l1377} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
        // 2994: b"bca\0":   l1377 = UNIQUE | NON_NULL, FIXED
        // 2994: b"bca\0" as *co ... st u8: typeof(_871 = move _872 as *const u8 (Pointer(ArrayToPointer))) = *const {l3975} u8
        // 2994: b"bca\0" as *co ... st u8:   l3975 = UNIQUE | NON_NULL, (empty)
        // 2994: b"bca\0": typeof(_872 = &raw const (*_873)) = *const {l3974} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
        // 2994: b"bca\0":   l3974 = UNIQUE | NON_NULL, (empty)
        // 2994: b"bca\0": typeof(_873 = const b"bca\x00") = & {l3973} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
        // 2994: b"bca\0":   l3973 = UNIQUE | NON_NULL, (empty)
        // 2994: b"bca\0" as *co ... _char: typeof(_870 = move _871 as *const i8 (Misc)) = *const {l3976} i8
        // 2994: b"bca\0" as *co ... _char:   l3976 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int,
    );
    if verbose == 0 {
    // 2997: verbose: typeof(_878) = *mut {l1383} i32
    // 2997: verbose:   l1383 = UNIQUE | NON_NULL, (empty)
        setopt(
            0 as libc::c_int,
            (*workers.offset(0 as libc::c_int as isize)).lgl,
            // 3000: (*workers.offse ... ).lgl: typeof(_881) = *mut {l1387} LGL
            // 3000: (*workers.offse ... ).lgl:   l1387 = UNIQUE | NON_NULL, (empty)
            // 3000: workers.offset( ... size): typeof(_882) = *mut {l1389} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3000: workers.offset( ... size):   l1389 = UNIQUE | NON_NULL, (empty)
            // 3000: workers: typeof(_883) = *mut {l1391} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3000: workers:   l1391 = UNIQUE | NON_NULL, (empty)
            // 3000: workers: typeof(_884) = *mut {l1393} *mut {l1394} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3000: workers:   l1393 = UNIQUE | NON_NULL, (empty)
            // 3000: workers:   l1394 = UNIQUE | NON_NULL, (empty)
            b"trep\0" as *const u8 as *const libc::c_char,
            // 3001: b"trep\0" as *c ... _char: typeof(_887) = *const {l1398} i8
            // 3001: b"trep\0" as *c ... _char:   l1398 = UNIQUE | NON_NULL, (empty)
            // 3001: b"trep\0" as *c ... st u8: typeof(_888) = *const {l1400} u8
            // 3001: b"trep\0" as *c ... st u8:   l1400 = UNIQUE | NON_NULL, (empty)
            // 3001: b"trep\0": typeof(_889) = *const {l1402} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3001: b"trep\0":   l1402 = UNIQUE | NON_NULL, (empty)
            // 3001: b"trep\0": typeof(_890) = & {l1404} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3001: b"trep\0":   l1404 = UNIQUE | NON_NULL, FIXED
            // 3001: b"trep\0": typeof(_890 = const b"trep\x00") = & {l3977} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3001: b"trep\0":   l3977 = UNIQUE | NON_NULL, (empty)
            // 3001: b"trep\0" as *c ... _char: typeof(_887 = move _888 as *const i8 (Misc)) = *const {l3980} i8
            // 3001: b"trep\0" as *c ... _char:   l3980 = UNIQUE | NON_NULL, (empty)
            // 3001: b"trep\0" as *c ... st u8: typeof(_888 = move _889 as *const u8 (Pointer(ArrayToPointer))) = *const {l3979} u8
            // 3001: b"trep\0" as *c ... st u8:   l3979 = UNIQUE | NON_NULL, (empty)
            // 3001: b"trep\0": typeof(_889 = &raw const (*_890)) = *const {l3978} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3001: b"trep\0":   l3978 = UNIQUE | NON_NULL, (empty)
            1 as libc::c_int,
        );
    }
    if verbose == 0 {
    // 3005: verbose: typeof(_895) = *mut {l1410} i32
    // 3005: verbose:   l1410 = UNIQUE | NON_NULL, (empty)
        setopt(
            0 as libc::c_int,
            (*workers.offset(0 as libc::c_int as isize)).lgl,
            // 3008: (*workers.offse ... ).lgl: typeof(_898) = *mut {l1414} LGL
            // 3008: (*workers.offse ... ).lgl:   l1414 = UNIQUE | NON_NULL, (empty)
            // 3008: workers.offset( ... size): typeof(_899) = *mut {l1416} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3008: workers.offset( ... size):   l1416 = UNIQUE | NON_NULL, (empty)
            // 3008: workers: typeof(_900) = *mut {l1418} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3008: workers:   l1418 = UNIQUE | NON_NULL, (empty)
            // 3008: workers: typeof(_901) = *mut {l1420} *mut {l1421} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3008: workers:   l1420 = UNIQUE | NON_NULL, (empty)
            // 3008: workers:   l1421 = UNIQUE | NON_NULL, (empty)
            b"profile\0" as *const u8 as *const libc::c_char,
            // 3009: b"profile\0" as ... _char: typeof(_904) = *const {l1425} i8
            // 3009: b"profile\0" as ... _char:   l1425 = UNIQUE | NON_NULL, (empty)
            // 3009: b"profile\0" as ... st u8: typeof(_905) = *const {l1427} u8
            // 3009: b"profile\0" as ... st u8:   l1427 = UNIQUE | NON_NULL, (empty)
            // 3009: b"profile\0": typeof(_906) = *const {l1429} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 3009: b"profile\0":   l1429 = UNIQUE | NON_NULL, (empty)
            // 3009: b"profile\0": typeof(_907) = & {l1431} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 3009: b"profile\0":   l1431 = UNIQUE | NON_NULL, FIXED
            // 3009: b"profile\0" as ... st u8: typeof(_905 = move _906 as *const u8 (Pointer(ArrayToPointer))) = *const {l3983} u8
            // 3009: b"profile\0" as ... st u8:   l3983 = UNIQUE | NON_NULL, (empty)
            // 3009: b"profile\0": typeof(_906 = &raw const (*_907)) = *const {l3982} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 3009: b"profile\0":   l3982 = UNIQUE | NON_NULL, (empty)
            // 3009: b"profile\0" as ... _char: typeof(_904 = move _905 as *const i8 (Misc)) = *const {l3984} i8
            // 3009: b"profile\0" as ... _char:   l3984 = UNIQUE | NON_NULL, (empty)
            // 3009: b"profile\0": typeof(_907 = const b"profile\x00") = & {l3981} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 3009: b"profile\0":   l3981 = UNIQUE | NON_NULL, (empty)
            0 as libc::c_int,
        );
    }
    if verbose != 0 {
    // 3013: verbose: typeof(_912) = *mut {l1437} i32
    // 3013: verbose:   l1437 = UNIQUE | NON_NULL, (empty)
        lglopts(
            (*workers.offset(0 as libc::c_int as isize)).lgl,
            // 3015: (*workers.offse ... ).lgl: typeof(_914) = *mut {l1440} LGL
            // 3015: (*workers.offse ... ).lgl:   l1440 = UNIQUE | NON_NULL, (empty)
            // 3015: workers.offset( ... size): typeof(_915) = *mut {l1442} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3015: workers.offset( ... size):   l1442 = UNIQUE | NON_NULL, (empty)
            // 3015: workers: typeof(_916) = *mut {l1444} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3015: workers:   l1444 = UNIQUE | NON_NULL, (empty)
            // 3015: workers: typeof(_917) = *mut {l1446} *mut {l1447} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3015: workers:   l1446 = UNIQUE | NON_NULL, (empty)
            // 3015: workers:   l1447 = UNIQUE | NON_NULL, (empty)
            b"c 0 \0" as *const u8 as *const libc::c_char,
            // 3016: b"c 0 \0" as *c ... _char: typeof(_920) = *const {l1451} i8
            // 3016: b"c 0 \0" as *c ... _char:   l1451 = UNIQUE | NON_NULL, (empty)
            // 3016: b"c 0 \0" as *c ... st u8: typeof(_921) = *const {l1453} u8
            // 3016: b"c 0 \0" as *c ... st u8:   l1453 = UNIQUE | NON_NULL, (empty)
            // 3016: b"c 0 \0": typeof(_922) = *const {l1455} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3016: b"c 0 \0":   l1455 = UNIQUE | NON_NULL, (empty)
            // 3016: b"c 0 \0": typeof(_923) = & {l1457} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3016: b"c 0 \0":   l1457 = UNIQUE | NON_NULL, FIXED
            // 3016: b"c 0 \0" as *c ... _char: typeof(_920 = move _921 as *const i8 (Misc)) = *const {l3988} i8
            // 3016: b"c 0 \0" as *c ... _char:   l3988 = UNIQUE | NON_NULL, (empty)
            // 3016: b"c 0 \0": typeof(_923 = const b"c 0 \x00") = & {l3985} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3016: b"c 0 \0":   l3985 = UNIQUE | NON_NULL, (empty)
            // 3016: b"c 0 \0": typeof(_922 = &raw const (*_923)) = *const {l3986} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3016: b"c 0 \0":   l3986 = UNIQUE | NON_NULL, (empty)
            // 3016: b"c 0 \0" as *c ... st u8: typeof(_921 = move _922 as *const u8 (Pointer(ArrayToPointer))) = *const {l3987} u8
            // 3016: b"c 0 \0" as *c ... st u8:   l3987 = UNIQUE | NON_NULL, (empty)
            0 as libc::c_int,
        );
    }
    setsighandlers();
    if !name.is_null() {
    // 3021: name: typeof(_929) = *const {l1464} i8
    // 3021: name:   l1464 = UNIQUE | NON_NULL, (empty)
    // 3021: name: typeof(_930) = *mut {l1466} *const {l1467} i8
    // 3021: name:   l1466 = UNIQUE | NON_NULL, (empty)
    // 3021: name:   l1467 = UNIQUE | NON_NULL, (empty)
        if strlen(name) >= 3 as libc::c_int as libc::c_ulong
        // 3022: name: typeof(_935) = *const {l1473} i8
        // 3022: name:   l1473 = UNIQUE | NON_NULL, (empty)
        // 3022: name: typeof(_936) = *mut {l1475} *const {l1476} i8
        // 3022: name:   l1475 = UNIQUE | NON_NULL, (empty)
        // 3022: name:   l1476 = UNIQUE | NON_NULL, (empty)
            && strcmp(
                name.offset(strlen(name) as isize)
                // 3024: name.offset(str ... ize)): typeof(_941) = *const {l1482} i8
                // 3024: name.offset(str ... ize)):   l1482 = UNIQUE | NON_NULL, (empty)
                // 3024: name.offset(str ... size): typeof(_942) = *const {l1484} i8
                // 3024: name.offset(str ... size):   l1484 = UNIQUE | NON_NULL, (empty)
                // 3024: name: typeof(_943) = *const {l1486} i8
                // 3024: name:   l1486 = UNIQUE | NON_NULL, (empty)
                // 3024: name: typeof(_944) = *mut {l1488} *const {l1489} i8
                // 3024: name:   l1488 = UNIQUE | NON_NULL, (empty)
                // 3024: name:   l1489 = UNIQUE | NON_NULL, (empty)
                // 3024: name: typeof(_947) = *const {l1493} i8
                // 3024: name:   l1493 = UNIQUE | NON_NULL, (empty)
                // 3024: name: typeof(_948) = *mut {l1495} *const {l1496} i8
                // 3024: name:   l1495 = UNIQUE | NON_NULL, (empty)
                // 3024: name:   l1496 = UNIQUE | NON_NULL, (empty)
                    .offset(-(3 as libc::c_int as isize)),
                b".gz\0" as *const u8 as *const libc::c_char,
                // 3026: b".gz\0" as *co ... _char: typeof(_953) = *const {l1502} i8
                // 3026: b".gz\0" as *co ... _char:   l1502 = UNIQUE | NON_NULL, (empty)
                // 3026: b".gz\0" as *co ... st u8: typeof(_954) = *const {l1504} u8
                // 3026: b".gz\0" as *co ... st u8:   l1504 = UNIQUE | NON_NULL, (empty)
                // 3026: b".gz\0": typeof(_955) = *const {l1506} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 3026: b".gz\0":   l1506 = UNIQUE | NON_NULL, (empty)
                // 3026: b".gz\0": typeof(_956) = & {l1508} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 3026: b".gz\0":   l1508 = UNIQUE | NON_NULL, FIXED
                // 3026: b".gz\0" as *co ... st u8: typeof(_954 = move _955 as *const u8 (Pointer(ArrayToPointer))) = *const {l3991} u8
                // 3026: b".gz\0" as *co ... st u8:   l3991 = UNIQUE | NON_NULL, (empty)
                // 3026: b".gz\0" as *co ... _char: typeof(_953 = move _954 as *const i8 (Misc)) = *const {l3992} i8
                // 3026: b".gz\0" as *co ... _char:   l3992 = UNIQUE | NON_NULL, (empty)
                // 3026: b".gz\0": typeof(_955 = &raw const (*_956)) = *const {l3990} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 3026: b".gz\0":   l3990 = UNIQUE | NON_NULL, (empty)
                // 3026: b".gz\0": typeof(_956 = const b".gz\x00") = & {l3989} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 3026: b".gz\0":   l3989 = UNIQUE | NON_NULL, (empty)
            ) == 0
        {
            bytes = (strlen(name)).wrapping_add(30 as libc::c_int as libc::c_ulong);
            // 3029: name: typeof(_959) = *const {l1512} i8
            // 3029: name:   l1512 = UNIQUE | NON_NULL, (empty)
            // 3029: name: typeof(_960) = *mut {l1514} *const {l1515} i8
            // 3029: name:   l1514 = UNIQUE | NON_NULL, (empty)
            // 3029: name:   l1515 = UNIQUE | NON_NULL, (empty)
            cmd = alloc_shim(0 as *mut libc::c_void, bytes) as *mut libc::c_char;
            // 3030: alloc(0 as *mut ... ytes): typeof(_963) = *mut {l1519} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3030: alloc(0 as *mut ... ytes):   l1519 = UNIQUE | NON_NULL, (empty)
            // 3030: 0 as *mut libc: ... _void: typeof(_964) = *mut {l1521} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3030: 0 as *mut libc: ... _void:   l1521 = UNIQUE | NON_NULL, (empty)
            // 3030: cmd = alloc(0 a ... _char: typeof(_27 = move _963 as *mut i8 (Misc)) = *mut {l3994} i8
            // 3030: cmd = alloc(0 a ... _char:   l3994 = UNIQUE | NON_NULL, (empty)
            // 3030: 0 as *mut libc: ... _void: typeof(_964 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l3993} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3030: 0 as *mut libc: ... _void:   l3993 = UNIQUE | NON_NULL, (empty)
            sprintf(
                cmd,
                // 3032: cmd: typeof(_967) = *mut {l1525} i8
                // 3032: cmd:   l1525 = UNIQUE | NON_NULL, (empty)
                b"gunzip -c %s\0" as *const u8 as *const libc::c_char,
                // 3033: b"gunzip -c %s\ ... _char: typeof(_968) = *const {l1527} i8
                // 3033: b"gunzip -c %s\ ... _char:   l1527 = UNIQUE | NON_NULL, (empty)
                // 3033: b"gunzip -c %s\ ... st u8: typeof(_969) = *const {l1529} u8
                // 3033: b"gunzip -c %s\ ... st u8:   l1529 = UNIQUE | NON_NULL, (empty)
                // 3033: b"gunzip -c %s\0": typeof(_970) = *const {l1531} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 3033: b"gunzip -c %s\0":   l1531 = UNIQUE | NON_NULL, (empty)
                // 3033: b"gunzip -c %s\0": typeof(_971) = & {l1533} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 3033: b"gunzip -c %s\0":   l1533 = UNIQUE | NON_NULL, FIXED
                // 3033: b"gunzip -c %s\0": typeof(_971 = const b"gunzip -c %s\x00") = & {l3995} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 3033: b"gunzip -c %s\0":   l3995 = UNIQUE | NON_NULL, (empty)
                // 3033: b"gunzip -c %s\0": typeof(_970 = &raw const (*_971)) = *const {l3996} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 3033: b"gunzip -c %s\0":   l3996 = UNIQUE | NON_NULL, (empty)
                // 3033: b"gunzip -c %s\ ... st u8: typeof(_969 = move _970 as *const u8 (Pointer(ArrayToPointer))) = *const {l3997} u8
                // 3033: b"gunzip -c %s\ ... st u8:   l3997 = UNIQUE | NON_NULL, (empty)
                // 3033: b"gunzip -c %s\ ... _char: typeof(_968 = move _969 as *const i8 (Misc)) = *const {l3998} i8
                // 3033: b"gunzip -c %s\ ... _char:   l3998 = UNIQUE | NON_NULL, (empty)
                name,
                // 3034: name: typeof(_972) = *const {l1535} i8
                // 3034: name:   l1535 = UNIQUE | NON_NULL, (empty)
                // 3034: name: typeof(_973) = *mut {l1537} *const {l1538} i8
                // 3034: name:   l1537 = UNIQUE | NON_NULL, (empty)
                // 3034: name:   l1538 = UNIQUE | NON_NULL, (empty)
            );
            file = popen(cmd, b"r\0" as *const u8 as *const libc::c_char);
            // 3036: popen(cmd, b"r\ ... char): typeof(_974) = *mut {l1540} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 3036: popen(cmd, b"r\ ... char):   l1540 = UNIQUE | NON_NULL, (empty)
            // 3036: cmd: typeof(_975) = *const {l1542} i8
            // 3036: cmd:   l1542 = UNIQUE | NON_NULL, (empty)
            // 3036: cmd: typeof(_976) = *mut {l1544} i8
            // 3036: cmd:   l1544 = UNIQUE | NON_NULL, (empty)
            // 3036: b"r\0" as *cons ... _char: typeof(_977) = *const {l1546} i8
            // 3036: b"r\0" as *cons ... _char:   l1546 = UNIQUE | NON_NULL, (empty)
            // 3036: b"r\0" as *const u8: typeof(_978) = *const {l1548} u8
            // 3036: b"r\0" as *const u8:   l1548 = UNIQUE | NON_NULL, (empty)
            // 3036: b"r\0": typeof(_979) = *const {l1550} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3036: b"r\0":   l1550 = UNIQUE | NON_NULL, (empty)
            // 3036: b"r\0": typeof(_980) = & {l1552} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3036: b"r\0":   l1552 = UNIQUE | NON_NULL, FIXED
            // 3036: file: typeof(_981) = *mut {l1554} *mut {l1555} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 3036: file:   l1554 = UNIQUE | NON_NULL, (empty)
            // 3036: file:   l1555 = UNIQUE | NON_NULL, (empty)
            // 3036: b"r\0" as *cons ... _char: typeof(_977 = move _978 as *const i8 (Misc)) = *const {l4003} i8
            // 3036: b"r\0" as *cons ... _char:   l4003 = UNIQUE | NON_NULL, (empty)
            // 3036: b"r\0" as *const u8: typeof(_978 = move _979 as *const u8 (Pointer(ArrayToPointer))) = *const {l4002} u8
            // 3036: b"r\0" as *const u8:   l4002 = UNIQUE | NON_NULL, (empty)
            // 3036: cmd: typeof(_975 = move _976 as *const i8 (Pointer(MutToConstPointer))) = *const {l3999} i8
            // 3036: cmd:   l3999 = UNIQUE | NON_NULL, (empty)
            // 3036: b"r\0": typeof(_979 = &raw const (*_980)) = *const {l4001} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3036: b"r\0":   l4001 = UNIQUE | NON_NULL, (empty)
            // 3036: b"r\0": typeof(_980 = const b"r\x00") = & {l4000} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3036: b"r\0":   l4000 = UNIQUE | NON_NULL, (empty)
            dealloc_shim(0 as *mut libc::c_void, cmd as *mut libc::c_void, bytes);
            // 3037: 0 as *mut libc: ... _void: typeof(_983) = *mut {l1558} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3037: 0 as *mut libc: ... _void:   l1558 = UNIQUE | NON_NULL, (empty)
            // 3037: cmd as *mut lib ... _void: typeof(_984) = *mut {l1560} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3037: cmd as *mut lib ... _void:   l1560 = UNIQUE | NON_NULL, (empty)
            // 3037: cmd: typeof(_985) = *mut {l1562} i8
            // 3037: cmd:   l1562 = UNIQUE | NON_NULL, (empty)
            // 3037: 0 as *mut libc: ... _void: typeof(_983 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l4004} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3037: 0 as *mut libc: ... _void:   l4004 = UNIQUE | NON_NULL, (empty)
            // 3037: cmd as *mut lib ... _void: typeof(_984 = move _985 as *mut libc::c_void (Misc)) = *mut {l4005} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3037: cmd as *mut lib ... _void:   l4005 = UNIQUE | NON_NULL, (empty)
            clin = 4 as libc::c_int;
        } else if strlen(name) >= 4 as libc::c_int as libc::c_ulong
        // 3039: name: typeof(_991) = *const {l1569} i8
        // 3039: name:   l1569 = UNIQUE | NON_NULL, (empty)
        // 3039: name: typeof(_992) = *mut {l1571} *const {l1572} i8
        // 3039: name:   l1571 = UNIQUE | NON_NULL, (empty)
        // 3039: name:   l1572 = UNIQUE | NON_NULL, (empty)
            && strcmp(
                name.offset(strlen(name) as isize)
                // 3041: name.offset(str ... ize)): typeof(_997) = *const {l1578} i8
                // 3041: name.offset(str ... ize)):   l1578 = UNIQUE | NON_NULL, (empty)
                // 3041: name.offset(str ... size): typeof(_998) = *const {l1580} i8
                // 3041: name.offset(str ... size):   l1580 = UNIQUE | NON_NULL, (empty)
                // 3041: name: typeof(_999) = *const {l1582} i8
                // 3041: name:   l1582 = UNIQUE | NON_NULL, (empty)
                // 3041: name: typeof(_1000) = *mut {l1584} *const {l1585} i8
                // 3041: name:   l1584 = UNIQUE | NON_NULL, (empty)
                // 3041: name:   l1585 = UNIQUE | NON_NULL, (empty)
                // 3041: name: typeof(_1003) = *const {l1589} i8
                // 3041: name:   l1589 = UNIQUE | NON_NULL, (empty)
                // 3041: name: typeof(_1004) = *mut {l1591} *const {l1592} i8
                // 3041: name:   l1591 = UNIQUE | NON_NULL, (empty)
                // 3041: name:   l1592 = UNIQUE | NON_NULL, (empty)
                    .offset(-(4 as libc::c_int as isize)),
                b".zip\0" as *const u8 as *const libc::c_char,
                // 3043: b".zip\0" as *c ... _char: typeof(_1009) = *const {l1598} i8
                // 3043: b".zip\0" as *c ... _char:   l1598 = UNIQUE | NON_NULL, (empty)
                // 3043: b".zip\0" as *c ... st u8: typeof(_1010) = *const {l1600} u8
                // 3043: b".zip\0" as *c ... st u8:   l1600 = UNIQUE | NON_NULL, (empty)
                // 3043: b".zip\0": typeof(_1011) = *const {l1602} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 3043: b".zip\0":   l1602 = UNIQUE | NON_NULL, (empty)
                // 3043: b".zip\0": typeof(_1012) = & {l1604} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 3043: b".zip\0":   l1604 = UNIQUE | NON_NULL, FIXED
                // 3043: b".zip\0": typeof(_1011 = &raw const (*_1012)) = *const {l4007} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 3043: b".zip\0":   l4007 = UNIQUE | NON_NULL, (empty)
                // 3043: b".zip\0" as *c ... _char: typeof(_1009 = move _1010 as *const i8 (Misc)) = *const {l4009} i8
                // 3043: b".zip\0" as *c ... _char:   l4009 = UNIQUE | NON_NULL, (empty)
                // 3043: b".zip\0" as *c ... st u8: typeof(_1010 = move _1011 as *const u8 (Pointer(ArrayToPointer))) = *const {l4008} u8
                // 3043: b".zip\0" as *c ... st u8:   l4008 = UNIQUE | NON_NULL, (empty)
                // 3043: b".zip\0": typeof(_1012 = const b".zip\x00") = & {l4006} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 3043: b".zip\0":   l4006 = UNIQUE | NON_NULL, (empty)
            ) == 0
        {
            bytes = (strlen(name)).wrapping_add(30 as libc::c_int as libc::c_ulong);
            // 3046: name: typeof(_1015) = *const {l1608} i8
            // 3046: name:   l1608 = UNIQUE | NON_NULL, (empty)
            // 3046: name: typeof(_1016) = *mut {l1610} *const {l1611} i8
            // 3046: name:   l1610 = UNIQUE | NON_NULL, (empty)
            // 3046: name:   l1611 = UNIQUE | NON_NULL, (empty)
            cmd = alloc_shim(0 as *mut libc::c_void, bytes) as *mut libc::c_char;
            // 3047: alloc(0 as *mut ... ytes): typeof(_1019) = *mut {l1615} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3047: alloc(0 as *mut ... ytes):   l1615 = UNIQUE | NON_NULL, (empty)
            // 3047: 0 as *mut libc: ... _void: typeof(_1020) = *mut {l1617} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3047: 0 as *mut libc: ... _void:   l1617 = UNIQUE | NON_NULL, (empty)
            // 3047: 0 as *mut libc: ... _void: typeof(_1020 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l4010} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3047: 0 as *mut libc: ... _void:   l4010 = UNIQUE | NON_NULL, (empty)
            // 3047: cmd = alloc(0 a ... _char: typeof(_27 = move _1019 as *mut i8 (Misc)) = *mut {l4011} i8
            // 3047: cmd = alloc(0 a ... _char:   l4011 = UNIQUE | NON_NULL, (empty)
            sprintf(
                cmd,
                // 3049: cmd: typeof(_1023) = *mut {l1621} i8
                // 3049: cmd:   l1621 = UNIQUE | NON_NULL, (empty)
                b"unzip -p %s\0" as *const u8 as *const libc::c_char,
                // 3050: b"unzip -p %s\0 ... _char: typeof(_1024) = *const {l1623} i8
                // 3050: b"unzip -p %s\0 ... _char:   l1623 = UNIQUE | NON_NULL, (empty)
                // 3050: b"unzip -p %s\0 ... st u8: typeof(_1025) = *const {l1625} u8
                // 3050: b"unzip -p %s\0 ... st u8:   l1625 = UNIQUE | NON_NULL, (empty)
                // 3050: b"unzip -p %s\0": typeof(_1026) = *const {l1627} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                // 3050: b"unzip -p %s\0":   l1627 = UNIQUE | NON_NULL, (empty)
                // 3050: b"unzip -p %s\0": typeof(_1027) = & {l1629} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                // 3050: b"unzip -p %s\0":   l1629 = UNIQUE | NON_NULL, FIXED
                // 3050: b"unzip -p %s\0 ... _char: typeof(_1024 = move _1025 as *const i8 (Misc)) = *const {l4015} i8
                // 3050: b"unzip -p %s\0 ... _char:   l4015 = UNIQUE | NON_NULL, (empty)
                // 3050: b"unzip -p %s\0 ... st u8: typeof(_1025 = move _1026 as *const u8 (Pointer(ArrayToPointer))) = *const {l4014} u8
                // 3050: b"unzip -p %s\0 ... st u8:   l4014 = UNIQUE | NON_NULL, (empty)
                // 3050: b"unzip -p %s\0": typeof(_1027 = const b"unzip -p %s\x00") = & {l4012} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                // 3050: b"unzip -p %s\0":   l4012 = UNIQUE | NON_NULL, (empty)
                // 3050: b"unzip -p %s\0": typeof(_1026 = &raw const (*_1027)) = *const {l4013} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                // 3050: b"unzip -p %s\0":   l4013 = UNIQUE | NON_NULL, (empty)
                name,
                // 3051: name: typeof(_1028) = *const {l1631} i8
                // 3051: name:   l1631 = UNIQUE | NON_NULL, (empty)
                // 3051: name: typeof(_1029) = *mut {l1633} *const {l1634} i8
                // 3051: name:   l1633 = UNIQUE | NON_NULL, (empty)
                // 3051: name:   l1634 = UNIQUE | NON_NULL, (empty)
            );
            file = popen(cmd, b"r\0" as *const u8 as *const libc::c_char);
            // 3053: popen(cmd, b"r\ ... char): typeof(_1030) = *mut {l1636} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 3053: popen(cmd, b"r\ ... char):   l1636 = UNIQUE | NON_NULL, (empty)
            // 3053: cmd: typeof(_1031) = *const {l1638} i8
            // 3053: cmd:   l1638 = UNIQUE | NON_NULL, (empty)
            // 3053: cmd: typeof(_1032) = *mut {l1640} i8
            // 3053: cmd:   l1640 = UNIQUE | NON_NULL, (empty)
            // 3053: b"r\0" as *cons ... _char: typeof(_1033) = *const {l1642} i8
            // 3053: b"r\0" as *cons ... _char:   l1642 = UNIQUE | NON_NULL, (empty)
            // 3053: b"r\0" as *const u8: typeof(_1034) = *const {l1644} u8
            // 3053: b"r\0" as *const u8:   l1644 = UNIQUE | NON_NULL, (empty)
            // 3053: b"r\0": typeof(_1035) = *const {l1646} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3053: b"r\0":   l1646 = UNIQUE | NON_NULL, (empty)
            // 3053: b"r\0": typeof(_1036) = & {l1648} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3053: b"r\0":   l1648 = UNIQUE | NON_NULL, FIXED
            // 3053: file: typeof(_1037) = *mut {l1650} *mut {l1651} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 3053: file:   l1650 = UNIQUE | NON_NULL, (empty)
            // 3053: file:   l1651 = UNIQUE | NON_NULL, (empty)
            // 3053: b"r\0" as *const u8: typeof(_1034 = move _1035 as *const u8 (Pointer(ArrayToPointer))) = *const {l4019} u8
            // 3053: b"r\0" as *const u8:   l4019 = UNIQUE | NON_NULL, (empty)
            // 3053: b"r\0" as *cons ... _char: typeof(_1033 = move _1034 as *const i8 (Misc)) = *const {l4020} i8
            // 3053: b"r\0" as *cons ... _char:   l4020 = UNIQUE | NON_NULL, (empty)
            // 3053: b"r\0": typeof(_1035 = &raw const (*_1036)) = *const {l4018} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3053: b"r\0":   l4018 = UNIQUE | NON_NULL, (empty)
            // 3053: cmd: typeof(_1031 = move _1032 as *const i8 (Pointer(MutToConstPointer))) = *const {l4016} i8
            // 3053: cmd:   l4016 = UNIQUE | NON_NULL, (empty)
            // 3053: b"r\0": typeof(_1036 = const b"r\x00") = & {l4017} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3053: b"r\0":   l4017 = UNIQUE | NON_NULL, (empty)
            dealloc_shim(0 as *mut libc::c_void, cmd as *mut libc::c_void, bytes);
            // 3054: 0 as *mut libc: ... _void: typeof(_1039) = *mut {l1654} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3054: 0 as *mut libc: ... _void:   l1654 = UNIQUE | NON_NULL, (empty)
            // 3054: cmd as *mut lib ... _void: typeof(_1040) = *mut {l1656} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3054: cmd as *mut lib ... _void:   l1656 = UNIQUE | NON_NULL, (empty)
            // 3054: cmd: typeof(_1041) = *mut {l1658} i8
            // 3054: cmd:   l1658 = UNIQUE | NON_NULL, (empty)
            // 3054: 0 as *mut libc: ... _void: typeof(_1039 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l4021} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3054: 0 as *mut libc: ... _void:   l4021 = UNIQUE | NON_NULL, (empty)
            // 3054: cmd as *mut lib ... _void: typeof(_1040 = move _1041 as *mut libc::c_void (Misc)) = *mut {l4022} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3054: cmd as *mut lib ... _void:   l4022 = UNIQUE | NON_NULL, (empty)
            clin = 4 as libc::c_int;
        } else if strlen(name) >= 5 as libc::c_int as libc::c_ulong
        // 3056: name: typeof(_1047) = *const {l1665} i8
        // 3056: name:   l1665 = UNIQUE | NON_NULL, (empty)
        // 3056: name: typeof(_1048) = *mut {l1667} *const {l1668} i8
        // 3056: name:   l1667 = UNIQUE | NON_NULL, (empty)
        // 3056: name:   l1668 = UNIQUE | NON_NULL, (empty)
            && strcmp(
                name.offset(strlen(name) as isize)
                // 3058: name.offset(str ... ize)): typeof(_1053) = *const {l1674} i8
                // 3058: name.offset(str ... ize)):   l1674 = UNIQUE | NON_NULL, (empty)
                // 3058: name.offset(str ... size): typeof(_1054) = *const {l1676} i8
                // 3058: name.offset(str ... size):   l1676 = UNIQUE | NON_NULL, (empty)
                // 3058: name: typeof(_1055) = *const {l1678} i8
                // 3058: name:   l1678 = UNIQUE | NON_NULL, (empty)
                // 3058: name: typeof(_1056) = *mut {l1680} *const {l1681} i8
                // 3058: name:   l1680 = UNIQUE | NON_NULL, (empty)
                // 3058: name:   l1681 = UNIQUE | NON_NULL, (empty)
                // 3058: name: typeof(_1059) = *const {l1685} i8
                // 3058: name:   l1685 = UNIQUE | NON_NULL, (empty)
                // 3058: name: typeof(_1060) = *mut {l1687} *const {l1688} i8
                // 3058: name:   l1687 = UNIQUE | NON_NULL, (empty)
                // 3058: name:   l1688 = UNIQUE | NON_NULL, (empty)
                    .offset(-(5 as libc::c_int as isize)),
                b".lzma\0" as *const u8 as *const libc::c_char,
                // 3060: b".lzma\0" as * ... _char: typeof(_1065) = *const {l1694} i8
                // 3060: b".lzma\0" as * ... _char:   l1694 = UNIQUE | NON_NULL, (empty)
                // 3060: b".lzma\0" as * ... st u8: typeof(_1066) = *const {l1696} u8
                // 3060: b".lzma\0" as * ... st u8:   l1696 = UNIQUE | NON_NULL, (empty)
                // 3060: b".lzma\0": typeof(_1067) = *const {l1698} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 3060: b".lzma\0":   l1698 = UNIQUE | NON_NULL, (empty)
                // 3060: b".lzma\0": typeof(_1068) = & {l1700} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 3060: b".lzma\0":   l1700 = UNIQUE | NON_NULL, FIXED
                // 3060: b".lzma\0" as * ... _char: typeof(_1065 = move _1066 as *const i8 (Misc)) = *const {l4026} i8
                // 3060: b".lzma\0" as * ... _char:   l4026 = UNIQUE | NON_NULL, (empty)
                // 3060: b".lzma\0": typeof(_1067 = &raw const (*_1068)) = *const {l4024} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 3060: b".lzma\0":   l4024 = UNIQUE | NON_NULL, (empty)
                // 3060: b".lzma\0": typeof(_1068 = const b".lzma\x00") = & {l4023} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 3060: b".lzma\0":   l4023 = UNIQUE | NON_NULL, (empty)
                // 3060: b".lzma\0" as * ... st u8: typeof(_1066 = move _1067 as *const u8 (Pointer(ArrayToPointer))) = *const {l4025} u8
                // 3060: b".lzma\0" as * ... st u8:   l4025 = UNIQUE | NON_NULL, (empty)
            ) == 0
        {
            bytes = (strlen(name)).wrapping_add(30 as libc::c_int as libc::c_ulong);
            // 3063: name: typeof(_1071) = *const {l1704} i8
            // 3063: name:   l1704 = UNIQUE | NON_NULL, (empty)
            // 3063: name: typeof(_1072) = *mut {l1706} *const {l1707} i8
            // 3063: name:   l1706 = UNIQUE | NON_NULL, (empty)
            // 3063: name:   l1707 = UNIQUE | NON_NULL, (empty)
            cmd = alloc_shim(0 as *mut libc::c_void, bytes) as *mut libc::c_char;
            // 3064: alloc(0 as *mut ... ytes): typeof(_1075) = *mut {l1711} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3064: alloc(0 as *mut ... ytes):   l1711 = UNIQUE | NON_NULL, (empty)
            // 3064: 0 as *mut libc: ... _void: typeof(_1076) = *mut {l1713} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3064: 0 as *mut libc: ... _void:   l1713 = UNIQUE | NON_NULL, (empty)
            // 3064: 0 as *mut libc: ... _void: typeof(_1076 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l4027} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3064: 0 as *mut libc: ... _void:   l4027 = UNIQUE | NON_NULL, (empty)
            // 3064: cmd = alloc(0 a ... _char: typeof(_27 = move _1075 as *mut i8 (Misc)) = *mut {l4028} i8
            // 3064: cmd = alloc(0 a ... _char:   l4028 = UNIQUE | NON_NULL, (empty)
            sprintf(cmd, b"lzcat %s\0" as *const u8 as *const libc::c_char, name);
            // 3065: cmd: typeof(_1079) = *mut {l1717} i8
            // 3065: cmd:   l1717 = UNIQUE | NON_NULL, (empty)
            // 3065: b"lzcat %s\0" a ... _char: typeof(_1080) = *const {l1719} i8
            // 3065: b"lzcat %s\0" a ... _char:   l1719 = UNIQUE | NON_NULL, (empty)
            // 3065: b"lzcat %s\0" a ... st u8: typeof(_1081) = *const {l1721} u8
            // 3065: b"lzcat %s\0" a ... st u8:   l1721 = UNIQUE | NON_NULL, (empty)
            // 3065: b"lzcat %s\0": typeof(_1082) = *const {l1723} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 3065: b"lzcat %s\0":   l1723 = UNIQUE | NON_NULL, (empty)
            // 3065: b"lzcat %s\0": typeof(_1083) = & {l1725} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 3065: b"lzcat %s\0":   l1725 = UNIQUE | NON_NULL, FIXED
            // 3065: name: typeof(_1084) = *const {l1727} i8
            // 3065: name:   l1727 = UNIQUE | NON_NULL, (empty)
            // 3065: name: typeof(_1085) = *mut {l1729} *const {l1730} i8
            // 3065: name:   l1729 = UNIQUE | NON_NULL, (empty)
            // 3065: name:   l1730 = UNIQUE | NON_NULL, (empty)
            // 3065: b"lzcat %s\0" a ... _char: typeof(_1080 = move _1081 as *const i8 (Misc)) = *const {l4032} i8
            // 3065: b"lzcat %s\0" a ... _char:   l4032 = UNIQUE | NON_NULL, (empty)
            // 3065: b"lzcat %s\0" a ... st u8: typeof(_1081 = move _1082 as *const u8 (Pointer(ArrayToPointer))) = *const {l4031} u8
            // 3065: b"lzcat %s\0" a ... st u8:   l4031 = UNIQUE | NON_NULL, (empty)
            // 3065: b"lzcat %s\0": typeof(_1082 = &raw const (*_1083)) = *const {l4030} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 3065: b"lzcat %s\0":   l4030 = UNIQUE | NON_NULL, (empty)
            // 3065: b"lzcat %s\0": typeof(_1083 = const b"lzcat %s\x00") = & {l4029} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 3065: b"lzcat %s\0":   l4029 = UNIQUE | NON_NULL, (empty)
            file = popen(cmd, b"r\0" as *const u8 as *const libc::c_char);
            // 3066: popen(cmd, b"r\ ... char): typeof(_1086) = *mut {l1732} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 3066: popen(cmd, b"r\ ... char):   l1732 = UNIQUE | NON_NULL, (empty)
            // 3066: cmd: typeof(_1087) = *const {l1734} i8
            // 3066: cmd:   l1734 = UNIQUE | NON_NULL, (empty)
            // 3066: cmd: typeof(_1088) = *mut {l1736} i8
            // 3066: cmd:   l1736 = UNIQUE | NON_NULL, (empty)
            // 3066: b"r\0" as *cons ... _char: typeof(_1089) = *const {l1738} i8
            // 3066: b"r\0" as *cons ... _char:   l1738 = UNIQUE | NON_NULL, (empty)
            // 3066: b"r\0" as *const u8: typeof(_1090) = *const {l1740} u8
            // 3066: b"r\0" as *const u8:   l1740 = UNIQUE | NON_NULL, (empty)
            // 3066: b"r\0": typeof(_1091) = *const {l1742} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3066: b"r\0":   l1742 = UNIQUE | NON_NULL, (empty)
            // 3066: b"r\0": typeof(_1092) = & {l1744} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3066: b"r\0":   l1744 = UNIQUE | NON_NULL, FIXED
            // 3066: file: typeof(_1093) = *mut {l1746} *mut {l1747} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 3066: file:   l1746 = UNIQUE | NON_NULL, (empty)
            // 3066: file:   l1747 = UNIQUE | NON_NULL, (empty)
            // 3066: b"r\0": typeof(_1091 = &raw const (*_1092)) = *const {l4035} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3066: b"r\0":   l4035 = UNIQUE | NON_NULL, (empty)
            // 3066: b"r\0" as *cons ... _char: typeof(_1089 = move _1090 as *const i8 (Misc)) = *const {l4037} i8
            // 3066: b"r\0" as *cons ... _char:   l4037 = UNIQUE | NON_NULL, (empty)
            // 3066: b"r\0" as *const u8: typeof(_1090 = move _1091 as *const u8 (Pointer(ArrayToPointer))) = *const {l4036} u8
            // 3066: b"r\0" as *const u8:   l4036 = UNIQUE | NON_NULL, (empty)
            // 3066: cmd: typeof(_1087 = move _1088 as *const i8 (Pointer(MutToConstPointer))) = *const {l4033} i8
            // 3066: cmd:   l4033 = UNIQUE | NON_NULL, (empty)
            // 3066: b"r\0": typeof(_1092 = const b"r\x00") = & {l4034} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3066: b"r\0":   l4034 = UNIQUE | NON_NULL, (empty)
            dealloc_shim(0 as *mut libc::c_void, cmd as *mut libc::c_void, bytes);
            // 3067: 0 as *mut libc: ... _void: typeof(_1095) = *mut {l1750} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3067: 0 as *mut libc: ... _void:   l1750 = UNIQUE | NON_NULL, (empty)
            // 3067: cmd as *mut lib ... _void: typeof(_1096) = *mut {l1752} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3067: cmd as *mut lib ... _void:   l1752 = UNIQUE | NON_NULL, (empty)
            // 3067: cmd: typeof(_1097) = *mut {l1754} i8
            // 3067: cmd:   l1754 = UNIQUE | NON_NULL, (empty)
            // 3067: cmd as *mut lib ... _void: typeof(_1096 = move _1097 as *mut libc::c_void (Misc)) = *mut {l4039} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3067: cmd as *mut lib ... _void:   l4039 = UNIQUE | NON_NULL, (empty)
            // 3067: 0 as *mut libc: ... _void: typeof(_1095 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l4038} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3067: 0 as *mut libc: ... _void:   l4038 = UNIQUE | NON_NULL, (empty)
            clin = 4 as libc::c_int;
        } else if strlen(name) >= 4 as libc::c_int as libc::c_ulong
        // 3069: name: typeof(_1103) = *const {l1761} i8
        // 3069: name:   l1761 = UNIQUE | NON_NULL, (empty)
        // 3069: name: typeof(_1104) = *mut {l1763} *const {l1764} i8
        // 3069: name:   l1763 = UNIQUE | NON_NULL, (empty)
        // 3069: name:   l1764 = UNIQUE | NON_NULL, (empty)
            && strcmp(
                name.offset(strlen(name) as isize)
                // 3071: name.offset(str ... ize)): typeof(_1109) = *const {l1770} i8
                // 3071: name.offset(str ... ize)):   l1770 = UNIQUE | NON_NULL, (empty)
                // 3071: name.offset(str ... size): typeof(_1110) = *const {l1772} i8
                // 3071: name.offset(str ... size):   l1772 = UNIQUE | NON_NULL, (empty)
                // 3071: name: typeof(_1111) = *const {l1774} i8
                // 3071: name:   l1774 = UNIQUE | NON_NULL, (empty)
                // 3071: name: typeof(_1112) = *mut {l1776} *const {l1777} i8
                // 3071: name:   l1776 = UNIQUE | NON_NULL, (empty)
                // 3071: name:   l1777 = UNIQUE | NON_NULL, (empty)
                // 3071: name: typeof(_1115) = *const {l1781} i8
                // 3071: name:   l1781 = UNIQUE | NON_NULL, (empty)
                // 3071: name: typeof(_1116) = *mut {l1783} *const {l1784} i8
                // 3071: name:   l1783 = UNIQUE | NON_NULL, (empty)
                // 3071: name:   l1784 = UNIQUE | NON_NULL, (empty)
                    .offset(-(4 as libc::c_int as isize)),
                b".bz2\0" as *const u8 as *const libc::c_char,
                // 3073: b".bz2\0" as *c ... _char: typeof(_1121) = *const {l1790} i8
                // 3073: b".bz2\0" as *c ... _char:   l1790 = UNIQUE | NON_NULL, (empty)
                // 3073: b".bz2\0" as *c ... st u8: typeof(_1122) = *const {l1792} u8
                // 3073: b".bz2\0" as *c ... st u8:   l1792 = UNIQUE | NON_NULL, (empty)
                // 3073: b".bz2\0": typeof(_1123) = *const {l1794} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 3073: b".bz2\0":   l1794 = UNIQUE | NON_NULL, (empty)
                // 3073: b".bz2\0": typeof(_1124) = & {l1796} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 3073: b".bz2\0":   l1796 = UNIQUE | NON_NULL, FIXED
                // 3073: b".bz2\0" as *c ... _char: typeof(_1121 = move _1122 as *const i8 (Misc)) = *const {l4043} i8
                // 3073: b".bz2\0" as *c ... _char:   l4043 = UNIQUE | NON_NULL, (empty)
                // 3073: b".bz2\0": typeof(_1124 = const b".bz2\x00") = & {l4040} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 3073: b".bz2\0":   l4040 = UNIQUE | NON_NULL, (empty)
                // 3073: b".bz2\0" as *c ... st u8: typeof(_1122 = move _1123 as *const u8 (Pointer(ArrayToPointer))) = *const {l4042} u8
                // 3073: b".bz2\0" as *c ... st u8:   l4042 = UNIQUE | NON_NULL, (empty)
                // 3073: b".bz2\0": typeof(_1123 = &raw const (*_1124)) = *const {l4041} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 3073: b".bz2\0":   l4041 = UNIQUE | NON_NULL, (empty)
            ) == 0
        {
            bytes = (strlen(name)).wrapping_add(30 as libc::c_int as libc::c_ulong);
            // 3076: name: typeof(_1127) = *const {l1800} i8
            // 3076: name:   l1800 = UNIQUE | NON_NULL, (empty)
            // 3076: name: typeof(_1128) = *mut {l1802} *const {l1803} i8
            // 3076: name:   l1802 = UNIQUE | NON_NULL, (empty)
            // 3076: name:   l1803 = UNIQUE | NON_NULL, (empty)
            cmd = alloc_shim(0 as *mut libc::c_void, bytes) as *mut libc::c_char;
            // 3077: alloc(0 as *mut ... ytes): typeof(_1131) = *mut {l1807} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3077: alloc(0 as *mut ... ytes):   l1807 = UNIQUE | NON_NULL, (empty)
            // 3077: 0 as *mut libc: ... _void: typeof(_1132) = *mut {l1809} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3077: 0 as *mut libc: ... _void:   l1809 = UNIQUE | NON_NULL, (empty)
            // 3077: cmd = alloc(0 a ... _char: typeof(_27 = move _1131 as *mut i8 (Misc)) = *mut {l4045} i8
            // 3077: cmd = alloc(0 a ... _char:   l4045 = UNIQUE | NON_NULL, (empty)
            // 3077: 0 as *mut libc: ... _void: typeof(_1132 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l4044} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3077: 0 as *mut libc: ... _void:   l4044 = UNIQUE | NON_NULL, (empty)
            sprintf(cmd, b"bzcat %s\0" as *const u8 as *const libc::c_char, name);
            // 3078: cmd: typeof(_1135) = *mut {l1813} i8
            // 3078: cmd:   l1813 = UNIQUE | NON_NULL, (empty)
            // 3078: b"bzcat %s\0" a ... _char: typeof(_1136) = *const {l1815} i8
            // 3078: b"bzcat %s\0" a ... _char:   l1815 = UNIQUE | NON_NULL, (empty)
            // 3078: b"bzcat %s\0" a ... st u8: typeof(_1137) = *const {l1817} u8
            // 3078: b"bzcat %s\0" a ... st u8:   l1817 = UNIQUE | NON_NULL, (empty)
            // 3078: b"bzcat %s\0": typeof(_1138) = *const {l1819} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 3078: b"bzcat %s\0":   l1819 = UNIQUE | NON_NULL, (empty)
            // 3078: b"bzcat %s\0": typeof(_1139) = & {l1821} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 3078: b"bzcat %s\0":   l1821 = UNIQUE | NON_NULL, FIXED
            // 3078: name: typeof(_1140) = *const {l1823} i8
            // 3078: name:   l1823 = UNIQUE | NON_NULL, (empty)
            // 3078: name: typeof(_1141) = *mut {l1825} *const {l1826} i8
            // 3078: name:   l1825 = UNIQUE | NON_NULL, (empty)
            // 3078: name:   l1826 = UNIQUE | NON_NULL, (empty)
            // 3078: b"bzcat %s\0" a ... st u8: typeof(_1137 = move _1138 as *const u8 (Pointer(ArrayToPointer))) = *const {l4048} u8
            // 3078: b"bzcat %s\0" a ... st u8:   l4048 = UNIQUE | NON_NULL, (empty)
            // 3078: b"bzcat %s\0": typeof(_1138 = &raw const (*_1139)) = *const {l4047} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 3078: b"bzcat %s\0":   l4047 = UNIQUE | NON_NULL, (empty)
            // 3078: b"bzcat %s\0": typeof(_1139 = const b"bzcat %s\x00") = & {l4046} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 3078: b"bzcat %s\0":   l4046 = UNIQUE | NON_NULL, (empty)
            // 3078: b"bzcat %s\0" a ... _char: typeof(_1136 = move _1137 as *const i8 (Misc)) = *const {l4049} i8
            // 3078: b"bzcat %s\0" a ... _char:   l4049 = UNIQUE | NON_NULL, (empty)
            file = popen(cmd, b"r\0" as *const u8 as *const libc::c_char);
            // 3079: popen(cmd, b"r\ ... char): typeof(_1142) = *mut {l1828} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 3079: popen(cmd, b"r\ ... char):   l1828 = UNIQUE | NON_NULL, (empty)
            // 3079: cmd: typeof(_1143) = *const {l1830} i8
            // 3079: cmd:   l1830 = UNIQUE | NON_NULL, (empty)
            // 3079: cmd: typeof(_1144) = *mut {l1832} i8
            // 3079: cmd:   l1832 = UNIQUE | NON_NULL, (empty)
            // 3079: b"r\0" as *cons ... _char: typeof(_1145) = *const {l1834} i8
            // 3079: b"r\0" as *cons ... _char:   l1834 = UNIQUE | NON_NULL, (empty)
            // 3079: b"r\0" as *const u8: typeof(_1146) = *const {l1836} u8
            // 3079: b"r\0" as *const u8:   l1836 = UNIQUE | NON_NULL, (empty)
            // 3079: b"r\0": typeof(_1147) = *const {l1838} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3079: b"r\0":   l1838 = UNIQUE | NON_NULL, (empty)
            // 3079: b"r\0": typeof(_1148) = & {l1840} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3079: b"r\0":   l1840 = UNIQUE | NON_NULL, FIXED
            // 3079: file: typeof(_1149) = *mut {l1842} *mut {l1843} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 3079: file:   l1842 = UNIQUE | NON_NULL, (empty)
            // 3079: file:   l1843 = UNIQUE | NON_NULL, (empty)
            // 3079: b"r\0": typeof(_1148 = const b"r\x00") = & {l4051} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3079: b"r\0":   l4051 = UNIQUE | NON_NULL, (empty)
            // 3079: b"r\0" as *const u8: typeof(_1146 = move _1147 as *const u8 (Pointer(ArrayToPointer))) = *const {l4053} u8
            // 3079: b"r\0" as *const u8:   l4053 = UNIQUE | NON_NULL, (empty)
            // 3079: b"r\0" as *cons ... _char: typeof(_1145 = move _1146 as *const i8 (Misc)) = *const {l4054} i8
            // 3079: b"r\0" as *cons ... _char:   l4054 = UNIQUE | NON_NULL, (empty)
            // 3079: b"r\0": typeof(_1147 = &raw const (*_1148)) = *const {l4052} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3079: b"r\0":   l4052 = UNIQUE | NON_NULL, (empty)
            // 3079: cmd: typeof(_1143 = move _1144 as *const i8 (Pointer(MutToConstPointer))) = *const {l4050} i8
            // 3079: cmd:   l4050 = UNIQUE | NON_NULL, (empty)
            dealloc_shim(0 as *mut libc::c_void, cmd as *mut libc::c_void, bytes);
            // 3080: 0 as *mut libc: ... _void: typeof(_1151) = *mut {l1846} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3080: 0 as *mut libc: ... _void:   l1846 = UNIQUE | NON_NULL, (empty)
            // 3080: cmd as *mut lib ... _void: typeof(_1152) = *mut {l1848} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3080: cmd as *mut lib ... _void:   l1848 = UNIQUE | NON_NULL, (empty)
            // 3080: cmd: typeof(_1153) = *mut {l1850} i8
            // 3080: cmd:   l1850 = UNIQUE | NON_NULL, (empty)
            // 3080: 0 as *mut libc: ... _void: typeof(_1151 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l4055} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3080: 0 as *mut libc: ... _void:   l4055 = UNIQUE | NON_NULL, (empty)
            // 3080: cmd as *mut lib ... _void: typeof(_1152 = move _1153 as *mut libc::c_void (Misc)) = *mut {l4056} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3080: cmd as *mut lib ... _void:   l4056 = UNIQUE | NON_NULL, (empty)
            clin = 4 as libc::c_int;
        } else if strlen(name) >= 3 as libc::c_int as libc::c_ulong
        // 3082: name: typeof(_1159) = *const {l1857} i8
        // 3082: name:   l1857 = UNIQUE | NON_NULL, (empty)
        // 3082: name: typeof(_1160) = *mut {l1859} *const {l1860} i8
        // 3082: name:   l1859 = UNIQUE | NON_NULL, (empty)
        // 3082: name:   l1860 = UNIQUE | NON_NULL, (empty)
            && strcmp(
                name.offset(strlen(name) as isize)
                // 3084: name.offset(str ... ize)): typeof(_1165) = *const {l1866} i8
                // 3084: name.offset(str ... ize)):   l1866 = UNIQUE | NON_NULL, (empty)
                // 3084: name.offset(str ... size): typeof(_1166) = *const {l1868} i8
                // 3084: name.offset(str ... size):   l1868 = UNIQUE | NON_NULL, (empty)
                // 3084: name: typeof(_1167) = *const {l1870} i8
                // 3084: name:   l1870 = UNIQUE | NON_NULL, (empty)
                // 3084: name: typeof(_1168) = *mut {l1872} *const {l1873} i8
                // 3084: name:   l1872 = UNIQUE | NON_NULL, (empty)
                // 3084: name:   l1873 = UNIQUE | NON_NULL, (empty)
                // 3084: name: typeof(_1171) = *const {l1877} i8
                // 3084: name:   l1877 = UNIQUE | NON_NULL, (empty)
                // 3084: name: typeof(_1172) = *mut {l1879} *const {l1880} i8
                // 3084: name:   l1879 = UNIQUE | NON_NULL, (empty)
                // 3084: name:   l1880 = UNIQUE | NON_NULL, (empty)
                    .offset(-(3 as libc::c_int as isize)),
                b".7z\0" as *const u8 as *const libc::c_char,
                // 3086: b".7z\0" as *co ... _char: typeof(_1177) = *const {l1886} i8
                // 3086: b".7z\0" as *co ... _char:   l1886 = UNIQUE | NON_NULL, (empty)
                // 3086: b".7z\0" as *co ... st u8: typeof(_1178) = *const {l1888} u8
                // 3086: b".7z\0" as *co ... st u8:   l1888 = UNIQUE | NON_NULL, (empty)
                // 3086: b".7z\0": typeof(_1179) = *const {l1890} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 3086: b".7z\0":   l1890 = UNIQUE | NON_NULL, (empty)
                // 3086: b".7z\0": typeof(_1180) = & {l1892} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 3086: b".7z\0":   l1892 = UNIQUE | NON_NULL, FIXED
                // 3086: b".7z\0" as *co ... _char: typeof(_1177 = move _1178 as *const i8 (Misc)) = *const {l4060} i8
                // 3086: b".7z\0" as *co ... _char:   l4060 = UNIQUE | NON_NULL, (empty)
                // 3086: b".7z\0" as *co ... st u8: typeof(_1178 = move _1179 as *const u8 (Pointer(ArrayToPointer))) = *const {l4059} u8
                // 3086: b".7z\0" as *co ... st u8:   l4059 = UNIQUE | NON_NULL, (empty)
                // 3086: b".7z\0": typeof(_1179 = &raw const (*_1180)) = *const {l4058} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 3086: b".7z\0":   l4058 = UNIQUE | NON_NULL, (empty)
                // 3086: b".7z\0": typeof(_1180 = const b".7z\x00") = & {l4057} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 3086: b".7z\0":   l4057 = UNIQUE | NON_NULL, (empty)
            ) == 0
        {
            bytes = (strlen(name)).wrapping_add(40 as libc::c_int as libc::c_ulong);
            // 3089: name: typeof(_1183) = *const {l1896} i8
            // 3089: name:   l1896 = UNIQUE | NON_NULL, (empty)
            // 3089: name: typeof(_1184) = *mut {l1898} *const {l1899} i8
            // 3089: name:   l1898 = UNIQUE | NON_NULL, (empty)
            // 3089: name:   l1899 = UNIQUE | NON_NULL, (empty)
            cmd = alloc_shim(0 as *mut libc::c_void, bytes) as *mut libc::c_char;
            // 3090: alloc(0 as *mut ... ytes): typeof(_1187) = *mut {l1903} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3090: alloc(0 as *mut ... ytes):   l1903 = UNIQUE | NON_NULL, (empty)
            // 3090: 0 as *mut libc: ... _void: typeof(_1188) = *mut {l1905} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3090: 0 as *mut libc: ... _void:   l1905 = UNIQUE | NON_NULL, (empty)
            // 3090: 0 as *mut libc: ... _void: typeof(_1188 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l4061} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3090: 0 as *mut libc: ... _void:   l4061 = UNIQUE | NON_NULL, (empty)
            // 3090: cmd = alloc(0 a ... _char: typeof(_27 = move _1187 as *mut i8 (Misc)) = *mut {l4062} i8
            // 3090: cmd = alloc(0 a ... _char:   l4062 = UNIQUE | NON_NULL, (empty)
            sprintf(
                cmd,
                // 3092: cmd: typeof(_1191) = *mut {l1909} i8
                // 3092: cmd:   l1909 = UNIQUE | NON_NULL, (empty)
                b"7z x -so %s 2>/dev/null\0" as *const u8 as *const libc::c_char,
                // 3093: b"7z x -so %s 2 ... _char: typeof(_1192) = *const {l1911} i8
                // 3093: b"7z x -so %s 2 ... _char:   l1911 = UNIQUE | NON_NULL, (empty)
                // 3093: b"7z x -so %s 2 ... st u8: typeof(_1193) = *const {l1913} u8
                // 3093: b"7z x -so %s 2 ... st u8:   l1913 = UNIQUE | NON_NULL, (empty)
                // 3093: b"7z x -so %s 2 ... ll\0": typeof(_1194) = *const {l1915} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
                // 3093: b"7z x -so %s 2 ... ll\0":   l1915 = UNIQUE | NON_NULL, (empty)
                // 3093: b"7z x -so %s 2 ... ll\0": typeof(_1195) = & {l1917} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
                // 3093: b"7z x -so %s 2 ... ll\0":   l1917 = UNIQUE | NON_NULL, FIXED
                // 3093: b"7z x -so %s 2 ... ll\0": typeof(_1194 = &raw const (*_1195)) = *const {l4064} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
                // 3093: b"7z x -so %s 2 ... ll\0":   l4064 = UNIQUE | NON_NULL, (empty)
                // 3093: b"7z x -so %s 2 ... ll\0": typeof(_1195 = const b"7z x -so %s 2>/dev/null\x00") = & {l4063} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
                // 3093: b"7z x -so %s 2 ... ll\0":   l4063 = UNIQUE | NON_NULL, (empty)
                // 3093: b"7z x -so %s 2 ... _char: typeof(_1192 = move _1193 as *const i8 (Misc)) = *const {l4066} i8
                // 3093: b"7z x -so %s 2 ... _char:   l4066 = UNIQUE | NON_NULL, (empty)
                // 3093: b"7z x -so %s 2 ... st u8: typeof(_1193 = move _1194 as *const u8 (Pointer(ArrayToPointer))) = *const {l4065} u8
                // 3093: b"7z x -so %s 2 ... st u8:   l4065 = UNIQUE | NON_NULL, (empty)
                name,
                // 3094: name: typeof(_1196) = *const {l1919} i8
                // 3094: name:   l1919 = UNIQUE | NON_NULL, (empty)
                // 3094: name: typeof(_1197) = *mut {l1921} *const {l1922} i8
                // 3094: name:   l1921 = UNIQUE | NON_NULL, (empty)
                // 3094: name:   l1922 = UNIQUE | NON_NULL, (empty)
            );
            file = popen(cmd, b"r\0" as *const u8 as *const libc::c_char);
            // 3096: popen(cmd, b"r\ ... char): typeof(_1198) = *mut {l1924} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 3096: popen(cmd, b"r\ ... char):   l1924 = UNIQUE | NON_NULL, (empty)
            // 3096: cmd: typeof(_1199) = *const {l1926} i8
            // 3096: cmd:   l1926 = UNIQUE | NON_NULL, (empty)
            // 3096: cmd: typeof(_1200) = *mut {l1928} i8
            // 3096: cmd:   l1928 = UNIQUE | NON_NULL, (empty)
            // 3096: b"r\0" as *cons ... _char: typeof(_1201) = *const {l1930} i8
            // 3096: b"r\0" as *cons ... _char:   l1930 = UNIQUE | NON_NULL, (empty)
            // 3096: b"r\0" as *const u8: typeof(_1202) = *const {l1932} u8
            // 3096: b"r\0" as *const u8:   l1932 = UNIQUE | NON_NULL, (empty)
            // 3096: b"r\0": typeof(_1203) = *const {l1934} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3096: b"r\0":   l1934 = UNIQUE | NON_NULL, (empty)
            // 3096: b"r\0": typeof(_1204) = & {l1936} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3096: b"r\0":   l1936 = UNIQUE | NON_NULL, FIXED
            // 3096: file: typeof(_1205) = *mut {l1938} *mut {l1939} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 3096: file:   l1938 = UNIQUE | NON_NULL, (empty)
            // 3096: file:   l1939 = UNIQUE | NON_NULL, (empty)
            // 3096: b"r\0": typeof(_1204 = const b"r\x00") = & {l4068} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3096: b"r\0":   l4068 = UNIQUE | NON_NULL, (empty)
            // 3096: cmd: typeof(_1199 = move _1200 as *const i8 (Pointer(MutToConstPointer))) = *const {l4067} i8
            // 3096: cmd:   l4067 = UNIQUE | NON_NULL, (empty)
            // 3096: b"r\0": typeof(_1203 = &raw const (*_1204)) = *const {l4069} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3096: b"r\0":   l4069 = UNIQUE | NON_NULL, (empty)
            // 3096: b"r\0" as *cons ... _char: typeof(_1201 = move _1202 as *const i8 (Misc)) = *const {l4071} i8
            // 3096: b"r\0" as *cons ... _char:   l4071 = UNIQUE | NON_NULL, (empty)
            // 3096: b"r\0" as *const u8: typeof(_1202 = move _1203 as *const u8 (Pointer(ArrayToPointer))) = *const {l4070} u8
            // 3096: b"r\0" as *const u8:   l4070 = UNIQUE | NON_NULL, (empty)
            dealloc_shim(0 as *mut libc::c_void, cmd as *mut libc::c_void, bytes);
            // 3097: 0 as *mut libc: ... _void: typeof(_1207) = *mut {l1942} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3097: 0 as *mut libc: ... _void:   l1942 = UNIQUE | NON_NULL, (empty)
            // 3097: cmd as *mut lib ... _void: typeof(_1208) = *mut {l1944} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3097: cmd as *mut lib ... _void:   l1944 = UNIQUE | NON_NULL, (empty)
            // 3097: cmd: typeof(_1209) = *mut {l1946} i8
            // 3097: cmd:   l1946 = UNIQUE | NON_NULL, (empty)
            // 3097: cmd as *mut lib ... _void: typeof(_1208 = move _1209 as *mut libc::c_void (Misc)) = *mut {l4073} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3097: cmd as *mut lib ... _void:   l4073 = UNIQUE | NON_NULL, (empty)
            // 3097: 0 as *mut libc: ... _void: typeof(_1207 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l4072} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3097: 0 as *mut libc: ... _void:   l4072 = UNIQUE | NON_NULL, (empty)
            clin = 4 as libc::c_int;
        } else {
            file = fopen(name, b"r\0" as *const u8 as *const libc::c_char);
            // 3100: fopen(name, b"r ... char): typeof(_1212) = *mut {l1950} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 3100: fopen(name, b"r ... char):   l1950 = UNIQUE | NON_NULL, (empty)
            // 3100: name: typeof(_1213) = *const {l1952} i8
            // 3100: name:   l1952 = UNIQUE | NON_NULL, (empty)
            // 3100: name: typeof(_1214) = *mut {l1954} *const {l1955} i8
            // 3100: name:   l1954 = UNIQUE | NON_NULL, (empty)
            // 3100: name:   l1955 = UNIQUE | NON_NULL, (empty)
            // 3100: b"r\0" as *cons ... _char: typeof(_1215) = *const {l1957} i8
            // 3100: b"r\0" as *cons ... _char:   l1957 = UNIQUE | NON_NULL, (empty)
            // 3100: b"r\0" as *const u8: typeof(_1216) = *const {l1959} u8
            // 3100: b"r\0" as *const u8:   l1959 = UNIQUE | NON_NULL, (empty)
            // 3100: b"r\0": typeof(_1217) = *const {l1961} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3100: b"r\0":   l1961 = UNIQUE | NON_NULL, (empty)
            // 3100: b"r\0": typeof(_1218) = & {l1963} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3100: b"r\0":   l1963 = UNIQUE | NON_NULL, FIXED
            // 3100: file: typeof(_1219) = *mut {l1965} *mut {l1966} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 3100: file:   l1965 = UNIQUE | NON_NULL, (empty)
            // 3100: file:   l1966 = UNIQUE | NON_NULL, (empty)
            // 3100: b"r\0" as *const u8: typeof(_1216 = move _1217 as *const u8 (Pointer(ArrayToPointer))) = *const {l4076} u8
            // 3100: b"r\0" as *const u8:   l4076 = UNIQUE | NON_NULL, (empty)
            // 3100: b"r\0": typeof(_1217 = &raw const (*_1218)) = *const {l4075} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3100: b"r\0":   l4075 = UNIQUE | NON_NULL, (empty)
            // 3100: b"r\0": typeof(_1218 = const b"r\x00") = & {l4074} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 3100: b"r\0":   l4074 = UNIQUE | NON_NULL, (empty)
            // 3100: b"r\0" as *cons ... _char: typeof(_1215 = move _1216 as *const i8 (Misc)) = *const {l4077} i8
            // 3100: b"r\0" as *cons ... _char:   l4077 = UNIQUE | NON_NULL, (empty)
            clin = 1 as libc::c_int;
        }
        if file.is_null() {
        // 3103: file: typeof(_1222) = *mut {l1970} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 3103: file:   l1970 = UNIQUE | NON_NULL, (empty)
        // 3103: file: typeof(_1223) = *mut {l1972} *mut {l1973} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 3103: file:   l1972 = UNIQUE | NON_NULL, (empty)
        // 3103: file:   l1973 = UNIQUE | NON_NULL, (empty)
            die(
                b"can not read %s\0" as *const u8 as *const libc::c_char,
                // 3105: b"can not read  ... _char: typeof(_1225) = *const {l1976} i8
                // 3105: b"can not read  ... _char:   l1976 = UNIQUE | NON_NULL, (empty)
                // 3105: b"can not read  ... st u8: typeof(_1226) = *const {l1978} u8
                // 3105: b"can not read  ... st u8:   l1978 = UNIQUE | NON_NULL, (empty)
                // 3105: b"can not read %s\0": typeof(_1227) = *const {l1980} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
                // 3105: b"can not read %s\0":   l1980 = UNIQUE | NON_NULL, (empty)
                // 3105: b"can not read %s\0": typeof(_1228) = & {l1982} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
                // 3105: b"can not read %s\0":   l1982 = UNIQUE | NON_NULL, FIXED
                // 3105: b"can not read %s\0": typeof(_1228 = const b"can not read %s\x00") = & {l4078} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
                // 3105: b"can not read %s\0":   l4078 = UNIQUE | NON_NULL, (empty)
                // 3105: b"can not read %s\0": typeof(_1227 = &raw const (*_1228)) = *const {l4079} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
                // 3105: b"can not read %s\0":   l4079 = UNIQUE | NON_NULL, (empty)
                // 3105: b"can not read  ... _char: typeof(_1225 = move _1226 as *const i8 (Misc)) = *const {l4081} i8
                // 3105: b"can not read  ... _char:   l4081 = UNIQUE | NON_NULL, (empty)
                // 3105: b"can not read  ... st u8: typeof(_1226 = move _1227 as *const u8 (Pointer(ArrayToPointer))) = *const {l4080} u8
                // 3105: b"can not read  ... st u8:   l4080 = UNIQUE | NON_NULL, (empty)
                name,
                // 3106: name: typeof(_1229) = *const {l1984} i8
                // 3106: name:   l1984 = UNIQUE | NON_NULL, (empty)
                // 3106: name: typeof(_1230) = *mut {l1986} *const {l1987} i8
                // 3106: name:   l1986 = UNIQUE | NON_NULL, (empty)
                // 3106: name:   l1987 = UNIQUE | NON_NULL, (empty)
            );
        }
    } else {
        file = stdin;
        // 3110: stdin: typeof(_1231) = *mut {l1989} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 3110: stdin:   l1989 = UNIQUE | NON_NULL, (empty)
        // 3110: stdin: typeof(_1232) = *mut {l1991} *mut {l1992} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 3110: stdin:   l1991 = UNIQUE | NON_NULL, (empty)
        // 3110: stdin:   l1992 = UNIQUE | NON_NULL, (empty)
        // 3110: file: typeof(_1233) = *mut {l1994} *mut {l1995} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 3110: file:   l1994 = UNIQUE | NON_NULL, (empty)
        // 3110: file:   l1995 = UNIQUE | NON_NULL, (empty)
        name = b"<stdin>\0" as *const u8 as *const libc::c_char;
        // 3111: b"<stdin>\0" as ... st u8: typeof(_1234) = *const {l1997} u8
        // 3111: b"<stdin>\0" as ... st u8:   l1997 = UNIQUE | NON_NULL, (empty)
        // 3111: b"<stdin>\0": typeof(_1235) = *const {l1999} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 3111: b"<stdin>\0":   l1999 = UNIQUE | NON_NULL, (empty)
        // 3111: b"<stdin>\0": typeof(_1236) = & {l2001} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 3111: b"<stdin>\0":   l2001 = UNIQUE | NON_NULL, FIXED
        // 3111: name: typeof(_1237) = *mut {l2003} *const {l2004} i8
        // 3111: name:   l2003 = UNIQUE | NON_NULL, (empty)
        // 3111: name:   l2004 = UNIQUE | NON_NULL, (empty)
        // 3111: name = b"<stdin ... _char: typeof((*_1237) = move _1234 as *const i8 (Misc)) = *const {l4085} i8
        // 3111: name = b"<stdin ... _char:   l4085 = UNIQUE | NON_NULL, (empty)
        // 3111: b"<stdin>\0": typeof(_1236 = const b"<stdin>\x00") = & {l4082} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 3111: b"<stdin>\0":   l4082 = UNIQUE | NON_NULL, (empty)
        // 3111: b"<stdin>\0" as ... st u8: typeof(_1234 = move _1235 as *const u8 (Pointer(ArrayToPointer))) = *const {l4084} u8
        // 3111: b"<stdin>\0" as ... st u8:   l4084 = UNIQUE | NON_NULL, (empty)
        // 3111: b"<stdin>\0": typeof(_1235 = &raw const (*_1236)) = *const {l4083} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 3111: b"<stdin>\0":   l4083 = UNIQUE | NON_NULL, (empty)
    }
    msg(
        -(1 as libc::c_int),
        0 as libc::c_int,
        b"parsing %s\0" as *const u8 as *const libc::c_char,
        // 3116: b"parsing %s\0" ... _char: typeof(_1243) = *const {l2011} i8
        // 3116: b"parsing %s\0" ... _char:   l2011 = UNIQUE | NON_NULL, (empty)
        // 3116: b"parsing %s\0" ... st u8: typeof(_1244) = *const {l2013} u8
        // 3116: b"parsing %s\0" ... st u8:   l2013 = UNIQUE | NON_NULL, (empty)
        // 3116: b"parsing %s\0": typeof(_1245) = *const {l2015} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 3116: b"parsing %s\0":   l2015 = UNIQUE | NON_NULL, (empty)
        // 3116: b"parsing %s\0": typeof(_1246) = & {l2017} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 3116: b"parsing %s\0":   l2017 = UNIQUE | NON_NULL, FIXED
        // 3116: b"parsing %s\0" ... st u8: typeof(_1244 = move _1245 as *const u8 (Pointer(ArrayToPointer))) = *const {l4088} u8
        // 3116: b"parsing %s\0" ... st u8:   l4088 = UNIQUE | NON_NULL, (empty)
        // 3116: b"parsing %s\0": typeof(_1245 = &raw const (*_1246)) = *const {l4087} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 3116: b"parsing %s\0":   l4087 = UNIQUE | NON_NULL, (empty)
        // 3116: b"parsing %s\0": typeof(_1246 = const b"parsing %s\x00") = & {l4086} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 3116: b"parsing %s\0":   l4086 = UNIQUE | NON_NULL, (empty)
        // 3116: b"parsing %s\0" ... _char: typeof(_1243 = move _1244 as *const i8 (Misc)) = *const {l4089} i8
        // 3116: b"parsing %s\0" ... _char:   l4089 = UNIQUE | NON_NULL, (empty)
        name,
        // 3117: name: typeof(_1247) = *const {l2019} i8
        // 3117: name:   l2019 = UNIQUE | NON_NULL, (empty)
        // 3117: name: typeof(_1248) = *mut {l2021} *const {l2022} i8
        // 3117: name:   l2021 = UNIQUE | NON_NULL, (empty)
        // 3117: name:   l2022 = UNIQUE | NON_NULL, (empty)
    );
    errstr = parse();
    // 3119: parse(): typeof(_1249) = *const {l2024} i8
    // 3119: parse():   l2024 = UNIQUE | NON_NULL, (empty)
    if !errstr.is_null() {
    // 3120: errstr: typeof(_1253) = *const {l2029} i8
    // 3120: errstr:   l2029 = UNIQUE | NON_NULL, (empty)
        die(
            b"parse error: %s\0" as *const u8 as *const libc::c_char,
            // 3122: b"parse error:  ... _char: typeof(_1255) = *const {l2032} i8
            // 3122: b"parse error:  ... _char:   l2032 = UNIQUE | NON_NULL, (empty)
            // 3122: b"parse error:  ... st u8: typeof(_1256) = *const {l2034} u8
            // 3122: b"parse error:  ... st u8:   l2034 = UNIQUE | NON_NULL, (empty)
            // 3122: b"parse error: %s\0": typeof(_1257) = *const {l2036} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
            // 3122: b"parse error: %s\0":   l2036 = UNIQUE | NON_NULL, (empty)
            // 3122: b"parse error: %s\0": typeof(_1258) = & {l2038} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
            // 3122: b"parse error: %s\0":   l2038 = UNIQUE | NON_NULL, FIXED
            // 3122: b"parse error:  ... st u8: typeof(_1256 = move _1257 as *const u8 (Pointer(ArrayToPointer))) = *const {l4092} u8
            // 3122: b"parse error:  ... st u8:   l4092 = UNIQUE | NON_NULL, (empty)
            // 3122: b"parse error:  ... _char: typeof(_1255 = move _1256 as *const i8 (Misc)) = *const {l4093} i8
            // 3122: b"parse error:  ... _char:   l4093 = UNIQUE | NON_NULL, (empty)
            // 3122: b"parse error: %s\0": typeof(_1257 = &raw const (*_1258)) = *const {l4091} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
            // 3122: b"parse error: %s\0":   l4091 = UNIQUE | NON_NULL, (empty)
            // 3122: b"parse error: %s\0": typeof(_1258 = const b"parse error: %s\x00") = & {l4090} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
            // 3122: b"parse error: %s\0":   l4090 = UNIQUE | NON_NULL, (empty)
            errstr,
            // 3123: errstr: typeof(_1259) = *const {l2040} i8
            // 3123: errstr:   l2040 = UNIQUE | NON_NULL, (empty)
        );
    }
    if clin == 1 as libc::c_int {
        fclose(file);
        // 3127: file: typeof(_1265) = *mut {l2047} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 3127: file:   l2047 = UNIQUE | NON_NULL, (empty)
        // 3127: file: typeof(_1266) = *mut {l2049} *mut {l2050} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 3127: file:   l2049 = UNIQUE | NON_NULL, (empty)
        // 3127: file:   l2050 = UNIQUE | NON_NULL, (empty)
    }
    if clin == 2 as libc::c_int {
        pclose(file);
        // 3130: file: typeof(_1272) = *mut {l2057} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 3130: file:   l2057 = UNIQUE | NON_NULL, (empty)
        // 3130: file: typeof(_1273) = *mut {l2059} *mut {l2060} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 3130: file:   l2059 = UNIQUE | NON_NULL, (empty)
        // 3130: file:   l2060 = UNIQUE | NON_NULL, (empty)
    }
    locs = 0 as libc::c_int;
    // 3132: locs: typeof(_1275) = *mut {l2063} i32
    // 3132: locs:   l2063 = UNIQUE | NON_NULL, (empty)
    setopts(
        (*workers.offset(0 as libc::c_int as isize)).lgl,
        // 3134: (*workers.offse ... ).lgl: typeof(_1277) = *mut {l2066} LGL
        // 3134: (*workers.offse ... ).lgl:   l2066 = UNIQUE | NON_NULL, (empty)
        // 3134: workers.offset( ... size): typeof(_1278) = *mut {l2068} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3134: workers.offset( ... size):   l2068 = UNIQUE | NON_NULL, (empty)
        // 3134: workers: typeof(_1279) = *mut {l2070} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3134: workers:   l2070 = UNIQUE | NON_NULL, (empty)
        // 3134: workers: typeof(_1280) = *mut {l2072} *mut {l2073} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3134: workers:   l2072 = UNIQUE | NON_NULL, (empty)
        // 3134: workers:   l2073 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int,
    );
    if locs >= 2 as libc::c_int {
    // 3137: locs: typeof(_1287) = *mut {l2081} i32
    // 3137: locs:   l2081 = UNIQUE | NON_NULL, (empty)
        nconsumers = 1 as libc::c_int;
        // 3138: nconsumers: typeof(_1290) = *mut {l2085} i32
        // 3138: nconsumers:   l2085 = UNIQUE | NON_NULL, (empty)
    } else if locs == 1 as libc::c_int {
    // 3139: locs: typeof(_1293) = *mut {l2089} i32
    // 3139: locs:   l2089 = UNIQUE | NON_NULL, (empty)
        nconsumers = 1 as libc::c_int;
        // 3140: nconsumers: typeof(_1296) = *mut {l2093} i32
        // 3140: nconsumers:   l2093 = UNIQUE | NON_NULL, (empty)
        i = 1 as libc::c_int;
        while i < nworkers {
        // 3142: nworkers: typeof(_1301) = *mut {l2099} i32
        // 3142: nworkers:   l2099 = UNIQUE | NON_NULL, (empty)
            if i < 8 as libc::c_int && i & 1 as libc::c_int == 0 {
                nconsumers += 1;
                // 3144: nconsumers: typeof(_1311) = *mut {l2110} i32
                // 3144: nconsumers:   l2110 = UNIQUE | NON_NULL, (empty)
                nconsumers;
                // 3145: nconsumers: typeof(_1314) = *mut {l2114} i32
                // 3145: nconsumers:   l2114 = UNIQUE | NON_NULL, (empty)
            }
            i += 1;
            i;
        }
    } else {
        nconsumers = nworkers;
        // 3151: nworkers: typeof(_1321) = *mut {l2122} i32
        // 3151: nworkers:   l2122 = UNIQUE | NON_NULL, (empty)
        // 3151: nconsumers: typeof(_1322) = *mut {l2124} i32
        // 3151: nconsumers:   l2124 = UNIQUE | NON_NULL, (empty)
    }
    msg(
        -(1 as libc::c_int),
        1 as libc::c_int,
        b"assuming %d consumers out of %d workers\0" as *const u8 as *const libc::c_char,
        // 3156: b"assuming %d c ... _char: typeof(_1328) = *const {l2131} i8
        // 3156: b"assuming %d c ... _char:   l2131 = UNIQUE | NON_NULL, (empty)
        // 3156: b"assuming %d c ... st u8: typeof(_1329) = *const {l2133} u8
        // 3156: b"assuming %d c ... st u8:   l2133 = UNIQUE | NON_NULL, (empty)
        // 3156: b"assuming %d c ... rs\0": typeof(_1330) = *const {l2135} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 3156: b"assuming %d c ... rs\0":   l2135 = UNIQUE | NON_NULL, (empty)
        // 3156: b"assuming %d c ... rs\0": typeof(_1331) = & {l2137} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 3156: b"assuming %d c ... rs\0":   l2137 = UNIQUE | NON_NULL, FIXED
        // 3156: b"assuming %d c ... rs\0": typeof(_1331 = const b"assuming %d consumers out of %d workers\x00") = & {l4094} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 3156: b"assuming %d c ... rs\0":   l4094 = UNIQUE | NON_NULL, (empty)
        // 3156: b"assuming %d c ... rs\0": typeof(_1330 = &raw const (*_1331)) = *const {l4095} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
        // 3156: b"assuming %d c ... rs\0":   l4095 = UNIQUE | NON_NULL, (empty)
        // 3156: b"assuming %d c ... st u8: typeof(_1329 = move _1330 as *const u8 (Pointer(ArrayToPointer))) = *const {l4096} u8
        // 3156: b"assuming %d c ... st u8:   l4096 = UNIQUE | NON_NULL, (empty)
        // 3156: b"assuming %d c ... _char: typeof(_1328 = move _1329 as *const i8 (Misc)) = *const {l4097} i8
        // 3156: b"assuming %d c ... _char:   l4097 = UNIQUE | NON_NULL, (empty)
        nconsumers,
        // 3157: nconsumers: typeof(_1333) = *mut {l2140} i32
        // 3157: nconsumers:   l2140 = UNIQUE | NON_NULL, (empty)
        nworkers,
        // 3158: nworkers: typeof(_1335) = *mut {l2143} i32
        // 3158: nworkers:   l2143 = UNIQUE | NON_NULL, (empty)
    );
    leavebehind = nconsumers / 4 as libc::c_int;
    // 3160: nconsumers: typeof(_1337) = *mut {l2146} i32
    // 3160: nconsumers:   l2146 = UNIQUE | NON_NULL, (empty)
    // 3160: leavebehind: typeof(_1343) = *mut {l2153} i32
    // 3160: leavebehind:   l2153 = UNIQUE | NON_NULL, (empty)
    msg(
        -(1 as libc::c_int),
        1 as libc::c_int,
        b"will leave behind %d consumers out of %d\0" as *const u8 as *const libc::c_char,
        // 3164: b"will leave be ... _char: typeof(_1349) = *const {l2160} i8
        // 3164: b"will leave be ... _char:   l2160 = UNIQUE | NON_NULL, (empty)
        // 3164: b"will leave be ... st u8: typeof(_1350) = *const {l2162} u8
        // 3164: b"will leave be ... st u8:   l2162 = UNIQUE | NON_NULL, (empty)
        // 3164: b"will leave be ... %d\0": typeof(_1351) = *const {l2164} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 3164: b"will leave be ... %d\0":   l2164 = UNIQUE | NON_NULL, (empty)
        // 3164: b"will leave be ... %d\0": typeof(_1352) = & {l2166} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 3164: b"will leave be ... %d\0":   l2166 = UNIQUE | NON_NULL, FIXED
        // 3164: b"will leave be ... _char: typeof(_1349 = move _1350 as *const i8 (Misc)) = *const {l4101} i8
        // 3164: b"will leave be ... _char:   l4101 = UNIQUE | NON_NULL, (empty)
        // 3164: b"will leave be ... st u8: typeof(_1350 = move _1351 as *const u8 (Pointer(ArrayToPointer))) = *const {l4100} u8
        // 3164: b"will leave be ... st u8:   l4100 = UNIQUE | NON_NULL, (empty)
        // 3164: b"will leave be ... %d\0": typeof(_1352 = const b"will leave behind %d consumers out of %d\x00") = & {l4098} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 3164: b"will leave be ... %d\0":   l4098 = UNIQUE | NON_NULL, (empty)
        // 3164: b"will leave be ... %d\0": typeof(_1351 = &raw const (*_1352)) = *const {l4099} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 3164: b"will leave be ... %d\0":   l4099 = UNIQUE | NON_NULL, (empty)
        leavebehind,
        // 3165: leavebehind: typeof(_1354) = *mut {l2169} i32
        // 3165: leavebehind:   l2169 = UNIQUE | NON_NULL, (empty)
        nconsumers,
        // 3166: nconsumers: typeof(_1356) = *mut {l2172} i32
        // 3166: nconsumers:   l2172 = UNIQUE | NON_NULL, (empty)
    );
    earlyworker = if nworkers > 1 as libc::c_int && cloneworker(1 as libc::c_int) != 0 {
    // 3168: if nworkers > 1 ... ker }: typeof(_1357) = *mut {l2174} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 3168: if nworkers > 1 ... ker }:   l2174 = UNIQUE | NON_NULL, (empty)
    // 3168: nworkers: typeof(_1361) = *mut {l2179} i32
    // 3168: nworkers:   l2179 = UNIQUE | NON_NULL, (empty)
        workers.offset(1 as libc::c_int as isize)
        // 3169: workers: typeof(_1366) = *mut {l2185} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3169: workers:   l2185 = UNIQUE | NON_NULL, (empty)
        // 3169: workers: typeof(_1367) = *mut {l2187} *mut {l2188} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3169: workers:   l2187 = UNIQUE | NON_NULL, (empty)
        // 3169: workers:   l2188 = UNIQUE | NON_NULL, (empty)
    } else {
        0 as *mut Worker
        // 3171: 0 as *mut Worker: typeof(_1357 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l4102} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3171: 0 as *mut Worker:   l4102 = UNIQUE | NON_NULL, (empty)
    };
    if !earlyworker.is_null() {
    // 3173: earlyworker: typeof(_1373) = *mut {l2195} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 3173: earlyworker:   l2195 = UNIQUE | NON_NULL, (empty)
        if pthread_create(
            &mut (*earlyworker).thread,
            // 3175: &mut (*earlywor ... hread: typeof(_1377) = *mut {l2200} u64
            // 3175: &mut (*earlywor ... hread:   l2200 = UNIQUE | NON_NULL, (empty)
            // 3175: &mut (*earlywor ... hread: typeof(_1378) = &mut {l2202} u64
            // 3175: &mut (*earlywor ... hread:   l2202 = UNIQUE | NON_NULL, (empty)
            // 3175: &mut (*earlywor ... hread: typeof(_1378 = &mut ((*_19).1: u64)) = &mut {l4103} u64
            // 3175: &mut (*earlywor ... hread:   l4103 = UNIQUE | NON_NULL, (empty)
            // 3175: &mut (*earlywor ... hread: typeof(_1377 = &raw mut (*_1378)) = *mut {l4104} u64
            // 3175: &mut (*earlywor ... hread:   l4104 = UNIQUE | NON_NULL, (empty)
            0 as *const pthread_attr_t,
            // 3176: 0 as *const pth ... ttr_t: typeof(_1379) = *const {l2204} DefId(0:510 ~ plingeling[18f5]::pthread_attr_t)
            // 3176: 0 as *const pth ... ttr_t:   l2204 = UNIQUE | NON_NULL, (empty)
            // 3176: 0 as *const pth ... ttr_t: typeof(_1379 = const 0_usize as *const pthread_attr_t (PointerFromExposedAddress)) = *const {l4105} DefId(0:510 ~ plingeling[18f5]::pthread_attr_t)
            // 3176: 0 as *const pth ... ttr_t:   l4105 = UNIQUE | NON_NULL, (empty)
            Some(work as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
            // 3177: Some(work as un ... void): typeof(_1380) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l2206} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> *mut {l2207} DefId(2:5583 ~ core[480a]::ffi::c_void)>
            // 3177: Some(work as un ... void):   l2206 = UNIQUE | NON_NULL, (empty)
            // 3177: Some(work as un ... void):   l2207 = UNIQUE | NON_NULL, (empty)
            // 3177: work as unsafe  ... _void: typeof(_1381) = fn(*mut {l2209} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> *mut {l2210} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3177: work as unsafe  ... _void:   l2209 = UNIQUE | NON_NULL, (empty)
            // 3177: work as unsafe  ... _void:   l2210 = UNIQUE | NON_NULL, (empty)
            // 3177: work: typeof(_1381 = work as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l4106} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> *mut {l4107} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3177: work:   l4106 = UNIQUE | NON_NULL, (empty)
            // 3177: work:   l4107 = UNIQUE | NON_NULL, (empty)
            // 3177: Some(work as un ... void): typeof(_1380 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>::Some(move _1381)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l4108} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> *mut {l4109} DefId(2:5583 ~ core[480a]::ffi::c_void)>
            // 3177: Some(work as un ... void):   l4108 = UNIQUE | NON_NULL, (empty)
            // 3177: Some(work as un ... void):   l4109 = UNIQUE | NON_NULL, (empty)
            earlyworker as *mut libc::c_void,
            // 3178: earlyworker as  ... _void: typeof(_1382) = *mut {l2212} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3178: earlyworker as  ... _void:   l2212 = UNIQUE | NON_NULL, (empty)
            // 3178: earlyworker: typeof(_1383) = *mut {l2214} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3178: earlyworker:   l2214 = UNIQUE | NON_NULL, (empty)
            // 3178: earlyworker as  ... _void: typeof(_1382 = move _1383 as *mut libc::c_void (Misc)) = *mut {l4110} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3178: earlyworker as  ... _void:   l4110 = UNIQUE | NON_NULL, (empty)
        ) != 0
        {
            die(
                b"failed to create additional early worker thread 1\0" as *const u8
                // 3182: b"failed to cre ... _char: typeof(_1385) = *const {l2217} i8
                // 3182: b"failed to cre ... _char:   l2217 = UNIQUE | NON_NULL, (empty)
                // 3182: b"failed to cre ... st u8: typeof(_1386) = *const {l2219} u8
                // 3182: b"failed to cre ... st u8:   l2219 = UNIQUE | NON_NULL, (empty)
                // 3182: b"failed to cre ...  1\0": typeof(_1387) = *const {l2221} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
                // 3182: b"failed to cre ...  1\0":   l2221 = UNIQUE | NON_NULL, (empty)
                // 3182: b"failed to cre ...  1\0": typeof(_1388) = & {l2223} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
                // 3182: b"failed to cre ...  1\0":   l2223 = UNIQUE | NON_NULL, FIXED
                // 3182: b"failed to cre ... _char: typeof(_1385 = move _1386 as *const i8 (Misc)) = *const {l4114} i8
                // 3182: b"failed to cre ... _char:   l4114 = UNIQUE | NON_NULL, (empty)
                // 3182: b"failed to cre ... st u8: typeof(_1386 = move _1387 as *const u8 (Pointer(ArrayToPointer))) = *const {l4113} u8
                // 3182: b"failed to cre ... st u8:   l4113 = UNIQUE | NON_NULL, (empty)
                // 3182: b"failed to cre ...  1\0": typeof(_1388 = const b"failed to create additional early worker thread 1\x00") = & {l4111} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
                // 3182: b"failed to cre ...  1\0":   l4111 = UNIQUE | NON_NULL, (empty)
                // 3182: b"failed to cre ...  1\0": typeof(_1387 = &raw const (*_1388)) = *const {l4112} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
                // 3182: b"failed to cre ...  1\0":   l4112 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
        }
        msg(
            -(1 as libc::c_int),
            2 as libc::c_int,
            b"started additional early worker 1\0" as *const u8 as *const libc::c_char,
            // 3189: b"started addit ... _char: typeof(_1394) = *const {l2230} i8
            // 3189: b"started addit ... _char:   l2230 = UNIQUE | NON_NULL, (empty)
            // 3189: b"started addit ... st u8: typeof(_1395) = *const {l2232} u8
            // 3189: b"started addit ... st u8:   l2232 = UNIQUE | NON_NULL, (empty)
            // 3189: b"started addit ...  1\0": typeof(_1396) = *const {l2234} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
            // 3189: b"started addit ...  1\0":   l2234 = UNIQUE | NON_NULL, (empty)
            // 3189: b"started addit ...  1\0": typeof(_1397) = & {l2236} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
            // 3189: b"started addit ...  1\0":   l2236 = UNIQUE | NON_NULL, FIXED
            // 3189: b"started addit ...  1\0": typeof(_1396 = &raw const (*_1397)) = *const {l4116} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
            // 3189: b"started addit ...  1\0":   l4116 = UNIQUE | NON_NULL, (empty)
            // 3189: b"started addit ... _char: typeof(_1394 = move _1395 as *const i8 (Misc)) = *const {l4118} i8
            // 3189: b"started addit ... _char:   l4118 = UNIQUE | NON_NULL, (empty)
            // 3189: b"started addit ... st u8: typeof(_1395 = move _1396 as *const u8 (Pointer(ArrayToPointer))) = *const {l4117} u8
            // 3189: b"started addit ... st u8:   l4117 = UNIQUE | NON_NULL, (empty)
            // 3189: b"started addit ...  1\0": typeof(_1397 = const b"started additional early worker 1\x00") = & {l4115} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
            // 3189: b"started addit ...  1\0":   l4115 = UNIQUE | NON_NULL, (empty)
        );
    }
    if locs == 0 {
    // 3192: locs: typeof(_1401) = *mut {l2241} i32
    // 3192: locs:   l2241 = UNIQUE | NON_NULL, (empty)
        msg(
            -(1 as libc::c_int),
            0 as libc::c_int,
            b"simplifying original formula with worker 0\0" as *const u8 as *const libc::c_char,
            // 3196: b"simplifying o ... _char: typeof(_1407) = *const {l2248} i8
            // 3196: b"simplifying o ... _char:   l2248 = UNIQUE | NON_NULL, (empty)
            // 3196: b"simplifying o ... st u8: typeof(_1408) = *const {l2250} u8
            // 3196: b"simplifying o ... st u8:   l2250 = UNIQUE | NON_NULL, (empty)
            // 3196: b"simplifying o ...  0\0": typeof(_1409) = *const {l2252} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
            // 3196: b"simplifying o ...  0\0":   l2252 = UNIQUE | NON_NULL, (empty)
            // 3196: b"simplifying o ...  0\0": typeof(_1410) = & {l2254} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
            // 3196: b"simplifying o ...  0\0":   l2254 = UNIQUE | NON_NULL, FIXED
            // 3196: b"simplifying o ... st u8: typeof(_1408 = move _1409 as *const u8 (Pointer(ArrayToPointer))) = *const {l4121} u8
            // 3196: b"simplifying o ... st u8:   l4121 = UNIQUE | NON_NULL, (empty)
            // 3196: b"simplifying o ...  0\0": typeof(_1409 = &raw const (*_1410)) = *const {l4120} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
            // 3196: b"simplifying o ...  0\0":   l4120 = UNIQUE | NON_NULL, (empty)
            // 3196: b"simplifying o ... _char: typeof(_1407 = move _1408 as *const i8 (Misc)) = *const {l4122} i8
            // 3196: b"simplifying o ... _char:   l4122 = UNIQUE | NON_NULL, (empty)
            // 3196: b"simplifying o ...  0\0": typeof(_1410 = const b"simplifying original formula with worker 0\x00") = & {l4119} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
            // 3196: b"simplifying o ...  0\0":   l4119 = UNIQUE | NON_NULL, (empty)
        );
        lglsetopt(
            (*workers.offset(0 as libc::c_int as isize)).lgl,
            // 3199: (*workers.offse ... ).lgl: typeof(_1412) = *mut {l2257} LGL
            // 3199: (*workers.offse ... ).lgl:   l2257 = UNIQUE | NON_NULL, (empty)
            // 3199: workers.offset( ... size): typeof(_1413) = *mut {l2259} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3199: workers.offset( ... size):   l2259 = UNIQUE | NON_NULL, (empty)
            // 3199: workers: typeof(_1414) = *mut {l2261} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3199: workers:   l2261 = UNIQUE | NON_NULL, (empty)
            // 3199: workers: typeof(_1415) = *mut {l2263} *mut {l2264} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3199: workers:   l2263 = UNIQUE | NON_NULL, (empty)
            // 3199: workers:   l2264 = UNIQUE | NON_NULL, (empty)
            b"clim\0" as *const u8 as *const libc::c_char,
            // 3200: b"clim\0" as *c ... _char: typeof(_1418) = *const {l2268} i8
            // 3200: b"clim\0" as *c ... _char:   l2268 = UNIQUE | NON_NULL, (empty)
            // 3200: b"clim\0" as *c ... st u8: typeof(_1419) = *const {l2270} u8
            // 3200: b"clim\0" as *c ... st u8:   l2270 = UNIQUE | NON_NULL, (empty)
            // 3200: b"clim\0": typeof(_1420) = *const {l2272} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3200: b"clim\0":   l2272 = UNIQUE | NON_NULL, (empty)
            // 3200: b"clim\0": typeof(_1421) = & {l2274} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3200: b"clim\0":   l2274 = UNIQUE | NON_NULL, FIXED
            // 3200: b"clim\0": typeof(_1421 = const b"clim\x00") = & {l4123} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3200: b"clim\0":   l4123 = UNIQUE | NON_NULL, (empty)
            // 3200: b"clim\0": typeof(_1420 = &raw const (*_1421)) = *const {l4124} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3200: b"clim\0":   l4124 = UNIQUE | NON_NULL, (empty)
            // 3200: b"clim\0" as *c ... _char: typeof(_1418 = move _1419 as *const i8 (Misc)) = *const {l4126} i8
            // 3200: b"clim\0" as *c ... _char:   l4126 = UNIQUE | NON_NULL, (empty)
            // 3200: b"clim\0" as *c ... st u8: typeof(_1419 = move _1420 as *const u8 (Pointer(ArrayToPointer))) = *const {l4125} u8
            // 3200: b"clim\0" as *c ... st u8:   l4125 = UNIQUE | NON_NULL, (empty)
            0 as libc::c_int,
        );
        work(workers.offset(0 as libc::c_int as isize) as *mut libc::c_void);
        // 3203: work(workers.of ... void): typeof(_1423) = *mut {l2277} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 3203: work(workers.of ... void):   l2277 = UNIQUE | NON_NULL, (empty)
        // 3203: workers.offset( ... _void: typeof(_1424) = *mut {l2279} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 3203: workers.offset( ... _void:   l2279 = UNIQUE | NON_NULL, (empty)
        // 3203: workers.offset( ... size): typeof(_1425) = *mut {l2281} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3203: workers.offset( ... size):   l2281 = UNIQUE | NON_NULL, (empty)
        // 3203: workers: typeof(_1426) = *mut {l2283} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3203: workers:   l2283 = UNIQUE | NON_NULL, (empty)
        // 3203: workers: typeof(_1427) = *mut {l2285} *mut {l2286} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3203: workers:   l2285 = UNIQUE | NON_NULL, (empty)
        // 3203: workers:   l2286 = UNIQUE | NON_NULL, (empty)
        // 3203: workers.offset( ... _void: typeof(_1424 = move _1425 as *mut libc::c_void (Misc)) = *mut {l4127} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 3203: workers.offset( ... _void:   l4127 = UNIQUE | NON_NULL, (empty)
        lglsetopt(
            (*workers.offset(0 as libc::c_int as isize)).lgl,
            // 3205: (*workers.offse ... ).lgl: typeof(_1431) = *mut {l2291} LGL
            // 3205: (*workers.offse ... ).lgl:   l2291 = UNIQUE | NON_NULL, (empty)
            // 3205: workers.offset( ... size): typeof(_1432) = *mut {l2293} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3205: workers.offset( ... size):   l2293 = UNIQUE | NON_NULL, (empty)
            // 3205: workers: typeof(_1433) = *mut {l2295} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3205: workers:   l2295 = UNIQUE | NON_NULL, (empty)
            // 3205: workers: typeof(_1434) = *mut {l2297} *mut {l2298} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3205: workers:   l2297 = UNIQUE | NON_NULL, (empty)
            // 3205: workers:   l2298 = UNIQUE | NON_NULL, (empty)
            b"clim\0" as *const u8 as *const libc::c_char,
            // 3206: b"clim\0" as *c ... _char: typeof(_1437) = *const {l2302} i8
            // 3206: b"clim\0" as *c ... _char:   l2302 = UNIQUE | NON_NULL, (empty)
            // 3206: b"clim\0" as *c ... st u8: typeof(_1438) = *const {l2304} u8
            // 3206: b"clim\0" as *c ... st u8:   l2304 = UNIQUE | NON_NULL, (empty)
            // 3206: b"clim\0": typeof(_1439) = *const {l2306} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3206: b"clim\0":   l2306 = UNIQUE | NON_NULL, (empty)
            // 3206: b"clim\0": typeof(_1440) = & {l2308} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3206: b"clim\0":   l2308 = UNIQUE | NON_NULL, FIXED
            // 3206: b"clim\0": typeof(_1439 = &raw const (*_1440)) = *const {l4129} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3206: b"clim\0":   l4129 = UNIQUE | NON_NULL, (empty)
            // 3206: b"clim\0" as *c ... _char: typeof(_1437 = move _1438 as *const i8 (Misc)) = *const {l4131} i8
            // 3206: b"clim\0" as *c ... _char:   l4131 = UNIQUE | NON_NULL, (empty)
            // 3206: b"clim\0" as *c ... st u8: typeof(_1438 = move _1439 as *const u8 (Pointer(ArrayToPointer))) = *const {l4130} u8
            // 3206: b"clim\0" as *c ... st u8:   l4130 = UNIQUE | NON_NULL, (empty)
            // 3206: b"clim\0": typeof(_1440 = const b"clim\x00") = & {l4128} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3206: b"clim\0":   l4128 = UNIQUE | NON_NULL, (empty)
            -(1 as libc::c_int),
        );
    }
    res = (*workers.offset(0 as libc::c_int as isize)).res;
    // 3210: workers.offset( ... size): typeof(_1445) = *mut {l2314} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 3210: workers.offset( ... size):   l2314 = UNIQUE | NON_NULL, (empty)
    // 3210: workers: typeof(_1446) = *mut {l2316} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 3210: workers:   l2316 = UNIQUE | NON_NULL, (empty)
    // 3210: workers: typeof(_1447) = *mut {l2318} *mut {l2319} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 3210: workers:   l2318 = UNIQUE | NON_NULL, (empty)
    // 3210: workers:   l2319 = UNIQUE | NON_NULL, (empty)
    if res != 0 {
        msg(
            -(1 as libc::c_int),
            1 as libc::c_int,
            b"simplification of first worker 0 produced %d\0" as *const u8 as *const libc::c_char,
            // 3215: b"simplificatio ... _char: typeof(_1458) = *const {l2331} i8
            // 3215: b"simplificatio ... _char:   l2331 = UNIQUE | NON_NULL, (empty)
            // 3215: b"simplificatio ... st u8: typeof(_1459) = *const {l2333} u8
            // 3215: b"simplificatio ... st u8:   l2333 = UNIQUE | NON_NULL, (empty)
            // 3215: b"simplificatio ... %d\0": typeof(_1460) = *const {l2335} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
            // 3215: b"simplificatio ... %d\0":   l2335 = UNIQUE | NON_NULL, (empty)
            // 3215: b"simplificatio ... %d\0": typeof(_1461) = & {l2337} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
            // 3215: b"simplificatio ... %d\0":   l2337 = UNIQUE | NON_NULL, FIXED
            // 3215: b"simplificatio ... %d\0": typeof(_1460 = &raw const (*_1461)) = *const {l4133} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
            // 3215: b"simplificatio ... %d\0":   l4133 = UNIQUE | NON_NULL, (empty)
            // 3215: b"simplificatio ... st u8: typeof(_1459 = move _1460 as *const u8 (Pointer(ArrayToPointer))) = *const {l4134} u8
            // 3215: b"simplificatio ... st u8:   l4134 = UNIQUE | NON_NULL, (empty)
            // 3215: b"simplificatio ... _char: typeof(_1458 = move _1459 as *const i8 (Misc)) = *const {l4135} i8
            // 3215: b"simplificatio ... _char:   l4135 = UNIQUE | NON_NULL, (empty)
            // 3215: b"simplificatio ... %d\0": typeof(_1461 = const b"simplification of first worker 0 produced %d\x00") = & {l4132} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
            // 3215: b"simplificatio ... %d\0":   l4132 = UNIQUE | NON_NULL, (empty)
            res,
        );
        if !earlyworker.is_null() {
        // 3218: earlyworker: typeof(_1465) = *mut {l2342} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3218: earlyworker:   l2342 = UNIQUE | NON_NULL, (empty)
            if pthread_join((*earlyworker).thread, 0 as *mut *mut libc::c_void) != 0 {
            // 3219: 0 as *mut *mut  ... _void: typeof(_1470) = *mut {l2348} *mut {l2349} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3219: 0 as *mut *mut  ... _void:   l2348 = UNIQUE | NON_NULL, (empty)
            // 3219: 0 as *mut *mut  ... _void:   l2349 = UNIQUE | NON_NULL, (empty)
            // 3219: 0 as *mut *mut  ... _void: typeof(_1470 = const 0_usize as *mut *mut libc::c_void (PointerFromExposedAddress)) = *mut {l4136} *mut {l4137} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3219: 0 as *mut *mut  ... _void:   l4136 = UNIQUE | NON_NULL, (empty)
            // 3219: 0 as *mut *mut  ... _void:   l4137 = UNIQUE | NON_NULL, (empty)
                die(b"failed to join early worker thread 1\0" as *const u8 as *const libc::c_char);
                // 3220: b"failed to joi ... _char: typeof(_1472) = *const {l2352} i8
                // 3220: b"failed to joi ... _char:   l2352 = UNIQUE | NON_NULL, (empty)
                // 3220: b"failed to joi ... st u8: typeof(_1473) = *const {l2354} u8
                // 3220: b"failed to joi ... st u8:   l2354 = UNIQUE | NON_NULL, (empty)
                // 3220: b"failed to joi ...  1\0": typeof(_1474) = *const {l2356} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
                // 3220: b"failed to joi ...  1\0":   l2356 = UNIQUE | NON_NULL, (empty)
                // 3220: b"failed to joi ...  1\0": typeof(_1475) = & {l2358} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
                // 3220: b"failed to joi ...  1\0":   l2358 = UNIQUE | NON_NULL, FIXED
                // 3220: b"failed to joi ... _char: typeof(_1472 = move _1473 as *const i8 (Misc)) = *const {l4141} i8
                // 3220: b"failed to joi ... _char:   l4141 = UNIQUE | NON_NULL, (empty)
                // 3220: b"failed to joi ...  1\0": typeof(_1475 = const b"failed to join early worker thread 1\x00") = & {l4138} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
                // 3220: b"failed to joi ...  1\0":   l4138 = UNIQUE | NON_NULL, (empty)
                // 3220: b"failed to joi ... st u8: typeof(_1473 = move _1474 as *const u8 (Pointer(ArrayToPointer))) = *const {l4140} u8
                // 3220: b"failed to joi ... st u8:   l4140 = UNIQUE | NON_NULL, (empty)
                // 3220: b"failed to joi ...  1\0": typeof(_1474 = &raw const (*_1475)) = *const {l4139} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
                // 3220: b"failed to joi ...  1\0":   l4139 = UNIQUE | NON_NULL, (empty)
            }
            msg(
                -(1 as libc::c_int),
                2 as libc::c_int,
                b"joined early worker 1\0" as *const u8 as *const libc::c_char,
                // 3225: b"joined early  ... _char: typeof(_1481) = *const {l2365} i8
                // 3225: b"joined early  ... _char:   l2365 = UNIQUE | NON_NULL, (empty)
                // 3225: b"joined early  ... st u8: typeof(_1482) = *const {l2367} u8
                // 3225: b"joined early  ... st u8:   l2367 = UNIQUE | NON_NULL, (empty)
                // 3225: b"joined early  ...  1\0": typeof(_1483) = *const {l2369} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 3225: b"joined early  ...  1\0":   l2369 = UNIQUE | NON_NULL, (empty)
                // 3225: b"joined early  ...  1\0": typeof(_1484) = & {l2371} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 3225: b"joined early  ...  1\0":   l2371 = UNIQUE | NON_NULL, FIXED
                // 3225: b"joined early  ... _char: typeof(_1481 = move _1482 as *const i8 (Misc)) = *const {l4145} i8
                // 3225: b"joined early  ... _char:   l4145 = UNIQUE | NON_NULL, (empty)
                // 3225: b"joined early  ...  1\0": typeof(_1483 = &raw const (*_1484)) = *const {l4143} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 3225: b"joined early  ...  1\0":   l4143 = UNIQUE | NON_NULL, (empty)
                // 3225: b"joined early  ...  1\0": typeof(_1484 = const b"joined early worker 1\x00") = & {l4142} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 3225: b"joined early  ...  1\0":   l4142 = UNIQUE | NON_NULL, (empty)
                // 3225: b"joined early  ... st u8: typeof(_1482 = move _1483 as *const u8 (Pointer(ArrayToPointer))) = *const {l4144} u8
                // 3225: b"joined early  ... st u8:   l4144 = UNIQUE | NON_NULL, (empty)
            );
        }
    } else {
        tobecloned = nworkers - 2 as libc::c_int + earlyworker.is_null() as libc::c_int;
        // 3229: nworkers: typeof(_1487) = *mut {l2375} i32
        // 3229: nworkers:   l2375 = UNIQUE | NON_NULL, (empty)
        // 3229: earlyworker: typeof(_1492) = *mut {l2381} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3229: earlyworker:   l2381 = UNIQUE | NON_NULL, (empty)
        if tobecloned > 0 as libc::c_int {
            msg(
                -(1 as libc::c_int),
                0 as libc::c_int,
                b"trying to clone %d workers\0" as *const u8 as *const libc::c_char,
                // 3234: b"trying to clo ... _char: typeof(_1503) = *const {l2393} i8
                // 3234: b"trying to clo ... _char:   l2393 = UNIQUE | NON_NULL, (empty)
                // 3234: b"trying to clo ... st u8: typeof(_1504) = *const {l2395} u8
                // 3234: b"trying to clo ... st u8:   l2395 = UNIQUE | NON_NULL, (empty)
                // 3234: b"trying to clo ... rs\0": typeof(_1505) = *const {l2397} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                // 3234: b"trying to clo ... rs\0":   l2397 = UNIQUE | NON_NULL, (empty)
                // 3234: b"trying to clo ... rs\0": typeof(_1506) = & {l2399} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                // 3234: b"trying to clo ... rs\0":   l2399 = UNIQUE | NON_NULL, FIXED
                // 3234: b"trying to clo ... rs\0": typeof(_1505 = &raw const (*_1506)) = *const {l4147} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                // 3234: b"trying to clo ... rs\0":   l4147 = UNIQUE | NON_NULL, (empty)
                // 3234: b"trying to clo ... rs\0": typeof(_1506 = const b"trying to clone %d workers\x00") = & {l4146} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                // 3234: b"trying to clo ... rs\0":   l4146 = UNIQUE | NON_NULL, (empty)
                // 3234: b"trying to clo ... st u8: typeof(_1504 = move _1505 as *const u8 (Pointer(ArrayToPointer))) = *const {l4148} u8
                // 3234: b"trying to clo ... st u8:   l4148 = UNIQUE | NON_NULL, (empty)
                // 3234: b"trying to clo ... _char: typeof(_1503 = move _1504 as *const i8 (Misc)) = *const {l4149} i8
                // 3234: b"trying to clo ... _char:   l4149 = UNIQUE | NON_NULL, (empty)
                tobecloned,
            );
            i = 1 as libc::c_int;
            while i < nworkers {
            // 3238: nworkers: typeof(_1512) = *mut {l2406} i32
            // 3238: nworkers:   l2406 = UNIQUE | NON_NULL, (empty)
                if earlyworker != workers.offset(i as isize) && cloneworker(i) == 0 {
                // 3239: earlyworker: typeof(_1516) = *mut {l2411} DefId(0:535 ~ plingeling[18f5]::Worker)
                // 3239: earlyworker:   l2411 = UNIQUE | NON_NULL, (empty)
                // 3239: workers.offset( ... size): typeof(_1517) = *mut {l2413} DefId(0:535 ~ plingeling[18f5]::Worker)
                // 3239: workers.offset( ... size):   l2413 = UNIQUE | NON_NULL, (empty)
                // 3239: workers: typeof(_1518) = *mut {l2415} DefId(0:535 ~ plingeling[18f5]::Worker)
                // 3239: workers:   l2415 = UNIQUE | NON_NULL, (empty)
                // 3239: workers: typeof(_1519) = *mut {l2417} *mut {l2418} DefId(0:535 ~ plingeling[18f5]::Worker)
                // 3239: workers:   l2417 = UNIQUE | NON_NULL, (empty)
                // 3239: workers:   l2418 = UNIQUE | NON_NULL, (empty)
                    break;
                }
                i += 1;
                i;
            }
        }
        tobestarted = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < nworkers {
        // 3248: nworkers: typeof(_1537) = *mut {l2437} i32
        // 3248: nworkers:   l2437 = UNIQUE | NON_NULL, (empty)
            if workers.offset(i as isize) != earlyworker {
            // 3249: workers.offset( ... size): typeof(_1540) = *mut {l2441} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3249: workers.offset( ... size):   l2441 = UNIQUE | NON_NULL, (empty)
            // 3249: workers: typeof(_1541) = *mut {l2443} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3249: workers:   l2443 = UNIQUE | NON_NULL, (empty)
            // 3249: workers: typeof(_1542) = *mut {l2445} *mut {l2446} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3249: workers:   l2445 = UNIQUE | NON_NULL, (empty)
            // 3249: workers:   l2446 = UNIQUE | NON_NULL, (empty)
            // 3249: earlyworker: typeof(_1545) = *mut {l2450} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3249: earlyworker:   l2450 = UNIQUE | NON_NULL, (empty)
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
            // 3259: b"starting %d w ... _char: typeof(_1558) = *const {l2464} i8
            // 3259: b"starting %d w ... _char:   l2464 = UNIQUE | NON_NULL, (empty)
            // 3259: b"starting %d w ... st u8: typeof(_1559) = *const {l2466} u8
            // 3259: b"starting %d w ... st u8:   l2466 = UNIQUE | NON_NULL, (empty)
            // 3259: b"starting %d w ... ds\0": typeof(_1560) = *const {l2468} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
            // 3259: b"starting %d w ... ds\0":   l2468 = UNIQUE | NON_NULL, (empty)
            // 3259: b"starting %d w ... ds\0": typeof(_1561) = & {l2470} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
            // 3259: b"starting %d w ... ds\0":   l2470 = UNIQUE | NON_NULL, FIXED
            // 3259: b"starting %d w ... _char: typeof(_1558 = move _1559 as *const i8 (Misc)) = *const {l4153} i8
            // 3259: b"starting %d w ... _char:   l4153 = UNIQUE | NON_NULL, (empty)
            // 3259: b"starting %d w ... ds\0": typeof(_1560 = &raw const (*_1561)) = *const {l4151} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
            // 3259: b"starting %d w ... ds\0":   l4151 = UNIQUE | NON_NULL, (empty)
            // 3259: b"starting %d w ... ds\0": typeof(_1561 = const b"starting %d worker threads\x00") = & {l4150} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
            // 3259: b"starting %d w ... ds\0":   l4150 = UNIQUE | NON_NULL, (empty)
            // 3259: b"starting %d w ... st u8: typeof(_1559 = move _1560 as *const u8 (Pointer(ArrayToPointer))) = *const {l4152} u8
            // 3259: b"starting %d w ... st u8:   l4152 = UNIQUE | NON_NULL, (empty)
            tobestarted,
        );
        i = 0 as libc::c_int;
        while i < nworkers {
        // 3263: nworkers: typeof(_1568) = *mut {l2478} i32
        // 3263: nworkers:   l2478 = UNIQUE | NON_NULL, (empty)
            w = workers.offset(i as isize);
            // 3264: workers.offset( ... size): typeof(_1569) = *mut {l2480} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3264: workers.offset( ... size):   l2480 = UNIQUE | NON_NULL, (empty)
            // 3264: workers: typeof(_1570) = *mut {l2482} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3264: workers:   l2482 = UNIQUE | NON_NULL, (empty)
            // 3264: workers: typeof(_1571) = *mut {l2484} *mut {l2485} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3264: workers:   l2484 = UNIQUE | NON_NULL, (empty)
            // 3264: workers:   l2485 = UNIQUE | NON_NULL, (empty)
            if !(w == earlyworker) {
            // 3265: w: typeof(_1577) = *mut {l2492} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3265: w:   l2492 = UNIQUE | NON_NULL, (empty)
            // 3265: earlyworker: typeof(_1578) = *mut {l2494} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3265: earlyworker:   l2494 = UNIQUE | NON_NULL, (empty)
                if !((*w).lgl).is_null() {
                // 3266: ((*w).lgl): typeof(_1581) = *mut {l2498} LGL
                // 3266: ((*w).lgl):   l2498 = UNIQUE | NON_NULL, (empty)
                    if pthread_create(
                        &mut (*w).thread,
                        // 3268: &mut (*w).thread: typeof(_1585) = *mut {l2503} u64
                        // 3268: &mut (*w).thread:   l2503 = UNIQUE | NON_NULL, (empty)
                        // 3268: &mut (*w).thread: typeof(_1586) = &mut {l2505} u64
                        // 3268: &mut (*w).thread:   l2505 = UNIQUE | NON_NULL, (empty)
                        // 3268: &mut (*w).thread: typeof(_1585 = &raw mut (*_1586)) = *mut {l4155} u64
                        // 3268: &mut (*w).thread:   l4155 = UNIQUE | NON_NULL, (empty)
                        // 3268: &mut (*w).thread: typeof(_1586 = &mut ((*_14).1: u64)) = &mut {l4154} u64
                        // 3268: &mut (*w).thread:   l4154 = UNIQUE | NON_NULL, (empty)
                        0 as *const pthread_attr_t,
                        // 3269: 0 as *const pth ... ttr_t: typeof(_1587) = *const {l2507} DefId(0:510 ~ plingeling[18f5]::pthread_attr_t)
                        // 3269: 0 as *const pth ... ttr_t:   l2507 = UNIQUE | NON_NULL, (empty)
                        // 3269: 0 as *const pth ... ttr_t: typeof(_1587 = const 0_usize as *const pthread_attr_t (PointerFromExposedAddress)) = *const {l4156} DefId(0:510 ~ plingeling[18f5]::pthread_attr_t)
                        // 3269: 0 as *const pth ... ttr_t:   l4156 = UNIQUE | NON_NULL, (empty)
                        Some(work as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
                        // 3270: Some(work as un ... void): typeof(_1588) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l2509} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> *mut {l2510} DefId(2:5583 ~ core[480a]::ffi::c_void)>
                        // 3270: Some(work as un ... void):   l2509 = UNIQUE | NON_NULL, (empty)
                        // 3270: Some(work as un ... void):   l2510 = UNIQUE | NON_NULL, (empty)
                        // 3270: work as unsafe  ... _void: typeof(_1589) = fn(*mut {l2512} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> *mut {l2513} DefId(2:5583 ~ core[480a]::ffi::c_void)
                        // 3270: work as unsafe  ... _void:   l2512 = UNIQUE | NON_NULL, (empty)
                        // 3270: work as unsafe  ... _void:   l2513 = UNIQUE | NON_NULL, (empty)
                        // 3270: work: typeof(_1589 = work as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l4157} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> *mut {l4158} DefId(2:5583 ~ core[480a]::ffi::c_void)
                        // 3270: work:   l4157 = UNIQUE | NON_NULL, (empty)
                        // 3270: work:   l4158 = UNIQUE | NON_NULL, (empty)
                        // 3270: Some(work as un ... void): typeof(_1588 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>::Some(move _1589)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l4159} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> *mut {l4160} DefId(2:5583 ~ core[480a]::ffi::c_void)>
                        // 3270: Some(work as un ... void):   l4159 = UNIQUE | NON_NULL, (empty)
                        // 3270: Some(work as un ... void):   l4160 = UNIQUE | NON_NULL, (empty)
                        w as *mut libc::c_void,
                        // 3271: w as *mut libc: ... _void: typeof(_1590) = *mut {l2515} DefId(2:5583 ~ core[480a]::ffi::c_void)
                        // 3271: w as *mut libc: ... _void:   l2515 = UNIQUE | NON_NULL, (empty)
                        // 3271: w: typeof(_1591) = *mut {l2517} DefId(0:535 ~ plingeling[18f5]::Worker)
                        // 3271: w:   l2517 = UNIQUE | NON_NULL, (empty)
                        // 3271: w as *mut libc: ... _void: typeof(_1590 = move _1591 as *mut libc::c_void (Misc)) = *mut {l4161} DefId(2:5583 ~ core[480a]::ffi::c_void)
                        // 3271: w as *mut libc: ... _void:   l4161 = UNIQUE | NON_NULL, (empty)
                    ) != 0
                    {
                        die(
                            b"failed to create worker thread %d\0" as *const u8
                            // 3275: b"failed to cre ... _char: typeof(_1593) = *const {l2520} i8
                            // 3275: b"failed to cre ... _char:   l2520 = UNIQUE | NON_NULL, (empty)
                            // 3275: b"failed to cre ... st u8: typeof(_1594) = *const {l2522} u8
                            // 3275: b"failed to cre ... st u8:   l2522 = UNIQUE | NON_NULL, (empty)
                            // 3275: b"failed to cre ... %d\0": typeof(_1595) = *const {l2524} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                            // 3275: b"failed to cre ... %d\0":   l2524 = UNIQUE | NON_NULL, (empty)
                            // 3275: b"failed to cre ... %d\0": typeof(_1596) = & {l2526} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                            // 3275: b"failed to cre ... %d\0":   l2526 = UNIQUE | NON_NULL, FIXED
                            // 3275: b"failed to cre ... st u8: typeof(_1594 = move _1595 as *const u8 (Pointer(ArrayToPointer))) = *const {l4164} u8
                            // 3275: b"failed to cre ... st u8:   l4164 = UNIQUE | NON_NULL, (empty)
                            // 3275: b"failed to cre ... %d\0": typeof(_1595 = &raw const (*_1596)) = *const {l4163} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                            // 3275: b"failed to cre ... %d\0":   l4163 = UNIQUE | NON_NULL, (empty)
                            // 3275: b"failed to cre ... _char: typeof(_1593 = move _1594 as *const i8 (Misc)) = *const {l4165} i8
                            // 3275: b"failed to cre ... _char:   l4165 = UNIQUE | NON_NULL, (empty)
                            // 3275: b"failed to cre ... %d\0": typeof(_1596 = const b"failed to create worker thread %d\x00") = & {l4162} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                            // 3275: b"failed to cre ... %d\0":   l4162 = UNIQUE | NON_NULL, (empty)
                                as *const libc::c_char,
                            i,
                        );
                    }
                    msg(
                        -(1 as libc::c_int),
                        2 as libc::c_int,
                        b"started worker %d\0" as *const u8 as *const libc::c_char,
                        // 3283: b"started worke ... _char: typeof(_1603) = *const {l2534} i8
                        // 3283: b"started worke ... _char:   l2534 = UNIQUE | NON_NULL, (empty)
                        // 3283: b"started worke ... st u8: typeof(_1604) = *const {l2536} u8
                        // 3283: b"started worke ... st u8:   l2536 = UNIQUE | NON_NULL, (empty)
                        // 3283: b"started worke ... %d\0": typeof(_1605) = *const {l2538} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                        // 3283: b"started worke ... %d\0":   l2538 = UNIQUE | NON_NULL, (empty)
                        // 3283: b"started worke ... %d\0": typeof(_1606) = & {l2540} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                        // 3283: b"started worke ... %d\0":   l2540 = UNIQUE | NON_NULL, FIXED
                        // 3283: b"started worke ... _char: typeof(_1603 = move _1604 as *const i8 (Misc)) = *const {l4169} i8
                        // 3283: b"started worke ... _char:   l4169 = UNIQUE | NON_NULL, (empty)
                        // 3283: b"started worke ... %d\0": typeof(_1606 = const b"started worker %d\x00") = & {l4166} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                        // 3283: b"started worke ... %d\0":   l4166 = UNIQUE | NON_NULL, (empty)
                        // 3283: b"started worke ... st u8: typeof(_1604 = move _1605 as *const u8 (Pointer(ArrayToPointer))) = *const {l4168} u8
                        // 3283: b"started worke ... st u8:   l4168 = UNIQUE | NON_NULL, (empty)
                        // 3283: b"started worke ... %d\0": typeof(_1605 = &raw const (*_1606)) = *const {l4167} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                        // 3283: b"started worke ... %d\0":   l4167 = UNIQUE | NON_NULL, (empty)
                        i,
                    );
                }
            }
            i += 1;
            i;
        }
        msg(
            -(1 as libc::c_int),
            2 as libc::c_int,
            b"joining %d workers\0" as *const u8 as *const libc::c_char,
            // 3294: b"joining %d wo ... _char: typeof(_1618) = *const {l2553} i8
            // 3294: b"joining %d wo ... _char:   l2553 = UNIQUE | NON_NULL, (empty)
            // 3294: b"joining %d wo ... st u8: typeof(_1619) = *const {l2555} u8
            // 3294: b"joining %d wo ... st u8:   l2555 = UNIQUE | NON_NULL, (empty)
            // 3294: b"joining %d wo ... rs\0": typeof(_1620) = *const {l2557} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 3294: b"joining %d wo ... rs\0":   l2557 = UNIQUE | NON_NULL, (empty)
            // 3294: b"joining %d wo ... rs\0": typeof(_1621) = & {l2559} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 3294: b"joining %d wo ... rs\0":   l2559 = UNIQUE | NON_NULL, FIXED
            // 3294: b"joining %d wo ... _char: typeof(_1618 = move _1619 as *const i8 (Misc)) = *const {l4173} i8
            // 3294: b"joining %d wo ... _char:   l4173 = UNIQUE | NON_NULL, (empty)
            // 3294: b"joining %d wo ... st u8: typeof(_1619 = move _1620 as *const u8 (Pointer(ArrayToPointer))) = *const {l4172} u8
            // 3294: b"joining %d wo ... st u8:   l4172 = UNIQUE | NON_NULL, (empty)
            // 3294: b"joining %d wo ... rs\0": typeof(_1621 = const b"joining %d workers\x00") = & {l4170} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 3294: b"joining %d wo ... rs\0":   l4170 = UNIQUE | NON_NULL, (empty)
            // 3294: b"joining %d wo ... rs\0": typeof(_1620 = &raw const (*_1621)) = *const {l4171} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 3294: b"joining %d wo ... rs\0":   l4171 = UNIQUE | NON_NULL, (empty)
            tobestarted + 1 as libc::c_int - earlyworker.is_null() as libc::c_int,
            // 3295: earlyworker: typeof(_1629) = *mut {l2568} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3295: earlyworker:   l2568 = UNIQUE | NON_NULL, (empty)
        );
        i = 0 as libc::c_int;
        while i < nworkers {
        // 3298: nworkers: typeof(_1635) = *mut {l2575} i32
        // 3298: nworkers:   l2575 = UNIQUE | NON_NULL, (empty)
            w = workers.offset(i as isize);
            // 3299: workers.offset( ... size): typeof(_1636) = *mut {l2577} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3299: workers.offset( ... size):   l2577 = UNIQUE | NON_NULL, (empty)
            // 3299: workers: typeof(_1637) = *mut {l2579} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3299: workers:   l2579 = UNIQUE | NON_NULL, (empty)
            // 3299: workers: typeof(_1638) = *mut {l2581} *mut {l2582} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3299: workers:   l2581 = UNIQUE | NON_NULL, (empty)
            // 3299: workers:   l2582 = UNIQUE | NON_NULL, (empty)
            if !((*w).lgl).is_null() {
            // 3300: ((*w).lgl): typeof(_1644) = *mut {l2589} LGL
            // 3300: ((*w).lgl):   l2589 = UNIQUE | NON_NULL, (empty)
                if pthread_join((*w).thread, 0 as *mut *mut libc::c_void) != 0 {
                // 3301: 0 as *mut *mut  ... _void: typeof(_1649) = *mut {l2595} *mut {l2596} DefId(2:5583 ~ core[480a]::ffi::c_void)
                // 3301: 0 as *mut *mut  ... _void:   l2595 = UNIQUE | NON_NULL, (empty)
                // 3301: 0 as *mut *mut  ... _void:   l2596 = UNIQUE | NON_NULL, (empty)
                // 3301: 0 as *mut *mut  ... _void: typeof(_1649 = const 0_usize as *mut *mut libc::c_void (PointerFromExposedAddress)) = *mut {l4174} *mut {l4175} DefId(2:5583 ~ core[480a]::ffi::c_void)
                // 3301: 0 as *mut *mut  ... _void:   l4174 = UNIQUE | NON_NULL, (empty)
                // 3301: 0 as *mut *mut  ... _void:   l4175 = UNIQUE | NON_NULL, (empty)
                    die(
                        b"failed to join worker thread %d\0" as *const u8 as *const libc::c_char,
                        // 3303: b"failed to joi ... _char: typeof(_1651) = *const {l2599} i8
                        // 3303: b"failed to joi ... _char:   l2599 = UNIQUE | NON_NULL, (empty)
                        // 3303: b"failed to joi ... st u8: typeof(_1652) = *const {l2601} u8
                        // 3303: b"failed to joi ... st u8:   l2601 = UNIQUE | NON_NULL, (empty)
                        // 3303: b"failed to joi ... %d\0": typeof(_1653) = *const {l2603} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 3303: b"failed to joi ... %d\0":   l2603 = UNIQUE | NON_NULL, (empty)
                        // 3303: b"failed to joi ... %d\0": typeof(_1654) = & {l2605} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 3303: b"failed to joi ... %d\0":   l2605 = UNIQUE | NON_NULL, FIXED
                        // 3303: b"failed to joi ... st u8: typeof(_1652 = move _1653 as *const u8 (Pointer(ArrayToPointer))) = *const {l4178} u8
                        // 3303: b"failed to joi ... st u8:   l4178 = UNIQUE | NON_NULL, (empty)
                        // 3303: b"failed to joi ... _char: typeof(_1651 = move _1652 as *const i8 (Misc)) = *const {l4179} i8
                        // 3303: b"failed to joi ... _char:   l4179 = UNIQUE | NON_NULL, (empty)
                        // 3303: b"failed to joi ... %d\0": typeof(_1654 = const b"failed to join worker thread %d\x00") = & {l4176} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 3303: b"failed to joi ... %d\0":   l4176 = UNIQUE | NON_NULL, (empty)
                        // 3303: b"failed to joi ... %d\0": typeof(_1653 = &raw const (*_1654)) = *const {l4177} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 3303: b"failed to joi ... %d\0":   l4177 = UNIQUE | NON_NULL, (empty)
                        i,
                    );
                }
                msg(
                    -(1 as libc::c_int),
                    2 as libc::c_int,
                    b"joined worker %d\0" as *const u8 as *const libc::c_char,
                    // 3310: b"joined worker ... _char: typeof(_1661) = *const {l2613} i8
                    // 3310: b"joined worker ... _char:   l2613 = UNIQUE | NON_NULL, (empty)
                    // 3310: b"joined worker ... st u8: typeof(_1662) = *const {l2615} u8
                    // 3310: b"joined worker ... st u8:   l2615 = UNIQUE | NON_NULL, (empty)
                    // 3310: b"joined worker ... %d\0": typeof(_1663) = *const {l2617} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                    // 3310: b"joined worker ... %d\0":   l2617 = UNIQUE | NON_NULL, (empty)
                    // 3310: b"joined worker ... %d\0": typeof(_1664) = & {l2619} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                    // 3310: b"joined worker ... %d\0":   l2619 = UNIQUE | NON_NULL, FIXED
                    // 3310: b"joined worker ... st u8: typeof(_1662 = move _1663 as *const u8 (Pointer(ArrayToPointer))) = *const {l4182} u8
                    // 3310: b"joined worker ... st u8:   l4182 = UNIQUE | NON_NULL, (empty)
                    // 3310: b"joined worker ... %d\0": typeof(_1664 = const b"joined worker %d\x00") = & {l4180} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                    // 3310: b"joined worker ... %d\0":   l4180 = UNIQUE | NON_NULL, (empty)
                    // 3310: b"joined worker ... _char: typeof(_1661 = move _1662 as *const i8 (Misc)) = *const {l4183} i8
                    // 3310: b"joined worker ... _char:   l4183 = UNIQUE | NON_NULL, (empty)
                    // 3310: b"joined worker ... %d\0": typeof(_1663 = &raw const (*_1664)) = *const {l4181} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                    // 3310: b"joined worker ... %d\0":   l4181 = UNIQUE | NON_NULL, (empty)
                    i,
                );
            }
            i += 1;
            i;
        }
    }
    winner = 0 as *mut Worker;
    // 3318: winner = 0 as * ... orker: typeof(_15 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l4184} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 3318: winner = 0 as * ... orker:   l4184 = UNIQUE | NON_NULL, (empty)
    maxconsumer = winner;
    // 3319: winner: typeof(_1671) = *mut {l2627} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 3319: winner:   l2627 = UNIQUE | NON_NULL, (empty)
    maxproducer = maxconsumer;
    // 3320: maxconsumer: typeof(_1672) = *mut {l2629} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 3320: maxconsumer:   l2629 = UNIQUE | NON_NULL, (empty)
    i = 0 as libc::c_int;
    while i < nworkers {
    // 3322: nworkers: typeof(_1678) = *mut {l2636} i32
    // 3322: nworkers:   l2636 = UNIQUE | NON_NULL, (empty)
        w = workers.offset(i as isize);
        // 3323: workers.offset( ... size): typeof(_1679) = *mut {l2638} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3323: workers.offset( ... size):   l2638 = UNIQUE | NON_NULL, (empty)
        // 3323: workers: typeof(_1680) = *mut {l2640} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3323: workers:   l2640 = UNIQUE | NON_NULL, (empty)
        // 3323: workers: typeof(_1681) = *mut {l2642} *mut {l2643} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3323: workers:   l2642 = UNIQUE | NON_NULL, (empty)
        // 3323: workers:   l2643 = UNIQUE | NON_NULL, (empty)
        if !((*w).lgl).is_null() {
        // 3324: ((*w).lgl): typeof(_1687) = *mut {l2650} LGL
        // 3324: ((*w).lgl):   l2650 = UNIQUE | NON_NULL, (empty)
            if (*w).res != 0 {
                if res == 0 {
                    res = (*w).res;
                    winner = w;
                    // 3328: w: typeof(_1694) = *mut {l2658} DefId(0:535 ~ plingeling[18f5]::Worker)
                    // 3328: w:   l2658 = UNIQUE | NON_NULL, (empty)
                    msg(
                        -(1 as libc::c_int),
                        0 as libc::c_int,
                        b"worker %d is the WINNER with result %d\0" as *const u8
                        // 3332: b"worker %d is  ... _char: typeof(_1700) = *const {l2665} i8
                        // 3332: b"worker %d is  ... _char:   l2665 = UNIQUE | NON_NULL, (empty)
                        // 3332: b"worker %d is  ... st u8: typeof(_1701) = *const {l2667} u8
                        // 3332: b"worker %d is  ... st u8:   l2667 = UNIQUE | NON_NULL, (empty)
                        // 3332: b"worker %d is  ... %d\0": typeof(_1702) = *const {l2669} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                        // 3332: b"worker %d is  ... %d\0":   l2669 = UNIQUE | NON_NULL, (empty)
                        // 3332: b"worker %d is  ... %d\0": typeof(_1703) = & {l2671} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                        // 3332: b"worker %d is  ... %d\0":   l2671 = UNIQUE | NON_NULL, FIXED
                        // 3332: b"worker %d is  ... %d\0": typeof(_1703 = const b"worker %d is the WINNER with result %d\x00") = & {l4185} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                        // 3332: b"worker %d is  ... %d\0":   l4185 = UNIQUE | NON_NULL, (empty)
                        // 3332: b"worker %d is  ... _char: typeof(_1700 = move _1701 as *const i8 (Misc)) = *const {l4188} i8
                        // 3332: b"worker %d is  ... _char:   l4188 = UNIQUE | NON_NULL, (empty)
                        // 3332: b"worker %d is  ... %d\0": typeof(_1702 = &raw const (*_1703)) = *const {l4186} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                        // 3332: b"worker %d is  ... %d\0":   l4186 = UNIQUE | NON_NULL, (empty)
                        // 3332: b"worker %d is  ... st u8: typeof(_1701 = move _1702 as *const u8 (Pointer(ArrayToPointer))) = *const {l4187} u8
                        // 3332: b"worker %d is  ... st u8:   l4187 = UNIQUE | NON_NULL, (empty)
                            as *const libc::c_char,
                        i,
                        res,
                    );
                } else if res != (*w).res {
                    die(b"result discrepancy\0" as *const u8 as *const libc::c_char);
                    // 3338: b"result discre ... _char: typeof(_1710) = *const {l2679} i8
                    // 3338: b"result discre ... _char:   l2679 = UNIQUE | NON_NULL, (empty)
                    // 3338: b"result discre ... st u8: typeof(_1711) = *const {l2681} u8
                    // 3338: b"result discre ... st u8:   l2681 = UNIQUE | NON_NULL, (empty)
                    // 3338: b"result discre ... cy\0": typeof(_1712) = *const {l2683} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 3338: b"result discre ... cy\0":   l2683 = UNIQUE | NON_NULL, (empty)
                    // 3338: b"result discre ... cy\0": typeof(_1713) = & {l2685} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 3338: b"result discre ... cy\0":   l2685 = UNIQUE | NON_NULL, FIXED
                    // 3338: b"result discre ... st u8: typeof(_1711 = move _1712 as *const u8 (Pointer(ArrayToPointer))) = *const {l4191} u8
                    // 3338: b"result discre ... st u8:   l4191 = UNIQUE | NON_NULL, (empty)
                    // 3338: b"result discre ... _char: typeof(_1710 = move _1711 as *const i8 (Misc)) = *const {l4192} i8
                    // 3338: b"result discre ... _char:   l4192 = UNIQUE | NON_NULL, (empty)
                    // 3338: b"result discre ... cy\0": typeof(_1712 = &raw const (*_1713)) = *const {l4190} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 3338: b"result discre ... cy\0":   l4190 = UNIQUE | NON_NULL, (empty)
                    // 3338: b"result discre ... cy\0": typeof(_1713 = const b"result discrepancy\x00") = & {l4189} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 3338: b"result discre ... cy\0":   l4189 = UNIQUE | NON_NULL, (empty)
                }
            }
            if maxconsumer.is_null() || (*w).stats.consumed > (*maxconsumer).stats.consumed {
            // 3341: maxconsumer: typeof(_1717) = *mut {l2690} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3341: maxconsumer:   l2690 = UNIQUE | NON_NULL, (empty)
                maxconsumer = w;
                // 3342: w: typeof(_1721) = *mut {l2695} DefId(0:535 ~ plingeling[18f5]::Worker)
                // 3342: w:   l2695 = UNIQUE | NON_NULL, (empty)
            }
            if maxproducer.is_null() || (*w).stats.produced > (*maxproducer).stats.produced {
            // 3344: maxproducer: typeof(_1724) = *mut {l2699} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3344: maxproducer:   l2699 = UNIQUE | NON_NULL, (empty)
                maxproducer = w;
                // 3345: w: typeof(_1728) = *mut {l2704} DefId(0:535 ~ plingeling[18f5]::Worker)
                // 3345: w:   l2704 = UNIQUE | NON_NULL, (empty)
            }
        }
        i += 1;
        i;
    }
    let mut BYTES_0: size_t = (nworkers as libc::c_ulong)
    // 3351: nworkers: typeof(_1737) = *mut {l2714} i32
    // 3351: nworkers:   l2714 = UNIQUE | NON_NULL, (empty)
        .wrapping_mul(::core::mem::size_of::<*mut Worker>() as libc::c_ulong);
    sorted = malloc(BYTES_0) as *mut *mut Worker;
    // 3353: malloc(BYTES_0): typeof(_1740) = *mut {l2718} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 3353: malloc(BYTES_0):   l2718 = UNIQUE | NON_NULL, (empty)
    // 3353: sorted = malloc ... orker: typeof(_18 = move _1740 as *mut *mut Worker (Misc)) = *mut {l4193} *mut {l4194} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 3353: sorted = malloc ... orker:   l4193 = UNIQUE | NON_NULL, (empty)
    // 3353: sorted = malloc ... orker:   l4194 = UNIQUE | NON_NULL, (empty)
    if sorted.is_null() {
    // 3354: sorted: typeof(_1744) = *mut {l2723} *mut {l2724} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 3354: sorted:   l2723 = UNIQUE | NON_NULL, (empty)
    // 3354: sorted:   l2724 = UNIQUE | NON_NULL, (empty)
        die(b"out of memory\0" as *const u8 as *const libc::c_char);
        // 3355: b"out of memory ... _char: typeof(_1747) = *const {l2728} i8
        // 3355: b"out of memory ... _char:   l2728 = UNIQUE | NON_NULL, (empty)
        // 3355: b"out of memory ... st u8: typeof(_1748) = *const {l2730} u8
        // 3355: b"out of memory ... st u8:   l2730 = UNIQUE | NON_NULL, (empty)
        // 3355: b"out of memory\0": typeof(_1749) = *const {l2732} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 3355: b"out of memory\0":   l2732 = UNIQUE | NON_NULL, (empty)
        // 3355: b"out of memory\0": typeof(_1750) = & {l2734} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 3355: b"out of memory\0":   l2734 = UNIQUE | NON_NULL, FIXED
        // 3355: b"out of memory\0": typeof(_1749 = &raw const (*_1750)) = *const {l4196} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 3355: b"out of memory\0":   l4196 = UNIQUE | NON_NULL, (empty)
        // 3355: b"out of memory ... _char: typeof(_1747 = move _1748 as *const i8 (Misc)) = *const {l4198} i8
        // 3355: b"out of memory ... _char:   l4198 = UNIQUE | NON_NULL, (empty)
        // 3355: b"out of memory ... st u8: typeof(_1748 = move _1749 as *const u8 (Pointer(ArrayToPointer))) = *const {l4197} u8
        // 3355: b"out of memory ... st u8:   l4197 = UNIQUE | NON_NULL, (empty)
        // 3355: b"out of memory\0": typeof(_1750 = const b"out of memory\x00") = & {l4195} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 3355: b"out of memory\0":   l4195 = UNIQUE | NON_NULL, (empty)
        exit(1 as libc::c_int);
    }
    memset(sorted as *mut libc::c_void, 0 as libc::c_int, BYTES_0);
    // 3358: memset(sorted a ... ES_0): typeof(_1753) = *mut {l2738} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 3358: memset(sorted a ... ES_0):   l2738 = UNIQUE | NON_NULL, (empty)
    // 3358: sorted as *mut  ... _void: typeof(_1754) = *mut {l2740} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 3358: sorted as *mut  ... _void:   l2740 = UNIQUE | NON_NULL, (empty)
    // 3358: sorted: typeof(_1755) = *mut {l2742} *mut {l2743} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 3358: sorted:   l2742 = UNIQUE | NON_NULL, (empty)
    // 3358: sorted:   l2743 = UNIQUE | NON_NULL, (empty)
    // 3358: sorted as *mut  ... _void: typeof(_1754 = move _1755 as *mut libc::c_void (Misc)) = *mut {l4199} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 3358: sorted as *mut  ... _void:   l4199 = UNIQUE | NON_NULL, (empty)
    incmem(BYTES_0);
    i = 0 as libc::c_int;
    while i < nworkers {
    // 3361: nworkers: typeof(_1765) = *mut {l2754} i32
    // 3361: nworkers:   l2754 = UNIQUE | NON_NULL, (empty)
        let ref mut fresh19 = *sorted.offset(i as isize);
        // 3362: ref mut fresh19: typeof(_1766) = &mut {l2756} *mut {l2757} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3362: ref mut fresh19:   l2756 = UNIQUE | NON_NULL, FIXED
        // 3362: ref mut fresh19:   l2757 = UNIQUE | NON_NULL, (empty)
        // 3362: sorted.offset(i ... size): typeof(_1767) = *mut {l2759} *mut {l2760} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3362: sorted.offset(i ... size):   l2759 = UNIQUE | NON_NULL, (empty)
        // 3362: sorted.offset(i ... size):   l2760 = UNIQUE | NON_NULL, (empty)
        // 3362: sorted: typeof(_1768) = *mut {l2762} *mut {l2763} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3362: sorted:   l2762 = UNIQUE | NON_NULL, (empty)
        // 3362: sorted:   l2763 = UNIQUE | NON_NULL, (empty)
        // 3362: ref mut fresh19: typeof(_1766 = &mut (*_1767)) = &mut {l4200} *mut {l4201} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3362: ref mut fresh19:   l4200 = UNIQUE | NON_NULL, (empty)
        // 3362: ref mut fresh19:   l4201 = UNIQUE | NON_NULL, (empty)
        *fresh19 = workers.offset(i as isize);
        // 3363: workers.offset( ... size): typeof(_1771) = *mut {l2767} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3363: workers.offset( ... size):   l2767 = UNIQUE | NON_NULL, (empty)
        // 3363: workers: typeof(_1772) = *mut {l2769} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3363: workers:   l2769 = UNIQUE | NON_NULL, (empty)
        // 3363: workers: typeof(_1773) = *mut {l2771} *mut {l2772} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3363: workers:   l2771 = UNIQUE | NON_NULL, (empty)
        // 3363: workers:   l2772 = UNIQUE | NON_NULL, (empty)
        i += 1;
        i;
    }
    printf(b"c\n\0" as *const u8 as *const libc::c_char);
    // 3367: b"c\n\0" as *co ... _char: typeof(_1782) = *const {l2782} i8
    // 3367: b"c\n\0" as *co ... _char:   l2782 = UNIQUE | NON_NULL, (empty)
    // 3367: b"c\n\0" as *co ... st u8: typeof(_1783) = *const {l2784} u8
    // 3367: b"c\n\0" as *co ... st u8:   l2784 = UNIQUE | NON_NULL, (empty)
    // 3367: b"c\n\0": typeof(_1784) = *const {l2786} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 3367: b"c\n\0":   l2786 = UNIQUE | NON_NULL, (empty)
    // 3367: b"c\n\0": typeof(_1785) = & {l2788} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 3367: b"c\n\0":   l2788 = UNIQUE | NON_NULL, FIXED
    // 3367: b"c\n\0" as *co ... _char: typeof(_1782 = move _1783 as *const i8 (Misc)) = *const {l4205} i8
    // 3367: b"c\n\0" as *co ... _char:   l4205 = UNIQUE | NON_NULL, (empty)
    // 3367: b"c\n\0": typeof(_1784 = &raw const (*_1785)) = *const {l4203} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 3367: b"c\n\0":   l4203 = UNIQUE | NON_NULL, (empty)
    // 3367: b"c\n\0" as *co ... st u8: typeof(_1783 = move _1784 as *const u8 (Pointer(ArrayToPointer))) = *const {l4204} u8
    // 3367: b"c\n\0" as *co ... st u8:   l4204 = UNIQUE | NON_NULL, (empty)
    // 3367: b"c\n\0": typeof(_1785 = const b"c\n\x00") = & {l4202} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 3367: b"c\n\0":   l4202 = UNIQUE | NON_NULL, (empty)
    qsort(
        sorted as *mut libc::c_void,
        // 3369: sorted as *mut  ... _void: typeof(_1787) = *mut {l2791} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 3369: sorted as *mut  ... _void:   l2791 = UNIQUE | NON_NULL, (empty)
        // 3369: sorted: typeof(_1788) = *mut {l2793} *mut {l2794} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3369: sorted:   l2793 = UNIQUE | NON_NULL, (empty)
        // 3369: sorted:   l2794 = UNIQUE | NON_NULL, (empty)
        // 3369: sorted as *mut  ... _void: typeof(_1787 = move _1788 as *mut libc::c_void (Misc)) = *mut {l4206} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 3369: sorted as *mut  ... _void:   l4206 = UNIQUE | NON_NULL, (empty)
        nworkers as size_t,
        // 3370: nworkers: typeof(_1791) = *mut {l2798} i32
        // 3370: nworkers:   l2798 = UNIQUE | NON_NULL, (empty)
        ::core::mem::size_of::<*mut Worker>() as libc::c_ulong,
        Some(
        // 3372: Some( cmproduce ... nt, ): typeof(_1794) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*const {l2802} DefId(2:5583 ~ core[480a]::ffi::c_void), *const {l2803} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32>
        // 3372: Some( cmproduce ... nt, ):   l2802 = UNIQUE | NON_NULL, (empty)
        // 3372: Some( cmproduce ... nt, ):   l2803 = UNIQUE | NON_NULL, (empty)
        // 3372: Some( cmproduce ... nt, ): typeof(_1794 = std::option::Option::<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>::Some(move _1795)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*const {l4209} DefId(2:5583 ~ core[480a]::ffi::c_void), *const {l4210} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32>
        // 3372: Some( cmproduce ... nt, ):   l4209 = UNIQUE | NON_NULL, (empty)
        // 3372: Some( cmproduce ... nt, ):   l4210 = UNIQUE | NON_NULL, (empty)
            cmproduced
            // 3373: cmproduced as u ... c_int: typeof(_1795) = fn(*const {l2805} DefId(2:5583 ~ core[480a]::ffi::c_void), *const {l2806} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32
            // 3373: cmproduced as u ... c_int:   l2805 = UNIQUE | NON_NULL, (empty)
            // 3373: cmproduced as u ... c_int:   l2806 = UNIQUE | NON_NULL, (empty)
            // 3373: cmproduced: typeof(_1795 = cmproduced as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32 (Pointer(ReifyFnPointer))) = fn(*const {l4207} DefId(2:5583 ~ core[480a]::ffi::c_void), *const {l4208} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32
            // 3373: cmproduced:   l4207 = UNIQUE | NON_NULL, (empty)
            // 3373: cmproduced:   l4208 = UNIQUE | NON_NULL, (empty)
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < nworkers {
    // 3378: nworkers: typeof(_1801) = *mut {l2813} i32
    // 3378: nworkers:   l2813 = UNIQUE | NON_NULL, (empty)
        w = *sorted.offset(i as isize);
        // 3379: *sorted.offset( ... size): typeof(_1802) = *mut {l2815} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3379: *sorted.offset( ... size):   l2815 = UNIQUE | NON_NULL, (empty)
        // 3379: sorted.offset(i ... size): typeof(_1803) = *mut {l2817} *mut {l2818} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3379: sorted.offset(i ... size):   l2817 = UNIQUE | NON_NULL, (empty)
        // 3379: sorted.offset(i ... size):   l2818 = UNIQUE | NON_NULL, (empty)
        // 3379: sorted: typeof(_1804) = *mut {l2820} *mut {l2821} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3379: sorted:   l2820 = UNIQUE | NON_NULL, (empty)
        // 3379: sorted:   l2821 = UNIQUE | NON_NULL, (empty)
        if !((*w).lgl).is_null() {
        // 3380: ((*w).lgl): typeof(_1810) = *mut {l2828} LGL
        // 3380: ((*w).lgl):   l2828 = UNIQUE | NON_NULL, (empty)
            id = w.offset_from(workers) as libc::c_long as libc::c_int;
            // 3381: w: typeof(_1813) = *mut {l2832} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3381: w:   l2832 = UNIQUE | NON_NULL, (empty)
            // 3381: workers: typeof(_1814) = *const {l2834} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3381: workers:   l2834 = UNIQUE | NON_NULL, (empty)
            // 3381: workers: typeof(_1815) = *mut {l2836} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3381: workers:   l2836 = UNIQUE | NON_NULL, (empty)
            // 3381: workers: typeof(_1816) = *mut {l2838} *mut {l2839} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3381: workers:   l2838 = UNIQUE | NON_NULL, (empty)
            // 3381: workers:   l2839 = UNIQUE | NON_NULL, (empty)
            // 3381: workers: typeof(_1814 = move _1815 as *const Worker (Pointer(MutToConstPointer))) = *const {l4211} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3381: workers:   l4211 = UNIQUE | NON_NULL, (empty)
            printf(
                b"c %2d %s %7d %3.0f%% = %7d units %3.0f%% + %7d cls %3.0f%% + %7d eqs %3.0f%%\n\0"
                // 3383: b"c %2d %s %7d  ... _char: typeof(_1818) = *const {l2842} i8
                // 3383: b"c %2d %s %7d  ... _char:   l2842 = UNIQUE | NON_NULL, (empty)
                // 3383: b"c %2d %s %7d  ... st u8: typeof(_1819) = *const {l2844} u8
                // 3383: b"c %2d %s %7d  ... st u8:   l2844 = UNIQUE | NON_NULL, (empty)
                // 3383: b"c %2d %s %7d  ... \n\0": typeof(_1820) = *const {l2846} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000004e)) }]
                // 3383: b"c %2d %s %7d  ... \n\0":   l2846 = UNIQUE | NON_NULL, (empty)
                // 3383: b"c %2d %s %7d  ... \n\0": typeof(_1821) = & {l2848} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000004e)) }]
                // 3383: b"c %2d %s %7d  ... \n\0":   l2848 = UNIQUE | NON_NULL, FIXED
                // 3383: b"c %2d %s %7d  ... \n\0": typeof(_1821 = const b"c %2d %s %7d %3.0f%% = %7d units %3.0f%% + %7d cls %3.0f%% + %7d eqs %3.0f%%\n\x00") = & {l4212} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000004e)) }]
                // 3383: b"c %2d %s %7d  ... \n\0":   l4212 = UNIQUE | NON_NULL, (empty)
                // 3383: b"c %2d %s %7d  ... _char: typeof(_1818 = move _1819 as *const i8 (Misc)) = *const {l4215} i8
                // 3383: b"c %2d %s %7d  ... _char:   l4215 = UNIQUE | NON_NULL, (empty)
                // 3383: b"c %2d %s %7d  ... \n\0": typeof(_1820 = &raw const (*_1821)) = *const {l4213} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000004e)) }]
                // 3383: b"c %2d %s %7d  ... \n\0":   l4213 = UNIQUE | NON_NULL, (empty)
                // 3383: b"c %2d %s %7d  ... st u8: typeof(_1819 = move _1820 as *const u8 (Pointer(ArrayToPointer))) = *const {l4214} u8
                // 3383: b"c %2d %s %7d  ... st u8:   l4214 = UNIQUE | NON_NULL, (empty)
                    as *const u8 as *const libc::c_char,
                id,
                if w == maxproducer {
                // 3386: if w == maxprod ... har }: typeof(_1823) = *const {l2851} i8
                // 3386: if w == maxprod ... har }:   l2851 = UNIQUE | NON_NULL, (empty)
                // 3386: w: typeof(_1825) = *mut {l2854} DefId(0:535 ~ plingeling[18f5]::Worker)
                // 3386: w:   l2854 = UNIQUE | NON_NULL, (empty)
                // 3386: maxproducer: typeof(_1826) = *mut {l2856} DefId(0:535 ~ plingeling[18f5]::Worker)
                // 3386: maxproducer:   l2856 = UNIQUE | NON_NULL, (empty)
                    b"PROD\0" as *const u8 as *const libc::c_char
                    // 3387: b"PROD\0" as *c ... st u8: typeof(_1827) = *const {l2858} u8
                    // 3387: b"PROD\0" as *c ... st u8:   l2858 = UNIQUE | NON_NULL, (empty)
                    // 3387: b"PROD\0": typeof(_1828) = *const {l2860} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 3387: b"PROD\0":   l2860 = UNIQUE | NON_NULL, (empty)
                    // 3387: b"PROD\0": typeof(_1829) = & {l2862} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 3387: b"PROD\0":   l2862 = UNIQUE | NON_NULL, FIXED
                    // 3387: b"PROD\0" as *c ... _char: typeof(_1823 = move _1827 as *const i8 (Misc)) = *const {l4219} i8
                    // 3387: b"PROD\0" as *c ... _char:   l4219 = UNIQUE | NON_NULL, (empty)
                    // 3387: b"PROD\0": typeof(_1829 = const b"PROD\x00") = & {l4216} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 3387: b"PROD\0":   l4216 = UNIQUE | NON_NULL, (empty)
                    // 3387: b"PROD\0": typeof(_1828 = &raw const (*_1829)) = *const {l4217} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 3387: b"PROD\0":   l4217 = UNIQUE | NON_NULL, (empty)
                    // 3387: b"PROD\0" as *c ... st u8: typeof(_1827 = move _1828 as *const u8 (Pointer(ArrayToPointer))) = *const {l4218} u8
                    // 3387: b"PROD\0" as *c ... st u8:   l4218 = UNIQUE | NON_NULL, (empty)
                } else {
                    b"prod\0" as *const u8 as *const libc::c_char
                    // 3389: b"prod\0" as *c ... st u8: typeof(_1830) = *const {l2864} u8
                    // 3389: b"prod\0" as *c ... st u8:   l2864 = UNIQUE | NON_NULL, (empty)
                    // 3389: b"prod\0": typeof(_1831) = *const {l2866} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 3389: b"prod\0":   l2866 = UNIQUE | NON_NULL, (empty)
                    // 3389: b"prod\0": typeof(_1832) = & {l2868} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 3389: b"prod\0":   l2868 = UNIQUE | NON_NULL, FIXED
                    // 3389: b"prod\0": typeof(_1832 = const b"prod\x00") = & {l4220} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 3389: b"prod\0":   l4220 = UNIQUE | NON_NULL, (empty)
                    // 3389: b"prod\0" as *c ... _char: typeof(_1823 = move _1830 as *const i8 (Misc)) = *const {l4223} i8
                    // 3389: b"prod\0" as *c ... _char:   l4223 = UNIQUE | NON_NULL, (empty)
                    // 3389: b"prod\0": typeof(_1831 = &raw const (*_1832)) = *const {l4221} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 3389: b"prod\0":   l4221 = UNIQUE | NON_NULL, (empty)
                    // 3389: b"prod\0" as *c ... st u8: typeof(_1830 = move _1831 as *const u8 (Pointer(ArrayToPointer))) = *const {l4222} u8
                    // 3389: b"prod\0" as *c ... st u8:   l4222 = UNIQUE | NON_NULL, (empty)
                },
                (*w).stats.produced,
                percent(
                    (*w).stats.produced as libc::c_double,
                    ((units + eqs) as libc::c_long + clauses.added) as libc::c_double,
                    // 3394: units: typeof(_1842) = *mut {l2879} i32
                    // 3394: units:   l2879 = UNIQUE | NON_NULL, (empty)
                    // 3394: eqs: typeof(_1844) = *mut {l2882} i32
                    // 3394: eqs:   l2882 = UNIQUE | NON_NULL, (empty)
                    // 3394: clauses: typeof(_1847) = *mut {l2886} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
                    // 3394: clauses:   l2886 = UNIQUE | NON_NULL, (empty)
                ),
                (*w).stats.units.produced,
                percent(
                    (*w).stats.units.produced as libc::c_double,
                    units as libc::c_double,
                    // 3399: units: typeof(_1855) = *mut {l2895} i32
                    // 3399: units:   l2895 = UNIQUE | NON_NULL, (empty)
                ),
                (*w).stats.cls.produced,
                percent(
                    (*w).stats.cls.produced as libc::c_double,
                    clauses.added as libc::c_double,
                    // 3404: clauses: typeof(_1862) = *mut {l2903} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
                    // 3404: clauses:   l2903 = UNIQUE | NON_NULL, (empty)
                ),
                (*w).stats.eqs.produced,
                percent(
                    (*w).stats.eqs.produced as libc::c_double,
                    eqs as libc::c_double,
                    // 3409: eqs: typeof(_1869) = *mut {l2911} i32
                    // 3409: eqs:   l2911 = UNIQUE | NON_NULL, (empty)
                ),
            );
        }
        i += 1;
        i;
    }
    fputs(b"c \0" as *const u8 as *const libc::c_char, stdout);
    // 3416: b"c \0" as *con ... _char: typeof(_1876) = *const {l2919} i8
    // 3416: b"c \0" as *con ... _char:   l2919 = UNIQUE | NON_NULL, (empty)
    // 3416: b"c \0" as *const u8: typeof(_1877) = *const {l2921} u8
    // 3416: b"c \0" as *const u8:   l2921 = UNIQUE | NON_NULL, (empty)
    // 3416: b"c \0": typeof(_1878) = *const {l2923} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 3416: b"c \0":   l2923 = UNIQUE | NON_NULL, (empty)
    // 3416: b"c \0": typeof(_1879) = & {l2925} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 3416: b"c \0":   l2925 = UNIQUE | NON_NULL, FIXED
    // 3416: stdout: typeof(_1880) = *mut {l2927} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 3416: stdout:   l2927 = UNIQUE | NON_NULL, (empty)
    // 3416: stdout: typeof(_1881) = *mut {l2929} *mut {l2930} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 3416: stdout:   l2929 = UNIQUE | NON_NULL, (empty)
    // 3416: stdout:   l2930 = UNIQUE | NON_NULL, (empty)
    // 3416: b"c \0": typeof(_1879 = const b"c \x00") = & {l4224} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 3416: b"c \0":   l4224 = UNIQUE | NON_NULL, (empty)
    // 3416: b"c \0": typeof(_1878 = &raw const (*_1879)) = *const {l4225} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 3416: b"c \0":   l4225 = UNIQUE | NON_NULL, (empty)
    // 3416: b"c \0" as *con ... _char: typeof(_1876 = move _1877 as *const i8 (Misc)) = *const {l4227} i8
    // 3416: b"c \0" as *con ... _char:   l4227 = UNIQUE | NON_NULL, (empty)
    // 3416: b"c \0" as *const u8: typeof(_1877 = move _1878 as *const u8 (Pointer(ArrayToPointer))) = *const {l4226} u8
    // 3416: b"c \0" as *const u8:   l4226 = UNIQUE | NON_NULL, (empty)
    i = 0 as libc::c_int;
    while i < 79 as libc::c_int {
        fputc('-' as i32, stdout);
        // 3419: stdout: typeof(_1889) = *mut {l2939} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 3419: stdout:   l2939 = UNIQUE | NON_NULL, (empty)
        // 3419: stdout: typeof(_1890) = *mut {l2941} *mut {l2942} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 3419: stdout:   l2941 = UNIQUE | NON_NULL, (empty)
        // 3419: stdout:   l2942 = UNIQUE | NON_NULL, (empty)
        i += 1;
        i;
    }
    fputc('\n' as i32, stdout);
    // 3423: stdout: typeof(_1898) = *mut {l2951} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 3423: stdout:   l2951 = UNIQUE | NON_NULL, (empty)
    // 3423: stdout: typeof(_1899) = *mut {l2953} *mut {l2954} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 3423: stdout:   l2953 = UNIQUE | NON_NULL, (empty)
    // 3423: stdout:   l2954 = UNIQUE | NON_NULL, (empty)
    printf(
        b"c    prod %7lld 100%% = %7d units 100%% + %7lld cls 100%% + %7d eqs 100%%\n\0"
        // 3425: b"c prod %7lld  ... _char: typeof(_1901) = *const {l2957} i8
        // 3425: b"c prod %7lld  ... _char:   l2957 = UNIQUE | NON_NULL, (empty)
        // 3425: b"c prod %7lld  ... st u8: typeof(_1902) = *const {l2959} u8
        // 3425: b"c prod %7lld  ... st u8:   l2959 = UNIQUE | NON_NULL, (empty)
        // 3425: b"c prod %7lld  ... \n\0": typeof(_1903) = *const {l2961} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000004b)) }]
        // 3425: b"c prod %7lld  ... \n\0":   l2961 = UNIQUE | NON_NULL, (empty)
        // 3425: b"c prod %7lld  ... \n\0": typeof(_1904) = & {l2963} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000004b)) }]
        // 3425: b"c prod %7lld  ... \n\0":   l2963 = UNIQUE | NON_NULL, FIXED
        // 3425: b"c prod %7lld  ... \n\0": typeof(_1904 = const b"c    prod %7lld 100%% = %7d units 100%% + %7lld cls 100%% + %7d eqs 100%%\n\x00") = & {l4228} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000004b)) }]
        // 3425: b"c prod %7lld  ... \n\0":   l4228 = UNIQUE | NON_NULL, (empty)
        // 3425: b"c prod %7lld  ... \n\0": typeof(_1903 = &raw const (*_1904)) = *const {l4229} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000004b)) }]
        // 3425: b"c prod %7lld  ... \n\0":   l4229 = UNIQUE | NON_NULL, (empty)
        // 3425: b"c prod %7lld  ... st u8: typeof(_1902 = move _1903 as *const u8 (Pointer(ArrayToPointer))) = *const {l4230} u8
        // 3425: b"c prod %7lld  ... st u8:   l4230 = UNIQUE | NON_NULL, (empty)
        // 3425: b"c prod %7lld  ... _char: typeof(_1901 = move _1902 as *const i8 (Misc)) = *const {l4231} i8
        // 3425: b"c prod %7lld  ... _char:   l4231 = UNIQUE | NON_NULL, (empty)
            as *const u8 as *const libc::c_char,
        units as libc::c_longlong + eqs as libc::c_longlong + clauses.added as libc::c_longlong,
        // 3427: units: typeof(_1909) = *mut {l2969} i32
        // 3427: units:   l2969 = UNIQUE | NON_NULL, (empty)
        // 3427: eqs: typeof(_1912) = *mut {l2973} i32
        // 3427: eqs:   l2973 = UNIQUE | NON_NULL, (empty)
        // 3427: clauses: typeof(_1915) = *mut {l2977} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
        // 3427: clauses:   l2977 = UNIQUE | NON_NULL, (empty)
        units,
        // 3428: units: typeof(_1918) = *mut {l2981} i32
        // 3428: units:   l2981 = UNIQUE | NON_NULL, (empty)
        clauses.added as libc::c_longlong,
        // 3429: clauses: typeof(_1920) = *mut {l2984} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
        // 3429: clauses:   l2984 = UNIQUE | NON_NULL, (empty)
        eqs,
        // 3430: eqs: typeof(_1922) = *mut {l2987} i32
        // 3430: eqs:   l2987 = UNIQUE | NON_NULL, (empty)
    );
    printf(b"c\n\0" as *const u8 as *const libc::c_char);
    // 3432: b"c\n\0" as *co ... _char: typeof(_1924) = *const {l2990} i8
    // 3432: b"c\n\0" as *co ... _char:   l2990 = UNIQUE | NON_NULL, (empty)
    // 3432: b"c\n\0" as *co ... st u8: typeof(_1925) = *const {l2992} u8
    // 3432: b"c\n\0" as *co ... st u8:   l2992 = UNIQUE | NON_NULL, (empty)
    // 3432: b"c\n\0": typeof(_1926) = *const {l2994} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 3432: b"c\n\0":   l2994 = UNIQUE | NON_NULL, (empty)
    // 3432: b"c\n\0": typeof(_1927) = & {l2996} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 3432: b"c\n\0":   l2996 = UNIQUE | NON_NULL, FIXED
    // 3432: b"c\n\0": typeof(_1926 = &raw const (*_1927)) = *const {l4233} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 3432: b"c\n\0":   l4233 = UNIQUE | NON_NULL, (empty)
    // 3432: b"c\n\0": typeof(_1927 = const b"c\n\x00") = & {l4232} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 3432: b"c\n\0":   l4232 = UNIQUE | NON_NULL, (empty)
    // 3432: b"c\n\0" as *co ... st u8: typeof(_1925 = move _1926 as *const u8 (Pointer(ArrayToPointer))) = *const {l4234} u8
    // 3432: b"c\n\0" as *co ... st u8:   l4234 = UNIQUE | NON_NULL, (empty)
    // 3432: b"c\n\0" as *co ... _char: typeof(_1924 = move _1925 as *const i8 (Misc)) = *const {l4235} i8
    // 3432: b"c\n\0" as *co ... _char:   l4235 = UNIQUE | NON_NULL, (empty)
    qsort(
        sorted as *mut libc::c_void,
        // 3434: sorted as *mut  ... _void: typeof(_1929) = *mut {l2999} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 3434: sorted as *mut  ... _void:   l2999 = UNIQUE | NON_NULL, (empty)
        // 3434: sorted: typeof(_1930) = *mut {l3001} *mut {l3002} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3434: sorted:   l3001 = UNIQUE | NON_NULL, (empty)
        // 3434: sorted:   l3002 = UNIQUE | NON_NULL, (empty)
        // 3434: sorted as *mut  ... _void: typeof(_1929 = move _1930 as *mut libc::c_void (Misc)) = *mut {l4236} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 3434: sorted as *mut  ... _void:   l4236 = UNIQUE | NON_NULL, (empty)
        nworkers as size_t,
        // 3435: nworkers: typeof(_1933) = *mut {l3006} i32
        // 3435: nworkers:   l3006 = UNIQUE | NON_NULL, (empty)
        ::core::mem::size_of::<*mut Worker>() as libc::c_ulong,
        Some(
        // 3437: Some( cmpconsum ... nt, ): typeof(_1936) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*const {l3010} DefId(2:5583 ~ core[480a]::ffi::c_void), *const {l3011} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32>
        // 3437: Some( cmpconsum ... nt, ):   l3010 = UNIQUE | NON_NULL, (empty)
        // 3437: Some( cmpconsum ... nt, ):   l3011 = UNIQUE | NON_NULL, (empty)
        // 3437: Some( cmpconsum ... nt, ): typeof(_1936 = std::option::Option::<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>::Some(move _1937)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*const {l4239} DefId(2:5583 ~ core[480a]::ffi::c_void), *const {l4240} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32>
        // 3437: Some( cmpconsum ... nt, ):   l4239 = UNIQUE | NON_NULL, (empty)
        // 3437: Some( cmpconsum ... nt, ):   l4240 = UNIQUE | NON_NULL, (empty)
            cmpconsumed
            // 3438: cmpconsumed as  ... c_int: typeof(_1937) = fn(*const {l3013} DefId(2:5583 ~ core[480a]::ffi::c_void), *const {l3014} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32
            // 3438: cmpconsumed as  ... c_int:   l3013 = UNIQUE | NON_NULL, (empty)
            // 3438: cmpconsumed as  ... c_int:   l3014 = UNIQUE | NON_NULL, (empty)
            // 3438: cmpconsumed: typeof(_1937 = cmpconsumed as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32 (Pointer(ReifyFnPointer))) = fn(*const {l4237} DefId(2:5583 ~ core[480a]::ffi::c_void), *const {l4238} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32
            // 3438: cmpconsumed:   l4237 = UNIQUE | NON_NULL, (empty)
            // 3438: cmpconsumed:   l4238 = UNIQUE | NON_NULL, (empty)
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        ),
    );
    sumconsumedeqs = 0 as libc::c_int;
    sumconsumedcls = sumconsumedeqs;
    sumconsumedunits = sumconsumedcls;
    sumconsumed = sumconsumedunits;
    i = 0 as libc::c_int;
    while i < nworkers {
    // 3447: nworkers: typeof(_1947) = *mut {l3025} i32
    // 3447: nworkers:   l3025 = UNIQUE | NON_NULL, (empty)
        w = *sorted.offset(i as isize);
        // 3448: *sorted.offset( ... size): typeof(_1948) = *mut {l3027} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3448: *sorted.offset( ... size):   l3027 = UNIQUE | NON_NULL, (empty)
        // 3448: sorted.offset(i ... size): typeof(_1949) = *mut {l3029} *mut {l3030} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3448: sorted.offset(i ... size):   l3029 = UNIQUE | NON_NULL, (empty)
        // 3448: sorted.offset(i ... size):   l3030 = UNIQUE | NON_NULL, (empty)
        // 3448: sorted: typeof(_1950) = *mut {l3032} *mut {l3033} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3448: sorted:   l3032 = UNIQUE | NON_NULL, (empty)
        // 3448: sorted:   l3033 = UNIQUE | NON_NULL, (empty)
        if !((*w).lgl).is_null() {
        // 3449: ((*w).lgl): typeof(_1956) = *mut {l3040} LGL
        // 3449: ((*w).lgl):   l3040 = UNIQUE | NON_NULL, (empty)
            id = w.offset_from(workers) as libc::c_long as libc::c_int;
            // 3450: w: typeof(_1959) = *mut {l3044} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3450: w:   l3044 = UNIQUE | NON_NULL, (empty)
            // 3450: workers: typeof(_1960) = *const {l3046} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3450: workers:   l3046 = UNIQUE | NON_NULL, (empty)
            // 3450: workers: typeof(_1961) = *mut {l3048} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3450: workers:   l3048 = UNIQUE | NON_NULL, (empty)
            // 3450: workers: typeof(_1962) = *mut {l3050} *mut {l3051} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3450: workers:   l3050 = UNIQUE | NON_NULL, (empty)
            // 3450: workers:   l3051 = UNIQUE | NON_NULL, (empty)
            // 3450: workers: typeof(_1960 = move _1961 as *const Worker (Pointer(MutToConstPointer))) = *const {l4241} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3450: workers:   l4241 = UNIQUE | NON_NULL, (empty)
            sumconsumed += (*w).stats.consumed;
            sumconsumedeqs += (*w).stats.eqs.consumed;
            sumconsumedcls += (*w).stats.cls.consumed;
            sumconsumedunits += (*w).stats.units.consumed;
            printf(
                b"c %2d %s %7d %3.0f%% = %7d units %3.0f%% + %7d cls %3.0f%% + %7d eqs %3.0f%%\n\0"
                // 3456: b"c %2d %s %7d  ... _char: typeof(_1972) = *const {l3062} i8
                // 3456: b"c %2d %s %7d  ... _char:   l3062 = UNIQUE | NON_NULL, (empty)
                // 3456: b"c %2d %s %7d  ... st u8: typeof(_1973) = *const {l3064} u8
                // 3456: b"c %2d %s %7d  ... st u8:   l3064 = UNIQUE | NON_NULL, (empty)
                // 3456: b"c %2d %s %7d  ... \n\0": typeof(_1974) = *const {l3066} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000004e)) }]
                // 3456: b"c %2d %s %7d  ... \n\0":   l3066 = UNIQUE | NON_NULL, (empty)
                // 3456: b"c %2d %s %7d  ... \n\0": typeof(_1975) = & {l3068} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000004e)) }]
                // 3456: b"c %2d %s %7d  ... \n\0":   l3068 = UNIQUE | NON_NULL, FIXED
                // 3456: b"c %2d %s %7d  ... _char: typeof(_1972 = move _1973 as *const i8 (Misc)) = *const {l4245} i8
                // 3456: b"c %2d %s %7d  ... _char:   l4245 = UNIQUE | NON_NULL, (empty)
                // 3456: b"c %2d %s %7d  ... st u8: typeof(_1973 = move _1974 as *const u8 (Pointer(ArrayToPointer))) = *const {l4244} u8
                // 3456: b"c %2d %s %7d  ... st u8:   l4244 = UNIQUE | NON_NULL, (empty)
                // 3456: b"c %2d %s %7d  ... \n\0": typeof(_1974 = &raw const (*_1975)) = *const {l4243} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000004e)) }]
                // 3456: b"c %2d %s %7d  ... \n\0":   l4243 = UNIQUE | NON_NULL, (empty)
                // 3456: b"c %2d %s %7d  ... \n\0": typeof(_1975 = const b"c %2d %s %7d %3.0f%% = %7d units %3.0f%% + %7d cls %3.0f%% + %7d eqs %3.0f%%\n\x00") = & {l4242} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000004e)) }]
                // 3456: b"c %2d %s %7d  ... \n\0":   l4242 = UNIQUE | NON_NULL, (empty)
                    as *const u8 as *const libc::c_char,
                id,
                if w == maxconsumer {
                // 3459: if w == maxcons ... har }: typeof(_1977) = *const {l3071} i8
                // 3459: if w == maxcons ... har }:   l3071 = UNIQUE | NON_NULL, (empty)
                // 3459: w: typeof(_1979) = *mut {l3074} DefId(0:535 ~ plingeling[18f5]::Worker)
                // 3459: w:   l3074 = UNIQUE | NON_NULL, (empty)
                // 3459: maxconsumer: typeof(_1980) = *mut {l3076} DefId(0:535 ~ plingeling[18f5]::Worker)
                // 3459: maxconsumer:   l3076 = UNIQUE | NON_NULL, (empty)
                    b"CONS\0" as *const u8 as *const libc::c_char
                    // 3460: b"CONS\0" as *c ... st u8: typeof(_1981) = *const {l3078} u8
                    // 3460: b"CONS\0" as *c ... st u8:   l3078 = UNIQUE | NON_NULL, (empty)
                    // 3460: b"CONS\0": typeof(_1982) = *const {l3080} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 3460: b"CONS\0":   l3080 = UNIQUE | NON_NULL, (empty)
                    // 3460: b"CONS\0": typeof(_1983) = & {l3082} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 3460: b"CONS\0":   l3082 = UNIQUE | NON_NULL, FIXED
                    // 3460: b"CONS\0": typeof(_1982 = &raw const (*_1983)) = *const {l4247} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 3460: b"CONS\0":   l4247 = UNIQUE | NON_NULL, (empty)
                    // 3460: b"CONS\0" as *c ... _char: typeof(_1977 = move _1981 as *const i8 (Misc)) = *const {l4249} i8
                    // 3460: b"CONS\0" as *c ... _char:   l4249 = UNIQUE | NON_NULL, (empty)
                    // 3460: b"CONS\0": typeof(_1983 = const b"CONS\x00") = & {l4246} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 3460: b"CONS\0":   l4246 = UNIQUE | NON_NULL, (empty)
                    // 3460: b"CONS\0" as *c ... st u8: typeof(_1981 = move _1982 as *const u8 (Pointer(ArrayToPointer))) = *const {l4248} u8
                    // 3460: b"CONS\0" as *c ... st u8:   l4248 = UNIQUE | NON_NULL, (empty)
                } else {
                    b"cons\0" as *const u8 as *const libc::c_char
                    // 3462: b"cons\0" as *c ... st u8: typeof(_1984) = *const {l3084} u8
                    // 3462: b"cons\0" as *c ... st u8:   l3084 = UNIQUE | NON_NULL, (empty)
                    // 3462: b"cons\0": typeof(_1985) = *const {l3086} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 3462: b"cons\0":   l3086 = UNIQUE | NON_NULL, (empty)
                    // 3462: b"cons\0": typeof(_1986) = & {l3088} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 3462: b"cons\0":   l3088 = UNIQUE | NON_NULL, FIXED
                    // 3462: b"cons\0": typeof(_1986 = const b"cons\x00") = & {l4250} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 3462: b"cons\0":   l4250 = UNIQUE | NON_NULL, (empty)
                    // 3462: b"cons\0": typeof(_1985 = &raw const (*_1986)) = *const {l4251} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 3462: b"cons\0":   l4251 = UNIQUE | NON_NULL, (empty)
                    // 3462: b"cons\0" as *c ... st u8: typeof(_1984 = move _1985 as *const u8 (Pointer(ArrayToPointer))) = *const {l4252} u8
                    // 3462: b"cons\0" as *c ... st u8:   l4252 = UNIQUE | NON_NULL, (empty)
                    // 3462: b"cons\0" as *c ... _char: typeof(_1977 = move _1984 as *const i8 (Misc)) = *const {l4253} i8
                    // 3462: b"cons\0" as *c ... _char:   l4253 = UNIQUE | NON_NULL, (empty)
                },
                (*w).stats.consumed,
                percent(
                    (*w).stats.consumed as libc::c_double,
                    ((units + eqs) as libc::c_long + clauses.added) as libc::c_double,
                    // 3467: units: typeof(_1996) = *mut {l3099} i32
                    // 3467: units:   l3099 = UNIQUE | NON_NULL, (empty)
                    // 3467: eqs: typeof(_1998) = *mut {l3102} i32
                    // 3467: eqs:   l3102 = UNIQUE | NON_NULL, (empty)
                    // 3467: clauses: typeof(_2001) = *mut {l3106} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
                    // 3467: clauses:   l3106 = UNIQUE | NON_NULL, (empty)
                ),
                (*w).stats.units.consumed,
                percent(
                    (*w).stats.units.consumed as libc::c_double,
                    units as libc::c_double,
                    // 3472: units: typeof(_2009) = *mut {l3115} i32
                    // 3472: units:   l3115 = UNIQUE | NON_NULL, (empty)
                ),
                (*w).stats.cls.consumed,
                percent(
                    (*w).stats.cls.consumed as libc::c_double,
                    clauses.added as libc::c_double,
                    // 3477: clauses: typeof(_2016) = *mut {l3123} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
                    // 3477: clauses:   l3123 = UNIQUE | NON_NULL, (empty)
                ),
                (*w).stats.eqs.consumed,
                percent(
                    (*w).stats.eqs.consumed as libc::c_double,
                    eqs as libc::c_double,
                    // 3482: eqs: typeof(_2023) = *mut {l3131} i32
                    // 3482: eqs:   l3131 = UNIQUE | NON_NULL, (empty)
                ),
            );
        }
        i += 1;
        i;
    }
    fputs(b"c \0" as *const u8 as *const libc::c_char, stdout);
    // 3489: b"c \0" as *con ... _char: typeof(_2030) = *const {l3139} i8
    // 3489: b"c \0" as *con ... _char:   l3139 = UNIQUE | NON_NULL, (empty)
    // 3489: b"c \0" as *const u8: typeof(_2031) = *const {l3141} u8
    // 3489: b"c \0" as *const u8:   l3141 = UNIQUE | NON_NULL, (empty)
    // 3489: b"c \0": typeof(_2032) = *const {l3143} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 3489: b"c \0":   l3143 = UNIQUE | NON_NULL, (empty)
    // 3489: b"c \0": typeof(_2033) = & {l3145} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 3489: b"c \0":   l3145 = UNIQUE | NON_NULL, FIXED
    // 3489: stdout: typeof(_2034) = *mut {l3147} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 3489: stdout:   l3147 = UNIQUE | NON_NULL, (empty)
    // 3489: stdout: typeof(_2035) = *mut {l3149} *mut {l3150} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 3489: stdout:   l3149 = UNIQUE | NON_NULL, (empty)
    // 3489: stdout:   l3150 = UNIQUE | NON_NULL, (empty)
    // 3489: b"c \0": typeof(_2032 = &raw const (*_2033)) = *const {l4255} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 3489: b"c \0":   l4255 = UNIQUE | NON_NULL, (empty)
    // 3489: b"c \0" as *const u8: typeof(_2031 = move _2032 as *const u8 (Pointer(ArrayToPointer))) = *const {l4256} u8
    // 3489: b"c \0" as *const u8:   l4256 = UNIQUE | NON_NULL, (empty)
    // 3489: b"c \0": typeof(_2033 = const b"c \x00") = & {l4254} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 3489: b"c \0":   l4254 = UNIQUE | NON_NULL, (empty)
    // 3489: b"c \0" as *con ... _char: typeof(_2030 = move _2031 as *const i8 (Misc)) = *const {l4257} i8
    // 3489: b"c \0" as *con ... _char:   l4257 = UNIQUE | NON_NULL, (empty)
    i = 0 as libc::c_int;
    while i < 79 as libc::c_int {
        fputc('-' as i32, stdout);
        // 3492: stdout: typeof(_2043) = *mut {l3159} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 3492: stdout:   l3159 = UNIQUE | NON_NULL, (empty)
        // 3492: stdout: typeof(_2044) = *mut {l3161} *mut {l3162} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
        // 3492: stdout:   l3161 = UNIQUE | NON_NULL, (empty)
        // 3492: stdout:   l3162 = UNIQUE | NON_NULL, (empty)
        i += 1;
        i;
    }
    fputc('\n' as i32, stdout);
    // 3496: stdout: typeof(_2052) = *mut {l3171} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 3496: stdout:   l3171 = UNIQUE | NON_NULL, (empty)
    // 3496: stdout: typeof(_2053) = *mut {l3173} *mut {l3174} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 3496: stdout:   l3173 = UNIQUE | NON_NULL, (empty)
    // 3496: stdout:   l3174 = UNIQUE | NON_NULL, (empty)
    printf(
        b"c    cons %7d %3.0f%% = %7d units %3.0f%% + %7d cls %3.0f%% + %7d eqs %3.0f%%\n\0"
        // 3498: b"c cons %7d %3 ... _char: typeof(_2055) = *const {l3177} i8
        // 3498: b"c cons %7d %3 ... _char:   l3177 = UNIQUE | NON_NULL, (empty)
        // 3498: b"c cons %7d %3 ... st u8: typeof(_2056) = *const {l3179} u8
        // 3498: b"c cons %7d %3 ... st u8:   l3179 = UNIQUE | NON_NULL, (empty)
        // 3498: b"c cons %7d %3 ... \n\0": typeof(_2057) = *const {l3181} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000004f)) }]
        // 3498: b"c cons %7d %3 ... \n\0":   l3181 = UNIQUE | NON_NULL, (empty)
        // 3498: b"c cons %7d %3 ... \n\0": typeof(_2058) = & {l3183} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000004f)) }]
        // 3498: b"c cons %7d %3 ... \n\0":   l3183 = UNIQUE | NON_NULL, FIXED
        // 3498: b"c cons %7d %3 ... _char: typeof(_2055 = move _2056 as *const i8 (Misc)) = *const {l4261} i8
        // 3498: b"c cons %7d %3 ... _char:   l4261 = UNIQUE | NON_NULL, (empty)
        // 3498: b"c cons %7d %3 ... \n\0": typeof(_2057 = &raw const (*_2058)) = *const {l4259} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000004f)) }]
        // 3498: b"c cons %7d %3 ... \n\0":   l4259 = UNIQUE | NON_NULL, (empty)
        // 3498: b"c cons %7d %3 ... st u8: typeof(_2056 = move _2057 as *const u8 (Pointer(ArrayToPointer))) = *const {l4260} u8
        // 3498: b"c cons %7d %3 ... st u8:   l4260 = UNIQUE | NON_NULL, (empty)
        // 3498: b"c cons %7d %3 ... \n\0": typeof(_2058 = const b"c    cons %7d %3.0f%% = %7d units %3.0f%% + %7d cls %3.0f%% + %7d eqs %3.0f%%\n\x00") = & {l4258} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000004f)) }]
        // 3498: b"c cons %7d %3 ... \n\0":   l4258 = UNIQUE | NON_NULL, (empty)
            as *const u8 as *const libc::c_char,
        sumconsumed,
        percent(
            sumconsumed as libc::c_double,
            ((units + eqs) as libc::c_long + clauses.added) as libc::c_double,
            // 3503: units: typeof(_2068) = *mut {l3194} i32
            // 3503: units:   l3194 = UNIQUE | NON_NULL, (empty)
            // 3503: eqs: typeof(_2070) = *mut {l3197} i32
            // 3503: eqs:   l3197 = UNIQUE | NON_NULL, (empty)
            // 3503: clauses: typeof(_2073) = *mut {l3201} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
            // 3503: clauses:   l3201 = UNIQUE | NON_NULL, (empty)
        ),
        sumconsumedunits,
        percent(sumconsumedunits as libc::c_double, units as libc::c_double),
        // 3506: units: typeof(_2081) = *mut {l3210} i32
        // 3506: units:   l3210 = UNIQUE | NON_NULL, (empty)
        sumconsumedcls,
        percent(
            sumconsumedcls as libc::c_double,
            clauses.added as libc::c_double,
            // 3510: clauses: typeof(_2088) = *mut {l3218} DefId(0:574 ~ plingeling[18f5]::C2RustUnnamed_5)
            // 3510: clauses:   l3218 = UNIQUE | NON_NULL, (empty)
        ),
        sumconsumedeqs,
        percent(sumconsumedeqs as libc::c_double, eqs as libc::c_double),
        // 3513: eqs: typeof(_2095) = *mut {l3226} i32
        // 3513: eqs:   l3226 = UNIQUE | NON_NULL, (empty)
    );
    let mut BYTES_1: size_t = (nworkers as libc::c_ulong)
    // 3515: nworkers: typeof(_2099) = *mut {l3231} i32
    // 3515: nworkers:   l3231 = UNIQUE | NON_NULL, (empty)
        .wrapping_mul(::core::mem::size_of::<*mut Worker>() as libc::c_ulong);
    decmem(BYTES_1);
    free(sorted as *mut libc::c_void);
    // 3518: sorted as *mut  ... _void: typeof(_2105) = *mut {l3238} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 3518: sorted as *mut  ... _void:   l3238 = UNIQUE | NON_NULL, (empty)
    // 3518: sorted: typeof(_2106) = *mut {l3240} *mut {l3241} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 3518: sorted:   l3240 = UNIQUE | NON_NULL, (empty)
    // 3518: sorted:   l3241 = UNIQUE | NON_NULL, (empty)
    // 3518: sorted as *mut  ... _void: typeof(_2105 = move _2106 as *mut libc::c_void (Misc)) = *mut {l4262} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 3518: sorted as *mut  ... _void:   l4262 = UNIQUE | NON_NULL, (empty)
    fflush(stdout);
    // 3519: stdout: typeof(_2108) = *mut {l3244} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 3519: stdout:   l3244 = UNIQUE | NON_NULL, (empty)
    // 3519: stdout: typeof(_2109) = *mut {l3246} *mut {l3247} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 3519: stdout:   l3246 = UNIQUE | NON_NULL, (empty)
    // 3519: stdout:   l3247 = UNIQUE | NON_NULL, (empty)
    if res == 0 {
        res = globalres;
        // 3521: globalres: typeof(_2114) = *mut {l3253} i32
        // 3521: globalres:   l3253 = UNIQUE | NON_NULL, (empty)
    }
    if res == 0 {
        die(b"no result by any worker\0" as *const u8 as *const libc::c_char);
        // 3524: b"no result by  ... _char: typeof(_2119) = *const {l3259} i8
        // 3524: b"no result by  ... _char:   l3259 = UNIQUE | NON_NULL, (empty)
        // 3524: b"no result by  ... st u8: typeof(_2120) = *const {l3261} u8
        // 3524: b"no result by  ... st u8:   l3261 = UNIQUE | NON_NULL, (empty)
        // 3524: b"no result by  ... er\0": typeof(_2121) = *const {l3263} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 3524: b"no result by  ... er\0":   l3263 = UNIQUE | NON_NULL, (empty)
        // 3524: b"no result by  ... er\0": typeof(_2122) = & {l3265} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 3524: b"no result by  ... er\0":   l3265 = UNIQUE | NON_NULL, FIXED
        // 3524: b"no result by  ... er\0": typeof(_2121 = &raw const (*_2122)) = *const {l4264} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 3524: b"no result by  ... er\0":   l4264 = UNIQUE | NON_NULL, (empty)
        // 3524: b"no result by  ... st u8: typeof(_2120 = move _2121 as *const u8 (Pointer(ArrayToPointer))) = *const {l4265} u8
        // 3524: b"no result by  ... st u8:   l4265 = UNIQUE | NON_NULL, (empty)
        // 3524: b"no result by  ... er\0": typeof(_2122 = const b"no result by any worker\x00") = & {l4263} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 3524: b"no result by  ... er\0":   l4263 = UNIQUE | NON_NULL, (empty)
        // 3524: b"no result by  ... _char: typeof(_2119 = move _2120 as *const i8 (Misc)) = *const {l4266} i8
        // 3524: b"no result by  ... _char:   l4266 = UNIQUE | NON_NULL, (empty)
    }
    msg(
        -(1 as libc::c_int),
        2 as libc::c_int,
        b"copying assignment\0" as *const u8 as *const libc::c_char,
        // 3529: b"copying assig ... _char: typeof(_2128) = *const {l3272} i8
        // 3529: b"copying assig ... _char:   l3272 = UNIQUE | NON_NULL, (empty)
        // 3529: b"copying assig ... st u8: typeof(_2129) = *const {l3274} u8
        // 3529: b"copying assig ... st u8:   l3274 = UNIQUE | NON_NULL, (empty)
        // 3529: b"copying assig ... nt\0": typeof(_2130) = *const {l3276} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
        // 3529: b"copying assig ... nt\0":   l3276 = UNIQUE | NON_NULL, (empty)
        // 3529: b"copying assig ... nt\0": typeof(_2131) = & {l3278} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
        // 3529: b"copying assig ... nt\0":   l3278 = UNIQUE | NON_NULL, FIXED
        // 3529: b"copying assig ... nt\0": typeof(_2130 = &raw const (*_2131)) = *const {l4268} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
        // 3529: b"copying assig ... nt\0":   l4268 = UNIQUE | NON_NULL, (empty)
        // 3529: b"copying assig ... st u8: typeof(_2129 = move _2130 as *const u8 (Pointer(ArrayToPointer))) = *const {l4269} u8
        // 3529: b"copying assig ... st u8:   l4269 = UNIQUE | NON_NULL, (empty)
        // 3529: b"copying assig ... nt\0": typeof(_2131 = const b"copying assignment\x00") = & {l4267} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
        // 3529: b"copying assig ... nt\0":   l4267 = UNIQUE | NON_NULL, (empty)
        // 3529: b"copying assig ... _char: typeof(_2128 = move _2129 as *const i8 (Misc)) = *const {l4270} i8
        // 3529: b"copying assig ... _char:   l4270 = UNIQUE | NON_NULL, (empty)
    );
    if !winner.is_null() && res == 10 as libc::c_int {
    // 3531: winner: typeof(_2136) = *mut {l3284} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 3531: winner:   l3284 = UNIQUE | NON_NULL, (empty)
        i = 1 as libc::c_int;
        while i <= nvars {
        // 3533: nvars: typeof(_2144) = *mut {l3293} i32
        // 3533: nvars:   l3293 = UNIQUE | NON_NULL, (empty)
            val = lglderef((*winner).lgl, i);
            // 3534: (*winner).lgl: typeof(_2146) = *mut {l3296} LGL
            // 3534: (*winner).lgl:   l3296 = UNIQUE | NON_NULL, (empty)
            *vals.offset(i as isize) != 0;
            // 3535: vals.offset(i a ... size): typeof(_2150) = *mut {l3301} i32
            // 3535: vals.offset(i a ... size):   l3301 = UNIQUE | NON_NULL, (empty)
            // 3535: vals: typeof(_2151) = *mut {l3303} i32
            // 3535: vals:   l3303 = UNIQUE | NON_NULL, (empty)
            // 3535: vals: typeof(_2152) = *mut {l3305} *mut {l3306} i32
            // 3535: vals:   l3305 = UNIQUE | NON_NULL, (empty)
            // 3535: vals:   l3306 = UNIQUE | NON_NULL, (empty)
            *vals.offset(i as isize) = val;
            // 3536: vals.offset(i a ... size): typeof(_2156) = *mut {l3311} i32
            // 3536: vals.offset(i a ... size):   l3311 = UNIQUE | NON_NULL, (empty)
            // 3536: vals: typeof(_2157) = *mut {l3313} i32
            // 3536: vals:   l3313 = UNIQUE | NON_NULL, (empty)
            // 3536: vals: typeof(_2158) = *mut {l3315} *mut {l3316} i32
            // 3536: vals:   l3315 = UNIQUE | NON_NULL, (empty)
            // 3536: vals:   l3316 = UNIQUE | NON_NULL, (empty)
            i += 1;
            i;
        }
    }
    resetsighandlers();
    if res == 10 as libc::c_int {
        printf(b"s SATISFIABLE\n\0" as *const u8 as *const libc::c_char);
        // 3543: b"s SATISFIABLE ... _char: typeof(_2172) = *const {l3331} i8
        // 3543: b"s SATISFIABLE ... _char:   l3331 = UNIQUE | NON_NULL, (empty)
        // 3543: b"s SATISFIABLE ... st u8: typeof(_2173) = *const {l3333} u8
        // 3543: b"s SATISFIABLE ... st u8:   l3333 = UNIQUE | NON_NULL, (empty)
        // 3543: b"s SATISFIABLE\n\0": typeof(_2174) = *const {l3335} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 3543: b"s SATISFIABLE\n\0":   l3335 = UNIQUE | NON_NULL, (empty)
        // 3543: b"s SATISFIABLE\n\0": typeof(_2175) = & {l3337} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 3543: b"s SATISFIABLE\n\0":   l3337 = UNIQUE | NON_NULL, FIXED
        // 3543: b"s SATISFIABLE ... _char: typeof(_2172 = move _2173 as *const i8 (Misc)) = *const {l4274} i8
        // 3543: b"s SATISFIABLE ... _char:   l4274 = UNIQUE | NON_NULL, (empty)
        // 3543: b"s SATISFIABLE\n\0": typeof(_2174 = &raw const (*_2175)) = *const {l4272} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 3543: b"s SATISFIABLE\n\0":   l4272 = UNIQUE | NON_NULL, (empty)
        // 3543: b"s SATISFIABLE ... st u8: typeof(_2173 = move _2174 as *const u8 (Pointer(ArrayToPointer))) = *const {l4273} u8
        // 3543: b"s SATISFIABLE ... st u8:   l4273 = UNIQUE | NON_NULL, (empty)
        // 3543: b"s SATISFIABLE\n\0": typeof(_2175 = const b"s SATISFIABLE\n\x00") = & {l4271} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 3543: b"s SATISFIABLE\n\0":   l4271 = UNIQUE | NON_NULL, (empty)
        if witness != 0 {
            fflush(stdout);
            // 3545: stdout: typeof(_2179) = *mut {l3342} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 3545: stdout:   l3342 = UNIQUE | NON_NULL, (empty)
            // 3545: stdout: typeof(_2180) = *mut {l3344} *mut {l3345} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
            // 3545: stdout:   l3344 = UNIQUE | NON_NULL, (empty)
            // 3545: stdout:   l3345 = UNIQUE | NON_NULL, (empty)
            if nvars != 0 {
            // 3546: nvars: typeof(_2184) = *mut {l3350} i32
            // 3546: nvars:   l3350 = UNIQUE | NON_NULL, (empty)
                printf(b"v\0" as *const u8 as *const libc::c_char);
                // 3547: b"v\0" as *cons ... _char: typeof(_2186) = *const {l3353} i8
                // 3547: b"v\0" as *cons ... _char:   l3353 = UNIQUE | NON_NULL, (empty)
                // 3547: b"v\0" as *const u8: typeof(_2187) = *const {l3355} u8
                // 3547: b"v\0" as *const u8:   l3355 = UNIQUE | NON_NULL, (empty)
                // 3547: b"v\0": typeof(_2188) = *const {l3357} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 3547: b"v\0":   l3357 = UNIQUE | NON_NULL, (empty)
                // 3547: b"v\0": typeof(_2189) = & {l3359} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 3547: b"v\0":   l3359 = UNIQUE | NON_NULL, FIXED
                // 3547: b"v\0": typeof(_2189 = const b"v\x00") = & {l4275} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 3547: b"v\0":   l4275 = UNIQUE | NON_NULL, (empty)
                // 3547: b"v\0" as *cons ... _char: typeof(_2186 = move _2187 as *const i8 (Misc)) = *const {l4278} i8
                // 3547: b"v\0" as *cons ... _char:   l4278 = UNIQUE | NON_NULL, (empty)
                // 3547: b"v\0": typeof(_2188 = &raw const (*_2189)) = *const {l4276} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 3547: b"v\0":   l4276 = UNIQUE | NON_NULL, (empty)
                // 3547: b"v\0" as *const u8: typeof(_2187 = move _2188 as *const u8 (Pointer(ArrayToPointer))) = *const {l4277} u8
                // 3547: b"v\0" as *const u8:   l4277 = UNIQUE | NON_NULL, (empty)
                i = 1 as libc::c_int;
                while i <= nvars {
                // 3549: nvars: typeof(_2195) = *mut {l3366} i32
                // 3549: nvars:   l3366 = UNIQUE | NON_NULL, (empty)
                    if i & 7 as libc::c_int == 0 {
                        fputs(b"\nv\0" as *const u8 as *const libc::c_char, stdout);
                        // 3551: b"\nv\0" as *co ... _char: typeof(_2202) = *const {l3374} i8
                        // 3551: b"\nv\0" as *co ... _char:   l3374 = UNIQUE | NON_NULL, (empty)
                        // 3551: b"\nv\0" as *co ... st u8: typeof(_2203) = *const {l3376} u8
                        // 3551: b"\nv\0" as *co ... st u8:   l3376 = UNIQUE | NON_NULL, (empty)
                        // 3551: b"\nv\0": typeof(_2204) = *const {l3378} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                        // 3551: b"\nv\0":   l3378 = UNIQUE | NON_NULL, (empty)
                        // 3551: b"\nv\0": typeof(_2205) = & {l3380} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                        // 3551: b"\nv\0":   l3380 = UNIQUE | NON_NULL, FIXED
                        // 3551: stdout: typeof(_2206) = *mut {l3382} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
                        // 3551: stdout:   l3382 = UNIQUE | NON_NULL, (empty)
                        // 3551: stdout: typeof(_2207) = *mut {l3384} *mut {l3385} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
                        // 3551: stdout:   l3384 = UNIQUE | NON_NULL, (empty)
                        // 3551: stdout:   l3385 = UNIQUE | NON_NULL, (empty)
                        // 3551: b"\nv\0" as *co ... st u8: typeof(_2203 = move _2204 as *const u8 (Pointer(ArrayToPointer))) = *const {l4281} u8
                        // 3551: b"\nv\0" as *co ... st u8:   l4281 = UNIQUE | NON_NULL, (empty)
                        // 3551: b"\nv\0": typeof(_2204 = &raw const (*_2205)) = *const {l4280} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                        // 3551: b"\nv\0":   l4280 = UNIQUE | NON_NULL, (empty)
                        // 3551: b"\nv\0": typeof(_2205 = const b"\nv\x00") = & {l4279} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                        // 3551: b"\nv\0":   l4279 = UNIQUE | NON_NULL, (empty)
                        // 3551: b"\nv\0" as *co ... _char: typeof(_2202 = move _2203 as *const i8 (Misc)) = *const {l4282} i8
                        // 3551: b"\nv\0" as *co ... _char:   l4282 = UNIQUE | NON_NULL, (empty)
                    }
                    lit = if *vals.offset(i as isize) < 0 as libc::c_int {
                    // 3553: vals.offset(i a ... size): typeof(_2211) = *mut {l3390} i32
                    // 3553: vals.offset(i a ... size):   l3390 = UNIQUE | NON_NULL, (empty)
                    // 3553: vals: typeof(_2212) = *mut {l3392} i32
                    // 3553: vals:   l3392 = UNIQUE | NON_NULL, (empty)
                    // 3553: vals: typeof(_2213) = *mut {l3394} *mut {l3395} i32
                    // 3553: vals:   l3394 = UNIQUE | NON_NULL, (empty)
                    // 3553: vals:   l3395 = UNIQUE | NON_NULL, (empty)
                        -i
                    } else {
                        i
                    };
                    printf(b" %d\0" as *const u8 as *const libc::c_char, lit);
                    // 3558: b" %d\0" as *co ... _char: typeof(_2220) = *const {l3403} i8
                    // 3558: b" %d\0" as *co ... _char:   l3403 = UNIQUE | NON_NULL, (empty)
                    // 3558: b" %d\0" as *co ... st u8: typeof(_2221) = *const {l3405} u8
                    // 3558: b" %d\0" as *co ... st u8:   l3405 = UNIQUE | NON_NULL, (empty)
                    // 3558: b" %d\0": typeof(_2222) = *const {l3407} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 3558: b" %d\0":   l3407 = UNIQUE | NON_NULL, (empty)
                    // 3558: b" %d\0": typeof(_2223) = & {l3409} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 3558: b" %d\0":   l3409 = UNIQUE | NON_NULL, FIXED
                    // 3558: b" %d\0" as *co ... _char: typeof(_2220 = move _2221 as *const i8 (Misc)) = *const {l4286} i8
                    // 3558: b" %d\0" as *co ... _char:   l4286 = UNIQUE | NON_NULL, (empty)
                    // 3558: b" %d\0": typeof(_2222 = &raw const (*_2223)) = *const {l4284} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 3558: b" %d\0":   l4284 = UNIQUE | NON_NULL, (empty)
                    // 3558: b" %d\0": typeof(_2223 = const b" %d\x00") = & {l4283} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 3558: b" %d\0":   l4283 = UNIQUE | NON_NULL, (empty)
                    // 3558: b" %d\0" as *co ... st u8: typeof(_2221 = move _2222 as *const u8 (Pointer(ArrayToPointer))) = *const {l4285} u8
                    // 3558: b" %d\0" as *co ... st u8:   l4285 = UNIQUE | NON_NULL, (empty)
                    i += 1;
                    i;
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
                // 3562: b"\n\0" as *con ... _char: typeof(_2231) = *const {l3418} i8
                // 3562: b"\n\0" as *con ... _char:   l3418 = UNIQUE | NON_NULL, (empty)
                // 3562: b"\n\0" as *const u8: typeof(_2232) = *const {l3420} u8
                // 3562: b"\n\0" as *const u8:   l3420 = UNIQUE | NON_NULL, (empty)
                // 3562: b"\n\0": typeof(_2233) = *const {l3422} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 3562: b"\n\0":   l3422 = UNIQUE | NON_NULL, (empty)
                // 3562: b"\n\0": typeof(_2234) = & {l3424} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 3562: b"\n\0":   l3424 = UNIQUE | NON_NULL, FIXED
                // 3562: b"\n\0": typeof(_2234 = const b"\n\x00") = & {l4287} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 3562: b"\n\0":   l4287 = UNIQUE | NON_NULL, (empty)
                // 3562: b"\n\0": typeof(_2233 = &raw const (*_2234)) = *const {l4288} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 3562: b"\n\0":   l4288 = UNIQUE | NON_NULL, (empty)
                // 3562: b"\n\0" as *con ... _char: typeof(_2231 = move _2232 as *const i8 (Misc)) = *const {l4290} i8
                // 3562: b"\n\0" as *con ... _char:   l4290 = UNIQUE | NON_NULL, (empty)
                // 3562: b"\n\0" as *const u8: typeof(_2232 = move _2233 as *const u8 (Pointer(ArrayToPointer))) = *const {l4289} u8
                // 3562: b"\n\0" as *const u8:   l4289 = UNIQUE | NON_NULL, (empty)
            }
            printf(b"v 0\n\0" as *const u8 as *const libc::c_char);
            // 3564: b"v 0\n\0" as * ... _char: typeof(_2236) = *const {l3427} i8
            // 3564: b"v 0\n\0" as * ... _char:   l3427 = UNIQUE | NON_NULL, (empty)
            // 3564: b"v 0\n\0" as * ... st u8: typeof(_2237) = *const {l3429} u8
            // 3564: b"v 0\n\0" as * ... st u8:   l3429 = UNIQUE | NON_NULL, (empty)
            // 3564: b"v 0\n\0": typeof(_2238) = *const {l3431} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3564: b"v 0\n\0":   l3431 = UNIQUE | NON_NULL, (empty)
            // 3564: b"v 0\n\0": typeof(_2239) = & {l3433} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3564: b"v 0\n\0":   l3433 = UNIQUE | NON_NULL, FIXED
            // 3564: b"v 0\n\0": typeof(_2239 = const b"v 0\n\x00") = & {l4291} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3564: b"v 0\n\0":   l4291 = UNIQUE | NON_NULL, (empty)
            // 3564: b"v 0\n\0": typeof(_2238 = &raw const (*_2239)) = *const {l4292} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 3564: b"v 0\n\0":   l4292 = UNIQUE | NON_NULL, (empty)
            // 3564: b"v 0\n\0" as * ... st u8: typeof(_2237 = move _2238 as *const u8 (Pointer(ArrayToPointer))) = *const {l4293} u8
            // 3564: b"v 0\n\0" as * ... st u8:   l4293 = UNIQUE | NON_NULL, (empty)
            // 3564: b"v 0\n\0" as * ... _char: typeof(_2236 = move _2237 as *const i8 (Misc)) = *const {l4294} i8
            // 3564: b"v 0\n\0" as * ... _char:   l4294 = UNIQUE | NON_NULL, (empty)
        }
    } else if res == 20 as libc::c_int {
        printf(b"s UNSATISFIABLE\n\0" as *const u8 as *const libc::c_char);
        // 3567: b"s UNSATISFIAB ... _char: typeof(_2244) = *const {l3439} i8
        // 3567: b"s UNSATISFIAB ... _char:   l3439 = UNIQUE | NON_NULL, (empty)
        // 3567: b"s UNSATISFIAB ... st u8: typeof(_2245) = *const {l3441} u8
        // 3567: b"s UNSATISFIAB ... st u8:   l3441 = UNIQUE | NON_NULL, (empty)
        // 3567: b"s UNSATISFIAB ... \n\0": typeof(_2246) = *const {l3443} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 3567: b"s UNSATISFIAB ... \n\0":   l3443 = UNIQUE | NON_NULL, (empty)
        // 3567: b"s UNSATISFIAB ... \n\0": typeof(_2247) = & {l3445} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 3567: b"s UNSATISFIAB ... \n\0":   l3445 = UNIQUE | NON_NULL, FIXED
        // 3567: b"s UNSATISFIAB ... st u8: typeof(_2245 = move _2246 as *const u8 (Pointer(ArrayToPointer))) = *const {l4297} u8
        // 3567: b"s UNSATISFIAB ... st u8:   l4297 = UNIQUE | NON_NULL, (empty)
        // 3567: b"s UNSATISFIAB ... _char: typeof(_2244 = move _2245 as *const i8 (Misc)) = *const {l4298} i8
        // 3567: b"s UNSATISFIAB ... _char:   l4298 = UNIQUE | NON_NULL, (empty)
        // 3567: b"s UNSATISFIAB ... \n\0": typeof(_2247 = const b"s UNSATISFIABLE\n\x00") = & {l4295} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 3567: b"s UNSATISFIAB ... \n\0":   l4295 = UNIQUE | NON_NULL, (empty)
        // 3567: b"s UNSATISFIAB ... \n\0": typeof(_2246 = &raw const (*_2247)) = *const {l4296} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 3567: b"s UNSATISFIAB ... \n\0":   l4296 = UNIQUE | NON_NULL, (empty)
    } else {
        printf(b"c s UNKNOWN\n\0" as *const u8 as *const libc::c_char);
        // 3569: b"c s UNKNOWN\n ... _char: typeof(_2249) = *const {l3448} i8
        // 3569: b"c s UNKNOWN\n ... _char:   l3448 = UNIQUE | NON_NULL, (empty)
        // 3569: b"c s UNKNOWN\n ... st u8: typeof(_2250) = *const {l3450} u8
        // 3569: b"c s UNKNOWN\n ... st u8:   l3450 = UNIQUE | NON_NULL, (empty)
        // 3569: b"c s UNKNOWN\n\0": typeof(_2251) = *const {l3452} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 3569: b"c s UNKNOWN\n\0":   l3452 = UNIQUE | NON_NULL, (empty)
        // 3569: b"c s UNKNOWN\n\0": typeof(_2252) = & {l3454} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 3569: b"c s UNKNOWN\n\0":   l3454 = UNIQUE | NON_NULL, FIXED
        // 3569: b"c s UNKNOWN\n ... st u8: typeof(_2250 = move _2251 as *const u8 (Pointer(ArrayToPointer))) = *const {l4301} u8
        // 3569: b"c s UNKNOWN\n ... st u8:   l4301 = UNIQUE | NON_NULL, (empty)
        // 3569: b"c s UNKNOWN\n\0": typeof(_2252 = const b"c s UNKNOWN\n\x00") = & {l4299} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 3569: b"c s UNKNOWN\n\0":   l4299 = UNIQUE | NON_NULL, (empty)
        // 3569: b"c s UNKNOWN\n ... _char: typeof(_2249 = move _2250 as *const i8 (Misc)) = *const {l4302} i8
        // 3569: b"c s UNKNOWN\n ... _char:   l4302 = UNIQUE | NON_NULL, (empty)
        // 3569: b"c s UNKNOWN\n\0": typeof(_2251 = &raw const (*_2252)) = *const {l4300} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 3569: b"c s UNKNOWN\n\0":   l4300 = UNIQUE | NON_NULL, (empty)
    }
    fflush(stdout);
    // 3571: stdout: typeof(_2254) = *mut {l3457} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 3571: stdout:   l3457 = UNIQUE | NON_NULL, (empty)
    // 3571: stdout: typeof(_2255) = *mut {l3459} *mut {l3460} DefId(0:449 ~ plingeling[18f5]::_IO_FILE)
    // 3571: stdout:   l3459 = UNIQUE | NON_NULL, (empty)
    // 3571: stdout:   l3460 = UNIQUE | NON_NULL, (empty)
    if verbose != 0 {
    // 3572: verbose: typeof(_2259) = *mut {l3465} i32
    // 3572: verbose:   l3465 = UNIQUE | NON_NULL, (empty)
        i = 0 as libc::c_int;
        while i < nworkers {
        // 3574: nworkers: typeof(_2265) = *mut {l3472} i32
        // 3574: nworkers:   l3472 = UNIQUE | NON_NULL, (empty)
            if !((*workers.offset(i as isize)).lgl).is_null() {
            // 3575: ((*workers.offs ... .lgl): typeof(_2269) = *mut {l3477} LGL
            // 3575: ((*workers.offs ... .lgl):   l3477 = UNIQUE | NON_NULL, (empty)
            // 3575: workers.offset( ... size): typeof(_2270) = *mut {l3479} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3575: workers.offset( ... size):   l3479 = UNIQUE | NON_NULL, (empty)
            // 3575: workers: typeof(_2271) = *mut {l3481} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3575: workers:   l3481 = UNIQUE | NON_NULL, (empty)
            // 3575: workers: typeof(_2272) = *mut {l3483} *mut {l3484} DefId(0:535 ~ plingeling[18f5]::Worker)
            // 3575: workers:   l3483 = UNIQUE | NON_NULL, (empty)
            // 3575: workers:   l3484 = UNIQUE | NON_NULL, (empty)
                printf(
                    b"c\nc ------------[worker %d statistics]------------ \nc\n\0" as *const u8
                    // 3577: b"c\nc -------- ... _char: typeof(_2276) = *const {l3489} i8
                    // 3577: b"c\nc -------- ... _char:   l3489 = UNIQUE | NON_NULL, (empty)
                    // 3577: b"c\nc -------- ... st u8: typeof(_2277) = *const {l3491} u8
                    // 3577: b"c\nc -------- ... st u8:   l3491 = UNIQUE | NON_NULL, (empty)
                    // 3577: b"c\nc -------- ... \n\0": typeof(_2278) = *const {l3493} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000037)) }]
                    // 3577: b"c\nc -------- ... \n\0":   l3493 = UNIQUE | NON_NULL, (empty)
                    // 3577: b"c\nc -------- ... \n\0": typeof(_2279) = & {l3495} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000037)) }]
                    // 3577: b"c\nc -------- ... \n\0":   l3495 = UNIQUE | NON_NULL, FIXED
                    // 3577: b"c\nc -------- ... \n\0": typeof(_2278 = &raw const (*_2279)) = *const {l4304} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000037)) }]
                    // 3577: b"c\nc -------- ... \n\0":   l4304 = UNIQUE | NON_NULL, (empty)
                    // 3577: b"c\nc -------- ... _char: typeof(_2276 = move _2277 as *const i8 (Misc)) = *const {l4306} i8
                    // 3577: b"c\nc -------- ... _char:   l4306 = UNIQUE | NON_NULL, (empty)
                    // 3577: b"c\nc -------- ... st u8: typeof(_2277 = move _2278 as *const u8 (Pointer(ArrayToPointer))) = *const {l4305} u8
                    // 3577: b"c\nc -------- ... st u8:   l4305 = UNIQUE | NON_NULL, (empty)
                    // 3577: b"c\nc -------- ... \n\0": typeof(_2279 = const b"c\nc ------------[worker %d statistics]------------ \nc\n\x00") = & {l4303} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000037)) }]
                    // 3577: b"c\nc -------- ... \n\0":   l4303 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                    i,
                );
                lglstats((*workers.offset(i as isize)).lgl);
                // 3581: (*workers.offse ... ).lgl: typeof(_2282) = *mut {l3499} LGL
                // 3581: (*workers.offse ... ).lgl:   l3499 = UNIQUE | NON_NULL, (empty)
                // 3581: workers.offset( ... size): typeof(_2283) = *mut {l3501} DefId(0:535 ~ plingeling[18f5]::Worker)
                // 3581: workers.offset( ... size):   l3501 = UNIQUE | NON_NULL, (empty)
                // 3581: workers: typeof(_2284) = *mut {l3503} DefId(0:535 ~ plingeling[18f5]::Worker)
                // 3581: workers:   l3503 = UNIQUE | NON_NULL, (empty)
                // 3581: workers: typeof(_2285) = *mut {l3505} *mut {l3506} DefId(0:535 ~ plingeling[18f5]::Worker)
                // 3581: workers:   l3505 = UNIQUE | NON_NULL, (empty)
                // 3581: workers:   l3506 = UNIQUE | NON_NULL, (empty)
            }
            i += 1;
            i;
        }
        printf(
            b"c\nc -------------[overall statistics]------------- \nc\n\0" as *const u8
            // 3587: b"c\nc -------- ... _char: typeof(_2294) = *const {l3516} i8
            // 3587: b"c\nc -------- ... _char:   l3516 = UNIQUE | NON_NULL, (empty)
            // 3587: b"c\nc -------- ... st u8: typeof(_2295) = *const {l3518} u8
            // 3587: b"c\nc -------- ... st u8:   l3518 = UNIQUE | NON_NULL, (empty)
            // 3587: b"c\nc -------- ... \n\0": typeof(_2296) = *const {l3520} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000037)) }]
            // 3587: b"c\nc -------- ... \n\0":   l3520 = UNIQUE | NON_NULL, (empty)
            // 3587: b"c\nc -------- ... \n\0": typeof(_2297) = & {l3522} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000037)) }]
            // 3587: b"c\nc -------- ... \n\0":   l3522 = UNIQUE | NON_NULL, FIXED
            // 3587: b"c\nc -------- ... \n\0": typeof(_2296 = &raw const (*_2297)) = *const {l4308} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000037)) }]
            // 3587: b"c\nc -------- ... \n\0":   l4308 = UNIQUE | NON_NULL, (empty)
            // 3587: b"c\nc -------- ... _char: typeof(_2294 = move _2295 as *const i8 (Misc)) = *const {l4310} i8
            // 3587: b"c\nc -------- ... _char:   l4310 = UNIQUE | NON_NULL, (empty)
            // 3587: b"c\nc -------- ... \n\0": typeof(_2297 = const b"c\nc -------------[overall statistics]------------- \nc\n\x00") = & {l4307} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000037)) }]
            // 3587: b"c\nc -------- ... \n\0":   l4307 = UNIQUE | NON_NULL, (empty)
            // 3587: b"c\nc -------- ... st u8: typeof(_2295 = move _2296 as *const u8 (Pointer(ArrayToPointer))) = *const {l4309} u8
            // 3587: b"c\nc -------- ... st u8:   l4309 = UNIQUE | NON_NULL, (empty)
                as *const libc::c_char,
        );
    } else {
        printf(b"c\n\0" as *const u8 as *const libc::c_char);
        // 3591: b"c\n\0" as *co ... _char: typeof(_2299) = *const {l3525} i8
        // 3591: b"c\n\0" as *co ... _char:   l3525 = UNIQUE | NON_NULL, (empty)
        // 3591: b"c\n\0" as *co ... st u8: typeof(_2300) = *const {l3527} u8
        // 3591: b"c\n\0" as *co ... st u8:   l3527 = UNIQUE | NON_NULL, (empty)
        // 3591: b"c\n\0": typeof(_2301) = *const {l3529} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 3591: b"c\n\0":   l3529 = UNIQUE | NON_NULL, (empty)
        // 3591: b"c\n\0": typeof(_2302) = & {l3531} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 3591: b"c\n\0":   l3531 = UNIQUE | NON_NULL, FIXED
        // 3591: b"c\n\0": typeof(_2301 = &raw const (*_2302)) = *const {l4312} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 3591: b"c\n\0":   l4312 = UNIQUE | NON_NULL, (empty)
        // 3591: b"c\n\0" as *co ... st u8: typeof(_2300 = move _2301 as *const u8 (Pointer(ArrayToPointer))) = *const {l4313} u8
        // 3591: b"c\n\0" as *co ... st u8:   l4313 = UNIQUE | NON_NULL, (empty)
        // 3591: b"c\n\0": typeof(_2302 = const b"c\n\x00") = & {l4311} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 3591: b"c\n\0":   l4311 = UNIQUE | NON_NULL, (empty)
        // 3591: b"c\n\0" as *co ... _char: typeof(_2299 = move _2300 as *const i8 (Misc)) = *const {l4314} i8
        // 3591: b"c\n\0" as *co ... _char:   l4314 = UNIQUE | NON_NULL, (empty)
    }
    stats();
    if verbose >= 2 as libc::c_int {
    // 3594: verbose: typeof(_2307) = *mut {l3537} i32
    // 3594: verbose:   l3537 = UNIQUE | NON_NULL, (empty)
        printf(b"c\n\0" as *const u8 as *const libc::c_char);
        // 3595: b"c\n\0" as *co ... _char: typeof(_2310) = *const {l3541} i8
        // 3595: b"c\n\0" as *co ... _char:   l3541 = UNIQUE | NON_NULL, (empty)
        // 3595: b"c\n\0" as *co ... st u8: typeof(_2311) = *const {l3543} u8
        // 3595: b"c\n\0" as *co ... st u8:   l3543 = UNIQUE | NON_NULL, (empty)
        // 3595: b"c\n\0": typeof(_2312) = *const {l3545} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 3595: b"c\n\0":   l3545 = UNIQUE | NON_NULL, (empty)
        // 3595: b"c\n\0": typeof(_2313) = & {l3547} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 3595: b"c\n\0":   l3547 = UNIQUE | NON_NULL, FIXED
        // 3595: b"c\n\0": typeof(_2313 = const b"c\n\x00") = & {l4315} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 3595: b"c\n\0":   l4315 = UNIQUE | NON_NULL, (empty)
        // 3595: b"c\n\0" as *co ... _char: typeof(_2310 = move _2311 as *const i8 (Misc)) = *const {l4318} i8
        // 3595: b"c\n\0" as *co ... _char:   l4318 = UNIQUE | NON_NULL, (empty)
        // 3595: b"c\n\0": typeof(_2312 = &raw const (*_2313)) = *const {l4316} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 3595: b"c\n\0":   l4316 = UNIQUE | NON_NULL, (empty)
        // 3595: b"c\n\0" as *co ... st u8: typeof(_2311 = move _2312 as *const u8 (Pointer(ArrayToPointer))) = *const {l4317} u8
        // 3595: b"c\n\0" as *co ... st u8:   l4317 = UNIQUE | NON_NULL, (empty)
    }
    msg(
        -(1 as libc::c_int),
        2 as libc::c_int,
        b"releasing %d workers\0" as *const u8 as *const libc::c_char,
        // 3600: b"releasing %d  ... _char: typeof(_2319) = *const {l3554} i8
        // 3600: b"releasing %d  ... _char:   l3554 = UNIQUE | NON_NULL, (empty)
        // 3600: b"releasing %d  ... st u8: typeof(_2320) = *const {l3556} u8
        // 3600: b"releasing %d  ... st u8:   l3556 = UNIQUE | NON_NULL, (empty)
        // 3600: b"releasing %d  ... rs\0": typeof(_2321) = *const {l3558} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
        // 3600: b"releasing %d  ... rs\0":   l3558 = UNIQUE | NON_NULL, (empty)
        // 3600: b"releasing %d  ... rs\0": typeof(_2322) = & {l3560} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
        // 3600: b"releasing %d  ... rs\0":   l3560 = UNIQUE | NON_NULL, FIXED
        // 3600: b"releasing %d  ... st u8: typeof(_2320 = move _2321 as *const u8 (Pointer(ArrayToPointer))) = *const {l4321} u8
        // 3600: b"releasing %d  ... st u8:   l4321 = UNIQUE | NON_NULL, (empty)
        // 3600: b"releasing %d  ... rs\0": typeof(_2321 = &raw const (*_2322)) = *const {l4320} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
        // 3600: b"releasing %d  ... rs\0":   l4320 = UNIQUE | NON_NULL, (empty)
        // 3600: b"releasing %d  ... rs\0": typeof(_2322 = const b"releasing %d workers\x00") = & {l4319} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
        // 3600: b"releasing %d  ... rs\0":   l4319 = UNIQUE | NON_NULL, (empty)
        // 3600: b"releasing %d  ... _char: typeof(_2319 = move _2320 as *const i8 (Misc)) = *const {l4322} i8
        // 3600: b"releasing %d  ... _char:   l4322 = UNIQUE | NON_NULL, (empty)
        nworkers,
        // 3601: nworkers: typeof(_2324) = *mut {l3563} i32
        // 3601: nworkers:   l3563 = UNIQUE | NON_NULL, (empty)
    );
    i = 0 as libc::c_int;
    while i < nworkers {
    // 3604: nworkers: typeof(_2330) = *mut {l3570} i32
    // 3604: nworkers:   l3570 = UNIQUE | NON_NULL, (empty)
        w = workers.offset(i as isize);
        // 3605: workers.offset( ... size): typeof(_2331) = *mut {l3572} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3605: workers.offset( ... size):   l3572 = UNIQUE | NON_NULL, (empty)
        // 3605: workers: typeof(_2332) = *mut {l3574} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3605: workers:   l3574 = UNIQUE | NON_NULL, (empty)
        // 3605: workers: typeof(_2333) = *mut {l3576} *mut {l3577} DefId(0:535 ~ plingeling[18f5]::Worker)
        // 3605: workers:   l3576 = UNIQUE | NON_NULL, (empty)
        // 3605: workers:   l3577 = UNIQUE | NON_NULL, (empty)
        if !((*w).lgl).is_null() {
        // 3606: ((*w).lgl): typeof(_2339) = *mut {l3584} LGL
        // 3606: ((*w).lgl):   l3584 = UNIQUE | NON_NULL, (empty)
            lglrelease((*w).lgl);
            // 3607: (*w).lgl: typeof(_2341) = *mut {l3587} LGL
            // 3607: (*w).lgl:   l3587 = UNIQUE | NON_NULL, (empty)
            msg(
                -(1 as libc::c_int),
                2 as libc::c_int,
                b"released worker %d\0" as *const u8 as *const libc::c_char,
                // 3611: b"released work ... _char: typeof(_2347) = *const {l3594} i8
                // 3611: b"released work ... _char:   l3594 = UNIQUE | NON_NULL, (empty)
                // 3611: b"released work ... st u8: typeof(_2348) = *const {l3596} u8
                // 3611: b"released work ... st u8:   l3596 = UNIQUE | NON_NULL, (empty)
                // 3611: b"released work ... %d\0": typeof(_2349) = *const {l3598} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                // 3611: b"released work ... %d\0":   l3598 = UNIQUE | NON_NULL, (empty)
                // 3611: b"released work ... %d\0": typeof(_2350) = & {l3600} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                // 3611: b"released work ... %d\0":   l3600 = UNIQUE | NON_NULL, FIXED
                // 3611: b"released work ... st u8: typeof(_2348 = move _2349 as *const u8 (Pointer(ArrayToPointer))) = *const {l4325} u8
                // 3611: b"released work ... st u8:   l4325 = UNIQUE | NON_NULL, (empty)
                // 3611: b"released work ... %d\0": typeof(_2349 = &raw const (*_2350)) = *const {l4324} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                // 3611: b"released work ... %d\0":   l4324 = UNIQUE | NON_NULL, (empty)
                // 3611: b"released work ... _char: typeof(_2347 = move _2348 as *const i8 (Misc)) = *const {l4326} i8
                // 3611: b"released work ... _char:   l4326 = UNIQUE | NON_NULL, (empty)
                // 3611: b"released work ... %d\0": typeof(_2350 = const b"released worker %d\x00") = & {l4323} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                // 3611: b"released work ... %d\0":   l4323 = UNIQUE | NON_NULL, (empty)
                i,
            );
            if !((*w).dead).is_null() {
            // 3614: ((*w).dead): typeof(_2355) = *mut {l3606} DefId(0:525 ~ plingeling[18f5]::Cls)
            // 3614: ((*w).dead):   l3606 = UNIQUE | NON_NULL, (empty)
                deletecls_shim((*w).dead);
                // 3615: (*w).dead: typeof(_2357) = *mut {l3609} DefId(0:525 ~ plingeling[18f5]::Cls)
                // 3615: (*w).dead:   l3609 = UNIQUE | NON_NULL, (empty)
                (*w).dead = 0 as *mut Cls;
                // 3616: (*w).dead = 0 a ... t Cls: typeof(((*_14).9: *mut Cls) = const 0_usize as *mut Cls (PointerFromExposedAddress)) = *mut {l4327} DefId(0:525 ~ plingeling[18f5]::Cls)
                // 3616: (*w).dead = 0 a ... t Cls:   l4327 = UNIQUE | NON_NULL, (empty)
            }
            let mut BYTES_2: size_t = ((*w).szcls as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
            decmem(BYTES_2);
            free((*w).cls as *mut libc::c_void);
            // 3621: (*w).cls as *mu ... _void: typeof(_2366) = *mut {l3619} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3621: (*w).cls as *mu ... _void:   l3619 = UNIQUE | NON_NULL, (empty)
            // 3621: (*w).cls: typeof(_2367) = *mut {l3621} i32
            // 3621: (*w).cls:   l3621 = UNIQUE | NON_NULL, (empty)
            // 3621: (*w).cls as *mu ... _void: typeof(_2366 = move _2367 as *mut libc::c_void (Misc)) = *mut {l4328} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 3621: (*w).cls as *mu ... _void:   l4328 = UNIQUE | NON_NULL, (empty)
        }
        i += 1;
        i;
    }
    let mut BYTES_3: size_t =
        (nworkers as libc::c_ulong).wrapping_mul(::core::mem::size_of::<Worker>() as libc::c_ulong);
        // 3627: nworkers: typeof(_2376) = *mut {l3631} i32
        // 3627: nworkers:   l3631 = UNIQUE | NON_NULL, (empty)
    decmem(BYTES_3);
    free(workers as *mut libc::c_void);
    // 3629: workers as *mut ... _void: typeof(_2382) = *mut {l3638} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 3629: workers as *mut ... _void:   l3638 = UNIQUE | NON_NULL, (empty)
    // 3629: workers: typeof(_2383) = *mut {l3640} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 3629: workers:   l3640 = UNIQUE | NON_NULL, (empty)
    // 3629: workers: typeof(_2384) = *mut {l3642} *mut {l3643} DefId(0:535 ~ plingeling[18f5]::Worker)
    // 3629: workers:   l3642 = UNIQUE | NON_NULL, (empty)
    // 3629: workers:   l3643 = UNIQUE | NON_NULL, (empty)
    // 3629: workers as *mut ... _void: typeof(_2382 = move _2383 as *mut libc::c_void (Misc)) = *mut {l4329} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 3629: workers as *mut ... _void:   l4329 = UNIQUE | NON_NULL, (empty)
    let mut BYTES_4: size_t = ((nvars + 1 as libc::c_int) as libc::c_ulong)
    // 3630: nvars: typeof(_2389) = *mut {l3649} i32
    // 3630: nvars:   l3649 = UNIQUE | NON_NULL, (empty)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    decmem(BYTES_4);
    free(fixed as *mut libc::c_void);
    // 3633: fixed as *mut l ... _void: typeof(_2397) = *mut {l3658} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 3633: fixed as *mut l ... _void:   l3658 = UNIQUE | NON_NULL, (empty)
    // 3633: fixed: typeof(_2398) = *mut {l3660} i32
    // 3633: fixed:   l3660 = UNIQUE | NON_NULL, (empty)
    // 3633: fixed: typeof(_2399) = *mut {l3662} *mut {l3663} i32
    // 3633: fixed:   l3662 = UNIQUE | NON_NULL, (empty)
    // 3633: fixed:   l3663 = UNIQUE | NON_NULL, (empty)
    // 3633: fixed as *mut l ... _void: typeof(_2397 = move _2398 as *mut libc::c_void (Misc)) = *mut {l4330} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 3633: fixed as *mut l ... _void:   l4330 = UNIQUE | NON_NULL, (empty)
    if noeqs == 0 {
    // 3634: noeqs: typeof(_2403) = *mut {l3668} i32
    // 3634: noeqs:   l3668 = UNIQUE | NON_NULL, (empty)
        let mut BYTES_5: size_t = ((nvars + 1 as libc::c_int) as libc::c_ulong)
        // 3635: nvars: typeof(_2408) = *mut {l3674} i32
        // 3635: nvars:   l3674 = UNIQUE | NON_NULL, (empty)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
        decmem(BYTES_5);
        free(repr as *mut libc::c_void);
        // 3638: repr as *mut li ... _void: typeof(_2416) = *mut {l3683} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 3638: repr as *mut li ... _void:   l3683 = UNIQUE | NON_NULL, (empty)
        // 3638: repr: typeof(_2417) = *mut {l3685} i32
        // 3638: repr:   l3685 = UNIQUE | NON_NULL, (empty)
        // 3638: repr: typeof(_2418) = *mut {l3687} *mut {l3688} i32
        // 3638: repr:   l3687 = UNIQUE | NON_NULL, (empty)
        // 3638: repr:   l3688 = UNIQUE | NON_NULL, (empty)
        // 3638: repr as *mut li ... _void: typeof(_2416 = move _2417 as *mut libc::c_void (Misc)) = *mut {l4331} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 3638: repr as *mut li ... _void:   l4331 = UNIQUE | NON_NULL, (empty)
    }
    let mut BYTES_6: size_t = ((nvars + 1 as libc::c_int) as libc::c_ulong)
    // 3640: nvars: typeof(_2423) = *mut {l3694} i32
    // 3640: nvars:   l3694 = UNIQUE | NON_NULL, (empty)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    decmem(BYTES_6);
    free(vals as *mut libc::c_void);
    // 3643: vals as *mut li ... _void: typeof(_2431) = *mut {l3703} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 3643: vals as *mut li ... _void:   l3703 = UNIQUE | NON_NULL, (empty)
    // 3643: vals: typeof(_2432) = *mut {l3705} i32
    // 3643: vals:   l3705 = UNIQUE | NON_NULL, (empty)
    // 3643: vals: typeof(_2433) = *mut {l3707} *mut {l3708} i32
    // 3643: vals:   l3707 = UNIQUE | NON_NULL, (empty)
    // 3643: vals:   l3708 = UNIQUE | NON_NULL, (empty)
    // 3643: vals as *mut li ... _void: typeof(_2431 = move _2432 as *mut libc::c_void (Misc)) = *mut {l4332} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 3643: vals as *mut li ... _void:   l4332 = UNIQUE | NON_NULL, (empty)
    deleteallcls();
    return res;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    // 3648: mut args: typeof(_1) = DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l1} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
    // 3648: mut args:   l1 = UNIQUE | NON_NULL, (empty)
    for arg in ::std::env::args() {
    // 3649: ::std::env::args(): typeof(_9) = &mut {l10} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 3649: ::std::env::args():   l10 = UNIQUE | NON_NULL, (empty)
    // 3649: ::std::env::args(): typeof(_10) = &mut {l12} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 3649: ::std::env::args():   l12 = UNIQUE | NON_NULL, (empty)
    // 3649: ::std::env::args(): typeof(_10 = &mut _5) = &mut {l51} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 3649: ::std::env::args():   l51 = UNIQUE | NON_NULL, (empty)
    // 3649: ::std::env::args(): typeof(_9 = &mut (*_10)) = &mut {l52} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 3649: ::std::env::args():   l52 = UNIQUE | NON_NULL, (empty)
        args.push(
        // 3650: args.push( (::s ... (), ): typeof(_15) = &mut {l18} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l19} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
        // 3650: args.push( (::s ... (), ):   l18 = UNIQUE | NON_NULL, (empty)
        // 3650: args.push( (::s ... (), ):   l19 = UNIQUE | NON_NULL, (empty)
        // 3650: args.push( (::s ... (), ): typeof(_15 = &mut _1) = &mut {l53} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l54} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
        // 3650: args.push( (::s ... (), ):   l53 = UNIQUE | NON_NULL, (empty)
        // 3650: args.push( (::s ... (), ):   l54 = UNIQUE | NON_NULL, (empty)
            (::std::ffi::CString::new(arg))
            // 3651: (::std::ffi::CS ... raw(): typeof(_16) = *mut {l21} i8
            // 3651: (::std::ffi::CS ... raw():   l21 = UNIQUE | NON_NULL, (empty)
                .expect("Failed to convert argument into CString.")
                // 3652: "Failed to conv ... ing.": typeof(_20) = & {l26} str
                // 3652: "Failed to conv ... ing.":   l26 = UNIQUE | NON_NULL, (empty)
                // 3652: "Failed to conv ... ing.": typeof(_21) = & {l28} str
                // 3652: "Failed to conv ... ing.":   l28 = UNIQUE | NON_NULL, FIXED
                // 3652: "Failed to conv ... ing.": typeof(_21 = const "Failed to convert argument into CString.") = & {l55} str
                // 3652: "Failed to conv ... ing.":   l55 = UNIQUE | NON_NULL, (empty)
                // 3652: "Failed to conv ... ing.": typeof(_20 = &(*_21)) = & {l56} str
                // 3652: "Failed to conv ... ing.":   l56 = UNIQUE | NON_NULL, (empty)
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    // 3656: args.push(::cor ... ut()): typeof(_23) = &mut {l31} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l32} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
    // 3656: args.push(::cor ... ut()):   l31 = UNIQUE | NON_NULL, (empty)
    // 3656: args.push(::cor ... ut()):   l32 = UNIQUE | NON_NULL, (empty)
    // 3656: ::core::ptr::nu ... mut(): typeof(_24) = *mut {l34} i8
    // 3656: ::core::ptr::nu ... mut():   l34 = UNIQUE | NON_NULL, (empty)
    // 3656: args.push(::cor ... ut()): typeof(_23 = &mut _1) = &mut {l57} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l58} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
    // 3656: args.push(::cor ... ut()):   l57 = UNIQUE | NON_NULL, (empty)
    // 3656: args.push(::cor ... ut()):   l58 = UNIQUE | NON_NULL, (empty)
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            // 3659: args.len(): typeof(_30) = & {l41} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l42} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 3659: args.len():   l41 = UNIQUE | NON_NULL, (empty)
            // 3659: args.len():   l42 = UNIQUE | NON_NULL, (empty)
            // 3659: args.len(): typeof(_30 = &_1) = & {l59} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l60} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 3659: args.len():   l59 = UNIQUE | NON_NULL, (empty)
            // 3659: args.len():   l60 = UNIQUE | NON_NULL, (empty)
            args.as_mut_ptr() as *mut *mut libc::c_char,
            // 3660: args.as_mut_ptr ... _char: typeof(_32) = *mut {l45} *mut {l46} i8
            // 3660: args.as_mut_ptr ... _char:   l45 = UNIQUE | NON_NULL, (empty)
            // 3660: args.as_mut_ptr ... _char:   l46 = UNIQUE | NON_NULL, (empty)
            // 3660: args.as_mut_ptr(): typeof(_33) = &mut {l48} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l49} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 3660: args.as_mut_ptr():   l48 = UNIQUE | NON_NULL, (empty)
            // 3660: args.as_mut_ptr():   l49 = UNIQUE | NON_NULL, (empty)
            // 3660: args.as_mut_ptr(): typeof(_33 = &mut _1) = &mut {l61} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l62} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 3660: args.as_mut_ptr():   l61 = UNIQUE | NON_NULL, (empty)
            // 3660: args.as_mut_ptr():   l62 = UNIQUE | NON_NULL, (empty)
        ) as i32)
    }
}
