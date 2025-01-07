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
    if cmd.branch.is_some() {
        client = client.branch(&cmd.branch.unwrap());
    }

    let client = client
        .on_process(|process| {
            let bar_size = 40.;
            let blank = ((1. - process.percent()) * bar_size).round() as usize;
            print!(
                "\r[{}{}]\t{}/{}\t{:.0}%",
                "#".repeat(bar_size as usize - blank),
                " ".repeat(blank),
                process.current,
                process.all,
                process.percent() * 100.
            );
        })
        .build();

    client
        .download()
        .await
        .unwrap_or_else(|err| println!("{err}"));
}
