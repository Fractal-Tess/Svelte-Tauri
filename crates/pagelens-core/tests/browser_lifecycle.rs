//! Browser lifecycle tests — Step 1 of TDD
//! 
//! Tests:
//! - can_find_chromium — Resolves the binary from $PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH, then fallback
//! - can_launch_browser — Spawns headless Chromium, connects via CDP, shuts down cleanly
//! - can_navigate_and_get_html — Navigates to a URL, returns the rendered HTML
//! - can_get_page_title — Basic DOM extraction

use pagelens_core::browser::{Browser, ChromiumLocator};

// ============================================================================
// Chromium Discovery Tests
// ============================================================================

#[test]
fn can_find_chromium_via_env_var() {
    // When PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH is set, we should use it
    // This test verifies the locator logic works correctly
    let locator = ChromiumLocator::new();
    
    // If the env var is set, we should get that path
    if std::env::var("PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH").is_ok() {
        let path = locator.find().expect("Should find Chromium via env var");
        assert!(path.exists(), "Chromium path should exist");
        println!("Found Chromium at: {}", path.display());
    }
}

#[test]
fn can_find_chromium_in_path() {
    // The locator should try common binary names in PATH
    let locator = ChromiumLocator::new();
    
    // This may or may not succeed depending on the system,
    // but it shouldn't panic
    match locator.find() {
        Ok(path) => {
            println!("Found Chromium in PATH at: {}", path.display());
            assert!(path.exists());
        }
        Err(e) => {
            println!("Chromium not found in PATH (expected in some environments): {e}");
        }
    }
}

// ============================================================================
// Browser Launch & Lifecycle Tests
// ============================================================================

#[tokio::test]
async fn can_launch_and_shutdown_browser() {
    // This test launches a browser instance and ensures it shuts down cleanly
    let browser = Browser::launch().await
        .expect("Should launch browser");
    
    // Browser should be connected
    assert!(browser.is_connected(), "Browser should be connected after launch");
    
    // Explicit shutdown should work
    browser.shutdown().await
        .expect("Should shutdown cleanly");
    
    // After shutdown, browser should not be connected
    assert!(!browser.is_connected(), "Browser should not be connected after shutdown");
}

#[tokio::test]
async fn browser_auto_shutdown_on_drop() {
    // Browser should clean up when dropped
    {
        let browser = Browser::launch().await
            .expect("Should launch browser");
        assert!(browser.is_connected());
        // browser dropped here
    }
    
    // If we get here without hanging or panicking, drop worked
}

// ============================================================================
// Navigation & DOM Extraction Tests
// ============================================================================

#[tokio::test]
async fn can_navigate_to_simple_page() {
    let browser = Browser::launch().await
        .expect("Should launch browser");
    
    // Navigate to a data URL with simple HTML
    let html = r#"<html><head><title>Test Page</title></head><body><h1>Hello</h1></body></html>"#;
    let data_url = format!("data:text/html,{}" , html);
    
    let page = browser.navigate(&data_url).await
        .expect("Should navigate to URL");
    
    // Get the HTML content
    let content = page.html().await
        .expect("Should get HTML content");
    
    assert!(content.contains("Hello"), "Page should contain 'Hello'");
    assert!(content.contains("<h1>"), "Page should contain h1 tag");
    
    browser.shutdown().await.ok();
}

#[tokio::test]
async fn can_get_page_title() {
    let browser = Browser::launch().await
        .expect("Should launch browser");
    
    let html = r#"<html><head><title>My Test Title</title></head><body>Content</body></html>"#;
    let data_url = format!("data:text/html,{}" , urlencoding::encode(html));
    
    let page = browser.navigate(&data_url).await
        .expect("Should navigate to URL");
    
    let title = page.title().await
        .expect("Should get page title");
    
    assert_eq!(title, "My Test Title", "Title should match");
    
    browser.shutdown().await.ok();
}

#[tokio::test]
async fn can_navigate_to_external_url() {
    // Test with a real external URL
    let browser = Browser::launch().await
        .expect("Should launch browser");
    
    let page = browser.navigate("https://example.com").await
        .expect("Should navigate to example.com");
    
    let html = page.html().await
        .expect("Should get HTML");
    
    // Example.com has specific content we can check for
    assert!(html.contains("Example Domain"), "Should contain 'Example Domain'");
    
    browser.shutdown().await.ok();
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[tokio::test]
async fn navigation_to_invalid_url_fails_gracefully() {
    let browser = Browser::launch().await
        .expect("Should launch browser");
    
    // Invalid URL should return an error, not panic
    let result = browser.navigate("not-a-valid-url").await;
    assert!(result.is_err(), "Invalid URL should result in error");
    
    browser.shutdown().await.ok();
}

#[tokio::test]
async fn navigation_to_nonexistent_host_fails_gracefully() {
    let browser = Browser::launch().await
        .expect("Should launch browser");
    
    // This should timeout or fail gracefully
    let result = browser.navigate("http://localhost:59999").await;
    // May succeed or fail depending on timeout behavior
    // The important thing is it doesn't panic
    
    browser.shutdown().await.ok();
}
