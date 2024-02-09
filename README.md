<div align="center">
<h1>Roost Stat</h1>
<img src="https://res.cloudinary.com/dt2ezdpje/image/upload/v1707517685/roost-stat-example-1_omvcsi.png" alt="roost_stat example">
</div>

## What does roost_stat do?

Shell command tool, when given a directory data concerning Rust file statistics is returned.

- Rust file count
- File Metrics
  - blanks
  - comments
  - lines of code

## Install

### Required

0. Ensure Rust is installed on your machine
1. Clone down repo and cd
2. Run command `Cargo build --release`
3. Add to path

### Usage

```bash
./roost_stat -m src <target directory>
```
