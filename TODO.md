# ✅ TODO - Projet Shell en Rust

## 1. 🔧 Mise en place du projet
- [ ] Créer un projet Rust avec `cargo new mon_shell`
- [ ] Ajouter un README
- [ ] Ajouter les dépendances utiles :
  - `regex` (parsing)
  - `termion` ou `crossterm` (I/O terminal)
  - `rustyline` ou `reedline` (ligne de commande)
  - `dirs` (gestion des chemins utilisateurs)

---

## 2. 📥 Lecture des entrées
- [ ] Implémenter une boucle REPL (Read-Eval-Print-Loop)
- [ ] Afficher un prompt (`$`)
- [ ] Lire la ligne tapée par l'utilisateur

---

## 3. 🔎 Parsing des commandes
- [ ] Découper la ligne en commande + arguments
- [ ] Gérer les guillemets (`"`, `'`)
- [ ] Gérer les échappements (`\`)

---

## 4. 🧠 Exécution des commandes
- [ ] Détecter les commandes externes
- [ ] Lancer des processus avec `std::process::Command`
- [ ] Afficher la sortie standard et les erreurs
- [ ] Gérer les erreurs (commande non trouvée, etc.)

---

## 5. 🏠 Commandes internes (built-ins)
- [ ] Implémenter `cd`
- [ ] Implémenter `exit`
- [ ] Implémenter `clear`
- [ ] Système pour différencier built-ins et externes

---

## 6. 🔄 Pipes et redirections
- [ ] Gérer les pipes (`|`) entre commandes
- [ ] Redirection de sortie `>` (fichier)
- [ ] Redirection d’entrée `<`
- [ ] Redirection avec append `>>`

---

## 7. 🎨 Fonctionnalités avancées
- [ ] Autocomplétion des commandes et fichiers
- [ ] Historique des commandes (flèches ↑ ↓)
- [ ] Support de la tabulation, navigation ligne
- [ ] Coloration ou style du prompt

---

## 8. 🧪 Robustesse & tests
- [ ] Gestion propre des erreurs utilisateur
- [ ] Ajouter des tests unitaires simples
- [ ] Tester les cas limites (entrées vides, pipes cassés, etc.)

---

## 9. 🚀 Améliorations futures
- [ ] Support des variables d’environnement (`$HOME`, `$PATH`)
- [ ] Substitution de commande (`$(...)`)
- [ ] Jobs en arrière-plan (`&`, `fg`, `bg`)
- [ ] Exécution de scripts `.myshell`

---

## 🗂️ Organisation suggérée
mon_shell/
├── src/
│   ├── main.rs
│   ├── parser.rs
│   ├── executor.rs
│   └── builtins.rs
├── Cargo.toml
└── README.md
