# General:

- Look at [https://actions-rs.github.io/]
- enabled missingdoc in Cranky.toml
- Use https://docs.rs/once_cell/latest/once_cell/
- See https://reddit.com/r/rust/comments/ylp4nz/what_crates_are_considered_as_defacto_standard/

# Gtk on Mac

- https://gitlab.gnome.org/GNOME/gtk-mac-integration

## bugs:
- todo_4: after popup is dissmissed, window lose focus
- window is moved to random location after double click after double click (yes, twice)

## Integration:
- Sidebar blur
- pick and react to dark/light OS theme (**this appears to work**)
- Native window controls don't work on :hover (both for CSD and non-CSD)
  - suspecting event loop blocking osx event loop
  - doesn't seem to be a misconfiguration
- Use native window control on OSX
  - draw them where GTK wants to draw them
    - See gtk/gtkwindowcontrols.c `update_window_buttons`
    - can't really draw theme exactly where GTK wants in gtk code
    - in gdk, follow settings to know what buttons to draw (decoration-layout thing), and where to draw them
    - in gtk, draw the buttons as usual, but maybe only allocate space?
- Draw buttons on the left by default (but should work if buttons are move to right)

## WIP:
  - SUBMITED: window is virtually too big
    - https://gitlab.gnome.org/GNOME/gtk/-/merge_requests/5194
