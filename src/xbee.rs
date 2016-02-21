#![feature(plugin)]
// Make linter fail for every warning
#![plugin(clippy)]
#![deny(clippy)]

#![allow(non_camel_case_types)]
#![allow(enum_variant_names)]

extern crate libc;
extern crate va_list;

use libc::time_t;
use libc::timespec;
use va_list::VaList;
//use std::fs::File;
use libc::FILE;

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
#[derive(Copy, Clone)]
pub struct Struct_xbee_conAddress {
    pub broadcast: ::std::os::raw::c_uchar,
    pub addr16_enabled: ::std::os::raw::c_uchar,
    pub addr16: [::std::os::raw::c_uchar; 2usize],
    pub addr64_enabled: ::std::os::raw::c_uchar,
    pub addr64: [::std::os::raw::c_uchar; 8usize],
    pub endpoints_enabled: ::std::os::raw::c_uchar,
    pub endpoint_local: ::std::os::raw::c_uchar,
    pub endpoint_remote: ::std::os::raw::c_uchar,
    pub profile_enabled: ::std::os::raw::c_uchar,
    pub profile_id: ::std::os::raw::c_ushort,
    pub cluster_enabled: ::std::os::raw::c_uchar,
    pub cluster_id: ::std::os::raw::c_ushort,
}
/*impl ::std::clone::Clone for Struct_xbee_conAddress {
    fn clone(&self) -> Self { *self }
}*/
impl ::std::default::Default for Struct_xbee_conAddress {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct Struct_xbee_conInfo {
    pub countRx: ::std::os::raw::c_int,
    pub countTx: ::std::os::raw::c_int,
    pub lastRxTime: time_t,
}
/*impl ::std::clone::Clone for Struct_xbee_conInfo {
    fn clone(&self) -> Self { *self }
}*/
impl ::std::default::Default for Struct_xbee_conInfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct Struct_xbee_conSettings {
    pub _bindgen_bitfield_1_: ::std::os::raw::c_uchar,
    pub _bindgen_bitfield_2_: ::std::os::raw::c_uchar,
    pub broadcastRadius: ::std::os::raw::c_uchar,
}
/*impl ::std::clone::Clone for Struct_xbee_conSettings {
    fn clone(&self) -> Self { *self }
}*/
impl ::std::default::Default for Struct_xbee_conSettings {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub enum Struct_xbee_ll_head { }
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct Struct_xbee_pkt {
    pub xbee: *mut Struct_xbee,
    pub con: *mut Struct_xbee_con,
    pub conType: *const ::std::os::raw::c_char,
    pub status: ::std::os::raw::c_uchar,
    pub options: ::std::os::raw::c_uchar,
    pub rssi: ::std::os::raw::c_uchar,
    pub apiIdentifier: ::std::os::raw::c_uchar,
    pub frameId: ::std::os::raw::c_uchar,
    pub timestamp: timespec,
    pub address: Struct_xbee_conAddress,
    pub atCommand: [::std::os::raw::c_uchar; 2usize],
    pub dataItems: *mut Struct_xbee_ll_head,
    pub dataLen: ::std::os::raw::c_int,
    pub data: [::std::os::raw::c_uchar; 1usize],
}
/*impl ::std::clone::Clone for Struct_xbee_pkt {
    fn clone(&self) -> Self { *self }
}*/
impl ::std::default::Default for Struct_xbee_pkt {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[derive(Clone, Copy)]
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
pub type xbee_err = Enum_xbee_errors;
pub type xbee_t_eofCallback =
    ::std::option::Option<unsafe extern "C" fn(xbee: *mut Struct_xbee,
                                               rxInfo:
                                                   *mut ::std::os::raw::c_void)>;
pub type xbee_t_conCallback =
    ::std::option::Option<unsafe extern "C" fn(xbee: *mut Struct_xbee,
                                               con: *mut Struct_xbee_con,
                                               pkt: *mut *mut Struct_xbee_pkt,
                                               data:
                                                   *mut *mut ::std::os::raw::c_void)>;
#[link(name = "xbee")]
extern "C" {
    pub static mut libxbee_revision: *const ::std::os::raw::c_char;
    pub static mut libxbee_commit: *const ::std::os::raw::c_char;
    pub static mut libxbee_committer: *const ::std::os::raw::c_char;
    pub static mut libxbee_buildtime: *const ::std::os::raw::c_char;
}
#[link(name = "xbee")]
extern "C" {
    pub fn xbee_freeMemory(ptr: *mut ::std::os::raw::c_void);
    pub fn xbee_validate(xbee: *mut Struct_xbee) -> xbee_err;
    pub fn xbee_setup(retXbee: *mut *mut Struct_xbee,
                      mode: *const ::std::os::raw::c_char, ...) -> xbee_err;
    pub fn xbee_vsetup(retXbee: *mut *mut Struct_xbee,
                       mode: *const ::std::os::raw::c_char, ap: VaList)
     -> xbee_err;
    pub fn xbee_attachEOFCallback(xbee: *mut Struct_xbee,
                                  eofCallback: xbee_t_eofCallback)
     -> xbee_err;
    pub fn xbee_shutdown(xbee: *mut Struct_xbee) -> xbee_err;
    pub fn xbee_dataSet(xbee: *mut Struct_xbee,
                        newData: *mut ::std::os::raw::c_void,
                        oldData: *mut *mut ::std::os::raw::c_void)
     -> xbee_err;
    pub fn xbee_dataGet(xbee: *mut Struct_xbee,
                        curData: *mut *mut ::std::os::raw::c_void)
     -> xbee_err;
    pub fn xbee_modeGetList(retList: *mut *mut *mut ::std::os::raw::c_char)
     -> xbee_err;
    pub fn xbee_modeGet(xbee: *mut Struct_xbee,
                        mode: *mut *const ::std::os::raw::c_char) -> xbee_err;
    pub fn xbee_conGetTypes(xbee: *mut Struct_xbee,
                            retList: *mut *mut *mut ::std::os::raw::c_char)
     -> xbee_err;
    pub fn xbee_conNew(xbee: *mut Struct_xbee,
                       retCon: *mut *mut Struct_xbee_con,
                       _type: *const ::std::os::raw::c_char,
                       address: *mut Struct_xbee_conAddress) -> xbee_err;
    pub fn xbee_conValidate(con: *mut Struct_xbee_con) -> xbee_err;
    pub fn xbee_conGetXBee(con: *mut Struct_xbee_con,
                           xbee: *mut *mut Struct_xbee) -> xbee_err;
    pub fn xbee_conTx(con: *mut Struct_xbee_con,
                      retVal: *mut ::std::os::raw::c_uchar,
                      format: *const ::std::os::raw::c_char, ...) -> xbee_err;
    pub fn xbee_convTx(con: *mut Struct_xbee_con,
                       retVal: *mut ::std::os::raw::c_uchar,
                       format: *const ::std::os::raw::c_char, args: VaList)
     -> xbee_err;
    pub fn xbee_connTx(con: *mut Struct_xbee_con,
                       retVal: *mut ::std::os::raw::c_uchar,
                       buf: *const ::std::os::raw::c_uchar,
                       len: ::std::os::raw::c_int) -> xbee_err;
    pub fn xbee_conxTx(con: *mut Struct_xbee_con,
                       retVal: *mut ::std::os::raw::c_uchar,
                       frameId: *mut ::std::os::raw::c_uchar,
                       format: *const ::std::os::raw::c_char, ...)
     -> xbee_err;
    pub fn xbee_convxTx(con: *mut Struct_xbee_con,
                        retVal: *mut ::std::os::raw::c_uchar,
                        frameId: *mut ::std::os::raw::c_uchar,
                        format: *const ::std::os::raw::c_char, args: VaList)
     -> xbee_err;
    pub fn xbee_connxTx(con: *mut Struct_xbee_con,
                        retVal: *mut ::std::os::raw::c_uchar,
                        frameId: *mut ::std::os::raw::c_uchar,
                        buf: *const ::std::os::raw::c_uchar,
                        len: ::std::os::raw::c_int) -> xbee_err;
    pub fn xbee_conRx(con: *mut Struct_xbee_con,
                      retPkt: *mut *mut Struct_xbee_pkt,
                      remainingPackets: *mut ::std::os::raw::c_int)
     -> xbee_err;
    pub fn xbee_conRxWait(con: *mut Struct_xbee_con,
                          retPkt: *mut *mut Struct_xbee_pkt,
                          remainingPackets: *mut ::std::os::raw::c_int)
     -> xbee_err;
    pub fn xbee_conPurge(con: *mut Struct_xbee_con) -> xbee_err;
    pub fn xbee_conSleepSet(con: *mut Struct_xbee_con,
                            state: Enum_xbee_conSleepStates) -> xbee_err;
    pub fn xbee_conSleepGet(con: *mut Struct_xbee_con,
                            state: *mut Enum_xbee_conSleepStates) -> xbee_err;
    pub fn xbee_conDataSet(con: *mut Struct_xbee_con,
                           newData: *mut ::std::os::raw::c_void,
                           oldData: *mut *mut ::std::os::raw::c_void)
     -> xbee_err;
    pub fn xbee_conDataGet(con: *mut Struct_xbee_con,
                           curData: *mut *mut ::std::os::raw::c_void)
     -> xbee_err;
    pub fn xbee_conTypeGet(con: *mut Struct_xbee_con,
                           _type: *mut *mut ::std::os::raw::c_char)
     -> xbee_err;
    pub fn xbee_conInfoGet(con: *mut Struct_xbee_con,
                           info: *mut Struct_xbee_conInfo) -> xbee_err;
    pub fn xbee_conCallbackSet(con: *mut Struct_xbee_con,
                               newCallback: xbee_t_conCallback,
                               oldCallback: *mut xbee_t_conCallback)
     -> xbee_err;
    pub fn xbee_conCallbackGet(con: *mut Struct_xbee_con,
                               curCallback: *mut xbee_t_conCallback)
     -> xbee_err;
    pub fn xbee_conSettings(con: *mut Struct_xbee_con,
                            newSettings: *mut Struct_xbee_conSettings,
                            oldSettings: *mut Struct_xbee_conSettings)
     -> xbee_err;
    pub fn xbee_conEnd(con: *mut Struct_xbee_con) -> xbee_err;
    pub fn xbee_pktFree(pkt: *mut Struct_xbee_pkt) -> xbee_err;
    pub fn xbee_pktValidate(pkt: *mut Struct_xbee_pkt) -> xbee_err;
    pub fn xbee_pktDataGet(pkt: *mut Struct_xbee_pkt,
                           key: *const ::std::os::raw::c_char,
                           id: ::std::os::raw::c_int,
                           index: ::std::os::raw::c_int,
                           retData: *mut *mut ::std::os::raw::c_void)
     -> xbee_err;
    pub fn xbee_pktAnalogGet(pkt: *mut Struct_xbee_pkt,
                             channel: ::std::os::raw::c_int,
                             index: ::std::os::raw::c_int,
                             retVal: *mut ::std::os::raw::c_int) -> xbee_err;
    pub fn xbee_pktDigitalGet(pkt: *mut Struct_xbee_pkt,
                              channel: ::std::os::raw::c_int,
                              index: ::std::os::raw::c_int,
                              retVal: *mut ::std::os::raw::c_int) -> xbee_err;
    pub fn xbee_netStart(xbee: *mut Struct_xbee, port: ::std::os::raw::c_int,
                         clientFilter:
                             ::std::option::Option<unsafe extern "C" fn(xbee:
                                                                            *mut Struct_xbee,
                                                                        remoteHost:
                                                                            *const ::std::os::raw::c_char)
                                                       ->
                                                           ::std::os::raw::c_int>)
     -> xbee_err;
    pub fn xbee_netvStart(xbee: *mut Struct_xbee, fd: ::std::os::raw::c_int,
                          clientFilter:
                              ::std::option::Option<unsafe extern "C" fn(xbee:
                                                                             *mut Struct_xbee,
                                                                         remoteHost:
                                                                             *const ::std::os::raw::c_char)
                                                        ->
                                                            ::std::os::raw::c_int>)
     -> xbee_err;
    pub fn xbee_netStop(xbee: *mut Struct_xbee) -> xbee_err;
    pub fn xbee_logTargetSet(xbee: *mut Struct_xbee, f: *mut FILE)
     -> xbee_err;
    pub fn xbee_logTargetGet(xbee: *mut Struct_xbee, f: *mut *mut FILE)
     -> xbee_err;
    pub fn xbee_logLevelSet(xbee: *mut Struct_xbee,
                            level: ::std::os::raw::c_int) -> xbee_err;
    pub fn xbee_logLevelGet(xbee: *mut Struct_xbee,
                            level: *mut ::std::os::raw::c_int) -> xbee_err;
    pub fn xbee_logRxSet(xbee: *mut Struct_xbee,
                         enable: ::std::os::raw::c_int) -> xbee_err;
    pub fn xbee_logRxGet(xbee: *mut Struct_xbee,
                         enabled: *mut ::std::os::raw::c_int) -> xbee_err;
    pub fn xbee_logTxSet(xbee: *mut Struct_xbee,
                         enable: ::std::os::raw::c_int) -> xbee_err;
    pub fn xbee_logTxGet(xbee: *mut Struct_xbee,
                         enabled: *mut ::std::os::raw::c_int) -> xbee_err;
    pub fn xbee_logColorSet(xbee: *mut Struct_xbee,
                            enable: ::std::os::raw::c_int) -> xbee_err;
    pub fn xbee_logColorGet(xbee: *mut Struct_xbee,
                            enabled: *mut ::std::os::raw::c_int) -> xbee_err;
    pub fn _xbee_logDev(file: *const ::std::os::raw::c_char,
                        line: ::std::os::raw::c_int,
                        function: *const ::std::os::raw::c_char,
                        xbee: *mut Struct_xbee,
                        minLevel: ::std::os::raw::c_int,
                        format: *const ::std::os::raw::c_char, ...)
     -> xbee_err;
    pub fn xbee_errorToStr(error: xbee_err) -> *const ::std::os::raw::c_char;
}
