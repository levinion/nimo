#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Commands {
    /// relative path to download from repo, such as levinion/dotfiles/nvim/init.lua
    pub path: String,
    #[clap(short, long)]
    /// output path that should be a dir
    pub output: Option<String>,
    #[clap(short, long)]
    /// repo branch, the default one is "main"
    pub branch: Option<String>,
}
