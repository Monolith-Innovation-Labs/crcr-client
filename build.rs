fn main() {
    slint_build::compile("ui/textbox.slint").expect("Textbox build failed");
    slint_build::compile("ui/app-window.slint").expect("Slint build failed");
}
