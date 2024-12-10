use hidapi::{HidApi, HidDevice};

const VENDOR_ID: u16 = 0x0cf2; // LianLi
const PRODUCT_ID: u16 = 0xa102; // LianLi-UNI SL-Infinity

const COMMAND_PREFIX: u8 = 224;
const RGB_SYNC_ENABLED: u8 = 1;
const PWM_MODE_ENABLED: u8 = 0xFF;
const CONFIG_REGISTER: u8 = 16;
const RGB_SYNC_COMMAND: u8 = 97;
const FAN_MODE_COMMAND: u8 = 98;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = HidApi::new()?;
    let device: HidDevice = api.open(VENDOR_ID, PRODUCT_ID)?;

    // Enable RGB sync
    device.write(&[
        COMMAND_PREFIX,
        CONFIG_REGISTER,
        RGB_SYNC_COMMAND,
        RGB_SYNC_ENABLED,
        0,
        0,
        0,
    ])?;
    // Set all fan channels to PWM mode
    device.write(&[
        COMMAND_PREFIX,
        CONFIG_REGISTER,
        FAN_MODE_COMMAND,
        PWM_MODE_ENABLED,
    ])?;

    println!("Lian Li Uni Fan SL-Infinity configured: RGB sync on, all channels in PWM mode");
    Ok(())
}
