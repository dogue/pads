use anyhow::Result;
use clap::arg;
use clap::Command;
use pulsectl::controllers::{AppControl, DeviceControl, SinkController};

fn main() -> Result<(), anyhow::Error> {
    let mut handler = SinkController::create()?;

    let options = Command::new("PulseAudio Device Switcher")
        .version("1.0")
        .author("Dogue <dogue@oddhelix.com>")
        .about("Switches the default PulseAudio output device and moves all active applications to that device")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("list")
                .about("Lists available output devices")
        )
        .subcommand(
            Command::new("set")
                .about("Sets the default output device")
                .arg(arg!(<DEVICE>))
        )
        .get_matches();

    match options.subcommand() {
        Some(("set", device)) => {
            let device = device
                .get_one::<String>("DEVICE")
                .unwrap()
                .parse::<u32>()
                .unwrap();

            set_device(&mut handler, device)?;
        }
        Some(("list", _)) => list_devices(&mut handler)?,
        _ => {}
    };

    Ok(())
}

fn set_device(handler: &mut SinkController, index: u32) -> Result<(), anyhow::Error> {
    let device_name = handler.get_device_by_index(index)?;
    handler.set_default_device(&device_name.name.unwrap_or("".to_owned()))?;

    let apps = handler.list_applications()?;

    for app in apps.clone() {
        handler.move_app_by_index(app.index, index)?;
    }

    Ok(())
}

fn list_devices(handler: &mut SinkController) -> Result<(), anyhow::Error> {
    let devices = handler.list_devices()?;
    let default = handler.get_default_device()?;

    println!("Available devices (*default):\n");

    for device in devices.clone() {
        if device.index == default.index {
            print!("*");
        }

        println!(
            "[{}]: {}",
            device.index,
            device.description.unwrap_or("Unknown device".to_owned())
        );
    }

    Ok(())
}