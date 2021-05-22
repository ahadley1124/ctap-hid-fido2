use crate::get_assertion_params;
use crate::public_key_credential_user_entity::PublicKeyCredentialUserEntity;
use crate::util;
use byteorder::{BigEndian, ReadBytesExt};
use serde_cbor::Value;
use std::io::Cursor;
use crate::auth_data::Flags;
use crate::get_assertion_params::Extension;

fn parse_cbor_authdata(authdata: Vec<u8>, ass: &mut get_assertion_params::Assertion) -> Result<(),String>{
    // copy
    ass.auth_data = authdata.to_vec();

    let mut index = 0;

    let clo_vec = |idx: usize, x: usize| (authdata[idx..idx + x].to_vec(), idx + x);

    // rpIdHash	(32)
    let ret = clo_vec(index, 32);
    ass.rpid_hash = ret.0;
    index = ret.1;

    // flags(1)
    let byte = authdata[index];
    ass.flags = Flags::parse(byte).unwrap();
    index += 1;

    // signCount(4)
    let clo = |idx: usize, x: usize| {
        let mut rdr = Cursor::new(authdata[idx..idx + x].to_vec());
        (rdr.read_u32::<BigEndian>().unwrap(), idx + x)
    };
    let ret = clo(index, 4);
    ass.sign_count = ret.0;
    index = ret.1;

    // rest is cbor objects
    // - [0] credentialPublicKey
    // - [1] extensions
    let slice = if ass.flags.attested_credential_data_included {
        let slice = &authdata[index..authdata.len()];
        let deserializer = serde_cbor::Deserializer::from_slice(&slice);
        //
        slice[deserializer.byte_offset()..].to_vec()
    } else {
        authdata[index..authdata.len()].to_vec()
    };

    if ass.flags.extension_data_included {
        //println!("{:02} - {:?}", slice.len(), util::to_hex_str(&slice));
        let maps = util::cbor_bytes_to_map(&slice)?;
        for (key, val) in &maps {
            if let Value::Text(member) = key {
                if *member == Extension::HmacSecret(None).to_string() {
                    let v = util::cbor_value_to_vec_u8(val)?;
                    let mut v2:[u8;32] = Default::default();
                    for index in 0..32{
                        v2[index] = v[index];
                    }
                    ass.extensions.push(Extension::HmacSecret(Some(v2)));
                }
            }
        }
    };
    Ok(())
}

pub fn parse_cbor(bytes: &[u8]) -> Result<get_assertion_params::Assertion, String> {
    let mut ass = get_assertion_params::Assertion::default();
    let maps = util::cbor_bytes_to_map(bytes)?;
    for (key, val) in &maps {
        if let Value::Integer(member) = key {
            match member {
                0x01 => ass.credential_id = util::cbor_get_bytes_from_map(val, "id")?,
                0x02 => {
                    if let Value::Bytes(xs) = val {
                        parse_cbor_authdata(xs.to_vec(), &mut ass)?;
                    }
                }
                0x03 => ass.signature = util::cbor_value_to_vec_u8(val)?,
                0x04 => {
                    ass.user = PublicKeyCredentialUserEntity::default()
                        .get_id(val)
                        .get_name(val)
                        .get_display_name(val)
                }
                0x05 => ass.number_of_credentials = util::cbor_value_to_num(val)?,
                _ => println!("- anything error"),
            }
        }
    }
    Ok(ass)
}
