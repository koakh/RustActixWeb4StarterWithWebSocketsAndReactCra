use static_files::NpmBuild;

fn main() -> std::io::Result<()> {
    NpmBuild::new("app")
        .install()?
        .run("build")?
        .target("./app/build")
        .change_detection()
        .to_resource_dir()
        .build()
}