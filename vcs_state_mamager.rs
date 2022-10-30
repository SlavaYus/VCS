use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Commit{
    /// Commit consists information about: name, files, time of commit,
    /// added and modified files and message
    pub name: u64,
    pub files: Vec<String>,
    pub time: String,
    pub modified: Vec<PathBuf>,
    pub added: Vec<PathBuf>,
    pub message: String,

}
#[derive(Serialize, Deserialize)]
pub struct Branch{
    /// Branch consists information about: hash, name, commits, current commit and last commit
    pub name: u64,
    pub called: String,
    pub commits: Vec<Commit>,
    pub last: u64,
    pub cur: u64,
    
}
#[derive(Serialize, Deserialize)]
pub struct Repo{
    /// Repo consists information about: path to directory, number of commits и branches, current commit and current branch, array of files in repozitory, 
    /// pathes to all files and vector of branches
    pub path: PathBuf,
    pub count: u64,
    pub all: Vec<String>,
    pub all_path: Vec<PathBuf>,
    pub current: u64,
    pub count_braches: u64,
    pub current_branch: u64,
    pub branches: Vec<Branch>,
    


}

