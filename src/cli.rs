use clap::Parser;

#[derive(Parser)]
pub struct Args {
  pub file_name: String,
  
  /// streams to replay. defaults to all streams.
  /// 0 for user input and 1 for cluster output
  #[clap(short = 't')]
  pub streams: Vec<u8>,

  /// replay logs in real time
  #[clap(short)]
  pub real_time: bool,

  /// speed to replay logs at
  #[clap(short = 's', default_value_t=1)]
  pub real_time_speed: u32,
}