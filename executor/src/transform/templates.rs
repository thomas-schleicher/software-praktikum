use crate::domain::definition::{
    DefinitionTemplate, HttpDefinition, PingDefinition, TracerouteDefinition,
};
use common::configuration::configuration::Configuration;
use uuid::Uuid;

pub fn create_definition_templates(
    config: &Configuration,
) -> Result<Vec<DefinitionTemplate>, &'static str> {
    let mut templates: Vec<DefinitionTemplate> = Vec::new();
    let uuid = Uuid::new_v4().to_string();

    if let Some(ping_config) = &config.ping_configuration {
        let ping_template = PingDefinition::template()
            .description(&uuid.as_str())
            .packets(ping_config.packet_count)
            .size(ping_config.size)
            .interval(config.interval);
        templates.push(DefinitionTemplate::Ping(ping_template));
    }

    if let Some(http_config) = &config.http_configuration {
        let https_template = HttpDefinition::template()
            .description(&uuid.as_str())
            .method(http_config.method.clone())
            .path(http_config.path.clone())
            .port(http_config.port)
            .header_bytes(http_config.header_bytes)
            .version(http_config.version)
            .interval(config.interval);
        templates.push(DefinitionTemplate::Http(https_template));
    }

    if let Some(traceroute_config) = &config.traceroute_configuration {
        let traceroute_template = TracerouteDefinition::template()
            .description(&uuid.as_str())
            .first_hop(traceroute_config.first_hop)
            .protocol(traceroute_config.protocol.clone())
            .packets(traceroute_config.packets)
            .size(traceroute_config.size)
            .max_hops(traceroute_config.max_hops)
            .paris(traceroute_config.paris)
            .dont_fragment(traceroute_config.dont_fragment)
            .port(traceroute_config.port);
        templates.push(DefinitionTemplate::Traceroute(traceroute_template));
    }

    if templates.is_empty() {
        return Err("No definition templates provided in configuration.");
    }

    Ok(templates)
}
