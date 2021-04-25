use crate::bio_enrollment_command;
use crate::bio_enrollment_params::{BioEnrollmentData,TemplateInfo};
use crate::bio_enrollment_response;
use crate::client_pin;
use crate::ctaphid;
use crate::FidoKeyHid;
use crate::HidParam;
use crate::pintoken::PinToken;

#[allow(unused_imports)]
use crate::util;

pub(crate) fn bio_enrollment(
    device: &FidoKeyHid,
    cid: &[u8;4],
    pin_token: Option<&PinToken>,
    sub_command: Option<bio_enrollment_command::SubCommand>,
    template_info: Option<TemplateInfo>,
    timeout_milliseconds: Option<u16>,
) -> Result<BioEnrollmentData, String> {
    let send_payload = bio_enrollment_command::create_payload(pin_token, sub_command, template_info, timeout_milliseconds);

    if util::is_debug() {
        println!("send(cbor) = {}", util::to_hex_str(&send_payload));
    }

    let response_cbor = ctaphid::ctaphid_cbor(device, cid, &send_payload)?;
    if util::is_debug() {
        println!("response(cbor) = {}", util::to_hex_str(&response_cbor));
    }

    let ret = bio_enrollment_response::parse_cbor(&response_cbor)?;

    Ok(ret)
}

pub fn bio_enrollment_init(
    hid_params: &[HidParam],
    pin: Option<&str>,
) -> Result<(FidoKeyHid,[u8;4],Option<PinToken>),String>{
    // init
    let device = FidoKeyHid::new(hid_params)?;
    let cid = ctaphid::ctaphid_init(&device)?;

    // pin token
    let pin_token = {
        if let Some(pin) = pin {
            Some(client_pin::get_pin_token(&device, &cid, pin)?)
        } else {
            None
        }
    };
    Ok((device,cid,pin_token))
}
