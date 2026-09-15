use std::io;
use std::process::ExitCode;

use rs_pq_arrow_schema::Mode;
use rs_pq_arrow_schema::PqFilename;

fn io_filename() -> impl Fn() -> String {
    || std::env::var("ENV_PQ_FILENAME").unwrap_or_default()
}

fn io_mode() -> impl Fn() -> Mode {
    || {
        std::env::var("ENV_SHOW_MODE")
            .ok()
            .and_then(|s| str::parse(&s).ok())
            .unwrap_or_default()
    }
}

fn sub() -> Result<(), io::Error> {
    let fname: String = io_filename()();
    let mode: Mode = io_mode()();
    mode.show_type_info(PqFilename(fname))?;
    Ok(())
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        let fname: String = io_filename()();
        eprintln!("ENV_PQ_FILENAME: {fname}");
        ExitCode::FAILURE
    })
}
