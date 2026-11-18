use std::{fmt::Write, fs, path::PathBuf};

use change_case::snake_case;

use crate::cluster_file::{GlobalsBitmaps, GlobalsEnums, GlobalsStructs};

mod cluster_file;

fn main() {
    let dir = fs::read_dir("xtask/1.6.1/clusters").unwrap();

    let mut errors = Vec::new();
    let mut cluster_names = Vec::new();

    for entry in dir {
        let entry = entry.unwrap();

        let path = entry.path();

        if path.extension().is_none_or(|ext| ext != "xml") {
            continue;
        }
        let file_name = path.file_stem().unwrap().to_str().unwrap();
        let cluster_name = snake_case(file_name);

        let content = fs::read_to_string(&path).unwrap();

        let mut file = match cluster_file::ClusterFile::parse(&content) {
            Ok(file) => {
                file
            }
            Err(err) => {
                errors.push((path, err));
                continue;
            }
        };

        file.cluster_file_name = Some(cluster_name.clone());

        fs::write(out_file_path("clusters", &cluster_name), file.to_string()).unwrap();

        cluster_names.push(cluster_name);
    }

    let mod_file = {
        let mut content = String::new();

        for name in &cluster_names {
            writeln!(&mut content, "pub mod {};", name).unwrap();
        }

        content
    };
    fs::write(out_file_path("clusters", "mod"), mod_file).unwrap();

    println!("Failed to parse {} files", errors.len());
    for (path, error) in errors {
        println!("{}: {}", path.display(), error)
    }

    let import = "use crate::cluster_def::*;\nuse crate::tlv::*;";

    {
        use std::fmt::Write;
        let global_structs = fs::read_to_string("xtask/1.6.1/globals/Structs.xml").unwrap();

        let structs = GlobalsStructs::parse(&global_structs).unwrap();
        
        let mut result_str = String::new();
        writeln!(result_str, "{import}").unwrap();
        writeln!(result_str, "use super::*;").unwrap();

        for struct_def in structs.structs {
            writeln!(result_str, "{}", struct_def.entry(&[])).unwrap();
        }

        fs::write("ratter-cluster/src/globals/structs.rs", result_str).unwrap();
    }

    {
        use std::fmt::Write;
        let global_structs = fs::read_to_string("xtask/1.6.1/globals/Enums.xml").unwrap();

        let structs = GlobalsEnums::parse(&global_structs).unwrap();
        
        let mut result_str = String::new();
        writeln!(result_str, "{import}").unwrap();

        for enum_def in structs.enums {
            writeln!(result_str, "{}", enum_def).unwrap();
        }

        fs::write("ratter-cluster/src/globals/enums.rs", result_str).unwrap();
    }

    {
        use std::fmt::Write;
        let global_structs = fs::read_to_string("xtask/1.6.1/globals/Bitmaps.xml").unwrap();

        let bitmaps = GlobalsBitmaps::parse(&global_structs).unwrap();
        
        let mut result_str = String::new();
        writeln!(result_str, "{import}").unwrap();

        for bitmap_def in bitmaps.bitmaps {
            writeln!(result_str, "{}", bitmap_def).unwrap();
        }

        fs::write("ratter-cluster/src/globals/bitmaps.rs", result_str).unwrap();
    }
}


fn out_file_path(dir_name: &str, name: &str) -> PathBuf {
    let mut file_path = PathBuf::from("ratter-cluster/src");

    file_path.push(&dir_name);
    file_path.push(&name);
    file_path.add_extension("rs");

    file_path
}