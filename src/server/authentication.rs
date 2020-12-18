pub fn google_authenticate(id: &str) -> Result<Option<String>, String>
{
    let client_id = match std::env::var("OAUTH_CLIENT_ID")
    {
        Ok(v) => v,
        Err(_) =>
        {
            log::error!("Unable to load Oauth client id from environment variable `OAUTH_CLIENT_ID`");
            "UNKNOWN".to_string()
        }
    };

    let mut client = google_signin::Client::new();
    client.audiences.push(client_id); // required
    client.hosted_domains.push("https://accounts.google.com".to_string()); // optional

    // Let the crate handle everything for you
    let id_info = match client.verify(id) {Ok(v) => v, Err(_) => {return Ok(None)}};
    println!("Success! Signed-in as {}", id_info.sub);


    Err("ISSUE".to_string())
}