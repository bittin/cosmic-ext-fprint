use vergen_gitcl::{Emitter, Gitcl};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Rebuild if i18n files change
    println!("cargo:rerun-if-changed=i18n");

    // Emit version information (if not cached by just vendor)
    let gitcl = Gitcl::all_git();
    Emitter::default().add_instructions(&gitcl)?.emit()?;

    println!("cargo:rerun-if-env-changed=VERGEN_GIT_COMMIT_DATE");
    println!("cargo:rerun-if-env-changed=VERGEN_GIT_SHA");

    Ok(())
}
