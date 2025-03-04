extern crate gst;

use gst::ElementT;
use gst::BinT;
use std::env;
use std::thread;
use std::num::Float;
use std::process::Command;


fn main(){
    gst::init();
    let pipeline_str = "autoaudiosrc ! audioconvert ! appsink name=appsink0 caps=\"audio/x-raw,format=F32LE,channels=1\"";
    let mut pipeline = gst::Pipeline::new_from_str(pipeline_str).unwrap();
	let mut mainloop = gst::MainLoop::new();
	let mut bus = pipeline.bus().expect("Couldn't get bus from pipeline");
	let bus_receiver = bus.receiver();
	let appsink = pipeline.get_by_name("appsink0").expect("Couldn't get appsink from pipeline");
	let appsink = gst::AppSink::new_from_element(appsink);
	mainloop.spawn();
	pipeline.play();
	Command::new("tput").args(&["civis","--","invisible"]).status();
	thread::spawn(move||{
		loop {
			match appsink.recv(){
				Ok(gst::appsink::Message::NewPreroll(sample)) | Ok(gst::appsink::Message::NewSample(sample)) => {
				    if let Some(mut buffer) = sample.buffer(){
						let rms = buffer.map_read(|mapping| {
							(mapping.iter::<f32>().fold(0.0f32, |rms, &sample| rms + sample*sample) / mapping.len::<f32>() as f32).sqrt()
						}).unwrap();
						for i in (0..80){
							if (rms*80.0) as u32 > i{
								print!("|");
							}else{
								print!(" ");
							}
						}
						print!("\r");
					}
				}
				Ok(gst::appsink::Message::Eos) => {
					println!("appsink thread received EOS, quitting");
					break;
				}
				Err(_) => {
					println!("appsink thread channel closed, quitting");
					break;
				}
			}
		}
	});
	
	for message in bus_receiver.iter(){
		match message.parse(){
			gst::Message::StateChangedParsed{ref msg, ref old, ref new, ref pending} => {
				println!("element `{}` changed from {:?} to {:?}", message.src_name(), old, new);
			}
			gst::Message::ErrorParsed{ref msg, ref error, ref debug} => {
				println!("error msg from element `{}`: {}, quitting", message.src_name(), error.message());
				break;
			}
			gst::Message::Eos(ref msg) => {
				println!("eos received quiting");
				break;
			}
			_ => {
				println!("msg of type `{}` from element `{}`", message.type_name(), message.src_name());
			}
		}
	}
	mainloop.quit();
	Command::new("tput").args(&["cnorm","--","normal"]).status();
}
