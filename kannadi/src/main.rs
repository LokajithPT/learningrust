use clap::Parser;
use std::path::{self, PathBuf};
use path_clean::PathClean; 
use std::collections::HashMap; 
use std::fs::{self, Metadata};
use std::path::Path; 
use walkdir::WalkDir;
use fs_extra::file::{self, CopyOptions};


/// A simple file synchronizer, a lite version of rsync.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The source directory to synchronize from.
    #[arg(value_name = "SOURCE")]
    source: PathBuf,

    /// The replica directory to synchronize to.
    #[arg(value_name = "REPLICA")]
    replica: PathBuf,
}

fn main() -> Result<(), std::io::Error>{
    let args = Args::parse();

    if !args.source.is_dir(){
        eprintln!("error: source is not a dir "); 
        return Ok(()); 

    }

    if !args.replica.is_dir() {
        eprintln!("error: replica path aint there ")
    }

    println!("Source directory: {:?}", args.source);
    println!("Replica directory: {:?}", args.replica);
    println!("___"); 

    let mut source_files = HashMap::new();


    for entry in WalkDir::new(&args.source ).into_iter().filter_map(
        |e| e.ok()
    ){
        if entry.file_type().is_file(){
            let relative_path = entry.path().strip_prefix(&args.source).unwrap();
            let clean_path = relative_path.to_path_buf().clean();


            let metadata = fs::metadata(entry.path())?;

            source_files.insert(clean_path, metadata);
        }
    }


    println!("Found {} files in source :" , source_files.len());
    for (path, metadata) in &source_files{
        println!(
            " - {:?} ({} bytes , modified: {:?})", 
            path, 
            metadata.len(), 
            metadata.modified()?
        );
    }



    let mut replica_files  = HashMap::new();


    for entry in WalkDir::new(&args.replica ).into_iter().filter_map(
        |e| e.ok()
    ){
        if entry.file_type().is_file(){
            let relative_path = entry.path().strip_prefix(&args.replica).unwrap();
            let clean_path = relative_path.to_path_buf().clean();


            let metadata = fs::metadata(entry.path())?;

            replica_files.insert(clean_path, metadata);
        }
    }


    println!("Found {} files in replica  :" , replica_files.len());
    for (path, metadata) in &replica_files {
        println!(
            " - {:?} ({} bytes , modified: {:?})", 
            path, 
            metadata.len(), 
            metadata.modified()?
        );
    }

    let mut filestocopy = Vec::new(); 

    for (path , source_meta ) in &source_files {
        let should_copy = match replica_files.get(path){
            None => true , 

            Some(replica_meta ) => {
                source_meta.len() != replica_meta.len() || source_meta.modified()? != replica_meta.modified()?
            }

        }; 

        if should_copy{
            filestocopy.push(path.clone());
        }
    }


    println!("\n --- sync plan ---"); 
    if filestocopy.is_empty(){
        println!("no files fam "); 

    }
    else {
        println!("files to copy from source to replica :");
        for path in &filestocopy{
            println!(" - {:?}", path); 

        }
    }
    

    //shit shit shit 
    
    if !filestocopy.is_empty(){
        println!("\n--- executing copy ---"); 
        let mut  options = CopyOptions::new(); 

        options.overwrite = true ; 


        for relative_path in &filestocopy {
            let source_path = args.source.join(relative_path); 
            let replica_path = args.replica.join(relative_path); 
            
            if let Some(parent_dir) = replica_path.parent(){
                if !parent_dir.exists(){
                    fs::create_dir_all(parent_dir)?;
                    println!("created dir : {:?}" , parent_dir);
                }
            }

            match file::copy(&source_path, &replica_path, &options){
                Ok(bytes) => println!(
                    "Copied {:?} -> {:?} ({} bytes )", 
                    source_path, replica_path, bytes 
                ), 
                Err(e) => eprintln!(
                    "Error copying {:?}: {}", 
                    source_path, e 
                ), 

            }

        }
    }


    let mut filestodelete = Vec::new(); 
    for (path , _metadata) in &replica_files {
        if !source_files.contains_key(path){
            filestodelete.push(path.clone());
        }
    }



    if  filestodelete.is_empty(){
        println!(" no files to delete "); 

    }
    else {
        println!("files to delete from replcia :"); 
        for path in &filestodelete {
            println!(" -{:?}", path );
        }
    }

    if !filestodelete.is_empty() {
        println!("executing delete "); 
        for relative_path in &filestodelete {
            let replica_path = args.replica.join(relative_path); 

            match fs::remove_file(&replica_path){
                Ok(_) => println!("deleted {:?}", replica_path), 
                Err(e) => eprintln!("error handling {:?}:{}", replica_path , e ), 

            }
        }
    }


    Ok(())
}


















