use cpal::traits::{DeviceTrait, HostTrait};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for host_id in cpal::available_hosts() {
        println!("Host: {}", host_id.name());

        let host = cpal::host_from_id(host_id)?;

        println!("  Input devices:");
        match host.input_devices() {
            Ok(devices) => {
                for device in devices {
                    match device.description() {
                        Ok(desc) => println!("    {}", desc),
                        Err(e) => println!("    <error reading device: {}>", e),
                    }
                }
            }
            Err(e) => println!("    <error enumerating: {}>", e),
        }

        println!("  Output devices:");
        match host.output_devices() {
            Ok(devices) => {
                for device in devices {
                    match device.description() {
                        Ok(desc) => println!("    {}", desc),
                        Err(e) => println!("    <error reading device: {}>", e),
                    }
                }
            }
            Err(e) => println!("    <error enumerating: {}>", e),
        }

        println!();
    }

    Ok(())
}
