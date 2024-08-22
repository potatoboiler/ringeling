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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::core::ffi::VaList) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn lglinit() -> *mut LGL;
    fn lglrelease(_: *mut LGL);
    fn lglsetopt(_: *mut LGL, _: *const libc::c_char, _: libc::c_int);
    fn lglsetphase(_: *mut LGL, lit: libc::c_int);
    fn lglresetphase(_: *mut LGL, lit: libc::c_int);
    fn lglsetimportant(_: *mut LGL, lit: libc::c_int);
    fn lglsetphases(_: *mut LGL);
    fn lglmaxvar(_: *mut LGL) -> libc::c_int;
    fn lglincvar(_: *mut LGL) -> libc::c_int;
    fn lgladd(_: *mut LGL, lit: libc::c_int);
    fn lglassume(_: *mut LGL, lit: libc::c_int);
    fn lglfixate(_: *mut LGL);
    fn lglsat(_: *mut LGL) -> libc::c_int;
    fn lglsimp(_: *mut LGL, iterations: libc::c_int) -> libc::c_int;
    fn lglderef(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglfixed(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglfailed(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglinconsistent(_: *mut LGL) -> libc::c_int;
    fn lglchanged(_: *mut LGL) -> libc::c_int;
    fn lglreducecache(_: *mut LGL);
    fn lglflushcache(_: *mut LGL);
    fn lglrepr(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglfreeze(_: *mut LGL, lit: libc::c_int);
    fn lglfrozen(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglmelt(_: *mut LGL, lit: libc::c_int);
    fn lglusable(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglreusable(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglreuse(_: *mut LGL, lit: libc::c_int);
    fn lglookahead(_: *mut LGL) -> libc::c_int;
    fn lglstats(_: *mut LGL);
    fn lglchkclone(_: *mut LGL);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag<'h0,'h1> {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: &'h0 (libc::c_void),
    // 79: overflow_arg_area: typeof(overflow_arg_area) = *mut {g87} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 79: overflow_arg_area:   g87 = UNIQUE | NON_NULL, (empty)
    pub reg_save_area: &'h1 (libc::c_void),
    // 80: reg_save_area: typeof(reg_save_area) = *mut {g88} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 80: reg_save_area:   g88 = UNIQUE | NON_NULL, (empty)
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __gnuc_va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    // 90: _IO_read_ptr: typeof(_IO_read_ptr) = *mut {g89} i8
    // 90: _IO_read_ptr:   g89 = UNIQUE | NON_NULL, FIXED
    pub _IO_read_end: *mut libc::c_char,
    // 91: _IO_read_end: typeof(_IO_read_end) = *mut {g90} i8
    // 91: _IO_read_end:   g90 = UNIQUE | NON_NULL, FIXED
    pub _IO_read_base: *mut libc::c_char,
    // 92: _IO_read_base: typeof(_IO_read_base) = *mut {g91} i8
    // 92: _IO_read_base:   g91 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_base: *mut libc::c_char,
    // 93: _IO_write_base: typeof(_IO_write_base) = *mut {g92} i8
    // 93: _IO_write_base:   g92 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_ptr: *mut libc::c_char,
    // 94: _IO_write_ptr: typeof(_IO_write_ptr) = *mut {g93} i8
    // 94: _IO_write_ptr:   g93 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_end: *mut libc::c_char,
    // 95: _IO_write_end: typeof(_IO_write_end) = *mut {g94} i8
    // 95: _IO_write_end:   g94 = UNIQUE | NON_NULL, FIXED
    pub _IO_buf_base: *mut libc::c_char,
    // 96: _IO_buf_base: typeof(_IO_buf_base) = *mut {g95} i8
    // 96: _IO_buf_base:   g95 = UNIQUE | NON_NULL, FIXED
    pub _IO_buf_end: *mut libc::c_char,
    // 97: _IO_buf_end: typeof(_IO_buf_end) = *mut {g96} i8
    // 97: _IO_buf_end:   g96 = UNIQUE | NON_NULL, FIXED
    pub _IO_save_base: *mut libc::c_char,
    // 98: _IO_save_base: typeof(_IO_save_base) = *mut {g97} i8
    // 98: _IO_save_base:   g97 = UNIQUE | NON_NULL, FIXED
    pub _IO_backup_base: *mut libc::c_char,
    // 99: _IO_backup_base: typeof(_IO_backup_base) = *mut {g98} i8
    // 99: _IO_backup_base:   g98 = UNIQUE | NON_NULL, FIXED
    pub _IO_save_end: *mut libc::c_char,
    // 100: _IO_save_end: typeof(_IO_save_end) = *mut {g99} i8
    // 100: _IO_save_end:   g99 = UNIQUE | NON_NULL, FIXED
    pub _markers: *mut _IO_marker,
    // 101: _markers: typeof(_markers) = *mut {g100} _IO_marker
    // 101: _markers:   g100 = UNIQUE | NON_NULL, FIXED
    pub _chain: *mut _IO_FILE,
    // 102: _chain: typeof(_chain) = *mut {g101} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 102: _chain:   g101 = UNIQUE | NON_NULL, FIXED
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    // 109: _lock: typeof(_lock) = *mut {g102} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 109: _lock:   g102 = UNIQUE | NON_NULL, FIXED
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    // 111: _codecvt: typeof(_codecvt) = *mut {g103} _IO_codecvt
    // 111: _codecvt:   g103 = UNIQUE | NON_NULL, FIXED
    pub _wide_data: *mut _IO_wide_data,
    // 112: _wide_data: typeof(_wide_data) = *mut {g104} _IO_wide_data
    // 112: _wide_data:   g104 = UNIQUE | NON_NULL, FIXED
    pub _freeres_list: *mut _IO_FILE,
    // 113: _freeres_list: typeof(_freeres_list) = *mut {g105} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 113: _freeres_list:   g105 = UNIQUE | NON_NULL, FIXED
    pub _freeres_buf: *mut libc::c_void,
    // 114: _freeres_buf: typeof(_freeres_buf) = *mut {g106} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 114: _freeres_buf:   g106 = UNIQUE | NON_NULL, FIXED
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type va_list = __gnuc_va_list;
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
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
// 137: mut __nptr: typeof(_1) = *const {g0} i8
// 137: mut __nptr:   g0 = READ | UNIQUE | OFFSET_ADD | NON_NULL, FIXED
    return strtol(
        __nptr,
        // 139: __nptr: typeof(_4) = *const {l4} i8
        // 139: __nptr:   l4 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        // 140: 0 as *mut libc: ... _char: typeof(_5) = *mut {l6} *mut {g13} i8
        // 140: 0 as *mut libc: ... _char:   l6 = WRITE | UNIQUE, (empty)
        // 140: 0 as *mut libc: ... _char:   g13 = WRITE | OFFSET_ADD, CELL | FIXED
        // 140: 0 as *mut libc: ... _void: typeof(_6) = *mut {l8} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 140: 0 as *mut libc: ... _void:   l8 = WRITE | UNIQUE, (empty)
        // 140: 0 as *mut libc: ... _char: typeof(_5 = move _6 as *mut *mut i8 (Misc)) = *mut {l12} *mut {g13} i8
        // 140: 0 as *mut libc: ... _char:   l12 = WRITE | UNIQUE, (empty)
        // 140: 0 as *mut libc: ... _char:   g13 = WRITE | OFFSET_ADD, CELL | FIXED
        // 140: 0 as *mut libc: ... _void: typeof(_6 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l11} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 140: 0 as *mut libc: ... _void:   l11 = WRITE | UNIQUE, (empty)
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn vprintf(
    mut __fmt: *const libc::c_char,
    // 146: mut __fmt: typeof(_1) = *const {g1} i8
    // 146: mut __fmt:   g1 = UNIQUE | NON_NULL, FIXED
    mut __arg: ::core::ffi::VaList,
) -> libc::c_int {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
    // 149: stdout: typeof(_4) = *mut {l4} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 149: stdout:   l4 = UNIQUE | NON_NULL, (empty)
    // 149: stdout: typeof(_5) = *mut {l6} *mut {l7} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 149: stdout:   l6 = UNIQUE | NON_NULL, (empty)
    // 149: stdout:   l7 = UNIQUE | NON_NULL, (empty)
    // 149: __fmt: typeof(_6) = *const {l9} i8
    // 149: __fmt:   l9 = UNIQUE | NON_NULL, (empty)
    // 149: __arg.as_va_list(): typeof(_8) = &mut {l12} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 149: __arg.as_va_list():   l12 = UNIQUE | NON_NULL, (empty)
    // 149: __arg.as_va_list(): typeof(_9) = &mut {l14} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 149: __arg.as_va_list():   l14 = UNIQUE | NON_NULL, FIXED
    // 149: __arg.as_va_list(): typeof(_10) = &mut {l16} DefId(2:46558 ~ core[480a]::ffi::VaList)
    // 149: __arg.as_va_list():   l16 = UNIQUE | NON_NULL, (empty)
    // 149: __arg.as_va_list(): typeof(_10 = &mut _2) = &mut {l18} DefId(2:46558 ~ core[480a]::ffi::VaList)
    // 149: __arg.as_va_list():   l18 = UNIQUE | NON_NULL, (empty)
    // 149: __arg.as_va_list(): typeof(_8 = &mut (*_9)) = &mut {l19} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 149: __arg.as_va_list():   l19 = UNIQUE | NON_NULL, (empty)
}
static verbose: libc::c_int = 0;
static exitonabort: libc::c_int = 0;
static lineno: libc::c_int = 0;
static name: *const libc::c_char = 0 as *const libc::c_char;
unsafe extern "C" fn die(mut fmt: *const libc::c_char, mut args: ...) {
// 155: mut fmt: typeof(_1) = *const {g2} i8
// 155: mut fmt:   g2 = UNIQUE | NON_NULL, FIXED
    let mut ap: ::core::ffi::VaListImpl;
    fputs(
        b"*** lgluntrace: \0" as *const u8 as *const libc::c_char,
        // 158: b"*** lgluntrac ... _char: typeof(_6) = *const {l6} i8
        // 158: b"*** lgluntrac ... _char:   l6 = UNIQUE | NON_NULL, (empty)
        // 158: b"*** lgluntrac ... st u8: typeof(_7) = *const {l8} u8
        // 158: b"*** lgluntrac ... st u8:   l8 = UNIQUE | NON_NULL, (empty)
        // 158: b"*** lgluntrac ... : \0": typeof(_8) = *const {l10} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 158: b"*** lgluntrac ... : \0":   l10 = UNIQUE | NON_NULL, (empty)
        // 158: b"*** lgluntrac ... : \0": typeof(_9) = & {l12} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 158: b"*** lgluntrac ... : \0":   l12 = UNIQUE | NON_NULL, FIXED
        // 158: b"*** lgluntrac ... : \0": typeof(_9 = const b"*** lgluntrace: \x00") = & {l48} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 158: b"*** lgluntrac ... : \0":   l48 = UNIQUE | NON_NULL, (empty)
        // 158: b"*** lgluntrac ... st u8: typeof(_7 = move _8 as *const u8 (Pointer(ArrayToPointer))) = *const {l50} u8
        // 158: b"*** lgluntrac ... st u8:   l50 = UNIQUE | NON_NULL, (empty)
        // 158: b"*** lgluntrac ... : \0": typeof(_8 = &raw const (*_9)) = *const {l49} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 158: b"*** lgluntrac ... : \0":   l49 = UNIQUE | NON_NULL, (empty)
        // 158: b"*** lgluntrac ... _char: typeof(_6 = move _7 as *const i8 (Misc)) = *const {l51} i8
        // 158: b"*** lgluntrac ... _char:   l51 = UNIQUE | NON_NULL, (empty)
        stderr,
        // 159: stderr: typeof(_10) = *mut {l14} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
        // 159: stderr:   l14 = UNIQUE | NON_NULL, (empty)
        // 159: stderr: typeof(_11) = *mut {l16} *mut {l17} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
        // 159: stderr:   l16 = UNIQUE | NON_NULL, (empty)
        // 159: stderr:   l17 = UNIQUE | NON_NULL, (empty)
    );
    ap = args.clone();
    // 161: args.clone(): typeof(_13) = & {l20} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 161: args.clone():   l20 = UNIQUE | NON_NULL, (empty)
    // 161: args.clone(): typeof(_13 = &_2) = & {l52} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 161: args.clone():   l52 = UNIQUE | NON_NULL, (empty)
    vfprintf(stderr, fmt, ap.as_va_list());
    // 162: stderr: typeof(_15) = *mut {l23} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 162: stderr:   l23 = UNIQUE | NON_NULL, (empty)
    // 162: stderr: typeof(_16) = *mut {l25} *mut {l26} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 162: stderr:   l25 = UNIQUE | NON_NULL, (empty)
    // 162: stderr:   l26 = UNIQUE | NON_NULL, (empty)
    // 162: fmt: typeof(_17) = *const {l28} i8
    // 162: fmt:   l28 = UNIQUE | NON_NULL, (empty)
    // 162: ap.as_va_list(): typeof(_19) = &mut {l31} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 162: ap.as_va_list():   l31 = UNIQUE | NON_NULL, (empty)
    // 162: ap.as_va_list(): typeof(_19 = &mut _4) = &mut {l53} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 162: ap.as_va_list():   l53 = UNIQUE | NON_NULL, (empty)
    fputc('\n' as i32, stderr);
    // 163: stderr: typeof(_22) = *mut {l35} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 163: stderr:   l35 = UNIQUE | NON_NULL, (empty)
    // 163: stderr: typeof(_23) = *mut {l37} *mut {l38} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 163: stderr:   l37 = UNIQUE | NON_NULL, (empty)
    // 163: stderr:   l38 = UNIQUE | NON_NULL, (empty)
    fflush(stderr);
    // 164: stderr: typeof(_25) = *mut {l41} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 164: stderr:   l41 = UNIQUE | NON_NULL, (empty)
    // 164: stderr: typeof(_26) = *mut {l43} *mut {l44} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 164: stderr:   l43 = UNIQUE | NON_NULL, (empty)
    // 164: stderr:   l44 = UNIQUE | NON_NULL, (empty)
    exit(1 as libc::c_int);
}
unsafe extern "C" fn perr(mut fmt: *const libc::c_char, mut args: ...) {
// 167: mut fmt: typeof(_1) = *const {g3} i8
// 167: mut fmt:   g3 = UNIQUE | NON_NULL, FIXED
    let mut ap: ::core::ffi::VaListImpl;
    fprintf(
        stderr,
        // 170: stderr: typeof(_6) = *mut {l6} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
        // 170: stderr:   l6 = UNIQUE | NON_NULL, (empty)
        // 170: stderr: typeof(_7) = *mut {l8} *mut {l9} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
        // 170: stderr:   l8 = UNIQUE | NON_NULL, (empty)
        // 170: stderr:   l9 = UNIQUE | NON_NULL, (empty)
        b"*** lgluntrace: parse error in '%s' line %d: \0" as *const u8 as *const libc::c_char,
        // 171: b"*** lgluntrac ... _char: typeof(_8) = *const {l11} i8
        // 171: b"*** lgluntrac ... _char:   l11 = UNIQUE | NON_NULL, (empty)
        // 171: b"*** lgluntrac ... st u8: typeof(_9) = *const {l13} u8
        // 171: b"*** lgluntrac ... st u8:   l13 = UNIQUE | NON_NULL, (empty)
        // 171: b"*** lgluntrac ... : \0": typeof(_10) = *const {l15} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
        // 171: b"*** lgluntrac ... : \0":   l15 = UNIQUE | NON_NULL, (empty)
        // 171: b"*** lgluntrac ... : \0": typeof(_11) = & {l17} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
        // 171: b"*** lgluntrac ... : \0":   l17 = UNIQUE | NON_NULL, FIXED
        // 171: b"*** lgluntrac ... : \0": typeof(_10 = &raw const (*_11)) = *const {l57} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
        // 171: b"*** lgluntrac ... : \0":   l57 = UNIQUE | NON_NULL, (empty)
        // 171: b"*** lgluntrac ... st u8: typeof(_9 = move _10 as *const u8 (Pointer(ArrayToPointer))) = *const {l58} u8
        // 171: b"*** lgluntrac ... st u8:   l58 = UNIQUE | NON_NULL, (empty)
        // 171: b"*** lgluntrac ... : \0": typeof(_11 = const b"*** lgluntrace: parse error in \'%s\' line %d: \x00") = & {l56} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
        // 171: b"*** lgluntrac ... : \0":   l56 = UNIQUE | NON_NULL, (empty)
        // 171: b"*** lgluntrac ... _char: typeof(_8 = move _9 as *const i8 (Misc)) = *const {l59} i8
        // 171: b"*** lgluntrac ... _char:   l59 = UNIQUE | NON_NULL, (empty)
        name,
        // 172: name: typeof(_12) = *const {l19} i8
        // 172: name:   l19 = UNIQUE | NON_NULL, (empty)
        // 172: name: typeof(_13) = *mut {l21} *const {l22} i8
        // 172: name:   l21 = UNIQUE | NON_NULL, (empty)
        // 172: name:   l22 = UNIQUE | NON_NULL, (empty)
        lineno,
        // 173: lineno: typeof(_15) = *mut {l25} i32
        // 173: lineno:   l25 = UNIQUE | NON_NULL, (empty)
    );
    ap = args.clone();
    // 175: args.clone(): typeof(_17) = & {l28} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 175: args.clone():   l28 = UNIQUE | NON_NULL, (empty)
    // 175: args.clone(): typeof(_17 = &_2) = & {l60} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 175: args.clone():   l60 = UNIQUE | NON_NULL, (empty)
    vfprintf(stderr, fmt, ap.as_va_list());
    // 176: stderr: typeof(_19) = *mut {l31} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 176: stderr:   l31 = UNIQUE | NON_NULL, (empty)
    // 176: stderr: typeof(_20) = *mut {l33} *mut {l34} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 176: stderr:   l33 = UNIQUE | NON_NULL, (empty)
    // 176: stderr:   l34 = UNIQUE | NON_NULL, (empty)
    // 176: fmt: typeof(_21) = *const {l36} i8
    // 176: fmt:   l36 = UNIQUE | NON_NULL, (empty)
    // 176: ap.as_va_list(): typeof(_23) = &mut {l39} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 176: ap.as_va_list():   l39 = UNIQUE | NON_NULL, (empty)
    // 176: ap.as_va_list(): typeof(_23 = &mut _4) = &mut {l61} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 176: ap.as_va_list():   l61 = UNIQUE | NON_NULL, (empty)
    fputc('\n' as i32, stderr);
    // 177: stderr: typeof(_26) = *mut {l43} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 177: stderr:   l43 = UNIQUE | NON_NULL, (empty)
    // 177: stderr: typeof(_27) = *mut {l45} *mut {l46} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 177: stderr:   l45 = UNIQUE | NON_NULL, (empty)
    // 177: stderr:   l46 = UNIQUE | NON_NULL, (empty)
    fflush(stderr);
    // 178: stderr: typeof(_29) = *mut {l49} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 178: stderr:   l49 = UNIQUE | NON_NULL, (empty)
    // 178: stderr: typeof(_30) = *mut {l51} *mut {l52} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 178: stderr:   l51 = UNIQUE | NON_NULL, (empty)
    // 178: stderr:   l52 = UNIQUE | NON_NULL, (empty)
    exit(1 as libc::c_int);
}
unsafe extern "C" fn msg(mut fmt: *const libc::c_char, mut args: ...) {
// 181: mut fmt: typeof(_1) = *const {g4} i8
// 181: mut fmt:   g4 = UNIQUE | NON_NULL, FIXED
    let mut ap: ::core::ffi::VaListImpl;
    if verbose == 0 {
    // 183: verbose: typeof(_7) = *mut {l7} i32
    // 183: verbose:   l7 = UNIQUE | NON_NULL, (empty)
        return;
    }
    fputs(
        b"c [lgluntrace] \0" as *const u8 as *const libc::c_char,
        // 187: b"c [lgluntrace ... _char: typeof(_10) = *const {l11} i8
        // 187: b"c [lgluntrace ... _char:   l11 = UNIQUE | NON_NULL, (empty)
        // 187: b"c [lgluntrace ... st u8: typeof(_11) = *const {l13} u8
        // 187: b"c [lgluntrace ... st u8:   l13 = UNIQUE | NON_NULL, (empty)
        // 187: b"c [lgluntrace] \0": typeof(_12) = *const {l15} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
        // 187: b"c [lgluntrace] \0":   l15 = UNIQUE | NON_NULL, (empty)
        // 187: b"c [lgluntrace] \0": typeof(_13) = & {l17} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
        // 187: b"c [lgluntrace] \0":   l17 = UNIQUE | NON_NULL, FIXED
        // 187: b"c [lgluntrace] \0": typeof(_12 = &raw const (*_13)) = *const {l47} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
        // 187: b"c [lgluntrace] \0":   l47 = UNIQUE | NON_NULL, (empty)
        // 187: b"c [lgluntrace ... st u8: typeof(_11 = move _12 as *const u8 (Pointer(ArrayToPointer))) = *const {l48} u8
        // 187: b"c [lgluntrace ... st u8:   l48 = UNIQUE | NON_NULL, (empty)
        // 187: b"c [lgluntrace] \0": typeof(_13 = const b"c [lgluntrace] \x00") = & {l46} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
        // 187: b"c [lgluntrace] \0":   l46 = UNIQUE | NON_NULL, (empty)
        // 187: b"c [lgluntrace ... _char: typeof(_10 = move _11 as *const i8 (Misc)) = *const {l49} i8
        // 187: b"c [lgluntrace ... _char:   l49 = UNIQUE | NON_NULL, (empty)
        stdout,
        // 188: stdout: typeof(_14) = *mut {l19} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
        // 188: stdout:   l19 = UNIQUE | NON_NULL, (empty)
        // 188: stdout: typeof(_15) = *mut {l21} *mut {l22} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
        // 188: stdout:   l21 = UNIQUE | NON_NULL, (empty)
        // 188: stdout:   l22 = UNIQUE | NON_NULL, (empty)
    );
    ap = args.clone();
    // 190: args.clone(): typeof(_17) = & {l25} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 190: args.clone():   l25 = UNIQUE | NON_NULL, (empty)
    // 190: args.clone(): typeof(_17 = &_2) = & {l50} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 190: args.clone():   l50 = UNIQUE | NON_NULL, (empty)
    vprintf(fmt, ap.as_va_list());
    // 191: fmt: typeof(_19) = *const {l28} i8
    // 191: fmt:   l28 = UNIQUE | NON_NULL, (empty)
    // 191: ap.as_va_list(): typeof(_21) = &mut {l31} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 191: ap.as_va_list():   l31 = UNIQUE | NON_NULL, (empty)
    // 191: ap.as_va_list(): typeof(_21 = &mut _3) = &mut {l51} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 191: ap.as_va_list():   l51 = UNIQUE | NON_NULL, (empty)
    fputc('\n' as i32, stdout);
    // 192: stdout: typeof(_24) = *mut {l35} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 192: stdout:   l35 = UNIQUE | NON_NULL, (empty)
    // 192: stdout: typeof(_25) = *mut {l37} *mut {l38} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 192: stdout:   l37 = UNIQUE | NON_NULL, (empty)
    // 192: stdout:   l38 = UNIQUE | NON_NULL, (empty)
    fflush(stdout);
    // 193: stdout: typeof(_27) = *mut {l41} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 193: stdout:   l41 = UNIQUE | NON_NULL, (empty)
    // 193: stdout: typeof(_28) = *mut {l43} *mut {l44} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 193: stdout:   l43 = UNIQUE | NON_NULL, (empty)
    // 193: stdout:   l44 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn isnumstr(mut str: *const libc::c_char) -> libc::c_int {
// 195: mut str: typeof(_1) = *const {g5} i8
// 195: mut str:   g5 = UNIQUE | NON_NULL, FIXED
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    // 196: mut p: typeof(_3) = *const {l3} i8
    // 196: mut p:   l3 = UNIQUE | NON_NULL, (empty)
    // 196: 0 as *const lib ... _char: typeof(_3 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l74} i8
    // 196: 0 as *const lib ... _char:   l74 = UNIQUE | NON_NULL, (empty)
    let mut ch: libc::c_int = 0;
    p = str;
    // 198: str: typeof(_5) = *const {l6} i8
    // 198: str:   l6 = UNIQUE | NON_NULL, (empty)
    if *p as libc::c_int == '-' as i32 {
        p = p.offset(1);
        // 200: p.offset(1): typeof(_11) = *const {l13} i8
        // 200: p.offset(1):   l13 = UNIQUE | NON_NULL, (empty)
        // 200: p: typeof(_12) = *const {l15} i8
        // 200: p:   l15 = UNIQUE | NON_NULL, (empty)
        p;
        // 201: p: typeof(_13) = *const {l17} i8
        // 201: p:   l17 = UNIQUE | NON_NULL, (empty)
    }
    let fresh0 = p;
    // 203: fresh0: typeof(_14) = *const {l19} i8
    // 203: fresh0:   l19 = UNIQUE | NON_NULL, (empty)
    p = p.offset(1);
    // 204: p.offset(1): typeof(_15) = *const {l21} i8
    // 204: p.offset(1):   l21 = UNIQUE | NON_NULL, (empty)
    // 204: p: typeof(_16) = *const {l23} i8
    // 204: p:   l23 = UNIQUE | NON_NULL, (empty)
    if *(*__ctype_b_loc()).offset(*fresh0 as libc::c_int as isize) as libc::c_int
    // 205: (*__ctype_b_loc ... size): typeof(_22) = *const {l30} u16
    // 205: (*__ctype_b_loc ... size):   l30 = UNIQUE | NON_NULL, (empty)
    // 205: (*__ctype_b_loc()): typeof(_23) = *const {l32} u16
    // 205: (*__ctype_b_loc()):   l32 = UNIQUE | NON_NULL, (empty)
    // 205: __ctype_b_loc(): typeof(_24) = *mut {l34} *const {l35} u16
    // 205: __ctype_b_loc():   l34 = UNIQUE | NON_NULL, (empty)
    // 205: __ctype_b_loc():   l35 = UNIQUE | NON_NULL, (empty)
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        == 0
    {
        return 0 as libc::c_int;
    }
    loop {
        ch = *p as libc::c_int;
        if !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
        // 213: (*__ctype_b_loc ... size): typeof(_41) = *const {l53} u16
        // 213: (*__ctype_b_loc ... size):   l53 = UNIQUE | NON_NULL, (empty)
        // 213: (*__ctype_b_loc()): typeof(_42) = *const {l55} u16
        // 213: (*__ctype_b_loc()):   l55 = UNIQUE | NON_NULL, (empty)
        // 213: __ctype_b_loc(): typeof(_43) = *mut {l57} *const {l58} u16
        // 213: __ctype_b_loc():   l57 = UNIQUE | NON_NULL, (empty)
        // 213: __ctype_b_loc():   l58 = UNIQUE | NON_NULL, (empty)
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0)
        {
            break;
        }
        p = p.offset(1);
        // 219: p.offset(1): typeof(_50) = *const {l66} i8
        // 219: p.offset(1):   l66 = UNIQUE | NON_NULL, (empty)
        // 219: p: typeof(_51) = *const {l68} i8
        // 219: p:   l68 = UNIQUE | NON_NULL, (empty)
        p;
        // 220: p: typeof(_52) = *const {l70} i8
        // 220: p:   l70 = UNIQUE | NON_NULL, (empty)
    }
    return (ch == 0) as libc::c_int;
}
unsafe extern "C" fn intarg(mut op: *mut libc::c_char) -> libc::c_int {
// 224: mut op: typeof(_1) = *mut {g6} i8
// 224: mut op:   g6 = UNIQUE | NON_NULL, FIXED
    let mut tok: *const libc::c_char = 0 as *const libc::c_char;
    // 225: mut tok: typeof(_3) = *const {l3} i8
    // 225: mut tok:   l3 = UNIQUE | NON_NULL, (empty)
    // 225: 0 as *const lib ... _char: typeof(_3 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l57} i8
    // 225: 0 as *const lib ... _char:   l57 = UNIQUE | NON_NULL, (empty)
    tok = strtok(
    // 226: strtok( 0 as *m ... ar, ): typeof(_4) = *mut {l5} i8
    // 226: strtok( 0 as *m ... ar, ):   l5 = UNIQUE | NON_NULL, (empty)
    // 226: tok = strtok( 0 ... ar, ): typeof(_3 = move _4 as *const i8 (Pointer(MutToConstPointer))) = *const {l63} i8
    // 226: tok = strtok( 0 ... ar, ):   l63 = UNIQUE | NON_NULL, (empty)
        0 as *mut libc::c_char,
        // 227: 0 as *mut libc: ... _char: typeof(_5) = *mut {l7} i8
        // 227: 0 as *mut libc: ... _char:   l7 = UNIQUE | NON_NULL, (empty)
        // 227: 0 as *mut libc: ... _char: typeof(_5 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l58} i8
        // 227: 0 as *mut libc: ... _char:   l58 = UNIQUE | NON_NULL, (empty)
        b" \0" as *const u8 as *const libc::c_char,
        // 228: b" \0" as *cons ... _char: typeof(_6) = *const {l9} i8
        // 228: b" \0" as *cons ... _char:   l9 = UNIQUE | NON_NULL, (empty)
        // 228: b" \0" as *const u8: typeof(_7) = *const {l11} u8
        // 228: b" \0" as *const u8:   l11 = UNIQUE | NON_NULL, (empty)
        // 228: b" \0": typeof(_8) = *const {l13} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 228: b" \0":   l13 = UNIQUE | NON_NULL, (empty)
        // 228: b" \0": typeof(_9) = & {l15} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 228: b" \0":   l15 = UNIQUE | NON_NULL, FIXED
        // 228: b" \0" as *const u8: typeof(_7 = move _8 as *const u8 (Pointer(ArrayToPointer))) = *const {l61} u8
        // 228: b" \0" as *const u8:   l61 = UNIQUE | NON_NULL, (empty)
        // 228: b" \0": typeof(_9 = const b" \x00") = & {l59} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 228: b" \0":   l59 = UNIQUE | NON_NULL, (empty)
        // 228: b" \0" as *cons ... _char: typeof(_6 = move _7 as *const i8 (Misc)) = *const {l62} i8
        // 228: b" \0" as *cons ... _char:   l62 = UNIQUE | NON_NULL, (empty)
        // 228: b" \0": typeof(_8 = &raw const (*_9)) = *const {l60} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 228: b" \0":   l60 = UNIQUE | NON_NULL, (empty)
    );
    if tok.is_null()
    // 230: tok: typeof(_14) = *const {l21} i8
    // 230: tok:   l21 = UNIQUE | NON_NULL, (empty)
        || isnumstr(tok) == 0
        // 231: tok: typeof(_17) = *const {l25} i8
        // 231: tok:   l25 = UNIQUE | NON_NULL, (empty)
        || !(strtok(
        // 232: (strtok( 0 as * ... r, )): typeof(_20) = *mut {l29} i8
        // 232: (strtok( 0 as * ... r, )):   l29 = UNIQUE | NON_NULL, (empty)
            0 as *mut libc::c_char,
            // 233: 0 as *mut libc: ... _char: typeof(_21) = *mut {l31} i8
            // 233: 0 as *mut libc: ... _char:   l31 = UNIQUE | NON_NULL, (empty)
            // 233: 0 as *mut libc: ... _char: typeof(_21 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l64} i8
            // 233: 0 as *mut libc: ... _char:   l64 = UNIQUE | NON_NULL, (empty)
            b" \0" as *const u8 as *const libc::c_char,
            // 234: b" \0" as *cons ... _char: typeof(_22) = *const {l33} i8
            // 234: b" \0" as *cons ... _char:   l33 = UNIQUE | NON_NULL, (empty)
            // 234: b" \0" as *const u8: typeof(_23) = *const {l35} u8
            // 234: b" \0" as *const u8:   l35 = UNIQUE | NON_NULL, (empty)
            // 234: b" \0": typeof(_24) = *const {l37} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 234: b" \0":   l37 = UNIQUE | NON_NULL, (empty)
            // 234: b" \0": typeof(_25) = & {l39} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 234: b" \0":   l39 = UNIQUE | NON_NULL, FIXED
            // 234: b" \0" as *cons ... _char: typeof(_22 = move _23 as *const i8 (Misc)) = *const {l68} i8
            // 234: b" \0" as *cons ... _char:   l68 = UNIQUE | NON_NULL, (empty)
            // 234: b" \0": typeof(_24 = &raw const (*_25)) = *const {l66} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 234: b" \0":   l66 = UNIQUE | NON_NULL, (empty)
            // 234: b" \0": typeof(_25 = const b" \x00") = & {l65} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 234: b" \0":   l65 = UNIQUE | NON_NULL, (empty)
            // 234: b" \0" as *const u8: typeof(_23 = move _24 as *const u8 (Pointer(ArrayToPointer))) = *const {l67} u8
            // 234: b" \0" as *const u8:   l67 = UNIQUE | NON_NULL, (empty)
        ))
        .is_null()
    {
        perr(
            b"expected integer argument for '%s'\0" as *const u8 as *const libc::c_char,
            // 239: b"expected inte ... _char: typeof(_28) = *const {l43} i8
            // 239: b"expected inte ... _char:   l43 = UNIQUE | NON_NULL, (empty)
            // 239: b"expected inte ... st u8: typeof(_29) = *const {l45} u8
            // 239: b"expected inte ... st u8:   l45 = UNIQUE | NON_NULL, (empty)
            // 239: b"expected inte ... s'\0": typeof(_30) = *const {l47} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
            // 239: b"expected inte ... s'\0":   l47 = UNIQUE | NON_NULL, (empty)
            // 239: b"expected inte ... s'\0": typeof(_31) = & {l49} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
            // 239: b"expected inte ... s'\0":   l49 = UNIQUE | NON_NULL, FIXED
            // 239: b"expected inte ... st u8: typeof(_29 = move _30 as *const u8 (Pointer(ArrayToPointer))) = *const {l71} u8
            // 239: b"expected inte ... st u8:   l71 = UNIQUE | NON_NULL, (empty)
            // 239: b"expected inte ... s'\0": typeof(_31 = const b"expected integer argument for \'%s\'\x00") = & {l69} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
            // 239: b"expected inte ... s'\0":   l69 = UNIQUE | NON_NULL, (empty)
            // 239: b"expected inte ... s'\0": typeof(_30 = &raw const (*_31)) = *const {l70} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
            // 239: b"expected inte ... s'\0":   l70 = UNIQUE | NON_NULL, (empty)
            // 239: b"expected inte ... _char: typeof(_28 = move _29 as *const i8 (Misc)) = *const {l72} i8
            // 239: b"expected inte ... _char:   l72 = UNIQUE | NON_NULL, (empty)
            op,
            // 240: op: typeof(_32) = *mut {l51} i8
            // 240: op:   l51 = UNIQUE | NON_NULL, (empty)
        );
        exit(1 as libc::c_int);
    }
    return atoi(tok);
    // 244: tok: typeof(_35) = *const {l55} i8
    // 244: tok:   l55 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn noarg(mut str: *const libc::c_char, mut op: *mut libc::c_char) -> libc::c_int {
// 246: mut str: typeof(_1) = *const {g7} i8
// 246: mut str:   g7 = UNIQUE | NON_NULL, FIXED
// 246: mut op: typeof(_2) = *mut {g8} i8
// 246: mut op:   g8 = UNIQUE | NON_NULL, FIXED
    if strcmp(str, op) != 0 {
    // 247: str: typeof(_7) = *const {l7} i8
    // 247: str:   l7 = UNIQUE | NON_NULL, (empty)
    // 247: op: typeof(_8) = *const {l9} i8
    // 247: op:   l9 = UNIQUE | NON_NULL, (empty)
    // 247: op: typeof(_9) = *mut {l11} i8
    // 247: op:   l11 = UNIQUE | NON_NULL, (empty)
    // 247: op: typeof(_8 = move _9 as *const i8 (Pointer(MutToConstPointer))) = *const {l40} i8
    // 247: op:   l40 = UNIQUE | NON_NULL, (empty)
        return 0 as libc::c_int;
    }
    if !(strtok(
    // 250: (strtok( 0 as * ... r, )): typeof(_14) = *mut {l17} i8
    // 250: (strtok( 0 as * ... r, )):   l17 = UNIQUE | NON_NULL, (empty)
        0 as *mut libc::c_char,
        // 251: 0 as *mut libc: ... _char: typeof(_15) = *mut {l19} i8
        // 251: 0 as *mut libc: ... _char:   l19 = UNIQUE | NON_NULL, (empty)
        // 251: 0 as *mut libc: ... _char: typeof(_15 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l41} i8
        // 251: 0 as *mut libc: ... _char:   l41 = UNIQUE | NON_NULL, (empty)
        b" \0" as *const u8 as *const libc::c_char,
        // 252: b" \0" as *cons ... _char: typeof(_16) = *const {l21} i8
        // 252: b" \0" as *cons ... _char:   l21 = UNIQUE | NON_NULL, (empty)
        // 252: b" \0" as *const u8: typeof(_17) = *const {l23} u8
        // 252: b" \0" as *const u8:   l23 = UNIQUE | NON_NULL, (empty)
        // 252: b" \0": typeof(_18) = *const {l25} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 252: b" \0":   l25 = UNIQUE | NON_NULL, (empty)
        // 252: b" \0": typeof(_19) = & {l27} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 252: b" \0":   l27 = UNIQUE | NON_NULL, FIXED
        // 252: b" \0": typeof(_19 = const b" \x00") = & {l42} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 252: b" \0":   l42 = UNIQUE | NON_NULL, (empty)
        // 252: b" \0": typeof(_18 = &raw const (*_19)) = *const {l43} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 252: b" \0":   l43 = UNIQUE | NON_NULL, (empty)
        // 252: b" \0" as *cons ... _char: typeof(_16 = move _17 as *const i8 (Misc)) = *const {l45} i8
        // 252: b" \0" as *cons ... _char:   l45 = UNIQUE | NON_NULL, (empty)
        // 252: b" \0" as *const u8: typeof(_17 = move _18 as *const u8 (Pointer(ArrayToPointer))) = *const {l44} u8
        // 252: b" \0" as *const u8:   l44 = UNIQUE | NON_NULL, (empty)
    ))
    .is_null()
    {
        perr(
            b"argument after '%s'\0" as *const u8 as *const libc::c_char,
            // 257: b"argument afte ... _char: typeof(_21) = *const {l30} i8
            // 257: b"argument afte ... _char:   l30 = UNIQUE | NON_NULL, (empty)
            // 257: b"argument afte ... st u8: typeof(_22) = *const {l32} u8
            // 257: b"argument afte ... st u8:   l32 = UNIQUE | NON_NULL, (empty)
            // 257: b"argument afte ... s'\0": typeof(_23) = *const {l34} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 257: b"argument afte ... s'\0":   l34 = UNIQUE | NON_NULL, (empty)
            // 257: b"argument afte ... s'\0": typeof(_24) = & {l36} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 257: b"argument afte ... s'\0":   l36 = UNIQUE | NON_NULL, FIXED
            // 257: b"argument afte ... s'\0": typeof(_24 = const b"argument after \'%s\'\x00") = & {l46} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 257: b"argument afte ... s'\0":   l46 = UNIQUE | NON_NULL, (empty)
            // 257: b"argument afte ... st u8: typeof(_22 = move _23 as *const u8 (Pointer(ArrayToPointer))) = *const {l48} u8
            // 257: b"argument afte ... st u8:   l48 = UNIQUE | NON_NULL, (empty)
            // 257: b"argument afte ... s'\0": typeof(_23 = &raw const (*_24)) = *const {l47} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 257: b"argument afte ... s'\0":   l47 = UNIQUE | NON_NULL, (empty)
            // 257: b"argument afte ... _char: typeof(_21 = move _22 as *const i8 (Misc)) = *const {l49} i8
            // 257: b"argument afte ... _char:   l49 = UNIQUE | NON_NULL, (empty)
            op,
            // 258: op: typeof(_25) = *mut {l38} i8
            // 258: op:   l38 = UNIQUE | NON_NULL, (empty)
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn exitonsig(mut sig: libc::c_int) {
    msg(
        core::ptr::addr_of!(*(&*(b"exit(%d) on signal %d\0") as *const u8 as *const libc::c_char)),
        // 265: b"exit(%d) on s ... _char: typeof(_4) = *const {l4} i8
        // 265: b"exit(%d) on s ... _char:   l4 = UNIQUE | NON_NULL, (empty)
        // 265: b"exit(%d) on s ... st u8: typeof(_5) = *const {l6} u8
        // 265: b"exit(%d) on s ... st u8:   l6 = UNIQUE | NON_NULL, (empty)
        // 265: b"exit(%d) on s ... %d\0": typeof(_6) = *const {l8} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 265: b"exit(%d) on s ... %d\0":   l8 = UNIQUE | NON_NULL, (empty)
        // 265: b"exit(%d) on s ... %d\0": typeof(_7) = & {l10} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 265: b"exit(%d) on s ... %d\0":   l10 = UNIQUE | NON_NULL, FIXED
        // 265: b"exit(%d) on s ... %d\0": typeof(_6 = &raw const (*_7)) = *const {l17} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 265: b"exit(%d) on s ... %d\0":   l17 = UNIQUE | NON_NULL, (empty)
        // 265: b"exit(%d) on s ... %d\0": typeof(_7 = const b"exit(%d) on signal %d\x00") = & {l16} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 265: b"exit(%d) on s ... %d\0":   l16 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
        // 265: b"exit(%d) on s ... st u8: typeof(_5 = move _6 as *const u8 (Pointer(ArrayToPointer))) = *const {l18} u8
        // 265: b"exit(%d) on s ... st u8:   l18 = UNIQUE | NON_NULL, (empty)
        // 265: b"exit(%d) on s ... _char: typeof(_4 = move _5 as *const i8 (Misc)) = *const {l19} i8
        // 265: b"exit(%d) on s ... _char:   l19 = UNIQUE | NON_NULL, (empty)
        sig,
        sig,
    );
    exit(sig);
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
// 271: mut argv: typeof(_2) = *mut {g9} *mut {g10} i8
// 271: mut argv:   g9 = UNIQUE | NON_NULL, FIXED
// 271: mut argv:   g10 = UNIQUE | NON_NULL, FIXED
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut close: libc::c_int = 0 as libc::c_int;
    let mut res: libc::c_int = 0;
    let mut arg: libc::c_int = 0;
    let mut buffer: [libc::c_char; 80] = [0; 80];
    let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
    // 279: mut tok: typeof(_11) = *mut {l11} i8
    // 279: mut tok:   l11 = UNIQUE | NON_NULL, (empty)
    // 279: 0 as *mut libc: ... _char: typeof(_11 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l1346} i8
    // 279: 0 as *mut libc: ... _char:   l1346 = UNIQUE | NON_NULL, (empty)
    let mut opt: *mut libc::c_char = 0 as *mut libc::c_char;
    // 280: mut opt: typeof(_12) = *mut {l13} i8
    // 280: mut opt:   l13 = UNIQUE | NON_NULL, (empty)
    // 280: 0 as *mut libc: ... _char: typeof(_12 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l1347} i8
    // 280: 0 as *mut libc: ... _char:   l1347 = UNIQUE | NON_NULL, (empty)
    let mut file: *mut FILE = 0 as *mut FILE;
    // 281: mut file: typeof(_13) = *mut {l15} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 281: mut file:   l15 = UNIQUE | NON_NULL, (empty)
    // 281: 0 as *mut FILE: typeof(_13 = const 0_usize as *mut _IO_FILE (PointerFromExposedAddress)) = *mut {l1348} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
    // 281: 0 as *mut FILE:   l1348 = UNIQUE | NON_NULL, (empty)
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    // 282: mut cmd: typeof(_14) = *mut {l17} i8
    // 282: mut cmd:   l17 = UNIQUE | NON_NULL, (empty)
    // 282: 0 as *mut libc: ... _char: typeof(_14 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l1349} i8
    // 282: 0 as *mut libc: ... _char:   l1349 = UNIQUE | NON_NULL, (empty)
    let mut lgl: *mut LGL = 0 as *mut LGL;
    // 283: mut lgl: typeof(_15) = *mut {l19} LGL
    // 283: mut lgl:   l19 = UNIQUE | NON_NULL, (empty)
    // 283: 0 as *mut LGL: typeof(_15 = const 0_usize as *mut LGL (PointerFromExposedAddress)) = *mut {l1350} LGL
    // 283: 0 as *mut LGL:   l1350 = UNIQUE | NON_NULL, (empty)
    i = 1 as libc::c_int;
    while i < argc {
        if strcmp(
            *argv.offset(i as isize),
            // 287: *argv.offset(i  ... size): typeof(_25) = *const {l30} i8
            // 287: *argv.offset(i  ... size):   l30 = UNIQUE | NON_NULL, (empty)
            // 287: *argv.offset(i  ... size): typeof(_26) = *mut {l32} i8
            // 287: *argv.offset(i  ... size):   l32 = UNIQUE | NON_NULL, (empty)
            // 287: argv.offset(i a ... size): typeof(_27) = *mut {l34} *mut {l35} i8
            // 287: argv.offset(i a ... size):   l34 = UNIQUE | NON_NULL, (empty)
            // 287: argv.offset(i a ... size):   l35 = UNIQUE | NON_NULL, (empty)
            // 287: argv: typeof(_28) = *mut {l37} *mut {l38} i8
            // 287: argv:   l37 = UNIQUE | NON_NULL, (empty)
            // 287: argv:   l38 = UNIQUE | NON_NULL, (empty)
            // 287: *argv.offset(i  ... size): typeof(_25 = move _26 as *const i8 (Pointer(MutToConstPointer))) = *const {l1351} i8
            // 287: *argv.offset(i  ... size):   l1351 = UNIQUE | NON_NULL, (empty)
            b"-h\0" as *const u8 as *const libc::c_char,
            // 288: b"-h\0" as *con ... _char: typeof(_31) = *const {l42} i8
            // 288: b"-h\0" as *con ... _char:   l42 = UNIQUE | NON_NULL, (empty)
            // 288: b"-h\0" as *const u8: typeof(_32) = *const {l44} u8
            // 288: b"-h\0" as *const u8:   l44 = UNIQUE | NON_NULL, (empty)
            // 288: b"-h\0": typeof(_33) = *const {l46} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 288: b"-h\0":   l46 = UNIQUE | NON_NULL, (empty)
            // 288: b"-h\0": typeof(_34) = & {l48} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 288: b"-h\0":   l48 = UNIQUE | NON_NULL, FIXED
            // 288: b"-h\0": typeof(_34 = const b"-h\x00") = & {l1352} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 288: b"-h\0":   l1352 = UNIQUE | NON_NULL, (empty)
            // 288: b"-h\0": typeof(_33 = &raw const (*_34)) = *const {l1353} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 288: b"-h\0":   l1353 = UNIQUE | NON_NULL, (empty)
            // 288: b"-h\0" as *const u8: typeof(_32 = move _33 as *const u8 (Pointer(ArrayToPointer))) = *const {l1354} u8
            // 288: b"-h\0" as *const u8:   l1354 = UNIQUE | NON_NULL, (empty)
            // 288: b"-h\0" as *con ... _char: typeof(_31 = move _32 as *const i8 (Misc)) = *const {l1355} i8
            // 288: b"-h\0" as *con ... _char:   l1355 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            printf(
                b"usage: lgluntrace [-h][-v][-e][<trace>[.gz]]\n\0" as *const u8
                // 292: b"usage: lglunt ... _char: typeof(_37) = *const {l52} i8
                // 292: b"usage: lglunt ... _char:   l52 = UNIQUE | NON_NULL, (empty)
                // 292: b"usage: lglunt ... st u8: typeof(_38) = *const {l54} u8
                // 292: b"usage: lglunt ... st u8:   l54 = UNIQUE | NON_NULL, (empty)
                // 292: b"usage: lglunt ... \n\0": typeof(_39) = *const {l56} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                // 292: b"usage: lglunt ... \n\0":   l56 = UNIQUE | NON_NULL, (empty)
                // 292: b"usage: lglunt ... \n\0": typeof(_40) = & {l58} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                // 292: b"usage: lglunt ... \n\0":   l58 = UNIQUE | NON_NULL, FIXED
                // 292: b"usage: lglunt ... \n\0": typeof(_40 = const b"usage: lgluntrace [-h][-v][-e][<trace>[.gz]]\n\x00") = & {l1356} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                // 292: b"usage: lglunt ... \n\0":   l1356 = UNIQUE | NON_NULL, (empty)
                // 292: b"usage: lglunt ... _char: typeof(_37 = move _38 as *const i8 (Misc)) = *const {l1359} i8
                // 292: b"usage: lglunt ... _char:   l1359 = UNIQUE | NON_NULL, (empty)
                // 292: b"usage: lglunt ... \n\0": typeof(_39 = &raw const (*_40)) = *const {l1357} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                // 292: b"usage: lglunt ... \n\0":   l1357 = UNIQUE | NON_NULL, (empty)
                // 292: b"usage: lglunt ... st u8: typeof(_38 = move _39 as *const u8 (Pointer(ArrayToPointer))) = *const {l1358} u8
                // 292: b"usage: lglunt ... st u8:   l1358 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        } else if strcmp(
            *argv.offset(i as isize),
            // 297: *argv.offset(i  ... size): typeof(_45) = *const {l64} i8
            // 297: *argv.offset(i  ... size):   l64 = UNIQUE | NON_NULL, (empty)
            // 297: *argv.offset(i  ... size): typeof(_46) = *mut {l66} i8
            // 297: *argv.offset(i  ... size):   l66 = UNIQUE | NON_NULL, (empty)
            // 297: argv.offset(i a ... size): typeof(_47) = *mut {l68} *mut {l69} i8
            // 297: argv.offset(i a ... size):   l68 = UNIQUE | NON_NULL, (empty)
            // 297: argv.offset(i a ... size):   l69 = UNIQUE | NON_NULL, (empty)
            // 297: argv: typeof(_48) = *mut {l71} *mut {l72} i8
            // 297: argv:   l71 = UNIQUE | NON_NULL, (empty)
            // 297: argv:   l72 = UNIQUE | NON_NULL, (empty)
            // 297: *argv.offset(i  ... size): typeof(_45 = move _46 as *const i8 (Pointer(MutToConstPointer))) = *const {l1360} i8
            // 297: *argv.offset(i  ... size):   l1360 = UNIQUE | NON_NULL, (empty)
            b"-v\0" as *const u8 as *const libc::c_char,
            // 298: b"-v\0" as *con ... _char: typeof(_51) = *const {l76} i8
            // 298: b"-v\0" as *con ... _char:   l76 = UNIQUE | NON_NULL, (empty)
            // 298: b"-v\0" as *const u8: typeof(_52) = *const {l78} u8
            // 298: b"-v\0" as *const u8:   l78 = UNIQUE | NON_NULL, (empty)
            // 298: b"-v\0": typeof(_53) = *const {l80} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 298: b"-v\0":   l80 = UNIQUE | NON_NULL, (empty)
            // 298: b"-v\0": typeof(_54) = & {l82} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 298: b"-v\0":   l82 = UNIQUE | NON_NULL, FIXED
            // 298: b"-v\0": typeof(_54 = const b"-v\x00") = & {l1361} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 298: b"-v\0":   l1361 = UNIQUE | NON_NULL, (empty)
            // 298: b"-v\0" as *con ... _char: typeof(_51 = move _52 as *const i8 (Misc)) = *const {l1364} i8
            // 298: b"-v\0" as *con ... _char:   l1364 = UNIQUE | NON_NULL, (empty)
            // 298: b"-v\0": typeof(_53 = &raw const (*_54)) = *const {l1362} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 298: b"-v\0":   l1362 = UNIQUE | NON_NULL, (empty)
            // 298: b"-v\0" as *const u8: typeof(_52 = move _53 as *const u8 (Pointer(ArrayToPointer))) = *const {l1363} u8
            // 298: b"-v\0" as *const u8:   l1363 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            verbose = 1 as libc::c_int;
            // 301: verbose: typeof(_56) = *mut {l85} i32
            // 301: verbose:   l85 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 303: *argv.offset(i  ... size): typeof(_59) = *const {l89} i8
            // 303: *argv.offset(i  ... size):   l89 = UNIQUE | NON_NULL, (empty)
            // 303: *argv.offset(i  ... size): typeof(_60) = *mut {l91} i8
            // 303: *argv.offset(i  ... size):   l91 = UNIQUE | NON_NULL, (empty)
            // 303: argv.offset(i a ... size): typeof(_61) = *mut {l93} *mut {l94} i8
            // 303: argv.offset(i a ... size):   l93 = UNIQUE | NON_NULL, (empty)
            // 303: argv.offset(i a ... size):   l94 = UNIQUE | NON_NULL, (empty)
            // 303: argv: typeof(_62) = *mut {l96} *mut {l97} i8
            // 303: argv:   l96 = UNIQUE | NON_NULL, (empty)
            // 303: argv:   l97 = UNIQUE | NON_NULL, (empty)
            // 303: *argv.offset(i  ... size): typeof(_59 = move _60 as *const i8 (Pointer(MutToConstPointer))) = *const {l1365} i8
            // 303: *argv.offset(i  ... size):   l1365 = UNIQUE | NON_NULL, (empty)
            b"-e\0" as *const u8 as *const libc::c_char,
            // 304: b"-e\0" as *con ... _char: typeof(_65) = *const {l101} i8
            // 304: b"-e\0" as *con ... _char:   l101 = UNIQUE | NON_NULL, (empty)
            // 304: b"-e\0" as *const u8: typeof(_66) = *const {l103} u8
            // 304: b"-e\0" as *const u8:   l103 = UNIQUE | NON_NULL, (empty)
            // 304: b"-e\0": typeof(_67) = *const {l105} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 304: b"-e\0":   l105 = UNIQUE | NON_NULL, (empty)
            // 304: b"-e\0": typeof(_68) = & {l107} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 304: b"-e\0":   l107 = UNIQUE | NON_NULL, FIXED
            // 304: b"-e\0": typeof(_68 = const b"-e\x00") = & {l1366} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 304: b"-e\0":   l1366 = UNIQUE | NON_NULL, (empty)
            // 304: b"-e\0": typeof(_67 = &raw const (*_68)) = *const {l1367} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 304: b"-e\0":   l1367 = UNIQUE | NON_NULL, (empty)
            // 304: b"-e\0" as *con ... _char: typeof(_65 = move _66 as *const i8 (Misc)) = *const {l1369} i8
            // 304: b"-e\0" as *con ... _char:   l1369 = UNIQUE | NON_NULL, (empty)
            // 304: b"-e\0" as *const u8: typeof(_66 = move _67 as *const u8 (Pointer(ArrayToPointer))) = *const {l1368} u8
            // 304: b"-e\0" as *const u8:   l1368 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            exitonabort = 1 as libc::c_int;
            // 307: exitonabort: typeof(_70) = *mut {l110} i32
            // 307: exitonabort:   l110 = UNIQUE | NON_NULL, (empty)
        } else if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
        // 308: (*argv.offset(i ... size): typeof(_74) = *mut {l115} i8
        // 308: (*argv.offset(i ... size):   l115 = UNIQUE | NON_NULL, (empty)
        // 308: (*argv.offset(i ... ize)): typeof(_75) = *mut {l117} i8
        // 308: (*argv.offset(i ... ize)):   l117 = UNIQUE | NON_NULL, (empty)
        // 308: argv.offset(i a ... size): typeof(_76) = *mut {l119} *mut {l120} i8
        // 308: argv.offset(i a ... size):   l119 = UNIQUE | NON_NULL, (empty)
        // 308: argv.offset(i a ... size):   l120 = UNIQUE | NON_NULL, (empty)
        // 308: argv: typeof(_77) = *mut {l122} *mut {l123} i8
        // 308: argv:   l122 = UNIQUE | NON_NULL, (empty)
        // 308: argv:   l123 = UNIQUE | NON_NULL, (empty)
            == '-' as i32
        {
            die(
                b"invalid command line option '%s' (try '-h')\0" as *const u8
                // 312: b"invalid comma ... _char: typeof(_84) = *const {l131} i8
                // 312: b"invalid comma ... _char:   l131 = UNIQUE | NON_NULL, (empty)
                // 312: b"invalid comma ... st u8: typeof(_85) = *const {l133} u8
                // 312: b"invalid comma ... st u8:   l133 = UNIQUE | NON_NULL, (empty)
                // 312: b"invalid comma ... ')\0": typeof(_86) = *const {l135} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 312: b"invalid comma ... ')\0":   l135 = UNIQUE | NON_NULL, (empty)
                // 312: b"invalid comma ... ')\0": typeof(_87) = & {l137} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 312: b"invalid comma ... ')\0":   l137 = UNIQUE | NON_NULL, FIXED
                // 312: b"invalid comma ... ')\0": typeof(_87 = const b"invalid command line option \'%s\' (try \'-h\')\x00") = & {l1370} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 312: b"invalid comma ... ')\0":   l1370 = UNIQUE | NON_NULL, (empty)
                // 312: b"invalid comma ... st u8: typeof(_85 = move _86 as *const u8 (Pointer(ArrayToPointer))) = *const {l1372} u8
                // 312: b"invalid comma ... st u8:   l1372 = UNIQUE | NON_NULL, (empty)
                // 312: b"invalid comma ... _char: typeof(_84 = move _85 as *const i8 (Misc)) = *const {l1373} i8
                // 312: b"invalid comma ... _char:   l1373 = UNIQUE | NON_NULL, (empty)
                // 312: b"invalid comma ... ')\0": typeof(_86 = &raw const (*_87)) = *const {l1371} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 312: b"invalid comma ... ')\0":   l1371 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
                *argv.offset(i as isize),
                // 314: *argv.offset(i  ... size): typeof(_88) = *mut {l139} i8
                // 314: *argv.offset(i  ... size):   l139 = UNIQUE | NON_NULL, (empty)
                // 314: argv.offset(i a ... size): typeof(_89) = *mut {l141} *mut {l142} i8
                // 314: argv.offset(i a ... size):   l141 = UNIQUE | NON_NULL, (empty)
                // 314: argv.offset(i a ... size):   l142 = UNIQUE | NON_NULL, (empty)
                // 314: argv: typeof(_90) = *mut {l144} *mut {l145} i8
                // 314: argv:   l144 = UNIQUE | NON_NULL, (empty)
                // 314: argv:   l145 = UNIQUE | NON_NULL, (empty)
            );
        } else if !name.is_null() {
        // 316: name: typeof(_95) = *const {l151} i8
        // 316: name:   l151 = UNIQUE | NON_NULL, (empty)
        // 316: name: typeof(_96) = *mut {l153} *const {l154} i8
        // 316: name:   l153 = UNIQUE | NON_NULL, (empty)
        // 316: name:   l154 = UNIQUE | NON_NULL, (empty)
            die(
                b"two traces '%s' and '%s' specified (try '-h')\0" as *const u8
                // 318: b"two traces '% ... _char: typeof(_98) = *const {l157} i8
                // 318: b"two traces '% ... _char:   l157 = UNIQUE | NON_NULL, (empty)
                // 318: b"two traces '% ... st u8: typeof(_99) = *const {l159} u8
                // 318: b"two traces '% ... st u8:   l159 = UNIQUE | NON_NULL, (empty)
                // 318: b"two traces '% ... ')\0": typeof(_100) = *const {l161} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                // 318: b"two traces '% ... ')\0":   l161 = UNIQUE | NON_NULL, (empty)
                // 318: b"two traces '% ... ')\0": typeof(_101) = & {l163} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                // 318: b"two traces '% ... ')\0":   l163 = UNIQUE | NON_NULL, FIXED
                // 318: b"two traces '% ... ')\0": typeof(_100 = &raw const (*_101)) = *const {l1375} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                // 318: b"two traces '% ... ')\0":   l1375 = UNIQUE | NON_NULL, (empty)
                // 318: b"two traces '% ... _char: typeof(_98 = move _99 as *const i8 (Misc)) = *const {l1377} i8
                // 318: b"two traces '% ... _char:   l1377 = UNIQUE | NON_NULL, (empty)
                // 318: b"two traces '% ... st u8: typeof(_99 = move _100 as *const u8 (Pointer(ArrayToPointer))) = *const {l1376} u8
                // 318: b"two traces '% ... st u8:   l1376 = UNIQUE | NON_NULL, (empty)
                // 318: b"two traces '% ... ')\0": typeof(_101 = const b"two traces \'%s\' and \'%s\' specified (try \'-h\')\x00") = & {l1374} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                // 318: b"two traces '% ... ')\0":   l1374 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
                name,
                // 320: name: typeof(_102) = *const {l165} i8
                // 320: name:   l165 = UNIQUE | NON_NULL, (empty)
                // 320: name: typeof(_103) = *mut {l167} *const {l168} i8
                // 320: name:   l167 = UNIQUE | NON_NULL, (empty)
                // 320: name:   l168 = UNIQUE | NON_NULL, (empty)
                *argv.offset(i as isize),
                // 321: *argv.offset(i  ... size): typeof(_104) = *mut {l170} i8
                // 321: *argv.offset(i  ... size):   l170 = UNIQUE | NON_NULL, (empty)
                // 321: argv.offset(i a ... size): typeof(_105) = *mut {l172} *mut {l173} i8
                // 321: argv.offset(i a ... size):   l172 = UNIQUE | NON_NULL, (empty)
                // 321: argv.offset(i a ... size):   l173 = UNIQUE | NON_NULL, (empty)
                // 321: argv: typeof(_106) = *mut {l175} *mut {l176} i8
                // 321: argv:   l175 = UNIQUE | NON_NULL, (empty)
                // 321: argv:   l176 = UNIQUE | NON_NULL, (empty)
            );
        } else {
            name = *argv.offset(i as isize);
            // 324: *argv.offset(i  ... size): typeof(_109) = *mut {l180} i8
            // 324: *argv.offset(i  ... size):   l180 = UNIQUE | NON_NULL, (empty)
            // 324: argv.offset(i a ... size): typeof(_110) = *mut {l182} *mut {l183} i8
            // 324: argv.offset(i a ... size):   l182 = UNIQUE | NON_NULL, (empty)
            // 324: argv.offset(i a ... size):   l183 = UNIQUE | NON_NULL, (empty)
            // 324: argv: typeof(_111) = *mut {l185} *mut {l186} i8
            // 324: argv:   l185 = UNIQUE | NON_NULL, (empty)
            // 324: argv:   l186 = UNIQUE | NON_NULL, (empty)
            // 324: name: typeof(_114) = *mut {l190} *const {l191} i8
            // 324: name:   l190 = UNIQUE | NON_NULL, (empty)
            // 324: name:   l191 = UNIQUE | NON_NULL, (empty)
            // 324: name = *argv.of ... size): typeof((*_114) = move _109 as *const i8 (Pointer(MutToConstPointer))) = *const {l1378} i8
            // 324: name = *argv.of ... size):   l1378 = UNIQUE | NON_NULL, (empty)
        }
        i += 1;
        i;
    }
    if !name.is_null() {
    // 329: name: typeof(_123) = *const {l201} i8
    // 329: name:   l201 = UNIQUE | NON_NULL, (empty)
    // 329: name: typeof(_124) = *mut {l203} *const {l204} i8
    // 329: name:   l203 = UNIQUE | NON_NULL, (empty)
    // 329: name:   l204 = UNIQUE | NON_NULL, (empty)
        len = strlen(name) as libc::c_int;
        // 330: name: typeof(_126) = *const {l207} i8
        // 330: name:   l207 = UNIQUE | NON_NULL, (empty)
        // 330: name: typeof(_127) = *mut {l209} *const {l210} i8
        // 330: name:   l209 = UNIQUE | NON_NULL, (empty)
        // 330: name:   l210 = UNIQUE | NON_NULL, (empty)
        if len >= 3 as libc::c_int
            && strcmp(
                name.offset(len as isize)
                // 333: name.offset(len ... ize)): typeof(_135) = *const {l219} i8
                // 333: name.offset(len ... ize)):   l219 = UNIQUE | NON_NULL, (empty)
                // 333: name.offset(len ... size): typeof(_136) = *const {l221} i8
                // 333: name.offset(len ... size):   l221 = UNIQUE | NON_NULL, (empty)
                // 333: name: typeof(_137) = *const {l223} i8
                // 333: name:   l223 = UNIQUE | NON_NULL, (empty)
                // 333: name: typeof(_138) = *mut {l225} *const {l226} i8
                // 333: name:   l225 = UNIQUE | NON_NULL, (empty)
                // 333: name:   l226 = UNIQUE | NON_NULL, (empty)
                    .offset(-(3 as libc::c_int as isize)),
                b".gz\0" as *const u8 as *const libc::c_char,
                // 335: b".gz\0" as *co ... _char: typeof(_145) = *const {l234} i8
                // 335: b".gz\0" as *co ... _char:   l234 = UNIQUE | NON_NULL, (empty)
                // 335: b".gz\0" as *co ... st u8: typeof(_146) = *const {l236} u8
                // 335: b".gz\0" as *co ... st u8:   l236 = UNIQUE | NON_NULL, (empty)
                // 335: b".gz\0": typeof(_147) = *const {l238} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 335: b".gz\0":   l238 = UNIQUE | NON_NULL, (empty)
                // 335: b".gz\0": typeof(_148) = & {l240} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 335: b".gz\0":   l240 = UNIQUE | NON_NULL, FIXED
                // 335: b".gz\0" as *co ... _char: typeof(_145 = move _146 as *const i8 (Misc)) = *const {l1382} i8
                // 335: b".gz\0" as *co ... _char:   l1382 = UNIQUE | NON_NULL, (empty)
                // 335: b".gz\0": typeof(_147 = &raw const (*_148)) = *const {l1380} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 335: b".gz\0":   l1380 = UNIQUE | NON_NULL, (empty)
                // 335: b".gz\0": typeof(_148 = const b".gz\x00") = & {l1379} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 335: b".gz\0":   l1379 = UNIQUE | NON_NULL, (empty)
                // 335: b".gz\0" as *co ... st u8: typeof(_146 = move _147 as *const u8 (Pointer(ArrayToPointer))) = *const {l1381} u8
                // 335: b".gz\0" as *co ... st u8:   l1381 = UNIQUE | NON_NULL, (empty)
            ) == 0
        {
            cmd = malloc((len + 20 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
            // 338: malloc((len + 2 ... long): typeof(_149) = *mut {l242} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 338: malloc((len + 2 ... long):   l242 = UNIQUE | NON_NULL, (empty)
            // 338: cmd = malloc((l ... _char: typeof(_14 = move _149 as *mut i8 (Misc)) = *mut {l1383} i8
            // 338: cmd = malloc((l ... _char:   l1383 = UNIQUE | NON_NULL, (empty)
            sprintf(
                cmd,
                // 340: cmd: typeof(_156) = *mut {l250} i8
                // 340: cmd:   l250 = UNIQUE | NON_NULL, (empty)
                b"gunzip -c %s\0" as *const u8 as *const libc::c_char,
                // 341: b"gunzip -c %s\ ... _char: typeof(_157) = *const {l252} i8
                // 341: b"gunzip -c %s\ ... _char:   l252 = UNIQUE | NON_NULL, (empty)
                // 341: b"gunzip -c %s\ ... st u8: typeof(_158) = *const {l254} u8
                // 341: b"gunzip -c %s\ ... st u8:   l254 = UNIQUE | NON_NULL, (empty)
                // 341: b"gunzip -c %s\0": typeof(_159) = *const {l256} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 341: b"gunzip -c %s\0":   l256 = UNIQUE | NON_NULL, (empty)
                // 341: b"gunzip -c %s\0": typeof(_160) = & {l258} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 341: b"gunzip -c %s\0":   l258 = UNIQUE | NON_NULL, FIXED
                // 341: b"gunzip -c %s\ ... _char: typeof(_157 = move _158 as *const i8 (Misc)) = *const {l1387} i8
                // 341: b"gunzip -c %s\ ... _char:   l1387 = UNIQUE | NON_NULL, (empty)
                // 341: b"gunzip -c %s\ ... st u8: typeof(_158 = move _159 as *const u8 (Pointer(ArrayToPointer))) = *const {l1386} u8
                // 341: b"gunzip -c %s\ ... st u8:   l1386 = UNIQUE | NON_NULL, (empty)
                // 341: b"gunzip -c %s\0": typeof(_160 = const b"gunzip -c %s\x00") = & {l1384} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 341: b"gunzip -c %s\0":   l1384 = UNIQUE | NON_NULL, (empty)
                // 341: b"gunzip -c %s\0": typeof(_159 = &raw const (*_160)) = *const {l1385} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 341: b"gunzip -c %s\0":   l1385 = UNIQUE | NON_NULL, (empty)
                name,
                // 342: name: typeof(_161) = *const {l260} i8
                // 342: name:   l260 = UNIQUE | NON_NULL, (empty)
                // 342: name: typeof(_162) = *mut {l262} *const {l263} i8
                // 342: name:   l262 = UNIQUE | NON_NULL, (empty)
                // 342: name:   l263 = UNIQUE | NON_NULL, (empty)
            );
            file = popen(cmd, b"r\0" as *const u8 as *const libc::c_char);
            // 344: popen(cmd, b"r\ ... char): typeof(_163) = *mut {l265} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
            // 344: popen(cmd, b"r\ ... char):   l265 = UNIQUE | NON_NULL, (empty)
            // 344: cmd: typeof(_164) = *const {l267} i8
            // 344: cmd:   l267 = UNIQUE | NON_NULL, (empty)
            // 344: cmd: typeof(_165) = *mut {l269} i8
            // 344: cmd:   l269 = UNIQUE | NON_NULL, (empty)
            // 344: b"r\0" as *cons ... _char: typeof(_166) = *const {l271} i8
            // 344: b"r\0" as *cons ... _char:   l271 = UNIQUE | NON_NULL, (empty)
            // 344: b"r\0" as *const u8: typeof(_167) = *const {l273} u8
            // 344: b"r\0" as *const u8:   l273 = UNIQUE | NON_NULL, (empty)
            // 344: b"r\0": typeof(_168) = *const {l275} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 344: b"r\0":   l275 = UNIQUE | NON_NULL, (empty)
            // 344: b"r\0": typeof(_169) = & {l277} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 344: b"r\0":   l277 = UNIQUE | NON_NULL, FIXED
            // 344: b"r\0" as *cons ... _char: typeof(_166 = move _167 as *const i8 (Misc)) = *const {l1392} i8
            // 344: b"r\0" as *cons ... _char:   l1392 = UNIQUE | NON_NULL, (empty)
            // 344: b"r\0": typeof(_169 = const b"r\x00") = & {l1389} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 344: b"r\0":   l1389 = UNIQUE | NON_NULL, (empty)
            // 344: b"r\0" as *const u8: typeof(_167 = move _168 as *const u8 (Pointer(ArrayToPointer))) = *const {l1391} u8
            // 344: b"r\0" as *const u8:   l1391 = UNIQUE | NON_NULL, (empty)
            // 344: cmd: typeof(_164 = move _165 as *const i8 (Pointer(MutToConstPointer))) = *const {l1388} i8
            // 344: cmd:   l1388 = UNIQUE | NON_NULL, (empty)
            // 344: b"r\0": typeof(_168 = &raw const (*_169)) = *const {l1390} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 344: b"r\0":   l1390 = UNIQUE | NON_NULL, (empty)
            free(cmd as *mut libc::c_void);
            // 345: cmd as *mut lib ... _void: typeof(_171) = *mut {l280} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 345: cmd as *mut lib ... _void:   l280 = UNIQUE | NON_NULL, (empty)
            // 345: cmd: typeof(_172) = *mut {l282} i8
            // 345: cmd:   l282 = UNIQUE | NON_NULL, (empty)
            // 345: cmd as *mut lib ... _void: typeof(_171 = move _172 as *mut libc::c_void (Misc)) = *mut {l1393} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 345: cmd as *mut lib ... _void:   l1393 = UNIQUE | NON_NULL, (empty)
            if !file.is_null() {
            // 346: file: typeof(_175) = *mut {l286} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
            // 346: file:   l286 = UNIQUE | NON_NULL, (empty)
                close = 2 as libc::c_int;
            }
        } else {
            file = fopen(name, b"r\0" as *const u8 as *const libc::c_char);
            // 350: fopen(name, b"r ... char): typeof(_177) = *mut {l289} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
            // 350: fopen(name, b"r ... char):   l289 = UNIQUE | NON_NULL, (empty)
            // 350: name: typeof(_178) = *const {l291} i8
            // 350: name:   l291 = UNIQUE | NON_NULL, (empty)
            // 350: name: typeof(_179) = *mut {l293} *const {l294} i8
            // 350: name:   l293 = UNIQUE | NON_NULL, (empty)
            // 350: name:   l294 = UNIQUE | NON_NULL, (empty)
            // 350: b"r\0" as *cons ... _char: typeof(_180) = *const {l296} i8
            // 350: b"r\0" as *cons ... _char:   l296 = UNIQUE | NON_NULL, (empty)
            // 350: b"r\0" as *const u8: typeof(_181) = *const {l298} u8
            // 350: b"r\0" as *const u8:   l298 = UNIQUE | NON_NULL, (empty)
            // 350: b"r\0": typeof(_182) = *const {l300} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 350: b"r\0":   l300 = UNIQUE | NON_NULL, (empty)
            // 350: b"r\0": typeof(_183) = & {l302} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 350: b"r\0":   l302 = UNIQUE | NON_NULL, FIXED
            // 350: b"r\0" as *cons ... _char: typeof(_180 = move _181 as *const i8 (Misc)) = *const {l1397} i8
            // 350: b"r\0" as *cons ... _char:   l1397 = UNIQUE | NON_NULL, (empty)
            // 350: b"r\0": typeof(_182 = &raw const (*_183)) = *const {l1395} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 350: b"r\0":   l1395 = UNIQUE | NON_NULL, (empty)
            // 350: b"r\0": typeof(_183 = const b"r\x00") = & {l1394} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 350: b"r\0":   l1394 = UNIQUE | NON_NULL, (empty)
            // 350: b"r\0" as *const u8: typeof(_181 = move _182 as *const u8 (Pointer(ArrayToPointer))) = *const {l1396} u8
            // 350: b"r\0" as *const u8:   l1396 = UNIQUE | NON_NULL, (empty)
            if !file.is_null() {
            // 351: file: typeof(_186) = *mut {l306} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
            // 351: file:   l306 = UNIQUE | NON_NULL, (empty)
                close = 1 as libc::c_int;
            }
        }
        if file.is_null() {
        // 355: file: typeof(_189) = *mut {l310} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
        // 355: file:   l310 = UNIQUE | NON_NULL, (empty)
            die(
                b"can not read '%s'\0" as *const u8 as *const libc::c_char,
                // 357: b"can not read  ... _char: typeof(_191) = *const {l313} i8
                // 357: b"can not read  ... _char:   l313 = UNIQUE | NON_NULL, (empty)
                // 357: b"can not read  ... st u8: typeof(_192) = *const {l315} u8
                // 357: b"can not read  ... st u8:   l315 = UNIQUE | NON_NULL, (empty)
                // 357: b"can not read  ... s'\0": typeof(_193) = *const {l317} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                // 357: b"can not read  ... s'\0":   l317 = UNIQUE | NON_NULL, (empty)
                // 357: b"can not read  ... s'\0": typeof(_194) = & {l319} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                // 357: b"can not read  ... s'\0":   l319 = UNIQUE | NON_NULL, FIXED
                // 357: b"can not read  ... s'\0": typeof(_193 = &raw const (*_194)) = *const {l1399} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                // 357: b"can not read  ... s'\0":   l1399 = UNIQUE | NON_NULL, (empty)
                // 357: b"can not read  ... s'\0": typeof(_194 = const b"can not read \'%s\'\x00") = & {l1398} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                // 357: b"can not read  ... s'\0":   l1398 = UNIQUE | NON_NULL, (empty)
                // 357: b"can not read  ... st u8: typeof(_192 = move _193 as *const u8 (Pointer(ArrayToPointer))) = *const {l1400} u8
                // 357: b"can not read  ... st u8:   l1400 = UNIQUE | NON_NULL, (empty)
                // 357: b"can not read  ... _char: typeof(_191 = move _192 as *const i8 (Misc)) = *const {l1401} i8
                // 357: b"can not read  ... _char:   l1401 = UNIQUE | NON_NULL, (empty)
                name,
                // 358: name: typeof(_195) = *const {l321} i8
                // 358: name:   l321 = UNIQUE | NON_NULL, (empty)
                // 358: name: typeof(_196) = *mut {l323} *const {l324} i8
                // 358: name:   l323 = UNIQUE | NON_NULL, (empty)
                // 358: name:   l324 = UNIQUE | NON_NULL, (empty)
            );
        }
    } else {
        name = b"<stdin>\0" as *const u8 as *const libc::c_char;
        // 362: b"<stdin>\0" as ... st u8: typeof(_197) = *const {l326} u8
        // 362: b"<stdin>\0" as ... st u8:   l326 = UNIQUE | NON_NULL, (empty)
        // 362: b"<stdin>\0": typeof(_198) = *const {l328} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 362: b"<stdin>\0":   l328 = UNIQUE | NON_NULL, (empty)
        // 362: b"<stdin>\0": typeof(_199) = & {l330} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 362: b"<stdin>\0":   l330 = UNIQUE | NON_NULL, FIXED
        // 362: name: typeof(_200) = *mut {l332} *const {l333} i8
        // 362: name:   l332 = UNIQUE | NON_NULL, (empty)
        // 362: name:   l333 = UNIQUE | NON_NULL, (empty)
        // 362: b"<stdin>\0": typeof(_198 = &raw const (*_199)) = *const {l1403} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 362: b"<stdin>\0":   l1403 = UNIQUE | NON_NULL, (empty)
        // 362: b"<stdin>\0" as ... st u8: typeof(_197 = move _198 as *const u8 (Pointer(ArrayToPointer))) = *const {l1404} u8
        // 362: b"<stdin>\0" as ... st u8:   l1404 = UNIQUE | NON_NULL, (empty)
        // 362: b"<stdin>\0": typeof(_199 = const b"<stdin>\x00") = & {l1402} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 362: b"<stdin>\0":   l1402 = UNIQUE | NON_NULL, (empty)
        // 362: name = b"<stdin ... _char: typeof((*_200) = move _197 as *const i8 (Misc)) = *const {l1405} i8
        // 362: name = b"<stdin ... _char:   l1405 = UNIQUE | NON_NULL, (empty)
        file = stdin;
        // 363: stdin: typeof(_201) = *mut {l335} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
        // 363: stdin:   l335 = UNIQUE | NON_NULL, (empty)
        // 363: stdin: typeof(_202) = *mut {l337} *mut {l338} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
        // 363: stdin:   l337 = UNIQUE | NON_NULL, (empty)
        // 363: stdin:   l338 = UNIQUE | NON_NULL, (empty)
    }
    if exitonabort != 0 {
    // 365: exitonabort: typeof(_206) = *mut {l343} i32
    // 365: exitonabort:   l343 = UNIQUE | NON_NULL, (empty)
        msg(b"setting signal handlers since '-e' specified\0" as *const u8 as *const libc::c_char);
        // 366: b"setting signa ... _char: typeof(_208) = *const {l346} i8
        // 366: b"setting signa ... _char:   l346 = UNIQUE | NON_NULL, (empty)
        // 366: b"setting signa ... st u8: typeof(_209) = *const {l348} u8
        // 366: b"setting signa ... st u8:   l348 = UNIQUE | NON_NULL, (empty)
        // 366: b"setting signa ... ed\0": typeof(_210) = *const {l350} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
        // 366: b"setting signa ... ed\0":   l350 = UNIQUE | NON_NULL, (empty)
        // 366: b"setting signa ... ed\0": typeof(_211) = & {l352} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
        // 366: b"setting signa ... ed\0":   l352 = UNIQUE | NON_NULL, FIXED
        // 366: b"setting signa ... _char: typeof(_208 = move _209 as *const i8 (Misc)) = *const {l1409} i8
        // 366: b"setting signa ... _char:   l1409 = UNIQUE | NON_NULL, (empty)
        // 366: b"setting signa ... st u8: typeof(_209 = move _210 as *const u8 (Pointer(ArrayToPointer))) = *const {l1408} u8
        // 366: b"setting signa ... st u8:   l1408 = UNIQUE | NON_NULL, (empty)
        // 366: b"setting signa ... ed\0": typeof(_210 = &raw const (*_211)) = *const {l1407} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
        // 366: b"setting signa ... ed\0":   l1407 = UNIQUE | NON_NULL, (empty)
        // 366: b"setting signa ... ed\0": typeof(_211 = const b"setting signal handlers since \'-e\' specified\x00") = & {l1406} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
        // 366: b"setting signa ... ed\0":   l1406 = UNIQUE | NON_NULL, (empty)
        signal(
            2 as libc::c_int,
            Some(exitonsig as unsafe extern "C" fn(libc::c_int) -> ()),
        );
        signal(
            11 as libc::c_int,
            Some(exitonsig as unsafe extern "C" fn(libc::c_int) -> ()),
        );
        signal(
            6 as libc::c_int,
            Some(exitonsig as unsafe extern "C" fn(libc::c_int) -> ()),
        );
        signal(
            15 as libc::c_int,
            Some(exitonsig as unsafe extern "C" fn(libc::c_int) -> ()),
        );
    }
    msg(b"reading %s\0" as *const u8 as *const libc::c_char, name);
    // 384: b"reading %s\0" ... _char: typeof(_229) = *const {l371} i8
    // 384: b"reading %s\0" ... _char:   l371 = UNIQUE | NON_NULL, (empty)
    // 384: b"reading %s\0" ... st u8: typeof(_230) = *const {l373} u8
    // 384: b"reading %s\0" ... st u8:   l373 = UNIQUE | NON_NULL, (empty)
    // 384: b"reading %s\0": typeof(_231) = *const {l375} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
    // 384: b"reading %s\0":   l375 = UNIQUE | NON_NULL, (empty)
    // 384: b"reading %s\0": typeof(_232) = & {l377} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
    // 384: b"reading %s\0":   l377 = UNIQUE | NON_NULL, FIXED
    // 384: name: typeof(_233) = *const {l379} i8
    // 384: name:   l379 = UNIQUE | NON_NULL, (empty)
    // 384: name: typeof(_234) = *mut {l381} *const {l382} i8
    // 384: name:   l381 = UNIQUE | NON_NULL, (empty)
    // 384: name:   l382 = UNIQUE | NON_NULL, (empty)
    // 384: b"reading %s\0": typeof(_231 = &raw const (*_232)) = *const {l1411} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
    // 384: b"reading %s\0":   l1411 = UNIQUE | NON_NULL, (empty)
    // 384: b"reading %s\0" ... st u8: typeof(_230 = move _231 as *const u8 (Pointer(ArrayToPointer))) = *const {l1412} u8
    // 384: b"reading %s\0" ... st u8:   l1412 = UNIQUE | NON_NULL, (empty)
    // 384: b"reading %s\0": typeof(_232 = const b"reading %s\x00") = & {l1410} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
    // 384: b"reading %s\0":   l1410 = UNIQUE | NON_NULL, (empty)
    // 384: b"reading %s\0" ... _char: typeof(_229 = move _230 as *const i8 (Misc)) = *const {l1413} i8
    // 384: b"reading %s\0" ... _char:   l1413 = UNIQUE | NON_NULL, (empty)
    len = 0 as libc::c_int;
    buffer[len as usize] = 0 as libc::c_int as libc::c_char;
    lineno = 1 as libc::c_int;
    // 387: lineno: typeof(_242) = *mut {l391} i32
    // 387: lineno:   l391 = UNIQUE | NON_NULL, (empty)
    res = 0 as libc::c_int;
    lgl = 0 as *mut LGL;
    // 389: lgl = 0 as *mut LGL: typeof(_15 = const 0_usize as *mut LGL (PointerFromExposedAddress)) = *mut {l1414} LGL
    // 389: lgl = 0 as *mut LGL:   l1414 = UNIQUE | NON_NULL, (empty)
    loop {
        ch = getc(file);
        // 391: file: typeof(_246) = *mut {l396} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
        // 391: file:   l396 = UNIQUE | NON_NULL, (empty)
        if ch == -(1 as libc::c_int) {
            break;
        }
        if ch == '\r' as i32 {
            continue;
        }
        if ch != '\n' as i32 {
            if len + 1 as libc::c_int
                >= ::core::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong as libc::c_int
            {
                perr(b"line buffer exceeded\0" as *const u8 as *const libc::c_char);
                // 402: b"line buffer e ... _char: typeof(_272) = *const {l423} i8
                // 402: b"line buffer e ... _char:   l423 = UNIQUE | NON_NULL, (empty)
                // 402: b"line buffer e ... st u8: typeof(_273) = *const {l425} u8
                // 402: b"line buffer e ... st u8:   l425 = UNIQUE | NON_NULL, (empty)
                // 402: b"line buffer e ... ed\0": typeof(_274) = *const {l427} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                // 402: b"line buffer e ... ed\0":   l427 = UNIQUE | NON_NULL, (empty)
                // 402: b"line buffer e ... ed\0": typeof(_275) = & {l429} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                // 402: b"line buffer e ... ed\0":   l429 = UNIQUE | NON_NULL, FIXED
                // 402: b"line buffer e ... _char: typeof(_272 = move _273 as *const i8 (Misc)) = *const {l1418} i8
                // 402: b"line buffer e ... _char:   l1418 = UNIQUE | NON_NULL, (empty)
                // 402: b"line buffer e ... st u8: typeof(_273 = move _274 as *const u8 (Pointer(ArrayToPointer))) = *const {l1417} u8
                // 402: b"line buffer e ... st u8:   l1417 = UNIQUE | NON_NULL, (empty)
                // 402: b"line buffer e ... ed\0": typeof(_275 = const b"line buffer exceeded\x00") = & {l1415} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                // 402: b"line buffer e ... ed\0":   l1415 = UNIQUE | NON_NULL, (empty)
                // 402: b"line buffer e ... ed\0": typeof(_274 = &raw const (*_275)) = *const {l1416} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                // 402: b"line buffer e ... ed\0":   l1416 = UNIQUE | NON_NULL, (empty)
            }
            let fresh1 = len;
            len = len + 1;
            buffer[fresh1 as usize] = ch as libc::c_char;
            buffer[len as usize] = 0 as libc::c_int as libc::c_char;
        } else {
            msg(
                b"line %d : %s\0" as *const u8 as *const libc::c_char,
                // 410: b"line %d : %s\ ... _char: typeof(_290) = *const {l445} i8
                // 410: b"line %d : %s\ ... _char:   l445 = UNIQUE | NON_NULL, (empty)
                // 410: b"line %d : %s\ ... st u8: typeof(_291) = *const {l447} u8
                // 410: b"line %d : %s\ ... st u8:   l447 = UNIQUE | NON_NULL, (empty)
                // 410: b"line %d : %s\0": typeof(_292) = *const {l449} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 410: b"line %d : %s\0":   l449 = UNIQUE | NON_NULL, (empty)
                // 410: b"line %d : %s\0": typeof(_293) = & {l451} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 410: b"line %d : %s\0":   l451 = UNIQUE | NON_NULL, FIXED
                // 410: b"line %d : %s\ ... st u8: typeof(_291 = move _292 as *const u8 (Pointer(ArrayToPointer))) = *const {l1421} u8
                // 410: b"line %d : %s\ ... st u8:   l1421 = UNIQUE | NON_NULL, (empty)
                // 410: b"line %d : %s\0": typeof(_292 = &raw const (*_293)) = *const {l1420} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 410: b"line %d : %s\0":   l1420 = UNIQUE | NON_NULL, (empty)
                // 410: b"line %d : %s\ ... _char: typeof(_290 = move _291 as *const i8 (Misc)) = *const {l1422} i8
                // 410: b"line %d : %s\ ... _char:   l1422 = UNIQUE | NON_NULL, (empty)
                // 410: b"line %d : %s\0": typeof(_293 = const b"line %d : %s\x00") = & {l1419} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 410: b"line %d : %s\0":   l1419 = UNIQUE | NON_NULL, (empty)
                lineno,
                // 411: lineno: typeof(_295) = *mut {l454} i32
                // 411: lineno:   l454 = UNIQUE | NON_NULL, (empty)
                buffer.as_mut_ptr(),
                // 412: buffer.as_mut_ptr(): typeof(_296) = *mut {l456} i8
                // 412: buffer.as_mut_ptr():   l456 = UNIQUE | NON_NULL, (empty)
                // 412: buffer.as_mut_ptr(): typeof(_297) = &mut {l458} [i8]
                // 412: buffer.as_mut_ptr():   l458 = UNIQUE | NON_NULL, FIXED
                // 412: buffer.as_mut_ptr(): typeof(_298) = &mut {l460} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000050)) }]
                // 412: buffer.as_mut_ptr():   l460 = UNIQUE | NON_NULL, (empty)
                // 412: buffer.as_mut_ptr(): typeof(_298 = &mut _10) = &mut {l1423} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000050)) }]
                // 412: buffer.as_mut_ptr():   l1423 = UNIQUE | NON_NULL, (empty)
                // 412: buffer.as_mut_ptr(): typeof(_297 = move _298 as &mut [i8] (Pointer(Unsize))) = &mut {l1424} [i8]
                // 412: buffer.as_mut_ptr():   l1424 = UNIQUE | NON_NULL, FIXED
            );
            tok = strtok(
            // 414: strtok( buffer. ... ar, ): typeof(_299) = *mut {l462} i8
            // 414: strtok( buffer. ... ar, ):   l462 = UNIQUE | NON_NULL, (empty)
                buffer.as_mut_ptr(),
                // 415: buffer.as_mut_ptr(): typeof(_300) = *mut {l464} i8
                // 415: buffer.as_mut_ptr():   l464 = UNIQUE | NON_NULL, (empty)
                // 415: buffer.as_mut_ptr(): typeof(_301) = &mut {l466} [i8]
                // 415: buffer.as_mut_ptr():   l466 = UNIQUE | NON_NULL, FIXED
                // 415: buffer.as_mut_ptr(): typeof(_302) = &mut {l468} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000050)) }]
                // 415: buffer.as_mut_ptr():   l468 = UNIQUE | NON_NULL, (empty)
                // 415: buffer.as_mut_ptr(): typeof(_302 = &mut _10) = &mut {l1425} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000050)) }]
                // 415: buffer.as_mut_ptr():   l1425 = UNIQUE | NON_NULL, (empty)
                // 415: buffer.as_mut_ptr(): typeof(_301 = move _302 as &mut [i8] (Pointer(Unsize))) = &mut {l1426} [i8]
                // 415: buffer.as_mut_ptr():   l1426 = UNIQUE | NON_NULL, FIXED
                b" \0" as *const u8 as *const libc::c_char,
                // 416: b" \0" as *cons ... _char: typeof(_303) = *const {l470} i8
                // 416: b" \0" as *cons ... _char:   l470 = UNIQUE | NON_NULL, (empty)
                // 416: b" \0" as *const u8: typeof(_304) = *const {l472} u8
                // 416: b" \0" as *const u8:   l472 = UNIQUE | NON_NULL, (empty)
                // 416: b" \0": typeof(_305) = *const {l474} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 416: b" \0":   l474 = UNIQUE | NON_NULL, (empty)
                // 416: b" \0": typeof(_306) = & {l476} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 416: b" \0":   l476 = UNIQUE | NON_NULL, FIXED
                // 416: b" \0" as *cons ... _char: typeof(_303 = move _304 as *const i8 (Misc)) = *const {l1430} i8
                // 416: b" \0" as *cons ... _char:   l1430 = UNIQUE | NON_NULL, (empty)
                // 416: b" \0": typeof(_305 = &raw const (*_306)) = *const {l1428} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 416: b" \0":   l1428 = UNIQUE | NON_NULL, (empty)
                // 416: b" \0" as *const u8: typeof(_304 = move _305 as *const u8 (Pointer(ArrayToPointer))) = *const {l1429} u8
                // 416: b" \0" as *const u8:   l1429 = UNIQUE | NON_NULL, (empty)
                // 416: b" \0": typeof(_306 = const b" \x00") = & {l1427} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 416: b" \0":   l1427 = UNIQUE | NON_NULL, (empty)
            );
            if tok.is_null() {
            // 418: tok: typeof(_309) = *mut {l480} i8
            // 418: tok:   l480 = UNIQUE | NON_NULL, (empty)
                perr(b"empty line\0" as *const u8 as *const libc::c_char);
                // 419: b"empty line\0" ... _char: typeof(_311) = *const {l483} i8
                // 419: b"empty line\0" ... _char:   l483 = UNIQUE | NON_NULL, (empty)
                // 419: b"empty line\0" ... st u8: typeof(_312) = *const {l485} u8
                // 419: b"empty line\0" ... st u8:   l485 = UNIQUE | NON_NULL, (empty)
                // 419: b"empty line\0": typeof(_313) = *const {l487} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 419: b"empty line\0":   l487 = UNIQUE | NON_NULL, (empty)
                // 419: b"empty line\0": typeof(_314) = & {l489} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 419: b"empty line\0":   l489 = UNIQUE | NON_NULL, FIXED
                // 419: b"empty line\0" ... st u8: typeof(_312 = move _313 as *const u8 (Pointer(ArrayToPointer))) = *const {l1433} u8
                // 419: b"empty line\0" ... st u8:   l1433 = UNIQUE | NON_NULL, (empty)
                // 419: b"empty line\0": typeof(_313 = &raw const (*_314)) = *const {l1432} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 419: b"empty line\0":   l1432 = UNIQUE | NON_NULL, (empty)
                // 419: b"empty line\0": typeof(_314 = const b"empty line\x00") = & {l1431} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 419: b"empty line\0":   l1431 = UNIQUE | NON_NULL, (empty)
                // 419: b"empty line\0" ... _char: typeof(_311 = move _312 as *const i8 (Misc)) = *const {l1434} i8
                // 419: b"empty line\0" ... _char:   l1434 = UNIQUE | NON_NULL, (empty)
            } else if strcmp(tok, b"add\0" as *const u8 as *const libc::c_char) == 0 {
            // 420: tok: typeof(_317) = *const {l493} i8
            // 420: tok:   l493 = UNIQUE | NON_NULL, (empty)
            // 420: tok: typeof(_318) = *mut {l495} i8
            // 420: tok:   l495 = UNIQUE | NON_NULL, (empty)
            // 420: b"add\0" as *co ... _char: typeof(_319) = *const {l497} i8
            // 420: b"add\0" as *co ... _char:   l497 = UNIQUE | NON_NULL, (empty)
            // 420: b"add\0" as *co ... st u8: typeof(_320) = *const {l499} u8
            // 420: b"add\0" as *co ... st u8:   l499 = UNIQUE | NON_NULL, (empty)
            // 420: b"add\0": typeof(_321) = *const {l501} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 420: b"add\0":   l501 = UNIQUE | NON_NULL, (empty)
            // 420: b"add\0": typeof(_322) = & {l503} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 420: b"add\0":   l503 = UNIQUE | NON_NULL, FIXED
            // 420: b"add\0" as *co ... st u8: typeof(_320 = move _321 as *const u8 (Pointer(ArrayToPointer))) = *const {l1438} u8
            // 420: b"add\0" as *co ... st u8:   l1438 = UNIQUE | NON_NULL, (empty)
            // 420: b"add\0": typeof(_321 = &raw const (*_322)) = *const {l1437} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 420: b"add\0":   l1437 = UNIQUE | NON_NULL, (empty)
            // 420: b"add\0" as *co ... _char: typeof(_319 = move _320 as *const i8 (Misc)) = *const {l1439} i8
            // 420: b"add\0" as *co ... _char:   l1439 = UNIQUE | NON_NULL, (empty)
            // 420: tok: typeof(_317 = move _318 as *const i8 (Pointer(MutToConstPointer))) = *const {l1435} i8
            // 420: tok:   l1435 = UNIQUE | NON_NULL, (empty)
            // 420: b"add\0": typeof(_322 = const b"add\x00") = & {l1436} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 420: b"add\0":   l1436 = UNIQUE | NON_NULL, (empty)
                lgladd(
                    lgl,
                    // 422: lgl: typeof(_324) = *mut {l506} LGL
                    // 422: lgl:   l506 = UNIQUE | NON_NULL, (empty)
                    intarg(b"add\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 423: b"add\0" as *co ... _char: typeof(_326) = *mut {l509} i8
                    // 423: b"add\0" as *co ... _char:   l509 = UNIQUE | NON_NULL, (empty)
                    // 423: b"add\0" as *co ... _char: typeof(_327) = *const {l511} i8
                    // 423: b"add\0" as *co ... _char:   l511 = UNIQUE | NON_NULL, (empty)
                    // 423: b"add\0" as *co ... st u8: typeof(_328) = *const {l513} u8
                    // 423: b"add\0" as *co ... st u8:   l513 = UNIQUE | NON_NULL, (empty)
                    // 423: b"add\0": typeof(_329) = *const {l515} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 423: b"add\0":   l515 = UNIQUE | NON_NULL, (empty)
                    // 423: b"add\0": typeof(_330) = & {l517} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 423: b"add\0":   l517 = UNIQUE | NON_NULL, FIXED
                    // 423: b"add\0": typeof(_330 = const b"add\x00") = & {l1440} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 423: b"add\0":   l1440 = UNIQUE | NON_NULL, (empty)
                    // 423: b"add\0": typeof(_329 = &raw const (*_330)) = *const {l1441} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 423: b"add\0":   l1441 = UNIQUE | NON_NULL, (empty)
                    // 423: b"add\0" as *co ... st u8: typeof(_328 = move _329 as *const u8 (Pointer(ArrayToPointer))) = *const {l1442} u8
                    // 423: b"add\0" as *co ... st u8:   l1442 = UNIQUE | NON_NULL, (empty)
                    // 423: b"add\0" as *co ... _char: typeof(_327 = move _328 as *const i8 (Misc)) = *const {l1443} i8
                    // 423: b"add\0" as *co ... _char:   l1443 = UNIQUE | NON_NULL, (empty)
                    // 423: b"add\0" as *co ... _char: typeof(_326 = move _327 as *mut i8 (Misc)) = *mut {l1444} i8
                    // 423: b"add\0" as *co ... _char:   l1444 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"return\0" as *const u8 as *const libc::c_char) == 0 {
            // 425: tok: typeof(_333) = *const {l521} i8
            // 425: tok:   l521 = UNIQUE | NON_NULL, (empty)
            // 425: tok: typeof(_334) = *mut {l523} i8
            // 425: tok:   l523 = UNIQUE | NON_NULL, (empty)
            // 425: b"return\0" as  ... _char: typeof(_335) = *const {l525} i8
            // 425: b"return\0" as  ... _char:   l525 = UNIQUE | NON_NULL, (empty)
            // 425: b"return\0" as  ... st u8: typeof(_336) = *const {l527} u8
            // 425: b"return\0" as  ... st u8:   l527 = UNIQUE | NON_NULL, (empty)
            // 425: b"return\0": typeof(_337) = *const {l529} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 425: b"return\0":   l529 = UNIQUE | NON_NULL, (empty)
            // 425: b"return\0": typeof(_338) = & {l531} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 425: b"return\0":   l531 = UNIQUE | NON_NULL, FIXED
            // 425: b"return\0": typeof(_338 = const b"return\x00") = & {l1446} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 425: b"return\0":   l1446 = UNIQUE | NON_NULL, (empty)
            // 425: b"return\0" as  ... _char: typeof(_335 = move _336 as *const i8 (Misc)) = *const {l1449} i8
            // 425: b"return\0" as  ... _char:   l1449 = UNIQUE | NON_NULL, (empty)
            // 425: b"return\0" as  ... st u8: typeof(_336 = move _337 as *const u8 (Pointer(ArrayToPointer))) = *const {l1448} u8
            // 425: b"return\0" as  ... st u8:   l1448 = UNIQUE | NON_NULL, (empty)
            // 425: b"return\0": typeof(_337 = &raw const (*_338)) = *const {l1447} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 425: b"return\0":   l1447 = UNIQUE | NON_NULL, (empty)
            // 425: tok: typeof(_333 = move _334 as *const i8 (Pointer(MutToConstPointer))) = *const {l1445} i8
            // 425: tok:   l1445 = UNIQUE | NON_NULL, (empty)
                arg = intarg(b"return\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
                // 426: b"return\0" as  ... _char: typeof(_340) = *mut {l534} i8
                // 426: b"return\0" as  ... _char:   l534 = UNIQUE | NON_NULL, (empty)
                // 426: b"return\0" as  ... _char: typeof(_341) = *const {l536} i8
                // 426: b"return\0" as  ... _char:   l536 = UNIQUE | NON_NULL, (empty)
                // 426: b"return\0" as  ... st u8: typeof(_342) = *const {l538} u8
                // 426: b"return\0" as  ... st u8:   l538 = UNIQUE | NON_NULL, (empty)
                // 426: b"return\0": typeof(_343) = *const {l540} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 426: b"return\0":   l540 = UNIQUE | NON_NULL, (empty)
                // 426: b"return\0": typeof(_344) = & {l542} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 426: b"return\0":   l542 = UNIQUE | NON_NULL, FIXED
                // 426: b"return\0" as  ... st u8: typeof(_342 = move _343 as *const u8 (Pointer(ArrayToPointer))) = *const {l1452} u8
                // 426: b"return\0" as  ... st u8:   l1452 = UNIQUE | NON_NULL, (empty)
                // 426: b"return\0" as  ... _char: typeof(_340 = move _341 as *mut i8 (Misc)) = *mut {l1454} i8
                // 426: b"return\0" as  ... _char:   l1454 = UNIQUE | NON_NULL, (empty)
                // 426: b"return\0" as  ... _char: typeof(_341 = move _342 as *const i8 (Misc)) = *const {l1453} i8
                // 426: b"return\0" as  ... _char:   l1453 = UNIQUE | NON_NULL, (empty)
                // 426: b"return\0": typeof(_344 = const b"return\x00") = & {l1450} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 426: b"return\0":   l1450 = UNIQUE | NON_NULL, (empty)
                // 426: b"return\0": typeof(_343 = &raw const (*_344)) = *const {l1451} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 426: b"return\0":   l1451 = UNIQUE | NON_NULL, (empty)
                if arg != res {
                    die(
                        b"expected return value %d but got %d\0" as *const u8
                        // 429: b"expected retu ... _char: typeof(_349) = *const {l548} i8
                        // 429: b"expected retu ... _char:   l548 = UNIQUE | NON_NULL, (empty)
                        // 429: b"expected retu ... st u8: typeof(_350) = *const {l550} u8
                        // 429: b"expected retu ... st u8:   l550 = UNIQUE | NON_NULL, (empty)
                        // 429: b"expected retu ... %d\0": typeof(_351) = *const {l552} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
                        // 429: b"expected retu ... %d\0":   l552 = UNIQUE | NON_NULL, (empty)
                        // 429: b"expected retu ... %d\0": typeof(_352) = & {l554} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
                        // 429: b"expected retu ... %d\0":   l554 = UNIQUE | NON_NULL, FIXED
                        // 429: b"expected retu ... _char: typeof(_349 = move _350 as *const i8 (Misc)) = *const {l1458} i8
                        // 429: b"expected retu ... _char:   l1458 = UNIQUE | NON_NULL, (empty)
                        // 429: b"expected retu ... st u8: typeof(_350 = move _351 as *const u8 (Pointer(ArrayToPointer))) = *const {l1457} u8
                        // 429: b"expected retu ... st u8:   l1457 = UNIQUE | NON_NULL, (empty)
                        // 429: b"expected retu ... %d\0": typeof(_352 = const b"expected return value %d but got %d\x00") = & {l1455} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
                        // 429: b"expected retu ... %d\0":   l1455 = UNIQUE | NON_NULL, (empty)
                        // 429: b"expected retu ... %d\0": typeof(_351 = &raw const (*_352)) = *const {l1456} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
                        // 429: b"expected retu ... %d\0":   l1456 = UNIQUE | NON_NULL, (empty)
                            as *const libc::c_char,
                        arg,
                        res,
                    );
                }
            } else if strcmp(tok, b"deref\0" as *const u8 as *const libc::c_char) == 0 {
            // 435: tok: typeof(_357) = *const {l560} i8
            // 435: tok:   l560 = UNIQUE | NON_NULL, (empty)
            // 435: tok: typeof(_358) = *mut {l562} i8
            // 435: tok:   l562 = UNIQUE | NON_NULL, (empty)
            // 435: b"deref\0" as * ... _char: typeof(_359) = *const {l564} i8
            // 435: b"deref\0" as * ... _char:   l564 = UNIQUE | NON_NULL, (empty)
            // 435: b"deref\0" as * ... st u8: typeof(_360) = *const {l566} u8
            // 435: b"deref\0" as * ... st u8:   l566 = UNIQUE | NON_NULL, (empty)
            // 435: b"deref\0": typeof(_361) = *const {l568} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 435: b"deref\0":   l568 = UNIQUE | NON_NULL, (empty)
            // 435: b"deref\0": typeof(_362) = & {l570} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 435: b"deref\0":   l570 = UNIQUE | NON_NULL, FIXED
            // 435: b"deref\0": typeof(_361 = &raw const (*_362)) = *const {l1461} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 435: b"deref\0":   l1461 = UNIQUE | NON_NULL, (empty)
            // 435: b"deref\0" as * ... _char: typeof(_359 = move _360 as *const i8 (Misc)) = *const {l1463} i8
            // 435: b"deref\0" as * ... _char:   l1463 = UNIQUE | NON_NULL, (empty)
            // 435: b"deref\0": typeof(_362 = const b"deref\x00") = & {l1460} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 435: b"deref\0":   l1460 = UNIQUE | NON_NULL, (empty)
            // 435: b"deref\0" as * ... st u8: typeof(_360 = move _361 as *const u8 (Pointer(ArrayToPointer))) = *const {l1462} u8
            // 435: b"deref\0" as * ... st u8:   l1462 = UNIQUE | NON_NULL, (empty)
            // 435: tok: typeof(_357 = move _358 as *const i8 (Pointer(MutToConstPointer))) = *const {l1459} i8
            // 435: tok:   l1459 = UNIQUE | NON_NULL, (empty)
                res = lglderef(
                    lgl,
                    // 437: lgl: typeof(_364) = *mut {l573} LGL
                    // 437: lgl:   l573 = UNIQUE | NON_NULL, (empty)
                    intarg(b"deref\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 438: b"deref\0" as * ... _char: typeof(_366) = *mut {l576} i8
                    // 438: b"deref\0" as * ... _char:   l576 = UNIQUE | NON_NULL, (empty)
                    // 438: b"deref\0" as * ... _char: typeof(_367) = *const {l578} i8
                    // 438: b"deref\0" as * ... _char:   l578 = UNIQUE | NON_NULL, (empty)
                    // 438: b"deref\0" as * ... st u8: typeof(_368) = *const {l580} u8
                    // 438: b"deref\0" as * ... st u8:   l580 = UNIQUE | NON_NULL, (empty)
                    // 438: b"deref\0": typeof(_369) = *const {l582} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 438: b"deref\0":   l582 = UNIQUE | NON_NULL, (empty)
                    // 438: b"deref\0": typeof(_370) = & {l584} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 438: b"deref\0":   l584 = UNIQUE | NON_NULL, FIXED
                    // 438: b"deref\0" as * ... _char: typeof(_366 = move _367 as *mut i8 (Misc)) = *mut {l1468} i8
                    // 438: b"deref\0" as * ... _char:   l1468 = UNIQUE | NON_NULL, (empty)
                    // 438: b"deref\0": typeof(_369 = &raw const (*_370)) = *const {l1465} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 438: b"deref\0":   l1465 = UNIQUE | NON_NULL, (empty)
                    // 438: b"deref\0" as * ... st u8: typeof(_368 = move _369 as *const u8 (Pointer(ArrayToPointer))) = *const {l1466} u8
                    // 438: b"deref\0" as * ... st u8:   l1466 = UNIQUE | NON_NULL, (empty)
                    // 438: b"deref\0" as * ... _char: typeof(_367 = move _368 as *const i8 (Misc)) = *const {l1467} i8
                    // 438: b"deref\0" as * ... _char:   l1467 = UNIQUE | NON_NULL, (empty)
                    // 438: b"deref\0": typeof(_370 = const b"deref\x00") = & {l1464} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 438: b"deref\0":   l1464 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"failed\0" as *const u8 as *const libc::c_char) == 0 {
            // 440: tok: typeof(_373) = *const {l588} i8
            // 440: tok:   l588 = UNIQUE | NON_NULL, (empty)
            // 440: tok: typeof(_374) = *mut {l590} i8
            // 440: tok:   l590 = UNIQUE | NON_NULL, (empty)
            // 440: b"failed\0" as  ... _char: typeof(_375) = *const {l592} i8
            // 440: b"failed\0" as  ... _char:   l592 = UNIQUE | NON_NULL, (empty)
            // 440: b"failed\0" as  ... st u8: typeof(_376) = *const {l594} u8
            // 440: b"failed\0" as  ... st u8:   l594 = UNIQUE | NON_NULL, (empty)
            // 440: b"failed\0": typeof(_377) = *const {l596} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 440: b"failed\0":   l596 = UNIQUE | NON_NULL, (empty)
            // 440: b"failed\0": typeof(_378) = & {l598} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 440: b"failed\0":   l598 = UNIQUE | NON_NULL, FIXED
            // 440: b"failed\0": typeof(_377 = &raw const (*_378)) = *const {l1471} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 440: b"failed\0":   l1471 = UNIQUE | NON_NULL, (empty)
            // 440: b"failed\0" as  ... st u8: typeof(_376 = move _377 as *const u8 (Pointer(ArrayToPointer))) = *const {l1472} u8
            // 440: b"failed\0" as  ... st u8:   l1472 = UNIQUE | NON_NULL, (empty)
            // 440: b"failed\0": typeof(_378 = const b"failed\x00") = & {l1470} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 440: b"failed\0":   l1470 = UNIQUE | NON_NULL, (empty)
            // 440: tok: typeof(_373 = move _374 as *const i8 (Pointer(MutToConstPointer))) = *const {l1469} i8
            // 440: tok:   l1469 = UNIQUE | NON_NULL, (empty)
            // 440: b"failed\0" as  ... _char: typeof(_375 = move _376 as *const i8 (Misc)) = *const {l1473} i8
            // 440: b"failed\0" as  ... _char:   l1473 = UNIQUE | NON_NULL, (empty)
                res = lglfailed(
                    lgl,
                    // 442: lgl: typeof(_380) = *mut {l601} LGL
                    // 442: lgl:   l601 = UNIQUE | NON_NULL, (empty)
                    intarg(b"failed\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 443: b"failed\0" as  ... _char: typeof(_382) = *mut {l604} i8
                    // 443: b"failed\0" as  ... _char:   l604 = UNIQUE | NON_NULL, (empty)
                    // 443: b"failed\0" as  ... _char: typeof(_383) = *const {l606} i8
                    // 443: b"failed\0" as  ... _char:   l606 = UNIQUE | NON_NULL, (empty)
                    // 443: b"failed\0" as  ... st u8: typeof(_384) = *const {l608} u8
                    // 443: b"failed\0" as  ... st u8:   l608 = UNIQUE | NON_NULL, (empty)
                    // 443: b"failed\0": typeof(_385) = *const {l610} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 443: b"failed\0":   l610 = UNIQUE | NON_NULL, (empty)
                    // 443: b"failed\0": typeof(_386) = & {l612} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 443: b"failed\0":   l612 = UNIQUE | NON_NULL, FIXED
                    // 443: b"failed\0": typeof(_386 = const b"failed\x00") = & {l1474} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 443: b"failed\0":   l1474 = UNIQUE | NON_NULL, (empty)
                    // 443: b"failed\0": typeof(_385 = &raw const (*_386)) = *const {l1475} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 443: b"failed\0":   l1475 = UNIQUE | NON_NULL, (empty)
                    // 443: b"failed\0" as  ... _char: typeof(_382 = move _383 as *mut i8 (Misc)) = *mut {l1478} i8
                    // 443: b"failed\0" as  ... _char:   l1478 = UNIQUE | NON_NULL, (empty)
                    // 443: b"failed\0" as  ... st u8: typeof(_384 = move _385 as *const u8 (Pointer(ArrayToPointer))) = *const {l1476} u8
                    // 443: b"failed\0" as  ... st u8:   l1476 = UNIQUE | NON_NULL, (empty)
                    // 443: b"failed\0" as  ... _char: typeof(_383 = move _384 as *const i8 (Misc)) = *const {l1477} i8
                    // 443: b"failed\0" as  ... _char:   l1477 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"fixed\0" as *const u8 as *const libc::c_char) == 0 {
            // 445: tok: typeof(_389) = *const {l616} i8
            // 445: tok:   l616 = UNIQUE | NON_NULL, (empty)
            // 445: tok: typeof(_390) = *mut {l618} i8
            // 445: tok:   l618 = UNIQUE | NON_NULL, (empty)
            // 445: b"fixed\0" as * ... _char: typeof(_391) = *const {l620} i8
            // 445: b"fixed\0" as * ... _char:   l620 = UNIQUE | NON_NULL, (empty)
            // 445: b"fixed\0" as * ... st u8: typeof(_392) = *const {l622} u8
            // 445: b"fixed\0" as * ... st u8:   l622 = UNIQUE | NON_NULL, (empty)
            // 445: b"fixed\0": typeof(_393) = *const {l624} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 445: b"fixed\0":   l624 = UNIQUE | NON_NULL, (empty)
            // 445: b"fixed\0": typeof(_394) = & {l626} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 445: b"fixed\0":   l626 = UNIQUE | NON_NULL, FIXED
            // 445: b"fixed\0" as * ... st u8: typeof(_392 = move _393 as *const u8 (Pointer(ArrayToPointer))) = *const {l1482} u8
            // 445: b"fixed\0" as * ... st u8:   l1482 = UNIQUE | NON_NULL, (empty)
            // 445: b"fixed\0" as * ... _char: typeof(_391 = move _392 as *const i8 (Misc)) = *const {l1483} i8
            // 445: b"fixed\0" as * ... _char:   l1483 = UNIQUE | NON_NULL, (empty)
            // 445: b"fixed\0": typeof(_393 = &raw const (*_394)) = *const {l1481} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 445: b"fixed\0":   l1481 = UNIQUE | NON_NULL, (empty)
            // 445: b"fixed\0": typeof(_394 = const b"fixed\x00") = & {l1480} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 445: b"fixed\0":   l1480 = UNIQUE | NON_NULL, (empty)
            // 445: tok: typeof(_389 = move _390 as *const i8 (Pointer(MutToConstPointer))) = *const {l1479} i8
            // 445: tok:   l1479 = UNIQUE | NON_NULL, (empty)
                res = lglfixed(
                    lgl,
                    // 447: lgl: typeof(_396) = *mut {l629} LGL
                    // 447: lgl:   l629 = UNIQUE | NON_NULL, (empty)
                    intarg(b"fixed\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 448: b"fixed\0" as * ... _char: typeof(_398) = *mut {l632} i8
                    // 448: b"fixed\0" as * ... _char:   l632 = UNIQUE | NON_NULL, (empty)
                    // 448: b"fixed\0" as * ... _char: typeof(_399) = *const {l634} i8
                    // 448: b"fixed\0" as * ... _char:   l634 = UNIQUE | NON_NULL, (empty)
                    // 448: b"fixed\0" as * ... st u8: typeof(_400) = *const {l636} u8
                    // 448: b"fixed\0" as * ... st u8:   l636 = UNIQUE | NON_NULL, (empty)
                    // 448: b"fixed\0": typeof(_401) = *const {l638} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 448: b"fixed\0":   l638 = UNIQUE | NON_NULL, (empty)
                    // 448: b"fixed\0": typeof(_402) = & {l640} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 448: b"fixed\0":   l640 = UNIQUE | NON_NULL, FIXED
                    // 448: b"fixed\0" as * ... _char: typeof(_398 = move _399 as *mut i8 (Misc)) = *mut {l1488} i8
                    // 448: b"fixed\0" as * ... _char:   l1488 = UNIQUE | NON_NULL, (empty)
                    // 448: b"fixed\0" as * ... _char: typeof(_399 = move _400 as *const i8 (Misc)) = *const {l1487} i8
                    // 448: b"fixed\0" as * ... _char:   l1487 = UNIQUE | NON_NULL, (empty)
                    // 448: b"fixed\0": typeof(_401 = &raw const (*_402)) = *const {l1485} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 448: b"fixed\0":   l1485 = UNIQUE | NON_NULL, (empty)
                    // 448: b"fixed\0" as * ... st u8: typeof(_400 = move _401 as *const u8 (Pointer(ArrayToPointer))) = *const {l1486} u8
                    // 448: b"fixed\0" as * ... st u8:   l1486 = UNIQUE | NON_NULL, (empty)
                    // 448: b"fixed\0": typeof(_402 = const b"fixed\x00") = & {l1484} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 448: b"fixed\0":   l1484 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"repr\0" as *const u8 as *const libc::c_char) == 0 {
            // 450: tok: typeof(_405) = *const {l644} i8
            // 450: tok:   l644 = UNIQUE | NON_NULL, (empty)
            // 450: tok: typeof(_406) = *mut {l646} i8
            // 450: tok:   l646 = UNIQUE | NON_NULL, (empty)
            // 450: b"repr\0" as *c ... _char: typeof(_407) = *const {l648} i8
            // 450: b"repr\0" as *c ... _char:   l648 = UNIQUE | NON_NULL, (empty)
            // 450: b"repr\0" as *c ... st u8: typeof(_408) = *const {l650} u8
            // 450: b"repr\0" as *c ... st u8:   l650 = UNIQUE | NON_NULL, (empty)
            // 450: b"repr\0": typeof(_409) = *const {l652} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 450: b"repr\0":   l652 = UNIQUE | NON_NULL, (empty)
            // 450: b"repr\0": typeof(_410) = & {l654} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 450: b"repr\0":   l654 = UNIQUE | NON_NULL, FIXED
            // 450: b"repr\0" as *c ... _char: typeof(_407 = move _408 as *const i8 (Misc)) = *const {l1493} i8
            // 450: b"repr\0" as *c ... _char:   l1493 = UNIQUE | NON_NULL, (empty)
            // 450: b"repr\0": typeof(_410 = const b"repr\x00") = & {l1490} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 450: b"repr\0":   l1490 = UNIQUE | NON_NULL, (empty)
            // 450: b"repr\0": typeof(_409 = &raw const (*_410)) = *const {l1491} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 450: b"repr\0":   l1491 = UNIQUE | NON_NULL, (empty)
            // 450: b"repr\0" as *c ... st u8: typeof(_408 = move _409 as *const u8 (Pointer(ArrayToPointer))) = *const {l1492} u8
            // 450: b"repr\0" as *c ... st u8:   l1492 = UNIQUE | NON_NULL, (empty)
            // 450: tok: typeof(_405 = move _406 as *const i8 (Pointer(MutToConstPointer))) = *const {l1489} i8
            // 450: tok:   l1489 = UNIQUE | NON_NULL, (empty)
                res = lglrepr(
                    lgl,
                    // 452: lgl: typeof(_412) = *mut {l657} LGL
                    // 452: lgl:   l657 = UNIQUE | NON_NULL, (empty)
                    intarg(b"repr\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 453: b"repr\0" as *c ... _char: typeof(_414) = *mut {l660} i8
                    // 453: b"repr\0" as *c ... _char:   l660 = UNIQUE | NON_NULL, (empty)
                    // 453: b"repr\0" as *c ... _char: typeof(_415) = *const {l662} i8
                    // 453: b"repr\0" as *c ... _char:   l662 = UNIQUE | NON_NULL, (empty)
                    // 453: b"repr\0" as *c ... st u8: typeof(_416) = *const {l664} u8
                    // 453: b"repr\0" as *c ... st u8:   l664 = UNIQUE | NON_NULL, (empty)
                    // 453: b"repr\0": typeof(_417) = *const {l666} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 453: b"repr\0":   l666 = UNIQUE | NON_NULL, (empty)
                    // 453: b"repr\0": typeof(_418) = & {l668} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 453: b"repr\0":   l668 = UNIQUE | NON_NULL, FIXED
                    // 453: b"repr\0": typeof(_417 = &raw const (*_418)) = *const {l1495} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 453: b"repr\0":   l1495 = UNIQUE | NON_NULL, (empty)
                    // 453: b"repr\0": typeof(_418 = const b"repr\x00") = & {l1494} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 453: b"repr\0":   l1494 = UNIQUE | NON_NULL, (empty)
                    // 453: b"repr\0" as *c ... st u8: typeof(_416 = move _417 as *const u8 (Pointer(ArrayToPointer))) = *const {l1496} u8
                    // 453: b"repr\0" as *c ... st u8:   l1496 = UNIQUE | NON_NULL, (empty)
                    // 453: b"repr\0" as *c ... _char: typeof(_414 = move _415 as *mut i8 (Misc)) = *mut {l1498} i8
                    // 453: b"repr\0" as *c ... _char:   l1498 = UNIQUE | NON_NULL, (empty)
                    // 453: b"repr\0" as *c ... _char: typeof(_415 = move _416 as *const i8 (Misc)) = *const {l1497} i8
                    // 453: b"repr\0" as *c ... _char:   l1497 = UNIQUE | NON_NULL, (empty)
                );
            } else if noarg(
                tok,
                // 456: tok: typeof(_421) = *const {l672} i8
                // 456: tok:   l672 = UNIQUE | NON_NULL, (empty)
                // 456: tok: typeof(_422) = *mut {l674} i8
                // 456: tok:   l674 = UNIQUE | NON_NULL, (empty)
                // 456: tok: typeof(_421 = move _422 as *const i8 (Pointer(MutToConstPointer))) = *const {l1499} i8
                // 456: tok:   l1499 = UNIQUE | NON_NULL, (empty)
                b"incvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                // 457: b"incvar\0" as  ... _char: typeof(_423) = *mut {l676} i8
                // 457: b"incvar\0" as  ... _char:   l676 = UNIQUE | NON_NULL, (empty)
                // 457: b"incvar\0" as  ... _char: typeof(_424) = *const {l678} i8
                // 457: b"incvar\0" as  ... _char:   l678 = UNIQUE | NON_NULL, (empty)
                // 457: b"incvar\0" as  ... st u8: typeof(_425) = *const {l680} u8
                // 457: b"incvar\0" as  ... st u8:   l680 = UNIQUE | NON_NULL, (empty)
                // 457: b"incvar\0": typeof(_426) = *const {l682} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 457: b"incvar\0":   l682 = UNIQUE | NON_NULL, (empty)
                // 457: b"incvar\0": typeof(_427) = & {l684} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 457: b"incvar\0":   l684 = UNIQUE | NON_NULL, FIXED
                // 457: b"incvar\0" as  ... st u8: typeof(_425 = move _426 as *const u8 (Pointer(ArrayToPointer))) = *const {l1502} u8
                // 457: b"incvar\0" as  ... st u8:   l1502 = UNIQUE | NON_NULL, (empty)
                // 457: b"incvar\0" as  ... _char: typeof(_424 = move _425 as *const i8 (Misc)) = *const {l1503} i8
                // 457: b"incvar\0" as  ... _char:   l1503 = UNIQUE | NON_NULL, (empty)
                // 457: b"incvar\0": typeof(_426 = &raw const (*_427)) = *const {l1501} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 457: b"incvar\0":   l1501 = UNIQUE | NON_NULL, (empty)
                // 457: b"incvar\0": typeof(_427 = const b"incvar\x00") = & {l1500} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 457: b"incvar\0":   l1500 = UNIQUE | NON_NULL, (empty)
                // 457: b"incvar\0" as  ... _char: typeof(_423 = move _424 as *mut i8 (Misc)) = *mut {l1504} i8
                // 457: b"incvar\0" as  ... _char:   l1504 = UNIQUE | NON_NULL, (empty)
            ) != 0
            {
                res = lglincvar(lgl);
                // 460: lgl: typeof(_429) = *mut {l687} LGL
                // 460: lgl:   l687 = UNIQUE | NON_NULL, (empty)
            } else if noarg(
                tok,
                // 462: tok: typeof(_432) = *const {l691} i8
                // 462: tok:   l691 = UNIQUE | NON_NULL, (empty)
                // 462: tok: typeof(_433) = *mut {l693} i8
                // 462: tok:   l693 = UNIQUE | NON_NULL, (empty)
                // 462: tok: typeof(_432 = move _433 as *const i8 (Pointer(MutToConstPointer))) = *const {l1505} i8
                // 462: tok:   l1505 = UNIQUE | NON_NULL, (empty)
                b"maxvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                // 463: b"maxvar\0" as  ... _char: typeof(_434) = *mut {l695} i8
                // 463: b"maxvar\0" as  ... _char:   l695 = UNIQUE | NON_NULL, (empty)
                // 463: b"maxvar\0" as  ... _char: typeof(_435) = *const {l697} i8
                // 463: b"maxvar\0" as  ... _char:   l697 = UNIQUE | NON_NULL, (empty)
                // 463: b"maxvar\0" as  ... st u8: typeof(_436) = *const {l699} u8
                // 463: b"maxvar\0" as  ... st u8:   l699 = UNIQUE | NON_NULL, (empty)
                // 463: b"maxvar\0": typeof(_437) = *const {l701} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 463: b"maxvar\0":   l701 = UNIQUE | NON_NULL, (empty)
                // 463: b"maxvar\0": typeof(_438) = & {l703} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 463: b"maxvar\0":   l703 = UNIQUE | NON_NULL, FIXED
                // 463: b"maxvar\0" as  ... _char: typeof(_434 = move _435 as *mut i8 (Misc)) = *mut {l1510} i8
                // 463: b"maxvar\0" as  ... _char:   l1510 = UNIQUE | NON_NULL, (empty)
                // 463: b"maxvar\0" as  ... st u8: typeof(_436 = move _437 as *const u8 (Pointer(ArrayToPointer))) = *const {l1508} u8
                // 463: b"maxvar\0" as  ... st u8:   l1508 = UNIQUE | NON_NULL, (empty)
                // 463: b"maxvar\0": typeof(_437 = &raw const (*_438)) = *const {l1507} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 463: b"maxvar\0":   l1507 = UNIQUE | NON_NULL, (empty)
                // 463: b"maxvar\0" as  ... _char: typeof(_435 = move _436 as *const i8 (Misc)) = *const {l1509} i8
                // 463: b"maxvar\0" as  ... _char:   l1509 = UNIQUE | NON_NULL, (empty)
                // 463: b"maxvar\0": typeof(_438 = const b"maxvar\x00") = & {l1506} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 463: b"maxvar\0":   l1506 = UNIQUE | NON_NULL, (empty)
            ) != 0
            {
                res = lglmaxvar(lgl);
                // 466: lgl: typeof(_440) = *mut {l706} LGL
                // 466: lgl:   l706 = UNIQUE | NON_NULL, (empty)
            } else if noarg(
                tok,
                // 468: tok: typeof(_443) = *const {l710} i8
                // 468: tok:   l710 = UNIQUE | NON_NULL, (empty)
                // 468: tok: typeof(_444) = *mut {l712} i8
                // 468: tok:   l712 = UNIQUE | NON_NULL, (empty)
                // 468: tok: typeof(_443 = move _444 as *const i8 (Pointer(MutToConstPointer))) = *const {l1511} i8
                // 468: tok:   l1511 = UNIQUE | NON_NULL, (empty)
                b"changed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                // 469: b"changed\0" as ... _char: typeof(_445) = *mut {l714} i8
                // 469: b"changed\0" as ... _char:   l714 = UNIQUE | NON_NULL, (empty)
                // 469: b"changed\0" as ... _char: typeof(_446) = *const {l716} i8
                // 469: b"changed\0" as ... _char:   l716 = UNIQUE | NON_NULL, (empty)
                // 469: b"changed\0" as ... st u8: typeof(_447) = *const {l718} u8
                // 469: b"changed\0" as ... st u8:   l718 = UNIQUE | NON_NULL, (empty)
                // 469: b"changed\0": typeof(_448) = *const {l720} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 469: b"changed\0":   l720 = UNIQUE | NON_NULL, (empty)
                // 469: b"changed\0": typeof(_449) = & {l722} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 469: b"changed\0":   l722 = UNIQUE | NON_NULL, FIXED
                // 469: b"changed\0": typeof(_448 = &raw const (*_449)) = *const {l1513} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 469: b"changed\0":   l1513 = UNIQUE | NON_NULL, (empty)
                // 469: b"changed\0" as ... _char: typeof(_446 = move _447 as *const i8 (Misc)) = *const {l1515} i8
                // 469: b"changed\0" as ... _char:   l1515 = UNIQUE | NON_NULL, (empty)
                // 469: b"changed\0" as ... _char: typeof(_445 = move _446 as *mut i8 (Misc)) = *mut {l1516} i8
                // 469: b"changed\0" as ... _char:   l1516 = UNIQUE | NON_NULL, (empty)
                // 469: b"changed\0": typeof(_449 = const b"changed\x00") = & {l1512} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 469: b"changed\0":   l1512 = UNIQUE | NON_NULL, (empty)
                // 469: b"changed\0" as ... st u8: typeof(_447 = move _448 as *const u8 (Pointer(ArrayToPointer))) = *const {l1514} u8
                // 469: b"changed\0" as ... st u8:   l1514 = UNIQUE | NON_NULL, (empty)
            ) != 0
            {
                res = lglchanged(lgl);
                // 472: lgl: typeof(_451) = *mut {l725} LGL
                // 472: lgl:   l725 = UNIQUE | NON_NULL, (empty)
            } else if noarg(
                tok,
                // 474: tok: typeof(_454) = *const {l729} i8
                // 474: tok:   l729 = UNIQUE | NON_NULL, (empty)
                // 474: tok: typeof(_455) = *mut {l731} i8
                // 474: tok:   l731 = UNIQUE | NON_NULL, (empty)
                // 474: tok: typeof(_454 = move _455 as *const i8 (Pointer(MutToConstPointer))) = *const {l1517} i8
                // 474: tok:   l1517 = UNIQUE | NON_NULL, (empty)
                b"inconsistent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                // 475: b"inconsistent\ ... _char: typeof(_456) = *mut {l733} i8
                // 475: b"inconsistent\ ... _char:   l733 = UNIQUE | NON_NULL, (empty)
                // 475: b"inconsistent\ ... _char: typeof(_457) = *const {l735} i8
                // 475: b"inconsistent\ ... _char:   l735 = UNIQUE | NON_NULL, (empty)
                // 475: b"inconsistent\ ... st u8: typeof(_458) = *const {l737} u8
                // 475: b"inconsistent\ ... st u8:   l737 = UNIQUE | NON_NULL, (empty)
                // 475: b"inconsistent\0": typeof(_459) = *const {l739} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 475: b"inconsistent\0":   l739 = UNIQUE | NON_NULL, (empty)
                // 475: b"inconsistent\0": typeof(_460) = & {l741} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 475: b"inconsistent\0":   l741 = UNIQUE | NON_NULL, FIXED
                // 475: b"inconsistent\0": typeof(_459 = &raw const (*_460)) = *const {l1519} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 475: b"inconsistent\0":   l1519 = UNIQUE | NON_NULL, (empty)
                // 475: b"inconsistent\ ... _char: typeof(_456 = move _457 as *mut i8 (Misc)) = *mut {l1522} i8
                // 475: b"inconsistent\ ... _char:   l1522 = UNIQUE | NON_NULL, (empty)
                // 475: b"inconsistent\0": typeof(_460 = const b"inconsistent\x00") = & {l1518} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 475: b"inconsistent\0":   l1518 = UNIQUE | NON_NULL, (empty)
                // 475: b"inconsistent\ ... _char: typeof(_457 = move _458 as *const i8 (Misc)) = *const {l1521} i8
                // 475: b"inconsistent\ ... _char:   l1521 = UNIQUE | NON_NULL, (empty)
                // 475: b"inconsistent\ ... st u8: typeof(_458 = move _459 as *const u8 (Pointer(ArrayToPointer))) = *const {l1520} u8
                // 475: b"inconsistent\ ... st u8:   l1520 = UNIQUE | NON_NULL, (empty)
            ) != 0
            {
                res = lglinconsistent(lgl);
                // 478: lgl: typeof(_462) = *mut {l744} LGL
                // 478: lgl:   l744 = UNIQUE | NON_NULL, (empty)
            } else if noarg(
                tok,
                // 480: tok: typeof(_465) = *const {l748} i8
                // 480: tok:   l748 = UNIQUE | NON_NULL, (empty)
                // 480: tok: typeof(_466) = *mut {l750} i8
                // 480: tok:   l750 = UNIQUE | NON_NULL, (empty)
                // 480: tok: typeof(_465 = move _466 as *const i8 (Pointer(MutToConstPointer))) = *const {l1523} i8
                // 480: tok:   l1523 = UNIQUE | NON_NULL, (empty)
                b"lkhd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                // 481: b"lkhd\0" as *c ... _char: typeof(_467) = *mut {l752} i8
                // 481: b"lkhd\0" as *c ... _char:   l752 = UNIQUE | NON_NULL, (empty)
                // 481: b"lkhd\0" as *c ... _char: typeof(_468) = *const {l754} i8
                // 481: b"lkhd\0" as *c ... _char:   l754 = UNIQUE | NON_NULL, (empty)
                // 481: b"lkhd\0" as *c ... st u8: typeof(_469) = *const {l756} u8
                // 481: b"lkhd\0" as *c ... st u8:   l756 = UNIQUE | NON_NULL, (empty)
                // 481: b"lkhd\0": typeof(_470) = *const {l758} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 481: b"lkhd\0":   l758 = UNIQUE | NON_NULL, (empty)
                // 481: b"lkhd\0": typeof(_471) = & {l760} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 481: b"lkhd\0":   l760 = UNIQUE | NON_NULL, FIXED
                // 481: b"lkhd\0" as *c ... _char: typeof(_468 = move _469 as *const i8 (Misc)) = *const {l1527} i8
                // 481: b"lkhd\0" as *c ... _char:   l1527 = UNIQUE | NON_NULL, (empty)
                // 481: b"lkhd\0" as *c ... _char: typeof(_467 = move _468 as *mut i8 (Misc)) = *mut {l1528} i8
                // 481: b"lkhd\0" as *c ... _char:   l1528 = UNIQUE | NON_NULL, (empty)
                // 481: b"lkhd\0": typeof(_471 = const b"lkhd\x00") = & {l1524} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 481: b"lkhd\0":   l1524 = UNIQUE | NON_NULL, (empty)
                // 481: b"lkhd\0": typeof(_470 = &raw const (*_471)) = *const {l1525} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 481: b"lkhd\0":   l1525 = UNIQUE | NON_NULL, (empty)
                // 481: b"lkhd\0" as *c ... st u8: typeof(_469 = move _470 as *const u8 (Pointer(ArrayToPointer))) = *const {l1526} u8
                // 481: b"lkhd\0" as *c ... st u8:   l1526 = UNIQUE | NON_NULL, (empty)
            ) != 0
            {
                res = lglookahead(lgl);
                // 484: lgl: typeof(_473) = *mut {l763} LGL
                // 484: lgl:   l763 = UNIQUE | NON_NULL, (empty)
            } else if noarg(
                tok,
                // 486: tok: typeof(_476) = *const {l767} i8
                // 486: tok:   l767 = UNIQUE | NON_NULL, (empty)
                // 486: tok: typeof(_477) = *mut {l769} i8
                // 486: tok:   l769 = UNIQUE | NON_NULL, (empty)
                // 486: tok: typeof(_476 = move _477 as *const i8 (Pointer(MutToConstPointer))) = *const {l1529} i8
                // 486: tok:   l1529 = UNIQUE | NON_NULL, (empty)
                b"fixate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                // 487: b"fixate\0" as  ... _char: typeof(_478) = *mut {l771} i8
                // 487: b"fixate\0" as  ... _char:   l771 = UNIQUE | NON_NULL, (empty)
                // 487: b"fixate\0" as  ... _char: typeof(_479) = *const {l773} i8
                // 487: b"fixate\0" as  ... _char:   l773 = UNIQUE | NON_NULL, (empty)
                // 487: b"fixate\0" as  ... st u8: typeof(_480) = *const {l775} u8
                // 487: b"fixate\0" as  ... st u8:   l775 = UNIQUE | NON_NULL, (empty)
                // 487: b"fixate\0": typeof(_481) = *const {l777} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 487: b"fixate\0":   l777 = UNIQUE | NON_NULL, (empty)
                // 487: b"fixate\0": typeof(_482) = & {l779} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 487: b"fixate\0":   l779 = UNIQUE | NON_NULL, FIXED
                // 487: b"fixate\0" as  ... st u8: typeof(_480 = move _481 as *const u8 (Pointer(ArrayToPointer))) = *const {l1532} u8
                // 487: b"fixate\0" as  ... st u8:   l1532 = UNIQUE | NON_NULL, (empty)
                // 487: b"fixate\0": typeof(_481 = &raw const (*_482)) = *const {l1531} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 487: b"fixate\0":   l1531 = UNIQUE | NON_NULL, (empty)
                // 487: b"fixate\0": typeof(_482 = const b"fixate\x00") = & {l1530} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 487: b"fixate\0":   l1530 = UNIQUE | NON_NULL, (empty)
                // 487: b"fixate\0" as  ... _char: typeof(_479 = move _480 as *const i8 (Misc)) = *const {l1533} i8
                // 487: b"fixate\0" as  ... _char:   l1533 = UNIQUE | NON_NULL, (empty)
                // 487: b"fixate\0" as  ... _char: typeof(_478 = move _479 as *mut i8 (Misc)) = *mut {l1534} i8
                // 487: b"fixate\0" as  ... _char:   l1534 = UNIQUE | NON_NULL, (empty)
            ) != 0
            {
                lglfixate(lgl);
                // 490: lgl: typeof(_484) = *mut {l782} LGL
                // 490: lgl:   l782 = UNIQUE | NON_NULL, (empty)
            } else if noarg(
                tok,
                // 492: tok: typeof(_487) = *const {l786} i8
                // 492: tok:   l786 = UNIQUE | NON_NULL, (empty)
                // 492: tok: typeof(_488) = *mut {l788} i8
                // 492: tok:   l788 = UNIQUE | NON_NULL, (empty)
                // 492: tok: typeof(_487 = move _488 as *const i8 (Pointer(MutToConstPointer))) = *const {l1535} i8
                // 492: tok:   l1535 = UNIQUE | NON_NULL, (empty)
                b"reduce\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                // 493: b"reduce\0" as  ... _char: typeof(_489) = *mut {l790} i8
                // 493: b"reduce\0" as  ... _char:   l790 = UNIQUE | NON_NULL, (empty)
                // 493: b"reduce\0" as  ... _char: typeof(_490) = *const {l792} i8
                // 493: b"reduce\0" as  ... _char:   l792 = UNIQUE | NON_NULL, (empty)
                // 493: b"reduce\0" as  ... st u8: typeof(_491) = *const {l794} u8
                // 493: b"reduce\0" as  ... st u8:   l794 = UNIQUE | NON_NULL, (empty)
                // 493: b"reduce\0": typeof(_492) = *const {l796} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 493: b"reduce\0":   l796 = UNIQUE | NON_NULL, (empty)
                // 493: b"reduce\0": typeof(_493) = & {l798} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 493: b"reduce\0":   l798 = UNIQUE | NON_NULL, FIXED
                // 493: b"reduce\0": typeof(_493 = const b"reduce\x00") = & {l1536} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 493: b"reduce\0":   l1536 = UNIQUE | NON_NULL, (empty)
                // 493: b"reduce\0": typeof(_492 = &raw const (*_493)) = *const {l1537} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 493: b"reduce\0":   l1537 = UNIQUE | NON_NULL, (empty)
                // 493: b"reduce\0" as  ... _char: typeof(_490 = move _491 as *const i8 (Misc)) = *const {l1539} i8
                // 493: b"reduce\0" as  ... _char:   l1539 = UNIQUE | NON_NULL, (empty)
                // 493: b"reduce\0" as  ... st u8: typeof(_491 = move _492 as *const u8 (Pointer(ArrayToPointer))) = *const {l1538} u8
                // 493: b"reduce\0" as  ... st u8:   l1538 = UNIQUE | NON_NULL, (empty)
                // 493: b"reduce\0" as  ... _char: typeof(_489 = move _490 as *mut i8 (Misc)) = *mut {l1540} i8
                // 493: b"reduce\0" as  ... _char:   l1540 = UNIQUE | NON_NULL, (empty)
            ) != 0
            {
                lglreducecache(lgl);
                // 496: lgl: typeof(_495) = *mut {l801} LGL
                // 496: lgl:   l801 = UNIQUE | NON_NULL, (empty)
            } else if noarg(
                tok,
                // 498: tok: typeof(_498) = *const {l805} i8
                // 498: tok:   l805 = UNIQUE | NON_NULL, (empty)
                // 498: tok: typeof(_499) = *mut {l807} i8
                // 498: tok:   l807 = UNIQUE | NON_NULL, (empty)
                // 498: tok: typeof(_498 = move _499 as *const i8 (Pointer(MutToConstPointer))) = *const {l1541} i8
                // 498: tok:   l1541 = UNIQUE | NON_NULL, (empty)
                b"flush\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                // 499: b"flush\0" as * ... _char: typeof(_500) = *mut {l809} i8
                // 499: b"flush\0" as * ... _char:   l809 = UNIQUE | NON_NULL, (empty)
                // 499: b"flush\0" as * ... _char: typeof(_501) = *const {l811} i8
                // 499: b"flush\0" as * ... _char:   l811 = UNIQUE | NON_NULL, (empty)
                // 499: b"flush\0" as * ... st u8: typeof(_502) = *const {l813} u8
                // 499: b"flush\0" as * ... st u8:   l813 = UNIQUE | NON_NULL, (empty)
                // 499: b"flush\0": typeof(_503) = *const {l815} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 499: b"flush\0":   l815 = UNIQUE | NON_NULL, (empty)
                // 499: b"flush\0": typeof(_504) = & {l817} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 499: b"flush\0":   l817 = UNIQUE | NON_NULL, FIXED
                // 499: b"flush\0": typeof(_504 = const b"flush\x00") = & {l1542} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 499: b"flush\0":   l1542 = UNIQUE | NON_NULL, (empty)
                // 499: b"flush\0" as * ... _char: typeof(_501 = move _502 as *const i8 (Misc)) = *const {l1545} i8
                // 499: b"flush\0" as * ... _char:   l1545 = UNIQUE | NON_NULL, (empty)
                // 499: b"flush\0" as * ... st u8: typeof(_502 = move _503 as *const u8 (Pointer(ArrayToPointer))) = *const {l1544} u8
                // 499: b"flush\0" as * ... st u8:   l1544 = UNIQUE | NON_NULL, (empty)
                // 499: b"flush\0": typeof(_503 = &raw const (*_504)) = *const {l1543} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 499: b"flush\0":   l1543 = UNIQUE | NON_NULL, (empty)
                // 499: b"flush\0" as * ... _char: typeof(_500 = move _501 as *mut i8 (Misc)) = *mut {l1546} i8
                // 499: b"flush\0" as * ... _char:   l1546 = UNIQUE | NON_NULL, (empty)
            ) != 0
            {
                lglflushcache(lgl);
                // 502: lgl: typeof(_506) = *mut {l820} LGL
                // 502: lgl:   l820 = UNIQUE | NON_NULL, (empty)
            } else if noarg(
                tok,
                // 504: tok: typeof(_509) = *const {l824} i8
                // 504: tok:   l824 = UNIQUE | NON_NULL, (empty)
                // 504: tok: typeof(_510) = *mut {l826} i8
                // 504: tok:   l826 = UNIQUE | NON_NULL, (empty)
                // 504: tok: typeof(_509 = move _510 as *const i8 (Pointer(MutToConstPointer))) = *const {l1547} i8
                // 504: tok:   l1547 = UNIQUE | NON_NULL, (empty)
                b"chkclone\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                // 505: b"chkclone\0" a ... _char: typeof(_511) = *mut {l828} i8
                // 505: b"chkclone\0" a ... _char:   l828 = UNIQUE | NON_NULL, (empty)
                // 505: b"chkclone\0" a ... _char: typeof(_512) = *const {l830} i8
                // 505: b"chkclone\0" a ... _char:   l830 = UNIQUE | NON_NULL, (empty)
                // 505: b"chkclone\0" a ... st u8: typeof(_513) = *const {l832} u8
                // 505: b"chkclone\0" a ... st u8:   l832 = UNIQUE | NON_NULL, (empty)
                // 505: b"chkclone\0": typeof(_514) = *const {l834} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 505: b"chkclone\0":   l834 = UNIQUE | NON_NULL, (empty)
                // 505: b"chkclone\0": typeof(_515) = & {l836} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 505: b"chkclone\0":   l836 = UNIQUE | NON_NULL, FIXED
                // 505: b"chkclone\0" a ... _char: typeof(_512 = move _513 as *const i8 (Misc)) = *const {l1551} i8
                // 505: b"chkclone\0" a ... _char:   l1551 = UNIQUE | NON_NULL, (empty)
                // 505: b"chkclone\0" a ... _char: typeof(_511 = move _512 as *mut i8 (Misc)) = *mut {l1552} i8
                // 505: b"chkclone\0" a ... _char:   l1552 = UNIQUE | NON_NULL, (empty)
                // 505: b"chkclone\0" a ... st u8: typeof(_513 = move _514 as *const u8 (Pointer(ArrayToPointer))) = *const {l1550} u8
                // 505: b"chkclone\0" a ... st u8:   l1550 = UNIQUE | NON_NULL, (empty)
                // 505: b"chkclone\0": typeof(_515 = const b"chkclone\x00") = & {l1548} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 505: b"chkclone\0":   l1548 = UNIQUE | NON_NULL, (empty)
                // 505: b"chkclone\0": typeof(_514 = &raw const (*_515)) = *const {l1549} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 505: b"chkclone\0":   l1549 = UNIQUE | NON_NULL, (empty)
            ) != 0
            {
                lglchkclone(lgl);
                // 508: lgl: typeof(_517) = *mut {l839} LGL
                // 508: lgl:   l839 = UNIQUE | NON_NULL, (empty)
            } else if strcmp(tok, b"assume\0" as *const u8 as *const libc::c_char) == 0 {
            // 509: tok: typeof(_520) = *const {l843} i8
            // 509: tok:   l843 = UNIQUE | NON_NULL, (empty)
            // 509: tok: typeof(_521) = *mut {l845} i8
            // 509: tok:   l845 = UNIQUE | NON_NULL, (empty)
            // 509: b"assume\0" as  ... _char: typeof(_522) = *const {l847} i8
            // 509: b"assume\0" as  ... _char:   l847 = UNIQUE | NON_NULL, (empty)
            // 509: b"assume\0" as  ... st u8: typeof(_523) = *const {l849} u8
            // 509: b"assume\0" as  ... st u8:   l849 = UNIQUE | NON_NULL, (empty)
            // 509: b"assume\0": typeof(_524) = *const {l851} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 509: b"assume\0":   l851 = UNIQUE | NON_NULL, (empty)
            // 509: b"assume\0": typeof(_525) = & {l853} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 509: b"assume\0":   l853 = UNIQUE | NON_NULL, FIXED
            // 509: tok: typeof(_520 = move _521 as *const i8 (Pointer(MutToConstPointer))) = *const {l1553} i8
            // 509: tok:   l1553 = UNIQUE | NON_NULL, (empty)
            // 509: b"assume\0": typeof(_525 = const b"assume\x00") = & {l1554} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 509: b"assume\0":   l1554 = UNIQUE | NON_NULL, (empty)
            // 509: b"assume\0": typeof(_524 = &raw const (*_525)) = *const {l1555} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 509: b"assume\0":   l1555 = UNIQUE | NON_NULL, (empty)
            // 509: b"assume\0" as  ... st u8: typeof(_523 = move _524 as *const u8 (Pointer(ArrayToPointer))) = *const {l1556} u8
            // 509: b"assume\0" as  ... st u8:   l1556 = UNIQUE | NON_NULL, (empty)
            // 509: b"assume\0" as  ... _char: typeof(_522 = move _523 as *const i8 (Misc)) = *const {l1557} i8
            // 509: b"assume\0" as  ... _char:   l1557 = UNIQUE | NON_NULL, (empty)
                lglassume(
                    lgl,
                    // 511: lgl: typeof(_527) = *mut {l856} LGL
                    // 511: lgl:   l856 = UNIQUE | NON_NULL, (empty)
                    intarg(b"assume\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 512: b"assume\0" as  ... _char: typeof(_529) = *mut {l859} i8
                    // 512: b"assume\0" as  ... _char:   l859 = UNIQUE | NON_NULL, (empty)
                    // 512: b"assume\0" as  ... _char: typeof(_530) = *const {l861} i8
                    // 512: b"assume\0" as  ... _char:   l861 = UNIQUE | NON_NULL, (empty)
                    // 512: b"assume\0" as  ... st u8: typeof(_531) = *const {l863} u8
                    // 512: b"assume\0" as  ... st u8:   l863 = UNIQUE | NON_NULL, (empty)
                    // 512: b"assume\0": typeof(_532) = *const {l865} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 512: b"assume\0":   l865 = UNIQUE | NON_NULL, (empty)
                    // 512: b"assume\0": typeof(_533) = & {l867} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 512: b"assume\0":   l867 = UNIQUE | NON_NULL, FIXED
                    // 512: b"assume\0": typeof(_533 = const b"assume\x00") = & {l1558} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 512: b"assume\0":   l1558 = UNIQUE | NON_NULL, (empty)
                    // 512: b"assume\0": typeof(_532 = &raw const (*_533)) = *const {l1559} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 512: b"assume\0":   l1559 = UNIQUE | NON_NULL, (empty)
                    // 512: b"assume\0" as  ... st u8: typeof(_531 = move _532 as *const u8 (Pointer(ArrayToPointer))) = *const {l1560} u8
                    // 512: b"assume\0" as  ... st u8:   l1560 = UNIQUE | NON_NULL, (empty)
                    // 512: b"assume\0" as  ... _char: typeof(_530 = move _531 as *const i8 (Misc)) = *const {l1561} i8
                    // 512: b"assume\0" as  ... _char:   l1561 = UNIQUE | NON_NULL, (empty)
                    // 512: b"assume\0" as  ... _char: typeof(_529 = move _530 as *mut i8 (Misc)) = *mut {l1562} i8
                    // 512: b"assume\0" as  ... _char:   l1562 = UNIQUE | NON_NULL, (empty)
                );
            } else if noarg(
                tok,
                // 515: tok: typeof(_536) = *const {l871} i8
                // 515: tok:   l871 = UNIQUE | NON_NULL, (empty)
                // 515: tok: typeof(_537) = *mut {l873} i8
                // 515: tok:   l873 = UNIQUE | NON_NULL, (empty)
                // 515: tok: typeof(_536 = move _537 as *const i8 (Pointer(MutToConstPointer))) = *const {l1563} i8
                // 515: tok:   l1563 = UNIQUE | NON_NULL, (empty)
                b"init\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                // 516: b"init\0" as *c ... _char: typeof(_538) = *mut {l875} i8
                // 516: b"init\0" as *c ... _char:   l875 = UNIQUE | NON_NULL, (empty)
                // 516: b"init\0" as *c ... _char: typeof(_539) = *const {l877} i8
                // 516: b"init\0" as *c ... _char:   l877 = UNIQUE | NON_NULL, (empty)
                // 516: b"init\0" as *c ... st u8: typeof(_540) = *const {l879} u8
                // 516: b"init\0" as *c ... st u8:   l879 = UNIQUE | NON_NULL, (empty)
                // 516: b"init\0": typeof(_541) = *const {l881} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 516: b"init\0":   l881 = UNIQUE | NON_NULL, (empty)
                // 516: b"init\0": typeof(_542) = & {l883} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 516: b"init\0":   l883 = UNIQUE | NON_NULL, FIXED
                // 516: b"init\0": typeof(_542 = const b"init\x00") = & {l1564} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 516: b"init\0":   l1564 = UNIQUE | NON_NULL, (empty)
                // 516: b"init\0" as *c ... _char: typeof(_539 = move _540 as *const i8 (Misc)) = *const {l1567} i8
                // 516: b"init\0" as *c ... _char:   l1567 = UNIQUE | NON_NULL, (empty)
                // 516: b"init\0" as *c ... _char: typeof(_538 = move _539 as *mut i8 (Misc)) = *mut {l1568} i8
                // 516: b"init\0" as *c ... _char:   l1568 = UNIQUE | NON_NULL, (empty)
                // 516: b"init\0": typeof(_541 = &raw const (*_542)) = *const {l1565} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 516: b"init\0":   l1565 = UNIQUE | NON_NULL, (empty)
                // 516: b"init\0" as *c ... st u8: typeof(_540 = move _541 as *const u8 (Pointer(ArrayToPointer))) = *const {l1566} u8
                // 516: b"init\0" as *c ... st u8:   l1566 = UNIQUE | NON_NULL, (empty)
            ) != 0
            {
                lgl = lglinit();
                // 519: lglinit(): typeof(_543) = *mut {l885} LGL
                // 519: lglinit():   l885 = UNIQUE | NON_NULL, (empty)
            } else if noarg(
                tok,
                // 521: tok: typeof(_546) = *const {l889} i8
                // 521: tok:   l889 = UNIQUE | NON_NULL, (empty)
                // 521: tok: typeof(_547) = *mut {l891} i8
                // 521: tok:   l891 = UNIQUE | NON_NULL, (empty)
                // 521: tok: typeof(_546 = move _547 as *const i8 (Pointer(MutToConstPointer))) = *const {l1569} i8
                // 521: tok:   l1569 = UNIQUE | NON_NULL, (empty)
                b"sat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                // 522: b"sat\0" as *co ... _char: typeof(_548) = *mut {l893} i8
                // 522: b"sat\0" as *co ... _char:   l893 = UNIQUE | NON_NULL, (empty)
                // 522: b"sat\0" as *co ... _char: typeof(_549) = *const {l895} i8
                // 522: b"sat\0" as *co ... _char:   l895 = UNIQUE | NON_NULL, (empty)
                // 522: b"sat\0" as *co ... st u8: typeof(_550) = *const {l897} u8
                // 522: b"sat\0" as *co ... st u8:   l897 = UNIQUE | NON_NULL, (empty)
                // 522: b"sat\0": typeof(_551) = *const {l899} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 522: b"sat\0":   l899 = UNIQUE | NON_NULL, (empty)
                // 522: b"sat\0": typeof(_552) = & {l901} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 522: b"sat\0":   l901 = UNIQUE | NON_NULL, FIXED
                // 522: b"sat\0": typeof(_551 = &raw const (*_552)) = *const {l1571} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 522: b"sat\0":   l1571 = UNIQUE | NON_NULL, (empty)
                // 522: b"sat\0" as *co ... _char: typeof(_549 = move _550 as *const i8 (Misc)) = *const {l1573} i8
                // 522: b"sat\0" as *co ... _char:   l1573 = UNIQUE | NON_NULL, (empty)
                // 522: b"sat\0" as *co ... st u8: typeof(_550 = move _551 as *const u8 (Pointer(ArrayToPointer))) = *const {l1572} u8
                // 522: b"sat\0" as *co ... st u8:   l1572 = UNIQUE | NON_NULL, (empty)
                // 522: b"sat\0": typeof(_552 = const b"sat\x00") = & {l1570} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 522: b"sat\0":   l1570 = UNIQUE | NON_NULL, (empty)
                // 522: b"sat\0" as *co ... _char: typeof(_548 = move _549 as *mut i8 (Misc)) = *mut {l1574} i8
                // 522: b"sat\0" as *co ... _char:   l1574 = UNIQUE | NON_NULL, (empty)
            ) != 0
            {
                res = lglsat(lgl);
                // 525: lgl: typeof(_554) = *mut {l904} LGL
                // 525: lgl:   l904 = UNIQUE | NON_NULL, (empty)
            } else if strcmp(tok, b"simp\0" as *const u8 as *const libc::c_char) == 0 {
            // 526: tok: typeof(_557) = *const {l908} i8
            // 526: tok:   l908 = UNIQUE | NON_NULL, (empty)
            // 526: tok: typeof(_558) = *mut {l910} i8
            // 526: tok:   l910 = UNIQUE | NON_NULL, (empty)
            // 526: b"simp\0" as *c ... _char: typeof(_559) = *const {l912} i8
            // 526: b"simp\0" as *c ... _char:   l912 = UNIQUE | NON_NULL, (empty)
            // 526: b"simp\0" as *c ... st u8: typeof(_560) = *const {l914} u8
            // 526: b"simp\0" as *c ... st u8:   l914 = UNIQUE | NON_NULL, (empty)
            // 526: b"simp\0": typeof(_561) = *const {l916} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 526: b"simp\0":   l916 = UNIQUE | NON_NULL, (empty)
            // 526: b"simp\0": typeof(_562) = & {l918} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 526: b"simp\0":   l918 = UNIQUE | NON_NULL, FIXED
            // 526: b"simp\0" as *c ... st u8: typeof(_560 = move _561 as *const u8 (Pointer(ArrayToPointer))) = *const {l1578} u8
            // 526: b"simp\0" as *c ... st u8:   l1578 = UNIQUE | NON_NULL, (empty)
            // 526: tok: typeof(_557 = move _558 as *const i8 (Pointer(MutToConstPointer))) = *const {l1575} i8
            // 526: tok:   l1575 = UNIQUE | NON_NULL, (empty)
            // 526: b"simp\0": typeof(_562 = const b"simp\x00") = & {l1576} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 526: b"simp\0":   l1576 = UNIQUE | NON_NULL, (empty)
            // 526: b"simp\0": typeof(_561 = &raw const (*_562)) = *const {l1577} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 526: b"simp\0":   l1577 = UNIQUE | NON_NULL, (empty)
            // 526: b"simp\0" as *c ... _char: typeof(_559 = move _560 as *const i8 (Misc)) = *const {l1579} i8
            // 526: b"simp\0" as *c ... _char:   l1579 = UNIQUE | NON_NULL, (empty)
                res = lglsimp(
                    lgl,
                    // 528: lgl: typeof(_564) = *mut {l921} LGL
                    // 528: lgl:   l921 = UNIQUE | NON_NULL, (empty)
                    intarg(b"simp\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 529: b"simp\0" as *c ... _char: typeof(_566) = *mut {l924} i8
                    // 529: b"simp\0" as *c ... _char:   l924 = UNIQUE | NON_NULL, (empty)
                    // 529: b"simp\0" as *c ... _char: typeof(_567) = *const {l926} i8
                    // 529: b"simp\0" as *c ... _char:   l926 = UNIQUE | NON_NULL, (empty)
                    // 529: b"simp\0" as *c ... st u8: typeof(_568) = *const {l928} u8
                    // 529: b"simp\0" as *c ... st u8:   l928 = UNIQUE | NON_NULL, (empty)
                    // 529: b"simp\0": typeof(_569) = *const {l930} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 529: b"simp\0":   l930 = UNIQUE | NON_NULL, (empty)
                    // 529: b"simp\0": typeof(_570) = & {l932} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 529: b"simp\0":   l932 = UNIQUE | NON_NULL, FIXED
                    // 529: b"simp\0" as *c ... st u8: typeof(_568 = move _569 as *const u8 (Pointer(ArrayToPointer))) = *const {l1582} u8
                    // 529: b"simp\0" as *c ... st u8:   l1582 = UNIQUE | NON_NULL, (empty)
                    // 529: b"simp\0" as *c ... _char: typeof(_566 = move _567 as *mut i8 (Misc)) = *mut {l1584} i8
                    // 529: b"simp\0" as *c ... _char:   l1584 = UNIQUE | NON_NULL, (empty)
                    // 529: b"simp\0": typeof(_569 = &raw const (*_570)) = *const {l1581} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 529: b"simp\0":   l1581 = UNIQUE | NON_NULL, (empty)
                    // 529: b"simp\0": typeof(_570 = const b"simp\x00") = & {l1580} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 529: b"simp\0":   l1580 = UNIQUE | NON_NULL, (empty)
                    // 529: b"simp\0" as *c ... _char: typeof(_567 = move _568 as *const i8 (Misc)) = *const {l1583} i8
                    // 529: b"simp\0" as *c ... _char:   l1583 = UNIQUE | NON_NULL, (empty)
                );
            } else if noarg(
                tok,
                // 532: tok: typeof(_573) = *const {l936} i8
                // 532: tok:   l936 = UNIQUE | NON_NULL, (empty)
                // 532: tok: typeof(_574) = *mut {l938} i8
                // 532: tok:   l938 = UNIQUE | NON_NULL, (empty)
                // 532: tok: typeof(_573 = move _574 as *const i8 (Pointer(MutToConstPointer))) = *const {l1585} i8
                // 532: tok:   l1585 = UNIQUE | NON_NULL, (empty)
                b"stats\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                // 533: b"stats\0" as * ... _char: typeof(_575) = *mut {l940} i8
                // 533: b"stats\0" as * ... _char:   l940 = UNIQUE | NON_NULL, (empty)
                // 533: b"stats\0" as * ... _char: typeof(_576) = *const {l942} i8
                // 533: b"stats\0" as * ... _char:   l942 = UNIQUE | NON_NULL, (empty)
                // 533: b"stats\0" as * ... st u8: typeof(_577) = *const {l944} u8
                // 533: b"stats\0" as * ... st u8:   l944 = UNIQUE | NON_NULL, (empty)
                // 533: b"stats\0": typeof(_578) = *const {l946} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 533: b"stats\0":   l946 = UNIQUE | NON_NULL, (empty)
                // 533: b"stats\0": typeof(_579) = & {l948} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 533: b"stats\0":   l948 = UNIQUE | NON_NULL, FIXED
                // 533: b"stats\0" as * ... _char: typeof(_575 = move _576 as *mut i8 (Misc)) = *mut {l1590} i8
                // 533: b"stats\0" as * ... _char:   l1590 = UNIQUE | NON_NULL, (empty)
                // 533: b"stats\0" as * ... _char: typeof(_576 = move _577 as *const i8 (Misc)) = *const {l1589} i8
                // 533: b"stats\0" as * ... _char:   l1589 = UNIQUE | NON_NULL, (empty)
                // 533: b"stats\0": typeof(_579 = const b"stats\x00") = & {l1586} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 533: b"stats\0":   l1586 = UNIQUE | NON_NULL, (empty)
                // 533: b"stats\0": typeof(_578 = &raw const (*_579)) = *const {l1587} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 533: b"stats\0":   l1587 = UNIQUE | NON_NULL, (empty)
                // 533: b"stats\0" as * ... st u8: typeof(_577 = move _578 as *const u8 (Pointer(ArrayToPointer))) = *const {l1588} u8
                // 533: b"stats\0" as * ... st u8:   l1588 = UNIQUE | NON_NULL, (empty)
            ) != 0
            {
                lglstats(lgl);
                // 536: lgl: typeof(_581) = *mut {l951} LGL
                // 536: lgl:   l951 = UNIQUE | NON_NULL, (empty)
            } else if strcmp(tok, b"freeze\0" as *const u8 as *const libc::c_char) == 0 {
            // 537: tok: typeof(_584) = *const {l955} i8
            // 537: tok:   l955 = UNIQUE | NON_NULL, (empty)
            // 537: tok: typeof(_585) = *mut {l957} i8
            // 537: tok:   l957 = UNIQUE | NON_NULL, (empty)
            // 537: b"freeze\0" as  ... _char: typeof(_586) = *const {l959} i8
            // 537: b"freeze\0" as  ... _char:   l959 = UNIQUE | NON_NULL, (empty)
            // 537: b"freeze\0" as  ... st u8: typeof(_587) = *const {l961} u8
            // 537: b"freeze\0" as  ... st u8:   l961 = UNIQUE | NON_NULL, (empty)
            // 537: b"freeze\0": typeof(_588) = *const {l963} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 537: b"freeze\0":   l963 = UNIQUE | NON_NULL, (empty)
            // 537: b"freeze\0": typeof(_589) = & {l965} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 537: b"freeze\0":   l965 = UNIQUE | NON_NULL, FIXED
            // 537: b"freeze\0" as  ... _char: typeof(_586 = move _587 as *const i8 (Misc)) = *const {l1595} i8
            // 537: b"freeze\0" as  ... _char:   l1595 = UNIQUE | NON_NULL, (empty)
            // 537: b"freeze\0" as  ... st u8: typeof(_587 = move _588 as *const u8 (Pointer(ArrayToPointer))) = *const {l1594} u8
            // 537: b"freeze\0" as  ... st u8:   l1594 = UNIQUE | NON_NULL, (empty)
            // 537: b"freeze\0": typeof(_588 = &raw const (*_589)) = *const {l1593} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 537: b"freeze\0":   l1593 = UNIQUE | NON_NULL, (empty)
            // 537: b"freeze\0": typeof(_589 = const b"freeze\x00") = & {l1592} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 537: b"freeze\0":   l1592 = UNIQUE | NON_NULL, (empty)
            // 537: tok: typeof(_584 = move _585 as *const i8 (Pointer(MutToConstPointer))) = *const {l1591} i8
            // 537: tok:   l1591 = UNIQUE | NON_NULL, (empty)
                lglfreeze(
                    lgl,
                    // 539: lgl: typeof(_591) = *mut {l968} LGL
                    // 539: lgl:   l968 = UNIQUE | NON_NULL, (empty)
                    intarg(b"freeze\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 540: b"freeze\0" as  ... _char: typeof(_593) = *mut {l971} i8
                    // 540: b"freeze\0" as  ... _char:   l971 = UNIQUE | NON_NULL, (empty)
                    // 540: b"freeze\0" as  ... _char: typeof(_594) = *const {l973} i8
                    // 540: b"freeze\0" as  ... _char:   l973 = UNIQUE | NON_NULL, (empty)
                    // 540: b"freeze\0" as  ... st u8: typeof(_595) = *const {l975} u8
                    // 540: b"freeze\0" as  ... st u8:   l975 = UNIQUE | NON_NULL, (empty)
                    // 540: b"freeze\0": typeof(_596) = *const {l977} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 540: b"freeze\0":   l977 = UNIQUE | NON_NULL, (empty)
                    // 540: b"freeze\0": typeof(_597) = & {l979} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 540: b"freeze\0":   l979 = UNIQUE | NON_NULL, FIXED
                    // 540: b"freeze\0": typeof(_596 = &raw const (*_597)) = *const {l1597} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 540: b"freeze\0":   l1597 = UNIQUE | NON_NULL, (empty)
                    // 540: b"freeze\0": typeof(_597 = const b"freeze\x00") = & {l1596} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 540: b"freeze\0":   l1596 = UNIQUE | NON_NULL, (empty)
                    // 540: b"freeze\0" as  ... st u8: typeof(_595 = move _596 as *const u8 (Pointer(ArrayToPointer))) = *const {l1598} u8
                    // 540: b"freeze\0" as  ... st u8:   l1598 = UNIQUE | NON_NULL, (empty)
                    // 540: b"freeze\0" as  ... _char: typeof(_594 = move _595 as *const i8 (Misc)) = *const {l1599} i8
                    // 540: b"freeze\0" as  ... _char:   l1599 = UNIQUE | NON_NULL, (empty)
                    // 540: b"freeze\0" as  ... _char: typeof(_593 = move _594 as *mut i8 (Misc)) = *mut {l1600} i8
                    // 540: b"freeze\0" as  ... _char:   l1600 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"melt\0" as *const u8 as *const libc::c_char) == 0 {
            // 542: tok: typeof(_600) = *const {l983} i8
            // 542: tok:   l983 = UNIQUE | NON_NULL, (empty)
            // 542: tok: typeof(_601) = *mut {l985} i8
            // 542: tok:   l985 = UNIQUE | NON_NULL, (empty)
            // 542: b"melt\0" as *c ... _char: typeof(_602) = *const {l987} i8
            // 542: b"melt\0" as *c ... _char:   l987 = UNIQUE | NON_NULL, (empty)
            // 542: b"melt\0" as *c ... st u8: typeof(_603) = *const {l989} u8
            // 542: b"melt\0" as *c ... st u8:   l989 = UNIQUE | NON_NULL, (empty)
            // 542: b"melt\0": typeof(_604) = *const {l991} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 542: b"melt\0":   l991 = UNIQUE | NON_NULL, (empty)
            // 542: b"melt\0": typeof(_605) = & {l993} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 542: b"melt\0":   l993 = UNIQUE | NON_NULL, FIXED
            // 542: tok: typeof(_600 = move _601 as *const i8 (Pointer(MutToConstPointer))) = *const {l1601} i8
            // 542: tok:   l1601 = UNIQUE | NON_NULL, (empty)
            // 542: b"melt\0": typeof(_604 = &raw const (*_605)) = *const {l1603} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 542: b"melt\0":   l1603 = UNIQUE | NON_NULL, (empty)
            // 542: b"melt\0": typeof(_605 = const b"melt\x00") = & {l1602} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 542: b"melt\0":   l1602 = UNIQUE | NON_NULL, (empty)
            // 542: b"melt\0" as *c ... st u8: typeof(_603 = move _604 as *const u8 (Pointer(ArrayToPointer))) = *const {l1604} u8
            // 542: b"melt\0" as *c ... st u8:   l1604 = UNIQUE | NON_NULL, (empty)
            // 542: b"melt\0" as *c ... _char: typeof(_602 = move _603 as *const i8 (Misc)) = *const {l1605} i8
            // 542: b"melt\0" as *c ... _char:   l1605 = UNIQUE | NON_NULL, (empty)
                lglmelt(
                    lgl,
                    // 544: lgl: typeof(_607) = *mut {l996} LGL
                    // 544: lgl:   l996 = UNIQUE | NON_NULL, (empty)
                    intarg(b"melt\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 545: b"melt\0" as *c ... _char: typeof(_609) = *mut {l999} i8
                    // 545: b"melt\0" as *c ... _char:   l999 = UNIQUE | NON_NULL, (empty)
                    // 545: b"melt\0" as *c ... _char: typeof(_610) = *const {l1001} i8
                    // 545: b"melt\0" as *c ... _char:   l1001 = UNIQUE | NON_NULL, (empty)
                    // 545: b"melt\0" as *c ... st u8: typeof(_611) = *const {l1003} u8
                    // 545: b"melt\0" as *c ... st u8:   l1003 = UNIQUE | NON_NULL, (empty)
                    // 545: b"melt\0": typeof(_612) = *const {l1005} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 545: b"melt\0":   l1005 = UNIQUE | NON_NULL, (empty)
                    // 545: b"melt\0": typeof(_613) = & {l1007} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 545: b"melt\0":   l1007 = UNIQUE | NON_NULL, FIXED
                    // 545: b"melt\0" as *c ... _char: typeof(_609 = move _610 as *mut i8 (Misc)) = *mut {l1610} i8
                    // 545: b"melt\0" as *c ... _char:   l1610 = UNIQUE | NON_NULL, (empty)
                    // 545: b"melt\0" as *c ... st u8: typeof(_611 = move _612 as *const u8 (Pointer(ArrayToPointer))) = *const {l1608} u8
                    // 545: b"melt\0" as *c ... st u8:   l1608 = UNIQUE | NON_NULL, (empty)
                    // 545: b"melt\0" as *c ... _char: typeof(_610 = move _611 as *const i8 (Misc)) = *const {l1609} i8
                    // 545: b"melt\0" as *c ... _char:   l1609 = UNIQUE | NON_NULL, (empty)
                    // 545: b"melt\0": typeof(_612 = &raw const (*_613)) = *const {l1607} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 545: b"melt\0":   l1607 = UNIQUE | NON_NULL, (empty)
                    // 545: b"melt\0": typeof(_613 = const b"melt\x00") = & {l1606} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 545: b"melt\0":   l1606 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"reuse\0" as *const u8 as *const libc::c_char) == 0 {
            // 547: tok: typeof(_616) = *const {l1011} i8
            // 547: tok:   l1011 = UNIQUE | NON_NULL, (empty)
            // 547: tok: typeof(_617) = *mut {l1013} i8
            // 547: tok:   l1013 = UNIQUE | NON_NULL, (empty)
            // 547: b"reuse\0" as * ... _char: typeof(_618) = *const {l1015} i8
            // 547: b"reuse\0" as * ... _char:   l1015 = UNIQUE | NON_NULL, (empty)
            // 547: b"reuse\0" as * ... st u8: typeof(_619) = *const {l1017} u8
            // 547: b"reuse\0" as * ... st u8:   l1017 = UNIQUE | NON_NULL, (empty)
            // 547: b"reuse\0": typeof(_620) = *const {l1019} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 547: b"reuse\0":   l1019 = UNIQUE | NON_NULL, (empty)
            // 547: b"reuse\0": typeof(_621) = & {l1021} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 547: b"reuse\0":   l1021 = UNIQUE | NON_NULL, FIXED
            // 547: b"reuse\0" as * ... st u8: typeof(_619 = move _620 as *const u8 (Pointer(ArrayToPointer))) = *const {l1614} u8
            // 547: b"reuse\0" as * ... st u8:   l1614 = UNIQUE | NON_NULL, (empty)
            // 547: tok: typeof(_616 = move _617 as *const i8 (Pointer(MutToConstPointer))) = *const {l1611} i8
            // 547: tok:   l1611 = UNIQUE | NON_NULL, (empty)
            // 547: b"reuse\0": typeof(_620 = &raw const (*_621)) = *const {l1613} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 547: b"reuse\0":   l1613 = UNIQUE | NON_NULL, (empty)
            // 547: b"reuse\0" as * ... _char: typeof(_618 = move _619 as *const i8 (Misc)) = *const {l1615} i8
            // 547: b"reuse\0" as * ... _char:   l1615 = UNIQUE | NON_NULL, (empty)
            // 547: b"reuse\0": typeof(_621 = const b"reuse\x00") = & {l1612} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 547: b"reuse\0":   l1612 = UNIQUE | NON_NULL, (empty)
                lglreuse(
                    lgl,
                    // 549: lgl: typeof(_623) = *mut {l1024} LGL
                    // 549: lgl:   l1024 = UNIQUE | NON_NULL, (empty)
                    intarg(b"reuse\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 550: b"reuse\0" as * ... _char: typeof(_625) = *mut {l1027} i8
                    // 550: b"reuse\0" as * ... _char:   l1027 = UNIQUE | NON_NULL, (empty)
                    // 550: b"reuse\0" as * ... _char: typeof(_626) = *const {l1029} i8
                    // 550: b"reuse\0" as * ... _char:   l1029 = UNIQUE | NON_NULL, (empty)
                    // 550: b"reuse\0" as * ... st u8: typeof(_627) = *const {l1031} u8
                    // 550: b"reuse\0" as * ... st u8:   l1031 = UNIQUE | NON_NULL, (empty)
                    // 550: b"reuse\0": typeof(_628) = *const {l1033} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 550: b"reuse\0":   l1033 = UNIQUE | NON_NULL, (empty)
                    // 550: b"reuse\0": typeof(_629) = & {l1035} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 550: b"reuse\0":   l1035 = UNIQUE | NON_NULL, FIXED
                    // 550: b"reuse\0" as * ... _char: typeof(_625 = move _626 as *mut i8 (Misc)) = *mut {l1620} i8
                    // 550: b"reuse\0" as * ... _char:   l1620 = UNIQUE | NON_NULL, (empty)
                    // 550: b"reuse\0": typeof(_628 = &raw const (*_629)) = *const {l1617} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 550: b"reuse\0":   l1617 = UNIQUE | NON_NULL, (empty)
                    // 550: b"reuse\0" as * ... st u8: typeof(_627 = move _628 as *const u8 (Pointer(ArrayToPointer))) = *const {l1618} u8
                    // 550: b"reuse\0" as * ... st u8:   l1618 = UNIQUE | NON_NULL, (empty)
                    // 550: b"reuse\0": typeof(_629 = const b"reuse\x00") = & {l1616} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 550: b"reuse\0":   l1616 = UNIQUE | NON_NULL, (empty)
                    // 550: b"reuse\0" as * ... _char: typeof(_626 = move _627 as *const i8 (Misc)) = *const {l1619} i8
                    // 550: b"reuse\0" as * ... _char:   l1619 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"frozen\0" as *const u8 as *const libc::c_char) == 0 {
            // 552: tok: typeof(_632) = *const {l1039} i8
            // 552: tok:   l1039 = UNIQUE | NON_NULL, (empty)
            // 552: tok: typeof(_633) = *mut {l1041} i8
            // 552: tok:   l1041 = UNIQUE | NON_NULL, (empty)
            // 552: b"frozen\0" as  ... _char: typeof(_634) = *const {l1043} i8
            // 552: b"frozen\0" as  ... _char:   l1043 = UNIQUE | NON_NULL, (empty)
            // 552: b"frozen\0" as  ... st u8: typeof(_635) = *const {l1045} u8
            // 552: b"frozen\0" as  ... st u8:   l1045 = UNIQUE | NON_NULL, (empty)
            // 552: b"frozen\0": typeof(_636) = *const {l1047} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 552: b"frozen\0":   l1047 = UNIQUE | NON_NULL, (empty)
            // 552: b"frozen\0": typeof(_637) = & {l1049} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 552: b"frozen\0":   l1049 = UNIQUE | NON_NULL, FIXED
            // 552: b"frozen\0" as  ... st u8: typeof(_635 = move _636 as *const u8 (Pointer(ArrayToPointer))) = *const {l1624} u8
            // 552: b"frozen\0" as  ... st u8:   l1624 = UNIQUE | NON_NULL, (empty)
            // 552: tok: typeof(_632 = move _633 as *const i8 (Pointer(MutToConstPointer))) = *const {l1621} i8
            // 552: tok:   l1621 = UNIQUE | NON_NULL, (empty)
            // 552: b"frozen\0": typeof(_636 = &raw const (*_637)) = *const {l1623} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 552: b"frozen\0":   l1623 = UNIQUE | NON_NULL, (empty)
            // 552: b"frozen\0": typeof(_637 = const b"frozen\x00") = & {l1622} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 552: b"frozen\0":   l1622 = UNIQUE | NON_NULL, (empty)
            // 552: b"frozen\0" as  ... _char: typeof(_634 = move _635 as *const i8 (Misc)) = *const {l1625} i8
            // 552: b"frozen\0" as  ... _char:   l1625 = UNIQUE | NON_NULL, (empty)
                res = lglfrozen(
                    lgl,
                    // 554: lgl: typeof(_639) = *mut {l1052} LGL
                    // 554: lgl:   l1052 = UNIQUE | NON_NULL, (empty)
                    intarg(b"frozen\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 555: b"frozen\0" as  ... _char: typeof(_641) = *mut {l1055} i8
                    // 555: b"frozen\0" as  ... _char:   l1055 = UNIQUE | NON_NULL, (empty)
                    // 555: b"frozen\0" as  ... _char: typeof(_642) = *const {l1057} i8
                    // 555: b"frozen\0" as  ... _char:   l1057 = UNIQUE | NON_NULL, (empty)
                    // 555: b"frozen\0" as  ... st u8: typeof(_643) = *const {l1059} u8
                    // 555: b"frozen\0" as  ... st u8:   l1059 = UNIQUE | NON_NULL, (empty)
                    // 555: b"frozen\0": typeof(_644) = *const {l1061} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 555: b"frozen\0":   l1061 = UNIQUE | NON_NULL, (empty)
                    // 555: b"frozen\0": typeof(_645) = & {l1063} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 555: b"frozen\0":   l1063 = UNIQUE | NON_NULL, FIXED
                    // 555: b"frozen\0" as  ... _char: typeof(_642 = move _643 as *const i8 (Misc)) = *const {l1629} i8
                    // 555: b"frozen\0" as  ... _char:   l1629 = UNIQUE | NON_NULL, (empty)
                    // 555: b"frozen\0": typeof(_645 = const b"frozen\x00") = & {l1626} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 555: b"frozen\0":   l1626 = UNIQUE | NON_NULL, (empty)
                    // 555: b"frozen\0": typeof(_644 = &raw const (*_645)) = *const {l1627} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 555: b"frozen\0":   l1627 = UNIQUE | NON_NULL, (empty)
                    // 555: b"frozen\0" as  ... _char: typeof(_641 = move _642 as *mut i8 (Misc)) = *mut {l1630} i8
                    // 555: b"frozen\0" as  ... _char:   l1630 = UNIQUE | NON_NULL, (empty)
                    // 555: b"frozen\0" as  ... st u8: typeof(_643 = move _644 as *const u8 (Pointer(ArrayToPointer))) = *const {l1628} u8
                    // 555: b"frozen\0" as  ... st u8:   l1628 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"usable\0" as *const u8 as *const libc::c_char) == 0 {
            // 557: tok: typeof(_648) = *const {l1067} i8
            // 557: tok:   l1067 = UNIQUE | NON_NULL, (empty)
            // 557: tok: typeof(_649) = *mut {l1069} i8
            // 557: tok:   l1069 = UNIQUE | NON_NULL, (empty)
            // 557: b"usable\0" as  ... _char: typeof(_650) = *const {l1071} i8
            // 557: b"usable\0" as  ... _char:   l1071 = UNIQUE | NON_NULL, (empty)
            // 557: b"usable\0" as  ... st u8: typeof(_651) = *const {l1073} u8
            // 557: b"usable\0" as  ... st u8:   l1073 = UNIQUE | NON_NULL, (empty)
            // 557: b"usable\0": typeof(_652) = *const {l1075} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 557: b"usable\0":   l1075 = UNIQUE | NON_NULL, (empty)
            // 557: b"usable\0": typeof(_653) = & {l1077} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 557: b"usable\0":   l1077 = UNIQUE | NON_NULL, FIXED
            // 557: b"usable\0": typeof(_653 = const b"usable\x00") = & {l1632} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 557: b"usable\0":   l1632 = UNIQUE | NON_NULL, (empty)
            // 557: tok: typeof(_648 = move _649 as *const i8 (Pointer(MutToConstPointer))) = *const {l1631} i8
            // 557: tok:   l1631 = UNIQUE | NON_NULL, (empty)
            // 557: b"usable\0": typeof(_652 = &raw const (*_653)) = *const {l1633} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 557: b"usable\0":   l1633 = UNIQUE | NON_NULL, (empty)
            // 557: b"usable\0" as  ... st u8: typeof(_651 = move _652 as *const u8 (Pointer(ArrayToPointer))) = *const {l1634} u8
            // 557: b"usable\0" as  ... st u8:   l1634 = UNIQUE | NON_NULL, (empty)
            // 557: b"usable\0" as  ... _char: typeof(_650 = move _651 as *const i8 (Misc)) = *const {l1635} i8
            // 557: b"usable\0" as  ... _char:   l1635 = UNIQUE | NON_NULL, (empty)
                res = lglusable(
                    lgl,
                    // 559: lgl: typeof(_655) = *mut {l1080} LGL
                    // 559: lgl:   l1080 = UNIQUE | NON_NULL, (empty)
                    intarg(b"usable\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 560: b"usable\0" as  ... _char: typeof(_657) = *mut {l1083} i8
                    // 560: b"usable\0" as  ... _char:   l1083 = UNIQUE | NON_NULL, (empty)
                    // 560: b"usable\0" as  ... _char: typeof(_658) = *const {l1085} i8
                    // 560: b"usable\0" as  ... _char:   l1085 = UNIQUE | NON_NULL, (empty)
                    // 560: b"usable\0" as  ... st u8: typeof(_659) = *const {l1087} u8
                    // 560: b"usable\0" as  ... st u8:   l1087 = UNIQUE | NON_NULL, (empty)
                    // 560: b"usable\0": typeof(_660) = *const {l1089} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 560: b"usable\0":   l1089 = UNIQUE | NON_NULL, (empty)
                    // 560: b"usable\0": typeof(_661) = & {l1091} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 560: b"usable\0":   l1091 = UNIQUE | NON_NULL, FIXED
                    // 560: b"usable\0" as  ... st u8: typeof(_659 = move _660 as *const u8 (Pointer(ArrayToPointer))) = *const {l1638} u8
                    // 560: b"usable\0" as  ... st u8:   l1638 = UNIQUE | NON_NULL, (empty)
                    // 560: b"usable\0": typeof(_661 = const b"usable\x00") = & {l1636} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 560: b"usable\0":   l1636 = UNIQUE | NON_NULL, (empty)
                    // 560: b"usable\0" as  ... _char: typeof(_658 = move _659 as *const i8 (Misc)) = *const {l1639} i8
                    // 560: b"usable\0" as  ... _char:   l1639 = UNIQUE | NON_NULL, (empty)
                    // 560: b"usable\0": typeof(_660 = &raw const (*_661)) = *const {l1637} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 560: b"usable\0":   l1637 = UNIQUE | NON_NULL, (empty)
                    // 560: b"usable\0" as  ... _char: typeof(_657 = move _658 as *mut i8 (Misc)) = *mut {l1640} i8
                    // 560: b"usable\0" as  ... _char:   l1640 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"reusable\0" as *const u8 as *const libc::c_char) == 0 {
            // 562: tok: typeof(_664) = *const {l1095} i8
            // 562: tok:   l1095 = UNIQUE | NON_NULL, (empty)
            // 562: tok: typeof(_665) = *mut {l1097} i8
            // 562: tok:   l1097 = UNIQUE | NON_NULL, (empty)
            // 562: b"reusable\0" a ... _char: typeof(_666) = *const {l1099} i8
            // 562: b"reusable\0" a ... _char:   l1099 = UNIQUE | NON_NULL, (empty)
            // 562: b"reusable\0" a ... st u8: typeof(_667) = *const {l1101} u8
            // 562: b"reusable\0" a ... st u8:   l1101 = UNIQUE | NON_NULL, (empty)
            // 562: b"reusable\0": typeof(_668) = *const {l1103} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 562: b"reusable\0":   l1103 = UNIQUE | NON_NULL, (empty)
            // 562: b"reusable\0": typeof(_669) = & {l1105} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 562: b"reusable\0":   l1105 = UNIQUE | NON_NULL, FIXED
            // 562: b"reusable\0" a ... st u8: typeof(_667 = move _668 as *const u8 (Pointer(ArrayToPointer))) = *const {l1644} u8
            // 562: b"reusable\0" a ... st u8:   l1644 = UNIQUE | NON_NULL, (empty)
            // 562: tok: typeof(_664 = move _665 as *const i8 (Pointer(MutToConstPointer))) = *const {l1641} i8
            // 562: tok:   l1641 = UNIQUE | NON_NULL, (empty)
            // 562: b"reusable\0": typeof(_669 = const b"reusable\x00") = & {l1642} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 562: b"reusable\0":   l1642 = UNIQUE | NON_NULL, (empty)
            // 562: b"reusable\0" a ... _char: typeof(_666 = move _667 as *const i8 (Misc)) = *const {l1645} i8
            // 562: b"reusable\0" a ... _char:   l1645 = UNIQUE | NON_NULL, (empty)
            // 562: b"reusable\0": typeof(_668 = &raw const (*_669)) = *const {l1643} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 562: b"reusable\0":   l1643 = UNIQUE | NON_NULL, (empty)
                res = lglreusable(
                    lgl,
                    // 564: lgl: typeof(_671) = *mut {l1108} LGL
                    // 564: lgl:   l1108 = UNIQUE | NON_NULL, (empty)
                    intarg(b"reusable\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 565: b"reusable\0" a ... _char: typeof(_673) = *mut {l1111} i8
                    // 565: b"reusable\0" a ... _char:   l1111 = UNIQUE | NON_NULL, (empty)
                    // 565: b"reusable\0" a ... _char: typeof(_674) = *const {l1113} i8
                    // 565: b"reusable\0" a ... _char:   l1113 = UNIQUE | NON_NULL, (empty)
                    // 565: b"reusable\0" a ... st u8: typeof(_675) = *const {l1115} u8
                    // 565: b"reusable\0" a ... st u8:   l1115 = UNIQUE | NON_NULL, (empty)
                    // 565: b"reusable\0": typeof(_676) = *const {l1117} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 565: b"reusable\0":   l1117 = UNIQUE | NON_NULL, (empty)
                    // 565: b"reusable\0": typeof(_677) = & {l1119} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 565: b"reusable\0":   l1119 = UNIQUE | NON_NULL, FIXED
                    // 565: b"reusable\0": typeof(_676 = &raw const (*_677)) = *const {l1647} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 565: b"reusable\0":   l1647 = UNIQUE | NON_NULL, (empty)
                    // 565: b"reusable\0" a ... _char: typeof(_673 = move _674 as *mut i8 (Misc)) = *mut {l1650} i8
                    // 565: b"reusable\0" a ... _char:   l1650 = UNIQUE | NON_NULL, (empty)
                    // 565: b"reusable\0": typeof(_677 = const b"reusable\x00") = & {l1646} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 565: b"reusable\0":   l1646 = UNIQUE | NON_NULL, (empty)
                    // 565: b"reusable\0" a ... _char: typeof(_674 = move _675 as *const i8 (Misc)) = *const {l1649} i8
                    // 565: b"reusable\0" a ... _char:   l1649 = UNIQUE | NON_NULL, (empty)
                    // 565: b"reusable\0" a ... st u8: typeof(_675 = move _676 as *const u8 (Pointer(ArrayToPointer))) = *const {l1648} u8
                    // 565: b"reusable\0" a ... st u8:   l1648 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"setimportant\0" as *const u8 as *const libc::c_char) == 0 {
            // 567: tok: typeof(_680) = *const {l1123} i8
            // 567: tok:   l1123 = UNIQUE | NON_NULL, (empty)
            // 567: tok: typeof(_681) = *mut {l1125} i8
            // 567: tok:   l1125 = UNIQUE | NON_NULL, (empty)
            // 567: b"setimportant\ ... _char: typeof(_682) = *const {l1127} i8
            // 567: b"setimportant\ ... _char:   l1127 = UNIQUE | NON_NULL, (empty)
            // 567: b"setimportant\ ... st u8: typeof(_683) = *const {l1129} u8
            // 567: b"setimportant\ ... st u8:   l1129 = UNIQUE | NON_NULL, (empty)
            // 567: b"setimportant\0": typeof(_684) = *const {l1131} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 567: b"setimportant\0":   l1131 = UNIQUE | NON_NULL, (empty)
            // 567: b"setimportant\0": typeof(_685) = & {l1133} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 567: b"setimportant\0":   l1133 = UNIQUE | NON_NULL, FIXED
            // 567: tok: typeof(_680 = move _681 as *const i8 (Pointer(MutToConstPointer))) = *const {l1651} i8
            // 567: tok:   l1651 = UNIQUE | NON_NULL, (empty)
            // 567: b"setimportant\ ... _char: typeof(_682 = move _683 as *const i8 (Misc)) = *const {l1655} i8
            // 567: b"setimportant\ ... _char:   l1655 = UNIQUE | NON_NULL, (empty)
            // 567: b"setimportant\0": typeof(_685 = const b"setimportant\x00") = & {l1652} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 567: b"setimportant\0":   l1652 = UNIQUE | NON_NULL, (empty)
            // 567: b"setimportant\0": typeof(_684 = &raw const (*_685)) = *const {l1653} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 567: b"setimportant\0":   l1653 = UNIQUE | NON_NULL, (empty)
            // 567: b"setimportant\ ... st u8: typeof(_683 = move _684 as *const u8 (Pointer(ArrayToPointer))) = *const {l1654} u8
            // 567: b"setimportant\ ... st u8:   l1654 = UNIQUE | NON_NULL, (empty)
                lglsetimportant(
                    lgl,
                    // 569: lgl: typeof(_687) = *mut {l1136} LGL
                    // 569: lgl:   l1136 = UNIQUE | NON_NULL, (empty)
                    intarg(
                        b"setimportant\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        // 571: b"setimportant\ ... _char: typeof(_689) = *mut {l1139} i8
                        // 571: b"setimportant\ ... _char:   l1139 = UNIQUE | NON_NULL, (empty)
                        // 571: b"setimportant\ ... _char: typeof(_690) = *const {l1141} i8
                        // 571: b"setimportant\ ... _char:   l1141 = UNIQUE | NON_NULL, (empty)
                        // 571: b"setimportant\ ... st u8: typeof(_691) = *const {l1143} u8
                        // 571: b"setimportant\ ... st u8:   l1143 = UNIQUE | NON_NULL, (empty)
                        // 571: b"setimportant\0": typeof(_692) = *const {l1145} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 571: b"setimportant\0":   l1145 = UNIQUE | NON_NULL, (empty)
                        // 571: b"setimportant\0": typeof(_693) = & {l1147} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 571: b"setimportant\0":   l1147 = UNIQUE | NON_NULL, FIXED
                        // 571: b"setimportant\0": typeof(_693 = const b"setimportant\x00") = & {l1656} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 571: b"setimportant\0":   l1656 = UNIQUE | NON_NULL, (empty)
                        // 571: b"setimportant\ ... _char: typeof(_690 = move _691 as *const i8 (Misc)) = *const {l1659} i8
                        // 571: b"setimportant\ ... _char:   l1659 = UNIQUE | NON_NULL, (empty)
                        // 571: b"setimportant\0": typeof(_692 = &raw const (*_693)) = *const {l1657} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 571: b"setimportant\0":   l1657 = UNIQUE | NON_NULL, (empty)
                        // 571: b"setimportant\ ... _char: typeof(_689 = move _690 as *mut i8 (Misc)) = *mut {l1660} i8
                        // 571: b"setimportant\ ... _char:   l1660 = UNIQUE | NON_NULL, (empty)
                        // 571: b"setimportant\ ... st u8: typeof(_691 = move _692 as *const u8 (Pointer(ArrayToPointer))) = *const {l1658} u8
                        // 571: b"setimportant\ ... st u8:   l1658 = UNIQUE | NON_NULL, (empty)
                    ),
                );
            } else if noarg(
                tok,
                // 575: tok: typeof(_696) = *const {l1151} i8
                // 575: tok:   l1151 = UNIQUE | NON_NULL, (empty)
                // 575: tok: typeof(_697) = *mut {l1153} i8
                // 575: tok:   l1153 = UNIQUE | NON_NULL, (empty)
                // 575: tok: typeof(_696 = move _697 as *const i8 (Pointer(MutToConstPointer))) = *const {l1661} i8
                // 575: tok:   l1661 = UNIQUE | NON_NULL, (empty)
                b"setphases\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                // 576: b"setphases\0"  ... _char: typeof(_698) = *mut {l1155} i8
                // 576: b"setphases\0"  ... _char:   l1155 = UNIQUE | NON_NULL, (empty)
                // 576: b"setphases\0"  ... _char: typeof(_699) = *const {l1157} i8
                // 576: b"setphases\0"  ... _char:   l1157 = UNIQUE | NON_NULL, (empty)
                // 576: b"setphases\0"  ... st u8: typeof(_700) = *const {l1159} u8
                // 576: b"setphases\0"  ... st u8:   l1159 = UNIQUE | NON_NULL, (empty)
                // 576: b"setphases\0": typeof(_701) = *const {l1161} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 576: b"setphases\0":   l1161 = UNIQUE | NON_NULL, (empty)
                // 576: b"setphases\0": typeof(_702) = & {l1163} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 576: b"setphases\0":   l1163 = UNIQUE | NON_NULL, FIXED
                // 576: b"setphases\0": typeof(_701 = &raw const (*_702)) = *const {l1663} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 576: b"setphases\0":   l1663 = UNIQUE | NON_NULL, (empty)
                // 576: b"setphases\0"  ... _char: typeof(_698 = move _699 as *mut i8 (Misc)) = *mut {l1666} i8
                // 576: b"setphases\0"  ... _char:   l1666 = UNIQUE | NON_NULL, (empty)
                // 576: b"setphases\0"  ... st u8: typeof(_700 = move _701 as *const u8 (Pointer(ArrayToPointer))) = *const {l1664} u8
                // 576: b"setphases\0"  ... st u8:   l1664 = UNIQUE | NON_NULL, (empty)
                // 576: b"setphases\0": typeof(_702 = const b"setphases\x00") = & {l1662} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 576: b"setphases\0":   l1662 = UNIQUE | NON_NULL, (empty)
                // 576: b"setphases\0"  ... _char: typeof(_699 = move _700 as *const i8 (Misc)) = *const {l1665} i8
                // 576: b"setphases\0"  ... _char:   l1665 = UNIQUE | NON_NULL, (empty)
            ) != 0
            {
                lglsetphases(lgl);
                // 579: lgl: typeof(_704) = *mut {l1166} LGL
                // 579: lgl:   l1166 = UNIQUE | NON_NULL, (empty)
            } else if strcmp(tok, b"setphase\0" as *const u8 as *const libc::c_char) == 0 {
            // 580: tok: typeof(_707) = *const {l1170} i8
            // 580: tok:   l1170 = UNIQUE | NON_NULL, (empty)
            // 580: tok: typeof(_708) = *mut {l1172} i8
            // 580: tok:   l1172 = UNIQUE | NON_NULL, (empty)
            // 580: b"setphase\0" a ... _char: typeof(_709) = *const {l1174} i8
            // 580: b"setphase\0" a ... _char:   l1174 = UNIQUE | NON_NULL, (empty)
            // 580: b"setphase\0" a ... st u8: typeof(_710) = *const {l1176} u8
            // 580: b"setphase\0" a ... st u8:   l1176 = UNIQUE | NON_NULL, (empty)
            // 580: b"setphase\0": typeof(_711) = *const {l1178} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 580: b"setphase\0":   l1178 = UNIQUE | NON_NULL, (empty)
            // 580: b"setphase\0": typeof(_712) = & {l1180} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 580: b"setphase\0":   l1180 = UNIQUE | NON_NULL, FIXED
            // 580: b"setphase\0" a ... _char: typeof(_709 = move _710 as *const i8 (Misc)) = *const {l1671} i8
            // 580: b"setphase\0" a ... _char:   l1671 = UNIQUE | NON_NULL, (empty)
            // 580: b"setphase\0" a ... st u8: typeof(_710 = move _711 as *const u8 (Pointer(ArrayToPointer))) = *const {l1670} u8
            // 580: b"setphase\0" a ... st u8:   l1670 = UNIQUE | NON_NULL, (empty)
            // 580: b"setphase\0": typeof(_712 = const b"setphase\x00") = & {l1668} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 580: b"setphase\0":   l1668 = UNIQUE | NON_NULL, (empty)
            // 580: tok: typeof(_707 = move _708 as *const i8 (Pointer(MutToConstPointer))) = *const {l1667} i8
            // 580: tok:   l1667 = UNIQUE | NON_NULL, (empty)
            // 580: b"setphase\0": typeof(_711 = &raw const (*_712)) = *const {l1669} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 580: b"setphase\0":   l1669 = UNIQUE | NON_NULL, (empty)
                lglsetphase(
                    lgl,
                    // 582: lgl: typeof(_714) = *mut {l1183} LGL
                    // 582: lgl:   l1183 = UNIQUE | NON_NULL, (empty)
                    intarg(b"setphase\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 583: b"setphase\0" a ... _char: typeof(_716) = *mut {l1186} i8
                    // 583: b"setphase\0" a ... _char:   l1186 = UNIQUE | NON_NULL, (empty)
                    // 583: b"setphase\0" a ... _char: typeof(_717) = *const {l1188} i8
                    // 583: b"setphase\0" a ... _char:   l1188 = UNIQUE | NON_NULL, (empty)
                    // 583: b"setphase\0" a ... st u8: typeof(_718) = *const {l1190} u8
                    // 583: b"setphase\0" a ... st u8:   l1190 = UNIQUE | NON_NULL, (empty)
                    // 583: b"setphase\0": typeof(_719) = *const {l1192} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 583: b"setphase\0":   l1192 = UNIQUE | NON_NULL, (empty)
                    // 583: b"setphase\0": typeof(_720) = & {l1194} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 583: b"setphase\0":   l1194 = UNIQUE | NON_NULL, FIXED
                    // 583: b"setphase\0" a ... _char: typeof(_717 = move _718 as *const i8 (Misc)) = *const {l1675} i8
                    // 583: b"setphase\0" a ... _char:   l1675 = UNIQUE | NON_NULL, (empty)
                    // 583: b"setphase\0": typeof(_719 = &raw const (*_720)) = *const {l1673} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 583: b"setphase\0":   l1673 = UNIQUE | NON_NULL, (empty)
                    // 583: b"setphase\0": typeof(_720 = const b"setphase\x00") = & {l1672} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 583: b"setphase\0":   l1672 = UNIQUE | NON_NULL, (empty)
                    // 583: b"setphase\0" a ... st u8: typeof(_718 = move _719 as *const u8 (Pointer(ArrayToPointer))) = *const {l1674} u8
                    // 583: b"setphase\0" a ... st u8:   l1674 = UNIQUE | NON_NULL, (empty)
                    // 583: b"setphase\0" a ... _char: typeof(_716 = move _717 as *mut i8 (Misc)) = *mut {l1676} i8
                    // 583: b"setphase\0" a ... _char:   l1676 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"resetphase\0" as *const u8 as *const libc::c_char) == 0 {
            // 585: tok: typeof(_723) = *const {l1198} i8
            // 585: tok:   l1198 = UNIQUE | NON_NULL, (empty)
            // 585: tok: typeof(_724) = *mut {l1200} i8
            // 585: tok:   l1200 = UNIQUE | NON_NULL, (empty)
            // 585: b"resetphase\0" ... _char: typeof(_725) = *const {l1202} i8
            // 585: b"resetphase\0" ... _char:   l1202 = UNIQUE | NON_NULL, (empty)
            // 585: b"resetphase\0" ... st u8: typeof(_726) = *const {l1204} u8
            // 585: b"resetphase\0" ... st u8:   l1204 = UNIQUE | NON_NULL, (empty)
            // 585: b"resetphase\0": typeof(_727) = *const {l1206} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 585: b"resetphase\0":   l1206 = UNIQUE | NON_NULL, (empty)
            // 585: b"resetphase\0": typeof(_728) = & {l1208} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 585: b"resetphase\0":   l1208 = UNIQUE | NON_NULL, FIXED
            // 585: b"resetphase\0": typeof(_727 = &raw const (*_728)) = *const {l1679} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 585: b"resetphase\0":   l1679 = UNIQUE | NON_NULL, (empty)
            // 585: b"resetphase\0" ... _char: typeof(_725 = move _726 as *const i8 (Misc)) = *const {l1681} i8
            // 585: b"resetphase\0" ... _char:   l1681 = UNIQUE | NON_NULL, (empty)
            // 585: b"resetphase\0": typeof(_728 = const b"resetphase\x00") = & {l1678} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 585: b"resetphase\0":   l1678 = UNIQUE | NON_NULL, (empty)
            // 585: b"resetphase\0" ... st u8: typeof(_726 = move _727 as *const u8 (Pointer(ArrayToPointer))) = *const {l1680} u8
            // 585: b"resetphase\0" ... st u8:   l1680 = UNIQUE | NON_NULL, (empty)
            // 585: tok: typeof(_723 = move _724 as *const i8 (Pointer(MutToConstPointer))) = *const {l1677} i8
            // 585: tok:   l1677 = UNIQUE | NON_NULL, (empty)
                lglresetphase(
                    lgl,
                    // 587: lgl: typeof(_730) = *mut {l1211} LGL
                    // 587: lgl:   l1211 = UNIQUE | NON_NULL, (empty)
                    intarg(
                        b"resetphase\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        // 589: b"resetphase\0" ... _char: typeof(_732) = *mut {l1214} i8
                        // 589: b"resetphase\0" ... _char:   l1214 = UNIQUE | NON_NULL, (empty)
                        // 589: b"resetphase\0" ... _char: typeof(_733) = *const {l1216} i8
                        // 589: b"resetphase\0" ... _char:   l1216 = UNIQUE | NON_NULL, (empty)
                        // 589: b"resetphase\0" ... st u8: typeof(_734) = *const {l1218} u8
                        // 589: b"resetphase\0" ... st u8:   l1218 = UNIQUE | NON_NULL, (empty)
                        // 589: b"resetphase\0": typeof(_735) = *const {l1220} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                        // 589: b"resetphase\0":   l1220 = UNIQUE | NON_NULL, (empty)
                        // 589: b"resetphase\0": typeof(_736) = & {l1222} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                        // 589: b"resetphase\0":   l1222 = UNIQUE | NON_NULL, FIXED
                        // 589: b"resetphase\0" ... _char: typeof(_733 = move _734 as *const i8 (Misc)) = *const {l1685} i8
                        // 589: b"resetphase\0" ... _char:   l1685 = UNIQUE | NON_NULL, (empty)
                        // 589: b"resetphase\0": typeof(_736 = const b"resetphase\x00") = & {l1682} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                        // 589: b"resetphase\0":   l1682 = UNIQUE | NON_NULL, (empty)
                        // 589: b"resetphase\0" ... _char: typeof(_732 = move _733 as *mut i8 (Misc)) = *mut {l1686} i8
                        // 589: b"resetphase\0" ... _char:   l1686 = UNIQUE | NON_NULL, (empty)
                        // 589: b"resetphase\0": typeof(_735 = &raw const (*_736)) = *const {l1683} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                        // 589: b"resetphase\0":   l1683 = UNIQUE | NON_NULL, (empty)
                        // 589: b"resetphase\0" ... st u8: typeof(_734 = move _735 as *const u8 (Pointer(ArrayToPointer))) = *const {l1684} u8
                        // 589: b"resetphase\0" ... st u8:   l1684 = UNIQUE | NON_NULL, (empty)
                    ),
                );
            } else if strcmp(tok, b"option\0" as *const u8 as *const libc::c_char) == 0 {
            // 592: tok: typeof(_739) = *const {l1226} i8
            // 592: tok:   l1226 = UNIQUE | NON_NULL, (empty)
            // 592: tok: typeof(_740) = *mut {l1228} i8
            // 592: tok:   l1228 = UNIQUE | NON_NULL, (empty)
            // 592: b"option\0" as  ... _char: typeof(_741) = *const {l1230} i8
            // 592: b"option\0" as  ... _char:   l1230 = UNIQUE | NON_NULL, (empty)
            // 592: b"option\0" as  ... st u8: typeof(_742) = *const {l1232} u8
            // 592: b"option\0" as  ... st u8:   l1232 = UNIQUE | NON_NULL, (empty)
            // 592: b"option\0": typeof(_743) = *const {l1234} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 592: b"option\0":   l1234 = UNIQUE | NON_NULL, (empty)
            // 592: b"option\0": typeof(_744) = & {l1236} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 592: b"option\0":   l1236 = UNIQUE | NON_NULL, FIXED
            // 592: b"option\0" as  ... st u8: typeof(_742 = move _743 as *const u8 (Pointer(ArrayToPointer))) = *const {l1690} u8
            // 592: b"option\0" as  ... st u8:   l1690 = UNIQUE | NON_NULL, (empty)
            // 592: b"option\0": typeof(_743 = &raw const (*_744)) = *const {l1689} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 592: b"option\0":   l1689 = UNIQUE | NON_NULL, (empty)
            // 592: tok: typeof(_739 = move _740 as *const i8 (Pointer(MutToConstPointer))) = *const {l1687} i8
            // 592: tok:   l1687 = UNIQUE | NON_NULL, (empty)
            // 592: b"option\0": typeof(_744 = const b"option\x00") = & {l1688} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 592: b"option\0":   l1688 = UNIQUE | NON_NULL, (empty)
            // 592: b"option\0" as  ... _char: typeof(_741 = move _742 as *const i8 (Misc)) = *const {l1691} i8
            // 592: b"option\0" as  ... _char:   l1691 = UNIQUE | NON_NULL, (empty)
                opt = strtok(
                // 593: strtok( 0 as *m ... ar, ): typeof(_745) = *mut {l1238} i8
                // 593: strtok( 0 as *m ... ar, ):   l1238 = UNIQUE | NON_NULL, (empty)
                    0 as *mut libc::c_char,
                    // 594: 0 as *mut libc: ... _char: typeof(_746) = *mut {l1240} i8
                    // 594: 0 as *mut libc: ... _char:   l1240 = UNIQUE | NON_NULL, (empty)
                    // 594: 0 as *mut libc: ... _char: typeof(_746 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l1692} i8
                    // 594: 0 as *mut libc: ... _char:   l1692 = UNIQUE | NON_NULL, (empty)
                    b" \0" as *const u8 as *const libc::c_char,
                    // 595: b" \0" as *cons ... _char: typeof(_747) = *const {l1242} i8
                    // 595: b" \0" as *cons ... _char:   l1242 = UNIQUE | NON_NULL, (empty)
                    // 595: b" \0" as *const u8: typeof(_748) = *const {l1244} u8
                    // 595: b" \0" as *const u8:   l1244 = UNIQUE | NON_NULL, (empty)
                    // 595: b" \0": typeof(_749) = *const {l1246} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                    // 595: b" \0":   l1246 = UNIQUE | NON_NULL, (empty)
                    // 595: b" \0": typeof(_750) = & {l1248} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                    // 595: b" \0":   l1248 = UNIQUE | NON_NULL, FIXED
                    // 595: b" \0": typeof(_749 = &raw const (*_750)) = *const {l1694} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                    // 595: b" \0":   l1694 = UNIQUE | NON_NULL, (empty)
                    // 595: b" \0": typeof(_750 = const b" \x00") = & {l1693} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                    // 595: b" \0":   l1693 = UNIQUE | NON_NULL, (empty)
                    // 595: b" \0" as *const u8: typeof(_748 = move _749 as *const u8 (Pointer(ArrayToPointer))) = *const {l1695} u8
                    // 595: b" \0" as *const u8:   l1695 = UNIQUE | NON_NULL, (empty)
                    // 595: b" \0" as *cons ... _char: typeof(_747 = move _748 as *const i8 (Misc)) = *const {l1696} i8
                    // 595: b" \0" as *cons ... _char:   l1696 = UNIQUE | NON_NULL, (empty)
                );
                if opt.is_null() {
                // 597: opt: typeof(_753) = *mut {l1252} i8
                // 597: opt:   l1252 = UNIQUE | NON_NULL, (empty)
                    perr(b"option name missing\0" as *const u8 as *const libc::c_char);
                    // 598: b"option name m ... _char: typeof(_755) = *const {l1255} i8
                    // 598: b"option name m ... _char:   l1255 = UNIQUE | NON_NULL, (empty)
                    // 598: b"option name m ... st u8: typeof(_756) = *const {l1257} u8
                    // 598: b"option name m ... st u8:   l1257 = UNIQUE | NON_NULL, (empty)
                    // 598: b"option name m ... ng\0": typeof(_757) = *const {l1259} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                    // 598: b"option name m ... ng\0":   l1259 = UNIQUE | NON_NULL, (empty)
                    // 598: b"option name m ... ng\0": typeof(_758) = & {l1261} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                    // 598: b"option name m ... ng\0":   l1261 = UNIQUE | NON_NULL, FIXED
                    // 598: b"option name m ... ng\0": typeof(_757 = &raw const (*_758)) = *const {l1698} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                    // 598: b"option name m ... ng\0":   l1698 = UNIQUE | NON_NULL, (empty)
                    // 598: b"option name m ... st u8: typeof(_756 = move _757 as *const u8 (Pointer(ArrayToPointer))) = *const {l1699} u8
                    // 598: b"option name m ... st u8:   l1699 = UNIQUE | NON_NULL, (empty)
                    // 598: b"option name m ... _char: typeof(_755 = move _756 as *const i8 (Misc)) = *const {l1700} i8
                    // 598: b"option name m ... _char:   l1700 = UNIQUE | NON_NULL, (empty)
                    // 598: b"option name m ... ng\0": typeof(_758 = const b"option name missing\x00") = & {l1697} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                    // 598: b"option name m ... ng\0":   l1697 = UNIQUE | NON_NULL, (empty)
                }
                lglsetopt(
                    lgl,
                    // 601: lgl: typeof(_760) = *mut {l1264} LGL
                    // 601: lgl:   l1264 = UNIQUE | NON_NULL, (empty)
                    opt,
                    // 602: opt: typeof(_761) = *const {l1266} i8
                    // 602: opt:   l1266 = UNIQUE | NON_NULL, (empty)
                    // 602: opt: typeof(_762) = *mut {l1268} i8
                    // 602: opt:   l1268 = UNIQUE | NON_NULL, (empty)
                    // 602: opt: typeof(_761 = move _762 as *const i8 (Pointer(MutToConstPointer))) = *const {l1701} i8
                    // 602: opt:   l1701 = UNIQUE | NON_NULL, (empty)
                    intarg(b"option\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 603: b"option\0" as  ... _char: typeof(_764) = *mut {l1271} i8
                    // 603: b"option\0" as  ... _char:   l1271 = UNIQUE | NON_NULL, (empty)
                    // 603: b"option\0" as  ... _char: typeof(_765) = *const {l1273} i8
                    // 603: b"option\0" as  ... _char:   l1273 = UNIQUE | NON_NULL, (empty)
                    // 603: b"option\0" as  ... st u8: typeof(_766) = *const {l1275} u8
                    // 603: b"option\0" as  ... st u8:   l1275 = UNIQUE | NON_NULL, (empty)
                    // 603: b"option\0": typeof(_767) = *const {l1277} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 603: b"option\0":   l1277 = UNIQUE | NON_NULL, (empty)
                    // 603: b"option\0": typeof(_768) = & {l1279} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 603: b"option\0":   l1279 = UNIQUE | NON_NULL, FIXED
                    // 603: b"option\0" as  ... st u8: typeof(_766 = move _767 as *const u8 (Pointer(ArrayToPointer))) = *const {l1704} u8
                    // 603: b"option\0" as  ... st u8:   l1704 = UNIQUE | NON_NULL, (empty)
                    // 603: b"option\0": typeof(_768 = const b"option\x00") = & {l1702} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 603: b"option\0":   l1702 = UNIQUE | NON_NULL, (empty)
                    // 603: b"option\0" as  ... _char: typeof(_765 = move _766 as *const i8 (Misc)) = *const {l1705} i8
                    // 603: b"option\0" as  ... _char:   l1705 = UNIQUE | NON_NULL, (empty)
                    // 603: b"option\0": typeof(_767 = &raw const (*_768)) = *const {l1703} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 603: b"option\0":   l1703 = UNIQUE | NON_NULL, (empty)
                    // 603: b"option\0" as  ... _char: typeof(_764 = move _765 as *mut i8 (Misc)) = *mut {l1706} i8
                    // 603: b"option\0" as  ... _char:   l1706 = UNIQUE | NON_NULL, (empty)
                );
            } else if noarg(
                tok,
                // 606: tok: typeof(_771) = *const {l1283} i8
                // 606: tok:   l1283 = UNIQUE | NON_NULL, (empty)
                // 606: tok: typeof(_772) = *mut {l1285} i8
                // 606: tok:   l1285 = UNIQUE | NON_NULL, (empty)
                // 606: tok: typeof(_771 = move _772 as *const i8 (Pointer(MutToConstPointer))) = *const {l1707} i8
                // 606: tok:   l1707 = UNIQUE | NON_NULL, (empty)
                b"release\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                // 607: b"release\0" as ... _char: typeof(_773) = *mut {l1287} i8
                // 607: b"release\0" as ... _char:   l1287 = UNIQUE | NON_NULL, (empty)
                // 607: b"release\0" as ... _char: typeof(_774) = *const {l1289} i8
                // 607: b"release\0" as ... _char:   l1289 = UNIQUE | NON_NULL, (empty)
                // 607: b"release\0" as ... st u8: typeof(_775) = *const {l1291} u8
                // 607: b"release\0" as ... st u8:   l1291 = UNIQUE | NON_NULL, (empty)
                // 607: b"release\0": typeof(_776) = *const {l1293} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 607: b"release\0":   l1293 = UNIQUE | NON_NULL, (empty)
                // 607: b"release\0": typeof(_777) = & {l1295} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 607: b"release\0":   l1295 = UNIQUE | NON_NULL, FIXED
                // 607: b"release\0" as ... _char: typeof(_774 = move _775 as *const i8 (Misc)) = *const {l1711} i8
                // 607: b"release\0" as ... _char:   l1711 = UNIQUE | NON_NULL, (empty)
                // 607: b"release\0": typeof(_776 = &raw const (*_777)) = *const {l1709} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 607: b"release\0":   l1709 = UNIQUE | NON_NULL, (empty)
                // 607: b"release\0" as ... _char: typeof(_773 = move _774 as *mut i8 (Misc)) = *mut {l1712} i8
                // 607: b"release\0" as ... _char:   l1712 = UNIQUE | NON_NULL, (empty)
                // 607: b"release\0" as ... st u8: typeof(_775 = move _776 as *const u8 (Pointer(ArrayToPointer))) = *const {l1710} u8
                // 607: b"release\0" as ... st u8:   l1710 = UNIQUE | NON_NULL, (empty)
                // 607: b"release\0": typeof(_777 = const b"release\x00") = & {l1708} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 607: b"release\0":   l1708 = UNIQUE | NON_NULL, (empty)
            ) != 0
            {
                lglrelease(lgl);
                // 610: lgl: typeof(_779) = *mut {l1298} LGL
                // 610: lgl:   l1298 = UNIQUE | NON_NULL, (empty)
            } else {
                perr(
                    b"invalid command '%s'\0" as *const u8 as *const libc::c_char,
                    // 613: b"invalid comma ... _char: typeof(_781) = *const {l1301} i8
                    // 613: b"invalid comma ... _char:   l1301 = UNIQUE | NON_NULL, (empty)
                    // 613: b"invalid comma ... st u8: typeof(_782) = *const {l1303} u8
                    // 613: b"invalid comma ... st u8:   l1303 = UNIQUE | NON_NULL, (empty)
                    // 613: b"invalid comma ... s'\0": typeof(_783) = *const {l1305} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 613: b"invalid comma ... s'\0":   l1305 = UNIQUE | NON_NULL, (empty)
                    // 613: b"invalid comma ... s'\0": typeof(_784) = & {l1307} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 613: b"invalid comma ... s'\0":   l1307 = UNIQUE | NON_NULL, FIXED
                    // 613: b"invalid comma ... s'\0": typeof(_784 = const b"invalid command \'%s\'\x00") = & {l1713} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 613: b"invalid comma ... s'\0":   l1713 = UNIQUE | NON_NULL, (empty)
                    // 613: b"invalid comma ... st u8: typeof(_782 = move _783 as *const u8 (Pointer(ArrayToPointer))) = *const {l1715} u8
                    // 613: b"invalid comma ... st u8:   l1715 = UNIQUE | NON_NULL, (empty)
                    // 613: b"invalid comma ... _char: typeof(_781 = move _782 as *const i8 (Misc)) = *const {l1716} i8
                    // 613: b"invalid comma ... _char:   l1716 = UNIQUE | NON_NULL, (empty)
                    // 613: b"invalid comma ... s'\0": typeof(_783 = &raw const (*_784)) = *const {l1714} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 613: b"invalid comma ... s'\0":   l1714 = UNIQUE | NON_NULL, (empty)
                    tok,
                    // 614: tok: typeof(_785) = *mut {l1309} i8
                    // 614: tok:   l1309 = UNIQUE | NON_NULL, (empty)
                );
            }
            lineno += 1;
            // 617: lineno: typeof(_786) = *mut {l1311} i32
            // 617: lineno:   l1311 = UNIQUE | NON_NULL, (empty)
            lineno;
            // 618: lineno: typeof(_789) = *mut {l1315} i32
            // 618: lineno:   l1315 = UNIQUE | NON_NULL, (empty)
            len = 0 as libc::c_int;
        }
    }
    if close == 1 as libc::c_int {
        fclose(file);
        // 623: file: typeof(_796) = *mut {l1323} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
        // 623: file:   l1323 = UNIQUE | NON_NULL, (empty)
    }
    if close == 2 as libc::c_int {
        pclose(file);
        // 626: file: typeof(_802) = *mut {l1330} DefId(0:113 ~ lgluntrace[16a7]::_IO_FILE)
        // 626: file:   l1330 = UNIQUE | NON_NULL, (empty)
    }
    msg(b"done %s\0" as *const u8 as *const libc::c_char, name);
    // 628: b"done %s\0" as ... _char: typeof(_804) = *const {l1333} i8
    // 628: b"done %s\0" as ... _char:   l1333 = UNIQUE | NON_NULL, (empty)
    // 628: b"done %s\0" as ... st u8: typeof(_805) = *const {l1335} u8
    // 628: b"done %s\0" as ... st u8:   l1335 = UNIQUE | NON_NULL, (empty)
    // 628: b"done %s\0": typeof(_806) = *const {l1337} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
    // 628: b"done %s\0":   l1337 = UNIQUE | NON_NULL, (empty)
    // 628: b"done %s\0": typeof(_807) = & {l1339} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
    // 628: b"done %s\0":   l1339 = UNIQUE | NON_NULL, FIXED
    // 628: name: typeof(_808) = *const {l1341} i8
    // 628: name:   l1341 = UNIQUE | NON_NULL, (empty)
    // 628: name: typeof(_809) = *mut {l1343} *const {l1344} i8
    // 628: name:   l1343 = UNIQUE | NON_NULL, (empty)
    // 628: name:   l1344 = UNIQUE | NON_NULL, (empty)
    // 628: b"done %s\0": typeof(_806 = &raw const (*_807)) = *const {l1718} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
    // 628: b"done %s\0":   l1718 = UNIQUE | NON_NULL, (empty)
    // 628: b"done %s\0" as ... _char: typeof(_804 = move _805 as *const i8 (Misc)) = *const {l1720} i8
    // 628: b"done %s\0" as ... _char:   l1720 = UNIQUE | NON_NULL, (empty)
    // 628: b"done %s\0" as ... st u8: typeof(_805 = move _806 as *const u8 (Pointer(ArrayToPointer))) = *const {l1719} u8
    // 628: b"done %s\0" as ... st u8:   l1719 = UNIQUE | NON_NULL, (empty)
    // 628: b"done %s\0": typeof(_807 = const b"done %s\x00") = & {l1717} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
    // 628: b"done %s\0":   l1717 = UNIQUE | NON_NULL, (empty)
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    // 632: mut args: typeof(_1) = DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l1} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
    // 632: mut args:   l1 = UNIQUE | NON_NULL, (empty)
    for arg in ::std::env::args() {
    // 633: ::std::env::args(): typeof(_9) = &mut {l10} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 633: ::std::env::args():   l10 = UNIQUE | NON_NULL, (empty)
    // 633: ::std::env::args(): typeof(_10) = &mut {l12} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 633: ::std::env::args():   l12 = UNIQUE | NON_NULL, (empty)
    // 633: ::std::env::args(): typeof(_9 = &mut (*_10)) = &mut {l52} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 633: ::std::env::args():   l52 = UNIQUE | NON_NULL, (empty)
    // 633: ::std::env::args(): typeof(_10 = &mut _5) = &mut {l51} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 633: ::std::env::args():   l51 = UNIQUE | NON_NULL, (empty)
        args.push(
        // 634: args.push( (::s ... (), ): typeof(_15) = &mut {l18} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l19} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
        // 634: args.push( (::s ... (), ):   l18 = UNIQUE | NON_NULL, (empty)
        // 634: args.push( (::s ... (), ):   l19 = UNIQUE | NON_NULL, (empty)
        // 634: args.push( (::s ... (), ): typeof(_15 = &mut _1) = &mut {l53} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l54} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
        // 634: args.push( (::s ... (), ):   l53 = UNIQUE | NON_NULL, (empty)
        // 634: args.push( (::s ... (), ):   l54 = UNIQUE | NON_NULL, (empty)
            (::std::ffi::CString::new(arg))
            // 635: (::std::ffi::CS ... raw(): typeof(_16) = *mut {l21} i8
            // 635: (::std::ffi::CS ... raw():   l21 = UNIQUE | NON_NULL, (empty)
                .expect("Failed to convert argument into CString.")
                // 636: "Failed to conv ... ing.": typeof(_20) = & {l26} str
                // 636: "Failed to conv ... ing.":   l26 = UNIQUE | NON_NULL, (empty)
                // 636: "Failed to conv ... ing.": typeof(_21) = & {l28} str
                // 636: "Failed to conv ... ing.":   l28 = UNIQUE | NON_NULL, FIXED
                // 636: "Failed to conv ... ing.": typeof(_21 = const "Failed to convert argument into CString.") = & {l55} str
                // 636: "Failed to conv ... ing.":   l55 = UNIQUE | NON_NULL, (empty)
                // 636: "Failed to conv ... ing.": typeof(_20 = &(*_21)) = & {l56} str
                // 636: "Failed to conv ... ing.":   l56 = UNIQUE | NON_NULL, (empty)
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    // 640: args.push(::cor ... ut()): typeof(_23) = &mut {l31} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l32} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
    // 640: args.push(::cor ... ut()):   l31 = UNIQUE | NON_NULL, (empty)
    // 640: args.push(::cor ... ut()):   l32 = UNIQUE | NON_NULL, (empty)
    // 640: ::core::ptr::nu ... mut(): typeof(_24) = *mut {l34} i8
    // 640: ::core::ptr::nu ... mut():   l34 = UNIQUE | NON_NULL, (empty)
    // 640: args.push(::cor ... ut()): typeof(_23 = &mut _1) = &mut {l57} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l58} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
    // 640: args.push(::cor ... ut()):   l57 = UNIQUE | NON_NULL, (empty)
    // 640: args.push(::cor ... ut()):   l58 = UNIQUE | NON_NULL, (empty)
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            // 643: args.len(): typeof(_30) = & {l41} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l42} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 643: args.len():   l41 = UNIQUE | NON_NULL, (empty)
            // 643: args.len():   l42 = UNIQUE | NON_NULL, (empty)
            // 643: args.len(): typeof(_30 = &_1) = & {l59} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l60} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 643: args.len():   l59 = UNIQUE | NON_NULL, (empty)
            // 643: args.len():   l60 = UNIQUE | NON_NULL, (empty)
            args.as_mut_ptr() as *mut *mut libc::c_char,
            // 644: args.as_mut_ptr ... _char: typeof(_32) = *mut {l45} *mut {l46} i8
            // 644: args.as_mut_ptr ... _char:   l45 = UNIQUE | NON_NULL, (empty)
            // 644: args.as_mut_ptr ... _char:   l46 = UNIQUE | NON_NULL, (empty)
            // 644: args.as_mut_ptr(): typeof(_33) = &mut {l48} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l49} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 644: args.as_mut_ptr():   l48 = UNIQUE | NON_NULL, (empty)
            // 644: args.as_mut_ptr():   l49 = UNIQUE | NON_NULL, (empty)
            // 644: args.as_mut_ptr(): typeof(_33 = &mut _1) = &mut {l61} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l62} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 644: args.as_mut_ptr():   l61 = UNIQUE | NON_NULL, (empty)
            // 644: args.as_mut_ptr():   l62 = UNIQUE | NON_NULL, (empty)
        ) as i32)
    }
}
