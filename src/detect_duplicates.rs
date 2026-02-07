use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug)]
struct Dict {
    pub input: String,
    _output: String,
}

pub struct DuplicateDetector {}

impl DuplicateDetector {
    pub fn exec(detect_file: PathBuf) -> Result<()> {
        let contents = Self::read_emoji_file(&detect_file)?;
        let dicts = Self::build_dicts(contents);
        let duplicates = Self::detect_duplicates(dicts);
        for (input, count) in duplicates {
            println!("{:?}: {:?}", input, count);
        }

        Ok(())
    }

    fn read_emoji_file(path: &PathBuf) -> Result<String> {
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        Ok(contents)
    }

    fn build_dicts(contents: String) -> Vec<Dict> {
        let mut dicts = vec![];

        for line in contents.lines() {
            let split_line = line.split('\t').collect::<Vec<_>>();
            if split_line.len() >= 2 {
                dicts.push(Dict {
                    input: split_line[0].to_string(),
                    _output: split_line[1].to_string(),
                });
            }
        }

        dicts
    }

    fn detect_duplicates(dicts: Vec<Dict>) -> Vec<(String, u32)> {
        let mut map = HashMap::<String, u32>::new();
        dicts.into_iter().for_each(|d| {
            let count = map.entry(d.input).or_insert(0);
            *count += 1;
        });

        let mut vec = map
            .into_iter()
            .filter(|x| x.1 >= 2)
            .collect::<Vec<(_, _)>>();
        vec.sort_by_key(|x| (x.1, x.0.to_string()));

        vec
    }
}
