

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use cpal::{ InputCallbackInfo, Sample, StreamError};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rand::Rng;

fn main() {
    let host = cpal::default_host();    
    let odevice = host.default_output_device().expect("no output device avaliable");
    let idevice = host.default_input_device().expect("hey, seems I not found some default input device here!");

    let config_default = idevice.default_input_config().expect("no def. input config for this input device");
    let _config = config_default.into();
    let error_callback = |err: StreamError| { eprintln!("an error occurred: {}", err) };
    // let timeout = Some(Duration::new(4, 0)); // listen for 4 second and after quit.
    
    // arc = atomically reference counted. A trhead safe referenced counted pointer.
    let buffer = Arc::new(Mutex::new(Vec::new())); 
    let buffer_clone = buffer.clone();

    let stream = idevice.build_input_stream(
       &_config,
       move |data: &[u8], _:&cpal::InputCallbackInfo| { 
        /* reat to events stream reading or writing stream data here. */ 
        let mut buffer = buffer_clone.lock().unwrap();
        buffer.extend_from_slice(data)
    },
       |err| eprintln!("Stream error: {:?}", err),
       None
    );

    let _stream = stream.expect("...");
    _stream.play();
    thread::sleep(Duration::from_secs(5)); // wait for 5 seconds
    _stream.pause();

    // retrieve the captured data
    let captured_data = Arc::try_unwrap(buffer).unwrap().into_inner().unwrap();
    println!("Captured {} bytes of audio data", captured_data.len());

    Ok(());
}