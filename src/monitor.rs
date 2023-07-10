/**
 * Copyright (c) 2023 Institute of Computing Technology, Chinese Academy of Sciences
 * xfuzz is licensed under Mulan PSL v2.
 * You can use this software according to the terms and conditions of the Mulan PSL v2.
 * You may obtain a copy of Mulan PSL v2 at:
 *          http://license.coscl.org.cn/MulanPSL2
 * THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
 * EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
 * MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 * See the Mulan PSL v2 for more details.
 */

use std::fs;
use std::path::PathBuf;

use libafl::prelude::{BytesInput, Corpus, InMemoryCorpus, Input, OnDiskCorpus, RomuDuoJrRand};
use libafl::state::{HasCorpus, StdState};

pub fn store_testcases(
    state: &mut StdState<
        BytesInput,
        InMemoryCorpus<BytesInput>,
        RomuDuoJrRand,
        OnDiskCorpus<BytesInput>,
    >,
    output_dir: String,
) {
    let corpus = state.corpus();

    let count = corpus.count();
    println!("Total corpus count: {count}");

    fs::create_dir_all(&output_dir).expect("Unable to create testcases directory");
    for id in corpus.ids() {
        let testcase: std::cell::RefMut<libafl::prelude::Testcase<BytesInput>> =
            corpus.get(id).unwrap().borrow_mut();
        let executions = testcase.executions();
        let scheduled_count = testcase.scheduled_count();
        let parent_id = if testcase.parent_id().is_some() {
            usize::from(testcase.parent_id().unwrap()) as i32
        } else {
            -1
        };
        println!("Corpus {id}: executions {executions}, scheduled_count {scheduled_count}, parent_id {parent_id}");
        let x = testcase.input().as_ref().unwrap();
        x.to_file(PathBuf::from(format!("{output_dir}/{id}")).as_path())
            .expect(format!("written {id} failed").as_str());
    }
}
