//! Help ▸ About: the framework's hosted About dialog over the app's info and the baked credits.

use craft::behavior::IconId;
use craft::ui::patterns::form_dialogs::{AboutImage, AboutInfo, AboutLink, AboutTab, HostedAbout};

use crate::PdfZApp;

/// The ArtCraft mark, unmodified (AGENTS.md §1.2); the framework draws it through its SVG picture.
const MARK: &[u8] = include_bytes!("../../../../assets/z-brand/mark.svg");
/// The ArtCraft wordmark for a light theme.
const LOGO: &[u8] = include_bytes!("../../../../assets/z-brand/logo.svg");
/// The ArtCraft wordmark for a dark theme, which the light one would not read on.
const LOGO_WHITE: &[u8] = include_bytes!("../../../../assets/z-brand/logo-white.svg");

/// What the About tab shows: PdfZ's name, blurb and version, the ArtCraft brand, and the community links.
fn info() -> AboutInfo {
    let info = AboutInfo::new(
        "PdfZ",
        tl!("A clean-room, open-source PDF application written in Rust. MIT OR Apache-2.0."),
        pdfz_engine::build_info::version(),
    )
    .mark(AboutImage::svg("Upstream", MARK))
    .part_of(AboutImage::svg("Upstream", LOGO).dark(LOGO_WHITE))
    .built_with("Rendering: hayro (bootstrap) · UI: egui · Icons: Lucide (ISC) · Fonts: Inter, JetBrains Mono, Dancing Script (OFL)");
    info.links(pdfz_engine::links::LINKS.iter().enumerate().map(|(i, l)| {
        let label = if i == 0 { tl!("Get help on GitHub") } else { tl!(l.label) };
        AboutLink::new(label.to_string(), l.url).icon(IconId::new(l.icon))
    }))
}

impl PdfZApp {
    /// Opens the About window on `tab` (Help ▸ About, `help.about`).
    pub(crate) fn open_about(&mut self, tab: AboutTab) {
        crate::dialogs::host::open(self, HostedAbout::new(info(), crate::credits_data::credits()).tab(tab));
    }

    /// Opens the link the user activated in the About window, the way the `help.*` link commands do.
    pub(crate) fn about_pass(&mut self, _ctx: &egui::Context) {
        let Some(url) = self.dialogs.get_mut::<HostedAbout>().and_then(HostedAbout::take_link) else {
            return;
        };
        self.open_url(&url);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn about_community_links_keep_their_icons_and_destinations() {
        let info = super::info();
        assert_eq!(info.links.len(), pdfz_engine::links::LINKS.len());
        for (shown, expected) in info.links.iter().zip(pdfz_engine::links::LINKS) {
            assert_eq!(shown.url, expected.url);
            assert_eq!(shown.icon, Some(craft::behavior::IconId::new(expected.icon)));
        }
    }
}
