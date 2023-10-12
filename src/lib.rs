// TODO: Make these configurable through env variables.
pub const GITHUB_ORG: &str = "keep-starknet-strange";
pub const GITHUB_REPO: &str = "where_is_starknet";
pub const ISSUE_TEMPLATE_FILE: &str = "STARKNET_MISSING.yml";
pub const SUBMIT_URL_TEMPLATE : &str = "https://github.com/{}/{}/issues/new?assignees=&labels=triage%2Chelp+wanted&projects=&template={}&title=%5BAdd+Starknet+to%5D%3A+&website={}&additional-information={}&exact-url={}";

/// Generate a URL to submit an issue to the Where is Starknet repo.
/// The URL will be pre-filled with the website and exact URL provided.
/// The issue will be labeled as `triage` and `help wanted`.
/// The issue will be assigned to no one.
/// # Arguments
/// * `website` - The website where Starknet is missing.
/// * `exact_url` - The exact URL where Starknet is missing.
/// # Example
/// ```
/// use where_is_starknet_bot::generate_submit_issue_url;
/// let website = "https://www.example.com";
/// let exact_url = "https://www.example.com/missing";
/// let submit_url = generate_submit_issue_url(website, exact_url);
/// ```
/// # Returns
/// A URL to submit an issue to the Where is Starknet repo.
pub fn generate_submit_issue_url(website: &str, exact_url: &str) -> String {
    let additional_information = "";
    format!(
        "https://github.com/{}/{}/issues/new?assignees=&labels=triage%2Chelp+wanted&projects=&template={}&title=%5BAdd+Starknet+to%5D%3A+&website={}&additional-information={}&exact-url={}",
        GITHUB_ORG,
        GITHUB_REPO,
        ISSUE_TEMPLATE_FILE,
        website,
        additional_information,
        exact_url,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_submit_issue_url() {
        let website = "https://www.example.com";
        let exact_url = "https://www.example.com/missing";
        let submit_url = generate_submit_issue_url(website, exact_url);
        assert!(submit_url.contains(website));
        assert!(submit_url.contains(exact_url));
    }
}
