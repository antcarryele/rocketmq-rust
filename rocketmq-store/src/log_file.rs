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

use std::sync::Arc;

use rocketmq_common::{
    common::message::message_single::MessageExtBrokerInner, TimeUtils::get_current_millis,
};

use crate::{
    base::message_result::PutMessageResult, hook::put_message_hook::BoxedPutMessageHook,
    store::running_flags::RunningFlags,
};

pub mod commit_log;
pub mod flush_manager_impl;
pub mod mapped_file;

#[trait_variant::make(MessageStore: Send)]
pub trait RocketMQMessageStore: Clone + 'static {
    /// Load previously stored messages.
    ///
    /// Returns `true` if success; `false` otherwise.
    async fn load(&mut self) -> bool;

    /// Launch this message store.
    ///
    /// # Throws
    ///
    /// Throws an `Exception` if there is any error.
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    fn shutdown(&mut self);

    fn set_confirm_offset(&mut self, phy_offset: i64);

    fn get_max_phy_offset(&self) -> i64;

    fn set_broker_init_max_offset(&mut self, broker_init_max_offset: i64);

    fn now(&self) -> u64 {
        get_current_millis()
    }

    fn get_state_machine_version(&self) -> i64;

    async fn put_message(&mut self, msg: MessageExtBrokerInner) -> PutMessageResult;

    fn truncate_files(&mut self, offset_to_truncate: i64) -> bool;

    fn is_os_page_cache_busy(&self) -> bool {
        false
    }

    fn get_running_flags(&self) -> &RunningFlags;

    fn is_shutdown(&self) -> bool;

    fn get_put_message_hook_list(&self) -> Arc<parking_lot::RwLock<Vec<BoxedPutMessageHook>>>;

    fn set_put_message_hook(&self, put_message_hook: BoxedPutMessageHook);
}
