# 🛡️ SentryPass Pro

<p align="center">
  <img src="https://img.shields.io/badge/Rust-2024_Edition-orange?style=for-the-badge&logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/Platform-Windows_10_%7C_11-blue?style=for-the-badge&logo=windows" alt="Windows">
  <img src="https://img.shields.io/badge/Sécurité-CSPRNG_Xoshiro256++-success?style=for-the-badge" alt="Sécurité">
  <img src="https://img.shields.io/badge/Vitesse-~12_µs-purple?style=for-the-badge" alt="Vitesse">
  <img src="https://img.shields.io/badge/Licence-MIT-green?style=for-the-badge" alt="Licence">
</p>

<p align="center">
  <b>Sentinelle Cryptographique & Suite de Cyber-Défense Haute Performance pour Windows</b><br>
  <i>Génération de mots de passe impénétrables, passphrases mnémoniques Diceware, tokens API et auditeur de sécurité en temps réel.</i>
</p>

---

## 📌 À Propos de SentryPass Pro

**SentryPass Pro** est une application de bureau Windows conçue pour les professionnels, développeurs et administrateurs système soucieux de leur sécurité numérique. 

Contrairement aux générateurs web traditionnels qui dépendent de scripts distants et de serveurs tiers, **SentryPass Pro fonctionne à 100% en local et hors-ligne**. Propulsé par un moteur cryptographique natif écrit en **Rust**, il garantit une génération aléatoire ultra-sécurisée en quelques microsecondes, enveloppée dans une interface de cyber-défense moderne au design sombre épuré (Glassmorphism), optimisée pour un affichage sur mesure sans aucun ascenseur ni scroll gênant.

---

## ✨ Fonctionnalités Principales

### 1. 🔐 Multi-Modes de Génération Cryptographique
* **Mots de Passe Aléatoires (Complexité Maximale) :**
  * Curseur de longueur fluide de **6 à 128 caractères** avec préréglages rapides (`12`, `16`, `24`, `32`, `64`).
  * Filtres modulables : Majuscules (`A-Z`), Minuscules (`a-z`), Chiffres (`0-9`), Symboles spéciaux (`!@#$%^&*`).
  * **Option Anti-Ambiguïté :** Élimine automatiquement les caractères visuellement confus (`0`, `O`, `1`, `l`, `I`, `|`) pour une retranscription manuscrite infaillible.
* **Passphrase Mnémonique (Style Diceware / 1Password) :**
  * Génère des phrases composées de vrais mots aléatoires issus d'un dictionnaire cryptographique soigné en français et anglais (ex: `Faucon-Azur-Cascade-Prisme-42`).
  * Mémorisation facile pour les humains, impossible à casser par force brute informatique.
  * Personnalisation du nombre de mots (3 à 8), du séparateur (`-`, `_`, `.`, espace) et inclusion facultative d'un chiffre.
* **Tokens d'API & Clés Cryptographiques :**
  * **Clés Hexadécimales 256-bit :** Idéales pour les secrets d'API, tokens HMAC et hashes SHA-256.
  * **Tokens Base64 URL-Safe :** Format standard pour les jetons OAuth, Bearer et JWT.
  * **UUID v4 :** Identifiants uniques universels aléatoires conformes à la norme RFC 4122 (`xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx`).
* **Code PIN Numérique :**
  * De 4 à 12 chiffres avec filtre automatique anti-séquences (évite les répétitions triviales comme `0000` ou les suites comme `1234`).

---

### 2. 🛡️ Auditeur de Sécurité en Direct (Password Health Check)
Un module complet pour tester et évaluer instantanément la robustesse de n'importe quel mot de passe existant :
* **Score de Robustesse en Temps Réel (0 à 100) :** Notation instantanée avec indicateur visuel de vulnérabilité (*Critique*, *Faible*, *Moyen*, *Robuste*, *Invulnérable*).
* **Checklist de Conformité Cyber :**
  * ✔️ Longueur minimale recommandée (≥ 12 caractères).
  * ✔️ Diversité de casse (présence conjointe de majuscules et de minuscules).
  * ✔️ Présence de chiffres et de symboles complexes.
  * ✔️ Détection des mots de passe de fuites connues et dictionnaires courants (`password`, `123456`, `azerty`, `admin`, etc.).
  * ✔️ Détection des répétitions et séquences évidentes du clavier.
* **Simulateur d'Attaques Réelles :**
  * 🖥️ **Cluster GPU Moderne (8x RTX 4090 / Hashcat) :** Calcul basé sur ~100 milliards de hash/seconde.
  * 🌐 **Attaque en Ligne Rapide :** Scénario sans protection anti-brute-force (~1 000 essais/seconde).
  * 🛡️ **Attaque Web avec Quota (Rate-limited) :** Scénario standard avec limitation de requêtes web (~5 essais/minute).

