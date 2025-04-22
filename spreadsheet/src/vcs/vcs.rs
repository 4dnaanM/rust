use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;
use crate::spreadsheet::SpreadSheet;

#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Commit {
    id: u32,
    message: String,
    parent: Option<u32>,
    // delta: Option<Delta>,
    spreadsheet: SpreadSheet, // Store the spreadsheet state
    // spread_dist: u32, // Track the last full spreadsheet state
    user: String, // Track the user who made the commit
}

// #[derive(Serialize, Deserialize, Clone)]
// pub struct Delta {
//     pub cells: HashMap<(u32,u32), String>,
// }



pub struct VersionControlSystem {
    commits_with_parent: HashMap<u32,(u32,String)>, // List of commit IDs
    current_id: u32,
    next_commit_id: u32,
    vcs_dir: String,
    // user_manager: UserManager,
    // current_user: Option<String>, // Track the logged-in user
}

impl VersionControlSystem {
    pub fn new(vcs_dir: &str) -> Self {
        if !Path::new(vcs_dir).exists() {
            fs::create_dir(vcs_dir).expect("Failed to create VCS directory");
            // panic!("Failed to create VCS directory");
        }

        VersionControlSystem {
            commits_with_parent: HashMap::new(),
            current_id: 0,
            next_commit_id: 1,
            vcs_dir: vcs_dir.to_string(),
        }
    }

    pub fn commit(&mut self, spreadsheet: &SpreadSheet, message: &str) {
        println!("Committing changes...");
        // Check if the commit message is empty
        println!("Hui");
        let commit = Commit {
            id: self.next_commit_id,
            message: message.to_string(),
            parent: Some(self.current_id),
            spreadsheet: spreadsheet.clone(),
            user: String::from("user"),
        };
        println!("Hui");
        let commit_path = format!("{}/commit_{}.json", self.vcs_dir, self.next_commit_id);
        println!("Hui");
        let file = File::create(&commit_path).expect("Failed to create commit file");
        println!("Hui");
        serde_json::to_writer(file, &commit).expect("Failed to serialize commit");
        println!("Hui");

        self.commits_with_parent.insert(self.next_commit_id, (self.current_id,commit.message));
        println!("Hui");
        
        println!(
            "Commit created with ID: {} by user: {}",
            self.current_id,
            commit.user,
            // self.current_user.clone().unwrap()
        );
        self.current_id = self.next_commit_id;
        self.next_commit_id += 1;

    }

    pub fn list(&self) {
        for commit in self.commits_with_parent.keys() {
            let commit_ = self.commits_with_parent.get(commit).unwrap();
            println!(
                "\nID: {}, Message: {}, Parent: {:?}\n",
                commit, commit_.1, commit_.0
            );
        }
    }

    pub fn checkout(&mut self, branch_id: u32, spreadsheet: &SpreadSheet) -> SpreadSheet {
        if let Some(_commit) = self.commits_with_parent.get(&branch_id) {
            self.current_id = branch_id;
            println!("Checked out to branch ID: {}", branch_id);
            return self.reconstruct_spreadsheet(branch_id);       
        } else {
            println!("Branch ID {} does not exist", branch_id);
            return spreadsheet.clone();
        }
    }

    fn reconstruct_spreadsheet(&self, commit_id: u32) -> SpreadSheet {
        
        let commit_path = format!("{}/commit_{}.json", self.vcs_dir, commit_id);
        let file = File::open(&commit_path).expect("Failed to open commit file");
        let commit: Commit = serde_json::from_reader(file).expect("Failed to deserialize commit");
        
        let spreadsheet = commit.spreadsheet.clone();
        spreadsheet
    }
}