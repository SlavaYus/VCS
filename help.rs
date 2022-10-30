


pub fn help(){
    /// Creates help
    println!("Commit consists information about: name, files, time of commit,
    added and modified files and message");
    println!("Branch consists information about: hash, name, commits, current commit and last commit");
    println!("Repo consists information about: path to directory, number of commits и branches, current commit and current branch, array of files in repozitory, 
    pathes to all files and vector of branches ");
    println!("help in help - print documentation");
    println!("commit in commit- makes commit");
    println!("status in status - print information about status and commit");
    println!("init in init- inits directory");
    println!("watch in init- function which walks throw directory");
    println!("modif in commit- checks which files are added or modified");
    println!("check in jump - checks is it able to make a jump");
    println!("jump_to_commit in jump - makes jump to commit");
    println!("jump_to_branch in jump  - makes jump to branch");
    println!("new_branch in new_branch - makes new branch");
    println!("merge in merge - merges branch and master");
    println!("logo in log - prints log of operations");
}