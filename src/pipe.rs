
use common::{ArgParsable};
use remote::RemoteClient;
use transform::{Command, CharStream, empty_stream};
use head::HeadOptions;
use cut::CutOptions;
use grep::GrepOptions;
use wget::WgetOptions;

pub fn pipe(input: String) -> Result<CharStream, String> {
    let mut prev_response: CharStream = empty_stream();
    
    let client = RemoteClient::new();

    for line in input.lines() {
        let mut parts = line.split(" ");
        let command = parts.next().ok_or("missing command on line")?;
        let args_parts: Vec<_> = parts.collect();
        let args = args_parts.join(" ");

        println!("{} {}", command, args);
        let remote = false;
        prev_response = execute(prev_response, command, &args, &client, remote)?;
    }

    Ok(prev_response)
}

fn execute(input: CharStream, command_name: &str, args: &str, client: &RemoteClient, remote: bool) -> Result<CharStream, String> {
    match command_name {
        "wget" => WgetOptions::from_args(&args)?
                    .execute(input, false, client),
        "head" => HeadOptions::from_args(&args)?
                    .execute(input, remote, client),
        "cut" =>  CutOptions::from_args(&args)?
                    .execute(input, remote, client),
        "grep" => GrepOptions::from_args(&args)?
                    .execute(input, remote, client),
        _ => return Err(format!("Unknown command: {:?}", command_name)) 
    }
}

// TODO: impl LinesTransform for pipe
// should be able to work in two modes? 
// * read commands from stdin
// * read commands as options, piping stdin to first command 
//      * need to parse pipes, -p p1 -p p2 ? quoting? query strings? 