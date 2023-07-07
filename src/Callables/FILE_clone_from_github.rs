use std::process::Command;

fn main(repository_link){
    let output = if cfg!(target_os = "windows") {
        println!("Did your mother ever teach you not to do stuff that aren't allowed?");
        return;
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("cd ~/Documents/Viron/Projects")
                .output()
                .expect("failed to execute process")
    };
}
