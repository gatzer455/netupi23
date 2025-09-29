# 🌻 Netupi23 - Minimalist Time Tracker & Pomodoro Timer

[**English**](#english) | [**Español**](#español)

---

## English

**Inspired by and forked from the work of [4DA](https://github.com/4DA)**  
*All acknowledgments to 4DA for the original concept and inspiration that made this project possible.*

A minimalist time tracking application written in Rust. Perfect for developers, freelancers, and anyone who wants to track their work time efficiently.

### ✨ Current Features

- **🍅 Pomodoro Timer**: 25-minute focused work sessions 
- **☕ Break Timer**: 5-minute break sessions
- **📊 Project Time Tracking**: Track time across different projects with persistent storage
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

| Command | Aliases | Description |
|---------|---------|-------------|
| `work <project-name> [description]` | | Start tracking work time for a specific project |
| `pomodoro` | `pomo` | Start a 25-minute Pomodoro work session |
| `break` | | Start a 5-minute break timer |
| `stop` | | Stop the current timer and save the session |
| `status` | `s` | Show current timer status and elapsed time |
| `projects` | | List all projects (coming soon) |
| `today` | | Show today's work summary (coming soon) |
| `help` | `h` | Show available commands |
| `clear` | `cls` | Clear the terminal screen |
| `exit` | `quit`, `q` | Exit the application |

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

**Coming Soon:**
- [ ] Pause/Resume functionality
- [ ] Daily summary reports (`today` command)
- [ ] Project listing (`projects` command)
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
- **📊 Seguimiento de Tiempo por Proyecto**: Rastrea tiempo en diferentes proyectos con almacenamiento persistente
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

### 📋 Comandos Disponibles

| Comando | Alias | Descripción |
|---------|-------|-------------|
| `work <nombre-proyecto> [descripción]` | | Iniciar seguimiento de tiempo para un proyecto específico |
| `pomodoro` | `pomo` | Iniciar una sesión de trabajo Pomodoro de 25 minutos |
| `break` | | Iniciar un temporizador de descanso de 5 minutos |
| `stop` | | Detener el temporizador actual y guardar la sesión |
| `status` | `s` | Mostrar estado actual del temporizador y tiempo transcurrido |
| `projects` | | Listar todos los proyectos (próximamente) |
| `today` | | Mostrar resumen del trabajo de hoy (próximamente) |
| `help` | `h` | Mostrar comandos disponibles |
| `clear` | `cls` | Limpiar la pantalla del terminal |
| `exit` | `quit`, `q` | Salir de la aplicación |

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

**Próximamente:**
- [ ] Funcionalidad de Pausar/Reanudar
- [ ] Reportes de resumen diario (comando `today`)
- [ ] Listado de proyectos (comando `projects`)
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