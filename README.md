# Fractal Processing Server

## Aperçu

Fractal Processing Server est une application serveur basée sur Rust conçue pour le rendu et la génération d'images fractales. Le serveur utilise la communication TCP pour envoyer et recevoir des messages contenant des données fractales. Ce README fournit une vue d'ensemble de la structure du projet, des fonctionnalités clés et des instructions d'utilisation.

## Table des matières

- [Fonctionnalités](#fonctionnalités)
- [Prérequis](#prérequis)
- [Installation](#installation)
- [Utilisation](#utilisation)
- [Exemples](#exemples)
- [Organisation du projet](#organisation)
- [Contributions](#contributions)
- [Licence](#licence)

## Fonctionnalités

- **Rendu Fractal :** Le serveur prend en charge le rendu de divers fractales, y compris les ensembles de Julia, les ensembles de Mandelbrot, et plus encore. Le rendu se fait sous forme d'image .png.
- **Gestion des Messages :** Gestion efficace des messages sur un flux TCP, avec des capacités de sérialisation et de désérialisation.
- **Multi-client :** Le serveur est multi-threadé permettant à plusieurs clients de se connecter en simultanée et opérer les calculs de fractal en parallèle.
- **Parallelisation des calculs :** Avec le module Rayon, les calculs sont effectués en parallèle sur le client afin d'optimiser le temps de calcul.

## Prérequis

- Langage de programmation Rust. Installez Rust depuis [rustup.rs](https://rustup.rs/).
- Dépendances spécifiées dans le fichier `Cargo.toml`.

## Installation

1. Clonez le dépôt :

   ```bash
   $ git clone https://github.com/Silverliee/frakt.git
   $ cd frakt
   ```

2. Build le projet :
   ```bash
   $ cargo build --release
   ```

## Utilisation

**1. Serveur:**

### Les options en ligne de commandes

Une aide en ligne de commande est disponible:

```bash
$ ./server --help
```

```
Usage: server [OPTIONS]

Options:

  --fractal
      Choose the fractal to be computed
      list of fractal available:
       Julia,
       Mandelbrot,
       IteratedSinZ,
       NetonRaphsonZ3,
       NewtonRaphsonZ4,
       NovaNewtonRaphsonZ3,
       NovaNewtonRaphsonZ4
       ex: --fractal=Julia

   --ip
      Choose the ip the server while listen tp
      Default value is localhost
      ex: --ip=0.0.0.0

   --port
      Choose the port
      Default value is 8787
      ex: --port=8080
```

Pour lancer une instance du serveur:

```bash
$ cd target/release
$ ./server --fractal=Mandelbrot --ip=127.0.0.1 --port=8787
```

Par defaut, le serveur est parametré pour demander le calcul d'une fractal Julia.

Pour que ces calculs soient effectués, il est nécessaire d'avoir un client qui se connecte à ce serveur.
Lorsque le serveur a tous les calculs pour générer la fractale finale, une image est créée dans le répertoire courant:

> ./image/server/

2. Client:

### Les options en ligne de commandes

Une aide en ligne de commande est disponible:

```bash
$ ./worker --help
```

```
Usage: worker <ip> <port>

Options:

   --ip
      Choose the ip the server while listen tp
      Default value is localhost
      ex: --ip=0.0.0.0

   --port
      Choose the port
      Default value is 8787
      ex: --port=8080
```

Pour utiliser le client:

```bash
$ cd target/release
$ ./worker
```

Par défault, le client se connecte en localhost sur le port 8787.
Lorsque le client se connecte, il effectue une demande de tâche au serveur. Lorsque les calculs liés à la tâche sont effectué, une image est créée dans le répertoire courant:

> ./image/worker/

## Organisation du projet

1. Organisation d'équipe

Notre équipe a travaillé principalement en pair/mob programming. L'idée était de centraliser les idées lors de réunion vocale sur Discord pendant qu'une personne a tour de rôle écrivait/modifiait le code. Cela explique aussi pourquoi la plupart des commits sur Git sont sous le compte de Damien (son ordinateur était la référence pour l'écriture).

2. Demarche d'élaboration

Nous avons répartis notre travail en plusieurs etapes de conception.  
Cela est dû à notre montée en compétences avec l'avancée des cours et la pratique.  
Le schéma suivant a été suivi:

- Mise en place des différents types de données/structures
- Creation du client
- Gestion des calculs des differentes fractales avec serveur de test
- Creation d'un serveur basique sans thread
- Refactorisation global du projet pour avoir 4 crates (client/serveur/lib partagées/complexes)
- Gestion des erreurs sur tout le projet
- Modification du serveur avec threads
- Documentation

Le code a été refactoré de nombreuses fois au cours du projet.

3. Spécificités du projet

N'ayant pas réussi à faire une interface graphique au niveau du serveur, le thread du serveur génère une nouvelle fractale aléatoire à calculer 5sec après la fin de celle en cours.  
Chaque calcul génère une image pour le client, ainsi que pour le serveur lorsque toutes les tâches ont été complétées. **(attention cela fait rapidement beaucoup d'images !)**  
Documentation via Rustdoc (point d'entrée html dans /documentation)

4. Bonus du projet

- Serveur multi thread
- Calcul client en parallèle avec Rayon
- Aucun unwrap(), panic() ou expect() (ni unsafe !)

## Contributions

MARQUES Rémi  
MOUYABI MAOUENE Chancy Catri  
MULET Jules  
TRAORE Mohamed Seydou  
WILLEMAIN Damien

## Licence

@Durjug_fractal_2024
