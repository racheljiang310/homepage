/* render_static.rs
 * This script serves to generate static files that can be used for deployment to vercel
 * Generated code can be found in public/ directory
*/
use std::fs;
use tera::{Tera, Context};

fn main() {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to load templates: {}", e);
            return;
        }
    };

    let pages = vec!["index", "about", "projects", "gallery", "contact"];

    for page in pages {
        let mut ctx = Context::new();
        ctx.insert("title", &format!("My Portfolio - {}", page));

        let rendered_html = match tera.render(&format!("{}.html", page), &ctx) {
            Ok(html) => html,
            Err(e) => {
                eprintln!("Failed to render {}: {}", page, e);
                continue;
            }
        };

        // Update the navigation links & static file service
        let updated_html = rendered_html.replace(
            r#"href="/about"#,
            r#"href="/about.html"#,
        ).replace(
            r#"href="/projects"#,
            r#"href="/projects.html"#,
        ).replace(
            r#"href="/gallery"#,
            r#"href="/gallery.html"#,
        ).replace(
            r#"href="/contact"#,
            r#"href="/contact.html"#,
        )
        .replace(r#"src="/static/images/"#, r#"src="/images/"#)
        .replace(r#"href="/static/styles"#, r#"href="/styles"#);

        fs::create_dir_all("public").unwrap();
        let file_path = format!("public/{}.html", page);
        fs::write(&file_path, updated_html).expect("Failed to write file");

        println!("Rendered: {}", file_path);
    }

    println!("✅ Static HTML files generated successfully!");
}