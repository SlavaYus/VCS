use crate::vcs_state_mamager::{Repo, Branch, Commit};


use std::fs;
use chrono::{Utc};

use crate::PathBuf;
use std::fs::{File};

use std::fs::DirBuilder;

/// Makes new branch if it is possible
/// Prints information about succsesful creating of new brach aor about the problems
pub fn new_branch(branch_name: String)->std::io::Result<()>{
    
    let file = File::open(PathBuf::from("./src/branch_list.json")).unwrap();
    let file_2 = File::open(PathBuf::from("./src/branch_list.json")).unwrap();
    let reader = std::io::BufReader::new(file);
    let reader_2 = std::io::BufReader::new(file_2);
    let mut repoz: Repo = serde_json::from_reader(reader).unwrap();
    let copy_repoz: Repo = serde_json::from_reader(reader_2).unwrap();
    if repoz.current_branch != 0{
        println!("Creating a new branch is possible only when you are in the master branch.");
        println!("Aborting...");
        Ok(())
    }
    else {
        let mut is_name = false;
        let cur_br = repoz.current_branch;
        let cur_name = repoz.current;
        let mut master = Branch{name: cur_br,
            called:("master").to_string(), commits: Vec::new(), last: 0, cur: 0};
        for u in copy_repoz.branches{
            if u.called == branch_name{
                is_name = true;
            }
            if u.name == 0{
                master = u;
            } 
        }
        if is_name{
            println!("Branch branch_name already exists.");
            println!("Aborting...");
        }
        else{
            println!("{} {} {} {}", "Created a new branch", branch_name, "from master's commit", repoz.current.to_string());
            
            repoz.count_braches += 1;
            let cur_branch = repoz.count_braches;
            let mut br = Branch{name: cur_name,
                called: branch_name, commits: Vec::new(), last: 0, cur: 0};
            
            for com in master.commits{
                if com.name != cur_name{
                    
                    
                }
                else{
                    
                    br.last = com.name;
                    br.cur = com.name;
                    br.commits.push(com);
                    break;

                }

            }
            
            repoz.current_branch = repoz.count_braches;
            
            let path = copy_repoz.path;
            let double_path = path.clone();
            let helppath = path.clone();
            
           
            
            let now = (Utc::now()).to_string();
            
            repoz.count += 1;
            repoz.current = repoz.count;
            let mut com = Commit{name: repoz.current, files: Vec::new(),
                time: ("time").to_string(), modified:Vec::new(), added: Vec::new(),
                 message:("Your first commit!").to_string()};
            com.time = now;



            DirBuilder::new()
                .recursive(true)
                .create(PathBuf::from(path.into_os_string().into_string().unwrap().to_string()
                +"/" + ".vcs" +"/"+&(cur_branch).to_string())).unwrap();
            DirBuilder::new()
            .recursive(true)
            .create(PathBuf::from(double_path.into_os_string().into_string().unwrap().to_string()
            +"/"+ ".vcs" + "/" + &(cur_branch).to_string()+"/"+&(repoz.current).to_string())).unwrap();
            for u in &repoz.all{
                let copy_path = helppath.clone();
                
                let mut is_num = 0;
                for i in 0..1000{
                    if *u == i.to_string(){
                        is_num = 1;
                    }
                }
                if is_num == 0{
                    
                    let file = File::create(PathBuf::from(copy_path.into_os_string().into_string().unwrap().to_string()
                    +"/"+ ".vcs" + "/" + &(cur_branch).to_string() + "/" + &(repoz.current).to_string() + "/" + &(*u))); 
        
                }
            }
            for u in &repoz.all{
                let copy_path1 = helppath.clone();
                let copy_path2 = helppath.clone();
                let mut is_num = 0;
                for i in 0..100{
                    if *u == i.to_string(){
                        is_num = 1;
                    }
                }
                if is_num == 0{
                    let text = (*u).clone();
                    fs::copy(PathBuf::from(copy_path1.into_os_string().into_string().unwrap().to_string()
                    +"/" + &(*u)), PathBuf::from(copy_path2.into_os_string().into_string().unwrap().to_string()
                    +"/" + ".vcs" + "/" + &(cur_branch).to_string() + "/" + &(repoz.current).to_string() + "/" + &(*u)));
                    com.files.push(text);
                
        
                }
                
                
            }
            br.commits.push(com);
            br.name = repoz.current_branch;
            br.last = repoz.current;
            br.cur = repoz.current;
            repoz.branches.push(br);
            let repoz_string = serde_json::to_string(&repoz)?;

    
            std::fs::write(
              PathBuf::from("./src/branch_list.json"),
              repoz_string,
            )?;

            





            
            let repoz_string = serde_json::to_string(&repoz)?;
            std::fs::write(
                PathBuf::from("./src/branch_list.json"),
                repoz_string,
            )?;
            
            

            
            
        }
        Ok(())
    }
}