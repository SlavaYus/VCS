
use crate::vcs_state_mamager::{Repo, Branch, Commit};


use std::fs;
use chrono::{Utc};

use crate::PathBuf;
use std::fs::{File};

use std::fs::DirBuilder;

/// Walks around directory and watchesonly files, does not goe into subdirectories
pub fn watch(path: PathBuf, mut repoz: &mut Vec<String>)-> std::io::Result<()>{
    
    repoz.clear();

    for entry in fs::read_dir(PathBuf::from(path.into_os_string().into_string().unwrap().to_string()
    +""))? {
        let dir = entry?;
       


        repoz.push(dir.file_name().into_string().unwrap().to_string());
    }
    
    Ok(())
}
///Makes Init
/// Creates directory .vcs, and master branch called 0 and a first commit in it called 0.
/// fills .json file about repozitory
pub fn init(path: PathBuf, mut repoz: Repo)->std::io::Result<()>{
    
    let copy_path0 = path.clone();
    let prpath = path.clone();
    


    let a = watch(copy_path0, &mut repoz.all);
   
    let b = match a{
        Ok(())=> "Ok",
        Err(_)=> "Error, not such directory!!!!!!",
    };
    
    let help = repoz.count_braches.to_string();
    let help_com = repoz.count.to_string();
    let helppath = path.clone();
    
    let mut br = Branch{name: 0, called:("master").to_string(), commits: Vec::new(), last: 0, cur: 0};
    let mut com = Commit{name: 0, files: Vec::new(),
         time: ("time").to_string(), modified:Vec::new(), added: Vec::new(),
          message:("Your first commit!").to_string()};
    
    
    let now = (Utc::now()).to_string();
    com.time = now;
    

    let double_path = path.clone();
    let triple_path = path.clone();
    DirBuilder::new()
    .recursive(true)
    .create(PathBuf::from(path.into_os_string().into_string().unwrap().to_string()
    +"/"+".vcs")).unwrap();

    DirBuilder::new()
    .recursive(true)
    .create(PathBuf::from(triple_path.into_os_string().into_string().unwrap().to_string()
    +"/"+".vcs" + "/" + &help)).unwrap();
    DirBuilder::new()
    .recursive(true)
    .create(PathBuf::from(double_path.into_os_string().into_string().unwrap().to_string()
    +"/"+ ".vcs" + "/" +&help+"/"+&help_com)).unwrap();

    
    for u in &repoz.all{
        let copy_path1 = helppath.clone();
        
        let mut is_num = 0;
        for i in 0..100{
            if *u == i.to_string(){
                is_num = 1;
            }
        }
        if is_num == 0{
            
            let file = File::create(PathBuf::from(copy_path1.into_os_string().into_string().unwrap().to_string()
            +"/" + ".vcs" + "/" + &help + "/" + &help_com + "/" + &(*u)));
            
            

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
            let o = (*u).clone();
            fs::copy(PathBuf::from(copy_path1.into_os_string().into_string().unwrap().to_string()
            +"/" + &(*u)), PathBuf::from(copy_path2.into_os_string().into_string().unwrap().to_string()
            +"/"+ ".vcs" + "/" + &help + "/" + &help_com + "/" + &(*u)));
            com.files.push(o);
        

        }
        
        
    }
    
    br.commits.push(com);
    br.last = 0;
    br.cur = 0;
    repoz.branches.push(br);
    let repoz_string = serde_json::to_string(&repoz)?;

    
    std::fs::write(
        PathBuf::from("./src/branch_list.json"),
        repoz_string,
    )?;
    println!("{} {}", "Initialized VCS repository in ",
     prpath.into_os_string().into_string().unwrap().to_string());
    println!("Created commit:");
    println!("[master 0] Initial commit");
    

    

    
    Ok(())



}