//! Parity between this crate and `gamedb_index/winfish.sqlite`, the port's source of truth.
//!
//! * `port/manifest.csv` has exactly one row per function in `port_functions`, with the
//!   same name, file, size and source hash (a changed hash means the decompiled source
//!   moved under an existing port and the port must be re-checked).
//! * Every `/// port: <address> <qualified name>` tag in `src/` names a real function,
//!   sits right above a Rust fn with that function's (mangled) name, and is the only
//!   way a row can be `ported`.
//! * The generated layouts cover every type the database holds.
//!
//! The database is opened read-only. Override its path with `WINFISH_DB`.

use rusqlite::{Connection, OpenFlags};
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

fn db() -> Connection {
    let path = std::env::var("WINFISH_DB").map(PathBuf::from).unwrap_or_else(|_| {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../gamedb_index/winfish.sqlite")
    });
    Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .unwrap_or_else(|e| panic!("open {}: {e}", path.display()))
}

/// Same rule as `mangle` in tools/gen_manifest.py.
fn mangle(name: &str) -> String {
    let s = name.replace('~', "dtor_");
    let mut out = String::new();
    for ch in s.chars() {
        let c = if ch.is_ascii_alphanumeric() || ch == '_' { ch } else { '_' };
        if c == '_' && out.ends_with('_') {
            continue;
        }
        out.push(c);
    }
    if out.starts_with('_') && !name.starts_with('_') {
        out = out.trim_start_matches('_').to_string();
    }
    if out.is_empty() || out.as_bytes()[0].is_ascii_digit() {
        out.insert(0, '_');
    }
    out
}

struct Row {
    qualified_name: String,
    file_path: String,
    size_bytes: i64,
    sha: String,
    status: String,
    rust_item: String,
}

fn manifest() -> BTreeMap<u32, Row> {
    let text = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("port/manifest.csv"))
        .expect("port/manifest.csv (run tools/gen_manifest.py)");
    let mut rows = BTreeMap::new();
    for (i, line) in text.lines().enumerate().skip(1) {
        let f = split_csv(line);
        assert_eq!(f.len(), 9, "manifest line {} malformed: {line}", i + 1);
        let addr = u32::from_str_radix(&f[0], 16).expect("address");
        let prev = rows.insert(
            addr,
            Row {
                qualified_name: f[1].clone(),
                file_path: f[2].clone(),
                size_bytes: f[3].parse().unwrap(),
                sha: f[4].clone(),
                status: f[6].clone(),
                rust_item: f[7].clone(),
            },
        );
        assert!(prev.is_none(), "duplicate manifest row {addr:08x}");
    }
    rows
}

fn split_csv(line: &str) -> Vec<String> {
    let (mut out, mut cur, mut quoted) = (Vec::new(), String::new(), false);
    let mut it = line.chars().peekable();
    while let Some(c) = it.next() {
        if c == '"' {
            if quoted && it.peek() == Some(&'"') {
                cur.push('"');
                it.next();
            } else {
                quoted = !quoted;
            }
        } else if c == ',' && !quoted {
            out.push(std::mem::take(&mut cur));
        } else {
            cur.push(c);
        }
    }
    out.push(cur);
    out
}

fn is_tag(line: &str) -> Option<(u32, String)> {
    let rest = line.trim_start().strip_prefix("///")?.trim_start().strip_prefix("port:")?;
    let mut parts = rest.split_whitespace();
    let (a, q) = (parts.next()?, parts.next()?);
    if parts.next().is_some() || a.len() != 8 {
        return None;
    }
    Some((u32::from_str_radix(a, 16).ok()?, q.to_string()))
}

/// address -> (qualified name in tag, "file:line", ident of the following fn)
fn scan_tags() -> HashMap<u32, (String, String, Option<String>)> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        for e in std::fs::read_dir(dir).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                walk(&p, out);
            } else if p.extension().is_some_and(|x| x == "rs") && p.file_name().unwrap() != "generated.rs" {
                out.push(p);
            }
        }
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    walk(&root.join("src"), &mut files);
    let mut tags = HashMap::new();
    for f in files {
        let text = std::fs::read_to_string(&f).unwrap();
        let lines: Vec<&str> = text.split('\n').collect();
        for (i, line) in lines.iter().enumerate() {
            let Some((addr, q)) = is_tag(line) else { continue };
            let mut ident = None;
            for l in lines.iter().skip(i + 1).take(40) {
                if is_tag(l).is_some() {
                    break;
                }
                let t = l.trim_start();
                if t.starts_with("//") {
                    continue;
                }
                if let Some(pos) = t.find("fn ") {
                    if pos == 0 || !t.as_bytes()[pos - 1].is_ascii_alphanumeric() {
                        let id: String =
                            t[pos + 3..].chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
                        ident = Some(id);
                        break;
                    }
                }
            }
            let rel = f.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/");
            let loc = format!("{rel}:{}", i + 1);
            let prev = tags.insert(addr, (q, loc.clone(), ident));
            assert!(prev.is_none(), "duplicate port tag {addr:08x} at {loc}");
        }
    }
    tags
}

