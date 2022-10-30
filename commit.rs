use crate::vcs_state_mamager::{Repo, Branch, Commit};


use std::fs;
use chrono::{Utc};

use crate::PathBuf;
use std::fs::{File};

use std::fs::DirBuilder;

/// Findes which files was modified or added.
/// modified files in '''let mut modified: Vec<PathBuf> = Vec::new();'''
/// added in '''let mut added: Vec<PathBuf> = Vec::new();'''
pub fn modif(com: &mut Commit){
    
    let file = File::open(PathBuf::from("./src/branch_list.json")).unwrap();
    let file_2 = File::open(PathBuf::from("./src/branch_list.json")).unwrap();
    let reader = std::io::BufReader::new(file);
    let reader_2 = std::io::BufReader::new(file_2);
    let mut repoz: Repo = serde_json::from_reader(reader).unwrap();
    let copy_repoz: Repo = serde_json::from_reader(reader_2).unwrap();

    let cur_commit = repoz.current;
    let curr_branch = repoz.current_branch;
    let path = &repoz.path;
    



    
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
        com.files.push((*u).to_string());
        let copy_path1 = path.clone();
        let copy_path2 = path.clone();
        let copy_path3 = path.clone();
        let copy_path4 = path.clone();
        let mut help = 0;
        
        for files in &(br.commits[br.commits.len() - 1].files){
            
            if *u == *files{
                help = 1;

            }

        }

        if crate::commands::status::is_num(u){
            
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
                    +"/"+ ".vcs" +"/" + &(curr_branch.to_string()) + "/" + &(cur_commit.to_string()) + "/" + &(*u)))
                    .expect("Should have been able to read the file");
                    if a != b{
                        modified.push(PathBuf::from(copy_path4.into_os_string().into_string().unwrap().to_string()
                        +"/" + &(*u)));
                    }

                }
                
              
    
            }

        }
        
        
    }
    
    com.modified = modified;
    com.added = added;
    

}
/// Makes commit
/// creates new file in directory .vcs which called as commit
/// Changes repoz and .json file about repozitory
pub fn commit(message: String, ind: i32)->std::io::Result<()>{
    
    if (ind == 0){
        crate::commands::status::status(1);

    }
    
    


    let file = File::open(PathBuf::from("./src/branch_list.json")).unwrap();
    let file_2 = File::open(PathBuf::from("./src/branch_list.json")).unwrap();
    let reader = std::io::BufReader::new(file);
    let reader_2 = std::io::BufReader::new(file_2);
    let mut repoz: Repo = serde_json::from_reader(reader).unwrap();
    let copy_repoz: Repo = serde_json::from_reader(reader_2).unwrap();
    

    
    repoz.count += 1;
    repoz.current += 1;
    let cur_commit = repoz.count;
    let curr_branch = repoz.current_branch;
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
    if br.cur != br.last{
        println!("You can create a new commit only from last one.");
        println!("Aborying...");
        return Ok(());

    }

    
    
    let mut com = Commit{name: cur_commit, files: Vec::new(), 
        time: ("time").to_string(), modified:Vec::new(), added: Vec::new(), message: message};
    
    let b = match a{
        Ok(())=> "",
        Err(_)=> "Error, not such directory!!!!!!",
    };
    modif(&mut com);
    let now = (Utc::now()).to_string();
    com.time = now;
    
    



    DirBuilder::new()
    .recursive(true)
    .create(PathBuf::from(double_path.into_os_string().into_string().unwrap().to_string()
    +"/" + ".vcs" + "/" +&(curr_branch.to_string())+"/"+&(cur_commit.to_string()))).unwrap();
    for u in &repoz.all{
        let copy_path1 = path.clone();
        
        let mut is_num = 0;
        for i in 0..100{
            if *u == i.to_string(){
                is_num = 1;
            }
        }
        if is_num == 0{
            
            let mut file = File::create(PathBuf::from(copy_path1.into_os_string().into_string().unwrap().to_string()
            +"/"+ ".vcs" +"/" +&(curr_branch.to_string()) + "/" + &(cur_commit.to_string()) + "/" + &(*u)));
            
            

        }
        
        
    }
    
    
    for u in &repoz.all{
        let copy_path1 = path.clone();
        let copy_path2 = path.clone();
        let mut is_num = 0;
        for i in 0..100{
            if *u == i.to_string(){
                is_num = 1;
            }
        }
        if is_num == 0{
            let o = (*u).clone();
            fs::copy(PathBuf::from(copy_path1.into_os_string().into_string().unwrap().to_string()
            +"/" + &(*u)), PathBuf::from(copy_path2.into_os_string().into_string().unwrap().to_string()
            +"/"+ ".vcs" + "/" + &(curr_branch.to_string()) + "/" + &(cur_commit.to_string()) + "/" + &(*u)));
            let files_names = com.files.clone();
            let mut help = 0;
            for u in files_names{
                if o == u{
                    help = 1;
                }
            }
            if help == 0{
                com.files.push(o);

            }
            
           
        

        }
        
        
    }
    br.commits.push(com);
    br.cur = cur_commit;
    br.last = cur_commit;
    let b = repoz.current_branch as usize;
    repoz.branches[b] = br;
    
    let repoz_string = serde_json::to_string(&repoz)?;

    
    std::fs::write(
        PathBuf::from("./src/branch_list.json"),
        repoz_string,
    )?;

    
    

    

    

    
    Ok(())


    

}