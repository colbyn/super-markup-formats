use std::process::{Command, Stdio};
use std::io::Write;
use std::fmt;

#[derive(Debug)]
pub enum HtmlPrettifyError {
    TidyNotInstalled,
    TidyExecutionFailed(String),
    Utf8ConversionError(std::string::FromUtf8Error),
}

impl fmt::Display for HtmlPrettifyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HtmlPrettifyError::TidyNotInstalled => write!(f, "The 'tidy' CLI tool is not installed or not found in PATH."),
            HtmlPrettifyError::TidyExecutionFailed(msg) => write!(f, "Failed to prettify HTML: {}", msg),
            HtmlPrettifyError::Utf8ConversionError(e) => write!(f, "UTF-8 error: {}", e),
        }
    }
}

impl std::error::Error for HtmlPrettifyError {}

/// Prettifies HTML using the `tidy` CLI tool.
pub fn prettify_html(html_str: &str) -> Result<String, HtmlPrettifyError> {
    // let mut child = Command::new("tidy")
    //     .args(&[
    //         "-quiet",          // suppress warnings
    //         "--show-warnings",
    //         "no",
    //         "-indent",         // pretty print
    //         "-wrap", "120",    // wrap lines at 120 chars
    //         "--tidy-mark", "no", // do not insert the generator meta tag
    //         "-as-html",        // treat input as HTML
    //         "-utf8",           // set output encoding
    //     ])
    //     .stdin(Stdio::piped())
    //     .stdout(Stdio::piped())
    //     .stderr(Stdio::null()) // ignore stderr unless debugging
    //     .spawn()
    //     .map_err(|e| {
    //         eprintln!("WARNING: {e}");
    //         HtmlPrettifyError::TidyNotInstalled
    //     })?;

    // -quiet --show-warnings no -indent -wrap 120 --tidy-mark no -as-html -utf8 --custom-tags blocklevel --drop-empty-elements no
    let mut child = Command::new("tidy")
        .args(&[
            "-quiet",                      // suppress non-critical output
            "--show-warnings", "no",       // don't show warnings
            "-indent",                     // pretty print
            "-wrap", "120",                // wrap lines at 120 chars
            "--tidy-mark", "no",           // do not insert generator meta tag
            "-ashtml",                     // treat input as HTML (not XHTML)
            "--output-html", "yes",
            "-utf8",                       // UTF-8 output
            "--custom-tags", "blocklevel", // treat custom tags like <wow-image> as valid
            "--drop-empty-elements", "no", // preserve empty tags
            // "--force-output", "yes",       // Tidy should produce output even if errors are encountered.
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            eprintln!("⚠️ Warning [tidy-not-installed]: {e}");
            HtmlPrettifyError::TidyNotInstalled
        })?;
    
    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(html_str.as_bytes()).unwrap();

    // if let Some(stdin) = child.stdin.as_mut() {
    //     stdin.write_all(html.as_bytes()).unwrap();
    // }

    // - OUTPUT -
    let output = child
        .wait_with_output()
        .map_err(|e| {
            eprintln!("⚠️ Warning [tidy-execution-failed]: {e}");
            HtmlPrettifyError::TidyExecutionFailed(e.to_string())
        })?;

    // if !output.status.success() {
    //     eprintln!("WARNING: TIDY CLI FAILED");
    //     return Err(HtmlPrettifyError::TidyExecutionFailed(format!(
    //         "Exit code: {}",
    //         output.status
    //     )));
    // }
    
    // - GET STDOUT/STDERR -
    // let stdout = String::from_utf8(output.stdout.clone()).map_err(HtmlPrettifyError::Utf8ConversionError);
    // let stderr = String::from_utf8(output.stderr.clone()).map_err(HtmlPrettifyError::Utf8ConversionError);

    let stdout = String::from_utf8(output.stdout.clone()).unwrap();
    let stderr = String::from_utf8(output.stderr.clone()).unwrap();
    
    // - STATUS -
    let status = output.status.code().unwrap_or(-1);
    if status != 0 && status != 1 {
        eprintln!(
            "⚠️ Warning [tidy-execution-failed]: tidy cli failed » [stderr]: {:?}",
            stderr,
        );
        return Err(HtmlPrettifyError::TidyExecutionFailed(format!(
            "Exit code: {}",
            status
        )));
    }
    Ok(stdout)
}

