/* automatically generated by rust-bindgen 0.58.1 */

pub const FELICA_IDM_LENGTH: u16 = 8;
pub const FELICA_PMM_LENGTH: u16 = 8;
pub const FELICA_BLOCK_LENGTH: u16 = 16;
pub const FELICA_AREA_NUM_MAX: u16 = 121;
pub const FELICA_POLLING_ANY: u16 = 65535;
pub const FELICA_POLLING_SUICA: u16 = 3;
pub const FELICA_POLLING_EDY: u16 = 65024;
pub const FELICA_SERVICE_SUICA_IN_OUT: u16 = 4239;
pub const FELICA_SERVICE_SUICA_HISTORY: u16 = 2319;
pub const FELICA_SERVICE_IRUCA_HISTORY: u16 = 56960;
pub const FELICA_SERVICE_EDY_NUMBER: u16 = 4363;
pub const FELICA_SERVICE_EDY_HISTORY: u16 = 5903;
pub const FELICA_CMD_POLLING: u16 = 0;
pub const FELICA_ANS_POLLING: u16 = 1;
pub const FELICA_CMD_REQUEST_SERVICE: u16 = 2;
pub const FELICA_ANS_REQUEST_SERVCCE: u16 = 3;
pub const FELICA_CMD_REQUEST_RESPONSE: u16 = 4;
pub const FELICA_ANS_REQUEST_RESPONSE: u16 = 5;
pub const FELICA_CMD_READ_WITHOUT_ENCRYPTION: u16 = 6;
pub const FELICA_ANS_READ_WITHOUT_ENCRYPTION: u16 = 7;
pub const FELICA_CMD_WRITE_WITHOUT_ENCRYPTION: u16 = 8;
pub const FELICA_ANS_WRITE_WITHOUT_ENCRYPTION: u16 = 9;
pub const FELICA_CMD_AUTHENTICATION1: u16 = 16;
pub const FELICA_ANS_AUTHENTICATION1: u16 = 17;
pub const FELICA_CMD_AUTHENTICATION2: u16 = 18;
pub const FELICA_ANS_AUTHENTICATION2: u16 = 19;
pub const FELICA_CMD_READ_FROM_SECURE_FILE: u16 = 20;
pub const FELICA_ANS_READ_FROM_SECURE_FILE: u16 = 21;
pub const FELICA_CMD_WRITE_TO_SECURE_FILE: u16 = 22;
pub const FELICA_ANS_WRITE_TO_SECURE_FILE: u16 = 23;
pub const FELICA_CMD_SEARCH_SERVICE_CODE: u16 = 10;
pub const FELICA_ANS_SEARCH_SERVICE_CODE: u16 = 11;
pub const FELICA_CMD_REQUEST_SYSTEM: u16 = 12;
pub const FELICA_ANS_REQUEST_SYSTEM: u16 = 13;
pub const PASORI2_CMD_SELF_DIAGNOSIS: u16 = 82;
pub const PASORI2_ANS_SELF_DIAGNOSIS: u16 = 83;
pub const PASORI2_CMD_GET_FIRMWARE_VERSION: u16 = 88;
pub const PASORI2_ANS_GET_FIRMWARE_VERSION: u16 = 89;
pub const PASORI2_CMD_SEND_PACKET: u16 = 92;
pub const PASORI2_ANS_SEND_PACKET: u16 = 93;
pub const PASORI2_DIAG_COMMUNICATION_LINE_TEST: u16 = 0;
pub const PASORI2_DIAG_EEPROM_TEST: u16 = 1;
pub const PASORI2_DIAG_RAM_TEST: u16 = 2;
pub const PASORI2_DIAG_CPU_FUNCTION_TEST: u16 = 3;
pub const PASORI2_DIAG_CPU_FANCTION_TEST: u16 = 3;
pub const PASORI2_DIAG_POLLING_TEST_TO_CARD: u16 = 16;
pub const PASORI_ERR_PARM: u16 = 1;
pub const PASORI_ERR_MEM: u16 = 2;
pub const PASORI_ERR_COM: u16 = 3;
pub const PASORI_ERR_DATA: u16 = 4;
pub const PASORI_ERR_CHKSUM: u16 = 5;
pub const PASORI_ERR_FORMAT: u16 = 6;
pub const PASORI_ERR_TYPE: u16 = 7;
pub type Uint8 = ::std::os::raw::c_uchar;
pub type Uint16 = ::std::os::raw::c_ushort;
pub const PASORI_TYPE_PASORI_TYPE_S310: PasoriType = 0;
pub const PASORI_TYPE_PASORI_TYPE_S320: PasoriType = 1;
pub const PASORI_TYPE_PASORI_TYPE_S330: PasoriType = 2;
pub type PasoriType = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tag_pasori {
    _unused: [u8; 0],
}
pub type Pasori = tag_pasori;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _ferica_area {
    pub code: Uint16,
    pub attr: Uint16,
    pub bin: Uint16,
    pub next: *mut _ferica_area,
}
#[test]
fn bindgen_test_layout_ferica_area() {
    assert_eq!(
        ::std::mem::size_of::<_ferica_area>(),
        16usize,
        concat!("Size of: ", stringify!(_ferica_area))
    );
    assert_eq!(
        ::std::mem::align_of::<_ferica_area>(),
        8usize,
        concat!("Alignment of ", stringify!(_ferica_area))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ferica_area>())).code as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_ferica_area),
            "::",
            stringify!(code)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ferica_area>())).attr as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(_ferica_area),
            "::",
            stringify!(attr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ferica_area>())).bin as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_ferica_area),
            "::",
            stringify!(bin)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_ferica_area>())).next as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_ferica_area),
            "::",
            stringify!(next)
        )
    );
}
pub type FelicaArea = _ferica_area;
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct tag_felica {
    pub p: *mut Pasori,
    pub systemcode: Uint16,
    pub IDm: [Uint8; 8usize],
    pub PMm: [Uint8; 8usize],
    pub area_num: Uint16,
    pub area: [FelicaArea; 256usize],
    pub service_num: Uint16,
    pub service: [FelicaArea; 256usize],
    pub next: *mut tag_felica,
}
#[test]
fn bindgen_test_layout_tag_felica() {
    assert_eq!(
        ::std::mem::size_of::<tag_felica>(),
        8240usize,
        concat!("Size of: ", stringify!(tag_felica))
    );
    assert_eq!(
        ::std::mem::align_of::<tag_felica>(),
        8usize,
        concat!("Alignment of ", stringify!(tag_felica))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tag_felica>())).p as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_felica),
            "::",
            stringify!(p)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tag_felica>())).systemcode as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_felica),
            "::",
            stringify!(systemcode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tag_felica>())).IDm as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_felica),
            "::",
            stringify!(IDm)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tag_felica>())).PMm as *const _ as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_felica),
            "::",
            stringify!(PMm)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tag_felica>())).area_num as *const _ as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_felica),
            "::",
            stringify!(area_num)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tag_felica>())).area as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_felica),
            "::",
            stringify!(area)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tag_felica>())).service_num as *const _ as usize },
        4128usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_felica),
            "::",
            stringify!(service_num)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tag_felica>())).service as *const _ as usize },
        4136usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_felica),
            "::",
            stringify!(service)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tag_felica>())).next as *const _ as usize },
        8232usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_felica),
            "::",
            stringify!(next)
        )
    );
}
pub type Felica = tag_felica;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tag_felica_block {
    pub data: [Uint8; 8usize],
}
#[test]
fn bindgen_test_layout_tag_felica_block() {
    assert_eq!(
        ::std::mem::size_of::<tag_felica_block>(),
        8usize,
        concat!("Size of: ", stringify!(tag_felica_block))
    );
    assert_eq!(
        ::std::mem::align_of::<tag_felica_block>(),
        1usize,
        concat!("Alignment of ", stringify!(tag_felica_block))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tag_felica_block>())).data as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_felica_block),
            "::",
            stringify!(data)
        )
    );
}
pub type FelicaBlock = tag_felica_block;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _felica_block_info {
    pub service: Uint16,
    pub mode: Uint8,
    pub block: Uint16,
}
#[test]
fn bindgen_test_layout_felica_block_info() {
    assert_eq!(
        ::std::mem::size_of::<_felica_block_info>(),
        6usize,
        concat!("Size of: ", stringify!(_felica_block_info))
    );
    assert_eq!(
        ::std::mem::align_of::<_felica_block_info>(),
        2usize,
        concat!("Alignment of ", stringify!(_felica_block_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_felica_block_info>())).service as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_felica_block_info),
            "::",
            stringify!(service)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_felica_block_info>())).mode as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(_felica_block_info),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_felica_block_info>())).block as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_felica_block_info),
            "::",
            stringify!(block)
        )
    );
}
pub type FelicaBlockInfo = _felica_block_info;

