use std::io;
use std::process::ExitCode;

use io::BufWriter;
use io::Write;

use io::BufRead;

use rs_longs2bin::Mode;

enum Arg {
    Help,
    Endian(Mode),
}

impl Arg {
    pub fn from_args() -> Self {
        for oarg in std::env::args_os() {
            let b: &[u8] = oarg.as_encoded_bytes();
            match b {
                b"-h" => return Self::Help,
                b"-l" => return Self::Endian(Mode::Le),
                b"-b" => return Self::Endian(Mode::Be),
                _ => continue,
            }
        }
        Self::Help
    }

    pub fn show_help() {
        println!("-h: Shows this help");
        println!("-l: little endian mode");
        println!("-b: big endian mode");
    }

    pub fn exec(mode: &Mode) -> Result<(), io::Error> {
        let lines = std::io::stdin().lock().lines();
        let parsed = lines.map(|rline| {
            rline.and_then(|line| {
                let lng: i64 = str::parse(&line).map_err(io::Error::other)?;
                Ok(lng)
            })
        });

        let o = std::io::stdout();
        let mut ol = o.lock();
        {
            let mut bw = BufWriter::new(&mut ol);
            mode.lngs2writer(parsed, &mut bw)?;
            bw.flush()?;
        }
        ol.flush()
    }
}

fn sub() -> Result<(), io::Error> {
    let arg: Arg = Arg::from_args();
    match arg {
        Arg::Help => {
            Arg::show_help();
            Ok(())
        }
        Arg::Endian(mode) => Arg::exec(&mode),
    }
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
