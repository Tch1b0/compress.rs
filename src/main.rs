pub mod compress;
pub mod decompress;
pub mod general;

use clap::Parser;
use compress::compress;
use decompress::decompress;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// the file to compress/decompress
    #[arg(short, long)]
    source_file: String,

    /// the compressed/decompressed file
    #[arg(short, long)]
    dest_file: String,

    /// whether or not to decompress
    #[arg(long, action)]
    decompress: bool,
}

fn main() {
    let args = Args::parse();

    if args.decompress {
        match decompress(args.source_file, args.dest_file) {
            Ok(s) => println!("done! New size: {s} Bytes"),
            Err(e) => panic!("{e:?}")
        }
    } else {
        match compress(args.source_file, args.dest_file) {
            Ok(s) => println!("done! New size: {s} Bytes"),
            Err(e) => panic!("{e:?}")
        }
    }
}
