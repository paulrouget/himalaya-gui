use css::Rules;
use egui::{Frame, Ui};
use styling::element as elt;

pub fn update(ui: &mut Ui, rules: &Rules, parent: &elt::Element) -> bool {
  let frame: Frame = elt::hbox().id("toolbar").parent(parent.clone()).compute(rules).into();
  let response = frame.show(ui, |ui| {
    ui.horizontal(|ui| {
      let clicked = ui.button("Sidebar").clicked();
      // FIXME: I wish we didn't have to do that. Content should be justified.
      ui.add_space(ui.available_width());
      clicked
    })
  });

  response.inner.inner
}
