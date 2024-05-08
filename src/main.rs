use anyhow::{anyhow, Result};
use clap::{arg, Arg, ArgAction, Command};
use std::{
    fs::File,
    io::{BufRead, BufReader, LineWriter, Write},
    path::PathBuf,
};

fn main() -> Result<()> {
    let matches = Command::new("Data extract and duplicater")
        .version("0.1")
        .arg(Arg::new("DATA"))
        .arg(Arg::new("LINES"))
        .arg(Arg::new("OUTPUT"))
        .arg(arg!(-l --line_repeat <line_repeat_amount> "Amount of times to repeat the <LINES> input file per data line"))
        .arg(arg!(-v --verbose "Show output").action(ArgAction::SetTrue))
        .get_matches();

    let line_file_path = PathBuf::from(
        matches
            .get_one::<String>("LINES")
            .ok_or(anyhow!("Missing LINES argument"))?,
    );
    let data_file_path = PathBuf::from(
        matches
            .get_one::<String>("DATA")
            .ok_or(anyhow!("Missing DATA argument"))?,
    );
    let output_file_path = PathBuf::from(
        matches
            .get_one::<String>("OUTPUT")
            .ok_or(anyhow!("Missing OUTPUT argument"))?,
    );
    let verbose = matches.get_flag("verbose");
    let line_repeat: usize = matches
        .get_one::<String>("line_repeat")
        .unwrap_or(&'1'.to_string())
        .parse()
        .unwrap();

    if !line_file_path.is_file() {
        return Err(anyhow!("LINES argument is not a file"));
    }

    if !data_file_path.is_file() {
        return Err(anyhow!("DATA argument is not a file"));
    }

    let f = File::create(output_file_path)?;
    let l = File::open(line_file_path)?;
    let d = File::open(data_file_path)?;

    let l_lines: Vec<String> = BufReader::new(l).lines().map_while(Result::ok).collect();
    let d_lines: Vec<String> = BufReader::new(d).lines().map_while(Result::ok).collect();

    for data in d_lines {
        for line in &l_lines {
            for block_index in 0..line_repeat {
                let real_block_index = block_index + 1;

                let mut new_line = line.to_owned();
                new_line = new_line.replace("{block_index}", &real_block_index.to_string());
                let data_fields: Vec<&str> = data.split(';').collect();

                if data_fields.is_empty() {
                    return Err(anyhow!("No data provided"));
                } else if data_fields.len() == 1 {
                    // try to replace both options
                    new_line = new_line.replace("{}", &data);
                    new_line = new_line.replace("{1}", &data);
                } else {
                    for (index, data_field) in data_fields.iter().enumerate() {
                        let pos = index + 1;

                        //let replace_string = ;
                        new_line = new_line.replace(&format!("{{{}}}", pos), data_field);
                    }
                }

                let mut wb = LineWriter::new(&f);
                let mut line_slice = new_line.as_bytes().to_vec();
                line_slice.push(b'\n');

                wb.write_all(&line_slice)?;

                if verbose {
                    println!("{}", new_line);
                }
            }
        }
    }

    Ok(())
}
