use crate::model::{UnmaskReport, DossierReport};
use colored::*;
use tokio::time::{sleep, Duration};
use viuer::Config as ViuerConfig;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, imageops::FilterType};
use std::io::Cursor;

fn render_ascii(img: &DynamicImage, width: u32, colorize: bool) -> String {
    // Character ramp from dark -> light

    const RAMP: &[u8] = b"@%#*+=-:. ";

    // Maintain aspect ratio; terminal characters are taller than wide
    // so we shrink height by a factor (~0.5) to look proportionate
    let (w, h) = img.dimensions();
    if w == 0 || h == 0 || width == 0 { return String::new(); }
    let scaled_height = ((h as f32 * (width as f32 / w as f32)) * 0.5)
        .max(1.0)
        .round() as u32;

    let small = img.resize_exact(width, scaled_height, FilterType::Triangle);

    let mut out = String::new();
    for y in 0..small.height() {
        for x in 0..small.width() {
            let pixel = small.get_pixel(x, y);
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;
            let lum = 0.299 * r + 0.587 * g + 0.114 * b;
            let idx = ((lum / 255.0) * (RAMP.len() as f32 - 1.0))
                .clamp(0.0, (RAMP.len() - 1) as f32) as usize;
            let ch = RAMP[idx] as char;

            if colorize {
                // Use 24-bit truecolor if terminal supports; colored crate will do the escape
                out.push_str(&format!("{}", ch.to_string().truecolor(r as u8, g as u8, b as u8)));
            } else {
                out.push(ch);
            }
        }
        out.push('\n');
    }
    out
}

