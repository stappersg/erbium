/*   Copyright 2024 Perry Lorier
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *
 *  SPDX-License-Identifier: Apache-2.0
 *
 *  Dumps out all the configuration, as it's been parsed.
 *  Primarily used for debugging and testing configurations.
 */

extern crate erbium;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args_os().collect();
    let config_file = match args.len() {
        1 => std::path::Path::new("erbium.conf"),
        2 => std::path::Path::new(&args[1]),
        _ => {
            println!("Usage: {} <configfile>", args[0].to_string_lossy());
            return Ok(());
        }
    };
    println!("Loading config from {}", config_file.display());
    let conf = erbium::config::load_config_from_path(config_file).await?;
    println!("Parse config: {:#?}", conf.read().await);
    Ok(())
}