#[test]
fn manifest_matches_database() {
    let con = db();
    let man = manifest();
    let mut stmt = con
        .prepare("SELECT address, qualified_name, file_path, size_bytes, text_sha256 FROM port_functions")
        .unwrap();
    let rows: Vec<(u32, String, String, i64, String)> = stmt
        .query_map([], |r| Ok((r.get::<_, i64>(0)? as u32, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)))
        .unwrap()
        .map(Result::unwrap)
        .collect();
    let mut problems = Vec::new();
    for (addr, q, fp, size, sha) in &rows {
        match man.get(addr) {
            None => problems.push(format!("{addr:08x} {q}: missing from manifest")),
            Some(m) => {
                if &m.qualified_name != q || &m.file_path != fp || m.size_bytes != *size {
                    problems.push(format!(
                        "{addr:08x}: manifest says {} / {} / {}, database {q} / {fp} / {size}",
                        m.qualified_name, m.file_path, m.size_bytes
                    ));
                }
                if &m.sha != sha {
                    problems.push(format!("{addr:08x} {q}: source text changed since the manifest was written (re-check the port)"));
                }
            }
        }
    }
    assert_eq!(rows.len(), man.len(), "database has {} functions, manifest {}", rows.len(), man.len());
    assert!(problems.is_empty(), "{} problems:\n{}", problems.len(), problems.join("\n"));
}

#[test]
fn port_tags_match_manifest_and_names() {
    let con = db();
    let names: HashMap<u32, (String, String)> = con
        .prepare("SELECT address, qualified_name, name FROM port_functions")
        .unwrap()
        .query_map([], |r| Ok((r.get::<_, i64>(0)? as u32, (r.get(1)?, r.get(2)?))))
        .unwrap()
        .map(Result::unwrap)
        .collect();
    let man = manifest();
    let tags = scan_tags();
    let mut problems = Vec::new();
    for (addr, (q, loc, ident)) in &tags {
        let Some((dbq, name)) = names.get(addr) else {
            problems.push(format!("{loc}: {addr:08x} is not a function in the database"));
            continue;
        };
        if q != dbq {
            problems.push(format!("{loc}: tag names {q}, database has {dbq}"));
        }
        let want = mangle(name);
        let alt = format!("{want}__{addr:08x}");
        if ident.as_deref() != Some(want.as_str()) && ident.as_deref() != Some(alt.as_str()) {
            problems.push(format!("{loc}: fn after tag is {ident:?}, expected {want} or {alt}"));
        }
        match man.get(addr) {
            Some(m) if m.status == "ported" && &m.rust_item == loc => {}
            Some(m) => problems.push(format!(
                "{loc}: manifest row is {} at {:?}; re-run tools/gen_manifest.py",
                m.status, m.rust_item
            )),
            None => problems.push(format!("{loc}: no manifest row")),
        }
    }
    for (addr, m) in &man {
        if m.status == "ported" && !tags.contains_key(addr) {
            problems.push(format!("{addr:08x} {} is marked ported but no tag exists", m.qualified_name));
        }
        assert!(matches!(m.status.as_str(), "ported" | "replaced" | "pending"), "bad status {}", m.status);
    }
    assert!(problems.is_empty(), "{} problems:\n{}", problems.len(), problems.join("\n"));
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for m in man.values() {
        *counts.entry(m.status.as_str()).or_default() += 1;
    }
    println!("coverage: {counts:?}");
}

#[test]
fn layouts_cover_every_type() {
    use winfish_rs::layouts::generated as g;
    let con = db();
    let count = |kind: &str| -> usize {
        con.query_row("SELECT count(*) FROM port_types WHERE kind=?1", [kind], |r| r.get::<_, i64>(0)).unwrap() as usize
    };
    assert_eq!(g::STRUCT_COUNT, count("struct"));
    assert_eq!(g::UNION_COUNT, count("union"));
    assert_eq!(g::ENUM_COUNT, count("enum"));
    assert_eq!(g::TYPEDEF_COUNT, count("typedef"));
    assert_eq!(g::FUNCDEF_COUNT, count("funcdef"));
}

#[test]
fn source_text_round_trips() {
    use sha2::{Digest, Sha256};
    let con = db();
    let mut stmt = con.prepare("SELECT address_hex, text, text_sha256 FROM port_functions").unwrap();
    let bad: Vec<String> = stmt
        .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?)))
        .unwrap()
        .map(Result::unwrap)
        .filter(|(_, text, sha)| format!("{:x}", Sha256::digest(text.as_bytes())) != *sha)
        .map(|(a, _, _)| a)
        .collect();
    assert!(bad.is_empty(), "function text does not match its stored hash: {bad:?}");
}
