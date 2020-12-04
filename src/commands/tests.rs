//! Tests for the commands

use super::{Commands, CommandParsingError, Command};

fn test_command_parsing(test_output: bool)
{
    let examples = vec![
        "{\"cmdType\": \"wtd\", \"db_key\": \"DB_KEY\"}",
        "{\"cmdType\": \"rfd\", \"db_key\": \"DB_KEY\"}",
        "{\"cmdType\": \"list_databases\", \"db_key\": \"DB_KEY\"}",
        "{\"cmdType\": \"get_val\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\"}",
        "{\"cmdType\": \"set_val\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"val\":\"DATA\"}",
        "{\"cmdType\": \"get_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"index\":42}",
        "{\"cmdType\": \"set_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"val\":\"DATA\", \"index\":42}",
        "{\"cmdType\": \"append_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"val\":\"DATA\"}",
        "{\"cmdType\": \"get_len_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\"}",
        "{\"cmdType\": \"get_recent_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"num\": 42}",
        "{\"cmdType\": \"cdb\", \"db_key\": \"DB_KEY\"}",
        "{\"cmdType\": \"a_auth\", \"id\": \"ID\", \"token\":\"TOKEN\"}",
        "{\"cmdType\": \"g_auth\", \"id_token\": \"ID_TOKEN\"}",
        "{\"cmdType\": \"event\", \"event_id\":\"ID\", \"destination\":\"DEST\", \"origin\": \"ORIGIN\", \"data\": \"DATA\"}"];

    let cmd_types = vec![Commands::WriteToDisk, Commands::ReadFromDisk, Commands::ListDatabases,
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
        "{\"cmdType\": \"wtd\", db_key\": \"DB_KEY\"}",
        "{\"cmdType\": \"rfd\", \"db_key\": \"DB_KEY\"",
        "{\"cmdType\": \"list_databases\", \"db_key\" \"DB_KEY\"}",
        "{cmdType\": \"get_val\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\"}"];

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
        "{\"cmdType\": \"wt\", \"db_key\": \"DB_KEY\"}",
        "{\"cmd\": \"rfd\", \"db_key\": \"DB_KEY\"}",
        "{\"cmdType\": [0, 1, 2, 3], \"db_key\": \"DB_KEY\"}",
        "{\"cmdType\": {\"a\": 0, \"b\": 1},\"key\":\"KEY\", \"db_key\": \"DB_KEY\"}"];

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
        "{\"cmdType\": \"wtd\", \"db_ky\": \"DB_KEY\"}",
        "{\"cmdType\": \"rfd\", \"db_ke\": \"DB_KEY\"}",
        "{\"cmdType\": \"list_databases\", \"db_ky\": \"DB_KEY\"}",
        "{\"cmdType\": \"get_val\",\"ke\":\"KEY\", \"db_key\": \"DB_KEY\"}",
        "{\"cmdType\": \"get_val\",\"key\":\"KEY\", \"db_ky\": \"DB_KEY\"}",

        "{\"cmdType\": \"set_val\",\"ke\":\"KEY\", \"db_key\": \"DB_KEY\", \"val\":\"DATA\"}",
        "{\"cmdType\": \"set_val\",\"key\":\"KEY\", \"db_ke\": \"DB_KEY\", \"val\":\"DATA\"}",
        "{\"cmdType\": \"set_val\",\"key\":\"KEY\", \"db_key\": \"DB_KE\", \"va\":\"DATA\"}",

        "{\"cmdType\": \"get_index\",\"ke\":\"KEY\", \"db_key\": \"DB_KEY\", \"index\":42}",
        "{\"cmdType\": \"get_index\",\"key\":\"KEY\", \"db_ky\": \"DB_KEY\", \"index\":42}",
        "{\"cmdType\": \"get_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"inde\":42}",

        "{\"cmdType\": \"set_index\",\"ke\":\"KEY\", \"db_key\": \"DB_KEY\", \"val\":\"DATA\", \"index\":42}",
        "{\"cmdType\": \"set_index\",\"key\":\"KEY\", \"db_ke\": \"DB_KEY\", \"val\":\"DATA\", \"index\":42}",
        "{\"cmdType\": \"set_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"va\":\"DATA\", \"index\":42}",
        "{\"cmdType\": \"set_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"val\":\"DATA\", \"inde\":42}",


        "{\"cmdType\": \"append_index\",\"ke\":\"KEY\", \"db_key\": \"DB_KEY\", \"val\":\"DATA\"}",
        "{\"cmdType\": \"append_index\",\"key\":\"KEY\", \"db_ke\": \"DB_KEY\", \"val\":\"DATA\"}",
        "{\"cmdType\": \"append_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"va\":\"DATA\"}",

        "{\"cmdType\": \"get_len_index\",\"ke\":\"KEY\", \"db_key\": \"DB_KEY\"}",
        "{\"cmdType\": \"get_len_index\",\"key\":\"KEY\", \"db_ke\": \"DB_KEY\"}",

        "{\"cmdType\": \"get_recent_index\",\"ke\":\"KEY\", \"db_key\": \"DB_KEY\", \"num\": 42}",
        "{\"cmdType\": \"get_recent_index\",\"key\":\"KEY\", \"db_ke\": \"DB_KEY\", \"num\": 42}",
        "{\"cmdType\": \"get_recent_index\",\"key\":\"KEY\", \"db_key\": \"DB_KEY\", \"nu\": 42}",

        "{\"cmdType\": \"cdb\", \"db_ke\": \"DB_KEY\"}",

        "{\"cmdType\": \"a_auth\", \"i\": \"ID\", \"token\":\"TOKEN\"}",
        "{\"cmdType\": \"a_auth\", \"id\": \"ID\", \"tokn\":\"TOKEN\"}",

        "{\"cmdType\": \"g_auth\", \"id_tokn\": \"ID_TOKEN\"}",

        "{\"cmdType\": \"event\", \"event_i\":\"ID\", \"destination\":\"DEST\", \"origin\": \"ORIGIN\", \"data\": \"DATA\"}",
        "{\"cmdType\": \"event\", \"event_id\":\"ID\", \"destinatio\":\"DEST\", \"origin\": \"ORIGIN\", \"data\": \"DATA\"}",
        "{\"cmdType\": \"event\", \"event_id\":\"ID\", \"destination\":\"DEST\", \"origi\": \"ORIGIN\", \"data\": \"DATA\"}",
        "{\"cmdType\": \"event\", \"event_id\":\"ID\", \"destination\":\"DEST\", \"origin\": \"ORIGIN\", \"dat\": \"DATA\"}"];

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