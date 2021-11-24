use crate::lib::utils::registry as RegistryUtil;
use super::*;

pub struct IGetDigestResult {
  pub found: bool,
  pub digest: Option<String>,
  pub image_url: String,
  pub tag: String,
}


impl Registry {
  pub async fn get_digest_of_image(self, image_url: String, tag: Option<String>) -> IGetDigestResult {
    let search_target_tag = if tag.is_some() { tag.unwrap() } else { String::from("latest") };
    if self.token.is_none() {
      return IGetDigestResult {
        found: false,
        digest: None,
        image_url: image_url,
        tag: search_target_tag,
      }
    }

    let token = self.token.clone().unwrap_or(String::from(""));

    let [namespace, repository] = RegistryUtil::image_url_split(image_url.clone());

    let url = format!("{}/v2/namespaces/{}/repositories/{}/images?status=active&currently_tagged=1&page=1&page_size=100", self.registry_url, namespace, repository);
    let https = HttpsConnector::new();
    let client = Client::builder()
      .build::<_, hyper::Body>(https);

      
    
    let req = Request::builder()
      .method(Method::GET)
      .uri(url)
      .header("Authorization", format!("Bearer {}", token))
      .body(Body::from(""))
      .unwrap();
    let req = client.request(req).await.unwrap();
    
    let res_body =  hyper::body::to_bytes(req.into_body()).await.unwrap();
    let res_string = String::from_utf8(res_body.to_vec()).unwrap();
    let result = serde_json::from_str(&res_string);


    if result.is_ok() {
      let result: serde_json::Value = result.unwrap();
      if result.get("count").is_some() {
        let result = result.get("results").unwrap().as_array().unwrap();
        for result_idx in 0..result.len(){
          let target_row = result.get(result_idx).unwrap();
          let digest = target_row.get("digest").unwrap().as_str().unwrap();
          let tags = target_row.get("tags");
          if tags.is_some() {
            let tags = target_row.get("tags").unwrap().as_array().unwrap();
            for tag_idx in 0..tags.len() {
              let tag_target= tags.get(tag_idx).unwrap();
              let tag_str = tag_target.get("tag").unwrap().as_str().unwrap();
              let tag_is_current = tag_target.get("is_current").unwrap().as_bool().unwrap();
              // debug!("DIGEST: {}, TAG: {} {}", digest, tag_str, tag_is_current);

              if tag_is_current {
                if tag_str == &search_target_tag {
                  return IGetDigestResult {
                    found: true,
                    digest: Some(digest.to_string()),
                    image_url: image_url.clone(),
                    tag: search_target_tag,
                  }
                }
              }
            }
          }
        }
      }
    }


    return IGetDigestResult {
      found: false,
      digest: None,
      image_url: image_url,
      tag: search_target_tag,
    }
  }

}