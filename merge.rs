use crate::vcs_state_mamager::{Repo, Branch, Commit};


use std::fs;
use chrono::{Utc};

use crate::PathBuf;
use std::fs::{File};

use std::fs::DirBuilder;

/// Checks if string inside the vector of strings
pub fn inside(a:String, mas:&Vec<String>)->bool{
    
    let mut help = 0;
    for u in mas{
        if a == *u{
            help = 1;
        }

    }
    if help == 1{
        true
    }
    else{
        false
    }
}

/// If possible, Makes merge of master branch and current branch
/// Prints about succesfull merge or about the problems
pub fn merge(branch_name: String)->std::io::Result<()>{
    
    let file = File::open(PathBuf::from("./src/branch_list.json")).unwrap();
    let file_2 = File::open(PathBuf::from("./src/branch_list.json")).unwrap();
    let file_3 = File::open(PathBuf::from("./src/branch_list.json")).unwrap();
    let reader = std::io::BufReader::new(file);
    let reader_2 = std::io::BufReader::new(file_2);
    let reader_3 = std::io::BufReader::new(file_3);
    let mut repoz: Repo = serde_json::from_reader(reader).unwrap();
    let copy_repoz: Repo = serde_json::from_reader(reader_2).unwrap();
    let copy_repoz_2: Repo = serde_json::from_reader(reader_3).unwrap();
    let path = repoz.path;
    let double_path = path.clone();
    if repoz.current_branch != 0{
        println!("The merge is possible only when you are in the last commit in master.");
        println!("Aborting...");
    }
    else{
        let mut master = Branch{name: 0, called: ("m").to_string(),
         commits: Vec::new(), last: 0, cur: 0};
        let mut br = Branch{name: 0, called: ("m").to_string(),
         commits: Vec::new(), last: 0, cur: 0};
        for u in copy_repoz.branches{
            
            if u.name == 0{
                master = u;
                
            }
            
        }
        for u in copy_repoz_2.branches{
            if u.called == branch_name{
                br = u;
                
            }
            
            
        }
        
        if repoz.current != master.commits[master.commits.len() - 1].name{
            println!("The merge is possible only when you are in the last commit in master.");
            println!("Aborting...");

        }
        else{
            let mut all: Vec<String> = Vec::new();
            let mut added:Vec<PathBuf> = Vec::new();
            let mut allpath: Vec<PathBuf> = Vec::new();
            let mut modified: Vec<PathBuf> = Vec::new();
            let mut problems: Vec<PathBuf> = Vec::new();
            for u in &br.commits[0].files{
                if inside((*u).to_string(), &(master.commits[master.commits.len() - 1].files)){
                    let copy_path1 = path.clone();
                    let copy_path2 = path.clone();
                    let copy_path3 = path.clone();
                    let copy_path4 = path.clone();
                    let copy_path5 = path.clone();
                    println!("{}", (PathBuf::from(copy_path5.into_os_string().into_string().unwrap().to_string()
                    +"/"+ ".vcs" + "/" + &(br.name.to_string()) + "/" + &(br.commits[1].name.to_string()) + "/" + &(*u))).into_os_string().into_string().unwrap().to_string() );
                    let a = fs::read_to_string(PathBuf::from(copy_path1.into_os_string().into_string().unwrap().to_string()
                    +"/"+ ".vcs" + "/" + &(br.name.to_string()) + "/" + &(br.commits[1].name.to_string()) + "/" + &(*u)))
                    .expect("Should have been able to read the file");
                    let b = fs::read_to_string(PathBuf::from(copy_path2.into_os_string().into_string().unwrap().to_string()
                    +"/"+ ".vcs" + "/" + "0" + "/" + &(master.commits[master.commits.len() - 1].name.to_string()) + "/" + &(*u)))
                    .expect("Should have been able to read the file");
                    if a != b{
                        problems.push(PathBuf::from(copy_path3.into_os_string().into_string().unwrap().to_string()
                        +"/"+ ".vcs" + "/" + &(br.name.to_string()) + "/" + &(br.commits[1].name.to_string()) + "/" + &(*u)));
                        problems.push(PathBuf::from(copy_path4.into_os_string().into_string().unwrap().to_string()
                        +"/"+ ".vcs" + "/" + "0" + "/" + &(master.commits[master.commits.len() - 1].name.to_string()) + "/" + &(*u)));
                    }
                }

            }


            






            for u in &master.commits[master.commits.len() - 1].files{
                let copy_path = path.clone();
                if inside((*u).to_string(), &(br.commits[br.commits.len() - 1].files)){
                    let copy_path1 = path.clone();
                    let copy_path2 = path.clone();
                    let copy_path3 = path.clone();
                    let a = fs::read_to_string(PathBuf::from(copy_path1.into_os_string().into_string().unwrap().to_string()
                    +"/"+ ".vcs" + "/" + &(br.name.to_string()) + "/" + &(br.commits[br.commits.len() - 1].name.to_string()) + "/" + &(*u)))
                    .expect("Should have been able to read the file");
                    let b = fs::read_to_string(PathBuf::from(copy_path2.into_os_string().into_string().unwrap().to_string()
                    +"/"+ ".vcs" + "/" + "0" + "/" + &(master.commits[master.commits.len() - 1].name.to_string()) + "/" + &(*u)))
                    .expect("Should have been able to read the file");
                    if a != b{
                        
                        modified.push(PathBuf::from(copy_path3.into_os_string().into_string().unwrap().to_string()
                        +"/"+ ".vcs" + "/" + "0" + "/" + &(master.commits[master.commits.len() - 1].name.to_string()) + "/" + &(*u)));
                    }
                }
                all.push((*u).to_string());
                allpath.push(PathBuf::from(copy_path.into_os_string().into_string().unwrap().to_string()
                +"/"+ ".vcs" + "/" + "0" + "/" + &(master.commits[master.commits.len() - 1].name.to_string()) + "/" + &(*u)));
                

            }

            for u in &br.commits[br.commits.len() - 1].files{
                if !inside((*u).to_string(), &(master.commits[master.commits.len() - 1].files)){
                    let copy_path1 = path.clone();
                    
                    added.push(PathBuf::from(copy_path1.into_os_string().into_string().unwrap().to_string()
                    +"/"+ ".vcs" + "/" + &(br.name.to_string()) + "/" +
                     &(br.commits[br.commits.len() - 1].name.to_string()) + "/" + &(*u)));
                    all.push((*u).to_string());

                }
                

            }
            let added2 = added.clone();
            let added3 = added.clone();
            let added4 = added.clone();
            
            
            for u in added2{
                allpath.push(u);
            }

            if problems.is_empty(){
                let mut com = Commit{name: 0, files: Vec::new(), 
                    time: ("time").to_string(), modified:Vec::new(), added: Vec::new(), message: ("Merged").to_string()};
                let now = (Utc::now()).to_string();
                com.time = now;
                repoz.current_branch = 0;
                repoz.count += 1;
                repoz.current = repoz.count;
                com.name = repoz.current;
                
                DirBuilder::new()
                .recursive(true)
                .create(PathBuf::from(double_path.into_os_string().into_string().unwrap().to_string()
                +"/" + ".vcs" + "/" +"0"+"/"+&(repoz.count.to_string()))).unwrap();
                let mut i:usize = 0;
                for file in &all{
                    let copy_path5 = path.clone();
                    let copy_path6 = path.clone();
                    let copy_path7 = allpath[i].clone();
                    File::create(PathBuf::from(copy_path5.into_os_string().into_string().unwrap().to_string()
                    +"/"+ ".vcs" +"/" +"0" + "/" + &&(repoz.count.to_string()) + "/" + &(*file)));
                    fs::copy(copy_path7, PathBuf::from(copy_path6.into_os_string().into_string().unwrap().to_string()
                    +"/"+ ".vcs" +"/" +"0" + "/" + &&(repoz.count.to_string()) + "/" + &(*file)));
                    i += 1;

                }
                let added2 = added.clone();
                let modified2 = modified.clone();

                com.files = all;
                com.modified = modified;
                com.added = added;
                repoz.branches[0].commits.push(com);
                repoz.branches[0].last = repoz.current;
                repoz.branches[0].cur = repoz.current;
                println!("Successfully created merge commit:");
                println!("{} {} {} {}", "[master ", repoz.current, " Merged branch", branch_name);
                println!("{} {} {} {}", (added3.len() + modified2.len()).to_string(), " files modified, ", 
            (added4.len()).to_string(), " added");
                for u in modified2{
                    println!("{} {}", "    modified: ", u.into_os_string().into_string().unwrap());
                }
                for u in added2{
                    println!("{} {}", "    added: ", u.into_os_string().into_string().unwrap());
                }
                println!("{} {}", "Deleted ", branch_name);
                
                


                
















            }
            else {
                println!("Merge confilict: file has been changed both in master and branch");
                for u in problems{
                    println!("{}", u.into_os_string().into_string().unwrap().to_string());
                }
                println!("Aborting...");
            }
            

            
            
            

        }

    }


    Ok(())
}