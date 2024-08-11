

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::{process, thread};
use std::time::Duration;
use cpal::{ BuildStreamError, Device, Host, InputCallbackInfo, Sample, Stream, StreamConfig, StreamError};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

// listen continuosly the audio
// match on this audio flow a pattern like "nerdz" and react with a event
// (feature) save only last 5s stream, discard the rest

fn main() {
    let (idevice, _odevice) = get_cpal_defaults();
    let (conf, buf, buf_clone, buf_size) = get_stream_defaults(&idevice);
    
    let stream = record_sound(idevice, &conf, buf_clone, buf_size);
    let _s = stream.play().unwrap(); 
    println!("Start recording audio 🎙️ with buffer size of {}", buf_size);


    thread::sleep(Duration::from_millis(1000)); // sleep 20ms to get some data on the buffer

    let local_buffer_clone = Arc::clone(&buf); // creo un nuovo puntatore
    let local_buffer_clone = local_buffer_clone.lock().unwrap(); // acquisisco un mutex così da poter accedere ai dati
    let buf_len: usize = local_buffer_clone.len();
    if buf_len >= 2000 {

        for index in 0..2000 {
            print!("{:} ", local_buffer_clone[index]);
        }
        println!("the buffer len() is {} byte", buf_len);
    } else {
        println!("il buffer ha solo {} elementi", buf_len); // 3480 significa che ci sono 3480 elementi.. essendo elementi u8
        for index in 0..buf_len {
            println!("{:?}", local_buffer_clone[index]);
        }
    }

    // keep the app running
    // thread::park(); 
}

// fn play_sound(odevice: Device, config: &StreamConfig, buffer_clone: Arc<Mutex<Vec<u8>>>) {
//     let stream = odevice.build_output_stream(&config, data_callback, error_callback, timeout);

// }

fn record_sound(idevice: Device, conf: &StreamConfig, buf_clone: Arc<Mutex<VecDeque<u8>>>, buf_size: usize) -> Stream {
    let stream = idevice.build_input_stream(
        &conf,
        move |data: &[u8], _:&cpal::InputCallbackInfo| { 
            /* reat to events stream reading or writing stream data here. */ 
            let mut buf: std::sync::MutexGuard<VecDeque<u8>> = buf_clone.lock().unwrap();

            // add the new data to the buffer
            for &sample in data {
                if buf.len() == buf_size {
                    buf.pop_front(); // remove the oldest sample if buffer is full
                }
                buf.push_back(sample);
            }
        },
        |err| eprintln!("Stream error: {:?}", err),
        None
    );

    stream.unwrap()
}

// fn time_indicator(stay_for: i32) {
//     let mut i = 1;
//     while i < stay_for {
//         print!(".");
//         thread::sleep(Duration::from_millis(500)); // sleep 1s and continue
//         i += 1;
//     }
// }

// return the buffer size (sample_rate * channels * duration_seconds)
fn get_buf_size(conf: &StreamConfig, sec: u32) -> usize {
    let size = (conf.channels as u32) * conf.sample_rate.0 * sec;
    size as usize
}

fn get_stream_defaults(idevice: &Device) -> (StreamConfig, Arc<Mutex<VecDeque<u8>>>, Arc<Mutex<VecDeque<u8>>>, usize ) {
    let conf_default: cpal::SupportedStreamConfig = idevice.default_input_config().expect("no def. input config for this input device");
    let conf: StreamConfig = conf_default.into();
    let buf_size = get_buf_size(&conf, 5);

    // make a thread-safe circular buffer reference-counting pointer (atomically reference counted)
    let buf: Arc<Mutex<VecDeque<u8>>> = Arc::new(Mutex::new(VecDeque::with_capacity(buf_size))); 

    // clono un puntatore Arc. Invocare clone in Arc produce una nuova instanza che punta
    // alla stessaa allocazione nell'heap per la risorsa Arc.
    // il 'reference counting' viene incrementato
    // Arc è trhead-safe
    let buf_clone = buf.clone(); 

    (conf, buf, buf_clone, buf_size)
}

fn get_cpal_defaults() -> (Device, Device) {
    let host = cpal::default_host();    
    let idevice = host.default_input_device().expect("hey, seems I not found some default input device here!");
    let odevice = host.default_output_device().expect("no output device avaliable");

    (idevice, odevice)
}

fn print_buffer_bytes(num_of_byte_to_print: usize, buf: &Arc<Mutex<Vec<u8>>>) {
    let buffer_clone = Arc::clone(&buf);


    let handle = thread::spawn(move || {
        let buffer: std::sync::MutexGuard<Vec<u8>> = buffer_clone.lock().unwrap();
        let mut buffer_slice = Vec::with_capacity(num_of_byte_to_print);

        for i in 0..num_of_byte_to_print {
            buffer_slice.push(buffer[i]);
        }

        println!("the buffer len is: {} and the first {} bytes are {:#?}", buffer.len(), num_of_byte_to_print, buffer_slice);
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