#[link(name = "pafe")]
extern "C" {
    pub fn pasori_open() -> *mut Pasori;
    pub fn pasori_init(p: *mut Pasori) -> ::std::os::raw::c_int;
    pub fn pasori_close(p: *mut Pasori);
    pub fn pasori_send(
        p: *mut Pasori,
        data: *mut Uint8,
        size: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pasori_recv(
        p: *mut Pasori,
        data: *mut Uint8,
        size: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pasori_packet_write(
        p: *mut Pasori,
        data: *mut Uint8,
        size: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pasori_packet_read(
        p: *mut Pasori,
        data: *mut Uint8,
        size: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pasori_write(
        p: *mut Pasori,
        data: *mut Uint8,
        size: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pasori_read(
        p: *mut Pasori,
        data: *mut Uint8,
        size: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pasori_reset(p: *mut Pasori) -> ::std::os::raw::c_int;
    pub fn pasori_version(
        p: *mut Pasori,
        v1: *mut ::std::os::raw::c_int,
        v2: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pasori_type(p: *mut Pasori) -> ::std::os::raw::c_int;
    pub fn pasori_test(
        p: *mut Pasori,
        code: ::std::os::raw::c_int,
        data: *mut Uint8,
        size: *mut ::std::os::raw::c_int,
        rdata: *mut Uint8,
        rsize: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pasori_test_echo(
        p: *mut Pasori,
        data: *mut Uint8,
        size: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pasori_test_eprom(p: *mut Pasori) -> ::std::os::raw::c_int;
    pub fn pasori_test_ram(p: *mut Pasori) -> ::std::os::raw::c_int;
    pub fn pasori_test_cpu(p: *mut Pasori) -> ::std::os::raw::c_int;
    pub fn pasori_test_polling(p: *mut Pasori) -> ::std::os::raw::c_int;
    pub fn pasori_list_passive_target(
        pp: *mut Pasori,
        payload: *mut ::std::os::raw::c_uchar,
        size: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn pasori_set_timeout(p: *mut Pasori, timeout: ::std::os::raw::c_int);
    pub fn felica_polling(
        p: *mut Pasori,
        systemcode: Uint16,
        RFU: Uint8,
        timeslot: Uint8,
    ) -> *mut Felica;
    pub fn felica_get_idm(f: *mut Felica, idm: *mut Uint8) -> ::std::os::raw::c_int;
    pub fn felica_get_pmm(f: *mut Felica, idm: *mut Uint8) -> ::std::os::raw::c_int;
    pub fn felica_read(
        f: *mut Felica,
        n: *mut ::std::os::raw::c_int,
        info: *mut FelicaBlockInfo,
        data: *mut Uint8,
    ) -> ::std::os::raw::c_int;
    pub fn felica_read_single(
        f: *mut Felica,
        servicecode: ::std::os::raw::c_int,
        mode: ::std::os::raw::c_int,
        addr: Uint8,
        data: *mut Uint8,
    ) -> ::std::os::raw::c_int;
    pub fn felica_write(
        f: *mut Felica,
        servicecode: ::std::os::raw::c_int,
        mode: ::std::os::raw::c_int,
        addr: Uint8,
        data: *mut Uint8,
    ) -> ::std::os::raw::c_int;
    pub fn felica_request_service(
        f: *mut Felica,
        n: *mut ::std::os::raw::c_int,
        list: *mut Uint16,
        data: *mut Uint16,
    ) -> ::std::os::raw::c_int;
    pub fn felica_request_response(f: *mut Felica, mode: *mut Uint8) -> ::std::os::raw::c_int;
    pub fn felica_search_service(f: *mut Felica) -> ::std::os::raw::c_int;
    pub fn felica_request_system(
        f: *mut Felica,
        n: *mut ::std::os::raw::c_int,
        data: *mut Uint16,
    ) -> ::std::os::raw::c_int;
}
