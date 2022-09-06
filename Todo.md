Publish:
- cleanup ui create
- remote Todo
- proper theme
- rename mbox / mailbox* to folders

## Rust
- inline functions
- unwrap / expect

## crate::css
- maybe Pin instead of ouroboros?
- does `!important` work?
- warn if a solve just endup to `*`

## crate::ui
- Show 2 spinners in sidebar as the folders are getting loaded
- support `px` in css
- add a spinner to the folders
- fonts started looking ugly
- remove himalaya* and egui* crates from non main app crate
- rename server to mailservice or something like that…
- show cells "inline" (like de "draft" label on fastmail webapp)
- remove egui and log from `styling` crate
- create a Flex crate
- implement more from himalaya library
- study https://github.com/terhechte/postsack/blob/main/ps-gui/src/widgets/table.rs
- file ton of issues
- toolbar:
  - [DONE] collapse sidebar
  - debug: Look at the Backend tab in egui demo. Lot of options and tools that we could enable
  - collapsable warning console
  - search textbox
  - "reload" button to pull new emails?
- documentation / Comments
- go through all the FIXME
- test?
- in mailbody, the subject doesn't wrap
- test on Windows and Linux
- check out the logs… lot of activities even if idle
- cache as much as possible for a rendering passthrough
- non-solarized theme
- make it gui agnostic:
  - use egui-like API/syntax
  - implement hbox/vbox/flex
  - generic styling system
  - events system
- Try https://docs.rs/lightningcss?
- need to click before key events work
- implement text overflow (clip with `…`)
- NSVisualEffectView doesn't work with Wgpu
- Mac: transparent sidebar
- if there's an issue with formatting in emails, everything falls appart
- support chinese characters and icons: https://github.com/emilk/egui/issues/162
- proper egui ids (based on element)
- hold key up: top white selection is flickering: it should always be white, as in:
  - the top row should never be not selected
- maybe the row selection background should be "sliding" instead of jumping
- Don't automatically show the mailbody. Only do it on Enter? (Like mutt)
- showing the bottom panel can cover the selected row. Maybe don't collapse the Mailbody when no selection?
- perf: maybe store text in egui::galley
- proper date formating
- re-introduce persistence
- integrate libweb
