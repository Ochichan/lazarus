//! Português - Portuguese translations

use super::Translations;
use std::collections::HashMap;

pub fn translations() -> Translations {
    let mut data = HashMap::new();

    // Common
    data.insert("app.name", "Lazarus");
    data.insert("app.tagline", "Gestão de Conhecimento Pessoal Offline");
    data.insert("nav.notes", "Notas");
    data.insert("nav.search", "Pesquisar");
    data.insert("nav.wiki", "Wiki");

    // Home
    data.insert("home.notes", "Notas");
    data.insert("home.streak", "Dias de Estudo");
    data.insert("home.day", "dia(s)");
    data.insert("home.quick_start", "Início Rápido");
    data.insert("home.new_note", "Nova Nota");
    data.insert("home.note_list", "Lista de Notas");
    data.insert("home.split_view", "Vista Dividida");
    data.insert("home.srs_review", "Revisão SRS");
    data.insert("home.search", "Pesquisar");
    data.insert("home.zim_manage", "Gerenciar ZIM");
    data.insert("home.security", "Segurança");
    data.insert("home.shortcuts", "Atalhos de Teclado");
    data.insert("home.shortcut.new_note", "Nova nota");
    data.insert("home.shortcut.search", "Pesquisar");
    data.insert("home.shortcut.help", "Ajuda");

    // Editor
    data.insert("editor.title_placeholder", "Digite o título");
    data.insert("editor.content_placeholder", "Comece a escrever aqui...");
    data.insert("editor.tags", "Tags");
    data.insert("editor.tags_placeholder", "tag1, tag2, ...");
    data.insert("editor.edit_note", "Editar Nota");
    data.insert("editor.encrypt", "Alternar criptografia");
    data.insert("editor.focus", "Foco");
    data.insert("editor.fullscreen", "Tela Cheia");
    data.insert("editor.save", "Salvar");
    data.insert("editor.saved", "Salvo");
    data.insert("editor.auto_saved", "Salvo automaticamente");
    data.insert("editor.changed", "Alterado...");
    data.insert("editor.words", "palavras");
    data.insert("editor.save_complete", "💾 Salvo!");
    data.insert("editor.encrypt_on", "🔒 Criptografia ativada");
    data.insert("editor.encrypt_off", "🔓 Criptografia desativada");
    data.insert(
        "editor.pin_required",
        "Configure o PIN primeiro (Menu Segurança)",
    );
    data.insert("editor.pin_locked", "Desbloqueie o PIN primeiro");

    // Notes list
    data.insert("notes.title", "Notas");
    data.insert("notes.export", "Exportar");
    data.insert("notes.import", "Importar");
    data.insert("notes.no_notes", "Ainda não há notas");
    data.insert("notes.create_first", "Crie sua primeira nota!");
    data.insert("notes.no_title", "Sem Título");
    data.insert("notes.edit", "Editar");
    data.insert("notes.delete", "Excluir");
    data.insert("notes.delete_confirm", "Tem certeza que deseja excluir?");
    data.insert("notes.encrypted", "Criptografado");
    data.insert("notes.created", "Criado");
    data.insert("notes.updated", "Atualizado");
    data.insert("notes.find_duplicates", "Encontrar Duplicados");
    data.insert("notes.select_all", "Selecionar Todos");
    data.insert("notes.selected", "selecionados");
    data.insert("notes.create_package", "Criar Pacote");
    data.insert("notes.duplicates_title", "Notas Duplicadas");
    data.insert("notes.no_duplicates", "Nenhuma nota duplicada encontrada");
    data.insert("notes.export_package", "Exportar Pacote");
    data.insert("notes.import_package", "Importar Pacote");
    data.insert("notes.package_title", "Título do Pacote");
    data.insert("notes.package_title_placeholder", "ex., Física 101");
    data.insert("notes.package_author", "Autor");
    data.insert("notes.package_author_placeholder", "Seu nome");
    data.insert("notes.package_description", "Descrição");
    data.insert(
        "notes.package_description_placeholder",
        "Descrição do pacote (opcional)",
    );
    data.insert("notes.drop_file", "Solte o arquivo .laz aqui");
    data.insert("notes.or", "ou");
    data.insert("notes.select_file", "Selecionar Arquivo");

    // Search
    data.insert("search.title", "Pesquisar");
    data.insert("search.placeholder", "Digite o termo de pesquisa...");
    data.insert("search.button", "Pesquisar");
    data.insert("search.results", "Resultados da Pesquisa");
    data.insert("search.no_results", "Nenhum resultado encontrado");
    data.insert("search.try_different", "Tente um termo diferente");
    data.insert("search.tips", "Dicas de Pesquisa");
    data.insert(
        "search.tip1",
        "Múltiplas palavras buscam resultados com todos os termos",
    );
    data.insert("search.tip2", "Pesquisa em notas e Wikipedia");
    data.insert(
        "search.tip3",
        "Correspondências de título aparecem primeiro",
    );

    // Wiki
    data.insert("wiki.search", "Pesquisar Wiki");
    data.insert("wiki.manage", "Gerenciar ZIM");
    data.insert("wiki.status", "Status");
    data.insert("wiki.loaded", "carregados");
    data.insert("wiki.directory", "Diretório ZIM");
    data.insert(
        "wiki.directory_hint",
        "Copie arquivos .zim para esta pasta e atualize",
    );
    data.insert("wiki.refresh", "Atualizar");
    data.insert("wiki.open_folder", "Abrir Pasta");
    data.insert("wiki.add", "Adicionar ZIM");
    data.insert("wiki.add_placeholder", "Digite o caminho do arquivo ZIM...");
    data.insert("wiki.loaded_files", "Arquivos ZIM Carregados");
    data.insert("wiki.remove", "Remover");
    data.insert("wiki.no_zim", "Sem arquivos ZIM");
    data.insert(
        "wiki.no_zim_hint",
        "Adicione um arquivo ZIM para usar Wikipedia",
    );
    data.insert("wiki.no_zim_loaded", "Nenhum arquivo ZIM carregado");
    data.insert("wiki.loaded_zims", "ZIMs Carregados");
    data.insert("wiki.add_btn", "Adicionar");
    data.insert("wiki.add_hint", "Digite o caminho completo do arquivo ZIM.");
    data.insert("wiki.name", "Nome");
    data.insert("wiki.path", "Caminho");
    data.insert("wiki.action", "Ação");
    data.insert("wiki.zim_added", " ZIM adicionados: ");
    data.insert("wiki.no_new_zim", "Sem novos arquivos ZIM");
    data.insert("wiki.refresh_failed", "Falha ao atualizar");
    data.insert("wiki.enter_path", "Por favor digite um caminho");
    data.insert("wiki.add_failed", "Falha ao adicionar");
    data.insert("wiki.remove_confirm", "Remover este ZIM?");
    data.insert("wiki.zim_removed", "ZIM removido: ");
    data.insert("wiki.remove_failed", "Falha ao remover");
    data.insert(
        "wiki.open_folder_msg",
        "Abra a pasta no explorador de arquivos:",
    );

    // SRS
    data.insert("srs.title", "Revisão SRS");
    data.insert("srs.cards", "Lista de Cartões");
    data.insert("srs.streak", "sequência");
    data.insert("srs.show_answer", "Mostrar Resposta");
    data.insert("srs.again", "De Novo");
    data.insert("srs.hard", "Difícil");
    data.insert("srs.good", "Bom");
    data.insert("srs.easy", "Fácil");
    data.insert("srs.complete", "🎉 Revisão de hoje completa!");
    data.insert("srs.no_cards", "Sem cartões para revisar");
    data.insert("srs.progress", "cartões");
    data.insert("srs.today_review", "Revisão de Hoje");
    data.insert("srs.new_cards", "Novos");
    data.insert("srs.learning", "Aprendendo");
    data.insert("srs.mature", "Maduros");
    data.insert("srs.create_from_notes", "Criar cartões a partir de notas");
    data.insert("srs.create_cards", "Criar Cartões");
    data.insert("srs.cards_created", "cartões criados");
    data.insert("srs.no_cards_extracted", "Sem cartões para extrair");
    data.insert("srs.repetitions", "Repetições");
    data.insert("srs.streak_days", "Sequência");
    data.insert("srs.start_review", "Iniciar Revisão");
    data.insert("srs.no_cards_yet", "Ainda não há cartões.");
    data.insert(
        "srs.create_hint",
        "Abra uma nota e clique em \"Criar Cartões\".",
    );
    data.insert("srs.interval", "Intervalo");
    data.insert("srs.days", " dias");
    data.insert("srs.delete_card_confirm", "Excluir este cartão?");
    data.insert("srs.delete_failed", "Falha ao excluir");

    // Security
    data.insert("security.title", "Configurações de Segurança");
    data.insert("security.pin_not_set", "PIN não configurado");
    data.insert("security.pin_enabled", "PIN ativado");
    data.insert("security.locked", "Bloqueado - Digite PIN");
    data.insert("security.pin_input", "Digite PIN (6-32 caracteres)");
    data.insert("security.set_pin", "Configurar PIN");
    data.insert("security.remove_pin", "Remover PIN");
    data.insert("security.lock", "Bloquear");
    data.insert("security.unlock", "Desbloquear");
    data.insert("security.wrong_pin", "PIN incorreto");
    data.insert("security.pin_set_success", "PIN configurado");
    data.insert("security.pin_removed", "PIN removido");
    data.insert("security.unlocked", "Desbloqueado");
    data.insert(
        "security.pin_min_length",
        "PIN deve ter pelo menos 6 caracteres",
    );
    data.insert("security.enter_current_pin", "Digite o PIN atual");

    // Backup
    data.insert("backup.title", "Backup");
    data.insert("backup.info", "Informações de Backup");
    data.insert("backup.now", "Fazer Backup Agora");
    data.insert("backup.complete", "Backup completo");
    data.insert("backup.no_changes", "Sem alterações (backup ignorado)");

    // Split view
    data.insert("split.select_note", "Selecione uma nota da esquerda");

    // Common buttons
    data.insert("common.confirm", "Confirmar");
    data.insert("common.cancel", "Cancelar");
    data.insert("common.close", "Fechar");
    data.insert("common.loading", "Carregando...");
    data.insert("common.error", "Ocorreu um erro");
    data.insert("common.success", "Sucesso");

    // Settings
    data.insert("settings.title", "Configurações");
    data.insert("settings.language", "Idioma");
    data.insert("settings.language_desc", "Selecione seu idioma preferido");
    data.insert("settings.stats", "Estatísticas");
    data.insert("settings.version", "Versão");
    data.insert("settings.storage", "Armazenamento");
    data.insert("settings.about", "Sobre");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "Grafo de Conhecimento");
    data.insert("app.footer", "Conhecimento para Todos");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "Grafo de Conhecimento");
    data.insert("app.footer", "Conhecimento para Todos");

    Translations::new(data)
}
