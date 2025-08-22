use crate::utils::create_two_cats;
use crate::utils::spawn_app;
use anyhow::Context;
use anyhow::Result;
use gha_demo::types::v1::types::Cat;
use gha_demo::types::v1::types::EyeColor;
use reqwest::StatusCode;
use serde::Serialize;
use uuid::Uuid;

#[tokio::test]
pub async fn test_get_all_cats_empty() -> Result<()> {
    // spawn our app
    let app = spawn_app().await.context("spawn testing app")?;

    // send the request
    let endpoint = format!("{}/v1/cats", app.address);
    let resp = app
        .api_client
        .get(endpoint)
        .send()
        .await
        .context("send request")?;

    // check status
    assert_eq!(resp.status(), StatusCode::OK);

    // get cats
    let cats: Vec<Cat> = resp.json().await?;

    // should be 0 cats
    assert!(cats.is_empty());

    Ok(())
}

#[tokio::test]
pub async fn test_get_all_cats() -> Result<()> {
    // spawn our app
    let app = spawn_app().await.context("spawn testing app")?;

    // normally it would be good just to have test cats be a part of the app
    // this is ok for demo sake
    create_two_cats(&app.db_pool).await?;

    // send the request
    let endpoint = format!("{}/v1/cats", app.address);
    let resp = app
        .api_client
        .get(endpoint)
        .send()
        .await
        .context("send request")?;

    // check status
    assert_eq!(resp.status(), StatusCode::OK);

    // get cats
    let cats: Vec<Cat> = resp.json().await?;

    // should be 0 cats
    assert_eq!(cats.len(), 2);

    Ok(())
}

#[tokio::test]
pub async fn test_get_cat() -> Result<()> {
    // spawn our app
    let app = spawn_app().await.context("spawn testing app")?;

    // normally it would be good just to have test cats be a part of the app
    // this is ok for demo sake
    let [cat1, _] = create_two_cats(&app.db_pool).await?;

    // send the request
    let endpoint = format!("{}/v1/cats/{}", app.address, cat1.cool_cat_club_id);
    let resp = app
        .api_client
        .get(endpoint)
        .send()
        .await
        .context("send request")?;

    // check status
    assert_eq!(resp.status(), StatusCode::OK);

    // get cat
    let cat: Cat = resp.json().await?;

    // should be 0 cats
    assert_eq!(cat1, cat);

    Ok(())
}

#[tokio::test]
pub async fn test_get_cat_not_found() -> Result<()> {
    // spawn our app
    let app = spawn_app().await.context("spawn testing app")?;

    // send the request
    let endpoint = format!("{}/v1/cats/{}", app.address, Uuid::nil());
    let resp = app
        .api_client
        .get(endpoint)
        .send()
        .await
        .context("send request")?;

    // check status
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    Ok(())
}

#[tokio::test]
pub async fn test_create_valid_cat() -> Result<()> {
    // spawn our app
    let app = spawn_app().await.context("spawn testing app")?;

    // cat
    let cat = Cat {
        name: "maisy".to_string(),
        cool_cat_club_id: Uuid::new_v4(),
        age: 3,
        eye_color: EyeColor::Blue,
    };

    // send the request
    let post_endpoint = format!("{}/v1/cats", app.address);
    let resp = app
        .api_client
        .post(post_endpoint)
        .json(&cat)
        .send()
        .await
        .context("send request")?;

    // check status
    assert_eq!(resp.status(), StatusCode::CREATED);

    // get the cat using the API for good measure
    let get_endpoint = format!("{}/v1/cats/{}", app.address, cat.cool_cat_club_id);
    let resp = app
        .api_client
        .get(get_endpoint)
        .send()
        .await
        .context("send request")?;

    assert_eq!(resp.status(), StatusCode::OK);

    // get cat
    let gotten_cat: Cat = resp.json().await?;

    // check
    assert_eq!(cat, gotten_cat);

    Ok(())
}

// normally would be good to put this in its own file

#[derive(Serialize, Debug)]
struct TestCat {
    pub name: Option<String>,
    pub cool_cat_club_id: Option<Uuid>,
    pub age: Option<i16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eye_color: Option<EyeColor>,
}

impl Default for TestCat {
    fn default() -> Self {
        Self {
            name: Some("KITTY".to_string()),
            cool_cat_club_id: Some(Uuid::new_v4()),
            age: Some(5),
            eye_color: Some(EyeColor::Blue),
        }
    }
}

impl TestCat {
    fn with_name(mut self, new_name: Option<String>) -> Self {
        self.name = new_name;
        self
    }

    fn with_cool_cat_club_id(mut self, new_id: Option<Uuid>) -> Self {
        self.cool_cat_club_id = new_id;
        self
    }

    fn with_age(mut self, new_age: Option<i16>) -> Self {
        self.age = new_age;
        self
    }

    fn with_eye_color(mut self, new_eye_color: Option<EyeColor>) -> Self {
        self.eye_color = new_eye_color;
        self
    }
}

#[tokio::test]
pub async fn test_create_invalid_cat() -> Result<()> {
    // spawn our app
    let app = spawn_app().await.context("spawn testing app")?;

    let post_endpoint = format!("{}/v1/cats", app.address);

    // cat
    let mut cases: Vec<(TestCat, &str)> = Vec::new();
    cases.push((TestCat::default().with_name(None), "Missing Name"));
    cases.push((TestCat::default().with_cool_cat_club_id(None), "Missing ID"));
    cases.push((TestCat::default().with_age(None), "Missing Age"));
    cases.push((TestCat::default().with_eye_color(None), "Missing Eye Color"));

    for (cat, msg) in cases {
        // send the request
        let resp = app
            .api_client
            .post(&post_endpoint)
            .json(&cat)
            .send()
            .await
            .context("send request")?;

        assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY, "{msg}");
    }

    Ok(())
}
