use serde::{Serialize, Deserialize};
use clap::{App, Arg, SubCommand};

use std::io::{BufReader};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Manifest {
    minecraft: ManifestMinecraft,
    manifest_type: Box<str>,
    manifest_version: u8,
    name: String,
    version: String,
    author: String,
    files: Vec<ManifestFile>,
    overrides: String,
}

impl Manifest {
    pub fn new(name: String, version: String, author: String, mc_version: String) -> Self {
        Manifest {
            minecraft: ManifestMinecraft {
                version: mc_version,
                mod_loaders: Vec::new(),
            },
            manifest_type: "minecraftModpack".to_owned().into_boxed_str(),
            manifest_version: 1,
            name,
            version,
            author,
            files: Vec::new(),
            overrides: "overrides".to_owned()
        }
    }
    pub fn add_mod_loader(&mut self, id: String, primary: bool) {
        self.minecraft.mod_loaders.push(ModLoader {
            id, primary
        });
    }
    pub fn add_file(&mut self, project_id: u64, file_id: u64) {
        self.files.push(ManifestFile{project_id, file_id, required: true});
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ManifestMinecraft {
    version: String,
    mod_loaders: Vec<ModLoader>,
}

#[derive(Serialize, Deserialize)]
struct ModLoader {
    id: String,
    primary: bool,
}

#[derive(Serialize, Deserialize)]
struct ManifestFile {
    #[serde(rename = "projectID")]
    project_id: u64,
    #[serde(rename = "fileID")]
    file_id: u64,
    required: bool
}

use std::fs::File;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(SubCommand::with_name("new")
            .arg(Arg::with_name("NAME").required(true))
            .arg(Arg::with_name("VERSION").required(true))
            .arg(Arg::with_name("AUTHOR").required(true))
            .arg(Arg::with_name("MC_VERSION").required(true))
        )
        .subcommand(SubCommand::with_name("modloader")
            .arg(Arg::with_name("ID").required(true))
            .arg(Arg::with_name("PRIMARY").required(true))
        )
        .subcommand(SubCommand::with_name("add")
            .arg(Arg::with_name("PROJECT").required(true))
            .arg(Arg::with_name("FILE").required(true))
        )
        .get_matches();
    
    const PATH: &str = "manifest.json";

    match matches.subcommand() {
        ("new", Some(matches)) => {
            let name = matches.value_of("NAME").unwrap().to_owned();
            let version = matches.value_of("VERSION").unwrap().to_owned();
            let author = matches.value_of("AUTHOR").unwrap().to_owned();
            let mc_version = matches.value_of("MC_VERSION").unwrap().to_owned();

            let manifest = Manifest::new(name, version, author, mc_version);

            let file = File::create(PATH).unwrap();
            serde_json::to_writer_pretty(file, &manifest).unwrap();
        },
        ("modloader", Some(matches)) => {
            let mut manifest: Manifest = {
                let file = BufReader::new(File::open(PATH).unwrap());
                serde_json::from_reader(file).unwrap()
            };

            let id = matches.value_of("ID").unwrap().to_owned();
            let primary = matches.value_of("PRIMARY").unwrap().parse().unwrap();
            
            manifest.add_mod_loader(id, primary);
            
            let file = File::create(PATH).unwrap();
            serde_json::to_writer_pretty(file, &manifest).unwrap();
        },
        ("add", Some(matches)) => {
            let mut manifest: Manifest = {
                let file = BufReader::new(File::open(PATH).unwrap());
                serde_json::from_reader(file).unwrap()
            };
            
            let project_id = matches.value_of("PROJECT").unwrap().parse().unwrap();
            let file_id = matches.value_of("FILE").unwrap().parse().unwrap();

            manifest.add_file(project_id, file_id);

            let file = File::create(PATH).unwrap();
            serde_json::to_writer_pretty(file, &manifest).unwrap();
        },
        _ => eprintln!("Big oof")
    }
}
