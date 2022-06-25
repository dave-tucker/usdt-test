use std::time::Duration;

use log::{info, warn};
use probe::probe;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};
use tokio::{signal, time::sleep};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    TermLogger::init(
        LevelFilter::Debug,
        ConfigBuilder::new()
            .set_target_level(LevelFilter::Error)
            .set_location_level(LevelFilter::Error)
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;
    probe!(clock, begin);
    tokio::spawn(async move {
        let mut elapsed: u128 = 0;
        loop {
            sleep(Duration::from_secs(1)).await;
            elapsed += 1;
            probe!(clock, loop, 1, elapsed);
            match elapsed % 24 {
                0 => info!("🕛"),
                1 => info!("🕧"),
                2 => info!("🕐"),
                3 => info!("🕜"),
                4 => info!("🕑"),
                5 => info!("🕝"),
                6 => info!("🕒"),
                7 => info!("🕞"),
                8 => info!("🕓"),
                9 => info!("🕟"),
                10 => info!("🕔"),
                11 => info!("🕠"),
                12 => info!("🕕"),
                13 => info!("🕡"),
                14 => info!("🕖"),
                15 => info!("🕢"),
                16 => info!("🕗"),
                17 => info!("🕣"),
                18 => info!("🕘"),
                19 => info!("🕤"),
                20 => info!("🕙"),
                21 => info!("🕥"),
                22 => info!("🕚"),
                _ => info!("🕦"),
            }
        }
    });
    warn!("Running until Ctrl-C");
    signal::ctrl_c().await?;
    probe!(clock, end);
    Ok(())
}
