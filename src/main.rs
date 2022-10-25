use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::error::ScanError;
use aws_sdk_dynamodb::model::{AttributeValue};
use aws_sdk_dynamodb::types::SdkError;
use dynamodb_helper::DynamoDb;
use tokio_stream::StreamExt;

// TODO remove test struct and test db
struct TestStruct {
    partition_key: String,
    value: i32,
}

struct TestDB {
    client: Client,
    table: String,
}

impl TestDB {
    fn new(client: Client, table: String) -> Self {
        TestDB {
            client,
            table,
        }
    }

    // pub async fn scan(&self) -> Result<Vec<TestStruct>, SdkError<ScanError>> {
    //     let items: Result<Vec<std::collections::HashMap<std::string::String, aws_sdk_dynamodb::model::AttributeValue>>, _> = self.client.scan()
    //         .table_name(&self.table)
    //         .into_paginator()
    //         .items()
    //         .send()
    //         .collect()
    //         .await
    //         ;
    //
    //     items
    //         .map(|v| v.iter().map(|i| i.into()).collect())
    //     // Ok(mapped_items)
    // }
}

impl From<TestStruct> for HashMap<String, AttributeValue> {
    fn from(_: TestStruct) -> Self {
        todo!()
    }
}

impl From<HashMap<String, AttributeValue>> for TestStruct {
    fn from(_: HashMap<String, AttributeValue>) -> Self {
        todo!()
    }
}

impl From<&HashMap<String, AttributeValue>> for TestStruct {
    fn from(_: &HashMap<String, AttributeValue>) -> Self {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    #[derive(DynamoDb)]
    pub struct ExampleStruct {
        #[partition]
        partition_key: String,
        a_number: u32,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[derive(DynamoDb)]
    #[exclusion("scan", "delete_table", "create_table")]
    pub struct ExampleTestStruct {
        #[partition]
        partition_key: String,
        value: i32,
        true_or_false: bool,
        some_string: String,
        number_liz: Vec<i16>,
        string_list: Vec<String>,
        a_map: HashMap<String, String>,
        optional_string: Option<String>,
        optional_number: Option<i32>,
    }

    #[tokio::test]
    async fn should_generate_a_helper_struct_with_build_method() {
        let _e = ExampleTestStruct {
            partition_key: "example".to_string(),
            value: 0,
            true_or_false: false,
            some_string: "".to_string(),
            number_liz: vec![],
            string_list: vec![],
            a_map: Default::default(),
            optional_string: None,
            optional_number: None
        };
        let _help = ExampleTestStructDb::build(aws_sdk_dynamodb::Region::new("eu-west-1"), "exampleTable").await;
    }
}
