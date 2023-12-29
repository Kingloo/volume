#![allow(unused_imports)]

use windows::core::Result;
use windows::Win32::Media::Audio::{IMMDeviceEnumerator, MMDeviceEnumerator, IMMDevice, eRender, eConsole};
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, COINIT_MULTITHREADED, CLSCTX_INPROC_SERVER};

fn main() -> Result<()> {
	let args: Vec<String> = std::env::args().collect();

	unsafe
	{
		CoInitializeEx(None, COINIT_MULTITHREADED)?;

		let audio_endpoint_volume: IAudioEndpointVolume = create_audio_endpoint_volume()?;

		if args.len() < 2 {
			print_volume(get_volume(&audio_endpoint_volume)?);
			return Ok(());
		}

		if let Ok(desired_volume_scalar) = args[1].parse::<f32>() {
			if desired_volume_scalar > 1.0 {
				println!("value too large {:?}", desired_volume_scalar);
				return Ok(());
			}
			set_volume(desired_volume_scalar, &audio_endpoint_volume)?;
		} else {
			println!("not a float: {:?}", args[1]);
			return Ok(());
		}
	}

	Ok(())
}

unsafe fn create_audio_endpoint_volume() -> Result<IAudioEndpointVolume> {
	let sav: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER)?;
	let default_device: IMMDevice = sav.GetDefaultAudioEndpoint(eRender, eConsole)?;
	let audio_endpoint_volume: IAudioEndpointVolume = default_device.Activate(CLSCTX_INPROC_SERVER, None)?;
	Ok(audio_endpoint_volume)
}

unsafe fn get_volume(audio_endpoint_volume: &IAudioEndpointVolume) -> Result<f32> {
	let current_volume: f32 = audio_endpoint_volume.GetMasterVolumeLevelScalar()?;
	Ok(current_volume)
}

unsafe fn set_volume(desired_volume_scalar: f32, audio_endpoint_volume: &IAudioEndpointVolume) -> Result<()> {
	audio_endpoint_volume.SetMasterVolumeLevelScalar(desired_volume_scalar, std::ptr::null())?;
	println!("set master volume to {:.0}%", convert_float_to_percent(desired_volume_scalar));
	Ok(())
}

fn print_volume(volume: f32) {
	println!("current master volume {:.0}%", convert_float_to_percent(volume));
}

fn convert_float_to_percent(volume: f32) -> f32 {
	volume * 100f32
}
