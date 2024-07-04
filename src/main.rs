use capnp::{message::ReaderOptions, serialize_packed};
use clap::Parser;
use cli::Args;
use hardware_logviewer::proto_capnp;
use std::{
  collections::HashSet,
  error::Error, fs::File,
  io::{stdout, BufReader, Write},
  thread::sleep,
  time::Duration
};

mod cli;

fn main() {
  let args = cli::Args::parse();
  process(args).unwrap();
}

fn process(args: Args) -> Result<(), Box<dyn Error>> {
  let file = File::open(args.file_name)?;
  let mut reader = BufReader::new(file);
  let speed = if args.real_time { args.real_time_speed } else { 0 };
  let streams: HashSet<u8> = args.streams.into_iter().collect();
  while let Ok(reader) = serialize_packed::read_message(&mut reader, ReaderOptions::new()) {
    let frame = reader.get_root::<proto_capnp::data_frame::Reader>()?;
    if !streams.is_empty() && !streams.contains(&frame.get_stream()) {
      continue;
    }
    if speed > 0 {
      let time = frame.get_time() / speed;
      if time > 0 {
        sleep(Duration::from_millis(time as u64));
      }
    }
    stdout().write(frame.get_payload()?)?;
  }
  Ok(())
}
