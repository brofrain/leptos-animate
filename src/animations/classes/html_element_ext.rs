use web_sys::HtmlElement;

type Classes = Vec<String>;

pub trait HtmlElementExt {
    /// Returns the classes that were not previously present, and were added by
    /// the method.
    fn add_unique_classes(&self, classes: &str) -> Classes;
    fn remove_classes(&self, classes: &Classes);
}

impl HtmlElementExt for HtmlElement {
    fn add_unique_classes(&self, classes: &str) -> Classes {
        let mut added_classes = Vec::new();

        for class in classes.split_whitespace() {
            let class_list = self.class_list();

            if class_list.contains(class) {
                continue;
            }

            class_list.add_1(class).unwrap();
            added_classes.push(class.to_owned());
        }

        added_classes
    }

    fn remove_classes(&self, classes: &Classes) {
        for class in classes {
            self.class_list().remove_1(class).unwrap();
        }
    }
}
