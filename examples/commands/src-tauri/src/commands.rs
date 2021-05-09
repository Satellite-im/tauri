// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::{command, State};

#[command]
pub fn simple_command(argument: String) {
  println!("{}", argument);
}

#[command]
pub fn stateful_command(argument: Option<String>, state: State<'_, super::MyState>) {
  println!("{:?} {:?}", argument, state.inner());
}