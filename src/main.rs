#![deny(clippy::all)]
#![forbid(unsafe_code)]
// Disable the Windows Console
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use rand::Rng;

use lavagna::*;

/// The uncluttered blackboard
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'u', long)]
    collab_url: Option<String>,
    #[clap(short = 'i', long)]
    collab_id: Option<u16>,
    #[clap(long)]
    show_debug_pane: bool,
    #[clap(short = 'v', long)]
    verbose: bool,
    #[clap(long)]
    ui: bool,
}

fn main() {
    let mut rng = rand::thread_rng();

    let args = Args::parse();

    let collab = if let Some(collab_url) = args.collab_url {
        Some(CollabOpt {
            url: collab_url,
            collab_id: args.collab_id.unwrap_or_else(|| rng.gen::<u16>()),
        })
    } else {
        None
    };

    run(Opt {
        collab,
        show_debug_pane: args.show_debug_pane,
        verbose: args.verbose,
        ui: args.ui,
    })
}
