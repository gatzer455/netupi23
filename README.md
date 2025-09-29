# 🌻 Netupi23 - Minimalist Time Tracker & Pomodoro Timer

[**English**](#english) | [**Español**](#español)

---

## English

**Inspired by and forked from the work of [4DA](https://github.com/4DA)**
*All acknowledgments to 4DA for the original concept and inspiration that made this project possible.*

A minimalist time tracking application written in Rust. Perfect for developers, freelancers, and anyone who wants to track their work time efficiently.


### 📦 Installation

Netupi23 is easy to install. Choose your preferred method:

- **Via Cargo (Recommended for Rust users)**:
  ```bash
  cargo install netupi23
  ```

This installs the `netupi` binary to `~/.cargo/bin`. Add to PATH if needed:
  ```bash
  echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
  source ~/.bashrc
  ```

- **Pre-built Binaries**: Download from [GitHub Releases](https://github.com/gatzer/netupi23/releases).
  For Linux/macOS:
  ```bash
  wget https://github.com/gatzer/netupi23/releases/latest/download/netupi-linux-x64
  chmod +x netupi-linux-x64
  sudo mv netupi-linux-x64 /usr/local/bin/netupi  # Or ~/.local/bin for user-local
  ```
  For Windows: Download `netupi-windows.exe` and add to PATH.

- **From Source**:
  Clone and build as in Quick Start, then:
  ```bash
  cargo install --path .
  ```

Run `netupi --help` to verify. Data is stored in `~/.local/share/netupi23/` (Linux/macOS) or `%LOCALAPPDATA%\netupi23` (Windows).



### ✨ Current Features

- **🍅 Pomodoro Timer**: 25-minute focused work sessions
- **☕ Break Timer**: 5-minute break sessions
- **📊 Project Time Tracking**: Track time across different projects with persistent storage, including listing totals, daily summaries, session details, and deletion
- **💾 Local Data Storage**: All sessions saved locally in JSON format
- **🚀 Interactive Mode**: Intuitive command-line interface with history and auto-completion
- **⚡ Auto-stop Feature**: Starting a new timer automatically stops the current one
- **🔒 Privacy First**: All data stays on your computer - no cloud, no tracking
- **📊 Status Checking**: Check your timer progress anytime with the status command

### 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/gatzer/netupi23.git
cd netupi23

# Build and run
cargo run
```

### 📋 Available Commands

### 📋 Available Commands

| Command | Aliases | Description |
|---------|---------|-------------|
| `work <project-name> [description]` | | Start tracking work time for a specific project |
| `pomodoro` | `pomo` | Start a 25-minute Pomodoro work session |
| `break` | | Start a 5-minute break timer |
| `stop` | | Stop the current timer and save the session |
| `status` | `s` | Show current timer status and elapsed time |
| `projects` | | List all projects with total time spent |
| `today` | | Show today's work summary by project |
| `project <name>` | | Show details and sessions for a specific project |
| `delete-project <name>` | | Delete all sessions for a project (irreversible!) |
| `help` | `h` | Show available commands |
| `clear` | `cls` | Clear the terminal screen |
| `exit` | `quit`, `q` | Exit the application |

*Single-command mode: Prefix with `netupi` (e.g., `netupi projects`).*

### 💡 Usage Examples

#### Track Work Time
```bash
🌻 Netupi23 - Interactive Time Tracker
======================================
Type 'help' for available commands or 'exit' to quit.

netupi> work "Website Redesign" "implementing new header"
🚀 Starting work session for project: Website Redesign
📝 Description: implementing new header
⏰ Work timer started! Use 'stop' to finish or 'status' to check progress.

netupi> status
⚡ Work Session - State: Running
⏱️  Elapsed time: 15:32

netupi> stop
✅ Timer stopped!
⏱️  Total time: 15 minutes
💾 Session saved successfully.
```

#### Pomodoro Workflow
```bash
netupi> pomodoro
🍅 Starting 25-minute Pomodoro session...
⏰ Pomodoro started! Use 'stop' to finish early or 'status' to check progress.

netupi> s
🍅 Pomodoro Work Session - State: Running
⏱️  Elapsed time: 05:43

netupi> stop
✅ Timer stopped!
⏱️  Total time: 25 minutes
💾 Session saved successfully.
```

#### Auto-stop Feature
```bash
netupi> pomodoro
🍅 Starting 25-minute Pomodoro session...
⏰ Pomodoro started! Use 'stop' to finish early or 'status' to check progress.

netupi> break
✅ Timer stopped!
⏱️  Total time: 5 minutes
💾 Session saved successfully.

☕ Starting short break...
⏰ Break started! Use 'stop' to finish early.
```

#### Check Status Anytime
```bash
netupi> work "Client Project"
🚀 Starting work session for project: Client Project
⏰ Work timer started! Use 'stop' to finish or 'status' to check progress.

netupi> s
⚡ Work Session - State: Running
⏱️  Elapsed time: 01:23:45
```
#### Project Management
```bash
netupi> projects
📂 Your Projects:
==================
Website Redesign: 2 hours 30 minutes
Client Project: 45 minutes

netupi> today
📅 Today's Work Summary:
======================
Client Project: 45 minutes

netupi> project "Website Redesign"
📊 Project Details: Website Redesign
=========================
Total time: 2 hours 30 minutes

Sessions:
- 2023-10-05 14:30: End: 2023-10-05 15:00 (30 minutes) | Description: implementing new header
- 2023-10-04 10:15: End: 2023-10-04 12:45 (2 hours 30 minutes) | Description: redesign layout

```

### 📁 Data Storage

All your data is stored locally and securely:

- **Linux/macOS**: `~/.local/share/netupi23/`
- **Windows**: `%LOCALAPPDATA%\netupi23\`

Files stored:
- `sessions.json`: All your work sessions with timestamps
- `config.json`: Application configuration and preferences

### 🛠️ Development Status

Netupi23 is actively being developed. Current focus areas:

**Implemented:**
- ✅ Basic Pomodoro and break timers
- ✅ Project time tracking with descriptions
- ✅ Status checking functionality
- ✅ Auto-stop functionality
- ✅ Data persistence
- ✅ Clean CLI interface without display conflicts
- ✅ Project listing and totals (`projects` command)
- ✅ Daily work summaries (`today` command)
- ✅ Project details and session history (`project <name>`)
- ✅ Delete project sessions (`delete-project <name>`)

**Coming Soon:**
- [ ] Pause/Resume functionality
- [x] Daily summary reports (`today` command)
- [x] Project listing (`projects` command)
- [ ] Configuration customization
- [ ] Desktop GUI application

### 🎯 Design Philosophy

Netupi23 follows CLI best practices:
- **Clean Interface**: No real-time display conflicts - check status when you need it
- **Predictable**: Commands behave consistently without unexpected screen updates
- **Professional**: Follows standard CLI patterns that developers expect

### 📄 License

This project is licensed under the AGPL 3.0 License - see the [LICENSE](LICENSE.md) file for details.

### 🙏 Acknowledgments

**Special thanks to [4DA](https://github.com/4DA) for the original inspiration and concept that made this project possible.**

- Built with [Rust](https://www.rust-lang.org/) for performance and reliability
- Uses [rustyline](https://github.com/kkawakam/rustyline) for excellent CLI experience

---

## Español

**Inspirado y basado en el trabajo de [4DA](https://github.com/4DA)**
*Todos los reconocimientos a 4DA por el concepto original e inspiración que hizo posible este proyecto.*

Una aplicación de seguimiento de tiempo minimalista escrita en Rust. Perfecta para desarrolladores, freelancers y cualquier persona que quiera rastrear su tiempo de trabajo de manera eficiente.

### ✨ Características Actuales

- **🍅 Temporizador Pomodoro**: Sesiones de trabajo enfocado de 25 minutos
- **☕ Temporizador de Descanso**: Sesiones de descanso de 5 minutos
- **📊 Seguimiento de Tiempo por Proyecto**: Rastrea tiempo en diferentes proyectos con almacenamiento persistente, incluyendo listado de totales, resúmenes diarios, detalles de sesiones y eliminación
- **💾 Almacenamiento Local**: Todas las sesiones guardadas localmente en formato JSON
- **🚀 Modo Interactivo**: Interfaz de línea de comandos intuitiva con historial y auto-completado
- **⚡ Función de Auto-parada**: Iniciar un nuevo temporizador detiene automáticamente el actual
- **🔒 Privacidad Primero**: Todos los datos se quedan en tu computador - sin nube, sin seguimiento
- **📊 Verificación de Estado**: Verifica el progreso de tu temporizador cuando lo necesites

### 🚀 Inicio Rápido

```bash
# Clonar el repositorio
git clone https://github.com/gatzer/netupi23.git
cd netupi23

# Compilar y ejecutar
cargo run
```

### 📦 Instalación

Netupi23 es fácil de instalar. Elige el método preferido:

- **Vía Cargo (Recomendado para usuarios de Rust)**:
  ```bash
  cargo install netupi23
  ```
  Esto instala el binario `netupi` en `~/.cargo/bin`. Agrega a PATH si es necesario:
  ```bash
  echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
  source ~/.bashrc
  ```

- **Binarios Precompilados**: Descarga desde [Lanzamientos de GitHub](https://github.com/gatzer/netupi23/releases).
  Para Linux/macOS:
  ```bash
  wget https://github.com/gatzer/netupi23/releases/latest/download/netupi-linux-x64
  chmod +x netupi-linux-x64
  sudo mv netupi-linux-x64 /usr/local/bin/netupi  # O ~/.local/bin para instalación local de usuario
  ```
  Para Windows: Descarga `netupi-windows.exe` y agrégalo a PATH.

- **Desde el Código Fuente**:
  Clona y compila como en Inicio Rápido, luego:
  ```bash
  cargo install --path .
  ```

Ejecuta `netupi --help` para verificar. Los datos se almacenan en `~/.local/share/netupi23/` (Linux/macOS) o `%LOCALAPPDATA%\netupi23` (Windows).

### 📋 Comandos Disponibles

### 📋 Comandos Disponibles

| Comando | Alias | Descripción |
|---------|-------|-------------|
| `work <nombre-proyecto> [descripción]` | | Iniciar seguimiento de tiempo para un proyecto específico |
| `pomodoro` | `pomo` | Iniciar una sesión de trabajo Pomodoro de 25 minutos |
| `break` | | Iniciar un temporizador de descanso de 5 minutos |
| `stop` | | Detener el temporizador actual y guardar la sesión |
| `status` | `s` | Mostrar estado actual del temporizador y tiempo transcurrido |
| `projects` | | Listar todos los proyectos con tiempo total invertido |
| `today` | | Mostrar resumen del trabajo de hoy por proyecto |
| `project <nombre>` | | Mostrar detalles y sesiones para un proyecto específico |
| `delete-project <nombre>` | | Eliminar todas las sesiones de un proyecto (¡irreversible!) |
| `help` | `h` | Mostrar comandos disponibles |
| `clear` | `cls` | Limpiar la pantalla del terminal |
| `exit` | `quit`, `q` | Salir de la aplicación |

*Modo de comando único: Prefija con `netupi` (ej. `netupi projects`).*


### 💡 Ejemplos de Uso

#### Rastrear Tiempo de Trabajo
```bash
🌻 Netupi23 - Interactive Time Tracker
======================================
Type 'help' for available commands or 'exit' to quit.

netupi> work "Rediseño Web" "implementando nueva cabecera"
🚀 Iniciando sesión de trabajo para proyecto: Rediseño Web
📝 Descripción: implementando nueva cabecera
⏰ ¡Temporizador de trabajo iniciado! Usa 'stop' para terminar o 'status' para verificar progreso.

netupi> status
⚡ Sesión de Trabajo - Estado: Ejecutándose
⏱️  Tiempo transcurrido: 15:32

netupi> stop
✅ ¡Temporizador detenido!
⏱️  Tiempo total: 15 minutos
💾 Sesión guardada exitosamente.
```

#### Flujo de Trabajo Pomodoro
```bash
netupi> pomodoro
🍅 Iniciando sesión Pomodoro de 25 minutos...
⏰ ¡Pomodoro iniciado! Usa 'stop' para terminar antes o 'status' para verificar progreso.

netupi> s
🍅 Sesión de Trabajo Pomodoro - Estado: Ejecutándose
⏱️  Tiempo transcurrido: 05:43

netupi> stop
✅ ¡Temporizador detenido!
⏱️  Tiempo total: 25 minutos
💾 Sesión guardada exitosamente.
```

#### Función de Auto-parada
```bash
netupi> pomodoro
🍅 Iniciando sesión Pomodoro de 25 minutos...
⏰ ¡Pomodoro iniciado! Usa 'stop' para terminar antes o 'status' para verificar progreso.

netupi> break
✅ ¡Temporizador detenido!
⏱️  Tiempo total: 5 minutos
💾 Sesión guardada exitosamente.

☕ Iniciando descanso corto...
⏰ ¡Descanso iniciado! Usa 'stop' para terminar antes.
```

#### Verificar Estado en Cualquier Momento
```bash
netupi> work "Proyecto Cliente"
🚀 Iniciando sesión de trabajo para proyecto: Proyecto Cliente
⏰ ¡Temporizador de trabajo iniciado! Usa 'stop' para terminar o 'status' para verificar progreso.

netupi> s
⚡ Sesión de Trabajo - Estado: Ejecutándose
⏱️  Tiempo transcurrido: 01:23:45
```
#### Gestión de Proyectos
```bash
netupi> projects
📂 Tus Proyectos:
==================
Rediseño Web: 2 horas 30 minutos
Proyecto Cliente: 45 minutos

netupi> today
📅 Resumen de Trabajo de Hoy:
======================
Proyecto Cliente: 45 minutos

netupi> project "Rediseño Web"
📊 Detalles del Proyecto: Rediseño Web
=========================
Tiempo total: 2 horas 30 minutos

Sesiones:
- 2023-10-05 14:30: Fin: 2023-10-05 15:00 (30 minutos) | Descripción: implementando nueva cabecera
- 2023-10-04 10:15: Fin: 2023-10-04 12:45 (2 horas 30 minutos) | Descripción: rediseñar layout

```
### 📁 Almacenamiento de Datos

Todos tus datos se almacenan local y seguramente:

- **Linux/macOS**: `~/.local/share/netupi23/`
- **Windows**: `%LOCALAPPDATA%\netupi23\`

Archivos almacenados:
- `sessions.json`: Todas tus sesiones de trabajo con timestamps
- `config.json`: Configuración de la aplicación y preferencias

### 🛠️ Estado de Desarrollo

Netupi23 está siendo desarrollado activamente. Áreas de enfoque actuales:

**Implementado:**
- ✅ Temporizadores básicos de Pomodoro y descanso
- ✅ Seguimiento de tiempo por proyecto con descripciones
- ✅ Funcionalidad de verificación de estado
- ✅ Funcionalidad de auto-parada
- ✅ Persistencia de datos
- ✅ Interfaz CLI limpia sin conflictos de visualización
- ✅ Listado de proyectos y totales (comando `projects`)
- ✅ Resúmenes diarios de trabajo (comando `today`)
- ✅ Detalles de proyecto e historial de sesiones (`project <nombre>`)
- ✅ Eliminar sesiones de proyecto (`delete-project <nombre>`)

**Próximamente:**
- [ ] Funcionalidad de Pausar/Reanudar
- [x] Reportes de resumen diario (comando `today`)
- [x] Listado de proyectos (comando `projects`)
- [ ] Personalización de configuración
- [ ] Aplicación GUI de escritorio

### 🎯 Filosofía de Diseño

Netupi23 sigue las mejores prácticas de CLI:
- **Interfaz Limpia**: Sin conflictos de visualización en tiempo real - verifica el estado cuando lo necesites
- **Predecible**: Los comandos se comportan consistentemente sin actualizaciones inesperadas de pantalla
- **Profesional**: Sigue patrones CLI estándar que los desarrolladores esperan

### 📄 Licencia

Este proyecto está licenciado bajo la Licencia AGPL 3.0 - ver el archivo [LICENSE.md](LICENSE.md) para detalles.

### 🙏 Agradecimientos

**Agradecimientos especiales a [4DA](https://github.com/4DA) por la inspiración original y el concepto que hizo posible este proyecto.**

- Construido con [Rust](https://www.rust-lang.org/) para rendimiento y confiabilidad
- Usa [rustyline](https://github.com/kkawakam/rustyline) para una excelente experiencia CLI

---
