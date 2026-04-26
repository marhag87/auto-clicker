use clap::ArgGroup;
use clap::Parser;
use device_query::Keycode;

#[derive(Debug, Clone, Parser)]
#[clap(name = "Auto-Clicker")]
#[clap(group(ArgGroup::new("trigger").args(["mouse", "keyboard"])))]
pub struct Opts {
    /// Use mouse key to toggle auto-click. 1 = left, 2 = right, etc.
    #[arg(short = 'm', long = "mouse")]
    pub mouse: Option<usize>,

    /// Use keyboard key to toggle auto-click
    #[arg(short = 'k', long = "keyboard")]
    pub keyboard: Option<Keycode>,

    /// Target clicks per second
    #[arg(short = 'c', long = "cps", default_value_t = 50)]
    pub clicks_per_second: u32,
}
