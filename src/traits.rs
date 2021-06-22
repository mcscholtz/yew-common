use yew::Html;

pub trait ToClass {
    fn to_class(&self) -> &'static str;
}

pub trait ToStyle {
    fn to_style(&self) -> &'static str;
}

pub trait ToClasses {
    fn to_classes(&self) -> String;
}

pub trait ToDynClass {
    fn to_class(&self) -> String;
}

pub trait View {
    fn view(&self) -> Html;
}