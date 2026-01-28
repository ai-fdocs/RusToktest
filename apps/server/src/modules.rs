use rustok_blog::BlogModule;
use rustok_commerce::CommerceModule;
use rustok_core::RusToKModule;

#[derive(Clone, Debug)]
pub struct ModuleInfo {
    pub slug: String,
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct ModuleRegistry {
    modules: Vec<ModuleInfo>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        let modules = register_modules()
            .into_iter()
            .map(|module| ModuleInfo {
                slug: module.slug().to_string(),
                name: module.name().to_string(),
                version: module.version().to_string(),
                dependencies: module
                    .dependencies()
                    .iter()
                    .map(|dependency| dependency.to_string())
                    .collect(),
            })
            .collect();

        Self { modules }
    }

    pub fn all(&self) -> &[ModuleInfo] {
        &self.modules
    }

    pub fn contains(&self, slug: &str) -> bool {
        self.modules.iter().any(|module| module.slug == slug)
    }
}

pub fn register_modules() -> Vec<Box<dyn RusToKModule>> {
    vec![Box::new(CommerceModule), Box::new(BlogModule)]
}
