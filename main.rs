use clap::{Parser};
use std::path::PathBuf;
use std::ffi::OsString;
pub mod command_parser;
pub mod vcs_state_mamager;
pub mod tests{pub mod tests;}
pub mod commands{pub mod init; pub mod log; pub mod commit; pub mod status;
     pub mod new_branch; pub mod jump; pub mod merge; pub mod help;}



#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    subcom: String,
    
    #[arg(long, default_value_t = ("").to_string())]
    path: String,
    #[arg(long, default_value_t = ("").to_string())]
    message: String,
    #[arg(long, default_value_t = ("").to_string())]
    commit: String,
    #[arg(long, default_value_t = ("").to_string())]
    branch: String,
    #[arg(long, default_value_t = ("").to_string())]
    name: String,

}

fn main() {
    let args = Args::parse();
    
    if args.subcom == "init"{
        let mut repoz = crate::vcs_state_mamager::Repo{path: PathBuf::from(&args.path),
             count: 0, all: Vec::new(), all_path: Vec::new(), current: 0, count_braches: 0,
             current_branch: 0, branches: Vec::new()};
        crate::commands::init::init(PathBuf::from(&args.path), repoz);
        
    }

    
    if args.subcom == "status"{
        crate::commands::status::status(0);
    }
    else if args.subcom == "commit"{ 
        
        let u = PathBuf::from(OsString::from(&args.path));
        println!("{:?}", args.message);
        crate::commands::commit::commit(args.message, 0);
        
        
    }
    else if args.subcom == "jump"{
        if args.commit != ""{
            crate::commands::jump::jump_to_commit(args.commit); 
        }
        else if args.branch != ""{
            crate::commands::jump::jump_to_branch(args.branch);
        } 
    }
    else if args.subcom == "new_branch"{
        if args.name != ""{
            crate::commands::new_branch::new_branch(args.name);
        }
    }
    else if args.subcom == "merge"{
        if args.branch != ""{
            crate::commands::merge::merge(args.branch);
        }
    }
    else if args.subcom == "log"{
        
        crate::commands::log::logo();
        
    }
    else if args.subcom == "help"{
        crate::commands::help::help();
    }

}
