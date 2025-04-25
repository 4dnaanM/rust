use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;

use crate::spreadsheet::{self, SpreadSheet};
use crate::utils::{Status, Type};

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
    m: usize,
    n: usize,
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
        CloneSpreadSheet { m, n, cells }
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
        for i in 0..m {
            for j in 0..n {
                let (_cor, c1, c2, v1, v2, t) = spreadsheet.get_cell_equation_parameters(i, j);
                cells[i][j] = SerialCell {
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
        CloneSpreadSheet { m, n, cells }
    }
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SerialSheetDiff {
    id: usize,
    msg: String,
    parent: usize,
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
            m,
            n,
        }
    }

    pub fn load(serial_vcs: SerialVcs, vcs_dir: String) -> Self {
        let map = serial_vcs.map;
        let commit_count = map.len();
        VersionControl {
            map,
            vcs_dir,
            curr_commit: 0,
            next_commit: commit_count as usize + 1,
            m: serial_vcs.m,
            n: serial_vcs.n,
            spread_sheet: CloneSpreadSheet::new(serial_vcs.m, serial_vcs.n),
        }
    }

    pub fn commit(&mut self, commit_msg: &str, spreadsheet: &mut SpreadSheet) {
        let serial_sheet_diff = SerialSheetDiff {
            id: self.next_commit,
            msg: commit_msg.to_string(),
            parent: self.curr_commit,
            cells: self.get_diff_spread(
                &mut CloneSpreadSheet::clone_spread(spreadsheet),
                &self.spread_sheet,
            ),
        };

        self.map
            .insert(self.next_commit, (self.curr_commit, commit_msg.to_string()));

        let commit_path = format!("{}/commit_{}.json", self.vcs_dir, self.curr_commit);
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

    pub fn checkout(&mut self, id: usize, spreadsheet: &mut SpreadSheet) -> SpreadSheet {
        let vcs_dir = &self.vcs_dir;

        // Create a parent string.

        spreadsheet.clone()
    }

    pub fn get_diff_spread(
        &self,
        current_spreadsheet: &mut CloneSpreadSheet,
        vcs_spreadsheet: &CloneSpreadSheet,
    ) -> Vec<SerialCell> {
        return vec![];
        // let mut diff = Vec::new();
        // let m = self.m;
        // let n = self.n;
        // for i in 0..m {
        //     for j in 0..n {
        //         if Cell::check_diff_cells(
        //             &current_spreadsheet.get_cell(i, j),
        //             &vcs_spreadsheet.get_cell(i, j),
        //         ) {
        //             let status = current_spreadsheet.get_cell_value(i, j).is_some();

        //             let val = if status {
        //                 current_spreadsheet.get_cell_value(i, j).unwrap()
        //             } else {
        //                 0
        //             };
        //             let (op, op_val, operands) = current_spreadsheet.get_op_serial(i, j);
        //             let cell = SerialCell {
        //                 row: i,
        //                 col: j,
        //                 status,
        //                 val,
        //                 op,
        //                 op_val,
        //                 operands,
        //             };
        //             diff.push(cell);
        //         }
        //     }
        // }
        // diff
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
        let serial_vcs = SerialVcs::new(&vcs);

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
