#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaviconFormat {
    Ico,
    Svg,
    Png,
}

impl FaviconFormat {
    pub const fn extension(self) -> &'static str {
        match self {
            FaviconFormat::Ico => "ico",
            FaviconFormat::Svg => "svg",
            FaviconFormat::Png => "png",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaviconRel {
    Icon,
    AppleTouchIcon,
    None,
}


#[derive(Debug, Clone, Copy)]
pub struct FaviconSpec {
    pub format: FaviconFormat,
    pub rel: FaviconRel,
    pub dimensions: (u32, u32),
    pub prefix: &'static str,
}

impl FaviconSpec {
    pub fn filename(&self) -> String {
        format!("{}.{}", self.prefix, self.format.extension())
    }
}

pub const FAVICON_SPECS: &[FaviconSpec] = &[
    FaviconSpec {
        format: FaviconFormat::Ico,
        rel: FaviconRel::None,
        dimensions: (48, 48),
        prefix: "favicon",
    },
    FaviconSpec {
        format: FaviconFormat::Svg,
        rel: FaviconRel::Icon,
        dimensions: (96, 96),
        prefix: "favicon",
    },
    FaviconSpec {
        format: FaviconFormat::Png,
        rel: FaviconRel::Icon,
        dimensions: (96, 96),
        prefix: "favicon-96x96",
    },
    FaviconSpec {
        format: FaviconFormat::Png,
        rel: FaviconRel::AppleTouchIcon,
        dimensions: (180, 180),
        prefix: "apple-touch-icon",
    },
    FaviconSpec {
        format: FaviconFormat::Png,
        rel: FaviconRel::Icon,
        dimensions: (192, 192),
        prefix: "web-app-manifest-192x192",
    },
    FaviconSpec {
        format: FaviconFormat::Png,
        rel: FaviconRel::Icon,
        dimensions: (512, 512),
        prefix: "web-app-manifest-512x512",
    },
];
