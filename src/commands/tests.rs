//! Tests for the commands

use super::{Commands, CommandParsingError, Command};

fn test_command_parsing(test_output: bool)
{
    let examples = vec![
        "{\"cmd\": \"write_to_disk\", \"db_key\": \"DB_KEY\"}",
        "{\"cmd\": \"read_from_disk\", \"db_key\": \"DB_KEY\"}",
        "{\"cmd\": \"list_keys\", \"db_key\": \"DB_KEY\"}",
        "{\"cmd\": \"list_databases\"}",
        "{\"cmd\": \"get_value\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\"}",
        "{\"cmd\": \"set_value\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"val\":\"DATA\"}",
        "{\"cmd\": \"get_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"index\":42}",
        "{\"cmd\": \"set_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"val\":\"DATA\", \"index\":42}",
        "{\"cmd\": \"append_list\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"val\":\"DATA\"}",
        "{\"cmd\": \"get_list_length\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\"}",
        "{\"cmd\": \"get_recent\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"num\": 42}",
        "{\"cmd\": \"create_database\", \"db_key\": \"DB_KEY\"}",
        "{\"cmd\": \"a_auth\", \"id\": \"ID\", \"token\":\"TOKEN\"}",
        "{\"cmd\": \"g_auth\", \"id_token\": \"ID_TOKEN\"}",
        "{\"cmd\": \"event\", \"event_id\":\"ID\", \"destination\":\"DEST\", \"origin\": \"ORIGIN\", \"data\": \"DATA\"}"];

    let cmd_types = vec![Commands::WriteToDisk, Commands::ReadFromDisk, Commands::ListKeys, Commands::ListDatabases,
                                    Commands::GetValue, Commands::SetValue, Commands::GetIndex,
                                    Commands::SetIndex, Commands::AppendIndex, Commands::GetLengthIndex,
                                    Commands::GetRecentIndex, Commands::CreateDatabase, Commands::AcronymAuth,
                                    Commands::GoogleAuth, Commands::Event];

    for (example, desired) in examples.iter().zip(cmd_types.iter())
    {
        let r = Command::from_string(example);
        if !r.is_ok()
        {
            panic!("Recieved an error for ${}$: {:?}", example, r.unwrap_err());
        }

        if test_output
        {
            assert_eq!(r.unwrap().cmd, *desired);
        }
    }
}

#[test]
pub fn test_command_parsing_success()
{
    test_command_parsing(false);
}

#[test]
pub fn test_command_parsing_output()
{
    test_command_parsing(true);
}

#[test]
pub fn test_command_parsing_json_failure()
{
    let examples = vec![
        "{\"cmd\": \"write_to_disk\", db_key\": \"DB_KEY\"}",
        "{\"cmd\": \"read_from_disk\", \"db_key\": \"DB_KEY\"",
        "{\"cmd\": \"list_keys\", \"db_key\" \"DB_KEY\"}",
        "{cmd\": \"get_value\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\"}"];

    for example in examples
    {
        let r = Command::from_string(example);
        assert!(r.is_err());

        if !r.is_err()
        {
            panic!("Did not recieve an error for ${}$", example);
        }

        match &r.unwrap_err()
        {
            CommandParsingError::BadJSON(_) => {},
            CommandParsingError::BadPacket(s) => panic!("Recieved BadPacket({:?}) instead for ${}$", s, example),
            CommandParsingError::ArgumentsNotPresent(s) => panic!("Recieved ArgumentsNotPresent({:?}) instead for ${}$", s, example),
        }
    }
} 

#[test]
pub fn test_command_parsing_packet_failure()
{
    let examples = vec![
        "{\"cm\": \"write_to_disk\", \"db_key\": \"DB_KEY\"}",
        "{\"cmdTye\": \"read_from_disk\", \"db_key\": \"DB_KEY\"}",
        "{\"cmd\": [0, 1, 2, 3], \"db_key\": \"DB_KEY\"}",
        "{\"cmd\": {\"a\": 0, \"b\": 1},\"key\":\"KEY\", \"db_key\": \"DB_KEY\"}"];

    for example in examples
    {
        let r = Command::from_string(example);
        
        if !r.is_err()
        {
            panic!("Did not recieve an error for ${}$", example);
        }

        match &r.unwrap_err()
        {
            CommandParsingError::BadJSON(s) => panic!("Recieved BadJSON({:?}) instead for ${}$", s, example),
            CommandParsingError::BadPacket(_) => {},
            CommandParsingError::ArgumentsNotPresent(s) => panic!("Recieved ArgumentsNotPresent({:?}) instead for ${}$", s, example),
        }
    }
}

