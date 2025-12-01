use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::Action;
use proxy_wasm::types::{ContextType, LogLevel};
use std::sync::Arc;
use ipnet_trie::IpnetTrie;
use ipnet::IpNet;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(MyRootContext {
            blocklist: Arc::new(IpnetTrie::new()),
        })
    });
}}

struct MyRootContext {
    blocklist: Arc<IpnetTrie<u8>>,
}

impl Context for MyRootContext {}

impl RootContext for MyRootContext {
    fn on_configure(&mut self, _: usize) -> bool {
        let config_bytes = match self.get_plugin_configuration() {
            Some(bytes) => bytes,
            None => {
                info!("on_configure:No plugin configuration found. Blocklist will remain empty.");
                return true; // No config is not an error, just means no IPs to block.
            }
        };

        info!("on_configure:Start importing blocklist.");
        // Create a new trie, import the configuration bytes, and replace the old one.
        let mut new_blocklist = IpnetTrie::new();
        new_blocklist.import_from_bytes(&config_bytes);

        self.blocklist = Arc::new(new_blocklist);
        info!(
            "on_configure:Successfully imported blocklist with {} IPv4 and {} IPv6 networks.",
            self.blocklist.len().0,
            self.blocklist.len().1
        );
        true
    }

    fn create_http_context(&self, _: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(MyHttpContext {
            blocklist: Arc::clone(&self.blocklist),
        }))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

struct MyHttpContext {
    blocklist: Arc<IpnetTrie<u8>>,
}

impl Context for MyHttpContext {}

impl HttpContext for MyHttpContext {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        if let Some(xff_raw) = self.get_http_request_header("x-forwarded-for") {
            // Split the header value by commas.
            let addresses: Vec<&str> = xff_raw.split(',').map(|s| s.trim()).collect();

            // Take the rightmost IP address.
            if let Some(last_address) = addresses.last() {
                if let Ok(ip) = last_address.parse::<std::net::IpAddr>() {
                    // Check if the IP is in the blocklist.
                    // We convert the IpAddr to a host IpNet (e.g., /32 for IPv4) for the lookup.
                    let ipnet = IpNet::from(ip);
                    if self.blocklist.longest_match(&ipnet).is_some() {
                        info!("Blocking request from {}", ip);
                        self.send_http_response(
                            403,
                            vec![("x-processed-by", "rust-ipacl")],
                            Some(b"Forbidden: IP address is blocked.\n"),
                        );
                        return Action::Pause;
                    }
                } else {
                    info!("Failed to parse IP address: {}", last_address);
                }
            }
        }

        Action::Continue
    }
}
