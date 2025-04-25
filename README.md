# Rust Lab - Spreadsheet with Mini VCS

## Overview

This project implements a terminal-based spreadsheet program with two modes: a basic spreadsheet mode and an extended version with version control features (Mini VCS). The program supports cell updates, formula evaluations, and a custom lightweight version control system for tracking changes.

## Instructions to Run

### Normal Mode

The **normal mode** supports standard spreadsheet operations such as cell updates, formula evaluations, and dependency tracking.

To run the program in **normal mode**:

```sh
make run
```

You will be prompted to enter:
- The number of rows in the spreadsheet
- The number of columns

This mode does **not** support version control and will not track changes or commits.

---

### VCS Mode (Mini Version Control System)

The **VCS mode** enables basic version control features through a custom system called `gitsap`. It supports operations such as committing changes, checking out previous commits, and viewing the commit history.

To start the program in **VCS mode**:

```sh
make ext1
```

You will be prompted to choose one of the following options:

#### 1. Load from Existing Commits
- Select option `1`
- Enter the path to the directory (`vcs_dir`) containing the commit JSON files
- The program will load the most recent commit and restore the spreadsheet state

#### 2. Initialize a New Tracked Spreadsheet
- Select option `2`
- Input the number of rows and columns
- A new version-controlled spreadsheet will be created
- Commits will be stored in the `vcs_dir` folder inside the `spreadsheet` directory
  > Any existing commit history in this location will be overwritten


The supported VCS commands are:
1. `gitsap list`: To list all the commits
2. `gitsap commit <COMMIT_MSG>`: To commit the current state of the spreadsheet
3. `gitsap checkout <COMMIT_ID>`: To checkout to some other commit. `<COMMIT_ID>` are integers starting from `1`.
---
## Build, Test, and Docs
### Build the Project
To build the project and compile the program:

```sh
make build
```

### Run Tests
To run the tests for the project:

```sh
make test
```

### Generate Documentation
To generate documentation for the project:

```sh
make docs
```

### Clean Artifacts
To clean up the build artifacts:

```sh
make clean
```

### Test Coverage (Requires `cargo-tarpaulin`)
To run test coverage reports:

```sh
make coverage
```

