// Zinc, the bare metal stack for rust.
// Copyright 2014 Vladimir "farcaller" Pouzanov <farcaller@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/*!
System initialization interface.

SysConf is a MCU-specific struct, that performs initial MCU configuration,
including but not limited to `.bss`, `.data` and stack memory initialization,
system clock configuration and any other boot sequences.
*/

#[cfg(mcu_lpc17xx)] pub use hal::lpc17xx::init::SysConf;
#[cfg(mcu_stm32f4)] pub use hal::stm32f4::init::SysConf;
