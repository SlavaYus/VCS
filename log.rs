use crate::vcs_state_mamager::{Repo};





use crate::PathBuf;
use std::fs::{File};

/// Prints log
pub fn logo(){
    
    let file = File::open(PathBuf::from("./src/branch_list.json")).unwrap();
    
    let reader = std::io::BufReader::new(file);
    
    let repoz: Repo = serde_json::from_reader(reader).unwrap();
    
    let path = repoz.path;
    for bran in repoz.branches{
        let oo = bran.commits[0].name;
        for com in bran.commits{
            
            if com.name == 0{
                println!("{} {}", "Commit: ", com.name);
                println!("{} {}", "Date: ", com.time);
                println!("{} {}", "Message: ", com.message);
            
                println!("{}", "Changes:");
                
                for u in com.files{
                    let copy_path4 = path.clone();
                    println!("{} {}", "    added: ", PathBuf::from(copy_path4.into_os_string().into_string().unwrap().to_string()
                    +"/" + &(u)).into_os_string().into_string().unwrap());
                }

            }
            else if (bran.name != 0) && (com.name == oo){

            }
            else{
                println!("{} {}", "Commit: ", com.name);
                println!("{} {}", "Date: ", com.time);
                println!("{} {}", "Message: ", com.message);
            
                println!("{}", "Changes:");
                if com.modified.is_empty() && com.added.is_empty(){
                    println!("    No changes")

                }
                for u in com.modified{
                    println!("{} {}", "    modified: ", u.into_os_string().into_string().unwrap());
                }
                for u in com.added{
                    println!("{} {}", "    added: ", u.into_os_string().into_string().unwrap());
                }
                println!("    ");

            }
            

        }

    }
}