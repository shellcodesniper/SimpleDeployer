use futures_util::stream::StreamExt;
use shiplift::{PullOptions};


use super::Docker;

#[derive(Debug, Clone, Default)]
pub struct LocalImageSearchResult {
  pub found: bool,
  pub image_url: String,
  pub image_tag: String,
  pub image_digest: Option<String>,
  pub image_id: Option<String>,
}

impl Docker {
  pub async fn get_local_image_digest(self, image_url: String, tag: Option<String>) -> LocalImageSearchResult {
    let search_target_tag= if let Some(tag) = tag { tag } else { String::from("latest") };

    match self.docker.images().list(&Default::default()).await {
      Ok(results) => {
        for result in results {
          // debug!("It's Result!!");
          // debug!("{:?}", result);
          let image_id = result.id.clone();
          let mut repo_image_digest: String = String::new();
          let repo_digest_result_list: Vec<String> = if let Some(rs) = result.repo_digests { rs } else { vec![] };
          // NOTE it can be empty

          for repo_digest in repo_digest_result_list {
            let repo_digest_split_collect: Vec<&str> = repo_digest.split('@').collect::<Vec<&str>>();
            if repo_digest_split_collect.len() < 2 {
              // NOTE maybe this is local image
              continue;
            }
            let mut repo_digest_split = repo_digest.split('@');
            let _ = repo_digest_split.next().unwrap();
            // NOTE this is image url part
            repo_image_digest = repo_digest_split.next().unwrap().to_string();
          }

          if result.repo_tags.is_some() {
            for repo_url in result.repo_tags.unwrap() {
              let mut repo_url_split = repo_url.split(':');
              let repo_image_url= repo_url_split.next().unwrap().to_string();
              let repo_image_tag= repo_url_split.next().unwrap().to_string();

              if image_url.clone() == repo_image_url && search_target_tag.clone() == repo_image_tag {
                let r = LocalImageSearchResult {
                  found: true,
                  image_url: image_url.clone(),
                  image_tag: search_target_tag.clone(),
                  image_digest: Some(repo_image_digest.clone()),
                  image_id: Some(image_id.clone()),
                };
                return r;
              }
            }
          }
        };
      }, Err(e) => {
        error!("Error: {}", e);
      }
    };
    debug!("Local Image Search Result");

    LocalImageSearchResult {
      found: false,
      image_url: image_url.clone(),
      image_tag: search_target_tag.clone(),
      image_digest: None,
      image_id: None,
    }
  }
  pub async fn download_image(self, image_url: String, tag: Option<String>) -> bool {
    debug!("Download!!");
    let image_download_tag = if tag.is_some() { tag.unwrap() } else { String::from("latest") };
    let image_download_url = format!("{}:{}", image_url, image_download_tag.clone()); 
    debug!("Downloading image: {}", image_download_url);

    let pull_opt = if self.auth.is_some() {
      PullOptions::builder().image(image_download_url).tag(image_download_tag).auth(self.auth.clone().unwrap()).build()
    } else {
      PullOptions::builder().image(image_download_url).tag("latest").build()
    };
    let mut stream = self.docker
        .images()
        .pull(&pull_opt);
      

    while let Some(pull_result) = stream.next().await {
        match pull_result {
            Ok(output) => println!("\t{:?}", output),
            Err(e) => error!("{}", e),
        }
    }
    true
  }

}