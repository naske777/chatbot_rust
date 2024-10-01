mod ia_models;

use std::fs;
use std::io;
use ia_models::PatternMatchingChatbot;

fn main() {
    // Listar las carpetas disponibles en el directorio "src"
    let paths = fs::read_dir("src").expect("No se pudo leer el directorio");
    let mut project_folders = Vec::new();

    // Filtrar y agregar nombres de carpetas que son completamente numéricas
    for path in paths {
        let path = path.expect("No se pudo leer la ruta");
        if path.path().is_dir() {
            if let Some(folder_name) = path.file_name().to_str() {
                if folder_name.chars().all(char::is_numeric) {
                    project_folders.push(folder_name.to_string());
                }
            }
        }
    }

    // Mostrar las carpetas disponibles al usuario
    println!("Proyectos disponibles:");
    for folder in &project_folders {
        println!("{}", folder);
    }

    // Solicitar al usuario que introduzca el nombre de la carpeta del proyecto
    println!("Introduce el nombre del proyecto que deseas probar:");
    let mut project = String::new();
    io::stdin().read_line(&mut project).expect("Error al leer la línea");
    let project = project.trim();

    // Buscar la carpeta seleccionada en la lista de carpetas disponibles
    let selected_folder = project_folders.iter().find(|&&ref folder| folder == project)
        .expect("No se encontró el proyecto especificado");

    // Construir la ruta del archivo de datos de entrenamiento
    let training_data_path = format!("src/{}/chatBot_{}.json", selected_folder, selected_folder);

    // Crear una nueva instancia del chatbot y cargar los datos de entrenamiento
    let mut chatbot = PatternMatchingChatbot::new();
    chatbot.load_training_data(&training_data_path);

    // Bucle principal para interactuar con el usuario
    loop {
        println!("Introduce tu pregunta (o escribe 'salir' para terminar): ");
        let mut question = String::new();
        io::stdin().read_line(&mut question).expect("Error al leer la línea");
        let question = question.trim();

        if question.eq_ignore_ascii_case("salir") {
            break;
        }

        // Encontrar y mostrar la mejor respuesta para la pregunta del usuario
        match chatbot.find_best_response(question) {
            Some(response) => println!("Respuesta: {}", response),
            None => println!("No sé la respuesta a eso."),
        }
    }
}