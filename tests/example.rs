// Copyright (c) The datatest-stable Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fs::File;
use std::io::Read;
use std::path::Path;

use camino::Utf8Path;
use datatest_stable::Result;

fn test_artifact(path: &Path) -> Result<()> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(())
}

fn test_artifact_utf8(path: &Utf8Path) -> Result<()> {
    test_artifact(path.as_ref())
}

datatest_stable::harness!(
    test_artifact,
    "tests/files",
    r"^.*/*",
    (ignore r"^.*/*-ignore"),
    test_artifact_utf8,
    "tests/files",
    r"^.*/*",
);
