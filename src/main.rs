use windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::{
	DEVICE_STATE_ACTIVE, IMMDevice, IMMDeviceCollection, IMMDeviceEnumerator, MMDeviceEnumerator, eCapture, eConsole, eRender,
};
use windows::Win32::System::Com::StructuredStorage::PropVariantToStringAlloc;
use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx, STGM_READ};
use windows::core::Result;

fn usage() -> Result<()> {
	let usage = String::from(
		"volume.exe {out|in} {inc|dec|0.NN}
	\tout = change default output device
	\tin = change default input device
	\tinc = increment by 0.01
	\tdec = decrement by 0.01
	\t0.NN = value from 0.00 to 1.00 as 0% to 100%",
	);
	println!("{}", usage);
	Ok(())
}

fn main() -> windows::core::Result<()> {
	let args: Vec<String> = std::env::args().collect();

	unsafe {
		CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
	}

	let device_enumerator: IMMDeviceEnumerator = unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER)? };

	match args.len() {
		0 => panic!("should be impossible!"),
		1 => print_current_volumes(&device_enumerator),
		3 => adjust_volume(&args, &device_enumerator),
		_other => usage(),
	}
}

fn print_current_volumes(device_enumerator: &IMMDeviceEnumerator) -> Result<()> {
	let default_output_device = get_default_output_device(device_enumerator)?;
	let default_input_device = get_default_input_device(device_enumerator)?;
	print_current_volume(&default_output_device)?;
	print_current_volume(&default_input_device)?;
	Ok(())
}

fn adjust_volume(args: &[String], device_enumerator: &IMMDeviceEnumerator) -> Result<()> {
	let device_to_adjust: IMMDevice = match args[1].as_str() {
		"out" => get_default_output_device(device_enumerator)?,
		"in" => get_default_input_device(device_enumerator)?,
		_other => return usage(),
	};

	let device_friendly_name = get_device_friendly_name(&device_to_adjust)?;

	let audio_endpoint_to_adjust: IAudioEndpointVolume = get_audio_endpoint(&device_to_adjust)?;

	let current_volume_scalar: f32 = get_volume(&audio_endpoint_to_adjust)?;

	let desired_volume_scalar: f32 = match args[2].as_str() {
		"inc" => current_volume_scalar + 0.01,
		"dec" => current_volume_scalar - 0.01,
		other => {
			if let Ok(value) = other.parse::<f32>() {
				value
			} else {
				return usage();
			}
		}
	};

	if !(0.0..=1.0).contains(&desired_volume_scalar) {
		eprintln!("failed: value must be between 0.0 and 1.0");
		return Ok(());
	}

	set_volume(desired_volume_scalar, &audio_endpoint_to_adjust)?;

	println!("{} → {:.0}%", device_friendly_name, convert_float_to_percent(desired_volume_scalar));

	Ok(())
}

fn print_current_volume(device: &IMMDevice) -> Result<()> {
	let friendly_name: String = get_device_friendly_name(device)?;
	let audio_endpoint: IAudioEndpointVolume = get_audio_endpoint(device)?;
	let current_volume: f32 = get_volume(&audio_endpoint)?;
	println!("{}\t{:.0}%", friendly_name, convert_float_to_percent(current_volume));
	Ok(())
}

fn get_default_output_device(device_enumerator: &IMMDeviceEnumerator) -> Result<IMMDevice> {
	let default_device: IMMDevice = unsafe { device_enumerator.GetDefaultAudioEndpoint(eRender, eConsole)? };
	Ok(default_device)
}

fn get_default_input_device(device_enumerator: &IMMDeviceEnumerator) -> Result<IMMDevice> {
	let input_devices: IMMDeviceCollection = unsafe { device_enumerator.EnumAudioEndpoints(eCapture, DEVICE_STATE_ACTIVE)? };
	let default_input_device = unsafe { input_devices.Item(0)? };
	Ok(default_input_device)
}

fn get_audio_endpoint(device: &IMMDevice) -> Result<IAudioEndpointVolume> {
	let audio_endpoint_volume: IAudioEndpointVolume = unsafe { device.Activate(CLSCTX_INPROC_SERVER, None)? };
	Ok(audio_endpoint_volume)
}

fn get_device_friendly_name(device: &IMMDevice) -> Result<String> {
	let prop_store = unsafe { device.OpenPropertyStore(STGM_READ)? };
	let friendly_name_prop = unsafe { prop_store.GetValue(&PKEY_Device_FriendlyName)? };
	let friendly_name = unsafe { PropVariantToStringAlloc(&friendly_name_prop)? };
	Ok(unsafe { friendly_name.to_string()? })
}

fn get_volume(audio_endpoint_volume: &IAudioEndpointVolume) -> Result<f32> {
	unsafe { audio_endpoint_volume.GetMasterVolumeLevelScalar() }
}

fn set_volume(desired_volume_scalar: f32, audio_endpoint_volume: &IAudioEndpointVolume) -> Result<()> {
	unsafe { audio_endpoint_volume.SetMasterVolumeLevelScalar(desired_volume_scalar, std::ptr::null()) }
}

fn convert_float_to_percent(volume: f32) -> f32 {
	volume * 100f32
}
