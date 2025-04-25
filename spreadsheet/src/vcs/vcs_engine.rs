use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;

use crate::spreadsheet::SpreadSheet;
use crate::utils::Type;

pub struct VersionControl {
    map: HashMap<usize, (usize, String)>,
    vcs_dir: String,
    curr_commit: usize,
    next_commit: usize,
    spread_sheet: CloneSpreadSheet,
    m: usize,
    n: usize,
}

#[derive(Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SerialVcs {
    map: HashMap<usize, (usize, String)>,
    m: usize,
    n: usize,
}

#[derive(Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SerialCell {
    row: usize,
    col: usize,
    c1: Option<(usize, usize)>,
    c2: Option<(usize, usize)>,
    v1: Option<i32>,
    v2: Option<i32>,
    t: Type,
}

impl SerialCell {
    pub fn compare(&self, other: &SerialCell) -> bool {
        self.row == other.row
            && self.col == other.col
            && self.c1 == other.c1
            && self.c2 == other.c2
            && self.v1 == other.v1
            && self.v2 == other.v2
            && self.t == other.t
    }
}

pub struct CloneSpreadSheet {
    cells: Vec<Vec<SerialCell>>,
}

impl CloneSpreadSheet {
    pub fn new(m: usize, n: usize) -> Self {
        let cells = vec![
            vec![
                SerialCell {
                    row: 0,
                    col: 0,
                    c1: None,
                    c2: None,
                    v1: None,
                    v2: None,
                    t: Type::Nul
                };
                n
            ];
            m
        ];
        CloneSpreadSheet { cells }
    }

    pub fn clone_spread(spreadsheet: &mut SpreadSheet) -> Self {
        let m = spreadsheet.m;
        let n = spreadsheet.n;
        let mut cells = vec![
            vec![
                SerialCell {
                    row: 0,
                    col: 0,
                    c1: None,
                    c2: None,
                    v1: None,
                    v2: None,
                    t: Type::Nul
                };
                n
            ];
            m
        ];
        for (i, row) in cells.iter_mut().enumerate().take(m) {
            for (j, cell) in row.iter_mut().enumerate().take(n) {
                let params = spreadsheet.get_cell_equation_parameters(i, j);
                let c1 = params.operand1_coordinates;
                let c2 = params.operand2_coordinates;
                let v1 = params.operand1_value;
                let v2 = params.operand2_value;
                let t = params.equation_type;
                *cell = SerialCell {
                    row: i,
                    col: j,
                    c1,
                    c2,
                    v1,
                    v2,
                    t,
                };
            }
        }
        CloneSpreadSheet { cells }
    }
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SerialSheetDiff {
    id: usize,
    cells: Vec<SerialCell>,
}

impl VersionControl {
    pub fn new(vcs_dir: String, m: &usize, n: &usize) -> Self {
        let mut map = HashMap::new();
        let vcs_dir2 = vcs_dir.clone();
        if !Path::new(&vcs_dir2).exists() {
            fs::create_dir(vcs_dir2).expect("Failed to create VCS directory");
            // panic!("Failed to create VCS directory");
        }
        map.insert(1, (0, "Init".to_string()));
        VersionControl {
            map,
            vcs_dir,
            curr_commit: 0,
            next_commit: 1,
            spread_sheet: CloneSpreadSheet::new(*m, *n),
            m: *m,
            n: *n,
        }
    }

    pub fn dummy() -> Self {
        let vcs_dir = "./vcs_dir".to_string();
        let m = 1;
        let n = 1;
        VersionControl {
            map: HashMap::new(),
            vcs_dir,
            curr_commit: 0,
            next_commit: 1,
            spread_sheet: CloneSpreadSheet::new(m, n),
            m: 0,
            n: 0,
        }
    }

    pub fn load(serial_vcs: SerialVcs, vcs_dir: String) -> Self {
        let map = serial_vcs.map;
        let commit_count = map.len();
        VersionControl {
            map,
            vcs_dir,
            curr_commit: 0,
            next_commit: commit_count + 1,
            m: serial_vcs.m,
            n: serial_vcs.n,
            spread_sheet: CloneSpreadSheet::new(serial_vcs.m, serial_vcs.n),
        }
    }

    pub fn commit(&mut self, commit_msg: &str, spreadsheet: &mut SpreadSheet) {
        let serial_sheet_diff = SerialSheetDiff {
            id: self.next_commit,
            cells: self.get_diff_spread(
                &mut CloneSpreadSheet::clone_spread(spreadsheet),
                &self.spread_sheet,
            ),
        };

        self.map
            .insert(self.next_commit, (self.curr_commit, commit_msg.to_string()));

        let commit_path = format!("{}/commit_{}.json", self.vcs_dir, self.next_commit);
        let file = File::create(&commit_path).expect("Failed to create commit file");
        serde_json::to_writer(file, &serial_sheet_diff).expect("Failed to serialize commit");

        SerialVcs::save(self);

        self.curr_commit = self.next_commit;
        self.next_commit += 1;
        self.spread_sheet = CloneSpreadSheet::clone_spread(spreadsheet);
    }

