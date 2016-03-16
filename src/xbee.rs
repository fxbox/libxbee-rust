#![feature(plugin)]
// Make linter fail for every warning
#![plugin(clippy)]
#![deny(clippy)]

#![allow(non_camel_case_types)]
#![allow(enum_variant_names)]

#![feature(custom_derive)]
#![allow(unused_attributes)]

extern crate libc;
extern crate va_list;

use libc::time_t;
use libc::timespec;
use std::os::raw::*;
use std::mem::*;
use std::option::Option;
use std::default::Default;
use va_list::VaList;
use libc::FILE;
use std::fmt::*;

pub enum Struct_xbee { }
pub enum Struct_xbee_con { }
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_xbee_conSleepStates {
    CON_AWAKE = 0,
    CON_SNOOZE = 1,
    CON_SLEEP = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Struct_xbee_conAddress {
    pub broadcast: c_uchar,
    pub addr16_enabled: c_uchar,
    pub addr16: [c_uchar; 2usize],
    pub addr64_enabled: c_uchar,
    pub addr64: [c_uchar; 8usize],
    pub endpoints_enabled: c_uchar,
    pub endpoint_local: c_uchar,
    pub endpoint_remote: c_uchar,
    pub profile_enabled: c_uchar,
    pub profile_id: c_ushort,
    pub cluster_enabled: c_uchar,
    pub cluster_id: c_ushort,
}

/*
impl Default for Struct_xbee_conAddress {
    fn default() -> Self { unsafe { zeroed() } }
}*/

impl Default for Struct_xbee_conAddress {
    fn default() -> Self {
        Struct_xbee_conAddress {broadcast : 0,
                                addr16_enabled : 0,
                                addr16 : [0; 2],
                                addr64_enabled : 0,
                                addr64 : [0; 8],
                                endpoints_enabled : 0,
                                endpoint_local : 0,
                                endpoint_remote : 0,
                                profile_enabled : 0,
                                profile_id : 0,
                                cluster_enabled : 0,
                                cluster_id : 0}
    }
}


#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct Struct_xbee_conInfo {
    pub countRx: c_int,
    pub countTx: c_int,
    pub lastRxTime: time_t,
}

