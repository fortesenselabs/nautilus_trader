// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2023 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

use std::borrow::Cow;

use nautilus_core::{time::UnixNanos, uuid::UUID4};
use nautilus_model::identifiers::trader_id::TraderId;
use pyo3::prelude::*;
use ustr::Ustr;

use crate::{
    enums::{LogColor, LogLevel},
    logging::{self, FileWriterConfig, LoggerConfig},
};

/// Initialize tracing.
///
/// Tracing is meant to be used to trace/debug async Rust code. It can be
/// configured to filter modules and write up to a specific level only using
/// by passing a configuration using the `RUST_LOG` environment variable.
///
/// # Safety
///
/// Should only be called once during an applications run, ideally at the
/// beginning of the run.
#[pyfunction()]
#[pyo3(name = "init_tracing")]
pub fn py_init_tracing() {
    logging::init_tracing();
}

/// Initialize logging.
///
/// Logging should be used for Python and sync Rust logic which is most of
/// the components in the main `nautilus_trader` package.
/// Logging can be configured to filter components and write up to a specific level only
/// by passing a configuration using the `NAUTILUS_LOG` environment variable.
///
/// # Safety
///
/// Should only be called once during an applications run, ideally at the
/// beginning of the run.
#[pyfunction]
#[pyo3(name = "init_logging")]
pub fn py_init_logging(
    trader_id: TraderId,
    instance_id: UUID4,
    config_spec: String,
    directory: Option<String>,
    file_name: Option<String>,
    file_format: Option<String>,
) {
    logging::init_logging(
        trader_id,
        instance_id,
        config_spec,
        directory,
        file_name,
        file_format,
    );
}

/// Create a new log event.
#[pyfunction]
#[pyo3(name = "logger_log")]
pub fn py_logger_log(
    timestamp_ns: UnixNanos,
    level: LogLevel,
    color: LogColor,
    component: String,
    message: String,
) {
    logging::log(
        timestamp_ns,
        level,
        color,
        Ustr::from(&component),
        Cow::from(message),
    );
}

#[pymethods]
impl FileWriterConfig {
    #[new]
    pub fn py_new(
        directory: Option<String>,
        file_name: Option<String>,
        file_format: Option<String>,
    ) -> Self {
        Self::new(directory, file_name, file_format)
    }
}

#[pymethods]
impl LoggerConfig {
    #[staticmethod]
    #[pyo3(name = "from_spec")]
    pub fn py_from_spec(spec: String) -> Self {
        LoggerConfig::from_spec(&spec)
    }
}
