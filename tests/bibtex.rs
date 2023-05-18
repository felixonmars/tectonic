// Copyright 2016-2021 the Tectonic Project
// Licensed under the MIT License.

use std::collections::HashSet;
use std::default::Default;
use std::path::PathBuf;

use tectonic::io::{FilesystemIo, IoProvider, IoStack, MemoryIo};
use tectonic::BibtexEngine;
use tectonic_bridge_core::{CoreBridgeLauncher, MinimalDriver};
use tectonic_io_base::stdstreams::GenuineStdoutIo;
use tectonic_status_base::NoopStatusBackend;

#[path = "util/mod.rs"]
mod util;
use crate::util::{test_path, ExpectedInfo};

struct TestCase {
    stem: String,
    subdir: Option<String>,
    test_bbl: bool,
}

impl TestCase {
    fn new(stem: &str, subdir: Option<&str>) -> Self {
        TestCase {
            stem: stem.to_owned(),
            subdir: subdir.map(String::from),
            test_bbl: true,
        }
    }

    fn test_bbl(mut self, test: bool) -> Self {
        self.test_bbl = test;
        self
    }

    fn test_dir(&self) -> PathBuf {
        let mut p = test_path(&["bibtex"]);
        if let Some(subdir) = &self.subdir {
            p.push(subdir);
        }
        p
    }

    fn go(&mut self) {
        util::set_test_root();

        let mut p = self.test_dir();

        let auxname = format!("{}.aux", self.stem);

        // MemoryIo layer that will accept the outputs.
        let mut mem = MemoryIo::new(true);

        let mut assets = FilesystemIo::new(&p, false, false, HashSet::new());

        let mut genio = GenuineStdoutIo::new();

        let io_list: Vec<&mut dyn IoProvider> = vec![&mut genio, &mut mem, &mut assets];

        let io = IoStack::new(io_list);
        let mut hooks = MinimalDriver::new(io);
        let mut status = NoopStatusBackend::default();
        let mut launcher = CoreBridgeLauncher::new(&mut hooks, &mut status);

        BibtexEngine::new()
            .process(&mut launcher, &auxname, &Default::default())
            .unwrap();

        // Check that outputs match expectations.

        p.push(&self.stem);

        let files = mem.files.borrow();

        if self.test_bbl {
            let expected_bbl = ExpectedInfo::read_with_extension(&mut p, "bbl");
            expected_bbl.test_from_collection(&files);
        }

        let expected_blg = ExpectedInfo::read_with_extension(&mut p, "blg");
        expected_blg.test_from_collection(&files);
    }
}

#[test]
fn single_entry() {
    TestCase::new("single_entry", None).go()
}

#[test]
fn test_empty_files() {
    TestCase::new("empty", Some("empty")).test_bbl(false).go()
}

#[test]
fn test_mismatched_function() {
    TestCase::new("function", Some("mismatched_braces"))
        .test_bbl(false)
        .go();
}

#[test]
fn test_mismatched_expr() {
    TestCase::new("expr", Some("mismatched_braces"))
        .test_bbl(false)
        .go();
}
