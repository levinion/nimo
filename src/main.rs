use clap::Parser;

mod cmd;

#[tokio::main]
async fn main() {
    let cmd = cmd::Commands::parse();
    let v: Vec<_> = cmd.path.splitn(3, "/").collect();
    let mut client = gitload::DownloaderBuilder::new(v[0], v[1], v[2]);
    if cmd.output.is_some() {
        client = client.local_path(&cmd.output.unwrap());
    }
    let client = client.build();
    client.download().await.unwrap();
}
