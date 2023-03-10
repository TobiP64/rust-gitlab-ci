// MIT License
//
// Copyright (c) 2021 Tobias Pfeiffer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::*;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CargoMessage {
	Suite(CargoTestReportSuite),
	Test(CargoTestReportTest),
	Bench(CargoTestReportBench)
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "event", rename_all = "lowercase")]
pub enum CargoTestReportSuite {
	Started(CargoTestReportSuiteStarted),
	Ok(CargoTestReportSuiteOkOrFailed),
	Failed(CargoTestReportSuiteOkOrFailed)
}

#[derive(Clone, Debug, Deserialize)]
pub struct CargoTestReportSuiteStarted {
	pub test_count: usize
}

#[derive(Clone, Debug, Deserialize)]
pub struct CargoTestReportSuiteOkOrFailed {
	pub passed:        usize,
	pub failed:        usize,
	#[serde(default)]
	pub allowed_fail:  usize,
	pub ignored:       usize,
	pub measured:      usize,
	pub filtered_out:  usize,
	pub exec_time:     f64
}

#[derive(Clone, Debug, Deserialize)]
pub struct CargoTestReportTest {
	pub name: String,
	#[serde(flatten)]
	pub event: CargoTestReportTestEvent
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "event", rename_all = "lowercase")]
pub enum CargoTestReportTestEvent {
	Started,
	Ignored,
	Ok(CargoTestReportTestOkOrFailed),
	Failed(CargoTestReportTestOkOrFailed)
}

#[derive(Clone, Debug, Deserialize)]
pub struct CargoTestReportTestOkOrFailed {
	pub stdout: Option<String>
}

#[derive(Clone, Debug, Deserialize)]
pub struct CargoTestReportBench {
	pub name:      String,
	pub median:    f64,
	pub deviation: f64
}