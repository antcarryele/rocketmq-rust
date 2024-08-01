/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::any::Any;

use crate::base::access_channel::AccessChannel;
use crate::Result;

pub enum Type {
    Produce,
    Consume,
}

pub trait TraceDispatcher: Any {
    fn start(&self, name_srv_addr: &str, access_channel: AccessChannel) -> Result<()>;
    fn append(&self, ctx: &dyn std::any::Any) -> bool;
    fn flush(&self) -> Result<()>;
    fn shutdown(&self);
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}
