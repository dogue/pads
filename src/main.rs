use anyhow::Result;
use clap::{Parser, Subcommand};
use pulsectl::controllers::{AppControl, DeviceControl, SinkController};

#[derive(Parser, Debug)]
#[command(author = "Dogue")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(
    about = "Switches the default PulseAudio output device and moves all active applications to that device"
)]
struct Options {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "List available output devices")]
    List,

    #[command(about = "Cycle to the next output device")]
    Next,

    #[command(about = "Set the active device")]
    Set { index: u32 },
}

fn main() -> Result<(), anyhow::Error> {
    let mut handler = SinkController::create()?;

    let options = Options::parse();

    match options.command {
        Commands::Set { index } => set_device(&mut handler, index)?,
        Commands::List => list_devices(&mut handler)?,
        Commands::Next => next_device(&mut handler)?,
    };

    Ok(())
}

fn set_device(handler: &mut SinkController, index: u32) -> Result<(), anyhow::Error> {
    // Set the default output for PulseAudio
    // This does not change which output currently running output streams use
    let device = handler.get_device_by_index(index)?;
    let device_name = device.name.unwrap_or_default();
    handler.set_default_device(&device_name)?;

    // Iterate over all currently available output streams and move them to the new default device
    let apps = handler.list_applications()?;
    for app in apps {
        handler.move_app_by_index(app.index, index)?;
    }

    Ok(())
}

fn list_devices(handler: &mut SinkController) -> Result<(), anyhow::Error> {
    let devices = handler.list_devices()?;
    let default = handler.get_default_device()?;

    println!("Available devices (*default):\n");

    // Prefix an asterisk to mark the current default device
    for device in devices {
        if device.index == default.index {
            print!("*");
        }

        // Print device index and name
        // There should probably be an optional flag to make
        // this output greppable for use with scripts
        println!(
            "[{}]: {}",
            device.index,
            device
                .description
                .unwrap_or_else(|| "Unknown device".to_owned())
        );
    }

    Ok(())
}

fn next_device(handler: &mut SinkController) -> Result<(), anyhow::Error> {
    let devices = handler.list_devices()?;
    let default = handler.get_default_device()?;
    let mut index: u32 = 0;

    // If the default device is *not* the last in the list
    // Set the next device in the list as the default
    // This method is necessary to deal with non-sequential device indexes
    if default.index < devices[devices.len() - 1].index {
        for (i, device) in devices.iter().enumerate() {
            if device.index == default.index {
                index = devices[i + 1].index;
            }
        }
    } else {
        // Wrap around and set the first device in the list as default
        index = devices[0].index;
    }

    set_device(handler, index)?;

    Ok(())
}
