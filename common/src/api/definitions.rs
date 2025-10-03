use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Definition {
    #[serde(rename = "type")]
    pub def_type: String,
    pub af: u8,
    pub resolve_on_probe: bool,
    pub packets: u32,
    pub size: u32,
    pub skip_dns_check: bool,
    pub include_probe_id: bool,
    pub target: String,
    pub tags: Vec<String>,
    pub interval: Option<u32>,
}

pub struct DefinitionTemplate {
    pub def_type: String,
    pub af: u8,
    pub resolve_on_probe: bool,
    pub packets: u32,
    pub size: u32,
    pub skip_dns_check: bool,
    pub include_probe_id: bool,
    pub tags: Vec<String>,
    pub interval: Option<u32>, 
}

impl DefinitionTemplate {
    pub fn to_definition(&self, target: String) -> Definition {
        Definition {
            def_type: self.def_type.clone(),
            af: self.af,
            resolve_on_probe: self.resolve_on_probe,
            packets: self.packets,
            size: self.size,
            skip_dns_check: self.skip_dns_check,
            include_probe_id: self.include_probe_id,
            target: target,
            tags: self.tags.clone(),
            interval: self.interval,
        }
    }
}

pub struct DefinitionTemplateBuilder {
    def_type: Option<String>,
    af: Option<u8>,
    resolve_on_probe: Option<bool>,
    packets: Option<u32>,
    size: Option<u32>,
    skip_dns_check: Option<bool>,
    include_probe_id: Option<bool>,
    tags: Option<Vec<String>>,
    interval: Option<u32>,
}

impl DefinitionTemplateBuilder {
    pub fn new() -> Self {
        Self {
            def_type: None,
            af: Some(4),
            resolve_on_probe: Some(false),
            packets: Some(3),
            size: Some(48),
            skip_dns_check: Some(false),
            include_probe_id: Some(false),
            tags: Some(vec![]),
            interval: None,
        }
    }

    pub fn def_type(mut self, def_type: impl Into<String>) -> Self {
        self.def_type = Some(def_type.into());
        self
    }

    pub fn af(mut self, af: u8) -> Self {
        self.af = Some(af);
        self
    }

    pub fn resolve_on_probe(mut self, v: bool) -> Self {
        self.resolve_on_probe = Some(v);
        self
    }

    pub fn packets(mut self, p: u32) -> Self {
        self.packets = Some(p);
        self
    }

    pub fn size(mut self, s: u32) -> Self {
        self.size = Some(s);
        self
    }

    pub fn skip_dns_check(mut self, v: bool) -> Self {
        self.skip_dns_check = Some(v);
        self
    }

    pub fn include_probe_id(mut self, v: bool) -> Self {
        self.include_probe_id = Some(v);
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    pub fn interval(mut self, interval: Option<u32>) -> Self {
        self.interval = interval;
        self
    }

    pub fn build(self) -> Result<DefinitionTemplate, &'static str> {
        Ok(DefinitionTemplate {
            def_type: self.def_type.ok_or("Missing field: definition type")?,
            af: self.af.unwrap(),
            resolve_on_probe: self.resolve_on_probe.unwrap(),
            packets: self.packets.unwrap(),
            size: self.size.unwrap(),
            skip_dns_check: self.skip_dns_check.unwrap(),
            include_probe_id: self.include_probe_id.unwrap(),
            tags: self.tags.unwrap(),
            interval: self.interval, 
        })
    }
}