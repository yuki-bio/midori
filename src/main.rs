use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Result ,Write};
use bio::io::fastq;
use clap::{Arg, Command};
use indicatif::{ProgressBar, ProgressStyle};

mod compressor;

fn append_bytes_to_file(bytes: &[u8], filename: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)?;
    
    file.write_all(&[0x1E])?;
    file.write_all(bytes)?;
    Ok(())
}

fn append_ascii_to_file(text: &str, filename: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)?;
    file.write_all(&[0x1E])?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

fn main() -> io::Result<()> {
    let matches = Command::new("file_io")
        .version("1.0")
        .author("Yuki <https://github.com/yuki-bio>")
        .about("A program that compresses FASTQ files into a binary file that can be directly analyzed.")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets the input file to use")
                .required(true)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Sets the output file to use")
                .required(true)
                .value_parser(clap::value_parser!(String)),
        )
        .get_matches();
    let reader = BufReader::new(File::open(matches.get_one::<String>("input").unwrap())?);
    let file_path=matches.get_one::<String>("output").unwrap();
    let mut file = File::create(file_path)?;
    let fastq_reader = fastq::Reader::new(reader);
    let mut flag=true;
    let mut base_id=String::from("");
    let mut base_description=String::from("");
    let mut previous_quality=String::from("");
    let total_records = fastq_reader.records().count() as u64;
    let progress_bar = ProgressBar::new(total_records);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));
    let reader = BufReader::new(File::open(matches.get_one::<String>("input").unwrap())?);
    let fastq_reader = fastq::Reader::new(reader);
    for result in fastq_reader.records() {
        match result {
            Ok(record) => {
                let id = record.id().to_string();
                let seq = record.seq().to_vec();
                let qual = record.qual().to_vec();
                let desc = record.desc().unwrap_or("").to_string();
                let id_diff=compressor::diff_text(&base_id,&id);
                let desc_diff=compressor::diff_text(&base_description,&desc);
                if flag{
                    let number: u16 = String::from_utf8(seq.to_vec()).unwrap().len() as u16; //Get DNA length.
                    let bytes = number.to_le_bytes();
                    file.write_all(&bytes)?;
                    base_id=id;
                    base_description=desc;
                    flag=false;
                }
                let id_comp=compressor::replace_at_symbols(&id_diff);
                let _ = append_ascii_to_file(&id_comp,file_path);
                let seq_str=String::from_utf8(seq.to_vec()).unwrap();
                let bytes = compressor::convert_dna_to_bits(&seq_str);
                let _ = append_bytes_to_file(&bytes, file_path);
                let desc_comp=compressor::replace_at_symbols(&desc_diff);
                let _ = append_ascii_to_file(&desc_comp,file_path);
                let quality_str=String::from_utf8(qual.to_vec()).unwrap();
                let quality_diff=compressor::diff_text(&previous_quality,&quality_str);
                let quality_comp=compressor::replace_at_symbols(&quality_diff);
                let _ = append_ascii_to_file(&quality_comp,file_path);
                previous_quality=String::from_utf8(qual.to_vec()).unwrap();
                progress_bar.inc(1);
            }
            Err(e) => {
                // Handle fastq::Error here
                return Err(io::Error::new(io::ErrorKind::Other, format!("Fastq error: {}", e)));
            }
        }
    }
    progress_bar.finish_with_message("done");
    Ok(())
}