impl Default for Struct_xbee_conInfo {
    fn default() -> Self { unsafe { zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct Struct_xbee_conSettings {
    pub _bindgen_bitfield_1_: c_uchar,
    pub _bindgen_bitfield_2_: c_uchar,
    pub broadcastRadius: c_uchar,
}

impl Default for Struct_xbee_conSettings {
    fn default() -> Self { unsafe { zeroed() } }
}
pub enum Struct_xbee_ll_head { }
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct Struct_xbee_pkt {
    pub xbee: *mut Struct_xbee,
    pub con: *mut Struct_xbee_con,
    pub conType: *const c_char,
    pub status: c_uchar,
    pub options: c_uchar,
    pub rssi: c_uchar,
    pub apiIdentifier: c_uchar,
    pub frameId: c_uchar,
    pub timestamp: timespec,
    pub address: Struct_xbee_conAddress,
    pub atCommand: [c_uchar; 2usize],
    pub dataItems: *mut Struct_xbee_ll_head,
    pub dataLen: c_int,
    pub data: [c_uchar; 1usize],
}

impl Default for Struct_xbee_pkt {
    fn default() -> Self { unsafe { zeroed() } }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(i32)]
pub enum Enum_xbee_errors {
    XBEE_ENONE = 0,
    XBEE_EUNKNOWN = -1,
    XBEE_ENOMEM = -2,
    XBEE_ESELECT = -3,
    XBEE_ESELECTINTERRUPTED = -4,
    XBEE_EEOF = -5,
    XBEE_EIO = -6,
    XBEE_ESEMAPHORE = -7,
    XBEE_EMUTEX = -8,
    XBEE_ETHREAD = -9,
    XBEE_ELINKEDLIST = -10,
    XBEE_ESETUP = -11,
    XBEE_EMISSINGPARAM = -12,
    XBEE_EINVAL = -13,
    XBEE_ERANGE = -14,
    XBEE_ELENGTH = -15,
    XBEE_EFAILED = -18,
    XBEE_ETIMEOUT = -17,
    XBEE_EWOULDBLOCK = -16,
    XBEE_EINUSE = -19,
    XBEE_EEXISTS = -20,
    XBEE_ENOTEXISTS = -21,
    XBEE_ENOFREEFRAMEID = -22,
    XBEE_ESTALE = -23,
    XBEE_ENOTIMPLEMENTED = -24,
    XBEE_ETX = -25,
    XBEE_EREMOTE = -26,
    XBEE_ESLEEPING = -27,
    XBEE_ECATCHALL = -28,
    XBEE_ESHUTDOWN = -29,
}

impl Display for Enum_xbee_errors {
    fn fmt(&self, f:&mut Formatter) -> Result{
        write!(f, "{}", *self as i32)
    }
}

pub type xbee_err = Enum_xbee_errors;
pub type xbee_t_eofCallback =
    Option<unsafe extern "C" fn(xbee: *mut Struct_xbee,
                                               rxInfo:
                                                   *mut c_void)>;
pub type xbee_t_conCallback =
    Option<unsafe extern "C" fn(xbee: *mut Struct_xbee,
                                               con: *mut Struct_xbee_con,
                                               pkt: *mut *mut Struct_xbee_pkt,
                                               data:
                                                   *mut *mut c_void)>;
#[link(name = "xbee")]
extern "C" {
    pub static mut libxbee_revision: *const c_char;
    pub static mut libxbee_commit: *const c_char;
    pub static mut libxbee_committer: *const c_char;
    pub static mut libxbee_buildtime: *const c_char;
}
#[link(name = "xbee")]
extern "C" {
    pub fn xbee_freeMemory(ptr: *mut c_void);
    pub fn xbee_validate(xbee: *mut Struct_xbee) -> xbee_err;
    pub fn xbee_setup(retXbee: *mut *mut Struct_xbee,
                      mode: *const c_char, ...) -> xbee_err;
    pub fn xbee_vsetup(retXbee: *mut *mut Struct_xbee,
                       mode: *const c_char, ap: VaList) -> xbee_err;
    pub fn xbee_attachEOFCallback(xbee: *mut Struct_xbee,
                                  eofCallback: xbee_t_eofCallback) -> xbee_err;
    pub fn xbee_shutdown(xbee: *mut Struct_xbee) -> xbee_err;
    pub fn xbee_dataSet(xbee: *mut Struct_xbee,
                        newData: *mut c_void,
                        oldData: *mut *mut c_void) -> xbee_err;
    pub fn xbee_dataGet(xbee: *mut Struct_xbee,
                        curData: *mut *mut c_void) -> xbee_err;
    pub fn xbee_modeGetList(retList: *mut *mut *mut c_char) -> xbee_err;
    pub fn xbee_modeGet(xbee: *mut Struct_xbee,
                        mode: *mut *const c_char) -> xbee_err;
    pub fn xbee_conGetTypes(xbee: *mut Struct_xbee,
                            retList: *mut *mut *mut c_char) -> xbee_err;
    pub fn xbee_conNew(xbee: *mut Struct_xbee,
                       retCon: *mut *mut Struct_xbee_con,
                       _type: *const ::std::os::raw::c_char,
                       address: *mut Struct_xbee_conAddress) -> xbee_err;
    pub fn xbee_conValidate(con: *mut Struct_xbee_con) -> xbee_err;
    pub fn xbee_conGetXBee(con: *mut Struct_xbee_con,
                           xbee: *mut *mut Struct_xbee) -> xbee_err;
    pub fn xbee_conTx(con: *mut Struct_xbee_con,
                      retVal: *mut c_uchar,
                      format: *const c_char, ...) -> xbee_err;
    pub fn xbee_convTx(con: *mut Struct_xbee_con,
                       retVal: *mut c_uchar,
                       format: *const c_char, args: VaList) -> xbee_err;
    pub fn xbee_connTx(con: *mut Struct_xbee_con,
                       retVal: *mut c_uchar,
                       buf: *const c_uchar,
                       len: c_int) -> xbee_err;
    pub fn xbee_conxTx(con: *mut Struct_xbee_con,
                       retVal: *mut c_uchar,
                       frameId: *mut c_uchar,
                       format: *const c_char, ...) -> xbee_err;
    pub fn xbee_convxTx(con: *mut Struct_xbee_con,
                        retVal: *mut c_uchar,
                        frameId: *mut c_uchar,
                        format: *const c_char, args: VaList) -> xbee_err;
    pub fn xbee_connxTx(con: *mut Struct_xbee_con,
                        retVal: *mut c_uchar,
                        frameId: *mut c_uchar,
                        buf: *const c_uchar,
                        len: c_int) -> xbee_err;
    pub fn xbee_conRx(con: *mut Struct_xbee_con,
                      retPkt: *mut *mut Struct_xbee_pkt,
                      remainingPackets: *mut c_int) -> xbee_err;
    pub fn xbee_conRxWait(con: *mut Struct_xbee_con,
                          retPkt: *mut *mut Struct_xbee_pkt,
                          remainingPackets: *mut c_int) -> xbee_err;
    pub fn xbee_conPurge(con: *mut Struct_xbee_con) -> xbee_err;
    pub fn xbee_conSleepSet(con: *mut Struct_xbee_con,
                            state: Enum_xbee_conSleepStates) -> xbee_err;
    pub fn xbee_conSleepGet(con: *mut Struct_xbee_con,
                            state: *mut Enum_xbee_conSleepStates) -> xbee_err;
    pub fn xbee_conDataSet(con: *mut Struct_xbee_con,
                           newData: *mut c_void,
                           oldData: *mut *mut c_void) -> xbee_err;
    pub fn xbee_conDataGet(con: *mut Struct_xbee_con,
                           curData: *mut *mut c_void) -> xbee_err;
    pub fn xbee_conTypeGet(con: *mut Struct_xbee_con,
                           _type: *mut *mut c_char) -> xbee_err;
    pub fn xbee_conInfoGet(con: *mut Struct_xbee_con,
                           info: *mut Struct_xbee_conInfo) -> xbee_err;
    pub fn xbee_conCallbackSet(con: *mut Struct_xbee_con,
                               newCallback: xbee_t_conCallback,
                               oldCallback: *mut xbee_t_conCallback) -> xbee_err;
    pub fn xbee_conCallbackGet(con: *mut Struct_xbee_con,
                               curCallback: *mut xbee_t_conCallback) -> xbee_err;
    pub fn xbee_conSettings(con: *mut Struct_xbee_con,
                            newSettings: *mut Struct_xbee_conSettings,
                            oldSettings: *mut Struct_xbee_conSettings) -> xbee_err;
    pub fn xbee_conEnd(con: *mut Struct_xbee_con) -> xbee_err;
    pub fn xbee_pktFree(pkt: *mut Struct_xbee_pkt) -> xbee_err;
    pub fn xbee_pktValidate(pkt: *mut Struct_xbee_pkt) -> xbee_err;
    pub fn xbee_pktDataGet(pkt: *mut Struct_xbee_pkt,
                           key: *const c_char,
                           id: c_int,
                           index: c_int,
                           retData: *mut *mut c_void) -> xbee_err;
    pub fn xbee_pktAnalogGet(pkt: *mut Struct_xbee_pkt,
                             channel: c_int,
                             index: c_int,
                             retVal: *mut c_int) -> xbee_err;
    pub fn xbee_pktDigitalGet(pkt: *mut Struct_xbee_pkt,
                              channel: c_int,
                              index: c_int,
                              retVal: *mut c_int) -> xbee_err;
    pub fn xbee_netStart(xbee: *mut Struct_xbee, port: c_int,
                         clientFilter:
                             Option<unsafe extern "C" fn(xbee: *mut Struct_xbee,
                                                         remoteHost: *const c_char)
                                                         -> c_int>) -> xbee_err;
    pub fn xbee_netvStart(xbee: *mut Struct_xbee, fd: c_int,
                          clientFilter:
                              Option<unsafe extern "C" fn(xbee: *mut Struct_xbee,
                                                          remoteHost: *const c_char)
                                                         -> c_int>) -> xbee_err;
    pub fn xbee_netStop(xbee: *mut Struct_xbee) -> xbee_err;
    pub fn xbee_logTargetSet(xbee: *mut Struct_xbee, f: *mut FILE) -> xbee_err;
    pub fn xbee_logTargetGet(xbee: *mut Struct_xbee, f: *mut *mut FILE) -> xbee_err;
    pub fn xbee_logLevelSet(xbee: *mut Struct_xbee,
                            level: c_int) -> xbee_err;
    pub fn xbee_logLevelGet(xbee: *mut Struct_xbee,
                            level: *mut c_int) -> xbee_err;
    pub fn xbee_logRxSet(xbee: *mut Struct_xbee,
                         enable: c_int) -> xbee_err;
    pub fn xbee_logRxGet(xbee: *mut Struct_xbee,
                         enabled: *mut c_int) -> xbee_err;
    pub fn xbee_logTxSet(xbee: *mut Struct_xbee,
                         enable: c_int) -> xbee_err;
    pub fn xbee_logTxGet(xbee: *mut Struct_xbee,
                         enabled: *mut c_int) -> xbee_err;
    pub fn xbee_logColorSet(xbee: *mut Struct_xbee,
                            enable: c_int) -> xbee_err;
    pub fn xbee_logColorGet(xbee: *mut Struct_xbee,
                            enabled: *mut c_int) -> xbee_err;
    pub fn _xbee_logDev(file: *const c_char,
                        line: c_int,
                        function: *const c_char,
                        xbee: *mut Struct_xbee,
                        minLevel: c_int,
                        format: *const c_char, ...) -> xbee_err;
    pub fn xbee_errorToStr(error: xbee_err) -> *const c_char;
}
