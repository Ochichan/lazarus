//! Français - French translations

use super::Translations;
use std::collections::HashMap;

pub fn translations() -> Translations {
    let mut data = HashMap::new();

    // Common
    data.insert("app.name", "Lazarus");
    data.insert(
        "app.tagline",
        "Gestion des Connaissances Personnelles Hors Ligne",
    );
    data.insert("nav.notes", "Notes");
    data.insert("nav.search", "Rechercher");
    data.insert("nav.wiki", "Wiki");

    // Home
    data.insert("home.notes", "Notes");
    data.insert("home.streak", "Jours d'Étude");
    data.insert("home.day", "jour(s)");
    data.insert("home.quick_start", "Démarrage Rapide");
    data.insert("home.new_note", "Nouvelle Note");
    data.insert("home.note_list", "Liste des Notes");
    data.insert("home.split_view", "Vue Divisée");
    data.insert("home.srs_review", "Révision SRS");
    data.insert("home.search", "Rechercher");
    data.insert("home.zim_manage", "Gérer ZIM");
    data.insert("home.security", "Sécurité");
    data.insert("home.shortcuts", "Raccourcis Clavier");
    data.insert("home.shortcut.new_note", "Nouvelle note");
    data.insert("home.shortcut.search", "Rechercher");
    data.insert("home.shortcut.help", "Aide");

    // Editor
    data.insert("editor.title_placeholder", "Entrez le titre");
    data.insert("editor.content_placeholder", "Commencez à écrire ici...");
    data.insert("editor.tags", "Tags");
    data.insert("editor.tags_placeholder", "tag1, tag2, ...");
    data.insert("editor.edit_note", "Modifier la Note");
    data.insert("editor.encrypt", "Activer le chiffrement");
    data.insert("editor.focus", "Focus");
    data.insert("editor.fullscreen", "Plein Écran");
    data.insert("editor.save", "Enregistrer");
    data.insert("editor.saved", "Enregistré");
    data.insert("editor.auto_saved", "Enregistrement automatique");
    data.insert("editor.changed", "Modifié...");
    data.insert("editor.words", "mots");
    data.insert("editor.save_complete", "💾 Enregistré!");
    data.insert("editor.encrypt_on", "🔒 Chiffrement activé");
    data.insert("editor.encrypt_off", "🔓 Chiffrement désactivé");
    data.insert(
        "editor.pin_required",
        "Configurez d'abord le PIN (Menu Sécurité)",
    );
    data.insert("editor.pin_locked", "Déverrouillez d'abord le PIN");

    // Notes list
    data.insert("notes.title", "Notes");
    data.insert("notes.export", "Exporter");
    data.insert("notes.import", "Importer");
    data.insert("notes.no_notes", "Pas encore de notes");
    data.insert("notes.create_first", "Créez votre première note!");
    data.insert("notes.no_title", "Sans Titre");
    data.insert("notes.edit", "Modifier");
    data.insert("notes.delete", "Supprimer");
    data.insert(
        "notes.delete_confirm",
        "Êtes-vous sûr de vouloir supprimer?",
    );
    data.insert("notes.encrypted", "Chiffré");
    data.insert("notes.created", "Créé");
    data.insert("notes.updated", "Mis à jour");
    data.insert("notes.find_duplicates", "Trouver les Doublons");
    data.insert("notes.select_all", "Tout Sélectionner");
    data.insert("notes.selected", "sélectionnés");
    data.insert("notes.create_package", "Créer un Paquet");
    data.insert("notes.duplicates_title", "Notes en Double");
    data.insert("notes.no_duplicates", "Aucune note en double trouvée");
    data.insert("notes.export_package", "Exporter le Paquet");
    data.insert("notes.import_package", "Importer le Paquet");
    data.insert("notes.package_title", "Titre du Paquet");
    data.insert("notes.package_title_placeholder", "ex., Physique 101");
    data.insert("notes.package_author", "Auteur");
    data.insert("notes.package_author_placeholder", "Votre nom");
    data.insert("notes.package_description", "Description");
    data.insert(
        "notes.package_description_placeholder",
        "Description du paquet (optionnel)",
    );
    data.insert("notes.drop_file", "Déposez le fichier .laz ici");
    data.insert("notes.or", "ou");
    data.insert("notes.select_file", "Sélectionner un Fichier");

    // Search
    data.insert("search.title", "Rechercher");
    data.insert("search.placeholder", "Entrez le terme de recherche...");
    data.insert("search.button", "Rechercher");
    data.insert("search.results", "Résultats de Recherche");
    data.insert("search.no_results", "Aucun résultat trouvé");
    data.insert("search.try_different", "Essayez un terme différent");
    data.insert("search.tips", "Conseils de Recherche");
    data.insert(
        "search.tip1",
        "Plusieurs mots recherchent des résultats contenant tous les termes",
    );
    data.insert("search.tip2", "Recherche dans les notes et Wikipedia");
    data.insert(
        "search.tip3",
        "Les correspondances de titre apparaissent en premier",
    );

    // Wiki
    data.insert("wiki.search", "Rechercher Wiki");
    data.insert("wiki.recent_articles", "Recent Articles");
    data.insert("wiki.manage", "Gérer ZIM");
    data.insert("wiki.status", "État");
    data.insert("wiki.loaded", "chargés");
    data.insert("wiki.directory", "Répertoire ZIM");
    data.insert(
        "wiki.directory_hint",
        "Copiez les fichiers .zim dans ce dossier et actualisez",
    );
    data.insert("wiki.refresh", "Actualiser");
    data.insert("wiki.open_folder", "Ouvrir le Dossier");
    data.insert("wiki.add", "Ajouter ZIM");
    data.insert("wiki.add_placeholder", "Entrez le chemin du fichier ZIM...");
    data.insert("wiki.loaded_files", "Fichiers ZIM Chargés");
    data.insert("wiki.remove", "Supprimer");
    data.insert("wiki.no_zim", "Pas de fichiers ZIM");
    data.insert(
        "wiki.no_zim_hint",
        "Ajoutez un fichier ZIM pour utiliser Wikipedia",
    );
    data.insert("wiki.no_zim_loaded", "Aucun fichier ZIM chargé");
    data.insert("wiki.loaded_zims", "ZIMs Chargés");
    data.insert("wiki.add_btn", "Ajouter");
    data.insert("wiki.add_hint", "Entrez le chemin complet du fichier ZIM.");
    data.insert("wiki.name", "Nom");
    data.insert("wiki.path", "Chemin");
    data.insert("wiki.action", "Action");
    data.insert("wiki.zim_added", " ZIM ajoutés: ");
    data.insert("wiki.no_new_zim", "Pas de nouveaux fichiers ZIM");
    data.insert("wiki.refresh_failed", "Échec de l'actualisation");
    data.insert("wiki.enter_path", "Veuillez entrer un chemin");
    data.insert("wiki.add_failed", "Échec de l'ajout");
    data.insert("wiki.remove_confirm", "Supprimer ce ZIM?");
    data.insert("wiki.zim_removed", "ZIM supprimé: ");
    data.insert("wiki.remove_failed", "Échec de la suppression");
    data.insert(
        "wiki.open_folder_msg",
        "Ouvrez le dossier dans l'explorateur de fichiers:",
    );

    // SRS
    data.insert("srs.title", "Révision SRS");
    data.insert("srs.cards", "Liste des Cartes");
    data.insert("srs.streak", "série");
    data.insert("srs.show_answer", "Afficher la Réponse");
    data.insert("srs.again", "Encore");
    data.insert("srs.hard", "Difficile");
    data.insert("srs.good", "Bien");
    data.insert("srs.easy", "Facile");
    data.insert("srs.complete", "🎉 Révision du jour terminée!");
    data.insert("srs.no_cards", "Pas de cartes à réviser");
    data.insert("srs.progress", "cartes");
    data.insert("srs.today_review", "Révision du Jour");
    data.insert("srs.new_cards", "Nouvelles");
    data.insert("srs.learning", "En cours");
    data.insert("srs.mature", "Maîtrisées");
    data.insert(
        "srs.create_from_notes",
        "Créer des cartes à partir des notes",
    );
    data.insert("srs.create_cards", "Créer des Cartes");
    data.insert("srs.cards_created", "cartes créées");
    data.insert("srs.no_cards_extracted", "Pas de cartes à extraire");
    data.insert("srs.repetitions", "Répétitions");
    data.insert("srs.streak_days", "Série");
    data.insert("srs.start_review", "Commencer la Révision");
    data.insert("srs.no_cards_yet", "Pas encore de cartes.");
    data.insert(
        "srs.create_hint",
        "Ouvrez une note et cliquez sur \"Créer des Cartes\".",
    );
    data.insert("srs.interval", "Intervalle");
    data.insert("srs.days", " jours");
    data.insert("srs.delete_card_confirm", "Supprimer cette carte?");
    data.insert("srs.delete_failed", "Échec de la suppression");

    // Security
    data.insert("security.title", "Paramètres de Sécurité");
    data.insert("security.pin_not_set", "PIN non configuré");
    data.insert("security.pin_enabled", "PIN activé");
    data.insert("security.locked", "Verrouillé - Entrez PIN");
    data.insert("security.pin_input", "Entrez PIN (6-32 caractères)");
    data.insert("security.set_pin", "Configurer PIN");
    data.insert("security.remove_pin", "Supprimer PIN");
    data.insert("security.lock", "Verrouiller");
    data.insert("security.unlock", "Déverrouiller");
    data.insert("security.wrong_pin", "PIN incorrect");
    data.insert("security.pin_set_success", "PIN configuré");
    data.insert("security.pin_removed", "PIN supprimé");
    data.insert("security.unlocked", "Déverrouillé");
    data.insert(
        "security.pin_min_length",
        "Le PIN doit avoir au moins 6 caractères",
    );
    data.insert("security.enter_current_pin", "Entrez le PIN actuel");

    // Backup
    data.insert("backup.title", "Sauvegarde");
    data.insert("backup.info", "Infos Sauvegarde");
    data.insert("backup.now", "Sauvegarder Maintenant");
    data.insert("backup.complete", "Sauvegarde terminée");
    data.insert(
        "backup.no_changes",
        "Pas de changements (sauvegarde ignorée)",
    );

    // Split view
    data.insert("split.select_note", "Sélectionnez une note à gauche");

    // Common buttons
    data.insert("common.confirm", "Confirmer");
    data.insert("common.cancel", "Annuler");
    data.insert("common.close", "Fermer");
    data.insert("common.loading", "Chargement...");
    data.insert("common.error", "Une erreur s'est produite");
    data.insert("common.success", "Succès");

    // Settings
    data.insert("settings.title", "Paramètres");
    data.insert("settings.language", "Langue");
    data.insert(
        "settings.language_desc",
        "Sélectionnez votre langue préférée",
    );
    data.insert("settings.stats", "Statistiques");
    data.insert("settings.version", "Version");
    data.insert("settings.storage", "Stockage");
    data.insert("settings.about", "À propos");

    // Knowledge Graph & Footer
    data.insert("home.knowledge_graph", "Graphe de Connaissances");
    data.insert("app.footer", "Le Savoir pour Tous");

    // === USB ===
    data.insert("usb.title", "Synchronisation USB");
    data.insert("usb.scan", "Scanner");
    data.insert("usb.scanning", "Recherche de périphériques USB");
    data.insert("usb.no_usb", "Aucun USB Lazarus détecté");
    data.insert(
        "usb.no_usb_hint",
        "Insérez une clé USB avec lazarus.sync ou initialisez-en une",
    );
    data.insert("usb.error", "Échec du scan");
    data.insert("usb.init_title", "Initialiser USB");
    data.insert(
        "usb.init_desc",
        "Créer une nouvelle clé USB Lazarus pour le partage hors ligne",
    );
    data.insert("usb.init_btn", "Initialiser");
    data.insert("usb.init_error", "Échec de initialisation");
    data.insert("usb.enter_path", "Entrez le chemin USB");
    data.insert("usb.notes", "Notes");
    data.insert("usb.posts", "Publications");
    data.insert("usb.packages", "Paquets");
    data.insert("usb.sync", "Synchroniser");
    data.insert("usb.export", "Exporter");
    data.insert("usb.import", "Importer");
    data.insert("home.usb_sync", "Sync USB");
    data.insert("home.shortcut.wiki", "Wiki");
    data.insert("home.shortcut.usb", "USB");
    data.insert("home.shortcut.graph", "Graphe");
    data.insert("home.shortcut.review", "Réviser");
    Translations::new(data)
}
