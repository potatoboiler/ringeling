#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types, label_break_value)]
extern crate libgeling;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type LGL;
    fn lglinit() -> *mut LGL;
    fn lglrelease(_: *mut LGL);
    fn lglsetopt(_: *mut LGL, _: *const libc::c_char, _: libc::c_int);
    fn lglfirstopt(_: *mut LGL) -> *mut libc::c_void;
    fn lglnextopt(
        _: *mut LGL,
        iterator: *mut libc::c_void,
        nameptr: *mut *const libc::c_char,
        valptr: *mut libc::c_int,
        minptr: *mut libc::c_int,
        maxptr: *mut libc::c_int,
    ) -> *mut libc::c_void;
    fn lglsetphase(_: *mut LGL, lit_0: libc::c_int);
    fn lglresetphase(_: *mut LGL, lit_0: libc::c_int);
    fn lglsetimportant(_: *mut LGL, lit_0: libc::c_int);
    fn lglsetphases(_: *mut LGL);
    fn lglonabort(
        _: *mut LGL,
        state: *mut libc::c_void,
        callback: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn lglwtrapi(_: *mut LGL, _: *mut FILE);
    fn lglmaxvar(_: *mut LGL) -> libc::c_int;
    fn lglincvar(_: *mut LGL) -> libc::c_int;
    fn lgladd(_: *mut LGL, lit_0: libc::c_int);
    fn lglassume(_: *mut LGL, lit_0: libc::c_int);
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::core::ffi::VaList) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn lglookahead(_: *mut LGL) -> libc::c_int;
    fn lglreuse(_: *mut LGL, lit_0: libc::c_int);
    fn lglreusable(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglusable(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglmelt(_: *mut LGL, lit_0: libc::c_int);
    fn lglfreeze(_: *mut LGL, lit_0: libc::c_int);
    fn lglrepr(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglflushcache(_: *mut LGL);
    fn lglreducecache(_: *mut LGL);
    fn lglchanged(_: *mut LGL) -> libc::c_int;
    fn lglinconsistent(_: *mut LGL) -> libc::c_int;
    fn lglfailed(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglfixed(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglderef(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglsimp(_: *mut LGL, iterations: libc::c_int) -> libc::c_int;
    fn lglsat(_: *mut LGL) -> libc::c_int;
    fn lglfixate(_: *mut LGL);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn fork() -> __pid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn times(__buffer: *mut tms) -> clock_t;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn lglchkclone(_: *mut LGL);
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
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
pub type pid_t = __pid_t;
pub type clock_t = __clock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tms {
    pub tms_utime: clock_t,
    pub tms_stime: clock_t,
    pub tms_cutime: clock_t,
    pub tms_cstime: clock_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(libc::c_int) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Data {
    pub lgl: *mut LGL,
    pub trace: *mut FILE,
    pub available: *mut libc::c_int,
    pub navailable: libc::c_int,
    pub frozen: *mut libc::c_int,
    pub nfrozen: libc::c_int,
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub c: libc::c_int,
    pub print: libc::c_int,
    pub noptsfuzz: libc::c_int,
}
pub type State = Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Event {
    pub state: State,
    pub rand: libc::c_uint,
    pub remove: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RNG {
    pub z: libc::c_uint,
    pub w: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Env {
    pub file: *mut FILE,
    pub prefix: *const libc::c_char,
    pub round: libc::c_int,
    pub bugs: libc::c_int,
    pub violations: libc::c_int,
    pub print: libc::c_int,
    pub quiet: libc::c_int,
    pub first: libc::c_int,
    pub nodd: libc::c_int,
    pub noptsfuzz: libc::c_int,
    pub alwaysfork: libc::c_int,
    pub terminal: libc::c_int,
    pub forked: libc::c_int,
    pub nevents: libc::c_int,
    pub timeout: libc::c_int,
    pub seed: libc::c_uint,
    pub events: *mut Event,
    pub rng: RNG,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
unsafe extern "C" fn initrng(mut seed: libc::c_uint) -> RNG {
    let mut res: RNG = RNG { z: 0, w: 0 };
    res.z = seed.wrapping_mul(1000632769 as libc::c_uint);
    res.w = seed.wrapping_mul(2019164533 as libc::c_uint);
    return res;
}
unsafe extern "C" fn nextrand(mut rng: *mut RNG) -> libc::c_uint {
    (*rng).z = (36969 as libc::c_int as libc::c_uint)
        .wrapping_mul((*rng).z & 65535 as libc::c_int as libc::c_uint)
        .wrapping_add((*rng).z >> 16 as libc::c_int);
    (*rng).w = (18000 as libc::c_int as libc::c_uint)
        .wrapping_mul((*rng).w & 65535 as libc::c_int as libc::c_uint)
        .wrapping_add((*rng).w >> 16 as libc::c_int);
    return ((*rng).z << 16 as libc::c_int).wrapping_add((*rng).w);
}
unsafe extern "C" fn pick(
    mut rng_ptr: *mut RNG,
    mut from: libc::c_uint,
    mut to: libc::c_uint,
) -> libc::c_int {
    let mut tmp: libc::c_uint = nextrand(rng_ptr);
    let mut res: libc::c_int = 0;
    if from <= to && to <= 2147483647 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(
            b"from <= to && to <= INT_MAX\0" as *const u8 as *const libc::c_char,
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"int pick(RNG *, unsigned int, unsigned int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_4591: {
        if from <= to && to <= 2147483647 as libc::c_int as libc::c_uint {
        } else {
            __assert_fail(
                b"from <= to && to <= INT_MAX\0" as *const u8 as *const libc::c_char,
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                85 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                    b"int pick(RNG *, unsigned int, unsigned int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    tmp = tmp.wrapping_rem(
        to.wrapping_sub(from)
            .wrapping_add(1 as libc::c_int as libc::c_uint),
    );
    tmp = tmp.wrapping_add(from);
    res = tmp as libc::c_int;
    return res;
}
unsafe extern "C" fn onabort(mut dummy: *mut libc::c_void) {
    exit(42 as libc::c_int);
}
unsafe extern "C" fn init(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
    let mut rng: RNG = initrng(r);
    let mut t: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    (*data).m = pick(
        &mut rng,
        10 as libc::c_int as libc::c_uint,
        200 as libc::c_int as libc::c_uint,
    );
    t = pick(
        &mut rng,
        390 as libc::c_int as libc::c_uint,
        450 as libc::c_int as libc::c_uint,
    );
    (*data).n = (*data).m * t / 100 as libc::c_int;
    if (*data).print != 0 {
        printf(
            b"cnf %d %d \0" as *const u8 as *const libc::c_char,
            (*data).m,
            (*data).n,
        );
        fflush(stdout);
    }
    (*data).lgl = lglinit();
    lglonabort(
        (*data).lgl,
        0 as *mut libc::c_void,
        Some(onabort as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    if !((*data).trace).is_null() {
        lglwtrapi((*data).lgl, (*data).trace);
    }
    (*data).navailable = (*data).m;
    (*data).available = malloc(
        ((*data).navailable as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < (*data).navailable {
        *((*data).available).offset(i as isize) = i + 1 as libc::c_int;
        i += 1;
        i;
    }
    if (*data).noptsfuzz != 0 {
        return ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
            *mut libc::c_void,
        >(Some(
            cnf as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
        ));
    }
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(
        if pick(
            &mut rng,
            0 as libc::c_int as libc::c_uint,
            1 as libc::c_int as libc::c_uint,
        ) != 0
        {
            Some(opts as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void)
        } else {
            Some(cnf as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void)
        },
    );
}
unsafe extern "C" fn inc(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut oldavailable: libc::c_int = 0;
    let mut newvars: libc::c_int = 0;
    let mut rng: RNG = RNG { z: 0, w: 0 };
    rng = initrng(r);
    if pick(
        &mut rng,
        0 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
    ) != 0
    {
        lglmaxvar((*data).lgl);
    }
    while pick(
        &mut rng,
        0 as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
    ) != 0
    {
        lglfixed(
            (*data).lgl,
            pick(
                &mut rng,
                1 as libc::c_int as libc::c_uint,
                (*data).m as libc::c_uint,
            ),
        );
    }
    while pick(
        &mut rng,
        0 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
    ) != 0
    {
        lglrepr(
            (*data).lgl,
            pick(
                &mut rng,
                1 as libc::c_int as libc::c_uint,
                (*data).m as libc::c_uint,
            ),
        );
    }
    newvars = pick(
        &mut rng,
        0 as libc::c_int as libc::c_uint,
        50 as libc::c_int as libc::c_uint,
    );
    if newvars != 0 {
        if pick(
            &mut rng,
            0 as libc::c_int as libc::c_uint,
            1 as libc::c_int as libc::c_uint,
        ) != 0
        {
            lglincvar((*data).lgl);
        }
        oldavailable = (*data).navailable;
        (*data).navailable += newvars;
        (*data).available = realloc(
            (*data).available as *mut libc::c_void,
            ((*data).navailable as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        i = 0 as libc::c_int;
        while i < newvars {
            *((*data).available).offset((oldavailable + i) as isize) =
                (*data).m + i + 1 as libc::c_int;
            i += 1;
            i;
        }
        (*data).m += newvars;
    }
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(Some(
        cnf as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
    ));
}
unsafe extern "C" fn opts(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut min: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut it: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut rng: RNG = initrng(r);
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if pick(
        &mut rng,
        0 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
    ) == 0
    {
        lglsetopt(
            (*data).lgl,
            b"plain\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    } else {
        it = lglfirstopt((*data).lgl);
        loop {
            it = lglnextopt((*data).lgl, it, &mut name, &mut val, &mut min, &mut max);
            if it.is_null() {
                break;
            }
            n += 1;
            n;
        }
        if n > 0 as libc::c_int {
        } else {
            __assert_fail(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                148 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                    b"void *opts(Data *, unsigned int)\0",
                ))
                .as_ptr(),
            );
        }
        'c_7012: {
            if n > 0 as libc::c_int {
            } else {
                __assert_fail(
                    b"n > 0\0" as *const u8 as *const libc::c_char,
                    b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                    148 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                        b"void *opts(Data *, unsigned int)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        m = pick(
            &mut rng,
            1 as libc::c_int as libc::c_uint,
            10 as libc::c_int as libc::c_uint,
        );
        it = lglfirstopt((*data).lgl);
        loop {
            it = lglnextopt((*data).lgl, it, &mut name, &mut val, &mut min, &mut max);
            if it.is_null() {
                break;
            }
            if pick(
                &mut rng,
                1 as libc::c_int as libc::c_uint,
                m as libc::c_uint,
            ) != 1 as libc::c_int
            {
                continue;
            }
            if strcmp(name, b"check\0" as *const u8 as *const libc::c_char) == 0 {
                continue;
            }
            if strcmp(name, b"drupligtrace\0" as *const u8 as *const libc::c_char) == 0 {
                continue;
            }
            if strcmp(name, b"log\0" as *const u8 as *const libc::c_char) == 0 {
                continue;
            }
            if strcmp(name, b"sleeponabort\0" as *const u8 as *const libc::c_char) == 0 {
                continue;
            }
            if strcmp(name, b"exitonabort\0" as *const u8 as *const libc::c_char) == 0 {
                continue;
            }
            if strcmp(name, b"verbose\0" as *const u8 as *const libc::c_char) == 0 {
                continue;
            }
            if strcmp(name, b"witness\0" as *const u8 as *const libc::c_char) == 0 {
                continue;
            }
            if strcmp(name, b"prune\0" as *const u8 as *const libc::c_char) == 0 {
                continue;
            }
            if pick(
                &mut rng,
                0 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
            ) != 0
            {
                if strcmp(name, b"locsmaxeff\0" as *const u8 as *const libc::c_char) == 0 {
                    max = 10 as libc::c_int * min;
                }
                if strcmp(name, b"locsrtc\0" as *const u8 as *const libc::c_char) == 0 {
                    max = 0 as libc::c_int;
                }
                while pick(
                    &mut rng,
                    0 as libc::c_int as libc::c_uint,
                    1 as libc::c_int as libc::c_uint,
                ) != 0
                    && val < max
                {
                    if val > 2147483647 as libc::c_int / 2 as libc::c_int {
                        break;
                    }
                    if val < 4 as libc::c_int {
                        val += 1;
                        val;
                    } else {
                        val *= 2 as libc::c_int;
                    }
                }
                if val > max {
                    val = max;
                }
            } else {
                while pick(
                    &mut rng,
                    0 as libc::c_int as libc::c_uint,
                    1 as libc::c_int as libc::c_uint,
                ) != 0
                    && val > min
                {
                    if val > 0 as libc::c_int {
                        val /= 2 as libc::c_int;
                    } else if val > -(4 as libc::c_int) {
                        val -= 1;
                        val;
                    } else {
                        if val
                            < (-(2147483647 as libc::c_int) - 1 as libc::c_int) / 2 as libc::c_int
                        {
                            break;
                        }
                        val *= 2 as libc::c_int;
                    }
                }
                if val < min {
                    val = min;
                }
            }
            lglsetopt((*data).lgl, name, val);
        }
    }
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(Some(
        cnf as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
    ));
}
unsafe extern "C" fn cnf(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(if (*data).c < (*data).n {
        Some(unit as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void)
    } else {
        Some(lkhd as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void)
    });
}
unsafe extern "C" fn lit(mut data: *mut Data, mut r: *mut RNG) -> libc::c_int {
    let mut pos: libc::c_int = pick(
        r,
        0 as libc::c_int as libc::c_uint,
        ((*data).navailable - 1 as libc::c_int) as libc::c_uint,
    );
    let mut res: libc::c_int = *((*data).available).offset(pos as isize);
    if (0 as libc::c_int) < res && res <= (*data).m {
    } else {
        __assert_fail(
            b"0 < res && res <= data->m\0" as *const u8 as *const libc::c_char,
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            198 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"int lit(Data *, RNG *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_6496: {
        if (0 as libc::c_int) < res && res <= (*data).m {
        } else {
            __assert_fail(
                b"0 < res && res <= data->m\0" as *const u8 as *const libc::c_char,
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                198 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                    b"int lit(Data *, RNG *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if pick(
        r,
        0 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
    ) != 0
    {
        res = -res;
    }
    return res;
}
unsafe extern "C" fn unit(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
    let mut rng: RNG = initrng(r);
    lgladd((*data).lgl, lit(data, &mut rng));
    if pick(
        &mut rng,
        0 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
    ) == 0
    {
        return ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
            *mut libc::c_void,
        >(Some(
            clause as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
        ));
    }
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(Some(
        binary as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
    ));
}
unsafe extern "C" fn binary(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
    let mut rng: RNG = initrng(r);
    lgladd((*data).lgl, lit(data, &mut rng));
    if pick(
        &mut rng,
        0 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
    ) == 0
    {
        return ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
            *mut libc::c_void,
        >(Some(
            clause as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
        ));
    }
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(Some(
        ternary as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
    ));
}
unsafe extern "C" fn ternary(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
    let mut rng: RNG = initrng(r);
    lgladd((*data).lgl, lit(data, &mut rng));
    if pick(
        &mut rng,
        0 as libc::c_int as libc::c_uint,
        2 as libc::c_int as libc::c_uint,
    ) != 0
    {
        return ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
            *mut libc::c_void,
        >(Some(
            clause as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
        ));
    }
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(Some(
        rest as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
    ));
}
unsafe extern "C" fn rest(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
    let mut rng: RNG = initrng(r);
    lgladd((*data).lgl, lit(data, &mut rng));
    if pick(
        &mut rng,
        0 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
    ) != 0
    {
        return ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
            *mut libc::c_void,
        >(Some(
            clause as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
        ));
    }
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(Some(
        rest as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
    ));
}
unsafe extern "C" fn clause(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
    lgladd((*data).lgl, 0 as libc::c_int);
    (*data).c += 1 as libc::c_int;
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(Some(
        cnf as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
    ));
}
unsafe extern "C" fn gcd(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if a > 0 as libc::c_int && b > 0 as libc::c_int {
    } else {
        __assert_fail(
            b"a > 0 && b > 0\0" as *const u8 as *const libc::c_char,
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            240 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"int gcd(int, int)\0"))
                .as_ptr(),
        );
    }
    'c_6163: {
        if a > 0 as libc::c_int && b > 0 as libc::c_int {
        } else {
            __assert_fail(
                b"a > 0 && b > 0\0" as *const u8 as *const libc::c_char,
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                240 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"int gcd(int, int)\0"))
                    .as_ptr(),
            );
        }
    };
    while b > 0 as libc::c_int {
        r = a % b;
        a = b;
        b = r;
    }
    return a;
}
unsafe extern "C" fn lkhd(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
    let mut rng: RNG = initrng(r);
    if pick(
        &mut rng,
        0 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
    ) == 0
    {
        lglookahead((*data).lgl);
    }
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(Some(
        sat as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
    ));
}
unsafe extern "C" fn sat(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
    let mut res: libc::c_int = 0;
    let mut freeze: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    let mut lit_0: libc::c_int = 0;
    let mut assumed: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nassumed: libc::c_int = 0;
    let mut szassumed: libc::c_int = 0;
    let mut lgl: *mut LGL = (*data).lgl;
    let mut next: State =
        Some(release as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void);
    let mut rng: RNG = RNG { z: 0, w: 0 };
    rng = initrng(r);
    if pick(
        &mut rng,
        0 as libc::c_int as libc::c_uint,
        500 as libc::c_int as libc::c_uint,
    ) == 0
    {
        lglchkclone(lgl);
    }
    freeze = pick(
        &mut rng,
        0 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
    );
    if freeze != 0 {
        if (*data).navailable > 1 as libc::c_int {
            (*data).nfrozen = pick(
                &mut rng,
                (((*data).navailable + 9 as libc::c_int) / 10 as libc::c_int) as libc::c_uint,
                (2 as libc::c_int * ((*data).navailable + 2 as libc::c_int) / 3 as libc::c_int)
                    as libc::c_uint,
            );
            (*data).frozen = malloc(
                ((*data).nfrozen as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as *mut libc::c_int;
            delta = pick(
                &mut rng,
                1 as libc::c_int as libc::c_uint,
                ((*data).navailable - 1 as libc::c_int) as libc::c_uint,
            );
            while gcd((*data).navailable, delta) != 1 as libc::c_int {
                delta += 1;
                if delta == (*data).navailable {
                    delta = 1 as libc::c_int;
                }
            }
            pos = pick(
                &mut rng,
                0 as libc::c_int as libc::c_uint,
                ((*data).navailable - 1 as libc::c_int) as libc::c_uint,
            );
            i = 0 as libc::c_int;
            while i < (*data).nfrozen {
                if 0 as libc::c_int <= pos && pos < (*data).navailable {
                } else {
                    __assert_fail(
                        b"0 <= pos && pos < data->navailable\0" as *const u8 as *const libc::c_char,
                        b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                        273 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                            b"void *sat(Data *, unsigned int)\0",
                        ))
                        .as_ptr(),
                    );
                }
                'c_6026: {
                    if 0 as libc::c_int <= pos && pos < (*data).navailable {
                    } else {
                        __assert_fail(
                            b"0 <= pos && pos < data->navailable\0" as *const u8
                                as *const libc::c_char,
                            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                            273 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                                b"void *sat(Data *, unsigned int)\0",
                            ))
                            .as_ptr(),
                        );
                    }
                };
                lit_0 = *((*data).available).offset(pos as isize);
                *((*data).frozen).offset(i as isize) = lit_0;
                pos += delta;
                if pos >= (*data).navailable {
                    pos -= (*data).navailable;
                }
                i += 1;
                i;
            }
            i = 0 as libc::c_int;
            while i < (*data).nfrozen {
                lglfreeze(lgl, *((*data).frozen).offset(i as isize));
                i += 1;
                i;
            }
        } else if (*data).navailable == 1 as libc::c_int {
            (*data).nfrozen = 1 as libc::c_int;
            (*data).frozen =
                malloc(::core::mem::size_of::<libc::c_int>() as libc::c_ulong) as *mut libc::c_int;
            *((*data).frozen).offset(0 as libc::c_int as isize) =
                *((*data).available).offset(0 as libc::c_int as isize);
        } else {
            (*data).nfrozen = 0 as libc::c_int;
            (*data).frozen = 0 as *mut libc::c_int;
        }
    }
    if (*data).navailable != 0
        && pick(
            &mut rng,
            0 as libc::c_int as libc::c_uint,
            3 as libc::c_int as libc::c_uint,
        ) == 0
    {
        nassumed = 0 as libc::c_int;
        szassumed = 1 as libc::c_int;
        assumed = malloc(
            (szassumed as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        loop {
            if nassumed == szassumed {
                szassumed *= 2 as libc::c_int;
                assumed = realloc(
                    assumed as *mut libc::c_void,
                    (szassumed as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
                ) as *mut libc::c_int;
            }
            pos = pick(
                &mut rng,
                0 as libc::c_int as libc::c_uint,
                ((*data).navailable - 1 as libc::c_int) as libc::c_uint,
            );
            lit_0 = *((*data).available).offset(pos as isize);
            if pick(
                &mut rng,
                0 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
            ) != 0
            {
                lit_0 = -lit_0;
            }
            lglassume(lgl, lit_0);
            if nassumed < szassumed {
            } else {
                __assert_fail(
                    b"nassumed < szassumed\0" as *const u8 as *const libc::c_char,
                    b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                    312 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                        b"void *sat(Data *, unsigned int)\0",
                    ))
                    .as_ptr(),
                );
            }
            'c_5735: {
                if nassumed < szassumed {
                } else {
                    __assert_fail(
                        b"nassumed < szassumed\0" as *const u8 as *const libc::c_char,
                        b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                        312 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                            b"void *sat(Data *, unsigned int)\0",
                        ))
                        .as_ptr(),
                    );
                }
            };
            let fresh0 = nassumed;
            nassumed = nassumed + 1;
            *assumed.offset(fresh0 as isize) = lit_0;
            if !(pick(
                &mut rng,
                0 as libc::c_int as libc::c_uint,
                10 as libc::c_int as libc::c_uint,
            ) == 0)
            {
                break;
            }
        }
    } else {
        assumed = 0 as *mut libc::c_int;
        szassumed = 0 as libc::c_int;
        nassumed = szassumed;
    }
    if pick(
        &mut rng,
        0 as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
    ) == 0
    {
        pos = pick(
            &mut rng,
            0 as libc::c_int as libc::c_uint,
            ((*data).navailable - 1 as libc::c_int) as libc::c_uint,
        );
        lit_0 = *((*data).available).offset(pos as isize);
        if pick(
            &mut rng,
            0 as libc::c_int as libc::c_uint,
            1 as libc::c_int as libc::c_uint,
        ) != 0
        {
            lit_0 = -lit_0;
        }
        if pick(
            &mut rng,
            0 as libc::c_int as libc::c_uint,
            3 as libc::c_int as libc::c_uint,
        ) == 0
        {
            lglresetphase(lgl, lit_0);
        } else {
            lglsetphase(lgl, lit_0);
        }
        if pick(
            &mut rng,
            0 as libc::c_int as libc::c_uint,
            11 as libc::c_int as libc::c_uint,
        ) == 0
        {
            lglsetimportant(lgl, lit_0);
        }
    }
    if pick(
        &mut rng,
        0 as libc::c_int as libc::c_uint,
        100 as libc::c_int as libc::c_uint,
    ) == 0
    {
        lglchkclone(lgl);
    }
    if pick(
        &mut rng,
        0 as libc::c_int as libc::c_uint,
        66 as libc::c_int as libc::c_uint,
    ) == 0
    {
        lglfixate(lgl);
    }
    if pick(
        &mut rng,
        0 as libc::c_int as libc::c_uint,
        20 as libc::c_int as libc::c_uint,
    ) != 0
    {
        res = lglsat(lgl);
    } else {
        res = lglsimp(
            lgl,
            pick(
                &mut rng,
                0 as libc::c_int as libc::c_uint,
                10 as libc::c_int as libc::c_uint,
            ),
        );
    }
    if res == 0 || res == 10 as libc::c_int || res == 20 as libc::c_int {
    } else {
        __assert_fail(
            b"!res || res == 10 || res == 20\0" as *const u8 as *const libc::c_char,
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            328 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"void *sat(Data *, unsigned int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_5469: {
        if res == 0 || res == 10 as libc::c_int || res == 20 as libc::c_int {
        } else {
            __assert_fail(
                b"!res || res == 10 || res == 20\0" as *const u8 as *const libc::c_char,
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                328 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    b"void *sat(Data *, unsigned int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if res == 10 as libc::c_int {
        if (*data).print != 0 {
            printf(b"sat \0" as *const u8 as *const libc::c_char);
        }
        if pick(
            &mut rng,
            0 as libc::c_int as libc::c_uint,
            4 as libc::c_int as libc::c_uint,
        ) == 0
        {
            lglinconsistent(lgl);
        }
        if pick(
            &mut rng,
            0 as libc::c_int as libc::c_uint,
            20 as libc::c_int as libc::c_uint,
        ) == 0
        {
            lglsetphases(lgl);
        }
        i = pick(
            &mut rng,
            0 as libc::c_int as libc::c_uint,
            (*data).m as libc::c_uint,
        );
        loop {
            let fresh1 = i;
            i = i - 1;
            if !(fresh1 != 0) {
                break;
            }
            lit_0 = pick(
                &mut rng,
                1 as libc::c_int as libc::c_uint,
                (2 as libc::c_int * (*data).m) as libc::c_uint,
            );
            if pick(
                &mut rng,
                0 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
            ) != 0
            {
                lit_0 = -lit_0;
            }
            lglderef(lgl, lit_0);
        }
        if pick(
            &mut rng,
            0 as libc::c_int as libc::c_uint,
            30 as libc::c_int as libc::c_uint,
        ) == 0
        {
            lglsetphases(lgl);
        }
        if freeze != 0 {
            if (*data).nfrozen <= (*data).navailable {
            } else {
                __assert_fail(
                    b"data->nfrozen <= data->navailable\0" as *const u8 as *const libc::c_char,
                    b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                    341 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                        b"void *sat(Data *, unsigned int)\0",
                    ))
                    .as_ptr(),
                );
            }
            'c_5301: {
                if (*data).nfrozen <= (*data).navailable {
                } else {
                    __assert_fail(
                        b"data->nfrozen <= data->navailable\0" as *const u8 as *const libc::c_char,
                        b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                        341 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                            b"void *sat(Data *, unsigned int)\0",
                        ))
                        .as_ptr(),
                    );
                }
            };
            (*data).navailable = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < (*data).nfrozen {
                lit_0 = *((*data).frozen).offset(i as isize);
                let mut current_block_94: u64;
                match i % 5 as libc::c_int {
                    0 => {
                        lglmelt(lgl, lit_0);
                        current_block_94 = 11046353700707405348;
                    }
                    1 => {
                        current_block_94 = 11046353700707405348;
                    }
                    2 => {
                        (*data).m += 1;
                        *((*data).available).offset((*data).navailable as isize) = (*data).m;
                        (*data).navailable += 1;
                        (*data).navailable;
                        current_block_94 = 17485376261910781866;
                    }
                    3 => {
                        lglmelt(lgl, lit_0);
                        current_block_94 = 17485376261910781866;
                    }
                    _ => {
                        current_block_94 = 17485376261910781866;
                    }
                }
                match current_block_94 {
                    11046353700707405348 => {
                        *((*data).available).offset((*data).navailable as isize) = lit_0;
                        (*data).navailable += 1;
                        (*data).navailable;
                    }
                    _ => {}
                }
                i += 1;
                i;
            }
            free((*data).frozen as *mut libc::c_void);
        }
        if freeze >= 2 as libc::c_int {
            (*data).n = (pick(
                &mut rng,
                101 as libc::c_int as libc::c_uint,
                130 as libc::c_int as libc::c_uint,
            ) * (*data).n
                + 99 as libc::c_int)
                / 100 as libc::c_int;
            next = Some(inc as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void);
        }
        if pick(
            &mut rng,
            0 as libc::c_int as libc::c_uint,
            4 as libc::c_int as libc::c_uint,
        ) == 0
        {
            lglchanged(lgl);
        }
        if pick(
            &mut rng,
            0 as libc::c_int as libc::c_uint,
            3 as libc::c_int as libc::c_uint,
        ) == 0
        {
            let mut count: libc::c_int = pick(
                &mut rng,
                1 as libc::c_int as libc::c_uint,
                ((*data).m / 10 as libc::c_int) as libc::c_uint,
            );
            let mut i_0: libc::c_int = 0;
            i_0 = 0 as libc::c_int;
            while i_0 < count {
                let mut lit_1: libc::c_int = pick(
                    &mut rng,
                    1 as libc::c_int as libc::c_uint,
                    (*data).m as libc::c_uint,
                );
                if lglusable(lgl, lit_1) == 0 && lglreusable(lgl, lit_1) != 0 {
                    lglreuse(lgl, lit_1);
                }
                i_0 += 1;
                i_0;
            }
        }
    } else if res == 20 as libc::c_int {
        if (*data).print != 0 {
            printf(b"uns \0" as *const u8 as *const libc::c_char);
        }
        if nassumed > 0 as libc::c_int {
            i = pick(
                &mut rng,
                0 as libc::c_int as libc::c_uint,
                (3 as libc::c_int * nassumed / 2 as libc::c_int) as libc::c_uint,
            );
            loop {
                let fresh2 = i;
                i = i - 1;
                if !(fresh2 > 0 as libc::c_int) {
                    break;
                }
                lglfailed(
                    (*data).lgl,
                    *assumed.offset(pick(
                        &mut rng,
                        0 as libc::c_int as libc::c_uint,
                        (nassumed - 1 as libc::c_int) as libc::c_uint,
                    ) as isize),
                );
            }
        }
        if pick(
            &mut rng,
            0 as libc::c_int as libc::c_uint,
            4 as libc::c_int as libc::c_uint,
        ) == 0
        {
            lglinconsistent(lgl);
        }
    } else if (*data).print != 0 {
        printf(b"nil \0" as *const u8 as *const libc::c_char);
    }
    if (*data).print != 0 {
        fflush(stdout);
    }
    if pick(
        &mut rng,
        0 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
    ) == 0
    {
        lglreducecache(lgl);
    } else if pick(
        &mut rng,
        0 as libc::c_int as libc::c_uint,
        17 as libc::c_int as libc::c_uint,
    ) == 0
    {
        lglflushcache(lgl);
    }
    if pick(
        &mut rng,
        0 as libc::c_int as libc::c_uint,
        1000 as libc::c_int as libc::c_uint,
    ) == 0
    {
        lglchkclone(lgl);
    }
    free(assumed as *mut libc::c_void);
    return ::core::mem::transmute::<State, *mut libc::c_void>(next);
}
unsafe extern "C" fn release(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
    lglrelease((*data).lgl);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn rantrav(mut env_0: *mut Env) {
    let mut state: State = None;
    let mut next: State = None;
    let mut rand: libc::c_uint = 0;
    let mut data: Data = Data {
        lgl: 0 as *mut LGL,
        trace: 0 as *mut FILE,
        available: 0 as *mut libc::c_int,
        navailable: 0,
        frozen: 0 as *mut libc::c_int,
        nfrozen: 0,
        m: 0,
        n: 0,
        c: 0,
        print: 0,
        noptsfuzz: 0,
    };
    memset(
        &mut data as *mut Data as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Data>() as libc::c_ulong,
    );
    data.print = (*env_0).print;
    (*env_0).rng.w = (*env_0).seed;
    (*env_0).rng.z = (*env_0).rng.w;
    state = Some(init as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void);
    while state.is_some() {
        rand = nextrand(&mut (*env_0).rng);
        if !((*env_0).file).is_null() {
            fprintf(
                (*env_0).file,
                b"%p %x\n\0" as *const u8 as *const libc::c_char,
                state,
                rand,
            );
            fflush((*env_0).file);
        }
        next = ::core::mem::transmute::<*mut libc::c_void, State>(state
            .expect("non-null function pointer")(
            &mut data, rand
        ));
        state = next;
    }
}
unsafe extern "C" fn erase() {
    let mut i: libc::c_int = 0;
    fputc('\r' as i32, stdout);
    i = 0 as libc::c_int;
    while i < 79 as libc::c_int {
        fputc(' ' as i32, stdout);
        i += 1;
        i;
    }
    fputc('\r' as i32, stdout);
}
unsafe extern "C" fn isnumstr(mut str: *const libc::c_char) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = str;
    while *p != 0 {
        if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            return 0 as libc::c_int;
        }
        p = p.offset(1);
        p;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn die(mut msg: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    fputs(
        b"*** lglmbt: \0" as *const u8 as *const libc::c_char,
        stderr,
    );
    ap = args.clone();
    vfprintf(stderr, msg, ap.as_va_list());
    fputc('\n' as i32, stderr);
    fflush(stderr);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn run(
    mut env_0: *mut Env,
    mut process: Option<unsafe extern "C" fn(*mut Env) -> ()>,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut saved1: libc::c_int = 0;
    let mut saved2: libc::c_int = 0;
    let mut null: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut id: pid_t = 0;
    (*env_0).forked += 1;
    (*env_0).forked;
    fflush(stdout);
    id = fork();
    if id != 0 {
        let mut wid: pid_t = wait(&mut status);
        if wid == id {
        } else {
            __assert_fail(
                b"wid == id\0" as *const u8 as *const libc::c_char,
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                455 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    b"int run(Env *, void (*)(Env *))\0",
                ))
                .as_ptr(),
            );
        }
        'c_7963: {
            if wid == id {
            } else {
                __assert_fail(
                    b"wid == id\0" as *const u8 as *const libc::c_char,
                    b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                    455 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                        b"int run(Env *, void (*)(Env *))\0",
                    ))
                    .as_ptr(),
                );
            }
        };
    } else {
        saved1 = dup(1 as libc::c_int);
        saved2 = dup(2 as libc::c_int);
        null = open(
            b"/dev/null\0" as *const u8 as *const libc::c_char,
            0o1 as libc::c_int,
        );
        close(1 as libc::c_int);
        close(2 as libc::c_int);
        tmp = dup(null);
        if tmp == 1 as libc::c_int {
        } else {
            __assert_fail(
                b"tmp == 1\0" as *const u8 as *const libc::c_char,
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                466 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    b"int run(Env *, void (*)(Env *))\0",
                ))
                .as_ptr(),
            );
        }
        'c_7887: {
            if tmp == 1 as libc::c_int {
            } else {
                __assert_fail(
                    b"tmp == 1\0" as *const u8 as *const libc::c_char,
                    b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                    466 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                        b"int run(Env *, void (*)(Env *))\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        tmp = dup(null);
        if tmp == 2 as libc::c_int {
        } else {
            __assert_fail(
                b"tmp == 2\0" as *const u8 as *const libc::c_char,
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                471 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    b"int run(Env *, void (*)(Env *))\0",
                ))
                .as_ptr(),
            );
        }
        'c_7844: {
            if tmp == 2 as libc::c_int {
            } else {
                __assert_fail(
                    b"tmp == 2\0" as *const u8 as *const libc::c_char,
                    b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                    471 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                        b"int run(Env *, void (*)(Env *))\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        if (*env_0).timeout > 0 as libc::c_int {
            alarm((*env_0).timeout as libc::c_uint);
        }
        process.expect("non-null function pointer")(env_0);
        close(null);
        close(2 as libc::c_int);
        tmp = dup(saved2);
        if tmp == 2 as libc::c_int {
        } else {
            __assert_fail(
                b"tmp == 2\0" as *const u8 as *const libc::c_char,
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                480 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    b"int run(Env *, void (*)(Env *))\0",
                ))
                .as_ptr(),
            );
        }
        'c_7769: {
            if tmp == 2 as libc::c_int {
            } else {
                __assert_fail(
                    b"tmp == 2\0" as *const u8 as *const libc::c_char,
                    b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                    480 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                        b"int run(Env *, void (*)(Env *))\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        close(1 as libc::c_int);
        tmp = dup(saved1);
        if tmp == 1 as libc::c_int {
        } else {
            __assert_fail(
                b"tmp == 1\0" as *const u8 as *const libc::c_char,
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                486 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    b"int run(Env *, void (*)(Env *))\0",
                ))
                .as_ptr(),
            );
        }
        'c_7719: {
            if tmp == 1 as libc::c_int {
            } else {
                __assert_fail(
                    b"tmp == 1\0" as *const u8 as *const libc::c_char,
                    b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                    486 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                        b"int run(Env *, void (*)(Env *))\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        exit(0 as libc::c_int);
    }
    if status & 0x7f as libc::c_int == 0 as libc::c_int {
        res = (status & 0xff00 as libc::c_int) >> 8 as libc::c_int;
        if res == 42 as libc::c_int {
            res = 0 as libc::c_int;
            (*env_0).violations += 1;
            (*env_0).violations;
        } else if (*env_0).print != 0 {
            printf(b"exit %d \0" as *const u8 as *const libc::c_char, res);
        }
    } else if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar as libc::c_int
        >> 1 as libc::c_int
        > 0 as libc::c_int
    {
        if (*env_0).print != 0 {
            printf(b"signal\0" as *const u8 as *const libc::c_char);
        }
        res = 1 as libc::c_int;
    } else {
        if (*env_0).print != 0 {
            printf(b"unknown\0" as *const u8 as *const libc::c_char);
        }
        res = 1 as libc::c_int;
    }
    return res;
}
unsafe extern "C" fn printrace(mut env_0: *mut Env) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut data: Data = Data {
        lgl: 0 as *mut LGL,
        trace: 0 as *mut FILE,
        available: 0 as *mut libc::c_int,
        navailable: 0,
        frozen: 0 as *mut libc::c_int,
        nfrozen: 0,
        m: 0,
        n: 0,
        c: 0,
        print: 0,
        noptsfuzz: 0,
    };
    let mut e: *mut Event = 0 as *mut Event;
    let mut i: libc::c_int = 0;
    memset(
        &mut data as *mut Data as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Data>() as libc::c_ulong,
    );
    data.print = 0 as libc::c_int;
    if !((*env_0).events).is_null() {
    } else {
        __assert_fail(
            b"env->events\0" as *const u8 as *const libc::c_char,
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            510 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"void printrace(Env *)\0"))
                .as_ptr(),
        );
    }
    'c_8228: {
        if !((*env_0).events).is_null() {
        } else {
            __assert_fail(
                b"env->events\0" as *const u8 as *const libc::c_char,
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                510 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                    b"void printrace(Env *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if !((*env_0).prefix).is_null() {
    } else {
        __assert_fail(
            b"env->prefix\0" as *const u8 as *const libc::c_char,
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            511 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"void printrace(Env *)\0"))
                .as_ptr(),
        );
    }
    'c_8192: {
        if !((*env_0).prefix).is_null() {
        } else {
            __assert_fail(
                b"env->prefix\0" as *const u8 as *const libc::c_char,
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                511 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                    b"void printrace(Env *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    name = malloc((strlen((*env_0).prefix)).wrapping_add(80 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    sprintf(
        name,
        b"%s-%u.trace\0" as *const u8 as *const libc::c_char,
        (*env_0).prefix,
        (*env_0).seed,
    );
    data.trace = fopen(name, b"w\0" as *const u8 as *const libc::c_char);
    data.noptsfuzz = (*env_0).noptsfuzz;
    if !(data.trace).is_null() {
    } else {
        __assert_fail(
            b"data.trace\0" as *const u8 as *const libc::c_char,
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            516 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"void printrace(Env *)\0"))
                .as_ptr(),
        );
    }
    'c_8104: {
        if !(data.trace).is_null() {
        } else {
            __assert_fail(
                b"data.trace\0" as *const u8 as *const libc::c_char,
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                516 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                    b"void printrace(Env *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int;
    while i < (*env_0).nevents {
        e = ((*env_0).events).offset(i as isize);
        if !((*e).remove != 0) {
            ((*e).state).expect("non-null function pointer")(&mut data, (*e).rand);
        }
        i += 1;
        i;
    }
    fclose(data.trace);
    free(name as *mut libc::c_void);
}
unsafe extern "C" fn prwc(mut env_0: *mut Env, mut prefix: *const libc::c_char) {
    let mut name: *mut libc::c_char =
        malloc((strlen(prefix)).wrapping_add(80 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
    let mut ch: libc::c_int = 0;
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut file: *mut FILE = 0 as *mut FILE;
    sprintf(
        name,
        b"%s-%u.trace\0" as *const u8 as *const libc::c_char,
        prefix,
        (*env_0).seed,
    );
    file = fopen(name, b"r\0" as *const u8 as *const libc::c_char);
    if !file.is_null() {
    } else {
        __assert_fail(
            b"file\0" as *const u8 as *const libc::c_char,
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            532 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                b"void prwc(Env *, const char *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_8363: {
        if !file.is_null() {
        } else {
            __assert_fail(
                b"file\0" as *const u8 as *const libc::c_char,
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                532 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                    b"void prwc(Env *, const char *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    loop {
        ch = getc(file);
        if !(ch != -(1 as libc::c_int)) {
            break;
        }
        if ch == '\n' as i32 {
            res += 1;
            res;
        }
    }
    fclose(file);
    if (*env_0).quiet == 0 {
        printf(b" %s %d\0" as *const u8 as *const libc::c_char, name, res);
        fflush(stdout);
    }
    free(name as *mut libc::c_void);
}
unsafe extern "C" fn dd(
    mut env_0: *mut Env,
    mut filename: *const libc::c_char,
    mut golden: libc::c_int,
    mut opt: libc::c_int,
) {
    let mut rand: libc::c_uint = 0;
    let mut state: State = None;
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    file = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if !file.is_null() {
    } else {
        __assert_fail(
            b"file\0" as *const u8 as *const libc::c_char,
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            550 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"void dd(Env *, const char *, int, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_8806: {
        if !file.is_null() {
        } else {
            __assert_fail(
                b"file\0" as *const u8 as *const libc::c_char,
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                550 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"void dd(Env *, const char *, int, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    (*env_0).nevents = 0 as libc::c_int;
    while fscanf(
        file,
        b"%p %x\n\0" as *const u8 as *const libc::c_char,
        &mut state as *mut State,
        &mut rand as *mut libc::c_uint,
    ) == 2 as libc::c_int
    {
        (*env_0).nevents += 1 as libc::c_int;
    }
    fclose(file);
    (*env_0).events = calloc(
        (*env_0).nevents as libc::c_ulong,
        ::core::mem::size_of::<Event>() as libc::c_ulong,
    ) as *mut Event;
    file = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if !file.is_null() {
    } else {
        __assert_fail(
            b"file\0" as *const u8 as *const libc::c_char,
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            557 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"void dd(Env *, const char *, int, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_8721: {
        if !file.is_null() {
        } else {
            __assert_fail(
                b"file\0" as *const u8 as *const libc::c_char,
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                557 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"void dd(Env *, const char *, int, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int;
    while fscanf(
        file,
        b"%p %x\n\0" as *const u8 as *const libc::c_char,
        &mut state as *mut State,
        &mut rand as *mut libc::c_uint,
    ) == 2 as libc::c_int
    {
        if i < (*env_0).nevents {
        } else {
            __assert_fail(
                b"i < env->nevents\0" as *const u8 as *const libc::c_char,
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                560 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"void dd(Env *, const char *, int, int)\0",
                ))
                .as_ptr(),
            );
        }
        'c_8661: {
            if i < (*env_0).nevents {
            } else {
                __assert_fail(
                    b"i < env->nevents\0" as *const u8 as *const libc::c_char,
                    b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                    560 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                        b"void dd(Env *, const char *, int, int)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        let ref mut fresh3 = (*((*env_0).events).offset(i as isize)).state;
        *fresh3 = state;
        (*((*env_0).events).offset(i as isize)).rand = rand;
        (*((*env_0).events).offset(i as isize)).remove = 0 as libc::c_int;
        i += 1;
        i;
    }
    fclose(file);
    if i == (*env_0).nevents {
    } else {
        __assert_fail(
            b"i == env->nevents\0" as *const u8 as *const libc::c_char,
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            567 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"void dd(Env *, const char *, int, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_8568: {
        if i == (*env_0).nevents {
        } else {
            __assert_fail(
                b"i == env->nevents\0" as *const u8 as *const libc::c_char,
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                567 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"void dd(Env *, const char *, int, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    (*env_0).prefix = b"bug\0" as *const u8 as *const libc::c_char;
    run(
        env_0,
        Some(printrace as unsafe extern "C" fn(*mut Env) -> ()),
    );
    prwc(env_0, b"bug\0" as *const u8 as *const libc::c_char);
    cmd = malloc(100 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    if (*env_0).nodd != 0 {
        sprintf(
            cmd,
            b"cp bug-%u.trace red-%u.trace\0" as *const u8 as *const libc::c_char,
            (*env_0).seed,
            (*env_0).seed,
        );
    } else {
        sprintf(
            cmd,
            b"./lglddtrace %s bug-%u.trace red-%u.trace\0" as *const u8 as *const libc::c_char,
            if opt != 0 {
                b"-O\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            (*env_0).seed,
            (*env_0).seed,
        );
    }
    let mut tmp: libc::c_int = system(cmd);
    prwc(env_0, b"red\0" as *const u8 as *const libc::c_char);
    free(cmd as *mut libc::c_void);
    free((*env_0).events as *mut libc::c_void);
}
unsafe extern "C" fn hashmac() -> libc::c_uint {
    let mut file: *mut FILE = fopen(
        b"/sys/class/net/eth0/address\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    let mut mac: [libc::c_uint; 6] = [0; 6];
    let mut res: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if file.is_null() {
        return 0 as libc::c_int as libc::c_uint;
    }
    if fscanf(
        file,
        b"%02x:%02x:%02x:%02x:%02x:%02x\0" as *const u8 as *const libc::c_char,
        mac.as_mut_ptr().offset(0 as libc::c_int as isize),
        mac.as_mut_ptr().offset(1 as libc::c_int as isize),
        mac.as_mut_ptr().offset(2 as libc::c_int as isize),
        mac.as_mut_ptr().offset(3 as libc::c_int as isize),
        mac.as_mut_ptr().offset(4 as libc::c_int as isize),
        mac.as_mut_ptr().offset(5 as libc::c_int as isize),
    ) == 6 as libc::c_int
    {
        res = mac[5 as libc::c_int as usize];
        res ^= mac[4 as libc::c_int as usize] << 4 as libc::c_int;
        res ^= mac[3 as libc::c_int as usize] << 8 as libc::c_int;
        res ^= mac[2 as libc::c_int as usize] << 16 as libc::c_int;
        res ^= mac[1 as libc::c_int as usize] << 20 as libc::c_int;
        res ^= mac[0 as libc::c_int as usize] << 24 as libc::c_int;
    }
    fclose(file);
    return res;
}
static mut usage: *const libc::c_char = b"usage: lglmbt [ <option> ... ] [ <seed> ]\n\nwhere <option> is one of the following:\n\n  -k | --keep-lines\n  -q | --quiet\n  -f | --first-bug-only\n  -n | --no-delta-debugging\n  -a | --always-fork\n  -O | --optimize-by-delta-debugging-options\n\n  -m <maxruns>\n\0"
    as *const u8 as *const libc::c_char;
static mut env: Env = Env {
    file: 0 as *const FILE as *mut FILE,
    prefix: 0 as *const libc::c_char,
    round: 0,
    bugs: 0,
    violations: 0,
    print: 0,
    quiet: 0,
    first: 0,
    nodd: 0,
    noptsfuzz: 0,
    alwaysfork: 0,
    terminal: 0,
    forked: 0,
    nevents: 0,
    timeout: 0,
    seed: 0,
    events: 0 as *const Event as *mut Event,
    rng: RNG { z: 0, w: 0 },
};
static mut start: libc::c_double = 0.;
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
    return currentime() - start;
}
unsafe extern "C" fn average(mut a: libc::c_double, mut b: libc::c_double) -> libc::c_double {
    return if b != 0. {
        a / b
    } else {
        0 as libc::c_int as libc::c_double
    };
}
unsafe extern "C" fn stats() {
    let mut t: libc::c_double = getime();
    let mut valid: libc::c_int = env.round - env.violations;
    printf(
        b"[lglmbt] finished after %.2f seconds\n\0" as *const u8 as *const libc::c_char,
        t,
    );
    printf(
        b"[lglmbt] %d rounds = %.0f rounds per second\n\0" as *const u8 as *const libc::c_char,
        env.round,
        average(env.round as libc::c_double, t),
    );
    printf(
        b"[lglmbt] %d violations = %.0f rounds per second\n\0" as *const u8 as *const libc::c_char,
        env.violations,
        average(env.violations as libc::c_double, t),
    );
    printf(
        b"[lglmbt] %d valid runs = %.0f rounds per second\n\0" as *const u8 as *const libc::c_char,
        valid,
        average(valid as libc::c_double, t),
    );
    printf(
        b"[lglmbt] %d bugs = %.0f bugs per second\n\0" as *const u8 as *const libc::c_char,
        env.bugs,
        average(env.bugs as libc::c_double, t),
    );
}
unsafe extern "C" fn sighandler(mut sig: libc::c_int) {
    fflush(stdout);
    fflush(stderr);
    printf(
        b"*** lglmbt: caught signal %d in round %d\n\0" as *const u8 as *const libc::c_char,
        sig,
        env.round,
    );
    fflush(stdout);
    stats();
    exit(1 as libc::c_int);
}
unsafe extern "C" fn setsighandlers() {
    signal(
        2 as libc::c_int,
        Some(sighandler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    signal(
        11 as libc::c_int,
        Some(sighandler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    signal(
        6 as libc::c_int,
        Some(sighandler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    signal(
        15 as libc::c_int,
        Some(sighandler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut prev: libc::c_int = 0;
    let mut opt: libc::c_int = 0;
    let mut mac: libc::c_int = 0;
    let mut pid: libc::c_int = 0;
    start = currentime();
    memset(
        &mut env as *mut Env as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Env>() as libc::c_ulong,
    );
    max = 2147483647 as libc::c_int;
    prev = 1 as libc::c_int;
    memset(
        &mut env as *mut Env as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Env>() as libc::c_ulong,
    );
    env.seed = -(1 as libc::c_int) as libc::c_uint;
    env.terminal = isatty(1 as libc::c_int);
    env.timeout = 0 as libc::c_int;
    opt = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < argc {
        if strcmp(
            *argv.offset(i as isize),
            b"-h\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            printf(b"%s\0" as *const u8 as *const libc::c_char, usage);
            exit(0 as libc::c_int);
        } else if strcmp(
            *argv.offset(i as isize),
            b"-k\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--keep-lines\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            env.terminal = 0 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-q\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--quiet\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            env.quiet = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-f\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--first-bug-only\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            env.first = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-n\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--no-delta-debugging\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            env.nodd = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-d\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--do-not-fuzz-options\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            env.noptsfuzz = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-a\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--always-fork\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            env.alwaysfork = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-O\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--optimize-by-delta-debugging-options\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            opt = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-t\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            i += 1;
            if i == argc {
                die(b"argument to '-t' missing (try '-h')\0" as *const u8 as *const libc::c_char);
            }
            if isnumstr(*argv.offset(i as isize)) == 0 {
                die(
                    b"argument '%s' to '-t' not a number (try '-h')\0" as *const u8
                        as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
            env.timeout = atoi(*argv.offset(i as isize));
        } else if strcmp(
            *argv.offset(i as isize),
            b"-m\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            i += 1;
            if i == argc {
                die(b"argument to '-m' missing (try '-h')\0" as *const u8 as *const libc::c_char);
            }
            if isnumstr(*argv.offset(i as isize)) == 0 {
                die(
                    b"argument '%s' to '-m' not a number (try '-h')\0" as *const u8
                        as *const libc::c_char,
                    *argv.offset(i as isize),
                );
            }
            max = atoi(*argv.offset(i as isize));
        } else if isnumstr(*argv.offset(i as isize)) == 0 {
            die(
                b"invalid command line option '%s' (try '-h')\0" as *const u8
                    as *const libc::c_char,
                *argv.offset(i as isize),
            );
        } else {
            env.seed = atoi(*argv.offset(i as isize)) as libc::c_uint;
        }
        i += 1;
        i;
    }
    env.print = (env.quiet == 0) as libc::c_int;
    if env.seed != -(1 as libc::c_int) as libc::c_uint && env.alwaysfork == 0 {
        rantrav(&mut env);
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    } else {
        mac = hashmac() as libc::c_int;
        pid = getpid();
        setsighandlers();
        env.round = 0 as libc::c_int;
        while env.round < max {
            if prev & 1 as libc::c_int == 0 {
                prev += 1;
                prev;
            }
            env.seed = mac as libc::c_uint;
            env.seed = (env.seed).wrapping_mul(123301093 as libc::c_int as libc::c_uint);
            env.seed = (env.seed as clock_t + times(0 as *mut tms)) as libc::c_uint;
            env.seed = (env.seed).wrapping_mul(223531513 as libc::c_int as libc::c_uint);
            env.seed = (env.seed).wrapping_add(pid as libc::c_uint);
            env.seed = (env.seed).wrapping_mul(31752023 as libc::c_int as libc::c_uint);
            env.seed = (env.seed).wrapping_add(prev as libc::c_uint);
            env.seed = (env.seed).wrapping_mul(43376579 as libc::c_int as libc::c_uint);
            env.seed = env.seed >> 1 as libc::c_int;
            prev = env.seed as libc::c_int;
            if env.quiet == 0 {
                if env.terminal != 0 {
                    erase();
                }
                printf(
                    b"%d %d \0" as *const u8 as *const libc::c_char,
                    env.round,
                    env.seed,
                );
                fflush(stdout);
            }
            res = run(
                &mut env,
                Some(rantrav as unsafe extern "C" fn(*mut Env) -> ()),
            );
            if res > 0 as libc::c_int {
                let mut name: [libc::c_char; 100] = [0; 100];
                env.bugs += 1;
                env.bugs;
                sprintf(
                    name.as_mut_ptr(),
                    b"/tmp/lglmbt-tmp-%d.trace\0" as *const u8 as *const libc::c_char,
                    getpid(),
                );
                env.file = fopen(
                    name.as_mut_ptr(),
                    b"w\0" as *const u8 as *const libc::c_char,
                );
                env.print = 0 as libc::c_int;
                tmp = run(
                    &mut env,
                    Some(rantrav as unsafe extern "C" fn(*mut Env) -> ()),
                );
                if tmp == res {
                } else {
                    __assert_fail(
                        b"tmp == res\0" as *const u8 as *const libc::c_char,
                        b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                        751 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                            b"int main(int, char **)\0",
                        ))
                        .as_ptr(),
                    );
                }
                'c_9379: {
                    if tmp == res {
                    } else {
                        __assert_fail(
                            b"tmp == res\0" as *const u8 as *const libc::c_char,
                            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                            751 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                                b"int main(int, char **)\0",
                            ))
                            .as_ptr(),
                        );
                    }
                };
                fclose(env.file);
                env.file = 0 as *mut FILE;
                dd(&mut env, name.as_mut_ptr(), res, opt);
                unlink(name.as_mut_ptr());
                env.print = (env.quiet == 0) as libc::c_int;
            }
            if env.quiet == 0 {
                if res != 0 || env.terminal == 0 {
                    printf(b"\n\0" as *const u8 as *const libc::c_char);
                }
                fflush(stdout);
            }
            if res != 0 && env.first != 0 {
                break;
            }
            env.round += 1;
            env.round;
        }
    }
    if env.quiet == 0 {
        if env.terminal != 0 {
            erase();
        }
        printf(
            b"forked %d\n\0" as *const u8 as *const libc::c_char,
            env.forked,
        );
    }
    stats();
    return 0 as libc::c_int;
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