---

### 3. 🎯 Ergonomie & Outils Professionnels
* **👁️ Masquage / Révélation :** Masquez le mot de passe à l'écran (`••••••••`) d'un simple clic pour vous protéger des regards indiscrets (*shoulder surfing*).
* **📢 Alphabet Phonétique (OTAN) :** Décomposition orale instantanée (`Alpha`, `Bravo`, `Kilo`, `Sept...`) pour dicter facilement vos mots de passe par téléphone ou visioconférence.
* **⚡ Génération par Lot :** Générez 5 mots de passe simultanés en un clic avec bouton de copie individuelle.
* **🕒 Historique de Session Éphémère :** Les 20 derniers mots de passe générés sont stockés temporairement dans un tiroir modal avec copie rapide et bouton de purge complète de la mémoire.
* **🚫 Zéro Débordement & Zéro Scroll :** Interface épurée et dimensionnée sur mesure pour s'adapter à 100% de la fenêtre de bureau sans ascenseur.

---

## ⚡ Architecture Technique & Performance

| Composant | Technologie Utilisée | Description |
| :--- | :--- | :--- |
| **Backend** | **Rust (Édition 2024)** | Moteur natif compilé, gestion de mémoire ultra-sécurisée sans Garbage Collector. |
| **Algorithme Aléatoire** | **Xoshiro256++** | Générateur pseudo-aléatoire de haute qualité avec graine entropique (*nanosecondes + PID + ASLR*). |
| **Vitesse de Calcul** | **~12 microsecondes (µs)** | Génération quasi-instantanée en code machine direct. |
| **Frontend UI** | **HTML5 / CSS3 Moderne / JS** | Thème sombre Cyber-Défense, polices *Outfit* & *JetBrains Mono*, modales popovers. |
| **Mode Bureau** | **Windows App Mode (Edge/Chromium)** | Fenêtre autonome dédiée 980x760 px, sans barre d'adresse ni onglets de navigation. |
| **Confidentialité** | **100% Hors-Ligne** | Aucune télémétrie, aucune donnée transmise sur le réseau. |

---

## 🚀 Installation & Lancement Rapide

### Prérequis
* Système d'exploitation : **Windows 10 ou Windows 11** (64-bit).
* Si vous souhaitez compiler depuis les sources : **Rust & Cargo** installés.

---

### Méthode 1 : Le Lanceur en 1 Clic (Recommandé)
Dans le dossier du projet :
1. Double-cliquez sur le fichier :
   ```text
   Lancer_SentryPass.bat
   ```
2. La fenêtre **SentryPass Pro** s'ouvre immédiatement sur votre écran !

---

### Méthode 2 : L'Exécutable Autonome (.exe)
Vous pouvez lancer directement le binaire compilé :
```text
target\release\sentrypass.exe
```
> **Astuce :** Faites un clic droit sur `sentrypass.exe` > **Envoyer vers** > **Bureau (créer un raccourci)** pour lancer SentryPass Pro directement depuis votre Bureau Windows !

---

### Méthode 3 : Compilation depuis les Sources
Pour compiler et exécuter l'application avec Cargo :
```powershell
# Cloner ou se placer dans le dossier
cd c:\Users\HP\Desktop\code\password_generator

# Compiler en mode Release optimisé
cargo build --release

# Lancer l'application
cargo run --release
```

---

## 📂 Structure du Répertoire

```text
password_generator/
├── src/
│   ├── main.rs              # Serveur Rust, RNG Xoshiro256++, moteur d'audit et routes API
│   └── index.html           # Interface graphique Cyber-Défense (HTML/CSS/JS sans scroll)
├── target/
│   └── release/
│       └── sentrypass.exe   # Exécutable binaire Windows autonome (~1.2 Mo)
├── Cargo.toml               # Configuration du projet Rust (package sentrypass)
├── Cargo.lock               # Verrouillage des dépendances
├── Lancer_SentryPass.bat    # Script lanceur 1-clic pour Windows
└── README.md                # Documentation officielle du projet
```

---

## 🔒 Sécurité & Déontologie

SentryPass Pro a été conçu selon le principe de **divulgation zéro** :
* **Aucun stockage persistant non sollicité :** Les mots de passe générés n'écrivent rien sur votre disque dur.
* **Mémoire éphémère :** L'historique de session ne réside qu'en mémoire vive (RAM) et est détruit dès la fermeture de l'application ou par le bouton de purge.
* **Indépendance réseau :** L'application n'effectue aucun appel vers des serveurs externes.

---

## 📜 Licence

Ce projet est distribué sous licence **MIT**. Vous êtes libre de l'utiliser, l'étudier, le modifier et le distribuer conformément aux termes de la licence.

<p align="center">
  <i>Développé par Bleu Serge Alain avec passion et sécurité en Rust. 🛡️</i>
</p>