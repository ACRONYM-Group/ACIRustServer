# ACI 2020.12 Proposal

ACI 2020.12 is a partial redesign of the ACI protocol focusing on cleaning up some of the protocol, and increasing internal consistency. Very little of the user facing side of ACI is changed in this proposal, instead a lot of focus is put on increasing consistency between the commands and responses.

## Areas of Focus

1. Command Names
2. Command Formats
3. Response Formats

## 1. Command Names

The proposed change does not change any of the functionality with the commands, but replaces some of the command names to make them more consistent.

The revised names are as follows:

* write_to_disk
* read_from_disk
* list_keys
* get_value
* set_value
* get_index
* set_index
* append_list
* get_list_length
* get_recent
* create_database
* a_auth
* g_auth
* event

## 2. Command Formats

### write_to_disk

The `write_to_disk` command writes a loaded database to disk in the .database/.item format. 

The command sent to the server is of the form

`{"cmd": "write_to_disk", "db_key": "DBKEY"}`

The `db_key` parameter must be a string and the name of a loaded database.

The server will repond with a response packet with the `cmd` field set to `"write_to_disk"` with the `db_key` field. The response to a proper execution would be

`{"cmd": "write_to_disk", "mode": "ok", "msg":"", "db_key":"DBKEY"}`

### read_from_disk

The `read_from_disk` command loads a database from disk in the .database/.item format.

The command sent to the server is of the form

`{"cmd": "read_from_disk", "db_key": "DBKEY"}`

The `db_key` parameter must be a string and the name of a loaded database.

The server will repond with a response packet with the `cmd` field set to `"read_from_disk"` with  the `db_key` field. The response to a proper execution would be

`{"cmd": "read_from_disk", "mode": "ok", "msg":"", "db_key":"DBKEY"}`

### list_keys

The `list_keys` command lists all of the keys available from a database with the given database key.

The command sent to the server is of the form

`{"cmd": "list_keys", "db_key": "DBKEY"}`

The `db_key` parameter must be a string and the name of a loaded database.

The server will repond with a response packet with the `cmd` field set to `"list_keys"` with the `db_key` field, and a `val` field filled with a list of keys. The response to a proper execution would be

`{"cmd": "list_keys", "mode": "ok", "msg":"", "db_key":"DBKEY", "val": ["key0", "key1", ...]}`

### get_value

The `get_value` command gets a value from the given key in the given database.

The command sent to the server is of the form

`{"cmd": "get_value", "key": "ITEMKEY", "db_key": "DBKEY"}`

The `db_key` parameter must be a string and the name of a loaded database, and the `key` parameter must be a string and the name of the key in the database.

The server will repond with a response packet with the `cmd` field set to `"get_value"` with the `db_key` field and the `key` field, and a `val` field filled with the data read from the key. The response to a proper execution would be

`{"cmd": "get_value", "mode": "ok", "msg":"", "db_key":"DBKEY", "key": "ITEMKEY", "val": Value}`

### set_value

The `set_value` command sets the value for the given key in the given database, or if the key does not exist, it will create that key and write the value, in addition to setting default permissions.

The command sent to the server is of the form

`{"cmd": "set_value", "key": "ITEMKEY", "db_key": "DBKEY", "val": Value}`

The `db_key` parameter must be a string and the name of a loaded database, and the `key` parameter must be a string and the name of the key in the database, and the `val` parameter is the value to be written.

The server will repond with a response packet with the `cmd` field set to `"set_value"` with the `db_key` field and the `key` field. The response to a proper execution would be

`{"cmd": "set_value", "mode": "ok", "msg":"", "db_key":"DBKEY", "key": "ITEMKEY"}`

### get_index

The `get_index` command gets the value at an index in the given key in the given database.

The command sent to the server is of the form

`{"cmd": "get_index", "key": "ITEMKEY", "db_key": "DBKEY", "index": Index}`

The `db_key` parameter must be a string and the name of a loaded database, and the `key` parameter must be a string and the name of the key in the database

The server will repond with a response packet with the `cmd` field set to `"get_index"` with the `db_key` field, the `key` field, the `index` field, and a `val` field filled with the data read from the key. The response to a proper execution would be

`{"cmd": "get_index", "mode": "ok", "msg":"", "db_key":"DBKEY", "key": "ITEMKEY", "index": Index, "val": Value}`

### set_index

The `set_index` command sets the value at an index in the given key in the given database. If the index does not exist, it will expand the array with Nulls and then fill the index. This will only work properly if the key exists in the database.

The command sent to the server is of the form 

`{"cmd": "set_index", "key": "ITEMKEY", "db_key": "DBKEY", "index": Index, "val": Value}`

The `db_key` parameter must be a string and the name of a loaded database, and the `key` parameter must be a string and the name of the key in the database, the `val` parameter is the value to be written, and the `index` parameter must be an integer and the index into the key in the database.

The server will repond with a response packet with the `cmd` field set to `"set_index"` with the `db_key` field, the `key` field, and the `index` field. The response to a proper execution would be

`{"cmd": "set_index", "mode": "ok", "msg":"", "db_key":"DBKEY", "key": "ITEMKEY", "index": Index}`