#[test]
pub fn test_command_parsing_packet_arguments_failure()
{
    let examples = vec![
        "{\"cmd\": \"write_to_disk\", \"db_ky\": \"DB_KEY\"}",
        "{\"cmd\": \"read_from_disk\", \"db_ke\": \"DB_KEY\"}",
        "{\"cmd\": \"list_keys\", \"db_ky\": \"DB_KEY\"}",
        "{\"cmd\": \"get_value\",\"ke\":\"KEY\", \"db_key\": \"DB_KEY\"}",
        "{\"cmd\": \"get_value\",\"key\":\"KEY\", \"db_ky\": \"DB_KEY\"}",

        "{\"cmd\": \"set_value\",\"ke\":\"KEY\", \"db_key\": \"DB_KEY\", \"val\":\"DATA\"}",
        "{\"cmd\": \"set_value\",\"key\":\"KEY\", \"db_ke\": \"DB_KEY\", \"val\":\"DATA\"}",
        "{\"cmd\": \"set_value\",\"key\":\"KEY\", \"db_key\": \"DB_KE\", \"va\":\"DATA\"}",

        "{\"cmd\": \"get_index\",\"ke\":\"KEY\", \"db_key\": \"DB_KEY\", \"index\":42}",
        "{\"cmd\": \"get_index\",\"key\":\"KEY\", \"db_ky\": \"DB_KEY\", \"index\":42}",
        "{\"cmd\": \"get_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"inde\":42}",

        "{\"cmd\": \"set_index\",\"ke\":\"KEY\", \"db_key\": \"DB_KEY\", \"val\":\"DATA\", \"index\":42}",
        "{\"cmd\": \"set_index\",\"key\":\"KEY\", \"db_ke\": \"DB_KEY\", \"val\":\"DATA\", \"index\":42}",
        "{\"cmd\": \"set_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"va\":\"DATA\", \"index\":42}",
        "{\"cmd\": \"set_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"val\":\"DATA\", \"inde\":42}",


        "{\"cmd\": \"append_list\",\"ke\":\"KEY\", \"db_key\": \"DB_KEY\", \"val\":\"DATA\"}",
        "{\"cmd\": \"append_list\",\"key\":\"KEY\", \"db_ke\": \"DB_KEY\", \"val\":\"DATA\"}",
        "{\"cmd\": \"append_list\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"va\":\"DATA\"}",

        "{\"cmd\": \"get_list_length\",\"ke\":\"KEY\", \"db_key\": \"DB_KEY\"}",
        "{\"cmd\": \"get_list_length\",\"key\":\"KEY\", \"db_ke\": \"DB_KEY\"}",

        "{\"cmd\": \"get_recent\",\"ke\":\"KEY\", \"db_key\": \"DB_KEY\", \"num\": 42}",
        "{\"cmd\": \"get_recent\",\"key\":\"KEY\", \"db_ke\": \"DB_KEY\", \"num\": 42}",
        "{\"cmd\": \"get_recent\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"nu\": 42}",

        "{\"cmd\": \"create_database\", \"db_ke\": \"DB_KEY\"}",

        "{\"cmd\": \"a_auth\", \"i\": \"ID\", \"token\":\"TOKEN\"}",
        "{\"cmd\": \"a_auth\", \"id\": \"ID\", \"tokn\":\"TOKEN\"}",

        "{\"cmd\": \"g_auth\", \"id_tokn\": \"ID_TOKEN\"}",

        "{\"cmd\": \"event\", \"event_i\":\"ID\", \"destination\":\"DEST\", \"origin\": \"ORIGIN\", \"data\": \"DATA\"}",
        "{\"cmd\": \"event\", \"event_id\":\"ID\", \"destinatio\":\"DEST\", \"origin\": \"ORIGIN\", \"data\": \"DATA\"}",
        "{\"cmd\": \"event\", \"event_id\":\"ID\", \"destination\":\"DEST\", \"origi\": \"ORIGIN\", \"data\": \"DATA\"}",
        "{\"cmd\": \"event\", \"event_id\":\"ID\", \"destination\":\"DEST\", \"origin\": \"ORIGIN\", \"dat\": \"DATA\"}"];

    for example in examples
    {
        let r = Command::from_string(example);
        
        if !r.is_err()
        {
            panic!("Did not recieve an error for ${}$", example);
        }

        match &r.unwrap_err()
        {
            CommandParsingError::BadJSON(s) => panic!("Recieved BadJSON({:?}) instead for ${}$", s, example),
            CommandParsingError::BadPacket(s) => panic!("Recieved BadPacket({:?}) instead for ${}$", s, example),
            CommandParsingError::ArgumentsNotPresent(_) => {}
        }
    }
}