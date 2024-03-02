use windows::core::Result;
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::{eConsole, eRender, IMMDevice, IMMDeviceEnumerator, MMDeviceEnumerator};
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED};

fn main() -> windows::core::Result<()> {
	let args: Vec<String> = std::env::args().collect();

	unsafe {
		CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;

		let audio_endpoint_volume: IAudioEndpointVolume = create_audio_endpoint_volume()?;

		let current_volume: f32 = get_volume(&audio_endpoint_volume)?;

		if args.len() < 2 {
			println!("current master volume {:.0}%", convert_float_to_percent(current_volume));
			return Ok(());
		}

		let desired_volume_scalar: f32 = match args[1].as_str() {
			"inc" => current_volume + 0.01,
			"dec" => current_volume - 0.01,
			str => str.parse::<f32>().unwrap_or(current_volume)
		};

		if !(0.0..=1.0).contains(&desired_volume_scalar) {
			eprintln!("failed: value must be between 0.0 and 1.0");
			return Ok(());
		}

		set_volume(desired_volume_scalar, &audio_endpoint_volume)?;

		println!("set master volume to {:.0}%", convert_float_to_percent(desired_volume_scalar));
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
	audio_endpoint_volume.GetMasterVolumeLevelScalar()
}

unsafe fn set_volume(desired_volume_scalar: f32, audio_endpoint_volume: &IAudioEndpointVolume) -> Result<()> {
	audio_endpoint_volume.SetMasterVolumeLevelScalar(desired_volume_scalar, std::ptr::null())
}

fn convert_float_to_percent(volume: f32) -> f32 {
	volume * 100f32
}
