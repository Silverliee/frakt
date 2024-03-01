# Fractal Processing Server

## Aperçu

Fractal Processing Server est une application serveur basée sur Rust conçue pour le rendu et la génération d'images fractales. Le serveur utilise la communication TCP pour envoyer et recevoir des messages contenant des données fractales. Ce README fournit une vue d'ensemble de la structure du projet, des fonctionnalités clés et des instructions d'utilisation.

## Table des matières

- [Fonctionnalités](#fonctionnalités)
- [Prérequis](#prérequis)
- [Installation](#installation)
- [Utilisation](#utilisation)
- [Exemples](#exemples)
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
$ ./server --fractal Mandelbrot --ip=127.0.0.1
```

Par defaut, le serveur est parametré pour demander le calcul d'une fractal Julia.

Pour que ces calculs soient effectués, il est nécessaire d'avoir un client qui se connecte à ce serveur.

2. Client:

### Les options en ligne de commandes

Une aide en ligne de commande est disponible:

```bash
$ ./worker --help
```

```
Usage: worker <ip> <port>
Default value for ip is localhost
Default value for port is 8787
```

Pour utiliser le client:

```bash
$ cd target/release
$ ./worker
```

Par défault, le client se connecte en localhost sur le port 8787.

## Contributions

MARQUES Rémi  
MOUYABI MAOUENE Chancy Catri  
MULET Jules  
TRAORE Mohamed Seydou  
WILLEMAIN Damien

## Licence

@Durjug_fractal_2024