    pub fn list(&self) {
        for (commit, (parent, message)) in &self.map {
            println!(
                "Commit ID: {}, Parent Commit: {}, Message: {}",
                commit, parent, message
            );
        }
    }

    pub fn checkout(&mut self, id: usize) -> SpreadSheet {
        let vcs_dir = &self.vcs_dir;
        self.spread_sheet = CloneSpreadSheet::new(self.m, self.n);
        if !Path::new(vcs_dir).exists() {
            panic!("VCS directory does not exist");
        }

        // Create a parent order.
        let mut commit_chain = Vec::new();
        if !self.map.contains_key(&id) {
            panic!("Commit id {} not found in VCS", id);
        }
        commit_chain.push(id);
        while *commit_chain.last().unwrap() != 1 {
            let last = *commit_chain.last().unwrap();
            let &(parent, _) = self.map.get(&last).unwrap_or_else(|| {
                panic!("Commit {} not found in VCS while traversing parents", last)
            });
            if parent == 0 {
                panic!("Encountered an invalid parent (0) for commit {}", last);
            }
            commit_chain.push(parent);
        }

        commit_chain.reverse();
        for commit_id in commit_chain {
            let commit_path = format!("{}/commit_{}.json", vcs_dir, commit_id);
            let file = File::open(&commit_path).expect("Failed to open commit file");
            let serial_sheet_diff: SerialSheetDiff =
                serde_json::from_reader(file).expect("Failed to deserialize commit");

            for cell in serial_sheet_diff.cells {
                let (row, col) = (cell.row, cell.col);
                self.spread_sheet.cells[row][col] = cell;
            }
        }

        let mut spreadsheet = SpreadSheet::new(self.m, self.n);
        for i in 0..self.m {
            for j in 0..self.n {
                let cell = &self.spread_sheet.cells[i][j];
                let (c1, c2, v1, v2, t) = (cell.c1, cell.c2, cell.v1, cell.v2, cell.t);
                if t == Type::Nul || (c1.is_none() ^ v1.is_none()) || (c2.is_none() ^ v2.is_none())
                {
                    continue;
                }
                spreadsheet.set_cell_equation((i, j), c1, c2, v1, v2, t);
            }
        }

        spreadsheet
    }

    pub fn get_diff_spread(
        &self,
        current_spreadsheet: &mut CloneSpreadSheet,
        vcs_spreadsheet: &CloneSpreadSheet,
    ) -> Vec<SerialCell> {
        let mut diff_cells = vec![];
        for i in 0..self.m {
            for j in 0..self.n {
                let current_cell = &current_spreadsheet.cells[i][j];
                let vcs_cell = &vcs_spreadsheet.cells[i][j];

                if !current_cell.compare(vcs_cell) {
                    diff_cells.push(current_cell.clone());
                }
            }
        }
        diff_cells
    }

    pub fn get_m_n(&self) -> (usize, usize) {
        (self.m, self.n)
    }
}

impl SerialVcs {
    pub fn new(vcs: &VersionControl) -> Self {
        // with save
        SerialVcs {
            map: vcs.map.clone(),
            m: vcs.m,
            n: vcs.n,
        }
    }
    pub fn save(vcs: &VersionControl) {
        // used when saved
        let vcs_dir = &vcs.vcs_dir;
        let serial_vcs = SerialVcs::new(vcs);

        let vcs_path = format!("{}/vcs.json", vcs_dir);
        let file = File::create(&vcs_path).expect("Failed to create VCS file");
        serde_json::to_writer(file, &serial_vcs).expect("Failed to serialize VCS");
    }
    pub fn load(vcs_dir: &str) -> SerialVcs {
        let vcs_path = format!("{}/vcs.json", vcs_dir);
        let file = File::open(&vcs_path).expect("Failed to open VCS file");
        let serial_vcs: SerialVcs =
            serde_json::from_reader(file).expect("Failed to deserialize VCS");
        serial_vcs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_control_new() {
        let vcs = VersionControl::new("./vcs_test".to_string(), &10, &10);
        assert_eq!(vcs.get_m_n(), (10, 10));
        assert_eq!(vcs.curr_commit, 0);
        assert_eq!(vcs.next_commit, 1);
        assert!(Path::new("./vcs_test").exists());
    }
}
