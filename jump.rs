use crate::vcs_state_mamager::{Repo, Branch, Commit};


use std::fs;


use crate::PathBuf;
use std::fs::{File};

/// Checks is it able to jump into directory.
pub fn check()->(Vec<PathBuf>, Vec<PathBuf>){
    
    let file = File::open(PathBuf::from("./src/branch_list.json")).unwrap();
    let file_2 = File::open(PathBuf::from("./src/branch_list.json")).unwrap();
    let reader = std::io::BufReader::new(file);
    let reader_2 = std::io::BufReader::new(file_2);
    let mut repoz: Repo = serde_json::from_reader(reader).unwrap();
    let copy_repoz: Repo = serde_json::from_reader(reader_2).unwrap();

    let cur_commit = repoz.count;
    let curr_branch = repoz.count_braches;
    let path = &repoz.path;
    



    
    let path2 = path.clone();
    let a = crate::commands::init::watch(path2, &mut repoz.all);
    let branches = copy_repoz.branches;
    let mut br = Branch{name: 0, called: ("m").to_string(), commits: Vec::new(), last: 0, cur: 0};
    for u in branches{
        if u.name == repoz.current_branch{
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
    return (added, modified);

}

/// Jumps into commit and prints information if we have jumped or not.
pub fn jump_to_commit(name: String)->std::io::Result<()>{
    
    let file = File::open(PathBuf::from("./src/branch_list.json")).unwrap();
    let file_2 = File::open(PathBuf::from("./src/branch_list.json")).unwrap();
    let reader = std::io::BufReader::new(file);
    let reader_2 = std::io::BufReader::new(file_2);
    let mut repoz: Repo = serde_json::from_reader(reader).unwrap();
    let copy_repoz: Repo = serde_json::from_reader(reader_2).unwrap();
    let mass = check();
    let  added = mass.0;
    
    let modified = mass.1;
    let path = copy_repoz.path;
    
    let helppath = path.clone();
    if added.is_empty() && modified.is_empty(){
        let mut help = 0;
        for i in 0..=repoz.count{
            if name == i.to_string(){
                help = 1;
            }
        }
        if help == 0{
            println!("{} {} {}", "No commit with hash", name, "exists.");
            println!("Aborting...");
        }
        else {
            let mut comm = Commit{name: 0, files: Vec::new(), 
                time: ("time").to_string(), modified:Vec::new(), added: Vec::new(), message: ("Hi").to_string()};
            let mut namebr = "ttt".to_string(); 
            for br in copy_repoz.branches{
                for com in br.commits{
                    if name == com.name.to_string(){

                        repoz.current = com.name;
                        repoz.current_branch = br.name;
                        comm = com;
                        namebr = br.called;
                        break;
                    }
                }
            }
            for entry in fs::read_dir(PathBuf::from(path.into_os_string().into_string().unwrap().to_string()))? {
                let dir = entry?;
                let copy_path = helppath.clone();
                


                if dir.file_name().into_string().unwrap().to_string() != (".vcs").to_string(){
                    fs::remove_file(PathBuf::from(copy_path.into_os_string().into_string().unwrap().to_string() + "/" + &(dir.file_name().into_string().unwrap().to_string())));
                }
            }
            let cur_branch = repoz.current_branch;
            let cur_commit = repoz.current;
            for u in &comm.files{
                let copy_path1 = helppath.clone();
                let copy_path2 = helppath.clone();
                let copy_path3 = helppath.clone();
                let mut file = File::create(PathBuf::from(copy_path1.into_os_string().into_string().unwrap().to_string()
                +"/" + &(*u)));
                fs::copy(PathBuf::from(copy_path2.into_os_string().into_string().unwrap().to_string()
                +"/" + ".vcs" + "/" + &(cur_branch.to_string()) + "/" + &(cur_commit.to_string()) + "/" + &(*u)),
                 PathBuf::from(copy_path3.into_os_string().into_string().unwrap().to_string()
                +"/" + &(*u)));


            }

    
            let repoz_string = serde_json::to_string(&repoz)?;
            std::fs::write(
            PathBuf::from("./src/branch_list.json"),
        repoz_string,
            )?;
            println!("{} {} {} {}", "Successfully jumped to commit", repoz.current.to_string(),
             ". Current branch:", namebr);
        }
        
        

    }
    else {
        println!("error: Your local changes to the following files should be commited or dropped:");
        for u in modified{
            println!("{}", u.into_os_string().into_string().unwrap());
        }
        for u in added{
            println!("{}", u.into_os_string().into_string().unwrap());
        }
        println!("Please commit your changes or drop them before you jump.");
        println!("Aborying...");
    }



    Ok(())

}


/// Jumps into last commit of branch and prints information if we have jumped or not.
pub fn jump_to_branch(name: String)->std::io::Result<()>{
    
    let file = File::open(PathBuf::from("./src/branch_list.json")).unwrap();
    let file_2 = File::open(PathBuf::from("./src/branch_list.json")).unwrap();
    let reader = std::io::BufReader::new(file);
    let reader_2 = std::io::BufReader::new(file_2);
    let mut repoz: Repo = serde_json::from_reader(reader).unwrap();
    let copy_repoz: Repo = serde_json::from_reader(reader_2).unwrap();
    let mass = check();
    let added = mass.0;
    let path = copy_repoz.path;
    
    let helppath = path.clone();
    
    let modified = mass.1;
    if added.is_empty() && modified.is_empty(){
        let mut help = 0;
        for br in &repoz.branches{
            if name == *br.called{
                help = 1;
            }
        }
        if help == 0{
            println!("{} {} {}", "No branch", name, "exists.");
            println!("Aborting...");
        }
        else {
            let mut comm = &Commit{name: 0, files: Vec::new(), 
                time: ("time").to_string(), modified:Vec::new(), added: Vec::new(), message: ("Hi").to_string()};
            let mut brr = Branch{name: 0, called:("master").to_string(), commits: Vec::new(), last: 0, cur: 0};
            for br in copy_repoz.branches{
                if name == br.called{
                    repoz.current = br.last;
                    repoz.current_branch = br.name;
                    brr = br;

                    break;
                }
            }
            comm = brr.commits.last().unwrap();
            for entry in fs::read_dir(PathBuf::from(path.into_os_string().into_string().unwrap().to_string()))? {
                let dir = entry?;
                let copy_path = helppath.clone();
                


                if dir.file_name().into_string().unwrap().to_string() != (".vcs").to_string(){
                    fs::remove_file(PathBuf::from(copy_path.into_os_string().into_string().unwrap().to_string() + "/" + &(dir.file_name().into_string().unwrap().to_string())));
                }
            }
            let cur_branch = repoz.current_branch;
            let cur_commit = repoz.current;
            for u in &comm.files{
                let copy_path1 = helppath.clone();
                let copy_path2 = helppath.clone();
                let copy_path3 = helppath.clone();
                let mut file = File::create(PathBuf::from(copy_path1.into_os_string().into_string().unwrap().to_string()
                +"/" + &(*u)));
                fs::copy(PathBuf::from(copy_path2.into_os_string().into_string().unwrap().to_string()
                +"/" + ".vcs" + "/" + &(cur_branch.to_string()) + "/" + &(cur_commit.to_string()) + "/" + &(*u)),
                 PathBuf::from(copy_path3.into_os_string().into_string().unwrap().to_string()
                +"/" + &(*u)));


            }
            let repoz_string = serde_json::to_string(&repoz)?;
            std::fs::write(
            PathBuf::from("./src/branch_list.json"),
        repoz_string,
            )?;
            println!("{} {} {} {}", "Successfully jumped to branch", name,
             ". Current commit:", repoz.current.to_string());
        }
        
        

    }
    else {
        println!("error: Your local changes to the following files should be commited or dropped:");
        for u in modified{
            println!("{} {}","modified: ", u.into_os_string().into_string().unwrap());
        }
        for u in added{
            println!("{} {}", "added: ", u.into_os_string().into_string().unwrap());
        }
        println!("Please commit your changes or drop them before you jump.");
        println!("Aborying...");
    }


    Ok(())
}