// Note: This function must now be `async` because it uses `await`.
pub async fn print_unmask_report(report: UnmaskReport) {
    let top_left = "╔";
    let top_right = "╗";
    let bottom_left = "╚";
    let bottom_right = "╝";
    let horizontal = "═";
    let vertical = "║";
    let divider_left = "╟";
    let divider_right = "╢";
    let divider_line = "─";
    let diamond = "◆";

    // ASCII art banner
    let banner = r#"
    ╔══════════════════════════════════════════════════════════╗
    ║     _____ _____ _      ___  _   _  ___                   ║
    ║    /  ___|  _  | |    / _ \| \ | |/ _ \                  ║
    ║    \ `--.| | | | |   / /_\ \  \| / /_\ \                 ║
    ║     `--. \ | | | |   |  _  | . ` |  _  |                 ║
    ║    /\__/ / \_/ | |___| | | | |\  | | | |                 ║
    ║    \____/ \___/\_____\_| |_\_| \_\_| |_/                 ║
    ║                                                          ║
    ║                                                          ║
    ╚══════════════════════════════════════════════════════════╝
    "#;

    println!("{}", banner.bright_purple().bold());
    sleep(Duration::from_millis(500)).await;
    // --- HEADER WITH NEON EFFECT ---
    println!("\n{}", format!("{}{}{}", top_left, horizontal.repeat(63), top_right).bright_cyan());
    let nft_name = format!(" {} {} {}",
        vertical.bright_cyan(),
        report.off_chain.name.bright_yellow().bold().underline(),
        diamond.bright_magenta()
    );
    println!("{}{:<63}{}", nft_name, "", vertical.bright_cyan());

    // --- ON-CHAIN INTEL SECTION ---
    let divider = format!("{}{}{}",
        divider_left.bright_cyan(),
        divider_line.repeat(63).bright_cyan(),
        divider_right.bright_cyan()
    );
    println!("{}", divider);
    let section_header = format!(" {} {} {}",
        vertical.bright_cyan(),
        "⛓️  ON-CHAIN INTELLIGENCE".bright_white().bold(),
        diamond.bright_blue()
    );
    println!("{}{:<63}{}", section_header, "", vertical.bright_cyan());
    println!("{}", divider);

    // Authority with emphasis
    let authority_line = format!(" {} {}  {}", vertical.bright_cyan(), "👑 Update Authority:".bright_yellow().bold(), report.on_chain.update_authority.to_string().bright_white());
    println!("{}{:<63}{}", authority_line, "", vertical.bright_cyan());

    // Mutability status with visual indicators
    let (mutable_icon, mutable_text, mutable_color) = if report.on_chain.is_mutable {
        ("🔓", "Mutable ⚠️  CAUTION", Color::BrightYellow)
    } else {
        ("🔒", "Frozen ✓ VERIFIED", Color::BrightGreen)
    };
    let mutable_line = format!(" {} {} Metadata State:    {}",
        vertical.bright_cyan(),
        mutable_icon,
        mutable_text.color(mutable_color).bold()
    );
    println!("{}{:<63}{}", mutable_line, "", vertical.bright_cyan());


    println!("{}", divider);
    let metadata_header = format!(" {} {} {}",
        vertical.bright_cyan(),
        " OFF-CHAIN METADATA".bright_white().bold(),
        diamond.bright_purple()
    );
    println!("{}{:<63}{}", metadata_header, "", vertical.bright_cyan());
    println!("{}", divider);

    let image_display = if report.off_chain.image.len() > 45 {
        format!("{}...", &report.off_chain.image[..45])
    } else {
        report.off_chain.image.clone()
    };
    let image_line = format!(" {} 🖼️  Image URI:         {}",
        vertical.bright_cyan(),
        image_display.bright_white()
    );
    println!("{}{:<63}{}", image_line, "", vertical.bright_cyan());

    // Attributes
    println!("{}", divider);
    let attr_header = format!(" {} ✨ {} Attributes Found",
        vertical.bright_cyan(),
        report.off_chain.attributes.len().to_string().bright_magenta().bold()
    );
    println!("{}{:<63}{}", attr_header, "", vertical.bright_cyan());

    for attr in report.off_chain.attributes.iter() {
        let trait_name = format!("{}:", attr.trait_type).bright_cyan().bold();
        let trait_value = attr.value.bright_white();
        let attr_line = format!(" {}    ▸ {:<15} {}",
            vertical.bright_cyan(),
            trait_name,
            trait_value
        );
        println!("{}{:<63}{}", attr_line, "", vertical.bright_cyan());
    }

    // --- FOOTER ---
    println!("{}", format!("{}{}{}",
        bottom_left.bright_cyan(),
        horizontal.repeat(63).bright_cyan(),
        bottom_right.bright_cyan()
    ));

    if let Some(image_bytes) = &report.image_data {
        println!("\n{}", divider);
        println!(" {} {}",
            vertical.bright_cyan(),
            "🖼️  VISUAL CONFIRMATION".bright_white().bold()
        );
        println!("{}", divider);

        // Try to render image; prefer portable ASCII (Pixel Clone)
        match ImageReader::new(Cursor::new(image_bytes)).with_guessed_format() {
            Ok(reader) => {
                match reader.decode() {
                    Ok(img) => {
                        // First, print ASCII so it's always visible
                        let ascii = render_ascii(&img, 60, true);
                        println!("{}", ascii);

                        // Then, best-effort nice rendering if terminal supports it
                        let conf = ViuerConfig {
                            width: Some(60),
                            height: Some(30),
                            absolute_offset: false,
                            ..Default::default()
                        };
                        let _ = viuer::print(&img, &conf);
                    }
                    Err(_) => {
                        println!(" {} {}", vertical.bright_cyan(), "❌ Could not decode image".bright_red());
                    }
                }
            }
            Err(_) => {
                println!(" {} {}", vertical.bright_cyan(), "❌ Could not decode image".bright_red());
            }
        }

        println!("{}", format!("{}{}{}",
            bottom_left.bright_cyan(),
            horizontal.repeat(63).bright_cyan(),
            bottom_right.bright_cyan()
        ));
    }

    println!("\n  {} Powered by {} | Scan complete ✓\n",
        "⚡".bright_yellow(),
        "shinobi".bright_purple().bold()
    );
}

pub async fn print_dossier_report(report: DossierReport) {
    let top_left = "╔";
    let top_right = "╗";
    let bottom_left = "╚";
    let bottom_right = "╝";
    let horizontal = "═";
    let vertical = "║";
    let divider_left = "╟";
    let divider_right = "╢";
    let divider_line = "─";

    println!("\n🥷 {} for target wallet...\n",
        "Generating dossier".bright_purple().bold()
    );

    sleep(Duration::from_millis(300)).await;

    // Truncate wallet address for display
    let display_wallet = if report.wallet_address.len() > 10 {
        format!("{}...{}",
            &report.wallet_address[..4],
            &report.wallet_address[report.wallet_address.len()-4..]
        )
    } else {
        report.wallet_address.clone()
    };

    println!("{}", format!("{}{}{}", top_left, horizontal.repeat(67), top_right).bright_cyan());
    println!(" {} {}  [Dossier] {}                                                             {}",
        vertical.bright_cyan(),
        "🎯".bright_yellow(),
        display_wallet.bright_white().bold(),
        vertical.bright_cyan()
    );

    let divider = format!("{}{}{}",
        divider_left.bright_cyan(),
        divider_line.repeat(67).bright_cyan(),
        divider_right.bright_cyan()
    );
    println!("{}", divider);

    println!(" {} {} PORTFOLIO SUMMARY                                                         {}",
        vertical.bright_cyan(),
        "📊".bright_blue(),
        vertical.bright_cyan()
    );
    println!("{}", divider);

    println!(" {} 🖼️  Total NFTs:       {}                                                        {}",
        vertical.bright_cyan(),
        report.total_nfts.to_string().bright_magenta().bold(),
        vertical.bright_cyan()
    );

    println!(" {} 💎 Top Collections:                                                          {}",
        vertical.bright_cyan(),
        vertical.bright_cyan()
    );

    let top_collections = report.collections.iter().take(10);
    for (i, collection) in top_collections.enumerate() {
        let rank = format!("{}.", i + 1).bright_yellow();
        let count_str = format!("{} NFTs", collection.count).bright_white();
        println!(" {}    {} {:<40} {}                                          {}",
            vertical.bright_cyan(),
            rank,
            collection.name.bright_cyan().bold(),
            count_str,
            vertical.bright_cyan()
        );
    }

    println!(" {}                                                                                {}",
        vertical.bright_cyan(),
        vertical.bright_cyan()
    );

    // Generate assessment
    let assessment = if !report.collections.is_empty() {
        let top = &report.collections[0];
        if top.count as f32 / report.total_nfts as f32 > 0.3 {
            format!("High-conviction '{}' holder.", top.name)
        } else if report.collections.len() > 5 {
            "Diversified collector across multiple collections.".to_string()
        } else {
            "Selective NFT holder.".to_string()
        }
    } else {
        "No NFT collections detected.".to_string()
    };

    println!(" {} 📝 Assessment:        {}                                                  {}",
        vertical.bright_cyan(),
        assessment.bright_green().italic(),
        vertical.bright_cyan()
    );

    println!("{}", format!("{}{}{}",
        bottom_left.bright_cyan(),
        horizontal.repeat(67).bright_cyan(),
        bottom_right.bright_cyan()
    ));

    println!("\n  {} Powered by {} | Dossier complete ✓\n",
        "⚡".bright_yellow(),
        "shinobi".bright_purple().bold()
    );
}