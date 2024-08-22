#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern crate libgeling;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type LGL;
    fn lglinit() -> *mut LGL;
    fn lglrelease(_: *mut LGL);
    fn lglusage(_: *mut LGL);
    fn lglversion() -> *const libc::c_char;
    fn lglseterm(
        _: *mut LGL,
        term: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
        _: *mut libc::c_void,
    );
    fn lglsec(_: *mut LGL) -> libc::c_double;
    fn lglstats(_: *mut LGL);
    fn lglflushtimers(lgl: *mut LGL);
    fn lglderef(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn lglbnr(name: *const libc::c_char, prefix: *const libc::c_char, file: *mut FILE);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn lglparsefile(
        _: *mut LGL,
        _: *mut FILE,
        force_0: libc::c_int,
        lineno_ptr: *mut libc::c_int,
        max_var_ptr: *mut libc::c_int,
    ) -> *const libc::c_char;
    fn lglparsepath(
        _: *mut LGL,
        path: *const libc::c_char,
        force_0: libc::c_int,
        lineno_ptr: *mut libc::c_int,
        max_var_ptr: *mut libc::c_int,
    ) -> *const libc::c_char;
    fn lglsimp(_: *mut LGL, iterations: libc::c_int) -> libc::c_int;
    fn lglopts(_: *mut LGL, prefix: *const libc::c_char, _: libc::c_int);
    fn lglrgopts(_: *mut LGL);
    fn lglpcs(_: *mut LGL, mixed: libc::c_int);
    fn lglsizes(_: *mut LGL);
    fn lglsetopt(_: *mut LGL, _: *const libc::c_char, _: libc::c_int);
    fn lglreadopts(_: *mut LGL, _: *mut FILE) -> libc::c_int;
    fn lglgetopt(_: *mut LGL, _: *const libc::c_char) -> libc::c_int;
    fn lglhasopt(_: *mut LGL, _: *const libc::c_char) -> libc::c_int;
    fn lglctrav(
        _: *mut LGL,
        state: *mut libc::c_void,
        trav: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
    );
    fn lglprint(_: *mut LGL, _: *mut FILE);
    fn lglassume(_: *mut LGL, lit: libc::c_int);
    fn lglsat(_: *mut LGL) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    // 100: _IO_read_ptr: typeof(_IO_read_ptr) = *mut {g125} i8
    // 100: _IO_read_ptr:   g125 = UNIQUE | NON_NULL, FIXED
    pub _IO_read_end: *mut libc::c_char,
    // 101: _IO_read_end: typeof(_IO_read_end) = *mut {g126} i8
    // 101: _IO_read_end:   g126 = UNIQUE | NON_NULL, FIXED
    pub _IO_read_base: *mut libc::c_char,
    // 102: _IO_read_base: typeof(_IO_read_base) = *mut {g127} i8
    // 102: _IO_read_base:   g127 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_base: *mut libc::c_char,
    // 103: _IO_write_base: typeof(_IO_write_base) = *mut {g128} i8
    // 103: _IO_write_base:   g128 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_ptr: *mut libc::c_char,
    // 104: _IO_write_ptr: typeof(_IO_write_ptr) = *mut {g129} i8
    // 104: _IO_write_ptr:   g129 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_end: *mut libc::c_char,
    // 105: _IO_write_end: typeof(_IO_write_end) = *mut {g130} i8
    // 105: _IO_write_end:   g130 = UNIQUE | NON_NULL, FIXED
    pub _IO_buf_base: *mut libc::c_char,
    // 106: _IO_buf_base: typeof(_IO_buf_base) = *mut {g131} i8
    // 106: _IO_buf_base:   g131 = UNIQUE | NON_NULL, FIXED
    pub _IO_buf_end: *mut libc::c_char,
    // 107: _IO_buf_end: typeof(_IO_buf_end) = *mut {g132} i8
    // 107: _IO_buf_end:   g132 = UNIQUE | NON_NULL, FIXED
    pub _IO_save_base: *mut libc::c_char,
    // 108: _IO_save_base: typeof(_IO_save_base) = *mut {g133} i8
    // 108: _IO_save_base:   g133 = UNIQUE | NON_NULL, FIXED
    pub _IO_backup_base: *mut libc::c_char,
    // 109: _IO_backup_base: typeof(_IO_backup_base) = *mut {g134} i8
    // 109: _IO_backup_base:   g134 = UNIQUE | NON_NULL, FIXED
    pub _IO_save_end: *mut libc::c_char,
    // 110: _IO_save_end: typeof(_IO_save_end) = *mut {g135} i8
    // 110: _IO_save_end:   g135 = UNIQUE | NON_NULL, FIXED
    pub _markers: *mut _IO_marker,
    // 111: _markers: typeof(_markers) = *mut {g136} _IO_marker
    // 111: _markers:   g136 = UNIQUE | NON_NULL, FIXED
    pub _chain: *mut _IO_FILE,
    // 112: _chain: typeof(_chain) = *mut {g137} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
    // 112: _chain:   g137 = UNIQUE | NON_NULL, FIXED
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    // 119: _lock: typeof(_lock) = *mut {g138} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 119: _lock:   g138 = UNIQUE | NON_NULL, FIXED
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    // 121: _codecvt: typeof(_codecvt) = *mut {g139} _IO_codecvt
    // 121: _codecvt:   g139 = UNIQUE | NON_NULL, FIXED
    pub _wide_data: *mut _IO_wide_data,
    // 122: _wide_data: typeof(_wide_data) = *mut {g140} _IO_wide_data
    // 122: _wide_data:   g140 = UNIQUE | NON_NULL, FIXED
    pub _freeres_list: *mut _IO_FILE,
    // 123: _freeres_list: typeof(_freeres_list) = *mut {g141} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
    // 123: _freeres_list:   g141 = UNIQUE | NON_NULL, FIXED
    pub _freeres_buf: *mut libc::c_void,
    // 124: _freeres_buf: typeof(_freeres_buf) = *mut {g142} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 124: _freeres_buf:   g142 = UNIQUE | NON_NULL, FIXED
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub type __sighandler_t = Option<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OBuf {
    pub line: [libc::c_char; 81],
    pub pos: libc::c_int,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
// 152: mut __nptr: typeof(_1) = *const {g0} i8
// 152: mut __nptr:   g0 = READ | UNIQUE | OFFSET_ADD | NON_NULL, FIXED
    return strtol(
        __nptr,
        // 154: __nptr: typeof(_4) = *const {l4} i8
        // 154: __nptr:   l4 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        // 155: 0 as *mut libc: ... _char: typeof(_5) = *mut {l6} *mut {g43} i8
        // 155: 0 as *mut libc: ... _char:   l6 = WRITE | UNIQUE, (empty)
        // 155: 0 as *mut libc: ... _char:   g43 = WRITE | OFFSET_ADD, CELL | FIXED
        // 155: 0 as *mut libc: ... _void: typeof(_6) = *mut {l8} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 155: 0 as *mut libc: ... _void:   l8 = WRITE | UNIQUE, (empty)
        // 155: 0 as *mut libc: ... _void: typeof(_6 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l11} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 155: 0 as *mut libc: ... _void:   l11 = WRITE | UNIQUE, (empty)
        // 155: 0 as *mut libc: ... _char: typeof(_5 = move _6 as *mut *mut i8 (Misc)) = *mut {l12} *mut {g43} i8
        // 155: 0 as *mut libc: ... _char:   l12 = WRITE | UNIQUE, (empty)
        // 155: 0 as *mut libc: ... _char:   g43 = WRITE | OFFSET_ADD, CELL | FIXED
        10 as libc::c_int,
    ) as libc::c_int;
}
static lgl4sigh: *mut LGL = 0 as *const LGL as *mut LGL;
static catchedsig: libc::c_int = 0;
static verbose: libc::c_int = 0;
static force: libc::c_int = 0;
static mut targets: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut sztargets: libc::c_int = 0;
static mut ntargets: libc::c_int = 0;
static sig_int_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static sig_segv_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static sig_abrt_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static sig_term_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static sig_bus_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static sig_alrm_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
unsafe extern "C" fn resetsighandlers() {
    signal(2 as libc::c_int, sig_int_handler);
    // 173: sig_int_handler: typeof(_4) = *mut {l4} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 173: sig_int_handler:   l4 = UNIQUE | NON_NULL, (empty)
    signal(11 as libc::c_int, sig_segv_handler);
    // 174: sig_segv_handler: typeof(_8) = *mut {l9} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 174: sig_segv_handler:   l9 = UNIQUE | NON_NULL, (empty)
    signal(6 as libc::c_int, sig_abrt_handler);
    // 175: sig_abrt_handler: typeof(_12) = *mut {l14} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 175: sig_abrt_handler:   l14 = UNIQUE | NON_NULL, (empty)
    signal(15 as libc::c_int, sig_term_handler);
    // 176: sig_term_handler: typeof(_16) = *mut {l19} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 176: sig_term_handler:   l19 = UNIQUE | NON_NULL, (empty)
    signal(7 as libc::c_int, sig_bus_handler);
    // 177: sig_bus_handler: typeof(_20) = *mut {l24} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 177: sig_bus_handler:   l24 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn caughtsigmsg(mut sig: libc::c_int) {
    if verbose < 0 as libc::c_int {
    // 180: verbose: typeof(_5) = *mut {l5} i32
    // 180: verbose:   l5 = UNIQUE | NON_NULL, (empty)
        return;
    }
    printf(
        b"c\nc CAUGHT SIGNAL %d\0" as *const u8 as *const libc::c_char,
        // 184: b"c\nc CAUGHT S ... _char: typeof(_9) = *const {l10} i8
        // 184: b"c\nc CAUGHT S ... _char:   l10 = UNIQUE | NON_NULL, (empty)
        // 184: b"c\nc CAUGHT S ... st u8: typeof(_10) = *const {l12} u8
        // 184: b"c\nc CAUGHT S ... st u8:   l12 = UNIQUE | NON_NULL, (empty)
        // 184: b"c\nc CAUGHT S ... %d\0": typeof(_11) = *const {l14} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
        // 184: b"c\nc CAUGHT S ... %d\0":   l14 = UNIQUE | NON_NULL, (empty)
        // 184: b"c\nc CAUGHT S ... %d\0": typeof(_12) = & {l16} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
        // 184: b"c\nc CAUGHT S ... %d\0":   l16 = UNIQUE | NON_NULL, FIXED
        // 184: b"c\nc CAUGHT S ... st u8: typeof(_10 = move _11 as *const u8 (Pointer(ArrayToPointer))) = *const {l91} u8
        // 184: b"c\nc CAUGHT S ... st u8:   l91 = UNIQUE | NON_NULL, (empty)
        // 184: b"c\nc CAUGHT S ... %d\0": typeof(_12 = const b"c\nc CAUGHT SIGNAL %d\x00") = & {l89} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
        // 184: b"c\nc CAUGHT S ... %d\0":   l89 = UNIQUE | NON_NULL, (empty)
        // 184: b"c\nc CAUGHT S ... _char: typeof(_9 = move _10 as *const i8 (Misc)) = *const {l92} i8
        // 184: b"c\nc CAUGHT S ... _char:   l92 = UNIQUE | NON_NULL, (empty)
        // 184: b"c\nc CAUGHT S ... %d\0": typeof(_11 = &raw const (*_12)) = *const {l90} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
        // 184: b"c\nc CAUGHT S ... %d\0":   l90 = UNIQUE | NON_NULL, (empty)
        sig,
    );
    match sig {
        2 => {
            printf(b" SIGINT\0" as *const u8 as *const libc::c_char);
            // 189: b" SIGINT\0" as ... _char: typeof(_16) = *const {l21} i8
            // 189: b" SIGINT\0" as ... _char:   l21 = UNIQUE | NON_NULL, (empty)
            // 189: b" SIGINT\0" as ... st u8: typeof(_17) = *const {l23} u8
            // 189: b" SIGINT\0" as ... st u8:   l23 = UNIQUE | NON_NULL, (empty)
            // 189: b" SIGINT\0": typeof(_18) = *const {l25} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 189: b" SIGINT\0":   l25 = UNIQUE | NON_NULL, (empty)
            // 189: b" SIGINT\0": typeof(_19) = & {l27} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 189: b" SIGINT\0":   l27 = UNIQUE | NON_NULL, FIXED
            // 189: b" SIGINT\0": typeof(_18 = &raw const (*_19)) = *const {l94} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 189: b" SIGINT\0":   l94 = UNIQUE | NON_NULL, (empty)
            // 189: b" SIGINT\0" as ... st u8: typeof(_17 = move _18 as *const u8 (Pointer(ArrayToPointer))) = *const {l95} u8
            // 189: b" SIGINT\0" as ... st u8:   l95 = UNIQUE | NON_NULL, (empty)
            // 189: b" SIGINT\0" as ... _char: typeof(_16 = move _17 as *const i8 (Misc)) = *const {l96} i8
            // 189: b" SIGINT\0" as ... _char:   l96 = UNIQUE | NON_NULL, (empty)
            // 189: b" SIGINT\0": typeof(_19 = const b" SIGINT\x00") = & {l93} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 189: b" SIGINT\0":   l93 = UNIQUE | NON_NULL, (empty)
        }
        11 => {
            printf(b" SIGSEGV\0" as *const u8 as *const libc::c_char);
            // 192: b" SIGSEGV\0" a ... _char: typeof(_21) = *const {l30} i8
            // 192: b" SIGSEGV\0" a ... _char:   l30 = UNIQUE | NON_NULL, (empty)
            // 192: b" SIGSEGV\0" a ... st u8: typeof(_22) = *const {l32} u8
            // 192: b" SIGSEGV\0" a ... st u8:   l32 = UNIQUE | NON_NULL, (empty)
            // 192: b" SIGSEGV\0": typeof(_23) = *const {l34} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 192: b" SIGSEGV\0":   l34 = UNIQUE | NON_NULL, (empty)
            // 192: b" SIGSEGV\0": typeof(_24) = & {l36} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 192: b" SIGSEGV\0":   l36 = UNIQUE | NON_NULL, FIXED
            // 192: b" SIGSEGV\0" a ... st u8: typeof(_22 = move _23 as *const u8 (Pointer(ArrayToPointer))) = *const {l99} u8
            // 192: b" SIGSEGV\0" a ... st u8:   l99 = UNIQUE | NON_NULL, (empty)
            // 192: b" SIGSEGV\0" a ... _char: typeof(_21 = move _22 as *const i8 (Misc)) = *const {l100} i8
            // 192: b" SIGSEGV\0" a ... _char:   l100 = UNIQUE | NON_NULL, (empty)
            // 192: b" SIGSEGV\0": typeof(_24 = const b" SIGSEGV\x00") = & {l97} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 192: b" SIGSEGV\0":   l97 = UNIQUE | NON_NULL, (empty)
            // 192: b" SIGSEGV\0": typeof(_23 = &raw const (*_24)) = *const {l98} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 192: b" SIGSEGV\0":   l98 = UNIQUE | NON_NULL, (empty)
        }
        6 => {
            printf(b" SIGABRT\0" as *const u8 as *const libc::c_char);
            // 195: b" SIGABRT\0" a ... _char: typeof(_26) = *const {l39} i8
            // 195: b" SIGABRT\0" a ... _char:   l39 = UNIQUE | NON_NULL, (empty)
            // 195: b" SIGABRT\0" a ... st u8: typeof(_27) = *const {l41} u8
            // 195: b" SIGABRT\0" a ... st u8:   l41 = UNIQUE | NON_NULL, (empty)
            // 195: b" SIGABRT\0": typeof(_28) = *const {l43} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 195: b" SIGABRT\0":   l43 = UNIQUE | NON_NULL, (empty)
            // 195: b" SIGABRT\0": typeof(_29) = & {l45} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 195: b" SIGABRT\0":   l45 = UNIQUE | NON_NULL, FIXED
            // 195: b" SIGABRT\0" a ... _char: typeof(_26 = move _27 as *const i8 (Misc)) = *const {l104} i8
            // 195: b" SIGABRT\0" a ... _char:   l104 = UNIQUE | NON_NULL, (empty)
            // 195: b" SIGABRT\0": typeof(_29 = const b" SIGABRT\x00") = & {l101} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 195: b" SIGABRT\0":   l101 = UNIQUE | NON_NULL, (empty)
            // 195: b" SIGABRT\0": typeof(_28 = &raw const (*_29)) = *const {l102} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 195: b" SIGABRT\0":   l102 = UNIQUE | NON_NULL, (empty)
            // 195: b" SIGABRT\0" a ... st u8: typeof(_27 = move _28 as *const u8 (Pointer(ArrayToPointer))) = *const {l103} u8
            // 195: b" SIGABRT\0" a ... st u8:   l103 = UNIQUE | NON_NULL, (empty)
        }
        15 => {
            printf(b" SIGTERM\0" as *const u8 as *const libc::c_char);
            // 198: b" SIGTERM\0" a ... _char: typeof(_31) = *const {l48} i8
            // 198: b" SIGTERM\0" a ... _char:   l48 = UNIQUE | NON_NULL, (empty)
            // 198: b" SIGTERM\0" a ... st u8: typeof(_32) = *const {l50} u8
            // 198: b" SIGTERM\0" a ... st u8:   l50 = UNIQUE | NON_NULL, (empty)
            // 198: b" SIGTERM\0": typeof(_33) = *const {l52} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 198: b" SIGTERM\0":   l52 = UNIQUE | NON_NULL, (empty)
            // 198: b" SIGTERM\0": typeof(_34) = & {l54} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 198: b" SIGTERM\0":   l54 = UNIQUE | NON_NULL, FIXED
            // 198: b" SIGTERM\0": typeof(_34 = const b" SIGTERM\x00") = & {l105} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 198: b" SIGTERM\0":   l105 = UNIQUE | NON_NULL, (empty)
            // 198: b" SIGTERM\0" a ... st u8: typeof(_32 = move _33 as *const u8 (Pointer(ArrayToPointer))) = *const {l107} u8
            // 198: b" SIGTERM\0" a ... st u8:   l107 = UNIQUE | NON_NULL, (empty)
            // 198: b" SIGTERM\0" a ... _char: typeof(_31 = move _32 as *const i8 (Misc)) = *const {l108} i8
            // 198: b" SIGTERM\0" a ... _char:   l108 = UNIQUE | NON_NULL, (empty)
            // 198: b" SIGTERM\0": typeof(_33 = &raw const (*_34)) = *const {l106} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 198: b" SIGTERM\0":   l106 = UNIQUE | NON_NULL, (empty)
        }
        7 => {
            printf(b" SIGBUS\0" as *const u8 as *const libc::c_char);
            // 201: b" SIGBUS\0" as ... _char: typeof(_36) = *const {l57} i8
            // 201: b" SIGBUS\0" as ... _char:   l57 = UNIQUE | NON_NULL, (empty)
            // 201: b" SIGBUS\0" as ... st u8: typeof(_37) = *const {l59} u8
            // 201: b" SIGBUS\0" as ... st u8:   l59 = UNIQUE | NON_NULL, (empty)
            // 201: b" SIGBUS\0": typeof(_38) = *const {l61} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 201: b" SIGBUS\0":   l61 = UNIQUE | NON_NULL, (empty)
            // 201: b" SIGBUS\0": typeof(_39) = & {l63} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 201: b" SIGBUS\0":   l63 = UNIQUE | NON_NULL, FIXED
            // 201: b" SIGBUS\0": typeof(_38 = &raw const (*_39)) = *const {l110} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 201: b" SIGBUS\0":   l110 = UNIQUE | NON_NULL, (empty)
            // 201: b" SIGBUS\0" as ... st u8: typeof(_37 = move _38 as *const u8 (Pointer(ArrayToPointer))) = *const {l111} u8
            // 201: b" SIGBUS\0" as ... st u8:   l111 = UNIQUE | NON_NULL, (empty)
            // 201: b" SIGBUS\0" as ... _char: typeof(_36 = move _37 as *const i8 (Misc)) = *const {l112} i8
            // 201: b" SIGBUS\0" as ... _char:   l112 = UNIQUE | NON_NULL, (empty)
            // 201: b" SIGBUS\0": typeof(_39 = const b" SIGBUS\x00") = & {l109} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 201: b" SIGBUS\0":   l109 = UNIQUE | NON_NULL, (empty)
        }
        14 => {
            printf(b" SIGALRM\0" as *const u8 as *const libc::c_char);
            // 204: b" SIGALRM\0" a ... _char: typeof(_41) = *const {l66} i8
            // 204: b" SIGALRM\0" a ... _char:   l66 = UNIQUE | NON_NULL, (empty)
            // 204: b" SIGALRM\0" a ... st u8: typeof(_42) = *const {l68} u8
            // 204: b" SIGALRM\0" a ... st u8:   l68 = UNIQUE | NON_NULL, (empty)
            // 204: b" SIGALRM\0": typeof(_43) = *const {l70} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 204: b" SIGALRM\0":   l70 = UNIQUE | NON_NULL, (empty)
            // 204: b" SIGALRM\0": typeof(_44) = & {l72} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 204: b" SIGALRM\0":   l72 = UNIQUE | NON_NULL, FIXED
            // 204: b" SIGALRM\0": typeof(_43 = &raw const (*_44)) = *const {l114} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 204: b" SIGALRM\0":   l114 = UNIQUE | NON_NULL, (empty)
            // 204: b" SIGALRM\0": typeof(_44 = const b" SIGALRM\x00") = & {l113} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 204: b" SIGALRM\0":   l113 = UNIQUE | NON_NULL, (empty)
            // 204: b" SIGALRM\0" a ... st u8: typeof(_42 = move _43 as *const u8 (Pointer(ArrayToPointer))) = *const {l115} u8
            // 204: b" SIGALRM\0" a ... st u8:   l115 = UNIQUE | NON_NULL, (empty)
            // 204: b" SIGALRM\0" a ... _char: typeof(_41 = move _42 as *const i8 (Misc)) = *const {l116} i8
            // 204: b" SIGALRM\0" a ... _char:   l116 = UNIQUE | NON_NULL, (empty)
        }
        _ => {}
    }
    printf(b"\nc\n\0" as *const u8 as *const libc::c_char);
    // 208: b"\nc\n\0" as * ... _char: typeof(_46) = *const {l75} i8
    // 208: b"\nc\n\0" as * ... _char:   l75 = UNIQUE | NON_NULL, (empty)
    // 208: b"\nc\n\0" as * ... st u8: typeof(_47) = *const {l77} u8
    // 208: b"\nc\n\0" as * ... st u8:   l77 = UNIQUE | NON_NULL, (empty)
    // 208: b"\nc\n\0": typeof(_48) = *const {l79} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 208: b"\nc\n\0":   l79 = UNIQUE | NON_NULL, (empty)
    // 208: b"\nc\n\0": typeof(_49) = & {l81} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 208: b"\nc\n\0":   l81 = UNIQUE | NON_NULL, FIXED
    // 208: b"\nc\n\0": typeof(_48 = &raw const (*_49)) = *const {l118} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 208: b"\nc\n\0":   l118 = UNIQUE | NON_NULL, (empty)
    // 208: b"\nc\n\0" as * ... _char: typeof(_46 = move _47 as *const i8 (Misc)) = *const {l120} i8
    // 208: b"\nc\n\0" as * ... _char:   l120 = UNIQUE | NON_NULL, (empty)
    // 208: b"\nc\n\0": typeof(_49 = const b"\nc\n\x00") = & {l117} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 208: b"\nc\n\0":   l117 = UNIQUE | NON_NULL, (empty)
    // 208: b"\nc\n\0" as * ... st u8: typeof(_47 = move _48 as *const u8 (Pointer(ArrayToPointer))) = *const {l119} u8
    // 208: b"\nc\n\0" as * ... st u8:   l119 = UNIQUE | NON_NULL, (empty)
    fflush(stdout);
    // 209: stdout: typeof(_51) = *mut {l84} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
    // 209: stdout:   l84 = UNIQUE | NON_NULL, (empty)
    // 209: stdout: typeof(_52) = *mut {l86} *mut {l87} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
    // 209: stdout:   l86 = UNIQUE | NON_NULL, (empty)
    // 209: stdout:   l87 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn catchsig(mut sig: libc::c_int) {
    if catchedsig == 0 {
    // 212: catchedsig: typeof(_5) = *mut {l5} i32
    // 212: catchedsig:   l5 = UNIQUE | NON_NULL, (empty)
        catchedsig = 1 as libc::c_int;
        // 213: catchedsig: typeof(_7) = *mut {l8} i32
        // 213: catchedsig:   l8 = UNIQUE | NON_NULL, (empty)
        caughtsigmsg(sig);
        fputs(
            b"c s UNKNOWN\n\0" as *const u8 as *const libc::c_char,
            // 216: b"c s UNKNOWN\n ... _char: typeof(_11) = *const {l13} i8
            // 216: b"c s UNKNOWN\n ... _char:   l13 = UNIQUE | NON_NULL, (empty)
            // 216: b"c s UNKNOWN\n ... st u8: typeof(_12) = *const {l15} u8
            // 216: b"c s UNKNOWN\n ... st u8:   l15 = UNIQUE | NON_NULL, (empty)
            // 216: b"c s UNKNOWN\n\0": typeof(_13) = *const {l17} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 216: b"c s UNKNOWN\n\0":   l17 = UNIQUE | NON_NULL, (empty)
            // 216: b"c s UNKNOWN\n\0": typeof(_14) = & {l19} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 216: b"c s UNKNOWN\n\0":   l19 = UNIQUE | NON_NULL, FIXED
            // 216: b"c s UNKNOWN\n\0": typeof(_14 = const b"c s UNKNOWN\n\x00") = & {l69} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 216: b"c s UNKNOWN\n\0":   l69 = UNIQUE | NON_NULL, (empty)
            // 216: b"c s UNKNOWN\n ... _char: typeof(_11 = move _12 as *const i8 (Misc)) = *const {l72} i8
            // 216: b"c s UNKNOWN\n ... _char:   l72 = UNIQUE | NON_NULL, (empty)
            // 216: b"c s UNKNOWN\n ... st u8: typeof(_12 = move _13 as *const u8 (Pointer(ArrayToPointer))) = *const {l71} u8
            // 216: b"c s UNKNOWN\n ... st u8:   l71 = UNIQUE | NON_NULL, (empty)
            // 216: b"c s UNKNOWN\n\0": typeof(_13 = &raw const (*_14)) = *const {l70} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 216: b"c s UNKNOWN\n\0":   l70 = UNIQUE | NON_NULL, (empty)
            stdout,
            // 217: stdout: typeof(_15) = *mut {l21} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
            // 217: stdout:   l21 = UNIQUE | NON_NULL, (empty)
            // 217: stdout: typeof(_16) = *mut {l23} *mut {l24} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
            // 217: stdout:   l23 = UNIQUE | NON_NULL, (empty)
            // 217: stdout:   l24 = UNIQUE | NON_NULL, (empty)
        );
        fflush(stdout);
        // 219: stdout: typeof(_18) = *mut {l27} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
        // 219: stdout:   l27 = UNIQUE | NON_NULL, (empty)
        // 219: stdout: typeof(_19) = *mut {l29} *mut {l30} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
        // 219: stdout:   l29 = UNIQUE | NON_NULL, (empty)
        // 219: stdout:   l30 = UNIQUE | NON_NULL, (empty)
        if verbose >= 0 as libc::c_int {
        // 220: verbose: typeof(_22) = *mut {l34} i32
        // 220: verbose:   l34 = UNIQUE | NON_NULL, (empty)
            lglflushtimers(lgl4sigh);
            // 221: lgl4sigh: typeof(_25) = *mut {l38} LGL
            // 221: lgl4sigh:   l38 = UNIQUE | NON_NULL, (empty)
            // 221: lgl4sigh: typeof(_26) = *mut {l40} *mut {l41} LGL
            // 221: lgl4sigh:   l40 = UNIQUE | NON_NULL, (empty)
            // 221: lgl4sigh:   l41 = UNIQUE | NON_NULL, (empty)
            lglstats(lgl4sigh);
            // 222: lgl4sigh: typeof(_28) = *mut {l44} LGL
            // 222: lgl4sigh:   l44 = UNIQUE | NON_NULL, (empty)
            // 222: lgl4sigh: typeof(_29) = *mut {l46} *mut {l47} LGL
            // 222: lgl4sigh:   l46 = UNIQUE | NON_NULL, (empty)
            // 222: lgl4sigh:   l47 = UNIQUE | NON_NULL, (empty)
            caughtsigmsg(sig);
        }
    }
    resetsighandlers();
    if (getenv(b"LGLNABORT\0" as *const u8 as *const libc::c_char)).is_null() {
    // 227: (getenv(b"LGLNA ... har)): typeof(_35) = *mut {l54} i8
    // 227: (getenv(b"LGLNA ... har)):   l54 = UNIQUE | NON_NULL, (empty)
    // 227: b"LGLNABORT\0"  ... _char: typeof(_36) = *const {l56} i8
    // 227: b"LGLNABORT\0"  ... _char:   l56 = UNIQUE | NON_NULL, (empty)
    // 227: b"LGLNABORT\0"  ... st u8: typeof(_37) = *const {l58} u8
    // 227: b"LGLNABORT\0"  ... st u8:   l58 = UNIQUE | NON_NULL, (empty)
    // 227: b"LGLNABORT\0": typeof(_38) = *const {l60} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
    // 227: b"LGLNABORT\0":   l60 = UNIQUE | NON_NULL, (empty)
    // 227: b"LGLNABORT\0": typeof(_39) = & {l62} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
    // 227: b"LGLNABORT\0":   l62 = UNIQUE | NON_NULL, FIXED
    // 227: b"LGLNABORT\0"  ... st u8: typeof(_37 = move _38 as *const u8 (Pointer(ArrayToPointer))) = *const {l75} u8
    // 227: b"LGLNABORT\0"  ... st u8:   l75 = UNIQUE | NON_NULL, (empty)
    // 227: b"LGLNABORT\0": typeof(_39 = const b"LGLNABORT\x00") = & {l73} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
    // 227: b"LGLNABORT\0":   l73 = UNIQUE | NON_NULL, (empty)
    // 227: b"LGLNABORT\0"  ... _char: typeof(_36 = move _37 as *const i8 (Misc)) = *const {l76} i8
    // 227: b"LGLNABORT\0"  ... _char:   l76 = UNIQUE | NON_NULL, (empty)
    // 227: b"LGLNABORT\0": typeof(_38 = &raw const (*_39)) = *const {l74} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
    // 227: b"LGLNABORT\0":   l74 = UNIQUE | NON_NULL, (empty)
        raise(sig);
    } else {
        exit(1 as libc::c_int);
    };
}
unsafe extern "C" fn setsighandlers() {
    sig_int_handler = signal(
    // 234: sig_int_handler: typeof(_5) = *mut {l5} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 234: sig_int_handler:   l5 = UNIQUE | NON_NULL, (empty)
        2 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    sig_segv_handler = signal(
    // 238: sig_segv_handler: typeof(_10) = *mut {l11} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 238: sig_segv_handler:   l11 = UNIQUE | NON_NULL, (empty)
        11 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    sig_abrt_handler = signal(
    // 242: sig_abrt_handler: typeof(_15) = *mut {l17} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 242: sig_abrt_handler:   l17 = UNIQUE | NON_NULL, (empty)
        6 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    sig_term_handler = signal(
    // 246: sig_term_handler: typeof(_20) = *mut {l23} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 246: sig_term_handler:   l23 = UNIQUE | NON_NULL, (empty)
        15 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    sig_bus_handler = signal(
    // 250: sig_bus_handler: typeof(_25) = *mut {l29} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 250: sig_bus_handler:   l29 = UNIQUE | NON_NULL, (empty)
        7 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
static timelimit: libc::c_int = -(1 as libc::c_int);
static caughtalarm: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn catchalrm(mut sig: libc::c_int) {
    if caughtalarm == 0 {
    // 258: caughtalarm: typeof(_4) = *mut {l4} i32
    // 258: caughtalarm:   l4 = UNIQUE | NON_NULL, (empty)
        caughtalarm = 1 as libc::c_int;
        // 259: caughtalarm: typeof(_6) = *mut {l7} i32
        // 259: caughtalarm:   l7 = UNIQUE | NON_NULL, (empty)
        caughtsigmsg(sig);
        if timelimit >= 0 as libc::c_int {
        // 261: timelimit: typeof(_11) = *mut {l13} i32
        // 261: timelimit:   l13 = UNIQUE | NON_NULL, (empty)
            printf(
                b"c time limit of %d reached after %.1f seconds\nc\n\0" as *const u8
                // 263: b"c time limit  ... _char: typeof(_14) = *const {l17} i8
                // 263: b"c time limit  ... _char:   l17 = UNIQUE | NON_NULL, (empty)
                // 263: b"c time limit  ... st u8: typeof(_15) = *const {l19} u8
                // 263: b"c time limit  ... st u8:   l19 = UNIQUE | NON_NULL, (empty)
                // 263: b"c time limit  ... \n\0": typeof(_16) = *const {l21} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                // 263: b"c time limit  ... \n\0":   l21 = UNIQUE | NON_NULL, (empty)
                // 263: b"c time limit  ... \n\0": typeof(_17) = & {l23} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                // 263: b"c time limit  ... \n\0":   l23 = UNIQUE | NON_NULL, FIXED
                // 263: b"c time limit  ... _char: typeof(_14 = move _15 as *const i8 (Misc)) = *const {l43} i8
                // 263: b"c time limit  ... _char:   l43 = UNIQUE | NON_NULL, (empty)
                // 263: b"c time limit  ... \n\0": typeof(_16 = &raw const (*_17)) = *const {l41} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                // 263: b"c time limit  ... \n\0":   l41 = UNIQUE | NON_NULL, (empty)
                // 263: b"c time limit  ... \n\0": typeof(_17 = const b"c time limit of %d reached after %.1f seconds\nc\n\x00") = & {l40} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                // 263: b"c time limit  ... \n\0":   l40 = UNIQUE | NON_NULL, (empty)
                // 263: b"c time limit  ... st u8: typeof(_15 = move _16 as *const u8 (Pointer(ArrayToPointer))) = *const {l42} u8
                // 263: b"c time limit  ... st u8:   l42 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
                timelimit,
                // 265: timelimit: typeof(_19) = *mut {l26} i32
                // 265: timelimit:   l26 = UNIQUE | NON_NULL, (empty)
                lglsec(lgl4sigh),
                // 266: lgl4sigh: typeof(_21) = *mut {l29} LGL
                // 266: lgl4sigh:   l29 = UNIQUE | NON_NULL, (empty)
                // 266: lgl4sigh: typeof(_22) = *mut {l31} *mut {l32} LGL
                // 266: lgl4sigh:   l31 = UNIQUE | NON_NULL, (empty)
                // 266: lgl4sigh:   l32 = UNIQUE | NON_NULL, (empty)
            );
            fflush(stdout);
            // 268: stdout: typeof(_24) = *mut {l35} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
            // 268: stdout:   l35 = UNIQUE | NON_NULL, (empty)
            // 268: stdout: typeof(_25) = *mut {l37} *mut {l38} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
            // 268: stdout:   l37 = UNIQUE | NON_NULL, (empty)
            // 268: stdout:   l38 = UNIQUE | NON_NULL, (empty)
        }
    }
}
unsafe extern "C" fn checkalarm<'h0>(mut ptr: &'h0 (libc::c_void)) -> libc::c_int {
// 272: mut ptr: typeof(_1) = *mut {g1} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 272: mut ptr:   g1 = UNIQUE | NON_NULL, (empty)
    return caughtalarm;
    // 273: caughtalarm: typeof(_3) = *mut {l3} i32
    // 273: caughtalarm:   l3 = READ | UNIQUE | NON_NULL, (empty)
}
unsafe fn checkalarm_shim(arg0: *mut libc::c_void) -> libc::c_int {
    {
    let safe_arg0 = &*arg0.cast_const();
    let safe_result = checkalarm(safe_arg0);
    let result = safe_result;
    result
}
}

unsafe extern "C" fn flushobuf(
    mut obuf: *mut OBuf,
    // 276: mut obuf: typeof(_1) = *mut {g2} DefId(0:160 ~ lglmain[0c57]::OBuf)
    // 276: mut obuf:   g2 = UNIQUE | NON_NULL, FIXED
    mut simponly: libc::c_int,
    mut file: *mut FILE,
    // 278: mut file: typeof(_3) = *mut {g3} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
    // 278: mut file:   g3 = UNIQUE | NON_NULL, FIXED
) {
    let fresh0 = (*obuf).pos;
    (*obuf).pos = (*obuf).pos + 1;
    (*obuf).line[fresh0 as usize] = '\n' as i32 as libc::c_char;
    let fresh1 = (*obuf).pos;
    (*obuf).pos = (*obuf).pos + 1;
    (*obuf).line[fresh1 as usize] = 0 as libc::c_int as libc::c_char;
    if simponly != 0 {
        fputs(b"c \0" as *const u8 as *const libc::c_char, file);
        // 287: b"c \0" as *con ... _char: typeof(_24) = *const {l24} i8
        // 287: b"c \0" as *con ... _char:   l24 = UNIQUE | NON_NULL, (empty)
        // 287: b"c \0" as *const u8: typeof(_25) = *const {l26} u8
        // 287: b"c \0" as *const u8:   l26 = UNIQUE | NON_NULL, (empty)
        // 287: b"c \0": typeof(_26) = *const {l28} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 287: b"c \0":   l28 = UNIQUE | NON_NULL, (empty)
        // 287: b"c \0": typeof(_27) = & {l30} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 287: b"c \0":   l30 = UNIQUE | NON_NULL, FIXED
        // 287: file: typeof(_28) = *mut {l32} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
        // 287: file:   l32 = UNIQUE | NON_NULL, (empty)
        // 287: b"c \0" as *const u8: typeof(_25 = move _26 as *const u8 (Pointer(ArrayToPointer))) = *const {l52} u8
        // 287: b"c \0" as *const u8:   l52 = UNIQUE | NON_NULL, (empty)
        // 287: b"c \0": typeof(_27 = const b"c \x00") = & {l50} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 287: b"c \0":   l50 = UNIQUE | NON_NULL, (empty)
        // 287: b"c \0" as *con ... _char: typeof(_24 = move _25 as *const i8 (Misc)) = *const {l53} i8
        // 287: b"c \0" as *con ... _char:   l53 = UNIQUE | NON_NULL, (empty)
        // 287: b"c \0": typeof(_26 = &raw const (*_27)) = *const {l51} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 287: b"c \0":   l51 = UNIQUE | NON_NULL, (empty)
    }
    fputc('v' as i32, file);
    // 289: file: typeof(_31) = *mut {l36} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
    // 289: file:   l36 = UNIQUE | NON_NULL, (empty)
    fputs(((*obuf).line).as_mut_ptr(), file);
    // 290: ((*obuf).line). ... ptr(): typeof(_33) = *const {l39} i8
    // 290: ((*obuf).line). ... ptr():   l39 = UNIQUE | NON_NULL, (empty)
    // 290: ((*obuf).line). ... ptr(): typeof(_34) = *mut {l41} i8
    // 290: ((*obuf).line). ... ptr():   l41 = UNIQUE | NON_NULL, (empty)
    // 290: ((*obuf).line). ... ptr(): typeof(_35) = &mut {l43} [i8]
    // 290: ((*obuf).line). ... ptr():   l43 = UNIQUE | NON_NULL, FIXED
    // 290: ((*obuf).line). ... ptr(): typeof(_36) = &mut {l45} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000051)) }]
    // 290: ((*obuf).line). ... ptr():   l45 = UNIQUE | NON_NULL, (empty)
    // 290: file: typeof(_37) = *mut {l47} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
    // 290: file:   l47 = UNIQUE | NON_NULL, (empty)
    // 290: ((*obuf).line). ... ptr(): typeof(_33 = move _34 as *const i8 (Pointer(MutToConstPointer))) = *const {l56} i8
    // 290: ((*obuf).line). ... ptr():   l56 = UNIQUE | NON_NULL, (empty)
    // 290: ((*obuf).line). ... ptr(): typeof(_36 = &mut ((*_1).0: [i8; 81])) = &mut {l54} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000051)) }]
    // 290: ((*obuf).line). ... ptr():   l54 = UNIQUE | NON_NULL, (empty)
    // 290: ((*obuf).line). ... ptr(): typeof(_35 = move _36 as &mut [i8] (Pointer(Unsize))) = &mut {l55} [i8]
    // 290: ((*obuf).line). ... ptr():   l55 = UNIQUE | NON_NULL, FIXED
    (*obuf).pos = 0 as libc::c_int;
}
unsafe extern "C" fn print2obuf(
    mut obuf: *mut OBuf,
    // 294: mut obuf: typeof(_1) = *mut {g4} DefId(0:160 ~ lglmain[0c57]::OBuf)
    // 294: mut obuf:   g4 = UNIQUE | NON_NULL, FIXED
    mut i: libc::c_int,
    mut simponly: libc::c_int,
    mut file: *mut FILE,
    // 297: mut file: typeof(_4) = *mut {g5} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
    // 297: mut file:   g5 = UNIQUE | NON_NULL, FIXED
) {
    let mut str: [libc::c_char; 20] = [0; 20];
    let mut len: libc::c_int = 0;
    sprintf(
        str.as_mut_ptr(),
        // 302: str.as_mut_ptr(): typeof(_8) = *mut {l8} i8
        // 302: str.as_mut_ptr():   l8 = UNIQUE | NON_NULL, (empty)
        // 302: str.as_mut_ptr(): typeof(_9) = &mut {l10} [i8]
        // 302: str.as_mut_ptr():   l10 = UNIQUE | NON_NULL, FIXED
        // 302: str.as_mut_ptr(): typeof(_10) = &mut {l12} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
        // 302: str.as_mut_ptr():   l12 = UNIQUE | NON_NULL, (empty)
        // 302: str.as_mut_ptr(): typeof(_9 = move _10 as &mut [i8] (Pointer(Unsize))) = &mut {l68} [i8]
        // 302: str.as_mut_ptr():   l68 = UNIQUE | NON_NULL, FIXED
        // 302: str.as_mut_ptr(): typeof(_10 = &mut _5) = &mut {l67} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
        // 302: str.as_mut_ptr():   l67 = UNIQUE | NON_NULL, (empty)
        b" %d\0" as *const u8 as *const libc::c_char,
        // 303: b" %d\0" as *co ... _char: typeof(_11) = *const {l14} i8
        // 303: b" %d\0" as *co ... _char:   l14 = UNIQUE | NON_NULL, (empty)
        // 303: b" %d\0" as *co ... st u8: typeof(_12) = *const {l16} u8
        // 303: b" %d\0" as *co ... st u8:   l16 = UNIQUE | NON_NULL, (empty)
        // 303: b" %d\0": typeof(_13) = *const {l18} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
        // 303: b" %d\0":   l18 = UNIQUE | NON_NULL, (empty)
        // 303: b" %d\0": typeof(_14) = & {l20} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
        // 303: b" %d\0":   l20 = UNIQUE | NON_NULL, FIXED
        // 303: b" %d\0" as *co ... st u8: typeof(_12 = move _13 as *const u8 (Pointer(ArrayToPointer))) = *const {l71} u8
        // 303: b" %d\0" as *co ... st u8:   l71 = UNIQUE | NON_NULL, (empty)
        // 303: b" %d\0": typeof(_14 = const b" %d\x00") = & {l69} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
        // 303: b" %d\0":   l69 = UNIQUE | NON_NULL, (empty)
        // 303: b" %d\0": typeof(_13 = &raw const (*_14)) = *const {l70} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
        // 303: b" %d\0":   l70 = UNIQUE | NON_NULL, (empty)
        // 303: b" %d\0" as *co ... _char: typeof(_11 = move _12 as *const i8 (Misc)) = *const {l72} i8
        // 303: b" %d\0" as *co ... _char:   l72 = UNIQUE | NON_NULL, (empty)
        i,
    );
    len = strlen(str.as_mut_ptr()) as libc::c_int;
    // 306: str.as_mut_ptr(): typeof(_17) = *const {l24} i8
    // 306: str.as_mut_ptr():   l24 = UNIQUE | NON_NULL, (empty)
    // 306: str.as_mut_ptr(): typeof(_18) = *mut {l26} i8
    // 306: str.as_mut_ptr():   l26 = UNIQUE | NON_NULL, (empty)
    // 306: str.as_mut_ptr(): typeof(_19) = &mut {l28} [i8]
    // 306: str.as_mut_ptr():   l28 = UNIQUE | NON_NULL, FIXED
    // 306: str.as_mut_ptr(): typeof(_20) = &mut {l30} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
    // 306: str.as_mut_ptr():   l30 = UNIQUE | NON_NULL, (empty)
    // 306: str.as_mut_ptr(): typeof(_17 = move _18 as *const i8 (Pointer(MutToConstPointer))) = *const {l75} i8
    // 306: str.as_mut_ptr():   l75 = UNIQUE | NON_NULL, (empty)
    // 306: str.as_mut_ptr(): typeof(_20 = &mut _5) = &mut {l73} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
    // 306: str.as_mut_ptr():   l73 = UNIQUE | NON_NULL, (empty)
    // 306: str.as_mut_ptr(): typeof(_19 = move _20 as &mut [i8] (Pointer(Unsize))) = &mut {l74} [i8]
    // 306: str.as_mut_ptr():   l74 = UNIQUE | NON_NULL, FIXED
    if (*obuf).pos + len > 79 as libc::c_int {
        flushobuf(obuf, simponly, file);
        // 308: obuf: typeof(_29) = *mut {l40} DefId(0:160 ~ lglmain[0c57]::OBuf)
        // 308: obuf:   l40 = UNIQUE | NON_NULL, (empty)
        // 308: file: typeof(_31) = *mut {l43} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
        // 308: file:   l43 = UNIQUE | NON_NULL, (empty)
    }
    strcpy(
    // 310: strcpy( ((*obuf ... (), ): typeof(_32) = *mut {l45} i8
    // 310: strcpy( ((*obuf ... (), ):   l45 = UNIQUE | NON_NULL, (empty)
        ((*obuf).line).as_mut_ptr().offset((*obuf).pos as isize),
        // 311: ((*obuf).line). ... size): typeof(_33) = *mut {l47} i8
        // 311: ((*obuf).line). ... size):   l47 = UNIQUE | NON_NULL, (empty)
        // 311: ((*obuf).line). ... ptr(): typeof(_34) = *mut {l49} i8
        // 311: ((*obuf).line). ... ptr():   l49 = UNIQUE | NON_NULL, (empty)
        // 311: ((*obuf).line). ... ptr(): typeof(_35) = &mut {l51} [i8]
        // 311: ((*obuf).line). ... ptr():   l51 = UNIQUE | NON_NULL, FIXED
        // 311: ((*obuf).line). ... ptr(): typeof(_36) = &mut {l53} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000051)) }]
        // 311: ((*obuf).line). ... ptr():   l53 = UNIQUE | NON_NULL, (empty)
        // 311: ((*obuf).line). ... ptr(): typeof(_35 = move _36 as &mut [i8] (Pointer(Unsize))) = &mut {l77} [i8]
        // 311: ((*obuf).line). ... ptr():   l77 = UNIQUE | NON_NULL, FIXED
        // 311: ((*obuf).line). ... ptr(): typeof(_36 = &mut ((*_1).0: [i8; 81])) = &mut {l76} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000051)) }]
        // 311: ((*obuf).line). ... ptr():   l76 = UNIQUE | NON_NULL, (empty)
        str.as_mut_ptr(),
        // 312: str.as_mut_ptr(): typeof(_39) = *const {l57} i8
        // 312: str.as_mut_ptr():   l57 = UNIQUE | NON_NULL, (empty)
        // 312: str.as_mut_ptr(): typeof(_40) = *mut {l59} i8
        // 312: str.as_mut_ptr():   l59 = UNIQUE | NON_NULL, (empty)
        // 312: str.as_mut_ptr(): typeof(_41) = &mut {l61} [i8]
        // 312: str.as_mut_ptr():   l61 = UNIQUE | NON_NULL, FIXED
        // 312: str.as_mut_ptr(): typeof(_42) = &mut {l63} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
        // 312: str.as_mut_ptr():   l63 = UNIQUE | NON_NULL, (empty)
        // 312: str.as_mut_ptr(): typeof(_41 = move _42 as &mut [i8] (Pointer(Unsize))) = &mut {l79} [i8]
        // 312: str.as_mut_ptr():   l79 = UNIQUE | NON_NULL, FIXED
        // 312: str.as_mut_ptr(): typeof(_42 = &mut _5) = &mut {l78} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
        // 312: str.as_mut_ptr():   l78 = UNIQUE | NON_NULL, (empty)
        // 312: str.as_mut_ptr(): typeof(_39 = move _40 as *const i8 (Pointer(MutToConstPointer))) = *const {l80} i8
        // 312: str.as_mut_ptr():   l80 = UNIQUE | NON_NULL, (empty)
    );
    (*obuf).pos += len;
}
unsafe extern "C" fn writefile(
    mut name: *const libc::c_char,
    // 317: mut name: typeof(_1) = *const {g6} i8
    // 317: mut name:   g6 = UNIQUE | NON_NULL, FIXED
    mut clptr: *mut libc::c_int,
    // 318: mut clptr: typeof(_2) = *mut {g7} i32
    // 318: mut clptr:   g7 = UNIQUE | NON_NULL, FIXED
) -> *mut FILE {
// 319: *mut FILE: typeof(_0) = *mut {g8} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
// 319: *mut FILE:   g8 = UNIQUE | NON_NULL, FIXED
    let mut len: libc::c_int = strlen(name) as libc::c_int;
    // 320: name: typeof(_6) = *const {l6} i8
    // 320: name:   l6 = UNIQUE | NON_NULL, (empty)
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    // 321: mut tmp: typeof(_7) = *mut {l8} i8
    // 321: mut tmp:   l8 = UNIQUE | NON_NULL, (empty)
    // 321: 0 as *mut libc: ... _char: typeof(_7 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l124} i8
    // 321: 0 as *mut libc: ... _char:   l124 = UNIQUE | NON_NULL, (empty)
    let mut res: *mut FILE = 0 as *mut FILE;
    // 322: mut res: typeof(_8) = *mut {l10} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
    // 322: mut res:   l10 = UNIQUE | NON_NULL, (empty)
    // 322: 0 as *mut FILE: typeof(_8 = const 0_usize as *mut _IO_FILE (PointerFromExposedAddress)) = *mut {l125} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
    // 322: 0 as *mut FILE:   l125 = UNIQUE | NON_NULL, (empty)
    if len >= 3 as libc::c_int
        && strcmp(
            name.offset(len as isize)
            // 325: name.offset(len ... ize)): typeof(_16) = *const {l19} i8
            // 325: name.offset(len ... ize)):   l19 = UNIQUE | NON_NULL, (empty)
            // 325: name.offset(len ... size): typeof(_17) = *const {l21} i8
            // 325: name.offset(len ... size):   l21 = UNIQUE | NON_NULL, (empty)
            // 325: name: typeof(_18) = *const {l23} i8
            // 325: name:   l23 = UNIQUE | NON_NULL, (empty)
                .offset(-(3 as libc::c_int as isize)),
            b".gz\0" as *const u8 as *const libc::c_char,
            // 327: b".gz\0" as *co ... _char: typeof(_25) = *const {l31} i8
            // 327: b".gz\0" as *co ... _char:   l31 = UNIQUE | NON_NULL, (empty)
            // 327: b".gz\0" as *co ... st u8: typeof(_26) = *const {l33} u8
            // 327: b".gz\0" as *co ... st u8:   l33 = UNIQUE | NON_NULL, (empty)
            // 327: b".gz\0": typeof(_27) = *const {l35} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 327: b".gz\0":   l35 = UNIQUE | NON_NULL, (empty)
            // 327: b".gz\0": typeof(_28) = & {l37} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 327: b".gz\0":   l37 = UNIQUE | NON_NULL, FIXED
            // 327: b".gz\0" as *co ... _char: typeof(_25 = move _26 as *const i8 (Misc)) = *const {l129} i8
            // 327: b".gz\0" as *co ... _char:   l129 = UNIQUE | NON_NULL, (empty)
            // 327: b".gz\0": typeof(_28 = const b".gz\x00") = & {l126} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 327: b".gz\0":   l126 = UNIQUE | NON_NULL, (empty)
            // 327: b".gz\0" as *co ... st u8: typeof(_26 = move _27 as *const u8 (Pointer(ArrayToPointer))) = *const {l128} u8
            // 327: b".gz\0" as *co ... st u8:   l128 = UNIQUE | NON_NULL, (empty)
            // 327: b".gz\0": typeof(_27 = &raw const (*_28)) = *const {l127} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 327: b".gz\0":   l127 = UNIQUE | NON_NULL, (empty)
        ) == 0
    {
        tmp = malloc((len + 20 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
        // 330: malloc((len + 2 ... long): typeof(_29) = *mut {l39} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 330: malloc((len + 2 ... long):   l39 = UNIQUE | NON_NULL, (empty)
        // 330: tmp = malloc((l ... _char: typeof(_7 = move _29 as *mut i8 (Misc)) = *mut {l130} i8
        // 330: tmp = malloc((l ... _char:   l130 = UNIQUE | NON_NULL, (empty)
        unlink(name);
        // 331: name: typeof(_36) = *const {l47} i8
        // 331: name:   l47 = UNIQUE | NON_NULL, (empty)
        sprintf(
            tmp,
            // 333: tmp: typeof(_38) = *mut {l50} i8
            // 333: tmp:   l50 = UNIQUE | NON_NULL, (empty)
            b"gzip -c > %s\0" as *const u8 as *const libc::c_char,
            // 334: b"gzip -c > %s\ ... _char: typeof(_39) = *const {l52} i8
            // 334: b"gzip -c > %s\ ... _char:   l52 = UNIQUE | NON_NULL, (empty)
            // 334: b"gzip -c > %s\ ... st u8: typeof(_40) = *const {l54} u8
            // 334: b"gzip -c > %s\ ... st u8:   l54 = UNIQUE | NON_NULL, (empty)
            // 334: b"gzip -c > %s\0": typeof(_41) = *const {l56} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 334: b"gzip -c > %s\0":   l56 = UNIQUE | NON_NULL, (empty)
            // 334: b"gzip -c > %s\0": typeof(_42) = & {l58} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 334: b"gzip -c > %s\0":   l58 = UNIQUE | NON_NULL, FIXED
            // 334: b"gzip -c > %s\0": typeof(_42 = const b"gzip -c > %s\x00") = & {l131} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 334: b"gzip -c > %s\0":   l131 = UNIQUE | NON_NULL, (empty)
            // 334: b"gzip -c > %s\ ... st u8: typeof(_40 = move _41 as *const u8 (Pointer(ArrayToPointer))) = *const {l133} u8
            // 334: b"gzip -c > %s\ ... st u8:   l133 = UNIQUE | NON_NULL, (empty)
            // 334: b"gzip -c > %s\ ... _char: typeof(_39 = move _40 as *const i8 (Misc)) = *const {l134} i8
            // 334: b"gzip -c > %s\ ... _char:   l134 = UNIQUE | NON_NULL, (empty)
            // 334: b"gzip -c > %s\0": typeof(_41 = &raw const (*_42)) = *const {l132} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 334: b"gzip -c > %s\0":   l132 = UNIQUE | NON_NULL, (empty)
            name,
            // 335: name: typeof(_43) = *const {l60} i8
            // 335: name:   l60 = UNIQUE | NON_NULL, (empty)
        );
        res = popen(tmp, b"w\0" as *const u8 as *const libc::c_char);
        // 337: popen(tmp, b"w\ ... char): typeof(_44) = *mut {l62} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
        // 337: popen(tmp, b"w\ ... char):   l62 = UNIQUE | NON_NULL, (empty)
        // 337: tmp: typeof(_45) = *const {l64} i8
        // 337: tmp:   l64 = UNIQUE | NON_NULL, (empty)
        // 337: tmp: typeof(_46) = *mut {l66} i8
        // 337: tmp:   l66 = UNIQUE | NON_NULL, (empty)
        // 337: b"w\0" as *cons ... _char: typeof(_47) = *const {l68} i8
        // 337: b"w\0" as *cons ... _char:   l68 = UNIQUE | NON_NULL, (empty)
        // 337: b"w\0" as *const u8: typeof(_48) = *const {l70} u8
        // 337: b"w\0" as *const u8:   l70 = UNIQUE | NON_NULL, (empty)
        // 337: b"w\0": typeof(_49) = *const {l72} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 337: b"w\0":   l72 = UNIQUE | NON_NULL, (empty)
        // 337: b"w\0": typeof(_50) = & {l74} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 337: b"w\0":   l74 = UNIQUE | NON_NULL, FIXED
        // 337: b"w\0": typeof(_49 = &raw const (*_50)) = *const {l137} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 337: b"w\0":   l137 = UNIQUE | NON_NULL, (empty)
        // 337: b"w\0" as *const u8: typeof(_48 = move _49 as *const u8 (Pointer(ArrayToPointer))) = *const {l138} u8
        // 337: b"w\0" as *const u8:   l138 = UNIQUE | NON_NULL, (empty)
        // 337: tmp: typeof(_45 = move _46 as *const i8 (Pointer(MutToConstPointer))) = *const {l135} i8
        // 337: tmp:   l135 = UNIQUE | NON_NULL, (empty)
        // 337: b"w\0" as *cons ... _char: typeof(_47 = move _48 as *const i8 (Misc)) = *const {l139} i8
        // 337: b"w\0" as *cons ... _char:   l139 = UNIQUE | NON_NULL, (empty)
        // 337: b"w\0": typeof(_50 = const b"w\x00") = & {l136} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 337: b"w\0":   l136 = UNIQUE | NON_NULL, (empty)
        if !res.is_null() {
        // 338: res: typeof(_54) = *mut {l79} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
        // 338: res:   l79 = UNIQUE | NON_NULL, (empty)
            *clptr = 2 as libc::c_int;
        }
        free(tmp as *mut libc::c_void);
        // 341: tmp as *mut lib ... _void: typeof(_57) = *mut {l83} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 341: tmp as *mut lib ... _void:   l83 = UNIQUE | NON_NULL, (empty)
        // 341: tmp: typeof(_58) = *mut {l85} i8
        // 341: tmp:   l85 = UNIQUE | NON_NULL, (empty)
        // 341: tmp as *mut lib ... _void: typeof(_57 = move _58 as *mut libc::c_void (Misc)) = *mut {l140} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 341: tmp as *mut lib ... _void:   l140 = UNIQUE | NON_NULL, (empty)
    } else {
        res = fopen(name, b"w\0" as *const u8 as *const libc::c_char);
        // 343: fopen(name, b"w ... char): typeof(_59) = *mut {l87} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
        // 343: fopen(name, b"w ... char):   l87 = UNIQUE | NON_NULL, (empty)
        // 343: name: typeof(_60) = *const {l89} i8
        // 343: name:   l89 = UNIQUE | NON_NULL, (empty)
        // 343: b"w\0" as *cons ... _char: typeof(_61) = *const {l91} i8
        // 343: b"w\0" as *cons ... _char:   l91 = UNIQUE | NON_NULL, (empty)
        // 343: b"w\0" as *const u8: typeof(_62) = *const {l93} u8
        // 343: b"w\0" as *const u8:   l93 = UNIQUE | NON_NULL, (empty)
        // 343: b"w\0": typeof(_63) = *const {l95} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 343: b"w\0":   l95 = UNIQUE | NON_NULL, (empty)
        // 343: b"w\0": typeof(_64) = & {l97} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 343: b"w\0":   l97 = UNIQUE | NON_NULL, FIXED
        // 343: b"w\0": typeof(_63 = &raw const (*_64)) = *const {l142} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 343: b"w\0":   l142 = UNIQUE | NON_NULL, (empty)
        // 343: b"w\0": typeof(_64 = const b"w\x00") = & {l141} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 343: b"w\0":   l141 = UNIQUE | NON_NULL, (empty)
        // 343: b"w\0" as *cons ... _char: typeof(_61 = move _62 as *const i8 (Misc)) = *const {l144} i8
        // 343: b"w\0" as *cons ... _char:   l144 = UNIQUE | NON_NULL, (empty)
        // 343: b"w\0" as *const u8: typeof(_62 = move _63 as *const u8 (Pointer(ArrayToPointer))) = *const {l143} u8
        // 343: b"w\0" as *const u8:   l143 = UNIQUE | NON_NULL, (empty)
        if !res.is_null() {
        // 344: res: typeof(_67) = *mut {l101} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
        // 344: res:   l101 = UNIQUE | NON_NULL, (empty)
            *clptr = 1 as libc::c_int;
        }
    }
    if res.is_null() {
    // 348: res: typeof(_71) = *mut {l106} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
    // 348: res:   l106 = UNIQUE | NON_NULL, (empty)
        fprintf(
            stderr,
            // 350: stderr: typeof(_73) = *mut {l109} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
            // 350: stderr:   l109 = UNIQUE | NON_NULL, (empty)
            // 350: stderr: typeof(_74) = *mut {l111} *mut {l112} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
            // 350: stderr:   l111 = UNIQUE | NON_NULL, (empty)
            // 350: stderr:   l112 = UNIQUE | NON_NULL, (empty)
            b"*** lingeling error: can not write %s\n\0" as *const u8 as *const libc::c_char,
            // 351: b"*** lingeling ... _char: typeof(_75) = *const {l114} i8
            // 351: b"*** lingeling ... _char:   l114 = UNIQUE | NON_NULL, (empty)
            // 351: b"*** lingeling ... st u8: typeof(_76) = *const {l116} u8
            // 351: b"*** lingeling ... st u8:   l116 = UNIQUE | NON_NULL, (empty)
            // 351: b"*** lingeling ... \n\0": typeof(_77) = *const {l118} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
            // 351: b"*** lingeling ... \n\0":   l118 = UNIQUE | NON_NULL, (empty)
            // 351: b"*** lingeling ... \n\0": typeof(_78) = & {l120} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
            // 351: b"*** lingeling ... \n\0":   l120 = UNIQUE | NON_NULL, FIXED
            // 351: b"*** lingeling ... _char: typeof(_75 = move _76 as *const i8 (Misc)) = *const {l148} i8
            // 351: b"*** lingeling ... _char:   l148 = UNIQUE | NON_NULL, (empty)
            // 351: b"*** lingeling ... st u8: typeof(_76 = move _77 as *const u8 (Pointer(ArrayToPointer))) = *const {l147} u8
            // 351: b"*** lingeling ... st u8:   l147 = UNIQUE | NON_NULL, (empty)
            // 351: b"*** lingeling ... \n\0": typeof(_78 = const b"*** lingeling error: can not write %s\n\x00") = & {l145} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
            // 351: b"*** lingeling ... \n\0":   l145 = UNIQUE | NON_NULL, (empty)
            // 351: b"*** lingeling ... \n\0": typeof(_77 = &raw const (*_78)) = *const {l146} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
            // 351: b"*** lingeling ... \n\0":   l146 = UNIQUE | NON_NULL, (empty)
            name,
            // 352: name: typeof(_79) = *const {l122} i8
            // 352: name:   l122 = UNIQUE | NON_NULL, (empty)
        );
    }
    return res;
}
unsafe extern "C" fn closefile(mut file: *mut FILE, mut type_0: libc::c_int) {
// 357: mut file: typeof(_1) = *mut {g9} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
// 357: mut file:   g9 = UNIQUE | NON_NULL, FIXED
    if type_0 == 1 as libc::c_int {
        fclose(file);
        // 359: file: typeof(_8) = *mut {l8} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
        // 359: file:   l8 = UNIQUE | NON_NULL, (empty)
    }
    if type_0 == 2 as libc::c_int {
        pclose(file);
        // 362: file: typeof(_13) = *mut {l14} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
        // 362: file:   l14 = UNIQUE | NON_NULL, (empty)
    }
}
unsafe extern "C" fn lgltravcounter<'h0>(mut voidptr: &'h0 mut i32, mut lit: libc::c_int) {
// 365: mut voidptr: typeof(_1) = *mut {g10} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 365: mut voidptr:   g10 = READ | WRITE | UNIQUE | NON_NULL, (empty)
    let mut cntptr: &mut (libc::c_int) = (voidptr);
    // 366: mut cntptr: typeof(_3) = *mut {l3} i32
    // 366: mut cntptr:   l3 = READ | WRITE | UNIQUE | NON_NULL, (empty)
    // 366: voidptr: typeof(_4) = *mut {l5} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 366: voidptr:   l5 = READ | WRITE | UNIQUE | NON_NULL, (empty)
    // 366: voidptr as *mut ... c_int: typeof(_3 = move _4 as *mut i32 (Misc)) = *mut {l11} i32
    // 366: voidptr as *mut ... c_int:   l11 = READ | WRITE | UNIQUE | NON_NULL, (empty)
    if lit == 0 {
        *cntptr += 1 as libc::c_int;
    }
}
unsafe fn lgltravcounter_shim(arg0: *mut libc::c_void, arg1: libc::c_int) {
    {
    let safe_arg0 = &mut *arg0;
    let safe_arg1 = arg1;
    let safe_result = lgltravcounter(safe_arg0,safe_arg1);
    let result = safe_result;
    result
}
}

unsafe extern "C" fn lglpushtarget(mut target: libc::c_int) {
    if ntargets == sztargets {
    // 372: ntargets: typeof(_5) = *mut {l5} i32
    // 372: ntargets:   l5 = READ | UNIQUE | NON_NULL, (empty)
    // 372: sztargets: typeof(_7) = *mut {l8} i32
    // 372: sztargets:   l8 = READ | UNIQUE | NON_NULL, (empty)
        sztargets = if sztargets != 0 {
        // 373: sztargets: typeof(_11) = *mut {l13} i32
        // 373: sztargets:   l13 = READ | UNIQUE | NON_NULL, (empty)
        // 373: sztargets: typeof(_16) = *mut {l20} i32
        // 373: sztargets:   l20 = READ | WRITE | UNIQUE | NON_NULL, (empty)
            2 as libc::c_int * sztargets
            // 374: sztargets: typeof(_14) = *mut {l17} i32
            // 374: sztargets:   l17 = READ | UNIQUE | NON_NULL, (empty)
        } else {
            1 as libc::c_int
        };
        targets = realloc(
        // 378: realloc( target ... g), ): typeof(_17) = *mut {l22} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 378: realloc( target ... g), ):   l22 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | FREE | NON_NULL, (empty)
        // 378: targets: typeof(_27) = *mut {l37} *mut {g104} i32
        // 378: targets:   l37 = READ | WRITE | UNIQUE | NON_NULL, (empty)
        // 378: targets:   g104 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | FREE | NON_NULL, (empty)
        // 378: targets = reall ... c_int: typeof((*_27) = move _17 as *mut i32 (Misc)) = *mut {l58} i32
        // 378: targets = reall ... c_int:   l58 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | FREE | NON_NULL, (empty)
            targets as *mut libc::c_void,
            // 379: targets as *mut ... _void: typeof(_18) = *mut {l24} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 379: targets as *mut ... _void:   l24 = UNIQUE | FREE | NON_NULL, (empty)
            // 379: targets: typeof(_19) = *mut {l26} i32
            // 379: targets:   l26 = UNIQUE | FREE | NON_NULL, (empty)
            // 379: targets: typeof(_20) = *mut {l28} *mut {g104} i32
            // 379: targets:   l28 = READ | UNIQUE | NON_NULL, (empty)
            // 379: targets:   g104 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | FREE | NON_NULL, (empty)
            // 379: targets as *mut ... _void: typeof(_18 = move _19 as *mut libc::c_void (Misc)) = *mut {l57} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 379: targets as *mut ... _void:   l57 = UNIQUE | FREE | NON_NULL, (empty)
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(sztargets as libc::c_ulong),
                // 381: sztargets: typeof(_26) = *mut {l35} i32
                // 381: sztargets:   l35 = READ | UNIQUE | NON_NULL, (empty)
        ) as *mut libc::c_int;
    }
    let fresh2 = ntargets;
    // 384: ntargets: typeof(_29) = *mut {l40} i32
    // 384: ntargets:   l40 = READ | UNIQUE | NON_NULL, (empty)
    ntargets = ntargets + 1;
    // 385: ntargets: typeof(_31) = *mut {l43} i32
    // 385: ntargets:   l43 = READ | UNIQUE | NON_NULL, (empty)
    // 385: ntargets: typeof(_33) = *mut {l46} i32
    // 385: ntargets:   l46 = READ | WRITE | UNIQUE | NON_NULL, (empty)
    *targets.offset(fresh2 as isize) = target;
    // 386: targets.offset( ... size): typeof(_35) = *mut {l49} i32
    // 386: targets.offset( ... size):   l49 = READ | WRITE | UNIQUE | NON_NULL, (empty)
    // 386: targets: typeof(_36) = *mut {l51} i32
    // 386: targets:   l51 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | NON_NULL, (empty)
    // 386: targets: typeof(_37) = *mut {l53} *mut {g104} i32
    // 386: targets:   l53 = READ | UNIQUE | NON_NULL, (empty)
    // 386: targets:   g104 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | FREE | NON_NULL, (empty)
}
static primes: [libc::c_int; 5] = [
    200000033 as libc::c_int,
    200000039 as libc::c_int,
    200000051 as libc::c_int,
    200000069 as libc::c_int,
    200000081 as libc::c_int,
];
static mut nprimes: libc::c_uint = 0;
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
// 396: mut argv: typeof(_2) = *mut {g11} *mut {g12} i8
// 396: mut argv:   g11 = UNIQUE | NON_NULL, FIXED
// 396: mut argv:   g12 = UNIQUE | NON_NULL, FIXED
    let mut current_block: u64;
    let mut res: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut clout: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut lineno: libc::c_int = 0;
    let mut simponly: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut target: libc::c_int = 0;
    let mut iname: *const libc::c_char = 0 as *const libc::c_char;
    // 408: mut iname: typeof(_15) = *const {l15} i8
    // 408: mut iname:   l15 = UNIQUE | NON_NULL, (empty)
    // 408: 0 as *const lib ... _char: typeof(_15 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l3130} i8
    // 408: 0 as *const lib ... _char:   l3130 = UNIQUE | NON_NULL, (empty)
    let mut oname: *const libc::c_char = 0 as *const libc::c_char;
    // 409: mut oname: typeof(_16) = *const {l17} i8
    // 409: mut oname:   l17 = UNIQUE | NON_NULL, (empty)
    // 409: 0 as *const lib ... _char: typeof(_16 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l3131} i8
    // 409: 0 as *const lib ... _char:   l3131 = UNIQUE | NON_NULL, (empty)
    let mut pname: *const libc::c_char = 0 as *const libc::c_char;
    // 410: mut pname: typeof(_17) = *const {l19} i8
    // 410: mut pname:   l19 = UNIQUE | NON_NULL, (empty)
    // 410: 0 as *const lib ... _char: typeof(_17 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l3132} i8
    // 410: 0 as *const lib ... _char:   l3132 = UNIQUE | NON_NULL, (empty)
    let mut match_0: *const libc::c_char = 0 as *const libc::c_char;
    // 411: mut match_0: typeof(_18) = *const {l21} i8
    // 411: mut match_0:   l21 = UNIQUE | NON_NULL, (empty)
    // 411: 0 as *const lib ... _char: typeof(_18 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l3133} i8
    // 411: 0 as *const lib ... _char:   l3133 = UNIQUE | NON_NULL, (empty)
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    // 412: mut p: typeof(_19) = *const {l23} i8
    // 412: mut p:   l23 = UNIQUE | NON_NULL, (empty)
    // 412: 0 as *const lib ... _char: typeof(_19 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l3134} i8
    // 412: 0 as *const lib ... _char:   l3134 = UNIQUE | NON_NULL, (empty)
    let mut err: *const libc::c_char = 0 as *const libc::c_char;
    // 413: mut err: typeof(_20) = *const {l25} i8
    // 413: mut err:   l25 = UNIQUE | NON_NULL, (empty)
    // 413: 0 as *const lib ... _char: typeof(_20 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l3135} i8
    // 413: 0 as *const lib ... _char:   l3135 = UNIQUE | NON_NULL, (empty)
    let mut thanks: *const libc::c_char = 0 as *const libc::c_char;
    // 414: mut thanks: typeof(_21) = *const {l27} i8
    // 414: mut thanks:   l27 = UNIQUE | NON_NULL, (empty)
    // 414: 0 as *const lib ... _char: typeof(_21 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l3136} i8
    // 414: 0 as *const lib ... _char:   l3136 = UNIQUE | NON_NULL, (empty)
    let mut out: *mut FILE = 0 as *mut FILE;
    // 415: mut out: typeof(_22) = *mut {l29} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
    // 415: mut out:   l29 = UNIQUE | NON_NULL, (empty)
    // 415: 0 as *mut FILE: typeof(_22 = const 0_usize as *mut _IO_FILE (PointerFromExposedAddress)) = *mut {l3137} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
    // 415: 0 as *mut FILE:   l3137 = UNIQUE | NON_NULL, (empty)
    let mut pfile: *mut FILE = 0 as *mut FILE;
    // 416: mut pfile: typeof(_23) = *mut {l31} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
    // 416: mut pfile:   l31 = UNIQUE | NON_NULL, (empty)
    // 416: 0 as *mut FILE: typeof(_23 = const 0_usize as *mut _IO_FILE (PointerFromExposedAddress)) = *mut {l3138} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
    // 416: 0 as *mut FILE:   l3138 = UNIQUE | NON_NULL, (empty)
    let mut maxvar: libc::c_int = 0;
    let mut lit: libc::c_int = 0;
    let mut nopts: libc::c_int = 0;
    let mut simplevel: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    // 421: mut tmp: typeof(_28) = *mut {l37} i8
    // 421: mut tmp:   l37 = UNIQUE | NON_NULL, (empty)
    // 421: 0 as *mut libc: ... _char: typeof(_28 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l3139} i8
    // 421: 0 as *mut libc: ... _char:   l3139 = UNIQUE | NON_NULL, (empty)
    let mut lgl: *mut LGL = 0 as *mut LGL;
    // 422: mut lgl: typeof(_29) = *mut {l39} LGL
    // 422: mut lgl:   l39 = UNIQUE | NON_NULL, (empty)
    // 422: 0 as *mut LGL: typeof(_29 = const 0_usize as *mut LGL (PointerFromExposedAddress)) = *mut {l3140} LGL
    // 422: 0 as *mut LGL:   l3140 = UNIQUE | NON_NULL, (empty)
    let mut obuf: OBuf = OBuf {
        line: [0; 81],
        pos: 0,
    };
    lineno = 1 as libc::c_int;
    out = 0 as *mut FILE;
    // 428: out = 0 as *mut FILE: typeof(_22 = const 0_usize as *mut _IO_FILE (PointerFromExposedAddress)) = *mut {l3141} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
    // 428: out = 0 as *mut FILE:   l3141 = UNIQUE | NON_NULL, (empty)
    simplevel = 0 as libc::c_int;
    simponly = simplevel;
    clout = simponly;
    res = clout;
    thanks = 0 as *const libc::c_char;
    // 433: thanks = 0 as * ... _char: typeof(_21 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l3142} i8
    // 433: thanks = 0 as * ... _char:   l3142 = UNIQUE | NON_NULL, (empty)
    pname = thanks;
    // 434: thanks: typeof(_37) = *const {l48} i8
    // 434: thanks:   l48 = UNIQUE | NON_NULL, (empty)
    oname = pname;
    // 435: pname: typeof(_38) = *const {l50} i8
    // 435: pname:   l50 = UNIQUE | NON_NULL, (empty)
    iname = oname;
    // 436: oname: typeof(_39) = *const {l52} i8
    // 436: oname:   l52 = UNIQUE | NON_NULL, (empty)
    lgl = lglinit();
    // 437: lglinit(): typeof(_40) = *mut {l54} LGL
    // 437: lglinit():   l54 = UNIQUE | NON_NULL, (empty)
    lgl4sigh = lgl;
    // 438: lgl: typeof(_41) = *mut {l56} LGL
    // 438: lgl:   l56 = UNIQUE | NON_NULL, (empty)
    // 438: lgl4sigh: typeof(_42) = *mut {l58} *mut {l59} LGL
    // 438: lgl4sigh:   l58 = UNIQUE | NON_NULL, (empty)
    // 438: lgl4sigh:   l59 = UNIQUE | NON_NULL, (empty)
    setsighandlers();
    i = 1 as libc::c_int;
    loop {
        if !(i < argc) {
            current_block = 17727836384662615028;
            break;
        }
        if strcmp(
            *argv.offset(i as isize),
            // 447: *argv.offset(i  ... size): typeof(_56) = *const {l74} i8
            // 447: *argv.offset(i  ... size):   l74 = UNIQUE | NON_NULL, (empty)
            // 447: *argv.offset(i  ... size): typeof(_57) = *mut {l76} i8
            // 447: *argv.offset(i  ... size):   l76 = UNIQUE | NON_NULL, (empty)
            // 447: argv.offset(i a ... size): typeof(_58) = *mut {l78} *mut {l79} i8
            // 447: argv.offset(i a ... size):   l78 = UNIQUE | NON_NULL, (empty)
            // 447: argv.offset(i a ... size):   l79 = UNIQUE | NON_NULL, (empty)
            // 447: argv: typeof(_59) = *mut {l81} *mut {l82} i8
            // 447: argv:   l81 = UNIQUE | NON_NULL, (empty)
            // 447: argv:   l82 = UNIQUE | NON_NULL, (empty)
            // 447: *argv.offset(i  ... size): typeof(_56 = move _57 as *const i8 (Pointer(MutToConstPointer))) = *const {l3143} i8
            // 447: *argv.offset(i  ... size):   l3143 = UNIQUE | NON_NULL, (empty)
            b"-h\0" as *const u8 as *const libc::c_char,
            // 448: b"-h\0" as *con ... _char: typeof(_62) = *const {l86} i8
            // 448: b"-h\0" as *con ... _char:   l86 = UNIQUE | NON_NULL, (empty)
            // 448: b"-h\0" as *const u8: typeof(_63) = *const {l88} u8
            // 448: b"-h\0" as *const u8:   l88 = UNIQUE | NON_NULL, (empty)
            // 448: b"-h\0": typeof(_64) = *const {l90} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 448: b"-h\0":   l90 = UNIQUE | NON_NULL, (empty)
            // 448: b"-h\0": typeof(_65) = & {l92} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 448: b"-h\0":   l92 = UNIQUE | NON_NULL, FIXED
            // 448: b"-h\0" as *con ... _char: typeof(_62 = move _63 as *const i8 (Misc)) = *const {l3147} i8
            // 448: b"-h\0" as *con ... _char:   l3147 = UNIQUE | NON_NULL, (empty)
            // 448: b"-h\0": typeof(_64 = &raw const (*_65)) = *const {l3145} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 448: b"-h\0":   l3145 = UNIQUE | NON_NULL, (empty)
            // 448: b"-h\0": typeof(_65 = const b"-h\x00") = & {l3144} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 448: b"-h\0":   l3144 = UNIQUE | NON_NULL, (empty)
            // 448: b"-h\0" as *const u8: typeof(_63 = move _64 as *const u8 (Pointer(ArrayToPointer))) = *const {l3146} u8
            // 448: b"-h\0" as *const u8:   l3146 = UNIQUE | NON_NULL, (empty)
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                // 451: *argv.offset(i  ... size): typeof(_68) = *const {l96} i8
                // 451: *argv.offset(i  ... size):   l96 = UNIQUE | NON_NULL, (empty)
                // 451: *argv.offset(i  ... size): typeof(_69) = *mut {l98} i8
                // 451: *argv.offset(i  ... size):   l98 = UNIQUE | NON_NULL, (empty)
                // 451: argv.offset(i a ... size): typeof(_70) = *mut {l100} *mut {l101} i8
                // 451: argv.offset(i a ... size):   l100 = UNIQUE | NON_NULL, (empty)
                // 451: argv.offset(i a ... size):   l101 = UNIQUE | NON_NULL, (empty)
                // 451: argv: typeof(_71) = *mut {l103} *mut {l104} i8
                // 451: argv:   l103 = UNIQUE | NON_NULL, (empty)
                // 451: argv:   l104 = UNIQUE | NON_NULL, (empty)
                // 451: *argv.offset(i  ... size): typeof(_68 = move _69 as *const i8 (Pointer(MutToConstPointer))) = *const {l3148} i8
                // 451: *argv.offset(i  ... size):   l3148 = UNIQUE | NON_NULL, (empty)
                b"--help\0" as *const u8 as *const libc::c_char,
                // 452: b"--help\0" as  ... _char: typeof(_74) = *const {l108} i8
                // 452: b"--help\0" as  ... _char:   l108 = UNIQUE | NON_NULL, (empty)
                // 452: b"--help\0" as  ... st u8: typeof(_75) = *const {l110} u8
                // 452: b"--help\0" as  ... st u8:   l110 = UNIQUE | NON_NULL, (empty)
                // 452: b"--help\0": typeof(_76) = *const {l112} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 452: b"--help\0":   l112 = UNIQUE | NON_NULL, (empty)
                // 452: b"--help\0": typeof(_77) = & {l114} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 452: b"--help\0":   l114 = UNIQUE | NON_NULL, FIXED
                // 452: b"--help\0" as  ... _char: typeof(_74 = move _75 as *const i8 (Misc)) = *const {l3152} i8
                // 452: b"--help\0" as  ... _char:   l3152 = UNIQUE | NON_NULL, (empty)
                // 452: b"--help\0": typeof(_76 = &raw const (*_77)) = *const {l3150} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 452: b"--help\0":   l3150 = UNIQUE | NON_NULL, (empty)
                // 452: b"--help\0" as  ... st u8: typeof(_75 = move _76 as *const u8 (Pointer(ArrayToPointer))) = *const {l3151} u8
                // 452: b"--help\0" as  ... st u8:   l3151 = UNIQUE | NON_NULL, (empty)
                // 452: b"--help\0": typeof(_77 = const b"--help\x00") = & {l3149} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 452: b"--help\0":   l3149 = UNIQUE | NON_NULL, (empty)
            ) == 0
        {
            printf(
                b"usage: lingeling [<option> ...][<file>[.<suffix>]]\n\0" as *const u8
                // 456: b"usage: lingel ... _char: typeof(_80) = *const {l118} i8
                // 456: b"usage: lingel ... _char:   l118 = UNIQUE | NON_NULL, (empty)
                // 456: b"usage: lingel ... st u8: typeof(_81) = *const {l120} u8
                // 456: b"usage: lingel ... st u8:   l120 = UNIQUE | NON_NULL, (empty)
                // 456: b"usage: lingel ... \n\0": typeof(_82) = *const {l122} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                // 456: b"usage: lingel ... \n\0":   l122 = UNIQUE | NON_NULL, (empty)
                // 456: b"usage: lingel ... \n\0": typeof(_83) = & {l124} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                // 456: b"usage: lingel ... \n\0":   l124 = UNIQUE | NON_NULL, FIXED
                // 456: b"usage: lingel ... _char: typeof(_80 = move _81 as *const i8 (Misc)) = *const {l3156} i8
                // 456: b"usage: lingel ... _char:   l3156 = UNIQUE | NON_NULL, (empty)
                // 456: b"usage: lingel ... st u8: typeof(_81 = move _82 as *const u8 (Pointer(ArrayToPointer))) = *const {l3155} u8
                // 456: b"usage: lingel ... st u8:   l3155 = UNIQUE | NON_NULL, (empty)
                // 456: b"usage: lingel ... \n\0": typeof(_83 = const b"usage: lingeling [<option> ...][<file>[.<suffix>]]\n\x00") = & {l3153} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                // 456: b"usage: lingel ... \n\0":   l3153 = UNIQUE | NON_NULL, (empty)
                // 456: b"usage: lingel ... \n\0": typeof(_82 = &raw const (*_83)) = *const {l3154} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                // 456: b"usage: lingel ... \n\0":   l3154 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            // 459: b"\n\0" as *con ... _char: typeof(_85) = *const {l127} i8
            // 459: b"\n\0" as *con ... _char:   l127 = UNIQUE | NON_NULL, (empty)
            // 459: b"\n\0" as *const u8: typeof(_86) = *const {l129} u8
            // 459: b"\n\0" as *const u8:   l129 = UNIQUE | NON_NULL, (empty)
            // 459: b"\n\0": typeof(_87) = *const {l131} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 459: b"\n\0":   l131 = UNIQUE | NON_NULL, (empty)
            // 459: b"\n\0": typeof(_88) = & {l133} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 459: b"\n\0":   l133 = UNIQUE | NON_NULL, FIXED
            // 459: b"\n\0": typeof(_88 = const b"\n\x00") = & {l3157} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 459: b"\n\0":   l3157 = UNIQUE | NON_NULL, (empty)
            // 459: b"\n\0" as *con ... _char: typeof(_85 = move _86 as *const i8 (Misc)) = *const {l3160} i8
            // 459: b"\n\0" as *con ... _char:   l3160 = UNIQUE | NON_NULL, (empty)
            // 459: b"\n\0" as *const u8: typeof(_86 = move _87 as *const u8 (Pointer(ArrayToPointer))) = *const {l3159} u8
            // 459: b"\n\0" as *const u8:   l3159 = UNIQUE | NON_NULL, (empty)
            // 459: b"\n\0": typeof(_87 = &raw const (*_88)) = *const {l3158} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 459: b"\n\0":   l3158 = UNIQUE | NON_NULL, (empty)
            printf(
                b"where <option> is one of the following:\n\0" as *const u8 as *const libc::c_char,
                // 461: b"where <option ... _char: typeof(_90) = *const {l136} i8
                // 461: b"where <option ... _char:   l136 = UNIQUE | NON_NULL, (empty)
                // 461: b"where <option ... st u8: typeof(_91) = *const {l138} u8
                // 461: b"where <option ... st u8:   l138 = UNIQUE | NON_NULL, (empty)
                // 461: b"where <option ... \n\0": typeof(_92) = *const {l140} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 461: b"where <option ... \n\0":   l140 = UNIQUE | NON_NULL, (empty)
                // 461: b"where <option ... \n\0": typeof(_93) = & {l142} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 461: b"where <option ... \n\0":   l142 = UNIQUE | NON_NULL, FIXED
                // 461: b"where <option ... \n\0": typeof(_92 = &raw const (*_93)) = *const {l3162} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 461: b"where <option ... \n\0":   l3162 = UNIQUE | NON_NULL, (empty)
                // 461: b"where <option ... st u8: typeof(_91 = move _92 as *const u8 (Pointer(ArrayToPointer))) = *const {l3163} u8
                // 461: b"where <option ... st u8:   l3163 = UNIQUE | NON_NULL, (empty)
                // 461: b"where <option ... \n\0": typeof(_93 = const b"where <option> is one of the following:\n\x00") = & {l3161} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 461: b"where <option ... \n\0":   l3161 = UNIQUE | NON_NULL, (empty)
                // 461: b"where <option ... _char: typeof(_90 = move _91 as *const i8 (Misc)) = *const {l3164} i8
                // 461: b"where <option ... _char:   l3164 = UNIQUE | NON_NULL, (empty)
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            // 463: b"\n\0" as *con ... _char: typeof(_95) = *const {l145} i8
            // 463: b"\n\0" as *con ... _char:   l145 = UNIQUE | NON_NULL, (empty)
            // 463: b"\n\0" as *const u8: typeof(_96) = *const {l147} u8
            // 463: b"\n\0" as *const u8:   l147 = UNIQUE | NON_NULL, (empty)
            // 463: b"\n\0": typeof(_97) = *const {l149} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 463: b"\n\0":   l149 = UNIQUE | NON_NULL, (empty)
            // 463: b"\n\0": typeof(_98) = & {l151} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 463: b"\n\0":   l151 = UNIQUE | NON_NULL, FIXED
            // 463: b"\n\0" as *con ... _char: typeof(_95 = move _96 as *const i8 (Misc)) = *const {l3168} i8
            // 463: b"\n\0" as *con ... _char:   l3168 = UNIQUE | NON_NULL, (empty)
            // 463: b"\n\0" as *const u8: typeof(_96 = move _97 as *const u8 (Pointer(ArrayToPointer))) = *const {l3167} u8
            // 463: b"\n\0" as *const u8:   l3167 = UNIQUE | NON_NULL, (empty)
            // 463: b"\n\0": typeof(_98 = const b"\n\x00") = & {l3165} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 463: b"\n\0":   l3165 = UNIQUE | NON_NULL, (empty)
            // 463: b"\n\0": typeof(_97 = &raw const (*_98)) = *const {l3166} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 463: b"\n\0":   l3166 = UNIQUE | NON_NULL, (empty)
            printf(
                b"-q               be quiet (same as '--verbose=-1')\n\0" as *const u8
                // 465: b"-q be quiet ( ... _char: typeof(_100) = *const {l154} i8
                // 465: b"-q be quiet ( ... _char:   l154 = UNIQUE | NON_NULL, (empty)
                // 465: b"-q be quiet ( ... st u8: typeof(_101) = *const {l156} u8
                // 465: b"-q be quiet ( ... st u8:   l156 = UNIQUE | NON_NULL, (empty)
                // 465: b"-q be quiet ( ... \n\0": typeof(_102) = *const {l158} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                // 465: b"-q be quiet ( ... \n\0":   l158 = UNIQUE | NON_NULL, (empty)
                // 465: b"-q be quiet ( ... \n\0": typeof(_103) = & {l160} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                // 465: b"-q be quiet ( ... \n\0":   l160 = UNIQUE | NON_NULL, FIXED
                // 465: b"-q be quiet ( ... \n\0": typeof(_103 = const b"-q               be quiet (same as \'--verbose=-1\')\n\x00") = & {l3169} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                // 465: b"-q be quiet ( ... \n\0":   l3169 = UNIQUE | NON_NULL, (empty)
                // 465: b"-q be quiet ( ... \n\0": typeof(_102 = &raw const (*_103)) = *const {l3170} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                // 465: b"-q be quiet ( ... \n\0":   l3170 = UNIQUE | NON_NULL, (empty)
                // 465: b"-q be quiet ( ... st u8: typeof(_101 = move _102 as *const u8 (Pointer(ArrayToPointer))) = *const {l3171} u8
                // 465: b"-q be quiet ( ... st u8:   l3171 = UNIQUE | NON_NULL, (empty)
                // 465: b"-q be quiet ( ... _char: typeof(_100 = move _101 as *const i8 (Misc)) = *const {l3172} i8
                // 465: b"-q be quiet ( ... _char:   l3172 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
            printf(
                b"-s               only simplify and print to output file\n\0" as *const u8
                // 469: b"-s only simpl ... _char: typeof(_105) = *const {l163} i8
                // 469: b"-s only simpl ... _char:   l163 = UNIQUE | NON_NULL, (empty)
                // 469: b"-s only simpl ... st u8: typeof(_106) = *const {l165} u8
                // 469: b"-s only simpl ... st u8:   l165 = UNIQUE | NON_NULL, (empty)
                // 469: b"-s only simpl ... \n\0": typeof(_107) = *const {l167} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                // 469: b"-s only simpl ... \n\0":   l167 = UNIQUE | NON_NULL, (empty)
                // 469: b"-s only simpl ... \n\0": typeof(_108) = & {l169} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                // 469: b"-s only simpl ... \n\0":   l169 = UNIQUE | NON_NULL, FIXED
                // 469: b"-s only simpl ... st u8: typeof(_106 = move _107 as *const u8 (Pointer(ArrayToPointer))) = *const {l3175} u8
                // 469: b"-s only simpl ... st u8:   l3175 = UNIQUE | NON_NULL, (empty)
                // 469: b"-s only simpl ... _char: typeof(_105 = move _106 as *const i8 (Misc)) = *const {l3176} i8
                // 469: b"-s only simpl ... _char:   l3176 = UNIQUE | NON_NULL, (empty)
                // 469: b"-s only simpl ... \n\0": typeof(_107 = &raw const (*_108)) = *const {l3174} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                // 469: b"-s only simpl ... \n\0":   l3174 = UNIQUE | NON_NULL, (empty)
                // 469: b"-s only simpl ... \n\0": typeof(_108 = const b"-s               only simplify and print to output file\n\x00") = & {l3173} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                // 469: b"-s only simpl ... \n\0":   l3173 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
            printf(
                b"-O<L>            set simplification level to <L>\n\0" as *const u8
                // 473: b"-O<L> set sim ... _char: typeof(_110) = *const {l172} i8
                // 473: b"-O<L> set sim ... _char:   l172 = UNIQUE | NON_NULL, (empty)
                // 473: b"-O<L> set sim ... st u8: typeof(_111) = *const {l174} u8
                // 473: b"-O<L> set sim ... st u8:   l174 = UNIQUE | NON_NULL, (empty)
                // 473: b"-O<L> set sim ... \n\0": typeof(_112) = *const {l176} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
                // 473: b"-O<L> set sim ... \n\0":   l176 = UNIQUE | NON_NULL, (empty)
                // 473: b"-O<L> set sim ... \n\0": typeof(_113) = & {l178} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
                // 473: b"-O<L> set sim ... \n\0":   l178 = UNIQUE | NON_NULL, FIXED
                // 473: b"-O<L> set sim ... \n\0": typeof(_112 = &raw const (*_113)) = *const {l3178} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
                // 473: b"-O<L> set sim ... \n\0":   l3178 = UNIQUE | NON_NULL, (empty)
                // 473: b"-O<L> set sim ... \n\0": typeof(_113 = const b"-O<L>            set simplification level to <L>\n\x00") = & {l3177} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
                // 473: b"-O<L> set sim ... \n\0":   l3177 = UNIQUE | NON_NULL, (empty)
                // 473: b"-O<L> set sim ... _char: typeof(_110 = move _111 as *const i8 (Misc)) = *const {l3180} i8
                // 473: b"-O<L> set sim ... _char:   l3180 = UNIQUE | NON_NULL, (empty)
                // 473: b"-O<L> set sim ... st u8: typeof(_111 = move _112 as *const u8 (Pointer(ArrayToPointer))) = *const {l3179} u8
                // 473: b"-O<L> set sim ... st u8:   l3179 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
            printf(
                b"-o <output>      set output file (default 'stdout')\n\0" as *const u8
                // 477: b"-o <output> s ... _char: typeof(_115) = *const {l181} i8
                // 477: b"-o <output> s ... _char:   l181 = UNIQUE | NON_NULL, (empty)
                // 477: b"-o <output> s ... st u8: typeof(_116) = *const {l183} u8
                // 477: b"-o <output> s ... st u8:   l183 = UNIQUE | NON_NULL, (empty)
                // 477: b"-o <output> s ... \n\0": typeof(_117) = *const {l185} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000035)) }]
                // 477: b"-o <output> s ... \n\0":   l185 = UNIQUE | NON_NULL, (empty)
                // 477: b"-o <output> s ... \n\0": typeof(_118) = & {l187} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000035)) }]
                // 477: b"-o <output> s ... \n\0":   l187 = UNIQUE | NON_NULL, FIXED
                // 477: b"-o <output> s ... _char: typeof(_115 = move _116 as *const i8 (Misc)) = *const {l3184} i8
                // 477: b"-o <output> s ... _char:   l3184 = UNIQUE | NON_NULL, (empty)
                // 477: b"-o <output> s ... st u8: typeof(_116 = move _117 as *const u8 (Pointer(ArrayToPointer))) = *const {l3183} u8
                // 477: b"-o <output> s ... st u8:   l3183 = UNIQUE | NON_NULL, (empty)
                // 477: b"-o <output> s ... \n\0": typeof(_118 = const b"-o <output>      set output file (default \'stdout\')\n\x00") = & {l3181} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000035)) }]
                // 477: b"-o <output> s ... \n\0":   l3181 = UNIQUE | NON_NULL, (empty)
                // 477: b"-o <output> s ... \n\0": typeof(_117 = &raw const (*_118)) = *const {l3182} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000035)) }]
                // 477: b"-o <output> s ... \n\0":   l3182 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
            printf(
                b"-p <options>     read options from file\n\0" as *const u8 as *const libc::c_char,
                // 481: b"-p <options>  ... _char: typeof(_120) = *const {l190} i8
                // 481: b"-p <options>  ... _char:   l190 = UNIQUE | NON_NULL, (empty)
                // 481: b"-p <options>  ... st u8: typeof(_121) = *const {l192} u8
                // 481: b"-p <options>  ... st u8:   l192 = UNIQUE | NON_NULL, (empty)
                // 481: b"-p <options>  ... \n\0": typeof(_122) = *const {l194} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 481: b"-p <options>  ... \n\0":   l194 = UNIQUE | NON_NULL, (empty)
                // 481: b"-p <options>  ... \n\0": typeof(_123) = & {l196} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 481: b"-p <options>  ... \n\0":   l196 = UNIQUE | NON_NULL, FIXED
                // 481: b"-p <options>  ... st u8: typeof(_121 = move _122 as *const u8 (Pointer(ArrayToPointer))) = *const {l3187} u8
                // 481: b"-p <options>  ... st u8:   l3187 = UNIQUE | NON_NULL, (empty)
                // 481: b"-p <options>  ... _char: typeof(_120 = move _121 as *const i8 (Misc)) = *const {l3188} i8
                // 481: b"-p <options>  ... _char:   l3188 = UNIQUE | NON_NULL, (empty)
                // 481: b"-p <options>  ... \n\0": typeof(_122 = &raw const (*_123)) = *const {l3186} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 481: b"-p <options>  ... \n\0":   l3186 = UNIQUE | NON_NULL, (empty)
                // 481: b"-p <options>  ... \n\0": typeof(_123 = const b"-p <options>     read options from file\n\x00") = & {l3185} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 481: b"-p <options>  ... \n\0":   l3185 = UNIQUE | NON_NULL, (empty)
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            // 483: b"\n\0" as *con ... _char: typeof(_125) = *const {l199} i8
            // 483: b"\n\0" as *con ... _char:   l199 = UNIQUE | NON_NULL, (empty)
            // 483: b"\n\0" as *const u8: typeof(_126) = *const {l201} u8
            // 483: b"\n\0" as *const u8:   l201 = UNIQUE | NON_NULL, (empty)
            // 483: b"\n\0": typeof(_127) = *const {l203} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 483: b"\n\0":   l203 = UNIQUE | NON_NULL, (empty)
            // 483: b"\n\0": typeof(_128) = & {l205} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 483: b"\n\0":   l205 = UNIQUE | NON_NULL, FIXED
            // 483: b"\n\0": typeof(_128 = const b"\n\x00") = & {l3189} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 483: b"\n\0":   l3189 = UNIQUE | NON_NULL, (empty)
            // 483: b"\n\0": typeof(_127 = &raw const (*_128)) = *const {l3190} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 483: b"\n\0":   l3190 = UNIQUE | NON_NULL, (empty)
            // 483: b"\n\0" as *con ... _char: typeof(_125 = move _126 as *const i8 (Misc)) = *const {l3192} i8
            // 483: b"\n\0" as *con ... _char:   l3192 = UNIQUE | NON_NULL, (empty)
            // 483: b"\n\0" as *const u8: typeof(_126 = move _127 as *const u8 (Pointer(ArrayToPointer))) = *const {l3191} u8
            // 483: b"\n\0" as *const u8:   l3191 = UNIQUE | NON_NULL, (empty)
            printf(b"-T <seconds>     set time limit\n\0" as *const u8 as *const libc::c_char);
            // 484: b"-T <seconds>  ... _char: typeof(_130) = *const {l208} i8
            // 484: b"-T <seconds>  ... _char:   l208 = UNIQUE | NON_NULL, (empty)
            // 484: b"-T <seconds>  ... st u8: typeof(_131) = *const {l210} u8
            // 484: b"-T <seconds>  ... st u8:   l210 = UNIQUE | NON_NULL, (empty)
            // 484: b"-T <seconds>  ... \n\0": typeof(_132) = *const {l212} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
            // 484: b"-T <seconds>  ... \n\0":   l212 = UNIQUE | NON_NULL, (empty)
            // 484: b"-T <seconds>  ... \n\0": typeof(_133) = & {l214} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
            // 484: b"-T <seconds>  ... \n\0":   l214 = UNIQUE | NON_NULL, FIXED
            // 484: b"-T <seconds>  ... \n\0": typeof(_133 = const b"-T <seconds>     set time limit\n\x00") = & {l3193} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
            // 484: b"-T <seconds>  ... \n\0":   l3193 = UNIQUE | NON_NULL, (empty)
            // 484: b"-T <seconds>  ... st u8: typeof(_131 = move _132 as *const u8 (Pointer(ArrayToPointer))) = *const {l3195} u8
            // 484: b"-T <seconds>  ... st u8:   l3195 = UNIQUE | NON_NULL, (empty)
            // 484: b"-T <seconds>  ... _char: typeof(_130 = move _131 as *const i8 (Misc)) = *const {l3196} i8
            // 484: b"-T <seconds>  ... _char:   l3196 = UNIQUE | NON_NULL, (empty)
            // 484: b"-T <seconds>  ... \n\0": typeof(_132 = &raw const (*_133)) = *const {l3194} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
            // 484: b"-T <seconds>  ... \n\0":   l3194 = UNIQUE | NON_NULL, (empty)
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            // 485: b"\n\0" as *con ... _char: typeof(_135) = *const {l217} i8
            // 485: b"\n\0" as *con ... _char:   l217 = UNIQUE | NON_NULL, (empty)
            // 485: b"\n\0" as *const u8: typeof(_136) = *const {l219} u8
            // 485: b"\n\0" as *const u8:   l219 = UNIQUE | NON_NULL, (empty)
            // 485: b"\n\0": typeof(_137) = *const {l221} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 485: b"\n\0":   l221 = UNIQUE | NON_NULL, (empty)
            // 485: b"\n\0": typeof(_138) = & {l223} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 485: b"\n\0":   l223 = UNIQUE | NON_NULL, FIXED
            // 485: b"\n\0" as *const u8: typeof(_136 = move _137 as *const u8 (Pointer(ArrayToPointer))) = *const {l3199} u8
            // 485: b"\n\0" as *const u8:   l3199 = UNIQUE | NON_NULL, (empty)
            // 485: b"\n\0" as *con ... _char: typeof(_135 = move _136 as *const i8 (Misc)) = *const {l3200} i8
            // 485: b"\n\0" as *con ... _char:   l3200 = UNIQUE | NON_NULL, (empty)
            // 485: b"\n\0": typeof(_137 = &raw const (*_138)) = *const {l3198} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 485: b"\n\0":   l3198 = UNIQUE | NON_NULL, (empty)
            // 485: b"\n\0": typeof(_138 = const b"\n\x00") = & {l3197} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 485: b"\n\0":   l3197 = UNIQUE | NON_NULL, (empty)
            printf(
                b"-a <assumption>  use multiple assumptions\n\0" as *const u8
                // 487: b"-a <assumptio ... _char: typeof(_140) = *const {l226} i8
                // 487: b"-a <assumptio ... _char:   l226 = UNIQUE | NON_NULL, (empty)
                // 487: b"-a <assumptio ... st u8: typeof(_141) = *const {l228} u8
                // 487: b"-a <assumptio ... st u8:   l228 = UNIQUE | NON_NULL, (empty)
                // 487: b"-a <assumptio ... \n\0": typeof(_142) = *const {l230} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                // 487: b"-a <assumptio ... \n\0":   l230 = UNIQUE | NON_NULL, (empty)
                // 487: b"-a <assumptio ... \n\0": typeof(_143) = & {l232} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                // 487: b"-a <assumptio ... \n\0":   l232 = UNIQUE | NON_NULL, FIXED
                // 487: b"-a <assumptio ... st u8: typeof(_141 = move _142 as *const u8 (Pointer(ArrayToPointer))) = *const {l3203} u8
                // 487: b"-a <assumptio ... st u8:   l3203 = UNIQUE | NON_NULL, (empty)
                // 487: b"-a <assumptio ... \n\0": typeof(_142 = &raw const (*_143)) = *const {l3202} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                // 487: b"-a <assumptio ... \n\0":   l3202 = UNIQUE | NON_NULL, (empty)
                // 487: b"-a <assumptio ... \n\0": typeof(_143 = const b"-a <assumption>  use multiple assumptions\n\x00") = & {l3201} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                // 487: b"-a <assumptio ... \n\0":   l3201 = UNIQUE | NON_NULL, (empty)
                // 487: b"-a <assumptio ... _char: typeof(_140 = move _141 as *const i8 (Misc)) = *const {l3204} i8
                // 487: b"-a <assumptio ... _char:   l3204 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            // 490: b"\n\0" as *con ... _char: typeof(_145) = *const {l235} i8
            // 490: b"\n\0" as *con ... _char:   l235 = UNIQUE | NON_NULL, (empty)
            // 490: b"\n\0" as *const u8: typeof(_146) = *const {l237} u8
            // 490: b"\n\0" as *const u8:   l237 = UNIQUE | NON_NULL, (empty)
            // 490: b"\n\0": typeof(_147) = *const {l239} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 490: b"\n\0":   l239 = UNIQUE | NON_NULL, (empty)
            // 490: b"\n\0": typeof(_148) = & {l241} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 490: b"\n\0":   l241 = UNIQUE | NON_NULL, FIXED
            // 490: b"\n\0": typeof(_148 = const b"\n\x00") = & {l3205} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 490: b"\n\0":   l3205 = UNIQUE | NON_NULL, (empty)
            // 490: b"\n\0" as *con ... _char: typeof(_145 = move _146 as *const i8 (Misc)) = *const {l3208} i8
            // 490: b"\n\0" as *con ... _char:   l3208 = UNIQUE | NON_NULL, (empty)
            // 490: b"\n\0": typeof(_147 = &raw const (*_148)) = *const {l3206} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 490: b"\n\0":   l3206 = UNIQUE | NON_NULL, (empty)
            // 490: b"\n\0" as *const u8: typeof(_146 = move _147 as *const u8 (Pointer(ArrayToPointer))) = *const {l3207} u8
            // 490: b"\n\0" as *const u8:   l3207 = UNIQUE | NON_NULL, (empty)
            printf(
                b"-h|--help        print command line option summary\n\0" as *const u8
                // 492: b"-h|--help pri ... _char: typeof(_150) = *const {l244} i8
                // 492: b"-h|--help pri ... _char:   l244 = UNIQUE | NON_NULL, (empty)
                // 492: b"-h|--help pri ... st u8: typeof(_151) = *const {l246} u8
                // 492: b"-h|--help pri ... st u8:   l246 = UNIQUE | NON_NULL, (empty)
                // 492: b"-h|--help pri ... \n\0": typeof(_152) = *const {l248} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                // 492: b"-h|--help pri ... \n\0":   l248 = UNIQUE | NON_NULL, (empty)
                // 492: b"-h|--help pri ... \n\0": typeof(_153) = & {l250} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                // 492: b"-h|--help pri ... \n\0":   l250 = UNIQUE | NON_NULL, FIXED
                // 492: b"-h|--help pri ... \n\0": typeof(_152 = &raw const (*_153)) = *const {l3210} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                // 492: b"-h|--help pri ... \n\0":   l3210 = UNIQUE | NON_NULL, (empty)
                // 492: b"-h|--help pri ... _char: typeof(_150 = move _151 as *const i8 (Misc)) = *const {l3212} i8
                // 492: b"-h|--help pri ... _char:   l3212 = UNIQUE | NON_NULL, (empty)
                // 492: b"-h|--help pri ... st u8: typeof(_151 = move _152 as *const u8 (Pointer(ArrayToPointer))) = *const {l3211} u8
                // 492: b"-h|--help pri ... st u8:   l3211 = UNIQUE | NON_NULL, (empty)
                // 492: b"-h|--help pri ... \n\0": typeof(_153 = const b"-h|--help        print command line option summary\n\x00") = & {l3209} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                // 492: b"-h|--help pri ... \n\0":   l3209 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
            printf(
                b"-f|--force       force reading even without header\n\0" as *const u8
                // 496: b"-f|--force fo ... _char: typeof(_155) = *const {l253} i8
                // 496: b"-f|--force fo ... _char:   l253 = UNIQUE | NON_NULL, (empty)
                // 496: b"-f|--force fo ... st u8: typeof(_156) = *const {l255} u8
                // 496: b"-f|--force fo ... st u8:   l255 = UNIQUE | NON_NULL, (empty)
                // 496: b"-f|--force fo ... \n\0": typeof(_157) = *const {l257} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                // 496: b"-f|--force fo ... \n\0":   l257 = UNIQUE | NON_NULL, (empty)
                // 496: b"-f|--force fo ... \n\0": typeof(_158) = & {l259} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                // 496: b"-f|--force fo ... \n\0":   l259 = UNIQUE | NON_NULL, FIXED
                // 496: b"-f|--force fo ... _char: typeof(_155 = move _156 as *const i8 (Misc)) = *const {l3216} i8
                // 496: b"-f|--force fo ... _char:   l3216 = UNIQUE | NON_NULL, (empty)
                // 496: b"-f|--force fo ... st u8: typeof(_156 = move _157 as *const u8 (Pointer(ArrayToPointer))) = *const {l3215} u8
                // 496: b"-f|--force fo ... st u8:   l3215 = UNIQUE | NON_NULL, (empty)
                // 496: b"-f|--force fo ... \n\0": typeof(_158 = const b"-f|--force       force reading even without header\n\x00") = & {l3213} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                // 496: b"-f|--force fo ... \n\0":   l3213 = UNIQUE | NON_NULL, (empty)
                // 496: b"-f|--force fo ... \n\0": typeof(_157 = &raw const (*_158)) = *const {l3214} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                // 496: b"-f|--force fo ... \n\0":   l3214 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
            printf(
                b"-r|--ranges      print value ranges of options\n\0" as *const u8
                // 500: b"-r|--ranges p ... _char: typeof(_160) = *const {l262} i8
                // 500: b"-r|--ranges p ... _char:   l262 = UNIQUE | NON_NULL, (empty)
                // 500: b"-r|--ranges p ... st u8: typeof(_161) = *const {l264} u8
                // 500: b"-r|--ranges p ... st u8:   l264 = UNIQUE | NON_NULL, (empty)
                // 500: b"-r|--ranges p ... \n\0": typeof(_162) = *const {l266} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                // 500: b"-r|--ranges p ... \n\0":   l266 = UNIQUE | NON_NULL, (empty)
                // 500: b"-r|--ranges p ... \n\0": typeof(_163) = & {l268} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                // 500: b"-r|--ranges p ... \n\0":   l268 = UNIQUE | NON_NULL, FIXED
                // 500: b"-r|--ranges p ... \n\0": typeof(_163 = const b"-r|--ranges      print value ranges of options\n\x00") = & {l3217} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                // 500: b"-r|--ranges p ... \n\0":   l3217 = UNIQUE | NON_NULL, (empty)
                // 500: b"-r|--ranges p ... \n\0": typeof(_162 = &raw const (*_163)) = *const {l3218} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                // 500: b"-r|--ranges p ... \n\0":   l3218 = UNIQUE | NON_NULL, (empty)
                // 500: b"-r|--ranges p ... st u8: typeof(_161 = move _162 as *const u8 (Pointer(ArrayToPointer))) = *const {l3219} u8
                // 500: b"-r|--ranges p ... st u8:   l3219 = UNIQUE | NON_NULL, (empty)
                // 500: b"-r|--ranges p ... _char: typeof(_160 = move _161 as *const i8 (Misc)) = *const {l3220} i8
                // 500: b"-r|--ranges p ... _char:   l3220 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
            printf(
                b"-d|--defaults    print default values of options\n\0" as *const u8
                // 504: b"-d|--defaults ... _char: typeof(_165) = *const {l271} i8
                // 504: b"-d|--defaults ... _char:   l271 = UNIQUE | NON_NULL, (empty)
                // 504: b"-d|--defaults ... st u8: typeof(_166) = *const {l273} u8
                // 504: b"-d|--defaults ... st u8:   l273 = UNIQUE | NON_NULL, (empty)
                // 504: b"-d|--defaults ... \n\0": typeof(_167) = *const {l275} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
                // 504: b"-d|--defaults ... \n\0":   l275 = UNIQUE | NON_NULL, (empty)
                // 504: b"-d|--defaults ... \n\0": typeof(_168) = & {l277} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
                // 504: b"-d|--defaults ... \n\0":   l277 = UNIQUE | NON_NULL, FIXED
                // 504: b"-d|--defaults ... \n\0": typeof(_168 = const b"-d|--defaults    print default values of options\n\x00") = & {l3221} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
                // 504: b"-d|--defaults ... \n\0":   l3221 = UNIQUE | NON_NULL, (empty)
                // 504: b"-d|--defaults ... \n\0": typeof(_167 = &raw const (*_168)) = *const {l3222} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
                // 504: b"-d|--defaults ... \n\0":   l3222 = UNIQUE | NON_NULL, (empty)
                // 504: b"-d|--defaults ... st u8: typeof(_166 = move _167 as *const u8 (Pointer(ArrayToPointer))) = *const {l3223} u8
                // 504: b"-d|--defaults ... st u8:   l3223 = UNIQUE | NON_NULL, (empty)
                // 504: b"-d|--defaults ... _char: typeof(_165 = move _166 as *const i8 (Misc)) = *const {l3224} i8
                // 504: b"-d|--defaults ... _char:   l3224 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
            printf(
                b"-P|--pcs         print (full) PCS file\n\0" as *const u8 as *const libc::c_char,
                // 508: b"-P|--pcs prin ... _char: typeof(_170) = *const {l280} i8
                // 508: b"-P|--pcs prin ... _char:   l280 = UNIQUE | NON_NULL, (empty)
                // 508: b"-P|--pcs prin ... st u8: typeof(_171) = *const {l282} u8
                // 508: b"-P|--pcs prin ... st u8:   l282 = UNIQUE | NON_NULL, (empty)
                // 508: b"-P|--pcs prin ... \n\0": typeof(_172) = *const {l284} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 508: b"-P|--pcs prin ... \n\0":   l284 = UNIQUE | NON_NULL, (empty)
                // 508: b"-P|--pcs prin ... \n\0": typeof(_173) = & {l286} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 508: b"-P|--pcs prin ... \n\0":   l286 = UNIQUE | NON_NULL, FIXED
                // 508: b"-P|--pcs prin ... _char: typeof(_170 = move _171 as *const i8 (Misc)) = *const {l3228} i8
                // 508: b"-P|--pcs prin ... _char:   l3228 = UNIQUE | NON_NULL, (empty)
                // 508: b"-P|--pcs prin ... \n\0": typeof(_172 = &raw const (*_173)) = *const {l3226} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 508: b"-P|--pcs prin ... \n\0":   l3226 = UNIQUE | NON_NULL, (empty)
                // 508: b"-P|--pcs prin ... \n\0": typeof(_173 = const b"-P|--pcs         print (full) PCS file\n\x00") = & {l3225} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 508: b"-P|--pcs prin ... \n\0":   l3225 = UNIQUE | NON_NULL, (empty)
                // 508: b"-P|--pcs prin ... st u8: typeof(_171 = move _172 as *const u8 (Pointer(ArrayToPointer))) = *const {l3227} u8
                // 508: b"-P|--pcs prin ... st u8:   l3227 = UNIQUE | NON_NULL, (empty)
            );
            printf(
                b"--pcs-mixed      print mixed PCS file\n\0" as *const u8 as *const libc::c_char,
                // 511: b"--pcs-mixed p ... _char: typeof(_175) = *const {l289} i8
                // 511: b"--pcs-mixed p ... _char:   l289 = UNIQUE | NON_NULL, (empty)
                // 511: b"--pcs-mixed p ... st u8: typeof(_176) = *const {l291} u8
                // 511: b"--pcs-mixed p ... st u8:   l291 = UNIQUE | NON_NULL, (empty)
                // 511: b"--pcs-mixed p ... \n\0": typeof(_177) = *const {l293} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 511: b"--pcs-mixed p ... \n\0":   l293 = UNIQUE | NON_NULL, (empty)
                // 511: b"--pcs-mixed p ... \n\0": typeof(_178) = & {l295} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 511: b"--pcs-mixed p ... \n\0":   l295 = UNIQUE | NON_NULL, FIXED
                // 511: b"--pcs-mixed p ... \n\0": typeof(_177 = &raw const (*_178)) = *const {l3230} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 511: b"--pcs-mixed p ... \n\0":   l3230 = UNIQUE | NON_NULL, (empty)
                // 511: b"--pcs-mixed p ... _char: typeof(_175 = move _176 as *const i8 (Misc)) = *const {l3232} i8
                // 511: b"--pcs-mixed p ... _char:   l3232 = UNIQUE | NON_NULL, (empty)
                // 511: b"--pcs-mixed p ... \n\0": typeof(_178 = const b"--pcs-mixed      print mixed PCS file\n\x00") = & {l3229} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 511: b"--pcs-mixed p ... \n\0":   l3229 = UNIQUE | NON_NULL, (empty)
                // 511: b"--pcs-mixed p ... st u8: typeof(_176 = move _177 as *const u8 (Pointer(ArrayToPointer))) = *const {l3231} u8
                // 511: b"--pcs-mixed p ... st u8:   l3231 = UNIQUE | NON_NULL, (empty)
            );
            printf(
                b"--pcs-reduced    print reduced PCS file\n\0" as *const u8 as *const libc::c_char,
                // 514: b"--pcs-reduced ... _char: typeof(_180) = *const {l298} i8
                // 514: b"--pcs-reduced ... _char:   l298 = UNIQUE | NON_NULL, (empty)
                // 514: b"--pcs-reduced ... st u8: typeof(_181) = *const {l300} u8
                // 514: b"--pcs-reduced ... st u8:   l300 = UNIQUE | NON_NULL, (empty)
                // 514: b"--pcs-reduced ... \n\0": typeof(_182) = *const {l302} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 514: b"--pcs-reduced ... \n\0":   l302 = UNIQUE | NON_NULL, (empty)
                // 514: b"--pcs-reduced ... \n\0": typeof(_183) = & {l304} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 514: b"--pcs-reduced ... \n\0":   l304 = UNIQUE | NON_NULL, FIXED
                // 514: b"--pcs-reduced ... st u8: typeof(_181 = move _182 as *const u8 (Pointer(ArrayToPointer))) = *const {l3235} u8
                // 514: b"--pcs-reduced ... st u8:   l3235 = UNIQUE | NON_NULL, (empty)
                // 514: b"--pcs-reduced ... \n\0": typeof(_182 = &raw const (*_183)) = *const {l3234} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 514: b"--pcs-reduced ... \n\0":   l3234 = UNIQUE | NON_NULL, (empty)
                // 514: b"--pcs-reduced ... _char: typeof(_180 = move _181 as *const i8 (Misc)) = *const {l3236} i8
                // 514: b"--pcs-reduced ... _char:   l3236 = UNIQUE | NON_NULL, (empty)
                // 514: b"--pcs-reduced ... \n\0": typeof(_183 = const b"--pcs-reduced    print reduced PCS file\n\x00") = & {l3233} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 514: b"--pcs-reduced ... \n\0":   l3233 = UNIQUE | NON_NULL, (empty)
            );
            printf(
                b"-e|--embedded    ditto but in an embedded format print\n\0" as *const u8
                // 517: b"-e|--embedded ... _char: typeof(_185) = *const {l307} i8
                // 517: b"-e|--embedded ... _char:   l307 = UNIQUE | NON_NULL, (empty)
                // 517: b"-e|--embedded ... st u8: typeof(_186) = *const {l309} u8
                // 517: b"-e|--embedded ... st u8:   l309 = UNIQUE | NON_NULL, (empty)
                // 517: b"-e|--embedded ... \n\0": typeof(_187) = *const {l311} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000038)) }]
                // 517: b"-e|--embedded ... \n\0":   l311 = UNIQUE | NON_NULL, (empty)
                // 517: b"-e|--embedded ... \n\0": typeof(_188) = & {l313} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000038)) }]
                // 517: b"-e|--embedded ... \n\0":   l313 = UNIQUE | NON_NULL, FIXED
                // 517: b"-e|--embedded ... st u8: typeof(_186 = move _187 as *const u8 (Pointer(ArrayToPointer))) = *const {l3239} u8
                // 517: b"-e|--embedded ... st u8:   l3239 = UNIQUE | NON_NULL, (empty)
                // 517: b"-e|--embedded ... \n\0": typeof(_188 = const b"-e|--embedded    ditto but in an embedded format print\n\x00") = & {l3237} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000038)) }]
                // 517: b"-e|--embedded ... \n\0":   l3237 = UNIQUE | NON_NULL, (empty)
                // 517: b"-e|--embedded ... \n\0": typeof(_187 = &raw const (*_188)) = *const {l3238} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000038)) }]
                // 517: b"-e|--embedded ... \n\0":   l3238 = UNIQUE | NON_NULL, (empty)
                // 517: b"-e|--embedded ... _char: typeof(_185 = move _186 as *const i8 (Misc)) = *const {l3240} i8
                // 517: b"-e|--embedded ... _char:   l3240 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
            printf(
                b"-n|--no-witness   do not print solution (see '--witness')\n\0" as *const u8
                // 521: b"-n|--no-witne ... _char: typeof(_190) = *const {l316} i8
                // 521: b"-n|--no-witne ... _char:   l316 = UNIQUE | NON_NULL, (empty)
                // 521: b"-n|--no-witne ... st u8: typeof(_191) = *const {l318} u8
                // 521: b"-n|--no-witne ... st u8:   l318 = UNIQUE | NON_NULL, (empty)
                // 521: b"-n|--no-witne ... \n\0": typeof(_192) = *const {l320} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003b)) }]
                // 521: b"-n|--no-witne ... \n\0":   l320 = UNIQUE | NON_NULL, (empty)
                // 521: b"-n|--no-witne ... \n\0": typeof(_193) = & {l322} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003b)) }]
                // 521: b"-n|--no-witne ... \n\0":   l322 = UNIQUE | NON_NULL, FIXED
                // 521: b"-n|--no-witne ... \n\0": typeof(_193 = const b"-n|--no-witness   do not print solution (see \'--witness\')\n\x00") = & {l3241} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003b)) }]
                // 521: b"-n|--no-witne ... \n\0":   l3241 = UNIQUE | NON_NULL, (empty)
                // 521: b"-n|--no-witne ... st u8: typeof(_191 = move _192 as *const u8 (Pointer(ArrayToPointer))) = *const {l3243} u8
                // 521: b"-n|--no-witne ... st u8:   l3243 = UNIQUE | NON_NULL, (empty)
                // 521: b"-n|--no-witne ... _char: typeof(_190 = move _191 as *const i8 (Misc)) = *const {l3244} i8
                // 521: b"-n|--no-witne ... _char:   l3244 = UNIQUE | NON_NULL, (empty)
                // 521: b"-n|--no-witne ... \n\0": typeof(_192 = &raw const (*_193)) = *const {l3242} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003b)) }]
                // 521: b"-n|--no-witne ... \n\0":   l3242 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            // 524: b"\n\0" as *con ... _char: typeof(_195) = *const {l325} i8
            // 524: b"\n\0" as *con ... _char:   l325 = UNIQUE | NON_NULL, (empty)
            // 524: b"\n\0" as *const u8: typeof(_196) = *const {l327} u8
            // 524: b"\n\0" as *const u8:   l327 = UNIQUE | NON_NULL, (empty)
            // 524: b"\n\0": typeof(_197) = *const {l329} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 524: b"\n\0":   l329 = UNIQUE | NON_NULL, (empty)
            // 524: b"\n\0": typeof(_198) = & {l331} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 524: b"\n\0":   l331 = UNIQUE | NON_NULL, FIXED
            // 524: b"\n\0" as *const u8: typeof(_196 = move _197 as *const u8 (Pointer(ArrayToPointer))) = *const {l3247} u8
            // 524: b"\n\0" as *const u8:   l3247 = UNIQUE | NON_NULL, (empty)
            // 524: b"\n\0" as *con ... _char: typeof(_195 = move _196 as *const i8 (Misc)) = *const {l3248} i8
            // 524: b"\n\0" as *con ... _char:   l3248 = UNIQUE | NON_NULL, (empty)
            // 524: b"\n\0": typeof(_198 = const b"\n\x00") = & {l3245} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 524: b"\n\0":   l3245 = UNIQUE | NON_NULL, (empty)
            // 524: b"\n\0": typeof(_197 = &raw const (*_198)) = *const {l3246} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 524: b"\n\0":   l3246 = UNIQUE | NON_NULL, (empty)
            printf(
                b"-c               increase checking level\n\0" as *const u8 as *const libc::c_char,
                // 526: b"-c increase c ... _char: typeof(_200) = *const {l334} i8
                // 526: b"-c increase c ... _char:   l334 = UNIQUE | NON_NULL, (empty)
                // 526: b"-c increase c ... st u8: typeof(_201) = *const {l336} u8
                // 526: b"-c increase c ... st u8:   l336 = UNIQUE | NON_NULL, (empty)
                // 526: b"-c increase c ... \n\0": typeof(_202) = *const {l338} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                // 526: b"-c increase c ... \n\0":   l338 = UNIQUE | NON_NULL, (empty)
                // 526: b"-c increase c ... \n\0": typeof(_203) = & {l340} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                // 526: b"-c increase c ... \n\0":   l340 = UNIQUE | NON_NULL, FIXED
                // 526: b"-c increase c ... st u8: typeof(_201 = move _202 as *const u8 (Pointer(ArrayToPointer))) = *const {l3251} u8
                // 526: b"-c increase c ... st u8:   l3251 = UNIQUE | NON_NULL, (empty)
                // 526: b"-c increase c ... _char: typeof(_200 = move _201 as *const i8 (Misc)) = *const {l3252} i8
                // 526: b"-c increase c ... _char:   l3252 = UNIQUE | NON_NULL, (empty)
                // 526: b"-c increase c ... \n\0": typeof(_202 = &raw const (*_203)) = *const {l3250} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                // 526: b"-c increase c ... \n\0":   l3250 = UNIQUE | NON_NULL, (empty)
                // 526: b"-c increase c ... \n\0": typeof(_203 = const b"-c               increase checking level\n\x00") = & {l3249} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                // 526: b"-c increase c ... \n\0":   l3249 = UNIQUE | NON_NULL, (empty)
            );
            printf(
                b"-l               increase logging level\n\0" as *const u8 as *const libc::c_char,
                // 529: b"-l increase l ... _char: typeof(_205) = *const {l343} i8
                // 529: b"-l increase l ... _char:   l343 = UNIQUE | NON_NULL, (empty)
                // 529: b"-l increase l ... st u8: typeof(_206) = *const {l345} u8
                // 529: b"-l increase l ... st u8:   l345 = UNIQUE | NON_NULL, (empty)
                // 529: b"-l increase l ... \n\0": typeof(_207) = *const {l347} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 529: b"-l increase l ... \n\0":   l347 = UNIQUE | NON_NULL, (empty)
                // 529: b"-l increase l ... \n\0": typeof(_208) = & {l349} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 529: b"-l increase l ... \n\0":   l349 = UNIQUE | NON_NULL, FIXED
                // 529: b"-l increase l ... _char: typeof(_205 = move _206 as *const i8 (Misc)) = *const {l3256} i8
                // 529: b"-l increase l ... _char:   l3256 = UNIQUE | NON_NULL, (empty)
                // 529: b"-l increase l ... \n\0": typeof(_207 = &raw const (*_208)) = *const {l3254} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 529: b"-l increase l ... \n\0":   l3254 = UNIQUE | NON_NULL, (empty)
                // 529: b"-l increase l ... st u8: typeof(_206 = move _207 as *const u8 (Pointer(ArrayToPointer))) = *const {l3255} u8
                // 529: b"-l increase l ... st u8:   l3255 = UNIQUE | NON_NULL, (empty)
                // 529: b"-l increase l ... \n\0": typeof(_208 = const b"-l               increase logging level\n\x00") = & {l3253} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 529: b"-l increase l ... \n\0":   l3253 = UNIQUE | NON_NULL, (empty)
            );
            printf(
                b"-v               increase verbose level\n\0" as *const u8 as *const libc::c_char,
                // 532: b"-v increase v ... _char: typeof(_210) = *const {l352} i8
                // 532: b"-v increase v ... _char:   l352 = UNIQUE | NON_NULL, (empty)
                // 532: b"-v increase v ... st u8: typeof(_211) = *const {l354} u8
                // 532: b"-v increase v ... st u8:   l354 = UNIQUE | NON_NULL, (empty)
                // 532: b"-v increase v ... \n\0": typeof(_212) = *const {l356} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 532: b"-v increase v ... \n\0":   l356 = UNIQUE | NON_NULL, (empty)
                // 532: b"-v increase v ... \n\0": typeof(_213) = & {l358} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 532: b"-v increase v ... \n\0":   l358 = UNIQUE | NON_NULL, FIXED
                // 532: b"-v increase v ... _char: typeof(_210 = move _211 as *const i8 (Misc)) = *const {l3260} i8
                // 532: b"-v increase v ... _char:   l3260 = UNIQUE | NON_NULL, (empty)
                // 532: b"-v increase v ... st u8: typeof(_211 = move _212 as *const u8 (Pointer(ArrayToPointer))) = *const {l3259} u8
                // 532: b"-v increase v ... st u8:   l3259 = UNIQUE | NON_NULL, (empty)
                // 532: b"-v increase v ... \n\0": typeof(_212 = &raw const (*_213)) = *const {l3258} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 532: b"-v increase v ... \n\0":   l3258 = UNIQUE | NON_NULL, (empty)
                // 532: b"-v increase v ... \n\0": typeof(_213 = const b"-v               increase verbose level\n\x00") = & {l3257} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                // 532: b"-v increase v ... \n\0":   l3257 = UNIQUE | NON_NULL, (empty)
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            // 534: b"\n\0" as *con ... _char: typeof(_215) = *const {l361} i8
            // 534: b"\n\0" as *con ... _char:   l361 = UNIQUE | NON_NULL, (empty)
            // 534: b"\n\0" as *const u8: typeof(_216) = *const {l363} u8
            // 534: b"\n\0" as *const u8:   l363 = UNIQUE | NON_NULL, (empty)
            // 534: b"\n\0": typeof(_217) = *const {l365} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 534: b"\n\0":   l365 = UNIQUE | NON_NULL, (empty)
            // 534: b"\n\0": typeof(_218) = & {l367} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 534: b"\n\0":   l367 = UNIQUE | NON_NULL, FIXED
            // 534: b"\n\0": typeof(_217 = &raw const (*_218)) = *const {l3262} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 534: b"\n\0":   l3262 = UNIQUE | NON_NULL, (empty)
            // 534: b"\n\0" as *con ... _char: typeof(_215 = move _216 as *const i8 (Misc)) = *const {l3264} i8
            // 534: b"\n\0" as *con ... _char:   l3264 = UNIQUE | NON_NULL, (empty)
            // 534: b"\n\0": typeof(_218 = const b"\n\x00") = & {l3261} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 534: b"\n\0":   l3261 = UNIQUE | NON_NULL, (empty)
            // 534: b"\n\0" as *const u8: typeof(_216 = move _217 as *const u8 (Pointer(ArrayToPointer))) = *const {l3263} u8
            // 534: b"\n\0" as *const u8:   l3263 = UNIQUE | NON_NULL, (empty)
            printf(
                b"--verify         online forward check\n\0" as *const u8 as *const libc::c_char,
                // 536: b"--verify onli ... _char: typeof(_220) = *const {l370} i8
                // 536: b"--verify onli ... _char:   l370 = UNIQUE | NON_NULL, (empty)
                // 536: b"--verify onli ... st u8: typeof(_221) = *const {l372} u8
                // 536: b"--verify onli ... st u8:   l372 = UNIQUE | NON_NULL, (empty)
                // 536: b"--verify onli ... \n\0": typeof(_222) = *const {l374} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 536: b"--verify onli ... \n\0":   l374 = UNIQUE | NON_NULL, (empty)
                // 536: b"--verify onli ... \n\0": typeof(_223) = & {l376} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 536: b"--verify onli ... \n\0":   l376 = UNIQUE | NON_NULL, FIXED
                // 536: b"--verify onli ... \n\0": typeof(_223 = const b"--verify         online forward check\n\x00") = & {l3265} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 536: b"--verify onli ... \n\0":   l3265 = UNIQUE | NON_NULL, (empty)
                // 536: b"--verify onli ... _char: typeof(_220 = move _221 as *const i8 (Misc)) = *const {l3268} i8
                // 536: b"--verify onli ... _char:   l3268 = UNIQUE | NON_NULL, (empty)
                // 536: b"--verify onli ... st u8: typeof(_221 = move _222 as *const u8 (Pointer(ArrayToPointer))) = *const {l3267} u8
                // 536: b"--verify onli ... st u8:   l3267 = UNIQUE | NON_NULL, (empty)
                // 536: b"--verify onli ... \n\0": typeof(_222 = &raw const (*_223)) = *const {l3266} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 536: b"--verify onli ... \n\0":   l3266 = UNIQUE | NON_NULL, (empty)
            );
            printf(b"--proof          generate proof file\n\0" as *const u8 as *const libc::c_char);
            // 538: b"--proof gener ... _char: typeof(_225) = *const {l379} i8
            // 538: b"--proof gener ... _char:   l379 = UNIQUE | NON_NULL, (empty)
            // 538: b"--proof gener ... st u8: typeof(_226) = *const {l381} u8
            // 538: b"--proof gener ... st u8:   l381 = UNIQUE | NON_NULL, (empty)
            // 538: b"--proof gener ... \n\0": typeof(_227) = *const {l383} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
            // 538: b"--proof gener ... \n\0":   l383 = UNIQUE | NON_NULL, (empty)
            // 538: b"--proof gener ... \n\0": typeof(_228) = & {l385} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
            // 538: b"--proof gener ... \n\0":   l385 = UNIQUE | NON_NULL, FIXED
            // 538: b"--proof gener ... st u8: typeof(_226 = move _227 as *const u8 (Pointer(ArrayToPointer))) = *const {l3271} u8
            // 538: b"--proof gener ... st u8:   l3271 = UNIQUE | NON_NULL, (empty)
            // 538: b"--proof gener ... \n\0": typeof(_227 = &raw const (*_228)) = *const {l3270} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
            // 538: b"--proof gener ... \n\0":   l3270 = UNIQUE | NON_NULL, (empty)
            // 538: b"--proof gener ... _char: typeof(_225 = move _226 as *const i8 (Misc)) = *const {l3272} i8
            // 538: b"--proof gener ... _char:   l3272 = UNIQUE | NON_NULL, (empty)
            // 538: b"--proof gener ... \n\0": typeof(_228 = const b"--proof          generate proof file\n\x00") = & {l3269} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
            // 538: b"--proof gener ... \n\0":   l3269 = UNIQUE | NON_NULL, (empty)
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            // 539: b"\n\0" as *con ... _char: typeof(_230) = *const {l388} i8
            // 539: b"\n\0" as *con ... _char:   l388 = UNIQUE | NON_NULL, (empty)
            // 539: b"\n\0" as *const u8: typeof(_231) = *const {l390} u8
            // 539: b"\n\0" as *const u8:   l390 = UNIQUE | NON_NULL, (empty)
            // 539: b"\n\0": typeof(_232) = *const {l392} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 539: b"\n\0":   l392 = UNIQUE | NON_NULL, (empty)
            // 539: b"\n\0": typeof(_233) = & {l394} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 539: b"\n\0":   l394 = UNIQUE | NON_NULL, FIXED
            // 539: b"\n\0": typeof(_233 = const b"\n\x00") = & {l3273} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 539: b"\n\0":   l3273 = UNIQUE | NON_NULL, (empty)
            // 539: b"\n\0" as *con ... _char: typeof(_230 = move _231 as *const i8 (Misc)) = *const {l3276} i8
            // 539: b"\n\0" as *con ... _char:   l3276 = UNIQUE | NON_NULL, (empty)
            // 539: b"\n\0": typeof(_232 = &raw const (*_233)) = *const {l3274} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 539: b"\n\0":   l3274 = UNIQUE | NON_NULL, (empty)
            // 539: b"\n\0" as *const u8: typeof(_231 = move _232 as *const u8 (Pointer(ArrayToPointer))) = *const {l3275} u8
            // 539: b"\n\0" as *const u8:   l3275 = UNIQUE | NON_NULL, (empty)
            printf(
                b"--thanks=<whom>  alternative way of specifying the seed\n\0" as *const u8
                // 541: b"--thanks=<who ... _char: typeof(_235) = *const {l397} i8
                // 541: b"--thanks=<who ... _char:   l397 = UNIQUE | NON_NULL, (empty)
                // 541: b"--thanks=<who ... st u8: typeof(_236) = *const {l399} u8
                // 541: b"--thanks=<who ... st u8:   l399 = UNIQUE | NON_NULL, (empty)
                // 541: b"--thanks=<who ... \n\0": typeof(_237) = *const {l401} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                // 541: b"--thanks=<who ... \n\0":   l401 = UNIQUE | NON_NULL, (empty)
                // 541: b"--thanks=<who ... \n\0": typeof(_238) = & {l403} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                // 541: b"--thanks=<who ... \n\0":   l403 = UNIQUE | NON_NULL, FIXED
                // 541: b"--thanks=<who ... \n\0": typeof(_238 = const b"--thanks=<whom>  alternative way of specifying the seed\n\x00") = & {l3277} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                // 541: b"--thanks=<who ... \n\0":   l3277 = UNIQUE | NON_NULL, (empty)
                // 541: b"--thanks=<who ... st u8: typeof(_236 = move _237 as *const u8 (Pointer(ArrayToPointer))) = *const {l3279} u8
                // 541: b"--thanks=<who ... st u8:   l3279 = UNIQUE | NON_NULL, (empty)
                // 541: b"--thanks=<who ... _char: typeof(_235 = move _236 as *const i8 (Misc)) = *const {l3280} i8
                // 541: b"--thanks=<who ... _char:   l3280 = UNIQUE | NON_NULL, (empty)
                // 541: b"--thanks=<who ... \n\0": typeof(_237 = &raw const (*_238)) = *const {l3278} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
                // 541: b"--thanks=<who ... \n\0":   l3278 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
            printf(
                b"                 (inspired by Vampire)\n\0" as *const u8 as *const libc::c_char,
                // 545: b" (inspired by ... _char: typeof(_240) = *const {l406} i8
                // 545: b" (inspired by ... _char:   l406 = UNIQUE | NON_NULL, (empty)
                // 545: b" (inspired by ... st u8: typeof(_241) = *const {l408} u8
                // 545: b" (inspired by ... st u8:   l408 = UNIQUE | NON_NULL, (empty)
                // 545: b" (inspired by ... \n\0": typeof(_242) = *const {l410} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 545: b" (inspired by ... \n\0":   l410 = UNIQUE | NON_NULL, (empty)
                // 545: b" (inspired by ... \n\0": typeof(_243) = & {l412} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 545: b" (inspired by ... \n\0":   l412 = UNIQUE | NON_NULL, FIXED
                // 545: b" (inspired by ... \n\0": typeof(_243 = const b"                 (inspired by Vampire)\n\x00") = & {l3281} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 545: b" (inspired by ... \n\0":   l3281 = UNIQUE | NON_NULL, (empty)
                // 545: b" (inspired by ... \n\0": typeof(_242 = &raw const (*_243)) = *const {l3282} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 545: b" (inspired by ... \n\0":   l3282 = UNIQUE | NON_NULL, (empty)
                // 545: b" (inspired by ... _char: typeof(_240 = move _241 as *const i8 (Misc)) = *const {l3284} i8
                // 545: b" (inspired by ... _char:   l3284 = UNIQUE | NON_NULL, (empty)
                // 545: b" (inspired by ... st u8: typeof(_241 = move _242 as *const u8 (Pointer(ArrayToPointer))) = *const {l3283} u8
                // 545: b" (inspired by ... st u8:   l3283 = UNIQUE | NON_NULL, (empty)
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            // 547: b"\n\0" as *con ... _char: typeof(_245) = *const {l415} i8
            // 547: b"\n\0" as *con ... _char:   l415 = UNIQUE | NON_NULL, (empty)
            // 547: b"\n\0" as *const u8: typeof(_246) = *const {l417} u8
            // 547: b"\n\0" as *const u8:   l417 = UNIQUE | NON_NULL, (empty)
            // 547: b"\n\0": typeof(_247) = *const {l419} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 547: b"\n\0":   l419 = UNIQUE | NON_NULL, (empty)
            // 547: b"\n\0": typeof(_248) = & {l421} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 547: b"\n\0":   l421 = UNIQUE | NON_NULL, FIXED
            // 547: b"\n\0" as *const u8: typeof(_246 = move _247 as *const u8 (Pointer(ArrayToPointer))) = *const {l3287} u8
            // 547: b"\n\0" as *const u8:   l3287 = UNIQUE | NON_NULL, (empty)
            // 547: b"\n\0" as *con ... _char: typeof(_245 = move _246 as *const i8 (Misc)) = *const {l3288} i8
            // 547: b"\n\0" as *con ... _char:   l3288 = UNIQUE | NON_NULL, (empty)
            // 547: b"\n\0": typeof(_247 = &raw const (*_248)) = *const {l3286} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 547: b"\n\0":   l3286 = UNIQUE | NON_NULL, (empty)
            // 547: b"\n\0": typeof(_248 = const b"\n\x00") = & {l3285} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 547: b"\n\0":   l3285 = UNIQUE | NON_NULL, (empty)
            printf(
                b"The following options can also be used in the form '--<name>=<int>',\njust '--<name>' for increment and '--no-<name>' for zero.  They\ncan be embedded into the CNF file, set through the API or capitalized\nwith prefix 'LGL' instead of '--' through environment variables.\nTheir default values are displayed in square brackets.\n\0"
                // 549: b"The following ... _char: typeof(_250) = *const {l424} i8
                // 549: b"The following ... _char:   l424 = UNIQUE | NON_NULL, (empty)
                // 549: b"The following ... st u8: typeof(_251) = *const {l426} u8
                // 549: b"The following ... st u8:   l426 = UNIQUE | NON_NULL, (empty)
                // 549: b"The following ... \n\0": typeof(_252) = *const {l428} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000144)) }]
                // 549: b"The following ... \n\0":   l428 = UNIQUE | NON_NULL, (empty)
                // 549: b"The following ... \n\0": typeof(_253) = & {l430} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000144)) }]
                // 549: b"The following ... \n\0":   l430 = UNIQUE | NON_NULL, FIXED
                // 549: b"The following ... \n\0": typeof(_253 = const b"The following options can also be used in the form \'--<name>=<int>\',\njust \'--<name>\' for increment and \'--no-<name>\' for zero.  They\ncan be embedded into the CNF file, set through the API or capitalized\nwith prefix \'LGL\' instead of \'--\' through environment variables.\nTheir default values are displayed in square brackets.\n\x00") = & {l3289} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000144)) }]
                // 549: b"The following ... \n\0":   l3289 = UNIQUE | NON_NULL, (empty)
                // 549: b"The following ... _char: typeof(_250 = move _251 as *const i8 (Misc)) = *const {l3292} i8
                // 549: b"The following ... _char:   l3292 = UNIQUE | NON_NULL, (empty)
                // 549: b"The following ... st u8: typeof(_251 = move _252 as *const u8 (Pointer(ArrayToPointer))) = *const {l3291} u8
                // 549: b"The following ... st u8:   l3291 = UNIQUE | NON_NULL, (empty)
                // 549: b"The following ... \n\0": typeof(_252 = &raw const (*_253)) = *const {l3290} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000144)) }]
                // 549: b"The following ... \n\0":   l3290 = UNIQUE | NON_NULL, (empty)
                    as *const u8 as *const libc::c_char,
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            // 552: b"\n\0" as *con ... _char: typeof(_255) = *const {l433} i8
            // 552: b"\n\0" as *con ... _char:   l433 = UNIQUE | NON_NULL, (empty)
            // 552: b"\n\0" as *const u8: typeof(_256) = *const {l435} u8
            // 552: b"\n\0" as *const u8:   l435 = UNIQUE | NON_NULL, (empty)
            // 552: b"\n\0": typeof(_257) = *const {l437} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 552: b"\n\0":   l437 = UNIQUE | NON_NULL, (empty)
            // 552: b"\n\0": typeof(_258) = & {l439} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 552: b"\n\0":   l439 = UNIQUE | NON_NULL, FIXED
            // 552: b"\n\0": typeof(_257 = &raw const (*_258)) = *const {l3294} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 552: b"\n\0":   l3294 = UNIQUE | NON_NULL, (empty)
            // 552: b"\n\0" as *const u8: typeof(_256 = move _257 as *const u8 (Pointer(ArrayToPointer))) = *const {l3295} u8
            // 552: b"\n\0" as *const u8:   l3295 = UNIQUE | NON_NULL, (empty)
            // 552: b"\n\0" as *con ... _char: typeof(_255 = move _256 as *const i8 (Misc)) = *const {l3296} i8
            // 552: b"\n\0" as *con ... _char:   l3296 = UNIQUE | NON_NULL, (empty)
            // 552: b"\n\0": typeof(_258 = const b"\n\x00") = & {l3293} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 552: b"\n\0":   l3293 = UNIQUE | NON_NULL, (empty)
            printf(
                b"The input <file> can be compressed.  This is detected by matching\nthe <suffix> of the filename against 'gz', 'bz2, 'xz', 'zip', '7z'.\nHowever uncompressing a file is implemented by starting an external\nprocess running corresponding helper programs, e.g., 'gzip', 'bzip2'.\nThus those have to be installed and in the current path if needed.\n\0"
                // 554: b"The input <fi ... _char: typeof(_260) = *const {l442} i8
                // 554: b"The input <fi ... _char:   l442 = UNIQUE | NON_NULL, (empty)
                // 554: b"The input <fi ... st u8: typeof(_261) = *const {l444} u8
                // 554: b"The input <fi ... st u8:   l444 = UNIQUE | NON_NULL, (empty)
                // 554: b"The input <fi ... \n\0": typeof(_262) = *const {l446} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000154)) }]
                // 554: b"The input <fi ... \n\0":   l446 = UNIQUE | NON_NULL, (empty)
                // 554: b"The input <fi ... \n\0": typeof(_263) = & {l448} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000154)) }]
                // 554: b"The input <fi ... \n\0":   l448 = UNIQUE | NON_NULL, FIXED
                // 554: b"The input <fi ... \n\0": typeof(_262 = &raw const (*_263)) = *const {l3298} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000154)) }]
                // 554: b"The input <fi ... \n\0":   l3298 = UNIQUE | NON_NULL, (empty)
                // 554: b"The input <fi ... st u8: typeof(_261 = move _262 as *const u8 (Pointer(ArrayToPointer))) = *const {l3299} u8
                // 554: b"The input <fi ... st u8:   l3299 = UNIQUE | NON_NULL, (empty)
                // 554: b"The input <fi ... _char: typeof(_260 = move _261 as *const i8 (Misc)) = *const {l3300} i8
                // 554: b"The input <fi ... _char:   l3300 = UNIQUE | NON_NULL, (empty)
                // 554: b"The input <fi ... \n\0": typeof(_263 = const b"The input <file> can be compressed.  This is detected by matching\nthe <suffix> of the filename against \'gz\', \'bz2, \'xz\', \'zip\', \'7z\'.\nHowever uncompressing a file is implemented by starting an external\nprocess running corresponding helper programs, e.g., \'gzip\', \'bzip2\'.\nThus those have to be installed and in the current path if needed.\n\x00") = & {l3297} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000154)) }]
                // 554: b"The input <fi ... \n\0":   l3297 = UNIQUE | NON_NULL, (empty)
                    as *const u8 as *const libc::c_char,
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            // 557: b"\n\0" as *con ... _char: typeof(_265) = *const {l451} i8
            // 557: b"\n\0" as *con ... _char:   l451 = UNIQUE | NON_NULL, (empty)
            // 557: b"\n\0" as *const u8: typeof(_266) = *const {l453} u8
            // 557: b"\n\0" as *const u8:   l453 = UNIQUE | NON_NULL, (empty)
            // 557: b"\n\0": typeof(_267) = *const {l455} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 557: b"\n\0":   l455 = UNIQUE | NON_NULL, (empty)
            // 557: b"\n\0": typeof(_268) = & {l457} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 557: b"\n\0":   l457 = UNIQUE | NON_NULL, FIXED
            // 557: b"\n\0" as *con ... _char: typeof(_265 = move _266 as *const i8 (Misc)) = *const {l3304} i8
            // 557: b"\n\0" as *con ... _char:   l3304 = UNIQUE | NON_NULL, (empty)
            // 557: b"\n\0": typeof(_267 = &raw const (*_268)) = *const {l3302} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 557: b"\n\0":   l3302 = UNIQUE | NON_NULL, (empty)
            // 557: b"\n\0": typeof(_268 = const b"\n\x00") = & {l3301} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 557: b"\n\0":   l3301 = UNIQUE | NON_NULL, (empty)
            // 557: b"\n\0" as *const u8: typeof(_266 = move _267 as *const u8 (Pointer(ArrayToPointer))) = *const {l3303} u8
            // 557: b"\n\0" as *const u8:   l3303 = UNIQUE | NON_NULL, (empty)
            lglusage(lgl);
            // 558: lgl: typeof(_270) = *mut {l460} LGL
            // 558: lgl:   l460 = UNIQUE | NON_NULL, (empty)
            current_block = 14603147171032977705;
            break;
        } else if strcmp(
            *argv.offset(i as isize),
            // 562: *argv.offset(i  ... size): typeof(_273) = *const {l464} i8
            // 562: *argv.offset(i  ... size):   l464 = UNIQUE | NON_NULL, (empty)
            // 562: *argv.offset(i  ... size): typeof(_274) = *mut {l466} i8
            // 562: *argv.offset(i  ... size):   l466 = UNIQUE | NON_NULL, (empty)
            // 562: argv.offset(i a ... size): typeof(_275) = *mut {l468} *mut {l469} i8
            // 562: argv.offset(i a ... size):   l468 = UNIQUE | NON_NULL, (empty)
            // 562: argv.offset(i a ... size):   l469 = UNIQUE | NON_NULL, (empty)
            // 562: argv: typeof(_276) = *mut {l471} *mut {l472} i8
            // 562: argv:   l471 = UNIQUE | NON_NULL, (empty)
            // 562: argv:   l472 = UNIQUE | NON_NULL, (empty)
            // 562: *argv.offset(i  ... size): typeof(_273 = move _274 as *const i8 (Pointer(MutToConstPointer))) = *const {l3305} i8
            // 562: *argv.offset(i  ... size):   l3305 = UNIQUE | NON_NULL, (empty)
            b"--version\0" as *const u8 as *const libc::c_char,
            // 563: b"--version\0"  ... _char: typeof(_279) = *const {l476} i8
            // 563: b"--version\0"  ... _char:   l476 = UNIQUE | NON_NULL, (empty)
            // 563: b"--version\0"  ... st u8: typeof(_280) = *const {l478} u8
            // 563: b"--version\0"  ... st u8:   l478 = UNIQUE | NON_NULL, (empty)
            // 563: b"--version\0": typeof(_281) = *const {l480} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 563: b"--version\0":   l480 = UNIQUE | NON_NULL, (empty)
            // 563: b"--version\0": typeof(_282) = & {l482} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 563: b"--version\0":   l482 = UNIQUE | NON_NULL, FIXED
            // 563: b"--version\0": typeof(_281 = &raw const (*_282)) = *const {l3307} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 563: b"--version\0":   l3307 = UNIQUE | NON_NULL, (empty)
            // 563: b"--version\0"  ... _char: typeof(_279 = move _280 as *const i8 (Misc)) = *const {l3309} i8
            // 563: b"--version\0"  ... _char:   l3309 = UNIQUE | NON_NULL, (empty)
            // 563: b"--version\0"  ... st u8: typeof(_280 = move _281 as *const u8 (Pointer(ArrayToPointer))) = *const {l3308} u8
            // 563: b"--version\0"  ... st u8:   l3308 = UNIQUE | NON_NULL, (empty)
            // 563: b"--version\0": typeof(_282 = const b"--version\x00") = & {l3306} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 563: b"--version\0":   l3306 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            printf(b"%s\n\0" as *const u8 as *const libc::c_char, lglversion());
            // 566: b"%s\n\0" as *c ... _char: typeof(_285) = *const {l486} i8
            // 566: b"%s\n\0" as *c ... _char:   l486 = UNIQUE | NON_NULL, (empty)
            // 566: b"%s\n\0" as *c ... st u8: typeof(_286) = *const {l488} u8
            // 566: b"%s\n\0" as *c ... st u8:   l488 = UNIQUE | NON_NULL, (empty)
            // 566: b"%s\n\0": typeof(_287) = *const {l490} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 566: b"%s\n\0":   l490 = UNIQUE | NON_NULL, (empty)
            // 566: b"%s\n\0": typeof(_288) = & {l492} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 566: b"%s\n\0":   l492 = UNIQUE | NON_NULL, FIXED
            // 566: lglversion(): typeof(_289) = *const {l494} i8
            // 566: lglversion():   l494 = UNIQUE | NON_NULL, (empty)
            // 566: b"%s\n\0" as *c ... st u8: typeof(_286 = move _287 as *const u8 (Pointer(ArrayToPointer))) = *const {l3312} u8
            // 566: b"%s\n\0" as *c ... st u8:   l3312 = UNIQUE | NON_NULL, (empty)
            // 566: b"%s\n\0" as *c ... _char: typeof(_285 = move _286 as *const i8 (Misc)) = *const {l3313} i8
            // 566: b"%s\n\0" as *c ... _char:   l3313 = UNIQUE | NON_NULL, (empty)
            // 566: b"%s\n\0": typeof(_288 = const b"%s\n\x00") = & {l3310} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 566: b"%s\n\0":   l3310 = UNIQUE | NON_NULL, (empty)
            // 566: b"%s\n\0": typeof(_287 = &raw const (*_288)) = *const {l3311} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 566: b"%s\n\0":   l3311 = UNIQUE | NON_NULL, (empty)
            fflush(stdout);
            // 567: stdout: typeof(_291) = *mut {l497} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
            // 567: stdout:   l497 = UNIQUE | NON_NULL, (empty)
            // 567: stdout: typeof(_292) = *mut {l499} *mut {l500} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
            // 567: stdout:   l499 = UNIQUE | NON_NULL, (empty)
            // 567: stdout:   l500 = UNIQUE | NON_NULL, (empty)
            current_block = 14603147171032977705;
            break;
        } else {
            if strcmp(
                *argv.offset(i as isize),
                // 572: *argv.offset(i  ... size): typeof(_296) = *const {l505} i8
                // 572: *argv.offset(i  ... size):   l505 = UNIQUE | NON_NULL, (empty)
                // 572: *argv.offset(i  ... size): typeof(_297) = *mut {l507} i8
                // 572: *argv.offset(i  ... size):   l507 = UNIQUE | NON_NULL, (empty)
                // 572: argv.offset(i a ... size): typeof(_298) = *mut {l509} *mut {l510} i8
                // 572: argv.offset(i a ... size):   l509 = UNIQUE | NON_NULL, (empty)
                // 572: argv.offset(i a ... size):   l510 = UNIQUE | NON_NULL, (empty)
                // 572: argv: typeof(_299) = *mut {l512} *mut {l513} i8
                // 572: argv:   l512 = UNIQUE | NON_NULL, (empty)
                // 572: argv:   l513 = UNIQUE | NON_NULL, (empty)
                // 572: *argv.offset(i  ... size): typeof(_296 = move _297 as *const i8 (Pointer(MutToConstPointer))) = *const {l3314} i8
                // 572: *argv.offset(i  ... size):   l3314 = UNIQUE | NON_NULL, (empty)
                b"-s\0" as *const u8 as *const libc::c_char,
                // 573: b"-s\0" as *con ... _char: typeof(_302) = *const {l517} i8
                // 573: b"-s\0" as *con ... _char:   l517 = UNIQUE | NON_NULL, (empty)
                // 573: b"-s\0" as *const u8: typeof(_303) = *const {l519} u8
                // 573: b"-s\0" as *const u8:   l519 = UNIQUE | NON_NULL, (empty)
                // 573: b"-s\0": typeof(_304) = *const {l521} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 573: b"-s\0":   l521 = UNIQUE | NON_NULL, (empty)
                // 573: b"-s\0": typeof(_305) = & {l523} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 573: b"-s\0":   l523 = UNIQUE | NON_NULL, FIXED
                // 573: b"-s\0": typeof(_304 = &raw const (*_305)) = *const {l3316} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 573: b"-s\0":   l3316 = UNIQUE | NON_NULL, (empty)
                // 573: b"-s\0": typeof(_305 = const b"-s\x00") = & {l3315} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 573: b"-s\0":   l3315 = UNIQUE | NON_NULL, (empty)
                // 573: b"-s\0" as *const u8: typeof(_303 = move _304 as *const u8 (Pointer(ArrayToPointer))) = *const {l3317} u8
                // 573: b"-s\0" as *const u8:   l3317 = UNIQUE | NON_NULL, (empty)
                // 573: b"-s\0" as *con ... _char: typeof(_302 = move _303 as *const i8 (Misc)) = *const {l3318} i8
                // 573: b"-s\0" as *con ... _char:   l3318 = UNIQUE | NON_NULL, (empty)
            ) == 0
            {
                simponly = 1 as libc::c_int;
            } else if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            // 577: (*argv.offset(i ... size): typeof(_311) = *mut {l530} i8
            // 577: (*argv.offset(i ... size):   l530 = UNIQUE | NON_NULL, (empty)
            // 577: (*argv.offset(i ... ize)): typeof(_312) = *mut {l532} i8
            // 577: (*argv.offset(i ... ize)):   l532 = UNIQUE | NON_NULL, (empty)
            // 577: argv.offset(i a ... size): typeof(_313) = *mut {l534} *mut {l535} i8
            // 577: argv.offset(i a ... size):   l534 = UNIQUE | NON_NULL, (empty)
            // 577: argv.offset(i a ... size):   l535 = UNIQUE | NON_NULL, (empty)
            // 577: argv: typeof(_314) = *mut {l537} *mut {l538} i8
            // 577: argv:   l537 = UNIQUE | NON_NULL, (empty)
            // 577: argv:   l538 = UNIQUE | NON_NULL, (empty)
                == '-' as i32
                && *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize) as libc::c_int
                // 579: (*argv.offset(i ... size): typeof(_323) = *mut {l548} i8
                // 579: (*argv.offset(i ... size):   l548 = UNIQUE | NON_NULL, (empty)
                // 579: (*argv.offset(i ... ize)): typeof(_324) = *mut {l550} i8
                // 579: (*argv.offset(i ... ize)):   l550 = UNIQUE | NON_NULL, (empty)
                // 579: argv.offset(i a ... size): typeof(_325) = *mut {l552} *mut {l553} i8
                // 579: argv.offset(i a ... size):   l552 = UNIQUE | NON_NULL, (empty)
                // 579: argv.offset(i a ... size):   l553 = UNIQUE | NON_NULL, (empty)
                // 579: argv: typeof(_326) = *mut {l555} *mut {l556} i8
                // 579: argv:   l555 = UNIQUE | NON_NULL, (empty)
                // 579: argv:   l556 = UNIQUE | NON_NULL, (empty)
                    == 'O' as i32
            {
                if simplevel > 0 as libc::c_int {
                    fprintf(
                        stderr,
                        // 584: stderr: typeof(_337) = *mut {l568} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 584: stderr:   l568 = UNIQUE | NON_NULL, (empty)
                        // 584: stderr: typeof(_338) = *mut {l570} *mut {l571} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 584: stderr:   l570 = UNIQUE | NON_NULL, (empty)
                        // 584: stderr:   l571 = UNIQUE | NON_NULL, (empty)
                        b"*** lingeling error: multiple '-O..' options\n\0" as *const u8
                        // 585: b"*** lingeling ... _char: typeof(_339) = *const {l573} i8
                        // 585: b"*** lingeling ... _char:   l573 = UNIQUE | NON_NULL, (empty)
                        // 585: b"*** lingeling ... st u8: typeof(_340) = *const {l575} u8
                        // 585: b"*** lingeling ... st u8:   l575 = UNIQUE | NON_NULL, (empty)
                        // 585: b"*** lingeling ... \n\0": typeof(_341) = *const {l577} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                        // 585: b"*** lingeling ... \n\0":   l577 = UNIQUE | NON_NULL, (empty)
                        // 585: b"*** lingeling ... \n\0": typeof(_342) = & {l579} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                        // 585: b"*** lingeling ... \n\0":   l579 = UNIQUE | NON_NULL, FIXED
                        // 585: b"*** lingeling ... _char: typeof(_339 = move _340 as *const i8 (Misc)) = *const {l3322} i8
                        // 585: b"*** lingeling ... _char:   l3322 = UNIQUE | NON_NULL, (empty)
                        // 585: b"*** lingeling ... \n\0": typeof(_342 = const b"*** lingeling error: multiple \'-O..\' options\n\x00") = & {l3319} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                        // 585: b"*** lingeling ... \n\0":   l3319 = UNIQUE | NON_NULL, (empty)
                        // 585: b"*** lingeling ... st u8: typeof(_340 = move _341 as *const u8 (Pointer(ArrayToPointer))) = *const {l3321} u8
                        // 585: b"*** lingeling ... st u8:   l3321 = UNIQUE | NON_NULL, (empty)
                        // 585: b"*** lingeling ... \n\0": typeof(_341 = &raw const (*_342)) = *const {l3320} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                        // 585: b"*** lingeling ... \n\0":   l3320 = UNIQUE | NON_NULL, (empty)
                            as *const libc::c_char,
                    );
                    res = 1 as libc::c_int;
                    current_block = 14603147171032977705;
                    break;
                } else {
                    simplevel = atoi((*argv.offset(i as isize)).offset(2 as libc::c_int as isize));
                    // 592: (*argv.offset(i ... size): typeof(_345) = *const {l583} i8
                    // 592: (*argv.offset(i ... size):   l583 = UNIQUE | NON_NULL, (empty)
                    // 592: (*argv.offset(i ... size): typeof(_346) = *mut {l585} i8
                    // 592: (*argv.offset(i ... size):   l585 = UNIQUE | NON_NULL, (empty)
                    // 592: (*argv.offset(i ... ize)): typeof(_347) = *mut {l587} i8
                    // 592: (*argv.offset(i ... ize)):   l587 = UNIQUE | NON_NULL, (empty)
                    // 592: argv.offset(i a ... size): typeof(_348) = *mut {l589} *mut {l590} i8
                    // 592: argv.offset(i a ... size):   l589 = UNIQUE | NON_NULL, (empty)
                    // 592: argv.offset(i a ... size):   l590 = UNIQUE | NON_NULL, (empty)
                    // 592: argv: typeof(_349) = *mut {l592} *mut {l593} i8
                    // 592: argv:   l592 = UNIQUE | NON_NULL, (empty)
                    // 592: argv:   l593 = UNIQUE | NON_NULL, (empty)
                    // 592: (*argv.offset(i ... size): typeof(_345 = move _346 as *const i8 (Pointer(MutToConstPointer))) = *const {l3323} i8
                    // 592: (*argv.offset(i ... size):   l3323 = UNIQUE | NON_NULL, (empty)
                    if simplevel <= 0 as libc::c_int {
                        fprintf(
                            stderr,
                            // 595: stderr: typeof(_359) = *mut {l604} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                            // 595: stderr:   l604 = UNIQUE | NON_NULL, (empty)
                            // 595: stderr: typeof(_360) = *mut {l606} *mut {l607} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                            // 595: stderr:   l606 = UNIQUE | NON_NULL, (empty)
                            // 595: stderr:   l607 = UNIQUE | NON_NULL, (empty)
                            b"*** lingeling error: invalid '%s' option\n\0" as *const u8
                            // 596: b"*** lingeling ... _char: typeof(_361) = *const {l609} i8
                            // 596: b"*** lingeling ... _char:   l609 = UNIQUE | NON_NULL, (empty)
                            // 596: b"*** lingeling ... st u8: typeof(_362) = *const {l611} u8
                            // 596: b"*** lingeling ... st u8:   l611 = UNIQUE | NON_NULL, (empty)
                            // 596: b"*** lingeling ... \n\0": typeof(_363) = *const {l613} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                            // 596: b"*** lingeling ... \n\0":   l613 = UNIQUE | NON_NULL, (empty)
                            // 596: b"*** lingeling ... \n\0": typeof(_364) = & {l615} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                            // 596: b"*** lingeling ... \n\0":   l615 = UNIQUE | NON_NULL, FIXED
                            // 596: b"*** lingeling ... \n\0": typeof(_363 = &raw const (*_364)) = *const {l3325} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                            // 596: b"*** lingeling ... \n\0":   l3325 = UNIQUE | NON_NULL, (empty)
                            // 596: b"*** lingeling ... st u8: typeof(_362 = move _363 as *const u8 (Pointer(ArrayToPointer))) = *const {l3326} u8
                            // 596: b"*** lingeling ... st u8:   l3326 = UNIQUE | NON_NULL, (empty)
                            // 596: b"*** lingeling ... _char: typeof(_361 = move _362 as *const i8 (Misc)) = *const {l3327} i8
                            // 596: b"*** lingeling ... _char:   l3327 = UNIQUE | NON_NULL, (empty)
                            // 596: b"*** lingeling ... \n\0": typeof(_364 = const b"*** lingeling error: invalid \'%s\' option\n\x00") = & {l3324} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                            // 596: b"*** lingeling ... \n\0":   l3324 = UNIQUE | NON_NULL, (empty)
                                as *const libc::c_char,
                            *argv.offset(i as isize),
                            // 598: *argv.offset(i  ... size): typeof(_365) = *mut {l617} i8
                            // 598: *argv.offset(i  ... size):   l617 = UNIQUE | NON_NULL, (empty)
                            // 598: argv.offset(i a ... size): typeof(_366) = *mut {l619} *mut {l620} i8
                            // 598: argv.offset(i a ... size):   l619 = UNIQUE | NON_NULL, (empty)
                            // 598: argv.offset(i a ... size):   l620 = UNIQUE | NON_NULL, (empty)
                            // 598: argv: typeof(_367) = *mut {l622} *mut {l623} i8
                            // 598: argv:   l622 = UNIQUE | NON_NULL, (empty)
                            // 598: argv:   l623 = UNIQUE | NON_NULL, (empty)
                        );
                        res = 1 as libc::c_int;
                        current_block = 14603147171032977705;
                        break;
                    }
                }
            } else if strcmp(
                *argv.offset(i as isize),
                // 606: *argv.offset(i  ... size): typeof(_373) = *const {l630} i8
                // 606: *argv.offset(i  ... size):   l630 = UNIQUE | NON_NULL, (empty)
                // 606: *argv.offset(i  ... size): typeof(_374) = *mut {l632} i8
                // 606: *argv.offset(i  ... size):   l632 = UNIQUE | NON_NULL, (empty)
                // 606: argv.offset(i a ... size): typeof(_375) = *mut {l634} *mut {l635} i8
                // 606: argv.offset(i a ... size):   l634 = UNIQUE | NON_NULL, (empty)
                // 606: argv.offset(i a ... size):   l635 = UNIQUE | NON_NULL, (empty)
                // 606: argv: typeof(_376) = *mut {l637} *mut {l638} i8
                // 606: argv:   l637 = UNIQUE | NON_NULL, (empty)
                // 606: argv:   l638 = UNIQUE | NON_NULL, (empty)
                // 606: *argv.offset(i  ... size): typeof(_373 = move _374 as *const i8 (Pointer(MutToConstPointer))) = *const {l3328} i8
                // 606: *argv.offset(i  ... size):   l3328 = UNIQUE | NON_NULL, (empty)
                b"-q\0" as *const u8 as *const libc::c_char,
                // 607: b"-q\0" as *con ... _char: typeof(_379) = *const {l642} i8
                // 607: b"-q\0" as *con ... _char:   l642 = UNIQUE | NON_NULL, (empty)
                // 607: b"-q\0" as *const u8: typeof(_380) = *const {l644} u8
                // 607: b"-q\0" as *const u8:   l644 = UNIQUE | NON_NULL, (empty)
                // 607: b"-q\0": typeof(_381) = *const {l646} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 607: b"-q\0":   l646 = UNIQUE | NON_NULL, (empty)
                // 607: b"-q\0": typeof(_382) = & {l648} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 607: b"-q\0":   l648 = UNIQUE | NON_NULL, FIXED
                // 607: b"-q\0": typeof(_381 = &raw const (*_382)) = *const {l3330} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 607: b"-q\0":   l3330 = UNIQUE | NON_NULL, (empty)
                // 607: b"-q\0" as *con ... _char: typeof(_379 = move _380 as *const i8 (Misc)) = *const {l3332} i8
                // 607: b"-q\0" as *con ... _char:   l3332 = UNIQUE | NON_NULL, (empty)
                // 607: b"-q\0" as *const u8: typeof(_380 = move _381 as *const u8 (Pointer(ArrayToPointer))) = *const {l3331} u8
                // 607: b"-q\0" as *const u8:   l3331 = UNIQUE | NON_NULL, (empty)
                // 607: b"-q\0": typeof(_382 = const b"-q\x00") = & {l3329} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 607: b"-q\0":   l3329 = UNIQUE | NON_NULL, (empty)
            ) == 0
            {
                lglsetopt(
                    lgl,
                    // 611: lgl: typeof(_384) = *mut {l651} LGL
                    // 611: lgl:   l651 = UNIQUE | NON_NULL, (empty)
                    b"verbose\0" as *const u8 as *const libc::c_char,
                    // 612: b"verbose\0" as ... _char: typeof(_385) = *const {l653} i8
                    // 612: b"verbose\0" as ... _char:   l653 = UNIQUE | NON_NULL, (empty)
                    // 612: b"verbose\0" as ... st u8: typeof(_386) = *const {l655} u8
                    // 612: b"verbose\0" as ... st u8:   l655 = UNIQUE | NON_NULL, (empty)
                    // 612: b"verbose\0": typeof(_387) = *const {l657} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 612: b"verbose\0":   l657 = UNIQUE | NON_NULL, (empty)
                    // 612: b"verbose\0": typeof(_388) = & {l659} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 612: b"verbose\0":   l659 = UNIQUE | NON_NULL, FIXED
                    // 612: b"verbose\0" as ... _char: typeof(_385 = move _386 as *const i8 (Misc)) = *const {l3336} i8
                    // 612: b"verbose\0" as ... _char:   l3336 = UNIQUE | NON_NULL, (empty)
                    // 612: b"verbose\0": typeof(_388 = const b"verbose\x00") = & {l3333} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 612: b"verbose\0":   l3333 = UNIQUE | NON_NULL, (empty)
                    // 612: b"verbose\0" as ... st u8: typeof(_386 = move _387 as *const u8 (Pointer(ArrayToPointer))) = *const {l3335} u8
                    // 612: b"verbose\0" as ... st u8:   l3335 = UNIQUE | NON_NULL, (empty)
                    // 612: b"verbose\0": typeof(_387 = &raw const (*_388)) = *const {l3334} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 612: b"verbose\0":   l3334 = UNIQUE | NON_NULL, (empty)
                    -(1 as libc::c_int),
                );
            } else if strcmp(
                *argv.offset(i as isize),
                // 616: *argv.offset(i  ... size): typeof(_394) = *const {l666} i8
                // 616: *argv.offset(i  ... size):   l666 = UNIQUE | NON_NULL, (empty)
                // 616: *argv.offset(i  ... size): typeof(_395) = *mut {l668} i8
                // 616: *argv.offset(i  ... size):   l668 = UNIQUE | NON_NULL, (empty)
                // 616: argv.offset(i a ... size): typeof(_396) = *mut {l670} *mut {l671} i8
                // 616: argv.offset(i a ... size):   l670 = UNIQUE | NON_NULL, (empty)
                // 616: argv.offset(i a ... size):   l671 = UNIQUE | NON_NULL, (empty)
                // 616: argv: typeof(_397) = *mut {l673} *mut {l674} i8
                // 616: argv:   l673 = UNIQUE | NON_NULL, (empty)
                // 616: argv:   l674 = UNIQUE | NON_NULL, (empty)
                // 616: *argv.offset(i  ... size): typeof(_394 = move _395 as *const i8 (Pointer(MutToConstPointer))) = *const {l3337} i8
                // 616: *argv.offset(i  ... size):   l3337 = UNIQUE | NON_NULL, (empty)
                b"-o\0" as *const u8 as *const libc::c_char,
                // 617: b"-o\0" as *con ... _char: typeof(_400) = *const {l678} i8
                // 617: b"-o\0" as *con ... _char:   l678 = UNIQUE | NON_NULL, (empty)
                // 617: b"-o\0" as *const u8: typeof(_401) = *const {l680} u8
                // 617: b"-o\0" as *const u8:   l680 = UNIQUE | NON_NULL, (empty)
                // 617: b"-o\0": typeof(_402) = *const {l682} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 617: b"-o\0":   l682 = UNIQUE | NON_NULL, (empty)
                // 617: b"-o\0": typeof(_403) = & {l684} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 617: b"-o\0":   l684 = UNIQUE | NON_NULL, FIXED
                // 617: b"-o\0": typeof(_402 = &raw const (*_403)) = *const {l3339} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 617: b"-o\0":   l3339 = UNIQUE | NON_NULL, (empty)
                // 617: b"-o\0" as *con ... _char: typeof(_400 = move _401 as *const i8 (Misc)) = *const {l3341} i8
                // 617: b"-o\0" as *con ... _char:   l3341 = UNIQUE | NON_NULL, (empty)
                // 617: b"-o\0" as *const u8: typeof(_401 = move _402 as *const u8 (Pointer(ArrayToPointer))) = *const {l3340} u8
                // 617: b"-o\0" as *const u8:   l3340 = UNIQUE | NON_NULL, (empty)
                // 617: b"-o\0": typeof(_403 = const b"-o\x00") = & {l3338} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 617: b"-o\0":   l3338 = UNIQUE | NON_NULL, (empty)
            ) == 0
            {
                i += 1;
                if i == argc {
                    fprintf(
                        stderr,
                        // 623: stderr: typeof(_410) = *mut {l692} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 623: stderr:   l692 = UNIQUE | NON_NULL, (empty)
                        // 623: stderr: typeof(_411) = *mut {l694} *mut {l695} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 623: stderr:   l694 = UNIQUE | NON_NULL, (empty)
                        // 623: stderr:   l695 = UNIQUE | NON_NULL, (empty)
                        b"*** lingeling error: argument to '-o' missing\n\0" as *const u8
                        // 624: b"*** lingeling ... _char: typeof(_412) = *const {l697} i8
                        // 624: b"*** lingeling ... _char:   l697 = UNIQUE | NON_NULL, (empty)
                        // 624: b"*** lingeling ... st u8: typeof(_413) = *const {l699} u8
                        // 624: b"*** lingeling ... st u8:   l699 = UNIQUE | NON_NULL, (empty)
                        // 624: b"*** lingeling ... \n\0": typeof(_414) = *const {l701} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                        // 624: b"*** lingeling ... \n\0":   l701 = UNIQUE | NON_NULL, (empty)
                        // 624: b"*** lingeling ... \n\0": typeof(_415) = & {l703} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                        // 624: b"*** lingeling ... \n\0":   l703 = UNIQUE | NON_NULL, FIXED
                        // 624: b"*** lingeling ... \n\0": typeof(_414 = &raw const (*_415)) = *const {l3343} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                        // 624: b"*** lingeling ... \n\0":   l3343 = UNIQUE | NON_NULL, (empty)
                        // 624: b"*** lingeling ... \n\0": typeof(_415 = const b"*** lingeling error: argument to \'-o\' missing\n\x00") = & {l3342} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                        // 624: b"*** lingeling ... \n\0":   l3342 = UNIQUE | NON_NULL, (empty)
                        // 624: b"*** lingeling ... st u8: typeof(_413 = move _414 as *const u8 (Pointer(ArrayToPointer))) = *const {l3344} u8
                        // 624: b"*** lingeling ... st u8:   l3344 = UNIQUE | NON_NULL, (empty)
                        // 624: b"*** lingeling ... _char: typeof(_412 = move _413 as *const i8 (Misc)) = *const {l3345} i8
                        // 624: b"*** lingeling ... _char:   l3345 = UNIQUE | NON_NULL, (empty)
                            as *const libc::c_char,
                    );
                    res = 1 as libc::c_int;
                    current_block = 14603147171032977705;
                    break;
                } else if !oname.is_null() {
                // 630: oname: typeof(_419) = *const {l708} i8
                // 630: oname:   l708 = UNIQUE | NON_NULL, (empty)
                    fprintf(
                        stderr,
                        // 632: stderr: typeof(_422) = *mut {l712} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 632: stderr:   l712 = UNIQUE | NON_NULL, (empty)
                        // 632: stderr: typeof(_423) = *mut {l714} *mut {l715} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 632: stderr:   l714 = UNIQUE | NON_NULL, (empty)
                        // 632: stderr:   l715 = UNIQUE | NON_NULL, (empty)
                        b"*** lingeling error: multiple output files '%s' and '%s'\n\0" as *const u8
                        // 633: b"*** lingeling ... _char: typeof(_424) = *const {l717} i8
                        // 633: b"*** lingeling ... _char:   l717 = UNIQUE | NON_NULL, (empty)
                        // 633: b"*** lingeling ... st u8: typeof(_425) = *const {l719} u8
                        // 633: b"*** lingeling ... st u8:   l719 = UNIQUE | NON_NULL, (empty)
                        // 633: b"*** lingeling ... \n\0": typeof(_426) = *const {l721} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003a)) }]
                        // 633: b"*** lingeling ... \n\0":   l721 = UNIQUE | NON_NULL, (empty)
                        // 633: b"*** lingeling ... \n\0": typeof(_427) = & {l723} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003a)) }]
                        // 633: b"*** lingeling ... \n\0":   l723 = UNIQUE | NON_NULL, FIXED
                        // 633: b"*** lingeling ... \n\0": typeof(_427 = const b"*** lingeling error: multiple output files \'%s\' and \'%s\'\n\x00") = & {l3346} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003a)) }]
                        // 633: b"*** lingeling ... \n\0":   l3346 = UNIQUE | NON_NULL, (empty)
                        // 633: b"*** lingeling ... \n\0": typeof(_426 = &raw const (*_427)) = *const {l3347} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003a)) }]
                        // 633: b"*** lingeling ... \n\0":   l3347 = UNIQUE | NON_NULL, (empty)
                        // 633: b"*** lingeling ... _char: typeof(_424 = move _425 as *const i8 (Misc)) = *const {l3349} i8
                        // 633: b"*** lingeling ... _char:   l3349 = UNIQUE | NON_NULL, (empty)
                        // 633: b"*** lingeling ... st u8: typeof(_425 = move _426 as *const u8 (Pointer(ArrayToPointer))) = *const {l3348} u8
                        // 633: b"*** lingeling ... st u8:   l3348 = UNIQUE | NON_NULL, (empty)
                            as *const libc::c_char,
                        oname,
                        // 635: oname: typeof(_428) = *const {l725} i8
                        // 635: oname:   l725 = UNIQUE | NON_NULL, (empty)
                        *argv.offset(i as isize),
                        // 636: *argv.offset(i  ... size): typeof(_429) = *mut {l727} i8
                        // 636: *argv.offset(i  ... size):   l727 = UNIQUE | NON_NULL, (empty)
                        // 636: argv.offset(i a ... size): typeof(_430) = *mut {l729} *mut {l730} i8
                        // 636: argv.offset(i a ... size):   l729 = UNIQUE | NON_NULL, (empty)
                        // 636: argv.offset(i a ... size):   l730 = UNIQUE | NON_NULL, (empty)
                        // 636: argv: typeof(_431) = *mut {l732} *mut {l733} i8
                        // 636: argv:   l732 = UNIQUE | NON_NULL, (empty)
                        // 636: argv:   l733 = UNIQUE | NON_NULL, (empty)
                    );
                    res = 1 as libc::c_int;
                    current_block = 14603147171032977705;
                    break;
                } else {
                    oname = *argv.offset(i as isize);
                    // 642: *argv.offset(i  ... size): typeof(_435) = *mut {l738} i8
                    // 642: *argv.offset(i  ... size):   l738 = UNIQUE | NON_NULL, (empty)
                    // 642: argv.offset(i a ... size): typeof(_436) = *mut {l740} *mut {l741} i8
                    // 642: argv.offset(i a ... size):   l740 = UNIQUE | NON_NULL, (empty)
                    // 642: argv.offset(i a ... size):   l741 = UNIQUE | NON_NULL, (empty)
                    // 642: argv: typeof(_437) = *mut {l743} *mut {l744} i8
                    // 642: argv:   l743 = UNIQUE | NON_NULL, (empty)
                    // 642: argv:   l744 = UNIQUE | NON_NULL, (empty)
                    // 642: oname = *argv.o ... size): typeof(_16 = move _435 as *const i8 (Pointer(MutToConstPointer))) = *const {l3350} i8
                    // 642: oname = *argv.o ... size):   l3350 = UNIQUE | NON_NULL, (empty)
                }
            } else if strcmp(
                *argv.offset(i as isize),
                // 645: *argv.offset(i  ... size): typeof(_442) = *const {l750} i8
                // 645: *argv.offset(i  ... size):   l750 = UNIQUE | NON_NULL, (empty)
                // 645: *argv.offset(i  ... size): typeof(_443) = *mut {l752} i8
                // 645: *argv.offset(i  ... size):   l752 = UNIQUE | NON_NULL, (empty)
                // 645: argv.offset(i a ... size): typeof(_444) = *mut {l754} *mut {l755} i8
                // 645: argv.offset(i a ... size):   l754 = UNIQUE | NON_NULL, (empty)
                // 645: argv.offset(i a ... size):   l755 = UNIQUE | NON_NULL, (empty)
                // 645: argv: typeof(_445) = *mut {l757} *mut {l758} i8
                // 645: argv:   l757 = UNIQUE | NON_NULL, (empty)
                // 645: argv:   l758 = UNIQUE | NON_NULL, (empty)
                // 645: *argv.offset(i  ... size): typeof(_442 = move _443 as *const i8 (Pointer(MutToConstPointer))) = *const {l3351} i8
                // 645: *argv.offset(i  ... size):   l3351 = UNIQUE | NON_NULL, (empty)
                b"-p\0" as *const u8 as *const libc::c_char,
                // 646: b"-p\0" as *con ... _char: typeof(_448) = *const {l762} i8
                // 646: b"-p\0" as *con ... _char:   l762 = UNIQUE | NON_NULL, (empty)
                // 646: b"-p\0" as *const u8: typeof(_449) = *const {l764} u8
                // 646: b"-p\0" as *const u8:   l764 = UNIQUE | NON_NULL, (empty)
                // 646: b"-p\0": typeof(_450) = *const {l766} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 646: b"-p\0":   l766 = UNIQUE | NON_NULL, (empty)
                // 646: b"-p\0": typeof(_451) = & {l768} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 646: b"-p\0":   l768 = UNIQUE | NON_NULL, FIXED
                // 646: b"-p\0" as *con ... _char: typeof(_448 = move _449 as *const i8 (Misc)) = *const {l3355} i8
                // 646: b"-p\0" as *con ... _char:   l3355 = UNIQUE | NON_NULL, (empty)
                // 646: b"-p\0": typeof(_450 = &raw const (*_451)) = *const {l3353} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 646: b"-p\0":   l3353 = UNIQUE | NON_NULL, (empty)
                // 646: b"-p\0" as *const u8: typeof(_449 = move _450 as *const u8 (Pointer(ArrayToPointer))) = *const {l3354} u8
                // 646: b"-p\0" as *const u8:   l3354 = UNIQUE | NON_NULL, (empty)
                // 646: b"-p\0": typeof(_451 = const b"-p\x00") = & {l3352} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 646: b"-p\0":   l3352 = UNIQUE | NON_NULL, (empty)
            ) == 0
            {
                i += 1;
                if i == argc {
                    fprintf(
                        stderr,
                        // 652: stderr: typeof(_458) = *mut {l776} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 652: stderr:   l776 = UNIQUE | NON_NULL, (empty)
                        // 652: stderr: typeof(_459) = *mut {l778} *mut {l779} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 652: stderr:   l778 = UNIQUE | NON_NULL, (empty)
                        // 652: stderr:   l779 = UNIQUE | NON_NULL, (empty)
                        b"*** lingeling error: argument to '-p' missing\n\0" as *const u8
                        // 653: b"*** lingeling ... _char: typeof(_460) = *const {l781} i8
                        // 653: b"*** lingeling ... _char:   l781 = UNIQUE | NON_NULL, (empty)
                        // 653: b"*** lingeling ... st u8: typeof(_461) = *const {l783} u8
                        // 653: b"*** lingeling ... st u8:   l783 = UNIQUE | NON_NULL, (empty)
                        // 653: b"*** lingeling ... \n\0": typeof(_462) = *const {l785} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                        // 653: b"*** lingeling ... \n\0":   l785 = UNIQUE | NON_NULL, (empty)
                        // 653: b"*** lingeling ... \n\0": typeof(_463) = & {l787} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                        // 653: b"*** lingeling ... \n\0":   l787 = UNIQUE | NON_NULL, FIXED
                        // 653: b"*** lingeling ... \n\0": typeof(_462 = &raw const (*_463)) = *const {l3357} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                        // 653: b"*** lingeling ... \n\0":   l3357 = UNIQUE | NON_NULL, (empty)
                        // 653: b"*** lingeling ... _char: typeof(_460 = move _461 as *const i8 (Misc)) = *const {l3359} i8
                        // 653: b"*** lingeling ... _char:   l3359 = UNIQUE | NON_NULL, (empty)
                        // 653: b"*** lingeling ... st u8: typeof(_461 = move _462 as *const u8 (Pointer(ArrayToPointer))) = *const {l3358} u8
                        // 653: b"*** lingeling ... st u8:   l3358 = UNIQUE | NON_NULL, (empty)
                        // 653: b"*** lingeling ... \n\0": typeof(_463 = const b"*** lingeling error: argument to \'-p\' missing\n\x00") = & {l3356} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                        // 653: b"*** lingeling ... \n\0":   l3356 = UNIQUE | NON_NULL, (empty)
                            as *const libc::c_char,
                    );
                    res = 1 as libc::c_int;
                    current_block = 14603147171032977705;
                    break;
                } else if !pname.is_null() {
                // 659: pname: typeof(_467) = *const {l792} i8
                // 659: pname:   l792 = UNIQUE | NON_NULL, (empty)
                    fprintf(
                        stderr,
                        // 661: stderr: typeof(_470) = *mut {l796} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 661: stderr:   l796 = UNIQUE | NON_NULL, (empty)
                        // 661: stderr: typeof(_471) = *mut {l798} *mut {l799} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 661: stderr:   l798 = UNIQUE | NON_NULL, (empty)
                        // 661: stderr:   l799 = UNIQUE | NON_NULL, (empty)
                        b"*** lingeling error: multiple option files '%s' and '%s'\n\0" as *const u8
                        // 662: b"*** lingeling ... _char: typeof(_472) = *const {l801} i8
                        // 662: b"*** lingeling ... _char:   l801 = UNIQUE | NON_NULL, (empty)
                        // 662: b"*** lingeling ... st u8: typeof(_473) = *const {l803} u8
                        // 662: b"*** lingeling ... st u8:   l803 = UNIQUE | NON_NULL, (empty)
                        // 662: b"*** lingeling ... \n\0": typeof(_474) = *const {l805} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003a)) }]
                        // 662: b"*** lingeling ... \n\0":   l805 = UNIQUE | NON_NULL, (empty)
                        // 662: b"*** lingeling ... \n\0": typeof(_475) = & {l807} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003a)) }]
                        // 662: b"*** lingeling ... \n\0":   l807 = UNIQUE | NON_NULL, FIXED
                        // 662: b"*** lingeling ... st u8: typeof(_473 = move _474 as *const u8 (Pointer(ArrayToPointer))) = *const {l3362} u8
                        // 662: b"*** lingeling ... st u8:   l3362 = UNIQUE | NON_NULL, (empty)
                        // 662: b"*** lingeling ... _char: typeof(_472 = move _473 as *const i8 (Misc)) = *const {l3363} i8
                        // 662: b"*** lingeling ... _char:   l3363 = UNIQUE | NON_NULL, (empty)
                        // 662: b"*** lingeling ... \n\0": typeof(_475 = const b"*** lingeling error: multiple option files \'%s\' and \'%s\'\n\x00") = & {l3360} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003a)) }]
                        // 662: b"*** lingeling ... \n\0":   l3360 = UNIQUE | NON_NULL, (empty)
                        // 662: b"*** lingeling ... \n\0": typeof(_474 = &raw const (*_475)) = *const {l3361} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003a)) }]
                        // 662: b"*** lingeling ... \n\0":   l3361 = UNIQUE | NON_NULL, (empty)
                            as *const libc::c_char,
                        pname,
                        // 664: pname: typeof(_476) = *const {l809} i8
                        // 664: pname:   l809 = UNIQUE | NON_NULL, (empty)
                        *argv.offset(i as isize),
                        // 665: *argv.offset(i  ... size): typeof(_477) = *mut {l811} i8
                        // 665: *argv.offset(i  ... size):   l811 = UNIQUE | NON_NULL, (empty)
                        // 665: argv.offset(i a ... size): typeof(_478) = *mut {l813} *mut {l814} i8
                        // 665: argv.offset(i a ... size):   l813 = UNIQUE | NON_NULL, (empty)
                        // 665: argv.offset(i a ... size):   l814 = UNIQUE | NON_NULL, (empty)
                        // 665: argv: typeof(_479) = *mut {l816} *mut {l817} i8
                        // 665: argv:   l816 = UNIQUE | NON_NULL, (empty)
                        // 665: argv:   l817 = UNIQUE | NON_NULL, (empty)
                    );
                    res = 1 as libc::c_int;
                    current_block = 14603147171032977705;
                    break;
                } else {
                    pname = *argv.offset(i as isize);
                    // 671: *argv.offset(i  ... size): typeof(_483) = *mut {l822} i8
                    // 671: *argv.offset(i  ... size):   l822 = UNIQUE | NON_NULL, (empty)
                    // 671: argv.offset(i a ... size): typeof(_484) = *mut {l824} *mut {l825} i8
                    // 671: argv.offset(i a ... size):   l824 = UNIQUE | NON_NULL, (empty)
                    // 671: argv.offset(i a ... size):   l825 = UNIQUE | NON_NULL, (empty)
                    // 671: argv: typeof(_485) = *mut {l827} *mut {l828} i8
                    // 671: argv:   l827 = UNIQUE | NON_NULL, (empty)
                    // 671: argv:   l828 = UNIQUE | NON_NULL, (empty)
                    // 671: pname = *argv.o ... size): typeof(_17 = move _483 as *const i8 (Pointer(MutToConstPointer))) = *const {l3364} i8
                    // 671: pname = *argv.o ... size):   l3364 = UNIQUE | NON_NULL, (empty)
                }
            } else if strcmp(
                *argv.offset(i as isize),
                // 674: *argv.offset(i  ... size): typeof(_490) = *const {l834} i8
                // 674: *argv.offset(i  ... size):   l834 = UNIQUE | NON_NULL, (empty)
                // 674: *argv.offset(i  ... size): typeof(_491) = *mut {l836} i8
                // 674: *argv.offset(i  ... size):   l836 = UNIQUE | NON_NULL, (empty)
                // 674: argv.offset(i a ... size): typeof(_492) = *mut {l838} *mut {l839} i8
                // 674: argv.offset(i a ... size):   l838 = UNIQUE | NON_NULL, (empty)
                // 674: argv.offset(i a ... size):   l839 = UNIQUE | NON_NULL, (empty)
                // 674: argv: typeof(_493) = *mut {l841} *mut {l842} i8
                // 674: argv:   l841 = UNIQUE | NON_NULL, (empty)
                // 674: argv:   l842 = UNIQUE | NON_NULL, (empty)
                // 674: *argv.offset(i  ... size): typeof(_490 = move _491 as *const i8 (Pointer(MutToConstPointer))) = *const {l3365} i8
                // 674: *argv.offset(i  ... size):   l3365 = UNIQUE | NON_NULL, (empty)
                b"-T\0" as *const u8 as *const libc::c_char,
                // 675: b"-T\0" as *con ... _char: typeof(_496) = *const {l846} i8
                // 675: b"-T\0" as *con ... _char:   l846 = UNIQUE | NON_NULL, (empty)
                // 675: b"-T\0" as *const u8: typeof(_497) = *const {l848} u8
                // 675: b"-T\0" as *const u8:   l848 = UNIQUE | NON_NULL, (empty)
                // 675: b"-T\0": typeof(_498) = *const {l850} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 675: b"-T\0":   l850 = UNIQUE | NON_NULL, (empty)
                // 675: b"-T\0": typeof(_499) = & {l852} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 675: b"-T\0":   l852 = UNIQUE | NON_NULL, FIXED
                // 675: b"-T\0" as *const u8: typeof(_497 = move _498 as *const u8 (Pointer(ArrayToPointer))) = *const {l3368} u8
                // 675: b"-T\0" as *const u8:   l3368 = UNIQUE | NON_NULL, (empty)
                // 675: b"-T\0": typeof(_498 = &raw const (*_499)) = *const {l3367} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 675: b"-T\0":   l3367 = UNIQUE | NON_NULL, (empty)
                // 675: b"-T\0" as *con ... _char: typeof(_496 = move _497 as *const i8 (Misc)) = *const {l3369} i8
                // 675: b"-T\0" as *con ... _char:   l3369 = UNIQUE | NON_NULL, (empty)
                // 675: b"-T\0": typeof(_499 = const b"-T\x00") = & {l3366} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 675: b"-T\0":   l3366 = UNIQUE | NON_NULL, (empty)
            ) == 0
            {
                i += 1;
                if i == argc {
                    fprintf(
                        stderr,
                        // 681: stderr: typeof(_506) = *mut {l860} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 681: stderr:   l860 = UNIQUE | NON_NULL, (empty)
                        // 681: stderr: typeof(_507) = *mut {l862} *mut {l863} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 681: stderr:   l862 = UNIQUE | NON_NULL, (empty)
                        // 681: stderr:   l863 = UNIQUE | NON_NULL, (empty)
                        b"*** lingeling error: argument to '-T' missing\n\0" as *const u8
                        // 682: b"*** lingeling ... _char: typeof(_508) = *const {l865} i8
                        // 682: b"*** lingeling ... _char:   l865 = UNIQUE | NON_NULL, (empty)
                        // 682: b"*** lingeling ... st u8: typeof(_509) = *const {l867} u8
                        // 682: b"*** lingeling ... st u8:   l867 = UNIQUE | NON_NULL, (empty)
                        // 682: b"*** lingeling ... \n\0": typeof(_510) = *const {l869} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                        // 682: b"*** lingeling ... \n\0":   l869 = UNIQUE | NON_NULL, (empty)
                        // 682: b"*** lingeling ... \n\0": typeof(_511) = & {l871} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                        // 682: b"*** lingeling ... \n\0":   l871 = UNIQUE | NON_NULL, FIXED
                        // 682: b"*** lingeling ... \n\0": typeof(_510 = &raw const (*_511)) = *const {l3371} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                        // 682: b"*** lingeling ... \n\0":   l3371 = UNIQUE | NON_NULL, (empty)
                        // 682: b"*** lingeling ... st u8: typeof(_509 = move _510 as *const u8 (Pointer(ArrayToPointer))) = *const {l3372} u8
                        // 682: b"*** lingeling ... st u8:   l3372 = UNIQUE | NON_NULL, (empty)
                        // 682: b"*** lingeling ... \n\0": typeof(_511 = const b"*** lingeling error: argument to \'-T\' missing\n\x00") = & {l3370} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                        // 682: b"*** lingeling ... \n\0":   l3370 = UNIQUE | NON_NULL, (empty)
                        // 682: b"*** lingeling ... _char: typeof(_508 = move _509 as *const i8 (Misc)) = *const {l3373} i8
                        // 682: b"*** lingeling ... _char:   l3373 = UNIQUE | NON_NULL, (empty)
                            as *const libc::c_char,
                    );
                    res = 1 as libc::c_int;
                    current_block = 14603147171032977705;
                    break;
                } else if timelimit >= 0 as libc::c_int {
                // 688: timelimit: typeof(_515) = *mut {l876} i32
                // 688: timelimit:   l876 = UNIQUE | NON_NULL, (empty)
                    fprintf(
                        stderr,
                        // 690: stderr: typeof(_519) = *mut {l881} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 690: stderr:   l881 = UNIQUE | NON_NULL, (empty)
                        // 690: stderr: typeof(_520) = *mut {l883} *mut {l884} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 690: stderr:   l883 = UNIQUE | NON_NULL, (empty)
                        // 690: stderr:   l884 = UNIQUE | NON_NULL, (empty)
                        b"*** lingeling error: timit limit set twice\n\0" as *const u8
                        // 691: b"*** lingeling ... _char: typeof(_521) = *const {l886} i8
                        // 691: b"*** lingeling ... _char:   l886 = UNIQUE | NON_NULL, (empty)
                        // 691: b"*** lingeling ... st u8: typeof(_522) = *const {l888} u8
                        // 691: b"*** lingeling ... st u8:   l888 = UNIQUE | NON_NULL, (empty)
                        // 691: b"*** lingeling ... \n\0": typeof(_523) = *const {l890} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                        // 691: b"*** lingeling ... \n\0":   l890 = UNIQUE | NON_NULL, (empty)
                        // 691: b"*** lingeling ... \n\0": typeof(_524) = & {l892} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                        // 691: b"*** lingeling ... \n\0":   l892 = UNIQUE | NON_NULL, FIXED
                        // 691: b"*** lingeling ... _char: typeof(_521 = move _522 as *const i8 (Misc)) = *const {l3377} i8
                        // 691: b"*** lingeling ... _char:   l3377 = UNIQUE | NON_NULL, (empty)
                        // 691: b"*** lingeling ... st u8: typeof(_522 = move _523 as *const u8 (Pointer(ArrayToPointer))) = *const {l3376} u8
                        // 691: b"*** lingeling ... st u8:   l3376 = UNIQUE | NON_NULL, (empty)
                        // 691: b"*** lingeling ... \n\0": typeof(_523 = &raw const (*_524)) = *const {l3375} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                        // 691: b"*** lingeling ... \n\0":   l3375 = UNIQUE | NON_NULL, (empty)
                        // 691: b"*** lingeling ... \n\0": typeof(_524 = const b"*** lingeling error: timit limit set twice\n\x00") = & {l3374} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                        // 691: b"*** lingeling ... \n\0":   l3374 = UNIQUE | NON_NULL, (empty)
                            as *const libc::c_char,
                    );
                    res = 1 as libc::c_int;
                    current_block = 14603147171032977705;
                    break;
                } else {
                    p = *argv.offset(i as isize);
                    // 698: *argv.offset(i  ... size): typeof(_526) = *mut {l895} i8
                    // 698: *argv.offset(i  ... size):   l895 = UNIQUE | NON_NULL, (empty)
                    // 698: argv.offset(i a ... size): typeof(_527) = *mut {l897} *mut {l898} i8
                    // 698: argv.offset(i a ... size):   l897 = UNIQUE | NON_NULL, (empty)
                    // 698: argv.offset(i a ... size):   l898 = UNIQUE | NON_NULL, (empty)
                    // 698: argv: typeof(_528) = *mut {l900} *mut {l901} i8
                    // 698: argv:   l900 = UNIQUE | NON_NULL, (empty)
                    // 698: argv:   l901 = UNIQUE | NON_NULL, (empty)
                    // 698: p = *argv.offse ... size): typeof(_19 = move _526 as *const i8 (Pointer(MutToConstPointer))) = *const {l3378} i8
                    // 698: p = *argv.offse ... size):   l3378 = UNIQUE | NON_NULL, (empty)
                    while *p as libc::c_int != 0
                        && *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                        // 700: (*__ctype_b_loc ... size): typeof(_540) = *const {l914} u16
                        // 700: (*__ctype_b_loc ... size):   l914 = UNIQUE | NON_NULL, (empty)
                        // 700: (*__ctype_b_loc()): typeof(_541) = *const {l916} u16
                        // 700: (*__ctype_b_loc()):   l916 = UNIQUE | NON_NULL, (empty)
                        // 700: __ctype_b_loc(): typeof(_542) = *mut {l918} *const {l919} u16
                        // 700: __ctype_b_loc():   l918 = UNIQUE | NON_NULL, (empty)
                        // 700: __ctype_b_loc():   l919 = UNIQUE | NON_NULL, (empty)
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                    {
                        p = p.offset(1);
                        // 704: p.offset(1): typeof(_549) = *const {l927} i8
                        // 704: p.offset(1):   l927 = UNIQUE | NON_NULL, (empty)
                        // 704: p: typeof(_550) = *const {l929} i8
                        // 704: p:   l929 = UNIQUE | NON_NULL, (empty)
                        p;
                        // 705: p: typeof(_551) = *const {l931} i8
                        // 705: p:   l931 = UNIQUE | NON_NULL, (empty)
                    }
                    if p == *argv.offset(i as isize) as *const libc::c_char
                    // 707: p: typeof(_558) = *const {l939} i8
                    // 707: p:   l939 = UNIQUE | NON_NULL, (empty)
                    // 707: *argv.offset(i  ... _char: typeof(_559) = *const {l941} i8
                    // 707: *argv.offset(i  ... _char:   l941 = UNIQUE | NON_NULL, (empty)
                    // 707: *argv.offset(i  ... size): typeof(_560) = *mut {l943} i8
                    // 707: *argv.offset(i  ... size):   l943 = UNIQUE | NON_NULL, (empty)
                    // 707: argv.offset(i a ... size): typeof(_561) = *mut {l945} *mut {l946} i8
                    // 707: argv.offset(i a ... size):   l945 = UNIQUE | NON_NULL, (empty)
                    // 707: argv.offset(i a ... size):   l946 = UNIQUE | NON_NULL, (empty)
                    // 707: argv: typeof(_562) = *mut {l948} *mut {l949} i8
                    // 707: argv:   l948 = UNIQUE | NON_NULL, (empty)
                    // 707: argv:   l949 = UNIQUE | NON_NULL, (empty)
                    // 707: *argv.offset(i  ... size): typeof(_559 = move _560 as *const i8 (Pointer(MutToConstPointer))) = *const {l3379} i8
                    // 707: *argv.offset(i  ... size):   l3379 = UNIQUE | NON_NULL, (empty)
                        || *p as libc::c_int != 0
                        || {
                            timelimit = atoi(*argv.offset(i as isize));
                            // 710: *argv.offset(i  ... size): typeof(_570) = *const {l958} i8
                            // 710: *argv.offset(i  ... size):   l958 = UNIQUE | NON_NULL, (empty)
                            // 710: *argv.offset(i  ... size): typeof(_571) = *mut {l960} i8
                            // 710: *argv.offset(i  ... size):   l960 = UNIQUE | NON_NULL, (empty)
                            // 710: argv.offset(i a ... size): typeof(_572) = *mut {l962} *mut {l963} i8
                            // 710: argv.offset(i a ... size):   l962 = UNIQUE | NON_NULL, (empty)
                            // 710: argv.offset(i a ... size):   l963 = UNIQUE | NON_NULL, (empty)
                            // 710: argv: typeof(_573) = *mut {l965} *mut {l966} i8
                            // 710: argv:   l965 = UNIQUE | NON_NULL, (empty)
                            // 710: argv:   l966 = UNIQUE | NON_NULL, (empty)
                            // 710: timelimit: typeof(_576) = *mut {l970} i32
                            // 710: timelimit:   l970 = UNIQUE | NON_NULL, (empty)
                            // 710: *argv.offset(i  ... size): typeof(_570 = move _571 as *const i8 (Pointer(MutToConstPointer))) = *const {l3380} i8
                            // 710: *argv.offset(i  ... size):   l3380 = UNIQUE | NON_NULL, (empty)
                            timelimit < 0 as libc::c_int
                            // 711: timelimit: typeof(_578) = *mut {l973} i32
                            // 711: timelimit:   l973 = UNIQUE | NON_NULL, (empty)
                        }
                    {
                        fprintf(
                            stderr,
                            // 715: stderr: typeof(_582) = *mut {l978} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                            // 715: stderr:   l978 = UNIQUE | NON_NULL, (empty)
                            // 715: stderr: typeof(_583) = *mut {l980} *mut {l981} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                            // 715: stderr:   l980 = UNIQUE | NON_NULL, (empty)
                            // 715: stderr:   l981 = UNIQUE | NON_NULL, (empty)
                            b"*** lingeling error: invalid time limit '-T %s'\n\0" as *const u8
                            // 716: b"*** lingeling ... _char: typeof(_584) = *const {l983} i8
                            // 716: b"*** lingeling ... _char:   l983 = UNIQUE | NON_NULL, (empty)
                            // 716: b"*** lingeling ... st u8: typeof(_585) = *const {l985} u8
                            // 716: b"*** lingeling ... st u8:   l985 = UNIQUE | NON_NULL, (empty)
                            // 716: b"*** lingeling ... \n\0": typeof(_586) = *const {l987} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                            // 716: b"*** lingeling ... \n\0":   l987 = UNIQUE | NON_NULL, (empty)
                            // 716: b"*** lingeling ... \n\0": typeof(_587) = & {l989} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                            // 716: b"*** lingeling ... \n\0":   l989 = UNIQUE | NON_NULL, FIXED
                            // 716: b"*** lingeling ... _char: typeof(_584 = move _585 as *const i8 (Misc)) = *const {l3384} i8
                            // 716: b"*** lingeling ... _char:   l3384 = UNIQUE | NON_NULL, (empty)
                            // 716: b"*** lingeling ... \n\0": typeof(_587 = const b"*** lingeling error: invalid time limit \'-T %s\'\n\x00") = & {l3381} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                            // 716: b"*** lingeling ... \n\0":   l3381 = UNIQUE | NON_NULL, (empty)
                            // 716: b"*** lingeling ... st u8: typeof(_585 = move _586 as *const u8 (Pointer(ArrayToPointer))) = *const {l3383} u8
                            // 716: b"*** lingeling ... st u8:   l3383 = UNIQUE | NON_NULL, (empty)
                            // 716: b"*** lingeling ... \n\0": typeof(_586 = &raw const (*_587)) = *const {l3382} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                            // 716: b"*** lingeling ... \n\0":   l3382 = UNIQUE | NON_NULL, (empty)
                                as *const libc::c_char,
                            *argv.offset(i as isize),
                            // 718: *argv.offset(i  ... size): typeof(_588) = *mut {l991} i8
                            // 718: *argv.offset(i  ... size):   l991 = UNIQUE | NON_NULL, (empty)
                            // 718: argv.offset(i a ... size): typeof(_589) = *mut {l993} *mut {l994} i8
                            // 718: argv.offset(i a ... size):   l993 = UNIQUE | NON_NULL, (empty)
                            // 718: argv.offset(i a ... size):   l994 = UNIQUE | NON_NULL, (empty)
                            // 718: argv: typeof(_590) = *mut {l996} *mut {l997} i8
                            // 718: argv:   l996 = UNIQUE | NON_NULL, (empty)
                            // 718: argv:   l997 = UNIQUE | NON_NULL, (empty)
                        );
                        res = 1 as libc::c_int;
                        current_block = 14603147171032977705;
                        break;
                    }
                }
            } else if strcmp(
                *argv.offset(i as isize),
                // 726: *argv.offset(i  ... size): typeof(_596) = *const {l1004} i8
                // 726: *argv.offset(i  ... size):   l1004 = UNIQUE | NON_NULL, (empty)
                // 726: *argv.offset(i  ... size): typeof(_597) = *mut {l1006} i8
                // 726: *argv.offset(i  ... size):   l1006 = UNIQUE | NON_NULL, (empty)
                // 726: argv.offset(i a ... size): typeof(_598) = *mut {l1008} *mut {l1009} i8
                // 726: argv.offset(i a ... size):   l1008 = UNIQUE | NON_NULL, (empty)
                // 726: argv.offset(i a ... size):   l1009 = UNIQUE | NON_NULL, (empty)
                // 726: argv: typeof(_599) = *mut {l1011} *mut {l1012} i8
                // 726: argv:   l1011 = UNIQUE | NON_NULL, (empty)
                // 726: argv:   l1012 = UNIQUE | NON_NULL, (empty)
                // 726: *argv.offset(i  ... size): typeof(_596 = move _597 as *const i8 (Pointer(MutToConstPointer))) = *const {l3385} i8
                // 726: *argv.offset(i  ... size):   l3385 = UNIQUE | NON_NULL, (empty)
                b"-a\0" as *const u8 as *const libc::c_char,
                // 727: b"-a\0" as *con ... _char: typeof(_602) = *const {l1016} i8
                // 727: b"-a\0" as *con ... _char:   l1016 = UNIQUE | NON_NULL, (empty)
                // 727: b"-a\0" as *const u8: typeof(_603) = *const {l1018} u8
                // 727: b"-a\0" as *const u8:   l1018 = UNIQUE | NON_NULL, (empty)
                // 727: b"-a\0": typeof(_604) = *const {l1020} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 727: b"-a\0":   l1020 = UNIQUE | NON_NULL, (empty)
                // 727: b"-a\0": typeof(_605) = & {l1022} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 727: b"-a\0":   l1022 = UNIQUE | NON_NULL, FIXED
                // 727: b"-a\0": typeof(_605 = const b"-a\x00") = & {l3386} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 727: b"-a\0":   l3386 = UNIQUE | NON_NULL, (empty)
                // 727: b"-a\0" as *const u8: typeof(_603 = move _604 as *const u8 (Pointer(ArrayToPointer))) = *const {l3388} u8
                // 727: b"-a\0" as *const u8:   l3388 = UNIQUE | NON_NULL, (empty)
                // 727: b"-a\0" as *con ... _char: typeof(_602 = move _603 as *const i8 (Misc)) = *const {l3389} i8
                // 727: b"-a\0" as *con ... _char:   l3389 = UNIQUE | NON_NULL, (empty)
                // 727: b"-a\0": typeof(_604 = &raw const (*_605)) = *const {l3387} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 727: b"-a\0":   l3387 = UNIQUE | NON_NULL, (empty)
            ) == 0
            {
                i += 1;
                if i == argc {
                    fprintf(
                        stderr,
                        // 733: stderr: typeof(_612) = *mut {l1030} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 733: stderr:   l1030 = UNIQUE | NON_NULL, (empty)
                        // 733: stderr: typeof(_613) = *mut {l1032} *mut {l1033} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 733: stderr:   l1032 = UNIQUE | NON_NULL, (empty)
                        // 733: stderr:   l1033 = UNIQUE | NON_NULL, (empty)
                        b"*** lingeling error: argument to '-a' missing\n\0" as *const u8
                        // 734: b"*** lingeling ... _char: typeof(_614) = *const {l1035} i8
                        // 734: b"*** lingeling ... _char:   l1035 = UNIQUE | NON_NULL, (empty)
                        // 734: b"*** lingeling ... st u8: typeof(_615) = *const {l1037} u8
                        // 734: b"*** lingeling ... st u8:   l1037 = UNIQUE | NON_NULL, (empty)
                        // 734: b"*** lingeling ... \n\0": typeof(_616) = *const {l1039} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                        // 734: b"*** lingeling ... \n\0":   l1039 = UNIQUE | NON_NULL, (empty)
                        // 734: b"*** lingeling ... \n\0": typeof(_617) = & {l1041} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                        // 734: b"*** lingeling ... \n\0":   l1041 = UNIQUE | NON_NULL, FIXED
                        // 734: b"*** lingeling ... \n\0": typeof(_616 = &raw const (*_617)) = *const {l3391} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                        // 734: b"*** lingeling ... \n\0":   l3391 = UNIQUE | NON_NULL, (empty)
                        // 734: b"*** lingeling ... _char: typeof(_614 = move _615 as *const i8 (Misc)) = *const {l3393} i8
                        // 734: b"*** lingeling ... _char:   l3393 = UNIQUE | NON_NULL, (empty)
                        // 734: b"*** lingeling ... \n\0": typeof(_617 = const b"*** lingeling error: argument to \'-a\' missing\n\x00") = & {l3390} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                        // 734: b"*** lingeling ... \n\0":   l3390 = UNIQUE | NON_NULL, (empty)
                        // 734: b"*** lingeling ... st u8: typeof(_615 = move _616 as *const u8 (Pointer(ArrayToPointer))) = *const {l3392} u8
                        // 734: b"*** lingeling ... st u8:   l3392 = UNIQUE | NON_NULL, (empty)
                            as *const libc::c_char,
                    );
                    res = 1 as libc::c_int;
                    current_block = 14603147171032977705;
                    break;
                } else {
                    target = atoi(*argv.offset(i as isize));
                    // 741: *argv.offset(i  ... size): typeof(_620) = *const {l1045} i8
                    // 741: *argv.offset(i  ... size):   l1045 = UNIQUE | NON_NULL, (empty)
                    // 741: *argv.offset(i  ... size): typeof(_621) = *mut {l1047} i8
                    // 741: *argv.offset(i  ... size):   l1047 = UNIQUE | NON_NULL, (empty)
                    // 741: argv.offset(i a ... size): typeof(_622) = *mut {l1049} *mut {l1050} i8
                    // 741: argv.offset(i a ... size):   l1049 = UNIQUE | NON_NULL, (empty)
                    // 741: argv.offset(i a ... size):   l1050 = UNIQUE | NON_NULL, (empty)
                    // 741: argv: typeof(_623) = *mut {l1052} *mut {l1053} i8
                    // 741: argv:   l1052 = UNIQUE | NON_NULL, (empty)
                    // 741: argv:   l1053 = UNIQUE | NON_NULL, (empty)
                    // 741: *argv.offset(i  ... size): typeof(_620 = move _621 as *const i8 (Pointer(MutToConstPointer))) = *const {l3394} i8
                    // 741: *argv.offset(i  ... size):   l3394 = UNIQUE | NON_NULL, (empty)
                    if target == 0 {
                        fprintf(
                            stderr,
                            // 744: stderr: typeof(_630) = *mut {l1061} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                            // 744: stderr:   l1061 = UNIQUE | NON_NULL, (empty)
                            // 744: stderr: typeof(_631) = *mut {l1063} *mut {l1064} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                            // 744: stderr:   l1063 = UNIQUE | NON_NULL, (empty)
                            // 744: stderr:   l1064 = UNIQUE | NON_NULL, (empty)
                            b"*** lingeling error: invalid literal in '-a %d'\n\0" as *const u8
                            // 745: b"*** lingeling ... _char: typeof(_632) = *const {l1066} i8
                            // 745: b"*** lingeling ... _char:   l1066 = UNIQUE | NON_NULL, (empty)
                            // 745: b"*** lingeling ... st u8: typeof(_633) = *const {l1068} u8
                            // 745: b"*** lingeling ... st u8:   l1068 = UNIQUE | NON_NULL, (empty)
                            // 745: b"*** lingeling ... \n\0": typeof(_634) = *const {l1070} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                            // 745: b"*** lingeling ... \n\0":   l1070 = UNIQUE | NON_NULL, (empty)
                            // 745: b"*** lingeling ... \n\0": typeof(_635) = & {l1072} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                            // 745: b"*** lingeling ... \n\0":   l1072 = UNIQUE | NON_NULL, FIXED
                            // 745: b"*** lingeling ... \n\0": typeof(_635 = const b"*** lingeling error: invalid literal in \'-a %d\'\n\x00") = & {l3395} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                            // 745: b"*** lingeling ... \n\0":   l3395 = UNIQUE | NON_NULL, (empty)
                            // 745: b"*** lingeling ... st u8: typeof(_633 = move _634 as *const u8 (Pointer(ArrayToPointer))) = *const {l3397} u8
                            // 745: b"*** lingeling ... st u8:   l3397 = UNIQUE | NON_NULL, (empty)
                            // 745: b"*** lingeling ... _char: typeof(_632 = move _633 as *const i8 (Misc)) = *const {l3398} i8
                            // 745: b"*** lingeling ... _char:   l3398 = UNIQUE | NON_NULL, (empty)
                            // 745: b"*** lingeling ... \n\0": typeof(_634 = &raw const (*_635)) = *const {l3396} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                            // 745: b"*** lingeling ... \n\0":   l3396 = UNIQUE | NON_NULL, (empty)
                                as *const libc::c_char,
                            target,
                        );
                        res = 1 as libc::c_int;
                        current_block = 14603147171032977705;
                        break;
                    } else {
                        lglpushtarget(target);
                    }
                }
            } else if strcmp(
                *argv.offset(i as isize),
                // 757: *argv.offset(i  ... size): typeof(_643) = *const {l1081} i8
                // 757: *argv.offset(i  ... size):   l1081 = UNIQUE | NON_NULL, (empty)
                // 757: *argv.offset(i  ... size): typeof(_644) = *mut {l1083} i8
                // 757: *argv.offset(i  ... size):   l1083 = UNIQUE | NON_NULL, (empty)
                // 757: argv.offset(i a ... size): typeof(_645) = *mut {l1085} *mut {l1086} i8
                // 757: argv.offset(i a ... size):   l1085 = UNIQUE | NON_NULL, (empty)
                // 757: argv.offset(i a ... size):   l1086 = UNIQUE | NON_NULL, (empty)
                // 757: argv: typeof(_646) = *mut {l1088} *mut {l1089} i8
                // 757: argv:   l1088 = UNIQUE | NON_NULL, (empty)
                // 757: argv:   l1089 = UNIQUE | NON_NULL, (empty)
                // 757: *argv.offset(i  ... size): typeof(_643 = move _644 as *const i8 (Pointer(MutToConstPointer))) = *const {l3399} i8
                // 757: *argv.offset(i  ... size):   l3399 = UNIQUE | NON_NULL, (empty)
                b"-d\0" as *const u8 as *const libc::c_char,
                // 758: b"-d\0" as *con ... _char: typeof(_649) = *const {l1093} i8
                // 758: b"-d\0" as *con ... _char:   l1093 = UNIQUE | NON_NULL, (empty)
                // 758: b"-d\0" as *const u8: typeof(_650) = *const {l1095} u8
                // 758: b"-d\0" as *const u8:   l1095 = UNIQUE | NON_NULL, (empty)
                // 758: b"-d\0": typeof(_651) = *const {l1097} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 758: b"-d\0":   l1097 = UNIQUE | NON_NULL, (empty)
                // 758: b"-d\0": typeof(_652) = & {l1099} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 758: b"-d\0":   l1099 = UNIQUE | NON_NULL, FIXED
                // 758: b"-d\0" as *const u8: typeof(_650 = move _651 as *const u8 (Pointer(ArrayToPointer))) = *const {l3402} u8
                // 758: b"-d\0" as *const u8:   l3402 = UNIQUE | NON_NULL, (empty)
                // 758: b"-d\0" as *con ... _char: typeof(_649 = move _650 as *const i8 (Misc)) = *const {l3403} i8
                // 758: b"-d\0" as *con ... _char:   l3403 = UNIQUE | NON_NULL, (empty)
                // 758: b"-d\0": typeof(_652 = const b"-d\x00") = & {l3400} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 758: b"-d\0":   l3400 = UNIQUE | NON_NULL, (empty)
                // 758: b"-d\0": typeof(_651 = &raw const (*_652)) = *const {l3401} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 758: b"-d\0":   l3401 = UNIQUE | NON_NULL, (empty)
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    // 761: *argv.offset(i  ... size): typeof(_655) = *const {l1103} i8
                    // 761: *argv.offset(i  ... size):   l1103 = UNIQUE | NON_NULL, (empty)
                    // 761: *argv.offset(i  ... size): typeof(_656) = *mut {l1105} i8
                    // 761: *argv.offset(i  ... size):   l1105 = UNIQUE | NON_NULL, (empty)
                    // 761: argv.offset(i a ... size): typeof(_657) = *mut {l1107} *mut {l1108} i8
                    // 761: argv.offset(i a ... size):   l1107 = UNIQUE | NON_NULL, (empty)
                    // 761: argv.offset(i a ... size):   l1108 = UNIQUE | NON_NULL, (empty)
                    // 761: argv: typeof(_658) = *mut {l1110} *mut {l1111} i8
                    // 761: argv:   l1110 = UNIQUE | NON_NULL, (empty)
                    // 761: argv:   l1111 = UNIQUE | NON_NULL, (empty)
                    // 761: *argv.offset(i  ... size): typeof(_655 = move _656 as *const i8 (Pointer(MutToConstPointer))) = *const {l3404} i8
                    // 761: *argv.offset(i  ... size):   l3404 = UNIQUE | NON_NULL, (empty)
                    b"--defaults\0" as *const u8 as *const libc::c_char,
                    // 762: b"--defaults\0" ... _char: typeof(_661) = *const {l1115} i8
                    // 762: b"--defaults\0" ... _char:   l1115 = UNIQUE | NON_NULL, (empty)
                    // 762: b"--defaults\0" ... st u8: typeof(_662) = *const {l1117} u8
                    // 762: b"--defaults\0" ... st u8:   l1117 = UNIQUE | NON_NULL, (empty)
                    // 762: b"--defaults\0": typeof(_663) = *const {l1119} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 762: b"--defaults\0":   l1119 = UNIQUE | NON_NULL, (empty)
                    // 762: b"--defaults\0": typeof(_664) = & {l1121} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 762: b"--defaults\0":   l1121 = UNIQUE | NON_NULL, FIXED
                    // 762: b"--defaults\0": typeof(_663 = &raw const (*_664)) = *const {l3406} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 762: b"--defaults\0":   l3406 = UNIQUE | NON_NULL, (empty)
                    // 762: b"--defaults\0" ... st u8: typeof(_662 = move _663 as *const u8 (Pointer(ArrayToPointer))) = *const {l3407} u8
                    // 762: b"--defaults\0" ... st u8:   l3407 = UNIQUE | NON_NULL, (empty)
                    // 762: b"--defaults\0": typeof(_664 = const b"--defaults\x00") = & {l3405} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 762: b"--defaults\0":   l3405 = UNIQUE | NON_NULL, (empty)
                    // 762: b"--defaults\0" ... _char: typeof(_661 = move _662 as *const i8 (Misc)) = *const {l3408} i8
                    // 762: b"--defaults\0" ... _char:   l3408 = UNIQUE | NON_NULL, (empty)
                ) == 0
            {
                lglopts(
                    lgl,
                    // 766: lgl: typeof(_667) = *mut {l1125} LGL
                    // 766: lgl:   l1125 = UNIQUE | NON_NULL, (empty)
                    b"\0" as *const u8 as *const libc::c_char,
                    // 767: b"\0" as *const ... _char: typeof(_668) = *const {l1127} i8
                    // 767: b"\0" as *const ... _char:   l1127 = UNIQUE | NON_NULL, (empty)
                    // 767: b"\0" as *const u8: typeof(_669) = *const {l1129} u8
                    // 767: b"\0" as *const u8:   l1129 = UNIQUE | NON_NULL, (empty)
                    // 767: b"\0": typeof(_670) = *const {l1131} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
                    // 767: b"\0":   l1131 = UNIQUE | NON_NULL, (empty)
                    // 767: b"\0": typeof(_671) = & {l1133} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
                    // 767: b"\0":   l1133 = UNIQUE | NON_NULL, FIXED
                    // 767: b"\0" as *const u8: typeof(_669 = move _670 as *const u8 (Pointer(ArrayToPointer))) = *const {l3411} u8
                    // 767: b"\0" as *const u8:   l3411 = UNIQUE | NON_NULL, (empty)
                    // 767: b"\0" as *const ... _char: typeof(_668 = move _669 as *const i8 (Misc)) = *const {l3412} i8
                    // 767: b"\0" as *const ... _char:   l3412 = UNIQUE | NON_NULL, (empty)
                    // 767: b"\0": typeof(_671 = const b"\x00") = & {l3409} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
                    // 767: b"\0":   l3409 = UNIQUE | NON_NULL, (empty)
                    // 767: b"\0": typeof(_670 = &raw const (*_671)) = *const {l3410} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
                    // 767: b"\0":   l3410 = UNIQUE | NON_NULL, (empty)
                    0 as libc::c_int,
                );
                current_block = 14603147171032977705;
                break;
            } else if strcmp(
                *argv.offset(i as isize),
                // 773: *argv.offset(i  ... size): typeof(_676) = *const {l1139} i8
                // 773: *argv.offset(i  ... size):   l1139 = UNIQUE | NON_NULL, (empty)
                // 773: *argv.offset(i  ... size): typeof(_677) = *mut {l1141} i8
                // 773: *argv.offset(i  ... size):   l1141 = UNIQUE | NON_NULL, (empty)
                // 773: argv.offset(i a ... size): typeof(_678) = *mut {l1143} *mut {l1144} i8
                // 773: argv.offset(i a ... size):   l1143 = UNIQUE | NON_NULL, (empty)
                // 773: argv.offset(i a ... size):   l1144 = UNIQUE | NON_NULL, (empty)
                // 773: argv: typeof(_679) = *mut {l1146} *mut {l1147} i8
                // 773: argv:   l1146 = UNIQUE | NON_NULL, (empty)
                // 773: argv:   l1147 = UNIQUE | NON_NULL, (empty)
                // 773: *argv.offset(i  ... size): typeof(_676 = move _677 as *const i8 (Pointer(MutToConstPointer))) = *const {l3413} i8
                // 773: *argv.offset(i  ... size):   l3413 = UNIQUE | NON_NULL, (empty)
                b"-e\0" as *const u8 as *const libc::c_char,
                // 774: b"-e\0" as *con ... _char: typeof(_682) = *const {l1151} i8
                // 774: b"-e\0" as *con ... _char:   l1151 = UNIQUE | NON_NULL, (empty)
                // 774: b"-e\0" as *const u8: typeof(_683) = *const {l1153} u8
                // 774: b"-e\0" as *const u8:   l1153 = UNIQUE | NON_NULL, (empty)
                // 774: b"-e\0": typeof(_684) = *const {l1155} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 774: b"-e\0":   l1155 = UNIQUE | NON_NULL, (empty)
                // 774: b"-e\0": typeof(_685) = & {l1157} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 774: b"-e\0":   l1157 = UNIQUE | NON_NULL, FIXED
                // 774: b"-e\0" as *const u8: typeof(_683 = move _684 as *const u8 (Pointer(ArrayToPointer))) = *const {l3416} u8
                // 774: b"-e\0" as *const u8:   l3416 = UNIQUE | NON_NULL, (empty)
                // 774: b"-e\0": typeof(_684 = &raw const (*_685)) = *const {l3415} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 774: b"-e\0":   l3415 = UNIQUE | NON_NULL, (empty)
                // 774: b"-e\0" as *con ... _char: typeof(_682 = move _683 as *const i8 (Misc)) = *const {l3417} i8
                // 774: b"-e\0" as *con ... _char:   l3417 = UNIQUE | NON_NULL, (empty)
                // 774: b"-e\0": typeof(_685 = const b"-e\x00") = & {l3414} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 774: b"-e\0":   l3414 = UNIQUE | NON_NULL, (empty)
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    // 777: *argv.offset(i  ... size): typeof(_688) = *const {l1161} i8
                    // 777: *argv.offset(i  ... size):   l1161 = UNIQUE | NON_NULL, (empty)
                    // 777: *argv.offset(i  ... size): typeof(_689) = *mut {l1163} i8
                    // 777: *argv.offset(i  ... size):   l1163 = UNIQUE | NON_NULL, (empty)
                    // 777: argv.offset(i a ... size): typeof(_690) = *mut {l1165} *mut {l1166} i8
                    // 777: argv.offset(i a ... size):   l1165 = UNIQUE | NON_NULL, (empty)
                    // 777: argv.offset(i a ... size):   l1166 = UNIQUE | NON_NULL, (empty)
                    // 777: argv: typeof(_691) = *mut {l1168} *mut {l1169} i8
                    // 777: argv:   l1168 = UNIQUE | NON_NULL, (empty)
                    // 777: argv:   l1169 = UNIQUE | NON_NULL, (empty)
                    // 777: *argv.offset(i  ... size): typeof(_688 = move _689 as *const i8 (Pointer(MutToConstPointer))) = *const {l3418} i8
                    // 777: *argv.offset(i  ... size):   l3418 = UNIQUE | NON_NULL, (empty)
                    b"--embedded\0" as *const u8 as *const libc::c_char,
                    // 778: b"--embedded\0" ... _char: typeof(_694) = *const {l1173} i8
                    // 778: b"--embedded\0" ... _char:   l1173 = UNIQUE | NON_NULL, (empty)
                    // 778: b"--embedded\0" ... st u8: typeof(_695) = *const {l1175} u8
                    // 778: b"--embedded\0" ... st u8:   l1175 = UNIQUE | NON_NULL, (empty)
                    // 778: b"--embedded\0": typeof(_696) = *const {l1177} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 778: b"--embedded\0":   l1177 = UNIQUE | NON_NULL, (empty)
                    // 778: b"--embedded\0": typeof(_697) = & {l1179} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 778: b"--embedded\0":   l1179 = UNIQUE | NON_NULL, FIXED
                    // 778: b"--embedded\0": typeof(_696 = &raw const (*_697)) = *const {l3420} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 778: b"--embedded\0":   l3420 = UNIQUE | NON_NULL, (empty)
                    // 778: b"--embedded\0": typeof(_697 = const b"--embedded\x00") = & {l3419} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 778: b"--embedded\0":   l3419 = UNIQUE | NON_NULL, (empty)
                    // 778: b"--embedded\0" ... st u8: typeof(_695 = move _696 as *const u8 (Pointer(ArrayToPointer))) = *const {l3421} u8
                    // 778: b"--embedded\0" ... st u8:   l3421 = UNIQUE | NON_NULL, (empty)
                    // 778: b"--embedded\0" ... _char: typeof(_694 = move _695 as *const i8 (Misc)) = *const {l3422} i8
                    // 778: b"--embedded\0" ... _char:   l3422 = UNIQUE | NON_NULL, (empty)
                ) == 0
            {
                lglopts(
                    lgl,
                    // 782: lgl: typeof(_700) = *mut {l1183} LGL
                    // 782: lgl:   l1183 = UNIQUE | NON_NULL, (empty)
                    b"c \0" as *const u8 as *const libc::c_char,
                    // 783: b"c \0" as *con ... _char: typeof(_701) = *const {l1185} i8
                    // 783: b"c \0" as *con ... _char:   l1185 = UNIQUE | NON_NULL, (empty)
                    // 783: b"c \0" as *const u8: typeof(_702) = *const {l1187} u8
                    // 783: b"c \0" as *const u8:   l1187 = UNIQUE | NON_NULL, (empty)
                    // 783: b"c \0": typeof(_703) = *const {l1189} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                    // 783: b"c \0":   l1189 = UNIQUE | NON_NULL, (empty)
                    // 783: b"c \0": typeof(_704) = & {l1191} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                    // 783: b"c \0":   l1191 = UNIQUE | NON_NULL, FIXED
                    // 783: b"c \0": typeof(_703 = &raw const (*_704)) = *const {l3424} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                    // 783: b"c \0":   l3424 = UNIQUE | NON_NULL, (empty)
                    // 783: b"c \0" as *con ... _char: typeof(_701 = move _702 as *const i8 (Misc)) = *const {l3426} i8
                    // 783: b"c \0" as *con ... _char:   l3426 = UNIQUE | NON_NULL, (empty)
                    // 783: b"c \0": typeof(_704 = const b"c \x00") = & {l3423} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                    // 783: b"c \0":   l3423 = UNIQUE | NON_NULL, (empty)
                    // 783: b"c \0" as *const u8: typeof(_702 = move _703 as *const u8 (Pointer(ArrayToPointer))) = *const {l3425} u8
                    // 783: b"c \0" as *const u8:   l3425 = UNIQUE | NON_NULL, (empty)
                    1 as libc::c_int,
                );
                current_block = 14603147171032977705;
                break;
            } else if strcmp(
                *argv.offset(i as isize),
                // 789: *argv.offset(i  ... size): typeof(_709) = *const {l1197} i8
                // 789: *argv.offset(i  ... size):   l1197 = UNIQUE | NON_NULL, (empty)
                // 789: *argv.offset(i  ... size): typeof(_710) = *mut {l1199} i8
                // 789: *argv.offset(i  ... size):   l1199 = UNIQUE | NON_NULL, (empty)
                // 789: argv.offset(i a ... size): typeof(_711) = *mut {l1201} *mut {l1202} i8
                // 789: argv.offset(i a ... size):   l1201 = UNIQUE | NON_NULL, (empty)
                // 789: argv.offset(i a ... size):   l1202 = UNIQUE | NON_NULL, (empty)
                // 789: argv: typeof(_712) = *mut {l1204} *mut {l1205} i8
                // 789: argv:   l1204 = UNIQUE | NON_NULL, (empty)
                // 789: argv:   l1205 = UNIQUE | NON_NULL, (empty)
                // 789: *argv.offset(i  ... size): typeof(_709 = move _710 as *const i8 (Pointer(MutToConstPointer))) = *const {l3427} i8
                // 789: *argv.offset(i  ... size):   l3427 = UNIQUE | NON_NULL, (empty)
                b"-r\0" as *const u8 as *const libc::c_char,
                // 790: b"-r\0" as *con ... _char: typeof(_715) = *const {l1209} i8
                // 790: b"-r\0" as *con ... _char:   l1209 = UNIQUE | NON_NULL, (empty)
                // 790: b"-r\0" as *const u8: typeof(_716) = *const {l1211} u8
                // 790: b"-r\0" as *const u8:   l1211 = UNIQUE | NON_NULL, (empty)
                // 790: b"-r\0": typeof(_717) = *const {l1213} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 790: b"-r\0":   l1213 = UNIQUE | NON_NULL, (empty)
                // 790: b"-r\0": typeof(_718) = & {l1215} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 790: b"-r\0":   l1215 = UNIQUE | NON_NULL, FIXED
                // 790: b"-r\0" as *con ... _char: typeof(_715 = move _716 as *const i8 (Misc)) = *const {l3431} i8
                // 790: b"-r\0" as *con ... _char:   l3431 = UNIQUE | NON_NULL, (empty)
                // 790: b"-r\0": typeof(_717 = &raw const (*_718)) = *const {l3429} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 790: b"-r\0":   l3429 = UNIQUE | NON_NULL, (empty)
                // 790: b"-r\0" as *const u8: typeof(_716 = move _717 as *const u8 (Pointer(ArrayToPointer))) = *const {l3430} u8
                // 790: b"-r\0" as *const u8:   l3430 = UNIQUE | NON_NULL, (empty)
                // 790: b"-r\0": typeof(_718 = const b"-r\x00") = & {l3428} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 790: b"-r\0":   l3428 = UNIQUE | NON_NULL, (empty)
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    // 793: *argv.offset(i  ... size): typeof(_721) = *const {l1219} i8
                    // 793: *argv.offset(i  ... size):   l1219 = UNIQUE | NON_NULL, (empty)
                    // 793: *argv.offset(i  ... size): typeof(_722) = *mut {l1221} i8
                    // 793: *argv.offset(i  ... size):   l1221 = UNIQUE | NON_NULL, (empty)
                    // 793: argv.offset(i a ... size): typeof(_723) = *mut {l1223} *mut {l1224} i8
                    // 793: argv.offset(i a ... size):   l1223 = UNIQUE | NON_NULL, (empty)
                    // 793: argv.offset(i a ... size):   l1224 = UNIQUE | NON_NULL, (empty)
                    // 793: argv: typeof(_724) = *mut {l1226} *mut {l1227} i8
                    // 793: argv:   l1226 = UNIQUE | NON_NULL, (empty)
                    // 793: argv:   l1227 = UNIQUE | NON_NULL, (empty)
                    // 793: *argv.offset(i  ... size): typeof(_721 = move _722 as *const i8 (Pointer(MutToConstPointer))) = *const {l3432} i8
                    // 793: *argv.offset(i  ... size):   l3432 = UNIQUE | NON_NULL, (empty)
                    b"--ranges\0" as *const u8 as *const libc::c_char,
                    // 794: b"--ranges\0" a ... _char: typeof(_727) = *const {l1231} i8
                    // 794: b"--ranges\0" a ... _char:   l1231 = UNIQUE | NON_NULL, (empty)
                    // 794: b"--ranges\0" a ... st u8: typeof(_728) = *const {l1233} u8
                    // 794: b"--ranges\0" a ... st u8:   l1233 = UNIQUE | NON_NULL, (empty)
                    // 794: b"--ranges\0": typeof(_729) = *const {l1235} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 794: b"--ranges\0":   l1235 = UNIQUE | NON_NULL, (empty)
                    // 794: b"--ranges\0": typeof(_730) = & {l1237} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 794: b"--ranges\0":   l1237 = UNIQUE | NON_NULL, FIXED
                    // 794: b"--ranges\0" a ... _char: typeof(_727 = move _728 as *const i8 (Misc)) = *const {l3436} i8
                    // 794: b"--ranges\0" a ... _char:   l3436 = UNIQUE | NON_NULL, (empty)
                    // 794: b"--ranges\0": typeof(_730 = const b"--ranges\x00") = & {l3433} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 794: b"--ranges\0":   l3433 = UNIQUE | NON_NULL, (empty)
                    // 794: b"--ranges\0": typeof(_729 = &raw const (*_730)) = *const {l3434} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 794: b"--ranges\0":   l3434 = UNIQUE | NON_NULL, (empty)
                    // 794: b"--ranges\0" a ... st u8: typeof(_728 = move _729 as *const u8 (Pointer(ArrayToPointer))) = *const {l3435} u8
                    // 794: b"--ranges\0" a ... st u8:   l3435 = UNIQUE | NON_NULL, (empty)
                ) == 0
            {
                lglrgopts(lgl);
                // 797: lgl: typeof(_733) = *mut {l1241} LGL
                // 797: lgl:   l1241 = UNIQUE | NON_NULL, (empty)
                current_block = 14603147171032977705;
                break;
            } else if strcmp(
                *argv.offset(i as isize),
                // 801: *argv.offset(i  ... size): typeof(_737) = *const {l1246} i8
                // 801: *argv.offset(i  ... size):   l1246 = UNIQUE | NON_NULL, (empty)
                // 801: *argv.offset(i  ... size): typeof(_738) = *mut {l1248} i8
                // 801: *argv.offset(i  ... size):   l1248 = UNIQUE | NON_NULL, (empty)
                // 801: argv.offset(i a ... size): typeof(_739) = *mut {l1250} *mut {l1251} i8
                // 801: argv.offset(i a ... size):   l1250 = UNIQUE | NON_NULL, (empty)
                // 801: argv.offset(i a ... size):   l1251 = UNIQUE | NON_NULL, (empty)
                // 801: argv: typeof(_740) = *mut {l1253} *mut {l1254} i8
                // 801: argv:   l1253 = UNIQUE | NON_NULL, (empty)
                // 801: argv:   l1254 = UNIQUE | NON_NULL, (empty)
                // 801: *argv.offset(i  ... size): typeof(_737 = move _738 as *const i8 (Pointer(MutToConstPointer))) = *const {l3437} i8
                // 801: *argv.offset(i  ... size):   l3437 = UNIQUE | NON_NULL, (empty)
                b"-P\0" as *const u8 as *const libc::c_char,
                // 802: b"-P\0" as *con ... _char: typeof(_743) = *const {l1258} i8
                // 802: b"-P\0" as *con ... _char:   l1258 = UNIQUE | NON_NULL, (empty)
                // 802: b"-P\0" as *const u8: typeof(_744) = *const {l1260} u8
                // 802: b"-P\0" as *const u8:   l1260 = UNIQUE | NON_NULL, (empty)
                // 802: b"-P\0": typeof(_745) = *const {l1262} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 802: b"-P\0":   l1262 = UNIQUE | NON_NULL, (empty)
                // 802: b"-P\0": typeof(_746) = & {l1264} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 802: b"-P\0":   l1264 = UNIQUE | NON_NULL, FIXED
                // 802: b"-P\0" as *const u8: typeof(_744 = move _745 as *const u8 (Pointer(ArrayToPointer))) = *const {l3440} u8
                // 802: b"-P\0" as *const u8:   l3440 = UNIQUE | NON_NULL, (empty)
                // 802: b"-P\0" as *con ... _char: typeof(_743 = move _744 as *const i8 (Misc)) = *const {l3441} i8
                // 802: b"-P\0" as *con ... _char:   l3441 = UNIQUE | NON_NULL, (empty)
                // 802: b"-P\0": typeof(_745 = &raw const (*_746)) = *const {l3439} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 802: b"-P\0":   l3439 = UNIQUE | NON_NULL, (empty)
                // 802: b"-P\0": typeof(_746 = const b"-P\x00") = & {l3438} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 802: b"-P\0":   l3438 = UNIQUE | NON_NULL, (empty)
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    // 805: *argv.offset(i  ... size): typeof(_749) = *const {l1268} i8
                    // 805: *argv.offset(i  ... size):   l1268 = UNIQUE | NON_NULL, (empty)
                    // 805: *argv.offset(i  ... size): typeof(_750) = *mut {l1270} i8
                    // 805: *argv.offset(i  ... size):   l1270 = UNIQUE | NON_NULL, (empty)
                    // 805: argv.offset(i a ... size): typeof(_751) = *mut {l1272} *mut {l1273} i8
                    // 805: argv.offset(i a ... size):   l1272 = UNIQUE | NON_NULL, (empty)
                    // 805: argv.offset(i a ... size):   l1273 = UNIQUE | NON_NULL, (empty)
                    // 805: argv: typeof(_752) = *mut {l1275} *mut {l1276} i8
                    // 805: argv:   l1275 = UNIQUE | NON_NULL, (empty)
                    // 805: argv:   l1276 = UNIQUE | NON_NULL, (empty)
                    // 805: *argv.offset(i  ... size): typeof(_749 = move _750 as *const i8 (Pointer(MutToConstPointer))) = *const {l3442} i8
                    // 805: *argv.offset(i  ... size):   l3442 = UNIQUE | NON_NULL, (empty)
                    b"--pcs\0" as *const u8 as *const libc::c_char,
                    // 806: b"--pcs\0" as * ... _char: typeof(_755) = *const {l1280} i8
                    // 806: b"--pcs\0" as * ... _char:   l1280 = UNIQUE | NON_NULL, (empty)
                    // 806: b"--pcs\0" as * ... st u8: typeof(_756) = *const {l1282} u8
                    // 806: b"--pcs\0" as * ... st u8:   l1282 = UNIQUE | NON_NULL, (empty)
                    // 806: b"--pcs\0": typeof(_757) = *const {l1284} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 806: b"--pcs\0":   l1284 = UNIQUE | NON_NULL, (empty)
                    // 806: b"--pcs\0": typeof(_758) = & {l1286} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 806: b"--pcs\0":   l1286 = UNIQUE | NON_NULL, FIXED
                    // 806: b"--pcs\0": typeof(_758 = const b"--pcs\x00") = & {l3443} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 806: b"--pcs\0":   l3443 = UNIQUE | NON_NULL, (empty)
                    // 806: b"--pcs\0": typeof(_757 = &raw const (*_758)) = *const {l3444} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 806: b"--pcs\0":   l3444 = UNIQUE | NON_NULL, (empty)
                    // 806: b"--pcs\0" as * ... st u8: typeof(_756 = move _757 as *const u8 (Pointer(ArrayToPointer))) = *const {l3445} u8
                    // 806: b"--pcs\0" as * ... st u8:   l3445 = UNIQUE | NON_NULL, (empty)
                    // 806: b"--pcs\0" as * ... _char: typeof(_755 = move _756 as *const i8 (Misc)) = *const {l3446} i8
                    // 806: b"--pcs\0" as * ... _char:   l3446 = UNIQUE | NON_NULL, (empty)
                ) == 0
            {
                printf(b"# generated by 'lingeling --pcs'\n\0" as *const u8 as *const libc::c_char);
                // 809: b"# generated b ... _char: typeof(_761) = *const {l1290} i8
                // 809: b"# generated b ... _char:   l1290 = UNIQUE | NON_NULL, (empty)
                // 809: b"# generated b ... st u8: typeof(_762) = *const {l1292} u8
                // 809: b"# generated b ... st u8:   l1292 = UNIQUE | NON_NULL, (empty)
                // 809: b"# generated b ... \n\0": typeof(_763) = *const {l1294} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                // 809: b"# generated b ... \n\0":   l1294 = UNIQUE | NON_NULL, (empty)
                // 809: b"# generated b ... \n\0": typeof(_764) = & {l1296} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                // 809: b"# generated b ... \n\0":   l1296 = UNIQUE | NON_NULL, FIXED
                // 809: b"# generated b ... _char: typeof(_761 = move _762 as *const i8 (Misc)) = *const {l3450} i8
                // 809: b"# generated b ... _char:   l3450 = UNIQUE | NON_NULL, (empty)
                // 809: b"# generated b ... \n\0": typeof(_763 = &raw const (*_764)) = *const {l3448} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                // 809: b"# generated b ... \n\0":   l3448 = UNIQUE | NON_NULL, (empty)
                // 809: b"# generated b ... st u8: typeof(_762 = move _763 as *const u8 (Pointer(ArrayToPointer))) = *const {l3449} u8
                // 809: b"# generated b ... st u8:   l3449 = UNIQUE | NON_NULL, (empty)
                // 809: b"# generated b ... \n\0": typeof(_764 = const b"# generated by \'lingeling --pcs\'\n\x00") = & {l3447} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                // 809: b"# generated b ... \n\0":   l3447 = UNIQUE | NON_NULL, (empty)
                printf(
                    b"# version %s\n\0" as *const u8 as *const libc::c_char,
                    // 811: b"# version %s\ ... _char: typeof(_766) = *const {l1299} i8
                    // 811: b"# version %s\ ... _char:   l1299 = UNIQUE | NON_NULL, (empty)
                    // 811: b"# version %s\ ... st u8: typeof(_767) = *const {l1301} u8
                    // 811: b"# version %s\ ... st u8:   l1301 = UNIQUE | NON_NULL, (empty)
                    // 811: b"# version %s\n\0": typeof(_768) = *const {l1303} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                    // 811: b"# version %s\n\0":   l1303 = UNIQUE | NON_NULL, (empty)
                    // 811: b"# version %s\n\0": typeof(_769) = & {l1305} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                    // 811: b"# version %s\n\0":   l1305 = UNIQUE | NON_NULL, FIXED
                    // 811: b"# version %s\ ... st u8: typeof(_767 = move _768 as *const u8 (Pointer(ArrayToPointer))) = *const {l3453} u8
                    // 811: b"# version %s\ ... st u8:   l3453 = UNIQUE | NON_NULL, (empty)
                    // 811: b"# version %s\n\0": typeof(_768 = &raw const (*_769)) = *const {l3452} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                    // 811: b"# version %s\n\0":   l3452 = UNIQUE | NON_NULL, (empty)
                    // 811: b"# version %s\n\0": typeof(_769 = const b"# version %s\n\x00") = & {l3451} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                    // 811: b"# version %s\n\0":   l3451 = UNIQUE | NON_NULL, (empty)
                    // 811: b"# version %s\ ... _char: typeof(_766 = move _767 as *const i8 (Misc)) = *const {l3454} i8
                    // 811: b"# version %s\ ... _char:   l3454 = UNIQUE | NON_NULL, (empty)
                    lglversion(),
                    // 812: lglversion(): typeof(_770) = *const {l1307} i8
                    // 812: lglversion():   l1307 = UNIQUE | NON_NULL, (empty)
                );
                lglpcs(lgl, 0 as libc::c_int);
                // 814: lgl: typeof(_772) = *mut {l1310} LGL
                // 814: lgl:   l1310 = UNIQUE | NON_NULL, (empty)
                current_block = 14603147171032977705;
                break;
            } else if strcmp(
                *argv.offset(i as isize),
                // 818: *argv.offset(i  ... size): typeof(_776) = *const {l1315} i8
                // 818: *argv.offset(i  ... size):   l1315 = UNIQUE | NON_NULL, (empty)
                // 818: *argv.offset(i  ... size): typeof(_777) = *mut {l1317} i8
                // 818: *argv.offset(i  ... size):   l1317 = UNIQUE | NON_NULL, (empty)
                // 818: argv.offset(i a ... size): typeof(_778) = *mut {l1319} *mut {l1320} i8
                // 818: argv.offset(i a ... size):   l1319 = UNIQUE | NON_NULL, (empty)
                // 818: argv.offset(i a ... size):   l1320 = UNIQUE | NON_NULL, (empty)
                // 818: argv: typeof(_779) = *mut {l1322} *mut {l1323} i8
                // 818: argv:   l1322 = UNIQUE | NON_NULL, (empty)
                // 818: argv:   l1323 = UNIQUE | NON_NULL, (empty)
                // 818: *argv.offset(i  ... size): typeof(_776 = move _777 as *const i8 (Pointer(MutToConstPointer))) = *const {l3455} i8
                // 818: *argv.offset(i  ... size):   l3455 = UNIQUE | NON_NULL, (empty)
                b"--pcs-mixed\0" as *const u8 as *const libc::c_char,
                // 819: b"--pcs-mixed\0 ... _char: typeof(_782) = *const {l1327} i8
                // 819: b"--pcs-mixed\0 ... _char:   l1327 = UNIQUE | NON_NULL, (empty)
                // 819: b"--pcs-mixed\0 ... st u8: typeof(_783) = *const {l1329} u8
                // 819: b"--pcs-mixed\0 ... st u8:   l1329 = UNIQUE | NON_NULL, (empty)
                // 819: b"--pcs-mixed\0": typeof(_784) = *const {l1331} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                // 819: b"--pcs-mixed\0":   l1331 = UNIQUE | NON_NULL, (empty)
                // 819: b"--pcs-mixed\0": typeof(_785) = & {l1333} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                // 819: b"--pcs-mixed\0":   l1333 = UNIQUE | NON_NULL, FIXED
                // 819: b"--pcs-mixed\0": typeof(_785 = const b"--pcs-mixed\x00") = & {l3456} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                // 819: b"--pcs-mixed\0":   l3456 = UNIQUE | NON_NULL, (empty)
                // 819: b"--pcs-mixed\0": typeof(_784 = &raw const (*_785)) = *const {l3457} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                // 819: b"--pcs-mixed\0":   l3457 = UNIQUE | NON_NULL, (empty)
                // 819: b"--pcs-mixed\0 ... st u8: typeof(_783 = move _784 as *const u8 (Pointer(ArrayToPointer))) = *const {l3458} u8
                // 819: b"--pcs-mixed\0 ... st u8:   l3458 = UNIQUE | NON_NULL, (empty)
                // 819: b"--pcs-mixed\0 ... _char: typeof(_782 = move _783 as *const i8 (Misc)) = *const {l3459} i8
                // 819: b"--pcs-mixed\0 ... _char:   l3459 = UNIQUE | NON_NULL, (empty)
            ) == 0
            {
                printf(
                    b"# generated by 'lingeling --pcs-mixed'\n\0" as *const u8
                    // 823: b"# generated b ... _char: typeof(_788) = *const {l1337} i8
                    // 823: b"# generated b ... _char:   l1337 = UNIQUE | NON_NULL, (empty)
                    // 823: b"# generated b ... st u8: typeof(_789) = *const {l1339} u8
                    // 823: b"# generated b ... st u8:   l1339 = UNIQUE | NON_NULL, (empty)
                    // 823: b"# generated b ... \n\0": typeof(_790) = *const {l1341} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                    // 823: b"# generated b ... \n\0":   l1341 = UNIQUE | NON_NULL, (empty)
                    // 823: b"# generated b ... \n\0": typeof(_791) = & {l1343} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                    // 823: b"# generated b ... \n\0":   l1343 = UNIQUE | NON_NULL, FIXED
                    // 823: b"# generated b ... \n\0": typeof(_791 = const b"# generated by \'lingeling --pcs-mixed\'\n\x00") = & {l3460} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                    // 823: b"# generated b ... \n\0":   l3460 = UNIQUE | NON_NULL, (empty)
                    // 823: b"# generated b ... \n\0": typeof(_790 = &raw const (*_791)) = *const {l3461} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                    // 823: b"# generated b ... \n\0":   l3461 = UNIQUE | NON_NULL, (empty)
                    // 823: b"# generated b ... _char: typeof(_788 = move _789 as *const i8 (Misc)) = *const {l3463} i8
                    // 823: b"# generated b ... _char:   l3463 = UNIQUE | NON_NULL, (empty)
                    // 823: b"# generated b ... st u8: typeof(_789 = move _790 as *const u8 (Pointer(ArrayToPointer))) = *const {l3462} u8
                    // 823: b"# generated b ... st u8:   l3462 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                );
                printf(
                    b"# version %s\n\0" as *const u8 as *const libc::c_char,
                    // 827: b"# version %s\ ... _char: typeof(_793) = *const {l1346} i8
                    // 827: b"# version %s\ ... _char:   l1346 = UNIQUE | NON_NULL, (empty)
                    // 827: b"# version %s\ ... st u8: typeof(_794) = *const {l1348} u8
                    // 827: b"# version %s\ ... st u8:   l1348 = UNIQUE | NON_NULL, (empty)
                    // 827: b"# version %s\n\0": typeof(_795) = *const {l1350} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                    // 827: b"# version %s\n\0":   l1350 = UNIQUE | NON_NULL, (empty)
                    // 827: b"# version %s\n\0": typeof(_796) = & {l1352} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                    // 827: b"# version %s\n\0":   l1352 = UNIQUE | NON_NULL, FIXED
                    // 827: b"# version %s\n\0": typeof(_795 = &raw const (*_796)) = *const {l3465} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                    // 827: b"# version %s\n\0":   l3465 = UNIQUE | NON_NULL, (empty)
                    // 827: b"# version %s\n\0": typeof(_796 = const b"# version %s\n\x00") = & {l3464} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                    // 827: b"# version %s\n\0":   l3464 = UNIQUE | NON_NULL, (empty)
                    // 827: b"# version %s\ ... _char: typeof(_793 = move _794 as *const i8 (Misc)) = *const {l3467} i8
                    // 827: b"# version %s\ ... _char:   l3467 = UNIQUE | NON_NULL, (empty)
                    // 827: b"# version %s\ ... st u8: typeof(_794 = move _795 as *const u8 (Pointer(ArrayToPointer))) = *const {l3466} u8
                    // 827: b"# version %s\ ... st u8:   l3466 = UNIQUE | NON_NULL, (empty)
                    lglversion(),
                    // 828: lglversion(): typeof(_797) = *const {l1354} i8
                    // 828: lglversion():   l1354 = UNIQUE | NON_NULL, (empty)
                );
                lglpcs(lgl, 1 as libc::c_int);
                // 830: lgl: typeof(_799) = *mut {l1357} LGL
                // 830: lgl:   l1357 = UNIQUE | NON_NULL, (empty)
                current_block = 14603147171032977705;
                break;
            } else if strcmp(
                *argv.offset(i as isize),
                // 834: *argv.offset(i  ... size): typeof(_803) = *const {l1362} i8
                // 834: *argv.offset(i  ... size):   l1362 = UNIQUE | NON_NULL, (empty)
                // 834: *argv.offset(i  ... size): typeof(_804) = *mut {l1364} i8
                // 834: *argv.offset(i  ... size):   l1364 = UNIQUE | NON_NULL, (empty)
                // 834: argv.offset(i a ... size): typeof(_805) = *mut {l1366} *mut {l1367} i8
                // 834: argv.offset(i a ... size):   l1366 = UNIQUE | NON_NULL, (empty)
                // 834: argv.offset(i a ... size):   l1367 = UNIQUE | NON_NULL, (empty)
                // 834: argv: typeof(_806) = *mut {l1369} *mut {l1370} i8
                // 834: argv:   l1369 = UNIQUE | NON_NULL, (empty)
                // 834: argv:   l1370 = UNIQUE | NON_NULL, (empty)
                // 834: *argv.offset(i  ... size): typeof(_803 = move _804 as *const i8 (Pointer(MutToConstPointer))) = *const {l3468} i8
                // 834: *argv.offset(i  ... size):   l3468 = UNIQUE | NON_NULL, (empty)
                b"--pcs-reduced\0" as *const u8 as *const libc::c_char,
                // 835: b"--pcs-reduced ... _char: typeof(_809) = *const {l1374} i8
                // 835: b"--pcs-reduced ... _char:   l1374 = UNIQUE | NON_NULL, (empty)
                // 835: b"--pcs-reduced ... st u8: typeof(_810) = *const {l1376} u8
                // 835: b"--pcs-reduced ... st u8:   l1376 = UNIQUE | NON_NULL, (empty)
                // 835: b"--pcs-reduced\0": typeof(_811) = *const {l1378} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                // 835: b"--pcs-reduced\0":   l1378 = UNIQUE | NON_NULL, (empty)
                // 835: b"--pcs-reduced\0": typeof(_812) = & {l1380} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                // 835: b"--pcs-reduced\0":   l1380 = UNIQUE | NON_NULL, FIXED
                // 835: b"--pcs-reduced\0": typeof(_812 = const b"--pcs-reduced\x00") = & {l3469} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                // 835: b"--pcs-reduced\0":   l3469 = UNIQUE | NON_NULL, (empty)
                // 835: b"--pcs-reduced\0": typeof(_811 = &raw const (*_812)) = *const {l3470} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                // 835: b"--pcs-reduced\0":   l3470 = UNIQUE | NON_NULL, (empty)
                // 835: b"--pcs-reduced ... _char: typeof(_809 = move _810 as *const i8 (Misc)) = *const {l3472} i8
                // 835: b"--pcs-reduced ... _char:   l3472 = UNIQUE | NON_NULL, (empty)
                // 835: b"--pcs-reduced ... st u8: typeof(_810 = move _811 as *const u8 (Pointer(ArrayToPointer))) = *const {l3471} u8
                // 835: b"--pcs-reduced ... st u8:   l3471 = UNIQUE | NON_NULL, (empty)
            ) == 0
            {
                printf(
                    b"# generated by 'lingeling --pcs-reduced'\n\0" as *const u8
                    // 839: b"# generated b ... _char: typeof(_815) = *const {l1384} i8
                    // 839: b"# generated b ... _char:   l1384 = UNIQUE | NON_NULL, (empty)
                    // 839: b"# generated b ... st u8: typeof(_816) = *const {l1386} u8
                    // 839: b"# generated b ... st u8:   l1386 = UNIQUE | NON_NULL, (empty)
                    // 839: b"# generated b ... \n\0": typeof(_817) = *const {l1388} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                    // 839: b"# generated b ... \n\0":   l1388 = UNIQUE | NON_NULL, (empty)
                    // 839: b"# generated b ... \n\0": typeof(_818) = & {l1390} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                    // 839: b"# generated b ... \n\0":   l1390 = UNIQUE | NON_NULL, FIXED
                    // 839: b"# generated b ... _char: typeof(_815 = move _816 as *const i8 (Misc)) = *const {l3476} i8
                    // 839: b"# generated b ... _char:   l3476 = UNIQUE | NON_NULL, (empty)
                    // 839: b"# generated b ... \n\0": typeof(_817 = &raw const (*_818)) = *const {l3474} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                    // 839: b"# generated b ... \n\0":   l3474 = UNIQUE | NON_NULL, (empty)
                    // 839: b"# generated b ... st u8: typeof(_816 = move _817 as *const u8 (Pointer(ArrayToPointer))) = *const {l3475} u8
                    // 839: b"# generated b ... st u8:   l3475 = UNIQUE | NON_NULL, (empty)
                    // 839: b"# generated b ... \n\0": typeof(_818 = const b"# generated by \'lingeling --pcs-reduced\'\n\x00") = & {l3473} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                    // 839: b"# generated b ... \n\0":   l3473 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                );
                printf(
                    b"# version %s\n\0" as *const u8 as *const libc::c_char,
                    // 843: b"# version %s\ ... _char: typeof(_820) = *const {l1393} i8
                    // 843: b"# version %s\ ... _char:   l1393 = UNIQUE | NON_NULL, (empty)
                    // 843: b"# version %s\ ... st u8: typeof(_821) = *const {l1395} u8
                    // 843: b"# version %s\ ... st u8:   l1395 = UNIQUE | NON_NULL, (empty)
                    // 843: b"# version %s\n\0": typeof(_822) = *const {l1397} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                    // 843: b"# version %s\n\0":   l1397 = UNIQUE | NON_NULL, (empty)
                    // 843: b"# version %s\n\0": typeof(_823) = & {l1399} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                    // 843: b"# version %s\n\0":   l1399 = UNIQUE | NON_NULL, FIXED
                    // 843: b"# version %s\n\0": typeof(_822 = &raw const (*_823)) = *const {l3478} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                    // 843: b"# version %s\n\0":   l3478 = UNIQUE | NON_NULL, (empty)
                    // 843: b"# version %s\n\0": typeof(_823 = const b"# version %s\n\x00") = & {l3477} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                    // 843: b"# version %s\n\0":   l3477 = UNIQUE | NON_NULL, (empty)
                    // 843: b"# version %s\ ... _char: typeof(_820 = move _821 as *const i8 (Misc)) = *const {l3480} i8
                    // 843: b"# version %s\ ... _char:   l3480 = UNIQUE | NON_NULL, (empty)
                    // 843: b"# version %s\ ... st u8: typeof(_821 = move _822 as *const u8 (Pointer(ArrayToPointer))) = *const {l3479} u8
                    // 843: b"# version %s\ ... st u8:   l3479 = UNIQUE | NON_NULL, (empty)
                    lglversion(),
                    // 844: lglversion(): typeof(_824) = *const {l1401} i8
                    // 844: lglversion():   l1401 = UNIQUE | NON_NULL, (empty)
                );
                lglpcs(lgl, -(1 as libc::c_int));
                // 846: lgl: typeof(_826) = *mut {l1404} LGL
                // 846: lgl:   l1404 = UNIQUE | NON_NULL, (empty)
                current_block = 14603147171032977705;
                break;
            } else if strcmp(
                *argv.offset(i as isize),
                // 850: *argv.offset(i  ... size): typeof(_833) = *const {l1412} i8
                // 850: *argv.offset(i  ... size):   l1412 = UNIQUE | NON_NULL, (empty)
                // 850: *argv.offset(i  ... size): typeof(_834) = *mut {l1414} i8
                // 850: *argv.offset(i  ... size):   l1414 = UNIQUE | NON_NULL, (empty)
                // 850: argv.offset(i a ... size): typeof(_835) = *mut {l1416} *mut {l1417} i8
                // 850: argv.offset(i a ... size):   l1416 = UNIQUE | NON_NULL, (empty)
                // 850: argv.offset(i a ... size):   l1417 = UNIQUE | NON_NULL, (empty)
                // 850: argv: typeof(_836) = *mut {l1419} *mut {l1420} i8
                // 850: argv:   l1419 = UNIQUE | NON_NULL, (empty)
                // 850: argv:   l1420 = UNIQUE | NON_NULL, (empty)
                // 850: *argv.offset(i  ... size): typeof(_833 = move _834 as *const i8 (Pointer(MutToConstPointer))) = *const {l3481} i8
                // 850: *argv.offset(i  ... size):   l3481 = UNIQUE | NON_NULL, (empty)
                b"-f\0" as *const u8 as *const libc::c_char,
                // 851: b"-f\0" as *con ... _char: typeof(_839) = *const {l1424} i8
                // 851: b"-f\0" as *con ... _char:   l1424 = UNIQUE | NON_NULL, (empty)
                // 851: b"-f\0" as *const u8: typeof(_840) = *const {l1426} u8
                // 851: b"-f\0" as *const u8:   l1426 = UNIQUE | NON_NULL, (empty)
                // 851: b"-f\0": typeof(_841) = *const {l1428} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 851: b"-f\0":   l1428 = UNIQUE | NON_NULL, (empty)
                // 851: b"-f\0": typeof(_842) = & {l1430} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 851: b"-f\0":   l1430 = UNIQUE | NON_NULL, FIXED
                // 851: b"-f\0" as *con ... _char: typeof(_839 = move _840 as *const i8 (Misc)) = *const {l3485} i8
                // 851: b"-f\0" as *con ... _char:   l3485 = UNIQUE | NON_NULL, (empty)
                // 851: b"-f\0" as *const u8: typeof(_840 = move _841 as *const u8 (Pointer(ArrayToPointer))) = *const {l3484} u8
                // 851: b"-f\0" as *const u8:   l3484 = UNIQUE | NON_NULL, (empty)
                // 851: b"-f\0": typeof(_841 = &raw const (*_842)) = *const {l3483} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 851: b"-f\0":   l3483 = UNIQUE | NON_NULL, (empty)
                // 851: b"-f\0": typeof(_842 = const b"-f\x00") = & {l3482} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 851: b"-f\0":   l3482 = UNIQUE | NON_NULL, (empty)
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    // 854: *argv.offset(i  ... size): typeof(_845) = *const {l1434} i8
                    // 854: *argv.offset(i  ... size):   l1434 = UNIQUE | NON_NULL, (empty)
                    // 854: *argv.offset(i  ... size): typeof(_846) = *mut {l1436} i8
                    // 854: *argv.offset(i  ... size):   l1436 = UNIQUE | NON_NULL, (empty)
                    // 854: argv.offset(i a ... size): typeof(_847) = *mut {l1438} *mut {l1439} i8
                    // 854: argv.offset(i a ... size):   l1438 = UNIQUE | NON_NULL, (empty)
                    // 854: argv.offset(i a ... size):   l1439 = UNIQUE | NON_NULL, (empty)
                    // 854: argv: typeof(_848) = *mut {l1441} *mut {l1442} i8
                    // 854: argv:   l1441 = UNIQUE | NON_NULL, (empty)
                    // 854: argv:   l1442 = UNIQUE | NON_NULL, (empty)
                    // 854: *argv.offset(i  ... size): typeof(_845 = move _846 as *const i8 (Pointer(MutToConstPointer))) = *const {l3486} i8
                    // 854: *argv.offset(i  ... size):   l3486 = UNIQUE | NON_NULL, (empty)
                    b"--force\0" as *const u8 as *const libc::c_char,
                    // 855: b"--force\0" as ... _char: typeof(_851) = *const {l1446} i8
                    // 855: b"--force\0" as ... _char:   l1446 = UNIQUE | NON_NULL, (empty)
                    // 855: b"--force\0" as ... st u8: typeof(_852) = *const {l1448} u8
                    // 855: b"--force\0" as ... st u8:   l1448 = UNIQUE | NON_NULL, (empty)
                    // 855: b"--force\0": typeof(_853) = *const {l1450} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 855: b"--force\0":   l1450 = UNIQUE | NON_NULL, (empty)
                    // 855: b"--force\0": typeof(_854) = & {l1452} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 855: b"--force\0":   l1452 = UNIQUE | NON_NULL, FIXED
                    // 855: b"--force\0": typeof(_854 = const b"--force\x00") = & {l3487} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 855: b"--force\0":   l3487 = UNIQUE | NON_NULL, (empty)
                    // 855: b"--force\0": typeof(_853 = &raw const (*_854)) = *const {l3488} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 855: b"--force\0":   l3488 = UNIQUE | NON_NULL, (empty)
                    // 855: b"--force\0" as ... st u8: typeof(_852 = move _853 as *const u8 (Pointer(ArrayToPointer))) = *const {l3489} u8
                    // 855: b"--force\0" as ... st u8:   l3489 = UNIQUE | NON_NULL, (empty)
                    // 855: b"--force\0" as ... _char: typeof(_851 = move _852 as *const i8 (Misc)) = *const {l3490} i8
                    // 855: b"--force\0" as ... _char:   l3490 = UNIQUE | NON_NULL, (empty)
                ) == 0
            {
                force = 1 as libc::c_int;
                // 858: force: typeof(_856) = *mut {l1455} i32
                // 858: force:   l1455 = UNIQUE | NON_NULL, (empty)
            } else if strcmp(
                *argv.offset(i as isize),
                // 860: *argv.offset(i  ... size): typeof(_860) = *const {l1460} i8
                // 860: *argv.offset(i  ... size):   l1460 = UNIQUE | NON_NULL, (empty)
                // 860: *argv.offset(i  ... size): typeof(_861) = *mut {l1462} i8
                // 860: *argv.offset(i  ... size):   l1462 = UNIQUE | NON_NULL, (empty)
                // 860: argv.offset(i a ... size): typeof(_862) = *mut {l1464} *mut {l1465} i8
                // 860: argv.offset(i a ... size):   l1464 = UNIQUE | NON_NULL, (empty)
                // 860: argv.offset(i a ... size):   l1465 = UNIQUE | NON_NULL, (empty)
                // 860: argv: typeof(_863) = *mut {l1467} *mut {l1468} i8
                // 860: argv:   l1467 = UNIQUE | NON_NULL, (empty)
                // 860: argv:   l1468 = UNIQUE | NON_NULL, (empty)
                // 860: *argv.offset(i  ... size): typeof(_860 = move _861 as *const i8 (Pointer(MutToConstPointer))) = *const {l3491} i8
                // 860: *argv.offset(i  ... size):   l3491 = UNIQUE | NON_NULL, (empty)
                b"-n\0" as *const u8 as *const libc::c_char,
                // 861: b"-n\0" as *con ... _char: typeof(_866) = *const {l1472} i8
                // 861: b"-n\0" as *con ... _char:   l1472 = UNIQUE | NON_NULL, (empty)
                // 861: b"-n\0" as *const u8: typeof(_867) = *const {l1474} u8
                // 861: b"-n\0" as *const u8:   l1474 = UNIQUE | NON_NULL, (empty)
                // 861: b"-n\0": typeof(_868) = *const {l1476} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 861: b"-n\0":   l1476 = UNIQUE | NON_NULL, (empty)
                // 861: b"-n\0": typeof(_869) = & {l1478} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 861: b"-n\0":   l1478 = UNIQUE | NON_NULL, FIXED
                // 861: b"-n\0": typeof(_868 = &raw const (*_869)) = *const {l3493} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 861: b"-n\0":   l3493 = UNIQUE | NON_NULL, (empty)
                // 861: b"-n\0": typeof(_869 = const b"-n\x00") = & {l3492} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 861: b"-n\0":   l3492 = UNIQUE | NON_NULL, (empty)
                // 861: b"-n\0" as *con ... _char: typeof(_866 = move _867 as *const i8 (Misc)) = *const {l3495} i8
                // 861: b"-n\0" as *con ... _char:   l3495 = UNIQUE | NON_NULL, (empty)
                // 861: b"-n\0" as *const u8: typeof(_867 = move _868 as *const u8 (Pointer(ArrayToPointer))) = *const {l3494} u8
                // 861: b"-n\0" as *const u8:   l3494 = UNIQUE | NON_NULL, (empty)
            ) == 0
                || strcmp(
                    *argv.offset(i as isize),
                    // 864: *argv.offset(i  ... size): typeof(_872) = *const {l1482} i8
                    // 864: *argv.offset(i  ... size):   l1482 = UNIQUE | NON_NULL, (empty)
                    // 864: *argv.offset(i  ... size): typeof(_873) = *mut {l1484} i8
                    // 864: *argv.offset(i  ... size):   l1484 = UNIQUE | NON_NULL, (empty)
                    // 864: argv.offset(i a ... size): typeof(_874) = *mut {l1486} *mut {l1487} i8
                    // 864: argv.offset(i a ... size):   l1486 = UNIQUE | NON_NULL, (empty)
                    // 864: argv.offset(i a ... size):   l1487 = UNIQUE | NON_NULL, (empty)
                    // 864: argv: typeof(_875) = *mut {l1489} *mut {l1490} i8
                    // 864: argv:   l1489 = UNIQUE | NON_NULL, (empty)
                    // 864: argv:   l1490 = UNIQUE | NON_NULL, (empty)
                    // 864: *argv.offset(i  ... size): typeof(_872 = move _873 as *const i8 (Pointer(MutToConstPointer))) = *const {l3496} i8
                    // 864: *argv.offset(i  ... size):   l3496 = UNIQUE | NON_NULL, (empty)
                    b"no-witness\0" as *const u8 as *const libc::c_char,
                    // 865: b"no-witness\0" ... _char: typeof(_878) = *const {l1494} i8
                    // 865: b"no-witness\0" ... _char:   l1494 = UNIQUE | NON_NULL, (empty)
                    // 865: b"no-witness\0" ... st u8: typeof(_879) = *const {l1496} u8
                    // 865: b"no-witness\0" ... st u8:   l1496 = UNIQUE | NON_NULL, (empty)
                    // 865: b"no-witness\0": typeof(_880) = *const {l1498} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 865: b"no-witness\0":   l1498 = UNIQUE | NON_NULL, (empty)
                    // 865: b"no-witness\0": typeof(_881) = & {l1500} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 865: b"no-witness\0":   l1500 = UNIQUE | NON_NULL, FIXED
                    // 865: b"no-witness\0" ... _char: typeof(_878 = move _879 as *const i8 (Misc)) = *const {l3500} i8
                    // 865: b"no-witness\0" ... _char:   l3500 = UNIQUE | NON_NULL, (empty)
                    // 865: b"no-witness\0" ... st u8: typeof(_879 = move _880 as *const u8 (Pointer(ArrayToPointer))) = *const {l3499} u8
                    // 865: b"no-witness\0" ... st u8:   l3499 = UNIQUE | NON_NULL, (empty)
                    // 865: b"no-witness\0": typeof(_881 = const b"no-witness\x00") = & {l3497} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 865: b"no-witness\0":   l3497 = UNIQUE | NON_NULL, (empty)
                    // 865: b"no-witness\0": typeof(_880 = &raw const (*_881)) = *const {l3498} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 865: b"no-witness\0":   l3498 = UNIQUE | NON_NULL, (empty)
                ) == 0
            {
                lglsetopt(
                    lgl,
                    // 869: lgl: typeof(_883) = *mut {l1503} LGL
                    // 869: lgl:   l1503 = UNIQUE | NON_NULL, (empty)
                    b"witness\0" as *const u8 as *const libc::c_char,
                    // 870: b"witness\0" as ... _char: typeof(_884) = *const {l1505} i8
                    // 870: b"witness\0" as ... _char:   l1505 = UNIQUE | NON_NULL, (empty)
                    // 870: b"witness\0" as ... st u8: typeof(_885) = *const {l1507} u8
                    // 870: b"witness\0" as ... st u8:   l1507 = UNIQUE | NON_NULL, (empty)
                    // 870: b"witness\0": typeof(_886) = *const {l1509} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 870: b"witness\0":   l1509 = UNIQUE | NON_NULL, (empty)
                    // 870: b"witness\0": typeof(_887) = & {l1511} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 870: b"witness\0":   l1511 = UNIQUE | NON_NULL, FIXED
                    // 870: b"witness\0" as ... st u8: typeof(_885 = move _886 as *const u8 (Pointer(ArrayToPointer))) = *const {l3503} u8
                    // 870: b"witness\0" as ... st u8:   l3503 = UNIQUE | NON_NULL, (empty)
                    // 870: b"witness\0": typeof(_886 = &raw const (*_887)) = *const {l3502} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 870: b"witness\0":   l3502 = UNIQUE | NON_NULL, (empty)
                    // 870: b"witness\0": typeof(_887 = const b"witness\x00") = & {l3501} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 870: b"witness\0":   l3501 = UNIQUE | NON_NULL, (empty)
                    // 870: b"witness\0" as ... _char: typeof(_884 = move _885 as *const i8 (Misc)) = *const {l3504} i8
                    // 870: b"witness\0" as ... _char:   l3504 = UNIQUE | NON_NULL, (empty)
                    0 as libc::c_int,
                );
            } else if strcmp(
                *argv.offset(i as isize),
                // 874: *argv.offset(i  ... size): typeof(_891) = *const {l1516} i8
                // 874: *argv.offset(i  ... size):   l1516 = UNIQUE | NON_NULL, (empty)
                // 874: *argv.offset(i  ... size): typeof(_892) = *mut {l1518} i8
                // 874: *argv.offset(i  ... size):   l1518 = UNIQUE | NON_NULL, (empty)
                // 874: argv.offset(i a ... size): typeof(_893) = *mut {l1520} *mut {l1521} i8
                // 874: argv.offset(i a ... size):   l1520 = UNIQUE | NON_NULL, (empty)
                // 874: argv.offset(i a ... size):   l1521 = UNIQUE | NON_NULL, (empty)
                // 874: argv: typeof(_894) = *mut {l1523} *mut {l1524} i8
                // 874: argv:   l1523 = UNIQUE | NON_NULL, (empty)
                // 874: argv:   l1524 = UNIQUE | NON_NULL, (empty)
                // 874: *argv.offset(i  ... size): typeof(_891 = move _892 as *const i8 (Pointer(MutToConstPointer))) = *const {l3505} i8
                // 874: *argv.offset(i  ... size):   l3505 = UNIQUE | NON_NULL, (empty)
                b"-c\0" as *const u8 as *const libc::c_char,
                // 875: b"-c\0" as *con ... _char: typeof(_897) = *const {l1528} i8
                // 875: b"-c\0" as *con ... _char:   l1528 = UNIQUE | NON_NULL, (empty)
                // 875: b"-c\0" as *const u8: typeof(_898) = *const {l1530} u8
                // 875: b"-c\0" as *const u8:   l1530 = UNIQUE | NON_NULL, (empty)
                // 875: b"-c\0": typeof(_899) = *const {l1532} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 875: b"-c\0":   l1532 = UNIQUE | NON_NULL, (empty)
                // 875: b"-c\0": typeof(_900) = & {l1534} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 875: b"-c\0":   l1534 = UNIQUE | NON_NULL, FIXED
                // 875: b"-c\0" as *con ... _char: typeof(_897 = move _898 as *const i8 (Misc)) = *const {l3509} i8
                // 875: b"-c\0" as *con ... _char:   l3509 = UNIQUE | NON_NULL, (empty)
                // 875: b"-c\0": typeof(_899 = &raw const (*_900)) = *const {l3507} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 875: b"-c\0":   l3507 = UNIQUE | NON_NULL, (empty)
                // 875: b"-c\0" as *const u8: typeof(_898 = move _899 as *const u8 (Pointer(ArrayToPointer))) = *const {l3508} u8
                // 875: b"-c\0" as *const u8:   l3508 = UNIQUE | NON_NULL, (empty)
                // 875: b"-c\0": typeof(_900 = const b"-c\x00") = & {l3506} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 875: b"-c\0":   l3506 = UNIQUE | NON_NULL, (empty)
            ) == 0
            {
                lglsetopt(
                    lgl,
                    // 879: lgl: typeof(_902) = *mut {l1537} LGL
                    // 879: lgl:   l1537 = UNIQUE | NON_NULL, (empty)
                    b"check\0" as *const u8 as *const libc::c_char,
                    // 880: b"check\0" as * ... _char: typeof(_903) = *const {l1539} i8
                    // 880: b"check\0" as * ... _char:   l1539 = UNIQUE | NON_NULL, (empty)
                    // 880: b"check\0" as * ... st u8: typeof(_904) = *const {l1541} u8
                    // 880: b"check\0" as * ... st u8:   l1541 = UNIQUE | NON_NULL, (empty)
                    // 880: b"check\0": typeof(_905) = *const {l1543} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 880: b"check\0":   l1543 = UNIQUE | NON_NULL, (empty)
                    // 880: b"check\0": typeof(_906) = & {l1545} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 880: b"check\0":   l1545 = UNIQUE | NON_NULL, FIXED
                    // 880: b"check\0" as * ... st u8: typeof(_904 = move _905 as *const u8 (Pointer(ArrayToPointer))) = *const {l3512} u8
                    // 880: b"check\0" as * ... st u8:   l3512 = UNIQUE | NON_NULL, (empty)
                    // 880: b"check\0" as * ... _char: typeof(_903 = move _904 as *const i8 (Misc)) = *const {l3513} i8
                    // 880: b"check\0" as * ... _char:   l3513 = UNIQUE | NON_NULL, (empty)
                    // 880: b"check\0": typeof(_905 = &raw const (*_906)) = *const {l3511} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 880: b"check\0":   l3511 = UNIQUE | NON_NULL, (empty)
                    // 880: b"check\0": typeof(_906 = const b"check\x00") = & {l3510} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 880: b"check\0":   l3510 = UNIQUE | NON_NULL, (empty)
                    lglgetopt(lgl, b"check\0" as *const u8 as *const libc::c_char)
                    // 881: lgl: typeof(_909) = *mut {l1549} LGL
                    // 881: lgl:   l1549 = UNIQUE | NON_NULL, (empty)
                    // 881: b"check\0" as * ... _char: typeof(_910) = *const {l1551} i8
                    // 881: b"check\0" as * ... _char:   l1551 = UNIQUE | NON_NULL, (empty)
                    // 881: b"check\0" as * ... st u8: typeof(_911) = *const {l1553} u8
                    // 881: b"check\0" as * ... st u8:   l1553 = UNIQUE | NON_NULL, (empty)
                    // 881: b"check\0": typeof(_912) = *const {l1555} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 881: b"check\0":   l1555 = UNIQUE | NON_NULL, (empty)
                    // 881: b"check\0": typeof(_913) = & {l1557} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 881: b"check\0":   l1557 = UNIQUE | NON_NULL, FIXED
                    // 881: b"check\0" as * ... st u8: typeof(_911 = move _912 as *const u8 (Pointer(ArrayToPointer))) = *const {l3516} u8
                    // 881: b"check\0" as * ... st u8:   l3516 = UNIQUE | NON_NULL, (empty)
                    // 881: b"check\0": typeof(_913 = const b"check\x00") = & {l3514} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 881: b"check\0":   l3514 = UNIQUE | NON_NULL, (empty)
                    // 881: b"check\0" as * ... _char: typeof(_910 = move _911 as *const i8 (Misc)) = *const {l3517} i8
                    // 881: b"check\0" as * ... _char:   l3517 = UNIQUE | NON_NULL, (empty)
                    // 881: b"check\0": typeof(_912 = &raw const (*_913)) = *const {l3515} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 881: b"check\0":   l3515 = UNIQUE | NON_NULL, (empty)
                        + 1 as libc::c_int,
                );
            } else if strcmp(
                *argv.offset(i as isize),
                // 885: *argv.offset(i  ... size): typeof(_918) = *const {l1563} i8
                // 885: *argv.offset(i  ... size):   l1563 = UNIQUE | NON_NULL, (empty)
                // 885: *argv.offset(i  ... size): typeof(_919) = *mut {l1565} i8
                // 885: *argv.offset(i  ... size):   l1565 = UNIQUE | NON_NULL, (empty)
                // 885: argv.offset(i a ... size): typeof(_920) = *mut {l1567} *mut {l1568} i8
                // 885: argv.offset(i a ... size):   l1567 = UNIQUE | NON_NULL, (empty)
                // 885: argv.offset(i a ... size):   l1568 = UNIQUE | NON_NULL, (empty)
                // 885: argv: typeof(_921) = *mut {l1570} *mut {l1571} i8
                // 885: argv:   l1570 = UNIQUE | NON_NULL, (empty)
                // 885: argv:   l1571 = UNIQUE | NON_NULL, (empty)
                // 885: *argv.offset(i  ... size): typeof(_918 = move _919 as *const i8 (Pointer(MutToConstPointer))) = *const {l3518} i8
                // 885: *argv.offset(i  ... size):   l3518 = UNIQUE | NON_NULL, (empty)
                b"-l\0" as *const u8 as *const libc::c_char,
                // 886: b"-l\0" as *con ... _char: typeof(_924) = *const {l1575} i8
                // 886: b"-l\0" as *con ... _char:   l1575 = UNIQUE | NON_NULL, (empty)
                // 886: b"-l\0" as *const u8: typeof(_925) = *const {l1577} u8
                // 886: b"-l\0" as *const u8:   l1577 = UNIQUE | NON_NULL, (empty)
                // 886: b"-l\0": typeof(_926) = *const {l1579} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 886: b"-l\0":   l1579 = UNIQUE | NON_NULL, (empty)
                // 886: b"-l\0": typeof(_927) = & {l1581} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 886: b"-l\0":   l1581 = UNIQUE | NON_NULL, FIXED
                // 886: b"-l\0" as *const u8: typeof(_925 = move _926 as *const u8 (Pointer(ArrayToPointer))) = *const {l3521} u8
                // 886: b"-l\0" as *const u8:   l3521 = UNIQUE | NON_NULL, (empty)
                // 886: b"-l\0": typeof(_927 = const b"-l\x00") = & {l3519} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 886: b"-l\0":   l3519 = UNIQUE | NON_NULL, (empty)
                // 886: b"-l\0" as *con ... _char: typeof(_924 = move _925 as *const i8 (Misc)) = *const {l3522} i8
                // 886: b"-l\0" as *con ... _char:   l3522 = UNIQUE | NON_NULL, (empty)
                // 886: b"-l\0": typeof(_926 = &raw const (*_927)) = *const {l3520} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 886: b"-l\0":   l3520 = UNIQUE | NON_NULL, (empty)
            ) == 0
            {
                lglsetopt(
                    lgl,
                    // 890: lgl: typeof(_929) = *mut {l1584} LGL
                    // 890: lgl:   l1584 = UNIQUE | NON_NULL, (empty)
                    b"log\0" as *const u8 as *const libc::c_char,
                    // 891: b"log\0" as *co ... _char: typeof(_930) = *const {l1586} i8
                    // 891: b"log\0" as *co ... _char:   l1586 = UNIQUE | NON_NULL, (empty)
                    // 891: b"log\0" as *co ... st u8: typeof(_931) = *const {l1588} u8
                    // 891: b"log\0" as *co ... st u8:   l1588 = UNIQUE | NON_NULL, (empty)
                    // 891: b"log\0": typeof(_932) = *const {l1590} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 891: b"log\0":   l1590 = UNIQUE | NON_NULL, (empty)
                    // 891: b"log\0": typeof(_933) = & {l1592} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 891: b"log\0":   l1592 = UNIQUE | NON_NULL, FIXED
                    // 891: b"log\0": typeof(_933 = const b"log\x00") = & {l3523} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 891: b"log\0":   l3523 = UNIQUE | NON_NULL, (empty)
                    // 891: b"log\0" as *co ... _char: typeof(_930 = move _931 as *const i8 (Misc)) = *const {l3526} i8
                    // 891: b"log\0" as *co ... _char:   l3526 = UNIQUE | NON_NULL, (empty)
                    // 891: b"log\0" as *co ... st u8: typeof(_931 = move _932 as *const u8 (Pointer(ArrayToPointer))) = *const {l3525} u8
                    // 891: b"log\0" as *co ... st u8:   l3525 = UNIQUE | NON_NULL, (empty)
                    // 891: b"log\0": typeof(_932 = &raw const (*_933)) = *const {l3524} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 891: b"log\0":   l3524 = UNIQUE | NON_NULL, (empty)
                    lglgetopt(lgl, b"log\0" as *const u8 as *const libc::c_char) + 1 as libc::c_int,
                    // 892: lgl: typeof(_936) = *mut {l1596} LGL
                    // 892: lgl:   l1596 = UNIQUE | NON_NULL, (empty)
                    // 892: b"log\0" as *co ... _char: typeof(_937) = *const {l1598} i8
                    // 892: b"log\0" as *co ... _char:   l1598 = UNIQUE | NON_NULL, (empty)
                    // 892: b"log\0" as *co ... st u8: typeof(_938) = *const {l1600} u8
                    // 892: b"log\0" as *co ... st u8:   l1600 = UNIQUE | NON_NULL, (empty)
                    // 892: b"log\0": typeof(_939) = *const {l1602} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 892: b"log\0":   l1602 = UNIQUE | NON_NULL, (empty)
                    // 892: b"log\0": typeof(_940) = & {l1604} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 892: b"log\0":   l1604 = UNIQUE | NON_NULL, FIXED
                    // 892: b"log\0" as *co ... _char: typeof(_937 = move _938 as *const i8 (Misc)) = *const {l3530} i8
                    // 892: b"log\0" as *co ... _char:   l3530 = UNIQUE | NON_NULL, (empty)
                    // 892: b"log\0": typeof(_940 = const b"log\x00") = & {l3527} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 892: b"log\0":   l3527 = UNIQUE | NON_NULL, (empty)
                    // 892: b"log\0": typeof(_939 = &raw const (*_940)) = *const {l3528} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 892: b"log\0":   l3528 = UNIQUE | NON_NULL, (empty)
                    // 892: b"log\0" as *co ... st u8: typeof(_938 = move _939 as *const u8 (Pointer(ArrayToPointer))) = *const {l3529} u8
                    // 892: b"log\0" as *co ... st u8:   l3529 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(
                *argv.offset(i as isize),
                // 895: *argv.offset(i  ... size): typeof(_945) = *const {l1610} i8
                // 895: *argv.offset(i  ... size):   l1610 = UNIQUE | NON_NULL, (empty)
                // 895: *argv.offset(i  ... size): typeof(_946) = *mut {l1612} i8
                // 895: *argv.offset(i  ... size):   l1612 = UNIQUE | NON_NULL, (empty)
                // 895: argv.offset(i a ... size): typeof(_947) = *mut {l1614} *mut {l1615} i8
                // 895: argv.offset(i a ... size):   l1614 = UNIQUE | NON_NULL, (empty)
                // 895: argv.offset(i a ... size):   l1615 = UNIQUE | NON_NULL, (empty)
                // 895: argv: typeof(_948) = *mut {l1617} *mut {l1618} i8
                // 895: argv:   l1617 = UNIQUE | NON_NULL, (empty)
                // 895: argv:   l1618 = UNIQUE | NON_NULL, (empty)
                // 895: *argv.offset(i  ... size): typeof(_945 = move _946 as *const i8 (Pointer(MutToConstPointer))) = *const {l3531} i8
                // 895: *argv.offset(i  ... size):   l3531 = UNIQUE | NON_NULL, (empty)
                b"-v\0" as *const u8 as *const libc::c_char,
                // 896: b"-v\0" as *con ... _char: typeof(_951) = *const {l1622} i8
                // 896: b"-v\0" as *con ... _char:   l1622 = UNIQUE | NON_NULL, (empty)
                // 896: b"-v\0" as *const u8: typeof(_952) = *const {l1624} u8
                // 896: b"-v\0" as *const u8:   l1624 = UNIQUE | NON_NULL, (empty)
                // 896: b"-v\0": typeof(_953) = *const {l1626} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 896: b"-v\0":   l1626 = UNIQUE | NON_NULL, (empty)
                // 896: b"-v\0": typeof(_954) = & {l1628} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 896: b"-v\0":   l1628 = UNIQUE | NON_NULL, FIXED
                // 896: b"-v\0" as *const u8: typeof(_952 = move _953 as *const u8 (Pointer(ArrayToPointer))) = *const {l3534} u8
                // 896: b"-v\0" as *const u8:   l3534 = UNIQUE | NON_NULL, (empty)
                // 896: b"-v\0": typeof(_954 = const b"-v\x00") = & {l3532} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 896: b"-v\0":   l3532 = UNIQUE | NON_NULL, (empty)
                // 896: b"-v\0" as *con ... _char: typeof(_951 = move _952 as *const i8 (Misc)) = *const {l3535} i8
                // 896: b"-v\0" as *con ... _char:   l3535 = UNIQUE | NON_NULL, (empty)
                // 896: b"-v\0": typeof(_953 = &raw const (*_954)) = *const {l3533} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 896: b"-v\0":   l3533 = UNIQUE | NON_NULL, (empty)
            ) == 0
            {
                lglsetopt(
                    lgl,
                    // 900: lgl: typeof(_956) = *mut {l1631} LGL
                    // 900: lgl:   l1631 = UNIQUE | NON_NULL, (empty)
                    b"verbose\0" as *const u8 as *const libc::c_char,
                    // 901: b"verbose\0" as ... _char: typeof(_957) = *const {l1633} i8
                    // 901: b"verbose\0" as ... _char:   l1633 = UNIQUE | NON_NULL, (empty)
                    // 901: b"verbose\0" as ... st u8: typeof(_958) = *const {l1635} u8
                    // 901: b"verbose\0" as ... st u8:   l1635 = UNIQUE | NON_NULL, (empty)
                    // 901: b"verbose\0": typeof(_959) = *const {l1637} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 901: b"verbose\0":   l1637 = UNIQUE | NON_NULL, (empty)
                    // 901: b"verbose\0": typeof(_960) = & {l1639} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 901: b"verbose\0":   l1639 = UNIQUE | NON_NULL, FIXED
                    // 901: b"verbose\0" as ... _char: typeof(_957 = move _958 as *const i8 (Misc)) = *const {l3539} i8
                    // 901: b"verbose\0" as ... _char:   l3539 = UNIQUE | NON_NULL, (empty)
                    // 901: b"verbose\0": typeof(_959 = &raw const (*_960)) = *const {l3537} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 901: b"verbose\0":   l3537 = UNIQUE | NON_NULL, (empty)
                    // 901: b"verbose\0": typeof(_960 = const b"verbose\x00") = & {l3536} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 901: b"verbose\0":   l3536 = UNIQUE | NON_NULL, (empty)
                    // 901: b"verbose\0" as ... st u8: typeof(_958 = move _959 as *const u8 (Pointer(ArrayToPointer))) = *const {l3538} u8
                    // 901: b"verbose\0" as ... st u8:   l3538 = UNIQUE | NON_NULL, (empty)
                    lglgetopt(lgl, b"verbose\0" as *const u8 as *const libc::c_char)
                    // 902: lgl: typeof(_963) = *mut {l1643} LGL
                    // 902: lgl:   l1643 = UNIQUE | NON_NULL, (empty)
                    // 902: b"verbose\0" as ... _char: typeof(_964) = *const {l1645} i8
                    // 902: b"verbose\0" as ... _char:   l1645 = UNIQUE | NON_NULL, (empty)
                    // 902: b"verbose\0" as ... st u8: typeof(_965) = *const {l1647} u8
                    // 902: b"verbose\0" as ... st u8:   l1647 = UNIQUE | NON_NULL, (empty)
                    // 902: b"verbose\0": typeof(_966) = *const {l1649} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 902: b"verbose\0":   l1649 = UNIQUE | NON_NULL, (empty)
                    // 902: b"verbose\0": typeof(_967) = & {l1651} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 902: b"verbose\0":   l1651 = UNIQUE | NON_NULL, FIXED
                    // 902: b"verbose\0" as ... _char: typeof(_964 = move _965 as *const i8 (Misc)) = *const {l3543} i8
                    // 902: b"verbose\0" as ... _char:   l3543 = UNIQUE | NON_NULL, (empty)
                    // 902: b"verbose\0": typeof(_967 = const b"verbose\x00") = & {l3540} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 902: b"verbose\0":   l3540 = UNIQUE | NON_NULL, (empty)
                    // 902: b"verbose\0": typeof(_966 = &raw const (*_967)) = *const {l3541} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 902: b"verbose\0":   l3541 = UNIQUE | NON_NULL, (empty)
                    // 902: b"verbose\0" as ... st u8: typeof(_965 = move _966 as *const u8 (Pointer(ArrayToPointer))) = *const {l3542} u8
                    // 902: b"verbose\0" as ... st u8:   l3542 = UNIQUE | NON_NULL, (empty)
                        + 1 as libc::c_int,
                );
            } else if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            // 905: (*argv.offset(i ... size): typeof(_973) = *mut {l1658} i8
            // 905: (*argv.offset(i ... size):   l1658 = UNIQUE | NON_NULL, (empty)
            // 905: (*argv.offset(i ... ize)): typeof(_974) = *mut {l1660} i8
            // 905: (*argv.offset(i ... ize)):   l1660 = UNIQUE | NON_NULL, (empty)
            // 905: argv.offset(i a ... size): typeof(_975) = *mut {l1662} *mut {l1663} i8
            // 905: argv.offset(i a ... size):   l1662 = UNIQUE | NON_NULL, (empty)
            // 905: argv.offset(i a ... size):   l1663 = UNIQUE | NON_NULL, (empty)
            // 905: argv: typeof(_976) = *mut {l1665} *mut {l1666} i8
            // 905: argv:   l1665 = UNIQUE | NON_NULL, (empty)
            // 905: argv:   l1666 = UNIQUE | NON_NULL, (empty)
                == '-' as i32
            {
                if *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize) as libc::c_int
                // 908: (*argv.offset(i ... size): typeof(_986) = *mut {l1677} i8
                // 908: (*argv.offset(i ... size):   l1677 = UNIQUE | NON_NULL, (empty)
                // 908: (*argv.offset(i ... ize)): typeof(_987) = *mut {l1679} i8
                // 908: (*argv.offset(i ... ize)):   l1679 = UNIQUE | NON_NULL, (empty)
                // 908: argv.offset(i a ... size): typeof(_988) = *mut {l1681} *mut {l1682} i8
                // 908: argv.offset(i a ... size):   l1681 = UNIQUE | NON_NULL, (empty)
                // 908: argv.offset(i a ... size):   l1682 = UNIQUE | NON_NULL, (empty)
                // 908: argv: typeof(_989) = *mut {l1684} *mut {l1685} i8
                // 908: argv:   l1684 = UNIQUE | NON_NULL, (empty)
                // 908: argv:   l1685 = UNIQUE | NON_NULL, (empty)
                    == '-' as i32
                {
                    match_0 = strchr(
                    // 911: strchr( (*argv. ... 32, ): typeof(_995) = *mut {l1692} i8
                    // 911: strchr( (*argv. ... 32, ):   l1692 = UNIQUE | NON_NULL, (empty)
                    // 911: match_0 = strch ... 32, ): typeof(_18 = move _995 as *const i8 (Pointer(MutToConstPointer))) = *const {l3545} i8
                    // 911: match_0 = strch ... 32, ):   l3545 = UNIQUE | NON_NULL, (empty)
                        (*argv.offset(i as isize)).offset(2 as libc::c_int as isize),
                        // 912: (*argv.offset(i ... size): typeof(_996) = *const {l1694} i8
                        // 912: (*argv.offset(i ... size):   l1694 = UNIQUE | NON_NULL, (empty)
                        // 912: (*argv.offset(i ... size): typeof(_997) = *mut {l1696} i8
                        // 912: (*argv.offset(i ... size):   l1696 = UNIQUE | NON_NULL, (empty)
                        // 912: (*argv.offset(i ... ize)): typeof(_998) = *mut {l1698} i8
                        // 912: (*argv.offset(i ... ize)):   l1698 = UNIQUE | NON_NULL, (empty)
                        // 912: argv.offset(i a ... size): typeof(_999) = *mut {l1700} *mut {l1701} i8
                        // 912: argv.offset(i a ... size):   l1700 = UNIQUE | NON_NULL, (empty)
                        // 912: argv.offset(i a ... size):   l1701 = UNIQUE | NON_NULL, (empty)
                        // 912: argv: typeof(_1000) = *mut {l1703} *mut {l1704} i8
                        // 912: argv:   l1703 = UNIQUE | NON_NULL, (empty)
                        // 912: argv:   l1704 = UNIQUE | NON_NULL, (empty)
                        // 912: (*argv.offset(i ... size): typeof(_996 = move _997 as *const i8 (Pointer(MutToConstPointer))) = *const {l3544} i8
                        // 912: (*argv.offset(i ... size):   l3544 = UNIQUE | NON_NULL, (empty)
                        '=' as i32,
                    );
                    if !match_0.is_null() {
                    // 915: match_0: typeof(_1009) = *const {l1714} i8
                    // 915: match_0:   l1714 = UNIQUE | NON_NULL, (empty)
                        p = match_0.offset(1 as libc::c_int as isize);
                        // 916: match_0.offset( ... size): typeof(_1010) = *const {l1716} i8
                        // 916: match_0.offset( ... size):   l1716 = UNIQUE | NON_NULL, (empty)
                        // 916: match_0: typeof(_1011) = *const {l1718} i8
                        // 916: match_0:   l1718 = UNIQUE | NON_NULL, (empty)
                        if *p as libc::c_int == '-' as i32 {
                            p = p.offset(1);
                            // 918: p.offset(1): typeof(_1019) = *const {l1727} i8
                            // 918: p.offset(1):   l1727 = UNIQUE | NON_NULL, (empty)
                            // 918: p: typeof(_1020) = *const {l1729} i8
                            // 918: p:   l1729 = UNIQUE | NON_NULL, (empty)
                            p;
                            // 919: p: typeof(_1021) = *const {l1731} i8
                            // 919: p:   l1731 = UNIQUE | NON_NULL, (empty)
                        }
                        len =
                            p.offset_from(*argv.offset(i as isize)) as libc::c_long as libc::c_int;
                            // 922: p: typeof(_1024) = *const {l1735} i8
                            // 922: p:   l1735 = UNIQUE | NON_NULL, (empty)
                            // 922: *argv.offset(i  ... size): typeof(_1025) = *const {l1737} i8
                            // 922: *argv.offset(i  ... size):   l1737 = UNIQUE | NON_NULL, (empty)
                            // 922: *argv.offset(i  ... size): typeof(_1026) = *mut {l1739} i8
                            // 922: *argv.offset(i  ... size):   l1739 = UNIQUE | NON_NULL, (empty)
                            // 922: argv.offset(i a ... size): typeof(_1027) = *mut {l1741} *mut {l1742} i8
                            // 922: argv.offset(i a ... size):   l1741 = UNIQUE | NON_NULL, (empty)
                            // 922: argv.offset(i a ... size):   l1742 = UNIQUE | NON_NULL, (empty)
                            // 922: argv: typeof(_1028) = *mut {l1744} *mut {l1745} i8
                            // 922: argv:   l1744 = UNIQUE | NON_NULL, (empty)
                            // 922: argv:   l1745 = UNIQUE | NON_NULL, (empty)
                            // 922: *argv.offset(i  ... size): typeof(_1025 = move _1026 as *const i8 (Pointer(MutToConstPointer))) = *const {l3546} i8
                            // 922: *argv.offset(i  ... size):   l3546 = UNIQUE | NON_NULL, (empty)
                        if strncmp(
                            *argv.offset(i as isize),
                            // 924: *argv.offset(i  ... size): typeof(_1033) = *const {l1751} i8
                            // 924: *argv.offset(i  ... size):   l1751 = UNIQUE | NON_NULL, (empty)
                            // 924: *argv.offset(i  ... size): typeof(_1034) = *mut {l1753} i8
                            // 924: *argv.offset(i  ... size):   l1753 = UNIQUE | NON_NULL, (empty)
                            // 924: argv.offset(i a ... size): typeof(_1035) = *mut {l1755} *mut {l1756} i8
                            // 924: argv.offset(i a ... size):   l1755 = UNIQUE | NON_NULL, (empty)
                            // 924: argv.offset(i a ... size):   l1756 = UNIQUE | NON_NULL, (empty)
                            // 924: argv: typeof(_1036) = *mut {l1758} *mut {l1759} i8
                            // 924: argv:   l1758 = UNIQUE | NON_NULL, (empty)
                            // 924: argv:   l1759 = UNIQUE | NON_NULL, (empty)
                            // 924: *argv.offset(i  ... size): typeof(_1033 = move _1034 as *const i8 (Pointer(MutToConstPointer))) = *const {l3547} i8
                            // 924: *argv.offset(i  ... size):   l3547 = UNIQUE | NON_NULL, (empty)
                            b"--write-api-trace=\0" as *const u8 as *const libc::c_char,
                            // 925: b"--write-api-t ... _char: typeof(_1039) = *const {l1763} i8
                            // 925: b"--write-api-t ... _char:   l1763 = UNIQUE | NON_NULL, (empty)
                            // 925: b"--write-api-t ... st u8: typeof(_1040) = *const {l1765} u8
                            // 925: b"--write-api-t ... st u8:   l1765 = UNIQUE | NON_NULL, (empty)
                            // 925: b"--write-api-t ... e=\0": typeof(_1041) = *const {l1767} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                            // 925: b"--write-api-t ... e=\0":   l1767 = UNIQUE | NON_NULL, (empty)
                            // 925: b"--write-api-t ... e=\0": typeof(_1042) = & {l1769} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                            // 925: b"--write-api-t ... e=\0":   l1769 = UNIQUE | NON_NULL, FIXED
                            // 925: b"--write-api-t ... st u8: typeof(_1040 = move _1041 as *const u8 (Pointer(ArrayToPointer))) = *const {l3550} u8
                            // 925: b"--write-api-t ... st u8:   l3550 = UNIQUE | NON_NULL, (empty)
                            // 925: b"--write-api-t ... e=\0": typeof(_1042 = const b"--write-api-trace=\x00") = & {l3548} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                            // 925: b"--write-api-t ... e=\0":   l3548 = UNIQUE | NON_NULL, (empty)
                            // 925: b"--write-api-t ... _char: typeof(_1039 = move _1040 as *const i8 (Misc)) = *const {l3551} i8
                            // 925: b"--write-api-t ... _char:   l3551 = UNIQUE | NON_NULL, (empty)
                            // 925: b"--write-api-t ... e=\0": typeof(_1041 = &raw const (*_1042)) = *const {l3549} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                            // 925: b"--write-api-t ... e=\0":   l3549 = UNIQUE | NON_NULL, (empty)
                            len as libc::c_ulong,
                        ) == 0
                        {
                            current_block = 7351195479953500246;
                        } else if strncmp(
                            *argv.offset(i as isize),
                            // 931: *argv.offset(i  ... size): typeof(_1047) = *const {l1775} i8
                            // 931: *argv.offset(i  ... size):   l1775 = UNIQUE | NON_NULL, (empty)
                            // 931: *argv.offset(i  ... size): typeof(_1048) = *mut {l1777} i8
                            // 931: *argv.offset(i  ... size):   l1777 = UNIQUE | NON_NULL, (empty)
                            // 931: argv.offset(i a ... size): typeof(_1049) = *mut {l1779} *mut {l1780} i8
                            // 931: argv.offset(i a ... size):   l1779 = UNIQUE | NON_NULL, (empty)
                            // 931: argv.offset(i a ... size):   l1780 = UNIQUE | NON_NULL, (empty)
                            // 931: argv: typeof(_1050) = *mut {l1782} *mut {l1783} i8
                            // 931: argv:   l1782 = UNIQUE | NON_NULL, (empty)
                            // 931: argv:   l1783 = UNIQUE | NON_NULL, (empty)
                            // 931: *argv.offset(i  ... size): typeof(_1047 = move _1048 as *const i8 (Pointer(MutToConstPointer))) = *const {l3552} i8
                            // 931: *argv.offset(i  ... size):   l3552 = UNIQUE | NON_NULL, (empty)
                            b"--thanks=\0" as *const u8 as *const libc::c_char,
                            // 932: b"--thanks=\0"  ... _char: typeof(_1053) = *const {l1787} i8
                            // 932: b"--thanks=\0"  ... _char:   l1787 = UNIQUE | NON_NULL, (empty)
                            // 932: b"--thanks=\0"  ... st u8: typeof(_1054) = *const {l1789} u8
                            // 932: b"--thanks=\0"  ... st u8:   l1789 = UNIQUE | NON_NULL, (empty)
                            // 932: b"--thanks=\0": typeof(_1055) = *const {l1791} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                            // 932: b"--thanks=\0":   l1791 = UNIQUE | NON_NULL, (empty)
                            // 932: b"--thanks=\0": typeof(_1056) = & {l1793} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                            // 932: b"--thanks=\0":   l1793 = UNIQUE | NON_NULL, FIXED
                            // 932: b"--thanks=\0": typeof(_1056 = const b"--thanks=\x00") = & {l3553} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                            // 932: b"--thanks=\0":   l3553 = UNIQUE | NON_NULL, (empty)
                            // 932: b"--thanks=\0": typeof(_1055 = &raw const (*_1056)) = *const {l3554} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                            // 932: b"--thanks=\0":   l3554 = UNIQUE | NON_NULL, (empty)
                            // 932: b"--thanks=\0"  ... _char: typeof(_1053 = move _1054 as *const i8 (Misc)) = *const {l3556} i8
                            // 932: b"--thanks=\0"  ... _char:   l3556 = UNIQUE | NON_NULL, (empty)
                            // 932: b"--thanks=\0"  ... st u8: typeof(_1054 = move _1055 as *const u8 (Pointer(ArrayToPointer))) = *const {l3555} u8
                            // 932: b"--thanks=\0"  ... st u8:   l3555 = UNIQUE | NON_NULL, (empty)
                            len as libc::c_ulong,
                        ) == 0
                        {
                            thanks = match_0.offset(1 as libc::c_int as isize);
                            // 936: match_0.offset( ... size): typeof(_1059) = *const {l1797} i8
                            // 936: match_0.offset( ... size):   l1797 = UNIQUE | NON_NULL, (empty)
                            // 936: match_0: typeof(_1060) = *const {l1799} i8
                            // 936: match_0:   l1799 = UNIQUE | NON_NULL, (empty)
                            current_block = 7351195479953500246;
                        } else if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize)
                        // 938: (*__ctype_b_loc ... size): typeof(_1067) = *const {l1807} u16
                        // 938: (*__ctype_b_loc ... size):   l1807 = UNIQUE | NON_NULL, (empty)
                        // 938: (*__ctype_b_loc()): typeof(_1068) = *const {l1809} u16
                        // 938: (*__ctype_b_loc()):   l1809 = UNIQUE | NON_NULL, (empty)
                        // 938: __ctype_b_loc(): typeof(_1069) = *mut {l1811} *const {l1812} u16
                        // 938: __ctype_b_loc():   l1811 = UNIQUE | NON_NULL, (empty)
                        // 938: __ctype_b_loc():   l1812 = UNIQUE | NON_NULL, (empty)
                            as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            == 0
                        {
                            current_block = 15067877082424188916;
                        } else {
                            loop {
                                p = p.offset(1);
                                // 946: p.offset(1): typeof(_1077) = *const {l1821} i8
                                // 946: p.offset(1):   l1821 = UNIQUE | NON_NULL, (empty)
                                // 946: p: typeof(_1078) = *const {l1823} i8
                                // 946: p:   l1823 = UNIQUE | NON_NULL, (empty)
                                if !(*p != 0) {
                                    current_block = 2956972668325154207;
                                    break;
                                }
                                if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize)
                                // 951: (*__ctype_b_loc ... size): typeof(_1088) = *const {l1834} u16
                                // 951: (*__ctype_b_loc ... size):   l1834 = UNIQUE | NON_NULL, (empty)
                                // 951: (*__ctype_b_loc()): typeof(_1089) = *const {l1836} u16
                                // 951: (*__ctype_b_loc()):   l1836 = UNIQUE | NON_NULL, (empty)
                                // 951: __ctype_b_loc(): typeof(_1090) = *mut {l1838} *const {l1839} u16
                                // 951: __ctype_b_loc():   l1838 = UNIQUE | NON_NULL, (empty)
                                // 951: __ctype_b_loc():   l1839 = UNIQUE | NON_NULL, (empty)
                                    as libc::c_int
                                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    == 0
                                {
                                    current_block = 15067877082424188916;
                                    break;
                                }
                            }
                            match current_block {
                                15067877082424188916 => {}
                                _ => {
                                    len = (match_0.offset_from(*argv.offset(i as isize))
                                    // 963: match_0: typeof(_1101) = *const {l1851} i8
                                    // 963: match_0:   l1851 = UNIQUE | NON_NULL, (empty)
                                    // 963: *argv.offset(i  ... size): typeof(_1102) = *const {l1853} i8
                                    // 963: *argv.offset(i  ... size):   l1853 = UNIQUE | NON_NULL, (empty)
                                    // 963: *argv.offset(i  ... size): typeof(_1103) = *mut {l1855} i8
                                    // 963: *argv.offset(i  ... size):   l1855 = UNIQUE | NON_NULL, (empty)
                                    // 963: argv.offset(i a ... size): typeof(_1104) = *mut {l1857} *mut {l1858} i8
                                    // 963: argv.offset(i a ... size):   l1857 = UNIQUE | NON_NULL, (empty)
                                    // 963: argv.offset(i a ... size):   l1858 = UNIQUE | NON_NULL, (empty)
                                    // 963: argv: typeof(_1105) = *mut {l1860} *mut {l1861} i8
                                    // 963: argv:   l1860 = UNIQUE | NON_NULL, (empty)
                                    // 963: argv:   l1861 = UNIQUE | NON_NULL, (empty)
                                    // 963: *argv.offset(i  ... size): typeof(_1102 = move _1103 as *const i8 (Pointer(MutToConstPointer))) = *const {l3557} i8
                                    // 963: *argv.offset(i  ... size):   l3557 = UNIQUE | NON_NULL, (empty)
                                        as libc::c_long
                                        - 2 as libc::c_int as libc::c_long)
                                        as libc::c_int;
                                    tmp = malloc((len + 1 as libc::c_int) as libc::c_ulong)
                                    // 967: malloc((len + 1 ... long): typeof(_1111) = *mut {l1868} DefId(2:5583 ~ core[480a]::ffi::c_void)
                                    // 967: malloc((len + 1 ... long):   l1868 = UNIQUE | NON_NULL, (empty)
                                    // 967: tmp = malloc((l ... _char: typeof(_28 = move _1111 as *mut i8 (Misc)) = *mut {l3558} i8
                                    // 967: tmp = malloc((l ... _char:   l3558 = UNIQUE | NON_NULL, (empty)
                                        as *mut libc::c_char;
                                    j = 0 as libc::c_int;
                                    p = (*argv.offset(i as isize))
                                    // 970: (*argv.offset(i ... size): typeof(_1118) = *mut {l1876} i8
                                    // 970: (*argv.offset(i ... size):   l1876 = UNIQUE | NON_NULL, (empty)
                                    // 970: (*argv.offset(i ... ize)): typeof(_1119) = *mut {l1878} i8
                                    // 970: (*argv.offset(i ... ize)):   l1878 = UNIQUE | NON_NULL, (empty)
                                    // 970: argv.offset(i a ... size): typeof(_1120) = *mut {l1880} *mut {l1881} i8
                                    // 970: argv.offset(i a ... size):   l1880 = UNIQUE | NON_NULL, (empty)
                                    // 970: argv.offset(i a ... size):   l1881 = UNIQUE | NON_NULL, (empty)
                                    // 970: argv: typeof(_1121) = *mut {l1883} *mut {l1884} i8
                                    // 970: argv:   l1883 = UNIQUE | NON_NULL, (empty)
                                    // 970: argv:   l1884 = UNIQUE | NON_NULL, (empty)
                                    // 970: p = (*argv.offs ... size): typeof(_19 = move _1118 as *const i8 (Pointer(MutToConstPointer))) = *const {l3559} i8
                                    // 970: p = (*argv.offs ... size):   l3559 = UNIQUE | NON_NULL, (empty)
                                        .offset(2 as libc::c_int as isize);
                                    while *p as libc::c_int != '=' as i32 {
                                        let fresh3 = j;
                                        j = j + 1;
                                        *tmp.offset(fresh3 as isize) = *p;
                                        // 975: tmp.offset(fres ... size): typeof(_1135) = *mut {l1899} i8
                                        // 975: tmp.offset(fres ... size):   l1899 = UNIQUE | NON_NULL, (empty)
                                        // 975: tmp: typeof(_1136) = *mut {l1901} i8
                                        // 975: tmp:   l1901 = UNIQUE | NON_NULL, (empty)
                                        p = p.offset(1);
                                        // 976: p.offset(1): typeof(_1139) = *const {l1905} i8
                                        // 976: p.offset(1):   l1905 = UNIQUE | NON_NULL, (empty)
                                        // 976: p: typeof(_1140) = *const {l1907} i8
                                        // 976: p:   l1907 = UNIQUE | NON_NULL, (empty)
                                        p;
                                        // 977: p: typeof(_1141) = *const {l1909} i8
                                        // 977: p:   l1909 = UNIQUE | NON_NULL, (empty)
                                    }
                                    *tmp.offset(j as isize) = 0 as libc::c_int as libc::c_char;
                                    // 979: tmp.offset(j as ... size): typeof(_1146) = *mut {l1915} i8
                                    // 979: tmp.offset(j as ... size):   l1915 = UNIQUE | NON_NULL, (empty)
                                    // 979: tmp: typeof(_1147) = *mut {l1917} i8
                                    // 979: tmp:   l1917 = UNIQUE | NON_NULL, (empty)
                                    val = atoi(match_0.offset(1 as libc::c_int as isize));
                                    // 980: match_0.offset( ... size): typeof(_1151) = *const {l1922} i8
                                    // 980: match_0.offset( ... size):   l1922 = UNIQUE | NON_NULL, (empty)
                                    // 980: match_0: typeof(_1152) = *const {l1924} i8
                                    // 980: match_0:   l1924 = UNIQUE | NON_NULL, (empty)
                                    current_block = 6988365858197790817;
                                }
                            }
                        }
                    } else {
                        if strncmp(
                            *argv.offset(i as isize),
                            // 987: *argv.offset(i  ... size): typeof(_1158) = *const {l1931} i8
                            // 987: *argv.offset(i  ... size):   l1931 = UNIQUE | NON_NULL, (empty)
                            // 987: *argv.offset(i  ... size): typeof(_1159) = *mut {l1933} i8
                            // 987: *argv.offset(i  ... size):   l1933 = UNIQUE | NON_NULL, (empty)
                            // 987: argv.offset(i a ... size): typeof(_1160) = *mut {l1935} *mut {l1936} i8
                            // 987: argv.offset(i a ... size):   l1935 = UNIQUE | NON_NULL, (empty)
                            // 987: argv.offset(i a ... size):   l1936 = UNIQUE | NON_NULL, (empty)
                            // 987: argv: typeof(_1161) = *mut {l1938} *mut {l1939} i8
                            // 987: argv:   l1938 = UNIQUE | NON_NULL, (empty)
                            // 987: argv:   l1939 = UNIQUE | NON_NULL, (empty)
                            // 987: *argv.offset(i  ... size): typeof(_1158 = move _1159 as *const i8 (Pointer(MutToConstPointer))) = *const {l3560} i8
                            // 987: *argv.offset(i  ... size):   l3560 = UNIQUE | NON_NULL, (empty)
                            b"--no-\0" as *const u8 as *const libc::c_char,
                            // 988: b"--no-\0" as * ... _char: typeof(_1164) = *const {l1943} i8
                            // 988: b"--no-\0" as * ... _char:   l1943 = UNIQUE | NON_NULL, (empty)
                            // 988: b"--no-\0" as * ... st u8: typeof(_1165) = *const {l1945} u8
                            // 988: b"--no-\0" as * ... st u8:   l1945 = UNIQUE | NON_NULL, (empty)
                            // 988: b"--no-\0": typeof(_1166) = *const {l1947} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                            // 988: b"--no-\0":   l1947 = UNIQUE | NON_NULL, (empty)
                            // 988: b"--no-\0": typeof(_1167) = & {l1949} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                            // 988: b"--no-\0":   l1949 = UNIQUE | NON_NULL, FIXED
                            // 988: b"--no-\0": typeof(_1166 = &raw const (*_1167)) = *const {l3562} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                            // 988: b"--no-\0":   l3562 = UNIQUE | NON_NULL, (empty)
                            // 988: b"--no-\0" as * ... _char: typeof(_1164 = move _1165 as *const i8 (Misc)) = *const {l3564} i8
                            // 988: b"--no-\0" as * ... _char:   l3564 = UNIQUE | NON_NULL, (empty)
                            // 988: b"--no-\0": typeof(_1167 = const b"--no-\x00") = & {l3561} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                            // 988: b"--no-\0":   l3561 = UNIQUE | NON_NULL, (empty)
                            // 988: b"--no-\0" as * ... st u8: typeof(_1165 = move _1166 as *const u8 (Pointer(ArrayToPointer))) = *const {l3563} u8
                            // 988: b"--no-\0" as * ... st u8:   l3563 = UNIQUE | NON_NULL, (empty)
                            5 as libc::c_int as libc::c_ulong,
                        ) == 0
                        {
                            tmp = strdup(
                            // 992: strdup( (*argv. ... e), ): typeof(_1170) = *mut {l1953} i8
                            // 992: strdup( (*argv. ... e), ):   l1953 = UNIQUE | NON_NULL, (empty)
                                (*argv.offset(i as isize)).offset(5 as libc::c_int as isize),
                                // 993: (*argv.offset(i ... size): typeof(_1171) = *const {l1955} i8
                                // 993: (*argv.offset(i ... size):   l1955 = UNIQUE | NON_NULL, (empty)
                                // 993: (*argv.offset(i ... size): typeof(_1172) = *mut {l1957} i8
                                // 993: (*argv.offset(i ... size):   l1957 = UNIQUE | NON_NULL, (empty)
                                // 993: (*argv.offset(i ... ize)): typeof(_1173) = *mut {l1959} i8
                                // 993: (*argv.offset(i ... ize)):   l1959 = UNIQUE | NON_NULL, (empty)
                                // 993: argv.offset(i a ... size): typeof(_1174) = *mut {l1961} *mut {l1962} i8
                                // 993: argv.offset(i a ... size):   l1961 = UNIQUE | NON_NULL, (empty)
                                // 993: argv.offset(i a ... size):   l1962 = UNIQUE | NON_NULL, (empty)
                                // 993: argv: typeof(_1175) = *mut {l1964} *mut {l1965} i8
                                // 993: argv:   l1964 = UNIQUE | NON_NULL, (empty)
                                // 993: argv:   l1965 = UNIQUE | NON_NULL, (empty)
                                // 993: (*argv.offset(i ... size): typeof(_1171 = move _1172 as *const i8 (Pointer(MutToConstPointer))) = *const {l3565} i8
                                // 993: (*argv.offset(i ... size):   l3565 = UNIQUE | NON_NULL, (empty)
                            );
                            val = 0 as libc::c_int;
                        } else {
                            tmp = strdup(
                            // 997: strdup( (*argv. ... e), ): typeof(_1181) = *mut {l1972} i8
                            // 997: strdup( (*argv. ... e), ):   l1972 = UNIQUE | NON_NULL, (empty)
                                (*argv.offset(i as isize)).offset(2 as libc::c_int as isize),
                                // 998: (*argv.offset(i ... size): typeof(_1182) = *const {l1974} i8
                                // 998: (*argv.offset(i ... size):   l1974 = UNIQUE | NON_NULL, (empty)
                                // 998: (*argv.offset(i ... size): typeof(_1183) = *mut {l1976} i8
                                // 998: (*argv.offset(i ... size):   l1976 = UNIQUE | NON_NULL, (empty)
                                // 998: (*argv.offset(i ... ize)): typeof(_1184) = *mut {l1978} i8
                                // 998: (*argv.offset(i ... ize)):   l1978 = UNIQUE | NON_NULL, (empty)
                                // 998: argv.offset(i a ... size): typeof(_1185) = *mut {l1980} *mut {l1981} i8
                                // 998: argv.offset(i a ... size):   l1980 = UNIQUE | NON_NULL, (empty)
                                // 998: argv.offset(i a ... size):   l1981 = UNIQUE | NON_NULL, (empty)
                                // 998: argv: typeof(_1186) = *mut {l1983} *mut {l1984} i8
                                // 998: argv:   l1983 = UNIQUE | NON_NULL, (empty)
                                // 998: argv:   l1984 = UNIQUE | NON_NULL, (empty)
                                // 998: (*argv.offset(i ... size): typeof(_1182 = move _1183 as *const i8 (Pointer(MutToConstPointer))) = *const {l3566} i8
                                // 998: (*argv.offset(i ... size):   l3566 = UNIQUE | NON_NULL, (empty)
                            );
                            val = lglgetopt(lgl, tmp) + 1 as libc::c_int;
                            // 1000: lgl: typeof(_1192) = *mut {l1991} LGL
                            // 1000: lgl:   l1991 = UNIQUE | NON_NULL, (empty)
                            // 1000: tmp: typeof(_1193) = *const {l1993} i8
                            // 1000: tmp:   l1993 = UNIQUE | NON_NULL, (empty)
                            // 1000: tmp: typeof(_1194) = *mut {l1995} i8
                            // 1000: tmp:   l1995 = UNIQUE | NON_NULL, (empty)
                            // 1000: tmp: typeof(_1193 = move _1194 as *const i8 (Pointer(MutToConstPointer))) = *const {l3567} i8
                            // 1000: tmp:   l3567 = UNIQUE | NON_NULL, (empty)
                        }
                        current_block = 6988365858197790817;
                    }
                    match current_block {
                        7351195479953500246 => {}
                        15067877082424188916 => {}
                        _ => {
                            if lglhasopt(lgl, tmp) == 0 {
                            // 1008: lgl: typeof(_1199) = *mut {l2001} LGL
                            // 1008: lgl:   l2001 = UNIQUE | NON_NULL, (empty)
                            // 1008: tmp: typeof(_1200) = *const {l2003} i8
                            // 1008: tmp:   l2003 = UNIQUE | NON_NULL, (empty)
                            // 1008: tmp: typeof(_1201) = *mut {l2005} i8
                            // 1008: tmp:   l2005 = UNIQUE | NON_NULL, (empty)
                            // 1008: tmp: typeof(_1200 = move _1201 as *const i8 (Pointer(MutToConstPointer))) = *const {l3568} i8
                            // 1008: tmp:   l3568 = UNIQUE | NON_NULL, (empty)
                                free(tmp as *mut libc::c_void);
                                // 1009: tmp as *mut lib ... _void: typeof(_1203) = *mut {l2008} DefId(2:5583 ~ core[480a]::ffi::c_void)
                                // 1009: tmp as *mut lib ... _void:   l2008 = UNIQUE | NON_NULL, (empty)
                                // 1009: tmp: typeof(_1204) = *mut {l2010} i8
                                // 1009: tmp:   l2010 = UNIQUE | NON_NULL, (empty)
                                // 1009: tmp as *mut lib ... _void: typeof(_1203 = move _1204 as *mut libc::c_void (Misc)) = *mut {l3569} DefId(2:5583 ~ core[480a]::ffi::c_void)
                                // 1009: tmp as *mut lib ... _void:   l3569 = UNIQUE | NON_NULL, (empty)
                                current_block = 15067877082424188916;
                            } else {
                                lglsetopt(lgl, tmp, val);
                                // 1012: lgl: typeof(_1206) = *mut {l2013} LGL
                                // 1012: lgl:   l2013 = UNIQUE | NON_NULL, (empty)
                                // 1012: tmp: typeof(_1207) = *const {l2015} i8
                                // 1012: tmp:   l2015 = UNIQUE | NON_NULL, (empty)
                                // 1012: tmp: typeof(_1208) = *mut {l2017} i8
                                // 1012: tmp:   l2017 = UNIQUE | NON_NULL, (empty)
                                // 1012: tmp: typeof(_1207 = move _1208 as *const i8 (Pointer(MutToConstPointer))) = *const {l3570} i8
                                // 1012: tmp:   l3570 = UNIQUE | NON_NULL, (empty)
                                free(tmp as *mut libc::c_void);
                                // 1013: tmp as *mut lib ... _void: typeof(_1211) = *mut {l2021} DefId(2:5583 ~ core[480a]::ffi::c_void)
                                // 1013: tmp as *mut lib ... _void:   l2021 = UNIQUE | NON_NULL, (empty)
                                // 1013: tmp: typeof(_1212) = *mut {l2023} i8
                                // 1013: tmp:   l2023 = UNIQUE | NON_NULL, (empty)
                                // 1013: tmp as *mut lib ... _void: typeof(_1211 = move _1212 as *mut libc::c_void (Misc)) = *mut {l3571} DefId(2:5583 ~ core[480a]::ffi::c_void)
                                // 1013: tmp as *mut lib ... _void:   l3571 = UNIQUE | NON_NULL, (empty)
                                current_block = 7351195479953500246;
                            }
                        }
                    }
                } else if *(*argv.offset(i as isize)).offset(2 as libc::c_int as isize) != 0 {
                // 1018: (*argv.offset(i ... size): typeof(_1215) = *mut {l2027} i8
                // 1018: (*argv.offset(i ... size):   l2027 = UNIQUE | NON_NULL, (empty)
                // 1018: (*argv.offset(i ... ize)): typeof(_1216) = *mut {l2029} i8
                // 1018: (*argv.offset(i ... ize)):   l2029 = UNIQUE | NON_NULL, (empty)
                // 1018: argv.offset(i a ... size): typeof(_1217) = *mut {l2031} *mut {l2032} i8
                // 1018: argv.offset(i a ... size):   l2031 = UNIQUE | NON_NULL, (empty)
                // 1018: argv.offset(i a ... size):   l2032 = UNIQUE | NON_NULL, (empty)
                // 1018: argv: typeof(_1218) = *mut {l2034} *mut {l2035} i8
                // 1018: argv:   l2034 = UNIQUE | NON_NULL, (empty)
                // 1018: argv:   l2035 = UNIQUE | NON_NULL, (empty)
                    current_block = 15067877082424188916;
                } else if lglhasopt(
                    lgl,
                    // 1021: lgl: typeof(_1225) = *mut {l2043} LGL
                    // 1021: lgl:   l2043 = UNIQUE | NON_NULL, (empty)
                    (*argv.offset(i as isize)).offset(1 as libc::c_int as isize),
                    // 1022: (*argv.offset(i ... size): typeof(_1226) = *const {l2045} i8
                    // 1022: (*argv.offset(i ... size):   l2045 = UNIQUE | NON_NULL, (empty)
                    // 1022: (*argv.offset(i ... size): typeof(_1227) = *mut {l2047} i8
                    // 1022: (*argv.offset(i ... size):   l2047 = UNIQUE | NON_NULL, (empty)
                    // 1022: (*argv.offset(i ... ize)): typeof(_1228) = *mut {l2049} i8
                    // 1022: (*argv.offset(i ... ize)):   l2049 = UNIQUE | NON_NULL, (empty)
                    // 1022: argv.offset(i a ... size): typeof(_1229) = *mut {l2051} *mut {l2052} i8
                    // 1022: argv.offset(i a ... size):   l2051 = UNIQUE | NON_NULL, (empty)
                    // 1022: argv.offset(i a ... size):   l2052 = UNIQUE | NON_NULL, (empty)
                    // 1022: argv: typeof(_1230) = *mut {l2054} *mut {l2055} i8
                    // 1022: argv:   l2054 = UNIQUE | NON_NULL, (empty)
                    // 1022: argv:   l2055 = UNIQUE | NON_NULL, (empty)
                    // 1022: (*argv.offset(i ... size): typeof(_1226 = move _1227 as *const i8 (Pointer(MutToConstPointer))) = *const {l3572} i8
                    // 1022: (*argv.offset(i ... size):   l3572 = UNIQUE | NON_NULL, (empty)
                ) == 0
                {
                    current_block = 15067877082424188916;
                } else {
                    val = lglgetopt(
                        lgl,
                        // 1028: lgl: typeof(_1236) = *mut {l2062} LGL
                        // 1028: lgl:   l2062 = UNIQUE | NON_NULL, (empty)
                        (*argv.offset(i as isize)).offset(1 as libc::c_int as isize),
                        // 1029: (*argv.offset(i ... size): typeof(_1237) = *const {l2064} i8
                        // 1029: (*argv.offset(i ... size):   l2064 = UNIQUE | NON_NULL, (empty)
                        // 1029: (*argv.offset(i ... size): typeof(_1238) = *mut {l2066} i8
                        // 1029: (*argv.offset(i ... size):   l2066 = UNIQUE | NON_NULL, (empty)
                        // 1029: (*argv.offset(i ... ize)): typeof(_1239) = *mut {l2068} i8
                        // 1029: (*argv.offset(i ... ize)):   l2068 = UNIQUE | NON_NULL, (empty)
                        // 1029: argv.offset(i a ... size): typeof(_1240) = *mut {l2070} *mut {l2071} i8
                        // 1029: argv.offset(i a ... size):   l2070 = UNIQUE | NON_NULL, (empty)
                        // 1029: argv.offset(i a ... size):   l2071 = UNIQUE | NON_NULL, (empty)
                        // 1029: argv: typeof(_1241) = *mut {l2073} *mut {l2074} i8
                        // 1029: argv:   l2073 = UNIQUE | NON_NULL, (empty)
                        // 1029: argv:   l2074 = UNIQUE | NON_NULL, (empty)
                        // 1029: (*argv.offset(i ... size): typeof(_1237 = move _1238 as *const i8 (Pointer(MutToConstPointer))) = *const {l3573} i8
                        // 1029: (*argv.offset(i ... size):   l3573 = UNIQUE | NON_NULL, (empty)
                    ) + 1 as libc::c_int;
                    lglsetopt(
                        lgl,
                        // 1032: lgl: typeof(_1249) = *mut {l2083} LGL
                        // 1032: lgl:   l2083 = UNIQUE | NON_NULL, (empty)
                        (*argv.offset(i as isize)).offset(1 as libc::c_int as isize),
                        // 1033: (*argv.offset(i ... size): typeof(_1250) = *const {l2085} i8
                        // 1033: (*argv.offset(i ... size):   l2085 = UNIQUE | NON_NULL, (empty)
                        // 1033: (*argv.offset(i ... size): typeof(_1251) = *mut {l2087} i8
                        // 1033: (*argv.offset(i ... size):   l2087 = UNIQUE | NON_NULL, (empty)
                        // 1033: (*argv.offset(i ... ize)): typeof(_1252) = *mut {l2089} i8
                        // 1033: (*argv.offset(i ... ize)):   l2089 = UNIQUE | NON_NULL, (empty)
                        // 1033: argv.offset(i a ... size): typeof(_1253) = *mut {l2091} *mut {l2092} i8
                        // 1033: argv.offset(i a ... size):   l2091 = UNIQUE | NON_NULL, (empty)
                        // 1033: argv.offset(i a ... size):   l2092 = UNIQUE | NON_NULL, (empty)
                        // 1033: argv: typeof(_1254) = *mut {l2094} *mut {l2095} i8
                        // 1033: argv:   l2094 = UNIQUE | NON_NULL, (empty)
                        // 1033: argv:   l2095 = UNIQUE | NON_NULL, (empty)
                        // 1033: (*argv.offset(i ... size): typeof(_1250 = move _1251 as *const i8 (Pointer(MutToConstPointer))) = *const {l3574} i8
                        // 1033: (*argv.offset(i ... size):   l3574 = UNIQUE | NON_NULL, (empty)
                        val,
                    );
                    current_block = 7351195479953500246;
                }
                match current_block {
                    7351195479953500246 => {}
                    _ => {
                        fprintf(
                            stderr,
                            // 1042: stderr: typeof(_1262) = *mut {l2104} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                            // 1042: stderr:   l2104 = UNIQUE | NON_NULL, (empty)
                            // 1042: stderr: typeof(_1263) = *mut {l2106} *mut {l2107} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                            // 1042: stderr:   l2106 = UNIQUE | NON_NULL, (empty)
                            // 1042: stderr:   l2107 = UNIQUE | NON_NULL, (empty)
                            b"*** lingeling error: invalid command line option '%s'\n\0"
                            // 1043: b"*** lingeling ... _char: typeof(_1264) = *const {l2109} i8
                            // 1043: b"*** lingeling ... _char:   l2109 = UNIQUE | NON_NULL, (empty)
                            // 1043: b"*** lingeling ... st u8: typeof(_1265) = *const {l2111} u8
                            // 1043: b"*** lingeling ... st u8:   l2111 = UNIQUE | NON_NULL, (empty)
                            // 1043: b"*** lingeling ... \n\0": typeof(_1266) = *const {l2113} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000037)) }]
                            // 1043: b"*** lingeling ... \n\0":   l2113 = UNIQUE | NON_NULL, (empty)
                            // 1043: b"*** lingeling ... \n\0": typeof(_1267) = & {l2115} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000037)) }]
                            // 1043: b"*** lingeling ... \n\0":   l2115 = UNIQUE | NON_NULL, FIXED
                            // 1043: b"*** lingeling ... \n\0": typeof(_1266 = &raw const (*_1267)) = *const {l3576} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000037)) }]
                            // 1043: b"*** lingeling ... \n\0":   l3576 = UNIQUE | NON_NULL, (empty)
                            // 1043: b"*** lingeling ... \n\0": typeof(_1267 = const b"*** lingeling error: invalid command line option \'%s\'\n\x00") = & {l3575} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000037)) }]
                            // 1043: b"*** lingeling ... \n\0":   l3575 = UNIQUE | NON_NULL, (empty)
                            // 1043: b"*** lingeling ... _char: typeof(_1264 = move _1265 as *const i8 (Misc)) = *const {l3578} i8
                            // 1043: b"*** lingeling ... _char:   l3578 = UNIQUE | NON_NULL, (empty)
                            // 1043: b"*** lingeling ... st u8: typeof(_1265 = move _1266 as *const u8 (Pointer(ArrayToPointer))) = *const {l3577} u8
                            // 1043: b"*** lingeling ... st u8:   l3577 = UNIQUE | NON_NULL, (empty)
                                as *const u8 as *const libc::c_char,
                            *argv.offset(i as isize),
                            // 1045: *argv.offset(i  ... size): typeof(_1268) = *mut {l2117} i8
                            // 1045: *argv.offset(i  ... size):   l2117 = UNIQUE | NON_NULL, (empty)
                            // 1045: argv.offset(i a ... size): typeof(_1269) = *mut {l2119} *mut {l2120} i8
                            // 1045: argv.offset(i a ... size):   l2119 = UNIQUE | NON_NULL, (empty)
                            // 1045: argv.offset(i a ... size):   l2120 = UNIQUE | NON_NULL, (empty)
                            // 1045: argv: typeof(_1270) = *mut {l2122} *mut {l2123} i8
                            // 1045: argv:   l2122 = UNIQUE | NON_NULL, (empty)
                            // 1045: argv:   l2123 = UNIQUE | NON_NULL, (empty)
                        );
                        res = 1 as libc::c_int;
                        current_block = 14603147171032977705;
                        break;
                    }
                }
            } else if !iname.is_null() {
            // 1052: iname: typeof(_1276) = *const {l2130} i8
            // 1052: iname:   l2130 = UNIQUE | NON_NULL, (empty)
                fprintf(
                    stderr,
                    // 1054: stderr: typeof(_1279) = *mut {l2134} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                    // 1054: stderr:   l2134 = UNIQUE | NON_NULL, (empty)
                    // 1054: stderr: typeof(_1280) = *mut {l2136} *mut {l2137} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                    // 1054: stderr:   l2136 = UNIQUE | NON_NULL, (empty)
                    // 1054: stderr:   l2137 = UNIQUE | NON_NULL, (empty)
                    b"*** lingeling error: can not read '%s' and '%s'\n\0" as *const u8
                    // 1055: b"*** lingeling ... _char: typeof(_1281) = *const {l2139} i8
                    // 1055: b"*** lingeling ... _char:   l2139 = UNIQUE | NON_NULL, (empty)
                    // 1055: b"*** lingeling ... st u8: typeof(_1282) = *const {l2141} u8
                    // 1055: b"*** lingeling ... st u8:   l2141 = UNIQUE | NON_NULL, (empty)
                    // 1055: b"*** lingeling ... \n\0": typeof(_1283) = *const {l2143} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                    // 1055: b"*** lingeling ... \n\0":   l2143 = UNIQUE | NON_NULL, (empty)
                    // 1055: b"*** lingeling ... \n\0": typeof(_1284) = & {l2145} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                    // 1055: b"*** lingeling ... \n\0":   l2145 = UNIQUE | NON_NULL, FIXED
                    // 1055: b"*** lingeling ... \n\0": typeof(_1283 = &raw const (*_1284)) = *const {l3580} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                    // 1055: b"*** lingeling ... \n\0":   l3580 = UNIQUE | NON_NULL, (empty)
                    // 1055: b"*** lingeling ... _char: typeof(_1281 = move _1282 as *const i8 (Misc)) = *const {l3582} i8
                    // 1055: b"*** lingeling ... _char:   l3582 = UNIQUE | NON_NULL, (empty)
                    // 1055: b"*** lingeling ... st u8: typeof(_1282 = move _1283 as *const u8 (Pointer(ArrayToPointer))) = *const {l3581} u8
                    // 1055: b"*** lingeling ... st u8:   l3581 = UNIQUE | NON_NULL, (empty)
                    // 1055: b"*** lingeling ... \n\0": typeof(_1284 = const b"*** lingeling error: can not read \'%s\' and \'%s\'\n\x00") = & {l3579} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                    // 1055: b"*** lingeling ... \n\0":   l3579 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                    iname,
                    // 1057: iname: typeof(_1285) = *const {l2147} i8
                    // 1057: iname:   l2147 = UNIQUE | NON_NULL, (empty)
                    *argv.offset(i as isize),
                    // 1058: *argv.offset(i  ... size): typeof(_1286) = *mut {l2149} i8
                    // 1058: *argv.offset(i  ... size):   l2149 = UNIQUE | NON_NULL, (empty)
                    // 1058: argv.offset(i a ... size): typeof(_1287) = *mut {l2151} *mut {l2152} i8
                    // 1058: argv.offset(i a ... size):   l2151 = UNIQUE | NON_NULL, (empty)
                    // 1058: argv.offset(i a ... size):   l2152 = UNIQUE | NON_NULL, (empty)
                    // 1058: argv: typeof(_1288) = *mut {l2154} *mut {l2155} i8
                    // 1058: argv:   l2154 = UNIQUE | NON_NULL, (empty)
                    // 1058: argv:   l2155 = UNIQUE | NON_NULL, (empty)
                );
                res = 1 as libc::c_int;
                current_block = 14603147171032977705;
                break;
            } else {
                iname = *argv.offset(i as isize);
                // 1064: *argv.offset(i  ... size): typeof(_1292) = *mut {l2160} i8
                // 1064: *argv.offset(i  ... size):   l2160 = UNIQUE | NON_NULL, (empty)
                // 1064: argv.offset(i a ... size): typeof(_1293) = *mut {l2162} *mut {l2163} i8
                // 1064: argv.offset(i a ... size):   l2162 = UNIQUE | NON_NULL, (empty)
                // 1064: argv.offset(i a ... size):   l2163 = UNIQUE | NON_NULL, (empty)
                // 1064: argv: typeof(_1294) = *mut {l2165} *mut {l2166} i8
                // 1064: argv:   l2165 = UNIQUE | NON_NULL, (empty)
                // 1064: argv:   l2166 = UNIQUE | NON_NULL, (empty)
                // 1064: iname = *argv.o ... size): typeof(_15 = move _1292 as *const i8 (Pointer(MutToConstPointer))) = *const {l3583} i8
                // 1064: iname = *argv.o ... size):   l3583 = UNIQUE | NON_NULL, (empty)
            }
            i += 1;
            i;
        }
    }
    match current_block {
        17727836384662615028 => {
            verbose = lglgetopt(lgl, b"verbose\0" as *const u8 as *const libc::c_char);
            // 1072: lgl: typeof(_1301) = *mut {l2174} LGL
            // 1072: lgl:   l2174 = UNIQUE | NON_NULL, (empty)
            // 1072: b"verbose\0" as ... _char: typeof(_1302) = *const {l2176} i8
            // 1072: b"verbose\0" as ... _char:   l2176 = UNIQUE | NON_NULL, (empty)
            // 1072: b"verbose\0" as ... st u8: typeof(_1303) = *const {l2178} u8
            // 1072: b"verbose\0" as ... st u8:   l2178 = UNIQUE | NON_NULL, (empty)
            // 1072: b"verbose\0": typeof(_1304) = *const {l2180} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 1072: b"verbose\0":   l2180 = UNIQUE | NON_NULL, (empty)
            // 1072: b"verbose\0": typeof(_1305) = & {l2182} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 1072: b"verbose\0":   l2182 = UNIQUE | NON_NULL, FIXED
            // 1072: verbose: typeof(_1306) = *mut {l2184} i32
            // 1072: verbose:   l2184 = UNIQUE | NON_NULL, (empty)
            // 1072: b"verbose\0" as ... _char: typeof(_1302 = move _1303 as *const i8 (Misc)) = *const {l3587} i8
            // 1072: b"verbose\0" as ... _char:   l3587 = UNIQUE | NON_NULL, (empty)
            // 1072: b"verbose\0": typeof(_1305 = const b"verbose\x00") = & {l3584} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 1072: b"verbose\0":   l3584 = UNIQUE | NON_NULL, (empty)
            // 1072: b"verbose\0": typeof(_1304 = &raw const (*_1305)) = *const {l3585} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 1072: b"verbose\0":   l3585 = UNIQUE | NON_NULL, (empty)
            // 1072: b"verbose\0" as ... st u8: typeof(_1303 = move _1304 as *const u8 (Pointer(ArrayToPointer))) = *const {l3586} u8
            // 1072: b"verbose\0" as ... st u8:   l3586 = UNIQUE | NON_NULL, (empty)
            if verbose >= 0 as libc::c_int {
            // 1073: verbose: typeof(_1310) = *mut {l2189} i32
            // 1073: verbose:   l2189 = UNIQUE | NON_NULL, (empty)
                lglbnr(
                    b"Lingeling SAT Solver\0" as *const u8 as *const libc::c_char,
                    // 1075: b"Lingeling SAT ... _char: typeof(_1313) = *const {l2193} i8
                    // 1075: b"Lingeling SAT ... _char:   l2193 = UNIQUE | NON_NULL, (empty)
                    // 1075: b"Lingeling SAT ... st u8: typeof(_1314) = *const {l2195} u8
                    // 1075: b"Lingeling SAT ... st u8:   l2195 = UNIQUE | NON_NULL, (empty)
                    // 1075: b"Lingeling SAT ... er\0": typeof(_1315) = *const {l2197} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 1075: b"Lingeling SAT ... er\0":   l2197 = UNIQUE | NON_NULL, (empty)
                    // 1075: b"Lingeling SAT ... er\0": typeof(_1316) = & {l2199} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 1075: b"Lingeling SAT ... er\0":   l2199 = UNIQUE | NON_NULL, FIXED
                    // 1075: b"Lingeling SAT ... er\0": typeof(_1316 = const b"Lingeling SAT Solver\x00") = & {l3588} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 1075: b"Lingeling SAT ... er\0":   l3588 = UNIQUE | NON_NULL, (empty)
                    // 1075: b"Lingeling SAT ... st u8: typeof(_1314 = move _1315 as *const u8 (Pointer(ArrayToPointer))) = *const {l3590} u8
                    // 1075: b"Lingeling SAT ... st u8:   l3590 = UNIQUE | NON_NULL, (empty)
                    // 1075: b"Lingeling SAT ... _char: typeof(_1313 = move _1314 as *const i8 (Misc)) = *const {l3591} i8
                    // 1075: b"Lingeling SAT ... _char:   l3591 = UNIQUE | NON_NULL, (empty)
                    // 1075: b"Lingeling SAT ... er\0": typeof(_1315 = &raw const (*_1316)) = *const {l3589} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 1075: b"Lingeling SAT ... er\0":   l3589 = UNIQUE | NON_NULL, (empty)
                    b"c \0" as *const u8 as *const libc::c_char,
                    // 1076: b"c \0" as *con ... _char: typeof(_1317) = *const {l2201} i8
                    // 1076: b"c \0" as *con ... _char:   l2201 = UNIQUE | NON_NULL, (empty)
                    // 1076: b"c \0" as *const u8: typeof(_1318) = *const {l2203} u8
                    // 1076: b"c \0" as *const u8:   l2203 = UNIQUE | NON_NULL, (empty)
                    // 1076: b"c \0": typeof(_1319) = *const {l2205} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                    // 1076: b"c \0":   l2205 = UNIQUE | NON_NULL, (empty)
                    // 1076: b"c \0": typeof(_1320) = & {l2207} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                    // 1076: b"c \0":   l2207 = UNIQUE | NON_NULL, FIXED
                    // 1076: b"c \0": typeof(_1319 = &raw const (*_1320)) = *const {l3593} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                    // 1076: b"c \0":   l3593 = UNIQUE | NON_NULL, (empty)
                    // 1076: b"c \0": typeof(_1320 = const b"c \x00") = & {l3592} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                    // 1076: b"c \0":   l3592 = UNIQUE | NON_NULL, (empty)
                    // 1076: b"c \0" as *con ... _char: typeof(_1317 = move _1318 as *const i8 (Misc)) = *const {l3595} i8
                    // 1076: b"c \0" as *con ... _char:   l3595 = UNIQUE | NON_NULL, (empty)
                    // 1076: b"c \0" as *const u8: typeof(_1318 = move _1319 as *const u8 (Pointer(ArrayToPointer))) = *const {l3594} u8
                    // 1076: b"c \0" as *const u8:   l3594 = UNIQUE | NON_NULL, (empty)
                    stdout,
                    // 1077: stdout: typeof(_1321) = *mut {l2209} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                    // 1077: stdout:   l2209 = UNIQUE | NON_NULL, (empty)
                    // 1077: stdout: typeof(_1322) = *mut {l2211} *mut {l2212} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                    // 1077: stdout:   l2211 = UNIQUE | NON_NULL, (empty)
                    // 1077: stdout:   l2212 = UNIQUE | NON_NULL, (empty)
                );
                if simponly != 0 {
                    printf(b"c simplifying only\n\0" as *const u8 as *const libc::c_char);
                    // 1080: b"c simplifying ... _char: typeof(_1327) = *const {l2218} i8
                    // 1080: b"c simplifying ... _char:   l2218 = UNIQUE | NON_NULL, (empty)
                    // 1080: b"c simplifying ... st u8: typeof(_1328) = *const {l2220} u8
                    // 1080: b"c simplifying ... st u8:   l2220 = UNIQUE | NON_NULL, (empty)
                    // 1080: b"c simplifying ... \n\0": typeof(_1329) = *const {l2222} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                    // 1080: b"c simplifying ... \n\0":   l2222 = UNIQUE | NON_NULL, (empty)
                    // 1080: b"c simplifying ... \n\0": typeof(_1330) = & {l2224} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                    // 1080: b"c simplifying ... \n\0":   l2224 = UNIQUE | NON_NULL, FIXED
                    // 1080: b"c simplifying ... st u8: typeof(_1328 = move _1329 as *const u8 (Pointer(ArrayToPointer))) = *const {l3598} u8
                    // 1080: b"c simplifying ... st u8:   l3598 = UNIQUE | NON_NULL, (empty)
                    // 1080: b"c simplifying ... \n\0": typeof(_1330 = const b"c simplifying only\n\x00") = & {l3596} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                    // 1080: b"c simplifying ... \n\0":   l3596 = UNIQUE | NON_NULL, (empty)
                    // 1080: b"c simplifying ... _char: typeof(_1327 = move _1328 as *const i8 (Misc)) = *const {l3599} i8
                    // 1080: b"c simplifying ... _char:   l3599 = UNIQUE | NON_NULL, (empty)
                    // 1080: b"c simplifying ... \n\0": typeof(_1329 = &raw const (*_1330)) = *const {l3597} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                    // 1080: b"c simplifying ... \n\0":   l3597 = UNIQUE | NON_NULL, (empty)
                }
                if !oname.is_null() {
                // 1082: oname: typeof(_1334) = *const {l2229} i8
                // 1082: oname:   l2229 = UNIQUE | NON_NULL, (empty)
                    printf(
                        b"c output file %s\n\0" as *const u8 as *const libc::c_char,
                        // 1084: b"c output file ... _char: typeof(_1336) = *const {l2232} i8
                        // 1084: b"c output file ... _char:   l2232 = UNIQUE | NON_NULL, (empty)
                        // 1084: b"c output file ... st u8: typeof(_1337) = *const {l2234} u8
                        // 1084: b"c output file ... st u8:   l2234 = UNIQUE | NON_NULL, (empty)
                        // 1084: b"c output file ... \n\0": typeof(_1338) = *const {l2236} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                        // 1084: b"c output file ... \n\0":   l2236 = UNIQUE | NON_NULL, (empty)
                        // 1084: b"c output file ... \n\0": typeof(_1339) = & {l2238} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                        // 1084: b"c output file ... \n\0":   l2238 = UNIQUE | NON_NULL, FIXED
                        // 1084: b"c output file ... \n\0": typeof(_1338 = &raw const (*_1339)) = *const {l3601} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                        // 1084: b"c output file ... \n\0":   l3601 = UNIQUE | NON_NULL, (empty)
                        // 1084: b"c output file ... \n\0": typeof(_1339 = const b"c output file %s\n\x00") = & {l3600} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                        // 1084: b"c output file ... \n\0":   l3600 = UNIQUE | NON_NULL, (empty)
                        // 1084: b"c output file ... st u8: typeof(_1337 = move _1338 as *const u8 (Pointer(ArrayToPointer))) = *const {l3602} u8
                        // 1084: b"c output file ... st u8:   l3602 = UNIQUE | NON_NULL, (empty)
                        // 1084: b"c output file ... _char: typeof(_1336 = move _1337 as *const i8 (Misc)) = *const {l3603} i8
                        // 1084: b"c output file ... _char:   l3603 = UNIQUE | NON_NULL, (empty)
                        oname,
                        // 1085: oname: typeof(_1340) = *const {l2240} i8
                        // 1085: oname:   l2240 = UNIQUE | NON_NULL, (empty)
                    );
                }
                if simponly != 0 || !oname.is_null() {
                // 1088: oname: typeof(_1347) = *const {l2248} i8
                // 1088: oname:   l2248 = UNIQUE | NON_NULL, (empty)
                    fflush(stdout);
                    // 1089: stdout: typeof(_1349) = *mut {l2251} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                    // 1089: stdout:   l2251 = UNIQUE | NON_NULL, (empty)
                    // 1089: stdout: typeof(_1350) = *mut {l2253} *mut {l2254} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                    // 1089: stdout:   l2253 = UNIQUE | NON_NULL, (empty)
                    // 1089: stdout:   l2254 = UNIQUE | NON_NULL, (empty)
                }
                lglsetopt(
                    lgl,
                    // 1092: lgl: typeof(_1352) = *mut {l2257} LGL
                    // 1092: lgl:   l2257 = UNIQUE | NON_NULL, (empty)
                    b"trep\0" as *const u8 as *const libc::c_char,
                    // 1093: b"trep\0" as *c ... _char: typeof(_1353) = *const {l2259} i8
                    // 1093: b"trep\0" as *c ... _char:   l2259 = UNIQUE | NON_NULL, (empty)
                    // 1093: b"trep\0" as *c ... st u8: typeof(_1354) = *const {l2261} u8
                    // 1093: b"trep\0" as *c ... st u8:   l2261 = UNIQUE | NON_NULL, (empty)
                    // 1093: b"trep\0": typeof(_1355) = *const {l2263} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 1093: b"trep\0":   l2263 = UNIQUE | NON_NULL, (empty)
                    // 1093: b"trep\0": typeof(_1356) = & {l2265} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 1093: b"trep\0":   l2265 = UNIQUE | NON_NULL, FIXED
                    // 1093: b"trep\0": typeof(_1356 = const b"trep\x00") = & {l3604} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 1093: b"trep\0":   l3604 = UNIQUE | NON_NULL, (empty)
                    // 1093: b"trep\0" as *c ... st u8: typeof(_1354 = move _1355 as *const u8 (Pointer(ArrayToPointer))) = *const {l3606} u8
                    // 1093: b"trep\0" as *c ... st u8:   l3606 = UNIQUE | NON_NULL, (empty)
                    // 1093: b"trep\0": typeof(_1355 = &raw const (*_1356)) = *const {l3605} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 1093: b"trep\0":   l3605 = UNIQUE | NON_NULL, (empty)
                    // 1093: b"trep\0" as *c ... _char: typeof(_1353 = move _1354 as *const i8 (Misc)) = *const {l3607} i8
                    // 1093: b"trep\0" as *c ... _char:   l3607 = UNIQUE | NON_NULL, (empty)
                    1 as libc::c_int,
                );
            }
            if !thanks.is_null() {
            // 1097: thanks: typeof(_1361) = *const {l2271} i8
            // 1097: thanks:   l2271 = UNIQUE | NON_NULL, (empty)
                let mut seed: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                let mut ch: libc::c_uint = 0;
                let mut iseed: libc::c_int = 0;
                p = thanks;
                // 1102: thanks: typeof(_1368) = *const {l2279} i8
                // 1102: thanks:   l2279 = UNIQUE | NON_NULL, (empty)
                loop {
                    ch = *p as libc::c_uint;
                    if !(ch != 0) {
                        break;
                    }
                    let fresh4 = i_0;
                    i_0 = i_0.wrapping_add(1);
                    seed = seed
                        .wrapping_add((primes[fresh4 as usize] as libc::c_uint).wrapping_mul(ch));
                        // 1111: primes: typeof(_1384) = *mut {l2296} [i32; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                        // 1111: primes:   l2296 = UNIQUE | NON_NULL, (empty)
                    if i_0 == nprimes {
                    // 1112: nprimes: typeof(_1394) = *mut {l2307} u32
                    // 1112: nprimes:   l2307 = UNIQUE | NON_NULL, (empty)
                        i_0 = 0 as libc::c_int as libc::c_uint;
                    }
                    p = p.offset(1);
                    // 1115: p.offset(1): typeof(_1396) = *const {l2310} i8
                    // 1115: p.offset(1):   l2310 = UNIQUE | NON_NULL, (empty)
                    // 1115: p: typeof(_1397) = *const {l2312} i8
                    // 1115: p:   l2312 = UNIQUE | NON_NULL, (empty)
                    p;
                    // 1116: p: typeof(_1398) = *const {l2314} i8
                    // 1116: p:   l2314 = UNIQUE | NON_NULL, (empty)
                }
                if seed >= 2147483647 as libc::c_int as libc::c_uint {
                    seed >>= 1 as libc::c_int;
                }
                iseed = seed as libc::c_int;
                if verbose != 0 {
                // 1122: verbose: typeof(_1410) = *mut {l2327} i32
                // 1122: verbose:   l2327 = UNIQUE | NON_NULL, (empty)
                    printf(
                        b"c will have to thank %s (--seed=%d)\nc\n\0" as *const u8
                        // 1124: b"c will have t ... _char: typeof(_1412) = *const {l2330} i8
                        // 1124: b"c will have t ... _char:   l2330 = UNIQUE | NON_NULL, (empty)
                        // 1124: b"c will have t ... st u8: typeof(_1413) = *const {l2332} u8
                        // 1124: b"c will have t ... st u8:   l2332 = UNIQUE | NON_NULL, (empty)
                        // 1124: b"c will have t ... \n\0": typeof(_1414) = *const {l2334} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                        // 1124: b"c will have t ... \n\0":   l2334 = UNIQUE | NON_NULL, (empty)
                        // 1124: b"c will have t ... \n\0": typeof(_1415) = & {l2336} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                        // 1124: b"c will have t ... \n\0":   l2336 = UNIQUE | NON_NULL, FIXED
                        // 1124: b"c will have t ... st u8: typeof(_1413 = move _1414 as *const u8 (Pointer(ArrayToPointer))) = *const {l3610} u8
                        // 1124: b"c will have t ... st u8:   l3610 = UNIQUE | NON_NULL, (empty)
                        // 1124: b"c will have t ... \n\0": typeof(_1415 = const b"c will have to thank %s (--seed=%d)\nc\n\x00") = & {l3608} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                        // 1124: b"c will have t ... \n\0":   l3608 = UNIQUE | NON_NULL, (empty)
                        // 1124: b"c will have t ... _char: typeof(_1412 = move _1413 as *const i8 (Misc)) = *const {l3611} i8
                        // 1124: b"c will have t ... _char:   l3611 = UNIQUE | NON_NULL, (empty)
                        // 1124: b"c will have t ... \n\0": typeof(_1414 = &raw const (*_1415)) = *const {l3609} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                        // 1124: b"c will have t ... \n\0":   l3609 = UNIQUE | NON_NULL, (empty)
                            as *const libc::c_char,
                        thanks,
                        // 1126: thanks: typeof(_1416) = *const {l2338} i8
                        // 1126: thanks:   l2338 = UNIQUE | NON_NULL, (empty)
                        iseed,
                    );
                }
                lglsetopt(lgl, b"seed\0" as *const u8 as *const libc::c_char, iseed);
                // 1130: lgl: typeof(_1419) = *mut {l2342} LGL
                // 1130: lgl:   l2342 = UNIQUE | NON_NULL, (empty)
                // 1130: b"seed\0" as *c ... _char: typeof(_1420) = *const {l2344} i8
                // 1130: b"seed\0" as *c ... _char:   l2344 = UNIQUE | NON_NULL, (empty)
                // 1130: b"seed\0" as *c ... st u8: typeof(_1421) = *const {l2346} u8
                // 1130: b"seed\0" as *c ... st u8:   l2346 = UNIQUE | NON_NULL, (empty)
                // 1130: b"seed\0": typeof(_1422) = *const {l2348} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 1130: b"seed\0":   l2348 = UNIQUE | NON_NULL, (empty)
                // 1130: b"seed\0": typeof(_1423) = & {l2350} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 1130: b"seed\0":   l2350 = UNIQUE | NON_NULL, FIXED
                // 1130: b"seed\0" as *c ... _char: typeof(_1420 = move _1421 as *const i8 (Misc)) = *const {l3615} i8
                // 1130: b"seed\0" as *c ... _char:   l3615 = UNIQUE | NON_NULL, (empty)
                // 1130: b"seed\0": typeof(_1423 = const b"seed\x00") = & {l3612} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 1130: b"seed\0":   l3612 = UNIQUE | NON_NULL, (empty)
                // 1130: b"seed\0" as *c ... st u8: typeof(_1421 = move _1422 as *const u8 (Pointer(ArrayToPointer))) = *const {l3614} u8
                // 1130: b"seed\0" as *c ... st u8:   l3614 = UNIQUE | NON_NULL, (empty)
                // 1130: b"seed\0": typeof(_1422 = &raw const (*_1423)) = *const {l3613} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 1130: b"seed\0":   l3613 = UNIQUE | NON_NULL, (empty)
            }
            if verbose >= 2 as libc::c_int {
            // 1132: verbose: typeof(_1428) = *mut {l2356} i32
            // 1132: verbose:   l2356 = UNIQUE | NON_NULL, (empty)
                printf(
                    b"c\nc options after command line parsing:\nc\n\0" as *const u8
                    // 1134: b"c\nc options  ... _char: typeof(_1431) = *const {l2360} i8
                    // 1134: b"c\nc options  ... _char:   l2360 = UNIQUE | NON_NULL, (empty)
                    // 1134: b"c\nc options  ... st u8: typeof(_1432) = *const {l2362} u8
                    // 1134: b"c\nc options  ... st u8:   l2362 = UNIQUE | NON_NULL, (empty)
                    // 1134: b"c\nc options  ... \n\0": typeof(_1433) = *const {l2364} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                    // 1134: b"c\nc options  ... \n\0":   l2364 = UNIQUE | NON_NULL, (empty)
                    // 1134: b"c\nc options  ... \n\0": typeof(_1434) = & {l2366} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                    // 1134: b"c\nc options  ... \n\0":   l2366 = UNIQUE | NON_NULL, FIXED
                    // 1134: b"c\nc options  ... \n\0": typeof(_1433 = &raw const (*_1434)) = *const {l3617} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                    // 1134: b"c\nc options  ... \n\0":   l3617 = UNIQUE | NON_NULL, (empty)
                    // 1134: b"c\nc options  ... _char: typeof(_1431 = move _1432 as *const i8 (Misc)) = *const {l3619} i8
                    // 1134: b"c\nc options  ... _char:   l3619 = UNIQUE | NON_NULL, (empty)
                    // 1134: b"c\nc options  ... \n\0": typeof(_1434 = const b"c\nc options after command line parsing:\nc\n\x00") = & {l3616} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                    // 1134: b"c\nc options  ... \n\0":   l3616 = UNIQUE | NON_NULL, (empty)
                    // 1134: b"c\nc options  ... st u8: typeof(_1432 = move _1433 as *const u8 (Pointer(ArrayToPointer))) = *const {l3618} u8
                    // 1134: b"c\nc options  ... st u8:   l3618 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                );
                lglopts(
                    lgl,
                    // 1138: lgl: typeof(_1436) = *mut {l2369} LGL
                    // 1138: lgl:   l2369 = UNIQUE | NON_NULL, (empty)
                    b"c \0" as *const u8 as *const libc::c_char,
                    // 1139: b"c \0" as *con ... _char: typeof(_1437) = *const {l2371} i8
                    // 1139: b"c \0" as *con ... _char:   l2371 = UNIQUE | NON_NULL, (empty)
                    // 1139: b"c \0" as *const u8: typeof(_1438) = *const {l2373} u8
                    // 1139: b"c \0" as *const u8:   l2373 = UNIQUE | NON_NULL, (empty)
                    // 1139: b"c \0": typeof(_1439) = *const {l2375} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                    // 1139: b"c \0":   l2375 = UNIQUE | NON_NULL, (empty)
                    // 1139: b"c \0": typeof(_1440) = & {l2377} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                    // 1139: b"c \0":   l2377 = UNIQUE | NON_NULL, FIXED
                    // 1139: b"c \0": typeof(_1440 = const b"c \x00") = & {l3620} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                    // 1139: b"c \0":   l3620 = UNIQUE | NON_NULL, (empty)
                    // 1139: b"c \0": typeof(_1439 = &raw const (*_1440)) = *const {l3621} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                    // 1139: b"c \0":   l3621 = UNIQUE | NON_NULL, (empty)
                    // 1139: b"c \0" as *const u8: typeof(_1438 = move _1439 as *const u8 (Pointer(ArrayToPointer))) = *const {l3622} u8
                    // 1139: b"c \0" as *const u8:   l3622 = UNIQUE | NON_NULL, (empty)
                    // 1139: b"c \0" as *con ... _char: typeof(_1437 = move _1438 as *const i8 (Misc)) = *const {l3623} i8
                    // 1139: b"c \0" as *con ... _char:   l3623 = UNIQUE | NON_NULL, (empty)
                    0 as libc::c_int,
                );
                printf(b"c\n\0" as *const u8 as *const libc::c_char);
                // 1142: b"c\n\0" as *co ... _char: typeof(_1443) = *const {l2381} i8
                // 1142: b"c\n\0" as *co ... _char:   l2381 = UNIQUE | NON_NULL, (empty)
                // 1142: b"c\n\0" as *co ... st u8: typeof(_1444) = *const {l2383} u8
                // 1142: b"c\n\0" as *co ... st u8:   l2383 = UNIQUE | NON_NULL, (empty)
                // 1142: b"c\n\0": typeof(_1445) = *const {l2385} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 1142: b"c\n\0":   l2385 = UNIQUE | NON_NULL, (empty)
                // 1142: b"c\n\0": typeof(_1446) = & {l2387} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 1142: b"c\n\0":   l2387 = UNIQUE | NON_NULL, FIXED
                // 1142: b"c\n\0": typeof(_1445 = &raw const (*_1446)) = *const {l3625} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 1142: b"c\n\0":   l3625 = UNIQUE | NON_NULL, (empty)
                // 1142: b"c\n\0" as *co ... _char: typeof(_1443 = move _1444 as *const i8 (Misc)) = *const {l3627} i8
                // 1142: b"c\n\0" as *co ... _char:   l3627 = UNIQUE | NON_NULL, (empty)
                // 1142: b"c\n\0": typeof(_1446 = const b"c\n\x00") = & {l3624} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 1142: b"c\n\0":   l3624 = UNIQUE | NON_NULL, (empty)
                // 1142: b"c\n\0" as *co ... st u8: typeof(_1444 = move _1445 as *const u8 (Pointer(ArrayToPointer))) = *const {l3626} u8
                // 1142: b"c\n\0" as *co ... st u8:   l3626 = UNIQUE | NON_NULL, (empty)
                lglsizes(lgl);
                // 1143: lgl: typeof(_1448) = *mut {l2390} LGL
                // 1143: lgl:   l2390 = UNIQUE | NON_NULL, (empty)
                printf(b"c\n\0" as *const u8 as *const libc::c_char);
                // 1144: b"c\n\0" as *co ... _char: typeof(_1450) = *const {l2393} i8
                // 1144: b"c\n\0" as *co ... _char:   l2393 = UNIQUE | NON_NULL, (empty)
                // 1144: b"c\n\0" as *co ... st u8: typeof(_1451) = *const {l2395} u8
                // 1144: b"c\n\0" as *co ... st u8:   l2395 = UNIQUE | NON_NULL, (empty)
                // 1144: b"c\n\0": typeof(_1452) = *const {l2397} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 1144: b"c\n\0":   l2397 = UNIQUE | NON_NULL, (empty)
                // 1144: b"c\n\0": typeof(_1453) = & {l2399} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 1144: b"c\n\0":   l2399 = UNIQUE | NON_NULL, FIXED
                // 1144: b"c\n\0" as *co ... st u8: typeof(_1451 = move _1452 as *const u8 (Pointer(ArrayToPointer))) = *const {l3630} u8
                // 1144: b"c\n\0" as *co ... st u8:   l3630 = UNIQUE | NON_NULL, (empty)
                // 1144: b"c\n\0" as *co ... _char: typeof(_1450 = move _1451 as *const i8 (Misc)) = *const {l3631} i8
                // 1144: b"c\n\0" as *co ... _char:   l3631 = UNIQUE | NON_NULL, (empty)
                // 1144: b"c\n\0": typeof(_1452 = &raw const (*_1453)) = *const {l3629} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 1144: b"c\n\0":   l3629 = UNIQUE | NON_NULL, (empty)
                // 1144: b"c\n\0": typeof(_1453 = const b"c\n\x00") = & {l3628} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 1144: b"c\n\0":   l3628 = UNIQUE | NON_NULL, (empty)
            }
            if !pname.is_null() {
            // 1146: pname: typeof(_1457) = *const {l2404} i8
            // 1146: pname:   l2404 = UNIQUE | NON_NULL, (empty)
                pfile = fopen(pname, b"r\0" as *const u8 as *const libc::c_char);
                // 1147: fopen(pname, b" ... char): typeof(_1458) = *mut {l2406} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                // 1147: fopen(pname, b" ... char):   l2406 = UNIQUE | NON_NULL, (empty)
                // 1147: pname: typeof(_1459) = *const {l2408} i8
                // 1147: pname:   l2408 = UNIQUE | NON_NULL, (empty)
                // 1147: b"r\0" as *cons ... _char: typeof(_1460) = *const {l2410} i8
                // 1147: b"r\0" as *cons ... _char:   l2410 = UNIQUE | NON_NULL, (empty)
                // 1147: b"r\0" as *const u8: typeof(_1461) = *const {l2412} u8
                // 1147: b"r\0" as *const u8:   l2412 = UNIQUE | NON_NULL, (empty)
                // 1147: b"r\0": typeof(_1462) = *const {l2414} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 1147: b"r\0":   l2414 = UNIQUE | NON_NULL, (empty)
                // 1147: b"r\0": typeof(_1463) = & {l2416} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 1147: b"r\0":   l2416 = UNIQUE | NON_NULL, FIXED
                // 1147: b"r\0": typeof(_1462 = &raw const (*_1463)) = *const {l3633} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 1147: b"r\0":   l3633 = UNIQUE | NON_NULL, (empty)
                // 1147: b"r\0" as *const u8: typeof(_1461 = move _1462 as *const u8 (Pointer(ArrayToPointer))) = *const {l3634} u8
                // 1147: b"r\0" as *const u8:   l3634 = UNIQUE | NON_NULL, (empty)
                // 1147: b"r\0" as *cons ... _char: typeof(_1460 = move _1461 as *const i8 (Misc)) = *const {l3635} i8
                // 1147: b"r\0" as *cons ... _char:   l3635 = UNIQUE | NON_NULL, (empty)
                // 1147: b"r\0": typeof(_1463 = const b"r\x00") = & {l3632} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 1147: b"r\0":   l3632 = UNIQUE | NON_NULL, (empty)
                if pfile.is_null() {
                // 1148: pfile: typeof(_1465) = *mut {l2419} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                // 1148: pfile:   l2419 = UNIQUE | NON_NULL, (empty)
                    fprintf(
                        stderr,
                        // 1150: stderr: typeof(_1467) = *mut {l2422} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 1150: stderr:   l2422 = UNIQUE | NON_NULL, (empty)
                        // 1150: stderr: typeof(_1468) = *mut {l2424} *mut {l2425} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 1150: stderr:   l2424 = UNIQUE | NON_NULL, (empty)
                        // 1150: stderr:   l2425 = UNIQUE | NON_NULL, (empty)
                        b"*** lingeling error: can not read option file %s\n\0" as *const u8
                        // 1151: b"*** lingeling ... _char: typeof(_1469) = *const {l2427} i8
                        // 1151: b"*** lingeling ... _char:   l2427 = UNIQUE | NON_NULL, (empty)
                        // 1151: b"*** lingeling ... st u8: typeof(_1470) = *const {l2429} u8
                        // 1151: b"*** lingeling ... st u8:   l2429 = UNIQUE | NON_NULL, (empty)
                        // 1151: b"*** lingeling ... \n\0": typeof(_1471) = *const {l2431} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
                        // 1151: b"*** lingeling ... \n\0":   l2431 = UNIQUE | NON_NULL, (empty)
                        // 1151: b"*** lingeling ... \n\0": typeof(_1472) = & {l2433} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
                        // 1151: b"*** lingeling ... \n\0":   l2433 = UNIQUE | NON_NULL, FIXED
                        // 1151: b"*** lingeling ... \n\0": typeof(_1472 = const b"*** lingeling error: can not read option file %s\n\x00") = & {l3636} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
                        // 1151: b"*** lingeling ... \n\0":   l3636 = UNIQUE | NON_NULL, (empty)
                        // 1151: b"*** lingeling ... _char: typeof(_1469 = move _1470 as *const i8 (Misc)) = *const {l3639} i8
                        // 1151: b"*** lingeling ... _char:   l3639 = UNIQUE | NON_NULL, (empty)
                        // 1151: b"*** lingeling ... \n\0": typeof(_1471 = &raw const (*_1472)) = *const {l3637} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
                        // 1151: b"*** lingeling ... \n\0":   l3637 = UNIQUE | NON_NULL, (empty)
                        // 1151: b"*** lingeling ... st u8: typeof(_1470 = move _1471 as *const u8 (Pointer(ArrayToPointer))) = *const {l3638} u8
                        // 1151: b"*** lingeling ... st u8:   l3638 = UNIQUE | NON_NULL, (empty)
                            as *const libc::c_char,
                        pname,
                        // 1153: pname: typeof(_1473) = *const {l2435} i8
                        // 1153: pname:   l2435 = UNIQUE | NON_NULL, (empty)
                    );
                    res = 1 as libc::c_int;
                    current_block = 14603147171032977705;
                } else {
                    if verbose >= 0 as libc::c_int {
                    // 1158: verbose: typeof(_1478) = *mut {l2441} i32
                    // 1158: verbose:   l2441 = UNIQUE | NON_NULL, (empty)
                        printf(
                            b"c reading options file %s\n\0" as *const u8 as *const libc::c_char,
                            // 1160: b"c reading opt ... _char: typeof(_1481) = *const {l2445} i8
                            // 1160: b"c reading opt ... _char:   l2445 = UNIQUE | NON_NULL, (empty)
                            // 1160: b"c reading opt ... st u8: typeof(_1482) = *const {l2447} u8
                            // 1160: b"c reading opt ... st u8:   l2447 = UNIQUE | NON_NULL, (empty)
                            // 1160: b"c reading opt ... \n\0": typeof(_1483) = *const {l2449} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                            // 1160: b"c reading opt ... \n\0":   l2449 = UNIQUE | NON_NULL, (empty)
                            // 1160: b"c reading opt ... \n\0": typeof(_1484) = & {l2451} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                            // 1160: b"c reading opt ... \n\0":   l2451 = UNIQUE | NON_NULL, FIXED
                            // 1160: b"c reading opt ... _char: typeof(_1481 = move _1482 as *const i8 (Misc)) = *const {l3643} i8
                            // 1160: b"c reading opt ... _char:   l3643 = UNIQUE | NON_NULL, (empty)
                            // 1160: b"c reading opt ... \n\0": typeof(_1483 = &raw const (*_1484)) = *const {l3641} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                            // 1160: b"c reading opt ... \n\0":   l3641 = UNIQUE | NON_NULL, (empty)
                            // 1160: b"c reading opt ... st u8: typeof(_1482 = move _1483 as *const u8 (Pointer(ArrayToPointer))) = *const {l3642} u8
                            // 1160: b"c reading opt ... st u8:   l3642 = UNIQUE | NON_NULL, (empty)
                            // 1160: b"c reading opt ... \n\0": typeof(_1484 = const b"c reading options file %s\n\x00") = & {l3640} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                            // 1160: b"c reading opt ... \n\0":   l3640 = UNIQUE | NON_NULL, (empty)
                            pname,
                            // 1161: pname: typeof(_1485) = *const {l2453} i8
                            // 1161: pname:   l2453 = UNIQUE | NON_NULL, (empty)
                        );
                        fflush(stdout);
                        // 1163: stdout: typeof(_1487) = *mut {l2456} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 1163: stdout:   l2456 = UNIQUE | NON_NULL, (empty)
                        // 1163: stdout: typeof(_1488) = *mut {l2458} *mut {l2459} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 1163: stdout:   l2458 = UNIQUE | NON_NULL, (empty)
                        // 1163: stdout:   l2459 = UNIQUE | NON_NULL, (empty)
                    }
                    nopts = lglreadopts(lgl, pfile);
                    // 1165: lgl: typeof(_1490) = *mut {l2462} LGL
                    // 1165: lgl:   l2462 = UNIQUE | NON_NULL, (empty)
                    // 1165: pfile: typeof(_1491) = *mut {l2464} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                    // 1165: pfile:   l2464 = UNIQUE | NON_NULL, (empty)
                    if verbose >= 0 as libc::c_int {
                    // 1166: verbose: typeof(_1495) = *mut {l2469} i32
                    // 1166: verbose:   l2469 = UNIQUE | NON_NULL, (empty)
                        printf(
                            b"c read and set %d options\nc\n\0" as *const u8 as *const libc::c_char,
                            // 1168: b"c read and se ... _char: typeof(_1498) = *const {l2473} i8
                            // 1168: b"c read and se ... _char:   l2473 = UNIQUE | NON_NULL, (empty)
                            // 1168: b"c read and se ... st u8: typeof(_1499) = *const {l2475} u8
                            // 1168: b"c read and se ... st u8:   l2475 = UNIQUE | NON_NULL, (empty)
                            // 1168: b"c read and se ... \n\0": typeof(_1500) = *const {l2477} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
                            // 1168: b"c read and se ... \n\0":   l2477 = UNIQUE | NON_NULL, (empty)
                            // 1168: b"c read and se ... \n\0": typeof(_1501) = & {l2479} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
                            // 1168: b"c read and se ... \n\0":   l2479 = UNIQUE | NON_NULL, FIXED
                            // 1168: b"c read and se ... \n\0": typeof(_1501 = const b"c read and set %d options\nc\n\x00") = & {l3644} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
                            // 1168: b"c read and se ... \n\0":   l3644 = UNIQUE | NON_NULL, (empty)
                            // 1168: b"c read and se ... \n\0": typeof(_1500 = &raw const (*_1501)) = *const {l3645} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
                            // 1168: b"c read and se ... \n\0":   l3645 = UNIQUE | NON_NULL, (empty)
                            // 1168: b"c read and se ... _char: typeof(_1498 = move _1499 as *const i8 (Misc)) = *const {l3647} i8
                            // 1168: b"c read and se ... _char:   l3647 = UNIQUE | NON_NULL, (empty)
                            // 1168: b"c read and se ... st u8: typeof(_1499 = move _1500 as *const u8 (Pointer(ArrayToPointer))) = *const {l3646} u8
                            // 1168: b"c read and se ... st u8:   l3646 = UNIQUE | NON_NULL, (empty)
                            nopts,
                        );
                        fflush(stdout);
                        // 1171: stdout: typeof(_1504) = *mut {l2483} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 1171: stdout:   l2483 = UNIQUE | NON_NULL, (empty)
                        // 1171: stdout: typeof(_1505) = *mut {l2485} *mut {l2486} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 1171: stdout:   l2485 = UNIQUE | NON_NULL, (empty)
                        // 1171: stdout:   l2486 = UNIQUE | NON_NULL, (empty)
                    }
                    fclose(pfile);
                    // 1173: pfile: typeof(_1507) = *mut {l2489} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                    // 1173: pfile:   l2489 = UNIQUE | NON_NULL, (empty)
                    current_block = 17418136423408909163;
                }
            } else {
                current_block = 17418136423408909163;
            }
            match current_block {
                14603147171032977705 => {}
                _ => {
                    if iname.is_null() {
                    // 1182: iname: typeof(_1510) = *const {l2493} i8
                    // 1182: iname:   l2493 = UNIQUE | NON_NULL, (empty)
                        iname = b"<stdin>\0" as *const u8 as *const libc::c_char;
                        // 1183: b"<stdin>\0" as ... st u8: typeof(_1511) = *const {l2495} u8
                        // 1183: b"<stdin>\0" as ... st u8:   l2495 = UNIQUE | NON_NULL, (empty)
                        // 1183: b"<stdin>\0": typeof(_1512) = *const {l2497} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                        // 1183: b"<stdin>\0":   l2497 = UNIQUE | NON_NULL, (empty)
                        // 1183: b"<stdin>\0": typeof(_1513) = & {l2499} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                        // 1183: b"<stdin>\0":   l2499 = UNIQUE | NON_NULL, FIXED
                        // 1183: b"<stdin>\0": typeof(_1513 = const b"<stdin>\x00") = & {l3648} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                        // 1183: b"<stdin>\0":   l3648 = UNIQUE | NON_NULL, (empty)
                        // 1183: iname = b"<stdi ... _char: typeof(_15 = move _1511 as *const i8 (Misc)) = *const {l3651} i8
                        // 1183: iname = b"<stdi ... _char:   l3651 = UNIQUE | NON_NULL, (empty)
                        // 1183: b"<stdin>\0": typeof(_1512 = &raw const (*_1513)) = *const {l3649} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                        // 1183: b"<stdin>\0":   l3649 = UNIQUE | NON_NULL, (empty)
                        // 1183: b"<stdin>\0" as ... st u8: typeof(_1511 = move _1512 as *const u8 (Pointer(ArrayToPointer))) = *const {l3650} u8
                        // 1183: b"<stdin>\0" as ... st u8:   l3650 = UNIQUE | NON_NULL, (empty)
                        err = lglparsefile(lgl, stdin, force, &mut lineno, &mut maxvar);
                        // 1184: lglparsefile(lg ... xvar): typeof(_1514) = *const {l2501} i8
                        // 1184: lglparsefile(lg ... xvar):   l2501 = UNIQUE | NON_NULL, (empty)
                        // 1184: lgl: typeof(_1515) = *mut {l2503} LGL
                        // 1184: lgl:   l2503 = UNIQUE | NON_NULL, (empty)
                        // 1184: stdin: typeof(_1516) = *mut {l2505} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 1184: stdin:   l2505 = UNIQUE | NON_NULL, (empty)
                        // 1184: stdin: typeof(_1517) = *mut {l2507} *mut {l2508} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                        // 1184: stdin:   l2507 = UNIQUE | NON_NULL, (empty)
                        // 1184: stdin:   l2508 = UNIQUE | NON_NULL, (empty)
                        // 1184: force: typeof(_1519) = *mut {l2511} i32
                        // 1184: force:   l2511 = UNIQUE | NON_NULL, (empty)
                        // 1184: &mut lineno: typeof(_1520) = *mut {l2513} i32
                        // 1184: &mut lineno:   l2513 = UNIQUE | NON_NULL, (empty)
                        // 1184: &mut lineno: typeof(_1521) = &mut {l2515} i32
                        // 1184: &mut lineno:   l2515 = UNIQUE | NON_NULL, (empty)
                        // 1184: &mut maxvar: typeof(_1522) = *mut {l2517} i32
                        // 1184: &mut maxvar:   l2517 = UNIQUE | NON_NULL, (empty)
                        // 1184: &mut maxvar: typeof(_1523) = &mut {l2519} i32
                        // 1184: &mut maxvar:   l2519 = UNIQUE | NON_NULL, (empty)
                        // 1184: &mut maxvar: typeof(_1522 = &raw mut (*_1523)) = *mut {l3655} i32
                        // 1184: &mut maxvar:   l3655 = UNIQUE | NON_NULL, (empty)
                        // 1184: &mut lineno: typeof(_1520 = &raw mut (*_1521)) = *mut {l3653} i32
                        // 1184: &mut lineno:   l3653 = UNIQUE | NON_NULL, (empty)
                        // 1184: &mut maxvar: typeof(_1523 = &mut _24) = &mut {l3654} i32
                        // 1184: &mut maxvar:   l3654 = UNIQUE | NON_NULL, (empty)
                        // 1184: &mut lineno: typeof(_1521 = &mut _11) = &mut {l3652} i32
                        // 1184: &mut lineno:   l3652 = UNIQUE | NON_NULL, (empty)
                    } else {
                        err = lglparsepath(lgl, iname, force, &mut lineno, &mut maxvar);
                        // 1186: lglparsepath(lg ... xvar): typeof(_1524) = *const {l2521} i8
                        // 1186: lglparsepath(lg ... xvar):   l2521 = UNIQUE | NON_NULL, (empty)
                        // 1186: lgl: typeof(_1525) = *mut {l2523} LGL
                        // 1186: lgl:   l2523 = UNIQUE | NON_NULL, (empty)
                        // 1186: iname: typeof(_1526) = *const {l2525} i8
                        // 1186: iname:   l2525 = UNIQUE | NON_NULL, (empty)
                        // 1186: force: typeof(_1528) = *mut {l2528} i32
                        // 1186: force:   l2528 = UNIQUE | NON_NULL, (empty)
                        // 1186: &mut lineno: typeof(_1529) = *mut {l2530} i32
                        // 1186: &mut lineno:   l2530 = UNIQUE | NON_NULL, (empty)
                        // 1186: &mut lineno: typeof(_1530) = &mut {l2532} i32
                        // 1186: &mut lineno:   l2532 = UNIQUE | NON_NULL, (empty)
                        // 1186: &mut maxvar: typeof(_1531) = *mut {l2534} i32
                        // 1186: &mut maxvar:   l2534 = UNIQUE | NON_NULL, (empty)
                        // 1186: &mut maxvar: typeof(_1532) = &mut {l2536} i32
                        // 1186: &mut maxvar:   l2536 = UNIQUE | NON_NULL, (empty)
                        // 1186: &mut maxvar: typeof(_1531 = &raw mut (*_1532)) = *mut {l3659} i32
                        // 1186: &mut maxvar:   l3659 = UNIQUE | NON_NULL, (empty)
                        // 1186: &mut lineno: typeof(_1530 = &mut _11) = &mut {l3656} i32
                        // 1186: &mut lineno:   l3656 = UNIQUE | NON_NULL, (empty)
                        // 1186: &mut maxvar: typeof(_1532 = &mut _24) = &mut {l3658} i32
                        // 1186: &mut maxvar:   l3658 = UNIQUE | NON_NULL, (empty)
                        // 1186: &mut lineno: typeof(_1529 = &raw mut (*_1530)) = *mut {l3657} i32
                        // 1186: &mut lineno:   l3657 = UNIQUE | NON_NULL, (empty)
                    }
                    if !err.is_null() {
                    // 1188: err: typeof(_1535) = *const {l2540} i8
                    // 1188: err:   l2540 = UNIQUE | NON_NULL, (empty)
                        fprintf(
                            stderr,
                            // 1190: stderr: typeof(_1537) = *mut {l2543} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                            // 1190: stderr:   l2543 = UNIQUE | NON_NULL, (empty)
                            // 1190: stderr: typeof(_1538) = *mut {l2545} *mut {l2546} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                            // 1190: stderr:   l2545 = UNIQUE | NON_NULL, (empty)
                            // 1190: stderr:   l2546 = UNIQUE | NON_NULL, (empty)
                            b"%s:%d: %s\n\0" as *const u8 as *const libc::c_char,
                            // 1191: b"%s:%d: %s\n\0 ... _char: typeof(_1539) = *const {l2548} i8
                            // 1191: b"%s:%d: %s\n\0 ... _char:   l2548 = UNIQUE | NON_NULL, (empty)
                            // 1191: b"%s:%d: %s\n\0 ... st u8: typeof(_1540) = *const {l2550} u8
                            // 1191: b"%s:%d: %s\n\0 ... st u8:   l2550 = UNIQUE | NON_NULL, (empty)
                            // 1191: b"%s:%d: %s\n\0": typeof(_1541) = *const {l2552} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                            // 1191: b"%s:%d: %s\n\0":   l2552 = UNIQUE | NON_NULL, (empty)
                            // 1191: b"%s:%d: %s\n\0": typeof(_1542) = & {l2554} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                            // 1191: b"%s:%d: %s\n\0":   l2554 = UNIQUE | NON_NULL, FIXED
                            // 1191: b"%s:%d: %s\n\0 ... st u8: typeof(_1540 = move _1541 as *const u8 (Pointer(ArrayToPointer))) = *const {l3662} u8
                            // 1191: b"%s:%d: %s\n\0 ... st u8:   l3662 = UNIQUE | NON_NULL, (empty)
                            // 1191: b"%s:%d: %s\n\0": typeof(_1542 = const b"%s:%d: %s\n\x00") = & {l3660} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                            // 1191: b"%s:%d: %s\n\0":   l3660 = UNIQUE | NON_NULL, (empty)
                            // 1191: b"%s:%d: %s\n\0": typeof(_1541 = &raw const (*_1542)) = *const {l3661} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                            // 1191: b"%s:%d: %s\n\0":   l3661 = UNIQUE | NON_NULL, (empty)
                            // 1191: b"%s:%d: %s\n\0 ... _char: typeof(_1539 = move _1540 as *const i8 (Misc)) = *const {l3663} i8
                            // 1191: b"%s:%d: %s\n\0 ... _char:   l3663 = UNIQUE | NON_NULL, (empty)
                            iname,
                            // 1192: iname: typeof(_1543) = *const {l2556} i8
                            // 1192: iname:   l2556 = UNIQUE | NON_NULL, (empty)
                            lineno,
                            err,
                            // 1194: err: typeof(_1545) = *const {l2559} i8
                            // 1194: err:   l2559 = UNIQUE | NON_NULL, (empty)
                        );
                        res = 1 as libc::c_int;
                    } else {
                        if verbose >= 1 as libc::c_int {
                        // 1198: verbose: typeof(_1550) = *mut {l2565} i32
                        // 1198: verbose:   l2565 = UNIQUE | NON_NULL, (empty)
                            printf(b"c\n\0" as *const u8 as *const libc::c_char);
                            // 1199: b"c\n\0" as *co ... _char: typeof(_1553) = *const {l2569} i8
                            // 1199: b"c\n\0" as *co ... _char:   l2569 = UNIQUE | NON_NULL, (empty)
                            // 1199: b"c\n\0" as *co ... st u8: typeof(_1554) = *const {l2571} u8
                            // 1199: b"c\n\0" as *co ... st u8:   l2571 = UNIQUE | NON_NULL, (empty)
                            // 1199: b"c\n\0": typeof(_1555) = *const {l2573} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                            // 1199: b"c\n\0":   l2573 = UNIQUE | NON_NULL, (empty)
                            // 1199: b"c\n\0": typeof(_1556) = & {l2575} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                            // 1199: b"c\n\0":   l2575 = UNIQUE | NON_NULL, FIXED
                            // 1199: b"c\n\0" as *co ... _char: typeof(_1553 = move _1554 as *const i8 (Misc)) = *const {l3667} i8
                            // 1199: b"c\n\0" as *co ... _char:   l3667 = UNIQUE | NON_NULL, (empty)
                            // 1199: b"c\n\0" as *co ... st u8: typeof(_1554 = move _1555 as *const u8 (Pointer(ArrayToPointer))) = *const {l3666} u8
                            // 1199: b"c\n\0" as *co ... st u8:   l3666 = UNIQUE | NON_NULL, (empty)
                            // 1199: b"c\n\0": typeof(_1556 = const b"c\n\x00") = & {l3664} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                            // 1199: b"c\n\0":   l3664 = UNIQUE | NON_NULL, (empty)
                            // 1199: b"c\n\0": typeof(_1555 = &raw const (*_1556)) = *const {l3665} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                            // 1199: b"c\n\0":   l3665 = UNIQUE | NON_NULL, (empty)
                            if verbose >= 2 as libc::c_int {
                            // 1200: verbose: typeof(_1560) = *mut {l2580} i32
                            // 1200: verbose:   l2580 = UNIQUE | NON_NULL, (empty)
                                printf(
                                    b"c final options:\nc\n\0" as *const u8 as *const libc::c_char,
                                    // 1202: b"c final optio ... _char: typeof(_1563) = *const {l2584} i8
                                    // 1202: b"c final optio ... _char:   l2584 = UNIQUE | NON_NULL, (empty)
                                    // 1202: b"c final optio ... st u8: typeof(_1564) = *const {l2586} u8
                                    // 1202: b"c final optio ... st u8:   l2586 = UNIQUE | NON_NULL, (empty)
                                    // 1202: b"c final optio ... \n\0": typeof(_1565) = *const {l2588} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                                    // 1202: b"c final optio ... \n\0":   l2588 = UNIQUE | NON_NULL, (empty)
                                    // 1202: b"c final optio ... \n\0": typeof(_1566) = & {l2590} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                                    // 1202: b"c final optio ... \n\0":   l2590 = UNIQUE | NON_NULL, FIXED
                                    // 1202: b"c final optio ... \n\0": typeof(_1565 = &raw const (*_1566)) = *const {l3669} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                                    // 1202: b"c final optio ... \n\0":   l3669 = UNIQUE | NON_NULL, (empty)
                                    // 1202: b"c final optio ... st u8: typeof(_1564 = move _1565 as *const u8 (Pointer(ArrayToPointer))) = *const {l3670} u8
                                    // 1202: b"c final optio ... st u8:   l3670 = UNIQUE | NON_NULL, (empty)
                                    // 1202: b"c final optio ... \n\0": typeof(_1566 = const b"c final options:\nc\n\x00") = & {l3668} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                                    // 1202: b"c final optio ... \n\0":   l3668 = UNIQUE | NON_NULL, (empty)
                                    // 1202: b"c final optio ... _char: typeof(_1563 = move _1564 as *const i8 (Misc)) = *const {l3671} i8
                                    // 1202: b"c final optio ... _char:   l3671 = UNIQUE | NON_NULL, (empty)
                                );
                            }
                            lglopts(
                                lgl,
                                // 1206: lgl: typeof(_1568) = *mut {l2593} LGL
                                // 1206: lgl:   l2593 = UNIQUE | NON_NULL, (empty)
                                b"c \0" as *const u8 as *const libc::c_char,
                                // 1207: b"c \0" as *con ... _char: typeof(_1569) = *const {l2595} i8
                                // 1207: b"c \0" as *con ... _char:   l2595 = UNIQUE | NON_NULL, (empty)
                                // 1207: b"c \0" as *const u8: typeof(_1570) = *const {l2597} u8
                                // 1207: b"c \0" as *const u8:   l2597 = UNIQUE | NON_NULL, (empty)
                                // 1207: b"c \0": typeof(_1571) = *const {l2599} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                                // 1207: b"c \0":   l2599 = UNIQUE | NON_NULL, (empty)
                                // 1207: b"c \0": typeof(_1572) = & {l2601} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                                // 1207: b"c \0":   l2601 = UNIQUE | NON_NULL, FIXED
                                // 1207: b"c \0": typeof(_1572 = const b"c \x00") = & {l3672} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                                // 1207: b"c \0":   l3672 = UNIQUE | NON_NULL, (empty)
                                // 1207: b"c \0": typeof(_1571 = &raw const (*_1572)) = *const {l3673} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                                // 1207: b"c \0":   l3673 = UNIQUE | NON_NULL, (empty)
                                // 1207: b"c \0" as *con ... _char: typeof(_1569 = move _1570 as *const i8 (Misc)) = *const {l3675} i8
                                // 1207: b"c \0" as *con ... _char:   l3675 = UNIQUE | NON_NULL, (empty)
                                // 1207: b"c \0" as *const u8: typeof(_1570 = move _1571 as *const u8 (Pointer(ArrayToPointer))) = *const {l3674} u8
                                // 1207: b"c \0" as *const u8:   l3674 = UNIQUE | NON_NULL, (empty)
                                0 as libc::c_int,
                            );
                        }
                        if timelimit >= 0 as libc::c_int {
                        // 1211: timelimit: typeof(_1577) = *mut {l2607} i32
                        // 1211: timelimit:   l2607 = UNIQUE | NON_NULL, (empty)
                            if verbose >= 0 as libc::c_int {
                            // 1212: verbose: typeof(_1582) = *mut {l2613} i32
                            // 1212: verbose:   l2613 = UNIQUE | NON_NULL, (empty)
                                printf(
                                    b"c\nc setting time limit of %d seconds\n\0" as *const u8
                                    // 1214: b"c\nc setting  ... _char: typeof(_1585) = *const {l2617} i8
                                    // 1214: b"c\nc setting  ... _char:   l2617 = UNIQUE | NON_NULL, (empty)
                                    // 1214: b"c\nc setting  ... st u8: typeof(_1586) = *const {l2619} u8
                                    // 1214: b"c\nc setting  ... st u8:   l2619 = UNIQUE | NON_NULL, (empty)
                                    // 1214: b"c\nc setting  ... \n\0": typeof(_1587) = *const {l2621} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                                    // 1214: b"c\nc setting  ... \n\0":   l2621 = UNIQUE | NON_NULL, (empty)
                                    // 1214: b"c\nc setting  ... \n\0": typeof(_1588) = & {l2623} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                                    // 1214: b"c\nc setting  ... \n\0":   l2623 = UNIQUE | NON_NULL, FIXED
                                    // 1214: b"c\nc setting  ... st u8: typeof(_1586 = move _1587 as *const u8 (Pointer(ArrayToPointer))) = *const {l3678} u8
                                    // 1214: b"c\nc setting  ... st u8:   l3678 = UNIQUE | NON_NULL, (empty)
                                    // 1214: b"c\nc setting  ... \n\0": typeof(_1588 = const b"c\nc setting time limit of %d seconds\n\x00") = & {l3676} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                                    // 1214: b"c\nc setting  ... \n\0":   l3676 = UNIQUE | NON_NULL, (empty)
                                    // 1214: b"c\nc setting  ... _char: typeof(_1585 = move _1586 as *const i8 (Misc)) = *const {l3679} i8
                                    // 1214: b"c\nc setting  ... _char:   l3679 = UNIQUE | NON_NULL, (empty)
                                    // 1214: b"c\nc setting  ... \n\0": typeof(_1587 = &raw const (*_1588)) = *const {l3677} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                                    // 1214: b"c\nc setting  ... \n\0":   l3677 = UNIQUE | NON_NULL, (empty)
                                        as *const libc::c_char,
                                    timelimit,
                                    // 1216: timelimit: typeof(_1590) = *mut {l2626} i32
                                    // 1216: timelimit:   l2626 = UNIQUE | NON_NULL, (empty)
                                );
                                fflush(stdout);
                                // 1218: stdout: typeof(_1592) = *mut {l2629} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                // 1218: stdout:   l2629 = UNIQUE | NON_NULL, (empty)
                                // 1218: stdout: typeof(_1593) = *mut {l2631} *mut {l2632} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                // 1218: stdout:   l2631 = UNIQUE | NON_NULL, (empty)
                                // 1218: stdout:   l2632 = UNIQUE | NON_NULL, (empty)
                            }
                            lglseterm(
                                lgl,
                                // 1221: lgl: typeof(_1595) = *mut {l2635} LGL
                                // 1221: lgl:   l2635 = UNIQUE | NON_NULL, (empty)
                                Some(
                                // 1222: Some( checkalar ... nt, ): typeof(_1596) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l2637} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32>
                                // 1222: Some( checkalar ... nt, ):   l2637 = UNIQUE | NON_NULL, (empty)
                                // 1222: Some( checkalar ... nt, ): typeof(_1596 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void) -> i32>::Some(move _1597)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l3681} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32>
                                // 1222: Some( checkalar ... nt, ):   l3681 = UNIQUE | NON_NULL, (empty)
                                    checkalarm_shim
                                    // 1223: checkalarm as u ... c_int: typeof(_1597) = fn(*mut {l2639} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32
                                    // 1223: checkalarm as u ... c_int:   l2639 = UNIQUE | NON_NULL, (empty)
                                    // 1223: checkalarm: typeof(_1597 = checkalarm as unsafe extern "C" fn(*mut libc::c_void) -> i32 (Pointer(ReifyFnPointer))) = fn(*mut {l3680} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32
                                    // 1223: checkalarm:   l3680 = UNIQUE | NON_NULL, (empty)
                                        as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
                                ),
                                &mut caughtalarm as *mut libc::c_int as *mut libc::c_void,
                                // 1226: &mut caughtalar ... _void: typeof(_1598) = *mut {l2641} DefId(2:5583 ~ core[480a]::ffi::c_void)
                                // 1226: &mut caughtalar ... _void:   l2641 = UNIQUE | NON_NULL, (empty)
                                // 1226: &mut caughtalar ... c_int: typeof(_1599) = *mut {l2643} i32
                                // 1226: &mut caughtalar ... c_int:   l2643 = UNIQUE | NON_NULL, (empty)
                                // 1226: &mut caughtalarm: typeof(_1600) = &mut {l2645} i32
                                // 1226: &mut caughtalarm:   l2645 = UNIQUE | NON_NULL, (empty)
                                // 1226: caughtalarm: typeof(_1601) = *mut {l2647} i32
                                // 1226: caughtalarm:   l2647 = UNIQUE | NON_NULL, (empty)
                                // 1226: &mut caughtalar ... _void: typeof(_1598 = move _1599 as *mut libc::c_void (Misc)) = *mut {l3684} DefId(2:5583 ~ core[480a]::ffi::c_void)
                                // 1226: &mut caughtalar ... _void:   l3684 = UNIQUE | NON_NULL, (empty)
                                // 1226: &mut caughtalarm: typeof(_1599 = &raw mut (*_1600)) = *mut {l3683} i32
                                // 1226: &mut caughtalarm:   l3683 = UNIQUE | NON_NULL, (empty)
                                // 1226: &mut caughtalarm: typeof(_1600 = &mut (*_1601)) = &mut {l3682} i32
                                // 1226: &mut caughtalarm:   l3682 = UNIQUE | NON_NULL, (empty)
                            );
                            sig_alrm_handler = signal(
                            // 1228: sig_alrm_handler: typeof(_1606) = *mut {l2653} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
                            // 1228: sig_alrm_handler:   l2653 = UNIQUE | NON_NULL, (empty)
                                14 as libc::c_int,
                                Some(catchalrm as unsafe extern "C" fn(libc::c_int) -> ()),
                            );
                            alarm(timelimit as libc::c_uint);
                            // 1232: timelimit: typeof(_1610) = *mut {l2658} i32
                            // 1232: timelimit:   l2658 = UNIQUE | NON_NULL, (empty)
                        }
                        i = 0 as libc::c_int;
                        while i < ntargets {
                        // 1235: ntargets: typeof(_1616) = *mut {l2665} i32
                        // 1235: ntargets:   l2665 = UNIQUE | NON_NULL, (empty)
                            lglassume(lgl, *targets.offset(i as isize));
                            // 1236: lgl: typeof(_1618) = *mut {l2668} LGL
                            // 1236: lgl:   l2668 = UNIQUE | NON_NULL, (empty)
                            // 1236: targets.offset( ... size): typeof(_1620) = *mut {l2671} i32
                            // 1236: targets.offset( ... size):   l2671 = UNIQUE | NON_NULL, (empty)
                            // 1236: targets: typeof(_1621) = *mut {l2673} i32
                            // 1236: targets:   l2673 = UNIQUE | NON_NULL, (empty)
                            // 1236: targets: typeof(_1622) = *mut {l2675} *mut {l2676} i32
                            // 1236: targets:   l2675 = UNIQUE | NON_NULL, (empty)
                            // 1236: targets:   l2676 = UNIQUE | NON_NULL, (empty)
                            i += 1;
                            i;
                        }
                        if simplevel > 0 as libc::c_int {
                            if verbose >= 1 as libc::c_int {
                            // 1241: verbose: typeof(_1637) = *mut {l2692} i32
                            // 1241: verbose:   l2692 = UNIQUE | NON_NULL, (empty)
                                printf(
                                    b"c simplifying with simplification level %d\n\0" as *const u8
                                    // 1243: b"c simplifying ... _char: typeof(_1640) = *const {l2696} i8
                                    // 1243: b"c simplifying ... _char:   l2696 = UNIQUE | NON_NULL, (empty)
                                    // 1243: b"c simplifying ... st u8: typeof(_1641) = *const {l2698} u8
                                    // 1243: b"c simplifying ... st u8:   l2698 = UNIQUE | NON_NULL, (empty)
                                    // 1243: b"c simplifying ... \n\0": typeof(_1642) = *const {l2700} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                                    // 1243: b"c simplifying ... \n\0":   l2700 = UNIQUE | NON_NULL, (empty)
                                    // 1243: b"c simplifying ... \n\0": typeof(_1643) = & {l2702} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                                    // 1243: b"c simplifying ... \n\0":   l2702 = UNIQUE | NON_NULL, FIXED
                                    // 1243: b"c simplifying ... _char: typeof(_1640 = move _1641 as *const i8 (Misc)) = *const {l3688} i8
                                    // 1243: b"c simplifying ... _char:   l3688 = UNIQUE | NON_NULL, (empty)
                                    // 1243: b"c simplifying ... \n\0": typeof(_1643 = const b"c simplifying with simplification level %d\n\x00") = & {l3685} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                                    // 1243: b"c simplifying ... \n\0":   l3685 = UNIQUE | NON_NULL, (empty)
                                    // 1243: b"c simplifying ... st u8: typeof(_1641 = move _1642 as *const u8 (Pointer(ArrayToPointer))) = *const {l3687} u8
                                    // 1243: b"c simplifying ... st u8:   l3687 = UNIQUE | NON_NULL, (empty)
                                    // 1243: b"c simplifying ... \n\0": typeof(_1642 = &raw const (*_1643)) = *const {l3686} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                                    // 1243: b"c simplifying ... \n\0":   l3686 = UNIQUE | NON_NULL, (empty)
                                        as *const libc::c_char,
                                    simplevel,
                                );
                                fflush(stdout);
                                // 1247: stdout: typeof(_1646) = *mut {l2706} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                // 1247: stdout:   l2706 = UNIQUE | NON_NULL, (empty)
                                // 1247: stdout: typeof(_1647) = *mut {l2708} *mut {l2709} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                // 1247: stdout:   l2708 = UNIQUE | NON_NULL, (empty)
                                // 1247: stdout:   l2709 = UNIQUE | NON_NULL, (empty)
                            }
                            res = lglsimp(lgl, simplevel);
                            // 1249: lgl: typeof(_1649) = *mut {l2712} LGL
                            // 1249: lgl:   l2712 = UNIQUE | NON_NULL, (empty)
                            if verbose >= 1 as libc::c_int {
                            // 1250: verbose: typeof(_1653) = *mut {l2717} i32
                            // 1250: verbose:   l2717 = UNIQUE | NON_NULL, (empty)
                                printf(
                                    b"c simplifying result %d after %.2f seconds\n\0" as *const u8
                                    // 1252: b"c simplifying ... _char: typeof(_1656) = *const {l2721} i8
                                    // 1252: b"c simplifying ... _char:   l2721 = UNIQUE | NON_NULL, (empty)
                                    // 1252: b"c simplifying ... st u8: typeof(_1657) = *const {l2723} u8
                                    // 1252: b"c simplifying ... st u8:   l2723 = UNIQUE | NON_NULL, (empty)
                                    // 1252: b"c simplifying ... \n\0": typeof(_1658) = *const {l2725} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                                    // 1252: b"c simplifying ... \n\0":   l2725 = UNIQUE | NON_NULL, (empty)
                                    // 1252: b"c simplifying ... \n\0": typeof(_1659) = & {l2727} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                                    // 1252: b"c simplifying ... \n\0":   l2727 = UNIQUE | NON_NULL, FIXED
                                    // 1252: b"c simplifying ... _char: typeof(_1656 = move _1657 as *const i8 (Misc)) = *const {l3692} i8
                                    // 1252: b"c simplifying ... _char:   l3692 = UNIQUE | NON_NULL, (empty)
                                    // 1252: b"c simplifying ... \n\0": typeof(_1659 = const b"c simplifying result %d after %.2f seconds\n\x00") = & {l3689} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                                    // 1252: b"c simplifying ... \n\0":   l3689 = UNIQUE | NON_NULL, (empty)
                                    // 1252: b"c simplifying ... st u8: typeof(_1657 = move _1658 as *const u8 (Pointer(ArrayToPointer))) = *const {l3691} u8
                                    // 1252: b"c simplifying ... st u8:   l3691 = UNIQUE | NON_NULL, (empty)
                                    // 1252: b"c simplifying ... \n\0": typeof(_1658 = &raw const (*_1659)) = *const {l3690} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                                    // 1252: b"c simplifying ... \n\0":   l3690 = UNIQUE | NON_NULL, (empty)
                                        as *const libc::c_char,
                                    res,
                                    lglsec(lgl),
                                    // 1255: lgl: typeof(_1662) = *mut {l2731} LGL
                                    // 1255: lgl:   l2731 = UNIQUE | NON_NULL, (empty)
                                );
                                fflush(stdout);
                                // 1257: stdout: typeof(_1664) = *mut {l2734} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                // 1257: stdout:   l2734 = UNIQUE | NON_NULL, (empty)
                                // 1257: stdout: typeof(_1665) = *mut {l2736} *mut {l2737} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                // 1257: stdout:   l2736 = UNIQUE | NON_NULL, (empty)
                                // 1257: stdout:   l2737 = UNIQUE | NON_NULL, (empty)
                            }
                        }
                        res = lglsat(lgl);
                        // 1260: lgl: typeof(_1667) = *mut {l2740} LGL
                        // 1260: lgl:   l2740 = UNIQUE | NON_NULL, (empty)
                        if timelimit >= 0 as libc::c_int {
                        // 1261: timelimit: typeof(_1671) = *mut {l2745} i32
                        // 1261: timelimit:   l2745 = UNIQUE | NON_NULL, (empty)
                            caughtalarm = 0 as libc::c_int;
                            // 1262: caughtalarm: typeof(_1674) = *mut {l2749} i32
                            // 1262: caughtalarm:   l2749 = UNIQUE | NON_NULL, (empty)
                            signal(14 as libc::c_int, sig_alrm_handler);
                            // 1263: sig_alrm_handler: typeof(_1678) = *mut {l2754} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
                            // 1263: sig_alrm_handler:   l2754 = UNIQUE | NON_NULL, (empty)
                        }
                        if !oname.is_null() {
                        // 1265: oname: typeof(_1682) = *const {l2759} i8
                        // 1265: oname:   l2759 = UNIQUE | NON_NULL, (empty)
                            let mut start: libc::c_double = lglsec(lgl);
                            // 1266: lgl: typeof(_1684) = *mut {l2762} LGL
                            // 1266: lgl:   l2762 = UNIQUE | NON_NULL, (empty)
                            let mut delta: libc::c_double = 0.;
                            if strcmp(oname, b"-\0" as *const u8 as *const libc::c_char) == 0 {
                            // 1268: oname: typeof(_1689) = *const {l2768} i8
                            // 1268: oname:   l2768 = UNIQUE | NON_NULL, (empty)
                            // 1268: b"-\0" as *cons ... _char: typeof(_1690) = *const {l2770} i8
                            // 1268: b"-\0" as *cons ... _char:   l2770 = UNIQUE | NON_NULL, (empty)
                            // 1268: b"-\0" as *const u8: typeof(_1691) = *const {l2772} u8
                            // 1268: b"-\0" as *const u8:   l2772 = UNIQUE | NON_NULL, (empty)
                            // 1268: b"-\0": typeof(_1692) = *const {l2774} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                            // 1268: b"-\0":   l2774 = UNIQUE | NON_NULL, (empty)
                            // 1268: b"-\0": typeof(_1693) = & {l2776} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                            // 1268: b"-\0":   l2776 = UNIQUE | NON_NULL, FIXED
                            // 1268: b"-\0" as *cons ... _char: typeof(_1690 = move _1691 as *const i8 (Misc)) = *const {l3696} i8
                            // 1268: b"-\0" as *cons ... _char:   l3696 = UNIQUE | NON_NULL, (empty)
                            // 1268: b"-\0" as *const u8: typeof(_1691 = move _1692 as *const u8 (Pointer(ArrayToPointer))) = *const {l3695} u8
                            // 1268: b"-\0" as *const u8:   l3695 = UNIQUE | NON_NULL, (empty)
                            // 1268: b"-\0": typeof(_1693 = const b"-\x00") = & {l3693} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                            // 1268: b"-\0":   l3693 = UNIQUE | NON_NULL, (empty)
                            // 1268: b"-\0": typeof(_1692 = &raw const (*_1693)) = *const {l3694} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                            // 1268: b"-\0":   l3694 = UNIQUE | NON_NULL, (empty)
                                out = stdout;
                                // 1269: stdout: typeof(_1694) = *mut {l2778} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                // 1269: stdout:   l2778 = UNIQUE | NON_NULL, (empty)
                                // 1269: stdout: typeof(_1695) = *mut {l2780} *mut {l2781} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                // 1269: stdout:   l2780 = UNIQUE | NON_NULL, (empty)
                                // 1269: stdout:   l2781 = UNIQUE | NON_NULL, (empty)
                                oname = b"<stdout>\0" as *const u8 as *const libc::c_char;
                                // 1270: b"<stdout>\0" a ... st u8: typeof(_1696) = *const {l2783} u8
                                // 1270: b"<stdout>\0" a ... st u8:   l2783 = UNIQUE | NON_NULL, (empty)
                                // 1270: b"<stdout>\0": typeof(_1697) = *const {l2785} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                                // 1270: b"<stdout>\0":   l2785 = UNIQUE | NON_NULL, (empty)
                                // 1270: b"<stdout>\0": typeof(_1698) = & {l2787} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                                // 1270: b"<stdout>\0":   l2787 = UNIQUE | NON_NULL, FIXED
                                // 1270: b"<stdout>\0": typeof(_1697 = &raw const (*_1698)) = *const {l3698} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                                // 1270: b"<stdout>\0":   l3698 = UNIQUE | NON_NULL, (empty)
                                // 1270: b"<stdout>\0": typeof(_1698 = const b"<stdout>\x00") = & {l3697} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                                // 1270: b"<stdout>\0":   l3697 = UNIQUE | NON_NULL, (empty)
                                // 1270: oname = b"<stdo ... _char: typeof(_16 = move _1696 as *const i8 (Misc)) = *const {l3700} i8
                                // 1270: oname = b"<stdo ... _char:   l3700 = UNIQUE | NON_NULL, (empty)
                                // 1270: b"<stdout>\0" a ... st u8: typeof(_1696 = move _1697 as *const u8 (Pointer(ArrayToPointer))) = *const {l3699} u8
                                // 1270: b"<stdout>\0" a ... st u8:   l3699 = UNIQUE | NON_NULL, (empty)
                                clout = 0 as libc::c_int;
                                current_block = 10528013381497917728;
                            } else {
                                out = writefile(oname, &mut clout);
                                // 1274: writefile(oname ... lout): typeof(_1700) = *mut {l2790} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                // 1274: writefile(oname ... lout):   l2790 = UNIQUE | NON_NULL, (empty)
                                // 1274: oname: typeof(_1701) = *const {l2792} i8
                                // 1274: oname:   l2792 = UNIQUE | NON_NULL, (empty)
                                // 1274: &mut clout: typeof(_1702) = *mut {l2794} i32
                                // 1274: &mut clout:   l2794 = UNIQUE | NON_NULL, (empty)
                                // 1274: &mut clout: typeof(_1703) = &mut {l2796} i32
                                // 1274: &mut clout:   l2796 = UNIQUE | NON_NULL, (empty)
                                // 1274: &mut clout: typeof(_1703 = &mut _8) = &mut {l3701} i32
                                // 1274: &mut clout:   l3701 = UNIQUE | NON_NULL, (empty)
                                // 1274: &mut clout: typeof(_1702 = &raw mut (*_1703)) = *mut {l3702} i32
                                // 1274: &mut clout:   l3702 = UNIQUE | NON_NULL, (empty)
                                if out.is_null() {
                                // 1275: out: typeof(_1705) = *mut {l2799} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                // 1275: out:   l2799 = UNIQUE | NON_NULL, (empty)
                                    res = 1 as libc::c_int;
                                    current_block = 14603147171032977705;
                                } else {
                                    current_block = 10528013381497917728;
                                }
                            }
                            match current_block {
                                14603147171032977705 => {}
                                _ => {
                                    if verbose >= 0 as libc::c_int {
                                    // 1285: verbose: typeof(_1710) = *mut {l2805} i32
                                    // 1285: verbose:   l2805 = UNIQUE | NON_NULL, (empty)
                                        count = 0 as libc::c_int;
                                        lglctrav(
                                            lgl,
                                            // 1288: lgl: typeof(_1714) = *mut {l2810} LGL
                                            // 1288: lgl:   l2810 = UNIQUE | NON_NULL, (empty)
                                            &mut count as *mut libc::c_int as *mut libc::c_void,
                                            // 1289: &mut count as * ... _void: typeof(_1715) = *mut {l2812} DefId(2:5583 ~ core[480a]::ffi::c_void)
                                            // 1289: &mut count as * ... _void:   l2812 = UNIQUE | NON_NULL, (empty)
                                            // 1289: &mut count as * ... c_int: typeof(_1716) = *mut {l2814} i32
                                            // 1289: &mut count as * ... c_int:   l2814 = UNIQUE | NON_NULL, (empty)
                                            // 1289: &mut count: typeof(_1717) = &mut {l2816} i32
                                            // 1289: &mut count:   l2816 = UNIQUE | NON_NULL, (empty)
                                            // 1289: &mut count as * ... _void: typeof(_1715 = move _1716 as *mut libc::c_void (Misc)) = *mut {l3705} DefId(2:5583 ~ core[480a]::ffi::c_void)
                                            // 1289: &mut count as * ... _void:   l3705 = UNIQUE | NON_NULL, (empty)
                                            // 1289: &mut count: typeof(_1717 = &mut _13) = &mut {l3703} i32
                                            // 1289: &mut count:   l3703 = UNIQUE | NON_NULL, (empty)
                                            // 1289: &mut count: typeof(_1716 = &raw mut (*_1717)) = *mut {l3704} i32
                                            // 1289: &mut count:   l3704 = UNIQUE | NON_NULL, (empty)
                                            Some(
                                            // 1290: Some( lgltravco ... (), ): typeof(_1718) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l2818} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()>
                                            // 1290: Some( lgltravco ... (), ):   l2818 = UNIQUE | NON_NULL, (empty)
                                            // 1290: Some( lgltravco ... (), ): typeof(_1718 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void, i32)>::Some(move _1719)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l3707} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()>
                                            // 1290: Some( lgltravco ... (), ):   l3707 = UNIQUE | NON_NULL, (empty)
                                                lgltravcounter_shim
                                                // 1291: lgltravcounter  ... -> (): typeof(_1719) = fn(*mut {l2820} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()
                                                // 1291: lgltravcounter  ... -> ():   l2820 = UNIQUE | NON_NULL, (empty)
                                                // 1291: lgltravcounter: typeof(_1719 = lgltravcounter as unsafe extern "C" fn(*mut libc::c_void, i32) (Pointer(ReifyFnPointer))) = fn(*mut {l3706} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()
                                                // 1291: lgltravcounter:   l3706 = UNIQUE | NON_NULL, (empty)
                                                    as unsafe extern "C" fn(
                                                        *mut libc::c_void,
                                                        libc::c_int,
                                                    )
                                                        -> (),
                                            ),
                                        );
                                        printf(
                                            b"c\nc writing 'p cnf %d %d' to '%s'\n\0" as *const u8
                                            // 1300: b"c\nc writing  ... _char: typeof(_1721) = *const {l2823} i8
                                            // 1300: b"c\nc writing  ... _char:   l2823 = UNIQUE | NON_NULL, (empty)
                                            // 1300: b"c\nc writing  ... st u8: typeof(_1722) = *const {l2825} u8
                                            // 1300: b"c\nc writing  ... st u8:   l2825 = UNIQUE | NON_NULL, (empty)
                                            // 1300: b"c\nc writing  ... \n\0": typeof(_1723) = *const {l2827} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
                                            // 1300: b"c\nc writing  ... \n\0":   l2827 = UNIQUE | NON_NULL, (empty)
                                            // 1300: b"c\nc writing  ... \n\0": typeof(_1724) = & {l2829} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
                                            // 1300: b"c\nc writing  ... \n\0":   l2829 = UNIQUE | NON_NULL, FIXED
                                            // 1300: b"c\nc writing  ... st u8: typeof(_1722 = move _1723 as *const u8 (Pointer(ArrayToPointer))) = *const {l3710} u8
                                            // 1300: b"c\nc writing  ... st u8:   l3710 = UNIQUE | NON_NULL, (empty)
                                            // 1300: b"c\nc writing  ... _char: typeof(_1721 = move _1722 as *const i8 (Misc)) = *const {l3711} i8
                                            // 1300: b"c\nc writing  ... _char:   l3711 = UNIQUE | NON_NULL, (empty)
                                            // 1300: b"c\nc writing  ... \n\0": typeof(_1723 = &raw const (*_1724)) = *const {l3709} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
                                            // 1300: b"c\nc writing  ... \n\0":   l3709 = UNIQUE | NON_NULL, (empty)
                                            // 1300: b"c\nc writing  ... \n\0": typeof(_1724 = const b"c\nc writing \'p cnf %d %d\' to \'%s\'\n\x00") = & {l3708} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
                                            // 1300: b"c\nc writing  ... \n\0":   l3708 = UNIQUE | NON_NULL, (empty)
                                                as *const libc::c_char,
                                            maxvar,
                                            count,
                                            oname,
                                            // 1304: oname: typeof(_1727) = *const {l2833} i8
                                            // 1304: oname:   l2833 = UNIQUE | NON_NULL, (empty)
                                        );
                                        fflush(stdout);
                                        // 1306: stdout: typeof(_1729) = *mut {l2836} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                        // 1306: stdout:   l2836 = UNIQUE | NON_NULL, (empty)
                                        // 1306: stdout: typeof(_1730) = *mut {l2838} *mut {l2839} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                        // 1306: stdout:   l2838 = UNIQUE | NON_NULL, (empty)
                                        // 1306: stdout:   l2839 = UNIQUE | NON_NULL, (empty)
                                    }
                                    lglprint(lgl, out);
                                    // 1308: lgl: typeof(_1732) = *mut {l2842} LGL
                                    // 1308: lgl:   l2842 = UNIQUE | NON_NULL, (empty)
                                    // 1308: out: typeof(_1733) = *mut {l2844} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                    // 1308: out:   l2844 = UNIQUE | NON_NULL, (empty)
                                    closefile(out, clout);
                                    // 1309: out: typeof(_1735) = *mut {l2847} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                    // 1309: out:   l2847 = UNIQUE | NON_NULL, (empty)
                                    if verbose >= 0 as libc::c_int {
                                    // 1310: verbose: typeof(_1740) = *mut {l2853} i32
                                    // 1310: verbose:   l2853 = UNIQUE | NON_NULL, (empty)
                                        delta = lglsec(lgl) - start;
                                        // 1311: lgl: typeof(_1743) = *mut {l2857} LGL
                                        // 1311: lgl:   l2857 = UNIQUE | NON_NULL, (empty)
                                        if delta < 0 as libc::c_int as libc::c_double {
                                            delta = 0 as libc::c_int as libc::c_double;
                                        }
                                        printf(
                                            b"c collected garbage and wrote '%s' in %.1f seconds\n\0"
                                            // 1316: b"c collected g ... _char: typeof(_1752) = *const {l2867} i8
                                            // 1316: b"c collected g ... _char:   l2867 = UNIQUE | NON_NULL, (empty)
                                            // 1316: b"c collected g ... st u8: typeof(_1753) = *const {l2869} u8
                                            // 1316: b"c collected g ... st u8:   l2869 = UNIQUE | NON_NULL, (empty)
                                            // 1316: b"c collected g ... \n\0": typeof(_1754) = *const {l2871} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                                            // 1316: b"c collected g ... \n\0":   l2871 = UNIQUE | NON_NULL, (empty)
                                            // 1316: b"c collected g ... \n\0": typeof(_1755) = & {l2873} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                                            // 1316: b"c collected g ... \n\0":   l2873 = UNIQUE | NON_NULL, FIXED
                                            // 1316: b"c collected g ... st u8: typeof(_1753 = move _1754 as *const u8 (Pointer(ArrayToPointer))) = *const {l3714} u8
                                            // 1316: b"c collected g ... st u8:   l3714 = UNIQUE | NON_NULL, (empty)
                                            // 1316: b"c collected g ... \n\0": typeof(_1755 = const b"c collected garbage and wrote \'%s\' in %.1f seconds\n\x00") = & {l3712} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                                            // 1316: b"c collected g ... \n\0":   l3712 = UNIQUE | NON_NULL, (empty)
                                            // 1316: b"c collected g ... \n\0": typeof(_1754 = &raw const (*_1755)) = *const {l3713} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
                                            // 1316: b"c collected g ... \n\0":   l3713 = UNIQUE | NON_NULL, (empty)
                                            // 1316: b"c collected g ... _char: typeof(_1752 = move _1753 as *const i8 (Misc)) = *const {l3715} i8
                                            // 1316: b"c collected g ... _char:   l3715 = UNIQUE | NON_NULL, (empty)
                                                as *const u8 as *const libc::c_char,
                                            oname,
                                            // 1318: oname: typeof(_1756) = *const {l2875} i8
                                            // 1318: oname:   l2875 = UNIQUE | NON_NULL, (empty)
                                            delta,
                                        );
                                        printf(b"c\n\0" as *const u8 as *const libc::c_char);
                                        // 1321: b"c\n\0" as *co ... _char: typeof(_1759) = *const {l2879} i8
                                        // 1321: b"c\n\0" as *co ... _char:   l2879 = UNIQUE | NON_NULL, (empty)
                                        // 1321: b"c\n\0" as *co ... st u8: typeof(_1760) = *const {l2881} u8
                                        // 1321: b"c\n\0" as *co ... st u8:   l2881 = UNIQUE | NON_NULL, (empty)
                                        // 1321: b"c\n\0": typeof(_1761) = *const {l2883} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                                        // 1321: b"c\n\0":   l2883 = UNIQUE | NON_NULL, (empty)
                                        // 1321: b"c\n\0": typeof(_1762) = & {l2885} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                                        // 1321: b"c\n\0":   l2885 = UNIQUE | NON_NULL, FIXED
                                        // 1321: b"c\n\0": typeof(_1762 = const b"c\n\x00") = & {l3716} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                                        // 1321: b"c\n\0":   l3716 = UNIQUE | NON_NULL, (empty)
                                        // 1321: b"c\n\0": typeof(_1761 = &raw const (*_1762)) = *const {l3717} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                                        // 1321: b"c\n\0":   l3717 = UNIQUE | NON_NULL, (empty)
                                        // 1321: b"c\n\0" as *co ... st u8: typeof(_1760 = move _1761 as *const u8 (Pointer(ArrayToPointer))) = *const {l3718} u8
                                        // 1321: b"c\n\0" as *co ... st u8:   l3718 = UNIQUE | NON_NULL, (empty)
                                        // 1321: b"c\n\0" as *co ... _char: typeof(_1759 = move _1760 as *const i8 (Misc)) = *const {l3719} i8
                                        // 1321: b"c\n\0" as *co ... _char:   l3719 = UNIQUE | NON_NULL, (empty)
                                        fflush(stdout);
                                        // 1322: stdout: typeof(_1764) = *mut {l2888} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                        // 1322: stdout:   l2888 = UNIQUE | NON_NULL, (empty)
                                        // 1322: stdout: typeof(_1765) = *mut {l2890} *mut {l2891} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                        // 1322: stdout:   l2890 = UNIQUE | NON_NULL, (empty)
                                        // 1322: stdout:   l2891 = UNIQUE | NON_NULL, (empty)
                                    }
                                    current_block = 4533671380017093834;
                                }
                            }
                        } else {
                            current_block = 4533671380017093834;
                        }
                        match current_block {
                            14603147171032977705 => {}
                            _ => {
                                if simponly == 0 || verbose >= 0 as libc::c_int {
                                // 1333: verbose: typeof(_1772) = *mut {l2899} i32
                                // 1333: verbose:   l2899 = UNIQUE | NON_NULL, (empty)
                                    if simponly != 0 {
                                        fputs(b"c \0" as *const u8 as *const libc::c_char, stdout);
                                        // 1335: b"c \0" as *con ... _char: typeof(_1778) = *const {l2906} i8
                                        // 1335: b"c \0" as *con ... _char:   l2906 = UNIQUE | NON_NULL, (empty)
                                        // 1335: b"c \0" as *const u8: typeof(_1779) = *const {l2908} u8
                                        // 1335: b"c \0" as *const u8:   l2908 = UNIQUE | NON_NULL, (empty)
                                        // 1335: b"c \0": typeof(_1780) = *const {l2910} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                                        // 1335: b"c \0":   l2910 = UNIQUE | NON_NULL, (empty)
                                        // 1335: b"c \0": typeof(_1781) = & {l2912} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                                        // 1335: b"c \0":   l2912 = UNIQUE | NON_NULL, FIXED
                                        // 1335: stdout: typeof(_1782) = *mut {l2914} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                        // 1335: stdout:   l2914 = UNIQUE | NON_NULL, (empty)
                                        // 1335: stdout: typeof(_1783) = *mut {l2916} *mut {l2917} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                        // 1335: stdout:   l2916 = UNIQUE | NON_NULL, (empty)
                                        // 1335: stdout:   l2917 = UNIQUE | NON_NULL, (empty)
                                        // 1335: b"c \0" as *const u8: typeof(_1779 = move _1780 as *const u8 (Pointer(ArrayToPointer))) = *const {l3722} u8
                                        // 1335: b"c \0" as *const u8:   l3722 = UNIQUE | NON_NULL, (empty)
                                        // 1335: b"c \0": typeof(_1781 = const b"c \x00") = & {l3720} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                                        // 1335: b"c \0":   l3720 = UNIQUE | NON_NULL, (empty)
                                        // 1335: b"c \0" as *con ... _char: typeof(_1778 = move _1779 as *const i8 (Misc)) = *const {l3723} i8
                                        // 1335: b"c \0" as *con ... _char:   l3723 = UNIQUE | NON_NULL, (empty)
                                        // 1335: b"c \0": typeof(_1780 = &raw const (*_1781)) = *const {l3721} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                                        // 1335: b"c \0":   l3721 = UNIQUE | NON_NULL, (empty)
                                    }
                                    if res == 10 as libc::c_int {
                                        fputs(
                                            b"s SATISFIABLE\n\0" as *const u8
                                            // 1339: b"s SATISFIABLE ... _char: typeof(_1789) = *const {l2924} i8
                                            // 1339: b"s SATISFIABLE ... _char:   l2924 = UNIQUE | NON_NULL, (empty)
                                            // 1339: b"s SATISFIABLE ... st u8: typeof(_1790) = *const {l2926} u8
                                            // 1339: b"s SATISFIABLE ... st u8:   l2926 = UNIQUE | NON_NULL, (empty)
                                            // 1339: b"s SATISFIABLE\n\0": typeof(_1791) = *const {l2928} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                            // 1339: b"s SATISFIABLE\n\0":   l2928 = UNIQUE | NON_NULL, (empty)
                                            // 1339: b"s SATISFIABLE\n\0": typeof(_1792) = & {l2930} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                            // 1339: b"s SATISFIABLE\n\0":   l2930 = UNIQUE | NON_NULL, FIXED
                                            // 1339: b"s SATISFIABLE ... st u8: typeof(_1790 = move _1791 as *const u8 (Pointer(ArrayToPointer))) = *const {l3726} u8
                                            // 1339: b"s SATISFIABLE ... st u8:   l3726 = UNIQUE | NON_NULL, (empty)
                                            // 1339: b"s SATISFIABLE ... _char: typeof(_1789 = move _1790 as *const i8 (Misc)) = *const {l3727} i8
                                            // 1339: b"s SATISFIABLE ... _char:   l3727 = UNIQUE | NON_NULL, (empty)
                                            // 1339: b"s SATISFIABLE\n\0": typeof(_1791 = &raw const (*_1792)) = *const {l3725} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                            // 1339: b"s SATISFIABLE\n\0":   l3725 = UNIQUE | NON_NULL, (empty)
                                            // 1339: b"s SATISFIABLE\n\0": typeof(_1792 = const b"s SATISFIABLE\n\x00") = & {l3724} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                            // 1339: b"s SATISFIABLE\n\0":   l3724 = UNIQUE | NON_NULL, (empty)
                                                as *const libc::c_char,
                                            stdout,
                                            // 1341: stdout: typeof(_1793) = *mut {l2932} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                            // 1341: stdout:   l2932 = UNIQUE | NON_NULL, (empty)
                                            // 1341: stdout: typeof(_1794) = *mut {l2934} *mut {l2935} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                            // 1341: stdout:   l2934 = UNIQUE | NON_NULL, (empty)
                                            // 1341: stdout:   l2935 = UNIQUE | NON_NULL, (empty)
                                        );
                                    } else if res == 20 as libc::c_int {
                                        fputs(
                                            b"s UNSATISFIABLE\n\0" as *const u8
                                            // 1345: b"s UNSATISFIAB ... _char: typeof(_1799) = *const {l2941} i8
                                            // 1345: b"s UNSATISFIAB ... _char:   l2941 = UNIQUE | NON_NULL, (empty)
                                            // 1345: b"s UNSATISFIAB ... st u8: typeof(_1800) = *const {l2943} u8
                                            // 1345: b"s UNSATISFIAB ... st u8:   l2943 = UNIQUE | NON_NULL, (empty)
                                            // 1345: b"s UNSATISFIAB ... \n\0": typeof(_1801) = *const {l2945} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                                            // 1345: b"s UNSATISFIAB ... \n\0":   l2945 = UNIQUE | NON_NULL, (empty)
                                            // 1345: b"s UNSATISFIAB ... \n\0": typeof(_1802) = & {l2947} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                                            // 1345: b"s UNSATISFIAB ... \n\0":   l2947 = UNIQUE | NON_NULL, FIXED
                                            // 1345: b"s UNSATISFIAB ... st u8: typeof(_1800 = move _1801 as *const u8 (Pointer(ArrayToPointer))) = *const {l3730} u8
                                            // 1345: b"s UNSATISFIAB ... st u8:   l3730 = UNIQUE | NON_NULL, (empty)
                                            // 1345: b"s UNSATISFIAB ... _char: typeof(_1799 = move _1800 as *const i8 (Misc)) = *const {l3731} i8
                                            // 1345: b"s UNSATISFIAB ... _char:   l3731 = UNIQUE | NON_NULL, (empty)
                                            // 1345: b"s UNSATISFIAB ... \n\0": typeof(_1802 = const b"s UNSATISFIABLE\n\x00") = & {l3728} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                                            // 1345: b"s UNSATISFIAB ... \n\0":   l3728 = UNIQUE | NON_NULL, (empty)
                                            // 1345: b"s UNSATISFIAB ... \n\0": typeof(_1801 = &raw const (*_1802)) = *const {l3729} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                                            // 1345: b"s UNSATISFIAB ... \n\0":   l3729 = UNIQUE | NON_NULL, (empty)
                                                as *const libc::c_char,
                                            stdout,
                                            // 1347: stdout: typeof(_1803) = *mut {l2949} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                            // 1347: stdout:   l2949 = UNIQUE | NON_NULL, (empty)
                                            // 1347: stdout: typeof(_1804) = *mut {l2951} *mut {l2952} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                            // 1347: stdout:   l2951 = UNIQUE | NON_NULL, (empty)
                                            // 1347: stdout:   l2952 = UNIQUE | NON_NULL, (empty)
                                        );
                                    } else {
                                        fputs(
                                            b"c s UNKNOWN\n\0" as *const u8 as *const libc::c_char,
                                            // 1351: b"c s UNKNOWN\n ... _char: typeof(_1806) = *const {l2955} i8
                                            // 1351: b"c s UNKNOWN\n ... _char:   l2955 = UNIQUE | NON_NULL, (empty)
                                            // 1351: b"c s UNKNOWN\n ... st u8: typeof(_1807) = *const {l2957} u8
                                            // 1351: b"c s UNKNOWN\n ... st u8:   l2957 = UNIQUE | NON_NULL, (empty)
                                            // 1351: b"c s UNKNOWN\n\0": typeof(_1808) = *const {l2959} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                            // 1351: b"c s UNKNOWN\n\0":   l2959 = UNIQUE | NON_NULL, (empty)
                                            // 1351: b"c s UNKNOWN\n\0": typeof(_1809) = & {l2961} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                            // 1351: b"c s UNKNOWN\n\0":   l2961 = UNIQUE | NON_NULL, FIXED
                                            // 1351: b"c s UNKNOWN\n\0": typeof(_1809 = const b"c s UNKNOWN\n\x00") = & {l3732} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                            // 1351: b"c s UNKNOWN\n\0":   l3732 = UNIQUE | NON_NULL, (empty)
                                            // 1351: b"c s UNKNOWN\n ... st u8: typeof(_1807 = move _1808 as *const u8 (Pointer(ArrayToPointer))) = *const {l3734} u8
                                            // 1351: b"c s UNKNOWN\n ... st u8:   l3734 = UNIQUE | NON_NULL, (empty)
                                            // 1351: b"c s UNKNOWN\n\0": typeof(_1808 = &raw const (*_1809)) = *const {l3733} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                            // 1351: b"c s UNKNOWN\n\0":   l3733 = UNIQUE | NON_NULL, (empty)
                                            // 1351: b"c s UNKNOWN\n ... _char: typeof(_1806 = move _1807 as *const i8 (Misc)) = *const {l3735} i8
                                            // 1351: b"c s UNKNOWN\n ... _char:   l3735 = UNIQUE | NON_NULL, (empty)
                                            stdout,
                                            // 1352: stdout: typeof(_1810) = *mut {l2963} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                            // 1352: stdout:   l2963 = UNIQUE | NON_NULL, (empty)
                                            // 1352: stdout: typeof(_1811) = *mut {l2965} *mut {l2966} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                            // 1352: stdout:   l2965 = UNIQUE | NON_NULL, (empty)
                                            // 1352: stdout:   l2966 = UNIQUE | NON_NULL, (empty)
                                        );
                                    }
                                    if !thanks.is_null() {
                                    // 1355: thanks: typeof(_1815) = *const {l2971} i8
                                    // 1355: thanks:   l2971 = UNIQUE | NON_NULL, (empty)
                                        printf(
                                            b"c\nc Thanks to %s!\nc\n\0" as *const u8
                                            // 1357: b"c\nc Thanks t ... _char: typeof(_1817) = *const {l2974} i8
                                            // 1357: b"c\nc Thanks t ... _char:   l2974 = UNIQUE | NON_NULL, (empty)
                                            // 1357: b"c\nc Thanks t ... st u8: typeof(_1818) = *const {l2976} u8
                                            // 1357: b"c\nc Thanks t ... st u8:   l2976 = UNIQUE | NON_NULL, (empty)
                                            // 1357: b"c\nc Thanks t ... \n\0": typeof(_1819) = *const {l2978} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                                            // 1357: b"c\nc Thanks t ... \n\0":   l2978 = UNIQUE | NON_NULL, (empty)
                                            // 1357: b"c\nc Thanks t ... \n\0": typeof(_1820) = & {l2980} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                                            // 1357: b"c\nc Thanks t ... \n\0":   l2980 = UNIQUE | NON_NULL, FIXED
                                            // 1357: b"c\nc Thanks t ... \n\0": typeof(_1820 = const b"c\nc Thanks to %s!\nc\n\x00") = & {l3736} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                                            // 1357: b"c\nc Thanks t ... \n\0":   l3736 = UNIQUE | NON_NULL, (empty)
                                            // 1357: b"c\nc Thanks t ... _char: typeof(_1817 = move _1818 as *const i8 (Misc)) = *const {l3739} i8
                                            // 1357: b"c\nc Thanks t ... _char:   l3739 = UNIQUE | NON_NULL, (empty)
                                            // 1357: b"c\nc Thanks t ... \n\0": typeof(_1819 = &raw const (*_1820)) = *const {l3737} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                                            // 1357: b"c\nc Thanks t ... \n\0":   l3737 = UNIQUE | NON_NULL, (empty)
                                            // 1357: b"c\nc Thanks t ... st u8: typeof(_1818 = move _1819 as *const u8 (Pointer(ArrayToPointer))) = *const {l3738} u8
                                            // 1357: b"c\nc Thanks t ... st u8:   l3738 = UNIQUE | NON_NULL, (empty)
                                                as *const libc::c_char,
                                            thanks,
                                            // 1359: thanks: typeof(_1821) = *const {l2982} i8
                                            // 1359: thanks:   l2982 = UNIQUE | NON_NULL, (empty)
                                        );
                                    }
                                    fflush(stdout);
                                    // 1362: stdout: typeof(_1823) = *mut {l2985} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                    // 1362: stdout:   l2985 = UNIQUE | NON_NULL, (empty)
                                    // 1362: stdout: typeof(_1824) = *mut {l2987} *mut {l2988} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                    // 1362: stdout:   l2987 = UNIQUE | NON_NULL, (empty)
                                    // 1362: stdout:   l2988 = UNIQUE | NON_NULL, (empty)
                                    if res == 10 as libc::c_int
                                        && lglgetopt(
                                            lgl,
                                            // 1365: lgl: typeof(_1831) = *mut {l2996} LGL
                                            // 1365: lgl:   l2996 = UNIQUE | NON_NULL, (empty)
                                            b"witness\0" as *const u8 as *const libc::c_char,
                                            // 1366: b"witness\0" as ... _char: typeof(_1832) = *const {l2998} i8
                                            // 1366: b"witness\0" as ... _char:   l2998 = UNIQUE | NON_NULL, (empty)
                                            // 1366: b"witness\0" as ... st u8: typeof(_1833) = *const {l3000} u8
                                            // 1366: b"witness\0" as ... st u8:   l3000 = UNIQUE | NON_NULL, (empty)
                                            // 1366: b"witness\0": typeof(_1834) = *const {l3002} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                                            // 1366: b"witness\0":   l3002 = UNIQUE | NON_NULL, (empty)
                                            // 1366: b"witness\0": typeof(_1835) = & {l3004} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                                            // 1366: b"witness\0":   l3004 = UNIQUE | NON_NULL, FIXED
                                            // 1366: b"witness\0" as ... st u8: typeof(_1833 = move _1834 as *const u8 (Pointer(ArrayToPointer))) = *const {l3742} u8
                                            // 1366: b"witness\0" as ... st u8:   l3742 = UNIQUE | NON_NULL, (empty)
                                            // 1366: b"witness\0": typeof(_1834 = &raw const (*_1835)) = *const {l3741} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                                            // 1366: b"witness\0":   l3741 = UNIQUE | NON_NULL, (empty)
                                            // 1366: b"witness\0" as ... _char: typeof(_1832 = move _1833 as *const i8 (Misc)) = *const {l3743} i8
                                            // 1366: b"witness\0" as ... _char:   l3743 = UNIQUE | NON_NULL, (empty)
                                            // 1366: b"witness\0": typeof(_1835 = const b"witness\x00") = & {l3740} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                                            // 1366: b"witness\0":   l3740 = UNIQUE | NON_NULL, (empty)
                                        ) != 0
                                    {
                                        obuf.pos = 0 as libc::c_int;
                                        i = 1 as libc::c_int;
                                        while i <= maxvar {
                                            lit = if lglderef(lgl, i) > 0 as libc::c_int {
                                            // 1372: lgl: typeof(_1845) = *mut {l3015} LGL
                                            // 1372: lgl:   l3015 = UNIQUE | NON_NULL, (empty)
                                                i
                                            } else {
                                                -i
                                            };
                                            print2obuf(&mut obuf, lit, simponly, stdout);
                                            // 1377: &mut obuf: typeof(_1851) = *mut {l3022} DefId(0:160 ~ lglmain[0c57]::OBuf)
                                            // 1377: &mut obuf:   l3022 = UNIQUE | NON_NULL, (empty)
                                            // 1377: &mut obuf: typeof(_1852) = &mut {l3024} DefId(0:160 ~ lglmain[0c57]::OBuf)
                                            // 1377: &mut obuf:   l3024 = UNIQUE | NON_NULL, (empty)
                                            // 1377: stdout: typeof(_1855) = *mut {l3028} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                            // 1377: stdout:   l3028 = UNIQUE | NON_NULL, (empty)
                                            // 1377: stdout: typeof(_1856) = *mut {l3030} *mut {l3031} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                            // 1377: stdout:   l3030 = UNIQUE | NON_NULL, (empty)
                                            // 1377: stdout:   l3031 = UNIQUE | NON_NULL, (empty)
                                            // 1377: &mut obuf: typeof(_1851 = &raw mut (*_1852)) = *mut {l3745} DefId(0:160 ~ lglmain[0c57]::OBuf)
                                            // 1377: &mut obuf:   l3745 = UNIQUE | NON_NULL, (empty)
                                            // 1377: &mut obuf: typeof(_1852 = &mut _30) = &mut {l3744} DefId(0:160 ~ lglmain[0c57]::OBuf)
                                            // 1377: &mut obuf:   l3744 = UNIQUE | NON_NULL, (empty)
                                            i += 1;
                                            i;
                                        }
                                        print2obuf(&mut obuf, 0 as libc::c_int, simponly, stdout);
                                        // 1381: &mut obuf: typeof(_1863) = *mut {l3039} DefId(0:160 ~ lglmain[0c57]::OBuf)
                                        // 1381: &mut obuf:   l3039 = UNIQUE | NON_NULL, (empty)
                                        // 1381: &mut obuf: typeof(_1864) = &mut {l3041} DefId(0:160 ~ lglmain[0c57]::OBuf)
                                        // 1381: &mut obuf:   l3041 = UNIQUE | NON_NULL, (empty)
                                        // 1381: stdout: typeof(_1867) = *mut {l3045} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                        // 1381: stdout:   l3045 = UNIQUE | NON_NULL, (empty)
                                        // 1381: stdout: typeof(_1868) = *mut {l3047} *mut {l3048} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                        // 1381: stdout:   l3047 = UNIQUE | NON_NULL, (empty)
                                        // 1381: stdout:   l3048 = UNIQUE | NON_NULL, (empty)
                                        // 1381: &mut obuf: typeof(_1864 = &mut _30) = &mut {l3746} DefId(0:160 ~ lglmain[0c57]::OBuf)
                                        // 1381: &mut obuf:   l3746 = UNIQUE | NON_NULL, (empty)
                                        // 1381: &mut obuf: typeof(_1863 = &raw mut (*_1864)) = *mut {l3747} DefId(0:160 ~ lglmain[0c57]::OBuf)
                                        // 1381: &mut obuf:   l3747 = UNIQUE | NON_NULL, (empty)
                                        if obuf.pos > 0 as libc::c_int {
                                            flushobuf(&mut obuf, simponly, stdout);
                                            // 1383: &mut obuf: typeof(_1874) = *mut {l3055} DefId(0:160 ~ lglmain[0c57]::OBuf)
                                            // 1383: &mut obuf:   l3055 = UNIQUE | NON_NULL, (empty)
                                            // 1383: &mut obuf: typeof(_1875) = &mut {l3057} DefId(0:160 ~ lglmain[0c57]::OBuf)
                                            // 1383: &mut obuf:   l3057 = UNIQUE | NON_NULL, (empty)
                                            // 1383: stdout: typeof(_1877) = *mut {l3060} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                            // 1383: stdout:   l3060 = UNIQUE | NON_NULL, (empty)
                                            // 1383: stdout: typeof(_1878) = *mut {l3062} *mut {l3063} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                            // 1383: stdout:   l3062 = UNIQUE | NON_NULL, (empty)
                                            // 1383: stdout:   l3063 = UNIQUE | NON_NULL, (empty)
                                            // 1383: &mut obuf: typeof(_1874 = &raw mut (*_1875)) = *mut {l3749} DefId(0:160 ~ lglmain[0c57]::OBuf)
                                            // 1383: &mut obuf:   l3749 = UNIQUE | NON_NULL, (empty)
                                            // 1383: &mut obuf: typeof(_1875 = &mut _30) = &mut {l3748} DefId(0:160 ~ lglmain[0c57]::OBuf)
                                            // 1383: &mut obuf:   l3748 = UNIQUE | NON_NULL, (empty)
                                        }
                                        fflush(stdout);
                                        // 1385: stdout: typeof(_1880) = *mut {l3066} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                        // 1385: stdout:   l3066 = UNIQUE | NON_NULL, (empty)
                                        // 1385: stdout: typeof(_1881) = *mut {l3068} *mut {l3069} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                        // 1385: stdout:   l3068 = UNIQUE | NON_NULL, (empty)
                                        // 1385: stdout:   l3069 = UNIQUE | NON_NULL, (empty)
                                    }
                                }
                                if verbose >= 0 as libc::c_int {
                                // 1388: verbose: typeof(_1884) = *mut {l3073} i32
                                // 1388: verbose:   l3073 = UNIQUE | NON_NULL, (empty)
                                    fputs(b"c\n\0" as *const u8 as *const libc::c_char, stdout);
                                    // 1389: b"c\n\0" as *co ... _char: typeof(_1887) = *const {l3077} i8
                                    // 1389: b"c\n\0" as *co ... _char:   l3077 = UNIQUE | NON_NULL, (empty)
                                    // 1389: b"c\n\0" as *co ... st u8: typeof(_1888) = *const {l3079} u8
                                    // 1389: b"c\n\0" as *co ... st u8:   l3079 = UNIQUE | NON_NULL, (empty)
                                    // 1389: b"c\n\0": typeof(_1889) = *const {l3081} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                                    // 1389: b"c\n\0":   l3081 = UNIQUE | NON_NULL, (empty)
                                    // 1389: b"c\n\0": typeof(_1890) = & {l3083} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                                    // 1389: b"c\n\0":   l3083 = UNIQUE | NON_NULL, FIXED
                                    // 1389: stdout: typeof(_1891) = *mut {l3085} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                    // 1389: stdout:   l3085 = UNIQUE | NON_NULL, (empty)
                                    // 1389: stdout: typeof(_1892) = *mut {l3087} *mut {l3088} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
                                    // 1389: stdout:   l3087 = UNIQUE | NON_NULL, (empty)
                                    // 1389: stdout:   l3088 = UNIQUE | NON_NULL, (empty)
                                    // 1389: b"c\n\0" as *co ... st u8: typeof(_1888 = move _1889 as *const u8 (Pointer(ArrayToPointer))) = *const {l3752} u8
                                    // 1389: b"c\n\0" as *co ... st u8:   l3752 = UNIQUE | NON_NULL, (empty)
                                    // 1389: b"c\n\0": typeof(_1890 = const b"c\n\x00") = & {l3750} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                                    // 1389: b"c\n\0":   l3750 = UNIQUE | NON_NULL, (empty)
                                    // 1389: b"c\n\0" as *co ... _char: typeof(_1887 = move _1888 as *const i8 (Misc)) = *const {l3753} i8
                                    // 1389: b"c\n\0" as *co ... _char:   l3753 = UNIQUE | NON_NULL, (empty)
                                    // 1389: b"c\n\0": typeof(_1889 = &raw const (*_1890)) = *const {l3751} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                                    // 1389: b"c\n\0":   l3751 = UNIQUE | NON_NULL, (empty)
                                    lglstats(lgl);
                                    // 1390: lgl: typeof(_1894) = *mut {l3091} LGL
                                    // 1390: lgl:   l3091 = UNIQUE | NON_NULL, (empty)
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    resetsighandlers();
    lgl4sigh = 0 as *mut LGL;
    // 1401: lgl4sigh: typeof(_1896) = *mut {l3094} *mut {l3095} LGL
    // 1401: lgl4sigh:   l3094 = UNIQUE | NON_NULL, (empty)
    // 1401: lgl4sigh:   l3095 = UNIQUE | NON_NULL, (empty)
    // 1401: lgl4sigh = 0 as ... t LGL: typeof((*_1896) = const 0_usize as *mut LGL (PointerFromExposedAddress)) = *mut {l3754} LGL
    // 1401: lgl4sigh = 0 as ... t LGL:   l3754 = UNIQUE | NON_NULL, (empty)
    lglrelease(lgl);
    // 1402: lgl: typeof(_1898) = *mut {l3098} LGL
    // 1402: lgl:   l3098 = UNIQUE | NON_NULL, (empty)
    free(targets as *mut libc::c_void);
    // 1403: targets as *mut ... _void: typeof(_1900) = *mut {l3101} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1403: targets as *mut ... _void:   l3101 = UNIQUE | NON_NULL, (empty)
    // 1403: targets: typeof(_1901) = *mut {l3103} i32
    // 1403: targets:   l3103 = UNIQUE | NON_NULL, (empty)
    // 1403: targets: typeof(_1902) = *mut {l3105} *mut {l3106} i32
    // 1403: targets:   l3105 = UNIQUE | NON_NULL, (empty)
    // 1403: targets:   l3106 = UNIQUE | NON_NULL, (empty)
    // 1403: targets as *mut ... _void: typeof(_1900 = move _1901 as *mut libc::c_void (Misc)) = *mut {l3755} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1403: targets as *mut ... _void:   l3755 = UNIQUE | NON_NULL, (empty)
    if verbose > 0 as libc::c_int {
    // 1404: verbose: typeof(_1906) = *mut {l3111} i32
    // 1404: verbose:   l3111 = UNIQUE | NON_NULL, (empty)
        printf(b"c exit %d\n\0" as *const u8 as *const libc::c_char, res);
        // 1405: b"c exit %d\n\0 ... _char: typeof(_1909) = *const {l3115} i8
        // 1405: b"c exit %d\n\0 ... _char:   l3115 = UNIQUE | NON_NULL, (empty)
        // 1405: b"c exit %d\n\0 ... st u8: typeof(_1910) = *const {l3117} u8
        // 1405: b"c exit %d\n\0 ... st u8:   l3117 = UNIQUE | NON_NULL, (empty)
        // 1405: b"c exit %d\n\0": typeof(_1911) = *const {l3119} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 1405: b"c exit %d\n\0":   l3119 = UNIQUE | NON_NULL, (empty)
        // 1405: b"c exit %d\n\0": typeof(_1912) = & {l3121} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 1405: b"c exit %d\n\0":   l3121 = UNIQUE | NON_NULL, FIXED
        // 1405: b"c exit %d\n\0 ... _char: typeof(_1909 = move _1910 as *const i8 (Misc)) = *const {l3759} i8
        // 1405: b"c exit %d\n\0 ... _char:   l3759 = UNIQUE | NON_NULL, (empty)
        // 1405: b"c exit %d\n\0": typeof(_1912 = const b"c exit %d\n\x00") = & {l3756} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 1405: b"c exit %d\n\0":   l3756 = UNIQUE | NON_NULL, (empty)
        // 1405: b"c exit %d\n\0 ... st u8: typeof(_1910 = move _1911 as *const u8 (Pointer(ArrayToPointer))) = *const {l3758} u8
        // 1405: b"c exit %d\n\0 ... st u8:   l3758 = UNIQUE | NON_NULL, (empty)
        // 1405: b"c exit %d\n\0": typeof(_1911 = &raw const (*_1912)) = *const {l3757} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 1405: b"c exit %d\n\0":   l3757 = UNIQUE | NON_NULL, (empty)
    }
    fflush(stdout);
    // 1407: stdout: typeof(_1915) = *mut {l3125} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
    // 1407: stdout:   l3125 = UNIQUE | NON_NULL, (empty)
    // 1407: stdout: typeof(_1916) = *mut {l3127} *mut {l3128} DefId(0:123 ~ lglmain[0c57]::_IO_FILE)
    // 1407: stdout:   l3127 = UNIQUE | NON_NULL, (empty)
    // 1407: stdout:   l3128 = UNIQUE | NON_NULL, (empty)
    return res;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    // 1411: mut args: typeof(_1) = DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l1} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
    // 1411: mut args:   l1 = UNIQUE | NON_NULL, (empty)
    for arg in ::std::env::args() {
    // 1412: ::std::env::args(): typeof(_9) = &mut {l10} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 1412: ::std::env::args():   l10 = UNIQUE | NON_NULL, (empty)
    // 1412: ::std::env::args(): typeof(_10) = &mut {l12} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 1412: ::std::env::args():   l12 = UNIQUE | NON_NULL, (empty)
    // 1412: ::std::env::args(): typeof(_10 = &mut _5) = &mut {l51} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 1412: ::std::env::args():   l51 = UNIQUE | NON_NULL, (empty)
    // 1412: ::std::env::args(): typeof(_9 = &mut (*_10)) = &mut {l52} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 1412: ::std::env::args():   l52 = UNIQUE | NON_NULL, (empty)
        args.push(
        // 1413: args.push( (::s ... (), ): typeof(_15) = &mut {l18} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l19} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
        // 1413: args.push( (::s ... (), ):   l18 = UNIQUE | NON_NULL, (empty)
        // 1413: args.push( (::s ... (), ):   l19 = UNIQUE | NON_NULL, (empty)
        // 1413: args.push( (::s ... (), ): typeof(_15 = &mut _1) = &mut {l53} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l54} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
        // 1413: args.push( (::s ... (), ):   l53 = UNIQUE | NON_NULL, (empty)
        // 1413: args.push( (::s ... (), ):   l54 = UNIQUE | NON_NULL, (empty)
            (::std::ffi::CString::new(arg))
            // 1414: (::std::ffi::CS ... raw(): typeof(_16) = *mut {l21} i8
            // 1414: (::std::ffi::CS ... raw():   l21 = UNIQUE | NON_NULL, (empty)
                .expect("Failed to convert argument into CString.")
                // 1415: "Failed to conv ... ing.": typeof(_20) = & {l26} str
                // 1415: "Failed to conv ... ing.":   l26 = UNIQUE | NON_NULL, (empty)
                // 1415: "Failed to conv ... ing.": typeof(_21) = & {l28} str
                // 1415: "Failed to conv ... ing.":   l28 = UNIQUE | NON_NULL, FIXED
                // 1415: "Failed to conv ... ing.": typeof(_20 = &(*_21)) = & {l56} str
                // 1415: "Failed to conv ... ing.":   l56 = UNIQUE | NON_NULL, (empty)
                // 1415: "Failed to conv ... ing.": typeof(_21 = const "Failed to convert argument into CString.") = & {l55} str
                // 1415: "Failed to conv ... ing.":   l55 = UNIQUE | NON_NULL, (empty)
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    // 1419: args.push(::cor ... ut()): typeof(_23) = &mut {l31} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l32} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
    // 1419: args.push(::cor ... ut()):   l31 = UNIQUE | NON_NULL, (empty)
    // 1419: args.push(::cor ... ut()):   l32 = UNIQUE | NON_NULL, (empty)
    // 1419: ::core::ptr::nu ... mut(): typeof(_24) = *mut {l34} i8
    // 1419: ::core::ptr::nu ... mut():   l34 = UNIQUE | NON_NULL, (empty)
    // 1419: args.push(::cor ... ut()): typeof(_23 = &mut _1) = &mut {l57} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l58} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
    // 1419: args.push(::cor ... ut()):   l57 = UNIQUE | NON_NULL, (empty)
    // 1419: args.push(::cor ... ut()):   l58 = UNIQUE | NON_NULL, (empty)
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            // 1422: args.len(): typeof(_30) = & {l41} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l42} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 1422: args.len():   l41 = UNIQUE | NON_NULL, (empty)
            // 1422: args.len():   l42 = UNIQUE | NON_NULL, (empty)
            // 1422: args.len(): typeof(_30 = &_1) = & {l59} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l60} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 1422: args.len():   l59 = UNIQUE | NON_NULL, (empty)
            // 1422: args.len():   l60 = UNIQUE | NON_NULL, (empty)
            args.as_mut_ptr() as *mut *mut libc::c_char,
            // 1423: args.as_mut_ptr ... _char: typeof(_32) = *mut {l45} *mut {l46} i8
            // 1423: args.as_mut_ptr ... _char:   l45 = UNIQUE | NON_NULL, (empty)
            // 1423: args.as_mut_ptr ... _char:   l46 = UNIQUE | NON_NULL, (empty)
            // 1423: args.as_mut_ptr(): typeof(_33) = &mut {l48} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l49} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 1423: args.as_mut_ptr():   l48 = UNIQUE | NON_NULL, (empty)
            // 1423: args.as_mut_ptr():   l49 = UNIQUE | NON_NULL, (empty)
            // 1423: args.as_mut_ptr(): typeof(_33 = &mut _1) = &mut {l61} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l62} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 1423: args.as_mut_ptr():   l61 = UNIQUE | NON_NULL, (empty)
            // 1423: args.as_mut_ptr():   l62 = UNIQUE | NON_NULL, (empty)
        ) as i32)
    }
}
unsafe extern "C" fn run_static_initializers() {
    nprimes = (::core::mem::size_of::<[libc::c_int; 5]>() as libc::c_ulong)
    // 1428: nprimes: typeof(_6) = *mut {l6} u32
    // 1428: nprimes:   l6 = READ | WRITE | UNIQUE | NON_NULL, (empty)
        .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_uint;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
