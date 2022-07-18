use crate::*;

use dcl_common::{Parcel, Result};

use std::fs::File;

use std::io::Write;
use std::path::Path;
use serde::{Serialize, Deserialize};

/// `ContentClient` implements all the request to interact with [Catalyst Content Servers](https://decentraland.github.io/catalyst-api-specs/#tag/Content-Server).
///
#[derive(Default)]
pub struct ContentClient {}

#[derive(Serialize)]
struct ParcelPointer<'a> {
    pointers: &'a Vec<Parcel>
}

impl ContentClient {
    pub async fn entity_information(server: &Server, entity: &Entity) -> Result<EntityInformation>
    {
        let result = server.get(
            format!("/content/audit/{}/{}", entity.kind, entity.id)
        ).await?;
        Ok(result)
    }

    pub async fn status(server: &Server) -> Result<ContentServerStatus>
    {
        let result = server.get("/content/status").await?;
        Ok(result)
    }

    pub async fn snapshot(server: &Server) -> Result<Snapshot>
    {
        let result = server.get("/content/snapshot").await?;
        Ok(result)
    }

    pub async fn snapshot_entities<T>(server: &Server, entity_type: EntityType, snapshot: &Snapshot ) -> Result<T>
        where T: for<'a> Deserialize<'a>
    {

        let hash : &ContentId = match entity_type {
            EntityType::Scene => &snapshot.entities.scene.hash,
            EntityType::Profile => &snapshot.entities.profile.hash,
            EntityType::Wearable => &snapshot.entities.wearable.hash
        };

        let result = server.raw_get(format!("/content/contents/{}", hash)).await?;

        let text = result.text().await?;
        // let result : Vec<T> = vec!();

        text.split('\n').for_each(|json| panic!("{}", json) );
        // let result =
        let result : T = serde_json::from_str(&text)?;
        Ok(result)
    }

    pub async fn scene_files_for_parcels(server: &Server, parcels: &Vec<Parcel>) -> Result<Vec<SceneFile>>
    {
        let pointers = ParcelPointer { pointers: parcels };        
        let result : Vec<SceneFile> = server.post(
            "/content/entities/active",
            &pointers
            ).await?;
        Ok(result)
    }

    pub async fn download<U, V>(server: &Server, hash_id: U, filename: V) -> Result<()>
    where
        U: AsRef<str> + std::fmt::Display,
        V: AsRef<Path>,
    {
        let response = server
            .raw_get(format!("/content/contents/{}", hash_id))
            .await?;
        let mut dest = File::create(filename)?;
        let content = response.bytes().await?;
        dest.write_all(&content)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dcl_common::Parcel;
    
    use httpmock::prelude::*;
    use std::fs;
    use tempdir::TempDir;

    #[test]
    fn it_implements_active_entities() {
        //TODO(fran): We want to use /content/entities/active to download
        //            entity files. It should support any serializable to
        //            be able to create the request using arbitrary lists of
        //            pointers or ids
        //            Also add examples to consume this endpoints.
        // assert!(false)
    }

    #[test]
    fn it_gets_scene_files_from_parcels() {
        let response = include_str!("../fixtures/scenes_from_parcels.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(POST)
                .path("/content/entities/active")
                .body_contains("{\"pointers\":[\"0,0\"]}");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let parcels = vec![Parcel(0,0)];
        let result : Vec<SceneFile> = tokio_test::block_on(
            ContentClient::scene_files_for_parcels(&server, &parcels)
        ).unwrap();

        m.assert();

        let expected : Vec<SceneFile> = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_gets_entity_information() {
        let response = include_str!("../fixtures/audit_scene_result.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET)
                .path("/content/audit/scene/id");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));
        let entity = Entity::scene("id");
        let result : EntityInformation = tokio_test::block_on(
            ContentClient::entity_information(&server, &entity)
        ).unwrap();

        m.assert();

        let expected : EntityInformation = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_gets_status() {
        let response = include_str!("../fixtures/content_server_status.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET)
                .path("/content/status");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));
        let result : ContentServerStatus = tokio_test::block_on(
            ContentClient::status(&server)
        ).unwrap();

        m.assert();

        let expected : ContentServerStatus = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_gets_snapshot() {
        let response = include_str!("../fixtures/snapshot.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET)
                .path("/content/snapshot");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));
        let result : Snapshot = tokio_test::block_on(
            ContentClient::snapshot(&server)
        ).unwrap();

        m.assert();

        let expected : Snapshot = serde_json::from_str(response).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_gets_snapshot_entities() {
        let response = include_str!("../fixtures/snapshot_entities_scene.json");
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET)
                .path("/content/contents/bafybeiep3b54f6rzh5lgx647m4alfydi65smdz63y4gtpxnu2ero4trlsy");
            then.status(200).body(response);
        });
        let server = Server::new(server.url(""));
        let snapshot : Snapshot = serde_json::from_str(
            include_str!("../fixtures/snapshot.json")
        ).unwrap();

        let result : Vec<SceneSnapshot> = tokio_test::block_on(
            ContentClient::snapshot_entities(
                &server,
                EntityType::Scene,
                &snapshot
            )
        ).unwrap();

        m.assert();

        // let expected : Snapshot = serde_json::from_str(response).unwrap();
        // assert_eq!(result, expected);
        assert!(false);
    }

    #[test]
    fn it_downloads_file() {
        let response = "File Content";

        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.path("/content/contents/a-hash");
            then.status(200).body(response);
        });

        let server = Server::new(server.url(""));

        let tmp_dir = TempDir::new("content-client-test").unwrap();
        let filename = tmp_dir.path().join("test.txt");
        // TODO(fran): use content file for download
        // let content_file = ContentFile {
        //     filename: filename.clone(),
        //     cid: ContentId::new("a-hash")
        // };

        tokio_test::block_on(
            ContentClient::download(&server, "a-hash", filename.clone())
        ).unwrap();

        m.assert();

        assert_eq!(fs::read_to_string(filename).unwrap(), "File Content");
    }
}
