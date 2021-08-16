use std::time::Duration;

// use libusb::request_type;
// use libusb::{Direction, Recipient, RequestType};
fn main() {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        if device_desc.vendor_id() == 0x16c0 {
            println!("Found our device");
            println!(
                "Bus {:03} Device {:03} ID {:04x}:{:04x}",
                device.bus_number(),
                device.address(),
                device_desc.vendor_id(),
                device_desc.product_id()
            );

            let mut dev = device.open().unwrap();

            dev.claim_interface(0).unwrap();
            println!("Opened device");

            let languages = dev.read_languages(Duration::from_secs(1)).unwrap();

            println!("{:?}", languages);

            // let request_type = request_type(Direction::Out, RequestType::Vendor, Recipient::Device);
            // let data = [1, 2, 3];

            //     device.write_control(request_type, 13, 0, 0, &data, Duration::from_secs(1))
            //         .unwrap();
            // let languages = device.read_languages(Duration::from_secs(1)).unwrap();

            // println!("{:?}", languages);
            // let us_english = libusb::language::from_lang_id(0x0409);

            // let mfg_string = device.read_manufacturer_string()

            // let class_code = device_desc.class_code();
        }
    }
}
