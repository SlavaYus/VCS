
use crate::vcs_state_mamager::{Repo, Branch};
use std::fs;

use crate::PathBuf;
use std::fs::{File};

pub fn is_num(a:&String)->bool{
    /// checks is String a whole number
    let mut is = 0;
    for u in 0..1000{
        if *a == u.to_string(){
            is = 1;
        }
    }
    if is == 1{
        false
    }
    else{
        true
    }

}
/// 1) Prints status 
/// 2) Prints information about commit
pub fn status(indicator: i32)->std::io::Result<()>{
    
    let mut file = File::open(PathBuf::from("./src/branch_list.json")).unwrap();
    let mut file_2 = File::open(PathBuf::from("./src/branch_list.json")).unwrap();
    let reader = std::io::BufReader::new(file);
    let reader_2 = std::io::BufReader::new(file_2);
    let mut repoz: Repo = serde_json::from_reader(reader).unwrap();
    let mut copy_repoz: Repo = serde_json::from_reader(reader_2).unwrap();

    let cur_commit = repoz.count;
    let curr_branch = repoz.count_braches;
    let path = &repoz.path;
    



    let double_path = path.clone();
    let path2 = path.clone();
    let a = crate::commands::init::watch(path2, &mut repoz.all);
    let branches = copy_repoz.branches;
    let mut br = Branch{name: 0, called: ("m").to_string(), commits: Vec::new(), last: 0, cur: 0};
    for u in branches{
        if u.name == curr_branch{
            br = u;
            break;
        }
    }
    let mut modified: Vec<PathBuf> = Vec::new();
    let mut added:Vec<PathBuf> = Vec::new();
    
    for u in &repoz.all{
        let copy_path1 = path.clone();
        let copy_path2 = path.clone();
        let copy_path3 = path.clone();
        let copy_path4 = path.clone();
        let mut help = 0;
        for files in &(br.commits[br.commits.len() - 1]).files{
            if *u == *files{
                help = 1;

            }

        }

        if is_num(u){
            
            if help == 0{
                if u != ".vcs"{
                    added.push(PathBuf::from(copy_path3.into_os_string().into_string().unwrap().to_string()
                    +"/" + &(*u)))

                }
                
            }
            else{
                if u != ".vcs"{
                    let a = fs::read_to_string(PathBuf::from(copy_path1.into_os_string().into_string().unwrap().to_string()
                    +"/" + &(*u)))
                    .expect("Should have been able to read the file");
                    let b = fs::read_to_string(PathBuf::from(copy_path2.into_os_string().into_string().unwrap().to_string()
                    +"/"+ ".vcs" + "/" + &(curr_branch.to_string()) + "/" + &(cur_commit.to_string()) + "/" + &(*u)))
                    .expect("Should have been able to read the file");
                    if a != b{
                        modified.push(PathBuf::from(copy_path4.into_os_string().into_string().unwrap().to_string()
                        +"/" + &(*u)));
                    }

                }
                
              
    
            }

        }
        
        
    }
    if indicator == 0{
        if added.is_empty() && modified.is_empty(){
            println!("No changes to be committed");
        }
        else{
            println!("{} {}", "On branch", br.called);
            println!("Changes to be commited:");
            for u in modified{
                println!("{} {}", "   modified: ", u.into_os_string().into_string().unwrap());
            }
            for u in added{
                println!("{} {}", "   added: ", u.into_os_string().into_string().unwrap());
            }
        }

    }
    else if indicator == 1{
        if added.is_empty() && modified.is_empty(){
            println!("No changes to be committed");
        }
        else{
            println!("{} {} {} {} {}", "[", br.called, ",", cur_commit.to_string(), "]Work in progress:");
            
            println!("{} {} {} {}", (modified.len() + added.len()).to_string(), " files changed,", 
        (added.len()).to_string(), "added");
            for u in modified{
                println!("{} {}", "   modified: ", u.into_os_string().into_string().unwrap());
            }
            for u in added{
                println!("{} {}", "   added: ", u.into_os_string().into_string().unwrap());
            }
        }
    }
    
    



    



    
    Ok(())



}