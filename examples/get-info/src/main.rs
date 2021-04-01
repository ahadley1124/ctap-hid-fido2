use ctap_hid_fido2;
use ctap_hid_fido2::util;
//use ctap_hid_fido2::credential_management_params;

fn main() {
    ctap_hid_fido2::hello();

    // PEND
    println!("credential_management_enumerate_credentials()");
    let rpid_hash:Vec<u8> = util::to_str_hex("0BDF390F1237B556DB51AF378D5795D5531385CCECDB4499D6BAFBD8918460CA".to_string());
    match ctap_hid_fido2::credential_management_enumerate_credentials(&ctap_hid_fido2::HidParam::get_default_params(),Some("1234"),rpid_hash) {
        Ok(results) => {
            for data in results{
                data.print("- credentials");
            }
        }
        Err(error) => {
            println!("- enumerate credentials error: {:?}", error);
        }
    };

    println!("credential_management_get_creds_metadata()");
    match ctap_hid_fido2::credential_management_get_creds_metadata(&ctap_hid_fido2::HidParam::get_default_params(),Some("1234")) {
        Ok(result) => {
            result.print("- creds metadata");
        }
        Err(error) => {
            println!("- creds metadata error: {:?}", error);
        }
    };
    println!("credential_management_enumerate_rps()");
    match ctap_hid_fido2::credential_management_enumerate_rps(&ctap_hid_fido2::HidParam::get_default_params(),Some("1234")) {
        Ok(results) => {
            for data in results{
                data.print("- rps");
            }
        }
        Err(error) => {
            println!("- enumerate rps error: {:?}", error);
        }
    };
    // PEND

    println!("----- get-info start -----");

    println!("get_hid_devices()");
    let devs = ctap_hid_fido2::get_hid_devices();
    for (info, dev) in devs {
        println!(
            "- vid=0x{:04x} , pid=0x{:04x} , info={:?}",
            dev.vid, dev.pid, info
        );
    }

    println!("get_fidokey_devices()");
    let devs = ctap_hid_fido2::get_fidokey_devices();
    for (info, dev) in devs {
        println!(
            "- vid=0x{:04x} , pid=0x{:04x} , info={:?}",
            dev.vid, dev.pid, info
        );
    }

    println!("get_info()");
    let infos = match ctap_hid_fido2::get_info(&ctap_hid_fido2::HidParam::get_default_params()) {
        Ok(result) => result,
        Err(error) => {
            println!("error: {:?}", error);
            return;
        }
    };
    for (key, value) in infos {
        println!("- {} / {}", key, value);
    }

    println!("get_pin_retries()");
    let retry =
        match ctap_hid_fido2::get_pin_retries(&ctap_hid_fido2::HidParam::get_default_params()) {
            Ok(result) => result,
            Err(error) => {
                println!("error: {:?}", error);
                return;
            }
        };
    println!("- pin retry = {}", retry);

    println!("get_info_u2f()");
    match ctap_hid_fido2::get_info_u2f(&ctap_hid_fido2::HidParam::get_default_params()) {
        Ok(result) => {
            println!("- info u2f : {:?}", result);
        }
        Err(error) => {
            println!("error: {:?}", error);
            return;
        }
    };

    /* Test for CTAP 2.1
    match ctap_hid_fido2::config(&ctap_hid_fido2::HidParam::get_default_params()) {
        Ok(result) => {
            println!("- config : {:?}", result);
        }
        Err(error) => {
            println!("- config error: {:?}", error);
        }
    };

    match ctap_hid_fido2::selection(&ctap_hid_fido2::HidParam::get_default_params()) {
        Ok(result) => {
            println!("- selection : {:?}", result);
        }
        Err(error) => {
            println!("- selection error: {:?}", error);
        }
    };
    */

    println!("----- get-info end -----");
}
