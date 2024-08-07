

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use cpal::{ InputCallbackInfo, Sample};
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

fn listen_sound<T: Sample>(data: &mut [T], _:&InputCallbackInfo) {
    for sample in data.iter_mut() {
    }
}

fn write_silence<T: Sample>(data: &mut [T], _:&cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = Sample::EQUILIBRIUM;
    }
}

// create silent stream. 
// problem: how to pass the error clousure?
// fn create_stream_1(sample_format: SampleFormat, odevice: Device, config: cpal::StreamConfig, err_fm: impl Fn()) {
//         let stream = match sample_format {
//             SampleFormat::F32 => odevice.build_output_stream(&config, write_silence::<f32>, err_fm, None),
//             SampleFormat::I16 => odevice.build_output_stream(&config, write_silence::<i16>, err_fn, None),
//             SampleFormat::U16 => odevice.build_output_stream(&config, write_silence::<u16>, err_fn, None),
//             SampleFormat::U8 => odevice.build_output_stream(&config, write_silence::<u8>, err_fn, None),
//             sample_format => panic!("Unsupported sample format '{sample_format}'")
//         }.unwrap();
      
//         stream.play().unwrap();
// }


fn write_noise<T: Sample>(data: &mut [T], _:&cpal::OutputCallbackInfo) {

    let mut rng = rand::thread_rng();
    for sample in data.iter_mut() {

        // generate a random sample val
        let random_value = match std::any::type_name::<T>() {
            "f32" => rng.gen_range(-1.0..=1.0) as T,
            "i16" => rng.gen_range(T::MIN ..= T::MAX),
            "u8" => rng.gen_range(T::MIN ..= T::MAX),
            _ => panic!("Unsupported sample type"),
        };
        
        // set the sample to the random value
    }
}


fn __explain()  {
    // Capture a closure's environment by value.
    // move converts any variables captured by reference or mutable reference to variables captured by value.

    let data = vec![1, 2, 3];
    let _clousure = move || println!("captured {data:?} by value");

    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned(); // creates owned data from borrowed data, usually by cloning.
        move || println!("this is a: {text}");
    }





    let fn_plain = create_fn();
    fn_plain();
    // move is often used when threads are involved.

    // let data = vec![1, 2, 3];

    std::thread::spawn(move || {
        println!("captured {data:?} by value")
    }).join().unwrap();

    // data was moved to the spawned thread, so we cannot use it here
    // move is also valid before an async block.

    let capture = "hello".to_owned();
    let block = async move {
        println!("rust says {capture} from async block");
    };
}