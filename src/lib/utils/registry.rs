  pub fn image_url_split(image_url: String) -> [String; 2] {
    if image_url.contains('/') {
      let mut split = image_url.split("/");
      let namespace = split.next().unwrap().to_string();
      let repository = split.next().unwrap().to_string();
      return [namespace, repository];
    }
    return [image_url.clone(), image_url.clone()];
  }