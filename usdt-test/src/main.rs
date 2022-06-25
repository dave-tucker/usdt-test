use aya::{
    include_bytes_aligned,
    maps::{Array, HashMap},
    programs::{
        usdt::{USDT_IP_TO_SPEC_MAP, USDT_SPEC_MAP},
        Usdt,
    },
    Bpf,
};
use aya_log::BpfLogger;
use clap::Parser;
use log::info;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};
use tokio::signal;

#[derive(Debug, Parser)]
struct Opt {
    #[clap(short, long)]
    pid: Option<i32>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse();

    TermLogger::init(
        LevelFilter::Debug,
        ConfigBuilder::new()
            .set_target_level(LevelFilter::Error)
            .set_location_level(LevelFilter::Error)
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    #[cfg(debug_assertions)]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/debug/usdt-test"
    ))?;
    #[cfg(not(debug_assertions))]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/release/usdt-test"
    ))?;
    BpfLogger::init(&mut bpf)?;

    let spec_map = Array::try_from(bpf.map_mut(USDT_SPEC_MAP).unwrap())?;
    let ip_to_spec_map = HashMap::try_from(bpf.map_mut(USDT_IP_TO_SPEC_MAP).unwrap())?;
    let program: &mut Usdt = bpf.program_mut("usdt").unwrap().try_into()?;
    program.load()?;
    program.attach(
        spec_map,
        ip_to_spec_map,
        "clock",
        "loop",
        "/home/dave/dev/aya-rs/usdt-test/target/debug/clock",
        opt.pid,
    )?;

    info!("Waiting for Ctrl-C...");
    signal::ctrl_c().await?;
    info!("Exiting...");

    Ok(())
}
