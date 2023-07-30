use mongodb::error::Result;
use mongodb::options::{ClientOptions, Credential, ServerApi, ServerApiVersion};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

pub async fn connect<'a, T>(collection: &str) -> Result<Collection<T>>
where
    T: Deserialize<'a> + Serialize,
{
    let mut client_options = ClientOptions::parse(
        "mongodb+srv://mokareads:<password>@core.luubkn0.mongodb.net/?retryWrites=true&w=majority",
    )
    .await?;
    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let credential = Credential::builder()
        .username("mokareads".to_string())
        .password("MK@Kublai12".to_string())
        .build();
    client_options.credential = Some(credential);

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    Ok(client.database("MokaReads").collection(collection))
}
