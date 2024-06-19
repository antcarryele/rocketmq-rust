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
use rocketmq_common::common::filter::expression_type::ExpressionType;
use rocketmq_common::common::hasher::string_hasher::JavaStringHasher;

use crate::protocol::heartbeat::subscription_data::SubscriptionData;

pub struct FilterAPI;

impl FilterAPI {
    pub fn build_subscription_data(
        topic: &str,
        sub_string: &str,
    ) -> Result<SubscriptionData, String> {
        let mut subscription_data = SubscriptionData {
            topic: topic.to_string(),
            sub_string: sub_string.to_string(),
            ..Default::default()
        };

        if sub_string.is_empty() || sub_string == SubscriptionData::SUB_ALL {
            subscription_data.sub_string = SubscriptionData::SUB_ALL.to_string();
            return Ok(subscription_data);
        }

        let tags: Vec<&str> = sub_string.split("||").collect();
        if tags.is_empty() {
            return Err("subString split error".to_string());
        }

        for tag in tags {
            let trimmed_tag = tag.trim();
            if !trimmed_tag.is_empty() {
                subscription_data.tags_set.insert(trimmed_tag.to_string());
                subscription_data
                    .code_set
                    .insert(JavaStringHasher::new().hash_str(tag));
            }
        }

        Ok(subscription_data)
    }

    pub fn build_subscription_data_with_expression_type(
        topic: &str,
        sub_string: &str,
        expression_type: Option<String>,
    ) -> Result<SubscriptionData, String> {
        let mut subscription_data = FilterAPI::build_subscription_data(topic, sub_string)?;
        if let Some(expr_type) = expression_type {
            subscription_data.expression_type = expr_type;
        }
        Ok(subscription_data)
    }

    pub fn build(
        topic: &str,
        sub_string: &str,
        type_: Option<String>,
    ) -> Result<SubscriptionData, String> {
        if type_.as_deref() == Some(ExpressionType::TAG) || type_.is_none() {
            return FilterAPI::build_subscription_data(topic, sub_string);
        }

        if sub_string.is_empty() {
            return Err(format!(
                "Expression can't be null! {}",
                type_.unwrap_or_default()
            ));
        }

        let mut subscription_data = SubscriptionData {
            topic: topic.to_string(),
            sub_string: sub_string.to_string(),
            ..Default::default()
        };
        if let Some(type_) = type_ {
            subscription_data.expression_type = type_;
        }
        Ok(subscription_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_subscription_data_creates_correct_subscription_data() {
        let topic = "test_topic";
        let sub_string = "tag1||tag2";
        let subscription_data = FilterAPI::build_subscription_data(topic, sub_string).unwrap();

        assert_eq!(subscription_data.topic, topic);
        assert_eq!(subscription_data.sub_string, sub_string);
        assert!(subscription_data.tags_set.contains("tag1"));
        assert!(subscription_data.tags_set.contains("tag2"));
    }

    #[test]
    fn build_subscription_data_with_empty_sub_string_creates_subscription_data_with_sub_all() {
        let topic = "test_topic";
        let sub_string = "";
        let subscription_data = FilterAPI::build_subscription_data(topic, sub_string).unwrap();

        assert_eq!(subscription_data.topic, topic);
        assert_eq!(subscription_data.sub_string, SubscriptionData::SUB_ALL);
    }

    #[test]
    fn build_subscription_data_with_expression_type_sets_expression_type() {
        let topic = "test_topic";
        let sub_string = "tag1||tag2";
        let expression_type = Some("SQL92".to_string());
        let subscription_data = FilterAPI::build_subscription_data_with_expression_type(
            topic,
            sub_string,
            expression_type.clone(),
        )
        .unwrap();

        assert_eq!(subscription_data.topic, topic);
        assert_eq!(subscription_data.sub_string, sub_string);
        assert_eq!(subscription_data.expression_type, expression_type.unwrap());
    }

    #[test]
    fn build_creates_correct_subscription_data_for_tag_expression_type() {
        let topic = "test_topic";
        let sub_string = "tag1||tag2";
        let type_ = Some(ExpressionType::TAG.to_string());
        let subscription_data = FilterAPI::build(topic, sub_string, type_).unwrap();

        assert_eq!(subscription_data.topic, topic);
        assert_eq!(subscription_data.sub_string, sub_string);
    }

    #[test]
    fn build_returns_error_for_empty_sub_string_and_non_tag_expression_type() {
        let topic = "test_topic";
        let sub_string = "";
        let type_ = Some(ExpressionType::SQL92.to_string());
        let result = FilterAPI::build(topic, sub_string, type_);

        assert!(result.is_err());
    }
}
