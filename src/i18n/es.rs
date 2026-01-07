//! Español - Spanish translations

use super::Translations;
use std::collections::HashMap;

pub fn translations() -> Translations {
    let mut data = HashMap::new();

    // Common
    data.insert("app.name", "Lazarus");
    data.insert(
        "app.tagline",
        "Gestión de Conocimiento Personal Sin Conexión",
    );
    data.insert("nav.notes", "Notas");
    data.insert("nav.search", "Buscar");
    data.insert("nav.wiki", "Wiki");

    // Home
    data.insert("home.notes", "Notas");
    data.insert("home.streak", "Días de Estudio");
    data.insert("home.day", "día(s)");
    data.insert("home.quick_start", "Inicio Rápido");
    data.insert("home.new_note", "Nueva Nota");
    data.insert("home.note_list", "Lista de Notas");
    data.insert("home.split_view", "Vista Dividida");
    data.insert("home.srs_review", "Revisión SRS");
    data.insert("home.search", "Buscar");
    data.insert("home.zim_manage", "Gestionar ZIM");
    data.insert("home.security", "Seguridad");
    data.insert("home.shortcuts", "Atajos de Teclado");
    data.insert("home.shortcut.new_note", "Nueva nota");
    data.insert("home.shortcut.search", "Buscar");
    data.insert("home.shortcut.help", "Ayuda");

    // Editor
    data.insert("editor.title_placeholder", "Ingrese el título");
    data.insert("editor.content_placeholder", "Empiece a escribir aquí...");
    data.insert("editor.tags", "Etiquetas");
    data.insert("editor.tags_placeholder", "etiqueta1, etiqueta2, ...");
    data.insert("editor.edit_note", "Editar Nota");
    data.insert("editor.encrypt", "Alternar cifrado");
    data.insert("editor.focus", "Enfoque");
    data.insert("editor.fullscreen", "Pantalla Completa");
    data.insert("editor.save", "Guardar");
    data.insert("editor.saved", "Guardado");
    data.insert("editor.auto_saved", "Guardado automático");
    data.insert("editor.changed", "Cambiado...");
    data.insert("editor.words", "palabras");
    data.insert("editor.save_complete", "💾 ¡Guardado!");
    data.insert("editor.encrypt_on", "🔒 Cifrado activado");
    data.insert("editor.encrypt_off", "🔓 Cifrado desactivado");
    data.insert(
        "editor.pin_required",
        "Configure el PIN primero (Menú de Seguridad)",
    );
    data.insert("editor.pin_locked", "Desbloquee el PIN primero");

    // Notes list
    data.insert("notes.title", "Notas");
    data.insert("notes.export", "Exportar");
    data.insert("notes.import", "Importar");
    data.insert("notes.no_notes", "Aún no hay notas");
    data.insert("notes.create_first", "¡Crea tu primera nota!");
    data.insert("notes.no_title", "Sin Título");
    data.insert("notes.edit", "Editar");
    data.insert("notes.delete", "Eliminar");
    data.insert("notes.delete_confirm", "¿Está seguro de eliminar?");
    data.insert("notes.encrypted", "Cifrado");
    data.insert("notes.created", "Creado");
    data.insert("notes.updated", "Actualizado");
    data.insert("notes.find_duplicates", "Buscar Duplicados");
    data.insert("notes.select_all", "Seleccionar Todo");
    data.insert("notes.selected", "seleccionados");
    data.insert("notes.create_package", "Crear Paquete");
    data.insert("notes.duplicates_title", "Notas Duplicadas");
    data.insert("notes.no_duplicates", "No se encontraron notas duplicadas");
    data.insert("notes.export_package", "Exportar Paquete");
    data.insert("notes.import_package", "Importar Paquete");
    data.insert("notes.package_title", "Título del Paquete");
    data.insert("notes.package_title_placeholder", "ej., Física 101");
    data.insert("notes.package_author", "Autor");
    data.insert("notes.package_author_placeholder", "Su nombre");
    data.insert("notes.package_description", "Descripción");
    data.insert(
        "notes.package_description_placeholder",
        "Descripción del paquete (opcional)",
    );
    data.insert("notes.drop_file", "Suelte el archivo .laz aquí");
    data.insert("notes.or", "o");
    data.insert("notes.select_file", "Seleccionar Archivo");

    // Search
    data.insert("search.title", "Buscar");
    data.insert("search.placeholder", "Ingrese término de búsqueda...");
    data.insert("search.button", "Buscar");
    data.insert("search.results", "Resultados de Búsqueda");
    data.insert("search.no_results", "No se encontraron resultados");
    data.insert("search.try_different", "Intente un término diferente");
    data.insert("search.tips", "Consejos de Búsqueda");
    data.insert(
        "search.tip1",
        "Múltiples palabras buscan resultados con todos los términos",
    );
    data.insert("search.tip2", "Busca en notas y Wikipedia");
    data.insert(
        "search.tip3",
        "Las coincidencias de título aparecen primero",
    );

    // Wiki
    data.insert("wiki.search", "Buscar Wiki");
    data.insert("wiki.recent_articles", "Recent Articles");
    data.insert("wiki.manage", "Gestionar ZIM");
    data.insert("wiki.status", "Estado");
    data.insert("wiki.loaded", "cargados");
    data.insert("wiki.directory", "Directorio ZIM");
    data.insert(
        "wiki.directory_hint",
        "Copie archivos .zim a esta carpeta y actualice",
    );
    data.insert("wiki.refresh", "Actualizar");
    data.insert("wiki.open_folder", "Abrir Carpeta");
    data.insert("wiki.add", "Agregar ZIM");
    data.insert("wiki.add_placeholder", "Ingrese ruta del archivo ZIM...");
    data.insert("wiki.loaded_files", "Archivos ZIM Cargados");
    data.insert("wiki.remove", "Eliminar");
    data.insert("wiki.no_zim", "Sin archivos ZIM");
    data.insert(
        "wiki.no_zim_hint",
        "Agregue un archivo ZIM para usar Wikipedia",
    );
    data.insert("wiki.no_zim_loaded", "No hay archivos ZIM cargados");
    data.insert("wiki.loaded_zims", "ZIMs Cargados");
    data.insert("wiki.add_btn", "Agregar");
    data.insert("wiki.add_hint", "Ingrese la ruta completa del archivo ZIM.");
    data.insert("wiki.name", "Nombre");
    data.insert("wiki.path", "Ruta");
    data.insert("wiki.action", "Acción");
    data.insert("wiki.zim_added", " ZIM agregados: ");
    data.insert("wiki.no_new_zim", "Sin nuevos archivos ZIM");
    data.insert("wiki.refresh_failed", "Error al actualizar");
    data.insert("wiki.enter_path", "Por favor ingrese una ruta");
    data.insert("wiki.add_failed", "Error al agregar");
    data.insert("wiki.remove_confirm", "¿Eliminar este ZIM?");
    data.insert("wiki.zim_removed", "ZIM eliminado: ");
    data.insert("wiki.remove_failed", "Error al eliminar");
    data.insert(
        "wiki.open_folder_msg",
        "Abra la carpeta en el explorador de archivos:",
    );

    // SRS
    data.insert("srs.title", "Revisión SRS");
    data.insert("srs.cards", "Lista de Tarjetas");
    data.insert("srs.streak", "racha");
    data.insert("srs.show_answer", "Mostrar Respuesta");
    data.insert("srs.again", "De Nuevo");
    data.insert("srs.hard", "Difícil");
    data.insert("srs.good", "Bien");
    data.insert("srs.easy", "Fácil");
    data.insert("srs.complete", "🎉 ¡Revisión de hoy completa!");
    data.insert("srs.no_cards", "Sin tarjetas para revisar");
    data.insert("srs.progress", "tarjetas");
    data.insert("srs.today_review", "Revisión de Hoy");
    data.insert("srs.new_cards", "Nuevas");
    data.insert("srs.learning", "Aprendiendo");
    data.insert("srs.mature", "Maduras");
    data.insert("srs.create_from_notes", "Crear tarjetas desde notas");
    data.insert("srs.create_cards", "Crear Tarjetas");
    data.insert("srs.cards_created", "tarjetas creadas");
    data.insert("srs.no_cards_extracted", "Sin tarjetas para extraer");
    data.insert("srs.repetitions", "Repeticiones");
    data.insert("srs.streak_days", "Racha");
    data.insert("srs.start_review", "Iniciar Revisión");
    data.insert("srs.no_cards_yet", "Aún no hay tarjetas.");
    data.insert(
        "srs.create_hint",
        "Abra una nota y haga clic en \"Crear Tarjetas\".",
    );
    data.insert("srs.interval", "Intervalo");
    data.insert("srs.days", " días");
    data.insert("srs.delete_card_confirm", "¿Eliminar esta tarjeta?");
    data.insert("srs.delete_failed", "Error al eliminar");

    // Security
    data.insert("security.title", "Configuración de Seguridad");
    data.insert("security.pin_not_set", "PIN no configurado");
    data.insert("security.pin_enabled", "PIN activado");
    data.insert("security.locked", "Bloqueado - Ingrese PIN");
    data.insert("security.pin_input", "Ingrese PIN (6-32 caracteres)");
    data.insert("security.set_pin", "Configurar PIN");
    data.insert("security.remove_pin", "Eliminar PIN");
    data.insert("security.lock", "Bloquear");
    data.insert("security.unlock", "Desbloquear");
    data.insert("security.wrong_pin", "PIN incorrecto");
    data.insert("security.pin_set_success", "PIN configurado");
    data.insert("security.pin_removed", "PIN eliminado");
    data.insert("security.unlocked", "Desbloqueado");
    data.insert(
        "security.pin_min_length",
        "El PIN debe tener al menos 6 caracteres",
    );
    data.insert("security.enter_current_pin", "Ingrese PIN actual");

    // Backup
    data.insert("backup.title", "Respaldo");
    data.insert("backup.info", "Información de Respaldo");
    data.insert("backup.now", "Respaldar Ahora");
    data.insert("backup.complete", "Respaldo completo");
    data.insert("backup.no_changes", "Sin cambios (respaldo omitido)");

    // Split view
    data.insert("split.select_note", "Seleccione una nota de la izquierda");

    // Common buttons
    data.insert("common.confirm", "Confirmar");
    data.insert("common.cancel", "Cancelar");
    data.insert("common.close", "Cerrar");
    data.insert("common.loading", "Cargando...");
    data.insert("common.error", "Ocurrió un error");
    data.insert("common.success", "Éxito");

    // Settings
    data.insert("settings.title", "Configuración");
    data.insert("settings.language", "Idioma");
    data.insert("settings.language_desc", "Selecciona tu idioma preferido");
    data.insert("settings.stats", "Estadísticas");
    data.insert("settings.version", "Versión");
    data.insert("settings.storage", "Almacenamiento");
    data.insert("settings.about", "Acerca de");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "Grafo de Conocimiento");
    data.insert("app.footer", "Conocimiento para Todos");

    // === USB ===
    data.insert("usb.title", "Sincronización USB");
    data.insert("usb.scan", "Escanear");
    data.insert("usb.scanning", "Buscando dispositivos USB");
    data.insert("usb.no_usb", "No se detectó ningún USB de Lazarus");
    data.insert(
        "usb.no_usb_hint",
        "Inserta un USB con lazarus.sync o inicializa uno nuevo",
    );
    data.insert("usb.error", "Error al escanear");
    data.insert("usb.init_title", "Inicializar USB");
    data.insert(
        "usb.init_desc",
        "Crear un nuevo USB de Lazarus para compartir sin conexión",
    );
    data.insert("usb.init_btn", "Inicializar");
    data.insert("usb.init_error", "Error al inicializar");
    data.insert("usb.enter_path", "Ingresa la ruta del USB");
    data.insert("usb.notes", "Notas");
    data.insert("usb.posts", "Publicaciones");
    data.insert("usb.packages", "Paquetes");
    data.insert("usb.sync", "Sincronizar");
    data.insert("usb.export", "Exportar");
    data.insert("usb.import", "Importar");
    data.insert("home.usb_sync", "Sincronizar USB");
    data.insert("home.shortcut.wiki", "Wiki");
    data.insert("home.shortcut.usb", "USB");
    data.insert("home.shortcut.graph", "Gráfico");
    data.insert("home.shortcut.review", "Revisar");
    Translations::new(data)
}
