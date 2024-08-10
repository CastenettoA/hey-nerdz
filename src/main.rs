

use std::sync::{Arc, Mutex};
use std::{process, thread};
use std::time::Duration;
use cpal::{ BuildStreamError, Device, Host, InputCallbackInfo, Sample, StreamConfig, StreamError};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rand::Rng;

fn main() {
    let (idevice, _odevice) = get_cpal_defaults();
    record_audio(&idevice, 5);
}

fn record_audio(idevice: &Device, second: u64) {
    let (config, buffer, buffer_clone) = get_stream_defaults(&idevice);
    
    let stream = idevice.build_input_stream(
        &config,
        move |data: &[u8], _:&cpal::InputCallbackInfo| { 
            /* reat to events stream reading or writing stream data here. */ 
            let mut buffer = buffer_clone.lock().unwrap();
            buffer.extend_from_slice(data)
        },
        |err| eprintln!("Stream error: {:?}", err),
        None
    );

   println!("unwraping the stream...");
   let stream_inner = stream.unwrap();

   println!("playing the stream");
   stream_inner.play().unwrap();

   // log time on stdo
   let _log_time_handler = thread::spawn(|| { // put the f() in a thread so it become non blocking
       log_time(6);
   });

   thread::sleep(Duration::from_secs(1)); // wait 1s before print buffer
//    println!("{:#?}", buffer);

   print_bytes(25, buffer);

   thread::sleep(Duration::from_secs(1));
//    println!("{:?}", buffer_clone);
//    process::exit(0); // exit from the program

    // // retrieve the captured data
    // let captured_data = Arc::try_unwrap(buffer).unwrap().into_inner().unwrap();
    
    // println!("now wait to seconds and after exit from the process.");
    // println!("Captured {} bytes of audio data", captured_data.len());
 
    //  _stream.pause();
 

}

fn log_time(len: i32) {
    let mut i = 1;
    while i < len {
        println!("{}", i);
        thread::sleep(Duration::from_secs(1)); // sleep 1s and continue
        i += 1;
    }
}

fn get_stream_defaults(idevice: &Device) -> (StreamConfig, Arc<Mutex<Vec<u8>>>, Arc<Mutex<Vec<u8>>> ) {
    let config_default = idevice.default_input_config().expect("no def. input config for this input device");
    let config = config_default.into();
    let buffer = Arc::new(Mutex::new(Vec::new())); 
    let buffer_clone = buffer.clone();

    (config, buffer, buffer_clone)
}

fn get_cpal_defaults() -> (Device, Device) {
    let host = cpal::default_host();    
    let idevice = host.default_input_device().expect("hey, seems I not found some default input device here!");
    let odevice = host.default_output_device().expect("no output device avaliable");

    (idevice, odevice)
}

fn print_bytes(num_of_byte: usize, buf: Arc<Mutex<Vec<u8>>>) {
    let buffer_clone = Arc::clone(&buf);

    let handle = thread::spawn(move || {
        let buffer: std::sync::MutexGuard<Vec<u8>> = buffer_clone.lock().unwrap();
        let mut buffer_slice = Vec::with_capacity(num_of_byte);

        for i in 0..num_of_byte {
            buffer_slice.push(buffer[i]);
        }

        println!("the buffer len is: {} and the first {} bytes are {:#?}", buffer.len(), num_of_byte, buffer_slice);
    });

    handle.join().unwrap();
}



/**** * For the people
 * 
 * Arc -> allows multiple ownership of the same data across threads
 *        aka thread-safe reference counting.
 * Mutex -> Ensures that only one thread can access the data at a time,
 *         preventing concurrent modification and data races.
 */