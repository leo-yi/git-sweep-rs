# git-sweep-rs

[English](README.en.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [हिन्दी](README.hi.md) · [Français](README.fr.md) · [Русский](README.ru.md)

> ⚠️ **Avertissement : Sauvegardez votre dépôt avant utilisation**
>
> Veuillez sauvegarder votre dépôt avant d'utiliser cet outil (par exemple, poussez les branches locales importantes vers un remote ou créez une sauvegarde). Cet outil supprime des branches et les suppressions sont irréversibles. Pour une première utilisation, exécutez `git-sweep-rs preview` pour inspecter les branches qui seraient supprimées, puis lancez `cleanup` ou `cleanup-local` après confirmation. Utilisez `--force` pour ignorer les confirmations (à manipuler avec précaution).

Outil de nettoyage de branches Git écrit en Rust, permettant de supprimer rapidement les branches (locales et distantes) déjà fusionnées dans la branche principale.

## ✨ Fonctionnalités

- 🚀 Haute performance : écrit en Rust pour la rapidité
- 🔍 Aperçu sûr : prévisualisez les branches avant suppression
- 🌐 Nettoyage distant : supprime les branches distantes fusionnées
- 💻 Nettoyage local : supprime les branches locales fusionnées
- 🎯 Flexible : configuration du nom de la branche principale et liste d'exclusion
- ✅ Confirmation interactive : demande une confirmation avant suppression (`--force` pour l'ignorer)

## 📦 Installation

### Compiler depuis les sources

Assurez-vous d'avoir [Rust](https://www.rust-lang.org/tools/install) installé, puis :

```bash
git clone https://github.com/leo-yi/git-sweep-rs
cd git-sweep-rs
cargo build --release
```

### Installer dans le système

**Méthode 1 : utiliser `cargo install`**

```bash
cargo install --path .
```

**Méthode 2 : copie manuelle**

```bash
sudo cp target/release/git-sweep-rs /usr/local/bin/
```

## 🚀 Utilisation

### Branches distantes

#### Prévisualiser les branches distantes à supprimer

```bash
git-sweep-rs preview --origin origin --master master
```

#### Supprimer des branches distantes

```bash
# demandera confirmation avant suppression
git-sweep-rs cleanup --origin origin --master master

# suppression forcée sans confirmation
git-sweep-rs cleanup --force --origin origin --master master
```

### Branches locales

#### Prévisualiser les branches locales à supprimer

```bash
git-sweep-rs preview-local --master master
```

#### Supprimer des branches locales

```bash
# demandera confirmation avant suppression
git-sweep-rs cleanup-local --master master

# suppression forcée sans confirmation
git-sweep-rs cleanup-local --force --master master
```

## 📖 Commandes

| Commande | Description | Cible |
|--------:|:-----------|:------|
| `preview` | Prévisualise les branches distantes à supprimer | branches distantes |
| `cleanup` | Supprime les branches distantes fusionnées | branches distantes |
| `preview-local` | Prévisualise les branches locales à supprimer | branches locales |
| `cleanup-local` | Supprime les branches locales fusionnées | branches locales |

### Options communes

#### preview / cleanup

- `--origin <ORIGIN>`: nom du remote (par défaut : `origin`)
- `--master <MASTER>`: nom de la branche principale (par défaut : `master`)
- `--nofetch`: ne pas récupérer les mises à jour du remote
- `--skip <BRANCHES>`: liste de branches à ignorer (séparées par des virgules)
- `--force`: supprime sans confirmation (cleanup uniquement)

#### preview-local / cleanup-local

- `--master <MASTER>`: nom de la branche principale (par défaut : `master`)
- `--skip <BRANCHES>`: liste de branches à ignorer (séparées par des virgules)
- `--force`: supprime sans confirmation (cleanup-local uniquement)

## 💡 Exemples

### Scénario 1 : Nettoyer les branches distantes

```bash
# 1. Prévisualiser les branches à supprimer
git-sweep-rs preview --origin origin --master main

# 2. Confirmer et supprimer
git-sweep-rs cleanup --origin origin --master main
```

### Scénario 2 : Nettoyer les branches locales

```bash
# 1. Prévisualiser les branches locales à supprimer
git-sweep-rs preview-local --master main

# 2. Confirmer et supprimer
git-sweep-rs cleanup-local --master main
```

### Scénario 3 : Ignorer des branches

```bash
# ignorer develop et staging
git-sweep-rs preview --master main --skip "develop,staging"
git-sweep-rs cleanup --master main --skip "develop,staging"
```

### Scénario 4 : Ne pas récupérer les mises à jour distantes

```bash
# utiliser les informations distantes en cache local
git-sweep-rs preview --nofetch
git-sweep-rs cleanup --nofetch
```

## ⚠️ Remarques

1. La suppression des branches distantes est destructive
   - Utilisez `preview` pour inspecter d'abord
   - Les branches distantes supprimées ne sont pas facilement récupérables
   - Les collaborateurs doivent exécuter `git fetch --prune` pour synchroniser

2. Suppression des branches locales
   - Utilisez `git branch -d` pour une suppression sûre
   - Pour forcer la suppression, utilisez `git branch -D`

3. Nom de la branche principale
   - Beaucoup de projets utilisent `main` au lieu de `master`
   - Utilisez `--master main` si besoin

## 🔧 Fonctionnement

### Nettoyage distant

1. Récupère les mises à jour du remote (sauf si `--nofetch`)
2. Liste les branches distantes via `git for-each-ref`
3. Utilise `git cherry` pour vérifier si une branche est totalement fusionnée dans main
4. Liste les branches fusionnées
5. Supprime la branche distante avec `git push origin :branch`

### Nettoyage local

1. Liste les branches locales fusionnées avec `git branch --merged`
2. Exclut la branche principale et la liste d'exclusion
3. Liste les branches fusionnées
4. Supprime la branche locale avec `git branch -d`

## 🤝 Contribution

Issues et PR sont les bienvenus !

## 📝 Licence

MIT License

## 🙏 Remerciements

Inspiré par le projet Python [git-sweep](https://github.com/arc90/git-sweep).

## 📮 Retour d'information

Si vous avez des problèmes ou des suggestions :

- Ouvrez un [Issue](https://github.com/leo-yi/git-sweep-rs/issues)
- Créez une [Pull Request](https://github.com/leo-yi/git-sweep-rs/pulls)

---

⭐ Si ce projet vous aide, merci de le starer !

