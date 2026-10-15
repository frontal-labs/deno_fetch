use deno_fetch::Options;
use deno_fetch::basic_auth;
use deno_fetch::extract_authority;
use url::Url;

#[test]
fn extracts_and_removes_url_authority_credentials() {
  let mut url = Url::parse("https://user:p%40ss@example.com/path").unwrap();
  assert_eq!(
    extract_authority(&mut url),
    Some(("user".to_string(), Some("p@ss".to_string())))
  );
  assert_eq!(url.as_str(), "https://example.com/path");
  assert_eq!(extract_authority(&mut url), None);
}

#[test]
fn creates_proxy_basic_auth_header_values() {
  assert_eq!(
    basic_auth("alice", Some("secret")).to_str().unwrap(),
    "Basic YWxpY2U6c2VjcmV0"
  );
  assert_eq!(
    basic_auth("alice", None).to_str().unwrap(),
    "Basic YWxpY2U6"
  );
}

#[test]
fn default_fetch_options_have_no_custom_roots() {
  let options = Options::default();
  assert!(options.root_cert_store().unwrap().is_none());
  assert!(options.proxy.is_none());
  assert!(options.unsafely_ignore_certificate_errors.is_none());
}
