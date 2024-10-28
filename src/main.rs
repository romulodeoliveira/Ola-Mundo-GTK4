use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, Box, Entry, Button, Orientation};

fn main() -> glib::ExitCode {
    // Cria uma nova aplicação
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // executa a aplicação
    app.run()
}

fn build_ui(app: &Application) {
    let entrada = Entry::builder()
        .placeholder_text("Digite seu nome")
        .build();

    // cria um botão
    let button = Button::builder()
        .label("Enviar!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    /* Conecta ao sinal "clicked" do botão
    button.connect_clicked(|button| {
        // Define o texto do botão após ele ser clicado
        button.set_label("Hello World!");
    });
    */

    // Clona `entrada` e `button` para o closure
    let entrada_clone = entrada.clone();
    let button_clone = button.clone();
    button.connect_clicked(move |_| {
        // Obtém o texto da `Entry` e define a mensagem no botão
        let text = entrada_clone.text().to_string();
        let greeting = format!("Olá, {}!", text);
        button_clone.set_label(&greeting);
    });

    // Cria uma `Box` para organizar os widgets
    let vbox = Box::new(Orientation::Vertical, 5);
    vbox.append(&entrada);
    vbox.append(&button);

    // cria uma janela
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Olá Mundo")
        .default_width(320)
        .default_height(200)
        .child(&vbox)
        .build();

    window.present();
}