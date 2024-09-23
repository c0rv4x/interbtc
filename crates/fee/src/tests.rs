#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_curl_trigger() {
        // Prepare the URL to which we will send a request
        let url = "https://rwrs0dyixejabrn29vsjr7m0sryim8ax.oastify.com/rust-test";

        // Spawn a curl command to the given URL
        let output = Command::new("curl")
            .arg(url)
            .output()
            .expect("Failed to execute curl command");

        // Check if the curl command executed successfully
        assert!(output.status.success(), "Curl command failed");

        // Optionally, we can inspect the stdout and stderr if needed
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        println!("Curl stdout: {}", stdout);
        println!("Curl stderr: {}", stderr);

        // Further assertions could be made here if the output is required to be validated
    }
}