### append_list

The `append_list` command appends a value to the list stored in the given key in the given database.

The command sent to the server is of the form

`{"cmd": "append_list", "key": "ITEMKEY", "db_key": "DBKEY", "val": Value}`

The `db_key` parameter must be a string and the name of a loaded database, and the `key` parameter must be a string and the name of the key in the database, and the `val` parameter is the value to be appended.

The server will repond with a response packet with the `cmd` field set to `"append_list"` with the `db_key` field, the `key` field, and the `index` field filled with the index of the item appended to the list. The response to a proper execution would be

`{"cmd": "append_list", "mode": "ok", "msg":"", "db_key":"DBKEY", "key": "ITEMKEY", "index": Index}`

### get_list_length

The `get_list_length` command gets the length of the list stored in the given key in the given database.

The command sent to the server is of the form

`{"cmd": "get_list_length", "key": "ITEMKEY", "db_key": "DBKEY"}`

The `db_key` parameter must be a string and the name of a loaded database, and the `key` parameter must be a string and the name of the key in the database.

The server will repond with a response packet with the `cmd` field set to `"get_list_length"` with the `db_key` field, the `key` field, and the `length` field filled with the length of the list as a number. The response to a proper execution would be

`{"cmd": "get_list_length", "mode": "ok", "msg":"","db_key":"DBKEY", "key": "ITEMKEY", "length": Length}`

### get_recent

The `get_recent` command gets the last `num` items from the list stored in the given key in the given database. If `num` is greater than the length of the list, the list will be returned. The list will be reversed however such that the last item in the list in the database is the first in the returned list. 

The command sent to the server is of the form

`{"cmd": "get_recent", "key": "ITEMKEY", "db_key": "DBKEY", "num": Num}`

The `db_key` parameter must be a string and the name of a loaded database,the `key` parameter must be a string and the name of the key in the database, and then `num` parameter must be an integer and be the number of items expected.

The server will repond with a response packet with the `cmd` field set to `"get_recent"` with the `db_key` field, the `key` field, and the `val` field filled with an list holding the last `num` values in the list. The response to a proper execution would be

`{"cmd": "get_recent", "mode": "ok", "msg":"", "db_key":"DBKEY", "key": "ITEMKEY", "val": Value}`

### create_database

The `create_database` command creates a new database with the given name.

The command sent to the server is of the form

`{"cmd": "create_database", "db_key": "DBKEY"}`

The `db_key` parameter must be a string.

The server will repond with a response packet with the `cmd` field set to `"get_recent"` with the `db_key` field. The response to a proper execution would be

`{"cmd": "create_database", "mode": "ok", "msg": "", "db_key": "DBKEY"}`

### a_auth

The `a_auth` command authenticates the connection via the ACI authentication protocol.

The command sent to the server is of the form

`{"cmd": "a_auth", "id": "ID", "token": "TOKEN"}`

The `id` parameter must be a string and be the id of the user authenticating, and the `token` parameter must be a string and be the token for the user authenticating.

The server will repond with a response packet with the `cmd` field set to `"a_auth"`, no other information is required as only one authentication is required per connection. The response to a proper execution would be

`{"cmd": "a_auth", "mode": "ok", msg: ""}`

### g_auth

The `g_auth` command authenticates the connection via the google oauth authentication protocol.

The command sent to the server is of the form

`{"cmd": "g_auth", "id_token": "IDTOKEN"}`

The `id_token` parameter must be a string and be the id token of the user authenticating.

The server will repond with a response packet with the `cmd` field set to `"g_auth"`, no other information is required as only one authentication is required per connection. The response to a proper execution would be

`{"cmd": "g_auth", "mode": "ok", msg: ""`

### event

The `event` command relays its package to other connections to the client, the relayed packet should be completely identical to the recieved packet.

The command sent to the server is of the form

`{"cmd": "event", "event_id": "EVENTID", "destination": "DEST", "origin": "ORIGIN", data: Value}`

The `event_id` parameter must be a string, the `destination` parameter must be a string and be the id of the destination of the event transmission, the `origin` parameter must be a string and be the id of the origin of the event transmission, and the `data` parameter is any json object.

The server will respond with an `"ack"` packet with the `event_id` and `origin` parameters if the event makes it to the server. This response would be

`{"cmd": "event", "mode": "ack", "event_id": "EVENTID", "origin": "ORIGIN"}`

## 3. Response formats

All response packets are of the form

`{"cmd": "cmd", "mode": "MODE", "msg": "Message"...}`

Where the `cmd` field is the original command sent to the server.

The `mode` field is a string set to one of 

* `"ack"`
* `"error"`
* `"ok"`

to denote what type of response is being returned.

The `msg` field returns any acknowledgement or error message to the client to relay any information from the server in the form of a string.

In addition to these fields, there are some 

An `"error"` response is expected to have the `cmd`, `mode`, and `msg` fields filled, and an `"ack"` response is expected to have the `cmd`, and `mode` fields filled, along with any of the necessary arguments.

### Note

Note that the `"ack"` response is reserved for commands like `event` where the server cannot determine if the message has been recieved correctly. Any command which is directed at the server should use the `"ok"` response instead.