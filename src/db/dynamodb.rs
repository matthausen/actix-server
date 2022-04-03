use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeDefinition, 
    CreateTableInput, 
    DynamoDb, 
    DynamoDbClient, 
    KeySchemaElement, 
    ListTablesInput, 
    ProvisionedThroughput,
};

use crate::config::load_config::Config;


// Initialise a connection with dynamodb.
// A global config is passed by reference.
// On boot it will list all existing tables. If it can't find them
// it will automatically generate one with the config defined in the yml file depending on the environment.
#[tokio::main]
pub async fn init_db_pool(cfg: &Config) {
    let region: Region = Region::Custom {
        endpoint: String::from(&cfg.yml_cfg.dynamodb.endpoint),
        name: String::from(&cfg.yml_cfg.dynamodb.region_name),
    };

    let client = DynamoDbClient::new(region);
    let list_tables_input: ListTablesInput = Default::default();
 
    match client.list_tables(list_tables_input).await {
        Ok(output) => {
            match output.table_names {
                Some(table_name_list) => {
                    if table_name_list.len() > 0 {
                        for table_name in table_name_list {
                            println!("Table {} already exist", table_name);
                        }
                    } else {
                        let table_name: String = String::from(&cfg.yml_cfg.dynamodb.table_name);
                        let _ = client.create_table(CreateTableInput {
                                table_name,
                                key_schema: vec![KeySchemaElement {
                                    attribute_name: "Id".into(),
                                    key_type: "HASH".into(),
                                }],
                                attribute_definitions: vec![AttributeDefinition {
                                    attribute_name: "Id".into(),
                                    attribute_type: "S".into(),
                                }],
                                provisioned_throughput: Some(ProvisionedThroughput {
                                    read_capacity_units: 20,
                                    write_capacity_units: 20,
                                }),
                                ..CreateTableInput::default()
                            })
                            .await;
                    }
                },
                None => println!("No tables in database!"),
            }
        },
        Err(error) => {
            println!("Error: {:?}", error);
        },
    }

}