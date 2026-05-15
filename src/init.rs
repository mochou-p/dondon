// mochou-p/dondon/src/init.rs

use cpal::SupportedStreamConfig;
use cpal::platform::{Device, Host, HostId, available_hosts, default_host, host_from_id};
use cpal::traits::{DeviceTrait, HostTrait};
use crate::utils;


pub fn setup() -> Option<(Host, Device, SupportedStreamConfig)> {
    let Some(host  ) = get_host  (       ) else { return None; };
    let Some(device) = get_device(&host  ) else { return None; };
    let Some(config) = get_config(&device) else { return None; };

    return Some((host, device, config))
}

fn get_host() -> Option<Host> {
    let Some(host_id) = utils::choose(
        "host",
        available_hosts(),
        Some(default_host().id()),
        |a, b| a == b,
        HostId::name
    ) else { return None; };

    let Ok(host) = host_from_id(host_id) else {
        eprintln!("\x1b[31merror: \x1b[0mselected host is unavailable");
        return None;
    };

    Some(host)
}

fn get_device(host: &Host) -> Option<Device> {
    let Ok(output_devices_filter) = host.output_devices() else { return None; };
    let    output_devices         = output_devices_filter.collect();

    let Some(device) = utils::choose(
        "output device",
        output_devices,
        host.default_output_device(),
        |a, b|   a.id() == b.id(),
        |device| device.id().unwrap().1
    ) else { return None; };

    Some(device)
}

fn get_config(device: &Device) -> Option<SupportedStreamConfig> {
    let Ok(config) = device.default_output_config() else {
        eprintln!("\x1b[31merror: \x1b[0mthere is no default output config");
        return None;
    };

    println!("\x1b[36m- autoselected \x1b[32m{}\x1b[36m as the default output config\x1b[0m\n", utils::config_text(&config));

    Some(config)
}

