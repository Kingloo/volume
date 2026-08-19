use windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::{
	DEVICE_STATE_ACTIVE, IMMDevice, IMMDeviceCollection, IMMDeviceEnumerator, MMDeviceEnumerator, eCapture, eConsole, eRender,
};
use windows::Win32::System::Com::StructuredStorage::PropVariantToStringAlloc;
use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx, STGM_READ};

mod constants;
mod volume;

use crate::constants::{GET_MASTER_VOLUME_SCALAR_FAILED, OUT_OF_RANGE, PARSE_FAILED};
use crate::volume::{Volume, VolumeError};

fn usage() -> windows::core::Result<()> {
	let usage = String::from(
		"volume.exe {out|in} {inc|dec|0.NN}
	\tout = change default output device
	\tin = change default input device
	\tinc = increment by 0.01
	\tdec = decrement by 0.01
	\t0.NN = value from 0.00 to 1.00 as 0% to 100%",
	);
	eprintln!("{}", usage);
	Ok(())
}

fn print_windows_error(error: &windows::core::Error) {
	let code = error.code();
	let message = error.message();
	eprintln!("HRESULT: {code}, message: '{message}'")
}

fn main() -> windows::core::Result<std::process::ExitCode> {
	let args: Vec<String> = std::env::args().collect();

	unsafe {
		CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
	}

	let device_enumerator: IMMDeviceEnumerator = unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER)? };

	if let Err(error) = match args.len() {
		0 => panic!("should be impossible!"),
		1 => print_current_volumes(&device_enumerator),
		3 => adjust_volume(&args, &device_enumerator),
		_other => usage(),
	} {
		print_windows_error(&error);
		Err(error)
	} else {
		Ok(std::process::ExitCode::SUCCESS)
	}
}

fn print_current_volumes(device_enumerator: &IMMDeviceEnumerator) -> windows::core::Result<()> {
	let default_output_device = get_default_output_device(device_enumerator)?;
	let default_input_device = get_default_input_device(device_enumerator)?;
	print_current_volume(&default_output_device)?;
	print_current_volume(&default_input_device)?;
	Ok(())
}

fn adjust_volume(args: &[String], device_enumerator: &IMMDeviceEnumerator) -> windows::core::Result<()> {
	let device_to_adjust: IMMDevice = match args[1].as_str() {
		"out" => get_default_output_device(device_enumerator)?,
		"in" => get_default_input_device(device_enumerator)?,
		_other => return usage(),
	};

	let device_friendly_name = get_device_friendly_name(&device_to_adjust)?;

	let audio_endpoint_to_adjust: IAudioEndpointVolume = get_audio_endpoint(&device_to_adjust)?;

	let current_volume: Volume = get_volume(&audio_endpoint_to_adjust)?;

	let desired_volume: Result<Volume, VolumeError> = match args[2].as_str() {
		"inc" => current_volume.add(0.01),
		"dec" => current_volume.sub(0.01),
		other => {
			if let Ok(value) = other.parse::<f32>() {
				Volume::new(value)
			} else {
				Err(VolumeError::ParseFailed(other.to_string()))
			}
		}
	};

	match desired_volume {
		Ok(volume) => {
			set_volume(volume, &audio_endpoint_to_adjust)?;
			println!("{} → {:.0}%", device_friendly_name, volume.as_percent());
			Ok(())
		}
		Err(e) => match e {
			VolumeError::OutOfRange => Err(windows::core::Error::new(windows::core::HRESULT(OUT_OF_RANGE), "value out of range")),
			VolumeError::ParseFailed(value) => Err(windows::core::Error::new(
				windows::core::HRESULT(PARSE_FAILED),
				format!("failed to parse: '{value}'"),
			)),
		},
	}
}

fn print_current_volume(device: &IMMDevice) -> windows::core::Result<()> {
	let friendly_name: String = get_device_friendly_name(device)?;
	let audio_endpoint: IAudioEndpointVolume = get_audio_endpoint(device)?;
	let current_volume: Volume = get_volume(&audio_endpoint)?;
	println!("{}\t{:.0}%", friendly_name, current_volume.as_percent());
	Ok(())
}

fn get_default_output_device(device_enumerator: &IMMDeviceEnumerator) -> windows::core::Result<IMMDevice> {
	unsafe { device_enumerator.GetDefaultAudioEndpoint(eRender, eConsole) }
}

fn get_default_input_device(device_enumerator: &IMMDeviceEnumerator) -> windows::core::Result<IMMDevice> {
	let input_devices: IMMDeviceCollection = unsafe { device_enumerator.EnumAudioEndpoints(eCapture, DEVICE_STATE_ACTIVE)? };
	let default_input_device = unsafe { input_devices.Item(0)? };
	Ok(default_input_device)
}

fn get_audio_endpoint(device: &IMMDevice) -> windows::core::Result<IAudioEndpointVolume> {
	unsafe { device.Activate(CLSCTX_INPROC_SERVER, None) }
}

fn get_device_friendly_name(device: &IMMDevice) -> windows::core::Result<String> {
	let prop_store = unsafe { device.OpenPropertyStore(STGM_READ)? };
	let friendly_name_prop = unsafe { prop_store.GetValue(&PKEY_Device_FriendlyName)? };
	let friendly_name = unsafe { PropVariantToStringAlloc(&friendly_name_prop)? };
	let name_as_string = unsafe { friendly_name.to_string() };
	match name_as_string {
		Ok(name) => Ok(name),
		Err(e) => windows::core::Result::Err(e.into()),
	}
}

fn get_volume(audio_endpoint_volume: &IAudioEndpointVolume) -> windows::core::Result<Volume> {
	let raw_volume = unsafe { audio_endpoint_volume.GetMasterVolumeLevelScalar()? };
	match raw_volume.try_into() {
		Ok(volume) => Ok(volume),
		Err(_) => Err(windows::core::Error::new(
			windows::core::HRESULT(GET_MASTER_VOLUME_SCALAR_FAILED),
			"failed to get master volume level scalar",
		)),
	}
}

fn set_volume(desired_volume_scalar: Volume, audio_endpoint_volume: &IAudioEndpointVolume) -> windows::core::Result<()> {
	unsafe { audio_endpoint_volume.SetMasterVolumeLevelScalar(desired_volume_scalar.into(), std::ptr::null()) }
}
