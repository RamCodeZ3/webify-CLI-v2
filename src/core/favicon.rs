use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use image::codecs::png::PngEncoder;
use image::{imageops::FilterType, DynamicImage, ExtendedColorType, ImageEncoder, RgbaImage};

use crate::core::favicon_type::{FaviconFormat, FaviconSpec, FAVICON_SPECS};
use crate::utils::error::{AppError, AppResult};

const ICO_SIZES: [(u32, u32); 3] = [(48, 48), (32, 32), (16, 16)];

#[derive(Debug, Default)]
pub struct FaviconGenerationReport {
    pub succeeded: Vec<PathBuf>,
    pub failed: Vec<(String, AppError)>,
}

impl FaviconGenerationReport {
    pub fn generated_count(&self) -> usize {
        self.succeeded.len()
    }
}

pub struct FaviconGenerator {
    app_name: String,
    output_dir: PathBuf,
    source_image: DynamicImage,
}

impl FaviconGenerator {
    pub fn new(
        source_path: impl AsRef<Path>,
        app_name: Option<String>,
        destination_path: Option<PathBuf>,
    ) -> AppResult<Self> {
        let source_path = source_path.as_ref();

        if !source_path.is_file() {
            return Err(AppError::PathNotFound(source_path.to_path_buf()));
        }

        let base_dest = destination_path.unwrap_or_else(|| {
            source_path
                .parent()
                .map(Path::to_path_buf)
                .unwrap_or_default()
        });
        let output_dir = base_dest.join("favicon");

        fs::create_dir_all(&output_dir).map_err(|source| AppError::Io {
            path: output_dir.clone(),
            source,
        })?;

        let source_image = image::open(source_path).map_err(|source| AppError::Image {
            path: source_path.to_path_buf(),
            source,
        })?;

        Ok(Self {
            app_name: app_name.unwrap_or_else(|| "MyWebSite".to_string()),
            output_dir,
            source_image,
        })
    }

    pub fn generate_all(&self) -> FaviconGenerationReport {
        let mut report = FaviconGenerationReport::default();

        for spec in FAVICON_SPECS {
            match self.generate_one(spec) {
                Ok(dest) => report.succeeded.push(dest),
                Err(err) => report.failed.push((spec.filename(), err)),
            }
        }

        match self.generate_webmanifest() {
            Ok(dest) => report.succeeded.push(dest),
            Err(err) => report.failed.push(("site.webmanifest".to_string(), err)),
        }

        report
    }

    pub fn html_snippet(&self) -> String {
        format!(
            r#"<link rel="icon" type="image/png" href="/favicon/favicon-96x96.png" sizes="96x96" />
<link rel="icon" type="image/svg+xml" href="/favicon/favicon.svg" />
<link rel="shortcut icon" href="/favicon/favicon.ico" />
<link rel="apple-touch-icon" sizes="180x180" href="/favicon/apple-touch-icon.png" />
<meta name="apple-mobile-web-app-title" content="{}" />
<link rel="manifest" href="/favicon/site.webmanifest" />"#,
            self.app_name
        )
    }

    fn generate_one(&self, spec: &FaviconSpec) -> AppResult<PathBuf> {
        let dest = self.output_dir.join(spec.filename());

        match spec.format {
            FaviconFormat::Svg => self.write_svg(&dest, spec.dimensions)?,
            FaviconFormat::Ico => self.write_ico(&dest)?,
            FaviconFormat::Png => self.write_png(&dest, spec.dimensions)?,
        }

        Ok(dest)
    }

    fn resized_rgba(&self, dimensions: (u32, u32)) -> RgbaImage {
        self.source_image
            .resize_exact(dimensions.0, dimensions.1, FilterType::Lanczos3)
            .to_rgba8()
    }

    fn write_png(&self, dest: &Path, dimensions: (u32, u32)) -> AppResult<()> {
        self.resized_rgba(dimensions)
            .save(dest)
            .map_err(|source| AppError::Image {
                path: dest.to_path_buf(),
                source,
            })
    }

    fn write_ico(&self, dest: &Path) -> AppResult<()> {
        let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

        for (w, h) in ICO_SIZES {
            let resized = self.resized_rgba((w, h));
            let icon_image = ico::IconImage::from_rgba_data(w, h, resized.into_raw());
            let entry = ico::IconDirEntry::encode(&icon_image).map_err(|source| AppError::Io {
                path: dest.to_path_buf(),
                source,
            })?;
            icon_dir.add_entry(entry);
        }

        let file = fs::File::create(dest).map_err(|source| AppError::Io {
            path: dest.to_path_buf(),
            source,
        })?;

        icon_dir.write(file).map_err(|source| AppError::Io {
            path: dest.to_path_buf(),
            source,
        })
    }

    fn write_svg(&self, dest: &Path, dimensions: (u32, u32)) -> AppResult<()> {
        let resized = self.resized_rgba(dimensions);
        let (w, h) = dimensions;

        let mut png_bytes = Vec::new();
        PngEncoder::new(&mut Cursor::new(&mut png_bytes))
            .write_image(resized.as_raw(), w, h, ExtendedColorType::Rgba8)
            .map_err(|source| AppError::Image {
                path: dest.to_path_buf(),
                source,
            })?;
        let encoded = STANDARD.encode(&png_bytes);

        let svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}"><image width="{w}" height="{h}" href="data:image/png;base64,{encoded}"/></svg>"#
        );

        fs::write(dest, svg).map_err(|source| AppError::Io {
            path: dest.to_path_buf(),
            source,
        })
    }

    fn generate_webmanifest(&self) -> AppResult<PathBuf> {
        let name = escape_json(&self.app_name);

        let manifest = format!(
            r#"{{
    "name": "{name}",
    "short_name": "{name}",
    "icons": [
        {{
            "src": "/favicon/web-app-manifest-192x192.png",
            "sizes": "192x192",
            "type": "image/png",
            "purpose": "maskable"
        }},
        {{
            "src": "/favicon/web-app-manifest-512x512.png",
            "sizes": "512x512",
            "type": "image/png",
            "purpose": "maskable"
        }}
    ],
    "theme_color": "#ffffff",
    "background_color": "#ffffff",
    "display": "standalone"
}}
"#
        );

        let dest = self.output_dir.join("site.webmanifest");
        fs::write(&dest, manifest).map_err(|source| AppError::Io {
            path: dest.clone(),
            source,
        })?;
        Ok(dest)
    }
}

fn escape_json(input: &str) -> String {
    input.replace('\\', "\\\\").replace('"', "\\\"")
}
