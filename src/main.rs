use std::io::{stdin, Read, Cursor, stdout, Write, Stdin};
use std::path::{Path, PathBuf, self};
use std::fs;
use diffy::{Patch, apply_bytes};
use sha256::digest;
use reqwest::blocking::{self, Client};

static gwogloader_endpoint: &str = "https://gwogloader.dev";

fn main() {
    let mut the_stdin: std::io::Stdin = stdin();
    let mut the_stdout: std::io::Stdout = stdout();

    println!("Welcome to the GwogLoader Installer!");
    println!("You only need to run this when Wormtown updates.");
    println!("To uninstall, right-click on Wormtown in steam -> properties -> installed files -> verify game files.");
    println!("To exit, press [CTRL]+[C], or close the window");
    println!("If you want to continue with the install, press [ENTER]");
    the_stdin.read_line(&mut String::from("")).unwrap();

    let paths: Vec<&Path> = vec![Path::new("Last Train Out Of WormTown_Data\\Managed"),Path::new("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Last Train Outta' Wormtown\\Last Train Out Of WormTown_Data\\Managed"), Path::new("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Last Train Outta' Wormtown Demo\\Last Train Out Of WormTown_Data\\Managed")];
    
    let mut i = 0;

    let mut max = paths.len();

    for path in paths{
        if(path.exists()){
            do_patch(path.display().to_string(), &the_stdin);
            break;
        }
        else{
            i = i + 1;
        }
    }

    if(i == max){
        println!("Couldn't locate your install of Wormtown! To fix this:");
        println!("  - Ensure you actually have Wormtown installed!");
        println!("  - Place this exe into your Wormtown folder, so that it is right next to the \"Last Train Out Of WormTown.exe\" file, then run it again.");
        println!("Press [ENTER] to close the window.");
        the_stdin.read_line(&mut String::from("")).unwrap();
        return;
    }
}

fn do_patch(path_to_managed: String, the_stdin: &Stdin){
    println!("  - Located the Wormtown game!");
    println!("  - Loading binary...");
    let ascBytes: Vec<u8> = fs::read(path_to_managed.clone()+"\\Assembly-CSharp.dll").unwrap();
    let hash = digest(&ascBytes);
    println!("  - Hash of Assembly-CSharp is {}", hash);
    println!("  - Downloading the latest build of GwogLoader...");
    let client: Client = Client::new();
    let maybe_data = client.get(gwogloader_endpoint.to_owned() + "/"+&hash+".zip").send();
    if(maybe_data.is_err()){
        println!("Failed to connect to GwogLoader's installation server, please check the following:");
        println!("  - Is your internet connection working?");
        println!("  - Is something blocking {}?", gwogloader_endpoint);
        println!("Press [ENTER] to close the window.");
        the_stdin.read_line(&mut String::from("")).unwrap();
        return;
    }
    let bytes = maybe_data.unwrap().bytes().unwrap();
    if(!std::str::from_utf8(&bytes).is_err()){
        if(String::from(std::str::from_utf8(&bytes).unwrap()).contains("404 Not Found")){
            println!("Couldn't find a version of GwogLoader for your version of Wormtown, please check the following:");
            println!("  - Is your copy of Wormtown already modded?");
            println!("      - If you're using GwogLoader, you don't need to do anything. GwogLoader will automatically update when you open the game!");
            println!("      - If you're using another mod/modloader, you will need to manually patch the GwogLoader Bootstrap into it. Check the GwogLoader github for more information.");
            println!("  - Did Wormtown JUST update? If so, ping @systemdev (nickname Gwog <3) in the Wormtown discord, and tell him to get to work!");
            println!("Press [ENTER] to close the window.");
            the_stdin.read_line(&mut String::from("")).unwrap();
            return;
        }
    }
    
    let target = PathBuf::from(path_to_managed.clone());
    
    println!("  - Extracting dependencies...");
    zip_extract::extract(Cursor::new(bytes), &target, true);
    println!("  - Reading patch...");
    let patch_bytes = std::fs::read(Path::new(&(path_to_managed.clone()+"\\Assembly-CSharp.patch"))).unwrap();
    let patch = Patch::from_bytes(&patch_bytes.as_slice()).unwrap();
    println!("  - Applying patch...");
    let patched_bytes = apply_bytes(&ascBytes, &patch).unwrap();
    println!("  - Writing patched Assembly-CSharp to disk...");
    std::fs::write(Path::new(&(path_to_managed.clone()+"\\Assembly-CSharp.dll")), patched_bytes);
    println!("DONE!");
    println!("To uninstall, verify your game files through Steam.");
    println!("Happy worming!");
    println!("Press [ENTER] to close the window.");
    the_stdin.read_line(&mut String::from("")).unwrap();
}